//! MDD morph animation, the edge-split modifier, the 3DS loader, the Utah teapot
//! and surface-scattered instances from the pinned WebGL examples.
use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use super::teapot_data::{PATCHES, VERTICES};
use super::terrain_loaders::parse_obj;
use super::trackball_sprites::{Mode, Trackball};
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    tsl::{surface::SurfaceNodes, *},
};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
fn floats(g: &BufferGeometry, name: &str) -> Vec<f32> {
    match g.attributes.get(name) {
        Some(Attribute::F32(a)) => a.array().to_vec(),
        _ => vec![],
    }
}
/// CubeTextureLoader: six sRGB faces with mipmaps.
async fn cube(r: &Renderer, base: &str) -> Result<GpuTexture> {
    let mut faces = vec![];
    for face in ["px", "nx", "py", "ny", "pz", "nz"] {
        let mut t = decode_texture_image(&fetch(&format!("{base}/{face}.png")).await?).await?;
        t.srgb = true;
        faces.push(t);
    }
    GpuTexture::from_cube_rgba(
        r,
        &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
    )
}
/// MDDLoader: big-endian frame times and absolute morph positions.
fn parse_mdd(data: &[u8]) -> Result<(Vec<f64>, Vec<Vec<f32>>)> {
    let bad = || Error::Asset("MDD: truncated".into());
    let u = |i: usize| -> Result<u32> {
        Ok(u32::from_be_bytes(
            data.get(i..i + 4)
                .ok_or_else(bad)?
                .try_into()
                .map_err(|_| bad())?,
        ))
    };
    let f = |i: usize| -> Result<f32> { Ok(f32::from_bits(u(i)?)) };
    let (frames, points) = (u(0)? as usize, u(4)? as usize);
    let mut offset = 8;
    let mut times = vec![];
    for _ in 0..frames {
        times.push(f(offset)? as f64);
        offset += 4;
    }
    let mut targets = vec![];
    for _ in 0..frames {
        let mut target = Vec::with_capacity(points * 3);
        for _ in 0..points * 3 {
            target.push(f(offset)?);
            offset += 4;
        }
        targets.push(target);
    }
    Ok((times, targets))
}
/// BufferGeometryUtils.mergeVertices( geometry, 1e-4 ) over position, normal and uv.
fn merge_vertices(
    positions: &[f32],
    normals: &[f32],
    uvs: &[f32],
) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<u32>) {
    let count = positions.len() / 3;
    // hashMultiplier = 10^4, hashAdditive = 0.5: Math.trunc( v × 1e4 + 0.5 ).
    let key = |v: f32| (v as f64 * 10000. + 0.5).trunc() as i64;
    let mut map: HashMap<Vec<i64>, u32> = HashMap::new();
    let (mut p, mut n, mut t, mut index) = (vec![], vec![], vec![], vec![]);
    for i in 0..count {
        let mut hash = Vec::with_capacity(8);
        hash.extend(positions[i * 3..i * 3 + 3].iter().map(|&v| key(v)));
        hash.extend(normals[i * 3..i * 3 + 3].iter().map(|&v| key(v)));
        if !uvs.is_empty() {
            hash.extend(uvs[i * 2..i * 2 + 2].iter().map(|&v| key(v)));
        }
        let next = (p.len() / 3) as u32;
        let id = *map.entry(hash).or_insert_with(|| {
            p.extend_from_slice(&positions[i * 3..i * 3 + 3]);
            n.extend_from_slice(&normals[i * 3..i * 3 + 3]);
            if !uvs.is_empty() {
                t.extend_from_slice(&uvs[i * 2..i * 2 + 2]);
            }
            next
        });
        index.push(id);
    }
    (p, n, t, index)
}
/// The merged model the edge-split modifier starts from.
struct Merged {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    index: Vec<u32>,
}
/// EdgeSplitModifier.modify( geometry, cutOffAngle, tryKeepNormals ).
fn edge_split(m: &Merged, cut_off_angle: f64, try_keep_normals: bool) -> Result<BufferGeometry> {
    let (indexes, positions) = (&m.index, &m.positions);
    let p = |i: u32| {
        let i = i as usize * 3;
        Vector3::new(
            positions[i] as f64,
            positions[i + 1] as f64,
            positions[i + 2] as f64,
        )
    };
    // computeNormals(): per-corner face normals.
    let mut normals = vec![0f32; indexes.len() * 3];
    for i in (0..indexes.len()).step_by(3) {
        let (a, b, c) = (p(indexes[i]), p(indexes[i + 1]), p(indexes[i + 2]));
        let n = (c - b).cross(a - b).normalize_or_zero();
        for j in 0..3 {
            normals[3 * (i + j)..3 * (i + j) + 3]
                .copy_from_slice(&[n.x as f32, n.y as f32, n.z as f32]);
        }
    }
    let normal = |j: usize| {
        Vector3::new(
            normals[3 * j] as f64,
            normals[3 * j + 1] as f64,
            normals[3 * j + 2] as f64,
        )
        .normalize_or_zero()
    };
    let mut point_to_index: Vec<Vec<usize>> = vec![vec![]; positions.len() / 3];
    for (i, &index) in indexes.iter().enumerate() {
        point_to_index[index as usize].push(i);
    }
    let cut_off = cut_off_angle.cos() - 0.001;
    let mut split_indexes: Vec<(usize, Vec<usize>)> = vec![];
    fn edge_split_rec(
        indexes: &[usize],
        cut_off: f64,
        original: Option<usize>,
        normal: &dyn Fn(usize) -> Vector3,
        out: &mut Vec<(usize, Vec<usize>)>,
    ) {
        if indexes.is_empty() {
            return;
        }
        let groups: Vec<(Vec<usize>, Vec<usize>)> = indexes
            .iter()
            .map(|&first| {
                let a = normal(first);
                let (mut split, mut current) = (vec![], vec![first]);
                for &j in indexes {
                    if j != first {
                        if normal(j).dot(a) < cut_off {
                            split.push(j);
                        } else {
                            current.push(j);
                        }
                    }
                }
                (split, current)
            })
            .collect();
        let mut best = 0;
        for (k, g) in groups.iter().enumerate() {
            if g.1.len() > groups[best].1.len() {
                best = k;
            }
        }
        let (split, current) = &groups[best];
        if let Some(original) = original {
            out.push((original, current.clone()));
        }
        if !split.is_empty() {
            // original || result.currentGroup[ 0 ]: an original of 0 is falsy.
            let next = original.filter(|&o| o != 0).unwrap_or(current[0]);
            edge_split_rec(split, cut_off, Some(next), normal, out);
        }
    }
    for vertex_indexes in &point_to_index {
        edge_split_rec(vertex_indexes, cut_off, None, &normal, &mut split_indexes);
    }
    // New attributes sized ( indexes.length + splits ), with appended copies.
    let size = indexes.len() + split_indexes.len();
    let copy = |source: &[f32], item: usize| {
        let mut a = vec![0f32; size * item];
        a[..source.len()].copy_from_slice(source);
        for (i, (original, _)) in split_indexes.iter().enumerate() {
            let index = indexes[*original] as usize;
            for j in 0..item {
                a[(indexes.len() + i) * item + j] = a[index * item + j];
            }
        }
        a
    };
    let new_positions = copy(&m.positions, 3);
    let new_uvs = copy(&m.uvs, 2);
    let mut new_index = indexes.clone();
    for (i, (_, group)) in split_indexes.iter().enumerate() {
        for &j in group {
            new_index[j] = (indexes.len() + i) as u32;
        }
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(new_positions)?);
    g.set_attribute(
        "uv",
        Attribute::F32(BufferAttribute::new(new_uvs, 2, false)?),
    );
    g.set_index(Some(new_index));
    g.compute_vertex_normals()?;
    if try_keep_normals {
        // changedNormals is indexed by vertex but marked at index-array positions.
        let mut changed = vec![false; m.normals.len() / 3];
        for (original, _) in &split_indexes {
            if let Some(c) = changed.get_mut(*original) {
                *c = true;
            }
        }
        if let Some(Attribute::F32(a)) = g.attributes.get_mut("normal") {
            let array = a.array_mut();
            for (i, &c) in changed.iter().enumerate() {
                if !c {
                    array[3 * i..3 * i + 3].copy_from_slice(&m.normals[3 * i..3 * i + 3]);
                }
            }
            a.mark_dirty();
        }
    }
    Ok(g)
}
/// TDSLoader: MDATA meshes (points, faces, uvs, material groups, local matrix) and materials.
struct TdsMesh {
    positions: Vec<f32>,
    uvs: Vec<f32>,
    index: Vec<u32>,
    matrix: Option<Matrix4>,
    material: Option<String>,
}
#[derive(Clone, Default)]
struct TdsMaterial {
    color: Option<[f64; 3]>,
    specular: Option<[f64; 3]>,
    shininess: Option<f64>,
    map: Option<String>,
    double_side: bool,
}
/// 3DS meshes, named materials and the master scale.
type Tds = (Vec<TdsMesh>, HashMap<String, TdsMaterial>, Option<f64>);
fn parse_3ds(data: &[u8]) -> Result<Tds> {
    let bad = || Error::Asset("3DS: truncated".into());
    let u16_at = |i: usize| -> Result<u16> {
        Ok(u16::from_le_bytes([
            *data.get(i).ok_or_else(bad)?,
            *data.get(i + 1).ok_or_else(bad)?,
        ]))
    };
    let u32_at = |i: usize| -> Result<u32> {
        Ok(u32::from_le_bytes(
            data.get(i..i + 4)
                .ok_or_else(bad)?
                .try_into()
                .map_err(|_| bad())?,
        ))
    };
    let f32_at = |i: usize| -> Result<f32> { Ok(f32::from_bits(u32_at(i)?)) };
    let string = |i: usize| -> Result<(String, usize)> {
        let end = i + data
            .get(i..)
            .ok_or_else(bad)?
            .iter()
            .position(|&c| c == 0)
            .ok_or_else(bad)?;
        Ok((String::from_utf8_lossy(&data[i..end]).into_owned(), end + 1))
    };
    // Chunk children: ( id, start of content, end ).
    let children = |start: usize, end: usize| -> Result<Vec<(u16, usize, usize)>> {
        let mut out = vec![];
        let mut p = start;
        while p + 6 <= end {
            let (id, size) = (u16_at(p)?, u32_at(p + 2)? as usize);
            if size < 6 {
                break;
            }
            out.push((id, p + 6, (p + size).min(data.len())));
            p += size;
        }
        Ok(out)
    };
    let color = |start: usize, end: usize| -> Result<Option<[f64; 3]>> {
        // setRGB without a color space: the values are used as linear working colors.
        for (id, s, _) in children(start, end)? {
            match id {
                0x0011 | 0x0012 => {
                    return Ok(Some([
                        data[s] as f64 / 255.,
                        data[s + 1] as f64 / 255.,
                        data[s + 2] as f64 / 255.,
                    ]));
                }
                0x0010 | 0x0013 => {
                    return Ok(Some([
                        f32_at(s)? as f64,
                        f32_at(s + 4)? as f64,
                        f32_at(s + 8)? as f64,
                    ]));
                }
                _ => {}
            }
        }
        Ok(None)
    };
    let percentage = |start: usize, end: usize| -> Result<f64> {
        for (id, s, _) in children(start, end)? {
            match id {
                0x0030 => return Ok(i16::from_le_bytes([data[s], data[s + 1]]) as f64 / 100.),
                0x0031 => return Ok(f32_at(s)? as f64),
                _ => {}
            }
        }
        Ok(0.)
    };
    let (mut meshes, mut materials, mut scale) = (vec![], HashMap::new(), None);
    let root = u16_at(0)?;
    if !matches!(root, 0x4D4D | 0x3DAA | 0xC23D) {
        return Err(Error::Asset("3DS: magic".into()));
    }
    let root_end = (u32_at(2)? as usize).min(data.len());
    for (id, s, e) in children(6, root_end)? {
        if id != 0x3D3D {
            continue;
        }
        for (id, s, e) in children(s, e)? {
            match id {
                0x0100 => scale = Some(f32_at(s)? as f64),
                0xAFFF => {
                    let (mut name, mut m) = (String::new(), TdsMaterial::default());
                    for (id, s, e) in children(s, e)? {
                        match id {
                            0xA000 => name = string(s)?.0,
                            0xA081 => m.double_side = true,
                            0xA020 | 0xA010 => m.color = color(s, e)?,
                            0xA030 => m.specular = color(s, e)?,
                            0xA040 => m.shininess = Some(percentage(s, e)? * 100.),
                            0xA200 => {
                                for (id, s, _) in children(s, e)? {
                                    if id == 0xA300 {
                                        m.map = Some(string(s)?.0);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    materials.insert(name, m);
                }
                0x4000 => {
                    let (_, content) = string(s)?;
                    for (id, s, e) in children(content, e)? {
                        if id != 0x4100 {
                            continue;
                        }
                        let mut mesh = TdsMesh {
                            positions: vec![],
                            uvs: vec![],
                            index: vec![],
                            matrix: None,
                            material: None,
                        };
                        for (id, s, e) in children(s, e)? {
                            match id {
                                0x4110 => {
                                    let n = u16_at(s)? as usize;
                                    for i in 0..n * 3 {
                                        mesh.positions.push(f32_at(s + 2 + i * 4)?);
                                    }
                                }
                                0x4140 => {
                                    let n = u16_at(s)? as usize;
                                    for i in 0..n * 2 {
                                        mesh.uvs.push(f32_at(s + 2 + i * 4)?);
                                    }
                                }
                                0x4120 => {
                                    let n = u16_at(s)? as usize;
                                    for i in 0..n {
                                        for k in 0..3 {
                                            mesh.index.push(u16_at(s + 2 + i * 8 + k * 2)? as u32);
                                        }
                                    }
                                    for (id, s, _) in children(s + 2 + n * 8, e)? {
                                        if id == 0x4130 && mesh.material.is_none() {
                                            mesh.material = Some(string(s)?.0);
                                        }
                                    }
                                }
                                0x4160 => {
                                    let v: Vec<f64> = (0..12)
                                        .map(|i| f32_at(s + i * 4).map(|x| x as f64))
                                        .collect::<Result<_>>()?;
                                    // elements[ 0..12 ] as the loader writes them, then transposed.
                                    let e = [
                                        v[0], v[6], v[3], v[9], v[2], v[8], v[5], v[11], v[1],
                                        v[7], v[4], v[10], 0., 0., 0., 1.,
                                    ];
                                    mesh.matrix = Some(Matrix4::from_cols_array(&e).transpose());
                                }
                                _ => {}
                            }
                        }
                        meshes.push(mesh);
                    }
                }
                _ => {}
            }
        }
    }
    Ok((meshes, materials, scale))
}
/// TeapotGeometry( size, segments, bottom, lid, body, fitLid, blinn ).
fn teapot(
    size: f64,
    segments: u32,
    bottom: bool,
    lid: bool,
    body: bool,
    fit_lid: bool,
    blinn: bool,
) -> Result<BufferGeometry> {
    let segments = segments.max(2) as usize;
    let blinn_scale = 1.3;
    let max_height = 3.15 * if blinn { 1. } else { blinn_scale };
    let max_height2 = max_height / 2.;
    let true_size = size / max_height2;
    let mut triangles = if bottom {
        (8 * segments - 4) * segments
    } else {
        0
    };
    triangles += if lid {
        (16 * segments - 4) * segments
    } else {
        0
    };
    triangles += if body { 40 * segments * segments } else { 0 };
    let mut indices = vec![0u32; triangles * 3];
    let mut vertices_count =
        if bottom { 4 } else { 0 } + if lid { 8 } else { 0 } + if body { 20 } else { 0 };
    vertices_count *= (segments + 1) * (segments + 1);
    let mut vertices = vec![0f32; vertices_count * 3];
    let mut normals = vec![0f32; vertices_count * 3];
    let mut uvs = vec![0f32; vertices_count * 2];
    // ms: the cubic Bezier basis (Matrix4.set is row-major).
    let ms = Matrix4::from_cols_array(&[
        -1., 3., -3., 1., 3., -6., 3., 0., -3., 3., 0., 0., 1., 0., 0., 0.,
    ])
    .transpose();
    let mst = ms.transpose();
    let (min_patches, max_patches) = (if body { 0 } else { 20 }, if bottom { 32 } else { 28 });
    let per_row = segments + 1;
    let (mut surf_count, mut vert_count, mut index_count) = (0, 0, 0);
    let not_degenerate = |v: &[f32], a: usize, b: usize, c: usize| {
        let same = |x: usize, y: usize| {
            v[x * 3] == v[y * 3] && v[x * 3 + 1] == v[y * 3 + 1] && v[x * 3 + 2] == v[y * 3 + 2]
        };
        !(same(a, b) || same(a, c) || same(b, c))
    };
    for surf in min_patches..max_patches {
        if !lid && (20..28).contains(&surf) {
            continue;
        }
        let mut mgm = [Matrix4::IDENTITY; 3];
        for (i, m) in mgm.iter_mut().enumerate() {
            let mut g = [0f64; 16];
            for r in 0..4 {
                for c in 0..4 {
                    let mut v = VERTICES[PATCHES[surf * 16 + r * 4 + c] as usize * 3 + i];
                    if fit_lid && (20..28).contains(&surf) && i != 2 {
                        v *= 1.077;
                    }
                    if !blinn && i == 2 {
                        v *= blinn_scale;
                    }
                    g[c * 4 + r] = v;
                }
            }
            // gmx.set( g[0..16] ) is row-major.
            let gmx = Matrix4::from_cols_array(&g).transpose();
            *m = mst * (gmx * ms);
        }
        for sstep in 0..=segments {
            let s = sstep as f64 / segments as f64;
            for tstep in 0..=segments {
                let t = tstep as f64 / segments as f64;
                let (mut sp, mut tp, mut dsp, mut dtp) =
                    ([0f64; 4], [0f64; 4], [0f64; 4], [0f64; 4]);
                let (mut sval, mut tval, mut dsval, mut dtval) = (1f64, 1f64, 0f64, 0f64);
                for p in (0..4).rev() {
                    sp[p] = sval;
                    tp[p] = tval;
                    sval *= s;
                    tval *= t;
                    if p == 3 {
                        dsp[p] = 0.;
                        dtp[p] = 0.;
                        dsval = 1.;
                        dtval = 1.;
                    } else {
                        dsp[p] = dsval * (3 - p) as f64;
                        dtp[p] = dtval * (3 - p) as f64;
                        dsval *= s;
                        dtval *= t;
                    }
                }
                let (vsp, vtp, vdsp, vdtp) = (
                    Vector4::from_array(sp),
                    Vector4::from_array(tp),
                    Vector4::from_array(dsp),
                    Vector4::from_array(dtp),
                );
                let (mut vert, mut sdir, mut tdir) = ([0f64; 3], [0f64; 3], [0f64; 3]);
                for i in 0..3 {
                    vert[i] = (mgm[i] * vsp).dot(vtp);
                    sdir[i] = (mgm[i] * vdsp).dot(vtp);
                    tdir[i] = (mgm[i] * vsp).dot(vdtp);
                }
                let norm = Vector3::from_array(tdir).cross(Vector3::from_array(sdir));
                let length = norm.length();
                let norm = if length == 0. { norm } else { norm / length };
                let out = if vert[0] == 0. && vert[1] == 0. {
                    Vector3::new(0., if vert[2] > max_height2 { 1. } else { -1. }, 0.)
                } else {
                    Vector3::new(norm.x, norm.z, -norm.y)
                };
                vertices[vert_count * 3] = (true_size * vert[0]) as f32;
                vertices[vert_count * 3 + 1] = (true_size * (vert[2] - max_height2)) as f32;
                vertices[vert_count * 3 + 2] = (-true_size * vert[1]) as f32;
                normals[vert_count * 3..vert_count * 3 + 3].copy_from_slice(&[
                    out.x as f32,
                    out.y as f32,
                    out.z as f32,
                ]);
                uvs[vert_count * 2] = (1. - t) as f32;
                uvs[vert_count * 2 + 1] = (1. - s) as f32;
                vert_count += 1;
            }
        }
        for sstep in 0..segments {
            for tstep in 0..segments {
                let v1 = surf_count * per_row * per_row + sstep * per_row + tstep;
                let (v2, v4) = (v1 + 1, v1 + per_row);
                let v3 = v2 + per_row;
                if not_degenerate(&vertices, v1, v2, v3) {
                    indices[index_count..index_count + 3]
                        .copy_from_slice(&[v1 as u32, v2 as u32, v3 as u32]);
                    index_count += 3;
                }
                if not_degenerate(&vertices, v1, v3, v4) {
                    indices[index_count..index_count + 3]
                        .copy_from_slice(&[v1 as u32, v3 as u32, v4 as u32]);
                    index_count += 3;
                }
            }
        }
        surf_count += 1;
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(vertices)?);
    g.set_attribute("normal", vec3s(normals)?);
    g.set_attribute("uv", Attribute::F32(BufferAttribute::new(uvs, 2, false)?));
    // The unused tail of the index array stays zero, as the original's Uint32Array.
    g.set_index(Some(indices));
    Ok(g)
}
/// Tessellation, bottom, lid, body, fitLid and blinn.
type TeapotKey = (u32, bool, bool, bool, bool, bool);
const TESSELLATION: [u32; 12] = [2, 3, 4, 5, 6, 8, 10, 15, 20, 30, 40, 50];
/// MeshSurfaceSampler over a non-indexed geometry, unweighted or uv-weighted.
struct Sampler {
    positions: Vec<f32>,
    normals: Vec<f32>,
    distribution: Vec<f32>,
}
impl Sampler {
    fn new(positions: &[f32], normals: &[f32], uvs: Option<&[f32]>) -> Self {
        let faces = positions.len() / 9;
        let p = |i: usize| {
            Vector3::new(
                positions[i * 3] as f64,
                positions[i * 3 + 1] as f64,
                positions[i * 3 + 2] as f64,
            )
        };
        let mut distribution = Vec::with_capacity(faces);
        let mut total = 0f64;
        for f in 0..faces {
            let (i0, i1, i2) = (3 * f, 3 * f + 1, 3 * f + 2);
            let mut weight = 1.;
            if let Some(uv) = uvs {
                // getX of the uv attribute: the u coordinates.
                weight = uv[i0 * 2] as f64 + uv[i1 * 2] as f64 + uv[i2 * 2] as f64;
            }
            // Triangle.getArea: | ( c - b ) × ( a - b ) | / 2.
            let (a, b, c) = (p(i0), p(i1), p(i2));
            let area = (c - b).cross(a - b).length() * 0.5;
            // faceWeights and the cumulative distribution are Float32Arrays.
            let w = (weight * area) as f32;
            total += w as f64;
            distribution.push(total as f32);
        }
        Self {
            positions: positions.to_vec(),
            normals: normals.to_vec(),
            distribution,
        }
    }
    fn sample(&self, seed: &mut u32) -> (Vector3, Vector3) {
        let total = *self.distribution.last().unwrap_or(&0.) as f64;
        let x = random(seed) * total;
        let d = &self.distribution;
        let (mut start, mut end, mut face) = (0i64, d.len() as i64 - 1, -1i64);
        while start <= end {
            let mid = ((start + end) as f64 / 2.).ceil() as i64;
            let m = mid as usize;
            if mid == 0 || (d[m - 1] as f64 <= x && d[m] as f64 > x) {
                face = mid;
                break;
            } else if x < d[m] as f64 {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        let face = face.max(0) as usize;
        let (mut u, mut v) = (random(seed), random(seed));
        if u + v > 1. {
            u = 1. - u;
            v = 1. - v;
        }
        let at = |a: &[f32], i: usize| {
            Vector3::new(a[i * 3] as f64, a[i * 3 + 1] as f64, a[i * 3 + 2] as f64)
        };
        let (i0, i1, i2) = (face * 3, face * 3 + 1, face * 3 + 2);
        let w = 1. - (u + v);
        let position =
            at(&self.positions, i0) * u + at(&self.positions, i1) * v + at(&self.positions, i2) * w;
        let normal =
            (at(&self.normals, i0) * u + at(&self.normals, i1) * v + at(&self.normals, i2) * w)
                .normalize_or_zero();
        (position, normal)
    }
}
/// `Object3D.lookAt` for a non-camera object at `position`: +Z toward the target.
fn look_quaternion(position: Vector3, target: Vector3) -> Quaternion {
    let mut z = target - position;
    z = if z.length_squared() == 0. {
        Vector3::Z
    } else {
        z.normalize()
    };
    let mut x = Vector3::Y.cross(z);
    if x.length_squared() == 0. {
        if Vector3::Y.z.abs() == 1. {
            z.x += 0.0001;
        } else {
            z.z += 0.0001;
        }
        z = z.normalize();
        x = Vector3::Y.cross(z);
    }
    let x = x.normalize();
    let y = z.cross(x);
    Quaternion::from_mat3(&Matrix3::from_cols(x, y, z))
}
struct Scatter {
    stem: Object3D,
    blossom: Object3D,
    colors: Vec<Color>,
    ages: Vec<f32>,
    scales: Vec<f32>,
    matrices: Vec<[f32; 16]>,
    sampler: Sampler,
    surface: (Vec<f32>, Vec<f32>, Vec<f32>),
    group: Object3D,
    count: usize,
}
fn ease_out_cubic(t: f64) -> f64 {
    let t = t - 1.;
    t * t * t + 1.
}
fn scale_curve(t: f64) -> f64 {
    ease_out_cubic((if t > 0.5 { 1. - t } else { t }) * 2.).abs()
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    controls: Option<Controls>,
    trackball: Option<Trackball>,
    mesh: Option<Object3D>,
    params: [f32; 7],
    dirty: bool,
    // MDD
    times: Vec<f64>,
    // Edge split
    merged: Option<Merged>,
    split_cache: HashMap<(bool, u32, bool), Arc<BufferGeometry>>,
    cerberus_map: Option<Arc<crate::material::Texture>>,
    // Teapot
    teapots: HashMap<TeapotKey, Arc<BufferGeometry>>,
    materials: Vec<Arc<Material>>,
    sky: Option<Object3D>,
    // Scatter
    scatter: Option<Scatter>,
    /// One resident node per shown geometry/material combination: swapping a node's
    /// geometry or material would rebuild its draw resources.
    variants: HashMap<String, Object3D>,
    shown: Option<Object3D>,
    edge_materials: HashMap<(bool, bool), Arc<Material>>,
    base: Option<Arc<Material>>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            223 => (35., 0.1, 100., Vector3::new(8., 8., 8.)),
            224 => (75., 0.1, 2000., Vector3::new(0., 0., 4.)),
            225 => (60., 0.1, 10., Vector3::new(0., 0., 2.)),
            226 => (45., 1., 80000., Vector3::new(-600., 550., 1300.)),
            _ => (60., 0.1, 100., Vector3::new(25., 25., 25.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = position;
        s.look_at(c, Vector3::ZERO)?;
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            controls: None,
            trackball: None,
            mesh: None,
            params: [0.; 7],
            dirty: true,
            times: vec![],
            merged: None,
            split_cache: HashMap::new(),
            cerberus_map: None,
            teapots: HashMap::new(),
            materials: vec![],
            sky: None,
            scatter: None,
            variants: HashMap::new(),
            shown: None,
            edge_materials: HashMap::new(),
            base: None,
        };
        match id {
            223 => d.mdd(s).await?,
            224 => d.edgesplit(s).await?,
            225 => d.tds(s, c).await?,
            226 => d.teapot_scene(s, r).await?,
            _ => d.scatter_scene(s).await?,
        }
        if let Some(controls) = &mut d.controls {
            controls.update(s, c)?;
        }
        Ok(d)
    }
    async fn mdd(&mut self, s: &mut Scene) -> Result<()> {
        let (times, targets) = parse_mdd(&fetch("/web/gallery/assets/mdd/cube.mdd").await?)?;
        let mut g = BoxGeometry::build(1., 1., 1.)?;
        let count = targets.len();
        g.morph_attributes.insert(
            "position".into(),
            targets.into_iter().map(vec3s).collect::<Result<_>>()?,
        );
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Normal(MeshNormalMaterial::default())),
        )));
        s.get_mut(mesh)?.morph_weights = vec![0.; count];
        self.mesh = Some(mesh);
        self.times = times;
        Ok(())
    }
    async fn edgesplit(&mut self, s: &mut Scene) -> Result<()> {
        let hemisphere = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x444444),
            intensity: 3.,
        }));
        // HemisphereLight sits at Object3D.DEFAULT_UP.
        s.get_mut(hemisphere)?.position = Vector3::Y;
        let objects = parse_obj(&String::from_utf8_lossy(
            &fetch("/web/gallery/assets/obj/cerberus/Cerberus.obj").await?,
        ));
        let o = objects
            .first()
            .ok_or(Error::Asset("Cerberus object".into()))?;
        let (positions, normals, uvs, index) = merge_vertices(&o.positions, &o.normals, &o.uvs);
        self.merged = Some(Merged {
            positions,
            normals,
            uvs,
            index,
        });
        let mut map =
            decode_texture_image(&fetch("/web/gallery/assets/obj/cerberus/Cerberus_A.jpg").await?)
                .await?;
        map.srgb = true;
        map.mipmap_filter = Some(Filter::Linear);
        self.cerberus_map = Some(Arc::new(map));
        // params: showMap, smoothShading, edgeSplit, cutOffAngle, tryKeepNormals.
        self.params[..5].copy_from_slice(&[0., 1., 1., 20., 1.]);
        let m = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        self.base = Some(Arc::new(Material::Standard(m)));
        let mut controls = Controls::new(Some(0.25), (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::ZERO);
        self.controls = Some(controls);
        Ok(())
    }
    /// Show the resident node for `name`, creating it on first use; hide the previous one.
    fn show_variant(
        &mut self,
        s: &mut Scene,
        name: String,
        make: impl FnOnce(&mut Scene) -> Result<Object3D>,
    ) -> Result<()> {
        let h = match self.variants.get(&name) {
            Some(&h) => h,
            None => {
                let h = make(s)?;
                self.variants.insert(name, h);
                h
            }
        };
        if let Some(previous) = self.shown.filter(|&p| p != h) {
            s.get_mut(previous)?.visible = false;
        }
        s.get_mut(h)?.visible = true;
        self.shown = Some(h);
        self.mesh = Some(h);
        Ok(())
    }
    fn split_geometry(&mut self) -> Result<Arc<BufferGeometry>> {
        let merged = self.merged.as_ref().ok_or(Error::Invalid("merged model"))?;
        let (split, angle, keep) = (self.params[2] > 0.5, self.params[3], self.params[4] > 0.5);
        let key = (
            split,
            if split { angle.to_bits() } else { 0 },
            split && keep,
        );
        if let Some(g) = self.split_cache.get(&key) {
            return Ok(g.clone());
        }
        let g = if split {
            edge_split(merged, angle as f64 * PI / 180., keep)?
        } else {
            let mut g = BufferGeometry::default();
            g.set_attribute("position", vec3s(merged.positions.clone())?);
            g.set_attribute("normal", vec3s(merged.normals.clone())?);
            g.set_attribute(
                "uv",
                Attribute::F32(BufferAttribute::new(merged.uvs.clone(), 2, false)?),
            );
            g.set_index(Some(merged.index.clone()));
            g
        };
        let g = Arc::new(g);
        // Each combination is built once and stays resident; the original rebuilds it.
        self.split_cache.insert(key, g.clone());
        Ok(g)
    }
    async fn tds(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 3.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0xffeedd),
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0., 0., 2.);
        let base = "/web/gallery/assets/3ds/portalgun";
        let load = |file: String| async move {
            // TextureLoader without a color space: NoColorSpace data, mipmapped.
            let mut t =
                decode_texture_image(&fetch(&format!("{base}/textures/{file}")).await?).await?;
            t.srgb = false;
            t.mipmap_filter = Some(Filter::Linear);
            Ok::<_, Error>(Arc::new(t))
        };
        let normal = load("normal.jpg".into()).await?;
        let (meshes, materials, scale) =
            parse_3ds(&fetch(&format!("{base}/portalgun.3ds")).await?)?;
        let group = s.insert(NodeKind::Group);
        if let Some(scale) = scale {
            s.get_mut(group)?.scale = Vector3::splat(scale);
        }
        for mesh in meshes {
            let mut g = BufferGeometry::default();
            g.set_attribute("position", vec3s(mesh.positions)?);
            if !mesh.uvs.is_empty() {
                g.set_attribute(
                    "uv",
                    Attribute::F32(BufferAttribute::new(mesh.uvs, 2, false)?),
                );
            }
            g.set_index(Some(mesh.index));
            if let Some(matrix) = mesh.matrix {
                g.apply_matrix4(matrix.inverse())?;
            }
            g.compute_vertex_normals()?;
            let mut m = MeshPhongMaterial::default();
            if let Some(info) = mesh.material.as_ref().and_then(|n| materials.get(n)) {
                if let Some(c) = info.color {
                    m.properties.color = Color::linear(c[0], c[1], c[2]);
                }
                if let Some(sh) = info.shininess {
                    m.shininess = sh;
                }
                if info.double_side {
                    m.properties.side = Side::Double;
                }
                if let Some(file) = &info.map {
                    m.properties.map = Some(load(file.clone()).await?);
                }
            }
            // The example: specular.setScalar( 0.1 ) and the shared normal map.
            m.specular = Color::linear(0.1, 0.1, 0.1);
            m.normal_map = Some(normal.clone());
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(g),
                Arc::new(Material::Phong(m)),
            )));
            if let Some(matrix) = mesh.matrix {
                let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
                let n = s.get_mut(h)?;
                n.position = translation;
                n.quaternion = rotation;
                n.scale = scale;
            }
            s.add(group, h)?;
        }
        let (w, h, _) = viewport_css();
        self.trackball = Some(Trackball::new(s, c, Vector2::new(w, h))?);
        Ok(())
    }
    async fn teapot_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x7c7c7c),
            intensity: 2.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 2.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0.32, 0.39, 0.7);
        let env = cube(
            r,
            "/web/gallery/assets/environment-materials/textures/cube/pisa",
        )
        .await?;
        let mut grid = decode_texture_image(
            &fetch("/web/gallery/assets/environment-materials/textures/uv_grid_opengl.jpg").await?,
        )
        .await?;
        grid.srgb = true;
        grid.wrap_s = Wrapping::Repeat;
        grid.wrap_t = Wrapping::Repeat;
        grid.anisotropy = 16;
        grid.mipmap_filter = Some(Filter::Linear);
        let grid = Arc::new(grid);
        let double = |mut p: MaterialProperties| {
            p.side = Side::Double;
            p
        };
        let mut wire = MeshBasicMaterial::default();
        wire.properties.wireframe = true;
        let mut flat = MeshPhongMaterial {
            specular: Color::BLACK,
            ..Default::default()
        };
        flat.properties.flat_shading = true;
        flat.properties = double(flat.properties);
        let mut smooth = MeshLambertMaterial::default();
        smooth.properties = double(smooth.properties);
        let mut glossy = MeshPhongMaterial {
            specular: Color::from_hex(0x404040),
            shininess: 300.,
            ..Default::default()
        };
        glossy.properties.color = Color::from_hex(0xc0c0c0);
        glossy.properties = double(glossy.properties);
        let mut textured = MeshPhongMaterial::default();
        textured.properties.map = Some(grid);
        textured.properties = double(textured.properties);
        // Reflective: Phong lighting, then envmap_fragment's MultiplyOperation with the
        // per-fragment reflection of the view ray about the (face-flipped) world normal.
        let reflect = WgslFn::new(
            "teapot_reflect",
            "fn teapot_reflect(value:vec4<f32>)->vec4<f32>{let n=normalize(fragment_surface.normal)*select(-1.0,1.0,fragment_front);let w=normalize(transpose(mat3x3(u.view[0].xyz,u.view[1].xyz,u.view[2].xyz))*n);let r=reflect(normalize(fragment_surface.position-u.camera.xyz),w);let e=textureSample(tsl_texture_0,tsl_sampler_0,vec3(-r.x,r.yz)).rgb;return vec4(value.rgb*e,value.a);}",
            &[Type::Vec4],
            Type::Vec4,
        )?
        .call(&[output()]);
        let program = SurfaceNodes {
            output: Some(reflect),
            ..Default::default()
        }
        .build_with_texture_types(r, &[], &[(&env.view, &env.sampler, Type::TextureCube)])
        .await?;
        let mut reflective = MeshPhongMaterial::default();
        reflective.properties.vertex_program = Some(Arc::new(program));
        reflective.properties = double(reflective.properties);
        self.materials = vec![
            Arc::new(Material::Basic(wire)),
            Arc::new(Material::Phong(flat)),
            Arc::new(Material::Lambert(smooth)),
            Arc::new(Material::Phong(glossy)),
            Arc::new(Material::Phong(textured)),
            Arc::new(Material::Phong(reflective)),
        ];
        // scene.background = textureCube for the reflective shading: a camera-centered box.
        let graph = NodeMaterial::new(vec4(
            WgslFn::new(
                "sky_color",
                "fn sky_color(d:vec3<f32>)->vec3<f32>{return textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb;}",
                &[Type::Vec3],
                Type::Vec3,
            )?
            .call(&[position_local()]),
            float(1.),
        ));
        let source = graph.wgsl_with_texture_types(&[Type::TextureCube], &[])?;
        // backgroundCube: gl_Position.z = gl_Position.w, so the unit box is never near-clipped.
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection_and_dimensions(
                r,
                &source,
                &[(&env.view, &env.sampler)],
                &[wgpu::TextureViewDimension::Cube],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(out.clip.xy,out.clip.w,out.clip.w);return out;}",
            )
            .await?,
        ));
        m.properties.side = Side::Back;
        m.properties.depth_write = false;
        m.properties.depth_test = false;
        let sky = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 1., 1.)?),
            Arc::new(Material::Shader(m)),
        )));
        let n = s.get_mut(sky)?;
        n.frustum_culled = false;
        n.render_order = -10000;
        n.visible = false;
        self.sky = Some(sky);
        // effectController: newTess 15, bottom, lid, body, fitLid false, nonblinn false, glossy.
        self.params = [7., 1., 1., 1., 0., 0., 3.];
        self.controls = Some(Controls::new(None, (0., f64::INFINITY), PI, true));
        Ok(())
    }
    fn update_teapot(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let tess = TESSELLATION[(self.params[0] as usize).min(11)];
        let [_, lid, body, bottom, fit, nonblinn, shading] = self.params.map(|v| v > 0.5);
        let _ = shading;
        let key = (tess, bottom, lid, body, fit, !nonblinn);
        // Each combination is built once and stays resident; the original rebuilds it.
        let g = match self.teapots.get(&key) {
            Some(g) => g.clone(),
            None => {
                let g = Arc::new(teapot(300., tess, bottom, lid, body, fit, !nonblinn)?);
                self.teapots.insert(key, g.clone());
                g
            }
        };
        let shading = (self.params[6] as usize).min(5);
        let material = self.materials[shading].clone();
        let name = format!("{key:?}/{shading}");
        self.show_variant(s, name, |s| {
            Ok(s.insert(NodeKind::Mesh(Mesh::new(g, material))))
        })?;
        // Reflective shading shows the cube as background; otherwise background is null.
        if let Some(sky) = self.sky {
            let p = s.get(c)?.position;
            let n = s.get_mut(sky)?;
            n.visible = shading == 5;
            n.position = p;
        }
        Ok(())
    }
    async fn scatter_scene(&mut self, s: &mut Scene) -> Result<()> {
        s.background = Color::from_hex(0xe39469);
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::from_hex(0xaa8899),
            intensity: 2.5,
            distance: 0.,
            decay: 0.,
        }));
        s.get_mut(light)?.position = Vector3::new(50., -25., 75.);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 3.,
        }));
        // scene.rotation turns everything in the scene, the point light included.
        let group = s.insert(NodeKind::Group);
        s.add(group, light)?;
        let surface_g = TorusKnotGeometry::build(10., 3., 100, 16, 2, 3)?.to_non_indexed()?;
        let (positions, normals, uvs) = (
            floats(&surface_g, "position"),
            floats(&surface_g, "normal"),
            floats(&surface_g, "uv"),
        );
        let mut surface_m = MeshLambertMaterial::default();
        surface_m.properties.color = Color::from_hex(0xfff784);
        let (asset, buffers, images) =
            load_asset("/web/gallery/assets/gltf/Flower/Flower.glb").await?;
        let nodes = crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        let mut parts = HashMap::new();
        for h in nodes {
            let n = s.get(h)?;
            if let NodeKind::Mesh(m) = &n.kind {
                parts.insert(n.name.clone(), (m.geometry.clone(), m.materials[0].clone()));
            }
            s.dispose(h)?;
        }
        // defaultTransform: makeRotationX( PI ) × makeScale( 7 ).
        let transform = Matrix4::from_rotation_x(PI) * Matrix4::from_scale(Vector3::splat(7.));
        let mut part = |name: &str| -> Result<Object3D> {
            let (g, m) = parts
                .get(name)
                .cloned()
                .ok_or(Error::Asset(format!("Flower {name}")))?;
            let mut g = (*g).clone();
            g.apply_matrix4(transform)?;
            let mut m = (*m).clone();
            if let Material::Standard(m) = &mut m {
                m.energy_conservation = true;
            }
            let h = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(g), Arc::new(m))));
            s.get_mut(h)?.frustum_culled = false;
            s.add(group, h)?;
            Ok(h)
        };
        let (stem, blossom) = (part("Stem")?, part("Blossom")?);
        let surface = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(surface_g),
            Arc::new(Material::Lambert(surface_m)),
        )));
        s.add(group, surface)?;
        let count = 2000;
        let palette = [0xf20587, 0xf2d479, 0xf2c879, 0xf2b077, 0xf24405];
        let colors: Vec<Color> = (0..count)
            .map(|_| Color::from_hex(palette[(random(&mut self.seed) * 5.).floor() as usize]))
            .collect();
        self.scatter = Some(Scatter {
            stem,
            blossom,
            colors,
            ages: vec![0.; count],
            scales: vec![0.; count],
            matrices: vec![[0.; 16]; count],
            sampler: Sampler::new(&positions, &normals, None),
            surface: (positions, normals, uvs),
            group,
            count,
        });
        self.params[0] = count as f32;
        self.resample()?;
        Ok(())
    }
    /// resample(): a new sampler, ages, scales and every particle's matrix.
    fn resample(&mut self) -> Result<()> {
        let sc = self.scatter.as_mut().ok_or(Error::Invalid("scatter"))?;
        for i in 0..sc.ages.len() {
            sc.ages[i] = random(&mut self.seed) as f32;
            sc.scales[i] = scale_curve(sc.ages[i] as f64) as f32;
            sc.matrices[i] = resample_particle(&sc.sampler, sc.scales[i], &mut self.seed);
        }
        Ok(())
    }
    fn push_instances(&self, s: &mut Scene) -> Result<()> {
        let sc = self.scatter.as_ref().ok_or(Error::Invalid("scatter"))?;
        let count = (self.params[0].max(0.) as usize).min(sc.count);
        for (h, colored) in [(sc.stem, false), (sc.blossom, true)] {
            let n = s.get_mut(h)?;
            n.instances = sc
                .matrices
                .iter()
                .enumerate()
                .map(|(i, m)| Instance {
                    matrix: Matrix4::from_cols_array(&m.map(|v| v as f64)),
                    color: if colored { sc.colors[i] } else { Color::WHITE },
                })
                .collect();
            n.instance_count = Some(count as u32);
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let step = (t - self.last) * 60.;
        self.last = t;
        match self.id {
            223 => {
                // AnimationMixer: LoopRepeat over the last key time, linear one-hot influences.
                let duration = *self.times.last().unwrap_or(&1.);
                let mut time = t;
                if duration > 0. && time >= duration {
                    time -= duration * (time / duration).floor();
                }
                let n = self.times.len();
                let mut weights = vec![0.; n];
                match self.times.iter().position(|&k| k > time) {
                    Some(0) => weights[0] = 1.,
                    Some(i) => {
                        let a = (time - self.times[i - 1]) / (self.times[i] - self.times[i - 1]);
                        weights[i - 1] = 1. - a;
                        weights[i] = a;
                    }
                    None => weights[n - 1] = 1.,
                }
                if let Some(mesh) = self.mesh {
                    s.get_mut(mesh)?.morph_weights = weights;
                }
            }
            224 => {
                if self.dirty {
                    let g = self.split_geometry()?;
                    let (show, smooth) = (self.params[0] > 0.5, self.params[1] > 0.5);
                    let map = self.cerberus_map.clone();
                    let base = self
                        .base
                        .clone()
                        .ok_or(Error::Invalid("edge-split material"))?;
                    let material = self
                        .edge_materials
                        .entry((show, smooth))
                        .or_insert_with(|| {
                            let mut m = (*base).clone();
                            if let Material::Standard(sm) = &mut m {
                                sm.properties.flat_shading = !smooth;
                                sm.properties.map = if show { map } else { None };
                            }
                            Arc::new(m)
                        })
                        .clone();
                    let name = format!("{:?}/{show}/{smooth}", Arc::as_ptr(&g));
                    self.show_variant(s, name, |s| {
                        let h = s.insert(NodeKind::Mesh(Mesh::new(g, material)));
                        let n = s.get_mut(h)?;
                        n.quaternion = Quaternion::from_rotation_y(-PI / 2.);
                        n.scale = Vector3::splat(3.5);
                        // translateZ( 1.5 ) along the rotated local axis.
                        n.position = n.quaternion * Vector3::new(0., 0., 1.5);
                        Ok(h)
                    })?;
                    self.dirty = false;
                }
            }
            225 => {
                let (w, h, _) = viewport_css();
                if let Some(t) = &mut self.trackball {
                    t.screen = Vector2::new(w, h);
                    for _ in 0..step.round().max(0.) as usize {
                        t.update(s)?;
                    }
                }
            }
            226 => {
                if self.dirty {
                    self.update_teapot(s, c)?;
                    self.dirty = false;
                }
                if let Some(sky) = self.sky {
                    let p = s.get(c)?.position;
                    s.get_mut(sky)?.position = p;
                }
            }
            _ => {
                let group = self.scatter.as_ref().map(|sc| sc.group);
                if let Some(group) = group {
                    s.get_mut(group)?.quaternion = Euler {
                        angles: Vector3::new((t / 4.).sin(), (t / 2.).sin(), 0.),
                        order: EulerOrder::XYZ,
                    }
                    .quaternion();
                }
                let count = self.params[0].max(0.) as usize;
                for _ in 0..step.round().max(0.) as usize {
                    let sc = self.scatter.as_mut().ok_or(Error::Invalid("scatter"))?;
                    for i in 0..count.min(sc.count) {
                        update_particle(sc, i, &mut self.seed);
                    }
                }
                self.push_instances(s)?;
            }
        }
        Ok(())
    }
    /// Absolute CSS-pixel pointer events for TrackballControls.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(t) = &mut self.trackball {
            match kind {
                10..=19 => t.down(kind - 10, x, y),
                20..=29 => t.state = Mode::None,
                _ => t.moved(x, y),
            }
        }
    }
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
        if let Some(t) = &mut self.trackball {
            // deltaMode 0: zoomStart.y -= deltaY × 0.00025.
            if wheel != 0. {
                t.zoom_start.y -= wheel * 0.00025;
            }
            return Ok(());
        }
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        // The edge-split example sets rotateSpeed 0.35.
        let speed = if self.id == 224 { 0.35 } else { 1. };
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx * speed, dy * speed, height);
        }
        // Each pointer and wheel handler calls update().
        controls.update(s, c)
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(t) = &mut self.trackball {
            if !down {
                t.key_state = Mode::None;
            } else if t.key_state == Mode::None {
                t.key_state = match code {
                    65 => Mode::Rotate,
                    83 => Mode::Zoom,
                    68 => Mode::Pan,
                    _ => Mode::None,
                };
            }
        }
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (224, 0..=4) | (226, 0..=6) => {
                self.params[index] = value;
                self.dirty = true;
            }
            // count: stemMesh.count = blossomMesh.count = api.count.
            (227, 0) if (0. ..=2000.).contains(&value) => self.params[0] = value,
            (227, 1) => {
                // distribution: 'random' or uv-'weighted'; onChange( resample ).
                let sc = self.scatter.as_mut().ok_or(Error::Invalid("scatter"))?;
                let (p, n, uv) = &sc.surface;
                sc.sampler = Sampler::new(p, n, (value > 0.5).then_some(uv.as_slice()));
                self.resample()?;
            }
            _ => return Err(Error::Invalid("models/modifiers parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// resampleParticle( i ): position on the surface, facing along the normal, scaled.
fn resample_particle(sampler: &Sampler, scale: f32, seed: &mut u32) -> [f32; 16] {
    let (position, normal) = sampler.sample(seed);
    let q = look_quaternion(position, normal + position);
    let m = Matrix4::from_scale_rotation_translation(Vector3::splat(scale as f64), q, position);
    m.to_cols_array().map(|v| v as f32)
}
/// updateParticle( i ): age by one frame; rescale the Float32 matrix or resample it.
fn update_particle(sc: &mut Scatter, i: usize, seed: &mut u32) {
    sc.ages[i] = (sc.ages[i] as f64 + 0.005) as f32;
    if sc.ages[i] >= 1. {
        sc.ages[i] = 0.001;
        sc.scales[i] = scale_curve(sc.ages[i] as f64) as f32;
        sc.matrices[i] = resample_particle(&sc.sampler, sc.scales[i], seed);
        return;
    }
    let previous = sc.scales[i] as f64;
    sc.scales[i] = scale_curve(sc.ages[i] as f64) as f32;
    let k = sc.scales[i] as f64 / previous;
    // Matrix4.scale( v ): columns 0..2 times the factor, stored back as Float32.
    for e in sc.matrices[i].iter_mut().take(12) {
        *e = (*e as f64 * k) as f32;
    }
}
