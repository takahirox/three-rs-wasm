//! Rust-owned browser application used by the web validation suite.
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, raycast::*, renderer::*, scene::*,
};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen::{JsCast, prelude::*};
mod gltf_viewer;
mod point_lights;

type AnimationCallback = Closure<dyn FnMut(f64)>;
struct State {
    timer: crate::time::Timer,
    renderer: Renderer,
    surface: wgpu::Surface<'static>,
    configuration: wgpu::SurfaceConfiguration,
    target: RenderTarget,
    scene: Scene,
    camera: Object3D,
    mesh: Object3D,
    canvas: web_sys::HtmlCanvasElement,
    frame: u32,
    animation: Option<AnimationCallback>,
    request: Option<i32>,
    paused: bool,
    rebuilds: u32,
    example: u32,
    rotation: Vector3,
    point_lights: Option<point_lights::PointLights>,
    gltf: Option<gltf_viewer::GltfViewer>,
}
impl State {
    fn render(&mut self, time: f64) -> Result<()> {
        self.timer.update();
        let _ = self
            .canvas
            .set_attribute("data-delta", &self.timer.get_delta().to_string());
        let width = self.canvas.width();
        let height = self.canvas.height();
        if width != self.target.width || height != self.target.height {
            self.target.set_size(&self.renderer.device, width, height)?;
            self.configuration.width = width;
            self.configuration.height = height;
            self.surface
                .configure(&self.renderer.device, &self.configuration);
        }
        if let Some(viewer) = &self.gltf {
            viewer.update(&mut self.scene, self.camera)?;
        } else if let Some(demo) = &mut self.point_lights {
            if !self.paused {
                demo.time += self.timer.get_delta().min(0.1) * demo.speed;
            }
            demo.update(&mut self.scene, self.camera, self.mesh)?;
            let _ = self
                .canvas
                .set_attribute("data-demo-time", &demo.time.to_string());
        } else if !self.paused {
            self.scene.get_mut(self.mesh)?.quaternion = if self.example == 2 {
                self.rotation.x += 0.005;
                self.rotation.y += 0.01;
                Euler {
                    angles: self.rotation,
                    order: EulerOrder::XYZ,
                }
                .quaternion()
            } else {
                Quaternion::from_rotation_y(time * 0.0003)
            };
        }
        self.renderer
            .render(&mut self.scene, self.camera, &self.target)?;
        let frame = self
            .surface
            .get_current_texture()
            .map_err(|e| Error::Gpu(e.to_string()))?;
        let format = self.configuration.format.add_srgb_suffix();
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(format),
            ..Default::default()
        });
        self.renderer.blit(&self.target, &view, format);
        frame.present();
        self.frame += 1;
        let _ = self
            .canvas
            .set_attribute("data-frames", &self.frame.to_string());
        Ok(())
    }
}

/// The JavaScript bootstrap owns one app lifetime. Scene state, rendering,
/// animation and input handling all stay in Rust.
#[wasm_bindgen]
pub struct BrowserApp {
    state: Rc<RefCell<State>>,
    pointer: Closure<dyn FnMut(web_sys::PointerEvent)>,
}
#[wasm_bindgen]
impl BrowserApp {
    /// Presentation controls; all scene and deformation state remains in Rust.
    pub fn point_lights_controls(&mut self, paused: bool, amount: f64, speed: f64) {
        let mut state = self.state.borrow_mut();
        state.paused = paused;
        if let Some(demo) = &mut state.point_lights {
            if amount.is_finite() {
                demo.amount = amount.clamp(0.0, 3.0);
            }
            if speed.is_finite() {
                demo.speed = speed.clamp(0.0, 2.0);
            }
        }
    }
    pub fn orbit(&mut self, dx: f64, dy: f64, zoom: f64) {
        if dx.is_finite()
            && dy.is_finite()
            && zoom.is_finite()
            && let Some(viewer) = &mut self.state.borrow_mut().gltf
        {
            viewer.orbit(dx, dy, zoom);
        }
        if dx.is_finite()
            && dy.is_finite()
            && zoom.is_finite()
            && let Some(demo) = &mut self.state.borrow_mut().point_lights
        {
            demo.orbit(dx, dy, zoom);
        }
    }
    /// Rebuild a small scene fragment, exercising owned geometry replacement,
    /// groups/draw ranges, attribute edits and stale-handle protection.
    pub fn rebuild(&mut self) -> std::result::Result<(), JsValue> {
        let result = (|| -> Result<()> {
            let mut state = self.state.borrow_mut();
            let original = state.mesh;
            let mut geometry = PlaneGeometry::build(2.0, 2.0, 1, 1)?;
            geometry.clear_groups();
            geometry.add_group(0, 3, 0);
            geometry.add_group(3, 3, 1);
            state.rebuilds += 1;
            geometry.set_draw_range(
                0,
                Some(if state.rebuilds.is_multiple_of(2) {
                    6
                } else {
                    3
                }),
            );
            if let Some(position) = geometry.attributes.get_mut("position") {
                position.set_component(0, 0, -0.8)?;
            }
            let mut first = Material::default();
            first.properties_mut().color = Color::from_hex(0xff3366);
            let mut second = Material::default();
            second.properties_mut().color = Color::from_hex(0x33ccff);
            let replacement = state.scene.insert(NodeKind::Mesh(Mesh {
                geometry: Arc::new(geometry),
                materials: vec![Arc::new(first), Arc::new(second)],
            }));
            let group = state.scene.insert(NodeKind::Group);
            state.scene.add(group, replacement)?;
            state.scene.remove_from_parent(replacement)?;
            state.scene.dispose(group)?;
            state.scene.dispose(original)?;
            if state.scene.get(original).is_ok() {
                return Err(Error::Invalid("stale handle retained"));
            }
            state.mesh = replacement;
            state.paused = true;
            state.target = RenderTarget::new(
                &state.renderer.device,
                state.canvas.width(),
                state.canvas.height(),
            )?;
            state
                .canvas
                .set_attribute("data-rebuilt", "true")
                .map_err(|e| Error::Asset(format!("{e:?}")))?;
            let _ = state
                .canvas
                .set_attribute("data-rebuilds", &state.rebuilds.to_string());
            Ok(())
        })();
        result.map_err(|e| JsValue::from_str(&e.to_string()))
    }
    #[wasm_bindgen(js_name = create)]
    pub async fn new(
        canvas: web_sys::HtmlCanvasElement,
        example: u32,
        animate: bool,
    ) -> std::result::Result<BrowserApp, JsValue> {
        async fn create(
            canvas: web_sys::HtmlCanvasElement,
            example: u32,
            animate: bool,
        ) -> Result<BrowserApp> {
            let renderer = Renderer::new().await?;
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::BROWSER_WEBGPU,
                ..Default::default()
            });
            let surface = instance
                .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
                .map_err(|e| Error::Gpu(e.to_string()))?;
            let mut configuration = surface
                .get_default_config(&renderer.adapter, canvas.width(), canvas.height())
                .ok_or(Error::Gpu("surface configuration unavailable".into()))?;
            configuration.view_formats = vec![configuration.format.add_srgb_suffix()];
            surface.configure(&renderer.device, &configuration);
            let target = RenderTarget::new(&renderer.device, canvas.width(), canvas.height())?;
            let mut scene = Scene::new();
            scene.background = Color::from_hex(0x102030);
            let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                aspect: canvas.width() as f64 / canvas.height() as f64,
                ..Default::default()
            })));
            scene.get_mut(camera)?.position.z = 5.0;
            let mut material = Material::default();
            material.properties_mut().color = Color::from_hex(0xee6633);
            let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(BoxGeometry::build(1.5, 1.5, 1.5)?),
                Arc::new(material),
            )));
            let mut point_lights = None;
            let mut gltf = None;
            if example == 4 || example == 5 {
                let viewer =
                    gltf_viewer::GltfViewer::create(&mut scene, camera, mesh, example).await?;
                let _ = canvas.set_attribute("data-triangles", &viewer.triangles.to_string());
                let _ = canvas.set_attribute("data-meshes", &viewer.meshes.to_string());
                gltf = Some(viewer);
            } else if example == 3 {
                let mut demo = point_lights::PointLights::create(
                    &mut scene,
                    camera,
                    mesh,
                    canvas.width() as f64 / canvas.height() as f64,
                )
                .await?;
                if !animate {
                    demo.time = 6.0;
                }
                point_lights = Some(demo);
            } else if example == 1 {
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                        left: -3.0,
                        right: 3.0,
                        top: 3.0,
                        bottom: -3.0,
                        ..Default::default()
                    }));
                scene.get_mut(mesh)?.position.x = -1.1;
                let group = scene.insert(NodeKind::Group);
                scene.get_mut(group)?.position = Vector3::new(0.8, 0.6, 0.0);
                let texture = Arc::new(Texture::load("/web/checker.png").await?);
                let mut standard = MeshStandardMaterial::default();
                standard.properties.map = Some(texture);
                standard.roughness = 0.7;
                let sphere = scene.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(SphereGeometry::build(0.75, 24, 16)?),
                    Arc::new(Material::Standard(standard)),
                )));
                scene.add(group, sphere)?;
                scene.insert(NodeKind::Light(Light::Ambient {
                    color: Color::WHITE,
                    intensity: 0.3,
                }));
                let directional = scene.insert(NodeKind::Light(Light::Directional {
                    color: Color::WHITE,
                    intensity: 2.0,
                    target: Vector3::ZERO,
                }));
                scene.get_mut(directional)?.position = Vector3::new(2.0, 3.0, 4.0);
                let point = scene.insert(NodeKind::Light(Light::Point {
                    color: Color::from_hex(0xff8040),
                    intensity: 10.0,
                    distance: 0.0,
                    decay: 2.0,
                }));
                scene.get_mut(point)?.position = Vector3::new(-2.0, 1.0, 2.0);
                let mut line_geometry = BufferGeometry::default();
                line_geometry.set_from_points(&[
                    Vector3::new(-2.0, -1.5, 0.0),
                    Vector3::new(0.0, -0.9, 0.0),
                    Vector3::new(2.0, -1.5, 0.0),
                ])?;
                let mut line_material = LineBasicMaterial::default();
                line_material.properties.color = Color::from_hex(0x44ff88);
                scene.insert(NodeKind::Line(Line {
                    geometry: Arc::new(line_geometry.clone()),
                    material: Arc::new(Material::Line(line_material)),
                    segments: false,
                }));
                let mut points_material = PointsMaterial::default();
                points_material.properties.color = Color::from_hex(0xffffff);
                points_material.size_attenuation = false;
                let points = scene.insert(NodeKind::Points(Points {
                    geometry: Arc::new(line_geometry),
                    material: Arc::new(Material::Points(points_material)),
                }));
                scene.get_mut(points)?.position.y = -0.4;
            } else if example == 2 {
                // Three.js r186 examples/webgl_geometry_cube.html, ported to WebGPU.
                scene.background = Color::BLACK;
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 70.0,
                        aspect: canvas.width() as f64 / canvas.height() as f64,
                        near: 0.1,
                        far: 100.0,
                        ..Default::default()
                    }));
                scene.get_mut(camera)?.position.z = 2.0;
                let mut material = Material::default();
                material.properties_mut().map =
                    Some(Arc::new(Texture::load("/web/crate.gif").await?));
                scene.get_mut(mesh)?.kind = NodeKind::Mesh(Mesh::new(
                    Arc::new(BoxGeometry::build(1.0, 1.0, 1.0)?),
                    Arc::new(material),
                ));
                if !animate {
                    scene.get_mut(mesh)?.quaternion = Euler {
                        angles: Vector3::new(0.4, 0.7, 0.0),
                        order: EulerOrder::XYZ,
                    }
                    .quaternion();
                }
            } else if example != 0 {
                return Err(Error::Invalid("example id"));
            }
            let mut timer = crate::time::Timer::default();
            timer.connect(
                web_sys::window()
                    .and_then(|w| w.document())
                    .ok_or(Error::Asset("document unavailable".into()))?,
            )?;
            let state = Rc::new(RefCell::new(State {
                timer,
                renderer,
                surface,
                configuration,
                target,
                scene,
                camera,
                mesh,
                canvas: canvas.clone(),
                frame: 0,
                animation: None,
                request: None,
                paused: !animate,
                rebuilds: 0,
                example,
                rotation: Vector3::ZERO,
                point_lights,
                gltf,
            }));
            let weak = Rc::downgrade(&state);
            let animation = Closure::wrap(Box::new(move |time: f64| {
                if let Some(state) = weak.upgrade() {
                    let mut state = state.borrow_mut();
                    if let Err(error) = state.render(time) {
                        let _ = state.canvas.set_attribute("data-error", &error.to_string());
                        return;
                    }
                    if let Some(callback) = &state.animation {
                        state.request = web_sys::window().and_then(|w| {
                            w.request_animation_frame(callback.as_ref().unchecked_ref())
                                .ok()
                        });
                    }
                }
            }) as Box<dyn FnMut(f64)>);
            state.borrow_mut().animation = Some(animation);
            let weak = Rc::downgrade(&state);
            let pointer = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                if let Some(state) = weak.upgrade() {
                    let mut state = state.borrow_mut();
                    if state.point_lights.is_some() || state.gltf.is_some() {
                        return;
                    }
                    let ndc = Vector2::new(
                        event.offset_x() as f64 / state.canvas.client_width() as f64 * 2.0 - 1.0,
                        1.0 - event.offset_y() as f64 / state.canvas.client_height() as f64 * 2.0,
                    );
                    let mut raycaster = Raycaster::default();
                    let result = (|| -> Result<bool> {
                        state.scene.update()?;
                        let (camera, world) = state.scene.camera(state.camera)?;
                        raycaster.set_from_camera(ndc, camera, world)?;
                        Ok(!raycaster
                            .intersect_object(&state.scene, state.mesh, false)?
                            .is_empty())
                    })();
                    match result {
                        Ok(selected) => {
                            state.paused = selected;
                            let _ = state.canvas.set_attribute(
                                "data-selected",
                                if selected { "true" } else { "false" },
                            );
                        }
                        Err(error) => {
                            let _ = state.canvas.set_attribute("data-error", &error.to_string());
                        }
                    }
                }
            }) as Box<dyn FnMut(web_sys::PointerEvent)>);
            canvas
                .add_event_listener_with_callback("pointerdown", pointer.as_ref().unchecked_ref())
                .map_err(|e| Error::Asset(format!("{e:?}")))?;
            {
                let mut s = state.borrow_mut();
                s.request = Some(
                    web_sys::window()
                        .ok_or(Error::Asset("window unavailable".into()))?
                        .request_animation_frame(
                            s.animation
                                .as_ref()
                                .expect("animation installed")
                                .as_ref()
                                .unchecked_ref(),
                        )
                        .map_err(|e| Error::Asset(format!("{e:?}")))?,
                );
            }
            Ok(BrowserApp { state, pointer })
        }
        create(canvas, example, animate)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
impl Drop for BrowserApp {
    fn drop(&mut self) {
        let mut s = self.state.borrow_mut();
        if let Some(id) = s.request.take()
            && let Some(window) = web_sys::window()
        {
            let _ = window.cancel_animation_frame(id);
        }
        let _ = s.canvas.remove_event_listener_with_callback(
            "pointerdown",
            self.pointer.as_ref().unchecked_ref(),
        );
        s.animation = None;
    }
}
