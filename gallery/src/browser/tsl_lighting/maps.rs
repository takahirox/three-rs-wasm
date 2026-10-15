use super::*;
impl Demo {
    pub(super) async fn maps(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let mut pixels = vec![];
        let mut faces = vec![];
        let mut size = 0;
        for face in ["px", "nx", "py", "ny", "pz", "nz"] {
            let path = if self.example == 108 {
                format!("/web/gallery/assets/tsl-environment/textures/cube/pisaHDR/{face}.hdr")
            } else {
                format!("{ASSETS}/textures/cube/Park3Med/{face}.jpg")
            };
            let bytes = fetch(&path).await?;
            if self.example == 108 {
                let im =
                    image::load_from_memory(&bytes).map_err(|e| Error::Asset(e.to_string()))?;
                size = im.width();
                pixels.extend(
                    im.to_rgba32f()
                        .as_raw()
                        .iter()
                        .map(|v| half::f16::from_f32(v.clamp(0.0, 65504.0))),
                );
            } else {
                let mut im = decode_texture_image(&bytes).await?;
                im.srgb = true;
                faces.push(im.clone());
                size = im.width;
                pixels.extend(im.rgba.iter().enumerate().map(|(i, v)| {
                    half::f16::from_f64(if i % 4 == 3 {
                        *v as f64 / 255.0
                    } else {
                        crate::math::srgb_to_linear(*v as f64 / 255.0)
                    })
                }));
            }
        }
        let env = EnvironmentMap::from_cube_hdr(r, size, &pixels)?;
        s.environment = Some(Arc::new(env));
        s.background_environment = true;
        s.background_blur = if self.example == 108 { 0.5 } else { 0.0 };
        if self.example == 108 {
            s.tone_mapping = ToneMapping::Aces;
            let g = Arc::new(SphereGeometry::build(0.4, 64, 64)?);
            for i in 0..6 {
                for j in 0..5 {
                    let m = MeshStandardMaterial {
                        roughness: i as f64 / 5.0,
                        metalness: j as f64 / 4.0,
                        energy_conservation: true,
                        ..Default::default()
                    };
                    let h = mesh(s, g.clone(), Material::Standard(m));
                    s.get_mut(h)?.position = Vector3::new(i as f64 - 2.5, j as f64 - 2.0, 0.0);
                }
            }
        } else {
            let cube = crate::texture_gpu::GpuTexture::from_cube_rgba(
                r,
                &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
            )?;
            let direction = normal_world();
            let color = WgslFn::new("sky_cube", "fn sky_cube(t:texture_cube<f32>,s:sampler,d:vec3<f32>)->vec4<f32>{return textureSampleLevel(t,s,d,0.0);}", &[Type::TextureCube,Type::Sampler,Type::Vec3],Type::Vec4)?.call(&[tsl::Texture::External(0).node(),tsl::Texture::External(0).sampler(),vec3(-direction.x(),direction.y(),direction.swizzle("z"))]);
            let mut sky = NodeMaterial::new(color)
                .build_with_texture_types(r, &[(&cube.view, &cube.sampler, Type::TextureCube)])
                .await?;
            sky.properties.side = Side::Back;
            sky.properties.depth_write = false;
            sky.properties.depth_test = false;
            let h = mesh(
                s,
                Arc::new(SphereGeometry::build(1.0, 32, 32)?),
                Material::Shader(sky),
            );
            s.get_mut(h)?.scale = Vector3::splat(10.0);
            s.get_mut(h)?.frustum_culled = false;
            s.get_mut(h)?.render_order = -10000;
            self.sky = Some(h);
            s.background_environment = false;
            let g = Arc::new(SphereGeometry::build(0.2, 64, 64)?);
            for (p, color) in [
                (Vector3::NEG_Z, 0x0000ff),
                (Vector3::Z, 0xff0000),
                (Vector3::X, 0xff00ff),
                (Vector3::NEG_X, 0x00ffff),
                (Vector3::NEG_Y, 0xffff00),
                (Vector3::Y, 0x00ff00),
            ] {
                let mut m = MeshBasicMaterial::default();
                m.properties.color = Color::from_hex(color);
                let h = mesh(s, g.clone(), Material::Basic(m));
                s.get_mut(h)?.position = p;
            }
            // PMREMGenerator.fromScene uses a 256px six-face GPU capture with no blur.
            let captured = EnvironmentMap::from_scene(r, s, 256, 0.0)?;
            let env = captured.gpu.as_ref().unwrap();
            let color = tsl::environment::pmrem(
                tsl::Texture::External(0),
                normal_world(),
                uniform(0, Type::Float),
                float(env.max_mip()),
            );
            let mut material = NodeMaterial::new(color)
                .build(r, &[(env.view(), env.sampler())])
                .await?;
            material.uniforms[0][0] = 0.5;
            self.objects.push(mesh(
                s,
                Arc::new(SphereGeometry::build(0.5, 64, 64)?),
                Material::Shader(material),
            ));
            self.params[0] = 0.5;
        }
        Ok(())
    }
}
