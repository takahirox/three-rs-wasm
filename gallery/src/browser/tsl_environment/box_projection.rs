use super::*;
impl Demo {
    pub(super) async fn box_projection(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let mut diffuse = super::super::gltf_viewer::decode_texture_image(
            &fetch(&format!("{NEXT_ASSETS}/textures/brick_diffuse.jpg")).await?,
        )
        .await?;
        diffuse.srgb = true;
        diffuse.mipmap_filter = Some(Filter::Linear);
        let mut bump = super::super::gltf_viewer::decode_texture_image(
            &fetch(&format!("{ASSETS}/textures/brick_bump.jpg")).await?,
        )
        .await?;
        bump.srgb = false;
        bump.mipmap_filter = Some(Filter::Linear);
        let bump = r.upload_texture(&Arc::new(bump))?;
        let node = bump_map(
            |uv| {
                tsl::Texture::External(0)
                    .sample(vec2(uv.x(), float(1.0) - uv.y()))
                    .x()
            },
            uv(),
            float(5.0),
        );
        let mut wall = standard(0xffffff, 1.0, 0.0);
        wall.properties.map = Some(Arc::new(diffuse));
        wall.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                normal: Some(node),
                environment: Some(splat(float(0.0), Type::Vec3)),
                ..Default::default()
            }
            .build(r, &[], &[(&bump.view, &bump.sampler)])
            .await?,
        ));
        let wall = Material::Standard(wall);
        let plane = Arc::new(PlaneGeometry::build(100.0, 100.0, 1, 1)?);
        for (x, z, angle) in [
            (-50.0, -50.0, 0.0),
            (50.0, -50.0, 0.0),
            (-50.0, 50.0, std::f64::consts::PI),
            (50.0, 50.0, std::f64::consts::PI),
            (100.0, 0.0, -std::f64::consts::FRAC_PI_2),
            (-100.0, 0.0, std::f64::consts::FRAC_PI_2),
        ] {
            let h = mesh(s, plane.clone(), wall.clone());
            let n = s.get_mut(h)?;
            n.position = Vector3::new(x, 0.0, z);
            n.quaternion = Quaternion::from_rotation_y(angle);
        }
        for (x, color) in [(-99.0, 0x9aaeff), (99.0, 0xf3aaaa)] {
            let h = s.insert(NodeKind::Light(Light::RectArea {
                color: Color::from_hex(color),
                intensity: 5.0,
                width: 50.0,
                height: 50.0,
            }));
            s.get_mut(h)?.position = Vector3::new(x, 5.0, 0.0);
            s.look_at(h, Vector3::new(0.0, 5.0, 0.0))?;
            let mut m = MeshBasicMaterial::default();
            m.properties.side = Side::Back;
            let helper = mesh(
                s,
                Arc::new(PlaneGeometry::build(50.0, 50.0, 1, 1)?),
                Material::Basic(m),
            );
            s.add(h, helper)?;
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(
                    vec![
                        25.0, 25.0, 0.0, -25.0, 25.0, 0.0, -25.0, -25.0, 0.0, 25.0, -25.0, 0.0,
                        25.0, 25.0, 0.0,
                    ],
                    3,
                    false,
                )?),
            );
            let line = s.insert(NodeKind::Line(Line {
                geometry: Arc::new(g),
                material: Arc::new(Material::Line(LineBasicMaterial::default())),
                segments: false,
            }));
            s.add(h, line)?;
        }
        let environment =
            EnvironmentMap::from_cube_scene(r, s, Vector3::new(0.0, -49.0, 0.0), 1.0, 1000.0, 512)?;
        let env = environment.gpu.as_ref().unwrap();
        let mut roughness = super::super::gltf_viewer::decode_texture_image(
            &fetch(&format!("{ASSETS}/textures/lava/lavatile.jpg")).await?,
        )
        .await?;
        roughness.srgb = false;
        roughness.wrap_s = Wrapping::Repeat;
        roughness.wrap_t = Wrapping::Repeat;
        roughness.mipmap_filter = Some(Filter::Linear);
        let roughness = r.upload_texture(&Arc::new(roughness))?;
        let bindings = [
            (env.view(), env.sampler()),
            (&roughness.view, &roughness.sampler),
        ];
        for projected in [false, true] {
            let direction = if projected {
                tsl::environment::parallax_correct(
                    tsl::environment::reflect_vector(),
                    position_world(),
                    vec3(float(200.0), float(200.0), float(100.0)),
                    vec3(float(0.0), float(-50.0), float(0.0)),
                )
            } else {
                tsl::environment::reflect_vector()
            };
            self.programs.push(Arc::new(
                SurfaceNodes {
                    environment: Some(tsl::environment::pmrem(
                        tsl::Texture::External(0),
                        direction,
                        environment_roughness(),
                        float(env.max_mip()),
                    )),
                    roughness: Some(
                        tsl::Texture::External(1)
                            .sample(vec2(uv().x() * float(2.0), float(1.0) - uv().y()))
                            .x()
                            * uniform(0, Type::Float),
                    ),
                    ..Default::default()
                }
                .build(r, &[], &bindings)
                .await?,
            ));
        }
        let mut m = standard(0xffffff, 0.25, 1.0);
        m.properties.vertex_program = Some(self.programs[1].clone());
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(200.0, 100.0, 100, 1)?),
            Material::Standard(m),
        );
        s.get_mut(h)?.position.y = -49.0;
        s.get_mut(h)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        self.objects.push(h);
        s.environment = Some(Arc::new(environment));
        self.params[..2].copy_from_slice(&[1.0, 0.25]);
        Ok(())
    }
}
