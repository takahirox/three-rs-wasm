//! Shader sources and lookup tables the engine embeds, for code built on it
//! that runs the same stages ( the examples gallery's ports ).

/// Tone mapping and the sRGB transfer of the output pass.
pub const OUTPUT_WGSL: &str = include_str!("shaders/output.wgsl");
/// LightProbeGrid's spherical-harmonics projection.
pub const PROBE_SH_VS: &str = include_str!("shaders/light_probe_grid/sh_vs.wgsl");
pub const PROBE_SH_FS: &str = include_str!("shaders/light_probe_grid/sh_fs.wgsl");
/// LightProbeGrid's repacking passes ( a fullscreen vertex stage and one
/// fragment stage per coefficient band texture ).
pub const PROBE_REPACK_VS: &str = include_str!("shaders/light_probe_grid/repack_vs.wgsl");
pub const PROBE_REPACK_FS: [&str; 7] = [
    include_str!("shaders/light_probe_grid/repack_0_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_1_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_2_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_3_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_4_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_5_fs.wgsl"),
    include_str!("shaders/light_probe_grid/repack_6_fs.wgsl"),
];
/// The rect area light LTC tables, half- and single-float.
pub const LTC_R186: &[u8] = include_bytes!("shaders/ltc-r186.bin");
pub const LTC_R186_F32: &[u8] = include_bytes!("shaders/ltc-r186-f32.bin");
/// TSL's noise, FXAA and MaterialX functions in WGSL.
pub const TSL_NOISE_WGSL: &str = include_str!("tsl/noise.wgsl");
pub const TSL_FXAA_WGSL: &str = include_str!("tsl/fxaa.wgsl");
pub const TSL_MATERIALX_WGSL: &str = include_str!("tsl/materialx.wgsl");
