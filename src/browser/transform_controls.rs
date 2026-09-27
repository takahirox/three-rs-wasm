//! TransformControls: the translate, rotate and scale gizmo, its invisible
//! pickers, the drag helpers and the drag plane, ported from the pinned addon.
//! The gizmo is resident geometry; only handle transforms and the highlighted
//! material choice change per update.
use crate::{
    Result, attribute::BufferAttribute, camera::Camera, geometry::*, material::*, math::*,
    raycast::Raycaster, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Mode {
    Translate,
    Rotate,
    Scale,
}
/// Events in dispatch order; the examples drain them after each call.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Event {
    Change,
    DraggingChanged(bool),
    ObjectChange,
    MouseDown,
    MouseUp,
}
/// The gizmo materials by `materialLib` role.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Look {
    Invisible,
    Helper,
    Red,
    Green,
    Blue,
    RedT,
    GreenT,
    BlueT,
    WhiteT,
    YellowT,
    Gray,
}
impl Look {
    /// ( color, opacity ) of the cloned gizmo material.
    fn style(self) -> (u32, f64) {
        match self {
            Self::Invisible => (0xffffff, 0.15),
            Self::Helper => (0xffffff, 0.5),
            Self::Red => (0xff0000, 1.),
            Self::Green => (0x00ff00, 1.),
            Self::Blue => (0x0000ff, 1.),
            Self::RedT => (0xff0000, 0.5),
            Self::GreenT => (0x00ff00, 0.5),
            Self::BlueT => (0x0000ff, 0.5),
            Self::WhiteT => (0xffffff, 0.25),
            Self::YellowT => (0xffff00, 0.25),
            Self::Gray => (0x787878, 1.),
        }
    }
}
fn gizmo_material(color: u32, opacity: f64, line: bool) -> Arc<Material> {
    let mut p = MaterialProperties {
        color: Color::from_hex(color),
        opacity,
        ..Default::default()
    };
    p.depth_test = false;
    p.depth_write = false;
    p.fog = false;
    p.tone_mapped = false;
    p.transparent = true;
    Arc::new(if line {
        Material::Line(LineBasicMaterial {
            properties: p,
            ..Default::default()
        })
    } else {
        Material::Basic(MeshBasicMaterial { properties: p })
    })
}
enum Shape {
    Mesh(BufferGeometry),
    Line(BufferGeometry),
}
/// One `[ object, position, rotation, scale, tag ]` gizmo definition entry.
struct Spec {
    shape: Shape,
    look: Look,
    position: Option<[f64; 3]>,
    rotation: Option<[f64; 3]>,
    scale: Option<[f64; 3]>,
    helper: bool,
}
fn spec(shape: Shape, look: Look, position: Option<[f64; 3]>, rotation: Option<[f64; 3]>) -> Spec {
    Spec {
        shape,
        look,
        position,
        rotation,
        scale: None,
        helper: false,
    }
}
fn helper(
    shape: Shape,
    position: Option<[f64; 3]>,
    rotation: Option<[f64; 3]>,
    scale: Option<[f64; 3]>,
) -> Spec {
    Spec {
        shape,
        look: Look::Helper,
        position,
        rotation,
        scale,
        helper: true,
    }
}
fn line_geometry(points: [f32; 6]) -> Result<BufferGeometry> {
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(points.to_vec(), 3, false)?),
    );
    Ok(g)
}
fn cylinder(top: f64, bottom: f64, height: f64, radial: u32) -> Result<BufferGeometry> {
    CylinderGeometry::build(top, bottom, height, radial, 1, false, 0., 2. * PI)
}
fn torus(radius: f64, tube: f64, radial: u32, tubular: u32, arc: f64) -> Result<BufferGeometry> {
    TorusGeometry::build(radius, tube, radial, tubular, arc, 0., 2. * PI)
}
/// The gizmo's `CircleGeometry( radius, arc )`: a thin torus, rotated.
fn circle(radius: f64, arc: f64) -> Result<BufferGeometry> {
    let mut g = torus(radius, 0.0075, 3, 64, arc * PI * 2.)?;
    g.rotate_y(PI / 2.)?;
    g.rotate_x(PI / 2.)?;
    Ok(g)
}
struct Handle {
    node: Object3D,
    name: &'static str,
    helper: bool,
    picker: bool,
    base: Arc<Material>,
    active: Arc<Material>,
}
/// Handles per mode, in `picker`, `gizmo`, `helper` order as the update visits them.
struct ModeHandles {
    groups: [Object3D; 3],
    handles: Vec<Handle>,
}
pub(super) struct TransformControls {
    pub(super) root: Object3D,
    modes: [ModeHandles; 3],
    pub(super) camera: Object3D,
    pub(super) object: Option<Object3D>,
    pub(super) enabled: bool,
    pub(super) axis: Option<&'static str>,
    pub(super) mode: Mode,
    pub(super) translation_snap: Option<f64>,
    pub(super) rotation_snap: Option<f64>,
    pub(super) scale_snap: Option<f64>,
    pub(super) local: bool,
    pub(super) size: f64,
    pub(super) dragging: bool,
    pub(super) show: [bool; 3],
    /// Examples whose 'change' listener renders run the update (the root's
    /// updateMatrixWorld) at every change event.
    pub(super) render_on_change: bool,
    pub(super) events: Vec<Event>,
    world_position: Vector3,
    world_position_start: Vector3,
    world_quaternion: Quaternion,
    world_quaternion_start: Quaternion,
    camera_position: Vector3,
    camera_quaternion: Quaternion,
    point_start: Vector3,
    point_end: Vector3,
    rotation_axis: Vector3,
    rotation_angle: f64,
    eye: Vector3,
    parent_quaternion: Quaternion,
    parent_scale: Vector3,
    world_quaternion_inv: Quaternion,
    parent_quaternion_inv: Quaternion,
    position_start: Vector3,
    quaternion_start: Quaternion,
    scale_start: Vector3,
    /// The drag plane's world transform as of the last update.
    plane: (Vector3, Quaternion),
    /// `_dirVector` persists between updates, as the module variable does.
    dir: Vector3,
}
fn norm(v: Vector3) -> Vector3 {
    let l = v.length();
    if l == 0. { v } else { v / l }
}
/// `Quaternion.setFromAxisAngle` (the axis is used as given).
fn axis_angle(axis: Vector3, angle: f64) -> Quaternion {
    let s = (angle / 2.).sin();
    Quaternion::from_xyzw(axis.x * s, axis.y * s, axis.z * s, (angle / 2.).cos())
}
fn angle_to(a: Vector3, b: Vector3) -> f64 {
    let d = (a.length_squared() * b.length_squared()).sqrt();
    if d == 0. {
        return PI / 2.;
    }
    (a.dot(b) / d).clamp(-1., 1.).acos()
}
/// `Matrix4.lookAt( eye, target, up )` as a rotation.
fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Quaternion {
    let mut z = eye - target;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    z = z.normalize();
    let mut x = up.cross(z);
    if x.length_squared() == 0. {
        if up.z.abs() == 1. {
            z.x += 0.0001;
        } else {
            z.z += 0.0001;
        }
        z = z.normalize();
        x = up.cross(z);
    }
    x = x.normalize();
    Quaternion::from_mat3(&Matrix3::from_cols(x, z.cross(x), z))
}
fn euler(r: [f64; 3]) -> Quaternion {
    Euler {
        angles: Vector3::from_array(r),
        order: EulerOrder::XYZ,
    }
    .quaternion()
}
fn decompose(m: Matrix4) -> (Vector3, Quaternion, Vector3) {
    let (s, q, t) = m.to_scale_rotation_translation();
    (t, q, s)
}
/// `Math.round( v / step ) * step`: halves round up.
fn snap(v: f64, step: f64) -> f64 {
    (v / step + 0.5).floor() * step
}
impl TransformControls {
    pub(super) fn new(s: &mut Scene, camera: Object3D) -> Result<Self> {
        let arrow = || -> Result<BufferGeometry> {
            let mut g = cylinder(0., 0.04, 0.1, 12)?;
            g.translate(Vector3::new(0., 0.05, 0.))?;
            Ok(g)
        };
        let scale_handle = || -> Result<BufferGeometry> {
            let mut g = BoxGeometry::build(0.08, 0.08, 0.08)?;
            g.translate(Vector3::new(0., 0.04, 0.))?;
            Ok(g)
        };
        let line = || line_geometry([0., 0., 0., 1., 0., 0.]);
        let line2 = || -> Result<BufferGeometry> {
            let mut g = cylinder(0.0075, 0.0075, 0.5, 3)?;
            g.translate(Vector3::new(0., 0.25, 0.))?;
            Ok(g)
        };
        let boxg = |w: f64, h: f64, d: f64| BoxGeometry::build(w, h, d);
        let picker_cone = || cylinder(0.2, 0., 0.6, 4);
        use Look::*;
        use Shape::{Line as L, Mesh as M};
        let (h, q) = (PI / 2., PI);
        let gizmo_translate: Vec<(&'static str, Vec<Spec>)> = vec![
            (
                "X",
                vec![
                    spec(M(arrow()?), Red, Some([0.5, 0., 0.]), Some([0., 0., -h])),
                    spec(M(arrow()?), Red, Some([-0.5, 0., 0.]), Some([0., 0., h])),
                    spec(M(line2()?), Red, Some([0., 0., 0.]), Some([0., 0., -h])),
                ],
            ),
            (
                "Y",
                vec![
                    spec(M(arrow()?), Green, Some([0., 0.5, 0.]), None),
                    spec(M(arrow()?), Green, Some([0., -0.5, 0.]), Some([q, 0., 0.])),
                    spec(M(line2()?), Green, None, None),
                ],
            ),
            (
                "Z",
                vec![
                    spec(M(arrow()?), Blue, Some([0., 0., 0.5]), Some([h, 0., 0.])),
                    spec(M(arrow()?), Blue, Some([0., 0., -0.5]), Some([-h, 0., 0.])),
                    spec(M(line2()?), Blue, None, Some([h, 0., 0.])),
                ],
            ),
            (
                "XYZ",
                vec![spec(
                    M(OctahedronGeometry::build(0.1, 0)?),
                    WhiteT,
                    Some([0., 0., 0.]),
                    None,
                )],
            ),
            (
                "XY",
                vec![spec(
                    M(boxg(0.15, 0.15, 0.01)?),
                    BlueT,
                    Some([0.15, 0.15, 0.]),
                    None,
                )],
            ),
            (
                "YZ",
                vec![spec(
                    M(boxg(0.15, 0.15, 0.01)?),
                    RedT,
                    Some([0., 0.15, 0.15]),
                    Some([0., h, 0.]),
                )],
            ),
            (
                "XZ",
                vec![spec(
                    M(boxg(0.15, 0.15, 0.01)?),
                    GreenT,
                    Some([0.15, 0., 0.15]),
                    Some([-h, 0., 0.]),
                )],
            ),
        ];
        let picker_translate: Vec<(&'static str, Vec<Spec>)> = vec![
            (
                "X",
                vec![
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([0.3, 0., 0.]),
                        Some([0., 0., -h]),
                    ),
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([-0.3, 0., 0.]),
                        Some([0., 0., h]),
                    ),
                ],
            ),
            (
                "Y",
                vec![
                    spec(M(picker_cone()?), Invisible, Some([0., 0.3, 0.]), None),
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([0., -0.3, 0.]),
                        Some([0., 0., q]),
                    ),
                ],
            ),
            (
                "Z",
                vec![
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([0., 0., 0.3]),
                        Some([h, 0., 0.]),
                    ),
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([0., 0., -0.3]),
                        Some([-h, 0., 0.]),
                    ),
                ],
            ),
            (
                "XYZ",
                vec![spec(
                    M(OctahedronGeometry::build(0.2, 0)?),
                    Invisible,
                    None,
                    None,
                )],
            ),
            (
                "XY",
                vec![spec(
                    M(boxg(0.2, 0.2, 0.01)?),
                    Invisible,
                    Some([0.15, 0.15, 0.]),
                    None,
                )],
            ),
            (
                "YZ",
                vec![spec(
                    M(boxg(0.2, 0.2, 0.01)?),
                    Invisible,
                    Some([0., 0.15, 0.15]),
                    Some([0., h, 0.]),
                )],
            ),
            (
                "XZ",
                vec![spec(
                    M(boxg(0.2, 0.2, 0.01)?),
                    Invisible,
                    Some([0.15, 0., 0.15]),
                    Some([-h, 0., 0.]),
                )],
            ),
        ];
        let octahedron = || OctahedronGeometry::build(0.01, 2);
        let helper_translate: Vec<(&'static str, Vec<Spec>)> = vec![
            ("START", vec![helper(M(octahedron()?), None, None, None)]),
            ("END", vec![helper(M(octahedron()?), None, None, None)]),
            (
                "DELTA",
                vec![helper(
                    L(line_geometry([0., 0., 0., 1., 1., 1.])?),
                    None,
                    None,
                    None,
                )],
            ),
            (
                "X",
                vec![helper(
                    L(line()?),
                    Some([-1e3, 0., 0.]),
                    None,
                    Some([1e6, 1., 1.]),
                )],
            ),
            (
                "Y",
                vec![helper(
                    L(line()?),
                    Some([0., -1e3, 0.]),
                    Some([0., 0., h]),
                    Some([1e6, 1., 1.]),
                )],
            ),
            (
                "Z",
                vec![helper(
                    L(line()?),
                    Some([0., 0., -1e3]),
                    Some([0., -h, 0.]),
                    Some([1e6, 1., 1.]),
                )],
            ),
        ];
        let gizmo_rotate: Vec<(&'static str, Vec<Spec>)> = vec![
            (
                "XYZE",
                vec![spec(M(circle(0.5, 1.)?), Gray, None, Some([0., h, 0.]))],
            ),
            ("X", vec![spec(M(circle(0.5, 0.5)?), Red, None, None)]),
            (
                "Y",
                vec![spec(M(circle(0.5, 0.5)?), Green, None, Some([0., 0., -h]))],
            ),
            (
                "Z",
                vec![spec(M(circle(0.5, 0.5)?), Blue, None, Some([0., h, 0.]))],
            ),
            (
                "E",
                vec![spec(M(circle(0.75, 1.)?), YellowT, None, Some([0., h, 0.]))],
            ),
        ];
        let helper_rotate: Vec<(&'static str, Vec<Spec>)> = vec![(
            "AXIS",
            vec![helper(
                L(line()?),
                Some([-1e3, 0., 0.]),
                None,
                Some([1e6, 1., 1.]),
            )],
        )];
        let picker_rotate: Vec<(&'static str, Vec<Spec>)> = vec![
            (
                "XYZE",
                vec![spec(
                    M(SphereGeometry::build(0.25, 10, 8)?),
                    Invisible,
                    None,
                    None,
                )],
            ),
            (
                "X",
                vec![spec(
                    M(torus(0.5, 0.1, 4, 24, 2. * PI)?),
                    Invisible,
                    Some([0., 0., 0.]),
                    Some([0., -h, -h]),
                )],
            ),
            (
                "Y",
                vec![spec(
                    M(torus(0.5, 0.1, 4, 24, 2. * PI)?),
                    Invisible,
                    Some([0., 0., 0.]),
                    Some([h, 0., 0.]),
                )],
            ),
            (
                "Z",
                vec![spec(
                    M(torus(0.5, 0.1, 4, 24, 2. * PI)?),
                    Invisible,
                    Some([0., 0., 0.]),
                    Some([0., 0., -h]),
                )],
            ),
            (
                "E",
                vec![spec(
                    M(torus(0.75, 0.1, 2, 24, 2. * PI)?),
                    Invisible,
                    None,
                    None,
                )],
            ),
        ];
        let gizmo_scale: Vec<(&'static str, Vec<Spec>)> = vec![
            (
                "X",
                vec![
                    spec(
                        M(scale_handle()?),
                        Red,
                        Some([0.5, 0., 0.]),
                        Some([0., 0., -h]),
                    ),
                    spec(M(line2()?), Red, Some([0., 0., 0.]), Some([0., 0., -h])),
                    spec(
                        M(scale_handle()?),
                        Red,
                        Some([-0.5, 0., 0.]),
                        Some([0., 0., h]),
                    ),
                ],
            ),
            (
                "Y",
                vec![
                    spec(M(scale_handle()?), Green, Some([0., 0.5, 0.]), None),
                    spec(M(line2()?), Green, None, None),
                    spec(
                        M(scale_handle()?),
                        Green,
                        Some([0., -0.5, 0.]),
                        Some([0., 0., q]),
                    ),
                ],
            ),
            (
                "Z",
                vec![
                    spec(
                        M(scale_handle()?),
                        Blue,
                        Some([0., 0., 0.5]),
                        Some([h, 0., 0.]),
                    ),
                    spec(M(line2()?), Blue, Some([0., 0., 0.]), Some([h, 0., 0.])),
                    spec(
                        M(scale_handle()?),
                        Blue,
                        Some([0., 0., -0.5]),
                        Some([-h, 0., 0.]),
                    ),
                ],
            ),
            (
                "XY",
                vec![spec(
                    M(boxg(0.15, 0.15, 0.01)?),
                    BlueT,
                    Some([0.15, 0.15, 0.]),
                    None,
                )],
            ),
            (
                "YZ",
                vec![spec(
                    M(boxg(0.15, 0.15, 0.01)?),
                    RedT,
                    Some([0., 0.15, 0.15]),
                    Some([0., h, 0.]),
                )],
            ),
            (
                "XZ",
                vec![spec(
                    M(boxg(0.15, 0.15, 0.01)?),
                    GreenT,
                    Some([0.15, 0., 0.15]),
                    Some([-h, 0., 0.]),
                )],
            ),
            (
                "XYZ",
                vec![spec(M(boxg(0.1, 0.1, 0.1)?), WhiteT, None, None)],
            ),
        ];
        let picker_scale: Vec<(&'static str, Vec<Spec>)> = vec![
            (
                "X",
                vec![
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([0.3, 0., 0.]),
                        Some([0., 0., -h]),
                    ),
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([-0.3, 0., 0.]),
                        Some([0., 0., h]),
                    ),
                ],
            ),
            (
                "Y",
                vec![
                    spec(M(picker_cone()?), Invisible, Some([0., 0.3, 0.]), None),
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([0., -0.3, 0.]),
                        Some([0., 0., q]),
                    ),
                ],
            ),
            (
                "Z",
                vec![
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([0., 0., 0.3]),
                        Some([h, 0., 0.]),
                    ),
                    spec(
                        M(picker_cone()?),
                        Invisible,
                        Some([0., 0., -0.3]),
                        Some([-h, 0., 0.]),
                    ),
                ],
            ),
            (
                "XY",
                vec![spec(
                    M(boxg(0.2, 0.2, 0.01)?),
                    Invisible,
                    Some([0.15, 0.15, 0.]),
                    None,
                )],
            ),
            (
                "YZ",
                vec![spec(
                    M(boxg(0.2, 0.2, 0.01)?),
                    Invisible,
                    Some([0., 0.15, 0.15]),
                    Some([0., h, 0.]),
                )],
            ),
            (
                "XZ",
                vec![spec(
                    M(boxg(0.2, 0.2, 0.01)?),
                    Invisible,
                    Some([0.15, 0., 0.15]),
                    Some([-h, 0., 0.]),
                )],
            ),
            (
                "XYZ",
                vec![spec(
                    M(boxg(0.2, 0.2, 0.2)?),
                    Invisible,
                    Some([0., 0., 0.]),
                    None,
                )],
            ),
        ];
        let helper_scale: Vec<(&'static str, Vec<Spec>)> = vec![
            (
                "X",
                vec![helper(
                    L(line()?),
                    Some([-1e3, 0., 0.]),
                    None,
                    Some([1e6, 1., 1.]),
                )],
            ),
            (
                "Y",
                vec![helper(
                    L(line()?),
                    Some([0., -1e3, 0.]),
                    Some([0., 0., h]),
                    Some([1e6, 1., 1.]),
                )],
            ),
            (
                "Z",
                vec![helper(
                    L(line()?),
                    Some([0., 0., -1e3]),
                    Some([0., -h, 0.]),
                    Some([1e6, 1., 1.]),
                )],
            ),
        ];
        // One material per role, shared as the addon shares its clones; the
        // highlighted variants are resident too.
        let roles = [
            Invisible, Helper, Red, Green, Blue, RedT, GreenT, BlueT, WhiteT, YellowT, Gray,
        ];
        let materials: Vec<(Look, [Arc<Material>; 4])> = roles
            .iter()
            .map(|&look| {
                let (color, opacity) = look.style();
                (
                    look,
                    [
                        gizmo_material(color, opacity, false),
                        gizmo_material(0xffff00, 1., false),
                        gizmo_material(color, opacity, true),
                        gizmo_material(0xffff00, 1., true),
                    ],
                )
            })
            .collect();
        let material = |look: Look, line: bool| {
            let m = &materials
                .iter()
                .find(|(l, _)| *l == look)
                .expect("gizmo role")
                .1;
            let o = if line { 2 } else { 0 };
            (m[o].clone(), m[o + 1].clone())
        };
        let root = s.insert(NodeKind::Group);
        s.get_mut(root)?.visible = false;
        let gizmo = s.insert(NodeKind::Group);
        s.add(root, gizmo)?;
        // setupGizmo(): clones with the definition transform baked into the geometry.
        let setup = |s: &mut Scene,
                     map: Vec<(&'static str, Vec<Spec>)>,
                     picker: bool|
         -> Result<(Object3D, Vec<Handle>)> {
            let group = s.insert(NodeKind::Group);
            s.add(gizmo, group)?;
            let mut handles = vec![];
            for (name, specs) in map {
                for spec in specs.into_iter().rev() {
                    let position = Vector3::from_array(spec.position.unwrap_or([0.; 3]));
                    let rotation = euler(spec.rotation.unwrap_or([0.; 3]));
                    let scale = Vector3::from_array(spec.scale.unwrap_or([1.; 3]));
                    let matrix =
                        Matrix4::from_scale_rotation_translation(scale, rotation, position);
                    let (kind, base, active) = match spec.shape {
                        Shape::Mesh(mut g) => {
                            g.apply_matrix4(matrix)?;
                            let (base, active) = material(spec.look, false);
                            (
                                NodeKind::Mesh(Mesh::new(Arc::new(g), base.clone())),
                                base,
                                active,
                            )
                        }
                        Shape::Line(mut g) => {
                            g.apply_matrix4(matrix)?;
                            let (base, active) = material(spec.look, true);
                            (
                                NodeKind::Line(Line {
                                    geometry: Arc::new(g),
                                    material: base.clone(),
                                    segments: false,
                                }),
                                base,
                                active,
                            )
                        }
                    };
                    let node = s.insert(kind);
                    let n = s.get_mut(node)?;
                    n.name = name.into();
                    n.render_order = i32::MAX;
                    s.add(group, node)?;
                    handles.push(Handle {
                        node,
                        name,
                        helper: spec.helper,
                        picker,
                        base,
                        active,
                    });
                }
            }
            Ok((group, handles))
        };
        let (gt, gizmo_t) = setup(s, gizmo_translate, false)?;
        let (gr, gizmo_r) = setup(s, gizmo_rotate, false)?;
        let (gs, gizmo_s) = setup(s, gizmo_scale, false)?;
        let (pt, picker_t) = setup(s, picker_translate, true)?;
        let (pr, picker_r) = setup(s, picker_rotate, true)?;
        let (ps, picker_s) = setup(s, picker_scale, true)?;
        let (ht, helper_t) = setup(s, helper_translate, false)?;
        let (hr, helper_r) = setup(s, helper_rotate, false)?;
        let (hs, helper_s) = setup(s, helper_scale, false)?;
        for p in [pt, pr, ps] {
            s.get_mut(p)?.visible = false;
        }
        let join = |a: Vec<Handle>, b: Vec<Handle>, c: Vec<Handle>| {
            a.into_iter().chain(b).chain(c).collect()
        };
        Ok(Self {
            root,
            modes: [
                ModeHandles {
                    groups: [pt, gt, ht],
                    handles: join(picker_t, gizmo_t, helper_t),
                },
                ModeHandles {
                    groups: [pr, gr, hr],
                    handles: join(picker_r, gizmo_r, helper_r),
                },
                ModeHandles {
                    groups: [ps, gs, hs],
                    handles: join(picker_s, gizmo_s, helper_s),
                },
            ],
            camera,
            object: None,
            enabled: true,
            axis: None,
            mode: Mode::Translate,
            translation_snap: None,
            rotation_snap: None,
            scale_snap: None,
            local: false,
            size: 1.,
            dragging: false,
            show: [true; 3],
            render_on_change: false,
            events: vec![],
            world_position: Vector3::ZERO,
            world_position_start: Vector3::ZERO,
            world_quaternion: Quaternion::IDENTITY,
            world_quaternion_start: Quaternion::IDENTITY,
            camera_position: Vector3::ZERO,
            camera_quaternion: Quaternion::IDENTITY,
            point_start: Vector3::ZERO,
            point_end: Vector3::ZERO,
            rotation_axis: Vector3::ZERO,
            rotation_angle: 0.,
            eye: Vector3::ZERO,
            parent_quaternion: Quaternion::IDENTITY,
            parent_scale: Vector3::ONE,
            world_quaternion_inv: Quaternion::IDENTITY,
            parent_quaternion_inv: Quaternion::IDENTITY,
            position_start: Vector3::ZERO,
            quaternion_start: Quaternion::IDENTITY,
            scale_start: Vector3::ONE,
            plane: (Vector3::ZERO, Quaternion::IDENTITY),
            dir: Vector3::ZERO,
        })
    }
    fn mode_index(&self) -> usize {
        match self.mode {
            Mode::Translate => 0,
            Mode::Rotate => 1,
            Mode::Scale => 2,
        }
    }
    /// A defined property changed: '<name>-changed' (when given), then 'change'.
    fn changed(&mut self, s: &mut Scene, event: Option<Event>) -> Result<()> {
        if let Some(e) = event {
            self.events.push(e);
        }
        self.events.push(Event::Change);
        if self.render_on_change {
            self.update(s)?;
        }
        Ok(())
    }
    fn set_axis(&mut self, s: &mut Scene, axis: Option<&'static str>) -> Result<()> {
        if self.axis != axis {
            self.axis = axis;
            self.changed(s, None)?;
        }
        Ok(())
    }
    fn set_dragging(&mut self, s: &mut Scene, dragging: bool) -> Result<()> {
        if self.dragging != dragging {
            self.dragging = dragging;
            self.changed(s, Some(Event::DraggingChanged(dragging)))?;
        }
        Ok(())
    }
    pub(super) fn set_mode(&mut self, s: &mut Scene, mode: Mode) -> Result<()> {
        if self.mode != mode {
            self.mode = mode;
            self.changed(s, None)?;
        }
        Ok(())
    }
    pub(super) fn set_space_local(&mut self, s: &mut Scene, local: bool) -> Result<()> {
        if self.local != local {
            self.local = local;
            self.changed(s, None)?;
        }
        Ok(())
    }
    pub(super) fn set_size(&mut self, s: &mut Scene, size: f64) -> Result<()> {
        if self.size != size {
            self.size = size;
            self.changed(s, None)?;
        }
        Ok(())
    }
    pub(super) fn set_enabled(&mut self, s: &mut Scene, enabled: bool) -> Result<()> {
        if self.enabled != enabled {
            self.enabled = enabled;
            self.changed(s, None)?;
        }
        Ok(())
    }
    pub(super) fn set_show(&mut self, s: &mut Scene, axis: usize, shown: bool) -> Result<()> {
        if self.show[axis] != shown {
            self.show[axis] = shown;
            self.changed(s, None)?;
        }
        Ok(())
    }
    /// setTranslationSnap, setRotationSnap and setScaleSnap together.
    pub(super) fn set_snaps(&mut self, s: &mut Scene, snaps: [Option<f64>; 3]) -> Result<()> {
        for (i, v) in snaps.into_iter().enumerate() {
            let slot = match i {
                0 => &mut self.translation_snap,
                1 => &mut self.rotation_snap,
                _ => &mut self.scale_snap,
            };
            if *slot != v {
                *slot = v;
                self.changed(s, None)?;
            }
        }
        Ok(())
    }
    /// Setting `camera` dispatches 'change' even for the same camera kind swap.
    pub(super) fn camera_changed(&mut self, s: &mut Scene) -> Result<()> {
        self.changed(s, None)
    }
    pub(super) fn attach(&mut self, s: &mut Scene, object: Object3D) -> Result<()> {
        if self.object != Some(object) {
            self.object = Some(object);
            self.changed(s, None)?;
        }
        s.get_mut(self.root)?.visible = true;
        Ok(())
    }
    /// reset(): restore the drag's start transform.
    pub(super) fn reset(&mut self, s: &mut Scene) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        if self.dragging
            && let Some(o) = self.object
        {
            let n = s.get_mut(o)?;
            n.position = self.position_start;
            n.quaternion = self.quaternion_start;
            n.scale = self.scale_start;
            self.changed(s, None)?;
            self.events.push(Event::ObjectChange);
            self.point_start = self.point_end;
        }
        Ok(())
    }
    fn raycaster(&self, s: &Scene, ndc: Vector2) -> Result<Raycaster> {
        let (camera, world) = s.camera(self.camera)?;
        let mut r = Raycaster::default();
        r.set_from_camera(ndc, camera, world)?;
        Ok(r)
    }
    /// The 100000² double-sided drag plane, as last updated.
    fn plane_hit(&self, r: &Raycaster) -> Option<Vector3> {
        let (position, quaternion) = self.plane;
        let normal = quaternion * Vector3::Z;
        let p = r.ray.intersect_plane(Plane {
            normal,
            constant: -normal.dot(position),
        })?;
        let local = quaternion.inverse() * (p - position);
        (local.x.abs() <= 50000. && local.y.abs() <= 50000.).then_some(p)
    }
    pub(super) fn pointer_hover(&mut self, s: &mut Scene, ndc: Vector2) -> Result<()> {
        if self.object.is_none() || self.dragging {
            return Ok(());
        }
        let r = self.raycaster(s, ndc)?;
        let pickers: Vec<Object3D> = self.modes[self.mode_index()]
            .handles
            .iter()
            .filter(|h| h.picker)
            .map(|h| h.node)
            .collect();
        let mut axis = None;
        for hit in r.intersect_objects(s, &pickers, false)? {
            if s.get(hit.object)?.visible {
                let index = self.mode_index();
                axis = self.modes[index]
                    .handles
                    .iter()
                    .find(|h| h.node == hit.object)
                    .map(|h| h.name);
                break;
            }
        }
        self.set_axis(s, axis)
    }
    pub(super) fn pointer_down(&mut self, s: &mut Scene, ndc: Vector2, button: i32) -> Result<()> {
        let Some(object) = self.object else {
            return Ok(());
        };
        if self.dragging || button != 0 {
            return Ok(());
        }
        if self.axis.is_some() {
            let r = self.raycaster(s, ndc)?;
            if let Some(point) = self.plane_hit(&r) {
                s.update_world_matrix(object, false, true)?;
                if let Some(p) = s.get(object)?.parent() {
                    s.update_world_matrix(p, false, true)?;
                }
                let n = s.get(object)?;
                self.position_start = n.position;
                self.quaternion_start = n.quaternion;
                self.scale_start = n.scale;
                let (position, quaternion, _) = decompose(n.matrix_world);
                self.world_position_start = position;
                self.world_quaternion_start = quaternion;
                self.point_start = point - position;
            }
            self.set_dragging(s, true)?;
            self.events.push(Event::MouseDown);
        }
        Ok(())
    }
    pub(super) fn pointer_move(&mut self, s: &mut Scene, ndc: Vector2) -> Result<()> {
        let (Some(object), Some(axis)) = (self.object, self.axis) else {
            return Ok(());
        };
        let mode = self.mode;
        let local = if mode == Mode::Scale {
            true
        } else if matches!(axis, "E" | "XYZE" | "XYZ") {
            false
        } else {
            self.local
        };
        if !self.dragging {
            return Ok(());
        }
        let r = self.raycaster(s, ndc)?;
        let Some(point) = self.plane_hit(&r) else {
            return Ok(());
        };
        self.point_end = point - self.world_position_start;
        let has = |c: char| axis.contains(c);
        match mode {
            Mode::Translate => {
                let mut offset = self.point_end - self.point_start;
                if local && axis != "XYZ" {
                    offset = self.world_quaternion_inv * offset;
                }
                if !has('X') {
                    offset.x = 0.;
                }
                if !has('Y') {
                    offset.y = 0.;
                }
                if !has('Z') {
                    offset.z = 0.;
                }
                offset = if local && axis != "XYZ" {
                    (self.quaternion_start * offset) / self.parent_scale
                } else {
                    (self.parent_quaternion_inv * offset) / self.parent_scale
                };
                let mut position = offset + self.position_start;
                if let Some(step) = self.translation_snap {
                    if local {
                        position = self.quaternion_start.inverse() * position;
                        for (k, c) in ['X', 'Y', 'Z'].into_iter().enumerate() {
                            if has(c) {
                                position[k] = snap(position[k], step);
                            }
                        }
                        position = self.quaternion_start * position;
                    } else {
                        s.get_mut(object)?.position = position;
                        s.update_world_matrix(object, true, false)?;
                        let mut world = s.get(object)?.world_position();
                        for (k, c) in ['X', 'Y', 'Z'].into_iter().enumerate() {
                            if has(c) {
                                world[k] = snap(world[k], step);
                            }
                        }
                        position = match s.get(object)?.parent() {
                            Some(p) => {
                                s.update_world_matrix(p, true, false)?;
                                s.get(p)?.world_to_local(world)
                            }
                            None => world,
                        };
                    }
                }
                s.get_mut(object)?.position = position;
            }
            Mode::Scale => {
                let factor = if axis.contains("XYZ") {
                    let mut d = self.point_end.length() / self.point_start.length();
                    if self.point_end.dot(self.point_start) < 0. {
                        d = -d;
                    }
                    Vector3::splat(d)
                } else {
                    let a = self.world_quaternion_inv * self.point_start;
                    let mut b = (self.world_quaternion_inv * self.point_end) / a;
                    for (k, c) in ['X', 'Y', 'Z'].into_iter().enumerate() {
                        if !has(c) {
                            b[k] = 1.;
                        }
                    }
                    b
                };
                let mut scale = self.scale_start * factor;
                if let Some(step) = self.scale_snap {
                    for (k, c) in ['X', 'Y', 'Z'].into_iter().enumerate() {
                        if has(c) {
                            let v = snap(scale[k], step);
                            scale[k] = if v == 0. || v.is_nan() { step } else { v };
                        }
                    }
                }
                s.get_mut(object)?.scale = scale;
            }
            Mode::Rotate => {
                let offset = self.point_end - self.point_start;
                s.update_world_matrix(self.camera, false, false)?;
                let camera_world = s.get(self.camera)?.matrix_world.w_axis.truncate();
                let speed = 20. / self.world_position.distance(camera_world);
                let mut in_plane = false;
                if axis == "XYZE" {
                    self.rotation_axis = norm(offset.cross(self.eye));
                    self.rotation_angle = offset.dot(self.rotation_axis.cross(self.eye)) * speed;
                } else if matches!(axis, "X" | "Y" | "Z") {
                    let unit = match axis {
                        "X" => Vector3::X,
                        "Y" => Vector3::Y,
                        _ => Vector3::Z,
                    };
                    self.rotation_axis = unit;
                    let mut t = unit;
                    if local {
                        t = self.world_quaternion * t;
                    }
                    t = t.cross(self.eye);
                    if t.length() == 0. {
                        in_plane = true;
                    } else {
                        self.rotation_angle = offset.dot(norm(t)) * speed;
                    }
                }
                if axis == "E" || in_plane {
                    self.rotation_axis = self.eye;
                    self.rotation_angle = angle_to(self.point_end, self.point_start);
                    let start = norm(self.point_start);
                    let end = norm(self.point_end);
                    self.rotation_angle *= if end.cross(start).dot(self.eye) < 0. {
                        1.
                    } else {
                        -1.
                    };
                }
                if let Some(step) = self.rotation_snap {
                    self.rotation_angle = snap(self.rotation_angle, step);
                }
                let q = if local && axis != "E" && axis != "XYZE" {
                    (self.quaternion_start * axis_angle(self.rotation_axis, self.rotation_angle))
                        .normalize()
                } else {
                    self.rotation_axis = self.parent_quaternion_inv * self.rotation_axis;
                    (axis_angle(self.rotation_axis, self.rotation_angle) * self.quaternion_start)
                        .normalize()
                };
                s.get_mut(object)?.quaternion = q;
            }
        }
        self.changed(s, None)?;
        self.events.push(Event::ObjectChange);
        Ok(())
    }
    pub(super) fn pointer_up(&mut self, s: &mut Scene, button: i32) -> Result<()> {
        if button != 0 {
            return Ok(());
        }
        if self.dragging && self.axis.is_some() {
            self.events.push(Event::MouseUp);
        }
        self.set_dragging(s, false)?;
        self.set_axis(s, None)
    }
    /// The root's updateMatrixWorld: the attached object's and camera's
    /// decompositions, then the gizmo handles and the drag plane.
    pub(super) fn update(&mut self, s: &mut Scene) -> Result<()> {
        if let Some(o) = self.object {
            s.update_world_matrix(o, false, true)?;
            let parent = match s.get(o)?.parent() {
                Some(p) => s.get(p)?.matrix_world,
                None => Matrix4::IDENTITY,
            };
            let (_, pq, ps) = decompose(parent);
            (self.parent_quaternion, self.parent_scale) = (pq, ps);
            let (wp, wq, _) = decompose(s.get(o)?.matrix_world);
            (self.world_position, self.world_quaternion) = (wp, wq);
            self.parent_quaternion_inv = self.parent_quaternion.inverse();
            self.world_quaternion_inv = self.world_quaternion.inverse();
        }
        s.update_world_matrix(self.camera, false, true)?;
        let camera_world = s.get(self.camera)?.matrix_world;
        let (cp, cq, _) = decompose(camera_world);
        (self.camera_position, self.camera_quaternion) = (cp, cq);
        let (camera, _) = s.camera(self.camera)?;
        let camera = camera.clone();
        self.eye = match camera {
            Camera::Orthographic(_) => norm(camera_world.z_axis.truncate()),
            Camera::Perspective(_) => norm(self.camera_position - self.world_position),
        };
        self.update_gizmo(s, &camera)?;
        self.update_plane();
        s.update_world_matrix(self.root, false, true)
    }
    fn update_gizmo(&mut self, s: &mut Scene, camera: &Camera) -> Result<()> {
        let local = self.mode == Mode::Scale || self.local;
        let quaternion = if local {
            self.world_quaternion
        } else {
            Quaternion::IDENTITY
        };
        let current = self.mode_index();
        for (i, m) in self.modes.iter().enumerate() {
            s.get_mut(m.groups[1])?.visible = i == current;
            s.get_mut(m.groups[2])?.visible = i == current;
        }
        let factor = match camera {
            Camera::Orthographic(o) => (o.top - o.bottom) / o.zoom,
            Camera::Perspective(p) => {
                self.world_position.distance(self.camera_position)
                    * (1.9 * (PI * p.fov / 360.).tan() / p.zoom).min(7.)
            }
        };
        let tiny = Vector3::splat(1e-10);
        let eye = self.eye;
        let aligned = |unit: Vector3| (quaternion * unit).dot(eye).abs();
        for handle in &self.modes[current].handles {
            let mut visible = true;
            let mut rotation = Quaternion::IDENTITY;
            let mut position = self.world_position;
            let mut scale = Vector3::splat(factor * self.size / 4.);
            if handle.helper {
                visible = false;
                match handle.name {
                    "AXIS" => {
                        visible = self.axis.is_some();
                        match self.axis {
                            Some("X") => {
                                rotation = quaternion;
                                if aligned(Vector3::X) > 0.9 {
                                    visible = false;
                                }
                            }
                            Some("Y") => {
                                rotation = quaternion * euler([0., 0., PI / 2.]);
                                if aligned(Vector3::Y) > 0.9 {
                                    visible = false;
                                }
                            }
                            Some("Z") => {
                                rotation = quaternion * euler([0., PI / 2., 0.]);
                                if aligned(Vector3::Z) > 0.9 {
                                    visible = false;
                                }
                            }
                            Some("XYZE") => {
                                rotation = look_at(Vector3::ZERO, self.rotation_axis, Vector3::Y)
                                    * euler([0., PI / 2., 0.]);
                                visible = self.dragging;
                            }
                            Some("E") => visible = false,
                            _ => {}
                        }
                    }
                    "START" => {
                        position = self.world_position_start;
                        visible = self.dragging;
                    }
                    "END" => visible = self.dragging,
                    "DELTA" => {
                        position = self.world_position_start;
                        rotation = self.world_quaternion_start;
                        let t = (tiny + self.world_position_start - self.world_position) * -1.;
                        scale = self.world_quaternion_start.inverse() * t;
                        visible = self.dragging;
                    }
                    name => {
                        rotation = quaternion;
                        if self.dragging {
                            position = self.world_position_start;
                        }
                        if let Some(axis) = self.axis {
                            visible = axis.contains(name);
                        }
                    }
                }
                let n = s.get_mut(handle.node)?;
                (n.visible, n.position, n.quaternion, n.scale) =
                    (visible, position, rotation, scale);
                continue;
            }
            rotation = quaternion;
            let name = handle.name;
            match self.mode {
                Mode::Translate | Mode::Scale => {
                    let hide = match name {
                        "X" => aligned(Vector3::X) > 0.99,
                        "Y" => aligned(Vector3::Y) > 0.99,
                        "Z" => aligned(Vector3::Z) > 0.99,
                        "XY" => aligned(Vector3::Z) < 0.2,
                        "YZ" => aligned(Vector3::X) < 0.2,
                        "XZ" => aligned(Vector3::Y) < 0.2,
                        _ => false,
                    };
                    if hide {
                        scale = tiny;
                        visible = false;
                    }
                }
                Mode::Rotate => {
                    let align = quaternion.inverse() * eye;
                    if name.contains('E') {
                        rotation = look_at(eye, Vector3::ZERO, Vector3::Y);
                    }
                    match name {
                        "X" => {
                            rotation =
                                quaternion * axis_angle(Vector3::X, (-align.y).atan2(align.z))
                        }
                        "Y" => {
                            rotation = quaternion * axis_angle(Vector3::Y, align.x.atan2(align.z))
                        }
                        "Z" => {
                            rotation = quaternion * axis_angle(Vector3::Z, align.y.atan2(align.x))
                        }
                        _ => {}
                    }
                }
            }
            let [sx, sy, sz] = self.show;
            visible = visible && (!name.contains('X') || sx);
            visible = visible && (!name.contains('Y') || sy);
            visible = visible && (!name.contains('Z') || sz);
            visible = visible && (!name.contains('E') || (sx && sy && sz));
            let active = self.enabled
                && self.axis.is_some_and(|axis| {
                    name == axis || axis.chars().any(|c| name.len() == 1 && name.starts_with(c))
                });
            let n = s.get_mut(handle.node)?;
            (n.visible, n.position, n.quaternion, n.scale) = (visible, position, rotation, scale);
            if !handle.picker {
                let chosen = if active { &handle.active } else { &handle.base };
                match &mut n.kind {
                    NodeKind::Mesh(m) if !Arc::ptr_eq(&m.materials[0], chosen) => {
                        m.materials[0] = chosen.clone()
                    }
                    NodeKind::Line(l) if !Arc::ptr_eq(&l.material, chosen) => {
                        l.material = chosen.clone()
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    /// TransformControlsPlane.updateMatrixWorld.
    fn update_plane(&mut self) {
        let local = self.mode == Mode::Scale || self.local;
        let q = if local {
            self.world_quaternion
        } else {
            Quaternion::IDENTITY
        };
        let (v1, v2, v3) = (q * Vector3::X, q * Vector3::Y, q * Vector3::Z);
        let mut align = v2;
        match self.mode {
            Mode::Translate | Mode::Scale => match self.axis {
                Some("X") => {
                    align = self.eye.cross(v1);
                    self.dir = v1.cross(align);
                }
                Some("Y") => {
                    align = self.eye.cross(v2);
                    self.dir = v2.cross(align);
                }
                Some("Z") => {
                    align = self.eye.cross(v3);
                    self.dir = v3.cross(align);
                }
                Some("XY") => self.dir = v3,
                Some("YZ") => self.dir = v1,
                Some("XZ") => {
                    align = v3;
                    self.dir = v2;
                }
                Some("XYZ") | Some("E") => self.dir = Vector3::ZERO,
                _ => {}
            },
            Mode::Rotate => self.dir = Vector3::ZERO,
        }
        let quaternion = if self.dir.length() == 0. {
            self.camera_quaternion
        } else {
            look_at(Vector3::ZERO, self.dir, align)
        };
        self.plane = (self.world_position, quaternion);
    }
}
