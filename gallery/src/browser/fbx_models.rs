//! FBX models several ports load, parsed at load by the FBXLoader port from
//! the examples' original files.
use super::gltf_viewer::fetch;
use crate::fbx::{self, FbxNodeKind};
use crate::geometry::BufferGeometry;
use crate::{Error, Result};

/// The examples' models/fbx directory.
pub(super) const MODELS: &str = "/web/gallery/assets/fbx-loader/models/fbx";

/// `new FBXLoader().load( 'models/fbx/stanford-bunny.fbx' )`'s
/// `object.children[ 0 ].geometry`.
pub(super) async fn stanford_bunny() -> Result<BufferGeometry> {
    let bytes = fetch(&format!("{MODELS}/stanford-bunny.fbx")).await?;
    let scene = fbx::parse(&bytes, &fbx::Options::default())?;
    let child = scene
        .nodes
        .iter()
        .find(|n| n.parent == Some(0))
        .ok_or(Error::Invalid("stanford-bunny.fbx has no children"))?;
    match &child.kind {
        FbxNodeKind::Mesh(m) => Ok(scene.geometries[m.geometry].geometry.clone()),
        _ => Err(Error::Invalid(
            "stanford-bunny.fbx's first child is not a mesh",
        )),
    }
}
