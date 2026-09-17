//! Reuse draw resources between submissions. Each draw in a submission owns a slot.
use wgpu::util::DeviceExt;

#[derive(PartialEq)]
enum Resource {
    Buffer(wgpu::Buffer, u64, Option<std::num::NonZeroU64>),
    Texture(wgpu::TextureView),
    Sampler(wgpu::Sampler),
}
struct Upload {
    buffer: wgpu::Buffer,
    bytes: Vec<u8>,
}
fn changed_range(previous: &[u8], next: &[u8]) -> Option<std::ops::Range<usize>> {
    debug_assert_eq!(previous.len(), next.len());
    debug_assert_eq!(next.len() % wgpu::COPY_BUFFER_ALIGNMENT as usize, 0);
    let first = previous.iter().zip(next).position(|(a, b)| a != b)?;
    let last = previous
        .iter()
        .zip(next)
        .rposition(|(a, b)| a != b)
        .unwrap();
    let alignment = wgpu::COPY_BUFFER_ALIGNMENT as usize;
    Some(first / alignment * alignment..(last + 1).div_ceil(alignment) * alignment)
}
fn upload(
    slot: &mut Option<Upload>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bytes: &[u8],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    if slot.as_ref().is_none_or(|s| s.bytes.len() != bytes.len()) {
        *slot = Some(Upload {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("resident draw data"),
                contents: bytes,
                usage: usage | wgpu::BufferUsages::COPY_DST,
            }),
            bytes: bytes.to_vec(),
        });
    }
    let slot = slot.as_mut().unwrap();
    if let Some(range) = changed_range(&slot.bytes, bytes) {
        queue.write_buffer(&slot.buffer, range.start as u64, &bytes[range.clone()]);
        slot.bytes[range.clone()].copy_from_slice(&bytes[range]);
    }
    slot.buffer.clone()
}

#[cfg(test)]
mod tests {
    use super::changed_range;
    #[test]
    fn changed_uploads_are_aligned_and_cover_all_edits() {
        let original = [0; 64];
        assert_eq!(changed_range(&original, &original), None);
        for first in 0..64 {
            for last in first..64 {
                let mut next = original;
                next[first] = 1;
                next[last] = 2;
                let range = changed_range(&original, &next).unwrap();
                assert_eq!(range.start % 4, 0);
                assert_eq!(range.end % 4, 0);
                let mut uploaded = original;
                uploaded[range.clone()].copy_from_slice(&next[range]);
                assert_eq!(uploaded, next);
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct Slot {
    uniform: Option<Upload>,
    instances: Option<Upload>,
    indirect: Option<Upload>,
    bindings: Option<CachedBindings>,
}
type CachedBindings = (wgpu::BindGroupLayout, Vec<(u32, Resource)>, wgpu::BindGroup);
impl Slot {
    pub fn indirect(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
    ) -> wgpu::Buffer {
        upload(
            &mut self.indirect,
            device,
            queue,
            bytes,
            wgpu::BufferUsages::INDIRECT,
        )
    }
    pub fn uniform(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
    ) -> wgpu::Buffer {
        upload(
            &mut self.uniform,
            device,
            queue,
            bytes,
            wgpu::BufferUsages::UNIFORM,
        )
    }
    pub fn instances(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
    ) -> wgpu::Buffer {
        upload(
            &mut self.instances,
            device,
            queue,
            bytes,
            wgpu::BufferUsages::VERTEX,
        )
    }
    pub fn bindings(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        entries: &[wgpu::BindGroupEntry<'_>],
    ) -> wgpu::BindGroup {
        let key = entries
            .iter()
            .map(|e| {
                (
                    e.binding,
                    match &e.resource {
                        wgpu::BindingResource::Buffer(b) => {
                            Resource::Buffer(b.buffer.clone(), b.offset, b.size)
                        }
                        wgpu::BindingResource::TextureView(t) => Resource::Texture((*t).clone()),
                        wgpu::BindingResource::Sampler(s) => Resource::Sampler((*s).clone()),
                        _ => unreachable!("draw bindings use individual resources"),
                    },
                )
            })
            .collect::<Vec<_>>();
        if let Some((old_layout, old_key, group)) = &self.bindings
            && old_layout == layout
            && old_key == &key
        {
            return group.clone();
        }
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("resident draw bindings"),
            layout,
            entries,
        });
        self.bindings = Some((layout.clone(), key, group.clone()));
        group
    }
}
