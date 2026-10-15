//! webgl_loader_svg: SVGLoader ( r186 ) on the page's 35 SVG files. The
//! document is parsed by the browser's DOMParser, as the loader parses it,
//! and the loader's walk is ported: presentation attributes, the browser's
//! inline style declarations and <style> rules ( read through their CSSOM
//! entries, as the loader reads them ), nested and `use` transforms with
//! ellipse arc transforms, path commands and number parsing, rect, polygon,
//! polyline, circle, ellipse and line nodes, units, and linear and radial
//! gradients drawn into canvas textures. Each path becomes the page's fill
//! meshes ( ShapePath.toShapes with the fill rule, then ShapeGeometry ) and
//! stroke meshes ( pointsToStroke's joins and caps ) in render order, with
//! the fill and stroke materials, under the grid and OrbitControls with
//! screen-space panning. The geometry is built on the CPU when a file or an
//! option is chosen, as the page builds it; the files are fetched with the
//! example instead of on each choice.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use super::interactive_scenes::grid_helper;
use super::text_shapes::triangulate;
use crate::attribute::BufferAttribute;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use js_sys::{Array, Function, Object, Reflect};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::{JsCast, JsValue};

const FILES: [&str; 35] = [
    "tiger.svg",
    "lineJoinsAndCaps.svg",
    "hexagon.svg",
    "energy.svg",
    "tests/1.svg",
    "tests/2.svg",
    "tests/3.svg",
    "tests/4.svg",
    "tests/5.svg",
    "tests/6.svg",
    "tests/7.svg",
    "tests/8.svg",
    "tests/9.svg",
    "tests/units.svg",
    "tests/ordering.svg",
    "tests/testDefs/Svg-defs.svg",
    "tests/testDefs/Svg-defs2.svg",
    "tests/testDefs/Wave-defs.svg",
    "tests/testDefs/defs4.svg",
    "tests/testDefs/defs5.svg",
    "style-css-inside-defs.svg",
    "styled-paths.svg",
    "multiple-css-classes.svg",
    "zero-radius.svg",
    "tests/styles.svg",
    "tests/roundJoinPrecisionIssue.svg",
    "tests/ellipseTransform.svg",
    "singlePointTest.svg",
    "singlePointTest2.svg",
    "singlePointTest3.svg",
    "emptyPath.svg",
    "emoji.svg",
    "blueprint.svg",
    "tests/wideStroke.svg",
    "tests/letter.svg",
];
const EPSILON: f64 = f64::EPSILON;

type P2 = [f64; 2];

// ---------------------------------------------------------------- JS numbers

fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}
fn js_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.min(b)
    }
}
/// parseFloat( string ).
fn parse_float(s: &str) -> f64 {
    js_sys::Number::parse_float(s)
}
/// Number( string ) of a token the number scanner accepted.
fn number(s: &str) -> f64 {
    js_sys::Number::from(JsValue::from_str(s)).value_of()
}

// ---------------------------------------------------------------- DOM

fn get(o: &JsValue, key: &str) -> JsValue {
    Reflect::get(o, &key.into()).unwrap_or(JsValue::UNDEFINED)
}
fn call(o: &JsValue, method: &str, args: &[JsValue]) -> JsValue {
    let Ok(f) = get(o, method).dyn_into::<Function>() else {
        return JsValue::UNDEFINED;
    };
    let array = Array::new();
    for a in args {
        array.push(a);
    }
    f.apply(o, &array).unwrap_or(JsValue::UNDEFINED)
}
fn has(node: &JsValue, name: &str) -> bool {
    call(node, "hasAttribute", &[name.into()]).as_bool() == Some(true)
}
fn attr(node: &JsValue, name: &str) -> Option<String> {
    call(node, "getAttribute", &[name.into()]).as_string()
}
fn name(node: &JsValue) -> String {
    get(node, "nodeName").as_string().unwrap_or_default()
}
fn list(o: &JsValue) -> Vec<JsValue> {
    let n = get(o, "length").as_f64().unwrap_or(0.) as u32;
    (0..n)
        .map(|i| Reflect::get_u32(o, i).unwrap_or(JsValue::UNDEFINED))
        .collect()
}

// ---------------------------------------------------------------- Colors

include!("svg_loader_keywords.rs");

/// Color.setStyle( style, SRGBColorSpace ), into the linear working space.
fn set_style(color: &mut Color, style: &str) {
    fn hsl(h: f64, s: f64, l: f64) -> [f64; 3] {
        let h = h.rem_euclid(1.);
        let (s, l) = (s.clamp(0., 1.), l.clamp(0., 1.));
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
                p + (q - p) * 6. * t
            } else if t < 1. / 2. {
                q
            } else if t < 2. / 3. {
                p + (q - p) * 6. * (2. / 3. - t)
            } else {
                p
            }
        };
        [hue(q, p, h + 1. / 3.), hue(q, p, h), hue(q, p, h - 1. / 3.)]
    }
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let decimal = |s: &str| {
        let s = s.trim();
        let (a, b) = s.split_once('.').unwrap_or(("", s));
        !b.is_empty()
            && b.bytes().all(|c| c.is_ascii_digit())
            && a.bytes().all(|c| c.is_ascii_digit())
            && (s.contains('.') || !a.is_empty() || !b.is_empty())
    };
    if let Some(open) = style.find('(') {
        let model = &style[..open];
        let Some(close) = style[open + 1..].find(')') else {
            return;
        };
        if model.is_empty()
            || !model
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        {
            return;
        }
        let parts: Vec<&str> = style[open + 1..open + 1 + close]
            .split(',')
            .map(str::trim)
            .collect();
        if parts.len() < 3 || parts.len() > 4 || (parts.len() == 4 && !decimal(parts[3])) {
            return;
        }
        match model {
            "rgb" | "rgba" => {
                if parts[..3].iter().all(|p| digits(p)) {
                    let c = |p: &str| (p.parse::<f64>().unwrap_or(0.)).min(255.) / 255.;
                    *color = Color::from_srgb(c(parts[0]), c(parts[1]), c(parts[2]));
                } else if parts[..3]
                    .iter()
                    .all(|p| p.ends_with('%') && digits(&p[..p.len() - 1]))
                {
                    let c =
                        |p: &str| (p[..p.len() - 1].parse::<f64>().unwrap_or(0.)).min(100.) / 100.;
                    *color = Color::from_srgb(c(parts[0]), c(parts[1]), c(parts[2]));
                }
            }
            "hsl" | "hsla" => {
                let pct = |p: &str| p.ends_with('%') && decimal(&p[..p.len() - 1]);
                if decimal(parts[0]) && pct(parts[1]) && pct(parts[2]) {
                    let [r, g, b] = hsl(
                        parse_float(parts[0]) / 360.,
                        parse_float(parts[1]) / 100.,
                        parse_float(parts[2]) / 100.,
                    );
                    *color = Color::from_srgb(r, g, b);
                }
            }
            _ => {}
        }
    } else if let Some(hex) = style.strip_prefix('#') {
        if hex.is_empty() || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return;
        }
        if hex.len() == 3 {
            let c = |i: usize| f64::from(u8::from_str_radix(&hex[i..i + 1], 16).unwrap_or(0)) / 15.;
            *color = Color::from_srgb(c(0), c(1), c(2));
        } else if hex.len() == 6 {
            *color = Color::from_hex(u32::from_str_radix(hex, 16).unwrap_or(0));
        }
    } else if !style.is_empty() {
        let lower = style.to_lowercase();
        if let Some((_, hex)) = KEYWORDS.iter().find(|(k, _)| *k == lower) {
            *color = Color::from_hex(*hex);
        }
    }
}

// ---------------------------------------------------------------- Curves

#[derive(Clone, Copy)]
struct Ellipse {
    ax: f64,
    ay: f64,
    rx: f64,
    ry: f64,
    start: f64,
    end: f64,
    clockwise: bool,
    rotation: f64,
}
impl Ellipse {
    /// EllipseCurve.getPoint( t ).
    fn point(&self, t: f64) -> P2 {
        let two_pi = PI * 2.;
        let mut delta = self.end - self.start;
        let same = delta.abs() < EPSILON;
        while delta < 0. {
            delta += two_pi;
        }
        while delta > two_pi {
            delta -= two_pi;
        }
        if delta < EPSILON {
            delta = if same { 0. } else { two_pi };
        }
        if self.clockwise && !same {
            delta = if delta == two_pi {
                -two_pi
            } else {
                delta - two_pi
            };
        }
        let angle = self.start + t * delta;
        let mut x = self.ax + self.rx * angle.cos();
        let mut y = self.ay + self.ry * angle.sin();
        if self.rotation != 0. {
            let (c, s) = (self.rotation.cos(), self.rotation.sin());
            let (tx, ty) = (x - self.ax, y - self.ay);
            x = tx * c - ty * s + self.ax;
            y = tx * s + ty * c + self.ay;
        }
        [x, y]
    }
}
#[derive(Clone, Copy)]
enum Curve {
    Line(P2, P2),
    Quadratic(P2, P2, P2),
    Cubic(P2, P2, P2, P2),
    Ellipse(Ellipse),
}
impl Curve {
    fn point(&self, t: f64) -> P2 {
        match *self {
            Curve::Line(a, b) => {
                if t == 1. {
                    b
                } else {
                    [(b[0] - a[0]) * t + a[0], (b[1] - a[1]) * t + a[1]]
                }
            }
            Curve::Quadratic(a, c, b) => {
                let k = 1. - t;
                let f = |i: usize| k * k * a[i] + 2. * (1. - t) * t * c[i] + t * t * b[i];
                [f(0), f(1)]
            }
            Curve::Cubic(a, c1, c2, b) => {
                let k = 1. - t;
                let f = |i: usize| {
                    k * k * k * a[i]
                        + 3. * k * k * t * c1[i]
                        + 3. * (1. - t) * t * t * c2[i]
                        + t * t * t * b[i]
                };
                [f(0), f(1)]
            }
            Curve::Ellipse(e) => e.point(t),
        }
    }
}
#[derive(Clone, Default)]
struct Path {
    curves: Vec<Curve>,
    current: P2,
    auto_close: bool,
}
impl Path {
    fn move_to(&mut self, p: P2) {
        self.current = p;
    }
    fn line_to(&mut self, p: P2) {
        self.curves.push(Curve::Line(self.current, p));
        self.current = p;
    }
    fn quadratic_to(&mut self, c: P2, p: P2) {
        self.curves.push(Curve::Quadratic(self.current, c, p));
        self.current = p;
    }
    fn bezier_to(&mut self, c1: P2, c2: P2, p: P2) {
        self.curves.push(Curve::Cubic(self.current, c1, c2, p));
        self.current = p;
    }
    /// absellipse(): a line from the current point when the arc starts elsewhere.
    fn absellipse(&mut self, e: Ellipse) {
        if !self.curves.is_empty() {
            let first = e.point(0.);
            if first != self.current {
                self.line_to(first);
            }
        }
        self.curves.push(Curve::Ellipse(e));
        self.current = e.point(1.);
    }
    /// CurvePath.getPoints( divisions ).
    fn points(&self, divisions: usize) -> Vec<P2> {
        let mut out: Vec<P2> = vec![];
        for c in &self.curves {
            let resolution = match c {
                Curve::Ellipse(_) => divisions * 2,
                Curve::Line(..) => 1,
                _ => divisions,
            };
            for d in 0..=resolution {
                let p = c.point(d as f64 / resolution as f64);
                if out.last() == Some(&p) {
                    continue;
                }
                out.push(p);
            }
        }
        if self.auto_close && out.len() > 1 && out[out.len() - 1] != out[0] {
            out.push(out[0]);
        }
        out
    }
}

// ---------------------------------------------------------------- Styles

#[derive(Clone, Debug)]
struct Style {
    fill: Option<String>,
    fill_opacity: f64,
    fill_rule: Option<String>,
    opacity: Option<f64>,
    stroke: Option<String>,
    stroke_opacity: f64,
    stroke_width: f64,
    line_join: String,
    line_cap: String,
    miter_limit: f64,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            fill: Some("#000".into()),
            fill_opacity: 1.,
            fill_rule: None,
            opacity: None,
            stroke: None,
            stroke_opacity: 1.,
            stroke_width: 1.,
            line_join: "miter".into(),
            line_cap: "butt".into(),
            miter_limit: 4.,
        }
    }
}

#[derive(Clone)]
struct Gradient {
    linear: bool,
    bbox_units: bool,
    spread: Wrapping,
    transform: Option<Matrix3>,
    stops: Vec<(f64, String, f64)>,
    p: [f64; 5],
}

struct ShapePath {
    sub_paths: Vec<Path>,
    current: Option<usize>,
    color: Color,
    style: Style,
    transform: Matrix3,
}
impl ShapePath {
    fn new() -> Self {
        Self {
            sub_paths: vec![],
            current: None,
            color: Color::WHITE,
            style: Style::default(),
            transform: Matrix3::IDENTITY,
        }
    }
    fn path(&mut self) -> Result<&mut Path> {
        let i = self
            .current
            .ok_or(Error::Asset("svg path without moveTo".into()))?;
        Ok(&mut self.sub_paths[i])
    }
    fn move_to(&mut self, p: P2) {
        let mut path = Path::default();
        path.move_to(p);
        self.sub_paths.push(path);
        self.current = Some(self.sub_paths.len() - 1);
    }
    fn line_to(&mut self, p: P2) -> Result<()> {
        self.path()?.line_to(p);
        Ok(())
    }
    fn quadratic_to(&mut self, c: P2, p: P2) -> Result<()> {
        self.path()?.quadratic_to(c, p);
        Ok(())
    }
    fn bezier_to(&mut self, c1: P2, c2: P2, p: P2) -> Result<()> {
        self.path()?.bezier_to(c1, c2, p);
        Ok(())
    }
}

/// Matrix3.set( n11 … n33 ), row-major.
fn m3(n: [f64; 9]) -> Matrix3 {
    Matrix3::from_cols(
        Vector3::new(n[0], n[3], n[6]),
        Vector3::new(n[1], n[4], n[7]),
        Vector3::new(n[2], n[5], n[8]),
    )
}
fn apply(m: &Matrix3, p: P2) -> P2 {
    let v = *m * Vector3::new(p[0], p[1], 1.);
    [v.x, v.y]
}
/// Matrix3.elements, column-major.
fn el(m: &Matrix3) -> [f64; 9] {
    m.to_cols_array()
}

// ---------------------------------------------------------------- Parser

struct Parser {
    paths: Vec<ShapePath>,
    stylesheets: HashMap<String, Vec<(String, String)>>,
    gradients: HashMap<String, Gradient>,
    stack: Vec<Matrix3>,
    current: Matrix3,
}

fn parse_floats(input: &str, flags: Option<[usize; 2]>, stride: usize) -> Vec<f64> {
    #[derive(PartialEq, Clone, Copy)]
    enum S {
        Sep,
        Int,
        Float,
        Exp,
    }
    let ws = |c: char| matches!(c, ' ' | '\t' | '\r' | '\n');
    let sign = |c: char| matches!(c, '-' | '+');
    let mut state = S::Sep;
    let mut seen_comma = true;
    let (mut num, mut exp) = (String::new(), String::new());
    let mut result: Vec<f64> = vec![];
    let push = |num: &mut String, exp: &mut String, result: &mut Vec<f64>| {
        if !num.is_empty() {
            if exp.is_empty() {
                result.push(number(num));
            } else {
                result.push(number(num) * 10f64.powf(number(exp)));
            }
        }
        num.clear();
        exp.clear();
    };
    for c in input.chars() {
        if let Some(f) = flags
            && stride > 0
            && f.contains(&(result.len() % stride))
            && matches!(c, '0' | '1')
        {
            state = S::Int;
            num = c.to_string();
            push(&mut num, &mut exp, &mut result);
            continue;
        }
        if state == S::Sep {
            if ws(c) {
                continue;
            }
            if c.is_ascii_digit() || sign(c) {
                state = S::Int;
                num = c.to_string();
                continue;
            }
            if c == '.' {
                state = S::Float;
                num = c.to_string();
                continue;
            }
            if c == ',' {
                if seen_comma {
                    return result;
                }
                seen_comma = true;
            }
        }
        if state == S::Int {
            if c.is_ascii_digit() {
                num.push(c);
                continue;
            }
            if c == '.' {
                num.push(c);
                state = S::Float;
                continue;
            }
            if c == 'e' || c == 'E' {
                state = S::Exp;
                continue;
            }
            if sign(c) && num.len() == 1 && num.starts_with(sign) {
                return result;
            }
        }
        if state == S::Float {
            if c.is_ascii_digit() {
                num.push(c);
                continue;
            }
            if c == 'e' || c == 'E' {
                state = S::Exp;
                continue;
            }
            if c == '.' && num.ends_with('.') {
                return result;
            }
        }
        if state == S::Exp {
            if c.is_ascii_digit() {
                exp.push(c);
                continue;
            }
            if sign(c) {
                if exp.is_empty() {
                    exp.push(c);
                    continue;
                }
                if exp.len() == 1 && exp.starts_with(sign) {
                    return result;
                }
            }
        }
        if ws(c) {
            push(&mut num, &mut exp, &mut result);
            state = S::Sep;
            seen_comma = false;
        } else if c == ',' {
            push(&mut num, &mut exp, &mut result);
            state = S::Sep;
            seen_comma = true;
        } else if sign(c) {
            push(&mut num, &mut exp, &mut result);
            state = S::Int;
            num = c.to_string();
        } else if c == '.' {
            push(&mut num, &mut exp, &mut result);
            state = S::Float;
            num = c.to_string();
        } else {
            // SyntaxError: the loader stops at the unexpected character.
            return result;
        }
    }
    push(&mut num, &mut exp, &mut result);
    result
}

/// parseFloatWithUnits, with the loader's default px unit and 90 DPI.
fn units(s: &str) -> f64 {
    let table: [(&str, f64); 6] = [
        ("mm", 90. / 25.4),
        ("cm", 90. / 2.54),
        ("in", 90.),
        ("pt", 90. / 72.),
        ("pc", 90. / 6.),
        ("px", 1.),
    ];
    for (u, scale) in table {
        if let Some(v) = s.strip_suffix(u) {
            return scale * parse_float(v);
        }
    }
    parse_float(s)
}
fn units_or(node: &JsValue, name: &str) -> f64 {
    match attr(node, name).filter(|v| !v.is_empty()) {
        Some(v) => units(&v),
        None => 0.,
    }
}

fn svg_angle(ux: f64, uy: f64, vx: f64, vy: f64) -> f64 {
    let dot = ux * vx + uy * vy;
    let len = (ux * ux + uy * uy).sqrt() * (vx * vx + vy * vy).sqrt();
    let mut ang = js_max(-1., js_min(1., dot / len)).acos();
    if ux * vy - uy * vx < 0. {
        ang = -ang;
    }
    ang
}

#[allow(clippy::too_many_arguments)]
fn arc(
    path: &mut ShapePath,
    mut rx: f64,
    mut ry: f64,
    rotation: f64,
    large: f64,
    sweep: f64,
    start: P2,
    end: P2,
) -> Result<()> {
    if rx == 0. || ry == 0. {
        return path.line_to(end);
    }
    let rot = rotation * PI / 180.;
    rx = rx.abs();
    ry = ry.abs();
    let dx2 = (start[0] - end[0]) / 2.;
    let dy2 = (start[1] - end[1]) / 2.;
    let x1p = rot.cos() * dx2 + rot.sin() * dy2;
    let y1p = -rot.sin() * dx2 + rot.cos() * dy2;
    let mut rxs = rx * rx;
    let mut rys = ry * ry;
    let (x1ps, y1ps) = (x1p * x1p, y1p * y1p);
    let cr = x1ps / rxs + y1ps / rys;
    if cr > 1. {
        let s = cr.sqrt();
        rx *= s;
        ry *= s;
        rxs = rx * rx;
        rys = ry * ry;
    }
    let dq = rxs * y1ps + rys * x1ps;
    let pq = (rxs * rys - dq) / dq;
    let mut q = js_max(0., pq).sqrt();
    if large == sweep {
        q = -q;
    }
    let cxp = q * rx * y1p / ry;
    let cyp = -q * ry * x1p / rx;
    let cx = rot.cos() * cxp - rot.sin() * cyp + (start[0] + end[0]) / 2.;
    let cy = rot.sin() * cxp + rot.cos() * cyp + (start[1] + end[1]) / 2.;
    let theta = svg_angle(1., 0., (x1p - cxp) / rx, (y1p - cyp) / ry);
    let delta = svg_angle(
        (x1p - cxp) / rx,
        (y1p - cyp) / ry,
        (-x1p - cxp) / rx,
        (-y1p - cyp) / ry,
    ) % (PI * 2.);
    path.path()?.absellipse(Ellipse {
        ax: cx,
        ay: cy,
        rx,
        ry,
        start: theta,
        end: theta + delta,
        clockwise: sweep == 0.,
        rotation: rot,
    });
    Ok(())
}

fn parse_path(node: &JsValue) -> Result<Option<ShapePath>> {
    let mut path = ShapePath::new();
    let d = attr(node, "d").unwrap_or_default();
    if d.is_empty() || d == "none" {
        return Ok(None);
    }
    // d.match( /[a-df-z][^a-df-z]*/ig )
    let mut commands: Vec<String> = vec![];
    let command_char = |c: char| c.is_ascii_alphabetic() && !matches!(c, 'e' | 'E');
    for c in d.chars() {
        if command_char(c) {
            commands.push(c.to_string());
        } else if let Some(last) = commands.last_mut() {
            last.push(c);
        }
    }
    let mut point: P2 = [0., 0.];
    let mut control: P2 = [0., 0.];
    let mut first: P2 = [0., 0.];
    let mut is_first = true;
    let mut set_first;
    let at = |n: &[f64], i: usize| n.get(i).copied().unwrap_or(f64::NAN);
    let reflect = |a: f64, b: f64| a - (b - a);
    for command in commands {
        let kind = command.chars().next().unwrap_or(' ');
        let data = command[kind.len_utf8()..].trim();
        set_first = false;
        if is_first {
            set_first = true;
            is_first = false;
        }
        let relative = kind.is_ascii_lowercase();
        let (ox, oy) = if relative {
            (point[0], point[1])
        } else {
            (0., 0.)
        };
        match kind.to_ascii_uppercase() {
            'M' => {
                let n = parse_floats(data, None, 0);
                let mut j = 0;
                while j < n.len() {
                    if relative {
                        point[0] += at(&n, j);
                        point[1] += at(&n, j + 1);
                    } else {
                        point = [at(&n, j), at(&n, j + 1)];
                    }
                    control = point;
                    if j == 0 {
                        path.move_to(point);
                        first = point;
                    } else {
                        path.line_to(point)?;
                    }
                    j += 2;
                }
            }
            'H' | 'V' => {
                let n = parse_floats(data, None, 0);
                for (j, &v) in n.iter().enumerate() {
                    let axis = usize::from(kind.eq_ignore_ascii_case(&'v'));
                    point[axis] = if relative { point[axis] + v } else { v };
                    control = point;
                    path.line_to(point)?;
                    if j == 0 && set_first {
                        first = point;
                    }
                }
            }
            'L' => {
                let n = parse_floats(data, None, 0);
                let mut j = 0;
                while j < n.len() {
                    if relative {
                        point[0] += at(&n, j);
                        point[1] += at(&n, j + 1);
                    } else {
                        point = [at(&n, j), at(&n, j + 1)];
                    }
                    control = point;
                    path.line_to(point)?;
                    if j == 0 && set_first {
                        first = point;
                    }
                    j += 2;
                }
            }
            'C' => {
                let n = parse_floats(data, None, 0);
                let mut j = 0;
                while j < n.len() {
                    let (ox, oy) = if relative {
                        (point[0], point[1])
                    } else {
                        (0., 0.)
                    };
                    path.bezier_to(
                        [ox + at(&n, j), oy + at(&n, j + 1)],
                        [ox + at(&n, j + 2), oy + at(&n, j + 3)],
                        [ox + at(&n, j + 4), oy + at(&n, j + 5)],
                    )?;
                    control = [ox + at(&n, j + 2), oy + at(&n, j + 3)];
                    point = [ox + at(&n, j + 4), oy + at(&n, j + 5)];
                    if j == 0 && set_first {
                        first = point;
                    }
                    j += 6;
                }
            }
            'S' => {
                let n = parse_floats(data, None, 0);
                let mut j = 0;
                while j < n.len() {
                    let (ox, oy) = if relative {
                        (point[0], point[1])
                    } else {
                        (0., 0.)
                    };
                    path.bezier_to(
                        [reflect(point[0], control[0]), reflect(point[1], control[1])],
                        [ox + at(&n, j), oy + at(&n, j + 1)],
                        [ox + at(&n, j + 2), oy + at(&n, j + 3)],
                    )?;
                    control = [ox + at(&n, j), oy + at(&n, j + 1)];
                    point = [ox + at(&n, j + 2), oy + at(&n, j + 3)];
                    if j == 0 && set_first {
                        first = point;
                    }
                    j += 4;
                }
            }
            'Q' => {
                let n = parse_floats(data, None, 0);
                let mut j = 0;
                while j < n.len() {
                    let (ox, oy) = if relative {
                        (point[0], point[1])
                    } else {
                        (0., 0.)
                    };
                    path.quadratic_to(
                        [ox + at(&n, j), oy + at(&n, j + 1)],
                        [ox + at(&n, j + 2), oy + at(&n, j + 3)],
                    )?;
                    control = [ox + at(&n, j), oy + at(&n, j + 1)];
                    point = [ox + at(&n, j + 2), oy + at(&n, j + 3)];
                    if j == 0 && set_first {
                        first = point;
                    }
                    j += 4;
                }
            }
            'T' => {
                let n = parse_floats(data, None, 0);
                let mut j = 0;
                while j < n.len() {
                    let (ox, oy) = if relative {
                        (point[0], point[1])
                    } else {
                        (0., 0.)
                    };
                    let r = [reflect(point[0], control[0]), reflect(point[1], control[1])];
                    path.quadratic_to(r, [ox + at(&n, j), oy + at(&n, j + 1)])?;
                    control = r;
                    point = [ox + at(&n, j), oy + at(&n, j + 1)];
                    if j == 0 && set_first {
                        first = point;
                    }
                    j += 2;
                }
            }
            'A' => {
                let n = parse_floats(data, Some([3, 4]), 7);
                let mut j = 0;
                while j < n.len() {
                    let (tx, ty) = (at(&n, j + 5), at(&n, j + 6));
                    let skip = if relative {
                        tx == 0. && ty == 0.
                    } else {
                        tx == point[0] && ty == point[1]
                    };
                    if !skip {
                        let start = point;
                        point = if relative {
                            [point[0] + tx, point[1] + ty]
                        } else {
                            [tx, ty]
                        };
                        control = point;
                        arc(
                            &mut path,
                            at(&n, j),
                            at(&n, j + 1),
                            at(&n, j + 2),
                            at(&n, j + 3),
                            at(&n, j + 4),
                            start,
                            point,
                        )?;
                        if j == 0 && set_first {
                            first = point;
                        }
                    }
                    j += 7;
                }
            }
            'Z' => {
                let p = path.path()?;
                p.auto_close = true;
                if !p.curves.is_empty() {
                    point = first;
                    p.current = point;
                    is_first = true;
                }
            }
            _ => {}
        }
        let _ = (ox, oy);
    }
    Ok(Some(path))
}

/// The polygon / polyline points: ( number )( , or whitespace )( number ) pairs.
fn point_pairs(text: &str) -> Vec<P2> {
    fn number_at(s: &[u8], mut i: usize) -> Option<usize> {
        let start = i;
        if i < s.len() && (s[i] == b'+' || s[i] == b'-') {
            i += 1;
        }
        let digits = |i: &mut usize| {
            let b = *i;
            while *i < s.len() && s[*i].is_ascii_digit() {
                *i += 1;
            }
            *i > b
        };
        let a = digits(&mut i);
        if i < s.len() && s[i] == b'.' {
            let save = i;
            i += 1;
            if !digits(&mut i) {
                i = save;
                if !a {
                    return None;
                }
            }
        } else if !a {
            return None;
        }
        if i < s.len() && (s[i] == b'e' || s[i] == b'E') {
            let save = i;
            i += 1;
            if i < s.len() && (s[i] == b'+' || s[i] == b'-') {
                i += 1;
            }
            if !digits(&mut i) {
                i = save;
            }
        }
        (i > start).then_some(i)
    }
    let s = text.as_bytes();
    let mut out = vec![];
    let mut i = 0;
    while i < s.len() {
        if let Some(a) = number_at(s, i)
            && a < s.len()
            && (s[a] == b',' || s[a].is_ascii_whitespace())
            && let Some(b) = number_at(s, a + 1)
        {
            out.push([units(&text[i..a]), units(&text[a + 1..b])]);
            i = b;
            continue;
        }
        i += 1;
    }
    out
}

impl Parser {
    fn parse_style(&self, node: &JsValue, style: &Style) -> Style {
        let mut style = style.clone();
        let mut sheet: HashMap<String, String> = HashMap::new();
        if let Some(class) = attr(node, "class") {
            for c in class.split_whitespace() {
                if let Some(d) = self.stylesheets.get(&format!(".{c}")) {
                    for (k, v) in d {
                        sheet.insert(k.clone(), v.clone());
                    }
                }
            }
        }
        if let Some(id) = attr(node, "id")
            && let Some(d) = self.stylesheets.get(&format!("#{id}"))
        {
            for (k, v) in d {
                sheet.insert(k.clone(), v.clone());
            }
        }
        let inline = get(node, "style");
        // addStyle: the attribute, the stylesheet entry by the style's own
        // key, then the inline declaration.
        let add = |svg: &str, js: &str, f: &mut dyn FnMut(&str)| {
            if has(node, svg)
                && let Some(v) = attr(node, svg)
            {
                f(&v);
            }
            if let Some(v) = sheet.get(js).filter(|v| !v.is_empty()) {
                f(v);
            }
            if !inline.is_undefined()
                && !inline.is_null()
                && let Some(v) = get(&inline, svg).as_string()
                && !v.is_empty()
            {
                f(&v);
            }
        };
        let clamp = |v: &str| js_max(0., js_min(1., units(v)));
        let positive = |v: &str| js_max(0., units(v));
        add("fill", "fill", &mut |v| style.fill = Some(v.to_string()));
        add("fill-opacity", "fillOpacity", &mut |v| {
            style.fill_opacity = clamp(v)
        });
        add("fill-rule", "fillRule", &mut |v| {
            style.fill_rule = Some(v.to_string())
        });
        add("opacity", "opacity", &mut |v| {
            style.opacity = Some(clamp(v))
        });
        add("stroke", "stroke", &mut |v| {
            style.stroke = Some(v.to_string())
        });
        add("stroke-opacity", "strokeOpacity", &mut |v| {
            style.stroke_opacity = clamp(v)
        });
        add("stroke-width", "strokeWidth", &mut |v| {
            style.stroke_width = positive(v)
        });
        add("stroke-linejoin", "strokeLineJoin", &mut |v| {
            style.line_join = v.to_string()
        });
        add("stroke-linecap", "strokeLineCap", &mut |v| {
            style.line_cap = v.to_string()
        });
        add("stroke-miterlimit", "strokeMiterLimit", &mut |v| {
            style.miter_limit = positive(v)
        });
        add("visibility", "visibility", &mut |_| {});
        style
    }
    /// parseCSSStylesheet: each rule's Object.entries( style ) by selector.
    fn parse_stylesheet(&mut self, node: &JsValue) {
        let sheet = get(node, "sheet");
        if sheet.is_undefined() || sheet.is_null() {
            return;
        }
        for rule in list(&get(&sheet, "cssRules")) {
            if get(&rule, "type").as_f64() != Some(1.) {
                continue;
            }
            let selector = get(&rule, "selectorText").as_string().unwrap_or_default();
            let style = get(&rule, "style");
            let entries: Vec<(String, String)> = Object::entries(&Object::from(style))
                .iter()
                .filter_map(|e| {
                    let k = Reflect::get_u32(&e, 0).ok()?.as_string()?;
                    let v = Reflect::get_u32(&e, 1).ok()?.as_string()?;
                    (!v.is_empty()).then_some((k, v))
                })
                .collect();
            for s in selector.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                let d = self.stylesheets.entry(s.to_string()).or_default();
                for (k, v) in &entries {
                    if let Some(e) = d.iter_mut().find(|(dk, _)| dk == k) {
                        e.1 = v.clone();
                    } else {
                        d.push((k.clone(), v.clone()));
                    }
                }
            }
        }
    }
    fn parse_transform(text: &str, transform: &mut Matrix3) {
        let parts: Vec<&str> = text.split(')').collect();
        for part in parts.iter().rev() {
            let t = part.trim();
            if t.is_empty() {
                continue;
            }
            let Some(open) = t.find('(') else {
                continue;
            };
            if open == 0 {
                continue;
            }
            let kind = &t[..open];
            let a = parse_floats(&t[open + 1..], None, 0);
            let mut current = Matrix3::IDENTITY;
            match kind {
                "translate" if !a.is_empty() => {
                    current = m3([
                        1.,
                        0.,
                        a[0],
                        0.,
                        1.,
                        a.get(1).copied().unwrap_or(0.),
                        0.,
                        0.,
                        1.,
                    ]);
                }
                "rotate" if !a.is_empty() => {
                    let angle = a[0] * PI / 180.;
                    let (cx, cy) = if a.len() >= 3 { (a[1], a[2]) } else { (0., 0.) };
                    let (c, s) = (angle.cos(), angle.sin());
                    let t1 = m3([1., 0., -cx, 0., 1., -cy, 0., 0., 1.]);
                    let r = m3([c, -s, 0., s, c, 0., 0., 0., 1.]);
                    let t2 = m3([1., 0., cx, 0., 1., cy, 0., 0., 1.]);
                    current = t2 * (r * t1);
                }
                "scale" if !a.is_empty() => {
                    let sy = a.get(1).copied().unwrap_or(a[0]);
                    current = m3([a[0], 0., 0., 0., sy, 0., 0., 0., 1.]);
                }
                "skewX" if a.len() == 1 => {
                    current = m3([1., (a[0] * PI / 180.).tan(), 0., 0., 1., 0., 0., 0., 1.])
                }
                "skewY" if a.len() == 1 => {
                    current = m3([1., 0., 0., (a[0] * PI / 180.).tan(), 1., 0., 0., 0., 1.])
                }
                "matrix" if a.len() == 6 => {
                    current = m3([a[0], a[2], a[4], a[1], a[3], a[5], 0., 0., 1.])
                }
                _ => {}
            }
            *transform = current * *transform;
        }
    }
    fn node_transform(&mut self, node: &JsValue) -> Option<Matrix3> {
        let is_use = name(node) == "use";
        if !(has(node, "transform") || (is_use && (has(node, "x") || has(node, "y")))) {
            return None;
        }
        let mut transform = Matrix3::IDENTITY;
        if is_use && (has(node, "x") || has(node, "y")) {
            transform = m3([
                1.,
                0.,
                units_or(node, "x"),
                0.,
                1.,
                units_or(node, "y"),
                0.,
                0.,
                1.,
            ]);
        }
        if let Some(t) = attr(node, "transform") {
            Self::parse_transform(&t, &mut transform);
        }
        if let Some(top) = self.stack.last() {
            transform = *top * transform;
        }
        self.current = transform;
        self.stack.push(transform);
        Some(transform)
    }
    fn parse_node(&mut self, node: &JsValue, style: &Style) -> Result<()> {
        if get(node, "nodeType").as_f64() != Some(1.) {
            return Ok(());
        }
        let transform = self.node_transform(node);
        let mut style = style.clone();
        let mut defs = false;
        let mut path = None;
        let node_name = name(node);
        match node_name.as_str() {
            "svg" | "g" => style = self.parse_style(node, &style),
            "style" => self.parse_stylesheet(node),
            "path" => {
                style = self.parse_style(node, &style);
                if has(node, "d") {
                    path = parse_path(node)?;
                }
            }
            "rect" => {
                style = self.parse_style(node, &style);
                let x = units_or(node, "x");
                let y = units_or(node, "y");
                let rx = attr(node, "rx")
                    .filter(|v| !v.is_empty())
                    .or_else(|| attr(node, "ry").filter(|v| !v.is_empty()))
                    .map_or(0., |v| units(&v));
                let ry = attr(node, "ry")
                    .filter(|v| !v.is_empty())
                    .or_else(|| attr(node, "rx").filter(|v| !v.is_empty()))
                    .map_or(0., |v| units(&v));
                let w = attr(node, "width").map_or(f64::NAN, |v| units(&v));
                let h = attr(node, "height").map_or(f64::NAN, |v| units(&v));
                let bci = 1. - 0.551915024494;
                let mut p = ShapePath::new();
                p.move_to([x + rx, y]);
                p.line_to([x + w - rx, y])?;
                let rounded = rx != 0. || ry != 0.;
                if rounded {
                    p.bezier_to(
                        [x + w - rx * bci, y],
                        [x + w, y + ry * bci],
                        [x + w, y + ry],
                    )?;
                }
                p.line_to([x + w, y + h - ry])?;
                if rounded {
                    p.bezier_to(
                        [x + w, y + h - ry * bci],
                        [x + w - rx * bci, y + h],
                        [x + w - rx, y + h],
                    )?;
                }
                p.line_to([x + rx, y + h])?;
                if rounded {
                    p.bezier_to(
                        [x + rx * bci, y + h],
                        [x, y + h - ry * bci],
                        [x, y + h - ry],
                    )?;
                }
                p.line_to([x, y + ry])?;
                if rounded {
                    p.bezier_to([x, y + ry * bci], [x + rx * bci, y], [x + rx, y])?;
                }
                path = Some(p);
            }
            "polygon" | "polyline" => {
                style = self.parse_style(node, &style);
                let mut p = ShapePath::new();
                for (i, pt) in point_pairs(&attr(node, "points").unwrap_or_default())
                    .into_iter()
                    .enumerate()
                {
                    if i == 0 {
                        p.move_to(pt);
                    } else {
                        p.line_to(pt)?;
                    }
                }
                if let Ok(c) = p.path() {
                    c.auto_close = node_name == "polygon";
                }
                path = Some(p);
            }
            "circle" | "ellipse" => {
                style = self.parse_style(node, &style);
                let (x, y) = (units_or(node, "cx"), units_or(node, "cy"));
                let (rx, ry) = if node_name == "circle" {
                    let r = units_or(node, "r");
                    (r, r)
                } else {
                    (units_or(node, "rx"), units_or(node, "ry"))
                };
                let mut sub = Path::default();
                sub.absellipse(Ellipse {
                    ax: x,
                    ay: y,
                    rx,
                    ry,
                    start: 0.,
                    end: PI * 2.,
                    clockwise: false,
                    rotation: 0.,
                });
                let mut p = ShapePath::new();
                p.sub_paths.push(sub);
                path = Some(p);
            }
            "line" => {
                style = self.parse_style(node, &style);
                let mut p = ShapePath::new();
                p.move_to([units_or(node, "x1"), units_or(node, "y1")]);
                p.line_to([units_or(node, "x2"), units_or(node, "y2")])?;
                p.path()?.auto_close = false;
                path = Some(p);
            }
            "defs" => defs = true,
            "use" => {
                style = self.parse_style(node, &style);
                let href = call(
                    node,
                    "getAttributeNS",
                    &["http://www.w3.org/1999/xlink".into(), "href".into()],
                )
                .as_string()
                .unwrap_or_default();
                let id = href.get(1..).unwrap_or("").to_string();
                let viewport = get(node, "viewportElement");
                let used = call(&viewport, "getElementById", &[id.into()]);
                if !used.is_undefined() && !used.is_null() {
                    self.parse_node(&used, &style)?;
                }
            }
            _ => {}
        }
        if let Some(mut p) = path {
            if let Some(fill) = &style.fill
                && fill != "none"
                && !fill.starts_with("url")
            {
                set_style(&mut p.color, fill);
            }
            transform_path(&mut p, &self.current);
            let mut path_style = style.clone();
            path_style.stroke_width = style.stroke_width * transform_scale(&self.current);
            p.style = path_style;
            p.transform = self.current;
            self.paths.push(p);
        }
        for child in list(&get(node, "childNodes")) {
            let n = name(&child);
            if defs && n != "style" && n != "defs" {
                continue;
            }
            self.parse_node(&child, &style)?;
        }
        if transform.is_some() {
            self.stack.pop();
            self.current = self.stack.last().copied().unwrap_or(Matrix3::IDENTITY);
        }
        Ok(())
    }
    fn parse_gradients(&mut self, xml: &JsValue) {
        struct Entry {
            linear: bool,
            attrs: HashMap<String, String>,
            stops: Option<Vec<(f64, String, f64)>>,
            href: Option<String>,
        }
        const ATTRS: [&str; 12] = [
            "x1",
            "y1",
            "x2",
            "y2",
            "cx",
            "cy",
            "r",
            "fx",
            "fy",
            "gradientUnits",
            "gradientTransform",
            "spreadMethod",
        ];
        let mut parsed: Vec<(String, Entry)> = vec![];
        for node in list(&call(
            xml,
            "querySelectorAll",
            &["linearGradient, radialGradient".into()],
        )) {
            let Some(id) = attr(&node, "id").filter(|v| !v.is_empty()) else {
                continue;
            };
            let href = call(
                &node,
                "getAttributeNS",
                &["http://www.w3.org/1999/xlink".into(), "href".into()],
            )
            .as_string()
            .filter(|v| !v.is_empty())
            .or_else(|| attr(&node, "href"))
            .unwrap_or_default();
            let mut attrs = HashMap::new();
            for a in ATTRS {
                if has(&node, a)
                    && let Some(v) = attr(&node, a)
                {
                    attrs.insert(a.to_string(), v);
                }
            }
            let stop_nodes = list(&call(&node, "querySelectorAll", &["stop".into()]));
            let stops = (!stop_nodes.is_empty()).then(|| {
                stop_nodes
                    .iter()
                    .map(|s| {
                        let style = get(s, "style");
                        let from_style = |k: &str| {
                            (!style.is_undefined() && !style.is_null())
                                .then(|| get(&style, k).as_string())
                                .flatten()
                        };
                        let color = attr(s, "stop-color")
                            .filter(|v| !v.is_empty())
                            .or_else(|| from_style("stop-color").filter(|v| !v.is_empty()))
                            .unwrap_or_else(|| "#000".into());
                        let opacity = attr(s, "stop-opacity")
                            .filter(|v| !v.is_empty())
                            .or_else(|| from_style("stop-opacity"));
                        let opacity = match opacity.filter(|v| !v.is_empty()) {
                            None => 1.,
                            Some(v) => js_max(0., js_min(1., parse_float(&v))),
                        };
                        let offset = js_max(
                            0.,
                            js_min(
                                1.,
                                parse_float(
                                    &attr(s, "offset")
                                        .filter(|v| !v.is_empty())
                                        .unwrap_or_else(|| "0".into()),
                                ),
                            ),
                        );
                        (offset, color, opacity)
                    })
                    .collect()
            });
            let linear = name(&node) != "radialGradient";
            let entry = Entry {
                linear,
                attrs,
                stops,
                href: href.strip_prefix('#').map(str::to_string),
            };
            if let Some(e) = parsed.iter_mut().find(|(k, _)| *k == id) {
                e.1 = entry;
            } else {
                parsed.push((id, entry));
            }
        }
        // inherit(): stops and missing attributes from the href chain.
        let ids: Vec<String> = parsed.iter().map(|(k, _)| k.clone()).collect();
        let mut done = std::collections::HashSet::new();
        fn inherit(
            parsed: &mut Vec<(String, Entry)>,
            id: &str,
            visited: &mut std::collections::HashSet<String>,
        ) {
            let Some(i) = parsed.iter().position(|(k, _)| k == id) else {
                return;
            };
            if visited.contains(id) {
                return;
            }
            visited.insert(id.to_string());
            if let Some(href) = parsed[i].1.href.clone()
                && parsed.iter().any(|(k, _)| *k == href)
            {
                inherit(parsed, &href, visited);
                let j = parsed.iter().position(|(k, _)| *k == href).unwrap_or(i);
                let (stops, attrs) = (parsed[j].1.stops.clone(), parsed[j].1.attrs.clone());
                let e = &mut parsed[i].1;
                if e.stops.is_none() {
                    e.stops = stops;
                }
                for (k, v) in attrs {
                    e.attrs.entry(k).or_insert(v);
                }
            }
        }
        for id in &ids {
            done.clear();
            inherit(&mut parsed, id, &mut done);
        }
        for (id, entry) in parsed {
            let a = &entry.attrs;
            let bbox_units = a.get("gradientUnits").map(String::as_str) != Some("userSpaceOnUse");
            let spread = match a.get("spreadMethod").map(String::as_str) {
                Some("reflect") => Wrapping::Mirror,
                Some("repeat") => Wrapping::Repeat,
                _ => Wrapping::Clamp,
            };
            let transform = a.get("gradientTransform").map(|t| {
                let mut m = Matrix3::IDENTITY;
                Self::parse_transform(t, &mut m);
                m
            });
            let mut stops = entry.stops.unwrap_or_default();
            stops.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap_or(std::cmp::Ordering::Equal));
            let coord = |k: &str, default: f64| match a.get(k) {
                None => default,
                Some(s) => match s.strip_suffix('%') {
                    Some(p) => parse_float(p) / 100.,
                    None => units(s),
                },
            };
            let p = if entry.linear {
                [
                    coord("x1", 0.),
                    coord("y1", 0.),
                    coord("x2", if bbox_units { 1. } else { 0. }),
                    coord("y2", 0.),
                    0.,
                ]
            } else {
                let d = if bbox_units { 0.5 } else { 0. };
                let (cx, cy) = (coord("cx", d), coord("cy", d));
                [cx, cy, coord("r", d), coord("fx", cx), coord("fy", cy)]
            };
            self.gradients.insert(
                id,
                Gradient {
                    linear: entry.linear,
                    bbox_units,
                    spread,
                    transform,
                    stops,
                    p,
                },
            );
        }
    }
}

fn scale_x(m: &Matrix3) -> f64 {
    let e = el(m);
    (e[0] * e[0] + e[1] * e[1]).sqrt()
}
fn scale_y(m: &Matrix3) -> f64 {
    let e = el(m);
    (e[3] * e[3] + e[4] * e[4]).sqrt()
}
fn transform_scale(m: &Matrix3) -> f64 {
    let e = el(m);
    (e[0] * e[4] - e[1] * e[3]).abs().sqrt()
}
fn flipped(m: &Matrix3) -> bool {
    let e = el(m);
    e[0] * e[4] - e[1] * e[3] < 0.
}
fn skewed(m: &Matrix3) -> bool {
    let e = el(m);
    let dot = e[0] * e[3] + e[1] * e[4];
    if dot == 0. {
        return false;
    }
    (dot / (scale_x(m) * scale_y(m))).abs() > EPSILON
}
/// eigenDecomposition( A, B, C ): ( rt1, rt2, cs, sn ).
fn eigen(a: f64, b: f64, c: f64) -> (f64, f64, f64, f64) {
    let sm = a + c;
    let df = a - c;
    let rt = (df * df + 4. * b * b).sqrt();
    let mut rt1 = f64::NAN;
    let rt2;
    if sm > 0. {
        rt1 = 0.5 * (sm + rt);
        let t = 1. / rt1;
        rt2 = a * t * c - b * t * b;
    } else if sm < 0. {
        rt2 = 0.5 * (sm - rt);
    } else {
        rt1 = 0.5 * rt;
        rt2 = -0.5 * rt;
    }
    let mut cs = if df > 0. { df + rt } else { df - rt };
    let sn;
    if cs.abs() > 2. * b.abs() {
        let t = -2. * b / cs;
        sn = 1. / (1. + t * t).sqrt();
        cs = t * sn;
    } else if b.abs() == 0. {
        cs = 1.;
        sn = 0.;
    } else {
        let t = -0.5 * cs / b;
        cs = 1. / (1. + t * t).sqrt();
        sn = t * cs;
    }
    if df > 0. {
        (rt1, rt2, -sn, cs)
    } else {
        (rt1, rt2, cs, sn)
    }
}
fn transform_path(path: &mut ShapePath, m: &Matrix3) {
    let e = el(m);
    for sub in &mut path.sub_paths {
        for curve in &mut sub.curves {
            match curve {
                Curve::Line(a, b) => {
                    *a = apply(m, *a);
                    *b = apply(m, *b);
                }
                Curve::Cubic(a, b, c, d) => {
                    *a = apply(m, *a);
                    *b = apply(m, *b);
                    *c = apply(m, *c);
                    *d = apply(m, *d);
                }
                Curve::Quadratic(a, b, c) => {
                    *a = apply(m, *a);
                    *b = apply(m, *b);
                    *c = apply(m, *c);
                }
                Curve::Ellipse(c) => {
                    [c.ax, c.ay] = apply(m, [c.ax, c.ay]);
                    if skewed(m) {
                        // transfEllipseGeneric.
                        let (ct, st) = (c.rotation.cos(), c.rotation.sin());
                        let f1 = *m * Vector3::new(c.rx * ct, c.rx * st, 0.);
                        let f2 = *m * Vector3::new(-c.ry * st, c.ry * ct, 0.);
                        let mf = m3([f1.x, f2.x, 0., f1.y, f2.y, 0., 0., 0., 1.]);
                        let inv = mf.inverse();
                        let q = inv.transpose() * inv;
                        let qe = el(&q);
                        let (rt1, rt2, cs, sn) = eigen(qe[0], qe[1], qe[4]);
                        let (r1, r2) = (rt1.sqrt(), rt2.sqrt());
                        c.rx = 1. / r1;
                        c.ry = 1. / r2;
                        c.rotation = sn.atan2(cs);
                        let full = (c.end - c.start) % (2. * PI) < EPSILON;
                        if !full {
                            let d = m3([r1, 0., 0., 0., r2, 0., 0., 0., 1.]);
                            let rt = m3([cs, sn, 0., -sn, cs, 0., 0., 0., 1.]);
                            let drf = d * rt * mf;
                            let angle = |phi: f64| {
                                let v = drf * Vector3::new(phi.cos(), phi.sin(), 0.);
                                v.y.atan2(v.x)
                            };
                            c.start = angle(c.start);
                            c.end = angle(c.end);
                            if flipped(m) {
                                c.clockwise = !c.clockwise;
                            }
                        }
                    } else {
                        // transfEllipseNoSkew.
                        let (sx, sy) = (scale_x(m), scale_y(m));
                        c.rx *= sx;
                        c.ry *= sy;
                        let theta = if sx > EPSILON {
                            e[1].atan2(e[0])
                        } else {
                            (-e[3]).atan2(e[4])
                        };
                        c.rotation += theta;
                        if flipped(m) {
                            c.start *= -1.;
                            c.end *= -1.;
                            c.clockwise = !c.clockwise;
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------- Shapes

fn area(p: &[P2]) -> f64 {
    let n = p.len();
    let mut a = 0.;
    let mut q = n.wrapping_sub(1);
    for i in 0..n {
        a += p[q][0] * p[i][1] - p[i][0] * p[q][1];
        q = i;
    }
    a * 0.5
}
fn inside(p: P2, polygon: &[P2]) -> bool {
    let mut result = false;
    let n = polygon.len();
    let mut j = n.wrapping_sub(1);
    for i in 0..n {
        let (a, b) = (polygon[i], polygon[j]);
        if (a[1] > p[1]) != (b[1] > p[1])
            && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]
        {
            result = !result;
        }
        j = i;
    }
    result
}
/// ShapePath.toShapes(): ( outline curves, holes' curves ) by fill rule.
fn to_shapes(path: &ShapePath) -> Vec<(Vec<Curve>, Vec<Vec<Curve>>)> {
    let evenodd = path.style.fill_rule.as_deref() == Some("evenodd");
    let is_inside = |w: i32| if evenodd { w & 1 != 0 } else { w != 0 };
    struct Entry {
        sub: usize,
        points: Vec<P2>,
        bounds: [f64; 4],
        interior: P2,
        abs_area: f64,
        winding: i32,
        container: Option<usize>,
        exclude: bool,
        outer: Option<bool>,
    }
    let mut entries = vec![];
    for (k, sub) in path.sub_paths.iter().enumerate() {
        let points = sub.points(12);
        if points.len() < 3 {
            continue;
        }
        let a = area(&points);
        if a == 0. {
            continue;
        }
        let mut b = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for p in &points {
            b = [
                b[0].min(p[0]),
                b[1].min(p[1]),
                b[2].max(p[0]),
                b[3].max(p[1]),
            ];
        }
        let mut interior = [(b[0] + b[2]) * 0.5, (b[1] + b[3]) * 0.5];
        if !inside(interior, &points) {
            let y = interior[1];
            let n = points.len();
            let mut xs = vec![];
            for i in 0..n {
                let (a, c) = (points[i], points[(i + 1) % n]);
                if (a[1] > y) != (c[1] > y) {
                    xs.push(a[0] + (y - a[1]) * (c[0] - a[0]) / (c[1] - a[1]));
                }
            }
            if xs.len() > 1 {
                xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                interior[0] = (xs[0] + xs[1]) / 2.;
            }
        }
        entries.push(Entry {
            sub: k,
            points,
            bounds: b,
            interior,
            abs_area: a.abs(),
            winding: if a < 0. { -1 } else { 1 },
            container: None,
            exclude: false,
            outer: None,
        });
    }
    entries.sort_by(|a, b| {
        b.abs_area
            .partial_cmp(&a.abs_area)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for i in 0..entries.len() {
        let mut container_winding = 0;
        for j in (0..i).rev() {
            let (c, e) = (&entries[j], &entries[i]);
            let contains = c.bounds[0] <= e.bounds[0]
                && e.bounds[2] <= c.bounds[2]
                && c.bounds[1] <= e.bounds[1]
                && e.bounds[3] <= c.bounds[3];
            if !contains || !inside(e.interior, &c.points) {
                continue;
            }
            let container = if c.exclude { c.container } else { Some(j) };
            container_winding = c.winding;
            entries[i].container = container;
            entries[i].winding += container_winding;
            break;
        }
        if is_inside(entries[i].winding) == is_inside(container_winding) {
            entries[i].exclude = true;
        }
    }
    for i in 0..entries.len() {
        if entries[i].exclude {
            continue;
        }
        let outer = match entries[i].container {
            None => true,
            Some(c) => entries[c].outer == Some(false),
        };
        entries[i].outer = Some(outer);
    }
    let mut shapes: Vec<(Vec<Curve>, Vec<Vec<Curve>>)> = vec![];
    let mut by_entry = HashMap::new();
    for (i, e) in entries.iter().enumerate() {
        if e.exclude || e.outer != Some(true) {
            continue;
        }
        by_entry.insert(i, shapes.len());
        shapes.push((path.sub_paths[e.sub].curves.clone(), vec![]));
    }
    for e in &entries {
        if e.exclude || e.outer != Some(false) {
            continue;
        }
        if let Some(&s) = e.container.and_then(|c| by_entry.get(&c)) {
            shapes[s].1.push(path.sub_paths[e.sub].curves.clone());
        }
    }
    shapes
}
/// ShapeGeometry( shape ): curveSegments 12, ShapeUtils winding and triangulation.
fn shape_geometry(outline: &[Curve], holes: &[Vec<Curve>]) -> Result<BufferGeometry> {
    let points = |curves: &[Curve]| {
        Path {
            curves: curves.to_vec(),
            current: [0., 0.],
            auto_close: false,
        }
        .points(12)
    };
    let mut contour = points(outline);
    if area(&contour) >= 0. {
        contour.reverse();
    }
    let mut hole_points: Vec<Vec<P2>> = holes
        .iter()
        .map(|h| {
            let mut p = points(h);
            if area(&p) < 0. {
                p.reverse();
            }
            p
        })
        .collect();
    let faces = triangulate(&mut contour, &mut hole_points);
    let mut vertices = contour;
    for h in hole_points {
        vertices.extend(h);
    }
    let positions: Vec<f32> = vertices
        .iter()
        .flat_map(|p| [p[0] as f32, p[1] as f32, 0.])
        .collect();
    let normals: Vec<f32> = vertices.iter().flat_map(|_| [0., 0., 1.]).collect();
    let uvs: Vec<f32> = vertices
        .iter()
        .flat_map(|p| [p[0] as f32, p[1] as f32])
        .collect();
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(positions, 3, false)?),
    );
    g.set_attribute(
        "normal",
        Attribute::F32(BufferAttribute::new(normals, 3, false)?),
    );
    g.set_attribute("uv", Attribute::F32(BufferAttribute::new(uvs, 2, false)?));
    g.set_index(Some(
        faces.iter().flat_map(|f| f.map(|i| i as u32)).collect(),
    ));
    Ok(g)
}

include!("svg_loader_stroke.rs");

// ---------------------------------------------------------------- Gradients

fn local_bbox(path: &ShapePath) -> Option<[f64; 4]> {
    let inv = path.transform.inverse();
    let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for sub in &path.sub_paths {
        for p in sub.points(12) {
            let p = apply(&inv, p);
            lo = [lo[0].min(p[0]), lo[1].min(p[1])];
            hi = [hi[0].max(p[0]), hi[1].max(p[1])];
        }
    }
    (hi[0] >= lo[0] && hi[1] >= lo[1]).then_some([lo[0], lo[1], hi[0] - lo[0], hi[1] - lo[1]])
}
/// buildGradientTexture: the gradient drawn into a canvas, mapped by the texture matrix.
fn gradient_texture(g: &Gradient, path: &ShapePath) -> Result<Option<Texture>> {
    if g.stops.is_empty() {
        return Ok(None);
    }
    let bbox = if g.bbox_units {
        match local_bbox(path) {
            Some(b) => Some(b),
            None => return Ok(None),
        }
    } else {
        None
    };
    let fail = |e: JsValue| Error::Asset(format!("{e:?}"));
    let resolve = |x: f64, y: f64| {
        let mut v = Vector3::new(x, y, 1.);
        if let Some(t) = &g.transform {
            v = *t * v;
        }
        if let Some(b) = bbox {
            v = Vector3::new(b[0] + v.x * b[2], b[1] + v.y * b[3], 1.);
        }
        path.transform * v
    };
    let add_stops = |grad: &web_sys::CanvasGradient| -> Result<()> {
        for (offset, color, opacity) in &g.stops {
            let mut css = color.clone();
            if *opacity < 1. {
                let mut c = Color::WHITE;
                set_style(&mut c, color);
                let srgb = c.0.map(crate::math::linear_to_srgb);
                let rgb = [srgb.x, srgb.y, srgb.z].map(|v| (v * 255.).round() as i32);
                css = format!("rgba({},{},{},{})", rgb[0], rgb[1], rgb[2], opacity);
            }
            grad.add_color_stop(offset.clamp(0., 1.) as f32, &css)
                .map_err(fail)?;
        }
        Ok(())
    };
    let (canvas, matrix) = if g.linear {
        let canvas = web_sys::OffscreenCanvas::new(256, 1).map_err(fail)?;
        let ctx: web_sys::OffscreenCanvasRenderingContext2d = canvas
            .get_context("2d")
            .map_err(fail)?
            .ok_or(Error::Invalid("2d"))?
            .dyn_into()
            .map_err(|_| Error::Invalid("2d"))?;
        let grad = ctx.create_linear_gradient(0., 0., 256., 0.);
        add_stops(&grad)?;
        ctx.set_fill_style_canvas_gradient(&grad);
        ctx.fill_rect(0., 0., 256., 1.);
        let (p1, p2) = (resolve(g.p[0], g.p[1]), resolve(g.p[2], g.p[3]));
        let (dx, dy) = (p2.x - p1.x, p2.y - p1.y);
        let mut len2 = dx * dx + dy * dy;
        if len2 == 0. {
            len2 = 1e-20;
        }
        let (a, b) = (dx / len2, dy / len2);
        let c = -(a * p1.x + b * p1.y);
        (canvas, m3([a, b, c, 0., 0., 0.5, 0., 0., 1.]))
    } else {
        let [mut cx, mut cy, mut r, mut fx, mut fy] = g.p;
        if let Some(t) = &g.transform {
            let v = *t * Vector3::new(cx, cy, 1.);
            (cx, cy) = (v.x, v.y);
            let v = *t * Vector3::new(fx, fy, 1.);
            (fx, fy) = (v.x, v.y);
        }
        if let Some(b) = bbox {
            cx = b[0] + cx * b[2];
            cy = b[1] + cy * b[3];
            fx = b[0] + fx * b[2];
            fy = b[1] + fy * b[3];
            r *= ((b[2] * b[2] + b[3] * b[3]) / 2.).sqrt();
        }
        if r <= 0. {
            return Ok(None);
        }
        let canvas = web_sys::OffscreenCanvas::new(256, 256).map_err(fail)?;
        let ctx: web_sys::OffscreenCanvasRenderingContext2d = canvas
            .get_context("2d")
            .map_err(fail)?
            .ok_or(Error::Invalid("2d"))?
            .dyn_into()
            .map_err(|_| Error::Invalid("2d"))?;
        let (min_x, min_y, span) = (cx - r, cy - r, 2. * r);
        let scale = 256. / span;
        ctx.set_transform(scale, 0., 0., scale, -min_x * scale, -min_y * scale)
            .map_err(fail)?;
        let grad = ctx
            .create_radial_gradient(fx, fy, 0., cx, cy, r)
            .map_err(fail)?;
        add_stops(&grad)?;
        ctx.set_fill_style_canvas_gradient(&grad);
        ctx.fill_rect(min_x, min_y, span, span);
        let norm = m3([
            1. / span,
            0.,
            -min_x / span,
            0.,
            1. / span,
            -min_y / span,
            0.,
            0.,
            1.,
        ]);
        (canvas, norm * path.transform.inverse())
    };
    let ctx: web_sys::OffscreenCanvasRenderingContext2d = canvas
        .get_context("2d")
        .map_err(fail)?
        .ok_or(Error::Invalid("2d"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("2d"))?;
    let (w, h) = (canvas.width(), canvas.height());
    let data = ctx
        .get_image_data(0., 0., f64::from(w), f64::from(h))
        .map_err(fail)?
        .data()
        .0;
    let mut texture = Texture::from_rgba(w, h, data, true)?;
    texture.flip_y = false;
    texture.matrix = Some(matrix);
    texture.wrap_s = g.spread;
    texture.wrap_t = g.spread;
    texture.mipmap_filter = Some(Filter::Linear);
    Ok(Some(texture))
}

// ---------------------------------------------------------------- Example

pub(super) struct Demo {
    controls: Controls,
    file: usize,
    draw_strokes: bool,
    draw_fills: bool,
    strokes_wireframe: bool,
    fills_wireframe: bool,
    group: Option<Object3D>,
    /// The 35 files' text, fetched with the page.
    texts: Vec<String>,
    dirty: bool,
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
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 200.);
        s.background = Color::from_hex(0xb0b0b0);
        let grid = s.insert(NodeKind::Line(grid_helper(160., 10, 0x8d8d8d, 0xc1c1c1)?));
        s.get_mut(grid)?.quaternion = Quaternion::from_rotation_x(PI / 2.);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        let mut texts = Vec::with_capacity(FILES.len());
        for file in FILES {
            texts.push(
                String::from_utf8_lossy(&fetch(&format!("/web/gallery/assets/svg/{file}")).await?)
                    .to_string(),
            );
        }
        let mut demo = Self {
            controls,
            file: 0,
            draw_strokes: true,
            draw_fills: true,
            strokes_wireframe: false,
            fills_wireframe: false,
            group: None,
            texts,
            dirty: true,
        };
        demo.build(s)?;
        Ok(demo)
    }
    /// loadSVG( url )'s onLoad: the group of fill and stroke meshes.
    fn build(&mut self, s: &mut Scene) -> Result<()> {
        let text = &self.texts[self.file];
        self.dirty = false;
        if let Some(g) = self.group.take() {
            s.dispose(g)?;
        }
        let window = web_sys::window().ok_or(Error::Invalid("window"))?;
        let ctor: Function = get(&window, "DOMParser")
            .dyn_into()
            .map_err(|_| Error::Invalid("DOMParser"))?;
        let parser =
            Reflect::construct(&ctor, &Array::new()).map_err(|e| Error::Asset(format!("{e:?}")))?;
        let xml = call(
            &parser,
            "parseFromString",
            &[text.as_str().into(), "image/svg+xml".into()],
        );
        let mut p = Parser {
            paths: vec![],
            stylesheets: HashMap::new(),
            gradients: HashMap::new(),
            stack: vec![],
            current: Matrix3::IDENTITY,
        };
        p.parse_gradients(&xml);
        p.parse_node(&get(&xml, "documentElement"), &Style::default())?;
        let group = s.insert(NodeKind::Group);
        let n = s.get_mut(group)?;
        n.scale = Vector3::new(0.25, -0.25, 0.25);
        n.position = Vector3::new(-70., 70., 0.);
        let mut order = 0;
        let mut add =
            |s: &mut Scene, geometry: BufferGeometry, material: &Arc<Material>| -> Result<()> {
                let h = s.insert(NodeKind::Mesh(Mesh {
                    geometry: Arc::new(geometry),
                    materials: vec![material.clone()],
                }));
                s.get_mut(h)?.render_order = order;
                order += 1;
                s.add(group, h)
            };
        for path in &p.paths {
            if self.draw_fills
                && let Some(fill) = &path.style.fill
                && fill != "none"
            {
                // createFillMaterial: the gradient texture, or the path colour.
                let mut texture = None;
                if let Some(id) = gradient_url(fill)
                    && let Some(g) = p.gradients.get(&id)
                {
                    texture = gradient_texture(g, path)?;
                }
                let mut m = MeshBasicMaterial::default();
                m.properties.opacity = path.style.fill_opacity
                    * path
                        .style
                        .opacity
                        .filter(|o| *o != 0. && !o.is_nan())
                        .unwrap_or(1.);
                m.properties.transparent = true;
                m.properties.side = Side::Double;
                m.properties.depth_write = false;
                m.properties.wireframe = self.fills_wireframe;
                match texture {
                    Some(t) => m.properties.map = Some(Arc::new(t)),
                    None => m.properties.color = path.color,
                }
                let material = Arc::new(Material::Basic(m));
                for (outline, holes) in to_shapes(path) {
                    add(s, shape_geometry(&outline, &holes)?, &material)?;
                }
            }
            if self.draw_strokes
                && let Some(stroke) = &path.style.stroke
                && stroke != "none"
            {
                let mut color = Color::WHITE;
                set_style(&mut color, stroke);
                let mut m = MeshBasicMaterial::default();
                m.properties.color = color;
                m.properties.opacity = path.style.stroke_opacity
                    * path
                        .style
                        .opacity
                        .filter(|o| *o != 0. && !o.is_nan())
                        .unwrap_or(1.);
                m.properties.transparent = true;
                m.properties.side = Side::Double;
                m.properties.depth_write = false;
                m.properties.wireframe = self.strokes_wireframe;
                let material = Arc::new(Material::Basic(m));
                for sub in &path.sub_paths {
                    if let Some(g) = points_to_stroke(&sub.points(12), &path.style, 12, 0.001)? {
                        add(s, g, &material)?;
                    }
                }
            }
        }
        self.group = Some(group);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if self.dirty {
            self.build(s)?;
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
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let on = value != 0.;
        match index {
            0 => self.file = (value as usize).min(FILES.len() - 1),
            1 => self.draw_strokes = on,
            2 => self.draw_fills = on,
            3 => self.strokes_wireframe = on,
            4 => self.fills_wireframe = on,
            _ => return Err(Error::Invalid("svg parameter")),
        }
        self.dirty = true;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}

/// GRADIENT_URL_RE: url( #id ) with optional quotes.
fn gradient_url(fill: &str) -> Option<String> {
    let s = fill.trim();
    let inner = s.strip_prefix("url(")?.strip_suffix(')')?.trim();
    let inner = inner.trim_matches(|c| c == '"' || c == '\'').trim();
    let id = inner.strip_prefix('#')?;
    (!id.is_empty() && !id.contains([')', '\'', '"']) && !id.contains(char::is_whitespace))
        .then(|| id.to_string())
}
