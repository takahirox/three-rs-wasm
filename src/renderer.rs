use crate::{Error, Result, material::*, math::*, scene::*};
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Weak},
};
use wgpu::util::DeviceExt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PipelineKey {
    topology: wgpu::PrimitiveTopology,
    side: u8,
    transparent: bool,
    alpha_mask: bool,
    depth_test: bool,
    depth_write: bool,
    samples: u32,
    format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    attachments: u32,
    mirrored: bool,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
    color: [f32; 4],
    corner: [f32; 2],
    tangent: [f32; 4],
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
}

pub use crate::render_target::{RenderTarget, RenderTarget3D, RenderTargetOptions};

pub struct Renderer {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    layout: wgpu::BindGroupLayout,
    shader: wgpu::ShaderModule,
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
    backgrounds: RefCell<crate::background::PipelineCache>,
    presentations:
        RefCell<HashMap<wgpu::TextureFormat, (wgpu::BindGroupLayout, wgpu::RenderPipeline)>>,
    environment_builds: std::cell::Cell<u64>,
}
struct Draw {
    pipeline: wgpu::RenderPipeline,
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
    /// (resident material textures, cumulative uploads, cumulative environment filters).
    pub fn resource_counts(&self) -> (usize, u64, u64) {
        let cache = self.textures.borrow();
        (cache.len(), cache.uploads, self.environment_builds.get())
    }
    /// Release cached resources whose application-owned inputs have been dropped.
    pub fn collect_resources(&self) {
        self.textures.borrow_mut().prune();
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
                required_features: adapter.features() & wgpu::Features::INDIRECT_FIRST_INSTANCE,
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
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("draw layout"),
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
                    "{}\n{}",
                    include_str!("shaders/cube_uv.wgsl"),
                    include_str!("shader.wgsl")
                )
                .into(),
            ),
        });
        Ok(Self {
            adapter,
            device,
            queue,
            layout,
            shader,
            pipelines: RefCell::new(HashMap::new()),
            textures: RefCell::new(Default::default()),
            white: Arc::new(Texture::from_rgba(1, 1, vec![255; 4], false)?),
            environment: RefCell::new(None),
            dfg,
            backgrounds: RefCell::new(Default::default()),
            presentations: RefCell::new(Default::default()),
            environment_builds: std::cell::Cell::new(0),
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
        let mut cache = self.presentations.borrow_mut();
        let (layout, pipeline) = cache.entry(format).or_insert_with(|| {
            let shader = self
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("present"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("present.wgsl").into()),
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
        let parameters = [exposure as f32, if aces { 1.0 } else { 0.0 }, 0.0, 0.0];
        let uniform = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("exposure"),
                contents: bytemuck::cast_slice(&parameters),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("present"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&target.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: uniform.as_entire_binding(),
                },
            ],
        });
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
        self.textures.borrow_mut().prune();
        {
            let mut cached = self.environment.borrow_mut();
            if let Some(image) = &scene.environment {
                if !cached
                    .as_ref()
                    .is_some_and(|(owner, _)| owner.ptr_eq(&Arc::downgrade(image)))
                {
                    self.environment_builds
                        .set(self.environment_builds.get() + 1);
                    *cached = Some((
                        Arc::downgrade(image),
                        crate::environment_gpu::build(&self.device, &self.queue, image)?,
                    ));
                }
            } else {
                *cached = None;
            }
        }
        scene.update()?;
        let (camera_data, camera_world) = scene.camera(camera)?;
        if camera_world.determinant() == 0.0 {
            return Err(Error::Invalid("singular camera transform"));
        }
        let perspective = matches!(camera_data, crate::camera::Camera::Perspective(_));
        let projection = camera_data.projection_matrix()?;
        let view_projection = projection * camera_world.inverse();
        let camera_layers = scene.get(camera)?.layers;
        let camera_position = camera_world.w_axis.truncate();
        let background = if scene.background_environment {
            self.environment.borrow().as_ref().map(|(_, env)| {
                crate::background::prepare(
                    &mut self.backgrounds.borrow_mut(),
                    &self.device,
                    target,
                    env,
                    projection,
                    camera_world,
                    [scene.environment_rotation, scene.background_blur],
                )
            })
        } else {
            None
        };
        let mut ambient = Vector3::ZERO;
        let mut light_position = [[0.0; 4]; 8];
        let mut light_color = [[0.0; 4]; 8];
        let mut light_params = [[0.0; 4]; 8];
        let mut light_count = 0;
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
                match light {
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
        visible.sort_by_key(|h| scene.get(*h).expect("valid node").render_order);
        let mut draws = Vec::new();
        let mut after_hooks = Vec::new();
        for h in visible {
            let n = scene.get(h)?;
            if !n.layers.test(camera_layers) {
                continue;
            }
            if n.geometry().is_none() {
                continue;
            }
            if n.frustum_culled
                && !crate::raycast::FrustumIntersect::intersects_frustum(
                    n,
                    &Frustum::from_projection(view_projection),
                )?
            {
                continue;
            }
            let before = n.render_hooks.before.clone();
            if let Some(callback) = before {
                callback(scene, h);
                scene.update_world_matrix(h, true, false)?;
            }
            let n = scene.get(h)?;
            let Some(geometry) = n.geometry() else {
                continue;
            };
            if let Some(callback) = n.render_hooks.after.clone() {
                after_hooks.push((h, callback));
            }
            let (materials, topology) = match &n.kind {
                NodeKind::Mesh(m) => (
                    m.materials.iter().map(|m| m.as_ref()).collect::<Vec<_>>(),
                    wgpu::PrimitiveTopology::TriangleList,
                ),
                NodeKind::Line(l) => (
                    vec![l.material.as_ref()],
                    if l.segments {
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
            for group in groups {
                let Some(material) = materials.get(group.material_index) else {
                    return Err(Error::Invalid("group material index"));
                };
                let properties = material.properties();
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
                let mut vertices = Vec::new();
                let positions = geometry
                    .attributes
                    .get("position")
                    .ok_or(Error::Invalid("position attribute"))?;
                for index in 0..geometry.vertex_count() {
                    let position = positions.vector3(index)?.as_vec3().to_array();
                    let normal = geometry
                        .attributes
                        .get("normal")
                        .map(|a| a.vector3(index))
                        .transpose()?
                        .unwrap_or(Vector3::Z)
                        .as_vec3()
                        .to_array();
                    let mut uv = [0.0; 2];
                    if let Some(a) = geometry.attributes.get("uv") {
                        uv = [
                            a.get_component(index, 0)? as f32,
                            a.get_component(index, 1)? as f32,
                        ];
                    }
                    if let Some(texture) = &properties.map {
                        let centered = Vector2::new(uv[0] as f64, uv[1] as f64) - texture.center;
                        let (s, c) = texture.rotation.sin_cos();
                        let transformed = Vector2::new(
                            c * centered.x + s * centered.y,
                            -s * centered.x + c * centered.y,
                        ) * texture.repeat
                            + texture.center
                            + texture.offset;
                        uv = [
                            transformed.x as f32,
                            if texture.flip_y {
                                1.0 - transformed.y as f32
                            } else {
                                transformed.y as f32
                            },
                        ];
                    }
                    let mut color = [1.0; 4];
                    if properties.vertex_colors
                        && let Some(a) = geometry.attributes.get("color")
                    {
                        for (c, value) in color.iter_mut().enumerate().take(a.item_size().min(4)) {
                            *value = a.get_component(index, c)? as f32;
                        }
                    }
                    let mut tangent = [0.0; 4];
                    if let Some(a) = geometry.attributes.get("tangent") {
                        for (c, value) in tangent.iter_mut().enumerate() {
                            *value = a.get_component(index, c)? as f32;
                        }
                    }
                    vertices.push(Vertex {
                        position,
                        normal,
                        uv,
                        color,
                        corner: [0.0, 0.0],
                        tangent,
                    });
                }
                let is_points = matches!(n.kind, NodeKind::Points(_));
                if is_points {
                    let centers = vertices;
                    vertices = Vec::with_capacity(centers.len() * 6);
                    for center in centers {
                        for corner in [
                            [-1.0, -1.0],
                            [1.0, -1.0],
                            [-1.0, 1.0],
                            [-1.0, 1.0],
                            [1.0, -1.0],
                            [1.0, 1.0],
                        ] {
                            let mut vertex = center;
                            vertex.corner = corner;
                            vertex.uv = [corner[0] * 0.5 + 0.5, 0.5 - corner[1] * 0.5];
                            vertices.push(vertex);
                        }
                    }
                }
                let point = match material {
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
                    Material::Standard(m) => {
                        (1.0, m.roughness as f32, m.metalness as f32, m.emissive.0)
                    }
                    _ => (0.0, 1.0, 0.0, Vector3::ZERO),
                };
                let u = Uniforms {
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
                        Material::Standard(m) => [
                            m.normal_scale.x as f32,
                            m.normal_scale.y as f32,
                            m.occlusion_strength as f32,
                            properties.alpha_test as f32,
                        ],
                        _ => [1.0, 1.0, 1.0, properties.alpha_test as f32],
                    },
                    environment: [
                        if scene.environment.is_some() {
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
                        Material::Standard(m) => [
                            f32::from(m.normal_map.is_some()),
                            f32::from(properties.transparent),
                            f32::from(m.emissive_map.is_some()),
                            f32::from(m.metallic_roughness_map.is_some()),
                        ],
                        _ => [0.0, f32::from(properties.transparent), 0.0, 0.0],
                    },
                    light_position,
                    light_color,
                    light_params,
                };
                let mut draw = self.prepare_draw(
                    &vertices,
                    &u,
                    material,
                    if is_points {
                        wgpu::PrimitiveTopology::TriangleList
                    } else {
                        topology
                    },
                    target,
                )?;
                let factor = if is_points { 6 } else { 1 };
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
                draw.instances = geometry.instance_count.unwrap_or(1);
                if let Some(indices) = &geometry.index {
                    if indices
                        .iter()
                        .any(|i| *i as usize >= geometry.vertex_count())
                    {
                        return Err(Error::Invalid("geometry index"));
                    }
                    let indices = if is_points {
                        indices
                            .iter()
                            .flat_map(|i| (0..6).map(move |corner| i * 6 + corner))
                            .collect::<Vec<_>>()
                    } else {
                        indices.clone()
                    };
                    draw.indices = Some(self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("indices"),
                            contents: bytemuck::cast_slice(&indices),
                            usage: wgpu::BufferUsages::INDEX,
                        },
                    ));
                }
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
                    draw.indirect = Some(self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("indirect commands"),
                            contents: bytemuck::cast_slice(&commands),
                            usage: wgpu::BufferUsages::INDIRECT,
                        },
                    ));
                }
                draws.push(draw);
                for attribute in geometry.attributes.values() {
                    attribute.notify_uploaded();
                }
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
        {
            let c = scene.background.0;
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
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: c.x,
                                g: c.y,
                                b: c.z,
                                a: 1.0,
                            }),
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
                            load: wgpu::LoadOp::Clear(1.0),
                            store: if target.options.samples > 1
                                && !target.options.store_multisampled_depth_buffer
                            {
                                wgpu::StoreOp::Discard
                            } else {
                                wgpu::StoreOp::Store
                            },
                        }),
                        stencil_ops: target.options.stencil_buffer.then_some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(0),
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
                occlusion_query_set: None,
            });
            let [x, y, w, h] = target.viewport;
            pass.set_viewport(x as f32, y as f32, w as f32, h as f32, 0.0, 1.0);
            if let Some([x, y, w, h]) = target.scissor {
                pass.set_scissor_rect(x, y, w, h);
            }
            if let Some((pipeline, bindings)) = &background {
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, bindings, &[]);
                pass.draw(0..3, 0..1);
            }
            for draw in &draws {
                pass.set_pipeline(&draw.pipeline);
                pass.set_bind_group(0, &draw.bind_group, &[]);
                pass.set_vertex_buffer(0, draw.vertices.slice(..));
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
        }
        self.queue.submit([encoder.finish()]);
        for (object, callback) in after_hooks {
            callback(scene, object);
        }
        Ok(())
    }
    fn prepare_draw(
        &self,
        vertices: &[Vertex],
        uniforms: &Uniforms,
        material: &Material,
        topology: wgpu::PrimitiveTopology,
        target: &RenderTarget,
    ) -> Result<Draw> {
        let properties = material.properties();
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertices"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let uniform_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("uniforms"),
                contents: bytemuck::bytes_of(uniforms),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let standard = match material {
            Material::Standard(m) => Some(m),
            _ => None,
        };
        let maps = [
            properties.map.as_ref(),
            standard.and_then(|m| m.metallic_roughness_map.as_ref()),
            standard.and_then(|m| m.normal_map.as_ref()),
            standard.and_then(|m| m.occlusion_map.as_ref()),
            standard.and_then(|m| m.emissive_map.as_ref()),
        ];
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
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("draw bindings"),
            layout: &self.layout,
            entries: &bindings,
        });
        let key = PipelineKey {
            topology,
            side: match properties.side {
                Side::Front => 0,
                Side::Back => 1,
                Side::Double => 2,
            },
            transparent: properties.transparent,
            alpha_mask: properties.alpha_test > 0.0,
            depth_test: properties.depth_test,
            depth_write: properties.depth_write,
            samples: target.options.samples.max(1),
            format: target.options.format,
            depth_format: target.depth_format(),
            attachments: target.options.count,
            mirrored: uniforms.point[2] == 0.0
                && glam::Mat4::from_cols_array(&uniforms.model).determinant() < 0.0,
        };
        let mut pipelines = self.pipelines.borrow_mut();
        let pipeline=pipelines.entry(key).or_insert_with(|| {
        let layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("draw pipeline layout"),
                bind_group_layouts: &[&self.layout],
                push_constant_ranges: &[],
            });
        self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("material pipeline"),layout:Some(&layout),
            vertex:wgpu::VertexState {module:&self.shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:std::mem::size_of::<Vertex>() as u64,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x2,3=>Float32x4,4=>Float32x2,5=>Float32x4]}]},
            fragment:Some(wgpu::FragmentState {module:&self.shader,entry_point:Some("fs_main"),compilation_options:wgpu::PipelineCompilationOptions {constants:&[("ALPHA_MASK", if key.alpha_mask {1.0} else {0.0})],..Default::default()},targets:&(0..target.options.count).map(|_|Some(wgpu::ColorTargetState {format:target.options.format,blend:properties.transparent.then_some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})).collect::<Vec<_>>()}),
            primitive:wgpu::PrimitiveState {topology,front_face:if key.mirrored {wgpu::FrontFace::Cw} else {wgpu::FrontFace::Ccw},strip_index_format:if topology==wgpu::PrimitiveTopology::LineStrip {Some(wgpu::IndexFormat::Uint32)} else {None},cull_mode:match properties.side {Side::Front=>Some(wgpu::Face::Back),Side::Back=>Some(wgpu::Face::Front),Side::Double=>None},..Default::default()},
            depth_stencil:target.depth_format().map(|format|wgpu::DepthStencilState {format,depth_write_enabled:properties.depth_write && target.options.depth_buffer,depth_compare:if properties.depth_test {wgpu::CompareFunction::LessEqual} else {wgpu::CompareFunction::Always},stencil:Default::default(),bias:Default::default()}),multisample:wgpu::MultisampleState {count:target.options.samples.max(1),..Default::default()},multiview:None,cache:None})
        }).clone();
        Ok(Draw {
            pipeline,
            vertices: vertex_buffer,
            bind_group,
            range: 0..vertices.len() as u32,
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
