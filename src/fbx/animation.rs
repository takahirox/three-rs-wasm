//! FBXLoader's AnimationParser: animation stacks, layers and curve nodes into
//! position, quaternion, scale and morph-influence keyframe tracks.
use super::js::{Obj, Value, number_key, parse_int};
use super::math::{Euler, M4, Order, Q, deg_to_rad, fround};
use super::scene::{Ctx, Graph, bad, map_key, model_name, numbers, prop_value};
use super::{FbxClip, FbxTrack, FbxTrackKind};
use crate::Result;
use std::collections::HashMap;

/// An AnimationCurve: seconds and values.
#[derive(Clone, Debug)]
struct Curve {
    times: Vec<f64>,
    values: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
struct Curves {
    x: Option<Curve>,
    y: Option<Curve>,
    z: Option<Curve>,
    morph: Option<Curve>,
}
impl Curves {
    fn len(&self) -> usize {
        [&self.x, &self.y, &self.z, &self.morph]
            .iter()
            .filter(|c| c.is_some())
            .count()
    }
}

#[derive(Clone, Debug)]
struct CurveNode {
    attr: String,
    curves: Curves,
}

/// One entry of a layer: the model's transform curve nodes, or a morph.
#[derive(Clone, Debug, Default)]
struct RawTracks {
    model_name: String,
    transform: Option<M4>,
    euler_order: Option<Order>,
    initial_rotation: Vec<f64>,
    pre_rotation: Option<Vec<f64>>,
    post_rotation: Option<Vec<f64>>,
    morph_name: Option<Value>,
    /// The curve nodes by attribute ( `T`, `R`, `S`, `DeformPercent`, … ).
    nodes: HashMap<String, CurveNode>,
}

struct RawClip {
    name: String,
    layer: Option<Vec<Option<RawTracks>>>,
    local_start: f64,
    local_stop: f64,
}

/// convertFBXTimeToSeconds().
fn seconds(t: f64) -> f64 {
    t / 46186158000.0
}

/// AnimationParser.parse().
pub(super) fn parse(ctx: &Ctx, graph: &Graph, root: usize, trim: bool) -> Result<Vec<FbxClip>> {
    if ctx.objects("AnimationCurve").is_undefined() {
        return Ok(vec![]);
    }
    // parseAnimationCurveNodes()
    let mut curve_nodes: HashMap<String, CurveNode> = HashMap::new();
    for (_, raw) in ctx.nodes("AnimationCurveNode") {
        let attr = raw
            .get("attrName")
            .as_str()
            .ok_or_else(|| bad("rawCurveNode.attrName.match is not a function"))?
            .to_string();
        if attr.contains(['S', 'R', 'T']) {
            curve_nodes.insert(
                map_key(&raw.get("id")),
                CurveNode {
                    attr,
                    curves: Curves::default(),
                },
            );
        }
    }
    // parseAnimationCurves()
    for (_, raw) in ctx.nodes("AnimationCurve") {
        let id = raw.get("id");
        let times = raw
            .get("KeyTime")
            .get("a")
            .to_f64s()
            .ok_or_else(|| bad("KeyTime.a is undefined"))?
            .into_iter()
            .map(seconds)
            .collect();
        let values = raw
            .get("KeyValueFloat")
            .get("a")
            .to_f64s()
            .ok_or_else(|| bad("KeyValueFloat.a is undefined"))?;
        let curve = Curve { times, values };
        let Some(links) = ctx.connections.get(&id) else {
            continue;
        };
        let parent = links
            .parents
            .first()
            .ok_or_else(|| bad("animation curve without a curve node"))?;
        let relationship = parent
            .relationship
            .as_str()
            .ok_or_else(|| bad("animationCurveRelationship.match is not a function"))?;
        let key = map_key(&parent.id);
        let missing = || bad("Cannot read properties of undefined (reading 'curves')");
        if relationship.contains('X') {
            curve_nodes.get_mut(&key).ok_or_else(missing)?.curves.x = Some(curve);
        } else if relationship.contains('Y') {
            curve_nodes.get_mut(&key).ok_or_else(missing)?.curves.y = Some(curve);
        } else if relationship.contains('Z') {
            curve_nodes.get_mut(&key).ok_or_else(missing)?.curves.z = Some(curve);
        } else if relationship.contains("DeformPercent")
            && let Some(n) = curve_nodes.get_mut(&key)
        {
            n.curves.morph = Some(curve);
        }
    }
    // parseAnimationLayers()
    let models = ctx.objects("Model");
    let deformer_nodes = ctx.objects("Deformer");
    let mut layers: HashMap<String, Vec<Option<RawTracks>>> = HashMap::new();
    for (node_id, _) in ctx.nodes("AnimationLayer") {
        let id = Value::Number(parse_int(&node_id));
        let Some(connection) = ctx.connections.get(&id) else {
            continue;
        };
        let mut entries: Vec<Option<RawTracks>> = vec![None; connection.children.len()];
        for (i, child) in connection.children.iter().enumerate() {
            let Some(curve_node) = curve_nodes.get(&map_key(&child.id)) else {
                continue;
            };
            let c = &curve_node.curves;
            let filtered = || -> Result<Vec<Value>> {
                Ok(ctx
                    .connections
                    .links(&child.id)?
                    .parents
                    .iter()
                    .filter(|p| !p.relationship.is_undefined())
                    .map(|p| p.id.clone())
                    .collect())
            };
            if c.x.is_some() || c.y.is_some() || c.z.is_some() {
                if entries[i].is_none() {
                    let parents = filtered()?;
                    let Some(model_id) = parents.first() else {
                        continue;
                    };
                    if !model_id.is_undefined() {
                        let raw_model = models.get(&model_id.to_key());
                        if raw_model.is_undefined() {
                            continue;
                        }
                        let raw_id = raw_model.get("id");
                        let mut node = RawTracks {
                            model_name: model_name(&raw_model.get("attrName")),
                            initial_rotation: vec![0.0, 0.0, 0.0],
                            ..Default::default()
                        };
                        for n in graph.traverse(root) {
                            let g = &graph.nodes[n];
                            if g.id.is_some_and(|v| Value::Number(v).strict_eq(&raw_id)) {
                                node.transform = Some(g.matrix);
                                if let Some(t) = &g.transform_data {
                                    node.euler_order = t.euler_order;
                                    if let Some(r) = &t.rotation {
                                        node.initial_rotation = r.clone();
                                    }
                                }
                            }
                        }
                        if node.transform.is_none() {
                            node.transform = Some(M4::IDENTITY);
                        }
                        if raw_model.has("PreRotation") {
                            node.pre_rotation =
                                Some(numbers(&prop_value(&raw_model, "PreRotation")));
                        }
                        if raw_model.has("PostRotation") {
                            node.post_rotation =
                                Some(numbers(&prop_value(&raw_model, "PostRotation")));
                        }
                        entries[i] = Some(node);
                    }
                }
                if let Some(e) = &mut entries[i] {
                    e.nodes.insert(curve_node.attr.clone(), curve_node.clone());
                }
            } else if c.morph.is_some() {
                if entries[i].is_none() {
                    let parents = filtered()?;
                    let Some(deformer_id) = parents.first() else {
                        continue;
                    };
                    let first_parent = |id: &Value| -> Result<Value> {
                        ctx.connections
                            .links(id)?
                            .parents
                            .first()
                            .map(|p| p.id.clone())
                            .ok_or_else(|| bad("missing morph connection"))
                    };
                    let morpher = first_parent(deformer_id)?;
                    let geo = first_parent(&morpher)?;
                    let model_id = first_parent(&geo)?;
                    let raw_model = models.get(&model_id.to_key());
                    if raw_model.is_undefined() {
                        return Err(bad(
                            "Cannot read properties of undefined (reading 'attrName')",
                        ));
                    }
                    entries[i] = Some(RawTracks {
                        model_name: model_name(&raw_model.get("attrName")),
                        morph_name: Some(deformer_nodes.get(&deformer_id.to_key()).get("attrName")),
                        ..Default::default()
                    });
                }
                if let Some(e) = &mut entries[i] {
                    e.nodes.insert(curve_node.attr.clone(), curve_node.clone());
                }
            }
        }
        layers.insert(map_key(&id), entries);
    }
    // parseAnimStacks()
    let stacks = Obj::new();
    let mut raw_clips: Vec<RawClip> = vec![];
    for (node_id, stack) in ctx.nodes("AnimationStack") {
        let children = &ctx
            .connections
            .links(&Value::Number(parse_int(&node_id)))?
            .children;
        let first = children
            .first()
            .ok_or_else(|| bad("animation stack without a layer"))?;
        let local = |name: &str| {
            let v = stack.get(name);
            if v.is_undefined() {
                0.0
            } else {
                seconds(v.get("value").to_number())
            }
        };
        let name = stack.get("attrName");
        stacks.set(&node_id, Value::Number(raw_clips.len() as f64));
        raw_clips.push(RawClip {
            name: if name.is_undefined() {
                String::new()
            } else {
                name.to_key()
            },
            layer: layers.get(&map_key(&first.id)).cloned(),
            local_start: local("LocalStart"),
            local_stop: local("LocalStop"),
        });
    }
    let mut clips = vec![];
    for key in stacks.keys() {
        let raw = &raw_clips[stacks.get(&key).to_number() as usize];
        clips.push(add_clip(graph, root, raw, trim)?);
    }
    Ok(clips)
}

/// addClip().
fn add_clip(graph: &Graph, root: usize, raw: &RawClip, trim: bool) -> Result<FbxClip> {
    let layer = raw
        .layer
        .as_ref()
        .ok_or_else(|| bad("rawClip.layer is undefined"))?;
    let mut tracks = vec![];
    for entry in layer.iter().flatten() {
        tracks.extend(generate_tracks(graph, root, entry)?);
    }
    if trim && raw.local_stop > raw.local_start {
        tracks = trim_tracks(tracks, raw.local_start, raw.local_stop);
    }
    // AnimationClip( name, -1, tracks ).resetDuration()
    let mut duration = 0.0f64;
    for t in &tracks {
        let last = t.times.last().map_or(f64::NAN, |&v| f64::from(v));
        duration = js_max(duration, last);
    }
    Ok(FbxClip {
        name: raw.name.clone(),
        duration,
        tracks,
    })
}

/// `Math.max( a, b )`.
fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == b {
        if a.is_sign_negative() { b } else { a }
    } else {
        a.max(b)
    }
}

/// trimTracks(): keyframes within [ start, end ], shifted to start at zero.
fn trim_tracks(tracks: Vec<FbxTrack>, start: f64, end: f64) -> Vec<FbxTrack> {
    let start = fround(start);
    let end = fround(end);
    let mut out = vec![];
    for mut t in tracks {
        let stride = t.values.len() as f64 / t.times.len() as f64;
        let mut times = vec![];
        let mut values = vec![];
        for (j, &time) in t.times.iter().enumerate() {
            let time = f64::from(time);
            if time < start || time > end {
                continue;
            }
            times.push((time - start) as f32);
            let mut k = 0.0;
            while k < stride {
                let i = j as f64 * stride + k;
                values.push(if i.fract() == 0.0 {
                    t.values.get(i as usize).copied().unwrap_or(f32::NAN)
                } else {
                    f32::NAN
                });
                k += 1.0;
            }
        }
        if times.is_empty() {
            continue;
        }
        t.times = times;
        t.values = values;
        out.push(t);
    }
    out
}

/// A KeyframeTrack: Float32 times and values; no keyframes throws.
fn track(name: String, kind: FbxTrackKind, times: &[f64], values: &[f64]) -> Result<FbxTrack> {
    if times.is_empty() {
        return Err(bad(&format!(
            "THREE.KeyframeTrack: no keyframes in track named {name}"
        )));
    }
    Ok(FbxTrack {
        name,
        kind,
        times: times.iter().map(|&v| v as f32).collect(),
        values: values.iter().map(|&v| v as f32).collect(),
    })
}

fn generate_tracks(graph: &Graph, root: usize, raw: &RawTracks) -> Result<Vec<FbxTrack>> {
    let mut tracks = vec![];
    let (initial_position, initial_scale) = match &raw.transform {
        Some(t) => {
            let (p, _, s) = t.decompose();
            (p.to_array().to_vec(), s.to_array().to_vec())
        }
        None => (vec![0.0; 3], vec![0.0; 3]),
    };
    if let Some(t) = raw.nodes.get("T")
        && t.curves.len() > 0
    {
        tracks.push(vector_track(
            &raw.model_name,
            &t.curves,
            initial_position,
            "position",
        )?);
    }
    if let Some(r) = raw.nodes.get("R")
        && r.curves.len() > 0
        && let Some(t) = rotation_track(raw, &r.curves)?
    {
        tracks.push(t);
    }
    if let Some(s) = raw.nodes.get("S")
        && s.curves.len() > 0
    {
        tracks.push(vector_track(
            &raw.model_name,
            &s.curves,
            initial_scale,
            "scale",
        )?);
    }
    if let Some(d) = raw.nodes.get("DeformPercent") {
        let curve = d
            .curves
            .morph
            .as_ref()
            .ok_or_else(|| bad("Cannot read properties of undefined (reading 'values')"))?;
        let values: Vec<f64> = curve.values.iter().map(|v| v / 100.0).collect();
        let mesh = graph.object_by_name(root, &raw.model_name).ok_or_else(|| {
            bad("Cannot read properties of null (reading 'morphTargetDictionary')")
        })?;
        let dictionary = match &graph.nodes[mesh].kind {
            super::scene::Kind::Mesh { morph_targets, .. } => morph_targets,
            _ => {
                return Err(bad(
                    "Cannot read properties of undefined (reading 'morphTargetDictionary')",
                ));
            }
        };
        let morph = raw.morph_name.clone().unwrap_or_default().to_key();
        let num = dictionary
            .iter()
            .find(|(k, _)| *k == morph)
            .map_or("undefined".to_string(), |(_, v)| v.to_string());
        tracks.push(track(
            format!("{}.morphTargetInfluences[{num}]", raw.model_name),
            FbxTrackKind::Number,
            &curve.times,
            &values,
        )?);
    }
    Ok(tracks)
}

/// getTimesForAllAxes(): the axes' times sorted without duplicates.
fn all_times(c: &Curves) -> Vec<f64> {
    let mut times = vec![];
    for axis in [&c.x, &c.y, &c.z].into_iter().flatten() {
        times.extend_from_slice(&axis.times);
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if times.len() > 1 {
        let mut target = 1;
        let mut last = times[0];
        for i in 1..times.len() {
            let current = times[i];
            if current != last {
                times[target] = current;
                last = current;
                target += 1;
            }
        }
        times.truncate(target);
    }
    times
}

fn vector_track(name: &str, c: &Curves, initial: Vec<f64>, kind: &str) -> Result<FbxTrack> {
    let times = all_times(c);
    // getKeyframeTrackValues()
    let mut prev = initial;
    let mut values = vec![];
    let (mut xi, mut yi, mut zi): (Option<usize>, Option<usize>, Option<usize>) =
        (None, None, None);
    let index = |curve: &Option<Curve>, t: f64| {
        curve.as_ref().map(|c| c.times.iter().position(|&v| v == t))
    };
    for &time in &times {
        if let Some(i) = index(&c.x, time) {
            xi = i;
        }
        if let Some(i) = index(&c.y, time) {
            yi = i;
        }
        if let Some(i) = index(&c.z, time) {
            zi = i;
        }
        for (k, (axis, i)) in [(&c.x, xi), (&c.y, yi), (&c.z, zi)].into_iter().enumerate() {
            match (axis, i) {
                (Some(curve), Some(i)) => {
                    let v = curve.values.get(i).copied().unwrap_or(f64::NAN);
                    values.push(v);
                    prev[k] = v;
                }
                _ => values.push(prev[k]),
            }
        }
    }
    track(
        format!("{name}.{kind}"),
        FbxTrackKind::Vector,
        &times,
        &values,
    )
}

/// synchronizeCurve(): the curve at the merged times ( or itself when the
/// key counts already match ).
fn synchronize(curve: &Option<Curve>, target: &[f64], initial: f64) -> Curve {
    let Some(curve) = curve else {
        return Curve {
            times: target.to_vec(),
            values: vec![initial; target.len()],
        };
    };
    if curve.times.len() == target.len() {
        return curve.clone();
    }
    Curve {
        times: target.to_vec(),
        values: target.iter().map(|&t| sample(curve, t, initial)).collect(),
    }
}

/// sampleCurveValue(): linear interpolation of the curve at a time.
fn sample(c: &Curve, time: f64, initial: f64) -> f64 {
    let (times, values) = (&c.times, &c.values);
    let v = |i: usize| values.get(i).copied().unwrap_or(f64::NAN);
    let first = times.first().copied().unwrap_or(f64::NAN);
    if time <= first {
        return v(0);
    }
    let last = times.last().copied().unwrap_or(f64::NAN);
    if time >= last {
        return values.last().copied().unwrap_or(f64::NAN);
    }
    for i in 0..times.len().saturating_sub(1) {
        if time >= times[i] && time <= times[i + 1] {
            if times[i] == time {
                return v(i);
            }
            let alpha = (time - times[i]) / (times[i + 1] - times[i]);
            return v(i) * (1.0 - alpha) + v(i + 1) * alpha;
        }
    }
    initial
}

/// interpolateRotations(): radians, with spans of 180° or more slerped.
fn interpolate_rotations(
    x: &Curve,
    y: &Curve,
    z: &Curve,
    order: Option<Order>,
) -> (Vec<f64>, Vec<f64>) {
    let order = order.unwrap_or(Order::Xyz);
    let mut times = vec![];
    let mut values = vec![];
    let g = |c: &Curve, i: usize| c.values.get(i).copied().unwrap_or(f64::NAN);
    times.push(x.times.first().copied().unwrap_or(f64::NAN));
    values.push(deg_to_rad(g(x, 0)));
    values.push(deg_to_rad(g(y, 0)));
    values.push(deg_to_rad(g(z, 0)));
    for i in 1..x.values.len() {
        let initial = [g(x, i - 1), g(y, i - 1), g(z, i - 1)];
        if initial.iter().any(|v| v.is_nan()) {
            continue;
        }
        let initial_rad = initial.map(deg_to_rad);
        let current = [g(x, i), g(y, i), g(z, i)];
        if current.iter().any(|v| v.is_nan()) {
            continue;
        }
        let current_rad = current.map(deg_to_rad);
        let span = [
            (current[0] - initial[0]).abs(),
            (current[1] - initial[1]).abs(),
            (current[2] - initial[2]).abs(),
        ];
        let time = |k: usize| x.times.get(k).copied().unwrap_or(f64::NAN);
        if span[0] >= 180.0 || span[1] >= 180.0 || span[2] >= 180.0 {
            let max = span[0].max(span[1]).max(span[2]);
            let sub = max / 180.0;
            let q1 = Q::from_euler(Euler::new(
                initial_rad[0],
                initial_rad[1],
                initial_rad[2],
                order,
            ));
            let mut q2 = Q::from_euler(Euler::new(
                current_rad[0],
                current_rad[1],
                current_rad[2],
                order,
            ));
            if q1.dot(q2) < 0.0 {
                q2 = q2.negate();
            }
            let initial_time = time(i - 1);
            let time_span = time(i) - initial_time;
            let mut t = 0.0;
            while t < 1.0 {
                let q = q1.slerp(q2, t);
                times.push(initial_time + t * time_span);
                let e = Euler::from_quaternion(q, order);
                values.extend([e.x, e.y, e.z]);
                t += 1.0 / sub;
            }
        } else {
            times.push(time(i));
            values.extend(current_rad);
        }
    }
    (times, values)
}

fn rotation_track(raw: &RawTracks, c: &Curves) -> Result<Option<FbxTrack>> {
    let mut times = None;
    let mut values = None;
    if c.x.is_some() || c.y.is_some() || c.z.is_some() {
        let merged = all_times(c);
        if !merged.is_empty() {
            let r = &raw.initial_rotation;
            let g = |i: usize| r.get(i).copied().unwrap_or(f64::NAN);
            let sx = synchronize(&c.x, &merged, g(0));
            let sy = synchronize(&c.y, &merged, g(1));
            let sz = synchronize(&c.z, &merged, g(2));
            let (t, v) = interpolate_rotations(&sx, &sy, &sz, raw.euler_order);
            times = Some(t);
            values = Some(v);
        }
    }
    let euler = |v: &[f64]| {
        let g = |i: usize| v.get(i).copied().map_or(f64::NAN, deg_to_rad);
        Q::from_euler(Euler::new(g(0), g(1), g(2), Order::Zyx))
    };
    let pre = raw.pre_rotation.as_deref().map(euler);
    let post = raw.post_rotation.as_deref().map(|v| euler(v).invert());
    let (Some(times), Some(values)) = (times, values) else {
        return Ok(None);
    };
    let order = raw.euler_order.unwrap_or(Order::Xyz);
    let mut out: Vec<f64> = vec![];
    let mut i = 0;
    while i < values.len() {
        let g = |k: usize| values.get(k).copied().unwrap_or(f64::NAN);
        let mut q = Q::from_euler(Euler::new(g(i), g(i + 1), g(i + 2), order));
        if let Some(p) = pre {
            q = Q::multiply(p, q);
        }
        if let Some(p) = post {
            q = Q::multiply(q, p);
        }
        if i > 2 {
            let at = (i - 3) / 3 * 4;
            let prev = Q {
                x: out[at],
                y: out[at + 1],
                z: out[at + 2],
                w: out[at + 3],
            };
            if prev.dot(q) < 0.0 {
                q = q.negate();
            }
        }
        out.extend(q.to_array());
        i += 3;
    }
    Ok(Some(track(
        format!("{}.quaternion", raw.model_name),
        FbxTrackKind::Quaternion,
        &times,
        &out,
    )?))
}

#[allow(dead_code)]
fn unused(_: &str) -> String {
    number_key(0.0)
}
