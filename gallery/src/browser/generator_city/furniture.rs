//! The street furniture generators' geometry ( three.js r186 city addons ):
//! streetlight, traffic signal, litter basket, bench, hydrant, street tree,
//! the three car bodies and the walking and standing pedestrians, each
//! merged from its parts with a per-vertex partId.
use super::geo::*;
use super::prims::*;
use std::f64::consts::PI;

/// strut( a, b, radius ): a cylinder from a to b.
fn strut(a: V3, b: V3, radius: f64) -> G {
    let dir = sub(b, a);
    let l = length(dir);
    cylinder(radius, radius, l, 6, 1, false)
        .apply_quaternion(from_unit_vectors([0., 1., 0.], normalize(dir)))
        .translate((a[0] + b[0]) / 2., (a[1] + b[1]) / 2., (a[2] + b[2]) / 2.)
}
pub(super) fn streetlight() -> G {
    let (h, reach, radius) = (9., 2.4, 0.1);
    let (metal, lens) = (0., 1.);
    let base = cylinder(0.18, 0.22, 0.6, 8, 1, false).translate(0., 0.3, 0.);
    let pole = cylinder(radius, radius * 1.7, h, 8, 1, false).translate(0., h / 2., 0.);
    let arm_base = [0., h - 0.4, 0.];
    let arm_knee = [0., h + 0.5, reach * 0.35];
    let arm_end = [0., h + 0.2, reach];
    let arm = merge_geometries(&[
        strut(arm_base, arm_knee, 0.07),
        strut(arm_knee, arm_end, 0.06),
    ]);
    let head = cuboid(0.26, 0.16, 0.7).translate(arm_end[0], arm_end[1] - 0.05, arm_end[2] + 0.2);
    let glass = cuboid(0.2, 0.05, 0.5).translate(arm_end[0], arm_end[1] - 0.14, arm_end[2] + 0.2);
    merge_geometries(&[
        part(base, metal),
        part(pole, metal),
        part(arm, metal),
        part(head, metal),
        part(glass, lens),
    ])
}
pub(super) fn trafficlight() -> G {
    let (h, reach, radius) = (6.5, 5.5, 0.14);
    let arm_y = h - 0.3;
    let metal = 0.;
    let base = cylinder(0.2, 0.26, 0.5, 8, 1, false).translate(0., 0.25, 0.);
    let pole = cylinder(radius, radius * 1.2, h, 8, 1, false).translate(0., h / 2., 0.);
    let arm = cylinder(0.07, 0.1, reach, 8, 1, false)
        .rotate_x(PI / 2.)
        .translate(0., arm_y, reach / 2.);
    let ped = cuboid(0.4, 0.42, 0.2).translate(0., 2.6, radius + 0.1);
    let (head_z, head_y) = (reach - 0.6, arm_y - 1.05);
    let housing = cuboid(0.36, 0.95, 0.32).translate(0., head_y, head_z);
    let drop_length = arm_y - (head_y + 0.475);
    let drop = cylinder(0.05, 0.05, drop_length, 6, 1, false).translate(
        0.,
        head_y + 0.475 + drop_length / 2.,
        head_z,
    );
    let lens_z = head_z - 0.19;
    let disc = |y: f64, id: f32| {
        part(
            cylinder(0.13, 0.13, 0.08, 12, 1, false)
                .rotate_x(PI / 2.)
                .translate(0., y, lens_z),
            id,
        )
    };
    merge_geometries(&[
        part(base, metal),
        part(pole, metal),
        part(arm, metal),
        part(drop, metal),
        part(ped, metal),
        part(housing, metal),
        disc(head_y + 0.28, 1.),
        disc(head_y, 2.),
        disc(head_y - 0.28, 3.),
    ])
}
pub(super) fn trashcan() -> G {
    let (r, h) = (0.28, 0.8);
    let (mesh, rim, trash) = (0., 1., 2.);
    let drum = cylinder(r, r * 0.92, h, 16, 1, true).translate(0., h / 2., 0.);
    let lip = cylinder(r + 0.03, r + 0.03, 0.07, 16, 1, false).translate(0., h, 0.);
    let foot = cylinder(r * 0.92, r * 0.86, 0.06, 16, 1, false).translate(0., 0.03, 0.);
    let bag = cylinder(r * 0.86, r * 0.7, h * 0.9, 12, 1, false).translate(0., h * 0.5, 0.);
    let mound = icosahedron(r * 0.82, 1)
        .scale(1., 0.55, 1.)
        .translate(0.02, h + 0.02, -0.01);
    merge_geometries(&[
        part(drum, mesh),
        part(lip, rim),
        part(foot, rim),
        part(bag, trash),
        part(mound, trash),
    ])
}
pub(super) fn hydrant() -> G {
    let (r, h) = (0.13, 0.55);
    let (body, cap) = (0., 1.);
    let c = |top: f64, bottom: f64, height: f64, radial: u32| {
        cylinder(top, bottom, height, radial, 1, false)
    };
    let footing = c(r * 1.5, r * 1.7, 0.08, 12).translate(0., 0.04, 0.);
    let barrel = c(r, r * 1.1, h, 12).translate(0., 0.08 + h / 2., 0.);
    let shoulder = c(r * 0.65, r, 0.12, 12).translate(0., 0.69, 0.);
    let dome = sphere(r * 0.65, 10, 4, (0., PI * 2.), (0., PI / 2.)).translate(0., 0.75, 0.);
    let nut = c(0.05, 0.05, 0.07, 6).translate(0., 0.87, 0.);
    let bonnet = c(r * 1.18, r * 1.18, 0.035, 12).translate(0., 0.645, 0.);
    let flange = c(r * 1.28, r * 1.28, 0.04, 12).translate(0., 0.11, 0.);
    let nozzle_l = c(0.06, 0.06, 0.12, 8)
        .rotate_z(PI / 2.)
        .translate(-(r + 0.03), 0.45, 0.);
    let cap_l = c(0.07, 0.07, 0.025, 8)
        .rotate_z(PI / 2.)
        .translate(-(r + 0.102), 0.45, 0.);
    let nozzle_r = c(0.06, 0.06, 0.12, 8)
        .rotate_z(PI / 2.)
        .translate(r + 0.03, 0.45, 0.);
    let cap_r = c(0.07, 0.07, 0.025, 8)
        .rotate_z(PI / 2.)
        .translate(r + 0.102, 0.45, 0.);
    let pumper = c(0.075, 0.075, 0.12, 8)
        .rotate_x(PI / 2.)
        .translate(0., 0.4, r + 0.03);
    let pumper_cap = c(0.085, 0.085, 0.025, 8)
        .rotate_x(PI / 2.)
        .translate(0., 0.4, r + 0.102);
    merge_geometries(&[
        part(footing, body),
        part(barrel, body),
        part(shoulder, body),
        part(dome, body),
        part(bonnet, body),
        part(flange, body),
        part(nozzle_l, body),
        part(nozzle_r, body),
        part(pumper, body),
        part(cap_l, cap),
        part(cap_r, cap),
        part(pumper_cap, cap),
        part(nut, cap),
    ])
}
pub(super) fn bench() -> G {
    let (length, depth, seat_y, back_y) = (1.8, 0.55, 0.45, 0.85);
    let (wood, iron) = (0., 1.);
    let half_length = length / 2.;
    let front_z = depth / 2. - 0.075;
    let back_z = -front_z;
    let mut parts = vec![];
    for side in [-1., 1.] {
        let x = side * (half_length - 0.07);
        parts.extend(
            [
                cuboid(0.05, seat_y, 0.06).translate(x, seat_y / 2., front_z),
                cuboid(0.05, back_y, 0.06).translate(x, back_y / 2., back_z),
                cuboid(0.06, 0.05, front_z - back_z + 0.1).translate(x, seat_y - 0.04, 0.),
                cuboid(0.05, 0.22, 0.05).translate(x, seat_y + 0.11, front_z),
                cuboid(0.05, 0.05, front_z - back_z + 0.06).translate(x, seat_y + 0.22, 0.),
                cuboid(0.05, 0.16, 0.05)
                    .rotate_x(-0.6)
                    .translate(x, seat_y + 0.18, front_z + 0.07),
            ]
            .map(|g| part(g, iron)),
        );
    }
    let slat = length - 0.1;
    let (z0, z1) = (back_z + 0.02, front_z - 0.02);
    for i in 0..5 {
        let z = z0 + (z1 - z0) * (i as f64 / 4.);
        parts.push(part(
            cuboid(slat, 0.03, 0.07).translate(0., seat_y, z),
            wood,
        ));
    }
    for i in 0..3 {
        let t = i as f64 / 2.;
        let y = seat_y + 0.13 + t * 0.27;
        let z = back_z + 0.01 - t * 0.06;
        parts.push(part(
            cuboid(slat, 0.09, 0.025).rotate_x(0.18).translate(0., y, z),
            wood,
        ));
    }
    merge_geometries(&parts)
}
const CROWN_CENTER: V3 = [0., 3.9, 0.1];
fn hash(x: f64, y: f64, z: f64) -> f64 {
    let s = (x * 127.1 + y * 311.7 + z * 74.7).sin() * 43758.5453;
    s - s.floor()
}
fn leaf_clump(radius: f64, x: f64, y: f64, z: f64) -> G {
    let mut g = merge_vertices(&icosahedron(radius, 1).scale(1., 0.82, 1.));
    for p in g.get_mut("position").expect("position").chunks_mut(3) {
        let v = [p[0] as f64, p[1] as f64, p[2] as f64];
        let n = hash(v[0], v[1], v[2]);
        let s = 0.86 + n * 0.3;
        for k in 0..3 {
            p[k] = (v[k] * s) as f32;
        }
    }
    let mut g = g.translate(x, y, z);
    g.compute_vertex_normals();
    let position = g.get("position").expect("position").clone();
    for (n, p) in g
        .get_mut("normal")
        .expect("normal")
        .chunks_mut(3)
        .zip(position.chunks(3))
    {
        let v = normalize(sub([p[0] as f64, p[1] as f64, p[2] as f64], CROWN_CENTER));
        for k in 0..3 {
            n[k] = (n[k] as f64 * 0.45 + v[k] * 0.55) as f32;
        }
    }
    g.normalize_normals();
    g
}
pub(super) fn street_tree() -> G {
    let (r, h) = (0.18, 2.6);
    let (trunk, leaf, grate) = (0., 1., 2.);
    let soil = cylinder(0.24, 0.3, 0.05, 10, 1, false).translate(0., 0.025, 0.);
    let pit = ring(0.24, 0.62, 16, 4)
        .rotate_x(-PI / 2.)
        .translate(0., 0.045, 0.);
    let flare = cylinder(r * 1.15, r * 1.6, 0.22, 8, 1, false).translate(0., 0.11, 0.);
    let stem = cylinder(r * 0.55, r * 1.1, h, 8, 1, false).translate(0., h / 2., 0.);
    let mut wood = vec![flare, stem];
    for (rx, rz, len) in [
        (0.5, 0.2, 1.5),
        (-0.45, -0.3, 1.4),
        (0.1, -0.55, 1.2),
        (-0.15, 0.5, 1.3),
    ] {
        wood.push(
            cylinder(0.03, 0.07, len, 5, 1, false)
                .translate(0., len / 2., 0.)
                .rotate_x(rx)
                .rotate_z(rz)
                .translate(0., h - 0.15, 0.),
        );
    }
    let clumps = [
        leaf_clump(2.15, 0., 3.9, 0.1),
        leaf_clump(1.5, 1.15, 4.35, 0.5),
        leaf_clump(1.45, -1.05, 3.55, -0.5),
        leaf_clump(1.3, 0.35, 4.75, -0.65),
        leaf_clump(1.25, -0.35, 4.5, 0.9),
        leaf_clump(1.1, 0.9, 3.3, -0.85),
    ];
    let mut parts = vec![
        part(soil, trunk),
        part(pit, grate),
        part(merge_geometries(&wood), trunk),
    ];
    parts.extend(clumps.into_iter().map(|c| part(c, leaf)));
    merge_geometries(&parts)
}
// ---------------------------------------------------------------- cars

pub(super) struct Spec {
    body: [[f64; 4]; 8],
    front: ([f64; 3], [f64; 3]),
    rear: ([f64; 3], [f64; 3]),
    pub(super) wheel_radius: f64,
    pub(super) wheel_z: f64,
    wheel_x: f64,
    pub(super) pillars: &'static [f64],
    pub(super) lamps: [f64; 2],
    sign: bool,
    pub(super) rails: bool,
}
impl Spec {
    pub(super) fn front_base(&self) -> [f64; 3] {
        self.front.0
    }
    pub(super) fn rear_base(&self) -> [f64; 3] {
        self.rear.0
    }
}
pub(super) const SEDAN: Spec = Spec {
    body: [
        [2.25, 0.79, 0.67, 0.79],
        [2.11, 0.89, 0.78, 0.91],
        [1.38, 0.94, 0.87, 1.00],
        [0.75, 0.92, 0.89, 1.025],
        [-0.45, 0.92, 0.91, 1.04],
        [-1.38, 0.94, 0.88, 1.06],
        [-2.10, 0.89, 0.77, 0.97],
        [-2.25, 0.81, 0.70, 0.85],
    ],
    front: ([0.82, 0.99, 0.78], [0.69, 1.45, 0.12]),
    rear: ([0.83, 1.02, -1.15], [0.71, 1.47, -0.72]),
    wheel_radius: 0.35,
    wheel_z: 1.38,
    wheel_x: 0.83,
    pillars: &[-0.30],
    lamps: [0.71, 0.76],
    sign: false,
    rails: false,
};
pub(super) const SUV: Spec = Spec {
    body: [
        [2.30, 0.84, 0.84, 0.99],
        [2.15, 0.94, 0.94, 1.10],
        [1.40, 0.98, 1.02, 1.17],
        [0.76, 0.96, 1.04, 1.19],
        [-0.45, 0.96, 1.05, 1.20],
        [-1.40, 0.98, 1.03, 1.21],
        [-2.16, 0.94, 0.94, 1.16],
        [-2.30, 0.84, 0.86, 1.03],
    ],
    front: ([0.86, 1.15, 0.78], [0.76, 1.73, 0.18]),
    rear: ([0.87, 1.15, -2.12], [0.77, 1.75, -1.66]),
    wheel_radius: 0.39,
    wheel_z: 1.40,
    wheel_x: 0.87,
    pillars: &[-0.30, -1.16],
    lamps: [0.91, 0.94],
    sign: false,
    rails: true,
};
pub(super) const TAXI: Spec = Spec {
    sign: true,
    ..SEDAN
};
const BODY: f32 = 0.;
const WINDOW: f32 = 1.;
const TYRE: f32 = 2.;
const ALLOY: f32 = 3.;
const TRIM: f32 = 4.;
const MIRROR: f32 = 5.;
const SIGN: f32 = 6.;
const FRONT: f32 = 7.;
const REAR: f32 = 8.;
fn car_body(spec: &Spec) -> G {
    let profile = &spec.body;
    let radius = spec.wheel_radius + 0.055;
    let mut stations: Vec<f64> = vec![];
    let mut add = |z: f64| {
        if !stations.contains(&z) {
            stations.push(z);
        }
    };
    for s in profile {
        add(s[0]);
    }
    for axle in [-spec.wheel_z, spec.wheel_z] {
        for i in 0..=6 {
            add(axle + radius * (i as f64 / 6. * PI).cos());
        }
    }
    stations.sort_by(|a, b| b.partial_cmp(a).expect("finite station"));
    let sections: Vec<Vec<V3>> = stations
        .iter()
        .map(|&z| {
            let mut index = 0;
            while index < profile.len() - 2 && z < profile[index + 1][0] {
                index += 1;
            }
            let (a, b) = (profile[index], profile[index + 1]);
            let t = (z - a[0]) / (b[0] - a[0]);
            let w = a[1] + (b[1] - a[1]) * t;
            let shoulder = a[2] + (b[2] - a[2]) * t;
            let deck = a[3] + (b[3] - a[3]) * t;
            let distance = (z.abs() - spec.wheel_z).abs();
            let sill = if distance <= radius {
                spec.wheel_radius + (radius * radius - distance * distance).max(0.).sqrt()
            } else {
                0.28
            };
            let right = [
                [w * 0.82, sill, z],
                [w * 0.97, sill + (shoulder - sill) * 0.12, z],
                [w, shoulder, z],
                [w * 0.91, deck - 0.025, z],
                [w * 0.52, deck, z],
            ];
            let mut section: Vec<V3> = right.to_vec();
            section.extend(right.iter().rev().map(|p| [-p[0], p[1], p[2]]));
            section.reverse();
            section
        })
        .collect();
    let mut g = part(loft(&sections, true, true, true), BODY);
    let normals = g.get("normal").expect("normal").clone();
    let ids = g.get_mut("partId").expect("partId");
    for (i, n) in normals.chunks(3).enumerate() {
        if n[2] as f64 > 0.9999 {
            ids[i] = FRONT;
        }
        if (n[2] as f64) < -0.9999 {
            ids[i] = REAR;
        }
    }
    g
}
fn lerp(a: V3, b: V3, t: f64) -> V3 {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}
fn panel(corners: [V3; 4], id: f32, curved: bool) -> G {
    let (columns, rows) = if curved { (4, 2) } else { (1, 1) };
    let normal = normalize(cross(
        sub(corners[1], corners[0]),
        sub(corners[3], corners[0]),
    ));
    let (mut position, mut uv, mut index): (Vec<f64>, Vec<f64>, Vec<u32>) =
        (vec![], vec![], vec![]);
    for y in 0..=rows {
        let v = y as f64 / rows as f64;
        for x in 0..=columns {
            let u = x as f64 / columns as f64;
            let mut p = lerp(
                lerp(corners[0], corners[1], u),
                lerp(corners[3], corners[2], u),
                v,
            );
            if curved {
                let arch = 4. * u * (1. - u);
                p[1] += arch * v * 0.035;
                let s = arch * 4. * v * (1. - v) * 0.025;
                for k in 0..3 {
                    p[k] += normal[k] * s;
                }
            }
            position.extend(p);
            uv.extend([u, v]);
            if x < columns && y < rows {
                let a = (y * (columns + 1) + x) as u32;
                let (b, d) = (a + 1, a + columns as u32 + 1);
                let c = d + 1;
                index.extend([a, b, d, b, c, d]);
            }
        }
    }
    let mut g = G {
        attributes: vec![
            ("position", 3, position.iter().map(|&v| v as f32).collect()),
            ("uv", 2, uv.iter().map(|&v| v as f32).collect()),
        ],
        index: Some(index),
    };
    g.compute_vertex_normals();
    part(g, id)
}
pub(super) fn car(spec: &Spec) -> G {
    let mut parts = vec![car_body(spec)];
    let corner = |p: [f64; 3], side: f64| [p[0] * side, p[1], p[2]];
    let (fl, fr) = (corner(spec.front.0, -1.), corner(spec.front.0, 1.));
    let (rl, rr) = (corner(spec.rear.0, -1.), corner(spec.rear.0, 1.));
    let (tfl, tfr) = (corner(spec.front.1, -1.), corner(spec.front.1, 1.));
    let (trl, trr) = (corner(spec.rear.1, -1.), corner(spec.rear.1, 1.));
    parts.extend([
        panel([fl, fr, tfr, tfl], WINDOW, true),
        panel([rr, rl, trl, trr], WINDOW, true),
        panel([fr, rr, trr, tfr], WINDOW, false),
        panel([rl, fl, tfl, trl], WINDOW, false),
    ]);
    let roof: Vec<Vec<V3>> = [spec.front.1, spec.rear.1]
        .iter()
        .map(|&[width, y, z]| {
            (0..5)
                .map(|i| {
                    let u = i as f64 / 4.;
                    [(u * 2. - 1.) * width, y + 4. * u * (1. - u) * 0.035, z]
                })
                .collect()
        })
        .collect();
    parts.push(part(loft(&roof, false, false, false), BODY));
    let r = spec.wheel_radius;
    let rim = r * 0.64;
    let width = r * 0.68;
    let profile = [
        [r * 0.93, -width * 0.44],
        [r, -width * 0.12],
        [r * 0.98, width * 0.27],
        [rim + 0.014, width * 0.44],
    ];
    for side in [-1., 1.] {
        for z in [-spec.wheel_z, spec.wheel_z] {
            let x = side * spec.wheel_x;
            let tyre = lathe(&profile, 24)
                .rotate_z(-side * PI / 2.)
                .translate(x, r, z);
            let lip = lathe(&[[rim + 0.014, width * 0.44], [rim, width * 0.29]], 24)
                .rotate_z(-side * PI / 2.)
                .translate(x, r, z);
            let mut hub = circle(rim, 24, 0., PI * 2.);
            hub.get_mut("position").expect("position")[2] = -0.015;
            hub.compute_vertex_normals();
            let hub = hub
                .rotate_y(side * PI / 2.)
                .translate(x + side * width * 0.29, r, z);
            let well = circle(r + 0.06, 8, 0., PI)
                .rotate_y(side * PI / 2.)
                .translate(x - side * (width * 0.5 + 0.02), r, z);
            parts.extend([
                part(tyre, TYRE),
                part(lip, ALLOY),
                part(hub, ALLOY),
                part(well, TRIM),
            ]);
        }
        let mirror = cuboid(0.16, 0.1, 0.2).rotate_y(side * 0.2).translate(
            side * (spec.body[2][1] + 0.06),
            spec.front.0[1] + 0.05,
            spec.front.0[2] - 0.16,
        );
        parts.push(part(mirror, MIRROR));
        if spec.rails {
            let sections: Vec<Vec<V3>> = [0.06, 0.13, 0.87, 0.94]
                .iter()
                .enumerate()
                .map(|(i, &t)| {
                    let z = tfr[2] + (trr[2] - tfr[2]) * t;
                    let roof_width = tfr[0] + (trr[0] - tfr[0]) * t;
                    let q = 0.63 / roof_width;
                    let y = tfr[1]
                        + (trr[1] - tfr[1]) * t
                        + (1. - q * q) * 0.035
                        + if i == 0 || i == 3 { 0.005 } else { 0.05 };
                    let x = side * 0.63;
                    let mut s = vec![
                        [x - 0.022, y - 0.018, z],
                        [x + 0.022, y - 0.018, z],
                        [x + 0.022, y + 0.018, z],
                        [x - 0.022, y + 0.018, z],
                    ];
                    s.reverse();
                    s
                })
                .collect();
            parts.push(part(loft(&sections, true, true, true), TRIM));
        }
    }
    if spec.sign {
        let roof_y = (tfr[1] + trr[1]) / 2. + 0.04;
        let section = |w: f64, d: f64, y: f64| vec![[w, y, d], [-w, y, d], [-w, y, -d], [w, y, -d]];
        let sign = loft(
            &[
                section(0.16, 0.065, roof_y + 0.1),
                section(0.20, 0.095, roof_y),
            ],
            true,
            true,
            true,
        )
        .translate(0., 0., -0.2);
        parts.push(part(sign, SIGN));
    }
    merge_geometries(&parts)
}
// ---------------------------------------------------------------- people

const SKIN: f32 = 0.;
const HEAD: f32 = 1.;
const COAT: f32 = 2.;
const LEGS: f32 = 3.;
const SHOES: f32 = 4.;
const BAG: f32 = 5.;
const SLEEVE: f32 = 6.;
const HANDLE: f32 = 7.;
fn joint(parent: V3, length: f64, swing: f64, splay: f64) -> V3 {
    let vertical = length * splay.cos();
    [
        parent[0] + length * splay.sin(),
        parent[1] - vertical * swing.cos(),
        parent[2] - vertical * swing.sin(),
    ]
}
fn ring_points(center: V3, rx: f64, rz: f64, segments: usize, phase: f64) -> Vec<V3> {
    (0..segments)
        .map(|i| {
            let a = i as f64 / segments as f64 * PI * 2. + phase;
            [
                center[0] + a.cos() * rx,
                center[1],
                center[2] + a.sin() * rz,
            ]
        })
        .collect()
}
fn limb_sections(joints: &[(V3, f64, f64)], segments: usize, phase: f64) -> Vec<Vec<V3>> {
    joints
        .iter()
        .enumerate()
        .map(|(i, &(center, rx, rz))| {
            let before = joints[i.saturating_sub(1)].0;
            let after = joints[(i + 1).min(joints.len() - 1)].0;
            let rotation = from_unit_vectors([0., -1., 0.], normalize(sub(after, before)));
            ring_points([0.; 3], rx, rz, segments, phase)
                .into_iter()
                .map(|p| {
                    let q = apply_quaternion(p, rotation);
                    [q[0] + center[0], q[1] + center[1], q[2] + center[2]]
                })
                .collect()
        })
        .collect()
}
fn limb(joints: &[(V3, f64, f64)], cap_end: bool, segments: usize) -> G {
    loft(&limb_sections(joints, segments, 0.), true, false, cap_end)
}
fn torso_section(y: f64, width: f64, depth: f64) -> Vec<V3> {
    vec![
        [width, y, depth * 0.45],
        [width * 0.65, y, depth],
        [-width * 0.65, y, depth],
        [-width, y, depth * 0.45],
        [-width, y, -depth * 0.45],
        [-width * 0.65, y, -depth],
        [width * 0.65, y, -depth],
        [width, y, -depth * 0.45],
    ]
}
fn hip_section(side: f64) -> Vec<V3> {
    let profile = [
        [1., 0.5],
        [-0.1, 1.],
        [-0.8, 0.7],
        [-1., 0.],
        [-0.8, -0.7],
        [-0.1, -1.],
        [1., -0.5],
    ];
    let mut points: Vec<V3> = profile
        .iter()
        .enumerate()
        .map(|(i, &[x, z])| {
            [
                side * 0.093 - side * x * 0.093,
                if i == 0 || i == 6 { 0.82 } else { 0.89 },
                z * 0.1,
            ]
        })
        .collect();
    if side > 0. {
        points.reverse();
    }
    points
}
fn shoe_section(z: f64, width: f64, top: f64) -> Vec<V3> {
    vec![
        [width, top - 0.025, z],
        [width * 0.7, top, z],
        [-width * 0.7, top, z],
        [-width, top - 0.025, z],
        [-width * 0.9, -0.085, z],
        [width * 0.9, -0.085, z],
    ]
}
/// Box3.setFromBufferAttribute: the Float32Array positions' extent.
fn bounds(g: &G) -> (V3, V3) {
    let (mut min, mut max) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for p in g.get("position").expect("position").chunks(3) {
        for k in 0..3 {
            min[k] = min[k].min(p[k] as f64);
            max[k] = max[k].max(p[k] as f64);
        }
    }
    (min, max)
}
pub(super) fn person(walking: bool) -> G {
    let mut parts = vec![];
    let head_profile = [
        [1.75, 0.014, 0.014, -0.012],
        [1.708, 0.081, 0.081, -0.01],
        [1.656, 0.093, 0.086, 0.],
        [1.617, 0.086, 0.077, 0.009],
        [1.561, 0.072, 0.065, 0.012],
        [1.532, 0.045, 0.043, 0.005],
    ];
    let mut head_sections: Vec<Vec<V3>> = head_profile
        .iter()
        .map(|&[y, rx, rz, z]| ring_points([0., y, z], rx, rz, 10, PI / 2.))
        .collect();
    head_sections[3][0][2] += 0.022;
    let mut head = loft(&head_sections, true, true, true);
    let ys: Vec<f32> = head
        .get("position")
        .expect("position")
        .chunks(3)
        .map(|p| p[1])
        .collect();
    for (uv, y) in head.get_mut("uv").expect("uv").chunks_mut(2).zip(ys) {
        uv[0] = y;
    }
    let mut heads = vec![head];
    for side in [-1., 1.] {
        heads.push(
            sphere(1., 4, 2, (0., PI * 2.), (0., PI))
                .scale(0.016, 0.026, 0.018)
                .translate(side * 0.092, 1.62, -0.004),
        );
    }
    let heads: Vec<G> = heads
        .into_iter()
        .map(|g| {
            g.translate(0., -1.535, 0.)
                .rotate_y(if walking { 0.14 } else { -0.16 })
                .rotate_z(if walking { -0.02 } else { 0.055 })
                .translate(0., 1.535, 0.)
        })
        .collect();
    let head_offset = 1.75 - bounds(&heads[0]).1[1];
    for (i, g) in heads.into_iter().enumerate() {
        parts.push(part(
            g.translate(0., head_offset, 0.),
            if i == 0 { HEAD } else { SKIN },
        ));
    }
    parts.push(part(
        cylinder(0.045, 0.052, 0.10, 5, 1, true).translate(0., 1.515, 0.),
        SKIN,
    ));
    let jacket = [
        [1.5, 0.055, 0.052],
        [1.425, 0.20, 0.10],
        [1.365, 0.195, 0.111],
        [1.285, 0.18, 0.115],
        [1.095, 0.152, 0.103],
        [1.015, 0.166, 0.108],
    ];
    let sections: Vec<Vec<V3>> = jacket
        .iter()
        .map(|s| torso_section(s[0], s[1], s[2]))
        .collect();
    parts.push(part(loft(&sections, true, false, true), COAT));
    let (mut trousers, mut hips) = (vec![], vec![]);
    for side in [-1., 1.] {
        let swing = if walking {
            -side * 0.35
        } else if side < 0. {
            -0.12
        } else {
            0.06
        };
        let shoulder = [side * 0.197, 1.425, 0.008];
        let upper_arm = joint(shoulder, 0.075, swing, side * 0.08);
        let elbow = joint(shoulder, 0.285, swing, side * 0.08);
        let wrist = joint(elbow, 0.255, swing - 0.22, side * 0.03);
        parts.push(part(
            limb(
                &[
                    ([side * 0.14, 1.415, 0.], 0.024, 0.035),
                    (upper_arm, 0.066, 0.063),
                    (elbow, 0.051, 0.048),
                    (wrist, 0.029, 0.033),
                ],
                false,
                5,
            ),
            SLEEVE,
        ));
        let along = normalize(sub(wrist, elbow));
        let offset = |s: f64| {
            [
                wrist[0] + along[0] * s,
                wrist[1] + along[1] * s,
                wrist[2] + along[2] * s,
            ]
        };
        let hand = offset(0.05);
        parts.push(part(
            limb(
                &[
                    (wrist, 0.029, 0.033),
                    (hand, 0.032, 0.039),
                    (offset(0.095), 0.019, 0.025),
                ],
                true,
                5,
            ),
            SKIN,
        ));
        if !walking && side > 0. {
            parts.push(part(
                cuboid(0.075, 0.21, 0.22).translate(hand[0], hand[1] - 0.19, hand[2]),
                BAG,
            ));
            parts.push(part(
                torus(0.045, 0.006, 3, 4, PI).rotate_y(PI / 2.).translate(
                    hand[0],
                    hand[1] - 0.08,
                    hand[2],
                ),
                HANDLE,
            ));
        }
        let mut shoe = loft(
            &[
                shoe_section(-0.075, 0.044, -0.008),
                shoe_section(0.015, 0.054, 0.016),
                shoe_section(0.125, 0.055, -0.025),
                shoe_section(0.18, 0.036, -0.045),
            ],
            true,
            true,
            true,
        );
        let yz: Vec<[f32; 2]> = shoe
            .get("position")
            .expect("position")
            .chunks(3)
            .map(|p| [p[1], p[2]])
            .collect();
        for (uv, v) in shoe.get_mut("uv").expect("uv").chunks_mut(2).zip(yz) {
            uv.copy_from_slice(&v);
        }
        let shoe = shoe
            .rotate_x(if walking {
                if side < 0. { -0.12 } else { 0.25 }
            } else {
                0.
            })
            .rotate_y(if walking { side * 0.03 } else { side * 0.16 });
        let shoe_min = bounds(&shoe).0;
        let hip = [side * 0.093, 0.89, 0.];
        let ankle = [
            side * if walking { 0.105 } else { 0.12 },
            -shoe_min[1],
            if walking {
                -side * 0.235
            } else if side < 0. {
                0.065
            } else {
                -0.025
            },
        ];
        let mut knee = lerp(hip, ankle, 0.53);
        knee[2] += if walking {
            if side > 0. { 0.095 } else { 0.015 }
        } else if side < 0. {
            0.06
        } else {
            0.01
        };
        let calf = lerp(knee, ankle, 0.40);
        let mut leg = limb_sections(
            &[
                (hip, 0.087, 0.092),
                (knee, 0.062, 0.067),
                (calf, 0.066, 0.067),
                ([ankle[0], ankle[1] - 0.018, ankle[2]], 0.045, 0.046),
            ],
            7,
            PI / 7. + if side > 0. { PI } else { 0. },
        );
        leg[0] = hip_section(side);
        hips.push(leg[0].clone());
        trousers.push(loft(&leg, true, false, true));
        parts.push(part(shoe.translate(ankle[0], ankle[1], ankle[2]), SHOES));
    }
    let mut crotch = hips[0].clone();
    crotch.extend_from_slice(&hips[1][1..6]);
    trousers.push(loft(
        &[ring_points([0., 1.04, 0.], 0.157, 0.1, 12, PI / 2.), crotch],
        true,
        false,
        false,
    ));
    let mut pants = merge_geometries(&trousers);
    pants.delete("normal");
    pants.delete("uv");
    let mut joined = merge_vertices(&pants);
    joined.compute_vertex_normals();
    let n = joined.count();
    joined.set("uv", 2, vec![0.; n * 2]);
    parts.push(part(joined, LEGS));
    merge_geometries(&parts)
}
