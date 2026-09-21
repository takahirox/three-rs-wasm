use super::*;
impl Demo {
    pub(super) async fn sss(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::BLACK;
        s.background_alpha = 0.0;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xc1c1c1),
            intensity: 1.0,
        }));
        let h = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 0.03,
            target: Vector3::ZERO,
        }));
        s.get_mut(h)?.position = Vector3::new(0.0, 0.5, 0.5);
        for (p, color, intensity, distance) in [
            (Vector3::new(0.0, -50.0, 350.0), 0xc1c1c1, 4.0, 300.0),
            (Vector3::new(-100.0, 20.0, -260.0), 0xc1c100, 0.75, 500.0),
        ] {
            let mut m = MeshBasicMaterial::default();
            m.properties.color = Color::from_hex(color);
            let h = mesh(
                s,
                Arc::new(SphereGeometry::build(4.0, 8, 8)?),
                Material::Basic(m),
            );
            s.get_mut(h)?.position = p;
            let light = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(color),
                intensity,
                distance,
                decay: 0.0,
            }));
            s.add(h, light)?;
        }
        let mut t = decode_texture_image(
            &fetch(&format!("{ASSETS}/models/fbx/bunny_thickness.jpg")).await?,
        )
        .await?;
        t.srgb = false;
        t.mipmap_filter = Some(Filter::Linear);
        let t = r.upload_texture(&Arc::new(t))?;
        let program = SurfaceNodes {
            thickness_color: Some(
                tsl::Texture::External(0)
                    .sample(vec2(uv().x(), float(1.0) - uv().y()))
                    .rgb()
                    * vec3(float(0.5), float(0.3), float(0.0)),
            ),
            thickness: Some(uniform(0, Type::Vec4)),
            thickness_scale: Some(uniform(1, Type::Float)),
            ..Default::default()
        }
        .build(r, &[], &[(&t.view, &t.sampler)])
        .await?;
        let mut m = MeshPhysicalMaterial::default();
        m.base.roughness = 0.3;
        m.base.energy_conservation = true;
        m.base.properties.color = Color::linear(1.0, 0.2, 0.2);
        m.base.properties.vertex_program = Some(Arc::new(program));
        let mut data = geometries(&fetch(&format!("{ASSETS}/bunny.bin")).await?)?;
        let h = mesh(s, Arc::new(data.remove(0)), Material::Physical(m));
        s.get_mut(h)?.position.z = 10.0;
        self.objects.push(h);
        self.animated = Some(h);
        self.params[..5].copy_from_slice(&[0.1, 0.4, 0.8, 2.0, 16.0]);
        Ok(())
    }
}
