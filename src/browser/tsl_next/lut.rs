use super::*;
use crate::{postprocessing::Effect, tsl::lut::Lut3D};
pub(super) struct Lut {
    scene: RenderTarget,
    effects: Vec<Effect>,
}
impl Demo {
    pub(super) async fn lut(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let (a, b, i) =
            super::super::gltf_viewer::load_asset(&format!("{ASSETS}/models/gltf/coffeeMug.glb"))
                .await?;
        crate::gltf::import_decoded(&a, &b, &i)?.instantiate(s)?;
        for h in s.handles().collect::<Vec<_>>() {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for mat in &mut m.materials {
                    if let Some(map) = &mut Arc::make_mut(mat).properties_mut().map {
                        Arc::make_mut(map).anisotropy = 8;
                    }
                }
            }
        }
        let mut noise =
            decode_image(&fetch(&format!("{ASSETS}/textures/noises/perlin/128x128.png")).await?)
                .await?;
        noise.srgb = false;
        noise.wrap_s = Wrapping::Repeat;
        noise.wrap_t = Wrapping::Repeat;
        noise.mipmap_filter = Some(Filter::Linear);
        let texture = r.upload_texture(&Arc::new(noise))?;
        let sample = |p: tsl::Node, level: bool| {
            let p = vec2(p.x(), float(1.0) - p.y());
            if level {
                tsl::Texture::External(0).sample_level(p, float(0.0))
            } else {
                tsl::Texture::External(0).sample(p)
            }
        };
        let time = uniform(0, Type::Float);
        let twist = sample(
            vec2(
                float(0.5),
                (uv().y() * float(0.2) - time.clone() * float(0.005)).modulo(float(1.0)),
            ),
            true,
        )
        .x() * float(10.0);
        let pos = position_geometry();
        let wind = vec2(
            sample(
                vec2(float(0.25), (time.clone() * float(0.01)).modulo(float(1.0))),
                true,
            )
            .x() - float(0.5),
            sample(
                vec2(float(0.75), (time.clone() * float(0.01)).modulo(float(1.0))),
                true,
            )
            .x() - float(0.5),
        ) * uv().y().pow(float(2.0))
            * float(10.0);
        let position = vec3(
            pos.x() * twist.cos() - pos.swizzle("z") * twist.sin() + wind.x(),
            pos.y() + wind.y(),
            pos.x() * twist.sin() + pos.swizzle("z") * twist.cos(),
        );
        let alpha = sample(
            uv() * vec2(float(0.5), float(0.3)) + vec2(float(0.0), -time * float(0.03)),
            false,
        )
        .x()
        .smoothstep(float(0.4), float(1.0))
            * uv().x().smoothstep(float(0.0), float(0.1))
            * (float(1.0) - uv().x()).smoothstep(float(0.0), float(0.1))
            * uv().y().smoothstep(float(0.0), float(0.1))
            * (float(1.0) - uv().y()).smoothstep(float(0.0), float(0.1));
        let color = mix(
            vec3(float(0.6), float(0.3), float(0.2)),
            splat(float(1.0), Type::Vec3),
            alpha.pow(float(3.0)),
        );
        let graph = NodeMaterial {
            position: Some(position),
            color: vec4(color, alpha),
        };
        let mut mat = graph.build(r, &[(&texture.view, &texture.sampler)]).await?;
        mat.properties.transparent = true;
        mat.properties.side = Side::Double;
        mat.properties.force_single_pass = false;
        mat.properties.depth_write = false;
        let mut g = PlaneGeometry::build(1.0, 1.0, 16, 64)?;
        g.apply_matrix4(
            Matrix4::from_scale(Vector3::new(1.5, 6.0, 1.5))
                * Matrix4::from_translation(Vector3::new(0.0, 0.5, 0.0)),
        )?;
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Shader(mat)),
        )));
        s.get_mut(h)?.position.y = 1.83;
        self.objects.push(h);
        let mut effects = vec![];
        for name in [
            "Bourbon 64.CUBE",
            "Chemical 168.CUBE",
            "Clayton 33.CUBE",
            "Cubicle 99.CUBE",
            "Remy 24.CUBE",
            "Presetpro-Cinematic.3dl",
            "NeutralLUT.png",
            "B&WLUT.png",
            "NightLUT.png",
        ] {
            let data = fetch(&format!("{ASSETS}/luts/{name}")).await?;
            let lut = if name.ends_with(".CUBE") {
                Lut3D::from_cube(
                    std::str::from_utf8(&data).map_err(|_| Error::Invalid("LUT text"))?,
                )?
            } else if name.ends_with(".3dl") {
                Lut3D::from_3dl(
                    std::str::from_utf8(&data).map_err(|_| Error::Invalid("LUT text"))?,
                )?
            } else {
                let im = image::load_from_memory(&data)
                    .map_err(|e| Error::Asset(e.to_string()))?
                    .to_rgba8();
                Lut3D::from_strip(im.width(), im.height(), im.as_raw())?
            };
            effects.push(
                lut.effect(r, wgpu::TextureFormat::Rgba16Float, true)
                    .await?,
            );
        }
        let options = RenderTargetOptions {
            samples: 4,
            format: wgpu::TextureFormat::Rgba16Float,
            ..Default::default()
        };
        let scene = RenderTarget::with_options(&r.device, 1, 1, options.clone())?;
        self.params[0] = 0.0;
        self.lut = Some(Lut { scene, effects });
        Ok(())
    }
}
impl Lut {
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        index: usize,
        intensity: f32,
    ) -> Result<()> {
        self.scene.set_size(&r.device, out.width, out.height)?;
        r.render(s, c, &self.scene)?;
        let effect = &mut self.effects[index.min(8)];
        effect.parameters[0][0] = intensity;
        effect.apply(r, &self.scene, None, out)
    }
}
