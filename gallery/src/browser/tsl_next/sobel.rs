use super::*;
use crate::postprocessing::Effect;
pub(super) struct Sobel {
    scene: RenderTarget,
    encoded: RenderTarget,
    encode: Effect,
    edge: Effect,
}
impl Demo {
    pub(super) async fn sobel(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let (a, b, i) = super::super::gltf_viewer::load_asset(&format!(
            "{ASSETS}/models/gltf/DragonAttenuation.glb"
        ))
        .await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        for h in instance.roots {
            if s.get(h)?.name == "Cloth Backdrop" {
                s.get_mut(h)?.visible = false;
            }
        }
        for h in s.handles().collect::<Vec<_>>() {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                m.materials = vec![Arc::new(Material::Standard(MeshStandardMaterial {
                    energy_conservation: true,
                    ..Default::default()
                }))];
            }
        }
        s.environment = Some(room::environment(r)?);
        let options = RenderTargetOptions {
            samples: 4,
            format: wgpu::TextureFormat::Rgba16Float,
            ..Default::default()
        };
        let scene = RenderTarget::with_options(&r.device, 1, 1, options.clone())?;
        let encoded = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples: 1,
                ..options.clone()
            },
        )?;
        let encode = tsl::effect(
            r,
            options.format,
            &vec4(
                tsl::display::srgb(
                    tsl::Texture::Input
                        .sample(uv())
                        .rgb()
                        .clamp(float(0.0), float(1.0)),
                ),
                float(1.0),
            ),
        )
        .await?;
        let edge = tsl::effect(
            r,
            options.format,
            &tsl::display::sobel(tsl::Texture::Input, uv(), uniform(0, Type::Vec2)),
        )
        .await?;
        self.sobel = Some(Sobel {
            scene,
            encoded,
            encode,
            edge,
        });
        Ok(())
    }
}
impl Sobel {
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        enabled: bool,
    ) -> Result<()> {
        for target in [&mut self.scene, &mut self.encoded] {
            target.set_size(&r.device, out.width, out.height)?;
        }
        r.render(s, c, &self.scene)?;
        if enabled {
            self.encode.apply(r, &self.scene, None, &self.encoded)?;
            self.edge.parameters[0] = [1.0 / out.width as f32, 1.0 / out.height as f32, 0.0, 0.0];
            self.edge.apply(r, &self.encoded, None, out)?;
        } else {
            self.encode.apply(r, &self.scene, None, out)?;
        }
        Ok(())
    }
}
