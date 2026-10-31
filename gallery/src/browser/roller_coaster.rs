//! three r187's `examples/jsm/misc/RollerCoaster.js`: the track, its lifters
//! and shadow, the sky's quads and the landscape's trees. The geometry is
//! built once on the CPU, as the addon builds it.
use super::xr_random::random;
use crate::{
    Result, attribute::BufferAttribute, geometry::Attribute, geometry::BufferGeometry, math::*,
    raycast::Raycaster, scene::*,
};
use std::f64::consts::PI;

/// The example's curve: `getPointAt` and `getTangentAt` over t in [ 0, 1 ].
pub trait Curve {
    fn point_at(&self, t: f64) -> Vector3;
    fn tangent_at(&self, t: f64) -> Vector3;
}

fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}

fn push(out: &mut Vec<f32>, v: Vector3) {
    out.extend([v.x as f32, v.y as f32, v.z as f32]);
}

/// Quaternion.setFromAxisAngle.
fn axis_angle(axis: Vector3, angle: f64) -> Quaternion {
    let s = (angle / 2.).sin();
    Quaternion::from_xyzw(axis.x * s, axis.y * s, axis.z * s, (angle / 2.).cos())
}

/// Vector3.applyQuaternion.
fn rotate(v: Vector3, q: Quaternion) -> Vector3 {
    let (vx, vy, vz) = (v.x, v.y, v.z);
    let (qx, qy, qz, qw) = (q.x, q.y, q.z, q.w);
    let tx = 2. * (qy * vz - qz * vy);
    let ty = 2. * (qz * vx - qx * vz);
    let tz = 2. * (qx * vy - qy * vx);
    Vector3::new(
        vx + qw * tx + qy * tz - qz * ty,
        vy + qw * ty + qz * tx - qx * tz,
        vz + qw * tz + qx * ty - qy * tx,
    )
}

/// Quaternion.multiplyQuaternions( a, b ).
fn mul(a: Quaternion, b: Quaternion) -> Quaternion {
    let (qax, qay, qaz, qaw) = (a.x, a.y, a.z, a.w);
    let (qbx, qby, qbz, qbw) = (b.x, b.y, b.z, b.w);
    Quaternion::from_xyzw(
        qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
        qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
        qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
        qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
    )
}

/// The heading change across t ± 0.01, wrapped to ( -π, π ].
fn heading_change(curve: &dyn Curve, t: f64) -> f64 {
    let bank_delta = 0.01;
    let sample1 = curve.tangent_at(((t - bank_delta) % 1. + 1.) % 1.);
    let sample2 = curve.tangent_at((t + bank_delta) % 1.);
    let mut change = sample2.x.atan2(sample2.z) - sample1.x.atan2(sample1.z);
    if change > PI {
        change -= PI * 2.;
    }
    if change < -PI {
        change += PI * 2.;
    }
    change
}

/// RollerCoasterGeometry( curve, divisions ): positions, normals and colors.
pub fn roller_coaster(curve: &dyn Curve, divisions: u32) -> Result<BufferGeometry> {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();
    let color1 = [1., 1., 1.];
    let color2 = [1., 1., 0.];
    let mut up;
    let mut quaternion;
    let mut prev_quaternion = axis_angle(Vector3::Y, PI / 2.);
    let mut prev_point = curve.point_at(0.);
    let step = [
        Vector3::new(-0.225, 0., 0.),
        Vector3::new(0., -0.050, 0.),
        Vector3::new(0., -0.175, 0.),
        Vector3::new(0., -0.050, 0.),
        Vector3::new(0.225, 0., 0.),
        Vector3::new(0., -0.175, 0.),
    ];
    let pi2 = PI * 2.;
    let tube = |sides: u32, radius: f64| -> Vec<Vector3> {
        (0..sides)
            .map(|i| {
                let angle = (i as f64 / sides as f64) * pi2;
                Vector3::new(angle.sin() * radius, angle.cos() * radius, 0.)
            })
            .collect()
    };
    let tube1 = tube(5, 0.06);
    let tube2 = tube(6, 0.025);
    for i in 1..=divisions {
        let point = curve.point_at(i as f64 / divisions as f64);
        up = Vector3::Y;
        let forward = (point - prev_point).normalize_or_zero();
        let right = up.cross(forward).normalize_or_zero();
        up = forward.cross(right);
        let angle = forward.x.atan2(forward.z);
        quaternion = axis_angle(up, angle);
        let t = i as f64 / divisions as f64;
        let roll = axis_angle(forward, -(heading_change(curve, t) * 8.).atan() * 0.5);
        quaternion = mul(roll, quaternion);
        let mut draw_shape = |shape: &[Vector3], color: [f32; 3]| {
            let normal = rotate(Vector3::NEG_Z, quaternion);
            for &s in shape {
                push(&mut vertices, rotate(s, quaternion) + point);
                push(&mut normals, normal);
                colors.extend(color);
            }
            let normal = rotate(Vector3::Z, quaternion);
            for &s in shape.iter().rev() {
                push(&mut vertices, rotate(s, quaternion) + point);
                push(&mut normals, normal);
                colors.extend(color);
            }
        };
        if i % 2 == 0 {
            draw_shape(&step, color2);
        }
        let mut extrude = |shape: &[Vector3], offset: Vector3, color: [f32; 3]| {
            let jl = shape.len();
            for j in 0..jl {
                let point1 = shape[j];
                let point2 = shape[(j + 1) % jl];
                let v1 = rotate(point1 + offset, quaternion) + point;
                let v2 = rotate(point2 + offset, quaternion) + point;
                let v3 = rotate(point2 + offset, prev_quaternion) + prev_point;
                let v4 = rotate(point1 + offset, prev_quaternion) + prev_point;
                for v in [v1, v2, v4, v2, v3, v4] {
                    push(&mut vertices, v);
                }
                let n1 = rotate(point1, quaternion).normalize_or_zero();
                let n2 = rotate(point2, quaternion).normalize_or_zero();
                let n3 = rotate(point2, prev_quaternion).normalize_or_zero();
                let n4 = rotate(point1, prev_quaternion).normalize_or_zero();
                for n in [n1, n2, n4, n2, n3, n4] {
                    push(&mut normals, n);
                }
                for _ in 0..6 {
                    colors.extend(color);
                }
            }
        };
        extrude(&tube1, Vector3::new(0., -0.125, 0.), color2);
        extrude(&tube2, Vector3::new(0.2, 0., 0.), color1);
        extrude(&tube2, Vector3::new(-0.2, 0., 0.), color1);
        prev_point = point;
        prev_quaternion = quaternion;
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(vertices)?);
    g.set_attribute("normal", vec3s(normals)?);
    g.set_attribute("color", vec3s(colors)?);
    Ok(g)
}

/// RollerCoasterLiftersGeometry( curve, divisions ).
pub fn lifters(curve: &dyn Curve, divisions: u32) -> Result<BufferGeometry> {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let tube1 = [
        Vector3::new(0., 0.05, -0.05),
        Vector3::new(0., 0.05, 0.05),
        Vector3::new(0., -0.05, 0.),
    ];
    let tube2 = [
        Vector3::new(-0.05, 0., 0.05),
        Vector3::new(-0.05, 0., -0.05),
        Vector3::new(0.05, 0., 0.),
    ];
    let tube3 = [
        Vector3::new(0.05, 0., -0.05),
        Vector3::new(0.05, 0., 0.05),
        Vector3::new(-0.05, 0., 0.),
    ];
    for i in 1..=divisions {
        let t = i as f64 / divisions as f64;
        let point = curve.point_at(t);
        let tangent = curve.tangent_at(t);
        let angle = tangent.x.atan2(tangent.z);
        let quaternion = axis_angle(Vector3::Y, angle);
        let roll = axis_angle(tangent, -(heading_change(curve, t) * 8.).atan() * 0.5);
        let banked = mul(roll, quaternion);
        let mut extrude = |shape: &[Vector3], from: Vector3, to: Vector3| {
            let jl = shape.len();
            for j in 0..jl {
                let point1 = shape[j];
                let point2 = shape[(j + 1) % jl];
                let v1 = rotate(point1, quaternion) + from;
                let v2 = rotate(point2, quaternion) + from;
                let v3 = rotate(point2, quaternion) + to;
                let v4 = rotate(point1, quaternion) + to;
                for v in [v1, v2, v4, v2, v3, v4] {
                    push(&mut vertices, v);
                }
                let n1 = rotate(point1, quaternion).normalize_or_zero();
                let n2 = rotate(point2, quaternion).normalize_or_zero();
                for n in [n1, n2, n1, n2, n2, n1] {
                    push(&mut normals, n);
                }
            }
        };
        let at = |v: Vector3, q: Quaternion| rotate(v, q) + point;
        if point.y > 10. {
            extrude(
                &tube1,
                at(Vector3::new(-0.75, -0.35, 0.), quaternion),
                at(Vector3::new(0.75, -0.35, 0.), quaternion),
            );
            extrude(
                &tube2,
                at(Vector3::new(-0.7, -0.3, 0.), quaternion),
                at(Vector3::new(-0.7, -point.y, 0.), quaternion),
            );
            extrude(
                &tube3,
                at(Vector3::new(0.7, -0.3, 0.), quaternion),
                at(Vector3::new(0.7, -point.y, 0.), quaternion),
            );
        } else {
            let from = at(Vector3::new(0., -0.2, 0.), banked);
            let to = Vector3::new(from.x, 0., from.z);
            extrude(&tube3, from, to);
        }
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(vertices)?);
    g.set_attribute("normal", vec3s(normals)?);
    Ok(g)
}

/// RollerCoasterShadowGeometry( curve, divisions ): positions only.
pub fn shadow(curve: &dyn Curve, divisions: u32) -> Result<BufferGeometry> {
    let mut vertices = Vec::new();
    let mut prev_quaternion = axis_angle(Vector3::Y, PI / 2.);
    let mut prev_point = curve.point_at(0.);
    prev_point.y = 0.;
    for i in 1..=divisions {
        let mut point = curve.point_at(i as f64 / divisions as f64);
        point.y = 0.;
        let forward = point - prev_point;
        let quaternion = axis_angle(Vector3::Y, forward.x.atan2(forward.z));
        let v1 = rotate(Vector3::new(-0.3, 0., 0.), quaternion) + point;
        let v2 = rotate(Vector3::new(0.3, 0., 0.), quaternion) + point;
        let v3 = rotate(Vector3::new(0.3, 0., 0.), prev_quaternion) + prev_point;
        let v4 = rotate(Vector3::new(-0.3, 0., 0.), prev_quaternion) + prev_point;
        for v in [v1, v2, v4, v2, v3, v4] {
            push(&mut vertices, v);
        }
        prev_point = point;
        prev_quaternion = quaternion;
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(vertices)?);
    Ok(g)
}

/// SkyGeometry: 100 random quads.
pub fn sky() -> Result<BufferGeometry> {
    let mut vertices = Vec::new();
    for _ in 0..100 {
        let x = random() * 800. - 400.;
        let y = random() * 50. + 50.;
        let z = random() * 800. - 400.;
        let size = random() * 40. + 20.;
        for (dx, dz) in [
            (-1., -1.),
            (1., -1.),
            (-1., 1.),
            (1., -1.),
            (1., 1.),
            (-1., 1.),
        ] {
            push(&mut vertices, Vector3::new(x + dx * size, y, z + dz * size));
        }
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(vertices)?);
    Ok(g)
}

/// TreesGeometry( landscape ): 2000 random spots, kept where a downward ray
/// from y = 50 hits the landscape.
pub fn trees(scene: &mut Scene, landscape: Object3D) -> Result<BufferGeometry> {
    let mut vertices = Vec::new();
    let mut colors = Vec::new();
    let mut raycaster = Raycaster::default();
    scene.update_world_matrix(landscape, true, false)?;
    for _ in 0..2000 {
        let x = random() * 500. - 250.;
        let z = random() * 500. - 250.;
        raycaster.set(Vector3::new(x, 50., z), Vector3::NEG_Y);
        let hits = raycaster.intersect_object(scene, landscape, false)?;
        let Some(hit) = hits.first() else {
            continue;
        };
        let y = hit.point.y;
        let height = random() * 5. + 0.5;
        let mut angle = random() * PI * 2.;
        for _ in 0..2 {
            push(
                &mut vertices,
                Vector3::new(x + angle.sin(), y, z + angle.cos()),
            );
            push(&mut vertices, Vector3::new(x, y + height, z));
            push(
                &mut vertices,
                Vector3::new(x + (angle + PI).sin(), y, z + (angle + PI).cos()),
            );
            angle += PI / 2.;
        }
        let r = random() * 0.1;
        // Color.setRGB( 0.2 + random, 0.4 + random, 0, SRGBColorSpace ).
        let c = Color::from_srgb(0.2 + r, 0.4 + r, 0.);
        for _ in 0..6 {
            colors.extend([c.0.x as f32, c.0.y as f32, c.0.z as f32]);
        }
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(vertices)?);
    g.set_attribute("color", vec3s(colors)?);
    Ok(g)
}
