use super::*;
pub(super) struct Anamorphic {
    input: RenderTarget,
    bloom: tsl::bloom::Bloom,
    combine: Effect,
}
impl Demo {
    pub(super) async fn anamorphic(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Neutral;
        self.params[..6].copy_from_slice(&[5.0, 0.3, 80.0, 0x7a8aff as f32, 0.0, 0.5]);
        let coord = screen_coordinate() / screen_size();
        let background = mix(
            rgb(0x111111),
            rgb(0),
            (coord - vec2(float(0.5), float(0.5))).length() * float(2.0),
        );
        let source = NodeMaterial::new(background).wgsl(0)?;
        let mut material = ShaderMaterial::new(Arc::new(crate::shader::ShaderProgram::with_projection(r, &source, &[], &[], "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var o=surface;o.clip=vec4(position.xy*2.0,1.0,1.0);return o;}").await?));
        material.properties.depth_write = false;
        material.properties.depth_test = false;
        let bg = mesh(
            s,
            Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1)?),
            Material::Shader(material),
        );
        s.get_mut(bg)?.frustum_culled = false;
        s.get_mut(bg)?.render_order = -100;
        let mut seed = 186;
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        for _ in 0..200 {
            positions.push([
                (random(&mut seed) as f32 - 0.5) * 20.0,
                (random(&mut seed) as f32 - 0.5) * 20.0,
                (random(&mut seed) as f32 - 0.5) * 20.0,
                0.0,
            ]);
            colors.push(
                Color::from_hex((random(&mut seed) * 0xffffff as f64) as u32)
                    .0
                    .extend(1.0)
                    .as_vec4()
                    .to_array(),
            );
        }
        let buffers = [
            GpuBuffer::new(r, bytemuck::cast_slice(&positions), BufferAccess::Read)?,
            GpuBuffer::new(r, bytemuck::cast_slice(&colors), BufferAccess::Read)?,
        ];
        let graph = NodeMaterial {
            position: Some(
                position_geometry()
                    + instanced_attribute(0).rgb()
                    + vec3(
                        float(0.0),
                        ((uniform(0, Type::Float) + instance_index().to_float() * float(0.5))
                            * uniform(1, Type::Float))
                        .sin()
                            * float(5.0),
                        float(0.0),
                    ),
            ),
            color: instanced_attribute(1),
        };
        let material = graph
            .build_with_storage(
                r,
                &buffers.iter().map(|b| (b, Type::Vec4)).collect::<Vec<_>>(),
                &[],
            )
            .await?;
        let mut geometry = SphereGeometry::build(0.1, 32, 32)?;
        geometry.instance_count = Some(200);
        let h = mesh(s, Arc::new(geometry), Material::Shader(material));
        s.get_mut(h)?.frustum_culled = false;
        self.objects.push(h);
        let input = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples: 4,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let mut bloom = tsl::bloom::Bloom::new(r).await?;
        bloom.resolution_scale = 0.25;
        let filter = Effect::new(
            r,
            wgpu::TextureFormat::Rgba16Float,
            r#"
fn effect(uv:vec2<f32>)->vec4<f32>{
    let samples=params[0].x;let halfSamples=samples*0.5;
    var total=vec4<f32>(0.0);
    for(var i = -halfSamples; i < halfSamples; i+=1.0){
        let softness=1.0-abs(i)/halfSamples;
        let shifted=vec2(uv.x+i*4.0/params[0].y,uv.y);
        let mirrored=1.0-abs(fract(shifted*0.5)*2.0-1.0);
        total+=textureSampleLevel(input_texture,input_sampler,mirrored,0.0)*softness*softness;
    }
    return total/(samples/3.0);
}"#,
        )
        .await?;
        bloom.set_high_pass_filter(r, filter)?;
        let combine = tsl::effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &(tsl::Texture::Input.sample(uv())
                + tsl::Texture::History.sample(uv()) * vec4(uniform(0, Type::Vec3), float(1.0))),
        )
        .await?;
        self.anamorphic = Some(Anamorphic {
            input,
            bloom,
            combine,
        });
        Ok(())
    }
}
impl Anamorphic {
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
        }
        r.render(s, c, &self.input)?;
        self.bloom.strength = params[0];
        self.bloom.threshold = params[1];
        self.bloom.radius = params[4];
        self.bloom.high_pass_filter_mut().unwrap().parameters[0] = [
            params[2],
            (target.width / 4) as f32,
            (target.height / 4) as f32,
            0.0,
        ];
        self.combine.parameters[0] = Color::from_hex(params[3] as u32)
            .0
            .extend(1.0)
            .as_vec4()
            .to_array();
        let output = self.bloom.render(r, &self.input)?;
        self.combine.apply(r, &self.input, Some(output), target)
    }
}
