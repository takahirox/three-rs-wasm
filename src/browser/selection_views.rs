//! The keyframed animation groups and keys, DragControls, the selection box and
//! the WebGPU ArrayCamera grid from the pinned examples.
use super::controls_attributes::{viewport_css, webgl_perspective};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, raycast::Raycaster, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

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
/// The keyframed clip at `time`: three keys at 0, 1 and 2 in a three-second clip.
/// Returns the linear alpha between keys, the discrete key and the segment.
fn keyframe(time: f64) -> (usize, f64) {
    let u = time.rem_euclid(3.);
    if u < 1. {
        (0, u)
    } else if u < 2. {
        (1, u - 1.)
    } else {
        (2, 0.)
    }
}
/// QuaternionKeyframeTrack from identity to a half turn about x and back.
fn keyed_rotation(time: f64) -> Quaternion {
    let (segment, alpha) = keyframe(time);
    let turn = match segment {
        0 => alpha,
        1 => 1. - alpha,
        _ => 0.,
    };
    Quaternion::from_rotation_x(PI * turn)
}
/// The red, green, blue discrete color track and the 1, 0, 1 opacity track.
fn keyed_color_opacity(time: f64) -> (Color, f64) {
    let (segment, alpha) = keyframe(time);
    let color = [
        Color::linear(1., 0., 0.),
        Color::linear(0., 1., 0.),
        Color::linear(0., 0., 1.),
    ][segment];
    let opacity = match segment {
        0 => 1. - alpha,
        1 => alpha,
        _ => 1.,
    };
    (color, opacity)
}
fn properties_mut(s: &mut Scene, h: Object3D) -> Result<&mut Material> {
    match &mut s.get_mut(h)?.kind {
        NodeKind::Mesh(m) => Ok(Arc::make_mut(&mut m.materials[0])),
        _ => Err(Error::Invalid("mesh")),
    }
}
fn set_emissive(s: &mut Scene, h: Object3D, hex: u32) -> Result<()> {
    if let Material::Lambert(m) = properties_mut(s, h)? {
        m.emissive = Color::from_hex(hex);
    }
    Ok(())
}
/// Pointer events in CSS pixels, with the shift state at the time.
#[derive(Clone, Copy)]
struct Pointer {
    kind: u32,
    x: f64,
    y: f64,
    shift: bool,
}
/// DragControls' selection state.
struct Drag {
    objects: Vec<Object3D>,
    draggable: Vec<Object3D>,
    group: Object3D,
    transform_group: bool,
    selected: Option<Object3D>,
    rotate: bool,
    plane: Plane,
    offset: Vector3,
    inverse: Matrix4,
    up: Vector3,
    right: Vector3,
    previous: Vector2,
}
/// SelectionBox with its start and end points in NDC.
struct Selection {
    start: Vector3,
    end: Vector3,
    collection: Vec<Object3D>,
    down: bool,
}
/// webgpu_camera_array: 6 × 6 sub-cameras sharing one shadow pass.
struct Grid {
    cameras: Vec<Object3D>,
    mesh: Object3D,
    rotation: [f64; 2],
    output: Option<RenderTarget>,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    animated: Vec<Object3D>,
    drag: Option<Drag>,
    selection: Option<Selection>,
    grid: Option<Grid>,
    events: Vec<Pointer>,
    shift: bool,
}
impl Demo {
    pub fn create(s: &mut Scene, c: Object3D, id: u32) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            253 => (40., 1., 1000., Vector3::new(50., 50., 100.)),
            254 => (40., 1., 1000., Vector3::new(25., 25., 50.)),
            255 => (70., 0.1, 500., Vector3::new(0., 0., 25.)),
            256 => (70., 0.1, 500., Vector3::new(0., 0., 50.)),
            _ => (50., 0.1, 2000., Vector3::new(0., 0., 3.)),
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
            animated: vec![],
            drag: None,
            selection: None,
            grid: None,
            events: vec![],
            shift: false,
        };
        match id {
            253 | 254 => {
                s.look_at(c, Vector3::ZERO)?;
                d.keyframes(s)?
            }
            255 | 256 => d.boxes(s)?,
            _ => d.camera_array(s)?,
        }
        Ok(d)
    }
    fn keyframes(&mut self, s: &mut Scene) -> Result<()> {
        let geometry = Arc::new(BoxGeometry::build(5., 5., 5.)?);
        let mut basic = MeshBasicMaterial::default();
        basic.properties.transparent = true;
        let material = Arc::new(Material::Basic(basic));
        if self.id == 253 {
            // AnimationObjectGroup: 25 meshes sharing one material and one clip state.
            for i in 0..5 {
                for j in 0..5 {
                    let h = s.insert(NodeKind::Mesh(Mesh::new(
                        geometry.clone(),
                        material.clone(),
                    )));
                    s.get_mut(h)?.position =
                        Vector3::new(32. - 16. * i as f64, 0., 32. - 16. * j as f64);
                    self.animated.push(h);
                }
            }
            return Ok(());
        }
        // AxesHelper( 10 ): x red, y green, z blue, fading toward the tips.
        let mut axes = BufferGeometry::default();
        let positions = vec![
            0., 0., 0., 10., 0., 0., 0., 0., 0., 0., 10., 0., 0., 0., 0., 0., 0., 10.,
        ];
        let colors = vec![
            1., 0., 0., 1., 0.6, 0., 0., 1., 0., 0.6, 1., 0., 0., 0., 1., 0., 0.6, 1.,
        ];
        axes.set_attribute(
            "position",
            Attribute::F32(crate::attribute::BufferAttribute::new(positions, 3, false)?),
        );
        axes.set_attribute(
            "color",
            Attribute::F32(crate::attribute::BufferAttribute::new(colors, 3, false)?),
        );
        let mut line = LineBasicMaterial::default();
        line.properties.vertex_colors = true;
        s.insert(NodeKind::Line(Line {
            geometry: Arc::new(axes),
            material: Arc::new(Material::Line(line)),
            segments: true,
        }));
        let h = s.insert(NodeKind::Mesh(Mesh::new(geometry, material)));
        self.animated.push(h);
        Ok(())
    }
    fn apply_keyframes(&self, s: &mut Scene, t: f64) -> Result<()> {
        let (segment, alpha) = keyframe(t);
        let lerp = |a: f64, b: f64| a + (b - a) * alpha;
        let (color, opacity) = keyed_color_opacity(t);
        for &h in &self.animated {
            let n = s.get_mut(h)?;
            n.quaternion = keyed_rotation(t);
            if self.id == 254 {
                let (p, k) = match segment {
                    0 => (lerp(0., 30.), lerp(1., 2.)),
                    1 => (lerp(30., 0.), lerp(2., 1.)),
                    _ => (0., 1.),
                };
                n.position = Vector3::new(p, 0., 0.);
                n.scale = Vector3::splat(k);
            }
            let m = properties_mut(s, h)?.properties_mut();
            m.color = color;
            m.opacity = opacity;
        }
        Ok(())
    }
    fn boxes(&mut self, s: &mut Scene) -> Result<()> {
        s.background = Color::from_hex(0xf0f0f0);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xaaaaaa),
            intensity: 1.,
        }));
        let angle = if self.id == 255 { PI / 9. } else { PI / 5. };
        let light = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 10000.,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 2.,
            angle,
            penumbra: 0.,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(0., 25., 50.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            map_size: Some(1024),
            near: 10.,
            far: 100.,
            ..Default::default()
        };
        let geometry = Arc::new(BoxGeometry::build(1., 1., 1.)?);
        let (range, offset) = if self.id == 255 {
            ([30., 15., 20.], [15., 7.5, 10.])
        } else {
            ([80., 45., 45.], [40., 25., 25.])
        };
        let mut objects = vec![];
        for _ in 0..200 {
            // Color.setHex( Math.random() × 0xffffff ) floors the value.
            let hex = (random(&mut self.seed) * 16777215.).floor() as u32;
            let mut m = MeshLambertMaterial::default();
            m.properties.color = Color::from_hex(hex);
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Lambert(m)),
            )));
            let mut r = [0.; 9];
            for v in &mut r {
                *v = random(&mut self.seed);
            }
            let n = s.get_mut(h)?;
            n.position = Vector3::new(
                r[0] * range[0] - offset[0],
                r[1] * range[1] - offset[1],
                r[2] * range[2] - offset[2],
            );
            n.quaternion = euler(r[3] * 2. * PI, r[4] * 2. * PI, r[5] * 2. * PI);
            n.scale = Vector3::new(r[6] * 2. + 1., r[7] * 2. + 1., r[8] * 2. + 1.);
            n.cast_shadow = true;
            n.receive_shadow = true;
            objects.push(h);
        }
        if self.id == 255 {
            let group = s.insert(NodeKind::Group);
            self.drag = Some(Drag {
                draggable: objects.clone(),
                objects,
                group,
                transform_group: false,
                selected: None,
                rotate: false,
                plane: Plane {
                    normal: Vector3::Y,
                    constant: 0.,
                },
                offset: Vector3::ZERO,
                inverse: Matrix4::IDENTITY,
                up: Vector3::Y,
                right: Vector3::X,
                previous: Vector2::ZERO,
            });
        } else {
            self.selection = Some(Selection {
                start: Vector3::ZERO,
                end: Vector3::ZERO,
                collection: vec![],
                down: false,
            });
        }
        Ok(())
    }
    fn camera_array(&mut self, s: &mut Scene) -> Result<()> {
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x999999),
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(0.5, 0.5, 1.);
        n.cast_shadow = true;
        // DirectionalLightShadow's ±5 camera at zoom 4.
        n.shadow = crate::shadow::Shadow {
            near: 0.5,
            far: 500.,
            extent: 5. / 4.,
            ..Default::default()
        };
        let mut background = MeshPhongMaterial::default();
        background.properties.color = Color::from_hex(0x000066);
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(100., 100., 1, 1)?),
            Arc::new(Material::Phong(background)),
        )));
        let n = s.get_mut(h)?;
        n.position.z = -1.;
        n.receive_shadow = true;
        let mut red = MeshPhongMaterial::default();
        red.properties.color = Color::from_hex(0xff0000);
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(CylinderGeometry::build(
                0.5,
                0.5,
                1.,
                32,
                1,
                false,
                0.,
                2. * PI,
            )?),
            Arc::new(Material::Phong(red)),
        )));
        let n = s.get_mut(mesh)?;
        n.cast_shadow = true;
        n.receive_shadow = true;
        let cameras = (0..36)
            .map(|_| {
                s.insert(NodeKind::Camera(Camera::Perspective(
                    PerspectiveCamera::default(),
                )))
            })
            .collect();
        self.grid = Some(Grid {
            cameras,
            mesh,
            rotation: [0.; 2],
            output: None,
        });
        Ok(())
    }
    /// updateCameras(): each sub-camera copies the root's 50° lens and the window
    /// aspect, then looks at the origin from its grid position.
    fn update_cameras(&mut self, s: &mut Scene) -> Result<()> {
        let g = self.grid.as_ref().ok_or(Error::Invalid("grid"))?;
        let (w, h, _) = viewport_css();
        for y in 0..6 {
            for x in 0..6 {
                let c = g.cameras[6 * y + x];
                let n = s.get_mut(c)?;
                n.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                    fov: 50.,
                    near: 0.1,
                    far: 2000.,
                    aspect: w / h,
                    ..Default::default()
                }));
                n.position = Vector3::new(
                    x as f64 / 6. - 0.5,
                    0.5 - y as f64 / 6.,
                    1.5 + (x + y) as f64 * 0.5,
                ) * 2.;
                s.look_at(c, Vector3::ZERO)?;
            }
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    fn ndc(x: f64, y: f64) -> Vector2 {
        let (w, h, _) = viewport_css();
        Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.)
    }
    fn ray(s: &Scene, c: Object3D, pointer: Vector2) -> Result<Raycaster> {
        let (camera, world) = s.camera(c)?;
        let mut r = Raycaster::default();
        r.set_from_camera(pointer, camera, world)?;
        Ok(r)
    }
    /// DragControls' pointer handlers, then the example's shift-click grouping.
    fn drag_event(&mut self, s: &mut Scene, c: Object3D, e: Pointer) -> Result<()> {
        let d = self.drag.as_mut().ok_or(Error::Invalid("drag"))?;
        s.update()?;
        let pointer = Self::ndc(e.x, e.y);
        let ray = Self::ray(s, c, pointer)?;
        let camera_world = s.get(c)?.matrix_world;
        let direction = -camera_world.z_axis.truncate().normalize();
        match e.kind {
            10..=19 => {
                let button = e.kind - 10;
                let rotate = button == 2;
                let hits = ray.intersect_objects(s, &d.draggable, true)?;
                if let Some(hit) = hits.first() {
                    d.selected = if d.transform_group {
                        s.ancestors(hit.object)?
                            .into_iter()
                            .chain([hit.object])
                            .find(|&a| matches!(s.get(a).map(|n| &n.kind), Ok(NodeKind::Group)))
                    } else {
                        Some(hit.object)
                    };
                    if let Some(selected) = d.selected {
                        let world = s.get(selected)?.matrix_world;
                        let position = world.w_axis.truncate();
                        d.plane = Plane::from_normal_point(direction, position);
                        if let Some(point) = ray.ray.intersect_plane(d.plane) {
                            if rotate {
                                let q = s.get(c)?.quaternion;
                                d.up = (q * Vector3::Y).normalize();
                                d.right = (q * Vector3::X).normalize();
                            } else {
                                let parent = s
                                    .get(selected)?
                                    .parent()
                                    .map(|p| s.get(p).map(|n| n.matrix_world))
                                    .transpose()?
                                    .unwrap_or(Matrix4::IDENTITY);
                                d.inverse = parent.inverse();
                                d.offset = point - position;
                            }
                        }
                    }
                }
                d.rotate = rotate;
                d.previous = pointer;
            }
            20..=29 => {
                d.selected = None;
                // `click` follows the primary button's pointerup.
                if e.kind == 20 && e.shift {
                    d.draggable.clear();
                    let hits = ray.intersect_objects(s, &d.objects, true)?;
                    if let Some(hit) = hits.first() {
                        let object = hit.object;
                        if s.get(d.group)?.children().contains(&object) {
                            set_emissive(s, object, 0x000000)?;
                            // scene.attach( object ): keep the world transform at the root.
                            let world = s.get(object)?.matrix_world;
                            s.remove_from_parent(object)?;
                            let (scale, rotation, translation) =
                                world.to_scale_rotation_translation();
                            let n = s.get_mut(object)?;
                            n.position = translation;
                            n.quaternion = rotation;
                            n.scale = scale;
                        } else {
                            set_emissive(s, object, 0xaaaaaa)?;
                            s.attach(d.group, object)?;
                        }
                        d.transform_group = true;
                        d.draggable.push(d.group);
                    }
                    if s.get(d.group)?.children().is_empty() {
                        d.transform_group = false;
                        d.draggable = d.objects.clone();
                    }
                }
            }
            _ => {
                if let Some(selected) = d.selected {
                    if d.rotate {
                        let diff = (pointer - d.previous) * 2.;
                        let n = s.get_mut(selected)?;
                        n.rotate_on_world_axis(d.up, diff.x);
                        n.rotate_on_world_axis(d.right.normalize(), -diff.y);
                    } else if let Some(point) = ray.ray.intersect_plane(d.plane) {
                        s.get_mut(selected)?.position =
                            d.inverse.transform_point3(point - d.offset);
                    }
                }
                d.previous = pointer;
            }
        }
        Ok(())
    }
    /// SelectionBox._updateFrustum for the perspective camera, then the search.
    fn select(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let k = self.selection.as_mut().ok_or(Error::Invalid("selection"))?;
        let (start, mut end) = (k.start, k.end);
        if start.x == end.x {
            end.x += f64::EPSILON;
        }
        if start.y == end.y {
            end.y += f64::EPSILON;
        }
        s.update()?;
        let (projection, world) = match s.camera(c)? {
            (Camera::Perspective(p), world) => {
                (webgl_perspective(p.fov, p.aspect, p.near, p.far), world)
            }
            _ => return Err(Error::Invalid("selection camera")),
        };
        let inverse = projection.inverse();
        let unproject = |v: Vector3| world.transform_point3(inverse.project_point3(v));
        let top_left = Vector3::new(start.x.min(end.x), start.y.max(end.y), start.z);
        let (ex, ey) = (start.x.max(end.x), start.y.min(end.y));
        k.end = Vector3::new(ex, ey, end.z);
        let near = world.w_axis.truncate();
        let tl = unproject(top_left);
        let tr = unproject(Vector3::new(ex, top_left.y, 0.));
        let dr = unproject(Vector3::new(ex, ey, end.z));
        let dl = unproject(Vector3::new(top_left.x, ey, 0.));
        let deep = |v: Vector3| (v - near).normalize() * f64::MAX + near;
        let (t1, t2, t3) = (deep(tl), deep(tr), deep(dr));
        let from = |a: Vector3, b: Vector3, c: Vector3| {
            let normal = (c - b).cross(a - b).normalize();
            Plane {
                normal,
                constant: -a.dot(normal),
            }
        };
        let mut far = from(t3, t2, t1);
        far.normal = -far.normal;
        let planes = [
            from(near, tl, tr),
            from(near, tr, dr),
            from(dr, dl, near),
            from(dl, tl, near),
            from(tr, dr, dl),
            far,
        ];
        let mut collection = vec![];
        for h in s.handles().collect::<Vec<_>>() {
            let n = s.get(h)?;
            if !matches!(n.kind, NodeKind::Mesh(_)) {
                continue;
            }
            // The unit box's bounding sphere is centered on the origin.
            let center = n.matrix_world.w_axis.truncate();
            // Frustum.containsPoint rejects only negative distances; NaN planes pass.
            if !planes.iter().any(|p| p.distance_to_point(center) < 0.) {
                collection.push(h);
            }
        }
        k.collection = collection;
        Ok(())
    }
    fn selection_event(&mut self, s: &mut Scene, c: Object3D, e: Pointer) -> Result<()> {
        let p = Self::ndc(e.x, e.y);
        let point = Vector3::new(p.x, p.y, 0.5);
        let k = self.selection.as_mut().ok_or(Error::Invalid("selection"))?;
        match e.kind {
            10..=19 => {
                for &h in &k.collection.clone() {
                    set_emissive(s, h, 0x000000)?;
                }
                k.start = point;
                k.down = true;
            }
            20..=29 => {
                k.down = false;
                k.end = point;
                self.select(s, c)?;
                let k = self.selection.as_ref().ok_or(Error::Invalid("selection"))?;
                for &h in &k.collection.clone() {
                    set_emissive(s, h, 0xffffff)?;
                }
            }
            _ => {
                if !k.down {
                    return Ok(());
                }
                for &h in &k.collection.clone() {
                    set_emissive(s, h, 0x000000)?;
                }
                k.end = point;
                self.select(s, c)?;
                let k = self.selection.as_ref().ok_or(Error::Invalid("selection"))?;
                for &h in &k.collection.clone() {
                    set_emissive(s, h, 0xffffff)?;
                }
            }
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        for e in std::mem::take(&mut self.events) {
            match self.id {
                255 => self.drag_event(s, c, e)?,
                256 => self.selection_event(s, c, e)?,
                _ => {}
            }
        }
        match self.id {
            253 | 254 => self.apply_keyframes(s, t)?,
            257 => {
                self.update_cameras(s)?;
                let g = self.grid.as_mut().ok_or(Error::Invalid("grid"))?;
                for _ in 0..steps {
                    g.rotation[0] += 0.005;
                    g.rotation[1] += 0.01;
                }
                s.get_mut(g.mesh)?.quaternion = euler(g.rotation[0], 0., g.rotation[1]);
            }
            _ => {}
        }
        Ok(())
    }
    /// The ArrayCamera: one clear, one shadow pass, 36 viewports with WebGPU's
    /// top-left origin.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        _c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let Some(g) = self.grid.as_mut() else {
            return Ok(false);
        };
        if g.output.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.options.samples != out.options.samples
        }) {
            g.output = Some(RenderTarget::with_options(
                &r.device,
                out.width,
                out.height,
                out.options.clone(),
            )?);
        }
        let target = g.output.as_mut().ok_or(Error::Invalid("grid target"))?;
        let (w, h, dpr) = viewport_css();
        let (cw, ch) = (w / 6., h / 6.);
        let mut result = Ok(());
        for y in 0..6 {
            for x in 0..6 {
                let i = 6 * y + x;
                let vx = ((x as f64 * cw).floor() * dpr).floor() as u32;
                let vy = ((y as f64 * ch).floor() * dpr).floor() as u32;
                let vw = (cw.ceil() * dpr).floor() as u32;
                let vh = (ch.ceil() * dpr).floor() as u32;
                let vx = vx.min(out.width - 1);
                let vy = vy.min(out.height - 1);
                target.viewport = [vx, vy, vw.min(out.width - vx), vh.min(out.height - vy)];
                target.set_load_color(i != 0);
                s.shadow_auto_update = i == 0;
                result = r.render(s, g.cameras[i], target);
                if result.is_err() {
                    break;
                }
            }
            if result.is_err() {
                break;
            }
        }
        s.shadow_auto_update = true;
        target.viewport = [0, 0, out.width, out.height];
        target.set_load_color(false);
        result.map(|_| true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.grid.as_ref().and_then(|g| g.output.as_ref())
    }
    /// Absolute CSS-pixel pointer events, applied in prepare().
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if matches!(self.id, 255 | 256) {
            self.events.push(Pointer {
                kind,
                x,
                y,
                shift: self.shift,
            });
        }
    }
    /// keydown sets the selection mode only for Shift; any keyup clears it.
    pub fn key(&mut self, code: u32, down: bool) {
        self.shift = down && code == 16;
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("selection/views parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
