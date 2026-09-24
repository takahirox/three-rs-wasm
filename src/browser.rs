//! Rust-owned browser application used by the web validation suite.
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, raycast::*, renderer::*, scene::*,
};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen::{JsCast, prelude::*};
mod buffer_particles;
mod controls_attributes;
mod environment_materials;
mod expanded;
mod expanded_geometry_colors;
mod expanded_indexed;
mod expanded_lights;
mod expanded_lines;
mod expanded_morph_models;
mod expanded_triangles;
mod gallery;
mod gallery_scenes;
mod geometry_materials;
mod gltf_examples;
mod gltf_viewer;
mod helpers_formats;
mod interactive_objects;
mod interactive_scenes;
mod interactive_shaders;
mod material_textures;
mod models_modifiers;
mod picking_buffers;
mod point_clouds;
mod point_lights;
mod robot;
mod room_environment;
mod shader_geometry;
mod shapes;
mod stereo_loaders;
mod teapot_data;
mod terrain_loaders;
mod trackball_sprites;
mod tsl_compute;
mod tsl_environment;
mod tsl_examples;
mod tsl_extended;
mod tsl_filters;
mod tsl_lighting;
mod tsl_materials;
mod tsl_next;
mod tsl_particles;
mod tsl_passes;
mod tsl_primitives;
mod tsl_procedural;
mod tsl_surface;
mod tsl_viewport;
mod views_loaders;

// Demo assets live under web/ both locally and below a static hosting prefix.
fn asset_url(url: &str) -> Result<String> {
    let Some(path) = url.strip_prefix("/web/") else {
        return Ok(url.to_owned());
    };
    let base = web_sys::window()
        .and_then(|window| window.document())
        .ok_or(Error::Invalid("document"))?
        .base_uri()
        .map_err(|_| Error::Invalid("document base URI"))?
        .ok_or(Error::Invalid("document base URI"))?;
    let (prefix, _) = base
        .rsplit_once("/web/")
        .ok_or(Error::Invalid("demo must be served under web/"))?;
    Ok(format!("{prefix}/web/{path}"))
}

type AnimationCallback = Closure<dyn FnMut(f64)>;
struct State {
    timer: crate::time::Timer,
    renderer: Renderer,
    surface: wgpu::Surface<'static>,
    configuration: wgpu::SurfaceConfiguration,
    target: RenderTarget,
    scene: Scene,
    camera: Object3D,
    mesh: Object3D,
    canvas: web_sys::HtmlCanvasElement,
    frame: u32,
    animation: Option<AnimationCallback>,
    request: Option<i32>,
    paused: bool,
    rebuilds: u32,
    example: u32,
    rotation: Vector3,
    point_lights: Option<point_lights::PointLights>,
    gltf: Option<gltf_viewer::OrbitViewer>,
    load_generation: u64,
    gallery_scene: Option<gallery_scenes::GalleryScene>,
}
impl State {
    fn request_render(&mut self) {
        if self.request.is_none()
            && let Some(callback) = &self.animation
        {
            self.request = web_sys::window().and_then(|w| {
                w.request_animation_frame(callback.as_ref().unchecked_ref())
                    .ok()
            });
        }
    }
    fn render(&mut self, time: f64) -> Result<()> {
        self.timer.update();
        let _ = self
            .canvas
            .set_attribute("data-delta", &self.timer.get_delta().to_string());
        let width = self.canvas.width();
        let height = self.canvas.height();
        // Scene loaders can replace the camera without resizing the render target.
        // Synchronize its projection on the first frame as well as after resizes.
        if let NodeKind::Camera(Camera::Perspective(camera)) =
            &mut self.scene.get_mut(self.camera)?.kind
        {
            camera.aspect = width as f64 / height as f64;
        }
        if width != self.target.width || height != self.target.height {
            self.target.set_size(&self.renderer.device, width, height)?;
            self.configuration.width = width;
            self.configuration.height = height;
            self.surface
                .configure(&self.renderer.device, &self.configuration);
        }
        if let Some(demo) = &mut self.gallery_scene {
            demo.update(
                &mut self.scene,
                self.camera,
                self.timer.get_delta().min(0.1),
                !self.paused,
            )?;
        } else if let Some(viewer) = &self.gltf {
            viewer.update(&mut self.scene, self.camera)?;
        } else if let Some(demo) = &mut self.point_lights {
            if !self.paused {
                demo.time += self.timer.get_delta().min(0.1) * demo.speed;
            }
            demo.update(&mut self.scene, self.camera, self.mesh)?;
            let _ = self
                .canvas
                .set_attribute("data-demo-time", &demo.time.to_string());
        } else if !self.paused {
            self.scene.get_mut(self.mesh)?.quaternion = if self.example == 2 {
                self.rotation.x += 0.005;
                self.rotation.y += 0.01;
                Euler {
                    angles: self.rotation,
                    order: EulerOrder::XYZ,
                }
                .quaternion()
            } else {
                Quaternion::from_rotation_y(time * 0.0003)
            };
        }
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &mut self.gallery_scene {
            demo.prepare(
                &self.renderer,
                &mut self.scene,
                self.camera,
                width as f64 / height as f64,
            )?;
        }
        let handled =
            if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &mut self.gallery_scene {
                demo.render(&self.renderer, &mut self.scene, self.camera, &self.target)?
            } else {
                false
            };
        if !handled {
            self.renderer
                .render(&mut self.scene, self.camera, &self.target)?;
        }
        if self.example == 93 {
            self.frame += 1;
            let _ = self
                .canvas
                .set_attribute("data-frames", &self.frame.to_string());
            return Ok(());
        }
        let frame = self
            .surface
            .get_current_texture()
            .map_err(|e| Error::Gpu(e.to_string()))?;
        // Raw/encoded targets contain display values; the CRT example requests linear output.
        let format = if [
            16, 27, 28, 35, 45, 46, 50, 54, 55, 57, 58, 65, 70, 77, 90, 99, 101, 107, 111, 118,
            120, 126, 127, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167,
            168, 169, 170, 172, 173, 174, 175, 176, 177, 181, 182, 183, 184, 185, 186, 187, 188,
            189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205,
            206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222,
            223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237,
        ]
        .contains(&self.example)
        {
            self.configuration.format
        } else {
            self.configuration.format.add_srgb_suffix()
        };
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(format),
            ..Default::default()
        });
        let presentation = match &self.gallery_scene {
            Some(gallery_scenes::GalleryScene::Expanded(demo)) => {
                demo.output_target().unwrap_or(&self.target)
            }
            _ => &self.target,
        };
        if [50, 54, 55, 57, 58, 118, 120, 126, 127].contains(&self.example) {
            self.renderer.blit_premultiplied_srgb(
                presentation,
                &view,
                format,
                self.scene.exposure,
                self.scene.output_tone_mapping(),
            );
        } else {
            self.renderer.blit_with_tone_mapping(
                presentation,
                &view,
                format,
                self.scene.exposure,
                if presentation.options.encode_srgb {
                    ToneMapping::None
                } else {
                    self.scene.output_tone_mapping()
                },
            );
        }
        frame.present();
        self.frame += 1;
        let _ = self
            .canvas
            .set_attribute("data-frames", &self.frame.to_string());
        Ok(())
    }
}

/// The JavaScript bootstrap owns one app lifetime. Scene state, rendering,
/// animation and input handling all stay in Rust.
#[wasm_bindgen]
pub struct BrowserApp {
    state: Rc<RefCell<State>>,
    pointer: Closure<dyn FnMut(web_sys::PointerEvent)>,
}
#[wasm_bindgen]
impl BrowserApp {
    /// Request a frame after a canvas resize, including on-demand examples.
    pub fn request_render(&self) {
        self.state.borrow_mut().request_render();
    }
    /// Configure MSAA for matched-quality renderer comparisons (default unchanged).
    pub fn set_samples(&self, samples: u32) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let mut options = state.target.options.clone();
        options.samples = samples;
        state.target = RenderTarget::with_options(
            &state.renderer.device,
            state.canvas.width(),
            state.canvas.height(),
            options,
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.request_render();
        Ok(())
    }
    /// Presentation controls; all scene and deformation state remains in Rust.
    pub fn point_lights_controls(&self, paused: bool, amount: f64, speed: f64) {
        let mut state = self.state.borrow_mut();
        state.paused = paused;
        if let Some(demo) = &mut state.point_lights {
            if amount.is_finite() {
                demo.amount = amount.clamp(0.0, 3.0);
            }
            if speed.is_finite() {
                demo.speed = speed.clamp(0.0, 2.0);
            }
        }
    }
    pub async fn load_model(&self, example: u32) -> std::result::Result<bool, JsValue> {
        if ![4, 5].contains(&example) {
            return Err(JsValue::from_str("unknown glTF example"));
        }
        let (generation, environment) = {
            let mut state = self.state.borrow_mut();
            state.load_generation += 1;
            (state.load_generation, state.scene.environment.clone())
        };
        let mut scene = Scene::new();
        let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
            PerspectiveCamera::default(),
        )));
        let placeholder = scene.insert(NodeKind::Group);
        let viewer =
            gltf_viewer::OrbitViewer::create(&mut scene, camera, placeholder, example, environment)
                .await;
        let mut state = self.state.borrow_mut();
        if state.load_generation != generation {
            return Ok(false);
        }
        let viewer = viewer.map_err(|e| JsValue::from_str(&e.to_string()))?;
        if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene
            .get_mut(camera)
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .kind
        {
            c.aspect = state.canvas.width() as f64 / state.canvas.height() as f64;
        }
        scene.environment_intensity = state.scene.environment_intensity;
        scene.background_environment = state.scene.background_environment;
        scene.exposure = state.scene.exposure;
        scene.background_blur = state.scene.background_blur;
        scene.environment_rotation = state.scene.environment_rotation;
        state.scene = scene;
        state.camera = camera;
        state.gltf = Some(viewer);
        state.example = example;
        state.renderer.collect_resources();
        Ok(true)
    }
    pub async fn load_environment(&self, url: String) -> std::result::Result<bool, JsValue> {
        let generation = {
            let mut state = self.state.borrow_mut();
            state.load_generation += 1;
            state.load_generation
        };
        let image = async {
            let bytes = gltf_viewer::fetch(&url).await?;
            crate::environment::EnvironmentMap::from_hdr(&bytes)
        }
        .await;
        let mut state = self.state.borrow_mut();
        if state.load_generation != generation {
            return Ok(false);
        }
        let image = image.map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.scene.environment = Some(Arc::new(image));
        state.renderer.collect_resources();
        Ok(true)
    }
    pub fn gltf_controls(
        &self,
        exposure: f64,
        intensity: f64,
        rotation: f64,
        blur: f64,
        background: bool,
    ) {
        if ![exposure, intensity, rotation, blur]
            .iter()
            .all(|v| v.is_finite())
        {
            return;
        }
        let mut state = self.state.borrow_mut();
        state.scene.exposure = exposure.max(0.0);
        state.scene.environment_intensity = intensity.max(0.0);
        state.scene.environment_rotation = rotation;
        state.scene.background_blur = blur.clamp(0.0, 1.0);
        state.scene.background_environment = background;
    }
    pub fn gallery_status(&self) -> String {
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) =
            &self.state.borrow().gallery_scene
        {
            demo.status()
        } else {
            String::new()
        }
    }
    pub fn gallery_dragging(&self, value: bool) {
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) =
            &mut self.state.borrow_mut().gallery_scene
        {
            demo.dragging(value);
        }
    }
    pub fn gallery_canvases(&self, canvases: js_sys::Array) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let State {
            renderer,
            gallery_scene,
            ..
        } = &mut *state;
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = gallery_scene {
            demo.attach_canvases(renderer, canvases)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        state.request_render();
        Ok(())
    }
    pub fn gallery_viewport(
        &self,
        index: usize,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &mut state.gallery_scene {
            demo.viewport(index, [x, y, width, height])
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            state.request_render();
        }
        Ok(())
    }
    pub fn gallery_input(
        &self,
        dx: f64,
        dy: f64,
        wheel: f64,
        dragging: bool,
    ) -> std::result::Result<(), JsValue> {
        if ![dx, dy, wheel].iter().all(|v| v.is_finite()) {
            return Ok(());
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        let height = state.canvas.client_height().max(1) as f64;
        let State {
            scene,
            camera,
            gallery_scene,
            ..
        } = &mut *state;
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = gallery_scene {
            return demo
                .input(scene, *camera, dx, dy, wheel, false, height)
                .map_err(|e| JsValue::from_str(&e.to_string()));
        }
        if let Some(demo) = gallery_scene {
            demo.input(scene, *camera, dx, dy, wheel, dragging)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        Ok(())
    }
    pub fn gallery_pan(&self, dx: f64, dy: f64) -> std::result::Result<(), JsValue> {
        if !dx.is_finite() || !dy.is_finite() {
            return Err(JsValue::from_str("invalid pan"));
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        let height = state.canvas.client_height().max(1) as f64;
        let State {
            scene,
            camera,
            gallery_scene,
            ..
        } = &mut *state;
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = gallery_scene {
            demo.input(scene, *camera, dx, dy, 0.0, true, height)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        } else {
            Ok(())
        }
    }
    pub fn gallery_select(&self, x: f64, y: f64) -> std::result::Result<(), JsValue> {
        if !x.is_finite() || !y.is_finite() {
            return Err(JsValue::from_str("pointer coordinates"));
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        let State {
            scene,
            camera,
            gallery_scene,
            ..
        } = &mut *state;
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = gallery_scene {
            demo.select(scene, *camera, x, y)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        Ok(())
    }
    /// Drawing-canvas pointer input: 0 down, 1 move, 2 up or leave (offset coordinates).
    pub fn gallery_draw(&self, kind: u32, x: f64, y: f64) -> std::result::Result<(), JsValue> {
        if !x.is_finite() || !y.is_finite() {
            return Err(JsValue::from_str("drawing coordinates"));
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &mut state.gallery_scene {
            demo.draw(kind, x, y)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        Ok(())
    }
    /// Keyboard state for examples that track modifier keys (keyCode, pressed).
    pub fn gallery_key(&self, code: u32, down: bool) {
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) =
            &mut self.state.borrow_mut().gallery_scene
        {
            demo.key(code, down);
        }
    }
    /// Comparison slider position in CSS pixels.
    pub fn gallery_slider(&self, x: f64) {
        if !x.is_finite() {
            return;
        }
        let mut state = self.state.borrow_mut();
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &mut state.gallery_scene {
            demo.slider(x);
        }
        state.request_render();
    }
    pub fn gallery_pointer(&self, x: f64, y: f64) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        let mut state = self.state.borrow_mut();
        let width = state.canvas.client_width() as f64;
        let height = state.canvas.client_height() as f64;
        let is_rtt = [39, 55, 56].contains(&state.example);
        let mut redraw = false;
        let State {
            renderer,
            scene,
            camera,
            gallery_scene,
            canvas,
            ..
        } = &mut *state;
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = gallery_scene {
            match demo.gpu_pointer(renderer, scene, *camera, x, y) {
                Ok(true) => redraw = true,
                Ok(false) => {}
                Err(e) => {
                    let _ = canvas.set_attribute("data-error", &e.to_string());
                }
            }
            if is_rtt {
                demo.pointer(x, y);
            } else {
                demo.pointer(x * width / 2.0, -y * height / 2.0);
            }
        } else if let Some(demo) = gallery_scene {
            demo.pointer(x, y);
        }
        // On-demand scenes redraw when the pointer moves something.
        if redraw {
            state.request_render();
        }
    }
    /// Animation clips available in the active gallery scene.
    pub fn animation_names(&self) -> String {
        if let Some(gallery_scenes::GalleryScene::Robot(robot)) = &self.state.borrow().gallery_scene
        {
            serde_json::to_string(
                &robot
                    .mixer
                    .actions
                    .iter()
                    .map(|a| &a.clip.name)
                    .collect::<Vec<_>>(),
            )
            .unwrap_or_else(|_| "[]".into())
        } else {
            "[]".into()
        }
    }
    pub fn select_animation(&self, index: usize) -> std::result::Result<(), JsValue> {
        if let Some(gallery_scenes::GalleryScene::Robot(robot)) =
            &mut self.state.borrow_mut().gallery_scene
        {
            robot
                .select(index)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        Ok(())
    }
    /// Freeze a selected clip for deterministic browser/reference comparisons.
    pub fn animation_time(&self, index: usize, time: f64) -> std::result::Result<(), JsValue> {
        if !time.is_finite() || time < 0.0 {
            return Err(JsValue::from_str("invalid animation time"));
        }
        let mut state = self.state.borrow_mut();
        state.paused = true;
        if let Some(gallery_scenes::GalleryScene::Robot(robot)) = &mut state.gallery_scene {
            if index >= robot.mixer.actions.len() {
                return Err(JsValue::from_str("invalid animation index"));
            }
            for (i, action) in robot.mixer.actions.iter_mut().enumerate() {
                action
                    .fade_to(if i == index { 1.0 } else { 0.0 }, 0.0)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                action.time = time;
                action.looping = crate::animation::LoopMode::Once;
            }
            robot.selected = index;
        }
        Ok(())
    }
    /// Serialize the live scene for debugging and upstream behavioral comparisons.
    pub fn transfer_counts(&self) -> String {
        serde_json::to_string(&self.state.borrow().renderer.transfer_counts())
            .expect("transfer counts")
    }
    pub fn gallery_wireframe(&self, enabled: bool) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let State {
            scene,
            gallery_scene,
            ..
        } = &mut *state;
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = gallery_scene {
            demo.wireframe(scene, enabled)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        } else {
            Err(JsValue::from_str("indexed example required"))
        }
    }
    pub fn gallery_morph(&self, spherify: f64, twist: f64) -> std::result::Result<(), JsValue> {
        if ![spherify, twist]
            .iter()
            .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        {
            return Err(JsValue::from_str("invalid morph weights"));
        }
        let mut state = self.state.borrow_mut();
        let State {
            scene,
            gallery_scene,
            ..
        } = &mut *state;
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = gallery_scene {
            demo.morph(scene, [spherify, twist])
                .map_err(|e| JsValue::from_str(&e.to_string()))
        } else {
            Err(JsValue::from_str("morph example required"))
        }
    }
    /// Freeze a procedural gallery scene at a reproducible elapsed time.
    pub fn gallery_time(&self, seconds: f64) -> std::result::Result<(), JsValue> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err(JsValue::from_str("invalid gallery time"));
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &mut state.gallery_scene {
            demo.seek(seconds);
            state.paused = true;
            Ok(())
        } else {
            Err(JsValue::from_str(
                "scene does not support procedural seeking",
            ))
        }
    }
    pub fn audio_play(&self) -> std::result::Result<bool, JsValue> {
        let state = self.state.borrow();
        let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &state.gallery_scene else {
            return Err(JsValue::from_str("not an audio example"));
        };
        demo.audio()
            .and_then(|a| a.play(&state.renderer))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    pub fn audio_result(&self) -> std::result::Result<Vec<f32>, JsValue> {
        let state = self.state.borrow();
        let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &state.gallery_scene else {
            return Err(JsValue::from_str("not an audio example"));
        };
        demo.audio()
            .and_then(|a| a.take().unwrap_or_else(|| Ok(Vec::new())))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    pub fn audio_spectrum(&self, bytes: Vec<u8>) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &state.gallery_scene else {
            return Err(JsValue::from_str("not an audio example"));
        };
        demo.audio()
            .and_then(|a| a.spectrum(&state.renderer, &bytes))
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.request_render();
        Ok(())
    }
    pub fn tsl_parameter(&self, index: usize, value: f32) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        if let Some(gallery_scenes::GalleryScene::Expanded(demo)) = &mut state.gallery_scene {
            demo.tsl_parameter(index, value)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        state.request_render();
        Ok(())
    }
    pub fn gallery_sheen(&self, value: f64) -> std::result::Result<(), JsValue> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(JsValue::from_str("invalid sheen"));
        }
        let mut state = self.state.borrow_mut();
        if state.example != 31 {
            return Err(JsValue::from_str("Sheen example required"));
        }
        let roots = state.scene.roots().to_vec();
        for root in roots {
            for h in state
                .scene
                .traverse(root, true)
                .map_err(|e| JsValue::from_str(&e.to_string()))?
            {
                if let NodeKind::Mesh(mesh) = &mut state
                    .scene
                    .get_mut(h)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?
                    .kind
                {
                    for m in &mut mesh.materials {
                        if let Material::Physical(p) = Arc::make_mut(m) {
                            p.sheen = value;
                        }
                    }
                }
            }
        }
        state.request_render();
        Ok(())
    }
    pub fn scene_json(&self) -> std::result::Result<String, JsValue> {
        self.state
            .borrow()
            .scene
            .to_json()
            .map(|v| v.to_string())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    pub fn texture_transform(&self, values: Vec<f64>) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let State {
            scene,
            gallery_scene,
            ..
        } = &mut *state;
        if let Some(demo) = gallery_scene {
            demo.texture_transform(scene, &values)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        Ok(())
    }
    pub fn background_intensity(&self, value: f64) {
        if value.is_finite() {
            self.state.borrow_mut().scene.background_intensity = value.max(0.0);
        }
    }
    pub fn gallery_pmrem(&self, enabled: bool) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let State {
            scene,
            gallery_scene,
            ..
        } = &mut *state;
        if let Some(demo) = gallery_scene {
            demo.pmrem(scene, enabled)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
        }
        Ok(())
    }
    pub fn resource_counts(&self) -> Vec<f64> {
        let (resident, uploads, filters) = self.state.borrow().renderer.resource_counts();
        vec![resident as f64, uploads as f64, filters as f64]
    }
    pub fn gltf_view(
        &self,
        yaw: f64,
        pitch: f64,
        distance: f64,
        exposure: f64,
        rotation: f64,
        blur: f64,
    ) {
        if ![yaw, pitch, distance, exposure, rotation, blur]
            .iter()
            .all(|v| v.is_finite())
            || distance <= 0.0
        {
            return;
        }
        let mut state = self.state.borrow_mut();
        if let Some(viewer) = &mut state.gltf {
            viewer.fixture(yaw, pitch, distance);
        }
        state.scene.exposure = exposure.max(0.0);
        state.scene.environment_rotation = rotation;
        state.scene.background_blur = blur.clamp(0.0, 1.0);
        state.request_render();
    }
    pub fn orbit(&self, dx: f64, dy: f64, zoom: f64) {
        if dx.is_finite()
            && dy.is_finite()
            && zoom.is_finite()
            && let Some(viewer) = &mut self.state.borrow_mut().gltf
        {
            viewer.orbit(dx, dy, zoom);
        }
        if dx.is_finite()
            && dy.is_finite()
            && zoom.is_finite()
            && let Some(demo) = &mut self.state.borrow_mut().point_lights
        {
            demo.orbit(dx, dy, zoom);
        }
    }
    /// Rebuild a small scene fragment, exercising owned geometry replacement,
    /// groups/draw ranges, attribute edits and stale-handle protection.
    pub fn rebuild(&self) -> std::result::Result<(), JsValue> {
        let result = (|| -> Result<()> {
            let mut state = self.state.borrow_mut();
            let original = state.mesh;
            let mut geometry = PlaneGeometry::build(2.0, 2.0, 1, 1)?;
            geometry.clear_groups();
            geometry.add_group(0, 3, 0);
            geometry.add_group(3, 3, 1);
            state.rebuilds += 1;
            geometry.set_draw_range(
                0,
                Some(if state.rebuilds.is_multiple_of(2) {
                    6
                } else {
                    3
                }),
            );
            if let Some(position) = geometry.attributes.get_mut("position") {
                position.set_component(0, 0, -0.8)?;
            }
            let mut first = Material::default();
            first.properties_mut().color = Color::from_hex(0xff3366);
            let mut second = Material::default();
            second.properties_mut().color = Color::from_hex(0x33ccff);
            let replacement = state.scene.insert(NodeKind::Mesh(Mesh {
                geometry: Arc::new(geometry),
                materials: vec![Arc::new(first), Arc::new(second)],
            }));
            let group = state.scene.insert(NodeKind::Group);
            state.scene.add(group, replacement)?;
            state.scene.remove_from_parent(replacement)?;
            state.scene.dispose(group)?;
            state.scene.dispose(original)?;
            if state.scene.get(original).is_ok() {
                return Err(Error::Invalid("stale handle retained"));
            }
            state.mesh = replacement;
            state.paused = true;
            state.target = RenderTarget::new(
                &state.renderer.device,
                state.canvas.width(),
                state.canvas.height(),
            )?;
            state
                .canvas
                .set_attribute("data-rebuilt", "true")
                .map_err(|e| Error::Asset(format!("{e:?}")))?;
            let _ = state
                .canvas
                .set_attribute("data-rebuilds", &state.rebuilds.to_string());
            Ok(())
        })();
        result.map_err(|e| JsValue::from_str(&e.to_string()))
    }
    #[wasm_bindgen(js_name = create)]
    pub async fn new(
        canvas: web_sys::HtmlCanvasElement,
        example: u32,
        animate: bool,
    ) -> std::result::Result<BrowserApp, JsValue> {
        async fn create(
            canvas: web_sys::HtmlCanvasElement,
            example: u32,
            animate: bool,
        ) -> Result<BrowserApp> {
            let renderer = Renderer::new().await?;
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::BROWSER_WEBGPU,
                ..Default::default()
            });
            let surface = instance
                .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
                .map_err(|e| Error::Gpu(e.to_string()))?;
            let mut configuration = surface
                .get_default_config(&renderer.adapter, canvas.width(), canvas.height())
                .ok_or(Error::Gpu("surface configuration unavailable".into()))?;
            if [46, 50, 54, 55, 57].contains(&example) {
                configuration.alpha_mode = wgpu::CompositeAlphaMode::PreMultiplied;
            }
            configuration.view_formats = vec![configuration.format.add_srgb_suffix()];
            surface.configure(&renderer.device, &configuration);
            let target = RenderTarget::with_options(
                &renderer.device,
                canvas.width(),
                canvas.height(),
                if example >= 4 {
                    RenderTargetOptions {
                        samples: if [
                            7, 8, 11, 12, 24, 25, 27, 36, 39, 40, 41, 42, 43, 44, 45, 46, 47, 50,
                            52, 62, 63, 67, 70, 71, 72, 73, 74, 76, 77, 78, 80, 84, 85, 86, 87, 88,
                            91, 92, 93, 95, 97, 99, 100, 101, 106, 107, 111, 112, 113, 114, 115,
                            117, 120, 121, 135, 138, 141, 142, 144, 145, 146, 147, 158, 159, 160,
                            161, 163, 164, 166, 167, 170, 171, 178, 179, 180, 182, 183, 184, 187,
                            190, 197, 200, 203, 204, 205, 211, 213, 216, 218, 230, 234,
                        ]
                        .contains(&example)
                        {
                            1
                        } else {
                            4
                        },
                        encode_srgb: [
                            16, 28, 90, 154, 155, 156, 157, 175, 176, 181, 185, 186, 188, 190, 191,
                            193, 194, 195, 196, 197, 198, 199, 201, 202, 203, 204, 205, 206, 207,
                            208, 209, 210, 212, 214, 215, 217, 218, 219, 220, 221, 222, 224, 225,
                            226, 227, 228, 229, 230, 231, 233, 234, 235, 236, 237,
                        ]
                        .contains(&example),
                        format: if [
                            16, 27, 28, 90, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164,
                            165, 166, 167, 168, 169, 170, 172, 173, 174, 175, 176, 177, 181, 182,
                            183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196,
                            197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
                            211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224,
                            225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237,
                        ]
                        .contains(&example)
                        {
                            wgpu::TextureFormat::Rgba8Unorm
                        } else {
                            wgpu::TextureFormat::Rgba16Float
                        },
                        ..Default::default()
                    }
                } else {
                    Default::default()
                },
            )?;
            let mut scene = Scene::new();
            scene.background = Color::from_hex(0x102030);
            let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                aspect: canvas.width() as f64 / canvas.height() as f64,
                ..Default::default()
            })));
            scene.get_mut(camera)?.position.z = 5.0;
            let mut material = Material::default();
            material.properties_mut().color = Color::from_hex(0xee6633);
            let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(BoxGeometry::build(1.5, 1.5, 1.5)?),
                Arc::new(material),
            )));
            let mut point_lights = None;
            let mut gltf = None;
            let mut gallery_scene = None;
            if (7..=237).contains(&example) {
                gallery_scene = Some(
                    gallery_scenes::GalleryScene::create(
                        &mut scene, camera, mesh, example, &renderer,
                    )
                    .await?,
                );
            } else if example == 6 {
                gltf = Some(gallery::pmrem_grid(&mut scene, camera, mesh).await?);
                let _ = canvas.set_attribute("data-meshes", "30");
            } else if example == 4 || example == 5 {
                let viewer =
                    gltf_viewer::OrbitViewer::create(&mut scene, camera, mesh, example, None)
                        .await?;
                let _ = canvas.set_attribute("data-triangles", &viewer.triangles.to_string());
                let _ = canvas.set_attribute("data-meshes", &viewer.meshes.to_string());
                gltf = Some(viewer);
            } else if example == 3 {
                let mut demo = point_lights::PointLights::create(
                    &mut scene,
                    camera,
                    mesh,
                    canvas.width() as f64 / canvas.height() as f64,
                    &renderer,
                )
                .await?;
                if !animate {
                    demo.time = 6.0;
                }
                point_lights = Some(demo);
            } else if example == 1 {
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                        left: -3.0,
                        right: 3.0,
                        top: 3.0,
                        bottom: -3.0,
                        ..Default::default()
                    }));
                scene.get_mut(mesh)?.position.x = -1.1;
                let group = scene.insert(NodeKind::Group);
                scene.get_mut(group)?.position = Vector3::new(0.8, 0.6, 0.0);
                let texture = Arc::new(Texture::load(&asset_url("/web/checker.png")?).await?);
                let mut standard = MeshStandardMaterial::default();
                standard.properties.map = Some(texture);
                standard.roughness = 0.7;
                let sphere = scene.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(SphereGeometry::build(0.75, 24, 16)?),
                    Arc::new(Material::Standard(standard)),
                )));
                scene.add(group, sphere)?;
                scene.insert(NodeKind::Light(Light::Ambient {
                    color: Color::WHITE,
                    intensity: 0.3,
                }));
                let directional = scene.insert(NodeKind::Light(Light::Directional {
                    color: Color::WHITE,
                    intensity: 2.0,
                    target: Vector3::ZERO,
                }));
                scene.get_mut(directional)?.position = Vector3::new(2.0, 3.0, 4.0);
                let point = scene.insert(NodeKind::Light(Light::Point {
                    color: Color::from_hex(0xff8040),
                    intensity: 10.0,
                    distance: 0.0,
                    decay: 2.0,
                }));
                scene.get_mut(point)?.position = Vector3::new(-2.0, 1.0, 2.0);
                let mut line_geometry = BufferGeometry::default();
                line_geometry.set_from_points(&[
                    Vector3::new(-2.0, -1.5, 0.0),
                    Vector3::new(0.0, -0.9, 0.0),
                    Vector3::new(2.0, -1.5, 0.0),
                ])?;
                let mut line_material = LineBasicMaterial::default();
                line_material.properties.color = Color::from_hex(0x44ff88);
                scene.insert(NodeKind::Line(Line {
                    geometry: Arc::new(line_geometry.clone()),
                    material: Arc::new(Material::Line(line_material)),
                    segments: false,
                }));
                let mut points_material = PointsMaterial::default();
                points_material.properties.color = Color::from_hex(0xffffff);
                points_material.size_attenuation = false;
                let points = scene.insert(NodeKind::Points(Points {
                    geometry: Arc::new(line_geometry),
                    material: Arc::new(Material::Points(points_material)),
                }));
                scene.get_mut(points)?.position.y = -0.4;
            } else if example == 2 {
                // Three.js r186 examples/webgl_geometry_cube.html, ported to WebGPU.
                scene.background = Color::BLACK;
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 70.0,
                        aspect: canvas.width() as f64 / canvas.height() as f64,
                        near: 0.1,
                        far: 100.0,
                        ..Default::default()
                    }));
                scene.get_mut(camera)?.position.z = 2.0;
                let mut material = Material::default();
                material.properties_mut().map = Some(Arc::new(
                    Texture::load(&asset_url("/web/crate.gif")?).await?,
                ));
                scene.get_mut(mesh)?.kind = NodeKind::Mesh(Mesh::new(
                    Arc::new(BoxGeometry::build(1.0, 1.0, 1.0)?),
                    Arc::new(material),
                ));
                if !animate {
                    scene.get_mut(mesh)?.quaternion = Euler {
                        angles: Vector3::new(0.4, 0.7, 0.0),
                        order: EulerOrder::XYZ,
                    }
                    .quaternion();
                }
            } else if example != 0 {
                return Err(Error::Invalid("example id"));
            }
            let mut timer = crate::time::Timer::default();
            timer.connect(
                web_sys::window()
                    .and_then(|w| w.document())
                    .ok_or(Error::Asset("document unavailable".into()))?,
            )?;
            let state = Rc::new(RefCell::new(State {
                timer,
                renderer,
                surface,
                configuration,
                target,
                scene,
                camera,
                mesh,
                canvas: canvas.clone(),
                frame: 0,
                animation: None,
                request: None,
                paused: !animate,
                rebuilds: 0,
                example,
                rotation: Vector3::ZERO,
                point_lights,
                gltf,
                load_generation: 0,
                gallery_scene,
            }));
            let weak = Rc::downgrade(&state);
            let animation = Closure::wrap(Box::new(move |time: f64| {
                if let Some(state) = weak.upgrade() {
                    let mut state = state.borrow_mut();
                    state.request = None;
                    if let Err(error) = state.render(time) {
                        let _ = state.canvas.set_attribute("data-error", &error.to_string());
                        return;
                    }
                    // These official static scenes render only on load, input and resize.
                    if !([
                        16, 28, 38, 153, 156, 157, 193, 195, 206, 213, 220, 224, 226, 235, 236, 237,
                    ]
                    .contains(&state.example)
                        || state.paused && state.example >= 39)
                    {
                        state.request_render();
                    }
                }
            }) as Box<dyn FnMut(f64)>);
            state.borrow_mut().animation = Some(animation);
            let weak = Rc::downgrade(&state);
            let pointer = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                if let Some(state) = weak.upgrade() {
                    let mut state = state.borrow_mut();
                    if state.point_lights.is_some()
                        || state.gltf.is_some()
                        || state.gallery_scene.is_some()
                    {
                        return;
                    }
                    let ndc = Vector2::new(
                        event.offset_x() as f64 / state.canvas.client_width() as f64 * 2.0 - 1.0,
                        1.0 - event.offset_y() as f64 / state.canvas.client_height() as f64 * 2.0,
                    );
                    let mut raycaster = Raycaster::default();
                    let result = (|| -> Result<bool> {
                        state.scene.update()?;
                        let (camera, world) = state.scene.camera(state.camera)?;
                        raycaster.set_from_camera(ndc, camera, world)?;
                        Ok(!raycaster
                            .intersect_object(&state.scene, state.mesh, false)?
                            .is_empty())
                    })();
                    match result {
                        Ok(selected) => {
                            state.paused = selected;
                            let _ = state.canvas.set_attribute(
                                "data-selected",
                                if selected { "true" } else { "false" },
                            );
                        }
                        Err(error) => {
                            let _ = state.canvas.set_attribute("data-error", &error.to_string());
                        }
                    }
                }
            }) as Box<dyn FnMut(web_sys::PointerEvent)>);
            canvas
                .add_event_listener_with_callback("pointerdown", pointer.as_ref().unchecked_ref())
                .map_err(|e| Error::Asset(format!("{e:?}")))?;
            {
                let mut s = state.borrow_mut();
                s.request = Some(
                    web_sys::window()
                        .ok_or(Error::Asset("window unavailable".into()))?
                        .request_animation_frame(
                            s.animation
                                .as_ref()
                                .expect("animation installed")
                                .as_ref()
                                .unchecked_ref(),
                        )
                        .map_err(|e| Error::Asset(format!("{e:?}")))?,
                );
            }
            Ok(BrowserApp { state, pointer })
        }
        create(canvas, example, animate)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
impl Drop for BrowserApp {
    fn drop(&mut self) {
        let mut s = self.state.borrow_mut();
        if let Some(id) = s.request.take()
            && let Some(window) = web_sys::window()
        {
            let _ = window.cancel_animation_frame(id);
        }
        let _ = s.canvas.remove_event_listener_with_callback(
            "pointerdown",
            self.pointer.as_ref().unchecked_ref(),
        );
        s.animation = None;
        s.renderer.device.destroy();
    }
}
