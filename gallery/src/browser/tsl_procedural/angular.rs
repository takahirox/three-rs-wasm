use super::super::gltf_viewer::fetch;
use super::*;
pub(in crate::browser) struct Angular {
    viewer: OrbitViewer,
    hull: Vec<Object3D>,
    params: [f32; 3],
}
impl Angular {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 35.,
            aspect,
            near: 0.1,
            far: 100.,
            ..Default::default()
        }));
        let position = Vector3::new(-5., 5., 12.);
        s.get_mut(c)?.position = position;
        s.look_at(c, Vector3::ZERO)?;
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
        viewer.fixture(
            position.x.atan2(position.z),
            (position.y / position.length()).asin(),
            1.8,
        );
        s.tone_mapping = ToneMapping::Aces;
        s.shadow_map_size = 4096;
        let im = image::load_from_memory(
            &fetch("/web/gallery/assets/tsl-lighting/royal_esplanade_2k.hdr.jpg.rgba16f.png")
                .await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?
        .to_rgba16();
        let (width, height) = im.dimensions();
        s.environment = Some(Arc::new(crate::environment::EnvironmentMap {
            width,
            height,
            rgba: im
                .as_raw()
                .iter()
                .map(|x| half::f16::from_bits(*x))
                .collect(),
            gpu: None,
        }));
        s.background_environment = true;
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 4.,
        }));
        {
            let n = s.get_mut(sun)?;
            n.position = Vector3::new(6.25, 3., 4.);
            n.cast_shadow = true;
            n.shadow.far = 20.;
            n.shadow.normal_bias = 0.05;
        }
        let mask=WgslFn::new("angular_mask","fn angular_mask(p:vec3<f32>,start:f32,arc:f32)->bool{let a=atan2(p.y,p.x)-start;let angle=a-6.283185307179586*floor(a/6.283185307179586);return !(angle>0.0 && angle<arc);}",&[Type::Vec3,Type::Float,Type::Float],Type::Bool).unwrap().call(&[position_local(),uniform(0,Type::Float),uniform(1,Type::Float)]);
        let program = Arc::new(
            SurfaceNodes {
                mask: Some(mask.clone()),
                output: Some(
                    front_facing().select(output(), vec4(uniform(2, Type::Vec3), float(1.))),
                ),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        );
        let shadow = Arc::new(shadow_program(r, None, Some(mask)).await?);
        let (a, b, i) = load_asset("/web/gallery/assets/tsl-procedural/gears.glb").await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let mut hull = vec![];
        for h in instance.meshes {
            let sliced = s.get(h)?.name == "outerHull";
            let mut material = MeshPhysicalMaterial::default();
            material.base.properties.color = Color::from_hex(0x858080);
            material.base.metalness = 0.5;
            material.base.roughness = 0.25;
            material.base.energy_conservation = true;
            if sliced {
                material.base.properties.side = Side::Double;
                material.base.properties.vertex_program = Some(program.clone());
                material.base.properties.shadow_program = Some(shadow.clone());
                hull.push(h);
            }
            let n = s.get_mut(h)?;
            n.cast_shadow = true;
            n.receive_shadow = true;
            if let NodeKind::Mesh(m) = &mut n.kind {
                m.materials = vec![Arc::new(Material::Physical(material))];
            }
        }
        let mut material = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        material.properties.color = Color::from_hex(0xaaaaaa);
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(10., 10., 10, 1)?),
            Material::Standard(material),
        );
        s.get_mut(h)?.position = Vector3::new(-4., -3., -4.);
        s.look_at(h, Vector3::ZERO)?;
        s.get_mut(h)?.receive_shadow = true;
        if hull.is_empty() {
            return Err(Error::Asset("missing outerHull".into()));
        }
        Ok(Self {
            viewer,
            hull,
            params: [1.75, 1.25, 0xb62f58 as f32],
        })
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, _d: f64, _a: bool) -> Result<()> {
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.1;
            p.far = 100.;
        }
        for &h in &self.hull {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                let p = Arc::make_mut(&mut m.materials[0]).properties_mut();
                p.vertex_uniforms[0][0] = self.params[0];
                p.vertex_uniforms[1][0] = self.params[1];
                let c = Color::from_hex(self.params[2] as u32).0;
                p.vertex_uniforms[2] = [c.x as f32, c.y as f32, c.z as f32, 0.];
            }
        }
        Ok(())
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= 3 || !v.is_finite() {
            return Err(Error::Invalid("angular parameter"));
        }
        self.params[i] = v;
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
            self.viewer.orbit_pixels(dx, dy, w, h, 0.1, 50.);
        }
        Ok(())
    }
}
