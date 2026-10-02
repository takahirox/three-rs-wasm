//! webgl_loader_texture_lottie: the LottieFiles logo animation drawn each frame
//! on a canvas, as lottie-web 5.13's canvas renderer draws it, and uploaded as
//! the map of a rounded box ( MeshStandardMaterial, roughness 0 ) lit by the
//! RoomEnvironment under ACES Filmic tone mapping, with OrbitControls turning
//! it ( autoRotate ). The player covers what the file uses, following
//! lottie-web: shape layers of a transformed group with a static path, a
//! stroke or fill and a trim path ( the 150-sample bezier length tables in
//! float32, three-decimal segment points, BezierEasing keyframes ), the
//! layer and group matrices, and the alpha-inverted track matte composited
//! through two buffer canvases. The canvas is 500 × 500 at the square of the
//! device pixel ratio, as the page sizes the container at the ratio and the
//! renderer applies it again. The page's frame scrubber sets the frame and
//! pauses; play resumes.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use crate::tsl::surface::SurfaceNodes;
use crate::tsl::{Texture as Tex, base_color, uv};
use crate::{Error, Result, camera::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

const ASSET: &str = "/web/gallery/assets/lottie/24017-lottie-logo-animation.json";

type Json = serde_json::Value;
fn numbers(v: &Json) -> Vec<f64> {
    match v {
        Json::Array(a) => a.iter().filter_map(Json::as_f64).collect(),
        Json::Number(n) => n.as_f64().into_iter().collect(),
        _ => vec![],
    }
}

/// BezierEasing ( Gaëtan Renaudeau ), as lottie-web's BezierFactory runs it:
/// eleven float32 samples, four Newton iterations or a binary subdivision.
struct Easing {
    p: [f64; 4],
    samples: [f32; 11],
}
impl Easing {
    fn new(p: [f64; 4]) -> Self {
        let mut samples = [0f32; 11];
        if p[0] != p[1] || p[2] != p[3] {
            for (i, s) in samples.iter_mut().enumerate() {
                *s = Self::calc(i as f64 * 0.1, p[0], p[2]) as f32;
            }
        }
        Self { p, samples }
    }
    fn calc(t: f64, a1: f64, a2: f64) -> f64 {
        (((1. - 3. * a2 + 3. * a1) * t + (3. * a2 - 6. * a1)) * t + 3. * a1) * t
    }
    fn slope(t: f64, a1: f64, a2: f64) -> f64 {
        3. * (1. - 3. * a2 + 3. * a1) * t * t + 2. * (3. * a2 - 6. * a1) * t + 3. * a1
    }
    fn get(&self, x: f64) -> f64 {
        let [x1, y1, x2, y2] = self.p;
        if x1 == y1 && x2 == y2 {
            return x;
        }
        if x == 0. {
            return 0.;
        }
        if x == 1. {
            return 1.;
        }
        Self::calc(self.t_for_x(x), y1, y2)
    }
    fn t_for_x(&self, x: f64) -> f64 {
        let (x1, x2) = (self.p[0], self.p[2]);
        let step = 0.1;
        let mut start = 0.;
        let mut current = 1;
        while current != 10 && f64::from(self.samples[current]) <= x {
            start += step;
            current += 1;
        }
        current -= 1;
        let (a, b) = (
            f64::from(self.samples[current]),
            f64::from(self.samples[current + 1]),
        );
        let mut t = start + (x - a) / (b - a) * step;
        let slope = Self::slope(t, x1, x2);
        if slope >= 0.001 {
            for _ in 0..4 {
                let slope = Self::slope(t, x1, x2);
                if slope == 0. {
                    return t;
                }
                t -= (Self::calc(t, x1, x2) - x) / slope;
            }
            t
        } else if slope == 0. {
            t
        } else {
            let (mut lo, mut hi) = (start, start + step);
            let mut i = 0;
            loop {
                t = lo + (hi - lo) / 2.;
                let current = Self::calc(t, x1, x2) - x;
                if current > 0. {
                    hi = t;
                } else {
                    lo = t;
                }
                i += 1;
                if current.abs() <= 0.0000001 || i >= 10 {
                    return t;
                }
            }
        }
    }
}

struct Key {
    t: f64,
    s: Vec<f64>,
    hold: bool,
    /// Per dimension ( or one shared ) easing, when the keyframe eases.
    easing: Vec<Easing>,
}
/// A property: its keyframes or static value, times lottie's multiplier.
struct Prop {
    keys: Vec<Key>,
    value: Vec<f64>,
    mult: f64,
}
impl Prop {
    fn new(v: &Json, mult: f64) -> Result<Self> {
        if v["a"].as_u64() != Some(1) {
            return Ok(Self {
                keys: vec![],
                value: numbers(&v["k"]),
                mult,
            });
        }
        let keys = v["k"]
            .as_array()
            .ok_or(Error::Invalid("lottie keyframes"))?;
        let mut out = vec![];
        for k in keys {
            if numbers(&k["to"])
                .iter()
                .chain(numbers(&k["ti"]).iter())
                .any(|&x| x != 0.)
            {
                return Err(Error::Invalid("lottie curved spatial keyframe"));
            }
            let s = numbers(&k["s"]);
            let easing = if k["o"].is_object() {
                let o = (numbers(&k["o"]["x"]), numbers(&k["o"]["y"]));
                let i = (numbers(&k["i"]["x"]), numbers(&k["i"]["y"]));
                let at = |v: &Vec<f64>, d: usize| v.get(d).or(v.first()).copied().unwrap_or(0.);
                (0..o.0.len().max(1))
                    .map(|d| Easing::new([at(&o.0, d), at(&o.1, d), at(&i.0, d), at(&i.1, d)]))
                    .collect()
            } else {
                vec![]
            };
            out.push(Key {
                t: k["t"].as_f64().unwrap_or(0.),
                s,
                hold: k["h"].as_u64() == Some(1),
                easing,
            });
        }
        Ok(Self {
            keys: out,
            value: vec![],
            mult,
        })
    }
    /// interpolateValue: the keyframe pair around the frame, eased per dimension.
    /// Straight spatial keyframes ( zero tangents ) move along the line by the same
    /// eased fraction, as the bezier length table interpolates them.
    fn get(&self, frame: f64) -> Vec<f64> {
        if self.keys.is_empty() {
            return self.value.iter().map(|v| v * self.mult).collect();
        }
        let len = self.keys.len() - 1;
        if len == 0 {
            return self.keys[0].s.iter().map(|v| v * self.mult).collect();
        }
        let mut i = 0;
        let (key, next) = loop {
            let (key, next) = (&self.keys[i], &self.keys[i + 1]);
            if i == len - 1 && frame >= next.t {
                break (if key.hold { next } else { key }, next);
            }
            if next.t > frame || i >= len - 1 {
                break (key, next);
            }
            i += 1;
        };
        let end = &next.s;
        (0..key.s.len())
            .map(|d| {
                let value = if key.hold {
                    key.s[d]
                } else {
                    let perc = if frame >= next.t {
                        1.
                    } else if frame < key.t {
                        0.
                    } else {
                        key.easing
                            .get(d)
                            .or(key.easing.first())
                            .map_or(0., |e| e.get((frame - key.t) / (next.t - key.t)))
                    };
                    key.s[d] + (end.get(d).copied().unwrap_or(key.s[d]) - key.s[d]) * perc
                };
                value * self.mult
            })
            .collect()
    }
}

/// lottie's Matrix, as ( a, b, c, d, e, f ): x' = a x + c y + e, y' = b x + d y + f.
#[derive(Clone, Copy)]
struct Affine([f64; 6]);
impl Affine {
    /// this × m: this transform first, then m.
    fn then(self, m: Self) -> Self {
        let [a, b, c, d, e, f] = self.0;
        let [a2, b2, c2, d2, e2, f2] = m.0;
        Self([
            a * a2 + b * c2,
            a * b2 + b * d2,
            c * a2 + d * c2,
            c * b2 + d * d2,
            e * a2 + f * c2 + e2,
            e * b2 + f * d2 + f2,
        ])
    }
    fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        let [a, b, c, d, e, f] = self.0;
        (a * x + c * y + e, b * x + d * y + f)
    }
}
/// TransformProperty: translate( −anchor ), scale, rotate, translate( position ).
struct Transform {
    p: Prop,
    a: Prop,
    s: Prop,
    r: Prop,
    o: Prop,
}
impl Transform {
    fn new(v: &Json) -> Result<Self> {
        if numbers(&v["sk"]["k"]).iter().any(|&x| x != 0.) {
            return Err(Error::Invalid("lottie skew"));
        }
        Ok(Self {
            p: Prop::new(&v["p"], 1.)?,
            a: Prop::new(&v["a"], 1.)?,
            s: Prop::new(&v["s"], 0.01)?,
            r: Prop::new(&v["r"], PI / 180.)?,
            o: Prop::new(&v["o"], 0.01)?,
        })
    }
    fn matrix(&self, frame: f64) -> Affine {
        let (a, s, r, p) = (
            self.a.get(frame),
            self.s.get(frame),
            self.r.get(frame),
            self.p.get(frame),
        );
        let at = |v: &Vec<f64>, i: usize, d: f64| v.get(i).copied().unwrap_or(d);
        let mut m = Affine([1., 0., 0., 1., -at(&a, 0, 0.), -at(&a, 1, 0.)]);
        m = m.then(Affine([at(&s, 0, 1.), 0., 0., at(&s, 1, 1.), 0., 0.]));
        // rotate( −r ): props ( cos, −sin, sin, cos ) of −r.
        let (sin, cos) = (-at(&r, 0, 0.)).sin_cos();
        m = m.then(Affine([cos, -sin, sin, cos, 0., 0.]));
        m.then(Affine([1., 0., 0., 1., at(&p, 0, 0.), at(&p, 1, 0.)]))
    }
}

/// A path with absolute in and out tangents, stored in float32 as lottie's
/// shape pool stores its points.
#[derive(Clone, Default)]
struct Path {
    v: Vec<[f32; 2]>,
    i: Vec<[f32; 2]>,
    o: Vec<[f32; 2]>,
    closed: bool,
}
/// bezierLengthPool's float32 tables and the double added length.
struct Lengths {
    added: f64,
    percents: Vec<f32>,
    lengths: Vec<f32>,
}
const CURVE_SEGMENTS: usize = 150;
fn bezier_length(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2]) -> Lengths {
    let mut added = 0.;
    let mut last: Option<[f64; 2]> = None;
    let mut percents = vec![0f32; CURVE_SEGMENTS];
    let mut lengths = vec![0f32; CURVE_SEGMENTS];
    for k in 0..CURVE_SEGMENTS {
        let perc = k as f64 / (CURVE_SEGMENTS - 1) as f64;
        let mut point = [0.; 2];
        let mut distance = 0.;
        for c in 0..2 {
            let (a, b, c3, d) = (
                f64::from(p1[c]),
                f64::from(p2[c]),
                f64::from(p3[c]),
                f64::from(p4[c]),
            );
            point[c] = (1. - perc).powi(3) * a
                + 3. * (1. - perc).powi(2) * perc * c3
                + 3. * (1. - perc) * perc.powi(2) * d
                + perc.powi(3) * b;
            if let Some(l) = last {
                distance += (point[c] - l[c]).powi(2);
            }
        }
        if distance != 0. {
            added += distance.sqrt();
        }
        last = Some(point);
        percents[k] = perc as f32;
        lengths[k] = added as f32;
    }
    Lengths {
        added,
        percents,
        lengths,
    }
}
fn distance_perc(perc: f64, data: &Lengths) -> f64 {
    let len = data.percents.len();
    let mut init = ((len - 1) as f64 * perc).floor() as isize;
    let length = perc * data.added;
    let at = |i: isize| f64::from(data.lengths[i as usize]);
    let pc = |i: isize| f64::from(data.percents[i as usize]);
    if init == len as isize - 1 || init == 0 || length == at(init) {
        return pc(init);
    }
    let dir = if at(init) > length { -1 } else { 1 };
    let mut l_perc = 0.;
    loop {
        if at(init) <= length && at(init + 1) > length {
            l_perc = (length - at(init)) / (at(init + 1) - at(init));
            break;
        }
        init += dir;
        if init < 0 || init >= len as isize - 1 {
            if init == len as isize - 1 {
                return pc(init);
            }
            break;
        }
    }
    let init = init.clamp(0, len as isize - 2);
    pc(init) + (pc(init + 1) - pc(init)) * l_perc
}
/// getNewSegment: [ x0, xo, xi, x1, y0, yo, yi, y1 ], rounded to thousandths in float32.
fn new_segment(
    p1: [f32; 2],
    p2: [f32; 2],
    p3: [f32; 2],
    p4: [f32; 2],
    start: f64,
    end: f64,
    data: &Lengths,
) -> [f32; 8] {
    let t0 = distance_perc(start.clamp(0., 1.), data);
    let t1 = distance_perc(end.min(1.), data);
    let (u0, u1) = (1. - t0, 1. - t1);
    let rows = [
        [
            u0 * u0 * u0,
            t0 * u0 * u0 * 3.,
            t0 * t0 * u0 * 3.,
            t0 * t0 * t0,
        ],
        [
            u0 * u0 * u1,
            t0 * u0 * u1 + u0 * t0 * u1 + u0 * u0 * t1,
            t0 * t0 * u1 + u0 * t0 * t1 + t0 * u0 * t1,
            t0 * t0 * t1,
        ],
        [
            u0 * u1 * u1,
            t0 * u1 * u1 + u0 * t1 * u1 + u0 * u1 * t1,
            t0 * t1 * u1 + u0 * t1 * t1 + t0 * u1 * t1,
            t0 * t1 * t1,
        ],
        [
            u1 * u1 * u1,
            t1 * u1 * u1 + u1 * t1 * u1 + u1 * u1 * t1,
            t1 * t1 * u1 + u1 * t1 * t1 + t1 * u1 * t1,
            t1 * t1 * t1,
        ],
    ];
    let mut out = [0f32; 8];
    for c in 0..2 {
        let (a, b, c3, d) = (
            f64::from(p1[c]),
            f64::from(p2[c]),
            f64::from(p3[c]),
            f64::from(p4[c]),
        );
        for (r, w) in rows.iter().enumerate() {
            out[c * 4 + r] =
                ((w[0] * a + w[1] * c3 + w[2] * d + w[3] * b) * 1000.).round() as f32 / 1000.;
        }
    }
    out
}
/// TrimModifier ( simultaneous ) on one path: the part between the start and end
/// lengths, rebuilt from whole and cut segments.
fn trim(path: &Path, s: f64, e: f64) -> Result<Option<Path>> {
    if e == s {
        return Ok(None);
    }
    if (e == 1. && s == 0.) || (e == 0. && s == 1.) {
        return Ok(Some(path.clone()));
    }
    let n = path.v.len();
    let count = if path.closed { n } else { n - 1 };
    let segments: Vec<Lengths> = (0..count)
        .map(|j| {
            let k = (j + 1) % n;
            bezier_length(path.v[j], path.v[k], path.o[j], path.i[k])
        })
        .collect();
    let total: f64 = segments.iter().map(|l| l.added).sum();
    let ranges = if e <= 1. {
        vec![(total * s, total * e)]
    } else if s >= 1. {
        vec![(total * (s - 1.), total * (e - 1.))]
    } else {
        vec![(total * s, total), (0., total * (e - 1.))]
    };
    // A trim wrapping past the path's end ( a non-zero offset ) is not used here.
    let &[(start, end)] = ranges.as_slice() else {
        return Err(Error::Invalid("lottie wrapping trim"));
    };
    let mut out = Path::default();
    let mut added = 0.;
    let mut fresh = true;
    let push =
        |out: &mut Path, v0: [f32; 2], o0: [f32; 2], i1: [f32; 2], v1: [f32; 2], fresh: bool| {
            if fresh {
                out.v.push(v0);
                out.i.push(v0);
                out.o.push(o0);
            } else if let Some(last) = out.o.last_mut() {
                *last = o0;
            }
            out.v.push(v1);
            out.i.push(i1);
            out.o.push(v1);
        };
    for (j, data) in segments.iter().enumerate() {
        let k = (j + 1) % n;
        if added + data.added < start {
            added += data.added;
            continue;
        }
        if added > end {
            break;
        }
        if start <= added && end >= added + data.added {
            push(&mut out, path.v[j], path.o[j], path.i[k], path.v[k], fresh);
        } else {
            let p = new_segment(
                path.v[j],
                path.v[k],
                path.o[j],
                path.i[k],
                (start - added) / data.added,
                (end - added) / data.added,
                data,
            );
            push(
                &mut out,
                [p[0], p[4]],
                [p[1], p[5]],
                [p[2], p[6]],
                [p[3], p[7]],
                fresh,
            );
        }
        fresh = false;
        added += data.added;
    }
    // A trimmed path is open; its ends' outer tangents sit on the vertices.
    if let (Some(&first), Some(&last)) = (out.v.first(), out.v.last()) {
        out.i[0] = first;
        if let Some(o) = out.o.last_mut() {
            *o = last;
        }
    }
    Ok((!out.v.is_empty()).then_some(out))
}

enum Style {
    Stroke {
        color: Prop,
        opacity: Prop,
        width: Prop,
        cap: &'static str,
        join: &'static str,
        miter: f64,
    },
    Fill {
        color: Prop,
        opacity: Prop,
    },
}
struct Layer {
    ind: i64,
    ip: f64,
    op: f64,
    matte_source: bool,
    matte: u64,
    transform: Transform,
    path: Path,
    style: Style,
    group: Transform,
    trim: Option<(Prop, Prop, Prop)>,
}
fn path(v: &Json) -> Result<Path> {
    if v["a"].as_u64() == Some(1) {
        return Err(Error::Invalid("lottie animated path"));
    }
    let k = &v["k"];
    let point = |p: &Json| {
        let n = numbers(p);
        [
            n.first().copied().unwrap_or(0.),
            n.get(1).copied().unwrap_or(0.),
        ]
    };
    let list = |key: &str| {
        k[key]
            .as_array()
            .map(|a| a.iter().map(point).collect::<Vec<_>>())
            .unwrap_or_default()
    };
    let (v, i, o) = (list("v"), list("i"), list("o"));
    let f32s = |p: [f64; 2]| [p[0] as f32, p[1] as f32];
    Ok(Path {
        i: v.iter()
            .zip(&i)
            .map(|(v, i)| f32s([v[0] + i[0], v[1] + i[1]]))
            .collect(),
        o: v.iter()
            .zip(&o)
            .map(|(v, o)| f32s([v[0] + o[0], v[1] + o[1]]))
            .collect(),
        v: v.iter().map(|&p| f32s(p)).collect(),
        closed: k["c"].as_bool().unwrap_or(false),
    })
}
fn layer(l: &Json) -> Result<Layer> {
    if l["ty"].as_u64() != Some(4) || l["parent"].is_number() || l["masksProperties"].is_array() {
        return Err(Error::Invalid("lottie layer type"));
    }
    let shapes = l["shapes"]
        .as_array()
        .ok_or(Error::Invalid("lottie shapes"))?;
    let mut group = None;
    let mut trim = None;
    for shape in shapes {
        match shape["ty"].as_str() {
            Some("gr") => group = Some(shape),
            Some("tm") => {
                if shape["m"].as_u64().unwrap_or(1) != 1 {
                    return Err(Error::Invalid("lottie trim mode"));
                }
                trim = Some((
                    Prop::new(&shape["s"], 0.01)?,
                    Prop::new(&shape["e"], 0.01)?,
                    Prop::new(&shape["o"], 1.)?,
                ));
            }
            _ => return Err(Error::Invalid("lottie layer shape")),
        }
    }
    let items = group
        .and_then(|g| g["it"].as_array())
        .ok_or(Error::Invalid("lottie group"))?;
    let (mut p, mut style, mut tr) = (None, None, None);
    for it in items {
        match it["ty"].as_str() {
            Some("sh") => p = Some(path(&it["ks"])?),
            Some("st") => {
                style = Some(Style::Stroke {
                    color: Prop::new(&it["c"], 255.)?,
                    opacity: Prop::new(&it["o"], 0.01)?,
                    width: Prop::new(&it["w"], 1.)?,
                    cap: ["butt", "round", "square"]
                        .get(it["lc"].as_u64().unwrap_or(1) as usize - 1)
                        .copied()
                        .unwrap_or("butt"),
                    join: ["miter", "round", "bevel"]
                        .get(it["lj"].as_u64().unwrap_or(1) as usize - 1)
                        .copied()
                        .unwrap_or("miter"),
                    miter: it["ml"].as_f64().unwrap_or(0.),
                })
            }
            Some("fl") => {
                style = Some(Style::Fill {
                    color: Prop::new(&it["c"], 255.)?,
                    opacity: Prop::new(&it["o"], 0.01)?,
                })
            }
            Some("tr") => tr = Some(Transform::new(it)?),
            _ => return Err(Error::Invalid("lottie group item")),
        }
    }
    Ok(Layer {
        ind: l["ind"].as_i64().unwrap_or(0),
        ip: l["ip"].as_f64().unwrap_or(0.),
        op: l["op"].as_f64().unwrap_or(0.),
        matte_source: l["td"].as_u64() == Some(1),
        matte: l["tt"].as_u64().unwrap_or(0),
        transform: Transform::new(&l["ks"])?,
        path: p.ok_or(Error::Invalid("lottie path"))?,
        style: style.ok_or(Error::Invalid("lottie style"))?,
        group: tr.ok_or(Error::Invalid("lottie group transform"))?,
        trim,
    })
}

fn canvas(
    width: u32,
    height: u32,
) -> Result<(
    web_sys::HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
)> {
    let canvas: web_sys::HtmlCanvasElement = web_sys::window()
        .and_then(|w| w.document())
        .ok_or(Error::Invalid("document"))?
        .create_element("canvas")
        .map_err(|_| Error::Invalid("canvas"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("canvas"))?;
    canvas.set_width(width);
    canvas.set_height(height);
    let context = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|c| c.dyn_into().ok())
        .ok_or(Error::Invalid("canvas context"))?;
    Ok((canvas, context))
}

/// The canvas renderer: the main canvas and the matte layer's two buffers.
struct Player {
    layers: Vec<Layer>,
    total_frames: f64,
    scale: f64,
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    buffers: [(
        web_sys::HtmlCanvasElement,
        web_sys::CanvasRenderingContext2d,
    ); 2],
}
impl Player {
    fn new(data: &Json, dpr: f64) -> Result<Self> {
        let layers = data["layers"]
            .as_array()
            .ok_or(Error::Invalid("lottie layers"))?
            .iter()
            .map(layer)
            .collect::<Result<Vec<_>>>()?;
        if layers.iter().any(|l| l.matte > 2) {
            return Err(Error::Invalid("lottie luma matte"));
        }
        let (w, h) = (
            data["w"].as_f64().unwrap_or(1.),
            data["h"].as_f64().unwrap_or(1.),
        );
        // The container is w × dpr CSS pixels; the canvas renderer multiplies by dpr again.
        let (width, height) = ((w * dpr * dpr) as u32, (h * dpr * dpr) as u32);
        let (canvas, ctx) = canvas(width, height)?;
        Ok(Self {
            total_frames: data["op"].as_f64().unwrap_or(1.) - data["ip"].as_f64().unwrap_or(0.),
            layers,
            scale: dpr * dpr,
            canvas,
            ctx,
            buffers: [self::canvas(width, height)?, self::canvas(width, height)?],
        })
    }
    fn clear(&self, ctx: &web_sys::CanvasRenderingContext2d) -> Result<()> {
        ctx.set_transform(1., 0., 0., 1., 0., 0.)
            .map_err(|_| Error::Invalid("setTransform"))?;
        ctx.clear_rect(
            0.,
            0.,
            f64::from(self.canvas.width()),
            f64::from(self.canvas.height()),
        );
        Ok(())
    }
    fn render(&self, frame: f64) -> Result<()> {
        self.clear(&self.ctx)?;
        for (index, layer) in self.layers.iter().enumerate().rev() {
            if layer.matte_source || frame < layer.ip || frame >= layer.op {
                continue;
            }
            if layer.matte == 0 {
                self.draw(layer, frame)?;
                continue;
            }
            // prepareLayer and exitLayer: the drawing so far, then this layer alone,
            // then its matte ( the layer before it ), composited source-out ( alpha
            // inverted ) or source-in, with the earlier drawing under it.
            let [(first, first_ctx), (second, second_ctx)] = &self.buffers;
            let image = |ctx: &web_sys::CanvasRenderingContext2d,
                         source: &web_sys::HtmlCanvasElement| {
                ctx.draw_image_with_html_canvas_element(source, 0., 0.)
                    .map_err(|_| Error::Invalid("drawImage"))
            };
            self.clear(first_ctx)?;
            image(first_ctx, &self.canvas)?;
            self.clear(&self.ctx)?;
            self.draw(layer, frame)?;
            self.clear(second_ctx)?;
            image(second_ctx, &self.canvas)?;
            self.clear(&self.ctx)?;
            let matte = self
                .layers
                .iter()
                .find(|l| l.ind == layer.ind - 1)
                .or_else(|| index.checked_sub(1).and_then(|i| self.layers.get(i)))
                .ok_or(Error::Invalid("lottie matte layer"))?;
            if frame >= matte.ip && frame < matte.op {
                self.draw(matte, frame)?;
            }
            self.ctx
                .set_transform(1., 0., 0., 1., 0., 0.)
                .map_err(|_| Error::Invalid("setTransform"))?;
            let op = |o: &str| {
                self.ctx
                    .set_global_composite_operation(o)
                    .map_err(|_| Error::Invalid("composite"))
            };
            op(if layer.matte == 2 {
                "source-out"
            } else {
                "source-in"
            })?;
            image(&self.ctx, second)?;
            op("destination-over")?;
            image(&self.ctx, first)?;
            op("source-over")?;
        }
        Ok(())
    }
    /// One shape layer: the layer matrix on the context, the group matrix on the
    /// ( trimmed ) path's points, then the stroke or fill.
    fn draw(&self, layer: &Layer, frame: f64) -> Result<()> {
        let ctx = &self.ctx;
        let m = layer
            .transform
            .matrix(frame)
            .then(Affine([self.scale, 0., 0., self.scale, 0., 0.]));
        let [a, b, c, d, e, f] = m.0;
        ctx.save();
        ctx.set_transform(a, b, c, d, e, f)
            .map_err(|_| Error::Invalid("setTransform"))?;
        let opacity = layer.transform.o.get(frame).first().copied().unwrap_or(1.)
            * layer.group.o.get(frame).first().copied().unwrap_or(1.);
        let path = match &layer.trim {
            Some((s, e, o)) => {
                let o = (o.get(frame).first().copied().unwrap_or(0.) % 360.) / 360.;
                let o = if o < 0. { o + 1. } else { o };
                let edge = |v: f64| {
                    if v > 1. {
                        1. + o
                    } else if v < 0. {
                        o
                    } else {
                        v + o
                    }
                };
                let (mut s, mut e) = (edge(s.get(frame)[0]), edge(e.get(frame)[0]));
                if s > e {
                    std::mem::swap(&mut s, &mut e);
                }
                trim(
                    &layer.path,
                    (s * 10000.).round() * 0.0001,
                    (e * 10000.).round() * 0.0001,
                )?
            }
            None => Some(layer.path.clone()),
        };
        let g = layer.group.matrix(frame);
        let rgb = |p: &Prop| {
            let c = p.get(frame);
            format!("rgb({},{},{})", c[0].floor(), c[1].floor(), c[2].floor())
        };
        let trace = |path: &Path| {
            let point = |p: [f32; 2]| g.apply(f64::from(p[0]), f64::from(p[1]));
            let n = path.v.len();
            for k in 1..n {
                if k == 1 {
                    let (x, y) = point(path.v[0]);
                    ctx.move_to(x, y);
                }
                let ((x1, y1), (x2, y2), (x, y)) =
                    (point(path.o[k - 1]), point(path.i[k]), point(path.v[k]));
                ctx.bezier_curve_to(x1, y1, x2, y2, x, y);
            }
            if n == 1 {
                let (x, y) = point(path.v[0]);
                ctx.move_to(x, y);
            }
            if path.closed && n > 0 {
                let ((x1, y1), (x2, y2), (x, y)) =
                    (point(path.o[n - 1]), point(path.i[0]), point(path.v[0]));
                ctx.bezier_curve_to(x1, y1, x2, y2, x, y);
                ctx.close_path();
            }
        };
        match &layer.style {
            Style::Stroke {
                color,
                opacity: o,
                width,
                cap,
                join,
                miter,
            } => {
                let w = width.get(frame)[0];
                let alpha = o.get(frame)[0] * opacity;
                if w != 0. && alpha != 0. {
                    ctx.save();
                    ctx.set_stroke_style_str(&rgb(color));
                    ctx.set_line_width(w);
                    ctx.set_line_cap(cap);
                    ctx.set_line_join(join);
                    ctx.set_miter_limit(*miter);
                    ctx.set_global_alpha(alpha);
                    ctx.begin_path();
                    if let Some(path) = &path {
                        trace(path);
                    }
                    ctx.stroke();
                    ctx.restore();
                }
            }
            Style::Fill { color, opacity: o } => {
                let alpha = o.get(frame)[0] * opacity;
                if alpha != 0. {
                    ctx.save();
                    ctx.set_fill_style_str(&rgb(color));
                    ctx.set_global_alpha(alpha);
                    ctx.begin_path();
                    if let Some(path) = &path {
                        trace(path);
                    }
                    ctx.fill();
                    ctx.restore();
                }
            }
        }
        ctx.restore();
        Ok(())
    }
}

pub struct Demo {
    controls: Controls,
    player: Player,
    texture: wgpu::Texture,
    time: f64,
    last: f64,
    seeked: bool,
    /// The scrubbed frame while paused.
    paused: Option<f64>,
    drawn: Option<f64>,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 0.1,
            far: 10.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 2.5);
        s.background = Color::from_hex(0x111111);
        s.aces_tone_mapping = true;
        s.environment = Some(super::room_environment::environment(r)?);
        let data: Json = serde_json::from_slice(&fetch(ASSET).await?)
            .map_err(|e| Error::Asset(e.to_string()))?;
        let dpr = web_sys::window().map_or(1., |w| w.device_pixel_ratio());
        let player = Player::new(&data, dpr)?;
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("lottie canvas texture"),
            size: wgpu::Extent3d {
                width: player.canvas.width(),
                height: player.canvas.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        // CanvasTexture: NearestFilter minification, linear magnification, no mipmaps.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let view = texture.create_view(&Default::default());
        let program = SurfaceNodes {
            color: Some(base_color().rgb() * Tex::External(0).sample(uv()).rgb()),
            ..Default::default()
        }
        .build(r, &[], &[(&view, &sampler)])
        .await?;
        let mut material = MeshStandardMaterial {
            roughness: 0.,
            ..Default::default()
        };
        material.properties.vertex_program = Some(Arc::new(program));
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(super::gtao::rounded_box(1., 1., 1., 7, 0.1)?),
            Arc::new(Material::Standard(material)),
        )));
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.auto_rotate = Some(2.);
        controls.update(s, c)?;
        Ok(Self {
            controls,
            player,
            texture,
            time: 0.,
            last: 0.,
            seeked: false,
            paused: None,
            drawn: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate && !self.seeked {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): controls.update() each frame ( autoRotate ), and the animation's
    /// frame, playing at 60 fps and looping over its 301 frames, drawn and uploaded.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        // controls.update() applies the pointer's rotation; autoRotate turns once per
        // 60 Hz frame of the clock.
        self.controls.update(s, c)?;
        let steps = ((self.time - self.last) * 60.).round().max(0.) as usize;
        self.last = self.time;
        for _ in 0..steps {
            self.controls.frame_update(s, c)?;
        }
        let frame = self
            .paused
            .unwrap_or((self.time * 60.) % self.player.total_frames);
        if self.drawn != Some(frame) {
            self.player.render(frame)?;
            r.queue.copy_external_image_to_texture(
                &wgpu::CopyExternalImageSourceInfo {
                    source: wgpu::ExternalImageSource::HTMLCanvasElement(
                        self.player.canvas.clone(),
                    ),
                    origin: wgpu::Origin2d::ZERO,
                    flip_y: true,
                },
                wgpu::CopyExternalImageDestInfo {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                    color_space: wgpu::PredefinedColorSpace::Srgb,
                    premultiplied_alpha: false,
                },
                self.texture.size(),
            );
            self.drawn = Some(frame);
        }
        Ok(())
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
    /// The scrubber ( goToAndStop on the frame, paused ) and play.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => self.paused = Some(f64::from(value)),
            _ => {
                if let Some(frame) = self.paused.take() {
                    // play() resumes from the scrubbed frame.
                    self.time = frame / 60.;
                    self.last = self.time;
                }
            }
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.seeked = true;
        self.time = t;
    }
}
