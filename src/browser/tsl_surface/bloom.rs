use super::*;
use crate::tsl::*;
pub(super) struct BloomPass {
    pub input: RenderTarget,
    pub bloom: tsl::bloom::Bloom,
    combine: crate::postprocessing::Effect,
    sampler: wgpu::Sampler,
    masked: bool,
    intensity_mask: bool,
}
impl BloomPass {
    pub async fn new(r: &Renderer, samples: u32, masked: bool) -> Result<Self> {
        let input = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples,
                count: if masked { 2 } else { 1 },
                format: wgpu::TextureFormat::Rgba16Float,
                color_formats: if masked {
                    vec![
                        wgpu::TextureFormat::Rgba16Float,
                        wgpu::TextureFormat::Rgba8Unorm,
                    ]
                } else {
                    vec![]
                },
                ..Default::default()
            },
        )?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let bloom = if masked {
            tsl::bloom::Bloom::with_input(
                r,
                tsl::Texture::External(0).sample(uv()),
                &[(
                    &input.textures()[1].create_view(&Default::default()),
                    &sampler,
                )],
            )
            .await?
        } else {
            tsl::bloom::Bloom::new(r).await?
        };
        let combine = effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &(tsl::Texture::Input.sample(uv()) + tsl::Texture::History.sample(uv())),
        )
        .await?;
        Ok(Self {
            input,
            bloom,
            combine,
            sampler,
            masked,
            intensity_mask: false,
        })
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        scene: &mut Scene,
        cam: Object3D,
        target: &RenderTarget,
    ) -> Result<()> {
        if (self.input.width, self.input.height) != (target.width, target.height) {
            self.input
                .set_size(&r.device, target.width, target.height)?;
            if self.intensity_mask {
                let views: Vec<_> = self
                    .input
                    .textures()
                    .iter()
                    .map(|t| t.create_view(&Default::default()))
                    .collect();
                self.bloom.set_textures(
                    r,
                    &views.iter().map(|v| (v, &self.sampler)).collect::<Vec<_>>(),
                )?;
            } else if self.masked {
                self.bloom.set_textures(
                    r,
                    &[(
                        &self.input.textures()[1].create_view(&Default::default()),
                        &self.sampler,
                    )],
                )?;
            }
        }
        r.render(scene, cam, &self.input)?;
        let bloom = self.bloom.render(r, &self.input)?;
        self.combine.apply(r, &self.input, Some(bloom), target)
    }
}
impl Demo {
    pub(super) async fn bloom_model(
        scene: &mut Scene,
        cam: Object3D,
        r: &Renderer,
        example: u32,
    ) -> Result<Self> {
        let emissive_only = example == 72;
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: if emissive_only { 45.0 } else { 40.0 },
            aspect,
            near: if emissive_only { 0.25 } else { 1.0 },
            far: if emissive_only { 20.0 } else { 100.0 },
            ..Default::default()
        }));
        let position = if emissive_only {
            Vector3::new(-1.8, 0.6, 2.7)
        } else {
            Vector3::new(-5.0, 2.5, -3.5)
        };
        let target = if emissive_only {
            Vector3::new(0.0, 0.0, -0.2)
        } else {
            Vector3::ZERO
        };
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, target)?;
        let (asset, buffers, images) = super::super::gltf_viewer::load_asset(if emissive_only {
            "/web/models/DamagedHelmet/glTF/DamagedHelmet.gltf"
        } else {
            "/web/models/PrimaryIonDrive.glb"
        })
        .await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
        if !emissive_only {
            for &h in &instance.meshes {
                if let NodeKind::Mesh(m) = &mut scene.get_mut(h)?.kind {
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
        }
        let mixer = if emissive_only {
            None
        } else {
            let mut mixer = crate::animation::AnimationMixer::default();
            mixer.play(instance.clips[0].clone())?;
            Some(mixer)
        };
        if emissive_only {
            scene.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_hdr(
                &super::super::gltf_viewer::fetch("/web/environments/moonless_golf_1k.hdr").await?,
            )?));
            scene.background_environment = true;
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
            for h in instance.meshes {
                if let NodeKind::Mesh(m) = &mut scene.get_mut(h)?.kind {
                    for material in &mut m.materials {
                        Arc::make_mut(material).properties_mut().vertex_program =
                            Some(program.clone());
                    }
                }
            }
        } else {
            scene.background = Color::BLACK;
            scene.background_alpha = 0.0;
            scene.insert(NodeKind::Light(Light::Ambient {
                color: Color::from_hex(0xcccccc),
                intensity: 1.0,
            }));
            let light = scene.insert(NodeKind::Light(Light::Point {
                color: Color::WHITE,
                intensity: 100.0,
                distance: 0.0,
                decay: 2.0,
            }));
            scene.add(cam, light)?;
        }
        scene.tone_mapping = if emissive_only {
            ToneMapping::Aces
        } else {
            ToneMapping::Reinhard
        };
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        let mut params = [[0.0; 4]; 16];
        params[1] = [
            0.0,
            if emissive_only { 2.5 } else { 1.0 },
            if emissive_only { 0.5 } else { 0.0 },
            1.0,
        ];
        Ok(Self {
            example,
            time: 0.0,
            lights: vec![],
            objects: vec![],
            params,
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer,
            depth: None,
            mrt_sampler: None,
            storage: None,
            jelly: None,
            mask: None,
            bloom: Some(BloomPass::new(r, if emissive_only { 1 } else { 4 }, emissive_only).await?),
        })
    }
}
impl Demo {
    pub(super) async fn selective_bloom(
        scene: &mut Scene,
        cam: Object3D,
        r: &Renderer,
    ) -> Result<Self> {
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.0,
            aspect,
            near: 1.0,
            far: 200.0,
            ..Default::default()
        }));
        scene.get_mut(cam)?.position = Vector3::new(0.0, 0.0, 20.0);
        scene.look_at(cam, Vector3::ZERO)?;
        scene.background = Color::BLACK;
        scene.background_alpha = 0.0;
        scene.tone_mapping = ToneMapping::Neutral;
        let geometry = Arc::new(IcosahedronGeometry::build(1.0, 15)?);
        let program = Arc::new(
            SurfaceNodes::default()
                .build_mrt(r, &[output(), uniform(2, Type::Float)], &[], &[])
                .await?,
        );
        let mut seed = 186u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.0
        };
        let mut objects = Vec::new();
        for _ in 0..50 {
            let mut m = MeshBasicMaterial::default();
            m.properties.color = Color::from_hsl(random(), 0.7, random() * 0.2 + 0.05);
            m.properties.vertex_program = Some(program.clone());
            m.properties.vertex_uniforms[2][0] = if random() > 0.5 { 1.0 } else { 0.0 };
            let h = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Basic(m)),
            )));
            let p = Vector3::new(
                random() * 10.0 - 5.0,
                random() * 10.0 - 5.0,
                random() * 10.0 - 5.0,
            )
            .normalize()
                * (random() * 4.0 + 2.0);
            scene.get_mut(h)?.position = p;
            scene.get_mut(h)?.scale = Vector3::splat(random() * random() + 0.5);
            objects.push(h);
        }
        let mut pass = BloomPass::new(r, 1, true).await?;
        // Original mask is HalfFloat like the color attachment, and is multiplied before extraction.
        pass.input = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                count: 2,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let views: Vec<_> = pass
            .input
            .textures()
            .iter()
            .map(|t| t.create_view(&Default::default()))
            .collect();
        pass.bloom = tsl::bloom::Bloom::with_input(
            r,
            tsl::Texture::External(0).sample(uv()) * tsl::Texture::External(1).sample(uv()),
            &views.iter().map(|v| (v, &pass.sampler)).collect::<Vec<_>>(),
        )
        .await?;
        pass.intensity_mask = true;
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 20.0);
        viewer.fixture(0.0, 0.0, 1.8);
        let mut params = [[0.0; 4]; 16];
        params[1] = [0.0, 1.0, 0.0, 1.0];
        Ok(Self {
            example: 73,
            time: 0.0,
            lights: vec![],
            objects,
            params,
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            storage: None,
            jelly: None,
            mask: None,
            bloom: Some(pass),
        })
    }
    pub(in crate::browser) fn select(
        &mut self,
        scene: &mut Scene,
        cam: Object3D,
        x: f64,
        y: f64,
    ) -> Result<()> {
        if self.example != 73 {
            return Ok(());
        }
        scene.update()?;
        let (camera, world) = scene.camera(cam)?;
        let mut ray = crate::raycast::Raycaster::default();
        ray.set_from_camera(Vector2::new(x, y), camera, world)?;
        if let Some(hit) = ray.intersect_objects(scene, &self.objects, false)?.first()
            && let NodeKind::Mesh(m) = &mut scene.get_mut(hit.object)?.kind
        {
            let p = Arc::make_mut(&mut m.materials[0]).properties_mut();
            p.vertex_uniforms[2][0] = 1.0 - p.vertex_uniforms[2][0];
        }
        Ok(())
    }
}
