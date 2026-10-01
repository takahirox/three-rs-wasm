//! webgpu_instancing_morph: 1,024 instanced horses on a 32 × 32 grid, each
//! galloping at its own time offset: every frame the horse's morph-weight
//! track is sampled on the CPU for each instance and written into the
//! per-instance influence texture (as `setMorphAt` and
//! `morphTexture.needsUpdate` do), and the vertex stage blends the 15
//! resident morph targets. A SunLight casts two shadow cascades fitted around
//! the view into one atlas; the ground blends them, under a hemisphere light
//! and fog, while the camera circles. Every stage runs the WGSL three.js r186
//! generates for the page (in `instancing_morph/`).
use super::gltf_viewer::load_asset;
use super::lights_projector::{m3, m4, pack};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const COUNT: usize = 1024;
const TARGETS: usize = 15;
/// SunLightShadow: 2048² tiles, two cascades side by side.
const TILE: u32 = 2048;
const CASCADES: usize = 2;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("instancing_morph/", $name, ".wgsl"))
    };
}
fn uniform(r: &Renderer, label: &str, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
fn bind(
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
fn texture(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    samples: u32,
    sampled: bool,
) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("morph target"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | if sampled {
                    wgpu::TextureUsages::TEXTURE_BINDING
                } else {
                    wgpu::TextureUsages::empty()
                },
            view_formats: &[],
        })
        .create_view(&Default::default())
}
fn orthographic(half: f64, near: f64, far: f64) -> Matrix4 {
    Matrix4::from_cols_array(&[
        1. / half,
        0.,
        0.,
        0.,
        0.,
        1. / half,
        0.,
        0.,
        0.,
        0.,
        -1. / (far - near),
        0.,
        0.,
        0.,
        -near / (far - near),
        1.,
    ])
}
/// One SunLightShadow cascade: camera, shadow matrix, view depths and viewport.
struct Cascade {
    projection: Matrix4,
    view: Matrix4,
    matrix: Matrix4,
    data: [f64; 4],
    viewport: [f32; 4],
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    horse: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    ground: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
}
pub(super) struct Demo {
    time: f64,
    offsets: Vec<f64>,
    /// The morph-weight track: key times and 15 weights per key.
    keys: Vec<f64>,
    weights: Vec<f32>,
    positions: wgpu::Buffer,
    colors: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    ground: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer),
    influences: wgpu::Texture,
    targets_view: wgpu::TextureView,
    instances: wgpu::Buffer,
    horse_render: wgpu::Buffer,
    horse_object: wgpu::Buffer,
    ground_render: wgpu::Buffer,
    ground_object: wgpu::Buffer,
    shadow_renders: Vec<wgpu::Buffer>,
    shadow_object: wgpu::Buffer,
    shadow_pipeline: wgpu::RenderPipeline,
    shadow_binds: Vec<[wgpu::BindGroup; 2]>,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    sampler: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 100.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        // The fixture's Math.random: the 1,024 time offsets at load, then each
        // instance's x jitter and hue.
        let mut seed = 186u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.
        };
        let offsets: Vec<f64> = (0..COUNT).map(|_| random() * 3.).collect();
        let (asset, buffers, _) = load_asset("/web/models/Horse/Horse.glb").await?;
        let primitive = asset
            .meshes()
            .next()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("horse mesh"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let positions: Vec<[f32; 3]> = reader
            .read_positions()
            .ok_or(Error::Invalid("horse positions"))?
            .collect();
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("horse index"))?
            .into_u32()
            .collect();
        let targets: Vec<Vec<[f32; 3]>> = reader
            .read_morph_targets()
            .map(|(p, _, _)| p.map(|p| p.collect()).unwrap_or_default())
            .collect();
        if targets.len() != TARGETS {
            return Err(Error::Invalid("horse morph targets"));
        }
        let animation = asset
            .animations()
            .next()
            .ok_or(Error::Invalid("horse clip"))?;
        let channel = animation
            .channels()
            .next()
            .ok_or(Error::Invalid("horse track"))?;
        let track = channel.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let keys: Vec<f64> = track
            .read_inputs()
            .ok_or(Error::Invalid("horse keys"))?
            .map(|t| t as f64)
            .collect();
        let weights: Vec<f32> = match track.read_outputs() {
            Some(gltf::animation::util::ReadOutputs::MorphTargetWeights(w)) => {
                w.into_f32().collect()
            }
            _ => return Err(Error::Invalid("horse weights")),
        };
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        // The morph targets as a 796 × 1 × 15 RGBA32F array (MorphNode's layout).
        let width = positions.len() as u32;
        let mut target_data = vec![0f32; width as usize * 4 * TARGETS];
        for (layer, target) in targets.iter().enumerate() {
            for (v, p) in target.iter().enumerate() {
                let at = (layer * width as usize + v) * 4;
                target_data[at..at + 3].copy_from_slice(p);
            }
        }
        let morph_targets = r.device.create_texture_with_data(
            &r.queue,
            &wgpu::TextureDescriptor {
                label: Some("horse morph targets"),
                size: wgpu::Extent3d {
                    width,
                    height: 1,
                    depth_or_array_layers: TARGETS as u32,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba32Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            bytemuck::cast_slice(&target_data),
        );
        let targets_view = morph_targets.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let influences = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("horse influences"),
            size: wgpu::Extent3d {
                width: TARGETS as u32 + 1,
                height: COUNT as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        // The 32 × 32 herd: positions jittered in x, hsl( random × 360, 50%, 66% ).
        let offset = 5000.;
        let (mut matrices, mut colors) = (vec![], vec![]);
        for x in 0..32 {
            for y in 0..32 {
                let px = offset - 300. * x as f64 + 200. * random();
                let pz = offset - 300. * y as f64;
                matrices.extend(
                    Matrix4::from_translation(Vector3::new(px, 0., pz))
                        .to_cols_array()
                        .map(|v| v as f32),
                );
                // Color.setStyle( 'hsl(…)' ) reads the components as sRGB.
                let hsl = Color::from_hsl(random() * 360. / 360., 0.5, 0.66).0;
                let color = Color::from_srgb(hsl.x, hsl.y, hsl.z).0;
                colors.extend([color.x as f32, color.y as f32, color.z as f32]);
            }
        }
        let vertex = wgpu::BufferUsages::VERTEX;
        let ground = PlaneGeometry::build(1_000_000., 1_000_000., 1, 1)?;
        let f = |name: &str| -> Result<Vec<f32>> {
            let a = ground
                .attributes
                .get(name)
                .ok_or(Error::Invalid("ground attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let ground_index = ground.index.clone().ok_or(Error::Invalid("ground index"))?;
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let horse_buffers = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let layouts = [
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &horse_buffers[0],
            },
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &horse_buffers[1],
            },
        ];
        let shadow_pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("horse shadow"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &module("horse shadow", wgsl!("shadow_vs")),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    buffers: &layouts,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module("horse shadow", wgsl!("shadow_fs")),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
                }),
                // As the page's shadow pipeline: clockwise front faces.
                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: Some(wgpu::Face::Back),
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
            });
        let instances = init(
            "horse instances",
            bytemuck::cast_slice(&matrices),
            wgpu::BufferUsages::UNIFORM,
        );
        let shadow_object = uniform(r, "horse shadow object", 80);
        let shadow_renders: Vec<wgpu::Buffer> = (0..CASCADES)
            .map(|_| uniform(r, "sun cascade camera", 128))
            .collect();
        let influences_view = influences.create_view(&Default::default());
        let shadow_binds = shadow_renders
            .iter()
            .map(|render| {
                [
                    bind(
                        r,
                        shadow_pipeline.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(
                        r,
                        shadow_pipeline.get_bind_group_layout(1),
                        &[
                            (0, shadow_object.as_entire_binding()),
                            (1, wgpu::BindingResource::TextureView(&influences_view)),
                            (2, wgpu::BindingResource::TextureView(&targets_view)),
                            (3, instances.as_entire_binding()),
                        ],
                    ),
                ]
            })
            .collect();
        let atlas = (TILE * CASCADES as u32, TILE);
        Ok(Self {
            time: 0.,
            offsets,
            keys,
            weights,
            positions: init("horse positions", bytemuck::cast_slice(&positions), vertex),
            colors: init("horse colors", bytemuck::cast_slice(&colors), vertex),
            index: init(
                "horse index",
                bytemuck::cast_slice(&index),
                wgpu::BufferUsages::INDEX,
            ),
            count: index.len() as u32,
            ground: (
                init(
                    "ground normals",
                    bytemuck::cast_slice(&f("normal")?),
                    vertex,
                ),
                init(
                    "ground positions",
                    bytemuck::cast_slice(&f("position")?),
                    vertex,
                ),
                init(
                    "ground index",
                    bytemuck::cast_slice(&ground_index),
                    wgpu::BufferUsages::INDEX,
                ),
            ),
            influences,
            targets_view,
            instances,
            horse_render: uniform(
                r,
                "horse render",
                pack(wgsl!("horse_fs"), "renderStruct", &[])?.len() as u64,
            ),
            horse_object: uniform(
                r,
                "horse object",
                pack(wgsl!("horse_fs"), "objectStruct", &[])?.len() as u64,
            ),
            ground_render: uniform(
                r,
                "ground render",
                pack(wgsl!("ground_fs"), "renderStruct", &[])?.len() as u64,
            ),
            ground_object: uniform(
                r,
                "ground object",
                pack(wgsl!("ground_fs"), "objectStruct", &[])?.len() as u64,
            ),
            shadow_renders,
            shadow_object,
            shadow_pipeline,
            shadow_binds,
            shadow_color: texture(r, atlas, wgpu::TextureFormat::Rgba8Unorm, 1, false),
            shadow_depth: texture(r, atlas, DEPTH, 1, true),
            output_render: uniform(r, "output render", 144),
            output_object: uniform(r, "output object", 64),
            quad: init(
                "quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            sampler: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            compare: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                compare: Some(wgpu::CompareFunction::LessEqual),
                ..Default::default()
            }),
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the camera circles and looks at the origin.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let (time, radius) = (self.time, 3000.);
        s.get_mut(c)?.position = Vector3::new(
            (time / 10.).sin() * radius,
            1500. + 1000. * (time / 5.).cos(),
            (time / 10.).cos() * radius,
        );
        s.look_at(c, Vector3::ZERO)
    }
    /// AnimationMixer.setTime( t ): the looping clip's weights at t, linearly
    /// interpolated in f32 as the track's interpolant does.
    fn weights_at(&self, t: f64) -> [f32; TARGETS] {
        let duration = *self.keys.last().unwrap_or(&1.);
        let t = t.rem_euclid(duration);
        let k = self
            .keys
            .partition_point(|&key| key <= t)
            .clamp(1, self.keys.len() - 1);
        let (t0, t1) = (self.keys[k - 1], self.keys[k]);
        let alpha = ((t - t0) / (t1 - t0)) as f32;
        std::array::from_fn(|i| {
            let (a, b) = (
                self.weights[(k - 1) * TARGETS + i],
                self.weights[k * TARGETS + i],
            );
            a * (1. - alpha) + b * alpha
        })
    }
    /// SunLightShadow.updateMatrices: practical splits, each cascade's bounding
    /// sphere in light space, snapped to its texels, in one atlas tile.
    fn cascades(
        &self,
        projection: Matrix4,
        world: Matrix4,
        near: f64,
        far: f64,
    ) -> [Cascade; CASCADES] {
        let size = TILE as f64;
        let inset = (2f64 / size).min(0.25);
        let resolution = size * (1. - 2. * inset);
        let camera_far = (near + 1e-6).max(far.min(10000.));
        let mut splits = [near, 0., camera_far];
        let amount = 0.5;
        let uniform_split = near + (camera_far - near) * amount;
        let log_split = near * (camera_far / near).powf(amount);
        splits[1] = (uniform_split + log_split) * 0.5;
        let light = Vector3::new(200., 1000., 50.);
        let direction = -light.normalize();
        let up = if Vector3::Y.dot(direction).abs() > 0.99 {
            Vector3::Z
        } else {
            Vector3::Y
        };
        // Matrix4.lookAt( origin, direction, up ).
        let z = -direction;
        let x = up.cross(z).normalize();
        let orientation = Matrix4::from_cols(
            x.extend(0.),
            z.cross(x).extend(0.),
            z.extend(0.),
            glam::DVec4::W,
        );
        let to_light = orientation.transpose() * world;
        let inverse = projection.inverse();
        let mut near_corners = [Vector3::ZERO; 4];
        let mut far_corners = [Vector3::ZERO; 4];
        let mut max_z = f64::MIN;
        for i in 0..4 {
            let x = if i == 0 || i == 1 { 1. } else { -1. };
            let y = if i == 0 || i == 3 { 1. } else { -1. };
            let n = inverse.project_point3(Vector3::new(x, y, 0.));
            let f = n * (camera_far / near);
            near_corners[i] = to_light.transform_point3(n);
            far_corners[i] = to_light.transform_point3(f);
            max_z = max_z.max(near_corners[i].z).max(far_corners[i].z);
        }
        max_z += camera_far;
        let shadow_near = 0.5;
        let mut data = [[0.; 4]; CASCADES];
        std::array::from_fn(|i| {
            let cascade_near = if i == 0 { splits[0] } else { data[i - 1][2] };
            let cascade_far = splits[i + 1];
            let fade_start = cascade_far - 0.1 * (cascade_far - splits[i]);
            data[i] = [
                if i == 0 { -1e10 } else { cascade_near },
                cascade_far,
                fade_start,
                0.,
            ];
            let near_alpha = (cascade_near - near) / (camera_far - near);
            let far_alpha = (cascade_far - near) / (camera_far - near);
            let mut corners = [Vector3::ZERO; 8];
            let mut center = Vector3::ZERO;
            for j in 0..4 {
                corners[j * 2] = near_corners[j] + (far_corners[j] - near_corners[j]) * near_alpha;
                corners[j * 2 + 1] =
                    near_corners[j] + (far_corners[j] - near_corners[j]) * far_alpha;
                center += corners[j * 2] + corners[j * 2 + 1];
            }
            center *= 1. / 8.;
            let mut radius_sq: f64 = 0.;
            let mut min_z = f64::MAX;
            for corner in &corners {
                radius_sq = radius_sq.max(corner.distance_squared(center));
                min_z = min_z.min(corner.z);
            }
            let mut radius = radius_sq.sqrt();
            radius /= 1. - 1. / resolution;
            let texel = 2. * radius / resolution;
            center.x = (center.x / texel).round() * texel;
            center.y = (center.y / texel).round() * texel;
            center.z = max_z + shadow_near;
            let center = orientation.transform_point3(center);
            let camera_world = Matrix4::from_translation(center) * orientation;
            let projection = orthographic(radius, shadow_near, max_z - min_z + 2. * shadow_near);
            let view = camera_world.inverse();
            // LightShadow._updateMatrix with the cascade's atlas viewport.
            let viewport = [i as f64 + inset, inset, 1. - 2. * inset, 1. - 2. * inset];
            let (sx, sy) = (viewport[2] / CASCADES as f64, viewport[3]);
            let (ox, oy) = (viewport[0] / CASCADES as f64, viewport[1]);
            let bias = Matrix4::from_cols_array(&[
                0.5 * sx,
                0.,
                0.,
                0.,
                0.,
                0.5 * sy,
                0.,
                0.,
                0.,
                0.,
                1.,
                0.,
                0.5 * sx + ox,
                0.5 * sy + oy,
                0.,
                1.,
            ]);
            Cascade {
                projection,
                view,
                matrix: bias * projection * view,
                data: data[i],
                // Viewports are measured from the top of the atlas.
                viewport: [
                    (size * viewport[0]) as f32,
                    (size - size * (viewport[1] + viewport[3])) as f32,
                    (size * viewport[2]) as f32,
                    (size * viewport[3]) as f32,
                ],
            }
        })
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = out.options.samples.max(1);
        let color = texture(r, size, HALF, samples, samples == 1);
        let resolve = (samples > 1).then(|| texture(r, size, HALF, 1, true));
        let depth = texture(r, size, DEPTH, samples, false);
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let attributes = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let layout = |i: usize, step| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: step,
            attributes: &attributes[i],
        };
        let vertex = wgpu::VertexStepMode::Vertex;
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        format,
                        scene: bool| {
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &module(label, vs),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        buffers,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module(label, fs),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState::from(format))],
                    }),
                    primitive: wgpu::PrimitiveState {
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    depth_stencil: scene.then(|| wgpu::DepthStencilState {
                        format: DEPTH,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: if scene { samples } else { 1 },
                        ..Default::default()
                    },
                    multiview: None,
                    cache: None,
                })
        };
        let horse = pipeline(
            "horses",
            wgsl!("horse_vs"),
            wgsl!("horse_fs"),
            &[layout(0, vertex), layout(1, wgpu::VertexStepMode::Instance)],
            HALF,
            true,
        );
        let influences = self.influences.create_view(&Default::default());
        let tex = wgpu::BindingResource::TextureView;
        let horse_binds = [
            bind(
                r,
                horse.get_bind_group_layout(0),
                &[(0, self.horse_render.as_entire_binding())],
            ),
            bind(
                r,
                horse.get_bind_group_layout(1),
                &[
                    (0, self.horse_object.as_entire_binding()),
                    (1, wgpu::BindingResource::Sampler(&self.sampler)),
                    (2, tex(&r.dfg)),
                    (3, tex(&influences)),
                    (4, tex(&self.targets_view)),
                    (5, self.instances.as_entire_binding()),
                ],
            ),
        ];
        let ground = pipeline(
            "ground",
            wgsl!("ground_vs"),
            wgsl!("ground_fs"),
            &[layout(0, vertex), layout(1, vertex)],
            HALF,
            true,
        );
        let ground_binds = [
            bind(
                r,
                ground.get_bind_group_layout(0),
                &[(0, self.ground_render.as_entire_binding())],
            ),
            bind(
                r,
                ground.get_bind_group_layout(1),
                &[
                    (0, self.ground_object.as_entire_binding()),
                    (1, wgpu::BindingResource::Sampler(&self.sampler)),
                    (2, tex(&r.dfg)),
                    (3, wgpu::BindingResource::Sampler(&self.compare)),
                    (4, tex(&self.shadow_depth)),
                ],
            ),
        ];
        let output = pipeline(
            "render output",
            include_str!("lights_dynamic/output_vs.wgsl"),
            include_str!("lights_dynamic/output_fs.wgsl"),
            &[layout(0, vertex)],
            out.options.format,
            false,
        );
        let output_binds = [
            bind(
                r,
                output.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            bind(
                r,
                output.get_bind_group_layout(1),
                &[
                    (0, wgpu::BindingResource::Sampler(&self.sampler)),
                    (1, tex(resolve.as_ref().unwrap_or(&color))),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        let mut data: Vec<f32> = [Matrix4::IDENTITY; 2]
            .iter()
            .flat_map(|m| m.to_cols_array().map(|v| v as f32))
            .collect();
        data.extend([size.0 as f32, size.1 as f32, 0., 0.]);
        r.queue
            .write_buffer(&self.output_render, 0, bytemuck::cast_slice(&data));
        r.queue.write_buffer(
            &self.output_object,
            0,
            bytemuck::cast_slice(&Matrix4::IDENTITY.to_cols_array().map(|v| v as f32)),
        );
        let screen = RenderTarget::with_options(
            &r.device,
            size.0,
            size.1,
            RenderTargetOptions {
                samples: 0,
                depth_buffer: false,
                ..out.options.clone()
            },
        )?;
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            color,
            resolve,
            depth,
            screen,
            horse: (horse, horse_binds),
            ground: (ground, ground_binds),
            output: (output, output_binds),
        });
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != out.options.samples.max(1)
        }) {
            self.resize(r, out)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera else {
            return Err(Error::Invalid("morph camera"));
        };
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let cascades = self.cascades(projection, world, p.near, p.far);
        // setMorphAt for every instance: [ base influence, 15 weights ].
        let mut rows = vec![0f32; COUNT * (TARGETS + 1)];
        for (i, offset) in self.offsets.iter().enumerate() {
            let w = self.weights_at(self.time + offset);
            rows[i * (TARGETS + 1)] = 1.;
            rows[i * (TARGETS + 1) + 1..(i + 1) * (TARGETS + 1)].copy_from_slice(&w);
        }
        r.queue.write_texture(
            self.influences.as_image_copy(),
            bytemuck::cast_slice(&rows),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some((TARGETS as u32 + 1) * 4),
                rows_per_image: Some(COUNT as u32),
            },
            self.influences.size(),
        );
        let sky = Color::from_hex(0x99ddff).0.to_array().to_vec();
        let third = |hex: u32| Color::from_hex(hex).0.to_array().map(|v| v / 3.).to_vec();
        let sun = vec![200., 1000., 50.];
        let horse_values: Vec<(&str, Vec<f64>)> = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
            ("nodeUniform15", sun.clone()),
            ("nodeUniform16", vec![1.; 3]),
            ("nodeUniform17", third(0x669933)),
            ("nodeUniform18", third(0x99ddff)),
            ("nodeUniform19", vec![0., 1., 0.]),
            ("nodeUniform20", sky.clone()),
            ("nodeUniform21", vec![5000.]),
            ("nodeUniform22", vec![10000.]),
        ];
        let write = |buffer: &wgpu::Buffer,
                     source: &str,
                     name: &str,
                     values: &[(&str, Vec<f64>)]|
         -> Result<()> {
            let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            r.queue
                .write_buffer(buffer, 0, &pack(source, name, &values)?);
            Ok(())
        };
        write(
            &self.horse_render,
            wgsl!("horse_fs"),
            "renderStruct",
            &horse_values,
        )?;
        let white = vec![1.; 3];
        write(
            &self.horse_object,
            wgsl!("horse_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.]),
                ("nodeUniform5", white.clone()),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform7", vec![0.]),
                ("nodeUniform8", vec![1.]),
                ("nodeUniform10", vec![1.]),
                ("nodeUniform13", m4(Matrix4::IDENTITY)),
            ],
        )?;
        write(
            &self.shadow_object,
            wgsl!("shadow_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.]),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform8", m4(Matrix4::IDENTITY)),
            ],
        )?;
        // SunShadowNode walks the cascades back to front: the first uniforms
        // are the far cascade's.
        let ground_values: Vec<(&str, Vec<f64>)> = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
            ("nodeUniform11", sun),
            ("nodeUniform12", vec![1.; 3]),
            ("nodeUniform13", vec![0.]),
            ("nodeUniform14", cascades[1].data.to_vec()),
            ("nodeUniform15", m4(cascades[1].matrix)),
            ("nodeUniform16", vec![0.]),
            ("nodeUniform18", vec![1.]),
            ("nodeUniform19", vec![(TILE * 2) as f64, TILE as f64]),
            ("nodeUniform20", cascades[0].data.to_vec()),
            ("nodeUniform21", m4(cascades[0].matrix)),
            ("nodeUniform22", vec![0.]),
            ("nodeUniform23", vec![1.]),
            ("nodeUniform24", vec![(TILE * 2) as f64, TILE as f64]),
            ("nodeUniform25", vec![1.]),
            ("nodeUniform26", third(0x669933)),
            ("nodeUniform27", third(0x99ddff)),
            ("nodeUniform28", vec![0., 1., 0.]),
            ("nodeUniform29", sky),
            ("nodeUniform30", vec![5000.]),
            ("nodeUniform31", vec![10000.]),
        ];
        write(
            &self.ground_render,
            wgsl!("ground_fs"),
            "renderStruct",
            &ground_values,
        )?;
        let ground_model = Matrix4::from_rotation_x(-PI / 2.);
        write(
            &self.ground_object,
            wgsl!("ground_fs"),
            "objectStruct",
            &[
                (
                    "nodeUniform0",
                    Color::from_hex(0x669933).0.to_array().to_vec(),
                ),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", m3(ground_model.inverse().transpose())),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(ground_model)),
            ],
        )?;
        for (buffer, cascade) in self.shadow_renders.iter().zip(&cascades) {
            let mut data: Vec<f32> = cascade
                .projection
                .to_cols_array()
                .map(|v| v as f32)
                .to_vec();
            data.extend(cascade.view.to_cols_array().map(|v| v as f32));
            r.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&data));
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("morph targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // The atlas is cleared once; each cascade draws into its tile.
        for (i, cascade) in cascades.iter().enumerate() {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sun shadow"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.shadow_color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if i == 0 {
                            wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.shadow_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: if i == 0 {
                            wgpu::LoadOp::Clear(1.)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            let [x, y, w, h] = cascade.viewport;
            pass.set_viewport(x, y, w, h, 0., 1.);
            pass.set_pipeline(&self.shadow_pipeline);
            pass.set_bind_group(0, &self.shadow_binds[i][0], &[]);
            pass.set_bind_group(1, &self.shadow_binds[i][1], &[]);
            pass.set_vertex_buffer(0, self.positions.slice(..));
            pass.set_vertex_buffer(1, self.colors.slice(..));
            pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.count, 0, 0..COUNT as u32);
        }
        {
            let background = Color::from_hex(0x99ddff).0;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("morph scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: t.resolve.as_ref(),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background.x,
                            g: background.y,
                            b: background.z,
                            a: 1.,
                        }),
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
            pass.set_pipeline(&t.horse.0);
            pass.set_bind_group(0, &t.horse.1[0], &[]);
            pass.set_bind_group(1, &t.horse.1[1], &[]);
            pass.set_vertex_buffer(0, self.positions.slice(..));
            pass.set_vertex_buffer(1, self.colors.slice(..));
            pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.count, 0, 0..COUNT as u32);
            pass.set_pipeline(&t.ground.0);
            pass.set_bind_group(0, &t.ground.1[0], &[]);
            pass.set_bind_group(1, &t.ground.1[1], &[]);
            pass.set_vertex_buffer(0, self.ground.0.slice(..));
            pass.set_vertex_buffer(1, self.ground.1.slice(..));
            pass.set_index_buffer(self.ground.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..6, 0, 0..1);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("morph output"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&t.output.0);
            pass.set_bind_group(0, &t.output.1[0], &[]);
            pass.set_bind_group(1, &t.output.1[1], &[]);
            pass.set_vertex_buffer(0, self.quad.slice(..));
            pass.draw(0..3, 0..1);
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
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("morph parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
