//! webgl_loader_md2_control: thirteen MD2CharacterComplex ogros sharing one
//! body and weapon geometry, each part a MorphBlendMesh blending its frame
//! animations, driven by the shared WASD / arrow controls, with the camera
//! riding the middle ogro on a Gyroscope, under a shadowing SunLight.
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::md2::{lambert, parse_md2, texture};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/md2";
const SKINS: [&str; 13] = [
    "grok.jpg",
    "ogrobase.png",
    "arboshak.png",
    "ctf_r.png",
    "ctf_b.png",
    "darkam.png",
    "freedom.png",
    "gib.png",
    "gordogh.png",
    "igdosh.png",
    "khorne.png",
    "nabogro.png",
    "sharokh.png",
];
const SCALE: f64 = 3.;
const TRANSITION_FRAMES: u32 = 15;
const ANIMATION_FPS: f64 = 6.;
/// One MorphBlendMesh animation: a frame range and its playback state.
#[derive(Clone)]
struct BlendAnimation {
    name: String,
    start: usize,
    length: usize,
    duration: f64,
    last_frame: usize,
    current_frame: usize,
    active: bool,
    time: f64,
    direction: f64,
    weight: f64,
    backwards: bool,
}
impl BlendAnimation {
    fn new(name: &str, start: usize, end: usize, fps: f64) -> Self {
        Self {
            name: name.to_string(),
            start,
            length: end - start + 1,
            duration: (end - start) as f64 / fps,
            last_frame: 0,
            current_frame: 0,
            active: false,
            time: 0.,
            direction: 1.,
            weight: 1.,
            backwards: false,
        }
    }
}
/// MorphBlendMesh: the mesh, its animations (`__default` first, then those
/// found by autoCreateAnimations) and its persistent morph influences.
struct Blend {
    mesh: Object3D,
    animations: Vec<BlendAnimation>,
    influences: Vec<f64>,
}
impl Blend {
    fn new(mesh: Object3D, names: &[String]) -> Self {
        let frames = names.len();
        let mut default =
            BlendAnimation::new("__default", 0, frames.saturating_sub(1), frames as f64);
        default.weight = 1.;
        let mut animations = vec![default];
        // autoCreateAnimations: /([a-z]+)_?(\d+)/i, frame ranges by first letters.
        let mut ranges: Vec<(String, usize, usize)> = vec![];
        for (i, key) in names.iter().enumerate() {
            let Some(name) = animation_name(key) else {
                continue;
            };
            match ranges.iter_mut().find(|r| r.0 == name) {
                Some(r) => {
                    r.1 = r.1.min(i);
                    r.2 = r.2.max(i);
                }
                None => ranges.push((name, i, i)),
            }
        }
        for (name, start, end) in ranges {
            animations.push(BlendAnimation::new(&name, start, end, ANIMATION_FPS));
        }
        Self {
            mesh,
            animations,
            influences: vec![0.; frames],
        }
    }
    fn get(&mut self, name: Option<&str>) -> Option<&mut BlendAnimation> {
        let name = name?;
        self.animations.iter_mut().find(|a| a.name == name)
    }
    fn set_weight(&mut self, name: Option<&str>, weight: f64) {
        if let Some(a) = self.get(name) {
            a.weight = weight;
        }
    }
    fn play(&mut self, name: Option<&str>) {
        if let Some(a) = self.get(name) {
            a.time = 0.;
            a.active = true;
        }
    }
    fn direction(&mut self, name: Option<&str>, backwards: bool) {
        if let Some(a) = self.get(name) {
            a.direction = if backwards { -1. } else { 1. };
            a.backwards = backwards;
        }
    }
    /// MorphBlendMesh.update( delta ).
    fn update(&mut self, delta: f64) {
        let influences = &mut self.influences;
        for a in &mut self.animations {
            if !a.active {
                continue;
            }
            let frame_time = a.duration / a.length as f64;
            a.time += a.direction * delta;
            a.time %= a.duration;
            if a.time < 0. {
                a.time += a.duration;
            }
            let keyframe =
                a.start + ((a.time / frame_time).floor().max(0.) as usize).min(a.length - 1);
            let weight = a.weight;
            if keyframe != a.current_frame {
                influences[a.last_frame] = 0.;
                influences[a.current_frame] = weight;
                influences[keyframe] = 0.;
                a.last_frame = a.current_frame;
                a.current_frame = keyframe;
            }
            let mut mix = (a.time % frame_time) / frame_time;
            if a.backwards {
                mix = 1. - mix;
            }
            if a.current_frame != a.last_frame {
                influences[a.current_frame] = mix * weight;
                influences[a.last_frame] = (1. - mix) * weight;
            } else {
                influences[a.current_frame] = weight;
            }
        }
    }
}
/// The first `[a-z]+` run followed by optional `_` and digits.
fn animation_name(key: &str) -> Option<String> {
    let bytes = key.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_alphabetic() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
                i += 1;
            }
            let end = i;
            let mut j = i;
            if j < bytes.len() && bytes[j] == b'_' {
                j += 1;
            }
            if j < bytes.len() && bytes[j].is_ascii_digit() {
                return Some(key[start..end].to_string());
            }
            // Backtrack as the regex does: a shorter letter run cannot be
            // followed by a digit, so continue after this run.
        } else {
            i += 1;
        }
    }
    None
}
/// One MD2CharacterComplex: its root, body and weapon parts, movement and
/// animation state.
struct Character {
    root: Object3D,
    body: Blend,
    weapon: Blend,
    speed: f64,
    orientation: f64,
    active: Option<String>,
    old: Option<String>,
    blend_counter: u32,
}
pub(super) struct Demo {
    time: f64,
    last: f64,
    characters: Vec<Character>,
    /// moveForward, moveBackward, moveLeft, moveRight.
    keys: [bool; 4],
    gyro: Object3D,
    controls: Controls,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 4000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 150., 1300.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::WHITE;
        s.fog = Some(Fog::Linear {
            color: Color::WHITE,
            near: 1000.,
            far: 4000.,
        });
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x666666),
            intensity: 3.,
        }));
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 7.,
        }));
        let n = s.get_mut(sun)?;
        n.position = Vector3::new(200., 450., 500.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            far: 2000.,
            ..Default::default()
        };
        let mut grass =
            decode_texture_image(&fetch(&format!("{ASSETS}/grasslight-big.jpg")).await?).await?;
        grass.srgb = true;
        grass.mipmap_filter = Some(Filter::Linear);
        grass.wrap_s = Wrapping::Repeat;
        grass.wrap_t = Wrapping::Repeat;
        grass.repeat = Vector2::new(64., 64.);
        let mut phong = MeshPhongMaterial::default();
        phong.properties.map = Some(Arc::new(grass));
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(16000., 16000., 1, 1)?),
            Arc::new(Material::Phong(phong)),
        )));
        let n = s.get_mut(ground)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.receive_shadow = true;
        // OrbitControls: target (0, 50, 0), updated once before the camera
        // joins the gyroscope.
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 50., 0.));
        controls.update(s, c)?;
        // The base character's parts, shared by the thirteen clones.
        let body = parse_md2(&fetch(&format!("{ASSETS}/ogro/ogro.md2")).await?)?;
        let weapon = parse_md2(&fetch(&format!("{ASSETS}/ogro/weapon.md2")).await?)?;
        let min_y = match body.geometry.attributes.get("position") {
            Some(Attribute::F32(a)) => a
                .array()
                .iter()
                .skip(1)
                .step_by(3)
                .fold(f32::INFINITY, |m, &v| m.min(v)),
            _ => 0.,
        };
        let mut skins = vec![];
        for name in SKINS {
            skins.push(texture(&format!("{ASSETS}/ogro/skins/{name}")).await?);
        }
        let weapon_skin = texture(&format!("{ASSETS}/ogro/skins/weapon.jpg")).await?;
        let (body_names, weapon_names) = (body.names, weapon.names);
        let (body_geometry, weapon_geometry) = (Arc::new(body.geometry), Arc::new(weapon.geometry));
        let mut characters = vec![];
        for (i, skin) in skins.iter().enumerate() {
            let root = s.insert(NodeKind::Group);
            s.get_mut(root)?.position = Vector3::new(
                (i as f64 - SKINS.len() as f64 / 2.) * 150.,
                -SCALE * min_y as f64,
                0.,
            );
            let part =
                |s: &mut Scene, g: &Arc<BufferGeometry>, map: &Arc<Texture>, visible: bool| {
                    let targets = g.morph_attributes.get("position").map_or(0, Vec::len);
                    let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                        g.clone(),
                        lambert(Some(map.clone()), false),
                    )));
                    let n = s.get_mut(mesh)?;
                    n.quaternion = Quaternion::from_rotation_y(-PI / 2.);
                    n.scale = Vector3::splat(SCALE);
                    // enableShadows( true ).
                    n.cast_shadow = true;
                    n.receive_shadow = true;
                    n.visible = visible;
                    n.morph_weights = vec![0.; targets];
                    s.add(root, mesh)?;
                    Ok::<_, crate::Error>(mesh)
                };
            // setSkin( i ) and setWeapon( 0 ).
            let body_mesh = part(s, &body_geometry, skin, true)?;
            let weapon_mesh = part(s, &weapon_geometry, &weapon_skin, true)?;
            characters.push(Character {
                root,
                body: Blend::new(body_mesh, &body_names),
                weapon: Blend::new(weapon_mesh, &weapon_names),
                speed: 0.,
                orientation: 0.,
                active: None,
                old: None,
                blend_counter: 0,
            });
        }
        // The Gyroscope on the middle ogro: its position, without rotation.
        let gyro = s.insert(NodeKind::Group);
        s.add(gyro, c)?;
        let mut d = Self {
            time: 0.,
            last: 0.,
            characters,
            keys: [false; 4],
            gyro,
            controls,
        };
        d.follow(s)?;
        Ok(d)
    }
    /// Gyroscope.updateMatrixWorld: the parent's world translation only.
    fn follow(&mut self, s: &mut Scene) -> Result<()> {
        let root = self.characters[SKINS.len() / 2].root;
        let position = s.get(root)?.position;
        s.get_mut(self.gyro)?.position = position;
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// render(): each character's update( delta ).
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let delta = self.time - self.last;
        self.last = self.time;
        let [forward, backward, left, right] = self.keys;
        let moving = forward || backward || left || right;
        for k in &mut self.characters {
            // updateMovementModel: maxSpeed is the config's walkSpeed (350).
            let max = 350.;
            let clamp = |v: f64| v.clamp(-max, max);
            if forward {
                k.speed = clamp(k.speed + delta * 600.);
            }
            if backward {
                k.speed = clamp(k.speed - delta * 600.);
            }
            if left {
                k.orientation += delta * 2.5;
                k.speed = clamp(k.speed + delta * 600.);
            }
            if right {
                k.orientation -= delta * 2.5;
                k.speed = clamp(k.speed + delta * 600.);
            }
            if !(forward || backward) {
                let ease = |x: f64| {
                    if x == 1. {
                        1.
                    } else {
                        1. - 2f64.powf(-10. * x)
                    }
                };
                if k.speed > 0. {
                    let e = ease(k.speed / max);
                    k.speed = (k.speed - e * delta * 600.).clamp(0., max);
                } else {
                    let e = ease(k.speed / -max);
                    k.speed = (k.speed + e * delta * 600.).clamp(-max, 0.);
                }
            }
            let step = k.speed * delta;
            let n = s.get_mut(k.root)?;
            n.position.x += k.orientation.sin() * step;
            n.position.z += k.orientation.cos() * step;
            n.quaternion = Quaternion::from_rotation_y(k.orientation);
            // updateBehaviors: run while moving, stand when slow and idle.
            let (move_animation, idle_animation) = ("run", "stand");
            if moving && k.active.as_deref() != Some(move_animation) {
                set_animation(k, move_animation);
            }
            if k.speed.abs() < 0.2 * max && !moving && k.active.as_deref() != Some(idle_animation) {
                set_animation(k, idle_animation);
            }
            // setAnimationDirectionForward, then Backward, on both animations.
            let (active, old) = (k.active.clone(), k.old.clone());
            for (pressed, backwards) in [(forward, false), (backward, true)] {
                if pressed {
                    for part in [&mut k.body, &mut k.weapon] {
                        part.direction(active.as_deref(), backwards);
                        part.direction(old.as_deref(), backwards);
                    }
                }
            }
            // updateAnimations.
            let mut mix = 1.;
            if k.blend_counter > 0 {
                mix = (TRANSITION_FRAMES - k.blend_counter) as f64 / TRANSITION_FRAMES as f64;
                k.blend_counter -= 1;
            }
            let (active, old) = (k.active.clone(), k.old.clone());
            for part in [&mut k.body, &mut k.weapon] {
                part.update(delta);
                part.set_weight(active.as_deref(), mix);
                part.set_weight(old.as_deref(), 1. - mix);
                s.get_mut(part.mesh)?.morph_weights = part.influences.clone();
            }
        }
        self.follow(s)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    /// keydown / keyup: W or ArrowUp, S or ArrowDown, A or ArrowLeft, D or ArrowRight.
    pub fn key(&mut self, code: u32, down: bool) {
        let index = match code {
            87 | 38 => 0,
            83 | 40 => 1,
            65 | 37 => 2,
            68 | 39 => 3,
            _ => return,
        };
        self.keys[index] = down;
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
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        // OrbitControls' lookAt runs Object3D.updateWorldMatrix, which the
        // Gyroscope does not override: during it the gyroscope takes the
        // ogro's full world matrix, rotation included.
        let rotation = s.get(self.characters[SKINS.len() / 2].root)?.quaternion;
        s.get_mut(self.gyro)?.quaternion = rotation;
        let result = self.controls.update(s, c);
        s.get_mut(self.gyro)?.quaternion = Quaternion::IDENTITY;
        result
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(crate::Error::Invalid("md2 control parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// setAnimation(): both parts start the new animation from weight 0.
fn set_animation(k: &mut Character, name: &str) {
    for part in [&mut k.body, &mut k.weapon] {
        part.set_weight(Some(name), 0.);
        part.play(Some(name));
    }
    k.old = k.active.take();
    k.active = Some(name.to_string());
    k.blend_counter = TRANSITION_FRAMES;
}
