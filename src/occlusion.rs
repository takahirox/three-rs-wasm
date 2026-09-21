//! Asynchronous GPU visibility queries with reusable resolve/readback buffers.
use crate::{Error, Result, renderer::Renderer, scene::Object3D};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};
pub struct OcclusionQueries {
    pub(crate) queries: wgpu::QuerySet,
    resolve: wgpu::Buffer,
    staging: wgpu::Buffer,
    capacity: u32,
    objects: Vec<Object3D>,
    pending: Arc<AtomicBool>,
    results: Arc<Mutex<HashMap<Object3D, bool>>>,
}
impl OcclusionQueries {
    /// Capacity counts draw groups, including multiple materials and separately
    /// captured back/front viewport passes of one object.
    pub fn new(r: &Renderer, objects: &[Object3D], capacity: u32) -> Result<Self> {
        if capacity == 0 || capacity > wgpu::QUERY_SET_MAX_QUERIES {
            return Err(Error::Invalid("occlusion query capacity"));
        }
        let queries = r.device.create_query_set(&wgpu::QuerySetDescriptor {
            label: Some("object occlusion"),
            ty: wgpu::QueryType::Occlusion,
            count: capacity,
        });
        let resolve = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("occlusion resolve"),
            size: capacity as u64 * 8,
            usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let staging = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("occlusion readback"),
            size: capacity as u64 * 8,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Ok(Self {
            queries,
            resolve,
            staging,
            capacity,
            objects: objects.to_vec(),
            pending: Arc::new(AtomicBool::new(false)),
            results: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    /// None until a query has completed. Invisible/frustum-culled objects are not queried.
    pub fn is_occluded(&self, object: Object3D) -> Option<bool> {
        self.results.lock().unwrap().get(&object).copied()
    }
    pub(crate) fn available(&self) -> bool {
        !self.pending.load(Ordering::Acquire)
    }
    pub(crate) fn contains(&self, object: Object3D) -> bool {
        self.objects.contains(&object)
    }
    pub(crate) fn validate_count(&self, count: u32) -> Result<()> {
        if count > self.capacity {
            Err(Error::Invalid("occlusion query capacity exceeded"))
        } else {
            Ok(())
        }
    }
    pub(crate) fn resolve(&self, encoder: &mut wgpu::CommandEncoder, count: u32) {
        encoder.resolve_query_set(&self.queries, 0..count, &self.resolve, 0);
        encoder.copy_buffer_to_buffer(&self.resolve, 0, &self.staging, 0, count as u64 * 8);
    }
    pub(crate) fn read(&self, objects: Vec<Object3D>) {
        self.pending.store(true, Ordering::Release);
        let staging = self.staging.clone();
        let pending = self.pending.clone();
        let results = self.results.clone();
        self.staging.slice(..objects.len() as u64 * 8).map_async(
            wgpu::MapMode::Read,
            move |result| {
                if result.is_ok() {
                    let mut values = HashMap::new();
                    {
                        let data = staging.slice(..objects.len() as u64 * 8).get_mapped_range();
                        for (object, bytes) in objects.iter().zip(data.as_chunks::<8>().0.iter()) {
                            let occluded = u64::from_le_bytes(*bytes) == 0;
                            values
                                .entry(*object)
                                .and_modify(|v| *v &= occluded)
                                .or_insert(occluded);
                        }
                    }
                    staging.unmap();
                    *results.lock().unwrap() = values;
                }
                pending.store(false, Ordering::Release);
            },
        );
    }
}
