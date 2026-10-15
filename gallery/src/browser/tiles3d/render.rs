//! The page's frame with the programs three compiles for it: the tiles
//! ( MeshBasicMaterial with the fade plugin's dither ) into the HalfFloat
//! output buffer with its depth texture, then renderer.setEffects' passes —
//! NormalPass, EffectPass( clouds, aerial perspective ) with CloudsEffect's
//! shadow and clouds passes, the lens flare, SMAA and dithering — and
//! WebGLOutput's AgX pass into the canvas. Every render target holds rows as
//! WebGL stores them ( the programs flip clip y ); the canvas pass does not.
use super::clouds;
use super::program::{
    Attachment, Block, Bound, LINEAR, LINEAR_REPEAT, NEAREST, NEAREST_REPEAT, PipelineKey, Program,
    SamplerKey, Samplers, Vertices, fullscreen_buffer, target,
};
use super::volume::{M4, V3, apply4, invert, multiply};
use crate::{Error, Result, renderer::Renderer};
use std::rc::Rc;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const RED_HALF: wgpu::TextureFormat = wgpu::TextureFormat::R16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
/// NormalBlending: ( SrcAlpha, OneMinusSrcAlpha ) color, ( One, OneMinusSrcAlpha ) alpha.
const NORMAL_BLENDING: wgpu::BlendState = wgpu::BlendState {
    color: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::SrcAlpha,
        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        operation: wgpu::BlendOperation::Add,
    },
    alpha: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        operation: wgpu::BlendOperation::Add,
    },
};
/// renderer.setClearColor's default ( 0, 0, 0, 1 ), which every pass clears to.
const CLEAR: wgpu::Color = wgpu::Color {
    r: 0.,
    g: 0.,
    b: 0.,
    a: 1.,
};

/// The camera as three uploads it.
#[derive(Clone, Copy)]
pub(crate) struct View {
    pub world: M4,
    pub view: M4,
    pub projection: M4,
    pub near: f64,
    pub far: f64,
    pub aspect: f64,
}
impl View {
    fn position(&self) -> V3 {
        [self.world[12], self.world[13], self.world[14]]
    }
}

/// A tile mesh's GPU geometry ( uploaded once ), map and world matrix.
pub(crate) struct TileMesh {
    position: wgpu::Buffer,
    normal: wgpu::Buffer,
    uv: wgpu::Buffer,
    count: u32,
    map: crate::texture_gpu::GpuTexture,
    pub world: M4,
    /// geometry.boundingSphere ( local ).
    sphere: (V3, f64),
    bindings: Option<MeshBindings>,
}

struct MeshBindings {
    color: (Block, wgpu::Buffer, wgpu::BindGroup),
    fade: (Block, wgpu::Buffer, wgpu::BindGroup),
    normal: (Block, wgpu::Buffer, wgpu::BindGroup),
}

impl TileMesh {
    pub fn new(
        r: &Renderer,
        positions: &[f32],
        normals: &[f32],
        uvs: &[f32],
        map: &std::sync::Arc<crate::material::Texture>,
        world: M4,
    ) -> Result<Self> {
        let buffer = |label, data: &[f32]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        };
        let sphere = super::content::bounding_sphere(positions);
        Ok(TileMesh {
            position: buffer("tile position", positions),
            normal: buffer("tile normal", normals),
            uv: buffer("tile uv", uvs),
            count: (positions.len() / 3) as u32,
            map: r.upload_texture(map)?,
            world,
            sphere,
            bindings: None,
        })
    }
    /// Frustum.intersectsObject: the world bounding sphere against the planes.
    fn in_frustum(&self, planes: &[[f64; 4]; 6]) -> bool {
        let c = apply4(self.sphere.0, &self.world);
        let radius = self.sphere.1 * super::volume::max_scale(&self.world);
        planes
            .iter()
            .all(|p| p[0] * c[0] + p[1] * c[1] + p[2] * c[2] + p[3] >= -radius)
    }
}

/// Frustum.setFromProjectionMatrix ( WebGL ): normalized planes.
fn frustum_planes(m: &M4) -> [[f64; 4]; 6] {
    let me = m;
    let rows = |a: usize, s: f64| -> [f64; 4] {
        let p = [
            me[3] + s * me[a],
            me[7] + s * me[4 + a],
            me[11] + s * me[8 + a],
            me[15] + s * me[12 + a],
        ];
        let inverse = 1. / (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        [
            p[0] * inverse,
            p[1] * inverse,
            p[2] * inverse,
            p[3] * inverse,
        ]
    };
    [
        rows(0, -1.),
        rows(0, 1.),
        rows(1, 1.),
        rows(1, -1.),
        rows(2, -1.),
        rows(2, 1.),
    ]
}

/// Matrix3.getNormalMatrix( m ): the inverse transpose of the upper 3×3.
fn normal_matrix(m: &M4) -> [f64; 9] {
    let (a, b, c, d, e, f, g, h, i) = (m[0], m[1], m[2], m[4], m[5], m[6], m[8], m[9], m[10]);
    // Matrix3.invert on ( n11 = a, n21 = b, n31 = c, n12 = d, ... ).
    let (n11, n21, n31, n12, n22, n32, n13, n23, n33) = (a, b, c, d, e, f, g, h, i);
    let t11 = n33 * n22 - n32 * n23;
    let t12 = n32 * n13 - n33 * n12;
    let t13 = n23 * n12 - n22 * n13;
    let det = n11 * t11 + n21 * t12 + n31 * t13;
    if det == 0. {
        return [0.; 9];
    }
    let di = 1. / det;
    let inv = [
        t11 * di,
        (n31 * n23 - n33 * n21) * di,
        (n32 * n21 - n31 * n22) * di,
        t12 * di,
        (n33 * n11 - n31 * n13) * di,
        (n31 * n12 - n32 * n11) * di,
        t13 * di,
        (n21 * n13 - n23 * n11) * di,
        (n22 * n11 - n21 * n12) * di,
    ];
    // transpose.
    [
        inv[0], inv[3], inv[6], inv[1], inv[4], inv[7], inv[2], inv[5], inv[8],
    ]
}

/// A fullscreen draw with its own uniform buffer ( each draw in a frame
/// reads its own values ).
struct Stage {
    program: Rc<Program>,
    block: Block,
    buffer: wgpu::Buffer,
    /// Bind groups by the views they bind ( the ping-pong targets alternate ).
    groups: Vec<(Vec<(wgpu::TextureView, SamplerKey)>, wgpu::BindGroup)>,
}

/// The stages of the frame, kept across frames: each draw takes its
/// program's next stage, so a frame reuses the same uniform buffers.
#[derive(Default)]
struct Pool {
    free: std::collections::HashMap<&'static str, Vec<Stage>>,
    used: Vec<Stage>,
    logging: bool,
}

impl Pool {
    fn take(&mut self, r: &Renderer, shared: &Shared, name: &'static str) -> Result<Stage> {
        let mut stage = match self.free.get_mut(name).and_then(|v| v.pop()) {
            Some(stage) => stage,
            None => {
                let program = shared.program(name)?;
                let block = program.block();
                let buffer = program.uniform_buffer(r, &block);
                Stage {
                    program: program.clone(),
                    block,
                    buffer,
                    groups: vec![],
                }
            }
        };
        stage.block.log = self.logging.then(Vec::new);
        Ok(stage)
    }
    /// The frame's stages back, in their order of use.
    fn recycle(&mut self) {
        for stage in std::mem::take(&mut self.used).into_iter().rev() {
            self.free.entry(stage.program.name).or_default().push(stage);
        }
    }
}

impl Stage {
    #[allow(clippy::too_many_arguments)]
    fn run(
        mut self,
        pool: &mut Pool,
        r: &Renderer,
        frame: &Shared,
        encoder: &mut wgpu::CommandEncoder,
        textures: &[(&str, &wgpu::TextureView, SamplerKey)],
        targets: &[Attachment],
        formats: &[wgpu::TextureFormat],
    ) -> Result<()> {
        r.queue.write_buffer(&self.buffer, 0, &self.block.data);
        if let Some(log) = &self.block.log {
            LOG.with(|l| l.borrow_mut().push((self.program.name, log.clone())));
        }
        let group = self.group(r, frame, textures)?;
        let key = PipelineKey {
            formats: formats.to_vec(),
            blend: vec![None; formats.len()],
            depth: None,
            cull_back: false,
        };
        self.program
            .draw_fullscreen(r, encoder, &key, &group, &frame.quad, targets, None);
        pool.used.push(self);
        Ok(())
    }
    /// The bind group for these textures, created the first time they are bound.
    fn group(
        &mut self,
        r: &Renderer,
        frame: &Shared,
        textures: &[(&str, &wgpu::TextureView, SamplerKey)],
    ) -> Result<wgpu::BindGroup> {
        let key: Vec<(wgpu::TextureView, SamplerKey)> =
            textures.iter().map(|t| (t.1.clone(), t.2)).collect();
        if let Some((_, group)) = self.groups.iter().find(|(k, _)| *k == key) {
            return Ok(group.clone());
        }
        let group = self.program.bind(r, &self.buffer, &|name| {
            textures.iter().find(|t| t.0 == name).map(|t| Bound {
                view: t.1,
                sampler: frame.samplers.get(r, t.2),
            })
        })?;
        if self.groups.len() >= 4 {
            self.groups.remove(0);
        }
        self.groups.push((key, group.clone()));
        Ok(group)
    }
}

/// A pass's program and the uniform values it set.
type PassLog = (&'static str, Vec<(String, Vec<f64>)>);

thread_local! {
    static LOG: std::cell::RefCell<Vec<PassLog>> = const { std::cell::RefCell::new(vec![]) };
}
/// window.tilesUniforms: the comparison hook that records each pass's uniforms.
fn logging() -> bool {
    web_sys::window()
        .and_then(|w| js_sys::Reflect::get(&w, &"tilesUniforms".into()).ok())
        .is_some_and(|v| v.is_truthy())
}

fn clear(view: &wgpu::TextureView) -> Attachment<'_> {
    Attachment {
        view,
        slice: None,
        clear: Some(CLEAR),
    }
}

/// A texture and a view of it ( and per-layer views of an array ).
struct Target {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    layers: Vec<wgpu::TextureView>,
}

fn target2d(r: &Renderer, label: &str, (w, h): (u32, u32), format: wgpu::TextureFormat) -> Target {
    let texture = target(r, label, (w, h, 1), format, false);
    let view = texture.create_view(&Default::default());
    Target {
        texture,
        view,
        layers: vec![],
    }
}

fn target_array(r: &Renderer, label: &str, size: u32, layers: u32) -> Target {
    let texture = target(r, label, (size, size, layers), HALF, false);
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });
    let layer_views = (0..layers)
        .map(|k| {
            texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: k,
                array_layer_count: Some(1),
                ..Default::default()
            })
        })
        .collect();
    Target {
        texture,
        view,
        layers: layer_views,
    }
}

fn depth_target(r: &Renderer, label: &str, (w, h): (u32, u32)) -> Target {
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    Target {
        texture,
        view,
        layers: vec![],
    }
}

/// The textures the example loads ( local weather, turbulence, the shape
/// volumes, STBN ) and postprocessing's SMAA lookup images.
pub(crate) struct Assets {
    pub local_weather: crate::texture_gpu::GpuTexture,
    pub turbulence: crate::texture_gpu::GpuTexture,
    pub shape: wgpu::TextureView,
    pub shape_detail: wgpu::TextureView,
    pub stbn: wgpu::TextureView,
    pub area: wgpu::TextureView,
    pub search: wgpu::TextureView,
    _textures: Vec<wgpu::Texture>,
}

/// Rows in WebGL's order for a texture uploaded with flipY.
fn flip_rows(rgba: &[u8], width: u32, height: u32) -> Vec<u8> {
    let row = width as usize * 4;
    let mut out = Vec::with_capacity(rgba.len());
    for y in (0..height as usize).rev() {
        out.extend_from_slice(&rgba[y * row..(y + 1) * row]);
    }
    out
}

impl Assets {
    /// `images`: local weather and turbulence PNGs ( TextureLoader: flipY,
    /// LinearMipMapLinear, repeat, NoColorSpace ), the shape volumes
    /// ( Data3DTexture, red, linear, repeat ), STBN ( red, nearest, repeat ).
    pub fn new(
        r: &Renderer,
        local_weather: &[u8],
        turbulence: &[u8],
        shape: &[u8],
        shape_detail: &[u8],
        stbn: &[u8],
    ) -> Result<Self> {
        let image = |bytes: &[u8]| -> Result<std::sync::Arc<crate::material::Texture>> {
            let mut t = crate::material::Texture::from_image(bytes)?;
            t.rgba = flip_rows(&t.rgba, t.width, t.height);
            t.srgb = false;
            t.flip_y = false;
            t.wrap_s = crate::material::Wrapping::Repeat;
            t.wrap_t = crate::material::Wrapping::Repeat;
            t.filter = crate::material::Filter::Linear;
            t.min_filter = Some(crate::material::Filter::Linear);
            t.mipmap_filter = Some(crate::material::Filter::Linear);
            t.anisotropy = 1;
            Ok(std::sync::Arc::new(t))
        };
        let volume = |label, size: (u32, u32, u32), data: &[u8]| -> Result<wgpu::Texture> {
            if data.len() != (size.0 * size.1 * size.2) as usize {
                return Err(Error::Asset(format!("{label}: {} bytes", data.len())));
            }
            Ok(r.device.create_texture_with_data(
                &r.queue,
                &wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: size.0,
                        height: size.1,
                        depth_or_array_layers: size.2,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D3,
                    format: wgpu::TextureFormat::R8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                data,
            ))
        };
        let shape = volume("cloud shape", (128, 128, 128), shape)?;
        let shape_detail = volume("cloud shape detail", (32, 32, 32), shape_detail)?;
        let stbn = volume("stbn", (128, 128, 64), stbn)?;
        let rgba = |label, bytes: &[u8], flip: bool| -> Result<wgpu::Texture> {
            let t = crate::material::Texture::from_image(bytes)?;
            let data = if flip {
                flip_rows(&t.rgba, t.width, t.height)
            } else {
                t.rgba.clone()
            };
            Ok(r.device.create_texture_with_data(
                &r.queue,
                &wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: t.width,
                        height: t.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: BYTE,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                &data,
            ))
        };
        // SMAAEffect: the search image with flipY, the area image without.
        let area = rgba("smaa area", include_bytes!("smaa_area.png"), false)?;
        let search = rgba("smaa search", include_bytes!("smaa_search.png"), true)?;
        let view = |t: &wgpu::Texture| t.create_view(&Default::default());
        Ok(Assets {
            local_weather: r.upload_texture(&image(local_weather)?)?,
            turbulence: r.upload_texture(&image(turbulence)?)?,
            shape: view(&shape),
            shape_detail: view(&shape_detail),
            stbn: view(&stbn),
            area: view(&area),
            search: view(&search),
            _textures: vec![shape, shape_detail, stbn, area, search],
        })
    }
}

/// What every pass shares: the programs, the triangle, the samplers and
/// three's empty textures for null sampler uniforms.
pub(crate) struct Shared {
    pivot_vertices: [wgpu::Buffer; 3],
    quad: wgpu::Buffer,
    samplers: Samplers,
    empty_3d: wgpu::TextureView,
    programs: std::collections::HashMap<&'static str, Rc<Program>>,
    _textures: Vec<wgpu::Texture>,
}

macro_rules! programs {
    ($r:expr, $( $name:literal $vertices:ident [ $( $unfilterable:literal ),* ] ),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($name, Rc::new(Program::new(
                $r,
                $name,
                include_str!(concat!("wgsl/", $name, ".vert.wgsl")),
                include_str!(concat!("wgsl/", $name, ".frag.wgsl")),
                Vertices::$vertices,
                &[$( $unfilterable ),*],
            )?));
        )*
        map
    }};
}

impl Shared {
    pub fn new(r: &Renderer) -> Result<Self> {
        let programs = programs!(r,
            "tile" Mesh [], "tile_fade" Mesh [], "normal" Mesh [],
            "shadow" Fullscreen [], "shadow_resolve" Fullscreen [],
            "clouds" Fullscreen ["depthBuffer"], "clouds_resolve" Fullscreen [],
            "aerial_perspective" Fullscreen ["depthBuffer"],
            "downsample_threshold" Fullscreen [], "downsampling" Fullscreen [], "upsampling" Fullscreen [],
            "kawase_blur" Fullscreen [], "copy" Fullscreen [], "lens_flare_features" Fullscreen [],
            "lens_flare" Fullscreen ["depthBuffer"],
            "smaa_edges" Fullscreen [], "smaa_weights" Fullscreen ["searchTexture"], "smaa_blend" Fullscreen ["depthBuffer"],
            "dithering" Fullscreen ["depthBuffer"], "output" Fullscreen [],
            "pivot" Mesh [],
        );
        // PivotPointMesh's PlaneGeometry( 0, 0 ): four vertices at the origin, two triangles.
        let uvs: [[f32; 2]; 4] = [[0., 1.], [1., 1.], [0., 0.], [1., 0.]];
        let corners: Vec<f32> = [0usize, 2, 1, 2, 3, 1]
            .iter()
            .flat_map(|&k| uvs[k])
            .collect();
        let vertex = |label, data: &[f32]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        };
        let pivot_vertices = [
            vertex("pivot position", &[0.; 18]),
            vertex("pivot normal", &[0., 0., 1.].repeat(6)),
            vertex("pivot uv", &corners),
        ];
        let empty = target(r, "empty 3D texture", (1, 1, 1), BYTE, true);
        Ok(Shared {
            pivot_vertices,
            quad: fullscreen_buffer(r),
            samplers: Samplers::default(),
            empty_3d: empty.create_view(&Default::default()),
            programs,
            _textures: vec![empty],
        })
    }
    fn program(&self, name: &str) -> Result<&Rc<Program>> {
        self.programs
            .get(name)
            .ok_or(Error::Invalid("tiles3d program"))
    }
}

/// The size-dependent render targets.
struct Targets {
    size: (u32, u32),
    scene: Target,
    depth: Target,
    normal: Target,
    normal_depth: Target,
    shadow: Target,
    shadow_history: [Target; 2],
    clouds_color: Target,
    clouds_depth_velocity: Target,
    clouds_shadow_length: Target,
    clouds_history: [(Target, Target); 2],
    effect: [Target; 2],
    flare: [Target; 2],
    down: Vec<Target>,
    up: Vec<Target>,
    kawase: [Target; 2],
    edges: Target,
    weights: Target,
}

/// Resolution: Math.max( 1, Math.round( base × scale ) ).
fn scaled(v: u32, scale: f64) -> u32 {
    ((v as f64 * scale + 0.5).floor() as u32).max(1)
}

impl Targets {
    fn new(r: &Renderer, (w, h): (u32, u32)) -> Self {
        let low = (w.div_ceil(4), h.div_ceil(4));
        let flare = (scaled(w, 0.5), scaled(h, 0.5));
        let mut mips = vec![];
        let (mut mw, mut mh) = flare;
        for _ in 0..8 {
            mw = scaled(mw, 0.5);
            mh = scaled(mh, 0.5);
            mips.push((mw, mh));
        }
        let kawase = (scaled(flare.0, 0.5), scaled(flare.1, 0.5));
        let s = clouds::SHADOW_MAP_SIZE;
        Targets {
            size: (w, h),
            scene: target2d(r, "output buffer", (w, h), HALF),
            depth: depth_target(r, "output depth", (w, h)),
            normal: target2d(r, "NormalPass.Target", (w, h), BYTE),
            normal_depth: depth_target(r, "NormalPass depth", (w, h)),
            shadow: target_array(r, "Shadow", s, 2 * clouds::CASCADE_COUNT as u32),
            shadow_history: [
                target_array(r, "Shadow.A", s, clouds::CASCADE_COUNT as u32),
                target_array(r, "Shadow.B", s, clouds::CASCADE_COUNT as u32),
            ],
            clouds_color: target2d(r, "Clouds", low, HALF),
            clouds_depth_velocity: target2d(r, "Clouds depth velocity", low, HALF),
            clouds_shadow_length: target2d(r, "Clouds shadow length", low, RED_HALF),
            clouds_history: [
                (
                    target2d(r, "Clouds.A", (w, h), HALF),
                    target2d(r, "Clouds.A shadow length", (w, h), RED_HALF),
                ),
                (
                    target2d(r, "Clouds.B", (w, h), HALF),
                    target2d(r, "Clouds.B shadow length", (w, h), RED_HALF),
                ),
            ],
            effect: [
                target2d(r, "effects A", (w, h), HALF),
                target2d(r, "effects B", (w, h), HALF),
            ],
            flare: [
                target2d(r, "LensFlare.Target1", flare, HALF),
                target2d(r, "LensFlare.Target2", flare, HALF),
            ],
            down: mips
                .iter()
                .map(|&m| target2d(r, "Downsampling.Mipmap", m, HALF))
                .collect(),
            up: mips[..7]
                .iter()
                .map(|&m| target2d(r, "Upsampling.Mipmap", m, HALF))
                .collect(),
            kawase: [
                target2d(r, "KawaseBlurPass.A", kawase, HALF),
                target2d(r, "KawaseBlurPass.B", kawase, HALF),
            ],
            edges: target2d(r, "SMAA.Edges", (w, h), BYTE),
            weights: target2d(r, "SMAA.Weights", (w, h), BYTE),
        }
    }
}

/// The page's postprocessing state across frames.
pub(crate) struct Frame {
    targets: Option<Targets>,
    /// CloudsEffect.frame and AerialPerspectiveEffect's frame uniform.
    frame: u64,
    time: f64,
    local_weather_offset: [f64; 2],
    previous: Option<(M4, M4)>,
    pool: Pool,
    shadow_previous: Option<Vec<M4>>,
    /// Which history targets hold the latest resolve.
    shadow_flip: usize,
    clouds_flip: usize,
    pub sun_direction: V3,
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            targets: None,
            frame: 0,
            time: 0.,
            local_weather_offset: [0., 0.],
            previous: None,
            pool: Pool::default(),
            shadow_previous: None,
            shadow_flip: 0,
            clouds_flip: 0,
            sun_direction: [0., 0., 1.],
        }
    }
}

fn f(v: &M4) -> Vec<f64> {
    v.to_vec()
}

impl Frame {
    /// renderer.render( scene, camera ) through setEffects.
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        r: &Renderer,
        shared: &Shared,
        atmosphere: &super::atmosphere::Precomputed,
        assets: &Assets,
        camera: &View,
        meshes: &mut [(&mut TileMesh, Option<(f64, f64)>)],
        pivot: Option<V3>,
        out: &wgpu::TextureView,
        out_format: wgpu::TextureFormat,
        size: (u32, u32),
        delta: f64,
    ) -> Result<()> {
        if self.targets.as_ref().is_none_or(|t| t.size != size) {
            self.targets = Some(Targets::new(r, size));
            // setSize invalidates the clouds reprojection.
            self.previous = None;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("tiles3d targets"))?;
        let (w, h) = size;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let mut pool = std::mem::take(&mut self.pool);
        pool.logging = logging();
        let pivot_stage = match pivot {
            Some(p) => {
                let mut stage = pool.take(r, shared, "pivot")?;
                let model = [
                    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., p[0], p[1], p[2], 1.,
                ];
                let b = &mut stage.block;
                b.set("modelMatrix", &f(&model));
                b.set("modelViewMatrix", &f(&multiply(&camera.view, &model)));
                b.set("projectionMatrix", &f(&camera.projection));
                b.set("viewMatrix", &f(&camera.view));
                b.set("cameraPosition", &camera.position());
                // renderer.getSize(): CSS pixels.
                let ratio = web_sys::window().map_or(1., |w| w.device_pixel_ratio());
                b.set(
                    "resolution",
                    &[size.0 as f64 / ratio, size.1 as f64 / ratio],
                );
                b.set1("size", 15.);
                b.set1("thickness", 2.);
                b.set1("opacity", 1.);
                Some(stage)
            }
            None => None,
        };
        let mut pivot_stage = pivot_stage;
        self.scene_passes(
            r,
            shared,
            camera,
            meshes,
            pivot_stage.as_mut(),
            t,
            &mut encoder,
        )?;
        if let Some(stage) = pivot_stage {
            pool.used.push(stage);
        }
        pool.logging = logging();
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("tiles3d targets"))?;
        self.time += delta;
        self.frame += 1;
        let frame = self.frame as f64;
        // CloudsEffect.updateSharedUniforms.
        // clouds.localWeatherVelocity ( 0.001, 0 ); window.tilesStaticWeather
        // stops it for the comparison, as the reference's fixtureStaticWeather does.
        let static_weather = web_sys::window()
            .and_then(|w| js_sys::Reflect::get(&w, &"tilesStaticWeather".into()).ok())
            .is_some_and(|v| v.is_truthy());
        if !static_weather {
            self.local_weather_offset[0] += 0.001 * delta;
        }
        let position = camera.position();
        let altitude_correction = clouds::altitude_correction(position);
        let zenith = super::volume::dot(self.sun_direction, clouds::surface_normal(position));
        let distance = 1e6 + (1e3 - 1e6) * zenith;
        let (shadow_far, cascades) = clouds::cascades(
            &camera.world,
            &camera.projection,
            camera.near,
            camera.far,
            self.sun_direction,
            distance,
        );
        let atmosphere_uniforms = |b: &mut Block| {
            b.set1("bottomRadius", clouds::BOTTOM_RADIUS);
            b.set1("topRadius", 6420000.);
            b.set("worldToECEFMatrix", &f(&super::volume::IDENTITY));
            b.set("ecefToWorldMatrix", &f(&super::volume::IDENTITY));
            b.set("altitudeCorrection", &altitude_correction);
            b.set("sunDirection", &self.sun_direction);
        };
        let s = clouds::SHADOW_MAP_SIZE as f64;
        // ShadowPass.update.
        let mut shadow = pool.take(r, shared, "shadow")?;
        {
            let b = &mut shadow.block;
            clouds::set_parameters(b, self.local_weather_offset);
            clouds::set_layers(b, true);
            atmosphere_uniforms(b);
            let inverse: Vec<f64> = cascades
                .iter()
                .flat_map(|c| c.inverse_matrix.to_vec())
                .collect();
            b.set("inverseShadowMatrices", &inverse);
            let previous = self
                .shadow_previous
                .clone()
                .unwrap_or_else(|| vec![super::volume::IDENTITY; clouds::CASCADE_COUNT]);
            let previous: Vec<f64> = previous.iter().flat_map(|m| m.to_vec()).collect();
            b.set("reprojectionMatrices", &previous);
            b.set("resolution", &[s, s]);
            b.set1("frame", frame);
            b.set1("maxIterationCount", 50.);
            b.set1("minStepSize", 100.);
            b.set1("maxStepSize", 1000.);
            b.set1("minDensity", 1e-5);
            b.set1("minExtinction", 1e-5);
            b.set1("minTransmittance", 1e-4);
            b.set1("opticalDepthTailScale", 2.);
        }
        let cloud_textures = [
            ("stbnTexture", &assets.stbn, NEAREST_REPEAT),
            (
                "localWeatherTexture",
                &assets.local_weather.view,
                super::program::MIPMAP_REPEAT,
            ),
            ("shapeTexture", &assets.shape, LINEAR_REPEAT),
            ("shapeDetailTexture", &assets.shape_detail, LINEAR_REPEAT),
            (
                "turbulenceTexture",
                &assets.turbulence.view,
                super::program::MIPMAP_REPEAT,
            ),
        ];
        let shadow_targets: Vec<Attachment> = t.shadow.layers.iter().map(clear).collect();
        shadow.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &cloud_textures,
            &shadow_targets,
            &[HALF; 4],
        )?;
        let mut resolve = pool.take(r, shared, "shadow_resolve")?;
        resolve.block.set("texelSize", &[1. / s, 1. / s]);
        resolve.block.set1("varianceGamma", 1.);
        resolve.block.set1("temporalAlpha", 0.01);
        let (history, next) = (
            &t.shadow_history[self.shadow_flip],
            &t.shadow_history[1 - self.shadow_flip],
        );
        let next_targets: Vec<Attachment> = next.layers.iter().map(clear).collect();
        resolve.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[
                ("inputBuffer", &t.shadow.view, LINEAR),
                ("historyBuffer", &history.view, LINEAR),
            ],
            &next_targets,
            &[HALF; 2],
        )?;
        self.shadow_previous = Some(cascades.iter().map(|c| c.matrix).collect());
        self.shadow_flip = 1 - self.shadow_flip;
        let shadow_buffer = &t.shadow_history[self.shadow_flip].view;
        // CloudsPass.update.
        let low = (w.div_ceil(4), h.div_ceil(4));
        let (ox, oy) = clouds::bayer_offset(self.frame);
        let resolution = ((low.0 * 4) as f64, (low.1 * 4) as f64);
        let (dx, dy) = (
            ((ox - 0.5) / resolution.0) * 4.,
            ((oy - 0.5) / resolution.1) * 4.,
        );
        let mut jittered = camera.projection;
        jittered[8] += dx * 2.;
        jittered[9] += dy * 2.;
        let (previous_projection, previous_view) =
            self.previous.unwrap_or((camera.projection, camera.view));
        let mut reprojection = previous_projection;
        reprojection[8] += dx * 2.;
        reprojection[9] += dy * 2.;
        let reprojection = multiply(&reprojection, &previous_view);
        let view_reprojection = multiply(&reprojection, &camera.world);
        let mut clouds_stage = pool.take(r, shared, "clouds")?;
        let (sun_luminance, sky_luminance) = super::atmosphere::radiance_to_relative_luminance();
        {
            let b = &mut clouds_stage.block;
            super::atmosphere::set_atmosphere(b, "ATMOSPHERE");
            b.set("SUN_SPECTRAL_RADIANCE_TO_LUMINANCE", &sun_luminance);
            b.set("SKY_SPECTRAL_RADIANCE_TO_LUMINANCE", &sky_luminance);
            clouds::set_parameters(b, self.local_weather_offset);
            clouds::set_layers(b, false);
            atmosphere_uniforms(b);
            b.set("viewMatrix", &f(&camera.view));
            b.set("inverseViewMatrix", &f(&camera.world));
            b.set("inverseProjectionMatrix", &f(&invert(&jittered)));
            b.set("reprojectionMatrix", &f(&reprojection));
            b.set("viewReprojectionMatrix", &f(&view_reprojection));
            b.set("resolution", &[resolution.0, resolution.1]);
            b.set(
                "targetUvScale",
                &[resolution.0 / w as f64, resolution.1 / h as f64],
            );
            b.set1("cameraNear", camera.near);
            b.set1("cameraFar", camera.far);
            b.set("cameraPosition", &position);
            if let Some(height) = clouds::geodetic_height(position) {
                b.set1("cameraHeight", height);
            }
            b.set1("frame", frame);
            b.set("temporalJitter", &[dx, dy]);
            b.set1("mipLevelScale", 0.25);
            b.set1("skyLightScale", 1.);
            b.set1("groundBounceScale", 1.);
            b.set1("powderScale", 0.8);
            b.set1("powderExponent", 150.);
            for (name, v) in [
                ("maxIterationCount", 500.),
                ("minStepSize", 50.),
                ("maxStepSize", 1000.),
                ("maxRayDistance", 2e5),
                ("perspectiveStepScale", 1.01),
                ("minDensity", 1e-5),
                ("minExtinction", 1e-5),
                ("minTransmittance", 1e-2),
                ("maxIterationCountToSun", 2.),
                ("maxIterationCountToGround", 3.),
                ("minSecondaryStepSize", 100.),
                ("secondaryStepScale", 2.),
                ("maxShadowFilterRadius", 6.),
                ("maxShadowLengthIterationCount", 500.),
                ("minShadowLengthStepSize", 50.),
                ("maxShadowLengthRayDistance", 2e5),
                ("hazeDensityScale", 3e-5),
                ("hazeExponent", 1e-3),
                ("hazeScatteringCoefficient", 0.9),
                ("hazeAbsorptionCoefficient", 0.5),
            ] {
                b.set1(name, v);
            }
            b.set("shadowTexelSize", &[1. / s, 1. / s]);
            let intervals: Vec<f64> = cascades.iter().flat_map(|c| c.interval).collect();
            b.set("shadowIntervals", &intervals);
            let matrices: Vec<f64> = cascades.iter().flat_map(|c| c.matrix.to_vec()).collect();
            b.set("shadowMatrices", &matrices);
            b.set1("shadowFar", shadow_far);
        }
        let atmosphere_textures = [
            ("transmittance_texture", &atmosphere.transmittance, LINEAR),
            ("scattering_texture", &atmosphere.scattering, LINEAR),
            ("irradiance_texture", &atmosphere.irradiance, LINEAR),
            ("single_mie_scattering_texture", &shared.empty_3d, NEAREST),
            (
                "higher_order_scattering_texture",
                &atmosphere.higher_order_scattering,
                LINEAR,
            ),
        ];
        let mut textures: Vec<(&str, &wgpu::TextureView, SamplerKey)> =
            atmosphere_textures.to_vec();
        textures.extend_from_slice(&cloud_textures);
        textures.push(("depthBuffer", &t.depth.view, NEAREST));
        textures.push(("shadowBuffer", shadow_buffer, LINEAR));
        clouds_stage.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &textures,
            &[
                clear(&t.clouds_color.view),
                clear(&t.clouds_depth_velocity.view),
                clear(&t.clouds_shadow_length.view),
            ],
            &[HALF, HALF, RED_HALF],
        )?;
        let mut clouds_resolve = pool.take(r, shared, "clouds_resolve")?;
        {
            let b = &mut clouds_resolve.block;
            b.set("texelSize", &[1. / w as f64, 1. / h as f64]);
            b.set1("frame", frame);
            b.set("jitterOffset", &[(ox - 0.5) * 4., (oy - 0.5) * 4.]);
            b.set1("varianceGamma", 2.);
            b.set1("temporalAlpha", 0.1);
        }
        let (history, next) = (
            &t.clouds_history[self.clouds_flip],
            &t.clouds_history[1 - self.clouds_flip],
        );
        clouds_resolve.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[
                ("colorBuffer", &t.clouds_color.view, LINEAR),
                ("depthVelocityBuffer", &t.clouds_depth_velocity.view, LINEAR),
                ("colorHistoryBuffer", &history.0.view, LINEAR),
                ("shadowLengthBuffer", &t.clouds_shadow_length.view, LINEAR),
                ("shadowLengthHistoryBuffer", &history.1.view, LINEAR),
            ],
            &[clear(&next.0.view), clear(&next.1.view)],
            &[HALF, RED_HALF],
        )?;
        self.previous = Some((camera.projection, camera.view));
        self.clouds_flip = 1 - self.clouds_flip;
        let clouds_output = &t.clouds_history[self.clouds_flip];
        // EffectPass( camera, clouds, aerialPerspective ).
        let mut aerial = pool.take(r, shared, "aerial_perspective")?;
        {
            let b = &mut aerial.block;
            effect_uniforms(b, camera, size, self.time);
            b.set1("e0BlendOpacity", 1.);
            b.set1("e1BlendOpacity", 1.);
            super::atmosphere::set_atmosphere(b, "e1ATMOSPHERE");
            b.set("e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE", &sun_luminance);
            b.set("e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE", &sky_luminance);
            b.set("e1ProjectionMatrix", &f(&camera.projection));
            b.set("e1ViewMatrix", &f(&camera.view));
            b.set("e1InverseProjectionMatrix", &f(&invert(&camera.projection)));
            b.set("e1InverseViewMatrix", &f(&camera.world));
            b.set("e1CameraPosition", &position);
            b.set1("e1BottomRadius", clouds::BOTTOM_RADIUS);
            b.set("e1EllipsoidRadii", &clouds::RADII);
            b.set("e1WorldToECEFMatrix", &f(&super::volume::IDENTITY));
            b.set("e1AltitudeCorrection", &altitude_correction);
            if let Some(height) = clouds::geodetic_height(position) {
                // The projected globe scale, remapped from ( 41.5, 13.8 ).
                let p = apply4(
                    [0., clouds::RADII[0], -(height.max(0.))],
                    &camera.projection,
                );
                let amount = ((p[1] - 41.5) / (13.8 - 41.5)).clamp(0., 1.);
                b.set1("e1GeometricErrorCorrectionAmount", amount);
            }
            b.set("e1SunDirection", &self.sun_direction);
            // cosSunAngularRadius starts as the angular radius itself.
            b.set1("e1CosSunAngularRadius", 0.004675);
            b.set1("e1AlbedoScale", 1.);
            b.set("e1MoonDirection", &[0., 0., 0.]);
            b.set1("e1MoonAngularRadius", 0.0045);
            b.set1("e1LunarRadianceScale", 1.);
            b.set("e1ShadowMapSize", &[s, s]);
            let intervals: Vec<f64> = cascades.iter().flat_map(|c| c.interval).collect();
            b.set("e1ShadowIntervals", &intervals);
            let matrices: Vec<f64> = cascades.iter().flat_map(|c| c.matrix.to_vec()).collect();
            b.set("e1ShadowMatrices", &matrices);
            let inverse: Vec<f64> = cascades
                .iter()
                .flat_map(|c| c.inverse_matrix.to_vec())
                .collect();
            b.set("e1InverseShadowMatrices", &inverse);
            b.set1("e1ShadowFar", shadow_far);
            b.set1("e1ShadowTopHeight", 2200.);
            b.set1("e1ShadowRadius", 3.);
            b.set1("e1Frame", frame);
        }
        aerial.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[
                ("inputBuffer", &t.scene.view, LINEAR),
                ("depthBuffer", &t.depth.view, NEAREST),
                ("e0CloudsBuffer", &clouds_output.0.view, LINEAR),
                ("e1Transmittance_texture", &atmosphere.transmittance, LINEAR),
                ("e1Scattering_texture", &atmosphere.scattering, LINEAR),
                ("e1Irradiance_texture", &atmosphere.irradiance, LINEAR),
                ("e1Single_mie_scattering_texture", &shared.empty_3d, NEAREST),
                (
                    "e1Higher_order_scattering_texture",
                    &atmosphere.higher_order_scattering,
                    LINEAR,
                ),
                ("e1NormalBuffer", &t.normal.view, NEAREST),
                ("e1OverlayBuffer", &clouds_output.0.view, LINEAR),
                ("e1ShadowBuffer", shadow_buffer, LINEAR),
                ("e1StbnTexture", &assets.stbn, NEAREST_REPEAT),
                ("e1ShadowLengthBuffer", &clouds_output.1.view, LINEAR),
            ],
            &[clear(&t.effect[0].view)],
            &[HALF],
        )?;
        // EffectPass( camera, new LensFlareEffect() ): its update, then the composite.
        let flare_size = (
            t.flare[0].texture.width() as f64,
            t.flare[0].texture.height() as f64,
        );
        let mut threshold = pool.take(r, shared, "downsample_threshold")?;
        threshold
            .block
            .set("texelSize", &[1. / flare_size.0, 1. / flare_size.1]);
        threshold.block.set1("thresholdLevel", 10.);
        threshold.block.set1("thresholdRange", 1.);
        threshold.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[("inputBuffer", &t.effect[0].view, LINEAR)],
            &[clear(&t.flare[0].view)],
            &[HALF],
        )?;
        // MipmapBlurPass ( 8 levels, radius 0.85 ).
        let mut previous = (&t.flare[0], flare_size);
        for mip in &t.down {
            let mut stage = pool.take(r, shared, "downsampling")?;
            stage
                .block
                .set("texelSize", &[1. / previous.1.0, 1. / previous.1.1]);
            stage.run(
                &mut pool,
                r,
                shared,
                &mut encoder,
                &[("inputBuffer", &previous.0.view, LINEAR)],
                &[clear(&mip.view)],
                &[HALF],
            )?;
            previous = (
                mip,
                (mip.texture.width() as f64, mip.texture.height() as f64),
            );
        }
        for i in (0..t.up.len()).rev() {
            let mip = &t.up[i];
            let mut stage = pool.take(r, shared, "upsampling")?;
            stage
                .block
                .set("texelSize", &[1. / previous.1.0, 1. / previous.1.1]);
            stage.block.set1("radius", 0.85);
            stage.run(
                &mut pool,
                r,
                shared,
                &mut encoder,
                &[
                    ("inputBuffer", &previous.0.view, LINEAR),
                    ("supportBuffer", &t.down[i].view, LINEAR),
                ],
                &[clear(&mip.view)],
                &[HALF],
            )?;
            previous = (
                mip,
                (mip.texture.width() as f64, mip.texture.height() as f64),
            );
        }
        // KawaseBlurPass ( KernelSize.SMALL: 0, 1, 1 ) into renderTarget2.
        let kawase_texel = [
            1. / flare_size.0,
            1. / flare_size.1,
            0.5 / flare_size.0,
            0.5 / flare_size.1,
        ];
        let sequence = [
            (0., &t.flare[0], &t.kawase[0]),
            (1., &t.kawase[0], &t.kawase[1]),
            (1., &t.kawase[1], &t.kawase[0]),
        ];
        for (kernel, input, output) in sequence {
            let mut stage = pool.take(r, shared, "kawase_blur")?;
            stage.block.set("texelSize", &kawase_texel);
            stage.block.set1("kernel", kernel);
            stage.block.set1("scale", 1.);
            stage.run(
                &mut pool,
                r,
                shared,
                &mut encoder,
                &[("inputBuffer", &input.view, LINEAR)],
                &[clear(&output.view)],
                &[HALF],
            )?;
        }
        let mut copy = pool.take(r, shared, "copy")?;
        copy.block.set1("opacity", 1.);
        copy.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[("inputBuffer", &t.kawase[0].view, LINEAR)],
            &[clear(&t.flare[1].view)],
            &[HALF],
        )?;
        let mut features = pool.take(r, shared, "lens_flare_features")?;
        features
            .block
            .set("texelSize", &[1. / flare_size.0, 1. / flare_size.1]);
        features.block.set1("ghostAmount", 0.001);
        features.block.set1("haloAmount", 0.001);
        features.block.set1("chromaticAberration", 10.);
        features.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[("inputBuffer", &t.flare[1].view, LINEAR)],
            &[clear(&t.flare[0].view)],
            &[HALF],
        )?;
        let mut flare = pool.take(r, shared, "lens_flare")?;
        effect_uniforms(&mut flare.block, camera, size, self.time);
        flare.block.set1("e0BlendOpacity", 1.);
        flare.block.set1("e0Intensity", 0.005);
        flare.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[
                ("inputBuffer", &t.effect[0].view, LINEAR),
                ("depthBuffer", &t.depth.view, NEAREST),
                ("e0BloomBuffer", &t.up[0].view, LINEAR),
                ("e0FeaturesBuffer", &t.flare[0].view, LINEAR),
            ],
            &[clear(&t.effect[1].view)],
            &[HALF],
        )?;
        // EffectPass( camera, new SMAAEffect() ).
        let texel = [1. / w as f64, 1. / h as f64];
        let mut edges = pool.take(r, shared, "smaa_edges")?;
        edges.block.set("texelSize", &texel);
        edges.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[("inputBuffer", &t.effect[1].view, LINEAR)],
            &[clear(&t.edges.view)],
            &[BYTE],
        )?;
        let mut weights = pool.take(r, shared, "smaa_weights")?;
        weights.block.set("texelSize", &texel);
        weights.block.set("resolution", &[w as f64, h as f64]);
        weights.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[
                ("inputBuffer", &t.edges.view, LINEAR),
                ("areaTexture", &assets.area, LINEAR),
                ("searchTexture", &assets.search, NEAREST),
            ],
            &[clear(&t.weights.view)],
            &[BYTE],
        )?;
        let mut blend = pool.take(r, shared, "smaa_blend")?;
        effect_uniforms(&mut blend.block, camera, size, self.time);
        blend.block.set1("e0BlendOpacity", 1.);
        blend.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[
                ("inputBuffer", &t.effect[1].view, LINEAR),
                ("depthBuffer", &t.depth.view, NEAREST),
                ("e0WeightMap", &t.weights.view, LINEAR),
            ],
            &[clear(&t.effect[0].view)],
            &[HALF],
        )?;
        // EffectPass( camera, new DitheringEffect() ).
        let mut dithering = pool.take(r, shared, "dithering")?;
        effect_uniforms(&mut dithering.block, camera, size, self.time);
        dithering.block.set1("e0BlendOpacity", 1.);
        dithering.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[
                ("inputBuffer", &t.effect[0].view, LINEAR),
                ("depthBuffer", &t.depth.view, NEAREST),
            ],
            &[clear(&t.effect[1].view)],
            &[HALF],
        )?;
        // A comparison hook: window.tilesDebug names a texture the output pass shows.
        let debug = web_sys::window()
            .and_then(|w| js_sys::Reflect::get(&w, &"tilesDebug".into()).ok())
            .and_then(|v| v.as_string());
        let shown = match debug.as_deref() {
            Some("scene") => &t.scene.view,
            Some("normal") => &t.normal.view,
            Some("clouds") => &clouds_output.0.view,
            Some("clouds_low") => &t.clouds_color.view,
            Some("flare") => &t.flare[0].view,
            Some("bloom") => &t.up[0].view,
            Some("edges") => &t.edges.view,
            Some("weights") => &t.weights.view,
            _ => &t.effect[1].view,
        };
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body.set_attribute("data-tiles-frame", &self.frame.to_string());
            let passes = LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));
            if !passes.is_empty() {
                let json: Vec<String> = passes
                    .iter()
                    .map(|(name, log)| {
                        let values: Vec<String> = log
                            .iter()
                            .map(|(n, v)| format!("{:?}:{:?}", n, v))
                            .collect();
                        format!("[{:?},{{{}}}]", name, values.join(","))
                    })
                    .collect();
                let _ = body.set_attribute("data-tiles-uniforms", &format!("[{}]", json.join(",")));
            }
        }
        // WebGLOutput: AgX at exposure 10, sRGB, into the canvas.
        let mut output = pool.take(r, shared, "output")?;
        output
            .block
            .set("modelViewMatrix", &f(&super::volume::IDENTITY));
        output.block.set(
            "projectionMatrix",
            &[
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -2., 0., 0., 0., -1., 1.,
            ],
        );
        output.block.set1("toneMappingExposure", 10.);
        output.run(
            &mut pool,
            r,
            shared,
            &mut encoder,
            &[("tDiffuse", shown, LINEAR)],
            &[clear(out)],
            &[out_format],
        )?;
        r.queue.submit([encoder.finish()]);
        pool.recycle();
        self.pool = pool;
        Ok(())
    }

    /// The tiles into the output buffer, then NormalPass's MeshNormalMaterial
    /// override.
    #[allow(clippy::too_many_arguments)]
    fn scene_passes(
        &self,
        r: &Renderer,
        shared: &Shared,
        camera: &View,
        meshes: &mut [(&mut TileMesh, Option<(f64, f64)>)],
        mut pivot: Option<&mut Stage>,
        t: &Targets,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<()> {
        let screen = multiply(&camera.projection, &camera.view);
        let planes = frustum_planes(&screen);
        // projectObject: frustum culling, then the opaque list's z order.
        let mut list: Vec<(f64, usize)> = meshes
            .iter()
            .enumerate()
            .filter(|(_, (m, _))| m.in_frustum(&planes))
            .map(|(i, (m, _))| {
                (
                    apply4([m.world[12], m.world[13], m.world[14]], &screen)[2],
                    i,
                )
            })
            .collect();
        list.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let tile = shared.program("tile")?;
        let tile_fade = shared.program("tile_fade")?;
        let normal = shared.program("normal")?;
        for &(_, i) in &list {
            let (mesh, fade) = &mut meshes[i];
            if mesh.bindings.is_none() {
                let make = |p: &Rc<Program>,
                            map: Option<&crate::texture_gpu::GpuTexture>|
                 -> Result<(Block, wgpu::Buffer, wgpu::BindGroup)> {
                    let block = p.block();
                    let buffer = p.uniform_buffer(r, &block);
                    let group = p.bind(r, &buffer, &|name| {
                        (name == "map")
                            .then(|| {
                                map.map(|m| Bound {
                                    view: &m.view,
                                    sampler: m.sampler.clone(),
                                })
                            })
                            .flatten()
                    })?;
                    Ok((block, buffer, group))
                };
                mesh.bindings = Some(MeshBindings {
                    color: make(tile, Some(&mesh.map))?,
                    fade: make(tile_fade, Some(&mesh.map))?,
                    normal: make(normal, None)?,
                });
            }
            let model_view = multiply(&camera.view, &mesh.world);
            let normal_matrix = normal_matrix(&model_view);
            let b = mesh
                .bindings
                .as_mut()
                .ok_or(Error::Invalid("tile bindings"))?;
            for (block, buffer) in [
                (&mut b.color.0, &b.color.1),
                (&mut b.fade.0, &b.fade.1),
                (&mut b.normal.0, &b.normal.1),
            ] {
                block.set("modelMatrix", &f(&mesh.world));
                block.set("modelViewMatrix", &f(&model_view));
                block.set("projectionMatrix", &f(&camera.projection));
                block.set("viewMatrix", &f(&camera.view));
                block.set("normalMatrix", &normal_matrix);
                block.set("cameraPosition", &camera.position());
                block.set("diffuse", &[1., 1., 1.]);
                block.set1("opacity", 1.);
                block.set("mapTransform", &[1., 0., 0., 0., 1., 0., 0., 0., 1.]);
                if let Some((fade_in, fade_out)) = fade {
                    block.set1("fadeIn", *fade_in);
                    block.set1("fadeOut", *fade_out);
                }
                r.queue.write_buffer(buffer, 0, &block.data);
            }
        }
        let depth = Some((DEPTH, wgpu::CompareFunction::LessEqual, true));
        for (pass, view, depth_view, clear_color) in [
            (0, &t.scene.view, &t.depth.view, CLEAR),
            // ClearPass: 0x7777ff in linear sRGB.
            (
                1,
                &t.normal.view,
                &t.normal_depth.view,
                wgpu::Color {
                    r: 0.1844749944900301,
                    g: 0.1844749944900301,
                    b: 1.,
                    a: 1.,
                },
            ),
        ] {
            let format = if pass == 0 { HALF } else { BYTE };
            let key = PipelineKey {
                formats: vec![format],
                blend: vec![None],
                depth,
                cull_back: true,
            };
            let pipelines = [
                tile.pipeline(r, &key),
                tile_fade.pipeline(r, &key),
                normal.pipeline(r, &key),
            ];
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(if pass == 0 { "tiles" } else { "NormalPass" }),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            for &(_, i) in &list {
                let (mesh, fade) = &meshes[i];
                let Some(b) = &mesh.bindings else { continue };
                let (pipeline, group) = match (pass, fade) {
                    (1, _) => (&pipelines[2], &b.normal.2),
                    (_, Some(_)) => (&pipelines[1], &b.fade.2),
                    _ => (&pipelines[0], &b.color.2),
                };
                rp.set_pipeline(pipeline);
                rp.set_bind_group(0, group, &[]);
                rp.set_vertex_buffer(0, mesh.position.slice(..));
                rp.set_vertex_buffer(1, mesh.normal.slice(..));
                rp.set_vertex_buffer(2, mesh.uv.slice(..));
                rp.draw(0..mesh.count, 0..1);
            }
            // The pivot mesh ( renderOrder Infinity, transparent, no depth test ),
            // which the normal pass's override draws as a zero-size plane.
            if let (0, Some(stage)) = (pass, pivot.as_deref_mut()) {
                r.queue.write_buffer(&stage.buffer, 0, &stage.block.data);
                let group = stage.group(r, shared, &[])?;
                let key = PipelineKey {
                    formats: vec![HALF],
                    blend: vec![Some(NORMAL_BLENDING)],
                    depth: Some((DEPTH, wgpu::CompareFunction::Always, false)),
                    cull_back: true,
                };
                rp.set_pipeline(&stage.program.pipeline(r, &key));
                rp.set_bind_group(0, &group, &[]);
                for (k, b) in shared.pivot_vertices.iter().enumerate() {
                    rp.set_vertex_buffer(k as u32, b.slice(..));
                }
                rp.draw(0..6, 0..1);
            }
        }
        Ok(())
    }
}

/// EffectMaterial's shared uniforms.
fn effect_uniforms(b: &mut Block, camera: &View, (w, h): (u32, u32), time: f64) {
    b.set("resolution", &[w as f64, h as f64]);
    b.set("texelSize", &[1. / w as f64, 1. / h as f64]);
    b.set1("cameraNear", camera.near);
    b.set1("cameraFar", camera.far);
    b.set1("aspect", camera.aspect);
    b.set1("time", time);
    b.set("viewMatrix", &f(&camera.view));
    b.set("cameraPosition", &camera.position());
}
