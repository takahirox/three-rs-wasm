use super::*;
pub(super) struct Lines {
    wide: Object3D,
    variants: Vec<Arc<crate::shader::ShaderProgram>>,
    thin: Object3D,
    target: RenderTarget,
    camera: Object3D,
    background: Object3D,
    wire: bool,
    pub(super) thin_scale: f32,
}
impl Demo {
    pub(super) async fn lines(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        #[derive(serde::Deserialize)]
        struct Data {
            positions: Vec<[f64; 3]>,
            colors: Vec<[f64; 3]>,
            wire: Vec<f32>,
        }
        let data: Data = serde_json::from_slice(&fetch(&format!("{ASSETS}/lines.json")).await?)
            .map_err(|e| Error::Asset(e.to_string()))?;
        let wire = self.example == 127;
        let mut segments = vec![];
        let mut distance = 0.;
        let mut positions = vec![];
        let mut colors = vec![];
        let mut distances = vec![];
        let count = if wire {
            data.wire.len() / 6
        } else {
            data.positions.len() - 1
        };
        let color = Color::from_hex(0x4080ff).0;
        for i in 0..count {
            let (a, b, ca, cb) = if wire {
                (
                    Vector3::new(
                        data.wire[i * 6] as f64,
                        data.wire[i * 6 + 1] as f64,
                        data.wire[i * 6 + 2] as f64,
                    ),
                    Vector3::new(
                        data.wire[i * 6 + 3] as f64,
                        data.wire[i * 6 + 4] as f64,
                        data.wire[i * 6 + 5] as f64,
                    ),
                    color,
                    color,
                )
            } else {
                (
                    Vector3::from_array(data.positions[i]),
                    Vector3::from_array(data.positions[i + 1]),
                    Vector3::from_array(data.colors[i]),
                    Vector3::from_array(data.colors[i + 1]),
                )
            };
            let a = a.as_vec3().as_dvec3();
            let b = b.as_vec3().as_dvec3();
            let end = (distance + (b - a).length()) as f32 as f64;
            segments.push([
                [a.x as f32, a.y as f32, a.z as f32, distance as f32],
                [b.x as f32, b.y as f32, b.z as f32, end as f32],
                [ca.x as f32, ca.y as f32, ca.z as f32, 1.],
                [cb.x as f32, cb.y as f32, cb.z as f32, 1.],
            ]);
            if wire || i == 0 {
                positions.extend(a.to_array().map(|v| v as f32));
                colors.extend(ca.to_array().map(|v| v as f32));
                distances.push(distance as f32);
            }
            positions.extend(b.to_array().map(|v| v as f32));
            colors.extend(cb.to_array().map(|v| v as f32));
            distances.push(end as f32);
            distance = end;
        }
        let data = tsl::lines::LineSegments::new(r, &segments)?;
        let mut variants = vec![];
        for mask in 0..8 {
            variants.push(
                data.material(
                    r,
                    tsl::lines::LineOptions {
                        world_units: mask & 1 != 0,
                        dashed: mask & 2 != 0,
                        alpha_to_coverage: mask & 4 != 0,
                    },
                )
                .await?
                .program,
            );
        }
        let wide = mesh(
            s,
            Arc::new(tsl::lines::geometry(count as u32)?),
            Material::Shader(ShaderMaterial::new(variants[0].clone())),
        );
        s.get_mut(wide)?.frustum_culled = false;
        let mut g = BufferGeometry::default();
        for (name, data, size) in [
            ("position", positions, 3),
            ("color", colors, 3),
            ("lineDistance", distances, 1),
        ] {
            g.set_attribute(
                name,
                Attribute::F32(BufferAttribute::new(data, size, false)?),
            );
        }
        let mut m = LineBasicMaterial::default();
        m.properties.vertex_colors = true;
        let thin = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Line(m)),
            segments: wire,
        }));
        s.get_mut(thin)?.visible = false;
        let background = mesh(
            s,
            Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
            Material::Shader(
                tsl::surface::background_material(
                    r,
                    vec3(
                        float(Color::from_hex(0x222222).0.x as f32),
                        float(Color::from_hex(0x222222).0.y as f32),
                        float(Color::from_hex(0x222222).0.z as f32),
                    ),
                )
                .await?,
            ),
        );
        s.get_mut(background)?.frustum_culled = false;
        s.get_mut(background)?.render_order = -100;
        s.get_mut(background)?.visible = false;
        let camera = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            aspect: 1.,
            near: 1.,
            far: 1000.,
            ..Default::default()
        })));
        let target = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples: 4,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        self.params = if wire {
            [0., 5., 0., 1., -1., 0., 0., 0.]
        } else {
            [0., 0., 5., 0., 0., 1., 0., -1.]
        };
        self.lines = Some(Lines {
            variants,
            wide,
            thin,
            target,
            camera,
            background,
            wire,
            thin_scale: 2.,
        });
        Ok(())
    }
}
impl Lines {
    pub fn output(&self) -> &RenderTarget {
        &self.target
    }
    pub fn update(&self, s: &mut Scene, p: &[f32; 8]) -> Result<()> {
        let (width, world, coverage, dashed, scale, offset, ratio) = if self.wire {
            (p[1], 0., 1., p[2], p[3], 0., p[4])
        } else {
            (p[2], p[1], p[3], p[4], p[5], p[6], p[7])
        };
        let dash = if ratio < 0. {
            3.
        } else if ratio < 0.5 {
            2.
        } else {
            1.
        };
        let gap = if ratio > 1.5 { 2. } else { 1. };
        s.get_mut(self.wide)?.visible = p[0] < 0.5;
        s.get_mut(self.thin)?.visible = p[0] >= 0.5;
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.wide)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.program = self.variants[usize::from(world > 0.5)
                | (usize::from(dashed > 0.5) * 2)
                | (usize::from(coverage > 0.5) * 4)]
                .clone();
            m.uniforms[0] = [
                width,
                world,
                dashed,
                web_sys::window().map_or(1., |w| w.device_pixel_ratio() as f32),
            ];
            m.uniforms[1] = [scale, offset, dash, gap];
            m.uniforms[2][0] = coverage;
            m.properties.alpha_to_coverage = coverage > 0.5;
        }
        if let NodeKind::Line(l) = &mut s.get_mut(self.thin)?.kind
            && let Material::Line(m) = Arc::make_mut(&mut l.material)
        {
            m.dash = if dashed > 0.5 {
                Some(LineDash {
                    scale: self.thin_scale as f64,
                    size: if ratio < 0. { 1. } else { dash as f64 },
                    gap: gap as f64,
                    offset: offset as f64,
                })
            } else {
                None
            };
            m.properties.vertex_colors = !(self.wire && dashed > 0.5);
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<()> {
        self.target.set_size(&r.device, out.width, out.height)?;
        self.target.options.load_color = false;
        self.target.viewport = [0, 0, out.width, out.height];
        self.target.scissor = None;
        s.get_mut(self.background)?.visible = false;
        r.render(s, c, &self.target)?;
        let pos = s.get(c)?.position;
        let q = s.get(c)?.quaternion;
        s.get_mut(self.camera)?.position = pos;
        s.get_mut(self.camera)?.quaternion = q;
        let inset = (20. * web_sys::window().map_or(1., |w| w.device_pixel_ratio())) as u32;
        let size = (out.height / 4)
            .min(out.width.saturating_sub(inset))
            .min(out.height.saturating_sub(inset));
        if size > 0 {
            self.target.options.load_color = true;
            self.target.viewport = [inset, out.height - size - inset, size, size];
            self.target.scissor = Some(self.target.viewport);
            s.get_mut(self.background)?.visible = true;
            r.render(s, self.camera, &self.target)?;
            s.get_mut(self.background)?.visible = false;
        }
        Ok(())
    }
}
