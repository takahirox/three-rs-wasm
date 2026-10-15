//! TreeGenerator (r186 addons/generators): a deterministic, recursive sweep of
//! tapered tubes baked into one indexed geometry of positions and normals. The
//! arithmetic follows the JavaScript in f64, with three.js's Vector3 and
//! Quaternion formulas, and stores Float32 vertices as the original does.

/// A Vector3 with three.js's operation order.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
const fn v3(x: f64, y: f64, z: f64) -> V3 {
    V3 { x, y, z }
}
impl V3 {
    fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    fn scale(self, s: f64) -> V3 {
        v3(self.x * s, self.y * s, self.z * s)
    }
    /// divideScalar( length || 1 ): multiplyScalar( 1 / length ).
    fn normalize(self) -> V3 {
        let l = self.length();
        self.scale(1. / if l == 0. { 1. } else { l })
    }
    fn dot(self, v: V3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    fn cross(a: V3, b: V3) -> V3 {
        v3(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
    fn add_scaled(self, v: V3, s: f64) -> V3 {
        v3(self.x + v.x * s, self.y + v.y * s, self.z + v.z * s)
    }
    fn lerp(self, v: V3, alpha: f64) -> V3 {
        v3(
            self.x + (v.x - self.x) * alpha,
            self.y + (v.y - self.y) * alpha,
            self.z + (v.z - self.z) * alpha,
        )
    }
    /// applyAxisAngle: Quaternion.setFromAxisAngle, then applyQuaternion.
    fn apply_axis_angle(self, axis: V3, angle: f64) -> V3 {
        let half = angle / 2.;
        let s = half.sin();
        let (qx, qy, qz, qw) = (axis.x * s, axis.y * s, axis.z * s, half.cos());
        let (vx, vy, vz) = (self.x, self.y, self.z);
        let tx = 2. * (qy * vz - qz * vy);
        let ty = 2. * (qz * vx - qx * vz);
        let tz = 2. * (qx * vy - qy * vx);
        v3(
            vx + qw * tx + qy * tz - qz * ty,
            vy + qw * ty + qz * tx - qx * tz,
            vz + qw * tz + qx * ty - qy * tx,
        )
    }
}
const UP: V3 = v3(0., 1., 0.);
/// The generator's parameters ( defaults overridden by the page's setters ).
#[derive(Clone)]
pub(super) struct Params {
    pub seed: u32,
    pub levels: usize,
    pub children: Vec<usize>,
    pub branch_angle: Vec<f64>,
    pub angle_variance: f64,
    pub length_ratio: f64,
    pub length_variance: f64,
    pub branch_length_falloff: f64,
    pub trunk_length: f64,
    pub trunk_radius: f64,
    pub taper: f64,
    pub taper_curve: f64,
    pub root_flare: f64,
    pub flare_frac: f64,
    pub radius_exponent: f64,
    pub min_radius: f64,
    pub min_length: f64,
    pub droop: f64,
    pub up_pull: f64,
    pub gnarl: Vec<f64>,
    pub radial_segments: usize,
    pub section_length: f64,
    pub child_start: f64,
    pub trunk_clear: f64,
}
/// createRandom( seed ): mulberry32.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6D2B79F5);
        let s = self.0;
        let mut t = (s ^ (s >> 15)).wrapping_mul(1 | s);
        t = (t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t))) ^ t;
        (t ^ (t >> 14)) as f64 / 4294967296.
    }
}
#[derive(Clone, Copy)]
struct Ring {
    pos: V3,
    tangent: V3,
    normal: V3,
    radius: f64,
}
struct Tube {
    rings: Vec<Ring>,
    radial: usize,
}
fn perpendicular(v: V3) -> V3 {
    let a = if v.x.abs() < 0.9 {
        v3(1., 0., 0.)
    } else {
        v3(0., 1., 0.)
    };
    V3::cross(v, a).normalize()
}
/// Rotates `n` by the rotation mapping tangent `t0` onto `t1`.
fn transport(t0: V3, t1: V3, n: V3) -> V3 {
    let axis = V3::cross(t0, t1);
    let sin = axis.length();
    if sin < 1e-6 {
        return n;
    }
    let axis = axis.scale(1. / sin);
    n.apply_axis_angle(axis, sin.atan2(t0.dot(t1)))
}
fn ring_at(rings: &[Ring], t: f64) -> Ring {
    let f = t.clamp(0., 0.999) * (rings.len() - 1) as f64;
    let i = f.floor() as usize;
    let frac = f - i as f64;
    let (a, b) = (rings[i], rings[i + 1]);
    Ring {
        pos: a.pos.lerp(b.pos, frac),
        tangent: a.tangent.lerp(b.tangent, frac).normalize(),
        normal: a.normal.lerp(b.normal, frac).normalize(),
        radius: a.radius + (b.radius - a.radius) * frac,
    }
}
#[allow(clippy::too_many_arguments)]
fn grow(
    tubes: &mut Vec<Tube>,
    base: V3,
    dir: V3,
    length: f64,
    base_radius: f64,
    level: usize,
    p: &Params,
    random: &mut Random,
) {
    let sections = ((length / p.section_length).round() as i64).clamp(3, 24) as usize;
    let radial = p.radial_segments.saturating_sub(level).max(3);
    let step = length / sections as f64;
    let gnarl = p.gnarl[level.min(p.gnarl.len() - 1)];
    let start = if level == 0 {
        p.trunk_clear
    } else {
        p.child_start
    };
    let mut tangent = dir.normalize();
    let mut normal = perpendicular(tangent);
    let mut rings = Vec::with_capacity(sections + 1);
    let mut pos = base;
    for s in 0..=sections {
        let t = s as f64 / sections as f64;
        let mut radius = base_radius * ((1. - p.taper) + p.taper * (1. - t).powf(p.taper_curve));
        if level == 0 && p.root_flare > 0. {
            let flare = ((p.flare_frac - t) / p.flare_frac).max(0.);
            radius *= 1. + p.root_flare * flare * flare * flare;
        }
        rings.push(Ring {
            pos,
            tangent,
            normal,
            radius,
        });
        if s < sections {
            let mut next = tangent;
            next.x += (random.next() * 2. - 1.) * gnarl;
            next.y += (random.next() * 2. - 1.) * gnarl;
            next.z += (random.next() * 2. - 1.) * gnarl;
            if level > 0 {
                next.y -= p.droop * step;
            }
            let next = next.normalize();
            normal = transport(tangent, next, normal);
            pos = pos.add_scaled(next, step);
            tangent = next;
        }
    }
    tubes.push(Tube {
        rings: rings.clone(),
        radial,
    });
    if level + 1 >= p.levels || length < p.min_length {
        return;
    }
    let n = p.children[level.min(p.children.len() - 1)];
    let angle = p.branch_angle[level.min(p.branch_angle.len() - 1)];
    let pipe_drop = (1. / n as f64).powf(1. / p.radius_exponent);
    let golden = std::f64::consts::PI * (3. - 5f64.sqrt());
    for i in 0..n {
        let t = start + (i as f64 + 0.5 + (random.next() - 0.5) * 0.6) / n as f64 * (1. - start);
        let ring = ring_at(&rings, t);
        let tilt =
            (angle + (random.next() * 2. - 1.) * p.angle_variance) * (std::f64::consts::PI / 180.);
        let roll = i as f64 * golden + (random.next() * 2. - 1.) * 0.4;
        let mut child = ring
            .tangent
            .apply_axis_angle(ring.normal, tilt)
            .apply_axis_angle(ring.tangent, roll);
        if p.up_pull > 0. {
            child = child.lerp(UP, p.up_pull).normalize();
        }
        let child_base = p.min_radius.max((base_radius * pipe_drop).min(ring.radius));
        let mut child_length = length * p.length_ratio * (1. - p.branch_length_falloff * t);
        if p.length_variance > 0. {
            child_length *= 1. + (random.next() * 2. - 1.) * p.length_variance;
        }
        grow(
            tubes,
            ring.pos,
            child,
            child_length,
            child_base,
            level + 1,
            p,
            random,
        );
    }
}
/// The baked geometry: Float32 positions and normals and the index.
pub(super) struct TreeGeometry {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub index: Vec<u32>,
}
pub(super) fn build(p: &Params) -> TreeGeometry {
    let mut random = Random(if p.seed == 0 { 1 } else { p.seed });
    let mut tubes = vec![];
    grow(
        &mut tubes,
        V3::default(),
        UP,
        p.trunk_length,
        p.trunk_radius,
        0,
        p,
        &mut random,
    );
    let (mut positions, mut normals, mut index) = (vec![], vec![], vec![]);
    let tau = std::f64::consts::PI * 2.;
    let mut vertex_offset = 0u32;
    for tube in &tubes {
        let radial = tube.radial;
        for ring in &tube.rings {
            let binormal = V3::cross(ring.tangent, ring.normal);
            for j in 0..radial {
                let angle = j as f64 / radial as f64 * tau;
                let (c, s) = (angle.cos(), angle.sin());
                let nx = c * ring.normal.x + s * binormal.x;
                let ny = c * ring.normal.y + s * binormal.y;
                let nz = c * ring.normal.z + s * binormal.z;
                positions.extend([
                    (ring.pos.x + nx * ring.radius) as f32,
                    (ring.pos.y + ny * ring.radius) as f32,
                    (ring.pos.z + nz * ring.radius) as f32,
                ]);
                normals.extend([nx as f32, ny as f32, nz as f32]);
            }
        }
        let radial = radial as u32;
        for r in 0..tube.rings.len() as u32 - 1 {
            let a = vertex_offset + r * radial;
            let b = a + radial;
            for j in 0..radial {
                let next = (j + 1) % radial;
                index.extend([a + j, b + next, b + j, a + j, a + next, b + next]);
            }
        }
        vertex_offset += tube.rings.len() as u32 * radial;
    }
    TreeGeometry {
        positions,
        normals,
        index,
    }
}
