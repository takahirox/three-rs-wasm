//! webgpu_lightprobes_complex's scene: two rooms joined by a doorway, a
//! warm and a cool shadow-casting point light and two LightProbeGrid volumes
//! ( 8 × 5 × 8 each, one per room ). The page bakes a direct pass and three
//! bounce passes, the left volume then the right: each bake adds the
//! volume's bounce grid to the scene, and after the direct pass both volumes
//! light the other's captures. Where grids overlap the most recently created
//! one lights the fragment. The lit materials are shadowed, unshadowed and
//! the back-side room shell. Every build runs the WGSL three.js r187
//! generates for the page ( `lightprobes_complex/` ); the bake and the rest
//! of the program are in `lightprobes.rs`.
use super::{Kind, Light, Material, MeshSetup, Names, Setup, Stage, Volume, standard};
use crate::{Result, geometry::*, math::*};
use std::f64::consts::PI;

macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!($name, ".wgsl"))
    };
}
/// A build's vertex and fragment WGSL.
fn shaders(stage: Stage, kind: Kind) -> (&'static str, &'static str) {
    match (stage, kind) {
        (Stage::Main0, Kind::Plain) => (wgsl!("plain_vs"), wgsl!("plain_fs")),
        (Stage::Main0, Kind::Shadowed) => (wgsl!("shadowed_vs"), wgsl!("shadowed_fs")),
        (Stage::Main0, Kind::Back) => (wgsl!("shadowed_vs"), wgsl!("back_fs")),
        (Stage::Main, Kind::Plain) => (wgsl!("gi_plain_vs"), wgsl!("gi_plain_fs")),
        (Stage::Main, Kind::Shadowed) => (wgsl!("gi_vs"), wgsl!("gi_fs")),
        (Stage::Main, Kind::Back) => (wgsl!("gi_vs"), wgsl!("gi_back_fs")),
        (Stage::Bake1, Kind::Plain) => (wgsl!("bake_plain_vs"), wgsl!("bake_plain_fs")),
        (Stage::Bake1, Kind::Shadowed) => (wgsl!("bake_vs"), wgsl!("bake_fs")),
        (Stage::Bake1, Kind::Back) => (wgsl!("bake_vs"), wgsl!("bake_back_fs")),
        (Stage::Bake2, Kind::Plain) => (wgsl!("bounce_plain_vs"), wgsl!("bounce_plain_fs")),
        (Stage::Bake2, Kind::Shadowed) => (wgsl!("bounce_vs"), wgsl!("bounce_fs")),
        (Stage::Bake2, Kind::Back) => (wgsl!("bounce_vs"), wgsl!("bounce_back_fs")),
    }
}
/// A build's uniform numbering: the unshadowed variants' lights take four
/// uniforms each, the shadowed ones' seventeen.
fn names(stage: Stage, kind: Kind) -> Names {
    let plain = kind == Kind::Plain;
    let (lights, grids) = match (stage.bake(), plain) {
        (false, true) => ([10, 14], 19),
        (false, false) => ([10, 27], 43),
        (true, true) => ([12, 16], 20),
        (true, false) => ([12, 28], 44),
    };
    Names::new(stage, lights, grids)
}
/// The page's setup: the camera at z 16, light intensity 30, the lights'
/// shadows with bias -0.002, normalBias 0.05 and radius 6.
pub(super) fn setup() -> Result<Setup> {
    let light = |x, color| Light {
        position: Vector3::new(x, 4.5, 0.),
        color,
        bias: -0.002,
        normal_bias: 0.05,
        radius: 6.,
    };
    let volume = |x| Volume {
        center: Vector3::new(x, 2.5, 0.),
        size: Vector3::new(8., 5., 8.),
    };
    Ok(Setup {
        camera_z: 16.,
        intensity: 30.,
        lights: vec![light(-4., 0xffaa44), light(4., 0x88bbff)],
        volumes: vec![volume(-4.), volume(4.)],
        meshes: rooms()?,
        kinds: &[Kind::Shadowed, Kind::Plain, Kind::Back],
        stages: &[Stage::Main0, Stage::Main, Stage::Bake1, Stage::Bake2],
        shaders,
        names,
    })
}
/// The two rooms of webgpu_lightprobes_complex.
fn rooms() -> Result<Vec<MeshSetup>> {
    let white = standard(0xeeeeee);
    let divider = standard(0xcccccc);
    let wood = standard(0x886644);
    let t = |x, y, z| Matrix4::from_translation(Vector3::new(x, y, z));
    let ry = Matrix4::from_rotation_y;
    let (lit, plain) = (Kind::Shadowed, Kind::Plain);
    let gap = 1.25;
    let column = || CylinderGeometry::build(0.3, 0.3, 4., 16, 1, false, 0., 2. * PI);
    let mut meshes = vec![
        (
            BoxGeometry::build(16., 5., 8.)?,
            t(0., 2.5, 0.),
            standard(0xcccccc),
            Kind::Back,
            false,
            false,
        ),
        (
            PlaneGeometry::build(8., 5., 1, 1)?,
            t(-7.99, 2.5, 0.) * ry(PI / 2.),
            standard(0xdd2200),
            plain,
            false,
            false,
        ),
        (
            PlaneGeometry::build(8., 5., 1, 1)?,
            t(7.99, 2.5, 0.) * ry(-PI / 2.),
            standard(0x0044ff),
            plain,
            false,
            false,
        ),
        (
            BoxGeometry::build(0.3, 5., 4. - gap)?,
            t(0., 2.5, -(gap + (4. - gap) / 2.)),
            divider,
            lit,
            true,
            true,
        ),
        (
            BoxGeometry::build(0.3, 5., 4. - gap)?,
            t(0., 2.5, gap + (4. - gap) / 2.),
            divider,
            lit,
            true,
            true,
        ),
        (
            BoxGeometry::build(0.3, 1.2, gap * 2.)?,
            t(0., 4.4, 0.),
            divider,
            lit,
            true,
            true,
        ),
    ];
    for x in [-6., -2.] {
        for z in [-2.5, 2.5] {
            meshes.push((column()?, t(x, 2., z), white, lit, true, false));
        }
    }
    meshes.push((
        BoxGeometry::build(2.4, 0.12, 1.4)?,
        t(-4., 1., 0.),
        wood,
        lit,
        true,
        false,
    ));
    for x in [-5.05, -2.95] {
        for z in [-0.55, 0.55] {
            meshes.push((
                BoxGeometry::build(0.1, 1., 0.1)?,
                t(x, 0.5, z),
                wood,
                plain,
                true,
                false,
            ));
        }
    }
    meshes.extend([
        (
            SphereGeometry::build(0.35, 32, 32)?,
            t(-4., 1.41, 0.),
            Material {
                color: 0xffd700,
                roughness: 0.4,
                metalness: 0.3,
            },
            lit,
            true,
            true,
        ),
        (
            BoxGeometry::build(1.5, 0.5, 1.)?,
            t(-7., 0.25, 0.),
            white,
            lit,
            true,
            false,
        ),
        (
            BoxGeometry::build(1., 1., 1.)?,
            t(-7., 0.5, 0.),
            white,
            lit,
            true,
            false,
        ),
        (
            BoxGeometry::build(0.5, 1.5, 1.)?,
            t(-7., 0.75, 0.),
            white,
            lit,
            true,
            false,
        ),
    ]);
    for x in [2., 6.] {
        for z in [-2.5, 2.5] {
            meshes.push((column()?, t(x, 2., z), white, lit, true, false));
        }
    }
    meshes.extend([
        (
            BoxGeometry::build(0.8, 1.2, 0.8)?,
            t(4., 0.6, 0.),
            white,
            lit,
            true,
            false,
        ),
        (
            TorusKnotGeometry::build(0.3, 0.1, 64, 16, 2, 3)?,
            t(4., 1.65, 0.),
            Material {
                color: 0xff44aa,
                roughness: 0.5,
                metalness: 0.2,
            },
            lit,
            true,
            false,
        ),
        (
            CylinderGeometry::build(0., 0.4, 2.5, 5, 1, false, 0., 2. * PI)?,
            t(6.5, 1.25, -1.5),
            white,
            lit,
            true,
            false,
        ),
    ]);
    Ok(meshes)
}
