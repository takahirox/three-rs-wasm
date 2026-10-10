//! webgl_renderer_pathtracer: three-gpu-pathtracer 0.0.24 ( MIT, Garrett
//! Johnson ) over the LDraw X-wing ( 7140 ) on a radial-faded metal floor,
//! lit by the blurred royal_esplanade UltraHDR environment under a gradient
//! background.
//!
//! Loading follows the page: LDrawLoader, LDrawUtils.mergeObject, the page's
//! material changes, the floor sized to the model's bounds, then
//! PathTracingSceneGenerator's merged geometry and SAH MeshBVH, packed into
//! the data textures PhysicalPathTracingMaterial samples ( `pathtracer/` ).
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use std::cell::RefCell;
use std::rc::Rc;

mod bvh;
mod environment;
mod gpu;
mod math;
mod raster;
mod render;
mod sampling;
mod scene;
mod textures;

const MODEL: &str = "/web/gallery/assets/ldraw/7140-1-X-wingFighter.mpd_Packed.mpd";
const ENVIRONMENT: &str = "/web/gallery/assets/probes-hdr/royal_esplanade_2k.hdr.jpg";
const FLOOR_SIZE: usize = 1024;

/// The CPU side of setScene: the meshes in merge order, the merged geometry,
/// its BVH and the decoded environment.
pub(crate) struct Prepared {
    pub meshes: Vec<scene::Mesh>,
    pub merged: scene::Merged,
    pub bvh: bvh::MeshBvh,
    pub bounds: ([f64; 3], [f64; 3]),
    pub floor_map: Vec<u8>,
    /// UltraHDRLoader's half-float RGBA rows, top first ( the texture flips Y ).
    pub environment: (usize, usize, Vec<u16>),
}

/// generateRadialFloorTexture( 1024 ).
fn radial_floor(dim: usize) -> Vec<u8> {
    let mut data = vec![0u8; dim * dim * 4];
    for x in 0..dim {
        for y in 0..dim {
            let x_norm = x as f64 / (dim - 1) as f64;
            let y_norm = y as f64 / (dim - 1) as f64;
            let x_cent = 2.0 * (x_norm - 0.5);
            let y_cent = 2.0 * (y_norm - 0.5);
            let mut a = (1.0 - (pow(x_cent, 2.) + pow(y_cent, 2.)).sqrt()).clamp(0.0, 1.0);
            a = pow(a, 1.5);
            a *= 1.5;
            a = a.min(1.0);
            let i = y * dim + x;
            data[i * 4..i * 4 + 3].fill(255);
            // Uint8Array stores ToUint8 of the product: truncation.
            data[i * 4 + 3] = (a * 255.) as u8;
        }
    }
    data
}

fn pow(a: f64, b: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::pow(a, b);
    #[cfg(not(target_arch = "wasm32"))]
    a.powf(b)
}

/// loadModel's work up to setScene.
async fn prepare() -> Result<Prepared> {
    let text = fetch(MODEL).await?;
    let mut loader = super::ldraw_loader::Loader::new(true);
    let (group, _) = loader.load(&String::from_utf8_lossy(&text))?;
    let merged_object = scene::merge_object(&loader, &group);
    // legoGroup.rotation.x = Math.PI; updateMatrixWorld: the merged meshes
    // carry no transform of their own.
    let model = math::compose([0.; 3], math::rotation_x(std::f64::consts::PI), [1.; 3]);
    let alone = math::multiply(&model, &math::IDENTITY);
    let mut meshes: Vec<scene::Mesh> = merged_object
        .into_iter()
        .map(|(face, positions, normals)| scene::Mesh {
            positions,
            normals,
            uvs: None,
            index: None,
            material: scene::page_material(&loader.faces[face]),
            matrix_world: alone,
        })
        .collect();
    // Box3.setFromObject( model ) before the model joins the scene.
    let bounds = scene::bounds(&meshes);
    // setScene's updateMatrixWorld under the scene.
    let in_scene = math::multiply(&math::multiply(&math::IDENTITY, &model), &math::IDENTITY);
    for m in &mut meshes {
        m.matrix_world = in_scene;
    }
    // The floor: PlaneGeometry scaled 2500, rotated −π/2 about X, at the
    // model's lowest point; double sided, transparent, radial-mapped metal.
    let (positions, normals, uvs, index) = scene::plane();
    let floor_local = math::compose(
        [0., bounds.0[1], 0.],
        math::rotation_x(-std::f64::consts::PI / 2.),
        [2500.; 3],
    );
    meshes.push(scene::Mesh {
        positions,
        normals,
        uvs: Some(uvs),
        index: Some(index),
        material: scene::Material {
            physical: false,
            color: [1.; 3],
            roughness: 0.15,
            metalness: 0.9,
            emissive: [0.; 3],
            emissive_intensity: 1.,
            opacity: 1.,
            transparent: true,
            side: scene::Side::Double,
            ior: 1.5,
            transmission: 0.,
            thickness: 0.,
            map: true,
            polygon_offset: false,
        },
        matrix_world: math::multiply(&math::IDENTITY, &floor_local),
    });
    let merged = scene::merge(&meshes);
    let bvh = bvh::build(&merged.positions, &merged.index);
    let environment = environment::ultra_hdr(&fetch(ENVIRONMENT).await?).await?;
    Ok(Prepared {
        meshes,
        merged,
        bvh,
        bounds,
        floor_map: radial_floor(FLOOR_SIZE),
        environment,
    })
}

/// FNV-1a over the bytes, as the comparison hooks hash uploads: hex:length.
fn hash(bytes: &[u8]) -> String {
    let mut h: u32 = 2166136261;
    for &b in bytes {
        h ^= b as u32;
        h = h.wrapping_mul(16777619);
    }
    format!("{h:x}:{}", bytes.len())
}

/// The data textures' hashes, for the browser comparison with the uploads
/// of the original.
fn data_hashes(p: &Prepared) -> Vec<(&'static str, String)> {
    let mut random = sampling::Random::new();
    let mut stratified = sampling::StratifiedSamples::new(&mut random);
    let blue = sampling::blue_noise(64, &mut random);
    stratified.init(20, 25, &mut random);
    stratified.next(&mut random);
    let m = &p.merged;
    let mut dereferenced = Vec::with_capacity(m.index.len());
    for &t in &p.bvh.indirect {
        dereferenced.extend_from_slice(&m.index[t as usize * 3..t as usize * 3 + 3]);
    }
    let ((_, bounds), (_, contents)) = bvh::textures(&p.bvh);
    let (_, _, position) = textures::float_texture(&m.positions, 3);
    let (_, index) = textures::index_texture(&dereferenced);
    let (_, attributes) = textures::attribute_array(&m.normals, &m.tangents, &m.uvs, &m.colors);
    let dim = textures::dimension(m.material_index.len());
    let mut material_index = m.material_index.clone();
    material_index.resize(dim * dim, 0);
    let materials: Vec<scene::Material> = p.meshes.iter().map(|m| m.material.clone()).collect();
    let (_, materials) = textures::materials(&materials);
    let gradient = textures::gradient(
        512,
        Color::from_hex(0xeeeeee).0.to_array(),
        Color::from_hex(0xeaeaea).0.to_array(),
    );
    vec![
        ("index", hash(bytemuck::cast_slice(&index))),
        ("position", hash(bytemuck::cast_slice(&position))),
        ("bvhBounds", hash(bytemuck::cast_slice(&bounds))),
        ("bvhContents", hash(bytemuck::cast_slice(&contents))),
        ("attributes", hash(bytemuck::cast_slice(&attributes))),
        ("materialIndex", hash(&material_index)),
        ("materials", hash(bytemuck::cast_slice(&materials))),
        ("blueNoise", hash(bytemuck::cast_slice(&blue))),
        (
            "stratified",
            hash(bytemuck::cast_slice(&stratified.samples)),
        ),
        ("gradient", hash(bytemuck::cast_slice(&gradient))),
        ("floorMap", hash(&p.floor_map)),
        ("environment", hash(bytemuck::cast_slice(&p.environment.2))),
        (
            "meshes",
            p.meshes
                .iter()
                .map(|m| {
                    format!(
                        "{}/{:.3}/{}",
                        m.positions.len() / 3,
                        m.material.roughness,
                        m.material.physical
                    )
                })
                .collect::<Vec<_>>()
                .join(" "),
        ),
    ]
}

fn atan2(y: f64, x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::atan2(y, x);
    #[cfg(not(target_arch = "wasm32"))]
    y.atan2(x)
}
fn acos(x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::acos(x);
    #[cfg(not(target_arch = "wasm32"))]
    x.acos()
}
fn tan(x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return js_sys::Math::tan(x);
    #[cfg(not(target_arch = "wasm32"))]
    x.tan()
}

/// controls.target0: the model's bounds centre.
fn center(bounds: &([f64; 3], [f64; 3])) -> [f64; 3] {
    let (lo, hi) = bounds;
    [
        (lo[0] + hi[0]) * 0.5,
        (lo[1] + hi[1]) * 0.5,
        (lo[2] + hi[2]) * 0.5,
    ]
}

/// The page's camera after controls.reset(): OrbitControls.update's
/// spherical round trip and target clamp, then lookAt, in three's f64
/// operations, with PerspectiveCamera( 45, aspect, 1, 10000 )'s projection.
fn page_camera(bounds: &([f64; 3], [f64; 3]), aspect: f64) -> render::Camera {
    let (lo, hi) = bounds;
    let size = [hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]];
    let radius = size[0].max(size[1].max(size[2])) * 0.4;
    let center = [
        (lo[0] + hi[0]) * 0.5,
        (lo[1] + hi[1]) * 0.5,
        (lo[2] + hi[2]) * 0.5,
    ];
    let position0 = [
        2.3 * radius + center[0],
        1. * radius + center[1],
        2. * radius + center[2],
    ];
    // update(): the offset in spherical coordinates and back.
    let v = [
        position0[0] - center[0],
        position0[1] - center[1],
        position0[2] - center[2],
    ];
    let r = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let (mut theta, mut phi) = (0., 0.);
    if r != 0. {
        theta = atan2(v[0], v[2]);
        phi = acos((v[1] / r).clamp(-1., 1.));
    }
    let eps = 0.000001;
    phi = phi.clamp(eps, std::f64::consts::PI - eps);
    // target.clampLength( 0, Infinity ).
    let l = (center[0] * center[0] + center[1] * center[1] + center[2] * center[2]).sqrt();
    let inv = 1. / if l == 0. { 1. } else { l };
    let target = center.map(|c| c * inv * l);
    let sin_phi_r = math::sin(phi) * r;
    let offset = [
        sin_phi_r * math::sin(theta),
        math::cos(phi) * r,
        sin_phi_r * math::cos(theta),
    ];
    let position = [
        target[0] + offset[0],
        target[1] + offset[1],
        target[2] + offset[2],
    ];
    // camera.lookAt( target ): Matrix4.lookAt( eye, target, up ), then the quaternion.
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let cross = |a: [f64; 3], b: [f64; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let len2 = |a: [f64; 3]| a[0] * a[0] + a[1] * a[1] + a[2] * a[2];
    let mut z = sub(position, target);
    if len2(z) == 0. {
        z[2] = 1.;
    }
    let z = math::normalize(z);
    let x = math::normalize(cross([0., 1., 0.], z));
    let y = cross(z, x);
    let (m11, m12, m13, m21, m22, m23, m31, m32, m33) =
        (x[0], y[0], z[0], x[1], y[1], z[1], x[2], y[2], z[2]);
    let trace = m11 + m22 + m33;
    let q = if trace > 0. {
        let s = 0.5 / (trace + 1.).sqrt();
        [(m32 - m23) * s, (m13 - m31) * s, (m21 - m12) * s, 0.25 / s]
    } else if m11 > m22 && m11 > m33 {
        let s = 2. * (1. + m11 - m22 - m33).sqrt();
        [0.25 * s, (m12 + m21) / s, (m13 + m31) / s, (m32 - m23) / s]
    } else if m22 > m33 {
        let s = 2. * (1. + m22 - m11 - m33).sqrt();
        [(m12 + m21) / s, 0.25 * s, (m23 + m32) / s, (m13 - m31) / s]
    } else {
        let s = 2. * (1. + m33 - m11 - m22).sqrt();
        [(m13 + m31) / s, (m23 + m32) / s, 0.25 * s, (m21 - m12) / s]
    };
    camera_from(position, q, aspect)
}

/// The camera at a pose: its world matrix, the inverse and
/// PerspectiveCamera( 45, aspect, 1, 10000 )'s inverse projection ( WebGL
/// clip space, as the path tracer unprojects ).
fn camera_from(position: [f64; 3], q: [f64; 4], aspect: f64) -> render::Camera {
    let world = math::compose(position, q, [1.; 3]);
    // updateProjectionMatrix ( WebGL coordinates ) and its inverse.
    let (near, far) = (1., 10000.);
    let top = near * tan(std::f64::consts::PI / 180. * 0.5 * 45.);
    let height = 2. * top;
    let width = aspect * height;
    let left = -0.5 * width;
    let (right, bottom) = (left + width, top - height);
    let mut p = [0.; 16];
    p[0] = 2. * near / (right - left);
    p[5] = 2. * near / (top - bottom);
    p[8] = (right + left) / (right - left);
    p[9] = (top + bottom) / (top - bottom);
    p[10] = -(far + near) / (far - near);
    p[14] = (-2. * far * near) / (far - near);
    p[11] = -1.;
    render::Camera {
        world,
        world_inverse: math::invert(&world),
        projection_inverse: math::invert(&p),
        position,
    }
}

/// Loading, then the GPU resources: the environment read-back in flight, and
/// the path tracer once its tables exist.
#[allow(clippy::large_enum_variant)]
enum Stage {
    Loading,
    Blurring(Box<Prepared>, render::Readback),
    Ready(
        Box<Prepared>,
        render::Environment,
        Option<crate::environment::EnvironmentMap>,
    ),
}

pub(super) struct Demo {
    slot: Rc<RefCell<Option<Result<Prepared>>>>,
    stage: Stage,
    tracer: Option<render::Tracer>,
    screen: Option<RenderTarget>,
    /// The example clock ( ms ) and whether a frame is due.
    now: f64,
    pending: bool,
    /// OrbitControls ( no damping ); the page camera is exact until moved.
    controls: Controls,
    orbiting: bool,
    /// The GUI: enable, pause, toneMapping, transparentBackground,
    /// resolutionScale, tiles, floor roughness, floor metalness.
    params: [f64; 8],
    /// Floor material changes for updateMaterials.
    materials_changed: bool,
    floor: Option<Object3D>,
    /// The gradient background, unset while the background is transparent.
    background: Option<std::sync::Arc<crate::environment::EnvironmentMap>>,
    /// The download button: the canvas read back after the next frame.
    download: Download,
}

#[derive(Default)]
enum Download {
    #[default]
    Idle,
    Requested,
    Reading(wgpu::Buffer, (u32, u32, u32), Rc<RefCell<bool>>),
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(150., 200., 250.);
        let slot = Rc::new(RefCell::new(None));
        let target = slot.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let result = prepare().await;
            *target.borrow_mut() = Some(result);
        });
        Ok(Self {
            slot,
            stage: Stage::Loading,
            tracer: None,
            screen: None,
            now: 0.,
            pending: false,
            controls: Controls::new(None, (0., f64::INFINITY), std::f64::consts::PI, true),
            orbiting: false,
            params: [1., 0., 1., 0., 1., 3., 0.15, 0.9],
            materials_changed: false,
            floor: None,
            background: None,
            download: Download::Idle,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.now += dt * 1000.;
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, r: &Renderer) -> Result<()> {
        if let Stage::Loading = self.stage
            && let Some(result) = self.slot.borrow_mut().take()
        {
            let p = result?;
            if let Some(body) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
            {
                let json = data_hashes(&p)
                    .iter()
                    .map(|(k, v)| format!("\"{k}\":\"{v}\""))
                    .collect::<Vec<_>>()
                    .join(",");
                let _ = body.set_attribute("data-pt-hashes", &format!("{{{json}}}"));
            }
            let readback = render::blur_environment(r, &p.environment)?;
            self.stage = Stage::Blurring(Box::new(p), readback);
        }
        if let Stage::Blurring(_, readback) = &self.stage
            && let Some(halfs) = readback.take()
        {
            let halfs = halfs?;
            let size = readback.size();
            let env = render::environment(r, size, &halfs);
            let raster_environment = raster::environment(size, &halfs);
            if let Some(body) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
            {
                let _ =
                    body.set_attribute("data-pt-environment", &hash(bytemuck::cast_slice(&halfs)));
                // A comparison hook: the read-back halfs, copied out on request.
                if let Some(window) = web_sys::window()
                    && js_sys::Reflect::get(&window, &"ptExposeEnvironment".into())
                        .is_ok_and(|v| v.is_truthy())
                {
                    let _ = js_sys::Reflect::set(
                        &window,
                        &"ptEnvironment".into(),
                        &js_sys::Uint16Array::from(&halfs[..]),
                    );
                }
            }
            let Stage::Blurring(p, _) = std::mem::replace(&mut self.stage, Stage::Loading) else {
                unreachable!()
            };
            self.stage = Stage::Ready(p, env, Some(raster_environment));
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let Stage::Ready(p, env, raster_environment) = &mut self.stage else {
            return Ok(false);
        };
        let size = (out.width, out.height);
        if let Some(environment) = raster_environment.take() {
            let (floor, background) = raster::install(s, p, environment)?;
            (self.floor, self.background) = (Some(floor), Some(background));
        }
        let Some(tracer) = self.tracer.as_mut() else {
            self.tracer = Some(render::Tracer::new(r, p, env, size, out.options.format)?);
            if let Some(body) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
            {
                let _ = body.set_attribute("data-pt-ready", "true");
                // hideProgressBar(); document.body.classList.add( 'checkerboard' ).
                body.set_class_name(format!("{} checkerboard", body.class_name()).trim());
            }
            return self.render(r, s, c, out);
        };
        // onWindowResize: the drawing buffer, the aspect and updateCamera.
        if tracer.canvas != size {
            tracer.canvas = size;
            tracer.reset();
        }
        // The canvas target's size and antialiasing.
        if self
            .screen
            .as_ref()
            .is_none_or(|t| (t.width, t.height) != size || t.options.samples != out.options.samples)
        {
            self.screen = Some(RenderTarget::with_options(
                &r.device,
                size.0,
                size.1,
                out.options.clone(),
            )?);
        }
        if std::mem::take(&mut self.materials_changed) {
            // floor.material.roughness / metalness, then updateMaterials.
            let floor = p
                .meshes
                .last_mut()
                .ok_or(Error::Invalid("pathtracer floor"))?;
            floor.material.roughness = self.params[6];
            floor.material.metalness = self.params[7];
            let materials: Vec<scene::Material> =
                p.meshes.iter().map(|m| m.material.clone()).collect();
            tracer.update_materials(r, &materials);
            if let Some(h) = self.floor
                && let NodeKind::Mesh(mesh) = &mut s.get_mut(h)?.kind
            {
                mesh.materials = vec![std::sync::Arc::new(raster::material(
                    &p.meshes[p.meshes.len() - 1].material,
                    &p.floor_map,
                )?)];
            }
        }
        if !std::mem::take(&mut self.pending) {
            // The button reads the canvas as last drawn.
            self.read_download(r);
            return Ok(true);
        }
        let aspect = size.0 as f64 / size.1 as f64;
        if let NodeKind::Camera(Camera::Perspective(pc)) = &mut s.get_mut(c)?.kind {
            pc.aspect = aspect;
        }
        let camera = if self.orbiting {
            let n = s.get_mut(c)?;
            n.matrix_auto_update = true;
            let (q, v) = (n.quaternion, n.position);
            camera_from([v.x, v.y, v.z], [q.x, q.y, q.z, q.w], aspect)
        } else {
            let camera = page_camera(&p.bounds, aspect);
            let n = s.get_mut(c)?;
            n.matrix = Matrix4::from_cols_array(&camera.world);
            n.matrix_auto_update = false;
            n.matrix_world_needs_update = true;
            camera
        };
        // animate(): the renderer's tone mapping and the path tracer's flags.
        let tone_mapping = self.params[2] != 0.;
        s.tone_mapping = if tone_mapping {
            crate::scene::ToneMapping::Aces
        } else {
            crate::scene::ToneMapping::None
        };
        tracer.tone_mapping = tone_mapping;
        tracer.enable = self.params[0] != 0.;
        tracer.pause = self.params[1] != 0.;
        s.background_map = if self.params[3] != 0. {
            None
        } else {
            self.background.clone()
        };
        let raster = tracer.render_sample(r, &camera, self.now)?;
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body.set_attribute("data-pt-samples", &format!("{}", tracer.samples.floor()));
        }
        let screen = self
            .screen
            .as_ref()
            .ok_or(Error::Invalid("pathtracer screen"))?;
        if raster {
            // renderer.render( scene, camera ).
            r.render(s, c, screen)?;
        }
        let mut encoder = r.device.create_command_encoder(&Default::default());
        tracer.draw_quad(r, &mut encoder, &screen.view, !raster)?;
        r.queue.submit([encoder.finish()]);
        self.read_download(r);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.screen.as_ref()
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    /// OrbitControls' change event: updateCamera resets the path tracer.
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
        let Stage::Ready(p, ..) = &self.stage else {
            return Ok(());
        };
        if dx == 0. && dy == 0. && wheel == 0. {
            return Ok(());
        }
        if !self.orbiting {
            // Hand the exact page camera to the controls.
            let aspect = match s.camera(c)?.0 {
                Camera::Perspective(p) => p.aspect,
                _ => 1.,
            };
            let camera = page_camera(&p.bounds, aspect);
            let n = s.get_mut(c)?;
            n.matrix_auto_update = true;
            n.position = Vector3::from_slice(&camera.position);
            self.controls
                .set_target(Vector3::from_array(center(&p.bounds)));
            self.orbiting = true;
        }
        let state = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &state, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&state, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        let (position, quaternion) = (s.get(c)?.position, s.get(c)?.quaternion);
        self.controls.update(s, c)?;
        // The change event: the camera moved beyond OrbitControls' EPS.
        let n = s.get(c)?;
        if (position.distance_squared(n.position) > 1e-6
            || 8. * (1. - quaternion.dot(n.quaternion)) > 1e-6)
            && let Some(t) = &mut self.tracer
        {
            t.reset();
        }
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let value = value as f64;
        if index == 8 {
            self.download = Download::Requested;
            return Ok(());
        }
        let previous = *self
            .params
            .get(index)
            .ok_or(Error::Invalid("pathtracer parameter"))?;
        self.params[index] = value;
        let Some(t) = &mut self.tracer else {
            return Ok(());
        };
        match index {
            3 => {
                // scene.background = v ? null : gradientMap; updateEnvironment.
                t.transparent_background = value != 0.;
                t.reset();
            }
            4 => {
                t.render_scale = value;
                t.reset();
            }
            5 => t.tiles = value.round().clamp(1., 6.) as usize,
            6 | 7 => self.materials_changed = previous != value,
            _ => {}
        }
        Ok(())
    }
    /// renderer.domElement.toDataURL(): the drawn frame ( preserveDrawingBuffer ).
    fn read_download(&mut self, r: &Renderer) {
        let (Download::Requested, Some(screen)) = (&self.download, &self.screen) else {
            return;
        };
        let (w, h) = (screen.width, screen.height);
        let row = (w * 4).div_ceil(256) * 256;
        let buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("pathtracer download"),
            size: (row * h) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            screen.texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row),
                    rows_per_image: Some(h),
                },
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );
        r.queue.submit([encoder.finish()]);
        let ready = Rc::new(RefCell::new(false));
        let signal = ready.clone();
        buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                *signal.borrow_mut() = result.is_ok()
            });
        self.download = Download::Reading(buffer, (w, h, row), ready);
    }
    /// The download's PNG once read back ( un-premultiplied, as the canvas
    /// encodes ).
    pub fn take_export(&mut self) -> Option<(String, Vec<u8>)> {
        let Download::Reading(buffer, (w, h, row), ready) = &self.download else {
            return None;
        };
        if !*ready.borrow() {
            return None;
        }
        let mapped = buffer.slice(..).get_mapped_range();
        let mut rgba = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..*h {
            for px in mapped[(y * row) as usize..(y * row + w * 4) as usize].chunks(4) {
                let a = px[3] as f64;
                let un = |c: u8| {
                    if a > 0. {
                        ((c as f64) * 255. / a).round().min(255.) as u8
                    } else {
                        0
                    }
                };
                rgba.extend([un(px[0]), un(px[1]), un(px[2]), px[3]]);
            }
        }
        drop(mapped);
        buffer.unmap();
        let (w, h) = (*w, *h);
        self.download = Download::Idle;
        let image = image::RgbaImage::from_raw(w, h, rgba)?;
        let mut png = std::io::Cursor::new(Vec::new());
        image.write_to(&mut png, image::ImageFormat::Png).ok()?;
        Some(("pathtraced-render.png".into(), png.into_inner()))
    }
    /// The example clock: the fixture renders each frame at a time.
    pub fn seek(&mut self, t: f64) {
        self.now = t * 1000.;
        self.pending = true;
    }
}
