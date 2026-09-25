//! Planar ShadowMesh shadows, the dynamic InstancedMesh, the depth texture,
//! render to texture and the normal-mapped head through its composer.
use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use crate::postprocessing::Effect;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, float, uv, vec4};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn euler(x: f64, y: f64, z: f64) -> Quaternion {
    Euler {
        angles: Vector3::new(x, y, z),
        order: EulerOrder::XYZ,
    }
    .quaternion()
}
async fn texture(path: &str, srgb: bool) -> Result<Texture> {
    let mut t = decode_texture_image(&fetch(&format!("{ASSETS}/{path}")).await?).await?;
    t.srgb = srgb;
    t.mipmap_filter = Some(Filter::Linear);
    Ok(t)
}
/// A ShaderMaterial program writing `color`, with the given vertex projection.
async fn program(
    r: &Renderer,
    color: crate::tsl::Node,
    projection: &str,
    textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
) -> Result<Arc<ShaderProgram>> {
    Ok(Arc::new(
        ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(textures.len())?,
            &[],
            textures,
            projection,
        )
        .await?,
    ))
}
const PROJECT: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";

/// webgl_shadowmesh: the objects, their ShadowMeshes and the light switch.
struct Shadows {
    objects: [Object3D; 5],
    shadows: [Object3D; 5],
    ground: Object3D,
    light: Object3D,
    arrows: Vec<Object3D>,
    bulb: [Object3D; 2],
    directional: bool,
    /// cube x/y, cylinder y/z, torus x/y and pyramid y rotations.
    rotation: [f64; 7],
    horizontal: f64,
    vertical: f64,
}
/// webgl_instancing_dynamic: the 10,000 instances and the color tween.
struct Instancing {
    mesh: Object3D,
    seeds: Vec<f64>,
    /// Color.getHex() of each HSL base color: the tweens restart from these.
    base: Vec<u32>,
    /// The camera's up.x, set after lookAt for the next frame.
    up_x: f64,
}
/// webgl_depth_texture: the scene target with its depth texture and the post pass.
struct DepthView {
    target: RenderTarget,
    post: Effect,
    /// The same pass reading one sample of a multisampled depth texture. WebGL
    /// resolves depth to one implementation-chosen sample; sample 3 is the closest
    /// to the reference GPU's.
    post_multisampled: Effect,
    /// format, type, samples.
    params: [f64; 3],
    pending_update: bool,
}
/// webgl_rtt: the render-to-texture scene, the screen quad and the output.
struct Rtt {
    scene: Scene,
    camera: Object3D,
    screen: Scene,
    screen_camera: Object3D,
    target: RenderTarget,
    tori: [Object3D; 2],
    quad: Object3D,
    time: f64,
    delta: f64,
    output: Option<RenderTarget>,
}
/// webgl_materials_normalmap: the composer's targets and passes.
struct Composer {
    material: Object3D,
    normal_map: Arc<Texture>,
    targets: [RenderTarget; 2],
    bleach: Effect,
    color: Effect,
    output: Effect,
    fxaa: Effect,
    params: [f64; 2],
}
pub struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    controls: Option<Controls>,
    shadows: Option<Shadows>,
    instancing: Option<Instancing>,
    depth: Option<DepthView>,
    rtt: Option<Rtt>,
    composer: Option<Composer>,
    pointer: Option<Vector2>,
    /// The light button's click, applied before the next frame.
    toggle: bool,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            278 => (55., 1., 3000., Vector3::new(0., 2.5, 10.)),
            279 => (60., 0.1, 100., Vector3::new(10., 10., 10.)),
            280 => (70., 0.01, 50., Vector3::new(0., 0., 4.)),
            281 => (30., 1., 10000., Vector3::new(0., 0., 100.)),
            _ => (27., 0.1, 100., Vector3::new(0., 0., 12.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = position;
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            controls: None,
            shadows: None,
            instancing: None,
            depth: None,
            rtt: None,
            composer: None,
            pointer: None,
            toggle: false,
        };
        match id {
            278 => d.shadow_scene(s, r).await?,
            279 => d.instancing_scene(s, c, r).await?,
            280 => d.depth_scene(s, c, r).await?,
            281 => d.rtt_scene(s, r).await?,
            _ => d.normalmap_scene(s, c, r).await?,
        }
        Ok(d)
    }
    /// ArrowHelper( direction, origin, 0.9, 0xffff00, 0.25, 0.08 ): a line and a
    /// five-sided cone turned from +Y to the direction.
    fn arrow(s: &mut Scene, direction: Vector3, origin: Vector3) -> Result<Object3D> {
        let (length, head_length, head_width) = (0.9, 0.25, 0.08);
        let group = s.insert(NodeKind::Group);
        let mut line_geometry = BufferGeometry::default();
        line_geometry.set_attribute(
            "position",
            crate::geometry::Attribute::F32(crate::attribute::BufferAttribute::new(
                vec![0., 0., 0., 0., 1., 0.],
                3,
                false,
            )?),
        );
        let mut line_material = LineBasicMaterial::default();
        line_material.properties.color = Color::from_hex(0xffff00);
        let line = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(line_geometry),
            material: Arc::new(Material::Line(line_material)),
            segments: false,
        }));
        s.get_mut(line)?.scale = Vector3::new(1., f64::max(0.0001, length - head_length), 1.);
        let mut cone_geometry = CylinderGeometry::build(0., 0.5, 1., 5, 1, false, 0., TAU)?;
        cone_geometry.translate(Vector3::new(0., -0.5, 0.))?;
        let mut cone_material = MeshBasicMaterial::default();
        cone_material.properties.color = Color::from_hex(0xffff00);
        let cone = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(cone_geometry),
            Arc::new(Material::Basic(cone_material)),
        )));
        let n = s.get_mut(cone)?;
        n.scale = Vector3::new(head_width, head_length, head_width);
        n.position.y = length;
        s.add(group, line)?;
        s.add(group, cone)?;
        let n = s.get_mut(group)?;
        n.position = origin;
        n.quaternion = if direction.y > 0.99999 {
            Quaternion::IDENTITY
        } else if direction.y < -0.99999 {
            Quaternion::from_axis_angle(Vector3::X, PI)
        } else {
            Quaternion::from_axis_angle(
                Vector3::new(direction.z, 0., -direction.x).normalize(),
                direction.y.acos(),
            )
        };
        Ok(group)
    }
    async fn shadow_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0x0096ff);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(5., 7., -1.);
        let sun = Vector3::new(5., 7., -1.);
        let direction = (Vector3::ZERO - sun).normalize();
        let mut arrows = vec![];
        for dy in [0., 0.2, -0.2] {
            arrows.push(Self::arrow(s, direction, sun + Vector3::new(0., dy, 0.))?);
        }
        let basic = |hex: u32| {
            let mut m = MeshBasicMaterial::default();
            m.properties.color = Color::from_srgb(
                ((hex >> 16) & 255) as f64 / 255.,
                ((hex >> 8) & 255) as f64 / 255.,
                (hex & 255) as f64 / 255.,
            );
            Arc::new(Material::Basic(m))
        };
        let sphere = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.09, 32, 16)?),
            basic(0xffffff),
        )));
        s.get_mut(sphere)?.visible = false;
        let holder = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(CylinderGeometry::build(
                0.05, 0.05, 0.13, 32, 1, false, 0., TAU,
            )?),
            basic(0x4b4b4b),
        )));
        s.get_mut(holder)?.visible = false;
        let lambert = |hex: u32, emissive: u32| {
            let mut m = MeshLambertMaterial {
                emissive: Color::from_hex(emissive),
                ..Default::default()
            };
            m.properties.color = Color::from_hex(hex);
            Arc::new(Material::Lambert(m))
        };
        let phong = |hex: u32, emissive: u32, flat: bool, shininess: f64| {
            let mut m = MeshPhongMaterial {
                emissive: Color::from_hex(emissive),
                shininess,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(hex);
            m.properties.flat_shading = flat;
            Arc::new(Material::Phong(m))
        };
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(30., 0.01, 40.)?),
            lambert(0x008200, 0),
        )));
        let default_shininess = MeshPhongMaterial::default().shininess;
        let objects: Vec<(BufferGeometry, Arc<Material>, Vector3)> = vec![
            (
                BoxGeometry::build(1., 1., 1.)?,
                lambert(0xff0000, 0x200000),
                Vector3::new(0., 0., -1.),
            ),
            (
                CylinderGeometry::build(0.3, 0.3, 2., 32, 1, false, 0., TAU)?,
                phong(0x0000ff, 0x000020, false, default_shininess),
                Vector3::new(0., 0., -2.5),
            ),
            (
                TorusGeometry::build(1., 0.2, 10, 16, TAU, 0., TAU)?,
                phong(0xff00ff, 0x200020, false, default_shininess),
                Vector3::new(0., 0., -6.),
            ),
            (
                SphereGeometry::build(0.5, 20, 10)?,
                phong(0xffffff, 0x222222, false, default_shininess),
                Vector3::new(4., 0.5, 2.),
            ),
            (
                CylinderGeometry::build(0., 0.5, 2., 4, 1, false, 0., TAU)?,
                phong(0xffff00, 0x440000, true, 0.),
                Vector3::new(-4., 1., 2.),
            ),
        ];
        // ShadowMesh: black at 0.6 opacity, no depth writes, each pixel darkened
        // once by the stencil (equal 0, increment on pass). The projection keeps
        // the shadow matrix's w.
        let shadow_program = program(
            r,
            vec4(crate::tsl::vec3(float(0.), float(0.), float(0.)), float(0.6)),
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=u.projection*u.view*u.model*vec4(surface.local_position,1.0);return out;}",
            &[],
        )
        .await?;
        let face = wgpu::StencilFaceState {
            compare: wgpu::CompareFunction::Equal,
            fail_op: wgpu::StencilOperation::Keep,
            depth_fail_op: wgpu::StencilOperation::Keep,
            pass_op: wgpu::StencilOperation::IncrementClamp,
        };
        let mut shadow_material = ShaderMaterial::new(shadow_program);
        shadow_material.properties.transparent = true;
        shadow_material.properties.depth_write = false;
        shadow_material.properties.stencil = Some(wgpu::StencilState {
            front: face,
            back: face,
            read_mask: 0xff,
            write_mask: 0xff,
        });
        let shadow_material = Arc::new(Material::Shader(shadow_material));
        let mut handles = vec![];
        let mut shadows = vec![];
        for (geometry, material, position) in objects {
            let geometry = Arc::new(geometry);
            let h = s.insert(NodeKind::Mesh(Mesh::new(geometry.clone(), material)));
            s.get_mut(h)?.position = position;
            handles.push(h);
            let shadow = s.insert(NodeKind::Mesh(Mesh::new(geometry, shadow_material.clone())));
            let n = s.get_mut(shadow)?;
            n.frustum_culled = false;
            n.matrix_auto_update = false;
            shadows.push(shadow);
        }
        self.shadows = Some(Shadows {
            objects: handles.try_into().map_err(|_| Error::Invalid("objects"))?,
            shadows: shadows.try_into().map_err(|_| Error::Invalid("shadows"))?,
            ground,
            light,
            arrows,
            bulb: [sphere, holder],
            directional: true,
            rotation: [0.; 7],
            horizontal: 0.,
            vertical: 0.,
        });
        Ok(())
    }
    /// ShadowMesh.update(): the planar projection onto y = 0.01 from the light's
    /// homogeneous position, times the mesh's world matrix from its last render.
    fn shadow_step(&mut self, s: &mut Scene, dt: f64) -> Result<()> {
        let k = self.shadows.as_mut().ok_or(Error::Invalid("shadows"))?;
        let r = &mut k.rotation;
        r[0] += dt;
        r[1] += dt;
        r[2] += dt;
        r[3] -= dt;
        r[4] -= dt;
        r[5] -= dt;
        r[6] += 0.5 * dt;
        k.horizontal += 0.5 * dt;
        if k.horizontal > TAU {
            k.horizontal -= TAU;
        }
        k.vertical += 1.5 * dt;
        if k.vertical > TAU {
            k.vertical -= TAU;
        }
        let [cube, cylinder, torus, _, pyramid] = k.objects;
        let (h, v) = (k.horizontal, k.vertical);
        let n = s.get_mut(cube)?;
        n.quaternion = euler(r[0], r[1], 0.);
        n.position = Vector3::new(h.sin() * 4., v.sin() * 2. + 2.9, n.position.z);
        let n = s.get_mut(cylinder)?;
        n.quaternion = euler(0., r[2], r[3]);
        n.position = Vector3::new(h.sin() * -4., v.sin() * 2. + 3.1, n.position.z);
        let n = s.get_mut(torus)?;
        n.quaternion = euler(r[4], r[5], 0.);
        n.position = Vector3::new(h.cos() * 4., v.cos() * 2. + 3.3, n.position.z);
        s.get_mut(pyramid)?.quaternion = euler(0., r[6], 0.);
        let light = s.get(k.light)?.position;
        let w = if k.directional { 0.001 } else { 0.9 };
        let l = light.extend(w);
        let (normal, constant) = (Vector3::Y, 0.01);
        let dot = normal.dot(light) - constant * w;
        let e = |i: usize, j: usize| -> f64 {
            let li = l[i];
            let nj = if j < 3 { normal[j] } else { -constant };
            (if i == j { dot } else { 0. }) - li * nj
        };
        // Column-major: element ( row i, column j ).
        let shadow = Matrix4::from_cols_array(&std::array::from_fn(|k| e(k % 4, k / 4)));
        for (object, shadow_mesh) in k.objects.iter().zip(k.shadows) {
            let world = s.get(*object)?.matrix_world;
            let n = s.get_mut(shadow_mesh)?;
            n.matrix = shadow * world;
            n.matrix_world_needs_update = true;
        }
        Ok(())
    }
    fn toggle_light(&mut self, s: &mut Scene) -> Result<()> {
        let k = self.shadows.as_mut().ok_or(Error::Invalid("shadows"))?;
        k.directional = !k.directional;
        let d = k.directional;
        s.background = Color::from_hex(if d { 0x0096ff } else { 0x000000 });
        if let NodeKind::Mesh(m) = &mut s.get_mut(k.ground)?.kind {
            Arc::make_mut(&mut m.materials[0]).properties_mut().color =
                Color::from_hex(if d { 0x008200 } else { 0x969696 });
        }
        let position = if d {
            Vector3::new(5., 7., -1.)
        } else {
            Vector3::new(0., 6., -2.)
        };
        s.get_mut(k.light)?.position = position;
        for &a in &k.arrows {
            s.get_mut(a)?.visible = d;
        }
        s.get_mut(k.bulb[0])?.position = position;
        s.get_mut(k.bulb[0])?.visible = !d;
        s.get_mut(k.bulb[1])?.position = position + Vector3::new(0., 0.12, 0.);
        s.get_mut(k.bulb[1])?.visible = !d;
        Ok(())
    }
    async fn instancing_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0xadd8e6);
        s.tone_mapping = ToneMapping::Neutral;
        s.environment = Some(super::room_environment::environment(r)?);
        s.look_at(c, Vector3::ZERO)?;
        let mut map = texture("shadow-rtt/edge3.jpg", true).await?;
        map.mipmap_filter = Some(Filter::Linear);
        let mut material = MeshStandardMaterial::default();
        material.properties.map = Some(Arc::new(map));
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 1., 1.)?),
            Arc::new(Material::Standard(material)),
        )));
        let amount = 100;
        let offset = (amount as f64 - 1.) / 2.;
        let cyan = Color::from_hex(0x00ffff).0;
        let (mut instances, mut seeds, mut base) = (vec![], vec![], vec![]);
        for x in 0..amount {
            for z in 0..amount {
                let position = Vector3::new(offset - x as f64, 0., offset - z as f64);
                let s1 = 0.5 + random(&mut self.seed) * 0.5;
                let l = 0.5 + random(&mut self.seed) * 0.5;
                let color = Color::from_hsl(1., s1, l);
                base.push(color.to_hex());
                instances.push(Instance {
                    matrix: Matrix4::from_scale_rotation_translation(
                        Vector3::new(1., 2., 1.),
                        Quaternion::IDENTITY,
                        position,
                    ),
                    color: Color(color.0 * cyan),
                });
                seeds.push(random(&mut self.seed));
            }
        }
        s.get_mut(mesh)?.instances = instances;
        self.instancing = Some(Instancing {
            mesh,
            seeds,
            base,
            up_x: 0.,
        });
        Ok(())
    }
    /// animate(): the camera path and up tilt, the instance heights, and the
    /// color tween started every 3 s (2 s, Sinusoidal.In) and reset on completion.
    fn instancing_step(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let k = self
            .instancing
            .as_mut()
            .ok_or(Error::Invalid("instancing"))?;
        let time = self.time;
        let n = s.get_mut(c)?;
        n.position = Vector3::new(
            (time / 4.).sin() * 10.,
            8. + (time / 2.).cos() * 2.,
            (time / 4.).cos() * 10.,
        );
        n.up = Vector3::new(k.up_x, 1., 0.);
        let target = Vector3::new((time / 4.).sin() * -8., 0., (time / 2.).cos() * -8.);
        s.look_at(c, target)?;
        k.up_x = (time / 400.).sin();
        // setInterval( startTween, 3000 ): tween n runs over [3n, 3n + 2).
        let started = (time / 3.).floor();
        let progress = (time - 3. * started) / 2.;
        let t = if started >= 1. && progress < 1. {
            1. - ((1. - progress) * PI / 2.).sin()
        } else {
            0.
        };
        let completed = if time >= 5. {
            ((time - 2.) / 3.).floor() as usize
        } else {
            0
        };
        let colors = [0x00ffff, 0xffff00, 0xff00ff].map(|hex| Color::from_hex(hex).0);
        let (current, next) = (colors[completed % 3], colors[(completed + 1) % 3]);
        let n = s.get_mut(k.mesh)?;
        for (i, instance) in n.instances.iter_mut().enumerate() {
            let mut position = instance.matrix.w_axis.truncate();
            // The matrices are stored as Float32 and decomposed each frame.
            position.y = ((((time + k.seeds[i]) * 2. + k.seeds[i]).sin().abs()) as f32) as f64;
            instance.matrix.w_axis = position.extend(1.);
            if t > 0. {
                let f = position.length() / 75.;
                let base = Color::from_hex(k.base[i]).0;
                instance.color = Color(base * if f <= t { next } else { current });
            }
        }
        Ok(())
    }
    async fn depth_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let mut material = MeshBasicMaterial::default();
        material.properties.color = Color::from_hex(0x0000ff);
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(1., 0.3, 128, 64, 2, 3)?),
            Arc::new(Material::Basic(material)),
        )));
        let mut instances = vec![];
        for _ in 0..50 {
            let angle = random(&mut self.seed) * 2. * PI;
            let z = random(&mut self.seed) * 2. - 1.;
            let z_scale = (1. - z * z).sqrt() * 5.;
            let position = Vector3::new(angle.cos() * z_scale, angle.sin() * z_scale, z * 5.);
            let (x, y, w) = (
                random(&mut self.seed),
                random(&mut self.seed),
                random(&mut self.seed),
            );
            instances.push(Instance {
                matrix: Matrix4::from_rotation_translation(euler(x, y, w), position),
                color: Color::WHITE,
            });
        }
        s.get_mut(mesh)?.instances = instances;
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        let target = Self::depth_target(r, 1, 1, 0.)?;
        let multisampled = Self::depth_target(r, 1, 1, 4.)?;
        // post-frag: 1 − the depth linearized between the near and far planes. The
        // window depth has the same hyperbolic form as WebGL's gl_FragCoord.z. The
        // raw value is written, as the ShaderMaterial has no color-space conversion.
        let post_source = |kind: &str, index: u32| {
            format!(
                "@group(1) @binding(0) var depth_texture:{kind};
fn effect(uv:vec2<f32>)->vec4<f32>{{let size=vec2<f32>(textureDimensions(depth_texture));let q=vec2<i32>(min(uv*size,size-1.0));let d=textureLoad(depth_texture,q,{index});let near=params[0].x;let far=params[0].y;let view_z=(near*far)/((far-near)*d-far);let depth=(view_z+near)/(near-far);return vec4(vec3(1.0-depth),1.0);}}"
            )
        };
        let post = Effect::with_depth(
            r,
            wgpu::TextureFormat::Rgba8Unorm,
            &post_source("texture_depth_2d", 0),
            target
                .depth_view
                .as_ref()
                .ok_or(Error::Invalid("depth view"))?,
        )
        .await?;
        let post_multisampled = Effect::with_multisampled_depth(
            r,
            wgpu::TextureFormat::Rgba8Unorm,
            &post_source("texture_depth_multisampled_2d", 3),
            multisampled
                .depth_view
                .as_ref()
                .ok_or(Error::Invalid("depth view"))?,
        )
        .await?;
        self.depth = Some(DepthView {
            target,
            post,
            post_multisampled,
            params: [0., 0., 0.],
            pending_update: false,
        });
        Ok(())
    }
    /// The WebGLRenderTarget with its DepthTexture. WebGPU multisamples at four
    /// samples: any nonzero sample count uses four.
    fn depth_target(r: &Renderer, width: u32, height: u32, samples: f64) -> Result<RenderTarget> {
        RenderTarget::with_options(
            &r.device,
            width,
            height,
            RenderTargetOptions {
                samples: if samples > 0. { 4 } else { 1 },
                format: wgpu::TextureFormat::Rgba8Unorm,
                encode_srgb: true,
                ..Default::default()
            },
        )
    }
    async fn rtt_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let (w, h, _) = viewport_css();
        // The render target and the orthographic camera use the page size at load.
        let target = RenderTarget::with_options(
            &r.device,
            w as u32,
            h as u32,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..Default::default()
            },
        )?;
        let ortho = |scene: &mut Scene| -> Result<Object3D> {
            let c = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                left: w / -2.,
                right: w / 2.,
                top: h / 2.,
                bottom: h / -2.,
                near: 1.,
                far: 1000.,
                ..Default::default()
            })));
            scene.get_mut(c)?.position.z = 500.;
            Ok(c)
        };
        let mut rtt = Scene::new();
        rtt.background = Color::BLACK;
        let camera = ortho(&mut rtt)?;
        for (hex, intensity, z) in [(0xffffff, 3., 1.), (0xffd5d5, 4.5, -1.)] {
            let light = rtt.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(hex),
                intensity,
                target: Vector3::ZERO,
            }));
            rtt.get_mut(light)?.position = Vector3::new(0., 0., z);
        }
        let plane = Arc::new(PlaneGeometry::build(w, h, 1, 1)?);
        // fragment_shader_pass_1: ( r, g, time ), written raw into the linear target.
        let pass = program(
            r,
            crate::tsl::WgslFn::new(
                "rtt_pass",
                "fn rtt_pass(uv:vec2<f32>,time:f32)->vec4<f32>{var r=uv.x;if uv.y<0.5 {r=0.0;}var g=uv.y;if uv.x<0.5 {g=0.0;}return vec4(r,g,time,1.0);}",
                &[crate::tsl::Type::Vec2, crate::tsl::Type::Float],
                crate::tsl::Type::Vec4,
            )?
            .call(&[uv(), crate::tsl::uniform(0, crate::tsl::Type::Vec4).x()]),
            PROJECT,
            &[],
        )
        .await?;
        let pass_material = ShaderMaterial::new(pass);
        let quad = rtt.insert(NodeKind::Mesh(Mesh::new(
            plane.clone(),
            Arc::new(Material::Shader(pass_material)),
        )));
        rtt.get_mut(quad)?.position.z = -100.;
        let torus = Arc::new(TorusGeometry::build(100., 25., 15, 30, TAU, 0., TAU)?);
        let mut tori = vec![];
        for (hex, specular, position, scale) in [
            (0x9c9c9c, 0xffaa00, Vector3::new(0., 0., 100.), 1.5),
            (0x9c0000, 0xff2200, Vector3::new(0., 150., 100.), 0.75),
        ] {
            let mut m = MeshPhongMaterial {
                specular: Color::from_hex(specular),
                shininess: 5.,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(hex);
            let h = rtt.insert(NodeKind::Mesh(Mesh::new(
                torus.clone(),
                Arc::new(Material::Phong(m)),
            )));
            let n = rtt.get_mut(h)?;
            n.position = position;
            n.scale = Vector3::splat(scale);
            tori.push(h);
        }
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let view = target.texture.create_view(&Default::default());
        // fragment_shader_screen: the texture with colorspace_fragment, i.e. encoded
        // by the output like the other materials.
        let mut screen = Scene::new();
        screen.background = Color::BLACK;
        let screen_camera = ortho(&mut screen)?;
        let screen_program = program(
            r,
            // Render targets are stored top row first: sample ( u, 1 − v ) as WebGL's
            // bottom-up render target texture.
            crate::tsl::Texture::External(0)
                .sample(crate::tsl::vec2(uv().x(), float(1.) - uv().y())),
            PROJECT,
            &[(&view, &sampler)],
        )
        .await?;
        let mut screen_material = ShaderMaterial::new(screen_program);
        screen_material.properties.depth_write = false;
        let screen_quad = screen.insert(NodeKind::Mesh(Mesh::new(
            plane,
            Arc::new(Material::Shader(screen_material)),
        )));
        screen.get_mut(screen_quad)?.position.z = -100.;
        // The spheres: MeshBasicMaterial with the render target as map (NoColorSpace).
        let sphere_program = program(
            r,
            // Render targets are stored top row first: sample ( u, 1 − v ) as WebGL's
            // bottom-up render target texture.
            crate::tsl::Texture::External(0)
                .sample(crate::tsl::vec2(uv().x(), float(1.) - uv().y())),
            PROJECT,
            &[(&view, &sampler)],
        )
        .await?;
        let sphere_material = Arc::new(Material::Shader(ShaderMaterial::new(sphere_program)));
        let sphere = Arc::new(SphereGeometry::build(10., 64, 32)?);
        for j in 0..5 {
            for i in 0..5 {
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    sphere.clone(),
                    sphere_material.clone(),
                )));
                let n = s.get_mut(h)?;
                n.position = Vector3::new((i as f64 - 2.) * 20., (j as f64 - 2.) * 20., 0.);
                n.quaternion = Quaternion::from_rotation_y(-PI / 2.);
            }
        }
        self.rtt = Some(Rtt {
            scene: rtt,
            camera,
            screen,
            screen_camera,
            target,
            tori: tori.try_into().map_err(|_| Error::Invalid("tori"))?,
            quad,
            time: 0.,
            delta: 0.01,
            output: None,
        });
        Ok(())
    }
    async fn normalmap_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0x494949);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.,
        }));
        let point = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 30.,
            distance: 0.,
            decay: 2.,
        }));
        s.get_mut(point)?.position = Vector3::new(0., 0., 6.);
        let directional = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(directional)?.position = Vector3::new(1., -0.5, -1.);
        let map = texture("shadow-rtt/Map-COL.jpg", true).await?;
        let specular = texture("shadow-rtt/Map-SPEC.jpg", true).await?;
        let normal_map = Arc::new(
            texture(
                "LeePerrySmith/Infinite-Level_02_Tangent_SmoothUV.jpg",
                false,
            )
            .await?,
        );
        let mut material = MeshPhongMaterial {
            specular: Color::from_hex(0x222222),
            shininess: 35.,
            specular_map: Some(Arc::new(specular)),
            normal_map: Some(normal_map.clone()),
            ..Default::default()
        };
        material.properties.color = Color::from_hex(0xefefef);
        material.properties.map = Some(Arc::new(map));
        let (asset, buffers, images) = load_asset("/web/models/LeePerrySmith.glb").await?;
        let meshes = crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        let geometry = match &s.get(meshes[0])?.kind {
            NodeKind::Mesh(m) => m.geometry.clone(),
            _ => return Err(Error::Invalid("LeePerrySmith mesh")),
        };
        for h in meshes {
            s.dispose(h)?;
        }
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            geometry,
            Arc::new(Material::Phong(material)),
        )));
        s.get_mut(mesh)?.position.y = -0.5;
        let mut controls = Controls::new(Some(0.05), (8., 50.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        let half = |r: &Renderer| {
            RenderTarget::with_options(
                &r.device,
                1,
                1,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    samples: 1,
                    ..Default::default()
                },
            )
        };
        let format = wgpu::TextureFormat::Rgba16Float;
        // BleachBypassShader (opacity 0.2) with three's luminance weights.
        let bleach = Effect::new(
            r,
            format,
            "fn effect(uv:vec2<f32>)->vec4<f32>{let base=textureSample(input_texture,input_sampler,uv);let lum=dot(vec3(0.2126729,0.7151522,0.0721750),base.rgb);let blend=vec3(lum);let l=min(1.0,max(0.0,10.0*(lum-0.45)));let result1=2.0*base.rgb*blend;let result2=1.0-2.0*(1.0-blend)*(1.0-base.rgb);let new_color=mix(result1,result2,l);let a2=0.2*base.a;return vec4(a2*new_color+(1.0-a2)*base.rgb,base.a);}",
        )
        .await?;
        // ColorCorrectionShader: mulRGB × pow( rgb + addRGB, powRGB ).
        let color = Effect::new(
            r,
            format,
            "fn effect(uv:vec2<f32>)->vec4<f32>{let c=textureSample(input_texture,input_sampler,uv);return vec4(vec3(1.1)*pow(c.rgb,vec3(1.4,1.45,1.45)),c.a);}",
        )
        .await?;
        // OutputPass: no tone mapping, then the sRGB transfer.
        let output = Effect::new(
            r,
            format,
            "fn effect(uv:vec2<f32>)->vec4<f32>{let c=textureSample(input_texture,input_sampler,uv);let s=select(c.rgb*12.92,pow(c.rgb,vec3(0.41666))*1.055-vec3(0.055),c.rgb>vec3(0.0031308));return vec4(s,c.a);}",
        )
        .await?;
        // FXAAPass to the canvas; its values are already display-encoded.
        let fxaa = Effect::new(
            r,
            wgpu::TextureFormat::Rgba8Unorm,
            &format!(
                "{}\nfn effect(uv:vec2<f32>)->vec4<f32>{{return tsl_fxaa(input_texture,input_sampler,uv,1.0/vec2<f32>(textureDimensions(input_texture)));}}",
                include_str!("../tsl/fxaa.wgsl")
            ),
        )
        .await?;
        self.composer = Some(Composer {
            material: mesh,
            normal_map,
            targets: [half(r)?, half(r)?],
            bleach,
            color,
            output,
            fxaa,
            params: [1., 1.],
        });
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let delta = t - self.last;
        let steps = (delta * 60.).round().max(0.) as usize;
        self.last = t;
        match self.id {
            278 => {
                if std::mem::take(&mut self.toggle) {
                    self.toggle_light(s)?;
                }
                self.shadow_step(s, delta)?
            }
            279 => self.instancing_step(s, c)?,
            281 => {
                let k = self.rtt.as_mut().ok_or(Error::Invalid("rtt"))?;
                let mouse = self.pointer.map_or(Vector2::ZERO, |p| {
                    let (w, h, _) = viewport_css();
                    Vector2::new(p.x - w / 2., p.y - h / 2.)
                });
                let time = t * 1000. * 0.0015;
                for _ in 0..steps {
                    let n = s.get_mut(c)?;
                    n.position.x += (mouse.x - n.position.x) * 0.05;
                    n.position.y += (-mouse.y - n.position.y) * 0.05;
                    if k.time > 1. || k.time < 0. {
                        k.delta *= -1.;
                    }
                    k.time += k.delta;
                }
                s.look_at(c, Vector3::ZERO)?;
                k.scene.get_mut(k.tori[0])?.quaternion = Quaternion::from_rotation_y(-time);
                k.scene.get_mut(k.tori[1])?.quaternion =
                    Quaternion::from_rotation_y(-time + PI / 2.);
                if let NodeKind::Mesh(m) = &mut k.scene.get_mut(k.quad)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0] = [k.time as f32, 0., 0., 0.];
                }
            }
            282 => {
                let k = self.composer.as_ref().ok_or(Error::Invalid("composer"))?;
                let [enabled, scale] = k.params;
                // The material changes only with the controls.
                let map = (enabled > 0.5).then(|| k.normal_map.clone());
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(k.material)?.kind
                    && let Material::Phong(m) = mesh.materials[0].as_ref()
                    && (m.normal_map.is_some() != map.is_some()
                        || m.normal_scale != Vector2::splat(scale))
                    && let Material::Phong(m) = Arc::make_mut(&mut mesh.materials[0])
                {
                    m.normal_map = map;
                    m.normal_scale = Vector2::splat(scale);
                }
                if let Some(controls) = &mut self.controls {
                    controls.frame_update(s, c)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        match self.id {
            280 => {
                let k = self.depth.as_mut().ok_or(Error::Invalid("depth"))?;
                if (k.target.width, k.target.height) != (out.width, out.height) || k.pending_update
                {
                    k.target = Self::depth_target(r, out.width, out.height, k.params[2])?;
                    let depth = k
                        .target
                        .depth_view
                        .as_ref()
                        .ok_or(Error::Invalid("depth view"))?;
                    if k.params[2] > 0. {
                        k.post_multisampled.set_depth(r, depth)?;
                    } else {
                        k.post.set_depth(r, depth)?;
                    }
                    k.pending_update = false;
                }
                r.render(s, c, &k.target)?;
                let (near, far) = match s.camera(c)?.0 {
                    Camera::Perspective(p) => (p.near, p.far),
                    _ => (0.01, 50.),
                };
                let post = if k.params[2] > 0. {
                    &mut k.post_multisampled
                } else {
                    &mut k.post
                };
                post.parameters[0] = [near as f32, far as f32, 0., 0.];
                post.apply(r, &k.target, None, out)?;
                // controls.update() follows the render in animate().
                if let Some(controls) = &mut self.controls {
                    controls.frame_update(s, c)?;
                }
                Ok(true)
            }
            281 => {
                let k = self.rtt.as_mut().ok_or(Error::Invalid("rtt"))?;
                r.render(&mut k.scene, k.camera, &k.target)?;
                if k.output.as_ref().is_none_or(|t| {
                    (t.width, t.height, t.options.samples)
                        != (out.width, out.height, out.options.samples)
                }) {
                    k.output = Some(RenderTarget::with_options(
                        &r.device,
                        out.width,
                        out.height,
                        out.options.clone(),
                    )?);
                }
                let target = k.output.as_mut().ok_or(Error::Invalid("rtt output"))?;
                // autoClear is false: the screen quad and the spheres share one clear.
                target.set_load_color(false);
                r.render(&mut k.screen, k.screen_camera, target)?;
                target.set_load_color(true);
                r.render(s, c, target)?;
                target.set_load_color(false);
                Ok(true)
            }
            282 => {
                let k = self.composer.as_mut().ok_or(Error::Invalid("composer"))?;
                for t in &mut k.targets {
                    if (t.width, t.height) != (out.width, out.height) {
                        t.set_size(&r.device, out.width, out.height)?;
                    }
                }
                let [a, b] = &k.targets;
                r.render(s, c, a)?;
                k.bleach.apply(r, a, None, b)?;
                k.color.apply(r, b, None, a)?;
                k.output.apply(r, a, None, b)?;
                k.fxaa.apply(r, b, None, out)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.rtt.as_ref().and_then(|k| k.output.as_ref())
    }
    /// Absolute CSS pointer moves.
    pub fn draw(&mut self, _kind: u32, x: f64, y: f64) {
        self.pointer = Some(Vector2::new(x, y));
    }
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
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            if self.id == 282 {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        match (self.id, index) {
            (278, 0) => self.toggle = !self.toggle,
            (280, 0..=2) => {
                let k = self.depth.as_mut().ok_or(Error::Invalid("depth"))?;
                k.params[index] = v;
                // setupRenderTarget(): a new target for every change.
                k.pending_update = true;
            }
            (282, 0..=1) => {
                self.composer
                    .as_mut()
                    .ok_or(Error::Invalid("composer"))?
                    .params[index] = v
            }
            _ => return Err(Error::Invalid("shadow/rtt parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
