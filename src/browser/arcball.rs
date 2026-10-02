//! misc_controls_arcball: ArcballControls ( r186 ) around the Cerberus OBJ
//! with its albedo, roughness / metalness and normal maps under the
//! venice_sunset environment, Reinhard tone mapping at exposure 3 over the
//! page's CSS gradient through the transparent canvas. The controls port the
//! addon's mouse operations ( rotate on the trackball surface, pan on the
//! trackball plane with the optional grid, wheel and middle-drag zoom, shift
//! wheel / drag field of view, ctrl-drag pan ), the double-click focus, the
//! rotation inertia and focus animations on the page clock, the three gizmo
//! circles, adjustNearFar, the distance and zoom limits, reset, and the
//! clipboard state copy and paste ( ctrl+c, ctrl+v and the buttons ), on a
//! perspective or an orthographic camera.
use super::controls_attributes::{viewport_css, webgl_orthographic, webgl_perspective};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::terrain_loaders::parse_obj;
use crate::attribute::BufferAttribute;
use crate::environment::EnvironmentMap;
use crate::raycast::Raycaster;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

const PERSPECTIVE_DISTANCE: f64 = 2.5;
const ORTHOGRAPHIC_DISTANCE: f64 = 120.;
const CURVE_POINTS: usize = 128;

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Idle,
    Rotate,
    Pan,
    Scale,
    Fov,
    Focus,
    AnimationFocus,
    AnimationRotate,
}
#[derive(Clone, Copy, PartialEq)]
enum Op {
    Pan,
    Rotate,
    Zoom,
    Fov,
}
#[derive(Clone, Copy, PartialEq)]
enum Key {
    Ctrl,
    Shift,
}
#[derive(Clone, Copy)]
enum Mouse {
    Button(i32),
    Wheel,
}
struct Action {
    op: Op,
    mouse: Mouse,
    key: Option<Key>,
}
impl Action {
    fn matches(&self, mouse: Mouse, key: Option<Key>) -> bool {
        let same = match (self.mouse, mouse) {
            (Mouse::Button(a), Mouse::Button(b)) => a == b,
            (Mouse::Wheel, Mouse::Wheel) => true,
            _ => false,
        };
        same && self.key == key
    }
}
fn state_of(op: Op) -> State {
    match op {
        Op::Pan => State::Pan,
        Op::Rotate => State::Rotate,
        Op::Zoom => State::Scale,
        Op::Fov => State::Fov,
    }
}
/// The camera the controls drive: three's object fields, beside the node.
#[derive(Clone, Copy)]
enum Lens {
    Perspective {
        fov: f64,
        aspect: f64,
    },
    Orthographic {
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
    },
}
enum Anim {
    None,
    Rotate { axis: Vector3, w0: f64 },
    Focus { point: Vector3, gizmo: Matrix4 },
}
struct Click {
    x: f64,
    y: f64,
    time: f64,
}

fn now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map_or(0., |p| p.now())
}
/// Vector3.normalize(): divided by its length, or left as is when zero.
fn normalize(v: Vector3) -> Vector3 {
    let l = v.length();
    if l == 0. { v } else { v / l }
}
fn decompose(m: Matrix4) -> (Vector3, Quaternion, Vector3) {
    let (s, q, t) = m.to_scale_rotation_translation();
    (t, q, s)
}
fn compose(position: Vector3, quaternion: Quaternion, scale: Vector3) -> Matrix4 {
    Matrix4::from_scale_rotation_translation(scale, quaternion, position)
}

pub(super) struct Demo {
    camera: Object3D,
    lens: Lens,
    near: f64,
    far: f64,
    zoom: f64,
    meshes: Vec<Object3D>,
    gizmos: Object3D,
    gizmo_lines: [Object3D; 3],
    /// Per circle: the idle ( opacity 0.6 ) and active ( 1 ) materials.
    gizmo_materials: [[Arc<Material>; 2]; 3],
    grid: Option<Object3D>,
    target: Vector3,
    radius_factor: f64,
    actions: Vec<Action>,
    mouse_op: Option<Op>,
    camera_matrix_state: Matrix4,
    fov_state: f64,
    up_state: Vector3,
    zoom_state: f64,
    gizmo_matrix_state: Matrix4,
    up0: Vector3,
    zoom0: f64,
    fov0: f64,
    initial_near: f64,
    near_pos0: f64,
    initial_far: f64,
    far_pos0: f64,
    near_pos: f64,
    far_pos: f64,
    camera_matrix_state0: Matrix4,
    gizmo_matrix_state0: Matrix4,
    target0: Vector3,
    button: i32,
    cursor: bool,
    dev_px_ratio: f64,
    down_valid: bool,
    nclicks: u32,
    down_events: Vec<Click>,
    click_start: f64,
    current_cursor: Vector3,
    start_cursor: Vector3,
    time_start: f64,
    anim: Anim,
    time_prev: f64,
    time_current: f64,
    angle_prev: f64,
    angle_current: f64,
    cursor_prev: Vector3,
    cursor_curr: Vector3,
    w_prev: f64,
    w_curr: f64,
    adjust_near_far: bool,
    scale_factor: f64,
    damping_factor: f64,
    w_max: f64,
    enable_animations: bool,
    enable_grid: bool,
    cursor_zoom: bool,
    min_fov: f64,
    max_fov: f64,
    enable_pan: bool,
    enable_rotate: bool,
    enable_zoom: bool,
    enable_focus: bool,
    enabled: bool,
    min_distance: f64,
    max_distance: f64,
    min_zoom: f64,
    max_zoom: f64,
    tb_radius: f64,
    state: State,
    shift: bool,
    ctrl: bool,
    queue: Vec<(u32, f64, f64)>,
    wheel: Vec<(f64, f64)>,
    keys: Vec<(u32, bool)>,
    pending: Vec<usize>,
    viewport: (f64, f64),
    last_pointer: (f64, f64),
    paste_slot: std::rc::Rc<std::cell::RefCell<Option<String>>>,
    pending_camera: Option<bool>,
    pending_gizmos: Option<bool>,
    /// Transformation matrices of the last operation ( camera, gizmos ).
    transformation: (Option<Matrix4>, Option<Matrix4>),
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        s.tone_mapping = ToneMapping::Reinhard;
        s.exposure = 3.;
        // WebGLRenderer( { alpha: true } ) with no background: the canvas
        // stays transparent over the page's gradient.
        s.background = Color::BLACK;
        s.background_alpha = 0.;
        let objects = parse_obj(&String::from_utf8_lossy(
            &fetch("/web/gallery/assets/obj/cerberus/Cerberus.obj").await?,
        ));
        let texture = |bytes: Vec<u8>, srgb: bool| async move {
            let mut t = decode_texture_image(&bytes).await?;
            t.srgb = srgb;
            t.wrap_s = Wrapping::Repeat;
            t.mipmap_filter = Some(Filter::Linear);
            Ok::<_, Error>(Arc::new(t))
        };
        let map = texture(
            fetch("/web/gallery/assets/obj/cerberus/Cerberus_A.jpg").await?,
            true,
        )
        .await?;
        let rm = texture(
            fetch("/web/gallery/assets/obj/cerberus/Cerberus_RM.jpg").await?,
            false,
        )
        .await?;
        let normal = texture(
            fetch("/web/gallery/assets/obj/cerberus/Cerberus_N.jpg").await?,
            false,
        )
        .await?;
        let material = Arc::new(Material::Standard(MeshStandardMaterial {
            energy_conservation: true,
            roughness: 1.,
            metalness: 1.,
            properties: MaterialProperties {
                map: Some(map),
                ..Default::default()
            },
            metallic_roughness_map: Some(rm),
            normal_map: Some(normal),
            ..Default::default()
        }));
        let group = s.insert(NodeKind::Group);
        let mut meshes = vec![];
        for o in &objects {
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(o.positions.clone(), 3, false)?),
            );
            if !o.normals.is_empty() {
                g.set_attribute(
                    "normal",
                    Attribute::F32(BufferAttribute::new(o.normals.clone(), 3, false)?),
                );
            }
            if !o.uvs.is_empty() {
                g.set_attribute(
                    "uv",
                    Attribute::F32(BufferAttribute::new(o.uvs.clone(), 2, false)?),
                );
            }
            let h = s.insert(NodeKind::Mesh(Mesh {
                geometry: Arc::new(g),
                materials: vec![material.clone()],
            }));
            s.add(group, h)?;
            meshes.push(h);
        }
        let n = s.get_mut(group)?;
        n.quaternion = Quaternion::from_rotation_y(PI / 2.);
        n.position.x += 0.25;
        let mut env = EnvironmentMap::from_hdr(
            &fetch("/web/gallery/assets/usdz/venice_sunset_1k.hdr").await?,
        )?;
        env.prefilter(r)?;
        s.environment = Some(Arc::new(env));
        // The gizmo circles ( Line, fog: false, transparent at 0.6 ).
        let gizmos = s.insert(NodeKind::Group);
        let mut lines = vec![];
        let mut materials = vec![];
        for (k, color) in [0xff8080, 0x80ff80, 0x8080ff].into_iter().enumerate() {
            let mut m = LineBasicMaterial::default();
            m.properties.color = Color::from_hex(color);
            m.properties.fog = false;
            m.properties.transparent = true;
            m.properties.opacity = 0.6;
            let mut active = m.clone();
            active.properties.opacity = 1.;
            let m = [
                Arc::new(Material::Line(m)),
                Arc::new(Material::Line(active)),
            ];
            let h = s.insert(NodeKind::Line(Line {
                geometry: Arc::new(BufferGeometry::default()),
                material: m[0].clone(),
                segments: false,
            }));
            let n = s.get_mut(h)?;
            match k {
                0 => n.quaternion = Quaternion::from_rotation_y(PI * 0.5),
                1 => n.quaternion = Quaternion::from_rotation_x(PI * 0.5),
                _ => {}
            }
            s.add(gizmos, h)?;
            lines.push(h);
            materials.push(m);
        }
        let (w, h, dpr) = viewport_css();
        let mut demo = Self {
            camera: c,
            lens: Lens::Perspective {
                fov: 45.,
                aspect: w / h,
            },
            near: 0.01,
            far: 2000.,
            zoom: 1.,
            meshes,
            gizmos,
            gizmo_lines: lines
                .try_into()
                .map_err(|_| Error::Invalid("arcball gizmos"))?,
            gizmo_materials: materials
                .try_into()
                .map_err(|_| Error::Invalid("arcball gizmos"))?,
            grid: None,
            target: Vector3::ZERO,
            radius_factor: 0.67,
            actions: vec![],
            mouse_op: None,
            camera_matrix_state: Matrix4::IDENTITY,
            fov_state: 1.,
            up_state: Vector3::ZERO,
            zoom_state: 1.,
            gizmo_matrix_state: Matrix4::IDENTITY,
            up0: Vector3::ZERO,
            zoom0: 1.,
            fov0: 0.,
            initial_near: 0.,
            near_pos0: 0.,
            initial_far: 0.,
            far_pos0: 0.,
            near_pos: 0.,
            far_pos: 0.,
            camera_matrix_state0: Matrix4::IDENTITY,
            gizmo_matrix_state0: Matrix4::IDENTITY,
            target0: Vector3::ZERO,
            button: -1,
            cursor: false,
            dev_px_ratio: dpr,
            down_valid: true,
            nclicks: 0,
            down_events: vec![],
            click_start: 0.,
            current_cursor: Vector3::ZERO,
            start_cursor: Vector3::ZERO,
            time_start: -1.,
            anim: Anim::None,
            time_prev: 0.,
            time_current: 0.,
            angle_prev: 0.,
            angle_current: 0.,
            cursor_prev: Vector3::ZERO,
            cursor_curr: Vector3::ZERO,
            w_prev: 0.,
            w_curr: 0.,
            adjust_near_far: false,
            scale_factor: 1.1,
            damping_factor: 25.,
            w_max: 20.,
            enable_animations: true,
            enable_grid: false,
            cursor_zoom: false,
            min_fov: 5.,
            max_fov: 90.,
            enable_pan: true,
            enable_rotate: true,
            enable_zoom: true,
            enable_focus: true,
            enabled: true,
            min_distance: 0.,
            max_distance: f64::INFINITY,
            min_zoom: 0.,
            max_zoom: f64::INFINITY,
            tb_radius: 1.,
            state: State::Idle,
            shift: false,
            ctrl: false,
            queue: vec![],
            wheel: vec![],
            keys: vec![],
            pending: vec![],
            viewport: (w, h),
            last_pointer: (0., 0.),
            paste_slot: Default::default(),
            pending_camera: None,
            pending_gizmos: None,
            transformation: (None, None),
        };
        // makePerspectiveCamera(), then the controls' setCamera.
        s.get_mut(c)?.position = Vector3::new(0., 0., PERSPECTIVE_DISTANCE);
        s.get_mut(c)?.up = Vector3::Y;
        demo.apply_lens(s)?;
        demo.set_camera(s)?;
        demo.initialize_mouse_actions();
        Ok(demo)
    }
    fn initialize_mouse_actions(&mut self) {
        for (op, mouse, key) in [
            (Op::Pan, Mouse::Button(0), Some(Key::Ctrl)),
            (Op::Pan, Mouse::Button(2), None),
            (Op::Rotate, Mouse::Button(0), None),
            (Op::Zoom, Mouse::Wheel, None),
            (Op::Zoom, Mouse::Button(1), None),
            (Op::Fov, Mouse::Wheel, Some(Key::Shift)),
            (Op::Fov, Mouse::Button(1), Some(Key::Shift)),
        ] {
            let action = Action { op, mouse, key };
            if let Some(i) = self.actions.iter().position(|a| a.matches(mouse, key)) {
                self.actions[i] = action;
            } else {
                self.actions.push(action);
            }
        }
    }
    fn op_from_action(&self, mouse: Mouse, key: Option<Key>) -> Option<Op> {
        self.actions
            .iter()
            .find(|a| a.matches(mouse, key))
            .or_else(|| key.and_then(|_| self.actions.iter().find(|a| a.matches(mouse, None))))
            .map(|a| a.op)
    }
    fn modifier(&self) -> Option<Key> {
        if self.ctrl {
            Some(Key::Ctrl)
        } else if self.shift {
            Some(Key::Shift)
        } else {
            None
        }
    }
    // ------------------------------------------------------------ camera
    fn is_perspective(&self) -> bool {
        matches!(self.lens, Lens::Perspective { .. })
    }
    fn fov(&self) -> f64 {
        match self.lens {
            Lens::Perspective { fov, .. } => fov,
            _ => 0.,
        }
    }
    /// updateProjectionMatrix(): the node's camera from the lens.
    fn apply_lens(&self, s: &mut Scene) -> Result<()> {
        let camera = match self.lens {
            Lens::Perspective { fov, aspect } => Camera::Perspective(PerspectiveCamera {
                fov,
                aspect,
                near: self.near,
                far: self.far,
                zoom: self.zoom,
                ..Default::default()
            }),
            Lens::Orthographic {
                left,
                right,
                top,
                bottom,
            } => Camera::Orthographic(OrthographicCamera {
                left,
                right,
                top,
                bottom,
                near: self.near,
                far: self.far,
                zoom: self.zoom,
                ..Default::default()
            }),
        };
        s.get_mut(self.camera)?.kind = NodeKind::Camera(camera);
        Ok(())
    }
    /// projectionMatrixInverse, in WebGL's clip space.
    fn projection_inverse(&self) -> Matrix4 {
        match self.lens {
            Lens::Perspective { fov, aspect } => {
                let fov = 2.
                    * ((fov.to_radians() * 0.5).tan() / self.zoom)
                        .atan()
                        .to_degrees();
                webgl_perspective(fov, aspect, self.near, self.far).inverse()
            }
            Lens::Orthographic {
                left,
                right,
                top,
                bottom,
            } => {
                let (dx, dy) = (
                    (right - left) / (2. * self.zoom),
                    (top - bottom) / (2. * self.zoom),
                );
                let (cx, cy) = ((right + left) / 2., (top + bottom) / 2.);
                webgl_orthographic(cx - dx, cx + dx, cy + dy, cy - dy, self.near, self.far)
                    .inverse()
            }
        }
    }
    fn camera_node(&self, s: &Scene) -> Result<(Vector3, Quaternion, Vector3)> {
        let n = s.get(self.camera)?;
        Ok((n.position, n.quaternion, n.scale))
    }
    fn camera_matrix(&self, s: &Scene) -> Result<Matrix4> {
        let (p, q, sc) = self.camera_node(s)?;
        Ok(compose(p, q, sc))
    }
    fn gizmo_position(&self, s: &Scene) -> Result<Vector3> {
        Ok(s.get(self.gizmos)?.position)
    }
    fn gizmo_matrix(&self, s: &Scene) -> Result<Matrix4> {
        let n = s.get(self.gizmos)?;
        Ok(compose(n.position, n.quaternion, n.scale))
    }
    fn set_gizmo_matrix(&self, s: &mut Scene, m: Matrix4) -> Result<()> {
        let (p, q, sc) = decompose(m);
        let n = s.get_mut(self.gizmos)?;
        n.position = p;
        n.quaternion = q;
        n.scale = sc;
        Ok(())
    }
    fn set_camera_matrix(&self, s: &mut Scene, m: Matrix4) -> Result<()> {
        let (p, q, sc) = decompose(m);
        let n = s.get_mut(self.camera)?;
        n.position = p;
        n.quaternion = q;
        n.scale = sc;
        Ok(())
    }
    /// setCamera( camera ).
    fn set_camera(&mut self, s: &mut Scene) -> Result<()> {
        s.look_at(self.camera, self.target)?;
        if let Lens::Perspective { fov, .. } = self.lens {
            self.fov0 = fov;
            self.fov_state = fov;
        }
        self.camera_matrix_state0 = self.camera_matrix(s)?;
        self.camera_matrix_state = self.camera_matrix_state0;
        self.zoom0 = self.zoom;
        self.zoom_state = self.zoom0;
        let position = s.get(self.camera)?.position;
        self.initial_near = self.near;
        self.near_pos0 = position.distance(self.target) - self.near;
        self.near_pos = self.initial_near;
        self.initial_far = self.far;
        self.far_pos0 = position.distance(self.target) - self.far;
        self.far_pos = self.initial_far;
        let up = s.get(self.camera)?.up;
        self.up0 = up;
        self.up_state = up;
        self.apply_lens(s)?;
        self.tb_radius = self.calculate_tb_radius(s)?;
        self.make_gizmos(s, self.target, self.tb_radius)
    }
    fn calculate_tb_radius(&self, s: &Scene) -> Result<f64> {
        let distance = s
            .get(self.camera)?
            .position
            .distance(self.gizmo_position(s)?);
        Ok(match self.lens {
            Lens::Perspective { fov, aspect } => {
                let half_v = (fov * 0.5).to_radians();
                let half_h = (aspect * half_v.tan()).atan();
                half_v.min(half_h).tan() * distance * self.radius_factor
            }
            Lens::Orthographic { top, right, .. } => top.min(right) * self.radius_factor,
        })
    }
    /// EllipseCurve( 0, 0, r, r ).getPoints( 128 ) as a line strip.
    fn circle(radius: f64) -> Result<Arc<BufferGeometry>> {
        let mut points = Vec::with_capacity((CURVE_POINTS + 1) * 3);
        for d in 0..=CURVE_POINTS {
            let t = d as f64 / CURVE_POINTS as f64;
            let angle = t * 2. * PI;
            points.extend([
                (radius * angle.cos()) as f32,
                (radius * angle.sin()) as f32,
                0.,
            ]);
        }
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(points, 3, false)?),
        );
        Ok(Arc::new(g))
    }
    fn set_gizmo_geometry(&self, s: &mut Scene, radius: f64) -> Result<()> {
        let g = Self::circle(radius)?;
        for h in self.gizmo_lines {
            if let NodeKind::Line(l) = &mut s.get_mut(h)?.kind {
                l.geometry = g.clone();
            }
        }
        Ok(())
    }
    fn make_gizmos(&mut self, s: &mut Scene, center: Vector3, radius: f64) -> Result<()> {
        self.gizmo_matrix_state0 = Matrix4::from_translation(center);
        self.gizmo_matrix_state = self.gizmo_matrix_state0;
        if self.zoom != 1. {
            let size = 1. / self.zoom;
            self.gizmo_matrix_state = Matrix4::from_translation(center)
                * Matrix4::from_scale(Vector3::splat(size))
                * Matrix4::from_translation(-center)
                * self.gizmo_matrix_state;
        }
        self.set_gizmo_matrix(s, self.gizmo_matrix_state)?;
        self.set_gizmo_geometry(s, radius)?;
        self.activate_gizmos(s, false)
    }
    fn activate_gizmos(&mut self, s: &mut Scene, active: bool) -> Result<()> {
        for (k, h) in self.gizmo_lines.into_iter().enumerate() {
            if let NodeKind::Line(l) = &mut s.get_mut(h)?.kind {
                l.material = self.gizmo_materials[k][usize::from(active)].clone();
            }
        }
        Ok(())
    }
    // ------------------------------------------------------------ cursor
    fn cursor_ndc(&self, x: f64, y: f64) -> Vector2 {
        let (w, h) = self.viewport;
        Vector2::new(x / w * 2. - 1., (h - y) / h * 2. - 1.)
    }
    fn cursor_position(&self, x: f64, y: f64) -> Vector2 {
        let ndc = self.cursor_ndc(x, y);
        match self.lens {
            Lens::Orthographic {
                left,
                right,
                top,
                bottom,
            } => Vector2::new(ndc.x * (right - left) * 0.5, ndc.y * (top - bottom) * 0.5),
            _ => ndc,
        }
    }
    fn unproject_near(&self, x: f64, y: f64) -> Vector3 {
        let ndc = self.cursor_ndc(x, y);
        self.projection_inverse()
            .project_point3(Vector3::new(ndc.x, ndc.y, -1.))
    }
    fn unproject_on_tb_surface(&self, s: &Scene, x: f64, y: f64, radius: f64) -> Result<Vector3> {
        if let Lens::Orthographic { .. } = self.lens {
            let p = self.cursor_position(x, y);
            let (x2, y2, r2) = (p.x * p.x, p.y * p.y, self.tb_radius * self.tb_radius);
            let z = if x2 + y2 <= r2 * 0.5 {
                (r2 - (x2 + y2)).sqrt()
            } else {
                (r2 * 0.5) / (x2 + y2).sqrt()
            };
            return Ok(Vector3::new(p.x, p.y, z));
        }
        let v = self.unproject_near(x, y);
        let mut ray = normalize(v);
        let distance = s
            .get(self.camera)?
            .position
            .distance(self.gizmo_position(s)?);
        let radius2 = radius * radius;
        let h = v.z;
        let l = (v.x * v.x + v.y * v.y).sqrt();
        if l == 0. {
            return Ok(Vector3::new(v.x, v.y, radius));
        }
        let m = h / l;
        let q = distance;
        let a = m * m + 1.;
        let b = 2. * m * q;
        let c = q * q - radius2;
        let delta = b * b - 4. * a * c;
        if delta >= 0. {
            let px = (-b - delta.sqrt()) / (2. * a);
            let py = m * px + q;
            let angle = py.atan2(px).rem_euclid(2. * PI).to_degrees();
            if angle >= 45. {
                let length = (px * px + (distance - py).powi(2)).sqrt();
                ray *= length;
                ray.z += distance;
                return Ok(ray);
            }
        }
        let (a, b, c) = (m, q, -radius2 * 0.5);
        let delta = b * b - 4. * a * c;
        let px = (-b - delta.sqrt()) / (2. * a);
        let py = m * px + q;
        let length = (px * px + (distance - py).powi(2)).sqrt();
        ray *= length;
        ray.z += distance;
        Ok(ray)
    }
    fn unproject_on_tb_plane(&self, s: &Scene, x: f64, y: f64) -> Result<Vector3> {
        if let Lens::Orthographic { .. } = self.lens {
            let p = self.cursor_position(x, y);
            return Ok(Vector3::new(p.x, p.y, 0.));
        }
        let v = self.unproject_near(x, y);
        let ray = normalize(v);
        let h = v.z;
        let l = (v.x * v.x + v.y * v.y).sqrt();
        let distance = s
            .get(self.camera)?
            .position
            .distance(self.gizmo_position(s)?);
        if l == 0. {
            return Ok(Vector3::ZERO);
        }
        let m = h / l;
        let q = distance;
        let x = -q / m;
        let length = (q * q + x * x).sqrt();
        let mut out = ray * length;
        out.z = 0.;
        Ok(out)
    }
    // ------------------------------------------------------------ transformations
    fn update_matrix_state(&mut self, s: &Scene) -> Result<()> {
        self.camera_matrix_state = self.camera_matrix(s)?;
        self.gizmo_matrix_state = self.gizmo_matrix(s)?;
        match self.lens {
            Lens::Orthographic { .. } => self.zoom_state = self.zoom,
            Lens::Perspective { fov, .. } => self.fov_state = fov,
        }
        Ok(())
    }
    fn update_tb_state(&mut self, s: &Scene, state: State, update: bool) -> Result<()> {
        self.state = state;
        if update {
            self.update_matrix_state(s)?;
        }
        Ok(())
    }
    fn apply_transform(&mut self, s: &mut Scene) -> Result<()> {
        let (camera, gizmos) = self.transformation;
        if let Some(t) = camera {
            self.set_camera_matrix(s, t * self.camera_matrix_state)?;
            if matches!(self.state, State::Rotate | State::AnimationRotate) {
                let q = s.get(self.camera)?.quaternion;
                s.get_mut(self.camera)?.up = q * self.up_state;
            }
        }
        if let Some(t) = gizmos {
            self.set_gizmo_matrix(s, t * self.gizmo_matrix_state)?;
        }
        if matches!(
            self.state,
            State::Scale | State::Focus | State::AnimationFocus
        ) {
            self.tb_radius = self.calculate_tb_radius(s)?;
            if self.adjust_near_far {
                let position = s.get(self.camera)?.position;
                let gizmo = self.gizmo_position(s)?;
                let distance = position.distance(gizmo);
                // Box3.setFromObject( gizmos ).getBoundingSphere().
                let (center, radius) = self.gizmo_sphere(s)?;
                let adjusted_near = self.near_pos0.max(radius + center.length());
                let regular_near = distance - self.initial_near;
                self.near = distance - adjusted_near.min(regular_near);
                let adjusted_far = self.far_pos0.min(-radius + center.length());
                let regular_far = distance - self.initial_far;
                self.far = distance - adjusted_far.min(regular_far);
                self.apply_lens(s)?;
            } else if self.near != self.initial_near || self.far != self.initial_far {
                self.near = self.initial_near;
                self.far = self.initial_far;
                self.apply_lens(s)?;
            }
        }
        Ok(())
    }
    /// The world box of the gizmo circles' points, as a sphere.
    fn gizmo_sphere(&self, s: &mut Scene) -> Result<(Vector3, f64)> {
        s.update()?;
        let (mut lo, mut hi) = (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        );
        for h in self.gizmo_lines {
            let n = s.get(h)?;
            if let NodeKind::Line(l) = &n.kind
                && let Some(Attribute::F32(a)) = l.geometry.get_attribute("position")
            {
                for p in a.array().chunks(3) {
                    let w = n.matrix_world.transform_point3(Vector3::new(
                        f64::from(p[0]),
                        f64::from(p[1]),
                        f64::from(p[2]),
                    ));
                    lo = lo.min(w);
                    hi = hi.max(w);
                }
            }
        }
        Ok(((lo + hi) * 0.5, (hi - lo).length() * 0.5))
    }
    fn pan(&mut self, s: &Scene, p0: Vector3, p1: Vector3, adjust: bool) -> Result<()> {
        let mut movement = p0 - p1;
        if !self.is_perspective() {
            movement *= 1. / self.zoom;
        } else if adjust {
            let a = self.camera_matrix_state0.w_axis.truncate();
            let b = self.gizmo_matrix_state0.w_axis.truncate();
            let factor = a.distance(b)
                / s.get(self.camera)?
                    .position
                    .distance(self.gizmo_position(s)?);
            movement *= 1. / factor;
        }
        let v = s.get(self.camera)?.quaternion * Vector3::new(movement.x, movement.y, 0.);
        let m = Matrix4::from_translation(v);
        self.transformation = (Some(m), Some(m));
        Ok(())
    }
    fn rotate(&mut self, s: &Scene, axis: Vector3, angle: f64) -> Result<()> {
        let point = self.gizmo_position(s)?;
        let m = Matrix4::from_translation(point)
            * Matrix4::from_axis_angle(axis, -angle)
            * Matrix4::from_translation(-point);
        self.transformation = (Some(m), None);
        Ok(())
    }
    fn scale(
        &mut self,
        s: &mut Scene,
        size: f64,
        point: Vector3,
        scale_gizmos: bool,
    ) -> Result<()> {
        let mut inverse = 1. / size;
        if !self.is_perspective() {
            self.zoom = self.zoom_state * size;
            if self.zoom > self.max_zoom {
                self.zoom = self.max_zoom;
                inverse = self.zoom_state / self.max_zoom;
            } else if self.zoom < self.min_zoom {
                self.zoom = self.min_zoom;
                inverse = self.zoom_state / self.min_zoom;
            }
            self.apply_lens(s)?;
            let g = self.gizmo_matrix_state.w_axis.truncate();
            let mut m2 = Matrix4::from_translation(g)
                * Matrix4::from_scale(Vector3::splat(inverse))
                * Matrix4::from_translation(-g);
            let mut p = point - g;
            p -= p * inverse;
            let m1 = Matrix4::from_translation(p);
            m2 = m1 * m2;
            self.transformation = (Some(m1), Some(m2));
            return Ok(());
        }
        let c = self.camera_matrix_state.w_axis.truncate();
        let g = self.gizmo_matrix_state.w_axis.truncate();
        let mut distance = c.distance(point);
        let mut amount = distance - distance * inverse;
        let new_distance = distance - amount;
        if new_distance < self.min_distance {
            inverse = self.min_distance / distance;
            amount = distance - distance * inverse;
        } else if new_distance > self.max_distance {
            inverse = self.max_distance / distance;
            amount = distance - distance * inverse;
        }
        let offset = normalize(point - c) * amount;
        let m1 = Matrix4::from_translation(offset);
        if scale_gizmos {
            distance = g.distance(point);
            amount = distance - distance * inverse;
            let offset = normalize(point - g) * amount;
            let m2 = Matrix4::from_translation(offset)
                * Matrix4::from_translation(g)
                * Matrix4::from_scale(Vector3::splat(inverse))
                * Matrix4::from_translation(-g);
            self.transformation = (Some(m1), Some(m2));
        } else {
            self.transformation = (Some(m1), None);
        }
        Ok(())
    }
    fn set_fov(&mut self, s: &mut Scene, value: f64) -> Result<()> {
        if let Lens::Perspective { aspect, .. } = self.lens {
            self.lens = Lens::Perspective {
                fov: value.clamp(self.min_fov, self.max_fov),
                aspect,
            };
            self.apply_lens(s)?;
        }
        Ok(())
    }
    fn calculate_rotation_axis(&self, a: Vector3, b: Vector3) -> Vector3 {
        let (_, q, _) = decompose(self.camera_matrix_state);
        normalize(q * a.cross(b))
    }
    fn focus(&mut self, s: &mut Scene, point: Vector3, size: f64, amount: f64) -> Result<()> {
        let offset = (point - self.gizmo_position(s)?) * amount;
        let translation = Matrix4::from_translation(offset);
        let gizmo_temp = self.gizmo_matrix_state;
        self.gizmo_matrix_state = translation * self.gizmo_matrix_state;
        self.set_gizmo_matrix(s, self.gizmo_matrix_state)?;
        let camera_temp = self.camera_matrix_state;
        self.camera_matrix_state = translation * self.camera_matrix_state;
        self.set_camera_matrix(s, self.camera_matrix_state)?;
        if self.enable_zoom {
            let g = self.gizmo_position(s)?;
            self.scale(s, size, g, true)?;
            self.apply_transform(s)?;
        }
        self.gizmo_matrix_state = gizmo_temp;
        self.camera_matrix_state = camera_temp;
        Ok(())
    }
    fn draw_grid(&mut self, s: &mut Scene) -> Result<()> {
        if self.grid.is_some() {
            return Ok(());
        }
        let multiplier = 3.;
        let (size, divisions) = match self.lens {
            Lens::Orthographic {
                left,
                right,
                top,
                bottom,
            } => {
                let (width, height) = (right - left, bottom - top);
                let max_length = width.max(height);
                let tick = max_length / 20.;
                let size = max_length / self.zoom * multiplier;
                (size, size / tick * self.zoom)
            }
            Lens::Perspective { fov, aspect } => {
                let distance = s
                    .get(self.camera)?
                    .position
                    .distance(self.gizmo_position(s)?);
                let half_v = (fov * 0.5).to_radians();
                let half_h = (aspect * half_v.tan()).atan();
                let max_length = half_v.max(half_h).tan() * distance * 2.;
                let tick = max_length / 20.;
                let size = max_length * multiplier;
                (size, size / tick)
            }
        };
        // GridHelper( size, divisions, 0x888888, 0x888888 ).
        let (step, half) = (size / divisions, size / 2.);
        let c = Color::from_hex(0x888888).0;
        let (mut vertices, mut colors) = (vec![], vec![]);
        let mut k = -half;
        let mut i = 0.;
        while i <= divisions {
            vertices
                .extend([-half, 0., k, half, 0., k, k, 0., -half, k, 0., half].map(|v| v as f32));
            for _ in 0..4 {
                colors.extend(c.to_array().map(|v| v as f32));
            }
            i += 1.;
            k += step;
        }
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(vertices, 3, false)?),
        );
        g.set_attribute(
            "color",
            Attribute::F32(BufferAttribute::new(colors, 3, false)?),
        );
        let mut m = LineBasicMaterial::default();
        m.properties.vertex_colors = true;
        m.properties.tone_mapped = false;
        let grid = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Line(m)),
            segments: true,
        }));
        let position = self.gizmo_position(s)?;
        let q = s.get(self.camera)?.quaternion;
        let n = s.get_mut(grid)?;
        n.position = position;
        n.quaternion = q * Quaternion::from_rotation_x(PI * 0.5);
        self.grid = Some(grid);
        Ok(())
    }
    fn dispose_grid(&mut self, s: &mut Scene) -> Result<()> {
        if let Some(g) = self.grid.take() {
            s.dispose(g)?;
        }
        Ok(())
    }
    fn cancel_animation(&mut self) {
        self.anim = Anim::None;
        self.time_start = -1.;
    }
    // ------------------------------------------------------------ events
    fn single_pan_start(&mut self, s: &mut Scene, x: f64, y: f64, op: Op) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        let animating = !matches!(self.anim, Anim::None);
        match op {
            Op::Pan => {
                if !self.enable_pan {
                    return Ok(());
                }
                if animating {
                    self.cancel_animation();
                    self.activate_gizmos(s, false)?;
                }
                self.update_tb_state(s, State::Pan, true)?;
                self.start_cursor = self.unproject_on_tb_plane(s, x, y)?;
                if self.enable_grid {
                    self.draw_grid(s)?;
                }
            }
            Op::Rotate => {
                if !self.enable_rotate {
                    return Ok(());
                }
                if animating {
                    self.cancel_animation();
                }
                self.update_tb_state(s, State::Rotate, true)?;
                self.start_cursor = self.unproject_on_tb_surface(s, x, y, self.tb_radius)?;
                self.activate_gizmos(s, true)?;
                if self.enable_animations {
                    self.time_prev = now();
                    self.time_current = self.time_prev;
                    self.angle_current = 0.;
                    self.angle_prev = 0.;
                    self.cursor_prev = self.start_cursor;
                    self.cursor_curr = self.cursor_prev;
                    self.w_curr = 0.;
                    self.w_prev = 0.;
                }
            }
            Op::Fov | Op::Zoom => {
                if (op == Op::Fov && !self.is_perspective()) || !self.enable_zoom {
                    return Ok(());
                }
                if animating {
                    self.cancel_animation();
                    self.activate_gizmos(s, false)?;
                }
                let state = if op == Op::Fov {
                    State::Fov
                } else {
                    State::Scale
                };
                self.update_tb_state(s, state, true)?;
                self.start_cursor.y = self.cursor_ndc(x, y).y * 0.5;
                self.current_cursor = self.start_cursor;
            }
        }
        Ok(())
    }
    fn single_pan_move(&mut self, s: &mut Scene, x: f64, y: f64, state: State) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        let restart = state != self.state;
        match state {
            State::Pan if self.enable_pan => {
                if restart {
                    self.update_tb_state(s, state, true)?;
                    self.start_cursor = self.unproject_on_tb_plane(s, x, y)?;
                    if self.enable_grid {
                        self.draw_grid(s)?;
                    }
                    self.activate_gizmos(s, false)?;
                } else {
                    self.current_cursor = self.unproject_on_tb_plane(s, x, y)?;
                    self.pan(s, self.start_cursor, self.current_cursor, false)?;
                    self.apply_transform(s)?;
                }
            }
            State::Rotate if self.enable_rotate => {
                if restart {
                    self.update_tb_state(s, state, true)?;
                    self.start_cursor = self.unproject_on_tb_surface(s, x, y, self.tb_radius)?;
                    if self.enable_grid {
                        self.dispose_grid(s)?;
                    }
                    self.activate_gizmos(s, true)?;
                } else {
                    self.current_cursor = self.unproject_on_tb_surface(s, x, y, self.tb_radius)?;
                    let distance = self.start_cursor.distance(self.current_cursor);
                    let angle = self.start_cursor.angle_between(self.current_cursor);
                    let amount = (distance / self.tb_radius).max(angle);
                    let axis = self.calculate_rotation_axis(self.start_cursor, self.current_cursor);
                    self.rotate(s, axis, amount)?;
                    self.apply_transform(s)?;
                    if self.enable_animations {
                        self.time_prev = self.time_current;
                        self.time_current = now();
                        self.angle_prev = self.angle_current;
                        self.angle_current = amount;
                        self.cursor_prev = self.cursor_curr;
                        self.cursor_curr = self.current_cursor;
                        self.w_prev = self.w_curr;
                        let t = (self.time_current - self.time_prev) / 1000.;
                        self.w_curr = if t == 0. {
                            0.
                        } else {
                            (self.angle_current - self.angle_prev) / t
                        };
                    }
                }
            }
            State::Scale if self.enable_zoom => {
                if restart {
                    self.update_tb_state(s, state, true)?;
                    self.start_cursor.y = self.cursor_ndc(x, y).y * 0.5;
                    self.current_cursor = self.start_cursor;
                    if self.enable_grid {
                        self.dispose_grid(s)?;
                    }
                    self.activate_gizmos(s, false)?;
                } else {
                    self.current_cursor.y = self.cursor_ndc(x, y).y * 0.5;
                    let size = self.notch_size(self.current_cursor.y - self.start_cursor.y);
                    let g = self.gizmo_matrix_state.w_axis.truncate();
                    self.scale(s, size, g, true)?;
                    self.apply_transform(s)?;
                }
            }
            State::Fov if self.enable_zoom && self.is_perspective() => {
                if restart {
                    self.update_tb_state(s, state, true)?;
                    self.start_cursor.y = self.cursor_ndc(x, y).y * 0.5;
                    self.current_cursor = self.start_cursor;
                    if self.enable_grid {
                        self.dispose_grid(s)?;
                    }
                    self.activate_gizmos(s, false)?;
                } else {
                    self.current_cursor.y = self.cursor_ndc(x, y).y * 0.5;
                    let size = self.notch_size(self.current_cursor.y - self.start_cursor.y);
                    let x0 = self
                        .camera_matrix_state
                        .w_axis
                        .truncate()
                        .distance(self.gizmo_position(s)?);
                    let x_new = (x0 / size).clamp(self.min_distance, self.max_distance);
                    let y0 = x0 * (self.fov_state * 0.5).to_radians().tan();
                    let new_fov = ((y0 / x_new).atan() * 2.)
                        .to_degrees()
                        .clamp(self.min_fov, self.max_fov);
                    let new_distance = y0 / (new_fov / 2.).to_radians().tan();
                    let size = x0 / new_distance;
                    let g = self.gizmo_matrix_state.w_axis.truncate();
                    self.set_fov(s, new_fov)?;
                    self.scale(s, size, g, false)?;
                    self.apply_transform(s)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    /// The drag zoom: eight wheel notches per half screen of movement.
    fn notch_size(&self, movement: f64) -> f64 {
        if movement < 0. {
            1. / self.scale_factor.powf(-movement * 8.)
        } else if movement > 0. {
            self.scale_factor.powf(movement * 8.)
        } else {
            1.
        }
    }
    fn single_pan_end(&mut self, s: &mut Scene) -> Result<()> {
        if self.state == State::Rotate {
            if !self.enable_rotate {
                return Ok(());
            }
            if self.enable_animations && now() - self.time_current < 120. {
                let w = ((self.w_prev + self.w_curr) / 2.).abs();
                let axis = self.calculate_rotation_axis(self.cursor_prev, self.cursor_curr);
                self.update_tb_state(s, State::AnimationRotate, true)?;
                self.time_start = -1.;
                self.anim = Anim::Rotate {
                    axis,
                    w0: w.min(self.w_max),
                };
            } else {
                self.update_tb_state(s, State::Idle, false)?;
                self.activate_gizmos(s, false)?;
            }
        } else if matches!(self.state, State::Pan | State::Idle) {
            self.update_tb_state(s, State::Idle, false)?;
            if self.enable_grid {
                self.dispose_grid(s)?;
            }
            self.activate_gizmos(s, false)?;
        }
        Ok(())
    }
    fn double_tap(&mut self, s: &mut Scene, x: f64, y: f64) -> Result<()> {
        if !(self.enabled && self.enable_pan && self.enable_focus) {
            return Ok(());
        }
        // unprojectOnObj: the first mesh face under the cursor.
        s.update()?;
        let ndc = self.cursor_ndc(x, y);
        let (camera, world) = s.camera(self.camera)?;
        let mut ray = Raycaster::default();
        ray.set_from_camera(ndc, camera, world)?;
        ray.near = self.near;
        ray.far = self.far;
        let hit = ray
            .intersect_objects(s, &self.meshes, false)?
            .first()
            .map(|h| h.point);
        let Some(point) = hit else {
            return Ok(());
        };
        if self.enable_animations {
            self.time_start = -1.;
            self.update_tb_state(s, State::AnimationFocus, true)?;
            self.anim = Anim::Focus {
                point,
                gizmo: self.gizmo_matrix_state,
            };
        } else {
            self.update_tb_state(s, State::Focus, true)?;
            self.focus(s, point, self.scale_factor, 1.)?;
            self.update_tb_state(s, State::Idle, false)?;
        }
        Ok(())
    }
    fn pointer_down(&mut self, s: &mut Scene, button: i32, x: f64, y: f64) -> Result<()> {
        if button == 0 {
            self.down_valid = true;
            self.down_events.push(Click { x, y, time: now() });
        } else {
            self.down_valid = false;
        }
        if !self.cursor {
            self.mouse_op = self.op_from_action(Mouse::Button(button), self.modifier());
            if let Some(op) = self.mouse_op {
                self.cursor = true;
                self.button = button;
                self.single_pan_start(s, x, y, op)?;
            }
        }
        Ok(())
    }
    fn pointer_move(&mut self, s: &mut Scene, x: f64, y: f64) -> Result<()> {
        if self.cursor
            && let Some(op) = self.op_from_action(Mouse::Button(self.button), self.modifier())
        {
            self.single_pan_move(s, x, y, state_of(op))?;
        }
        if self.down_valid
            && let Some(last) = self.down_events.last()
            && ((x - last.x).powi(2) + (y - last.y).powi(2)).sqrt() * self.dev_px_ratio > 24.
        {
            self.down_valid = false;
        }
        Ok(())
    }
    fn pointer_up(&mut self, s: &mut Scene, x: f64, y: f64) -> Result<()> {
        if self.cursor {
            self.cursor = false;
            self.single_pan_end(s)?;
            self.button = -1;
        }
        let t = now();
        if self.down_valid {
            let down = self.down_events.last().map_or(t, |e| e.time);
            if t - down <= 250. {
                if self.nclicks == 0 {
                    self.nclicks = 1;
                    self.click_start = t;
                } else {
                    let interval = t - self.click_start;
                    let movement = match (self.down_events.first(), self.down_events.get(1)) {
                        (Some(a), Some(b)) => {
                            ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt() * self.dev_px_ratio
                        }
                        _ => 0.,
                    };
                    if interval <= 300. && movement <= 24. {
                        self.nclicks = 0;
                        self.down_events.clear();
                        self.double_tap(s, x, y)?;
                    } else {
                        self.nclicks = 1;
                        if !self.down_events.is_empty() {
                            self.down_events.remove(0);
                        }
                        self.click_start = t;
                    }
                }
            } else {
                self.down_valid = false;
                self.nclicks = 0;
                self.down_events.clear();
            }
        } else {
            self.nclicks = 0;
            self.down_events.clear();
        }
        Ok(())
    }
    fn on_wheel(&mut self, s: &mut Scene, delta_x: f64, delta_y: f64) -> Result<()> {
        if !(self.enabled && self.enable_zoom) {
            return Ok(());
        }
        let Some(op) = self.op_from_action(Mouse::Wheel, self.modifier()) else {
            return Ok(());
        };
        let sgn = delta_y / 125.;
        let mut size = if sgn > 0. {
            1. / self.scale_factor
        } else if sgn < 0. {
            self.scale_factor
        } else {
            1.
        };
        match op {
            Op::Zoom => {
                self.update_tb_state(s, State::Scale, true)?;
                if sgn > 0. {
                    size = 1. / self.scale_factor.powf(sgn);
                } else if sgn < 0. {
                    size = self.scale_factor.powf(-sgn);
                }
                let g = self.gizmo_position(s)?;
                if self.cursor_zoom && self.enable_pan {
                    let (x, y) = self.last_pointer;
                    let q = s.get(self.camera)?.quaternion;
                    let mut point = q * self.unproject_on_tb_plane(s, x, y)?;
                    if !self.is_perspective() {
                        point *= 1. / self.zoom;
                    }
                    self.scale(s, size, point + g, true)?;
                } else {
                    self.scale(s, size, g, true)?;
                }
                self.apply_transform(s)?;
                if self.grid.is_some() {
                    self.dispose_grid(s)?;
                    self.draw_grid(s)?;
                }
                self.update_tb_state(s, State::Idle, false)?;
            }
            Op::Fov => {
                if self.is_perspective() {
                    self.update_tb_state(s, State::Fov, true)?;
                    if delta_x != 0. {
                        let sgn = delta_x / 125.;
                        size = if sgn > 0. {
                            1. / self.scale_factor.powf(sgn)
                        } else if sgn < 0. {
                            self.scale_factor.powf(-sgn)
                        } else {
                            1.
                        };
                    }
                    let x0 = self
                        .camera_matrix_state
                        .w_axis
                        .truncate()
                        .distance(self.gizmo_position(s)?);
                    let x_new = (x0 / size).clamp(self.min_distance, self.max_distance);
                    let y0 = x0 * (self.fov() * 0.5).to_radians().tan();
                    let new_fov = ((y0 / x_new).atan() * 2.)
                        .to_degrees()
                        .clamp(self.min_fov, self.max_fov);
                    let new_distance = y0 / (new_fov / 2.).to_radians().tan();
                    let size = x0 / new_distance;
                    self.set_fov(s, new_fov)?;
                    let g = self.gizmo_position(s)?;
                    self.scale(s, size, g, false)?;
                    self.apply_transform(s)?;
                }
                if self.grid.is_some() {
                    self.dispose_grid(s)?;
                    self.draw_grid(s)?;
                }
                self.update_tb_state(s, State::Idle, false)?;
            }
            _ => {}
        }
        Ok(())
    }
    /// The requestAnimationFrame steps of the focus and inertia animations.
    fn animate(&mut self, s: &mut Scene, time: f64) -> Result<()> {
        match self.anim {
            Anim::None => {}
            Anim::Focus { point, gizmo } => {
                if self.time_start == -1. {
                    self.time_start = time;
                }
                if self.state != State::AnimationFocus {
                    self.cancel_animation();
                    return Ok(());
                }
                let t = (time - self.time_start) / 500.;
                self.gizmo_matrix_state = gizmo;
                self.set_gizmo_matrix(s, gizmo)?;
                if t >= 1. {
                    self.focus(s, point, self.scale_factor, 1.)?;
                    self.cancel_animation();
                    self.update_tb_state(s, State::Idle, false)?;
                    self.activate_gizmos(s, false)?;
                } else {
                    let amount = 1. - (1. - t).powi(3);
                    let size = (1. - amount) + self.scale_factor * amount;
                    self.focus(s, point, size, amount)?;
                }
            }
            Anim::Rotate { axis, w0 } => {
                if self.time_start == -1. {
                    self.angle_prev = 0.;
                    self.angle_current = 0.;
                    self.time_start = time;
                }
                if self.state != State::AnimationRotate {
                    self.cancel_animation();
                    if self.state != State::Rotate {
                        self.activate_gizmos(s, false)?;
                    }
                    return Ok(());
                }
                let dt = (time - self.time_start) / 1000.;
                let w = w0 - self.damping_factor * dt;
                if w > 0. {
                    self.angle_current = 0.5 * -self.damping_factor * dt * dt + w0 * dt;
                    self.rotate(s, axis, self.angle_current)?;
                    self.apply_transform(s)?;
                } else {
                    self.cancel_animation();
                    self.update_tb_state(s, State::Idle, false)?;
                    self.activate_gizmos(s, false)?;
                }
            }
        }
        Ok(())
    }
    /// reset().
    fn reset(&mut self, s: &mut Scene) -> Result<()> {
        self.target = self.target0;
        self.zoom = self.zoom0;
        if let Lens::Perspective { aspect, .. } = self.lens {
            self.lens = Lens::Perspective {
                fov: self.fov0,
                aspect,
            };
        }
        self.near = self.near_pos;
        self.far = self.far_pos;
        self.camera_matrix_state = self.camera_matrix_state0;
        self.set_camera_matrix(s, self.camera_matrix_state)?;
        s.get_mut(self.camera)?.up = self.up0;
        self.apply_lens(s)?;
        self.gizmo_matrix_state = self.gizmo_matrix_state0;
        self.set_gizmo_matrix(s, self.gizmo_matrix_state0)?;
        self.tb_radius = self.calculate_tb_radius(s)?;
        let g = self.gizmo_position(s)?;
        self.make_gizmos(s, g, self.tb_radius)?;
        s.look_at(self.camera, g)?;
        self.update_tb_state(s, State::Idle, false)
    }
    /// copyState(): the arcballState JSON to the clipboard.
    fn copy_state(&self, s: &Scene) -> Result<()> {
        let n = s.get(self.camera)?;
        let camera = compose(n.position, n.quaternion, n.scale).to_cols_array();
        let gizmo = self.gizmo_matrix(s)?.to_cols_array();
        let mut state = serde_json::json!({
            "cameraFar": self.far,
            "cameraMatrix": {"elements": camera},
            "cameraNear": self.near,
            "cameraUp": {"x": n.up.x, "y": n.up.y, "z": n.up.z},
            "cameraZoom": self.zoom,
            "gizmoMatrix": {"elements": gizmo},
            "target": [self.target.x, self.target.y, self.target.z],
        });
        if let Lens::Perspective { fov, .. } = self.lens {
            state["cameraFov"] = fov.into();
        }
        let text = serde_json::json!({ "arcballState": state }).to_string();
        clipboard_call("writeText", Some(&text));
        Ok(())
    }
    /// setStateFromJSON( json ).
    fn set_state(&mut self, s: &mut Scene, json: &str) -> Result<()> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| Error::Asset(e.to_string()))?;
        let state = &value["arcballState"];
        if state.is_null() {
            return Ok(());
        }
        let f = |v: &serde_json::Value| v.as_f64().unwrap_or(0.);
        let elements = |v: &serde_json::Value| -> Option<Matrix4> {
            let a: Vec<f64> = v["elements"].as_array()?.iter().map(f).collect();
            (a.len() == 16).then(|| Matrix4::from_cols_slice(&a))
        };
        if let Some(t) = state["target"].as_array() {
            self.target = Vector3::new(f(&t[0]), f(&t[1]), f(&t[2]));
        }
        if let Some(m) = elements(&state["cameraMatrix"]) {
            self.camera_matrix_state = m;
            self.set_camera_matrix(s, m)?;
        }
        let up = &state["cameraUp"];
        s.get_mut(self.camera)?.up = Vector3::new(f(&up["x"]), f(&up["y"]), f(&up["z"]));
        self.near = f(&state["cameraNear"]);
        self.far = f(&state["cameraFar"]);
        self.zoom = f(&state["cameraZoom"]);
        if let Lens::Perspective { aspect, .. } = self.lens {
            self.lens = Lens::Perspective {
                fov: f(&state["cameraFov"]),
                aspect,
            };
        }
        if let Some(m) = elements(&state["gizmoMatrix"]) {
            self.gizmo_matrix_state = m;
            self.set_gizmo_matrix(s, m)?;
        }
        self.apply_lens(s)?;
        self.tb_radius = self.calculate_tb_radius(s)?;
        let saved = self.gizmo_matrix_state0;
        let g = self.gizmo_position(s)?;
        self.make_gizmos(s, g, self.tb_radius)?;
        self.gizmo_matrix_state0 = saved;
        s.look_at(self.camera, g)?;
        self.update_tb_state(s, State::Idle, false)
    }
    /// The page's setCamera( type ): a new camera at the type's distance.
    fn switch_camera(&mut self, s: &mut Scene, orthographic: bool) -> Result<()> {
        let (w, h) = self.viewport;
        self.zoom = 1.;
        self.near = 0.01;
        self.far = 2000.;
        if orthographic {
            let half_v = (45f64 * 0.5).to_radians();
            let half_h = ((w / h) * half_v.tan()).atan();
            let (half_w, half_h) = (
                PERSPECTIVE_DISTANCE * half_h.tan(),
                PERSPECTIVE_DISTANCE * half_v.tan(),
            );
            self.lens = Lens::Orthographic {
                left: -half_w,
                right: half_w,
                top: half_h,
                bottom: -half_h,
            };
        } else {
            self.lens = Lens::Perspective {
                fov: 45.,
                aspect: w / h,
            };
        }
        let n = s.get_mut(self.camera)?;
        n.position = Vector3::new(
            0.,
            0.,
            if orthographic {
                ORTHOGRAPHIC_DISTANCE
            } else {
                PERSPECTIVE_DISTANCE
            },
        );
        n.quaternion = Quaternion::IDENTITY;
        n.scale = Vector3::ONE;
        n.up = Vector3::Y;
        self.set_camera(s)
    }
    /// The page's and the controls' resize listeners.
    fn resize(&mut self, s: &mut Scene, w: f64, h: f64) -> Result<()> {
        self.viewport = (w, h);
        match &mut self.lens {
            Lens::Orthographic {
                left,
                right,
                top,
                bottom,
            } => {
                let half_v = (45f64 * 0.5).to_radians();
                let half_h = ((w / h) * half_v.tan()).atan();
                let (half_w, half_hh) = (
                    PERSPECTIVE_DISTANCE * half_h.tan(),
                    PERSPECTIVE_DISTANCE * half_v.tan(),
                );
                (*left, *right, *top, *bottom) = (-half_w, half_w, half_hh, -half_hh);
            }
            Lens::Perspective { aspect, .. } => *aspect = w / h,
        }
        self.apply_lens(s)?;
        let scale = {
            let sc = s.get(self.gizmos)?.scale;
            (sc.x + sc.y + sc.z) / 3.
        };
        self.tb_radius = self.calculate_tb_radius(s)?;
        self.set_gizmo_geometry(s, self.tb_radius / scale)
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let (w, h, _) = viewport_css();
        if (w, h) != self.viewport {
            self.resize(s, w, h)?;
        }
        for (code, down) in std::mem::take(&mut self.keys) {
            match code {
                16 => self.shift = down,
                17 | 91 | 93 => self.ctrl = down,
                67 if down && self.ctrl => self.copy_state(s)?,
                86 if down && self.ctrl => self.pending.push(16),
                _ => {}
            }
        }
        if let Some(orthographic) = self.pending_camera.take() {
            self.switch_camera(s, orthographic)?;
        }
        if let Some(visible) = self.pending_gizmos.take() {
            s.get_mut(self.gizmos)?.visible = visible;
        }
        for (kind, x, y) in std::mem::take(&mut self.queue) {
            self.last_pointer = (x, y);
            match kind {
                10..=12 => self.pointer_down(s, kind as i32 - 10, x, y)?,
                0 => self.pointer_move(s, x, y)?,
                20..=22 => self.pointer_up(s, x, y)?,
                _ => {}
            }
        }
        for (dx, dy) in std::mem::take(&mut self.wheel) {
            self.on_wheel(s, dx, dy)?;
        }
        for index in std::mem::take(&mut self.pending) {
            match index {
                15 => self.copy_state(s)?,
                16 => clipboard_paste(&self.paste_slot),
                17 => self.reset(s)?,
                _ => {}
            }
        }
        let pasted = self
            .paste_slot
            .try_borrow_mut()
            .ok()
            .and_then(|mut slot| slot.take());
        if let Some(text) = pasted {
            self.set_state(s, &text)?;
        }
        self.animate(s, now())?;
        Ok(())
    }
    /// Absolute CSS pointer events ( 0 move, 10 + button down, 20 + button
    /// up ) and 30 for the wheel's deltas.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if kind == 30 {
            self.wheel.push((x, y));
        } else {
            self.queue.push((kind, x, y));
        }
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.keys.push((code, down));
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        let _ = wheel;
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let on = value != 0.;
        let v = f64::from(value);
        match index {
            0 => self.pending_camera = Some(value < 0.5),
            1 => self.enabled = on,
            2 => self.enable_focus = on,
            3 => self.enable_grid = on,
            4 => self.enable_rotate = on,
            5 => self.enable_pan = on,
            6 => self.enable_zoom = on,
            7 => self.cursor_zoom = on,
            8 => self.adjust_near_far = on,
            9 => self.scale_factor = v,
            10 => self.min_distance = v,
            11 => self.max_distance = v,
            12 => self.min_zoom = v,
            13 => self.max_zoom = v,
            14 => self.pending_gizmos = Some(on),
            15..=17 => self.pending.push(index),
            18 => self.enable_animations = on,
            19 => self.damping_factor = v,
            20 => self.w_max = v,
            _ => return Err(Error::Invalid("arcball parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}

/// navigator.clipboard[ method ]( text ).
fn clipboard_call(method: &str, text: Option<&str>) -> Option<js_sys::Promise> {
    let navigator = web_sys::window()?.navigator();
    let clipboard = js_sys::Reflect::get(&navigator, &"clipboard".into()).ok()?;
    let f: js_sys::Function = js_sys::Reflect::get(&clipboard, &method.into())
        .ok()?
        .dyn_into()
        .ok()?;
    let result = match text {
        Some(t) => f.call1(&clipboard, &t.into()),
        None => f.call0(&clipboard),
    };
    result.ok()?.dyn_into().ok()
}
/// pasteState(): readText(), then setStateFromJSON on the next frame.
fn clipboard_paste(slot: &std::rc::Rc<std::cell::RefCell<Option<String>>>) {
    let Some(promise) = clipboard_call("readText", None) else {
        return;
    };
    let slot = slot.clone();
    wasm_bindgen_futures::spawn_local(async move {
        if let Ok(v) = wasm_bindgen_futures::JsFuture::from(promise).await
            && let Some(text) = v.as_string()
        {
            *slot.borrow_mut() = Some(text);
        }
    });
}
