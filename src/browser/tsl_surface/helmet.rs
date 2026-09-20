use super::*;
use crate::tsl::*;
impl Demo {
    pub(super) async fn fog_background(
        scene: &mut Scene,
        cam: Object3D,
        r: &Renderer,
    ) -> Result<Self> {
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.0,
            aspect,
            near: 0.25,
            far: 20.0,
            ..Default::default()
        }));
        let position = Vector3::new(-1.8, 0.6, 2.7);
        let target = Vector3::new(0.0, -0.1, -0.2);
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, target)?;
        let (asset, buffers, images) = super::super::gltf_viewer::load_asset(
            "/web/models/DamagedHelmet/glTF/DamagedHelmet.gltf",
        )
        .await?;
        crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
        scene.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_hdr(
            &super::super::gltf_viewer::fetch("/web/environments/royal_esplanade_2k.hdr").await?,
        )?));
        scene.background = Color::BLACK;
        scene.background_alpha = 0.0;
        let input = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples: 4,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let fog = Color::from_hex(0x4080cc).0;
        let tonemap = WgslFn::new(
            "fog_tonemap",
            &format!(
                "{}\nfn fog_tonemap(value:vec3<f32>)->vec3<f32>{{return tone_output(value,1.0,1.0);}}",
                include_str!("../../shaders/output.wgsl")
            ),
            &[Type::Vec3],
            Type::Vec3,
        )?;
        let distance =
            float(0.25 * 20.0) / (float(20.0) - depth_texture(tsl::uv()) * float(20.0 - 0.25));
        let color = mix(
            tonemap.call(&[tsl::Texture::Input.sample(tsl::uv()).rgb()]),
            vec3(
                float(fog.x as f32),
                float(fog.y as f32),
                float(fog.z as f32),
            ),
            distance.smoothstep(float(2.7), float(4.0)),
        );
        let effect = multisampled_depth_effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &color,
            &input
                .depth_texture()
                .unwrap()
                .create_view(&Default::default()),
        )
        .await?;
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        Ok(Self {
            example: 67,
            time: 0.0,
            lights: vec![],
            objects: vec![],
            params: [[0.0; 4]; 16],
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: Some((input, effect)),
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
}
impl Demo {
    pub(super) async fn material_mrt(
        scene: &mut Scene,
        cam: Object3D,
        r: &Renderer,
    ) -> Result<Self> {
        let mut demo = Self::fog_background(scene, cam, r).await?;
        demo.example = 70;
        let position = Vector3::new(-1.8, 0.6, 2.7);
        let target = Vector3::new(0.0, 0.0, -0.2);
        let offset = position - target;
        demo.viewer = OrbitViewer::from_camera(target, offset.length());
        demo.viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        scene.background_environment = true;
        scene.background_outputs = vec![
            BackgroundOutput::Color,
            BackgroundOutput::NormalView,
            BackgroundOutput::Color,
            BackgroundOutput::Zero,
        ];
        scene.background_alpha = 1.0;
        let program = Arc::new(
            SurfaceNodes::default()
                .build_mrt(
                    r,
                    &[
                        output(),
                        normal_view() * float(0.5) + float(0.5),
                        diffuse_color(),
                        emissive(),
                    ],
                    &[],
                    &[],
                )
                .await?,
        );
        for root in scene.roots() {
            for h in scene.traverse(root, true)? {
                if let NodeKind::Mesh(m) = &mut scene.get_mut(h)?.kind {
                    for material in &mut m.materials {
                        Arc::make_mut(material).properties_mut().vertex_program =
                            Some(program.clone());
                    }
                }
            }
        }
        let input = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                count: 4,
                samples: 4,
                format: wgpu::TextureFormat::Rgba16Float,
                color_formats: vec![
                    wgpu::TextureFormat::Rgba16Float,
                    wgpu::TextureFormat::Rgba8Unorm,
                    wgpu::TextureFormat::Rgba8Unorm,
                    wgpu::TextureFormat::Rgba8Unorm,
                ],
                ..Default::default()
            },
        )?;
        let sampler = r.device.create_sampler(&Default::default());
        let views: Vec<_> = input
            .textures()
            .iter()
            .map(|t| t.create_view(&Default::default()))
            .collect();
        let color = tsl::Texture::External(0).sample(tsl::uv());
        let tonemap = WgslFn::new(
            "mrt_display",
            &format!(
                "{}\nfn mrt_display(value:vec3<f32>)->vec3<f32>{{return srgb_output(aces_output(value,1.0));}}",
                include_str!("../../shaders/output.wgsl")
            ),
            &[Type::Vec3],
            Type::Vec3,
        )?;
        let mut out = tsl::uv()
            .x()
            .less_than(float(0.2))
            .select(vec4(tonemap.call(&[color.rgb()]), float(1.0)), color);
        for (edge, index) in [(0.4, 1), (0.6, 3), (0.8, 2)] {
            out = tsl::uv()
                .x()
                .less_than(float(edge))
                .select(out, tsl::Texture::External(index).sample(tsl::uv()));
        }
        let effect = effect_with_textures(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &out,
            &views.iter().map(|v| (v, &sampler)).collect::<Vec<_>>(),
        )
        .await?;
        demo.depth = Some((input, effect));
        demo.mrt_sampler = Some(sampler);
        Ok(demo)
    }
}
