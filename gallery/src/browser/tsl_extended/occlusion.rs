use super::*;
impl Demo {
    pub(super) async fn occlusion(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xb0b0b0),
            intensity: 1.0,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 1.0,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0.32, 0.39, 0.7);
        let mut plane = MeshPhongMaterial::default();
        plane.properties.side = Side::Double;
        plane.properties.vertex_program = Some(Arc::new(
            tsl::surface::SurfaceNodes {
                color: Some(uniform(0, Type::Vec3)),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        ));
        plane.properties.vertex_uniforms[0] = [0.0, 0.0, 1.0, 1.0];
        self.objects.push(mesh(
            s,
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
            Material::Phong(plane),
        ));
        let mut sphere = MeshPhongMaterial::default();
        sphere.properties.color = Color::from_hex(0xffff00);
        let h = mesh(
            s,
            Arc::new(SphereGeometry::build(0.5, 32, 16)?),
            Material::Phong(sphere),
        );
        s.get_mut(h)?.position.z = -1.0;
        self.objects.push(h);
        self.occlusion = Some(crate::occlusion::OcclusionQueries::new(r, &[h], 1)?);
        Ok(())
    }
    pub(super) fn update_occlusion(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        self.viewer.update(s, c)?;
        if let Some(queries) = &self.occlusion {
            let occluded = queries.is_occluded(self.objects[1]).unwrap_or(false);
            if let NodeKind::Mesh(m) = &mut s.get_mut(self.objects[0])?.kind
                && let Material::Phong(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.properties.vertex_uniforms[0] = if occluded {
                    [0.0, 1.0, 0.0, 1.0]
                } else {
                    [0.0, 0.0, 1.0, 1.0]
                };
            }
        }
        Ok(())
    }
}
