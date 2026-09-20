//! Persistent GPU mip chain for screen-space volume refraction.
use crate::render_target::RenderTarget;

pub(crate) struct MipChain {
    texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    generator: crate::mipmap::MipGenerator,
}
impl MipChain {
    pub fn new(device: &wgpu::Device, target: &RenderTarget) -> Self {
        let levels = target.width.max(target.height).ilog2() + 1;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("transmission mip chain"),
            size: target.texture.size(),
            mip_level_count: levels,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: target.texture.format(),
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let generator =
            crate::mipmap::MipGenerator::new(device, &texture).expect("render target mip format");
        Self {
            view: texture.create_view(&Default::default()),
            texture,
            generator,
        }
    }
    pub fn update(&self, device: &wgpu::Device, queue: &wgpu::Queue, target: &RenderTarget) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("transmission mip generation"),
        });
        encoder.copy_texture_to_texture(
            target.texture.as_image_copy(),
            self.texture.as_image_copy(),
            target.texture.size(),
        );
        self.generator.encode(&mut encoder);
        queue.submit([encoder.finish()]);
    }
}
