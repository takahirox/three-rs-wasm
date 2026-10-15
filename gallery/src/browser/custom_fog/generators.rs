//! TerrainGenerator and ForestGenerator ( three.js r186 addons ): the baked
//! height grid with its thermal erosion and the scattered tree blobs, in
//! f64 with the generators' Float32Array stores and three.js's Object3D
//! matrix composition.
use super::super::trackball_sprites::noise;

/// The page's TerrainGenerator parameters over the defaults.
#[derive(Clone, Copy)]
pub(super) struct TerrainParams {
    pub(super) seed: u32,
    pub(super) size: f64,
    pub(super) segments: usize,
    pub(super) height_scale: f64,
    pub(super) frequency: f64,
    pub(super) octaves: usize,
    pub(super) lacunarity: f64,
    pub(super) gain: f64,
    pub(super) erosion: f64,
    pub(super) warp: f64,
    pub(super) valley_bias: f64,
    pub(super) sea_level: f64,
    pub(super) talus: f64,
    pub(super) talus_passes: usize,
}
impl TerrainParams {
    /// new TerrainGenerator( { seed: 1, size: 900, segments: 512,
    /// frequency: 0.0065, heightScale: 150, erosion: 0.7, valleyBias: 1.2 } ).
    pub(super) fn page() -> Self {
        Self {
            seed: 1,
            size: 900.,
            segments: 512,
            height_scale: 150.,
            frequency: 0.0065,
            octaves: 5,
            lacunarity: 1.97,
            gain: 0.5,
            erosion: 0.7,
            warp: 0.35,
            valley_bias: 1.2,
            sea_level: 0.15,
            talus: 1.,
            talus_passes: 12,
        }
    }
}
/// createRandom( seed ): mulberry32.
pub(super) fn create_random(seed: u32) -> impl FnMut() -> f64 {
    let mut s = if seed == 0 { 1 } else { seed };
    move || {
        s = s.wrapping_add(0x6D2B79F5);
        let mut t = (s ^ (s >> 15)).wrapping_mul(1 | s);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        (t ^ (t >> 14)) as f64 / 4294967296.
    }
}
/// V8's Math.hypot: scaled by the largest magnitude, Kahan-summed.
pub(super) fn hypot(values: &[f64]) -> f64 {
    let max = values.iter().fold(0f64, |m, v| m.max(v.abs()));
    if max == 0. {
        return 0.;
    }
    let (mut sum, mut compensation) = (0f64, 0f64);
    for v in values {
        let n = v.abs() / max;
        let summand = n * n - compensation;
        let preliminary = sum + summand;
        compensation = (preliminary - sum) - summand;
        sum = preliminary;
    }
    sum.sqrt() * max
}
/// The baked terrain: the height grid and the mesh attributes.
pub(super) struct Terrain {
    pub(super) params: TerrainParams,
    pub(super) heights: Vec<f32>,
    pub(super) grid: usize,
    pub(super) min_y: f32,
    pub(super) max_y: f32,
    pub(super) positions: Vec<f32>,
    pub(super) normals: Vec<f32>,
    pub(super) index: Vec<u32>,
}
/// heightField( p ): the warped, derivative-damped fractal.
fn height_field(p: &TerrainParams) -> impl Fn(f64, f64) -> f64 {
    let mut random = create_random(p.seed);
    let offset_x = random() * 256.;
    let offset_z = random() * 256.;
    let slice = random() * 256.;
    let p = *p;
    let warp_field = move |x: f64, z: f64, zr: f64| {
        let (mut freq, mut amp, mut sum, mut norm) = (1., 1., 0., 0.);
        for i in 0..2 {
            sum += amp
                * noise(
                    x * freq + offset_x,
                    z * freq + offset_z,
                    zr + i as f64 * 1.7,
                );
            norm += amp;
            freq *= p.lacunarity;
            amp *= p.gain;
        }
        sum / norm
    };
    let eroded = move |x: f64, z: f64| {
        let (mut sum, mut amp, mut dx, mut dz, mut px, mut pz, mut freq) =
            (0., 1., 0., 0., x, z, 1.);
        let e = 0.004;
        for i in 0..p.octaves {
            let zr = slice + i as f64 * 1.7;
            let bx = px * freq + offset_x;
            let bz = pz * freq + offset_z;
            let n = noise(bx, bz, zr);
            let nx = noise(bx + e, bz, zr);
            let nz = noise(bx, bz + e, zr);
            dx += (nx - n) / e * freq;
            dz += (nz - n) / e * freq;
            sum += amp * n / (1. + p.erosion * (dx * dx + dz * dz));
            let rx = 0.8 * px - 0.6 * pz;
            pz = 0.6 * px + 0.8 * pz;
            px = rx;
            freq *= p.lacunarity;
            amp *= p.gain;
        }
        sum * 0.5 + 0.5
    };
    move |world_x: f64, world_z: f64| {
        let x = world_x * p.frequency;
        let z = world_z * p.frequency;
        let wx = x + p.warp * warp_field(x + 1.3, z + 7.2, slice + 40.);
        let wz = z + p.warp * warp_field(x + 5.2, z + 1.3, slice + 70.);
        let h = (eroded(wx, wz) * 1.1).min(1.).powf(p.valley_bias);
        (h - p.sea_level) * p.height_scale
    }
}
/// thermalErode( h, N, cellSize, talus, passes ).
fn thermal_erode(h: &mut [f32], n: usize, cell_size: f64, talus: f64, passes: usize) {
    let drop = talus * cell_size;
    let carry = 0.5;
    let mut delta = vec![0f32; n * n];
    let off = [-1isize, 1, -(n as isize), n as isize];
    for _ in 0..passes {
        delta.fill(0.);
        for z in 0..n {
            for x in 0..n {
                let i = z * n + x;
                let hi = h[i] as f64;
                let mut ex = [
                    if x > 0 {
                        hi - h[i - 1] as f64 - drop
                    } else {
                        0.
                    },
                    if x < n - 1 {
                        hi - h[i + 1] as f64 - drop
                    } else {
                        0.
                    },
                    if z > 0 {
                        hi - h[i - n] as f64 - drop
                    } else {
                        0.
                    },
                    if z < n - 1 {
                        hi - h[i + n] as f64 - drop
                    } else {
                        0.
                    },
                ];
                let (mut sum, mut peak) = (0., 0f64);
                for d in &mut ex {
                    if *d <= 0. {
                        *d = 0.;
                        continue;
                    }
                    sum += *d;
                    if *d > peak {
                        peak = *d;
                    }
                }
                if sum <= 0. {
                    continue;
                }
                let movement = carry * peak;
                delta[i] = (delta[i] as f64 - movement) as f32;
                for k in 0..4 {
                    if ex[k] > 0. {
                        let j = (i as isize + off[k]) as usize;
                        delta[j] = (delta[j] as f64 + movement * ex[k] / sum) as f32;
                    }
                }
            }
        }
        for k in 0..n * n {
            h[k] = (h[k] as f64 + delta[k] as f64) as f32;
        }
    }
}
impl Terrain {
    /// TerrainGenerator.build().
    pub(super) fn build(p: TerrainParams) -> Self {
        let n = p.segments + 1;
        let half = p.size / 2.;
        let coord: Vec<f64> = (0..n)
            .map(|i| i as f64 / p.segments as f64 * p.size - half)
            .collect();
        let height = height_field(&p);
        let mut heights = vec![0f32; n * n];
        for iz in 0..n {
            for ix in 0..n {
                heights[iz * n + ix] = height(coord[ix], coord[iz]) as f32;
            }
        }
        if p.talus_passes > 0 {
            thermal_erode(
                &mut heights,
                n,
                p.size / p.segments as f64,
                p.talus,
                p.talus_passes,
            );
        }
        let mut positions = vec![0f32; n * n * 3];
        let mut normals = vec![0f32; n * n * 3];
        let cell_size = p.size / p.segments as f64;
        let (mut min, mut max) = (f32::INFINITY, f32::NEG_INFINITY);
        for iz in 0..n {
            for ix in 0..n {
                let o = iz * n + ix;
                let y = heights[o];
                positions[o * 3] = coord[ix] as f32;
                positions[o * 3 + 1] = y;
                positions[o * 3 + 2] = coord[iz] as f32;
                let left = ix.saturating_sub(1);
                let right = (ix + 1).min(n - 1);
                let back = iz.saturating_sub(1);
                let front = (iz + 1).min(n - 1);
                let nx = (heights[iz * n + left] as f64 - heights[iz * n + right] as f64)
                    / ((right - left) as f64 * cell_size);
                let nz = (heights[back * n + ix] as f64 - heights[front * n + ix] as f64)
                    / ((front - back) as f64 * cell_size);
                let length = hypot(&[nx, 1., nz]);
                normals[o * 3] = (nx / length) as f32;
                normals[o * 3 + 1] = (1. / length) as f32;
                normals[o * 3 + 2] = (nz / length) as f32;
                if y < min {
                    min = y;
                }
                if y > max {
                    max = y;
                }
            }
        }
        let mut index = Vec::with_capacity(p.segments * p.segments * 6);
        for iz in 0..p.segments {
            for ix in 0..p.segments {
                let a = (iz * n + ix) as u32;
                let (b, c) = (a + 1, a + n as u32);
                let d = c + 1;
                let even = (ix + iz) % 2 == 0;
                index.extend([
                    a,
                    c,
                    if even { b } else { d },
                    if even { b } else { a },
                    if even { c } else { d },
                    if even { d } else { b },
                ]);
            }
        }
        Self {
            params: p,
            heights,
            grid: n,
            min_y: min,
            max_y: max,
            positions,
            normals,
            index,
        }
    }
    /// sampleHeight( x, z ): bilinear over the baked grid.
    pub(super) fn sample_height(&self, x: f64, z: f64) -> f64 {
        let p = &self.params;
        let n = self.grid;
        let half = p.size / 2.;
        let segments = p.segments as f64;
        let fx = ((x + half) / p.size * segments).min(segments).max(0.);
        let fz = ((z + half) / p.size * segments).min(segments).max(0.);
        let ix = ((n - 2) as f64).min(fx.floor()) as usize;
        let iz = ((n - 2) as f64).min(fz.floor()) as usize;
        let tx = fx - ix as f64;
        let tz = fz - iz as f64;
        let h = |i: usize| self.heights[i] as f64;
        let (h00, h10) = (h(iz * n + ix), h(iz * n + ix + 1));
        let (h01, h11) = (h((iz + 1) * n + ix), h((iz + 1) * n + ix + 1));
        (h00 * (1. - tx) + h10 * tx) * (1. - tz) + (h01 * (1. - tx) + h11 * tx) * tz
    }
    /// sampleSlope( x, z ): the surface normal's y.
    pub(super) fn sample_slope(&self, x: f64, z: f64) -> f64 {
        let e = self.params.size / self.params.segments as f64;
        let hx = self.sample_height(x + e, z) - self.sample_height(x - e, z);
        let hz = self.sample_height(x, z + e) - self.sample_height(x, z - e);
        2. * e / (hx * hx + 4. * e * e + hz * hz).sqrt()
    }
}
/// ForestGenerator.defaults with the page's count and castShadow.
const COUNT: usize = 500_000;
const RADIUS: f64 = 1.3;
const HEIGHT: f64 = 4.;
const DISTORTION: f64 = 0.5;
const SINK: f64 = 0.4;
const ALTITUDE_MIN: f64 = 0.12;
const ALTITUDE_MAX: f64 = 0.46;
const MIN_SLOPE: f64 = 0.55;
const DENSITY_FREQUENCY: f64 = 0.012;
const MIN_SCALE: f64 = 0.7;
const MAX_SCALE: f64 = 1.8;
/// The forest: the blob geometry and the per-instance attributes.
pub(super) struct Forest {
    pub(super) positions: Vec<f32>,
    pub(super) normals: Vec<f32>,
    pub(super) ao: Vec<f32>,
    pub(super) index: Vec<u32>,
    /// Per instance: the matrix ( 16 ), the cull data ( 4 ) and the region.
    pub(super) matrices: Vec<f32>,
    pub(super) cull: Vec<f32>,
    pub(super) region: Vec<f32>,
    pub(super) count: usize,
}
// The JS order of min and max, which differs from clamp for NaN.
#[allow(clippy::manual_clamp)]
fn smooth_blend(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).min(1.).max(0.);
    t * t * (3. - 2. * t)
}
fn blob_noise(x: f64, y: f64, z: f64) -> f64 {
    (x * 3.1).sin() * (y * 2.7 + 1.3).sin() * (z * 3.5 + 2.1).sin()
}
/// IcosahedronGeometry( 1, 0 )'s 60 positions ( PolyhedronGeometry's face
/// order, normalized ), as stored.
fn icosahedron() -> Vec<f32> {
    let t = (1. + 5f64.sqrt()) / 2.;
    let v: [[f64; 3]; 12] = [
        [-1., t, 0.],
        [1., t, 0.],
        [-1., -t, 0.],
        [1., -t, 0.],
        [0., -1., t],
        [0., 1., t],
        [0., -1., -t],
        [0., 1., -t],
        [t, 0., -1.],
        [t, 0., 1.],
        [-t, 0., -1.],
        [-t, 0., 1.],
    ];
    let faces = [
        0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, 1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7,
        1, 8, 3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, 4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9,
        8, 1,
    ];
    let lerp = |a: [f64; 3], b: [f64; 3], t: f64| {
        [
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
        ]
    };
    let mut out = vec![];
    for f in faces.chunks(3) {
        let (a, b, c) = (v[f[0]], v[f[1]], v[f[2]]);
        // subdivideFace at detail 0: v[0][1] = a.lerp( b, 1 ), v[1][0] = a.lerp( c, 1 ).
        for p in [lerp(a, b, 1.), lerp(a, c, 1.), a] {
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            let inv = 1. / length;
            out.extend(p.map(|x| (x * inv) as f32));
        }
    }
    out
}
/// Object3D.updateMatrix(): compose( position, Euler XYZ, scale ).
fn compose(p: [f64; 3], r: [f64; 3], s: [f64; 3]) -> [f64; 16] {
    let (c1, c2, c3) = ((r[0] / 2.).cos(), (r[1] / 2.).cos(), (r[2] / 2.).cos());
    let (s1, s2, s3) = ((r[0] / 2.).sin(), (r[1] / 2.).sin(), (r[2] / 2.).sin());
    let x = s1 * c2 * c3 + c1 * s2 * s3;
    let y = c1 * s2 * c3 - s1 * c2 * s3;
    let z = c1 * c2 * s3 + s1 * s2 * c3;
    let w = c1 * c2 * c3 - s1 * s2 * s3;
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    let [sx, sy, sz] = s;
    [
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
        p[0],
        p[1],
        p[2],
        1.,
    ]
}
impl Forest {
    /// ForestGenerator.build( terrain ).
    pub(super) fn build(terrain: &Terrain) -> Self {
        // blobGeometry: the icosahedron welded by position ( mergeVertices ).
        let raw = icosahedron();
        let key = |v: f32| (v as f64 * 10000. + 0.5).trunc() as i64;
        let mut keys: Vec<[i64; 3]> = vec![];
        let mut welded: Vec<f32> = vec![];
        let mut index = vec![];
        for p in raw.chunks(3) {
            let k = [key(p[0]), key(p[1]), key(p[2])];
            let id = match keys.iter().position(|q| *q == k) {
                Some(i) => i,
                None => {
                    keys.push(k);
                    welded.extend_from_slice(p);
                    keys.len() - 1
                }
            };
            index.push(id as u32);
        }
        let count = welded.len() / 3;
        let mut positions = vec![0f32; count * 3];
        let mut normals = vec![0f32; count * 3];
        let mut ao = vec![0f32; count];
        for i in 0..count {
            let (ux, uy, uz) = (
                welded[i * 3] as f64,
                welded[i * 3 + 1] as f64,
                welded[i * 3 + 2] as f64,
            );
            let h = (uy + 1.) / 2.;
            let taper = 1. - 0.62 * h;
            let lump = 1. + DISTORTION * blob_noise(ux, uy, uz);
            let r = taper * lump;
            positions[i * 3] = (ux * r * RADIUS) as f32;
            positions[i * 3 + 1] = (h * HEIGHT) as f32;
            positions[i * 3 + 2] = (uz * r * RADIUS) as f32;
            let inv = 1. / hypot(&[ux, 0.55, uz]);
            normals[i * 3] = (ux * inv) as f32;
            normals[i * 3 + 1] = (0.55 * inv) as f32;
            normals[i * 3 + 2] = (uz * inv) as f32;
            ao[i] = h as f32;
        }
        let size = terrain.params.size;
        let min_y = terrain.min_y as f64;
        let span = terrain.max_y as f64 - min_y;
        let mut random = create_random(1);
        let d_off_x = random() * 256.;
        let d_off_z = random() * 256.;
        let d_slice = random() * 256.;
        let density_at = |x: f64, z: f64| {
            smooth_blend(
                -0.12,
                0.22,
                noise(
                    x * DENSITY_FREQUENCY + d_off_x,
                    z * DENSITY_FREQUENCY + d_off_z,
                    d_slice,
                ),
            )
        };
        let mut cull = vec![0f32; COUNT * 4];
        let mut cull_random = create_random(1 ^ 0x9e3779b9);
        let mut region = vec![0f32; COUNT];
        let r_off_x = cull_random() * 256.;
        let r_off_z = cull_random() * 256.;
        let r_slice = cull_random() * 256.;
        let mut matrices = vec![0f32; COUNT * 16];
        let (mut placed, mut attempts) = (0, 0);
        let max_attempts = COUNT * 14;
        while placed < COUNT && attempts < max_attempts {
            attempts += 1;
            let x = (random() - 0.5) * size;
            let z = (random() - 0.5) * size;
            let y = terrain.sample_height(x, z);
            let altitude = (y - min_y) / span;
            if !(ALTITUDE_MIN..=ALTITUDE_MAX).contains(&altitude) {
                continue;
            }
            if terrain.sample_slope(x, z) < MIN_SLOPE {
                continue;
            }
            let mut density = density_at(x, z);
            density *= smooth_blend(ALTITUDE_MAX, ALTITUDE_MAX - 0.14, altitude);
            if random() >= density {
                continue;
            }
            let position = [x, y - SINK, z];
            let rx = (random() - 0.5) * 0.12;
            let ry = random() * std::f64::consts::PI * 2.;
            let rz = (random() - 0.5) * 0.12;
            let s = MIN_SCALE + random() * random() * (MAX_SCALE - MIN_SCALE);
            let sx = s * (0.85 + random() * 0.3);
            let sz = s * (0.85 + random() * 0.3);
            let m = compose(position, [rx, ry, rz], [sx, s, sz]);
            for (k, v) in m.iter().enumerate() {
                matrices[placed * 16 + k] = *v as f32;
            }
            let c = placed * 4;
            cull[c] = x as f32;
            cull[c + 1] = position[1] as f32;
            cull[c + 2] = z as f32;
            cull[c + 3] = cull_random() as f32;
            #[allow(clippy::manual_clamp)]
            let value = (noise(x * 0.02 + r_off_x, z * 0.02 + r_off_z, r_slice) * 0.6 + 0.5)
                .max(0.)
                .min(1.);
            region[placed] = value as f32;
            placed += 1;
        }
        Self {
            positions,
            normals,
            ao,
            index,
            matrices,
            cull,
            region,
            count: placed,
        }
    }
}
