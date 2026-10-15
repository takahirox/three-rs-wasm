use super::*;
use crate::compute::{BufferAccess, GpuBuffer};
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.0
}
impl Demo {
    pub(super) async fn path(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let points: Vec<[f64; 2]> =
            serde_json::from_slice(&fetch(&format!("{ASSETS}/path.json")).await?)
                .map_err(|e| Error::Asset(e.to_string()))?;
        let (mut positions, mut colors, mut times) = (vec![], vec![], vec![]);
        let mut seed = 186;
        for (i, p) in points.iter().enumerate() {
            positions.push([
                (p[0] + 0.5 - random(&mut seed)) as f32,
                (p[1] + 0.5 - random(&mut seed)) as f32,
                (0.5 - random(&mut seed)) as f32,
                0.0,
            ]);
            times.push([i as f32 / 1000.0, random(&mut seed) as f32, 0.0, 0.0]);
            colors.push(
                Color::from_hsl(0.75 + random(&mut seed) * 0.25, 1.0, 0.4)
                    .0
                    .as_vec3()
                    .extend(1.0)
                    .to_array(),
            );
        }
        let buffers = [positions, colors, times]
            .iter()
            .map(|data| GpuBuffer::new(r, bytemuck::cast_slice(data), BufferAccess::Read))
            .collect::<Result<Vec<_>>>()?;
        let p = instanced_attribute(0);
        let data = instanced_attribute(2);
        let time = uniform(0, Type::Float);
        let d = (data.x() - (time.clone() * float(0.4)).modulo(float(1.0))).abs();
        let wrapped = d.greater_than(float(0.5)).select(float(1.0) - d.clone(), d);
        let scale = wrapped
            .greater_than(float(0.1))
            .select(float(1.0), float(3.0) - wrapped * float(20.0));
        let graph = SurfaceNodes {
            position: Some(
                position_geometry() * scale
                    + vec3(
                        p.x(),
                        p.y() + (data.x() + time + data.y()).sin() * float(0.25),
                        p.swizzle("z"),
                    ),
            ),
            color: Some(instanced_attribute(1).rgb()),
            ..Default::default()
        };
        let bindings = buffers.iter().map(|b| (b, Type::Vec4)).collect::<Vec<_>>();
        let mut mat = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        mat.properties.vertex_program = Some(Arc::new(graph.build(r, &bindings, &[]).await?));
        let mut g = IcosahedronGeometry::build(0.1, 0)?;
        g.instance_count = Some(1000);
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Standard(mat)),
        )));
        s.get_mut(h)?.position = Vector3::new(2.5, 5.0, 0.0);
        s.get_mut(h)?.frustum_culled = false;
        self.objects.push(h);
        let c = Color::from_hex(0x94254c).0;
        let center = vec3(float(c.x as f32), float(c.y as f32), float(c.z as f32));
        let bg = background_material(
            r,
            mix(
                center,
                splat(float(0.0), Type::Vec3),
                (uv() - float(0.5)).length() / float(0.65),
            ),
        )
        .await?;
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
            Arc::new(Material::Shader(bg)),
        )));
        s.get_mut(h)?.frustum_culled = false;
        s.get_mut(h)?.render_order = i32::MIN;
        s.environment = Some(room::environment(r)?);
        s.tone_mapping = ToneMapping::Neutral;
        Ok(())
    }
}
