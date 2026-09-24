//! The noise terrain (first-person and raycast), and the GCode, VOX and OBJ/MTL
//! loaders from the pinned WebGL examples.
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::trackball_sprites::{FirstPerson, noise};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    raycast::Raycaster, renderer::*, scene::*,
};
use std::collections::HashMap;
use std::f64::consts::{PI, TAU};
use std::sync::Arc;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
/// `Uint8ClampedArray` element conversion: NaN is 0, round half to even, clamp.
fn clamped(v: f64) -> u8 {
    if v.is_nan() || v <= 0. {
        return 0;
    }
    if v >= 255. {
        return 255;
    }
    let f = v.floor();
    // Ties go to the even neighbor.
    let r = if f + 0.5 < v || (v == f + 0.5 && f % 2. != 0.) {
        f + 1.
    } else {
        f
    };
    r as u8
}
/// `generateHeight` of the terrain examples: four ImprovedNoise octaves in a Uint8Array.
fn terrain_height(rand: &mut dyn FnMut() -> f64) -> Vec<u8> {
    let (width, size) = (256, 256 * 256);
    let mut data = vec![0u8; size];
    let z = rand() * 100.;
    let mut quality = 1.;
    for _ in 0..4 {
        for (i, d) in data.iter_mut().enumerate() {
            let (x, y) = ((i % width) as f64, (i / width) as f64);
            // data[ i ] += v stores ToUint8: truncation modulo 256.
            let v = *d as f64 + (noise(x / quality, y / quality, z) * quality * 1.75).abs();
            *d = (v.trunc() as i64).rem_euclid(256) as u8;
        }
        quality *= 5.;
    }
    data
}
/// `generateTexture`: baked slope shading at 256², a 4× Canvas2D upscale, then noise.
fn terrain_texture(data: &[u8], rand: &mut dyn FnMut() -> f64) -> Result<Texture> {
    use wasm_bindgen::{Clamped, JsCast};
    let fail = |e: wasm_bindgen::JsValue| Error::Asset(format!("terrain canvas: {e:?}"));
    let (width, size) = (256i64, 256usize * 256);
    let get = |k: i64| {
        if k < 0 || k as usize >= size {
            f64::NAN
        } else {
            data[k as usize] as f64
        }
    };
    let sun = 1. / 3f64.sqrt();
    let mut image = vec![0u8; size * 4];
    for j in 0..size {
        let k = j as i64;
        let (mut x, mut y, mut z) = (
            get(k - 2) - get(k + 2),
            2.,
            get(k - width * 2) - get(k + width * 2),
        );
        // Vector3.normalize(): divideScalar( length() || 1 ).
        let length = (x * x + y * y + z * z).sqrt();
        let length = if length.is_nan() || length == 0. {
            1.
        } else {
            length
        };
        x /= length;
        y /= length;
        z /= length;
        let shade = x * sun + y * sun + z * sun;
        let h = 0.5 + data[j] as f64 * 0.007;
        image[j * 4] = clamped((96. + shade * 128.) * h);
        image[j * 4 + 1] = clamped((32. + shade * 96.) * h);
        image[j * 4 + 2] = clamped((shade * 96.) * h);
        image[j * 4 + 3] = 255;
    }
    let canvas = web_sys::OffscreenCanvas::new(256, 256).map_err(fail)?;
    let context: web_sys::OffscreenCanvasRenderingContext2d = canvas
        .get_context("2d")
        .map_err(fail)?
        .ok_or(Error::Invalid("terrain context"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("terrain context"))?;
    let pixels = web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&image), 256, 256)
        .map_err(fail)?;
    context.put_image_data(&pixels, 0., 0.).map_err(fail)?;
    let scaled = web_sys::OffscreenCanvas::new(1024, 1024).map_err(fail)?;
    let context: web_sys::OffscreenCanvasRenderingContext2d = scaled
        .get_context("2d")
        .map_err(fail)?
        .ok_or(Error::Invalid("terrain context"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("terrain context"))?;
    context.scale(4., 4.).map_err(fail)?;
    context
        .draw_image_with_offscreen_canvas(&canvas, 0., 0.)
        .map_err(fail)?;
    let mut rgba = context
        .get_image_data(0., 0., 1024., 1024.)
        .map_err(fail)?
        .data()
        .0;
    for px in rgba.chunks_mut(4) {
        let v = (rand() * 5.).trunc();
        for c in &mut px[..3] {
            *c = clamped(*c as f64 + v);
        }
    }
    // CanvasTexture: sRGB, clamped, mipmapped, flipped on upload.
    let mut t = Texture::from_rgba(1024, 1024, rgba, true)?;
    t.mipmap_filter = Some(Filter::Linear);
    Ok(t)
}
/// GCodeLoader.parse: extruded and travel segments, rotated into Y-up.
fn parse_gcode(text: &str) -> (Vec<f32>, Vec<f32>) {
    #[derive(Clone, Copy, Default)]
    struct State {
        x: f64,
        y: f64,
        z: f64,
        e: f64,
        f: f64,
        relative: bool,
        override_: bool,
        extrusion_relative: bool,
    }
    let mut state = State::default();
    let (mut extruded, mut path) = (vec![], vec![]);
    // data.replace( /;.+/g, '' ): a semicolon and at least one following character.
    let mut cleaned = String::with_capacity(text.len());
    for line in text.split('\n') {
        // '.' stops at a line terminator, so a trailing carriage return survives.
        let (body, cr) = match line.strip_suffix('\r') {
            Some(body) => (body, "\r"),
            None => (line, ""),
        };
        match body.find(';') {
            Some(i) if i + 1 < body.len() => cleaned.push_str(&body[..i]),
            _ => cleaned.push_str(body),
        }
        cleaned.push_str(cr);
        cleaned.push('\n');
    }
    cleaned.pop();
    // parseFloat: the longest numeric prefix, NaN when there is none.
    let parse_float = |s: &str| -> f64 {
        let s = s.trim_start();
        let mut end = 0;
        let bytes = s.as_bytes();
        let mut best = f64::NAN;
        while end < bytes.len() {
            end += 1;
            if let Ok(v) = s[..end].parse::<f64>() {
                best = v;
            } else if !matches!(bytes[end - 1], b'e' | b'E' | b'+' | b'-' | b'.') {
                break;
            }
        }
        best
    };
    for line in cleaned.split('\n') {
        let mut tokens = line.split(' ');
        let cmd = tokens.next().unwrap_or("").to_uppercase();
        let mut args: HashMap<char, f64> = HashMap::new();
        for token in tokens {
            if let Some(first) = token.chars().next() {
                let key = first.to_lowercase().next().unwrap_or(first);
                args.insert(key, parse_float(&token[first.len_utf8()..]));
            }
        }
        let absolute = |v1: f64, v2: f64, relative: bool| if relative { v1 + v2 } else { v2 };
        match cmd.as_str() {
            "G0" | "G1" => {
                let mut line = state;
                if let Some(&v) = args.get(&'x') {
                    line.x = absolute(state.x, v, state.relative);
                }
                if let Some(&v) = args.get(&'y') {
                    line.y = absolute(state.y, v, state.relative);
                }
                if let Some(&v) = args.get(&'z') {
                    line.z = absolute(state.z, v, state.relative);
                }
                if let Some(&v) = args.get(&'e') {
                    let relative = if state.override_ {
                        state.extrusion_relative
                    } else {
                        state.relative
                    };
                    line.e = absolute(state.e, v, relative);
                }
                if let Some(&v) = args.get(&'f') {
                    line.f = absolute(state.f, v, state.relative);
                }
                let delta = if state.relative {
                    line.e
                } else {
                    line.e - state.e
                };
                // The segment is extruded when it adds filament: the flag is set on the old
                // state after the new one was cloned, so it is not carried forward. The
                // unsplit layers concatenate in chronological order.
                let extruding = delta > 0.;
                let target = if extruding { &mut extruded } else { &mut path };
                target
                    .extend([state.x, state.y, state.z, line.x, line.y, line.z].map(|v| v as f32));
                state = line;
            }
            "G90" => {
                state.relative = false;
                state.override_ = false;
            }
            "G91" => {
                state.relative = true;
                state.override_ = false;
            }
            "M82" => {
                state.override_ = true;
                state.extrusion_relative = false;
            }
            "M83" => {
                state.override_ = true;
                state.extrusion_relative = true;
            }
            "G92" => {
                for (k, v) in [
                    ('x', &mut state.x),
                    ('y', &mut state.y),
                    ('z', &mut state.z),
                    ('e', &mut state.e),
                ] {
                    if let Some(&a) = args.get(&k) {
                        *v = a;
                    }
                }
            }
            _ => {}
        }
    }
    (extruded, path)
}
const GCODE_ASSETS: [&str; 3] = ["benchy", "test_m82", "test_m83"];
/// VOXLoader: one chunk's greedy-meshed faces with sRGB palette colors.
struct VoxChunk {
    size: [usize; 3],
    voxels: Vec<[u8; 4]>,
}
/// A VOX scene-graph node as buildObject walks it.
enum VoxNode {
    Transform {
        child: u32,
        translation: Option<[f64; 3]>,
        rotation: Option<u8>,
    },
    Group(Vec<u32>),
    Shape(Vec<u32>),
}
/// Parsed VOX chunks, the palette and the scene-graph nodes.
type Vox = (Vec<VoxChunk>, Vec<u32>, HashMap<u32, VoxNode>);
fn parse_vox(data: &[u8]) -> Result<Vox> {
    let bad = |m: &'static str| Error::Asset(format!("VOX: {m}"));
    let u32_at = |i: usize| -> Result<u32> {
        Ok(u32::from_le_bytes(
            data.get(i..i + 4)
                .ok_or(bad("truncated"))?
                .try_into()
                .map_err(|_| bad("u32"))?,
        ))
    };
    if u32_at(0)? != 542658390 || !matches!(u32_at(4)?, 150 | 200) {
        return Err(bad("header"));
    }
    let read_string = |i: usize| -> Result<(String, usize)> {
        let n = u32_at(i)? as usize;
        let s = data.get(i + 4..i + 4 + n).ok_or(bad("string"))?;
        Ok((String::from_utf8_lossy(s).into_owned(), 4 + n))
    };
    let read_dict = |i: usize| -> Result<(HashMap<String, String>, usize)> {
        let count = u32_at(i)?;
        let (mut dict, mut o) = (HashMap::new(), i + 4);
        for _ in 0..count {
            let (k, a) = read_string(o)?;
            o += a;
            let (v, b) = read_string(o)?;
            o += b;
            dict.insert(k, v);
        }
        Ok((dict, o - i))
    };
    let mut chunks: Vec<VoxChunk> = vec![];
    let mut palette = DEFAULT_PALETTE.to_vec();
    let mut nodes = HashMap::new();
    let mut i = 8;
    while i < data.len() {
        let id = data.get(i..i + 4).ok_or(bad("chunk id"))?;
        let size = u32_at(i + 4)? as usize;
        i += 12;
        match id {
            b"SIZE" => {
                chunks.push(VoxChunk {
                    size: [
                        u32_at(i)? as usize,
                        u32_at(i + 4)? as usize,
                        u32_at(i + 8)? as usize,
                    ],
                    voxels: vec![],
                });
                i += size;
            }
            b"XYZI" => {
                let n = u32_at(i)? as usize;
                let chunk = chunks.last_mut().ok_or(bad("XYZI before SIZE"))?;
                chunk.voxels = data
                    .get(i + 4..i + 4 + n * 4)
                    .ok_or(bad("voxels"))?
                    .chunks(4)
                    .map(|v| [v[0], v[1], v[2], v[3]])
                    .collect();
                i += 4 + n * 4;
            }
            b"RGBA" => {
                palette = vec![0];
                for j in 0..256 {
                    palette.push(u32_at(i + j * 4)?);
                }
                i += 1024;
            }
            b"nTRN" => {
                let node = u32_at(i)?;
                let (_, a) = read_dict(i + 4)?;
                let mut o = i + 4 + a;
                let child = u32_at(o)?;
                o += 12;
                let frames = u32_at(o)?;
                o += 4;
                let (mut translation, mut rotation) = (None, None);
                for f in 0..frames {
                    let (frame, s) = read_dict(o)?;
                    o += s;
                    if f == 0 {
                        rotation = frame.get("_r").and_then(|r| r.trim().parse::<u8>().ok());
                        translation = frame.get("_t").map(|t| {
                            let p: Vec<f64> = t
                                .split(' ')
                                .map(|v| v.parse().unwrap_or(f64::NAN))
                                .collect();
                            [
                                p[0],
                                p.get(1).copied().unwrap_or(0.),
                                p.get(2).copied().unwrap_or(0.),
                            ]
                        });
                    }
                }
                nodes.insert(
                    node,
                    VoxNode::Transform {
                        child,
                        translation,
                        rotation,
                    },
                );
                i = o;
            }
            b"nGRP" => {
                let node = u32_at(i)?;
                let (_, a) = read_dict(i + 4)?;
                let mut o = i + 4 + a;
                let n = u32_at(o)?;
                o += 4;
                let mut children = vec![];
                for _ in 0..n {
                    children.push(u32_at(o)?);
                    o += 4;
                }
                nodes.insert(node, VoxNode::Group(children));
                i = o;
            }
            b"nSHP" => {
                let node = u32_at(i)?;
                let (_, a) = read_dict(i + 4)?;
                let mut o = i + 4 + a;
                let n = u32_at(o)?;
                o += 4;
                let mut models = vec![];
                for _ in 0..n {
                    models.push(u32_at(o)?);
                    let (_, s) = read_dict(o + 4)?;
                    o += 4 + s;
                }
                nodes.insert(node, VoxNode::Shape(models));
                i = o;
            }
            _ => i += size,
        }
    }
    Ok((chunks, palette, nodes))
}
/// buildMesh: per-axis slice masks merged greedily into quads, with computeVertexNormals.
fn vox_mesh(chunk: &VoxChunk, palette: &[u32]) -> Result<BufferGeometry> {
    let [sx, sy, sz] = chunk.size;
    let mut volume = vec![0u8; sx * sy * sz];
    for v in &chunk.voxels {
        let index = v[0] as usize + v[1] as usize * sx + v[2] as usize * sx * sy;
        if let Some(cell) = volume.get_mut(index) {
            *cell = v[3];
        }
    }
    let (mut vertices, mut indices, mut colors) = (vec![], vec![], vec![]);
    let mut has_colors = false;
    let dims = [sx, sy, sz];
    let linear = |c: f64| {
        if c < 0.04045 {
            c * 0.0773993808
        } else {
            (c * 0.9478672986 + 0.0521327014).powf(2.4)
        }
    };
    for d in 0..3 {
        let (u, v) = ((d + 1) % 3, (d + 2) % 3);
        let (dd, du, dv) = (dims[d], dims[u], dims[v]);
        let mut q = [0usize; 3];
        q[d] = 1;
        let mut mask = vec![0i16; du * dv];
        for slice in 0..=dd {
            let mut n = 0;
            for vv in 0..dv {
                for uu in 0..du {
                    let mut pos = [0usize; 3];
                    pos[d] = slice;
                    pos[u] = uu;
                    pos[v] = vv;
                    let behind = if slice > 0 {
                        volume[(pos[0] - q[0]) + (pos[1] - q[1]) * sx + (pos[2] - q[2]) * sx * sy]
                    } else {
                        0
                    };
                    let infront = if slice < dd {
                        volume[pos[0] + pos[1] * sx + pos[2] * sx * sy]
                    } else {
                        0
                    };
                    mask[n] = if behind > 0 && infront == 0 {
                        behind as i16
                    } else if infront > 0 && behind == 0 {
                        -(infront as i16)
                    } else {
                        0
                    };
                    n += 1;
                }
            }
            n = 0;
            for vv in 0..dv {
                let mut uu = 0;
                while uu < du {
                    let c = mask[n];
                    if c == 0 {
                        uu += 1;
                        n += 1;
                        continue;
                    }
                    let mut w = 1;
                    while uu + w < du && mask[n + w] == c {
                        w += 1;
                    }
                    let mut h = 1;
                    'grow: while vv + h < dv {
                        for k in 0..w {
                            if mask[n + k + h * du] != c {
                                break 'grow;
                            }
                        }
                        h += 1;
                    }
                    let mut pos = [0f64; 3];
                    pos[d] = slice as f64;
                    pos[u] = uu as f64;
                    pos[v] = vv as f64;
                    let (mut ou, mut ov) = ([0f64; 3], [0f64; 3]);
                    ou[u] = w as f64;
                    ov[v] = h as f64;
                    let hex = palette.get(c.unsigned_abs() as usize).copied().unwrap_or(0);
                    let rgb = [hex & 0xff, (hex >> 8) & 0xff, (hex >> 16) & 0xff]
                        .map(|b| b as f64 / 255.);
                    if rgb.iter().any(|&x| x > 0.) {
                        has_colors = true;
                    }
                    let color = rgb.map(|x| linear(x) as f32);
                    let to_three = |p: [f64; 3]| {
                        [
                            (p[0] - sx as f64 / 2.) as f32,
                            (p[2] - sz as f64 / 2.) as f32,
                            (-p[1] + sy as f64 / 2.) as f32,
                        ]
                    };
                    let add = |a: [f64; 3], b: [f64; 3]| [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
                    let v0 = to_three(pos);
                    let v1 = to_three(add(pos, ou));
                    let v2 = to_three(add(add(pos, ou), ov));
                    let v3 = to_three(add(pos, ov));
                    let idx = (vertices.len() / 3) as u32;
                    let quad = if c > 0 {
                        [v0, v1, v2, v3]
                    } else {
                        [v0, v3, v2, v1]
                    };
                    for p in quad {
                        vertices.extend(p);
                        colors.extend(color);
                    }
                    indices.extend([idx, idx + 1, idx + 2, idx, idx + 2, idx + 3]);
                    for hh in 0..h {
                        for ww in 0..w {
                            mask[n + ww + hh * du] = 0;
                        }
                    }
                    uu += w;
                    n += w;
                }
            }
        }
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(vertices)?);
    g.set_index(Some(indices));
    g.compute_vertex_normals()?;
    if has_colors {
        g.set_attribute("color", vec3s(colors)?);
    }
    Ok(g)
}
/// A parsed OBJ object: non-indexed triangles and its material groups.
pub(super) struct ObjObject {
    pub(super) positions: Vec<f32>,
    pub(super) normals: Vec<f32>,
    pub(super) uvs: Vec<f32>,
    pub(super) groups: Vec<(String, usize, usize)>,
}
/// OBJLoader's parser state for faces, objects and `usemtl` groups (MTL materials given).
pub(super) fn parse_obj(text: &str) -> Vec<ObjObject> {
    struct Mat {
        name: String,
        start: usize,
        end: Option<usize>,
        count: i64,
        inherited: bool,
    }
    struct Obj {
        from_declaration: bool,
        positions: Vec<f32>,
        normals: Vec<f32>,
        uvs: Vec<f32>,
        has_uv: bool,
        materials: Vec<Mat>,
    }
    impl Obj {
        fn current(&mut self) -> Option<&mut Mat> {
            self.materials.last_mut()
        }
        /// _finalize( end ): close the last group, then prune empty groups.
        fn finalize(&mut self, end: bool) -> Option<usize> {
            let len = self.positions.len() / 3;
            let mut last = None;
            if let Some(m) = self.current() {
                if m.end.is_none() {
                    m.end = Some(len);
                    m.count = len as i64 - m.start as i64;
                    m.inherited = false;
                }
                last = Some(self.materials.len() - 1);
            }
            if end && self.materials.len() > 1 {
                self.materials.retain(|m| m.count > 0);
            }
            if end && self.materials.is_empty() {
                self.materials.push(Mat {
                    name: String::new(),
                    start: 0,
                    end: Some(0),
                    count: -1,
                    inherited: false,
                });
            }
            last
        }
    }
    let new_object = |from_declaration: bool| Obj {
        from_declaration,
        positions: vec![],
        normals: vec![],
        uvs: vec![],
        has_uv: false,
        materials: vec![],
    };
    let mut objects: Vec<Obj> = vec![new_object(false)];
    let (mut vertices, mut normals, mut uvs): (Vec<f64>, Vec<f64>, Vec<f64>) =
        (vec![], vec![], vec![]);
    let text = text.replace("\r\n", "\n").replace("\\\n", "");
    let pf = |s: Option<&str>| s.and_then(|v| v.parse::<f64>().ok()).unwrap_or(f64::NAN);
    let index = |value: &str, len: usize, size: usize| -> usize {
        let i: i64 = value.parse().unwrap_or(0);
        let i = if i >= 0 {
            i - 1
        } else {
            i + (len / size) as i64
        };
        i.max(0) as usize * size
    };
    for line in text.split('\n') {
        let line = line.trim_start();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let first = line.as_bytes()[0];
        if first == b'v' {
            let data: Vec<&str> = line.split_whitespace().collect();
            match data[0] {
                "v" => vertices.extend([
                    pf(data.get(1).copied()),
                    pf(data.get(2).copied()),
                    pf(data.get(3).copied()),
                ]),
                "vn" => normals.extend([
                    pf(data.get(1).copied()),
                    pf(data.get(2).copied()),
                    pf(data.get(3).copied()),
                ]),
                "vt" => uvs.extend([pf(data.get(1).copied()), pf(data.get(2).copied())]),
                _ => {}
            }
        } else if first == b'f' {
            let face: Vec<Vec<&str>> = line[1..]
                .split_whitespace()
                .map(|v| v.split('/').collect())
                .collect();
            let object = objects.last_mut().expect("object");
            for j in 1..face.len().saturating_sub(1) {
                let tri = [&face[0], &face[j], &face[j + 1]];
                for t in tri {
                    let a = index(t[0], vertices.len(), 3);
                    object
                        .positions
                        .extend(vertices[a..a + 3].iter().map(|&v| v as f32));
                }
                if tri.iter().all(|t| t.get(2).is_some_and(|s| !s.is_empty())) {
                    for t in tri {
                        let a = index(t[2], normals.len(), 3);
                        object
                            .normals
                            .extend(normals[a..a + 3].iter().map(|&v| v as f32));
                    }
                } else {
                    // addFaceNormal: ( c - b ) × ( a - b ), normalized.
                    let p = |t: &Vec<&str>| {
                        let a = index(t[0], vertices.len(), 3);
                        Vector3::new(vertices[a], vertices[a + 1], vertices[a + 2])
                    };
                    let (a, b, c) = (p(tri[0]), p(tri[1]), p(tri[2]));
                    let n = (c - b).cross(a - b).normalize_or_zero();
                    for _ in 0..3 {
                        object.normals.extend([n.x as f32, n.y as f32, n.z as f32]);
                    }
                }
                if tri.iter().all(|t| t.get(1).is_some_and(|s| !s.is_empty())) {
                    for t in tri {
                        let a = index(t[1], uvs.len(), 2);
                        object.uvs.extend(uvs[a..a + 2].iter().map(|&v| v as f32));
                    }
                    object.has_uv = true;
                } else {
                    object.uvs.extend([0.; 6]);
                }
            }
        } else if first == b'o' || first == b'g' {
            // startObject( name ): the initial undeclared object is only renamed.
            let current = objects.last_mut().expect("object");
            if !current.from_declaration {
                current.from_declaration = true;
                continue;
            }
            let previous = current.current().map(|m| m.name.clone());
            current.finalize(true);
            let mut object = new_object(true);
            if let Some(name) = previous.filter(|n| !n.is_empty()) {
                object.materials.push(Mat {
                    name,
                    start: 0,
                    end: None,
                    count: -1,
                    inherited: true,
                });
            }
            objects.push(object);
        } else if let Some(name) = line.strip_prefix("usemtl ") {
            let object = objects.last_mut().expect("object");
            let previous = object.finalize(false);
            let mut start = 0;
            if let Some(p) = previous {
                let (inherited, count, end) = {
                    let m = &object.materials[p];
                    (m.inherited, m.count, m.end)
                };
                start = end.unwrap_or(0);
                if inherited || count <= 0 {
                    object.materials.remove(p);
                }
            }
            object.materials.push(Mat {
                name: name.trim().to_string(),
                start,
                end: None,
                count: -1,
                inherited: false,
            });
        }
    }
    if let Some(object) = objects.last_mut() {
        object.finalize(true);
    }
    objects
        .into_iter()
        .filter(|o| !o.positions.is_empty())
        .map(|o| ObjObject {
            groups: o
                .materials
                .iter()
                .map(|m| (m.name.clone(), m.start, m.count.max(0) as usize))
                .collect(),
            positions: o.positions,
            normals: o.normals,
            uvs: if o.has_uv { o.uvs } else { vec![] },
        })
        .collect()
}
/// One MTL material: Kd, Ks, Ns and map_Kd.
type MtlEntry = (
    Option<[f64; 3]>,
    Option<[f64; 3]>,
    Option<f64>,
    Option<String>,
);
/// MTLLoader: `newmtl` blocks of Kd, Ks, Ns and map_Kd, as MeshPhongMaterial parameters.
fn parse_mtl(text: &str) -> HashMap<String, MtlEntry> {
    let mut out = HashMap::new();
    let mut current: Option<String> = None;
    for line in text.split('\n') {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = match line.find(' ') {
            Some(i) => (line[..i].to_lowercase(), line[i + 1..].trim().to_string()),
            None => (line.to_lowercase(), String::new()),
        };
        if key == "newmtl" {
            out.insert(value.clone(), (None, None, None, None));
            current = Some(value);
            continue;
        }
        let Some(entry) = current.as_ref().and_then(|c| out.get_mut(c)) else {
            continue;
        };
        let rgb = || {
            let v: Vec<f64> = value
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            (v.len() >= 3).then(|| [v[0], v[1], v[2]])
        };
        match key.as_str() {
            "kd" => entry.0 = rgb(),
            "ks" => entry.1 = rgb(),
            "ns" => entry.2 = value.parse().ok(),
            "map_kd" => entry.3 = Some(value.clone()),
            _ => {}
        }
    }
    out
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    walker: FirstPerson,
    controls: Option<Controls>,
    pointer: Vector2,
    mesh: Option<Object3D>,
    helper: Option<Object3D>,
    terrain: Vec<f32>,
    // GCode
    gcode: Vec<String>,
    models: Vec<Option<Object3D>>,
    asset: usize,
    shown: usize,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far) = match id {
            218 => (60., 1., 10000.),
            219 => (60., 10., 20000.),
            220 => (60., 1., 1000.),
            221 => (50., 0.01, 10.),
            _ => (45., 0.1, 20.),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            walker: FirstPerson::default(),
            controls: None,
            pointer: Vector2::ZERO,
            mesh: None,
            helper: None,
            terrain: vec![],
            gcode: vec![],
            models: vec![None; 3],
            asset: 0,
            shown: usize::MAX,
        };
        match id {
            218 | 219 => d.terrain_scene(s, c, r).await?,
            220 => {
                for asset in GCODE_ASSETS {
                    let bytes = fetch(&format!("/web/gallery/assets/gcode/{asset}.gcode")).await?;
                    d.gcode.push(String::from_utf8_lossy(&bytes).into_owned());
                }
                s.background = Color::BLACK;
                s.get_mut(c)?.position = Vector3::new(0., 0., 70.);
                d.controls = Some(Controls::new(None, (10., 100.), PI, true));
            }
            221 => d.vox_scene(s, c).await?,
            _ => d.obj_scene(s, c, r).await?,
        }
        if let Some(controls) = &mut d.controls {
            // The controls' constructor update() looks at the target.
            controls.update(s, c)?;
        }
        Ok(d)
    }
    async fn terrain_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        // webgl_geometry_terrain seeds its own Math.random with sin( seed++ ); the raycast
        // variant draws from the page's (seeded) Math.random.
        let mut sine_seed = PI / 4.;
        let mut seed = self.seed;
        let raycast = self.id == 219;
        let mut rand = move || {
            if raycast {
                random(&mut seed)
            } else {
                let x = sine_seed.sin() * 10000.;
                sine_seed += 1.;
                x - x.floor()
            }
        };
        let data = terrain_height(&mut rand);
        let mut g = PlaneGeometry::build(7500., 7500., 255, 255)?;
        g.rotate_x(-PI / 2.)?;
        if let Some(Attribute::F32(a)) = g.attributes.get_mut("position") {
            for (i, v) in a.array_mut().chunks_mut(3).enumerate() {
                v[1] = data[i] as f32 * 10.;
            }
            a.mark_dirty();
            self.terrain = a.array().to_vec();
        }
        g.compute_bounding_sphere()?;
        let texture = terrain_texture(&data, &mut rand)?;
        let mut m = MeshBasicMaterial::default();
        m.properties.map = Some(Arc::new(texture));
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Basic(m)),
        )));
        self.mesh = Some(mesh);
        if raycast {
            s.background = Color::from_hex(0xbfd1e5);
            let target_y = data[128 + 128 * 256] as f64 + 500.;
            s.get_mut(c)?.position = Vector3::new(2000., target_y + 2000., 0.);
            let mut controls = Controls::new(None, (1000., 10000.), PI / 2., true);
            controls.set_target(Vector3::new(0., target_y, 0.));
            self.controls = Some(controls);
            // ConeGeometry( 20, 100, 3 ), translated and turned to point along +Z.
            let mut cone = CylinderGeometry::build(0., 20., 100., 3, 1, false, 0., TAU)?;
            cone.translate(Vector3::new(0., 50., 0.))?;
            cone.rotate_x(PI / 2.)?;
            // WebGL's MeshNormalMaterial writes packed normals without the output encoding:
            // decode them so the encoded target stores the original's values.
            let normal = crate::tsl::WgslFn::new(
                "raw_normal",
                "fn raw_normal(k:f32)->vec4<f32>{let c=normalize(fragment_surface.normal)*0.5+0.5+vec3(k);return vec4(select(pow((c+vec3(0.055))/1.055,vec3(2.4)),c/12.92,c<=vec3(0.04045)),1.0);}",
                &[crate::tsl::Type::Float],
                crate::tsl::Type::Vec4,
            )?
            .call(&[crate::tsl::float(0.)]);
            let program = crate::shader::ShaderProgram::with_projection(
                r,
                &crate::tsl::NodeMaterial::new(normal).wgsl(0)?,
                &[],
                &[],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
            )
            .await?;
            self.helper = Some(s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(cone),
                Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
            ))));
        } else {
            s.background = Color::from_hex(0xefd1b5);
            s.fog = Some(Fog::Exp2 {
                color: Color::from_hex(0xefd1b5),
                density: 0.0025,
            });
            s.get_mut(c)?.position = Vector3::new(100., 800., -800.);
            s.look_at(c, Vector3::new(-100., 810., -800.))?;
            self.walker.speed = 150.;
            let look = s.get(c)?.quaternion * -Vector3::Z;
            self.walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
            self.walker.lon = look.x.atan2(look.z).to_degrees();
        }
        self.seed = seed;
        Ok(())
    }
    async fn vox_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.background = Color::BLACK;
        s.get_mut(c)?.position = Vector3::new(0.175, 0.075, 0.175);
        let hemisphere = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0xcccccc),
            ground: Color::from_hex(0x444444),
            intensity: 3.,
        }));
        // HemisphereLight sits at Object3D.DEFAULT_UP.
        s.get_mut(hemisphere)?.position = Vector3::Y;
        for (intensity, p) in [
            (2.5, Vector3::new(1.5, 3., 2.5)),
            (1.5, Vector3::new(-1.5, -3., -2.5)),
        ] {
            let light = s.insert(NodeKind::Light(Light::Directional {
                color: Color::WHITE,
                intensity,
                target: Vector3::ZERO,
            }));
            s.get_mut(light)?.position = p;
        }
        let (chunks, palette, nodes) =
            parse_vox(&fetch("/web/gallery/assets/vox/monu10.vox").await?)?;
        // result.scene.children[ 0 ]: buildObject( 0 ) returns the group of node 1 here,
        // whose first child is the transformed shape mesh.
        let (model, translation) =
            first_vox_mesh(&nodes).ok_or(Error::Asset("VOX scene".into()))?;
        let chunk = chunks
            .get(model as usize)
            .ok_or(Error::Asset("VOX model".into()))?;
        let g = vox_mesh(chunk, &palette)?;
        // r186's WebGL physical shading also conserves energy between the lobes.
        let mut m = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        m.properties.vertex_colors = g.attributes.contains_key("color");
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Standard(m)),
        )));
        let n = s.get_mut(mesh)?;
        if let Some(t) = translation {
            n.position = Vector3::new(t[0], t[2], -t[1]);
        }
        n.position.y = 0.;
        n.scale = Vector3::splat(0.0015);
        self.mesh = Some(mesh);
        self.controls = Some(Controls::new(None, (0.1, 0.5), PI, true));
        Ok(())
    }
    async fn obj_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let _ = r;
        s.background = Color::BLACK;
        s.get_mut(c)?.position = Vector3::new(0., 0., 2.5);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 15.,
            distance: 0.,
            decay: 2.,
        }));
        s.add(c, light)?;
        let base = "/web/gallery/assets/obj/male02";
        let mtl = parse_mtl(&String::from_utf8_lossy(
            &fetch(&format!("{base}/male02.mtl")).await?,
        ));
        let objects = parse_obj(&String::from_utf8_lossy(
            &fetch(&format!("{base}/male02.obj")).await?,
        ));
        // MTLLoader loads each map once per material; the port shares decoded files.
        let mut textures: HashMap<String, Arc<crate::material::Texture>> = HashMap::new();
        let mut materials: HashMap<String, Arc<Material>> = HashMap::new();
        let root = s.insert(NodeKind::Group);
        for object in objects {
            let mut g = BufferGeometry::default();
            g.set_attribute("position", vec3s(object.positions)?);
            g.set_attribute("normal", vec3s(object.normals)?);
            if !object.uvs.is_empty() {
                g.set_attribute(
                    "uv",
                    Attribute::F32(BufferAttribute::new(object.uvs, 2, false)?),
                );
            }
            let mut list = vec![];
            for (name, _, _) in &object.groups {
                if !materials.contains_key(name) {
                    let mut m = MeshPhongMaterial::default();
                    if let Some((kd, ks, ns, map)) = mtl.get(name) {
                        let srgb = |v: [f64; 3]| {
                            Color::linear(
                                crate::math::srgb_to_linear(v[0]),
                                crate::math::srgb_to_linear(v[1]),
                                crate::math::srgb_to_linear(v[2]),
                            )
                        };
                        if let Some(kd) = kd {
                            m.properties.color = srgb(*kd);
                        }
                        if let Some(ks) = ks {
                            m.specular = srgb(*ks);
                        }
                        if let Some(ns) = ns {
                            m.shininess = *ns;
                        }
                        if let Some(file) = map {
                            if !textures.contains_key(file) {
                                let mut t =
                                    decode_texture_image(&fetch(&format!("{base}/{file}")).await?)
                                        .await?;
                                t.srgb = true;
                                t.wrap_s = Wrapping::Repeat;
                                t.wrap_t = Wrapping::Repeat;
                                t.mipmap_filter = Some(Filter::Linear);
                                textures.insert(file.clone(), Arc::new(t));
                            }
                            m.properties.map = textures.get(file).cloned();
                        }
                    }
                    materials.insert(name.clone(), Arc::new(Material::Phong(m)));
                }
                list.push(materials[name].clone());
            }
            if list.len() > 1 {
                for (i, (_, start, count)) in object.groups.iter().enumerate() {
                    g.add_group(*start, *count, i);
                }
            }
            let mesh = s.insert(NodeKind::Mesh(Mesh {
                geometry: Arc::new(g),
                materials: list,
            }));
            s.add(root, mesh)?;
        }
        let n = s.get_mut(root)?;
        n.position.y = -0.95;
        n.scale = Vector3::splat(0.01);
        self.controls = Some(Controls::new(Some(0.05), (2., 5.), PI, true));
        Ok(())
    }
    fn show_gcode(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        if self.shown == self.asset {
            return Ok(());
        }
        if let Some(h) = self.models.get(self.shown).copied().flatten() {
            s.get_mut(h)?.visible = false;
        }
        // Parsed once per asset and kept resident; the original re-parses on each switch.
        let h = match self.models[self.asset] {
            Some(h) => h,
            None => {
                let (extruded, path) = parse_gcode(&self.gcode[self.asset]);
                let group = s.insert(NodeKind::Group);
                for (vertices, color) in [(extruded, 0x00ff00), (path, 0xff0000)] {
                    let mut g = BufferGeometry::default();
                    g.set_attribute("position", vec3s(vertices)?);
                    let mut m = LineBasicMaterial::default();
                    m.properties.color = Color::from_hex(color);
                    let line = s.insert(NodeKind::Line(Line {
                        geometry: Arc::new(g),
                        material: Arc::new(Material::Line(m)),
                        segments: true,
                    }));
                    s.add(group, line)?;
                }
                let n = s.get_mut(group)?;
                n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
                n.position = [
                    Vector3::new(-100., -20., 100.),
                    Vector3::ZERO,
                    Vector3::ZERO,
                ][self.asset];
                self.models[self.asset] = Some(group);
                group
            }
        };
        s.get_mut(h)?.visible = true;
        self.shown = self.asset;
        // controls.reset(): the saved camera state, then update().
        s.get_mut(c)?.position = Vector3::new(0., 0., 70.);
        let mut controls = Controls::new(None, (10., 100.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
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
        let delta = t - self.last;
        self.last = t;
        match self.id {
            // Zero-length frames do not step FirstPersonControls, as in the other port.
            218 if delta > 0. => self.walker.update(s, c, delta)?,
            220 => self.show_gcode(s, c)?,
            221 | 222 => {
                // animate(): controls.update() every frame.
                if let Some(controls) = &mut self.controls {
                    controls.update(s, c)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    /// `onPointerMove` of the raycast terrain: NDC from the canvas, the first hit's
    /// face normal orients the cone, which then moves to the hit point.
    pub fn pointer_move(&mut self, s: &mut Scene, c: Object3D, x: f64, y: f64) -> Result<()> {
        self.pointer = Vector2::new(x, y);
        let (Some(mesh), Some(helper)) = (self.mesh, self.helper) else {
            return Ok(());
        };
        s.update_world_matrix(c, true, false)?;
        let (camera, world) = {
            let (camera, _) = s.camera(c)?;
            (camera.clone(), s.get(c)?.matrix_world)
        };
        let mut raycaster = Raycaster::default();
        raycaster.set_from_camera(self.pointer, &camera, world)?;
        let hits = raycaster.intersect_object(s, mesh, false)?;
        let Some(hit) = hits.first() else {
            return Ok(());
        };
        let Some(face) = hit.face_index else {
            return Ok(());
        };
        let geometry = s
            .get(mesh)?
            .geometry()
            .ok_or(Error::Invalid("terrain"))?
            .clone();
        let ids: Vec<usize> = (0..3)
            .map(|k| geometry.vertex_index(face * 3 + k))
            .collect::<Result<_>>()?;
        let p = |i: usize| {
            Vector3::new(
                self.terrain[i * 3] as f64,
                self.terrain[i * 3 + 1] as f64,
                self.terrain[i * 3 + 2] as f64,
            )
        };
        // Triangle.getNormal: ( c - b ) × ( a - b ), normalized.
        let (a, b, cc) = (p(ids[0]), p(ids[1]), p(ids[2]));
        let normal = (cc - b).cross(a - b).normalize_or_zero();
        s.get_mut(helper)?.position = Vector3::ZERO;
        s.look_at(helper, normal)?;
        s.get_mut(helper)?.position = hit.point;
        Ok(())
    }
    /// Absolute CSS-pixel pointer events for FirstPersonControls.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if self.id == 218 {
            self.walker.pointer(kind, x, y);
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
        let pointer = self.pointer;
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, pointer);
        } else if pan {
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        // Each pointer and wheel handler calls update().
        controls.update(s, c)
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if self.id == 218 {
            self.walker.key(code, down);
        }
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (220, 0) if (0. ..=2.).contains(&value) => self.asset = value as usize,
            _ => return Err(Error::Invalid("terrain/loader parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// Walk buildObject( 0 ) to `scene.children[ 0 ]` when it is a single-model shape:
/// the model id and its transform's translation.
fn first_vox_mesh(nodes: &HashMap<u32, VoxNode>) -> Option<(u32, Option<[f64; 3]>)> {
    // Resolve the scene root through translation-free transforms.
    let mut id = 0;
    loop {
        match nodes.get(&id)? {
            VoxNode::Transform {
                child,
                translation: None,
                rotation: None,
            } if !matches!(nodes.get(child)?, VoxNode::Shape(_)) => id = *child,
            _ => break,
        }
    }
    let VoxNode::Group(children) = nodes.get(&id)? else {
        return None;
    };
    match nodes.get(children.first()?)? {
        VoxNode::Transform {
            child,
            translation,
            rotation: None,
        } => match nodes.get(child)? {
            VoxNode::Shape(models) if models.len() == 1 => Some((models[0], *translation)),
            _ => None,
        },
        _ => None,
    }
}
/// VOXLoader's DEFAULT_PALETTE.
const DEFAULT_PALETTE: [u32; 256] = [
    0x00000000, 0xffffffff, 0xffccffff, 0xff99ffff, 0xff66ffff, 0xff33ffff, 0xff00ffff, 0xffffccff,
    0xffccccff, 0xff99ccff, 0xff66ccff, 0xff33ccff, 0xff00ccff, 0xffff99ff, 0xffcc99ff, 0xff9999ff,
    0xff6699ff, 0xff3399ff, 0xff0099ff, 0xffff66ff, 0xffcc66ff, 0xff9966ff, 0xff6666ff, 0xff3366ff,
    0xff0066ff, 0xffff33ff, 0xffcc33ff, 0xff9933ff, 0xff6633ff, 0xff3333ff, 0xff0033ff, 0xffff00ff,
    0xffcc00ff, 0xff9900ff, 0xff6600ff, 0xff3300ff, 0xff0000ff, 0xffffffcc, 0xffccffcc, 0xff99ffcc,
    0xff66ffcc, 0xff33ffcc, 0xff00ffcc, 0xffffcccc, 0xffcccccc, 0xff99cccc, 0xff66cccc, 0xff33cccc,
    0xff00cccc, 0xffff99cc, 0xffcc99cc, 0xff9999cc, 0xff6699cc, 0xff3399cc, 0xff0099cc, 0xffff66cc,
    0xffcc66cc, 0xff9966cc, 0xff6666cc, 0xff3366cc, 0xff0066cc, 0xffff33cc, 0xffcc33cc, 0xff9933cc,
    0xff6633cc, 0xff3333cc, 0xff0033cc, 0xffff00cc, 0xffcc00cc, 0xff9900cc, 0xff6600cc, 0xff3300cc,
    0xff0000cc, 0xffffff99, 0xffccff99, 0xff99ff99, 0xff66ff99, 0xff33ff99, 0xff00ff99, 0xffffcc99,
    0xffcccc99, 0xff99cc99, 0xff66cc99, 0xff33cc99, 0xff00cc99, 0xffff9999, 0xffcc9999, 0xff999999,
    0xff669999, 0xff339999, 0xff009999, 0xffff6699, 0xffcc6699, 0xff996699, 0xff666699, 0xff336699,
    0xff006699, 0xffff3399, 0xffcc3399, 0xff993399, 0xff663399, 0xff333399, 0xff003399, 0xffff0099,
    0xffcc0099, 0xff990099, 0xff660099, 0xff330099, 0xff000099, 0xffffff66, 0xffccff66, 0xff99ff66,
    0xff66ff66, 0xff33ff66, 0xff00ff66, 0xffffcc66, 0xffcccc66, 0xff99cc66, 0xff66cc66, 0xff33cc66,
    0xff00cc66, 0xffff9966, 0xffcc9966, 0xff999966, 0xff669966, 0xff339966, 0xff009966, 0xffff6666,
    0xffcc6666, 0xff996666, 0xff666666, 0xff336666, 0xff006666, 0xffff3366, 0xffcc3366, 0xff993366,
    0xff663366, 0xff333366, 0xff003366, 0xffff0066, 0xffcc0066, 0xff990066, 0xff660066, 0xff330066,
    0xff000066, 0xffffff33, 0xffccff33, 0xff99ff33, 0xff66ff33, 0xff33ff33, 0xff00ff33, 0xffffcc33,
    0xffcccc33, 0xff99cc33, 0xff66cc33, 0xff33cc33, 0xff00cc33, 0xffff9933, 0xffcc9933, 0xff999933,
    0xff669933, 0xff339933, 0xff009933, 0xffff6633, 0xffcc6633, 0xff996633, 0xff666633, 0xff336633,
    0xff006633, 0xffff3333, 0xffcc3333, 0xff993333, 0xff663333, 0xff333333, 0xff003333, 0xffff0033,
    0xffcc0033, 0xff990033, 0xff660033, 0xff330033, 0xff000033, 0xffffff00, 0xffccff00, 0xff99ff00,
    0xff66ff00, 0xff33ff00, 0xff00ff00, 0xffffcc00, 0xffcccc00, 0xff99cc00, 0xff66cc00, 0xff33cc00,
    0xff00cc00, 0xffff9900, 0xffcc9900, 0xff999900, 0xff669900, 0xff339900, 0xff009900, 0xffff6600,
    0xffcc6600, 0xff996600, 0xff666600, 0xff336600, 0xff006600, 0xffff3300, 0xffcc3300, 0xff993300,
    0xff663300, 0xff333300, 0xff003300, 0xffff0000, 0xffcc0000, 0xff990000, 0xff660000, 0xff330000,
    0xff0000ee, 0xff0000dd, 0xff0000bb, 0xff0000aa, 0xff000088, 0xff000077, 0xff000055, 0xff000044,
    0xff000022, 0xff000011, 0xff00ee00, 0xff00dd00, 0xff00bb00, 0xff00aa00, 0xff008800, 0xff007700,
    0xff005500, 0xff004400, 0xff002200, 0xff001100, 0xffee0000, 0xffdd0000, 0xffbb0000, 0xffaa0000,
    0xff880000, 0xff770000, 0xff550000, 0xff440000, 0xff220000, 0xff110000, 0xffeeeeee, 0xffdddddd,
    0xffbbbbbb, 0xffaaaaaa, 0xff888888, 0xff777777, 0xff555555, 0xff444444, 0xff222222, 0xff111111,
];
