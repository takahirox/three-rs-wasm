//! webgl_watch: the Draco rolex glTF, tilted 45°, under Neutral tone mapping
//! ( exposure 0.7 ) against the blurred lobe HDR that also lights it ( × 1.5 ),
//! with a PCF-shadowing directional light, a blue point light under the dial
//! and the glass replaced by an additive, iridescent, clear-coated physical
//! material. The materials drop their vertex colours, as the page sets them;
//! the GUI's roughness and metalness edit every gold and silver slot ( the page
//! shares each material by name ), its opacity the glass. The page's six-second Quadratic.Out tween
//! brings the camera in to distance 1 before OrbitControls ( damped ) take
//! over, and the hands follow the clock: the local time, or under the gallery
//! clock 2024-01-15 10:08:30.250 local plus the gallery time. The page's
//! optional TAA and UnrealBloom effects ( off by default ) are not ported.
use super::controls_attributes::{Controls, additive, camera_state};
use super::gltf_viewer::{fetch, load_asset};
use crate::shadow::Shadow;
use crate::{Error, Result, camera::*, material::*, math::*, renderer::*, scene::*};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
const TORAD: f64 = PI / 180.;
const TARGET: Vector3 = Vector3::new(0., -0.1, 0.);

pub struct Demo {
    controls: Controls,
    /// The tween's start ( distance, phi, theta ).
    start: (f64, f64, f64),
    tweening: bool,
    time: f64,
    /// The gallery clock, once it drives the page.
    seeked: Option<f64>,
    hands: HashMap<&'static str, Object3D>,
    /// The Gold and Silver material slots: ( mesh, slot ).
    gold_silver: Vec<(Object3D, usize)>,
    glass: Object3D,
    /// roughness, metalness, opacity.
    setting: [f64; 3],
    dirty: bool,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 55.,
            near: 0.1,
            far: 20.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0.8, 0.5, -1.5);
        s.tone_mapping = ToneMapping::Neutral;
        s.exposure = 0.7;
        let mut env = crate::environment::EnvironmentMap::from_hdr(
            &fetch(&format!("{ASSETS}/spot-skinning/lobe.hdr")).await?,
        )?;
        env.prefilter(r)?;
        s.environment = Some(Arc::new(env));
        s.background_environment = true;
        s.background_blur = 0.5;
        s.background_intensity = 1.;
        s.environment_intensity = 1.5;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 6.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(0.2, 0.6, 0.4);
        n.cast_shadow = true;
        n.shadow = Shadow {
            near: 0.1,
            far: 2.,
            extent: 0.5,
            map_size: Some(2048),
            radius: 8.,
            bias: -0.0005,
            ..Default::default()
        };
        let point = s.insert(NodeKind::Light(Light::Point {
            color: Color::from_hex(0x7b8cad),
            intensity: 1.,
            distance: 0.,
            decay: 2.,
        }));
        s.get_mut(point)?.position = Vector3::new(-0.3, -0.2, -0.2);
        let (a, b, i) = load_asset(&format!("{ASSETS}/watch/rolex.glb")).await?;
        let model = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        // gltf.scene.rotation.x = π / 4.
        let root = s.insert(NodeKind::Group);
        s.get_mut(root)?.quaternion = Quaternion::from_rotation_x(PI * 0.25);
        for &h in &model.roots {
            s.add(root, h)?;
        }
        // The page drops the vertex colours ( the base, strap and wheels carry COLOR_0 ).
        // Gold and Silver, the file's only metallic materials, are the ones the GUI
        // edits: every slot using them, as the page shares each material by name.
        let mut gold_silver = vec![];
        let mut glass = None;
        for &h in &model.meshes {
            let parent_name = s
                .get(h)?
                .parent()
                .and_then(|p| s.get(p).ok().map(|n| n.name.clone()))
                .unwrap_or_default();
            let node = s.get_mut(h)?;
            if parent_name != "glass" && parent_name != "floor" {
                node.cast_shadow = true;
                node.receive_shadow = true;
            }
            if let NodeKind::Mesh(m) = &mut node.kind {
                for (slot, material) in m.materials.iter_mut().enumerate() {
                    Arc::make_mut(material).properties_mut().vertex_colors = false;
                    if let Material::Standard(x)
                    | Material::Physical(MeshPhysicalMaterial { base: x, .. }) =
                        material.as_ref()
                        && x.metalness == 1.
                    {
                        gold_silver.push((h, slot));
                    }
                }
            }
            if parent_name == "glass" {
                glass = Some(h);
            }
        }
        let glass = glass.ok_or(Error::Invalid("rolex glass"))?;
        let mut m = MeshPhysicalMaterial::default();
        m.base.properties.color = Color::from_hex(0x020205);
        m.base.properties.transparent = true;
        m.base.properties.opacity = 0.8;
        m.base.properties.blending = Some(additive());
        m.base.metalness = 0.;
        m.base.roughness = 0.;
        m.iridescence = 0.3;
        m.clearcoat = 1.;
        if let NodeKind::Mesh(mesh) = &mut s.get_mut(glass)?.kind {
            mesh.materials = vec![Arc::new(Material::Physical(m))];
        }
        let mut hands = HashMap::new();
        for name in ["hour", "minute", "second", "mini_01", "mini_02", "mini_03"] {
            let h = model
                .nodes
                .iter()
                .copied()
                .find(|&h| s.get(h).is_ok_and(|n| n.name == name))
                .ok_or(Error::Invalid("rolex hand"))?;
            s.get_mut(h)?.matrix_auto_update = true;
            hands.insert(name, h);
        }
        let mut controls = Controls::new(None, (0.3, 10.), PI, true);
        controls.set_target(TARGET);
        controls.update(s, c)?;
        // moveCamera(): the tween starts from the controls' distance and angles.
        let offset = s.get(c)?.position - TARGET;
        let distance = offset.length();
        let start = (
            distance,
            (offset.y / distance).clamp(-1., 1.).acos(),
            offset.x.atan2(offset.z),
        );
        Ok(Self {
            controls,
            start,
            tweening: true,
            time: 0.,
            seeked: None,
            hands,
            gold_silver,
            glass,
            setting: [0.1, 1., 0.8],
            dirty: false,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// getTime(): the hands from the date.
    fn hands(&self, s: &mut Scene) -> Result<()> {
        let date = match self.seeked {
            Some(t) => {
                let base = js_sys::Date::new_with_year_month_day_hr_min_sec_milli(
                    2024, 0, 15, 10, 8, 30, 250,
                );
                js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(
                    base.get_time() + t * 1000.,
                ))
            }
            None => js_sys::Date::new_0(),
        };
        let mut hour = f64::from(date.get_hours());
        if hour >= 12. {
            hour -= 12.;
        }
        let day = f64::from(date.get_day()).min(30.);
        for (name, angle) in [
            ("hour", -hour * 30.),
            ("minute", -f64::from(date.get_minutes()) * 6.),
            ("second", -f64::from(date.get_seconds()) * 6.),
            ("mini_03", -day * 12.),
            ("mini_02", -f64::from(date.get_month()) * 30.),
            ("mini_01", -f64::from(date.get_milliseconds()) * 0.36),
        ] {
            s.get_mut(self.hands[name])?.quaternion = Quaternion::from_rotation_y(angle * TORAD);
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.dirty) {
            let [roughness, metalness, opacity] = self.setting;
            for &(h, slot) in &self.gold_silver {
                if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind
                    && let Some(material) = m.materials.get_mut(slot)
                    && let Material::Standard(m)
                    | Material::Physical(MeshPhysicalMaterial { base: m, .. }) =
                        Arc::make_mut(material)
                {
                    m.roughness = roughness;
                    m.metalness = metalness;
                }
            }
            if let NodeKind::Mesh(m) = &mut s.get_mut(self.glass)?.kind {
                Arc::make_mut(&mut m.materials[0]).properties_mut().opacity = opacity;
            }
        }
        // controls.update(), then TWEEN.update(): the tween places the camera until it completes.
        self.controls.update(s, c)?;
        if self.tweening {
            let t = self.seeked.unwrap_or(self.time);
            let k = (t * 1000. / 6000.).clamp(0., 1.);
            let e = k * (2. - k);
            let (d0, phi, theta0) = self.start;
            let distance = d0 + (1. - d0) * e;
            let theta = theta0 + (-PI * 0.2 - theta0) * e;
            s.get_mut(c)?.position = TARGET
                + Vector3::new(
                    distance * phi.sin() * theta.sin(),
                    distance * phi.cos(),
                    distance * phi.sin() * theta.cos(),
                );
            s.look_at(c, TARGET)?;
            if k >= 1. {
                self.tweening = false;
                self.controls.set_damping(Some(0.05));
            }
        }
        self.hands(s)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    /// controls.enabled is false until the tween completes.
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
        if self.tweening {
            return Ok(());
        }
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
    /// roughness, metalness, opacity.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if let Some(v) = self.setting.get_mut(index) {
            *v = f64::from(value);
            self.dirty = true;
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.seeked = Some(t);
    }
}
