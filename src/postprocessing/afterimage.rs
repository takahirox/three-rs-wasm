//! Persistent GPU history with Three.js r186 AfterImageNode's component threshold.
use super::Effect;
use crate::{Result, renderer::*, tsl::*};
pub struct AfterImagePass {
    targets: [RenderTarget; 2],
    effect: Effect,
    current: usize,
    pub damp: f32,
}
impl AfterImagePass {
    pub async fn new(renderer: &Renderer) -> Result<Self> {
        let create = || {
            RenderTarget::with_options(
                &renderer.device,
                1,
                1,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    depth_buffer: false,
                    ..Default::default()
                },
            )
        };
        let old = Texture::History.sample(uv());
        let mask = (old.clone() - float(0.1)).sign().max(float(0.0));
        let graph = Texture::Input
            .sample(uv())
            .max(old * mask * uniform(0, Type::Float));
        Ok(Self {
            targets: [create()?, create()?],
            effect: effect(renderer, wgpu::TextureFormat::Rgba16Float, &graph).await?,
            current: 0,
            damp: 0.8,
        })
    }
    pub fn render(&mut self, renderer: &Renderer, input: &RenderTarget) -> Result<()> {
        for target in &mut self.targets {
            target.set_size(&renderer.device, input.width, input.height)?;
        }
        self.effect.parameters[0][0] = self.damp;
        let next = 1 - self.current;
        self.effect.apply(
            renderer,
            input,
            Some(&self.targets[self.current]),
            &self.targets[next],
        )?;
        self.current = next;
        Ok(())
    }
    pub fn output(&self) -> &RenderTarget {
        &self.targets[self.current]
    }
}
