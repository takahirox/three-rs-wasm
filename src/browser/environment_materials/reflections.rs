use super::*;
impl Demo {
    pub(super) async fn reflections(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let a = cube(r, if self.id == 178 { "pisa" } else { "Bridge2" }).await?;
        let b = if self.id == 179 {
            let t = image("textures/2294472375_24a3b8ef46_o.jpg", true).await?;
            let size = t.height;
            let t = r.upload_texture(&Arc::new(t))?;
            Some(
                GpuTexture::from_equirectangular_with_format(
                    r,
                    &t.view,
                    size,
                    wgpu::TextureFormat::Rgba8UnormSrgb,
                )
                .await?,
            )
        } else {
            None
        };
        let mut bindings = vec![(&a.view, &a.sampler, Type::TextureCube)];
        if let Some(b) = &b {
            bindings.push((&b.view, &b.sampler, Type::TextureCube));
        }
        self.params = if self.id == 178 {
            [16777215., 0., 0.98, 0., 1., 0., 0., 0.]
        } else {
            [0.; 8]
        };
        let reflected = call(
            "simple_reflection",
            "fn simple_reflection(p:vec3<f32>,n:vec3<f32>,ratio:f32,refracting:f32)->vec3<f32>{let v=normalize(p-u.camera.xyz);return normalize(select(reflect(v,n),refract(v,n,ratio),refracting>0.5));}",
            &[Type::Vec3, Type::Vec3, Type::Float, Type::Float],
            Type::Vec3,
            &[
                position_world(),
                normal_world(),
                uniform(0, Type::Vec4).swizzle("z"),
                uniform(0, Type::Vec4).y(),
            ],
        )?;
        for sky in [false, true] {
            let dir = if sky {
                position_local()
            } else {
                reflected.clone()
            };
            let dir = call(
                "rotate_environment",
                "fn rotate_environment(d:vec3<f32>)->vec3<f32>{return mat3x3(u.custom[4].xyz,u.custom[5].xyz,u.custom[6].xyz)*d;}",
                &[Type::Vec3],
                Type::Vec3,
                &[dir],
            )?;
            let source = if self.id == 178 {
                "fn environment_color(d:vec3<f32>)->vec3<f32>{return textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb;}"
            } else {
                "fn environment_color(d:vec3<f32>)->vec3<f32>{if u.custom[0].x>0.5{return textureSample(tsl_texture_1,tsl_sampler_1,d).rgb;}return textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb;}"
            };
            let source = if sky {
                source
                    .replace("textureSample(", "textureSampleLevel(")
                    .replace("vec3(-d.x,d.yz))", "vec3(-d.x,d.yz),0.0)")
                    .replace("tsl_sampler_1,d)", "tsl_sampler_1,d,0.0)")
            } else {
                source.to_owned()
            };
            let color = call(
                "environment_color",
                &source,
                &[Type::Vec3],
                Type::Vec3,
                &[dir],
            )?;
            let graph = NodeMaterial::new(vec4(
                if sky {
                    color
                } else {
                    color * uniform(1, Type::Vec3)
                },
                if sky {
                    float(1.)
                } else {
                    uniform(0, Type::Vec4).swizzle("w")
                },
            ));
            let mut m = graph.build_with_texture_types(r, &bindings).await?;
            if sky {
                // The camera-centered box fills the viewport without writing depth.
                m.properties.side = Side::Back;
                m.properties.depth_write = false;
                let h = mesh(
                    s,
                    Arc::new(BoxGeometry::build(5., 5., 5.)?),
                    Material::Shader(m),
                );
                let n = s.get_mut(h)?;
                n.frustum_culled = false;
                n.render_order = -10000;
                self.sky = Some(h);
            } else {
                let g = Arc::new(if self.id == 178 {
                    SphereGeometry::build(0.1, 32, 16)?
                } else {
                    IcosahedronGeometry::build(1., 15)?
                });
                let m = Arc::new(Material::Shader(m));
                let mut seed = 186;
                for _ in 0..if self.id == 178 { 500 } else { 1 } {
                    let h = s.insert(NodeKind::Mesh(Mesh::new(g.clone(), m.clone())));
                    if self.id == 178 {
                        let n = s.get_mut(h)?;
                        n.position = Vector3::new(
                            random(&mut seed) * 10. - 5.,
                            random(&mut seed) * 10. - 5.,
                            random(&mut seed) * 10. - 5.,
                        );
                        n.scale = Vector3::splat(random(&mut seed) * 3. + 1.);
                    }
                    self.objects.push(h);
                }
            }
        }
        Ok(())
    }
    pub(super) fn update_reflections(&mut self, s: &mut Scene, c: Object3D, dt: f64) -> Result<()> {
        if self.id == 179 {
            self.rotation += Vector3::new(
                self.params[2] as f64,
                self.params[3] as f64,
                self.params[4] as f64,
            ) * dt
                * 0.06;
            if self.params[5] > 0.5 {
                self.material_rotation = self.rotation;
            }
        }
        let material_changed = self.dirty || self.id == 179;
        if material_changed {
            let first = self.objects[0];
            let mut material = match &s.get(first)?.kind {
                NodeKind::Mesh(m) => m.materials[0].as_ref().clone(),
                _ => return Err(Error::Invalid("reflection mesh")),
            };
            if let Material::Shader(m) = &mut material {
                let rgb = Color::from_hex(if self.id == 178 {
                    self.params[0] as u32
                } else {
                    0xffffff
                })
                .0;
                m.uniforms[1] = [rgb.x as f32, rgb.y as f32, rgb.z as f32, 1.];
                m.uniforms[0] = if self.id == 178 {
                    [
                        0.,
                        self.params[1],
                        self.params[2],
                        if self.params[3] > 0.5 {
                            self.params[4]
                        } else {
                            1.
                        },
                    ]
                } else {
                    [self.params[0], self.params[1], 0.98, 1.]
                };
                m.properties.transparent = self.id == 178 && self.params[3] > 0.5;
                set_rotation(m, self.material_rotation);
            }
            let m = Arc::new(material);
            for &h in &self.objects {
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(h)?.kind {
                    mesh.materials[0] = m.clone();
                }
            }
        }
        if self.id == 178 {
            for (i, &h) in self.objects.iter().enumerate() {
                let n = s.get_mut(h)?;
                n.position.x = 5. * (self.time * 0.1 + i as f64).cos();
                n.position.y = 5. * (self.time * 0.1 + i as f64 * 1.1).sin();
            }
        }
        let position = s.get(c)?.position;
        if let Some(h) = self.sky {
            let n = s.get_mut(h)?;
            n.position = position;
            if let NodeKind::Mesh(mesh) = &mut n.kind
                && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
            {
                m.uniforms[0][0] = if self.id == 179 { self.params[0] } else { 0. };
                set_rotation(m, self.rotation);
            }
        }
        Ok(())
    }
}
fn set_rotation(m: &mut ShaderMaterial, rotation: Vector3) {
    let q = Quaternion::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
    let matrix = Matrix3::from_quat(q).transpose().to_cols_array();
    for i in 0..3 {
        m.uniforms[i + 4] = [
            matrix[i * 3] as f32,
            matrix[i * 3 + 1] as f32,
            matrix[i * 3 + 2] as f32,
            0.,
        ];
    }
}
