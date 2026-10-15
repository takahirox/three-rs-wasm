use super::*;
use crate::tsl::*;
impl Demo {
    pub(super) async fn tornado(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 25.0,
            aspect,
            near: 0.1,
            far: 50.0,
            ..Default::default()
        }));
        let position = Vector3::new(1.0, 1.0, 3.0);
        let target = Vector3::new(0.0, 0.4, 0.0);
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, target)?;
        scene.background = Color::from_hex(0x201919);
        scene.tone_mapping = ToneMapping::Aces;
        let mut image = super::super::gltf_viewer::decode_image(
            &super::super::gltf_viewer::fetch("/web/gallery/assets/flames-rgb-256x256.png").await?,
        )
        .await?;
        image.srgb = false;
        image.wrap_s = Wrapping::Repeat;
        image.wrap_t = Wrapping::Repeat;
        image.mipmap_filter = Some(Filter::Linear);
        let image = r.upload_texture(&Arc::new(image))?;
        let color = WgslFn::new(
            "tornado_color",
            include_str!("tornado.wgsl"),
            &[
                Type::Vec2,
                Type::Float,
                Type::Float,
                Type::Vec3,
                Type::Float,
                Type::Texture,
                Type::Sampler,
            ],
            Type::Vec4,
        )?;
        let position_fn = WgslFn::new(
            "tornado_position",
            include_str!("tornado-position.wgsl"),
            &[Type::Vec3, Type::Vec4, Type::Float, Type::Float],
            Type::Vec3,
        )?;
        let mut objects = Vec::new();
        for kind in 0..3 {
            let mut graph = NodeMaterial::new(color.call(&[
                uv(),
                uniform(0, Type::Float),
                uniform(2, Type::Float),
                uniform(3, Type::Vec3),
                float(kind as f32),
                tsl::Texture::External(0).node(),
                tsl::Texture::External(0).sampler(),
            ]));
            let geometry = if kind == 0 {
                let mut g = PlaneGeometry::build(2.0, 2.0, 1, 1)?;
                g.rotate_x(-std::f64::consts::FRAC_PI_2)?;
                g
            } else {
                graph.position = Some(position_fn.call(&[
                    position_geometry(),
                    uniform(2, Type::Vec4),
                    uniform(0, Type::Float),
                    float(kind as f32),
                ]));
                let mut g = CylinderGeometry::build(
                    1.0,
                    1.0,
                    1.0,
                    20,
                    20,
                    true,
                    0.0,
                    std::f64::consts::TAU,
                )?;
                g.translate(Vector3::new(0.0, 0.5, 0.0))?;
                g
            };
            let mut m = graph.build(r, &[(&image.view, &image.sampler)]).await?;
            m.properties.transparent = true;
            m.properties.force_single_pass = false;
            if kind > 0 {
                m.properties.side = Side::Double;
            }
            let h = scene.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(geometry),
                Arc::new(Material::Shader(m)),
            )));
            scene.get_mut(h)?.frustum_culled = false;
            objects.push(h);
        }
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        let mut params = [[0.0; 4]; 16];
        params[1] = [1.0, 1.0, 0.1, 1.0];
        params[2] = [0.2, 1.0, 0.3, 0.2];
        params[3] = Color::from_hex(0xff8b4d).0.extend(0.0).as_vec4().to_array();
        Ok(Self {
            example: 76,
            time: 0.0,
            lights: vec![],
            objects,
            params,
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            storage: None,
            jelly: None,
            mask: None,
            bloom: Some(super::bloom::BloomPass::new(r, 4, false).await?),
        })
    }
}
