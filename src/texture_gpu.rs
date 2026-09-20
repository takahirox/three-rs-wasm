use crate::{Error, Result, material::*};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
#[derive(Clone)]
pub struct GpuTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}
impl GpuTexture {
    /// Upload +X,-X,+Y,-Y,+Z,-Z cube faces and generate their mip chains on the GPU.
    pub fn from_cube_rgba(
        renderer: &crate::renderer::Renderer,
        faces: &[Texture; 6],
    ) -> Result<Self> {
        Self::cube_rgba(renderer, std::slice::from_ref(faces), true)
    }
    /// Upload an explicit cube mip chain, with six faces per level. No filtering
    /// or regeneration is performed on caller-supplied levels.
    pub fn from_cube_mipmaps(
        renderer: &crate::renderer::Renderer,
        levels: &[[Texture; 6]],
    ) -> Result<Self> {
        Self::cube_rgba(renderer, levels, false)
    }
    fn cube_rgba(
        renderer: &crate::renderer::Renderer,
        levels: &[[Texture; 6]],
        generate: bool,
    ) -> Result<Self> {
        let faces = levels
            .first()
            .ok_or(Error::Invalid("empty cube mip chain"))?;
        let first = &faces[0];
        let size = first.width;
        if size == 0
            || size > renderer.device.limits().max_texture_dimension_2d
            || faces.iter().any(|f| {
                f.width != size
                    || f.height != size
                    || f.srgb != first.srgb
                    || f.rgba.len() != size as usize * size as usize * 4
            })
        {
            return Err(Error::Invalid("cube texture faces"));
        }
        let mip_count = if generate {
            size.ilog2() + 1
        } else {
            levels.len() as u32
        };
        if mip_count > size.ilog2() + 1
            || levels.iter().enumerate().any(|(level, faces)| {
                let side = (size >> level).max(1);
                faces.iter().any(|f| {
                    f.width != side
                        || f.height != side
                        || f.srgb != first.srgb
                        || f.rgba.len() != side as usize * side as usize * 4
                })
            })
        {
            return Err(Error::Invalid("cube mip dimensions"));
        }
        let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("cube texture"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 6,
            },
            mip_level_count: mip_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: if first.srgb {
                wgpu::TextureFormat::Rgba8UnormSrgb
            } else {
                wgpu::TextureFormat::Rgba8Unorm
            },
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        for (level, faces) in levels.iter().enumerate() {
            let size = (size >> level).max(1);
            for (layer, image) in faces.iter().enumerate() {
                renderer.queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: level as u32,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: layer as u32,
                        },
                        aspect: wgpu::TextureAspect::All,
                    },
                    &image.rgba,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(size * 4),
                        rows_per_image: Some(size),
                    },
                    wgpu::Extent3d {
                        width: size,
                        height: size,
                        depth_or_array_layers: 1,
                    },
                );
            }
        }
        if generate {
            crate::mipmap::MipGenerator::new(&renderer.device, &texture)?
                .update(&renderer.device, &renderer.queue);
        }
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Ok(Self {
            texture,
            view,
            sampler,
        })
    }

    /// Transcode every layer/mip to a supported GPU block format, retaining the
    /// compressed representation. No RGBA fallback is used.
    pub fn from_basis_array(
        renderer: &crate::renderer::Renderer,
        bytes: &[u8],
        srgb: bool,
    ) -> Result<Self> {
        let t =
            basisu::Transcoder::new(bytes).map_err(|e| Error::Asset(format!("Basis: {e:?}")))?;
        if t.face_count() != 1
            || t.layer_count() == 0
            || t.is_video()
            || t.layer_count() > renderer.device.limits().max_texture_array_layers
        {
            return Err(Error::Invalid("compressed texture array dimensions"));
        }
        let mut image = Texture::from_rgba(1, 1, vec![0; 4], srgb)?;
        (image.width, image.height) = t.base_dimensions();
        image.mipmap_filter = Some(Filter::Linear);
        compressed_texture_layers(
            &renderer.device,
            &renderer.queue,
            &image,
            bytes,
            t.layer_count(),
            true,
        )
    }
}
#[derive(Default)]
pub(crate) struct TextureCache {
    entries: HashMap<usize, (Weak<Texture>, GpuTexture)>,
    pub uploads: u64,
    mip_pipelines: HashMap<wgpu::TextureFormat, wgpu::RenderPipeline>,
}
impl TextureCache {
    pub fn prune(&mut self) {
        self.entries
            .retain(|_, (owner, _)| owner.strong_count() > 0);
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn get(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &Arc<Texture>,
    ) -> Result<GpuTexture> {
        let key = Arc::as_ptr(image) as usize;
        if let Some((owner, gpu)) = self.entries.get(&key)
            && owner.strong_count() > 0
        {
            return Ok(gpu.clone());
        }
        if let Some(bytes) = &image.basis {
            let gpu = compressed_texture(device, queue, image, bytes)?;
            self.entries
                .insert(key, (Arc::downgrade(image), gpu.clone()));
            self.uploads += 1;
            return Ok(gpu);
        }
        let valid_data = image.rgba.len() == image.width as usize * image.height as usize * 4;
        #[cfg(target_arch = "wasm32")]
        let valid_data = valid_data || image.bitmap.is_some();
        if image.width == 0
            || image.height == 0
            || image.width > device.limits().max_texture_dimension_2d
            || image.height > device.limits().max_texture_dimension_2d
            || !valid_data
        {
            return Err(Error::Invalid("texture dimensions/data"));
        }
        let levels = if image.mipmap_filter.is_some() {
            image.width.max(image.height).ilog2() + 1
        } else {
            1
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("cached material texture"),
            size: wgpu::Extent3d {
                width: image.width,
                height: image.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: levels,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: if image.srgb {
                wgpu::TextureFormat::Rgba8UnormSrgb
            } else {
                wgpu::TextureFormat::Rgba8Unorm
            },
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        #[cfg(target_arch = "wasm32")]
        if let Some(bitmap) = &image.bitmap {
            queue.copy_external_image_to_texture(
                &wgpu::CopyExternalImageSourceInfo {
                    source: wgpu::ExternalImageSource::ImageBitmap(bitmap.0.clone()),
                    origin: wgpu::Origin2d::ZERO,
                    flip_y: false,
                },
                wgpu::CopyExternalImageDestInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                    color_space: wgpu::PredefinedColorSpace::Srgb,
                    premultiplied_alpha: false,
                },
                texture.size(),
            );
        }
        if !image.rgba.is_empty() {
            queue.write_texture(
                texture.as_image_copy(),
                &image.rgba,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(image.width * 4),
                    rows_per_image: Some(image.height),
                },
                texture.size(),
            );
        }
        if levels > 1 {
            let pipeline = self
                .mip_pipelines
                .entry(texture.format())
                .or_insert_with(|| {
                    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some("linear mip filtering"),
                        source: wgpu::ShaderSource::Wgsl(
                            include_str!("shaders/mipmap.wgsl").into(),
                        ),
                    });
                    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("mip filter"),
                        layout: None,
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: Some("vs"),
                            compilation_options: Default::default(),
                            buffers: &[],
                        },
                        fragment: Some(wgpu::FragmentState {
                            module: &shader,
                            entry_point: Some("fs"),
                            compilation_options: Default::default(),
                            targets: &[Some(wgpu::ColorTargetState {
                                format: texture.format(),
                                blend: None,
                                write_mask: wgpu::ColorWrites::ALL,
                            })],
                        }),
                        primitive: Default::default(),
                        depth_stencil: None,
                        multisample: Default::default(),
                        multiview: None,
                        cache: None,
                    })
                });
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });
            let views = (0..levels)
                .map(|level| {
                    texture.create_view(&wgpu::TextureViewDescriptor {
                        base_mip_level: level,
                        mip_level_count: Some(1),
                        ..Default::default()
                    })
                })
                .collect::<Vec<_>>();
            let mut encoder = device.create_command_encoder(&Default::default());
            for level in 1..levels as usize {
                let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("mip source"),
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&views[level - 1]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                });
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("mip generation"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &views[level],
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &bindings, &[]);
                pass.draw(0..3, 0..1);
            }
            queue.submit([encoder.finish()]);
        }
        let sampler = material_sampler(device, image);
        let gpu = GpuTexture {
            view: texture.create_view(&Default::default()),
            sampler,
            texture,
        };
        self.entries
            .insert(key, (Arc::downgrade(image), gpu.clone()));
        self.uploads += 1;
        Ok(gpu)
    }
}

fn material_sampler(device: &wgpu::Device, image: &Texture) -> wgpu::Sampler {
    let wrap = |v| match v {
        Wrapping::Clamp => wgpu::AddressMode::ClampToEdge,
        Wrapping::Repeat => wgpu::AddressMode::Repeat,
        Wrapping::Mirror => wgpu::AddressMode::MirrorRepeat,
    };
    let filter = |v| match v {
        Filter::Nearest => wgpu::FilterMode::Nearest,
        Filter::Linear => wgpu::FilterMode::Linear,
    };
    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wrap(image.wrap_s),
        address_mode_v: wrap(image.wrap_t),
        mag_filter: filter(image.filter),
        min_filter: filter(image.min_filter.unwrap_or(image.filter)),
        mipmap_filter: filter(image.mipmap_filter.unwrap_or(Filter::Nearest)),
        lod_max_clamp: if image.mipmap_filter.is_some() {
            32.0
        } else {
            0.0
        },
        anisotropy_clamp: if image.filter == Filter::Linear
            && image.min_filter.unwrap_or(image.filter) == Filter::Linear
            && image.mipmap_filter == Some(Filter::Linear)
        {
            image.anisotropy.clamp(1, 16)
        } else {
            1
        },
        ..Default::default()
    })
}

fn compressed_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    image: &Texture,
    bytes: &[u8],
) -> Result<GpuTexture> {
    compressed_texture_layers(device, queue, image, bytes, 1, false)
}
fn compressed_texture_layers(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    image: &Texture,
    bytes: &[u8],
    layers: u32,
    array: bool,
) -> Result<GpuTexture> {
    use basisu::{SourceFormat, TargetFormat};
    let t = basisu::Transcoder::new(bytes).map_err(|e| Error::Asset(format!("Basis: {e:?}")))?;
    let features = device.features();
    let (target, linear) = if t.source_format() == SourceFormat::UastcLdr
        && features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC)
    {
        (
            TargetFormat::Astc4x4Rgba,
            wgpu::TextureFormat::Astc {
                block: wgpu::AstcBlock::B4x4,
                channel: wgpu::AstcChannel::Unorm,
            },
        )
    } else if t.source_format() == SourceFormat::Etc1s
        && features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2)
    {
        if t.has_alpha() {
            (TargetFormat::Etc2Rgba, wgpu::TextureFormat::Etc2Rgba8Unorm)
        } else {
            (TargetFormat::Etc1Rgb, wgpu::TextureFormat::Etc2Rgb8Unorm)
        }
    } else if features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC) {
        (TargetFormat::Bc7Rgba, wgpu::TextureFormat::Bc7RgbaUnorm)
    } else if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2) {
        if t.has_alpha() {
            (TargetFormat::Etc2Rgba, wgpu::TextureFormat::Etc2Rgba8Unorm)
        } else {
            (TargetFormat::Etc1Rgb, wgpu::TextureFormat::Etc2Rgb8Unorm)
        }
    } else {
        return Err(Error::Gpu(
            "GPU compressed textures require BC, ETC2 or compatible ASTC support".into(),
        ));
    };
    if image.width == 0
        || image.height == 0
        || image.width > device.limits().max_texture_dimension_2d
        || image.height > device.limits().max_texture_dimension_2d
        || !image.width.is_multiple_of(4)
        || !image.height.is_multiple_of(4)
    {
        return Err(Error::Invalid("GPU block texture dimensions"));
    }
    if t.base_dimensions() != (image.width, image.height)
        || t.level_count() == 0
        || t.level_count() > image.width.max(image.height).ilog2() + 1
        || !t.supports(target)
    {
        return Err(Error::Invalid(
            "compressed texture metadata/target mismatch",
        ));
    }
    let levels = t.level_count();
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("cached compressed material texture"),
        size: wgpu::Extent3d {
            width: image.width,
            height: image.height,
            depth_or_array_layers: layers,
        },
        mip_level_count: levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: if image.srgb {
            linear.add_srgb_suffix()
        } else {
            linear
        },
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    for layer in 0..layers {
        for level in 0..levels {
            let data = t
                .transcode_image(level, layer, 0, target, basisu::DecodeFlags::NONE)
                .map_err(|e| Error::Asset(format!("Basis mip {level}: {e:?}")))?;
            let width = (image.width >> level).max(1).div_ceil(4) * 4;
            let height = (image.height >> level).max(1).div_ceil(4) * 4;
            let row = width / 4 * target.bytes_per_block_or_pixel() as u32;
            if data.len() != row as usize * (height / 4) as usize {
                return Err(Error::Invalid("compressed mip byte count"));
            }
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: level,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: layer,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row),
                    rows_per_image: Some(height / 4),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }
    }
    Ok(GpuTexture {
        view: texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(if array {
                wgpu::TextureViewDimension::D2Array
            } else {
                wgpu::TextureViewDimension::D2
            }),
            ..Default::default()
        }),
        sampler: material_sampler(device, image),
        texture,
    })
}
