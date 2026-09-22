//! Reusable asynchronous RGBA8 render-target readback, including MRT attachments.
use crate::{
    Error, Result,
    renderer::{RenderTarget, Renderer},
};
use std::sync::{Arc, Mutex};
enum State {
    Idle,
    Pending,
    Ready(Result<Vec<u8>>),
}
pub struct RgbaReadback {
    width: u32,
    height: u32,
    stride: u32,
    buffer: wgpu::Buffer,
    state: Arc<Mutex<State>>,
}
impl RgbaReadback {
    pub fn new(r: &Renderer, width: u32, height: u32) -> Result<Self> {
        if width == 0
            || height == 0
            || width > r.device.limits().max_texture_dimension_2d
            || height > r.device.limits().max_texture_dimension_2d
        {
            return Err(Error::Invalid("readback dimensions"));
        }
        let stride = (width * 4).div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT)
            * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let size = stride as u64 * height as u64;
        if size > r.device.limits().max_buffer_size {
            return Err(Error::Invalid("readback buffer size"));
        }
        Ok(Self {
            width,
            height,
            stride,
            buffer: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("resident RGBA readback"),
                size,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            state: Arc::new(Mutex::new(State::Idle)),
        })
    }
    /// Starts one copy. Consume the previous result before starting another.
    /// Completion runs from browser GPU callbacks or a native device poll.
    pub fn begin(&self, r: &Renderer, target: &RenderTarget, attachment: usize) -> Result<()> {
        self.begin_notify(r, target, attachment, || {})
    }
    pub fn begin_notify(
        &self,
        r: &Renderer,
        target: &RenderTarget,
        attachment: usize,
        notify: impl FnOnce() + Send + 'static,
    ) -> Result<()> {
        if target.width != self.width || target.height != self.height {
            return Err(Error::Invalid("readback target dimensions"));
        }
        let textures = target.textures();
        let texture = textures
            .get(attachment)
            .ok_or(Error::Invalid("readback attachment"))?;
        if !matches!(
            texture.format(),
            wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Rgba8UnormSrgb
        ) || !texture.usage().contains(wgpu::TextureUsages::COPY_SRC)
        {
            return Err(Error::Invalid("readback requires copyable RGBA8 texture"));
        }
        let mut state = self.state.lock().unwrap();
        if !matches!(*state, State::Idle) {
            return Err(Error::Invalid("readback result pending"));
        }
        *state = State::Pending;
        drop(state);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: target.layer,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(self.stride),
                    rows_per_image: Some(self.height),
                },
            },
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
        r.queue.submit([encoder.finish()]);
        map(
            &self.buffer,
            self.state.clone(),
            self.width as usize * 4,
            self.stride as usize,
            notify,
        );
        Ok(())
    }
    pub fn take(&self) -> Option<Result<Vec<u8>>> {
        take(&self.state)
    }
    pub fn is_idle(&self) -> bool {
        matches!(*self.state.lock().unwrap(), State::Idle)
    }
}

/// Reusable asynchronous readback of a fixed-size region of a GPU buffer.
/// The source remains GPU-resident; mapping never blocks the browser thread.
pub struct BufferReadback {
    buffer: wgpu::Buffer,
    state: Arc<Mutex<State>>,
}
impl BufferReadback {
    pub fn new(r: &Renderer, size: u64) -> Result<Self> {
        if size == 0 || !size.is_multiple_of(4) || size > r.device.limits().max_buffer_size {
            return Err(Error::Invalid("readback buffer size/alignment"));
        }
        Ok(Self {
            buffer: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("resident buffer readback"),
                size,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            state: Arc::new(Mutex::new(State::Idle)),
        })
    }
    pub fn begin(&self, r: &Renderer, source: &wgpu::Buffer, offset: u64) -> Result<()> {
        self.begin_notify(r, source, offset, || {})
    }
    pub fn begin_notify(
        &self,
        r: &Renderer,
        source: &wgpu::Buffer,
        offset: u64,
        notify: impl FnOnce() + Send + 'static,
    ) -> Result<()> {
        let size = self.buffer.size();
        if !offset.is_multiple_of(4)
            || offset
                .checked_add(size)
                .is_none_or(|end| end > source.size())
            || !source.usage().contains(wgpu::BufferUsages::COPY_SRC)
        {
            return Err(Error::Invalid("readback source range/usage"));
        }
        let mut state = self.state.lock().unwrap();
        if !matches!(*state, State::Idle) {
            return Err(Error::Invalid("readback result pending"));
        }
        *state = State::Pending;
        drop(state);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(source, offset, &self.buffer, 0, size);
        r.queue.submit([encoder.finish()]);
        map(
            &self.buffer,
            self.state.clone(),
            size as usize,
            size as usize,
            notify,
        );
        Ok(())
    }
    pub fn take(&self) -> Option<Result<Vec<u8>>> {
        take(&self.state)
    }
    pub fn is_idle(&self) -> bool {
        matches!(*self.state.lock().unwrap(), State::Idle)
    }
}
fn take(state: &Mutex<State>) -> Option<Result<Vec<u8>>> {
    let mut state = state.lock().unwrap();
    if matches!(*state, State::Ready(_)) {
        let State::Ready(result) = std::mem::replace(&mut *state, State::Idle) else {
            unreachable!()
        };
        Some(result)
    } else {
        None
    }
}
fn map(
    buffer: &wgpu::Buffer,
    state: Arc<Mutex<State>>,
    row: usize,
    stride: usize,
    notify: impl FnOnce() + Send + 'static,
) {
    let mapped = buffer.clone();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let result = result.map_err(|e| Error::Gpu(e.to_string())).map(|()| {
                let data = mapped.slice(..).get_mapped_range();
                let bytes = data
                    .chunks_exact(stride)
                    .flat_map(|r| r[..row].iter().copied())
                    .collect();
                drop(data);
                mapped.unmap();
                bytes
            });
            *state.lock().unwrap() = State::Ready(result);
            notify();
        });
}
