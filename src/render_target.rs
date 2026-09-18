use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct RenderTargetOptions {
    pub samples: u32,
    pub count: u32,
    pub depth: u32,
    pub depth_buffer: bool,
    pub stencil_buffer: bool,
    pub resolve_color_buffer: bool,
    pub resolve_depth_buffer: bool,
    pub resolve_stencil_buffer: bool,
    pub store_multisampled_color_buffer: bool,
    pub store_multisampled_depth_buffer: bool,
    pub store_multisampled_stencil_buffer: bool,
    pub use_array_depth_texture: bool,
    pub format: wgpu::TextureFormat,
    /// Tone-map (using scene settings) and encode built-in output before blending,
    /// matching the WebGL canvas.
    /// Use an unorm (non-sRGB) attachment and present without another conversion.
    pub encode_srgb: bool,
}
impl Default for RenderTargetOptions {
    fn default() -> Self {
        Self {
            samples: 0,
            count: 1,
            depth: 1,
            depth_buffer: true,
            stencil_buffer: false,
            resolve_color_buffer: true,
            resolve_depth_buffer: true,
            resolve_stencil_buffer: true,
            store_multisampled_color_buffer: true,
            store_multisampled_depth_buffer: true,
            store_multisampled_stencil_buffer: true,
            use_array_depth_texture: false,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            encode_srgb: false,
        }
    }
}

/// Owns color/depth attachments. Clone and copy operations allocate independent
/// attachments without copying pixels, as Three.js RenderTarget does.
pub struct RenderTarget {
    pub texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) textures: Vec<wgpu::Texture>,
    pub(crate) views: Vec<wgpu::TextureView>,
    pub(crate) multisampled: Vec<wgpu::Texture>,
    pub(crate) multisampled_views: Vec<wgpu::TextureView>,
    depth_texture: Option<wgpu::Texture>,
    pub(crate) depth_view: Option<wgpu::TextureView>,
    pub width: u32,
    pub height: u32,
    pub viewport: [u32; 4],
    pub scissor: Option<[u32; 4]>,
    pub(crate) options: RenderTargetOptions,
    pub(crate) dimension: wgpu::TextureDimension,
    pub(crate) layer: u32,
}
impl RenderTarget {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Result<Self> {
        Self::with_options(device, width, height, RenderTargetOptions::default())
    }
    pub fn with_options(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        options: RenderTargetOptions,
    ) -> Result<Self> {
        Self::allocate(device, width, height, options, wgpu::TextureDimension::D2)
    }
    fn allocate(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        options: RenderTargetOptions,
        dimension: wgpu::TextureDimension,
    ) -> Result<Self> {
        let limits = device.limits();
        let samples = options.samples.max(1);
        let max_size = if dimension == wgpu::TextureDimension::D3 {
            limits.max_texture_dimension_3d
        } else {
            limits.max_texture_dimension_2d
        };
        if width == 0
            || height == 0
            || width > max_size
            || height > max_size
            || options.depth == 0
            || options.count == 0
            || options.count > limits.max_color_attachments
            || ![1, 4].contains(&samples)
        {
            return Err(Error::Invalid(
                "render target dimensions, attachment count or samples",
            ));
        }
        if options.depth > 1 && samples > 1 {
            return Err(Error::Invalid("multisampling layered textures"));
        }
        if dimension == wgpu::TextureDimension::D3 && options.depth > max_size
            || dimension == wgpu::TextureDimension::D2
                && options.depth > limits.max_texture_array_layers
        {
            return Err(Error::Invalid("render target depth"));
        }
        let mut textures = Vec::new();
        let mut views = Vec::new();
        let mut multisampled = Vec::new();
        let mut multisampled_views = Vec::new();
        for _ in 0..options.count {
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("render target color"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: options.depth,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension,
                format: options.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let view = Self::color_view(&texture, dimension, 0);
            textures.push(texture);
            views.push(view);
            if samples > 1 {
                let msaa = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("render target msaa"),
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format: options.format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                });
                multisampled_views.push(msaa.create_view(&Default::default()));
                multisampled.push(msaa);
            }
        }
        let depth_texture = if options.depth_buffer || options.stencil_buffer {
            Some(device.create_texture(&wgpu::TextureDescriptor {
                label: Some("render target depth"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: if options.use_array_depth_texture {
                        options.depth
                    } else {
                        1
                    },
                },
                mip_level_count: 1,
                sample_count: samples,
                dimension: wgpu::TextureDimension::D2,
                format: if options.stencil_buffer {
                    wgpu::TextureFormat::Depth24PlusStencil8
                } else {
                    wgpu::TextureFormat::Depth32Float
                },
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }))
        } else {
            None
        };
        let depth_view = depth_texture
            .as_ref()
            .map(|t| Self::color_view(t, wgpu::TextureDimension::D2, 0));
        Ok(Self {
            texture: textures[0].clone(),
            view: views[0].clone(),
            textures,
            views,
            multisampled,
            multisampled_views,
            depth_texture,
            depth_view,
            width,
            height,
            viewport: [0, 0, width, height],
            scissor: None,
            options,
            dimension,
            layer: 0,
        })
    }
    fn color_view(
        texture: &wgpu::Texture,
        dimension: wgpu::TextureDimension,
        layer: u32,
    ) -> wgpu::TextureView {
        texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(if dimension == wgpu::TextureDimension::D3 {
                wgpu::TextureViewDimension::D3
            } else {
                wgpu::TextureViewDimension::D2
            }),
            base_array_layer: if dimension == wgpu::TextureDimension::D3 {
                0
            } else {
                layer
            },
            array_layer_count: if dimension == wgpu::TextureDimension::D3 {
                None
            } else {
                Some(1)
            },
            ..Default::default()
        })
    }
    pub fn options(&self) -> &RenderTargetOptions {
        &self.options
    }
    pub fn textures(&self) -> &[wgpu::Texture] {
        &self.textures
    }
    pub fn depth_texture(&self) -> Option<&wgpu::Texture> {
        self.depth_texture.as_ref()
    }
    pub fn set_depth_texture(&mut self, texture: wgpu::Texture) -> Result<()> {
        let expected_layers = if self.options.use_array_depth_texture {
            self.options.depth
        } else {
            1
        };
        if texture.width() != self.width
            || texture.height() != self.height
            || texture.depth_or_array_layers() != expected_layers
            || texture.sample_count() != self.options.samples.max(1)
            || Some(texture.format()) != self.depth_format()
            || !texture
                .usage()
                .contains(wgpu::TextureUsages::RENDER_ATTACHMENT)
        {
            return Err(Error::Invalid("depth attachment"));
        }
        self.depth_view = Some(Self::color_view(
            &texture,
            wgpu::TextureDimension::D2,
            if self.options.use_array_depth_texture {
                self.layer
            } else {
                0
            },
        ));
        self.depth_texture = Some(texture);
        Ok(())
    }
    pub fn set_texture(&mut self, index: usize, texture: wgpu::Texture) -> Result<()> {
        if index >= self.textures.len()
            || texture.width() != self.width
            || texture.height() != self.height
            || texture.depth_or_array_layers() != self.options.depth
            || texture.sample_count() != 1
            || texture.dimension() != self.dimension
            || texture.format() != self.options.format
            || !texture
                .usage()
                .contains(wgpu::TextureUsages::RENDER_ATTACHMENT)
        {
            return Err(Error::Invalid("color attachment"));
        }
        self.views[index] = Self::color_view(&texture, self.dimension, self.layer);
        self.textures[index] = texture;
        self.texture = self.textures[0].clone();
        self.view = self.views[0].clone();
        Ok(())
    }
    pub fn set_layer(&mut self, layer: u32) -> Result<()> {
        if layer >= self.options.depth {
            return Err(Error::Invalid("render target layer"));
        }
        self.layer = layer;
        self.views = self
            .textures
            .iter()
            .map(|t| Self::color_view(t, self.dimension, layer))
            .collect();
        self.view = self.views[0].clone();
        if self.options.use_array_depth_texture {
            self.depth_view = self
                .depth_texture
                .as_ref()
                .map(|t| Self::color_view(t, wgpu::TextureDimension::D2, layer));
        }
        Ok(())
    }
    pub fn set_size(&mut self, device: &wgpu::Device, width: u32, height: u32) -> Result<()> {
        self.set_size_3d(device, width, height, self.options.depth)
    }
    pub fn set_size_3d(
        &mut self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
        depth: u32,
    ) -> Result<()> {
        if self.width != width || self.height != height || self.options.depth != depth {
            let mut options = self.options.clone();
            options.depth = depth;
            *self = Self::allocate(device, width, height, options, self.dimension)?;
        }
        self.viewport = [0, 0, width, height];
        self.scissor = None;
        Ok(())
    }
    pub fn clone_target(&self, device: &wgpu::Device) -> Result<Self> {
        let mut target = Self::allocate(
            device,
            self.width,
            self.height,
            self.options.clone(),
            self.dimension,
        )?;
        target.viewport = self.viewport;
        target.scissor = self.scissor;
        Ok(target)
    }
    pub fn copy_from(&mut self, device: &wgpu::Device, source: &Self) -> Result<()> {
        *self = source.clone_target(device)?;
        Ok(())
    }
    pub fn dispose(self) {
        for t in &self.textures {
            t.destroy();
        }
        for t in &self.multisampled {
            t.destroy();
        }
        if let Some(t) = self.depth_texture {
            t.destroy();
        }
    }
    pub(crate) fn depth_format(&self) -> Option<wgpu::TextureFormat> {
        self.depth_texture.as_ref().map(wgpu::Texture::format)
    }
}

/// A 3D texture attachment. Select the rendered depth slice with `set_layer`.
pub struct RenderTarget3D(pub RenderTarget);
impl RenderTarget3D {
    pub fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        depth: u32,
        mut options: RenderTargetOptions,
    ) -> Result<Self> {
        options.depth = depth;
        Ok(Self(RenderTarget::allocate(
            device,
            width,
            height,
            options,
            wgpu::TextureDimension::D3,
        )?))
    }
}
impl std::ops::Deref for RenderTarget3D {
    type Target = RenderTarget;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for RenderTarget3D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
