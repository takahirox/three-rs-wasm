use super::*;
use wasm_bindgen::JsCast;
fn blend(i: usize) -> wgpu::BlendState {
    use wgpu::{BlendComponent as C, BlendFactor as F, BlendOperation as O};
    let component = |src_factor, dst_factor| C {
        src_factor,
        dst_factor,
        operation: O::Add,
    };
    match i {
        0 => wgpu::BlendState::REPLACE,
        1 => wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
        2 => wgpu::BlendState {
            color: component(F::One, F::One),
            alpha: component(F::One, F::One),
        },
        3 => wgpu::BlendState {
            color: component(F::Zero, F::OneMinusSrc),
            alpha: component(F::Zero, F::One),
        },
        _ => wgpu::BlendState {
            color: component(F::Dst, F::OneMinusSrcAlpha),
            alpha: component(F::Zero, F::One),
        },
    }
}
async fn material(r: &Renderer, t: Texture, premultiplied: bool) -> Result<ShaderMaterial> {
    let t = r.upload_texture(&Arc::new(t))?;
    let tex = tsl::Texture::External(0).sample(vec2(uv().x(), float(1.) - uv().y()));
    let rgb = call(
        "blend_encode",
        "fn blend_encode(c:vec3<f32>)->vec3<f32>{return srgb_output(c);}",
        &[Type::Vec3],
        Type::Vec3,
        &[tex.clone().rgb()],
    )?;
    NodeMaterial::new(vec4(
        if premultiplied {
            rgb * tex.clone().swizzle("w")
        } else {
            rgb
        },
        tex.swizzle("w"),
    ))
    .build(r, &[(&t.view, &t.sampler)])
    .await
}
impl Demo {
    pub(super) async fn blending(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let fail = |e: wasm_bindgen::JsValue| Error::Asset(format!("label canvas: {e:?}"));
        let mut labels = vec![];
        for name in ["No", "Normal", "Additive", "Subtractive", "Multiply"] {
            let canvas = web_sys::OffscreenCanvas::new(128, 32).map_err(fail)?;
            let context = canvas
                .get_context("2d")
                .map_err(fail)?
                .ok_or(Error::Invalid("label context"))?
                .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
                .map_err(|e| fail(e.into()))?;
            context.set_fill_style_str("rgba(0,0,0,0.95)");
            context.fill_rect(0., 0., 128., 32.);
            context.set_fill_style_str("white");
            context.set_font("bold 12pt arial");
            context.fill_text(name, 10., 22.).map_err(fail)?;
            let mut t = Texture::from_rgba(
                128,
                32,
                context
                    .get_image_data(0., 0., 128., 32.)
                    .map_err(fail)?
                    .data()
                    .to_vec(),
                true,
            )?;
            t.mipmap_filter = Some(Filter::Linear);
            let mut m = material(r, t, false).await?;
            m.properties.transparent = true;
            labels.push(Arc::new(Material::Shader(m)));
        }
        let g = Arc::new(PlaneGeometry::build(100., 100., 1, 1)?);
        let labelg = Arc::new(PlaneGeometry::build(100., 25., 1, 1)?);
        for (row, path) in [
            "textures/uv_grid_opengl.jpg",
            "textures/sprite0.jpg",
            "textures/sprite0.png",
            "textures/lensflare/lensflare0.png",
            "textures/lensflare/lensflare0_alpha.png",
        ]
        .iter()
        .enumerate()
        {
            let base = material(r, image(path, true).await?, true).await?;
            let y = 300. - row as f64 * 150.;
            for (i, label) in labels.iter().enumerate() {
                let mut m = base.clone();
                m.properties.transparent = true;
                m.properties.blending = Some(blend(i));
                let x = (i as f64 - 2.5) * 110.;
                let h = mesh(s, g.clone(), Material::Shader(m));
                s.get_mut(h)?.position = Vector3::new(x, y, 0.);
                let h = s.insert(NodeKind::Mesh(Mesh::new(labelg.clone(), label.clone())));
                s.get_mut(h)?.position = Vector3::new(x, y - 75., 0.);
            }
        }
        let mut rgba = vec![];
        for y in 0..128 {
            for x in 0..128 {
                let v = if x < 64 && y < 64 {
                    if x >= 32 && y >= 32 { 0x99 } else { 0x55 }
                } else if x >= 64 && y >= 64 {
                    if x >= 96 && y >= 96 { 0x77 } else { 0x55 }
                } else {
                    0xdd
                };
                rgba.extend([v, v, v, 255]);
            }
        }
        let mut t = Texture::from_rgba(128, 128, rgba, true)?;
        t.wrap_s = Wrapping::Repeat;
        t.wrap_t = Wrapping::Repeat;
        t.mipmap_filter = Some(Filter::Linear);
        let t = r.upload_texture(&Arc::new(t))?;
        let coordinate = vec2(
            uv().x() * float(64.) - uniform(0, Type::Float) * float(0.16),
            (float(1.) - uv().y()) * float(32.) + uniform(0, Type::Float) * float(0.08),
        );
        let color = call(
            "blend_background",
            "fn blend_background(c:vec4<f32>)->vec4<f32>{return vec4(srgb_output(c.rgb),c.a);}",
            &[Type::Vec4],
            Type::Vec4,
            &[tsl::Texture::External(0).sample(coordinate)],
        )?;
        let graph = NodeMaterial::new(color);
        let mut m=ShaderMaterial::new(Arc::new(ShaderProgram::with_projection(r,&graph.wgsl(1)?,&[],&[(&t.view,&t.sampler)],"fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.xy*2.0,1.0,1.0);return out;}").await?));
        m.properties.depth_write = false;
        m.properties.depth_test = false;
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
            Material::Shader(m),
        );
        s.get_mut(h)?.frustum_culled = false;
        s.get_mut(h)?.render_order = -10000;
        self.sky = Some(h);
        Ok(())
    }
    pub(super) fn update_blending(&mut self, s: &mut Scene) -> Result<()> {
        if let Some(h) = self.sky
            && let NodeKind::Mesh(mesh) = &mut s.get_mut(h)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
        {
            m.uniforms[0][0] = self.time as f32;
        }
        Ok(())
    }
}
