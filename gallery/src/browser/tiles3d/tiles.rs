//! 3d-tiles-renderer 0.4.27's TilesRendererBase and three TilesRenderer, with
//! the plugins the page registers that change which tiles load and show:
//! CesiumIonAuthPlugin ( and the GoogleCloudAuthPlugin it hands the Google
//! asset to ), UpdateOnChangePlugin and TilesFadePlugin's visibility hooks.
//!
//! Tiles live in an arena. A tile's JSON children become tiles when
//! ensureChildrenArePreprocessed processes them, as preprocessNode does.
//! Downloads and parses are futures polled once per frame. The priority
//! queues start their jobs on the frame after they are scheduled, as their
//! requestAnimationFrame callbacks do.
use super::volume::{Frustum, M4, V3, Volume, apply4, multiply};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub(crate) type TileId = usize;

pub(crate) const FAILED: i32 = -1;
pub(crate) const UNLOADED: i32 = 0;
pub(crate) const QUEUED: i32 = 1;
pub(crate) const LOADING: i32 = 2;
pub(crate) const PARSING: i32 = 3;
pub(crate) const LOADED: i32 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Refine {
    Replace,
    Add,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Traversal {
    pub distance_from_camera: f64,
    pub error: f64,
    pub in_frustum: bool,
    pub is_leaf: bool,
    pub used: bool,
    pub used_last_frame: bool,
    pub visible: bool,
    pub was_set_visible: bool,
    pub active: bool,
    pub was_set_active: bool,
    pub all_children_ready: bool,
    pub all_children_loaded: bool,
    pub kicked: bool,
    pub all_used_children_processed: bool,
    pub last_frame_visited: i64,
}

pub(crate) struct Tile {
    pub json: serde_json::Value,
    pub parent: Option<TileId>,
    pub children: Vec<TileId>,
    pub content_uri: Option<String>,
    pub geometric_error: f64,
    pub refine: Refine,
    // internal
    pub has_content: bool,
    pub has_renderable: bool,
    pub has_unrenderable: bool,
    pub loading_state: i32,
    pub base_path: String,
    pub depth: i64,
    pub depth_from_rendered_parent: i64,
    pub virtual_child_count: usize,
    pub traversal: Option<Traversal>,
    // engineData
    pub transform: M4,
    pub volume: Volume,
    pub scene: Option<Rc<super::content::TileContent>>,
    pub external: bool,
    pub priority: Option<f64>,
}

/// A queue job's state.
pub(crate) struct Pending {
    pub abort: Rc<RefCell<bool>>,
    pub url: String,
    pub extension: Option<String>,
}

/// PriorityQueue: items with a callback each, the highest priority popped last.
#[derive(Default)]
pub(crate) struct Queue {
    pub max_jobs: usize,
    pub items: Vec<TileId>,
    pub callbacks: HashSet<TileId>,
    pub curr_jobs: usize,
    pub scheduled: bool,
}
impl Queue {
    fn new(max_jobs: usize) -> Self {
        Queue {
            max_jobs,
            ..Default::default()
        }
    }
    pub fn running(&self) -> bool {
        !self.items.is_empty() || self.curr_jobs != 0
    }
    fn has(&self, t: TileId) -> bool {
        self.callbacks.contains(&t)
    }
    fn add(&mut self, t: TileId) {
        self.items.insert(0, t);
        self.callbacks.insert(t);
        self.scheduled = true;
    }
    fn remove(&mut self, t: TileId) {
        if let Some(i) = self.items.iter().position(|&x| x == t) {
            self.items.remove(i);
            self.callbacks.remove(&t);
        }
    }
}

/// A future's result, once it lands.
pub(crate) type Slot<T> = Rc<RefCell<Option<Result<T, String>>>>;

/// The in-flight work of one tile: its download, then its parse.
pub(crate) enum Job {
    Download(Slot<Vec<u8>>),
    Parse(Slot<Option<super::content::TileContent>>),
}

#[derive(Default)]
pub(crate) struct Stats {
    pub in_cache_since_load: i64,
    pub in_cache: i64,
    pub queued: i64,
    pub downloading: i64,
    pub parsing: i64,
    pub loaded: i64,
    pub failed: i64,
    pub in_frustum: i64,
    pub used: i64,
    pub active: i64,
    pub visible: i64,
    pub tiles_processed: i64,
}

/// The LRUCache, with its item list order, used / loaded sets and bytes.
pub(crate) struct Lru {
    pub min_size: f64,
    pub max_size: f64,
    pub min_bytes: f64,
    pub max_bytes: f64,
    pub unload_percent: f64,
    pub items: Vec<TileId>,
    pub item_set: HashSet<TileId>,
    pub used: HashSet<TileId>,
    pub loaded: HashSet<TileId>,
    pub bytes: HashMap<TileId, f64>,
    pub cached_bytes: f64,
    pub rerun: bool,
}
impl Default for Lru {
    fn default() -> Self {
        let gb = 2f64.powi(30);
        Lru {
            min_size: 6000.,
            max_size: 8000.,
            min_bytes: 0.3 * gb,
            max_bytes: 0.4 * gb,
            unload_percent: 0.05,
            items: vec![],
            item_set: HashSet::new(),
            used: HashSet::new(),
            loaded: HashSet::new(),
            bytes: HashMap::new(),
            cached_bytes: 0.,
            rerun: false,
        }
    }
}
impl Lru {
    pub fn is_full(&self) -> bool {
        self.item_set.len() as f64 >= self.max_size || self.cached_bytes >= self.max_bytes
    }
    fn set_memory(&mut self, t: TileId, bytes: f64) {
        if !self.item_set.contains(&t) {
            return;
        }
        self.cached_bytes -= self.bytes.get(&t).copied().unwrap_or(0.);
        self.bytes.insert(t, bytes);
        self.cached_bytes += bytes;
    }
    fn mark_used(&mut self, t: TileId) {
        if self.item_set.contains(&t) && !self.used.contains(&t) {
            self.used.insert(t);
        }
    }
}

/// The renderer's camera: matrices in three's WebGL conventions and the
/// resolution setResolutionFromRenderer gives.
#[derive(Clone)]
pub(crate) struct CameraState {
    pub world: M4,
    pub world_inverse: M4,
    pub projection: M4,
    pub projection_inverse: M4,
    pub resolution: (f64, f64),
}

pub(crate) struct CameraInfo {
    pub frustum: Frustum,
    pub sse_denominator: f64,
    pub position: V3,
}

/// What the page's plugins and the renderer need from a frame's update.
#[derive(Default)]
pub(crate) struct Events {
    pub visibility: Vec<(TileId, bool)>,
    pub disposed: Vec<(TileId, Rc<super::content::TileContent>)>,
    pub loaded: Vec<TileId>,
}

pub(crate) struct Tiles {
    pub tiles: Vec<Tile>,
    pub root: Option<TileId>,
    pub root_url: String,
    pub root_loading: i32,
    pub error_target: f64,
    pub max_depth: f64,
    pub load_siblings: bool,
    pub load_ancestors: bool,
    pub display_active_tiles: bool,
    pub max_tiles_processed: i64,
    pub frame_count: i64,
    pub stats: Stats,
    pub lru: Lru,
    pub download_queue: Queue,
    pub parse_queue: Queue,
    pub process_queue: Queue,
    pub queued_tiles: Vec<TileId>,
    pub used_set: HashSet<TileId>,
    pub loading_tiles: HashSet<TileId>,
    pub visible_tiles: HashSet<TileId>,
    pub active_tiles: HashSet<TileId>,
    pub cached_since_load: HashSet<TileId>,
    pub is_loading: bool,
    pub pending: HashMap<TileId, Pending>,
    pub jobs: HashMap<TileId, Job>,
    /// Downloaded bytes waiting in the parse queue.
    pub parse_input: HashMap<TileId, Vec<u8>>,
    pub camera: Option<CameraState>,
    pub cameras: Vec<CameraInfo>,
    /// GLTF up axis: rotation +π/2 about X.
    pub up_rotation: M4,
    /// GoogleCloudAuth: the API key and the session read from the root tileset.
    pub google_key: Option<String>,
    pub session: Option<String>,
    /// UpdateOnChangePlugin.
    pub needs_update: bool,
    camera_matrix: Option<M4>,
    pub needs_update_throttled: bool,
    pub events: Events,
    /// TilesFadePlugin's setTileVisible answer, consulted before the renderer's.
    pub fade: super::fade::Fade,
}

fn extension(url: &str) -> Option<String> {
    let mut end = url.len();
    if let Some(q) = url.find('?') {
        end = end.min(q);
    }
    if let Some(f) = url.find('#') {
        end = end.min(f);
    }
    let head = &url[..end];
    let period = head.rfind('.')?;
    let slash = head.rfind('/');
    let protocol = url.find("://");
    let host_only = matches!((protocol, slash), (Some(p), Some(s)) if p + 2 == s);
    if host_only || slash.is_some_and(|s| period < s) {
        return None;
    }
    let ext = &head[period + 1..];
    (!ext.is_empty()).then(|| ext.to_owned())
}

/// new URL( url, base ).toString().
pub(crate) fn resolve(url: &str, base: &str) -> String {
    web_sys::Url::new_with_base(url, base)
        .map(|u| u.href())
        .unwrap_or_else(|_| url.to_owned())
}

impl Tiles {
    pub fn new(root_url: String) -> Self {
        let mut download_queue = Queue::new(25);
        download_queue.max_jobs = 25;
        Tiles {
            tiles: vec![],
            root: None,
            root_url,
            root_loading: UNLOADED,
            error_target: 16.,
            max_depth: f64::INFINITY,
            load_siblings: true,
            load_ancestors: true,
            display_active_tiles: false,
            max_tiles_processed: 250,
            frame_count: 0,
            stats: Stats::default(),
            lru: Lru::default(),
            download_queue,
            parse_queue: Queue::new(5),
            process_queue: Queue::new(25),
            queued_tiles: vec![],
            used_set: HashSet::new(),
            loading_tiles: HashSet::new(),
            visible_tiles: HashSet::new(),
            active_tiles: HashSet::new(),
            cached_since_load: HashSet::new(),
            is_loading: false,
            pending: HashMap::new(),
            jobs: HashMap::new(),
            parse_input: HashMap::new(),
            camera: None,
            cameras: vec![],
            up_rotation: super::ellipsoid::rotation_from_euler(
                std::f64::consts::PI / 2.,
                0.,
                0.,
                "XYZ",
            ),
            google_key: None,
            session: None,
            needs_update: true,
            camera_matrix: None,
            needs_update_throttled: false,
            events: Events::default(),
            fade: super::fade::Fade::default(),
        }
    }

    fn processed(&self, t: TileId) -> bool {
        self.tiles[t].traversal.is_some()
    }
    fn tr(&self, t: TileId) -> &Traversal {
        self.tiles[t].traversal.as_ref().expect("processed tile")
    }
    fn trm(&mut self, t: TileId) -> &mut Traversal {
        self.tiles[t].traversal.as_mut().expect("processed tile")
    }

    /// A JSON tile as an unprocessed arena tile.
    fn add_tile(&mut self, json: serde_json::Value) -> TileId {
        let id = self.tiles.len();
        self.tiles.push(Tile {
            json,
            parent: None,
            children: vec![],
            content_uri: None,
            geometric_error: 0.,
            refine: Refine::Replace,
            has_content: false,
            has_renderable: false,
            has_unrenderable: false,
            loading_state: UNLOADED,
            base_path: String::new(),
            depth: -1,
            depth_from_rendered_parent: -1,
            virtual_child_count: 0,
            traversal: None,
            transform: super::volume::IDENTITY,
            volume: Volume::default(),
            scene: None,
            external: false,
            priority: None,
        });
        // The JSON children are tiles from the start; their state waits for
        // preprocessNode.
        let children: Vec<serde_json::Value> = self.tiles[id].json["children"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let ids: Vec<TileId> = children.into_iter().map(|c| self.add_tile(c)).collect();
        for &c in &ids {
            self.tiles[c].parent = Some(id);
        }
        self.tiles[id].children = ids;
        id
    }

    /// preprocessNode ( base, then three's transform and bounding volume ).
    fn preprocess_node(&mut self, t: TileId, base_path: &str, parent: Option<TileId>) {
        self.stats.tiles_processed += 1;
        let json = &self.tiles[t].json;
        let uri = json["content"]["uri"]
            .as_str()
            .or(json["content"]["url"].as_str())
            .map(str::to_owned);
        let geometric_error = json["geometricError"].as_f64().unwrap_or(0.);
        let refine = match json["refine"].as_str() {
            Some("ADD") | Some("add") => Some(Refine::Add),
            Some(_) => Some(Refine::Replace),
            None => None,
        };
        let transform_json: Option<Vec<f64>> = json["transform"]
            .as_array()
            .map(|a| a.iter().filter_map(|v| v.as_f64()).collect());
        let volume_json = json["boundingVolume"].clone();
        let tile = &mut self.tiles[t];
        tile.parent = parent;
        tile.base_path = base_path.to_owned();
        tile.geometric_error = geometric_error;
        if let Some(uri) = &uri {
            let ext = extension(uri);
            let unrenderable = ext.as_deref().is_some_and(|e| e.ends_with("json"));
            tile.has_content = true;
            tile.has_unrenderable = unrenderable;
            tile.has_renderable = !unrenderable;
        }
        tile.content_uri = uri;
        let (depth, from_rendered, parent_refine, parent_transform) = match parent {
            Some(p) => {
                let p = &self.tiles[p];
                (
                    p.depth + 1,
                    p.depth_from_rendered_parent,
                    Some(p.refine),
                    p.transform,
                )
            }
            None => (0, 0, None, super::volume::IDENTITY),
        };
        let tile = &mut self.tiles[t];
        tile.depth = depth;
        tile.depth_from_rendered_parent = from_rendered + i64::from(tile.has_renderable);
        tile.traversal = Some(Traversal {
            distance_from_camera: f64::INFINITY,
            error: f64::INFINITY,
            last_frame_visited: -1,
            ..Default::default()
        });
        tile.refine = refine.or(parent_refine).unwrap_or(Refine::Replace);
        let mut transform = super::volume::IDENTITY;
        if let Some(a) = transform_json
            && a.len() == 16
        {
            transform.copy_from_slice(&a);
        }
        if parent.is_some() {
            transform = multiply(&parent_transform, &transform);
        }
        tile.transform = transform;
        let mut volume = Volume::default();
        if let Some(s) = volume_json["sphere"].as_array() {
            let d: Vec<f64> = s.iter().filter_map(|v| v.as_f64()).collect();
            volume.set_sphere(&d, &transform);
        }
        if let Some(b) = volume_json["box"].as_array() {
            let d: Vec<f64> = b.iter().filter_map(|v| v.as_f64()).collect();
            volume.set_box(&d, &transform);
        }
        tile.volume = volume;
        // UpdateOnChangePlugin.preprocessNode.
        self.needs_update = true;
    }

    /// ensureChildrenArePreprocessed.
    fn ensure_children(&mut self, t: TileId, force: Option<bool>) {
        let force = force.unwrap_or(self.stats.tiles_processed < self.max_tiles_processed);
        let children = self.tiles[t].children.clone();
        if children.is_empty() || self.processed(*children.last().expect("children")) {
            return;
        }
        if force {
            self.process_queue.remove(t);
            let base = self.tiles[t].base_path.clone();
            for c in children {
                if !self.processed(c) {
                    self.preprocess_node(c, &base, Some(t));
                }
            }
        } else if !self.process_queue.has(t) {
            self.process_queue.add(t);
        }
    }

    /// preprocessTileset( json, url, parent ): the base path and the root node.
    fn preprocess_tileset(
        &mut self,
        json: serde_json::Value,
        url: &str,
        parent: Option<TileId>,
    ) -> TileId {
        let base = match url.rfind('/') {
            Some(i) => &url[..i],
            None => url,
        };
        let base = resolve(
            base,
            &web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.base_uri().ok().flatten())
                .unwrap_or_default(),
        );
        let root = self.add_tile(json["root"].clone());
        self.preprocess_node(root, &base, parent);
        root
    }

    /// The root tileset's JSON arrived ( GoogleCloudAuth read its session ).
    pub fn set_root(&mut self, json: serde_json::Value, url: &str) {
        let root = self.preprocess_tileset(json, url, None);
        self.root = Some(root);
        self.root_loading = LOADED;
        self.needs_update = true;
    }

    /// prepareForTraversal for the one camera ( the group is the identity ).
    fn prepare(&mut self) {
        let Some(c) = &self.camera else {
            self.cameras.clear();
            return;
        };
        let sse = (2. / c.projection[5]) / c.resolution.1;
        let m = multiply(
            &c.projection,
            &multiply(&c.world_inverse, &super::volume::IDENTITY),
        );
        let frustum = Frustum::from_projection(&m);
        let position = apply4(apply4([0., 0., 0.], &c.world), &super::volume::IDENTITY);
        self.cameras = vec![CameraInfo {
            frustum,
            sse_denominator: sse,
            position,
        }];
    }

    /// calculateTileViewError ( no region plugin ).
    fn view_error(&self, t: TileId) -> (bool, f64, f64) {
        let tile = &self.tiles[t];
        let mut in_view = false;
        let (mut in_view_error, mut in_view_distance) = (0., f64::INFINITY);
        let (mut max_error, mut min_distance) = (0f64, f64::INFINITY);
        for info in &self.cameras {
            let distance = tile.volume.distance_to_point(info.position);
            let error = if distance == 0. {
                f64::INFINITY
            } else {
                tile.geometric_error / (distance * info.sse_denominator)
            };
            if tile.volume.intersects_frustum(&info.frustum) {
                in_view = true;
                in_view_error = f64::max(in_view_error, error);
                in_view_distance = f64::min(in_view_distance, distance);
            }
            max_error = max_error.max(error);
            min_distance = min_distance.min(distance);
        }
        if in_view {
            (true, in_view_error, in_view_distance)
        } else {
            (false, max_error, min_distance)
        }
    }

    // ---- optimizedTraverseFunctions ----

    fn is_download_finished(state: i32) -> bool {
        state == LOADED || state == FAILED
    }
    fn used_this_frame(&self, t: TileId) -> bool {
        self.processed(t) && self.tr(t).last_frame_visited == self.frame_count && self.tr(t).used
    }
    fn children_processed(&self, t: TileId) -> bool {
        let tile = &self.tiles[t];
        let children_ready =
            tile.children.is_empty() || self.processed(*tile.children.last().expect("children"));
        let content_ready =
            !tile.has_unrenderable || Self::is_download_finished(tile.loading_state);
        children_ready && content_ready
    }
    fn can_unconditionally_refine(&self, t: TileId) -> bool {
        let tile = &self.tiles[t];
        tile.has_unrenderable
            || tile
                .parent
                .is_some_and(|p| self.tiles[p].geometric_error < tile.geometric_error)
    }
    fn reset_frame_state(&mut self, t: TileId) {
        self.ensure_children(t, None);
        if self.tr(t).last_frame_visited != self.frame_count {
            let frame = self.frame_count;
            let (in_view, error, distance) = self.view_error(t);
            let tr = self.trm(t);
            tr.last_frame_visited = frame;
            tr.used = false;
            tr.in_frustum = false;
            tr.is_leaf = false;
            tr.visible = false;
            tr.active = false;
            tr.error = f64::INFINITY;
            tr.distance_from_camera = f64::INFINITY;
            tr.all_children_ready = false;
            tr.all_children_loaded = false;
            tr.kicked = false;
            tr.all_used_children_processed = false;
            tr.in_frustum = in_view;
            tr.error = error;
            tr.distance_from_camera = distance;
        }
    }
    fn recursively_mark_used(&mut self, t: TileId, cache_only: bool) {
        self.reset_frame_state(t);
        if cache_only {
            self.mark_tile_used(t);
        } else {
            self.trm(t).used = true;
        }
        if self.can_unconditionally_refine(t) && self.children_processed(t) {
            for c in self.tiles[t].children.clone() {
                self.recursively_mark_used(c, cache_only);
            }
        }
    }
    fn recursively_mark_previously_used(&mut self, t: TileId) {
        self.reset_frame_state(t);
        if self.tr(t).used_last_frame {
            self.trm(t).used = true;
            if self.tr(t).was_set_active {
                self.trm(t).active = true;
            }
            if (!self.tr(t).active || self.can_unconditionally_refine(t))
                && self.children_processed(t)
            {
                for c in self.tiles[t].children.clone() {
                    self.recursively_mark_previously_used(c);
                }
            }
        }
    }
    fn can_traverse(&self, t: TileId) -> bool {
        if self.tr(t).error <= self.error_target && !self.can_unconditionally_refine(t) {
            return false;
        }
        if self.max_depth > 0. && (self.tiles[t].depth + 1) as f64 >= self.max_depth {
            return false;
        }
        self.children_processed(t)
    }
    fn kick_active_children(&mut self, t: TileId) {
        for c in self.tiles[t].children.clone() {
            if self.used_this_frame(c) {
                if self.tr(c).active {
                    self.trm(c).kicked = true;
                    self.trm(c).active = false;
                }
                self.kick_active_children(c);
            }
        }
    }
    fn is_child_ready(&self, t: TileId) -> bool {
        !self.can_unconditionally_refine(t)
            && (!self.tiles[t].has_content
                || Self::is_download_finished(self.tiles[t].loading_state))
    }
    fn mark_used_tiles(&mut self, t: TileId) {
        self.reset_frame_state(t);
        if !self.tr(t).in_frustum {
            return;
        }
        if !self.can_traverse(t) {
            self.trm(t).used = true;
            return;
        }
        let mut any_used = false;
        let mut any_in_frustum = false;
        let children = self.tiles[t].children.clone();
        for &c in &children {
            self.mark_used_tiles(c);
            any_used = any_used || self.used_this_frame(c);
            any_in_frustum = any_in_frustum || self.tr(c).in_frustum;
        }
        if self.tiles[t].refine == Refine::Replace && !any_in_frustum && !children.is_empty() {
            self.trm(t).in_frustum = false;
            self.mark_tile_used(t);
            for &c in &children {
                self.recursively_mark_used(c, true);
            }
            return;
        }
        self.trm(t).used = true;
        if self.tiles[t].refine == Refine::Replace
            && any_used
            && (self.load_siblings || self.load_ancestors)
        {
            for &c in &children {
                self.recursively_mark_used(c, false);
            }
        }
    }
    fn mark_used_set_leaves(&mut self, t: TileId) {
        if !self.used_this_frame(t) {
            return;
        }
        let children = self.tiles[t].children.clone();
        let any_used = children.iter().any(|&c| self.used_this_frame(c));
        if !any_used {
            self.trm(t).is_leaf = true;
        } else {
            for &c in &children {
                self.mark_used_set_leaves(c);
            }
            let mut all_loaded = true;
            for &c in &children {
                if self.used_this_frame(c) {
                    let can_display = !self.can_unconditionally_refine(c);
                    let content_ready = !self.tiles[c].has_content
                        || Self::is_download_finished(self.tiles[c].loading_state);
                    let ready = (can_display && content_ready) || self.tr(c).all_children_loaded;
                    if !ready {
                        all_loaded = false;
                    }
                }
            }
            self.trm(t).all_children_loaded = all_loaded;
        }
        let mut all_processed = true;
        for &c in &children {
            if self.used_this_frame(c) && !self.tr(c).all_used_children_processed {
                all_processed = false;
            }
        }
        let value = all_processed && self.children_processed(t);
        self.trm(t).all_used_children_processed = value;
    }
    fn mark_visible_tiles(&mut self, t: TileId) {
        if !self.used_this_frame(t) {
            return;
        }
        let children = self.tiles[t].children.clone();
        if self.load_ancestors
            && !self.tr(t).all_children_loaded
            && !self.can_unconditionally_refine(t)
        {
            self.trm(t).is_leaf = true;
        }
        if self.tr(t).is_leaf {
            if !self.can_unconditionally_refine(t) {
                self.trm(t).active = true;
                if self.children_processed(t)
                    && self.tiles[t].has_content
                    && !Self::is_download_finished(self.tiles[t].loading_state)
                {
                    for &c in &children {
                        self.recursively_mark_previously_used(c);
                    }
                }
            }
            return;
        }
        let mut all_ready = !children.is_empty();
        for &c in &children {
            self.mark_visible_tiles(c);
            if self.used_this_frame(c) {
                let ready = self.tr(c).active && self.is_child_ready(c);
                if !ready && !self.tr(c).all_children_ready {
                    all_ready = false;
                }
            }
        }
        self.trm(t).all_children_ready = all_ready;
        if !all_ready && self.tr(t).was_set_active && self.is_child_ready(t) {
            self.trm(t).active = true;
            self.kick_active_children(t);
        }
    }
    fn toggle_tiles(&mut self, t: TileId) {
        let used = self.used_this_frame(t);
        if used {
            let tile = &self.tiles[t];
            if tile.has_unrenderable || (tile.has_renderable && tile.refine == Refine::Add) {
                self.trm(t).active = true;
            }
            if (self.tr(t).active || self.tr(t).kicked) && self.tiles[t].has_content {
                self.mark_tile_used(t);
                if self.tiles[t].has_unrenderable || self.tr(t).all_used_children_processed {
                    self.queue_for_download(t);
                }
                if self.tiles[t].loading_state != LOADED {
                    self.trm(t).active = false;
                }
            }
            if self.load_ancestors && self.tiles[t].has_content {
                self.mark_tile_used(t);
                self.queue_for_download(t);
            }
            if self.tiles[t].virtual_child_count > 0 && self.tiles[t].has_content {
                self.mark_tile_used(t);
            }
            let tile = &self.tiles[t];
            let visible = tile.has_renderable
                && self.tr(t).active
                && self.tr(t).in_frustum
                && tile.loading_state == LOADED;
            self.trm(t).visible = visible;
            self.stats.used += 1;
            if self.tr(t).in_frustum {
                self.stats.in_frustum += 1;
            }
        }
        if used || (self.processed(t) && self.tr(t).used_last_frame) {
            let mut set_active = false;
            let mut set_visible = false;
            if used {
                set_active = self.tr(t).active;
                set_visible = if self.display_active_tiles {
                    self.tr(t).active || self.tr(t).visible
                } else {
                    self.tr(t).visible
                };
            } else {
                self.reset_frame_state(t);
            }
            let tile = &self.tiles[t];
            if tile.has_renderable && tile.loading_state == LOADED {
                if set_active {
                    self.stats.active += 1;
                }
                if set_visible {
                    self.stats.visible += 1;
                }
                if self.tr(t).was_set_active != set_active {
                    self.set_tile_active(t, set_active);
                }
                if self.tr(t).was_set_visible != set_visible {
                    self.set_tile_visible_plugins(t, set_visible);
                }
            } else if !tile.has_renderable {
                set_visible = self.tr(t).is_leaf;
            }
            let tr = self.trm(t);
            tr.was_set_active = set_active;
            tr.was_set_visible = set_visible;
            tr.used_last_frame = used;
            for c in self.tiles[t].children.clone() {
                self.toggle_tiles(c);
            }
        }
    }

    fn set_tile_active(&mut self, t: TileId, active: bool) {
        if active {
            self.active_tiles.insert(t);
        } else {
            self.active_tiles.remove(&t);
        }
    }
    /// invokeOnePlugin( setTileVisible ): the fade plugin first, then the renderer.
    fn set_tile_visible_plugins(&mut self, t: TileId, visible: bool) {
        if self
            .fade
            .set_tile_visible(t, visible, self.tiles[t].depth_from_rendered_parent)
        {
            return;
        }
        self.set_tile_visible(t, visible);
    }
    /// The renderer's setTileVisible: the visible set and its event.
    pub fn set_tile_visible(&mut self, t: TileId, visible: bool) {
        if visible {
            self.visible_tiles.insert(t);
        } else {
            self.visible_tiles.remove(&t);
        }
        self.events.visibility.push((t, visible));
    }

    fn mark_tile_used(&mut self, t: TileId) {
        self.used_set.insert(t);
        self.lru.mark_used(t);
    }
    fn queue_for_download(&mut self, t: TileId) {
        if self.tiles[t].loading_state != UNLOADED || self.lru.is_full() {
            return;
        }
        self.queued_tiles.push(t);
    }

    /// lruPriorityCallback.
    fn lru_priority(&self, a: TileId, b: TileId) -> Ordering {
        let (ta, tb) = (&self.tiles[a], &self.tiles[b]);
        let (ra, rb) = (self.tr(a), self.tr(b));
        let sign = |c: bool| if c { Ordering::Less } else { Ordering::Greater };
        if ra.last_frame_visited != rb.last_frame_visited {
            sign(ra.last_frame_visited > rb.last_frame_visited)
        } else if ta.depth_from_rendered_parent != tb.depth_from_rendered_parent {
            sign(ta.depth_from_rendered_parent <= tb.depth_from_rendered_parent)
        } else if ta.loading_state != tb.loading_state {
            sign(ta.loading_state > tb.loading_state)
        } else if ta.has_unrenderable != tb.has_unrenderable {
            sign(ta.has_unrenderable)
        } else if ra.error != rb.error {
            sign(ra.error > rb.error)
        } else {
            Ordering::Equal
        }
    }
    /// errorPriorityCallback ( unifiedPriorityCallback with loadAncestors ).
    fn error_priority(&self, a: TileId, b: TileId) -> Ordering {
        let (ta, tb) = (&self.tiles[a], &self.tiles[b]);
        let pa = ta.priority.unwrap_or(f64::INFINITY);
        let pb = tb.priority.unwrap_or(f64::INFINITY);
        let up = |c: bool| if c { Ordering::Greater } else { Ordering::Less };
        if pa != pb {
            return up(pa > pb);
        }
        let (Some(ra), Some(rb)) = (ta.traversal.as_ref(), tb.traversal.as_ref()) else {
            return Ordering::Equal;
        };
        if ra.used != rb.used {
            up(ra.used)
        } else if ra.error != rb.error {
            up(ra.error > rb.error)
        } else if ra.distance_from_camera != rb.distance_from_camera {
            up(ra.distance_from_camera <= rb.distance_from_camera)
        } else if ta.depth_from_rendered_parent != tb.depth_from_rendered_parent {
            up(ta.depth_from_rendered_parent <= tb.depth_from_rendered_parent)
        } else {
            Ordering::Equal
        }
    }

    /// TilesRendererBase.update ( and the three renderer's camera check ).
    pub fn update(&mut self) {
        let Some(root) = self.root else { return };
        // UpdateOnChangePlugin.doTilesNeedUpdate.
        let mut changed = false;
        if let Some(c) = &self.camera {
            let m = multiply(
                &c.projection_inverse,
                &multiply(&c.world_inverse, &super::volume::IDENTITY),
            );
            changed = self.camera_matrix != Some(m);
            self.camera_matrix = Some(m);
        }
        let needs = std::mem::take(&mut self.needs_update) || changed;
        self.fade.update_before(&mut self.display_active_tiles);
        if !needs {
            self.fade_update_after();
            return;
        }
        self.stats.in_frustum = 0;
        self.stats.used = 0;
        self.stats.active = 0;
        self.stats.visible = 0;
        self.stats.tiles_processed = 0;
        self.frame_count += 1;
        for t in std::mem::take(&mut self.used_set) {
            self.lru.used.remove(&t);
        }
        self.prepare();
        self.mark_used_tiles(root);
        self.mark_used_set_leaves(root);
        self.mark_visible_tiles(root);
        self.toggle_tiles(root);
        self.remove_unused_pending();
        let mut queued = std::mem::take(&mut self.queued_tiles);
        queued.sort_by(|&a, &b| self.lru_priority(a, b));
        for t in queued {
            if self.lru.is_full() {
                break;
            }
            self.request_contents(t);
        }
        self.unload_unused();
        let running = self.download_queue.running()
            || self.parse_queue.running()
            || self.process_queue.running();
        if !running && self.is_loading {
            self.cached_since_load.clear();
            self.stats.in_cache_since_load = 0;
            self.is_loading = false;
        }
        self.fade_update_after();
    }

    /// TilesFadePlugin's update-after: the fades advance and the fading
    /// tiles stay in the cache.
    fn fade_update_after(&mut self) {
        let complete = self
            .fade
            .update_after(&mut self.display_active_tiles, &self.visible_tiles);
        for t in complete {
            // onFadeComplete( tile, false ): the other plugins hide it.
            self.set_tile_visible(t, false);
        }
        for t in self.fade.fading_tiles() {
            self.mark_tile_used(t);
        }
    }

    fn remove_unused_pending(&mut self) {
        let remove: Vec<TileId> = self
            .loading_tiles
            .iter()
            .copied()
            .filter(|t| !self.lru.used.contains(t) && self.tiles[*t].loading_state == QUEUED)
            .collect();
        for t in remove {
            self.lru_remove(t);
        }
    }

    /// LRUCache.remove and the tile's remove callback.
    fn lru_remove(&mut self, t: TileId) {
        if !self.lru.item_set.contains(&t) {
            return;
        }
        self.lru.cached_bytes -= self.lru.bytes.remove(&t).unwrap_or(0.);
        self.remove_callback(t);
        if let Some(i) = self.lru.items.iter().position(|&x| x == t) {
            self.lru.items.remove(i);
        }
        self.lru.used.remove(&t);
        self.lru.item_set.remove(&t);
        self.lru.loaded.remove(&t);
    }
    /// requestTileContents' remove callback: abort, dispose and reset.
    fn remove_callback(&mut self, t: TileId) {
        if let Some(p) = self.pending.get(&t) {
            *p.abort.borrow_mut() = true;
        }
        if self.tiles[t].external {
            self.tiles[t].children.clear();
        } else {
            self.dispose_tile(t);
        }
        self.stats.in_cache -= 1;
        if self.cached_since_load.remove(&t) {
            self.stats.in_cache_since_load -= 1;
        }
        match self.tiles[t].loading_state {
            QUEUED => self.stats.queued -= 1,
            LOADING => self.stats.downloading -= 1,
            PARSING => self.stats.parsing -= 1,
            LOADED => self.stats.loaded -= 1,
            _ => {}
        }
        self.tiles[t].loading_state = UNLOADED;
        self.parse_queue.remove(t);
        self.download_queue.remove(t);
        self.loading_tiles.remove(&t);
    }
    /// disposeTile: hidden, inactive, its scene released.
    fn dispose_tile(&mut self, t: TileId) {
        if self.processed(t) {
            if self.tr(t).visible {
                self.set_tile_visible_plugins(t, false);
                self.trm(t).visible = false;
            }
            if self.tr(t).active {
                self.set_tile_active(t, false);
                self.trm(t).active = false;
            }
        }
        if let Some(scene) = self.tiles[t].scene.take() {
            // TilesFadePlugin's dispose-model: the parent fades quickly, the tile's fade ends.
            if self.visible_tiles.contains(&t)
                && let Some(p) = self.tiles[t].parent
            {
                self.fade.quick.insert(p);
            }
            self.fade.delete(t);
            self.visible_tiles.remove(&t);
            self.events.disposed.push((t, scene));
        }
    }

    /// requestTileContents: the cache entry and the download job.
    fn request_contents(&mut self, t: TileId) {
        if self.tiles[t].loading_state != UNLOADED {
            return;
        }
        let Some(uri) = self.tiles[t].content_uri.clone() else {
            return;
        };
        let url = resolve(&uri, &format!("{}/", self.tiles[t].base_path));
        let ext = extension(&url);
        if self.lru.item_set.contains(&t) || self.lru.is_full() {
            return;
        }
        self.lru.items.push(t);
        self.lru.used.insert(t);
        self.lru.item_set.insert(t);
        if !self.is_loading {
            self.is_loading = true;
        }
        let bytes = self.bytes_used(t);
        self.lru.set_memory(t, bytes);
        self.cached_since_load.insert(t);
        self.stats.in_cache_since_load += 1;
        self.stats.in_cache += 1;
        self.stats.queued += 1;
        self.tiles[t].loading_state = QUEUED;
        self.loading_tiles.insert(t);
        self.pending.insert(
            t,
            Pending {
                abort: Rc::new(RefCell::new(false)),
                url,
                extension: ext,
            },
        );
        self.download_queue.add(t);
    }

    /// calculateBytesUsed: estimateBytesUsed of the tile's scene.
    fn bytes_used(&self, t: TileId) -> f64 {
        self.tiles[t].scene.as_ref().map_or(0., |s| s.bytes)
    }

    /// LRUCache.unloadUnusedContent ( scheduleUnload's microtask ).
    fn unload_unused(&mut self) {
        let lru = &self.lru;
        let unused = lru.items.len() as f64 - lru.used.len() as f64;
        let unloaded = lru.items.len() as f64 - lru.loaded.len() as f64;
        let excess_nodes = (lru.items.len() as f64 - lru.min_size).min(unused).max(0.);
        let excess_bytes = lru.cached_bytes - lru.min_bytes;
        let has_nodes = excess_nodes > 0. && unused > 0.
            || unloaded != 0. && lru.items.len() as f64 > lru.max_size;
        let has_bytes = unused != 0. && lru.cached_bytes > lru.min_bytes
            || unloaded != 0. && lru.cached_bytes > lru.max_bytes;
        if !(has_bytes || has_nodes) {
            return;
        }
        let mut items = lru.items.clone();
        items.sort_by(|&a, &b| {
            let (ua, ub) = (lru.used.contains(&a), lru.used.contains(&b));
            if ua == ub {
                let (la, lb) = (lru.loaded.contains(&a), lru.loaded.contains(&b));
                if la == lb {
                    self.lru_priority(a, b).reverse()
                } else if la {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else if ua {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        let lru = &self.lru;
        let max_unload = (lru.min_size * lru.unload_percent).max(excess_nodes * lru.unload_percent);
        let nodes_to_unload = max_unload.min(unused).min(excess_nodes).ceil();
        let max_bytes_unload =
            (lru.unload_percent * excess_bytes).max(lru.unload_percent * lru.min_bytes);
        let bytes_to_unload = max_bytes_unload.min(excess_bytes);
        let (mut removed_nodes, mut removed_bytes) = (0usize, 0.);
        while lru.cached_bytes - removed_bytes > lru.max_bytes
            || (items.len() - removed_nodes) as f64 > lru.max_size
        {
            let item = items[removed_nodes];
            let bytes = lru.bytes.get(&item).copied().unwrap_or(0.);
            if lru.used.contains(&item) && lru.loaded.contains(&item)
                || lru.cached_bytes - removed_bytes - bytes < lru.max_bytes
                    && (items.len() - removed_nodes) as f64 <= lru.max_size
            {
                break;
            }
            removed_bytes += bytes;
            removed_nodes += 1;
        }
        while removed_bytes < bytes_to_unload || (removed_nodes as f64) < nodes_to_unload {
            let Some(&item) = items.get(removed_nodes) else {
                break;
            };
            let bytes = lru.bytes.get(&item).copied().unwrap_or(0.);
            if lru.used.contains(&item)
                || lru.cached_bytes - removed_bytes - bytes < lru.min_bytes
                    && removed_nodes as f64 >= nodes_to_unload
            {
                break;
            }
            removed_bytes += bytes;
            removed_nodes += 1;
        }
        let removed: Vec<TileId> = items.drain(..removed_nodes).collect();
        self.lru.items = items;
        for t in removed {
            self.lru.cached_bytes -= self.lru.bytes.get(&t).copied().unwrap_or(0.);
            self.remove_callback(t);
            self.lru.bytes.remove(&t);
            self.lru.item_set.remove(&t);
            self.lru.loaded.remove(&t);
            self.lru.used.remove(&t);
        }
        let rerun = ((removed_nodes as f64) < excess_nodes
            || removed_bytes < excess_bytes && (removed_nodes as f64) < unused)
            && removed_nodes > 0;
        self.lru.rerun = rerun;
    }

    /// The queues' scheduled runs ( the next animation frame ), the
    /// throttled needs-update and the finished jobs.
    pub fn tick(&mut self, loader: &mut dyn Loader) {
        if std::mem::take(&mut self.needs_update_throttled) {
            self.needs_update = true;
        }
        if std::mem::take(&mut self.lru.rerun) {
            self.unload_unused();
        }
        self.poll_jobs();
        // processNodeQueue: children preprocessed in their parent's download priority.
        if std::mem::take(&mut self.process_queue.scheduled) {
            let mut items = std::mem::take(&mut self.process_queue.items);
            items.sort_by(
                |&a, &b| match (self.tiles[a].parent, self.tiles[b].parent) {
                    (pa, pb) if pa == pb => Ordering::Equal,
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                    (Some(pa), Some(pb)) => self.error_priority(pa, pb),
                },
            );
            let mut ran = 0;
            while ran < self.process_queue.max_jobs {
                let Some(t) = items.pop() else { break };
                self.process_queue.callbacks.remove(&t);
                let base = self.tiles[t].base_path.clone();
                for c in self.tiles[t].children.clone() {
                    if !self.processed(c) {
                        self.preprocess_node(c, &base, Some(t));
                    }
                }
                self.needs_update_throttled = true;
                ran += 1;
            }
            self.process_queue.items = items;
            if !self.process_queue.items.is_empty() {
                self.process_queue.scheduled = true;
            }
        }
        if std::mem::take(&mut self.download_queue.scheduled) {
            self.run_downloads(loader);
        }
        if std::mem::take(&mut self.parse_queue.scheduled) {
            self.run_parses(loader);
        }
    }

    /// The download queue's tryRunJobs: each popped tile starts its fetch.
    fn run_downloads(&mut self, loader: &mut dyn Loader) {
        let mut items = std::mem::take(&mut self.download_queue.items);
        items.sort_by(|&a, &b| self.error_priority(a, b));
        let mut iterated = 0;
        while self.download_queue.max_jobs > self.download_queue.curr_jobs
            && !items.is_empty()
            && iterated < self.download_queue.max_jobs
        {
            iterated += 1;
            let t = items.pop().expect("item");
            self.download_queue.callbacks.remove(&t);
            let Some(p) = self.pending.get(&t) else {
                continue;
            };
            if *p.abort.borrow() {
                continue;
            }
            self.download_queue.curr_jobs += 1;
            self.tiles[t].loading_state = LOADING;
            self.stats.downloading += 1;
            self.stats.queued -= 1;
            let job = loader.download(p, self.google_key.as_deref(), self.session.as_deref());
            self.jobs.insert(t, Job::Download(job));
        }
        self.download_queue.items = items;
    }

    /// The parse queue's tryRunJobs: external tilesets join the tree at once,
    /// models start their parse.
    fn run_parses(&mut self, loader: &mut dyn Loader) {
        let mut items = std::mem::take(&mut self.parse_queue.items);
        items.sort_by(|&a, &b| self.error_priority(a, b));
        let mut iterated = 0;
        while self.parse_queue.max_jobs > self.parse_queue.curr_jobs
            && !items.is_empty()
            && iterated < self.parse_queue.max_jobs
        {
            iterated += 1;
            let t = items.pop().expect("item");
            self.parse_queue.callbacks.remove(&t);
            let Some(bytes) = self.parse_input.remove(&t) else {
                continue;
            };
            let Some(p) = self.pending.get(&t) else {
                continue;
            };
            if *p.abort.borrow() {
                continue;
            }
            let (url, json) = (p.url.clone(), p.extension.as_deref() == Some("json"));
            if json {
                let content: serde_json::Value = match serde_json::from_slice(&bytes) {
                    Ok(v) => v,
                    Err(e) => {
                        self.fail(t, e.to_string());
                        continue;
                    }
                };
                if !content["root"].is_null() {
                    let root = self.preprocess_tileset(content, &url, Some(t));
                    self.tiles[t].children.push(root);
                    self.tiles[t].external = true;
                    self.finish(t, None);
                    continue;
                }
            }
            self.parse_queue.curr_jobs += 1;
            let job = loader.parse(p, bytes, &self.tiles[t].transform, &self.up_rotation);
            self.jobs.insert(t, Job::Parse(job));
        }
        self.parse_queue.items = items;
    }

    /// The promise chain's continuations for the finished downloads and parses.
    fn poll_jobs(&mut self) {
        let ready: Vec<TileId> = self
            .jobs
            .iter()
            .filter(|(_, j)| match j {
                Job::Download(c) => c.borrow().is_some(),
                Job::Parse(c) => c.borrow().is_some(),
            })
            .map(|(&t, _)| t)
            .collect();
        for t in ready {
            let Some(job) = self.jobs.remove(&t) else {
                continue;
            };
            let aborted = self.pending.get(&t).is_none_or(|p| *p.abort.borrow());
            match job {
                Job::Download(cell) => {
                    self.download_queue.curr_jobs -= 1;
                    self.download_queue.scheduled = true;
                    let result = cell.borrow_mut().take().expect("ready");
                    if aborted {
                        continue;
                    }
                    match result {
                        Ok(bytes) => {
                            self.stats.downloading -= 1;
                            self.stats.parsing += 1;
                            self.tiles[t].loading_state = PARSING;
                            self.parse_input.insert(t, bytes);
                            self.parse_queue.add(t);
                        }
                        Err(e) => self.fail(t, e),
                    }
                }
                Job::Parse(cell) => {
                    self.parse_queue.curr_jobs -= 1;
                    self.parse_queue.scheduled = true;
                    let result = cell.borrow_mut().take().expect("ready");
                    if aborted {
                        continue;
                    }
                    match result {
                        Ok(content) => self.finish(t, content),
                        Err(e) => self.fail(t, e),
                    }
                }
            }
        }
    }

    /// The final `then`: loaded, its bytes in the cache, needs-update.
    fn finish(&mut self, t: TileId, content: Option<super::content::TileContent>) {
        if let Some(c) = content {
            self.tiles[t].scene = Some(Rc::new(c));
            self.events.loaded.push(t);
        }
        self.stats.parsing -= 1;
        self.stats.loaded += 1;
        self.tiles[t].loading_state = LOADED;
        self.loading_tiles.remove(&t);
        if self.lru.item_set.contains(&t) {
            self.lru.loaded.insert(t);
        }
        let bytes = self.bytes_used(t);
        if self.lru.bytes.get(&t).copied().unwrap_or(0.) == 0. && bytes > 0. && self.lru.is_full() {
            self.lru_remove(t);
            return;
        }
        self.lru.set_memory(t, bytes);
        self.needs_update = true;
    }

    /// The chain's catch: failed, out of the queues.
    fn fail(&mut self, t: TileId, error: String) {
        self.parse_queue.remove(t);
        self.download_queue.remove(t);
        match self.tiles[t].loading_state {
            QUEUED => self.stats.queued -= 1,
            LOADING => self.stats.downloading -= 1,
            PARSING => self.stats.parsing -= 1,
            LOADED => self.stats.loaded -= 1,
            _ => {}
        }
        self.stats.failed += 1;
        web_sys::console::error_1(
            &format!(
                "TilesRenderer : Failed to load tile at url \"{}\". {error}",
                self.tiles[t].content_uri.clone().unwrap_or_default()
            )
            .into(),
        );
        self.tiles[t].loading_state = FAILED;
        self.loading_tiles.remove(&t);
        if self.lru.item_set.contains(&t) {
            self.lru.loaded.insert(t);
        }
    }
}

/// The fetch and the model parse, started by the queues.
pub(crate) trait Loader {
    fn download(&mut self, p: &Pending, key: Option<&str>, session: Option<&str>) -> Slot<Vec<u8>>;
    fn parse(
        &mut self,
        p: &Pending,
        bytes: Vec<u8>,
        transform: &M4,
        up: &M4,
    ) -> Slot<Option<super::content::TileContent>>;
}
