//! Five pinned WebGPU examples for material groups, clipping and texture sampling.
use super::gltf_viewer::{OrbitViewer, decode_texture_image, fetch};
use crate::material::Texture;
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, surface::SurfaceNodes, *},
};
use std::sync::Arc;
mod paired;
mod partial;
mod solids;
const ASSETS: &str = "/web/gallery/assets/material-textures";
fn mesh(s: &mut Scene, geometry: Arc<BufferGeometry>, material: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(geometry, Arc::new(material))))
}
async fn image(name: &str) -> Result<Texture> {
    decode_texture_image(&fetch(&format!("{ASSETS}/{name}")).await?).await
}
fn camera(s: &mut Scene, c: Object3D, fov: f64, near: f64, far: f64) -> Result<()> {
    let aspect = match s.camera(c)?.0 {
        Camera::Perspective(p) => p.aspect,
        _ => 1.,
    };
    s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov,
        near,
        far,
        aspect,
        ..Default::default()
    }));
    Ok(())
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    viewer: OrbitViewer,
    orbit: Vector2,
    pan: Vector3,
    params: [f32; 7],
    objects: Vec<Object3D>,
    paired: Option<paired::Pair>,
    partial: Option<partial::Partial>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let mut d = Self {
            id,
            time: 0.,
            viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            params: [1., 1., 0., 1., 0.8, 1., 0.1],
            objects: Vec::new(),
            paired: None,
            partial: None,
        };
        match id {
            148 => d.solids(s, c).await?,
            149 => d.clipping(s, c)?,
            150 | 151 | 174 => d.paired = Some(paired::Pair::new(r, id == 151, id == 174).await?),
            152 => {
                camera(s, c, 70., 0.01, 10.)?;
                s.get_mut(c)?.position.z = 2.;
                d.partial = Some(partial::Partial::new(s, r).await?);
            }
            _ => return Err(Error::Invalid("material/texture example")),
        }
        Ok(d)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.paired.as_ref().map(|p| &p.output)
    }
    /// The target `output` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_mut(&mut self) -> Option<&mut RenderTarget> {
        self.paired.as_mut().map(|p| &mut p.output)
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        if let Some(p) = &mut self.paired
            && let Some(w) = web_sys::window()
        {
            p.pointer = Vector2::new(
                x * w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1.) / 2.,
                y * w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(1.) / 2.,
            );
        }
    }
    pub fn seek(&mut self, time: f64) {
        self.time = time;
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if self.id != 149 || index >= 7 || !value.is_finite() {
            return Err(Error::Invalid("clipping parameter"));
        }
        self.params[index] = match index {
            4 => value.clamp(0.3, 1.25),
            6 => value.clamp(-0.4, 3.),
            _ => f32::from(value > 0.5),
        };
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
        }
        if self.id <= 149 {
            let damp = if self.id == 148 { 0.05 } else { 1. };
            self.viewer.orbit_pixels(
                self.orbit.x * damp,
                self.orbit.y * damp,
                0.,
                1.,
                0.,
                f64::INFINITY,
            );
            self.viewer.pan_world(self.pan * damp);
            self.orbit *= 1. - damp;
            self.pan *= 1. - damp;
            self.viewer.update(s, c)?;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
                p.near = if self.id == 148 { 0.1 } else { 0.25 };
                p.far = if self.id == 148 { 100. } else { 16. };
            }
        }
        if self.id == 149 {
            self.update_clipping(s)?;
        }
        if let Some(p) = &mut self.paired {
            p.update()?;
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        w: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if self.id > 149 {
            return Ok(());
        }
        if pan {
            self.pan += OrbitViewer::pan_delta(s, c, self.viewer.radius(), dx, dy, height)?;
        } else {
            self.orbit += Vector2::new(dx, dy) / height.max(1.);
        }
        self.viewer.orbit_pixels(
            0.,
            0.,
            w,
            height,
            if self.id == 148 { 5. } else { 0. },
            if self.id == 148 { 50. } else { f64::INFINITY },
        );
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if let Some(p) = &mut self.paired {
            p.render(r, out)?;
            return Ok(true);
        }
        if let Some(p) = &mut self.partial {
            r.render(s, c, out)?;
            p.update(r, self.time)?;
            return Ok(true);
        }
        Ok(false)
    }
}

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_gpu_pointer(
        &mut self,
        r: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        cam: crate::scene::Object3D,
        x: f64,
        y: f64,
    ) -> Option<crate::Result<bool>> {
        self.pointer(x, y);
        None
    }
    fn on_render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Option<crate::Result<bool>> {
        Some(self.render(renderer, scene, camera, target))
    }
    fn on_output_target(&self) -> Option<Option<&crate::renderer::RenderTarget>> {
        Some(self.output())
    }
    fn on_output_target_mut(&mut self) -> Option<Option<&mut crate::renderer::RenderTarget>> {
        Some(self.output_mut())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/webgpu_materials_arrays.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_materials_arrays(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 148, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_clipping.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_clipping(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 149, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_materials_texture_manualmipmap.html: its renderer, its
/// scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_materials_texture_manualmipmap(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 150, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_textures_anisotropy.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_textures_anisotropy(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 151, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_textures_partialupdate.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_textures_partialupdate(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 152, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_materials_texture_filters.html: its renderer, its scene
/// and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_materials_texture_filters(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 174, renderer).await?),
            ))
        },
    )
    .await
}
