//! The transpiled MaterialXLoader natively, as webgpu_loader_materialx runs
//! it ( three_tsl::materialx_page ): the ground grid's and each document's
//! WGSL and texture bindings against what three.js r186 builds for the
//! original page ( tests/materialx/: its `getShaderAsync` output ).
use three_tsl::materialx_page::{page_shaders, Document, Shaders, Source};

const SAMPLES: [&str; 12] = [
    "brass_tiled",
    "brick_procedural",
    "carpaint",
    "chrome",
    "copper",
    "gold",
    "jade",
    "marble_solid",
    "metal_brushed",
    "plastic",
    "velvet",
    "wood_tiled",
];
const LOCAL: [&str; 24] = [
    "standard_surface_heightnormal",
    "standard_surface_conditional_if_float",
    "standard_surface_image_transform",
    "standard_surface_color3_vec3_cm_test",
    "standard_surface_rotate2d_test",
    "standard_surface_rotate3d_test",
    "standard_surface_heighttonormal_normal_input",
    "standard_surface_roughness_test",
    "standard_surface_opacity_test",
    "standard_surface_opacity_only_test",
    "standard_surface_specular_test",
    "standard_surface_ior_test",
    "standard_surface_combined_test",
    "standard_surface_texture_opacity_test",
    "standard_surface_transmission_test",
    "standard_surface_transmission_only_test",
    "standard_surface_transmission_rough",
    "standard_surface_thin_film_rainbow_test",
    "standard_surface_thin_film_ior_clamp_test",
    "standard_surface_sheen_test",
    "gltf_pbr_glass_dispersion",
    "open_pbr_surface_velvet",
    "open_pbr_surface_pearl",
    "open_pbr_surface_honey",
];

/// Per document, how the gallery renders it ( Opaque, Transparent or
/// Transmissive ) and its object group's texture bindings: an image's file
/// name, the DFG LUT, the PMREM or the transmission texture.
const BINDINGS: [&str; 36] = [
    "brass_tiled Opaque 1:brass_color.jpg 4:brass_roughness.jpg 6:dfg 8:pmrem",
    "brick_procedural Opaque 1:brick_base_gray.jpg 3:brick_variation_mask.jpg 5:brick_dirt_mask.jpg 7:brick_mask.jpg 10:brick_roughness.jpg 12:dfg 14:brick_normal.jpg 16:pmrem",
    "carpaint Opaque 2:dfg 4:pmrem",
    "chrome Opaque 2:dfg 4:pmrem",
    "copper Opaque 2:dfg 4:pmrem",
    "gold Opaque 2:dfg 4:pmrem",
    "jade Opaque 2:dfg 4:pmrem",
    "marble_solid Opaque 2:dfg 4:pmrem",
    "metal_brushed Opaque 2:dfg 4:pmrem",
    "plastic Opaque 2:dfg 4:pmrem",
    "velvet Opaque 2:dfg 4:pmrem",
    "wood_tiled Opaque 1:wood_color.jpg 4:wood_roughness.jpg 6:dfg 8:pmrem",
    "heightnormal Opaque 1:grid.png 4:dfg 6:pmrem",
    "conditional_if_float Opaque 1:grid.png 4:dfg 6:pmrem",
    "image_transform Opaque 1:grid.png 4:dfg 6:pmrem",
    "color3_vec3_cm_test Opaque 2:dfg 4:grid.png 6:pmrem",
    "rotate2d_test Opaque 1:grid.png 4:dfg 6:pmrem",
    "rotate3d_test Opaque 1:grid.png 4:dfg 6:pmrem",
    "heighttonormal_normal_input Opaque 2:dfg 4:grid.png 6:pmrem",
    "roughness_test Opaque 2:grid.png 4:dfg 6:pmrem",
    "opacity_test Transparent 2:dfg 4:pmrem",
    "opacity_only_test Transparent 2:dfg 4:pmrem",
    "specular_test Opaque 2:dfg 4:pmrem",
    "ior_test Opaque 2:dfg 4:pmrem",
    "combined_test Transparent 2:dfg 4:pmrem",
    "texture_opacity_test Transparent 1:grid.png 4:dfg 6:pmrem",
    "transmission_test Transmissive 2:dfg 4:transmission 6:pmrem",
    "transmission_only_test Transmissive 2:dfg 4:transmission 6:pmrem",
    "transmission_rough Transmissive 2:dfg 4:transmission 6:pmrem",
    "thin_film_rainbow_test Opaque 2:dfg 4:pmrem",
    "thin_film_ior_clamp_test Opaque 2:dfg 4:pmrem",
    "sheen_test Opaque 2:dfg 4:pmrem",
    "gltf_pbr_glass_dispersion Transmissive 2:transmission 4:dfg 6:pmrem",
    "open_pbr_velvet Opaque 2:dfg 4:pmrem",
    "open_pbr_pearl Opaque 2:dfg 4:pmrem",
    "open_pbr_honey Transmissive 2:dfg 4:transmission 6:pmrem",
];

#[test]
fn materialx_shaders_match_r186() {
    // The node system recurses deeply in an unoptimized build.
    std::thread::Builder::new()
        .stack_size(1 << 30)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}

fn short(n: &str) -> String {
    if let Some(r) = n.strip_prefix("standard_surface_") {
        r.to_string()
    } else if let Some(r) = n.strip_prefix("open_pbr_surface_") {
        format!("open_pbr_{r}")
    } else {
        n.to_string()
    }
}

fn describe(name: &str, s: &Shaders) -> String {
    let transmissive = s.textures.iter().any(|b| b.source == Source::Transmission);
    let kind = if transmissive { "Transmissive" } else if s.transparent { "Transparent" } else { "Opaque" };
    let mut out = format!("{name} {kind}");
    for b in &s.textures {
        let source = match &b.source {
            Source::Image(uri) => {
                assert_eq!(b.wrap, (1000, 1000), "{name} {uri}: RepeatWrapping");
                uri.rsplit('/').next().unwrap_or_default().to_string()
            }
            Source::Dfg => "dfg".into(),
            Source::Pmrem => "pmrem".into(),
            Source::Transmission => "transmission".into(),
        };
        out += &format!(" {}:{source}", b.binding);
    }
    out
}

fn run() {
    let sample = "../web/gallery/assets/materialx/resources/Materials/Examples/StandardSurface/";
    let mut names = vec![];
    let mut documents = vec![];
    for n in SAMPLES {
        names.push(n.to_string());
        documents.push(Document {
            text: std::fs::read_to_string(format!("{sample}standard_surface_{n}.mtlx")).unwrap(),
            path: "https://raw.githubusercontent.com/materialx/MaterialX/main/resources/Materials/Examples/StandardSurface/".into(),
        });
    }
    for n in LOCAL {
        names.push(short(n));
        documents.push(Document {
            text: std::fs::read_to_string(format!("../web/gallery/assets/materialx/three/{n}.mtlx")).unwrap(),
            path: "materialx/".into(),
        });
    }
    let page = page_shaders(&documents, 640.0, 400.0).unwrap();
    let mut failures = vec![];
    let compare = |failures: &mut Vec<String>, name: &str, stage: &str, got: &str| {
        let want = std::fs::read_to_string(format!("tests/materialx/{name}_{stage}.wgsl")).unwrap_or_default();
        if got != want {
            let at = got.chars().zip(want.chars()).position(|(a, b)| a != b).unwrap_or(got.len().min(want.len()));
            let ctx = |t: &str| t.chars().skip(at.saturating_sub(80)).take(160).collect::<String>();
            std::fs::create_dir_all("target/materialx").ok();
            std::fs::write(format!("target/materialx/{name}_{stage}.wgsl"), got).ok();
            failures.push(format!("{name} {stage} at {at}:\n  want {:?}\n  got  {:?}", ctx(&want), ctx(got)));
        }
    };
    compare(&mut failures, "grid", "vs", &page.grid.vertex);
    compare(&mut failures, "grid", "fs", &page.grid.fragment);
    for (name, shaders) in names.iter().zip(&page.materials) {
        match shaders {
            Ok(s) => {
                compare(&mut failures, name, "vs", &s.vertex);
                compare(&mut failures, name, "fs", &s.fragment);
                let got = describe(name, s);
                if !BINDINGS.contains(&got.as_str()) {
                    failures.push(format!("bindings {got}"));
                }
            }
            Err(e) => failures.push(format!("{name}: {e}")),
        }
    }
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
}
