use super::*;
impl Demo {
    pub(super) async fn gather(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.0,
            right: 1.0,
            top: 1.0,
            bottom: -1.0,
            near: 0.1,
            far: 2000.0,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0.0, 0.0, 2.0);
        s.look_at(c, Vector3::ZERO)?;
        s.background = Color::from_hex(0x313131);
        let mut source = Scene::new();
        source.background = Color::from_hex(0x808080);
        let camera = source.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.0,
            aspect: 1.0,
            near: 2.5 - 0.5 * 3.0_f64.sqrt(),
            far: 2.5 + 0.5 * 3.0_f64.sqrt(),
            ..Default::default()
        })));
        source.get_mut(camera)?.position.z = 2.5;
        let light = source.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 1.0,
            target: Vector3::ZERO,
        }));
        source.get_mut(light)?.position = Vector3::new(1.0, 1.0, 0.0);
        source.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.1,
        }));
        let mut material = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        material.properties.color = Color::from_hex(0xff0000);
        let object = mesh(
            &mut source,
            Arc::new(BoxGeometry::build(1.0, 1.0, 1.0)?),
            Material::Standard(material),
        );
        source.get_mut(object)?.quaternion = Quaternion::from_euler(
            glam::EulerRot::XYZ,
            std::f64::consts::FRAC_PI_4,
            std::f64::consts::FRAC_PI_4,
            0.0,
        );
        let target = RenderTarget::with_options(
            &r.device,
            100,
            100,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..Default::default()
            },
        )?;
        r.render(&mut source, camera, &target)?;
        let color_view = target.texture.create_view(&Default::default());
        let depth_view = target
            .depth_texture()
            .unwrap()
            .create_view(&Default::default());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let comparison = r.device.create_sampler(&wgpu::SamplerDescriptor {
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let color =
            tsl::sampling::gather(tsl::Texture::External(0), uv() * float(10.0), 0, [0, 7])?;
        let depth =
            tsl::sampling::gather_compare(tsl::Texture::External(1), uv(), float(1.0), [0, 7])?;
        let graph = NodeMaterial::new(uv().y().greater_than(float(0.5)).select(color, depth));
        let material = graph
            .build_with_texture_types(
                r,
                &[
                    (&color_view, &sampler, Type::Texture),
                    (&depth_view, &comparison, Type::DepthTexture),
                ],
            )
            .await?;
        self.objects.push(mesh(
            s,
            Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1)?),
            Material::Shader(material),
        ));
        Ok(())
    }
}
