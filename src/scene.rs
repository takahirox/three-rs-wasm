//! Nodes belong to exactly one scene. Handles cannot revive after removal, nor
//! accidentally address a node in another scene.
use crate::{Error, Result, camera::Camera, geometry::BufferGeometry, material::Material, math::*};
use serde::Serialize;
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

static NEXT_SCENE: AtomicU32 = AtomicU32::new(1);
/// Opaque scene-scoped handle. Callers cannot fabricate handles.
///
/// ```compile_fail
/// use three_rs_wasm::scene::Object3D;
/// let forged = Object3D { scene: 1, index: 0, generation: 0 };
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Object3D {
    scene: u32,
    index: usize,
    generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Layers {
    pub mask: u32,
}
impl Default for Layers {
    fn default() -> Self {
        Self { mask: 1 }
    }
}
impl Layers {
    pub fn set(&mut self, channel: u32) {
        self.mask = 1_u32.wrapping_shl(channel);
    }
    pub fn enable(&mut self, channel: u32) {
        self.mask |= 1_u32.wrapping_shl(channel);
    }
    pub fn disable(&mut self, channel: u32) {
        self.mask &= !1_u32.wrapping_shl(channel);
    }
    pub fn toggle(&mut self, channel: u32) {
        self.mask ^= 1_u32.wrapping_shl(channel);
    }
    pub fn enable_all(&mut self) {
        self.mask = u32::MAX;
    }
    pub fn disable_all(&mut self) {
        self.mask = 0;
    }
    pub fn is_enabled(self, channel: u32) -> bool {
        self.mask & 1_u32.wrapping_shl(channel) != 0
    }
    pub fn test(self, other: Self) -> bool {
        self.mask & other.mask != 0
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Mesh {
    pub geometry: Arc<BufferGeometry>,
    pub materials: Vec<Arc<Material>>,
}
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Instance {
    pub matrix: Matrix4,
    pub color: Color,
}
impl Default for Instance {
    fn default() -> Self {
        Self {
            matrix: Matrix4::IDENTITY,
            color: Color::WHITE,
        }
    }
}
impl Mesh {
    pub fn new(geometry: Arc<BufferGeometry>, material: Arc<Material>) -> Self {
        Self {
            geometry,
            materials: vec![material],
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Line {
    pub geometry: Arc<BufferGeometry>,
    pub material: Arc<Material>,
    pub segments: bool,
}
#[derive(Clone, Debug, Serialize)]
pub struct Points {
    pub geometry: Arc<BufferGeometry>,
    pub material: Arc<Material>,
}
#[derive(Clone, Debug, Serialize)]
pub enum Light {
    /// Direction from world position toward the origin, with two stabilized cascades.
    Sun {
        color: Color,
        intensity: f64,
    },
    RectArea {
        color: Color,
        intensity: f64,
        width: f64,
        height: f64,
    },
    Hemisphere {
        sky: Color,
        ground: Color,
        intensity: f64,
    },
    Spot {
        color: Color,
        intensity: f64,
        target: Vector3,
        distance: f64,
        decay: f64,
        angle: f64,
        penumbra: f64,
    },
    Ambient {
        color: Color,
        intensity: f64,
    },
    Directional {
        color: Color,
        intensity: f64,
        target: Vector3,
    },
    Point {
        color: Color,
        intensity: f64,
        distance: f64,
        decay: f64,
    },
}
#[derive(Clone, Debug, Default, Serialize)]
pub enum NodeKind {
    #[default]
    Group,
    Mesh(Mesh),
    Line(Line),
    Points(Points),
    Camera(Camera),
    Light(Light),
}

type RenderCallback = Arc<dyn Fn(&mut Scene, Object3D) + Send + Sync>;
#[derive(Default)]
pub struct RenderHooks {
    pub before: Option<RenderCallback>,
    pub after: Option<RenderCallback>,
}
impl Clone for RenderHooks {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for RenderHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderHooks")
            .field("before", &self.before.is_some())
            .field("after", &self.after.is_some())
            .finish()
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Node {
    pub identity: crate::identity::Identity,
    #[serde(skip)]
    pub render_hooks: RenderHooks,
    pub name: String,
    pub kind: NodeKind,
    pub position: Vector3,
    pub quaternion: Quaternion,
    pub scale: Vector3,
    pub pivot: Option<Vector3>,
    pub up: Vector3,
    pub matrix: Matrix4,
    pub matrix_world: Matrix4,
    pub matrix_auto_update: bool,
    pub matrix_world_auto_update: bool,
    pub matrix_world_needs_update: bool,
    pub visible: bool,
    pub layers: Layers,
    pub frustum_culled: bool,
    pub render_order: i32,
    pub is_static: bool,
    pub cast_shadow: bool,
    pub receive_shadow: bool,
    pub shadow: crate::shadow::Shadow,
    pub morph_weights: Vec<f64>,
    pub skin: Option<crate::deformation::Skin>,
    /// Empty uses the ordinary mesh path. Instanced transforms must be invertible
    /// and have positive determinant (as with Three.js InstancedMesh).
    pub instances: Vec<Instance>,
    /// Active prefix of resident instances. None draws the full capacity.
    /// Changing this count does not mutate or re-upload the geometry.
    /// Direct draws only; indirect commands own their instance counts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<u32>,
    pub user_data: serde_json::Map<String, serde_json::Value>,
    #[serde(skip)]
    parent: Option<Object3D>,
    #[serde(skip)]
    children: Vec<Object3D>,
}
impl Default for Node {
    fn default() -> Self {
        Self {
            identity: Default::default(),
            render_hooks: Default::default(),
            name: String::new(),
            kind: NodeKind::Group,
            position: Vector3::ZERO,
            quaternion: Quaternion::IDENTITY,
            scale: Vector3::ONE,
            pivot: None,
            up: Vector3::Y,
            matrix: Matrix4::IDENTITY,
            matrix_world: Matrix4::IDENTITY,
            matrix_auto_update: true,
            matrix_world_auto_update: true,
            matrix_world_needs_update: false,
            visible: true,
            layers: Layers::default(),
            frustum_culled: true,
            render_order: 0,
            is_static: false,
            cast_shadow: false,
            receive_shadow: false,
            shadow: Default::default(),
            morph_weights: Vec::new(),
            skin: None,
            instances: Vec::new(),
            instance_count: None,
            user_data: Default::default(),
            parent: None,
            children: Vec::new(),
        }
    }
}
impl Node {
    pub(crate) fn draw_instance_count(&self, geometry: &BufferGeometry) -> Result<u32> {
        if self.instance_count.is_some()
            && (geometry.indirect.is_some() || geometry.gpu_indirect.is_some())
        {
            return Err(Error::Invalid(
                "instance count override requires direct drawing",
            ));
        }
        let capacity = if self.instances.is_empty() {
            geometry.instance_count.unwrap_or(1)
        } else {
            self.instances.len() as u32
        };
        let count = self.instance_count.unwrap_or(capacity);
        if count > capacity {
            return Err(Error::Invalid("instance count exceeds capacity"));
        }
        Ok(count)
    }
    pub fn parent(&self) -> Option<Object3D> {
        self.parent
    }
    pub fn children(&self) -> &[Object3D] {
        &self.children
    }
    pub fn update_matrix(&mut self) {
        self.matrix =
            Matrix4::from_scale_rotation_translation(self.scale, self.quaternion, self.position);
        if let Some(pivot) = self.pivot {
            self.matrix.w_axis += (pivot - self.matrix.transform_vector3(pivot)).extend(0.0);
        }
        self.matrix_world_needs_update = true;
    }
    pub fn apply_matrix4(&mut self, m: Matrix4) {
        if self.matrix_auto_update {
            self.update_matrix();
        }
        self.matrix = m * self.matrix;
        (self.scale, self.quaternion, self.position) = self.matrix.to_scale_rotation_translation();
        self.matrix_world_needs_update = true;
    }
    pub fn apply_quaternion(&mut self, q: Quaternion) {
        self.quaternion = q * self.quaternion;
    }
    pub fn rotate_on_axis(&mut self, axis: Vector3, angle: f64) {
        self.quaternion *= Quaternion::from_axis_angle(axis, angle);
    }
    pub fn rotate_on_world_axis(&mut self, axis: Vector3, angle: f64) {
        self.quaternion = Quaternion::from_axis_angle(axis, angle) * self.quaternion;
    }
    pub fn translate_on_axis(&mut self, axis: Vector3, distance: f64) {
        self.position += self.quaternion * axis * distance;
    }
    pub fn set_rotation_from_euler(&mut self, e: Euler) {
        self.quaternion = e.quaternion();
    }
    pub fn rotation(&self, order: EulerOrder) -> Euler {
        Euler::from_quaternion(self.quaternion, order)
    }
    pub fn world_position(&self) -> Vector3 {
        self.matrix_world.w_axis.truncate()
    }
    pub fn world_quaternion(&self) -> Quaternion {
        self.matrix_world.to_scale_rotation_translation().1
    }
    pub fn world_scale(&self) -> Vector3 {
        self.matrix_world.to_scale_rotation_translation().0
    }
    pub fn model_view_matrix(&self, camera_world: Matrix4) -> Matrix4 {
        camera_world.inverse() * self.matrix_world
    }
    pub fn normal_matrix(&self, camera_world: Matrix4) -> Matrix3 {
        Matrix3::from_mat4(self.model_view_matrix(camera_world))
            .inverse()
            .transpose()
    }
    pub fn world_direction(&self) -> Vector3 {
        let direction = self.matrix_world.z_axis.truncate().normalize();
        if matches!(self.kind, NodeKind::Camera(_)) {
            -direction
        } else {
            direction
        }
    }
    pub fn local_to_world(&self, p: Vector3) -> Vector3 {
        self.matrix_world.transform_point3(p)
    }
    pub fn world_to_local(&self, p: Vector3) -> Vector3 {
        self.matrix_world.inverse().transform_point3(p)
    }
    pub fn geometry(&self) -> Option<&Arc<BufferGeometry>> {
        match &self.kind {
            NodeKind::Mesh(m) => Some(&m.geometry),
            NodeKind::Line(l) => Some(&l.geometry),
            NodeKind::Points(p) => Some(&p.geometry),
            _ => None,
        }
    }
}
#[derive(Debug)]
struct Slot {
    generation: u64,
    node: Option<Node>,
}
/// Per-scene defaults replace mutable JavaScript class globals.
#[derive(Clone, Copy, Debug)]
pub struct NodeDefaults {
    pub up: Vector3,
    pub matrix_auto_update: bool,
    pub matrix_world_auto_update: bool,
}
impl Default for NodeDefaults {
    fn default() -> Self {
        Self {
            up: Vector3::Y,
            matrix_auto_update: true,
            matrix_world_auto_update: true,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize)]
pub enum Fog {
    Linear { color: Color, near: f64, far: f64 },
    Exp2 { color: Color, density: f64 },
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum ToneMapping {
    #[default]
    None = 0,
    Aces = 1,
    Reinhard = 2,
    Neutral = 3,
    Linear = 4,
}
/// Background values written to corresponding MRT attachments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BackgroundOutput {
    Color,
    NormalView,
    Zero,
}
#[derive(Debug)]
pub struct Scene {
    pub clipping_planes: Vec<Plane>,
    /// Include scene-wide clipping planes when rendering shadow maps.
    pub clipping_shadows: bool,
    pub shadow_map_size: u32,
    pub defaults: NodeDefaults,
    id: u32,
    pub(crate) cache_owner: Arc<()>,
    slots: Vec<Slot>,
    free: Vec<usize>,
    pub background: Color,
    /// Alpha used when clearing a solid background. RGB is premultiplied by it.
    pub background_alpha: f64,
    pub environment: Option<Arc<crate::environment::EnvironmentMap>>,
    pub environment_intensity: f64,
    pub environment_rotation: f64,
    pub background_environment: bool,
    pub background_outputs: Vec<BackgroundOutput>,
    pub background_blur: f64,
    pub fog: Option<Fog>,
    /// Sample the equirectangular source directly instead of cube conversion.
    pub background_equirectangular: bool,
    pub background_intensity: f64,
    pub exposure: f64,
    pub aces_tone_mapping: bool,
    pub tone_mapping: ToneMapping,
}
impl Default for Scene {
    fn default() -> Self {
        Self {
            shadow_map_size: 512,
            clipping_planes: Vec::new(),
            clipping_shadows: true,
            defaults: Default::default(),
            id: NEXT_SCENE.fetch_add(1, Ordering::Relaxed),
            cache_owner: Arc::new(()),
            slots: Vec::new(),
            free: Vec::new(),
            background: Color::BLACK,
            background_alpha: 1.0,
            environment: None,
            environment_intensity: 1.0,
            environment_rotation: 0.0,
            background_environment: false,
            background_outputs: Vec::new(),
            background_blur: 0.0,
            fog: None,
            background_equirectangular: false,
            background_intensity: 1.0,
            exposure: 1.0,
            aces_tone_mapping: false,
            tone_mapping: ToneMapping::None,
        }
    }
}
impl Scene {
    pub(crate) fn cache_id(&self) -> u32 {
        self.id
    }
    pub fn output_tone_mapping(&self) -> ToneMapping {
        if self.tone_mapping != ToneMapping::None {
            self.tone_mapping
        } else if self.aces_tone_mapping {
            ToneMapping::Aces
        } else {
            ToneMapping::None
        }
    }

    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, kind: NodeKind) -> Object3D {
        self.insert_node(Node {
            kind,
            up: self.defaults.up,
            matrix_auto_update: self.defaults.matrix_auto_update,
            matrix_world_auto_update: self.defaults.matrix_world_auto_update,
            ..Default::default()
        })
    }
    pub fn insert_node(&mut self, mut node: Node) -> Object3D {
        node.parent = None;
        node.children.clear();
        let index = if let Some(i) = self.free.pop() {
            self.slots[i].node = Some(node);
            i
        } else {
            self.slots.push(Slot {
                generation: 0,
                node: Some(node),
            });
            self.slots.len() - 1
        };
        Object3D {
            scene: self.id,
            index,
            generation: self.slots[index].generation,
        }
    }
    pub fn get(&self, h: Object3D) -> Result<&Node> {
        let slot = self
            .slots
            .get(h.index)
            .filter(|s| h.scene == self.id && s.generation == h.generation)
            .ok_or(Error::InvalidHandle)?;
        slot.node.as_ref().ok_or(Error::InvalidHandle)
    }
    /// Node borrows prevent graph mutation while their references are in use.
    ///
    /// ```compile_fail
    /// use three_rs_wasm::scene::{Scene, NodeKind};
    /// let mut scene = Scene::new();
    /// let handle = scene.insert(NodeKind::Group);
    /// let node = scene.get(handle).unwrap();
    /// scene.dispose(handle).unwrap();
    /// println!("{}", node.name);
    /// ```
    pub fn get_mut(&mut self, h: Object3D) -> Result<&mut Node> {
        let slot = self
            .slots
            .get_mut(h.index)
            .filter(|s| h.scene == self.id && s.generation == h.generation)
            .ok_or(Error::InvalidHandle)?;
        slot.node.as_mut().ok_or(Error::InvalidHandle)
    }
    pub fn len(&self) -> usize {
        self.slots.len() - self.free.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn allocated_slots(&self) -> usize {
        self.slots.len()
    }
    pub fn handles(&self) -> impl Iterator<Item = Object3D> + '_ {
        self.slots
            .iter()
            .enumerate()
            .filter(|(_, s)| s.node.is_some())
            .map(|(index, s)| Object3D {
                scene: self.id,
                index,
                generation: s.generation,
            })
    }
    pub fn roots(&self) -> Vec<Object3D> {
        self.handles()
            .filter(|h| self.get(*h).is_ok_and(|n| n.parent.is_none()))
            .collect()
    }
    pub fn add(&mut self, parent: Object3D, child: Object3D) -> Result<()> {
        self.get(child)?;
        self.get(parent)?;
        let mut ancestor = Some(parent);
        while let Some(a) = ancestor {
            if a == child {
                return Err(Error::Cycle);
            }
            ancestor = self.get(a)?.parent;
        }
        self.remove_from_parent(child)?;
        self.get_mut(parent)?.children.push(child);
        self.get_mut(child)?.parent = Some(parent);
        Ok(())
    }
    pub fn remove_from_parent(&mut self, child: Object3D) -> Result<()> {
        if let Some(parent) = self.get(child)?.parent {
            self.get_mut(parent)?.children.retain(|h| *h != child);
            self.get_mut(child)?.parent = None;
        }
        Ok(())
    }
    pub fn remove(&mut self, parent: Object3D, child: Object3D) -> Result<()> {
        self.get(parent)?;
        if self.get(child)?.parent == Some(parent) {
            self.remove_from_parent(child)?;
        }
        Ok(())
    }
    pub fn clear(&mut self, parent: Object3D) -> Result<()> {
        for child in self.get(parent)?.children.clone() {
            self.remove_from_parent(child)?;
        }
        Ok(())
    }
    /// Drop an entire subtree and release its resource references.
    pub fn dispose(&mut self, root: Object3D) -> Result<()> {
        let handles = self.traverse(root, false)?;
        self.remove_from_parent(root)?;
        for h in handles.into_iter().rev() {
            let slot = &mut self.slots[h.index];
            slot.node = None;
            slot.generation = slot
                .generation
                .checked_add(1)
                .expect("node generation exhausted");
            self.free.push(h.index);
        }
        Ok(())
    }
    fn update_node_world(&mut self, h: Object3D, force: bool) -> Result<bool> {
        let parent_world = self
            .get(h)?
            .parent
            .map(|p| self.get(p).map(|n| n.matrix_world))
            .transpose()?
            .unwrap_or(Matrix4::IDENTITY);
        let node = self.get_mut(h)?;
        if node.matrix_auto_update {
            node.update_matrix();
        }
        let update = node.matrix_world_needs_update || force;
        if update {
            if node.matrix_world_auto_update {
                node.matrix_world = parent_world * node.matrix;
            }
            node.matrix_world_needs_update = false;
        }
        Ok(update)
    }
    pub fn update_matrix_world(&mut self, root: Object3D, force: bool) -> Result<()> {
        let mut stack = vec![(root, force)];
        while let Some((h, force)) = stack.pop() {
            let update = self.update_node_world(h, force)?;
            stack.extend(self.get(h)?.children.iter().rev().map(|h| (*h, update)));
        }
        Ok(())
    }
    pub fn update_world_matrix(
        &mut self,
        h: Object3D,
        parents: bool,
        children: bool,
    ) -> Result<()> {
        self.update_world_matrix_force(h, parents, children, false)
    }
    pub fn update_world_matrix_force(
        &mut self,
        h: Object3D,
        parents: bool,
        children: bool,
        force: bool,
    ) -> Result<()> {
        if parents {
            for parent in self.ancestors(h)?.into_iter().rev() {
                self.update_node_world(parent, false)?;
            }
        }
        if children {
            self.update_matrix_world(h, force)
        } else {
            self.update_node_world(h, force)?;
            Ok(())
        }
    }
    pub fn update(&mut self) -> Result<()> {
        for root in self.roots() {
            self.update_matrix_world(root, false)?;
        }
        Ok(())
    }
    pub fn attach(&mut self, parent: Object3D, child: Object3D) -> Result<()> {
        // Validate the relationship before mutating either graph or transforms.
        self.get(child)?;
        let mut p = Some(parent);
        while let Some(h) = p {
            if h == child {
                return Err(Error::Cycle);
            }
            p = self.get(h)?.parent;
        }
        self.update_world_matrix(parent, true, false)?;
        self.update_world_matrix(child, true, false)?;
        let parent_matrix = self.get(parent)?.matrix_world;
        if parent_matrix.determinant() == 0.0 {
            return Err(Error::Invalid("singular parent transform"));
        }
        let old_parent = self
            .get(child)?
            .parent
            .map(|h| self.get(h).map(|n| n.matrix_world))
            .transpose()?
            .unwrap_or(Matrix4::IDENTITY);
        self.get_mut(child)?
            .apply_matrix4(parent_matrix.inverse() * old_parent);
        self.add(parent, child)?;
        self.update_world_matrix(child, false, true)
    }
    pub fn traverse(&self, root: Object3D, visible_only: bool) -> Result<Vec<Object3D>> {
        let mut result = Vec::new();
        let mut stack = vec![root];
        while let Some(h) = stack.pop() {
            let n = self.get(h)?;
            if visible_only && !n.visible {
                continue;
            }
            result.push(h);
            stack.extend(n.children.iter().rev());
        }
        Ok(result)
    }
    pub fn ancestors(&self, h: Object3D) -> Result<Vec<Object3D>> {
        let mut result = Vec::new();
        let mut p = self.get(h)?.parent;
        while let Some(h) = p {
            result.push(h);
            p = self.get(h)?.parent;
        }
        Ok(result)
    }
    pub fn find(&self, root: Object3D, predicate: impl Fn(&Node) -> bool) -> Result<Vec<Object3D>> {
        Ok(self
            .traverse(root, false)?
            .into_iter()
            .filter(|h| predicate(self.get(*h).expect("validated handle")))
            .collect())
    }
    pub fn get_object_by_name(&self, root: Object3D, name: &str) -> Result<Option<Object3D>> {
        Ok(self.find(root, |n| n.name == name)?.first().copied())
    }
    pub fn get_object_by_id(&self, root: Object3D, id: u32) -> Result<Option<Object3D>> {
        Ok(self
            .find(root, |node| node.identity.id == id)?
            .first()
            .copied())
    }
    pub fn clone_subtree(&mut self, root: Object3D, recursive: bool) -> Result<Object3D> {
        let source = self.get(root)?.clone();
        let children = source.children.clone();
        let copy = self.insert_node(source);
        if recursive {
            for child in children {
                let clone = self.clone_subtree(child, true)?;
                self.add(copy, clone)?;
            }
        }
        Ok(copy)
    }
    pub fn copy_node(&mut self, target: Object3D, source: Object3D, recursive: bool) -> Result<()> {
        let mut copy = self.get(source)?.clone();
        let source_children = copy.children.clone();
        let target_node = self.get_mut(target)?;
        copy.identity = std::mem::take(&mut target_node.identity);
        copy.render_hooks = std::mem::take(&mut target_node.render_hooks);
        copy.parent = target_node.parent;
        copy.children = std::mem::take(&mut target_node.children);
        *target_node = copy;
        if recursive {
            for child in source_children {
                let new_child = self.clone_subtree(child, true)?;
                self.add(target, new_child)?;
            }
        }
        Ok(())
    }
    pub fn look_at(&mut self, h: Object3D, target: Vector3) -> Result<()> {
        self.update_world_matrix(h, true, false)?;
        let node = self.get(h)?;
        let position = node.world_position();
        let z = if matches!(node.kind, NodeKind::Camera(_) | NodeKind::Light(_)) {
            position - target
        } else {
            target - position
        };
        let mut z = if z.length_squared() == 0.0 {
            Vector3::Z
        } else {
            z.normalize()
        };
        let mut x = node.up.cross(z);
        if x.length_squared() == 0.0 {
            if node.up.z.abs() == 1.0 {
                z.x += 0.0001;
            } else {
                z.z += 0.0001;
            }
            z = z.normalize();
            x = node.up.cross(z);
        }
        x = x.normalize();
        let y = z.cross(x);
        let mut q = Quaternion::from_mat3(&Matrix3::from_cols(x, y, z));
        if let Some(parent) = node.parent {
            let m = self.get(parent)?.matrix_world;
            let rotation = Matrix3::from_cols(
                m.x_axis.truncate().normalize(),
                m.y_axis.truncate().normalize(),
                m.z_axis.truncate().normalize(),
            );
            q = Quaternion::from_mat3(&rotation).inverse() * q;
        }
        self.get_mut(h)?.quaternion = q;
        Ok(())
    }
    pub fn camera(&self, h: Object3D) -> Result<(&Camera, Matrix4)> {
        let n = self.get(h)?;
        match &n.kind {
            NodeKind::Camera(c) => Ok((c, n.matrix_world)),
            _ => Err(Error::Invalid("camera node")),
        }
    }
    /// Stable traversal data without arena internals or process-specific handles.
    pub fn to_json(&self) -> Result<serde_json::Value> {
        fn encode(scene: &Scene, h: Object3D) -> Result<serde_json::Value> {
            let n = scene.get(h)?;
            let mut value = serde_json::to_value(n).map_err(|e| Error::Asset(e.to_string()))?;
            let object = value.as_object_mut().expect("node object");
            object.remove("parent");
            object.insert(
                "children".into(),
                serde_json::Value::Array(
                    n.children
                        .iter()
                        .map(|h| encode(scene, *h))
                        .collect::<Result<_>>()?,
                ),
            );
            Ok(value)
        }
        Ok(
            serde_json::json!({"background":self.background,"children":self.roots().iter().map(|h|encode(self,*h)).collect::<Result<Vec<_>>>()?}),
        )
    }
}
