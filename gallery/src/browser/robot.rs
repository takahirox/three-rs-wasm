//! Original RobotExpressive animation scene; all model evaluation stays in Rust.
use super::gltf_viewer::{OrbitViewer, fetch};
use crate::{
    Result, animation::*, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    scene::*,
};
use std::sync::Arc;
pub(super) struct Robot {
    pub viewer: OrbitViewer,
    pub mixer: AnimationMixer,
    pub selected: usize,
}
impl Robot {
    pub async fn create(scene: &mut Scene, camera: Object3D, aspect: f64) -> Result<Self> {
        scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.0,
            aspect,
            near: 0.25,
            far: 100.0,
            ..Default::default()
        }));
        scene.background = Color::from_hex(0xe0e0e0);
        scene.fog = Some(Fog::Linear {
            color: scene.background,
            near: 20.0,
            far: 100.0,
        });
        let hemi = scene.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x8d8d8d),
            intensity: 3.0,
        }));
        scene.get_mut(hemi)?.position.y = 20.0;
        let light = scene.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.0,
            target: Vector3::ZERO,
        }));
        scene.get_mut(light)?.position = Vector3::new(0.0, 20.0, 10.0);
        let mut material = MeshPhongMaterial::default();
        material.properties.color = Color::from_hex(0xcbcbcb);
        material.properties.depth_write = false;
        let floor = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2000.0, 2000.0, 1, 1)?),
            Arc::new(Material::Phong(material)),
        )));
        scene.get_mut(floor)?.quaternion =
            Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        let mut lines = Vec::new();
        for i in 0..=40 {
            let p = -100.0 + i as f32 * 5.0;
            lines.extend([-100.0, 0.0, p, 100.0, 0.0, p, p, 0.0, -100.0, p, 0.0, 100.0]);
        }
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(lines, 3, false)?),
        );
        let mut material = LineBasicMaterial::default();
        material.properties.color = Color::BLACK;
        material.properties.transparent = true;
        material.properties.opacity = 0.2;
        scene.insert(NodeKind::Line(Line {
            geometry: Arc::new(geometry),
            material: Arc::new(Material::Line(material)),
            segments: true,
        }));
        let bytes = fetch("/web/models/RobotExpressive/RobotExpressive.glb").await?;
        let asset =
            gltf::Gltf::from_slice(&bytes).map_err(|e| crate::Error::Asset(e.to_string()))?;
        let imported = crate::gltf::import_animated(
            &asset,
            &[asset
                .blob
                .clone()
                .ok_or(crate::Error::Invalid("Robot GLB buffer"))?],
            &[],
        )?;
        let instance = imported.instantiate(scene)?;
        let mut mixer = AnimationMixer::default();
        let mut selected = 0;
        for clip in instance.clips {
            let walking = clip.name == "Walking";
            let index = mixer.play(clip)?;
            mixer.actions[index].weight = if walking { 1.0 } else { 0.0 };
            if walking {
                selected = index;
            }
        }
        mixer.update(scene, 0.0)?;
        let offset = Vector3::new(-5.0, 1.0, 10.0);
        let mut viewer = OrbitViewer::from_camera(Vector3::new(0.0, 2.0, 0.0), offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        Ok(Self {
            viewer,
            mixer,
            selected,
        })
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        self.mixer
            .update(scene, if animate { delta } else { 0.0 })?;
        self.viewer.update(scene, camera)?;
        if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(camera)?.kind {
            c.near = 0.25;
            c.far = 100.0;
        }
        Ok(())
    }
    pub fn select(&mut self, index: usize) -> Result<()> {
        if index >= self.mixer.actions.len() {
            return Err(crate::Error::Invalid("animation selection"));
        }
        for action in &mut self.mixer.actions {
            action.fade_to(0.0, 0.2)?;
        }
        let action = &mut self.mixer.actions[index];
        action.time = 0.0;
        action.fade_to(1.0, 0.2)?;
        self.selected = index;
        Ok(())
    }
}
