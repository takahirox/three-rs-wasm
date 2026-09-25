pub use crate::texture_gpu::GpuTexture;
use crate::{Error, Result, material::*, math::*, scene::*};
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Weak},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PipelineKey {
    shader_id: u64,
    material_kind: u8,
    texture_mask: u8,
    extension_mask: u16,
    light_count: u8,
    light_types: u32,
    receive_shadow: bool,
    physical: bool,
    instanced: bool,
    clipping: bool,
    dashed: bool,
    blend: Option<wgpu::BlendState>,
    attachment_blending: Vec<Option<wgpu::BlendState>>,
    stencil: Option<wgpu::StencilState>,
    /// Depth bias: constant units and the slope factor's bits.
    depth_bias: (i32, u32),
    color_write: bool,
    topology: wgpu::PrimitiveTopology,
    side: u8,
    transparent: bool,
    alpha_mask: bool,
    alpha_to_coverage: bool,
    encode_srgb: bool,
    depth_test: bool,
    depth_write: bool,
    samples: u32,
    format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    attachment_formats: Vec<wgpu::TextureFormat>,
    mirrored: bool,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) normal: [f32; 3],
    pub(crate) uv: [f32; 2],
    pub(crate) color: [f32; 4],
    pub(crate) corner: [f32; 2],
    pub(crate) tangent: [f32; 4],
    pub(crate) uv1: [f32; 2],
}
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct InstanceVertex {
    pub(crate) matrix: [f32; 16],
    pub(crate) color: [f32; 4],
}
pub(crate) fn instance_layout(instanced: bool) -> wgpu::VertexBufferLayout<'static> {
    const ATTRIBUTES: [wgpu::VertexAttribute; 5] =
        wgpu::vertex_attr_array![6=>Float32x4,7=>Float32x4,8=>Float32x4,9=>Float32x4,10=>Float32x4];
    wgpu::VertexBufferLayout {
        array_stride: if instanced { 80 } else { 0 },
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &ATTRIBUTES,
    }
}
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [f32; 16],
    model: [f32; 16],
    normal: [f32; 16],
    view: [f32; 16],
    projection: [f32; 16],
    color: [f32; 4],
    camera: [f32; 4],
    material: [f32; 4],
    emissive: [f32; 4],
    ambient: [f32; 4],
    point: [f32; 4],
    pbr: [f32; 4],
    environment: [f32; 4],
    maps: [f32; 4],
    light_position: [[f32; 4]; 8],
    light_color: [[f32; 4]; 8],
    light_params: [[f32; 4]; 8],
    light_direction: [[f32; 4]; 8],
    specular: [f32; 4],
    flags: [f32; 4],
    fog_color: [f32; 4],
    fog_params: [f32; 4],
    shadow_matrices: [[f32; 16]; 48],
    shadow_params: [[f32; 4]; 8],
    shadow_filters: [[f32; 4]; 8],
    shadow_cascades: [[f32; 4]; 16],
    custom: [[f32; 4]; 16],
    clipping: crate::clipping::Clipping,
    physical: [[f32; 4]; 4],
    uv_transforms: [[f32; 4]; 15],
    transmission: [[f32; 4]; 3],
    extension_maps: crate::physical_maps::MapUniforms,
    coat_normal: [f32; 4],
    iridescence: [f32; 4],
    line: [[f32; 4]; 2],
    output: [f32; 4],
}

pub use crate::render_target::{RenderTarget, RenderTarget3D, RenderTargetOptions};

pub struct Renderer {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    layout: wgpu::BindGroupLayout,
    viewport_layout: wgpu::BindGroupLayout,
    shader: wgpu::ShaderModule,
    pub(crate) shadows: crate::shadow::ShadowRenderer,
    geometry: RefCell<crate::geometry_gpu::Cache>,
    deformation: RefCell<crate::deformation_gpu::Cache>,
    pipelines: RefCell<HashMap<PipelineKey, wgpu::RenderPipeline>>,
    textures: RefCell<crate::texture_gpu::TextureCache>,
    white: Arc<Texture>,
    environment: RefCell<
        Option<(
            Weak<crate::environment::EnvironmentMap>,
            crate::environment_gpu::GpuEnvironment,
        )>,
    >,
    dfg: wgpu::TextureView,
    ltc: wgpu::TextureView,
    ltc_sampler: wgpu::Sampler,
    physical_maps: RefCell<crate::physical_maps::Cache>,
    backgrounds: RefCell<crate::background::PipelineCache>,
    presentations:
        RefCell<HashMap<wgpu::TextureFormat, (wgpu::BindGroupLayout, wgpu::RenderPipeline)>>,
    environment_builds: std::cell::Cell<u64>,
    viewport: RefCell<Option<crate::viewport::Snapshot>>,
    transmission_target: RefCell<Option<RenderTarget>>,
    transmission_sampler: wgpu::Sampler,
    transmission_mips: RefCell<Option<crate::transmission::MipChain>>,
    draw_slots: RefCell<DrawSlots>,
    scene_draw_slots: RefCell<HashMap<u32, (std::sync::Weak<()>, DrawSlots)>>,
    present_slots: RefCell<Vec<(wgpu::Texture, crate::draw_gpu::Slot)>>,
}
#[derive(Clone, Copy, PartialEq)]
enum ScenePass {
    All,
    Opaque,
    Transparent,
}
type DrawSlots = HashMap<(Object3D, usize, bool), crate::draw_gpu::Slot>;
struct DrawGeometry<'a> {
    key: (Object3D, usize, bool),
    vertices: wgpu::Buffer,
    deformation: crate::deformation_gpu::Bindings,
    instances: &'a [Instance],
    transmission_view: Option<&'a wgpu::TextureView>,
}
#[derive(Clone)]
struct Draw {
    viewport: u8,
    object: Object3D,
    stencil_reference: u32,
    custom_bindings: Option<wgpu::BindGroup>,
    instance_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    back_pipeline: Option<wgpu::RenderPipeline>,
    vertices: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    range: std::ops::Range<u32>,
    instances: u32,
    indices: Option<wgpu::Buffer>,
    indirect: Option<wgpu::Buffer>,
    indirect_offsets: Vec<u64>,
    transparent: bool,
    render_order: i32,
    group_order: i32,
    depth: f64,
}
impl Renderer {
    pub(crate) fn validate_shader_program(
        &self,
        module: &wgpu::ShaderModule,
        extra: &wgpu::BindGroupLayout,
        outputs: u32,
        viewport: bool,
    ) {
        let layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    if viewport {
                        &self.viewport_layout
                    } else {
                        &self.layout
                    },
                    extra,
                ],
                push_constant_ranges: &[],
            });
        let _ = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("validate custom mesh shader"),layout:Some(&layout),vertex:wgpu::VertexState {module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:std::mem::size_of::<Vertex>() as u64,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x2,3=>Float32x4,4=>Float32x2,5=>Float32x4,11=>Float32x2]},instance_layout(true)]},fragment:Some(wgpu::FragmentState {module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&vec![Some(wgpu::ColorTargetState {format:wgpu::TextureFormat::Rgba8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL});outputs as usize]}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview:None,cache:None});
    }
    /// Upload once and retain a sampled texture for custom material/effect bindings.
    pub fn upload_texture(&self, image: &Arc<crate::material::Texture>) -> Result<GpuTexture> {
        self.textures
            .borrow_mut()
            .get(&self.device, &self.queue, image)
    }
    /// Cumulative geometry uploads/bytes, static skin/morph bytes, and pose bytes.
    /// A pose-only frame must not increase the first three counters.
    pub fn transfer_counts(&self) -> (u64, u64, u64, u64) {
        let g = self.geometry.borrow();
        let d = self.deformation.borrow();
        (g.uploads, g.bytes, d.static_bytes, d.pose_bytes)
    }
    /// (resident material textures, cumulative uploads, cumulative environment filters).
    pub fn resource_counts(&self) -> (usize, u64, u64) {
        let cache = self.textures.borrow();
        (cache.len(), cache.uploads, self.environment_builds.get())
    }
    /// Release cached resources whose application-owned inputs have been dropped.
    pub fn collect_resources(&self) {
        self.draw_slots.borrow_mut().clear();
        self.scene_draw_slots.borrow_mut().clear();
        self.shadows.collect_resources();
        for (_, _, slot) in self.backgrounds.borrow_mut().values_mut() {
            *slot = Default::default();
        }
        self.present_slots.borrow_mut().clear();
        self.viewport.borrow_mut().take();
        self.geometry.borrow_mut().prune();
        self.textures.borrow_mut().prune();
        self.physical_maps.borrow_mut().prune();
        if self
            .environment
            .borrow()
            .as_ref()
            .is_some_and(|(owner, _)| owner.strong_count() == 0)
        {
            *self.environment.borrow_mut() = None;
        }
    }
    pub async fn new() -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY | wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .map_err(|e| Error::Gpu(e.to_string()))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("three-rs-wasm"),
                required_features: adapter.features()
                    & (wgpu::Features::FLOAT32_FILTERABLE
                        | wgpu::Features::INDIRECT_FIRST_INSTANCE
                        | wgpu::Features::TEXTURE_COMPRESSION_BC
                        | wgpu::Features::TEXTURE_COMPRESSION_ETC2
                        | wgpu::Features::TEXTURE_COMPRESSION_ASTC),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            })
            .await
            .map_err(|e| Error::Gpu(e.to_string()))?;
        let mut entries = vec![wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }];
        for binding in 1..=14 {
            let sampler = binding <= 10 && binding % 2 == 0 || binding == 13;
            entries.push(wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: if sampler {
                    wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
                } else {
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    }
                },
                count: None,
            });
        }
        entries.extend([
            wgpu::BindGroupLayoutEntry {
                binding: 15,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 16,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                count: None,
            },
        ]);
        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 17,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2Array,
                multisampled: false,
            },
            count: None,
        });
        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 18,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        });
        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 19,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });
        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 20,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2Array,
                multisampled: false,
            },
            count: None,
        });
        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 24,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });
        entries.extend(crate::deformation_gpu::layout_entries());
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("draw layout"),
            entries: &entries,
        });
        for binding in [25, 26] {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: binding == 25,
                    },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            });
        }
        let viewport_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viewport draw layout"),
            entries: &entries,
        });
        let dfg = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("r186 DFG"),
            size: wgpu::Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rg16Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            dfg.as_image_copy(),
            include_bytes!("shaders/dfg-r186.bin"),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(64),
                rows_per_image: Some(16),
            },
            dfg.size(),
        );
        let dfg = dfg.create_view(&Default::default());
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("materials"),
            source: wgpu::ShaderSource::Wgsl(
                format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}",
                    include_str!("shaders/cube_uv.wgsl"),
                    concat!(
                        include_str!("shaders/deformation.wgsl"),
                        "\n",
                        include_str!("shaders/output.wgsl"),
                        "\n",
                        include_str!("shader.wgsl")
                    ),
                    crate::shader::DEFAULT_HOOKS,
                    crate::shader::DEFAULT_OUTPUT,
                    crate::shader::DEFAULT_PROJECTION,
                    crate::shader::DEFAULT_SURFACE,
                    crate::shader::DEFAULT_ENVIRONMENT
                )
                .into(),
            ),
        });
        let shadows = crate::shadow::ShadowRenderer::new(&device);
        let ltc = crate::area_light::texture(&device, &queue);
        let ltc_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("LTC linear magnification, nearest minification"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let transmission_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("viewport refraction sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Ok(Self {
            shadows,
            geometry: Default::default(),
            deformation: Default::default(),
            adapter,
            device,
            queue,
            layout,
            viewport_layout,
            shader,
            pipelines: RefCell::new(HashMap::new()),
            textures: RefCell::new(Default::default()),
            white: Arc::new(Texture::from_rgba(1, 1, vec![255; 4], false)?),
            environment: RefCell::new(None),
            dfg,
            ltc,
            ltc_sampler,
            physical_maps: Default::default(),
            backgrounds: RefCell::new(Default::default()),
            presentations: RefCell::new(Default::default()),
            environment_builds: std::cell::Cell::new(0),
            transmission_sampler,
            viewport: Default::default(),
            transmission_target: Default::default(),
            transmission_mips: Default::default(),
            draw_slots: Default::default(),
            scene_draw_slots: Default::default(),
            present_slots: Default::default(),
        })
    }

    /// Present an offscreen color target to a canvas surface view.
    pub fn blit(
        &self,
        target: &RenderTarget,
        view: &wgpu::TextureView,
        format: wgpu::TextureFormat,
    ) {
        self.blit_tone_mapped(target, view, format, 1.0, false);
    }
    pub fn blit_tone_mapped(
        &self,
        target: &RenderTarget,
        view: &wgpu::TextureView,
        format: wgpu::TextureFormat,
        exposure: f64,
        aces: bool,
    ) {
        self.blit_with_tone_mapping(
            target,
            view,
            format,
            exposure,
            if aces {
                ToneMapping::Aces
            } else {
                ToneMapping::None
            },
        );
    }
    pub fn blit_with_tone_mapping(
        &self,
        target: &RenderTarget,
        view: &wgpu::TextureView,
        format: wgpu::TextureFormat,
        exposure: f64,
        tone_mapping: ToneMapping,
    ) {
        self.present(
            target,
            view,
            format,
            [exposure as f32, tone_mapping as u32 as f32, 0.0, 0.0],
        );
    }
    /// Convert a premultiplied linear target to premultiplied sRGB canvas output.
    /// Use an unorm (non-sRGB) destination view to avoid a second conversion.
    pub fn blit_premultiplied_srgb(
        &self,
        target: &RenderTarget,
        view: &wgpu::TextureView,
        format: wgpu::TextureFormat,
        exposure: f64,
        tone_mapping: ToneMapping,
    ) {
        self.present(
            target,
            view,
            format,
            [exposure as f32, tone_mapping as u32 as f32, 1.0, 0.0],
        );
    }
    fn present(
        &self,
        target: &RenderTarget,
        view: &wgpu::TextureView,
        format: wgpu::TextureFormat,
        parameters: [f32; 4],
    ) {
        let mut cache = self.presentations.borrow_mut();
        let (layout, pipeline) = cache.entry(format).or_insert_with(|| {
            let shader = self
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("present"),
                    source: wgpu::ShaderSource::Wgsl(
                        concat!(
                            include_str!("shaders/output.wgsl"),
                            "\n",
                            include_str!("present.wgsl")
                        )
                        .into(),
                    ),
                });
            let layout = self
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("present"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });
            let pipeline_layout =
                self.device
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("present"),
                        bind_group_layouts: &[&layout],
                        push_constant_ranges: &[],
                    });
            let pipeline = self
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("present"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format,
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: Default::default(),
                    depth_stencil: None,
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                });
            (layout, pipeline)
        });
        // Retain both history targets and the direct scene presentation. Bound
        // storage so resizing or replacing targets does not grow this cache.
        let mut slots = self.present_slots.borrow_mut();
        let index = if let Some(i) = slots.iter().position(|(t, _)| t == &target.texture) {
            i
        } else {
            // A resized canvas must not retain full-sized attachments from
            // previous dimensions merely to fill the history cache.
            slots.retain(|(t, _)| {
                t.size() == target.texture.size() && t.format() == target.texture.format()
            });
            if slots.len() == 3 {
                slots.remove(0);
            }
            slots.push((target.texture.clone(), Default::default()));
            slots.len() - 1
        };
        let slot = &mut slots[index].1;
        let uniform = slot.uniform(&self.device, &self.queue, bytemuck::cast_slice(&parameters));
        let bind_group = slot.bindings(
            &self.device,
            layout,
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&target.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: uniform.as_entire_binding(),
                },
            ],
        );
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("present"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        self.queue.submit([encoder.finish()]);
    }
    pub fn render(&self, scene: &mut Scene, camera: Object3D, target: &RenderTarget) -> Result<()> {
        self.render_with_occlusion(scene, camera, target, None)
    }
    /// Render with optional asynchronous, resident GPU occlusion queries.
    pub fn render_with_occlusion(
        &self,
        scene: &mut Scene,
        camera: Object3D,
        target: &RenderTarget,
        queries: Option<&crate::occlusion::OcclusionQueries>,
    ) -> Result<()> {
        let queries = queries.filter(|q| q.available());
        if !(0.0..=1.0).contains(&scene.background_alpha) {
            return Err(Error::Invalid("scene background alpha"));
        }
        let key = scene.cache_id();
        {
            let mut cache = self.scene_draw_slots.borrow_mut();
            cache.retain(|_, (owner, _)| owner.strong_count() > 0);
            let (_, slots) = cache
                .entry(key)
                .or_insert_with(|| (Arc::downgrade(&scene.cache_owner), HashMap::new()));
            std::mem::swap(slots, &mut self.draw_slots.borrow_mut());
        }
        let result = self.render_with_slots(scene, camera, target, queries);
        let mut cache = self.scene_draw_slots.borrow_mut();
        std::mem::swap(
            &mut cache.get_mut(&key).unwrap().1,
            &mut self.draw_slots.borrow_mut(),
        );
        result
    }
    fn render_with_slots(
        &self,
        scene: &mut Scene,
        camera: Object3D,
        target: &RenderTarget,
        queries: Option<&crate::occlusion::OcclusionQueries>,
    ) -> Result<()> {
        let mut needs_transmission = false;
        let mut has_hooks = false;
        for root in scene.roots() {
            for handle in scene.traverse(root, true)? {
                let node = scene.get(handle)?;
                has_hooks |=
                    node.render_hooks.before.is_some() || node.render_hooks.after.is_some();
                if let NodeKind::Mesh(mesh) = &node.kind {
                    needs_transmission |= mesh
                        .materials
                        .iter()
                        .any(|m| matches!(m.as_ref(),Material::Physical(p) if p.transmission>0.0));
                }
            }
        }
        // HDR postprocessing targets can retain opaque color/depth and snapshot
        // it directly, like WebGPU's viewport texture. Preserve the existing
        // path for encoded outputs and special render hooks/occlusion queries.
        let reuse_opaque = needs_transmission
            && target.options.format == wgpu::TextureFormat::Rgba16Float
            && !target.options.encode_srgb
            && target.dimension == wgpu::TextureDimension::D2
            && target.texture.depth_or_array_layers() == 1
            && (target.options.samples <= 1
                || (target.options.resolve_color_buffer
                    && target.options.store_multisampled_color_buffer
                    && target.options.store_multisampled_depth_buffer
                    && target.options.store_multisampled_stencil_buffer))
            && !has_hooks
            && queries.is_none();
        if reuse_opaque {
            self.render_inner(scene, camera, target, ScenePass::Opaque, None, None)?;
            let mut mips = self.transmission_mips.borrow_mut();
            if mips.as_ref().is_none_or(|m| !m.matches(target)) {
                *mips = Some(crate::transmission::MipChain::new(&self.device, target));
            }
            let mips = mips.as_ref().unwrap();
            mips.update(&self.device, &self.queue, target);
            self.render_inner(
                scene,
                camera,
                target,
                ScenePass::Transparent,
                Some(&mips.view),
                None,
            )?;
        } else if needs_transmission {
            let mut cached = self.transmission_target.borrow_mut();
            if cached.as_ref().is_none_or(|t| {
                t.width != target.width
                    || t.height != target.height
                    || t.options.samples != target.options.samples
            }) {
                *cached = Some(RenderTarget::with_options(
                    &self.device,
                    target.width,
                    target.height,
                    RenderTargetOptions {
                        format: wgpu::TextureFormat::Rgba16Float,
                        samples: target.options.samples,
                        ..Default::default()
                    },
                )?);
            }
            let background = cached.as_mut().expect("transmission target");
            background.viewport = target.viewport;
            background.scissor = target.scissor;
            self.render_inner(scene, camera, background, ScenePass::Opaque, None, None)?;
            if self
                .transmission_mips
                .borrow()
                .as_ref()
                .is_none_or(|m| !m.matches(background))
            {
                *self.transmission_mips.borrow_mut() =
                    Some(crate::transmission::MipChain::new(&self.device, background));
            }
            let mips = self.transmission_mips.borrow();
            let mips = mips.as_ref().expect("transmission mip chain");
            mips.update(&self.device, &self.queue, background);
            self.render_inner(
                scene,
                camera,
                target,
                ScenePass::All,
                Some(&mips.view),
                queries,
            )?
        } else {
            self.render_inner(scene, camera, target, ScenePass::All, None, queries)?
        }
        self.draw_slots
            .borrow_mut()
            .retain(|(handle, group, opaque), _| {
                scene.get(*handle).is_ok_and(|node| {
                    node.geometry().is_some_and(|g| {
                        let count = match &node.kind {
                            NodeKind::Mesh(m) if m.materials.len() > 1 => g.groups.len(),
                            _ => 1,
                        };
                        *group < count && (!*opaque || needs_transmission)
                    })
                })
            });
        Ok(())
    }
    fn render_inner(
        &self,
        scene: &mut Scene,
        camera: Object3D,
        target: &RenderTarget,
        phase: ScenePass,
        transmission_view: Option<&wgpu::TextureView>,
        queries: Option<&crate::occlusion::OcclusionQueries>,
    ) -> Result<()> {
        let opaque_only = phase == ScenePass::Opaque;
        let resume = phase == ScenePass::Transparent;
        let valid_rectangle = |r: [u32; 4]| {
            r[2] > 0
                && r[3] > 0
                && r[0].checked_add(r[2]).is_some_and(|x| x <= target.width)
                && r[1].checked_add(r[3]).is_some_and(|y| y <= target.height)
        };
        if !valid_rectangle(target.viewport) || target.scissor.is_some_and(|r| !valid_rectangle(r))
        {
            return Err(Error::Invalid("viewport or scissor"));
        }
        self.geometry.borrow_mut().prune();
        self.textures.borrow_mut().prune();
        self.physical_maps.borrow_mut().prune();
        {
            let mut cached = self.environment.borrow_mut();
            if let Some(image) = &scene.environment {
                if !cached
                    .as_ref()
                    .is_some_and(|(owner, _)| owner.ptr_eq(&Arc::downgrade(image)))
                {
                    // A map already prefiltered on the GPU is reused without filtering.
                    if image.gpu.is_none() {
                        self.environment_builds
                            .set(self.environment_builds.get() + 1);
                    }
                    *cached = Some((
                        Arc::downgrade(image),
                        crate::environment_gpu::build(&self.device, &self.queue, image)?,
                    ));
                }
            } else {
                *cached = None;
            }
        }
        self.deformation.borrow_mut().prune(scene);
        scene.update()?;
        let (camera_data, camera_world) = scene.camera(camera)?;
        if camera_world.determinant() == 0.0 {
            return Err(Error::Invalid("singular camera transform"));
        }
        let perspective = matches!(camera_data, crate::camera::Camera::Perspective(_));
        let projection = camera_data.projection_matrix()?;
        let view_projection = projection * camera_world.inverse();
        let frustum = Frustum::from_projection(view_projection);
        let camera_layers = scene.get(camera)?.layers;
        let camera_position = camera_world.w_axis.truncate();
        let background = if scene.background_environment {
            self.environment.borrow().as_ref().map(|(_, env)| {
                crate::background::prepare(
                    &mut self.backgrounds.borrow_mut(),
                    &self.device,
                    &self.queue,
                    target,
                    env,
                    projection,
                    camera_world,
                    [
                        scene.environment_rotation,
                        scene.background_blur,
                        if env.source_is_cube_uv || scene.background_pmrem {
                            -1.0
                        } else if scene.background_equirectangular {
                            1.0
                        } else {
                            0.0
                        },
                        scene.background_intensity,
                        scene.exposure,
                        scene.output_tone_mapping() as u32 as f64,
                    ],
                    &scene.background_outputs,
                )
            })
        } else {
            None
        };
        let mut ambient = Vector3::ZERO;
        let mut light_position = [[0.0; 4]; 8];
        let mut light_color = [[0.0; 4]; 8];
        let mut light_params = [[0.0; 4]; 8];
        let mut light_direction = [[0.0; 4]; 8];
        let mut light_count = 0;
        let mut shadow_lights = Vec::new();
        let mut visible = Vec::new();
        for root in scene.roots() {
            visible.extend(scene.traverse(root, true)?);
        }
        for &h in &visible {
            let n = scene.get(h)?;
            if !n.layers.test(camera_layers) {
                continue;
            }
            if let NodeKind::Light(light) = &n.kind {
                if !matches!(light, Light::Ambient { .. }) && light_count == 8 {
                    return Err(Error::Invalid("more than eight nonambient lights"));
                }
                if !matches!(light, Light::Ambient { .. }) {
                    shadow_lights.push(h);
                }
                match light {
                    Light::Sun { color, intensity } => {
                        let d = n.world_position().normalize_or_zero();
                        light_position[light_count] = d.extend(0.).as_vec4().to_array();
                        light_color[light_count] =
                            (color.0 * *intensity).extend(1.).as_vec4().to_array();
                        light_count += 1;
                    }
                    Light::RectArea {
                        color,
                        intensity,
                        width,
                        height,
                    } => {
                        if !width.is_finite()
                            || !height.is_finite()
                            || *width <= 0.0
                            || *height <= 0.0
                        {
                            return Err(Error::Invalid("area light dimensions"));
                        }
                        let rotation = n.world_quaternion();
                        light_position[light_count] =
                            n.world_position().extend(4.0).as_vec4().to_array();
                        light_color[light_count] =
                            (color.0 * *intensity).extend(1.0).as_vec4().to_array();
                        light_direction[light_count] = (rotation * Vector3::X * *width * 0.5)
                            .extend(0.0)
                            .as_vec4()
                            .to_array();
                        light_params[light_count] = (rotation * Vector3::Y * *height * 0.5)
                            .extend(0.0)
                            .as_vec4()
                            .to_array();
                        light_count += 1;
                    }
                    Light::Hemisphere {
                        sky,
                        ground,
                        intensity,
                    } => {
                        light_position[light_count] = n
                            .world_position()
                            .normalize_or_zero()
                            .extend(3.0)
                            .as_vec4()
                            .to_array();
                        light_color[light_count] =
                            (sky.0 * *intensity).extend(0.0).as_vec4().to_array();
                        light_params[light_count] =
                            (ground.0 * *intensity).extend(0.0).as_vec4().to_array();
                        light_count += 1;
                    }
                    Light::Spot {
                        color,
                        intensity,
                        target,
                        distance,
                        decay,
                        angle,
                        penumbra,
                    } => {
                        if !angle.is_finite()
                            || *angle <= 0.0
                            || *angle > std::f64::consts::FRAC_PI_2
                            // Three.js leaves penumbra unclamped: cos( angle × ( 1 − penumbra ) ).
                            || !penumbra.is_finite()
                            || *penumbra < 0.0
                        {
                            return Err(Error::Invalid("spot angle or penumbra"));
                        }
                        light_position[light_count] =
                            n.world_position().extend(2.0).as_vec4().to_array();
                        light_direction[light_count] = (n.world_position() - *target)
                            .normalize_or_zero()
                            .extend(0.0)
                            .as_vec4()
                            .to_array();
                        light_color[light_count] =
                            (color.0 * *intensity).extend(1.0).as_vec4().to_array();
                        light_params[light_count] = [
                            *distance as f32,
                            *decay as f32,
                            angle.cos() as f32,
                            (angle * (1.0 - penumbra)).cos() as f32,
                        ];
                        light_count += 1;
                    }
                    Light::Ambient { color, intensity } => ambient += color.0 * *intensity,
                    Light::Directional {
                        color,
                        intensity,
                        target,
                    } => {
                        if light_count == 8 {
                            return Err(Error::Invalid("more than eight nonambient lights"));
                        }
                        let d = (n.world_position() - *target).normalize_or_zero();
                        light_position[light_count] = [d.x as f32, d.y as f32, d.z as f32, 0.0];
                        light_color[light_count] =
                            (color.0 * *intensity).extend(1.0).as_vec4().to_array();
                        light_count += 1;
                    }
                    Light::Point {
                        color,
                        intensity,
                        distance,
                        decay,
                    } => {
                        if light_count == 8 {
                            return Err(Error::Invalid("more than eight nonambient lights"));
                        }
                        light_position[light_count] =
                            n.world_position().extend(1.0).as_vec4().to_array();
                        light_color[light_count] =
                            (color.0 * *intensity).extend(1.0).as_vec4().to_array();
                        light_params[light_count] = [*distance as f32, *decay as f32, 0.0, 0.0];
                        light_count += 1;
                    }
                }
            }
        }
        let shadows = self.shadows.prepare(
            &self.device,
            &self.queue,
            &mut self.textures.borrow_mut(),
            scene,
            &visible,
            &shadow_lights,
            camera_data,
            camera_world,
            &mut self.geometry.borrow_mut(),
            &mut self.deformation.borrow_mut(),
        )?;
        visible.sort_by_key(|h| scene.get(*h).expect("valid node").render_order);
        let mut draws = Vec::new();
        let mut after_hooks = Vec::new();
        for &h in &visible {
            let n = scene.get(h)?;
            if !n.layers.test(camera_layers) {
                continue;
            }
            if n.geometry().is_none() {
                continue;
            }
            if n.frustum_culled
                && n.skin.is_none()
                && n.morph_weights.is_empty()
                && n.instances.is_empty()
                && !frustum.intersects_sphere(
                    self.geometry
                        .borrow_mut()
                        .sphere(n.geometry().unwrap())?
                        .transformed(n.matrix_world),
                )
            {
                continue;
            }
            let before = if transmission_view.is_some() {
                None
            } else {
                n.render_hooks.before.clone()
            };
            if let Some(callback) = before {
                callback(scene, h);
                scene.update_world_matrix(h, true, false)?;
            }
            let n = scene.get(h)?;
            let Some(geometry) = n.geometry() else {
                continue;
            };
            let wide = if let NodeKind::Line(line) = &n.kind {
                if let Material::Line(m) = line.material.as_ref() {
                    (m.linewidth != 1.0
                        || (m.dash.is_some() && !geometry.has_attribute("lineDistance")))
                    .then_some(line.segments)
                } else {
                    None
                }
            } else {
                None
            };
            let expanded_line = wide.is_some();
            if let Some(callback) = n.render_hooks.after.clone().filter(|_| !opaque_only) {
                after_hooks.push((h, callback));
            }
            let (materials, topology) = match &n.kind {
                NodeKind::Mesh(m) => (
                    m.materials.iter().map(|m| m.as_ref()).collect::<Vec<_>>(),
                    wgpu::PrimitiveTopology::TriangleList,
                ),
                NodeKind::Line(l) => (
                    vec![l.material.as_ref()],
                    if expanded_line {
                        wgpu::PrimitiveTopology::TriangleList
                    } else if l.segments {
                        wgpu::PrimitiveTopology::LineList
                    } else {
                        wgpu::PrimitiveTopology::LineStrip
                    },
                ),
                NodeKind::Points(p) => (
                    vec![p.material.as_ref()],
                    wgpu::PrimitiveTopology::PointList,
                ),
                _ => continue,
            };
            let groups = if materials.len() > 1 {
                geometry.groups.clone()
            } else {
                vec![crate::geometry::Group {
                    start: 0,
                    count: geometry.draw_count(),
                    material_index: 0,
                }]
            };
            for (group_index, group) in groups.into_iter().enumerate() {
                let Some(material) = materials.get(group.material_index) else {
                    return Err(Error::Invalid("group material index"));
                };
                let properties = material.properties();
                let transparent = properties.transparent
                    || matches!(material,Material::Physical(m) if m.transmission>0.0);
                if (opaque_only && transparent) || (resume && !transparent) {
                    continue;
                }
                if !properties.visible {
                    continue;
                }
                let start = group.start.max(geometry.draw_range.start);
                let end = group
                    .start
                    .saturating_add(group.count)
                    .min(geometry.draw_count())
                    .min(
                        geometry
                            .draw_range
                            .start
                            .saturating_add(geometry.draw_range.count.unwrap_or(usize::MAX)),
                    );
                if end <= start {
                    continue;
                }
                // Shader points use native one-pixel primitives; PointsMaterial
                // alone requests the sized billboard expansion.
                let is_points = matches!(n.kind, NodeKind::Points(_))
                    && matches!(material, Material::Points(_));
                let wireframe = properties.wireframe && matches!(n.kind, NodeKind::Mesh(_));
                if wireframe
                    && ((geometry.indirect.is_some() || geometry.gpu_indirect.is_some())
                        || start % 3 != 0
                        || end % 3 != 0)
                {
                    return Err(Error::Invalid(
                        "wireframe requires direct complete triangles",
                    ));
                }

                let gpu_geometry = self.geometry.borrow_mut().get(
                    &self.device,
                    &self.queue,
                    geometry,
                    properties.vertex_colors,
                    is_points,
                    wireframe,
                    wide,
                )?;
                let point = match material {
                    Material::Line(_) if expanded_line => [
                        target.viewport[2] as f32,
                        target.viewport[3] as f32,
                        0.0,
                        0.0,
                    ],
                    Material::Points(m) if is_points => [
                        target.width as f32,
                        target.height as f32,
                        m.size as f32,
                        if m.size_attenuation && perspective {
                            1.0
                        } else {
                            0.0
                        },
                    ],
                    _ => [target.width as f32, target.height as f32, 0.0, 0.0],
                };
                let (kind, roughness, metalness, emissive) = match material {
                    Material::Standard(m)
                    | Material::Physical(MeshPhysicalMaterial { base: m, .. }) => {
                        (1.0, m.roughness as f32, m.metalness as f32, m.emissive.0)
                    }
                    Material::Toon(m) => (6.0, 1.0, 0.0, m.base.emissive.0),
                    Material::Matcap(_) => (7.0, 1.0, 0.0, Vector3::ZERO),
                    Material::Depth(_) => (8.0, 1.0, 0.0, Vector3::ZERO),
                    Material::Lambert(m) => (2.0, 1.0, 0.0, m.emissive.0),
                    Material::Phong(m) => (3.0, 1.0, 0.0, m.emissive.0),
                    Material::Shader(_) => (5.0, 1.0, 0.0, Vector3::ZERO),
                    Material::Normal(_) => (4.0, 1.0, 0.0, Vector3::ZERO),
                    _ => (0.0, 1.0, 0.0, Vector3::ZERO),
                };
                let (fog_color, fog_params) = match scene.fog.filter(|_| properties.fog) {
                    Some(Fog::Linear { color, near, far }) => {
                        if !near.is_finite() || !far.is_finite() || far <= near {
                            return Err(Error::Invalid("fog range"));
                        }
                        (
                            color.0.extend(0.0).as_vec4().to_array(),
                            [1.0, near as f32, far as f32, 0.0],
                        )
                    }
                    Some(Fog::Exp2 { color, density }) => {
                        if !density.is_finite() || density < 0.0 {
                            return Err(Error::Invalid("fog density"));
                        }
                        (
                            color.0.extend(0.0).as_vec4().to_array(),
                            [2.0, density as f32, 0.0, 0.0],
                        )
                    }
                    None => ([0.0; 4], [0.0; 4]),
                };
                let clipping = crate::clipping::prepare(scene, properties, false)?;
                let mut uv_transforms = [[0.0; 4]; 15];
                for (i, map) in material.texture_maps().iter().enumerate() {
                    let columns = if let Some(t) = map {
                        if t.tex_coord > 1 {
                            return Err(Error::Invalid("texture coordinate channel"));
                        }
                        let mut columns = t.uv_matrix().to_cols_array_2d().map(|column| {
                            [column[0] as f32, column[1] as f32, column[2] as f32, 0.0]
                        });
                        columns[2][3] = t.tex_coord as f32;
                        columns[0][3] = f32::from(t.flip_y);
                        columns
                    } else {
                        [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                        ]
                    };
                    uv_transforms[i * 3..i * 3 + 3].copy_from_slice(&columns);
                }
                let mut u = Uniforms {
                    output: [
                        scene.exposure as f32,
                        if material.properties().tone_mapped {
                            scene.output_tone_mapping() as u32 as f32
                        } else {
                            0.0
                        },
                        0.0,
                        0.0,
                    ],
                    uv_transforms,
                    clipping,
                    physical: match material {
                        Material::Physical(m) => {
                            if ![
                                m.ior,
                                m.specular_intensity,
                                m.clearcoat,
                                m.clearcoat_roughness,
                                m.sheen,
                                m.sheen_roughness,
                                m.anisotropy,
                                m.anisotropy_rotation,
                                m.retroreflectivity,
                            ]
                            .iter()
                            .all(|v| v.is_finite())
                                || m.ior < 1.0
                            {
                                return Err(Error::Invalid("physical material factors"));
                            }
                            [
                                [
                                    m.clearcoat.clamp(0.0, 1.0) as f32,
                                    m.clearcoat_roughness.clamp(0.0, 1.0) as f32,
                                    m.sheen_roughness.clamp(0.07, 1.0) as f32,
                                    m.ior as f32,
                                ],
                                (m.sheen_color.0 * m.sheen.clamp(0.0, 1.0))
                                    .extend(m.anisotropy.clamp(0.0, 1.0))
                                    .as_vec4()
                                    .to_array(),
                                m.specular_color
                                    .0
                                    .extend(m.specular_intensity.clamp(0.0, 1.0))
                                    .as_vec4()
                                    .to_array(),
                                [
                                    m.anisotropy_rotation as f32,
                                    1.0,
                                    f32::from(m.base.energy_conservation),
                                    m.retroreflectivity.clamp(0.0, 1.0) as f32,
                                ],
                            ]
                        }
                        Material::Standard(m) => {
                            let mut p = [[0.0; 4]; 4];
                            p[3][2] = f32::from(m.energy_conservation);
                            p
                        }
                        _ => [[0.0; 4]; 4],
                    },
                    mvp: (view_projection * n.matrix_world).as_mat4().to_cols_array(),
                    model: n.matrix_world.as_mat4().to_cols_array(),
                    normal: n
                        .matrix_world
                        .inverse()
                        .transpose()
                        .as_mat4()
                        .to_cols_array(),
                    projection: projection.as_mat4().to_cols_array(),
                    view: camera_world.inverse().as_mat4().to_cols_array(),
                    color: properties
                        .color
                        .0
                        .extend(properties.opacity)
                        .as_vec4()
                        .to_array(),
                    camera: camera_position.extend(1.0).as_vec4().to_array(),
                    material: [kind, roughness, metalness, light_count as f32],
                    emissive: emissive.extend(0.0).as_vec4().to_array(),
                    ambient: ambient.extend(0.0).as_vec4().to_array(),
                    point,
                    pbr: match material {
                        Material::Standard(m)
                        | Material::Physical(MeshPhysicalMaterial { base: m, .. }) => [
                            m.normal_scale.x as f32,
                            m.normal_scale.y as f32,
                            m.occlusion_strength as f32,
                            properties.alpha_test as f32,
                        ],
                        Material::Phong(m) => [
                            m.normal_scale.x as f32,
                            m.normal_scale.y as f32,
                            1.0,
                            properties.alpha_test as f32,
                        ],
                        Material::Lambert(m)
                        | Material::Toon(MeshToonMaterial { base: m, .. })
                        | Material::Matcap(MeshMatcapMaterial { base: m, .. }) => [
                            m.normal_scale.x as f32,
                            m.normal_scale.y as f32,
                            1.0,
                            properties.alpha_test as f32,
                        ],
                        _ => [1.0, 1.0, 1.0, properties.alpha_test as f32],
                    },
                    environment: [
                        if scene.environment.is_some()
                            || properties
                                .vertex_program
                                .as_ref()
                                .is_some_and(|p| p.custom_environment)
                        {
                            scene.environment_intensity as f32
                        } else {
                            0.0
                        },
                        scene.environment_rotation as f32,
                        self.environment
                            .borrow()
                            .as_ref()
                            .map_or(0.0, |(_, e)| e.max_mip),
                        0.0,
                    ],
                    maps: match material {
                        Material::Standard(m)
                        | Material::Physical(MeshPhysicalMaterial { base: m, .. }) => [
                            f32::from(m.normal_map.is_some()),
                            f32::from(properties.transparent),
                            f32::from(m.emissive_map.is_some()),
                            f32::from(m.metallic_roughness_map.is_some()),
                        ],
                        Material::Phong(m) => [
                            f32::from(m.normal_map.is_some()),
                            f32::from(properties.transparent),
                            0.0,
                            0.0,
                        ],
                        Material::Lambert(m)
                        | Material::Toon(MeshToonMaterial { base: m, .. })
                        | Material::Matcap(MeshMatcapMaterial { base: m, .. }) => [
                            f32::from(m.normal_map.is_some()),
                            f32::from(properties.transparent),
                            0.0,
                            0.0,
                        ],
                        _ => [0.0, f32::from(properties.transparent), 0.0, 0.0],
                    },
                    light_position,
                    light_color,
                    light_params,
                    light_direction,
                    specular: match material {
                        Material::Phong(m) => m
                            .specular
                            .0
                            .extend(m.shininess.max(1e-4))
                            .as_vec4()
                            .to_array(),
                        _ => [0.0; 4],
                    },
                    flags: [
                        f32::from(properties.flat_shading),
                        f32::from(n.receive_shadow),
                        f32::from(properties.vertex_colors),
                        0.0,
                    ],
                    fog_color,
                    fog_params,
                    custom: match material {
                        Material::Shader(m) => m.uniforms,
                        Material::Toon(m) => {
                            let mut values = [[0.0; 4]; 16];
                            values[0][0] = f32::from(m.gradient_map.is_some());
                            values
                        }
                        Material::Matcap(m) => {
                            let mut values = [[0.0; 4]; 16];
                            values[0][0] = f32::from(m.matcap.is_some());
                            values
                        }
                        _ => properties.vertex_uniforms,
                    },
                    extension_maps: crate::physical_maps::MapUniforms::new(
                        &if let Material::Physical(m) = material {
                            m.extension_maps()
                        } else {
                            [None; 12]
                        },
                    ),
                    line: if let Material::Line(m) = material {
                        if !m.linewidth.is_finite() || m.linewidth <= 0.0 {
                            return Err(Error::Invalid("line width"));
                        }
                        let d = m.dash.unwrap_or_default();
                        if !d.size.is_finite()
                            || !d.gap.is_finite()
                            || !d.scale.is_finite()
                            || !d.offset.is_finite()
                            || d.size <= 0.0
                            || d.gap < 0.0
                            || d.scale <= 0.0
                        {
                            return Err(Error::Invalid("line dash parameters"));
                        }
                        [
                            [
                                if expanded_line {
                                    m.linewidth as f32
                                } else {
                                    0.0
                                },
                                d.size as f32,
                                d.gap as f32,
                                d.scale as f32,
                            ],
                            [d.offset as f32, f32::from(m.dash.is_some()), 0.0, 0.0],
                        ]
                    } else {
                        [[0.0; 4]; 2]
                    },
                    iridescence: if let Material::Physical(m) = material {
                        [
                            m.iridescence as f32,
                            m.iridescence_ior as f32,
                            m.iridescence_thickness_range[0] as f32,
                            m.iridescence_thickness_range[1] as f32,
                        ]
                    } else {
                        [0.0; 4]
                    },
                    coat_normal: if let Material::Physical(m) = material {
                        [
                            m.clearcoat_normal_scale.x as f32,
                            m.clearcoat_normal_scale.y as f32,
                            0.0,
                            0.0,
                        ]
                    } else {
                        [1.0, 1.0, 0.0, 0.0]
                    },
                    transmission: if let Material::Physical(m) = material {
                        if !m.transmission.is_finite()
                            || !(0.0..=1.0).contains(&m.transmission)
                            || !m.thickness.is_finite()
                            || m.thickness < 0.0
                            || m.attenuation_distance <= 0.0
                            || m.attenuation_distance.is_nan()
                        {
                            return Err(Error::Invalid("physical transmission parameters"));
                        }
                        [
                            [
                                m.transmission as f32,
                                m.thickness as f32,
                                m.attenuation_distance as f32,
                                m.dispersion as f32,
                            ],
                            m.attenuation_color.0.extend(0.0).as_vec4().to_array(),
                            [
                                target.viewport[0] as f32 / target.width as f32,
                                target.viewport[1] as f32 / target.height as f32,
                                target.viewport[2] as f32 / target.width as f32,
                                target.viewport[3] as f32 / target.height as f32,
                            ],
                        ]
                    } else {
                        [
                            [0.0; 4],
                            [0.0; 4],
                            [
                                target.viewport[0] as f32 / target.width as f32,
                                target.viewport[1] as f32 / target.height as f32,
                                target.viewport[2] as f32 / target.width as f32,
                                target.viewport[3] as f32 / target.height as f32,
                            ],
                        ]
                    },
                    shadow_matrices: shadows.matrices,
                    shadow_params: shadows.params,
                    shadow_filters: shadows.filters,
                    shadow_cascades: shadows.cascades,
                };
                // WebGL keeps the ALPHA_TO_COVERAGE clipping shader without MSAA: edge
                // fragments with nonzero clip opacity survive. WebGPU outputs clip hard.
                if target.options.samples <= 1 && !target.options.encode_srgb {
                    u.clipping.params[3] = 0.;
                }
                if let Some(selected) = &properties.lights {
                    u.ambient = [0.0; 4];
                    for &light in selected {
                        if visible.contains(&light)
                            && scene.get(light)?.layers.test(camera_layers)
                            && let NodeKind::Light(Light::Ambient { color, intensity }) =
                                &scene.get(light)?.kind
                        {
                            for (i, c) in color.0.to_array().iter().enumerate() {
                                u.ambient[i] += (*c * *intensity) as f32;
                            }
                        }
                    }
                    let mut count = 0;
                    for (index, light) in shadow_lights.iter().enumerate() {
                        if selected.contains(light) {
                            u.light_position[count] = light_position[index];
                            u.light_color[count] = light_color[index];
                            u.light_params[count] = light_params[index];
                            u.light_direction[count] = light_direction[index];
                            u.shadow_params[count] = shadows.params[index];
                            u.shadow_filters[count] = shadows.filters[index];
                            u.shadow_cascades[count * 2] = shadows.cascades[index * 2];
                            u.shadow_cascades[count * 2 + 1] = shadows.cascades[index * 2 + 1];

                            count += 1;
                        }
                    }
                    u.material[3] = count as f32;
                }
                let mut draw = self.prepare_draw(
                    DrawGeometry {
                        key: (h, group_index, opaque_only),
                        vertices: gpu_geometry.vertices,
                        deformation: self.deformation.borrow_mut().get(
                            &self.device,
                            &self.queue,
                            scene,
                            h,
                        )?,
                        instances: &n.instances,
                        transmission_view,
                    },
                    &u,
                    material,
                    if wireframe {
                        wgpu::PrimitiveTopology::LineList
                    } else if is_points {
                        wgpu::PrimitiveTopology::TriangleList
                    } else {
                        topology
                    },
                    target,
                    &shadows,
                )?;
                let factor = if wireframe {
                    2
                } else if is_points {
                    6
                } else {
                    1
                };
                draw.render_order = n.render_order;
                draw.group_order = scene
                    .ancestors(h)?
                    .into_iter()
                    .find_map(|a| {
                        scene
                            .get(a)
                            .ok()
                            .filter(|n| matches!(n.kind, NodeKind::Group))
                            .map(|n| n.render_order)
                    })
                    .unwrap_or(0);
                draw.depth = view_projection.project_point3(n.world_position()).z;
                draw.range = (start as u32) * factor..(end as u32) * factor;
                if let Some(segments) = wide {
                    if (geometry.indirect.is_some() || geometry.gpu_indirect.is_some())
                        || (segments && start % 2 != 0)
                    {
                        return Err(Error::Invalid("wide line direct range"));
                    }
                    draw.range = if segments {
                        (start as u32 / 2) * 6..(end as u32 / 2) * 6
                    } else {
                        start as u32 * 6..end.saturating_sub(1) as u32 * 6
                    };
                }
                draw.instances = n.draw_instance_count(geometry)?;
                draw.indices = gpu_geometry.indices;
                if let Some(commands) = &geometry.indirect {
                    let command_size = if geometry.index.is_some() { 5 } else { 4 };
                    let offsets = if geometry.indirect_offsets.is_empty() {
                        vec![geometry.indirect_offset]
                    } else {
                        geometry.indirect_offsets.clone()
                    };
                    let source_commands = commands;
                    let mut commands = source_commands.clone();
                    for offset in offsets {
                        if offset % 4 != 0 || offset / 4 + command_size > commands.len() {
                            return Err(Error::Invalid("indirect command offset"));
                        }
                        if !self
                            .device
                            .features()
                            .contains(wgpu::Features::INDIRECT_FIRST_INSTANCE)
                            && commands[offset / 4 + command_size - 1] != 0
                        {
                            return Err(Error::Invalid(
                                "indirect first instance requires adapter feature",
                            ));
                        }
                        if is_points {
                            let i = offset / 4;
                            commands[i] = source_commands[i]
                                .checked_mul(6)
                                .ok_or(Error::Invalid("point indirect count"))?;
                            commands[i + 2] = source_commands[i + 2]
                                .checked_mul(6)
                                .ok_or(Error::Invalid("point indirect offset"))?;
                            if command_size == 5 {
                                commands[i + 3] = ((source_commands[i + 3] as i32)
                                    .checked_mul(6)
                                    .ok_or(Error::Invalid("point base vertex"))?)
                                    as u32;
                            }
                        }
                        draw.indirect_offsets.push(offset as u64);
                    }
                    draw.indirect = Some(
                        self.draw_slots
                            .borrow_mut()
                            .get_mut(&(h, group_index, opaque_only))
                            .unwrap()
                            .indirect(&self.device, &self.queue, bytemuck::cast_slice(&commands)),
                    );
                }
                if let Some(buffer) = &geometry.gpu_indirect {
                    if geometry.indirect.is_some()
                        || is_points
                        || wireframe
                        || !buffer.usage().contains(wgpu::BufferUsages::INDIRECT)
                    {
                        return Err(Error::Invalid("GPU indirect geometry"));
                    }
                    let offsets = if geometry.indirect_offsets.is_empty() {
                        vec![geometry.indirect_offset]
                    } else {
                        geometry.indirect_offsets.clone()
                    };
                    for offset in offsets {
                        let size = if draw.indices.is_some() { 20 } else { 16 };
                        if offset % 4 != 0
                            || (offset as u64)
                                .checked_add(size)
                                .is_none_or(|end| end > buffer.size())
                        {
                            return Err(Error::Invalid("GPU indirect command offset"));
                        }
                        draw.indirect_offsets.push(offset as u64);
                    }
                    draw.indirect = Some(buffer.clone());
                }
                if draw.viewport != 0
                    && let Some(pipeline) = draw.back_pipeline.take()
                {
                    // Shared color captures each side; depth remains the pre-object snapshot.
                    let mut back = draw.clone();
                    back.pipeline = pipeline;
                    draw.viewport &= 1;
                    draws.push(back);
                }
                draws.push(draw);
            }
        }
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("scene render"),
            });
        draws.sort_by(|a, b| {
            a.transparent
                .cmp(&b.transparent)
                .then(a.group_order.cmp(&b.group_order))
                .then(a.render_order.cmp(&b.render_order))
                .then_with(|| {
                    if a.transparent {
                        b.depth.total_cmp(&a.depth)
                    } else {
                        a.depth.total_cmp(&b.depth)
                    }
                })
        });
        let query_objects: Vec<_> = draws
            .iter()
            .filter(|d| queries.is_some_and(|q| q.contains(d.object)))
            .map(|d| d.object)
            .collect();
        if let Some(q) = queries {
            q.validate_count(query_objects.len() as u32)?;
        }
        let queries = queries.filter(|_| !query_objects.is_empty());
        let mut start = 0;
        let mut first = true;
        let mut query_index = 0;
        loop {
            // An empty initial pass initializes attachments if the first object samples them.
            if !first && start < draws.len() && draws[start].viewport != 0 {
                self.viewport
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .encode(&mut encoder, draws[start].viewport);
            }
            let end = if first && draws.first().is_some_and(|d| d.viewport != 0) {
                0
            } else {
                (start + 1..draws.len())
                    .find(|&i| draws[i].viewport != 0)
                    .unwrap_or(draws.len())
            };
            let resume = resume || !first;
            let c = if target.options.encode_srgb {
                scene.background.0.map(linear_to_srgb)
            } else {
                scene.background.0
            };
            let attachments: Vec<_> = target
                .views
                .iter()
                .enumerate()
                .map(|(i, view)| {
                    Some(wgpu::RenderPassColorAttachment {
                        view: target.multisampled_views.get(i).unwrap_or(view),
                        depth_slice: if target.dimension == wgpu::TextureDimension::D3 {
                            Some(target.layer)
                        } else {
                            None
                        },
                        resolve_target: if target.options.samples > 1
                            && target.options.resolve_color_buffer
                        {
                            Some(view)
                        } else {
                            None
                        },
                        ops: wgpu::Operations {
                            load: if resume || target.options.load_color {
                                wgpu::LoadOp::Load
                            } else {
                                wgpu::LoadOp::Clear(
                                    if let Some(color) = target.options.clear_colors.get(i) {
                                        *color
                                    } else if i == 0 {
                                        wgpu::Color {
                                            r: c.x * scene.background_alpha,
                                            g: c.y * scene.background_alpha,
                                            b: c.z * scene.background_alpha,
                                            a: scene.background_alpha,
                                        }
                                    } else {
                                        wgpu::Color::TRANSPARENT
                                    },
                                )
                            },
                            store: if target.options.samples > 1
                                && !target.options.store_multisampled_color_buffer
                            {
                                wgpu::StoreOp::Discard
                            } else {
                                wgpu::StoreOp::Store
                            },
                        },
                    })
                })
                .collect();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("scene"),
                color_attachments: &attachments,
                depth_stencil_attachment: target.depth_view.as_ref().map(|view| {
                    wgpu::RenderPassDepthStencilAttachment {
                        view,
                        depth_ops: target.options.depth_buffer.then_some(wgpu::Operations {
                            load: if resume || target.options.load_depth {
                                wgpu::LoadOp::Load
                            } else {
                                wgpu::LoadOp::Clear(1.0)
                            },
                            store: if target.options.samples > 1
                                && !target.options.store_multisampled_depth_buffer
                            {
                                wgpu::StoreOp::Discard
                            } else {
                                wgpu::StoreOp::Store
                            },
                        }),
                        stencil_ops: target.options.stencil_buffer.then_some(wgpu::Operations {
                            load: if resume {
                                wgpu::LoadOp::Load
                            } else {
                                wgpu::LoadOp::Clear(0)
                            },
                            store: if target.options.samples > 1
                                && !target.options.store_multisampled_stencil_buffer
                            {
                                wgpu::StoreOp::Discard
                            } else {
                                wgpu::StoreOp::Store
                            },
                        }),
                    }
                }),
                timestamp_writes: None,
                occlusion_query_set: queries.map(|q| &q.queries),
            });
            let [x, y, w, h] = target.viewport;
            pass.set_viewport(x as f32, y as f32, w as f32, h as f32, 0.0, 1.0);
            if let Some([x, y, w, h]) = target.scissor {
                pass.set_scissor_rect(x, y, w, h);
            }
            if let Some((pipeline, bindings)) = background.as_ref().filter(|_| !resume) {
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, bindings, &[]);
                pass.draw(0..3, 0..1);
            }
            for draw in &draws[start..end] {
                let query = queries.is_some_and(|q| q.contains(draw.object));
                if query {
                    pass.begin_occlusion_query(query_index);
                    query_index += 1;
                }
                // Match Three.js: transparent double-sided meshes draw back faces first.
                for pipeline in draw
                    .back_pipeline
                    .iter()
                    .chain(std::iter::once(&draw.pipeline))
                {
                    pass.set_pipeline(pipeline);
                    pass.set_stencil_reference(draw.stencil_reference);
                    pass.set_bind_group(0, &draw.bind_group, &[]);
                    if let Some(bindings) = &draw.custom_bindings {
                        pass.set_bind_group(1, bindings, &[]);
                    }
                    pass.set_vertex_buffer(0, draw.vertices.slice(..));
                    pass.set_vertex_buffer(1, draw.instance_buffer.slice(..));
                    if let Some(indices) = &draw.indices {
                        pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
                    }
                    if let Some(commands) = &draw.indirect {
                        for &offset in &draw.indirect_offsets {
                            if draw.indices.is_some() {
                                pass.draw_indexed_indirect(commands, offset);
                            } else {
                                pass.draw_indirect(commands, offset);
                            }
                        }
                    } else if draw.indices.is_some() {
                        pass.draw_indexed(draw.range.clone(), 0, 0..draw.instances);
                    } else {
                        pass.draw(draw.range.clone(), 0..draw.instances);
                    }
                }
                if query {
                    pass.end_occlusion_query();
                }
            }
            drop(pass);
            if end == draws.len() {
                break;
            }
            start = end;
            first = false;
        }
        if let Some(q) = queries {
            q.resolve(&mut encoder, query_objects.len() as u32);
        }
        self.queue.submit([encoder.finish()]);
        if let Some(q) = queries {
            q.read(query_objects);
        }
        for (object, callback) in after_hooks {
            callback(scene, object);
        }
        Ok(())
    }
    fn prepare_draw(
        &self,
        geometry: DrawGeometry<'_>,
        uniforms: &Uniforms,
        material: &Material,
        topology: wgpu::PrimitiveTopology,
        target: &RenderTarget,
        shadows: &crate::shadow::Atlas,
    ) -> Result<Draw> {
        let DrawGeometry {
            key,
            vertices,
            deformation,
            instances,
            transmission_view,
        } = geometry;
        let object = key.0;
        let properties = material.properties();
        let custom = if let Material::Shader(m) = material {
            Some(&m.program)
        } else {
            properties.vertex_program.as_ref()
        };
        if custom.is_some_and(|p| p.viewport != 0) {
            let mut cache = self.viewport.borrow_mut();
            if cache.as_ref().is_none_or(|s| !s.matches(target)) {
                *cache = Some(crate::viewport::Snapshot::new(&self.device, target)?);
            }
        }
        let vertex_buffer = vertices;
        let mut slots = self.draw_slots.borrow_mut();
        let slot = slots.entry(key).or_default();
        let uniform_buffer = slot.uniform(&self.device, &self.queue, bytemuck::bytes_of(uniforms));
        let extension_maps = self.physical_maps.borrow_mut().get(
            &self.device,
            &self.queue,
            if let Material::Physical(m) = material {
                m.extension_maps()
            } else {
                [None; 12]
            },
        )?;
        let maps = material.texture_maps();
        let mut cache = self.textures.borrow_mut();
        let images = maps
            .into_iter()
            .map(|image| cache.get(&self.device, &self.queue, image.unwrap_or(&self.white)))
            .collect::<Result<Vec<_>>>()?;
        let fallback = cache.get(&self.device, &self.queue, &self.white)?;
        let environment = self.environment.borrow();
        let env = environment.as_ref().map(|(_, e)| e);
        let mut bindings = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }];
        bindings.extend([
            wgpu::BindGroupEntry {
                binding: 21,
                resource: deformation.data.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 22,
                resource: deformation.pose.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 23,
                resource: deformation.info.as_entire_binding(),
            },
        ]);
        for (i, image) in images.iter().enumerate() {
            bindings.push(wgpu::BindGroupEntry {
                binding: i as u32 * 2 + 1,
                resource: wgpu::BindingResource::TextureView(&image.view),
            });
            bindings.push(wgpu::BindGroupEntry {
                binding: i as u32 * 2 + 2,
                resource: wgpu::BindingResource::Sampler(&image.sampler),
            });
        }
        bindings.extend([
            wgpu::BindGroupEntry {
                binding: 11,
                resource: wgpu::BindingResource::TextureView(
                    env.map_or(&fallback.view, |e| &e.view),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 12,
                resource: wgpu::BindingResource::TextureView(
                    env.map_or(&fallback.view, |e| &e.source),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 13,
                resource: wgpu::BindingResource::Sampler(
                    env.map_or(&fallback.sampler, |e| &e.sampler),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 14,
                resource: wgpu::BindingResource::TextureView(&self.dfg),
            },
        ]);
        bindings.push(wgpu::BindGroupEntry {
            binding: 20,
            resource: wgpu::BindingResource::TextureView(&extension_maps),
        });
        bindings.push(wgpu::BindGroupEntry {
            binding: 24,
            resource: wgpu::BindingResource::Sampler(&self.ltc_sampler),
        });
        bindings.push(wgpu::BindGroupEntry {
            binding: 17,
            resource: wgpu::BindingResource::TextureView(&self.ltc),
        });
        bindings.push(wgpu::BindGroupEntry {
            binding: 18,
            resource: wgpu::BindingResource::TextureView(
                transmission_view.unwrap_or(&fallback.view),
            ),
        });
        bindings.push(wgpu::BindGroupEntry {
            binding: 19,
            resource: wgpu::BindingResource::Sampler(&self.transmission_sampler),
        });
        bindings.extend([
            wgpu::BindGroupEntry {
                binding: 15,
                resource: wgpu::BindingResource::TextureView(&shadows.view),
            },
            wgpu::BindGroupEntry {
                binding: 16,
                resource: wgpu::BindingResource::Sampler(&self.shadows.sampler),
            },
        ]);
        let uses_viewport = custom.is_some_and(|p| p.viewport != 0);
        let draw_layout = if uses_viewport {
            &self.viewport_layout
        } else {
            &self.layout
        };
        let viewport = self.viewport.borrow();
        if uses_viewport {
            bindings.extend([
                wgpu::BindGroupEntry {
                    binding: 25,
                    resource: wgpu::BindingResource::TextureView(
                        viewport.as_ref().map_or(&fallback.view, |v| &v.color_view),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 26,
                    resource: wgpu::BindingResource::TextureView(
                        viewport.as_ref().map_or(&fallback.view, |v| &v.depth_view),
                    ),
                },
            ]);
        }
        let bind_group = slot.bindings(&self.device, draw_layout, &bindings);

        if custom.is_some_and(|p| p.outputs > target.options.count) {
            return Err(Error::Invalid(
                "shader outputs exceed render target attachments",
            ));
        }
        let instance_data = if instances.is_empty() {
            vec![InstanceVertex {
                matrix: Matrix4::IDENTITY.as_mat4().to_cols_array(),
                color: [1.0; 4],
            }]
        } else {
            instances
                .iter()
                .map(|instance| {
                    if !instance.matrix.is_finite() || instance.matrix.determinant() <= 0.0 {
                        return Err(Error::Invalid("instance transform"));
                    }
                    Ok(InstanceVertex {
                        matrix: instance.matrix.as_mat4().to_cols_array(),
                        color: instance.color.0.extend(1.0).as_vec4().to_array(),
                    })
                })
                .collect::<Result<Vec<_>>>()?
        };
        let instance_buffer = slot.instances(
            &self.device,
            &self.queue,
            bytemuck::cast_slice(&instance_data),
        );
        if properties.stencil.is_some() && !target.options.stencil_buffer {
            return Err(Error::Invalid(
                "stencil material requires stencil attachment",
            ));
        }
        let two_pass = properties.transparent
            && !properties.force_single_pass
            && properties.side == Side::Double
            && topology == wgpu::PrimitiveTopology::TriangleList;
        let key = PipelineKey {
            extension_mask: if let Material::Physical(p) = material {
                p.extension_maps()
                    .iter()
                    .enumerate()
                    .fold(0, |mask, (i, map)| mask | (u16::from(map.is_some()) << i))
            } else {
                0
            },
            material_kind: uniforms.material[0] as u8,
            light_count: uniforms.material[3] as u8,
            light_types: uniforms
                .light_position
                .iter()
                .enumerate()
                .fold(0, |mask, (i, light)| mask | ((light[3] as u32) << (i * 3))),
            texture_mask: maps
                .iter()
                .enumerate()
                .fold(0, |mask, (i, map)| mask | (u8::from(map.is_some()) << i)),
            receive_shadow: uniforms.flags[1] > 0.5,
            physical: matches!(material, Material::Physical(_)),
            dashed: uniforms.line[1][1] > 0.5,
            clipping: uniforms.clipping.params[0] + uniforms.clipping.params[1] > 0.0,
            attachment_blending: properties.attachment_blending.clone(),
            blend: properties.blending.or_else(|| {
                properties
                    .transparent
                    .then_some(wgpu::BlendState::ALPHA_BLENDING)
            }),
            stencil: properties.stencil.clone(),
            depth_bias: properties
                .polygon_offset
                .map_or((0, 0), |(factor, units)| (units, factor.to_bits())),
            color_write: properties.color_write,
            instanced: !instances.is_empty(),
            shader_id: custom.map_or(0, |p| p.id),
            topology,
            side: match properties.side {
                Side::Front => 0,
                Side::Back => 1,
                Side::Double => {
                    if two_pass {
                        0
                    } else {
                        2
                    }
                }
            },
            transparent: properties.transparent,
            alpha_mask: properties.alpha_test > 0.0,
            alpha_to_coverage: properties.alpha_to_coverage && target.options.samples > 1,
            encode_srgb: target.options.encode_srgb,
            depth_test: properties.depth_test,
            depth_write: properties.depth_write,
            samples: target.options.samples.max(1),
            format: target.options.format,
            depth_format: target.depth_format(),
            attachment_formats: target.color_formats(),
            mirrored: uniforms.point[2] == 0.0
                && glam::Mat4::from_cols_array(&uniforms.model).determinant() < 0.0,
        };
        let mut pipelines = self.pipelines.borrow_mut();
        let create_pipeline = |key: &PipelineKey| {
            let layout = self
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("draw pipeline layout"),
                    bind_group_layouts: &custom
                        .map_or_else(|| vec![draw_layout], |p| vec![draw_layout, &p.layout]),
                    push_constant_ranges: &[],
                });
            let shader = custom.map_or(&self.shader, |p| &p.module);
            self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("material pipeline"),layout:Some(&layout),
            vertex:wgpu::VertexState {module:shader,entry_point:Some("vs_main"),compilation_options:wgpu::PipelineCompilationOptions {constants:&[("INSTANCED",if key.instanced {1.0}else{0.0})],..Default::default()},buffers:&[wgpu::VertexBufferLayout {array_stride:std::mem::size_of::<Vertex>() as u64,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x2,3=>Float32x4,4=>Float32x2,5=>Float32x4,11=>Float32x2]},instance_layout(key.instanced)]},
            fragment:Some(wgpu::FragmentState {module:shader,entry_point:Some("fs_main"),compilation_options:wgpu::PipelineCompilationOptions {constants:&[("LIGHT_COUNT",key.light_count as f64),("LIGHT_TYPES",key.light_types as f64),("COLOR_MAP",f64::from(key.texture_mask & 1 != 0)),("MR_MAP",f64::from(key.texture_mask & 2 != 0)),("NORMAL_MAP",f64::from(key.texture_mask & 4 != 0)),("AO_MAP",f64::from(key.texture_mask & 8 != 0)),("EMISSIVE_MAP",f64::from(key.texture_mask & 16 != 0)),("RECEIVE_SHADOW",f64::from(key.receive_shadow)),("MATERIAL_KIND",key.material_kind as f64),("PHYSICAL",if key.physical {1.0}else{0.0}),("EXTENSION_MAP_MASK",key.extension_mask as f64),("ENCODE_SRGB", f64::from(key.encode_srgb)),("ALPHA_MASK", if key.alpha_mask {1.0} else {0.0}),("CLIPPING",if key.clipping {1.0}else{0.0}),("LINE_DASH",if key.dashed {1.0}else{0.0})],..Default::default()},targets:&key.attachment_formats.iter().enumerate().map(|(i,&format)|Some(wgpu::ColorTargetState {format,blend:key.attachment_blending.get(i).copied().unwrap_or(key.blend),write_mask:if key.color_write {wgpu::ColorWrites::ALL}else{wgpu::ColorWrites::empty()}})).collect::<Vec<_>>()}),
            primitive:wgpu::PrimitiveState {topology,front_face:if key.mirrored {wgpu::FrontFace::Cw} else {wgpu::FrontFace::Ccw},strip_index_format:if topology==wgpu::PrimitiveTopology::LineStrip {Some(wgpu::IndexFormat::Uint32)} else {None},cull_mode:match key.side {0=>Some(wgpu::Face::Back),1=>Some(wgpu::Face::Front),_=>None},..Default::default()},
            depth_stencil:target.depth_format().map(|format|wgpu::DepthStencilState {format,depth_write_enabled:properties.depth_write && target.options.depth_buffer,depth_compare:if properties.depth_test {wgpu::CompareFunction::LessEqual} else {wgpu::CompareFunction::Always},stencil:key.stencil.clone().unwrap_or_default(),bias:wgpu::DepthBiasState{constant:key.depth_bias.0,slope_scale:f32::from_bits(key.depth_bias.1),clamp:0.0}}),multisample:wgpu::MultisampleState {count:target.options.samples.max(1),alpha_to_coverage_enabled:key.alpha_to_coverage,..Default::default()},multiview:None,cache:None})
        };
        let pipeline = pipelines
            .entry(key.clone())
            .or_insert_with(|| create_pipeline(&key))
            .clone();
        let back_pipeline = two_pass.then(|| {
            let mut back = key.clone();
            back.side = 1;
            pipelines
                .entry(back.clone())
                .or_insert_with(|| create_pipeline(&back))
                .clone()
        });
        Ok(Draw {
            viewport: custom.map_or(0, |p| p.viewport),
            object,
            stencil_reference: properties.stencil_reference,
            custom_bindings: custom.map(|p| p.bindings.clone()),
            instance_buffer,
            pipeline,
            back_pipeline,
            vertices: vertex_buffer,
            bind_group,
            range: 0..0,
            instances: 1,
            indices: None,
            indirect: None,
            indirect_offsets: Vec::new(),
            transparent: properties.transparent,
            render_order: 0,
            group_order: 0,
            depth: 0.0,
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_rgba(&self, target: &RenderTarget) -> Result<Vec<u8>> {
        if !matches!(
            target.options.format,
            wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Rgba8UnormSrgb
        ) {
            return Err(Error::Invalid("RGBA8 readback format"));
        }
        let stride = (target.width * 4).div_ceil(256) * 256;
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback"),
            size: stride as u64 * target.height as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &target.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: target.layer,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(stride),
                    rows_per_image: Some(target.height),
                },
            },
            wgpu::Extent3d {
                width: target.width,
                height: target.height,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit([encoder.finish()]);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                let _ = sender.send(result);
            });
        self.device
            .poll(wgpu::PollType::Wait)
            .map_err(|e| Error::Gpu(e.to_string()))?;
        receiver
            .recv()
            .map_err(|e| Error::Gpu(e.to_string()))?
            .map_err(|e| Error::Gpu(e.to_string()))?;
        let result = buffer
            .slice(..)
            .get_mapped_range()
            .chunks_exact(stride as usize)
            .flat_map(|row| row[..target.width as usize * 4].iter().copied())
            .collect();
        buffer.unmap();
        Ok(result)
    }
}
