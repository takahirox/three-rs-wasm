//! A program three compiles for this example, as the generated WGSL pair
//! ( tools/tiles3d ): its uniform block's layout read from the WGSL struct,
//! values set by their WebGL uniform names, its texture bindings and its
//! pipelines for the targets it renders into.
use crate::{Error, Result, renderer::Renderer};
use std::cell::RefCell;
use std::collections::HashMap;
use wgpu::util::DeviceExt;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Scalar {
    F32,
    I32,
    U32,
}

/// One member of the uniform block: offset, scalar kind, components per
/// column ( vectors and matrix columns ), columns, array elements.
#[derive(Clone, Debug)]
struct Member {
    offset: usize,
    scalar: Scalar,
    rows: usize,
    columns: usize,
    count: usize,
    /// Bytes between array elements.
    stride: usize,
}

fn type_layout(ty: &str) -> Result<(usize, usize, Scalar, usize, usize, usize, usize)> {
    // ( align, size, scalar, rows, columns, count, stride )
    let ty = ty.trim();
    if let Some(inner) = ty.strip_prefix("array<").and_then(|t| t.strip_suffix('>')) {
        let (element, count) = inner.rsplit_once(',').ok_or(Error::Invalid("array type"))?;
        let count: usize = count
            .trim()
            .parse()
            .map_err(|_| Error::Invalid("array count"))?;
        let (align, size, scalar, rows, columns, _, _) = type_layout(element)?;
        let stride = size.div_ceil(align.max(16)) * align.max(16);
        return Ok((
            align.max(16),
            stride * count,
            scalar,
            rows,
            columns,
            count,
            stride,
        ));
    }
    let scalar = |s: &str| match s {
        "f32" => Ok(Scalar::F32),
        "i32" => Ok(Scalar::I32),
        "u32" => Ok(Scalar::U32),
        _ => Err(Error::Invalid("uniform scalar")),
    };
    if let Some(rest) = ty.strip_prefix("vec") {
        let n: usize = rest[..1].parse().map_err(|_| Error::Invalid("vector"))?;
        let s = scalar(rest[2..rest.len() - 1].trim())?;
        let align = if n == 2 { 8 } else { 16 };
        return Ok((align, 4 * n, s, n, 1, 1, 0));
    }
    if let Some(rest) = ty.strip_prefix("mat") {
        let c: usize = rest[..1].parse().map_err(|_| Error::Invalid("matrix"))?;
        let r: usize = rest[2..3].parse().map_err(|_| Error::Invalid("matrix"))?;
        let column = if r == 2 { 8 } else { 16 };
        return Ok((column, column * c, Scalar::F32, r, c, 1, 0));
    }
    Ok((4, 4, scalar(ty)?, 1, 1, 1, 0))
}

/// The `struct Uniforms` of a generated module.
#[derive(Debug, Default)]
pub(crate) struct BlockLayout {
    members: HashMap<String, Member>,
    pub size: usize,
}

impl BlockLayout {
    fn parse(source: &str) -> Result<Self> {
        let start = source
            .find("struct Uniforms {")
            .ok_or(Error::Invalid("no uniform block"))?;
        let body = &source[start + 17..];
        let body = &body[..body
            .find("\n}")
            .ok_or(Error::Invalid("uniform block end"))?];
        let mut members = HashMap::new();
        let mut offset = 0usize;
        let mut max_align = 16;
        for line in body.lines() {
            let mut line = line.trim().trim_end_matches(',');
            if line.is_empty() {
                continue;
            }
            let (mut forced_align, mut forced_size) = (None, None);
            while let Some(rest) = line.strip_prefix('@') {
                let end = rest.find(')').ok_or(Error::Invalid("attribute"))?;
                let (attr, value) = rest[..end]
                    .split_once('(')
                    .ok_or(Error::Invalid("attribute"))?;
                let v: usize = value
                    .parse()
                    .map_err(|_| Error::Invalid("attribute value"))?;
                match attr {
                    "align" => forced_align = Some(v),
                    "size" => forced_size = Some(v),
                    _ => {}
                }
                line = rest[end + 1..].trim();
            }
            let (name, ty) = line
                .split_once(':')
                .ok_or(Error::Invalid("uniform member"))?;
            let (align, size, scalar, rows, columns, count, stride) = type_layout(ty)?;
            let align = forced_align.unwrap_or(align);
            max_align = max_align.max(align);
            offset = offset.div_ceil(align) * align;
            members.insert(
                name.trim().to_owned(),
                Member {
                    offset,
                    scalar,
                    rows,
                    columns,
                    count,
                    stride,
                },
            );
            offset += forced_size.unwrap_or(size);
        }
        Ok(BlockLayout {
            members,
            size: offset.div_ceil(max_align) * max_align,
        })
    }
}

/// Uniform values in the block's layout.
pub(crate) struct Block {
    layout: std::rc::Rc<BlockLayout>,
    pub data: Vec<u8>,
    /// The values set, by name, for the comparison hook.
    pub log: Option<Vec<(String, Vec<f64>)>>,
}

impl Block {
    /// Sets a uniform by its WebGL name ( `a`, `a[1]`, `s.b.c[0].d` ) as three
    /// uploads it: matrices column-major, arrays from the named element on.
    /// Returns false when the program has no such uniform.
    pub fn set(&mut self, name: &str, values: &[f64]) -> bool {
        if let Some(log) = &mut self.log {
            log.push((name.to_owned(), values.to_vec()));
        }
        let (base, first) = match name.strip_suffix(']').and_then(|n| n.rsplit_once('[')) {
            Some((b, k)) if !b.contains('.') => (b, k.parse().unwrap_or(0)),
            _ => (name, 0),
        };
        let member = if base.contains('.') {
            let leaf: String = name
                .chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect();
            let mut leaf = leaf.trim_end_matches('_').to_owned();
            while leaf.contains("__") {
                leaf = leaf.replace("__", "_");
            }
            self.layout.members.get(&format!("pt_s_{leaf}")).cloned()
        } else {
            self.layout
                .members
                .get(base)
                .or_else(|| self.layout.members.get(&format!("pt_u_{base}")))
                .cloned()
        };
        let Some(m) = member else { return false };
        let padded = !base.contains('.') && !self.layout.members.contains_key(base);
        let per_column = m.rows;
        let column_bytes = if m.columns > 1 {
            if m.rows == 2 { 8 } else { 16 }
        } else {
            0
        };
        // Components per array element as three passes them.
        let element = if padded {
            values.len().max(1) / (m.count - first).max(1)
        } else {
            m.rows * m.columns
        };
        let element = element.max(1);
        for (k, chunk) in values.chunks(element).enumerate() {
            let index = first + k;
            if index >= m.count {
                break;
            }
            let base_offset = m.offset + index * m.stride;
            for (j, v) in chunk.iter().enumerate() {
                let (column, row) = if m.columns > 1 {
                    (j / per_column, j % per_column)
                } else {
                    (0, j)
                };
                let at = base_offset + column * column_bytes + row * 4;
                let bytes = match m.scalar {
                    Scalar::F32 => (*v as f32).to_le_bytes(),
                    Scalar::I32 => (*v as i32).to_le_bytes(),
                    Scalar::U32 => (*v as u32).to_le_bytes(),
                };
                if at + 4 <= self.data.len() {
                    self.data[at..at + 4].copy_from_slice(&bytes);
                }
            }
        }
        true
    }
    pub fn set1(&mut self, name: &str, v: f64) -> bool {
        self.set(name, &[v])
    }
}

/// A texture binding: its GLSL name and the view dimension and sample type.
struct TextureBinding {
    name: String,
    texture: u32,
    sampler: u32,
}

/// What a texture uniform reads: a view and the sampler state three gives it.
pub(crate) struct Bound<'a> {
    pub view: &'a wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Vertices {
    /// postprocessing's and three's fullscreen triangle: position and uv.
    Fullscreen,
    /// Position, normal and uv buffers.
    Mesh,
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct PipelineKey {
    pub formats: Vec<wgpu::TextureFormat>,
    pub blend: Vec<Option<wgpu::BlendState>>,
    pub depth: Option<(wgpu::TextureFormat, wgpu::CompareFunction, bool)>,
    /// Back-face culling of counter-clockwise WebGL front faces drawn with
    /// clip y flipped ( so clockwise in WebGPU ).
    pub cull_back: bool,
}

pub(crate) struct Program {
    pub name: &'static str,
    vs: wgpu::ShaderModule,
    fs: wgpu::ShaderModule,
    pub layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    textures: Vec<TextureBinding>,
    block: std::rc::Rc<BlockLayout>,
    vertices: Vertices,
    pipelines: RefCell<Vec<(PipelineKey, wgpu::RenderPipeline)>>,
}

fn texture_entries(source: &str) -> Vec<(u32, String, String)> {
    // ( binding, name, type ) of every `@group(0) @binding(n) var name: type;`.
    let mut out = vec![];
    let mut lines = source.lines();
    while let Some(line) = lines.next() {
        let Some(rest) = line.trim().strip_prefix("@group(0) @binding(") else {
            continue;
        };
        let index: u32 = rest.trim_end_matches([')', ' ']).parse().unwrap_or(0);
        let Some(decl) = lines.next() else { break };
        let decl = decl.trim().trim_end_matches(';');
        let Some((name, ty)) = decl.split_once(':') else {
            continue;
        };
        let name = name
            .trim()
            .trim_start_matches("var<uniform>")
            .trim_start_matches("var")
            .trim();
        out.push((index, name.to_owned(), ty.trim().to_owned()));
    }
    out
}

impl Program {
    /// `unfilterable` names the textures bound to unfilterable formats
    /// ( depth, float32 without float32-filterable ), read with nearest samplers.
    pub fn new(
        r: &Renderer,
        name: &'static str,
        vs: &str,
        fs: &str,
        vertices: Vertices,
        unfilterable: &[&str],
    ) -> Result<Self> {
        let block = std::rc::Rc::new(BlockLayout::parse(fs)?);
        let mut entries = vec![wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }];
        let mut textures = vec![];
        let declared = texture_entries(fs);
        for (index, wgsl, ty) in &declared {
            if *index == 0 {
                continue;
            }
            // naga's names: the GLSL name, `_t` / `_s`, maybe a `_N` suffix.
            let mut base = wgsl.clone();
            while let Some((head, tail)) = base.rsplit_once('_') {
                if !tail.is_empty() && tail.chars().all(|c| c.is_ascii_digit()) {
                    base = head.to_owned();
                } else {
                    break;
                }
            }
            if let Some(b) = base.strip_suffix("_t") {
                let unfilter = unfilterable.contains(&b);
                let dimension = if ty.starts_with("texture_3d") {
                    wgpu::TextureViewDimension::D3
                } else if ty.starts_with("texture_2d_array") {
                    wgpu::TextureViewDimension::D2Array
                } else if ty.starts_with("texture_cube") {
                    wgpu::TextureViewDimension::Cube
                } else {
                    wgpu::TextureViewDimension::D2
                };
                let sample_type = if ty.contains("<u32>") {
                    wgpu::TextureSampleType::Uint
                } else if ty.contains("<i32>") {
                    wgpu::TextureSampleType::Sint
                } else {
                    wgpu::TextureSampleType::Float {
                        filterable: !unfilter,
                    }
                };
                entries.push(wgpu::BindGroupLayoutEntry {
                    binding: *index,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type,
                        view_dimension: dimension,
                        multisampled: false,
                    },
                    count: None,
                });
                textures.push(TextureBinding {
                    name: b.to_owned(),
                    texture: *index,
                    sampler: index + 1,
                });
            } else if let Some(b) = base.strip_suffix("_s") {
                let unfilter = unfilterable.contains(&b) || ty.contains("<u32>");
                entries.push(wgpu::BindGroupLayoutEntry {
                    binding: *index,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(if unfilter {
                        wgpu::SamplerBindingType::NonFiltering
                    } else {
                        wgpu::SamplerBindingType::Filtering
                    }),
                    count: None,
                });
            }
        }
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(name),
                entries: &entries,
            });
        let pipeline_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(name),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
        let module = |label: &str, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        Ok(Program {
            name,
            vs: module(name, vs),
            fs: module(name, fs),
            layout,
            pipeline_layout,
            textures,
            block,
            vertices,
            pipelines: RefCell::new(vec![]),
        })
    }

    pub fn block(&self) -> Block {
        Block {
            layout: self.block.clone(),
            data: vec![0; self.block.size.max(16)],
            log: None,
        }
    }

    pub fn uniform_buffer(&self, r: &Renderer, block: &Block) -> wgpu::Buffer {
        r.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(self.name),
                contents: &block.data,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }

    /// The bind group over the block's buffer and the named textures.
    pub fn bind<'a>(
        &self,
        r: &Renderer,
        buffer: &wgpu::Buffer,
        resources: &dyn Fn(&str) -> Option<Bound<'a>>,
    ) -> Result<wgpu::BindGroup> {
        let mut bound = vec![];
        for t in &self.textures {
            let b = resources(&t.name)
                .ok_or_else(|| Error::Asset(format!("{}: no texture for {}", self.name, t.name)))?;
            bound.push((t, b));
        }
        let mut entries = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }];
        for (t, b) in &bound {
            entries.push(wgpu::BindGroupEntry {
                binding: t.texture,
                resource: wgpu::BindingResource::TextureView(b.view),
            });
            entries.push(wgpu::BindGroupEntry {
                binding: t.sampler,
                resource: wgpu::BindingResource::Sampler(&b.sampler),
            });
        }
        Ok(r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(self.name),
            layout: &self.layout,
            entries: &entries,
        }))
    }

    pub fn pipeline(&self, r: &Renderer, key: &PipelineKey) -> wgpu::RenderPipeline {
        if let Some((_, p)) = self.pipelines.borrow().iter().find(|(k, _)| k == key) {
            return p.clone();
        }
        // The triangle has no normals: WebGL reads the attribute's default 0.
        let fullscreen = [wgpu::VertexBufferLayout {
            array_stride: 32,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2],
        }];
        let mesh = [
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![1 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: 8,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![2 => Float32x2],
            },
        ];
        let targets: Vec<_> = key
            .formats
            .iter()
            .enumerate()
            .map(|(k, f)| {
                Some(wgpu::ColorTargetState {
                    format: *f,
                    blend: key.blend.get(k).copied().flatten(),
                    write_mask: wgpu::ColorWrites::ALL,
                })
            })
            .collect();
        let pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(self.name),
                layout: Some(&self.pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &self.vs,
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    buffers: if self.vertices == Vertices::Mesh {
                        &mesh
                    } else {
                        &fullscreen
                    },
                },
                fragment: Some(wgpu::FragmentState {
                    module: &self.fs,
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    targets: &targets,
                }),
                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: key.cull_back.then_some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: key
                    .depth
                    .map(|(format, compare, write)| wgpu::DepthStencilState {
                        format,
                        depth_write_enabled: write,
                        depth_compare: compare,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        self.pipelines
            .borrow_mut()
            .push((key.clone(), pipeline.clone()));
        pipeline
    }
}

/// The fullscreen triangle's vertices ( position, a zero normal, uv ).
pub(crate) fn fullscreen_buffer(r: &Renderer) -> wgpu::Buffer {
    let v: [f32; 24] = [
        -1., -1., 0., 0., 0., 0., 0., 0., 3., -1., 0., 0., 0., 0., 2., 0., -1., 3., 0., 0., 0., 0.,
        0., 2.,
    ];
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("fullscreen triangle"),
            contents: bytemuck::cast_slice(&v),
            usage: wgpu::BufferUsages::VERTEX,
        })
}

/// Sampler states three gives textures, created once each.
#[derive(Default)]
pub(crate) struct Samplers {
    cache: RefCell<Vec<(SamplerKey, wgpu::Sampler)>>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct SamplerKey {
    pub linear: bool,
    pub mipmaps: bool,
    pub repeat: bool,
}

pub(crate) const LINEAR: SamplerKey = SamplerKey {
    linear: true,
    mipmaps: false,
    repeat: false,
};
pub(crate) const NEAREST: SamplerKey = SamplerKey {
    linear: false,
    mipmaps: false,
    repeat: false,
};
pub(crate) const LINEAR_REPEAT: SamplerKey = SamplerKey {
    linear: true,
    mipmaps: false,
    repeat: true,
};
pub(crate) const NEAREST_REPEAT: SamplerKey = SamplerKey {
    linear: false,
    mipmaps: false,
    repeat: true,
};
pub(crate) const MIPMAP_REPEAT: SamplerKey = SamplerKey {
    linear: true,
    mipmaps: true,
    repeat: true,
};

impl Samplers {
    pub fn get(&self, r: &Renderer, key: SamplerKey) -> wgpu::Sampler {
        if let Some((_, s)) = self.cache.borrow().iter().find(|(k, _)| *k == key) {
            return s.clone();
        }
        let filter = if key.linear {
            wgpu::FilterMode::Linear
        } else {
            wgpu::FilterMode::Nearest
        };
        let wrap = if key.repeat {
            wgpu::AddressMode::Repeat
        } else {
            wgpu::AddressMode::ClampToEdge
        };
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("tiles3d sampler"),
            address_mode_u: wrap,
            address_mode_v: wrap,
            address_mode_w: wrap,
            mag_filter: filter,
            min_filter: filter,
            mipmap_filter: if key.mipmaps {
                wgpu::FilterMode::Linear
            } else {
                wgpu::FilterMode::Nearest
            },
            ..Default::default()
        });
        self.cache.borrow_mut().push((key, sampler.clone()));
        sampler
    }
}

/// A color attachment: a view ( a 3D texture's slice ), cleared or loaded.
pub(crate) struct Attachment<'a> {
    pub view: &'a wgpu::TextureView,
    pub slice: Option<u32>,
    pub clear: Option<wgpu::Color>,
}

impl Program {
    /// One fullscreen draw ( FullScreenQuad / postprocessing's triangle ).
    #[allow(clippy::too_many_arguments)]
    pub fn draw_fullscreen(
        &self,
        r: &Renderer,
        encoder: &mut wgpu::CommandEncoder,
        key: &PipelineKey,
        group: &wgpu::BindGroup,
        quad: &wgpu::Buffer,
        targets: &[Attachment],
        viewport: Option<[f32; 4]>,
    ) {
        let pipeline = self.pipeline(r, key);
        let attachments: Vec<_> = targets
            .iter()
            .map(|a| {
                Some(wgpu::RenderPassColorAttachment {
                    view: a.view,
                    depth_slice: a.slice,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: a.clear.map_or(wgpu::LoadOp::Load, wgpu::LoadOp::Clear),
                        store: wgpu::StoreOp::Store,
                    },
                })
            })
            .collect();
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(self.name),
            color_attachments: &attachments,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        if let Some([x, y, w, h]) = viewport {
            pass.set_viewport(x, y, w, h, 0., 1.);
        }
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, group, &[]);
        pass.set_vertex_buffer(0, quad.slice(..));
        pass.draw(0..3, 0..1);
    }
}

/// A texture rendered into and sampled ( and copied ), 2D or 3D.
pub(crate) fn target(
    r: &Renderer,
    label: &str,
    size: (u32, u32, u32),
    format: wgpu::TextureFormat,
    three_d: bool,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: size.2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: if three_d {
            wgpu::TextureDimension::D3
        } else {
            wgpu::TextureDimension::D2
        },
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}
