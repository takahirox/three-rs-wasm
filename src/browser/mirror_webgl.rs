//! webgl_mirror: a box room with a turning half sphere, a bouncing
//! icosahedron and two Reflectors ( the ground circle and the back wall ),
//! lit by four point lights. Each Reflector renders the scene from its
//! mirrored camera, with the oblique near plane at the mirror ( clipBias
//! 0.003 ), into its 4× multisampled half-float target, in its
//! onBeforeRender: just before it is drawn, from inside whichever render
//! draws it. The other mirror's onBeforeRender then runs inside that
//! render too. The port replays that order ( `visit` ): every render pass
//! writes a new version of its mirror's target, and every draw samples the
//! version and the texture matrix current when it is drawn, including
//! stale ones when a mirror faces away or was not drawn. Versions rotate
//! through three targets per mirror; their materials are built once.
use super::controls_attributes::{Controls, camera_state};
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

/// ReflectorShader: texture2DProj of the reflection with the texture matrix
/// ( u.custom[0..4] ) applied to the object-space position, blended over
/// the mirror color ( u.custom[4].rgb ). The reflection target is stored
/// from its top row.
const REFLECTOR: &str = "fn reflector_overlay(base:f32,blend:f32)->f32{return select(1.0-2.0*(1.0-base)*(1.0-blend),2.0*base*blend,base<0.5);}fn reflector()->vec4<f32>{let m=mat4x4<f32>(u.custom[0],u.custom[1],u.custom[2],u.custom[3]);let p=m*vec4(fragment_surface.local_position,1.0);let uv=p.xy/p.w;let base=textureSample(tsl_texture_0,tsl_sampler_0,vec2(uv.x,1.0-uv.y));let c=u.custom[4].rgb;return vec4(reflector_overlay(base.r,c.r),reflector_overlay(base.g,c.g),reflector_overlay(base.b,c.b),1.0);}";
const PROJECTION: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
/// Versions each mirror keeps: two written per frame and the previous one.
const SLOTS: usize = 3;
/// A camera as WebGL sees it: its world matrix and its GL projection.
#[derive(Clone, Copy)]
struct View {
    world: Matrix4,
    projection: Matrix4,
}
/// One Reflector: its node, world matrix, color, bounding radius, the
/// version and texture matrix it currently holds.
struct Mirror {
    node: Object3D,
    model: Matrix4,
    radius: f64,
    color: Color,
    version: usize,
    texture_matrix: Matrix4,
    /// Per slot, the reflection target and the material sampling it.
    targets: Vec<RenderTarget>,
    materials: Vec<Arc<Material>>,
}
/// A render: its camera, the mirrors hidden in it, and per drawn mirror the
/// slot and texture matrix it samples; it writes `output` ( mirror, slot ),
/// or the canvas.
struct Job {
    view: View,
    hidden: [bool; 2],
    draws: [Option<(usize, Matrix4)>; 2],
    output: Option<(usize, usize)>,
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    rotation: f64,
    steps: u32,
    group: Object3D,
    small: Object3D,
    mirrors: [Mirror; 2],
    /// The reflection cameras the jobs render with.
    camera: Object3D,
    resolution: f64,
    /// The reflection targets' size and the drawing buffer's.
    size: (u32, u32),
    drawing: (u32, u32),
    program: Arc<ShaderProgram>,
    sampler: wgpu::Sampler,
}
/// three's makePerspective ( WebGL clip space ) for a PerspectiveCamera.
pub(super) fn gl_perspective(fov: f64, aspect: f64, near: f64, far: f64) -> Matrix4 {
    let top = near * (PI / 180. * 0.5 * fov).tan();
    let height = 2. * top;
    let width = aspect * height;
    let left = -0.5 * width;
    let (right, bottom) = (left + width, top - height);
    let x = 2. * near / (right - left);
    let y = 2. * near / (top - bottom);
    let a = (right + left) / (right - left);
    let b = (top + bottom) / (top - bottom);
    let c = -(far + near) / (far - near);
    let d = -2. * far * near / (far - near);
    Matrix4::from_cols_array(&[x, 0., 0., 0., 0., y, 0., 0., a, b, c, -1., 0., 0., d, 0.])
}
/// The same projection with WebGPU's [ 0, 1 ] depth: z' = ( z + w ) / 2.
pub(super) fn gpu(projection: Matrix4) -> Matrix4 {
    let mut e = projection.to_cols_array();
    for column in 0..4 {
        e[column * 4 + 2] = 0.5 * e[column * 4 + 2] + 0.5 * e[column * 4 + 3];
    }
    Matrix4::from_cols_array(&e)
}
/// Object3D.lookAt for a camera at `eye` with `up`: its world rotation.
fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Quaternion {
    let mut z = eye - target;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    let z = z.normalize();
    let mut x = up.cross(z);
    if x.length_squared() == 0. {
        let z = if up.z.abs() == 1. {
            Vector3::new(z.x + 0.0001, z.y, z.z)
        } else {
            Vector3::new(z.x, z.y, z.z + 0.0001)
        }
        .normalize();
        x = up.cross(z);
    }
    let x = x.normalize();
    let y = z.cross(x);
    Quaternion::from_mat3(&glam::DMat3::from_cols(x, y, z))
}
impl Mirror {
    fn normal(&self) -> Vector3 {
        self.model.transform_vector3(Vector3::Z).normalize()
    }
    fn position(&self) -> Vector3 {
        self.model.w_axis.truncate()
    }
    /// onBeforeRender's reflection camera for `view`, with the texture
    /// matrix ( from the unclipped projection ) and the oblique projection.
    /// None when the mirror faces away.
    fn reflect(&self, view: &View) -> Option<(View, Matrix4)> {
        let normal = self.normal();
        let position = self.position();
        let camera = view.world.w_axis.truncate();
        let reflect = |v: Vector3| v - normal * (2. * v.dot(normal));
        let to_mirror = position - camera;
        if to_mirror.dot(normal) > 0. {
            return None;
        }
        let eye = -reflect(to_mirror) + position;
        let rotation = Matrix4::from_mat3(glam::DMat3::from_mat4(view.world));
        let look = rotation.transform_vector3(Vector3::new(0., 0., -1.)) + camera;
        let target = -reflect(position - look) + position;
        let up = reflect(rotation.transform_vector3(Vector3::Y));
        let world = Matrix4::from_rotation_translation(look_at(eye, target, up), eye);
        let inverse = world.inverse();
        let texture_matrix = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 0.5, 0., 0.5, 0.5, 0.5, 1.,
        ]) * view.projection
            * inverse
            * self.model;
        // The plane in the reflection camera's view space.
        let plane_normal = inverse.transform_vector3(normal).normalize();
        let point = inverse.transform_point3(position);
        let mut clip = plane_normal.extend(-point.dot(plane_normal));
        let mut e = view.projection.to_cols_array();
        let q = Vector4::new(
            (clip.x.signum() + e[8]) / e[0],
            (clip.y.signum() + e[9]) / e[5],
            -1.,
            (1. + e[10]) / e[14],
        );
        clip *= 2. / clip.dot(q);
        e[2] = clip.x;
        e[6] = clip.y;
        e[10] = clip.z + 1. - 0.003;
        e[14] = clip.w;
        Some((
            View {
                world,
                projection: Matrix4::from_cols_array(&e),
            },
            texture_matrix,
        ))
    }
    /// The frustum test of its bounding sphere and its projected z.
    fn depth(&self, view: &View) -> Option<f64> {
        let screen = view.projection * view.world.inverse();
        let frustum = Frustum::from_projection(gpu(view.projection) * view.world.inverse());
        let scale = self.model.to_scale_rotation_translation().0.max_element();
        frustum
            .intersects_sphere(Sphere {
                center: self.position(),
                radius: self.radius * scale,
            })
            .then(|| {
                let p = screen * self.position().extend(1.);
                p.z / p.w
            })
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let perspective = PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 500.,
            aspect,
            ..Default::default()
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(perspective.clone()));
        s.get_mut(c)?.position = Vector3::new(0., 75., 160.);
        let mut controls = Controls::new(None, (10., 400.), PI, true);
        controls.set_target(Vector3::new(0., 40., 0.));
        controls.update(s, c)?;
        s.background = Color::BLACK;
        let phong = |color: u32, emissive: u32, flat: bool| {
            let mut m = MeshPhongMaterial::default();
            m.properties.color = Color::from_hex(color);
            m.emissive = Color::from_hex(emissive);
            m.properties.flat_shading = flat;
            Arc::new(Material::Phong(m))
        };
        let mesh = |s: &mut Scene, g: &Arc<BufferGeometry>, m: &Arc<Material>| {
            s.insert(NodeKind::Mesh(Mesh::new(g.clone(), m.clone())))
        };
        let rotation_x = |a: f64| Quaternion::from_rotation_x(a);
        // The reflectors, at their placements.
        let circle = Arc::new(CircleGeometry::build(40., 64, 0., 2. * PI)?);
        let wall = Arc::new(PlaneGeometry::build(100., 100., 1, 1)?);
        let nearest = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let placeholder = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let view = placeholder.texture.create_view(&Default::default());
        let node = WgslFn::new("reflector", REFLECTOR, &[], Type::Vec4)?.call(&[]);
        let wgsl = NodeMaterial::new(node).wgsl_with_texture_types(&[Type::Texture], &[])?;
        let program = Arc::new(
            ShaderProgram::with_projection_and_sample_types(
                r,
                &wgsl,
                &[(&view, &nearest)],
                &[wgpu::TextureViewDimension::D2],
                &[wgpu::TextureSampleType::Float { filterable: true }],
                PROJECTION,
            )
            .await?,
        );
        let mut mirrors = vec![];
        for (geometry, radius, color, position, rotation) in [
            (
                &circle,
                40.,
                0xb5b5b5,
                Vector3::new(0., 0.5, 0.),
                rotation_x(-PI / 2.),
            ),
            (
                &wall,
                50. * 2f64.sqrt(),
                0xc1cbcb,
                Vector3::new(0., 50., -50.),
                Quaternion::IDENTITY,
            ),
        ] {
            let mut material = ShaderMaterial::new(program.clone());
            let color = Color::from_hex(color);
            material.uniforms[4] = [color.0.x as f32, color.0.y as f32, color.0.z as f32, 0.];
            let node = mesh(s, geometry, &Arc::new(Material::Shader(material)));
            let n = s.get_mut(node)?;
            n.position = position;
            n.quaternion = rotation;
            mirrors.push(Mirror {
                node,
                model: Matrix4::from_rotation_translation(rotation, position),
                radius,
                color,
                version: 0,
                texture_matrix: Matrix4::IDENTITY,
                targets: vec![],
                materials: vec![],
            });
        }
        let mirrors: [Mirror; 2] = mirrors
            .try_into()
            .map_err(|_| Error::Invalid("mirror reflectors"))?;
        // The half sphere with its cap, turning in the sphere group.
        let group = s.insert(NodeKind::Group);
        let white = phong(0xffffff, 0x8d8d8d, false);
        let cap_geometry = Arc::new(CylinderGeometry::build(
            0.1,
            15. * (PI / 180. * 30.).cos(),
            0.1,
            24,
            1,
            false,
            0.,
            2. * PI,
        )?);
        let sphere_geometry = Arc::new(SphereGeometry::with_angles(
            15.,
            24,
            24,
            PI / 2.,
            PI * 2.,
            0.,
            PI / 180. * 120.,
        )?);
        let half = mesh(s, &sphere_geometry, &white);
        let cap = mesh(s, &cap_geometry, &white);
        let n = s.get_mut(cap)?;
        n.position.y = -15. * (PI / 180. * 30.).sin() - 0.05;
        n.quaternion = rotation_x(-PI);
        s.add(half, cap)?;
        let n = s.get_mut(half)?;
        n.quaternion =
            rotation_x(-PI / 180. * 135.) * Quaternion::from_rotation_z(-PI / 180. * 20.);
        n.position.y = 7.5 + 15. * (PI / 180. * 30.).sin();
        s.add(group, half)?;
        let small = mesh(
            s,
            &Arc::new(IcosahedronGeometry::build(5., 0)?),
            &phong(0xffffff, 0x7b7b7b, true),
        );
        // The walls.
        let plane = Arc::new(PlaneGeometry::build(100.1, 100.1, 1, 1)?);
        for (color, position, rotation) in [
            (0xffffff, Vector3::new(0., 100., 0.), rotation_x(PI / 2.)),
            (0xffffff, Vector3::ZERO, rotation_x(-PI / 2.)),
            (
                0x7f7fff,
                Vector3::new(0., 50., 50.),
                Quaternion::from_rotation_y(PI),
            ),
            (
                0x00ff00,
                Vector3::new(50., 50., 0.),
                Quaternion::from_rotation_y(-PI / 2.),
            ),
            (
                0xff0000,
                Vector3::new(-50., 50., 0.),
                Quaternion::from_rotation_y(PI / 2.),
            ),
        ] {
            let node = mesh(s, &plane, &phong(color, 0, false));
            let n = s.get_mut(node)?;
            n.position = position;
            n.quaternion = rotation;
        }
        for (color, intensity, distance, position) in [
            (0xe7e7e7, 2.5, 250., Vector3::new(0., 60., 0.)),
            (0x00ff00, 0.5, 1000., Vector3::new(550., 50., 0.)),
            (0xff0000, 0.5, 1000., Vector3::new(-550., 50., 0.)),
            (0xbbbbfe, 0.5, 1000., Vector3::new(0., 50., 550.)),
        ] {
            let light = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(color),
                intensity,
                distance,
                decay: 0.,
            }));
            s.get_mut(light)?.position = position;
        }
        let camera = s.insert(NodeKind::Camera(Camera::Perspective(perspective)));
        Ok(Self {
            controls,
            time: 0.,
            rotation: 0.,
            steps: 1,
            group,
            small,
            mirrors,
            camera,
            resolution: 1.,
            size: (0, 0),
            drawing: (0, 0),
            program,
            sampler: nearest,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        self.steps = self.steps.max(u32::from(animate));
        Ok(())
    }
    /// animate(): the sphere group turns per frame; the icosahedron follows
    /// Date.now() * 0.01.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        for _ in 0..std::mem::take(&mut self.steps) {
            self.rotation -= 0.002;
        }
        s.get_mut(self.group)?.quaternion = Quaternion::from_rotation_y(self.rotation);
        let timer = self.time * 10.;
        let n = s.get_mut(self.small)?;
        n.position = Vector3::new(
            (timer * 0.1).cos() * 30.,
            (timer * 0.2).cos().abs() * 20. + 5.,
            (timer * 0.1).sin() * 30.,
        );
        n.quaternion = Euler {
            angles: Vector3::new(0., PI / 2. - timer * 0.1, timer * 0.8),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        self.controls.update(s, c)
    }
    /// The reflection targets at the drawing buffer size × resolution, each
    /// with the material sampling it ( setSize starts them cleared ).
    fn resize(&mut self, r: &Renderer) -> Result<()> {
        let (w, h) = self.drawing;
        let size = (
            ((w as f64 * self.resolution).round() as u32).max(1),
            ((h as f64 * self.resolution).round() as u32).max(1),
        );
        for m in &mut self.mirrors {
            m.targets.clear();
            m.materials.clear();
            for _ in 0..SLOTS {
                let target = RenderTarget::with_options(
                    &r.device,
                    size.0,
                    size.1,
                    RenderTargetOptions {
                        format: wgpu::TextureFormat::Rgba16Float,
                        depth_buffer: true,
                        samples: 4,
                        ..Default::default()
                    },
                )?;
                let mut program = (*self.program).clone();
                let view = target.texture.create_view(&Default::default());
                program.rebind(r, &[], &[(&view, &self.sampler)])?;
                let mut material = ShaderMaterial::new(Arc::new(program));
                material.uniforms[4] = [
                    m.color.0.x as f32,
                    m.color.0.y as f32,
                    m.color.0.z as f32,
                    0.,
                ];
                m.materials.push(Arc::new(Material::Shader(material)));
                m.targets.push(target);
            }
        }
        self.size = size;
        Ok(())
    }
    /// One render with `view`: each visible mirror's onBeforeRender in draw
    /// order ( a nested render when it faces the camera ), then the draws'
    /// versions. Nested renders are queued before the render containing them.
    fn visit(
        &mut self,
        view: View,
        hidden: [bool; 2],
        jobs: &mut Vec<Job>,
    ) -> [Option<(usize, Matrix4)>; 2] {
        let mut order: Vec<(f64, usize)> = (0..2)
            .filter(|&m| !hidden[m])
            .filter_map(|m| self.mirrors[m].depth(&view).map(|z| (z, m)))
            .collect();
        order.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        let mut draws = [None, None];
        for (_, m) in order {
            if let Some((reflected, texture_matrix)) = self.mirrors[m].reflect(&view) {
                self.mirrors[m].texture_matrix = texture_matrix;
                let mut inner = hidden;
                inner[m] = true;
                let nested = self.visit(reflected, inner, jobs);
                let slot = (self.mirrors[m].version + 1) % SLOTS;
                self.mirrors[m].version = slot;
                jobs.push(Job {
                    view: reflected,
                    hidden: inner,
                    draws: nested,
                    output: Some((m, slot)),
                });
            }
            draws[m] = Some((self.mirrors[m].version, self.mirrors[m].texture_matrix));
        }
        draws
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.drawing != (out.width, out.height) || self.mirrors[0].targets.is_empty() {
            self.drawing = (out.width, out.height);
            self.resize(r)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera else {
            return Err(Error::Invalid("mirror camera"));
        };
        let main = View {
            world,
            projection: gl_perspective(p.fov, p.aspect, p.near, p.far),
        };
        let perspective = p.clone();
        let mut jobs = vec![];
        let draws = self.visit(main, [false; 2], &mut jobs);
        jobs.push(Job {
            view: main,
            hidden: [false; 2],
            draws,
            output: None,
        });
        for job in &jobs {
            for (m, mirror) in self.mirrors.iter().enumerate() {
                let node = s.get_mut(mirror.node)?;
                node.visible = !job.hidden[m];
                if let Some((slot, matrix)) = job.draws[m]
                    && let NodeKind::Mesh(mesh) = &mut node.kind
                {
                    mesh.materials[0] = mirror.materials[slot].clone();
                    if let Material::Shader(material) = Arc::make_mut(&mut mesh.materials[0]) {
                        let e = matrix.to_cols_array();
                        for k in 0..4 {
                            material.uniforms[k] = [
                                e[k * 4] as f32,
                                e[k * 4 + 1] as f32,
                                e[k * 4 + 2] as f32,
                                e[k * 4 + 3] as f32,
                            ];
                        }
                    }
                }
            }
            match job.output {
                Some((m, slot)) => {
                    let (_, rotation, translation) = job.view.world.to_scale_rotation_translation();
                    let n = s.get_mut(self.camera)?;
                    n.position = translation;
                    n.quaternion = rotation;
                    n.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        projection_override: Some(gpu(job.view.projection)),
                        ..perspective.clone()
                    }));
                    s.update()?;
                    r.render(s, self.camera, &self.mirrors[m].targets[slot])?;
                }
                None => r.render(s, c, out)?,
            }
        }
        for mirror in &self.mirrors {
            s.get_mut(mirror.node)?.visible = true;
        }
        Ok(true)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// Mirrors › resolution: the reflection targets' scale.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("mirror parameter"));
        }
        self.resolution = value as f64;
        self.mirrors[0].targets.clear();
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.steps += 1;
    }
}
