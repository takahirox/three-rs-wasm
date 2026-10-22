//! An example's hooks, which the gallery's frame loop and its page's input
//! call through `expanded::Demo`. Each hook returns `None` ( or does nothing )
//! for an example that does not handle it.
#[allow(unused_variables, clippy::too_many_arguments)]
pub(super) trait Example {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        None
    }
    fn on_select(
        &mut self,
        scene: &mut crate::scene::Scene,
        cam: crate::scene::Object3D,
        x: f64,
        y: f64,
    ) -> Option<crate::Result<()>> {
        None
    }
    fn on_gpu_pointer(
        &mut self,
        r: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        cam: crate::scene::Object3D,
        x: f64,
        y: f64,
    ) -> Option<crate::Result<bool>> {
        None
    }
    fn on_pointer(&mut self, x: f64, y: f64) {}
    fn on_dragging(&mut self, value: bool) {}
    fn on_render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Option<crate::Result<bool>> {
        None
    }
    fn on_output_target(&self) -> Option<Option<&crate::renderer::RenderTarget>> {
        None
    }
    fn on_output_target_mut(&mut self) -> Option<Option<&mut crate::renderer::RenderTarget>> {
        None
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        None
    }
    fn on_audio_frame(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
    ) -> Option<crate::Result<Vec<f32>>> {
        None
    }
    fn on_audio_data(&mut self, bytes: &[u8]) -> Option<crate::Result<()>> {
        None
    }
    fn on_take_export(
        &mut self,
        renderer: &crate::renderer::Renderer,
    ) -> Option<Option<(String, Vec<u8>)>> {
        None
    }
    fn on_status(&self) -> Option<String> {
        None
    }
    fn on_key(&mut self, code: u32, down: bool) {}
    fn on_slider(&mut self, x: f64) {}
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        None
    }
    fn on_seek(&mut self, seconds: f64) {}
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        None
    }
    fn on_viewport(&mut self, index: usize, rectangle: [f64; 4]) -> Option<crate::Result<()>> {
        None
    }
    fn on_attach_canvases(
        &mut self,
        r: &crate::renderer::Renderer,
        canvases: js_sys::Array,
    ) -> Option<crate::Result<()>> {
        None
    }
    fn on_presented(&mut self, canvas: &web_sys::HtmlCanvasElement) -> crate::Result<()> {
        Ok(())
    }
    /// The MVP page's orbit input ( `BrowserApp::orbit` ).
    fn on_orbit(&mut self, dx: f64, dy: f64, zoom: f64) {}
    /// The gallery page's pointer drag and wheel ( `BrowserApp::gallery_input` )
    /// for an example that orbits its own viewer, before the gallery viewer.
    fn on_scene_input(
        &mut self,
        dx: f64,
        dy: f64,
        wheel: f64,
        dragging: bool,
    ) -> Option<crate::Result<()>> {
        None
    }
    /// The gallery viewer's zoom for this example: whether the wheel zooms,
    /// and the distance range.
    fn on_orbit_limits(&self) -> Option<(bool, f64, f64)> {
        None
    }
    fn on_wireframe(
        &mut self,
        scene: &mut crate::scene::Scene,
        enabled: bool,
    ) -> Option<crate::Result<()>> {
        None
    }
    fn on_morph(
        &mut self,
        scene: &mut crate::scene::Scene,
        weights: [f64; 2],
    ) -> Option<crate::Result<()>> {
        None
    }
    fn on_pmrem(
        &mut self,
        scene: &mut crate::scene::Scene,
        enabled: bool,
    ) -> Option<crate::Result<()>> {
        None
    }
    fn on_texture_transform(
        &mut self,
        scene: &mut crate::scene::Scene,
        values: &[f64],
    ) -> Option<crate::Result<()>> {
        None
    }
    fn on_animation_names(&self) -> Option<String> {
        None
    }
    fn on_select_animation(&mut self, index: usize) -> Option<crate::Result<()>> {
        None
    }
    fn on_animation_time(&mut self, index: usize, time: f64) -> Option<crate::Result<()>> {
        None
    }
    fn on_point_lights_controls(&mut self, amount: f64, speed: f64) {}
    fn on_gltf_view(&mut self, yaw: f64, pitch: f64, distance: f64) {}
    /// A glTF viewer's model switch: a new scene, its camera and the viewer,
    /// built apart from the page's state.
    fn on_load_model(
        &self,
        model: u32,
        environment: Option<std::sync::Arc<crate::environment::EnvironmentMap>>,
    ) -> Option<LoadModel> {
        None
    }
    /// A press on the canvas at `ndc`: whether it picked the example's object
    /// ( the MVP page's cube ).
    fn on_pick(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        ndc: crate::math::Vector2,
    ) -> Option<crate::Result<bool>> {
        None
    }
    /// Replace the example's object ( the MVP page's rebuild ): the rebuild count.
    fn on_rebuild(&mut self, scene: &mut crate::scene::Scene) -> Option<crate::Result<u32>> {
        None
    }
    #[cfg(feature = "tsl_procedural")]
    fn on_audio(&self) -> Option<crate::Result<&super::tsl_procedural::audio::Audio>> {
        None
    }
}

/// What `Example::on_load_model` builds: the scene, its camera and the example.
pub(super) type LoadModel = std::pin::Pin<
    Box<
        dyn std::future::Future<
                Output = crate::Result<(
                    crate::scene::Scene,
                    crate::scene::Object3D,
                    Box<dyn Example>,
                )>,
            >,
    >,
>;
