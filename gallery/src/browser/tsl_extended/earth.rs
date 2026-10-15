use super::*;
use crate::tsl::surface::{SurfaceNodes, bump_map};
impl Demo {
    pub(super) async fn earth(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let position = Vector3::new(4.5, 2.0, 3.0);
        s.get_mut(c)?.position = position;
        s.look_at(c, Vector3::ZERO)?;
        self.viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
        self.viewer.fixture(
            position.x.atan2(position.z),
            (position.y / position.length()).asin(),
            1.8,
        );
        self.params[..4].copy_from_slice(&[0x4db2ff as f32, 0xbc490b as f32, 0.25, 0.35]);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 2.0,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position.z = 3.0;
        let mut textures = Vec::new();
        for (name, srgb) in [
            ("earth_day_4096.jpg", true),
            ("earth_night_4096.jpg", true),
            ("earth_bump_roughness_clouds_4096.jpg", false),
        ] {
            let mut image =
                decode_image(&fetch(&format!("/web/gallery/assets/{name}")).await?).await?;
            image.srgb = srgb;
            image.anisotropy = 8;
            image.mipmap_filter = Some(Filter::Linear);
            textures.push(r.upload_texture(&Arc::new(image))?);
        }
        let bindings = textures
            .iter()
            .map(|t| (&t.view, &t.sampler))
            .collect::<Vec<_>>();
        let tex = |i, coordinate: tsl::Node| {
            tsl::Texture::External(i).sample(vec2(coordinate.x(), float(1.0) - coordinate.y()))
        };
        let clouds = tex(2, uv()).swizzle("z").smoothstep(float(0.2), float(1.0));
        // r186 caches cloudsStrength before the bump context is evaluated; only
        // its raw height sample receives the forward-difference UV offset.
        let elevation = |coordinate: tsl::Node| tex(2, coordinate).x().max(clouds.clone());
        let orientation = normal_world().swizzle("z");
        let camera = WgslFn::new(
            "earth_view",
            "fn earth_view(p:vec3<f32>)->vec3<f32>{return normalize(p-u.camera.xyz);}",
            &[Type::Vec3],
            Type::Vec3,
        )?;
        let fresnel = float(1.0) - camera.call(&[position_world()]).dot(normal_world()).abs();
        let atmosphere = mix(
            uniform(2, Type::Vec3),
            uniform(1, Type::Vec3),
            orientation.smoothstep(float(-0.25), float(0.75)),
        );
        let final_color = mix(
            mix(
                tex(1, uv()).rgb(),
                output().rgb(),
                orientation.smoothstep(float(-0.25), float(0.5)),
            ),
            atmosphere.clone(),
            (orientation.smoothstep(float(-0.5), float(1.0)) * fresnel.pow(float(2.0)))
                .clamp(float(0.0), float(1.0)),
        );
        let graph = SurfaceNodes {
            color: Some(mix(
                tex(0, uv()).rgb(),
                splat(float(1.0), Type::Vec3),
                clouds.clone() * float(2.0),
            )),
            roughness: Some(mix(
                uniform(3, Type::Vec2).x(),
                uniform(3, Type::Vec2).y(),
                tex(2, uv())
                    .y()
                    .max(clouds.less_than(float(0.01)).select(float(0.0), float(1.0))),
            )),
            normal: Some(bump_map(elevation, uv(), float(1.0))),
            output: Some(vec4(final_color, output().swizzle("w"))),
            ..Default::default()
        };
        let mut material = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        material.properties.vertex_program = Some(Arc::new(graph.build(r, &[], &bindings).await?));
        let geometry = Arc::new(SphereGeometry::build(1.0, 64, 64)?);
        self.objects
            .push(mesh(s, geometry.clone(), Material::Standard(material)));
        let alpha = ((float(1.0) - fresnel) / (float(1.0) - float(0.73))).pow(float(3.0))
            * orientation.smoothstep(float(-0.5), float(1.0));
        let mut atmosphere_material = NodeMaterial::new(vec4(atmosphere, alpha))
            .build(r, &[])
            .await?;
        atmosphere_material.properties.side = Side::Back;
        atmosphere_material.properties.transparent = true;
        let h = mesh(s, geometry, Material::Shader(atmosphere_material));
        s.get_mut(h)?.scale = Vector3::splat(1.04);
        self.objects.push(h);
        Ok(())
    }
    pub(super) fn update_earth(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        self.viewer.update(s, c)?;
        s.get_mut(self.objects[0])?.quaternion = Quaternion::from_rotation_y(self.time * 0.025);
        for h in &self.objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                let m = Arc::make_mut(&mut m.materials[0]);
                let uniforms = match m {
                    Material::Shader(m) => &mut m.uniforms,
                    Material::Standard(m) => &mut m.properties.vertex_uniforms,
                    _ => continue,
                };
                uniforms[1] = Color::from_hex(self.params[0] as u32)
                    .0
                    .extend(1.0)
                    .as_vec4()
                    .to_array();
                uniforms[2] = Color::from_hex(self.params[1] as u32)
                    .0
                    .extend(1.0)
                    .as_vec4()
                    .to_array();
                uniforms[3] = [self.params[2], self.params[3], 0.0, 0.0];
            }
        }
        Ok(())
    }
}
