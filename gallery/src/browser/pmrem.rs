//! r187's PMREMGenerator ( WebGPU ): a PMREM is a half-float cube render
//! target whose levels, down to 8 texels ( LOD_MIN 3 ), hold the radiance
//! prefiltered for roughness `1 - sqrt( 1 - lod / maxLod )`. The input goes
//! into a source cube target of the same size ( an equirect or cube texture
//! through the box drawn by CubeCamera's six faces, or a scene the caller
//! captures with [`face_cameras`] ), which gets its mip chain as
//! WebGPUTexturePassUtils generates it; fromScene's blur, when its sigma is
//! above zero, runs before the mipmaps. Each level of the PMREM is then drawn
//! from the source: GGX VNDF importance sampling ( 256 samples ) for the sharp
//! levels and the integration over a 16-texel source mip for the three
//! roughest. Every stage runs the WGSL r187 generates for it ( pmrem/ ), all
//! on the GPU. The targets, the passes and their uniforms are built once, so
//! regenerating ( PMREMNode does each frame for a CubeCamera's target )
//! allocates nothing.
mod room;
use super::gltf_viewer::fetch;
use super::lights_projector::{m3, m4, pack};
use crate::{Error, Result, geometry::BoxGeometry, math::*, renderer::Renderer};
pub(super) use room::room_environment;
use wgpu::util::DeviceExt;

pub(super) const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const EQUIRECT_VS: &str = include_str!("pmrem/equirect_vs.wgsl");
const EQUIRECT_FS: &str = include_str!("pmrem/equirect_fs.wgsl");
const CUBEMAP_VS: &str = include_str!("pmrem/cubemap_vs.wgsl");
const CUBEMAP_FS: &str = include_str!("pmrem/cubemap_fs.wgsl");
const BLUR_VS: &str = include_str!("pmrem/blur_vs.wgsl");
const BLUR_FS: &str = include_str!("pmrem/blur_fs.wgsl");
const GGX_VS: &str = include_str!("pmrem/ggx_vs.wgsl");
const GGX_FS: &str = include_str!("pmrem/ggx_fs.wgsl");
const INTEGRATION_VS: &str = include_str!("pmrem/integration_vs.wgsl");
const INTEGRATION_FS: &str = include_str!("pmrem/integration_fs.wgsl");
const MIPMAP: &str = include_str!("pmrem/mipmap.wgsl");
const MIN_SIZE: u32 = 256;
const LOD_MIN: u32 = 3;
const GGX_SAMPLES: f64 = 256.;
const INTEGRATION_SIZE: f64 = 16.;
const INTEGRATION_LEVELS: u32 = 3;

/// PMREMGenerator._setSize: a power of two, at least 256.
pub(super) fn cube_size(size: u32) -> u32 {
    MIN_SIZE.max(1 << (31 - size.max(1).leading_zeros()))
}

/// CubeCamera( near, far )'s six face cameras in the WebGPU coordinate system
/// ( fov -90, at `position` ): each one's projection, view and world matrices,
/// in the cube's layer order.
pub(super) fn face_cameras(
    near: f64,
    far: f64,
    position: Vector3,
) -> [(Matrix4, Matrix4, Matrix4); 6] {
    // PerspectiveCamera.updateProjectionMatrix with fov -90: top = -near.
    let projection = Matrix4::from_cols_array(&[
        -1.,
        0.,
        0.,
        0., //
        0.,
        -1.,
        0.,
        0., //
        0.,
        0.,
        -far / (far - near),
        -1., //
        0.,
        0.,
        -far * near / (far - near),
        0.,
    ]);
    // Object3D.lookAt( target ) with the camera's up.
    let look = |target: Vector3, up: Vector3| {
        let z = (-target).normalize();
        let x = up.cross(z).normalize();
        let y = z.cross(x);
        let world = Matrix4::from_cols(
            x.extend(0.),
            y.extend(0.),
            z.extend(0.),
            position.extend(1.),
        );
        (projection, world.inverse(), world)
    };
    [
        look(Vector3::new(-1., 0., 0.), Vector3::new(0., -1., 0.)),
        look(Vector3::new(1., 0., 0.), Vector3::new(0., -1., 0.)),
        look(Vector3::new(0., 1., 0.), Vector3::new(0., 0., 1.)),
        look(Vector3::new(0., -1., 0.), Vector3::new(0., 0., -1.)),
        look(Vector3::new(0., 0., 1.), Vector3::new(0., -1., 0.)),
        look(Vector3::new(0., 0., -1.), Vector3::new(0., -1., 0.)),
    ]
}

/// A cube render target's texture: half float, six layers, `levels` mips.
pub(super) fn cube_texture(r: &Renderer, label: &str, size: u32, levels: u32) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 6,
        },
        mip_level_count: levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: HALF,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}
/// A cube view of every level ( None ) or of one.
pub(super) fn cube_view(texture: &wgpu::Texture, level: Option<u32>) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::Cube),
        base_mip_level: level.unwrap_or(0),
        mip_level_count: level.map(|_| 1),
        ..Default::default()
    })
}
/// The 2D view of one face of one level: a render pass's target.
pub(super) fn face_view(texture: &wgpu::Texture, face: u32, level: u32) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2),
        base_mip_level: level,
        mip_level_count: Some(1),
        base_array_layer: face,
        array_layer_count: Some(1),
        ..Default::default()
    })
}
fn bind(
    r: &Renderer,
    layout: wgpu::BindGroupLayout,
    entries: &[(u32, wgpu::BindingResource)],
) -> wgpu::BindGroup {
    r.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("PMREM"),
        layout: &layout,
        entries: &entries
            .iter()
            .map(|(binding, resource)| wgpu::BindGroupEntry {
                binding: *binding,
                resource: resource.clone(),
            })
            .collect::<Vec<_>>(),
    })
}
fn module(r: &Renderer, label: &str, source: &str) -> wgpu::ShaderModule {
    r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}
fn uniform(r: &Renderer, label: &str, data: &[u8]) -> wgpu::Buffer {
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
}
fn sampler(r: &Renderer, mag: wgpu::FilterMode, mipmap: wgpu::FilterMode) -> wgpu::Sampler {
    r.device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("PMREM"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: mag,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: mipmap,
        ..Default::default()
    })
}
fn pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("PMREM"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        })],
        ..Default::default()
    })
}

/// WebGPUTexturePassUtils.generateMipmaps for a cube texture: each level from
/// the one above, a face at a time ( main_cube, minFilter linear, no flip ).
pub(super) struct CubeMipmaps {
    pipeline: wgpu::RenderPipeline,
    /// Per level from 1: the bind group over the level above and the six faces.
    levels: Vec<(wgpu::BindGroup, Vec<wgpu::TextureView>)>,
}
impl CubeMipmaps {
    pub(super) fn new(r: &Renderer, texture: &wgpu::Texture) -> Self {
        let mipmap = module(r, "PMREM mipmap", MIPMAP);
        let pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("PMREM mipmap"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &mipmap,
                    entry_point: Some("mainVS"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &mipmap,
                    entry_point: Some("main_cube"),
                    compilation_options: Default::default(),
                    targets: &[Some(texture.format().into())],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        let minify = sampler(r, wgpu::FilterMode::Nearest, wgpu::FilterMode::Nearest);
        let no_flip = uniform(r, "PMREM no flip", bytemuck::bytes_of(&0u32));
        let levels = (1..texture.mip_level_count())
            .map(|level| {
                let above = cube_view(texture, Some(level - 1));
                let group = bind(
                    r,
                    pipeline.get_bind_group_layout(0),
                    &[
                        (0, wgpu::BindingResource::Sampler(&minify)),
                        (1, wgpu::BindingResource::TextureView(&above)),
                        (2, no_flip.as_entire_binding()),
                    ],
                );
                (
                    group,
                    (0..6).map(|f| face_view(texture, f, level)).collect(),
                )
            })
            .collect();
        Self { pipeline, levels }
    }
    pub(super) fn encode(&self, encoder: &mut wgpu::CommandEncoder) {
        for (group, faces) in &self.levels {
            for (face, target) in faces.iter().enumerate() {
                let face = face as u32;
                let mut p = pass(encoder, target);
                p.set_pipeline(&self.pipeline);
                p.set_bind_group(0, group, &[]);
                p.draw(0..3, face..face + 1);
            }
        }
    }
}

/// A PMREM material: its pipeline and, per face, its camera's bind group.
struct Material {
    pipeline: wgpu::RenderPipeline,
    cameras: Vec<wgpu::BindGroup>,
    normals: bool,
}
impl Material {
    fn new(r: &Renderer, label: &str, (vs, fs): (&str, &str), normals: bool) -> Result<Self> {
        let position = [wgpu::vertex_attr_array![0 => Float32x3]];
        let normal = [wgpu::vertex_attr_array![1 => Float32x3]];
        let layout = |attributes| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes,
        };
        let buffers = if normals {
            vec![layout(&position[0]), layout(&normal[0])]
        } else {
            vec![layout(&position[0])]
        };
        let (vs_module, fs_module) = (module(r, label, vs), module(r, label, fs));
        // NoBlending, no depth; the box seen from inside ( BackSide ).
        let pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &vs_module,
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    buffers: &buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fs_module,
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    targets: &[Some(HALF.into())],
                }),
                // Seen from inside, only the box's back faces cover a face.
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        let cameras = face_cameras(1., 10., Vector3::ZERO)
            .iter()
            .map(|(projection, view, world)| {
                let values: [(&str, &[f64]); 3] = [
                    ("cameraProjectionMatrix", &m4(*projection)),
                    ("cameraViewMatrix", &m4(*view)),
                    ("cameraWorldMatrix", &m4(*world)),
                ];
                // The render struct both stages bind: the vertex stage's, or
                // the fragment's when it holds more ( cameraWorldMatrix ).
                let data = [vs, fs]
                    .iter()
                    .filter(|s| s.contains("struct renderStruct"))
                    .map(|s| pack(s, "renderStruct", &values))
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .max_by_key(|d| d.len())
                    .ok_or(Error::Invalid("PMREM render struct"))?;
                Ok(bind(
                    r,
                    pipeline.get_bind_group_layout(0),
                    &[(0, uniform(r, "PMREM camera", &data).as_entire_binding())],
                ))
            })
            .collect::<Result<_>>()?;
        Ok(Self {
            pipeline,
            cameras,
            normals,
        })
    }
}

/// BoxGeometry( 5, 5, 5 ): positions, normals and the index.
struct BoxMesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
}
impl BoxMesh {
    fn new(r: &Renderer) -> Result<Self> {
        let mut geometry = BoxGeometry::build(5., 5., 5.)?;
        let floats = |name: &str| -> Result<Vec<f32>> {
            let a = geometry
                .attributes
                .get(name)
                .ok_or(Error::Invalid("PMREM box"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let buffer = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let positions = buffer(
            "PMREM box position",
            bytemuck::cast_slice(&floats("position")?),
            vertex,
        );
        let normals = buffer(
            "PMREM box normal",
            bytemuck::cast_slice(&floats("normal")?),
            vertex,
        );
        let index = geometry
            .index
            .take()
            .ok_or(Error::Invalid("PMREM box index"))?;
        Ok(Self {
            positions,
            normals,
            index: buffer(
                "PMREM box index",
                bytemuck::cast_slice(&index),
                wgpu::BufferUsages::INDEX,
            ),
            count: index.len() as u32,
        })
    }
    /// CubeCamera.update over the box: `material` into the six `targets`.
    fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        material: &Material,
        group: &wgpu::BindGroup,
        targets: &[wgpu::TextureView],
    ) {
        for (target, camera) in targets.iter().zip(&material.cameras) {
            let mut p = pass(encoder, target);
            p.set_pipeline(&material.pipeline);
            p.set_bind_group(0, camera, &[]);
            p.set_bind_group(1, group, &[]);
            p.set_vertex_buffer(0, self.positions.slice(..));
            if material.normals {
                p.set_vertex_buffer(1, self.normals.slice(..));
            }
            p.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
            p.draw_indexed(0..self.count, 0, 0..1);
        }
    }
}

/// An input texture drawn into the source target ( _textureToCubemap ): the
/// equirect or the cubemap material with its bind group.
pub(super) struct Source {
    material: Material,
    group: wgpu::BindGroup,
}

/// fromScene's blur: the material and its two passes ( the source into the
/// PMREM's level 0, then back ).
struct Blur {
    material: Material,
    object: wgpu::Buffer,
    groups: [wgpu::BindGroup; 2],
}

/// One level of the PMREM: GGX or integration, its material's bind group and
/// its six faces.
struct Level {
    integration: bool,
    group: wgpu::BindGroup,
    faces: Vec<wgpu::TextureView>,
}

/// A PMREM ( PMREMGenerator's cube render target ) with its generator's
/// source target and passes.
pub(super) struct Pmrem {
    /// The face size and the last level ( PMREMNode's maxLod ).
    pub size: u32,
    pub max_lod: u32,
    /// Every level, as materials sample it.
    pub view: wgpu::TextureView,
    source: wgpu::Texture,
    source_faces: Vec<wgpu::TextureView>,
    source_mipmaps: CubeMipmaps,
    box_mesh: BoxMesh,
    ggx: Material,
    integration: Material,
    levels: Vec<Level>,
    blur: Option<Blur>,
    trilinear: wgpu::Sampler,
}
impl Pmrem {
    /// The targets and passes for a face size of `size` ( _setSize ).
    pub(super) fn new(r: &Renderer, size: u32) -> Result<Self> {
        let size = cube_size(size);
        let levels = size.ilog2() + 1;
        let max_lod = size.ilog2() - LOD_MIN;
        let source = cube_texture(r, "PMREM source", size, levels);
        let texture = cube_texture(r, "PMREM", size, max_lod + 1);
        // The render targets' LinearMipmapLinearFilter.
        let trilinear = sampler(r, wgpu::FilterMode::Linear, wgpu::FilterMode::Linear);
        let ggx = Material::new(r, "PMREM ggx", (GGX_VS, GGX_FS), false)?;
        let integration = Material::new(
            r,
            "PMREM integration",
            (INTEGRATION_VS, INTEGRATION_FS),
            true,
        )?;
        let identity = m4(Matrix4::IDENTITY);
        let identity3 = m3(Matrix4::IDENTITY);
        let source_view = cube_view(&source, None);
        let source_lod = (size as f64 / INTEGRATION_SIZE).log2();
        // _applyPMREM's uniforms per level.
        let levels = (0..=max_lod)
            .map(|lod| {
                // PMREMGenerator.lodToRoughness
                let roughness = if max_lod > 0 {
                    1. - (1. - lod as f64 / max_lod as f64).sqrt()
                } else {
                    0.
                };
                let integration_level = lod + INTEGRATION_LEVELS > max_lod;
                let (material, object) = if integration_level {
                    let object = pack(
                        INTEGRATION_FS,
                        "objectStruct",
                        &[
                            ("nodeUniform0", &identity),
                            ("nodeUniform1", &[roughness]),
                            ("nodeUniform2", &[INTEGRATION_SIZE]),
                            ("nodeUniform5", &identity3),
                            ("nodeUniform7", &[source_lod]),
                        ],
                    )?;
                    (&integration, object)
                } else {
                    // For V = N, pdf( L ) = D( H ) / 4: the source texel's
                    // solid angle matched to 1 / ( GGX_SAMPLES * pdf ), with a
                    // half-mip bias.
                    let lod_bias = if roughness > 0. {
                        (size as f64).log2()
                            + 0.5 * (6. / (GGX_SAMPLES * roughness.powi(4))).log2()
                            + 0.5
                    } else {
                        0.
                    };
                    let object = pack(
                        GGX_FS,
                        "objectStruct",
                        &[
                            ("nodeUniform0", &identity),
                            ("nodeUniform1", &[roughness]),
                            ("nodeUniform3", &[lod_bias]),
                        ],
                    )?;
                    (&ggx, object)
                };
                let object = uniform(r, "PMREM object", &object);
                let group = bind(
                    r,
                    material.pipeline.get_bind_group_layout(1),
                    &[
                        (0, object.as_entire_binding()),
                        (1, wgpu::BindingResource::Sampler(&trilinear)),
                        (2, wgpu::BindingResource::TextureView(&source_view)),
                    ],
                );
                Ok(Level {
                    integration: integration_level,
                    group,
                    faces: (0..6).map(|f| face_view(&texture, f, lod)).collect(),
                })
            })
            .collect::<Result<_>>()?;
        Ok(Self {
            size,
            max_lod,
            view: cube_view(&texture, None),
            source_faces: (0..6).map(|f| face_view(&source, f, 0)).collect(),
            source_mipmaps: CubeMipmaps::new(r, &source),
            box_mesh: BoxMesh::new(r)?,
            source,
            ggx,
            integration,
            levels,
            blur: None,
            trilinear,
        })
    }
    /// Level 0's faces of the source target: what fromScene's CubeCamera
    /// renders into, `size`² each.
    pub(super) fn source_faces(&self) -> &[wgpu::TextureView] {
        &self.source_faces
    }
    /// The equirect material over `equirect` ( sampled through `sampler` ).
    pub(super) fn equirect_source(
        &self,
        r: &Renderer,
        equirect: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> Result<Source> {
        let material = Material::new(r, "PMREM equirect", (EQUIRECT_VS, EQUIRECT_FS), false)?;
        let object = uniform(
            r,
            "PMREM equirect object",
            &pack(
                EQUIRECT_VS,
                "objectStruct",
                &[("nodeUniform0", &m4(Matrix4::IDENTITY))],
            )?,
        );
        let group = bind(
            r,
            material.pipeline.get_bind_group_layout(1),
            &[
                (0, wgpu::BindingResource::Sampler(sampler)),
                (1, wgpu::BindingResource::TextureView(equirect)),
                (2, object.as_entire_binding()),
            ],
        );
        Ok(Source { material, group })
    }
    /// The cubemap material over `cube` ( a cube view, sampled through `sampler` ).
    pub(super) fn cubemap_source(
        &self,
        r: &Renderer,
        cube: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> Result<Source> {
        let material = Material::new(r, "PMREM cubemap", (CUBEMAP_VS, CUBEMAP_FS), false)?;
        let object = uniform(
            r,
            "PMREM cubemap object",
            &pack(
                CUBEMAP_VS,
                "objectStruct",
                &[("nodeUniform1", &m4(Matrix4::IDENTITY))],
            )?,
        );
        let group = bind(
            r,
            material.pipeline.get_bind_group_layout(1),
            &[
                (0, wgpu::BindingResource::Sampler(sampler)),
                (1, wgpu::BindingResource::TextureView(cube)),
                (2, object.as_entire_binding()),
            ],
        );
        Ok(Source { material, group })
    }
    /// fromEquirectangular / fromCubemap: `source` into the source target,
    /// its mipmaps, then every level.
    pub(super) fn encode_from_texture(&self, encoder: &mut wgpu::CommandEncoder, source: &Source) {
        self.box_mesh
            .draw(encoder, &source.material, &source.group, &self.source_faces);
        self.source_mipmaps.encode(encoder);
        self.encode_levels(encoder);
    }
    /// Sets fromScene's blur radius ( radians; zero for none ).
    pub(super) fn set_blur(&mut self, r: &Renderer, sigma: f64) -> Result<()> {
        if sigma <= 0. {
            self.blur = None;
            return Ok(());
        }
        // The variance split between two passes, sigma clamped to the
        // sphere's maximum angular distance.
        let object = pack(
            BLUR_FS,
            "objectStruct",
            &[
                ("nodeUniform0", &m4(Matrix4::IDENTITY)),
                (
                    "nodeUniform1",
                    &[sigma.min(std::f64::consts::PI) / 2f64.sqrt()],
                ),
                ("nodeUniform4", &m3(Matrix4::IDENTITY)),
            ],
        )?;
        if let Some(blur) = &self.blur {
            r.queue.write_buffer(&blur.object, 0, &object);
            return Ok(());
        }
        let material = Material::new(r, "PMREM blur", (BLUR_VS, BLUR_FS), true)?;
        let object = uniform(r, "PMREM blur object", &object);
        let group = |view: &wgpu::TextureView| {
            bind(
                r,
                material.pipeline.get_bind_group_layout(1),
                &[
                    (0, object.as_entire_binding()),
                    (1, wgpu::BindingResource::Sampler(&self.trilinear)),
                    (2, wgpu::BindingResource::TextureView(view)),
                ],
            )
        };
        let groups = [group(&cube_view(&self.source, None)), group(&self.view)];
        self.blur = Some(Blur {
            material,
            object,
            groups,
        });
        Ok(())
    }
    /// fromScene after the caller's CubeCamera rendered the scene into the
    /// source's level 0 ( [`Self::source_faces`], cleared, with a depth
    /// buffer ): the blur when set, the mipmaps, then every level.
    pub(super) fn encode_from_scene(&self, encoder: &mut wgpu::CommandEncoder) {
        if let Some(blur) = &self.blur {
            let level0 = &self.levels[0].faces;
            self.box_mesh
                .draw(encoder, &blur.material, &blur.groups[0], level0);
            self.box_mesh
                .draw(encoder, &blur.material, &blur.groups[1], &self.source_faces);
        }
        self.source_mipmaps.encode(encoder);
        self.encode_levels(encoder);
    }
    /// _applyPMREM: each level of the PMREM from the source.
    fn encode_levels(&self, encoder: &mut wgpu::CommandEncoder) {
        for level in &self.levels {
            let material = if level.integration {
                &self.integration
            } else {
                &self.ggx
            };
            self.box_mesh
                .draw(encoder, material, &level.group, &level.faces);
        }
    }
}

/// An RGBE .hdr as HDRLoader loads it ( half-float RGBA, flipY, linear
/// filters, no mipmaps ): the texture's view and its width.
pub(super) async fn load_rgbe(r: &Renderer, url: &str) -> Result<(wgpu::TextureView, u32)> {
    let (width, height, texels) = super::trackball_sprites::parse_rgbe(&fetch(url).await?)?;
    // flipY: the last row uploads first.
    let row = width as usize * 4;
    let texels: Vec<u16> = texels.chunks(row).rev().flatten().copied().collect();
    let view = r
        .device
        .create_texture_with_data(
            &r.queue,
            &wgpu::TextureDescriptor {
                label: Some("PMREM equirect"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: HALF,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            bytemuck::cast_slice(&texels),
        )
        .create_view(&Default::default());
    Ok((view, width))
}

/// fromEquirectangular( equirect ) for an equirect `width` texels wide
/// ( linear filters, clamped ), submitted.
pub(super) fn from_equirect(
    r: &Renderer,
    equirect: &wgpu::TextureView,
    width: u32,
) -> Result<Pmrem> {
    let pmrem = Pmrem::new(r, width / 4)?;
    let linear = sampler(r, wgpu::FilterMode::Linear, wgpu::FilterMode::Nearest);
    let source = pmrem.equirect_source(r, equirect, &linear)?;
    let mut encoder = r.device.create_command_encoder(&Default::default());
    pmrem.encode_from_texture(&mut encoder, &source);
    r.queue.submit([encoder.finish()]);
    Ok(pmrem)
}

/// The PMREM of the Radiance HDR at `url` ( HDRLoader, then
/// `scene.environment`'s fromEquirectangular ).
pub(super) async fn equirect_pmrem(r: &Renderer, url: &str) -> Result<Pmrem> {
    let (equirect, width) = load_rgbe(r, url).await?;
    from_equirect(r, &equirect, width)
}
