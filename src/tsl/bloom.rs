//! Three.js r186's five-level HDR bloom. Adjacent Gaussian taps share bilinear fetches.
use super::*;
use crate::renderer::{RenderTarget, RenderTargetOptions};
pub struct Bloom {
    bright: RenderTarget,
    horizontal: Vec<RenderTarget>,
    vertical: Vec<RenderTarget>,
    high_pass: Effect,
    blur: Vec<Effect>,
    composite: Effect,
    sampler: wgpu::Sampler,
    prefiltered: Option<(RenderTarget, Effect)>,
    pub resolution_scale: f32,
    pub strength: f32,
    pub radius: f32,
    pub threshold: f32,
}
impl Bloom {
    pub async fn new(renderer: &Renderer) -> Result<Self> {
        Self::with_input(renderer, Texture::Input.sample(uv()), &[]).await
    }
    pub async fn with_input(
        renderer: &Renderer,
        input: Node,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<Self> {
        let target = || {
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
        let bright = target()?;
        let mut horizontal = Vec::new();
        let mut vertical = Vec::new();
        let mut blur = Vec::new();
        for kernel in [6, 10, 14, 18, 22] {
            horizontal.push(target()?);
            vertical.push(target()?);
            let sigma = kernel as f64 / 3.0;
            let coefficients: Vec<_> = (0..kernel)
                .map(|i| 0.39894 * (-0.5 * (i * i) as f64 / (sigma * sigma)).exp() / sigma)
                .collect();
            let mut source = format!(
                "fn effect(uv:vec2<f32>)->vec4<f32>{{var sum=textureSample(input_texture,input_sampler,uv).rgb*{:.10};let step=params[0].xy;",
                coefficients[0]
            );
            for i in (1..kernel).step_by(2) {
                let a = coefficients[i];
                let b = coefficients.get(i + 1).copied().unwrap_or(0.0);
                let offset = (i as f64 * a + (i + 1) as f64 * b) / (a + b);
                source.push_str(&format!("sum+=(textureSample(input_texture,input_sampler,uv+step*{offset:.10}).rgb+textureSample(input_texture,input_sampler,uv-step*{offset:.10}).rgb)*{:.10};",a+b));
            }
            source.push_str("return vec4(sum,1.0);}");
            blur.push(Effect::new(renderer, wgpu::TextureFormat::Rgba16Float, &source).await?);
        }
        let threshold = uniform(0, Type::Float);
        let luminance = input
            .rgb()
            .dot(vec3(float(0.2126), float(0.7152), float(0.0722)));
        let high_pass = effect_with_textures(
            renderer,
            wgpu::TextureFormat::Rgba16Float,
            &(input * luminance.smoothstep(threshold.clone(), threshold + float(0.01))),
            textures,
        )
        .await?;
        let sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut color = splat(float(0.0), Type::Vec4);
        for (i, factor) in [1.0, 0.8, 0.6, 0.4, 0.2].into_iter().enumerate() {
            color = color
                + Texture::External(i).sample(uv())
                    * mix(
                        float(factor),
                        float(1.2 - factor),
                        uniform(0, Type::Vec2).y(),
                    );
        }
        let views: Vec<_> = vertical
            .iter()
            .map(|t| t.texture.create_view(&Default::default()))
            .collect();
        let composite = effect_with_textures(
            renderer,
            wgpu::TextureFormat::Rgba16Float,
            &(color * uniform(0, Type::Float)),
            &views.iter().map(|v| (v, &sampler)).collect::<Vec<_>>(),
        )
        .await?;
        Ok(Self {
            bright,
            horizontal,
            vertical,
            high_pass,
            blur,
            composite,
            sampler,
            prefiltered: None,
            resolution_scale: 0.5,
            strength: 1.0,
            radius: 0.0,
            threshold: 0.0,
        })
    }
    /// Apply a GPU filter to the full-resolution bright extraction before the pyramid.
    /// The filter reads the extracted HDR image through `input_texture`.
    pub fn set_high_pass_filter(&mut self, renderer: &Renderer, filter: Effect) -> Result<()> {
        self.prefiltered = Some((
            RenderTarget::with_options(
                &renderer.device,
                1,
                1,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    depth_buffer: false,
                    ..Default::default()
                },
            )?,
            filter,
        ));
        Ok(())
    }
    pub fn high_pass_filter_mut(&mut self) -> Option<&mut Effect> {
        self.prefiltered.as_mut().map(|(_, filter)| filter)
    }
    pub fn set_textures(
        &mut self,
        r: &Renderer,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<()> {
        self.high_pass.set_textures(r, textures)
    }
    pub fn render(&mut self, r: &Renderer, input: &RenderTarget) -> Result<&RenderTarget> {
        if !self.resolution_scale.is_finite()
            || self.resolution_scale <= 0.0
            || self.resolution_scale > 1.0
        {
            return Err(Error::Invalid("bloom resolution scale"));
        }
        let (width, height) = (
            ((input.width as f32 * self.resolution_scale) as u32).max(1),
            ((input.height as f32 * self.resolution_scale) as u32).max(1),
        );
        if self.bright.width != width || self.bright.height != height {
            self.bright.set_size(&r.device, width, height)?;
            let (mut w, mut h) = (width, height);
            for i in 0..5 {
                self.horizontal[i].set_size(&r.device, w, h)?;
                self.vertical[i].set_size(&r.device, w, h)?;
                w = (w / 2).max(1);
                h = (h / 2).max(1);
            }
            let views: Vec<_> = self
                .vertical
                .iter()
                .map(|t| t.texture.create_view(&Default::default()))
                .collect();
            self.composite.set_textures(
                r,
                &views.iter().map(|v| (v, &self.sampler)).collect::<Vec<_>>(),
            )?;
        }
        self.high_pass.parameters[0][0] = self.threshold;
        if let Some((extracted, filter)) = &mut self.prefiltered {
            if (extracted.width, extracted.height) != (input.width, input.height) {
                extracted.set_size(&r.device, input.width, input.height)?;
            }
            self.high_pass.apply(r, input, None, extracted)?;
            filter.apply(r, extracted, None, &self.bright)?;
        } else {
            self.high_pass.apply(r, input, None, &self.bright)?;
        }
        for i in 0..5 {
            let (w, h) = (
                self.horizontal[i].width as f32,
                self.horizontal[i].height as f32,
            );
            self.blur[i].parameters[0] = [1.0 / w, 0.0, 0.0, 0.0];
            self.blur[i].apply(
                r,
                if i == 0 {
                    &self.bright
                } else {
                    &self.vertical[i - 1]
                },
                None,
                &self.horizontal[i],
            )?;
            self.blur[i].parameters[0] = [0.0, 1.0 / h, 0.0, 0.0];
            self.blur[i].apply(r, &self.horizontal[i], None, &self.vertical[i])?;
        }
        self.composite.parameters[0] = [self.strength, self.radius, 0.0, 0.0];
        self.composite
            .apply(r, &self.bright, None, &self.horizontal[0])?;
        Ok(&self.horizontal[0])
    }
}
