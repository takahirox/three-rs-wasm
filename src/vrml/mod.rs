//! VRMLLoader ( examples/jsm/loaders/VRMLLoader.js, r186 ): VRML 2.0 text
//! into the loader's scene — groups and transforms, shapes as Phong / basic
//! meshes, line segments and points with their materials and textures
//! ( ImageTexture URLs and PixelTexture data ), the Background node's sky
//! and ground spheres, and cameras. The lexer and parser reproduce the
//! chevrotain grammar the original builds, and the scene builders follow it
//! step by step: USE clones objects and materials and shares the rest,
//! values keep their JavaScript coercions, and the math is three.js's, so the
//! result matches the loader's.
//!
//! [`parse`] returns the scene as plain data: the objects in `traverse()`
//! order with their local transforms and world matrices, and shared
//! geometry, material and texture tables. Object and material ids count
//! from 0 in creation order, as the loader's `id`s advance during a parse.
mod build;
mod canonical;
mod parse;

pub use canonical::canonical;

use crate::Result;
use crate::geometry::BufferGeometry;

/// VRMLLoader's options.
#[derive(Clone, Debug)]
pub struct Options {
    /// The texture path ( `resourcePath`, or the asset URL's base ).
    pub path: String,
    /// `window.innerWidth` and `innerHeight`, for cameras.
    pub window_size: (f64, f64),
}
impl Default for Options {
    fn default() -> Self {
        Self {
            path: String::new(),
            window_size: (1.0, 1.0),
        }
    }
}

/// The loaded scene.
#[derive(Clone, Debug, Default)]
pub struct VrmlScene {
    /// The Scene and its descendants in `traverse()` order; index 0 is the
    /// scene.
    pub nodes: Vec<VrmlNode>,
    pub geometries: Vec<VrmlGeometry>,
    /// Every material the parse created ( the renderables' among them ).
    pub materials: Vec<VrmlMaterial>,
    pub textures: Vec<VrmlTexture>,
}

#[derive(Clone, Debug)]
pub struct VrmlNode {
    /// Creation order among the parse's objects ( `Object3D.id` ).
    pub id: u32,
    pub name: String,
    pub parent: Option<usize>,
    pub kind: VrmlNodeKind,
    pub position: [f64; 3],
    pub quaternion: [f64; 4],
    pub scale: [f64; 3],
    pub render_order: f64,
    pub visible: bool,
    /// `matrixWorld` after `updateMatrixWorld()`, column-major.
    pub matrix_world: [f64; 16],
}

#[derive(Clone, Debug, PartialEq)]
pub enum VrmlNodeKind {
    Scene,
    Group,
    Object3D,
    Mesh {
        geometry: usize,
        material: usize,
    },
    LineSegments {
        geometry: usize,
        material: usize,
    },
    Points {
        geometry: usize,
        material: usize,
    },
    PerspectiveCamera {
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
    },
    OrthographicCamera {
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
        near: f64,
        far: f64,
    },
}

#[derive(Clone, Debug)]
pub struct VrmlGeometry {
    pub geometry: BufferGeometry,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VrmlMaterialKind {
    MeshBasic,
    MeshPhong,
    Points,
    LineBasic,
}

#[derive(Clone, Debug)]
pub struct VrmlMaterial {
    /// Creation order among the parse's materials ( `Material.id` ).
    pub id: u32,
    pub kind: VrmlMaterialKind,
    pub name: String,
    pub color: [f64; 3],
    /// MeshPhongMaterial's emissive, specular and shininess.
    pub emissive: [f64; 3],
    pub specular: [f64; 3],
    pub shininess: f64,
    pub opacity: f64,
    pub transparent: bool,
    /// FrontSide 0, BackSide 1, DoubleSide 2.
    pub side: u32,
    pub vertex_colors: bool,
    pub depth_write: bool,
    pub depth_test: bool,
    pub fog: bool,
    /// PointsMaterial's size and sizeAttenuation.
    pub size: f64,
    pub size_attenuation: bool,
    pub map: Option<usize>,
}

/// Where a texture's image comes from.
#[derive(Clone, Debug, PartialEq)]
pub enum VrmlImage {
    /// An ImageTexture's URL ( the texture path joined with the file name ).
    Url(String),
    /// A PixelTexture's DataTexture: RGBA bytes.
    Data {
        width: f64,
        height: f64,
        data: Vec<u8>,
    },
}

#[derive(Clone, Debug)]
pub struct VrmlTexture {
    pub name: String,
    pub image: VrmlImage,
    /// three.js wrapping and filter constants.
    pub wrap_s: u32,
    pub wrap_t: u32,
    pub mag_filter: u32,
    pub min_filter: u32,
    pub generate_mipmaps: bool,
    pub flip_y: bool,
    /// SRGBColorSpace.
    pub srgb: bool,
    pub offset: [f64; 2],
    pub repeat: [f64; 2],
    pub center: [f64; 2],
    pub rotation: f64,
    /// `matrix` after `updateMatrix()` ( the uv transform ), column-major.
    pub matrix: [f64; 9],
}

/// VRMLLoader.parse( data, path ).
pub fn parse(text: &str, options: &Options) -> Result<VrmlScene> {
    let tree = parse::tree(text)?;
    build::scene(&tree, options)
}
