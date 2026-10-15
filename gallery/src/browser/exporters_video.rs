//! The EXR and KTX2 exporters, the video cube wall, the async-compiled noise
//! materials and the object-space normal map.
pub(super) mod formats;

use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::{fetch, load_asset};
use crate::shader::ShaderProgram;
use crate::tsl::surface::SurfaceNodes;
use crate::tsl::{NodeMaterial, Type, WgslFn, float, uv, vec3, vec4};
use crate::{
    Error, Result, camera::*, environment::EnvironmentMap, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use formats::{Ktx2Kind, Pixels};
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

const ASSETS: &str = "/web/gallery/assets";
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}

/// One asynchronous copy of a PMREM atlas to a mappable buffer, as
/// `readRenderTargetPixelsAsync` does for the exporters.
struct AtlasReadback {
    buffer: wgpu::Buffer,
    stride: u32,
    width: u32,
    height: u32,
    ready: Arc<Mutex<Option<std::result::Result<(), wgpu::BufferAsyncError>>>>,
}
impl AtlasReadback {
    fn begin(r: &Renderer, texture: &wgpu::Texture) -> Self {
        let (width, height) = (texture.width(), texture.height());
        let stride = (width * 8).div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT)
            * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("PMREM export readback"),
            size: stride as u64 * height as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(stride),
                    rows_per_image: Some(height),
                },
            },
            texture.size(),
        );
        r.queue.submit([encoder.finish()]);
        let ready = Arc::new(Mutex::new(None));
        let signal = ready.clone();
        buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                *signal.lock().unwrap() = Some(result);
            });
        Self {
            buffer,
            stride,
            width,
            height,
            ready,
        }
    }
    /// The half-float texels, once mapped. The atlas rows are stored in the same
    /// order as WebGL's bottom-up readback of three's cube-UV target.
    fn take(&self) -> Option<Result<Vec<u16>>> {
        let result = self.ready.lock().unwrap().take()?;
        if result.is_err() {
            return Some(Err(Error::Invalid("PMREM readback")));
        }
        let mapped = self.buffer.slice(..).get_mapped_range();
        let row = self.width as usize * 4;
        let mut texels = Vec::with_capacity(row * self.height as usize);
        for y in 0..self.height as usize {
            let start = y * self.stride as usize;
            texels.extend_from_slice(bytemuck::cast_slice(&mapped[start..start + row * 2]));
        }
        drop(mapped);
        self.buffer.unmap();
        Some(Ok(texels))
    }
}

/// Reorder a readback of the port's PMREM atlas into three's cube-UV texels. The
/// port samples its atlas with the direction's y negated (`cube_uv.wgsl`), so each
/// face region is stored upside down and the ±Y faces trade places.
fn three_cube_uv(texels: &[u16], width: u32, height: u32) -> Vec<u16> {
    let (width, height) = (width as usize, height as usize);
    let max_mip = (height / 4).ilog2() as i32;
    let mut out = texels.to_vec();
    // Resolution levels max_mip..=4, then six extra 16-texel levels offset in x.
    for level in (-2..=max_mip).rev() {
        let extra = (4 - level).max(0) as usize;
        let size = 1usize << level.max(4);
        let (x0, y0) = (extra * 48, 4 * ((1usize << max_mip) - size));
        for face in 0..6usize {
            let target = match face {
                1 => 4,
                4 => 1,
                f => f,
            };
            let origin = |f: usize| (x0 + (f % 3) * size, y0 + if f > 2 { size } else { 0 });
            let ((sx, sy), (dx, dy)) = (origin(face), origin(target));
            for row in 0..size {
                let from = ((sy + size - 1 - row) * width + sx) * 4;
                let to = ((dy + row) * width + dx) * 4;
                if from + size * 4 <= texels.len() && to + size * 4 <= out.len() {
                    out[to..to + size * 4].copy_from_slice(&texels[from..from + size * 4]);
                }
            }
        }
    }
    out
}

/// misc_exporter_exr / misc_exporter_ktx2: the PMREM background, the data
/// texture quad and the export options.
struct Exporter {
    ktx2: bool,
    env: Arc<EnvironmentMap>,
    quad: Object3D,
    /// createDataTexture(): 800 × 800 RGBA Float32 normals.
    data: Vec<f32>,
    /// target (0 pmrem, 1 data-texture), then for EXR type (0 Float, 1 Half)
    /// and compression (0 ZIP, 1 ZIPS, 2 NONE).
    params: [f64; 3],
    readback: Option<AtlasReadback>,
}
/// webgpu_materials_video: the 20 × 10 cube wall.
struct VideoWall {
    video: web_sys::HtmlVideoElement,
    texture: wgpu::Texture,
    uploaded: Option<f64>,
    meshes: Vec<Object3D>,
    hue: Vec<f64>,
    saturation: Vec<f64>,
    delta: Vec<[f64; 2]>,
    rotation: Vec<[f64; 2]>,
    counter: u64,
}
/// webgpu_compile_async: the moving sphere and the 256 unique noise materials.
struct CompileGrid {
    sphere: Object3D,
    group: Object3D,
    plane: Arc<BufferGeometry>,
    /// Material sources and positions still to be built, a few per frame.
    pending: Vec<(String, Vector3)>,
    /// The first CSS size: a later resize switches to the 12-unit frustum.
    size: (f64, f64),
    resized: bool,
}
pub struct Demo {
    id: u32,
    time: f64,
    last: f64,
    controls: Option<Controls>,
    exporter: Option<Exporter>,
    wall: Option<VideoWall>,
    grid: Option<CompileGrid>,
    /// Absolute CSS pointer position, once the pointer has moved.
    pointer: Option<Vector2>,
    export: Option<(String, Vec<u8>)>,
    /// swapScene()'s camera reset, applied before the next frame.
    pending_camera: bool,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            273 | 274 => (50., 0.1, 100., Vector3::new(10., 0., 0.)),
            275 => (40., 1., 10000., Vector3::new(0., 0., 500.)),
            277 => (40., 1., 1000., Vector3::new(-10., 0., 23.)),
            _ => (50., 0.1, 100., Vector3::new(0., 0., 20.)),
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
            exporter: None,
            wall: None,
            grid: None,
            pointer: None,
            export: None,
            pending_camera: false,
        };
        match id {
            273 | 274 => d.exporter_scene(s, c, r).await?,
            275 => d.video_scene(s, r).await?,
            276 => d.compile_scene(s, c, r).await?,
            _ => d.normal_scene(s, c, r).await?,
        }
        Ok(d)
    }
    async fn exporter_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let ktx2 = self.id == 274;
        let hdr = if ktx2 {
            "/web/environments/venice_sunset_1k.hdr".to_string()
        } else {
            format!("{ASSETS}/tsl-procedural/san_giuseppe_bridge_2k.hdr")
        };
        // PMREMGenerator.fromEquirectangular: the cube-UV atlas is the background.
        let mut env = EnvironmentMap::from_hdr(&fetch(&hdr).await?)?;
        env.prefilter(r)?;
        let env = Arc::new(env);
        s.environment = Some(env.clone());
        s.background_environment = true;
        s.background_pmrem = true;
        if ktx2 {
            s.tone_mapping = ToneMapping::AgX;
        }
        // createDataTexture(): normals of a hemisphere inside radius 320, stored as
        // Float32 like the original.
        let (size, radius) = (800usize, 320.);
        let factor = PI * 0.5 / radius;
        let mut data = vec![0f32; 4 * size * size];
        for i in 0..size {
            for j in 0..size {
                let idx = i * size * 4 + j * 4;
                let coord = Vector2::new(j as f64, i as f64) - Vector2::splat(size as f64 / 2.);
                let normal = if coord.length() < radius {
                    Vector3::new(
                        (coord.x * factor).sin(),
                        (coord.y * factor).sin(),
                        (coord.x * factor).cos(),
                    )
                } else {
                    Vector3::Z
                };
                data[idx] = (0.5 + 0.5 * normal.x) as f32;
                data[idx + 1] = (0.5 + 0.5 * normal.y) as f32;
                data[idx + 2] = (0.5 + 0.5 * normal.z) as f32;
                data[idx + 3] = 1.;
            }
        }
        // The Float DataTexture, nearest-filtered, shown by a MeshBasicMaterial. It
        // is resident as half floats, which keep its normals to 2⁻¹¹.
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("exporter data texture"),
            size: wgpu::Extent3d {
                width: size as u32,
                height: size as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let half: Vec<half::f16> = data.iter().map(|&v| half::f16::from_f32(v)).collect();
        r.queue.write_texture(
            texture.as_image_copy(),
            bytemuck::cast_slice(&half),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size as u32 * 8),
                rows_per_image: Some(size as u32),
            },
            texture.size(),
        );
        let view = texture.create_view(&Default::default());
        let sampler = r.device.create_sampler(&Default::default());
        let color = crate::tsl::Texture::External(0).sample(uv());
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(1)?,
            &[],
            &[(&view, &sampler)],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let quad = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(50., 50., 1, 1)?),
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        s.get_mut(quad)?.visible = false;
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.exporter = Some(Exporter {
            ktx2,
            env,
            quad,
            data,
            params: [0., 1., 0.],
            readback: None,
        });
        Ok(())
    }
    /// The page's `#video`, playing from 3 s, as a VideoTexture on 200 Phong cubes
    /// whose UVs each cover one cell of the frame.
    async fn video_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        use wasm_bindgen::JsCast;
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or(Error::Invalid("document"))?;
        let video: web_sys::HtmlVideoElement = document
            .get_element_by_id("video")
            .ok_or(Error::Invalid("video element"))?
            .dyn_into()
            .map_err(|_| Error::Invalid("video element"))?;
        while video.ready_state() < 2 {
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                let _ = web_sys::window()
                    .map(|w| w.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 20));
            });
            wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|_| Error::Invalid("video wait"))?;
        }
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 7.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0.5, 1., 1.).normalize();
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("video texture"),
            size: wgpu::Extent3d {
                width: video.video_width(),
                height: video.video_height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        // VideoTexture: linear filtering without mipmaps, clamped, flipped on upload.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // One program: each Phong material multiplies its color by the video.
        let graph = SurfaceNodes {
            color: Some(
                crate::tsl::base_color().rgb()
                    * crate::tsl::Texture::External(0).sample(uv()).rgb(),
            ),
            ..Default::default()
        };
        let program = Arc::new(graph.build(r, &[], &[(&view, &sampler)]).await?);
        let (xgrid, ygrid) = (20usize, 10usize);
        let (ux, uy) = (1. / xgrid as f64, 1. / ygrid as f64);
        let (xsize, ysize) = (480. / xgrid as f64, 204. / ygrid as f64);
        let mut seed = 186u32;
        let mut wall = VideoWall {
            video,
            texture,
            uploaded: None,
            meshes: vec![],
            hue: vec![],
            saturation: vec![],
            delta: vec![],
            rotation: vec![],
            counter: 1,
        };
        for i in 0..xgrid {
            for j in 0..ygrid {
                let mut geometry = BoxGeometry::build(xsize, ysize, xsize)?;
                // change_uvs( geometry, ux, uy, i, j ), in Float32.
                if let Some(Attribute::F32(uvs)) = geometry.attributes.get_mut("uv") {
                    for (k, v) in uvs.array_mut().iter_mut().enumerate() {
                        *v = if k % 2 == 0 {
                            ((*v as f64 + i as f64) * ux) as f32
                        } else {
                            ((*v as f64 + j as f64) * uy) as f32
                        };
                    }
                }
                let (hue, saturation) = (i as f64 / xgrid as f64, 1. - j as f64 / ygrid as f64);
                let mut m = MeshPhongMaterial::default();
                m.properties.color = Color::from_hsl(hue, saturation, 0.5);
                m.properties.vertex_program = Some(program.clone());
                let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(geometry),
                    Arc::new(Material::Phong(m)),
                )));
                s.get_mut(mesh)?.position = Vector3::new(
                    (i as f64 - xgrid as f64 / 2.) * xsize,
                    (j as f64 - ygrid as f64 / 2.) * ysize,
                    0.,
                );
                let dx = 0.001 * (0.5 - random(&mut seed));
                let dy = 0.001 * (0.5 - random(&mut seed));
                wall.meshes.push(mesh);
                wall.hue.push(hue);
                wall.saturation.push(saturation);
                wall.delta.push([dx, dy]);
                wall.rotation.push([0., 0.]);
            }
        }
        self.wall = Some(wall);
        Ok(())
    }
    /// The 256 MeshBasicNodeMaterials, each with its own noise graph and
    /// constants, are built before they are shown, as compileAsync does.
    async fn compile_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        use crate::tsl::materialx::{Dimension, cell, fractal_vec3, perlin_vec3, worley_vec3};
        s.background = Color::from_hex(0x111111);
        let (w, h, _) = viewport_css();
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Orthographic(Self::frustum(20., w / h)));
        // MeshNormalNodeMaterial: directionToColor( normalView ), taken as sRGB.
        let normal_color = crate::tsl::srgb_to_linear(
            crate::tsl::normal_view_geometry().normalize() * float(0.5) + float(0.5),
        );
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(vec4(normal_color, float(1.))).wgsl(0)?,
            &[],
            &[],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let sphere = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.5, 32, 32)?),
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        let group = s.insert(NodeKind::Group);
        s.get_mut(group)?.visible = false;
        let plane = Arc::new(PlaneGeometry::build(0.9, 0.9, 1, 1)?);
        let (count, grid) = (256usize, 16usize);
        let start = -(grid as f64 - 1.) / 2.;
        let hash = |seed: u32| crate::tsl::hash(crate::tsl::uint(seed));
        let mut pending = vec![];
        for i in 0..count {
            let seed = float(i as f32 * 0.1 + 1.);
            let scale = float((i % 5) as f32 + 2.);
            let uv_node = uv() * scale + hash(i as u32);
            let color = match i % 4 {
                0 => perlin_vec3(uv_node * seed, Dimension::D2) * float(0.5) + float(0.5),
                1 => worley_vec3(
                    uv_node * (seed * float(0.5)),
                    Dimension::D2,
                    float(1.),
                    float(1.),
                ),
                2 => {
                    let cell = cell(uv_node * seed, Dimension::D2);
                    vec3(cell.clone(), cell.clone() * float(0.7), cell * float(0.4))
                }
                _ => {
                    // mx_fractal_noise_vec3 has no 2D overload: TSL widens the UV to
                    // vec3( uv, 0.0 ) for the 3D noise.
                    let p = uv_node * (seed * float(0.3));
                    fractal_vec3(
                        vec3(p.x(), p.y(), float(0.)),
                        Dimension::D3,
                        float(3.),
                        float(2.),
                        float(0.5),
                    ) * float(0.5)
                        + float(0.5)
                }
            };
            let tint = vec3(
                hash(i as u32 * 3),
                hash(i as u32 * 3 + 1),
                hash(i as u32 * 3 + 2),
            ) * float(0.3)
                + float(0.7);
            pending.push((
                NodeMaterial::new(vec4(color * tint, float(1.))).wgsl(0)?,
                Vector3::new(start + (i % grid) as f64, start + (i / grid) as f64, 0.),
            ));
        }
        pending.reverse();
        self.grid = Some(CompileGrid {
            sphere,
            group,
            plane,
            pending,
            size: (w, h),
            resized: false,
        });
        Ok(())
    }
    fn frustum(size: f64, aspect: f64) -> OrthographicCamera {
        OrthographicCamera {
            left: size * aspect / -2.,
            right: size * aspect / 2.,
            top: size / 2.,
            bottom: size / -2.,
            near: 0.1,
            far: 100.,
            ..Default::default()
        }
    }
    /// Nefertiti with its object-space normal map, attribute normals removed,
    /// double-sided, halved and recentered; the point light rides on the camera.
    async fn normal_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.6,
        }));
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 4.5,
            distance: 0.,
            decay: 0.,
        }));
        s.add(c, light)?;
        let (asset, buffers, images) =
            load_asset(&format!("{ASSETS}/exporters-video/Nefertiti.glb")).await?;
        let meshes = crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        for h in meshes {
            let n = s.get_mut(h)?;
            n.matrix_auto_update = true;
            let NodeKind::Mesh(mesh) = &mut n.kind else {
                continue;
            };
            let mut geometry = (*mesh.geometry).clone();
            geometry.delete_attribute("normal");
            let bounds = geometry.compute_bounding_box()?;
            mesh.geometry = Arc::new(geometry);
            let material = Arc::make_mut(&mut mesh.materials[0]);
            let Material::Standard(m) = material else {
                return Err(Error::Invalid("Nefertiti material"));
            };
            let normal_map = m.normal_map.take().ok_or(Error::Invalid("normal map"))?;
            m.properties.side = Side::Double;
            // normal_fragment_maps with USE_NORMALMAP_OBJECTSPACE: the map's normal,
            // flipped for back faces, through the normal matrix.
            let normal = WgslFn::new(
                "object_space_normal",
                "fn object_space_normal(sample:vec3<f32>)->vec3<f32>{let n=(sample*2.0-1.0)*select(-1.0,1.0,fragment_front);return normalize((u.view*vec4((u.normal*vec4(n,0.0)).xyz,0.0)).xyz);}",
                &[Type::Vec3],
                Type::Vec3,
            )?
            // glTF textures are not flipped: the UVs sample them directly.
            .call(&[crate::tsl::Texture::External(0).sample(uv()).rgb()]);
            let gpu = r.upload_texture(&normal_map)?;
            // Without the normal attribute, WebGL reads normals as zero: nonPerturbedNormal
            // and geometryRoughness are NaN, and the roughness clamp min( NaN, 1.0 )
            // yields 1.0 on the reference GPU.
            let program = SurfaceNodes {
                normal: Some(normal),
                roughness: Some(float(1.)),
                ..Default::default()
            }
            .build(r, &[], &[(&gpu.view, &gpu.sampler)])
            .await?;
            m.properties.vertex_program = Some(Arc::new(program));
            n.scale = Vector3::splat(0.5);
            n.position = -(bounds.min + bounds.max) * 0.25;
        }
        let mut controls = Controls::new(None, (10., 50.), PI, true);
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
    pub fn prepare(&mut self, r: &Renderer, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        match self.id {
            273 | 274 => {
                let k = self.exporter.as_mut().ok_or(Error::Invalid("exporter"))?;
                let data = k.params[0] > 0.5;
                if std::mem::take(&mut self.pending_camera) {
                    s.get_mut(c)?.position = if data {
                        Vector3::new(0., 0., 70.)
                    } else {
                        Vector3::new(10., 0., 0.)
                    };
                }
                s.background_environment = !data;
                s.get_mut(k.quad)?.visible = data;
                if k.ktx2 {
                    s.tone_mapping = if data {
                        ToneMapping::None
                    } else {
                        ToneMapping::AgX
                    };
                }
                // animate() calls controls.update() once per frame, enabled or not.
                if let Some(controls) = &mut self.controls {
                    controls.frame_update(s, c)?;
                }
            }
            275 => self.video_step(r, s, c, steps)?,
            276 => {
                let k = self.grid.as_mut().ok_or(Error::Invalid("grid"))?;
                // The programs are built between frames while the sphere animates, as
                // compileAsync does; setTimeout( addMeshes, 1000 ) shows them, building
                // any still pending when it fires.
                let batch = if t >= 1. { k.pending.len() } else { 16 };
                for _ in 0..batch {
                    let Some((wgsl, position)) = k.pending.pop() else {
                        break;
                    };
                    let program = ShaderProgram::with_projection_unvalidated(
                        r,
                        &wgsl,
                        &[],
                        "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
                    );
                    let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                        k.plane.clone(),
                        Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
                    )));
                    s.get_mut(mesh)?.position = position;
                    s.add(k.group, mesh)?;
                }
                s.get_mut(k.group)?.visible = t >= 1.;
                s.get_mut(k.sphere)?.position.x = (t * 2.).sin() * 8.;
                let (w, h, _) = viewport_css();
                if (w, h) != k.size {
                    // onWindowResize uses a 12-unit frustum, unlike init's 20.
                    k.resized = true;
                    k.size = (w, h);
                }
                let size = if k.resized { 12. } else { 20. };
                s.get_mut(c)?.kind =
                    NodeKind::Camera(Camera::Orthographic(Self::frustum(size, w / h)));
            }
            _ => {}
        }
        Ok(())
    }
    /// render(): the camera eases toward the mouse; the colors cycle with time;
    /// from the 201st frame of every 1000 the cubes drift, reversing every 1000.
    fn video_step(&mut self, r: &Renderer, s: &mut Scene, c: Object3D, steps: usize) -> Result<()> {
        let k = self.wall.as_mut().ok_or(Error::Invalid("wall"))?;
        let time = k.video.current_time();
        if k.video.ready_state() >= 2 && k.uploaded != Some(time) {
            r.queue.copy_external_image_to_texture(
                &wgpu::CopyExternalImageSourceInfo {
                    source: wgpu::ExternalImageSource::HTMLVideoElement(k.video.clone()),
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
                k.texture.size(),
            );
            k.uploaded = Some(time);
        }
        // mouseX and mouseY stay 0 until the first mousemove.
        let (w, h, _) = viewport_css();
        let mouse = self.pointer.map_or(Vector2::ZERO, |p| {
            Vector2::new(p.x - w / 2., (p.y - h / 2.) * 0.3)
        });
        let clock = self.time * 1000. * 0.00005;
        for (i, &mesh) in k.meshes.iter().enumerate() {
            let h = (360. * (k.hue[i] + clock) % 360.) / 360.;
            if let NodeKind::Mesh(m) = &mut s.get_mut(mesh)?.kind {
                Arc::make_mut(&mut m.materials[0]).properties_mut().color =
                    Color::from_hsl(h, k.saturation[i], 0.5);
            }
        }
        for _ in 0..steps {
            let n = s.get_mut(c)?;
            n.position.x += (mouse.x - n.position.x) * 0.05;
            n.position.y += (-mouse.y - n.position.y) * 0.05;
            if k.counter % 1000 > 200 {
                for (i, &mesh) in k.meshes.iter().enumerate() {
                    let [dx, dy] = k.delta[i];
                    k.rotation[i][0] += 10. * dx;
                    k.rotation[i][1] += 10. * dy;
                    let n = s.get_mut(mesh)?;
                    n.position.x -= 150. * dx;
                    n.position.y += 150. * dy;
                    n.position.z += 300. * dx;
                }
            }
            if k.counter % 1000 == 0 {
                for d in &mut k.delta {
                    d[0] *= -1.;
                    d[1] *= -1.;
                }
            }
            k.counter += 1;
        }
        for (i, &mesh) in k.meshes.iter().enumerate() {
            s.get_mut(mesh)?.quaternion = Euler {
                angles: Vector3::new(k.rotation[i][0], k.rotation[i][1], 0.),
                order: EulerOrder::XYZ,
            }
            .quaternion();
        }
        s.look_at(c, Vector3::ZERO)
    }
    /// Absolute CSS-pixel pointer moves.
    pub fn draw(&mut self, _kind: u32, x: f64, y: f64) {
        self.pointer = Some(Vector2::new(x, y));
    }
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
        let exporter = self.exporter.as_ref();
        // The data-texture view disables the controls.
        if exporter.is_some_and(|k| k.params[0] > 0.5) {
            return Ok(());
        }
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            if self.id == 277 {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else if exporter.is_some() {
            // rotateSpeed = −0.25, to track the pointer.
            controls.rotate(-0.25 * dx, -0.25 * dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    /// Controls: target, then (EXR) type and compression, then the export button.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let k = self
            .exporter
            .as_mut()
            .ok_or(Error::Invalid("exporter parameter"))?;
        let v = value as f64;
        let export = if k.ktx2 { 1 } else { 3 };
        if index == export {
            let data = k.params[0] > 0.5;
            let name = format!(
                "{}.{}",
                if data { "data-texture" } else { "pmrem" },
                if k.ktx2 { "ktx2" } else { "exr" }
            );
            if data {
                self.export = Some((name, k.encode(&Pixels::Float(&k.data), 800, 800)));
            } else {
                self.export = Some((name, vec![]));
                k.readback = None;
            }
            return Ok(());
        }
        if index >= export || !(0. ..3.).contains(&v) {
            return Err(Error::Invalid("exporter parameter"));
        }
        if index == 0 && v != k.params[0] {
            // swapScene(): each target resets the camera; the data view disables the controls.
            self.pending_camera = true;
        }
        k.params[index] = v;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    /// The exported file: data textures at once, the PMREM after its readback.
    pub fn take_export(&mut self, r: &Renderer) -> Option<(String, Vec<u8>)> {
        let (name, bytes) = self.export.take()?;
        if !bytes.is_empty() {
            return Some((name, bytes));
        }
        let k = self.exporter.as_mut()?;
        let gpu = k.env.gpu.as_ref()?;
        let readback = k
            .readback
            .get_or_insert_with(|| AtlasReadback::begin(r, gpu.texture()));
        match readback.take() {
            Some(Ok(texels)) => {
                let (w, h) = (readback.width, readback.height);
                let texels = three_cube_uv(&texels, w, h);
                k.readback = None;
                Some((name, k.encode(&Pixels::Half(&texels), w, h)))
            }
            Some(Err(_)) => {
                k.readback = None;
                None
            }
            None => {
                self.export = Some((name, vec![]));
                None
            }
        }
    }
}
impl Exporter {
    fn encode(&self, pixels: &Pixels, width: u32, height: u32) -> Vec<u8> {
        if self.ktx2 {
            let (kind, bytes): (Ktx2Kind, &[u8]) = match pixels {
                Pixels::Float(v) => (Ktx2Kind::Float, bytemuck::cast_slice(v)),
                Pixels::Half(v) => (Ktx2Kind::HalfLinear, bytemuck::cast_slice(v)),
            };
            formats::ktx2(kind, bytes, width, height)
        } else {
            let compression = [
                formats::ZIP_COMPRESSION,
                formats::ZIPS_COMPRESSION,
                formats::NO_COMPRESSION,
            ][self.params[2] as usize];
            formats::exr(
                pixels,
                width as usize,
                height as usize,
                self.params[1] > 0.5,
                compression,
            )
        }
    }
}
