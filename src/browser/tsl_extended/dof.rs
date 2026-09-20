use super::*;
pub(super) struct DofPass {
    input: RenderTarget,
    effect: tsl::dof::DepthOfField,
    copy: Effect,
}
impl Demo {
    pub(super) async fn dof(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        self.params[..3].copy_from_slice(&[500.0, 200.0, 10.0]);
        let cube = super::cube::castle(r).await?;
        // oscSine(t) = (sin((t+.75)*2pi)+1)/2, applied independently to XYZ.
        let oscillation = (((position_world() / float(1000.0)
            + uniform(0, Type::Float) * float(0.2)
            + float(0.75))
            * float(std::f32::consts::TAU))
        .sin()
            + float(1.0))
            * float(0.5);
        let material = NodeMaterial::new(
            tsl::sampling::cube(tsl::Texture::External(0), super::cube::reflection()?).rgb()
                * oscillation,
        )
        .build_with_texture_types(r, &[(&cube.view, &cube.sampler, Type::TextureCube)])
        .await?;
        let h = mesh(
            s,
            Arc::new(SphereGeometry::build(60.0, 20, 10)?),
            Material::Shader(material),
        );
        for i in 0..14 {
            for j in 0..9 {
                for k in 0..14 {
                    s.get_mut(h)?.instances.push(Instance {
                        matrix: Matrix4::from_translation(Vector3::new(
                            200.0 * (i as f64 - 7.0),
                            200.0 * (j as f64 - 4.5),
                            200.0 * (k as f64 - 7.0),
                        )),
                        color: Color::WHITE,
                    });
                }
            }
        }
        self.objects.push(h);
        let input = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let effect = tsl::dof::DepthOfField::new(
            r,
            &input
                .depth_texture()
                .unwrap()
                .create_view(&Default::default()),
        )
        .await?;
        let copy = tsl::effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &tsl::Texture::Input.sample(uv()),
        )
        .await?;
        self.dof = Some(DofPass {
            input,
            effect,
            copy,
        });
        Ok(())
    }
}
impl DofPass {
    pub(super) fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        target: &RenderTarget,
        params: &[f32; 16],
    ) -> Result<()> {
        if (self.input.width, self.input.height) != (target.width, target.height) {
            self.input
                .set_size(&r.device, target.width, target.height)?;
            self.effect.set_depth(
                r,
                &self
                    .input
                    .depth_texture()
                    .unwrap()
                    .create_view(&Default::default()),
            )?;
        }
        r.render(s, c, &self.input)?;
        self.effect.focus_distance = params[0];
        self.effect.focal_length = params[1];
        self.effect.bokeh_scale = params[2];
        let Camera::Perspective(camera) = s.camera(c)?.0 else {
            return Err(Error::Invalid("DOF camera"));
        };
        let output = self
            .effect
            .render(r, &self.input, camera.near as f32, camera.far as f32)?;
        self.copy.apply(r, output, None, target)
    }
}
