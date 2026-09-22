use super::*;
use crate::{
    compute::{BufferAccess, GpuBuffer},
    readback::BufferReadback,
    tsl::compute::{BufferCompute, BufferStore},
};
pub(in crate::browser) struct Audio {
    kernel: BufferCompute,
    output: GpuBuffer,
    reader: BufferReadback,
    spectrum: wgpu::Texture,
    values: [[f32; 4]; 16],
}
impl Audio {
    pub async fn create(s: &mut Scene, r: &Renderer) -> Result<Self> {
        let window = web_sys::window().ok_or(Error::Invalid("window"))?;
        let source = js_sys::Reflect::get(&window, &"galleryAudioSource".into())
            .map_err(|_| Error::Invalid("audio source"))?;
        let wave = js_sys::Float32Array::new(&source).to_vec();
        if wave.is_empty() {
            return Err(Error::Invalid("audio source is empty"));
        }
        let rate = js_sys::Reflect::get(&window, &"galleryAudioRate".into())
            .map_err(|_| Error::Invalid("sample rate"))?
            .as_f64()
            .filter(|x| x.is_finite() && *x > 0.)
            .ok_or(Error::Invalid("sample rate"))?;
        let original = GpuBuffer::new(r, bytemuck::cast_slice(&wave), BufferAccess::Read)?;
        let output = GpuBuffer::zeroed(r, wave.len() as u64 * 4, BufferAccess::ReadWrite)?;
        let index = instance_index().to_float();
        let pitch = uniform(0, Type::Float);
        let volume = uniform(1, Type::Float);
        let delay = uniform(2, Type::Float);
        let mut result = storage_element(0, (index.clone() * pitch.clone()).to_uint());
        for i in 1..7 {
            let time = (index.clone() - delay.clone() * float(rate as f32) * float(i as f32))
                * pitch.clone();
            result = result
                + storage_element(0, time.to_uint()) * (volume.clone() / float((i * i) as f32));
        }
        let kernel = BufferCompute::new(
            r,
            wave.len() as u32,
            &[(&original, Type::Float), (&output, Type::Float)],
            &[BufferStore {
                binding: 1,
                index: instance_index(),
                value: result,
            }],
        )
        .await?;
        let reader = BufferReadback::new(r, wave.len() as u64 * 4)?;
        let spectrum = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("audio analyser"),
            size: wgpu::Extent3d {
                width: 1024,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let sampler = r.device.create_sampler(&Default::default());
        let screen = tsl::viewport::screen_uv();
        let value = tsl::Texture::External(0)
            .sample(vec2(screen.x(), screen.x()))
            .x()
            * screen.y();
        // A single full-screen GPU background, as in the official scene.
        let material = background_material_with_textures(
            r,
            vec4(vec3(float(0.), float(0.), value), float(1.)),
            &[(&spectrum.create_view(&Default::default()), &sampler)],
        )
        .await?;
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
            Material::Shader(material),
        );
        s.get_mut(h)?.frustum_culled = false;
        let mut values = [[0.; 4]; 16];
        values[0][0] = 1.5;
        values[1][0] = 0.2;
        values[2][0] = 0.55;
        Ok(Self {
            kernel,
            output,
            reader,
            spectrum,
            values,
        })
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        let (min, max) = match i {
            0 => (0.5, 2.),
            1 => (0., 1.),
            2 => (0.1, 1.),
            _ => return Err(Error::Invalid("audio parameter")),
        };
        if !v.is_finite() {
            return Err(Error::Invalid("audio parameter"));
        }
        self.values[i][0] = v.clamp(min, max);
        Ok(())
    }
    pub fn play(&self, r: &Renderer) -> Result<bool> {
        if !self.reader.is_idle() {
            return Ok(false);
        }
        self.kernel.set_uniforms(r, &self.values)?;
        self.kernel.dispatch(r)?;
        self.reader.begin_notify(r, &self.output.buffer, 0, || {
            if let Some(w) = web_sys::window()
                && let Ok(e) = web_sys::Event::new("gallery-audio-ready")
            {
                let _ = w.dispatch_event(&e);
            }
        })?;
        Ok(true)
    }
    pub fn take(&self) -> Option<Result<Vec<f32>>> {
        self.reader.take().map(|result| {
            result.map(|bytes| {
                bytes
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|v| f32::from_le_bytes(*v))
                    .collect()
            })
        })
    }
    pub fn spectrum(&self, r: &Renderer, bytes: &[u8]) -> Result<()> {
        if bytes.len() != 1024 {
            return Err(Error::Invalid("audio spectrum size"));
        }
        r.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.spectrum,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1024),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 1024,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        Ok(())
    }
}
