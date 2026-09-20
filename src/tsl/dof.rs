//! r186's near/far bokeh depth of field. CoC fields share an RG16F attachment;
//! all filtering and compositing stays on the GPU.
use super::*;
use crate::renderer::{RenderTarget, RenderTargetOptions};
pub struct DepthOfField {
    targets: Vec<RenderTarget>,
    coc: Effect,
    gaussian: Effect,
    copy: Effect,
    blur64: Effect,
    blur16: Effect,
    composite: Effect,
    pub focus_distance: f32,
    pub focal_length: f32,
    pub bokeh_scale: f32,
}
impl DepthOfField {
    pub async fn new(r: &Renderer, depth: &wgpu::TextureView) -> Result<Self> {
        let mut targets = Vec::new();
        // CoC, horizontal Gaussian, vertical Gaussian, half-size near CoC,
        // shared 64-tap intermediate, near, far, final output.
        for format in [
            wgpu::TextureFormat::Rg16Float,
            wgpu::TextureFormat::R16Float,
            wgpu::TextureFormat::R16Float,
            wgpu::TextureFormat::R16Float,
            wgpu::TextureFormat::Rgba16Float,
            wgpu::TextureFormat::Rgba16Float,
            wgpu::TextureFormat::Rgba16Float,
            wgpu::TextureFormat::Rgba16Float,
        ] {
            targets.push(RenderTarget::with_options(
                &r.device,
                1,
                1,
                RenderTargetOptions {
                    format,
                    depth_buffer: false,
                    ..Default::default()
                },
            )?);
        }
        let distance = uniform(0, Type::Vec4).x() * uniform(0, Type::Vec4).y()
            / (uniform(0, Type::Vec4).y()
                - depth_texture(uv()) * (uniform(0, Type::Vec4).y() - uniform(0, Type::Vec4).x()));
        let signed = distance - uniform(0, Type::Vec4).swizzle("z");
        let coc = signed
            .abs()
            .smoothstep(float(0.0), uniform(0, Type::Vec4).swizzle("w"));
        let coc = depth_effect(
            r,
            wgpu::TextureFormat::Rg16Float,
            &vec4(
                vec3(
                    signed
                        .less_than(float(0.000001))
                        .select(coc.clone(), float(0.0)),
                    signed.less_than(float(0.0)).select(float(0.0), coc),
                    float(0.0),
                ),
                float(1.0),
            ),
            depth,
        )
        .await?;
        let mut code = String::from(
            "fn effect(uv:vec2<f32>)->vec4<f32>{var v=textureSample(input_texture,input_sampler,uv).r;",
        );
        let sigma = 7.0_f64 / 3.0;
        let mut weight = 1.0;
        for i in 1..7 {
            let w = (-0.5 * (i * i) as f64 / (sigma * sigma)).exp();
            weight += 2.0 * w;
            code.push_str(&format!("v+=(textureSample(input_texture,input_sampler,uv+params[0].xy*{i}.0).r+textureSample(input_texture,input_sampler,uv-params[0].xy*{i}.0).r)*{w:.12};"));
        }
        code.push_str(&format!("return vec4(v/{weight:.12},0.0,0.0,1.0);}}"));
        let gaussian = Effect::new(r, wgpu::TextureFormat::R16Float, &code).await?;
        let copy = effect(
            r,
            wgpu::TextureFormat::R16Float,
            &Texture::Input.sample(uv()),
        )
        .await?;
        let mut kernels = [String::new(), String::new()];
        for i in 0..80 {
            let theta = i as f64 * 2.39996323;
            let rad = (i as f64 / 80.0).sqrt();
            let p = format!(
                "vec2<f32>({:.12},{:.12})",
                rad * theta.cos(),
                rad * theta.sin()
            );
            let group = usize::from(i % 5 == 0);
            if !kernels[group].is_empty() {
                kernels[group].push(',');
            }
            kernels[group].push_str(&p);
        }
        let blur64=Effect::new(r,wgpu::TextureFormat::Rgba16Float,&format!(r#"
const kernel=array<vec2<f32>,64>({});
fn effect(uv:vec2<f32>)->vec4<f32>{{let fields=textureSample(history_texture,input_sampler,uv);let coc=select(fields.r,fields.g,params[0].w>0.5);let step=params[0].xy*params[0].z*coc;var acc=vec3<f32>(0.0);for(var i=0;i<64;i++){{acc+=textureSampleLevel(input_texture,input_sampler,uv+step*kernel[i],0.0).rgb;}}return vec4(acc/64.0,coc);}}
"#,kernels[0])).await?;
        let blur16=Effect::new(r,wgpu::TextureFormat::Rgba16Float,&format!(r#"
const kernel=array<vec2<f32>,16>({});
fn effect(uv:vec2<f32>)->vec4<f32>{{let col=textureSample(input_texture,input_sampler,uv);let step=params[0].xy*params[0].z*col.a;var acc=col.rgb;for(var i=0;i<16;i++){{acc=max(acc,textureSampleLevel(input_texture,input_sampler,uv+step*kernel[i],0.0).rgb);}}return vec4(acc,col.a);}}
"#,kernels[1])).await?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let far = Texture::History.sample(uv());
        let near = Texture::External(0).sample(uv());
        let composite = effect_with_textures(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &mix(
                mix(
                    Texture::Input.sample(uv()),
                    far.clone(),
                    far.swizzle("w").clamp(float(0.0), float(0.5)) * float(2.0),
                ),
                near.clone(),
                near.swizzle("w").clamp(float(0.0), float(0.5)) * float(2.0),
            ),
            &[(
                &targets[5].texture.create_view(&Default::default()),
                &sampler,
            )],
        )
        .await?;
        Ok(Self {
            targets,
            coc,
            gaussian,
            copy,
            blur64,
            blur16,
            composite,
            focus_distance: 500.0,
            focal_length: 200.0,
            bokeh_scale: 10.0,
        })
    }
    pub fn set_depth(&mut self, r: &Renderer, view: &wgpu::TextureView) -> Result<()> {
        self.coc.set_depth(r, view)
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        input: &RenderTarget,
        near: f32,
        far: f32,
    ) -> Result<&RenderTarget> {
        if ![
            near,
            far,
            self.focus_distance,
            self.focal_length,
            self.bokeh_scale,
        ]
        .iter()
        .all(|v| v.is_finite())
            || near <= 0.0
            || far <= near
            || self.focal_length <= 0.0
            || self.bokeh_scale < 0.0
        {
            return Err(Error::Invalid("depth of field parameters"));
        }
        let (w, h) = (input.width, input.height);
        if (self.targets[0].width, self.targets[0].height) != (w, h) {
            for (i, t) in self.targets.iter_mut().enumerate() {
                let half = (3..=6).contains(&i);
                t.set_size(
                    &r.device,
                    if half { w.div_ceil(2) } else { w },
                    if half { h.div_ceil(2) } else { h },
                )?;
            }
            let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });
            self.composite.set_textures(
                r,
                &[(
                    &self.targets[5].texture.create_view(&Default::default()),
                    &sampler,
                )],
            )?;
        }
        self.coc.parameters[0] = [near, far, self.focus_distance, self.focal_length];
        self.coc.apply(r, input, None, &self.targets[0])?;
        self.gaussian.parameters[0] = [1.0 / w as f32, 0.0, 0.0, 0.0];
        self.gaussian
            .apply(r, &self.targets[0], None, &self.targets[1])?;
        self.gaussian.parameters[0] = [0.0, 1.0 / h as f32, 0.0, 0.0];
        self.gaussian
            .apply(r, &self.targets[1], None, &self.targets[2])?;
        self.copy
            .apply(r, &self.targets[2], None, &self.targets[3])?;
        for side in 0..2 {
            self.blur64.parameters[0] = [
                1.0 / w as f32,
                1.0 / h as f32,
                self.bokeh_scale,
                side as f32,
            ];
            self.blur64.apply(
                r,
                input,
                Some(&self.targets[if side == 0 { 3 } else { 0 }]),
                &self.targets[4],
            )?;
            self.blur16.parameters[0] = self.blur64.parameters[0];
            self.blur16
                .apply(r, &self.targets[4], None, &self.targets[5 + side])?;
        }
        self.composite
            .apply(r, input, Some(&self.targets[6]), &self.targets[7])?;
        Ok(&self.targets[7])
    }
}
