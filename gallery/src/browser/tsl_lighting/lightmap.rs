use super::*;
use crate::attribute::BufferAttribute;
impl Demo {
    pub(super) async fn lightmap(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let root = format!("{ASSETS}/models/json/lightmap");
        let json: serde_json::Value =
            serde_json::from_slice(&fetch(&format!("{root}/lightmap.json")).await?)
                .map_err(|e| Error::Asset(e.to_string()))?;
        let data = &json["geometries"][0]["data"];
        let mut g = BufferGeometry::default();
        for (name, a) in data["attributes"]
            .as_object()
            .ok_or(Error::Invalid("lightmap geometry"))?
        {
            let values = a["array"]
                .as_array()
                .ok_or(Error::Invalid("lightmap attribute"))?
                .iter()
                .map(|x| x.as_f64().unwrap_or(0.0) as f32)
                .collect();
            g.set_attribute(
                name,
                Attribute::F32(BufferAttribute::new(
                    values,
                    a["itemSize"].as_u64().unwrap_or(3) as usize,
                    false,
                )?),
            );
        }
        for group in data["groups"]
            .as_array()
            .ok_or(Error::Invalid("lightmap groups"))?
        {
            g.add_group(
                group["start"].as_u64().unwrap() as usize,
                group["count"].as_u64().unwrap() as usize,
                group["materialIndex"].as_u64().unwrap() as usize,
            );
        }
        let mut textures = std::collections::HashMap::new();
        for entry in json["textures"].as_array().unwrap() {
            let image = json["images"]
                .as_array()
                .unwrap()
                .iter()
                .find(|i| i["uuid"] == entry["image"])
                .unwrap();
            let mut tex = decode_texture_image(
                &fetch(&format!("{root}/{}", image["url"].as_str().unwrap())).await?,
            )
            .await?;
            tex.srgb = false;
            tex.mipmap_filter = Some(Filter::Linear);
            tex.anisotropy = 4;
            if entry["wrap"][0] == 1000 {
                tex.wrap_s = Wrapping::Repeat;
                tex.wrap_t = Wrapping::Repeat;
            }
            textures.insert(
                entry["uuid"].as_str().unwrap().to_owned(),
                (
                    r.upload_texture(&Arc::new(tex))?,
                    entry["repeat"][0].as_f64().unwrap() as f32,
                ),
            );
        }
        let mut materials = vec![];
        for entry in json["materials"].as_array().unwrap() {
            let light = &textures[entry["lightMap"].as_str().unwrap()].0;
            let (bump, repeat) = &textures[entry["bumpMap"].as_str().unwrap()];
            let height = |p: tsl::Node| {
                tsl::Texture::External(1)
                    .sample(vec2(p.x(), float(1.0) - p.y()))
                    .x()
            };
            let program = SurfaceNodes {
                light_map: Some(
                    tsl::Texture::External(0)
                        .sample(vec2(uv1().x(), float(1.0) - uv1().y()))
                        .rgb()
                        * uniform(0, Type::Float),
                ),
                normal: Some(bump_map(height, uv() * float(*repeat), float(2.0))),
                ..Default::default()
            }
            .build(
                r,
                &[],
                &[(&light.view, &light.sampler), (&bump.view, &bump.sampler)],
            )
            .await?;
            let mut m = MeshPhongMaterial {
                specular: Color::from_hex(0x494949),
                shininess: 100.0,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(entry["color"].as_u64().unwrap() as u32);
            m.properties.vertex_program = Some(Arc::new(program));
            m.properties.vertex_uniforms[0][0] = 2.5;
            // Specular map is the same grayscale texture, with the original UV repeat.
            let image = json["textures"]
                .as_array()
                .unwrap()
                .iter()
                .find(|t| t["uuid"] == entry["bumpMap"])
                .unwrap();
            let im = json["images"]
                .as_array()
                .unwrap()
                .iter()
                .find(|i| i["uuid"] == image["image"])
                .unwrap();
            let mut tex = decode_texture_image(
                &fetch(&format!("{root}/{}", im["url"].as_str().unwrap())).await?,
            )
            .await?;
            tex.srgb = false;
            tex.mipmap_filter = Some(Filter::Linear);
            tex.anisotropy = 4;
            tex.wrap_s = Wrapping::Repeat;
            tex.wrap_t = Wrapping::Repeat;
            tex.repeat = Vector2::splat(*repeat as f64);
            m.specular_map = Some(Arc::new(tex));
            materials.push(Arc::new(Material::Phong(m)));
        }
        let mut m = Mesh::new(Arc::new(g), materials[0].clone());
        m.materials = materials;
        let h = s.insert(NodeKind::Mesh(m));
        s.get_mut(h)?.scale = Vector3::splat(100.0);
        self.objects.push(h);
        self.params[0] = 2.5;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0xd5deff),
            intensity: 1.0,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(300.0, 250.0, -500.0);
        let top = Color::from_hex(0xd5deff).0;
        let height = (position_local() + float(400.0))
            .normalize()
            .y()
            .max(float(0.0))
            .pow(float(0.6));
        let mut sky = NodeMaterial::new(mix(
            splat(float(1.0), Type::Vec3),
            vec3(
                float(top.x as f32),
                float(top.y as f32),
                float(top.z as f32),
            ),
            height,
        ))
        .build(r, &[])
        .await?;
        sky.properties.side = Side::Back;
        mesh(
            s,
            Arc::new(SphereGeometry::build(4000.0, 32, 15)?),
            Material::Shader(sky),
        );
        Ok(())
    }
}
