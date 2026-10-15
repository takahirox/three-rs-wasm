//! Bind groups for generated WGSL by declaration name: each `@binding( b )
//! @group( g ) var … name` of a module ( or a vertex and fragment pair ) is
//! resolved through a callback, so a port names its resources by the node
//! names three.js gives them ( `NodeBuffer_1012`, `nodeUniform10_sampler`,
//! `object`, `render` ) instead of by slot.
use crate::{Error, Result, renderer::Renderer};

/// ( group, binding, name ) for each resource the sources declare, without
/// duplicates.
pub(super) fn declarations(sources: &[&str]) -> Vec<(u32, u32, String)> {
    let mut out: Vec<(u32, u32, String)> = vec![];
    for source in sources {
        let mut rest = *source;
        while let Some(at) = rest.find("@binding(") {
            rest = &rest[at + "@binding(".len()..];
            let number = |s: &str| -> Option<(u32, usize)> {
                let end = s.find(')')?;
                Some((s[..end].trim().parse().ok()?, end + 1))
            };
            let Some((binding, used)) = number(rest) else {
                continue;
            };
            let after = &rest[used..];
            let Some(g) = after.find("@group(") else {
                continue;
            };
            let Some((group, used)) = number(&after[g + "@group(".len()..]) else {
                continue;
            };
            let tail = &after[g + "@group(".len() + used..];
            let Some(v) = tail.find("var") else {
                continue;
            };
            let mut decl = &tail[v + 3..];
            if decl.starts_with('<') {
                decl = &decl[decl.find('>').map_or(0, |e| e + 1)..];
            }
            let name: String = decl
                .trim_start()
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if !out.iter().any(|(gg, bb, _)| *gg == group && *bb == binding) {
                out.push((group, binding, name));
            }
        }
    }
    out
}
/// Whether a declared name is referenced beyond its declaration: an
/// automatic layout leaves out what the entry points never touch ( the
/// samplers three.js declares beside textures it only loads ).
fn used(sources: &[&str], name: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    sources.iter().any(|source| {
        source.lines().any(|line| {
            !line.contains("@binding(")
                && line.match_indices(name).any(|(at, _)| {
                    !line[..at].ends_with(word) && !line[at + name.len()..].starts_with(word)
                })
        })
    })
}
/// One bind group per declared group of a pipeline, resolving each name.
pub(super) fn groups<'a>(
    r: &Renderer,
    layout: impl Fn(u32) -> wgpu::BindGroupLayout,
    sources: &[&str],
    resolve: impl Fn(&str) -> Option<wgpu::BindingResource<'a>>,
) -> Result<Vec<wgpu::BindGroup>> {
    let declared = declarations(sources);
    let count = declared.iter().map(|(g, _, _)| g + 1).max().unwrap_or(0);
    (0..count)
        .map(|group| {
            let mut entries = vec![];
            for (g, binding, name) in &declared {
                if *g != group || !used(sources, name) {
                    continue;
                }
                let resource = resolve(name).ok_or_else(|| {
                    Error::Asset(format!("unresolved WGSL binding {name} ({g}, {binding})"))
                })?;
                entries.push(wgpu::BindGroupEntry {
                    binding: *binding,
                    resource,
                });
            }
            Ok(r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout(group),
                entries: &entries,
            }))
        })
        .collect()
}
/// A stage's uniform buffers: its object and render structs, if declared.
pub(super) struct Uniforms {
    pub(super) sources: Vec<&'static str>,
    pub(super) object: Option<wgpu::Buffer>,
    pub(super) render: Option<wgpu::Buffer>,
}
impl Uniforms {
    pub(super) fn new(r: &Renderer, label: &str, sources: &[&'static str]) -> Result<Self> {
        let make = |name: &str| -> Result<Option<wgpu::Buffer>> {
            sources
                .iter()
                .find(|s| s.contains(&format!("struct {name} {{")))
                .map(|s| super::retro::uniform(r, label, s, name))
                .transpose()
        };
        Ok(Self {
            sources: sources.to_vec(),
            object: make("objectStruct")?,
            render: make("renderStruct")?,
        })
    }
    /// Packs `values` into the named struct ( objectStruct or renderStruct ).
    pub(super) fn write(&self, r: &Renderer, name: &str, values: &[(&str, &[f64])]) -> Result<()> {
        let buffer = if name == "objectStruct" {
            &self.object
        } else {
            &self.render
        };
        let (Some(buffer), Some(source)) = (
            buffer,
            self.sources
                .iter()
                .find(|s| s.contains(&format!("struct {name} {{"))),
        ) else {
            return Ok(());
        };
        r.queue.write_buffer(
            buffer,
            0,
            &super::lights_projector::pack(source, name, values)?,
        );
        Ok(())
    }
    /// The bind groups of `pipeline` with these uniforms and the named
    /// resources.
    pub(super) fn bind<'a>(
        &'a self,
        r: &Renderer,
        layout: impl Fn(u32) -> wgpu::BindGroupLayout,
        resources: &[(&str, wgpu::BindingResource<'a>)],
    ) -> Result<Vec<wgpu::BindGroup>> {
        groups(r, layout, &self.sources, |name| match name {
            "object" => self.object.as_ref().map(|b| b.as_entire_binding()),
            "render" => self.render.as_ref().map(|b| b.as_entire_binding()),
            _ => resources
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, res)| res.clone()),
        })
    }
}
/// A render pipeline's fixed state, as the generated page sets it.
pub(super) struct Spec<'a> {
    pub(super) label: &'a str,
    pub(super) shaders: (&'a str, &'a str),
    pub(super) buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub(super) format: wgpu::TextureFormat,
    /// Normal alpha blending.
    pub(super) blend: bool,
    /// Format, compare, write and ( constant, slope ) bias.
    pub(super) depth: Option<(wgpu::TextureFormat, wgpu::CompareFunction, bool, (i32, f32))>,
    pub(super) cw: bool,
    pub(super) cull: Option<wgpu::Face>,
    pub(super) samples: u32,
}
impl Spec<'_> {
    pub(super) fn build(&self, r: &Renderer) -> wgpu::RenderPipeline {
        let module = |source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(self.label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let blend = wgpu::BlendState {
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
        r.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(self.label),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &module(self.shaders.0),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    buffers: self.buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module(self.shaders.1),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.format,
                        blend: self.blend.then_some(blend),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    front_face: if self.cw {
                        wgpu::FrontFace::Cw
                    } else {
                        wgpu::FrontFace::Ccw
                    },
                    cull_mode: self.cull,
                    ..Default::default()
                },
                depth_stencil: self
                    .depth
                    .map(
                        |(format, compare, write, (constant, slope))| wgpu::DepthStencilState {
                            format,
                            depth_write_enabled: write,
                            depth_compare: compare,
                            stencil: Default::default(),
                            bias: wgpu::DepthBiasState {
                                constant,
                                slope_scale: slope,
                                clamp: 0.,
                            },
                        },
                    ),
                multisample: wgpu::MultisampleState {
                    count: self.samples,
                    ..Default::default()
                },
                multiview: None,
                cache: None,
            })
    }
}
/// A vertex buffer layout of one attribute.
pub(super) const fn attribute(
    attributes: &[wgpu::VertexAttribute],
    stride: u64,
) -> wgpu::VertexBufferLayout<'_> {
    wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes,
    }
}
