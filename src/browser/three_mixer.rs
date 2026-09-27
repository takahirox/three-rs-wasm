//! Three.js AnimationMixer semantics for the blending examples: actions with
//! weight and time-scale interpolants, cross-fades with warping, loop events,
//! activation order, and PropertyMixer's normal and additive accumulation.
use crate::animation::{Clip, Property};
use crate::{Error, Result, math::*, scene::*};
use std::sync::Arc;

/// A two-key control interpolant: ( t0, t1, v0, v1 ), clamped outside.
fn control(i: &[f64; 4], t: f64) -> f64 {
    if t <= i[0] {
        i[2]
    } else if t >= i[1] {
        i[3]
    } else {
        i[2] + (i[3] - i[2]) * (t - i[0]) / (i[1] - i[0])
    }
}
/// `Quaternion.slerpFlat`.
fn slerp_flat(a: &[f64], b: &[f64], t: f64) -> [f64; 4] {
    let [mut x0, mut y0, mut z0, mut w0] = [a[0], a[1], a[2], a[3]];
    let [mut x1, mut y1, mut z1, mut w1] = [b[0], b[1], b[2], b[3]];
    if t <= 0. {
        return [x0, y0, z0, w0];
    }
    if t >= 1. {
        return [x1, y1, z1, w1];
    }
    if w0 != w1 || x0 != x1 || y0 != y1 || z0 != z1 {
        let mut dot = x0 * x1 + y0 * y1 + z0 * z1 + w0 * w1;
        if dot < 0. {
            (x1, y1, z1, w1) = (-x1, -y1, -z1, -w1);
            dot = -dot;
        }
        let (mut s, mut t) = (1. - t, t);
        if dot < 0.9995 {
            let theta = dot.acos();
            let sin = theta.sin();
            s = (s * theta).sin() / sin;
            t = (t * theta).sin() / sin;
            (x0, y0, z0, w0) = (
                x0 * s + x1 * t,
                y0 * s + y1 * t,
                z0 * s + z1 * t,
                w0 * s + w1 * t,
            );
        } else {
            (x0, y0, z0, w0) = (
                x0 * s + x1 * t,
                y0 * s + y1 * t,
                z0 * s + z1 * t,
                w0 * s + w1 * t,
            );
            let f = 1. / (x0 * x0 + y0 * y0 + z0 * z0 + w0 * w0).sqrt();
            (x0, y0, z0, w0) = (x0 * f, y0 * f, z0 * f, w0 * f);
        }
    }
    [x0, y0, z0, w0]
}
/// `Quaternion.slerp` through `slerpFlat`.
pub(super) fn slerp_quaternion(a: Quaternion, b: Quaternion, t: f64) -> [f64; 4] {
    slerp_flat(&a.to_array(), &b.to_array(), t)
}
/// `Quaternion.multiplyQuaternionsFlat`: a × b.
fn multiply_flat(a: &[f64], b: &[f64]) -> [f64; 4] {
    let (x0, y0, z0, w0) = (a[0], a[1], a[2], a[3]);
    let (x1, y1, z1, w1) = (b[0], b[1], b[2], b[3]);
    [
        x0 * w1 + w0 * x1 + y0 * z1 - z0 * y1,
        y0 * w1 + w0 * y1 + z0 * x1 - x0 * z1,
        z0 * w1 + w0 * z1 + x0 * y1 - y0 * x1,
        w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
    ]
}
fn read(scene: &Scene, target: Object3D, property: Property) -> Result<Vec<f64>> {
    let node = scene.get(target)?;
    Ok(match property {
        Property::Position => node.position.to_array().to_vec(),
        Property::Rotation => node.quaternion.to_array().to_vec(),
        Property::Scale => node.scale.to_array().to_vec(),
        Property::Weights => node.morph_weights.clone(),
    })
}
fn write(scene: &mut Scene, target: Object3D, property: Property, v: &[f64]) -> Result<()> {
    let node = scene.get_mut(target)?;
    match property {
        Property::Position => node.position = Vector3::new(v[0], v[1], v[2]),
        Property::Rotation => node.quaternion = Quaternion::from_xyzw(v[0], v[1], v[2], v[3]),
        Property::Scale => node.scale = Vector3::new(v[0], v[1], v[2]),
        Property::Weights => node.morph_weights = v.to_vec(),
    }
    node.matrix_auto_update = true;
    node.matrix_world_needs_update = true;
    Ok(())
}
/// PropertyMixer: the two accumulation buffers, the additive buffer and the
/// original state saved when the first action using it activates.
struct Binding {
    target: Object3D,
    property: Property,
    original: Vec<f64>,
    accu: [Vec<f64>; 2],
    add: Vec<f64>,
    weight: f64,
    weight_additive: f64,
    uses: u32,
}
impl Binding {
    fn quaternion(&self) -> bool {
        self.property == Property::Rotation
    }
    fn mix(&self, dst: &[f64], src: &[f64], t: f64) -> Vec<f64> {
        if self.quaternion() {
            slerp_flat(dst, src, t).to_vec()
        } else {
            dst.iter()
                .zip(src)
                .map(|(d, s)| d * (1. - t) + s * t)
                .collect()
        }
    }
    fn mix_additive(&self, dst: &[f64], src: &[f64], t: f64) -> Vec<f64> {
        if self.quaternion() {
            let work = multiply_flat(dst, src);
            slerp_flat(dst, &work, t).to_vec()
        } else {
            dst.iter().zip(src).map(|(d, s)| d + s * t).collect()
        }
    }
    fn accumulate(&mut self, accu: usize, incoming: &[f64], weight: f64) {
        if self.weight == 0. {
            self.accu[accu] = incoming.to_vec();
            self.weight = weight;
        } else {
            self.weight += weight;
            let t = weight / self.weight;
            self.accu[accu] = self.mix(&self.accu[accu], incoming, t);
        }
    }
    fn accumulate_additive(&mut self, incoming: &[f64], weight: f64) {
        if self.weight_additive == 0. {
            self.add = if self.quaternion() {
                vec![0., 0., 0., 1.]
            } else {
                vec![0.; self.original.len()]
            };
        }
        self.add = self.mix_additive(&self.add, incoming, weight);
        self.weight_additive += weight;
    }
    fn apply(&mut self, scene: &mut Scene, accu: usize) -> Result<()> {
        let (weight, additive) = (self.weight, self.weight_additive);
        self.weight = 0.;
        self.weight_additive = 0.;
        let mut value = self.accu[accu].clone();
        if weight < 1. {
            value = self.mix(&value, &self.original, 1. - weight);
        }
        if additive > 0. {
            value = self.mix_additive(&value, &self.add, 1.);
        }
        self.accu[accu] = value.clone();
        write(scene, self.target, self.property, &value)
    }
}
pub(super) struct MixAction {
    pub clip: Arc<Clip>,
    bindings: Vec<usize>,
    pub time: f64,
    pub time_scale: f64,
    pub weight: f64,
    pub enabled: bool,
    pub paused: bool,
    pub additive: bool,
    loop_count: i64,
    weight_interp: Option<[f64; 4]>,
    scale_interp: Option<[f64; 4]>,
    restore_time_scale: Option<f64>,
    effective_weight: f64,
    effective_time_scale: f64,
    cache_index: usize,
}
#[derive(Default)]
pub(super) struct ThreeMixer {
    pub actions: Vec<MixAction>,
    /// `_actions`: the active ones first, in three's lend/take-back order.
    order: Vec<usize>,
    active: usize,
    bindings: Vec<Binding>,
    pub time: f64,
    pub time_scale: f64,
    accu_index: usize,
}
impl ThreeMixer {
    pub fn new() -> Self {
        Self {
            time_scale: 1.,
            ..Default::default()
        }
    }
    /// `clipAction( clip )`, bound to the clip's targets.
    pub fn clip_action(&mut self, scene: &Scene, clip: Arc<Clip>, additive: bool) -> Result<usize> {
        let mut bindings = vec![];
        for track in &clip.tracks {
            let index = match self
                .bindings
                .iter()
                .position(|b| b.target == track.target && b.property == track.property)
            {
                Some(i) => i,
                None => {
                    let original = read(scene, track.target, track.property)?;
                    self.bindings.push(Binding {
                        target: track.target,
                        property: track.property,
                        accu: [original.clone(), original.clone()],
                        add: original.clone(),
                        original,
                        weight: 0.,
                        weight_additive: 0.,
                        uses: 0,
                    });
                    self.bindings.len() - 1
                }
            };
            bindings.push(index);
        }
        let index = self.actions.len();
        self.actions.push(MixAction {
            clip,
            bindings,
            time: 0.,
            time_scale: 1.,
            weight: 1.,
            enabled: true,
            paused: false,
            additive,
            loop_count: -1,
            weight_interp: None,
            scale_interp: None,
            restore_time_scale: None,
            effective_weight: 1.,
            effective_time_scale: 1.,
            cache_index: self.order.len(),
        });
        self.order.push(index);
        Ok(index)
    }
    fn is_active(&self, a: usize) -> bool {
        self.actions[a].cache_index < self.active
    }
    pub fn play(&mut self, scene: &Scene, a: usize) -> Result<()> {
        if self.is_active(a) {
            return Ok(());
        }
        for &b in &self.actions[a].bindings {
            let binding = &mut self.bindings[b];
            if binding.uses == 0 {
                binding.original = read(scene, binding.target, binding.property)?;
            }
            binding.uses += 1;
        }
        // _lendAction: swap into the active section's end.
        let prev = self.actions[a].cache_index;
        let last = self.active;
        self.active += 1;
        let first_inactive = self.order[last];
        self.actions[a].cache_index = last;
        self.order[last] = a;
        self.actions[first_inactive].cache_index = prev;
        self.order[prev] = first_inactive;
        Ok(())
    }
    pub fn stop(&mut self, scene: &mut Scene, a: usize) -> Result<()> {
        if self.is_active(a) {
            for i in 0..self.actions[a].bindings.len() {
                let b = self.actions[a].bindings[i];
                let binding = &mut self.bindings[b];
                binding.uses -= 1;
                if binding.uses == 0 {
                    write(
                        scene,
                        binding.target,
                        binding.property,
                        &binding.original.clone(),
                    )?;
                }
            }
            // _takeBackAction.
            let prev = self.actions[a].cache_index;
            self.active -= 1;
            let first_inactive = self.active;
            let last_active = self.order[first_inactive];
            self.actions[a].cache_index = first_inactive;
            self.order[first_inactive] = a;
            self.actions[last_active].cache_index = prev;
            self.order[prev] = last_active;
        }
        self.reset(a);
        Ok(())
    }
    pub fn reset(&mut self, a: usize) {
        let action = &mut self.actions[a];
        action.paused = false;
        action.enabled = true;
        action.time = 0.;
        action.loop_count = -1;
        action.weight_interp = None;
        action.scale_interp = None;
        action.restore_time_scale = None;
    }
    pub fn set_effective_weight(&mut self, a: usize, weight: f64) {
        let action = &mut self.actions[a];
        action.weight = weight;
        action.effective_weight = if action.enabled { weight } else { 0. };
        action.weight_interp = None;
    }
    pub fn effective_weight(&self, a: usize) -> f64 {
        self.actions[a].effective_weight
    }
    pub fn set_effective_time_scale(&mut self, a: usize, time_scale: f64) {
        let action = &mut self.actions[a];
        action.time_scale = time_scale;
        action.effective_time_scale = if action.paused { 0. } else { time_scale };
        action.scale_interp = None;
        action.restore_time_scale = None;
    }
    pub fn stop_fading(&mut self, a: usize) {
        self.actions[a].weight_interp = None;
    }
    /// `_scheduleFading( duration, weightNow, weightThen )`.
    pub fn schedule_fading(&mut self, a: usize, duration: f64, now: f64, then: f64) {
        let t = self.time;
        self.actions[a].weight_interp = Some([t, t + duration, now, then]);
    }
    pub fn fade_in(&mut self, a: usize, duration: f64) {
        self.schedule_fading(a, duration, 0., 1.);
    }
    pub fn fade_out(&mut self, a: usize, duration: f64) {
        self.schedule_fading(a, duration, 1., 0.);
    }
    fn warp(&mut self, a: usize, start: f64, end: f64, duration: f64) {
        let t = self.time;
        let scale = self.actions[a].time_scale;
        self.actions[a].scale_interp = Some([t, t + duration, start / scale, end / scale]);
    }
    /// `from.crossFadeTo( to, duration, warp )`.
    pub fn cross_fade(&mut self, from: usize, to: usize, duration: f64, warp: bool) {
        self.fade_out(from, duration);
        self.fade_in(to, duration);
        if warp {
            let fade_in = self.actions[to].clip.duration();
            let fade_out = self.actions[from].clip.duration();
            self.actions[from].restore_time_scale = Some(self.actions[from].time_scale);
            self.actions[to].restore_time_scale = Some(self.actions[to].time_scale);
            self.warp(from, 1., fade_out / fade_in, duration);
            self.warp(to, fade_in / fade_out, 1., duration);
        }
    }
    /// `mixer.update( delta )`.
    pub fn update(&mut self, scene: &mut Scene, delta: f64) -> Result<()> {
        self.update_with(scene, delta, &mut |_, _| {})
    }
    /// `mixer.update( delta )`, calling `on_loop` when an action dispatches
    /// 'loop', before the remaining actions update, as event listeners run.
    pub fn update_with(
        &mut self,
        scene: &mut Scene,
        delta: f64,
        on_loop: &mut dyn FnMut(&mut ThreeMixer, usize),
    ) -> Result<()> {
        if !delta.is_finite() {
            return Err(Error::Invalid("mixer delta"));
        }
        let delta = delta * self.time_scale;
        self.time += delta;
        let time = self.time;
        let direction = if delta > 0. {
            1.
        } else if delta < 0. {
            -1.
        } else {
            0.
        };
        self.accu_index ^= 1;
        let accu = self.accu_index;
        for k in 0..self.active {
            let a = self.order[k];
            if !self.actions[a].enabled {
                self.update_weight(a, time);
                continue;
            }
            let _ = direction;
            let scaled = delta * self.update_time_scale(a, time);
            let (clip_time, looped) = self.update_time(a, scaled);
            if looped {
                on_loop(self, a);
            }
            let weight = self.update_weight(a, time);
            if weight > 0. {
                let action = &self.actions[a];
                let additive = action.additive;
                let samples = action
                    .clip
                    .tracks
                    .iter()
                    .map(|t| sample_raw(t, clip_time))
                    .collect::<Result<Vec<_>>>()?;
                let bindings = action.bindings.clone();
                for (b, value) in bindings.into_iter().zip(samples) {
                    if additive {
                        self.bindings[b].accumulate_additive(&value, weight);
                    } else {
                        self.bindings[b].accumulate(accu, &value, weight);
                    }
                }
            }
        }
        for binding in &mut self.bindings {
            if binding.uses > 0 {
                binding.apply(scene, accu)?;
            }
        }
        Ok(())
    }
    fn update_weight(&mut self, a: usize, time: f64) -> f64 {
        let action = &mut self.actions[a];
        let mut weight = 0.;
        if action.enabled {
            weight = action.weight;
            if let Some(i) = action.weight_interp {
                let value = control(&i, time);
                weight *= value;
                if time > i[1] {
                    action.weight_interp = None;
                    if value == 0. {
                        action.enabled = false;
                    }
                }
            }
        }
        action.effective_weight = weight;
        weight
    }
    fn update_time_scale(&mut self, a: usize, time: f64) -> f64 {
        let action = &mut self.actions[a];
        let mut scale = 0.;
        if !action.paused {
            scale = action.time_scale;
            if let Some(i) = action.scale_interp {
                let value = control(&i, time);
                scale *= value;
                if time > i[1] {
                    if scale == 0. {
                        action.paused = true;
                    } else {
                        if let Some(restore) = action.restore_time_scale {
                            scale = restore;
                        }
                        action.time_scale = scale;
                    }
                    action.scale_interp = None;
                    action.restore_time_scale = None;
                }
            }
        }
        action.effective_time_scale = scale;
        scale
    }
    /// `_updateTime` for LoopRepeat with infinite repetitions.
    fn update_time(&mut self, a: usize, delta: f64) -> (f64, bool) {
        let action = &mut self.actions[a];
        let duration = action.clip.duration();
        let mut time = action.time + delta;
        if delta == 0. {
            return (time, false);
        }
        let mut loop_count = action.loop_count;
        if loop_count == -1 && delta >= 0. {
            loop_count = 0;
        }
        if time >= duration || time < 0. {
            let loop_delta = (time / duration).floor();
            time -= duration * loop_delta;
            if loop_delta.is_finite() {
                loop_count += loop_delta.abs() as i64;
            }
            action.loop_count = loop_count;
            action.time = time;
            (time, true)
        } else {
            action.loop_count = loop_count;
            action.time = time;
            (time, false)
        }
    }
}
/// A keyframe track's interpolant as three evaluates it: linear values or
/// `slerpFlat` between raw keys, without normalizing; out of range and NaN
/// times take the end keys.
fn sample_raw(track: &crate::animation::Track, time: f64) -> Result<Vec<f64>> {
    use crate::animation::Interpolation;
    if !matches!(
        track.interpolation,
        Interpolation::Linear | Interpolation::Step
    ) {
        return track.sample(if time.is_finite() { time } else { 0. });
    }
    let times = &track.times;
    let last = times.len() - 1;
    if time.is_nan() || time >= times[last] {
        return Ok(track.values[last].clone());
    }
    if time <= times[0] {
        return Ok(track.values[0].clone());
    }
    let upper = times.partition_point(|&t| t <= time);
    let lower = upper - 1;
    let (a, b) = (&track.values[lower], &track.values[upper]);
    if matches!(track.interpolation, Interpolation::Step) {
        return Ok(a.clone());
    }
    let t = (time - times[lower]) / (times[upper] - times[lower]);
    Ok(if track.property == Property::Rotation {
        slerp_flat(a, b, t).to_vec()
    } else {
        a.iter().zip(b).map(|(a, b)| a + (b - a) * t).collect()
    })
}
/// `AnimationUtils.makeClipAdditive( clip )`: each track relative to its first key.
pub(super) fn make_additive(clip: &Clip) -> Clip {
    let mut clip = clip.clone();
    for track in &mut clip.tracks {
        if matches!(
            track.interpolation,
            crate::animation::Interpolation::CubicSpline
        ) {
            continue;
        }
        let reference = track.values.first().cloned().unwrap_or_default();
        if track.property == Property::Rotation {
            let q = Quaternion::from_xyzw(reference[0], reference[1], reference[2], reference[3])
                .normalize()
                .conjugate();
            let r = q.to_array();
            for v in &mut track.values {
                *v = multiply_flat(&r, v).to_vec();
            }
        } else {
            for v in &mut track.values {
                for (x, r) in v.iter_mut().zip(&reference) {
                    *x -= r;
                }
            }
        }
    }
    clip
}
/// `AnimationUtils.subclip( clip, name, startFrame, endFrame, fps )`.
pub(super) fn subclip(clip: &Clip, start: f64, end: f64, fps: f64) -> Clip {
    let mut out = Clip {
        name: clip.name.clone(),
        tracks: vec![],
    };
    for track in &clip.tracks {
        let mut t = track.clone();
        t.times.clear();
        t.values.clear();
        for (time, value) in track.times.iter().zip(&track.values) {
            let frame = time * fps;
            if frame < start || frame >= end {
                continue;
            }
            t.times.push(*time);
            t.values.push(value.clone());
        }
        if !t.times.is_empty() {
            out.tracks.push(t);
        }
    }
    let min = out
        .tracks
        .iter()
        .map(|t| t.times[0])
        .fold(f64::INFINITY, f64::min);
    for t in &mut out.tracks {
        for time in &mut t.times {
            *time -= min;
        }
    }
    out
}
