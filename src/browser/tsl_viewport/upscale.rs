use super::*;
use crate::postprocessing::Effect;
pub(super) struct Upscale {
    scene: RenderTarget,
    easu: RenderTarget,
    up: Effect,
    sharpen: Effect,
    copy: Effect,
}
impl Demo {
    pub(super) async fn upscale(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        self.params[..2].copy_from_slice(&[1.0, 0.5]);
        s.background = Color::from_hex(0xbfe3dd);
        s.environment = Some(super::super::room_environment::environment(r)?);
        let (a, b, i) = load_asset(&format!("{ASSETS}/models/gltf/LittlestTokyo.glb")).await?;
        let imported = crate::gltf::import_animated_decoded(&a, &b, &i)?;
        let instance = imported.instantiate(s)?;
        let group = s.insert(NodeKind::Group);
        s.get_mut(group)?.scale = Vector3::splat(0.01);
        for h in instance.roots {
            s.add(group, h)?;
        }
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        self.mixer = Some(mixer);
        for h in instance.meshes {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for m in &mut m.materials {
                    match Arc::make_mut(m) {
                        Material::Standard(m)
                        | Material::Physical(MeshPhysicalMaterial { base: m, .. }) => {
                            m.energy_conservation = true
                        }
                        _ => {}
                    }
                }
            }
        }
        let scene = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: HDR,
                samples: 4,
                ..Default::default()
            },
        )?;
        let easu = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: HDR,
                depth_buffer: false,
                ..Default::default()
            },
        )?;
        self.post = Some(Upscale {
            scene,
            easu,
            up: tsl::effect(r, HDR, &tsl::fsr1::easu(tsl::Texture::Input, uv())).await?,
            sharpen: tsl::effect(
                r,
                HDR,
                &tsl::fsr1::rcas(
                    tsl::Texture::Input,
                    uv(),
                    float(0.2),
                    float(0.0).greater_than(float(1.0)),
                ),
            )
            .await?,
            copy: tsl::effect(r, HDR, &tsl::Texture::Input.sample(uv())).await?,
        });
        Ok(())
    }
}
impl Upscale {
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        p: &[f32; 8],
    ) -> Result<()> {
        let scale = p[1].clamp(0.25, 1.0);
        let (w, h) = (
            (out.width as f32 * scale) as u32,
            (out.height as f32 * scale) as u32,
        );
        self.scene.set_size(&r.device, w.max(1), h.max(1))?;
        self.easu.set_size(&r.device, out.width, out.height)?;
        r.render(s, c, &self.scene)?;
        if p[0] > 0.5 {
            self.up.apply(r, &self.scene, None, &self.easu)?;
            self.sharpen.apply(r, &self.easu, None, out)?;
        } else {
            self.copy.apply(r, &self.scene, None, out)?;
        }
        Ok(())
    }
}
