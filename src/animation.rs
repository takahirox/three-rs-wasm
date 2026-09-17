//! Keyframe interpolation and weighted animation actions, independent of file format.
use crate::{Error, Result, math::*, scene::*};
use std::{collections::HashMap, sync::Arc};
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Property {
    Position,
    Rotation,
    Scale,
    Weights,
}
#[derive(Clone, Copy, Debug)]
pub enum Interpolation {
    Step,
    Linear,
    CubicSpline,
}
#[derive(Clone, Debug)]
pub struct Track {
    pub target: Object3D,
    pub property: Property,
    /// Nondecreasing times; at duplicate keys the last value wins (Three.js behavior).
    pub times: Vec<f64>,
    /// Cubic tracks store in-tangent, value, out-tangent for each key.
    pub values: Vec<Vec<f64>>,
    pub interpolation: Interpolation,
}
impl Track {
    pub fn validate(&self) -> Result<()> {
        let factor = if matches!(self.interpolation, Interpolation::CubicSpline) {
            3
        } else {
            1
        };
        let width = match self.property {
            Property::Position | Property::Scale => 3,
            Property::Rotation => 4,
            Property::Weights => self.values.first().map_or(0, Vec::len),
        };
        if self.times.is_empty()
            || width == 0
            || self.values.len() != self.times.len() * factor
            || self.times.iter().any(|t| !t.is_finite() || *t < 0.0)
            || self.times.windows(2).any(|t| t[1] < t[0])
            || self
                .values
                .iter()
                .any(|v| v.len() != width || v.iter().any(|x| !x.is_finite()))
        {
            return Err(Error::Invalid("animation track data"));
        }
        if self.property == Property::Rotation
            && self
                .values
                .iter()
                .skip(if factor == 3 { 1 } else { 0 })
                .step_by(factor)
                .any(|v| v.iter().map(|x| x * x).sum::<f64>() < 1e-20)
        {
            return Err(Error::Invalid("zero animation quaternion"));
        }
        Ok(())
    }
    pub fn sample(&self, time: f64) -> Result<Vec<f64>> {
        self.validate()?;
        if !time.is_finite() {
            return Err(Error::Invalid("animation time"));
        }
        let cubic = matches!(self.interpolation, Interpolation::CubicSpline);
        let stride = if cubic { 3 } else { 1 };
        let offset = usize::from(cubic);
        let upper = self.times.partition_point(|&t| t <= time);
        if upper == 0 {
            return Ok(normalize(self.property, self.values[offset].clone()));
        }
        if upper == self.times.len() {
            return Ok(normalize(
                self.property,
                self.values[(upper - 1) * stride + offset].clone(),
            ));
        }
        let lower = upper - 1;
        let span = self.times[upper] - self.times[lower];
        let t = (time - self.times[lower]) / span;
        let a = &self.values[lower * stride + offset];
        let b = &self.values[upper * stride + offset];
        let result = match self.interpolation {
            Interpolation::Step => a.clone(),
            Interpolation::Linear if self.property == Property::Rotation => {
                quat(a).slerp(quat(b), t).to_array().to_vec()
            }
            Interpolation::Linear => a.iter().zip(b).map(|(a, b)| a + (b - a) * t).collect(),
            Interpolation::CubicSpline => a
                .iter()
                .zip(b)
                .enumerate()
                .map(|(c, (a, b))| {
                    (2.0 * t * t * t - 3.0 * t * t + 1.0) * a
                        + (t * t * t - 2.0 * t * t + t) * span * self.values[lower * 3 + 2][c]
                        + (-2.0 * t * t * t + 3.0 * t * t) * b
                        + (t * t * t - t * t) * span * self.values[upper * 3][c]
                })
                .collect(),
        };
        Ok(normalize(self.property, result))
    }
}
fn quat(v: &[f64]) -> Quaternion {
    Quaternion::from_xyzw(v[0], v[1], v[2], v[3]).normalize()
}
fn normalize(property: Property, value: Vec<f64>) -> Vec<f64> {
    if property == Property::Rotation {
        quat(&value).to_array().to_vec()
    } else {
        value
    }
}
#[derive(Clone, Debug)]
pub struct Clip {
    pub name: String,
    pub tracks: Vec<Track>,
}
impl Clip {
    pub fn duration(&self) -> f64 {
        self.tracks
            .iter()
            .filter_map(|t| t.times.last())
            .copied()
            .fold(0.0, f64::max)
    }
    pub fn validate(&self) -> Result<()> {
        for track in &self.tracks {
            track.validate()?;
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum LoopMode {
    Once,
    Repeat,
    PingPong,
}
pub struct Action {
    pub clip: Arc<Clip>,
    pub time: f64,
    pub time_scale: f64,
    pub weight: f64,
    pub paused: bool,
    pub enabled: bool,
    pub looping: LoopMode,
    fade: Option<(f64, f64, f64, f64)>,
}
impl Action {
    pub fn new(clip: Arc<Clip>) -> Result<Self> {
        clip.validate()?;
        Ok(Self {
            clip,
            time: 0.0,
            time_scale: 1.0,
            weight: 1.0,
            paused: false,
            enabled: true,
            looping: LoopMode::Repeat,
            fade: None,
        })
    }
    pub fn fade_to(&mut self, weight: f64, duration: f64) -> Result<()> {
        if !weight.is_finite() || weight < 0.0 || !duration.is_finite() || duration < 0.0 {
            return Err(Error::Invalid("animation fade"));
        }
        if duration == 0.0 {
            self.weight = weight;
            self.fade = None;
        } else {
            self.fade = Some((self.weight, weight, 0.0, duration));
        }
        Ok(())
    }
}
#[derive(Default)]
pub struct AnimationMixer {
    pub actions: Vec<Action>,
    rest: HashMap<(Object3D, Property), Vec<f64>>,
}
impl AnimationMixer {
    pub fn play(&mut self, clip: Arc<Clip>) -> Result<usize> {
        let index = self.actions.len();
        self.actions.push(Action::new(clip)?);
        Ok(index)
    }
    pub fn cross_fade(&mut self, from: usize, to: usize, duration: f64) -> Result<()> {
        if from == to || from >= self.actions.len() || to >= self.actions.len() {
            return Err(Error::Invalid("animation action index"));
        }
        self.actions[from].fade_to(0.0, duration)?;
        self.actions[to].weight = 0.0;
        self.actions[to].enabled = true;
        self.actions[to].fade_to(1.0, duration)?;
        Ok(())
    }
    /// Update after creating/binding scene nodes, before rendering. Disabled actions
    /// release their contribution back to the captured rest pose.
    pub fn update(&mut self, scene: &mut Scene, delta: f64) -> Result<()> {
        if !delta.is_finite() || delta < 0.0 {
            return Err(Error::Invalid("animation delta"));
        }
        type Samples = HashMap<(Object3D, Property), Vec<(f64, Vec<f64>)>>;
        let mut samples = Samples::new();
        for action in &mut self.actions {
            if !action.time.is_finite()
                || !action.time_scale.is_finite()
                || !action.weight.is_finite()
                || action.weight < 0.0
            {
                return Err(Error::Invalid("animation action"));
            }
            for track in &action.clip.tracks {
                let key = (track.target, track.property);
                if let std::collections::hash_map::Entry::Vacant(entry) = self.rest.entry(key) {
                    entry.insert(read(scene, track.target, track.property)?);
                }
            }
            if !action.enabled {
                continue;
            }
            if !action.paused {
                action.time += delta * action.time_scale;
            }
            if let Some((start, end, elapsed, duration)) = &mut action.fade {
                *elapsed += delta;
                action.weight = *start + (*end - *start) * (*elapsed / *duration).min(1.0);
                if *elapsed >= *duration {
                    action.fade = None;
                }
            }
            if action.weight == 0.0 {
                continue;
            }
            let duration = action.clip.duration();
            let time = if duration == 0.0 {
                0.0
            } else {
                match action.looping {
                    LoopMode::Once => action.time.clamp(0.0, duration),
                    LoopMode::Repeat => action.time.rem_euclid(duration),
                    LoopMode::PingPong => {
                        let phase = action.time.rem_euclid(2.0 * duration);
                        phase.min(2.0 * duration - phase)
                    }
                }
            };
            for track in &action.clip.tracks {
                samples
                    .entry((track.target, track.property))
                    .or_default()
                    .push((action.weight, track.sample(time)?));
            }
        }
        for (&(target, property), rest) in &self.rest {
            let values = samples.entry((target, property)).or_default();
            let sum = values.iter().map(|(w, _)| w).sum::<f64>();
            if sum < 1.0 {
                values.push((1.0 - sum, rest.clone()));
            }
            if values.iter().any(|(_, v)| v.len() != rest.len()) {
                return Err(Error::Invalid("animation binding width"));
            }
            let result = if property == Property::Rotation {
                let mut blended = Quaternion::IDENTITY;
                let mut weight = 0.0;
                for (w, v) in values {
                    if *w > 0.0 {
                        blended = blended.slerp(quat(v), *w / (weight + *w));
                        weight += *w;
                    }
                }
                blended.to_array().to_vec()
            } else {
                let total = sum.max(1.0);
                (0..rest.len())
                    .map(|c| values.iter().map(|(w, v)| w * v[c] / total).sum())
                    .collect()
            };
            let node = scene.get_mut(target)?;
            match property {
                Property::Position => node.position = Vector3::from_slice(&result),
                Property::Rotation => node.quaternion = quat(&result),
                Property::Scale => node.scale = Vector3::from_slice(&result),
                Property::Weights => node.morph_weights = result,
            }
            node.matrix_auto_update = true;
            node.matrix_world_needs_update = true;
        }
        Ok(())
    }
    pub fn restore(&mut self, scene: &mut Scene) -> Result<()> {
        for action in &mut self.actions {
            action.enabled = false;
        }
        self.update(scene, 0.0)
    }
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
