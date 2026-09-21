use super::*;
use crate::postprocessing::Effect;
pub(super) struct Post {
    scene: RenderTarget,
    intermediate: RenderTarget,
    blurred: RenderTarget,
    filter: Effect,
    finish: Effect,
    bloom: Option<tsl::bloom::Bloom>,
    gaussian: Option<Effect>,
    flare: Option<(RenderTarget, Effect)>,
    sampler: wgpu::Sampler,
}
async fn environment(name: &str) -> Result<EnvironmentMap> {
    let bytes = fetch(&format!("{ASSETS}/{name}.rgba16f.png")).await?;
    // PNG is a lossless container for half-float bit patterns, not color data.
    let image = image::load_from_memory(&bytes)
        .map_err(|e| Error::Asset(e.to_string()))?
        .to_rgba16();
    let (width, height) = image.dimensions();
    let rgba = image
        .as_raw()
        .iter()
        .map(|v| half::f16::from_bits(*v))
        .collect();
    Ok(EnvironmentMap {
        width,
        height,
        rgba,
        gpu: None,
    })
}
impl Demo {
    pub(super) async fn post(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let dof = self.example == 111;
        let name = if dof {
            "bath_day.glb"
        } else {
            "space_ship_hallway.glb"
        };
        let (a, b, i) = load_asset(&format!("{ASSETS}/models/gltf/{name}")).await?;
        let imported = crate::gltf::import_animated_decoded(&a, &b, &i)?;
        let center = imported.bounds.center();
        let instance = imported.instantiate(s)?;
        self.objects = instance.meshes.clone();
        if dof {
            let mut mixer = crate::animation::AnimationMixer::default();
            mixer.play(instance.clips[0].clone())?;
            self.mixer = Some(mixer);
            s.background = Color::from_hex(0x90d5ff);
            s.environment_rotation = -std::f64::consts::FRAC_PI_2;
            self.params[..4].copy_from_slice(&[1.0, 3.0, 2.0, 4.0]);
        } else {
            let group = s.insert(NodeKind::Group);
            s.get_mut(group)?.position = -center;
            for h in instance.roots {
                s.add(group, h)?;
            }
            s.background_environment = true;
            s.background_intensity = 2.0;
            s.environment_intensity = 15.0;
            s.tone_mapping = ToneMapping::Aces;
            self.params[..6].copy_from_slice(&[1.0, 1.0, 0.5, 25.0, 0.25, 1.0]);
        }
        s.environment = Some(Arc::new(
            environment(if dof {
                "spruit_sunrise_2k.hdr.jpg"
            } else {
                "ice_planet_close.jpg"
            })
            .await?,
        ));
        for h in &self.objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                for material in &mut m.materials {
                    match Arc::make_mut(material) {
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
                count: if dof { 1 } else { 2 },
                ..Default::default()
            },
        )?;
        let target = |format| {
            RenderTarget::with_options(
                &r.device,
                1,
                1,
                RenderTargetOptions {
                    format,
                    depth_buffer: false,
                    ..Default::default()
                },
            )
        };
        let filter_format = if dof {
            HDR
        } else {
            wgpu::TextureFormat::Rgba8Unorm
        };
        let intermediate = target(HDR)?;
        let blurred = target(HDR)?;
        let flare = if dof {
            None
        } else {
            Some((
                target(filter_format)?,
                tsl::effect(r, HDR, &tsl::Texture::Input.sample(uv())).await?,
            ))
        };
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            min_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let (filter, finish, bloom, gaussian) = if dof {
            let p = uniform(0, Type::Vec4);
            let z = -float(0.1 * 100.0) / (float(100.0) - depth_texture(uv()) * float(99.9));
            let blend = (z - uniform(1, Type::Float)).abs().smoothstep(p.x(), p.y());
            let blurred = tsl::display::box_blur(
                tsl::Texture::Input,
                uv(),
                p.swizzle("z"),
                p.swizzle("w"),
                float(0.0).greater_than(float(1.0)),
            );
            let color = mix(tsl::Texture::Input.sample(uv()), blurred, blend).rgb();
            let encode = WgslFn::new(
                "basic_dof_output",
                &format!(
                    "{}\nfn basic_dof_output(c:vec3<f32>)->vec3<f32>{{return srgb_output(tone_output(c,1.0,3.0));}}",
                    include_str!("../../shaders/output.wgsl")
                ),
                &[Type::Vec3],
                Type::Vec3,
            )?;
            let filter = tsl::depth_effect(
                r,
                HDR,
                &vec4(encode.call(&[color]), float(1.0)),
                &scene
                    .depth_texture()
                    .unwrap()
                    .create_view(&Default::default()),
            )
            .await?;
            let finish = tsl::effect(
                r,
                HDR,
                &tsl::display::fxaa(tsl::Texture::Input, uv(), uniform(0, Type::Vec2)),
            )
            .await?;
            (filter, finish, None, None)
        } else {
            let program = Arc::new(
                SurfaceNodes::default()
                    .build_mrt(
                        r,
                        &[output(), vec4(emissive(), output().swizzle("w"))],
                        &[],
                        &[],
                    )
                    .await?,
            );
            for h in &self.objects {
                if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                    for m in &mut m.materials {
                        Arc::make_mut(m).properties_mut().vertex_program = Some(program.clone());
                    }
                }
            }
            s.background_outputs = vec![BackgroundOutput::Color, BackgroundOutput::Zero];
            let mut bloom = tsl::bloom::Bloom::with_input(
                r,
                tsl::Texture::External(0).sample(uv()),
                &[(
                    &scene.textures()[1].create_view(&Default::default()),
                    &sampler,
                )],
            )
            .await?;
            bloom.threshold = 0.0;
            let filter = tsl::effect(
                r,
                filter_format,
                &tsl::display::lensflare(
                    tsl::Texture::Input,
                    uv(),
                    splat(float(1.0), Type::Vec3),
                    uniform(0, Type::Vec4),
                ),
            )
            .await?;
            let gaussian = tsl::effect(
                r,
                HDR,
                &gaussian_blur(tsl::Texture::Input, uv(), uniform(0, Type::Vec2), 4)?,
            )
            .await?;
            let finish = tsl::effect_with_textures(
                r,
                HDR,
                &(tsl::Texture::Input.sample(uv())
                    + tsl::Texture::History.sample(uv())
                    + tsl::Texture::External(0).sample(uv())),
                &[(
                    &intermediate.texture.create_view(&Default::default()),
                    &sampler,
                )],
            )
            .await?;
            (filter, finish, Some(bloom), Some(gaussian))
        };
        self.post = Some(Post {
            scene,
            intermediate,
            blurred,
            filter,
            finish,
            bloom,
            gaussian,
            flare,
            sampler,
        });
        Ok(())
    }
}
impl Post {
    pub(super) fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        p: &[f32; 8],
        focus: Vector3,
    ) -> Result<()> {
        let dof = self.bloom.is_none();
        let (w, h) = (out.width, out.height);
        if (self.scene.width, self.scene.height) != (w, h) {
            self.scene.set_size(&r.device, w, h)?;
            let (sw, sh) = if dof {
                (w, h)
            } else {
                (
                    ((w as f64 / 4.0).round() as u32).max(1),
                    ((h as f64 / 4.0).round() as u32).max(1),
                )
            };
            self.intermediate.set_size(&r.device, w, h)?;
            if !dof {
                self.blurred.set_size(&r.device, w, h)?;
            }
            if let Some((flare, _)) = &mut self.flare {
                flare.set_size(&r.device, sw, sh)?;
            }
            if let Some(bloom) = &mut self.bloom {
                bloom.set_textures(
                    r,
                    &[(
                        &self.scene.textures()[1].create_view(&Default::default()),
                        &self.sampler,
                    )],
                )?;
                self.finish.set_textures(
                    r,
                    &[(
                        &self.intermediate.texture.create_view(&Default::default()),
                        &self.sampler,
                    )],
                )?;
            } else {
                self.filter.set_depth(
                    r,
                    &self
                        .scene
                        .depth_texture()
                        .unwrap()
                        .create_view(&Default::default()),
                )?;
            }
        }
        r.render(s, c, &self.scene)?;
        if let Some(bloom) = &mut self.bloom {
            bloom.strength = p[0];
            bloom.radius = p[1];
            let bloom = bloom.render(r, &self.scene)?;
            self.filter.parameters[0] = [p[2], 4.0, p[4], p[3]];
            let (flare, copy) = self.flare.as_ref().unwrap();
            self.filter.apply(r, bloom, None, flare)?;
            copy.apply(r, flare, None, &self.intermediate)?;
            let gaussian = self.gaussian.as_mut().unwrap();
            gaussian.parameters[0] = [8.0 / self.intermediate.width as f32, 0.0, 0.0, 0.0];
            gaussian.apply(r, &self.intermediate, None, &self.blurred)?;
            gaussian.parameters[0] = [0.0, 8.0 / self.intermediate.height as f32, 0.0, 0.0];
            gaussian.apply(r, &self.blurred, None, &self.intermediate)?;
            self.finish.apply(r, &self.scene, Some(bloom), out)?;
        } else {
            let view = s.camera(c)?.1.inverse();
            self.filter.parameters[0].copy_from_slice(&p[..4]);
            self.filter.parameters[1][0] = view.transform_point3(focus).z as f32;
            self.filter
                .apply(r, &self.scene, None, &self.intermediate)?;
            self.finish.parameters[0] = [1.0 / w as f32, 1.0 / h as f32, 0.0, 0.0];
            self.finish.apply(r, &self.intermediate, None, out)?;
        }
        Ok(())
    }
}
