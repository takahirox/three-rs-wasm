use super::*;
impl Demo {
    pub(super) async fn toon(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0x444488);
        let g = Arc::new(SphereGeometry::build(32.0, 32, 16)?);
        let outline =
            toon_outline(r, vec4(splat(float(0.0), Type::Vec3), float(1.0)), 0.003).await?;
        for a in 0..6 {
            let count = a + 2;
            let mut bytes = vec![];
            for j in 0..count {
                let v = (j as f64 / count as f64 * 256.0) as u8;
                bytes.extend_from_slice(&[v, 0, 0, 255]);
            }
            let mut t = crate::material::Texture::from_rgba(count, 1, bytes, false)?;
            t.filter = Filter::Nearest;
            t.min_filter = Some(Filter::Nearest);
            t.mipmap_filter = None;
            let t = Arc::new(t);
            for b in 0..6 {
                for c in 0..6 {
                    let alpha = a as f64 * 0.2;
                    let beta = b as f64 * 0.2;
                    let gamma = c as f64 * 0.2;
                    let mut m = MeshToonMaterial {
                        gradient_map: Some(t.clone()),
                        ..Default::default()
                    };
                    m.base.properties.color = Color(
                        Color::from_hsl(alpha, 0.5, gamma * 0.5 + 0.1).0 * (1.0 - beta * 0.2),
                    );
                    let h = mesh(s, g.clone(), Material::Toon(m));
                    let p = Vector3::new(
                        alpha * 400.0 - 200.0,
                        beta * 400.0 - 200.0,
                        gamma * 400.0 - 200.0,
                    );
                    s.get_mut(h)?.position = p;
                    let o = mesh(s, g.clone(), Material::Shader(outline.clone()));
                    s.get_mut(o)?.position = p;
                    // A group makes the hull and its fill adjacent in render order, as the pass does.
                    let order = (a * 36 + b * 6 + c) * 2;
                    s.get_mut(o)?.render_order = order as i32;
                    s.get_mut(h)?.render_order = order as i32 + 1;
                }
            }
        }
        let mut labels = geometries(&fetch(&format!("{ASSETS}/labels.bin")).await?)?.into_iter();
        for p in [
            Vector3::new(-350.0, 0.0, 0.0),
            Vector3::new(350.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -300.0),
            Vector3::new(0.0, 0.0, 300.0),
        ]
        .into_iter()
        {
            let h = mesh(
                s,
                Arc::new(labels.next().ok_or(Error::Invalid("label geometry"))?),
                Material::Basic(MeshBasicMaterial::default()),
            );
            s.get_mut(h)?.position = p;
        }
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xc1c1c1),
            intensity: 3.0,
        }));
        let h = mesh(
            s,
            Arc::new(SphereGeometry::build(4.0, 8, 8)?),
            Material::Basic(MeshBasicMaterial::default()),
        );
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 2.0,
            distance: 800.0,
            decay: 0.0,
        }));
        s.add(h, light)?;
        self.animated = Some(h);
        Ok(())
    }
}
