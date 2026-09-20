use super::*;
use crate::attribute::BufferAttribute;
use wasm_bindgen::JsCast;
struct CanvasTarget {
    surface: wgpu::Surface<'static>,
    format: wgpu::TextureFormat,
}
pub(super) struct Elements {
    canvases: Vec<CanvasTarget>,
    scenes: Vec<(Scene, Object3D, Object3D, OrbitViewer)>,
    rectangles: Vec<[f64; 4]>,
    target: RenderTarget,
    active: usize,
}
impl Elements {
    pub(super) async fn new(r: &Renderer) -> Result<Self> {
        #[derive(serde::Deserialize)]
        struct GeometryData {
            position: Vec<f32>,
            normal: Vec<f32>,
            uv: Vec<f32>,
        }
        let data: GeometryData =
            serde_json::from_slice(&fetch("/web/gallery/assets/dodecahedron.json").await?)
                .map_err(|e| Error::Asset(e.to_string()))?;
        let mut dodeca = BufferGeometry::default();
        for (name, array, size) in [
            ("position", data.position, 3),
            ("normal", data.normal, 3),
            ("uv", data.uv, 2),
        ] {
            dodeca.set_attribute(
                name,
                Attribute::F32(BufferAttribute::new(array, size, false)?),
            );
        }
        let geometries = [
            Arc::new(BoxGeometry::build(1.0, 1.0, 1.0)?),
            Arc::new(SphereGeometry::build(0.5, 12, 8)?),
            Arc::new(dodeca),
            Arc::new(CylinderGeometry::build(
                0.5,
                0.5,
                1.0,
                12,
                1,
                false,
                0.0,
                std::f64::consts::TAU,
            )?),
        ];
        let bg = tsl::surface::background_material(r, rgb(0xeeeeee)).await?;
        let mut seed = 186;
        let mut scenes = Vec::new();
        for _ in 0..40 {
            let mut s = Scene::new();
            let c = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                fov: 50.0,
                aspect: 1.0,
                near: 1.0,
                far: 10.0,
                ..Default::default()
            })));
            s.get_mut(c)?.position.z = 2.0;
            let choice = (random(&mut seed) * 4.0) as usize;
            let hue = random(&mut seed);
            let channel = |n: f64| {
                let k = (n + hue * 12.0) % 12.0;
                let v = 0.75 - 0.25 * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0);
                if v <= 0.04045 {
                    v / 12.92
                } else {
                    ((v + 0.055) / 1.055).powf(2.4)
                }
            };
            let mut m = MeshStandardMaterial {
                energy_conservation: true,
                roughness: 0.5,
                ..Default::default()
            };
            m.properties.color = Color(Vector3::new(channel(0.0), channel(8.0), channel(4.0)));
            m.properties.flat_shading = true;
            let object = mesh(&mut s, geometries[choice].clone(), Material::Standard(m));
            let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
                sky: Color::from_hex(0xaaaaaa),
                ground: Color::from_hex(0x444444),
                intensity: 3.0,
            }));
            s.get_mut(hemi)?.position.y = 1.0;
            let light = s.insert(NodeKind::Light(Light::Directional {
                color: Color::WHITE,
                intensity: 1.5,
                target: Vector3::ZERO,
            }));
            s.get_mut(light)?.position = Vector3::ONE;
            let background = mesh(
                &mut s,
                Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
                Material::Shader(bg.clone()),
            );
            s.get_mut(background)?.frustum_culled = false;
            s.get_mut(background)?.render_order = -100;
            scenes.push((s, c, object, OrbitViewer::from_camera(Vector3::ZERO, 2.0)));
        }
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
        Ok(Self {
            canvases: Vec::new(),
            scenes,
            rectangles: vec![[0.0; 4]; 40],
            target,
            active: 0,
        })
    }
    pub(super) fn attach_canvases(&mut self, r: &Renderer, canvases: js_sys::Array) -> Result<()> {
        if canvases.length() != 40 {
            return Err(Error::Invalid("canvas target count"));
        }
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });
        let mut targets = Vec::new();
        let mut size = 200;
        for canvas in canvases.iter() {
            let canvas = canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| Error::Invalid("canvas target"))?;
            size = canvas.width().max(1);
            let surface = instance
                .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
                .map_err(|e| Error::Gpu(e.to_string()))?;
            let mut config = surface
                .get_default_config(&r.adapter, size, size)
                .ok_or(Error::Invalid("canvas configuration"))?;
            config.view_formats = vec![config.format.add_srgb_suffix()];
            surface.configure(&r.device, &config);
            targets.push(CanvasTarget {
                surface,
                format: config.format.add_srgb_suffix(),
            });
        }
        self.target.set_size(&r.device, size, size)?;
        self.canvases = targets;
        Ok(())
    }
    fn render_canvases(&mut self, r: &Renderer) -> Result<()> {
        self.target.options.load_color = false;
        self.target.viewport = [0, 0, self.target.width, self.target.height];
        self.target.scissor = None;
        for ((scene, camera, _, viewer), canvas) in self.scenes.iter_mut().zip(&self.canvases) {
            viewer.update(scene, *camera)?;
            if let NodeKind::Camera(Camera::Perspective(camera)) = &mut scene.get_mut(*camera)?.kind
            {
                camera.near = 1.0;
                camera.far = 10.0;
            }
            r.render(scene, *camera, &self.target)?;
            let frame = canvas
                .surface
                .get_current_texture()
                .map_err(|e| Error::Gpu(e.to_string()))?;
            r.blit_with_tone_mapping(
                &self.target,
                &frame.texture.create_view(&wgpu::TextureViewDescriptor {
                    format: Some(canvas.format),
                    ..Default::default()
                }),
                canvas.format,
                1.0,
                ToneMapping::None,
            );
            frame.present();
        }
        Ok(())
    }
    pub(super) fn viewport(&mut self, index: usize, rectangle: [f64; 4]) -> Result<()> {
        if index >= 40
            || !rectangle.iter().all(|v| v.is_finite())
            || rectangle[2] < 0.0
            || rectangle[3] < 0.0
        {
            return Err(Error::Invalid("element viewport"));
        }
        self.rectangles[index] = rectangle;
        Ok(())
    }
    pub(super) fn select(&mut self, index: usize) -> Result<()> {
        if index >= 40 {
            return Err(Error::Invalid("element index"));
        }
        self.active = index;
        Ok(())
    }
    pub(super) fn input(&mut self, dx: f64, dy: f64) {
        self.scenes[self.active]
            .3
            .orbit_pixels(dx, dy, 0.0, 200.0, 2.0, 5.0);
    }
    pub(super) fn output(&self) -> &RenderTarget {
        &self.target
    }
    pub(super) fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        target: &RenderTarget,
        time: f64,
    ) -> Result<()> {
        if !self.canvases.is_empty() {
            return self.render_canvases(r);
        }
        if (self.target.width, self.target.height) != (target.width, target.height) {
            self.target
                .set_size(&r.device, target.width, target.height)?;
        }
        self.target.viewport = [0, 0, target.width, target.height];
        self.target.scissor = None;
        self.target.options.load_color = false;
        s.background = Color::WHITE;
        r.render(s, c, &self.target)?;
        self.target.options.load_color = true;
        for ((scene, camera, object, viewer), rect) in self.scenes.iter_mut().zip(&self.rectangles)
        {
            let [left, top, width, height] = *rect;
            let x = left.max(0.0);
            let y = top.max(0.0);
            let right = (left + width).min(target.width as f64);
            let bottom = (top + height).min(target.height as f64);
            if right <= x || bottom <= y {
                continue;
            }
            viewer.update(scene, *camera)?;
            if let NodeKind::Camera(Camera::Perspective(camera)) = &mut scene.get_mut(*camera)?.kind
            {
                camera.near = 1.0;
                camera.far = 10.0;
                camera.view = Some(ViewOffset {
                    full_width: width,
                    full_height: height,
                    offset_x: x - left,
                    offset_y: y - top,
                    width: right - x,
                    height: bottom - y,
                });
            }
            scene.get_mut(*object)?.quaternion = Quaternion::from_rotation_y(time);
            self.target.viewport = [x as u32, y as u32, (right - x) as u32, (bottom - y) as u32];
            self.target.scissor = Some(self.target.viewport);
            r.render(scene, *camera, &self.target)?;
        }
        Ok(())
    }
}
