//! What an example's frame loop drives: the example, behind the gallery's
//! camera viewer. The examples are the modules' ( gallery/src/browser/example.rs ).
use super::gltf_viewer::OrbitViewer;
use crate::{Result, camera::*, math::*, scene::*};

/// An example's page content: the example behind the gallery's camera viewer
/// and clip planes, which drive the camera when the example does not.
pub(super) struct Demo {
    viewer: OrbitViewer,
    near: f64,
    far: f64,
    pub(super) content: Box<dyn super::example::Example>,
}
/// An example module's page content: its demo behind the gallery's camera
/// viewer and clip planes.
pub(super) fn module(
    viewer: OrbitViewer,
    near: f64,
    far: f64,
    content: Box<dyn super::example::Example>,
) -> Demo {
    Demo {
        viewer,
        near,
        far,
        content,
    }
}
/// An example that drives its camera itself.
pub(super) fn own(content: Box<dyn super::example::Example>) -> Demo {
    module(
        OrbitViewer::from_camera(Vector3::ZERO, 1.0),
        0.1,
        1000.0,
        content,
    )
}
impl Demo {
    #[cfg(feature = "example_tsl_procedural")]
    pub fn audio(&self) -> crate::Result<&super::tsl_procedural::audio::Audio> {
        if let Some(audio) = self.content.on_audio() {
            audio
        } else {
            Err(crate::Error::Invalid("not an audio example"))
        }
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        if let Some(result) = self.content.on_update(scene, camera, delta, animate) {
            return result;
        }
        self.viewer.update(scene, camera)?;
        if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(camera)?.kind {
            c.near = self.near;
            c.far = self.far;
        }
        Ok(())
    }
    pub fn wireframe(&mut self, scene: &mut Scene, enabled: bool) -> Result<()> {
        self.content
            .on_wireframe(scene, enabled)
            .unwrap_or(Err(crate::Error::Invalid("indexed example")))
    }
    pub fn morph(&mut self, scene: &mut Scene, weights: [f64; 2]) -> Result<()> {
        self.content
            .on_morph(scene, weights)
            .unwrap_or(Err(crate::Error::Invalid("morph example")))
    }
    pub fn select(&mut self, scene: &mut Scene, cam: Object3D, x: f64, y: f64) -> Result<()> {
        self.content.on_select(scene, cam, x, y).unwrap_or(Ok(()))
    }
    pub fn gpu_pointer(
        &mut self,
        r: &crate::renderer::Renderer,
        scene: &mut Scene,
        cam: Object3D,
        x: f64,
        y: f64,
    ) -> Result<bool> {
        self.content
            .on_gpu_pointer(r, scene, cam, x, y)
            .unwrap_or(Ok(false))
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        self.content.on_pointer(x, y);
    }
    pub fn dragging(&mut self, value: bool) {
        self.content.on_dragging(value);
    }
    pub fn render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut Scene,
        camera: Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Result<bool> {
        self.content
            .on_render(renderer, scene, camera, target)
            .unwrap_or(Ok(false))
    }
    pub fn output_target(&self) -> Option<&crate::renderer::RenderTarget> {
        self.content.on_output_target().unwrap_or(None)
    }
    /// The target `output_target` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_target_mut(&mut self) -> Option<&mut crate::renderer::RenderTarget> {
        self.content.on_output_target_mut().unwrap_or(None)
    }
    /// Every target the example may make its output target ( see
    /// Example::on_output_targets_mut ).
    pub fn output_targets_mut(&mut self) -> Vec<&mut crate::renderer::RenderTarget> {
        self.content.on_output_targets_mut()
    }
    pub fn prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut Scene,
        camera: Object3D,
        aspect: f64,
    ) -> Result<()> {
        self.content.on_prepare(renderer, scene, camera, aspect)
    }
    pub fn tsl_parameter(&mut self, index: usize, value: f32) -> Result<()> {
        self.content
            .on_tsl_parameter(index, value)
            .unwrap_or(Err(crate::Error::Invalid("not a TSL example")))
    }
    /// webaudio_timing's listener, sources and pending plays.
    pub fn audio_frame(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
    ) -> crate::Result<Vec<f32>> {
        self.content
            .on_audio_frame(scene, camera)
            .unwrap_or(Err(crate::Error::Invalid("not a positional audio example")))
    }
    /// webaudio_visualizer's analyser bins.
    pub fn audio_data(&mut self, bytes: &[u8]) -> crate::Result<()> {
        self.content
            .on_audio_data(bytes)
            .unwrap_or(Err(crate::Error::Invalid("not an analyser example")))
    }
    /// A file produced by an exporter button, taken once.
    pub fn take_export(
        &mut self,
        renderer: &crate::renderer::Renderer,
    ) -> Option<(String, Vec<u8>)> {
        self.content.on_take_export(renderer).unwrap_or(None)
    }
    pub fn status(&self) -> String {
        self.content.on_status().unwrap_or_default()
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.content.on_key(code, down);
    }
    pub fn slider(&mut self, x: f64) {
        self.content.on_slider(x);
    }
    /// The pointer input, and whether it needs a frame.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) -> Result<bool> {
        self.content
            .on_draw(kind, x, y)
            .unwrap_or(Err(crate::Error::Invalid("not a drawing example")))?;
        Ok(self.content.on_draw_redraws())
    }
    pub fn seek(&mut self, seconds: f64) {
        self.content.on_seek(seconds);
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if let Some(result) = self
            .content
            .on_input(scene, camera, dx, dy, wheel, pan, height)
        {
            return result;
        }
        if pan {
            self.viewer.pan_pixels(scene, camera, dx, dy, height)?;
        } else {
            let (zoom, min, max) =
                self.content
                    .on_orbit_limits()
                    .unwrap_or((true, 0.0, f64::INFINITY));
            self.viewer
                .orbit_pixels(dx, dy, if zoom { wheel } else { 0.0 }, height, min, max);
        }
        Ok(())
    }
    pub fn viewport(&mut self, index: usize, rectangle: [f64; 4]) -> Result<()> {
        self.content
            .on_viewport(index, rectangle)
            .unwrap_or(Err(crate::Error::Invalid("multiple elements example")))
    }
    /// After the frame is submitted, in the same task: effects that read the
    /// canvas back (AsciiEffect).
    pub fn presented(&mut self, canvas: &web_sys::HtmlCanvasElement) -> Result<()> {
        self.content.on_presented(canvas)
    }
    pub fn attach_canvases(
        &mut self,
        r: &crate::renderer::Renderer,
        canvases: js_sys::Array,
    ) -> Result<()> {
        self.content
            .on_attach_canvases(r, canvases)
            .unwrap_or(Err(crate::Error::Invalid("multiple canvases example")))
    }
}
