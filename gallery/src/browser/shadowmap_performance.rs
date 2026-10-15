//! webgl_shadowmap_performance: 601 galloping horses ( the Horse glTF's
//! morph animation in 25 AnimationObjectGroups ), the extruded "THREE.JS"
//! text and two blocks on the ground, under an ambient light and the
//! SunLight's PCF cascades, in linear fog, with FirstPersonControls. Each
//! frame the mixer advances the groups' actions by the timer, the horses run
//! along x ( wrapping to a seeded random start ) and the controls move the
//! camera, as the page does, on the engine's renderer. Each horse is its own
//! mesh with its own material color, as the page clones them; the clones
//! share the glTF geometry, and their morph weights are sampled from the
//! clip's linear keyframes on the CPU, as AnimationMixer samples them.
use super::gltf_viewer::{fetch, load_asset};
use super::text_clipping::text_geometry;
use super::text_shapes::{Extrude, Font};
use super::trackball_sprites::FirstPerson;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const FLOOR: f64 = -250.;
const GROUPS: usize = 25;
const SPEED: f64 = 550.;
/// The fixture's Math.random: a 32-bit LCG seeded with 186.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(self.0) / 4294967296.
    }
}
/// `Color.offsetHSL( 0, s, l )` in the working ( linear ) color space.
pub(super) fn offset_hsl(c: Color, ds: f64, dl: f64) -> Color {
    let v = c.0;
    let (max, min) = (v.max_element(), v.min_element());
    let l = (min + max) / 2.;
    let (mut h, mut s) = (0., 0.);
    if min != max {
        let delta = max - min;
        s = if l <= 0.5 {
            delta / (max + min)
        } else {
            delta / (2. - max - min)
        };
        h = if max == v.x {
            (v.y - v.z) / delta + if v.y < v.z { 6. } else { 0. }
        } else if max == v.y {
            (v.z - v.x) / delta + 2.
        } else {
            (v.x - v.y) / delta + 4.
        } / 6.;
    }
    let (h, s, l) = (
        h.rem_euclid(1.),
        (s + ds).clamp(0., 1.),
        (l + dl).clamp(0., 1.),
    );
    if s == 0. {
        return Color(Vector3::splat(l));
    }
    let hue = |p: f64, q: f64, mut t: f64| {
        if t < 0. {
            t += 1.;
        }
        if t > 1. {
            t -= 1.;
        }
        if t < 1. / 6. {
            p + (q - p) * 6. * t
        } else if t < 0.5 {
            q
        } else if t < 2. / 3. {
            p + (q - p) * 6. * (2. / 3. - t)
        } else {
            p
        }
    };
    let p = if l <= 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let q = 2. * l - p;
    Color(Vector3::new(
        hue(q, p, h + 1. / 3.),
        hue(q, p, h),
        hue(q, p, h - 1. / 3.),
    ))
}
/// A horse: its node, its group's phase and its run.
struct Horse {
    node: Object3D,
    group: usize,
}
pub(super) struct Demo {
    walker: FirstPerson,
    random: Random,
    horses: Vec<Horse>,
    /// Each group's action phase ( startAt( −duration × phase ) ).
    phases: Vec<Option<f64>>,
    /// The clip's keyframe times and influences ( one row per keyframe ).
    times: Vec<f64>,
    values: Vec<Vec<f64>>,
    duration: f64,
    time: f64,
    last: f64,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 23.,
            near: 5.,
            far: 3000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(700., 50., 1900.);
        s.background = Color::from_hex(0x59472b);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x59472b),
            near: 1000.,
            far: 3000.,
        });
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.,
        }));
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 3.,
        }));
        let n = s.get_mut(sun)?;
        n.position = Vector3::new(0., 1500., 1000.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            far: 3000.,
            normal_bias: 1.,
            ..Default::default()
        };
        // The ground and the two blocks share the plane's Phong material.
        let plane_material = Arc::new(Material::Phong(MeshPhongMaterial {
            properties: MaterialProperties {
                color: Color::from_hex(0xffdd99),
                ..Default::default()
            },
            ..Default::default()
        }));
        let mesh =
            |s: &mut Scene, g: BufferGeometry, m: Arc<Material>, cast: bool| -> Result<Object3D> {
                let h = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(g), m)));
                let n = s.get_mut(h)?;
                n.cast_shadow = cast;
                n.receive_shadow = true;
                Ok(h)
            };
        let ground = mesh(
            s,
            PlaneGeometry::build(100., 100., 1, 1)?,
            plane_material.clone(),
            false,
        )?;
        let n = s.get_mut(ground)?;
        n.position = Vector3::new(0., FLOOR, 0.);
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.scale = Vector3::splat(100.);
        // The text, centered by its bounding box.
        let font =
            Font::parse(&fetch("/web/gallery/assets/fonts/helvetiker_bold.typeface.json").await?)?;
        let text = text_geometry(
            &font,
            "THREE.JS",
            200.,
            &Extrude {
                curve_segments: 12,
                steps: 1,
                depth: 50.,
                bevel: Some((2., 5., 3)),
            },
        )?;
        let (min_x, max_x) = match text.attributes.get("position") {
            Some(Attribute::F32(a)) => a
                .array()
                .chunks(3)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
                    (lo.min(f64::from(p[0])), hi.max(f64::from(p[0])))
                }),
            _ => return Err(Error::Invalid("text positions")),
        };
        let text_material = Arc::new(Material::Phong(MeshPhongMaterial {
            properties: MaterialProperties {
                color: Color::from_hex(0xff0000),
                ..Default::default()
            },
            specular: Color::from_hex(0xffffff),
            ..Default::default()
        }));
        let text_mesh = mesh(s, text, text_material, true)?;
        s.get_mut(text_mesh)?.position = Vector3::new(-0.5 * (max_x - min_x), FLOOR + 67., 0.);
        for (w, h, d) in [(1500., 220., 150.), (1600., 170., 250.)] {
            let block = mesh(
                s,
                BoxGeometry::build(w, h, d)?,
                plane_material.clone(),
                true,
            )?;
            s.get_mut(block)?.position = Vector3::new(0., FLOOR - 50., 20.);
        }
        // The horse: its geometry, material and morph clip.
        let (a, b, i) = load_asset("/web/gallery/assets/gltf/Horse.glb").await?;
        // The template mesh, from a scratch scene ( the page clones it ).
        let mut scratch = Scene::new();
        let template =
            crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(&mut scratch)?;
        let first = *template
            .meshes
            .first()
            .ok_or(Error::Invalid("horse mesh"))?;
        let (geometry, material) = match &scratch.get(first)?.kind {
            NodeKind::Mesh(m) => (m.geometry.clone(), m.materials[0].clone()),
            _ => return Err(Error::Invalid("horse mesh")),
        };
        let animation = a.animations().next().ok_or(Error::Invalid("horse clip"))?;
        let channel = animation
            .channels()
            .next()
            .ok_or(Error::Invalid("horse track"))?;
        let reader = channel.reader(|buffer| b.get(buffer.index()).map(Vec::as_slice));
        let times: Vec<f64> = reader
            .read_inputs()
            .ok_or(Error::Invalid("horse times"))?
            .map(f64::from)
            .collect();
        let flat: Vec<f64> = match reader.read_outputs() {
            Some(gltf::animation::util::ReadOutputs::MorphTargetWeights(w)) => {
                w.into_f32().map(f64::from).collect()
            }
            _ => return Err(Error::Invalid("horse weights")),
        };
        let targets = flat.len() / times.len();
        let values: Vec<Vec<f64>> = flat.chunks(targets).map(<[f64]>::to_vec).collect();
        let duration = times.last().copied().unwrap_or(1.);
        // addMorph( mesh, clip, 550, 1, 100 − random × 3000, FLOOR, i, true, true ).
        let mut random = Random(186);
        let mut horses = vec![];
        let mut phases = vec![None; GROUPS];
        for z in (-600..601).step_by(2) {
            let x = 100. - random.next() * 3000.;
            let base = match material.as_ref() {
                Material::Standard(m) => m.properties.color,
                _ => Color::WHITE,
            };
            let (ds, dl) = (random.next() * 0.5 - 0.25, random.next() * 0.5 - 0.25);
            let mut cloned = material.as_ref().clone();
            if let Material::Standard(m) = &mut cloned {
                m.properties.color = offset_hsl(base, ds, dl);
                // r186's WebGL physical shading: the DFG LUT's multiple scattering.
                m.energy_conservation = true;
            }
            let group = (random.next() * GROUPS as f64).floor() as usize;
            if phases[group].is_none() {
                let randomness = 0.6 * random.next() - 0.3;
                phases[group] = Some((group as f64 + randomness) / GROUPS as f64);
            }
            let node = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(cloned),
            )));
            let n = s.get_mut(node)?;
            n.position = Vector3::new(x, FLOOR, f64::from(z));
            n.quaternion = Quaternion::from_rotation_y(PI / 2.);
            n.cast_shadow = true;
            n.receive_shadow = true;
            n.morph_weights = vec![0.; targets];
            horses.push(Horse { node, group });
        }
        // controls.lookAt( scene.position ): the controls take the orientation.
        let position = s.get(c)?.position;
        let look = (Vector3::ZERO - position).normalize();
        s.look_at(c, Vector3::ZERO)?;
        let mut walker = FirstPerson::default();
        walker.speed = 500.;
        walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
        walker.lon = look.x.atan2(look.z).to_degrees();
        Ok(Self {
            walker,
            random,
            horses,
            phases,
            times,
            values,
            duration,
            time: 0.,
            last: 0.,
        })
    }
    /// LinearInterpolant over the clip's keyframes.
    fn sample(&self, t: f64) -> Vec<f64> {
        let n = self.times.len();
        if t <= self.times[0] {
            return self.values[0].clone();
        }
        if t >= self.times[n - 1] {
            return self.values[n - 1].clone();
        }
        let k = self.times.partition_point(|&x| x <= t).max(1);
        let (t0, t1) = (self.times[k - 1], self.times[k]);
        let a = (t - t0) / (t1 - t0);
        self.values[k - 1]
            .iter()
            .zip(&self.values[k])
            .map(|(v0, v1)| v0 * (1. - a) + v1 * a)
            .collect()
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// render(): the mixer, the horses' runs, then the controls.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let delta = self.time - self.last;
        self.last = self.time;
        // setDuration( 1 ): timeScale = duration / 1; startAt( −phase ). An
        // update with a zero delta has no time direction, so the scheduled
        // start waits for the mixer's first non-zero step.
        let started = self.time > 0.;
        let weights: Vec<Vec<f64>> = self
            .phases
            .iter()
            .map(|phase| {
                let run = if started {
                    self.time + phase.unwrap_or(0.)
                } else {
                    0.
                };
                let local = (run * self.duration).rem_euclid(self.duration);
                self.sample(local)
            })
            .collect();
        for horse in &self.horses {
            let n = s.get_mut(horse.node)?;
            n.morph_weights.clone_from(&weights[horse.group]);
            n.position.x += SPEED * delta;
            if n.position.x > 2000. {
                n.position.x = -1000. - self.random.next() * 500.;
            }
        }
        self.walker.update(s, c, delta)
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.walker.pointer(kind, x, y);
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.walker.key(code, down);
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
