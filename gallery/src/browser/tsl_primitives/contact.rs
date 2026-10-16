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
        let (position, color) = camera_helper(
            Matrix4::from_rotation_translation(
                Quaternion::from_rotation_x(std::f64::consts::FRAC_PI_2),
                Vector3::new(0., -0.3, 0.),
            ),
            (-1.25, 1.25, 1.25, -1.25, 0., 0.3),
        );
        let mut g = BufferGeometry::default();
        for (name, values) in [("position", position), ("color", color)] {
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

/// CameraHelper( camera ) for an orthographic camera with the given world
/// matrix, its lines unprojected by update() ( WebGL clip depth, as at
/// construction ) and the geometry moved by the camera's world matrix.
fn camera_helper(
    world: Matrix4,
    (left, right, top, bottom, near, far): (f64, f64, f64, f64, f64, f64),
) -> (Vec<f32>, Vec<f32>) {
    let (w, h, p) = (1. / (right - left), 1. / (top - bottom), 1. / (far - near));
    let projection = Matrix4::from_cols_array(&[
        2. * w,
        0.,
        0.,
        0.,
        0.,
        2. * h,
        0.,
        0.,
        0.,
        0.,
        -2. * p,
        0.,
        -(right + left) * w,
        -(top + bottom) * h,
        -(far + near) * p,
        1.,
    ]);
    let inverse = projection.inverse();
    let point = |name: &str| -> Vector3 {
        let c = |x: f64, y: f64, near: bool| {
            inverse.project_point3(Vector3::new(x, y, if near { -1. } else { 1. }))
        };
        match name {
            "c" => c(0., 0., true),
            "t" => c(0., 0., false),
            "n1" => c(-1., -1., true),
            "n2" => c(1., -1., true),
            "n3" => c(-1., 1., true),
            "n4" => c(1., 1., true),
            "f1" => c(-1., -1., false),
            "f2" => c(1., -1., false),
            "f3" => c(-1., 1., false),
            "f4" => c(1., 1., false),
            "u1" => c(0.7, 1.1, true),
            "u2" => c(-0.7, 1.1, true),
            "u3" => c(0., 2., true),
            "cf1" => c(-1., 0., false),
            "cf2" => c(1., 0., false),
            "cf3" => c(0., -1., false),
            "cf4" => c(0., 1., false),
            "cn1" => c(-1., 0., true),
            "cn2" => c(1., 0., true),
            "cn3" => c(0., -1., true),
            "cn4" => c(0., 1., true),
            // "p" is never set by update().
            _ => Vector3::ZERO,
        }
    };
    let (frustum, cone, up, target, cross) = (0xffaa00, 0xff0000, 0x00aaff, 0xffffff, 0x333333);
    let lines: [(&str, &str, u32); 25] = [
        ("n1", "n2", frustum),
        ("n2", "n4", frustum),
        ("n4", "n3", frustum),
        ("n3", "n1", frustum),
        ("f1", "f2", frustum),
        ("f2", "f4", frustum),
        ("f4", "f3", frustum),
        ("f3", "f1", frustum),
        ("n1", "f1", frustum),
        ("n2", "f2", frustum),
        ("n3", "f3", frustum),
        ("n4", "f4", frustum),
        ("p", "n1", cone),
        ("p", "n2", cone),
        ("p", "n3", cone),
        ("p", "n4", cone),
        ("u1", "u2", up),
        ("u2", "u3", up),
        ("u3", "u1", up),
        ("c", "t", target),
        ("p", "c", cross),
        ("cn1", "cn2", cross),
        ("cn3", "cn4", cross),
        ("cf1", "cf2", cross),
        ("cf3", "cf4", cross),
    ];
    let (mut positions, mut colors) = (vec![], vec![]);
    for (a, b, color) in lines {
        for name in [a, b] {
            // Float32 positions, then geometry.applyMatrix4( camera.matrixWorld ).
            let local = point(name).to_array().map(|v| f64::from(v as f32));
            let p = world.project_point3(Vector3::from_array(local));
            positions.extend(p.to_array().map(|v| v as f32));
            colors.extend(Color::from_hex(color).0.to_array().map(|v| v as f32));
        }
    }
    (positions, colors)
}
