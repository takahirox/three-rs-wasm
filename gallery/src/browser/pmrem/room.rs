//! `pmremGenerator.fromScene( new RoomEnvironment(), sigma )`: RoomEnvironment
//! ( r187, unchanged from r186 ) drawn by fromScene's CubeCamera( 0.1, 100 )
//! into the PMREM's source with the WGSL r187 generates for its materials:
//! the room ( MeshStandardMaterial, BackSide ), the six instanced boxes
//! ( MeshStandardMaterial ) and the six emissive MeshLambertMaterial light
//! panels, lit by the PointLight( 0xffffff, 900, 28, 2 ); then the blur and
//! the PMREM's levels. All of it runs once, on the GPU.
use super::super::lights_projector::{m3, m4, pack};
use super::{HALF, Pmrem, face_cameras};
use crate::{Error, Result, geometry::BoxGeometry, math::*, renderer::Renderer};
use wgpu::util::DeviceExt;

const ROOM_VS: &str = include_str!("room_vs.wgsl");
const ROOM_FS: &str = include_str!("room_fs.wgsl");
const BOXES_VS: &str = include_str!("room_boxes_vs.wgsl");
const BOXES_FS: &str = include_str!("room_boxes_fs.wgsl");
const LIGHT_VS: &str = include_str!("room_light_vs.wgsl");
const LIGHT_FS: &str = include_str!("room_light_fs.wgsl");
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// The scene's position.y.
const OFFSET: f64 = -3.5;
/// The PointLight: its color × intensity, position, distance and decay.
const LIGHT: (f64, [f64; 3], f64, f64) = (900., [0.418, 16.199, 0.300], 28., 2.);
/// The instanced boxes: position, rotation.y and scale.
const BOXES: [([f64; 3], f64, [f64; 3]); 6] = [
    ([-10.906, 2.009, 1.846], -0.195, [2.328, 7.905, 4.651]),
    ([-5.607, -0.754, -0.758], 0.994, [1.970, 1.534, 3.955]),
    ([6.167, 0.857, 7.803], 0.561, [3.927, 6.285, 3.687]),
    ([-2.017, 0.018, 6.124], 0.333, [2.002, 4.566, 2.064]),
    ([2.291, -0.756, -2.621], -0.286, [1.546, 1.552, 1.496]),
    ([-2.193, -0.369, -5.547], 0.516, [3.875, 3.487, 2.986]),
];
/// The light panels: position, scale and emissiveIntensity.
const PANELS: [([f64; 3], [f64; 3], f64); 6] = [
    ([-16.116, 14.37, 8.208], [0.1, 2.428, 2.739], 50.),
    ([-16.109, 18.021, -8.207], [0.1, 2.425, 2.751], 50.),
    ([14.904, 12.198, -1.832], [0.15, 4.265, 6.331], 17.),
    ([-0.462, 8.89, 14.520], [4.38, 5.441, 0.088], 43.),
    ([3.235, 11.486, -12.541], [2.5, 2.0, 0.1], 20.),
    ([0.0, 20.0, 0.0], [1.0, 0.1, 1.0], 100.),
];

fn pipeline(
    r: &Renderer,
    label: &str,
    (vs, fs): (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
    cull: wgpu::Face,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    let (vs, fs) = (module(vs), module(fs));
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
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(cull),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}
fn bind(
    r: &Renderer,
    layout: wgpu::BindGroupLayout,
    entries: &[(u32, wgpu::BindingResource)],
) -> wgpu::BindGroup {
    r.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("RoomEnvironment"),
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

/// The PMREM of RoomEnvironment, blurred by `sigma` radians.
pub(in super::super) fn room_environment(r: &Renderer, sigma: f64) -> Result<Pmrem> {
    let mut pmrem = Pmrem::new(r, 256)?;
    pmrem.set_blur(r, sigma)?;
    let init = |label, data: &[u8], usage| {
        r.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: data,
                usage,
            })
    };
    // BoxGeometry() without its uv.
    let geometry = BoxGeometry::build(1., 1., 1.)?;
    let floats = |name: &str| -> Result<Vec<f32>> {
        let a = geometry
            .attributes
            .get(name)
            .ok_or(Error::Invalid("RoomEnvironment box"))?;
        (0..a.count())
            .flat_map(|i| (0..3).map(move |k| (i, k)))
            .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
            .collect()
    };
    let vertex = wgpu::BufferUsages::VERTEX;
    let positions = init(
        "RoomEnvironment position",
        bytemuck::cast_slice(&floats("position")?),
        vertex,
    );
    let normals = init(
        "RoomEnvironment normal",
        bytemuck::cast_slice(&floats("normal")?),
        vertex,
    );
    let index = geometry
        .index
        .as_ref()
        .ok_or(Error::Invalid("RoomEnvironment index"))?;
    let indices = init(
        "RoomEnvironment index",
        bytemuck::cast_slice(index),
        wgpu::BufferUsages::INDEX,
    );
    let count = index.len() as u32;
    let scene = Matrix4::from_translation(Vector3::new(0., OFFSET, 0.));
    let instances: Vec<f32> = BOXES
        .iter()
        .flat_map(|(p, ry, s)| {
            Matrix4::from_scale_rotation_translation(
                Vector3::from_array(*s),
                Quaternion::from_rotation_y(*ry),
                Vector3::from_array(*p),
            )
            .to_cols_array()
            .map(|v| v as f32)
        })
        .collect();
    let instances = init(
        "RoomEnvironment instances",
        bytemuck::cast_slice(&instances),
        vertex,
    );
    let attribute = |location| {
        [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: location,
        }]
    };
    let layout = |attributes| wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes,
    };
    // The room and the panels read normal at 0 and position at 1, the boxes
    // position at 0, normal at 1 and the instance matrix at 2 to 5.
    let (first, second) = (attribute(0), attribute(1));
    let matrix = wgpu::vertex_attr_array![2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4];
    let instanced = wgpu::VertexBufferLayout {
        array_stride: 64,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &matrix,
    };
    let room_pipeline = pipeline(
        r,
        "RoomEnvironment room",
        (ROOM_VS, ROOM_FS),
        &[layout(&first), layout(&second)],
        wgpu::Face::Front,
    );
    let boxes_pipeline = pipeline(
        r,
        "RoomEnvironment boxes",
        (BOXES_VS, BOXES_FS),
        &[layout(&first), layout(&second), instanced],
        wgpu::Face::Back,
    );
    let light_pipeline = pipeline(
        r,
        "RoomEnvironment panel",
        (LIGHT_VS, LIGHT_FS),
        &[layout(&first), layout(&second)],
        wgpu::Face::Back,
    );
    let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    let uniform = |source: &str, name: &str, values: &[(&str, &[f64])]| -> Result<wgpu::Buffer> {
        Ok(init(
            "RoomEnvironment uniforms",
            &pack(source, name, values)?,
            wgpu::BufferUsages::UNIFORM,
        ))
    };
    let normal = |model: Matrix4| m3(model.inverse().transpose());
    let room_model = scene
        * Matrix4::from_scale_rotation_translation(
            Vector3::new(31.713, 28.305, 28.591),
            Quaternion::IDENTITY,
            Vector3::new(-0.757, 13.219, 0.717),
        );
    let room_object = uniform(
        ROOM_FS,
        "objectStruct",
        &[
            ("nodeUniform0", &[1.; 3]),
            ("nodeUniform1", &[1.]),
            ("nodeUniform2", &[0.]),
            ("nodeUniform3", &[1.]),
            ("nodeUniform5", &normal(room_model)),
            ("nodeUniform6", &[0.; 3]),
            ("nodeUniform7", &[1.]),
            ("nodeUniform9", &m4(room_model)),
        ],
    )?;
    let room_group = bind(
        r,
        room_pipeline.get_bind_group_layout(1),
        &[
            (0, room_object.as_entire_binding()),
            (1, wgpu::BindingResource::Sampler(&clamp)),
            (2, wgpu::BindingResource::TextureView(&r.dfg)),
        ],
    );
    let boxes_object = uniform(
        BOXES_FS,
        "objectStruct",
        &[
            ("nodeUniform4", &[1.; 3]),
            ("nodeUniform5", &[1.]),
            ("nodeUniform6", &[0.]),
            ("nodeUniform7", &[1.]),
            ("nodeUniform9", &normal(scene)),
            ("nodeUniform10", &[0.; 3]),
            ("nodeUniform11", &[1.]),
            ("nodeUniform13", &m4(scene)),
        ],
    )?;
    let boxes_group = bind(
        r,
        boxes_pipeline.get_bind_group_layout(1),
        &[
            (0, boxes_object.as_entire_binding()),
            (1, wgpu::BindingResource::Sampler(&clamp)),
            (2, wgpu::BindingResource::TextureView(&r.dfg)),
        ],
    );
    let panel_groups = PANELS
        .iter()
        .map(|(p, s, intensity)| {
            let model = scene
                * Matrix4::from_scale_rotation_translation(
                    Vector3::from_array(*s),
                    Quaternion::IDENTITY,
                    Vector3::from_array(*p),
                );
            let object = uniform(
                LIGHT_FS,
                "objectStruct",
                &[
                    ("nodeUniform0", &[0.; 3]),
                    ("nodeUniform1", &[1.]),
                    ("nodeUniform2", &[1.; 3]),
                    ("nodeUniform3", &[*intensity]),
                    ("nodeUniform5", &normal(model)),
                    ("nodeUniform7", &m4(model)),
                ],
            )?;
            Ok(bind(
                r,
                light_pipeline.get_bind_group_layout(1),
                &[(0, object.as_entire_binding())],
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    let depth = r
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("RoomEnvironment depth"),
            size: wgpu::Extent3d {
                width: pmrem.size,
                height: pmrem.size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&Default::default());
    let (intensity, light, distance, decay) = LIGHT;
    let light_world = scene.transform_point3(Vector3::from_array(light));
    let mut encoder = r.device.create_command_encoder(&Default::default());
    // fromScene's CubeCamera( 0.1, 100 ): each face cleared to the renderer's
    // clear color ( black ).
    let cameras = face_cameras(0.1, 100., Vector3::ZERO);
    for ((projection, view, _), face) in cameras.iter().zip(pmrem.source_faces()) {
        let light_view = view.transform_point3(light_world).to_array();
        let camera = |source: &str, names: [&str; 4]| -> Result<wgpu::Buffer> {
            uniform(
                source,
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", &m4(*projection)),
                    ("cameraViewMatrix", &m4(*view)),
                    (names[0], &light_view),
                    (names[1], &[intensity; 3]),
                    (names[2], &[distance]),
                    (names[3], &[decay]),
                ],
            )
        };
        let room_camera = camera(
            ROOM_FS,
            ["nodeUniform10", "nodeUniform11", "nodeUniform12", "nodeUniform13"],
        )?;
        let boxes_camera = camera(
            BOXES_FS,
            ["nodeUniform14", "nodeUniform15", "nodeUniform16", "nodeUniform17"],
        )?;
        let light_camera = camera(
            LIGHT_FS,
            ["nodeUniform6", "nodeUniform8", "nodeUniform9", "nodeUniform10"],
        )?;
        let group = |p: &wgpu::RenderPipeline, buffer: &wgpu::Buffer| {
            bind(
                r,
                p.get_bind_group_layout(0),
                &[(0, buffer.as_entire_binding())],
            )
        };
        let room_camera = group(&room_pipeline, &room_camera);
        let boxes_camera = group(&boxes_pipeline, &boxes_camera);
        let light_camera = group(&light_pipeline, &light_camera);
        let mut p = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RoomEnvironment"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: face,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.),
                    store: wgpu::StoreOp::Discard,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        p.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
        // Room and panels: normal at 0, position at 1.
        p.set_vertex_buffer(0, normals.slice(..));
        p.set_vertex_buffer(1, positions.slice(..));
        p.set_pipeline(&room_pipeline);
        p.set_bind_group(0, &room_camera, &[]);
        p.set_bind_group(1, &room_group, &[]);
        p.draw_indexed(0..count, 0, 0..1);
        p.set_pipeline(&light_pipeline);
        p.set_bind_group(0, &light_camera, &[]);
        for panel in &panel_groups {
            p.set_bind_group(1, panel, &[]);
            p.draw_indexed(0..count, 0, 0..1);
        }
        // The boxes: position at 0, normal at 1, the instances at 2.
        p.set_vertex_buffer(0, positions.slice(..));
        p.set_vertex_buffer(1, normals.slice(..));
        p.set_vertex_buffer(2, instances.slice(..));
        p.set_pipeline(&boxes_pipeline);
        p.set_bind_group(0, &boxes_camera, &[]);
        p.set_bind_group(1, &boxes_group, &[]);
        p.draw_indexed(0..count, 0, 0..BOXES.len() as u32);
    }
    pmrem.encode_from_scene(&mut encoder);
    r.queue.submit([encoder.finish()]);
    Ok(pmrem)
}
