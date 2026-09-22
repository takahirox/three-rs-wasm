use super::super::gltf_viewer::fetch;
use super::*;
use crate::{
    environment::EnvironmentMap,
    postprocessing::Effect,
    tsl::{
        bloom::Bloom,
        materialx::{self, Dimension},
    },
};
pub(in crate::browser) struct Retro {
    viewer: OrbitViewer,
    orbit_delta: Vector2,
    pan_delta: Vector3,
    floor: Object3D,
    camera: Object3D,
    lights: [Object3D; 2],
    bands: Vec<Object3D>,
    reflected: RenderTarget,
    main: RenderTarget,
    bloom: Bloom,
    finish: Effect,
    sampler: wgpu::Sampler,
    parameters: [f32; 3],
}
fn noise(p: tsl::Node, d: Dimension) -> tsl::Node {
    materialx::perlin(p, d)
}
fn bandlimit(p: tsl::Node) -> tsl::Node {
    float(1.) - p.fwidth().length().smoothstep(float(0.5), float(1.5))
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
async fn basic(
    s: &mut Scene,
    r: &Renderer,
    g: BufferGeometry,
    color: tsl::Node,
    side: Side,
) -> Result<Object3D> {
    let mut m = MeshBasicMaterial::default();
    m.properties.side = side;
    m.properties.vertex_program = Some(Arc::new(
        SurfaceNodes {
            color: Some(color),
            ..Default::default()
        }
        .build(r, &[], &[])
        .await?,
    ));
    Ok(mesh(s, Arc::new(g), Material::Basic(m)))
}
async fn environment(r: &Renderer) -> Result<EnvironmentMap> {
    let mut s = Scene::default();
    let y = position_local().normalize().y().abs();
    basic(
        &mut s,
        r,
        SphereGeometry::build(10., 32, 16)?,
        mix(rgb(0x05070c), rgb(0x2a3148), (float(1.) - y).pow(float(4.))),
        Side::Back,
    )
    .await?;
    let p = position_world();
    let cell = vec2(
        (p.x() + p.swizzle("z") * float(1.7)) * float(3.),
        p.y() * float(2.5),
    );
    let q = cell.fract();
    let window = q.x().smoothstep(float(0.15), float(0.3))
        * (float(1.) - q.x().smoothstep(float(0.7), float(0.85)))
        * q.y().smoothstep(float(0.2), float(0.35))
        * (float(1.) - q.y().smoothstep(float(0.65), float(0.8)));
    let lit = noise(cell.floor() * float(0.731) + float(0.37), Dimension::D2)
        .smoothstep(float(0.), float(0.1));
    let color = mix(
        rgb(0x88a8ff),
        rgb(0xffc16b),
        (noise(cell.floor() * float(0.317), Dimension::D2) + float(1.)) * float(0.5),
    );
    let program = Arc::new(
        SurfaceNodes {
            color: Some(mix(rgb(0x05060a), color * float(12.), window * lit)),
            ..Default::default()
        }
        .build(r, &[], &[])
        .await?,
    );
    for i in 0..8 {
        let angle = i as f64 / 8. * std::f64::consts::TAU + 0.35;
        let height = 2.5 + (i as f64 * 2.7) % 3.5;
        let mut m = MeshBasicMaterial::default();
        m.properties.vertex_program = Some(program.clone());
        let h = mesh(
            &mut s,
            Arc::new(BoxGeometry::build(2., height, 1.)?),
            Material::Basic(m),
        );
        s.get_mut(h)?.position =
            Vector3::new(angle.cos() * 9., height / 2. - 0.5, angle.sin() * 9.);
        s.look_at(h, Vector3::new(0., height / 2. - 0.5, 0.))?;
    }
    for i in 0..6 {
        let a = i as f64 / 6. * std::f64::consts::TAU + 0.7;
        let mut m = MeshBasicMaterial::default();
        m.properties.color = Color(if i % 3 == 2 {
            Vector3::new(7., 9., 13.)
        } else {
            Vector3::new(18., 11., 4.)
        });
        let h = mesh(
            &mut s,
            Arc::new(SphereGeometry::build(0.35, 32, 16)?),
            Material::Basic(m),
        );
        s.get_mut(h)?.position = Vector3::new(a.cos() * 8., 2.5 + (i % 3) as f64, a.sin() * 8.);
    }
    let mut m = MeshBasicMaterial::default();
    m.properties.color = Color(Vector3::new(2.5, 3., 4.));
    let h = mesh(
        &mut s,
        Arc::new(SphereGeometry::build(0.6, 32, 16)?),
        Material::Basic(m),
    );
    s.get_mut(h)?.position = Vector3::new(4., 7., -5.);
    EnvironmentMap::from_scene(r, &mut s, 256, 0.02)
}
impl Retro {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            aspect,
            near: 0.1,
            far: 100.,
            ..Default::default()
        }));
        let center = Vector3::new(0., 0.33, 0.);
        let p = Vector3::new(2.3, 1.32, 3.25) - center;
        let mut viewer = OrbitViewer::from_camera(center, p.length());
        viewer.fixture(p.x.atan2(p.z), (p.y / p.length()).asin(), 1.8);
        s.background = Color::from_hex(0x0f131a);
        s.fog = Some(Fog::Linear {
            color: s.background,
            near: 4.,
            far: 18.,
        });
        s.tone_mapping = ToneMapping::Aces;
        let lights = [
            s.insert(NodeKind::Light(Light::Point {
                color: Color::WHITE,
                intensity: 5.,
                distance: 12.,
                decay: 2.,
            })),
            s.insert(NodeKind::Light(Light::Point {
                color: Color::WHITE,
                intensity: 1.75,
                distance: 12.,
                decay: 2.,
            })),
        ];
        let reflected = plain(r, 1, 1)?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let p = position_world().swizzle("xz");
        let grain = noise(p.clone() * float(60.), Dimension::D2)
            * bandlimit(p.clone() * float(60.))
            * float(0.2);
        let patches = noise(p.clone() * float(1.5), Dimension::D2) * float(0.08);
        let asphalt = rgb(0x1b1e23) * (grain + patches + float(1.));
        let puddle = noise(p.clone() * float(0.45) + float(6.27), Dimension::D2)
            .smoothstep(float(0.25), float(0.45));
        let strength =
            (float(1.) - puddle.clone()) * bandlimit(p.clone() * float(180.)) * float(0.0015);
        let h = noise(p.clone() * float(180.), Dimension::D2);
        let hx = noise(
            (p.clone() + vec2(float(0.002), float(0.))) * float(180.),
            Dimension::D2,
        );
        let hz = noise(
            (p.clone() + vec2(float(0.), float(0.002))) * float(180.),
            Dimension::D2,
        );
        let bump = vec3(
            (h.clone() - hx) * strength.clone(),
            (hz - h) * strength,
            float(0.002),
        )
        .normalize();
        let normal = transform_normal_to_view(bump.clone());
        let screen = tsl::viewport::screen_uv();
        let coord = vec2(float(1.) - screen.x(), screen.y()) + bump.swizzle("xy") * float(0.03);
        let blur = WgslFn::new(
            "retro_hash_blur",
            include_str!("retro.wgsl"),
            &[Type::Texture, Type::Sampler, Type::Vec2, Type::Float],
            Type::Vec3,
        )?
        .call(&[
            tsl::Texture::External(0).node(),
            tsl::Texture::External(0).sampler(),
            coord,
            mix(float(0.08), float(0.005), puddle.clone()),
        ]);
        let program = SurfaceNodes {
            color: Some(mix(asphalt.clone(), asphalt * float(0.3), puddle.clone())),
            roughness: Some(mix(
                noise(p.clone() * float(50.), Dimension::D2)
                    * bandlimit(p * float(50.))
                    * float(0.15)
                    + float(0.6),
                float(0.05),
                puddle.clone(),
            )),
            normal: Some(normal),
            emissive: Some(blur * mix(float(0.15), float(0.7), puddle)),
            ..Default::default()
        }
        .build(r, &[], &[(&reflected.view, &sampler)])
        .await?;
        let mut m = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        m.properties.vertex_program = Some(Arc::new(program));
        let floor = mesh(
            s,
            Arc::new(PlaneGeometry::build(40., 40., 1, 1)?),
            Material::Standard(m),
        );
        s.get_mut(floor)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        let p = position_local();
        let program = Arc::new(
            SurfaceNodes {
                roughness: Some(
                    noise(p.clone() * float(30.), Dimension::D3) * float(0.2) + float(0.45),
                ),
                color: Some(
                    rgb(0xff6a00)
                        * (noise((p + float(7.3)) * float(20.), Dimension::D3) * float(0.1)
                            + float(1.)),
                ),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        );
        let mut orange = MeshPhysicalMaterial::default();
        orange.base.energy_conservation = true;
        orange.base.properties.vertex_program = Some(program);
        let orange = Arc::new(Material::Physical(orange));
        let mut band = MeshPhysicalMaterial {
            retroreflectivity: 1.,
            ..Default::default()
        };
        band.base.energy_conservation = true;
        band.base.roughness = 0.22;
        band.base.properties.color = Color::from_hex(0xf7f0d6);
        let band = Arc::new(Material::Physical(band));
        #[derive(serde::Deserialize)]
        struct AttributeData {
            size: usize,
            array: Vec<f32>,
        }
        #[derive(serde::Deserialize)]
        struct Data {
            index: Option<Vec<u32>>,
            attributes: std::collections::HashMap<String, AttributeData>,
        }
        let data: Data = serde_json::from_slice(
            &fetch("/web/gallery/assets/tsl-procedural/retro-base.json").await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let mut base = BufferGeometry::default();
        base.set_index(data.index);
        for (name, a) in data.attributes {
            base.set_attribute(
                &name,
                Attribute::F32(crate::attribute::BufferAttribute::new(
                    a.array, a.size, false,
                )?),
            );
        }
        let base = Arc::new(base);
        let body = Arc::new(CylinderGeometry::build(
            0.035,
            0.17,
            0.71,
            96,
            1,
            false,
            0.,
            std::f64::consts::TAU,
        )?);
        let mut band_geometries = Vec::new();
        for (center, height) in [(0.04 + 0.71 * 0.6, 0.10), (0.04 + 0.71 * 0.4, 0.06)] {
            let radius =
                |y: f64| (0.17 + (0.035 - 0.17) * ((y - 0.04) / 0.71).clamp(0., 1.)) * 1.01;
            band_geometries.push((
                center,
                Arc::new(CylinderGeometry::build(
                    radius(center + height * 0.5),
                    radius(center - height * 0.5),
                    height,
                    96,
                    1,
                    true,
                    0.,
                    std::f64::consts::TAU,
                )?),
            ));
        }
        let mut bands = Vec::new();
        for i in 0..5 {
            let a = i as f64 / 5. * std::f64::consts::TAU;
            let pos = Vector3::new(a.cos() * 1.1, 0., a.sin() * 1.1);
            for (g, y, m) in [
                (base.clone(), 0., orange.clone()),
                (body.clone(), 0.04 + 0.71 * 0.5, orange.clone()),
            ] {
                let h = s.insert(NodeKind::Mesh(Mesh::new(g, m)));
                s.get_mut(h)?.position = pos + Vector3::Y * y;
            }
            for (y, g) in &band_geometries {
                let h = s.insert(NodeKind::Mesh(Mesh::new(g.clone(), band.clone())));
                s.get_mut(h)?.position = pos + Vector3::Y * *y;
                bands.push(h);
            }
        }
        s.environment = Some(Arc::new(environment(r).await?));
        let camera = s.insert(s.get(c)?.kind.clone());
        s.get_mut(camera)?.matrix_auto_update = false;
        let mut bloom = Bloom::new(r).await?;
        bloom.strength = 0.15;
        bloom.radius = 0.;
        bloom.threshold = 0.;
        let finish = tsl::effect_with_textures(
            r,
            HDR,
            &(tsl::Texture::Input.sample(uv()) + tsl::Texture::External(0).sample(uv())),
            &[(&reflected.view, &sampler)],
        )
        .await?;
        Ok(Self {
            viewer,
            orbit_delta: Vector2::ZERO,
            pan_delta: Vector3::ZERO,
            floor,
            camera,
            lights,
            bands,
            reflected,
            main: target(r, 1, 1)?,
            bloom,
            finish,
            sampler,
            parameters: [1., 1., 5.],
        })
    }
    pub fn seek(&mut self, _: f64) {}
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= 3 || !v.is_finite() {
            return Err(Error::Invalid("retroreflection parameter"));
        }
        self.parameters[i] = v.clamp(0., if i == 2 { 30. } else { 1. });
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, _: f64, _: bool) -> Result<()> {
        self.viewer.orbit_pixels(
            self.orbit_delta.x * 0.05,
            self.orbit_delta.y * 0.05,
            0.,
            1.,
            0.000001,
            f64::INFINITY,
        );
        self.viewer.pan_world(self.pan_delta * 0.05);
        self.orbit_delta *= 0.95;
        self.pan_delta *= 0.95;
        self.viewer
            .limit_pitch(0.05, std::f64::consts::FRAC_PI_2 - 1e-6);
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.1;
            p.far = 100.;
        }
        let p = s.get(c)?.position;
        for (i, &h) in self.lights.iter().enumerate() {
            s.get_mut(h)?.position = Vector3::new(p.x, if i == 0 { p.y } else { -p.y }, p.z);
            if let NodeKind::Light(Light::Point { intensity, .. }) = &mut s.get_mut(h)?.kind {
                *intensity = self.parameters[2] as f64 * if i == 0 { 1. } else { 0.35 };
            }
        }
        let amount = (self.parameters[0] * self.parameters[1]) as f64;
        for &h in &self.bands {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind
                && matches!(m.materials[0].as_ref(),Material::Physical(p) if p.retroreflectivity!=amount)
                && let Material::Physical(p) = Arc::make_mut(&mut m.materials[0])
            {
                p.retroreflectivity = amount;
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
            self.pan_delta += OrbitViewer::pan_delta(s, c, self.viewer.radius(), dx, dy, h)?;
        } else {
            self.orbit_delta += Vector2::new(dx, dy) / h.max(1.);
            self.viewer
                .orbit_pixels(0., 0., w, h, 0.000001, f64::INFINITY);
            self.viewer
                .limit_pitch(0.05, std::f64::consts::FRAC_PI_2 - 1e-6);
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let (w, h) = (out.width.div_ceil(2), out.height.div_ceil(2));
        if (self.reflected.width, self.reflected.height) != (w, h) {
            self.reflected = plain(r, w, h)?;
            if let NodeKind::Mesh(m) = &mut s.get_mut(self.floor)?.kind {
                Arc::make_mut(
                    Arc::make_mut(&mut m.materials[0])
                        .properties_mut()
                        .vertex_program
                        .as_mut()
                        .unwrap(),
                )
                .rebind(r, &[], &[(&self.reflected.view, &self.sampler)])?;
            }
        }
        let resized = (self.main.width, self.main.height) != (out.width, out.height);
        if resized {
            self.main = target(r, out.width, out.height)?;
        }
        s.update_world_matrix(c, true, false)?;
        let (cam, world) = s.camera(c)?;
        if let Camera::Perspective(cam) = cam
            && let Some((cam, matrix)) =
                crate::reflection::planar_camera(cam, world, Vector4::new(0., 1., 0., 0.))?
        {
            let n = s.get_mut(self.camera)?;
            n.kind = NodeKind::Camera(Camera::Perspective(cam));
            n.matrix = matrix;
            n.matrix_world_needs_update = true;
            s.get_mut(self.floor)?.visible = false;
            let result = r.render(s, self.camera, &self.reflected);
            s.get_mut(self.floor)?.visible = true;
            result?;
        }
        r.render(s, c, &self.main)?;
        let bloom = self.bloom.render(r, &self.main)?;
        if resized {
            self.finish
                .set_textures(r, &[(&bloom.view, &self.sampler)])?;
        }
        self.finish.apply(r, &self.main, None, out)?;
        Ok(true)
    }
}
