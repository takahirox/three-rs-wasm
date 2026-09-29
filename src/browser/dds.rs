//! webgl_loader_texture_dds: DDSLoader's DXT1/DXT3/DXT5 and BC6H block
//! textures and its uncompressed ARGB/RGB mip chains, uploaded once in their
//! own GPU formats (no RGBA decoding), on thirteen turning meshes; three
//! single-file cube maps are reflected by the torus and two boxes.
use super::controls_attributes::additive;
use super::gltf_viewer::fetch;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/dds";
/// DDSLoader.parse: the format, base size, mip count and each face's mip
/// chain (face-major). ARGB and RGB mips are reordered to RGBA as the
/// loader does; block data is kept as stored.
struct Dds {
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
    levels: u32,
    cube: bool,
    mips: Vec<Vec<u8>>,
}
fn parse_dds(data: &[u8]) -> Result<Dds> {
    let bad = |m: &'static str| Error::Asset(format!("DDS: {m}"));
    let int = |i: usize| -> Result<u32> {
        data.get(i * 4..i * 4 + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or(bad("truncated header"))
    };
    if int(0)? != 0x2053_4444 {
        return Err(bad("invalid magic number"));
    }
    let four_cc = |s: &[u8; 4]| u32::from_le_bytes(*s);
    let mut offset = int(1)? as usize + 4;
    // (format, block bytes); 0 block bytes marks 32-bit ARGB, 1 marks 24-bit RGB.
    let (format, block) = match int(21)? {
        f if f == four_cc(b"DXT1") => (wgpu::TextureFormat::Bc1RgbaUnorm, 8),
        f if f == four_cc(b"DXT3") => (wgpu::TextureFormat::Bc2RgbaUnorm, 16),
        f if f == four_cc(b"DXT5") => (wgpu::TextureFormat::Bc3RgbaUnorm, 16),
        f if f == four_cc(b"DX10") => {
            offset += 20;
            match int(32)? {
                95 => (wgpu::TextureFormat::Bc6hRgbUfloat, 16),
                96 => (wgpu::TextureFormat::Bc6hRgbFloat, 16),
                _ => return Err(bad("unsupported DXGI format")),
            }
        }
        _ => {
            let (bits, r, g, b, a) = (int(22)?, int(23)?, int(24)?, int(25)?, int(26)?);
            let rgb = r & 0xff0000 != 0 && g & 0xff00 != 0 && b & 0xff != 0;
            if bits == 32 && rgb && a & 0xff00_0000 != 0 {
                (wgpu::TextureFormat::Rgba8Unorm, 0)
            } else if bits == 24 && rgb {
                (wgpu::TextureFormat::Rgba8Unorm, 1)
            } else {
                return Err(bad("unsupported FourCC code"));
            }
        }
    };
    let levels = if int(2)? & 0x20000 != 0 {
        int(7)?.max(1)
    } else {
        1
    };
    let caps2 = int(28)?;
    let cube = caps2 & 0x200 != 0;
    if cube && caps2 & 0xfc00 != 0xfc00 {
        return Err(bad("incomplete cubemap faces"));
    }
    let (width, height) = (int(4)?, int(3)?);
    if width == 0 || height == 0 || levels > 32 {
        return Err(bad("dimensions"));
    }
    let mut mips = vec![];
    for _ in 0..if cube { 6 } else { 1 } {
        let (mut w, mut h) = (width as usize, height as usize);
        for _ in 0..levels {
            let length = match block {
                0 => w * h * 4,
                1 => w * h * 3,
                _ => w.max(4) / 4 * h.max(4) / 4 * block,
            };
            let source = data
                .get(offset..offset + length)
                .ok_or(bad("truncated data"))?;
            mips.push(match block {
                0 => source
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .flat_map(|p| [p[2], p[1], p[0], p[3]])
                    .collect(),
                1 => source
                    .as_chunks::<3>()
                    .0
                    .iter()
                    .flat_map(|p| [p[2], p[1], p[0], 255])
                    .collect(),
                _ => source.to_vec(),
            });
            offset += length;
            w = (w >> 1).max(1);
            h = (h >> 1).max(1);
        }
    }
    Ok(Dds {
        format,
        width,
        height,
        levels,
        cube,
        mips,
    })
}
/// A texture uploaded once in its DDS format with its sampler. `linear`
/// is the example's (or the loader's single-mip) min/magFilter override,
/// which samples the base level only; mip chains otherwise use trilinear
/// filtering with anisotropy 4, as WebGL applies it to mipmapped filters.
struct DdsTexture {
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}
async fn load(r: &Renderer, file: &str, srgb: bool, linear: bool) -> Result<DdsTexture> {
    let dds = parse_dds(&fetch(&format!("{ASSETS}/{file}")).await?)?;
    let format = if srgb {
        dds.format.add_srgb_suffix()
    } else {
        dds.format
    };
    let (bw, bh) = format.block_dimensions();
    if !dds.width.is_multiple_of(bw) || !dds.height.is_multiple_of(bh) {
        return Err(Error::Invalid("DDS block texture dimensions"));
    }
    let faces = if dds.cube { 6 } else { 1 };
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(file),
        size: wgpu::Extent3d {
            width: dds.width,
            height: dds.height,
            depth_or_array_layers: faces,
        },
        mip_level_count: dds.levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let block_bytes = format.block_copy_size(None).unwrap_or(4);
    for face in 0..faces {
        for level in 0..dds.levels {
            let w = (dds.width >> level).max(1);
            let h = (dds.height >> level).max(1);
            // Block copies cover the level's physical (block-rounded) size.
            let (columns, rows) = (w.div_ceil(bw), h.div_ceil(bh));
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
                &dds.mips[(face * dds.levels + level) as usize],
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(columns * block_bytes),
                    rows_per_image: Some(rows),
                },
                wgpu::Extent3d {
                    width: columns * bw,
                    height: rows * bh,
                    depth_or_array_layers: 1,
                },
            );
        }
    }
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(if dds.cube {
            wgpu::TextureViewDimension::Cube
        } else {
            wgpu::TextureViewDimension::D2
        }),
        ..Default::default()
    });
    let base_only = linear || dds.levels == 1;
    let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some(file),
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        lod_max_clamp: if base_only { 0. } else { 32. },
        anisotropy_clamp: if base_only || dds.cube { 1 } else { 4 },
        ..Default::default()
    });
    Ok(DdsTexture { view, sampler })
}
/// MeshBasicMaterial's per-vertex `vReflect` into local_normal. The
/// single-file cube maps are CompressedTextures, not CubeTextures, so
/// WebGL's flipEnvMap is +1 and the direction is not mirrored.
const REFLECT: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let v=normalize(surface.position-u.camera.xyz);let n=normalize((u.normal*vec4(surface.local_normal,0.0)).xyz);out.local_normal=reflect(v,n);return out;}";
const PLAIN: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
/// A MeshBasicMaterial of a map, an envMap or both (the default
/// MultiplyOperation with reflectivity 1: map × envMap), with the fragment
/// body's final `return` choosing the alpha handling.
async fn basic(
    r: &Renderer,
    map: Option<&DdsTexture>,
    env: Option<&DdsTexture>,
    body: &str,
) -> Result<ShaderMaterial> {
    let mut textures = vec![];
    let (mut types, mut dimensions) = (vec![], vec![]);
    let mut color = String::from("var c=vec4(1.0);");
    if let Some(t) = map {
        color += "c=textureSample(tsl_texture_0,tsl_sampler_0,fragment_surface.uv);";
        textures.push((&t.view, &t.sampler));
        types.push(Type::Texture);
        dimensions.push(wgpu::TextureViewDimension::D2);
    }
    if let Some(t) = env {
        let i = textures.len();
        color += &format!(
            "c=vec4(c.rgb*textureSample(tsl_texture_{i},tsl_sampler_{i},fragment_surface.local_normal).rgb,c.a);"
        );
        textures.push((&t.view, &t.sampler));
        types.push(Type::TextureCube);
        dimensions.push(wgpu::TextureViewDimension::Cube);
    }
    let node = WgslFn::new(
        "dds_basic",
        &format!("fn dds_basic()->vec4<f32>{{{color}{body}}}"),
        &[],
        Type::Vec4,
    )?
    .call(&[]);
    let program = ShaderProgram::with_projection_and_dimensions(
        r,
        &NodeMaterial::new(node).wgsl_with_texture_types(&types, &[])?,
        &textures,
        &dimensions,
        if env.is_some() { REFLECT } else { PLAIN },
    )
    .await?;
    Ok(ShaderMaterial::new(Arc::new(program)))
}
/// OPAQUE materials write alpha 1; the signed BC6H texels below zero
/// encode to 0 as WebGL's output does.
const OPAQUE: &str = "return vec4(max(c.rgb,vec3(0.0)),1.0);";
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
            far: 100.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., -2., 16.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        let map1 = load(r, "disturb_dxt1_nomip.dds", true, true).await?;
        let map2 = load(r, "disturb_dxt1_mip.dds", true, false).await?;
        let map3 = load(r, "hepatica_dxt3_mip.dds", true, false).await?;
        let map4 = load(r, "explosion_dxt5_mip.dds", true, false).await?;
        let map5 = load(r, "disturb_argb_nomip.dds", true, true).await?;
        let map6 = load(r, "disturb_argb_mip.dds", true, false).await?;
        let map7 = load(r, "disturb_dx10_bc6h_signed_nomip.dds", false, false).await?;
        let map8 = load(r, "disturb_dx10_bc6h_signed_mip.dds", false, false).await?;
        let map9 = load(r, "disturb_dx10_bc6h_unsigned_nomip.dds", false, false).await?;
        let map10 = load(r, "disturb_dx10_bc6h_unsigned_mip.dds", false, false).await?;
        let map11 = load(r, "wave_normals_24bit_uncompressed.dds", false, false).await?;
        let cube1 = load(r, "Mountains.dds", true, true).await?;
        let cube2 = load(r, "Mountains_argb_mip.dds", true, true).await?;
        let cube3 = load(r, "Mountains_argb_nomip.dds", true, true).await?;
        let material1 = basic(r, Some(&map1), Some(&cube1), OPAQUE).await?;
        let material2 = basic(r, Some(&map2), None, OPAQUE).await?;
        // alphaTest: 0.5, DoubleSide.
        let mut material3 = basic(
            r,
            Some(&map3),
            None,
            "if c.a<0.5 {discard;}return vec4(c.rgb,1.0);",
        )
        .await?;
        material3.properties.side = Side::Double;
        // AdditiveBlending, depthTest: false, transparent, DoubleSide: back
        // faces, then front faces.
        let mut material4 = basic(r, Some(&map4), None, "return c;").await?;
        material4.properties.transparent = true;
        material4.properties.blending = Some(additive());
        material4.properties.depth_test = false;
        material4.properties.side = Side::Double;
        material4.properties.force_single_pass = false;
        let material5 = basic(r, None, Some(&cube2), OPAQUE).await?;
        let material6 = basic(r, None, Some(&cube3), OPAQUE).await?;
        let mut materials = vec![
            material1, material2, material3, material4, material5, material6,
        ];
        for map in [&map5, &map6, &map7, &map8, &map9, &map10] {
            materials.push(basic(r, Some(map), None, OPAQUE).await?);
        }
        let mut material13 = basic(r, Some(&map11), None, "return c;").await?;
        material13.properties.transparent = true;
        materials.push(material13);
        let box_geometry = Arc::new(BoxGeometry::build(2., 2., 2.)?);
        let torus = Arc::new(TorusGeometry::build(
            1.,
            0.4,
            12,
            48,
            std::f64::consts::TAU,
            0.,
            std::f64::consts::TAU,
        )?);
        let positions = [
            (-10., -2.),
            (-6., -2.),
            (-6., 2.),
            (-10., 2.),
            (-2., 2.),
            (-2., -2.),
            (2., -2.),
            (2., 2.),
            (6., -2.),
            (6., 2.),
            (10., -2.),
            (10., 2.),
            (-10., -6.),
        ];
        let mut meshes = vec![];
        for (i, (material, (x, y))) in materials.into_iter().zip(positions).enumerate() {
            let geometry = if i == 0 {
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
        Err(Error::Invalid("dds parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
