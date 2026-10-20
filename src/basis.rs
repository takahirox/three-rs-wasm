//! KTX2 / Basis Universal transcoding, installed by the application as
//! three.js's loaders take a KTX2Loader ( `GLTFLoader.setKTX2Loader`, a page's
//! own `KTX2Loader` ): `install` makes the transcoders reachable. The loaders
//! read a file's header with `basisu` themselves; the codecs ( ETC1S, UASTC,
//! ASTC, BC7, … ) are reached only through `install`, so a split build links
//! them into the parts of the pages that install them ( tools/gallery/split/ ),
//! as three.js fetches basis_transcoder.wasm only for those pages.
use crate::{Error, Result};
use basisu::{DecodeFlags, TargetFormat, Transcoder};
use std::sync::OnceLock;

type Level = fn(
    &Transcoder<'_>,
    u32,
    TargetFormat,
    DecodeFlags,
) -> std::result::Result<Vec<u8>, basisu::Error>;
type Image = fn(
    &Transcoder<'_>,
    u32,
    u32,
    u32,
    TargetFormat,
    DecodeFlags,
) -> std::result::Result<Vec<u8>, basisu::Error>;

struct Transcoders {
    level: Level,
    image: Image,
}

static INSTALLED: OnceLock<Transcoders> = OnceLock::new();

/// Make KTX2 and Basis textures loadable.
pub fn install() {
    // black_box: LTO would see the one value this static ever holds and call
    // the transcoders directly from the loaders, linking them everywhere.
    INSTALLED.get_or_init(|| {
        std::hint::black_box(Transcoders {
            level: |t, level, target, flags| t.transcode(level, target, flags),
            image: |t, level, layer, face, target, flags| {
                t.transcode_image(level, layer, face, target, flags)
            },
        })
    });
}

fn installed() -> Result<&'static Transcoders> {
    INSTALLED.get().ok_or_else(|| {
        Error::Asset(
            "THREE.GLTFLoader: setKTX2Loader must be called before loading KTX2 textures".into(),
        )
    })
}

/// `Transcoder::transcode` through the installed transcoders.
pub(crate) fn transcode(
    t: &Transcoder<'_>,
    level: u32,
    target: TargetFormat,
    flags: DecodeFlags,
) -> Result<Vec<u8>> {
    (installed()?.level)(t, level, target, flags).map_err(|e| Error::Asset(format!("Basis: {e:?}")))
}

/// `Transcoder::transcode_image` through the installed transcoders.
pub(crate) fn transcode_image(
    t: &Transcoder<'_>,
    level: u32,
    layer: u32,
    face: u32,
    target: TargetFormat,
    flags: DecodeFlags,
) -> Result<Vec<u8>> {
    (installed()?.image)(t, level, layer, face, target, flags)
        .map_err(|e| Error::Asset(format!("Basis mip {level}: {e:?}")))
}
