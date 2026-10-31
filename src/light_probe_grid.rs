//! LightProbeGrid ( `examples/jsm/lighting/LightProbeGrid.js`, r187 ): a
//! box of light probes, one per cell at its center, baked on the GPU into a
//! padded RGBA16F 3D atlas of L2 spherical harmonics, which lit materials
//! sample for diffuse irradiance as LightProbeGridNode does. A grid is a
//! scene node of kind `NodeKind::LightProbeGrid`; its position is the box's
//! center.
//!
//! `bake` follows LightProbeGrid.bake(): per batch of eight probes, the six
//! 90° faces of each into its tiles of a face atlas ( the grid itself hidden,
//! SunLights replaced by directional bake lights whose shadow covers the
//! casters ), then the SH projection of the batch's rows ( 9 coefficients ×
//! 6 faces × 4 row blocks, each texel weighted by its solid angle ) into a
//! float batch target, and the seven repack passes summing the face blocks
//! into the atlas with its boundary padding. Indirect passes light the
//! captures with a snapshot of the previous pass. The SH projection and
//! repack shaders are the WGSL three.js r187 generates for them.
//!
//! The page's ArrayCamera draws the 48 sub-cameras' viewports in one render;
//! here each sub-camera is one render into its viewport of the face atlas
//! ( the atlas cleared by the first ), as the WebXR views are drawn.
use crate::{
    Error, Result,
    camera::{Camera, PerspectiveCamera},
    math::*,
    render_target::{RenderTarget, RenderTargetOptions},
    renderer::Renderer,
    scene::{Light, NodeKind, Object3D, Scene},
};
use serde::Serialize;
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Padding texels at each boundary of every atlas sub-volume.
pub const ATLAS_PADDING: u32 = 1;
/// Probes captured per render, six sub-cameras each.
const PROBES_PER_BATCH: u32 = 8;
/// Row blocks per face, so each probe's projection spreads over more fragments.
const FACE_BLOCKS: u32 = 4;
/// The batch target's columns: 9 coefficients × 6 faces × FACE_BLOCKS.
const COLUMNS: u32 = 9 * 6 * FACE_BLOCKS;
/// Per-face forward and up vectors of the capture cameras.
const FACE_FORWARD: [Vector3; 6] = [
    Vector3::X,
    Vector3::NEG_X,
    Vector3::Y,
    Vector3::NEG_Y,
    Vector3::Z,
    Vector3::NEG_Z,
];
const FACE_UP: [Vector3; 6] = [
    Vector3::Y,
    Vector3::Y,
    Vector3::NEG_Z,
    Vector3::Z,
    Vector3::Y,
    Vector3::Y,
];
const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const FLOAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
const QUAD_VS: &str = include_str!("shaders/light_probe_grid/quad_vs.wgsl");
/// The projection for 8-texel faces ( the default cubemapSize ).
const SH_FS: &str = include_str!("shaders/light_probe_grid/sh_fs.wgsl");
const REPACK_FS: [&str; 7] = [
    include_str!("shaders/light_probe_grid/repack_0_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_1_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_2_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_3_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_4_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_5_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_6_fs.wgsl"),
];

/// The SH projection three.js r187 generates for `size`-texel faces
/// ( projectSHNode( faces, size, batchStart ) ): the face size, its rows per
/// block, texel size and half texel are constants of the generated loop. The
/// 8-texel shader with those constants replaced; `size` must be a multiple
/// of 4.
pub fn sh_source(size: u32) -> Result<String> {
    if size == 0 || !size.is_multiple_of(FACE_BLOCKS) {
        return Err(Error::Invalid(
            "LightProbeGrid: cubemapSize must be a multiple of 4.",
        ));
    }
    // TSL writes its float constants as JavaScript numbers ( `0.25` ).
    let float = |v: f64| format!("{v:?}");
    let rows = size / FACE_BLOCKS;
    let texel = float(2. / f64::from(size));
    let half = float(1. / f64::from(size));
    // Each pattern to a marker first, so a value never matches a later pattern.
    let mut out = SH_FS.to_owned();
    let mut values = vec![];
    for (from, to) in [
        (
            "( nodeConst0 % 4 ) * 2 )".to_owned(),
            format!("( nodeConst0 % 4 ) * {rows} )"),
        ),
        (
            "( nodeVar1 * 8 )".to_owned(),
            format!("( nodeVar1 * {size} )"),
        ),
        (
            "object.nodeUniform1 ) * 8 )".to_owned(),
            format!("object.nodeUniform1 ) * {size} )"),
        ),
        ("i < 8;".to_owned(), format!("i < {size};")),
        ("j < 2;".to_owned(), format!("j < {rows};")),
        ("+ 0.125 )".to_owned(), format!("+ {half} )")),
        ("- 0.125 )".to_owned(), format!("- {half} )")),
        ("* 0.25 )".to_owned(), format!("* {texel} )")),
        ("- 0.25 )".to_owned(), format!("- {texel} )")),
        ("+ 0.25 )".to_owned(), format!("+ {texel} )")),
    ] {
        if !out.contains(&from) {
            return Err(Error::Invalid("LightProbeGrid SH template"));
        }
        out = out.replace(&from, &format!("\u{0}{}\u{0}", values.len()));
        values.push(to);
    }
    for (i, to) in values.iter().enumerate() {
        out = out.replace(&format!("\u{0}{i}\u{0}"), to);
    }
    Ok(out)
}

/// A grid's atlas.
#[derive(Debug)]
pub struct GridAtlas {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
}
/// LightProbeGrid: the box's size, its probe counts, intensity and priority.
#[derive(Clone, Debug, Serialize)]
pub struct LightProbeGrid {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub resolution: [u32; 3],
    pub intensity: f64,
    /// Where grids overlap, a fragment takes the irradiance of the grid with
    /// the highest priority ( on ties the later one ).
    pub priority: f64,
    /// The baked atlas ( `texture` ), None before the first bake.
    #[serde(skip)]
    pub atlas: Option<Arc<GridAtlas>>,
}
impl LightProbeGrid {
    /// `new LightProbeGrid( width, height, depth, widthProbes, heightProbes,
    /// depthProbes )`; missing counts default to the rounded size ( at least
    /// one ).
    pub fn new(width: f64, height: f64, depth: f64, probes: Option<[u32; 3]>) -> Self {
        let count = |v: f64| (v.round() as u32).max(1);
        Self {
            width,
            height,
            depth,
            resolution: probes.unwrap_or([count(width), count(height), count(depth)]),
            intensity: 1.,
            priority: 0.,
            atlas: None,
        }
    }
    /// getProbePosition( ix, iy, iz ) around the box center `position`: each
    /// probe at the center of its cell.
    pub fn probe_position(&self, position: Vector3, index: [u32; 3]) -> Vector3 {
        let r = self.resolution;
        let axis = |p: f64, size: f64, i: u32, n: u32| {
            p - size / 2. + (f64::from(i) + 0.5) * size / f64::from(n)
        };
        Vector3::new(
            axis(position.x, self.width, index[0], r[0]),
            axis(position.y, self.height, index[1], r[1]),
            axis(position.z, self.depth, index[2], r[2]),
        )
    }
    /// boundingBox: the box around `position`.
    pub fn bounds(&self, position: Vector3) -> (Vector3, Vector3) {
        let half = Vector3::new(self.width, self.height, self.depth) * 0.5;
        (position - half, position + half)
    }
    pub fn probe_count(&self) -> u32 {
        self.resolution.iter().product()
    }
    fn atlas_size(&self) -> wgpu::Extent3d {
        let [nx, ny, nz] = self.resolution;
        wgpu::Extent3d {
            width: nx,
            height: ny,
            depth_or_array_layers: 7 * (nz + 2 * ATLAS_PADDING),
        }
    }
    fn create_atlas(&self, r: &Renderer) -> Arc<GridAtlas> {
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("LightProbeGrid atlas"),
            size: self.atlas_size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: HALF,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        Arc::new(GridAtlas { texture, view })
    }
}

/// LightProbeGrid.bake()'s options.
#[derive(Clone, Copy, Debug)]
pub struct BakeOptions {
    /// Each face's resolution, a multiple of 4.
    pub cubemap_size: u32,
    pub near: f64,
    pub far: f64,
    pub bounces: u32,
    pub start: u32,
    /// None bakes the probes from `start` to the end.
    pub count: Option<u32>,
    pub pass: u32,
}
impl Default for BakeOptions {
    fn default() -> Self {
        Self {
            cubemap_size: 8,
            near: 0.1,
            far: 100.,
            bounces: 0,
            start: 0,
            count: None,
            pass: 0,
        }
    }
}

/// A Z slice's repack uniforms and its bind group per repack pass.
type RepackSlice = (wgpu::Buffer, Vec<wgpu::BindGroup>);
/// The bake's pooled resources: the face atlas, the batch target and the SH
/// and repack pipelines, as the module-level pool of the original keeps them
/// between bakes.
pub struct GridBaker {
    /// The face atlas ( six tiles per batch slot ) by its face size, and the
    /// SH pass's pipeline and bind group over it.
    faces: Option<(u32, RenderTarget, wgpu::RenderPipeline, wgpu::BindGroup)>,
    /// Per resolution, each Z slice's repack uniforms and per-pass bind groups.
    repack_slices: Option<([u32; 3], Vec<RepackSlice>)>,
    /// The capture camera, kept between bakes.
    camera: Option<Object3D>,
    /// The batch target by its probe count.
    batch: Option<(u32, wgpu::Texture, wgpu::TextureView)>,
    /// The SH pass's objectStruct { opacity, batchStart, the face map's UV
    /// transform }.
    sh_object: wgpu::Buffer,
    repack: Vec<wgpu::RenderPipeline>,
    /// The snapshot grid lighting the indirect passes, and the pass it holds
    /// ( -1 before the first indirect pass ).
    bounce: Option<(Arc<GridAtlas>, i64)>,
    /// One directional bake light per SunLight.
    bake_lights: Vec<(Object3D, Object3D)>,
}
fn pipeline(
    r: &Renderer,
    label: &str,
    vs: &str,
    fs: &str,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module(vs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module(fs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}
fn bytes(values: &[(usize, &[f32])], size: usize) -> Vec<u8> {
    let mut data = vec![0u8; size];
    for (offset, v) in values {
        data[*offset..*offset + v.len() * 4].copy_from_slice(bytemuck::cast_slice(v));
    }
    data
}
/// A mat3x3 in a uniform buffer: three padded columns.
const IDENTITY3: [f32; 12] = [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0.];
impl GridBaker {
    pub fn new(r: &Renderer) -> Self {
        Self {
            faces: None,
            repack_slices: None,
            camera: None,
            batch: None,
            sh_object: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("LightProbeGrid SH object"),
                size: 64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            repack: REPACK_FS
                .iter()
                .map(|fs| pipeline(r, "LightProbeGrid repack", QUAD_VS, fs, HALF))
                .collect(),
            bounce: None,
            bake_lights: vec![],
        }
    }
    /// ensureBakeTargets() and the SH material: the face atlas and the SH
    /// pipeline per face size, the batch target per probe count.
    fn ensure_targets(&mut self, r: &Renderer, size: u32, total: u32) -> Result<()> {
        if self.faces.as_ref().is_none_or(|f| f.0 != size) {
            let target = RenderTarget::with_options(
                &r.device,
                6 * size,
                PROBES_PER_BATCH * size,
                RenderTargetOptions {
                    format: HALF,
                    ..Default::default()
                },
            )?;
            let sh = pipeline(r, "LightProbeGrid SH", QUAD_VS, &sh_source(size)?, FLOAT);
            let group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &sh.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.sh_object.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&target.view),
                    },
                ],
            });
            self.faces = Some((size, target, sh, group));
        }
        if self.batch.as_ref().is_none_or(|b| b.0 != total) {
            let texture = r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("LightProbeGrid batch"),
                size: wgpu::Extent3d {
                    width: COLUMNS,
                    height: total,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: FLOAT,
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            let view = texture.create_view(&Default::default());
            self.batch = Some((total, texture, view));
            self.repack_slices = None;
        }
        Ok(())
    }
}

/// The world box of the shadow casters ( Box3.expandByObject over meshes
/// casting shadows ).
fn caster_box(scene: &Scene) -> Result<Option<(Vector3, Vector3)>> {
    let mut lo = Vector3::splat(f64::INFINITY);
    let mut hi = Vector3::splat(f64::NEG_INFINITY);
    for root in scene.roots() {
        for h in scene.traverse(root, false)? {
            let n = scene.get(h)?;
            let NodeKind::Mesh(mesh) = &n.kind else {
                continue;
            };
            if !n.cast_shadow {
                continue;
            }
            let local = match mesh.geometry.bounding_box {
                Some(b) => b,
                None => Box3::from_points(mesh.geometry.positions()?),
            };
            let world = local.transformed(n.matrix_world);
            lo = lo.min(world.min);
            hi = hi.max(world.max);
        }
    }
    Ok((lo.x <= hi.x).then_some((lo, hi)))
}

/// LightProbeGrid.bake( renderer, scene, options ) for the grid node `grid`.
pub fn bake(
    baker: &mut GridBaker,
    r: &Renderer,
    scene: &mut Scene,
    grid: Object3D,
    options: BakeOptions,
) -> Result<()> {
    let (mut g, position) = match &scene.get(grid)?.kind {
        NodeKind::LightProbeGrid(g) => ((**g).clone(), scene.get(grid)?.position),
        _ => return Err(Error::Invalid("bake: not a LightProbeGrid")),
    };
    let total = g.probe_count();
    let start = options.start;
    let count = options.count.unwrap_or(total.saturating_sub(start));
    let end = start + count;
    if end > total {
        return Err(Error::Invalid("LightProbeGrid: Invalid probe range."));
    }
    if options.bounces > 0 && count != total {
        return Err(Error::Invalid(
            "LightProbeGrid: For ranged baking, use pass instead of bounces.",
        ));
    }
    if count == 0 {
        return Ok(());
    }
    if options.pass > 0
        && start > 0
        && baker
            .bounce
            .as_ref()
            .is_none_or(|b| b.1 != i64::from(options.pass))
    {
        return Err(Error::Invalid(
            "LightProbeGrid: Start each indirect pass at probe 0.",
        ));
    }
    // _ensureTextures()
    let atlas = match &g.atlas {
        Some(a) if a.texture.size() == g.atlas_size() => a.clone(),
        _ => {
            let a = g.create_atlas(r);
            g.atlas = Some(a.clone());
            if let NodeKind::LightProbeGrid(live) = &mut scene.get_mut(grid)?.kind {
                live.atlas = Some(a.clone());
            }
            a
        }
    };
    baker.ensure_targets(r, options.cubemap_size, total)?;
    // The bake's scene state: the grid out of the captures, the SunLights
    // replaced, every shadow rendered once.
    scene.update()?;
    let visible = scene.get(grid)?.visible;
    scene.get_mut(grid)?.visible = false;
    let replaced = replace_sun_lights(baker, scene)?;
    let shadow_auto_update = scene.shadow_auto_update;
    // The capture sub-cameras: PerspectiveCamera( 90, 1, near, far ).
    let projection = Camera::Perspective(PerspectiveCamera {
        fov: 90.,
        aspect: 1.,
        near: options.near,
        far: options.far,
        ..Default::default()
    });
    let camera = match baker.camera {
        Some(h) if scene.get(h).is_ok() => h,
        _ => {
            let h = scene.insert(NodeKind::Camera(projection.clone()));
            baker.camera = Some(h);
            h
        }
    };
    scene.get_mut(camera)?.kind = NodeKind::Camera(projection);
    let mut bounce_node = None;
    let result = (|| -> Result<()> {
        let mut first = true;
        for pass in options.pass..=options.pass + options.bounces {
            if let Some(h) = bounce_node.take() {
                scene.dispose(h)?;
            }
            bounce_node = Some(update_bounce_grid(
                baker, r, scene, &g, position, &atlas, pass, start,
            )?);
            capture(
                baker, r, scene, camera, &g, position, start, end, &mut first,
            )?;
            repack(baker, r, &g, &atlas, start, end);
        }
        Ok(())
    })();
    if let Some(h) = bounce_node {
        scene.dispose(h)?;
    }
    scene.shadow_auto_update = shadow_auto_update;
    restore_sun_lights(scene, &replaced)?;
    scene.get_mut(grid)?.visible = visible;
    result
}
/// _updateBounceGrid(): the bounce grid lights the captures, its atlas a
/// snapshot of the live one taken at an indirect pass's first probe; the
/// direct pass adds it unlit ( intensity 0 ), so it still takes the
/// fragments inside it from grids of a lower priority.
#[allow(clippy::too_many_arguments)]
fn update_bounce_grid(
    baker: &mut GridBaker,
    r: &Renderer,
    scene: &mut Scene,
    g: &LightProbeGrid,
    position: Vector3,
    atlas: &GridAtlas,
    pass: u32,
    start: u32,
) -> Result<Object3D> {
    let snapshot = match &baker.bounce {
        Some((a, _)) if a.texture.size() == atlas.texture.size() => a.clone(),
        _ => {
            let a = g.create_atlas(r);
            baker.bounce = Some((a.clone(), -1));
            a
        }
    };
    if pass == 0 {
        if start == 0 {
            baker.bounce = Some((snapshot.clone(), -1));
        }
    } else if start == 0 {
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_texture(
            atlas.texture.as_image_copy(),
            snapshot.texture.as_image_copy(),
            atlas.texture.size(),
        );
        r.queue.submit([encoder.finish()]);
        baker.bounce = Some((snapshot.clone(), i64::from(pass)));
    }
    let mut copy = g.clone();
    copy.atlas = Some(snapshot);
    if pass == 0 {
        copy.intensity = 0.;
    }
    let h = scene.insert(NodeKind::LightProbeGrid(Box::new(copy)));
    scene.get_mut(h)?.position = position;
    Ok(h)
}
/// _captureProbes(): per batch of eight probes, each probe's six faces into
/// its row of the face atlas, then the batch's SH rows.
#[allow(clippy::too_many_arguments)]
fn capture(
    baker: &mut GridBaker,
    r: &Renderer,
    scene: &mut Scene,
    camera: Object3D,
    g: &LightProbeGrid,
    position: Vector3,
    start: u32,
    end: u32,
    first: &mut bool,
) -> Result<()> {
    let [nx, _, nz] = g.resolution;
    let per_layer = nx * nz;
    let (Some((size, target, sh, group)), Some((_, _, batch))) = (&mut baker.faces, &baker.batch)
    else {
        return Err(Error::Invalid("LightProbeGrid bake targets"));
    };
    let size = *size;
    for batch_start in (start..end).step_by(PROBES_PER_BATCH as usize) {
        let batch_count = PROBES_PER_BATCH.min(end - batch_start);
        // renderer.render( scene, captureCamera ): the atlas cleared once,
        // then every used sub-camera in its viewport ( the unused slots
        // draw nothing ).
        for slot in 0..batch_count {
            let probe = batch_start + slot;
            let (ix, iy, iz) = (probe % nx, probe / per_layer, (probe / nx) % nz);
            let p = g.probe_position(position, [ix, iy, iz]);
            scene.get_mut(camera)?.position = p;
            for face in 0..6 {
                scene.get_mut(camera)?.up = FACE_UP[face];
                scene.look_at(camera, p + FACE_FORWARD[face])?;
                target.viewport = [face as u32 * size, slot * size, size, size];
                target.set_load_color(!(slot == 0 && face == 0));
                r.render(scene, camera, target)?;
                if *first {
                    // Every shadow map renders once for the bake.
                    scene.shadow_auto_update = false;
                    *first = false;
                }
            }
        }
        target.set_load_color(false);
        target.viewport = [0, 0, target.width, target.height];
        // The SH projection of the batch's rows ( the batch target's rows
        // follow the bake order ).
        r.queue.write_buffer(
            &baker.sh_object,
            0,
            &bytes(
                &[
                    (0, &[1.]),
                    (4, &[f32::from_bits(batch_start)]),
                    (16, &IDENTITY3),
                ],
                64,
            ),
        );
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("LightProbeGrid SH"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: batch,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_viewport(
                0.,
                batch_start as f32,
                COLUMNS as f32,
                batch_count as f32,
                0.,
                1.,
            );
            pass.set_pipeline(sh);
            pass.set_bind_group(0, &*group, &[]);
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
    }
    Ok(())
}
/// _repackProbes(): the baked range's rectangles in each Z slice, into the
/// slice and, at the ends, the padding slices.
fn repack(
    baker: &mut GridBaker,
    r: &Renderer,
    g: &LightProbeGrid,
    atlas: &GridAtlas,
    start: u32,
    end: u32,
) {
    let Some((_, _, batch)) = &baker.batch else {
        return;
    };
    let [nx, ny, nz] = g.resolution;
    if baker
        .repack_slices
        .as_ref()
        .is_none_or(|(res, _)| *res != g.resolution)
    {
        // objectStruct { nodeUniform0 ( opacity ), nodeUniform1 ( slice ),
        // nodeUniform2 ( resolution ), nodeUniform4 ( UV transform ) }, one
        // per Z slice.
        let slices = (0..nz)
            .map(|iz| {
                let object = r
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("LightProbeGrid repack object"),
                        contents: &bytes(
                            &[
                                (0, &[1.]),
                                (4, &[f32::from_bits(iz)]),
                                (16, &[nx as f32, ny as f32, nz as f32]),
                                (32, &IDENTITY3),
                            ],
                            80,
                        ),
                        usage: wgpu::BufferUsages::UNIFORM,
                    });
                let groups = baker
                    .repack
                    .iter()
                    .map(|pipeline| {
                        r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: None,
                            layout: &pipeline.get_bind_group_layout(0),
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: object.as_entire_binding(),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::TextureView(batch),
                                },
                            ],
                        })
                    })
                    .collect();
                (object, groups)
            })
            .collect();
        baker.repack_slices = Some((g.resolution, slices));
    }
    let Some((_, slices)) = &baker.repack_slices else {
        return;
    };
    let per_layer = nx * nz;
    let (start_y, end_y) = (start / per_layer, end / per_layer);
    let padded = nz + 2 * ATLAS_PADDING;
    let mut encoder = r.device.create_command_encoder(&Default::default());
    for iz in 0..nz {
        let clamp = |v: i64| v.clamp(0, i64::from(nx)) as u32;
        let slice_start = start_y * nx + clamp(i64::from(start % per_layer) - i64::from(iz * nx));
        let slice_end = end_y * nx + clamp(i64::from(end % per_layer) - i64::from(iz * nx));
        let groups = &slices[iz as usize].1;
        let mut probe = slice_start;
        while probe < slice_end {
            let (ix, iy) = (probe % nx, probe / nx);
            // Complete rows within a slice coalesce into one rectangle.
            let width = (nx - ix).min(slice_end - probe);
            let height = if width == nx {
                (ny - iy).min((slice_end - probe) / nx)
            } else {
                1
            };
            for (t, pipeline) in baker.repack.iter().enumerate() {
                let base = t as u32 * padded;
                let mut targets = vec![base + ATLAS_PADDING + iz];
                if iz == 0 {
                    targets.push(base);
                }
                if iz == nz - 1 {
                    targets.push(base + ATLAS_PADDING + nz);
                }
                for slice in targets {
                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("LightProbeGrid repack"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &atlas.view,
                            depth_slice: Some(slice),
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        ..Default::default()
                    });
                    pass.set_viewport(ix as f32, iy as f32, width as f32, height as f32, 0., 1.);
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, &groups[t], &[]);
                    pass.draw(0..3, 0..1);
                }
            }
            probe += width * height;
        }
    }
    r.queue.submit([encoder.finish()]);
}
/// replaceSunLights(): each visible shadow-casting SunLight hidden behind a
/// directional bake light whose shadow camera covers the casters' sphere.
fn replace_sun_lights(
    baker: &mut GridBaker,
    scene: &mut Scene,
) -> Result<Vec<(Object3D, Object3D)>> {
    let mut suns = vec![];
    for root in scene.roots() {
        for h in scene.traverse(root, false)? {
            let n = scene.get(h)?;
            if n.visible && n.cast_shadow && matches!(n.kind, NodeKind::Light(Light::Sun { .. })) {
                suns.push(h);
            }
        }
    }
    if suns.is_empty() {
        return Ok(vec![]);
    }
    let (center, radius) = match caster_box(scene)? {
        Some((lo, hi)) => ((lo + hi) * 0.5, ((hi - lo).length() * 0.5).max(1.)),
        None => (Vector3::ZERO, 1.),
    };
    let mut replaced = vec![];
    for (k, sun) in suns.into_iter().enumerate() {
        let n = scene.get(sun)?;
        let (color, intensity) = match n.kind {
            NodeKind::Light(Light::Sun { color, intensity }) => (color, intensity),
            _ => continue,
        };
        let map_size = n.shadow.map_size;
        let direction = n.matrix_world.w_axis.truncate().normalize_or_zero();
        let light = match baker.bake_lights.get(k) {
            Some(&(_, light)) if scene.get(light).is_ok() => light,
            _ => {
                let light = scene.insert(NodeKind::Group);
                baker.bake_lights.push((sun, light));
                light
            }
        };
        let l = scene.get_mut(light)?;
        l.kind = NodeKind::Light(Light::Directional {
            color,
            intensity,
            target: center,
        });
        l.visible = true;
        l.cast_shadow = true;
        l.position = center + direction * (radius * 2.);
        l.shadow.map_size = map_size;
        l.shadow.extent = radius;
        l.shadow.near = radius * 0.5;
        l.shadow.far = radius * 3.5;
        scene.get_mut(sun)?.visible = false;
        replaced.push((sun, light));
    }
    scene.update()?;
    Ok(replaced)
}
fn restore_sun_lights(scene: &mut Scene, replaced: &[(Object3D, Object3D)]) -> Result<()> {
    for &(sun, light) in replaced {
        scene.get_mut(light)?.visible = false;
        scene.get_mut(sun)?.visible = true;
    }
    Ok(())
}

/// LightProbeGridHelper's material: each instance's SH irradiance from the
/// grid's atlas at its texel-center UVW ( the instance color ), for the
/// world normal.
const HELPER: &str = r#"
fn probe_helper(normal:vec3<f32>,uvw:vec3<f32>)->vec4<f32>{
 let res=u.probe_grid[2].xyz;let padded=res.z+2.0;let depth=padded*7.0;let base=uvw.z*res.z+1.0;
 var s:array<vec4<f32>,7>;
 for(var t=0;t<7;t++){s[t]=textureSampleLevel(grid_atlas,grid_sampler,vec3(uvw.xy,(base+padded*f32(t))/depth),0.0);}
 let sh=array<vec3<f32>,9>(s[0].xyz,vec3(s[0].w,s[1].xy),vec3(s[1].zw,s[2].x),s[2].yzw,s[3].xyz,vec3(s[3].w,s[4].xy),vec3(s[4].zw,s[5].x),s[5].yzw,s[6].xyz);
 let n=normal;
 let e=sh[0]*0.886227+sh[1]*1.023328*n.y+sh[2]*1.023328*n.z+sh[3]*1.023328*n.x+sh[4]*0.858086*n.x*n.y+sh[5]*0.858086*n.y*n.z+sh[6]*(n.z*n.z*0.743125-0.247708)+sh[7]*0.858086*n.x*n.z+sh[8]*0.429043*(n.x*n.x-n.y*n.y);
 return vec4(max(e,vec3(0.0)),1.0);
}"#;
/// The helper's shader program, shared by helpers.
pub async fn helper_program(r: &Renderer) -> Result<Arc<crate::shader::ShaderProgram>> {
    use crate::tsl::*;
    let color = WgslFn::new(
        "probe_helper",
        HELPER,
        &[Type::Vec3, Type::Vec3],
        Type::Vec4,
    )?
    .call(&[normal_world(), base_color().rgb()]);
    Ok(Arc::new(
        crate::shader::ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(0)?,
            &[],
            &[],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?,
    ))
}
/// `new LightProbeGridHelper( probes, sphereSize )`: one sphere per probe at
/// its position, shaded by its irradiance.
pub fn helper(
    scene: &mut Scene,
    grid: Object3D,
    sphere_size: f64,
    program: &Arc<crate::shader::ShaderProgram>,
) -> Result<Object3D> {
    let n = scene.get(grid)?;
    let (g, position) = match &n.kind {
        NodeKind::LightProbeGrid(g) => ((**g).clone(), n.position),
        _ => return Err(Error::Invalid("helper: not a LightProbeGrid")),
    };
    let [nx, ny, nz] = g.resolution;
    let mut instances = Vec::with_capacity(g.probe_count() as usize);
    for iz in 0..nz {
        for iy in 0..ny {
            for ix in 0..nx {
                // Texel centers, as LightProbeGridNode samples them.
                let uvw = [(ix, nx), (iy, ny), (iz, nz)]
                    .map(|(i, n)| ((f64::from(i) + 0.5) / f64::from(n)) as f32 as f64);
                instances.push(crate::scene::Instance {
                    matrix: Matrix4::from_translation(g.probe_position(position, [ix, iy, iz])),
                    color: Color::linear(uvw[0], uvw[1], uvw[2]),
                });
            }
        }
    }
    let material = crate::material::ShaderMaterial::new(program.clone());
    let h = scene.insert(NodeKind::Mesh(crate::scene::Mesh::new(
        Arc::new(crate::geometry::SphereGeometry::build(sphere_size, 16, 16)?),
        Arc::new(crate::material::Material::Shader(material)),
    )));
    scene.get_mut(h)?.instances = instances;
    Ok(h)
}

#[cfg(test)]
mod tests {
    use super::*;
    /// r187's generated projection for 32-texel faces ( webgpu_lightprobes ).
    const SH_FS_32: &str =
        include_str!("../gallery/src/browser/lightprobes_complex/probe_sh_fs.wgsl");
    #[test]
    fn the_sh_projection_follows_the_face_size() {
        assert_eq!(sh_source(8).unwrap(), SH_FS);
        assert_eq!(sh_source(32).unwrap(), SH_FS_32);
        let s16 = sh_source(16).unwrap();
        for part in [
            "i < 16;",
            "j < 4;",
            "* 0.125 )",
            "+ 0.0625 )",
            "( nodeVar1 * 16 )",
        ] {
            assert!(s16.contains(part), "{part}");
        }
        // A value equal to another pattern is not replaced twice.
        let s4 = sh_source(4).unwrap();
        assert!(s4.contains("+ 0.25 )") && s4.contains("* 0.5 )") && s4.contains("j < 1;"));
        assert!(sh_source(6).is_err());
    }
}
