use super::*;
use crate::tsl::environment::pmrem;
async fn hdr(r: &Renderer, path: &str) -> Result<EnvironmentMap> {
    let mut e = EnvironmentMap::from_hdr(&fetch(path).await?)?;
    e.prefilter(r)?;
    Ok(e)
}
async fn cube(r: &Renderer, hdr: bool) -> Result<EnvironmentMap> {
    let mut pixels = vec![];
    let mut size = 0;
    for face in ["px", "nx", "py", "ny", "pz", "nz"] {
        let path = if hdr {
            format!("{ASSETS}/textures/cube/pisaHDR/{face}.hdr")
        } else {
            format!("{ASSETS}/textures/cube/MilkyWay/dark-s_{face}.jpg")
        };
        let bytes = fetch(&path).await?;
        let (width, height, rgba) = if hdr {
            let im = image::load_from_memory(&bytes).map_err(|e| Error::Asset(e.to_string()))?;
            (
                im.width(),
                im.height(),
                im.to_rgba32f()
                    .as_raw()
                    .iter()
                    .map(|v| half::f16::from_f32(v.clamp(0.0, 65504.0)))
                    .collect::<Vec<_>>(),
            )
        } else {
            let im = super::super::gltf_viewer::decode_texture_image(&bytes).await?;
            (
                im.width,
                im.height,
                im.rgba
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        half::f16::from_f64(if i % 4 == 3 {
                            *v as f64 / 255.0
                        } else {
                            crate::math::srgb_to_linear(*v as f64 / 255.0)
                        })
                    })
                    .collect(),
            )
        };
        if size == 0 {
            size = width;
        }
        if width != size || height != size {
            return Err(Error::Invalid("cube face dimensions"));
        }
        pixels.extend(rgba);
    }
    EnvironmentMap::from_cube_hdr(r, size, &pixels)
}
fn rotate(direction: tsl::Node, angle: tsl::Node) -> tsl::Node {
    // Row-vector * Matrix4.makeRotationY, matching the original TSL mul order.
    vec3(
        direction.x() * angle.cos() - direction.swizzle("z") * angle.sin(),
        direction.y(),
        direction.x() * angle.sin() + direction.swizzle("z") * angle.cos(),
    )
}
impl Demo {
    pub(super) async fn maps(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let envs = if self.example == 103 {
            vec![cube(r, false).await?, cube(r, true).await?]
        } else {
            vec![
                hdr(
                    r,
                    &format!("{ASSETS}/textures/equirectangular/pedestrian_overpass_1k.hdr"),
                )
                .await?,
                hdr(
                    r,
                    &format!("{NEXT_ASSETS}/textures/equirectangular/752-hdri-skies-com_1k.hdr"),
                )
                .await?,
            ]
        };
        let maps = envs
            .iter()
            .map(|e| e.gpu.as_ref().unwrap())
            .collect::<Vec<_>>();
        let bindings = maps
            .iter()
            .map(|e| (e.view(), e.sampler()))
            .collect::<Vec<_>>();
        let graph =
            |direction: tsl::Node, position: tsl::Node, roughness: tsl::Node, normal: tsl::Node| {
                let a = uniform(0, Type::Vec4);
                let b = uniform(1, Type::Vec4);
                if self.example == 103 {
                    mix(
                        pmrem(
                            tsl::Texture::External(0),
                            direction.clone(),
                            roughness.clone(),
                            float(maps[0].max_mip()),
                        ),
                        pmrem(
                            tsl::Texture::External(1),
                            direction,
                            roughness,
                            float(maps[1].max_mip()),
                        ),
                        a.x(),
                    )
                } else {
                    let c = mix(
                        pmrem(
                            tsl::Texture::External(0),
                            rotate(direction.clone(), a.swizzle("z")),
                            roughness.clone(),
                            float(maps[0].max_mip()),
                        ),
                        pmrem(
                            tsl::Texture::External(1),
                            rotate(direction, a.swizzle("w")),
                            roughness,
                            float(maps[1].max_mip()),
                        ),
                        (position.y() + a.x()).clamp(float(0.0), float(1.0)),
                    );
                    saturation(
                        hue(mix(c, normal, b.x()) * b.y(), b.swizzle("z")),
                        b.swizzle("w"),
                    )
                }
            };
        let program = Arc::new(
            SurfaceNodes {
                environment: Some(graph(
                    if self.example == 104 {
                        tsl::environment::reflect_vector()
                    } else {
                        environment_direction()
                    },
                    position_world(),
                    environment_roughness(),
                    tsl::environment::material_normal_world(),
                )),
                ..Default::default()
            }
            .build(r, &[], &bindings)
            .await?,
        );
        let (a, b, images) = super::super::gltf_viewer::load_asset(
            "/web/models/DamagedHelmet/glTF/DamagedHelmet.gltf",
        )
        .await?;
        crate::gltf::import_decoded(&a, &b, &images)?.instantiate(s)?;
        for h in s.handles().collect::<Vec<_>>() {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for mat in &mut m.materials {
                    Arc::make_mut(mat).properties_mut().vertex_program = Some(program.clone());
                }
                self.objects.push(h);
            }
        }
        if self.example == 104 {
            for (x, roughness) in [(-2.0, 1.0), (2.0, 0.0)] {
                let mut m = standard(0xffffff, roughness, 1.0);
                m.properties.vertex_program = Some(program.clone());
                let h = mesh(
                    s,
                    Arc::new(SphereGeometry::build(0.5, 64, 32)?),
                    Material::Standard(m),
                );
                s.get_mut(h)?.position.x = x;
                self.objects.push(h);
            }
            self.params = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0];
        }
        let color = graph(
            normal_world(),
            position_local(),
            uniform(0, Type::Vec4).y(),
            -normal_world(),
        );
        let shader = NodeMaterial::new(vec4(color, float(1.0))).wgsl(bindings.len())?;
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip.z=out.clip.w;return out;}";
        let mut material = ShaderMaterial::new(Arc::new(
            crate::shader::ShaderProgram::with_projection(r, &shader, &[], &bindings, projection)
                .await?,
        ));
        material.properties.side = Side::Back;
        material.properties.depth_write = false;
        material.properties.depth_test = false;
        let h = mesh(
            s,
            Arc::new(SphereGeometry::build(1.0, 32, 32)?),
            Material::Shader(material),
        );
        s.get_mut(h)?.frustum_culled = false;
        s.get_mut(h)?.render_order = -10000;
        self.sky = Some(h);
        s.environment = Some(Arc::new(envs.into_iter().next().unwrap()));
        s.tone_mapping = ToneMapping::Linear;
        Ok(())
    }
}
