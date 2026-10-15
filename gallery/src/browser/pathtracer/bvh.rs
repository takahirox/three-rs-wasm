//! three-mesh-bvh 0.9.10's MeshBVH as PathTracingSceneGenerator builds it
//! ( SAH strategy, maxLeafSize 1 from the deprecated maxLeafTris, an indirect
//! Uint32 triangle buffer, maxDepth 40 ), packed into the buffers
//! MeshBVHUniformStruct uploads. Bounds live in Float32Arrays and the SAH bins
//! are the module's shared 32 bins, as in the original, so the tree and its
//! textures match the original's bit for bit.

const BIN_COUNT: usize = 32;
const PRIMITIVE_INTERSECT_COST: f64 = 1.25;
const TRAVERSAL_COST: f64 = 1.;
const FLOAT32_EPSILON: f64 = 1. / 16777216.;
const MAX_DEPTH: usize = 40;
const MAX_LEAF_SIZE: usize = 1;
const IS_LEAFNODE_FLAG: u32 = 0xffff;

enum Node {
    Leaf {
        bounds: [f32; 6],
        offset: usize,
        count: usize,
    },
    Inner {
        bounds: [f32; 6],
        axis: u32,
        left: Box<Node>,
        right: Box<Node>,
    },
}

#[derive(Clone)]
struct Bin {
    count: usize,
    bounds: [f32; 6],
    right_cache: [f32; 6],
    left_cache: [f32; 6],
    candidate: f64,
}

struct Builder<'a> {
    bounds: &'a mut [f32],
    indirect: &'a mut [u32],
    bins: Vec<Bin>,
    left_bounds: [f32; 6],
    reached_max_depth: bool,
}

const EMPTY: [f32; 6] = [
    f32::INFINITY,
    f32::INFINITY,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NEG_INFINITY,
    f32::NEG_INFINITY,
];

/// getBounds: the primitives' union and their centroids' union, each written
/// to a Float32Array.
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
            let center = primitive[i * 6 + 2 * d] as f64;
            let half = primitive[i * 6 + 2 * d + 1] as f64;
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

/// expandByPrimitiveBounds into a Float32Array.
fn expand(primitive: &[f32], start: usize, bounds: &mut [f32; 6]) {
    for d in 0..3 {
        let center = primitive[start + 2 * d] as f64;
        let half = primitive[start + 2 * d + 1] as f64;
        let (min, max) = (center - half, center + half);
        if min < bounds[d] as f64 {
            bounds[d] = min as f32;
        }
        if max > bounds[d + 3] as f64 {
            bounds[d + 3] = max as f32;
        }
    }
}

fn union(a: &[f32; 6], b: &[f32; 6]) -> [f32; 6] {
    let mut t = [0f32; 6];
    for d in 0..3 {
        t[d] = if a[d] < b[d] { a[d] } else { b[d] };
        t[d + 3] = if a[d + 3] > b[d + 3] {
            a[d + 3]
        } else {
            b[d + 3]
        };
    }
    t
}

fn surface_area(b: &[f32; 6]) -> f64 {
    let d0 = b[3] as f64 - b[0] as f64;
    let d1 = b[4] as f64 - b[1] as f64;
    let d2 = b[5] as f64 - b[2] as f64;
    2. * (d0 * d1 + d1 * d2 + d2 * d0)
}

/// JavaScript's `~~x`: ToInt32.
fn to_int32(x: f64) -> i64 {
    if !x.is_finite() {
        return 0;
    }
    let t = x.trunc();
    let m = t.rem_euclid(4294967296.);
    (if m >= 2147483648. { m - 4294967296. } else { m }) as i64
}

impl Builder<'_> {
    /// getOptimalSplit with the SAH strategy: ( axis, position ).
    fn optimal_split(
        &mut self,
        node: &[f32; 6],
        centroid: &[f32; 6],
        offset: usize,
        count: usize,
    ) -> (i32, f64) {
        let mut axis = -1;
        let mut pos = 0.;
        let root_area = surface_area(node);
        let mut best = PRIMITIVE_INTERSECT_COST * count as f64;
        let (c_start, c_end) = (offset * 6, (offset + count) * 6);
        for a in 0..3 {
            let axis_left = centroid[a] as f64;
            let axis_right = centroid[a + 3] as f64;
            let axis_length = axis_right - axis_left;
            let bin_width = axis_length / BIN_COUNT as f64;
            if count < BIN_COUNT / 4 {
                // Every primitive position is a candidate.
                let mut truncated: Vec<usize> = (0..count).collect();
                for (b, c) in (c_start..c_end).step_by(6).enumerate() {
                    let bin = &mut self.bins[truncated[b]];
                    bin.candidate = self.bounds[c + 2 * a] as f64;
                    bin.count = 0;
                    bin.right_cache = EMPTY;
                    bin.left_cache = EMPTY;
                    bin.bounds = EMPTY;
                    expand(self.bounds, c, &mut bin.bounds);
                }
                // Array.sort by candidate ( stable ).
                let bins = &self.bins;
                truncated.sort_by(|&x, &y| {
                    let d = bins[x].candidate - bins[y].candidate;
                    if d < 0. {
                        std::cmp::Ordering::Less
                    } else if d > 0. {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                let mut split_count = count;
                let mut bi = 0;
                while bi < split_count {
                    while bi + 1 < split_count
                        && self.bins[truncated[bi + 1]].candidate
                            == self.bins[truncated[bi]].candidate
                    {
                        truncated.remove(bi + 1);
                        split_count -= 1;
                    }
                    bi += 1;
                }
                for c in (c_start..c_end).step_by(6) {
                    let center = self.bounds[c + 2 * a] as f64;
                    for &b in &truncated[..split_count] {
                        let bounds = &*self.bounds;
                        let bin = &mut self.bins[b];
                        if center >= bin.candidate {
                            expand(bounds, c, &mut bin.right_cache);
                        } else {
                            expand(bounds, c, &mut bin.left_cache);
                            bin.count += 1;
                        }
                    }
                }
                for &b in &truncated[..split_count] {
                    let bin = &self.bins[b];
                    let left_count = bin.count;
                    let right_count = count - bin.count;
                    let left_prob = if left_count != 0 {
                        surface_area(&bin.left_cache) / root_area
                    } else {
                        0.
                    };
                    let right_prob = if right_count != 0 {
                        surface_area(&bin.right_cache) / root_area
                    } else {
                        0.
                    };
                    let cost = TRAVERSAL_COST
                        + PRIMITIVE_INTERSECT_COST
                            * (left_prob * left_count as f64 + right_prob * right_count as f64);
                    if cost < best {
                        axis = a as i32;
                        best = cost;
                        pos = bin.candidate;
                    }
                }
            } else {
                for i in 0..BIN_COUNT {
                    let bin = &mut self.bins[i];
                    bin.count = 0;
                    bin.candidate = axis_left + bin_width + i as f64 * bin_width;
                    bin.bounds = EMPTY;
                }
                for c in (c_start..c_end).step_by(6) {
                    let tri_center = self.bounds[c + 2 * a] as f64;
                    let relative = tri_center - axis_left;
                    let mut index = to_int32(relative / bin_width);
                    if index >= BIN_COUNT as i64 {
                        index = BIN_COUNT as i64 - 1;
                    }
                    // A negative index reads an undefined bin and throws in the
                    // original; centroids never lie left of the axis start.
                    let bin = &mut self.bins[index as usize];
                    bin.count += 1;
                    expand(self.bounds, c, &mut bin.bounds);
                }
                let last = BIN_COUNT - 1;
                self.bins[last].right_cache = self.bins[last].bounds;
                for i in (0..BIN_COUNT - 1).rev() {
                    let next = self.bins[i + 1].right_cache;
                    self.bins[i].right_cache = union(&self.bins[i].bounds, &next);
                }
                let mut left_count = 0;
                for i in 0..BIN_COUNT - 1 {
                    let bin = &self.bins[i];
                    let bin_count = bin.count;
                    let right_bounds = self.bins[i + 1].right_cache;
                    if bin_count != 0 {
                        self.left_bounds = if left_count == 0 {
                            bin.bounds
                        } else {
                            union(&bin.bounds, &self.left_bounds)
                        };
                    }
                    left_count += bin_count;
                    let left_prob = if left_count != 0 {
                        surface_area(&self.left_bounds) / root_area
                    } else {
                        0.
                    };
                    let right_count = count - left_count;
                    let right_prob = if right_count != 0 {
                        surface_area(&right_bounds) / root_area
                    } else {
                        0.
                    };
                    let cost = TRAVERSAL_COST
                        + PRIMITIVE_INTERSECT_COST
                            * (left_prob * left_count as f64 + right_prob * right_count as f64);
                    if cost < best {
                        axis = a as i32;
                        best = cost;
                        pos = self.bins[i].candidate;
                    }
                }
            }
        }
        (axis, pos)
    }

    /// partition_indirect: Hoare partition of the indirect buffer and bounds.
    fn partition(&mut self, offset: usize, count: usize, axis: i32, pos: f64) -> usize {
        let mut left = offset as i64;
        let mut right = (offset + count) as i64 - 1;
        let axis_offset = axis as usize * 2;
        loop {
            while left <= right && (self.bounds[left as usize * 6 + axis_offset] as f64) < pos {
                left += 1;
            }
            while left <= right && (self.bounds[right as usize * 6 + axis_offset] as f64) >= pos {
                right -= 1;
            }
            if left < right {
                let (l, r) = (left as usize, right as usize);
                self.indirect.swap(l, r);
                for i in 0..6 {
                    self.bounds.swap(l * 6 + i, r * 6 + i);
                }
                left += 1;
                right -= 1;
            } else {
                return left as usize;
            }
        }
    }

    fn split(
        &mut self,
        bounds: [f32; 6],
        centroid: [f32; 6],
        offset: usize,
        count: usize,
        depth: usize,
    ) -> Node {
        if !self.reached_max_depth && depth >= MAX_DEPTH {
            self.reached_max_depth = true;
        }
        if count <= MAX_LEAF_SIZE || depth >= MAX_DEPTH {
            return Node::Leaf {
                bounds,
                offset,
                count,
            };
        }
        let (axis, pos) = self.optimal_split(&bounds, &centroid, offset, count);
        if axis == -1 {
            return Node::Leaf {
                bounds,
                offset,
                count,
            };
        }
        let split = self.partition(offset, count, axis, pos);
        if split == offset || split == offset + count {
            return Node::Leaf {
                bounds,
                offset,
                count,
            };
        }
        let lcount = split - offset;
        let (lb, lc) = get_bounds(self.bounds, offset, lcount);
        let left = self.split(lb, lc, offset, lcount, depth + 1);
        let rcount = count - lcount;
        let (rb, rc) = get_bounds(self.bounds, split, rcount);
        let right = self.split(rb, rc, split, rcount, depth + 1);
        Node::Inner {
            bounds,
            axis: axis as u32,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// The packed BVH and its triangle order.
pub(crate) struct MeshBvh {
    /// The 32-byte nodes: six bounds floats, then offset / right index and
    /// count + leaf flag / split axis ( as u32 words ).
    pub nodes: Vec<[u32; 8]>,
    pub indirect: Vec<u32>,
}

/// new MeshBVH( geometry, { strategy: SAH, maxLeafTris: 1, indirect: true } ).
pub(crate) fn build(positions: &[f32], index: &[u32]) -> MeshBvh {
    let triangles = index.len() / 3;
    let mut indirect: Vec<u32> = (0..triangles as u32).collect();
    // computePrimitiveBounds: centre and half extent per axis, padded by a
    // float32 epsilon scaled to the coordinates.
    let mut primitive = vec![0f32; triangles * 6];
    for i in 0..triangles {
        let tri = indirect[i] as usize * 3;
        let (ai, bi, ci) = (
            index[tri] as usize,
            index[tri + 1] as usize,
            index[tri + 2] as usize,
        );
        for el in 0..3 {
            let a = positions[ai * 3 + el] as f64;
            let b = positions[bi * 3 + el] as f64;
            let c = positions[ci * 3 + el] as f64;
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
            primitive[i * 6 + el * 2] = (min + half) as f32;
            primitive[i * 6 + el * 2 + 1] = (half + (min.abs() + half) * FLOAT32_EPSILON) as f32;
        }
    }
    let (bounds, centroid) = get_bounds(&primitive, 0, triangles);
    let mut builder = Builder {
        bounds: &mut primitive,
        indirect: &mut indirect,
        bins: vec![
            Bin {
                count: 0,
                bounds: [0.; 6],
                right_cache: [0.; 6],
                left_cache: [0.; 6],
                candidate: 0.
            };
            BIN_COUNT
        ],
        left_bounds: [0.; 6],
        reached_max_depth: false,
    };
    let root = builder.split(bounds, centroid, 0, triangles, 0);
    let mut nodes = vec![];
    populate(&root, &mut nodes);
    MeshBvh { nodes, indirect }
}

/// populateBuffer: depth first, the right child's index relative to its parent.
fn populate(node: &Node, out: &mut Vec<[u32; 8]>) {
    let here = out.len();
    let bits = |b: &[f32; 6]| b.map(f32::to_bits);
    match node {
        Node::Leaf {
            bounds,
            offset,
            count,
        } => {
            let b = bits(bounds);
            out.push([
                b[0],
                b[1],
                b[2],
                b[3],
                b[4],
                b[5],
                *offset as u32,
                *count as u32 | (IS_LEAFNODE_FLAG << 16),
            ]);
        }
        Node::Inner {
            bounds,
            axis,
            left,
            right,
        } => {
            let b = bits(bounds);
            out.push([b[0], b[1], b[2], b[3], b[4], b[5], 0, *axis]);
            populate(left, out);
            let right_index = out.len();
            out[here][6] = (right_index - here) as u32;
            populate(right, out);
        }
    }
}

/// bvhToTextures: the RGBA32F bounds ( two texels per node ) and RG32UI
/// contents ( leaf count | flag, offset; or split axis, right index ).
pub(crate) fn textures(bvh: &MeshBvh) -> ((u32, Vec<f32>), (u32, Vec<u32>)) {
    let count = bvh.nodes.len();
    let bounds_dim = 2 * ((count as f64 / 2.).sqrt().ceil() as u32);
    let mut bounds = vec![0f32; 4 * (bounds_dim * bounds_dim) as usize];
    let contents_dim = (count as f64).sqrt().ceil() as u32;
    let mut contents = vec![0u32; 2 * (contents_dim * contents_dim) as usize];
    for (i, n) in bvh.nodes.iter().enumerate() {
        for b in 0..3 {
            bounds[8 * i + b] = f32::from_bits(n[b]);
            bounds[8 * i + 4 + b] = f32::from_bits(n[3 + b]);
        }
        let is_leaf = n[7] >> 16 == IS_LEAFNODE_FLAG;
        if is_leaf {
            contents[i * 2] = (IS_LEAFNODE_FLAG << 16) | (n[7] & 0xffff);
            contents[i * 2 + 1] = n[6];
        } else {
            contents[i * 2] = n[7];
            contents[i * 2 + 1] = n[6];
        }
    }
    ((bounds_dim, bounds), (contents_dim, contents))
}
