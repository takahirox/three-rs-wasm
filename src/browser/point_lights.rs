//! Port of r186 webgpu_lights_pointlights: asset-specific OBJ import and CPU
//! equivalent of its tetrahedron displacement. No general-purpose loader API.
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    scene::*,
};
use std::sync::Arc;
use wasm_bindgen::JsCast;

pub(super) struct PointLights {
    base: Vec<Vector3>,
    normals: Vec<Vector3>,
    phases: Vec<(f64, f64)>,
    lights: [Object3D; 2],
    pub time: f64,
    pub amount: f64,
    pub speed: f64,
    yaw: f64,
    pitch: f64,
    radius: f64,
}
impl PointLights {
    pub async fn create(
        scene: &mut Scene,
        camera: Object3D,
        mesh: Object3D,
        aspect: f64,
    ) -> Result<Self> {
        use wasm_bindgen_futures::JsFuture;
        let response = JsFuture::from(
            web_sys::window()
                .ok_or(Error::Invalid("window"))?
                .fetch_with_str("/web/models/WaltHead.obj"),
        )
        .await
        .map_err(|e| Error::Asset(format!("{e:?}")))?
        .dyn_into::<web_sys::Response>()
        .map_err(|_| Error::Invalid("model response"))?;
        if !response.ok() {
            return Err(Error::Asset(format!("model HTTP {}", response.status())));
        }
        let source = JsFuture::from(
            response
                .text()
                .map_err(|e| Error::Asset(format!("{e:?}")))?,
        )
        .await
        .map_err(|e| Error::Asset(format!("{e:?}")))?
        .as_string()
        .ok_or(Error::Invalid("model text"))?;
        let mut vertices = Vec::new();
        let mut base = Vec::new();
        let mut normals = Vec::new();
        let mut phases = Vec::new();
        let mut seed = 186_u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.0
        };
        for line in source.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            match fields.first().copied() {
                Some("v") if fields.len() == 4 => {
                    let mut p = [0.0; 3];
                    for (i, value) in p.iter_mut().enumerate() {
                        *value = fields[i + 1]
                            .parse::<f64>()
                            .map_err(|_| Error::Invalid("OBJ vertex"))?;
                        if !value.is_finite() {
                            return Err(Error::Invalid("OBJ vertex"));
                        }
                    }
                    vertices.push(Vector3::from_array(p));
                }
                Some("f") => {
                    if fields.len() != 4 {
                        return Err(Error::Invalid("Walt model triangle"));
                    }
                    let mut v = [Vector3::ZERO; 3];
                    for i in 0..3 {
                        let index = fields[i + 1]
                            .split('/')
                            .next()
                            .and_then(|n| n.parse::<usize>().ok())
                            .and_then(|n| n.checked_sub(1))
                            .ok_or(Error::Invalid("OBJ index"))?;
                        v[i] = *vertices.get(index).ok_or(Error::Invalid("OBJ index"))?;
                    }
                    let normal = (v[1] - v[0]).cross(v[2] - v[0]).normalize_or_zero();
                    let center = (v[0] + v[1] + v[2]) / 3.0 - normal;
                    base.extend([
                        v[0], v[1], v[2], center, v[1], v[0], center, v[2], v[1], center, v[0],
                        v[2],
                    ]);
                    normals.push(normal);
                    phases.push((random(), random()));
                }
                _ => {}
            }
        }
        if base.is_empty() {
            return Err(Error::Invalid("empty Walt model"));
        }
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(
                base.iter().flat_map(|v| v.as_vec3().to_array()).collect(),
                3,
                false,
            )?),
        );
        geometry.compute_vertex_normals()?;
        let material = MeshStandardMaterial {
            roughness: 0.4,
            ..Default::default()
        };
        let node = scene.get_mut(mesh)?;
        node.kind = NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Standard(material)),
        ));
        node.position = Vector3::new(0.0, -30.0, 0.0);
        node.scale = Vector3::splat(0.8);
        node.frustum_culled = false;
        scene.background = Color::BLACK;
        scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.0,
            aspect,
            near: 1.0,
            far: 1000.0,
            ..Default::default()
        }));
        scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, 100.0);
        scene.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xaaaaaa),
            intensity: 0.1,
        }));
        let sphere = Arc::new(SphereGeometry::build(0.5, 16, 8)?);
        let mut lights = Vec::new();
        for color in [0xff0040, 0x0040ff] {
            let light = scene.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(color),
                intensity: 2000.0,
                distance: 0.0,
                decay: 2.0,
            }));
            let mut material = Material::default();
            material.properties_mut().color = Color::from_hex(color);
            let marker = scene.insert(NodeKind::Mesh(Mesh::new(
                sphere.clone(),
                Arc::new(material),
            )));
            scene.add(light, marker)?;
            lights.push(light);
        }
        Ok(Self {
            base,
            normals,
            phases,
            lights: [lights[0], lights[1]],
            time: 0.0,
            amount: 1.0,
            speed: 1.0,
            yaw: 0.0,
            pitch: 0.0,
            radius: 100.0,
        })
    }
    pub fn orbit(&mut self, dx: f64, dy: f64, zoom: f64) {
        self.yaw -= dx * 0.008;
        self.pitch = (self.pitch + dy * 0.008).clamp(-1.3, 1.3);
        self.radius = (self.radius * (zoom * 0.001).exp()).clamp(55.0, 220.0);
    }
    pub fn update(&mut self, scene: &mut Scene, camera: Object3D, mesh: Object3D) -> Result<()> {
        let t = self.time * 0.5;
        let positions = [
            Vector3::new(
                t.sin() * 20.0,
                (t * 0.75).cos() * -30.0,
                (t * 0.5).cos() * 20.0,
            ),
            Vector3::new(
                (t * 0.5).cos() * 20.0,
                (t * 0.75).sin() * -30.0,
                t.sin() * 20.0,
            ),
        ];
        for (light, position) in self.lights.iter().zip(positions) {
            scene.get_mut(*light)?.position = position;
        }
        scene.get_mut(camera)?.position = Vector3::new(
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.cos() * self.pitch.cos(),
        ) * self.radius;
        scene.look_at(camera, Vector3::ZERO)?;
        let effectors = positions.map(|p| (p + Vector3::new(0.0, 30.0, 0.0)) / 0.8);
        let NodeKind::Mesh(mesh) = &mut scene.get_mut(mesh)?.kind else {
            return Err(Error::Invalid("head mesh"));
        };
        let geometry = Arc::make_mut(&mut mesh.geometry);
        let Some(Attribute::F32(attribute)) = geometry.attributes.get_mut("position") else {
            return Err(Error::Invalid("head positions"));
        };
        let array = attribute.array_mut();
        for (face, base) in self.base.chunks_exact(12).enumerate() {
            let (phase, seed) = self.phases[face];
            let wave = (((phase + self.time) * 2.0 + seed).sin() * 0.5).abs();
            for (corner, p) in base.iter().enumerate() {
                let distance_effect = effectors
                    .iter()
                    .map(|light| (20.0 - p.distance(*light)).max(0.0) / 2.0)
                    .sum::<f64>();
                let displaced = (*p + self.normals[face] * (wave + distance_effect) * self.amount)
                    .as_vec3()
                    .to_array();
                let offset = (face * 12 + corner) * 3;
                array[offset..offset + 3].copy_from_slice(&displaced);
            }
        }
        Ok(())
    }
}
