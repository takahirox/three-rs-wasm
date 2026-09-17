//! Depth atlas shared by directional, spot and point light shadow passes.
use crate::{Error, Result, camera::*, material::*, math::*, scene::*};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, serde::Serialize)]
pub struct Shadow {
    pub near: f64,
    pub far: f64,
    /// Half width/height of a directional light's orthographic shadow camera.
    pub extent: f64,
    /// Offset added to the comparison depth, in normalized depth units.
    pub bias: f64,
    /// Offset the receiving surface along its world-space normal.
    pub normal_bias: f64,
    /// PCF radius in texels; zero selects a single filtered comparison.
    pub radius: f64,
}
impl Default for Shadow {
    fn default() -> Self {
        Self {
            near: 0.5,
            far: 500.0,
            extent: 5.0,
            bias: 0.0,
            normal_bias: 0.0,
            radius: 1.0,
        }
    }
}
pub(crate) struct Atlas {
    pub view: wgpu::TextureView,
    pub matrices: [[f32; 16]; 48],
    pub params: [[f32; 4]; 8],
    pub filters: [[f32; 4]; 8],
}
pub(crate) struct ShadowRenderer {
    layout: wgpu::BindGroupLayout,
    pipelines: [wgpu::RenderPipeline; 3],
    pub sampler: wgpu::Sampler,
    empty: wgpu::TextureView,
    white: Arc<Texture>,
    target: std::cell::RefCell<Option<ShadowTarget>>,
    slots: std::cell::RefCell<Vec<crate::draw_gpu::Slot>>,
}
struct ShadowTarget {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    layers: Vec<wgpu::TextureView>,
}
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniform {
    matrix: [f32; 16],
    alpha: [f32; 4],
    model: [f32; 16],
    clipping: crate::clipping::Clipping,
    uv_transform: [[f32; 4]; 3],
}
impl ShadowRenderer {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shadow depth"),
            source: wgpu::ShaderSource::Wgsl(
                concat!(
                    include_str!("shaders/deformation.wgsl"),
                    "\n",
                    include_str!("shaders/shadow.wgsl")
                )
                .into(),
            ),
        });
        let mut entries = vec![
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ];
        entries.extend(crate::deformation_gpu::layout_entries());
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shadow layout"),
            entries: &entries,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipelines = [Some(wgpu::Face::Back), Some(wgpu::Face::Front), None].map(|cull_mode| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("shadow depth"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: 80,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttribute {
                                    format: wgpu::VertexFormat::Float32x3,
                                    offset: 0,
                                    shader_location: 0,
                                },
                                wgpu::VertexAttribute {
                                    format: wgpu::VertexFormat::Float32x2,
                                    offset: 24,
                                    shader_location: 1,
                                },
                                wgpu::VertexAttribute {
                                    format: wgpu::VertexFormat::Float32x2,
                                    offset: 72,
                                    shader_location: 2,
                                },
                            ],
                        },
                        crate::renderer::instance_layout(true),
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[],
                }),
                primitive: wgpu::PrimitiveState {
                    cull_mode,
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                multiview: None,
                cache: None,
            })
        });
        let empty = depth_texture(device, 1, 1).create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        Self {
            layout,
            pipelines,
            empty,
            target: Default::default(),
            slots: Default::default(),
            sampler: device.create_sampler(&wgpu::SamplerDescriptor {
                compare: Some(wgpu::CompareFunction::LessEqual),
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            white: Arc::new(Texture::from_rgba(1, 1, vec![255; 4], false).unwrap()),
        }
    }
    pub fn collect_resources(&self) {
        self.slots.borrow_mut().clear();
        *self.target.borrow_mut() = None;
    }
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cache: &mut crate::texture_gpu::TextureCache,
        scene: &Scene,
        visible: &[Object3D],
        lights: &[Object3D],
        geometry_cache: &mut crate::geometry_gpu::Cache,
        deformation_cache: &mut crate::deformation_gpu::Cache,
    ) -> Result<Atlas> {
        let mut slots = self.slots.borrow_mut();
        let mut cursor = 0;
        let mut atlas = Atlas {
            view: self.empty.clone(),
            matrices: [[0.0; 16]; 48],
            params: [[0.0; 4]; 8],
            filters: [[0.0; 4]; 8],
        };
        let mut cameras = Vec::new();
        for (i, &handle) in lights.iter().enumerate() {
            let node = scene.get(handle)?;
            if !node.cast_shadow {
                continue;
            }
            let shadow = node.shadow;
            if ![
                shadow.near,
                shadow.far,
                shadow.extent,
                shadow.bias,
                shadow.normal_bias,
                shadow.radius,
            ]
            .iter()
            .all(|v| v.is_finite())
                || shadow.near <= 0.0
                || shadow.far <= shadow.near
                || shadow.extent <= 0.0
                || shadow.radius < 0.0
            {
                return Err(Error::Invalid("shadow camera or filter"));
            }
            let position = node.world_position();
            let mut views = Vec::new();
            let projection = match &node.kind {
                NodeKind::Light(Light::Directional { target, .. }) => {
                    views.push(view(position, *target, node.up));
                    OrthographicCamera {
                        left: -shadow.extent,
                        right: shadow.extent,
                        top: shadow.extent,
                        bottom: -shadow.extent,
                        near: shadow.near,
                        far: shadow.far,
                        ..Default::default()
                    }
                    .projection_matrix()?
                }
                NodeKind::Light(Light::Spot { target, angle, .. }) => {
                    views.push(view(position, *target, node.up));
                    PerspectiveCamera {
                        fov: (2.0 * angle).to_degrees().min(179.9),
                        aspect: 1.0,
                        near: shadow.near,
                        far: shadow.far,
                        ..Default::default()
                    }
                    .projection_matrix()?
                }
                NodeKind::Light(Light::Point { .. }) => {
                    for (axis, up) in [
                        (Vector3::X, Vector3::Y),
                        (-Vector3::X, Vector3::Y),
                        (Vector3::Y, Vector3::Z),
                        (-Vector3::Y, -Vector3::Z),
                        (Vector3::Z, Vector3::Y),
                        (-Vector3::Z, Vector3::Y),
                    ] {
                        views.push(view(position, position + axis, up));
                    }
                    PerspectiveCamera {
                        fov: 90.0,
                        aspect: 1.0,
                        near: shadow.near,
                        far: shadow.far,
                        ..Default::default()
                    }
                    .projection_matrix()?
                }
                _ => continue,
            };
            atlas.params[i] = [
                (cameras.len() + 1) as f32,
                shadow.bias as f32,
                shadow.normal_bias as f32,
                if views.len() == 6 { 1.0 } else { 0.0 },
            ];
            atlas.filters[i] = [shadow.radius as f32, 0.0, 0.0, 0.0];
            for v in views {
                let matrix = projection * v;
                atlas.matrices[cameras.len()] = matrix.as_mat4().to_cols_array();
                cameras.push(matrix);
            }
        }
        if cameras.is_empty() {
            slots.clear();
            *self.target.borrow_mut() = None;
            return Ok(atlas);
        }
        let size = scene.shadow_map_size;
        if size == 0 || size > device.limits().max_texture_dimension_2d {
            return Err(Error::Invalid("shadow map size"));
        }
        let mut cached = self.target.borrow_mut();
        if cached.as_ref().is_none_or(|t| {
            t.texture.width() != size || t.texture.depth_or_array_layers() != cameras.len() as u32
        }) {
            let texture = depth_texture(device, size, cameras.len() as u32);
            let view = texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                ..Default::default()
            });
            let layers = (0..cameras.len())
                .map(|layer| {
                    texture.create_view(&wgpu::TextureViewDescriptor {
                        dimension: Some(wgpu::TextureViewDimension::D2),
                        base_array_layer: layer as u32,
                        array_layer_count: Some(1),
                        ..Default::default()
                    })
                })
                .collect();
            *cached = Some(ShadowTarget {
                texture,
                view,
                layers,
            });
        }
        let target = cached.as_ref().expect("shadow target");
        atlas.view = target.view.clone();
        let mut encoder = device.create_command_encoder(&Default::default());
        for (layer, camera) in cameras.iter().enumerate() {
            let mut draws = Vec::new();
            for &handle in visible {
                let node = scene.get(handle)?;
                if !node.cast_shadow {
                    continue;
                }
                let NodeKind::Mesh(mesh) = &node.kind else {
                    continue;
                };
                if mesh.materials.iter().any(|m| {
                    matches!(m.as_ref(), crate::material::Material::Shader(_))
                        || m.properties().vertex_program.is_some()
                }) {
                    return Err(Error::Invalid(
                        "custom WGSL shadow caster: supply a separate evaluated shadow mesh",
                    ));
                }
                let g = &mesh.geometry;
                let gpu = geometry_cache.get(device, g, false, false, false, None)?;
                let deformation = deformation_cache.get(device, queue, scene, handle)?;
                let groups = if mesh.materials.len() > 1 {
                    g.groups.clone()
                } else {
                    vec![crate::geometry::Group {
                        start: 0,
                        count: g.draw_count(),
                        material_index: 0,
                    }]
                };
                for group in groups {
                    let material = mesh
                        .materials
                        .get(group.material_index)
                        .ok_or(Error::Invalid("shadow material group"))?
                        .properties();
                    if !material.visible {
                        continue;
                    }
                    let start = group.start.max(g.draw_range.start);
                    let end = group
                        .start
                        .saturating_add(group.count)
                        .min(g.draw_count())
                        .min(
                            g.draw_range
                                .start
                                .saturating_add(g.draw_range.count.unwrap_or(usize::MAX)),
                        );
                    if end <= start {
                        continue;
                    }
                    let map = material.map.as_ref().unwrap_or(&self.white);
                    let image = cache.get(device, queue, map)?;
                    let defaults = [Instance::default()];
                    let instances = if node.instances.is_empty() {
                        &defaults[..]
                    } else {
                        &node.instances
                    };
                    let data = instances
                        .iter()
                        .map(|i| {
                            if !i.matrix.is_finite() || i.matrix.determinant() <= 0.0 {
                                return Err(Error::Invalid("instance transform"));
                            }
                            Ok(crate::renderer::InstanceVertex {
                                matrix: i.matrix.as_mat4().to_cols_array(),
                                color: i.color.0.extend(1.0).as_vec4().to_array(),
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    if cursor == slots.len() {
                        slots.push(Default::default());
                    }
                    let slot = &mut slots[cursor];
                    cursor += 1;
                    let instance_buffer =
                        slot.instances(device, queue, bytemuck::cast_slice(&data));
                    let uniform = slot.uniform(
                        device,
                        queue,
                        bytemuck::bytes_of(&Uniform {
                            matrix: (*camera * node.matrix_world).as_mat4().to_cols_array(),
                            model: node.matrix_world.as_mat4().to_cols_array(),
                            clipping: crate::clipping::prepare(scene, material, true)?,
                            uv_transform: std::array::from_fn(|c| {
                                let v = map.uv_matrix().col(c).as_vec3();
                                [
                                    v.x,
                                    v.y,
                                    v.z,
                                    if c == 2 { map.tex_coord as f32 } else { 0.0 },
                                ]
                            }),
                            alpha: [
                                material.opacity as f32,
                                material.alpha_test as f32,
                                0.0,
                                0.0,
                            ],
                        }),
                    );
                    let bind = slot.bindings(
                        device,
                        &self.layout,
                        &[
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
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: uniform.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(&image.view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Sampler(&image.sampler),
                            },
                        ],
                    );
                    let mirrored = node.matrix_world.determinant() < 0.0;
                    let side = match material.side {
                        Side::Double => 2,
                        Side::Front => usize::from(mirrored),
                        Side::Back => usize::from(!mirrored),
                    };
                    draws.push((
                        gpu.clone(),
                        instance_buffer,
                        bind,
                        start as u32..end as u32,
                        instances.len() as u32,
                        side,
                    ));
                }
            }
            let attachment = &target.layers[layer];
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("shadow depth"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: attachment,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            for (gpu, instances, bindings, range, count, side) in &draws {
                pass.set_pipeline(&self.pipelines[*side]);
                pass.set_bind_group(0, bindings, &[]);
                pass.set_vertex_buffer(0, gpu.vertices.slice(..));
                pass.set_vertex_buffer(1, instances.slice(..));
                if let Some(indices) = &gpu.indices {
                    pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(range.clone(), 0, 0..*count);
                } else {
                    pass.draw(range.clone(), 0..*count);
                }
            }
        }
        slots.truncate(cursor);
        queue.submit([encoder.finish()]);
        Ok(atlas)
    }
}
fn view(position: Vector3, target: Vector3, mut up: Vector3) -> Matrix4 {
    let direction = (target - position).normalize_or_zero();
    if direction.cross(up).length_squared() < 1e-10 {
        up = if direction.z.abs() < 0.9 {
            Vector3::Z
        } else {
            Vector3::X
        };
    }
    Matrix4::look_at_rh(
        position,
        if direction == Vector3::ZERO {
            position - Vector3::Z
        } else {
            target
        },
        up,
    )
}
fn depth_texture(device: &wgpu::Device, size: u32, layers: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("shadow atlas"),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: layers,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}
