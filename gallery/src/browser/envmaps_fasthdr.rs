//! webgl_materials_envmaps_fasthdr ( r187 ): five spheres ( transmissive
//! glass, rough, mirror, brushed metal and glossy green ) lit by a FastHDR
//! prefiltered HDR cubemap: a Poly Haven HDRI that PMREMGenerator filtered
//! into a cube of six 512-texel faces and seven levels, stored as UASTC HDR
//! KTX2 ( textures/pmrem/ ). As on the page, KTX2Loader loads it and it serves
//! as scene.environment and scene.background with isPMREMTexture, without
//! filtering again; ACES Filmic, damped OrbitControls and the image,
//! exposure, fov and blurriness GUI.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use crate::environment::EnvironmentMap;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;

/// The GUI's images, in its order.
const IMAGES: [&str; 8] = [
    "ballroom",
    "brown_photostudio_02",
    "cape_hill",
    "cannon",
    "metro_noord",
    "the_sky_is_on_fire",
    "studio_small_09",
    "wide_street_01",
];
/// KTX2Loader.setPath( 'textures/pmrem/' ).loadAsync( `${ name }_2k.pmrem.ktx2` ).
fn url(image: usize) -> String {
    format!("textures/pmrem/{}_2k.pmrem.ktx2", IMAGES[image])
}
type Loaded = Rc<RefCell<Option<(usize, Result<Vec<u8>>)>>>;
pub(super) struct Demo {
    controls: Controls,
    /// The image requested by the GUI, and the one fetched or in flight.
    image: usize,
    requested: Option<usize>,
    /// A finished load and the image it is of.
    loaded: Loaded,
    fov: f64,
    exposure: f64,
    blurriness: f64,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        // The page's KTX2Loader.
        crate::basis::install();
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 0.1,
            far: 50.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(7., 0., 0.);
        s.tone_mapping = ToneMapping::Aces;
        let geometry = Arc::new(SphereGeometry::build(0.45, 64, 32)?);
        let standard = |metalness: f64, roughness: f64, color: u32| MeshStandardMaterial {
            properties: MaterialProperties {
                color: Color::from_hex(color),
                ..Default::default()
            },
            metalness,
            roughness,
            // r186's WebGL physical shading: the DFG LUT's multiple scattering.
            energy_conservation: true,
            ..Default::default()
        };
        let glass = MeshPhysicalMaterial {
            base: standard(0., 0., 0xffffff),
            transmission: 1.,
            thickness: 2.,
            ..Default::default()
        };
        let materials = [
            Material::Physical(glass),
            Material::Standard(standard(0., 1., 0xffffff)),
            Material::Standard(standard(1., 0., 0xffffff)),
            Material::Standard(standard(1., 0.5, 0x888888)),
            Material::Standard(standard(0., 0., 0x6ab440)),
        ];
        for (z, material) in [2., 1., 0., -1., -2.].into_iter().zip(materials) {
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(material),
            )));
            s.get_mut(mesh)?.position.z = z;
        }
        let mut controls = Controls::new(Some(0.05), (0.1, 20.), PI, true);
        controls.update(s, c)?;
        // The first image loads with the page.
        let bytes = fetch(&url(0)).await?;
        install(s, r, &bytes)?;
        Ok(Self {
            controls,
            image: 0,
            requested: Some(0),
            loaded: Rc::new(RefCell::new(None)),
            fov: 40.,
            exposure: 1.,
            blurriness: 0.,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// A changed image loads in the background; a load that is no longer the
    /// GUI's image is dropped ( texture.dispose() ), else it replaces the
    /// environment and the background. render(): controls.update().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        if self.requested != Some(self.image) {
            self.requested = Some(self.image);
            let (slot, image) = (self.loaded.clone(), self.image);
            wasm_bindgen_futures::spawn_local(async move {
                let result = fetch(&url(image)).await;
                *slot.borrow_mut() = Some((image, result));
            });
        }
        let loaded = self.loaded.borrow_mut().take();
        if let Some((image, bytes)) = loaded
            && image == self.image
        {
            install(s, r, &bytes?)?;
        }
        // render(): toneMappingExposure and backgroundBlurriness from the GUI.
        s.exposure = self.exposure;
        s.background_blur = self.blurriness;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.fov = self.fov;
        }
        self.controls.update(s, c)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// The GUI: image, exposure, fov, background blurriness.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => self.image = (value as usize).min(IMAGES.len() - 1),
            1 => self.exposure = f64::from(value),
            2 => self.fov = f64::from(value),
            3 => self.blurriness = f64::from(value),
            _ => return Err(Error::Invalid("envmaps_fasthdr parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
/// `texture.isPMREMTexture = true; scene.environment = scene.background =
/// texture`.
fn install(s: &mut Scene, r: &Renderer, bytes: &[u8]) -> Result<()> {
    s.environment = Some(Arc::new(EnvironmentMap::from_pmrem_ktx2(r, bytes)?));
    s.background_environment = true;
    s.background_pmrem = true;
    // WebGLBackground tone-maps textures that are not sRGB.
    s.background_tone_mapped = true;
    Ok(())
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
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
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

/// The program of three.js examples/webgl_materials_envmaps_fasthdr.html: its renderer, its scene
/// and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_materials_envmaps_fasthdr(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                50.,
                Box::new(Demo::create(scene, camera, 470, renderer).await?),
            ))
        },
    )
    .await
}
