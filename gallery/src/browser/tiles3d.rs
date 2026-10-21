//! webgl_loader_3dtiles: Google Photorealistic 3D Tiles over Tokyo, streamed
//! through Cesium ion by 3d-tiles-renderer 0.4.27, under takram's
//! atmosphere and clouds.
//!
//! `tiles3d/` ports the tile renderer: traversal, the queues, the cache, the
//! page's plugins and the tile models ( GLTFLoader, creased normals ). The
//! camera starts in the ellipsoid's camera frame above Tokyo
//! ( 35.6812°, 139.80°, 500 m, azimuth −90°, elevation −10° ).
use crate::{Error, Result, camera::*, math::*, renderer::*, scene::*};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

mod atmosphere;
mod clouds;
mod content;
mod controls;
mod ellipsoid;
mod fade;
mod ion_key;
mod program;
mod render;
mod sun;
mod tiles;
mod volume;
mod vsop_earth;

use tiles::{Pending, TileId, Tiles};
use volume::M4;

const ASSET: &str = "https://api.cesium.com/v1/assets/2275207/endpoint";
/// @takram/three-clouds' and @takram/three-geospatial's DEFAULT_*_URL.
const CLOUDS_ASSETS: &str = "https://media.githubusercontent.com/media/takram-design-engineering/three-geospatial/45a1c6c1bb9fd38b3680fd120795ff4c32df68ff/packages/clouds/assets";
const STBN_URL: &str = "https://media.githubusercontent.com/media/takram-design-engineering/three-geospatial/9627216cc50057994c98a2118f3c4a23765d43b9/packages/core/assets/stbn.bin";
/// Date.UTC( 2024, 2, 1 ).
const MARCH_2024: f64 = 1709251200000.;
const DEG2RAD: f64 = std::f64::consts::PI / 180.;
const FOV: f64 = 75.;

type Cell<T> = Rc<RefCell<Option<std::result::Result<T, String>>>>;

async fn fetch(url: &str) -> std::result::Result<web_sys::Response, String> {
    fetch_with(url, None).await
}
/// fetch( url ) with a referrer policy: the ion key allows the gallery's path,
/// and the default cross-origin referrer is only the page's origin.
async fn fetch_with(
    url: &str,
    policy: Option<web_sys::ReferrerPolicy>,
) -> std::result::Result<web_sys::Response, String> {
    let window = web_sys::window().ok_or("window")?;
    let init = web_sys::RequestInit::new();
    if let Some(policy) = policy {
        init.set_referrer_policy(policy);
    }
    JsFuture::from(window.fetch_with_str_and_init(url, &init))
        .await
        .map_err(|e| format!("{e:?}"))?
        .dyn_into::<web_sys::Response>()
        .map_err(|_| "response".into())
}
async fn fetch_bytes(url: &str) -> std::result::Result<Vec<u8>, String> {
    read_bytes(fetch(url).await?).await
}
async fn read_bytes(response: web_sys::Response) -> std::result::Result<Vec<u8>, String> {
    if !response.ok() {
        return Err(format!(
            "Failed to load model with error code {}",
            response.status()
        ));
    }
    let data = JsFuture::from(response.array_buffer().map_err(|e| format!("{e:?}"))?)
        .await
        .map_err(|e| format!("{e:?}"))?;
    Ok(js_sys::Uint8Array::new(&data).to_vec())
}
async fn fetch_json(url: &str) -> std::result::Result<serde_json::Value, String> {
    serde_json::from_slice(&fetch_bytes(url).await?).map_err(|e| e.to_string())
}
/// URL.searchParams.set on each pair.
fn with_params(url: &str, params: &[(&str, &str)]) -> String {
    let Ok(u) = web_sys::Url::new(url) else {
        return url.to_owned();
    };
    for (k, v) in params {
        u.search_params().set(k, v);
    }
    u.href()
}

/// The ion endpoint, the Google root tileset and its session.
struct Boot {
    root: serde_json::Value,
    url: String,
    key: String,
    session: Option<String>,
}
async fn boot() -> std::result::Result<Boot, String> {
    // CesiumIonAuth.refreshToken, then _initializeFromAsset's Google hand-off.
    // The full page URL as the referrer, which the key's allowed URL matches.
    let response = fetch_with(
        &with_params(ASSET, &[("access_token", ion_key::ION_KEY)]),
        Some(web_sys::ReferrerPolicy::NoReferrerWhenDowngrade),
    )
    .await?;
    let asset: serde_json::Value =
        serde_json::from_slice(&read_bytes(response).await?).map_err(|e| e.to_string())?;
    let url = asset["options"]["url"]
        .as_str()
        .ok_or("ion asset url")?
        .to_owned();
    let key = web_sys::Url::new(&url)
        .ok()
        .and_then(|u| u.search_params().get("key"))
        .ok_or("Google key")?;
    // GoogleCloudAuth.fetch of the root: the key, no session yet.
    let root = fetch_json(&with_params(&url, &[("key", &key)])).await?;
    let session = session_token(&root["root"]);
    Ok(Boot {
        root,
        url,
        key,
        session,
    })
}
/// getSessionToken: the first content URI's session parameter, depth first.
fn session_token(tile: &serde_json::Value) -> Option<String> {
    if let Some(uri) = tile["content"]["uri"].as_str() {
        let params = uri.split_once('?').map(|(_, p)| p)?;
        return web_sys::UrlSearchParams::new_with_str(params)
            .ok()?
            .get("session");
    }
    for c in tile["children"].as_array()? {
        if let Some(s) = session_token(c) {
            return Some(s);
        }
    }
    None
}

/// The queues' fetches and parses as futures.
struct Loader;
impl tiles::Loader for Loader {
    fn download(&mut self, p: &Pending, key: Option<&str>, session: Option<&str>) -> Cell<Vec<u8>> {
        let cell: Cell<Vec<u8>> = Rc::default();
        let mut params = vec![];
        if let Some(k) = key {
            params.push(("key", k));
        }
        if let Some(s) = session {
            params.push(("session", s));
        }
        let url = with_params(&p.url, &params);
        let out = cell.clone();
        wasm_bindgen_futures::spawn_local(async move {
            *out.borrow_mut() = Some(fetch_bytes(&url).await);
        });
        cell
    }
    fn parse(
        &mut self,
        p: &Pending,
        bytes: Vec<u8>,
        transform: &M4,
        up: &M4,
    ) -> Cell<Option<content::TileContent>> {
        let cell: Cell<Option<content::TileContent>> = Rc::default();
        let (url, transform, up, out) = (p.url.clone(), *transform, *up, cell.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let result = content::parse(bytes, &url, transform, up)
                .await
                .map(Some)
                .map_err(|e| e.to_string());
            *out.borrow_mut() = Some(result);
        });
        cell
    }
}

/// The fetched cloud textures: local weather, turbulence, shape, shape detail, STBN.
type AssetBytes = [Vec<u8>; 5];

pub(super) struct Demo {
    boot: Cell<Boot>,
    tiles: Tiles,
    loader: Loader,
    camera_world: M4,
    /// Each shown tile's meshes ( GPU geometry from its first display until
    /// the tile is disposed ) and whether it is drawn.
    meshes: HashMap<TileId, (Vec<render::TileMesh>, bool)>,
    asset_bytes: Cell<AssetBytes>,
    shared: Option<render::Shared>,
    atmosphere: Option<atmosphere::Precomputed>,
    assets: Option<render::Assets>,
    frame: render::Frame,
    screen: Option<RenderTarget>,
    /// GlobeControls ( enableDamping ) driving the camera.
    controls: controls::Controls,
    controls_time: Option<f64>,
    /// params.hourUTC.
    hour: f64,
    delta: f64,
    now: f64,
    pending: bool,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        // The page's DRACOLoader.
        crate::compression::install_draco();
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: FOV,
            near: 1.,
            far: 1e6,
            aspect,
            ..Default::default()
        }));
        // ellipsoid.getObjectFrame( ..., camera.matrix, CAMERA_FRAME ).
        let ellipsoid = ellipsoid::Ellipsoid::default();
        let camera_world = ellipsoid.object_frame(
            35.6812 * DEG2RAD,
            139.80 * DEG2RAD,
            500.,
            -90. * DEG2RAD,
            -10. * DEG2RAD,
            0.,
            ellipsoid::CAMERA_FRAME,
        );
        {
            let n = s.get_mut(c)?;
            n.matrix = Matrix4::from_cols_array(&camera_world);
            n.matrix_auto_update = false;
            n.matrix_world_needs_update = true;
        }
        s.background = Color::BLACK;
        let boot: Cell<Boot> = Rc::default();
        let out = boot.clone();
        wasm_bindgen_futures::spawn_local(async move {
            *out.borrow_mut() = Some(boot_task().await);
        });
        // loadCloudTexture, loadData3DTexture and STBNLoader.load.
        let asset_bytes: Cell<AssetBytes> = Rc::default();
        let out = asset_bytes.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let load = async {
                Ok::<AssetBytes, String>([
                    fetch_bytes(&format!("{CLOUDS_ASSETS}/local_weather.png")).await?,
                    fetch_bytes(&format!("{CLOUDS_ASSETS}/turbulence.png")).await?,
                    fetch_bytes(&format!("{CLOUDS_ASSETS}/shape.bin")).await?,
                    fetch_bytes(&format!("{CLOUDS_ASSETS}/shape_detail.bin")).await?,
                    fetch_bytes(STBN_URL).await?,
                ])
            };
            *out.borrow_mut() = Some(load.await);
        });
        let mut tiles = Tiles::new(ASSET.to_owned());
        tiles.fade = fade::Fade::new();
        Ok(Self {
            boot,
            tiles,
            loader: Loader,
            camera_world,
            meshes: HashMap::new(),
            asset_bytes,
            shared: None,
            atmosphere: None,
            assets: None,
            frame: render::Frame::default(),
            screen: None,
            controls: controls::Controls::new(controls::Camera::from_matrix(
                &camera_world,
                FOV,
                aspect,
                10.,
                1e6,
            )),
            // EnvironmentControls' _lastTime starts at construction.
            controls_time: web_sys::window()
                .and_then(|w| w.performance())
                .map(|p| p.now()),
            hour: 0.,
            delta: 0.,
            now: 0.,
            pending: false,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
            self.delta += dt;
            self.pending = true;
        }
        Ok(())
    }
    /// animate(): controls.update(), tiles.update(), then the scene follows
    /// the visible set.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        if self.shared.is_none() {
            self.shared = Some(render::Shared::new(r)?);
            // await texturesGenerator.update().
            self.atmosphere = Some(atmosphere::precompute(r)?);
        }
        if let Some(result) = self.asset_bytes.borrow_mut().take() {
            let [weather, turbulence, shape, detail, stbn] = result.map_err(Error::Asset)?;
            self.assets = Some(render::Assets::new(
                r,
                &weather,
                &turbulence,
                &shape,
                &detail,
                &stbn,
            )?);
        }
        if let Some(result) = self.boot.borrow_mut().take() {
            let b = result.map_err(Error::Asset)?;
            // GoogleCloudAuthPlugin.init: errorTarget 20 ( useRecommendedSettings ).
            self.tiles.error_target = 20.;
            self.tiles.google_key = Some(b.key);
            self.tiles.session = b.session;
            self.tiles.root_url = b.url.clone();
            self.tiles.set_root(b.root, &b.url);
        }
        self.tiles.tick(&mut self.loader);
        if !std::mem::take(&mut self.pending) {
            return Ok(());
        }
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let resolution = canvas_size().unwrap_or((1., 1.));
        // controls.update( Math.min( performance delta, 64 ms ) ): the camera,
        // then GlobeControls.adjustCamera's near and far.
        let now = web_sys::window()
            .and_then(|w| w.performance())
            .map_or(0., |p| p.now());
        let delta = (now - self.controls_time.unwrap_or(now)) * 1e-3;
        self.controls_time = Some(now);
        let dpr = web_sys::window().map_or(1., |w| w.device_pixel_ratio());
        self.controls.client = resolution;
        if self.controls.camera.aspect != aspect {
            self.controls.camera.aspect = aspect;
            self.controls.camera.update_projection_matrix();
        }
        self.controls.update(
            &controls::TilesScene { tiles: &self.tiles },
            delta.min(64. / 1000.),
            dpr,
        );
        let camera = &self.controls.camera;
        self.camera_world = camera.world;
        let (near, far) = (camera.near, camera.far);
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = near;
            p.far = far;
        }
        {
            let n = s.get_mut(c)?;
            n.matrix = Matrix4::from_cols_array(&self.camera_world);
            n.matrix_world_needs_update = true;
        }
        let projection = camera.projection;
        self.tiles.camera = Some(tiles::CameraState {
            world: self.camera_world,
            world_inverse: volume::invert(&self.camera_world),
            projection,
            projection_inverse: volume::invert(&projection),
            resolution,
        });
        self.tiles.fade.now = self.now;
        self.tiles.update();
        self.sync(r)?;
        // A comparison hook: window.tilesState asks for the traversal summary.
        if let Some(window) = web_sys::window()
            && js_sys::Reflect::get(&window, &"tilesState".into()).is_ok_and(|v| v.is_truthy())
            && let Some(body) = window.document().and_then(|d| d.body())
        {
            let _ = body.set_attribute("data-tiles", &self.summary());
        }
        Ok(())
    }

    /// TilesRenderer.setTileVisible's group membership, and the fade plugin's
    /// rule that a visible tile outside the frustum is hidden. A tile's
    /// geometry is uploaded when first shown and kept until it is disposed.
    fn sync(&mut self, r: &Renderer) -> Result<()> {
        let events = std::mem::take(&mut self.tiles.events);
        for (t, visible) in events.visibility {
            if !visible {
                if let Some(entry) = self.meshes.get_mut(&t) {
                    entry.1 = false;
                }
                continue;
            }
            if let Some(entry) = self.meshes.get_mut(&t) {
                entry.1 = true;
                continue;
            }
            let Some(scene) = self.tiles.tiles[t].scene.clone() else {
                continue;
            };
            let mut meshes = vec![];
            for (matrix, geometry, material) in &scene.meshes {
                let array = |name: &str| match geometry.attributes.get(name) {
                    Some(crate::geometry::Attribute::F32(a)) => Ok(a.array().to_vec()),
                    _ => Err(Error::Invalid("tile attribute")),
                };
                let map = material
                    .properties()
                    .map
                    .clone()
                    .ok_or(Error::Invalid("tile map"))?;
                meshes.push(render::TileMesh::new(
                    r,
                    &array("position")?,
                    &array("normal")?,
                    &array("uv")?,
                    &map,
                    *matrix,
                )?);
            }
            self.meshes.insert(t, (meshes, true));
        }
        for (t, _) in events.disposed {
            self.meshes.remove(&t);
        }
        Ok(())
    }

    /// renderer.render( scene, camera ) with the effects, into the canvas.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let (Some(shared), Some(atmosphere), Some(assets)) =
            (&self.shared, &self.atmosphere, &self.assets)
        else {
            return Ok(false);
        };
        let size = (out.width, out.height);
        if self
            .screen
            .as_ref()
            .is_none_or(|t| (t.width, t.height) != size)
        {
            self.screen = Some(RenderTarget::with_options(
                &r.device,
                size.0,
                size.1,
                out.options.clone(),
            )?);
        }
        let Some(camera) = &self.tiles.camera else {
            return Ok(false);
        };
        let near_far = match s.camera(c)?.0 {
            Camera::Perspective(p) => (p.near, p.far, p.aspect),
            _ => (1., 1e6, 1.),
        };
        let view = render::View {
            world: camera.world,
            view: camera.world_inverse,
            projection: camera.projection,
            near: near_far.0,
            far: near_far.1,
            aspect: near_far.2,
        };
        // updateSunDirection: getSunDirectionECEF( Date.UTC( 2024, 2, 1 ) + hour ).
        self.frame.sun_direction = sun::sun_direction_ecef(MARCH_2024 + self.hour * 3600000.);
        let fade = self.tiles.fade.values();
        let mut draws = vec![];
        for (t, (meshes, visible)) in self.meshes.iter_mut() {
            let tile = &self.tiles.tiles[*t];
            let shown = *visible
                && (tile.traversal.as_ref().is_some_and(|tr| tr.in_frustum)
                    || self.tiles.fade.is_fading_out(*t));
            if !shown {
                continue;
            }
            let f = fade.get(t).copied();
            for m in meshes.iter_mut() {
                draws.push((m, f));
            }
        }
        let screen = self
            .screen
            .as_ref()
            .ok_or(Error::Invalid("tiles3d screen"))?;
        let delta = std::mem::take(&mut self.delta);
        let pivot = self.controls.pivot();
        self.frame.render(
            r,
            shared,
            atmosphere,
            assets,
            &view,
            &mut draws,
            pivot,
            &screen.view,
            screen.options.format,
            size,
            delta,
        )?;
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.screen.as_ref()
    }
    /// The target `output` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_mut(&mut self) -> Option<&mut RenderTarget> {
        self.screen.as_mut()
    }

    /// The visible tiles' paths in the tree ( child indices from the root,
    /// through external tilesets ), sorted, for the comparison.
    fn summary(&self) -> String {
        let path = |mut t: TileId| {
            let mut parts = vec![];
            while let Some(p) = self.tiles.tiles[t].parent {
                parts.push(
                    self.tiles.tiles[p]
                        .children
                        .iter()
                        .position(|&c| c == t)
                        .unwrap_or(usize::MAX)
                        .to_string(),
                );
                t = p;
            }
            parts.reverse();
            parts.join("/")
        };
        let mut paths: Vec<String> = (0..self.tiles.tiles.len())
            .filter(|&t| {
                self.tiles.tiles[t]
                    .traversal
                    .as_ref()
                    .is_some_and(|tr| tr.used || self.tiles.visible_tiles.contains(&t))
            })
            .map(|t| {
                let tile = &self.tiles.tiles[t];
                let tr = tile.traversal.as_ref().expect("traversal");
                format!(
                    "{} e={:.4} f={} s={} v={} a={} ge={}",
                    path(t),
                    tr.error,
                    tr.in_frustum as u8,
                    tile.loading_state,
                    self.tiles.visible_tiles.contains(&t) as u8,
                    tr.active as u8,
                    tile.geometric_error
                )
            })
            .collect();
        paths.sort();
        let s = &self.tiles.stats;
        let cam: Vec<String> = self
            .camera_world
            .iter()
            .map(|v| format!("{v:.3}"))
            .collect();
        format!(
            "{} {} {} {} {}\n{}\n{}",
            s.in_cache,
            s.visible,
            s.active,
            s.used,
            self.tiles.frame_count,
            cam.join(","),
            paths.join("\n")
        )
    }
    /// GlobeControls' pointer listeners ( client coordinates ): 10 + button
    /// down, 0 move, 20 + button up.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        let (left, top) = canvas_origin().unwrap_or((0., 0.));
        let (x, y) = (x - left, y - top);
        match kind {
            10..=19 => self.controls.pointer_down(
                &controls::TilesScene { tiles: &self.tiles },
                kind - 10,
                x,
                y,
            ),
            20..=29 => self.controls.pointer_up(),
            _ => self.controls.pointer_move(x, y),
        }
        self.pending = true;
    }
    /// The shift key ( shift + drag rotates ).
    pub fn key(&mut self, code: u32, down: bool) {
        if code == 16 {
            self.controls.shift = down;
        }
    }
    #[allow(clippy::too_many_arguments)]
    /// The wheel ( deltaMode pixels ).
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        if wheel != 0. {
            self.controls.wheel(wheel, None);
            self.pending = true;
        }
        Ok(())
    }
    /// The hour slider ( UTC ).
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index == 0 {
            self.hour = value as f64;
        }
        Ok(())
    }
    /// The page's clock set directly: animate( time ) with _deltaTime from
    /// the previous time.
    pub fn seek(&mut self, t: f64) {
        self.delta += (t * 1000. - self.now).max(0.) / 1000.;
        self.now = t * 1000.;
        self.pending = true;
    }
}

async fn boot_task() -> std::result::Result<Boot, String> {
    boot().await
}

/// The canvas's client rect origin ( PointerTracker.getAdjustedPointer ).
fn canvas_origin() -> Option<(f64, f64)> {
    let canvas = web_sys::window()?
        .document()?
        .query_selector("canvas")
        .ok()??;
    let rect = canvas.get_bounding_client_rect();
    Some((rect.left(), rect.top()))
}

/// renderer.getSize(): the canvas's CSS size.
fn canvas_size() -> Option<(f64, f64)> {
    let canvas = web_sys::window()?
        .document()?
        .query_selector("canvas")
        .ok()??;
    Some((canvas.client_width() as f64, canvas.client_height() as f64))
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
        if let Some(target) = self.output() {
            return Some(Some(target));
        }
        None
    }
    fn on_output_target_mut(&mut self) -> Option<Option<&mut crate::renderer::RenderTarget>> {
        Some(self.output_mut())
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

/// The program of three.js examples/webgl_loader_3dtiles.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_3dtiles(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 1,
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                10.,
                1e6,
                Box::new(Demo::create(scene, camera, 488, renderer).await?),
            ))
        },
    )
    .await
}
