//! The page's scene as three-gpu-pathtracer's PathTracingSceneGenerator sees
//! it: LDrawUtils.mergeObject over the loaded X-wing, the page's material
//! changes, the floor, then StaticGeometryGenerator's per-mesh baking
//! ( world transforms, setCommonAttributes ) and merge, and the material
//! index attribute. Every vertex value is computed in f64 and rounded to f32
//! where the original writes a Float32Array.
use super::super::ldraw_loader::{Child, FaceMaterial, Group, Kind, Loader, Object, Slot};
use super::math::{self, M4};

/// Material sides, as MaterialsTexture writes them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Side {
    Front,
    Double,
}

/// The fields of a MeshStandardMaterial or MeshPhysicalMaterial the path
/// tracer and the raster fallback read.
#[derive(Clone, Debug)]
pub(crate) struct Material {
    pub physical: bool,
    pub color: [f64; 3],
    pub roughness: f64,
    pub metalness: f64,
    pub emissive: [f64; 3],
    pub emissive_intensity: f64,
    pub opacity: f64,
    pub transparent: bool,
    pub side: Side,
    pub ior: f64,
    pub transmission: f64,
    pub thickness: f64,
    /// The floor's radial texture.
    pub map: bool,
    /// LDraw's polygonOffset ( factor 1 ).
    pub polygon_offset: bool,
}

/// A merged mesh of the model, or the floor.
pub(crate) struct Mesh {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    /// PlaneGeometry's uvs and index; the merged model meshes have neither.
    pub uvs: Option<Vec<f32>>,
    pub index: Option<Vec<u32>>,
    pub material: Material,
    pub matrix_world: M4,
}

/// The merged geometry PathTracingSceneGenerator builds.
pub(crate) struct Merged {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub tangents: Vec<f32>,
    pub uvs: Vec<f32>,
    pub colors: Vec<f32>,
    pub index: Vec<u32>,
    pub material_index: Vec<u8>,
}

/// Object3D.updateMatrixWorld down the loader's groups, then
/// LDrawUtils.mergeObject's traversal: each face object's geometry ( one per
/// material group ) in world space, keyed by material in first-seen order.
pub(crate) fn merge_object(_loader: &Loader, root: &Group) -> Vec<(usize, Vec<f32>, Vec<f32>)> {
    let mut merged: Vec<(usize, Vec<f32>, Vec<f32>)> = vec![];
    fn visit(group: &Group, parent: Option<&M4>, merged: &mut Vec<(usize, Vec<f32>, Vec<f32>)>) {
        let q = group.quaternion;
        let local = math::compose(
            [group.position.x, group.position.y, group.position.z],
            [q.x, q.y, q.z, q.w],
            [group.scale.x, group.scale.y, group.scale.z],
        );
        // updateMatrixWorld: a root's world matrix is its own matrix.
        let world = parent.map_or(local, |p| math::multiply(p, &local));
        for child in &group.children {
            match child {
                Child::Group(g) => visit(g, Some(&world), merged),
                Child::Object(o) if o.kind == Kind::Mesh => add_mesh(o, &world, merged),
                Child::Object(_) => {}
            }
        }
    }
    // The page merges the loaded group before it rotates it.
    visit(root, None, &mut merged);
    merged
}

/// The mesh's geometry clone through its world matrix ( first two vertices
/// of each triangle swapped under a mirroring matrix ), split per material
/// group when the mesh has several materials.
fn add_mesh(o: &Object, world: &M4, merged: &mut Vec<(usize, Vec<f32>, Vec<f32>)>) {
    let g = &o.geometry;
    let mut positions = g.positions.clone();
    let mut normals = g.normals.clone().unwrap_or_default();
    if math::determinant(world) < 0. {
        permute(&mut positions);
        permute(&mut normals);
    }
    // BufferGeometry.applyMatrix4: positions, then the normals through the
    // normal matrix.
    for p in positions.chunks_mut(3) {
        let v = math::apply4([p[0] as f64, p[1] as f64, p[2] as f64], world);
        p.copy_from_slice(&v.map(|x| x as f32));
    }
    let normal_matrix = math::normal_matrix(world);
    for n in normals.chunks_mut(3) {
        let v = math::apply_normal([n[0] as f64, n[1] as f64, n[2] as f64], &normal_matrix);
        n.copy_from_slice(&v.map(|x| x as f32));
    }
    let face = |slot: &Slot| match slot {
        Slot::Face(i) => *i,
        _ => usize::MAX,
    };
    let mut push = |material: usize, p: &[f32], n: &[f32]| {
        if let Some(entry) = merged.iter_mut().find(|e| e.0 == material) {
            entry.1.extend_from_slice(p);
            entry.2.extend_from_slice(n);
        } else {
            merged.push((material, p.to_vec(), n.to_vec()));
        }
    };
    if o.materials.len() == 1 {
        push(face(&o.materials[0]), &positions, &normals);
    } else {
        let vertices = positions.len() / 3;
        for &(start, count, material) in &g.groups {
            // extractGroup
            let n = count.min(vertices.saturating_sub(start));
            let (a, b) = (start * 3, (start + n) * 3);
            push(
                face(&o.materials[material]),
                &positions[a..b],
                &normals[a..b],
            );
        }
    }
}

/// permuteAttribute for triangles: each triangle's first two vertices swap.
fn permute(verts: &mut [f32]) {
    let n = verts.len() / 3;
    let mut offset = 0;
    for _ in 0..n {
        if offset + 5 >= verts.len() {
            break;
        }
        for k in 0..3 {
            verts.swap(offset + k, offset + 3 + k);
        }
        offset += 9;
    }
}

/// The page's material changes: roughness × 0.25, and translucent bricks
/// become transmissive MeshPhysicalMaterials ( HSL lightness at least 0.35 ).
pub(crate) fn page_material(f: &FaceMaterial) -> Material {
    let roughness = f.roughness * 0.25;
    let standard = Material {
        physical: false,
        color: [f.color.x, f.color.y, f.color.z],
        roughness,
        metalness: f.metalness,
        emissive: [f.emissive.x, f.emissive.y, f.emissive.z],
        emissive_intensity: 1.,
        opacity: f.opacity,
        transparent: f.transparent,
        side: Side::Front,
        ior: 1.5,
        transmission: 0.,
        thickness: 0.,
        map: false,
        polygon_offset: true,
    };
    if f.opacity < 1. {
        let [h, s, l] = get_hsl(standard.color);
        Material {
            physical: true,
            color: set_hsl(h, s, l.max(0.35)),
            roughness,
            metalness: 0.,
            emissive: [0.; 3],
            opacity: 1.,
            transparent: false,
            ior: 1.4,
            transmission: 1.,
            thickness: 1.,
            polygon_offset: false,
            ..standard
        }
    } else {
        standard
    }
}

/// Color.getHSL in the working color space.
fn get_hsl([r, g, b]: [f64; 3]) -> [f64; 3] {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let lightness = (min + max) / 2.;
    if min == max {
        return [0., 0., lightness];
    }
    let delta = max - min;
    let saturation = if lightness <= 0.5 {
        delta / (max + min)
    } else {
        delta / (2. - max - min)
    };
    let mut hue = if max == r {
        (g - b) / delta + if g < b { 6. } else { 0. }
    } else if max == g {
        (b - r) / delta + 2.
    } else {
        (r - g) / delta + 4.
    };
    hue /= 6.;
    [hue, saturation, lightness]
}

/// Color.setHSL in the working color space.
fn set_hsl(h: f64, s: f64, l: f64) -> [f64; 3] {
    // euclideanModulo( h, 1 ), clamp( s ), clamp( l ).
    let h = ((h % 1.) + 1.) % 1.;
    let s = s.clamp(0., 1.);
    let l = l.clamp(0., 1.);
    if s == 0. {
        return [l; 3];
    }
    let p = if l <= 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let q = 2. * l - p;
    let hue = |p: f64, q: f64, mut t: f64| {
        if t < 0. {
            t += 1.;
        }
        if t > 1. {
            t -= 1.;
        }
        if t < 1. / 6. {
            return q + (p - q) * 6. * t;
        }
        if t < 1. / 2. {
            return p;
        }
        if t < 2. / 3. {
            return q + (p - q) * 6. * (2. / 3. - t);
        }
        q
    };
    [hue(p, q, h + 1. / 3.), hue(p, q, h), hue(p, q, h - 1. / 3.)]
}

/// Box3.setFromObject over the merged meshes ( their geometry boxes through
/// the model's world matrix ).
pub(crate) fn bounds(meshes: &[Mesh]) -> ([f64; 3], [f64; 3]) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for m in meshes {
        let (mut a, mut b) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for p in m.positions.chunks(3) {
            for k in 0..3 {
                let v = p[k] as f64;
                if v < a[k] {
                    a[k] = v;
                }
                if v > b[k] {
                    b[k] = v;
                }
            }
        }
        for c in 0..8 {
            let corner = [
                if c & 4 != 0 { b[0] } else { a[0] },
                if c & 2 != 0 { b[1] } else { a[1] },
                if c & 1 != 0 { b[2] } else { a[2] },
            ];
            let w = math::apply4(corner, &m.matrix_world);
            for k in 0..3 {
                lo[k] = lo[k].min(w[k]);
                hi[k] = hi[k].max(w[k]);
            }
        }
    }
    (lo, hi)
}

/// PlaneGeometry( 1, 1 ): positions, normals, uvs and its Uint16 index.
pub(crate) fn plane() -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<u32>) {
    (
        vec![-0.5, 0.5, 0., 0.5, 0.5, 0., -0.5, -0.5, 0., 0.5, -0.5, 0.],
        vec![0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1.],
        vec![0., 1., 1., 1., 0., 0., 1., 0.],
        vec![0, 2, 1, 2, 3, 1],
    )
}

/// One baked geometry: convertToStaticGeometry ( world positions, normals
/// through the normal matrix ), then setCommonAttributes ( a sequential index,
/// zero uvs and uv2, computeTangents, white colors ).
struct Baked {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    tangents: Vec<f32>,
    colors: Vec<f32>,
    index: Vec<u32>,
}

fn bake(m: &Mesh) -> Baked {
    let world = &m.matrix_world;
    let normal_matrix = math::normal_matrix(world);
    let mut positions = Vec::with_capacity(m.positions.len());
    let mut normals = Vec::with_capacity(m.normals.len());
    for (p, n) in m.positions.chunks(3).zip(m.normals.chunks(3)) {
        let v = math::apply4([p[0] as f64, p[1] as f64, p[2] as f64], world);
        positions.extend(v.map(|x| x as f32));
        let v = math::apply_normal([n[0] as f64, n[1] as f64, n[2] as f64], &normal_matrix);
        normals.extend(v.map(|x| x as f32));
    }
    let count = positions.len() / 3;
    let mut baked = Baked {
        positions,
        normals,
        uvs: m.uvs.clone().unwrap_or_else(|| vec![0.; count * 2]),
        tangents: vec![],
        colors: vec![1.; count * 4],
        index: m
            .index
            .clone()
            .unwrap_or_else(|| (0..count as u32).collect()),
    };
    if math::determinant(world) < 0. {
        // invertGeometry on the index.
        for t in baked.index.chunks_mut(3) {
            t.swap(0, 2);
        }
    }
    baked.tangents = compute_tangents(&baked);
    baked
}

/// BufferGeometry.computeTangents.
fn compute_tangents(b: &Baked) -> Vec<f32> {
    let count = b.positions.len() / 3;
    let mut tan1 = vec![[0f64; 3]; count];
    let mut tan2 = vec![[0f64; 3]; count];
    let v = |i: u32| {
        let i = i as usize * 3;
        [
            b.positions[i] as f64,
            b.positions[i + 1] as f64,
            b.positions[i + 2] as f64,
        ]
    };
    let uv = |i: u32| {
        let i = i as usize * 2;
        [b.uvs[i] as f64, b.uvs[i + 1] as f64]
    };
    for t in b.index.chunks(3) {
        let (a, bb, c) = (t[0], t[1], t[2]);
        let (va, vb, vc) = (v(a), v(bb), v(c));
        let (ua, ub, uc) = (uv(a), uv(bb), uv(c));
        let vb = [vb[0] - va[0], vb[1] - va[1], vb[2] - va[2]];
        let vc = [vc[0] - va[0], vc[1] - va[1], vc[2] - va[2]];
        let ub = [ub[0] - ua[0], ub[1] - ua[1]];
        let uc = [uc[0] - ua[0], uc[1] - ua[1]];
        let r = 1.0 / (ub[0] * uc[1] - uc[0] * ub[1]);
        if !r.is_finite() {
            continue;
        }
        // sdir = ( vB * uvC.y + vC * -uvB.y ) * r; tdir = ( vC * uvB.x + vB * -uvC.x ) * r.
        let sdir: [f64; 3] = std::array::from_fn(|k| (vb[k] * uc[1] + vc[k] * -ub[1]) * r);
        let tdir: [f64; 3] = std::array::from_fn(|k| (vc[k] * ub[0] + vb[k] * -uc[0]) * r);
        for i in [a, bb, c] {
            for k in 0..3 {
                tan1[i as usize][k] += sdir[k];
                tan2[i as usize][k] += tdir[k];
            }
        }
    }
    let mut tangents = vec![0f32; count * 4];
    for &i in &b.index {
        let i = i as usize;
        let n = [
            b.normals[i * 3] as f64,
            b.normals[i * 3 + 1] as f64,
            b.normals[i * 3 + 2] as f64,
        ];
        let t = tan1[i];
        let d = n[0] * t[0] + n[1] * t[1] + n[2] * t[2];
        let tmp = math::normalize([t[0] - n[0] * d, t[1] - n[1] * d, t[2] - n[2] * d]);
        let cross = [
            n[1] * t[2] - n[2] * t[1],
            n[2] * t[0] - n[0] * t[2],
            n[0] * t[1] - n[1] * t[0],
        ];
        let test = cross[0] * tan2[i][0] + cross[1] * tan2[i][1] + cross[2] * tan2[i][2];
        let w = if test < 0.0 { -1.0 } else { 1.0 };
        tangents[i * 4..i * 4 + 4].copy_from_slice(&[
            tmp[0] as f32,
            tmp[1] as f32,
            tmp[2] as f32,
            w,
        ]);
    }
    tangents
}

/// StaticGeometryGenerator.generate over the meshes in merge order, then
/// updateMaterialIndexAttribute ( one material per mesh ).
pub(crate) fn merge(meshes: &[Mesh]) -> Merged {
    let mut out = Merged {
        positions: vec![],
        normals: vec![],
        tangents: vec![],
        uvs: vec![],
        colors: vec![],
        index: vec![],
        material_index: vec![],
    };
    for (i, m) in meshes.iter().enumerate() {
        let b = bake(m);
        let offset = (out.positions.len() / 3) as u32;
        out.index.extend(b.index.iter().map(|&k| k + offset));
        let start = out.material_index.len();
        out.material_index.resize(start + b.positions.len() / 3, 0);
        for &k in &b.index {
            out.material_index[(k + offset) as usize] = i as u8;
        }
        out.positions.extend(b.positions);
        out.normals.extend(b.normals);
        out.tangents.extend(b.tangents);
        out.uvs.extend(b.uvs);
        out.colors.extend(b.colors);
    }
    out
}
