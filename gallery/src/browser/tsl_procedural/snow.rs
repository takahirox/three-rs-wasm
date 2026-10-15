use super::super::gltf_viewer::fetch;
use super::*;
use crate::{
    compute::{BufferAccess, GpuBuffer},
    postprocessing::Effect,
    tsl::compute::{BufferCompute, BufferStore},
};
const COUNT: u32 = 100000;
pub(in crate::browser) struct Snow {
    viewer: OrbitViewer,
    time: f64,
    previous: f64,
    pending: bool,
    pub dragging: bool,
    collision: RenderTarget,
    collision_camera: Object3D,
    variants: Vec<(Object3D, Arc<Material>, Arc<Material>)>,
    init: BufferCompute,
    compute: BufferCompute,
    initialized: bool,
    main: RenderTarget,
    glow: RenderTarget,
    blurred: [RenderTarget; 4],
    blur: [Effect; 2],
    finish: Effect,
    sampler: wgpu::Sampler,
    glow_scene: Scene,
    glow_camera: Object3D,
}
fn store(binding: usize, value: tsl::Node) -> BufferStore {
    BufferStore {
        binding,
        index: instance_index(),
        value,
    }
}
fn plain(r: &Renderer, w: u32, h: u32) -> Result<RenderTarget> {
    RenderTarget::with_options(
        &r.device,
        w,
        h,
        RenderTargetOptions {
            format: HDR,
            ..Default::default()
        },
    )
}
fn standard(hex: u32, roughness: f64) -> MeshStandardMaterial {
    let mut m = MeshStandardMaterial {
        roughness,
        metalness: 0.,
        energy_conservation: true,
        ..Default::default()
    };
    m.properties.color = Color::from_hex(hex);
    m
}
impl Snow {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            aspect,
            near: 0.1,
            far: 100.,
            ..Default::default()
        }));
        s.get_mut(c)?.layers.mask = 5;
        let center = Vector3::new(0., 10., 0.);
        let p = Vector3::new(20., 2., 20.) - center;
        let mut viewer = OrbitViewer::from_camera(center, p.length());
        viewer.fixture(p.x.atan2(p.z), (p.y / p.length()).asin(), 1.8);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x0f3c37),
            near: 5.,
            far: 40.,
        });
        s.tone_mapping = ToneMapping::Aces;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0xf9ff9b),
            intensity: 9.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(10., 10., 0.);
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0x0f3c37),
            ground: Color::from_hex(0x080d10),
            intensity: 100.,
        }));
        s.get_mut(hemi)?.position = Vector3::Y;
        let collision_camera =
            s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                left: -50.,
                right: 50.,
                top: 50.,
                bottom: -50.,
                near: 0.1,
                far: 50.,
                ..Default::default()
            })));
        s.get_mut(collision_camera)?.position.y = 50.;
        s.get_mut(collision_camera)?.layers.mask = 3;
        s.look_at(collision_camera, Vector3::ZERO)?;
        let collision = RenderTarget::with_options(
            &r.device,
            1024,
            1024,
            RenderTargetOptions {
                format: wgpu::TextureFormat::R16Float,
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
            seed as f64 / 4294967296.
        };
        let sy = (random() * 0xffffff as f64).round() as u32;
        let sz = (random() * 0xffffff as f64).round() as u32;
        let _scale_seed = random();
        let ix = instance_index();
        let rx = hash(ix.clone());
        let ry = hash(ix.clone() + uint(sy));
        let rz = hash(ix.clone() + uint(sz));
        let pos = vec3(
            rx.clone() * float(100.) - float(50.),
            ry.clone() * float(500.) + float(3.),
            rz * float(100.) - float(50.),
        );
        let scale = hash(ix.clone()) * float(0.8) + float(0.2);
        let init = BufferCompute::new(
            r,
            COUNT,
            &bindings,
            &[
                store(0, vec4(pos.clone(), float(0.))),
                store(
                    1,
                    vec4(vec3(scale.clone(), scale.clone(), scale), float(0.)),
                ),
                store(
                    2,
                    vec4(vec3(float(1000.), float(10000.), float(1000.)), float(0.)),
                ),
                store(
                    3,
                    vec4(
                        vec3(pos.x(), ry * float(-0.1) - float(0.02), pos.swizzle("z")),
                        rx,
                    ),
                ),
            ],
        )
        .await?;
        let position = storage_element(0, instance_index());
        let scale = storage_element(1, instance_index());
        let data = storage_element(3, instance_index());
        let sampler = r.device.create_sampler(&Default::default());
        let height = tsl::Texture::External(0)
            .sample_level(
                (position.swizzle("xz") + float(50.)) / float(100.),
                float(0.),
            )
            .x()
            + scale.x() * float(0.2);
        let falling = position.y().greater_than(height);
        let t = uniform(0, Type::Float);
        let random = data.swizzle("w");
        let next = vec3(
            data.x() + (t.clone() * random.clone() * random.clone() * float(0.4)).sin() * float(3.),
            position.y() + data.y(),
            data.swizzle("z") + (t * random.clone() * float(0.4)).cos() * random * float(10.),
        );
        let compute = BufferCompute::with_textures(
            r,
            COUNT,
            &bindings,
            &[
                store(
                    0,
                    falling
                        .clone()
                        .select(vec4(next, float(0.)), position.clone()),
                ),
                store(
                    2,
                    falling.select(storage_element(2, instance_index()), position),
                ),
            ],
            &[(&collision.view, &sampler)],
        )
        .await?;
        let collider = Arc::new(
            SurfaceNodes {
                color: Some(position_world().y()),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        );
        let mut collision_material = MeshBasicMaterial::default();
        collision_material.properties.fog = false;
        collision_material.properties.vertex_program = Some(collider);
        let mut variants = Vec::new();
        let mut geometry = SphereGeometry::build(0.2, 5, 5)?;
        geometry.instance_count = Some(COUNT);
        let geometry = Arc::new(geometry);
        for (binding, layer) in [(0, 2), (2, 1)] {
            let position = position_geometry() * storage_element(1, instance_index()).rgb()
                + storage_element(binding, instance_index()).rgb();
            let mut material = standard(0xeeeeee, 0.9);
            material.properties.vertex_program = Some(Arc::new(
                SurfaceNodes {
                    position: Some(position.clone()),
                    ..Default::default()
                }
                .build(r, &bindings, &[])
                .await?,
            ));
            let mut collider = collision_material.clone();
            collider.properties.vertex_program = Some(Arc::new(
                SurfaceNodes {
                    position: Some(position),
                    color: Some(position_world().y()),
                    ..Default::default()
                }
                .build(r, &bindings, &[])
                .await?,
            ));
            let main = Arc::new(Material::Standard(material));
            let other = Arc::new(Material::Basic(collider));
            let h = s.insert(NodeKind::Mesh(Mesh::new(geometry.clone(), main.clone())));
            s.get_mut(h)?.layers.set(layer);
            s.get_mut(h)?.frustum_culled = false;
            variants.push((h, main, other));
        }
        let mut ground = PlaneGeometry::build(100., 100., 1, 1)?;
        ground.rotate_x(-std::f64::consts::FRAC_PI_2)?;
        let mut mat = standard(0x0c1e1e, 0.5);
        mat.properties.transparent = true;
        mat.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                color: Some(vec4(
                    rgb(0x0c1e1e),
                    float(1.)
                        - (position_local().swizzle("xz") * float(0.05))
                            .length()
                            .clamp(float(0.), float(1.)),
                )),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        ));
        let floor = mesh(s, Arc::new(ground), Material::Standard(mat));
        if let NodeKind::Mesh(m) = &s.get(floor)?.kind {
            let mut collider = collision_material.clone();
            collider.properties.transparent = true;
            collider.properties.blending = Some(wgpu::BlendState::REPLACE);
            variants.push((
                floor,
                m.materials[0].clone(),
                Arc::new(Material::Basic(collider)),
            ));
        }
        for i in 0..9 {
            let (g, y) = if i == 8 {
                (
                    CylinderGeometry::build(1., 1., 8., 32, 1, false, 0., std::f64::consts::TAU)?,
                    4.,
                )
            } else {
                let radius = 1. + i as f64;
                (
                    CylinderGeometry::build(
                        0.,
                        radius * 0.95,
                        radius * 1.25,
                        32,
                        1,
                        false,
                        0.,
                        std::f64::consts::TAU,
                    )?,
                    (8 - i) as f64 * 1.5 + 4.8,
                )
            };
            let h = mesh(s, Arc::new(g), Material::Standard(standard(0x0d492c, 0.6)));
            s.get_mut(h)?.position.y = y;
            if let NodeKind::Mesh(m) = &s.get(h)?.kind {
                variants.push((
                    h,
                    m.materials[0].clone(),
                    Arc::new(Material::Basic(collision_material.clone())),
                ));
            }
        }
        #[derive(serde::Deserialize)]
        struct AttributeData {
            size: usize,
            array: Vec<f32>,
        }
        #[derive(serde::Deserialize)]
        struct GeometryData {
            index: Vec<u32>,
            attributes: std::collections::HashMap<String, AttributeData>,
        }
        let data: GeometryData = serde_json::from_slice(
            &fetch("/web/gallery/assets/tsl-procedural/snow-teapot.json").await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let mut g = BufferGeometry::default();
        for (name, a) in data.attributes {
            g.set_attribute(
                &name,
                Attribute::F32(crate::attribute::BufferAttribute::new(
                    a.array, a.size, false,
                )?),
            );
        }
        g.set_index(Some(data.index));
        let geometry = Arc::new(g);
        let mut m = MeshBasicMaterial::default();
        m.properties.color = Color::from_hex(0xfcfb9e);
        let h = mesh(s, geometry.clone(), Material::Basic(m.clone()));
        s.get_mut(h)?.position.y = 18.;
        if let NodeKind::Mesh(m) = &s.get(h)?.kind {
            variants.push((
                h,
                m.materials[0].clone(),
                Arc::new(Material::Basic(collision_material)),
            ));
        }
        let mut glow_scene = Scene::new();
        let glow_camera = glow_scene.insert(s.get(c)?.kind.clone());
        let h = mesh(&mut glow_scene, geometry, Material::Basic(m));
        glow_scene.get_mut(h)?.position.y = 18.;
        let color = mix(
            rgb(0x0f4140),
            rgb(0x060a0d),
            (uv() - float(0.5)).length() * float(2.),
        );
        let bg = mesh(
            s,
            Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
            Material::Shader(background_material(r, vec4(color, float(1.))).await?),
        );
        s.get_mut(bg)?.layers.set(2);
        s.get_mut(bg)?.frustum_culled = false;
        s.get_mut(bg)?.render_order = -100;
        let main = super::target(r, 1, 1)?;
        let glow = super::target(r, 1, 1)?;
        let blurred = [
            plain(r, 1, 1)?,
            plain(r, 1, 1)?,
            plain(r, 1, 1)?,
            plain(r, 1, 1)?,
        ];
        let blur = [
            tsl::effect(
                r,
                HDR,
                &gaussian_blur(tsl::Texture::Input, uv(), uniform(0, Type::Vec2), 4)?,
            )
            .await?,
            tsl::effect(
                r,
                HDR,
                &gaussian_blur(tsl::Texture::Input, uv(), uniform(0, Type::Vec2), 6)?,
            )
            .await?,
        ];
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            min_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let v =
            float(1.) - ((uv() - float(0.5)).length() * float(1.35)).clamp(float(0.), float(1.));
        let color = (tsl::Texture::Input.sample(uv())
            + tsl::Texture::External(0).sample(uv()) * float(0.1))
            * v
            + tsl::Texture::External(1).sample(uv()) * float(10.)
            + tsl::Texture::External(2).sample(uv());
        let finish = effect_with_textures(
            r,
            HDR,
            &color,
            &[
                (&blurred[1].view, &sampler),
                (&glow.view, &sampler),
                (&blurred[3].view, &sampler),
            ],
        )
        .await?;
        Ok(Self {
            viewer,
            time: 0.,
            previous: 0.,
            pending: true,
            dragging: false,
            collision,
            collision_camera,
            variants,
            init,
            compute,
            initialized: false,
            main,
            glow,
            blurred,
            blur,
            finish,
            sampler,
            glow_scene,
            glow_camera,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
            self.pending = true;
        }
        if !self.dragging {
            self.viewer.orbit_pixels(
                (self.time - self.previous) * (-0.7) / 60.,
                0.,
                0.,
                1.,
                25.,
                35.,
            );
        }
        self.previous = self.time;
        self.viewer.limit_pitch(
            std::f64::consts::FRAC_PI_2 - std::f64::consts::PI / 1.7,
            std::f64::consts::FRAC_PI_2 - 1e-6,
        );
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.1;
            p.far = 100.;
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
        if w != 0. && !self.dragging {
            self.viewer.orbit_pixels(-0.7 / 3600., 0., 0., 1., 25., 35.);
        }
        if p {
            self.viewer.pan_pixels(s, c, dx, dy, h)?;
        } else {
            self.viewer.orbit_pixels(dx, dy, w, h, 25., 35.);
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
            for (h, _, m) in &self.variants {
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(*h)?.kind {
                    mesh.materials[0] = m.clone();
                }
            }
            let result = r.render(s, self.collision_camera, &self.collision);
            for (h, m, _) in &self.variants {
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(*h)?.kind {
                    mesh.materials[0] = m.clone();
                }
            }
            result?;
            let mut params = [[0.; 4]; 16];
            params[0][0] = self.time as f32;
            self.compute.set_uniforms(r, &params)?;
            self.compute.dispatch(r)?;
            self.pending = false;
        }
        if self.main.width != o.width || self.main.height != o.height {
            self.main.set_size(&r.device, o.width, o.height)?;
            self.glow.set_size(&r.device, o.width, o.height)?;
            for (i, t) in self.blurred.iter_mut().enumerate() {
                let scale = if i < 2 { 0.5 } else { 0.2 };
                t.set_size(
                    &r.device,
                    ((o.width as f64 * scale).round() as u32).max(1),
                    ((o.height as f64 * scale).round() as u32).max(1),
                )?;
            }
            self.finish.set_textures(
                r,
                &[
                    (&self.blurred[1].view, &self.sampler),
                    (&self.glow.view, &self.sampler),
                    (&self.blurred[3].view, &self.sampler),
                ],
            )?;
        }
        self.glow_scene.get_mut(self.glow_camera)?.kind = s.get(c)?.kind.clone();
        self.glow_scene.get_mut(self.glow_camera)?.position = s.get(c)?.position;
        self.glow_scene.get_mut(self.glow_camera)?.quaternion = s.get(c)?.quaternion;
        r.render(s, c, &self.main)?;
        r.render(&mut self.glow_scene, self.glow_camera, &self.glow)?;
        for i in 0..2 {
            let w = self.blurred[i * 2].width as f32;
            let h = self.blurred[i * 2].height as f32;
            self.blur[i].parameters[0] = [1. / w, 0., 0., 0.];
            self.blur[i].apply(
                r,
                if i == 0 { &self.main } else { &self.glow },
                None,
                &self.blurred[i * 2],
            )?;
            self.blur[i].parameters[0] = [0., 1. / h, 0., 0.];
            self.blur[i].apply(r, &self.blurred[i * 2], None, &self.blurred[i * 2 + 1])?;
        }
        self.finish.apply(r, &self.main, None, o)?;
        Ok(true)
    }
}
