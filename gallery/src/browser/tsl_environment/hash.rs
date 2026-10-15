use super::*;
impl Demo {
    pub(super) async fn hash(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.environment = Some(super::super::room_environment::environment(r)?);
        let alpha = alpha_hash(
            uniform(0, Type::Vec2).x(),
            position_local(),
            uniform(0, Type::Vec2).y().greater_than(float(0.5)),
        );
        let graph = SurfaceNodes {
            color: Some(vec4(base_color().rgb(), alpha)),
            ..Default::default()
        };
        let mut m = standard(0xffffff, 1.0, 0.0);
        m.properties.vertex_program = Some(Arc::new(graph.build(r, &[], &[]).await?));
        let h = mesh(
            s,
            Arc::new(IcosahedronGeometry::build(0.5, 3)?),
            Material::Standard(m),
        );
        let mut seed = 186;
        let mut instances = vec![];
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    instances.push(Instance {
                        matrix: Matrix4::from_translation(Vector3::new(
                            1.0 - x as f64,
                            1.0 - y as f64,
                            1.0 - z as f64,
                        )),
                        color: Color::from_hex((random(&mut seed) * 0xffffff as f64) as u32),
                    });
                }
            }
        }
        s.get_mut(h)?.instances = instances;
        self.objects.push(h);
        self.ssaa = Some(crate::postprocessing::ssaa::SsaaPass::new(r).await?);
        self.params[..3].copy_from_slice(&[0.5, 1.0, 3.0]);
        Ok(())
    }
}
