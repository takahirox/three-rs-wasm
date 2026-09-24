//! Orbit and map controls, the camera/helper split view, custom vertex attributes
//! and the draw-range connection lines from the pinned WebGL examples.
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::tsl::Node;
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    tsl::{self, *},
};
use std::f64::consts::{PI, TAU};
use std::sync::Arc;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
pub(super) fn call(
    name: &str,
    source: &str,
    args: &[Type],
    out: Type,
    nodes: &[Node],
) -> Result<Node> {
    Ok(WgslFn::new(name, source, args, out)?.call(nodes))
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
/// `AdditiveBlending` of a non-premultiplied material: `blendFunc( SRC_ALPHA, ONE )`.
pub(super) fn additive() -> wgpu::BlendState {
    let component = wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::SrcAlpha,
        dst_factor: wgpu::BlendFactor::One,
        operation: wgpu::BlendOperation::Add,
    };
    wgpu::BlendState {
        color: component,
        alpha: component,
    }
}
const FULLSCREEN: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.xy*2.0,1.0,1.0);return out;}";
/// WebGL `gl_PointSize` squares from resident positions: u.custom[0] is (size in device
/// pixels, attenuation scale or 0, viewport width, viewport height). GL clamps sizes to
/// at least one pixel.
const POINT_QUADS: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let p=tsl_attribute_0[surface.instance_index].xyz;let mv=u.view*u.model*vec4(p,1.0);var size=u.custom[0].x;if u.custom[0].y>0.0 {size=size*(u.custom[0].y/-mv.z);}size=max(size,1.0);out.clip=u.projection*mv;out.clip=vec4(out.clip.xy+position.xy*size*2.0/u.custom[0].zw*out.clip.w,out.clip.zw);return out;}";
/// A viewport-filling triangle: WebGL's scissored clear color for one view.
async fn clear_triangle(s: &mut Scene, r: &Renderer) -> Result<Object3D> {
    let graph = NodeMaterial::new(vec4(uniform(0, Type::Vec3), float(1.)));
    let mut m = ShaderMaterial::new(Arc::new(
        ShaderProgram::with_projection(r, &graph.wgsl(0)?, &[], &[], FULLSCREEN).await?,
    ));
    m.properties.depth_test = false;
    m.properties.depth_write = false;
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        vec3s(vec![-0.5, -0.5, 0., 1.5, -0.5, 0., -0.5, 1.5, 0.])?,
    );
    g.set_attribute("normal", vec3s(vec![0., 0., 1., 0., 0., 1., 0., 0., 1.])?);
    g.set_attribute(
        "uv",
        Attribute::F32(BufferAttribute::new(vec![0.; 6], 2, false)?),
    );
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(g),
        Arc::new(Material::Shader(m)),
    )));
    s.get_mut(h)?.frustum_culled = false;
    s.get_mut(h)?.render_order = -10000;
    Ok(h)
}
/// Instanced point squares drawn from a resident position buffer.
async fn point_quads(
    s: &mut Scene,
    r: &Renderer,
    positions: &GpuBuffer,
    count: u32,
    color: Color,
) -> Result<Object3D> {
    let graph = NodeMaterial::new(vec4(uniform(1, Type::Vec3), float(1.)));
    let source = graph.wgsl_with_storage(0, &[Type::Vec4])?;
    let mut m = ShaderMaterial::new(Arc::new(
        ShaderProgram::with_projection(r, &source, &[positions], &[], POINT_QUADS).await?,
    ));
    m.uniforms[1] = color.0.as_vec3().extend(1.).to_array();
    let mut g = PlaneGeometry::build(1., 1., 1, 1)?;
    g.instance_count = Some(count);
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(g),
        Arc::new(Material::Shader(m)),
    )));
    s.get_mut(h)?.frustum_culled = false;
    Ok(h)
}
pub(super) fn set_uniform(s: &mut Scene, h: Object3D, index: usize, value: [f32; 4]) -> Result<()> {
    let material = match &mut s.get_mut(h)?.kind {
        NodeKind::Mesh(m) => &mut m.materials[0],
        NodeKind::Line(l) => &mut l.material,
        _ => return Err(Error::Invalid("uniform node")),
    };
    if let Material::Shader(m) = Arc::make_mut(material) {
        m.uniforms[index] = value;
    }
    Ok(())
}
/// `Matrix4.makePerspective` in WebGL's -1..1 depth convention.
pub(super) fn webgl_perspective(fov: f64, aspect: f64, near: f64, far: f64) -> Matrix4 {
    let top = near * (fov.to_radians() * 0.5).tan();
    let height = 2. * top;
    let width = aspect * height;
    let (l, r, t, b) = (-0.5 * width, 0.5 * width, top, top - height);
    Matrix4::from_cols_array(&[
        2. * near / (r - l),
        0.,
        0.,
        0.,
        0.,
        2. * near / (t - b),
        0.,
        0.,
        (r + l) / (r - l),
        (t + b) / (t - b),
        -(far + near) / (far - near),
        -1.,
        0.,
        0.,
        -2. * far * near / (far - near),
        0.,
    ])
}
/// `Matrix4.makeOrthographic` in WebGL's -1..1 depth convention.
fn webgl_orthographic(l: f64, r: f64, t: f64, b: f64, near: f64, far: f64) -> Matrix4 {
    let (w, h, p) = (r - l, t - b, far - near);
    Matrix4::from_cols_array(&[
        2. / w,
        0.,
        0.,
        0.,
        0.,
        2. / h,
        0.,
        0.,
        0.,
        0.,
        -2. / p,
        0.,
        -(r + l) / w,
        -(t + b) / h,
        -(far + near) / p,
        1.,
    ])
}
/// CameraHelper: 50 vertices at fixed NDC points, unprojected on the GPU by
/// u.custom[0..4] = camera.matrixWorld × projectionMatrixInverse.
pub(super) async fn camera_helper(
    r: &Renderer,
) -> Result<(Arc<BufferGeometry>, Arc<ShaderProgram>)> {
    let c = |x: f32, y: f32, near: bool| [x, y, if near { -1. } else { 1. }];
    let point = |name: &str| -> [f32; 3] {
        match name {
            "c" => c(0., 0., true),
            "t" => c(0., 0., false),
            "n1" => c(-1., -1., true),
            "n2" => c(1., -1., true),
            "n3" => c(-1., 1., true),
            "n4" => c(1., 1., true),
            "f1" => c(-1., -1., false),
            "f2" => c(1., -1., false),
            "f3" => c(-1., 1., false),
            "f4" => c(1., 1., false),
            "u1" => c(0.7, 1.1, true),
            "u2" => c(-0.7, 1.1, true),
            "u3" => c(0., 2., true),
            "cf1" => c(-1., 0., false),
            "cf2" => c(1., 0., false),
            "cf3" => c(0., -1., false),
            "cf4" => c(0., 1., false),
            "cn1" => c(-1., 0., true),
            "cn2" => c(1., 0., true),
            "cn3" => c(0., -1., true),
            "cn4" => c(0., 1., true),
            // "p" is never set by update(): it stays at the helper (camera) origin.
            _ => [0., 0., 2.],
        }
    };
    let (frustum, cone, up, target, cross) = (0xffaa00, 0xff0000, 0x00aaff, 0xffffff, 0x333333);
    let lines: [(&str, &str, u32, u32); 25] = [
        ("n1", "n2", frustum, frustum),
        ("n2", "n4", frustum, frustum),
        ("n4", "n3", frustum, frustum),
        ("n3", "n1", frustum, frustum),
        ("f1", "f2", frustum, frustum),
        ("f2", "f4", frustum, frustum),
        ("f4", "f3", frustum, frustum),
        ("f3", "f1", frustum, frustum),
        ("n1", "f1", frustum, frustum),
        ("n2", "f2", frustum, frustum),
        ("n3", "f3", frustum, frustum),
        ("n4", "f4", frustum, frustum),
        ("p", "n1", cone, cone),
        ("p", "n2", cone, cone),
        ("p", "n3", cone, cone),
        ("p", "n4", cone, cone),
        ("u1", "u2", up, up),
        ("u2", "u3", up, up),
        ("u3", "u1", up, up),
        ("c", "t", target, target),
        ("p", "c", cross, cross),
        ("cn1", "cn2", cross, cross),
        ("cn3", "cn4", cross, cross),
        ("cf1", "cf2", cross, cross),
        ("cf3", "cf4", cross, cross),
    ];
    let (mut positions, mut colors) = (vec![], vec![]);
    for (a, b, ca, cb) in lines {
        for (name, color) in [(a, ca), (b, cb)] {
            positions.extend(point(name));
            colors.extend(Color::from_hex(color).0.as_vec3().to_array());
        }
    }
    // u.custom[4] holds the camera position for the unset "p" (z = 2) vertices.
    let helper_projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let m=mat4x4(u.custom[0],u.custom[1],u.custom[2],u.custom[3]);var world=u.custom[4].xyz;if position.z<1.5 {let p=m*vec4(position,1.0);world=p.xyz/p.w;}out.clip=u.projection*u.view*vec4(world,1.0);out.local_normal=surface.local_normal;return out;}";
    let graph = NodeMaterial::new(vec4(normal_local(), float(1.)));
    let program = Arc::new(
        ShaderProgram::with_projection(r, &graph.wgsl(0)?, &[], &[], helper_projection).await?,
    );
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(positions)?);
    g.set_attribute("normal", vec3s(colors)?);
    let g = Arc::new(g);
    Ok((g, program))
}
/// CameraHelper.update(): the camera's projection inverse, in f64.
pub(super) fn update_camera_helper(
    s: &mut Scene,
    active: Object3D,
    helper: Object3D,
) -> Result<()> {
    let projection = match &s.get(active)?.kind {
        NodeKind::Camera(Camera::Perspective(p)) => {
            webgl_perspective(p.fov, p.aspect, p.near, p.far)
        }
        NodeKind::Camera(Camera::Orthographic(o)) => {
            webgl_orthographic(o.left, o.right, o.top, o.bottom, o.near, o.far)
        }
        _ => return Err(Error::Invalid("helper camera")),
    };
    let world = s.get(active)?.matrix_world;
    let m = world * projection.inverse();
    for (k, column) in m.to_cols_array().chunks(4).enumerate() {
        set_uniform(s, helper, k, std::array::from_fn(|i| column[i] as f32))?;
    }
    set_uniform(s, helper, 4, world.w_axis.as_vec4().to_array())?;
    Ok(())
}
/// OrbitControls (and MapControls), stepped once per animation frame like the original.
pub(super) struct Controls {
    target: Vector3,
    delta_theta: f64,
    delta_phi: f64,
    pan: Vector3,
    scale: f64,
    damping: Option<f64>,
    distance: (f64, f64),
    max_polar: f64,
    screen_space: bool,
    zoom_to_cursor: bool,
    cursor_zoom: bool,
    dolly_direction: Vector3,
    /// autoRotate speed: update( deltaTime = null ) turns by 2π / 60 / 60 × speed.
    pub(super) auto_rotate: Option<f64>,
    /// The camera's up: offsets are rotated into Y-up space and back.
    pub(super) up: Vector3,
    /// minPolarAngle.
    pub(super) min_polar: f64,
}
impl Controls {
    pub(super) fn new(
        damping: Option<f64>,
        distance: (f64, f64),
        max_polar: f64,
        screen_space: bool,
    ) -> Self {
        Self {
            target: Vector3::ZERO,
            delta_theta: 0.,
            delta_phi: 0.,
            pan: Vector3::ZERO,
            scale: 1.,
            damping,
            distance,
            max_polar,
            screen_space,
            zoom_to_cursor: false,
            cursor_zoom: false,
            dolly_direction: Vector3::ZERO,
            auto_rotate: None,
            up: Vector3::Y,
            min_polar: 0.,
        }
    }
    /// The animation loop's update(): autoRotate turns first while no pointer is active.
    pub(super) fn frame_update(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        if let Some(speed) = self.auto_rotate {
            self.delta_theta -= TAU / 60. / 60. * speed;
        }
        self.update(s, c)
    }
    pub(super) fn set_target(&mut self, target: Vector3) {
        self.target = target;
    }
    pub(super) fn rotate(&mut self, dx: f64, dy: f64, height: f64) {
        self.delta_theta -= TAU * dx / height;
        self.delta_phi -= TAU * dy / height;
    }
    /// `_pan` for a perspective camera, from the camera's current matrix.
    pub(super) fn pan(&mut self, camera: &CameraState, dx: f64, dy: f64, height: f64) {
        let offset = camera.position - self.target;
        let distance = offset.length() * (camera.fov / 2.).to_radians().tan();
        let x = camera.quaternion * Vector3::X;
        let up = if self.screen_space {
            camera.quaternion * Vector3::Y
        } else {
            Vector3::Y.cross(x)
        };
        self.pan += x * (-2. * dx * distance / height);
        self.pan += up * (2. * dy * distance / height);
    }
    /// Wheel: `_updateZoomParameters`, then `_dollyIn` / `_dollyOut`.
    pub(super) fn dolly(&mut self, wheel: f64, camera: &CameraState, pointer: Vector2) {
        if self.zoom_to_cursor {
            self.cursor_zoom = true;
            // ( mouse.x, mouse.y, 1 ).unproject( camera ): a point on the far plane.
            let far = camera.world.transform_point3(
                camera
                    .projection
                    .inverse()
                    .project_point3(pointer.extend(1.)),
            );
            self.dolly_direction = (far - camera.position).normalize();
        }
        let scale = 0.95f64.powf((wheel * 0.01).abs());
        if wheel < 0. {
            self.scale *= scale;
        } else if wheel > 0. {
            self.scale /= scale;
        }
    }
    fn clamp(&self, d: f64) -> f64 {
        d.min(self.distance.1).max(self.distance.0)
    }
    /// `OrbitControls.update()` with `minPolarAngle` 0 and unbounded azimuth.
    pub(super) fn update(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        const EPS: f64 = 0.000001;
        let quat = Quaternion::from_rotation_arc(self.up, Vector3::Y);
        let offset = quat * (s.get(c)?.position - self.target);
        let mut radius = offset.length();
        let (mut theta, mut phi) = if radius == 0. {
            (0., 0.)
        } else {
            (
                offset.x.atan2(offset.z),
                (offset.y / radius).clamp(-1., 1.).acos(),
            )
        };
        let f = self.damping.unwrap_or(1.);
        theta += self.delta_theta * f;
        phi += self.delta_phi * f;
        phi = phi.min(self.max_polar).max(self.min_polar);
        phi = phi.clamp(EPS, PI - EPS);
        self.target += self.pan * f;
        radius = if self.zoom_to_cursor && self.cursor_zoom {
            self.clamp(radius)
        } else {
            self.clamp(radius * self.scale)
        };
        let sin_phi_radius = phi.sin() * radius;
        let v = quat.inverse()
            * Vector3::new(
                sin_phi_radius * theta.sin(),
                phi.cos() * radius,
                sin_phi_radius * theta.cos(),
            );
        s.get_mut(c)?.position = self.target + v;
        s.look_at(c, self.target)?;
        if let Some(f) = self.damping {
            self.delta_theta *= 1. - f;
            self.delta_phi *= 1. - f;
            self.pan *= 1. - f;
        } else {
            self.delta_theta = 0.;
            self.delta_phi = 0.;
            self.pan = Vector3::ZERO;
        }
        if self.zoom_to_cursor && self.cursor_zoom {
            // Move the camera down the pointer ray, then re-derive the target.
            let previous = v.length();
            let radius = self.clamp(previous * self.scale);
            let n = s.get_mut(c)?;
            n.position += self.dolly_direction * (previous - radius);
            let (position, direction) = (n.position, n.quaternion * -Vector3::Z);
            s.update_world_matrix(c, true, false)?;
            if self.screen_space {
                self.target = position + direction * radius;
            } else if Vector3::Y.dot(direction).abs() < (70f64).to_radians().cos() {
                s.look_at(c, self.target)?;
            } else if direction.y != 0. {
                // Ray.intersectPlane with the up-normal plane through the target.
                let t = (self.target.y - position.y) / direction.y;
                if t >= 0. {
                    self.target = position + direction * t;
                }
            }
        }
        self.scale = 1.;
        self.cursor_zoom = false;
        s.update_world_matrix(c, true, false)
    }
}
/// The camera state the controls read: position, orientation, projection and field of view.
pub(super) struct CameraState {
    position: Vector3,
    quaternion: Quaternion,
    world: Matrix4,
    projection: Matrix4,
    fov: f64,
}
pub(super) fn camera_state(s: &Scene, c: Object3D) -> Result<CameraState> {
    let n = s.get(c)?;
    let (camera, _) = s.camera(c)?;
    let Camera::Perspective(p) = camera else {
        return Err(Error::Invalid("perspective controls camera"));
    };
    Ok(CameraState {
        position: n.position,
        quaternion: n.quaternion,
        world: n.matrix_world,
        projection: camera.projection_matrix()?,
        fov: p.fov,
    })
}
/// `webgl_camera`: the rig cameras, their helpers and the views.
struct Rig {
    rig: Object3D,
    perspective: Object3D,
    orthographic: Object3D,
    helpers: [Object3D; 2],
    mesh: Object3D,
    child: Object3D,
    points: Object3D,
    clear: Object3D,
    ortho: bool,
    output: Option<RenderTarget>,
    _positions: GpuBuffer,
}
/// `webgl_buffergeometry_drawrange` particle state and its resident buffers.
struct Particles {
    positions: Vec<f32>,
    velocities: Vec<Vector3>,
    connections: Vec<u32>,
    segments: Vec<[f32; 4]>,
    position_buffer: GpuBuffer,
    segment_buffer: GpuBuffer,
    dots: Object3D,
    lines: Object3D,
    group: Object3D,
    dirty: bool,
}
/// `webgl_custom_attributes` per-vertex noise, its HSL-drifting color and the displacement buffer.
struct Displaced {
    sphere: Object3D,
    noise: Vec<f32>,
    displacement: Vec<f32>,
    buffer: GpuBuffer,
    color: [f64; 3],
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    pointer: Vector2,
    controls: Option<Controls>,
    params: [f32; 6],
    camera: Option<Rig>,
    particles: Option<Particles>,
    displaced: Option<Displaced>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            pointer: Vector2::ZERO,
            controls: None,
            params: [1., 1., 150., 0., 20., 500.],
            camera: None,
            particles: None,
            displaced: None,
        };
        let (fov, near, far, position) = match id {
            208 => (60., 1., 1000., Vector3::new(400., 200., 0.)),
            209 => (60., 1., 1000., Vector3::new(0., 200., -200.)),
            210 => (50., 1., 10000., Vector3::new(0., 0., 2500.)),
            211 => (30., 1., 10000., Vector3::new(0., 0., 300.)),
            _ => (45., 1., 4000., Vector3::new(0., 0., 1750.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect: if id == 210 { 0.5 * aspect } else { aspect },
            ..Default::default()
        }));
        s.get_mut(c)?.position = position;
        s.look_at(c, Vector3::ZERO)?;
        match id {
            208 | 209 => d.city(s)?,
            210 => d.rig(s, r).await?,
            211 => d.displaced(s, r).await?,
            _ => d.particles(s, r).await?,
        }
        Ok(d)
    }
    /// The instanced city of the controls examples, with fog and three lights.
    fn city(&mut self, s: &mut Scene) -> Result<()> {
        s.background = Color::from_hex(0xcccccc);
        s.fog = Some(Fog::Exp2 {
            color: Color::from_hex(0xcccccc),
            density: 0.002,
        });
        let map = self.id == 209;
        let geometry = if map {
            let mut g = BoxGeometry::build(1., 1., 1.)?;
            g.translate(Vector3::new(0., 0.5, 0.))?;
            g
        } else {
            // ConeGeometry( 10, 30, 4, 1 ).
            CylinderGeometry::build(0., 10., 30., 4, 1, false, 0., TAU)?
        };
        let mut m = MeshPhongMaterial::default();
        m.properties.color = Color::from_hex(if map { 0xeeeeee } else { 0xffffff });
        m.properties.flat_shading = true;
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Phong(m)),
        )));
        let mut instances = Vec::with_capacity(500);
        for _ in 0..500 {
            let x = random(&mut self.seed) * 1600. - 800.;
            let z = random(&mut self.seed) * 1600. - 800.;
            let scale = if map {
                Vector3::new(20., random(&mut self.seed) * 80. + 10., 20.)
            } else {
                Vector3::ONE
            };
            instances.push(Instance {
                matrix: Matrix4::from_scale_rotation_translation(
                    scale,
                    Quaternion::IDENTITY,
                    Vector3::new(x, 0., z),
                ),
                color: Color::WHITE,
            });
        }
        s.get_mut(h)?.instances = instances;
        for (color, p) in [(0xffffff, 1.), (0x002288, -1.)] {
            let light = s.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(color),
                intensity: 3.,
                target: Vector3::ZERO,
            }));
            s.get_mut(light)?.position = Vector3::splat(p);
        }
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x555555),
            intensity: 1.,
        }));
        self.controls = Some(Controls::new(Some(0.05), (100., 500.), PI / 2., false));
        Ok(())
    }
    async fn rig(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::BLACK;
        let rig = s.insert(NodeKind::Group);
        let perspective = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 150.,
            far: 1000.,
            ..Default::default()
        })));
        let orthographic = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            near: 150.,
            far: 1000.,
            ..Default::default()
        })));
        for camera in [perspective, orthographic] {
            // Counteract the cameras' -Z front against the rig's +Z lookAt.
            s.get_mut(camera)?.quaternion = Quaternion::from_rotation_y(PI);
            s.add(rig, camera)?;
        }
        let (g, program) = camera_helper(r).await?;
        let mut helpers = Vec::with_capacity(2);
        for _ in 0..2 {
            let m = ShaderMaterial::new(program.clone());
            let helper = s.insert(NodeKind::Line(Line {
                geometry: g.clone(),
                material: Arc::new(Material::Shader(m)),
                segments: true,
            }));
            s.get_mut(helper)?.frustum_culled = false;
            helpers.push(helper);
        }
        let helpers = [helpers[0], helpers[1]];
        let wire = |radius: f64, color: u32| -> Result<NodeKind> {
            let mut m = MeshBasicMaterial::default();
            m.properties.color = Color::from_hex(color);
            m.properties.wireframe = true;
            Ok(NodeKind::Mesh(Mesh::new(
                Arc::new(SphereGeometry::build(radius, 16, 8)?),
                Arc::new(Material::Basic(m)),
            )))
        };
        let mesh = s.insert(wire(100., 0xffffff)?);
        let child = s.insert(wire(50., 0x00ff00)?);
        s.get_mut(child)?.position.y = 150.;
        s.add(mesh, child)?;
        let small = s.insert(wire(5., 0x0000ff)?);
        s.get_mut(small)?.position.z = 150.;
        s.add(rig, small)?;
        // MathUtils.randFloatSpread( 2000 ) for x, y, z.
        let mut positions = Vec::with_capacity(40000);
        for _ in 0..10000 {
            for _ in 0..3 {
                positions.push((2000. * (0.5 - random(&mut self.seed))) as f32);
            }
            positions.push(1.);
        }
        let buffer = GpuBuffer::new(r, bytemuck::cast_slice(&positions), BufferAccess::Read)?;
        let points = point_quads(s, r, &buffer, 10000, Color::from_hex(0x888888)).await?;
        let clear = clear_triangle(s, r).await?;
        set_uniform(
            s,
            clear,
            0,
            Color::from_hex(0x111111).0.as_vec3().extend(1.).to_array(),
        )?;
        self.camera = Some(Rig {
            rig,
            perspective,
            orthographic,
            helpers,
            mesh,
            child,
            points,
            clear,
            ortho: false,
            output: None,
            _positions: buffer,
        });
        Ok(())
    }
    async fn displaced(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        // The raw output clears with the background's display value, as WebGL's sRGB clear.
        s.background = Color::linear(5. / 255., 5. / 255., 5. / 255.);
        let geometry = SphereGeometry::build(50., 128, 64)?;
        let count = geometry
            .attributes
            .get("position")
            .map(|a| a.count())
            .ok_or(Error::Invalid("sphere positions"))?;
        let noise: Vec<f32> = (0..count)
            .map(|_| (random(&mut self.seed) * 5.) as f32)
            .collect();
        let displacement = vec![0f32; count];
        let buffer = GpuBuffer::new(r, bytemuck::cast_slice(&displacement), BufferAccess::Read)?;
        let mut water =
            decode_texture_image(&fetch("/web/gallery/assets/water.jpg").await?).await?;
        water.srgb = false;
        water.wrap_s = Wrapping::Repeat;
        water.wrap_t = Wrapping::Repeat;
        water.mipmap_filter = Some(Filter::Linear);
        let water = r.upload_texture(&Arc::new(water))?;
        // Vertex: position + amplitude × normal × displacement, vUv from amplitude.
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let a=u.custom[0].x;let p=position+a*surface.local_normal*tsl_attribute_0[tsl_vertex_index];out.clip=u.projection*u.view*u.model*vec4(p,1.0);out.uv=(0.5+a)*surface.uv+vec2(a);return out;}";
        // Fragment: the half-Lambert grey of the texture times color, written raw.
        // flipY uploads put v = 0 at the image bottom.
        let color = call(
            "custom_color",
            "fn custom_color(n:vec3<f32>,t:vec4<f32>,c:vec3<f32>)->vec4<f32>{let d=dot(n,normalize(vec3(0.5,0.2,1.0)))*0.5+0.5;let g=vec4(vec3(t.r*0.3+t.g*0.59+t.b*0.11),1.0);return g*vec4(vec3(d)*c,1.0);}",
            &[Type::Vec3, Type::Vec4, Type::Vec3],
            Type::Vec4,
            &[
                normal_local(),
                tsl::Texture::External(0).sample(vec2(uv().x(), float(1.) - uv().y())),
                uniform(1, Type::Vec3),
            ],
        )?;
        let graph = NodeMaterial::new(color);
        let source = graph.wgsl_with_storage(1, &[Type::Float])?;
        let m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(
                r,
                &source,
                &[&buffer],
                &[(&water.view, &water.sampler)],
                projection,
            )
            .await?,
        ));
        let sphere = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Shader(m)),
        )));
        s.get_mut(sphere)?.frustum_culled = false;
        let c = Color::from_hex(0xff2200).0;
        self.displaced = Some(Displaced {
            sphere,
            noise,
            displacement,
            buffer,
            color: [c.x, c.y, c.z],
        });
        Ok(())
    }
    async fn particles(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::BLACK;
        let group = s.insert(NodeKind::Group);
        // BoxHelper of the 800-unit box: 8 corners, 12 indexed edges, additive grey.
        let h = 400f32;
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            vec3s(vec![
                h, h, h, -h, h, h, -h, -h, h, h, -h, h, h, h, -h, -h, h, -h, -h, -h, -h, h, -h, -h,
            ])?,
        );
        g.set_index(Some(vec![
            0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
        ]));
        let mut m = LineBasicMaterial::default();
        m.properties.color = Color::from_hex(0x474747);
        m.properties.transparent = true;
        m.properties.blending = Some(additive());
        let helper = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Line(m)),
            segments: true,
        }));
        s.add(group, helper)?;
        let mut positions = vec![0f32; 3000];
        let mut velocities = Vec::with_capacity(1000);
        for i in 0..1000 {
            for k in 0..3 {
                positions[i * 3 + k] = (random(&mut self.seed) * 800. - 400.) as f32;
            }
            velocities.push(Vector3::from_array(
                [0; 3].map(|_| -1. + random(&mut self.seed) * 2.),
            ));
        }
        let position_buffer = GpuBuffer::zeroed(r, 1000 * 16, BufferAccess::Read)?;
        let capacity = 1000 * 999 / 2;
        let segment_buffer = GpuBuffer::zeroed(r, capacity as u64 * 32, BufferAccess::Read)?;
        let dots = point_quads(s, r, &position_buffer, 1000, Color::WHITE).await?;
        if let NodeKind::Mesh(mesh) = &mut s.get_mut(dots)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
        {
            m.properties.transparent = true;
            m.properties.blending = Some(additive());
        }
        // LineSegments with vertex colors: one resident segment per instance.
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let s=tsl_attribute_0[surface.instance_index*2u+u32(position.x)];out.clip=u.projection*u.view*u.model*vec4(s.xyz,1.0);out.local_normal=vec3(s.w);return out;}";
        let graph = NodeMaterial::new(vec4(normal_local(), float(1.)));
        let source = graph.wgsl_with_storage(0, &[Type::Vec4])?;
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(r, &source, &[&segment_buffer], &[], projection).await?,
        ));
        m.properties.transparent = true;
        m.properties.blending = Some(additive());
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(vec![0., 0., 0., 1., 0., 0.])?);
        g.set_attribute("normal", vec3s(vec![0., 0., 1., 0., 0., 1.])?);
        g.instance_count = Some(capacity);
        let lines = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Shader(m)),
            segments: true,
        }));
        s.get_mut(lines)?.frustum_culled = false;
        s.get_mut(lines)?.instance_count = Some(0);
        // Equal-depth transparent objects keep the original's creation order.
        for (order, h) in [helper, dots, lines].into_iter().enumerate() {
            s.get_mut(h)?.render_order = order as i32;
            if h != helper {
                s.add(group, h)?;
            }
        }
        s.get_mut(dots)?.instance_count = Some(500);
        self.particles = Some(Particles {
            positions,
            velocities,
            connections: vec![0; 1000],
            segments: Vec::with_capacity(capacity as usize * 2),
            position_buffer,
            segment_buffer,
            dots,
            lines,
            group,
            dirty: true,
        });
        self.controls = Some(Controls::new(None, (1000., 3000.), PI, true));
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// One original animation frame; per-frame increments run as 60 fps steps of example time.
    pub fn prepare(&mut self, r: &Renderer, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        match self.id {
            208 | 209 => {
                if let Some(controls) = &mut self.controls {
                    controls.update(s, c)?;
                }
            }
            210 => self.prepare_rig(s, c, t)?,
            211 => {
                let d = self.displaced.as_mut().ok_or(Error::Invalid("displaced"))?;
                let time = t * 10.;
                let angle = 0.01 * time;
                s.get_mut(d.sphere)?.quaternion = Euler {
                    angles: Vector3::new(0., angle, angle),
                    order: EulerOrder::XYZ,
                }
                .quaternion();
                for _ in 0..steps {
                    d.color = offset_hue(d.color, 0.0005);
                    for n in d.noise.iter_mut() {
                        let v = (*n as f64 + 0.5 * (0.5 - random(&mut self.seed))) as f32;
                        *n = (v as f64).clamp(-5., 5.) as f32;
                    }
                }
                for (i, (v, n)) in d.displacement.iter_mut().zip(&d.noise).enumerate() {
                    let wave = (0.1 * i as f64 + time).sin() as f32;
                    *v = (wave as f64 + *n as f64) as f32;
                }
                d.buffer
                    .write(r, 0, bytemuck::cast_slice(&d.displacement))?;
                let amplitude = 2.5 * (angle * 0.125).sin();
                let color = d.color;
                let sphere = d.sphere;
                set_uniform(s, sphere, 0, [amplitude as f32, 0., 0., 0.])?;
                set_uniform(
                    s,
                    sphere,
                    1,
                    [color[0] as f32, color[1] as f32, color[2] as f32, 1.],
                )?;
            }
            _ => self.prepare_particles(r, s, t, steps)?,
        }
        Ok(())
    }
    fn prepare_rig(&mut self, s: &mut Scene, c: Object3D, t: f64) -> Result<()> {
        let aspect = css_aspect();
        let rig = self.camera.as_ref().ok_or(Error::Invalid("rig"))?;
        let r = t * 0.5;
        let mesh = s.get_mut(rig.mesh)?;
        mesh.position = Vector3::new(700. * r.cos(), 700. * r.sin(), 700. * r.sin());
        let far = mesh.position.length();
        let target = mesh.position;
        let child = s.get_mut(rig.child)?;
        child.position.x = 70. * (2. * r).cos();
        child.position.z = 70. * r.sin();
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.aspect = 0.5 * aspect;
        }
        let fov = 35. + 30. * (0.5 * r).sin();
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(rig.perspective)?.kind {
            p.aspect = 0.5 * aspect;
            if !rig.ortho {
                p.fov = fov;
                p.far = far;
            }
        }
        let frustum = 600.;
        if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(rig.orthographic)?.kind {
            o.left = 0.5 * frustum * aspect / -2.;
            o.right = 0.5 * frustum * aspect / 2.;
            o.top = frustum / 2.;
            o.bottom = frustum / -2.;
            if rig.ortho {
                o.far = far;
            }
        }
        s.look_at(rig.rig, target)?;
        let (active, helper) = if rig.ortho {
            (rig.orthographic, rig.helpers[1])
        } else {
            (rig.perspective, rig.helpers[0])
        };
        s.update_world_matrix(active, true, false)?;
        update_camera_helper(s, active, helper)?;
        for (h, visible) in [(rig.helpers[0], !rig.ortho), (rig.helpers[1], rig.ortho)] {
            s.get_mut(h)?.visible = visible;
        }
        Ok(())
    }
    fn prepare_particles(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        t: f64,
        steps: usize,
    ) -> Result<()> {
        let [
            show_dots,
            show_lines,
            min_distance,
            limit,
            max_connections,
            count,
        ] = self.params;
        let p = self.particles.as_mut().ok_or(Error::Invalid("particles"))?;
        let count = count as usize;
        let (limit, max_connections) = (limit > 0.5, max_connections as u32);
        let min_distance = min_distance as f64;
        for _ in 0..steps {
            p.segments.clear();
            p.connections[..count].fill(0);
            for i in 0..count {
                let v = &mut p.velocities[i];
                for k in 0..3 {
                    p.positions[i * 3 + k] = (p.positions[i * 3 + k] as f64 + v[k]) as f32;
                }
                let out = |x: f32| (x as f64) < -400. || (x as f64) > 400.;
                if out(p.positions[i * 3 + 1]) {
                    v.y = -v.y;
                }
                if out(p.positions[i * 3]) {
                    v.x = -v.x;
                }
                if out(p.positions[i * 3 + 2]) {
                    v.z = -v.z;
                }
                if limit && p.connections[i] >= max_connections {
                    continue;
                }
                for j in i + 1..count {
                    if limit && p.connections[j] >= max_connections {
                        continue;
                    }
                    let d =
                        |k: usize| p.positions[i * 3 + k] as f64 - p.positions[j * 3 + k] as f64;
                    let dist = (d(0) * d(0) + d(1) * d(1) + d(2) * d(2)).sqrt();
                    if dist < min_distance {
                        p.connections[i] += 1;
                        p.connections[j] += 1;
                        let alpha = (1. - dist / min_distance) as f32;
                        let a = &p.positions[i * 3..i * 3 + 3];
                        let b = &p.positions[j * 3..j * 3 + 3];
                        p.segments.push([a[0], a[1], a[2], alpha]);
                        p.segments.push([b[0], b[1], b[2], alpha]);
                    }
                }
            }
            p.dirty = true;
        }
        if p.dirty {
            let records: Vec<[f32; 4]> = p
                .positions
                .chunks(3)
                .map(|v| [v[0], v[1], v[2], 1.])
                .collect();
            p.position_buffer
                .write(r, 0, bytemuck::cast_slice(&records))?;
            if !p.segments.is_empty() {
                p.segment_buffer
                    .write(r, 0, bytemuck::cast_slice(&p.segments))?;
            }
            p.dirty = false;
        }
        let (dots, lines, group, segments) = (p.dots, p.lines, p.group, p.segments.len() / 2);
        s.get_mut(lines)?.instance_count = Some(segments as u32);
        s.get_mut(lines)?.visible = show_lines > 0.5;
        s.get_mut(dots)?.instance_count = Some(count as u32);
        s.get_mut(dots)?.visible = show_dots > 0.5;
        s.get_mut(group)?.quaternion = Quaternion::from_rotation_y(t * 0.1);
        let (w, h, dpr) = viewport();
        // PointsMaterial size 3 without attenuation.
        set_uniform(s, dots, 0, [(3. * dpr) as f32, 0., w as f32, h as f32])?;
        Ok(())
    }
    /// `webgl_camera`: the active rig camera on the left, the overview camera on the right.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let Some(rig) = self.camera.as_mut() else {
            return Ok(false);
        };
        if rig.output.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.options.samples != out.options.samples
        }) {
            let mut options = out.options.clone();
            options.store_multisampled_color_buffer = true;
            rig.output = Some(RenderTarget::with_options(
                &r.device, out.width, out.height, options,
            )?);
        }
        let (css_w, css_h, dpr) = viewport_css();
        // setViewport( 0 / W/2, 0, W/2, H ): WebGL rounds × pixelRatio.
        let half = ((css_w / 2.) * dpr).round() as u32;
        let width = half.min(out.width);
        let views = [
            (
                0,
                width,
                if rig.ortho {
                    rig.orthographic
                } else {
                    rig.perspective
                },
            ),
            (width, width.min(out.width - width), c),
        ];
        let active_helper = rig.helpers[usize::from(rig.ortho)];
        let (points, clear) = (rig.points, rig.clear);
        for (i, (x, w, camera)) in views.into_iter().enumerate() {
            if w == 0 {
                continue;
            }
            s.get_mut(active_helper)?.visible = i == 1;
            s.get_mut(clear)?.visible = i == 1;
            set_uniform(
                s,
                points,
                0,
                [
                    dpr as f32,
                    (css_h * 0.5) as f32,
                    w as f32,
                    out.height as f32,
                ],
            )?;
            let target = rig.output.as_mut().expect("camera target");
            target.viewport = [x, 0, w, out.height];
            target.scissor = Some([x, 0, w, out.height]);
            target.set_load_color(i != 0);
            r.render(s, camera, target)?;
        }
        let target = rig.output.as_mut().expect("camera target");
        target.scissor = None;
        target.viewport = [0, 0, out.width, out.height];
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.camera.as_ref().and_then(|r| r.output.as_ref())
    }
    pub fn gpu_pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new(x, y);
    }
    /// Pointer and wheel handlers: OrbitControls calls update() on every event.
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
        let pointer = self.pointer;
        let map = self.id == 209;
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, pointer);
        } else if pan != map {
            // MapControls swaps the buttons: left pans, right (or modified left) rotates.
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let (Some(rig), true) = (&mut self.camera, down) {
            match code {
                79 => rig.ortho = true,
                80 => rig.ortho = false,
                _ => {}
            }
        }
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (209, 0) => {
                if let Some(c) = &mut self.controls {
                    c.zoom_to_cursor = value > 0.5;
                }
            }
            (209, 1) => {
                if let Some(c) = &mut self.controls {
                    c.screen_space = value > 0.5;
                }
            }
            (212, 0 | 1 | 3) => self.params[index] = value,
            (212, 2) if (10. ..=300.).contains(&value) => self.params[2] = value,
            (212, 4) if (0. ..=30.).contains(&value) => self.params[4] = value.round(),
            (212, 5) if (0. ..=1000.).contains(&value) => self.params[5] = value.round(),
            _ => return Err(Error::Invalid("controls/attributes parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// `Color.offsetHSL( h, 0, 0 )` in the linear working color space.
fn offset_hue(c: [f64; 3], dh: f64) -> [f64; 3] {
    let [r, g, b] = c;
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let lightness = (min + max) / 2.;
    let (mut hue, saturation);
    if min == max {
        hue = 0.;
        saturation = 0.;
    } else {
        let delta = max - min;
        saturation = if lightness <= 0.5 {
            delta / (max + min)
        } else {
            delta / (2. - max - min)
        };
        hue = if max == r {
            (g - b) / delta + if g < b { 6. } else { 0. }
        } else if max == g {
            (b - r) / delta + 2.
        } else {
            (r - g) / delta + 4.
        };
        hue /= 6.;
    }
    // setHSL: euclideanModulo( h, 1 ), clamped s and l.
    let h = ((hue + dh) % 1. + 1.) % 1.;
    let (s, l) = (saturation.clamp(0., 1.), lightness.clamp(0., 1.));
    if s == 0. {
        return [l; 3];
    }
    let p = if l <= 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let q = 2. * l - p;
    let hue2rgb = |p: f64, q: f64, mut t: f64| {
        if t < 0. {
            t += 1.;
        }
        if t > 1. {
            t -= 1.;
        }
        if t < 1. / 6. {
            p + (q - p) * 6. * t
        } else if t < 1. / 2. {
            q
        } else if t < 2. / 3. {
            p + (q - p) * 6. * (2. / 3. - t)
        } else {
            p
        }
    };
    [
        hue2rgb(q, p, h + 1. / 3.),
        hue2rgb(q, p, h),
        hue2rgb(q, p, h - 1. / 3.),
    ]
}
pub(super) fn viewport_css() -> (f64, f64, f64) {
    let window = web_sys::window();
    let size = |f: fn(
        &web_sys::Window,
    ) -> std::result::Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>| {
        window
            .as_ref()
            .and_then(|w| f(w).ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(512.)
    };
    let dpr = window
        .as_ref()
        .map(|w| w.device_pixel_ratio())
        .unwrap_or(1.);
    (
        size(web_sys::Window::inner_width),
        size(web_sys::Window::inner_height),
        dpr,
    )
}
fn css_aspect() -> f64 {
    let (w, h, _) = viewport_css();
    w / h.max(1.)
}
/// Drawing-buffer size in device pixels, and the pixel ratio.
fn viewport() -> (f64, f64, f64) {
    let (w, h, dpr) = viewport_css();
    ((w * dpr).round(), (h * dpr).round(), dpr)
}
