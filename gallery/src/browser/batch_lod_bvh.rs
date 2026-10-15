//! webgl_batch_lod_bvh: 500,000 instances of ten torus knots in one
//! BatchedMesh ( MeshStandardMaterial, metalness 1, roughness 0.8, under the
//! RoomEnvironment PMREM with ACES Filmic at exposure 0.8 ), placed on a
//! 708 × 708 grid with the page's random rotations and HSL colors. Each knot
//! carries the four LODs @three.ez/simplify-geometry builds with meshoptimizer
//! ( baked by tools/tsl/prepare-batch-lod.mjs; the knots themselves are built
//! at load ) in one shared index buffer. As @three.ez/batched-mesh-extensions'
//! onBeforeRender does, every frame culls the instances' boxes against the
//! frustum through a BVH ( bvh.js's single-precision planes and boxes ), picks
//! each visible instance's LOD from its screen-size metric and orders the list
//! front to back. The instances' matrices and colors stay resident; the port
//! draws each knot and LOD's visible instances with one instanced draw, where
//! WebGL issues one multi-draw entry per instance. Pointer moves raycast the
//! instance boxes and the knots' triangles ( three-mesh-bvh's first hit ) on
//! the CPU, as the original does, and color the hovered instance red.
use super::controls_attributes::{Controls, camera_state, viewport_css};
use super::gltf_viewer::fetch;
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, renderer::Renderer, scene::*,
};
use std::{f64::consts::PI, sync::Arc};

const INSTANCES: usize = 500_000;
const KNOTS: [(u32, u32); 10] = [
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (2, 1),
    (2, 3),
    (3, 1),
    (4, 1),
    (5, 3),
];
/// addGeometryLOD's screen-height ratios.
const METRICS: [f64; 4] = [0.08, 0.04, 0.033, 0.02];
const LODS: usize = METRICS.len() + 1;
const FOV: f64 = 50.;
const NEAR: f64 = 0.1;
const FAR: f64 = 1000.;
/// The fixture's Math.random: a seeded LCG.
struct Lcg(u32);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 as f64 / 4294967296.
    }
}
/// A box as bvh.js stores it: min x, max x, min y, max y, min z, max z.
type Box6 = [f32; 6];
struct BvhNode {
    bounds: Box6,
    /// Children, or ( count > 0 ) an item range.
    left: u32,
    right: u32,
    start: u32,
    count: u32,
}
/// A top-down BVH over boxes ( the instances, or a knot's triangles ).
struct Bvh {
    nodes: Vec<BvhNode>,
    items: Vec<u32>,
}
impl Bvh {
    fn build(boxes: &[Box6]) -> Self {
        let mut bvh = Self {
            nodes: Vec::with_capacity(boxes.len() / 2),
            items: (0..boxes.len() as u32).collect(),
        };
        let centers: Vec<[f32; 3]> = boxes
            .iter()
            .map(|b| [b[0] + b[1], b[2] + b[3], b[4] + b[5]])
            .collect();
        if !boxes.is_empty() {
            bvh.split(boxes, &centers, 0, boxes.len());
        }
        bvh
    }
    fn split(&mut self, boxes: &[Box6], centers: &[[f32; 3]], start: usize, end: usize) -> u32 {
        let mut bounds = [f32::MAX, f32::MIN, f32::MAX, f32::MIN, f32::MAX, f32::MIN];
        for &i in &self.items[start..end] {
            let b = boxes[i as usize];
            for k in 0..3 {
                bounds[2 * k] = bounds[2 * k].min(b[2 * k]);
                bounds[2 * k + 1] = bounds[2 * k + 1].max(b[2 * k + 1]);
            }
        }
        let node = self.nodes.len();
        self.nodes.push(BvhNode {
            bounds,
            left: 0,
            right: 0,
            start: start as u32,
            count: (end - start) as u32,
        });
        if end - start <= 4 {
            return node as u32;
        }
        let axis = (0..3)
            .max_by(|&a, &b| {
                (bounds[2 * a + 1] - bounds[2 * a]).total_cmp(&(bounds[2 * b + 1] - bounds[2 * b]))
            })
            .unwrap_or(0);
        let middle = (start + end) / 2;
        self.items[start..end].select_nth_unstable_by(middle - start, |&a, &b| {
            centers[a as usize][axis].total_cmp(&centers[b as usize][axis])
        });
        let left = self.split(boxes, centers, start, middle);
        let right = self.split(boxes, centers, middle, end);
        let n = &mut self.nodes[node];
        (n.left, n.right, n.count) = (left, right, 0);
        node as u32
    }
    /// bvh.js frustumCulling: a node fully inside a plane drops it from the
    /// mask; a leaf's items pass when their boxes reach inside every plane left.
    fn cull(&self, planes: &[[f32; 4]; 6], boxes: &[Box6], visit: &mut impl FnMut(u32)) {
        if self.nodes.is_empty() {
            return;
        }
        let mut stack = vec![(0u32, 63u32)];
        while let Some((node, mut mask)) = stack.pop() {
            let n = &self.nodes[node as usize];
            if n.count > 0 {
                for &i in &self.items[n.start as usize..(n.start + n.count) as usize] {
                    if intersected(planes, &boxes[i as usize], mask) {
                        visit(i);
                    }
                }
                continue;
            }
            match box_mask(planes, &n.bounds, mask) {
                None => continue,
                Some(m) => mask = m,
            }
            // Depth first, left before right.
            stack.push((n.right, mask));
            stack.push((n.left, mask));
        }
    }
    /// bvh.js rayIntersections: the leaves' items whose boxes the ray meets
    /// between near and far.
    fn ray(
        &self,
        origin: [f32; 3],
        direction: [f32; 3],
        boxes: &[Box6],
        visit: &mut impl FnMut(u32),
    ) {
        if self.nodes.is_empty() {
            return;
        }
        let inverse = direction.map(|d| 1. / d);
        let sign = inverse.map(|d| usize::from(d < 0.));
        let mut stack = vec![0u32];
        while let Some(node) = stack.pop() {
            let n = &self.nodes[node as usize];
            if !slab(&n.bounds, origin, inverse, sign) {
                continue;
            }
            if n.count > 0 {
                for &i in &self.items[n.start as usize..(n.start + n.count) as usize] {
                    if slab(&boxes[i as usize], origin, inverse, sign) {
                        visit(i);
                    }
                }
                continue;
            }
            stack.push(n.right);
            stack.push(n.left);
        }
    }
}
/// bvh.js intersectsBoxMask: None when outside a plane, else the planes the
/// box does not lie fully inside.
fn box_mask(planes: &[[f32; 4]; 6], b: &Box6, mut mask: u32) -> Option<u32> {
    for (u, p) in planes.iter().enumerate() {
        if mask & (32 >> u) == 0 {
            continue;
        }
        let [x, y, z, w] = p.map(f64::from);
        let (px, nx) = if x > 0. { (b[1], b[0]) } else { (b[0], b[1]) };
        let (py, ny) = if y > 0. { (b[3], b[2]) } else { (b[2], b[3]) };
        let (pz, nz) = if z > 0. { (b[5], b[4]) } else { (b[4], b[5]) };
        if x * f64::from(px) + y * f64::from(py) + z * f64::from(pz) < -w {
            return None;
        }
        if x * f64::from(nx) + y * f64::from(ny) + z * f64::from(nz) > -w {
            mask ^= 32 >> u;
        }
    }
    Some(mask)
}
/// bvh.js isIntersected: the box's far corner along each masked plane is not behind it.
fn intersected(planes: &[[f32; 4]; 6], b: &Box6, mask: u32) -> bool {
    planes.iter().enumerate().all(|(u, p)| {
        if mask & (32 >> u) == 0 {
            return true;
        }
        let [x, y, z, w] = p.map(f64::from);
        let px = if x > 0. { b[1] } else { b[0] };
        let py = if y > 0. { b[3] } else { b[2] };
        let pz = if z > 0. { b[5] } else { b[4] };
        x * f64::from(px) + y * f64::from(py) + z * f64::from(pz) >= -w
    })
}
/// bvh.js's ray-box slab test from the origin ( near 0, far infinity ).
fn slab(b: &Box6, origin: [f32; 3], inverse: [f32; 3], sign: [usize; 3]) -> bool {
    let o = origin.map(f64::from);
    let l = inverse.map(f64::from);
    let e = b.map(f64::from);
    let s = sign[0];
    let c = (e[s] - o[0]) * l[0];
    let u = (e[s ^ 1] - o[0]) * l[0];
    let mut near = if c > 0. { c } else { 0. };
    let mut far = if u < f64::INFINITY { u } else { f64::INFINITY };
    for k in 1..3 {
        let s = sign[k];
        let c = (e[s + 2 * k] - o[k]) * l[k];
        if c > far {
            return false;
        }
        let u = (e[(s ^ 1) + 2 * k] - o[k]) * l[k];
        if near > u {
            return false;
        }
        near = if c > near { c } else { near };
        far = if u < far { u } else { far };
    }
    near <= f64::INFINITY && far >= 0.
}
/// One torus knot: its LOD index ranges, bounds and triangles ( LOD 0 ) for raycasts.
struct Knot {
    /// getBoundingBoxAt and getBoundingSphereAt.
    min: Vector3,
    max: Vector3,
    center: Vector3,
    radius: f64,
    triangles: Vec<[Vector3; 3]>,
    blas: Bvh,
    blas_boxes: Vec<Box6>,
}
/// Per instance: its knot, Float32 matrix ( as BatchedMesh's matrices texture
/// holds it ) and color.
struct InstanceData {
    knot: usize,
    matrix: [f32; 16],
    color: Color,
}
pub(super) struct Demo {
    controls: Controls,
    knots: Vec<Knot>,
    instances: Vec<InstanceData>,
    boxes: Vec<Box6>,
    tlas: Bvh,
    /// Per knot and LOD: its node and the visible instances, front to back.
    nodes: Vec<Object3D>,
    lists: Vec<Vec<u32>>,
    /// freeze, useBVH, useLOD.
    freeze: bool,
    use_bvh: bool,
    use_lod: bool,
    hovered: Option<u32>,
    hovered_color: Color,
    /// Pointer events in CSS pixels, and MapControls' grab: the ground plane's
    /// height and the grabbed point.
    queue: Vec<(u32, f64, f64)>,
    grab: Option<(f64, Vector3)>,
}
/// three's Matrix4.compose with unit scale.
fn compose(position: Vector3, q: [f64; 4]) -> [f64; 16] {
    let [x, y, z, w] = q;
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    [
        1. - (yy + zz),
        xy + wz,
        xz - wy,
        0.,
        xy - wz,
        1. - (xx + zz),
        yz + wx,
        0.,
        xz + wy,
        yz - wx,
        1. - (xx + yy),
        0.,
        position.x,
        position.y,
        position.z,
        1.,
    ]
}
/// Vector3.applyMatrix4 with the matrix's Float32 elements.
fn apply(m: &[f32; 16], v: Vector3) -> Vector3 {
    let e = m.map(f64::from);
    let w = 1. / (e[3] * v.x + e[7] * v.y + e[11] * v.z + e[15]);
    Vector3::new(
        (e[0] * v.x + e[4] * v.y + e[8] * v.z + e[12]) * w,
        (e[1] * v.x + e[5] * v.y + e[9] * v.z + e[13]) * w,
        (e[2] * v.x + e[6] * v.y + e[10] * v.z + e[14]) * w,
    )
}
/// Box3.applyMatrix4, stored in single precision.
fn transformed_box(m: &[f32; 16], min: Vector3, max: Vector3) -> Box6 {
    let mut lo = Vector3::splat(f64::INFINITY);
    let mut hi = Vector3::splat(f64::NEG_INFINITY);
    for k in 0..8 {
        let p = Vector3::new(
            if k & 4 != 0 { max.x } else { min.x },
            if k & 2 != 0 { max.y } else { min.y },
            if k & 1 != 0 { max.z } else { min.z },
        );
        let q = apply(m, p);
        lo = lo.min(q);
        hi = hi.max(q);
    }
    [
        lo.x as f32,
        hi.x as f32,
        lo.y as f32,
        hi.y as f32,
        lo.z as f32,
        hi.z as f32,
    ]
}
/// The camera's WebGL projection ( PerspectiveCamera.updateProjectionMatrix ).
fn projection(aspect: f64) -> [f64; 16] {
    let top = NEAR * (PI / 180. * 0.5 * FOV).tan();
    let height = 2. * top;
    let width = aspect * height;
    let left = -0.5 * width;
    let (right, bottom) = (left + width, top - height);
    let x = 2. * NEAR / (right - left);
    let y = 2. * NEAR / (top - bottom);
    let a = (right + left) / (right - left);
    let b = (top + bottom) / (top - bottom);
    let c = -(FAR + NEAR) / (FAR - NEAR);
    let d = -2. * FAR * NEAR / (FAR - NEAR);
    [x, 0., 0., 0., 0., y, 0., 0., a, b, c, -1., 0., 0., d, 0.]
}
/// three's Matrix4.multiplyMatrices on column-major elements.
fn multiply(a: &[f64; 16], b: &[f64; 16]) -> [f64; 16] {
    let mut out = [0.; 16];
    for col in 0..4 {
        for row in 0..4 {
            out[col * 4 + row] = a[row] * b[col * 4]
                + a[4 + row] * b[col * 4 + 1]
                + a[8 + row] * b[col * 4 + 2]
                + a[12 + row] * b[col * 4 + 3];
        }
    }
    out
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: FOV,
            near: NEAR,
            far: FAR,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 20., 55.);
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 0.8;
        s.background = Color::BLACK;
        s.environment = Some(super::room_environment::environment(r)?);
        // MapControls: planar panning, the polar angle limited to the horizon.
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI / 2., false);
        controls.update(s, c)?;
        let bake = fetch("/web/gallery/assets/batch_lod/lods.bin").await?;
        let mut at = 0;
        let word = |at: &mut usize| -> Result<u32> {
            let b = bake
                .get(*at..*at + 4)
                .ok_or(Error::Invalid("batch lod bake"))?;
            *at += 4;
            Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        };
        if word(&mut at)? as usize != KNOTS.len() || word(&mut at)? as usize != METRICS.len() {
            return Err(Error::Invalid("batch lod bake"));
        }
        let material = Arc::new(Material::Standard(MeshStandardMaterial {
            metalness: 1.,
            roughness: 0.8,
            ..Default::default()
        }));
        let mut knots = vec![];
        let mut nodes = vec![];
        for (p, q) in KNOTS {
            let mut geometry = TorusKnotGeometry::build(1., 0.4, 256, 32, p, q)?;
            let position = geometry
                .attributes
                .get("position")
                .ok_or(Error::Invalid("torus knot position"))?;
            let count = position.count();
            let points: Vec<Vector3> = (0..count)
                .map(|i| position.vector3(i))
                .collect::<Result<_>>()?;
            // The baked LODs index these vertices: the knot must match three's.
            let mut hash = 0x811c9dc5u32;
            for v in &points {
                for c in [v.x, v.y, v.z] {
                    for b in (c as f32).to_le_bytes() {
                        hash = (hash ^ b as u32).wrapping_mul(16777619);
                    }
                }
            }
            if word(&mut at)? != hash || word(&mut at)? as usize != count {
                return Err(Error::Invalid("torus knot differs from the baked LODs"));
            }
            let mut index = geometry
                .index
                .clone()
                .ok_or(Error::Invalid("torus knot index"))?;
            let mut lods = [(0, index.len()); LODS];
            for lod in lods.iter_mut().skip(1) {
                let n = word(&mut at)? as usize;
                let bytes = bake
                    .get(at..at + n * 2)
                    .ok_or(Error::Invalid("batch lod bake"))?;
                *lod = (index.len(), n);
                index.extend(
                    bytes
                        .chunks(2)
                        .map(|b| u16::from_le_bytes([b[0], b[1]]) as u32),
                );
                at += (n * 2).div_ceil(4) * 4;
            }
            let lod0 = &index[..lods[0].1];
            // getBoundingBoxAt / getBoundingSphereAt over LOD 0's index range.
            let (mut min, mut max) = (
                Vector3::splat(f64::INFINITY),
                Vector3::splat(f64::NEG_INFINITY),
            );
            for &i in lod0 {
                min = min.min(points[i as usize]);
                max = max.max(points[i as usize]);
            }
            let center = (min + max) * 0.5;
            let radius = lod0
                .iter()
                .map(|&i| center.distance_squared(points[i as usize]))
                .fold(0., f64::max)
                .sqrt();
            let triangles: Vec<[Vector3; 3]> = lod0
                .chunks(3)
                .map(|t| [t[0], t[1], t[2]].map(|i| points[i as usize]))
                .collect();
            let blas_boxes: Vec<Box6> = triangles
                .iter()
                .map(|t| {
                    let lo = t[0].min(t[1]).min(t[2]);
                    let hi = t[0].max(t[1]).max(t[2]);
                    [lo.x, hi.x, lo.y, hi.y, lo.z, hi.z].map(|v| v as f32)
                })
                .collect();
            geometry.index = Some(index);
            let geometry = Arc::new(geometry);
            for &(start, count) in &lods {
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    geometry.clone(),
                    material.clone(),
                )));
                let n = s.get_mut(h)?;
                n.draw_range = Some(DrawRange {
                    start,
                    count: Some(count),
                });
                n.frustum_culled = false;
                n.visible = false;
                nodes.push(h);
            }
            knots.push(Knot {
                min,
                max,
                center,
                radius,
                blas: Bvh::build(&blas_boxes),
                blas_boxes,
                triangles,
            });
        }
        // The instances on a 2D grid with random knots, rotations and colors.
        let mut random = Lcg(186);
        let side = (INSTANCES as f64).sqrt().ceil() as usize;
        let size = 5.5;
        let start = (side as f64 / -2. * size) + (size / 2.);
        let mut instances = Vec::with_capacity(INSTANCES);
        let mut boxes = Vec::with_capacity(INSTANCES);
        for i in 0..INSTANCES {
            let (row, column) = (i / side, i % side);
            let knot = (random.next() * KNOTS.len() as f64).floor() as usize;
            let position =
                Vector3::new(column as f64 * size + start, 0., row as f64 * size + start);
            // Quaternion.random(): Ken Shoemake's uniform rotations.
            let theta1 = 2. * PI * random.next();
            let theta2 = 2. * PI * random.next();
            let x0 = random.next();
            let (r1, r2) = ((1. - x0).sqrt(), x0.sqrt());
            let q = [
                r1 * theta1.sin(),
                r1 * theta1.cos(),
                r2 * theta2.sin(),
                r2 * theta2.cos(),
            ];
            let matrix = compose(position, q).map(|v| v as f32);
            let color = Color::from_hsl(random.next(), 0.6, 0.5);
            // The first setColorAt creates the colors texture: its texture and
            // source UUIDs draw four randoms each.
            if i == 0 {
                for _ in 0..8 {
                    random.next();
                }
            }
            let k = &knots[knot];
            boxes.push(transformed_box(&matrix, k.min, k.max));
            instances.push(InstanceData {
                knot,
                matrix,
                color: Color(color.0.map(|v| v as f32 as f64)),
            });
        }
        let tlas = Bvh::build(&boxes);
        Ok(Self {
            controls,
            knots,
            instances,
            boxes,
            tlas,
            lists: vec![vec![]; nodes.len()],
            nodes,
            freeze: false,
            use_bvh: true,
            use_lod: true,
            hovered: None,
            hovered_color: Color::WHITE,
            queue: vec![],
            grab: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// The camera's world matrix, its inverse and aspect.
    fn camera(s: &Scene, c: Object3D) -> Result<(Matrix4, f64)> {
        let (camera, world) = s.camera(c)?;
        let aspect = match camera {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        Ok((world, aspect))
    }
    /// onBeforeRender: frustum culling ( BVH or per-instance spheres ), LODs
    /// and the front-to-back order, then each node's instances.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.drain(s, c)?;
        self.controls.update(s, c)?;
        s.update()?;
        if !self.freeze {
            self.cull(s, c)?;
        }
        for (list, &h) in self.lists.iter().zip(&self.nodes) {
            let n = s.get_mut(h)?;
            n.visible = !list.is_empty();
            if list.is_empty() {
                continue;
            }
            // Grown capacity with headroom: the resident instance buffer is
            // recreated only when a view needs more than twice what any had.
            if n.instances.len() < list.len() {
                let capacity = (list.len() * 2).next_power_of_two().max(256);
                n.instances.resize(capacity, Instance::default());
            }
            for (slot, &i) in n.instances.iter_mut().zip(list) {
                let d = &self.instances[i as usize];
                *slot = Instance {
                    matrix: Matrix4::from_cols_array(&d.matrix.map(f64::from)),
                    color: d.color,
                };
            }
            n.instance_count = Some(list.len() as u32);
        }
        Ok(())
    }
    fn cull(&mut self, s: &Scene, c: Object3D) -> Result<()> {
        let (world, aspect) = Self::camera(s, c)?;
        let view = world.inverse().to_cols_array();
        let w = multiply(&projection(aspect), &view);
        let camera = world.transform_point3(Vector3::ZERO);
        let forward = world.transform_vector3(Vector3::NEG_Z).normalize();
        let tan2 = (FOV * 0.5 * (PI / 180.)).tan().powi(2);
        let squared = METRICS.map(|m| m * m);
        let lod = |metric: f64| -> usize {
            (1..LODS)
                .rev()
                .find(|&s| metric <= squared[s - 1])
                .unwrap_or(0)
        };
        for list in &mut self.lists {
            list.clear();
        }
        let mut visible: Vec<(u32, usize, f64)> = vec![];
        if self.use_bvh {
            // bvh.js Frustum.setFromProjectionMatrix ( WebGL ), in single precision.
            let plane = |a: f64, b: f64, c: f64, d: f64| {
                let l = (a * a + b * b + c * c).sqrt();
                [
                    (a / l) as f32,
                    (b / l) as f32,
                    (c / l) as f32,
                    (d / l) as f32,
                ]
            };
            let t = &w;
            let planes = [
                plane(t[3] + t[0], t[7] + t[4], t[11] + t[8], t[15] + t[12]),
                plane(t[3] - t[0], t[7] - t[4], t[11] - t[8], t[15] - t[12]),
                plane(t[3] - t[1], t[7] - t[5], t[11] - t[9], t[15] - t[13]),
                plane(t[3] + t[1], t[7] + t[5], t[11] + t[9], t[15] + t[13]),
                plane(t[3] - t[2], t[7] - t[6], t[11] - t[10], t[15] - t[14]),
                plane(t[3] + t[2], t[7] + t[6], t[11] + t[10], t[15] + t[14]),
            ];
            let (instances, knots, use_lod) = (&self.instances, &self.knots, self.use_lod);
            self.tlas.cull(&planes, &self.boxes, &mut |i| {
                let d = &instances[i as usize];
                let p = Vector3::new(
                    d.matrix[12] as f64,
                    d.matrix[13] as f64,
                    d.matrix[14] as f64,
                );
                let level = if use_lod {
                    let radius = knots[d.knot].radius;
                    lod(radius.powi(2) / (p.distance_squared(camera) * tan2))
                } else {
                    0
                };
                visible.push((i, d.knot * LODS + level, (p - camera).dot(forward)));
            });
        } else {
            // linearCulling: each instance's bounding sphere against three's Frustum.
            let t = &w;
            let plane = |a: f64, b: f64, c: f64, d: f64| {
                let inverse = 1. / (a * a + b * b + c * c).sqrt();
                [a * inverse, b * inverse, c * inverse, d * inverse]
            };
            let planes = [
                plane(t[3] - t[0], t[7] - t[4], t[11] - t[8], t[15] - t[12]),
                plane(t[3] + t[0], t[7] + t[4], t[11] + t[8], t[15] + t[12]),
                plane(t[3] + t[1], t[7] + t[5], t[11] + t[9], t[15] + t[13]),
                plane(t[3] - t[1], t[7] - t[5], t[11] - t[9], t[15] - t[13]),
                plane(t[3] - t[2], t[7] - t[6], t[11] - t[10], t[15] - t[14]),
                plane(t[3] + t[2], t[7] + t[6], t[11] + t[10], t[15] + t[14]),
            ];
            for (i, d) in self.instances.iter().enumerate() {
                let k = &self.knots[d.knot];
                let m = d.matrix.map(f64::from);
                let center = apply(&d.matrix, k.center);
                let scale = [
                    m[0] * m[0] + m[1] * m[1] + m[2] * m[2],
                    m[4] * m[4] + m[5] * m[5] + m[6] * m[6],
                    m[8] * m[8] + m[9] * m[9] + m[10] * m[10],
                ];
                let radius = k.radius * scale[0].max(scale[1]).max(scale[2]).sqrt();
                if planes
                    .iter()
                    .any(|p| p[0] * center.x + p[1] * center.y + p[2] * center.z + p[3] < -radius)
                {
                    continue;
                }
                let level = if self.use_lod {
                    lod(radius.powi(2) / (center.distance_squared(camera) * tan2))
                } else {
                    0
                };
                visible.push((
                    i as u32,
                    d.knot * LODS + level,
                    (center - camera).dot(forward),
                ));
            }
        }
        // The radix sort's front-to-back order ( stable ), kept within each draw.
        visible.sort_by(|a, b| a.2.total_cmp(&b.2));
        for (i, slot, _) in visible {
            self.lists[slot].push(i);
        }
        Ok(())
    }
    /// Pointer events, in CSS pixels.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.queue.push((kind, x, y));
    }
    /// The ray Raycaster.setFromCamera casts through a pointer.
    fn pointer_ray(s: &Scene, c: Object3D, x: f64, y: f64) -> Result<(Vector3, Vector3)> {
        let (w, h, _) = viewport_css();
        let mouse = Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.);
        let (world, aspect) = Self::camera(s, c)?;
        let origin = world.transform_point3(Vector3::ZERO);
        let inverse = Matrix4::from_cols_array(&projection(aspect)).inverse();
        let target = world.transform_point3(inverse.project_point3(mouse.extend(0.5)));
        Ok((origin, (target - origin).normalize()))
    }
    /// The pointer listeners in their order: the page's document pointermove
    /// raycasts with the camera before MapControls pans for the same move.
    /// MapControls grabs the point under the pointer on the plane through the
    /// target ( normal up ) at the press, and each move pans so that point
    /// stays under the pointer.
    fn drain(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let events = std::mem::take(&mut self.queue);
        let mut hover = None;
        for (kind, x, y) in events {
            match kind {
                0 => {
                    hover = Some(Self::pointer_ray(s, c, x, y)?);
                    if let Some((height, start)) = self.grab {
                        let (o, d) = Self::pointer_ray(s, c, x, y)?;
                        if let Some(current) = ground(o, d, height) {
                            self.controls.set_pan(-(current - start));
                            self.controls.update(s, c)?;
                            s.update()?;
                        }
                    }
                }
                10 => {
                    self.controls.set_pan(Vector3::ZERO);
                    let height = self.controls.target().y;
                    let (o, d) = Self::pointer_ray(s, c, x, y)?;
                    // A missed plane keeps the previous grab point, as _panWorldStart does.
                    let start = ground(o, d, height)
                        .or(self.grab.map(|g| g.1))
                        .unwrap_or(Vector3::ZERO);
                    self.grab = Some((height, start));
                }
                20..=22 => self.grab = None,
                _ => {}
            }
        }
        if let Some((origin, direction)) = hover {
            self.raycast(origin, direction);
        }
        Ok(())
    }
    /// raycast(): the hovered instance turns red.
    fn raycast(&mut self, origin: Vector3, direction: Vector3) {
        let mut best: Option<(f64, u32)> = None;
        let mut test = |i: u32| {
            let d = &self.instances[i as usize];
            let k = &self.knots[d.knot];
            let m = Matrix4::from_cols_array(&d.matrix.map(f64::from));
            let local = m.inverse();
            let o = local.transform_point3(origin);
            let dir = local.transform_vector3(direction).normalize();
            let o32 = [o.x as f32, o.y as f32, o.z as f32];
            let d32 = [dir.x as f32, dir.y as f32, dir.z as f32];
            let mut hit: Option<f64> = None;
            k.blas.ray(o32, d32, &k.blas_boxes, &mut |t| {
                let [a, b, c] = k.triangles[t as usize];
                if let Some(t) = triangle(o, dir, a, b, c) {
                    let point = m.transform_point3(o + dir * t);
                    let distance = point.distance(origin);
                    if hit.is_none_or(|h| distance < h) {
                        hit = Some(distance);
                    }
                }
            });
            if let Some(distance) = hit
                && best.is_none_or(|(b, _)| distance < b)
            {
                best = Some((distance, i));
            }
        };
        if self.use_bvh {
            let o = [origin.x as f32, origin.y as f32, origin.z as f32];
            let d = [direction.x as f32, direction.y as f32, direction.z as f32];
            let tlas = &self.tlas;
            let boxes = &self.boxes;
            tlas.ray(o, d, boxes, &mut test);
        } else {
            for i in 0..self.instances.len() as u32 {
                test(i);
            }
        }
        let batch = best.map(|(_, i)| i);
        if self.hovered == batch {
            return;
        }
        // The page tests the ids for truthiness: instance 0 is never recolored.
        if let Some(previous) = self.hovered.filter(|&i| i != 0) {
            self.instances[previous as usize].color = self.hovered_color;
        }
        if let Some(i) = batch.filter(|&i| i != 0) {
            self.hovered_color = self.instances[i as usize].color;
            self.instances[i as usize].color = Color::from_hex(0xff0000);
        }
        self.hovered = batch;
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    /// MapControls: the wheel dollies; left drags pan through draw(), others rotate.
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
        self.drain(s, c)?;
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if self.grab.is_none() && (dx != 0. || dy != 0.) {
            self.controls.rotate(dx, dy, height);
        }
        let _ = pan;
        self.controls.update(s, c)
    }
    /// instanceCount ( disabled ), freeze, useBVH and useLOD.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let on = value != 0.;
        match index {
            0 => {}
            1 => self.freeze = on,
            2 => self.use_bvh = on,
            3 => self.use_lod = on,
            _ => return Err(Error::Invalid("batch lod parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
/// Ray.intersectPlane with the plane y = height.
fn ground(origin: Vector3, direction: Vector3, height: f64) -> Option<Vector3> {
    let plane = Plane::from_normal_point(Vector3::Y, Vector3::new(0., height, 0.));
    Ray { origin, direction }.intersect_plane(plane)
}
/// Ray.intersectTriangle with back-face culling ( FrontSide ): the distance along the ray.
fn triangle(
    origin: Vector3,
    direction: Vector3,
    a: Vector3,
    b: Vector3,
    c: Vector3,
) -> Option<f64> {
    let edge1 = b - a;
    let edge2 = c - a;
    let normal = edge1.cross(edge2);
    let ddn = direction.dot(normal);
    if ddn >= 0. {
        return None;
    }
    let (sign, ddn) = (-1., -ddn);
    let diff = origin - a;
    let dq = sign * direction.dot(diff.cross(edge2));
    if dq < 0. {
        return None;
    }
    let de = sign * direction.dot(edge1.cross(diff));
    if de < 0. || dq + de > ddn {
        return None;
    }
    let qn = -sign * diff.dot(normal);
    if qn < 0. {
        return None;
    }
    Some(qn / ddn)
}
