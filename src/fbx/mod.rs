//! FBXLoader ( examples/jsm/loaders/FBXLoader.js, r187 ): binary ( ≥ 6400 )
//! and ASCII ( ≥ 7000 ) FBX into the loader's scene graph — groups, bones,
//! meshes and skinned meshes with their geometry, Phong / Lambert materials
//! and textures, NURBS curves as lines, cameras and lights — and its
//! animation clips. The parse follows the original step by step, including
//! the FBXTree's JavaScript object semantics and three.js's math, so the
//! result matches the loader's.
//!
//! [`parse`] returns the loader's output as plain data: the nodes in
//! `traverse()` order with their local transforms, shared geometry, material
//! and texture tables, and the clips' keyframe tracks by name.
mod animation;
mod binary;
mod canonical;
mod geometry;
pub(crate) mod js;
pub(crate) mod math;
mod scene;
mod text;

pub use canonical::canonical;

use crate::geometry::BufferGeometry;
use crate::math::{Color, Matrix4, Quaternion, Vector2, Vector3};
use crate::{Error, Result};

/// FBXLoader's options.
#[derive(Clone, Debug)]
pub struct Options {
    /// `trimAnimationClips`.
    pub trim_animation_clips: bool,
    /// The texture path ( `resourcePath`, or the model URL's base ).
    pub path: String,
    /// `window.innerWidth` and `innerHeight`, for cameras without an aspect.
    pub window_size: (f64, f64),
}
impl Default for Options {
    fn default() -> Self {
        Self {
            trim_animation_clips: false,
            path: String::new(),
            window_size: (1.0, 1.0),
        }
    }
}

/// The loaded asset.
#[derive(Clone, Debug, Default)]
pub struct FbxScene {
    /// The returned group and its descendants in `traverse()` order; index 0
    /// is the root.
    pub nodes: Vec<FbxNode>,
    pub geometries: Vec<FbxGeometry>,
    pub materials: Vec<FbxMaterial>,
    pub textures: Vec<FbxTexture>,
    pub clips: Vec<FbxClip>,
    /// `userData.unitScaleFactor`.
    pub unit_scale_factor: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct FbxNode {
    pub name: String,
    /// The FBX object id ( `ID` ), for models.
    pub id: Option<f64>,
    pub parent: Option<usize>,
    pub kind: FbxNodeKind,
    pub position: Vector3,
    pub quaternion: Quaternion,
    pub scale: Vector3,
    /// `matrixWorld` as the loader leaves it.
    pub matrix_world: Matrix4,
    pub cast_shadow: bool,
}

#[derive(Clone, Debug)]
pub enum FbxNodeKind {
    Group,
    Object3D,
    Bone,
    Mesh(FbxMesh),
    /// A NURBS curve: its geometry and LineBasicMaterial.
    Line {
        geometry: Option<usize>,
        material: usize,
    },
    PerspectiveCamera {
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
    },
    PointLight {
        color: Color,
        intensity: f64,
        distance: f64,
        decay: f64,
    },
    DirectionalLight {
        color: Color,
        intensity: f64,
        target: Option<usize>,
    },
    SpotLight {
        color: Color,
        intensity: f64,
        distance: f64,
        angle: f64,
        penumbra: f64,
        decay: f64,
        target: Option<usize>,
    },
    AmbientLight {
        color: Color,
        intensity: f64,
    },
}

#[derive(Clone, Debug)]
pub struct FbxMesh {
    pub geometry: usize,
    /// The mesh's material, or materials when `multi_material`.
    pub materials: Vec<usize>,
    pub multi_material: bool,
    /// `morphTargetDictionary`: target name and index.
    pub morph_targets: Vec<(String, usize)>,
    pub skin: Option<FbxSkin>,
}

/// A SkinnedMesh's skeleton and bind matrix ( attached binding ).
#[derive(Clone, Debug)]
pub struct FbxSkin {
    /// Bone node indices ( `None` for a cluster without a bone ).
    pub bones: Vec<Option<usize>>,
    pub bone_inverses: Vec<Matrix4>,
    pub bind_matrix: Matrix4,
}

#[derive(Clone, Debug)]
pub struct FbxGeometry {
    pub geometry: BufferGeometry,
    /// The `position` morph attributes' names.
    pub morph_names: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbxMaterialKind {
    Phong,
    Lambert,
    LineBasic,
}

#[derive(Clone, Debug)]
pub struct FbxMaterial {
    pub kind: FbxMaterialKind,
    pub name: Option<String>,
    pub color: Color,
    pub specular: Color,
    pub shininess: f64,
    pub emissive: Color,
    pub emissive_intensity: f64,
    pub opacity: f64,
    pub transparent: bool,
    pub vertex_colors: bool,
    pub bump_scale: f64,
    pub displacement_scale: f64,
    pub reflectivity: f64,
    pub map: Option<usize>,
    pub alpha_map: Option<usize>,
    pub ao_map: Option<usize>,
    pub bump_map: Option<usize>,
    pub displacement_map: Option<usize>,
    pub emissive_map: Option<usize>,
    pub env_map: Option<usize>,
    pub normal_map: Option<usize>,
    pub specular_map: Option<usize>,
}

/// Where a texture's image comes from.
#[derive(Clone, Debug, PartialEq)]
pub enum FbxImage {
    /// The placeholder `new Texture()` for an unresolved file name.
    None,
    /// A URL: the texture path joined with the file name, or a data URI.
    Url(String),
    /// Image data embedded in the file ( a Blob URL in the original ).
    Embedded { mime: String, data: Vec<u8> },
}

#[derive(Clone, Debug)]
pub struct FbxTexture {
    pub name: Option<String>,
    pub image: FbxImage,
    /// RepeatWrapping when true, ClampToEdgeWrapping otherwise.
    pub repeat_s: bool,
    pub repeat_t: bool,
    pub repeat: Vector2,
    pub offset: Vector2,
    /// SRGBColorSpace.
    pub srgb: bool,
    /// EquirectangularReflectionMapping ( a reflection map ).
    pub equirectangular: bool,
}

#[derive(Clone, Debug)]
pub struct FbxClip {
    pub name: String,
    pub duration: f64,
    pub tracks: Vec<FbxTrack>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbxTrackKind {
    Vector,
    Quaternion,
    Number,
}

/// A keyframe track: `nodeName.property`, with Float32 times and values.
#[derive(Clone, Debug)]
pub struct FbxTrack {
    pub name: String,
    pub kind: FbxTrackKind,
    pub times: Vec<f32>,
    pub values: Vec<f32>,
}
impl FbxTrack {
    /// The track's node name and property ( PropertyBinding.parseTrackName
    /// for the names the loader writes ).
    pub fn binding(&self) -> (&str, &str) {
        self.name.split_once('.').unwrap_or((&self.name, ""))
    }
}

impl FbxScene {
    /// PropertyBinding.findNode( root, name ): the root, or the first
    /// depth-first descendant with the name.
    pub fn find_node(&self, name: &str) -> Option<usize> {
        if name.is_empty() || name == "." || self.nodes.first()?.name == name {
            return Some(0);
        }
        fn search(scene: &FbxScene, children: &[usize], name: &str) -> Option<usize> {
            for &c in children {
                if scene.nodes[c].name == name {
                    return Some(c);
                }
                let grand: Vec<usize> = (0..scene.nodes.len())
                    .filter(|&i| scene.nodes[i].parent == Some(c))
                    .collect();
                if let Some(r) = search(scene, &grand, name) {
                    return Some(r);
                }
            }
            None
        }
        let roots: Vec<usize> = (0..self.nodes.len())
            .filter(|&i| self.nodes[i].parent == Some(0))
            .collect();
        search(self, &roots, name)
    }
}

/// FBXLoader.parse( buffer, path ).
pub fn parse(bytes: &[u8], options: &Options) -> Result<FbxScene> {
    let tree = if binary::is_binary(bytes) {
        binary::parse(bytes)?
    } else {
        let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes);
        let text = String::from_utf8_lossy(bytes);
        if !text::is_ascii(&text) {
            return Err(Error::Asset("THREE.FBXLoader: Unknown format.".into()));
        }
        let version = text::version(&text)?;
        if version < 7000.0 {
            return Err(Error::Asset(format!(
                "THREE.FBXLoader: FBX version not supported, FileVersion: {version}"
            )));
        }
        text::parse(&text)?
    };
    scene::parse(&tree, options)
}
