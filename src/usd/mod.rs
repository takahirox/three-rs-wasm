//! USDLoader ( examples/jsm/loaders/USDLoader.js, r186 ) and its parsers and
//! composer ( examples/jsm/loaders/usd/ ): USDZ packages, USDA text and USDC
//! crate layers into the loader's scene — transforms from xformOps, meshes
//! and skinned meshes with GeomSubset groups, UsdPreviewSurface and OpenPBR
//! MeshPhysicalMaterials with their textures, geometry primitives, cameras,
//! lights, skeletons, and skeletal and transform animation clips. The parse
//! follows the original step by step, with the parsers' JavaScript value
//! shapes and three.js's math, so the result matches the loader's.
//!
//! [`parse`] returns the loader's group as plain data: the nodes in
//! `traverse()` order with their local transforms and world matrices, and
//! shared geometry, material and texture tables.
mod canonical;
mod compose;
pub(crate) mod js;
mod usda;
mod usdc;

pub use canonical::canonical;

use crate::Result;
use crate::geometry::BufferGeometry;
use js::Obj;
use std::collections::HashMap;
use std::rc::Rc;

/// A spec of `specsByPath`: its type and fields.
#[derive(Debug)]
pub(crate) struct Spec {
    pub spec_type: f64,
    pub fields: Obj,
}
impl Spec {
    pub(crate) fn new(spec_type: f64, fields: Obj) -> Rc<Spec> {
        Rc::new(Spec { spec_type, fields })
    }
}

/// `specsByPath`: an object keyed by path, in insertion order ( paths are
/// never array indices ).
#[derive(Debug, Default)]
pub(crate) struct Specs {
    order: Vec<Rc<str>>,
    map: HashMap<Rc<str>, Rc<Spec>>,
}
impl Specs {
    pub(crate) fn set(&mut self, path: &str, spec: Rc<Spec>) {
        if let Some(s) = self.map.get_mut(path) {
            *s = spec;
        } else {
            let k: Rc<str> = path.into();
            self.order.push(k.clone());
            self.map.insert(k, spec);
        }
    }
    pub(crate) fn get(&self, path: &str) -> Option<Rc<Spec>> {
        self.map.get(path).cloned()
    }
    pub(crate) fn paths(&self) -> Vec<Rc<str>> {
        self.order.clone()
    }
}

/// The loaded group.
#[derive(Clone, Debug, Default)]
pub struct UsdScene {
    /// The group and its descendants in `traverse()` order; index 0 is the
    /// group.
    pub nodes: Vec<UsdNode>,
    pub geometries: Vec<BufferGeometry>,
    pub materials: Vec<UsdMaterial>,
    pub textures: Vec<UsdTexture>,
    pub skeletons: Vec<UsdSkeleton>,
    pub clips: Vec<UsdClip>,
    /// `group.skeleton`, when the layer has one skeleton.
    pub skeleton: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct UsdNode {
    pub name: String,
    pub parent: Option<usize>,
    pub kind: UsdNodeKind,
    pub position: [f64; 3],
    pub quaternion: [f64; 4],
    pub scale: [f64; 3],
    pub pivot: Option<[f64; 3]>,
    /// `matrixWorld` after `updateMatrixWorld()`, column-major.
    pub matrix_world: [f64; 16],
}

#[derive(Clone, Debug, PartialEq)]
pub enum UsdNodeKind {
    Group,
    Object3D,
    Bone,
    Mesh {
        geometry: usize,
        /// The mesh's material, or materials when `multi`.
        materials: Vec<usize>,
        multi: bool,
        /// A SkinnedMesh's skeleton ( `None` when unbound ) and bind matrix.
        skin: Option<(Option<usize>, [f64; 16])>,
    },
    PerspectiveCamera {
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
        film_gauge: f64,
        film_offset: f64,
        focus: f64,
    },
    OrthographicCamera {
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
        near: f64,
        far: f64,
    },
    DirectionalLight {
        color: [f64; 3],
        intensity: f64,
    },
    PointLight {
        color: [f64; 3],
        intensity: f64,
    },
    SpotLight {
        color: [f64; 3],
        intensity: f64,
        angle: f64,
        penumbra: f64,
    },
    RectAreaLight {
        color: [f64; 3],
        intensity: f64,
        width: f64,
        height: f64,
    },
}

/// A Skeleton: its bones ( node indices, `None` for bones outside the
/// group ) and inverse bind matrices.
#[derive(Clone, Debug)]
pub struct UsdSkeleton {
    pub bones: Vec<Option<usize>>,
    pub bone_inverses: Vec<[f64; 16]>,
}

/// A MeshPhysicalMaterial's parameters as the composer leaves them.
#[derive(Clone, Debug)]
pub struct UsdMaterial {
    pub color: [f64; 3],
    pub emissive: [f64; 3],
    pub emissive_intensity: f64,
    pub roughness: f64,
    pub metalness: f64,
    pub ior: f64,
    pub specular_intensity: f64,
    pub specular_color: [f64; 3],
    pub clearcoat: f64,
    pub clearcoat_roughness: f64,
    pub sheen: f64,
    pub transmission: f64,
    pub thickness: f64,
    pub attenuation_color: [f64; 3],
    pub attenuation_distance: f64,
    pub iridescence: f64,
    pub iridescence_ior: f64,
    pub iridescence_thickness_range: [f64; 2],
    pub anisotropy: f64,
    pub opacity: f64,
    pub transparent: bool,
    pub alpha_test: f64,
    pub normal_scale: [f64; 2],
    pub map: Option<usize>,
    pub emissive_map: Option<usize>,
    pub normal_map: Option<usize>,
    pub roughness_map: Option<usize>,
    pub metalness_map: Option<usize>,
    pub ao_map: Option<usize>,
    pub specular_color_map: Option<usize>,
}

/// Where a texture's image comes from.
#[derive(Clone, Debug, PartialEq)]
pub enum UsdImage {
    /// An image file of the package ( a Blob URL in the original ).
    Asset { name: String, data: Rc<Vec<u8>> },
    /// A URL relative to the layer.
    Url(String),
}

#[derive(Clone, Debug)]
pub struct UsdTexture {
    pub image: UsdImage,
    /// SRGBColorSpace when true, NoColorSpace otherwise.
    pub srgb: bool,
    /// three.js wrapping constants.
    pub wrap_s: u32,
    pub wrap_t: u32,
    pub repeat: [f64; 2],
    pub offset: [f64; 2],
    pub rotation: f64,
    /// The uv channel ( `texture.channel` ).
    pub channel: u32,
    /// UsdUVTexture's `inputs:scale` and `inputs:bias` ( `userData` ).
    pub scale: Option<Vec<f64>>,
    pub bias: Option<Vec<f64>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsdTrackKind {
    Vector,
    Quaternion,
}

#[derive(Clone, Debug)]
pub struct UsdTrack {
    /// `nodeName.property`.
    pub name: String,
    pub kind: UsdTrackKind,
    pub times: Vec<f32>,
    pub values: Vec<f32>,
}

#[derive(Clone, Debug)]
pub struct UsdClip {
    pub name: String,
    pub duration: f64,
    pub tracks: Vec<UsdTrack>,
}

/// USDLoader.parse( buffer, path ).
pub fn parse(bytes: &[u8], path: &str) -> Result<UsdScene> {
    compose::load(bytes, path)
}
