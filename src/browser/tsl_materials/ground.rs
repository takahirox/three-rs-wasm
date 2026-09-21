use super::*;
impl Demo {
    pub(super) async fn ground(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let mut env = crate::environment::EnvironmentMap::from_hdr(
            &fetch(&format!(
                "{ASSETS}/textures/equirectangular/blouberg_sunrise_2_1k.hdr"
            ))
            .await?,
        )?;
        let size = env.height;
        let gpu = env.prefilter(r)?;
        let cube = GpuTexture::from_equirectangular(r, &gpu.source, size).await?;
        s.environment = Some(Arc::new(env));
        let (a, b, i) =
            super::super::gltf_viewer::load_asset(&format!("{ASSETS}/models/gltf/ferrari.glb"))
                .await?;
        let imported = crate::gltf::import_animated_decoded(&a, &b, &i)?;
        let instance = imported.instantiate(s)?;
        let root = instance.roots[0];
        s.get_mut(root)?.matrix_auto_update = true;
        s.get_mut(root)?.scale *= 4.0;
        s.get_mut(root)?.quaternion = Quaternion::from_rotation_y(std::f64::consts::PI);
        for h in instance.meshes {
            let n = s.get_mut(h)?;
            let name = n.name.clone();
            if let NodeKind::Mesh(mesh) = &mut n.kind {
                let mut replacement = None;
                if name == "body" {
                    let mut m = MeshPhysicalMaterial {
                        clearcoat: 1.0,
                        clearcoat_roughness: 0.2,
                        ..Default::default()
                    };
                    m.base.roughness = 0.8;
                    m.base.metalness = 1.0;
                    m.base.properties.color = Color::BLACK;
                    replacement = Some(Material::Physical(m));
                } else if name == "glass" {
                    let mut m = MeshPhysicalMaterial {
                        transmission: 1.0,
                        ..Default::default()
                    };
                    m.base.roughness = 0.0;
                    m.base.metalness = 0.25;
                    replacement = Some(Material::Physical(m));
                } else if ["rim_fl", "rim_fr", "rim_rl", "rim_rr", "trim"].contains(&name.as_str())
                {
                    replacement = Some(Material::Standard(MeshStandardMaterial {
                        roughness: 0.5,
                        metalness: 1.0,
                        ..Default::default()
                    }));
                }
                if let Some(m) = replacement {
                    mesh.materials = vec![Arc::new(m)];
                }
                for m in &mut mesh.materials {
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
        let mut shadow =
            decode_texture_image(&fetch(&format!("{ASSETS}/models/gltf/ferrari_ao.png")).await?)
                .await?;
        shadow.srgb = false;
        shadow.mipmap_filter = Some(Filter::Linear);
        let mut m = MeshBasicMaterial::default();
        m.properties.map = Some(Arc::new(shadow));
        m.properties.transparent = true;
        let blend = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::Zero,
            dst_factor: wgpu::BlendFactor::Src,
            operation: wgpu::BlendOperation::Add,
        };
        m.properties.blending = Some(wgpu::BlendState {
            color: blend,
            alpha: blend,
        });
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(0.655 * 4.0, 1.3 * 4.0, 1, 1)?),
            Material::Basic(m),
        );
        s.get_mut(h)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        s.add(root, h)?;
        let camera = WgslFn::new(
            "ground_camera",
            "fn ground_camera()->vec3<f32>{return u.camera.xyz;}",
            &[],
            Type::Vec3,
        )?
        .call(&[]);
        let direction = tsl::environment::ground_projected_normal(
            position_world(),
            camera,
            float(100.0),
            float(15.0),
        );
        let color=WgslFn::new("ground_cube","fn ground_cube(t:texture_cube<f32>,s:sampler,d:vec3<f32>)->vec4<f32>{return textureSampleLevel(t,s,d,0.0);}",&[Type::TextureCube,Type::Sampler,Type::Vec3],Type::Vec4)?.call(&[tsl::Texture::External(0).node(),tsl::Texture::External(0).sampler(),direction]);
        let mut m = NodeMaterial::new(color)
            .build_with_texture_types(r, &[(&cube.view, &cube.sampler, Type::TextureCube)])
            .await?;
        m.properties.side = Side::Double;
        let sky = mesh(
            s,
            Arc::new(IcosahedronGeometry::build(1.0, 16)?),
            Material::Shader(m),
        );
        s.get_mut(sky)?.scale = Vector3::splat(100.0);
        self.sky = Some(sky);
        self.params[0] = 1.0;
        Ok(())
    }
}
