//! three r187's XRControllerModelFactory ( `examples/jsm/webxr/` ) and the
//! motion-controllers library it bundles ( `examples/jsm/libs/
//! motion-controllers.module.js`, @webxr-input-profiles/motion-controllers
//! 1.0 ): on `connected`, the input source's profile is fetched from the
//! WebXR input profiles CDN, its glTF model is loaded under the grip space,
//! and every frame the gamepad's buttons and axes move the model's nodes
//! between their min and max transforms.
use super::gltf_viewer::{fetch, load_asset_bytes};
use crate::{
    Error, Result,
    geometry::SphereGeometry,
    material::*,
    math::*,
    scene::*,
    xr::{XrGamepad, XrInputSource},
};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

pub const DEFAULT_PROFILES_PATH: &str =
    "https://cdn.jsdelivr.net/npm/@webxr-input-profiles/assets@1.0/dist/profiles";
const DEFAULT_PROFILE: &str = "generic-trigger";
const BUTTON_TOUCH_THRESHOLD: f64 = 0.05;
const AXIS_TOUCH_THRESHOLD: f64 = 0.1;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Default,
    Touched,
    Pressed,
}
impl State {
    fn name(self) -> &'static str {
        match self {
            State::Default => "default",
            State::Touched => "touched",
            State::Pressed => "pressed",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ComponentProperty {
    Button,
    XAxis,
    YAxis,
    State,
}

struct VisualResponse {
    property: ComponentProperty,
    states: Vec<String>,
    transform: bool,
    value_node_name: String,
    min_node_name: String,
    max_node_name: String,
    value_node: Option<Object3D>,
    min_node: Option<Object3D>,
    max_node: Option<Object3D>,
    value: f64,
    visible: bool,
}

impl VisualResponse {
    fn update(&mut self, values: &Values) {
        // normalizeAxes.
        let (mut x, mut y) = (values.x_axis.unwrap_or(0.), values.y_axis.unwrap_or(0.));
        let hypotenuse = (x * x + y * y).sqrt();
        if hypotenuse > 1. {
            let theta = y.atan2(x);
            x = theta.cos();
            y = theta.sin();
        }
        let in_state = self.states.iter().any(|s| s == values.state.name());
        match self.property {
            ComponentProperty::XAxis => self.value = if in_state { x * 0.5 + 0.5 } else { 0.5 },
            ComponentProperty::YAxis => self.value = if in_state { y * 0.5 + 0.5 } else { 0.5 },
            ComponentProperty::Button => {
                self.value = if in_state {
                    values.button.unwrap_or(0.)
                } else {
                    0.
                }
            }
            ComponentProperty::State => {
                if self.transform {
                    self.value = if in_state { 1. } else { 0. };
                } else {
                    self.visible = in_state;
                }
            }
        }
    }
}

struct Values {
    state: State,
    button: Option<f64>,
    x_axis: Option<f64>,
    y_axis: Option<f64>,
}

struct Component {
    id: String,
    touchpad: bool,
    touch_point_node_name: Option<String>,
    button: Option<usize>,
    x_axis: Option<usize>,
    y_axis: Option<usize>,
    values: Values,
    responses: Vec<VisualResponse>,
}

impl Component {
    fn update(&mut self, gamepad: &XrGamepad) {
        self.values.state = State::Default;
        if let Some(i) = self.button
            && let Some(&(pressed, touched, value)) = gamepad.buttons.get(i)
        {
            let value = value.clamp(0., 1.);
            self.values.button = Some(value);
            if pressed || value == 1. {
                self.values.state = State::Pressed;
            } else if touched || value > BUTTON_TOUCH_THRESHOLD {
                self.values.state = State::Touched;
            }
        }
        for (index, slot) in [(self.x_axis, 0), (self.y_axis, 1)] {
            if let Some(i) = index
                && let Some(&axis) = gamepad.axes.get(i)
            {
                let axis = axis.clamp(-1., 1.);
                if slot == 0 {
                    self.values.x_axis = Some(axis);
                } else {
                    self.values.y_axis = Some(axis);
                }
                if self.values.state == State::Default && axis.abs() > AXIS_TOUCH_THRESHOLD {
                    self.values.state = State::Touched;
                }
            }
        }
        for r in &mut self.responses {
            r.update(&self.values);
        }
    }
}

/// MotionController: the layout's components for the source's handedness.
struct MotionController {
    components: Vec<Component>,
}

impl MotionController {
    fn new(profile: &serde_json::Value, handedness: &str) -> Result<Self> {
        let layout = &profile["layouts"][handedness];
        let components = layout["components"]
            .as_object()
            .ok_or(Error::Invalid("motion controller layout"))?;
        let mut out = Vec::new();
        for (id, c) in components {
            let indices = c["gamepadIndices"]
                .as_object()
                .filter(|o| !o.is_empty())
                .ok_or(Error::Invalid("Invalid arguments supplied"))?;
            let index = |k: &str| indices.get(k).and_then(|v| v.as_u64()).map(|v| v as usize);
            let mut responses = Vec::new();
            for (_, r) in c["visualResponses"]
                .as_object()
                .ok_or(Error::Invalid("Invalid arguments supplied"))?
            {
                let s = |k: &str| r[k].as_str().unwrap_or("").to_owned();
                let property = match r["componentProperty"].as_str() {
                    Some("button") => ComponentProperty::Button,
                    Some("xAxis") => ComponentProperty::XAxis,
                    Some("yAxis") => ComponentProperty::YAxis,
                    Some("state") => ComponentProperty::State,
                    _ => {
                        return Err(Error::Invalid(
                            "Unexpected visualResponse componentProperty",
                        ));
                    }
                };
                let transform = r["valueNodeProperty"].as_str() == Some("transform");
                responses.push(VisualResponse {
                    property,
                    states: r["states"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(str::to_owned))
                                .collect()
                        })
                        .unwrap_or_default(),
                    transform,
                    value_node_name: s("valueNodeName"),
                    min_node_name: if transform {
                        s("minNodeName")
                    } else {
                        String::new()
                    },
                    max_node_name: if transform {
                        s("maxNodeName")
                    } else {
                        String::new()
                    },
                    value_node: None,
                    min_node: None,
                    max_node: None,
                    value: 0.,
                    visible: false,
                });
            }
            let mut component = Component {
                id: id.clone(),
                touchpad: c["type"].as_str() == Some("touchpad"),
                touch_point_node_name: c["touchPointNodeName"].as_str().map(str::to_owned),
                button: index("button"),
                x_axis: index("xAxis"),
                y_axis: index("yAxis"),
                values: Values {
                    state: State::Default,
                    button: index("button").map(|_| 0.),
                    x_axis: index("xAxis").map(|_| 0.),
                    y_axis: index("yAxis").map(|_| 0.),
                },
                responses,
            };
            // The VisualResponse constructor's default values.
            let defaults = Values {
                state: State::Default,
                button: Some(0.),
                x_axis: Some(0.),
                y_axis: Some(0.),
            };
            for r in &mut component.responses {
                r.update(&defaults);
            }
            out.push(component);
        }
        Ok(MotionController { components: out })
    }
}

/// A loaded asset: the controller's glTF, kept for later connections
/// ( `_assetCache` ).
type Asset = Rc<(gltf::Gltf, Vec<Vec<u8>>, Vec<Texture>)>;

/// XRControllerModelFactory: the profiles path and the asset cache.
#[derive(Default)]
pub struct ControllerModelFactory {
    cache: HashMap<String, Asset>,
    profiles: Option<Rc<serde_json::Value>>,
}

/// XRControllerModel: an Object3D under the grip space.
pub struct ControllerModel {
    pub root: Object3D,
    connection: u32,
    motion: Option<MotionController>,
    scene: Option<Object3D>,
}

impl ControllerModel {
    pub fn new(scene: &mut Scene) -> Self {
        ControllerModel {
            root: scene.insert(NodeKind::Group),
            connection: 0,
            motion: None,
            scene: None,
        }
    }
    pub fn ready(&self) -> bool {
        self.motion.is_some()
    }
    /// `connected`: a new connection id, or None when the source has no
    /// model ( not a tracked pointer, no gamepad, or a hand ).
    pub fn connect(&mut self, source: &XrInputSource) -> Option<u32> {
        if source.target_ray_mode != "tracked-pointer" || !source.has_gamepad || source.has_hand {
            return None;
        }
        self.connection += 1;
        Some(self.connection)
    }
    /// `disconnected`: the model's scene is removed.
    pub fn disconnect(&mut self, scene: &mut Scene) -> Result<()> {
        self.connection += 1;
        self.motion = None;
        if let Some(h) = self.scene.take() {
            scene.remove(self.root, h)?;
        }
        Ok(())
    }
    /// XRControllerModel.updateMatrixWorld: the gamepad's state on the nodes.
    pub fn update(&mut self, scene: &mut Scene, gamepad: &XrGamepad) -> Result<()> {
        let Some(motion) = &mut self.motion else {
            return Ok(());
        };
        for component in &mut motion.components {
            component.update(gamepad);
            for r in &component.responses {
                let Some(value_node) = r.value_node else {
                    continue;
                };
                if !r.transform {
                    scene.get_mut(value_node)?.visible = r.visible;
                } else if let (Some(min), Some(max)) = (r.min_node, r.max_node) {
                    let (min_q, min_p) = {
                        let n = scene.get(min)?;
                        (n.quaternion, n.position)
                    };
                    let (max_q, max_p) = {
                        let n = scene.get(max)?;
                        (n.quaternion, n.position)
                    };
                    let n = scene.get_mut(value_node)?;
                    n.quaternion = slerp(min_q, max_q, r.value);
                    n.position = min_p + (max_p - min_p) * r.value;
                    n.update_matrix();
                }
            }
        }
        Ok(())
    }
}

/// Quaternion.slerpQuaternions ( three's algorithm ).
fn slerp(a: Quaternion, b: Quaternion, t: f64) -> Quaternion {
    if t == 0. {
        return a;
    }
    if t == 1. {
        return b;
    }
    let (x, y, z, w) = (a.x, a.y, a.z, a.w);
    let mut cos_half = w * b.w + x * b.x + y * b.y + z * b.z;
    let mut q = if cos_half < 0. {
        cos_half = -cos_half;
        Quaternion::from_xyzw(-b.x, -b.y, -b.z, -b.w)
    } else {
        b
    };
    if cos_half >= 1. {
        return a;
    }
    let sqr_sin_half = 1. - cos_half * cos_half;
    if sqr_sin_half <= f64::EPSILON {
        let s = 1. - t;
        q = Quaternion::from_xyzw(
            s * x + t * q.x,
            s * y + t * q.y,
            s * z + t * q.z,
            s * w + t * q.w,
        );
        return q.normalize();
    }
    let sin_half = sqr_sin_half.sqrt();
    let half = sin_half.atan2(cos_half);
    let ratio_a = ((1. - t) * half).sin() / sin_half;
    let ratio_b = (t * half).sin() / sin_half;
    Quaternion::from_xyzw(
        x * ratio_a + q.x * ratio_b,
        y * ratio_a + q.y * ratio_b,
        z * ratio_a + q.z * ratio_b,
        w * ratio_a + q.w * ratio_b,
    )
}

fn warn(message: &str) {
    web_sys::console::warn_1(&wasm_bindgen::JsValue::from_str(message));
}

async fn fetch_json(url: &str) -> Result<serde_json::Value> {
    serde_json::from_slice(&fetch(url).await?).map_err(|e| Error::Asset(e.to_string()))
}

impl ControllerModelFactory {
    /// fetchProfile: the first of the source's profiles in the profiles list
    /// ( or the default ), its profile and the layout's asset URL.
    async fn profile(
        profiles: Rc<serde_json::Value>,
        source: &XrInputSource,
    ) -> Result<(serde_json::Value, String)> {
        let base = DEFAULT_PROFILES_PATH;
        let path = source
            .profiles
            .iter()
            .find_map(|id| profiles[id.as_str()]["path"].as_str())
            .or_else(|| profiles[DEFAULT_PROFILE]["path"].as_str())
            .ok_or(Error::Asset("No matching profile name found".into()))?;
        let profile_path = format!("{base}/{path}");
        let profile = fetch_json(&profile_path).await?;
        let layout = &profile["layouts"][source.handedness.as_str()];
        let asset = layout["assetPath"].as_str().ok_or_else(|| {
            Error::Asset(format!(
                "No matching handedness, {}, in profile",
                source.handedness
            ))
        })?;
        Ok((profile.clone(), profile_path.replace("profile.json", asset)))
    }
    /// The asynchronous half of `connected`: the profile, then the model
    /// ( cached by URL ). Returns what `attach` adds to the scene.
    pub async fn load(
        factory: &std::cell::RefCell<Self>,
        source: XrInputSource,
    ) -> Result<(serde_json::Value, Asset)> {
        let list = factory.borrow().profiles.clone();
        let list = match list {
            Some(list) => list,
            None => {
                let list = Rc::new(
                    fetch_json(&format!("{DEFAULT_PROFILES_PATH}/profilesList.json")).await?,
                );
                factory.borrow_mut().profiles = Some(list.clone());
                list
            }
        };
        let (profile, url) = Self::profile(list, &source).await?;
        let cached = factory.borrow().cache.get(&url).cloned();
        let asset = match cached {
            Some(asset) => asset,
            None => {
                let bytes = fetch(&url).await?;
                let base = url.rsplit_once('/').map(|(b, _)| b).unwrap_or("");
                let asset = Rc::new(load_asset_bytes(&bytes, base).await?);
                factory.borrow_mut().cache.insert(url, asset.clone());
                asset
            }
        };
        Ok((profile, asset))
    }
}

impl ControllerModel {
    /// addAssetSceneToControllerModel for connection `id` ( dropped when a
    /// later connection or a disconnection superseded it ).
    pub fn attach(
        &mut self,
        scene: &mut Scene,
        id: u32,
        handedness: &str,
        profile: &serde_json::Value,
        asset: &Asset,
    ) -> Result<()> {
        if id != self.connection {
            return Ok(());
        }
        let mut motion = MotionController::new(profile, handedness)?;
        let (gltf, buffers, images) = &**asset;
        let instance =
            crate::gltf::import_animated_decoded(gltf, buffers, images)?.instantiate(scene)?;
        // WebGPURenderer's standard materials conserve energy.
        for &h in &instance.meshes {
            if let NodeKind::Mesh(m) = &mut scene.get_mut(h)?.kind {
                for m in &mut m.materials {
                    if let Material::Standard(m)
                    | Material::Physical(MeshPhysicalMaterial { base: m, .. }) = Arc::make_mut(m)
                    {
                        m.energy_conservation = true;
                    }
                }
            }
        }
        let group = scene.insert(NodeKind::Group);
        for &root in &instance.roots {
            scene.add(group, root)?;
        }
        // findNodes.
        for component in &mut motion.components {
            if component.touchpad {
                // An undefined touchPointNodeName finds no node.
                let name = component.touch_point_node_name.clone();
                let node = match &name {
                    Some(name) => scene.get_object_by_name(group, name)?,
                    None => None,
                };
                match node {
                    Some(node) => {
                        let sphere = scene.insert(NodeKind::Mesh(Mesh::new(
                            Arc::new(SphereGeometry::build(0.001, 32, 16)?),
                            Arc::new(Material::Basic(MeshBasicMaterial {
                                properties: MaterialProperties {
                                    color: Color::from_hex(0x0000ff),
                                    ..Default::default()
                                },
                            })),
                        )));
                        scene.add(node, sphere)?;
                    }
                    None => warn(&format!(
                        "Could not find touch dot, {}, in touchpad component {}",
                        name.as_deref().unwrap_or("undefined"),
                        component.id
                    )),
                }
            }
            for r in &mut component.responses {
                if r.transform {
                    r.min_node = scene.get_object_by_name(group, &r.min_node_name)?;
                    r.max_node = scene.get_object_by_name(group, &r.max_node_name)?;
                    if r.min_node.is_none() {
                        warn(&format!("Could not find {} in the model", r.min_node_name));
                        continue;
                    }
                    if r.max_node.is_none() {
                        warn(&format!("Could not find {} in the model", r.max_node_name));
                        continue;
                    }
                }
                r.value_node = scene.get_object_by_name(group, &r.value_node_name)?;
                if r.value_node.is_none() {
                    warn(&format!(
                        "Could not find {} in the model",
                        r.value_node_name
                    ));
                }
            }
        }
        scene.add(self.root, group)?;
        self.scene = Some(group);
        self.motion = Some(motion);
        Ok(())
    }
}
