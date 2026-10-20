//! The DRACOLoader of an example the shell dispatches itself
//! ( webgl_loader_gltf_avif ). It is a module of its own, so the split build
//! gives the Draco decoder to that page's parts rather than to every page
//! ( tools/gallery/modules.py ); kept out of line, so the shell's call stays a
//! call into this module.
#[inline(never)]
pub(super) fn install() {
    crate::compression::install_draco();
}
