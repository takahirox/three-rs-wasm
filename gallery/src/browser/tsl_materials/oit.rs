use super::*;
use crate::postprocessing::Effect;
const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
pub(super) struct Oit {
    beauty: RenderTarget,
    accum: RenderTarget,
    composite: Effect,
    sampler: wgpu::Sampler,
    opaque: Object3D,
    transparent: Vec<(Object3D, Arc<Material>, Arc<Material>)>,
}
impl Demo {
    pub(super) async fn oit(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0x202020);
        let h = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x444444),
            intensity: 1.5,
        }));
        s.get_mut(h)?.position = Vector3::Y;
        let h = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 2.5,
            target: Vector3::ZERO,
        }));
        s.get_mut(h)?.position = Vector3::new(4.0, 10.0, 6.0);
        let opaque = mesh(
            s,
            Arc::new(TorusKnotGeometry::build(1.0, 0.4, 128, 32, 2, 3)?),
            Material::Standard(MeshStandardMaterial {
                roughness: 0.2,
                energy_conservation: true,
                ..Default::default()
            }),
        );
        let program = Arc::new(
            SurfaceNodes::default()
                .build_mrt(r, &tsl::oit::outputs(), &[], &[])
                .await?,
        );
        let mut transparent = vec![];
        for (colors, planes) in [
            (&[0xff4030, 0x30d060, 0x3070ff][..], true),
            (&[0xffc020, 0xd040d0, 0x40c0d0, 0xa0d040][..], false),
        ] {
            let g = Arc::new(if planes {
                PlaneGeometry::build(6.0, 6.0, 1, 1)?
            } else {
                SphereGeometry::build(0.8, 32, 16)?
            });
            for (i, &color) in colors.iter().enumerate() {
                let mut m = MeshStandardMaterial {
                    roughness: if planes { 0.5 } else { 0.3 },
                    energy_conservation: true,
                    ..Default::default()
                };
                m.properties.color = Color::from_hex(color);
                m.properties.transparent = true;
                m.properties.opacity = 0.5;
                if planes {
                    m.properties.side = Side::Double;
                }
                let h = mesh(s, g.clone(), Material::Standard(m.clone()));
                if planes {
                    s.get_mut(h)?.quaternion =
                        Quaternion::from_rotation_y(i as f64 / 3.0 * std::f64::consts::PI);
                } else {
                    let angle = i as f64 / 4.0 * std::f64::consts::TAU;
                    s.get_mut(h)?.position =
                        Vector3::new(angle.cos() * 2.5, 0.0, angle.sin() * 2.5);
                }
                let normal = Arc::new(Material::Standard(m.clone()));
                m.properties.depth_write = false;
                m.properties.vertex_program = Some(program.clone());
                m.properties.attachment_blending = tsl::oit::blending();
                transparent.push((h, normal, Arc::new(Material::Standard(m))));
            }
        }
        let beauty = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: HDR,
                ..Default::default()
            },
        )?;
        let mut accum = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: HDR,
                count: 2,
                color_formats: vec![HDR, wgpu::TextureFormat::R8Unorm],
                clear_colors: vec![wgpu::Color::TRANSPARENT, wgpu::Color::WHITE],
                load_depth: true,
                ..Default::default()
            },
        )?;
        accum.set_depth_texture(beauty.depth_texture().unwrap().clone())?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let color = tsl::oit::composite(
            tsl::Texture::Input.sample(uv()),
            tsl::Texture::History.sample(uv()),
            tsl::Texture::External(0).sample(uv()).x(),
        );
        let composite = effect_with_textures(
            r,
            HDR,
            &color,
            &[(
                &accum.textures()[1].create_view(&Default::default()),
                &sampler,
            )],
        )
        .await?;
        self.oit = Some(Oit {
            beauty,
            accum,
            composite,
            sampler,
            opaque,
            transparent,
        });
        self.params[..2].copy_from_slice(&[1.0, 0.5]);
        Ok(())
    }
}
impl Oit {
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        p: &[f32; 8],
    ) -> Result<()> {
        for (_, normal, oit) in &mut self.transparent {
            if normal.properties().opacity != p[1] as f64 {
                Arc::make_mut(normal).properties_mut().opacity = p[1] as f64;
                Arc::make_mut(oit).properties_mut().opacity = p[1] as f64;
            }
        }
        if p[0] < 0.5 {
            for (h, m, _) in &self.transparent {
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(*h)?.kind {
                    mesh.materials[0] = m.clone();
                }
            }
            return r.render(s, c, out);
        }
        if self.beauty.width != out.width || self.beauty.height != out.height {
            self.beauty.set_size(&r.device, out.width, out.height)?;
            self.accum.set_size(&r.device, out.width, out.height)?;
            self.accum
                .set_depth_texture(self.beauty.depth_texture().unwrap().clone())?;
            self.composite.set_textures(
                r,
                &[(
                    &self.accum.textures()[1].create_view(&Default::default()),
                    &self.sampler,
                )],
            )?;
        }
        for (h, _, _) in &self.transparent {
            s.get_mut(*h)?.visible = false;
        }
        let result = r.render(s, c, &self.beauty);
        for (h, _, material) in &self.transparent {
            let node = s.get_mut(*h)?;
            node.visible = true;
            if let NodeKind::Mesh(mesh) = &mut node.kind {
                mesh.materials[0] = material.clone();
            }
        }
        result?;
        s.get_mut(self.opaque)?.visible = false;
        let result = r.render(s, c, &self.accum);
        s.get_mut(self.opaque)?.visible = true;
        result?;
        self.composite
            .apply(r, &self.beauty, Some(&self.accum), out)?;
        Ok(())
    }
}
