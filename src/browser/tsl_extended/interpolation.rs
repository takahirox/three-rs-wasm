use super::*;
use crate::{attribute::BufferAttribute, shader::UvInterpolation};
pub(super) struct Comparison {
    targets: Vec<RenderTarget>,
    materials: Vec<Arc<Material>>,
    effect: Effect,
}
impl Comparison {
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        index: usize,
        objects: &[Object3D],
    ) -> Result<()> {
        let index = index.min(4);
        for h in objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                m.materials[0] = self.materials[index].clone();
            }
        }
        let width = (out.width / 2).max(1);
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.aspect = width as f64 / out.height as f64;
        }
        for target in &mut self.targets {
            target.set_size(&r.device, width, out.height)?;
            r.render(s, c, target)?;
        }
        self.effect
            .apply(r, &self.targets[0], Some(&self.targets[1]), out)
    }
}
impl Demo {
    pub(super) async fn interpolation(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let mut pixels = vec![];
        for y in (0..16).rev() {
            for x in 0..16 {
                pixels.extend_from_slice(match (x < 8, y < 8) {
                    (true, true) => &[255, 0, 0, 255],
                    (false, true) => &[0, 128, 0, 255],
                    (true, false) => &[0, 0, 255, 255],
                    _ => &[255, 255, 0, 255],
                });
            }
        }
        let mut image = crate::material::Texture::from_rgba(16, 16, pixels, true)?;
        image.min_filter = Some(Filter::Nearest);
        image.filter = Filter::Nearest;
        image.wrap_s = Wrapping::Repeat;
        image.wrap_t = Wrapping::Repeat;
        let texture = r.upload_texture(&Arc::new(image))?;
        let graph = NodeMaterial::new(tsl::Texture::External(0).sample(uv()).rgb());
        let mut materials = vec![];
        for interpolation in [
            UvInterpolation::Center,
            UvInterpolation::Centroid,
            UvInterpolation::Sample,
            UvInterpolation::FlatFirst,
            UvInterpolation::FlatEither,
        ] {
            materials.push(Arc::new(Material::Shader(
                graph
                    .build_interpolated(r, &[(&texture.view, &texture.sampler)], interpolation)
                    .await?,
            )));
        }
        let uvs = [
            [0.0, 1.0, 0.5, 1.0, 0.5, 0.5, 0.0, 0.5],
            [1.0, 1.0, 0.5, 1.0, 0.5, 0.5, 1.0, 0.5],
            [0.0, 0.0, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5],
            [1.0, 0.0, 0.5, 0.0, 0.5, 0.5, 1.0, 0.5],
        ];
        let mut seed = 186;
        for x in -5..5 {
            for y in -5..5 {
                let mut g = BufferGeometry::default();
                g.set_attribute(
                    "position",
                    Attribute::F32(BufferAttribute::new(
                        vec![
                            -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0,
                        ],
                        3,
                        false,
                    )?),
                );
                g.set_attribute(
                    "uv",
                    Attribute::F32(BufferAttribute::new(
                        uvs[(random(&mut seed) * 4.0) as usize].to_vec(),
                        2,
                        false,
                    )?),
                );
                g.set_index(Some(vec![0, 1, 2, 2, 3, 0]));
                let h = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(g), materials[0].clone())));
                s.get_mut(h)?.position = Vector3::new(x as f64 * 2.0, y as f64 * 2.0, 0.0);
                self.objects.push(h);
            }
        }
        let a = tsl::Texture::Input.sample(vec2(uv().x() * float(2.0), uv().y()));
        let b = tsl::Texture::History.sample(vec2(uv().x() * float(2.0) - float(1.0), uv().y()));
        let effect = tsl::effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &uv().x().less_than(float(0.5)).select(a, b),
        )
        .await?;
        let targets = [1, 4]
            .into_iter()
            .map(|samples| {
                RenderTarget::with_options(
                    &r.device,
                    1,
                    1,
                    RenderTargetOptions {
                        format: wgpu::TextureFormat::Rgba16Float,
                        samples,
                        ..Default::default()
                    },
                )
            })
            .collect::<Result<Vec<_>>>()?;
        self.params[0] = 0.0;
        self.interpolation = Some(Comparison {
            targets,
            materials,
            effect,
        });
        Ok(())
    }
}
