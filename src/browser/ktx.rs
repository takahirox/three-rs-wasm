//! webgl_loader_texture_ktx: KTXLoader's KTX 1 files in every block format
//! the device reports (PVRTC, S3TC, ETC1, ETC2/EAC and ASTC, as the original
//! queries its WebGL extensions), on turning boxes: basic color maps, the
//! lens flare maps drawn over everything and two block-compressed normal
//! maps under a point light. The block formats are uploaded as stored;
//! WebGPU has no PVRTC, so those levels are decoded once at load (`pvrtc`).
use super::dds::{DdsTexture, OPAQUE, basic};
use super::gltf_viewer::fetch;
use super::pvrtc::decode;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::sync::Arc;
use wasm_bindgen::JsCast;

const ASSETS: &str = "/web/gallery/assets/ktx";
/// KTXLoader.parse: glInternalFormat, base size and each level's image.
struct Ktx {
    internal: u32,
    width: u32,
    height: u32,
    levels: Vec<Vec<u8>>,
}
fn parse_ktx(data: &[u8]) -> Result<Ktx> {
    let bad = |m: &'static str| Error::Asset(format!("KTX: {m}"));
    const IDENTIFIER: [u8; 12] = [
        0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
    ];
    if data.get(..12) != Some(&IDENTIFIER) {
        return Err(bad("missing KTX identifier"));
    }
    let int = |i: usize| -> Result<u32> {
        let o = 12 + i * 4;
        data.get(o..o + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or(bad("truncated header"))
    };
    if int(0)? != 0x0403_0201 {
        return Err(bad("big-endian files"));
    }
    let (internal, width, height) = (int(4)?, int(6)?, int(7)?);
    let (faces, levels) = (int(10)?, int(11)?.max(1));
    if faces != 1 || width == 0 || height == 0 || levels > 16 {
        return Err(bad("dimensions"));
    }
    let mut offset = 64 + int(12)? as usize;
    let mut out = vec![];
    for _ in 0..levels {
        let size = data
            .get(offset..offset + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
            .ok_or(bad("truncated level"))?;
        offset += 4;
        out.push(
            data.get(offset..offset + size)
                .ok_or(bad("truncated level"))?
                .to_vec(),
        );
        // mipPadding: each image is 4-byte aligned.
        offset += size + (3 - (size + 3) % 4);
    }
    Ok(Ktx {
        internal,
        width,
        height,
        levels: out,
    })
}
/// The WebGPU format of a glInternalFormat (the sRGB variant where the
/// original marks the texture SRGBColorSpace and WebGL has one), or None
/// for PVRTC ( 2 or 4 bpp ), which is decoded.
fn format(internal: u32, srgb: bool) -> Result<std::result::Result<wgpu::TextureFormat, u32>> {
    use wgpu::{AstcBlock, AstcChannel, TextureFormat as F};
    let astc = |block| F::Astc {
        block,
        channel: if srgb {
            AstcChannel::UnormSrgb
        } else {
            AstcChannel::Unorm
        },
    };
    let format = match internal {
        0x8c00 | 0x8c02 => return Ok(Err(4)),
        0x8c01 | 0x8c03 => return Ok(Err(2)),
        0x83f0 | 0x83f1 => F::Bc1RgbaUnorm,
        0x83f3 => F::Bc3RgbaUnorm,
        0x8dbd => F::Bc5RgUnorm,
        0x8d64 => F::Etc2Rgb8Unorm,
        0x9272 => F::EacRg11Unorm,
        0x93b0 => astc(AstcBlock::B4x4),
        0x93b7 => astc(AstcBlock::B8x8),
        _ => return Err(Error::Asset(format!("KTX: glInternalFormat {internal:#x}"))),
    };
    Ok(Ok(if srgb {
        format.add_srgb_suffix()
    } else {
        format
    }))
}
/// A KTX color map with CompressedTextureLoader's filters: trilinear over
/// the stored mips.
async fn load(r: &Renderer, file: &str, srgb: bool) -> Result<DdsTexture> {
    let k = parse_ktx(&fetch(&format!("{ASSETS}/{file}")).await?)?;
    let levels = k.levels.len() as u32;
    let (format, bpp) = match format(k.internal, srgb)? {
        Ok(format) => (format, None),
        Err(bpp) => (wgpu::TextureFormat::Rgba8Unorm, Some(bpp)),
    };
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(file),
        size: wgpu::Extent3d {
            width: k.width,
            height: k.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let (bw, bh) = format.block_dimensions();
    let bytes = format.block_copy_size(None).unwrap_or(4);
    for (level, data) in k.levels.iter().enumerate() {
        let (w, h) = ((k.width >> level).max(1), (k.height >> level).max(1));
        let decoded = bpp.map(|bpp| decode(data, w, h, bpp));
        let (columns, rows) = (w.div_ceil(bw), h.div_ceil(bh));
        r.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: level as u32,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            decoded.as_deref().unwrap_or(data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(columns * bytes),
                rows_per_image: Some(rows),
            },
            wgpu::Extent3d {
                width: columns * bw,
                height: rows * bh,
                depth_or_array_layers: 1,
            },
        );
    }
    let view = texture.create_view(&Default::default());
    let base_only = levels == 1;
    let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some(file),
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        lod_max_clamp: if base_only { 0. } else { 32. },
        anisotropy_clamp: 1,
        ..Default::default()
    });
    Ok(DdsTexture { view, sampler })
}
/// MeshStandardMaterial( { normalMap } ): a core CompressedTexture uploaded
/// as stored (the two-channel formats sample ( r, g, 0 )).
async fn normal_map(file: &str) -> Result<Arc<Texture>> {
    let k = parse_ktx(&fetch(&format!("{ASSETS}/{file}")).await?)?;
    let format = format(k.internal, false)?.map_err(|_| Error::Asset("KTX normal map".into()))?;
    let mut t = Texture::from_rgba(1, 1, vec![0; 4], false)?;
    t.rgba = vec![];
    t.width = k.width;
    t.height = k.height;
    t.flip_y = false;
    if k.levels.len() > 1 {
        t.mipmap_filter = Some(Filter::Linear);
    }
    t.blocks = Some(Arc::new(BlockMips {
        format,
        levels: k.levels,
    }));
    Ok(Arc::new(t))
}
/// renderer.extensions.has( … ) of the original's WebGL context.
fn extensions() -> Vec<String> {
    let names = (|| {
        let canvas = web_sys::OffscreenCanvas::new(1, 1).ok()?;
        let gl: web_sys::WebGl2RenderingContext =
            canvas.get_context("webgl2").ok()??.dyn_into().ok()?;
        let list = gl.get_supported_extensions()?;
        Some(list.iter().filter_map(|v| v.as_string()).collect())
    })();
    names.unwrap_or_default()
}
pub(super) struct Demo {
    time: f64,
    meshes: Vec<Object3D>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 1.,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 1000.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.02,
        }));
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 2.,
            distance: 0.,
            decay: 0.,
        }));
        s.get_mut(light)?.position.z = 300.;
        let has = {
            let list = extensions();
            move |name: &str| list.iter().any(|e| e == name)
        };
        let features = r.device.features();
        // depthTest: false, transparent, DoubleSide: back faces, then front.
        let flare = |mut m: ShaderMaterial| {
            m.properties.transparent = true;
            m.properties.depth_test = false;
            m.properties.side = Side::Double;
            m.properties.force_single_pass = false;
            Material::Shader(m)
        };
        let color = async |file: &str, srgb: bool| -> Result<Material> {
            Ok(Material::Shader(
                basic(r, Some(&load(r, file, srgb).await?), None, OPAQUE).await?,
            ))
        };
        let lens = async |file: &str| -> Result<Material> {
            Ok(flare(
                basic(r, Some(&load(r, file, true).await?), None, "return c;").await?,
            ))
        };
        let standard = async |file: &str| -> Result<Material> {
            Ok(Material::Standard(MeshStandardMaterial {
                normal_map: Some(normal_map(file).await?),
                ..Default::default()
            }))
        };
        let mut materials = vec![];
        if has("WEBGL_compressed_texture_pvrtc") {
            materials.push(color("disturb_PVR2bpp.ktx", true).await?);
            materials.push(lens("lensflare_PVR4bpp.ktx").await?);
        }
        if has("WEBGL_compressed_texture_s3tc")
            && features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC)
        {
            materials.push(color("disturb_BC1.ktx", true).await?);
            materials.push(lens("lensflare_BC3.ktx").await?);
            materials.push(standard("normal.bc5.ktx").await?);
        }
        let etc = features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2);
        if has("WEBGL_compressed_texture_etc1") && etc {
            materials.push(color("disturb_ETC1.ktx", false).await?);
        }
        if has("WEBGL_compressed_texture_etc") && etc {
            materials.push(standard("normal.eac_rg.ktx").await?);
        }
        if has("WEBGL_compressed_texture_astc")
            && features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC)
        {
            materials.push(color("disturb_ASTC4x4.ktx", true).await?);
            materials.push(lens("lensflare_ASTC8x8.ktx").await?);
        }
        let geometry = Arc::new(BoxGeometry::build(200., 200., 200.)?);
        let count = materials.len();
        let (top, bottom) = (count.min(4), count.saturating_sub(4));
        let mut meshes = vec![];
        for (i, material) in materials.into_iter().enumerate() {
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(material),
            )));
            let (x, y) = if i < 4 {
                (-(top as f64) * 300. / 2. + 150. + i as f64 * 300., 150.)
            } else {
                (
                    -(bottom as f64) * 300. / 2. + 150. + (i - 4) as f64 * 300.,
                    -150.,
                )
            };
            s.get_mut(mesh)?.position = Vector3::new(x, y, 0.);
            meshes.push(mesh);
        }
        Ok(Self { time: 0., meshes })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): every mesh's rotation.x and rotation.y follow the clock.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let t = self.time;
        let q = Quaternion::from_euler(glam::EulerRot::XYZ, t, t, 0.);
        for &mesh in &self.meshes {
            s.get_mut(mesh)?.quaternion = q;
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("ktx parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
