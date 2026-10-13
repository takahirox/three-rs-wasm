//! @takram/three-atmosphere's AtmosphereParameters.DEFAULT and
//! PrecomputedTexturesGenerator.update(): Bruneton's precomputation on the GPU
//! with the programs three compiles for it ( float32 targets, as with
//! OES_texture_float_linear ). WebGPU cannot blend float32 targets, so the
//! additive passes ( CustomBlending One + One ) copy the target, render the
//! program into a scratch target and sum the two in a pass of their own.
//! The CPU copies of the transmittance and irradiance textures
//! ( readRenderTargetPixelsAsync ) only feed SkyLightProbe and
//! SunDirectionalLight, which this example does not create.
use super::program::{
    Attachment, Bound, LINEAR, PipelineKey, Program, Samplers, Vertices, fullscreen_buffer, target,
};
use crate::{Error, Result, renderer::Renderer};

pub(crate) const TRANSMITTANCE: (u32, u32) = (256, 64);
pub(crate) const IRRADIANCE: (u32, u32) = (64, 16);
pub(crate) const SCATTERING: (u32, u32, u32) = (8 * 32, 128, 32);
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;

/// DensityProfileLayer( width, expTerm, expScale, linearTerm, constantTerm ).
type Layer = [f64; 5];

/// AtmosphereParameters.DEFAULT.toUniform() ( lengths in kilometers ).
pub(crate) fn set_atmosphere(block: &mut super::program::Block, prefix: &str) {
    let layers = |block: &mut super::program::Block, name: &str, layers: [Layer; 2]| {
        for (k, l) in layers.iter().enumerate() {
            for (field, v) in [
                "width",
                "exp_term",
                "exp_scale",
                "linear_term",
                "constant_term",
            ]
            .iter()
            .zip(l)
            {
                block.set1(&format!("{prefix}.{name}.layers[{k}].{field}"), *v);
            }
        }
    };
    block.set(
        &format!("{prefix}.solar_irradiance"),
        &[1.474, 1.8504, 1.91198],
    );
    block.set1(&format!("{prefix}.sun_angular_radius"), 0.004675);
    block.set1(&format!("{prefix}.bottom_radius"), 6360000. * (1. / 1000.));
    block.set1(&format!("{prefix}.top_radius"), 6420000. * (1. / 1000.));
    layers(
        block,
        "rayleigh_density",
        [[0., 0., 0., 0., 0.], [0., 1., -0.125, 0., 0.]],
    );
    block.set(
        &format!("{prefix}.rayleigh_scattering"),
        &[0.005802, 0.013558, 0.0331],
    );
    layers(
        block,
        "mie_density",
        [[0., 0., 0., 0., 0.], [0., 1., -0.833333, 0., 0.]],
    );
    block.set(
        &format!("{prefix}.mie_scattering"),
        &[0.003996, 0.003996, 0.003996],
    );
    block.set(
        &format!("{prefix}.mie_extinction"),
        &[0.00444, 0.00444, 0.00444],
    );
    block.set1(&format!("{prefix}.mie_phase_function_g"), 0.8);
    layers(
        block,
        "absorption_density",
        [
            [25., 0., 0., 1. / 15., -2. / 3.],
            [0., 0., 0., -1. / 15., 8. / 3.],
        ],
    );
    block.set(
        &format!("{prefix}.absorption_extinction"),
        &[0.00065, 0.001881, 0.000085],
    );
    block.set(&format!("{prefix}.ground_albedo"), &[0.1, 0.1, 0.1]);
    // Math.cos( radians( 120 ) ).
    block.set1(
        &format!("{prefix}.mu_s_min"),
        crate::bounce::trig::cos(120. * (std::f64::consts::PI / 180.)),
    );
}

/// AtmosphereParameters' sun / sky radiance to relative luminance: divided
/// by the luminance of the sun with the unit radiance.
pub(crate) fn radiance_to_relative_luminance() -> ([f64; 3], [f64; 3]) {
    let sun = [98242.786222, 69954.398112, 66475.012354];
    let sky = [114974.916437, 71305.954816, 65310.548555];
    let luminance = 0.2126 * sun[0] + 0.7152 * sun[1] + 0.0722 * sun[2];
    (sun.map(|v| v / luminance), sky.map(|v| v / luminance))
}

pub(crate) struct Precomputed {
    pub transmittance: wgpu::TextureView,
    pub scattering: wgpu::TextureView,
    pub irradiance: wgpu::TextureView,
    pub higher_order_scattering: wgpu::TextureView,
    _textures: Vec<wgpu::Texture>,
}

const ADD: &str = "
@group(0) @binding(0) var a: texture_2d<f32>;
@group(0) @binding(1) var b: texture_2d<f32>;
@group(0) @binding(2) var a3: texture_3d<f32>;
@group(0) @binding(3) var b3: texture_3d<f32>;
struct Out { @builtin(position) position: vec4<f32>, @location(0) @interpolate(flat) layer: u32 }
@vertex fn vs(@builtin(vertex_index) i: u32, @builtin(instance_index) layer: u32) -> Out {
    var p = array<vec2<f32>, 3>(vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0));
    return Out(vec4<f32>(p[i], 0.0, 1.0), layer);
}
@fragment fn fs2(v: Out) -> @location(0) vec4<f32> {
    let c = vec2<i32>(v.position.xy);
    return textureLoad(a, c, 0) + textureLoad(b, c, 0);
}
@fragment fn fs3(v: Out) -> @location(0) vec4<f32> {
    let c = vec3<i32>(vec2<i32>(v.position.xy), i32(v.layer));
    return textureLoad(a3, c, 0) + textureLoad(b3, c, 0);
}";

/// The emulated One + One blending of a float32 target.
struct Add {
    layout: wgpu::BindGroupLayout,
    two: wgpu::RenderPipeline,
    three: wgpu::RenderPipeline,
    dummy2: wgpu::TextureView,
    dummy3: wgpu::TextureView,
    _dummies: Vec<wgpu::Texture>,
}

impl Add {
    fn new(r: &Renderer) -> Self {
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("precompute add"),
            source: wgpu::ShaderSource::Wgsl(ADD.into()),
        });
        let texture = |binding, dimension| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                view_dimension: dimension,
                multisampled: false,
            },
            count: None,
        };
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("precompute add"),
                entries: &[
                    texture(0, wgpu::TextureViewDimension::D2),
                    texture(1, wgpu::TextureViewDimension::D2),
                    texture(2, wgpu::TextureViewDimension::D3),
                    texture(3, wgpu::TextureViewDimension::D3),
                ],
            });
        let pipeline_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("precompute add"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
        let pipeline = |fs| {
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("precompute add"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &module,
                        entry_point: Some("vs"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module,
                        entry_point: Some(fs),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: FORMAT,
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: Default::default(),
                    depth_stencil: None,
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                })
        };
        let d2 = target(r, "precompute add dummy", (1, 1, 1), FORMAT, false);
        let d3 = target(r, "precompute add dummy", (1, 1, 1), FORMAT, true);
        Add {
            two: pipeline("fs2"),
            three: pipeline("fs3"),
            layout,
            dummy2: d2.create_view(&Default::default()),
            dummy3: d3.create_view(&Default::default()),
            _dummies: vec![d2, d3],
        }
    }
}

struct Context<'a> {
    r: &'a Renderer,
    quad: wgpu::Buffer,
    samplers: Samplers,
    add: Add,
}

/// A precomputation target with its emulated-blending scratch copies.
struct Target {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    size: (u32, u32, u32),
    three_d: bool,
}

impl Target {
    fn new(r: &Renderer, label: &str, size: (u32, u32, u32), three_d: bool) -> Self {
        let texture = target(r, label, size, FORMAT, three_d);
        let view = texture.create_view(&Default::default());
        Target {
            texture,
            view,
            size,
            three_d,
        }
    }
}

impl Context<'_> {
    /// renderer.setRenderTarget( target[, layer] ); renderer.render( quad ),
    /// for every layer of a 3D target, with the program's NoBlending or
    /// additive CustomBlending.
    fn render(
        &self,
        program: &Program,
        block: &mut super::program::Block,
        target: &Target,
        additive: bool,
        textures: &[(&str, &wgpu::TextureView)],
    ) -> Result<()> {
        let r = self.r;
        let buffer = program.uniform_buffer(r, block);
        let sampler = self.samplers.get(r, LINEAR);
        let group = program.bind(r, &buffer, &|name| {
            textures
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, view)| Bound {
                    view,
                    sampler: sampler.clone(),
                })
        })?;
        let key = PipelineKey {
            formats: vec![FORMAT],
            blend: vec![None],
            depth: None,
            cull_back: false,
        };
        let (scratch, previous) = if additive {
            let scratch = Target::new(r, "precompute scratch", target.size, target.three_d);
            let previous = Target::new(r, "precompute previous", target.size, target.three_d);
            let mut encoder = r.device.create_command_encoder(&Default::default());
            encoder.copy_texture_to_texture(
                target.texture.as_image_copy(),
                previous.texture.as_image_copy(),
                target.texture.size(),
            );
            r.queue.submit([encoder.finish()]);
            (Some(scratch), Some(previous))
        } else {
            (None, None)
        };
        let layers = if target.three_d { target.size.2 } else { 1 };
        for layer in 0..layers {
            if target.three_d {
                block.set1("layer", layer as f64);
                r.queue.write_buffer(&buffer, 0, &block.data);
            }
            let out = scratch.as_ref().unwrap_or(target);
            let mut encoder = r.device.create_command_encoder(&Default::default());
            program.draw_fullscreen(
                r,
                &mut encoder,
                &key,
                &group,
                &self.quad,
                &[Attachment {
                    view: &out.view,
                    slice: target.three_d.then_some(layer),
                    clear: Some(wgpu::Color::TRANSPARENT),
                }],
                None,
            );
            r.queue.submit([encoder.finish()]);
        }
        if let (Some(scratch), Some(previous)) = (scratch, previous) {
            let add = &self.add;
            let (a2, b2, a3, b3) = if target.three_d {
                (&add.dummy2, &add.dummy2, &previous.view, &scratch.view)
            } else {
                (&previous.view, &scratch.view, &add.dummy3, &add.dummy3)
            };
            let group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("precompute add"),
                layout: &add.layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(a2),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(b2),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(a3),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(b3),
                    },
                ],
            });
            let mut encoder = r.device.create_command_encoder(&Default::default());
            for layer in 0..layers {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("precompute add"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &target.view,
                        depth_slice: target.three_d.then_some(layer),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                pass.set_pipeline(if target.three_d { &add.three } else { &add.two });
                pass.set_bind_group(0, &group, &[]);
                pass.draw(0..3, layer..layer + 1);
            }
            r.queue.submit([encoder.finish()]);
        }
        Ok(())
    }
}

macro_rules! precompute_program {
    ($r:expr, $name:literal) => {
        Program::new(
            $r,
            $name,
            include_str!(concat!("wgsl/", $name, ".vert.wgsl")),
            include_str!(concat!("wgsl/", $name, ".frag.wgsl")),
            Vertices::Fullscreen,
            &[],
        )
    };
}

/// PrecomputedTexturesGenerator( renderer ).update(): precompute( context,
/// additive = false ) with lambdas ( 680, 550, 440 ) and an identity
/// luminanceFromRadiance.
pub(crate) fn precompute(r: &Renderer) -> Result<Precomputed> {
    if !r
        .device
        .features()
        .contains(wgpu::Features::FLOAT32_FILTERABLE)
    {
        return Err(Error::Gpu(
            "webgl_loader_3dtiles needs float32-filterable ( OES_texture_float_linear )".into(),
        ));
    }
    let context = Context {
        r,
        quad: fullscreen_buffer(r),
        samplers: Samplers::default(),
        add: Add::new(r),
    };
    let transmittance = Target::new(
        r,
        "transmittance",
        (TRANSMITTANCE.0, TRANSMITTANCE.1, 1),
        false,
    );
    let scattering = Target::new(r, "scattering", SCATTERING, true);
    let irradiance = Target::new(r, "irradiance", (IRRADIANCE.0, IRRADIANCE.1, 1), false);
    let higher_order = Target::new(r, "higher order scattering", SCATTERING, true);
    let delta_irradiance = Target::new(
        r,
        "delta irradiance",
        (IRRADIANCE.0, IRRADIANCE.1, 1),
        false,
    );
    // deltaMultipleScattering shares deltaRayleighScattering's texture.
    let delta_rayleigh = Target::new(r, "delta rayleigh scattering", SCATTERING, true);
    let delta_mie = Target::new(r, "delta mie scattering", SCATTERING, true);
    let delta_density = Target::new(r, "delta scattering density", SCATTERING, true);
    let delta_multiple = &delta_rayleigh;
    let block = |p: &Program| {
        let mut b = p.block();
        set_atmosphere(&mut b, "ATMOSPHERE");
        b.set(
            "luminanceFromRadiance",
            &[1., 0., 0., 0., 1., 0., 0., 0., 1.],
        );
        b
    };
    let t = &transmittance.view;
    // computeTransmittance.
    let p = precompute_program!(r, "transmittance")?;
    context.render(&p, &mut block(&p), &transmittance, false, &[])?;
    // computeDirectIrradiance into deltaIrradiance and irradiance.
    let p = precompute_program!(r, "direct_irradiance_delta")?;
    context.render(
        &p,
        &mut block(&p),
        &delta_irradiance,
        false,
        &[("transmittanceTexture", t)],
    )?;
    let p = precompute_program!(r, "direct_irradiance")?;
    context.render(
        &p,
        &mut block(&p),
        &irradiance,
        false,
        &[("transmittanceTexture", t)],
    )?;
    // computeSingleScattering into deltaRayleigh, deltaMie and scattering.
    let p = precompute_program!(r, "single_scattering_delta_rayleigh")?;
    context.render(
        &p,
        &mut block(&p),
        &delta_rayleigh,
        false,
        &[("transmittanceTexture", t)],
    )?;
    let p = precompute_program!(r, "single_scattering_delta_mie")?;
    context.render(
        &p,
        &mut block(&p),
        &delta_mie,
        false,
        &[("transmittanceTexture", t)],
    )?;
    let p = precompute_program!(r, "single_scattering")?;
    context.render(
        &p,
        &mut block(&p),
        &scattering,
        false,
        &[("transmittanceTexture", t)],
    )?;
    let density = precompute_program!(r, "scattering_density")?;
    let indirect_delta = precompute_program!(r, "indirect_irradiance_delta")?;
    let indirect = precompute_program!(r, "indirect_irradiance")?;
    let multiple_delta = precompute_program!(r, "multiple_scattering_delta")?;
    let multiple = precompute_program!(r, "multiple_scattering")?;
    for order in 2..=4 {
        let order = order as f64;
        let mut b = block(&density);
        b.set1("scatteringOrder", order);
        context.render(
            &density,
            &mut b,
            &delta_density,
            false,
            &[
                ("transmittanceTexture", t),
                ("singleRayleighScatteringTexture", &delta_rayleigh.view),
                ("singleMieScatteringTexture", &delta_mie.view),
                ("multipleScatteringTexture", &delta_multiple.view),
                ("irradianceTexture", &delta_irradiance.view),
            ],
        )?;
        let sources = [
            ("singleRayleighScatteringTexture", &delta_rayleigh.view),
            ("singleMieScatteringTexture", &delta_mie.view),
            ("multipleScatteringTexture", &delta_multiple.view),
        ];
        let mut b = block(&indirect_delta);
        b.set1("scatteringOrder", order - 1.);
        context.render(&indirect_delta, &mut b, &delta_irradiance, false, &sources)?;
        let mut b = block(&indirect);
        b.set1("scatteringOrder", order - 1.);
        context.render(&indirect, &mut b, &irradiance, true, &sources)?;
        let sources = [
            ("transmittanceTexture", t),
            ("scatteringDensityTexture", &delta_density.view),
        ];
        context.render(
            &multiple_delta,
            &mut block(&multiple_delta),
            delta_multiple,
            false,
            &sources,
        )?;
        context.render(
            &multiple,
            &mut block(&multiple),
            &scattering,
            true,
            &sources,
        )?;
        context.render(
            &multiple,
            &mut block(&multiple),
            &higher_order,
            true,
            &sources,
        )?;
    }
    Ok(Precomputed {
        transmittance: transmittance.view,
        scattering: scattering.view,
        irradiance: irradiance.view,
        higher_order_scattering: higher_order.view,
        _textures: vec![
            transmittance.texture,
            scattering.texture,
            irradiance.texture,
            higher_order.texture,
        ],
    })
}
