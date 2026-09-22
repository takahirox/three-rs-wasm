use super::*;
use crate::{postprocessing::Effect, readback::RgbaReadback};
pub(in crate::browser) struct Readback {
    base: super::super::tsl_surface::Demo,
    target: RenderTarget,
    reader: RgbaReadback,
    texture: wgpu::Texture,
    effect: Effect,
    selection: usize,
    completed: u32,
    dirty: bool,
    size: (u32, u32),
}
impl Readback {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let base = super::super::tsl_surface::Demo::create(s, c, 63, r).await?;
        let target = RenderTarget::with_options(
            &r.device,
            512,
            512,
            RenderTargetOptions {
                count: 2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..Default::default()
            },
        )?;
        let reader = RgbaReadback::new(r, 512, 512)?;
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("CPU readback DataTexture"),
            size: wgpu::Extent3d {
                width: 512,
                height: 512,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let sampler = r.device.create_sampler(&Default::default());
        let effect = effect_with_textures(
            r,
            HDR,
            &tsl::Texture::External(0).sample(uv()),
            &[(&texture.create_view(&Default::default()), &sampler)],
        )
        .await?;
        Ok(Self {
            base,
            target,
            reader,
            texture,
            effect,
            selection: 0,
            completed: 0,
            dirty: true,
            size: (0, 0),
        })
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        self.dirty |= a;
        self.base.update(s, c, d, a)
    }
    pub fn seek(&mut self, t: f64) {
        self.dirty = true;
        self.base.seek(t);
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i != 0 || !v.is_finite() || !(0.0..=2.0).contains(&v) || v.fract() != 0. {
            return Err(Error::Invalid("readback selection"));
        }
        self.dirty = true;
        self.selection = v as usize;
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        w: f64,
        p: bool,
        h: f64,
    ) -> Result<()> {
        self.dirty = true;
        self.base.input(s, c, dx, dy, w, p, h)
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.size != (out.width, out.height) {
            self.dirty = true;
            self.size = (out.width, out.height);
        }
        if let Some(result) = self.reader.take() {
            let bytes = result?;
            r.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &bytes,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(512 * 4),
                    rows_per_image: Some(512),
                },
                wgpu::Extent3d {
                    width: 512,
                    height: 512,
                    depth_or_array_layers: 1,
                },
            );
            self.completed += 1;
            if let Some(canvas) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.query_selector("canvas").ok().flatten())
            {
                let _ = canvas.set_attribute("data-readbacks", &self.completed.to_string());
            }
        }
        if self.selection == 0 {
            return self.base.render(r, s, c, out);
        }
        if self.reader.is_idle() && self.dirty {
            r.render(s, c, &self.target)?;
            self.reader
                .begin_notify(r, &self.target, self.selection - 1, || {
                    if let Some(w) = web_sys::window()
                        && let Ok(e) = web_sys::Event::new("gallery-readback-ready")
                    {
                        let _ = w.dispatch_event(&e);
                    }
                })?;
            self.dirty = false;
        }
        self.effect.apply(r, &self.target, None, out)?;
        Ok(true)
    }
}
