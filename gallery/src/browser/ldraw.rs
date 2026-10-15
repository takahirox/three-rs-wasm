//! webgl_loader_ldraw: the page's seventeen packed LDraw models, parsed at
//! load by the port of LDrawLoader ( ldraw_loader.rs ) and shown under the
//! RoomEnvironment PMREM with ACES Filmic on a half-float output, rotated
//! 180° about X. Each part is a group with its faces ( one draw per color
//! group ), its lines and its conditional lines; the conditional lines run
//! LDrawConditionalLineMaterial's vertex test, which discards a segment
//! whose two control points fall on different sides of it on screen. The
//! damped orbit resets to the model's bounds on each model, and the GUI
//! switches the model, flat colors, merging, the building step, smoothed
//! normals and the two kinds of lines.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use super::ldraw_loader::{Child, Group, Kind, Loader, Object, Slot};
use crate::{
    Error, Result, attribute::*, camera::*, geometry::*, material::*, math::*, renderer::*,
    scene::*, shader::ShaderProgram,
};
use std::{cell::RefCell, collections::HashMap, f64::consts::PI, rc::Rc, sync::Arc};

const ASSETS: &str = "/web/gallery/assets/ldraw";
/// modelFileList in its order.
const MODELS: [&str; 17] = [
    "car.ldr_Packed.mpd",
    "1621-1-LunarMPVVehicle.mpd_Packed.mpd",
    "889-1-RadarTruck.mpd_Packed.mpd",
    "4838-1-MiniVehicles.mpd_Packed.mpd",
    "4915-1-MiniConstruction.mpd_Packed.mpd",
    "4918-1-MiniFlyers.mpd_Packed.mpd",
    "5935-1-IslandHopper.mpd_Packed.mpd",
    "30023-1-Lighthouse.ldr_Packed.mpd",
    "30051-1-X-wingFighter-Mini.mpd_Packed.mpd",
    "30054-1-AT-ST-Mini.mpd_Packed.mpd",
    "4489-1-AT-AT-Mini.mpd_Packed.mpd",
    "4494-1-Imperial%20Shuttle-Mini.mpd_Packed.mpd",
    "6965-1-TIEIntercep_4h4MXk5.mpd_Packed.mpd",
    "6966-1-JediStarfighter-Mini.mpd_Packed.mpd",
    "7140-1-X-wingFighter.mpd_Packed.mpd",
    "10174-1-ImperialAT-ST-UCS.mpd_Packed.mpd",
    "6156-1-WindowBrick.mpd_Packed.mpd",
];
/// LDrawConditionalLineMaterial's vertex test. The control points and the
/// direction travel in the normal ( control0 ), tangent ( control1 ) and the
/// two UV slots ( direction ); the discard flag in the line distance varying.
const CONDITIONAL: &str = "fn deform(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{return position;}
fn shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>{if surface.line_distance>0.5 {discard;}return u.custom[0];}";
const CONDITIONAL_PROJECTION: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{
 var out=surface;let mv=u.view*u.model;
 var c0=u.projection*mv*vec4(surface.local_normal,1.0);
 var c1=u.projection*vec4(surface.tangent.xyz+mv[3].xyz,1.0);
 var p0=u.projection*mv*vec4(position,1.0);
 var p1=u.projection*mv*vec4(position+vec3(surface.uv,surface.uv1.x),1.0);
 let c0xy=c0.xy/c0.w;let c1xy=c1.xy/c1.w;let p0xy=p0.xy/p0.w;let p1xy=p1.xy/p1.w;
 let dir=p1xy-p0xy;let norm=vec2(-dir.y,dir.x);
 let d0=dot(normalize(norm),normalize(c0xy-p1xy));let d1=dot(normalize(norm),normalize(c1xy-p1xy));
 out.line_distance=select(0.0,1.0,sign(d0)!=sign(d1));
 return out;}";
/// The GUI: model, flatColors, mergeModel, buildingStep, smoothNormals,
/// displayLines and conditionalLines.
#[derive(Clone, Copy, PartialEq)]
struct Options {
    model: usize,
    flat: bool,
    merge: bool,
    smooth: bool,
}
pub(super) struct Demo {
    controls: Controls,
    conditional: Arc<ShaderProgram>,
    options: Options,
    building_step: usize,
    steps: usize,
    lines: bool,
    conditional_lines: bool,
    /// GUI visibility changes, applied in prepare().
    visibility_changed: bool,
    /// The loaded options, a load in flight and its result.
    loaded: Option<Options>,
    requested: Option<Options>,
    loading: bool,
    result: Rc<RefCell<Option<Result<Loaded>>>>,
    reset_camera: bool,
    root: Option<Object3D>,
    /// Per group node: its building step; the line nodes and whether conditional.
    groups: Vec<(Object3D, usize)>,
    line_nodes: Vec<(Object3D, bool)>,
}
/// A finished load: the loader's materials, the model and its step count.
type Loaded = (Loader, Group, usize);
async fn load(options: Options) -> Result<Loaded> {
    let bytes = fetch(&format!("{ASSETS}/{}", MODELS[options.model])).await?;
    let text = String::from_utf8_lossy(&bytes);
    // Only smooth when not rendering with flat colors.
    let mut loader = Loader::new(options.smooth && !options.flat);
    let (group, steps) = loader.load(&text)?;
    Ok((loader, group, steps))
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(150., 200., 250.);
        s.background = Color::from_hex(0xdeebed);
        s.tone_mapping = ToneMapping::Aces;
        s.environment = Some(super::room_environment::environment(r)?);
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        let conditional = Arc::new(
            ShaderProgram::with_projection(r, CONDITIONAL, &[], &[], CONDITIONAL_PROJECTION)
                .await?,
        );
        let options = Options {
            model: 0,
            flat: false,
            merge: false,
            smooth: true,
        };
        // The first model loads before the page shows.
        let result = Rc::new(RefCell::new(Some(load(options).await)));
        Ok(Self {
            controls,
            conditional,
            options,
            building_step: 0,
            steps: 1,
            lines: true,
            conditional_lines: true,
            visibility_changed: false,
            loaded: None,
            requested: Some(options),
            loading: true,
            result,
            reset_camera: true,
            root: None,
            groups: vec![],
            line_nodes: vec![],
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if self.requested != Some(self.options) && !self.loading {
            self.requested = Some(self.options);
            self.loading = true;
            let (slot, options) = (self.result.clone(), self.options);
            wasm_bindgen_futures::spawn_local(async move {
                let result = load(options).await;
                *slot.borrow_mut() = Some(result);
            });
        }
        let finished = self.result.borrow_mut().take();
        if let Some(result) = finished {
            // A change made while loading reloads with the current options next.
            self.loading = false;
            let (loader, group, steps) = result?;
            let options = self.requested.ok_or(Error::Invalid("ldraw load"))?;
            self.install(s, c, &loader, &group, steps, options)?;
        }
        if std::mem::take(&mut self.visibility_changed) {
            self.visibility(s)?;
        }
        self.controls.update(s, c)
    }
    /// onLoad: replace the model, rotate it 180° about X, show every step and
    /// ( for a new model ) reset the orbit to its bounds.
    fn install(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        loader: &Loader,
        group: &Group,
        steps: usize,
        options: Options,
    ) -> Result<()> {
        if let Some(root) = self.root.take() {
            s.dispose(root)?;
        }
        self.groups.clear();
        self.line_nodes.clear();
        let mut materials = Materials::new(loader, options.flat, self.conditional.clone());
        let mut geometries = HashMap::new();
        let root = self.build(s, None, group, &mut materials, &mut geometries)?;
        s.get_mut(root)?.quaternion = Quaternion::from_rotation_x(PI);
        self.root = Some(root);
        self.loaded = Some(options);
        // LDrawUtils.mergeObject's group has no numBuildingSteps: the page's
        // step becomes NaN and hides the merged model.
        self.steps = if options.merge { 0 } else { steps };
        self.building_step = steps.saturating_sub(1);
        self.visibility(s)?;
        if std::mem::take(&mut self.reset_camera) {
            s.update()?;
            let (lo, hi) = self.bounds(s, root)?;
            let size = hi - lo;
            let radius = size.x.max(size.y.max(size.z)) * 0.5;
            let center = (lo + hi) * 0.5;
            // controls.reset() to target0 / position0.
            self.controls.set_target(center);
            s.get_mut(c)?.position = Vector3::new(-2.3, 1., 2.) * radius + center;
        }
        Ok(())
    }
    fn build(
        &mut self,
        s: &mut Scene,
        parent: Option<Object3D>,
        group: &Group,
        materials: &mut Materials,
        geometries: &mut HashMap<usize, Arc<BufferGeometry>>,
    ) -> Result<Object3D> {
        let h = s.insert(NodeKind::Group);
        {
            let n = s.get_mut(h)?;
            n.name = group.name.clone();
            n.position = group.position;
            n.quaternion = group.quaternion;
            n.scale = group.scale;
        }
        if let Some(p) = parent {
            s.add(p, h)?;
        }
        self.groups.push((h, group.building_step));
        for child in &group.children {
            match child {
                Child::Group(g) => {
                    self.build(s, Some(h), g, materials, geometries)?;
                }
                Child::Object(o) => self.object(s, h, o, materials, geometries)?,
            }
        }
        Ok(h)
    }
    /// A Mesh ( one draw per material group ), or line segments split into one
    /// node per material group over the shared geometry.
    fn object(
        &mut self,
        s: &mut Scene,
        parent: Object3D,
        o: &Object,
        materials: &mut Materials,
        geometries: &mut HashMap<usize, Arc<BufferGeometry>>,
    ) -> Result<()> {
        let key = Rc::as_ptr(&o.geometry) as usize;
        let geometry = match geometries.get(&key) {
            Some(g) => g.clone(),
            None => {
                let g = Arc::new(geometry(o)?);
                geometries.insert(key, g.clone());
                g
            }
        };
        let resolved: Vec<Arc<Material>> = o
            .materials
            .iter()
            .map(|slot| materials.get(slot))
            .collect::<Result<_>>()?;
        match o.kind {
            Kind::Mesh => {
                let h = s.insert(NodeKind::Mesh(Mesh {
                    geometry,
                    materials: resolved,
                }));
                s.add(parent, h)?;
            }
            Kind::Lines | Kind::Conditional => {
                for &(start, count, material) in &o.geometry.groups {
                    let h = s.insert(NodeKind::Line(Line {
                        geometry: geometry.clone(),
                        material: resolved[material].clone(),
                        segments: true,
                    }));
                    s.get_mut(h)?.draw_range = Some(DrawRange {
                        start,
                        count: Some(count),
                    });
                    s.add(parent, h)?;
                    self.line_nodes.push((h, o.kind == Kind::Conditional));
                }
            }
        }
        Ok(())
    }
    /// Box3.setFromObject: each geometry's box through its world matrix.
    fn bounds(&self, s: &Scene, root: Object3D) -> Result<(Vector3, Vector3)> {
        let mut lo = Vector3::splat(f64::INFINITY);
        let mut hi = Vector3::splat(f64::NEG_INFINITY);
        for h in s.traverse(root, false)? {
            let n = s.get(h)?;
            let Some(g) = n.geometry() else { continue };
            let Some(position) = g.attributes.get("position") else {
                continue;
            };
            let (mut a, mut b) = (
                Vector3::splat(f64::INFINITY),
                Vector3::splat(f64::NEG_INFINITY),
            );
            for i in 0..position.count() {
                let p = position.vector3(i)?;
                a = a.min(p);
                b = b.max(p);
            }
            if a.x > b.x {
                continue;
            }
            for k in 0..8 {
                let p = Vector3::new(
                    if k & 4 != 0 { b.x } else { a.x },
                    if k & 2 != 0 { b.y } else { a.y },
                    if k & 1 != 0 { b.z } else { a.z },
                );
                let q = n.matrix_world.project_point3(p);
                lo = lo.min(q);
                hi = hi.max(q);
            }
        }
        Ok((lo, hi))
    }
    /// updateObjectsVisibility.
    fn visibility(&self, s: &mut Scene) -> Result<()> {
        for &(h, conditional) in &self.line_nodes {
            s.get_mut(h)?.visible = if conditional {
                self.conditional_lines
            } else {
                self.lines
            };
        }
        let merged = self.loaded.is_some_and(|o| o.merge);
        for &(h, step) in &self.groups {
            s.get_mut(h)?.visible = !merged && step <= self.building_step;
        }
        Ok(())
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
    /// Model, Flat Colors, Merge model, Building step, Smooth Normals,
    /// Display Lines, Conditional Lines.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let on = value != 0.;
        match index {
            0 => {
                self.options.model = (value as usize).min(MODELS.len() - 1);
                self.reset_camera = true;
            }
            1 => self.options.flat = on,
            2 => self.options.merge = on,
            3 => {
                if self.steps > 1 {
                    self.building_step = (value.round().max(0.) as usize).min(self.steps - 1);
                }
            }
            4 => self.options.smooth = on,
            5 => self.lines = on,
            6 => self.conditional_lines = on,
            _ => return Err(Error::Invalid("ldraw parameter")),
        }
        self.visibility_changed = true;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
/// createObject's geometry with its groups; conditional lines carry their
/// control points and directions in the normal, tangent and UV slots.
fn geometry(o: &Object) -> Result<BufferGeometry> {
    let g = &o.geometry;
    let mut geometry = BufferGeometry::default();
    geometry.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(g.positions.clone(), 3, false)?),
    );
    if let Some(normals) = &g.normals {
        geometry.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(normals.clone(), 3, false)?),
        );
    }
    if let Some([c0, c1, d]) = &g.controls {
        geometry.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(c0.clone(), 3, false)?),
        );
        let tangent: Vec<f32> = c1.chunks(3).flat_map(|v| [v[0], v[1], v[2], 0.]).collect();
        geometry.set_attribute(
            "tangent",
            Attribute::F32(BufferAttribute::new(tangent, 4, false)?),
        );
        let uv: Vec<f32> = d.chunks(3).flat_map(|v| [v[0], v[1]]).collect();
        let uv1: Vec<f32> = d.chunks(3).flat_map(|v| [v[2], 0.]).collect();
        geometry.set_attribute("uv", Attribute::F32(BufferAttribute::new(uv, 2, false)?));
        geometry.set_attribute("uv1", Attribute::F32(BufferAttribute::new(uv1, 2, false)?));
    }
    for &(start, count, material) in &g.groups {
        geometry.add_group(start, count, material);
    }
    Ok(geometry)
}
/// The loader's materials as engine materials, one instance each.
struct Materials<'a> {
    loader: &'a Loader,
    flat: bool,
    program: Arc<ShaderProgram>,
    cache: HashMap<String, Arc<Material>>,
}
impl<'a> Materials<'a> {
    fn new(loader: &'a Loader, flat: bool, program: Arc<ShaderProgram>) -> Self {
        Self {
            loader,
            flat,
            program,
            cache: HashMap::new(),
        }
    }
    fn get(&mut self, slot: &Slot) -> Result<Arc<Material>> {
        let key = format!("{slot:?}");
        if let Some(m) = self.cache.get(&key) {
            return Ok(m.clone());
        }
        let l = self.loader;
        let material = match slot {
            Slot::Code(_) => return Err(Error::Invalid("ldraw unresolved material")),
            Slot::Face(i) => {
                let f = &l.faces[*i];
                let properties = MaterialProperties {
                    color: Color(f.color),
                    opacity: f.opacity,
                    transparent: f.transparent,
                    depth_write: !f.transparent,
                    polygon_offset: Some((1., 0)),
                    tone_mapped: !self.flat,
                    ..Default::default()
                };
                if self.flat {
                    // The page's MeshBasicMaterial conversion.
                    Material::Basic(MeshBasicMaterial { properties })
                } else {
                    Material::Standard(MeshStandardMaterial {
                        properties,
                        roughness: f.roughness,
                        metalness: f.metalness,
                        emissive: Color(f.emissive),
                        ..Default::default()
                    })
                }
            }
            Slot::Edge(i) => {
                let e = &l.edges[*i];
                Material::Line(LineBasicMaterial {
                    properties: MaterialProperties {
                        color: Color(e.color),
                        opacity: e.opacity,
                        transparent: e.transparent,
                        depth_write: !e.transparent,
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }
            Slot::Conditional(i) => {
                let e = &l.conditionals[*i];
                let mut m = ShaderMaterial::new(self.program.clone());
                m.properties.transparent = e.transparent;
                m.properties.depth_write = !e.transparent;
                m.uniforms[0] = [
                    e.color.x as f32,
                    e.color.y as f32,
                    e.color.z as f32,
                    e.opacity as f32,
                ];
                Material::Shader(m)
            }
        };
        let material = Arc::new(material);
        self.cache.insert(key, material.clone());
        Ok(material)
    }
}
