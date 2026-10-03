//! CityGenerator ( three.js r186 addon ): the block and lot layout, each
//! lot's seeded SkyscraperGenerator tower, the rounded sidewalk slabs and
//! curbs, the street furniture's placements along every curb, and the
//! tower boxes of the GI proxy.
use super::super::generator_building::skyscraper::{self, Building, Settings};
use super::super::text_shapes::{Extrude, Path, Shape, extrude};
use super::geo::*;
use std::f64::consts::PI;

/// CityGenerator.defaults and cityLayout().
#[derive(Clone, Copy)]
pub(super) struct Layout {
    pub(super) street: f64,
    pub(super) lots_x: usize,
    pub(super) lots_z: usize,
    pub(super) blocks_x: usize,
    pub(super) blocks_z: usize,
    pub(super) block_w: f64,
    pub(super) block_d: f64,
    pub(super) sidewalk: f64,
    pub(super) inner_lot_x: f64,
    pub(super) inner_lot_z: f64,
    pub(super) city_w: f64,
    pub(super) city_d: f64,
}
pub(super) const CURB_HEIGHT: f64 = 0.15;
const CURB_RADIUS: f64 = 5.;
pub(super) fn layout() -> Layout {
    let (street, lot, lots_x, lots_z, blocks_x, blocks_z, sidewalk) = (22., 30., 3, 2, 2, 2, 5.);
    let block_w = lots_x as f64 * lot;
    let block_d = lots_z as f64 * lot;
    Layout {
        street,
        lots_x,
        lots_z,
        blocks_x,
        blocks_z,
        block_w,
        block_d,
        sidewalk,
        inner_lot_x: (block_w - 2. * sidewalk) / lots_x as f64,
        inner_lot_z: (block_d - 2. * sidewalk) / lots_z as f64,
        city_w: blocks_x as f64 * block_w + (blocks_x - 1) as f64 * street,
        city_d: blocks_z as f64 * block_d + (blocks_z - 1) as f64 * street,
    }
}
/// createRandom( seed ): mulberry32.
fn create_random(seed: u32) -> impl FnMut() -> f64 {
    let mut s = if seed == 0 { 1 } else { seed };
    move || {
        s = s.wrapping_add(0x6D2B79F5);
        let mut t = (s ^ (s >> 15)).wrapping_mul(1 | s);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        (t ^ (t >> 14)) as f64 / 4294967296.
    }
}
/// One tower: its geometry, position and the GI proxy box ( centre, size ).
pub(super) struct Tower {
    pub(super) building: Building,
    pub(super) position: V3,
    pub(super) proxy: (V3, V3),
}
/// A parked or travelling car: placement and paint.
pub(super) struct Car {
    pub(super) matrix: M4,
    pub(super) color: u32,
}
/// The furniture placements, per generator.
#[derive(Default)]
pub(super) struct Furniture {
    pub(super) lights: Vec<M4>,
    pub(super) signals: Vec<M4>,
    pub(super) cans: Vec<M4>,
    pub(super) benches: Vec<M4>,
    pub(super) hydrants: Vec<M4>,
    pub(super) trees: Vec<M4>,
    pub(super) people: Vec<M4>,
    pub(super) cars: Vec<Car>,
}
pub(super) struct City {
    pub(super) towers: Vec<Tower>,
    /// The sidewalk slabs' placements, one per block.
    pub(super) slabs: Vec<M4>,
    pub(super) furniture: Furniture,
}
/// The kerbside paint mix: cumulative thresholds and colours.
const CAR_COLORS: [(f64, u32); 9] = [
    (0.22, 0xf5c518),
    (0.42, 0x111216),
    (0.59, 0xe9e8e3),
    (0.72, 0xb2b5b8),
    (0.84, 0x3e4247),
    (0.90, 0x74787c),
    (0.95, 0x1c2a3f),
    (0.98, 0x571f1f),
    (1.01, 0x5c4834),
];
pub(super) const TAXI_COLOR: u32 = 0xf5c518;
/// place( x, y, z, faceX, faceZ ): local +Z turned to face ( faceX, faceZ ).
fn place(x: f64, y: f64, z: f64, face_x: f64, face_z: f64) -> M4 {
    let up = [0., 1., 0.];
    let f = normalize([face_x, 0., face_z]);
    let r = normalize(cross(up, f));
    set_position(basis(r, up, f), [x, y, z])
}
/// placeYawScale( x, y, z, yaw, scale ).
fn place_yaw_scale(x: f64, y: f64, z: f64, yaw: f64, scale: f64) -> M4 {
    compose(
        [x, y, z],
        from_axis_angle([0., 1., 0.], yaw),
        [scale, scale, scale],
    )
}
pub(super) fn build(seed: f64) -> City {
    let l = layout();
    let mut random = create_random(seed as u32);
    let (curb, sw) = (CURB_HEIGHT, l.sidewalk);
    let (mut towers, mut slabs) = (vec![], vec![]);
    for bx in 0..l.blocks_x {
        for bz in 0..l.blocks_z {
            let block_x = -l.city_w / 2. + bx as f64 * (l.block_w + l.street);
            let block_z = -l.city_d / 2. + bz as f64 * (l.block_d + l.street);
            slabs.push(translation([
                block_x + l.block_w / 2.,
                0.,
                block_z + l.block_d / 2.,
            ]));
            let (zone_x, zone_z) = (block_x + sw, block_z + sw);
            for lx in 0..l.lots_x {
                for lz in 0..l.lots_z {
                    let corner_x = if lx == 0 {
                        -1.
                    } else if lx == l.lots_x - 1 {
                        1.
                    } else {
                        0.
                    };
                    let corner_z = if lz == 0 {
                        -1.
                    } else if lz == l.lots_z - 1 {
                        1.
                    } else {
                        0.
                    };
                    let on_corner = corner_x != 0. && corner_z != 0.;
                    let tall = random();
                    let fw = l.inner_lot_x - (0.4 + random() * 1.);
                    let fd = l.inner_lot_z - (0.4 + random() * 1.);
                    let total_height = 38. + tall * tall * 114.;
                    let tower_seed = (random() * 100000.).floor();
                    let floor_height = 3.4 + random() * 1.8;
                    let bay_width = 1.9 + random() * 2.1;
                    let pier_width = 0.4 + random() * 0.5;
                    let pier_depth = 0.3 + random() * 0.4;
                    let chamfer = if on_corner { 3. + random() * 4. } else { 0. };
                    let setback = if random() < 0.4 {
                        0.8 + random() * 2.
                    } else {
                        0.
                    };
                    let string_course_every = if random() < 0.85 {
                        3 + (random() * 6.).floor() as usize
                    } else {
                        0
                    };
                    let building = skyscraper::build(&Settings {
                        seed: tower_seed,
                        height: total_height,
                        width: fw,
                        depth: fd,
                        floor_height,
                        bay_width,
                        chamfer,
                        setback,
                        pier: Some((pier_width, pier_depth)),
                        chamfer_corner: (corner_x, corner_z),
                        string_course_every,
                    });
                    let lot_left = zone_x + lx as f64 * l.inner_lot_x;
                    let lot_near = zone_z + lz as f64 * l.inner_lot_z;
                    let cx = if corner_x == -1. {
                        lot_left + fw / 2.
                    } else if corner_x == 1. {
                        lot_left + l.inner_lot_x - fw / 2.
                    } else {
                        lot_left + l.inner_lot_x / 2.
                    };
                    let cz = if corner_z == -1. {
                        lot_near + fd / 2.
                    } else if corner_z == 1. {
                        lot_near + l.inner_lot_z - fd / 2.
                    } else {
                        lot_near + l.inner_lot_z / 2.
                    };
                    towers.push(Tower {
                        building,
                        position: [cx, curb, cz],
                        proxy: ([cx, curb + total_height / 2., cz], [fw, total_height, fd]),
                    });
                }
            }
        }
    }
    let furniture = furniture(&l, &mut random);
    City {
        towers,
        slabs,
        furniture,
    }
}
/// The four curb edges of a block: start corner, direction along the
/// edge, outward normal and length.
fn block_edges(x: f64, z: f64, w: f64, d: f64) -> [(f64, f64, f64, f64, f64, f64, f64); 4] {
    [
        (x, z, 1., 0., 0., -1., w),
        (x, z + d, 1., 0., 0., 1., w),
        (x, z, 0., 1., -1., 0., d),
        (x + w, z, 0., 1., 1., 0., d),
    ]
}
/// Math.round: halves round up.
fn round(x: f64) -> f64 {
    (x + 0.5).floor()
}
/// Math.sign( v ) || 1.
fn sign_or_one(v: f64) -> f64 {
    if v > 0. {
        1.
    } else if v < 0. {
        -1.
    } else {
        1.
    }
}
fn furniture(l: &Layout, random: &mut impl FnMut() -> f64) -> Furniture {
    let top = CURB_HEIGHT;
    let sw = l.sidewalk;
    let mut f = Furniture::default();
    for bx in 0..l.blocks_x {
        for bz in 0..l.blocks_z {
            let block_x = -l.city_w / 2. + bx as f64 * (l.block_w + l.street);
            let block_z = -l.city_d / 2. + bz as f64 * (l.block_d + l.street);
            for (x0, z0, dx, dz, nx, nz, len) in block_edges(block_x, block_z, l.block_w, l.block_d)
            {
                let fdir = sign_or_one(nx + nz);
                let on_walk = |t: f64, lateral: f64| {
                    place(
                        x0 + dx * t - nx * lateral,
                        top,
                        z0 + dz * t - nz * lateral,
                        nx,
                        nz,
                    )
                };
                let lc = round(len / 30.).max(1.) as usize;
                for i in 0..lc {
                    f.lights
                        .push(on_walk(len * (i as f64 + 0.5) / lc as f64, 0.8));
                }
                let tc = round(len / 16.).max(1.) as usize;
                for i in 0..tc {
                    let t = len * (i as f64 + 0.5) / tc as f64 + 5.;
                    if t > 7. && t < len - 7. && random() < 0.85 {
                        let yaw = random() * PI * 2.;
                        let scale = 0.8 + random() * 0.5;
                        f.trees.push(place_yaw_scale(
                            x0 + dx * t - nx * 1.5,
                            top,
                            z0 + dz * t - nz * 1.5,
                            yaw,
                            scale,
                        ));
                    }
                }
                let hyd_t = len * (0.25 + random() * 0.5);
                f.hydrants.push(on_walk(hyd_t, 0.7));
                if random() < 0.4 {
                    let t = len * (0.3 + random() * 0.4);
                    f.benches.push(on_walk(t, 1.7));
                }
                let pc = round(len / 9.).max(2.) as usize;
                for i in 0..pc {
                    if random() < 0.7 {
                        let t = len * (i as f64 + random()) / pc as f64;
                        let lateral = 0.9 + random() * (sw - 2.);
                        let yaw = random() * PI * 2.;
                        let scale = 0.92 + random() * 0.16;
                        f.people.push(place_yaw_scale(
                            x0 + dx * t - nx * lateral,
                            top,
                            z0 + dz * t - nz * lateral,
                            yaw,
                            scale,
                        ));
                    }
                }
                let car_color = |random: &mut dyn FnMut() -> f64| {
                    let r = random();
                    for (threshold, color) in CAR_COLORS {
                        if r < threshold {
                            return color;
                        }
                    }
                    CAR_COLORS[CAR_COLORS.len() - 1].1
                };
                let mut t = 9.;
                while t < len - 9. {
                    if (t - hyd_t).abs() > 3. && random() < 0.72 {
                        let matrix = place(
                            x0 + dx * t + nx * 1.5,
                            0.,
                            z0 + dz * t + nz * 1.5,
                            dx * fdir,
                            dz * fdir,
                        );
                        let color = car_color(random);
                        f.cars.push(Car { matrix, color });
                    }
                    t += 5.8;
                }
                let mut t = 14.;
                while t < len - 14. {
                    if random() < 0.4 {
                        let matrix = place(
                            x0 + dx * t + nx * 5.5,
                            0.,
                            z0 + dz * t + nz * 5.5,
                            dx * fdir,
                            dz * fdir,
                        );
                        let color = car_color(random);
                        f.cars.push(Car { matrix, color });
                    }
                    t += 13.;
                }
            }
            for (cx, cz) in [(0., 0.), (1., 0.), (0., 1.), (1., 1.)] {
                let x = block_x + cx * l.block_w;
                let z = block_z + cz * l.block_d;
                let ix = if cx != 0. { -1. } else { 1. };
                let iz = if cz != 0. { -1. } else { 1. };
                if cx == cz {
                    f.signals
                        .push(place(x + ix * 1.4, top, z + iz * 1.4, -ix, -iz));
                }
                f.cans.push(place(x + ix * 2.2, top, z + iz * 2.2, ix, iz));
            }
        }
    }
    f
}
/// The car's body type: taxis by paint, the rest by a hash of the index.
pub(super) fn car_type(index: usize, color: u32) -> usize {
    if color == TAXI_COLOR {
        2
    } else if ((index as u64 * 2654435761) & 0xffffffff) % 100 < 42 {
        1
    } else {
        0
    }
}
/// PersonGenerator.build's per-placement scale and pose: ( matrix, walking ).
pub(super) fn pose_person(index: usize, placement: &M4) -> (M4, bool) {
    let h = |k: u64| ((index as u64 * k) & 0xffffffff) % 1000;
    let h1 = h(2654435761) as f64 / 1000.;
    let h2 = h(1597334677) as f64 / 1000.;
    let h3 = h(3812015801) as f64 / 1000.;
    let scale = 0.92 + h2 * 0.15;
    let width = scale * (0.9 + h3 * 0.2);
    (
        multiply(placement, &scaling([width, scale, width])),
        h1 < 0.65,
    )
}
/// roundedRect( width, depth, radius ).
fn rounded_rect(width: f64, depth: f64, radius: f64) -> Path {
    let (w, d) = (width / 2., depth / 2.);
    let r = radius.min(w).min(d);
    let mut s = Path::default();
    s.move_to([-w + r, -d]);
    s.line_to([w - r, -d]);
    s.quadratic_to([w, -d], [w, -d + r]);
    s.line_to([w, d - r]);
    s.quadratic_to([w, d], [w - r, d]);
    s.line_to([-w + r, d]);
    s.quadratic_to([-w, d], [-w, d - r]);
    s.line_to([-w, -d + r]);
    s.quadratic_to([-w, -d], [-w + r, -d]);
    s
}
/// extrudeUp( shape, height ): ExtrudeGeometry ( curveSegments 6, no
/// bevel ) turned to rise along +Y.
fn extrude_up(shape: Shape, height: f64) -> G {
    let (position, _) = extrude(
        &[shape],
        &Extrude {
            curve_segments: 6,
            steps: 1,
            depth: height,
            bevel: None,
        },
    );
    let mut g = G {
        attributes: vec![("position", 3, position)],
        index: None,
    };
    g.compute_vertex_normals();
    g.rotate_x(-PI / 2.)
}
/// SidewalkGenerator's slab and curb ( width 90, depth 60, curbWidth 0.13,
/// curbLip 0.01 ).
pub(super) fn sidewalk() -> (G, G) {
    let l = layout();
    let (width, depth, height, radius) = (l.block_w, l.block_d, CURB_HEIGHT, CURB_RADIUS);
    let (curb_width, curb_lip) = (0.13, 0.01);
    let inner = (radius - curb_width).max(0.5);
    let slab = extrude_up(
        Shape {
            outline: rounded_rect(
                width - 2. * curb_width + 0.06,
                depth - 2. * curb_width + 0.06,
                inner,
            ),
            holes: vec![],
        },
        height,
    );
    let curb = extrude_up(
        Shape {
            outline: rounded_rect(width, depth, radius),
            holes: vec![rounded_rect(
                width - 2. * curb_width,
                depth - 2. * curb_width,
                inner,
            )],
        },
        height + curb_lip,
    );
    (slab, curb)
}
