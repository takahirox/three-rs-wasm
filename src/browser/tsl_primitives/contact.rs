use super::*;
use crate::postprocessing::Effect;
pub(super) struct Contact {
    camera: Object3D,
    helper: Object3D,
    depth: RenderTarget,
    horizontal: RenderTarget,
    blurred: RenderTarget,
    h: Effect,
    v: Effect,
    normal: Arc<Material>,
    depth_material: Arc<Material>,
    objects: Vec<Object3D>,
    plane: Object3D,
    fill: Object3D,
}
impl Demo {
    pub(super) async fn contact(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::WHITE;
        let normal = Arc::new(Material::Shader(
            NodeMaterial::new(tsl::srgb_to_linear(
                normal_view_geometry() * float(0.5) + float(0.5),
            ))
            .build(r, &[])
            .await?,
        ));
        let mut objects = vec![];
        for (i, g) in [
            BoxGeometry::build(0.4, 0.4, 0.4)?,
            IcosahedronGeometry::build(0.3, 0)?,
            TorusKnotGeometry::build(0.4, 0.05, 256, 24, 1, 3)?,
        ]
        .into_iter()
        .enumerate()
        {
            let h = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(g), normal.clone())));
            let a = i as f64 / 3. * std::f64::consts::TAU;
            s.get_mut(h)?.position = Vector3::new(a.cos() / 2., 0.1, a.sin() / 2.);
            objects.push(h);
            self.objects.push(h);
            self.rotations.push(Vector3::ZERO);
        }
        let mut dm = NodeMaterial::new(vec4(
            splat(float(0.), Type::Vec3),
            (float(1.) - fragment_depth()) * uniform(0, Type::Float),
        ))
        .build(r, &[])
        .await?;
        dm.properties.depth_test = false;
        dm.properties.depth_write = false;
        let depth_material = Arc::new(Material::Shader(dm));
        let target = || {
            RenderTarget::with_options(
                &r.device,
                512,
                512,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    ..Default::default()
                },
            )
        };
        let depth = target()?;
        let horizontal = target()?;
        let blurred = target()?;
        let blur = gaussian_blur(tsl::Texture::Input, uv(), uniform(0, Type::Vec2), 4)?;
        let h = effect(r, wgpu::TextureFormat::Rgba8Unorm, &blur).await?;
        let v = effect(r, wgpu::TextureFormat::Rgba8Unorm, &blur).await?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut pm = NodeMaterial::new(vec4(
            splat(float(0.), Type::Vec3),
            tsl::Texture::External(0).sample(uv()).swizzle("w") * uniform(0, Type::Float),
        ))
        .build(r, &[(&blurred.view, &sampler)])
        .await?;
        pm.properties.transparent = true;
        pm.properties.depth_write = false;
        let geometry = Arc::new(PlaneGeometry::build(2.5, 2.5, 1, 1)?);
        let plane = mesh(s, geometry.clone(), Material::Shader(pm));
        s.get_mut(plane)?.position.y = -0.3;
        s.get_mut(plane)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        s.get_mut(plane)?.render_order = 1;
        let mut fm = MeshBasicMaterial::default();
        fm.properties.transparent = true;
        fm.properties.depth_write = false;
        let fill = mesh(s, geometry, Material::Basic(fm));
        s.get_mut(fill)?.position.y = -0.3;
        s.get_mut(fill)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        let camera = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.25,
            right: 1.25,
            top: 1.25,
            bottom: -1.25,
            near: 0.,
            far: 0.3,
            ..Default::default()
        })));
        s.get_mut(camera)?.position.y = -0.3;
        s.get_mut(camera)?.quaternion = Quaternion::from_rotation_x(std::f64::consts::FRAC_PI_2);
        #[derive(serde::Deserialize)]
        struct Helper {
            position: Vec<f32>,
            color: Vec<f32>,
        }
        let data: Helper = serde_json::from_slice(&fetch(&format!("{ASSETS}/helper.json")).await?)
            .map_err(|e| Error::Asset(e.to_string()))?;
        let mut g = BufferGeometry::default();
        for (name, values) in [("position", data.position), ("color", data.color)] {
            g.set_attribute(
                name,
                Attribute::F32(BufferAttribute::new(values, 3, false)?),
            );
        }
        let mut m = LineBasicMaterial::default();
        m.properties.vertex_colors = true;
        let helper = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Line(m)),
            segments: true,
        }));
        s.get_mut(helper)?.visible = false;
        self.params = [3.5, 1., 1., 0xffffff_u32 as f32, 1., 0., 0., 0.];
        self.contact = Some(Contact {
            helper,
            camera,
            depth,
            horizontal,
            blurred,
            h,
            v,
            normal,
            depth_material,
            objects,
            plane,
            fill,
        });
        Ok(())
    }
}
impl Contact {
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        p: &[f32; 8],
    ) -> Result<()> {
        if let Material::Shader(m) = Arc::make_mut(&mut self.depth_material) {
            m.uniforms[0][0] = p[1];
        }
        for &h in &self.objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                m.materials[0] = self.depth_material.clone();
            }
        }
        s.get_mut(self.plane)?.visible = false;
        s.get_mut(self.fill)?.visible = false;
        s.get_mut(self.helper)?.visible = false;
        let bg = s.background;
        let alpha = s.background_alpha;
        s.background = Color::BLACK;
        s.background_alpha = 0.;
        let result = r.render(s, self.camera, &self.depth);
        s.background = bg;
        s.background_alpha = alpha;
        for &h in &self.objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                m.materials[0] = self.normal.clone();
            }
        }
        s.get_mut(self.plane)?.visible = true;
        s.get_mut(self.fill)?.visible = true;
        result?;
        self.h.parameters[0] = [p[0] / 512., 0., 0., 0.];
        self.v.parameters[0] = [0., p[0] / 512., 0., 0.];
        self.h.apply(r, &self.depth, None, &self.horizontal)?;
        self.v.apply(r, &self.horizontal, None, &self.blurred)?;
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.plane)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[0][0] = p[2];
        }
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.fill)?.kind {
            let p0 = Arc::make_mut(&mut m.materials[0]).properties_mut();
            p0.color = Color::from_hex(p[3] as u32);
            p0.opacity = p[4] as f64;
        }
        s.get_mut(self.helper)?.visible = p[5] > 0.5;
        r.render(s, c, out)
    }
}
