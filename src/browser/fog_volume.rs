//! webgpu_postprocessing_fog: the Lucy statue ( PLYLoader, computeVertexNormals )
//! over a ground plane, lit by a hemisphere light and the SunLight addon's two
//! fitted shadow cascades, with volumetric cloud fog ray-marched at reduced
//! resolution through a precomputed 96³ tri-noise texture and upsampled by a
//! joint bilateral filter ( or a Gaussian blur, or none ). The cascades are
//! fitted to the view frustum on the CPU each frame as SunLightShadow does;
//! the noise texture is generated once, as the page does. Every stage runs
//! the WGSL three.js r186 generates for the page (in `fog_volume/`; the blur
//! and plain output vertex modules are the fog_scattering and volume_caustics
//! ones, byte-identical).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::gltf_viewer::fetch;
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::uniform;
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const RED: wgpu::TextureFormat = wgpu::TextureFormat::R8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// The shadow map size and the cascade count ( a 2 × 1 atlas ).
const MAP: u32 = 2048;
const CASCADES: usize = 2;
const NOISE: u32 = 96;
const SUN: [f64; 3] = [5., 10., 5.];
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("fog_volume/", $name, ".wgsl"))
    };
}
const BLUR_VS: &str = include_str!("fog_scattering/blur_vs.wgsl");
const GAUSS_VS: &str = include_str!("volume_caustics/composite_vs.wgsl");
const RAW_VS: &str = include_str!("volume_caustics/blur_vs.wgsl");
/// Math.round: halves round up.
fn js_round(x: f64) -> f64 {
    (x + 0.5).floor()
}
/// Matrix4.lookAt( eye, target, up ) ( rotation only ).
fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Matrix4 {
    let mut z = eye - target;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    z = z.normalize();
    let mut x = up.cross(z);
    if x.length_squared() == 0. {
        if up.z.abs() == 1. {
            z.x += 0.0001;
        } else {
            z.z += 0.0001;
        }
        z = z.normalize();
        x = up.cross(z);
    }
    x = x.normalize();
    let y = z.cross(x);
    Matrix4::from_cols(x.extend(0.), y.extend(0.), z.extend(0.), Vector4::W)
}
/// One fitted cascade: its camera's view and projection, its shadow matrix
/// in the atlas, its ( begin, end, fade start ) data and its viewport.
#[derive(Clone, Copy)]
pub(super) struct Cascade {
    pub view: Matrix4,
    pub projection: Matrix4,
    pub matrix: Matrix4,
    pub data: [f64; 4],
    pub viewport: [f32; 4],
}
/// SunLightShadow.updateMatrices: practical split cascades fitted to the view
/// frustum, as bounding spheres snapped to the texel grid.
pub(super) fn cascades(
    light: Vector3,
    camera_world: Matrix4,
    projection: Matrix4,
    near: f64,
    far: f64,
    shadow_far: f64,
) -> [Cascade; CASCADES] {
    let shadow_near = 0.5f64;
    let inset = 0.25f64.min((1f64.ceil() + 1.) / MAP as f64);
    let resolution_x = MAP as f64 * (1. - 2. * inset);
    let resolution = resolution_x;
    let camera_far = (near + 1e-6).max(shadow_far.min(far));
    let mut splits = [0.; CASCADES + 1];
    splits[0] = near;
    for (i, split) in splits.iter_mut().enumerate().take(CASCADES).skip(1) {
        let amount = i as f64 / CASCADES as f64;
        let uniform = near + (camera_far - near) * amount;
        let logarithmic = near * (camera_far / near).powf(amount);
        *split = (uniform + logarithmic) * 0.5;
    }
    splits[CASCADES] = camera_far;
    let direction = (-light).normalize();
    let mut up = Vector3::Y;
    if up.dot(direction).abs() > 0.99 {
        up = Vector3::Z;
    }
    let orientation = look_at(Vector3::ZERO, direction, up);
    let view_to_light = orientation.transpose() * camera_world;
    let inverse_projection = projection.inverse();
    let mut near_corners = [Vector3::ZERO; 4];
    let mut far_corners = [Vector3::ZERO; 4];
    let mut global_max_z = f64::NEG_INFINITY;
    for i in 0..4 {
        let x = if i == 0 || i == 1 { 1. } else { -1. };
        let y = if i == 0 || i == 3 { 1. } else { -1. };
        let n = inverse_projection.project_point3(Vector3::new(x, y, 0.));
        let f = n * (camera_far / near);
        near_corners[i] = view_to_light.transform_point3(n);
        far_corners[i] = view_to_light.transform_point3(f);
        global_max_z = global_max_z.max(near_corners[i].z).max(far_corners[i].z);
    }
    global_max_z += camera_far;
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
            corners[j * 2] = near_corners[j].lerp(far_corners[j], near_alpha);
            corners[j * 2 + 1] = near_corners[j].lerp(far_corners[j], far_alpha);
            center += corners[j * 2];
            center += corners[j * 2 + 1];
        }
        center *= 1. / 8.;
        let (mut radius_sq, mut min_z) = (0f64, f64::INFINITY);
        for c in corners {
            radius_sq = radius_sq.max(c.distance_squared(center));
            min_z = min_z.min(c.z);
        }
        let mut radius = radius_sq.sqrt();
        if resolution > 1. {
            radius /= 1. - 1. / resolution;
            let texel = 2. * radius / resolution_x;
            center.x = js_round(center.x / texel) * texel;
            center.y = js_round(center.y / texel) * texel;
        }
        center.z = global_max_z + shadow_near;
        let position = orientation.transform_vector3(center);
        let rotation = Quaternion::from_mat4(&orientation);
        let world = Matrix4::from_rotation_translation(rotation, position);
        let cascade_far_plane = global_max_z - min_z + 2. * shadow_near;
        // makeOrthographic( −r, r, r, −r, near, far ) in WebGPU depth.
        let p = 1. / (cascade_far_plane - shadow_near);
        let projection = Matrix4::from_cols_array(&[
            1. / radius,
            0.,
            0.,
            0.,
            0.,
            1. / radius,
            0.,
            0.,
            0.,
            0.,
            -p,
            0.,
            0.,
            0.,
            -shadow_near * p,
            1.,
        ]);
        let view = world.inverse();
        let viewport = [i as f64 + inset, inset, 1. - 2. * inset, 1. - 2. * inset];
        let (scale_x, scale_y) = (viewport[2] / 2., viewport[3]);
        let (offset_x, offset_y) = (viewport[0] / 2., viewport[1]);
        let bias = Matrix4::from_cols_array(&[
            0.5 * scale_x,
            0.,
            0.,
            0.,
            0.,
            0.5 * scale_y,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            0.5 * scale_x + offset_x,
            0.5 * scale_y + offset_y,
            0.,
            1.,
        ]);
        Cascade {
            view,
            projection,
            matrix: bias * projection * view,
            data: data[i],
            viewport: [
                (viewport[0] * MAP as f64) as f32,
                (viewport[1] * MAP as f64) as f32,
                (viewport[2] * MAP as f64) as f32,
                (viewport[3] * MAP as f64) as f32,
            ],
        }
    })
}
/// createNoise3DTexture( 96 ): two tri-noise octaves per voxel, floored to bytes.
fn noise_texture() -> Vec<u8> {
    let tri = |x: f64| (((x % 1.) + 1.) % 1. - 0.5).abs();
    let tri_noise = |x: f64, y: f64, z: f64| {
        let (mut px, mut py, mut pz) = (x, y, z);
        let (mut bx, mut by, mut bz) = (x, y, z);
        let mut z_factor = 1.4;
        let mut rz = 0.;
        for _ in 0..4 {
            let (gx, gy, gz) = (bx * 2., by * 2., bz * 2.);
            let (dx, dy, dz) = (tri(gz + tri(gy)), tri(gz + tri(gx)), tri(gy + tri(gx)));
            px += dx;
            py += dy;
            pz += dz;
            bx = bx * 1.8 + 0.14;
            by = by * 1.8 + 0.14;
            bz = bz * 1.8 + 0.14;
            z_factor *= 1.5;
            px *= 1.2;
            py *= 1.2;
            pz *= 1.2;
            rz += tri(pz + tri(px + tri(py))) / z_factor;
        }
        rz
    };
    let size = NOISE as f64;
    let mut data = Vec::with_capacity((NOISE * NOISE * NOISE) as usize);
    for z in 0..NOISE {
        for y in 0..NOISE {
            for x in 0..NOISE {
                let (u, v, w) = (x as f64 / size, y as f64 / size, z as f64 / size);
                let n1 = tri_noise(u * 4., v * 4., w * 4.);
                let n2 = tri_noise(u * 8. + 1.7, v * 8. + 0.9, w * 8. + 2.5) * 0.45;
                data.push(((n1 + n2) * 1.15 * 255.).clamp(0., 255.).floor() as u8);
            }
        }
    }
    data
}
/// PLYLoader's Lucy: scale( 0.0024 ), then computeVertexNormals, which
/// accumulates the face normals in the Float32 attribute as it goes.
fn lucy_normals(positions: &[f32], index: &[u32]) -> Vec<f32> {
    let p = |i: u32| {
        let i = i as usize * 3;
        Vector3::new(
            positions[i] as f64,
            positions[i + 1] as f64,
            positions[i + 2] as f64,
        )
    };
    let mut normals = vec![0f32; positions.len()];
    for f in index.chunks(3) {
        let (a, b, c) = (p(f[0]), p(f[1]), p(f[2]));
        let cb = (c - b).cross(a - b);
        for &v in f {
            let i = v as usize * 3;
            normals[i] = (normals[i] as f64 + cb.x) as f32;
            normals[i + 1] = (normals[i + 1] as f64 + cb.y) as f32;
            normals[i + 2] = (normals[i + 2] as f64 + cb.z) as f32;
        }
    }
    for n in normals.chunks_mut(3) {
        let v = Vector3::new(n[0] as f64, n[1] as f64, n[2] as f64);
        let length = v.length();
        let v = v * (1. / if length == 0. { 1. } else { length });
        n.copy_from_slice(&[v.x as f32, v.y as f32, v.z as f32]);
    }
    normals
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    low: (u32, u32),
    msaa: Option<wgpu::TextureView>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    fog: wgpu::TextureView,
    fog_depth: wgpu::TextureView,
    blur: [wgpu::TextureView; 2],
    lucy: Draw,
    grounds: [Draw; 2],
    fog_draw: Draw,
    blur_draws: [Draw; 2],
    /// The JBU, Gaussian blur and raw outputs.
    outputs: [Draw; 3],
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    /// denoiser, resolution scale, steps, cloud threshold, cloud scale, max
    /// ray distance, fog density, height falloff, fog height, cloud speed,
    /// range fog near and far, spatial sigma, depth sensitivity, gaussian
    /// blur, ground.
    params: [f64; 16],
    time: f64,
    last: f64,
    cloud_time: f64,
    pending: bool,
    lucy: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32, Sphere),
    ground: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    shadow: (wgpu::TextureView, wgpu::TextureView),
    shadow_draws: [Draw; CASCADES],
    shadow_renders: [wgpu::Buffer; CASCADES],
    shadow_object: wgpu::Buffer,
    lucy_render: wgpu::Buffer,
    lucy_object: wgpu::Buffer,
    ground_renders: [wgpu::Buffer; 2],
    ground_objects: [wgpu::Buffer; 2],
    fog_render: wgpu::Buffer,
    fog_object: wgpu::Buffer,
    noise: wgpu::TextureView,
    blur_objects: [wgpu::Buffer; 2],
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    gauss_render: wgpu::Buffer,
    raw_render: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    clamp: wgpu::Sampler,
    repeat: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
    /// Targets of other resolution scales at the current size.
    cached: Vec<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(10., 4., 1.);
        let mut controls = Controls::new(Some(0.05), (2., 20.), PI / 2., true);
        controls.set_target(Vector3::new(0., 1., 0.));
        controls.update(s, c)?;
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
        let (mut positions, index) = super::refraction_loaders::formats::parse_ply(
            &fetch("/web/gallery/assets/ply/binary/Lucy100k.ply").await?,
        )?;
        for p in &mut positions {
            *p = (*p as f64 * 0.0024) as f32;
        }
        let normals = lucy_normals(&positions, &index);
        let sphere = {
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
        };
        let lucy = (
            init("Lucy", bytemuck::cast_slice(&positions), vertex),
            init("Lucy", bytemuck::cast_slice(&normals), vertex),
            init("Lucy index", bytemuck::cast_slice(&index), index_usage),
            index.len() as u32,
            sphere,
        );
        // PlaneGeometry( 200, 200 ): two triangles facing +z.
        let ground_positions: [f32; 12] = [
            -100., 100., 0., 100., 100., 0., -100., -100., 0., 100., -100., 0.,
        ];
        let ground_normals: [f32; 12] = [0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1.];
        let ground = (
            init(
                "fog ground",
                bytemuck::cast_slice(&ground_positions),
                vertex,
            ),
            init("fog ground", bytemuck::cast_slice(&ground_normals), vertex),
            init(
                "fog ground index",
                bytemuck::cast_slice(&[0u32, 2, 1, 2, 3, 1]),
                index_usage,
            ),
            6,
        );
        let texture = |size: (u32, u32), format| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("fog shadow"),
                    size: wgpu::Extent3d {
                        width: size.0,
                        height: size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let shadow = (
            texture((MAP * CASCADES as u32, MAP), BYTE),
            texture((MAP * CASCADES as u32, MAP), DEPTH),
        );
        let attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs[i],
        });
        // The shadow pass draws the statue's back faces.
        let shadow_pipeline = sampled_pipeline(
            r,
            "fog shadow",
            (wgsl!("shadow_vs"), wgsl!("shadow_fs")),
            &layouts[..1],
            &[BYTE],
            Some((wgpu::CompareFunction::LessEqual, true)),
            (true, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
        );
        let shadow_renders =
            [0, 1].map(|_| uniform(r, "fog shadow", wgsl!("shadow_vs"), "renderStruct"));
        let [s0, s1] = shadow_renders;
        let shadow_renders = [s0?, s1?];
        let shadow_object = uniform(r, "fog shadow", wgsl!("shadow_vs"), "objectStruct")?;
        let shadow_draws = shadow_renders.each_ref().map(|render| {
            (
                shadow_pipeline.clone(),
                vec![
                    bind(
                        r,
                        shadow_pipeline.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(
                        r,
                        shadow_pipeline.get_bind_group_layout(1),
                        &[(0, shadow_object.as_entire_binding())],
                    ),
                ],
            )
        });
        let noise = r
            .device
            .create_texture_with_data(
                &r.queue,
                &wgpu::TextureDescriptor {
                    label: Some("fog noise"),
                    size: wgpu::Extent3d {
                        width: NOISE,
                        height: NOISE,
                        depth_or_array_layers: NOISE,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D3,
                    format: RED,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                &noise_texture(),
            )
            .create_view(&Default::default());
        let sampler = |address, compare| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address,
                address_mode_v: address,
                address_mode_w: address,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                compare,
                ..Default::default()
            })
        };
        Ok(Self {
            controls,
            params: [
                0., 0.4, 16., 0.66, 0.019, 70., 1.05, 1.2, 3.5, 0.04, 62.5, 100., 1.2, 30., 0.5, 0.,
            ],
            time: 0.,
            last: 0.,
            cloud_time: 0.,
            pending: true,
            lucy,
            ground,
            shadow,
            shadow_draws,
            shadow_renders,
            shadow_object,
            lucy_render: uniform(r, "fog Lucy", wgsl!("lucy_fs"), "renderStruct")?,
            lucy_object: uniform(r, "fog Lucy", wgsl!("lucy_fs"), "objectStruct")?,
            ground_renders: [
                uniform(r, "fog ground", wgsl!("ground_fs"), "renderStruct")?,
                uniform(r, "fog grid", wgsl!("grid_fs"), "renderStruct")?,
            ],
            ground_objects: [
                uniform(r, "fog ground", wgsl!("ground_fs"), "objectStruct")?,
                uniform(r, "fog grid", wgsl!("grid_fs"), "objectStruct")?,
            ],
            fog_render: uniform(r, "fog pass", wgsl!("fog_fs"), "renderStruct")?,
            fog_object: uniform(r, "fog pass", wgsl!("fog_fs"), "objectStruct")?,
            noise,
            blur_objects: [
                uniform(r, "fog blur", wgsl!("blur_h_fs"), "objectStruct")?,
                uniform(r, "fog blur", wgsl!("blur_v_fs"), "objectStruct")?,
            ],
            output_render: uniform(r, "fog output", wgsl!("output_fs"), "renderStruct")?,
            output_object: uniform(r, "fog output", wgsl!("output_fs"), "objectStruct")?,
            gauss_render: uniform(r, "fog gauss", wgsl!("gauss_fs"), "renderStruct")?,
            raw_render: uniform(r, "fog raw", wgsl!("raw_fs"), "renderStruct")?,
            quad_uv: init(
                "fog quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            clamp: sampler(wgpu::AddressMode::ClampToEdge, None),
            repeat: sampler(wgpu::AddressMode::Repeat, None),
            compare: sampler(
                wgpu::AddressMode::ClampToEdge,
                Some(wgpu::CompareFunction::LessEqual),
            ),
            targets: None,
            cached: vec![],
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
    /// The low-resolution fog target ( floor of the scaled size ).
    fn low_size(&self, width: u32, height: u32) -> (u32, u32) {
        let scale = self.params[1];
        (
            ((width as f64 * scale).floor() as u32).max(1),
            ((height as f64 * scale).floor() as u32).max(1),
        )
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget, samples: u32) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let low = self.low_size(width, height);
        let texture = |size: (u32, u32), format, samples| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("fog target"),
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
        let fog = texture(low, RED, 1);
        let fog_depth = texture(low, DEPTH, 1);
        // GaussianBlurNode's targets: RGBA bytes, the blurred value in every channel.
        let blur = [texture(low, BYTE, 1), texture(low, BYTE, 1)];
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs[i],
        });
        let lit = |shaders, render: &wgpu::Buffer, object: &wgpu::Buffer| -> Draw {
            let p = sampled_pipeline(
                r,
                "fog lit",
                shaders,
                &layouts,
                &[HALF],
                Some((wgpu::CompareFunction::LessEqual, true)),
                (false, false),
                (samples, wgpu::PrimitiveTopology::TriangleList),
            );
            (
                p.clone(),
                vec![
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(
                        r,
                        p.get_bind_group_layout(1),
                        &[
                            (0, object.as_entire_binding()),
                            (1, sampler(&self.clamp)),
                            (2, tex(&r.dfg)),
                            (3, sampler(&self.compare)),
                            (4, tex(&self.shadow.1)),
                        ],
                    ),
                ],
            )
        };
        let lucy = lit(
            (wgsl!("lucy_vs"), wgsl!("lucy_fs")),
            &self.lucy_render,
            &self.lucy_object,
        );
        let grounds = [
            lit(
                (wgsl!("ground_vs"), wgsl!("ground_fs")),
                &self.ground_renders[0],
                &self.ground_objects[0],
            ),
            lit(
                (wgsl!("grid_vs"), wgsl!("grid_fs")),
                &self.ground_renders[1],
                &self.ground_objects[1],
            ),
        ];
        let quad_attrs = [wgpu::vertex_attr_array![0 => Float32x2]];
        let quad = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &quad_attrs[0],
        }];
        let screen = |label, shaders, format, depth| {
            sampled_pipeline(
                r,
                label,
                shaders,
                &quad,
                &[format],
                depth,
                (false, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
            )
        };
        let two = |p: &wgpu::RenderPipeline,
                   render: &wgpu::Buffer,
                   entries: &[(u32, wgpu::BindingResource)]|
         -> Draw {
            (
                p.clone(),
                vec![
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(r, p.get_bind_group_layout(1), entries),
                ],
            )
        };
        let ms = samples > 1;
        let fog_pipeline = screen(
            "fog pass",
            (
                wgsl!("fog_vs"),
                if ms {
                    wgsl!("fog_ms_fs")
                } else {
                    wgsl!("fog_fs")
                },
            ),
            RED,
            Some((wgpu::CompareFunction::LessEqual, true)),
        );
        let fog_draw = two(
            &fog_pipeline,
            &self.fog_render,
            &[
                (0, self.fog_object.as_entire_binding()),
                (1, tex(&depth)),
                (2, sampler(&self.repeat)),
                (3, tex(&self.noise)),
            ],
        );
        let blur_draws = [
            (wgsl!("blur_h_fs"), &fog, 0),
            (wgsl!("blur_v_fs"), &blur[0], 1),
        ]
        .map(|(fs, source, i)| {
            let p = screen("fog blur", (BLUR_VS, fs), BYTE, None);
            (
                p.clone(),
                vec![bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[
                        (0, sampler(&self.clamp)),
                        (1, tex(source)),
                        (2, self.blur_objects[i].as_entire_binding()),
                    ],
                )],
            )
        });
        let format = out.options.format;
        let jbu = screen(
            "fog output",
            (
                wgsl!("output_vs"),
                if ms {
                    wgsl!("output_ms_fs")
                } else {
                    wgsl!("output_fs")
                },
            ),
            format,
            None,
        );
        let gauss = screen("fog output", (GAUSS_VS, wgsl!("gauss_fs")), format, None);
        let raw = screen("fog output", (RAW_VS, wgsl!("raw_fs")), format, None);
        let outputs = [
            two(
                &jbu,
                &self.output_render,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&color)),
                    (2, sampler(&self.clamp)),
                    (3, tex(&fog)),
                    (4, self.output_object.as_entire_binding()),
                    (5, tex(&depth)),
                ],
            ),
            two(
                &gauss,
                &self.gauss_render,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&color)),
                    (2, sampler(&self.clamp)),
                    (3, tex(&blur[1])),
                ],
            ),
            two(
                &raw,
                &self.raw_render,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&color)),
                    (2, sampler(&self.clamp)),
                    (3, tex(&fog)),
                ],
            ),
        ];
        let screen_target = RenderTarget::with_options(
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
            low,
            msaa,
            color,
            depth,
            fog,
            fog_depth,
            blur,
            lucy,
            grounds,
            fog_draw,
            blur_draws,
            outputs,
            screen: screen_target,
        });
        Ok(())
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("fog output"),
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
        set(
            &mut pass,
            &t.outputs[(self.params[0].round() as usize).min(2)],
        );
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
        let low = self.low_size(out.width, out.height);
        let fits = |t: &Targets| {
            t.width == out.width
                && t.height == out.height
                && t.format == out.options.format
                && t.samples == samples
                && t.low == low
        };
        if self.targets.as_ref().is_none_or(|t| !fits(t)) {
            // The resolution scale moves between a few sizes: the targets of
            // the sizes visited stay resident, as at most a handful.
            if let Some(i) = self.cached.iter().position(fits) {
                let t = self.cached.swap_remove(i);
                if let Some(old) = self.targets.replace(t) {
                    self.cached.push(old);
                }
            } else {
                if let Some(old) = self.targets.take()
                    && old.width == out.width
                    && old.height == out.height
                    && old.samples == samples
                {
                    self.cached.push(old);
                    if self.cached.len() > 4 {
                        self.cached.remove(0);
                    }
                } else {
                    self.cached.clear();
                }
                self.resize(r, out, samples)?;
            }
        }
        let t = self.targets.as_ref().ok_or(Error::Invalid("fog targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate(): the cloud clock advances by the timer's delta.
        let delta = (self.time - self.last).max(0.);
        self.last = self.time;
        let [
            _,
            resolution_scale,
            steps,
            threshold,
            cloud_scale,
            max_ray,
            density,
            falloff,
            fog_height,
            cloud_speed,
            range_near,
            range_far,
            spatial_sigma,
            depth_sensitivity,
            blur_radius,
            grid,
        ] = self.params;
        self.cloud_time += delta * cloud_speed;
        self.controls.frame_update(s, c)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.w_axis.truncate();
        let cascades = cascades(Vector3::from_array(SUN), world, projection, 0.1, 100., 25.);
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
        let lucy_model = Matrix4::from_translation(Vector3::new(0., 0.8, 0.))
            * Matrix4::from_rotation_y(-PI / 2.);
        for (i, cascade) in cascades.iter().enumerate() {
            write(
                &self.shadow_renders[i],
                wgsl!("shadow_vs"),
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", m4(cascade.projection)),
                    ("cameraViewMatrix", m4(cascade.view)),
                ],
            )?;
        }
        write(
            &self.shadow_object,
            wgsl!("shadow_vs"),
            "objectStruct",
            &[("nodeUniform0", vec![1.]), ("nodeUniform3", m4(lucy_model))],
        )?;
        let size = vec![(MAP * CASCADES as u32) as f64, MAP as f64];
        // The hemisphere and sun lights and the two cascades, by the names
        // the statue's shader gives them ( the ground's are one less ).
        let lights = |offset: u32| -> Vec<(String, Vec<f64>)> {
            let n = |k: u32| format!("nodeUniform{}", k - offset);
            vec![
                ("cameraProjectionMatrix".to_string(), m4(projection)),
                ("cameraViewMatrix".to_string(), m4(view)),
                (n(11), vec![0.5; 3]),
                (n(13), vec![0., 1., 0.]),
                (n(10), vec![0.13318315973474043; 3]),
                (n(15), vec![2.5; 3]),
                (n(14), SUN.to_vec()),
                (n(18), m4(cascades[1].matrix)),
                (n(17), cascades[1].data.to_vec()),
                (n(24), m4(cascades[0].matrix)),
                (n(23), cascades[0].data.to_vec()),
                (n(16), vec![0.]),
                (n(19), vec![0.]),
                (n(21), vec![1.]),
                (n(22), size.clone()),
                (n(25), vec![0.]),
                (n(26), vec![1.]),
                (n(27), size.clone()),
                (n(28), vec![1.]),
            ]
        };
        fn named(values: &[(String, Vec<f64>)]) -> Vec<(&str, Vec<f64>)> {
            values
                .iter()
                .map(|(n, v)| (n.as_str(), v.clone()))
                .collect()
        }
        let (lucy_lights, ground_lights) = (lights(0), lights(1));
        write(
            &self.lucy_render,
            wgsl!("lucy_fs"),
            "renderStruct",
            &named(&lucy_lights),
        )?;
        write(
            &self.lucy_object,
            wgsl!("lucy_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.05]),
                ("nodeUniform3", vec![0.3]),
                ("nodeUniform5", m3(lucy_model.inverse().transpose())),
                ("nodeUniform6", vec![0.; 3]),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(lucy_model)),
            ],
        )?;
        let ground_model = Matrix4::from_translation(Vector3::new(0., -1., 0.))
            * Matrix4::from_rotation_x(-PI / 2.);
        let ground_normal = m3(ground_model.inverse().transpose());
        write(
            &self.ground_renders[0],
            wgsl!("ground_fs"),
            "renderStruct",
            &named(&ground_lights),
        )?;
        write(
            &self.ground_objects[0],
            wgsl!("ground_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.]),
                ("nodeUniform1", vec![0.]),
                ("nodeUniform2", vec![1.]),
                ("nodeUniform4", ground_normal.clone()),
                ("nodeUniform5", vec![0.; 3]),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform8", m4(ground_model)),
            ],
        )?;
        write(
            &self.ground_renders[1],
            wgsl!("grid_fs"),
            "renderStruct",
            &named(&ground_lights),
        )?;
        write(
            &self.ground_objects[1],
            wgsl!("grid_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", m4(ground_model)),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", ground_normal),
                ("nodeUniform6", vec![0.; 3]),
                ("nodeUniform7", vec![1.]),
            ],
        )?;
        let projection_inverse = m4(projection.inverse());
        write(
            &self.fog_render,
            wgsl!("fog_fs"),
            "renderStruct",
            &[("nodeUniform3", vec![t.low.0 as f64, t.low.1 as f64])],
        )?;
        write(
            &self.fog_object,
            wgsl!("fog_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", camera_position.to_array().to_vec()),
                ("nodeUniform1", m4(world)),
                ("nodeUniform2", projection_inverse.clone()),
                ("nodeUniform5", vec![fog_height]),
                ("nodeUniform6", vec![max_ray]),
                ("nodeUniform7", vec![steps]),
                ("nodeUniform8", vec![cloud_scale]),
                ("nodeUniform10", vec![self.cloud_time]),
                ("nodeUniform11", vec![threshold]),
                ("nodeUniform12", vec![falloff]),
                ("nodeUniform13", vec![density]),
                ("nodeUniform14", vec![range_near]),
                ("nodeUniform15", vec![range_far]),
            ],
        )?;
        let texel = vec![1. / t.low.0 as f64, 1. / t.low.1 as f64];
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
                    ("nodeUniform1", vec![blur_radius]),
                    ("nodeUniform2", texel.clone()),
                ],
            )?;
        }
        let full = vec![t.width as f64, t.height as f64];
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[("nodeUniform2", full.clone()), ("nodeUniform8", vec![1.])],
        )?;
        write(
            &self.output_object,
            wgsl!("output_fs"),
            "objectStruct",
            &[
                ("nodeUniform3", vec![resolution_scale]),
                ("nodeUniform4", vec![spatial_sigma]),
                ("nodeUniform5", projection_inverse),
                ("nodeUniform7", vec![depth_sensitivity]),
            ],
        )?;
        write(
            &self.gauss_render,
            wgsl!("gauss_fs"),
            "renderStruct",
            &[("nodeUniform2", vec![1.])],
        )?;
        write(
            &self.raw_render,
            wgsl!("raw_fs"),
            "renderStruct",
            &[("nodeUniform3", vec![1.]), ("nodeUniform2", full)],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        for (i, cascade) in cascades.iter().enumerate() {
            let clear = i == 0;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fog shadow"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.shadow.0,
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.shadow.1,
                    depth_ops: Some(wgpu::Operations {
                        load: if clear {
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
            set(&mut pass, &self.shadow_draws[i]);
            pass.set_vertex_buffer(0, self.lucy.0.slice(..));
            pass.set_index_buffer(self.lucy.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.lucy.3, 0, 0..1);
        }
        {
            let (target, resolve) = match &t.msaa {
                Some(msaa) => (msaa, Some(&t.color)),
                None => (&t.color, None),
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fog scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: resolve,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
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
            // The statue, then the ground ( the opaque order of the page ).
            let view_projection = projection * view;
            let frustum = Frustum::from_projection(view_projection);
            let sphere = self.lucy.4;
            let lucy_sphere = Sphere {
                center: lucy_model.transform_point3(sphere.center),
                radius: sphere.radius,
            };
            let mut order = vec![((view_projection * Vector4::new(0., -1., 0., 1.)).z, false)];
            if frustum.intersects_sphere(lucy_sphere) {
                order.push(((view_projection * lucy_sphere.center.extend(1.)).z, true));
            }
            order.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (_, statue) in order {
                if statue {
                    // Normal, then position.
                    set(&mut pass, &t.lucy);
                    pass.set_vertex_buffer(0, self.lucy.1.slice(..));
                    pass.set_vertex_buffer(1, self.lucy.0.slice(..));
                    pass.set_index_buffer(self.lucy.2.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..self.lucy.3, 0, 0..1);
                } else {
                    // The solid ground reads normal then position, the grid
                    // position then normal.
                    let grid = grid > 0.5;
                    let (first, second) = if grid {
                        (&self.ground.0, &self.ground.1)
                    } else {
                        (&self.ground.1, &self.ground.0)
                    };
                    set(&mut pass, &t.grounds[usize::from(grid)]);
                    pass.set_vertex_buffer(0, first.slice(..));
                    pass.set_vertex_buffer(1, second.slice(..));
                    pass.set_index_buffer(self.ground.2.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..self.ground.3, 0, 0..1);
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fog low resolution"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.fog,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.fog_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            set(&mut pass, &t.fog_draw);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        }
        if self.params[0].round() as usize == 1 {
            for (target, draw) in t.blur.iter().zip(&t.blur_draws) {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("fog blur"),
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
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("fog parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
