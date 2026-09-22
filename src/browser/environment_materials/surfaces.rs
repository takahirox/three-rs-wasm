use super::*;
impl Demo {
    pub(super) async fn surfaces(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        r: &Renderer,
        aspect: f64,
    ) -> Result<()> {
        if self.id == 180 {
            self.params = [1., 0.4, 1., 0.2, 1., 2.436143, 1., 0.];
            s.get_mut(c)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                left: -500. * aspect,
                right: 500. * aspect,
                top: 500.,
                bottom: -500.,
                near: 1.,
                far: 10000.,
                ..Default::default()
            }));
            let g = Arc::new(
                super::super::tsl_materials::geometries(
                    &fetch(&format!("{ASSETS}/ninja.bin")).await?,
                )?
                .remove(0),
            );
            let mut m = MeshStandardMaterial {
                energy_conservation: true,
                roughness: 0.4,
                metalness: 1.,
                normal_scale: Vector2::new(1., -1.),
                normal_map: Some(Arc::new(image("models/obj/ninja/normal.png", false).await?)),
                occlusion_map: Some(Arc::new(image("models/obj/ninja/ao.jpg", false).await?)),
                ..Default::default()
            };
            m.properties.color = Color::from_hex(0xc1c1c1);
            m.properties.side = Side::Double;
            let displacement = r.upload_texture(&Arc::new(
                image("models/obj/ninja/displacement.jpg", false).await?,
            ))?;
            let coordinate = vec2(uv().x(), float(1.) - uv().y());
            let h = tsl::Texture::External(0)
                .sample_level(coordinate, float(0.))
                .x();
            let position = position_geometry()
                + normal_geometry().normalize() * (h * uniform(0, Type::Float) - float(0.428408));
            m.properties.vertex_program = Some(Arc::new(
                SurfaceNodes {
                    position: Some(position),
                    ..Default::default()
                }
                .build(r, &[], &[(&displacement.view, &displacement.sampler)])
                .await?,
            ));
            m.properties.vertex_uniforms[0][0] = 2.436143;
            let h = mesh(s, g, Material::Standard(m));
            s.get_mut(h)?.scale = Vector3::splat(25.);
            s.get_mut(h)?.frustum_culled = false;
            self.objects.push(h);
            let cubemap = cube(r, "SwedishRoyalCastle").await?;
            s.environment = Some(Arc::new(
                crate::environment::EnvironmentMap::from_cube_texture(r, &cubemap)?,
            ));
            self.ambient = Some(s.insert(NodeKind::Light(Light::Ambient {
                color: Color::WHITE,
                intensity: 0.2,
            })));
            for (color, intensity, position, parent) in [
                (0xff0000, 1.5, Vector3::new(0., 0., 2500.), false),
                (0xff6666, 3., Vector3::ZERO, true),
                (0x0000ff, 1.5, Vector3::new(-1000., 0., 1000.), false),
            ] {
                let h = s.insert(NodeKind::Light(Light::Point {
                    color: Color::from_hex(color),
                    intensity,
                    distance: 0.,
                    decay: 0.,
                }));
                s.get_mut(h)?.position = position;
                if parent {
                    s.add(c, h)?;
                }
                if color == 0xff0000 {
                    self.light = Some(h);
                }
            }
        } else {
            self.params = [1., 10., 0., 0., 0., 0., 0., 0.];
            s.background = Color::from_hex(0x060708);
            let (a, b, images) = load_asset("/web/models/LeePerrySmith.glb").await?;
            let mut temp = Scene::default();
            crate::gltf::import_decoded(&a, &b, &images)?.instantiate(&mut temp)?;
            let g = temp
                .handles()
                .find_map(|h| match &temp.get(h).ok()?.kind {
                    NodeKind::Mesh(m) => Some(m.geometry.clone()),
                    _ => None,
                })
                .ok_or(Error::Invalid("head geometry"))?;
            let height = r.upload_texture(&Arc::new(
                image(
                    "models/gltf/LeePerrySmith/Infinite-Level_02_Disp_NoSmoothUV-4096.jpg",
                    false,
                )
                .await?,
            ))?;
            let normal = tsl::surface::bump_map(
                |uv| tsl::Texture::External(0).sample(uv).x(),
                vec2(uv().x(), float(1.) - uv().y()),
                uniform(0, Type::Float),
            );
            let mut m = MeshPhongMaterial {
                specular: Color::from_hex(0x666666),
                shininess: 25.,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(0x9c6e49);
            m.properties.shadow_program =
                Some(Arc::new(tsl::surface::shadow_program(r, None, None).await?));
            m.properties.vertex_program = Some(Arc::new(
                SurfaceNodes {
                    normal: Some(normal),
                    ..Default::default()
                }
                .build(r, &[], &[(&height.view, &height.sampler)])
                .await?,
            ));
            self.bump_program = m.properties.vertex_program.clone();
            m.properties.vertex_uniforms[0][0] = 10.;
            let h = mesh(s, g, Material::Phong(m));
            let n = s.get_mut(h)?;
            n.position.y = -0.5;
            n.cast_shadow = true;
            n.receive_shadow = true;
            self.objects.push(h);
            let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
                sky: Color::from_hex(0x8d7c7c),
                ground: Color::from_hex(0x494966),
                intensity: 3.,
            }));
            s.get_mut(hemi)?.position.y = 1.;
            let spot = s.insert(NodeKind::Light(Light::Spot {
                color: Color::from_hex(0xffffde),
                intensity: 200.,
                target: Vector3::ZERO,
                distance: 0.,
                decay: 2.,
                angle: std::f64::consts::FRAC_PI_3,
                penumbra: 0.,
            }));
            let n = s.get_mut(spot)?;
            n.position = Vector3::new(3.5, 0., 7.);
            n.cast_shadow = true;
            n.shadow.map_size = Some(2048);
            n.shadow.near = 2.;
            n.shadow.far = 15.;
            n.shadow.bias = -0.005;
        }
        Ok(())
    }
    pub(super) fn update_surfaces(&mut self, s: &mut Scene, _c: Object3D) -> Result<()> {
        if self.id == 180 {
            if let Some(h) = self.light {
                s.get_mut(h)?.position = Vector3::new(
                    2500. * (self.time * 0.6).cos(),
                    0.,
                    2500. * (self.time * 0.6).sin(),
                );
            }
            s.environment_intensity = self.params[4] as f64;
            if let Some(h) = self.ambient
                && let NodeKind::Light(Light::Ambient { intensity, .. }) = &mut s.get_mut(h)?.kind
            {
                *intensity = self.params[3] as f64;
            }
        }
        if self.dirty {
            for &h in &self.objects {
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(h)?.kind {
                    let m = Arc::make_mut(&mut mesh.materials[0]);
                    if let Material::Standard(m) = m {
                        m.metalness = self.params[0] as f64;
                        m.roughness = self.params[1] as f64;
                        m.occlusion_strength = self.params[2] as f64;
                        m.normal_scale = Vector2::new(1., -1.) * self.params[6] as f64;
                        m.properties.vertex_uniforms[0][0] = self.params[5];
                    } else {
                        let properties = m.properties_mut();
                        properties.vertex_program = if self.params[0] > 0.5 {
                            self.bump_program.clone()
                        } else {
                            None
                        };
                        properties.vertex_uniforms[0][0] = self.params[1];
                    }
                }
            }
        }
        Ok(())
    }
}
