//! r187's PMREMGenerator.fromEquirectangular: the equirect HDR drawn into a
//! half-float cube render target ( its face size a quarter of the image's
//! width, at least 256 ) through CubeCamera's six faces, that target's mip
//! chain as WebGPUTexturePassUtils generates it, then each level of the
//! PMREM cube ( down to 8 texels, LOD_MIN 3 ) drawn from it: GGX VNDF
//! importance sampling ( 256 samples ) for the sharp levels, and the
//! integration over a 16-texel source mip for the three roughest. Every
//! stage runs the WGSL r187 generates for it ( loader_materialx/pmrem_*.wgsl,
//! mipmap.wgsl ), all on the GPU at load.
use super::super::gltf_viewer::fetch;
use super::super::lights_projector::{m4, pack};
use super::super::pmrem_cube_uv::bind;
use crate::{Error, Result, geometry::BoxGeometry, math::*, renderer::Renderer};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const EQUIRECT_VS: &str = include_str!("pmrem_equirect_vs.wgsl");
const EQUIRECT_FS: &str = include_str!("pmrem_equirect_fs.wgsl");
const GGX_VS: &str = include_str!("pmrem_ggx_vs.wgsl");
const GGX_FS: &str = include_str!("pmrem_ggx_fs.wgsl");
const INTEGRATION_VS: &str = include_str!("pmrem_integration_vs.wgsl");
const INTEGRATION_FS: &str = include_str!("pmrem_integration_fs.wgsl");
const MIPMAP: &str = include_str!("mipmap.wgsl");
const MIN_SIZE: u32 = 256;
const LOD_MIN: u32 = 3;
const GGX_SAMPLES: f64 = 256.;
const INTEGRATION_SIZE: f64 = 16.;
const INTEGRATION_LEVELS: u32 = 3;

/// The PMREM: its cube texture's view ( every level ) and its last level,
/// which PMREMNode's maxLod uniform holds.
pub(super) struct CubePmrem {
    pub view: wgpu::TextureView,
    pub max_lod: u32,
}

/// CubeCamera's six face cameras in the WebGPU coordinate system ( fov -90,
/// near 1, far 10, at the origin ): each one's projection, view and world
/// matrices.
fn face_cameras() -> [(Matrix4, Matrix4, Matrix4); 6] {
    // PerspectiveCamera.updateProjectionMatrix with fov -90: top = -near.
    let (near, far) = (1., 10.);
    let projection = Matrix4::from_cols_array(&[
        -1.,
        0.,
        0.,
        0., //
        0.,
        -1.,
        0.,
        0., //
        0.,
        0.,
        -far / (far - near),
        -1., //
        0.,
        0.,
        -far * near / (far - near),
        0.,
    ]);
    // Matrix4.lookAt( eye, target, up ) for a camera at the origin.
    let look = |target: Vector3, up: Vector3| {
        let z = (Vector3::ZERO - target).normalize();
        let x = up.cross(z).normalize();
        let y = z.cross(x);
        let world = Matrix4::from_cols(x.extend(0.), y.extend(0.), z.extend(0.), Vector4::W);
        (projection, world.transpose(), world)
    };
    [
        look(Vector3::new(-1., 0., 0.), Vector3::new(0., -1., 0.)),
        look(Vector3::new(1., 0., 0.), Vector3::new(0., -1., 0.)),
        look(Vector3::new(0., 1., 0.), Vector3::new(0., 0., 1.)),
        look(Vector3::new(0., -1., 0.), Vector3::new(0., 0., -1.)),
        look(Vector3::new(0., 0., 1.), Vector3::new(0., -1., 0.)),
        look(Vector3::new(0., 0., -1.), Vector3::new(0., -1., 0.)),
    ]
}

fn cube_texture(r: &Renderer, label: &str, size: u32, levels: u32) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 6,
        },
        mip_level_count: levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: HALF,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}
fn cube_view(texture: &wgpu::Texture, level: Option<u32>) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::Cube),
        base_mip_level: level.unwrap_or(0),
        mip_level_count: level.map(|_| 1),
        ..Default::default()
    })
}
/// The 2D view of one face of one level: a render pass's target.
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
fn module(r: &Renderer, label: &str, source: &str) -> wgpu::ShaderModule {
    r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}
/// A PMREM material's pipeline: NoBlending, no depth, the box drawn from
/// inside ( every texel of a face covered once ).
fn pipeline(
    r: &Renderer,
    label: &str,
    (vs, fs): (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
) -> wgpu::RenderPipeline {
    let (vs, fs) = (module(r, label, vs), module(r, label, fs));
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: None,
            vertex: wgpu::VertexState {
                module: &vs,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &[Some(HALF.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}
fn uniform(r: &Renderer, label: &str, data: &[u8]) -> wgpu::Buffer {
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage: wgpu::BufferUsages::UNIFORM,
        })
}
fn pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("PMREM"),
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
    })
}

/// The PMREM of the Radiance HDR at `url`, as HDRLoader loads it ( half-float
/// RGBA, flipY, linear filters, no mipmaps ) and the page's
/// `scene.environment` makes it.
pub(super) async fn equirect_pmrem(r: &Renderer, url: &str) -> Result<CubePmrem> {
    let (width, height, texels) = super::super::trackball_sprites::parse_rgbe(&fetch(url).await?)?;
    // flipY: the last row uploads first.
    let row = width as usize * 4;
    let texels: Vec<u16> = texels.chunks(row).rev().flatten().copied().collect();
    let equirect = r
        .device
        .create_texture_with_data(
            &r.queue,
            &wgpu::TextureDescriptor {
                label: Some("PMREM equirect"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: HALF,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            bytemuck::cast_slice(&texels),
        )
        .create_view(&Default::default());
    // _setSize( width / 4 ): a power of two, at least MIN_SIZE.
    let quarter = (width / 4).max(1);
    let size = MIN_SIZE.max(1 << (31 - quarter.leading_zeros()));
    let levels = size.ilog2() + 1;
    let max_lod = size.ilog2() - LOD_MIN;
    // The source target ( generateMipmaps ) and the PMREM ( its prefiltered levels ).
    let source = cube_texture(r, "PMREM source", size, levels);
    let pmrem = cube_texture(r, "PMREM", size, max_lod + 1);
    // Samplers: the equirect's ( linear, clamped ), the render targets'
    // ( LinearMipmapLinearFilter ), and the mipmap passes' ( minFilter
    // linear only ).
    let sampler = |filter, mipmap| {
        r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: mipmap,
            ..Default::default()
        })
    };
    let linear = sampler(wgpu::FilterMode::Linear, wgpu::FilterMode::Nearest);
    let trilinear = sampler(wgpu::FilterMode::Linear, wgpu::FilterMode::Linear);
    let minify = sampler(wgpu::FilterMode::Nearest, wgpu::FilterMode::Nearest);
    // The box: BoxGeometry( 5, 5, 5 ), BackSide.
    let mut box_geometry = BoxGeometry::build(5., 5., 5.)?;
    let floats = |name: &str| -> Result<Vec<f32>> {
        let a = box_geometry
            .attributes
            .get(name)
            .ok_or(Error::Invalid("PMREM box"))?;
        (0..a.count())
            .flat_map(|i| (0..3).map(move |k| (i, k)))
            .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
            .collect()
    };
    let vertex = |label, data: &[f32]| {
        r.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::VERTEX,
            })
    };
    let positions = vertex("PMREM box position", &floats("position")?);
    let normals = vertex("PMREM box normal", &floats("normal")?);
    let index = box_geometry
        .index
        .take()
        .ok_or(Error::Invalid("PMREM box index"))?;
    let indices = r
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("PMREM box index"),
            contents: bytemuck::cast_slice(&index),
            usage: wgpu::BufferUsages::INDEX,
        });
    let position = [wgpu::vertex_attr_array![0 => Float32x3]];
    let normal = [wgpu::vertex_attr_array![1 => Float32x3]];
    let layout = |attributes| wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes,
    };
    let equirect_pipeline = pipeline(
        r,
        "PMREM equirect",
        (EQUIRECT_VS, EQUIRECT_FS),
        &[layout(&position[0])],
    );
    let ggx_pipeline = pipeline(r, "PMREM ggx", (GGX_VS, GGX_FS), &[layout(&position[0])]);
    let integration_pipeline = pipeline(
        r,
        "PMREM integration",
        (INTEGRATION_VS, INTEGRATION_FS),
        &[layout(&position[0]), layout(&normal[0])],
    );
    let identity = m4(Matrix4::IDENTITY);
    let identity3 = [1., 0., 0., 0., 1., 0., 0., 0., 1.];
    // Per face, its camera's render struct for each material.
    let cameras = face_cameras();
    let render = |source: &str,
                  (projection, view, world): &(Matrix4, Matrix4, Matrix4)|
     -> Result<wgpu::Buffer> {
        Ok(uniform(
            r,
            "PMREM camera",
            &pack(
                source,
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", &m4(*projection)),
                    ("cameraViewMatrix", &m4(*view)),
                    ("cameraWorldMatrix", &m4(*world)),
                ],
            )?,
        ))
    };
    let mut encoder = r.device.create_command_encoder(&Default::default());
    let draw = |encoder: &mut wgpu::CommandEncoder,
                target: &wgpu::TextureView,
                pipeline: &wgpu::RenderPipeline,
                groups: [&wgpu::BindGroup; 2],
                with_normals: bool| {
        let mut p = pass(encoder, target);
        p.set_pipeline(pipeline);
        p.set_bind_group(0, groups[0], &[]);
        p.set_bind_group(1, groups[1], &[]);
        p.set_vertex_buffer(0, positions.slice(..));
        if with_normals {
            p.set_vertex_buffer(1, normals.slice(..));
        }
        p.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
        p.draw_indexed(0..index.len() as u32, 0, 0..1);
    };
    // _textureToCubemap: the equirect material into the source's level 0.
    let object = uniform(
        r,
        "PMREM equirect object",
        &pack(EQUIRECT_VS, "objectStruct", &[("nodeUniform0", &identity)])?,
    );
    let material = bind(
        r,
        equirect_pipeline.get_bind_group_layout(1),
        &[
            (0, wgpu::BindingResource::Sampler(&linear)),
            (1, wgpu::BindingResource::TextureView(&equirect)),
            (2, object.as_entire_binding()),
        ],
    );
    for (face, camera) in cameras.iter().enumerate() {
        let camera = bind(
            r,
            equirect_pipeline.get_bind_group_layout(0),
            &[(0, render(EQUIRECT_VS, camera)?.as_entire_binding())],
        );
        let target = face_view(&source, face as u32, 0);
        draw(
            &mut encoder,
            &target,
            &equirect_pipeline,
            [&camera, &material],
            false,
        );
    }
    // The source's mipmaps: each level from the one above, a face at a time
    // ( main_cube, minFilter linear, no flip ).
    let mipmap = module(r, "PMREM mipmap", MIPMAP);
    let mipmap_pipeline = r
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("PMREM mipmap"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &mipmap,
                entry_point: Some("mainVS"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &mipmap,
                entry_point: Some("main_cube"),
                compilation_options: Default::default(),
                targets: &[Some(HALF.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });
    let no_flip = uniform(r, "PMREM no flip", bytemuck::bytes_of(&0u32));
    for level in 1..levels {
        let above = cube_view(&source, Some(level - 1));
        let group = bind(
            r,
            mipmap_pipeline.get_bind_group_layout(0),
            &[
                (0, wgpu::BindingResource::Sampler(&minify)),
                (1, wgpu::BindingResource::TextureView(&above)),
                (2, no_flip.as_entire_binding()),
            ],
        );
        for face in 0..6 {
            let target = face_view(&source, face, level);
            let mut p = pass(&mut encoder, &target);
            p.set_pipeline(&mipmap_pipeline);
            p.set_bind_group(0, &group, &[]);
            p.draw(0..3, face..face + 1);
        }
    }
    // _applyPMREM: each level of the PMREM from the source.
    let source_view = cube_view(&source, None);
    let ggx_cameras: Vec<wgpu::BindGroup> = cameras
        .iter()
        .map(|c| {
            Ok(bind(
                r,
                ggx_pipeline.get_bind_group_layout(0),
                &[(0, render(GGX_VS, c)?.as_entire_binding())],
            ))
        })
        .collect::<Result<_>>()?;
    let integration_cameras: Vec<wgpu::BindGroup> = cameras
        .iter()
        .map(|c| {
            Ok(bind(
                r,
                integration_pipeline.get_bind_group_layout(0),
                &[(0, render(INTEGRATION_VS, c)?.as_entire_binding())],
            ))
        })
        .collect::<Result<_>>()?;
    let source_lod = (size as f64 / INTEGRATION_SIZE).log2();
    for lod in 0..=max_lod {
        // PMREMGenerator.lodToRoughness
        let roughness = 1. - (1. - lod as f64 / max_lod as f64).sqrt();
        let integration = lod + INTEGRATION_LEVELS > max_lod;
        let (pipe, cameras, object) = if integration {
            let object = pack(
                INTEGRATION_FS,
                "objectStruct",
                &[
                    ("nodeUniform0", &identity),
                    ("nodeUniform1", &[roughness]),
                    ("nodeUniform2", &[INTEGRATION_SIZE]),
                    ("nodeUniform5", &identity3),
                    ("nodeUniform7", &[source_lod]),
                ],
            )?;
            (&integration_pipeline, &integration_cameras, object)
        } else {
            let lod_bias = if roughness > 0. {
                (size as f64).log2() + 0.5 * (6. / (GGX_SAMPLES * roughness.powi(4))).log2() + 0.5
            } else {
                0.
            };
            let object = pack(
                GGX_FS,
                "objectStruct",
                &[
                    ("nodeUniform0", &identity),
                    ("nodeUniform1", &[roughness]),
                    ("nodeUniform3", &[lod_bias]),
                ],
            )?;
            (&ggx_pipeline, &ggx_cameras, object)
        };
        let object = uniform(r, "PMREM object", &object);
        let material = bind(
            r,
            pipe.get_bind_group_layout(1),
            &[
                (0, object.as_entire_binding()),
                (1, wgpu::BindingResource::Sampler(&trilinear)),
                (2, wgpu::BindingResource::TextureView(&source_view)),
            ],
        );
        for (face, camera) in cameras.iter().enumerate() {
            let target = face_view(&pmrem, face as u32, lod);
            draw(
                &mut encoder,
                &target,
                pipe,
                [camera, &material],
                integration,
            );
        }
    }
    r.queue.submit([encoder.finish()]);
    Ok(CubePmrem {
        view: cube_view(&pmrem, None),
        max_lod,
    })
}
