//! MeshletDrawingPipeline ( three.js r187 examples/jsm/pipelines/
//! MeshletDrawingPipeline.js ) and MeshletGeometryUtils.createMeshlets
//! ( examples/jsm/utils/MeshletGeometryUtils.js ), for the meshes a page
//! draws through it.
//!
//! Geometries are processed at load into clusters of 64 triangles with six
//! simplified levels of detail, through the meshoptimizer modules three.js
//! bundles ( `meshopt.rs` ), one geometry per projection as the pipeline
//! does; the objects of a geometry are drawn by the regular pipeline until it
//! is processed. Per render list ( a view: the camera, a shadow cascade, a
//! bake light ) a culling holds the objects, the clusters of their
//! geometries, the work queues and the indirect draw arguments of its
//! batches ( the objects sharing a material and a vertex layout ). Every
//! projection runs the culling on the GPU in one compute pass: the objects
//! are frustum culled and select their level of detail, the clusters of the
//! visible objects are culled ( normal cones in camera views ) and appended
//! to the queues, and the draw arguments are written; each batch then draws
//! its queue with one drawIndirect of pulled vertices. Camera views run the
//! occlusion culling: the clusters visible in the last frame are drawn into
//! a depth buffer, its hierarchical depth is reduced on the GPU and the other
//! clusters are tested against it, while it culls at least 30 % of the
//! clusters ( measured in the three projections before each read of the
//! counters, every 30 projections ). The counters are read back without
//! waiting for the GPU, and the buffers grow or shrink to the measured
//! budgets.
//!
//! Every stage runs the WGSL three.js r187 generates for the pipeline
//! ( `meshlet/` ); the hierarchical depth's kernels embed the size of the
//! view as constants, so they are generated per view as three generates them.
//!
//! The parts of the pipeline the ported pages do not use are not ported: LOD
//! objects ( every object takes the full distance range ), instanced meshes,
//! position nodes, far materials, static bundle groups, the depth prepass and
//! the debug view. The batch table is uploaded when it changes, where the
//! original rewrites it each projection.
use super::lights_projector::pack;
use super::meshopt::Meshopt;
use crate::{Error, Result, math::*, renderer::Renderer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// The triangles of a meshlet.
pub(super) const MESHLET_SIZE: u32 = 64;
/// The levels of detail of a geometry.
pub(super) const MAX_LODS: usize = 6;
const MAX_INSTANCES: f64 = (1u32 << 24) as f64;
const CULLING_GROWTH: f64 = 1.5;
const BATCH_GROWTH: f64 = 1.25;
const CLUSTERS_PER_OBJECT: f64 = 8.;
const MIN_CLUSTER_BUDGET: f64 = (1u32 << 22) as f64;
const QUEUE_ITEMS_PER_OBJECT: f64 = 4.;
const QUEUE_MIN_OBJECTS: f64 = 64.;
const COUNTER_READBACK_INTERVAL: u32 = 30;
const OCCLUSION_MIN_CULLED: f64 = 0.3;
const OCCLUSION_PROBE_PROJECTIONS: u32 = 3;
const MAX_WORK_ITEMS: f64 = (1u32 << 20) as f64;
const MAX_WORKGROUPS: u32 = 65535;
const WORKGROUP_SIZE: u32 = 64;
const BATCH_STRIDE: usize = 3;
const GEOMETRY_LOD_ROW: usize = 2;
const GEOMETRY_STRIDE: usize = GEOMETRY_LOD_ROW + MAX_LODS;
const GEOMETRY_SHIFT: u32 = 16;
const MAX_GEOMETRIES: f64 = ((1u32 << GEOMETRY_SHIFT) - 1) as f64;
const FLAG_CONE: f32 = 1.;
const MAX_UNUSED_PROJECTIONS: u32 = 60;
const MAX_HZB_LEVELS: usize = 16;
const RANGE_GAP: u32 = 32;
const MAX_UPDATE_RANGES: usize = 64;
/// The distance range of objects which are not levels of LOD objects.
const NO_LEVEL_DISTANCE: f32 = 3.4e38;

/// LOD_TARGETS: ratio, error, attribute weights and flags of each level.
#[allow(clippy::type_complexity)]
const LOD_TARGETS: [(f64, f64, [f64; 5], &[&str]); MAX_LODS] = [
    (1.0, 0.0, [0.25, 0.25, 0.25, 0.5, 0.5], &[]),
    (
        0.55,
        0.004,
        [0.2, 0.2, 0.2, 0.35, 0.35],
        &["RegularizeLight", "Permissive", "Prune"],
    ),
    (
        0.25,
        0.015,
        [0.12, 0.12, 0.12, 0.2, 0.2],
        &["RegularizeLight", "Permissive", "Prune"],
    ),
    (
        0.1,
        0.05,
        [0.08, 0.08, 0.08, 0.12, 0.12],
        &["RegularizeLight", "Permissive", "Prune"],
    ),
    (
        0.04,
        0.14,
        [0.04, 0.04, 0.04, 0.06, 0.06],
        &["Regularize", "Permissive", "Prune"],
    ),
    (
        0.015,
        0.3,
        [0.02, 0.02, 0.02, 0.03, 0.03],
        &["Regularize", "Permissive", "Prune"],
    ),
];
const MIN_LOD_REDUCTION: f64 = 0.85;
const SLOPPY_ERROR_SCALE: f64 = 4.;
const MAX_MESHLET_VERTICES: u32 = MESHLET_SIZE * 3;

macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("meshlet/", $name, ".wgsl"))
    };
}
const RESET: &str = wgsl!("reset_cs");
const CULL: &str = wgsl!("cull_cs");
const CLUSTER_ARGS: &str = wgsl!("cluster_args_cs");
const CLUSTER_CULL: &str = wgsl!("cluster_cull_cs");
const CLUSTER_CULL_CONE: &str = wgsl!("cluster_cull_cone_cs");
const DRAW_ARGS: &str = wgsl!("draw_args_cs");
const CLUSTER_CULL_FIRST: &str = wgsl!("cluster_cull_first_cs");
const OCCLUSION_ARGS: &str = wgsl!("occlusion_args_cs");
const OBJECT_OCCLUSION: &str = wgsl!("object_occlusion_cs");
const CLUSTER_CULL_SECOND: &str = wgsl!("cluster_cull_second_cs");
const HZB_FIRST: &str = wgsl!("hzb_first_cs");
const HZB: &str = wgsl!("hzb_cs");
/// createDepthMesh's vertex and fragment stages: the pulled positions of the
/// clusters, depth only ( the occlusion view ) or into a shadow map.
pub(super) const DEPTH_VS: &str = wgsl!("depth_vs");
pub(super) const DEPTH_FS: &str = wgsl!("depth_fs");

/// A geometry as createMeshlets reads it: position, normal and uv, the other
/// vertex attributes ( name, values and item size, in the geometry's order ),
/// the index and the bounding sphere ( center and radius ).
pub(super) struct GeometrySource<'a> {
    pub positions: &'a [f32],
    pub normals: &'a [f32],
    pub uvs: Option<&'a [f32]>,
    pub extras: Vec<(&'a str, &'a [f32], usize)>,
    pub index: Option<&'a [u32]>,
    pub sphere: ([f64; 3], f64),
}

/// createMeshlets(): the welded vertices interleaved ( `stride` vec4
/// elements per vertex: position and uv.x, normal and uv.y, one per other
/// attribute ), the triangles of every level padded to 64 per cluster, the
/// clusters' bounding spheres and normal cones, and per level its first
/// triangle, triangles, first cluster and error.
pub(super) struct Clusters {
    pub vertex_array: Vec<f32>,
    pub stride: u32,
    pub index_array: Vec<u32>,
    pub chunk_array: Vec<f32>,
    pub chunk_count: u32,
    pub lod_offsets: [[f64; 4]; MAX_LODS],
    pub lod_count: u32,
    pub max_chunks: u32,
    pub layout_key: String,
    pub center: [f64; 3],
    pub radius: f64,
}

/// MeshletGeometryUtils.createMeshlets( geometry ) for a whole geometry.
pub(super) fn create_meshlets(meshopt: &Meshopt, g: &GeometrySource) -> Result<Clusters> {
    let source_count = g.positions.len() / 3;
    let index_count = g.index.map_or(source_count, |i| i.len());
    let count = index_count;
    let index_at = |i: usize| g.index.map_or(i as u32, |x| x[i]);
    // Weld identical vertices: the key is the bits of position, normal, uv
    // and the other attributes ( -0 stored as 0 ), hashed with FNV-1a.
    let mut key_attributes: Vec<(&[f32], usize)> = vec![(g.positions, 3), (g.normals, 3)];
    if let Some(uv) = g.uvs {
        key_attributes.push((uv, 2));
    }
    for (_, values, size) in &g.extras {
        key_attributes.push((values, *size));
    }
    let key_size: usize = key_attributes.iter().map(|(_, s)| s).sum();
    let mut keys = vec![0f32; source_count * key_size];
    let table_size = (2f64.powf(((source_count * 2 + 1) as f64).log2().ceil())) as usize;
    let mut table = vec![-1i32; table_size];
    let mut remap = vec![0u32; source_count];
    let mut sources: Vec<usize> = vec![];
    for i in 0..source_count {
        let offset = i * key_size;
        let mut k = offset;
        for (values, size) in &key_attributes {
            for c in 0..*size {
                keys[k] = values[i * size + c] + 0.;
                k += 1;
            }
        }
        let mut hash: u32 = 2166136261;
        for value in &keys[offset..k] {
            hash = (hash ^ value.to_bits()).wrapping_mul(16777619);
        }
        let mut slot = hash as usize & (table_size - 1);
        let mut index = -1i64;
        while table[slot] != -1 {
            let other = sources[table[slot] as usize] * key_size;
            let mut c = 0;
            while c < key_size && keys[other + c] == keys[offset + c] {
                c += 1;
            }
            if c == key_size {
                index = table[slot] as i64;
                break;
            }
            slot = (slot + 1) & (table_size - 1);
        }
        if index == -1 {
            index = sources.len() as i64;
            sources.push(i);
            table[slot] = index as i32;
        }
        remap[i] = index as u32;
    }
    let vertex_count = sources.len();
    let mut positions = vec![0f32; vertex_count * 3];
    let mut simplifier_attributes = vec![0f32; vertex_count * 5];
    let stride = 2 + g.extras.len();
    let mut vertex_array = vec![0f32; vertex_count * stride * 4];
    for (i, &s) in sources.iter().enumerate() {
        for (e, (_, values, size)) in g.extras.iter().enumerate() {
            for c in 0..*size {
                vertex_array[(i * stride + 2 + e) * 4 + c] = values[s * size + c];
            }
        }
        let p = &g.positions[s * 3..s * 3 + 3];
        let n = &g.normals[s * 3..s * 3 + 3];
        let (u, v) = g.uvs.map_or((0., 0.), |uv| (uv[s * 2], uv[s * 2 + 1]));
        positions[i * 3..i * 3 + 3].copy_from_slice(p);
        simplifier_attributes[i * 5..i * 5 + 5].copy_from_slice(&[n[0], n[1], n[2], u, v]);
        vertex_array[i * stride * 4..i * stride * 4 + 8]
            .copy_from_slice(&[p[0], p[1], p[2], u, n[0], n[1], n[2], v]);
    }
    let source_indices: Vec<u32> = (0..count).map(|i| remap[index_at(i) as usize]).collect();
    let source_scale = meshopt.get_scale(&positions, 3)?;
    struct Lod {
        meshlets: super::meshopt::Meshlets,
        bounds: Vec<super::meshopt::Bounds>,
        error: f64,
    }
    let mut lods: Vec<Lod> = vec![];
    let mut indices = source_indices.clone();
    let mut previous_error = 0.;
    let mut total_chunks = 0usize;
    for (i, (ratio, target_error, weights, flags)) in LOD_TARGETS.iter().enumerate() {
        let mut error = 0.;
        if i > 0 {
            // Pruning can reduce a level below the target of the next one.
            let target = (indices.len() as f64)
                .min(3f64.max((source_indices.len() as f64 * ratio / 3.).floor() * 3.));
            let mut simplified = meshopt.simplify_with_attributes(
                &indices,
                &positions,
                3,
                &simplifier_attributes,
                5,
                weights,
                target,
                *target_error,
                flags,
            )?;
            // A level the topology blocks is simplified without preserving
            // it, with a larger error.
            if indices.len() as f64 > target
                && simplified.0.len() as f64 > indices.len() as f64 * MIN_LOD_REDUCTION
            {
                let sloppy = meshopt.simplify_sloppy(
                    &indices,
                    &positions,
                    3,
                    target,
                    target_error * SLOPPY_ERROR_SCALE,
                )?;
                if sloppy.0.len() >= 3 && sloppy.0.len() < simplified.0.len() {
                    simplified = sloppy;
                }
            }
            if simplified.0.len() >= 3 {
                indices = simplified.0;
                error = previous_error + simplified.1 * source_scale;
            } else {
                error = previous_error;
            }
        }
        previous_error = error;
        let (meshlets, bounds) = meshopt.build_meshlets(
            &indices,
            &positions,
            3,
            MAX_MESHLET_VERTICES,
            MESHLET_SIZE,
            0.25,
        )?;
        total_chunks += meshlets.count;
        lods.push(Lod {
            meshlets,
            bounds,
            error,
        });
    }
    let total_triangles = total_chunks * MESHLET_SIZE as usize;
    let mut index_array = vec![0u32; total_triangles * 3];
    let mut chunk_array = vec![0f32; total_chunks * 8];
    let mut lod_offsets = [[0f64; 4]; MAX_LODS];
    let (mut chunk_offset, mut triangle_offset) = (0usize, 0usize);
    for (l, lod) in lods.iter().enumerate() {
        let num_triangles = lod.meshlets.count * MESHLET_SIZE as usize;
        lod_offsets[l] = [
            triangle_offset as f64,
            num_triangles as f64,
            chunk_offset as f64,
            lod.error,
        ];
        for m in 0..lod.meshlets.count {
            let (vertices, triangles) = lod.meshlets.extract(m);
            let meshlet_triangles = triangles.len() / 3;
            // Padded to 64 triangles with degenerate triangles.
            for t in 0..MESHLET_SIZE as usize {
                let triangle = triangle_offset + m * MESHLET_SIZE as usize + t;
                for k in 0..3 {
                    index_array[triangle * 3 + k] = if t < meshlet_triangles {
                        vertices[triangles[t * 3 + k] as usize]
                    } else {
                        vertices[0]
                    };
                }
            }
            let b = &lod.bounds[m];
            chunk_array[chunk_offset * 8..chunk_offset * 8 + 8].copy_from_slice(&[
                b.center[0],
                b.center[1],
                b.center[2],
                b.radius,
                b.cone_axis[0],
                b.cone_axis[1],
                b.cone_axis[2],
                b.cone_cutoff,
            ]);
            chunk_offset += 1;
        }
        triangle_offset += num_triangles;
    }
    let extras: Vec<String> = g
        .extras
        .iter()
        .map(|(name, _, size)| format!("{name}:{size}"))
        .collect();
    Ok(Clusters {
        vertex_array,
        stride: stride as u32,
        index_array,
        chunk_array,
        chunk_count: total_chunks as u32,
        lod_offsets,
        lod_count: lods.len() as u32,
        max_chunks: lods[0].meshlets.count as u32,
        layout_key: format!("{}|", extras.join(",")),
        center: g.sphere.0,
        radius: g.sphere.1,
    })
}

/// A storage buffer with its CPU copy's pending update ranges ( in 32-bit
/// elements ).
struct Upload {
    ranges: Vec<(usize, usize)>,
}
impl Upload {
    fn new() -> Self {
        Self { ranges: vec![] }
    }
    fn mark(&mut self, start: usize, count: usize) {
        if count > 0 {
            self.ranges.push((start, count));
        }
    }
    fn flush(&mut self, r: &Renderer, buffer: &wgpu::Buffer, data: &[u8]) {
        for (start, count) in self.ranges.drain(..) {
            r.queue.write_buffer(
                buffer,
                start as u64 * 4,
                &data[start * 4..(start + count) * 4],
            );
        }
    }
}

fn storage(r: &Renderer, label: &str, bytes: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: bytes.max(16).div_ceil(16) * 16,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | usage,
        mapped_at_creation: false,
    })
}
fn uniform_buffer(r: &Renderer, label: &str, source: &str, name: &str) -> Result<wgpu::Buffer> {
    Ok(r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: pack(source, name, &[])?.len() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}
fn raw_uniform(r: &Renderer, label: &str, bytes: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: bytes,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
fn write_struct(
    r: &Renderer,
    buffer: &wgpu::Buffer,
    source: &str,
    values: &[(&str, &[f64])],
) -> Result<()> {
    r.queue
        .write_buffer(buffer, 0, &pack(source, "objectStruct", values)?);
    Ok(())
}
fn bind(
    r: &Renderer,
    layout: &wgpu::BindGroupLayout,
    entries: &[(u32, wgpu::BindingResource)],
) -> wgpu::BindGroup {
    r.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("meshlet"),
        layout,
        entries: &entries
            .iter()
            .map(|(binding, resource)| wgpu::BindGroupEntry {
                binding: *binding,
                resource: resource.clone(),
            })
            .collect::<Vec<_>>(),
    })
}
fn compute_pipeline(r: &Renderer, label: &str, source: &str) -> wgpu::ComputePipeline {
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
}
/// The workgroups of compute( count ): one thread per item, in rows of
/// 65535 workgroups.
fn workgroups(count: u32) -> (u32, u32) {
    let groups = count.max(1).div_ceil(WORKGROUP_SIZE);
    if groups > MAX_WORKGROUPS {
        (MAX_WORKGROUPS, groups.div_ceil(MAX_WORKGROUPS))
    } else {
        (groups, 1)
    }
}

/// MeshletGeometryPool: the vertices and triangles of the processed
/// geometries, in one storage buffer each; the triangles hold the address
/// of each vertex's first element.
struct GeometryPool {
    vertex: Vec<f32>,
    index: Vec<u32>,
    vertex_end: usize,
    index_end: usize,
    /// Per geometry its first triangle.
    geometries: HashMap<usize, u32>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    vertex_upload: Upload,
    index_upload: Upload,
    /// Changes when the buffers are created again.
    generation: u64,
}
impl GeometryPool {
    fn new() -> Self {
        Self {
            vertex: vec![0.; 1024 * 4],
            index: vec![0; 1024 * 3],
            vertex_end: 0,
            index_end: 0,
            geometries: HashMap::new(),
            vertex_buffer: None,
            index_buffer: None,
            vertex_upload: Upload::new(),
            index_upload: Upload::new(),
            generation: 0,
        }
    }
    /// add( clusters ): the first triangle of the geometry in the pool.
    fn add(&mut self, id: usize, c: &Clusters) -> u32 {
        if let Some(&base) = self.geometries.get(&id) {
            return base;
        }
        let vertex_base = self.vertex_end;
        let index_base = self.index_end;
        self.reserve(
            vertex_base + c.vertex_array.len() / 4,
            index_base + c.index_array.len(),
        );
        self.vertex[vertex_base * 4..vertex_base * 4 + c.vertex_array.len()]
            .copy_from_slice(&c.vertex_array);
        for (i, &v) in c.index_array.iter().enumerate() {
            self.index[index_base + i] = vertex_base as u32 + v * c.stride;
        }
        self.vertex_upload
            .mark(vertex_base * 4, c.vertex_array.len());
        self.index_upload.mark(index_base, c.index_array.len());
        self.vertex_end += c.vertex_array.len() / 4;
        self.index_end += c.index_array.len();
        let base = (index_base / 3) as u32;
        self.geometries.insert(id, base);
        base
    }
    /// reserve(): the buffers grow by half.
    fn reserve(&mut self, vertices: usize, indices: usize) {
        if vertices * 4 > self.vertex.len() {
            let len = (vertices * 4).max((self.vertex.len() as f64 * 1.5 / 4.).ceil() as usize * 4);
            self.vertex.resize(len, 0.);
            self.vertex_buffer = None;
        }
        if indices > self.index.len() {
            let len = indices.max((self.index.len() as f64 * 1.5).ceil() as usize);
            self.index.resize(len, 0);
            self.index_buffer = None;
        }
    }
    fn flush(&mut self, r: &Renderer) {
        if self.vertex_buffer.is_none() || self.index_buffer.is_none() {
            self.generation += 1;
        }
        let vertex = self.vertex_buffer.get_or_insert_with(|| {
            self.vertex_upload.ranges = vec![(0, self.vertex_end * 4)];
            storage(
                r,
                "meshlet pool vertices",
                self.vertex.len() as u64 * 4,
                wgpu::BufferUsages::empty(),
            )
        });
        self.vertex_upload
            .flush(r, vertex, bytemuck::cast_slice(&self.vertex));
        let index = self.index_buffer.get_or_insert_with(|| {
            self.index_upload.ranges = vec![(0, self.index_end)];
            storage(
                r,
                "meshlet pool triangles",
                self.index.len() as u64 * 4,
                wgpu::BufferUsages::empty(),
            )
        });
        self.index_upload
            .flush(r, index, bytemuck::cast_slice(&self.index));
    }
}

/// The capacity of a culling's buffers.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Capacity {
    batches: u32,
    objects: u32,
    data: u32,
    queue: u32,
    clusters: u32,
    visibility: u32,
    chunks: u32,
    geometries: u32,
}

/// getQueueCapacity(): the work items of a queue.
fn queue_capacity(objects: f64, max_chunks: f64, budget: f64) -> f64 {
    (objects * max_chunks).min(MAX_WORK_ITEMS).min(
        (objects * QUEUE_ITEMS_PER_OBJECT)
            .max(max_chunks * QUEUE_MIN_OBJECTS)
            .max(budget),
    )
}

/// The occlusion data of a camera view ( createOcclusion ): the depth of the
/// clusters visible in the last frame, its hierarchical depth and the
/// kernels reducing it.
struct Occlusion {
    id: u64,
    width: u32,
    height: u32,
    depth: wgpu::TextureView,
    hzb: wgpu::Buffer,
    levels: wgpu::Buffer,
    level_count: u32,
    /// Per level its kernel, bind group and thread count.
    kernels: Vec<(wgpu::ComputePipeline, wgpu::BindGroup, u32)>,
}
impl Occlusion {
    fn new(r: &Renderer, width: u32, height: u32) -> Self {
        let depth = r
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("meshlet occlusion depth"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24Plus,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default());
        let mut levels = vec![];
        let (mut w, mut h, mut size) = (width, height, 0u32);
        loop {
            w = w.div_ceil(2);
            h = h.div_ceil(2);
            levels.push((size, w, h));
            size += w * h;
            if w <= 1 && h <= 1 {
                break;
            }
        }
        let hzb = storage(
            r,
            "meshlet hzb",
            size as u64 * 4,
            wgpu::BufferUsages::empty(),
        );
        let mut kernels = vec![];
        for (i, &(offset, w, h)) in levels.iter().enumerate() {
            let (source_offset, sw, sh) = if i > 0 {
                levels[i - 1]
            } else {
                (0, width, height)
            };
            let count = w * h;
            // The kernel three generates for the level: its size, the source's
            // and their offsets are constants.
            let source = if i == 0 {
                HZB_FIRST
                    .replace("65536u", &format!("{count}u"))
                    .replace("% 256u", &format!("% {w}u"))
                    .replace("/ 256u", &format!("/ {w}u"))
                    .replace(
                        "( ( nodeConst0 * 2u ) + 0u ), 511u",
                        &format!("( ( nodeConst0 * 2u ) + 0u ), {}u", sw - 1),
                    )
                    .replace(
                        "( ( nodeConst0 * 2u ) + 1u ), 511u",
                        &format!("( ( nodeConst0 * 2u ) + 1u ), {}u", sw - 1),
                    )
                    .replace(
                        "( ( nodeConst1 * 2u ) + 0u ), 511u",
                        &format!("( ( nodeConst1 * 2u ) + 0u ), {}u", sh - 1),
                    )
                    .replace(
                        "( ( nodeConst1 * 2u ) + 1u ), 511u",
                        &format!("( ( nodeConst1 * 2u ) + 1u ), {}u", sh - 1),
                    )
            } else {
                HZB.replace("16384u", &format!("{count}u"))
                    .replace("/ 128u", &format!("/ {w}u"))
                    .replace("% 128u", &format!("% {w}u"))
                    .replace(
                        "( ( nodeConst0 * 2u ) + 0u ), 255u ) * 256u",
                        &format!("( ( nodeConst0 * 2u ) + 0u ), {}u ) * {sw}u", sh - 1),
                    )
                    .replace(
                        "( ( nodeConst0 * 2u ) + 1u ), 255u ) * 256u",
                        &format!("( ( nodeConst0 * 2u ) + 1u ), {}u ) * {sw}u", sh - 1),
                    )
                    .replace(
                        "( ( nodeConst1 * 2u ) + 0u ), 255u ) ) + 0u )",
                        &format!(
                            "( ( nodeConst1 * 2u ) + 0u ), {}u ) ) + {source_offset}u )",
                            sw - 1
                        ),
                    )
                    .replace(
                        "( ( nodeConst1 * 2u ) + 1u ), 255u ) ) + 0u )",
                        &format!(
                            "( ( nodeConst1 * 2u ) + 1u ), {}u ) ) + {source_offset}u )",
                            sw - 1
                        ),
                    )
                    .replace(
                        "instanceIndex + 65536u",
                        &format!("instanceIndex + {offset}u"),
                    )
            };
            let pipeline = compute_pipeline(r, "MeshletDrawingPipeline HZB", &source);
            let object = r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("meshlet hzb object"),
                size: 16,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            r.queue
                .write_buffer(&object, 0, bytemuck::cast_slice(&[count, 0, 0, 0]));
            let layout = pipeline.get_bind_group_layout(0);
            let group = if i == 0 {
                bind(
                    r,
                    &layout,
                    &[
                        (0, wgpu::BindingResource::TextureView(&depth)),
                        (1, hzb.as_entire_binding()),
                        (2, object.as_entire_binding()),
                    ],
                )
            } else {
                bind(
                    r,
                    &layout,
                    &[
                        (0, hzb.as_entire_binding()),
                        (1, object.as_entire_binding()),
                    ],
                )
            };
            kernels.push((pipeline, group, count));
        }
        let mut level_data = vec![0f32; MAX_HZB_LEVELS * 4];
        for i in 0..MAX_HZB_LEVELS {
            let (offset, w, h) = levels[i.min(levels.len() - 1)];
            level_data[i * 4..i * 4 + 3].copy_from_slice(&[offset as f32, w as f32, h as f32]);
        }
        let level_buffer = raw_uniform(r, "meshlet hzb levels", (MAX_HZB_LEVELS * 16) as u64);
        r.queue
            .write_buffer(&level_buffer, 0, bytemuck::cast_slice(&level_data));
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self {
            id: NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            width,
            height,
            depth,
            hzb,
            levels: level_buffer,
            level_count: levels.len() as u32,
            kernels,
        }
    }
}

/// A batch: the objects of an entry in the culling's buffers, drawn by one
/// indirect draw of their visible clusters.
struct Batch {
    index: u32,
    object_offset: u32,
    data_offset: u32,
    queue_offset: u32,
    queue_capacity: u32,
    capacity: u32,
    max_chunks: u32,
    queue_budget: f64,
    count: u32,
    added: bool,
    culling: u64,
    /// The pulled meshes' render order ( their ids ).
    mesh_id: u64,
    /// Per object its slot, count and record value of the last write.
    geometry_states: Vec<(i64, u32, u32)>,
    /// The slots whose matrices the last upload wrote.
    history_ranges: Vec<(u32, u32)>,
    /// createDepthMesh's object struct ( opacity, data and queue offsets ),
    /// and its group for the culling's and the pool's buffers.
    depth_object: wgpu::Buffer,
    depth_group: Option<(u64, wgpu::BindGroup)>,
}

/// The objects of a render list sharing geometry layout, material and shadow
/// state ( _getEntry ).
struct Entry {
    id: u64,
    material: usize,
    layout: String,
    receive_shadow: bool,
    objects: Vec<MeshletObject>,
    count: u32,
    projection: u64,
    unused_projections: u32,
    z: f64,
    queue_budget: f64,
    queue_measured: bool,
    max_chunks: u32,
    batch: Option<Batch>,
}

/// MeshletCulling: the buffers, kernels and uniforms of a render list's
/// culling.
struct Culling {
    id: u64,
    shadow: bool,
    occlusion: bool,
    history: bool,
    capacity: Capacity,
    object_end: u32,
    data_end: u32,
    queue_end: u32,
    visibility_end: u32,
    chunk_end: u32,
    batch_end: u32,
    geometry_end: u32,
    geometry_indices: HashMap<usize, u32>,
    world: Vec<f32>,
    bounds: Vec<f32>,
    level: Vec<f32>,
    record: Vec<u32>,
    /// The clusters' spheres and cones, then the batch table, then the
    /// geometry table ( one storage buffer ).
    chunk: Vec<f32>,
    table_uploaded: Vec<f32>,
    world_upload: Upload,
    bounds_upload: Upload,
    level_upload: Upload,
    record_upload: Upload,
    chunk_upload: Upload,
    buffers: CullingBuffers,
    /// Changes with the buffers, so the draws' groups are created again.
    generation: u64,
    uniforms: CullingUniforms,
    groups: Option<CullingGroups>,
}
struct CullingBuffers {
    world: wgpu::Buffer,
    bounds: wgpu::Buffer,
    level: wgpu::Buffer,
    mvp: wgpu::Buffer,
    record: wgpu::Buffer,
    chunk: wgpu::Buffer,
    counters: wgpu::Buffer,
    queue: wgpu::Buffer,
    owner: wgpu::Buffer,
    args: wgpu::Buffer,
    visibility: Option<wgpu::Buffer>,
    dispatch: wgpu::Buffer,
    /// The staging buffer of the counters' readback.
    readback: wgpu::Buffer,
}
struct CullingUniforms {
    reset: wgpu::Buffer,
    cull: wgpu::Buffer,
    cluster_args: wgpu::Buffer,
    cluster_cull: wgpu::Buffer,
    draw_args: wgpu::Buffer,
    first: wgpu::Buffer,
    occlusion_args: wgpu::Buffer,
    object_occlusion: wgpu::Buffer,
    second: wgpu::Buffer,
    planes: wgpu::Buffer,
}
struct CullingGroups {
    occlusion: Option<u64>,
    reset: wgpu::BindGroup,
    cull: wgpu::BindGroup,
    cluster_args: wgpu::BindGroup,
    cluster_cull: wgpu::BindGroup,
    draw_args: wgpu::BindGroup,
    first: Option<wgpu::BindGroup>,
    occlusion_args: Option<wgpu::BindGroup>,
    object_occlusion: Option<wgpu::BindGroup>,
    second: Option<wgpu::BindGroup>,
}

/// The values of a culling's view ( updateView ).
struct ViewUniforms {
    proj_screen: Matrix4,
    planes: [[f64; 4]; 6],
    camera_position: Vector3,
    zoom: f64,
    lod_position: Vector3,
    lod_scale: f64,
    lod_perspective: f64,
    lod_threshold: f64,
    min_screen_radius: f64,
    coarse_screen_radius: f64,
    size: [f64; 2],
}

/// The counters' readback of a render list.
#[derive(Default)]
struct Readback {
    /// Mapped counters, once the map completes.
    result: Option<Vec<u32>>,
    failed: bool,
}

/// The objects of a render list across projections ( _getRecord ).
struct Record {
    entries: Vec<Entry>,
    next_entry: u64,
    culling: Option<Culling>,
    occlusion: Option<Occlusion>,
    cluster_budget: f64,
    needs_resize: bool,
    /// The pending readback: its state, culling id and generation, and per
    /// batch its entry, batch index, queue capacity, batch capacity and
    /// clusters per object.
    #[allow(clippy::type_complexity)]
    readback: Option<(
        Rc<RefCell<Readback>>,
        u64,
        u64,
        Vec<(u64, u32, u32, u32, u32)>,
        bool,
    )>,
    readback_projections: u32,
    occlusion_skipped: bool,
    occlusion_projections: u32,
    occlusion_visibility: u64,
}

/// One object drawn through the pipeline: its geometry ( the page's index
/// of its clusters ), material, receiveShadow, world matrix and depth for
/// sorting.
#[derive(Clone)]
pub(super) struct MeshletObject {
    pub geometry: usize,
    pub material: usize,
    pub receive_shadow: bool,
    pub matrix: [f64; 16],
    pub z: f64,
}

/// A render list's view: the camera's world position, projection and view,
/// whether it is perspective, its zoom, the render target's size and whether
/// it draws a shadow map.
pub(super) struct View {
    pub position: Vector3,
    pub projection: Matrix4,
    pub view: Matrix4,
    pub perspective: bool,
    pub zoom: f64,
    pub width: u32,
    pub height: u32,
    pub shadow: bool,
}

/// One batch to draw: its material and layout, the culling's buffers and the
/// argument offset of its indirect draw, its ranges, its depth and order,
/// and the generation of the buffers its groups bind.
pub(super) struct BatchDraw {
    pub material: usize,
    pub layout: String,
    pub args: wgpu::Buffer,
    pub args_offset: u64,
    pub data_offset: u32,
    pub queue_offset: u32,
    pub world: wgpu::Buffer,
    pub queue: wgpu::Buffer,
    pub vertices: wgpu::Buffer,
    pub triangles: wgpu::Buffer,
    pub generation: (u64, u64),
    pub z: f64,
    pub order: u64,
}

/// The result of a projection's _drawBatches: the batches to draw and the
/// objects the regular pipeline draws instead ( their entries wait for
/// geometries or exceed the instances ).
pub(super) struct Projection {
    pub batches: Vec<BatchDraw>,
    pub regular: Vec<MeshletObject>,
}

/// Which compute work and passes a projection issued ( for the page's
/// statistics ).
pub(super) struct Kernels {
    reset: wgpu::ComputePipeline,
    cull: wgpu::ComputePipeline,
    cluster_args: wgpu::ComputePipeline,
    cluster_cull: wgpu::ComputePipeline,
    cluster_cull_cone: wgpu::ComputePipeline,
    draw_args: wgpu::ComputePipeline,
    first: wgpu::ComputePipeline,
    occlusion_args: wgpu::ComputePipeline,
    object_occlusion: wgpu::ComputePipeline,
    second: wgpu::ComputePipeline,
    /// createDepthMesh's pipelines: the occlusion view's depth ( front
    /// faces, no color ), and a shadow map's ( back faces, as BackSide
    /// flips the winding, into its rgba8 color and depth ).
    pub occlusion_depth: wgpu::RenderPipeline,
    pub shadow_depth: wgpu::RenderPipeline,
}

/// MeshletDrawingPipeline.
pub(super) struct MeshletPipeline {
    pub lod_threshold: f64,
    pub shadow_lod_threshold: f64,
    pub min_screen_radius: f64,
    pub coarse_screen_radius: f64,
    pub occlusion_culling: bool,
    meshopt: Meshopt,
    projection: u64,
    pending_geometry: Option<u64>,
    waiting_geometry: Option<u64>,
    clusters: HashMap<usize, Clusters>,
    pool: GeometryPool,
    records: HashMap<u32, Record>,
    kernels: Kernels,
    next_culling: u64,
    next_mesh: u64,
    /// The occlusion view's render struct ( the camera's projection and
    /// view ) and its group.
    occlusion_render: wgpu::Buffer,
    occlusion_group: wgpu::BindGroup,
}

fn depth_pipeline(
    r: &Renderer,
    color: Option<wgpu::TextureFormat>,
    cw: bool,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("meshlet depth"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    let targets = [color.map(|format| wgpu::ColorTargetState {
        format,
        blend: None,
        write_mask: wgpu::ColorWrites::ALL,
    })];
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("meshlet depth"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module(DEPTH_VS),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module(DEPTH_FS),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: if color.is_some() { &targets } else { &[] },
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: if cw {
                    wgpu::FrontFace::Cw
                } else {
                    wgpu::FrontFace::Ccw
                },
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}

/// Frustum.setFromProjectionMatrix( m, WebGPUCoordinateSystem ): the planes'
/// normals and constants.
pub(super) fn frustum_planes(m: Matrix4) -> [[f64; 4]; 6] {
    let e = m.to_cols_array();
    let plane = |x: f64, y: f64, z: f64, w: f64| {
        let inverse = 1. / (x * x + y * y + z * z).sqrt();
        [x * inverse, y * inverse, z * inverse, w * inverse]
    };
    [
        plane(e[3] - e[0], e[7] - e[4], e[11] - e[8], e[15] - e[12]),
        plane(e[3] + e[0], e[7] + e[4], e[11] + e[8], e[15] + e[12]),
        plane(e[3] + e[1], e[7] + e[5], e[11] + e[9], e[15] + e[13]),
        plane(e[3] - e[1], e[7] - e[5], e[11] - e[9], e[15] - e[13]),
        plane(e[3] - e[2], e[7] - e[6], e[11] - e[10], e[15] - e[14]),
        plane(e[2], e[6], e[10], e[14]),
    ]
}

impl MeshletPipeline {
    pub(super) fn new(r: &Renderer, meshopt: Meshopt) -> Result<Self> {
        let kernels = Kernels {
            reset: compute_pipeline(r, "MeshletDrawingPipeline Reset", RESET),
            cull: compute_pipeline(r, "MeshletDrawingPipeline Cull", CULL),
            cluster_args: compute_pipeline(r, "MeshletDrawingPipeline Cluster Args", CLUSTER_ARGS),
            cluster_cull: compute_pipeline(r, "MeshletDrawingPipeline Cluster Cull", CLUSTER_CULL),
            cluster_cull_cone: compute_pipeline(
                r,
                "MeshletDrawingPipeline Cluster Cull",
                CLUSTER_CULL_CONE,
            ),
            draw_args: compute_pipeline(r, "MeshletDrawingPipeline Draw Args", DRAW_ARGS),
            first: compute_pipeline(r, "MeshletDrawingPipeline Cluster Cull", CLUSTER_CULL_FIRST),
            occlusion_args: compute_pipeline(
                r,
                "MeshletDrawingPipeline Occlusion Args",
                OCCLUSION_ARGS,
            ),
            object_occlusion: compute_pipeline(
                r,
                "MeshletDrawingPipeline Object Occlusion",
                OBJECT_OCCLUSION,
            ),
            second: compute_pipeline(
                r,
                "MeshletDrawingPipeline Cluster Cull",
                CLUSTER_CULL_SECOND,
            ),
            occlusion_depth: depth_pipeline(r, None, false),
            shadow_depth: depth_pipeline(r, Some(wgpu::TextureFormat::Rgba8Unorm), true),
        };
        let occlusion_render = raw_uniform(r, "meshlet occlusion render", 128);
        let occlusion_group = bind(
            r,
            &kernels.occlusion_depth.get_bind_group_layout(0),
            &[(0, occlusion_render.as_entire_binding())],
        );
        Ok(Self {
            lod_threshold: 1.,
            shadow_lod_threshold: 1.,
            min_screen_radius: 0.,
            coarse_screen_radius: 0.,
            occlusion_culling: true,
            meshopt,
            projection: 0,
            pending_geometry: None,
            waiting_geometry: None,
            clusters: HashMap::new(),
            pool: GeometryPool::new(),
            records: HashMap::new(),
            kernels,
            next_culling: 1,
            next_mesh: 1 << 40,
            occlusion_render,
            occlusion_group,
        })
    }
    pub(super) fn kernels(&self) -> &Kernels {
        &self.kernels
    }
    /// begin(): a new projection.
    pub(super) fn begin(&mut self) {
        self.projection += 1;
    }
    /// _hasClusters( geometry ).
    pub(super) fn has_clusters(&self, geometry: usize) -> bool {
        self.clusters.contains_key(&geometry)
    }
    /// _getClusters( geometry ): processes the geometry if no other one was
    /// processed in this projection, else marks the projection as waiting;
    /// whether the clusters are available.
    pub(super) fn get_clusters<'a>(
        &mut self,
        geometry: usize,
        source: impl FnOnce() -> Result<GeometrySource<'a>>,
    ) -> Result<bool> {
        if self.clusters.contains_key(&geometry) {
            return Ok(true);
        }
        if self.pending_geometry == Some(self.projection) {
            self.waiting_geometry = Some(self.projection);
            return Ok(false);
        }
        self.pending_geometry = Some(self.projection);
        let clusters = create_meshlets(&self.meshopt, &source()?)?;
        self.clusters.insert(geometry, clusters);
        Ok(true)
    }
    /// finishRenderList → _drawBatches( renderList ): the GPU-driven
    /// pipeline of the objects collected in this projection, submitted at
    /// once as renderer.compute() submits it: the culling's compute pass, or
    /// with the occlusion culling its two phases around the occlusion view's
    /// depth; then, every interval of projections, the counters' readback.
    pub(super) fn draw_batches(
        &mut self,
        r: &Renderer,
        key: u32,
        v: &View,
        objects: Vec<MeshletObject>,
    ) -> Result<Projection> {
        let projection = self.projection;
        let waiting = self.waiting_geometry == Some(projection);
        let record = self.records.entry(key).or_insert_with(|| Record {
            entries: vec![],
            next_entry: 0,
            culling: None,
            occlusion: None,
            cluster_budget: 0.,
            needs_resize: false,
            readback: None,
            readback_projections: 0,
            occlusion_skipped: false,
            occlusion_projections: 0,
            occlusion_visibility: 0,
        });
        // The counters read since the last projection ( the readback's
        // promise resolves between frames ).
        process_readback(r, record, &mut self.pool, &self.clusters)?;
        // _getEntry for each object pushed in this projection.
        for o in objects {
            let layout = self
                .clusters
                .get(&o.geometry)
                .map_or_else(String::new, |c| c.layout_key.clone());
            let i = match record.entries.iter().position(|e| {
                e.material == o.material
                    && e.layout == layout
                    && e.receive_shadow == o.receive_shadow
            }) {
                Some(i) => i,
                None => {
                    record.next_entry += 1;
                    record.entries.push(Entry {
                        id: record.next_entry,
                        material: o.material,
                        layout: layout.clone(),
                        receive_shadow: o.receive_shadow,
                        objects: vec![],
                        count: 0,
                        projection: 0,
                        unused_projections: 0,
                        z: 0.,
                        queue_budget: f64::INFINITY,
                        queue_measured: false,
                        max_chunks: 0,
                        batch: None,
                    });
                    record.entries.len() - 1
                }
            };
            let e = &mut record.entries[i];
            if e.projection != projection {
                e.projection = projection;
                e.objects.clear();
                e.count = 0;
            }
            if e.objects.is_empty() {
                e.z = o.z;
            }
            e.objects.push(o);
            e.count += 1;
        }
        let shadow = v.shadow;
        let (width, height) = (v.width, v.height);
        let lod_scale = v.projection.to_cols_array()[5] * height as f64 / 2.;
        // updateView: the occlusion view of camera views; the culling is
        // created again with its batches when the view changes.
        let occlusion_wanted = self.occlusion_culling && !shadow;
        if record
            .occlusion
            .as_ref()
            .is_some_and(|o| o.width != width || o.height != height)
            || record.occlusion.is_some() != occlusion_wanted
        {
            record.occlusion = None;
            record.culling = None;
            for entry in &mut record.entries {
                entry.batch = None;
            }
        }
        if occlusion_wanted && record.occlusion.is_none() {
            record.occlusion = Some(Occlusion::new(r, width, height));
            record.culling = None;
            for entry in &mut record.entries {
                entry.batch = None;
            }
        }
        let occlusion = record.occlusion.is_some();
        // The entries collected in this projection, in reverse order of
        // creation; the others are removed after unused projections.
        let mut i = record.entries.len();
        while i > 0 {
            i -= 1;
            let e = &mut record.entries[i];
            if e.projection != projection {
                e.unused_projections += 1;
                if e.unused_projections >= MAX_UNUSED_PROJECTIONS {
                    record.entries.remove(i);
                }
            }
        }
        let mut active: Vec<usize> = (0..record.entries.len())
            .rev()
            .filter(|&i| record.entries[i].projection == projection)
            .collect();
        if record.entries.is_empty() {
            record.culling = None;
            record.occlusion = None;
        }
        // The entries which have batches first ( a stable sort ), so the
        // drawn entries don't change when the objects exceed the instances.
        active.sort_by_key(|&i| u8::from(record.entries[i].batch.is_none()));
        let max_instances = max_instances(r);
        let mut available = max_instances;
        let mut regular = vec![];
        let mut kept_active = vec![];
        for &i in &active {
            let e = &mut record.entries[i];
            if e.count as f64 <= available {
                available -= e.count as f64;
                e.unused_projections = 0;
                kept_active.push(i);
            } else {
                regular.extend(e.objects.iter().cloned());
            }
        }
        let mut active = kept_active;
        if active.is_empty() {
            return Ok(Projection {
                batches: vec![],
                regular,
            });
        }
        // The clusters per object of a batch: the most detailed level of its
        // geometries.
        for &i in &active {
            let e = &mut record.entries[i];
            e.max_chunks = e
                .objects
                .iter()
                .map(|o| self.clusters.get(&o.geometry).map_or(0, |c| c.max_chunks))
                .max()
                .unwrap_or(0);
        }
        let culling_id = record.culling.as_ref().map(|c| c.id);
        let needs_batch = |e: &Entry| {
            e.batch
                .as_ref()
                .is_none_or(|b| !b.added || Some(b.culling) != culling_id || e.count > b.capacity)
        };
        let capacity_of = |e: &Entry| -> u32 {
            let current = e.batch.as_ref().map_or(64., |b| b.capacity as f64);
            current
                .max((e.count as f64 * BATCH_GROWTH / 64.).ceil() * 64.)
                .min(max_instances) as u32
        };
        // While geometries wait to be processed, the entries which don't fit
        // into the culling are drawn by the regular pipeline.
        if waiting {
            let resize = record.culling.is_none() || record.needs_resize;
            let mut keep = vec![];
            for &i in &active {
                let e = &record.entries[i];
                let culling = record.culling.as_ref();
                let wait = resize
                    || needs_batch(e)
                    || e.batch
                        .as_ref()
                        .is_some_and(|b| e.max_chunks > b.max_chunks)
                    || e.objects.iter().any(|o| {
                        culling.is_none_or(|c| !c.geometry_indices.contains_key(&o.geometry))
                    });
                if wait {
                    regular.extend(e.objects.iter().cloned());
                } else {
                    keep.push(i);
                }
            }
            active = keep;
            if active.is_empty() {
                return Ok(Projection {
                    batches: vec![],
                    regular,
                });
            }
        }
        // The culling is resized if the visible clusters exceeded a budget
        // in the last read of its counters, or a geometry doesn't fit.
        let mut rebuild = record.culling.is_none() || record.needs_resize;
        record.needs_resize = false;
        if !rebuild {
            let culling = record
                .culling
                .as_mut()
                .ok_or(Error::Invalid("meshlet culling"))?;
            rebuild = !add_geometries(
                culling,
                &mut self.pool,
                &self.clusters,
                &active,
                &record.entries,
            ) || active.iter().any(|&i| {
                let e = &record.entries[i];
                e.batch
                    .as_ref()
                    .is_some_and(|b| e.max_chunks > b.max_chunks)
            });
        }
        if !rebuild {
            for &i in &active {
                if !needs_batch(&record.entries[i]) {
                    continue;
                }
                let capacity = capacity_of(&record.entries[i]);
                let culling = record
                    .culling
                    .as_mut()
                    .ok_or(Error::Invalid("meshlet culling"))?;
                let e = &mut record.entries[i];
                let batch = create_batch(r, culling, e, capacity, &mut self.next_mesh)?;
                let added = batch.added;
                e.batch = Some(batch);
                if !added {
                    rebuild = true;
                    break;
                }
            }
        }
        if rebuild {
            // The culling is resized for all batches; the batches which can be
            // kept are added to the resized culling.
            let mut capacities = vec![];
            let mut kept = vec![];
            for &i in &active {
                let keep = record.culling.is_some() && !needs_batch(&record.entries[i]);
                let e = &mut record.entries[i];
                if keep {
                    let max_chunks = e.max_chunks;
                    let b = e.batch.as_mut().ok_or(Error::Invalid("meshlet batch"))?;
                    b.max_chunks = b.max_chunks.max(max_chunks);
                    capacities.push(b.capacity);
                    kept.push(i);
                } else {
                    capacities.push(capacity_of(e));
                }
            }
            let capacity = culling_capacity(
                r,
                &active
                    .iter()
                    .map(|&i| &record.entries[i])
                    .collect::<Vec<_>>(),
                &capacities,
                occlusion,
                shadow,
                record.cluster_budget,
                &self.clusters,
            );
            match record.culling.as_mut() {
                None => {
                    let id = self.next_culling;
                    self.next_culling += 1;
                    record.culling = Some(Culling::new(r, id, capacity, shadow, occlusion)?);
                }
                Some(culling) => {
                    for (i, e) in record.entries.iter_mut().enumerate() {
                        if !kept.contains(&i) {
                            e.batch = None;
                        }
                    }
                    culling.resize(r, capacity)?;
                }
            }
            let culling = record
                .culling
                .as_mut()
                .ok_or(Error::Invalid("meshlet culling"))?;
            add_geometries(
                culling,
                &mut self.pool,
                &self.clusters,
                &active,
                &record.entries,
            );
            // The budgets of the new buffers are checked after a few
            // projections, the occlusion culling needs a frame's clusters.
            record.readback_projections = COUNTER_READBACK_INTERVAL - if occlusion { 2 } else { 1 };
            for (k, &i) in active.iter().enumerate() {
                let e = &mut record.entries[i];
                if kept.contains(&i) {
                    let budget = e.queue_budget;
                    let b = e.batch.as_mut().ok_or(Error::Invalid("meshlet batch"))?;
                    b.queue_budget = budget;
                    if culling.relocate(b) {
                        continue;
                    }
                }
                e.batch = Some(create_batch(
                    r,
                    culling,
                    e,
                    capacities[k],
                    &mut self.next_mesh,
                )?);
            }
        }
        let culling = record
            .culling
            .as_mut()
            .ok_or(Error::Invalid("meshlet culling"))?;
        // updateView.
        let proj_screen = v.projection * v.view;
        let view_uniforms = ViewUniforms {
            proj_screen,
            planes: frustum_planes(proj_screen),
            camera_position: v.position,
            zoom: v.zoom,
            lod_position: v.position,
            lod_scale,
            lod_perspective: if v.perspective { 1. } else { 0. },
            lod_threshold: if shadow {
                self.shadow_lod_threshold
            } else {
                self.lod_threshold
            },
            min_screen_radius: self.min_screen_radius,
            coarse_screen_radius: self.coarse_screen_radius,
            size: [width as f64, height as f64],
        };
        // The objects of the batches not drawn in this projection are not
        // culled.
        for b in 0..culling.batch_end {
            culling.set_object_count(b, 0);
        }
        for &i in &active {
            let Entry { objects, batch, .. } = &mut record.entries[i];
            let geometries: Vec<u32> = objects
                .iter()
                .map(|o| {
                    culling
                        .geometry_indices
                        .get(&o.geometry)
                        .copied()
                        .unwrap_or(0)
                })
                .collect();
            let b = batch.as_mut().ok_or(Error::Invalid("meshlet batch"))?;
            culling.update(b, objects, &geometries);
        }
        // The occlusion culling is skipped while it culls few clusters,
        // except in the projections measuring it again.
        let occlusion_active = occlusion
            && (!record.occlusion_skipped
                || record.readback_projections
                    >= COUNTER_READBACK_INTERVAL - OCCLUSION_PROBE_PROJECTIONS);
        record.occlusion_projections =
            if occlusion_active && record.occlusion_visibility == culling.generation {
                record.occlusion_projections + 1
            } else {
                0
            };
        record.occlusion_visibility = culling.generation;
        self.pool.flush(r);
        culling.flush(r, &self.kernels, &view_uniforms, record.occlusion.as_ref())?;
        let culling = &*culling;
        let (vertices, triangles) = (
            self.pool
                .vertex_buffer
                .clone()
                .ok_or(Error::Invalid("meshlet pool"))?,
            self.pool
                .index_buffer
                .clone()
                .ok_or(Error::Invalid("meshlet pool"))?,
        );
        prepare_depth(
            r,
            &self.kernels,
            culling,
            &mut record.entries,
            &vertices,
            &triangles,
            self.pool.generation,
        )?;
        let k = &self.kernels;
        let g = culling
            .groups
            .as_ref()
            .ok_or(Error::Invalid("meshlet groups"))?;
        let dispatch = |pass: &mut wgpu::ComputePass,
                        p: &wgpu::ComputePipeline,
                        group: &wgpu::BindGroup,
                        count: u32| {
            pass.set_pipeline(p);
            pass.set_bind_group(0, group, &[]);
            let (x, y) = workgroups(count);
            pass.dispatch_workgroups(x, y, 1);
        };
        let counters_count = 2 + culling.capacity.batches * 2;
        let objects = culling.object_end.max(1);
        let args_count = (culling.batch_end * 2).max(1);
        let missing = || Error::Invalid("meshlet occlusion");
        let mut encoder = r
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("MeshletDrawingPipeline"),
            });
        if occlusion_active {
            let o = record.occlusion.as_ref().ok_or_else(missing)?;
            // The first phase: the clusters visible in the last frame.
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                dispatch(&mut pass, &k.reset, &g.reset, counters_count);
                dispatch(&mut pass, &k.cull, &g.cull, objects);
                dispatch(&mut pass, &k.cluster_args, &g.cluster_args, 1);
                pass.set_pipeline(&k.first);
                pass.set_bind_group(0, g.first.as_ref().ok_or_else(missing)?, &[]);
                pass.dispatch_workgroups_indirect(&culling.buffers.dispatch, 0);
                dispatch(
                    &mut pass,
                    &k.occlusion_args,
                    g.occlusion_args.as_ref().ok_or_else(missing)?,
                    args_count,
                );
            }
            // Their depth in the occlusion view.
            let data: Vec<f32> = [v.projection, v.view]
                .iter()
                .flat_map(|m| m.to_cols_array().map(|x| x as f32))
                .collect();
            r.queue
                .write_buffer(&self.occlusion_render, 0, bytemuck::cast_slice(&data));
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("meshlet occlusion depth"),
                    color_attachments: &[],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &o.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                pass.set_pipeline(&k.occlusion_depth);
                pass.set_bind_group(0, &self.occlusion_group, &[]);
                for &i in &active {
                    let Some(b) = &record.entries[i].batch else {
                        continue;
                    };
                    if let Some((_, group)) = &b.depth_group {
                        pass.set_bind_group(1, group, &[]);
                        pass.draw_indirect(
                            &culling.buffers.args,
                            b.index as u64 * 2 * 16 + culling.prepass_args_offset(),
                        );
                    }
                }
            }
            // The hierarchical depth and the second phase.
            let mut pass = encoder.begin_compute_pass(&Default::default());
            for (p, group, count) in &o.kernels {
                dispatch(&mut pass, p, group, *count);
            }
            dispatch(
                &mut pass,
                &k.object_occlusion,
                g.object_occlusion.as_ref().ok_or_else(missing)?,
                objects,
            );
            pass.set_pipeline(&k.second);
            pass.set_bind_group(0, g.second.as_ref().ok_or_else(missing)?, &[]);
            pass.dispatch_workgroups_indirect(&culling.buffers.dispatch, 0);
            dispatch(&mut pass, &k.draw_args, &g.draw_args, args_count);
        } else {
            // One compute pass for all batches of the render list.
            let mut pass = encoder.begin_compute_pass(&Default::default());
            dispatch(&mut pass, &k.reset, &g.reset, counters_count);
            dispatch(&mut pass, &k.cull, &g.cull, objects);
            dispatch(&mut pass, &k.cluster_args, &g.cluster_args, 1);
            pass.set_pipeline(if shadow {
                &k.cluster_cull
            } else {
                &k.cluster_cull_cone
            });
            pass.set_bind_group(0, &g.cluster_cull, &[]);
            pass.dispatch_workgroups_indirect(&culling.buffers.dispatch, 0);
            dispatch(&mut pass, &k.draw_args, &g.draw_args, args_count);
        }
        // _readCounters: every interval of projections, without waiting.
        let mut map = None;
        let occlusion_measured = record.occlusion_projections >= OCCLUSION_PROBE_PROJECTIONS;
        if record.readback.is_none() {
            record.readback_projections += 1;
            if record.readback_projections >= COUNTER_READBACK_INTERVAL {
                record.readback_projections = 0;
                encoder.copy_buffer_to_buffer(
                    &culling.buffers.counters,
                    0,
                    &culling.buffers.readback,
                    0,
                    counters_count as u64 * 4,
                );
                let state = Rc::new(RefCell::new(Readback::default()));
                let batches = active
                    .iter()
                    .filter_map(|&i| {
                        let e = &record.entries[i];
                        e.batch
                            .as_ref()
                            .map(|b| (e.id, b.index, b.queue_capacity, b.capacity, b.max_chunks))
                    })
                    .collect();
                map = Some((culling.buffers.readback.clone(), state.clone()));
                record.readback = Some((
                    state,
                    culling.id,
                    culling.generation,
                    batches,
                    occlusion_measured,
                ));
            }
        }
        r.queue.submit([encoder.finish()]);
        if let Some((buffer, state)) = map {
            let mapped = buffer.clone();
            buffer
                .slice(..)
                .map_async(wgpu::MapMode::Read, move |result| {
                    let mut s = state.borrow_mut();
                    match result {
                        Ok(()) => {
                            let data = mapped.slice(..).get_mapped_range();
                            s.result = Some(bytemuck::cast_slice(&data).to_vec());
                            drop(data);
                            mapped.unmap();
                        }
                        Err(_) => s.failed = true,
                    }
                });
        }
        let generation = (culling.generation, self.pool.generation);
        let mut draws = vec![];
        for &i in &active {
            let e = &record.entries[i];
            let b = e.batch.as_ref().ok_or(Error::Invalid("meshlet batch"))?;
            draws.push(BatchDraw {
                material: e.material,
                layout: e.layout.clone(),
                args: culling.buffers.args.clone(),
                args_offset: b.index as u64 * 2 * 16,
                data_offset: b.data_offset,
                queue_offset: b.queue_offset,
                world: culling.buffers.world.clone(),
                queue: culling.buffers.queue.clone(),
                vertices: vertices.clone(),
                triangles: triangles.clone(),
                generation,
                z: e.z,
                order: b.mesh_id,
            });
        }
        Ok(Projection {
            batches: draws,
            regular,
        })
    }

    /// The batches of a shadow view, drawn with createDepthMesh's material
    /// ( the pipeline `shadow_depth` and the view's render group are set by
    /// the caller ).
    pub(super) fn encode_depth(&self, pass: &mut wgpu::RenderPass, key: u32, draws: &[BatchDraw]) {
        let Some(record) = self.records.get(&key) else {
            return;
        };
        for d in draws {
            let batch = record
                .entries
                .iter()
                .filter_map(|e| e.batch.as_ref())
                .find(|b| b.mesh_id == d.order);
            if let Some((_, group)) = batch.and_then(|b| b.depth_group.as_ref()) {
                pass.set_bind_group(1, group, &[]);
                pass.draw_indirect(&d.args, d.args_offset);
            }
        }
    }
}

/// MeshletDrawingPipeline.maxInstances: the storage binding limit over 128
/// bytes per object.
fn max_instances(r: &Renderer) -> f64 {
    let limits = r.device.limits();
    let size = (limits.max_storage_buffer_binding_size as f64).min(limits.max_buffer_size as f64);
    (size / 128.).floor().min(MAX_INSTANCES)
}

/// The readback's counters, once mapped: the budgets of the clusters and the
/// queues, the occlusion measure, and the culling resized or shrunk.
fn process_readback(
    r: &Renderer,
    record: &mut Record,
    pool: &mut GeometryPool,
    clusters: &HashMap<usize, Clusters>,
) -> Result<()> {
    let Some((state, ..)) = &record.readback else {
        return Ok(());
    };
    let (ready, failed) = {
        let s = state.borrow();
        (s.result.clone(), s.failed)
    };
    if ready.is_none() && !failed {
        return Ok(());
    }
    let Some((_, id, generation, batches, measured)) = record.readback.take() else {
        return Ok(());
    };
    let Some(counters) = ready else {
        return Ok(());
    };
    // The counters of a culling resized or disposed since are not valid.
    let Some(culling) = record.culling.as_ref() else {
        return Ok(());
    };
    if culling.id != id || culling.generation != generation {
        return Ok(());
    }
    let mut exceeded = false;
    let mut shrink = false;
    let mut drawn = 0f64;
    if counters[0] as f64 > culling.capacity.clusters as f64 {
        record.cluster_budget = (counters[0] as f64 * CULLING_GROWTH).ceil();
        exceeded = true;
    }
    for &(entry, index, queue_capacity, capacity, max_chunks) in &batches {
        let visible = counters[1 + index as usize * 2] as f64;
        drawn += visible.min(queue_capacity as f64);
        let Some(e) = record.entries.iter_mut().find(|e| e.id == entry) else {
            continue;
        };
        if visible > queue_capacity as f64 {
            e.queue_budget = (visible * CULLING_GROWTH).ceil();
            exceeded = true;
        } else if !e.queue_measured {
            // The first measure replaces the budget of all clusters; the
            // buffers shrink if much larger.
            e.queue_budget = (visible * CULLING_GROWTH).ceil();
            if queue_capacity as f64 > queue_capacity_f(capacity, max_chunks, e.queue_budget) * 4. {
                shrink = true;
            }
        }
        e.queue_measured = true;
    }
    if measured {
        let occluded = counters[culling.occluded_counter() as usize] as f64;
        record.occlusion_skipped = occluded < (occluded + drawn) * OCCLUSION_MIN_CULLED;
    }
    if exceeded {
        record.needs_resize = true;
    } else if shrink {
        let ids: Vec<u64> = batches.iter().map(|b| b.0).collect();
        shrink_culling(r, record, pool, clusters, &ids)?;
    }
    Ok(())
}

/// _shrinkCulling: the buffers resized to the measured budgets of the
/// measured entries, their batches added again.
fn shrink_culling(
    r: &Renderer,
    record: &mut Record,
    pool: &mut GeometryPool,
    clusters: &HashMap<usize, Clusters>,
    ids: &[u64],
) -> Result<()> {
    let Some(culling_id) = record.culling.as_ref().map(|c| c.id) else {
        return Ok(());
    };
    let entries: Vec<Option<usize>> = ids
        .iter()
        .map(|id| record.entries.iter().position(|e| e.id == *id))
        .collect();
    if entries.iter().any(|i| {
        i.and_then(|i| record.entries[i].batch.as_ref())
            .is_none_or(|b| b.culling != culling_id)
    }) {
        record.needs_resize = true;
        return Ok(());
    }
    let entries: Vec<usize> = entries.into_iter().flatten().collect();
    let capacities: Vec<u32> = entries
        .iter()
        .map(|&i| record.entries[i].batch.as_ref().map_or(0, |b| b.capacity))
        .collect();
    let occlusion = record.occlusion.is_some();
    let shadow = record.culling.as_ref().is_some_and(|c| c.shadow);
    let capacity = culling_capacity(
        r,
        &entries
            .iter()
            .map(|&i| &record.entries[i])
            .collect::<Vec<_>>(),
        &capacities,
        occlusion,
        shadow,
        record.cluster_budget,
        clusters,
    );
    for (i, e) in record.entries.iter_mut().enumerate() {
        if !entries.contains(&i) {
            e.batch = None;
        }
    }
    let culling = record
        .culling
        .as_mut()
        .ok_or(Error::Invalid("meshlet culling"))?;
    culling.resize(r, capacity)?;
    if !add_geometries(culling, pool, clusters, &entries, &record.entries) {
        record.needs_resize = true;
    }
    for &i in &entries {
        let e = &mut record.entries[i];
        let budget = e.queue_budget;
        if let Some(b) = e.batch.as_mut() {
            b.queue_budget = budget;
            if !culling.relocate(b) {
                record.needs_resize = true;
            }
        }
    }
    Ok(())
}

/// _addGeometries: the geometries of the entries in the culling's table and
/// the pool; false when the culling is full.
fn add_geometries(
    culling: &mut Culling,
    pool: &mut GeometryPool,
    clusters: &HashMap<usize, Clusters>,
    active: &[usize],
    entries: &[Entry],
) -> bool {
    for &i in active {
        for o in &entries[i].objects {
            if culling.geometry_indices.contains_key(&o.geometry) {
                continue;
            }
            let Some(c) = clusters.get(&o.geometry) else {
                continue;
            };
            let base = pool.add(o.geometry, c);
            if !culling.add_geometry(o.geometry, c, base) {
                return false;
            }
        }
    }
    true
}

/// _getCullingCapacity: the buffers for the entries, with room for more
/// batches and objects.
fn culling_capacity(
    r: &Renderer,
    entries: &[&Entry],
    capacities: &[u32],
    occlusion: bool,
    shadow: bool,
    cluster_budget: f64,
    clusters: &HashMap<usize, Clusters>,
) -> Capacity {
    let (mut batches, mut objects, mut data, mut queue) = (0f64, 0f64, 0f64, 0f64);
    let (mut cluster_count, mut visibility, mut chunks) = (0f64, 0f64, 0f64);
    let mut geometries: Vec<usize> = vec![];
    for (e, &capacity) in entries.iter().zip(capacities) {
        let count = capacity as f64;
        let max_chunks = e
            .max_chunks
            .max(e.batch.as_ref().map_or(0, |b| b.max_chunks)) as f64;
        batches += 1.;
        objects += count;
        data += count;
        queue += queue_capacity(count, max_chunks, e.queue_budget);
        cluster_count += count * max_chunks;
        if occlusion {
            visibility += count * max_chunks;
        }
        for o in &e.objects {
            if !geometries.contains(&o.geometry) {
                geometries.push(o.geometry);
                chunks += clusters.get(&o.geometry).map_or(0, |c| c.chunk_count) as f64;
            }
        }
    }
    let geometry_capacity = (geometries.len() as f64 * 2.).clamp(64., MAX_GEOMETRIES);
    let limits = r.device.limits();
    let max_binding =
        (limits.max_storage_buffer_binding_size as f64).min(limits.max_buffer_size as f64);
    let grow = |value: f64, minimum: f64, bytes: f64| {
        (value * CULLING_GROWTH)
            .ceil()
            .max(minimum)
            .min((max_binding / bytes).floor())
    };
    let batches = (batches * 2.).max(16.);
    let objects = grow(objects, 1024., 16.);
    let data = grow(data, 1024., if shadow { 64. } else { 128. });
    let queue = grow(queue, 1024., 16.);
    let clusters = grow(
        cluster_count
            .min((objects * CLUSTERS_PER_OBJECT).max(MIN_CLUSTER_BUDGET))
            .max(cluster_budget),
        1024.,
        4.,
    );
    let visibility = grow(visibility, 0., 4.);
    let chunks = (chunks * 2.).max(1024.);
    Capacity {
        batches: batches as u32,
        objects: objects as u32,
        data: data as u32,
        queue: queue as u32,
        clusters: clusters as u32,
        visibility: visibility as u32,
        chunks: chunks as u32,
        geometries: geometry_capacity as u32,
    }
}

/// createBatch: a new batch of the entry in the culling.
fn create_batch(
    r: &Renderer,
    culling: &mut Culling,
    e: &Entry,
    capacity: u32,
    next_mesh: &mut u64,
) -> Result<Batch> {
    let mesh_id = *next_mesh;
    *next_mesh += 1;
    let mut b = Batch {
        index: 0,
        object_offset: 0,
        data_offset: 0,
        queue_offset: 0,
        queue_capacity: 0,
        capacity,
        max_chunks: e.max_chunks,
        queue_budget: e.queue_budget,
        count: 0,
        added: false,
        culling: culling.id,
        mesh_id,
        geometry_states: vec![],
        history_ranges: vec![],
        depth_object: uniform_buffer(r, "meshlet depth object", DEPTH_FS, "objectStruct")?,
        depth_group: None,
    };
    b.added = culling.add(&mut b);
    Ok(b)
}

/// The groups of the batches' depth draws ( the occlusion view and the
/// shadow maps: createDepthMesh ), created again when their buffers change.
fn prepare_depth(
    r: &Renderer,
    k: &Kernels,
    culling: &Culling,
    entries: &mut [Entry],
    vertices: &wgpu::Buffer,
    triangles: &wgpu::Buffer,
    pool_generation: u64,
) -> Result<()> {
    let generation = culling.generation ^ (pool_generation << 32);
    // The shadow maps' depth pipeline, or the occlusion view's.
    let layout = if culling.shadow {
        k.shadow_depth.get_bind_group_layout(1)
    } else {
        k.occlusion_depth.get_bind_group_layout(1)
    };
    for e in entries {
        let Some(b) = e.batch.as_mut() else { continue };
        if b.culling != culling.id
            || b.depth_group
                .as_ref()
                .is_some_and(|(g, _)| *g == generation)
        {
            continue;
        }
        write_struct(
            r,
            &b.depth_object,
            DEPTH_FS,
            &[
                ("nodeUniform0", &[1.]),
                ("nodeUniform4", &[b.data_offset as f64]),
                ("nodeUniform6", &[b.queue_offset as f64]),
            ],
        )?;
        let group = bind(
            r,
            &layout,
            &[
                (0, b.depth_object.as_entire_binding()),
                (1, culling.buffers.world.as_entire_binding()),
                (2, culling.buffers.queue.as_entire_binding()),
                (3, vertices.as_entire_binding()),
                (4, triangles.as_entire_binding()),
            ],
        );
        b.depth_group = Some((generation, group));
    }
    Ok(())
}

/// The bind group layouts of the culling kernels.
struct Layouts {
    reset: wgpu::BindGroupLayout,
    cull: wgpu::BindGroupLayout,
    cluster_args: wgpu::BindGroupLayout,
    cluster_cull: wgpu::BindGroupLayout,
    cluster_cull_cone: wgpu::BindGroupLayout,
    draw_args: wgpu::BindGroupLayout,
    first: wgpu::BindGroupLayout,
    occlusion_args: wgpu::BindGroupLayout,
    object_occlusion: wgpu::BindGroupLayout,
    second: wgpu::BindGroupLayout,
}
impl Kernels {
    fn layouts(&self) -> Layouts {
        let l = |p: &wgpu::ComputePipeline| p.get_bind_group_layout(0);
        Layouts {
            reset: l(&self.reset),
            cull: l(&self.cull),
            cluster_args: l(&self.cluster_args),
            cluster_cull: l(&self.cluster_cull),
            cluster_cull_cone: l(&self.cluster_cull_cone),
            draw_args: l(&self.draw_args),
            first: l(&self.first),
            occlusion_args: l(&self.occlusion_args),
            object_occlusion: l(&self.object_occlusion),
            second: l(&self.second),
        }
    }
}

/// getQueueCapacity over integer capacities.
fn queue_capacity_f(objects: u32, max_chunks: u32, budget: f64) -> f64 {
    queue_capacity(objects as f64, max_chunks as f64, budget)
}

/// SlotRanges: the slots written by an upload, merged across gaps.
struct SlotRanges {
    ranges: Vec<(u32, u32)>,
    start: i64,
    end: i64,
}
impl SlotRanges {
    fn new() -> Self {
        Self {
            ranges: vec![],
            start: -1,
            end: -1,
        }
    }
    fn mark(&mut self, slot: u32) {
        let slot = slot as i64;
        if self.end != -1 && slot <= self.end + RANGE_GAP as i64 {
            self.end = slot + 1;
            return;
        }
        if self.end != -1 {
            self.ranges.push((self.start as u32, self.end as u32));
        }
        self.start = slot;
        self.end = slot + 1;
    }
    fn finish(mut self) -> Vec<(u32, u32)> {
        if self.end != -1 {
            self.ranges.push((self.start as u32, self.end as u32));
        }
        if self.ranges.len() > MAX_UPDATE_RANGES {
            let (first, last) = (self.ranges[0].0, self.ranges[self.ranges.len() - 1].1);
            self.ranges = vec![(first, last)];
        }
        self.ranges
    }
}

impl Culling {
    fn new(
        r: &Renderer,
        id: u64,
        capacity: Capacity,
        shadow: bool,
        occlusion: bool,
    ) -> Result<Self> {
        let uniforms = CullingUniforms {
            reset: uniform_buffer(r, "meshlet reset", RESET, "objectStruct")?,
            cull: uniform_buffer(r, "meshlet cull", CULL, "objectStruct")?,
            cluster_args: uniform_buffer(r, "meshlet cluster args", CLUSTER_ARGS, "objectStruct")?,
            cluster_cull: uniform_buffer(
                r,
                "meshlet cluster cull",
                if shadow {
                    CLUSTER_CULL
                } else {
                    CLUSTER_CULL_CONE
                },
                "objectStruct",
            )?,
            draw_args: uniform_buffer(r, "meshlet draw args", DRAW_ARGS, "objectStruct")?,
            first: uniform_buffer(r, "meshlet first", CLUSTER_CULL_FIRST, "objectStruct")?,
            occlusion_args: uniform_buffer(
                r,
                "meshlet occlusion args",
                OCCLUSION_ARGS,
                "objectStruct",
            )?,
            object_occlusion: uniform_buffer(
                r,
                "meshlet object occlusion",
                OBJECT_OCCLUSION,
                "objectStruct",
            )?,
            second: uniform_buffer(r, "meshlet second", CLUSTER_CULL_SECOND, "objectStruct")?,
            planes: raw_uniform(r, "meshlet planes", 96),
        };
        let buffers = Self::allocate_buffers(r, capacity, !shadow, occlusion);
        let mut c = Self {
            id,
            shadow,
            occlusion,
            history: !shadow,
            capacity,
            object_end: 0,
            data_end: 0,
            queue_end: 0,
            visibility_end: 0,
            chunk_end: 0,
            batch_end: 0,
            geometry_end: 0,
            geometry_indices: HashMap::new(),
            world: vec![],
            bounds: vec![],
            level: vec![],
            record: vec![],
            chunk: vec![],
            table_uploaded: vec![],
            world_upload: Upload::new(),
            bounds_upload: Upload::new(),
            level_upload: Upload::new(),
            record_upload: Upload::new(),
            chunk_upload: Upload::new(),
            buffers,
            generation: 0,
            uniforms,
            groups: None,
        };
        c.allocate_arrays();
        Ok(c)
    }
    fn allocate_buffers(
        r: &Renderer,
        c: Capacity,
        history: bool,
        occlusion: bool,
    ) -> CullingBuffers {
        let data = c.data as u64;
        let none = wgpu::BufferUsages::empty();
        let counters = 2 + c.batches as u64 * 2;
        CullingBuffers {
            world: storage(
                r,
                "meshlet world",
                data * 64 * if history { 2 } else { 1 },
                none,
            ),
            bounds: storage(r, "meshlet bounds", data * 16, none),
            level: storage(r, "meshlet level", data * 32, none),
            mvp: storage(r, "meshlet mvp", data * 64, none),
            record: storage(r, "meshlet record", c.objects as u64 * 16, none),
            chunk: storage(
                r,
                "meshlet chunks",
                (c.chunks as u64 * 8
                    + c.batches as u64 * BATCH_STRIDE as u64 * 4
                    + c.geometries as u64 * GEOMETRY_STRIDE as u64 * 4)
                    * 4,
                none,
            ),
            counters: storage(
                r,
                "meshlet counters",
                counters * 4,
                wgpu::BufferUsages::COPY_SRC,
            ),
            queue: storage(r, "meshlet queue", c.queue as u64 * 16, none),
            owner: storage(
                r,
                "meshlet cluster owner",
                (c.clusters.max(1)) as u64 * 4,
                none,
            ),
            args: storage(
                r,
                "meshlet args",
                c.batches as u64 * 2 * 4 * 2 * 4,
                wgpu::BufferUsages::INDIRECT,
            ),
            visibility: occlusion.then(|| {
                storage(
                    r,
                    "meshlet visibility",
                    c.visibility.max(1) as u64 * 4,
                    none,
                )
            }),
            dispatch: {
                let b = storage(
                    r,
                    "meshlet cluster dispatch",
                    16,
                    wgpu::BufferUsages::INDIRECT,
                );
                r.queue
                    .write_buffer(&b, 0, bytemuck::cast_slice(&[0u32, 1, 1, 0]));
                b
            },
            readback: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("meshlet counters readback"),
                size: (counters * 4).div_ceil(16) * 16,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
        }
    }
    fn allocate_arrays(&mut self) {
        let c = self.capacity;
        let data = c.data as usize;
        self.world = vec![0.; data * 16 * if self.history { 2 } else { 1 }];
        self.bounds = vec![0.; data * 4];
        self.level = vec![0.; data * 8];
        self.record = vec![0; c.objects as usize * 4];
        self.chunk = vec![
            0.;
            c.chunks as usize * 8
                + c.batches as usize * BATCH_STRIDE * 4
                + c.geometries as usize * GEOMETRY_STRIDE * 4
        ];
        self.table_uploaded = vec![];
        self.world_upload = Upload::new();
        self.bounds_upload = Upload::new();
        self.level_upload = Upload::new();
        self.record_upload = Upload::new();
        self.chunk_upload = Upload::new();
        self.generation += 1;
        self.groups = None;
    }
    /// resize(): new buffers; the batches must be added again.
    fn resize(&mut self, r: &Renderer, capacity: Capacity) -> Result<()> {
        self.capacity = capacity;
        self.buffers = Self::allocate_buffers(r, capacity, self.history, self.occlusion);
        self.allocate_arrays();
        self.object_end = 0;
        self.data_end = 0;
        self.queue_end = 0;
        self.visibility_end = 0;
        self.chunk_end = 0;
        self.batch_end = 0;
        self.geometry_end = 0;
        self.geometry_indices.clear();
        Ok(())
    }
    fn table_offset(&self) -> usize {
        self.capacity.chunks as usize * 8
    }
    fn geometry_offset(&self) -> usize {
        self.table_offset() + self.capacity.batches as usize * BATCH_STRIDE * 4
    }
    fn prepass_args_offset(&self) -> u64 {
        self.capacity.batches as u64 * 2 * 4 * 4
    }
    fn occluded_counter(&self) -> u32 {
        1 + self.capacity.batches * 2
    }
    /// add( batch ): the batch's ranges and its rows of the table; false if
    /// the buffers are full.
    fn add(&mut self, b: &mut Batch) -> bool {
        let c = self.capacity;
        let objects = b.capacity;
        let queue = queue_capacity_f(objects, b.max_chunks, b.queue_budget) as u32;
        let visibility = if self.occlusion {
            objects * b.max_chunks
        } else {
            0
        };
        if self.batch_end + 1 > c.batches
            || self.object_end + objects > c.objects
            || self.data_end + objects > c.data
            || self.queue_end + queue > c.queue
            || self.visibility_end + visibility > c.visibility
        {
            return false;
        }
        let index = self.batch_end;
        self.batch_end += 1;
        b.index = index;
        b.object_offset = self.object_end;
        b.queue_offset = self.queue_end;
        b.queue_capacity = queue;
        b.data_offset = self.data_end;
        b.culling = self.id;
        b.count = 0;
        b.geometry_states.clear();
        b.history_ranges.clear();
        b.depth_group = None;
        let base = self.table_offset() + index as usize * BATCH_STRIDE * 4;
        let bits = |v: u32| f32::from_bits(v);
        self.chunk[base..base + 12].copy_from_slice(&[
            bits(self.object_end),
            bits(0),
            bits(self.queue_end),
            bits(queue),
            bits(b.data_offset),
            bits(0),
            bits(self.visibility_end),
            bits(b.max_chunks),
            0.,
            0.,
            MAX_LODS as f32,
            FLAG_CONE * if self.shadow { 0. } else { 1. },
        ]);
        for i in self.object_end..self.object_end + objects {
            self.record[i as usize * 4 + 3] = index;
        }
        self.record_upload
            .mark(self.object_end as usize * 4, objects as usize * 4);
        self.object_end += objects;
        self.data_end += objects;
        self.queue_end += queue;
        self.visibility_end += visibility;
        b.added = true;
        true
    }
    /// relocate(): the batch added to the resized culling, its objects
    /// written again.
    fn relocate(&mut self, b: &mut Batch) -> bool {
        b.added = self.add(b);
        b.added
    }
    /// addGeometry( clusters, triangleBase ): the clusters and the rows of a
    /// geometry; false if the buffers are full.
    fn add_geometry(&mut self, geometry: usize, c: &Clusters, triangle_base: u32) -> bool {
        if self.geometry_indices.contains_key(&geometry) {
            return true;
        }
        if self.geometry_end + 1 > self.capacity.geometries
            || self.chunk_end + c.chunk_count > self.capacity.chunks
        {
            return false;
        }
        let chunk_offset = self.chunk_end;
        self.chunk[chunk_offset as usize * 8..(chunk_offset + c.chunk_count) as usize * 8]
            .copy_from_slice(&c.chunk_array);
        self.chunk_upload
            .mark(chunk_offset as usize * 8, c.chunk_count as usize * 8);
        self.chunk_end += c.chunk_count;
        let index = self.geometry_end;
        self.geometry_end += 1;
        let base = self.geometry_offset() + index as usize * GEOMETRY_STRIDE * 4;
        let bits = |v: u32| f32::from_bits(v);
        self.chunk[base..base + 8].copy_from_slice(&[
            c.center[0] as f32,
            c.center[1] as f32,
            c.center[2] as f32,
            c.radius as f32,
            bits(c.lod_count),
            bits(triangle_base),
            bits(chunk_offset),
            bits(c.max_chunks),
        ]);
        for l in 0..MAX_LODS {
            let at = base + (GEOMETRY_LOD_ROW + l) * 4;
            let lod = c.lod_offsets[l];
            self.chunk[at..at + 4].copy_from_slice(&lod.map(|v| v as f32));
        }
        self.chunk_upload.mark(base, GEOMETRY_STRIDE * 4);
        self.geometry_indices.insert(geometry, index);
        true
    }
    fn set_object_count(&mut self, batch: u32, count: u32) {
        let at = self.table_offset() + batch as usize * BATCH_STRIDE * 4 + 1;
        self.chunk[at] = f32::from_bits(count);
    }
    /// MeshletBatch.update(): the geometries of the slots, the objects'
    /// transforms when they changed, and the batch's object count.
    fn update(&mut self, b: &mut Batch, objects: &[MeshletObject], geometries: &[u32]) {
        // writeGeometries.
        b.geometry_states.truncate(objects.len());
        let mut ranges = SlotRanges::new();
        for (o, &g) in geometries.iter().enumerate() {
            let value = b.index | (g << GEOMETRY_SHIFT);
            if b.geometry_states.len() <= o {
                b.geometry_states.push((-1, 0, u32::MAX));
            }
            let state = b.geometry_states[o];
            if state.0 != o as i64 || state.1 != 1 || state.2 != value {
                let at = (b.object_offset as usize + o) * 4 + 3;
                if self.record[at] != value {
                    self.record[at] = value;
                    ranges.mark(o as u32);
                }
                b.geometry_states[o] = (o as i64, 1, value);
            }
        }
        for (from, to) in ranges.finish() {
            self.record_upload.mark(
                (b.object_offset + from) as usize * 4,
                (to - from) as usize * 4,
            );
        }
        // The slots written in the last upload moved since the last frame:
        // their matrices of the last frame become the previous matrices.
        let history = std::mem::take(&mut b.history_ranges);
        for (start, end) in history {
            self.copy_history(b, start, end);
        }
        // write().
        let mut transform = SlotRanges::new();
        let level = SlotRanges::new();
        for (o, object) in objects.iter().enumerate() {
            let index = b.data_offset as usize + o;
            let e = &object.matrix;
            let same = (0..16).all(|i| self.world[index * 16 + i] == e[i] as f32);
            if !same {
                for (w, v) in self.world[index * 16..index * 16 + 16].iter_mut().zip(e) {
                    *w = *v as f32;
                }
                // writeBounds: the maximum scale, negative if not uniform.
                let scale =
                    |a: usize| (e[a] * e[a] + e[a + 1] * e[a + 1] + e[a + 2] * e[a + 2]).sqrt();
                let (sx, sy, sz) = (scale(0), scale(4), scale(8));
                let max = sx.max(sy).max(sz);
                let uniform = max - sx.min(sy).min(sz) <= max * 1e-4;
                self.bounds[index * 4 + 3] = if uniform { max } else { -max } as f32;
                transform.mark(o as u32);
            }
            // writeLevel: objects which are not levels take the whole range.
            let l = index * 8;
            if !(self.level[l + 3] == -NO_LEVEL_DISTANCE
                && self.level[l + 4] == NO_LEVEL_DISTANCE
                && self.level[l + 5] == NO_LEVEL_DISTANCE)
            {
                self.level[l + 3] = -NO_LEVEL_DISTANCE;
                self.level[l + 4] = NO_LEVEL_DISTANCE;
                self.level[l + 5] = NO_LEVEL_DISTANCE;
                self.level_upload.mark(l, 8);
            }
        }
        let _ = level;
        let ranges = transform.finish();
        for &(from, to) in &ranges {
            let start = b.data_offset + from;
            self.world_upload
                .mark(start as usize * 16, (to - from) as usize * 16);
            self.bounds_upload
                .mark(start as usize * 4, (to - from) as usize * 4);
        }
        b.history_ranges = ranges;
        // Added objects didn't move.
        let count = objects.len() as u32;
        let previous = b.count;
        self.copy_history(b, previous, count);
        b.count = count;
        self.set_object_count(b.index, count);
    }
    /// copyHistory(): the current matrices of the slots become their
    /// previous matrices.
    fn copy_history(&mut self, b: &Batch, from: u32, to: u32) {
        if !self.history || to <= from {
            return;
        }
        let start = b.data_offset as usize * 16;
        let previous = (self.capacity.data as usize + b.data_offset as usize) * 16;
        let (from, to) = (from as usize, to as usize);
        self.world
            .copy_within(start + from * 16..start + to * 16, previous + from * 16);
        self.world_upload
            .mark(previous + from * 16, (to - from) * 16);
    }
    /// The uploads, the uniforms of the view and the kernels' groups.
    fn flush(
        &mut self,
        r: &Renderer,
        k: &Kernels,
        v: &ViewUniforms,
        occlusion: Option<&Occlusion>,
    ) -> Result<()> {
        // The batch table, when its counts changed.
        let (t0, t1) = (self.table_offset(), self.geometry_offset());
        if self.table_uploaded.as_slice() != &self.chunk[t0..t1] {
            self.chunk_upload.mark(t0, t1 - t0);
            self.table_uploaded = self.chunk[t0..t1].to_vec();
        }
        let b = &self.buffers;
        self.world_upload
            .flush(r, &b.world, bytemuck::cast_slice(&self.world));
        self.bounds_upload
            .flush(r, &b.bounds, bytemuck::cast_slice(&self.bounds));
        self.level_upload
            .flush(r, &b.level, bytemuck::cast_slice(&self.level));
        self.record_upload
            .flush(r, &b.record, bytemuck::cast_slice(&self.record));
        self.chunk_upload
            .flush(r, &b.chunk, bytemuck::cast_slice(&self.chunk));
        let u = &self.uniforms;
        let n = |x: u32| x as f64;
        let table = n(self.capacity.chunks * 2);
        let geometry = n(self.capacity.chunks * 2 + self.capacity.batches * BATCH_STRIDE as u32);
        let objects = n(self.object_end);
        let max_clusters = n(self.capacity.clusters);
        let camera = v.camera_position.to_array();
        let proj = v.proj_screen.to_cols_array();
        write_struct(
            r,
            &u.reset,
            RESET,
            &[("nodeUniform1", &[n(2 + self.capacity.batches * 2)])],
        )?;
        write_struct(
            r,
            &u.cull,
            CULL,
            &[
                ("nodeUniform1", &[objects]),
                ("nodeUniform3", &[table]),
                ("nodeUniform5", &[geometry]),
                ("nodeUniform8", &camera),
                ("nodeUniform10", &[v.zoom]),
                ("nodeUniform11", &[v.lod_scale]),
                ("nodeUniform12", &[v.lod_perspective]),
                ("nodeUniform13", &v.lod_position.to_array()),
                ("nodeUniform14", &[v.min_screen_radius]),
                ("nodeUniform15", &[v.lod_threshold]),
                ("nodeUniform16", &[v.coarse_screen_radius]),
                ("nodeUniform18", &proj),
                ("nodeUniform20", &[max_clusters]),
            ],
        )?;
        write_struct(
            r,
            &u.cluster_args,
            CLUSTER_ARGS,
            &[("nodeUniform2", &[max_clusters]), ("nodeUniform3", &[1.])],
        )?;
        let (max_clusters_v, geometry_v, table_v) = ([max_clusters], [geometry], [table]);
        let mut cluster_cull: Vec<(&str, &[f64])> = vec![
            ("nodeUniform1", &max_clusters_v),
            ("nodeUniform5", &geometry_v),
            ("nodeUniform7", &table_v),
        ];
        if !self.shadow {
            cluster_cull.push(("nodeUniform8", &camera));
        }
        write_struct(
            r,
            &u.cluster_cull,
            if self.shadow {
                CLUSTER_CULL
            } else {
                CLUSTER_CULL_CONE
            },
            &cluster_cull,
        )?;
        let args_count = n((self.batch_end * 2).max(1));
        write_struct(
            r,
            &u.draw_args,
            DRAW_ARGS,
            &[("nodeUniform3", &[table]), ("nodeUniform4", &[args_count])],
        )?;
        let planes: Vec<f32> = v.planes.iter().flatten().map(|&x| x as f32).collect();
        r.queue
            .write_buffer(&u.planes, 0, bytemuck::cast_slice(&planes));
        if let Some(o) = occlusion {
            let size = &v.size[..];
            let levels = [n(o.level_count)];
            write_struct(
                r,
                &u.first,
                CLUSTER_CULL_FIRST,
                &[
                    ("nodeUniform1", &[max_clusters]),
                    ("nodeUniform5", &[geometry]),
                    ("nodeUniform7", &[table]),
                    ("nodeUniform8", &camera),
                ],
            )?;
            write_struct(
                r,
                &u.occlusion_args,
                OCCLUSION_ARGS,
                &[
                    ("nodeUniform1", &[n(self.capacity.batches * 2 * 4)]),
                    ("nodeUniform4", &[table]),
                    ("nodeUniform5", &[args_count]),
                ],
            )?;
            write_struct(
                r,
                &u.object_occlusion,
                OBJECT_OCCLUSION,
                &[
                    ("nodeUniform2", &[table]),
                    ("nodeUniform3", &proj),
                    ("nodeUniform5", &[geometry]),
                    ("nodeUniform9", size),
                    ("nodeUniform10", &levels),
                    ("nodeUniform11", &[n(self.object_end.max(1))]),
                ],
            )?;
            write_struct(
                r,
                &u.second,
                CLUSTER_CULL_SECOND,
                &[
                    ("nodeUniform1", &[max_clusters]),
                    ("nodeUniform5", &[geometry]),
                    ("nodeUniform7", &[table]),
                    ("nodeUniform8", &[n(self.occluded_counter())]),
                    ("nodeUniform10", &camera),
                    ("nodeUniform12", &proj),
                    ("nodeUniform15", size),
                    ("nodeUniform16", &levels),
                ],
            )?;
        }
        let occlusion_id = occlusion.map(|o| o.id);
        if self
            .groups
            .as_ref()
            .is_none_or(|g| g.occlusion != occlusion_id)
        {
            self.groups = Some(self.create_groups(r, k, occlusion)?);
        }
        Ok(())
    }
    fn create_groups(
        &self,
        r: &Renderer,
        kernels: &Kernels,
        occlusion: Option<&Occlusion>,
    ) -> Result<CullingGroups> {
        let k = kernels.layouts();
        let b = &self.buffers;
        let u = &self.uniforms;
        fn e(x: &wgpu::Buffer) -> wgpu::BindingResource<'_> {
            x.as_entire_binding()
        }
        let groups = CullingGroups {
            occlusion: occlusion.map(|o| o.id),
            reset: bind(r, &k.reset, &[(0, e(&b.counters)), (1, e(&u.reset))]),
            cull: bind(
                r,
                &k.cull,
                &[
                    (0, e(&b.record)),
                    (1, e(&u.cull)),
                    (2, e(&b.chunk)),
                    (3, e(&b.world)),
                    (4, e(&u.planes)),
                    (5, e(&b.bounds)),
                    (6, e(&b.level)),
                    (7, e(&b.mvp)),
                    (8, e(&b.counters)),
                    (9, e(&b.owner)),
                ],
            ),
            cluster_args: bind(
                r,
                &k.cluster_args,
                &[
                    (0, e(&b.dispatch)),
                    (1, e(&b.counters)),
                    (2, e(&u.cluster_args)),
                ],
            ),
            cluster_cull: bind(
                r,
                if self.shadow {
                    &k.cluster_cull
                } else {
                    &k.cluster_cull_cone
                },
                &[
                    (0, e(&b.counters)),
                    (1, e(&u.cluster_cull)),
                    (2, e(&b.owner)),
                    (3, e(&b.record)),
                    (4, e(&b.chunk)),
                    (5, e(&b.world)),
                    (6, e(&u.planes)),
                    (7, e(&b.queue)),
                ],
            ),
            draw_args: bind(
                r,
                &k.draw_args,
                &[
                    (0, e(&b.args)),
                    (1, e(&b.counters)),
                    (2, e(&b.chunk)),
                    (3, e(&u.draw_args)),
                ],
            ),
            first: None,
            occlusion_args: None,
            object_occlusion: None,
            second: None,
        };
        let Some(o) = occlusion else {
            return Ok(groups);
        };
        let visibility = b
            .visibility
            .as_ref()
            .ok_or(Error::Invalid("meshlet visibility"))?;
        Ok(CullingGroups {
            first: Some(bind(
                r,
                &k.first,
                &[
                    (0, e(&b.counters)),
                    (1, e(&u.first)),
                    (2, e(&b.owner)),
                    (3, e(&b.record)),
                    (4, e(&b.chunk)),
                    (5, e(&b.world)),
                    (6, e(&u.planes)),
                    (7, e(visibility)),
                    (8, e(&b.queue)),
                ],
            )),
            occlusion_args: Some(bind(
                r,
                &k.occlusion_args,
                &[
                    (0, e(&b.args)),
                    (1, e(&u.occlusion_args)),
                    (2, e(&b.counters)),
                    (3, e(&b.chunk)),
                ],
            )),
            object_occlusion: Some(bind(
                r,
                &k.object_occlusion,
                &[
                    (0, e(&b.record)),
                    (1, e(&b.chunk)),
                    (2, e(&u.object_occlusion)),
                    (3, e(&b.world)),
                    (4, e(&b.bounds)),
                    (5, e(&o.hzb)),
                    (6, e(&o.levels)),
                ],
            )),
            second: Some(bind(
                r,
                &k.second,
                &[
                    (0, e(&b.counters)),
                    (1, e(&u.second)),
                    (2, e(&b.owner)),
                    (3, e(&b.record)),
                    (4, e(&b.chunk)),
                    (5, e(visibility)),
                    (6, e(&b.world)),
                    (7, e(&u.planes)),
                    (8, e(&o.hzb)),
                    (9, e(&o.levels)),
                    (10, e(&b.queue)),
                ],
            )),
            ..groups
        })
    }
}
