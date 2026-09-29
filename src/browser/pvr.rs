//! webgl_loader_texture_pvrtc: PVRLoader's v2 and v3 PVRTC 2/4 bpp maps
//! (with and without mips, opaque and with alpha) on six turning boxes, and
//! two cube maps reflected by the tori. WebGPU has no PVRTC formats: each
//! level is decoded once at load (see `pvrtc`) and stays resident as RGBA8.
//! WebGL has no sRGB PVRTC formats either, so the texels are sampled as
//! stored whatever the textures' color space, as there.
use super::dds::{DdsTexture, OPAQUE, basic};
use super::gltf_viewer::fetch;
use super::pvrtc::decode;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/pvrtc";
/// PVRLoader.parse: size, bpp, faces and each ( face, level ) payload.
pub(super) struct Pvr {
    pub width: u32,
    pub height: u32,
    pub faces: u32,
    pub levels: u32,
    /// Face-major mips.
    pub mips: Vec<Vec<u8>>,
}
fn parse_pvr(data: &[u8]) -> Result<Pvr> {
    let bad = |m: &'static str| Error::Asset(format!("PVR: {m}"));
    let int = |i: usize| -> Result<u32> {
        data.get(i * 4..i * 4 + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or(bad("truncated header"))
    };
    let (offset, bpp, width, height, faces, levels) = if int(0)? == 0x0352_5650 {
        let bpp = match int(2)? {
            0 | 1 => 2,
            2 | 3 => 4,
            _ => return Err(bad("unsupported pixel format")),
        };
        (
            52 + int(12)? as usize,
            bpp,
            int(7)?,
            int(6)?,
            int(10)?,
            int(11)?,
        )
    } else if int(11)? == 0x2152_5650 {
        let bpp = match int(4)? & 0xff {
            24 => 2,
            25 => 4,
            _ => return Err(bad("unsupported format flags")),
        };
        (
            int(0)? as usize,
            bpp,
            int(2)?,
            int(1)?,
            int(12)?,
            int(3)? + 1,
        )
    } else {
        return Err(bad("unknown format"));
    };
    if width == 0 || height == 0 || levels == 0 || levels > 16 || !(1..=6).contains(&faces) {
        return Err(bad("dimensions"));
    }
    let (bw, bh) = if bpp == 2 { (8, 4) } else { (4, 4) };
    let block = bw * bh * bpp / 8;
    let mut mips = vec![vec![]; (faces * levels) as usize];
    let mut at = offset;
    // _extract: every face of a level, level by level; at least 2 × 2 blocks.
    for level in 0..levels {
        let (w, h) = ((width >> level).max(1), (height >> level).max(1));
        let size = ((w / bw).max(2) * (h / bh).max(2) * block) as usize;
        for face in 0..faces {
            let bytes = data.get(at..at + size).ok_or(bad("truncated data"))?;
            mips[(face * levels + level) as usize] = decode(bytes, w, h, bpp);
            at += size;
        }
    }
    Ok(Pvr {
        width,
        height,
        faces,
        levels,
        mips,
    })
}
/// Decoded RGBA8 levels as one resident texture (a cube for six faces).
/// `linear` is a Linear minFilter, which samples the base level only; the
/// mip chain is sampled trilinearly without anisotropy.
pub(super) fn upload(r: &Renderer, name: &str, pvr: &Pvr, linear: bool) -> DdsTexture {
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(name),
        size: wgpu::Extent3d {
            width: pvr.width,
            height: pvr.height,
            depth_or_array_layers: pvr.faces,
        },
        mip_level_count: pvr.levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    for face in 0..pvr.faces {
        for level in 0..pvr.levels {
            let (w, h) = ((pvr.width >> level).max(1), (pvr.height >> level).max(1));
            r.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: level,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: face,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &pvr.mips[(face * pvr.levels + level) as usize],
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(w * 4),
                    rows_per_image: Some(h),
                },
                wgpu::Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: 1,
                },
            );
        }
    }
    let cube = pvr.faces == 6;
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(if cube {
            wgpu::TextureViewDimension::Cube
        } else {
            wgpu::TextureViewDimension::D2
        }),
        ..Default::default()
    });
    let base_only = linear || pvr.levels == 1;
    let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some(name),
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        lod_max_clamp: if base_only { 0. } else { 32. },
        anisotropy_clamp: 1,
        ..Default::default()
    });
    DdsTexture { view, sampler }
}
async fn load(r: &Renderer, file: &str, linear: bool) -> Result<DdsTexture> {
    let pvr = parse_pvr(&fetch(&format!("{ASSETS}/{file}")).await?)?;
    Ok(upload(r, file, &pvr, linear))
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
        let disturb_4 = load(r, "disturb_4bpp_rgb.pvr", true).await?;
        let disturb_4_v3 = load(r, "disturb_4bpp_rgb_v3.pvr", true).await?;
        let disturb_4_mips = load(r, "disturb_4bpp_rgb_mips.pvr", false).await?;
        let disturb_2 = load(r, "disturb_2bpp_rgb.pvr", true).await?;
        let flare_4 = load(r, "flare_4bpp_rgba.pvr", true).await?;
        let flare_2 = load(r, "flare_2bpp_rgba.pvr", true).await?;
        let cube_1 = load(r, "park3_cube_nomip_4bpp_rgb.pvr", true).await?;
        let cube_2 = load(r, "park3_cube_mip_2bpp_rgb_v3.pvr", true).await?;
        // side: DoubleSide, depthTest: false, transparent: back faces, then
        // front faces.
        let flare = |mut m: ShaderMaterial| {
            m.properties.transparent = true;
            m.properties.depth_test = false;
            m.properties.side = Side::Double;
            m.properties.force_single_pass = false;
            m
        };
        let materials = [
            basic(r, Some(&disturb_4), None, OPAQUE).await?,
            basic(r, Some(&disturb_4_mips), None, OPAQUE).await?,
            basic(r, Some(&disturb_2), None, OPAQUE).await?,
            basic(r, Some(&disturb_4_v3), None, OPAQUE).await?,
            flare(basic(r, Some(&flare_4), None, "return c;").await?),
            flare(basic(r, Some(&flare_2), None, "return c;").await?),
            basic(r, None, Some(&cube_1), OPAQUE).await?,
            basic(r, None, Some(&cube_2), OPAQUE).await?,
        ];
        let box_geometry = Arc::new(BoxGeometry::build(200., 200., 200.)?);
        let torus = Arc::new(TorusGeometry::build(
            100.,
            50.,
            32,
            24,
            std::f64::consts::TAU,
            0.,
            std::f64::consts::TAU,
        )?);
        let positions = [
            (-500., 200.),
            (-166., 200.),
            (166., 200.),
            (500., 200.),
            (-500., -200.),
            (-166., -200.),
            (166., -200.),
            (500., -200.),
        ];
        let mut meshes = vec![];
        for (i, (material, (x, y))) in materials.into_iter().zip(positions).enumerate() {
            let geometry = if i >= 6 {
                torus.clone()
            } else {
                box_geometry.clone()
            };
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                geometry,
                Arc::new(Material::Shader(material)),
            )));
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
        Err(Error::Invalid("pvrtc parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
