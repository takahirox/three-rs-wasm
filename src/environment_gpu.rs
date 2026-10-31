//! three.js r187's PMREMGenerator ( WebGL, src/extras/PMREMGenerator.js ),
//! entirely on the GPU. A PMREM is a half-float cube whose levels, down to 8
//! texels ( LOD_MIN 3 ), hold the radiance prefiltered for roughness
//! `1 - sqrt( 1 - lod / maxLod )`; materials sample it with
//! `roughnessToMip` ( shaders/envmap.wgsl ). The input is drawn into a
//! source cube of the same size ( at least 256 ): an equirectangular map
//! through the box's four subpixel samples, or a cube; fromScene's blur, when
//! its sigma is above zero, follows. The source then gets its mip chain, as
//! generateMipmap's box filter makes it, and each level of the PMREM is drawn
//! from it: GGX VNDF importance sampling ( 256 samples ) for the sharp levels
//! and the integration over the 16-texel source mip for the three roughest
//! ( shaders/pmrem.wgsl ). The targets, the passes and their uniforms are
//! built once, so regenerating ( a CubeCamera's PMREM every frame ) allocates
//! nothing.
use crate::{Error, Result, environment::EnvironmentMap};
use wgpu::util::DeviceExt;
pub(crate) type GpuEnvironment = crate::environment::PrefilteredEnvironment;

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
/// Smaller inputs are upsampled so that every PMREM has enough mip levels.
const MIN_SIZE: u32 = 256;
/// Log2 of the face size of the roughest level.
const LOD_MIN: u32 = 3;
const GGX_SAMPLES: f64 = 256.;
const INTEGRATION_SIZE: u32 = 16;
const INTEGRATION_LEVELS: u32 = 3;
/// Each pass's uniforms, at the device's offset alignment.
const STRIDE: u64 = 256;

/// PMREMGenerator._setSize: a power of two, at least 256.
pub(crate) fn cube_size(size: u32) -> u32 {
    MIN_SIZE.max(1 << (31 - size.max(1).leading_zeros()))
}
/// PMREMGenerator.lodToRoughness.
pub fn lod_to_roughness(lod: u32, max_lod: u32) -> f64 {
    if max_lod > 0 {
        1. - (1. - lod as f64 / max_lod as f64).sqrt()
    } else {
        0.
    }
}

/// What the source cube is drawn from.
pub(crate) enum Input<'a> {
    /// An equirectangular map ( PMREMEquirectangularToCubemap ).
    Equirect(&'a wgpu::TextureView),
    /// A cube in the engine's orientation ( PMREMCubemapToCubemap ).
    Cube(&'a wgpu::TextureView),
}

fn cube_texture(device: &wgpu::Device, label: &str, size: u32, levels: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 6,
        },
        mip_level_count: levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: FORMAT,
        // COPY_SRC lets exporters read a PMREM back, as readRenderTargetPixels does.
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}
fn cube_view(texture: &wgpu::Texture) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::Cube),
        ..Default::default()
    })
}
fn face_view(texture: &wgpu::Texture, face: u32, level: u32) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2),
        base_mip_level: level,
        mip_level_count: Some(1),
        base_array_layer: face,
        array_layer_count: Some(1),
        ..Default::default()
    })
}
/// The PMREM texture's sampler: LinearMipmapLinearFilter, clamped.
pub(crate) fn pmrem_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("PMREM"),
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    })
}

#[derive(Clone, Copy, PartialEq)]
enum Stage {
    Equirect,
    Cube,
    Mipmap,
    Blur,
    Ggx,
    Integration,
}
impl Stage {
    fn entry(self) -> &'static str {
        match self {
            Stage::Equirect => "fs_equirect",
            Stage::Cube => "fs_cube",
            Stage::Mipmap => "fs_mipmap",
            Stage::Blur => "fs_blur",
            Stage::Ggx => "fs_ggx",
            Stage::Integration => "fs_integration",
        }
    }
}

/// The passes of one PMREM, built once and drawn on every regeneration.
pub(crate) struct Generator {
    pipelines: Vec<wgpu::RenderPipeline>,
    passes: Vec<(usize, wgpu::BindGroup, wgpu::TextureView)>,
}
impl Generator {
    /// The PMREM of `size` ( rounded as _setSize does ) from `input`, blurred
    /// by `sigma` radians first when above zero ( fromScene ).
    pub(crate) fn new(
        device: &wgpu::Device,
        size: u32,
        input: Input,
        sigma: f32,
    ) -> Result<(wgpu::Texture, Self)> {
        let size = cube_size(size);
        if size > device.limits().max_texture_dimension_2d {
            return Err(Error::Invalid("PMREM size"));
        }
        let max_lod = size.ilog2() - LOD_MIN;
        let pmrem = cube_texture(device, "PMREM", size, max_lod + 1);
        let source_levels = size.ilog2() + 1;
        let source = cube_texture(device, "PMREM source", size, source_levels);
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("PMREMGenerator"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/pmrem.wgsl").into()),
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("PMREM passes"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // ( stage, face, level, uniforms, the source view, the target )
        let mut plan: Vec<(
            Stage,
            u32,
            u32,
            [f32; 4],
            wgpu::TextureView,
            wgpu::TextureView,
        )> = vec![];
        let (input_stage, input_view) = match input {
            Input::Equirect(view) => (Stage::Equirect, view.clone()),
            Input::Cube(view) => (Stage::Cube, view.clone()),
        };
        for face in 0..6 {
            plan.push((
                input_stage,
                face,
                0,
                [0.; 4],
                input_view.clone(),
                face_view(&source, face, 0),
            ));
        }
        let source_cube = cube_view(&source);
        let pmrem_cube = cube_view(&pmrem);
        if sigma > 0. {
            // _blur: the variance split between two passes, through level 0
            // of the PMREM; sigma clamped to the sphere's angular distance.
            let sigma = sigma.min(std::f32::consts::PI) / std::f32::consts::SQRT_2;
            for face in 0..6 {
                plan.push((
                    Stage::Blur,
                    face,
                    0,
                    [0., 0., 0., sigma],
                    source_cube.clone(),
                    face_view(&pmrem, face, 0),
                ));
            }
            for face in 0..6 {
                plan.push((
                    Stage::Blur,
                    face,
                    0,
                    [0., 0., 0., sigma],
                    pmrem_cube.clone(),
                    face_view(&source, face, 0),
                ));
            }
        }
        for level in 1..source_levels {
            for face in 0..6 {
                plan.push((
                    Stage::Mipmap,
                    face,
                    level,
                    [0.; 4],
                    face_view(&source, face, level - 1),
                    face_view(&source, face, level),
                ));
            }
        }
        // _applyPMREM.
        let source_lod = (size / INTEGRATION_SIZE).ilog2() as f32;
        for lod in 0..=max_lod {
            let roughness = lod_to_roughness(lod, max_lod);
            let (stage, uniforms) = if lod + INTEGRATION_LEVELS > max_lod {
                (Stage::Integration, [roughness as f32, 0., source_lod, 0.])
            } else {
                // For V = N, pdf( L ) = D( H ) / 4: the source texel's solid
                // angle matched to 1 / ( GGX_SAMPLES * pdf ), half a mip up.
                let bias = if roughness > 0. {
                    (size as f64).log2()
                        + 0.5 * (6. / (GGX_SAMPLES * roughness.powi(4))).log2()
                        + 0.5
                } else {
                    0.
                };
                (Stage::Ggx, [roughness as f32, bias as f32, 0., 0.])
            };
            for face in 0..6 {
                plan.push((
                    stage,
                    face,
                    lod,
                    uniforms,
                    source_cube.clone(),
                    face_view(&pmrem, face, lod),
                ));
            }
        }
        let mut data = vec![0u8; plan.len() * STRIDE as usize];
        for (i, (_, face, level, [roughness, lod_bias, source_lod, sigma], ..)) in
            plan.iter().enumerate()
        {
            let at = i * STRIDE as usize;
            let words: [u32; 8] = [
                *face,
                ((size >> level) as f32).to_bits(),
                roughness.to_bits(),
                lod_bias.to_bits(),
                source_lod.to_bits(),
                sigma.to_bits(),
                INTEGRATION_SIZE,
                0,
            ];
            data[at..at + 32].copy_from_slice(bytemuck::cast_slice(&words));
        }
        let uniforms = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("PMREM passes"),
            contents: &data,
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let mut pipelines: Vec<(Stage, wgpu::RenderPipeline)> = vec![];
        let mut passes = vec![];
        for (i, (stage, _, _, _, view, target)) in plan.into_iter().enumerate() {
            let slot = match pipelines.iter().position(|(s, _)| *s == stage) {
                Some(slot) => slot,
                None => {
                    pipelines.push((stage, pipeline(device, &module, stage)));
                    pipelines.len() - 1
                }
            };
            let layout = pipelines[slot].1.get_bind_group_layout(0);
            let mut entries = vec![wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&view),
            }];
            // The box filter loads texels: no uniforms, no sampler.
            if !matches!(stage, Stage::Mipmap) {
                entries.extend([
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &uniforms,
                            offset: i as u64 * STRIDE,
                            size: wgpu::BufferSize::new(32),
                        }),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ]);
            }
            let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("PMREM pass"),
                layout: &layout,
                entries: &entries,
            });
            passes.push((slot, bindings, target));
        }
        Ok((
            pmrem,
            Self {
                pipelines: pipelines.into_iter().map(|(_, p)| p).collect(),
                passes,
            },
        ))
    }
    pub(crate) fn encode(&self, encoder: &mut wgpu::CommandEncoder) {
        for (pipeline, bindings, target) in &self.passes {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("PMREM"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipelines[*pipeline]);
            pass.set_bind_group(0, bindings, &[]);
            pass.draw(0..3, 0..1);
        }
    }
}
fn pipeline(
    device: &wgpu::Device,
    module: &wgpu::ShaderModule,
    stage: Stage,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(stage.entry()),
        layout: None,
        vertex: wgpu::VertexState {
            module,
            entry_point: Some("vs"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module,
            entry_point: Some(stage.entry()),
            compilation_options: Default::default(),
            targets: &[Some(FORMAT.into())],
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview: None,
        cache: None,
    })
}

/// A PMREM from `input`, generated once.
fn generate(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    size: u32,
    input: Input,
    sigma: f32,
    source: Option<wgpu::TextureView>,
) -> Result<GpuEnvironment> {
    let (texture, generator) = Generator::new(device, size, input, sigma)?;
    let mut encoder = device.create_command_encoder(&Default::default());
    generator.encode(&mut encoder);
    queue.submit([encoder.finish()]);
    Ok(environment(device, texture, source, false))
}
/// The environment over a PMREM cube; `source` is the equirectangular map
/// when there is one ( the sharp background samples it ).
fn environment(
    device: &wgpu::Device,
    texture: wgpu::Texture,
    source: Option<wgpu::TextureView>,
    flip: bool,
) -> GpuEnvironment {
    let pmrem_background = source.is_none();
    GpuEnvironment {
        view: cube_view(&texture),
        source: source.unwrap_or_else(|| face_view(&texture, 0, 0)),
        sampler: pmrem_sampler(device),
        max_mip: (texture.mip_level_count() - 1) as f32,
        pmrem_background,
        flip,
        texture,
    }
}

/// fromEquirectangular: the PMREM of a linear HDR equirectangular map.
pub(crate) fn build(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    image: &EnvironmentMap,
) -> Result<GpuEnvironment> {
    if let Some(gpu) = &image.gpu {
        return Ok(gpu.clone());
    }
    if image.width < 64
        || image.height == 0
        || image.rgba.len() != image.width as usize * image.height as usize * 4
        || image.height > device.limits().max_texture_dimension_2d
        || image.width > device.limits().max_texture_dimension_2d
    {
        return Err(Error::Invalid("HDR dimensions"));
    }
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("HDR equirectangular"),
        size: wgpu::Extent3d {
            width: image.width,
            height: image.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        texture.as_image_copy(),
        bytemuck::cast_slice(&image.rgba),
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(image.width * 8),
            rows_per_image: Some(image.height),
        },
        texture.size(),
    );
    let source = texture.create_view(&Default::default());
    generate(
        device,
        queue,
        image.width / 4,
        Input::Equirect(&source),
        0.,
        Some(source.clone()),
    )
}

/// CubeCamera's six faces, in the engine's cube orientation ( the cube
/// holds the direction ( -x, y, z ) at x, as an uploaded CubeTexture does ):
/// each face's look direction and up.
fn cube_faces() -> [(crate::math::Vector3, crate::math::Vector3); 6] {
    use crate::math::Vector3;
    // CubeCamera uses negative FOV; reversing its up vectors is equivalent.
    [
        (Vector3::NEG_X, Vector3::Y),
        (Vector3::X, Vector3::Y),
        (Vector3::Y, Vector3::NEG_Z),
        (Vector3::NEG_Y, Vector3::Z),
        (Vector3::Z, Vector3::Y),
        (Vector3::NEG_Z, Vector3::Y),
    ]
}
/// Render `scene` into the six layers of `target` from `camera` at `position`.
fn capture(
    renderer: &crate::renderer::Renderer,
    scene: &mut crate::scene::Scene,
    camera: crate::scene::Object3D,
    position: crate::math::Vector3,
    target: &mut crate::render_target::RenderTarget,
) -> Result<()> {
    scene.get_mut(camera)?.position = position;
    for (i, (direction, up)) in cube_faces().into_iter().enumerate() {
        scene.get_mut(camera)?.up = up;
        scene.look_at(camera, position + direction)?;
        target.set_layer(i as u32)?;
        renderer.render(scene, camera, target)?;
    }
    Ok(())
}
fn capture_target(device: &wgpu::Device, size: u32) -> Result<crate::render_target::RenderTarget> {
    use crate::render_target::*;
    RenderTarget::with_options(
        device,
        size,
        size,
        RenderTargetOptions {
            depth: 6,
            format: wgpu::TextureFormat::Rgba16Float,
            ..Default::default()
        },
    )
}
fn capture_camera(scene: &mut crate::scene::Scene, near: f64, far: f64) -> crate::scene::Object3D {
    use crate::{camera::*, scene::*};
    scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 90.0,
        aspect: 1.0,
        near,
        far,
        ..Default::default()
    })))
}

impl EnvironmentMap {
    /// fromScene( scene, sigma ): a CubeCamera capture at the origin ( near
    /// 0.1, far 100 ) of `size` texels, blurred by `sigma` radians, then its
    /// PMREM. CPU readback is never used. Call again only when the source
    /// scene changes.
    pub fn from_scene(
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        size: u32,
        sigma: f32,
    ) -> Result<Self> {
        if size < 16
            || !size.is_power_of_two()
            || size > renderer.device.limits().max_texture_dimension_2d
            || !sigma.is_finite()
            || sigma < 0.0
        {
            return Err(Error::Invalid("environment capture size/sigma"));
        }
        Self::capture_pmrem(
            renderer,
            scene,
            crate::math::Vector3::ZERO,
            0.1,
            100.,
            size,
            sigma,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn capture_pmrem(
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        position: crate::math::Vector3,
        near: f64,
        far: f64,
        size: u32,
        sigma: f32,
    ) -> Result<Self> {
        // The capture is the PMREM's size, as fromScene's source target is.
        let size = cube_size(size);
        let mut target = capture_target(&renderer.device, size)?;
        let camera = capture_camera(scene, near, far);
        let result = capture(renderer, scene, camera, position, &mut target).and_then(|()| {
            generate(
                &renderer.device,
                &renderer.queue,
                size,
                Input::Cube(&cube_view(&target.texture)),
                sigma,
                None,
            )
        });
        scene.dispose(camera)?;
        Ok(Self::resident(result?))
    }
    fn resident(gpu: GpuEnvironment) -> Self {
        let size = gpu.texture.size();
        Self {
            width: size.width,
            height: size.height,
            rgba: vec![],
            gpu: Some(gpu),
        }
    }
}

impl EnvironmentMap {
    /// fromCubemap on six linear HDR faces (+X,-X,+Y,-Y,+Z,-Z), entirely on
    /// the GPU. `rgba` stores all faces consecutively.
    pub fn from_cube_hdr(
        renderer: &crate::renderer::Renderer,
        size: u32,
        rgba: &[half::f16],
    ) -> Result<Self> {
        let cube = crate::texture_gpu::GpuTexture::from_cube_hdr(renderer, size, rgba)?;
        Self::from_cube_texture(renderer, &cube)
    }
    /// fromCubemap on a resident cube, sharing the original upload with a sky.
    pub fn from_cube_texture(
        renderer: &crate::renderer::Renderer,
        cube: &crate::texture_gpu::GpuTexture,
    ) -> Result<Self> {
        let size = cube.texture.width();
        if size == 0
            || size > renderer.device.limits().max_texture_dimension_2d
            || cube.texture.height() != size
            || cube.texture.depth_or_array_layers() != 6
        {
            return Err(Error::Invalid("PMREM cube dimensions"));
        }
        let view = cube.texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        Ok(Self::resident(generate(
            &renderer.device,
            &renderer.queue,
            size,
            Input::Cube(&view),
            0.,
            None,
        )?))
    }
}

impl EnvironmentMap {
    /// Capture a CubeCamera at a world-space origin, then generate its PMREM
    /// ( fromCubemap on its target ). The six render passes and the PMREM
    /// stay on the GPU.
    pub fn from_cube_scene(
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        position: crate::math::Vector3,
        near: f64,
        far: f64,
        size: u32,
    ) -> Result<Self> {
        if !position.is_finite()
            || !near.is_finite()
            || !far.is_finite()
            || near <= 0.0
            || far <= near
            || size < 16
            || !size.is_power_of_two()
            || size > renderer.device.limits().max_texture_dimension_2d
        {
            return Err(Error::Invalid("cube capture camera/size"));
        }
        // The CubeCamera's target keeps its own size; the PMREM is at least 256.
        let mut target = capture_target(&renderer.device, size)?;
        let camera = capture_camera(scene, near, far);
        let result = capture(renderer, scene, camera, position, &mut target).and_then(|()| {
            generate(
                &renderer.device,
                &renderer.queue,
                size,
                Input::Cube(&cube_view(&target.texture)),
                0.,
                None,
            )
        });
        scene.dispose(camera)?;
        Ok(Self::resident(result?))
    }
}

/// A CubeCamera's resident capture and its PMREM ( WebGLEnvironments'
/// fromCubemap on the cube render target, regenerated after each update ):
/// the six faces render into one half-float cube target, then the PMREM
/// passes run, all resident, so regenerating it every frame allocates
/// nothing. `environment` stays the same map, for materials' envMap.
pub struct CubeCapture {
    target: crate::render_target::RenderTarget,
    generator: Generator,
    pub environment: std::sync::Arc<EnvironmentMap>,
}
impl CubeCapture {
    pub fn new(renderer: &crate::renderer::Renderer, size: u32) -> Result<Self> {
        let device = &renderer.device;
        if size < 16 || !size.is_power_of_two() || size > device.limits().max_texture_dimension_2d {
            return Err(Error::Invalid("cube capture size"));
        }
        let target = capture_target(device, size)?;
        let (texture, generator) =
            Generator::new(device, size, Input::Cube(&cube_view(&target.texture)), 0.)?;
        Ok(Self {
            target,
            generator,
            environment: std::sync::Arc::new(EnvironmentMap::resident(environment(
                device, texture, None, false,
            ))),
        })
    }
    /// CubeCamera.update(): the six faces from `camera` ( a 90° perspective
    /// camera node in `scene` ) at `position`, then the PMREM.
    pub fn update(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        position: crate::math::Vector3,
    ) -> Result<()> {
        capture(renderer, scene, camera, position, &mut self.target)?;
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        self.generator.encode(&mut encoder);
        renderer.queue.submit([encoder.finish()]);
        Ok(())
    }
}

/// A prefiltered PMREM cube stored as UASTC HDR KTX2 ( r187's
/// `textures/pmrem/*.pmrem.ktx2`: six 512-texel faces and seven levels ),
/// used as it is: KTX2Loader's transcode, then `isPMREMTexture` without
/// filtering again. The transcode target follows KTX2Loader's order for
/// UASTC HDR: BC6H where the device has BC textures, else half floats
/// ( WebGPU has no ASTC HDR ). As an uploaded cube texture it is sampled
/// with x mirrored ( WebGLMaterials' envMapRotation ).
pub(crate) fn pmrem_ktx2(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bytes: &[u8],
) -> Result<GpuEnvironment> {
    #[cfg(not(feature = "basisu"))]
    {
        let _ = (device, queue, bytes);
        Err(crate::Error::Asset(crate::BASIS_DISABLED.into()))
    }
    #[cfg(feature = "basisu")]
    {
        use basisu::{SourceFormat, TargetFormat};
        let t =
            basisu::Transcoder::new(bytes).map_err(|e| Error::Asset(format!("Basis: {e:?}")))?;
        let (width, height) = t.base_dimensions();
        let levels = t.level_count();
        if t.source_format() != SourceFormat::UastcHdr4x4
            || t.face_count() != 6
            || width != height
            || !width.is_power_of_two()
            || levels == 0
            || levels > width.ilog2() + 1
        {
            return Err(Error::Invalid("PMREM KTX2: a UASTC HDR cube"));
        }
        let (target, format) = if device
            .features()
            .contains(wgpu::Features::TEXTURE_COMPRESSION_BC)
        {
            (TargetFormat::Bc6h, wgpu::TextureFormat::Bc6hRgbUfloat)
        } else {
            (TargetFormat::RgbaHalf, wgpu::TextureFormat::Rgba16Float)
        };
        // Face by face, each face's levels: LayerMajor's order.
        let mut data = vec![vec![]; 6];
        for level in 0..levels {
            for (face, out) in data.iter_mut().enumerate() {
                out.extend(crate::basis::transcode_image(
                    &t,
                    level,
                    0,
                    face as u32,
                    target,
                    basisu::DecodeFlags::NONE,
                )?);
            }
        }
        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("PMREM KTX2"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 6,
                },
                mip_level_count: levels,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &data.concat(),
        );
        Ok(environment(device, texture, None, true))
    }
}
