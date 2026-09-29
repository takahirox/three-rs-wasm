//! webgpu_loader_texture_ktx2: the page's three sections of KTX2 textures, each
//! a plane in its own scene drawn into the viewport of its list item (only
//! when fully visible), under the scrolling page text. Uncompressed and block
//! formats are uploaded as stored; Basis Universal is transcoded to the
//! device's block format, as KTX2Loader chooses it.
use super::gltf_viewer::fetch;
use crate::shader::ShaderProgram;
use crate::texture_gpu::GpuTexture;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::sync::Arc;
use wasm_bindgen::JsCast;

const ASSETS: &str = "/web/gallery/assets/ktx2";
const SECTIONS: [(&str, &str, &[&str]); 3] = [
    (
        "Uncompressed",
        "Uncompressed formats (rgba8, rgba16, rgba32) load as THREE.DataTexture objects. Lossless, easy to read/write, uncompressed on GPU, optionally compressed over the network.",
        &[
            "2d_rgba8.ktx2",
            "2d_rgba8_linear.ktx2",
            "2d_rgba16_linear.ktx2",
            "2d_rgba32_linear.ktx2",
            "2d_rgb9e5_linear.ktx2",
            "2d_r11g11b10_linear.ktx2",
        ],
    ),
    (
        "Compressed",
        "Compressed formats (ASTC, BCn, ...) load as THREE.CompressedTexture objects, reducing memory cost. Requires native support on the device GPU: no single compressed format is supported on every device.",
        &[
            "2d_astc4x4.ktx2",
            "2d_etc1.ktx2",
            "2d_etc2.ktx2",
            "2d_bc1.ktx2",
            "2d_bc3.ktx2",
            "2d_bc4.ktx2",
            "2d_bc5.ktx2",
            "2d_bc7.ktx2",
        ],
    ),
    (
        "Universal",
        "Basis Universal textures are specialized intermediate formats supporting fast runtime transcoding into other GPU texture compression formats. After transcoding, universal textures can be used on any device at reduced memory cost.",
        &["2d_etc1s.ktx2", "2d_uastc.ktx2"],
    ),
];
/// The KTX2 container: format, size, the levels' byte ranges and the data
/// format descriptor's color space ( srgb, srgb-linear or none ).
struct Ktx2 {
    vk_format: u32,
    width: u32,
    height: u32,
    levels: Vec<(usize, usize)>,
    color_space: &'static str,
}
fn parse_ktx2(data: &[u8]) -> Result<Ktx2> {
    let bad = |m: &'static str| Error::Asset(format!("KTX2: {m}"));
    const IDENTIFIER: [u8; 12] = [
        0xab, 0x4b, 0x54, 0x58, 0x20, 0x32, 0x30, 0xbb, 0x0d, 0x0a, 0x1a, 0x0a,
    ];
    if data.get(..12) != Some(&IDENTIFIER) {
        return Err(bad("identifier"));
    }
    let u32_at = |o: usize| -> Result<u32> {
        data.get(o..o + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or(bad("truncated"))
    };
    let u64_at = |o: usize| -> Result<usize> {
        data.get(o..o + 8)
            .map(|b| u64::from_le_bytes(b.try_into().unwrap_or_default()) as usize)
            .ok_or(bad("truncated"))
    };
    let vk_format = u32_at(12)?;
    let (width, height) = (u32_at(20)?, u32_at(24)?);
    let level_count = u32_at(40)?.max(1) as usize;
    let supercompression = u32_at(44)?;
    let dfd = u32_at(48)? as usize;
    if vk_format != 0 && supercompression != 0 {
        return Err(bad("unsupported supercompression"));
    }
    let mut levels = vec![];
    for i in 0..level_count {
        let o = 80 + i * 24;
        let (offset, length) = (u64_at(o)?, u64_at(o + 8)?);
        if data.get(offset..offset + length).is_none() {
            return Err(bad("level range"));
        }
        levels.push((offset, length));
    }
    // The basic descriptor block: colorModel, colorPrimaries, transferFunction.
    let primaries = *data.get(dfd + 13).ok_or(bad("dfd"))?;
    let transfer = *data.get(dfd + 14).ok_or(bad("dfd"))?;
    let color_space = match primaries {
        1 if transfer == 2 => "srgb",
        1 => "srgb-linear",
        _ => "",
    };
    Ok(Ktx2 {
        vk_format,
        width,
        height,
        levels,
        color_space,
    })
}
/// FORMAT_MAP / TYPE_MAP for the example's vkFormats; true for block formats.
fn format(vk: u32) -> Result<(wgpu::TextureFormat, bool)> {
    use wgpu::TextureFormat as F;
    Ok(match vk {
        37 => (F::Rgba8Unorm, false),
        43 => (F::Rgba8UnormSrgb, false),
        97 => (F::Rgba16Float, false),
        109 => (F::Rgba32Float, false),
        122 => (F::Rg11b10Ufloat, false),
        123 => (F::Rgb9e5Ufloat, false),
        132 => (F::Bc1RgbaUnormSrgb, true),
        138 => (F::Bc3RgbaUnormSrgb, true),
        139 => (F::Bc4RUnorm, true),
        141 => (F::Bc5RgUnorm, true),
        146 => (F::Bc7RgbaUnormSrgb, true),
        148 => (F::Etc2Rgb8UnormSrgb, true),
        152 => (F::Etc2Rgba8UnormSrgb, true),
        158 => (
            F::Astc {
                block: wgpu::AstcBlock::B4x4,
                channel: wgpu::AstcChannel::UnormSrgb,
            },
            true,
        ),
        _ => return Err(Error::Asset(format!("KTX2: unsupported vkFormat {vk}"))),
    })
}
/// One texture uploaded as stored, with KTX2Loader's filters: nearest for
/// DataTextures, trilinear for CompressedTextures.
fn upload(
    r: &Renderer,
    name: &str,
    data: &[u8],
    k: &Ktx2,
) -> Result<(wgpu::Texture, wgpu::Sampler, bool)> {
    let (format, block) = format(k.vk_format)?;
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(name),
        size: wgpu::Extent3d {
            width: k.width,
            height: k.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: k.levels.len() as u32,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let (bw, bh) = format.block_dimensions();
    let bytes = format.block_copy_size(None).unwrap_or(4);
    for (level, &(offset, length)) in k.levels.iter().enumerate() {
        let w = (k.width >> level).max(1);
        let h = (k.height >> level).max(1);
        let (columns, rows) = (w.div_ceil(bw), h.div_ceil(bh));
        r.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: level as u32,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data[offset..offset + length],
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
    let filter = if block {
        wgpu::FilterMode::Linear
    } else {
        wgpu::FilterMode::Nearest
    };
    let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: filter,
        min_filter: filter,
        mipmap_filter: if block {
            wgpu::FilterMode::Linear
        } else {
            wgpu::FilterMode::Nearest
        },
        ..Default::default()
    });
    Ok((texture, sampler, format != wgpu::TextureFormat::Rgba32Float))
}
/// MeshBasicMaterial( { map } ) on the flipY'd plane: the map at ( u, 1 − v ).
const MAP: &str = "fn ktx2_map()->vec4<f32>{let uv=fragment_surface.uv;return vec4(textureSample(tsl_texture_0,tsl_sampler_0,vec2(uv.x,1.0-uv.y)).rgb,1.0);}";
/// The scissored clear to 0xe0e0e0: a triangle over the viewport.
const CLEAR: &str = "fn ktx2_clear()->vec4<f32>{return vec4(u.custom[0].xyz,1.0);}";
const FULLSCREEN: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.xy,0.0,1.0);return out;}";
struct View {
    element: web_sys::Element,
    scene: Scene,
    camera: Object3D,
}
pub(super) struct Demo {
    views: Vec<View>,
    /// The canvas-sized target the scenes draw into, and the white clear.
    target: Option<RenderTarget>,
    white: Scene,
    white_camera: Object3D,
    _textures: Vec<wgpu::Texture>,
}
impl Demo {
    pub async fn create(_s: &mut Scene, _c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let fail = |_| Error::Invalid("ktx2 page");
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or(Error::Invalid("document"))?;
        let content = document.create_element("div").map_err(fail)?;
        content.set_id("content");
        let info = document.create_element("div").map_err(fail)?;
        info.set_id("info");
        info.set_inner_html("<a href=\"https://threejs.org\" target=\"_blank\" rel=\"noopener\">three.js</a> - KTX2 texture loader");
        content.append_child(&info).map_err(fail)?;
        document
            .body()
            .ok_or(Error::Invalid("body"))?
            .append_child(&content)
            .map_err(fail)?;
        let plane = {
            let g = PlaneGeometry::build(1., 1., 1, 1)?;
            Arc::new(g)
        };
        let triangle = Arc::new(super::ssao::fullscreen_triangle()?);
        let clear_program = Arc::new(
            ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(WgslFn::new("ktx2_clear", CLEAR, &[], Type::Vec4)?.call(&[]))
                    .wgsl(0)?,
                &[],
                &[],
                FULLSCREEN,
            )
            .await?,
        );
        let clear_material = |color: Color| {
            let mut m = ShaderMaterial::new(clear_program.clone());
            m.uniforms[0] = color.0.as_vec3().extend(1.).to_array();
            m.properties.depth_test = false;
            m.properties.depth_write = false;
            Arc::new(Material::Shader(m))
        };
        let mut views = vec![];
        let mut textures = vec![];
        for (title, description, files) in SECTIONS {
            let section = document.create_element("section").map_err(fail)?;
            let header = document.create_element("h2").map_err(fail)?;
            header.set_text_content(Some(title));
            section.append_child(&header).map_err(fail)?;
            let paragraph = document.create_element("p").map_err(fail)?;
            paragraph.set_class_name("description");
            paragraph.set_text_content(Some(description));
            section.append_child(&paragraph).map_err(fail)?;
            for file in files {
                let item = document.create_element("div").map_err(fail)?;
                item.set_class_name("list-item");
                let element = document.create_element("div").map_err(fail)?;
                item.append_child(&element).map_err(fail)?;
                let label: web_sys::HtmlElement = document
                    .create_element("div")
                    .map_err(fail)?
                    .dyn_into()
                    .map_err(|_| Error::Invalid("ktx2 label"))?;
                label.set_inner_text(&format!("file: {file}"));
                item.append_child(&label).map_err(fail)?;
                section.append_child(&item).map_err(fail)?;
                let bytes = fetch(&format!("{ASSETS}/{file}")).await?;
                let k = parse_ktx2(&bytes)?;
                let (view, sampler, filterable) = if k.vk_format == 0 {
                    let gpu = GpuTexture::from_basis(r, &bytes, k.color_space == "srgb")?;
                    let (view, sampler) = (gpu.view.clone(), gpu.sampler.clone());
                    textures.push(gpu.texture.clone());
                    (view, sampler, true)
                } else {
                    let (texture, sampler, filterable) = upload(r, file, &bytes, &k)?;
                    let view = texture.create_view(&Default::default());
                    textures.push(texture);
                    (view, sampler, filterable)
                };
                let program = ShaderProgram::with_projection_and_sample_types(
                    r,
                    &NodeMaterial::new(WgslFn::new("ktx2_map", MAP, &[], Type::Vec4)?.call(&[]))
                        .wgsl_with_texture_types(&[Type::Texture], &[])?,
                    &[(&view, &sampler)],
                    &[wgpu::TextureViewDimension::D2],
                    &[wgpu::TextureSampleType::Float { filterable }],
                    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
                )
                .await?;
                let mut scene = Scene::new();
                scene.background = Color::from_hex(0xe0e0e0);
                let camera =
                    scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 50.,
                        aspect: 1.,
                        near: 1.,
                        far: 10.,
                        ..Default::default()
                    })));
                scene.get_mut(camera)?.position = Vector3::new(0., 0., 2.);
                let clear = scene.insert(NodeKind::Mesh(Mesh::new(
                    triangle.clone(),
                    clear_material(Color::from_hex(0xe0e0e0)),
                )));
                let n = scene.get_mut(clear)?;
                n.frustum_culled = false;
                n.render_order = -10000;
                scene.insert(NodeKind::Mesh(Mesh::new(
                    plane.clone(),
                    Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
                )));
                label.set_inner_text(&format!("file: {file}\ncolorSpace: {}", k.color_space));
                views.push(View {
                    element,
                    scene,
                    camera,
                });
            }
            content.append_child(&section).map_err(fail)?;
        }
        let mut white = Scene::new();
        white.background = Color::WHITE;
        let white_camera = white.insert(NodeKind::Camera(Camera::Perspective(
            PerspectiveCamera::default(),
        )));
        Ok(Self {
            views,
            target: None,
            white,
            white_camera,
            _textures: textures,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    /// animate(): the white clear, then each fully visible item's scene in
    /// its viewport and scissor (floored device pixels), over its 0xe0e0e0 clear.
    pub fn render(
        &mut self,
        r: &Renderer,
        _s: &mut Scene,
        _c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .target
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.target = Some(RenderTarget::with_options(
                &r.device,
                out.width,
                out.height,
                out.options.clone(),
            )?);
        }
        let target = self.target.as_mut().ok_or(Error::Invalid("ktx2 target"))?;
        target.viewport = [0, 0, target.width, target.height];
        target.scissor = None;
        target.set_load_color(false);
        r.render(&mut self.white, self.white_camera, target)?;
        let window = web_sys::window().ok_or(Error::Invalid("window"))?;
        let dpr = window.device_pixel_ratio();
        let (width, height) = (target.width as f64 / dpr, target.height as f64 / dpr);
        target.set_load_color(true);
        for view in &mut self.views {
            let rect = view.element.get_bounding_client_rect();
            if rect.top() < 0. || rect.bottom() > height || rect.left() < 0. || rect.right() > width
            {
                continue;
            }
            let px = |v: f64| (v * dpr).floor().max(0.) as u32;
            let area = [
                px(rect.left()),
                px(rect.top()),
                px(rect.width()),
                px(rect.height()),
            ];
            target.viewport = area;
            target.scissor = Some(area);
            r.render(&mut view.scene, view.camera, target)?;
        }
        target.viewport = [0, 0, target.width, target.height];
        target.scissor = None;
        target.set_load_color(false);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.target.as_ref()
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
        Err(Error::Invalid("ktx2 parameter"))
    }
    pub fn seek(&mut self, _t: f64) {}
}
