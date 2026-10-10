//! webgl_loader_gltf_progressive_lod: three Needle Cloud models under a fogged
//! quarry environment, streamed by @needle-tools/gltf-progressive 3.2.0.
//!
//! The page loads each model's low-resolution glTF ( Draco, KTX2 and WebP,
//! with a NEEDLE_progressive extension on every mesh and texture ). After
//! every render the package's LODsManager walks the render list. For each
//! mesh it projects the bounds to a screen coverage and picks a mesh LOD from
//! the primitive densities ( 200,000 triangles per screen ) and a texture LOD
//! from the pixel height. The chosen files are fetched through a 50-slot
//! PromiseQueue and swapped in: geometries per mesh, textures per material
//! slot. `progressive.rs` ports that manager operation by operation.
use super::controls_attributes::{Controls, camera_state};
use crate::animation::AnimationMixer;
use crate::environment::EnvironmentMap;
use crate::{Error, Result, camera::*, math::*, renderer::*, scene::*};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;

mod progressive;
use progressive::{Loaded, Lods};

/// The page's three loads: the world, the airship and the knight.
const MODELS: [(&str, f64, [f64; 3], f64); 3] = [
    (
        "https://cloud.needle.tools/-/assets/Z23hmXBZ2sPRdk-world/file",
        0.1,
        [0., 0., 0.],
        0.,
    ),
    (
        "https://cloud.needle.tools/-/assets/Z23hmXBZnlceI-ZnlceI-world/file",
        0.0005,
        [1.6, 6., 7.],
        PI * 1.4,
    ),
    (
        "https://cloud.needle.tools/-/assets/Z23hmXBZ21QnG-Z21QnG-product/file",
        0.5,
        [2., 5.15, 2.3],
        PI,
    ),
];
const ENVIRONMENT: &str = "/web/gallery/assets/draco-variants/quarry_01_1k.hdr";

pub(super) struct Demo {
    controls: Controls,
    mixer: AnimationMixer,
    lods: Lods,
    /// Each model as it loads, in the page's order.
    slots: Vec<Rc<RefCell<Option<Result<Loaded>>>>>,
    airship: Option<Object3D>,
    /// THREE.Timer on the example clock ( ms ) and the page's accumulated time.
    now: f64,
    previous: f64,
    time: f64,
    pending: bool,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 0.1,
            far: 40.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-9., 2., -13.);
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 1.;
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x131055),
            near: 15.,
            far: 50.,
        });
        // HDRLoader's callback: the background colour and the rotated environment.
        let mut env = EnvironmentMap::from_hdr(&super::gltf_viewer::fetch(ENVIRONMENT).await?)?;
        env.prefilter(r)?;
        s.background = Color::from_hex(0x192022);
        s.background_blur = 0.5;
        s.environment = Some(Arc::new(env));
        s.environment_rotation = PI / -2.;
        let mut controls = Controls::new(None, (0.1, 20.), PI, true);
        controls.set_target(Vector3::new(-1., 2.1, 0.));
        controls.update(s, c)?;
        let slots = MODELS
            .iter()
            .map(|&(url, ..)| {
                let slot = Rc::new(RefCell::new(None));
                let target = slot.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    *target.borrow_mut() = Some(progressive::load_model(url).await);
                });
                slot
            })
            .collect();
        Ok(Self {
            controls,
            mixer: AnimationMixer::default(),
            lods: Lods::default(),
            slots,
            airship: None,
            now: 0.,
            previous: 0.,
            time: 0.,
            pending: false,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
            self.pending = true;
        }
        Ok(())
    }
    /// The loads' callbacks as they finish ( scale, placement, the mixer's
    /// clip actions ), then animate()'s timer, mixer and airship bob.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        for (i, slot) in self.slots.iter().enumerate() {
            let Some(result) = slot.borrow_mut().take() else {
                continue;
            };
            let loaded = result?;
            let instance = loaded.gltf.clone().instantiate(s)?;
            let (_, scale, position, rotation) = MODELS[i];
            let group = s.insert(NodeKind::Group);
            {
                let n = s.get_mut(group)?;
                n.scale = Vector3::splat(scale);
                n.position = Vector3::from_array(position);
                n.quaternion = Quaternion::from_rotation_y(rotation);
            }
            for &root in &instance.roots {
                s.add(group, root)?;
            }
            for clip in &instance.clips {
                self.mixer.play(clip.clone())?;
            }
            if i == 1 {
                self.airship = Some(group);
            }
            self.lods.register(s, group, &loaded, &instance)?;
        }
        if std::mem::take(&mut self.pending) {
            // timer.update(); dt = timer.getDelta(); time += dt.
            let dt = (self.now - self.previous) / 1000.;
            self.previous = self.now;
            self.time += dt;
            self.mixer.update(s, dt)?;
            if let Some(airship) = self.airship {
                s.get_mut(airship)?.position.y += crate::bounce::trig::sin(self.time) * 0.002;
            }
            self.lods.pending_frame = true;
        }
        // The PromiseQueue's timer and the finished loads' continuations.
        self.lods.tick(s)
    }
    /// renderer.render( scene, camera ), then the LODsManager's onAfterRender.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if !std::mem::take(&mut self.lods.pending_frame) {
            // A redraw outside the page's animation loop: no LOD update.
            r.render(s, c, out)?;
            return Ok(true);
        }
        s.update()?;
        self.lods.before_render(s, c)?;
        r.render(s, c, out)?;
        let height = out.height as f64 / device_pixel_ratio();
        self.lods.after_render(s, c, height, self.now)?;
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body.set_attribute("data-lod-state", &self.lods.summary(s));
        }
        Ok(true)
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
        if dx == 0. && dy == 0. && wheel == 0. {
            return Ok(());
        }
        let state = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &state, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&state, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("progressive LOD parameter"))
    }
    /// The example clock: the fixture renders each frame at a time.
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
        self.pending = true;
    }
}

fn device_pixel_ratio() -> f64 {
    web_sys::window().map_or(1., |w| w.device_pixel_ratio())
}
