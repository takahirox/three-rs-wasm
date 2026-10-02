//! PMREMGenerator (three.js r186, WebGPU): a source pass into the
//! 3·2^lodMax × 4·2^lodMax cubeUV target over level 0's planes, then the
//! GGX levels, each blurred into the ping-pong target and copied back. The planes, the
//! targets and every pass's uniforms are built once, so regenerating (as
//! PMREMNode does for a render-target environment each frame) allocates
//! nothing. The GGX pass runs the WGSL r186 generates for that lodMax.
use super::lights_projector::{m4, pack};
use crate::{Result, math::*, renderer::*};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
/// LOD_MIN and EXTRA_LODS.
const LOD_MIN: i32 = 4;
const EXTRA_LODS: usize = 6;
/// The 36 vertices of one level's planes: twelve bytes each.
const PLANES: u64 = 36 * 12;
/// The GGX blur's WGSL at lodMax 8 ( its cubeUV texel sizes are constants ).
pub(super) const GGX_8: (&str, &str) = (
    include_str!("retro/pmrem_ggx_vs.wgsl"),
    include_str!("retro/pmrem_ggx_fs.wgsl"),
);

/// A bind group over `layout`.
pub(super) fn bind(
    r: &Renderer,
    layout: wgpu::BindGroupLayout,
    entries: &[(u32, wgpu::BindingResource)],
) -> wgpu::BindGroup {
    r.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &entries
            .iter()
            .map(|(binding, resource)| wgpu::BindGroupEntry {
                binding: *binding,
                resource: resource.clone(),
            })
            .collect::<Vec<_>>(),
    })
}
/// A pipeline over `buffers` with back faces culled; `depth` is None for
/// color-only passes.
#[allow(clippy::too_many_arguments)]
pub(super) fn raw_pipeline(
    r: &Renderer,
    label: &str,
    vs: &str,
    fs: &str,
    buffers: &[wgpu::VertexBufferLayout],
    format: wgpu::TextureFormat,
    samples: u32,
    depth: Option<(wgpu::CompareFunction, bool)>,
    cw: bool,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module(vs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &module(fs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: if cw {
                    wgpu::FrontFace::Cw
                } else {
                    wgpu::FrontFace::Ccw
                },
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: depth.map(|(compare, write)| wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: write,
                depth_compare: compare,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: samples,
                ..Default::default()
            },
            multiview: None,
            cache: None,
        })
}
const DIRECTION: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    format: wgpu::VertexFormat::Float32x3,
    offset: 0,
    shader_location: 0,
}];
const POSITION: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    format: wgpu::VertexFormat::Float32x3,
    offset: 0,
    shader_location: 1,
}];
/// The planes' layouts: outputDirection at location 0, position at 1.
const LAYOUTS: [wgpu::VertexBufferLayout<'static>; 2] = [
    wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &DIRECTION,
    },
    wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &POSITION,
    },
];
/// The flat camera's render struct ( OrthographicCamera( -1, 1, 1, -1, 0, 1 ) ).
pub(super) fn flat_camera(r: &Renderer, source: &str) -> Result<wgpu::Buffer> {
    Ok(r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("PMREM camera"),
            contents: &pack(
                source,
                "renderStruct",
                &[
                    (
                        "cameraProjectionMatrix",
                        &[
                            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                        ],
                    ),
                    ("cameraViewMatrix", &m4(Matrix4::IDENTITY)),
                ],
            )?,
            usage: wgpu::BufferUsages::UNIFORM,
        }))
}
/// One GGX pass: its viewport, level and bind groups ( blur or copy ).
struct Pass {
    copy: bool,
    viewport: [f32; 4],
    lod: usize,
    group: wgpu::BindGroup,
}
pub(super) struct Pmrem {
    /// The cubeUV texture materials sample.
    pub view: wgpu::TextureView,
    ping_pong: wgpu::TextureView,
    directions: wgpu::Buffer,
    positions: wgpu::Buffer,
    ggx: wgpu::RenderPipeline,
    camera: wgpu::BindGroup,
    passes: Vec<Pass>,
    cube_size: u32,
}
impl Pmrem {
    /// The targets, planes and passes for `lod_max`, with the GGX WGSL r186
    /// generates for it.
    pub(super) fn new(
        r: &Renderer,
        clamp: &wgpu::Sampler,
        lod_max: i32,
        (ggx_vs, ggx_fs): (&str, &str),
    ) -> Result<Self> {
        let lods = (lod_max - LOD_MIN + 1) as usize + EXTRA_LODS;
        let cube_size = 1u32 << lod_max;
        let size_lods: Vec<u32> = (0..lods)
            .map(|i| 1 << (lod_max - (i as i32).min(lod_max - LOD_MIN)))
            .collect();
        let texture = || {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("PMREM"),
                    size: wgpu::Extent3d {
                        width: 3 * cube_size,
                        height: 4 * cube_size,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: HALF,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING
                        | wgpu::TextureUsages::COPY_SRC,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let (view, ping_pong) = (texture(), texture());
        // _createPlanes( lodMax ): per level, six faces of six vertices with
        // the directions overshooting each face by one texel.
        const FACES: [usize; 6] = [3, 1, 5, 0, 4, 2];
        let mut positions = vec![0f32; lods * 108];
        let mut directions = vec![0f32; lods * 108];
        for (lod, &size) in size_lods.iter().enumerate() {
            let texel = 1. / (size as f32 - 2.);
            let (min, max) = (-texel, 1. + texel);
            let uv = [min, min, max, min, max, max, min, min, max, max, min, max];
            for (face, &index) in FACES.iter().enumerate() {
                let x = (face % 3) as f32 * 2. / 3. - 1.;
                let y = if face > 2 { 0. } else { -1. };
                let corners = [
                    [x, y],
                    [x + 2. / 3., y],
                    [x + 2. / 3., y + 1.],
                    [x, y],
                    [x + 2. / 3., y + 1.],
                    [x, y + 1.],
                ];
                for (vertex, [px, py]) in corners.iter().enumerate() {
                    let at = lod * 108 + (index * 6 + vertex) * 3;
                    positions[at..at + 3].copy_from_slice(&[*px, *py, 0.]);
                    let u = uv[vertex * 2] * 2. - 1.;
                    let v = uv[vertex * 2 + 1] * 2. - 1.;
                    let direction = match index {
                        0 => [1., v, u],
                        1 => [-u, 1., -v],
                        2 => [-u, v, 1.],
                        3 => [-1., v, -u],
                        4 => [-u, -1., v],
                        _ => [u, v, -1.],
                    };
                    directions[at..at + 3].copy_from_slice(&direction);
                }
            }
        }
        let init = |data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("PMREM"),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let (directions, positions) = (
            init(bytemuck::cast_slice(&directions), vertex),
            init(bytemuck::cast_slice(&positions), vertex),
        );
        let ggx = source_pipeline(r, "PMREM GGX", ggx_vs, ggx_fs);
        let camera = bind(
            r,
            ggx.get_bind_group_layout(0),
            &[(0, flat_camera(r, ggx_vs)?.as_entire_binding())],
        );
        let mut passes = vec![];
        let last = (lods - 1) as f64;
        for (lod_out, &size) in size_lods.iter().enumerate().take(lods).skip(1) {
            let lod_in = lod_out - 1;
            let target_roughness = lod_out as f64 / last;
            let source_roughness = lod_in as f64 / last;
            let roughness =
                (target_roughness * target_roughness - source_roughness * source_roughness).sqrt()
                    * (target_roughness * 1.25);
            let x = 3 * size * (lod_out as u32).saturating_sub((lod_max - LOD_MIN) as u32);
            let y = 4 * (cube_size - size);
            let viewport = [x, y, 3 * size, 2 * size].map(|v| v as f32);
            for copy in [false, true] {
                // The blur reads the previous level, the copy the level just written.
                let mip = lod_max - if copy { lod_out } else { lod_in } as i32;
                let object = init(
                    &pack(
                        ggx_vs,
                        "objectStruct",
                        &[
                            ("nodeUniform0", &[if copy { 0. } else { roughness }]),
                            ("nodeUniform1", &[mip as f64]),
                            ("nodeUniform5", &m4(Matrix4::IDENTITY)),
                        ],
                    )?,
                    wgpu::BufferUsages::UNIFORM,
                );
                let source = if copy { &ping_pong } else { &view };
                let group = bind(
                    r,
                    ggx.get_bind_group_layout(1),
                    &[
                        (0, object.as_entire_binding()),
                        (1, wgpu::BindingResource::Sampler(clamp)),
                        (2, wgpu::BindingResource::TextureView(source)),
                    ],
                );
                passes.push(Pass {
                    copy,
                    viewport,
                    lod: lod_out,
                    group,
                });
            }
        }
        Ok(Self {
            view,
            ping_pong,
            directions,
            positions,
            ggx,
            camera,
            passes,
            cube_size,
        })
    }
    /// Encodes the source pass ( `source` over level 0's planes, viewport
    /// 3·2^lodMax × 2·2^lodMax ) and the GGX levels.
    pub(super) fn encode(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        source: &wgpu::RenderPipeline,
        groups: [&wgpu::BindGroup; 2],
    ) {
        let pass = |encoder: &mut wgpu::CommandEncoder,
                    view: &wgpu::TextureView,
                    clear: bool,
                    [x, y, w, h]: [f32; 4],
                    pipeline: &wgpu::RenderPipeline,
                    groups: [&wgpu::BindGroup; 2],
                    lod: usize| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("PMREM"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if clear {
                            wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_viewport(x, y, w, h, 0., 1.);
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, groups[0], &[]);
            pass.set_bind_group(1, groups[1], &[]);
            let range = lod as u64 * PLANES..(lod as u64 + 1) * PLANES;
            pass.set_vertex_buffer(0, self.directions.slice(range.clone()));
            pass.set_vertex_buffer(1, self.positions.slice(range));
            pass.draw(0..36, 0..1);
        };
        let size = self.cube_size as f32;
        pass(
            encoder,
            &self.view,
            true,
            [0., 0., 3. * size, 2. * size],
            source,
            groups,
            0,
        );
        self.encode_levels(encoder);
    }
    /// Encodes the GGX levels over a level 0 already written ( as
    /// fromScene's cube camera does ).
    pub(super) fn encode_levels(&self, encoder: &mut wgpu::CommandEncoder) {
        let pass = |encoder: &mut wgpu::CommandEncoder,
                    view: &wgpu::TextureView,
                    [x, y, w, h]: [f32; 4],
                    group: &wgpu::BindGroup,
                    lod: usize| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("PMREM"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_viewport(x, y, w, h, 0., 1.);
            pass.set_pipeline(&self.ggx);
            pass.set_bind_group(0, &self.camera, &[]);
            pass.set_bind_group(1, group, &[]);
            let range = lod as u64 * PLANES..(lod as u64 + 1) * PLANES;
            pass.set_vertex_buffer(0, self.directions.slice(range.clone()));
            pass.set_vertex_buffer(1, self.positions.slice(range));
            pass.draw(0..36, 0..1);
        };
        for p in &self.passes {
            let destination = if p.copy { &self.view } else { &self.ping_pong };
            pass(encoder, destination, p.viewport, &p.group, p.lod);
        }
    }
}
/// A source pipeline over the planes ( outputDirection, position ).
pub(super) fn source_pipeline(
    r: &Renderer,
    label: &str,
    vs: &str,
    fs: &str,
) -> wgpu::RenderPipeline {
    raw_pipeline(r, label, vs, fs, &LAYOUTS, HALF, 1, None, false)
}
/// An RGBE .hdr environment as HDRLoader loads it (half float, flipped,
/// mipmapped on the GPU), through PMREMGenerator.fromEquirectangular at
/// lodMax 8; also the equirect texture.
pub(super) async fn rgbe_pmrem(
    r: &Renderer,
    clamp: &wgpu::Sampler,
    url: &str,
) -> Result<(Pmrem, wgpu::TextureView)> {
    use super::gltf_viewer::fetch;
    use super::retro::mipmapped_raw;
    use super::shadowmap_opacity::Mipmaps;
    const EQUIRECT_VS: &str = include_str!("retro/pmrem_equirect_vs.wgsl");
    const EQUIRECT_FS: &str = include_str!("retro/pmrem_equirect_fs.wgsl");
    let (width, height, texels) = super::trackball_sprites::parse_rgbe(&fetch(url).await?)?;
    let row = width as usize * 4;
    let texels: Vec<u16> = texels.chunks(row).rev().flatten().copied().collect();
    let mut mipmaps = Mipmaps::new(r);
    let equirect = mipmapped_raw(
        r,
        &mut mipmaps,
        bytemuck::cast_slice(&texels),
        (width, height),
        8,
        HALF,
    );
    let pmrem = Pmrem::new(r, clamp, 8, GGX_8)?;
    let source = source_pipeline(r, "PMREM equirect", EQUIRECT_VS, EQUIRECT_FS);
    let object = r
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("PMREM equirect"),
            contents: &pack(
                EQUIRECT_VS,
                "objectStruct",
                &[("nodeUniform3", &m4(Matrix4::IDENTITY))],
            )?,
            usage: wgpu::BufferUsages::UNIFORM,
        });
    let groups = [
        bind(
            r,
            source.get_bind_group_layout(0),
            &[(0, flat_camera(r, EQUIRECT_VS)?.as_entire_binding())],
        ),
        bind(
            r,
            source.get_bind_group_layout(1),
            &[
                (0, wgpu::BindingResource::Sampler(clamp)),
                (1, wgpu::BindingResource::TextureView(&equirect)),
                (2, object.as_entire_binding()),
            ],
        ),
    ];
    let mut encoder = r.device.create_command_encoder(&Default::default());
    pmrem.encode(&mut encoder, &source, [&groups[0], &groups[1]]);
    r.queue.submit([encoder.finish()]);
    Ok((pmrem, equirect))
}
