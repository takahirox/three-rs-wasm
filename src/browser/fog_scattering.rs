//! webgpu_custom_fog_scattering: a stand of bare TreeGenerator trees in
//! FogExp2, with the fog's scattering approximated in post: the scene pass is
//! blurred at half resolution ( GaussianBlurNode, sigma 4, direction scaled by
//! the scattering factor ) and mixed over the sharp color by the density fog
//! factor of the pass's view depth. Ten tree variants are grown on the CPU
//! once, as the page does, and drawn as instanced meshes with their seeded
//! placements, plus two hero trunks, over a black ground. FirstPersonControls
//! move the camera. Every stage runs the WGSL three.js r186 generates for the
//! page (in `fog_scattering/`; the instanced vertex module is the page's with
//! each mesh's instance count, and the plain output is the volume_caustics
//! and ssgi modules, byte-identical).
mod tree;
use super::deferred::{Draw, sampled_pipeline, set};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::uniform;
use super::trackball_sprites::FirstPerson;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("fog_scattering/", $name, ".wgsl"))
    };
}
const PLAIN_VS: &str = include_str!("volume_caustics/high_vs.wgsl");
const PLAIN_FS: &str = include_str!("ssgi/output_direct_fs.wgsl");
/// The instanced vertex module's matrix array, sized by the mesh's count.
const INSTANCES: &str = "array< mat4x4<f32>, 18 >";
const FOG: u32 = 0xc6cace;
fn linear(hex: u32, intensity: f64) -> Vec<f64> {
    [16, 8, 0]
        .map(|shift| {
            let c = ((hex >> shift) & 255) as f64 / 255.;
            let l = if c < 0.04045 {
                c * 0.0773993808
            } else {
                (c * 0.9478672986 + 0.0521327014).powf(2.4)
            };
            l * intensity
        })
        .to_vec()
}
/// Euler XYZ → Quaternion → Matrix4.compose, in three.js's order.
fn compose(position: [f64; 3], rotation: [f64; 3], scale: [f64; 3]) -> Matrix4 {
    let [rx, ry, rz] = rotation.map(|a| a / 2.);
    let (c1, c2, c3) = (rx.cos(), ry.cos(), rz.cos());
    let (s1, s2, s3) = (rx.sin(), ry.sin(), rz.sin());
    let x = s1 * c2 * c3 + c1 * s2 * s3;
    let y = c1 * s2 * c3 - s1 * c2 * s3;
    let z = c1 * c2 * s3 + s1 * s2 * c3;
    let w = c1 * c2 * c3 - s1 * s2 * s3;
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    let [sx, sy, sz] = scale;
    Matrix4::from_cols_array(&[
        (1. - (yy + zz)) * sx,
        (xy + wz) * sx,
        (xz - wy) * sx,
        0.,
        (xy - wz) * sy,
        (1. - (xx + zz)) * sy,
        (yz + wx) * sy,
        0.,
        (xz + wy) * sz,
        (yz - wx) * sz,
        (1. - (xx + yy)) * sz,
        0.,
        position[0],
        position[1],
        position[2],
        1.,
    ])
}
/// MathUtils.seededRandom ( Mulberry32 over a shared seed ).
struct Seeded(u32);
impl Seeded {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6D2B79F5);
        let mut t = self.0;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        (t ^ (t >> 14)) as f64 / 4294967296.
    }
}
/// computeBoundingSphere: the box center and the farthest vertex.
fn bounds(positions: &[f32]) -> Sphere {
    let points: Vec<Vector3> = positions
        .chunks(3)
        .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
        .collect();
    let (lo, hi) = points.iter().fold(
        (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        ),
        |(lo, hi), p| (lo.min(*p), hi.max(*p)),
    );
    let center = (lo + hi) * 0.5;
    Sphere {
        center,
        radius: points
            .iter()
            .map(|p| p.distance_squared(center))
            .fold(0., f64::max)
            .sqrt(),
    }
}
/// Sphere.applyMatrix4.
fn transform(sphere: Sphere, m: Matrix4) -> Sphere {
    let (scale, _, _) = m.to_scale_rotation_translation();
    Sphere {
        center: m.transform_point3(sphere.center),
        radius: sphere.radius * scale.abs().max_element(),
    }
}
/// Sphere.union ( from an empty sphere ).
fn union(a: Option<Sphere>, b: Sphere) -> Sphere {
    let Some(mut a) = a else {
        return b;
    };
    if a.center == b.center {
        a.radius = a.radius.max(b.radius);
        return a;
    }
    let offset = (b.center - a.center).normalize_or_zero() * b.radius;
    for point in [b.center + offset, b.center - offset] {
        let v = point - a.center;
        let length_sq = v.length_squared();
        if length_sq > a.radius * a.radius {
            let length = length_sq.sqrt();
            let delta = (length - a.radius) * 0.5;
            a.center += v * (delta / length);
            a.radius += delta;
        }
    }
    a
}
/// One tree variant: its buffers and local bounding sphere.
struct Variant {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    sphere: Sphere,
}
/// A draw of a variant: a hero trunk ( one model ) or an instanced mesh.
struct Item {
    variant: usize,
    instances: u32,
    /// The world bounding sphere for culling and sorting.
    sphere: Sphere,
    pipeline: wgpu::RenderPipeline,
    object: wgpu::Buffer,
    matrices: Option<wgpu::Buffer>,
    hero: Option<Matrix4>,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<wgpu::TextureView>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    blur: [wgpu::TextureView; 2],
    blur_size: (u32, u32),
    items: Vec<Draw>,
    ground: Draw,
    blur_draws: [Draw; 2],
    outputs: [Draw; 2],
    screen: RenderTarget,
}
pub(super) struct Demo {
    walker: FirstPerson,
    /// fog density, scattering factor, scattering enabled.
    params: [f64; 3],
    time: f64,
    last: f64,
    pending: bool,
    variants: Vec<Variant>,
    items: Vec<Item>,
    ground: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    hero_render: wgpu::Buffer,
    tree_render: wgpu::Buffer,
    ground_render: wgpu::Buffer,
    ground_object: wgpu::Buffer,
    blur_objects: [wgpu::Buffer; 2],
    output_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    clamp: wgpu::Sampler,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 55.,
            near: 0.1,
            far: far(0.11),
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0.4, 1.7, 9.);
        // controls.lookAt( - 0.2, 1.7, - 8 ): the camera looks there and
        // _setOrientation() takes its latitude and longitude.
        s.look_at(c, Vector3::new(-0.2, 1.7, -8.))?;
        let mut walker = FirstPerson::default();
        walker.speed = 2.;
        let look = s.get(c)?.quaternion * -Vector3::Z;
        walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
        walker.lon = look.x.atan2(look.z).to_degrees();
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let index_usage = wgpu::BufferUsages::INDEX;
        // The ten variants of the page's generator settings.
        let base = tree::Params {
            seed: 1,
            levels: 5,
            children: vec![3, 12, 8],
            branch_angle: vec![62., 46., 38., 32.],
            angle_variance: 24.,
            length_ratio: 0.62,
            length_variance: 0.3,
            branch_length_falloff: 0.45,
            trunk_length: 9.,
            trunk_radius: 0.42,
            taper: 0.94,
            taper_curve: 0.85,
            root_flare: 0.5,
            flare_frac: 0.12,
            radius_exponent: 2.,
            min_radius: 0.0007,
            min_length: 0.04,
            droop: 0.08,
            up_pull: 0.18,
            gnarl: vec![0.045, 0.13, 0.2, 0.26, 0.3],
            radial_segments: 7,
            section_length: 0.26,
            child_start: 0.22,
            trunk_clear: 0.25,
        };
        let variants: Vec<Variant> = (0..10)
            .map(|v| {
                let p = tree::Params {
                    seed: v + 1,
                    trunk_length: 3.4 + v as f64 * 0.16,
                    trunk_radius: 0.065 + v as f64 * 0.004,
                    children: vec![7 + v as usize % 3, 4, 3, 1],
                    trunk_clear: 0.38 + (v % 4) as f64 * 0.05,
                    ..base.clone()
                };
                let g = tree::build(&p);
                Variant {
                    positions: init("tree positions", bytemuck::cast_slice(&g.positions), vertex),
                    normals: init("tree normals", bytemuck::cast_slice(&g.normals), vertex),
                    index: init("tree index", bytemuck::cast_slice(&g.index), index_usage),
                    count: g.index.len() as u32,
                    sphere: bounds(&g.positions),
                }
            })
            .collect();
        // The seeded placements of the 13 × 12 grid.
        let mut random = Seeded(25);
        random.next();
        let mut placements: Vec<Vec<Matrix4>> = vec![vec![]; variants.len()];
        let (cols, rows, spacing) = (13, 12, 1.9);
        for i in 0..cols {
            for j in 0..rows {
                let v = (random.next() * variants.len() as f64).floor() as usize;
                let x =
                    (i as f64 - cols as f64 / 2.) * spacing + (random.next() - 0.5) * spacing * 1.4;
                let z = j as f64 * spacing - rows as f64 * spacing
                    + 4.2
                    + (random.next() - 0.5) * spacing * 1.4;
                let scale = 0.7 + random.next() * 0.65;
                let rotation = [
                    (random.next() - 0.5) * 0.08,
                    random.next() * std::f64::consts::PI * 2.,
                    (random.next() - 0.5) * 0.08,
                ];
                let sy = scale * (0.85 + random.next() * 0.3);
                placements[v].push(compose([x, 0., z], rotation, [scale, sy, scale]));
            }
        }
        let attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs[i],
        });
        let lit = |vs: &str, fs: &str, samples| {
            sampled_pipeline(
                r,
                "fog scattering tree",
                (vs, fs),
                &layouts,
                &[HALF],
                Some((wgpu::CompareFunction::LessEqual, true)),
                (false, false),
                (samples, wgpu::PrimitiveTopology::TriangleList),
            )
        };
        let samples = 4;
        let mut items = vec![];
        // The two hero trunks: ( x, z, scale, rotation.y, variant ).
        for (x, z, scale, ry, v) in [(-1.1, 4.9, 1.25, 1.1, 8), (1.5, 4., 1.1, 0.3, 5)] {
            let model = compose([x, 0., z], [0., ry, 0.], [scale; 3]);
            items.push(Item {
                variant: v,
                instances: 1,
                sphere: transform(variants[v].sphere, model),
                pipeline: lit(wgsl!("hero_vs"), wgsl!("hero_fs"), samples),
                object: uniform(r, "fog scattering hero", wgsl!("hero_fs"), "objectStruct")?,
                matrices: None,
                hero: Some(model),
            });
        }
        for (v, list) in placements.iter().enumerate() {
            if list.is_empty() {
                continue;
            }
            let vs = wgsl!("tree_vs")
                .replace(INSTANCES, &format!("array< mat4x4<f32>, {} >", list.len()));
            let data: Vec<f32> = list
                .iter()
                .flat_map(|m| m.to_cols_array().map(|v| v as f32))
                .collect();
            let sphere = list
                .iter()
                .fold(None, |acc, m| {
                    Some(union(acc, transform(variants[v].sphere, *m)))
                })
                .ok_or(Error::Invalid("tree instances"))?;
            items.push(Item {
                variant: v,
                instances: list.len() as u32,
                sphere,
                pipeline: lit(&vs, wgsl!("tree_fs"), samples),
                object: uniform(r, "fog scattering trees", wgsl!("tree_fs"), "objectStruct")?,
                matrices: Some(init(
                    "fog scattering instances",
                    bytemuck::cast_slice(&data),
                    wgpu::BufferUsages::UNIFORM,
                )),
                hero: None,
            });
        }
        // PlaneGeometry( 600, 600 ).rotateX( - π / 2 ): normal and position.
        let mut plane = PlaneGeometry::build(600., 600., 1, 1)?;
        plane.apply_matrix4(Matrix4::from_rotation_x(-std::f64::consts::FRAC_PI_2))?;
        let read = |name: &str| -> Result<Vec<f32>> {
            let a = plane
                .attributes
                .get(name)
                .ok_or(Error::Invalid("ground attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let plane_index = plane.index.clone().ok_or(Error::Invalid("ground index"))?;
        let ground = (
            init(
                "ground normals",
                bytemuck::cast_slice(&read("normal")?),
                vertex,
            ),
            init(
                "ground positions",
                bytemuck::cast_slice(&read("position")?),
                vertex,
            ),
            init(
                "ground index",
                bytemuck::cast_slice(&plane_index),
                index_usage,
            ),
            plane_index.len() as u32,
        );
        Ok(Self {
            walker,
            params: [0.11, 2., 1.],
            time: 0.,
            last: 0.,
            pending: true,
            variants,
            items,
            ground,
            hero_render: uniform(r, "fog scattering hero", wgsl!("hero_fs"), "renderStruct")?,
            tree_render: uniform(r, "fog scattering trees", wgsl!("tree_fs"), "renderStruct")?,
            ground_render: uniform(
                r,
                "fog scattering ground",
                wgsl!("ground_vs"),
                "renderStruct",
            )?,
            ground_object: uniform(
                r,
                "fog scattering ground",
                wgsl!("ground_vs"),
                "objectStruct",
            )?,
            blur_objects: [
                uniform(r, "fog scattering blur", wgsl!("blur_h_fs"), "objectStruct")?,
                uniform(r, "fog scattering blur", wgsl!("blur_v_fs"), "objectStruct")?,
            ],
            output_object: uniform(
                r,
                "fog scattering output",
                wgsl!("output_fs"),
                "objectStruct",
            )?,
            quad_uv: init(
                "fog scattering quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            clamp: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget, samples: u32) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let texture = |size: (u32, u32), format, samples| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("fog scattering target"),
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
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let color = texture((width, height), HALF, 1);
        let msaa = (samples > 1).then(|| texture((width, height), HALF, samples));
        let depth = texture((width, height), DEPTH, samples);
        // GaussianBlurNode at resolutionScale 0.5.
        let blur_size = (
            ((width as f64) * 0.5).round().max(1.) as u32,
            ((height as f64) * 0.5).round().max(1.) as u32,
        );
        let blur = [texture(blur_size, HALF, 1), texture(blur_size, HALF, 1)];
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let items = self
            .items
            .iter()
            .map(|item| {
                // The pipelines were built for 4× MSAA; rebuild for one sample.
                let pipeline = if samples == 4 {
                    item.pipeline.clone()
                } else {
                    let attrs = [
                        wgpu::vertex_attr_array![0 => Float32x3],
                        wgpu::vertex_attr_array![1 => Float32x3],
                    ];
                    let layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &attrs[i],
                    });
                    let vs = if item.hero.is_some() {
                        wgsl!("hero_vs").to_string()
                    } else {
                        wgsl!("tree_vs").replace(
                            INSTANCES,
                            &format!("array< mat4x4<f32>, {} >", item.instances),
                        )
                    };
                    let fs = if item.hero.is_some() {
                        wgsl!("hero_fs")
                    } else {
                        wgsl!("tree_fs")
                    };
                    sampled_pipeline(
                        r,
                        "fog scattering tree",
                        (&vs, fs),
                        &layouts,
                        &[HALF],
                        Some((wgpu::CompareFunction::LessEqual, true)),
                        (false, false),
                        (samples, wgpu::PrimitiveTopology::TriangleList),
                    )
                };
                let render = if item.hero.is_some() {
                    &self.hero_render
                } else {
                    &self.tree_render
                };
                let mut entries = vec![
                    (0, item.object.as_entire_binding()),
                    (1, sampler(&self.clamp)),
                    (2, tex(&r.dfg)),
                ];
                if let Some(m) = &item.matrices {
                    entries.push((3, m.as_entire_binding()));
                }
                (
                    pipeline.clone(),
                    vec![
                        bind(
                            r,
                            pipeline.get_bind_group_layout(0),
                            &[(0, render.as_entire_binding())],
                        ),
                        bind(r, pipeline.get_bind_group_layout(1), &entries),
                    ],
                )
            })
            .collect();
        let ground_attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let ground_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ground_attrs[i],
        });
        let ground_pipeline = sampled_pipeline(
            r,
            "fog scattering ground",
            (wgsl!("ground_vs"), wgsl!("ground_fs")),
            &ground_layouts,
            &[HALF],
            Some((wgpu::CompareFunction::LessEqual, true)),
            (false, false),
            (samples, wgpu::PrimitiveTopology::TriangleList),
        );
        let ground = (
            ground_pipeline.clone(),
            vec![
                bind(
                    r,
                    ground_pipeline.get_bind_group_layout(0),
                    &[(0, self.ground_render.as_entire_binding())],
                ),
                bind(
                    r,
                    ground_pipeline.get_bind_group_layout(1),
                    &[(0, self.ground_object.as_entire_binding())],
                ),
            ],
        );
        let quad_attrs = [wgpu::vertex_attr_array![0 => Float32x2]];
        let quad = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &quad_attrs[0],
        }];
        let screen = |label, shaders, format| {
            sampled_pipeline(
                r,
                label,
                shaders,
                &quad,
                &[format],
                None,
                (false, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
            )
        };
        let single = |p: &wgpu::RenderPipeline, entries: &[(u32, wgpu::BindingResource)]| {
            (
                p.clone(),
                vec![bind(r, p.get_bind_group_layout(0), entries)],
            )
        };
        let blur_h = screen(
            "fog scattering blur",
            (wgsl!("blur_vs"), wgsl!("blur_h_fs")),
            HALF,
        );
        let blur_v = screen(
            "fog scattering blur",
            (wgsl!("blur_vs"), wgsl!("blur_v_fs")),
            HALF,
        );
        let blur_draws = [
            single(
                &blur_h,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&color)),
                    (2, self.blur_objects[0].as_entire_binding()),
                ],
            ),
            single(
                &blur_v,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&blur[0])),
                    (2, self.blur_objects[1].as_entire_binding()),
                ],
            ),
        ];
        let format = out.options.format;
        let output_fs = if samples > 1 {
            wgsl!("output_ms_fs")
        } else {
            wgsl!("output_fs")
        };
        let composite = screen(
            "fog scattering output",
            (wgsl!("output_vs"), output_fs),
            format,
        );
        let plain = screen("fog scattering output", (PLAIN_VS, PLAIN_FS), format);
        let outputs = [
            single(
                &composite,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&color)),
                    (2, sampler(&self.clamp)),
                    (3, tex(&blur[1])),
                    (4, self.output_object.as_entire_binding()),
                    (5, tex(&depth)),
                ],
            ),
            single(&plain, &[(0, sampler(&self.clamp)), (1, tex(&color))]),
        ];
        let screen = RenderTarget::with_options(
            &r.device,
            width,
            height,
            RenderTargetOptions {
                samples: 0,
                depth_buffer: false,
                ..out.options.clone()
            },
        )?;
        self.targets = Some(Targets {
            width,
            height,
            format,
            samples,
            msaa,
            color,
            depth,
            blur,
            blur_size,
            items,
            ground,
            blur_draws,
            outputs,
            screen,
        });
        Ok(())
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("fog scattering output"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &t.screen.view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        set(&mut pass, &t.outputs[usize::from(self.params[2] < 0.5)]);
        pass.set_vertex_buffer(0, self.quad_uv.slice(..));
        pass.draw(0..3, 0..1);
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != samples
        }) {
            self.resize(r, out, samples)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("fog scattering targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        let [density, scattering, _] = self.params;
        // updateFogRange(): the far plane where FogExp2 leaves 0.1 %.
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.far = far(density);
        }
        // animate(): controls.update( timer.getDelta() ).
        let delta = (self.time - self.last).max(0.);
        self.last = self.time;
        self.walker.update(s, c, delta)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
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
        let fog = linear(FOG, 1.);
        let (sky, ground_color) = (linear(0xdce6ed, 1.5), linear(0x292521, 1.5));
        let up = vec![0., 1., 0.];
        let sun = linear(0xdce6ed, 2.);
        let camera_values = [
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
        ];
        // The lit trees: the hemisphere light ( sky, direction, ground ), the
        // directional light's color, position and target, and the fog.
        let mut hero_values = camera_values.to_vec();
        hero_values.extend([
            ("nodeUniform8", sky.clone()),
            ("nodeUniform10", up.clone()),
            ("nodeUniform7", ground_color.clone()),
            ("nodeUniform13", sun.clone()),
            ("nodeUniform11", vec![-3., 8., 5.]),
            ("nodeUniform12", vec![0.; 3]),
            ("nodeUniform14", fog.clone()),
            ("nodeUniform15", vec![density]),
        ]);
        write(
            &self.hero_render,
            wgsl!("hero_fs"),
            "renderStruct",
            &hero_values,
        )?;
        let mut tree_values = camera_values.to_vec();
        tree_values.extend([
            ("nodeUniform9", sky.clone()),
            ("nodeUniform11", up.clone()),
            ("nodeUniform8", ground_color.clone()),
            ("nodeUniform14", sun),
            ("nodeUniform12", vec![-3., 8., 5.]),
            ("nodeUniform13", vec![0.; 3]),
            ("nodeUniform15", fog.clone()),
            ("nodeUniform16", vec![density]),
        ]);
        write(
            &self.tree_render,
            wgsl!("tree_fs"),
            "renderStruct",
            &tree_values,
        )?;
        let mut ground_values = camera_values.to_vec();
        ground_values.extend([
            ("nodeUniform3", sky),
            ("nodeUniform7", up),
            ("nodeUniform2", ground_color),
            ("nodeUniform8", fog),
            ("nodeUniform9", vec![density]),
        ]);
        write(
            &self.ground_render,
            wgsl!("ground_vs"),
            "renderStruct",
            &ground_values,
        )?;
        write(
            &self.ground_object,
            wgsl!("ground_vs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![0.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform5", m3(Matrix4::IDENTITY)),
                ("nodeUniform10", m4(Matrix4::IDENTITY)),
            ],
        )?;
        for item in &self.items {
            match item.hero {
                Some(model) => write(
                    &item.object,
                    wgsl!("hero_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform0", vec![1.]),
                        ("nodeUniform2", m3(model.inverse().transpose())),
                        ("nodeUniform3", vec![0.; 3]),
                        ("nodeUniform4", vec![1.]),
                        ("nodeUniform6", m4(model)),
                    ],
                )?,
                None => write(
                    &item.object,
                    wgsl!("tree_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform3", m3(Matrix4::IDENTITY)),
                        ("nodeUniform4", vec![0.; 3]),
                        ("nodeUniform5", vec![1.]),
                        ("nodeUniform7", m4(Matrix4::IDENTITY)),
                    ],
                )?,
            }
        }
        let texel = vec![1. / t.blur_size.0 as f64, 1. / t.blur_size.1 as f64];
        for (b, fs) in self
            .blur_objects
            .iter()
            .zip([wgsl!("blur_h_fs"), wgsl!("blur_v_fs")])
        {
            write(
                b,
                fs,
                "objectStruct",
                &[
                    ("nodeUniform1", vec![scattering]),
                    ("nodeUniform2", texel.clone()),
                ],
            )?;
        }
        write(
            &self.output_object,
            wgsl!("output_fs"),
            "objectStruct",
            &[
                ("nodeUniform2", vec![density]),
                ("nodeUniform3", vec![0.1]),
                ("nodeUniform4", vec![far(density)]),
            ],
        )?;
        let view_projection = projection * view;
        let frustum = Frustum::from_projection(view_projection);
        // Opaque objects front to back by their bounding sphere center's
        // clip depth; the ground ( its sphere at the origin ) among them.
        let mut order: Vec<(f64, Option<usize>)> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| frustum.intersects_sphere(item.sphere))
            .map(|(i, item)| {
                let z = (view_projection * item.sphere.center.extend(1.)).z;
                (z, Some(i))
            })
            .collect();
        order.push(((view_projection * Vector4::new(0., 0., 0., 1.)).z, None));
        order.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let [cr, cg, cb] = [0, 1, 2].map(|i| linear(FOG, 1.)[i]);
            let clear = wgpu::Color {
                r: cr,
                g: cg,
                b: cb,
                a: 1.,
            };
            let (view_target, resolve) = match &t.msaa {
                Some(msaa) => (msaa, Some(&t.color)),
                None => (&t.color, None),
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fog scattering scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: view_target,
                    depth_slice: None,
                    resolve_target: resolve,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for (_, item) in order {
                match item {
                    Some(i) => {
                        let item = &self.items[i];
                        let v = &self.variants[item.variant];
                        set(&mut pass, &t.items[i]);
                        pass.set_vertex_buffer(0, v.positions.slice(..));
                        pass.set_vertex_buffer(1, v.normals.slice(..));
                        pass.set_index_buffer(v.index.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..v.count, 0, 0..item.instances);
                    }
                    None => {
                        set(&mut pass, &t.ground);
                        pass.set_vertex_buffer(0, self.ground.0.slice(..));
                        pass.set_vertex_buffer(1, self.ground.1.slice(..));
                        pass.set_index_buffer(self.ground.2.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..self.ground.3, 0, 0..1);
                    }
                }
            }
        }
        if self.params[2] > 0.5 {
            for (target, draw) in t.blur.iter().zip(&t.blur_draws) {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("fog scattering blur"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: target,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                set(&mut pass, draw);
                pass.set_vertex_buffer(0, self.quad_uv.slice(..));
                pass.draw(0..3, 0..1);
            }
        }
        self.present(&mut encoder, t);
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// FirstPersonControls' pointer: down ( 10 + button ), move, up ( 20 + button ).
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.walker.pointer(kind, x, y);
        self.pending = true;
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.walker.key(code, down);
    }
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
    /// fog density, scattering factor and scattering enabled.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("fog scattering parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
/// Math.min( 120, sqrt( − ln 0.001 ) / density ).
fn far(density: f64) -> f64 {
    120f64.min((-(0.001f64).ln()).sqrt() / density)
}
