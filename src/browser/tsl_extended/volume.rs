use super::*;
impl Demo {
    pub(super) async fn volume(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let compute = self.example == 83;
        let cloud = self.example != 81;
        let size = if compute { 200 } else { 128 };
        let kind = if cloud { "cloud" } else { "perlin" };
        let data = if compute {
            vec![]
        } else {
            fetch(&format!("/web/gallery/assets/volume-{kind}.raw")).await?
        };
        if !compute && data.len() != 128 * 128 * 128 {
            return Err(Error::Invalid("volume texture data"));
        }
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("resident noise volume"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: size,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: if compute {
                wgpu::TextureFormat::Rgba8Unorm
            } else {
                wgpu::TextureFormat::R8Unorm
            },
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::TEXTURE_BINDING
                | if compute {
                    wgpu::TextureUsages::STORAGE_BINDING
                } else {
                    wgpu::TextureUsages::empty()
                },
            view_formats: &[],
        });
        if !compute {
            r.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(128),
                    rows_per_image: Some(128),
                },
                texture.size(),
            );
        }
        let view = texture.create_view(&Default::default());
        if compute {
            let buffer = GpuBuffer::zeroed(r, 16, BufferAccess::Uniform)?;
            let source = format!(
                "{}\n{}",
                include_str!("../../tsl/noise.wgsl"),
                include_str!("volume_compute.wgsl")
            );
            let kernel = crate::compute::ComputeKernel::with_texture_dimensions(
                r,
                &source,
                &[&buffer],
                &[(
                    &view,
                    wgpu::TextureFormat::Rgba8Unorm,
                    wgpu::StorageTextureAccess::WriteOnly,
                    wgpu::TextureViewDimension::D3,
                )],
            )
            .await?;
            self.volume_compute = Some((kernel, buffer));
        }
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let t = tsl::Texture::External(0);
        let origin = uniform(1, Type::Vec3);
        let p = position_local();
        let options = uniform(0, Type::Vec4);
        let color = if cloud {
            let c = if compute {
                tsl::volume::computed_cloud(t, origin, p, options)
            } else {
                tsl::volume::cloud(t, origin, p, options)
            };
            vec4(c.rgb() + rgb(0x798aa0), c.swizzle("w"))
        } else {
            tsl::volume::opaque(t, origin, p, options)
        };
        let mut m = NodeMaterial::new(color)
            .build_with_texture_types(r, &[(&view, &sampler, Type::Texture3D)])
            .await?;
        m.properties.side = Side::Back;
        m.properties.transparent = true;
        self.objects.push(mesh(
            s,
            Arc::new(BoxGeometry::build(
                if compute { 10.0 } else { 1.0 },
                if compute { 10.0 } else { 1.0 },
                if compute { 10.0 } else { 1.0 },
            )?),
            Material::Shader(m),
        ));
        if compute {
            self.viewer = OrbitViewer::from_camera(Vector3::ZERO, 3.25_f64.sqrt());
            self.viewer
                .fixture(0.0, (1.0 / 3.25_f64.sqrt()).asin(), 1.8);
        } else {
            self.viewer.fixture(0.0, 0.0, 1.8);
        }
        if cloud {
            self.params[..4].copy_from_slice(if compute {
                &[0.08, 0.08, 0.1, 100.0]
            } else {
                &[0.25, 0.25, 0.1, 100.0]
            });
            self.sky(s, r).await?;
        } else {
            self.params[..3].copy_from_slice(&[0.6, 200.0, 1.0]);
        }
        Ok(())
    }
    async fn sky(&self, s: &mut Scene, r: &Renderer) -> Result<()> {
        // Canvas gradient samples at pixel centers in sRGB, then the GPU filters linear color.
        let mut data = vec![];
        for y in 0..32 {
            let t = (y as f64 + 0.5) / 32.0;
            let (a, b, t) = if t < 0.5 {
                ([1.0, 74.0, 132.0], [5.0, 97.0, 160.0], t * 2.0)
            } else {
                ([5.0, 97.0, 160.0], [67.0, 122.0, 182.0], t * 2.0 - 1.0)
            };
            for c in 0..3 {
                data.push((a[c] * (1.0 - t) + b[c] * t).round() as u8);
            }
            data.push(255);
        }
        let mut picture = crate::material::Texture::from_rgba(1, 32, data, true)?;
        picture.mipmap_filter = Some(Filter::Linear);
        let picture = r.upload_texture(&Arc::new(picture))?;
        let mut m = NodeMaterial::new(
            tsl::Texture::External(0).sample(vec2(uv().x(), float(1.0) - uv().y())),
        )
        .build(r, &[(&picture.view, &picture.sampler)])
        .await?;
        m.properties.side = Side::Back;
        mesh(
            s,
            Arc::new(SphereGeometry::build(10.0, 32, 16)?),
            Material::Shader(m),
        );
        Ok(())
    }
    pub(super) fn update_volume(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.1;
            p.far = 100.0;
        }
        let h = self.objects[0];
        let angle = if self.example == 82 {
            -self.time / 7.5
        } else if self.example == 83 {
            std::f64::consts::FRAC_PI_2
        } else {
            0.0
        };
        let rotation = Quaternion::from_rotation_y(angle);
        s.get_mut(h)?.quaternion = rotation;
        let origin = rotation.inverse() * s.get(c)?.position;
        if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[0].copy_from_slice(&self.params[..4]);
            m.uniforms[1] = [origin.x as f32, origin.y as f32, origin.z as f32, 0.0];
        }
        Ok(())
    }
}
