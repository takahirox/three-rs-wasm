//! webgl_raycaster_bvh: three-mesh-bvh ( v0.9.10, MIT, Garrett Johnson ) on
//! the Stanford bunny: the MeshBVH the page builds once ( CENTER strategy,
//! maxLeafSize 10, Float32 primitive and node bounds, the Hoare partition of
//! the geometry index ), up to 3000 rays from points on a 3.75 sphere
//! turning about time-varying axes and cast toward the origin each frame on
//! the CPU, as the page casts them ( through the BVH, or every triangle when
//! useBVH is off; firstHitOnly only changes how the nearest hit is found ),
//! the origin and hit spheres of a dynamic InstancedMesh, the translucent ray
//! LineSegments with their draw range. BVHHelper never draws under the
//! pinned r186: its BVHRootHelper extends Object3D, whose intersectsFrustum
//! returns undefined, so the renderer culls it while frustumCulled is set;
//! displayHelper and helperDepth change nothing on screen. The bunny is FBXLoader's parse baked by tools/tsl/prepare-bunny.mjs.
//! The page advances its rays and the bunny's turn once per animation frame;
//! the port steps the same per-frame updates at 60 Hz of the example clock.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use super::ssao::Random;
use crate::attribute::BufferAttribute;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/subsurface-scattering";
const MAX_RAYS: usize = 3000;
const RAY_COLOR: u32 = 0x444444;
const FLOAT32_EPSILON: f64 = 1. / 16777216.;
const MAX_LEAF_SIZE: usize = 10;
const MAX_DEPTH: usize = 40;

enum Node {
    Leaf {
        bounds: [f32; 6],
        offset: usize,
        count: usize,
    },
    Inner {
        bounds: [f32; 6],
        left: Box<Node>,
        right: Box<Node>,
    },
}
impl Node {
    fn bounds(&self) -> &[f32; 6] {
        match self {
            Node::Leaf { bounds, .. } | Node::Inner { bounds, .. } => bounds,
        }
    }
}

/// getBounds: the union of the primitives' boxes and of their centroids,
/// each written to a Float32Array.
fn get_bounds(primitive: &[f32], offset: usize, count: usize) -> ([f32; 6], [f32; 6]) {
    let mut b = [
        f64::INFINITY,
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    ];
    let mut c = b;
    for i in offset..offset + count {
        for d in 0..3 {
            let center = f64::from(primitive[i * 6 + 2 * d]);
            let half = f64::from(primitive[i * 6 + 2 * d + 1]);
            let (l, r) = (center - half, center + half);
            if l < b[d] {
                b[d] = l;
            }
            if r > b[d + 3] {
                b[d + 3] = r;
            }
            if center < c[d] {
                c[d] = center;
            }
            if center > c[d + 3] {
                c[d + 3] = center;
            }
        }
    }
    (b.map(|v| v as f32), c.map(|v| v as f32))
}

/// The packed MeshBVH: buildTree's recursion with the CENTER split.
struct Bvh {
    root: Node,
}
impl Bvh {
    fn build(positions: &[f32], index: &mut [u32]) -> Self {
        let triangles = index.len() / 3;
        // computePrimitiveBounds: centre and half extent per axis, padded by
        // a float32 epsilon scaled to the coordinates.
        let mut primitive = vec![0f32; triangles * 6];
        for t in 0..triangles {
            for el in 0..3 {
                let [a, b, c] =
                    [0, 1, 2].map(|k| f64::from(positions[index[t * 3 + k] as usize * 3 + el]));
                let mut min = a;
                if b < min {
                    min = b;
                }
                if c < min {
                    min = c;
                }
                let mut max = a;
                if b > max {
                    max = b;
                }
                if c > max {
                    max = c;
                }
                let half = (max - min) / 2.;
                primitive[t * 6 + el * 2] = (min + half) as f32;
                primitive[t * 6 + el * 2 + 1] =
                    (half + (min.abs() + half) * FLOAT32_EPSILON) as f32;
            }
        }
        let (bounds, centroid) = get_bounds(&primitive, 0, triangles);
        let root = split(index, &mut primitive, bounds, centroid, 0, triangles, 0);
        Self { root }
    }
    /// raycastFirst: the nearest front-facing hit in the local frame.
    fn raycast_first(
        &self,
        positions: &[f32],
        index: &[u32],
        ray: &RayF,
    ) -> Option<(f64, Vector3)> {
        let mut best: Option<(f64, Vector3)> = None;
        let mut stack = vec![&self.root];
        while let Some(node) = stack.pop() {
            let Some(near) = ray.box_distance(node.bounds()) else {
                continue;
            };
            if best.is_some_and(|(d, _)| near > d) {
                continue;
            }
            match node {
                Node::Leaf { offset, count, .. } => {
                    for t in *offset..offset + count {
                        if let Some(hit) = ray.triangle(positions, index, t)
                            && best.is_none_or(|(d, _)| hit.0 < d)
                        {
                            best = Some(hit);
                        }
                    }
                }
                Node::Inner { left, right, .. } => {
                    stack.push(right);
                    stack.push(left);
                }
            }
        }
        best
    }
    fn raycast_all(&self, positions: &[f32], index: &[u32], ray: &RayF) -> Option<(f64, Vector3)> {
        let mut hits = vec![];
        let mut stack = vec![&self.root];
        while let Some(node) = stack.pop() {
            if ray.box_distance(node.bounds()).is_none() {
                continue;
            }
            match node {
                Node::Leaf { offset, count, .. } => {
                    hits.extend(
                        (*offset..offset + count).filter_map(|t| ray.triangle(positions, index, t)),
                    );
                }
                Node::Inner { left, right, .. } => {
                    stack.push(right);
                    stack.push(left);
                }
            }
        }
        hits.into_iter().min_by(|a, b| a.0.total_cmp(&b.0))
    }
}

fn split(
    index: &mut [u32],
    primitive: &mut [f32],
    bounds: [f32; 6],
    centroid: [f32; 6],
    offset: usize,
    count: usize,
    depth: usize,
) -> Node {
    let leaf = Node::Leaf {
        bounds,
        offset,
        count,
    };
    if count <= MAX_LEAF_SIZE || depth >= MAX_DEPTH {
        return leaf;
    }
    // getLongestEdgeIndex of the centroid box, split at its centre.
    let mut axis = None;
    let mut longest = f64::NEG_INFINITY;
    for i in 0..3 {
        let dist = f64::from(centroid[i + 3]) - f64::from(centroid[i]);
        if dist > longest {
            longest = dist;
            axis = Some(i);
        }
    }
    let Some(axis) = axis else {
        return leaf;
    };
    let pos = (f64::from(centroid[axis]) + f64::from(centroid[axis + 3])) / 2.;
    // Hoare partition; centres on the plane go right.
    let mut left = offset as isize;
    let mut right = (offset + count) as isize - 1;
    let key = |primitive: &[f32], i: isize| f64::from(primitive[i as usize * 6 + axis * 2]);
    let split_offset = loop {
        while left <= right && key(primitive, left) < pos {
            left += 1;
        }
        while left <= right && key(primitive, right) >= pos {
            right -= 1;
        }
        if left < right {
            let (l, r) = (left as usize, right as usize);
            for i in 0..3 {
                index.swap(l * 3 + i, r * 3 + i);
            }
            for i in 0..6 {
                primitive.swap(l * 6 + i, r * 6 + i);
            }
            left += 1;
            right -= 1;
        } else {
            break left as usize;
        }
    };
    if split_offset == offset || split_offset == offset + count {
        return leaf;
    }
    let lcount = split_offset - offset;
    let (lb, lc) = get_bounds(primitive, offset, lcount);
    let left = split(index, primitive, lb, lc, offset, lcount, depth + 1);
    let rcount = count - lcount;
    let (rb, rc) = get_bounds(primitive, split_offset, rcount);
    let right = split(index, primitive, rb, rc, split_offset, rcount, depth + 1);
    Node::Inner {
        bounds,
        left: Box::new(left),
        right: Box::new(right),
    }
}

struct RayF {
    origin: Vector3,
    direction: Vector3,
}
impl RayF {
    /// The slab test's entry distance ( 0 inside the box ).
    fn box_distance(&self, b: &[f32; 6]) -> Option<f64> {
        let (mut near, mut far) = (f64::NEG_INFINITY, f64::INFINITY);
        for axis in 0..3 {
            let (o, d) = (self.origin[axis], self.direction[axis]);
            let (lo, hi) = (f64::from(b[axis]), f64::from(b[axis + 3]));
            if d == 0. {
                if o < lo || o > hi {
                    return None;
                }
                continue;
            }
            let (t1, t2) = ((lo - o) / d, (hi - o) / d);
            near = near.max(t1.min(t2));
            far = far.min(t1.max(t2));
            if near > far {
                return None;
            }
        }
        (far >= 0.).then_some(near.max(0.))
    }
    /// Ray.intersectTriangle( a, b, c, backfaceCulling = true ).
    fn triangle(&self, positions: &[f32], index: &[u32], t: usize) -> Option<(f64, Vector3)> {
        let [a, b, c] = [0, 1, 2].map(|k| {
            let i = index[t * 3 + k] as usize * 3;
            Vector3::new(
                f64::from(positions[i]),
                f64::from(positions[i + 1]),
                f64::from(positions[i + 2]),
            )
        });
        let (edge1, edge2) = (b - a, c - a);
        let normal = edge1.cross(edge2);
        let mut ddn = self.direction.dot(normal);
        let sign = if ddn > 0. {
            return None;
        } else if ddn < 0. {
            ddn = -ddn;
            -1.
        } else {
            return None;
        };
        let diff = self.origin - a;
        let ddqxe2 = sign * self.direction.dot(diff.cross(edge2));
        if ddqxe2 < 0. {
            return None;
        }
        let dde1xq = sign * self.direction.dot(edge1.cross(diff));
        if dde1xq < 0. || ddqxe2 + dde1xq > ddn {
            return None;
        }
        let qdn = -sign * diff.dot(normal);
        if qdn < 0. {
            return None;
        }
        let distance = qdn / ddn;
        Some((distance, self.origin + self.direction * distance))
    }
}

/// Vector3.applyAxisAngle: setFromAxisAngle, then applyQuaternion.
fn apply_axis_angle(v: [f64; 3], axis: [f64; 3], angle: f64) -> [f64; 3] {
    let s = (angle / 2.).sin();
    let (qx, qy, qz, qw) = (axis[0] * s, axis[1] * s, axis[2] * s, (angle / 2.).cos());
    let [vx, vy, vz] = v;
    let tx = 2. * (qy * vz - qz * vy);
    let ty = 2. * (qz * vx - qx * vz);
    let tz = 2. * (qx * vy - qy * vx);
    [
        vx + qw * tx + qy * tz - qz * ty,
        vy + qw * ty + qz * tx - qx * tz,
        vz + qw * tz + qx * ty - qy * tx,
    ]
}

fn translation(p: [f64; 3], scale: f64) -> Matrix4 {
    Matrix4::from_scale_rotation_translation(
        Vector3::splat(scale),
        Quaternion::IDENTITY,
        Vector3::from_array(p),
    )
}

pub(super) struct Demo {
    controls: Controls,
    bunny: Object3D,
    spheres: Object3D,
    lines: Object3D,
    positions: Arc<Vec<f32>>,
    index: Vec<u32>,
    bvh: Bvh,
    /// The instance matrices as stored ( Float32 ): origin and hit translations.
    stored: Vec<[f32; 3]>,
    rotation: f64,
    frames: u64,
    time: f64,
    count: usize,
    first_hit_only: bool,
    use_bvh: bool,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 1.,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 10.);
        s.background = Color::from_hex(0xeeeeee);
        let hemisphere = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x999999),
            intensity: 3.,
        }));
        s.get_mut(hemisphere)?.position = Vector3::Y;
        // The ray lines: MAX_RAYS segments, drawn up to the frame's count.
        let mut line_geometry = BufferGeometry::default();
        line_geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(vec![0.; MAX_RAYS * 2 * 3], 3, false)?),
        );
        line_geometry.set_draw_range(0, Some(0));
        let mut line_material = LineBasicMaterial::default();
        line_material.properties.color = Color::from_hex(RAY_COLOR);
        line_material.properties.transparent = true;
        line_material.properties.opacity = 0.25;
        line_material.properties.depth_write = false;
        // The instanced spheres: ray origins at even, hits at odd indices.
        let mut sphere_material = MeshBasicMaterial::default();
        sphere_material.properties.color = Color::from_hex(RAY_COLOR);
        let spheres = s.insert(NodeKind::Mesh(Mesh {
            geometry: Arc::new(SphereGeometry::build(1., 32, 16)?),
            materials: vec![Arc::new(Material::Basic(sphere_material))],
        }));
        let lines = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(line_geometry),
            material: Arc::new(Material::Line(line_material)),
            segments: true,
        }));
        // initRays(): randomDirection() × 3.75 for every instance.
        let mut random = Random(186);
        let initial: Vec<[f32; 3]> = (0..MAX_RAYS * 2)
            .map(|_| {
                let theta = random.next() * PI * 2.;
                let u = random.next() * 2. - 1.;
                let c = (1. - u * u).sqrt();
                [c * theta.cos() * 3.75, u * 3.75, c * theta.sin() * 3.75].map(|v| v as f32)
            })
            .collect();
        {
            let n = s.get_mut(spheres)?;
            n.instances = initial
                .iter()
                .map(|p| Instance {
                    matrix: translation(p.map(f64::from), 1.),
                    ..Default::default()
                })
                .collect();
            n.instance_count = Some(0);
            n.frustum_culled = false;
        }
        s.get_mut(lines)?.frustum_culled = false;
        // The bunny: translate( 0, 0.5 / 0.0075, 0 ), then computeBoundsTree().
        struct Bake {
            vertices: usize,
        }
        let json: serde_json::Value =
            serde_json::from_slice(&fetch(&format!("{ASSETS}/bunny.json")).await?)
                .map_err(|e| Error::Asset(e.to_string()))?;
        let bake = Bake {
            vertices: json["vertices"]
                .as_u64()
                .ok_or(Error::Invalid("bunny bake"))? as usize,
        };
        let bytes = fetch(&format!("{ASSETS}/bunny.bin")).await?;
        let n = bake.vertices;
        if bytes.len() != n * 8 * 4 {
            return Err(Error::Invalid("bunny bake"));
        }
        let floats: Vec<f32> = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|&b| f32::from_le_bytes(b))
            .collect();
        let (positions, rest) = floats.split_at(n * 3);
        let (normals, uvs) = rest.split_at(n * 3);
        let shift = 0.5 / 0.0075;
        let positions: Vec<f32> = positions
            .chunks(3)
            .flat_map(|p| [p[0], (f64::from(p[1]) + shift) as f32, p[2]])
            .collect();
        // applyNormalMatrix normalizes the normals.
        let normals: Vec<f32> = normals
            .chunks(3)
            .flat_map(|p| {
                let v = Vector3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2]));
                let l = v.length();
                let v = if l == 0. { v } else { v / l };
                [v.x as f32, v.y as f32, v.z as f32]
            })
            .collect();
        let mut index: Vec<u32> = (0..n as u32).collect();
        let bvh = Bvh::build(&positions, &mut index);
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(positions.clone(), 3, false)?),
        );
        geometry.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(normals, 3, false)?),
        );
        geometry.set_attribute(
            "uv",
            Attribute::F32(BufferAttribute::new(uvs.to_vec(), 2, false)?),
        );
        geometry.set_index(Some(index.clone()));
        // FBXLoader's MeshPhongMaterial for the model.
        let mut material = MeshPhongMaterial {
            specular: Color::from_hex(0x808080),
            shininess: 19.,
            ..Default::default()
        };
        material.properties.color = Color::WHITE;
        let bunny = s.insert(NodeKind::Mesh(Mesh {
            geometry: Arc::new(geometry),
            materials: vec![Arc::new(Material::Phong(material))],
        }));
        s.get_mut(bunny)?.scale = Vector3::splat(0.0075);
        let mut controls = Controls::new(None, (5., 75.), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            controls,
            bunny,
            spheres,
            lines,
            positions: Arc::new(positions),
            index,
            bvh,
            stored: initial,
            rotation: 0.,
            frames: 0,
            time: 0.,
            count: 150,
            first_hit_only: true,
            use_bvh: true,
        })
    }
    /// render(): the bunny's turn, then updateRays().
    fn frame(&mut self, s: &mut Scene) -> Result<()> {
        self.rotation += 0.002;
        let world = Matrix4::from_scale_rotation_translation(
            Vector3::splat(0.0075),
            Quaternion::from_rotation_y(self.rotation),
            Vector3::ZERO,
        );
        let inverse = world.inverse();
        let offset = 1e-4 * (self.frames as f64 / 60. * 1000.);
        let mut lines = Vec::with_capacity(self.count * 6);
        let mut instances = std::mem::take(&mut s.get_mut(self.spheres)?.instances);
        for i in 0..self.count {
            let p = self.stored[i * 2].map(f64::from);
            let i = i as f64;
            let axis = [
                (i * 100. + offset).sin(),
                (-i * 10. + offset).cos(),
                (i + offset).sin(),
            ];
            let l = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
            let axis = if l == 0. { axis } else { axis.map(|v| v / l) };
            let position = apply_axis_angle(p, axis, 0.001);
            let i = i as usize;
            self.stored[i * 2] = position.map(|v| v as f32);
            instances[i * 2].matrix = translation(self.stored[i * 2].map(f64::from), 0.02);
            // The ray toward the origin, in the bunny's frame ( transformDirection
            // normalizes ).
            let origin = Vector3::from_array(position);
            let direction = (-origin).normalize();
            let local = RayF {
                origin: inverse.transform_point3(origin),
                direction: inverse.transform_vector3(direction).normalize(),
            };
            let hit = if self.use_bvh && self.first_hit_only {
                self.bvh.raycast_first(&self.positions, &self.index, &local)
            } else if self.use_bvh {
                // Every hit through the BVH, sorted: the nearest.
                self.bvh.raycast_all(&self.positions, &self.index, &local)
            } else {
                // Every triangle, the nearest of the sorted hits.
                (0..self.index.len() / 3)
                    .filter_map(|t| local.triangle(&self.positions, &self.index, t))
                    .min_by(|a, b| a.0.total_cmp(&b.0))
            };
            lines.extend(self.stored[i * 2]);
            if let Some((_, point)) = hit {
                let point = world.transform_point3(point);
                self.stored[i * 2 + 1] = [point.x as f32, point.y as f32, point.z as f32];
                instances[i * 2 + 1].matrix = translation(point.to_array(), 0.01);
                lines.extend(self.stored[i * 2 + 1]);
            } else {
                self.stored[i * 2 + 1] = self.stored[i * 2];
                instances[i * 2 + 1].matrix = instances[i * 2].matrix;
                lines.extend([0f32; 3]);
            }
        }
        let n = s.get_mut(self.spheres)?;
        n.instances = instances;
        n.instance_count = Some((self.count * 2) as u32);
        if let NodeKind::Line(l) = &mut s.get_mut(self.lines)?.kind {
            let g = Arc::make_mut(&mut l.geometry);
            if let Some(Attribute::F32(a)) = g.attributes.get_mut("position") {
                a.array_mut()[..lines.len()].copy_from_slice(&lines);
            }
            g.set_draw_range(0, Some(lines.len() / 3));
        }
        self.frames += 1;
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        // One page frame per 60 Hz step of the clock, the first at time 0; a clock moved
        // back steps nothing until it passes the frames already run.
        let target = (self.time * 60.).round().max(0.) as u64 + 1;
        while self.frames < target {
            self.frame(s)?;
        }
        s.get_mut(self.bunny)?.quaternion = Quaternion::from_rotation_y(self.rotation);
        self.controls.update(s, c)
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
        match index {
            0 => self.count = (value as usize).clamp(1, MAX_RAYS),
            1 => self.first_hit_only = value != 0.,
            2 => self.use_bvh = value != 0.,
            // displayHelper and helperDepth: the culled helper.
            3 | 4 => {}
            _ => return Err(Error::Invalid("raycaster bvh parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
