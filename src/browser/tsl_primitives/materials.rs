use super::*;
use crate::tsl::Node;
fn rgb(hex: u32) -> Node {
    let c = Color::from_hex(hex);
    vec3(
        float(c.0.x as f32),
        float(c.0.y as f32),
        float(c.0.z as f32),
    )
}
fn sample(i: usize, p: Node) -> Node {
    tsl::Texture::External(i).sample(vec2(p.x(), float(1.) - p.y()))
}
fn attribute(g: &mut BufferGeometry, name: &str, data: Vec<f32>, size: usize) -> Result<()> {
    g.set_attribute(
        name,
        Attribute::F32(BufferAttribute::new(data, size, false)?),
    );
    Ok(())
}
impl Demo {
    pub(super) async fn materials(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        #[derive(serde::Deserialize)]
        struct AttributeData {
            size: usize,
            array: Vec<f32>,
        }
        #[derive(serde::Deserialize)]
        struct Data {
            index: Vec<u32>,
            attributes: std::collections::HashMap<String, AttributeData>,
        }
        let data: Data =
            serde_json::from_slice(&fetch("/web/gallery/assets/teapot-50-18.json").await?)
                .map_err(|e| Error::Asset(e.to_string()))?;
        let mut g = BufferGeometry::default();
        for (name, a) in data.attributes {
            attribute(&mut g, &name, a.array, a.size)?;
        }
        g.set_index(Some(data.index));
        let g = Arc::new(g);
        let uv_texture = texture(r, "uv_grid_opengl.jpg").await?;
        let alpha = texture(r, "alphaMap.jpg").await?;
        let tex = sample(0, uv());
        let luminance = tex
            .rgb()
            .dot(vec3(float(0.299), float(0.587), float(0.114)));
        let p = position_local() * float(0.01);
        let n = normal_local().abs();
        let n = n.clone() / (n.x() + n.y() + n.swizzle("z"));
        let tri = sample(0, p.swizzle("yz")) * n.x()
            + sample(0, p.swizzle("zx")) * n.y()
            + sample(0, p.swizzle("xy")) * n.swizzle("z");
        let wgsl_luminance = WgslFn::new(
            "material_luminance",
            "fn luminance_helper(c:vec3<f32>)->f32{return dot(c,vec3(0.299,0.587,0.114));} fn material_luminance(c:vec3<f32>)->f32{return luminance_helper(c);}",
            &[Type::Vec3],
            Type::Float,
        )?;
        let wgsl_texture = WgslFn::new(
            "material_texture",
            "fn material_texture(t:texture_2d<f32>,s:sampler,p:vec2<f32>)->vec4<f32>{return textureSample(t,s,vec2(p.x,1.0-p.y))*vec4(0.0,1.0,0.0,1.0);}",
            &[Type::Texture, Type::Sampler, Type::Vec2],
            Type::Vec4,
        )?;
        let nodes = vec![
            position_local(),
            position_world(),
            normal_local(),
            normal_world(),
            normal_view_geometry(),
            tex.clone(),
            vec4(rgb(0x0099ff), tex.x()),
            alpha_test(vec4(tex.rgb(), sample(1, uv()).x()), float(0.5)),
            projection_position(),
            vec4(
                tsl::srgb_to_linear(normal_view_geometry() * float(0.5) + float(0.5)),
                float(0.5),
            ),
            luminance.clone(),
            luminance.clone(),
            wgsl_luminance.call(&[tex.rgb()]),
            wgsl_texture.call(&[
                tsl::Texture::External(0).node(),
                tsl::Texture::External(0).sampler(),
                uv(),
            ]),
            tri,
            sample(
                0,
                vec2(
                    (screen_coordinate() / screen_size()).x(),
                    float(1.) - (screen_coordinate() / screen_size()).y(),
                ),
            ),
            // r186's Loop return is discarded: its generated DiffuseColor is zero.
            // Preserve the pinned output; this is not evidence of a general Loop API.
            splat(float(0.0), Type::Vec3),
        ];
        let mut seed = 186;
        for (i, node) in nodes.into_iter().enumerate() {
            let mut m = NodeMaterial::new(node.max(float(0.)))
                .build(
                    r,
                    &[
                        (&uv_texture.view, &uv_texture.sampler),
                        (&alpha.view, &alpha.sampler),
                    ],
                )
                .await?;
            m.properties.transparent = [6, 9].contains(&i);
            let h = mesh(s, g.clone(), Material::Shader(m));
            s.get_mut(h)?.position = Vector3::new(
                (i % 4) as f64 * 200. - 400.,
                0.,
                (i / 4) as f64 * 200. - 200.,
            );
            self.objects.push(h);
            self.rotations.push(Vector3::new(
                random(&mut seed) * 200. - 100.,
                random(&mut seed) * 200. - 100.,
                random(&mut seed) * 200. - 100.,
            ));
        }
        let mut positions = vec![];
        for i in 0..=40 {
            let k = i as f32 * 25. - 500.;
            positions.extend_from_slice(&[-500., 0., k, 500., 0., k, k, 0., -500., k, 0., 500.]);
        }
        let mut g = BufferGeometry::default();
        attribute(&mut g, "position", positions, 3)?;
        let mut m = LineBasicMaterial::default();
        m.properties.color = Color::from_hex(0x303030);
        let h = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Line(m)),
            segments: true,
        }));
        s.get_mut(h)?.position.y = -75.;
        Ok(())
    }
    pub(super) async fn sandbox(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0x222222);
        let uv_texture = texture(r, "uv_grid_opengl.jpg").await?;
        let displacement = texture(r, "transition1.png").await?;
        let p = uv() + uniform(0, Type::Float) * vec2(float(-0.5), float(0.1));
        let color = (sample(0, p.clone()) + checker(p)) * float(0.5);
        let m = NodeMaterial::new(color)
            .build(r, &[(&uv_texture.view, &uv_texture.sampler)])
            .await?;
        let h = mesh(
            s,
            Arc::new(BoxGeometry::build(1., 1., 1.)?),
            Material::Shader(m),
        );
        s.get_mut(h)?.position.y = 1.;
        self.objects.push(h);
        self.rotations.push(Vector3::ZERO);
        let displace = sample(0, uv()).x() * float(0.25);
        let mut graph = NodeMaterial::new(displace.clone());
        graph.position = Some(
            position_geometry()
                + normal_geometry()
                    * tsl::Texture::External(0)
                        .sample_level(vec2(uv().x(), float(1.) - uv().y()), float(0.))
                        .x()
                    * float(0.25),
        );
        let m = graph
            .build(r, &[(&displacement.view, &displacement.sampler)])
            .await?;
        let h = mesh(
            s,
            Arc::new(SphereGeometry::build(0.5, 64, 64)?),
            Material::Shader(m),
        );
        s.get_mut(h)?.position = Vector3::new(-2., -1., 0.);
        let data = crate::material::Texture::from_rgba(
            512,
            512,
            [255, 0, 0, 255].repeat(512 * 512),
            false,
        )?;
        let data = r.upload_texture(&Arc::new(data))?;
        let mut m = NodeMaterial::new(
            tsl::Texture::External(0).sample(uv()) + vec4(rgb(0x0000ff), float(0.)),
        )
        .build(r, &[(&data.view, &data.sampler)])
        .await?;
        m.properties.transparent = true;
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
            Material::Shader(m),
        );
        s.get_mut(h)?.position = Vector3::new(0., -1., 0.);
        let t = crate::material::Texture::from_basis_compressed(
            fetch(&format!("{ASSETS}/2d_uastc.ktx2")).await?,
            true,
        )?;
        let t = r.upload_texture(&Arc::new(t))?;
        let osc = (((uniform(0, Type::Float) + float(0.75)) * float(std::f32::consts::TAU)).sin()
            + float(1.))
            * float(0.5);
        let c = sample(0, uv());
        let emissive = rgb(0x663300) * (float(1.) - osc.clone()) + rgb(0x0000ff) * osc.clone();
        let c = alpha_test(c, osc) + vec4(emissive, float(0.));
        let mut m = NodeMaterial::new(c)
            .build(r, &[(&t.view, &t.sampler)])
            .await?;
        m.properties.transparent = true;
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
            Material::Shader(m),
        );
        s.get_mut(h)?.position = Vector3::new(-2., 1., 0.);
        self.objects.push(h);
        let mut seed = 186;
        let mut points = vec![];
        for _ in 0..3000 {
            points.push((random(&mut seed) - 0.5) as f32);
        }
        let mut g = BufferGeometry::default();
        attribute(&mut g, "position", points, 3)?;
        let graph = NodeMaterial::new((position_local() * float(3.)).max(float(0.)));
        let sh = graph.build(r, &[]).await?;

        let h = s.insert(NodeKind::Points(Points {
            geometry: Arc::new(g),
            material: Arc::new(Material::Shader(sh)),
        }));
        s.get_mut(h)?.position = Vector3::new(2., -1., 0.);
        let positions = vec![
            -0.5, -0.5, 0., 0.5, -0.5, 0., 0.5, 0.5, 0., -0.5, 0.5, 0., -0.5, -0.5, 0.,
        ];
        let mut g = BufferGeometry::default();
        attribute(&mut g, "position", positions.clone(), 3)?;
        attribute(&mut g, "color", positions, 3)?;
        let mut m = LineBasicMaterial::default();
        m.properties.vertex_colors = true;
        let h = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Line(m)),
            segments: false,
        }));
        s.get_mut(h)?.position = Vector3::new(2., 1., 0.);
        Ok(())
    }
}
