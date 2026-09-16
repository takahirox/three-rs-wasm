//! Browser resource loading and camera controls for the reusable static glTF importer.
use crate::{Error, Result, camera::*, math::*, scene::*};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub(super) async fn fetch(url: &str) -> Result<Vec<u8>> {
    let response = JsFuture::from(
        web_sys::window()
            .ok_or(Error::Invalid("window"))?
            .fetch_with_str(url),
    )
    .await
    .map_err(|e| Error::Asset(format!("{e:?}")))?
    .dyn_into::<web_sys::Response>()
    .map_err(|_| Error::Invalid("response"))?;
    if !response.ok() {
        return Err(Error::Asset(format!("HTTP {}: {url}", response.status())));
    }
    let data = JsFuture::from(
        response
            .array_buffer()
            .map_err(|e| Error::Asset(format!("{e:?}")))?,
    )
    .await
    .map_err(|e| Error::Asset(format!("{e:?}")))?;
    Ok(js_sys::Uint8Array::new(&data).to_vec())
}
fn external(base: &str, uri: &str) -> Result<String> {
    if uri.starts_with("data:") {
        return Ok(uri.to_owned());
    }
    Ok(format!("{base}/{uri}"))
}
pub(super) struct GltfViewer {
    center: Vector3,
    radius: f64,
    initial_radius: f64,
    yaw: f64,
    pitch: f64,
    pub triangles: usize,
    pub meshes: usize,
}
impl GltfViewer {
    pub async fn create(
        scene: &mut Scene,
        camera: Object3D,
        placeholder: Object3D,
        example: u32,
        environment: Option<Arc<crate::environment::EnvironmentMap>>,
    ) -> Result<Self> {
        let url = if example == 4 {
            "/web/models/DamagedHelmet/glTF/DamagedHelmet.gltf"
        } else {
            "/web/models/BoomBox.glb"
        };
        let base = url.rsplit_once('/').ok_or(Error::Invalid("asset URL"))?.0;
        let bytes = fetch(url).await?;
        let asset = gltf::Gltf::from_slice(&bytes).map_err(|e| Error::Asset(e.to_string()))?;
        let mut buffers = Vec::new();
        for buffer in asset.buffers() {
            let data = match buffer.source() {
                gltf::buffer::Source::Bin => {
                    asset.blob.clone().ok_or(Error::Invalid("GLB buffer"))?
                }
                gltf::buffer::Source::Uri(uri) => fetch(&external(base, uri)?).await?,
            };
            if data.len() < buffer.length() {
                return Err(Error::Invalid("short glTF buffer"));
            }
            buffers.push(data);
        }
        let mut images = Vec::new();
        for image in asset.images() {
            let data = match image.source() {
                gltf::image::Source::Uri { uri, .. } => fetch(&external(base, uri)?).await?,
                gltf::image::Source::View { view, .. } => {
                    let buffer = buffers
                        .get(view.buffer().index())
                        .ok_or(Error::Invalid("image buffer"))?;
                    let end = view
                        .offset()
                        .checked_add(view.length())
                        .ok_or(Error::Invalid("image range"))?;
                    buffer
                        .get(view.offset()..end)
                        .ok_or(Error::Invalid("image range"))?
                        .to_vec()
                }
            };
            images.push(decode_image(&data).await?);
        }
        let imported = crate::gltf::import_decoded(&asset, &buffers, &images)?;
        let bounds = imported.bounds;
        let triangles = imported.triangles;
        let meshes = imported.mesh_count();
        let environment = match environment {
            Some(image) => image,
            None => Arc::new(crate::environment::EnvironmentMap::from_hdr(
                &fetch("/web/environments/royal_esplanade_2k.hdr").await?,
            )?),
        };
        imported.instantiate(scene)?;
        scene.dispose(placeholder)?;
        scene.environment = Some(environment);
        scene.background_environment = true;
        scene.aces_tone_mapping = true;
        let radius = bounds.size().max_element() * 1.8;
        scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.0,
            aspect: 1.0,
            near: radius / 100.0,
            far: radius * 100.0,
            ..Default::default()
        }));
        scene.background = Color::from_hex(0x101820);
        let viewer = Self {
            center: bounds.center(),
            radius,
            initial_radius: radius,
            yaw: -0.55,
            pitch: 0.18,
            triangles,
            meshes,
        };
        viewer.update(scene, camera)?;
        Ok(viewer)
    }
    pub fn fixture(&mut self, yaw: f64, pitch: f64, distance: f64) {
        self.yaw = yaw;
        self.pitch = pitch;
        self.radius = self.initial_radius / 1.8 * distance;
    }
    pub fn orbit(&mut self, dx: f64, dy: f64, zoom: f64) {
        self.yaw -= dx * 0.008;
        self.pitch = (self.pitch + dy * 0.008).clamp(-1.4, 1.4);
        self.radius = (self.radius * (zoom * 0.001).exp())
            .clamp(self.initial_radius * 0.3, self.initial_radius * 3.0);
    }
    pub fn update(&self, scene: &mut Scene, camera: Object3D) -> Result<()> {
        if let NodeKind::Camera(Camera::Perspective(perspective)) = &mut scene.get_mut(camera)?.kind
        {
            perspective.near = self.radius / 100.0;
            perspective.far = self.radius * 100.0;
        }
        scene.get_mut(camera)?.position = self.center
            + Vector3::new(
                self.yaw.sin() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.cos() * self.pitch.cos(),
            ) * self.radius;
        scene.look_at(camera, self.center)
    }
}

// JPEG IDCT rounding differs between native codecs. Use the browser codec, as
// GLTFLoader does, for JPEG normal maps; keep lossless images on the Rust path
// to preserve RGB under transparent pixels without a canvas premultiply roundtrip.
async fn decode_image(bytes: &[u8]) -> Result<crate::material::Texture> {
    use crate::material::Texture;
    if !bytes.starts_with(&[0xff, 0xd8]) {
        return Texture::from_image(bytes);
    }
    let fail = |e| Error::Asset(format!("JPEG decode: {e:?}"));
    let parts = js_sys::Array::new();
    parts.push(&js_sys::Uint8Array::from(bytes));
    let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).map_err(fail)?;
    let options = web_sys::ImageBitmapOptions::new();
    options.set_color_space_conversion(web_sys::ColorSpaceConversion::None);
    options.set_premultiply_alpha(web_sys::PremultiplyAlpha::None);
    let bitmap: web_sys::ImageBitmap = JsFuture::from(
        web_sys::window()
            .ok_or(Error::Invalid("window"))?
            .create_image_bitmap_with_blob_and_image_bitmap_options(&blob, &options)
            .map_err(fail)?,
    )
    .await
    .map_err(fail)?
    .dyn_into()
    .map_err(fail)?;
    let result = (|| {
        let canvas =
            web_sys::OffscreenCanvas::new(bitmap.width(), bitmap.height()).map_err(fail)?;
        let context: web_sys::OffscreenCanvasRenderingContext2d = canvas
            .get_context("2d")
            .map_err(fail)?
            .ok_or(Error::Invalid("JPEG decode context"))?
            .dyn_into()
            .map_err(|e: js_sys::Object| fail(e.into()))?;
        context
            .draw_image_with_image_bitmap(&bitmap, 0.0, 0.0)
            .map_err(fail)?;
        let image = context
            .get_image_data(0.0, 0.0, bitmap.width() as f64, bitmap.height() as f64)
            .map_err(fail)?;
        Texture::from_rgba(bitmap.width(), bitmap.height(), image.data().0, true)
    })();
    bitmap.close();
    result
}
