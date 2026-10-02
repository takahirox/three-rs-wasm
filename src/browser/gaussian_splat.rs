//! webgpu_gaussian_splat: a Gaussian splat scene ( the lion SPZ v3, the
//! millipede SPLAT or the tomatoes SPZ v4 ) drawn as instanced, depth-sorted
//! screen-space Gaussians. The loaders decode and pack the splats on the CPU
//! once, as SPZLoader, SPLATLoader and GaussianSplatUtils do. The GPU then
//! runs GaussianSplat's passes: a 4,096-bin counting sort (reset, histogram,
//! prefix and scatter) whenever the view direction turns past the page's
//! threshold, the spherical-harmonics colors whenever the camera moves, and
//! the instanced quads reading the sorted order. Every stage runs the WGSL
//! three.js r186 generates for the page (in `gaussian_splat/`, the kernels
//! without their unused subgroup built-ins).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::gltf_viewer::fetch;
use super::lights_projector::{m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::uniform;
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::io::Read;
use std::rc::Rc;
use wgpu::util::DeviceExt;

const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const BIN_COUNT: u32 = 4096;
const WORKGROUP: u32 = 256;
const SORT_DIRECTION_THRESHOLD: f64 = 0.9995;
const KERNEL_CUTOFF: f64 = 2.;
const SH_C0: f64 = 0.2820947917738781;
const SH_BAND_COMPONENTS: [usize; 4] = [0, 9, 15, 21];
const SH_BAND_WORDS: [usize; 4] = [0, 3, 4, 6];
const SH_DEGREE_TO_VECTORS: [usize; 5] = [0, 3, 8, 15, 24];
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("gaussian_splat/", $name, ".wgsl"))
    };
}
/// The page's sources: name, URL, Euler rotation and camera direction.
const SOURCES: [(&str, [f64; 3], [f64; 3]); 3] = [
    (
        "/web/gallery/assets/splat/millipede.splat",
        [PI * 1.10, PI * -0.10, PI * 0.15],
        [0., 0.4, 1.],
    ),
    (
        "/web/gallery/assets/spz/lion.v3.spz",
        [PI, 0., 0.],
        [0., 0.2, 1.],
    ),
    (
        "/web/gallery/assets/spz/tomatoes.v4.spz",
        [0., 0., 0.],
        [0., 0.35, 1.],
    ),
];
/// Uint8ClampedArray's ToUint8Clamp: round half to even within 0 … 255.
fn clamp_u8(x: f64) -> u8 {
    if x.is_nan() || x <= 0. {
        return 0;
    }
    if x >= 255. {
        return 255;
    }
    let f = x.floor();
    // Halves round to even.
    let r = if f + 0.5 < x || (x == f + 0.5 && f % 2. != 0.) {
        f + 1.
    } else {
        f
    };
    r as u8
}
/// GaussianSplatUtils.writeCovariance: R·S·(R·S)ᵀ in three.js's order.
fn covariance(s: [f64; 3], q: [f64; 4]) -> [f32; 6] {
    let l = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    let [x, y, z, w] = if l == 0. {
        [0., 0., 0., 1.]
    } else {
        let l = 1. / l;
        [q[0] * l, q[1] * l, q[2] * l, q[3] * l]
    };
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    let [sx, sy, sz] = s;
    // Matrix3 elements, column-major.
    let a = [
        (1. - (yy + zz)) * sx,
        (xy + wz) * sx,
        (xz - wy) * sx,
        (xy - wz) * sy,
        (1. - (xx + zz)) * sy,
        (yz + wx) * sy,
        (xz + wy) * sz,
        (yz - wx) * sz,
        (1. - (xx + yy)) * sz,
    ];
    let b = [a[0], a[3], a[6], a[1], a[4], a[7], a[2], a[5], a[8]];
    let (a11, a12, a13, a21, a22, a23, a31, a32, a33) =
        (a[0], a[3], a[6], a[1], a[4], a[7], a[2], a[5], a[8]);
    let (b11, b12, b13, b21, b22, b23, b31, b32, b33) =
        (b[0], b[3], b[6], b[1], b[4], b[7], b[2], b[5], b[8]);
    let te0 = a11 * b11 + a12 * b21 + a13 * b31;
    let te3 = a11 * b12 + a12 * b22 + a13 * b32;
    let te6 = a11 * b13 + a12 * b23 + a13 * b33;
    let te4 = a21 * b12 + a22 * b22 + a23 * b32;
    let te7 = a21 * b13 + a22 * b23 + a23 * b33;
    let te8 = a31 * b13 + a32 * b23 + a33 * b33;
    [te0, te3, te6, te4, te7, te8].map(|v| v as f32)
}
/// A decoded splat set as createGaussianSplatGeometry stores it.
struct SplatData {
    centers: Vec<f32>,
    covariances: Vec<f32>,
    colors: Vec<u8>,
    /// Packed spherical-harmonics words per band ( 1 … degree ).
    bands: Vec<Vec<u32>>,
}
fn read_int24(b: &[u8], o: usize) -> i32 {
    ((b[o] as i32) << 8 | (b[o + 1] as i32) << 16 | (b[o + 2] as i32) << 24) >> 8
}
/// parseSPZAttributes for versions 2 to 4.
#[allow(clippy::too_many_arguments)]
fn spz_attributes(
    positions: &[u8],
    alphas: &[u8],
    colors: &[u8],
    scales: &[u8],
    rotations: &[u8],
    harmonics: &[u8],
    count: usize,
    version: u32,
    fractional_bits: u8,
    stored_degree: usize,
) -> Result<SplatData> {
    let degree = stored_degree.min(3);
    let scale_lut: Vec<f32> = (0..256)
        .map(|i| (i as f64 / 16. - 10.).exp() as f32)
        .collect();
    let color_scale = SH_C0 / 0.15;
    let color_lut: Vec<u8> = (0..256)
        .map(|i| clamp_u8(((i as f64 / 255. - 0.5) * color_scale + 0.5) * 255.))
        .collect();
    let quat_lut: Vec<f64> = (0..1024)
        .map(|i| {
            let v = std::f64::consts::FRAC_1_SQRT_2 * ((i & 511) as f64 / 511.);
            if i & 512 != 0 { -v } else { v }
        })
        .collect();
    let fixed = 1. / (1u64 << fractional_bits) as f64;
    let mut centers = Vec::with_capacity(count * 3);
    let mut covariances = Vec::with_capacity(count * 6);
    let mut out_colors = Vec::with_capacity(count * 4);
    for i in 0..count {
        for k in 0..3 {
            centers.push((read_int24(positions, i * 9 + k * 3) as f64 * fixed) as f32);
        }
        let s = [0, 1, 2].map(|k| scale_lut[scales[i * 3 + k] as usize] as f64);
        let q = if version >= 3 {
            let packed = u32::from_le_bytes([
                rotations[i * 4],
                rotations[i * 4 + 1],
                rotations[i * 4 + 2],
                rotations[i * 4 + 3],
            ]);
            let largest = (packed >> 30) as usize;
            let a = quat_lut[(packed & 1023) as usize];
            let b = quat_lut[((packed >> 10) & 1023) as usize];
            let c = quat_lut[((packed >> 20) & 1023) as usize];
            let mut t = [0.; 4];
            let order: [usize; 3] = match largest {
                0 => [1, 2, 3],
                1 => [0, 2, 3],
                2 => [0, 1, 3],
                _ => [0, 1, 2],
            };
            t[order[0]] = c;
            t[order[1]] = b;
            t[order[2]] = a;
            t[largest] = (1. - (a * a + b * b + c * c)).max(0.).sqrt();
            t
        } else {
            let qx = rotations[i * 3] as f64 / 127.5 - 1.;
            let qy = rotations[i * 3 + 1] as f64 / 127.5 - 1.;
            let qz = rotations[i * 3 + 2] as f64 / 127.5 - 1.;
            [
                qx,
                qy,
                qz,
                (1. - qx * qx - qy * qy - qz * qz).max(0.).sqrt(),
            ]
        };
        covariances.extend(covariance(s, q));
        out_colors.extend([
            color_lut[colors[i * 3] as usize],
            color_lut[colors[i * 3 + 1] as usize],
            color_lut[colors[i * 3 + 2] as usize],
            alphas[i],
        ]);
    }
    // readSphericalHarmonics: each band's bytes in 0x80-filled packed words.
    let mut bands = vec![];
    let stride = SH_DEGREE_TO_VECTORS[stored_degree] * 3;
    let mut bytes: Vec<Vec<u8>> = (1..=degree)
        .map(|band| vec![0x80u8; count * SH_BAND_WORDS[band] * 4])
        .collect();
    for i in 0..count {
        let mut offset = i * stride;
        for (b, band_bytes) in bytes.iter_mut().enumerate() {
            let band = b + 1;
            let target = i * SH_BAND_WORDS[band] * 4;
            for j in 0..SH_BAND_COMPONENTS[band] {
                band_bytes[target + j] = harmonics[offset];
                offset += 1;
            }
        }
    }
    for b in bytes {
        bands.push(bytemuck::cast_slice::<u8, u32>(&b).to_vec());
    }
    Ok(SplatData {
        centers,
        covariances,
        colors: out_colors,
        bands,
    })
}
/// SPZLoader: gzip-wrapped versions 1 – 3 and the zstd-streamed version 4.
fn parse_spz(data: &[u8]) -> Result<SplatData> {
    let bad = |m: &'static str| Error::Asset(format!("SPZ: {m}"));
    let u32_at = |b: &[u8], o: usize| u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
    if data.len() >= 8 && u32_at(data, 0) == 0x5053474e {
        // Version 4: a table of zstd streams.
        let count = u32_at(data, 8) as usize;
        let stored = data[12] as usize;
        let fractional = data[13];
        let streams = data[15] as usize;
        let toc = u32_at(data, 16) as usize;
        let sizes = [
            count * 9,
            count,
            count * 3,
            count * 3,
            count * 4,
            count * SH_DEGREE_TO_VECTORS[stored] * 3,
        ];
        let mut offset = toc + streams * 16;
        let mut entries = vec![];
        for i in 0..streams {
            let e = toc + i * 16;
            let size =
                u64::from_le_bytes(data[e..e + 8].try_into().map_err(|_| bad("toc"))?) as usize;
            entries.push((offset, size));
            offset += size;
        }
        let mut decoded = vec![];
        let mut next = 0;
        for size in sizes {
            if size == 0 {
                decoded.push(vec![]);
                continue;
            }
            let (o, n) = *entries.get(next).ok_or(bad("stream"))?;
            next += 1;
            let mut source = data.get(o..o + n).ok_or(bad("stream bytes"))?;
            let mut decoder = ruzstd::streaming_decoder::StreamingDecoder::new(&mut source)
                .map_err(|_| bad("zstd"))?;
            let mut out = Vec::with_capacity(size);
            decoder
                .read_to_end(&mut out)
                .map_err(|_| bad("zstd stream"))?;
            out.truncate(size);
            decoded.push(out);
        }
        return spz_attributes(
            &decoded[0],
            &decoded[1],
            &decoded[2],
            &decoded[3],
            &decoded[4],
            &decoded[5],
            count,
            4,
            fractional,
            stored,
        );
    }
    // gzip: the member header, then raw deflate.
    if data.len() < 18 || data[0] != 0x1f || data[1] != 0x8b {
        return Err(bad("gzip header"));
    }
    let flags = data[3];
    let mut at = 10;
    if flags & 4 != 0 {
        at += 2 + u16::from_le_bytes([data[at], data[at + 1]]) as usize;
    }
    for flag in [8, 16] {
        if flags & flag != 0 {
            while data[at] != 0 {
                at += 1;
            }
            at += 1;
        }
    }
    if flags & 2 != 0 {
        at += 2;
    }
    let bytes = miniz_oxide::inflate::decompress_to_vec(&data[at..]).map_err(|_| bad("inflate"))?;
    let count = u32_at(&bytes, 8) as usize;
    let version = u32_at(&bytes, 4);
    let stored = bytes[12] as usize;
    let fractional = bytes[13];
    if !(2..=3).contains(&version) {
        return Err(bad("version"));
    }
    let rotations = count * if version == 3 { 4 } else { 3 };
    let mut o = 16;
    let mut take = |n: usize| {
        let s = o;
        o += n;
        bytes.get(s..o).ok_or(bad("truncated"))
    };
    let positions = take(count * 9)?;
    let alphas = take(count)?;
    let colors = take(count * 3)?;
    let scales = take(count * 3)?;
    let rots = take(rotations)?;
    let harmonics = take(count * SH_DEGREE_TO_VECTORS[stored] * 3)?;
    spz_attributes(
        positions, alphas, colors, scales, rots, harmonics, count, version, fractional, stored,
    )
}
/// SPLATLoader: 32-byte rows of center, scale, color and quaternion.
fn parse_splat(data: &[u8]) -> Result<SplatData> {
    if !data.len().is_multiple_of(32) {
        return Err(Error::Asset("SPLAT: byte length".into()));
    }
    let count = data.len() / 32;
    let f = |o: usize| f32::from_le_bytes([data[o], data[o + 1], data[o + 2], data[o + 3]]);
    let mut centers = Vec::with_capacity(count * 3);
    let mut covariances = Vec::with_capacity(count * 6);
    let mut colors = Vec::with_capacity(count * 4);
    for i in 0..count {
        let r = i * 32;
        centers.extend([f(r), f(r + 4), f(r + 8)]);
        let s = [f(r + 12) as f64, f(r + 16) as f64, f(r + 20) as f64];
        colors.extend_from_slice(&data[r + 24..r + 28]);
        let q = |k: usize| (data[r + k] as f64 - 128.) / 128.;
        covariances.extend(covariance(s, [q(29), q(30), q(31), q(28)]));
    }
    Ok(SplatData {
        centers,
        covariances,
        colors,
        bands: vec![],
    })
}
/// BufferGeometry.computeBoundingSphere over the centers.
fn center_sphere(c: &[f32]) -> Sphere {
    let points = || {
        c.chunks(3)
            .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
    };
    let (lo, hi) = points().fold(
        (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        ),
        |(lo, hi), p| (lo.min(p), hi.max(p)),
    );
    let center = (lo + hi) * 0.5;
    Sphere {
        center,
        radius: points()
            .map(|p| p.distance_squared(center))
            .fold(0., f64::max)
            .sqrt(),
    }
}
/// GaussianSplat.computeBoundingSphere: the kernel-padded box's sphere,
/// widened to every splat's extent.
fn kernel_sphere(c: &[f32], cov: &[f32]) -> Sphere {
    let radius = |i: usize| {
        KERNEL_CUTOFF
            * (cov[i * 6] as f64)
                .max(cov[i * 6 + 3] as f64)
                .max(cov[i * 6 + 5] as f64)
                .sqrt()
    };
    let n = c.len() / 3;
    let p = |i: usize| Vector3::new(c[i * 3] as f64, c[i * 3 + 1] as f64, c[i * 3 + 2] as f64);
    let (mut lo, mut hi) = (
        Vector3::splat(f64::INFINITY),
        Vector3::splat(f64::NEG_INFINITY),
    );
    for i in 0..n {
        let r = radius(i);
        lo = lo.min(p(i) - Vector3::splat(r));
        hi = hi.max(p(i) + Vector3::splat(r));
    }
    let center = (lo + hi) * 0.5;
    let radius = (0..n)
        .map(|i| center.distance(p(i)) + radius(i))
        .fold(0., f64::max);
    Sphere { center, radius }
}
/// One loaded source on the GPU.
struct Splats {
    count: u32,
    degree: usize,
    model: Matrix4,
    /// The centers' sphere ( the camera fit ) and the kernel sphere ( the sort range ).
    fit: Sphere,
    sort_sphere: Sphere,
    sort: [(wgpu::ComputePipeline, wgpu::BindGroup); 4],
    sort_objects: [wgpu::Buffer; 4],
    harmonics: Option<(wgpu::ComputePipeline, wgpu::BindGroup, wgpu::Buffer)>,
    draw: Draw,
    render: wgpu::Buffer,
    object: wgpu::Buffer,
    vs: &'static str,
    fs: &'static str,
    /// The last sort direction and the camera and world matrices of the
    /// last spherical-harmonics pass.
    sorted: Option<Vector3>,
    harmonics_at: Option<(Matrix4, Matrix4)>,
}
type SplatLoad = Rc<RefCell<Option<Result<SplatData>>>>;
async fn load(source: usize) -> Result<SplatData> {
    let bytes = fetch(SOURCES[source].0).await?;
    if source == 0 {
        parse_splat(&bytes)
    } else {
        parse_spz(&bytes)
    }
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    depth: wgpu::TextureView,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    pending: bool,
    source: usize,
    loaded: Vec<Option<Splats>>,
    loading: Option<(usize, SplatLoad)>,
    switch: Option<usize>,
    quad: (wgpu::Buffer, wgpu::Buffer),
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 0.01,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0.35, 3.2);
        let mut controls = Controls::new(Some(0.05), (1.2, 8.), PI, true);
        controls.set_target(Vector3::ZERO);
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let quad = (
            init(
                "splat quad",
                bytemuck::cast_slice(&[-2f32, -2., 0., 2., -2., 0., 2., 2., 0., -2., 2., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            init(
                "splat quad index",
                bytemuck::cast_slice(&[0u32, 1, 2, 0, 2, 3]),
                wgpu::BufferUsages::INDEX,
            ),
        );
        let mut demo = Self {
            controls,
            pending: true,
            source: 1,
            loaded: vec![None, None, None],
            loading: None,
            switch: None,
            quad,
            targets: None,
        };
        let data = load(1).await?;
        demo.install(r, s, c, 1, Some(data))?;
        Ok(demo)
    }
    /// new GaussianSplat( data ), the source's rotation and fitCameraToSplats.
    fn install(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        source: usize,
        data: Option<SplatData>,
    ) -> Result<()> {
        if let Some(data) = data {
            self.loaded[source] = Some(Self::upload(r, source, &data)?);
        }
        self.source = source;
        let splats = self.loaded[source]
            .as_ref()
            .ok_or(Error::Invalid("splats"))?;
        let (scale, _, _) = splats.model.to_scale_rotation_translation();
        let sphere_center = splats.model.transform_point3(splats.fit.center);
        let radius = (splats.fit.radius * scale.abs().max_element()).max(0.01);
        let fov = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.fov,
            _ => 50.,
        };
        let distance = radius / (fov.to_radians() * 0.5).sin();
        self.controls.set_distances(radius * 1.1, radius * 8.);
        self.controls.set_target(sphere_center);
        let direction = Vector3::from_array(SOURCES[source].2).normalize();
        s.get_mut(c)?.position = direction * (distance * 1.15) + sphere_center;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = (distance / 1000.).max(0.001);
            p.far = (distance * 10.).max(10.);
        }
        self.controls.update(s, c)?;
        // A new GaussianSplat sorts and recomputes its colors on first use.
        if let Some(splats) = &mut self.loaded[source] {
            splats.sorted = None;
            splats.harmonics_at = None;
        }
        self.pending = true;
        Ok(())
    }
    fn upload(r: &Renderer, source: usize, data: &SplatData) -> Result<Splats> {
        let count = data.centers.len() / 3;
        let degree = data.bands.len();
        let storage = wgpu::BufferUsages::STORAGE;
        let init = |label, bytes: &[u8]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytes,
                    usage: storage,
                })
        };
        let mut centers = vec![0f32; count * 4];
        let mut cov_a = vec![0f32; count * 4];
        let mut cov_b = vec![0f32; count * 4];
        let mut colors = vec![0u32; count];
        for i in 0..count {
            centers[i * 4..i * 4 + 3].copy_from_slice(&data.centers[i * 3..i * 3 + 3]);
            cov_a[i * 4..i * 4 + 4].copy_from_slice(&data.covariances[i * 6..i * 6 + 4]);
            cov_b[i * 4..i * 4 + 2].copy_from_slice(&data.covariances[i * 6 + 4..i * 6 + 6]);
            let c = &data.colors[i * 4..i * 4 + 4];
            colors[i] = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
        }
        let centers = init("splat centers", bytemuck::cast_slice(&centers));
        let cov_a = init("splat covariance a", bytemuck::cast_slice(&cov_a));
        let cov_b = init("splat covariance b", bytemuck::cast_slice(&cov_b));
        let colors = init("splat colors", bytemuck::cast_slice(&colors));
        let bands: Vec<wgpu::Buffer> = data
            .bands
            .iter()
            .map(|b| init("splat harmonics", bytemuck::cast_slice(b)))
            .collect();
        let order_data: Vec<u32> = (0..count as u32).collect();
        let order = init("splat order", bytemuck::cast_slice(&order_data));
        let bins = init("splat bins", bytemuck::cast_slice(&vec![0u32; count]));
        let histogram = init(
            "splat histogram",
            bytemuck::cast_slice(&vec![0u32; BIN_COUNT as usize]),
        );
        let offsets = init(
            "splat offsets",
            bytemuck::cast_slice(&vec![0u32; BIN_COUNT as usize]),
        );
        let compute = |label: &str, source: &str| {
            let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            });
            r.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    module: &module,
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    cache: None,
                })
        };
        let sources = [
            wgsl!("reset_cs"),
            wgsl!("histogram_cs"),
            wgsl!("prefix_cs"),
            wgsl!("scatter_cs"),
        ];
        let sort_objects = [
            uniform(r, "splat reset", sources[0], "objectStruct")?,
            uniform(r, "splat histogram", sources[1], "objectStruct")?,
            uniform(r, "splat prefix", sources[2], "objectStruct")?,
            uniform(r, "splat scatter", sources[3], "objectStruct")?,
        ];
        let pipelines = sources.map(|s| compute("splat sort", s));
        let entries: [Vec<(u32, wgpu::BindingResource)>; 4] = [
            vec![
                (0, histogram.as_entire_binding()),
                (1, offsets.as_entire_binding()),
                (2, sort_objects[0].as_entire_binding()),
            ],
            vec![
                (0, centers.as_entire_binding()),
                (1, sort_objects[1].as_entire_binding()),
                (2, bins.as_entire_binding()),
                (3, histogram.as_entire_binding()),
            ],
            vec![
                (0, histogram.as_entire_binding()),
                (1, offsets.as_entire_binding()),
                (2, sort_objects[2].as_entire_binding()),
            ],
            vec![
                (0, bins.as_entire_binding()),
                (1, offsets.as_entire_binding()),
                (2, order.as_entire_binding()),
                (3, sort_objects[3].as_entire_binding()),
            ],
        ];
        let mut k = 0;
        let sort = pipelines.map(|p| {
            let g = bind(r, p.get_bind_group_layout(0), &entries[k]);
            k += 1;
            (p, g)
        });
        // The SH kernel's per-splat contribution ( degree 2 for the lion,
        // 3 for the tomatoes; none for the millipede ).
        let contribution = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("splat harmonics contribution"),
            size: (count.max(1) * 16) as u64,
            usage: storage,
            mapped_at_creation: false,
        });
        let harmonics = if degree > 0 {
            let src = if degree == 2 {
                wgsl!("sh2_cs")
            } else {
                wgsl!("sh3_cs")
            };
            let p = compute("splat harmonics", src);
            let object = uniform(r, "splat harmonics", src, "objectStruct")?;
            let mut e = vec![
                (0, centers.as_entire_binding()),
                (1, object.as_entire_binding()),
            ];
            for (i, b) in bands.iter().enumerate() {
                e.push((2 + i as u32, b.as_entire_binding()));
            }
            e.push((2 + bands.len() as u32, contribution.as_entire_binding()));
            let g = bind(r, p.get_bind_group_layout(0), &e);
            Some((p, g, object))
        } else {
            None
        };
        let (vs, fs) = if degree > 0 {
            (wgsl!("splat_vs"), wgsl!("splat_fs"))
        } else {
            (wgsl!("plain_vs"), wgsl!("plain_fs"))
        };
        let attrs = [wgpu::vertex_attr_array![0 => Float32x3]];
        let layout = [wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs[0],
        }];
        let pipeline = sampled_pipeline(
            r,
            "gaussian splat",
            (vs, fs),
            &layout,
            &[wgpu::TextureFormat::Rgba8Unorm],
            Some((wgpu::CompareFunction::LessEqual, false)),
            (false, true),
            (1, wgpu::PrimitiveTopology::TriangleList),
        );
        let render = uniform(r, "splat render", vs, "renderStruct")?;
        let object = uniform(r, "splat object", vs, "objectStruct")?;
        let mut e = vec![
            (0, object.as_entire_binding()),
            (1, order.as_entire_binding()),
            (2, centers.as_entire_binding()),
            (3, cov_a.as_entire_binding()),
            (4, cov_b.as_entire_binding()),
            (5, colors.as_entire_binding()),
        ];
        if degree > 0 {
            e.push((6, contribution.as_entire_binding()));
        }
        let draw = (
            pipeline.clone(),
            vec![
                bind(
                    r,
                    pipeline.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                ),
                bind(r, pipeline.get_bind_group_layout(1), &e),
            ],
        );
        let [rx, ry, rz] = SOURCES[source].1;
        let model = Matrix4::from_quat(Quaternion::from_euler(glam::EulerRot::XYZ, rx, ry, rz));
        Ok(Splats {
            count: count as u32,
            degree,
            model,
            fit: center_sphere(&data.centers),
            sort_sphere: kernel_sphere(&data.centers, &data.covariances),
            sort,
            sort_objects,
            harmonics,
            draw,
            render,
            object,
            vs,
            fs,
            sorted: None,
            harmonics_at: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets, s: &Splats, draw: bool) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("gaussian splat"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &t.screen.view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    // scene.background 0x07080f in the sRGB working space.
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 7. / 255.,
                        g: 8. / 255.,
                        b: 15. / 255.,
                        a: 1.,
                    }),
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
        if draw {
            set(&mut pass, &s.draw);
            pass.set_vertex_buffer(0, self.quad.0.slice(..));
            pass.set_index_buffer(self.quad.1.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..6, 0, 0..s.count);
        }
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        // A source loaded before is still resident.
        if let Some(source) = self.switch.take() {
            self.install(r, s, c, source, None)?;
        }
        // A finished load replaces the splats ( loadSplatSource ).
        if let Some((source, slot)) = &self.loading
            && slot.borrow().is_some()
        {
            let source = *source;
            let result = slot
                .borrow_mut()
                .take()
                .ok_or(Error::Invalid("splat load"))?;
            self.loading = None;
            self.install(r, s, c, source, Some(result?))?;
        }
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width || t.height != out.height || t.format != out.options.format
        }) {
            let depth = r
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("splat depth"),
                    size: wgpu::Extent3d {
                        width: out.width,
                        height: out.height,
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
            self.targets = Some(Targets {
                width: out.width,
                height: out.height,
                format: out.options.format,
                depth,
                screen: RenderTarget::with_options(
                    &r.device,
                    out.width,
                    out.height,
                    RenderTargetOptions {
                        samples: 0,
                        depth_buffer: false,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        ..out.options.clone()
                    },
                )?,
            });
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("splat targets"))?;
        let splats = self.loaded[self.source]
            .as_mut()
            .ok_or(Error::Invalid("splats"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        if !std::mem::take(&mut self.pending) {
            let splats = self.loaded[self.source]
                .as_ref()
                .ok_or(Error::Invalid("splats"))?;
            self.present(&mut encoder, t, splats, true);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate(): the damped controls, the sort, then the render's
        // spherical-harmonics update.
        self.controls.frame_update(s, c)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let near = match camera {
            Camera::Perspective(p) => p.near,
            _ => 0.01,
        };
        let model_view = view * splats.model;
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
        let count = splats.count as f64;
        let e = model_view.to_cols_array();
        let direction = Vector3::new(e[2], e[6], e[10]).normalize();
        if splats
            .sorted
            .is_none_or(|last| direction.dot(last) < SORT_DIRECTION_THRESHOLD)
        {
            // _updateSortUniforms: the depth range of the kernel sphere.
            let center =
                view.transform_point3(splats.model.transform_point3(splats.sort_sphere.center));
            let (scale, _, _) = splats.model.to_scale_rotation_translation();
            let radius = splats.sort_sphere.radius * scale.abs().max_element();
            let depth = -center.z;
            let near_depth = near.max(depth - radius);
            let far_depth = (near_depth + 0.0001).max(depth + radius);
            write(
                &splats.sort_objects[0],
                wgsl!("reset_cs"),
                "objectStruct",
                &[("nodeUniform2", vec![BIN_COUNT as f64])],
            )?;
            write(
                &splats.sort_objects[1],
                wgsl!("histogram_cs"),
                "objectStruct",
                &[
                    ("nodeUniform1", m4(model_view)),
                    ("nodeUniform2", vec![near_depth, far_depth]),
                    ("nodeUniform5", vec![count]),
                ],
            )?;
            write(
                &splats.sort_objects[2],
                wgsl!("prefix_cs"),
                "objectStruct",
                &[("nodeUniform2", vec![1.])],
            )?;
            write(
                &splats.sort_objects[3],
                wgsl!("scatter_cs"),
                "objectStruct",
                &[("nodeUniform3", vec![count])],
            )?;
            let groups = [
                BIN_COUNT.div_ceil(WORKGROUP),
                splats.count.div_ceil(WORKGROUP),
                1,
                splats.count.div_ceil(WORKGROUP),
            ];
            let mut pass = encoder.begin_compute_pass(&Default::default());
            for ((p, g), n) in splats.sort.iter().zip(groups) {
                pass.set_pipeline(p);
                pass.set_bind_group(0, g, &[]);
                pass.dispatch_workgroups(n, 1, 1);
            }
            drop(pass);
            splats.sorted = Some(direction);
        }
        if let Some((p, g, object)) = &splats.harmonics
            && splats.harmonics_at != Some((world, splats.model))
        {
            let local = splats
                .model
                .inverse()
                .transform_point3(world.w_axis.truncate());
            let source = if splats.degree == 2 {
                wgsl!("sh2_cs")
            } else {
                wgsl!("sh3_cs")
            };
            let name = if splats.degree == 2 {
                "nodeUniform5"
            } else {
                "nodeUniform6"
            };
            write(
                object,
                source,
                "objectStruct",
                &[
                    ("nodeUniform1", local.to_array().to_vec()),
                    (name, vec![count]),
                ],
            )?;
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(p);
            pass.set_bind_group(0, g, &[]);
            pass.dispatch_workgroups(splats.count.div_ceil(WORKGROUP), 1, 1);
            drop(pass);
            splats.harmonics_at = Some((world, splats.model));
        }
        write(
            &splats.render,
            splats.vs,
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                (
                    if splats.degree > 0 {
                        "nodeUniform9"
                    } else {
                        "nodeUniform8"
                    },
                    vec![t.width as f64, t.height as f64],
                ),
            ],
        )?;
        let mv_name = if splats.degree > 0 {
            "nodeUniform7"
        } else {
            "nodeUniform6"
        };
        write(
            &splats.object,
            splats.fs,
            "objectStruct",
            &[("nodeUniform0", vec![1.]), (mv_name, m4(model_view))],
        )?;
        let splats = self.loaded[self.source]
            .as_ref()
            .ok_or(Error::Invalid("splats"))?;
        self.present(&mut encoder, t, splats, true);
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
    /// The splat source: millipede, lion or tomatoes.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("splat parameter"));
        }
        let source = (value.round() as usize).min(2);
        if self.loaded[source].is_some() {
            self.loading = None;
            self.switch = Some(source);
            return Ok(());
        }
        let slot: SplatLoad = Rc::default();
        let result = slot.clone();
        wasm_bindgen_futures::spawn_local(async move {
            *result.borrow_mut() = Some(load(source).await);
        });
        self.loading = Some((source, slot));
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
