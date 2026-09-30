//! EffectComposer passes as full-screen draws on GPU targets: CopyShader,
//! GammaCorrectionShader (and OutputPass's sRGB transfer), FilmPass,
//! VignetteShader, DotScreenPass, ColorifyShader, SepiaShader,
//! BleachBypassShader, the separable blur shaders, BloomPass's convolution
//! and combine, and FocusShader. One uniform buffer holds every draw's
//! uniforms at a dynamic offset; bind groups are cached per (input, mask)
//! pair until the targets are recreated.
use crate::renderer::*;
use std::collections::HashMap;

/// One uniform slot per draw, at the dynamic offset alignment.
const SLOT: u64 = 256;
const SLOTS: usize = 32;
/// The shared full-screen shader, one entry point per ShaderPass. As ANGLE
/// renders WebGL, the composer targets are stored from their bottom row with
/// the triangle's y negated (u.mode.y), so the interpolated vUv, and the hash
/// FilmPass draws from it, round as in WebGL; the scene and mask targets and
/// the canvas are stored from their top row (u.mode.z: the input is).
pub(super) fn source(kernel: &[f32]) -> String {
    let kernel = kernel
        .iter()
        .map(|k| format!("{k:?}"))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "struct U{{a:vec4<f32>,b:vec4<f32>,view:vec4<f32>,mode:vec4<f32>}}
@group(0) @binding(0) var<uniform> u:U;
@group(0) @binding(1) var t:texture_2d<f32>;
@group(0) @binding(2) var mask:texture_2d<f32>;
@group(0) @binding(3) var s:sampler;
struct V{{@builtin(position) p:vec4<f32>,@location(0) uv:vec2<f32>}}
// FullScreenQuad's triangle and its uv.
@vertex fn vs(@builtin(vertex_index) i:u32)->V{{
 let p=array(vec2(-1.0,3.0),vec2(-1.0,-1.0),vec2(3.0,-1.0));let uv=array(vec2(0.0,2.0),vec2(0.0,0.0),vec2(2.0,0.0));
 return V(vec4(p[i].x,select(p[i].y,-p[i].y,u.mode.y>0.5),0.0,1.0),uv[i]);}}
fn tex(uv:vec2<f32>)->vec4<f32>{{return textureSampleLevel(t,s,select(uv,vec2(uv.x,1.0-uv.y),u.mode.z>0.5),0.0);}}
fn luminance(c:vec3<f32>)->f32{{return dot(c,vec3(0.2126729,0.7151522,0.0721750));}}
// The stencil test: mode 1 draws where the head is, 2 where it is not; the
// rest keeps the read buffer (EffectComposer's copy pass).
fn finish(v:V,effect:vec4<f32>)->vec4<f32>{{
 if u.mode.x==0.0 {{return effect;}}
 let size=vec2<i32>(textureDimensions(mask));let p=vec2<i32>(v.p.xy);
 let head=textureLoad(mask,select(p,vec2(p.x,size.y-1-p.y),u.mode.y>0.5),0).r>0.5;
 if head==(u.mode.x==1.0) {{return effect;}}
 return tex(v.uv);}}
@fragment fn fs_copy(v:V)->@location(0) vec4<f32>{{return finish(v,tex(v.uv));}}
@fragment fn fs_gamma(v:V)->@location(0) vec4<f32>{{
 let c=tex(v.uv);return vec4(select(pow(c.rgb,vec3(0.41666))*1.055-vec3(0.055),c.rgb*12.92,c.rgb<=vec3(0.0031308)),c.a);}}
fn rand(uv:vec2<f32>)->f32{{let pi=3.141592653589793;let dt=dot(uv,vec2(12.9898,78.233));let sn=dt-pi*floor(dt/pi);return fract(sin(sn)*43758.5453);}}
@fragment fn fs_film(v:V)->@location(0) vec4<f32>{{
 let uv=v.uv;let base=tex(uv);let noise=rand(fract(uv+vec2(u.a.x)));
 var color=base.rgb+base.rgb*clamp(0.1+noise,0.0,1.0);color=mix(base.rgb,color,u.a.y);
 if u.a.z>0.5 {{color=vec3(luminance(color));}}
 return vec4(color,base.a);}}
@fragment fn fs_vignette(v:V)->@location(0) vec4<f32>{{
 let texel=tex(v.uv);let uv=(v.uv-vec2(0.5))*vec2(u.a.x);
 return vec4(mix(texel.rgb,vec3(1.0-u.a.y),dot(uv,uv)),texel.a);}}
@fragment fn fs_dot(v:V)->@location(0) vec4<f32>{{
 let uv=v.uv;let sn=sin(u.b.x);let c=cos(u.b.x);let t2=uv*u.a.xy-u.a.zw;
 let point=vec2(c*t2.x-sn*t2.y,sn*t2.x+c*t2.y)*u.b.y;let pattern=(sin(point.x)*sin(point.y))*4.0;
 let color=tex(uv);let average=(color.r+color.g+color.b)/3.0;
 return vec4(vec3(average*10.0-5.0+pattern),color.a);}}
@fragment fn fs_colorify(v:V)->@location(0) vec4<f32>{{
 let texel=tex(v.uv);return finish(v,vec4(luminance(texel.xyz)*u.a.xyz,texel.w));}}
@fragment fn fs_sepia(v:V)->@location(0) vec4<f32>{{
 let color=tex(v.uv);let c=color.rgb;let amount=u.a.x;
 let r=dot(c,vec3(1.0-0.607*amount,0.769*amount,0.189*amount));
 let g=dot(c,vec3(0.349*amount,1.0-0.314*amount,0.168*amount));
 let b=dot(c,vec3(0.272*amount,0.534*amount,1.0-0.869*amount));
 return vec4(min(vec3(1.0),vec3(r,g,b)),color.a);}}
@fragment fn fs_bleach(v:V)->@location(0) vec4<f32>{{
 let base=tex(v.uv);let lum=luminance(base.rgb);let blend=vec3(lum);
 let l=min(1.0,max(0.0,10.0*(lum-0.45)));let result1=2.0*base.rgb*blend;
 let result2=1.0-2.0*(1.0-blend)*(1.0-base.rgb);let new_color=mix(result1,result2,l);
 let a2=u.a.x*base.a;var mix_rgb=a2*new_color.rgb;mix_rgb+=((1.0-a2)*base.rgb);
 return vec4(mix_rgb,base.a);}}
const WEIGHTS=array(0.051,0.0918,0.12245,0.1531,0.1633,0.1531,0.12245,0.0918,0.051);
@fragment fn fs_blur(v:V)->@location(0) vec4<f32>{{
 let uv=v.uv;var sum=vec4(0.0);
 for(var i=0;i<9;i++){{sum+=tex(uv+f32(i-4)*u.a.xy)*WEIGHTS[i];}}
 return finish(v,sum);}}
const KERNEL=array({kernel});
@fragment fn fs_convolution(v:V)->@location(0) vec4<f32>{{
 var coord=v.uv-12.0*u.a.xy;var sum=vec4(0.0);
 for(var i=0;i<25;i++){{sum+=tex(coord)*KERNEL[i];coord+=u.a.xy;}}
 return sum;}}
@fragment fn fs_combine(v:V)->@location(0) vec4<f32>{{return u.a.x*tex(v.uv);}}
@fragment fn fs_focus(v:V)->@location(0) vec4<f32>{{
 let uv=v.uv;var color=tex(uv);let org=color;var add=color;
 let vin=(uv-vec2(0.5))*vec2(1.4);let sample_dist=dot(vin,vin)*2.0;
 let f=(u.a.w*100.0+sample_dist)*u.a.z*4.0;let sample_size=vec2(1.0/u.a.x,1.0/u.a.y)*vec2(f);
 const OFFSETS=array(vec2(0.111964,0.993712),vec2(0.846724,0.532032),vec2(0.943883,-0.330279),vec2(0.330279,-0.943883),vec2(-0.532032,-0.846724),vec2(-0.993712,-0.111964),vec2(-0.707107,0.707107));
 for(var i=0;i<7;i++){{let tmp=tex(uv+OFFSETS[i]*sample_size);add+=tmp;if tmp.b<color.b {{color=tmp;}}}}
 color=color*vec4(2.0)-(add/vec4(8.0));
 color=color+(add/vec4(8.0)-color)*(vec4(1.0)-vec4(sample_dist*0.5));
 return vec4(color.rgb*color.rgb*vec3(0.95)+color.rgb,1.0);}}"
    )
}
/// BloomPass's buildKernel( sigma ): the normalized 25-tap Gaussian.
pub(super) fn kernel(sigma: f64) -> Vec<f32> {
    let size = ((2. * (sigma * 3.).ceil() + 1.) as usize).min(25);
    let half = (size - 1) as f64 * 0.5;
    let values: Vec<f64> = (0..size)
        .map(|i| (-(i as f64 - half).powi(2) / (2. * sigma * sigma)).exp())
        .collect();
    let sum: f64 = values.iter().sum();
    values.iter().map(|v| (v / sum) as f32).collect()
}
/// One draw: its fragment entry, the input and mask views (indices into the
/// caller's views), the target (None: the canvas) and its viewport.
pub(super) struct Draw {
    pub(super) entry: &'static str,
    pub(super) input: usize,
    pub(super) mask: usize,
    pub(super) output: Option<usize>,
    pub(super) viewport: [f32; 4],
    pub(super) uniforms: [[f32; 4]; 2],
    /// The stencil test: 0 none, 1 draw where the mask is set, 2 where not.
    pub(super) mode: f32,
    pub(super) load: bool,
    /// The target is a composer target, stored from its bottom row.
    pub(super) gl: bool,
    /// The input is stored from its top row (a scene render).
    pub(super) flip: bool,
}
impl Draw {
    /// A draw into a composer target from a composer target.
    pub(super) fn new(
        entry: &'static str,
        input: usize,
        output: Option<usize>,
        viewport: [f32; 4],
        a: [f32; 4],
    ) -> Self {
        Self {
            entry,
            input,
            mask: input,
            output,
            viewport,
            uniforms: [a, [0.; 4]],
            mode: 0.,
            load: false,
            gl: output.is_some(),
            flip: false,
        }
    }
}
pub(super) struct Kit {
    module: wgpu::ShaderModule,
    layout: wgpu::PipelineLayout,
    group: wgpu::BindGroupLayout,
    pipelines: HashMap<(&'static str, wgpu::TextureFormat), wgpu::RenderPipeline>,
    uniforms: wgpu::Buffer,
    sampler: wgpu::Sampler,
    /// Bind groups per (input, mask), valid until the targets change.
    bind: HashMap<(usize, usize), wgpu::BindGroup>,
}
impl Kit {
    pub(super) fn new(r: &Renderer) -> Self {
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("composer passes"),
            source: wgpu::ShaderSource::Wgsl(source(&kernel(4.)).into()),
        });
        let texture_entry = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("composer passes"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            min_binding_size: wgpu::BufferSize::new(64),
                        },
                        count: None,
                    },
                    texture_entry(1),
                    texture_entry(2),
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });
        let layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&group],
                push_constant_ranges: &[],
            });
        let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("composer pass uniforms"),
            size: SLOT * SLOTS as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        // Render targets: linear filtering, clamped, no mipmaps.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Self {
            module,
            layout,
            group,
            pipelines: HashMap::new(),
            uniforms,
            sampler,
            bind: HashMap::new(),
        }
    }
    /// The targets were recreated: their bind groups go with them.
    pub(super) fn reset(&mut self) {
        self.bind.clear();
    }
    fn pipeline(&mut self, r: &Renderer, entry: &'static str, format: wgpu::TextureFormat) {
        if self.pipelines.contains_key(&(entry, format)) {
            return;
        }
        // BloomPass's combine: AdditiveBlending (SRC_ALPHA, ONE).
        let additive = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        };
        let pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(entry),
                layout: Some(&self.layout),
                vertex: wgpu::VertexState {
                    module: &self.module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &self.module,
                    entry_point: Some(entry),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: (entry == "fs_combine").then_some(wgpu::BlendState {
                            color: additive,
                            alpha: additive,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        self.pipelines.insert((entry, format), pipeline);
    }
    /// Encode the draws in order into one submission. `views` and `formats`
    /// are the caller's targets; `screen` is the canvas target.
    pub(super) fn run(
        &mut self,
        r: &Renderer,
        draws: &[Draw],
        views: &[&wgpu::TextureView],
        formats: &[wgpu::TextureFormat],
        screen: (&wgpu::TextureView, wgpu::TextureFormat),
    ) {
        let mut data = vec![0f32; SLOTS * SLOT as usize / 4];
        for (i, d) in draws.iter().enumerate() {
            let slot = &mut data[i * SLOT as usize / 4..][..16];
            slot[0..4].copy_from_slice(&d.uniforms[0]);
            slot[4..8].copy_from_slice(&d.uniforms[1]);
            slot[8..12].copy_from_slice(&d.viewport);
            slot[12] = d.mode;
            slot[13] = f32::from(d.gl);
            slot[14] = f32::from(d.flip);
        }
        r.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
        for d in draws {
            let format = d.output.map_or(screen.1, |o| formats[o]);
            self.pipeline(r, d.entry, format);
            if !self.bind.contains_key(&(d.input, d.mask)) {
                let bind = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("composer pass"),
                    layout: &self.group,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                                buffer: &self.uniforms,
                                offset: 0,
                                size: wgpu::BufferSize::new(64),
                            }),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(views[d.input]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(views[d.mask]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: wgpu::BindingResource::Sampler(&self.sampler),
                        },
                    ],
                });
                self.bind.insert((d.input, d.mask), bind);
            }
        }
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let mut screen_cleared = false;
        for (i, d) in draws.iter().enumerate() {
            let (view, format) = match d.output {
                None => screen,
                Some(o) => (views[o], formats[o]),
            };
            let load = if d.load || (d.output.is_none() && screen_cleared) {
                wgpu::LoadOp::Load
            } else {
                wgpu::LoadOp::Clear(wgpu::Color::BLACK)
            };
            screen_cleared |= d.output.is_none();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(d.entry),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            let [x, y, w, h] = d.viewport;
            pass.set_viewport(x, y, w, h, 0., 1.);
            pass.set_pipeline(&self.pipelines[&(d.entry, format)]);
            pass.set_bind_group(0, &self.bind[&(d.input, d.mask)], &[i as u32 * SLOT as u32]);
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
    }
}
