use super::super::gltf_viewer::fetch;
use super::*;
use crate::{
    compute::{BufferAccess, GpuBuffer},
    tsl::{
        compute::{BufferCompute, BufferStore},
        sprites::SpriteNodeMaterial,
    },
};
const COUNT: u32 = 50000;
pub(in crate::browser) struct Rain {
    viewer: OrbitViewer,
    time: f64,
    previous: f64,
    delta: f64,
    pending: bool,
    collision: RenderTarget,
    camera: Object3D,
    colliders: Vec<(Object3D, Arc<Material>, Arc<Material>)>,
    init: BufferCompute,
    compute: BufferCompute,
    initialized: bool,
    obstacle: Object3D,
    monkey: Object3D,
    particles: [Object3D; 2],
    parameters: [f32; 3],
}
fn store(binding: usize, value: tsl::Node) -> BufferStore {
    BufferStore {
        binding,
        index: instance_index(),
        value,
    }
}
impl Rain {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            aspect,
            near: 0.1,
            far: 110.,
            ..Default::default()
        }));
        let p = Vector3::new(40., 8., 0.);
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, p.length());
        viewer.fixture(p.x.atan2(p.z), (p.y / p.length()).asin(), 1.8);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 0.5,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(3., 17., 17.);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x111111),
            intensity: 1.,
        }));
        let camera = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -50.,
            right: 50.,
            top: 50.,
            bottom: -50.,
            near: 0.1,
            far: 50.,
            ..Default::default()
        })));
        s.get_mut(camera)?.position.y = 50.;
        s.get_mut(camera)?.layers.set(1);
        s.look_at(camera, Vector3::ZERO)?;
        let collision = RenderTarget::with_options(
            &r.device,
            1024,
            1024,
            RenderTargetOptions {
                format: HDR,
                ..Default::default()
            },
        )?;
        let mut buffers = Vec::new();
        for _ in 0..4 {
            buffers.push(GpuBuffer::zeroed(
                r,
                COUNT as u64 * 16,
                BufferAccess::ReadWrite,
            )?);
        }
        let bindings = buffers.iter().map(|b| (b, Type::Vec4)).collect::<Vec<_>>();
        let mut seed = 186u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            ((seed as f64 / 4294967296.) * 0xffffff as f64).round() as u32
        };
        let sy = random();
        let sz = random();
        let respawn = random();
        let rx = hash(instance_index());
        let ry = hash(instance_index() + uint(sy));
        let rz = hash(instance_index() + uint(sz));
        let pos = vec3(
            rx.clone() * float(100.) - float(50.),
            ry.clone() * float(25.),
            rz.clone() * float(100.) - float(50.),
        );
        let init = BufferCompute::new(
            r,
            COUNT,
            &bindings,
            &[
                store(0, vec4(pos, float(0.))),
                store(
                    1,
                    vec4(
                        vec3(float(0.), rx * float(-0.04) - float(0.2), float(0.)),
                        float(0.),
                    ),
                ),
                store(
                    2,
                    vec4(
                        vec3(
                            rz * float(100.) - float(50.),
                            float(-1.),
                            ry * float(100.) - float(50.),
                        ),
                        float(0.),
                    ),
                ),
                store(3, vec4(vec3(float(1000.), float(0.), float(0.)), float(0.))),
            ],
        )
        .await?;
        let pos = storage_element(0, instance_index()) + storage_element(1, instance_index());
        let floor = tsl::Texture::External(0)
            .sample_level((pos.swizzle("xz") + float(50.)) / float(100.), float(0.))
            .y()
            + float(0.05);
        let hit = (pos.y() - float(0.9)).less_than(floor.clone());
        let ripple = hit.clone().select(
            vec4(vec3(pos.x(), floor, pos.swizzle("z")), float(0.)),
            storage_element(2, instance_index()),
        );
        let age = hit.clone().select(
            float(1.),
            storage_element(3, instance_index()).x() + uniform(1, Type::Float) * float(4.),
        );
        let height = tsl::Texture::External(0)
            .sample_level((ripple.swizzle("xz") + float(50.)) / float(100.), float(0.))
            .y()
            + float(0.05);
        let age = ripple.y().greater_than(height).select(float(1000.), age);
        // TSL promotes scalar operands to the unsigned instance index before hashing.
        let time = uniform(0, Type::Float);
        let next = vec4(
            vec3(
                hash(instance_index() + time.clone().to_uint()) * float(100.) - float(50.),
                float(25.),
                hash(instance_index() + (time + float(respawn as f32)).to_uint()) * float(100.)
                    - float(50.),
            ),
            float(0.),
        );
        let sampler = r.device.create_sampler(&Default::default());
        let compute = BufferCompute::with_textures(
            r,
            COUNT,
            &bindings,
            &[
                store(0, hit.select(next, pos)),
                store(2, ripple),
                store(3, vec4(vec3(age, float(0.), float(0.)), float(0.))),
            ],
            &[(&collision.view, &sampler)],
        )
        .await?;
        let color = ((float(1.) - (uv() - vec2(float(0.5), float(0.))).length()) * float(3.)).exp()
            * float(0.1);
        let mut node = SpriteNodeMaterial::new(vec4(
            vec3(color.clone(), color.clone(), color.clone()),
            color * float(0.2),
        ));
        node.position = storage_element(0, instance_index()).rgb();
        node.horizontal_rotation = true;
        let mut mat = node.build_with_storage(r, &bindings, &[]).await?;
        mat.properties.depth_write = false;
        let mut g = PlaneGeometry::build(0.1, 2., 1, 1)?;
        g.instance_count = Some(COUNT / 2);
        let rain = mesh(s, Arc::new(g), Material::Shader(mat));
        s.get_mut(rain)?.frustum_culled = false;
        let age = storage_element(3, instance_index()).x();
        let distance = age.clone() - (uv() - float(0.5)).length() * float(7.);
        let color = distance
            .clone()
            .less_than(float(1.))
            .select(distance.clone(), float(1.))
            - (distance.max(float(1.)) - float(1.));
        let alpha = (float(1.) - age * float(0.3)).max(float(0.)) * float(0.5);
        let program = SurfaceNodes {
            position: Some(position_geometry() + storage_element(2, instance_index()).rgb()),
            color: Some(vec4(
                vec3(color.clone(), color.clone(), color.clone()),
                (color * alpha).max(float(0.)),
            )),
            ..Default::default()
        }
        .build(r, &bindings, &[])
        .await?;
        let mut mat = MeshBasicMaterial::default();
        mat.properties.vertex_program = Some(Arc::new(program));
        mat.properties.transparent = true;
        mat.properties.side = Side::Double;
        mat.properties.depth_write = false;
        // Only the fixed 12-vertex ripple shape is merged once, matching mergeGeometries.
        let mut parts = Vec::new();
        for (w, h, matrix) in [
            (
                2.5,
                2.5,
                Matrix4::from_rotation_x(-std::f64::consts::FRAC_PI_2),
            ),
            (
                1.,
                2.,
                Matrix4::from_rotation_y(-std::f64::consts::FRAC_PI_2),
            ),
            (1., 2., Matrix4::IDENTITY),
        ] {
            parts.push(crate::batching::BatchEntry {
                geometry: Arc::new(PlaneGeometry::build(w, h, 1, 1)?),
                matrix,
                visible: true,
            });
        }
        let merged = crate::batching::build(
            &parts,
            Arc::new(Material::Basic(MeshBasicMaterial::default())),
        )?;
        let mut geometry = (*merged.geometry).clone();
        geometry.instance_count = Some(COUNT / 2);
        let ripples = mesh(s, Arc::new(geometry), Material::Basic(mat));
        s.get_mut(ripples)?.frustum_culled = false;
        let mut g = PlaneGeometry::build(1000., 1000., 1, 1)?;
        g.rotate_x(-std::f64::consts::FRAC_PI_2)?;
        let mut m = MeshBasicMaterial::default();
        m.properties.color = Color::from_hex(0x050505);
        mesh(s, Arc::new(g), Material::Basic(m));
        let mut m = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0x333333);
        let obstacle = mesh(
            s,
            Arc::new(BoxGeometry::build(30., 1., 15.)?),
            Material::Standard(m),
        );
        s.get_mut(obstacle)?.position.y = 12.;
        s.get_mut(obstacle)?.scale.x = 3.5;
        s.get_mut(obstacle)?.layers.mask = 3;
        #[derive(serde::Deserialize)]
        struct Array<T> {
            array: Vec<T>,
        }
        #[derive(serde::Deserialize)]
        struct Attributes {
            position: Array<f32>,
        }
        #[derive(serde::Deserialize)]
        struct Data {
            attributes: Attributes,
            index: Array<u16>,
        }
        #[derive(serde::Deserialize)]
        struct Asset {
            data: Data,
        }
        let asset: Asset = serde_json::from_slice(
            &fetch("/web/gallery/assets/suzanne_buffergeometry.json").await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(crate::attribute::BufferAttribute::new(
                asset.data.attributes.position.array,
                3,
                false,
            )?),
        );
        g.set_index(Some(
            asset.data.index.array.into_iter().map(u32::from).collect(),
        ));
        g.compute_vertex_normals()?;
        let monkey = mesh(
            s,
            Arc::new(g),
            Material::Standard(MeshStandardMaterial {
                roughness: 1.,
                metalness: 0.,
                energy_conservation: true,
                ..Default::default()
            }),
        );
        s.get_mut(monkey)?.scale = Vector3::splat(5.);
        s.get_mut(monkey)?.position.y = 4.5;
        s.get_mut(monkey)?.quaternion = Quaternion::from_rotation_y(std::f64::consts::FRAC_PI_2);
        s.get_mut(monkey)?.layers.mask = 3;
        let program = Arc::new(
            SurfaceNodes {
                color: Some(position_world()),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        );
        let mut collider = MeshBasicMaterial::default();
        collider.properties.vertex_program = Some(program);
        let collider = Arc::new(Material::Basic(collider));
        let mut colliders = Vec::new();
        for h in [obstacle, monkey] {
            if let NodeKind::Mesh(m) = &s.get(h)?.kind {
                colliders.push((h, m.materials[0].clone(), collider.clone()));
            }
        }
        Ok(Self {
            viewer,
            time: 0.,
            previous: 0.,
            delta: 0.,
            pending: true,
            collision,
            camera,
            colliders,
            init,
            compute,
            initialized: false,
            obstacle,
            monkey,
            particles: [rain, ripples],
            parameters: [0., 3.5, (COUNT / 2) as f32],
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= 3 || !v.is_finite() {
            return Err(Error::Invalid("rain parameter"));
        }
        self.parameters[i] = match i {
            0 => v.clamp(-50., 50.),
            1 => v.clamp(0.1, 3.5),
            _ => v.round().clamp(200., COUNT as f32),
        };
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
            self.pending = true;
        }
        self.delta = self.time - self.previous;
        self.previous = self.time;
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.1;
            p.far = 110.;
        }
        s.get_mut(self.monkey)?.quaternion =
            Quaternion::from_rotation_y(std::f64::consts::FRAC_PI_2 + self.time);
        let n = s.get_mut(self.obstacle)?;
        n.position = n.position.lerp(
            Vector3::new(0., 12., -self.parameters[0] as f64),
            self.delta * 10.,
        );
        n.scale.x = self.parameters[1] as f64;
        for &h in &self.particles {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                let count = Some(self.parameters[2] as u32);
                if m.geometry.instance_count != count {
                    Arc::make_mut(&mut m.geometry).instance_count = count;
                }
            }
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        w: f64,
        p: bool,
        h: f64,
    ) -> Result<()> {
        if p {
            self.viewer.pan_pixels(s, c, dx, dy, h)?;
        } else {
            self.viewer.orbit_pixels(dx, dy, w, h, 5., 50.);
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        o: &RenderTarget,
    ) -> Result<bool> {
        if !self.initialized {
            self.init.dispatch(r)?;
            self.initialized = true;
        }
        if self.pending {
            for (h, _, m) in &self.colliders {
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(*h)?.kind {
                    mesh.materials[0] = m.clone();
                }
            }
            let result = r.render(s, self.camera, &self.collision);
            for (h, m, _) in &self.colliders {
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(*h)?.kind {
                    mesh.materials[0] = m.clone();
                }
            }
            result?;
            let mut values = [[0.; 4]; 16];
            values[0][0] = self.time as f32;
            values[1][0] = self.delta as f32;
            self.compute.set_uniforms(r, &values)?;
            self.compute.dispatch(r)?;
            self.pending = false;
        }
        r.render(s, c, o)?;
        Ok(true)
    }
}
