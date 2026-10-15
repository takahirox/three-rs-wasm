use super::*;
pub(super) struct Pair {
    scenes: Vec<Scene>,
    cameras: Vec<Object3D>,
    pub pointer: Vector2,
    anisotropy: bool,
    pub output: RenderTarget,
}
impl Pair {
    pub async fn new(r: &Renderer, anisotropy: bool, filters: bool) -> Result<Self> {
        let painting = if anisotropy {
            None
        } else {
            Some(image("758px-Canestra_di_frutta_(Caravaggio).jpg").await?)
        };
        let mut scenes = Vec::new();
        let mut cameras = Vec::new();
        let geometry = Arc::new(PlaneGeometry::build(100., 100., 1, 1)?);
        for i in 0..2 {
            let mut s = Scene::default();
            let c = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                fov: 35.,
                near: 1.,
                far: if anisotropy { 25000. } else { 5000. },
                ..Default::default()
            })));
            s.get_mut(c)?.position.z = 1500.;
            s.fog = Some(Fog::Linear {
                color: if anisotropy {
                    Color::from_hex(0xf2f7ff)
                } else {
                    Color::BLACK
                },
                near: if anisotropy { 1. } else { 1500. },
                far: if anisotropy { 25000. } else { 4000. },
            });
            let floor = if anisotropy {
                s.insert(NodeKind::Light(Light::Ambient {
                    color: Color::from_hex(0xeef0ff),
                    intensity: 3.,
                }));
                let l = s.insert(NodeKind::Light(Light::Directional {
                    color: Color::WHITE,
                    intensity: 6.,
                    target: Vector3::ZERO,
                }));
                s.get_mut(l)?.position = Vector3::ONE;
                let mut t = image("crate.gif").await?;
                t.mipmap_filter = Some(Filter::Linear);
                t.anisotropy = if i == 0 { 16 } else { 1 };
                t.wrap_s = Wrapping::Repeat;
                t.wrap_t = Wrapping::Repeat;
                t.repeat = Vector2::splat(512.);
                let mut m = MeshPhongMaterial::default();
                m.properties.map = Some(Arc::new(t));
                mesh(&mut s, geometry.clone(), Material::Phong(m))
            } else if filters {
                let mut rgba = Vec::with_capacity(128 * 128 * 4);
                for y in 0..128 {
                    for x in 0..128 {
                        let v = if (x < 64) == (y < 64) { 255 } else { 68 };
                        rgba.extend([v, v, v, 255]);
                    }
                }
                let mut t = Texture::from_rgba(128, 128, rgba, true)?;
                t.wrap_s = Wrapping::Repeat;
                t.wrap_t = Wrapping::Repeat;
                t.repeat = Vector2::splat(1000.);
                t.filter = if i == 0 {
                    Filter::Linear
                } else {
                    Filter::Nearest
                };
                t.mipmap_filter = if i == 0 { Some(Filter::Linear) } else { None };
                let mut m = MeshBasicMaterial::default();
                m.properties.color = Color::from_hex(if i == 0 { 0xffffff } else { 0xffccaa });
                m.properties.map = Some(Arc::new(t));
                mesh(&mut s, geometry.clone(), Material::Basic(m))
            } else {
                let bytes = fetch(&format!("{ASSETS}/manual-mips.rgba")).await?;
                let mut offset = 0;
                let mut levels = Vec::new();
                for level in 0..8 {
                    let size = 128 >> level;
                    let len = (size * size * 4) as usize;
                    let mut t =
                        Texture::from_rgba(size, size, bytes[offset..offset + len].to_vec(), true)?;
                    offset += len;
                    t.wrap_s = Wrapping::Repeat;
                    t.wrap_t = Wrapping::Repeat;
                    t.filter = if i == 0 {
                        Filter::Linear
                    } else {
                        Filter::Nearest
                    };
                    t.mipmap_filter = Some(t.filter);
                    levels.push(t);
                }
                let gpu = GpuTexture::from_rgba_mipmaps(r, &levels)?;
                let mut m = MeshBasicMaterial::default();
                m.properties.color = Color::from_hex(if i == 0 { 0xffffff } else { 0xffccaa });
                m.properties.vertex_program = Some(Arc::new(
                    SurfaceNodes {
                        color: Some(
                            tsl::Texture::External(0)
                                .sample(vec2(uv().x(), float(1.) - uv().y()) * float(1000.))
                                .rgb()
                                * vec3(
                                    float(m.properties.color.0.x as f32),
                                    float(m.properties.color.0.y as f32),
                                    float(m.properties.color.0.z as f32),
                                ),
                        ),
                        ..Default::default()
                    }
                    .build(r, &[], &[(&gpu.view, &gpu.sampler)])
                    .await?,
                ));
                mesh(&mut s, geometry.clone(), Material::Basic(m))
            };
            {
                let n = s.get_mut(floor)?;
                n.scale = Vector3::splat(1000.);
                n.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
            }
            if let Some(t) = &painting {
                let w = t.width as f64;
                let h = t.height as f64;
                let mut t = t.clone();
                t.filter = if i == 0 {
                    Filter::Linear
                } else {
                    Filter::Nearest
                };
                // r186 WebGPU samples generated mips for LinearFilter, but
                // emits a level-zero textureLoad for NearestFilter.
                t.mipmap_filter = if i == 0 && !filters {
                    Some(Filter::Nearest)
                } else {
                    None
                };
                let mut m = MeshBasicMaterial::default();
                m.properties.map = Some(Arc::new(t));
                m.properties.color = Color::from_hex(if i == 0 { 0xffffff } else { 0xffccaa });
                let p = mesh(&mut s, geometry.clone(), Material::Basic(m));
                s.get_mut(p)?.scale = Vector3::new(w / 100., h / 100., 1.);
                let mut black = MeshBasicMaterial::default();
                black.properties.color = Color::BLACK;
                let f = mesh(&mut s, geometry.clone(), Material::Basic(black.clone()));
                s.get_mut(f)?.position.z = -10.;
                s.get_mut(f)?.scale = Vector3::new(1.1 * w / 100., 1.1 * h / 100., 1.);
                black.properties.transparent = true;
                black.properties.opacity = 0.75;
                let sh = mesh(&mut s, geometry.clone(), Material::Basic(black));
                let n = s.get_mut(sh)?;
                n.position = Vector3::new(0., -1.1 * h / 2., -1.1 * h / 2.);
                n.scale = Vector3::new(1.1 * w / 100., 1.1 * h / 100., 1.);
                n.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
                s.get_mut(floor)?.position.y = -1.117 * h / 2.;
            }
            if filters {
                s.fog = None;
                let output=WgslFn::new("filter_output","fn filter_output(c:vec4<f32>)->vec4<f32>{return vec4(srgb_output(c.rgb)*(1.0-smoothstep(1500.0,4000.0,fragment_view_z)),c.a);}",&[Type::Vec4],Type::Vec4)?.call(&[tsl::output()]);
                let program = Arc::new(
                    SurfaceNodes {
                        output: Some(output),
                        ..Default::default()
                    }
                    .build(r, &[], &[])
                    .await?,
                );
                for h in s.handles().collect::<Vec<_>>() {
                    if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                        Arc::make_mut(&mut m.materials[0])
                            .properties_mut()
                            .vertex_program = Some(program.clone());
                    }
                }
            }
            scenes.push(s);
            cameras.push(c);
        }
        Ok(Self {
            scenes,
            cameras,
            pointer: Vector2::ZERO,
            anisotropy,
            output: RenderTarget::with_options(
                &r.device,
                1,
                1,
                RenderTargetOptions {
                    samples: 4,
                    format: if filters {
                        wgpu::TextureFormat::Rgba8Unorm
                    } else {
                        wgpu::TextureFormat::Rgba16Float
                    },
                    ..Default::default()
                },
            )?,
        })
    }
    pub fn update(&mut self) -> Result<()> {
        for (s, &c) in self.scenes.iter_mut().zip(&self.cameras) {
            let n = s.get_mut(c)?;
            n.position.x += (self.pointer.x - n.position.x) * 0.05;
            n.position.y += (200. + self.pointer.y - n.position.y) * 0.05;
            if self.anisotropy {
                n.position.y = n.position.y.clamp(50., 1000.);
            }
            s.look_at(c, Vector3::ZERO)?;
        }
        Ok(())
    }
    pub fn render(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        self.output.set_size(&r.device, out.width, out.height)?;
        let dpr = web_sys::window()
            .map(|w| w.device_pixel_ratio())
            .unwrap_or(1.);
        for (i, (s, &c)) in self.scenes.iter_mut().zip(&self.cameras).enumerate() {
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
                p.aspect = out.width as f64 / out.height as f64;
            }
            self.output.scissor = Some([
                if i == 0 { 0 } else { out.width / 2 },
                0,
                ((out.width as f64 / 2. - 2. * dpr).max(1.)) as u32,
                out.height,
            ]);
            self.output.set_load_color(i != 0);
            r.render(s, c, &self.output)?;
        }
        Ok(())
    }
}
