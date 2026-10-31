//! KHR_materials_diffuse_roughness ( r187 ): GLTFLoader's extension and
//! MeshPhysicalMaterial.diffuseRoughness's BRDF_EON.
use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    material::*,
    math::{Color, Vector3},
    renderer::*,
    scene::*,
};

#[test]
fn importer_reads_khr_materials_diffuse_roughness() {
    // r187's DiffuseRoughnessParameterSweep: GLTFLoader makes the materials
    // without the extension MeshStandardMaterials, the others
    // MeshPhysicalMaterials with diffuseRoughnessFactor ( 0 when absent ).
    let bytes = include_bytes!("../web/examples/assets/gltf/DiffuseRoughnessParameterSweep.glb");
    let asset = gltf::Gltf::from_slice(bytes).unwrap();
    let buffers = vec![asset.blob.clone().unwrap()];
    let imported = three_rs_wasm::gltf::import(&asset, &buffers, &[]).unwrap();
    let sources = imported.sources.clone();
    let mut scene = Scene::new();
    let meshes = imported.instantiate(&mut scene).unwrap();
    assert_eq!(meshes.len(), 24);
    let mut standard = 0;
    for (handle, (mesh, primitive)) in meshes.into_iter().zip(sources) {
        let source = asset
            .meshes()
            .nth(mesh)
            .unwrap()
            .primitives()
            .nth(primitive)
            .unwrap()
            .material();
        let expected = source
            .extension_value("KHR_materials_diffuse_roughness")
            .map(|e| e["diffuseRoughnessFactor"].as_f64().unwrap_or(0.0));
        let NodeKind::Mesh(m) = &scene.get(handle).unwrap().kind else {
            panic!("mesh")
        };
        match (&*m.materials[0], expected) {
            (Material::Standard(_), None) => standard += 1,
            (Material::Physical(p), Some(factor)) => {
                assert_eq!(p.diffuse_roughness, factor);
                assert!(p.diffuse_roughness_map.is_none());
            }
            (material, expected) => panic!("{material:?} {expected:?}"),
        }
    }
    assert_eq!(standard, 4);
    // diffuseRoughnessTexture becomes diffuseRoughnessMap, as non-color data;
    // the required extension is supported.
    let json = br#"{"asset":{"version":"2.0"},"extensionsUsed":["KHR_materials_diffuse_roughness"],"extensionsRequired":["KHR_materials_diffuse_roughness"],
        "images":[{"uri":"a.png"}],"textures":[{"source":0}],
        "materials":[{"extensions":{"KHR_materials_diffuse_roughness":{"diffuseRoughnessFactor":0.5,"diffuseRoughnessTexture":{"index":0}}}}],
        "buffers":[{"byteLength":36}],"bufferViews":[{"buffer":0,"byteLength":36}],
        "accessors":[{"bufferView":0,"componentType":5126,"count":3,"type":"VEC3","min":[0,0,0],"max":[1,1,0]}],
        "meshes":[{"primitives":[{"attributes":{"POSITION":0},"material":0}]}],"nodes":[{"mesh":0}],"scenes":[{"nodes":[0]}],"scene":0}"#;
    let asset = gltf::Gltf::from_slice_without_validation(json).unwrap();
    let positions: Vec<u8> = [0f32, 0., 0., 1., 0., 0., 0., 1., 0.]
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    let image = Texture::from_rgba(1, 1, vec![128, 0, 0, 255], false).unwrap();
    let imported = three_rs_wasm::gltf::import_decoded(&asset, &[positions], &[image]).unwrap();
    let mut scene = Scene::new();
    let mesh = imported.instantiate(&mut scene).unwrap()[0];
    let NodeKind::Mesh(m) = &scene.get(mesh).unwrap().kind else {
        panic!("mesh")
    };
    let Material::Physical(p) = &*m.materials[0] else {
        panic!("physical")
    };
    assert_eq!(p.diffuse_roughness, 0.5);
    assert!(!p.diffuse_roughness_map.as_ref().unwrap().srgb);
}

/// r187's BRDF_EON ( lights_physical_pars_fragment ) in f64.
fn brdf_eon(l: [f64; 3], v: [f64; 3], n: [f64; 3], rho: [f64; 3], r: f64) -> [f64; 3] {
    use std::f64::consts::PI;
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let fon = |mu: f64, a: f64| {
        let c = 1.0 - mu;
        a * (1.0
            + r * c * (0.0571085289 + c * (0.491881867 + c * (-0.332181442 + c * 0.0714429953))))
    };
    let (nl, nv) = (dot(n, l).clamp(0., 1.), dot(n, v).clamp(0., 1.));
    let s = dot(l, v) - nl * nv;
    let s_over_t = if s > 0. { s / nl.max(nv).max(1e-7) } else { s };
    let a = 1.0 / (1.0 + (0.5 - 2.0 / (3.0 * PI)) * r);
    let average = a * (1.0 + (2.0 / 3.0 - 28.0 / (15.0 * PI)) * r);
    rho.map(|c| {
        let single = c / PI * a * (1.0 + r * s_over_t);
        let multi = c * c * average / (1.0 - c * (1.0 - average)).max(1e-7);
        single
            + multi / PI * (1.0 - fon(nv, a)).max(1e-7) * (1.0 - fon(nl, a)).max(1e-7)
                / (1.0 - average).max(1e-7)
    })
}

#[test]
fn diffuse_roughness_lights_with_brdf_eon() {
    // A directional light behind a camera at a grazing view of a plane:
    // diffuseRoughness ( × its map's red channel ) replaces the Lambertian
    // lobe by BRDF_EON, whose back-scattering brightens the rough render by
    // ( EON − ρ / π ) × E over the Lambertian one.
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        aspect: 1.,
        ..Default::default()
    })));
    let eye = [2., 0., 1.];
    scene.get_mut(camera).unwrap().position = Vector3::from_array(eye);
    scene.look_at(camera, Vector3::ZERO).unwrap();
    let intensity = 3.0;
    let light = scene.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity,
        target: Vector3::ZERO,
    }));
    scene.get_mut(light).unwrap().position = Vector3::new(2., 0., 1.2);
    let rho = [0.45, 0.28, 0.21];
    let plane = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(8., 8., 1, 1).unwrap()),
        Arc::new(Material::Physical(MeshPhysicalMaterial::default())),
    )));
    let target = RenderTarget::new(&renderer.device, 1, 1).unwrap();
    let mut render = |roughness: f64, map: Option<u8>| {
        let mut m = MeshPhysicalMaterial::default();
        m.base.properties.color = Color::linear(rho[0], rho[1], rho[2]);
        m.base.roughness = 0.95;
        m.base.metalness = 0.;
        m.diffuse_roughness = roughness;
        m.diffuse_roughness_map = map.map(|red| {
            let mut t = Texture::from_rgba(1, 1, vec![red, 255, 255, 255], false).unwrap();
            t.flip_y = false;
            Arc::new(t)
        });
        if let NodeKind::Mesh(mesh) = &mut scene.get_mut(plane).unwrap().kind {
            mesh.materials[0] = Arc::new(Material::Physical(m));
        }
        renderer.render(&mut scene, camera, &target).unwrap();
        renderer.read_rgba(&target).unwrap()
    };
    let lambert = render(0., None);
    let norm = |v: [f64; 3]| {
        let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        v.map(|c| c / l)
    };
    let (l, v, n) = (norm([2., 0., 1.2]), norm(eye), [0., 0., 1.]);
    let irradiance = intensity * l[2];
    let decode = |c: u8| {
        let c = c as f64 / 255.;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let encode = |c: f64| {
        let c = if c <= 0.0031308 {
            c * 12.92
        } else {
            1.055 * c.powf(1. / 2.4) - 0.055
        };
        (c * 255.).round()
    };
    for (roughness, map, effective) in [(1., None, 1.), (1., Some(128u8), 128. / 255.)] {
        let rough = render(roughness, map);
        let eon = brdf_eon(l, v, n, rho, effective);
        for i in 0..3 {
            let expected =
                encode(decode(lambert[i]) + (eon[i] - rho[i] / std::f64::consts::PI) * irradiance);
            assert!(
                (rough[i] as f64 - expected).abs() <= 1.,
                "{roughness} {map:?} channel {i}: {rough:?} {lambert:?} expected {expected}"
            );
        }
        assert!(rough[0] > lambert[0] + 10, "{rough:?} {lambert:?}");
    }
}
