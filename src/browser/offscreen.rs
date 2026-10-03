//! webgl_worker_offscreencanvas: the page's scene.js twice, side by side. The
//! left canvas renders on the main thread with the gallery app; the right one
//! is transferred to a module worker ( `web/gallery/offscreen-worker.js` ) that
//! loads this package and renders the same scene into the OffscreenCanvas with
//! its own renderer and animation frames, as the page's worker does. The scene
//! is a hundred matcap icosahedra placed by scene.js's sine PRNG in fog, their
//! group turning by Date.now(). "START JANK" blocks the main thread every
//! 1/60 s with ten million Math.random() calls, which stalls the left canvas
//! only. Under the gallery clock both canvases take Date.now() as the gallery
//! time.
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use js_sys::Reflect;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

const MATCAP: &str = "/web/gallery/assets/offscreen/matcap-porcelain-white.jpg";

fn global() -> JsValue {
    js_sys::global().into()
}
fn call(target: &JsValue, name: &str, args: &[JsValue]) -> Result<JsValue> {
    let f: js_sys::Function = Reflect::get(target, &name.into())
        .ok()
        .and_then(|f| f.dyn_into().ok())
        .ok_or(Error::Invalid("global function"))?;
    f.apply(target, &args.iter().collect::<js_sys::Array>())
        .map_err(|e| Error::Asset(format!("{e:?}")))
}
/// fetch() and ImageBitmapLoader ( premultiplyAlpha none ) through the global
/// scope, so the worker can load the matcap too. The texture keeps the bitmap's
/// rows and samples flipped, as the flipY ImageBitmap's upload is; CanvasTexture
/// leaves it in NoColorSpace, mipmapped.
pub(super) async fn matcap(url: &str) -> Result<Texture> {
    let fail = |e: JsValue| Error::Asset(format!("{e:?}"));
    let response = JsFuture::from(js_sys::Promise::from(call(
        &global(),
        "fetch",
        &[url.into()],
    )?))
    .await
    .map_err(fail)?;
    if !Reflect::get(&response, &"ok".into())
        .map_err(fail)?
        .is_truthy()
    {
        return Err(Error::Asset(format!("HTTP error: {url}")));
    }
    let blob = JsFuture::from(js_sys::Promise::from(call(&response, "blob", &[])?))
        .await
        .map_err(fail)?;
    let options = js_sys::Object::new();
    Reflect::set(&options, &"premultiplyAlpha".into(), &"none".into()).map_err(fail)?;
    let bitmap: web_sys::ImageBitmap = JsFuture::from(js_sys::Promise::from(call(
        &global(),
        "createImageBitmap",
        &[blob, options.into()],
    )?))
    .await
    .map_err(fail)?
    .dyn_into()
    .map_err(fail)?;
    let (w, h) = (bitmap.width(), bitmap.height());
    let canvas = web_sys::OffscreenCanvas::new(w, h).map_err(fail)?;
    let context: web_sys::OffscreenCanvasRenderingContext2d = canvas
        .get_context("2d")
        .map_err(fail)?
        .ok_or(Error::Invalid("matcap context"))?
        .dyn_into()
        .map_err(|e: js_sys::Object| fail(e.into()))?;
    context
        .draw_image_with_image_bitmap(&bitmap, 0., 0.)
        .map_err(fail)?;
    let data = context
        .get_image_data(0., 0., f64::from(w), f64::from(h))
        .map_err(fail)?
        .data()
        .0;
    bitmap.close();
    let mut texture = Texture::from_rgba(w, h, data, false)?;
    texture.mipmap_filter = Some(Filter::Linear);
    Ok(texture)
}

/// scene.js init(): the camera, the fog and the hundred icosahedra in the group.
pub(super) fn build(
    s: &mut Scene,
    c: Object3D,
    aspect: f64,
    matcap: Arc<Texture>,
) -> Result<Object3D> {
    let n = s.get_mut(c)?;
    n.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 40.,
        near: 1.,
        far: 1000.,
        aspect,
        ..Default::default()
    }));
    n.position = Vector3::new(0., 0., 200.);
    s.fog = Some(Fog::Linear {
        color: Color::from_hex(0x444466),
        near: 100.,
        far: 400.,
    });
    s.background = Color::from_hex(0x444466);
    let group = s.insert(NodeKind::Group);
    let geometry = Arc::new(IcosahedronGeometry::build(5., 8)?);
    let materials: Vec<_> = [0xaa24df, 0x605d90, 0xe04a3f, 0xe30456]
        .map(|hex| {
            let mut m = MeshMatcapMaterial::default();
            m.base.properties.color = Color::from_hex(hex);
            m.matcap = Some(matcap.clone());
            Arc::new(Material::Matcap(m))
        })
        .into_iter()
        .collect();
    // scene.js's PRNG: frac( sin( seed++ ) × 10000 ) from seed 1.
    let mut seed = 1.;
    let mut random = || {
        let x = f64::sin(seed) * 10000.;
        seed += 1.;
        x - x.floor()
    };
    for i in 0..100 {
        let h = s.insert(NodeKind::Mesh(Mesh {
            geometry: geometry.clone(),
            materials: vec![materials[i % materials.len()].clone()],
        }));
        let n = s.get_mut(h)?;
        n.position.x = random() * 200. - 100.;
        n.position.y = random() * 200. - 100.;
        n.position.z = random() * 200. - 100.;
        n.scale = Vector3::splat(random() + 1.);
        s.add(group, h)?;
    }
    Ok(group)
}
/// animate(): group.rotation.y = −Date.now() / 4000.
fn turn(s: &mut Scene, group: Object3D, now: f64) -> Result<()> {
    s.get_mut(group)?.quaternion = Quaternion::from_rotation_y(-now / 4000.);
    Ok(())
}

pub struct Demo {
    group: Object3D,
    /// The gallery clock, in seconds, once it drives the page.
    seeked: Option<f64>,
    jank: Option<(i32, Closure<dyn FnMut()>)>,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let texture = matcap(&super::asset_url(MATCAP)?).await?;
        let group = build(s, c, aspect, Arc::new(texture))?;
        Ok(Self {
            group,
            seeked: None,
            jank: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        turn(
            s,
            self.group,
            self.seeked.map_or_else(js_sys::Date::now, |t| t * 1000.),
        )
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    /// The jank button: setInterval( jank, 1000 / 60 ), or clearInterval.
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        let window = web_sys::window().ok_or(Error::Invalid("window"))?;
        let document = window.document().ok_or(Error::Invalid("document"))?;
        let button = document.get_element_by_id("button");
        let result = document.get_element_by_id("result");
        if let Some((id, _)) = self.jank.take() {
            window.clear_interval_with_handle(id);
            if let Some(b) = &button {
                b.set_text_content(Some("START JANK"));
            }
            if let Some(r) = &result {
                r.set_text_content(Some(""));
            }
            return Ok(());
        }
        let jank = Closure::<dyn FnMut()>::new(move || {
            let mut number = 0.;
            for _ in 0..10_000_000 {
                number += js_sys::Math::random();
            }
            if let Some(r) = &result {
                r.set_text_content(Some(&number.to_string()));
            }
        });
        let id = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                jank.as_ref().unchecked_ref(),
                1000 / 60,
            )
            .map_err(|_| Error::Invalid("setInterval"))?;
        if let Some(b) = &button {
            b.set_text_content(Some("STOP JANK"));
        }
        self.jank = Some((id, jank));
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.seeked = Some(t);
    }
}
impl Drop for Demo {
    fn drop(&mut self) {
        if let (Some((id, _)), Some(window)) = (self.jank.take(), web_sys::window()) {
            window.clear_interval_with_handle(id);
        }
    }
}

thread_local! {
    /// The worker's gallery clock, in seconds, when the page posts one.
    static STILL: Cell<Option<f64>> = const { Cell::new(None) };
    /// The worker's MSAA sample count, following the gallery's.
    static SAMPLES: Cell<u32> = const { Cell::new(4) };
}
/// The worker's copy of the gallery's sample count.
#[wasm_bindgen]
pub fn offscreen_samples(samples: u32) {
    SAMPLES.with(|s| s.set(samples));
}
/// The worker's copy of the gallery clock.
#[wasm_bindgen]
pub fn offscreen_time(seconds: f64) {
    STILL.with(|s| s.set(Some(seconds)));
}
/// offscreen.js: init( drawingSurface, width, height, pixelRatio, path ) in the
/// worker, then the animation frames, each posting the frame count back.
#[wasm_bindgen]
pub async fn offscreen_worker(
    canvas: web_sys::OffscreenCanvas,
    width: u32,
    height: u32,
    pixel_ratio: f64,
    matcap_url: String,
) -> std::result::Result<(), JsValue> {
    async fn run(
        canvas: web_sys::OffscreenCanvas,
        width: u32,
        height: u32,
        pixel_ratio: f64,
        matcap_url: String,
    ) -> Result<()> {
        // WebGLRenderer.setSize( width, height, false ) at the pixel ratio.
        let (w, h) = (
            ((f64::from(width) * pixel_ratio).floor() as u32).max(1),
            ((f64::from(height) * pixel_ratio).floor() as u32).max(1),
        );
        canvas.set_width(w);
        canvas.set_height(h);
        let renderer = Renderer::new().await?;
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::OffscreenCanvas(canvas))
            .map_err(|e| Error::Gpu(e.to_string()))?;
        let configuration = surface
            .get_default_config(&renderer.adapter, w, h)
            .ok_or(Error::Gpu("surface configuration unavailable".into()))?;
        surface.configure(&renderer.device, &configuration);
        // WebGLRenderer( { antialias: true } ): 4× MSAA, sRGB-encoded in the shader.
        let target_for = move |renderer: &Renderer, samples: u32| {
            crate::render_target::RenderTarget::with_options(
                &renderer.device,
                w,
                h,
                crate::render_target::RenderTargetOptions {
                    samples,
                    encode_srgb: true,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    ..Default::default()
                },
            )
        };
        let target = target_for(&renderer, SAMPLES.with(Cell::get))?;
        let mut scene = Scene::new();
        let camera = scene.insert(NodeKind::Camera(Camera::Perspective(Default::default())));
        let group = build(
            &mut scene,
            camera,
            f64::from(width) / f64::from(height),
            Arc::new(matcap(&matcap_url).await?),
        )?;
        struct Worker {
            renderer: Renderer,
            surface: wgpu::Surface<'static>,
            format: wgpu::TextureFormat,
            target: crate::render_target::RenderTarget,
            scene: Scene,
            camera: Object3D,
            group: Object3D,
            frames: u32,
        }
        let state = Rc::new(RefCell::new(Worker {
            renderer,
            surface,
            format: configuration.format,
            target,
            scene,
            camera,
            group,
            frames: 0,
        }));
        type Frame = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;
        let frame: Frame = Rc::new(RefCell::new(None));
        let next = frame.clone();
        *frame.borrow_mut() = Some(Closure::new(move || {
            let step = || -> Result<()> {
                let mut k = state.borrow_mut();
                let k = &mut *k;
                let samples = SAMPLES.with(Cell::get);
                if k.target.options.samples != samples {
                    k.target = target_for(&k.renderer, samples)?;
                }
                let now = STILL
                    .with(Cell::get)
                    .map_or_else(js_sys::Date::now, |t| t * 1000.);
                turn(&mut k.scene, k.group, now)?;
                k.renderer.render(&mut k.scene, k.camera, &k.target)?;
                let output = k
                    .surface
                    .get_current_texture()
                    .map_err(|e| Error::Gpu(e.to_string()))?;
                let view = output.texture.create_view(&wgpu::TextureViewDescriptor {
                    format: Some(k.format),
                    ..Default::default()
                });
                k.renderer.blit_with_tone_mapping(
                    &k.target,
                    &view,
                    k.format,
                    1.,
                    ToneMapping::None,
                );
                output.present();
                k.frames += 1;
                let message = js_sys::Object::new();
                let _ = Reflect::set(&message, &"frames".into(), &k.frames.into());
                call(&global(), "postMessage", &[message.into()])?;
                Ok(())
            };
            if let Err(e) = step() {
                let message = js_sys::Object::new();
                let _ = Reflect::set(&message, &"error".into(), &e.to_string().into());
                let _ = call(&global(), "postMessage", &[message.into()]);
                return;
            }
            if let Some(f) = next.borrow().as_ref() {
                let _ = call(&global(), "requestAnimationFrame", &[f.as_ref().clone()]);
            }
        }));
        if let Some(f) = frame.borrow().as_ref() {
            call(&global(), "requestAnimationFrame", &[f.as_ref().clone()])?;
        }
        // The closure keeps itself alive through `next` for the worker's lifetime.
        std::mem::forget(frame);
        Ok(())
    }
    run(canvas, width, height, pixel_ratio, matcap_url)
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
