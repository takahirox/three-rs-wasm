//! ColladaLoader scene graphs: the skinned, animated stormtrooper and the
//! kinematics robot tweened between random joint values.
use super::super::controls_attributes::{Controls, camera_state};
use super::super::gltf_viewer::fetch;
use super::super::interactive_scenes::grid_helper;
use super::dae_object;
use super::formats::{
    DaeScene, DaeTransform, multiply_matrices, parse_collada, rotation_axis, transform_matrix,
};
use crate::animation::{AnimationMixer, Clip, Interpolation, Property, Track};
use crate::{Error, Result, camera::*, deformation::Skin, math::*, renderer::*, scene::*};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
/// MathUtils.randInt( low, high ).
fn rand_int(seed: &mut u32, low: f64, high: f64) -> f64 {
    low + (random(seed) * (high - low + 1.)).floor()
}
fn place(s: &mut Scene, h: Object3D, matrix: &[f64; 16]) -> Result<()> {
    let (scale, rotation, translation) =
        Matrix4::from_cols_array(matrix).to_scale_rotation_translation();
    let n = s.get_mut(h)?;
    (n.position, n.quaternion, n.scale) = (translation, rotation, scale);
    Ok(())
}
/// The composed scene: the visual scene group and each node's object.
struct Built {
    root: Object3D,
    nodes: Vec<Object3D>,
}
/// ColladaComposer.buildVisualScene and buildNode: a node with no child nodes
/// and one object is that object; others are a Group (a Bone for joints) of
/// the child nodes, then the instanced objects. Skins bind their skeleton.
async fn compose(s: &mut Scene, dae: &mut DaeScene, base: &str) -> Result<Built> {
    let mut textures = HashMap::new();
    let mut objects: Vec<Option<Object3D>> = vec![None; dae.meshes.len()];

    let mut skins = vec![];
    for (i, mesh) in std::mem::take(&mut dae.meshes).into_iter().enumerate() {
        let skin = mesh
            .skin
            .as_ref()
            .map(|k| (k.joints.clone(), k.skeletons.clone()));
        objects[i] = Some(dae_object(s, mesh, &mut textures, base, &[]).await?);
        if let Some(skin) = skin {
            skins.push((i, skin));
        }
    }
    let mut handles: Vec<Option<Object3D>> = vec![None; dae.nodes.len()];
    // Children before parents: nodes are stored in document (pre-)order.
    for index in (0..dae.nodes.len()).rev() {
        let node = &dae.nodes[index];
        let mut kids: Vec<Object3D> = node.children.iter().filter_map(|&c| handles[c]).collect();
        for &m in &node.meshes {
            kids.extend(objects[m]);
        }
        let h = if node.children.is_empty() && kids.len() == 1 {
            kids[0]
        } else {
            let g = s.insert(NodeKind::Group);
            for k in kids {
                s.add(g, k)?;
            }
            g
        };
        s.get_mut(h)?.name = if node.joint {
            node.sid.clone()
        } else {
            node.name.clone()
        };
        place(s, h, &node.matrix)?;
        handles[index] = Some(h);
    }
    let nodes: Vec<Object3D> = handles
        .into_iter()
        .map(|h| h.ok_or(Error::Invalid("Collada node")))
        .collect::<Result<_>>()?;
    let root = s.insert(NodeKind::Group);
    for &r in &dae.roots {
        s.add(root, nodes[r])?;
    }
    // buildSkeleton: the skeleton roots' bones in traversal order, sorted by the
    // skin's joint order, unmatched bones appended; then bind.
    for (mesh, (joints, skeletons)) in skins {
        let mut bones: Vec<(Object3D, Matrix4)> = vec![];
        for skeleton in &skeletons {
            let Some(index) = dae.nodes.iter().position(|n| n.id == *skeleton) else {
                continue;
            };
            for h in s.traverse(nodes[index], false)? {
                if let Some(i) = nodes.iter().position(|&n| n == h)
                    && dae.nodes[i].joint
                {
                    let name = &dae.nodes[i].sid;
                    let inverse = joints
                        .iter()
                        .find(|(n, _)| n == name)
                        .map(|(_, m)| Matrix4::from_cols_array(m))
                        .unwrap_or(Matrix4::IDENTITY);
                    bones.push((h, inverse));
                }
            }
        }
        let mut sorted: Vec<Option<(Object3D, Matrix4)>> = vec![None; joints.len()];
        let mut processed = vec![false; bones.len()];
        for (i, (name, _)) in joints.iter().enumerate() {
            if let Some(j) = bones
                .iter()
                .position(|(h, _)| s.get(*h).is_ok_and(|n| n.name == *name))
            {
                sorted[i] = Some(bones[j]);
                processed[j] = true;
            }
        }
        let mut list: Vec<(Object3D, Matrix4)> = sorted.into_iter().flatten().collect();
        for (j, b) in bones.iter().enumerate() {
            if !processed[j] {
                list.push(*b);
            }
        }
        let h = objects[mesh].ok_or(Error::Invalid("skinned mesh"))?;
        s.get_mut(h)?.skin = Some(Skin {
            joints: list.iter().map(|b| b.0).collect(),
            inverse_bind_matrices: list.iter().map(|b| b.1).collect(),
        });
    }
    // The loader turns Z_UP assets upright and scales by the unit.
    let n = s.get_mut(root)?;
    if dae.z_up {
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
    }
    n.scale = Vector3::splat(dae.unit);
    Ok(Built { root, nodes })
}
/// buildMatrixTracks: each matrix key decomposed into position, quaternion and
/// scale tracks on the node, all linearly interpolated.
fn matrix_clip(dae: &DaeScene, nodes: &[Object3D]) -> Result<Clip> {
    let mut tracks = vec![];
    for channel in &dae.channels {
        let Some(index) = dae.nodes.iter().position(|n| n.id == channel.node) else {
            continue;
        };
        let is_matrix = dae.nodes[index].transforms.iter().any(|(sid, t)| {
            sid.as_deref() == Some(&channel.sid) && matches!(t, DaeTransform::Matrix(_))
        });
        if !is_matrix || channel.stride != 16 {
            continue;
        }
        let mut keys: Vec<(f64, [f64; 16])> = channel
            .times
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                let v = &channel.values[i * 16..i * 16 + 16];
                (t, std::array::from_fn(|k| v[(k % 4) * 4 + k / 4]))
            })
            .collect();
        keys.sort_by(|a, b| a.0.total_cmp(&b.0));
        let (mut p, mut q, mut sc) = (vec![], vec![], vec![]);
        for (_, m) in &keys {
            let (scale, rotation, translation) =
                Matrix4::from_cols_array(m).to_scale_rotation_translation();
            p.push(translation.to_array().to_vec());
            q.push(rotation.to_array().to_vec());
            sc.push(scale.to_array().to_vec());
        }
        let times: Vec<f64> = keys.iter().map(|k| k.0).collect();
        for (property, values) in [
            (Property::Position, p),
            (Property::Rotation, q),
            (Property::Scale, sc),
        ] {
            tracks.push(Track {
                target: nodes[index],
                property,
                times: times.clone(),
                values,
                interpolation: Interpolation::Linear,
            });
        }
    }
    Ok(Clip {
        name: "default".into(),
        tracks,
    })
}
/// A kinematics joint bound to its visual node.
struct KinematicsJoint {
    object: Object3D,
    transforms: Vec<(Option<String>, DaeTransform)>,
    index: String,
    revolute: bool,
    axis: [f64; 3],
    min: f64,
    max: f64,
}
/// The kinematics tween: the joint values at its start and its targets.
struct Tween {
    start_time: f64,
    duration: f64,
    from: Vec<f64>,
    to: Vec<f64>,
}
struct Robot {
    joints: Vec<KinematicsJoint>,
    /// tweenParameters, per non-static joint.
    values: Vec<Option<f64>>,
    tween: Option<Tween>,
    next_setup: f64,
    seed: u32,
}
impl Robot {
    /// setJointValue: the node's transforms with the joint's rotation or translation.
    fn set(&self, s: &mut Scene, j: &KinematicsJoint, value: f64) -> Result<()> {
        if value > j.max || value < j.min || j.min >= j.max {
            return Ok(());
        }
        let mut matrix = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ];
        for (sid, t) in &j.transforms {
            let m = if sid.as_deref().is_some_and(|sid| sid.contains(&j.index)) {
                if j.revolute {
                    rotation_axis(j.axis, value.to_radians())
                } else {
                    transform_matrix(&DaeTransform::Translate(j.axis.map(|a| a * value)))
                }
            } else {
                transform_matrix(t)
            };
            matrix = multiply_matrices(&matrix, &m);
        }
        place(s, j.object, &matrix)
    }
    /// setupTween: a random duration and random integer targets from the current values.
    fn setup(&mut self, time: f64) {
        let duration = rand_int(&mut self.seed, 1000., 5000.);
        let mut from = vec![];
        let mut to = vec![];
        for (k, j) in self.joints.iter().enumerate() {
            if j.min >= j.max {
                from.push(0.);
                to.push(0.);
                continue;
            }
            // `old ? old : zeroPosition` ( zero ).
            let position = self.values[k].unwrap_or(0.);
            self.values[k] = Some(position);
            from.push(position);
            to.push(rand_int(&mut self.seed, j.min, j.max));
        }
        self.tween = Some(Tween {
            start_time: time,
            duration,
            from,
            to,
        });
        self.next_setup = time + duration;
    }
    /// TWEEN.update( time ) with Quadratic.Out easing, then onUpdate.
    fn update(&mut self, s: &mut Scene, time: f64) -> Result<()> {
        // The example-clock setTimeout: setupTween from the last updated values.
        while time >= self.next_setup {
            let at = self.next_setup;
            self.setup(at);
        }
        let Some(t) = &self.tween else {
            return Ok(());
        };
        if time < t.start_time {
            return Ok(());
        }
        let e = if t.duration == 0. {
            1.
        } else {
            ((time - t.start_time) / t.duration).min(1.)
        };
        let eased = e * (2. - e);
        let values: Vec<f64> = (0..self.joints.len())
            .map(|k| t.from[k] + (t.to[k] - t.from[k]) * eased)
            .collect();
        for (k, j) in self.joints.iter().enumerate() {
            if j.min < j.max {
                self.values[k] = Some(values[k]);
                self.set(s, j, values[k])?;
            }
        }
        Ok(())
    }
}
pub(in crate::browser) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    controls: Option<Controls>,
    mixer: Option<AnimationMixer>,
    robot: Option<Robot>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            316 => (25., 1., 1000., Vector3::new(15., 10., -15.)),
            _ => (45., 1., 2000., Vector3::new(2., 2., 3.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = position;
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            controls: None,
            mixer: None,
            robot: None,
        };
        match id {
            316 => d.skinning(s, c).await?,
            _ => d.kinematics(s).await?,
        }
        Ok(d)
    }
    async fn skinning(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let base = format!("{ASSETS}/collada-anim/stormtrooper");
        let mut dae = parse_collada(&String::from_utf8_lossy(
            &fetch(&format!("{base}/stormtrooper.dae")).await?,
        ))?;
        let built = compose(s, &mut dae, &base).await?;
        let clip = matrix_clip(&dae, &built.nodes)?;
        let mut mixer = AnimationMixer::default();
        mixer.play(Arc::new(clip))?;
        self.mixer = Some(mixer);
        s.insert(NodeKind::Line(grid_helper(10., 20, 0xc1c1c1, 0x8d8d8d)?));
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.6,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(1.5, 1., -1.5);
        // OrbitControls: screen-space panning, distance 5 to 40, target ( 0, 2, 0 ).
        let mut controls = Controls::new(None, (5., 40.), PI, true);
        controls.set_target(Vector3::new(0., 2., 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        let _ = built.root;
        Ok(())
    }
    async fn kinematics(&mut self, s: &mut Scene) -> Result<()> {
        let mut dae = parse_collada(&String::from_utf8_lossy(
            &fetch(&format!("{ASSETS}/collada-anim/abb_irb52_7_120.dae")).await?,
        ))?;
        // setupKinematics: each bound axis's node, found by its transform sid.
        let mut bound = vec![];
        for (joint, target) in &dae.binds {
            let Some(index) = dae.nodes.iter().position(|n| {
                n.transforms
                    .iter()
                    .any(|(sid, _)| sid.as_deref() == Some(target))
            }) else {
                continue;
            };
            bound.push((joint.clone(), index));
        }
        let joints_data: Vec<_> = dae
            .joints
            .iter()
            .map(|(sid, j)| (sid.clone(), j.revolute, j.axis, j.min, j.max))
            .collect();
        let names: Vec<String> = dae.nodes.iter().map(|n| n.name.clone()).collect();
        let transforms: Vec<_> = dae.nodes.iter().map(|n| n.transforms.clone()).collect();
        let built = compose(s, &mut dae, ASSETS).await?;
        // collada.scene.scale = 10.
        s.get_mut(built.root)?.scale = Vector3::splat(10.);
        let mut joints = vec![];
        for (sid, revolute, axis, min, max) in joints_data {
            let Some(&(_, index)) = bound.iter().find(|(j, _)| *j == sid) else {
                continue;
            };
            // The last node of that name in traversal order.
            let name = &names[index];
            let object = names
                .iter()
                .rposition(|n| n == name)
                .map(|i| built.nodes[i])
                .ok_or(Error::Invalid("kinematics node"))?;
            joints.push(KinematicsJoint {
                object,
                transforms: transforms[index].clone(),
                index: sid,
                revolute,
                axis,
                min,
                max,
            });
        }
        s.insert(NodeKind::Line(grid_helper(20., 20, 0xc1c1c1, 0x8d8d8d)?));
        let light = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0xfff7f7),
            ground: Color::from_hex(0x494966),
            intensity: 3.,
        }));
        s.get_mut(light)?.position = Vector3::Y;
        let count = joints.len();
        let mut robot = Robot {
            joints,
            values: vec![None; count],
            tween: None,
            next_setup: f64::INFINITY,
            seed: 186,
        };
        robot.setup(0.);
        self.robot = Some(robot);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let t = self.time;
        let delta = t - self.last;
        self.last = t;
        if let Some(mixer) = &mut self.mixer {
            mixer.update(s, delta.max(0.))?;
        }
        if let Some(robot) = &mut self.robot {
            robot.update(s, t * 1000.)?;
            // render(): the camera circles at Date.now() × 0.0001.
            let timer = t * 0.1;
            s.get_mut(c)?.position = Vector3::new(timer.cos() * 20., 10., timer.sin() * 20.);
            s.look_at(c, Vector3::new(0., 5., 0.))?;
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
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        let _ = self.id;
        Err(Error::Invalid("Collada animation parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
