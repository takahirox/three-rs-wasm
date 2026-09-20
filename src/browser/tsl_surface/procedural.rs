use super::*;
use crate::tsl::*;
impl Demo {
    pub(super) async fn shadertoy(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.0,
            right: 1.0,
            top: 1.0,
            bottom: -1.0,
            near: 0.0,
            far: 1.0,
            ..Default::default()
        }));
        scene.get_mut(cam)?.position = Vector3::ZERO;
        scene.get_mut(cam)?.quaternion = Quaternion::IDENTITY;
        let shader = WgslFn::new(
            "toy",
            include_str!("shadertoy.wgsl"),
            &[Type::Vec2, Type::Vec2, Type::Float],
            Type::Vec4,
        )?;
        let color = shader.call(&[
            vec2(
                screen_coordinate().x(),
                screen_size().y() - screen_coordinate().y(),
            ) / screen_size(),
            screen_size(),
            uniform(0, Type::Float),
        ]);
        // BasicNodeMaterial is opaque; the shader's alpha affects mixing but not blending.
        let material = NodeMaterial::new(color.rgb()).build(r, &[]).await?;
        let object = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
            Arc::new(Material::Shader(material)),
        )));
        scene.get_mut(object)?.frustum_culled = false;
        Ok(Self {
            example: 65,
            time: 0.0,
            lights: vec![],
            objects: vec![object],
            params: [[0.0; 4]; 16],
            viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
}
impl Demo {
    pub(super) async fn flames(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 25.0,
            aspect,
            near: 0.1,
            far: 100.0,
            ..Default::default()
        }));
        let position = Vector3::new(1.0, 1.0, 3.0);
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, Vector3::ZERO)?;
        scene.background = Color::from_hex(0x201919);
        let mut images = Vec::new();
        for name in ["flames-grayscale-256x256.png", "flames-rgb-256x256.png"] {
            let mut image = super::super::gltf_viewer::decode_image(
                &super::super::gltf_viewer::fetch(&format!("/web/gallery/assets/{name}")).await?,
            )
            .await?;
            image.srgb = false;
            image.mipmap_filter = Some(Filter::Linear);
            images.push(r.upload_texture(&Arc::new(image))?);
        }
        // The official 128 x 1 CanvasTexture: the browser generates it once at initialization.
        use wasm_bindgen::JsCast;
        let canvas = web_sys::OffscreenCanvas::new(128, 1).unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
            .unwrap();
        let gradient = context.create_linear_gradient(0.0, 0.0, 128.0, 0.0);
        for (i, c) in ["#090033", "#5f1f93", "#e02e96", "#ffbd80", "#fff0db"]
            .iter()
            .enumerate()
        {
            gradient.add_color_stop(i as f32 / 4.0, c).unwrap();
        }
        context.set_fill_style_canvas_gradient(&gradient);
        context.fill_rect(0.0, 0.0, 128.0, 1.0);
        let image = crate::material::Texture::from_rgba(
            128,
            1,
            context
                .get_image_data(0.0, 0.0, 128.0, 1.0)
                .unwrap()
                .data()
                .to_vec(),
            true,
        )?;
        images.push(r.upload_texture(&Arc::new(image))?);
        let function = WgslFn::new(
            "flame",
            include_str!("flames.wgsl"),
            &[
                Type::Vec2,
                Type::Float,
                Type::Float,
                Type::Texture,
                Type::Sampler,
                Type::Texture,
                Type::Sampler,
                Type::Texture,
                Type::Sampler,
            ],
            Type::Vec4,
        )?;
        let mut objects = Vec::new();
        for kind in 0..2 {
            let mut args = vec![tsl::uv(), uniform(0, Type::Float), float(kind as f32)];
            for i in 0..3 {
                args.extend([
                    tsl::Texture::External(i).node(),
                    tsl::Texture::External(i).sampler(),
                ]);
            }
            let mut graph = tsl::sprites::SpriteNodeMaterial::new(function.call(&args));
            graph.horizontal_rotation = true;
            let mut material = graph
                .build(
                    r,
                    &[],
                    &images
                        .iter()
                        .map(|i| (&i.view, &i.sampler))
                        .collect::<Vec<_>>(),
                )
                .await?;
            material.properties.side = Side::Double;
            let h = scene.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1)?),
                Arc::new(Material::Shader(material)),
            )));
            scene.get_mut(h)?.position.x = if kind == 0 { -0.5 } else { 0.5 };
            if kind == 0 {
                scene.get_mut(h)?.scale.x = 0.5;
            }
            scene.get_mut(h)?.frustum_culled = false;
            objects.push(h);
        }
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
        viewer.fixture(
            position.x.atan2(position.z),
            (position.y / position.length()).asin(),
            1.8,
        );
        Ok(Self {
            example: 66,
            time: 0.0,
            lights: vec![],
            objects,
            params: [[0.0; 4]; 16],
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
}
