use super::*;
impl Demo {
    pub(super) async fn refraction(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let mut material = MeshPhongMaterial {
            emissive: Color::from_hex(0x7b7b7b),
            ..Default::default()
        };
        material.properties.flat_shading = true;
        self.group = Some(mesh(
            s,
            Arc::new(IcosahedronGeometry::build(5.0, 0)?),
            Material::Phong(material),
        ));
        let mut tex = decode_texture_image(
            &fetch(&format!(
                "{ASSETS}/textures/floors/FloorsCheckerboard_S_Normal.jpg"
            ))
            .await?,
        )
        .await?;
        tex.srgb = false;
        tex.wrap_s = Wrapping::Repeat;
        tex.wrap_t = Wrapping::Repeat;
        tex.mipmap_filter = Some(Filter::Linear);
        let gpu = r.upload_texture(&Arc::new(tex))?;
        let p = uv() * float(5.0);
        let normal = tsl::Texture::External(0)
            .sample(vec2(p.x(), float(1.0) - p.y()))
            .swizzle("xy");
        let coordinate = vp::screen_uv() + (normal * float(2.0) - float(1.0)) * float(0.1);
        let program = SurfaceNodes {
            backdrop: Some(vec4(vp::color(vp::safe_uv(coordinate)).rgb(), float(1.0))),
            ..Default::default()
        }
        .build(r, &[], &[(&gpu.view, &gpu.sampler)])
        .await?;
        let mut material = MeshBasicMaterial::default();
        material.properties.transparent = true;
        material.properties.vertex_program = Some(Arc::new(program));
        let plane = Arc::new(PlaneGeometry::build(100.1, 100.1, 1, 1)?);
        let h = mesh(s, plane.clone(), Material::Basic(material));
        s.get_mut(h)?.position.y = 50.0;
        for (color, p, q) in [
            (
                0xffffff,
                Vector3::new(0.0, 100.0, 0.0),
                Quaternion::from_rotation_x(std::f64::consts::FRAC_PI_2),
            ),
            (
                0xffffff,
                Vector3::ZERO,
                Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2),
            ),
            (
                0x7f7fff,
                Vector3::new(0.0, 50.0, -50.0),
                Quaternion::IDENTITY,
            ),
            (
                0x00ff00,
                Vector3::new(50.0, 50.0, 0.0),
                Quaternion::from_rotation_y(-std::f64::consts::FRAC_PI_2),
            ),
            (
                0xff0000,
                Vector3::new(-50.0, 50.0, 0.0),
                Quaternion::from_rotation_y(std::f64::consts::FRAC_PI_2),
            ),
        ] {
            let mut m = MeshPhongMaterial::default();
            m.properties.color = Color::from_hex(color);
            let h = mesh(s, plane.clone(), Material::Phong(m));
            let n = s.get_mut(h)?;
            n.position = p;
            n.quaternion = q;
        }
        for (color, intensity, distance, p) in [
            (0xe7e7e7, 2.5, 250.0, Vector3::new(0.0, 60.0, 0.0)),
            (0x00ff00, 0.5, 1000.0, Vector3::new(550.0, 50.0, 0.0)),
            (0xff0000, 0.5, 1000.0, Vector3::new(-550.0, 50.0, 0.0)),
            (0xbbbbfe, 0.5, 1000.0, Vector3::new(0.0, 50.0, 550.0)),
        ] {
            let h = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(color),
                intensity,
                distance,
                decay: 0.0,
            }));
            s.get_mut(h)?.position = p;
        }
        Ok(())
    }
}
