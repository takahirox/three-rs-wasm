//! A tile's model: TilesRenderer.parseTile for glb content ( GLTFLoader with
//! Draco, the up-axis rotation and the tile transform ), then
//! TileCreasedNormalsPlugin's processTileModel ( non-indexed geometry with
//! normals creased at 30°, computed on the CPU as its worker does ), and
//! estimateBytesUsed for the cache.
use super::volume::{M4, multiply};
use crate::attribute::BufferAttribute;
use crate::geometry::{Attribute, BufferGeometry};
use crate::material::Material;
use crate::scene::{NodeKind, Scene};
use crate::{Error, Result};
use std::sync::Arc;

pub(crate) struct TileContent {
    /// Each mesh's world matrix, geometry and material.
    pub meshes: Vec<(M4, Arc<BufferGeometry>, Arc<Material>)>,
    /// Each mesh's geometry.boundingSphere ( local ), for the raycasts.
    pub spheres: Vec<([f64; 3], f64)>,
    pub bytes: f64,
}

/// computeCreasedNormals( positions, creaseAngle ).
pub(crate) fn creased_normals(positions: &[f32], crease_angle: f64) -> Vec<f32> {
    let crease_dot = crate::bounce::trig::cos(crease_angle);
    let hash_multiplier = (1. + 1e-10) * 1e2;
    let vertex_count = positions.len() / 3;
    let face_count = vertex_count / 3;
    let mut face_normals = vec![0f64; face_count * 3];
    for f in 0..face_count {
        let p = |k: usize| positions[9 * f + k] as f64;
        let (ax, ay, az) = (p(0), p(1), p(2));
        let (bx, by, bz) = (p(3), p(4), p(5));
        let (cx, cy, cz) = (p(6), p(7), p(8));
        let (v1x, v1y, v1z) = (cx - bx, cy - by, cz - bz);
        let (v2x, v2y, v2z) = (ax - bx, ay - by, az - bz);
        let nx = v1y * v2z - v1z * v2y;
        let ny = v1z * v2x - v1x * v2z;
        let nz = v1x * v2y - v1y * v2x;
        let l = (nx * nx + ny * ny + nz * nz).sqrt();
        let inv = 1. / if l == 0. || l.is_nan() { 1. } else { l };
        face_normals[3 * f] = nx * inv;
        face_normals[3 * f + 1] = ny * inv;
        face_normals[3 * f + 2] = nz * inv;
    }
    // ~~( x ): truncation to int32.
    let to_i32 = |v: f64| -> i32 {
        if v.is_finite() {
            (v.trunc() as i64) as i32
        } else {
            0
        }
    };
    let mut vertex_ids = vec![0i32; vertex_count];
    let mut quantized = vec![0i32; vertex_count * 3];
    let mut table_size = 1usize;
    while table_size < vertex_count * 2 {
        table_size <<= 1;
    }
    let mask = (table_size - 1) as i32;
    let mut table = vec![0i32; table_size];
    let mut unique = 0i32;
    for i in 0..vertex_count {
        let qx = to_i32(positions[3 * i] as f64 * hash_multiplier);
        let qy = to_i32(positions[3 * i + 1] as f64 * hash_multiplier);
        let qz = to_i32(positions[3 * i + 2] as f64 * hash_multiplier);
        let mut slot =
            (qx.wrapping_mul(73856093) ^ qy.wrapping_mul(19349663) ^ qz.wrapping_mul(83492791))
                & mask;
        loop {
            let id = table[slot as usize];
            if id == 0 {
                let q = 3 * unique as usize;
                quantized[q] = qx;
                quantized[q + 1] = qy;
                quantized[q + 2] = qz;
                table[slot as usize] = unique + 1;
                vertex_ids[i] = unique;
                unique += 1;
                break;
            }
            let q = 3 * (id - 1) as usize;
            if quantized[q] == qx && quantized[q + 1] == qy && quantized[q + 2] == qz {
                vertex_ids[i] = id - 1;
                break;
            }
            slot = (slot + 1) & mask;
        }
    }
    let unique = unique as usize;
    let mut offsets = vec![0i32; unique + 1];
    for &id in &vertex_ids {
        offsets[id as usize + 1] += 1;
    }
    for i in 0..unique {
        offsets[i + 1] += offsets[i];
    }
    let mut bucket_faces = vec![0i32; vertex_count];
    let mut cursors = offsets[..unique].to_vec();
    for f in 0..face_count {
        for k in 0..3 {
            let id = vertex_ids[3 * f + k] as usize;
            bucket_faces[cursors[id] as usize] = f as i32;
            cursors[id] += 1;
        }
    }
    let mut normals = vec![0f32; vertex_count * 3];
    for f in 0..face_count {
        let (nx, ny, nz) = (
            face_normals[3 * f],
            face_normals[3 * f + 1],
            face_normals[3 * f + 2],
        );
        for n in 0..3 {
            let i = 3 * f + n;
            let id = vertex_ids[i] as usize;
            let (mut sx, mut sy, mut sz) = (0., 0., 0.);
            for k in offsets[id]..offsets[id + 1] {
                let o = 3 * bucket_faces[k as usize] as usize;
                let (ox, oy, oz) = (face_normals[o], face_normals[o + 1], face_normals[o + 2]);
                if nx * ox + ny * oy + nz * oz > crease_dot {
                    sx += ox;
                    sy += oy;
                    sz += oz;
                }
            }
            let l = (sx * sx + sy * sy + sz * sz).sqrt();
            let inv = 1. / if l == 0. || l.is_nan() { 1. } else { l };
            normals[3 * i] = (sx * inv) as f32;
            normals[3 * i + 1] = (sy * inv) as f32;
            normals[3 * i + 2] = (sz * inv) as f32;
        }
    }
    normals
}

/// BufferGeometry.toNonIndexed for float attributes.
fn non_indexed(g: &BufferGeometry) -> Result<BufferGeometry> {
    let Some(index) = g.get_index().map(|i| i.to_vec()) else {
        return Ok(g.clone());
    };
    let mut out = BufferGeometry::default();
    for (name, attribute) in &g.attributes {
        let Attribute::F32(a) = attribute else {
            continue;
        };
        let size = a.item_size();
        let src = a.array();
        let mut values = Vec::with_capacity(index.len() * size);
        for &i in &index {
            values.extend_from_slice(&src[i as usize * size..i as usize * size + size]);
        }
        out.set_attribute(
            name,
            Attribute::F32(BufferAttribute::new(values, size, false)?),
        );
    }
    Ok(out)
}

/// parseTile for a glb, processTileModel and estimateBytesUsed.
pub(crate) async fn parse(bytes: Vec<u8>, url: &str, transform: M4, up: M4) -> Result<TileContent> {
    let base = url
        .split('?')
        .next()
        .unwrap_or(url)
        .rsplit_once('/')
        .map_or("", |(b, _)| b)
        .to_owned();
    let (asset, buffers, images) =
        super::super::gltf_viewer::load_asset_bytes(&bytes, &base).await?;
    let imported = crate::gltf::import_decoded(&asset, &buffers, &images)?;
    let mut scratch = Scene::new();
    let handles = imported.instantiate(&mut scratch)?;
    // scene.matrix = tile transform × up rotation.
    let scene_matrix = multiply(&transform, &up);
    let mut meshes = vec![];
    let mut spheres = vec![];
    let mut total = 0.;
    for h in handles {
        let n = scratch.get(h)?;
        let NodeKind::Mesh(m) = &n.kind else { continue };
        let source = non_indexed(&m.geometry)?;
        let positions = match source.attributes.get("position") {
            Some(Attribute::F32(a)) => a.array().to_vec(),
            _ => return Err(Error::Invalid("tile positions")),
        };
        let normals = creased_normals(&positions, 30. * (std::f64::consts::PI / 180.));
        let mut geometry = source;
        geometry.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(normals, 3, false)?),
        );
        geometry.bounding_box = None;
        geometry.bounding_sphere = None;
        // estimateBytesUsed: every attribute's array bytes ( no index now ).
        for a in geometry.attributes.values() {
            if let Attribute::F32(a) = a {
                total += (a.array().len() * 4) as f64;
            }
        }
        let material = m.materials[0].clone();
        if let Some(map) = &material.properties().map {
            // RGBA8 with generated mipmaps: 4 / 3 of the base level.
            total += (map.width as f64 * map.height as f64 * 4.)
                * if map.mipmap_filter.is_some() {
                    4. / 3.
                } else {
                    1.
                };
        }
        let node = n.matrix.to_cols_array();
        spheres.push(bounding_sphere(&positions));
        meshes.push((multiply(&scene_matrix, &node), Arc::new(geometry), material));
    }
    Ok(TileContent {
        meshes,
        spheres,
        bytes: total,
    })
}

/// BufferGeometry.computeBoundingSphere: the box center, the farthest vertex.
pub(crate) fn bounding_sphere(positions: &[f32]) -> ([f64; 3], f64) {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for p in positions.as_chunks::<3>().0 {
        for k in 0..3 {
            min[k] = min[k].min(p[k] as f64);
            max[k] = max[k].max(p[k] as f64);
        }
    }
    let center = [
        (min[0] + max[0]) * 0.5,
        (min[1] + max[1]) * 0.5,
        (min[2] + max[2]) * 0.5,
    ];
    let mut radius_sq = 0f64;
    for p in positions.as_chunks::<3>().0 {
        let d = [
            p[0] as f64 - center[0],
            p[1] as f64 - center[1],
            p[2] as f64 - center[2],
        ];
        radius_sq = radius_sq.max(d[0] * d[0] + d[1] * d[1] + d[2] * d[2]);
    }
    (center, radius_sq.sqrt())
}
