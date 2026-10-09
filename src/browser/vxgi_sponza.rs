//! webgpu_vxgi_sponza: Sponza, fetched at run time from glTF-Sample-Assets
//! as the page does ( its license does not grant redistribution ), lit by a
//! shadowed directional light and a 0.5 ambient light under SkyMesh, with
//! VXGINode's voxel cone traced AO and indirect light resolved by TRAA and
//! FirstPersonControls.
//!
//! VXGIVolume collects the scene's triangles on the CPU once ( split to the
//! sub-voxel edge, their albedo from each texture's 64² downscale at the
//! centroid, as the addon samples it ) and voxelizes them on the GPU into a
//! 144 × 64 × 96 opacity volume with its mips; the directional light is
//! injected through its shadow map and bounced once into the radiance
//! volume, again when the light or the injection settings change. Each frame
//! the shadow pass, the pre-pass ( packed normals and velocity ), the cone
//! tracer ( AO and GI, its cones rotating with frameId ), the scene pass
//! reading them in the materials' lighting, TRAA and the output run the WGSL
//! three.js r186 generates for the page ( in `vxgi_sponza/`, the cone tracer
//! byte-identical to webgpu_vxgi's; the compute kernels drop the subgroup
//! built-in they declare but never use ). `vxgi_sponza/tables.rs` lists what
//! each material uniform holds, as the reference page's draws show. The
//! output views follow updatePostprocessing(): Direct is the scene pass
//! without the GI context, AO and GI show the effect alone, and without
//! temporal filtering the scene pass is shown without TRAA.
#[path = "vxgi_sponza/tables.rs"]
mod tables;
use super::deferred::{Draw, culled_pipeline, set};
use super::gltf_viewer::{fetch, load_asset_bytes};
use super::lightprobes_sponza::sponza_url;
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::uniform;
use super::trackball_sprites::FirstPerson;
use super::vxgi::{apply, color, compute_pipeline, depth, halton, texture, view};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW: u32 = 2048;
/// VXGINode's TEMPORAL_CYCLE.
const TEMPORAL_CYCLE: usize = 64;
/// VXGIVolume's MAX_EDGE_SUBVOXELS, TRIANGLE_STRIDE and the scene
/// collector's IMAGE_SAMPLE_SIZE.
const MAX_EDGE_SUBVOXELS: f64 = 16.;
const TRIANGLE_STRIDE: usize = 20;
const IMAGE_SAMPLE_SIZE: u32 = 64;
/// VXGIVolume's resolution.
const RESOLUTION: f64 = 128.;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("vxgi_sponza/", $name, ".wgsl"))
    };
}

/// What a material uniform holds.
#[derive(Clone, Copy)]
enum V {
    /// The pass camera's projection ( jittered for the main camera ) and view.
    Projection,
    View,
    /// The main camera's projection without TRAA's jitter.
    Unjittered,
    /// VelocityNode's previous camera matrices.
    PreviousProjection,
    PreviousView,
    Model,
    NormalMatrix,
    Color,
    NormalScale,
    AlphaTest,
    LightColor,
    LightPosition,
    LightTarget,
    ShadowMatrix,
    /// The sun's direction for SkyMesh.
    SunDirection,
    Ambient,
    CameraPosition,
    Size,
    /// TSL `time`, for SkyMesh's clouds.
    Time,
    Const(&'static [f64]),
}
/// What a material texture binding holds.
#[derive(Clone, Copy, PartialEq)]
enum Role {
    Map,
    RoughnessMetalness,
    NormalMap,
    Dfg,
    Shadow,
    Ao,
    Gi,
}
/// The material variants: textured ( base, normal and
/// metallic-roughness maps ), masked ( the same, alpha-tested and double
/// sided ), base ( a base color map ) and SkyMesh.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Textured,
    Masked,
    Base,
}
struct Variant {
    vs: &'static str,
    fs: &'static str,
    values: &'static [(&'static str, &'static str, V)],
    textures: &'static [(&'static str, Role)],
}
const SHADOWS: [Variant; 2] = [
    Variant {
        vs: wgsl!("shadow_vs"),
        fs: wgsl!("shadow_fs"),
        values: tables::SHADOW,
        textures: tables::SHADOW_TEXTURES,
    },
    Variant {
        vs: wgsl!("shadow_masked_vs"),
        fs: wgsl!("shadow_masked_fs"),
        values: tables::SHADOW_MASKED,
        textures: tables::SHADOW_MASKED_TEXTURES,
    },
];
const PREPASS: [Variant; 4] = [
    Variant {
        vs: wgsl!("prepass_textured_vs"),
        fs: wgsl!("prepass_textured_fs"),
        values: tables::PREPASS_TEXTURED,
        textures: tables::PREPASS_TEXTURED_TEXTURES,
    },
    Variant {
        vs: wgsl!("prepass_masked_vs"),
        fs: wgsl!("prepass_masked_fs"),
        values: tables::PREPASS_MASKED,
        textures: tables::PREPASS_MASKED_TEXTURES,
    },
    Variant {
        vs: wgsl!("prepass_base_vs"),
        fs: wgsl!("prepass_base_fs"),
        values: tables::PREPASS_BASE,
        textures: tables::PREPASS_BASE_TEXTURES,
    },
    Variant {
        vs: wgsl!("prepass_sky_vs"),
        fs: wgsl!("prepass_sky_fs"),
        values: tables::PREPASS_SKY,
        textures: tables::PREPASS_SKY_TEXTURES,
    },
];
const SCENE: [Variant; 4] = [
    Variant {
        vs: wgsl!("scene_textured_vs"),
        fs: wgsl!("scene_textured_fs"),
        values: tables::SCENE_TEXTURED,
        textures: tables::SCENE_TEXTURED_TEXTURES,
    },
    Variant {
        vs: wgsl!("scene_masked_vs"),
        fs: wgsl!("scene_masked_fs"),
        values: tables::SCENE_MASKED,
        textures: tables::SCENE_MASKED_TEXTURES,
    },
    Variant {
        vs: wgsl!("scene_base_vs"),
        fs: wgsl!("scene_base_fs"),
        values: tables::SCENE_BASE,
        textures: tables::SCENE_BASE_TEXTURES,
    },
    Variant {
        vs: wgsl!("scene_sky_vs"),
        fs: wgsl!("scene_sky_fs"),
        values: tables::SCENE_SKY,
        textures: tables::SCENE_SKY_TEXTURES,
    },
];
/// The scene pass without the GI context ( the Direct output ).
const DIRECT: [Variant; 4] = [
    Variant {
        vs: wgsl!("direct_textured_vs"),
        fs: wgsl!("direct_textured_fs"),
        values: tables::DIRECT_TEXTURED,
        textures: tables::DIRECT_TEXTURED_TEXTURES,
    },
    Variant {
        vs: wgsl!("direct_masked_vs"),
        fs: wgsl!("direct_masked_fs"),
        values: tables::DIRECT_MASKED,
        textures: tables::DIRECT_MASKED_TEXTURES,
    },
    Variant {
        vs: wgsl!("direct_base_vs"),
        fs: wgsl!("direct_base_fs"),
        values: tables::DIRECT_BASE,
        textures: tables::DIRECT_BASE_TEXTURES,
    },
    Variant {
        vs: wgsl!("direct_sky_vs"),
        fs: wgsl!("direct_sky_fs"),
        values: tables::DIRECT_SKY,
        textures: tables::DIRECT_SKY_TEXTURES,
    },
];

/// The values a frame's uniforms read.
struct Frame {
    unjittered: Matrix4,
    previous_projection: Matrix4,
    previous_view: Matrix4,
    camera_position: Vector3,
    size: [f64; 2],
    light_color: [f64; 3],
    light_position: Vector3,
    light_target: Vector3,
    shadow_matrix: Matrix4,
    ambient: [f64; 3],
    sun: Vector3,
    time: f64,
}
/// The module declaring a struct ( the stages share identical ones ).
fn source_of(v: &Variant, name: &str) -> &'static str {
    if v.fs.contains(&format!("struct {name} {{")) {
        v.fs
    } else {
        v.vs
    }
}
/// A struct's members from a variant's table, for one draw.
fn values(
    v: &Variant,
    source: &str,
    name: &str,
    f: &Frame,
    camera: (Matrix4, Matrix4),
    model: Matrix4,
    mesh: Option<&Mesh>,
) -> Result<Vec<u8>> {
    let mut out: Vec<(&str, Vec<f64>)> = vec![];
    for &(structure, member, value) in v.values {
        if structure != name {
            continue;
        }
        let data = match value {
            V::Projection => m4(camera.0),
            V::View => m4(camera.1),
            V::Unjittered => m4(f.unjittered),
            V::PreviousProjection => m4(f.previous_projection),
            V::PreviousView => m4(f.previous_view),
            V::Model => m4(model),
            V::NormalMatrix => m3(model.inverse().transpose()),
            V::Color => mesh.map_or(vec![1.; 3], |m| m.color.to_vec()),
            V::NormalScale => mesh.map_or(vec![1.; 2], |m| m.normal_scale.to_vec()),
            V::AlphaTest => vec![mesh.map_or(0., |m| m.alpha_test)],
            V::LightColor => f.light_color.to_vec(),
            V::LightPosition => f.light_position.to_array().to_vec(),
            V::LightTarget => f.light_target.to_array().to_vec(),
            V::ShadowMatrix => m4(f.shadow_matrix),
            V::SunDirection => f.sun.to_array().to_vec(),
            V::Ambient => f.ambient.to_vec(),
            V::CameraPosition => f.camera_position.to_array().to_vec(),
            V::Size => f.size.to_vec(),
            V::Time => vec![f.time],
            V::Const(c) => c.to_vec(),
        };
        out.push((member, data));
    }
    let refs: Vec<(&str, &[f64])> = out.iter().map(|(n, v)| (*n, &v[..])).collect();
    pack(source, name, &refs)
}

/// A Sponza primitive: its buffers, bounds, model, material and its draws'
/// object uniforms.
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    uvs: wgpu::Buffer,
    tangents: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    sphere: Sphere,
    model: Matrix4,
    kind: Kind,
    color: [f64; 3],
    normal_scale: [f64; 2],
    alpha_test: f64,
    maps: [Option<crate::texture_gpu::GpuTexture>; 3],
    /// Shadow, pre-pass, scene and direct scene objects.
    objects: [wgpu::Buffer; 4],
}
impl Mesh {
    fn visible(&self, frustum: &Frustum) -> bool {
        let scale = self.model.to_scale_rotation_translation().0.max_element();
        frustum.intersects_sphere(Sphere {
            center: self.model.transform_point3(self.sphere.center),
            radius: self.sphere.radius * scale,
        })
    }
    fn draw(&self, pass: &mut wgpu::RenderPass, shadow: bool) {
        if shadow {
            pass.set_vertex_buffer(0, self.uvs.slice(..));
            pass.set_vertex_buffer(1, self.positions.slice(..));
        } else {
            pass.set_vertex_buffer(0, self.uvs.slice(..));
            pass.set_vertex_buffer(1, self.normals.slice(..));
            if self.kind == Kind::Base {
                pass.set_vertex_buffer(2, self.positions.slice(..));
            } else {
                pass.set_vertex_buffer(2, self.tangents.slice(..));
                pass.set_vertex_buffer(3, self.positions.slice(..));
            }
        }
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
    fn variant(&self) -> usize {
        match self.kind {
            Kind::Textured => 0,
            Kind::Masked => 1,
            Kind::Base => 2,
        }
    }
}
/// The SkyMesh box ( scaled 450000 ), its pre-pass, scene and direct scene
/// objects.
struct SkyBox {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    model: Matrix4,
    objects: [wgpu::Buffer; 3],
}

/// The scene collector's downscaled texture: IMAGE_SAMPLE_SIZE² RGBA,
/// drawn by the browser's canvas with high-quality smoothing as the addon
/// draws the image.
async fn downscale(t: &Texture) -> Result<Vec<u8>> {
    use wasm_bindgen::JsCast;
    let fail = |_| Error::Invalid("collector canvas");
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or(Error::Invalid("document"))?;
    let canvas = |w: u32,
                  h: u32|
     -> Result<(
        web_sys::HtmlCanvasElement,
        web_sys::CanvasRenderingContext2d,
    )> {
        let c = document
            .create_element("canvas")
            .map_err(fail)?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| Error::Invalid("collector canvas"))?;
        c.set_width(w);
        c.set_height(h);
        let x = c
            .get_context("2d")
            .map_err(fail)?
            .ok_or(Error::Invalid("collector context"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|_| Error::Invalid("collector context"))?;
        Ok((c, x))
    };
    // The loader's ImageBitmap ( premultiplyAlpha none ), as drawImage reads it.
    let image = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&t.rgba),
        t.width,
        t.height,
    )
    .map_err(fail)?;
    let options = web_sys::ImageBitmapOptions::new();
    options.set_premultiply_alpha(web_sys::PremultiplyAlpha::None);
    let promise = web_sys::window()
        .ok_or(Error::Invalid("window"))?
        .create_image_bitmap_with_image_data_and_image_bitmap_options(&image, &options)
        .map_err(fail)?;
    let source = crate::material::BrowserBitmap(
        wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(fail)?
            .dyn_into::<web_sys::ImageBitmap>()
            .map_err(|_| Error::Invalid("collector bitmap"))?,
    );
    let size = IMAGE_SAMPLE_SIZE;
    let (_, small) = canvas(size, size)?;
    small.set_image_smoothing_enabled(true);
    // imageSmoothingQuality = 'high' ( not in web-sys ).
    let _ = js_sys::Reflect::set(&small, &"imageSmoothingQuality".into(), &"high".into());
    small
        .draw_image_with_image_bitmap_and_dw_and_dh(&source.0, 0., 0., size as f64, size as f64)
        .map_err(fail)?;
    Ok(small
        .get_image_data(0., 0., size as f64, size as f64)
        .map_err(fail)?
        .data()
        .0)
}
/// SRGBToLinear over the bytes, as the collector's LUT.
fn srgb_lut() -> [f64; 256] {
    std::array::from_fn(|i| {
        let c = i as f64 / 255.;
        if c < 0.04045 {
            c * 0.0773993808
        } else {
            (c * 0.9478672986 + 0.0521327014).powf(2.4)
        }
    })
}
/// A primitive for the collector: positions, uvs, index, world matrix, its
/// material color, map sample, alpha test and side.
struct Source {
    positions: Vec<f32>,
    uvs: Vec<f32>,
    index: Vec<u32>,
    model: Matrix4,
    color: [f64; 3],
    map: Option<(Arc<Vec<u8>>, bool)>,
    alpha_test: f64,
    side: f64,
}
/// collectSceneTriangles(): each primitive's world triangles inside the
/// volume, albedo from the map at the centroid ( alpha-tested and below
/// minOpacity skipped ), split along their longest edge until no edge
/// exceeds the limit.
fn collect_triangles(sources: &[Source], bounds: (Vector3, Vector3), max_edge_sq: f64) -> Vec<f32> {
    let lut = srgb_lut();
    let mut out = vec![];
    for s in sources {
        let p = |i: u32| {
            let i = i as usize * 3;
            apply(
                &s.model,
                [
                    s.positions[i] as f64,
                    s.positions[i + 1] as f64,
                    s.positions[i + 2] as f64,
                ],
            )
        };
        for t in s.index.chunks(3) {
            if t.len() < 3 {
                continue;
            }
            let (a, b, c) = (p(t[0]), p(t[1]), p(t[2]));
            let lo = [0, 1, 2].map(|k| a[k].min(b[k]).min(c[k]));
            let hi = [0, 1, 2].map(|k| a[k].max(b[k]).max(c[k]));
            // Box3.intersectsBox.
            if hi[0] < bounds.0.x
                || lo[0] > bounds.1.x
                || hi[1] < bounds.0.y
                || lo[1] > bounds.1.y
                || hi[2] < bounds.0.z
                || lo[2] > bounds.1.z
            {
                continue;
            }
            let d = (0..3).map(|k| (hi[k] - lo[k]).powi(2)).sum::<f64>();
            if d < 1e-14 {
                continue;
            }
            let mut alpha = 1.;
            let mut albedo = s.color;
            if let Some((data, srgb)) = &s.map {
                let uv = |i: u32| {
                    [
                        s.uvs[i as usize * 2] as f64,
                        s.uvs[i as usize * 2 + 1] as f64,
                    ]
                };
                let (ua, ub, uc) = (uv(t[0]), uv(t[1]), uv(t[2]));
                // Vector2 add, add, multiplyScalar( 1 / 3 ), then transformUv
                // ( identity, RepeatWrapping ).
                let third = 1. / 3.;
                let mut centroid = [
                    (ua[0] + ub[0] + uc[0]) * third,
                    (ua[1] + ub[1] + uc[1]) * third,
                ];
                for v in &mut centroid {
                    if *v < 0. || *v > 1. {
                        *v -= v.floor();
                    }
                }
                let size = IMAGE_SAMPLE_SIZE as f64;
                let x = ((centroid[0] * size).floor()).clamp(0., size - 1.) as usize;
                let y = ((centroid[1] * size).floor()).clamp(0., size - 1.) as usize;
                let i = (y * IMAGE_SAMPLE_SIZE as usize + x) * 4;
                let channel = |k: usize| {
                    if *srgb {
                        f64::from(lut[data[i + k] as usize] as f32)
                    } else {
                        data[i + k] as f64 / 255.
                    }
                };
                albedo = [
                    albedo[0] * channel(0),
                    albedo[1] * channel(1),
                    albedo[2] * channel(2),
                ];
                alpha *= data[i + 3] as f64 / 255.;
            }
            if s.alpha_test > 0. && alpha < s.alpha_test {
                continue;
            }
            if alpha < 0.1 {
                continue;
            }
            let mut stack: Vec<[f64; 9]> =
                vec![[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]];
            while let Some(v) = stack.pop() {
                let [ax, ay, az, bx, by, bz, cx, cy, cz] = v;
                let ab = (bx - ax).powi(2) + (by - ay).powi(2) + (bz - az).powi(2);
                let bc = (cx - bx).powi(2) + (cy - by).powi(2) + (cz - bz).powi(2);
                let ca = (ax - cx).powi(2) + (ay - cy).powi(2) + (az - cz).powi(2);
                let longest = ab.max(bc).max(ca);
                if longest > max_edge_sq {
                    if longest == ab {
                        let m = [(ax + bx) * 0.5, (ay + by) * 0.5, (az + bz) * 0.5];
                        stack.push([ax, ay, az, m[0], m[1], m[2], cx, cy, cz]);
                        stack.push([m[0], m[1], m[2], bx, by, bz, cx, cy, cz]);
                    } else if longest == bc {
                        let m = [(bx + cx) * 0.5, (by + cy) * 0.5, (bz + cz) * 0.5];
                        stack.push([ax, ay, az, bx, by, bz, m[0], m[1], m[2]]);
                        stack.push([ax, ay, az, m[0], m[1], m[2], cx, cy, cz]);
                    } else {
                        let m = [(cx + ax) * 0.5, (cy + ay) * 0.5, (cz + az) * 0.5];
                        stack.push([ax, ay, az, bx, by, bz, m[0], m[1], m[2]]);
                        stack.push([m[0], m[1], m[2], bx, by, bz, cx, cy, cz]);
                    }
                    continue;
                }
                for x in [
                    ax, ay, az, 0., bx, by, bz, 0., cx, cy, cz, 0., albedo[0], albedo[1],
                    albedo[2], s.side, 0., 0., 0., 0.,
                ] {
                    out.push(x as f32);
                }
            }
        }
    }
    out
}

/// A compute kernel: its pipeline, bind group and dispatch size.
type Kernel = (wgpu::ComputePipeline, wgpu::BindGroup, u32);
/// The voxel volume: its grid, storage, textures and kernels.
struct Volume {
    grid: [u32; 3],
    voxel_size: f64,
    bounds_min: Vector3,
    levels: u32,
    /// Clear, voxelize, resolve and the opacity mips.
    voxelize: Vec<Kernel>,
    /// Per ping-pong index: the inject kernel ( direct light into it ), its
    /// radiance mips, and the bounce into it from the other index.
    inject: [Kernel; 2],
    radiance_mips: [Vec<Kernel>; 2],
    bounce: [Kernel; 2],
    inject_object: wgpu::Buffer,
    bounce_object: wgpu::Buffer,
    lights: wgpu::Buffer,
    opacity: wgpu::TextureView,
    radiance: wgpu::TextureView,
}
impl Volume {
    fn new(
        r: &Renderer,
        triangles: &[f32],
        bounds: (Vector3, Vector3),
        shadow: &wgpu::TextureView,
    ) -> Result<Self> {
        // _voxelize: the longest axis gets `resolution` voxels; all axes are
        // padded by a voxel and rounded up to whole mips.
        let size = bounds.1 - bounds.0;
        let voxel_size = size.x.max(size.y).max(size.z) / RESOLUTION;
        let mut levels = (RESOLUTION.log2().floor() - 2.).clamp(1., 8.) as u32;
        let multiple = 2f64.powi(levels as i32 - 1);
        let axis = |s: f64| (((s / voxel_size).ceil() + 2.) / multiple).ceil() * multiple;
        let grid = [axis(size.x), axis(size.y), axis(size.z)].map(|v| v as u32);
        levels = levels.min((grid[0].max(grid[1]) as f64).log2().floor() as u32 + 1);
        let bounds_min = bounds.0 - Vector3::splat(voxel_size);
        let voxels = grid[0] * grid[1] * grid[2];
        let groups = |n: u32| n.div_ceil(64);
        let storage = |label, size: u64| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size,
                usage: wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            })
        };
        let occupancy = storage("vxgi occupancy", voxels as u64 * 4);
        let triangle_ids = storage("vxgi triangle ids", voxels as u64 * 4);
        let triangle_count = (triangles.len() / TRIANGLE_STRIDE) as u32;
        let triangle_buffer = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vxgi triangles"),
                contents: bytemuck::cast_slice(triangles),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let mips = 32 - grid[0].max(grid[1]).max(grid[2]).leading_zeros();
        let volume_texture = |format, mip_level_count| {
            r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("vxgi volume"),
                size: wgpu::Extent3d {
                    width: grid[0],
                    height: grid[1],
                    depth_or_array_layers: grid[2],
                },
                mip_level_count,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D3,
                format,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
        };
        let level = |t: &wgpu::Texture, mip: u32| {
            t.create_view(&wgpu::TextureViewDescriptor {
                base_mip_level: mip,
                mip_level_count: Some(1),
                ..Default::default()
            })
        };
        let opacity_texture = volume_texture(BYTE, mips);
        // Radiance index 0 ( the traced texture ) and 1 ( the ping-pong ).
        let radiance_textures = [volume_texture(HALF, mips), volume_texture(HALF, mips)];
        let direct = view(&volume_texture(HALF, 1));
        let write = |source: &str, values: &[(&str, Vec<f64>)]| -> Result<wgpu::Buffer> {
            let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            Ok(r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("vxgi kernel"),
                    contents: &pack(source, "objectStruct", &values)?,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }))
        };
        let v = voxels as f64;
        let min = bounds_min.to_array().to_vec();
        let res = wgpu::BindingResource::TextureView;
        let kernel = |source: &str,
                      object: &wgpu::Buffer,
                      entries: Vec<(u32, wgpu::BindingResource)>,
                      count: u32|
         -> Kernel {
            let p = compute_pipeline(r, "vxgi kernel", source);
            // The mip kernels take their uniforms at binding 2, the others at 0.
            let at = if entries.iter().any(|(b, _)| *b == 0) {
                2
            } else {
                0
            };
            let mut e = vec![(at, object.as_entire_binding())];
            e.extend(entries);
            let g = bind(r, p.get_bind_group_layout(0), &e);
            (p, g, count)
        };
        let mip_counts: Vec<u32> = (1..levels)
            .map(|m| (0..3).map(|k| (grid[k] >> m).max(1)).product())
            .collect();
        let mut voxelize = vec![];
        let clear_object = write(
            wgsl!("clear_cs"),
            &[("nodeUniform0", vec![v]), ("nodeUniform3", vec![v])],
        )?;
        voxelize.push(kernel(
            wgsl!("clear_cs"),
            &clear_object,
            vec![
                (1, occupancy.as_entire_binding()),
                (2, triangle_ids.as_entire_binding()),
            ],
            groups(voxels),
        ));
        if triangle_count > 0 {
            let object = write(
                wgsl!("voxelize_cs"),
                &[
                    ("nodeUniform0", vec![triangle_count as f64]),
                    ("nodeUniform2", min.clone()),
                    ("nodeUniform3", vec![voxel_size]),
                    ("nodeUniform4", vec![grid[0] as f64]),
                    ("nodeUniform5", vec![grid[1] as f64]),
                    ("nodeUniform8", vec![triangle_count as f64]),
                ],
            )?;
            voxelize.push(kernel(
                wgsl!("voxelize_cs"),
                &object,
                vec![
                    (1, triangle_buffer.as_entire_binding()),
                    (2, occupancy.as_entire_binding()),
                    (3, triangle_ids.as_entire_binding()),
                ],
                groups(triangle_count),
            ));
        }
        let resolve_object = write(
            wgsl!("resolve_cs"),
            &[("nodeUniform0", vec![v]), ("nodeUniform3", vec![v])],
        )?;
        let opacity_levels: Vec<wgpu::TextureView> =
            (0..levels).map(|m| level(&opacity_texture, m)).collect();
        voxelize.push(kernel(
            wgsl!("resolve_cs"),
            &resolve_object,
            vec![
                (1, occupancy.as_entire_binding()),
                (2, res(&opacity_levels[0])),
            ],
            groups(voxels),
        ));
        let opacity_mips = [
            wgsl!("opacity_mip_1_cs"),
            wgsl!("opacity_mip_2_cs"),
            wgsl!("opacity_mip_3_cs"),
            wgsl!("opacity_mip_4_cs"),
        ];
        let radiance_mip_sources = [
            wgsl!("radiance_mip_1_cs"),
            wgsl!("radiance_mip_2_cs"),
            wgsl!("radiance_mip_3_cs"),
            wgsl!("radiance_mip_4_cs"),
        ];
        for m in 1..levels as usize {
            let count = mip_counts[m - 1];
            let source = opacity_mips[(m - 1).min(3)];
            let object = write(source, &[("nodeUniform2", vec![count as f64])])?;
            voxelize.push(kernel(
                source,
                &object,
                vec![
                    (0, res(&opacity_levels[m - 1])),
                    (1, res(&opacity_levels[m])),
                ],
                groups(count),
            ));
        }
        // _collectLights: the directional light's slot, the others Vector4().
        let lights = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vxgi lights"),
            size: 32 * 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        // The shadow map is read with textureSampleLevel: a filtering-free sampler.
        let point_sampler = r.device.create_sampler(&Default::default());
        let sampler = |mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        let (linear, trilinear) = (
            sampler(wgpu::FilterMode::Nearest),
            sampler(wgpu::FilterMode::Linear),
        );
        let radiance_levels: Vec<Vec<wgpu::TextureView>> = radiance_textures
            .iter()
            .map(|t| (0..levels).map(|m| level(t, m)).collect())
            .collect();
        let radiance_full: Vec<wgpu::TextureView> = radiance_textures.iter().map(view).collect();
        let inject_object = uniform(r, "vxgi inject", wgsl!("inject_cs"), "objectStruct")?;
        // The page's kernel writes the direct light and the ping-pong index; the
        // same kernel serves the other index for an even bounce count.
        let inject = [0, 1].map(|i| {
            kernel(
                wgsl!("inject_cs"),
                &inject_object,
                vec![
                    (1, occupancy.as_entire_binding()),
                    (2, triangle_ids.as_entire_binding()),
                    (3, triangle_buffer.as_entire_binding()),
                    (4, lights.as_entire_binding()),
                    (5, wgpu::BindingResource::Sampler(&point_sampler)),
                    (6, res(shadow)),
                    (7, res(&direct)),
                    (8, res(&radiance_levels[i][0])),
                ],
                groups(voxels),
            )
        });
        let mut radiance_mips: [Vec<Kernel>; 2] = [vec![], vec![]];
        for (i, mips) in radiance_mips.iter_mut().enumerate() {
            for m in 1..levels as usize {
                let count = mip_counts[m - 1];
                let source = radiance_mip_sources[(m - 1).min(3)];
                let object = write(source, &[("nodeUniform2", vec![count as f64])])?;
                mips.push(kernel(
                    source,
                    &object,
                    vec![
                        (0, res(&radiance_levels[i][m])),
                        (1, res(&radiance_levels[i][m - 1])),
                    ],
                    groups(count),
                ));
            }
        }
        let bounce_object = uniform(r, "vxgi bounce", wgsl!("bounce_cs"), "objectStruct")?;
        let opacity = view(&opacity_texture);
        // bounce[ target ] reads the other index and writes the target.
        let bounce = [0, 1].map(|target| {
            kernel(
                wgsl!("bounce_cs"),
                &bounce_object,
                vec![
                    (1, occupancy.as_entire_binding()),
                    (2, triangle_ids.as_entire_binding()),
                    (3, triangle_buffer.as_entire_binding()),
                    (4, wgpu::BindingResource::Sampler(&linear)),
                    (5, res(&direct)),
                    (6, wgpu::BindingResource::Sampler(&trilinear)),
                    (7, res(&opacity)),
                    (8, wgpu::BindingResource::Sampler(&trilinear)),
                    (9, res(&radiance_full[1 - target])),
                    (10, res(&radiance_levels[target][0])),
                ],
                groups(voxels),
            )
        });
        Ok(Self {
            grid,
            voxel_size,
            bounds_min,
            levels,
            voxelize,
            inject,
            radiance_mips,
            bounce,
            inject_object,
            bounce_object,
            lights,
            opacity,
            radiance: view(&radiance_textures[0]),
        })
    }
    fn volume_size(&self) -> Vector3 {
        Vector3::new(
            self.grid[0] as f64,
            self.grid[1] as f64,
            self.grid[2] as f64,
        ) * self.voxel_size
    }
    fn run(encoder: &mut wgpu::CommandEncoder, kernels: &[&Kernel]) {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        for (p, g, n) in kernels {
            pass.set_pipeline(p);
            pass.set_bind_group(0, g, &[]);
            pass.dispatch_workgroups(*n, 1, 1);
        }
    }
    /// _updateLighting: the light into the ping-pong index that ends in the
    /// traced texture, then each bounce with its mips.
    fn light(
        &self,
        r: &Renderer,
        encoder: &mut wgpu::CommandEncoder,
        key: &Lighting,
    ) -> Result<()> {
        let v = (self.grid[0] * self.grid[1] * self.grid[2]) as f64;
        let min = self.bounds_min.to_array().to_vec();
        let write =
            |buffer: &wgpu::Buffer, source: &str, values: &[(&str, Vec<f64>)]| -> Result<()> {
                let values: Vec<(&str, &[f64])> =
                    values.iter().map(|(n, v)| (*n, &v[..])).collect();
                r.queue
                    .write_buffer(buffer, 0, &pack(source, "objectStruct", &values)?);
                Ok(())
            };
        write(
            &self.inject_object,
            wgsl!("inject_cs"),
            &[
                ("nodeUniform0", vec![v]),
                ("nodeUniform4", min.clone()),
                ("nodeUniform5", vec![self.voxel_size]),
                ("nodeUniform7", m4(key.shadow_matrix)),
                (
                    "nodeUniform8",
                    vec![0., key.shadow_near, key.shadow_far, 0.],
                ),
                ("nodeUniform12", vec![v]),
            ],
        )?;
        write(
            &self.bounce_object,
            wgsl!("bounce_cs"),
            &[
                ("nodeUniform0", vec![v]),
                ("nodeUniform4", min),
                ("nodeUniform5", vec![self.voxel_size]),
                ("nodeUniform7", self.volume_size().to_array().to_vec()),
                ("nodeUniform8", vec![key.max_distance]),
                ("nodeUniform9", vec![60.]),
                ("nodeUniform10", vec![(self.levels - 1) as f64]),
                ("nodeUniform12", vec![key.step_scale]),
                ("nodeUniform15", vec![v]),
            ],
        )?;
        // The light: position and type 0, the direction toward the light,
        // color × intensity with decay 2, no cone; the other slots Vector4().
        let mut lights = vec![0f32; 128];
        let p = key.light_position;
        let d = (key.light_position - key.light_target).normalize();
        let c = key.light_color;
        lights[..12].copy_from_slice(&[
            p.x as f32,
            p.y as f32,
            p.z as f32,
            0.,
            d.x as f32,
            d.y as f32,
            d.z as f32,
            0.,
            c[0] as f32,
            c[1] as f32,
            c[2] as f32,
            2.,
        ]);
        for slot in 4..32 {
            lights[slot * 4 + 3] = 1.;
        }
        r.queue
            .write_buffer(&self.lights, 0, bytemuck::cast_slice(&lights));
        let bounces = key.bounces;
        let mut index = bounces % 2;
        let mut kernels: Vec<&Kernel> = vec![&self.inject[index]];
        kernels.extend(self.radiance_mips[index].iter());
        for _ in 0..bounces {
            let target = 1 - index;
            kernels.push(&self.bounce[target]);
            kernels.extend(self.radiance_mips[target].iter());
            index = target;
        }
        Self::run(encoder, &kernels);
        Ok(())
    }
}
/// The injection key: _collectLights's light state and the bounce settings.
#[derive(Clone, PartialEq)]
struct Lighting {
    light_position: Vector3,
    light_target: Vector3,
    light_color: [f64; 3],
    shadow_matrix: Matrix4,
    shadow_near: f64,
    shadow_far: f64,
    bounces: usize,
    step_scale: f64,
    max_distance: f64,
}

/// Whether a WGSL module reads a binding beyond declaring it ( the
/// derived layouts drop the unused ones ).
fn used(source: &str, name: &str) -> bool {
    source.matches(&format!("{name} ")).count()
        + source.matches(&format!("{name},")).count()
        + source.matches(&format!("{name})")).count()
        > 1
}
/// A material pipeline's group 1 bindings: `@binding( N ) @group( 1 ) var
/// NAME`, the object uniforms, each texture by its role and its sampler.
fn object_entries<'a>(
    v: &Variant,
    object: &'a wgpu::Buffer,
    resources: &'a Resources,
    mesh: Option<&'a Mesh>,
) -> Vec<(u32, wgpu::BindingResource<'a>)> {
    let mut entries = vec![];
    for source in [v.vs, v.fs] {
        for line in source.lines() {
            let Some(rest) = line.trim().strip_prefix("@binding( ") else {
                continue;
            };
            let Some((binding, rest)) = rest.split_once(" ) @group( 1 )") else {
                continue;
            };
            let Ok(binding) = binding.parse::<u32>() else {
                continue;
            };
            if entries.iter().any(|(b, _)| *b == binding) {
                continue;
            }
            let name = rest
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .trim_end_matches(':')
                .to_string();
            if rest.trim().is_empty() || name.is_empty() {
                // The object struct's binding: its `var<uniform> object` on the next line.
                entries.push((binding, object.as_entire_binding()));
                continue;
            }
            let (base, sampler) = match name.strip_suffix("_sampler") {
                Some(b) => (b.to_string(), true),
                None => (name.clone(), false),
            };
            if !used(v.fs, &name) && !used(v.vs, &name) {
                continue;
            }
            let Some(&(_, role)) = v.textures.iter().find(|(n, _)| *n == base) else {
                continue;
            };
            let map = |k: usize| mesh.and_then(|m| m.maps[k].as_ref());
            let resource = match (role, sampler) {
                (Role::Map, false) => map(0).map(|t| wgpu::BindingResource::TextureView(&t.view)),
                (Role::Map, true) => map(0).map(|t| wgpu::BindingResource::Sampler(&t.sampler)),
                (Role::RoughnessMetalness, false) => {
                    map(1).map(|t| wgpu::BindingResource::TextureView(&t.view))
                }
                (Role::RoughnessMetalness, true) => {
                    map(1).map(|t| wgpu::BindingResource::Sampler(&t.sampler))
                }
                (Role::NormalMap, false) => {
                    map(2).map(|t| wgpu::BindingResource::TextureView(&t.view))
                }
                (Role::NormalMap, true) => {
                    map(2).map(|t| wgpu::BindingResource::Sampler(&t.sampler))
                }
                (Role::Dfg, false) => Some(wgpu::BindingResource::TextureView(resources.dfg)),
                (Role::Shadow, false) => {
                    Some(wgpu::BindingResource::TextureView(&resources.shadow_depth))
                }
                (Role::Shadow, true) => Some(wgpu::BindingResource::Sampler(&resources.compare)),
                (Role::Ao, false) => resources.ao.map(wgpu::BindingResource::TextureView),
                (Role::Gi, false) => resources.gi.map(wgpu::BindingResource::TextureView),
                (_, true) => Some(wgpu::BindingResource::Sampler(&resources.linear)),
            };
            if let Some(resource) = resource {
                entries.push((binding, resource));
            }
        }
    }
    entries
}
/// The resources material bindings read beside a mesh's maps.
struct Resources<'a> {
    dfg: &'a wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    compare: wgpu::Sampler,
    linear: wgpu::Sampler,
    ao: Option<&'a wgpu::TextureView>,
    gi: Option<&'a wgpu::TextureView>,
}
/// The previous frame's matrices VelocityNode reads.
#[derive(Clone, Copy)]
struct Motion {
    projection: Matrix4,
    view: Matrix4,
}
/// TRAANode's camera matrices from its last resolve.
#[derive(Clone, Copy)]
struct Resolved {
    world: Matrix4,
    projection_inverse: Matrix4,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    /// The pre-pass MRT ( packed normal, velocity ) and its depth.
    prepass: [(wgpu::Texture, wgpu::TextureView); 2],
    prepass_depth: (wgpu::Texture, wgpu::TextureView),
    ao: wgpu::TextureView,
    gi: wgpu::TextureView,
    scene: (wgpu::Texture, wgpu::TextureView),
    scene_depth: wgpu::TextureView,
    resolve: (wgpu::Texture, wgpu::TextureView),
    history: wgpu::Texture,
    history_depth: wgpu::Texture,
    screen: RenderTarget,
    /// Per mesh: the pre-pass, scene and direct scene draws; the sky's.
    draws: Vec<[Draw; 3]>,
    sky: [Draw; 3],
    cone: Draw,
    /// Resolve with the placeholder, or the history, previous depth.
    traa: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// The outputs: the resolve, the scene ( Direct or without temporal
    /// filtering ), AO and GI.
    outputs: [Draw; 4],
}
pub(super) struct Demo {
    controls: FirstPerson,
    /// The GUI's: output, voxel view, voxel view level, cone count, cone
    /// angle, step scale, max distance, normal offset, AO distance, AO
    /// intensity, AO min visibility, GI intensity, bounces, directional
    /// radiance, temporal filtering, light azimuth, elevation and intensity
    /// and ambient intensity.
    params: [f64; 19],
    time: f64,
    last: f64,
    loaded: bool,
    /// A frame was requested ( the clock advanced ); other renders, as a
    /// GUI change requests, present the last output.
    pending: bool,
    frame_id: usize,
    jitter: usize,
    built: bool,
    resolves: usize,
    motion: Option<Motion>,
    resolved: Option<Resolved>,
    voxelized: bool,
    lighting: Option<Lighting>,
    meshes: Vec<Mesh>,
    sky: SkyBox,
    volume: Volume,
    model_center: Vector3,
    target_y: f64,
    light_base_distance: f64,
    shadow_extent: f64,
    shadow_far: f64,
    shadow_renders: [wgpu::Buffer; 2],
    shadow_draws: Vec<Draw>,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    prepass_renders: [wgpu::Buffer; 4],
    scene_renders: [wgpu::Buffer; 4],
    direct_renders: [wgpu::Buffer; 4],
    cone_object: wgpu::Buffer,
    traa_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    placeholder: wgpu::TextureView,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
fn layouts(
    kind: Option<Kind>,
    shadow: bool,
    sky_scene: bool,
) -> Vec<wgpu::VertexBufferLayout<'static>> {
    const UV: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];
    const POSITION_1: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];
    const NORMAL_1: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];
    const TANGENT_2: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![2 => Float32x4];
    const POSITION_2: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![2 => Float32x3];
    const POSITION_3: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![3 => Float32x3];
    const POSITION_0: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
    let b = |stride, attributes: &'static [wgpu::VertexAttribute]| wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes,
    };
    if shadow {
        return vec![b(8, &UV), b(12, &POSITION_1)];
    }
    match kind {
        None if sky_scene => vec![b(12, &POSITION_0)],
        None => vec![b(12, &POSITION_0), b(12, &NORMAL_1)],
        Some(Kind::Base) => vec![b(8, &UV), b(12, &NORMAL_1), b(12, &POSITION_2)],
        Some(_) => vec![
            b(8, &UV),
            b(12, &NORMAL_1),
            b(16, &TANGENT_2),
            b(12, &POSITION_3),
        ],
    }
}
/// A material pipeline: the variant's culling ( front faces clockwise for
/// the shadow and sky ) and depth.
fn material_pipeline(
    r: &Renderer,
    v: &Variant,
    kind: Option<Kind>,
    pass: usize,
    formats: &[wgpu::TextureFormat],
) -> wgpu::RenderPipeline {
    let shadow = pass == 0;
    let masked = kind == Some(Kind::Masked);
    let cw = (shadow && !masked) || kind.is_none();
    let cull = if masked { None } else { Some(wgpu::Face::Back) };
    culled_pipeline(
        r,
        "vxgi sponza material",
        (v.vs, v.fs),
        &layouts(kind, shadow, kind.is_none() && pass == 2),
        formats,
        Some((wgpu::CompareFunction::LessEqual, kind.is_some())),
        (cw, false),
        (1, wgpu::PrimitiveTopology::TriangleList),
        cull,
    )
}
/// The page's light: updateLightPosition() from the azimuth and elevation.
fn light_position(params: &[f64; 19], target: Vector3, base: f64) -> (Vector3, Vector3) {
    let elevation = params[16].to_radians();
    let azimuth = params[15].to_radians();
    let phi = PI / 2. - elevation;
    let sun = Vector3::new(
        phi.sin() * azimuth.sin(),
        phi.cos(),
        phi.sin() * azimuth.cos(),
    );
    (target + sun * base, sun)
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.1,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(-10.25, 4.99, 0.40);
        n.quaternion = Euler {
            angles: Vector3::new(1.6505, -1.5008, 1.6507),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        // FirstPersonControls( camera ): the orientation from the camera's look.
        let direction = s.get(c)?.quaternion * -Vector3::Z;
        let phi = (direction.y / direction.length()).clamp(-1., 1.).acos();
        let mut controls = FirstPerson::default();
        controls.speed = 2.;
        controls.lat = 90. - phi.to_degrees();
        controls.lon = direction.x.atan2(direction.z).to_degrees();
        // The model.
        let url = sponza_url().await?;
        let base = url.rsplit_once('/').ok_or(Error::Invalid("Sponza URL"))?.0;
        let (asset, buffers, images) = load_asset_bytes(&fetch(&url).await?, base).await?;
        let mut sponza = Scene::new();
        let nodes =
            crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(&mut sponza)?;
        sponza.update()?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let floats = |g: &BufferGeometry, name: &str, size: usize| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("Sponza attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..size).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let mut cache = crate::texture_gpu::TextureCache::default();
        let mut meshes = vec![];
        let mut sources = vec![];
        let mut downscaled: Vec<(usize, Arc<Vec<u8>>)> = vec![];
        // computeSceneBounds ( every vertex ) and Box3.setFromObject ( the
        // geometries' boxes ).
        let (mut lo, mut hi) = (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        );
        let (mut box_lo, mut box_hi) = (lo, hi);
        for h in nodes {
            let n = sponza.get(h)?;
            let NodeKind::Mesh(mesh) = &n.kind else {
                continue;
            };
            let Some(Material::Standard(m)) = mesh.materials.first().map(|m| m.as_ref()) else {
                continue;
            };
            let g = &mesh.geometry;
            let model = n.matrix_world;
            let positions = floats(g, "position", 3)?;
            let points: Vec<Vector3> = positions
                .chunks(3)
                .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            for p in &points {
                let w = Vector3::from_array(apply(&model, p.to_array()));
                lo = lo.min(w);
                hi = hi.max(w);
            }
            let (min, max) = points.iter().fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            );
            let world = Box3 { min, max }.transformed(model);
            box_lo = box_lo.min(world.min);
            box_hi = box_hi.max(world.max);
            let center = (min + max) * 0.5;
            let index: Vec<u32> = g
                .index
                .clone()
                .unwrap_or_else(|| (0..points.len() as u32).collect());
            let maps = [
                m.properties.map.as_ref(),
                m.metallic_roughness_map.as_ref(),
                m.normal_map.as_ref(),
            ];
            let kind = if maps[1].is_none() || maps[2].is_none() {
                Kind::Base
            } else if m.properties.alpha_test > 0. {
                Kind::Masked
            } else {
                Kind::Textured
            };
            let gpu = [0, 1, 2].map(|k| {
                maps[k]
                    .map(|t| cache.get(&r.device, &r.queue, t))
                    .transpose()
            });
            let [a, b, c2] = gpu;
            let uvs = floats(g, "uv", 2)?;
            let tangents = if kind == Kind::Base {
                vec![0f32; points.len() * 4]
            } else {
                floats(g, "tangent", 4)?
            };
            let color = m.properties.color.0.to_array();
            let variant = match kind {
                Kind::Textured => 0,
                Kind::Masked => 1,
                Kind::Base => 2,
            };
            let objects = [
                uniform(
                    r,
                    "vxgi shadow object",
                    SHADOWS[usize::from(kind == Kind::Masked)].fs,
                    "objectStruct",
                )?,
                uniform(
                    r,
                    "vxgi pre-pass object",
                    PREPASS[variant].fs,
                    "objectStruct",
                )?,
                uniform(r, "vxgi scene object", SCENE[variant].fs, "objectStruct")?,
                uniform(r, "vxgi direct object", DIRECT[variant].fs, "objectStruct")?,
            ];
            // The collector's map sample, shared per texture.
            let map = match maps[0] {
                Some(t) => {
                    let key = Arc::as_ptr(t) as usize;
                    let data = match downscaled.iter().find(|(k, _)| *k == key) {
                        Some((_, d)) => d.clone(),
                        None => {
                            let d = Arc::new(downscale(t).await?);
                            downscaled.push((key, d.clone()));
                            d
                        }
                    };
                    Some((data, t.srgb))
                }
                None => None,
            };
            sources.push(Source {
                positions: positions.clone(),
                uvs: uvs.clone(),
                index: index.clone(),
                model,
                color,
                map,
                alpha_test: m.properties.alpha_test,
                side: match m.properties.side {
                    Side::Back => 1.,
                    Side::Double => 2.,
                    Side::Front => 0.,
                },
            });
            meshes.push(Mesh {
                positions: init(
                    "vxgi positions",
                    bytemuck::cast_slice(&positions),
                    wgpu::BufferUsages::VERTEX,
                ),
                normals: init(
                    "vxgi normals",
                    bytemuck::cast_slice(&floats(g, "normal", 3)?),
                    wgpu::BufferUsages::VERTEX,
                ),
                uvs: init(
                    "vxgi uvs",
                    bytemuck::cast_slice(&uvs),
                    wgpu::BufferUsages::VERTEX,
                ),
                tangents: init(
                    "vxgi tangents",
                    bytemuck::cast_slice(&tangents),
                    wgpu::BufferUsages::VERTEX,
                ),
                index: init(
                    "vxgi index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                sphere: Sphere {
                    center,
                    radius: points.iter().map(|p| p.distance(center)).fold(0., f64::max),
                },
                model,
                kind,
                color,
                normal_scale: [m.normal_scale.x, m.normal_scale.y],
                alpha_test: m.properties.alpha_test,
                maps: [a?, b?, c2?],
                objects,
            });
        }
        // The page's lights around the model's box.
        let size = box_hi - box_lo;
        let model_center = (box_lo + box_hi) * 0.5;
        let target_y = model_center.y + size.y * 0.2;
        let light_base_distance = size.x.max(size.z);
        let shadow_extent = size.x.max(size.z) * 0.7;
        let shadow_far = size.y * 4.;
        // The volume around the scene bounds; the triangles inside it.
        let voxel = (hi - lo).max_element() / RESOLUTION;
        let multiple = 2f64.powi((RESOLUTION.log2().floor() - 2.).clamp(1., 8.) as i32 - 1);
        let bounds_min = lo - Vector3::splat(voxel);
        let volume_box = {
            let size = hi - lo;
            let axis = |s: f64| (((s / voxel).ceil() + 2.) / multiple).ceil() * multiple;
            (
                bounds_min,
                bounds_min + Vector3::new(axis(size.x), axis(size.y), axis(size.z)) * voxel,
            )
        };
        let triangles = collect_triangles(
            &sources,
            volume_box,
            (MAX_EDGE_SUBVOXELS * voxel * 0.5).powi(2),
        );
        // The directional shadow: a 2048² map and its color target.
        let shadow_color = view(&texture(r, (SHADOW, SHADOW, 1), BYTE));
        let shadow_depth = view(&texture(r, (SHADOW, SHADOW, 1), DEPTH));
        let volume = Volume::new(r, &triangles, (lo, hi), &shadow_depth)?;
        let shadow_pipelines = [0, 1].map(|i| {
            material_pipeline(
                r,
                &SHADOWS[i],
                Some(if i == 0 { Kind::Textured } else { Kind::Masked }),
                0,
                &[BYTE],
            )
        });
        let shadow_renders = [0, 1].map(|i| {
            uniform(
                r,
                "vxgi shadow render",
                source_of(&SHADOWS[i], "renderStruct"),
                "renderStruct",
            )
        });
        let [s0, s1] = shadow_renders;
        let shadow_renders = [s0?, s1?];
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let compare = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let shadow_draws = meshes
            .iter()
            .map(|m| {
                let i = usize::from(m.kind == Kind::Masked);
                let p = &shadow_pipelines[i];
                let resources = Resources {
                    dfg: &r.dfg,
                    shadow_depth: shadow_depth.clone(),
                    compare: compare.clone(),
                    linear: linear.clone(),
                    ao: None,
                    gi: None,
                };
                let entries = object_entries(&SHADOWS[i], &m.objects[0], &resources, Some(m));
                (
                    p.clone(),
                    vec![
                        bind(
                            r,
                            p.get_bind_group_layout(0),
                            &[(0, shadow_renders[i].as_entire_binding())],
                        ),
                        bind(r, p.get_bind_group_layout(1), &entries),
                    ],
                )
            })
            .collect();
        // SkyMesh: the unit box scaled 450000.
        let sky_geometry = BoxGeometry::build(1., 1., 1.)?;
        let sky = SkyBox {
            positions: init(
                "vxgi sky positions",
                bytemuck::cast_slice(&floats(&sky_geometry, "position", 3)?),
                wgpu::BufferUsages::VERTEX,
            ),
            normals: init(
                "vxgi sky normals",
                bytemuck::cast_slice(&floats(&sky_geometry, "normal", 3)?),
                wgpu::BufferUsages::VERTEX,
            ),
            index: init(
                "vxgi sky index",
                bytemuck::cast_slice(&sky_geometry.index.clone().unwrap_or_default()),
                wgpu::BufferUsages::INDEX,
            ),
            count: sky_geometry.index.as_ref().map_or(0, |i| i.len() as u32),
            model: Matrix4::from_scale(Vector3::splat(450000.)),
            objects: [
                uniform(r, "vxgi sky object", PREPASS[3].fs, "objectStruct")?,
                uniform(r, "vxgi sky object", SCENE[3].fs, "objectStruct")?,
                uniform(r, "vxgi sky object", DIRECT[3].fs, "objectStruct")?,
            ],
        };
        let renders = |variants: &[Variant; 4], label| -> Result<[wgpu::Buffer; 4]> {
            let [a, b, c, d] =
                [0, 1, 2, 3].map(|i| uniform(r, label, variants[i].fs, "renderStruct"));
            Ok([a?, b?, c?, d?])
        };
        // TRAANode's DepthTexture( 1, 1 ) placeholder, cleared to the far plane.
        let placeholder = view(&texture(r, (1, 1, 1), DEPTH));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("vxgi placeholder"),
            depth_stencil_attachment: depth(&placeholder),
            ..Default::default()
        });
        r.queue.submit([encoder.finish()]);
        Ok(Self {
            controls,
            params: [
                0., 0., 0., 3., 40., 0.5, 0., 1.5, 1., 1., 0., 1.5, 1., 0., 1., 135., 55., 100.,
                0.5,
            ],
            time: 0.,
            last: 0.,
            loaded: false,
            pending: true,
            frame_id: 0,
            jitter: 0,
            built: false,
            resolves: 0,
            motion: None,
            resolved: None,
            voxelized: false,
            lighting: None,
            meshes,
            sky,
            volume,
            model_center,
            target_y,
            light_base_distance,
            shadow_extent,
            shadow_far,
            shadow_renders,
            shadow_draws,
            shadow_color,
            shadow_depth,
            prepass_renders: renders(&PREPASS, "vxgi pre-pass render")?,
            scene_renders: renders(&SCENE, "vxgi scene render")?,
            direct_renders: renders(&DIRECT, "vxgi direct render")?,
            cone_object: uniform(r, "vxgi cone", wgsl!("cone_fs"), "objectStruct")?,
            traa_object: uniform(r, "vxgi traa", wgsl!("traa_fs"), "objectStruct")?,
            output_render: uniform(r, "vxgi output", wgsl!("output_fs"), "renderStruct")?,
            quad_uv: init(
                "vxgi quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                wgpu::BufferUsages::VERTEX,
            ),
            placeholder,
            linear,
            compare,
            targets: None,
        })
    }
}
impl Demo {
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let size = (width, height, 1);
        let make = |format| {
            let t = texture(r, size, format);
            let v = view(&t);
            (t, v)
        };
        let prepass = [make(HALF), make(HALF)];
        let prepass_depth = make(DEPTH);
        let ao = view(&texture(r, size, wgpu::TextureFormat::R8Unorm));
        let gi = view(&texture(r, size, HALF));
        let scene = make(HALF);
        let scene_depth = view(&texture(r, size, DEPTH));
        let resolve = make(HALF);
        let history = texture(r, size, HALF);
        let history_depth = texture(r, size, DEPTH);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let kinds = [
            Some(Kind::Textured),
            Some(Kind::Masked),
            Some(Kind::Base),
            None,
        ];
        let prepass_pipelines =
            [0, 1, 2, 3].map(|i| material_pipeline(r, &PREPASS[i], kinds[i], 1, &[HALF, HALF]));
        let scene_pipelines =
            [0, 1, 2, 3].map(|i| material_pipeline(r, &SCENE[i], kinds[i], 2, &[HALF]));
        let direct_pipelines =
            [0, 1, 2, 3].map(|i| material_pipeline(r, &DIRECT[i], kinds[i], 2, &[HALF]));
        let resources = Resources {
            dfg: &r.dfg,
            shadow_depth: self.shadow_depth.clone(),
            compare: self.compare.clone(),
            linear: self.linear.clone(),
            ao: Some(&ao),
            gi: Some(&gi),
        };
        let draw = |p: &wgpu::RenderPipeline,
                    v: &Variant,
                    render: &wgpu::Buffer,
                    object: &wgpu::Buffer,
                    mesh: Option<&Mesh>|
         -> Draw {
            let entries = object_entries(v, object, &resources, mesh);
            (
                p.clone(),
                vec![
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(r, p.get_bind_group_layout(1), &entries),
                ],
            )
        };
        let draws = self
            .meshes
            .iter()
            .map(|m| {
                let i = m.variant();
                [
                    draw(
                        &prepass_pipelines[i],
                        &PREPASS[i],
                        &self.prepass_renders[i],
                        &m.objects[1],
                        Some(m),
                    ),
                    draw(
                        &scene_pipelines[i],
                        &SCENE[i],
                        &self.scene_renders[i],
                        &m.objects[2],
                        Some(m),
                    ),
                    draw(
                        &direct_pipelines[i],
                        &DIRECT[i],
                        &self.direct_renders[i],
                        &m.objects[3],
                        Some(m),
                    ),
                ]
            })
            .collect();
        let sky = [
            draw(
                &prepass_pipelines[3],
                &PREPASS[3],
                &self.prepass_renders[3],
                &self.sky.objects[0],
                None,
            ),
            draw(
                &scene_pipelines[3],
                &SCENE[3],
                &self.scene_renders[3],
                &self.sky.objects[1],
                None,
            ),
            draw(
                &direct_pipelines[3],
                &DIRECT[3],
                &self.direct_renders[3],
                &self.sky.objects[2],
                None,
            ),
        ];
        let quad_attrs = [wgpu::vertex_attr_array![0 => Float32x2]];
        let quad = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &quad_attrs[0],
        }];
        let screen_pass = |label, vs, fs, formats: &[wgpu::TextureFormat]| {
            culled_pipeline(
                r,
                label,
                (vs, fs),
                &quad,
                formats,
                None,
                (false, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
                Some(wgpu::Face::Back),
            )
        };
        let trilinear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let cone_pipeline = screen_pass(
            "vxgi cone",
            wgsl!("cone_vs"),
            wgsl!("cone_fs"),
            &[wgpu::TextureFormat::R8Unorm, HALF],
        );
        let cone = (
            cone_pipeline.clone(),
            vec![bind(
                r,
                cone_pipeline.get_bind_group_layout(0),
                &[
                    (0, tex(&prepass_depth.1)),
                    (1, self.cone_object.as_entire_binding()),
                    (2, sampler(&self.linear)),
                    (3, tex(&prepass[0].1)),
                    (4, sampler(&trilinear)),
                    (5, tex(&self.volume.opacity)),
                    (6, sampler(&trilinear)),
                    (7, tex(&self.volume.radiance)),
                ],
            )],
        );
        let traa_pipeline = screen_pass("vxgi traa", wgsl!("traa_vs"), wgsl!("traa_fs"), &[HALF]);
        let history_view = view(&history);
        let history_depth_view = view(&history_depth);
        let traa_source = wgsl!("traa_fs");
        let traa_groups = [&self.placeholder, &history_depth_view].map(|previous| {
            let mut entries = vec![
                (1, tex(&prepass[1].1)),
                (2, sampler(&self.linear)),
                (3, tex(&scene.1)),
                (4, tex(&prepass_depth.1)),
                (5, self.traa_object.as_entire_binding()),
                (6, tex(previous)),
                (7, sampler(&self.linear)),
                (8, tex(&history_view)),
            ];
            if used(traa_source, "nodeUniform0_sampler") {
                entries.push((0, sampler(&self.linear)));
            }
            bind(r, traa_pipeline.get_bind_group_layout(0), &entries)
        });
        let format = out.options.format;
        let single = |fs, source: &wgpu::TextureView| -> Draw {
            let p = screen_pass("vxgi output", wgsl!("output_vs"), fs, &[format]);
            let groups = vec![
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, self.output_render.as_entire_binding())],
                ),
                bind(
                    r,
                    p.get_bind_group_layout(1),
                    &[
                        (0, sampler(&self.linear)),
                        (1, wgpu::BindingResource::TextureView(source)),
                    ],
                ),
            ];
            (p, groups)
        };
        let outputs = [
            single(wgsl!("output_fs"), &resolve.1),
            single(wgsl!("output_scene_fs"), &scene.1),
            single(wgsl!("output_ao_fs"), &ao),
            single(wgsl!("output_gi_fs"), &gi),
        ];
        let screen = RenderTarget::with_options(
            &r.device,
            width,
            height,
            RenderTargetOptions {
                samples: 0,
                depth_buffer: false,
                ..out.options.clone()
            },
        )?;
        self.targets = Some(Targets {
            width,
            height,
            format,
            prepass,
            prepass_depth,
            ao,
            gi,
            scene,
            scene_depth,
            resolve,
            history,
            history_depth,
            screen,
            draws,
            sky,
            cone,
            traa: (traa_pipeline, traa_groups),
            outputs,
        });
        Ok(())
    }
}
impl Demo {
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.pending = true;
        }
        Ok(())
    }
    /// animate(): controls.update( timer.getDelta() ).
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let delta = (self.time - self.last).max(0.);
        self.last = self.time;
        self.controls.update(s, c, delta)
    }
    /// The output: 0 the resolve, 1 the scene ( Direct, or Combined without
    /// temporal filtering ), 2 AO and 3 GI ( the GUI lists Combined, Direct,
    /// AO and GI ).
    fn output_index(&self) -> usize {
        match self.params[0].round() as usize {
            1 => 1,
            2 => 2,
            3 => 3,
            _ if self.params[14] < 0.5 => 1,
            _ => 0,
        }
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("vxgi output"),
            color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
            ..Default::default()
        });
        set(&mut pass, &t.outputs[self.output_index()]);
        pass.set_vertex_buffer(0, self.quad_uv.slice(..));
        pass.draw(0..3, 0..1);
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width || t.height != out.height || t.format != out.options.format
        }) {
            self.resize(r, out)?;
            self.resolves = 0;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("vxgi targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        if std::mem::replace(&mut self.loaded, true) {
            self.frame_id += 1;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        // updatePostprocessing's pipeline: Direct drops the GI context, AO and
        // GI show the effect alone, and TRAA resolves only the Combined view.
        let output = self.output_index();
        let mode = self.params[0].round() as usize;
        let traa_on = output == 0;
        let gi_on = mode != 1;
        let scene_on = mode <= 1;
        let jittered = if traa_on && self.built {
            let [x, y] = halton(self.jitter);
            let mut p = projection;
            p.z_axis.x += 2. * (x - 0.5) / t.width as f64;
            p.z_axis.y -= 2. * (y - 0.5) / t.height as f64;
            p
        } else {
            projection
        };
        let motion = self.motion.unwrap_or(Motion { projection, view });
        self.motion = Some(Motion { projection, view });
        // The light, its shadow camera and matrix.
        let p = &self.params;
        let target = Vector3::new(self.model_center.x, self.target_y, self.model_center.z);
        let (light, sun) = light_position(p, target, self.light_base_distance);
        let e = self.shadow_extent;
        let shadow_projection = Matrix4::orthographic_rh(-e, e, -e, e, 0.1, self.shadow_far);
        let shadow_view = Matrix4::look_at_rh(light, target, Vector3::Y);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let shadow_matrix = bias * shadow_projection * shadow_view;
        let light_rgb = Color::from_hex(0xfff2dc).0 * p[17];
        let frame = Frame {
            unjittered: projection,
            previous_projection: motion.projection,
            previous_view: motion.view,
            camera_position: world.w_axis.truncate(),
            size: [t.width as f64, t.height as f64],
            light_color: light_rgb.to_array(),
            light_position: light,
            light_target: target,
            shadow_matrix,
            ambient: [p[18]; 3],
            sun,
            time: self.time,
        };
        // The uniforms: per variant its render struct, per draw its object.
        for (v, buffer) in SHADOWS.iter().zip(&self.shadow_renders) {
            r.queue.write_buffer(
                buffer,
                0,
                &values(
                    v,
                    source_of(v, "renderStruct"),
                    "renderStruct",
                    &frame,
                    (shadow_projection, shadow_view),
                    Matrix4::IDENTITY,
                    None,
                )?,
            );
        }
        for (variants, renders) in [
            (&PREPASS, &self.prepass_renders),
            (&SCENE, &self.scene_renders),
            (&DIRECT, &self.direct_renders),
        ] {
            for (v, buffer) in variants.iter().zip(renders.iter()) {
                let source = source_of(v, "renderStruct");
                r.queue.write_buffer(
                    buffer,
                    0,
                    &values(
                        v,
                        source,
                        "renderStruct",
                        &frame,
                        (jittered, view),
                        Matrix4::IDENTITY,
                        None,
                    )?,
                );
            }
        }
        for m in &self.meshes {
            let i = m.variant();
            let shadow = &SHADOWS[usize::from(m.kind == Kind::Masked)];
            for (k, (v, camera)) in [
                (shadow, (shadow_projection, shadow_view)),
                (&PREPASS[i], (jittered, view)),
                (&SCENE[i], (jittered, view)),
                (&DIRECT[i], (jittered, view)),
            ]
            .into_iter()
            .enumerate()
            {
                r.queue.write_buffer(
                    &m.objects[k],
                    0,
                    &values(v, v.fs, "objectStruct", &frame, camera, m.model, Some(m))?,
                );
            }
        }
        for (k, v) in [&PREPASS[3], &SCENE[3], &DIRECT[3]].into_iter().enumerate() {
            r.queue.write_buffer(
                &self.sky.objects[k],
                0,
                &values(
                    v,
                    v.fs,
                    "objectStruct",
                    &frame,
                    (jittered, view),
                    self.sky.model,
                    None,
                )?,
            );
        }
        let [
            _,
            _,
            _,
            cone_count,
            cone_angle,
            step_scale,
            max_distance,
            normal_offset,
            ao_distance,
            ao_intensity,
            ao_min_visibility,
            gi_intensity,
            bounces,
            _,
            temporal,
            ..,
        ] = self.params;
        let frame_value = if temporal > 0.5 {
            (self.frame_id % TEMPORAL_CYCLE) as f64
        } else {
            0.
        };
        let volume = &self.volume;
        let write = |buffer: &wgpu::Buffer,
                     source: &str,
                     name: &str,
                     values: &[(&str, Vec<f64>)]|
         -> Result<()> {
            let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            r.queue
                .write_buffer(buffer, 0, &pack(source, name, &values)?);
            Ok(())
        };
        write(
            &self.cone_object,
            wgsl!("cone_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m4(jittered.inverse())),
                ("nodeUniform2", m4(world)),
                ("nodeUniform4", vec![frame_value]),
                ("nodeUniform5", vec![cone_count.round()]),
                ("nodeUniform6", vec![cone_angle]),
                ("nodeUniform7", vec![volume.voxel_size]),
                ("nodeUniform8", vec![normal_offset]),
                ("nodeUniform9", volume.bounds_min.to_array().to_vec()),
                ("nodeUniform10", volume.volume_size().to_array().to_vec()),
                ("nodeUniform11", vec![max_distance]),
                ("nodeUniform12", vec![ao_distance]),
                ("nodeUniform13", vec![(volume.levels - 1) as f64]),
                ("nodeUniform15", vec![step_scale]),
                ("nodeUniform17", vec![gi_intensity]),
                ("nodeUniform18", vec![ao_min_visibility]),
                ("nodeUniform19", vec![ao_intensity]),
                ("nodeUniform20", vec![0.]),
                ("nodeUniform21", world.w_axis.truncate().to_array().to_vec()),
                ("nodeUniform22", vec![0.]),
                ("nodeUniform23", vec![1.]),
            ],
        )?;
        let previous = self.resolved.unwrap_or(Resolved {
            world: Matrix4::IDENTITY,
            projection_inverse: Matrix4::IDENTITY,
        });
        self.resolved = Some(Resolved {
            world,
            projection_inverse: jittered.inverse(),
        });
        write(
            &self.traa_object,
            wgsl!("traa_fs"),
            "objectStruct",
            &[
                ("nodeUniform3", vec![0.1, 1000.]),
                ("nodeUniform4", m4(view)),
                ("nodeUniform5", m4(previous.world)),
                ("nodeUniform6", m4(previous.projection_inverse)),
                ("nodeUniform8", m3(Matrix4::IDENTITY)),
                ("nodeUniform10", m3(Matrix4::IDENTITY)),
                ("nodeUniform11", vec![1.]),
            ],
        )?;
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[("nodeUniform1", vec![1.])],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // The directional shadow, culled to its camera.
        {
            let frustum = Frustum::from_projection(shadow_projection * shadow_view);
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("vxgi shadow"),
                color_attachments: &[color(&self.shadow_color, wgpu::Color::BLACK)],
                depth_stencil_attachment: depth(&self.shadow_depth),
                ..Default::default()
            });
            for (m, d) in self.meshes.iter().zip(&self.shadow_draws) {
                if m.visible(&frustum) {
                    set(&mut pass, d);
                    m.draw(&mut pass, true);
                }
            }
        }
        // Opaque draws front to back, as the render list sorts them.
        let frustum = Frustum::from_projection(jittered * view);
        let mut order: Vec<(f64, usize)> = self
            .meshes
            .iter()
            .enumerate()
            .filter(|(_, m)| m.visible(&frustum))
            .map(|(k, m)| {
                let center = m.model.transform_point3(m.sphere.center);
                ((view * center.extend(1.)).z, k)
            })
            .collect();
        order.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        let black = wgpu::Color::BLACK;
        let sky_draw = |pass: &mut wgpu::RenderPass, d: &Draw, normals: bool| {
            set(pass, d);
            pass.set_vertex_buffer(0, self.sky.positions.slice(..));
            if normals {
                pass.set_vertex_buffer(1, self.sky.normals.slice(..));
            }
            pass.set_index_buffer(self.sky.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.sky.count, 0, 0..1);
        };
        if gi_on {
            // The pre-pass: packed normals and velocity.
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("vxgi pre-pass"),
                    color_attachments: &[
                        color(&t.prepass[0].1, black),
                        color(&t.prepass[1].1, black),
                    ],
                    depth_stencil_attachment: depth(&t.prepass_depth.1),
                    ..Default::default()
                });
                sky_draw(&mut pass, &t.sky[0], true);
                for &(_, k) in &order {
                    set(&mut pass, &t.draws[k][0]);
                    self.meshes[k].draw(&mut pass, false);
                }
            }
            // VXGIVolume.update: voxelize once, then light when the light or the
            // injection settings change.
            if !std::mem::replace(&mut self.voxelized, true) {
                let kernels: Vec<&Kernel> = self.volume.voxelize.iter().collect();
                Volume::run(&mut encoder, &kernels);
            }
            let key = Lighting {
                light_position: light,
                light_target: target,
                light_color: light_rgb.to_array(),
                shadow_matrix,
                shadow_near: 0.1,
                shadow_far: self.shadow_far,
                bounces: bounces.round().max(0.) as usize,
                step_scale,
                max_distance,
            };
            if self.lighting.as_ref() != Some(&key) {
                self.volume.light(r, &mut encoder, &key)?;
                self.lighting = Some(key);
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("vxgi cone"),
                    color_attachments: &[color(&t.ao, wgpu::Color::WHITE), color(&t.gi, black)],
                    ..Default::default()
                });
                set(&mut pass, &t.cone);
                pass.set_vertex_buffer(0, self.quad_uv.slice(..));
                pass.draw(0..3, 0..1);
            }
        }
        if scene_on {
            let variant = if mode == 1 { 2 } else { 1 };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("vxgi scene"),
                color_attachments: &[color(&t.scene.1, black)],
                depth_stencil_attachment: depth(&t.scene_depth),
                ..Default::default()
            });
            sky_draw(&mut pass, &t.sky[variant], false);
            for &(_, k) in &order {
                set(&mut pass, &t.draws[k][variant]);
                self.meshes[k].draw(&mut pass, false);
            }
        }
        if traa_on {
            let full = wgpu::Extent3d {
                width: t.width,
                height: t.height,
                depth_or_array_layers: 1,
            };
            if self.resolves == 0 {
                encoder.copy_texture_to_texture(
                    t.scene.0.as_image_copy(),
                    t.history.as_image_copy(),
                    full,
                );
            }
            let (pipeline, groups) = &t.traa;
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("vxgi traa"),
                    color_attachments: &[color(&t.resolve.1, black)],
                    ..Default::default()
                });
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &groups[usize::from(self.resolves >= 2)], &[]);
                pass.set_vertex_buffer(0, self.quad_uv.slice(..));
                pass.draw(0..3, 0..1);
            }
            encoder.copy_texture_to_texture(
                t.resolve.0.as_image_copy(),
                t.history.as_image_copy(),
                full,
            );
            encoder.copy_texture_to_texture(
                t.prepass_depth.0.as_image_copy(),
                t.history_depth.as_image_copy(),
                full,
            );
            self.resolves += 1;
            self.jitter = (self.jitter + 1) % 32;
            self.built = true;
        }
        self.present(&mut encoder, t);
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.controls.pointer(kind, x, y);
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.controls.key(code, down);
    }
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
    /// The GUI, in its order: output, voxel view, voxel view level, cone
    /// count, cone angle, step scale, max distance, normal offset, AO
    /// distance, intensity and min visibility, GI intensity, bounces,
    /// directional radiance, temporal filtering, light azimuth, elevation and
    /// intensity and ambient intensity. The voxel views and directional
    /// radiance are not reproduced.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            1 | 2 | 13 => return Err(Error::Invalid("vxgi parameter not reproduced")),
            // updatePostprocessing rebuilds the pipeline and the TRAA output.
            0 | 14 => self.built = false,
            _ => {}
        }
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("webgpu_vxgi_sponza parameter"))? = f64::from(value);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
