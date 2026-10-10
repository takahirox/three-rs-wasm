//! webgl_morphtargets_webcam: MediaPipe's FaceLandmarker drives the Face Cap
//! head over the mirrored webcam image.
//!
//! The camera is MediaPipe's virtual camera ( 63° fov, 1 to 10,000 cm, at the
//! origin ). The video plane fills the frustum 100 cm away. The whole scene is
//! mirrored in x for a selfie view. The model is reparented into the
//! registration transform under the pose container; the head and teeth draw
//! with MeshNormalMaterial.
//!
//! MediaPipe stays the page's JavaScript ( `window.galleryFaceDetect`, which
//! the gallery page installs ). Each animation frame calls its
//! detectForVideo once the video has metadata. This module then copies the
//! pose matrix verbatim and maps the blendshape scores onto the morph target
//! influences. The eye look scores become the eyes' x and z rotations, as
//! animate() does.
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, uv};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::sync::Arc;
use wasm_bindgen::JsCast;

const MODEL: &str = "/web/gallery/assets/draco-variants/facecap.glb";
const MP_FOV: f64 = 63.;
const VIDEO_DISTANCE: f64 = 100.;

/// The page's blendshapesMap: MediaPipe category → Face Cap morph target.
const BLENDSHAPES: [(&str, &str); 51] = [
    ("browDownLeft", "browDown_L"),
    ("browDownRight", "browDown_R"),
    ("browInnerUp", "browInnerUp"),
    ("browOuterUpLeft", "browOuterUp_L"),
    ("browOuterUpRight", "browOuterUp_R"),
    ("cheekPuff", "cheekPuff"),
    ("cheekSquintLeft", "cheekSquint_L"),
    ("cheekSquintRight", "cheekSquint_R"),
    ("eyeBlinkLeft", "eyeBlink_L"),
    ("eyeBlinkRight", "eyeBlink_R"),
    ("eyeLookDownLeft", "eyeLookDown_L"),
    ("eyeLookDownRight", "eyeLookDown_R"),
    ("eyeLookInLeft", "eyeLookIn_L"),
    ("eyeLookInRight", "eyeLookIn_R"),
    ("eyeLookOutLeft", "eyeLookOut_L"),
    ("eyeLookOutRight", "eyeLookOut_R"),
    ("eyeLookUpLeft", "eyeLookUp_L"),
    ("eyeLookUpRight", "eyeLookUp_R"),
    ("eyeSquintLeft", "eyeSquint_L"),
    ("eyeSquintRight", "eyeSquint_R"),
    ("eyeWideLeft", "eyeWide_L"),
    ("eyeWideRight", "eyeWide_R"),
    ("jawForward", "jawForward"),
    ("jawLeft", "jawLeft"),
    ("jawOpen", "jawOpen"),
    ("jawRight", "jawRight"),
    ("mouthClose", "mouthClose"),
    ("mouthDimpleLeft", "mouthDimple_L"),
    ("mouthDimpleRight", "mouthDimple_R"),
    ("mouthFrownLeft", "mouthFrown_L"),
    ("mouthFrownRight", "mouthFrown_R"),
    ("mouthFunnel", "mouthFunnel"),
    ("mouthLeft", "mouthLeft"),
    ("mouthLowerDownLeft", "mouthLowerDown_L"),
    ("mouthLowerDownRight", "mouthLowerDown_R"),
    ("mouthPressLeft", "mouthPress_L"),
    ("mouthPressRight", "mouthPress_R"),
    ("mouthPucker", "mouthPucker"),
    ("mouthRight", "mouthRight"),
    ("mouthRollLower", "mouthRollLower"),
    ("mouthRollUpper", "mouthRollUpper"),
    ("mouthShrugLower", "mouthShrugLower"),
    ("mouthShrugUpper", "mouthShrugUpper"),
    ("mouthSmileLeft", "mouthSmile_L"),
    ("mouthSmileRight", "mouthSmile_R"),
    ("mouthStretchLeft", "mouthStretch_L"),
    ("mouthStretchRight", "mouthStretch_R"),
    ("mouthUpperUpLeft", "mouthUpperUp_L"),
    ("mouthUpperUpRight", "mouthUpperUp_R"),
    ("noseSneerLeft", "noseSneer_L"),
    ("noseSneerRight", "noseSneer_R"),
];

struct Face {
    head: Object3D,
    eyes: [(Object3D, [f64; 3]); 2],
    /// morphTargetDictionary: the head's target names in index order.
    targets: Vec<String>,
}

struct Stream {
    texture: wgpu::Texture,
    uploaded: Option<f64>,
}

pub(super) struct Demo {
    container: Object3D,
    registration: Object3D,
    video_mesh: Object3D,
    face: Option<Face>,
    video: Option<web_sys::HtmlVideoElement>,
    stream: Option<Stream>,
    program: Option<ShaderProgram>,
    sampler: wgpu::Sampler,
    /// The camera aspect once the video has metadata.
    aspect: Option<f64>,
    pending: bool,
    /// Slider changes, applied before the next frame.
    changes: Vec<(usize, f64)>,
}

/// Euler XYZ from a quaternion ( Euler.setFromQuaternion ).
fn euler(q: Quaternion) -> [f64; 3] {
    let m = Matrix3::from_quat(q).to_cols_array();
    let (m11, m12, m13) = (m[0], m[3], m[6]);
    let (m22, m23) = (m[4], m[7]);
    let (m32, m33) = (m[5], m[8]);
    let y = m13.clamp(-1., 1.).asin();
    if m13.abs() < 0.9999999 {
        [(-m23).atan2(m33), y, (-m12).atan2(m11)]
    } else {
        [m32.atan2(m22), y, 0.]
    }
}
/// Quaternion.setFromEuler, XYZ.
fn quaternion([x, y, z]: [f64; 3]) -> Quaternion {
    let (c1, c2, c3) = ((x / 2.).cos(), (y / 2.).cos(), (z / 2.).cos());
    let (s1, s2, s3) = ((x / 2.).sin(), (y / 2.).sin(), (z / 2.).sin());
    Quaternion::from_xyzw(
        s1 * c2 * c3 + c1 * s2 * s3,
        c1 * s2 * c3 - s1 * c2 * s3,
        c1 * c2 * s3 + s1 * s2 * c3,
        c1 * c2 * c3 - s1 * s2 * s3,
    )
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: MP_FOV,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::ZERO;
        s.tone_mapping = ToneMapping::Aces;
        s.background = Color::from_hex(0x666666);
        // scene.scale.x = -1: everything below mirrors with the scene.
        let root = s.insert(NodeKind::Group);
        s.get_mut(root)?.scale = Vector3::new(-1., 1., 1.);
        let ambient = s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 5.,
        }));
        s.add(root, ambient)?;
        let container = s.insert(NodeKind::Group);
        {
            let n = s.get_mut(container)?;
            n.matrix = Matrix4::from_translation(Vector3::new(0., 0., -50.));
            n.matrix_auto_update = false;
            n.matrix_world_needs_update = true;
        }
        s.add(root, container)?;
        let registration = s.insert(NodeKind::Group);
        {
            let n = s.get_mut(registration)?;
            n.scale = Vector3::splat(0.958);
            n.quaternion = quaternion([std::f64::consts::PI / 2., 0., 0.]);
            n.position = Vector3::new(0., 0.12, 1.18);
        }
        s.add(container, registration)?;
        // The video plane: a 1 × 1 plane until the video's metadata sizes it.
        let mut black = MeshBasicMaterial::default();
        black.properties.color = Color::BLACK;
        black.properties.depth_test = false;
        black.properties.depth_write = false;
        let video_mesh = s.insert(NodeKind::Mesh(Mesh {
            geometry: Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
            materials: vec![Arc::new(Material::Basic(black))],
        }));
        {
            let n = s.get_mut(video_mesh)?;
            n.position.z = -VIDEO_DISTANCE;
            n.render_order = -1;
        }
        s.add(root, video_mesh)?;
        let video = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("video"))
            .and_then(|v| v.dyn_into().ok());
        // VideoTexture: linear filtering without mipmaps, sRGB.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let placeholder = video_texture(r, wgpu::Extent3d::default());
        let program = ShaderProgram::with_textures(
            r,
            &NodeMaterial::new(crate::tsl::Texture::External(0).sample(uv())).wgsl(1)?,
            &[],
            &[(&placeholder.create_view(&Default::default()), &sampler)],
        )
        .await?;
        let mut demo = Self {
            container,
            registration,
            video_mesh,
            face: None,
            video,
            stream: None,
            program: Some(program),
            sampler,
            aspect: None,
            pending: false,
            changes: vec![],
        };
        demo.load(s).await?;
        Ok(demo)
    }

    /// The GLTFLoader callback: grp_transform into the registration, normal
    /// materials on the head ( mesh_2 ) and teeth ( mesh_3 ), the eyes.
    async fn load(&mut self, s: &mut Scene) -> Result<()> {
        let bytes = super::gltf_viewer::fetch(MODEL).await?;
        let base = MODEL.rsplit_once('/').ok_or(Error::Invalid("model URL"))?.0;
        let (asset, buffers, images) = super::gltf_viewer::load_asset_bytes(&bytes, base).await?;
        // GLTFLoader's morphTargetDictionary from the head mesh's extras.targetNames.
        let json =
            gltf::binary::Glb::from_slice(&bytes).map_err(|e| Error::Asset(e.to_string()))?;
        let json: serde_json::Value =
            serde_json::from_slice(&json.json).map_err(|e| Error::Asset(e.to_string()))?;
        let targets = json["meshes"][2]["extras"]["targetNames"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect();
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        let named = |name: &str| -> Result<Object3D> {
            instance
                .nodes
                .iter()
                .copied()
                .find(|&h| s.get(h).is_ok_and(|n| n.name == name))
                .ok_or(Error::Invalid("facecap node"))
        };
        let group = named("grp_transform")?;
        let eyes = [named("eyeLeft")?, named("eyeRight")?];
        s.add(self.registration, group)?;
        let mut head = None;
        for (&h, &(mesh, _)) in instance.meshes.iter().zip(&instance.sources) {
            if mesh == 2 || mesh == 3 {
                if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                    m.materials = vec![Arc::new(Material::Normal(MeshNormalMaterial::default()))];
                }
                if mesh == 2 {
                    head = Some(h);
                }
            }
        }
        let mut eye_state = vec![];
        for e in eyes {
            let n = s.get_mut(e)?;
            let (scale, rotation, position) = n.matrix.to_scale_rotation_translation();
            n.scale = scale;
            n.quaternion = rotation;
            n.position = position;
            n.matrix_auto_update = true;
            eye_state.push((e, euler(rotation)));
        }
        self.face = Some(Face {
            head: head.ok_or(Error::Invalid("facecap head"))?,
            eyes: [eye_state[0], eye_state[1]],
            targets,
        });
        Ok(())
    }

    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending = true;
        }
        Ok(())
    }

    /// The loadedmetadata handler, the video frame upload and animate().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let Some(video) = self.video.clone() else {
            return Ok(());
        };
        if self.aspect.is_none() && video.ready_state() >= 1 && video.video_width() > 0 {
            let aspect = video.video_width() as f64 / video.video_height() as f64;
            self.aspect = Some(aspect);
            let height = 2. * VIDEO_DISTANCE * (MP_FOV / 2.).to_radians().tan();
            s.get_mut(self.video_mesh)?.scale = Vector3::new(height * aspect, height, 1.);
        }
        if let (Some(aspect), NodeKind::Camera(Camera::Perspective(p))) =
            (self.aspect, &mut s.get_mut(c)?.kind)
        {
            p.aspect = aspect;
        }
        if let Some(face) = &self.face {
            let n = s.get_mut(face.head)?;
            n.morph_weights.resize(face.targets.len(), 0.);
            for (index, value) in self.changes.drain(..) {
                if let Some(w) = n.morph_weights.get_mut(index) {
                    *w = value;
                }
            }
        }
        self.upload(s, r, &video)?;
        if std::mem::take(&mut self.pending) && self.face.is_some() && video.ready_state() >= 1 {
            self.detect(s)?;
        }
        // The GUI's listened influences.
        if let Some(face) = &self.face
            && let Some(body) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
        {
            let weights = s
                .get(face.head)?
                .morph_weights
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(",");
            let _ = body.set_attribute("data-morph", &weights);
        }
        Ok(())
    }

    /// VideoTexture: each new video frame copied once.
    fn upload(
        &mut self,
        s: &mut Scene,
        r: &Renderer,
        video: &web_sys::HtmlVideoElement,
    ) -> Result<()> {
        if video.ready_state() < 2 || video.video_width() == 0 {
            return Ok(());
        }
        let size = wgpu::Extent3d {
            width: video.video_width(),
            height: video.video_height(),
            depth_or_array_layers: 1,
        };
        if let Some(mut program) = self.program.take() {
            let texture = video_texture(r, size);
            program.rebind(
                r,
                &[],
                &[(&texture.create_view(&Default::default()), &self.sampler)],
            )?;
            let mut material = ShaderMaterial::new(Arc::new(program));
            material.properties.depth_test = false;
            material.properties.depth_write = false;
            if let NodeKind::Mesh(m) = &mut s.get_mut(self.video_mesh)?.kind {
                m.materials[0] = Arc::new(Material::Shader(material));
            }
            self.stream = Some(Stream {
                texture,
                uploaded: None,
            });
        }
        let k = self.stream.as_mut().ok_or(Error::Invalid("video stream"))?;
        let time = video.current_time();
        if k.uploaded != Some(time) && k.texture.size() == size {
            r.queue.copy_external_image_to_texture(
                &wgpu::CopyExternalImageSourceInfo {
                    source: wgpu::ExternalImageSource::HTMLVideoElement(video.clone()),
                    origin: wgpu::Origin2d::ZERO,
                    flip_y: true,
                },
                wgpu::CopyExternalImageDestInfo {
                    texture: &k.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                    color_space: wgpu::PredefinedColorSpace::Srgb,
                    premultiplied_alpha: false,
                },
                size,
            );
            k.uploaded = Some(time);
        }
        Ok(())
    }

    /// faceLandmarker.detectForVideo through the page's JavaScript, then the
    /// pose matrix, the morph influences and the eye rotations.
    fn detect(&mut self, s: &mut Scene) -> Result<()> {
        let Some(window) = web_sys::window() else {
            return Ok(());
        };
        let Ok(detect) = js_sys::Reflect::get(&window, &"galleryFaceDetect".into()) else {
            return Ok(());
        };
        let Some(detect) = detect.dyn_ref::<js_sys::Function>() else {
            return Ok(());
        };
        let results = detect
            .call0(&window)
            .map_err(|e| Error::Asset(format!("{e:?}")))?;
        if results.is_null() || results.is_undefined() {
            return Ok(());
        }
        let get = |k: &str| {
            js_sys::Reflect::get(&results, &k.into()).unwrap_or(wasm_bindgen::JsValue::NULL)
        };
        let matrix = get("matrix");
        if !matrix.is_null() && !matrix.is_undefined() {
            let data: Vec<f64> = js_sys::Array::from(&matrix)
                .iter()
                .filter_map(|v| v.as_f64())
                .collect();
            if data.len() == 16 {
                let n = s.get_mut(self.container)?;
                n.matrix = Matrix4::from_cols_slice(&data);
                n.matrix_world_needs_update = true;
            }
        }
        let names = get("names");
        if names.is_null() || names.is_undefined() {
            return Ok(());
        }
        let names: Vec<String> = js_sys::Array::from(&names)
            .iter()
            .filter_map(|v| v.as_string())
            .collect();
        let scores: Vec<f64> = js_sys::Array::from(&get("scores"))
            .iter()
            .filter_map(|v| v.as_f64())
            .collect();
        let face = self.face.as_ref().ok_or(Error::Invalid("face"))?;
        let mut weights = s.get(face.head)?.morph_weights.clone();
        weights.resize(face.targets.len(), 0.);
        let (mut left_h, mut right_h, mut left_v, mut right_v) = (0., 0., 0., 0.);
        for (name, &score) in names.iter().zip(&scores) {
            if let Some((_, target)) = BLENDSHAPES.iter().find(|(k, _)| k == name)
                && let Some(index) = face.targets.iter().position(|t| t == target)
            {
                weights[index] = score;
            }
            match name.as_str() {
                "eyeLookInLeft" => left_h += score,
                "eyeLookOutLeft" => left_h -= score,
                "eyeLookInRight" => right_h -= score,
                "eyeLookOutRight" => right_h += score,
                "eyeLookUpLeft" => left_v -= score,
                "eyeLookDownLeft" => left_v += score,
                "eyeLookUpRight" => right_v -= score,
                "eyeLookDownRight" => right_v += score,
                _ => {}
            }
        }
        s.get_mut(face.head)?.morph_weights = weights;
        let limit = 30f64.to_radians();
        for ((eye, rest), (h, v)) in face.eyes.iter().zip([(left_h, left_v), (right_h, right_v)]) {
            s.get_mut(*eye)?.quaternion = quaternion([v * limit, rest[1], h * limit]);
        }
        Ok(())
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
    /// The GUI's influence sliders, in morphTargetDictionary order.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index >= 52 {
            return Err(Error::Invalid("morph influence"));
        }
        self.changes.push((index, value as f64));
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}

fn video_texture(r: &Renderer, size: wgpu::Extent3d) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("webcam face texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}
