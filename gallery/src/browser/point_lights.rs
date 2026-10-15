//! Port of r186 webgpu_lights_pointlights with GPU vertex displacement.
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    scene::*,
};
use std::sync::Arc;

pub(super) struct PointLights {
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
        renderer: &crate::renderer::Renderer,
    ) -> Result<Self> {
        let bytes = super::gltf_viewer::fetch("/web/models/WaltHead.obj").await?;
        let source = std::str::from_utf8(&bytes).map_err(|_| Error::Invalid("model text"))?;
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
        geometry.set_attribute(
            "uv",
            Attribute::F32(BufferAttribute::new(
                (0..base.len())
                    .flat_map(|i| [(i / 12) as f32, 0.0])
                    .collect(),
                2,
                false,
            )?),
        );
        let face_data: Vec<f32> = normals
            .iter()
            .zip(&phases)
            .flat_map(|(n, (phase, seed))| {
                [
                    n.x as f32,
                    n.y as f32,
                    n.z as f32,
                    *phase as f32,
                    *seed as f32,
                    0.0,
                    0.0,
                    0.0,
                ]
            })
            .collect();
        let buffer = crate::compute::GpuBuffer::new(
            renderer,
            bytemuck::cast_slice(&face_data),
            crate::compute::BufferAccess::Read,
        )?;
        let program=Arc::new(crate::shader::ShaderProgram::new(renderer,r#"
            @group(1) @binding(0) var<storage,read> faces:array<vec4<f32>>;
            fn deform(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{
                let i=u32(uv.x)*2u;let face=faces[i];let seed=faces[i+1u].x;
                let wave=abs(sin((face.w+u.custom[0].x)*2.0+seed)*0.5);
                let distance_effect=(max(20.0-distance(position,u.custom[1].xyz),0.0)+max(20.0-distance(position,u.custom[2].xyz),0.0))*0.5;
                return position+face.xyz*(wave+distance_effect)*u.custom[0].y;
            }
            fn shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>{return base;}
        "#,&[&buffer]).await?);
        let mut material = MeshStandardMaterial {
            roughness: 0.4,
            ..Default::default()
        };
        material.properties.vertex_program = Some(program);
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
        let material = Arc::make_mut(&mut mesh.materials[0]).properties_mut();
        material.vertex_uniforms[0] = [self.time as f32, self.amount as f32, 0.0, 0.0];
        material.vertex_uniforms[1] = effectors[0].extend(0.0).as_vec4().to_array();
        material.vertex_uniforms[2] = effectors[1].extend(0.0).as_vec4().to_array();
        Ok(())
    }
}
