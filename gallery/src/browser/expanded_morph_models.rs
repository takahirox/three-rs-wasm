use super::gltf_viewer::{decode_image, fetch, load_asset};
use crate::{Result, animation::AnimationMixer, camera::*, material::*, math::*, scene::*};
use std::sync::Arc;

pub(super) struct Horse {
    mixer: AnimationMixer,
    pub theta: f64,
}
impl Horse {
    pub async fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<Self> {
        scene.background = Color::from_hex(0xf0f0f0);
        scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.0,
            aspect,
            near: 1.0,
            far: 10000.0,
            ..Default::default()
        }));
        for (color, sign) in [(0xefefff, 1.0), (0xffefef, -1.0)] {
            let light = scene.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(color),
                intensity: 5.0,
                target: Vector3::ZERO,
            }));
            scene.get_mut(light)?.position = Vector3::splat(sign).normalize();
        }
        let (asset, buffers, images) = load_asset("/web/models/Horse/Horse.glb").await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
        let root = scene.get_mut(instance.roots[0])?;
        root.scale = Vector3::splat(1.5);
        root.matrix_auto_update = true;
        let mut mixer = AnimationMixer::default();
        let action = mixer.play(instance.clips[0].clone())?;
        mixer.actions[action].time_scale = instance.clips[0].duration();
        Ok(Self { mixer, theta: 0.0 })
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        seconds: f64,
        animate: bool,
    ) -> Result<()> {
        if animate {
            self.theta += 0.1;
        }
        let theta = self.theta.to_radians();
        scene.get_mut(camera)?.position =
            Vector3::new(600.0 * theta.sin(), 300.0, 600.0 * theta.cos());
        scene.look_at(camera, Vector3::new(0.0, 150.0, 0.0))?;
        self.mixer.actions[0].time = seconds * self.mixer.actions[0].time_scale;
        self.mixer.update(scene, 0.0)
    }
}
pub(super) struct Sphere {
    root: Object3D,
    mesh: Object3D,
    points: Object3D,
    rotation_x: f64,
    rotation_y: f64,
}
impl Sphere {
    pub async fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<Self> {
        scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.0,
            aspect,
            near: 0.2,
            far: 100.0,
            ..Default::default()
        }));
        for (color, intensity, sign) in [(0xff2200, 50000.0, 1.0), (0x22ff00, 10000.0, -1.0)] {
            let light = scene.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(color),
                intensity,
                distance: 0.0,
                decay: 2.0,
            }));
            scene.get_mut(light)?.position = Vector3::splat(100.0 * sign);
        }
        scene.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x111111),
            intensity: 1.0,
        }));
        let (asset, buffers, images) =
            load_asset("/web/models/AnimatedMorphSphere/AnimatedMorphSphere.gltf").await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
        let root = instance.roots[0];
        let mesh = instance.meshes[0];
        let (rotation_x, rotation_y, _) = scene.get(root)?.quaternion.to_euler(glam::EulerRot::XYZ);
        scene.get_mut(root)?.matrix_auto_update = true;
        let geometry = scene.get(mesh)?.geometry().unwrap().clone();
        let mut material = PointsMaterial {
            size: 10.0,
            size_attenuation: false,
            ..Default::default()
        };
        let mut map = decode_image(&fetch("/web/gallery/assets/disc.png").await?).await?;
        map.srgb = false;
        map.mipmap_filter = Some(Filter::Linear);
        material.properties.map = Some(Arc::new(map));
        material.properties.alpha_test = 0.5;
        let points = scene.insert(NodeKind::Points(Points {
            geometry,
            material: Arc::new(Material::Points(material)),
        }));
        scene.get_mut(points)?.morph_weights = scene.get(mesh)?.morph_weights.clone();
        scene.add(mesh, points)?;
        Ok(Self {
            root,
            mesh,
            points,
            rotation_x,
            rotation_y,
        })
    }
    pub fn update(&self, scene: &mut Scene, seconds: f64) -> Result<()> {
        scene.get_mut(self.root)?.quaternion = Quaternion::from_euler(
            glam::EulerRot::XYZ,
            self.rotation_x,
            self.rotation_y + seconds * 0.5,
            std::f64::consts::FRAC_PI_2,
        );
        let phase = (seconds * 0.5).rem_euclid(2.0);
        let weight = phase.min(2.0 - phase);
        for handle in [self.mesh, self.points] {
            scene.get_mut(handle)?.morph_weights[1] = weight;
        }
        Ok(())
    }
}
