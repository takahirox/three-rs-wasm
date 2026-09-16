//! Typed attribute storage with explicit upload versions and shared interleaving.
use crate::{Error, Result, math::*};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct UploadHook(Option<std::sync::Arc<dyn Fn() + Send + Sync>>);
impl Clone for UploadHook {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for UploadHook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UploadHook")
            .field(&self.0.is_some())
            .finish()
    }
}

pub trait Component: Copy + Default + std::fmt::Debug + Send + Sync + 'static {
    fn decode(self, normalized: bool) -> f64;
    fn encode(value: f64, normalized: bool) -> Self;
}
macro_rules! unsigned {
    ($($t:ty),*) => {$ (
        impl Component for $t {
            fn decode(self, normalized: bool) -> f64 { if normalized { self as f64 / <$t>::MAX as f64 } else { self as f64 } }
            fn encode(v: f64, normalized: bool) -> Self { let v=if normalized { (v * <$t>::MAX as f64 + 0.5).floor() } else {v}; js_integer(v,<$t>::BITS) as Self }
        }
    )*};
}
macro_rules! signed {
    ($($t:ty),*) => {$ (
        impl Component for $t {
            fn decode(self, normalized: bool) -> f64 { if normalized { (self as f64 / <$t>::MAX as f64).max(-1.0) } else { self as f64 } }
            fn encode(v: f64, normalized: bool) -> Self { let v=if normalized { (v * <$t>::MAX as f64 + 0.5).floor() } else {v}; js_integer(v,<$t>::BITS) as Self }
        }
    )*};
}
unsigned!(u8, u16, u32);
signed!(i8, i16, i32);
fn js_integer(v: f64, bits: u32) -> u32 {
    if !v.is_finite() {
        0
    } else {
        v.trunc().rem_euclid(2_f64.powi(bits as i32)) as u32
    }
}
impl Component for f32 {
    fn decode(self, _: bool) -> f64 {
        self as f64
    }
    fn encode(v: f64, _: bool) -> Self {
        v as Self
    }
}
impl Component for f64 {
    fn decode(self, _: bool) -> f64 {
        self
    }
    fn encode(v: f64, _: bool) -> Self {
        v
    }
}
impl Component for half::f16 {
    fn decode(self, _: bool) -> f64 {
        self.to_f64()
    }
    fn encode(v: f64, _: bool) -> Self {
        // r186 DataUtils truncates the float32 mantissa and clamps overflow.
        // IEEE round-to-nearest conversion alone would change observable values.
        let bits = (v.clamp(-65504.0, 65504.0) as f32).to_bits();
        let sign = (bits >> 16) & 0x8000;
        let exponent = ((bits >> 23) & 255) as i32 - 127;
        let mantissa = bits & 0x7fffff;
        let (base, shift) = if exponent < -27 {
            (0, 24)
        } else if exponent < -14 {
            (0x400 >> (-exponent - 14), (-exponent - 1) as u32)
        } else if exponent <= 15 {
            (((exponent + 15) as u32) << 10, 13)
        } else if exponent < 128 {
            (0x7c00, 24)
        } else {
            (0x7c00, 13)
        };
        Self::from_bits((sign + base + (mantissa >> shift)) as u16)
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClampedU8(pub u8);
impl Component for ClampedU8 {
    fn decode(self, normalized: bool) -> f64 {
        self.0.decode(normalized)
    }
    fn encode(v: f64, normalized: bool) -> Self {
        // Uint8ClampedArray uses ties-to-even, unlike Math.round.
        let v = if normalized {
            (v * 255.0 + 0.5).floor()
        } else {
            v
        };
        Self(v.clamp(0.0, 255.0).round_ties_even() as u8)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Usage {
    #[default]
    Static,
    Dynamic,
    Stream,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GpuType {
    #[default]
    Float,
    Int,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRange {
    pub start: usize,
    pub count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BufferAttribute<T> {
    pub identity: crate::identity::Identity,
    #[serde(skip)]
    upload_hook: UploadHook,
    array: Vec<T>,
    item_size: usize,
    pub normalized: bool,
    pub name: String,
    pub usage: Usage,
    pub gpu_type: GpuType,
    version: u64,
    pub update_ranges: Vec<UpdateRange>,
}
impl<T: Component> BufferAttribute<T> {
    pub fn new(array: Vec<T>, item_size: usize, normalized: bool) -> Result<Self> {
        if item_size == 0 || !array.len().is_multiple_of(item_size) {
            return Err(Error::Invalid("attribute item size"));
        }
        Ok(Self {
            identity: Default::default(),
            upload_hook: Default::default(),
            array,
            item_size,
            normalized,
            name: String::new(),
            usage: Usage::Static,
            gpu_type: GpuType::Float,
            version: 0,
            update_ranges: Vec::new(),
        })
    }
    pub fn array(&self) -> &[T] {
        &self.array
    }
    pub fn array_mut(&mut self) -> &mut [T] {
        self.mark_dirty();
        &mut self.array
    }
    pub fn count(&self) -> usize {
        self.array.len() / self.item_size
    }
    pub fn item_size(&self) -> usize {
        self.item_size
    }
    pub fn version(&self) -> u64 {
        self.version
    }
    pub fn on_upload(&mut self, callback: impl Fn() + Send + Sync + 'static) {
        self.upload_hook = UploadHook(Some(std::sync::Arc::new(callback)));
    }
    pub fn notify_uploaded(&self) {
        if let Some(callback) = &self.upload_hook.0 {
            callback();
        }
    }
    pub fn mark_dirty(&mut self) {
        self.version = self.version.wrapping_add(1);
    }
    pub fn set_usage(&mut self, usage: Usage) {
        self.usage = usage;
    }
    pub fn add_update_range(&mut self, start: usize, count: usize) -> Result<()> {
        if start
            .checked_add(count)
            .is_none_or(|end| end > self.array.len())
        {
            return Err(Error::Invalid("update range"));
        }
        self.update_ranges.push(UpdateRange { start, count });
        Ok(())
    }
    pub fn clear_update_ranges(&mut self) {
        self.update_ranges.clear();
    }
    pub fn get_component(&self, index: usize, component: usize) -> Result<f64> {
        if index >= self.count() || component >= self.item_size {
            return Err(Error::Invalid("attribute index"));
        }
        Ok(self.array[index * self.item_size + component].decode(self.normalized))
    }
    pub fn set_component(&mut self, index: usize, component: usize, value: f64) -> Result<()> {
        if index >= self.count() || component >= self.item_size {
            return Err(Error::Invalid("attribute index"));
        }
        self.array[index * self.item_size + component] = T::encode(value, self.normalized);
        self.mark_dirty();
        Ok(())
    }
    pub fn set_tuple(&mut self, index: usize, value: &[f64]) -> Result<()> {
        if index >= self.count() || value.len() > self.item_size {
            return Err(Error::Invalid("attribute tuple"));
        }
        for (component, value) in value.iter().enumerate() {
            self.set_component(index, component, *value)?;
        }
        Ok(())
    }
    pub fn vector3(&self, index: usize) -> Result<Vector3> {
        Ok(Vector3::new(
            self.get_component(index, 0)?,
            self.get_component(index, 1)?,
            self.get_component(index, 2)?,
        ))
    }
    pub fn copy_at(&mut self, index: usize, other: &Self, source_index: usize) -> Result<()> {
        if self.item_size != other.item_size
            || index >= self.count()
            || source_index >= other.count()
        {
            return Err(Error::Invalid("attribute copy"));
        }
        let size = self.item_size;
        self.array[index * size..(index + 1) * size]
            .copy_from_slice(&other.array[source_index * size..(source_index + 1) * size]);
        self.mark_dirty();
        Ok(())
    }
    pub fn set(&mut self, values: &[T], offset: usize) -> Result<()> {
        let end = offset
            .checked_add(values.len())
            .ok_or(Error::Invalid("attribute range"))?;
        let target = self
            .array
            .get_mut(offset..end)
            .ok_or(Error::Invalid("attribute range"))?;
        target.copy_from_slice(values);
        self.mark_dirty();
        Ok(())
    }
    pub fn copy_array(&mut self, values: &[T]) -> Result<()> {
        self.set(values, 0)
    }
    pub fn copy_from(&mut self, source: &Self) {
        self.array = source.array.clone();
        self.item_size = source.item_size;
        self.normalized = source.normalized;
        self.usage = source.usage;
        self.name = source.name.clone();
        self.mark_dirty();
    }
    pub fn apply_matrix4(&mut self, m: Matrix4) -> Result<()> {
        self.map_vectors(|p| m.project_point3(p))
    }
    pub fn apply_matrix3(&mut self, m: Matrix3) -> Result<()> {
        if self.item_size == 2 {
            for i in 0..self.count() {
                let p = m * Vector3::new(self.get_component(i, 0)?, self.get_component(i, 1)?, 1.0);
                self.set_tuple(i, &[p.x, p.y])?;
            }
            return Ok(());
        }
        self.map_vectors(|p| m * p)
    }
    pub fn apply_normal_matrix(&mut self, m: Matrix3) -> Result<()> {
        self.map_vectors(|p| (m * p).normalize_or_zero())
    }
    pub fn transform_direction(&mut self, m: Matrix4) -> Result<()> {
        self.map_vectors(|p| m.transform_vector3(p).normalize_or_zero())
    }
    fn map_vectors(&mut self, mut f: impl FnMut(Vector3) -> Vector3) -> Result<()> {
        if self.item_size < 3 {
            return Err(Error::Invalid("vector attribute size"));
        }
        for i in 0..self.count() {
            let p = f(self.vector3(i)?);
            self.set_tuple(i, &p.to_array())?;
        }
        Ok(())
    }
    /// Releases owned CPU storage; GPU allocations follow their owning renderer resources.
    pub fn dispose(&mut self) {
        self.array = Vec::new();
        self.update_ranges.clear();
        self.mark_dirty();
    }
}

pub type Int8BufferAttribute = BufferAttribute<i8>;
pub type Uint8BufferAttribute = BufferAttribute<u8>;
pub type Uint8ClampedBufferAttribute = BufferAttribute<ClampedU8>;
pub type Int16BufferAttribute = BufferAttribute<i16>;
pub type Uint16BufferAttribute = BufferAttribute<u16>;
pub type Int32BufferAttribute = BufferAttribute<i32>;
pub type Uint32BufferAttribute = BufferAttribute<u32>;
pub type Float16BufferAttribute = BufferAttribute<half::f16>;
pub type Float32BufferAttribute = BufferAttribute<f32>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstancedBufferAttribute<T> {
    pub attribute: BufferAttribute<T>,
    pub mesh_per_attribute: std::num::NonZeroU32,
}

/// Interleaved views share one allocation, version and upload ranges. Cloning this
/// buffer copies its storage; cloning a view retains its shared backing buffer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InterleavedBuffer<T> {
    pub storage: BufferAttribute<T>,
}
impl<T: Component> InterleavedBuffer<T> {
    pub fn new(array: Vec<T>, stride: usize) -> Result<Self> {
        Ok(Self {
            storage: BufferAttribute::new(array, stride, false)?,
        })
    }
    pub fn stride(&self) -> usize {
        self.storage.item_size()
    }
    pub fn count(&self) -> usize {
        self.storage.count()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstancedInterleavedBuffer<T> {
    pub buffer: InterleavedBuffer<T>,
    pub mesh_per_attribute: std::num::NonZeroU32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InterleavedBufferAttribute<T> {
    pub data: Arc<RwLock<InterleavedBuffer<T>>>,
    item_size: usize,
    offset: usize,
    pub normalized: bool,
    pub name: String,
}
impl<T: Component> InterleavedBufferAttribute<T> {
    pub fn mark_dirty(&mut self) {
        self.data
            .write()
            .expect("interleaved buffer lock poisoned")
            .storage
            .mark_dirty();
    }
    pub fn set_tuple(&mut self, index: usize, values: &[f64]) -> Result<()> {
        if values.len() > self.item_size || index >= self.count() {
            return Err(Error::Invalid("interleaved tuple"));
        }
        for (component, value) in values.iter().enumerate() {
            self.set_component(index, component, *value)?;
        }
        Ok(())
    }
    pub fn version(&self) -> u64 {
        self.data
            .read()
            .expect("interleaved buffer lock poisoned")
            .storage
            .version()
    }
    pub fn vector3(&self, index: usize) -> Result<Vector3> {
        Ok(Vector3::new(
            self.get_component(index, 0)?,
            self.get_component(index, 1)?,
            self.get_component(index, 2)?,
        ))
    }
    pub fn notify_uploaded(&self) {
        self.data
            .read()
            .expect("interleaved buffer lock poisoned")
            .storage
            .notify_uploaded();
    }

    pub fn new(
        data: Arc<RwLock<InterleavedBuffer<T>>>,
        item_size: usize,
        offset: usize,
        normalized: bool,
    ) -> Result<Self> {
        if item_size == 0
            || offset.checked_add(item_size).is_none_or(|end| {
                end > data
                    .read()
                    .expect("interleaved buffer lock poisoned")
                    .stride()
            })
        {
            return Err(Error::Invalid("interleaved layout"));
        }
        Ok(Self {
            data,
            item_size,
            offset,
            normalized,
            name: String::new(),
        })
    }
    pub fn count(&self) -> usize {
        self.data
            .read()
            .expect("interleaved buffer lock poisoned")
            .count()
    }
    pub fn item_size(&self) -> usize {
        self.item_size
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
    pub fn get_component(&self, index: usize, component: usize) -> Result<f64> {
        let data = self.data.read().expect("interleaved buffer lock poisoned");
        if index >= data.count() || component >= self.item_size {
            return Err(Error::Invalid("interleaved index"));
        }
        Ok(
            data.storage.array[index * data.stride() + self.offset + component]
                .decode(self.normalized),
        )
    }
    pub fn set_component(&mut self, index: usize, component: usize, value: f64) -> Result<()> {
        let mut data = self.data.write().expect("interleaved buffer lock poisoned");
        let stride = data.stride();
        if index >= data.count() || component >= self.item_size {
            return Err(Error::Invalid("interleaved index"));
        }
        data.storage.array[index * stride + self.offset + component] =
            T::encode(value, self.normalized);
        data.storage.mark_dirty();
        Ok(())
    }
    pub fn to_attribute(&self) -> Result<BufferAttribute<T>> {
        let data = self.data.read().expect("interleaved buffer lock poisoned");
        let mut array = Vec::with_capacity(data.count() * self.item_size);
        for row in data.storage.array.chunks_exact(data.stride()) {
            array.extend_from_slice(&row[self.offset..self.offset + self.item_size]);
        }
        BufferAttribute::new(array, self.item_size, self.normalized)
    }
    pub fn apply_matrix4(&mut self, m: Matrix4) -> Result<()> {
        self.map_vectors(|p| m.project_point3(p))
    }
    pub fn apply_normal_matrix(&mut self, m: Matrix3) -> Result<()> {
        self.map_vectors(|p| (m * p).normalize_or_zero())
    }
    pub fn transform_direction(&mut self, m: Matrix4) -> Result<()> {
        self.map_vectors(|p| m.transform_vector3(p).normalize_or_zero())
    }
    fn map_vectors(&mut self, mut f: impl FnMut(Vector3) -> Vector3) -> Result<()> {
        if self.item_size < 3 {
            return Err(Error::Invalid("vector attribute size"));
        }
        for i in 0..self.count() {
            let p = f(Vector3::new(
                self.get_component(i, 0)?,
                self.get_component(i, 1)?,
                self.get_component(i, 2)?,
            ));
            for j in 0..3 {
                self.set_component(i, j, p[j])?;
            }
        }
        Ok(())
    }
}
