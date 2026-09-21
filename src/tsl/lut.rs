//! Native 3D color lookup tables. Parsing happens once; trilinear grading is GPU work.
use crate::{Error, Result, postprocessing::Effect, renderer::*};
#[derive(Debug)]
pub struct Lut3D {
    pub size: u32,
    pub rgba: Vec<u8>,
}
impl Lut3D {
    pub fn new(size: u32, rgba: Vec<u8>) -> Result<Self> {
        if !(2..=256).contains(&size)
            || rgba.len() != size as usize * size as usize * size as usize * 4
        {
            return Err(Error::Invalid("LUT dimensions/data"));
        }
        Ok(Self { size, rgba })
    }
    pub fn from_cube(source: &str) -> Result<Self> {
        let mut size = 0;
        let mut rgba = vec![];
        for line in source.lines() {
            let v = line
                .split('#')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .collect::<Vec<_>>();
            if v.first() == Some(&"LUT_3D_SIZE") {
                size = v.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);
            } else if v.first().is_some_and(|v| v.starts_with("DOMAIN_")) {
                let expected = if v[0] == "DOMAIN_MIN" { 0.0 } else { 1.0 };
                if v.len() != 4
                    || v[1..]
                        .iter()
                        .any(|x| x.parse::<f64>().ok() != Some(expected))
                {
                    return Err(Error::Invalid("LUT requires normalized domain"));
                }
            } else if v.len() == 3 {
                let values = v
                    .iter()
                    .map(|v| v.parse::<f64>())
                    .collect::<std::result::Result<Vec<_>, _>>();
                if let Ok(values) = values {
                    if values
                        .iter()
                        .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
                    {
                        return Err(Error::Invalid("LUT value"));
                    }
                    rgba.extend(values.into_iter().map(|v| (v * 255.0) as u8));
                    rgba.push(255);
                }
            }
        }
        Self::new(size, rgba)
    }
    pub fn from_3dl(source: &str) -> Result<Self> {
        let rows = source
            .lines()
            .filter_map(|l| {
                let v = l
                    .split_whitespace()
                    .map(str::parse::<f64>)
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .ok()?;
                (!v.is_empty()).then_some(v)
            })
            .collect::<Vec<_>>();
        let grid = rows.first().ok_or(Error::Invalid("3DL grid"))?;
        let size = grid.len();
        if !(2..=256).contains(&size)
            || grid.windows(2).any(|w| w[1] - w[0] != grid[1] - grid[0])
            || rows.len() != 1 + size.pow(3)
        {
            return Err(Error::Invalid("3DL grid/data"));
        }
        if rows[1..]
            .iter()
            .any(|r| r.len() != 3 || r.iter().any(|v| !v.is_finite() || *v < 0.0))
        {
            return Err(Error::Invalid("3DL value"));
        }
        let max = rows[1..].iter().flatten().copied().fold(0.0, f64::max);
        if max <= 0.0 {
            return Err(Error::Invalid("3DL range"));
        }
        let scale = 255.0 / 2.0f64.powf(max.log2().ceil());
        let mut rgba = vec![255; size.pow(3) * 4];
        for (i, row) in rows[1..].iter().enumerate() {
            let j = ((i % size) * size * size + (i / size % size) * size + i / (size * size)) * 4;
            for c in 0..3 {
                rgba[j + c] = (row[c] * scale) as u8;
            }
        }
        Self::new(size as u32, rgba)
    }
    pub fn from_strip(width: u32, height: u32, rgba: &[u8]) -> Result<Self> {
        if width.min(height) < 2
            || width.min(height) > 256
            || width.max(height) > 256 * 256
            || rgba.len() != width as usize * height as usize * 4
        {
            return Err(Error::Invalid("LUT strip pixels"));
        }
        if height == width * width {
            return Self::new(width, rgba.to_vec());
        }
        if width != height * height {
            return Err(Error::Invalid("LUT strip dimensions"));
        }
        let n = height as usize;
        let mut output = vec![0; rgba.len()];
        for z in 0..n {
            for y in 0..n {
                let src = (y * n * n + z * n) * 4;
                let dst = (z * n * n + y * n) * 4;
                output[dst..dst + n * 4].copy_from_slice(&rgba[src..src + n * 4]);
            }
        }
        Self::new(height, output)
    }
    /// GPU color grading. Set `linear_input` to encode linear scene color to
    /// sRGB in this same pass; LUTs are defined in display color space.
    /// `parameters[0][0]` controls the grading intensity (default 1).
    pub async fn effect(
        &self,
        r: &Renderer,
        format: wgpu::TextureFormat,
        linear_input: bool,
    ) -> Result<Effect> {
        if !(2..=256).contains(&self.size)
            || self.rgba.len() != (self.size as usize).pow(3) * 4
            || self.size > r.device.limits().max_texture_dimension_3d
        {
            return Err(Error::Invalid("LUT exceeds GPU limits"));
        }
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("3D LUT"),
            size: wgpu::Extent3d {
                width: self.size,
                height: self.size,
                depth_or_array_layers: self.size,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        r.queue.write_texture(
            texture.as_image_copy(),
            &self.rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.size * 4),
                rows_per_image: Some(self.size),
            },
            texture.size(),
        );
        let view = texture.create_view(&Default::default());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut effect=Effect::with_texture_dimensions(r,format,r#"
@group(1) @binding(0) var lut:texture_3d<f32>;
@group(1) @binding(1) var lut_sampler:sampler;
fn effect(uv:vec2<f32>)->vec4<f32>{var base=textureSample(input_texture,input_sampler,uv);if params[0].y>0.5 {let c=base.rgb;base=vec4(select(1.055*pow(max(c,vec3(0.0)),vec3(0.41666))-0.055,c*12.92,c<=vec3(0.0031308)),base.a);}let size=f32(textureDimensions(lut).x);let coordinate=vec3(0.5/size)+base.rgb*(1.0-1.0/size);return vec4(mix(base.rgb,textureSample(lut,lut_sampler,coordinate).rgb,params[0].x),base.a);}
"#,&[(&view,&sampler,wgpu::TextureViewDimension::D3)]).await?;
        effect.parameters[0] = [1.0, if linear_input { 1.0 } else { 0.0 }, 0.0, 0.0];
        Ok(effect)
    }
}
