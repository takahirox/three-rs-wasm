//! webgl_geometry_spline_editor: three CatmullRomCurve3 outlines ( uniform
//! with the tension control, centripetal, chordal; 200 points each,
//! rewritten when a control point moves, as updateSplineOutline does )
//! through Lambert boxes of random colour, which the page attaches the
//! TransformControls gizmo to on hover and detaches on a click without
//! movement. A shadow-casting spot light throws the boxes' and the lines'
//! shadows onto a ShadowMaterial plane over a translucent grid; OrbitControls
//! rotate the camera while no gizmo drag is active. Points are added at
//! random positions and removed down to four, and exportSpline logs and
//! prompts the point code as the page does.
use super::controls_attributes::{Controls, camera_state, viewport_css};
use super::interactive_scenes::grid_helper;
use super::transform_controls::{Event, TransformControls};
use crate::curve::{CatmullRomCurve3, CatmullRomType};
use crate::raycast::Raycaster;
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const ARC_SEGMENTS: usize = 200;

fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}

enum Edit {
    Add,
    Remove,
    Export,
}

enum Input {
    Pointer(u32, f64, f64),
    Orbit(f64, f64, f64, bool, f64),
}

pub(super) struct Demo {
    controls: Controls,
    tc: TransformControls,
    orbit_enabled: bool,
    seed: u32,
    box_geometry: Arc<BufferGeometry>,
    boxes: Vec<Object3D>,
    /// The uniform, centripetal and chordal outlines.
    lines: [Object3D; 3],
    visible: [bool; 3],
    tension: f64,
    queue: Vec<Input>,
    pending: Vec<Edit>,
    down: Vector2,
    dirty: bool,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 250., 1000.);
        s.background = Color::from_hex(0xf0f0f0);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xf0f0f0),
            intensity: 3.,
        }));
        let spot = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 4.5,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 0.,
            angle: PI * 0.2,
            penumbra: 0.,
            ies: false,
        }));
        let n = s.get_mut(spot)?;
        n.position = Vector3::new(0., 1500., 200.);
        n.cast_shadow = true;
        n.shadow.near = 200.;
        n.shadow.far = 2000.;
        n.shadow.bias = -0.000222;
        n.shadow.map_size = Some(1024);
        let mut plane_geometry = PlaneGeometry::build(2000., 2000., 1, 1)?;
        plane_geometry.rotate_x(-PI / 2.)?;
        let mut shadow = ShadowMaterial::default();
        shadow.properties.opacity = 0.2;
        let plane = s.insert(NodeKind::Mesh(Mesh {
            geometry: Arc::new(plane_geometry),
            materials: vec![Arc::new(Material::Shadow(shadow))],
        }));
        let n = s.get_mut(plane)?;
        n.position.y = -200.;
        n.receive_shadow = true;
        let mut grid = grid_helper(2000., 100, 0x444444, 0x888888)?;
        let p = Arc::make_mut(&mut grid.material).properties_mut();
        p.opacity = 0.25;
        p.transparent = true;
        let grid = s.insert(NodeKind::Line(grid));
        s.get_mut(grid)?.position.y = -199.;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        let tc = TransformControls::new(s, c)?;
        let box_geometry = Arc::new(BoxGeometry::build(20., 20., 20.)?);
        let mut lines = vec![];
        for color in [0xff0000, 0x00ff00, 0x0000ff] {
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(vec![0.; ARC_SEGMENTS * 3], 3, false)?),
            );
            let mut material = LineBasicMaterial::default();
            material.properties.color = Color::from_hex(color);
            material.properties.opacity = 0.35;
            let line = s.insert(NodeKind::Line(Line {
                geometry: Arc::new(g),
                material: Arc::new(Material::Line(material)),
                segments: false,
            }));
            s.get_mut(line)?.cast_shadow = true;
            lines.push(line);
        }
        let lines: [Object3D; 3] = lines
            .try_into()
            .map_err(|_| Error::Invalid("spline lines"))?;
        let mut demo = Self {
            controls,
            tc,
            orbit_enabled: true,
            seed: 186,
            box_geometry,
            boxes: vec![],
            lines,
            visible: [true; 3],
            tension: 0.5,
            queue: vec![],
            pending: vec![],
            down: Vector2::ZERO,
            dirty: true,
        };
        // Four random boxes, then load() of the page's point set.
        for _ in 0..4 {
            demo.add_object(s)?;
        }
        let points = [
            [289.76843686945404, 452.51481137238443, 56.10018915737797],
            [-53.56300074753207, 171.49711742836848, -14.495472686253045],
            [-91.40118730204415, 176.4306956436485, -6.958271935582161],
            [-383.785318791128, 491.1365363371675, 47.869296953772746],
        ];
        for (h, p) in demo.boxes.iter().zip(points) {
            s.get_mut(*h)?.position = Vector3::from_array(p);
        }
        Ok(demo)
    }
    /// addSplineObject(): a random colour, then a random position.
    fn add_object(&mut self, s: &mut Scene) -> Result<()> {
        let mut material = MeshLambertMaterial::default();
        material.properties.color =
            Color::from_hex((random(&mut self.seed) * 16777215.).floor() as u32);
        let h = s.insert(NodeKind::Mesh(Mesh {
            geometry: self.box_geometry.clone(),
            materials: vec![Arc::new(Material::Lambert(material))],
        }));
        let x = random(&mut self.seed) * 1000. - 500.;
        let y = random(&mut self.seed) * 600.;
        let z = random(&mut self.seed) * 800. - 400.;
        let n = s.get_mut(h)?;
        n.position = Vector3::new(x, y, z);
        n.cast_shadow = true;
        n.receive_shadow = true;
        self.boxes.push(h);
        self.dirty = true;
        Ok(())
    }
    /// updateSplineOutline(): getPoint( i / 199 ) of each curve.
    fn update_outline(&mut self, s: &mut Scene) -> Result<()> {
        let points = self
            .boxes
            .iter()
            .map(|&h| Ok(s.get(h)?.position))
            .collect::<Result<Vec<_>>>()?;
        let types = [
            CatmullRomType::Uniform {
                tension: self.tension,
            },
            CatmullRomType::Centripetal,
            CatmullRomType::Chordal,
        ];
        for (k, curve_type) in types.into_iter().enumerate() {
            let mut curve = CatmullRomCurve3::new(points.clone());
            curve.curve_type = curve_type;
            let mut array = Vec::with_capacity(ARC_SEGMENTS * 3);
            for i in 0..ARC_SEGMENTS {
                let p = curve.point(i as f64 / (ARC_SEGMENTS - 1) as f64)?;
                array.extend([p.x as f32, p.y as f32, p.z as f32]);
            }
            if let NodeKind::Line(l) = &mut s.get_mut(self.lines[k])?.kind
                && let Some(Attribute::F32(a)) = Arc::make_mut(&mut l.geometry)
                    .attributes
                    .get_mut("position")
            {
                a.array_mut().copy_from_slice(&array);
            }
        }
        Ok(())
    }
    fn drain(&mut self) {
        for e in std::mem::take(&mut self.tc.events) {
            match e {
                Event::DraggingChanged(v) => self.orbit_enabled = !v,
                Event::ObjectChange => self.dirty = true,
                _ => {}
            }
        }
    }
    fn ndc(x: f64, y: f64) -> Vector2 {
        let (w, h, _) = viewport_css();
        Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.)
    }
    fn pointer(&mut self, s: &mut Scene, c: Object3D, kind: u32, x: f64, y: f64) -> Result<()> {
        let ndc = Self::ndc(x, y);
        match kind {
            // The canvas listeners ( OrbitControls, TransformControls ), then
            // the document's.
            10..=12 => {
                if self.tc.enabled {
                    self.tc.pointer_hover(s, ndc)?;
                    self.drain();
                    self.tc.pointer_down(s, ndc, kind as i32 - 10)?;
                    self.drain();
                }
                self.down = Vector2::new(x, y);
            }
            0 => {
                if self.tc.enabled {
                    if self.tc.dragging {
                        self.tc.pointer_move(s, ndc)?;
                    } else {
                        self.tc.pointer_hover(s, ndc)?;
                    }
                    self.drain();
                }
                // onPointerMove: the gizmo attaches to the hovered box.
                let (camera, world) = s.camera(c)?;
                let mut ray = Raycaster::default();
                ray.set_from_camera(ndc, camera, world)?;
                if let Some(hit) = ray.intersect_objects(s, &self.boxes, false)?.first()
                    && self.tc.object != Some(hit.object)
                {
                    self.tc.attach(s, hit.object)?;
                    self.drain();
                }
            }
            20..=22 => {
                if self.tc.enabled {
                    self.tc.pointer_up(s, kind as i32 - 20)?;
                    self.drain();
                }
                // onPointerUp: a click without movement detaches.
                if self.down.distance(Vector2::new(x, y)) == 0. {
                    self.tc.detach(s)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        for input in std::mem::take(&mut self.queue) {
            match input {
                Input::Pointer(kind, x, y) => self.pointer(s, c, kind, x, y)?,
                Input::Orbit(dx, dy, wheel, pan, height) => {
                    if self.orbit_enabled {
                        let camera = camera_state(s, c)?;
                        if wheel != 0. {
                            self.controls.dolly(wheel, &camera, Vector2::ZERO);
                        } else if pan {
                            self.controls.pan(&camera, dx, dy, height);
                        } else {
                            self.controls.rotate(dx, dy, height);
                        }
                        self.controls.update(s, c)?;
                    }
                }
            }
        }
        for edit in std::mem::take(&mut self.pending) {
            match edit {
                // addPoint(): a random box appended to the curves.
                Edit::Add => self.add_object(s)?,
                // removePoint(): down to four points.
                Edit::Remove => {
                    if self.boxes.len() > 4
                        && let Some(point) = self.boxes.pop()
                    {
                        if self.tc.object == Some(point) {
                            self.tc.detach(s)?;
                        }
                        s.dispose(point)?;
                        self.dirty = true;
                    }
                }
                // exportSpline(): the points as code, logged and prompted.
                Edit::Export => {
                    let lines = self
                        .boxes
                        .iter()
                        .map(|&h| {
                            let p = s.get(h)?.position;
                            Ok(format!("new THREE.Vector3({}, {}, {})", p.x, p.y, p.z))
                        })
                        .collect::<Result<Vec<_>>>()?;
                    web_sys::console::log_1(&lines.join(",\n").into());
                    if let Some(window) = web_sys::window() {
                        let code = format!("[{}]", lines.join(",\n\t"));
                        let _ =
                            window.prompt_with_message_and_default("copy and paste code", &code);
                    }
                }
            }
        }
        if std::mem::take(&mut self.dirty) {
            self.update_outline(s)?;
        }
        for (k, &line) in self.lines.iter().enumerate() {
            s.get_mut(line)?.visible = self.visible[k];
        }
        self.tc.update(s)?;
        self.drain();
        Ok(())
    }
    /// Absolute CSS pointer events: 0 move, 10 + button down, 20 + button up.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.queue.push(Input::Pointer(kind, x, y));
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        self.queue.push(Input::Orbit(dx, dy, wheel, pan, height));
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 | 2 | 3 => self.visible[[0, 0, 1, 2][index]] = value != 0.,
            1 => {
                self.tension = f64::from(value);
                self.dirty = true;
            }
            4 => self.pending.push(Edit::Add),
            5 => self.pending.push(Edit::Remove),
            6 => self.pending.push(Edit::Export),
            _ => return Err(Error::Invalid("spline editor parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
