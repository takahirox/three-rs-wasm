//! GPU-resident geometry, invalidated by attribute versions or a new Arc allocation.
use crate::{Error, Result, geometry::BufferGeometry, math::*, renderer::Vertex};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
#[derive(Clone)]
pub(crate) struct Buffers {
    pub vertices: wgpu::Buffer,
    pub indices: Option<wgpu::Buffer>,
}
struct Entry {
    owner: Weak<BufferGeometry>,
    versions: Vec<u64>,
    buffers: Buffers,
}
struct Bounds {
    owner: Weak<BufferGeometry>,
    versions: Vec<u64>,
    sphere: Sphere,
}
#[derive(Default)]
pub(crate) struct Cache {
    entries: HashMap<(usize, bool, bool, bool, Option<bool>), Entry>,
    bounds: HashMap<usize, Bounds>,
    pub uploads: u64,
    pub bytes: u64,
}
impl Cache {
    pub fn prune(&mut self) {
        self.entries.retain(|_, e| e.owner.strong_count() > 0);
        self.bounds.retain(|_, e| e.owner.strong_count() > 0);
    }
    pub fn sphere(&mut self, geometry: &Arc<BufferGeometry>) -> Result<Sphere> {
        if let Some(sphere) = geometry.bounding_sphere {
            return Ok(sphere);
        }
        let key = Arc::as_ptr(geometry) as usize;
        let versions = geometry
            .attributes
            .get("position")
            .into_iter()
            .chain(
                geometry
                    .morph_attributes
                    .get("position")
                    .into_iter()
                    .flatten(),
            )
            .map(|a| a.version())
            .collect::<Vec<_>>();
        if let Some(bounds) = self.bounds.get(&key)
            && bounds.owner.strong_count() > 0
            && bounds.versions == versions
        {
            return Ok(bounds.sphere);
        }
        let sphere = (**geometry).clone().compute_bounding_sphere()?;
        self.bounds.insert(
            key,
            Bounds {
                owner: Arc::downgrade(geometry),
                versions,
                sphere,
            },
        );
        Ok(sphere)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn get(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        geometry: &Arc<BufferGeometry>,
        vertex_colors: bool,
        is_points: bool,
        wireframe: bool,
        wide: Option<bool>,
    ) -> Result<Buffers> {
        let key = (
            Arc::as_ptr(geometry) as usize,
            vertex_colors,
            is_points,
            wireframe,
            wide,
        );
        let versions = geometry
            .attributes
            .values()
            .map(|a| a.version())
            .collect::<Vec<_>>();
        if let Some(entry) = self.entries.get(&key)
            && entry.owner.strong_count() > 0
            && entry.versions == versions
        {
            return Ok(entry.buffers.clone());
        }
        let mut vertices = Vec::new();
        let positions = geometry
            .attributes
            .get("position")
            .ok_or(Error::Invalid("position attribute"))?;
        for index in 0..geometry.vertex_count() {
            let position = positions.vector3(index)?.as_vec3().to_array();
            let normal = geometry
                .attributes
                .get("normal")
                .map(|a| a.vector3(index))
                .transpose()?
                .unwrap_or(Vector3::Z)
                .as_vec3()
                .to_array();
            let mut uv = [0.0; 2];
            if let Some(a) = geometry.attributes.get("uv") {
                uv = [
                    a.get_component(index, 0)? as f32,
                    a.get_component(index, 1)? as f32,
                ];
            }
            let uv1 = geometry
                .attributes
                .get("uv1")
                .map(|a| {
                    Ok::<_, Error>([
                        a.get_component(index, 0)? as f32,
                        a.get_component(index, 1)? as f32,
                    ])
                })
                .transpose()?
                .unwrap_or([0.0; 2]);
            let mut color = [1.0; 4];
            if vertex_colors && let Some(a) = geometry.attributes.get("color") {
                for (c, value) in color.iter_mut().enumerate().take(a.item_size().min(4)) {
                    *value = a.get_component(index, c)? as f32;
                }
            }
            let mut tangent = [0.0; 4];
            if let Some(a) = geometry.attributes.get("tangent") {
                for (c, value) in tangent.iter_mut().enumerate() {
                    *value = a.get_component(index, c)? as f32;
                }
            }
            vertices.push(Vertex {
                position,
                normal,
                uv,
                color,
                // Native one-pixel dashed lines carry the precomputed distance
                // in the otherwise unused expansion coordinate.
                corner: [
                    geometry
                        .attributes
                        .get("lineDistance")
                        .map(|a| a.get_component(index, 0))
                        .transpose()?
                        .unwrap_or(0.0) as f32,
                    0.0,
                ],
                tangent,
                uv1,
            });
        }
        if is_points {
            let centers = vertices;
            vertices = Vec::with_capacity(centers.len() * 6);
            for center in centers {
                for corner in [
                    [-1.0, -1.0],
                    [1.0, -1.0],
                    [-1.0, 1.0],
                    [-1.0, 1.0],
                    [1.0, -1.0],
                    [1.0, 1.0],
                ] {
                    let mut vertex = center;
                    vertex.corner = corner;
                    if !geometry.has_attribute("uv") {
                        // Three uses (PointCoord.x, 1 - PointCoord.y): UVs
                        // increase upwards before the texture's flipY transform.
                        vertex.uv = [corner[0] * 0.5 + 0.5, corner[1] * 0.5 + 0.5];
                    }
                    vertices.push(vertex);
                }
            }
        }

        if let Some(segments) = wide {
            let source = vertices;
            vertices = Vec::new();
            let mut distance = 0.0f32;
            let step = if segments { 2 } else { 1 };
            for i in (0..geometry.draw_count().saturating_sub(1)).step_by(step) {
                let a = geometry.vertex_index(i)?;
                let b = geometry.vertex_index(i + 1)?;
                let p = source[a];
                let q = source[b];
                let da = geometry
                    .attributes
                    .get("lineDistance")
                    .map(|d| d.get_component(a, 0).map(|v| v as f32))
                    .transpose()?
                    .unwrap_or(distance);
                distance += (glam::Vec3::from_array(q.position)
                    - glam::Vec3::from_array(p.position))
                .length();
                let db = geometry
                    .attributes
                    .get("lineDistance")
                    .map(|d| d.get_component(b, 0).map(|v| v as f32))
                    .transpose()?
                    .unwrap_or(distance);
                for corner in [
                    [0.0, -1.0],
                    [1.0, -1.0],
                    [0.0, 1.0],
                    [0.0, 1.0],
                    [1.0, -1.0],
                    [1.0, 1.0],
                ] {
                    let mut v = if corner[0] == 0.0 { p } else { q };
                    v.position = p.position;
                    v.normal = q.position;
                    v.corner = corner;
                    v.tangent = [da, db, a as f32, b as f32];
                    vertices.push(v);
                }
            }
        }
        let vertex_bytes = bytemuck::cast_slice(&vertices);
        let vertex_buffer = upload(
            device,
            queue,
            "resident vertices",
            vertex_bytes,
            wgpu::BufferUsages::VERTEX,
        );
        let indices = if wide.is_some() {
            None
        } else if wireframe {
            let mut indices = Vec::with_capacity(geometry.draw_count() * 2);
            for i in (0..geometry.draw_count().saturating_sub(2)).step_by(3) {
                let a = geometry.vertex_index(i)? as u32;
                let b = geometry.vertex_index(i + 1)? as u32;
                let c = geometry.vertex_index(i + 2)? as u32;
                indices.extend([a, b, b, c, c, a]);
            }
            Some(indices)
        } else if let Some(indices) = &geometry.index {
            if indices
                .iter()
                .any(|&i| i as usize >= geometry.vertex_count())
            {
                return Err(Error::Invalid("geometry index"));
            }
            Some(if is_points {
                indices
                    .iter()
                    .flat_map(|i| (0..6).map(move |corner| i * 6 + corner))
                    .collect()
            } else {
                indices.clone()
            })
        } else {
            None
        };
        let index_buffer = indices.as_ref().map(|indices| {
            upload(
                device,
                queue,
                "resident indices",
                bytemuck::cast_slice(indices),
                wgpu::BufferUsages::INDEX,
            )
        });
        for a in geometry.attributes.values() {
            a.notify_uploaded();
        }
        self.uploads += 1;
        self.bytes +=
            vertex_bytes.len() as u64 + indices.as_ref().map_or(0, |v| v.len() as u64 * 4);
        let buffers = Buffers {
            vertices: vertex_buffer,
            indices: index_buffer,
        };
        self.entries.insert(
            key,
            Entry {
                owner: Arc::downgrade(geometry),
                versions,
                buffers: buffers.clone(),
            },
        );
        Ok(buffers)
    }
}

/// Queue uploads, not mapped-at-creation buffers: WebGPU's mapped range holds a
/// Wasm memory view until unmap, which allocator growth can detach.
fn upload(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    bytes: &[u8],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let size =
        (bytes.len() as u64).div_ceil(wgpu::COPY_BUFFER_ALIGNMENT) * wgpu::COPY_BUFFER_ALIGNMENT;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: size.max(wgpu::COPY_BUFFER_ALIGNMENT),
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    if bytes.len() as u64 == size {
        queue.write_buffer(&buffer, 0, bytes);
    } else {
        let mut padded = bytes.to_vec();
        padded.resize(size as usize, 0);
        queue.write_buffer(&buffer, 0, &padded);
    }
    buffer
}
