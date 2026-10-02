//! webgl_simple_gi: a vertex-colored torus knot inside a back-faced box with
//! eight random colors, lit by SimpleGI. Each animation frame, after the
//! page's render, SimpleGI takes the next 32 torus vertices, renders the
//! scene's clone from each one ( a 90° camera at the vertex, looking along
//! its normal ) into a 32 × 32 target and stores the target's mean color as
//! the vertex color. When every vertex is done, the clone takes the current
//! colors and the next of three bounces starts; the first bounce's clone has
//! no color attribute, which WebGL reads as black.
//!
//! The original renders each view and reads its pixels back synchronously,
//! summing them on the CPU. The port renders the frame's 32 views into the
//! tiles of one 1024 × 32 target and sums each tile's quantized 8-bit texels
//! in a compute pass that writes the vertex colors in place, so the colors
//! stay on the GPU; the clone's colors are a GPU copy. The views are linear
//! ( as WebGLRenderer writes render targets ); the canvas is sRGB-encoded in
//! the shader, as MeshBasicMaterial's output is.
use super::controls_attributes::{Controls, camera_state};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use wgpu::util::DeviceExt;

const SIZE: u32 = 32;
/// Views per animation frame.
const BATCH: u32 = 32;
const BOUNCES: u32 = 3;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const VIEW: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
/// One uniform slot per draw ( dynamic offsets ).
const SLOT: u64 = 256;
const SHADER: &str = r#"
struct Camera { view_projection: mat4x4<f32> };
struct Material { color: vec4<f32> };
@group( 0 ) @binding( 0 ) var<uniform> camera: Camera;
@group( 1 ) @binding( 0 ) var<uniform> material: Material;
struct Varyings { @builtin( position ) position: vec4<f32>, @location( 0 ) color: vec3<f32> };
@vertex fn torus( @location( 0 ) position: vec3<f32>, @location( 1 ) color: vec3<f32> ) -> Varyings {
	return Varyings( camera.view_projection * vec4( position, 1.0 ), color * material.color.rgb );
}
@vertex fn room( @location( 0 ) position: vec3<f32> ) -> Varyings {
	return Varyings( camera.view_projection * vec4( position, 1.0 ), material.color.rgb );
}
@fragment fn view_fs( in: Varyings ) -> @location( 0 ) vec4<f32> {
	return vec4( in.color, 1.0 );
}
// sRGBTransferOETF.
@fragment fn screen_fs( in: Varyings ) -> @location( 0 ) vec4<f32> {
	let c = in.color;
	return vec4( select( pow( c, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), c * 12.92, c <= vec3( 0.0031308 ) ), 1.0 );
}
"#;
/// Each workgroup sums one tile's texels as the page's readRenderTargetPixels
/// loop does ( 8-bit values ) and writes the mean to its vertex's color.
const SUM: &str = r#"
struct Batch { start: u32, count: u32 };
@group( 0 ) @binding( 0 ) var tiles: texture_2d<f32>;
@group( 0 ) @binding( 1 ) var<storage, read_write> colors: array<f32>;
@group( 0 ) @binding( 2 ) var<uniform> batch: Batch;
var<workgroup> sums: array<vec3<u32>, 64>;
@compute @workgroup_size( 64 ) fn main( @builtin( workgroup_id ) tile: vec3<u32>, @builtin( local_invocation_index ) lane: u32 ) {
	var sum = vec3<u32>( 0u );
	for ( var k = lane; k < 1024u; k += 64u ) {
		let texel = textureLoad( tiles, vec2( tile.x * 32u + k % 32u, k / 32u ), 0 ).rgb;
		sum += vec3<u32>( round( texel * 255.0 ) );
	}
	sums[ lane ] = sum;
	workgroupBarrier();
	for ( var stride = 32u; stride > 0u; stride >>= 1u ) {
		if ( lane < stride ) { sums[ lane ] += sums[ lane + stride ]; }
		workgroupBarrier();
	}
	if ( lane == 0u && tile.x < batch.count ) {
		let total = vec3<f32>( sums[ 0 ] ) / f32( 1024u * 255u );
		let vertex = batch.start + tile.x;
		colors[ vertex * 3u ] = total.r;
		colors[ vertex * 3u + 1u ] = total.g;
		colors[ vertex * 3u + 2u ] = total.b;
	}
}
"#;
/// The fixture's Math.random: a 32-bit LCG seeded with 186.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(self.0) / 4294967296.
    }
}
/// `Color.setHex` with color management: sRGB bytes to linear.
fn linear(hex: u32) -> [f32; 4] {
    let c = |shift: u32| {
        let c = f64::from((hex >> shift) & 255) / 255.;
        (if c < 0.04045 {
            c * 0.0773993808
        } else {
            (c * 0.9478672986 + 0.0521327014).powf(2.4)
        }) as f32
    };
    [c(16), c(8), c(0), 1.]
}
/// `Matrix4.lookAt` for a camera at `eye` facing `target` ( Y up ).
fn look_at(eye: Vector3, target: Vector3) -> Matrix4 {
    let up = Vector3::Y;
    let mut z = eye - target;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    z = z.normalize();
    let mut x = up.cross(z);
    if x.length_squared() == 0. {
        z.z += 0.0001;
        z = z.normalize();
        x = up.cross(z);
    }
    let x = x.normalize();
    let y = z.cross(x);
    Matrix4::from_cols(x.extend(0.), y.extend(0.), z.extend(0.), eye.extend(1.))
}
/// `PerspectiveCamera` projection with WebGPU's 0..1 depth.
fn perspective(fov: f64, aspect: f64, near: f64, far: f64) -> Matrix4 {
    let top = near * (fov.to_radians() * 0.5).tan();
    let (height, width) = (2. * top, 2. * top * aspect);
    Matrix4::from_cols_array(&[
        2. * near / width,
        0.,
        0.,
        0.,
        0.,
        2. * near / height,
        0.,
        0.,
        0.,
        0.,
        -far / (far - near),
        -1.,
        0.,
        0.,
        -far * near / (far - near),
        0.,
    ])
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    pipelines: [wgpu::RenderPipeline; 2],
}
pub(super) struct Demo {
    controls: Controls,
    pending: bool,
    bounces: u32,
    vertex: u32,
    /// Torus world positions and normals ( the mesh stays at the origin ).
    points: Vec<(Vector3, Vector3)>,
    torus: (wgpu::Buffer, wgpu::Buffer, u32),
    /// The torus's live colors ( the page's ) and the clone's.
    colors: wgpu::Buffer,
    clone_colors: wgpu::Buffer,
    room: (wgpu::Buffer, wgpu::Buffer, Vec<(u32, u32)>),
    /// Camera slots: the page's camera, then the frame's views.
    cameras: wgpu::Buffer,
    camera_group: wgpu::BindGroup,
    material_group: wgpu::BindGroup,
    layouts: [wgpu::BindGroupLayout; 2],
    view_pipelines: [wgpu::RenderPipeline; 2],
    tiles: wgpu::TextureView,
    tile_depth: wgpu::TextureView,
    sum: wgpu::ComputePipeline,
    sum_group: wgpu::BindGroup,
    batch: wgpu::Buffer,
    targets: Option<Targets>,
}
fn texture(
    r: &Renderer,
    label: &str,
    (width, height): (u32, u32),
    format: wgpu::TextureFormat,
) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
        .create_view(&Default::default())
}
fn pipeline(
    r: &Renderer,
    layouts: &[wgpu::BindGroupLayout; 2],
    torus: bool,
    fragment: &str,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("simple gi"),
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });
    let layout = r
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("simple gi"),
            bind_group_layouts: &[&layouts[0], &layouts[1]],
            push_constant_ranges: &[],
        });
    let attributes = [
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 1,
        },
    ];
    let buffer = |k: usize| wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &attributes[k..=k],
    };
    let buffers = [buffer(0), buffer(1)];
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("simple gi"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some(if torus { "torus" } else { "room" }),
                compilation_options: Default::default(),
                buffers: if torus { &buffers } else { &buffers[..1] },
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some(fragment),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            // The torus is front-sided, the room back-sided.
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(if torus {
                    wgpu::Face::Back
                } else {
                    wgpu::Face::Front
                }),
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
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 4.);
        let mut controls = Controls::new(None, (1., 10.), std::f64::consts::PI, true);
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let f32s = |g: &BufferGeometry, name: &str| -> Result<Vec<f32>> {
            match g.attributes.get(name) {
                Some(Attribute::F32(a)) => Ok(a.array().to_vec()),
                _ => Err(Error::Invalid("simple gi attribute")),
            }
        };
        let knot = TorusKnotGeometry::build(0.75, 0.3, 128, 32, 1, 3)?;
        let positions = f32s(&knot, "position")?;
        let normals = f32s(&knot, "normal")?;
        let index = knot.index.clone().ok_or(Error::Invalid("torus index"))?;
        let point = |a: &[f32]| Vector3::new(f64::from(a[0]), f64::from(a[1]), f64::from(a[2]));
        let points: Vec<_> = positions
            .chunks(3)
            .zip(normals.chunks(3))
            .map(|(p, n)| (point(p), point(n).normalize()))
            .collect();
        let vertex = wgpu::BufferUsages::VERTEX;
        let colors = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("torus colors"),
            size: (positions.len() * 4) as u64,
            usage: vertex | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let clone_colors = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("clone torus colors"),
            size: (positions.len() * 4) as u64,
            usage: vertex | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let room = BoxGeometry::build(3., 3., 3.)?;
        let room_index = room.index.clone().ok_or(Error::Invalid("room index"))?;
        let groups = room
            .groups
            .iter()
            .map(|g| (g.start as u32, g.count as u32))
            .collect();
        // Eight room materials from the seeded Math.random ( the box uses six ).
        let mut random = Random(186);
        let mut materials = vec![0u8; SLOT as usize * 9];
        let mut put = |k: usize, color: [f32; 4]| {
            materials[k * SLOT as usize..][..16].copy_from_slice(bytemuck::cast_slice(&color));
        };
        put(0, [1., 1., 1., 1.]);
        for k in 0..8 {
            put(
                1 + k,
                linear((random.next() * f64::from(0xffffff)).floor() as u32),
            );
        }
        let materials = init(
            "simple gi materials",
            &materials,
            wgpu::BufferUsages::UNIFORM,
        );
        let cameras = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("simple gi cameras"),
            size: SLOT * u64::from(1 + BATCH),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let layout = |label| {
            r.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(label),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                })
        };
        let layouts = [layout("simple gi camera"), layout("simple gi material")];
        let slot = |buffer: &wgpu::Buffer, layout: &wgpu::BindGroupLayout| {
            r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer,
                        offset: 0,
                        size: wgpu::BufferSize::new(64),
                    }),
                }],
            })
        };
        let camera_group = slot(&cameras, &layouts[0]);
        // Material slots: the torus's white, then the room's eight colors.
        let material_group = slot(&materials, &layouts[1]);
        let view_pipelines = [
            pipeline(r, &layouts, true, "view_fs", VIEW),
            pipeline(r, &layouts, false, "view_fs", VIEW),
        ];
        let tiles = texture(r, "simple gi views", (SIZE * BATCH, SIZE), VIEW);
        let tile_depth = texture(r, "simple gi view depth", (SIZE * BATCH, SIZE), DEPTH);
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("simple gi sum"),
            source: wgpu::ShaderSource::Wgsl(SUM.into()),
        });
        let sum = r
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("simple gi sum"),
                layout: None,
                module: &module,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                cache: None,
            });
        let batch = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("simple gi batch"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let sum_group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &sum.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&tiles),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: colors.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: batch.as_entire_binding(),
                },
            ],
        });
        Ok(Self {
            controls,
            pending: true,
            bounces: 0,
            vertex: 0,
            points,
            torus: (
                init("torus positions", bytemuck::cast_slice(&positions), vertex),
                init(
                    "torus index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                index.len() as u32,
            ),
            colors,
            clone_colors,
            room: (
                init(
                    "room positions",
                    bytemuck::cast_slice(&f32s(&room, "position")?),
                    vertex,
                ),
                init(
                    "room index",
                    bytemuck::cast_slice(&room_index),
                    wgpu::BufferUsages::INDEX,
                ),
                groups,
            ),
            cameras,
            camera_group,
            material_group,
            layouts,
            view_pipelines,
            tiles,
            tile_depth,
            sum,
            sum_group,
            batch,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        self.pending |= animate;
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    /// The scene: the torus ( its live or cloned colors ) and the room's six
    /// faces, with the camera in `camera` slot.
    fn scene_pass(
        &self,
        pass: &mut wgpu::RenderPass,
        pipelines: [&wgpu::RenderPipeline; 2],
        camera: u32,
        colors: &wgpu::Buffer,
    ) {
        let camera = camera * SLOT as u32;
        pass.set_bind_group(0, &self.camera_group, &[camera]);
        pass.set_pipeline(pipelines[0]);
        pass.set_bind_group(1, &self.material_group, &[0]);
        pass.set_vertex_buffer(0, self.torus.0.slice(..));
        pass.set_vertex_buffer(1, colors.slice(..));
        pass.set_index_buffer(self.torus.1.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.torus.2, 0, 0..1);
        pass.set_pipeline(pipelines[1]);
        pass.set_vertex_buffer(0, self.room.0.slice(..));
        pass.set_index_buffer(self.room.1.slice(..), wgpu::IndexFormat::Uint32);
        for (k, &(start, count)) in self.room.2.iter().enumerate() {
            pass.set_bind_group(1, &self.material_group, &[(1 + k as u32) * SLOT as u32]);
            pass.draw_indexed(start..start + count, 0, 0..1);
        }
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let resized = self.targets.as_ref().is_none_or(|t| {
            t.width != out.width || t.height != out.height || t.format != out.options.format
        });
        if resized {
            let screen = RenderTarget::with_options(
                &r.device,
                out.width,
                out.height,
                RenderTargetOptions {
                    samples: 0,
                    depth_buffer: false,
                    ..out.options.clone()
                },
            )?;
            self.targets = Some(Targets {
                width: out.width,
                height: out.height,
                format: out.options.format,
                depth: texture(r, "simple gi depth", (out.width, out.height), DEPTH),
                pipelines: [
                    pipeline(r, &self.layouts, true, "screen_fs", out.options.format),
                    pipeline(r, &self.layouts, false, "screen_fs", out.options.format),
                ],
                screen,
            });
        }
        let advance = std::mem::take(&mut self.pending);
        if !advance && !resized {
            return Ok(true);
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (fov, near, far, aspect) = match camera {
            Camera::Perspective(p) => (p.fov, p.near, p.far, p.aspect),
            _ => (70., 0.1, 100., 1.),
        };
        let mut slots = vec![0u8; (SLOT * u64::from(1 + BATCH)) as usize];
        let mut put = |k: usize, m: Matrix4| {
            let m: Vec<f32> = m.to_cols_array().iter().map(|&v| v as f32).collect();
            slots[k * SLOT as usize..][..64].copy_from_slice(bytemuck::cast_slice(&m));
        };
        put(0, perspective(fov, aspect, near, far) * world.inverse());
        // SimpleGI's compute(): the next views, while bounces remain.
        let total = self.points.len() as u32;
        let batch = if advance && self.bounces < BOUNCES {
            let start = self.vertex;
            let count = BATCH.min(total - start);
            let projection = perspective(90., 1., 0.01, 100.);
            for k in 0..count {
                let (position, normal) = self.points[(start + k) as usize];
                put(
                    1 + k as usize,
                    projection * look_at(position, position + normal).inverse(),
                );
            }
            Some((start, count))
        } else {
            None
        };
        r.queue.write_buffer(&self.cameras, 0, &slots[..64]);
        if let Some((_, count)) = batch {
            let end = (SLOT * u64::from(count)) as usize;
            r.queue
                .write_buffer(&self.cameras, SLOT, &slots[SLOT as usize..][..end]);
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("simple gi targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("simple gi scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            self.scene_pass(
                &mut pass,
                [&t.pipelines[0], &t.pipelines[1]],
                0,
                &self.colors,
            );
        }
        if let Some((start, count)) = batch {
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("simple gi views"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.tiles,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.tile_depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Discard,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                for k in 0..count {
                    let x = (k * SIZE) as f32;
                    pass.set_viewport(x, 0., SIZE as f32, SIZE as f32, 0., 1.);
                    pass.set_scissor_rect(k * SIZE, 0, SIZE, SIZE);
                    self.scene_pass(
                        &mut pass,
                        [&self.view_pipelines[0], &self.view_pipelines[1]],
                        1 + k,
                        &self.clone_colors,
                    );
                }
            }
            r.queue
                .write_buffer(&self.batch, 0, bytemuck::cast_slice(&[start, count]));
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&self.sum);
                pass.set_bind_group(0, &self.sum_group, &[]);
                pass.dispatch_workgroups(count, 1, 1);
            }
            self.vertex += count;
            if self.vertex >= total {
                // scene.clone(): the clone's torus takes the current colors.
                encoder.copy_buffer_to_buffer(
                    &self.colors,
                    0,
                    &self.clone_colors,
                    0,
                    self.colors.size(),
                );
                self.bounces += 1;
                self.vertex = 0;
            }
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
