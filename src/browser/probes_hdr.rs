//! The cube-camera light probe, HDR/LDR/generated environment maps, UltraHDR
//! gain-map environments, the transmissive sphere and the dungeon scene.
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::gltf_viewer::{fetch, load_asset};
use super::lights_probes::{cube_background, cube_sh, pisa_faces, pisa_hdr, sh_call};
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, float, vec4};
use crate::{
    Error, Result, camera::*, environment::EnvironmentMap, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

const ASSETS: &str = "/web/gallery/assets";
const PROJECT: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";

/// UltraHDRLoader metadata from the gain map's XMP (`hdrgm:` attributes).
#[derive(Default)]
struct GainMap {
    version: bool,
    gain_min: f64,
    gain_max: f64,
    gamma: f64,
    offset_sdr: f64,
    offset_hdr: f64,
    capacity_min: f64,
    capacity_max: f64,
}
/// A JPEG decoded by the browser and drawn to a 2D canvas at `size`, as the
/// loader's `drawImage` / `getImageData` do (the gain map is scaled by it).
async fn canvas_pixels(bytes: &[u8], size: Option<(u32, u32)>) -> Result<(u32, u32, Vec<u8>)> {
    let fail = |e: wasm_bindgen::JsValue| Error::Asset(format!("UltraHDR image: {e:?}"));
    let cast = |_| Error::Invalid("UltraHDR image type");
    let parts = js_sys::Array::new();
    parts.push(&js_sys::Uint8Array::from(bytes));
    let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).map_err(fail)?;
    let window = web_sys::window().ok_or(Error::Invalid("window"))?;
    let bitmap: web_sys::ImageBitmap = wasm_bindgen_futures::JsFuture::from(
        window.create_image_bitmap_with_blob(&blob).map_err(fail)?,
    )
    .await
    .map_err(fail)?
    .dyn_into()
    .map_err(|_| Error::Invalid("UltraHDR bitmap"))?;
    let (width, height) = size.unwrap_or((bitmap.width(), bitmap.height()));
    let document = window.document().ok_or(Error::Invalid("document"))?;
    let canvas: web_sys::HtmlCanvasElement = document
        .create_element("canvas")
        .map_err(fail)?
        .dyn_into()
        .map_err(cast)?;
    canvas.set_width(width);
    canvas.set_height(height);
    let attributes = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&attributes, &"willReadFrequently".into(), &true.into());
    let _ = js_sys::Reflect::set(&attributes, &"colorSpace".into(), &"srgb".into());
    let context: web_sys::CanvasRenderingContext2d = canvas
        .get_context_with_context_options("2d", &attributes)
        .map_err(fail)?
        .ok_or(Error::Invalid("2d context"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("2d context"))?;
    context
        .draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &bitmap,
            0.,
            0.,
            bitmap.width() as f64,
            bitmap.height() as f64,
            0.,
            0.,
            width as f64,
            height as f64,
        )
        .map_err(fail)?;
    let data = context
        .get_image_data(0., 0., width as f64, height as f64)
        .map_err(fail)?
        .data()
        .0;
    Ok((width, height, data))
}
/// UltraHDRLoader.parse: the MPF primary and gain-map JPEGs, the gain-map XMP,
/// and the HDR recovery formula with the loader's sRGB table, stored with
/// `DataUtils.toHalfFloat` (HalfFloatType, flipY: rows are top first).
async fn ultra_hdr(bytes: &[u8]) -> Result<EnvironmentMap> {
    let mut meta = GainMap::default();
    let (mut primary, mut gainmap) = (None, None);
    let mut offset = 0;
    let bad = || Error::Asset("UltraHDR: truncated section".into());
    while offset + 1 < bytes.len() {
        if bytes[offset] != 0xff {
            offset += 1;
            continue;
        }
        let marker = bytes[offset + 1];
        if marker == 0xd8 {
            offset += 2;
            continue;
        }
        let length = || -> Result<usize> {
            Ok(((*bytes.get(offset + 2).ok_or_else(bad)? as usize) << 8)
                | *bytes.get(offset + 3).ok_or_else(bad)? as usize)
        };
        if matches!(marker, 0xe0..=0xe2) {
            let end = (offset + 2 + length()?).min(bytes.len());
            let section = &bytes[offset..end];
            let section_offset = offset + 2;
            if marker == 0xe1 {
                let text = String::from_utf8_lossy(section);
                if !text.contains("Container:Directory") && text.contains("hdrgm:Version") {
                    let attribute = |name: &str| -> Option<f64> {
                        let key = format!("hdrgm:{name}=\"");
                        let start = text.find(&key)? + key.len();
                        let end = start + text[start..].find('"')?;
                        text[start..end].parse().ok()
                    };
                    meta.version = true;
                    meta.gain_min = attribute("GainMapMin").unwrap_or(0.);
                    meta.gain_max = attribute("GainMapMax").unwrap_or(1.);
                    meta.gamma = attribute("Gamma").unwrap_or(1.);
                    meta.offset_sdr = attribute("OffsetSDR").unwrap_or(f64::NAN) / (1. / 64.);
                    meta.offset_hdr = attribute("OffsetHDR").unwrap_or(f64::NAN) / (1. / 64.);
                    meta.capacity_min = attribute("HDRCapacityMin").unwrap_or(0.);
                    meta.capacity_max = attribute("HDRCapacityMax").unwrap_or(1.);
                }
            } else if marker == 0xe2 && section.len() >= 8 && section[4..8] == [0x4d, 0x50, 0x46, 0]
            {
                // MPF: primary and gain-map sizes and offsets after 60 bytes of tags.
                // Offsets are relative to the loader's DataView, two bytes into the section.
                let data = &section[2..];
                let little = data.get(6..10) == Some(&[0x49, 0x49, 0x2a, 0][..]);
                let u32_at = |i: usize| -> Result<usize> {
                    let b: [u8; 4] = data
                        .get(i..i + 4)
                        .ok_or_else(bad)?
                        .try_into()
                        .map_err(|_| bad())?;
                    Ok(if little {
                        u32::from_le_bytes(b)
                    } else {
                        u32::from_be_bytes(b)
                    } as usize)
                };
                let (primary_size, primary_offset) = (u32_at(60)?, u32_at(64)?);
                let (gain_size, gain_offset) = (u32_at(76)?, u32_at(80)? + section_offset + 6);
                primary = bytes.get(primary_offset..primary_offset + primary_size);
                gainmap = bytes.get(gain_offset..(gain_offset + gain_size).min(bytes.len()));
            }
            offset = end;
            continue;
        }
        if marker >= 0xc0 && marker != 0xd9 && !(0xd0..=0xd7).contains(&marker) && marker != 0xff {
            offset += 2 + length()?;
            continue;
        }
        offset += 2;
    }
    if !meta.version {
        return Err(Error::Asset("UltraHDR: not a valid UltraHDR image".into()));
    }
    let (primary, gainmap) = primary
        .zip(gainmap)
        .ok_or_else(|| Error::Asset("UltraHDR: could not parse images".into()))?;
    let (width, height, sdr) = canvas_pixels(primary, None).await?;
    let (_, _, gain) = canvas_pixels(gainmap, Some((width, height))).await?;
    let max_boost = 1.8f64.powf(meta.capacity_max * 0.5);
    let weight = ((max_boost.log2() - meta.capacity_min) / (meta.capacity_max - meta.capacity_min))
        .clamp(0., 1.);
    let srgb_to_linear = |v: f64| {
        if v < 10.31475 {
            v * 0.000303527
        } else if v < 1024. {
            (((v as i64) as f64) * 0.003717127 + 0.0521327014).powf(2.4)
        } else {
            (v * 0.003717127 + 0.0521327014).powf(2.4)
        }
    };
    let mut rgba = vec![half::f16::from_bits(15360); sdr.len()];
    for i in (0..sdr.len()).step_by(4) {
        for c in 0..3 {
            let g = gain[i + c] as f64 * 0.00392156862745098;
            let recovery = if meta.gamma == 1. {
                g
            } else {
                g.powf(1. / meta.gamma)
            };
            let boost = meta.gain_min + (meta.gain_max - meta.gain_min) * recovery;
            let factor = if boost * weight == 0. {
                1.
            } else {
                2f64.powf(boost * weight)
            };
            let hdr = (sdr[i + c] as f64 + meta.offset_sdr) * factor - meta.offset_hdr;
            let linear = srgb_to_linear(hdr).clamp(0., 65504.);
            rgba[i + c] = half::f16::from_bits(super::refraction_loaders::formats::to_half_float(
                linear as f32,
            ));
        }
    }
    Ok(EnvironmentMap {
        width,
        height,
        rgba,
        gpu: None,
    })
}

/// The LightProbeHelper sphere: SH irradiance in uniforms 0..8, intensity in 9.
async fn probe_helper(
    s: &mut Scene,
    r: &Renderer,
    sh: &[Vector3; 9],
    size: f64,
) -> Result<Object3D> {
    let color = vec4(sh_call(Some(9))?, float(1.));
    let program =
        ShaderProgram::with_projection(r, &NodeMaterial::new(color).wgsl(0)?, &[], &[], PROJECT)
            .await?;
    let mut m = ShaderMaterial::new(Arc::new(program));
    for (i, c) in sh.iter().enumerate() {
        m.uniforms[i] = [c.x as f32, c.y as f32, c.z as f32, 0.];
    }
    m.uniforms[9] = [1., 0., 0., 0.];
    let helper = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(SphereGeometry::build(1., 32, 16)?),
        Arc::new(Material::Shader(m)),
    )));
    s.get_mut(helper)?.scale = Vector3::splat(size);
    Ok(helper)
}

/// webgl_materials_envmaps_hdr: the three PMREMs, the two raw cube backgrounds
/// and the debug plane showing the chosen atlas.
struct Envmaps {
    torus: Object3D,
    plane: Object3D,
    /// Generated, LDR, HDR.
    environments: [Arc<EnvironmentMap>; 3],
    backgrounds: [Object3D; 2],
    /// envMap, roughness, metalness, exposure, debug.
    params: [f64; 5],
    rotation: f64,
    planes: [Arc<Material>; 3],
}
/// The physical torus knot or sphere with its roughness/metalness controls.
struct Hdr {
    mesh: Object3D,
    /// autoRotate, metalness, roughness, exposure, resolution (0 2k, 1 4k), type.
    params: [f64; 6],
    rotation: f64,
    /// The resolution whose environment is shown.
    loaded: usize,
    /// Environments by resolution; a load in flight fills its slot.
    environments: [std::rc::Rc<std::cell::RefCell<Option<Arc<EnvironmentMap>>>>; 2],
    requested: [bool; 2],
}
/// webgpu_materials_transmission's material controls.
struct Transmission {
    mesh: Object3D,
    /// color, transmission, opacity, metalness, roughness, ior, thickness,
    /// specularIntensity, specularColor, envMapIntensity, exposure.
    params: [f64; 11],
}
pub struct Demo {
    id: u32,
    time: f64,
    last: f64,
    controls: Option<Controls>,
    envmaps: Option<Envmaps>,
    hdr: Option<Hdr>,
    transmission: Option<Transmission>,
    model: Vec<Object3D>,
    /// webgpu_performance's `static` control, applied before the next frame.
    static_meshes: bool,
    /// The cube background, kept centered on the camera.
    sky: Option<Object3D>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            283 => (40., 1., 1000., Vector3::new(0., 0., 30.)),
            284 => (40., 1., 1000., Vector3::new(0., 0., 120.)),
            285 => (50., 1., 500., Vector3::new(0., 0., -6.)),
            286 => (40., 1., 2000., Vector3::new(0., 0., 120.)),
            _ => (45., 0.1, 100., Vector3::new(60., 60., 60.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = position;
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            controls: None,
            envmaps: None,
            hdr: None,
            transmission: None,
            model: vec![],
            static_meshes: true,
            sky: None,
        };
        match id {
            283 => d.cube_camera_scene(s, c, r).await?,
            284 => d.envmaps_scene(s, c, r).await?,
            285 => d.ultrahdr_scene(s, c).await?,
            286 => d.transmission_scene(s, c).await?,
            _ => d.performance_scene(s, c).await?,
        }
        Ok(d)
    }
    /// The pisa background captured by a 256² CubeCamera into an 8-bit
    /// NoColorSpace cube, then LightProbeGenerator.fromCubeRenderTarget. The
    /// target's texels are the faces' texels (same layout, 256² each), stored as
    /// 8-bit linear values; the SH is computed from them on the CPU, once.
    async fn cube_camera_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let faces = pisa_faces().await?;
        let sh = cube_sh(&faces, true);
        let cube = crate::texture_gpu::GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        self.sky = Some(cube_background(s, r, &cube).await?);
        probe_helper(s, r, &sh, 5.).await?;
        let mut controls = Controls::new(None, (10., 50.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        Ok(())
    }
    async fn envmaps_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let torus = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(18., 8., 150, 20, 2, 3)?),
            Arc::new(Material::Standard(MeshStandardMaterial {
                metalness: 0.,
                roughness: 0.,
                ..Default::default()
            })),
        )));
        // DebugEnvironment: a back-sided room, a point light and three emissive panels.
        let mut debug = Scene::new();
        debug.background = Color::BLACK;
        let mut box_geometry = BoxGeometry::build(1., 1., 1.)?;
        box_geometry.delete_attribute("uv");
        let box_geometry = Arc::new(box_geometry);
        let mut room = MeshStandardMaterial {
            metalness: 0.,
            ..Default::default()
        };
        room.properties.side = Side::Back;
        let h = debug.insert(NodeKind::Mesh(Mesh::new(
            box_geometry.clone(),
            Arc::new(Material::Standard(room)),
        )));
        debug.get_mut(h)?.scale = Vector3::splat(10.);
        debug.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 50.,
            distance: 0.,
            decay: 2.,
        }));
        for (hex, position, scale) in [
            (
                0xff0000,
                Vector3::new(-5., 2., 0.),
                Vector3::new(0.1, 1., 1.),
            ),
            (
                0x00ff00,
                Vector3::new(0., 5., 0.),
                Vector3::new(1., 0.1, 1.),
            ),
            (
                0x0000ff,
                Vector3::new(2., 1., 5.),
                Vector3::new(1.5, 2., 0.1),
            ),
        ] {
            let mut m = MeshLambertMaterial {
                emissive: Color(Vector3::splat(10.)),
                ..Default::default()
            };
            m.properties.color = Color::from_hex(hex);
            let h = debug.insert(NodeKind::Mesh(Mesh::new(
                box_geometry.clone(),
                Arc::new(Material::Lambert(m)),
            )));
            let n = debug.get_mut(h)?;
            n.position = position;
            n.scale = scale;
        }
        let generated = EnvironmentMap::from_scene(r, &mut debug, 256, 0.)?;
        let faces = pisa_faces().await?;
        let ldr_cube = crate::texture_gpu::GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        let ldr = EnvironmentMap::from_cube_texture(r, &ldr_cube)?;
        let hdr_cube = pisa_hdr(r).await?;
        let hdr = EnvironmentMap::from_cube_texture(r, &hdr_cube)?;
        // WebGL draws cube backgrounds on a unit box.
        let unit_box = Arc::new(BoxGeometry::build(1., 1., 1.)?);
        let ldr_background = cube_background(s, r, &ldr_cube).await?;
        let hdr_background = cube_background(s, r, &hdr_cube).await?;
        for h in [ldr_background, hdr_background] {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                m.geometry = unit_box.clone();
                // WebGLBackground tone-maps a cube background unless it is sRGB.
                if h == ldr_background {
                    Arc::make_mut(&mut m.materials[0])
                        .properties_mut()
                        .tone_mapped = false;
                }
            }
        }
        let environments = [Arc::new(generated), Arc::new(ldr), Arc::new(hdr)];
        // The debug plane maps the chosen PMREM atlas in three's texel layout: the
        // port's atlas stores each face region upside down with ±Y exchanged.
        let mut planes = vec![];
        for env in &environments {
            let gpu = env.gpu.as_ref().ok_or(Error::Invalid("PMREM"))?;
            let atlas = crate::tsl::WgslFn::new(
                "three_atlas",
                ATLAS,
                &[crate::tsl::Type::Vec2],
                crate::tsl::Type::Vec4,
            )?
            .call(&[crate::tsl::uv()]);
            let program = ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(atlas).wgsl(1)?,
                &[],
                &[(gpu.view(), gpu.sampler())],
                PROJECT,
            )
            .await?;
            planes.push(Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(
                program,
            )))));
        }
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(200., 200., 1, 1)?),
            planes[2].clone(),
        )));
        let n = s.get_mut(plane)?;
        n.position.y = -50.;
        n.quaternion = Quaternion::from_rotation_x(-PI * 0.5);
        n.visible = false;
        let mut controls = Controls::new(None, (50., 300.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.envmaps = Some(Envmaps {
            torus,
            plane,
            environments,
            backgrounds: [ldr_background, hdr_background],
            params: [2., 0., 0., 1., 0.],
            rotation: 0.,
            planes: planes.try_into().map_err(|_| Error::Invalid("planes"))?,
        });
        Ok(())
    }
    async fn ultrahdr_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let env = Arc::new(
            ultra_hdr(&fetch(&format!("{ASSETS}/probes-hdr/spruit_sunrise_2k.hdr.jpg")).await?)
                .await?,
        );
        s.environment = Some(env.clone());
        s.background_environment = true;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(1., 0.4, 128, 128, 1, 3)?),
            Arc::new(Material::Standard(MeshStandardMaterial {
                metalness: 1.,
                roughness: 0.,
                ..Default::default()
            })),
        )));
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.hdr = Some(Hdr {
            mesh,
            params: [1., 1., 0., 1., 0., 0.],
            rotation: 0.,
            loaded: 0,
            environments: [
                std::rc::Rc::new(std::cell::RefCell::new(Some(env))),
                Default::default(),
            ],
            requested: [true, false],
        });
        Ok(())
    }
    async fn transmission_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let env =
            ultra_hdr(&fetch(&format!("{ASSETS}/probes-hdr/royal_esplanade_2k.hdr.jpg")).await?)
                .await?;
        s.environment = Some(Arc::new(env));
        s.background_environment = true;
        // generateTexture(): a 2 × 2 canvas, white on the bottom row. alphaMap reads
        // its green channel: a white map with that alpha carries the same values.
        let mut rgba = vec![255, 255, 255, 0, 255, 255, 255, 0];
        rgba.extend([255u8; 8]);
        let mut alpha = Texture::from_rgba(2, 2, rgba, false)?;
        alpha.wrap_s = Wrapping::Repeat;
        alpha.wrap_t = Wrapping::Repeat;
        alpha.repeat = Vector2::new(1., 3.5);
        alpha.filter = Filter::Nearest;
        alpha.min_filter = Some(Filter::Linear);
        alpha.mipmap_filter = Some(Filter::Linear);
        let mut m = MeshPhysicalMaterial::default();
        m.base.metalness = 0.;
        m.base.roughness = 0.;
        m.base.energy_conservation = true;
        m.ior = 1.5;
        m.transmission = 1.;
        m.thickness = 0.01;
        m.specular_intensity = 1.;
        m.base.properties.map = Some(Arc::new(alpha));
        m.base.properties.side = Side::Double;
        m.base.properties.transparent = true;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(20., 64, 32)?),
            Arc::new(Material::Physical(m)),
        )));
        let mut controls = Controls::new(None, (10., 150.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.transmission = Some(Transmission {
            mesh,
            params: [
                f64::from(0xffffff),
                1.,
                1.,
                0.,
                0.,
                1.5,
                0.01,
                1.,
                f64::from(0xffffff),
                1.,
                1.,
            ],
        });
        Ok(())
    }
    async fn performance_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let env =
            ultra_hdr(&fetch(&format!("{ASSETS}/probes-hdr/royal_esplanade_2k.hdr.jpg")).await?)
                .await?;
        s.environment = Some(Arc::new(env));
        let (asset, buffers, images) =
            load_asset(&format!("{ASSETS}/probes-hdr/dungeon_warkarma.glb")).await?;
        self.model = crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        for &h in &self.model {
            if let NodeKind::Mesh(mesh) = &mut s.get_mut(h)?.kind {
                for m in &mut mesh.materials {
                    match Arc::make_mut(m) {
                        Material::Standard(m) => m.energy_conservation = true,
                        Material::Physical(m) => m.base.energy_conservation = true,
                        _ => {}
                    }
                }
            }
        }
        let mut controls = Controls::new(None, (2., 60.), PI, true);
        controls.set_target(Vector3::new(0., 0., -0.2));
        controls.update(s, c)?;
        self.controls = Some(controls);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        if let Some(sky) = self.sky {
            let p = s.get(c)?.position;
            s.get_mut(sky)?.position = p;
        }
        match self.id {
            284 => {
                let k = self.envmaps.as_mut().ok_or(Error::Invalid("envmaps"))?;
                let [env, roughness, metalness, exposure, debug] = k.params;
                let env = env as usize;
                for _ in 0..steps {
                    k.rotation += 0.005;
                }
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(k.torus)?.kind
                    && let Material::Standard(current) = mesh.materials[0].as_ref()
                    && (current.roughness, current.metalness) != (roughness, metalness)
                    && let Material::Standard(m) = Arc::make_mut(&mut mesh.materials[0])
                {
                    m.roughness = roughness;
                    m.metalness = metalness;
                }
                s.get_mut(k.torus)?.quaternion = Quaternion::from_rotation_y(k.rotation);
                if s.environment
                    .as_ref()
                    .is_none_or(|e| !Arc::ptr_eq(e, &k.environments[env]))
                {
                    s.environment = Some(k.environments[env].clone());
                    if let NodeKind::Mesh(m) = &mut s.get_mut(k.plane)?.kind {
                        m.materials[0] = k.planes[env].clone();
                    }
                }
                s.get_mut(k.plane)?.visible = debug > 0.5;
                // Generated uses the PMREM itself as the background; LDR and HDR the cubes.
                s.background_environment = env == 0;
                s.background_pmrem = env == 0;
                s.get_mut(k.backgrounds[0])?.visible = env == 1;
                s.get_mut(k.backgrounds[1])?.visible = env == 2;
                s.exposure = exposure;
                let p = s.get(c)?.position;
                for b in k.backgrounds {
                    s.get_mut(b)?.position = p;
                }
            }
            285 => {
                let k = self.hdr.as_mut().ok_or(Error::Invalid("hdr"))?;
                let [auto, metalness, roughness, exposure, _, _] = k.params;
                if auto > 0.5 {
                    for _ in 0..steps {
                        k.rotation += 0.005;
                    }
                }
                s.get_mut(k.mesh)?.quaternion = Quaternion::from_rotation_y(k.rotation);
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(k.mesh)?.kind
                    && let Material::Standard(current) = mesh.materials[0].as_ref()
                    && (current.roughness, current.metalness) != (roughness, metalness)
                    && let Material::Standard(m) = Arc::make_mut(&mut mesh.materials[0])
                {
                    m.roughness = roughness;
                    m.metalness = metalness;
                }
                // loadEnvironment( resolution ): the new texture replaces the old once loaded.
                let index = k.params[4] as usize;
                if !k.requested[index] {
                    k.requested[index] = true;
                    let slot = k.environments[index].clone();
                    let name = ["spruit_sunrise_2k", "spruit_sunrise_4k"][index];
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(bytes) =
                            fetch(&format!("{ASSETS}/probes-hdr/{name}.hdr.jpg")).await
                            && let Ok(env) = ultra_hdr(&bytes).await
                        {
                            *slot.borrow_mut() = Some(Arc::new(env));
                        }
                    });
                }
                if k.environments[index].borrow().is_some() {
                    k.loaded = index;
                }
                if let Some(env) = k.environments[k.loaded].borrow().as_ref()
                    && s.environment.as_ref().is_none_or(|e| !Arc::ptr_eq(e, env))
                {
                    s.environment = Some(env.clone());
                }
                s.exposure = exposure;
                if let Some(controls) = &mut self.controls {
                    controls.frame_update(s, c)?;
                }
            }
            286 => {
                let k = self
                    .transmission
                    .as_ref()
                    .ok_or(Error::Invalid("transmission"))?;
                let p = k.params;
                s.exposure = p[10];
                s.environment_intensity = p[9];
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(k.mesh)?.kind
                    && let Material::Physical(current) = mesh.materials[0].as_ref()
                {
                    let color = Color::from_hex(p[0] as u32);
                    let specular = Color::from_hex(p[8] as u32);
                    let changed = current.base.properties.color != color
                        || current.transmission != p[1]
                        || current.base.properties.opacity != p[2]
                        || current.base.metalness != p[3]
                        || current.base.roughness != p[4]
                        || current.ior != p[5]
                        || current.thickness != p[6]
                        || current.specular_intensity != p[7]
                        || current.specular_color != specular;
                    if changed && let Material::Physical(m) = Arc::make_mut(&mut mesh.materials[0])
                    {
                        m.base.properties.color = color;
                        m.transmission = p[1];
                        m.base.properties.opacity = p[2];
                        m.base.metalness = p[3];
                        m.base.roughness = p[4];
                        m.ior = p[5];
                        m.thickness = p[6];
                        m.specular_intensity = p[7];
                        m.specular_color = specular;
                    }
                }
            }
            _ => {
                // setStatic(): no visual change; static meshes skip per-frame updates.
                for &h in &self.model {
                    s.get_mut(h)?.is_static = self.static_meshes;
                }
            }
        }
        Ok(())
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
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            if self.id == 283 {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        match self.id {
            284 if index < 5 => {
                self.envmaps
                    .as_mut()
                    .ok_or(Error::Invalid("envmaps"))?
                    .params[index] = v
            }
            285 if index < 6 => self.hdr.as_mut().ok_or(Error::Invalid("hdr"))?.params[index] = v,
            286 if index < 11 => {
                self.transmission
                    .as_mut()
                    .ok_or(Error::Invalid("transmission"))?
                    .params[index] = v
            }
            287 if index == 0 => self.static_meshes = v > 0.5,
            _ => return Err(Error::Invalid("probes/hdr parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// Sample the port's PMREM atlas at a uv of three's atlas (GL rows, bottom up):
/// each face region is mirrored vertically and faces 1 and 4 trade places.
const ATLAS: &str = "fn three_atlas(uv:vec2<f32>)->vec4<f32>{
 let size=vec2<f32>(textureDimensions(tsl_texture_0));let max_mip=log2(size.y/4.0);
 let p=uv*size;var q=p;
 for(var level=i32(max_mip);level>=-2;level--){
  let extra=f32(max(4-level,0));let s=exp2(f32(max(level,4)));
  let x0=extra*48.0;let y0=4.0*(exp2(max_mip)-s);
  if p.y>=y0 && p.y<y0+2.0*s && p.x>=x0 && p.x<x0+3.0*s {
   let col=floor((p.x-x0)/s);let row=floor((p.y-y0)/s);var face=u32(col+3.0*row);
   if face==1u {face=4u;} else if face==4u {face=1u;}
   let local=vec2(p.x-x0-col*s,p.y-y0-row*s);
   q=vec2(x0+f32(face%3u)*s+local.x,y0+select(0.0,s,face>2u)+(s-local.y));
   break;
  }
 }
 return vec4(textureSampleLevel(tsl_texture_0,tsl_sampler_0,q/size,0.0).rgb,1.0);
}";
