//! An example module's hooks, which the gallery's frame and input handling
//! call through `expanded::Demo` ( its Content::Module ). Each hook returns
//! `None` ( or does nothing ) for an example that does not handle it.
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
    #[cfg(feature = "tsl_procedural")]
    fn on_audio(&self) -> Option<crate::Result<&super::tsl_procedural::audio::Audio>> {
        None
    }
}
