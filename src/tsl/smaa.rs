//! SMAA 1x Medium, r186. Three resident HDR GPU passes with the original area
//! and search tables; no CPU edge detection and no substitute FXAA pass.
use crate::{Result, postprocessing::Effect, renderer::*};
pub struct SmaaPass {
    edges: RenderTarget,
    weights: RenderTarget,
    detect: Effect,
    search: Effect,
    blend: Effect,
}
impl SmaaPass {
    pub async fn new(
        r: &Renderer,
        area: (&wgpu::TextureView, &wgpu::Sampler),
        search: (&wgpu::TextureView, &wgpu::Sampler),
    ) -> Result<Self> {
        let options = RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba16Float,
            depth_buffer: false,
            ..Default::default()
        };
        Ok(Self {
            edges: RenderTarget::with_options(&r.device, 1, 1, options.clone())?,
            weights: RenderTarget::with_options(&r.device, 1, 1, options.clone())?,
            detect: Effect::new(r, options.format, include_str!("smaa_edges.wgsl")).await?,
            search: Effect::with_textures(
                r,
                options.format,
                include_str!("smaa_weights.wgsl"),
                &[area, search],
            )
            .await?,
            blend: Effect::new(r, options.format, include_str!("smaa_blend.wgsl")).await?,
        })
    }
    pub fn render(&mut self, r: &Renderer, input: &RenderTarget, out: &RenderTarget) -> Result<()> {
        self.edges.set_size(&r.device, input.width, input.height)?;
        self.weights
            .set_size(&r.device, input.width, input.height)?;
        self.detect.apply(r, input, None, &self.edges)?;
        self.search.apply(r, &self.edges, None, &self.weights)?;
        self.blend.apply(r, input, Some(&self.weights), out)
    }
}
