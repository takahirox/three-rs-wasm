use serde_json::json;
use three_rs_wasm::compression::*;
#[test]
fn original_ktx2_etc1s_and_uastc_decode_to_visible_rgba() {
    for bytes in [
        include_bytes!("fixtures/ktx2/2d_etc1s.ktx2").as_slice(),
        include_bytes!("fixtures/ktx2/2d_uastc.ktx2").as_slice(),
    ] {
        let image = decode_basis(bytes, true).unwrap();
        assert_eq!(
            image.rgba.len(),
            image.width as usize * image.height as usize * 4
        );
        assert!(image.rgba.chunks_exact(4).any(|p| p[0] > 128));
        assert_eq!((image.width, image.height), (40, 40));
        let first = &image.rgba[..4];
        assert!(
            (170..=195).contains(&first[0])
                && (140..=160).contains(&first[1])
                && first[2] < 25
                && first[3] == 255
        );
    }
    assert!(decode_basis(b"invalid", true).is_err());
}
#[test]
fn meshopt_encoded_positions_are_expanded_and_importable() {
    let points = [-1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    let bytes = bytemuck::cast_slice(&points);
    let mut encoded = vec![0; optimesh::vertexcodec::encode_vertex_buffer_bound(3, 12)];
    let length = optimesh::vertexcodec::encode_vertex_buffer(&mut encoded, bytes, 3, 12);
    encoded.truncate(length);
    let mut source = json!({"asset":{"version":"2.0"},"extensionsUsed":["EXT_meshopt_compression"],"extensionsRequired":["EXT_meshopt_compression"],
 "buffers":[{"byteLength":length},{"byteLength":36}],
 "bufferViews":[{"buffer":1,"byteLength":36,"byteStride":12,"extensions":{"EXT_meshopt_compression":{"buffer":0,"byteOffset":0,"byteLength":length,"byteStride":12,"count":3,"mode":"ATTRIBUTES","filter":"NONE"}}}],
 "accessors":[{"bufferView":0,"componentType":5126,"count":3,"type":"VEC3","min":[-1,0,0],"max":[1,1,0]}],
 "meshes":[{"primitives":[{"attributes":{"POSITION":0}}]}],"nodes":[{"mesh":0}],"scenes":[{"nodes":[0]}],"scene":0});
    let prepared = prepare_gltf(&serde_json::to_vec(&source).unwrap(), &[encoded.clone()]).unwrap();
    let model =
        three_rs_wasm::gltf::import_animated(&prepared.asset, &prepared.buffers, &[]).unwrap();
    assert_eq!(model.triangles, 1);
    source["bufferViews"][0]["extensions"]["EXT_meshopt_compression"]["byteLength"] =
        json!(length + 1);
    assert!(prepare_gltf(&serde_json::to_vec(&source).unwrap(), &[encoded]).is_err());
}
