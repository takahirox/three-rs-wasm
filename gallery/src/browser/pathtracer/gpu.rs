//! The GPU side: the data textures, the generated WGSL pipelines and their
//! bind groups. Every pass that renders into a render target the original
//! later samples flips clip-space y, so that texture memory rows match
//! WebGL's ( row 0 is gl_FragCoord.y 0.5 and uv.y 0 in both ).
use crate::{Error, Result, renderer::Renderer};
use wgpu::util::DeviceExt;

/// One `@group( 0 ) @binding( n ) var name : type;` of a generated module.
pub(crate) struct Binding {
    pub index: u32,
    /// The GLSL name: the WGSL name without naga's `_N` suffix and `_t` / `_s`.
    pub base: String,
    pub texture: bool,
    pub ty: String,
}

pub(crate) fn bindings(source: &str) -> Vec<Binding> {
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
        let mut base = name.to_string();
        while let Some((head, tail)) = base.rsplit_once('_') {
            if tail.chars().all(|c| c.is_ascii_digit()) && !tail.is_empty() {
                base = head.to_string();
            } else {
                break;
            }
        }
        let texture = base.ends_with("_t");
        if let Some(b) = base.strip_suffix("_t").or_else(|| base.strip_suffix("_s")) {
            base = b.to_string();
        }
        out.push(Binding {
            index,
            base,
            texture,
            ty: ty.trim().to_string(),
        });
    }
    out
}

/// A texture view and how its sampler filters.
pub(crate) struct Resource<'a> {
    pub view: &'a wgpu::TextureView,
    pub filter: bool,
    /// RepeatWrapping on u / v.
    pub repeat: (bool, bool),
    /// LinearMipmapLinearFilter over a mip chain.
    pub mipmaps: bool,
}

/// The explicit layout and bind group of a generated module: every declared
/// binding, the uniform block at 0 and each texture with its own sampler.
pub(crate) fn bind_group<'a>(
    r: &Renderer,
    source: &str,
    uniforms: Option<&wgpu::Buffer>,
    resources: &dyn Fn(&str) -> Option<Resource<'a>>,
) -> Result<(wgpu::BindGroupLayout, wgpu::BindGroup)> {
    let (layout, group) = bind_group_in(r, source, uniforms, resources, None)?;
    Ok((layout.ok_or(Error::Invalid("bind group layout"))?, group))
}

/// A bind group over an existing layout of the same module.
pub(crate) fn bind_group_with_layout<'a>(
    r: &Renderer,
    source: &str,
    uniforms: Option<&wgpu::Buffer>,
    resources: &dyn Fn(&str) -> Option<Resource<'a>>,
    layout: &wgpu::BindGroupLayout,
) -> Result<((), wgpu::BindGroup)> {
    Ok((
        (),
        bind_group_in(r, source, uniforms, resources, Some(layout))?.1,
    ))
}

fn bind_group_in<'a>(
    r: &Renderer,
    source: &str,
    uniforms: Option<&wgpu::Buffer>,
    resources: &dyn Fn(&str) -> Option<Resource<'a>>,
    existing: Option<&wgpu::BindGroupLayout>,
) -> Result<(Option<wgpu::BindGroupLayout>, wgpu::BindGroup)> {
    let declared = bindings(source);
    let mut entries = vec![];
    let mut resolved: Vec<(u32, Resource<'a>)> = vec![];
    let mut samplers = vec![];
    for b in &declared {
        if b.ty.starts_with("Uniforms") || b.base == "global" {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: b.index,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });
            continue;
        }
        let resource = resources(&b.base)
            .ok_or_else(|| Error::Asset(format!("pathtracer: no resource for {}", b.base)))?;
        if b.texture {
            let (sample_type, dimension) = if b.ty.starts_with("texture_2d_array") {
                (
                    sample(&b.ty, resource.filter),
                    wgpu::TextureViewDimension::D2Array,
                )
            } else {
                (
                    sample(&b.ty, resource.filter),
                    wgpu::TextureViewDimension::D2,
                )
            };
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: b.index,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type,
                    view_dimension: dimension,
                    multisampled: false,
                },
                count: None,
            });
        } else {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: b.index,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(if resource.filter {
                    wgpu::SamplerBindingType::Filtering
                } else {
                    wgpu::SamplerBindingType::NonFiltering
                }),
                count: None,
            });
        }
        resolved.push((b.index, resource));
    }
    let created = existing.is_none().then(|| {
        r.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("pathtracer bindings"),
                entries: &entries,
            })
    });
    let layout = existing
        .or(created.as_ref())
        .ok_or(Error::Invalid("bind group layout"))?;
    let start = 0;
    for (_, res) in &resolved {
        let filter = if res.filter {
            wgpu::FilterMode::Linear
        } else {
            wgpu::FilterMode::Nearest
        };
        let wrap = |repeat: bool| {
            if repeat {
                wgpu::AddressMode::Repeat
            } else {
                wgpu::AddressMode::ClampToEdge
            }
        };
        samplers.push(r.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("pathtracer sampler"),
            address_mode_u: wrap(res.repeat.0),
            address_mode_v: wrap(res.repeat.1),
            mag_filter: filter,
            min_filter: filter,
            mipmap_filter: if res.mipmaps {
                wgpu::FilterMode::Linear
            } else {
                wgpu::FilterMode::Nearest
            },
            ..Default::default()
        }));
    }
    let mut group_entries = vec![];
    for b in &declared {
        if b.ty.starts_with("Uniforms") || b.base == "global" {
            let buffer = uniforms.ok_or(Error::Invalid("pathtracer uniforms"))?;
            group_entries.push(wgpu::BindGroupEntry {
                binding: b.index,
                resource: buffer.as_entire_binding(),
            });
        }
    }
    for (k, (index, res)) in resolved.iter().enumerate() {
        let b = declared
            .iter()
            .find(|b| b.index == *index)
            .ok_or(Error::Invalid("binding"))?;
        group_entries.push(wgpu::BindGroupEntry {
            binding: *index,
            resource: if b.texture {
                wgpu::BindingResource::TextureView(res.view)
            } else {
                wgpu::BindingResource::Sampler(&samplers[start + k])
            },
        });
    }
    let group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("pathtracer bind group"),
        layout,
        entries: &group_entries,
    });
    Ok((created, group))
}

fn sample(ty: &str, filter: bool) -> wgpu::TextureSampleType {
    if ty.contains("<u32>") {
        wgpu::TextureSampleType::Uint
    } else if ty.contains("<i32>") {
        wgpu::TextureSampleType::Sint
    } else {
        wgpu::TextureSampleType::Float { filterable: filter }
    }
}

/// A texture with its data, rows as given ( memory row 0 first ).
pub(crate) fn texture(
    r: &Renderer,
    label: &str,
    size: (u32, u32, u32),
    format: wgpu::TextureFormat,
    data: &[u8],
) -> wgpu::Texture {
    r.device.create_texture_with_data(
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
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        data,
    )
}

/// A render target that is also sampled and read back.
pub(crate) fn target(
    r: &Renderer,
    label: &str,
    size: (u32, u32),
    format: wgpu::TextureFormat,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

/// A fullscreen-triangle vertex stage ( FullScreenQuad's big triangle and
/// its uvs ), with clip y flipped for render targets.
pub(crate) fn fullscreen_vs(fragment: &str, flip: bool) -> String {
    // The fragment stage's location for vUv.
    let location = fragment
        .find(") vUv:")
        .and_then(|end| {
            fragment[..end]
                .rfind("@location(")
                .map(|start| fragment[start + 10..end].to_string())
        })
        .unwrap_or_else(|| "0".into());
    format!(
        "struct VertexOutput {{ @location({location}) vUv: vec2<f32>, @builtin(position) position: vec4<f32> }}
@vertex fn main(@builtin(vertex_index) i: u32) -> VertexOutput {{
    var p = array<vec2<f32>, 3>(vec2<f32>(-1.0, 3.0), vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0));
    var uv = array<vec2<f32>, 3>(vec2<f32>(0.0, 2.0), vec2<f32>(0.0, 0.0), vec2<f32>(2.0, 0.0));
    return VertexOutput(uv[i], vec4<f32>(p[i].x, {}p[i].y, 0.0, 1.0));
}}",
        if flip { "-" } else { "" }
    )
}

pub(crate) fn module(r: &Renderer, label: &str, source: &str) -> wgpu::ShaderModule {
    r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}

/// A pipeline over one bind group layout with an optional blend state.
#[allow(clippy::too_many_arguments)]
pub(crate) fn pipeline(
    r: &Renderer,
    label: &str,
    vs: &wgpu::ShaderModule,
    fs: &wgpu::ShaderModule,
    layout: &wgpu::BindGroupLayout,
    buffers: &[wgpu::VertexBufferLayout],
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
) -> wgpu::RenderPipeline {
    let pipeline_layout = r
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts: &[layout],
            push_constant_ranges: &[],
        });
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: vs,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: fs,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}

/// three's NormalBlending: ( SrcAlpha, OneMinusSrcAlpha ) for color and
/// ( One, OneMinusSrcAlpha ) for alpha, or with premultiplied alpha
/// ( One, OneMinusSrcAlpha ) for both.
pub(crate) fn normal_blending(premultiplied: bool) -> wgpu::BlendState {
    let c = |src| wgpu::BlendComponent {
        src_factor: src,
        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        operation: wgpu::BlendOperation::Add,
    };
    wgpu::BlendState {
        color: c(if premultiplied {
            wgpu::BlendFactor::One
        } else {
            wgpu::BlendFactor::SrcAlpha
        }),
        alpha: c(wgpu::BlendFactor::One),
    }
}
