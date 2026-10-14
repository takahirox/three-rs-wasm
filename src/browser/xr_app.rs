//! webgpu_xr_cubes, webgpu_xr_rollercoaster and webgpu_xr_shadows: three
//! r186's WebXR examples on the WebGPU renderer, without their WebGL
//! fallback. The page's JavaScript ( `web/gallery/xr.js` ) owns the XR
//! session, the XRButton / VRButton and the XRGPUBinding projection layer;
//! this app owns the scene, the XRManager state ( `crate::xr` ) and the
//! rendering, on the canvas before the session and into the layer's texture
//! during it.
use super::roller_coaster::{self, Curve};
use super::xr_controller_model::{ControllerModel, ControllerModelFactory};
use super::xr_random::random;
use crate::{
    Error, Result,
    camera::{Camera, PerspectiveCamera},
    geometry::*,
    material::*,
    math::*,
    raycast::Raycaster,
    render_target::{RenderTarget, RenderTargetOptions},
    renderer::Renderer,
    scene::*,
    shadow::ShadowFilter,
    xr::*,
};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

/// The gallery's ids of the three examples.
pub const CUBES: u32 = 498;
pub const ROLLERCOASTER: u32 = 499;
pub const SHADOWS: u32 = 500;

fn now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.)
}

/// THREE.Timer, with performance.now.
struct Timer {
    start: f64,
    previous: f64,
    current: f64,
    delta: f64,
    elapsed: f64,
}
impl Timer {
    fn new() -> Self {
        Timer {
            start: now(),
            previous: 0.,
            current: 0.,
            delta: 0.,
            elapsed: 0.,
        }
    }
    /// update(): a hidden document keeps the delta at zero ( connect( document ) ).
    fn update(&mut self) {
        let hidden = web_sys::window()
            .and_then(|w| w.document())
            .is_some_and(|d| d.hidden());
        if hidden {
            self.delta = 0.;
        } else {
            self.previous = self.current;
            self.current = now() - self.start;
            self.delta = self.current - self.previous;
            self.elapsed += self.delta;
        }
    }
    fn delta(&self) -> f64 {
        self.delta / 1000.
    }
    fn elapsed(&self) -> f64 {
        self.elapsed / 1000.
    }
}

fn mesh(s: &mut Scene, geometry: BufferGeometry, material: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(geometry),
        Arc::new(material),
    )))
}

fn props(color: u32) -> MaterialProperties {
    MaterialProperties {
        color: Color::from_hex(color),
        ..Default::default()
    }
}

fn perspective(fov: f64, aspect: f64, near: f64, far: f64) -> NodeKind {
    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov,
        aspect,
        near,
        far,
        ..Default::default()
    }))
}

fn hemisphere(s: &mut Scene, sky: u32, ground: u32, intensity: f64) -> Object3D {
    let h = s.insert(NodeKind::Light(Light::Hemisphere {
        sky: Color::from_hex(sky),
        ground: Color::from_hex(ground),
        intensity,
    }));
    s.get_mut(h).expect("light").position = Vector3::Y;
    h
}

/// Euler XYZ rotation.
fn euler(r: Vector3) -> Quaternion {
    Euler {
        angles: r,
        order: EulerOrder::XYZ,
    }
    .quaternion()
}

/// BoxLineGeometry( width, height, depth, widthSegments, heightSegments, depthSegments ).
fn box_line_geometry(w: f64, h: f64, d: f64, ws: u32, hs: u32, ds: u32) -> Result<BufferGeometry> {
    let (ws, hs, ds) = (ws as i32, hs as i32, ds as i32);
    let width_half = w / 2.;
    let height_half = h / 2.;
    let depth_half = d / 2.;
    let segment_width = w / ws as f64;
    let segment_height = h / hs as f64;
    let segment_depth = d / ds as f64;
    let mut v: Vec<f64> = Vec::new();
    let mut x = -width_half;
    let mut y = -height_half;
    let mut z = -depth_half;
    for _ in 0..=ws {
        v.extend([x, -height_half, -depth_half, x, height_half, -depth_half]);
        v.extend([x, height_half, -depth_half, x, height_half, depth_half]);
        v.extend([x, height_half, depth_half, x, -height_half, depth_half]);
        v.extend([x, -height_half, depth_half, x, -height_half, -depth_half]);
        x += segment_width;
    }
    for _ in 0..=hs {
        v.extend([-width_half, y, -depth_half, width_half, y, -depth_half]);
        v.extend([width_half, y, -depth_half, width_half, y, depth_half]);
        v.extend([width_half, y, depth_half, -width_half, y, depth_half]);
        v.extend([-width_half, y, depth_half, -width_half, y, -depth_half]);
        y += segment_height;
    }
    for _ in 0..=ds {
        v.extend([-width_half, -height_half, z, -width_half, height_half, z]);
        v.extend([-width_half, height_half, z, width_half, height_half, z]);
        v.extend([width_half, height_half, z, width_half, -height_half, z]);
        v.extend([width_half, -height_half, z, -width_half, -height_half, z]);
        z += segment_depth;
    }
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(crate::attribute::BufferAttribute::new(
            v.into_iter().map(|x| x as f32).collect(),
            3,
            false,
        )?),
    );
    Ok(g)
}

/// The cubes example's floating box.
struct Cube {
    node: Object3D,
    rotation: Vector3,
    velocity: Vector3,
}

/// webgpu_xr_cubes.
struct Cubes {
    room: Object3D,
    cubes: Vec<Cube>,
    /// room.children's order ( selection moves the first cube to the end ).
    order: Vec<usize>,
    intersected: Option<(usize, Color)>,
    timer: Timer,
    line: Option<Object3D>,
}

impl Cubes {
    fn new(s: &mut Scene, camera: Object3D, xr: &mut XrManager) -> Result<Self> {
        s.background = Color::from_hex(0x505050);
        {
            let n = s.get_mut(camera)?;
            n.kind = perspective(50., 1., 0.1, 10.);
            n.position = Vector3::new(0., 1.6, 3.);
        }
        let mut room_geometry = box_line_geometry(6., 6., 6., 10, 10, 10)?;
        room_geometry.translate(Vector3::new(0., 3., 0.))?;
        let room = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(room_geometry),
            material: Arc::new(Material::Line(LineBasicMaterial {
                properties: props(0xbcbcbc),
                ..Default::default()
            })),
            segments: true,
        }));
        hemisphere(s, 0xa5a5a5, 0x898989, 3.);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0xffffff),
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::ONE.normalize();
        let geometry = Arc::new(BoxGeometry::build(0.15, 0.15, 0.15)?);
        let mut cubes = Vec::new();
        for _ in 0..200 {
            let color = Color::from_hex((random() * 16777215.).floor() as u32);
            let node = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Lambert(MeshLambertMaterial {
                    properties: MaterialProperties {
                        color,
                        ..Default::default()
                    },
                    ..Default::default()
                })),
            )));
            let position = Vector3::new(random() * 4. - 2., random() * 4., random() * 4. - 2.);
            let rotation = Vector3::new(random() * 2. * PI, random() * 2. * PI, random() * 2. * PI);
            let scale = Vector3::new(random() + 0.5, random() + 0.5, random() + 0.5);
            let velocity = Vector3::new(
                random() * 0.01 - 0.005,
                random() * 0.01 - 0.005,
                random() * 0.01 - 0.005,
            );
            let n = s.get_mut(node)?;
            n.position = position;
            n.quaternion = euler(rotation);
            n.scale = scale;
            s.add(room, node)?;
            cubes.push(Cube {
                node,
                rotation,
                velocity,
            });
        }
        let order = (0..cubes.len()).collect();
        // setupControllers: the target ray ( with a line on `connected` ) and
        // the grip with its controller model.
        xr.controller(s, 0);
        Ok(Cubes {
            room,
            cubes,
            order,
            intersected: None,
            timer: Timer::new(),
            line: None,
        })
    }
    /// buildController( data ) for `connected`.
    fn connected(&mut self, s: &mut Scene, xr: &XrManager, source: &XrInputSource) -> Result<()> {
        let controller = xr.controllers[0].target_ray;
        let node = match source.target_ray_mode.as_str() {
            "tracked-pointer" => {
                let mut g = BufferGeometry::default();
                g.set_attribute(
                    "position",
                    Attribute::F32(crate::attribute::BufferAttribute::new(
                        vec![0., 0., 0., 0., 0., -1.],
                        3,
                        false,
                    )?),
                );
                g.set_attribute(
                    "color",
                    Attribute::F32(crate::attribute::BufferAttribute::new(
                        vec![0.5, 0.5, 0.5, 0., 0., 0.],
                        3,
                        false,
                    )?),
                );
                let mut m = LineBasicMaterial::default();
                m.properties.vertex_colors = true;
                m.properties.blending = Some(additive_blending());
                s.insert(NodeKind::Line(Line {
                    geometry: Arc::new(g),
                    material: Arc::new(Material::Line(m)),
                    segments: false,
                }))
            }
            "gaze" => {
                let mut g = RingGeometry::build(0.02, 0.04, 32, 1, 0., PI * 2.)?;
                g.translate(Vector3::new(0., 0., -1.))?;
                let mut m = MeshBasicMaterial::default();
                m.properties.opacity = 0.5;
                m.properties.transparent = true;
                mesh(s, g, Material::Basic(m))
            }
            _ => return Ok(()),
        };
        s.add(controller, node)?;
        self.line = Some(node);
        Ok(())
    }
    /// `disconnected`: this.remove( this.children[ 0 ] ).
    fn disconnected(&mut self, s: &mut Scene, xr: &XrManager) -> Result<()> {
        let controller = xr.controllers[0].target_ray;
        if let Some(&first) = s.get(controller)?.children().first() {
            s.remove(controller, first)?;
        }
        self.line = None;
        Ok(())
    }
    fn set_emissive(s: &mut Scene, node: Object3D, color: Color) -> Result<()> {
        if let NodeKind::Mesh(m) = &mut s.get_mut(node)?.kind
            && let Material::Lambert(l) = Arc::make_mut(&mut m.materials[0])
        {
            l.emissive = color;
        }
        Ok(())
    }
    fn emissive(s: &Scene, node: Object3D) -> Result<Color> {
        if let NodeKind::Mesh(m) = &s.get(node)?.kind
            && let Material::Lambert(l) = m.materials[0].as_ref()
        {
            return Ok(l.emissive);
        }
        Err(Error::Invalid("cube material"))
    }
    fn animate(&mut self, s: &mut Scene, xr: &XrManager) -> Result<()> {
        self.timer.update();
        let delta = self.timer.delta() * 60.;
        let controller = &xr.controllers[0];
        s.update()?;
        if controller.is_selecting {
            let first = self.order.remove(0);
            let cube = &mut self.cubes[first];
            s.remove(self.room, cube.node)?;
            let (position, quaternion) = {
                let n = s.get(controller.target_ray)?;
                (n.position, n.quaternion)
            };
            s.get_mut(cube.node)?.position = position;
            cube.velocity.x = (random() - 0.5) * 0.02 * delta;
            cube.velocity.y = (random() - 0.5) * 0.02 * delta;
            cube.velocity.z = (random() * 0.01 - 0.05) * delta;
            cube.velocity = quaternion * cube.velocity;
            s.add(self.room, cube.node)?;
            self.order.push(first);
        }
        // raycaster.setFromXRController( controller ); intersectObjects( room.children ).
        s.update()?;
        let (origin, direction) = controller_ray(s, controller.target_ray)?;
        let mut raycaster = Raycaster::default();
        raycaster.set(origin, direction);
        let children: Vec<Object3D> = self.order.iter().map(|&i| self.cubes[i].node).collect();
        let hits = raycaster.intersect_objects(s, &children, false)?;
        if let Some(hit) = hits.first() {
            let index = self.order[children.iter().position(|&c| c == hit.object).unwrap_or(0)];
            if self.intersected.map(|(i, _)| i) != Some(index) {
                if let Some((i, color)) = self.intersected {
                    Self::set_emissive(s, self.cubes[i].node, color)?;
                }
                let node = self.cubes[index].node;
                self.intersected = Some((index, Self::emissive(s, node)?));
                Self::set_emissive(s, node, Color::from_hex(0xff0000))?;
            }
        } else {
            if let Some((i, color)) = self.intersected {
                Self::set_emissive(s, self.cubes[i].node, color)?;
            }
            self.intersected = None;
        }
        // Keep cubes inside room.
        for &i in &self.order {
            let cube = &mut self.cubes[i];
            cube.velocity *= 1. - 0.001 * delta;
            let n = s.get_mut(cube.node)?;
            n.position += cube.velocity;
            for axis in 0..3 {
                let (lo, hi) = if axis == 1 { (0., 6.) } else { (-3., 3.) };
                if n.position[axis] < lo || n.position[axis] > hi {
                    n.position[axis] = n.position[axis].clamp(lo, hi);
                    cube.velocity[axis] = -cube.velocity[axis];
                }
            }
            cube.rotation += cube.velocity * 2. * delta;
            n.quaternion = euler(cube.rotation);
        }
        Ok(())
    }
}

/// AdditiveBlending without premultiplied alpha, as r186's WebGPU pipelines
/// set it: color ( srcAlpha, one ), alpha ( one, one ).
fn additive_blending() -> wgpu::BlendState {
    wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
    }
}

/// The rollercoaster's closed curve.
struct Track;
impl Curve for Track {
    fn point_at(&self, t: f64) -> Vector3 {
        let t = t * PI * 2.;
        let x = (t * 3.).sin() * (t * 4.).cos() * 50.;
        let y = (t * 10.).sin() * 2. + (t * 17.).cos() * 2. + 5.;
        let z = t.sin() * (t * 4.).sin() * 50.;
        Vector3::new(x, y, z) * 2.
    }
    fn tangent_at(&self, t: f64) -> Vector3 {
        let delta = 0.0001;
        let t1 = (t - delta).max(0.);
        let t2 = (t + delta).min(1.);
        (self.point_at(t2) - self.point_at(t1)).normalize_or_zero()
    }
}

/// Matrix4.lookAt( eye, target, up )'s rotation.
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
    let y = z.cross(x);
    Quaternion::from_mat3(&Matrix3::from_cols(x, y, z))
}

/// webgpu_xr_rollercoaster.
struct RollerCoaster {
    train: Object3D,
    funfairs: [Object3D; 2],
    velocity: f64,
    progress: f64,
    prev_time: f64,
}

impl RollerCoaster {
    fn new(s: &mut Scene, camera: Object3D, xr: &mut XrManager) -> Result<Self> {
        xr.reference_space_type = "local".into();
        s.background = Color::from_hex(0xf0f0ff);
        let light = hemisphere(s, 0xfff0f0, 0x060606, 3.);
        s.get_mut(light)?.position = Vector3::ONE;
        let train = s.insert(NodeKind::Group);
        s.get_mut(camera)?.kind = perspective(50., 1., 0.1, 500.);
        s.add(train, camera)?;
        // environment
        let mut g = PlaneGeometry::build(500., 500., 15, 15)?;
        g.rotate_x(-PI / 2.)?;
        if let Some(Attribute::F32(a)) = g.attributes.get_mut("position") {
            let positions = a.array_mut();
            for i in (0..positions.len()).step_by(3) {
                let mut v = Vector3::new(
                    positions[i] as f64,
                    positions[i + 1] as f64,
                    positions[i + 2] as f64,
                );
                v.x += random() * 10. - 5.;
                v.z += random() * 10. - 5.;
                let distance = v.length() / 5. - 25.;
                v.y = random() * distance.max(0.);
                positions[i] = v.x as f32;
                positions[i + 1] = v.y as f32;
                positions[i + 2] = v.z as f32;
            }
        }
        g.compute_vertex_normals()?;
        let landscape = mesh(
            s,
            g,
            Material::Lambert(MeshLambertMaterial {
                properties: props(0x407000),
                ..Default::default()
            }),
        );
        let trees = roller_coaster::trees(s, landscape)?;
        let mut basic = MeshBasicMaterial::default();
        basic.properties.side = Side::Double;
        basic.properties.vertex_colors = true;
        mesh(s, trees, Material::Basic(basic));
        mesh(
            s,
            roller_coaster::sky()?,
            Material::Basic(MeshBasicMaterial {
                properties: props(0xffffff),
            }),
        );
        let mut phong = MeshPhongMaterial::default();
        phong.properties.vertex_colors = true;
        mesh(
            s,
            roller_coaster::roller_coaster(&Track, 1500)?,
            Material::Phong(phong),
        );
        let lifters = mesh(
            s,
            roller_coaster::lifters(&Track, 100)?,
            Material::Phong(MeshPhongMaterial::default()),
        );
        s.get_mut(lifters)?.position.y = 0.1;
        let mut shadow = MeshBasicMaterial {
            properties: props(0x305000),
        };
        shadow.properties.depth_write = false;
        shadow.properties.transparent = true;
        let shadow = mesh(
            s,
            roller_coaster::shadow(&Track, 500)?,
            Material::Basic(shadow),
        );
        s.get_mut(shadow)?.position.y = 0.1;
        let lambert = |color| {
            Material::Lambert(MeshLambertMaterial {
                properties: props(color),
                ..Default::default()
            })
        };
        let wheel = mesh(
            s,
            CylinderGeometry::build(10., 10., 5., 15, 1, false, 0., PI * 2.)?,
            lambert(0xff8080),
        );
        let n = s.get_mut(wheel)?;
        n.position = Vector3::new(-80., 10., -70.);
        n.quaternion = euler(Vector3::new(PI / 2., 0., 0.));
        let carousel = mesh(
            s,
            CylinderGeometry::build(5., 6., 4., 10, 1, false, 0., PI * 2.)?,
            lambert(0x8080ff),
        );
        s.get_mut(carousel)?.position = Vector3::new(50., 2., 30.);
        Ok(RollerCoaster {
            train,
            funfairs: [wheel, carousel],
            velocity: 0.,
            progress: 0.,
            prev_time: now(),
        })
    }
    fn animate(&mut self, s: &mut Scene) -> Result<()> {
        let time = now();
        let delta = time - self.prev_time;
        let rotations = [
            Vector3::new(PI / 2., time * 0.0004, 0.),
            Vector3::new(0., time * 0.0004, 0.),
        ];
        for (i, &f) in self.funfairs.iter().enumerate() {
            s.get_mut(f)?.quaternion = euler(rotations[i]);
        }
        self.progress += self.velocity;
        self.progress %= 1.;
        let mut position = Track.point_at(self.progress);
        position.y += 0.3;
        let tangent = Track.tangent_at(self.progress);
        self.velocity -= tangent.y * 0.0000001 * delta;
        self.velocity = self.velocity.clamp(0.00004, 0.0002);
        // banking
        let bank_delta = 0.01;
        let t1 = ((self.progress - bank_delta) % 1. + 1.) % 1.;
        let t2 = (self.progress + bank_delta) % 1.;
        let tangent1 = Track.tangent_at(t1);
        let tangent2 = Track.tangent_at(t2);
        let mut heading = tangent2.x.atan2(tangent2.z) - tangent1.x.atan2(tangent1.z);
        if heading > PI {
            heading -= PI * 2.;
        }
        if heading < -PI {
            heading += PI * 2.;
        }
        let bank = Quaternion::from_axis_angle(tangent, -(heading * 8.).atan() * 0.5);
        let up = bank * Vector3::Y;
        let n = s.get_mut(self.train)?;
        n.position = position;
        n.up = up;
        // Object3D.lookAt for a non-camera: +Z toward the target.
        n.quaternion = look_at(position - tangent, position, up);
        self.prev_time = time;
        Ok(())
    }
}

/// webgpu_xr_shadows.
struct Shadows {
    directional: Object3D,
    torus_knot: Object3D,
    box_: Object3D,
    icosahedron: Object3D,
    cone: Object3D,
    glass: Object3D,
    rotations: [Vector3; 5],
    timer: Timer,
}

impl Shadows {
    fn new(s: &mut Scene, camera: Object3D) -> Result<Self> {
        s.background = Color::from_hex(0x24282f);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x24282f),
            near: 8.,
            far: 16.,
        });
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 1.2;
        {
            let n = s.get_mut(camera)?;
            n.kind = perspective(50., 1., 0.1, 30.);
            n.position.y = 1.6;
        }
        let rig = s.insert(NodeKind::Group);
        s.get_mut(rig)?.position.z = 4.;
        s.add(rig, camera)?;
        hemisphere(s, 0xb8c5df, 0x3c3028, 1.5);
        let directional = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0xfff1d6),
            intensity: 4.,
            target: Vector3::ZERO,
        }));
        {
            let n = s.get_mut(directional)?;
            n.position = Vector3::new(-4., 6., 2.);
            n.cast_shadow = true;
            n.shadow.near = 0.1;
            n.shadow.far = 15.;
            n.shadow.extent = 5.;
            n.shadow.map_size = Some(2048);
            n.shadow.bias = -0.0005;
            n.shadow.filter = ShadowFilter::Pcf;
        }
        let standard = |color: u32, roughness: f64, metalness: f64| {
            Material::Standard(MeshStandardMaterial {
                properties: props(color),
                roughness,
                metalness,
                energy_conservation: true,
                ..Default::default()
            })
        };
        let caster =
            |s: &mut Scene, g: BufferGeometry, m: Material, p: Vector3| -> Result<Object3D> {
                let h = mesh(s, g, m);
                let n = s.get_mut(h)?;
                n.position = p;
                n.cast_shadow = true;
                n.receive_shadow = true;
                Ok(h)
            };
        let floor = mesh(
            s,
            PlaneGeometry::build(14., 14., 1, 1)?,
            standard(0x73786f, 0.9, 0.),
        );
        {
            let n = s.get_mut(floor)?;
            n.quaternion = euler(Vector3::new(-PI / 2., 0., 0.));
            n.receive_shadow = true;
        }
        caster(
            s,
            CylinderGeometry::build(1.25, 1.4, 0.3, 64, 1, false, 0., PI * 2.)?,
            standard(0x40464d, 0.6, 0.2),
            Vector3::new(0., 0.15, 0.),
        )?;
        let torus_knot = caster(
            s,
            TorusKnotGeometry::build(0.55, 0.18, 128, 24, 2, 3)?,
            standard(0xe7ad52, 0.25, 0.35),
            Vector3::new(0., 1.35, 0.),
        )?;
        let box_ = caster(
            s,
            BoxGeometry::build(0.7, 0.7, 0.7)?,
            standard(0xd85f50, 0.5, 0.),
            Vector3::new(-1.8, 0.6, 0.25),
        )?;
        let icosahedron = caster(
            s,
            IcosahedronGeometry::build(0.5, 2)?,
            standard(0x4f91b8, 0.35, 0.15),
            Vector3::new(1.8, 0.7, 0.1),
        )?;
        let cone = caster(
            s,
            CylinderGeometry::build(0., 0.5, 1.1, 48, 1, false, 0., PI * 2.)?,
            standard(0x68a66b, 0.65, 0.),
            Vector3::new(0., 0.55, -2.),
        )?;
        let mut physical = MeshPhysicalMaterial {
            base: MeshStandardMaterial {
                properties: props(0xb9e7ff),
                roughness: 0.05,
                metalness: 0.,
                energy_conservation: true,
                ..Default::default()
            },
            transmission: 1.,
            thickness: 0.9,
            ior: 1.45,
            attenuation_color: Color::from_hex(0x72b5d4),
            attenuation_distance: 3.,
            specular_intensity: 1.,
            ..Default::default()
        };
        physical.base.properties.side = Side::Double;
        let glass = caster(
            s,
            IcosahedronGeometry::build(0.65, 4)?,
            Material::Physical(physical),
            Vector3::new(0.9, 1.15, 1.2),
        )?;
        let rotations = [
            Vector3::ZERO,
            Vector3::new(0.2, 0.4, 0.1),
            Vector3::ZERO,
            Vector3::ZERO,
            Vector3::ZERO,
        ];
        s.get_mut(box_)?.quaternion = euler(rotations[1]);
        Ok(Shadows {
            directional,
            torus_knot,
            box_,
            icosahedron,
            cone,
            glass,
            rotations,
            timer: Timer::new(),
        })
    }
    fn animate(&mut self, s: &mut Scene) -> Result<()> {
        self.timer.update();
        let elapsed = self.timer.elapsed();
        let delta = self.timer.delta();
        let r = &mut self.rotations;
        s.get_mut(self.torus_knot)?.position.y = 1.35 + (elapsed * 1.5).sin() * 0.15;
        r[0].x += delta * 0.35;
        r[0].y += delta * 0.7;
        r[1].y += delta * 0.4;
        r[2].x += delta * 0.3;
        r[2].y -= delta * 0.5;
        r[3].y -= delta * 0.35;
        s.get_mut(self.glass)?.position.y = 1.15 + (elapsed * 1.1).sin() * 0.08;
        r[4].x -= delta * 0.2;
        r[4].y += delta * 0.3;
        for (h, rotation) in [
            self.torus_knot,
            self.box_,
            self.icosahedron,
            self.cone,
            self.glass,
        ]
        .into_iter()
        .zip(*r)
        {
            s.get_mut(h)?.quaternion = euler(rotation);
        }
        let n = s.get_mut(self.directional)?;
        n.position.x = (elapsed * 0.25).cos() * 4.;
        n.position.z = (elapsed * 0.25).sin() * 4.;
        Ok(())
    }
}

enum Example {
    Cubes(Box<Cubes>),
    RollerCoaster(Box<RollerCoaster>),
    Shadows(Box<Shadows>),
}

struct State {
    renderer: Renderer,
    surface: wgpu::Surface<'static>,
    configuration: wgpu::SurfaceConfiguration,
    canvas: web_sys::HtmlCanvasElement,
    target: RenderTarget,
    scene: Scene,
    camera: Object3D,
    xr: XrManager,
    example: Example,
    /// The tone-mapped example's scene target per view, before its output pass.
    view_target: Option<RenderTarget>,
    layer_format: wgpu::TextureFormat,
    /// The scene's background while a session overrides it.
    saved_background: Option<(Color, f64)>,
    models: Vec<ControllerModel>,
    factory: Rc<RefCell<ControllerModelFactory>>,
    frames: u32,
    xr_frames: u32,
}

impl State {
    /// The renderer's output: in-shader for DirectRenderPipeline ( cubes and
    /// rollercoaster ), or a frame-buffer target of the renderer's output
    /// type ( UnsignedByteType ) and an output pass ( shadows ).
    fn direct(&self) -> bool {
        !matches!(self.example, Example::Shadows(_))
    }
    fn animate(&mut self) -> Result<()> {
        let s = &mut self.scene;
        match &mut self.example {
            Example::Cubes(e) => e.animate(s, &self.xr)?,
            Example::RollerCoaster(e) => e.animate(s)?,
            Example::Shadows(e) => e.animate(s)?,
        }
        // XRControllerModel.updateMatrixWorld.
        for (i, model) in self.models.iter_mut().enumerate() {
            if let Some(c) = self.xr.controllers.get(i) {
                model.update(s, &c.gamepad)?;
            }
        }
        Ok(())
    }
    fn desktop_frame(&mut self) -> Result<()> {
        if self.xr.presenting {
            return Ok(());
        }
        let width = self.canvas.width().max(1);
        let height = self.canvas.height().max(1);
        if width != self.target.width || height != self.target.height {
            self.target.set_size(&self.renderer.device, width, height)?;
            self.configuration.width = width;
            self.configuration.height = height;
            self.surface
                .configure(&self.renderer.device, &self.configuration);
        }
        if let NodeKind::Camera(Camera::Perspective(c)) = &mut self.scene.get_mut(self.camera)?.kind
        {
            c.aspect = width as f64 / height as f64;
        }
        self.animate()?;
        self.renderer
            .render(&mut self.scene, self.camera, &self.target)?;
        let frame = self
            .surface
            .get_current_texture()
            .map_err(|e| Error::Gpu(e.to_string()))?;
        // The canvas is written as three writes it: encoded values from the
        // direct pipeline's shaders, or the output pass's tone mapping and
        // sRGB transfer computed in its shader.
        let format = self.configuration.format;
        let view = frame.texture.create_view(&Default::default());
        if self.direct() {
            self.renderer.blit(&self.target, &view, format);
        } else {
            self.renderer.blit_encoded(
                &self.target,
                &view,
                format,
                self.scene.exposure,
                self.scene.tone_mapping,
            );
        }
        frame.present();
        self.frames += 1;
        let _ = self
            .canvas
            .set_attribute("data-frames", &self.frames.to_string());
        Ok(())
    }
    fn xr_frame(
        &mut self,
        views: &[XrView],
        texture: JsValue,
        poses: &[f32],
        gamepads: &[f32],
    ) -> Result<()> {
        if !self.xr.presenting {
            return Err(Error::Invalid("XR frame without a session"));
        }
        let (width, height) = self
            .xr
            .target()
            .map(|t| (t.width, t.height))
            .ok_or(Error::Invalid("XR target"))?;
        self.xr.set_views(&mut self.scene, views)?;
        self.update_controllers(poses, gamepads)?;
        self.animate()?;
        self.xr.update_camera(&mut self.scene, self.camera)?;
        let color = self.renderer.device.create_texture_from_web(
            texture,
            &wgpu::TextureDescriptor {
                label: Some("XR projection layer"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 2,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.layer_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
        );
        let direct = self.direct();
        let (exposure, tone_mapping, layer_format) = (
            self.scene.exposure,
            self.scene.tone_mapping,
            self.layer_format,
        );
        let view_target = &mut self.view_target;
        self.xr.render(
            &self.renderer,
            &mut self.scene,
            color,
            |r, s, camera, target| {
                if direct {
                    return r.render(s, camera, target);
                }
                let scene_target = view_target
                    .as_ref()
                    .ok_or(Error::Invalid("XR view target"))?;
                r.render(s, camera, scene_target)?;
                r.blit_encoded(
                    scene_target,
                    &target.view,
                    layer_format,
                    exposure,
                    tone_mapping,
                );
                Ok(())
            },
        )?;
        self.xr_frames += 1;
        let _ = self
            .canvas
            .set_attribute("data-xr-frames", &self.xr_frames.to_string());
        Ok(())
    }
    /// Per controller slot: [ target ray tracked, 16 floats, grip tracked, 16
    /// floats ]; gamepads: [ button count, ( pressed, touched, value )*,
    /// axis count, axes* ] per slot.
    fn update_controllers(&mut self, poses: &[f32], gamepads: &[f32]) -> Result<()> {
        let mut gp = gamepads.iter().copied();
        for (i, chunk) in poses.chunks(34).enumerate() {
            let Some(controller) = self.xr.controllers.get_mut(i) else {
                break;
            };
            controller.update(&mut self.scene, &pose_from(chunk))?;
            let buttons = gp.next().unwrap_or(0.) as usize;
            controller.gamepad.buttons = (0..buttons)
                .map(|_| {
                    (
                        gp.next().unwrap_or(0.) > 0.5,
                        gp.next().unwrap_or(0.) > 0.5,
                        gp.next().unwrap_or(0.) as f64,
                    )
                })
                .collect();
            let axes = gp.next().unwrap_or(0.) as usize;
            controller.gamepad.axes = (0..axes).map(|_| gp.next().unwrap_or(0.) as f64).collect();
        }
        Ok(())
    }
}

fn pose_from(chunk: &[f32]) -> XrInputPose {
    let matrix = |o: usize| -> Option<Matrix4> {
        (chunk.get(o).copied().unwrap_or(0.) > 0.5 && chunk.len() >= o + 17)
            .then(|| Matrix4::from_cols_array(&std::array::from_fn(|i| chunk[o + 1 + i] as f64)))
    };
    XrInputPose {
        target_ray: matrix(0),
        grip: matrix(17),
    }
}

/// The XR examples' app, created by `web/gallery/xr.js`.
#[wasm_bindgen]
pub struct XrApp {
    state: Rc<RefCell<State>>,
}

fn js_error(e: Error) -> JsValue {
    JsValue::from_str(&e.to_string())
}

#[wasm_bindgen]
impl XrApp {
    #[wasm_bindgen(js_name = create)]
    pub async fn new(
        canvas: web_sys::HtmlCanvasElement,
        example: u32,
    ) -> std::result::Result<XrApp, JsValue> {
        async fn create(canvas: web_sys::HtmlCanvasElement, example: u32) -> Result<XrApp> {
            // renderer.xr.enabled: WebGPUBackend requests an XR-compatible adapter.
            wgpu::set_xr_compatible(true);
            let renderer = Renderer::new().await;
            wgpu::set_xr_compatible(false);
            let renderer = renderer?;
            renderer.viewport_transmission.set(true);
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::BROWSER_WEBGPU,
                ..Default::default()
            });
            let surface = instance
                .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
                .map_err(|e| Error::Gpu(e.to_string()))?;
            let mut configuration = surface
                .get_default_config(
                    &renderer.adapter,
                    canvas.width().max(1),
                    canvas.height().max(1),
                )
                .ok_or(Error::Gpu("surface configuration unavailable".into()))?;
            configuration.view_formats = vec![configuration.format.add_srgb_suffix()];
            surface.configure(&renderer.device, &configuration);
            let mut scene = Scene::new();
            let camera = scene.insert(perspective(50., 1., 0.1, 10.));
            let mut xr = XrManager::new();
            let example = match example {
                CUBES => Example::Cubes(Box::new(Cubes::new(&mut scene, camera, &mut xr)?)),
                ROLLERCOASTER => Example::RollerCoaster(Box::new(RollerCoaster::new(
                    &mut scene, camera, &mut xr,
                )?)),
                SHADOWS => Example::Shadows(Box::new(Shadows::new(&mut scene, camera)?)),
                _ => return Err(Error::Invalid("XR example")),
            };
            let mut models = Vec::new();
            if let Example::Cubes(_) = example {
                let grip = xr.controller_grip(&mut scene, 0);
                let model = ControllerModel::new(&mut scene);
                scene.add(grip, model.root)?;
                models.push(model);
            }
            let direct = !matches!(example, Example::Shadows(_));
            let target = RenderTarget::with_options(
                &renderer.device,
                canvas.width().max(1),
                canvas.height().max(1),
                output_options(direct, true),
            )?;
            Ok(XrApp {
                state: Rc::new(RefCell::new(State {
                    renderer,
                    surface,
                    configuration,
                    canvas,
                    target,
                    scene,
                    camera,
                    xr,
                    example,
                    view_target: None,
                    layer_format: wgpu::TextureFormat::Bgra8Unorm,
                    saved_background: None,
                    models,
                    factory: Default::default(),
                    frames: 0,
                    xr_frames: 0,
                })),
            })
        }
        create(canvas, example).await.map_err(js_error)
    }
    /// One animation frame on the canvas ( the renderer's own loop before
    /// and after a session ).
    pub fn frame(&self) -> std::result::Result<(), JsValue> {
        self.state.borrow_mut().desktop_frame().map_err(js_error)
    }
    /// Whether controller `index`'s model is in the scene ( its
    /// motionController is set ).
    pub fn controller_model_ready(&self, index: u32) -> bool {
        self.state
            .borrow()
            .models
            .get(index as usize)
            .is_some_and(ControllerModel::ready)
    }
    /// The renderer's GPUDevice, for XRGPUBinding.
    pub fn gpu_device(&self) -> JsValue {
        self.state
            .borrow()
            .renderer
            .device
            .as_web_device()
            .unwrap_or(JsValue::NULL)
    }
    /// `renderer.xr.setReferenceSpaceType`'s value.
    pub fn reference_space_type(&self) -> String {
        self.state.borrow().xr.reference_space_type.clone()
    }
    /// How many controllers the page made ( `getController` slots ).
    pub fn controller_count(&self) -> u32 {
        self.state.borrow().xr.controllers.len() as u32
    }
    /// setSession: the projection layer's size and color format, and the
    /// session's environmentBlendMode.
    pub fn xr_start(
        &self,
        width: u32,
        height: u32,
        format: &str,
        environment_blend_mode: &str,
    ) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let state = &mut *state;
        // Background: an additive display clears to opaque black, an
        // alpha-blended one to transparent black, whatever the background.
        state.saved_background = Some((state.scene.background, state.scene.background_alpha));
        match environment_blend_mode {
            "additive" => {
                (state.scene.background, state.scene.background_alpha) = (Color::BLACK, 1.)
            }
            "alpha-blend" => {
                (state.scene.background, state.scene.background_alpha) = (Color::BLACK, 0.)
            }
            _ => {}
        }
        let format = match format {
            "bgra8unorm" => wgpu::TextureFormat::Bgra8Unorm,
            "rgba8unorm" => wgpu::TextureFormat::Rgba8Unorm,
            "rgba16float" => wgpu::TextureFormat::Rgba16Float,
            _ => return Err(JsValue::from_str("unsupported XR layer format")),
        };
        state.layer_format = format;
        let direct = state.direct();
        let mut options = output_options(direct, true);
        options.format = format;
        if !direct {
            // The frame-buffer target carries the samples and depth; the
            // output pass writes the layer directly.
            options.samples = 1;
            options.depth_buffer = false;
            state.view_target = Some(
                RenderTarget::with_options(
                    &state.renderer.device,
                    width,
                    height,
                    output_options(false, true),
                )
                .map_err(js_error)?,
            );
        }
        state
            .xr
            .start(&state.renderer, &mut state.scene, width, height, options)
            .map_err(js_error)
    }
    /// onSessionEnd.
    pub fn xr_end(&self) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let state = &mut *state;
        let events: Vec<usize> = state
            .xr
            .controllers
            .iter()
            .enumerate()
            .filter(|(_, c)| c.source.is_some())
            .map(|(i, _)| i)
            .collect();
        state.xr.end(&mut state.scene).map_err(js_error)?;
        if let Some((background, alpha)) = state.saved_background.take() {
            (state.scene.background, state.scene.background_alpha) = (background, alpha);
        }
        for i in events {
            state.disconnected(i).map_err(js_error)?;
        }
        state.view_target = None;
        Ok(())
    }
    /// One XR frame: the views ( see `XrView::from_arrays` ), the layer's
    /// color texture, and the controllers' poses and gamepads.
    pub fn xr_frame(
        &self,
        matrices: &[f32],
        viewports: &[u32],
        texture: JsValue,
        poses: &[f32],
        gamepads: &[f32],
    ) -> std::result::Result<(), JsValue> {
        let views = XrView::from_arrays(matrices, viewports).map_err(js_error)?;
        self.state
            .borrow_mut()
            .xr_frame(&views, texture, poses, gamepads)
            .map_err(js_error)
    }
    /// The depthNear / depthFar for `session.updateRenderState`.
    pub fn xr_depth_range(&self) -> Vec<f64> {
        self.state
            .borrow()
            .xr
            .depth_range()
            .map(|(n, f)| vec![n, f])
            .unwrap_or_default()
    }
    /// A session input event for controller `index` ( selectstart,
    /// selectend, select, squeeze… ), after its pose update.
    pub fn xr_event(
        &self,
        index: u32,
        kind: &str,
        pose: &[f32],
    ) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let state = &mut *state;
        let index = index as usize;
        let Some(c) = state.xr.controllers.get_mut(index) else {
            return Ok(());
        };
        c.update(&mut state.scene, &pose_from(pose))
            .map_err(js_error)?;
        match kind {
            "selectstart" => c.is_selecting = true,
            "selectend" => c.is_selecting = false,
            _ => {}
        }
        Ok(())
    }
    /// `connected` for controller `index`; resolves once the controller's
    /// model ( if any ) is in the scene.
    pub async fn xr_connect(
        &self,
        index: u32,
        handedness: String,
        target_ray_mode: String,
        profiles: Vec<String>,
        has_gamepad: bool,
        has_hand: bool,
    ) -> std::result::Result<(), JsValue> {
        let source = XrInputSource {
            handedness,
            target_ray_mode,
            profiles,
            has_gamepad,
            has_hand,
        };
        let index = index as usize;
        let (id, factory) = {
            let mut state = self.state.borrow_mut();
            let state = &mut *state;
            let Some(c) = state.xr.controllers.get_mut(index) else {
                return Ok(());
            };
            c.source = Some(source.clone());
            if let Example::Cubes(e) = &mut state.example {
                e.connected(&mut state.scene, &state.xr, &source)
                    .map_err(js_error)?;
            }
            let id = state.models.get_mut(index).and_then(|m| m.connect(&source));
            (id, state.factory.clone())
        };
        let Some(id) = id else {
            return Ok(());
        };
        let loaded = ControllerModelFactory::load(&factory, source.clone()).await;
        let (profile, asset) = match loaded {
            Ok(v) => v,
            Err(e) => {
                web_sys::console::warn_1(&JsValue::from_str(&e.to_string()));
                return Ok(());
            }
        };
        let mut state = self.state.borrow_mut();
        let state = &mut *state;
        if let Some(model) = state.models.get_mut(index) {
            model
                .attach(&mut state.scene, id, &source.handedness, &profile, &asset)
                .map_err(js_error)?;
        }
        Ok(())
    }
    /// `disconnected` for controller `index`.
    pub fn xr_disconnect(&self, index: u32) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let state = &mut *state;
        let index = index as usize;
        if let Some(c) = state.xr.controllers.get_mut(index) {
            c.source = None;
            c.is_selecting = false;
        }
        state.disconnected(index).map_err(js_error)
    }
}

impl State {
    fn disconnected(&mut self, index: usize) -> Result<()> {
        if let Some(c) = self.xr.controllers.get(index) {
            let (ray, grip) = (c.target_ray, c.grip);
            self.scene.get_mut(ray)?.visible = false;
            self.scene.get_mut(grip)?.visible = false;
        }
        if let Example::Cubes(e) = &mut self.example
            && index == 0
        {
            e.disconnected(&mut self.scene, &self.xr)?;
        }
        if let Some(model) = self.models.get_mut(index) {
            model.disconnect(&mut self.scene)?;
        }
        Ok(())
    }
}

/// The canvas or view target: antialias's four samples; DirectRenderPipeline
/// encodes sRGB in its shaders; the tone-mapped example renders linear
/// UnsignedByteType ( its outputBufferType ) for the output pass.
fn output_options(direct: bool, antialias: bool) -> RenderTargetOptions {
    RenderTargetOptions {
        samples: if antialias { 4 } else { 1 },
        format: wgpu::TextureFormat::Rgba8Unorm,
        encode_srgb: direct,
        ..Default::default()
    }
}
