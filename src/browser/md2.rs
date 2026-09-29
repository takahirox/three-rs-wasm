//! webgl_loader_md2: MD2Loader's frames as absolute morph targets, the
//! morph-target sequence clips, and MD2Character's body, skins, weapons,
//! wireframe and playback rate, under two shadowing spot lights.
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/md2";
const SKINS: [&str; 5] = [
    "ratamahatta.png",
    "ctf_b.png",
    "ctf_r.png",
    "dead.png",
    "gearwhore.png",
];
const WEAPONS: [&str; 11] = [
    "weapon",
    "w_bfg",
    "w_blaster",
    "w_chaingun",
    "w_glauncher",
    "w_hyperblaster",
    "w_machinegun",
    "w_railgun",
    "w_rlauncher",
    "w_shotgun",
    "w_sshotgun",
];
/// The 162 precomputed MD2 vertex normals.
const NORMALS: [[f32; 3]; 162] = include!("md2_normals.in");
/// A NumberKeyframeTrack with linear interpolation: before the first key and
/// after the last the end values hold, and at duplicate keys the last wins.
struct Track {
    times: Vec<f64>,
    values: Vec<f64>,
}
impl Track {
    fn sample(&self, time: f64) -> f64 {
        let upper = self.times.partition_point(|&t| t <= time);
        if upper == 0 {
            return self.values[0];
        }
        if upper == self.times.len() {
            return self.values[upper - 1];
        }
        let (t0, t1) = (self.times[upper - 1], self.times[upper]);
        let (a, b) = (self.values[upper - 1], self.values[upper]);
        a + (b - a) * (time - t0) / (t1 - t0)
    }
}
/// A morph-target sequence clip: its name, its tracks (one per target, with
/// the target's morph index) and its duration.
struct Clip {
    name: String,
    tracks: Vec<(usize, Track)>,
    duration: f64,
}
pub(super) struct Md2 {
    pub(super) geometry: BufferGeometry,
    clips: Vec<Clip>,
    /// The frame names, in morph-target order (morphTargetDictionary).
    pub(super) names: Vec<String>,
}
/// MD2Loader.parse(): non-indexed positions, normals and uvs from frame 0,
/// every frame as an absolute morph target, and
/// `AnimationClip.CreateClipsFromMorphTargetSequences( frames, 10, false )`.
pub(super) fn parse_md2(data: &[u8]) -> Result<Md2> {
    let bad = || Error::Invalid("MD2 data");
    let i32_at = |o: usize| -> Result<i32> {
        Ok(i32::from_le_bytes(
            data.get(o..o + 4)
                .ok_or_else(bad)?
                .try_into()
                .map_err(|_| bad())?,
        ))
    };
    let u16_at = |o: usize| -> Result<u16> {
        Ok(u16::from_le_bytes(
            data.get(o..o + 2)
                .ok_or_else(bad)?
                .try_into()
                .map_err(|_| bad())?,
        ))
    };
    let f32_at = |o: usize| -> Result<f32> {
        Ok(f32::from_le_bytes(
            data.get(o..o + 4)
                .ok_or_else(bad)?
                .try_into()
                .map_err(|_| bad())?,
        ))
    };
    let header: Vec<i32> = (0..17).map(|i| i32_at(i * 4)).collect::<Result<_>>()?;
    if header[0] != 844121161 || header[1] != 8 || header[16] as usize != data.len() {
        return Err(Error::Invalid("not a valid MD2 file"));
    }
    let [skin_width, skin_height] = [header[2] as f32, header[3] as f32];
    let (vertices, st_count, tris, frames) = (
        header[6] as usize,
        header[7] as usize,
        header[8] as usize,
        header[10] as usize,
    );
    let mut uvs = Vec::with_capacity(st_count * 2);
    for i in 0..st_count {
        let o = header[12] as usize + i * 4;
        let u = u16_at(o)? as i16 as f32;
        let v = u16_at(o + 2)? as i16 as f32;
        uvs.push([u / skin_width, 1. - v / skin_height]);
    }
    let mut vertex_indices = Vec::with_capacity(tris * 3);
    let mut uv_indices = Vec::with_capacity(tris * 3);
    for i in 0..tris {
        let o = header[13] as usize + i * 12;
        for k in 0..3 {
            vertex_indices.push(u16_at(o + k * 2)? as usize);
            uv_indices.push(u16_at(o + 6 + k * 2)? as usize);
        }
    }
    let mut names = vec![];
    let mut frame_data: Vec<(Vec<f32>, Vec<f32>)> = vec![];
    let mut o = header[14] as usize;
    for _ in 0..frames {
        let scale = [f32_at(o)?, f32_at(o + 4)?, f32_at(o + 8)?];
        let translation = [f32_at(o + 12)?, f32_at(o + 16)?, f32_at(o + 20)?];
        o += 24;
        let name: String = data
            .get(o..o + 16)
            .ok_or_else(bad)?
            .iter()
            .take_while(|&&c| c != 0)
            .map(|&c| c as char)
            .collect();
        o += 16;
        let (mut p, mut n) = (
            Vec::with_capacity(vertices * 3),
            Vec::with_capacity(vertices * 3),
        );
        for _ in 0..vertices {
            let b = data.get(o..o + 4).ok_or_else(bad)?;
            o += 4;
            let x = b[0] as f32 * scale[0] + translation[0];
            let y = b[1] as f32 * scale[1] + translation[1];
            let z = b[2] as f32 * scale[2] + translation[2];
            // Y-up.
            p.extend([x, z, y]);
            let normal = NORMALS.get(b[3] as usize).ok_or_else(bad)?;
            n.extend([normal[0], normal[2], normal[1]]);
        }
        names.push(name);
        frame_data.push((p, n));
    }
    let gather = |source: &[f32]| -> Result<Vec<f32>> {
        let mut out = Vec::with_capacity(vertex_indices.len() * 3);
        for &v in &vertex_indices {
            out.extend_from_slice(source.get(v * 3..v * 3 + 3).ok_or_else(bad)?);
        }
        Ok(out)
    };
    let vec3 = |v: Vec<f32>| -> Result<Attribute> {
        Ok(Attribute::F32(BufferAttribute::new(v, 3, false)?))
    };
    let first = frame_data.first().ok_or_else(bad)?;
    let mut geometry = BufferGeometry::default();
    geometry.set_attribute("position", vec3(gather(&first.0)?)?);
    geometry.set_attribute("normal", vec3(gather(&first.1)?)?);
    let mut uv = Vec::with_capacity(uv_indices.len() * 2);
    for &i in &uv_indices {
        uv.extend(uvs.get(i).ok_or_else(bad)?);
    }
    geometry.set_attribute("uv", Attribute::F32(BufferAttribute::new(uv, 2, false)?));
    let (mut positions, mut normals) = (vec![], vec![]);
    for (p, n) in &frame_data {
        positions.push(vec3(gather(p)?)?);
        normals.push(vec3(gather(n)?)?);
    }
    geometry
        .morph_attributes
        .insert("position".into(), positions);
    geometry.morph_attributes.insert("normal".into(), normals);
    geometry.morph_targets_relative = false;
    // Group names by /^([\w-]*?)([\d]+)$/: the prefix before trailing digits.
    let mut groups: Vec<(String, Vec<usize>)> = vec![];
    for (i, name) in names.iter().enumerate() {
        let prefix = name.trim_end_matches(|c: char| c.is_ascii_digit());
        if prefix.len() == name.len()
            || !prefix
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            continue;
        }
        match groups.iter_mut().find(|(n, _)| n == prefix) {
            Some((_, v)) => v.push(i),
            None => groups.push((prefix.to_string(), vec![i])),
        }
    }
    let clips = groups
        .into_iter()
        .map(|(name, targets)| sequence_clip(name, &targets))
        .collect();
    Ok(Md2 {
        geometry,
        clips,
        names,
    })
}
/// AnimationClip.CreateFromMorphTargetSequence( name, targets, 10, false ).
fn sequence_clip(name: String, targets: &[usize]) -> Clip {
    let n = targets.len();
    let mut tracks = vec![];
    let mut duration: f64 = 0.;
    for (i, &target) in targets.iter().enumerate() {
        let keys = [((i + n - 1) % n, 0.), (i, 1.), ((i + 1) % n, 0.)];
        // getKeyframeOrder: a stable sort by time.
        let mut order = [0, 1, 2];
        order.sort_by_key(|&k| keys[k].0);
        let mut times: Vec<f64> = order.iter().map(|&k| keys[k].0 as f64).collect();
        let mut values: Vec<f64> = order.iter().map(|&k| keys[k].1).collect();
        if times[0] == 0. {
            times.push(n as f64);
            values.push(values[0]);
        }
        let times: Vec<f64> = times.iter().map(|t| t / 10.).collect();
        duration = duration.max(*times.last().unwrap_or(&0.));
        tracks.push((target, Track { times, values }));
    }
    Clip {
        name,
        tracks,
        duration,
    }
}
/// One MD2Character part: its mesh, clips and its current action time.
struct Part {
    mesh: Object3D,
    clips: Vec<Clip>,
    targets: usize,
    texture: Arc<Material>,
    wireframe: Arc<Material>,
}
pub(super) struct Demo {
    time: f64,
    last: f64,
    body: Part,
    weapons: Vec<Part>,
    /// meshWeapon: the last loaded weapon, then the selected one.
    weapon: usize,
    /// The body's textured material per skin, kept resident.
    skins: Vec<Arc<Material>>,
    /// GUI changes, applied at the next frame.
    pending: Vec<(usize, f32)>,
    /// activeClipName, and the body action's time (null when missing).
    clip: String,
    action: Option<f64>,
    time_scale: f64,
    wireframe: bool,
    controls: Controls,
}
pub(super) fn lambert(map: Option<Arc<Texture>>, wireframe: bool) -> Arc<Material> {
    let mut m = MeshLambertMaterial::default();
    if wireframe {
        m.properties.color = Color::from_hex(0xffaa00);
        m.properties.wireframe = true;
    } else {
        m.properties.map = map;
    }
    Arc::new(Material::Lambert(m))
}
pub(super) async fn texture(url: &str) -> Result<Arc<Texture>> {
    let mut t = decode_texture_image(&fetch(url).await?).await?;
    t.srgb = true;
    t.mipmap_filter = Some(Filter::Linear);
    Ok(Arc::new(t))
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 2., 4.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::from_hex(0x050505);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x050505),
            near: 2.5,
            far: 10.,
        });
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x666666),
            intensity: 1.,
        }));
        for position in [Vector3::new(2., 5., 10.), Vector3::new(-1., 3.5, 3.5)] {
            let light = s.insert(NodeKind::Light(Light::Spot {
                color: Color::WHITE,
                intensity: 150.,
                target: Vector3::ZERO,
                distance: 0.,
                decay: 2.,
                angle: 0.5,
                penumbra: 0.5,
            }));
            let n = s.get_mut(light)?;
            n.position = position;
            n.cast_shadow = true;
            n.shadow.map_size = Some(1024);
        }
        let mut grass =
            decode_texture_image(&fetch(&format!("{ASSETS}/grasslight-big.jpg")).await?).await?;
        grass.srgb = true;
        grass.mipmap_filter = Some(Filter::Linear);
        grass.wrap_s = Wrapping::Repeat;
        grass.wrap_t = Wrapping::Repeat;
        grass.repeat = Vector2::new(8., 8.);
        let mut phong = MeshPhongMaterial::default();
        phong.properties.map = Some(Arc::new(grass));
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(20., 20., 1, 1)?),
            Arc::new(Material::Phong(phong)),
        )));
        let n = s.get_mut(ground)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.receive_shadow = true;
        // MD2Character: scale 0.03, the parts turned by −π/2 under the root.
        let scale = 0.03;
        let mut skins = vec![];
        for name in SKINS {
            skins.push(texture(&format!("{ASSETS}/ratamahatta/skins/{name}")).await?);
        }
        let root = s.insert(NodeKind::Group);
        let part = |s: &mut Scene, md2: Md2, skin: Arc<Texture>| -> Result<Part> {
            let targets = md2
                .geometry
                .morph_attributes
                .get("position")
                .map_or(0, Vec::len);
            let texture = lambert(Some(skin), false);
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(md2.geometry),
                texture.clone(),
            )));
            let n = s.get_mut(mesh)?;
            n.quaternion = Quaternion::from_rotation_y(-PI / 2.);
            n.scale = Vector3::splat(scale);
            n.cast_shadow = true;
            n.receive_shadow = true;
            n.morph_weights = vec![0.; targets];
            s.add(root, mesh)?;
            Ok(Part {
                mesh,
                clips: md2.clips,
                targets,
                texture,
                wireframe: lambert(None, true),
            })
        };
        let md2 = parse_md2(&fetch(&format!("{ASSETS}/ratamahatta/ratamahatta.md2")).await?)?;
        // root.position.y = −scale × the body's bounding-box minimum y.
        let min_y = match md2.geometry.attributes.get("position") {
            Some(Attribute::F32(a)) => a
                .array()
                .iter()
                .skip(1)
                .step_by(3)
                .fold(f32::INFINITY, |m, &v| m.min(v)),
            _ => 0.,
        };
        s.get_mut(root)?.position.y = -scale * min_y as f64;
        let clip = md2
            .clips
            .first()
            .map(|c| c.name.clone())
            .unwrap_or_default();
        let body = part(s, md2, skins[0].clone())?;
        let mut weapons = vec![];
        for name in WEAPONS {
            let md2 = parse_md2(&fetch(&format!("{ASSETS}/ratamahatta/{name}.md2")).await?)?;
            let skin = texture(&format!("{ASSETS}/ratamahatta/skins/{name}.png")).await?;
            let weapon = part(s, md2, skin)?;
            s.get_mut(weapon.mesh)?.visible = false;
            weapons.push(weapon);
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
        controls.update(s, c)?;
        let weapon = weapons.len() - 1;
        let mut d = Self {
            time: 0.,
            last: 0.,
            body,
            weapons,
            weapon,
            skins: skins
                .iter()
                .map(|t| lambert(Some(t.clone()), false))
                .collect(),
            pending: vec![],
            clip: String::new(),
            action: None,
            time_scale: 1.,
            wireframe: false,
            controls,
        };
        // onLoadComplete: setAnimation( the first clip ).
        d.set_animation(&clip);
        Ok(d)
    }
    /// setAnimation(): a new body action from time 0; the weapon follows.
    fn set_animation(&mut self, name: &str) {
        self.action = self.body.clips.iter().any(|c| c.name == name).then_some(0.);
        self.clip = name.to_string();
    }
    /// The parts' morph influences at the action time: each clip track writes
    /// its target; the stopped clips' targets are restored to zero.
    fn apply(part: &Part, s: &mut Scene, clip: &str, time: Option<f64>) -> Result<()> {
        let mut weights = vec![0.; part.targets];
        if let (Some(time), Some(clip)) = (time, part.clips.iter().find(|c| c.name == clip)) {
            for (target, track) in &clip.tracks {
                if let Some(w) = weights.get_mut(*target) {
                    *w = track.sample(time);
                }
            }
        }
        s.get_mut(part.mesh)?.morph_weights = weights;
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// character.update( delta ): the mixer advances the body action (and the
    /// weapon action synced to it) by delta × timeScale, looping.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        for (index, value) in std::mem::take(&mut self.pending) {
            self.apply_parameter(s, index, value)?;
        }
        let delta = self.time - self.last;
        self.last = self.time;
        if let Some(time) = &mut self.action {
            let duration = self
                .body
                .clips
                .iter()
                .find(|c| c.name == self.clip)
                .map_or(0., |c| c.duration);
            *time += delta * self.time_scale;
            if duration > 0. {
                *time -= duration * (*time / duration).floor();
            }
        }
        Self::apply(&self.body, s, &self.clip, self.action)?;
        let weapon = &self.weapons[self.weapon];
        Self::apply(weapon, s, &self.clip, self.action)?;
        Ok(())
    }
    fn set_material(s: &mut Scene, part: &Part, wireframe: bool) -> Result<()> {
        if let NodeKind::Mesh(m) = &mut s.get_mut(part.mesh)?.kind {
            m.materials[0] = if wireframe {
                part.wireframe.clone()
            } else {
                part.texture.clone()
            };
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
        let camera: CameraState = camera_state(s, c)?;
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
        if index >= 2 + self.skins.len() + self.weapons.len() + self.body.clips.len() {
            return Err(Error::Invalid("md2 parameter"));
        }
        self.pending.push((index, value));
        Ok(())
    }
    /// The GUI: speed, wireframe, then the skin, weapon and animation buttons.
    fn apply_parameter(&mut self, s: &mut Scene, index: usize, value: f32) -> Result<()> {
        let skins = self.skins.len();
        let weapons = self.weapons.len();
        match index {
            0 => {
                // setPlaybackRate: timeScale = 1 / rate.
                self.time_scale = if value != 0. { 1. / value as f64 } else { 0. };
            }
            1 => {
                self.wireframe = value > 0.5;
                Self::set_material(s, &self.body, self.wireframe)?;
                Self::set_material(s, &self.weapons[self.weapon], self.wireframe)?;
            }
            i if i < 2 + skins => {
                // setSkin: only while the body shows its textured material.
                if !self.wireframe {
                    self.body.texture = self.skins[i - 2].clone();
                    Self::set_material(s, &self.body, false)?;
                }
            }
            i if i < 2 + skins + weapons => {
                for w in &self.weapons {
                    s.get_mut(w.mesh)?.visible = false;
                }
                self.weapon = i - 2 - skins;
                s.get_mut(self.weapons[self.weapon].mesh)?.visible = true;
                Self::set_material(s, &self.weapons[self.weapon], self.wireframe)?;
            }
            i if i < 2 + skins + weapons + self.body.clips.len() => {
                let name = self.body.clips[i - 2 - skins - weapons].name.clone();
                self.set_animation(&name);
            }
            _ => return Err(Error::Invalid("md2 parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
