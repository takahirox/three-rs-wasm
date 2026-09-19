use three_rs_wasm::tsl::*;

#[test]
fn compute_graphs_validate_integer_types_stages_and_storage_coordinates() {
    use compute::texture_store_wgsl;
    let index = instance_index();
    let coordinate = uvec2(index.modulo(uint(9)), index / uint(9));
    let color = vec4(splat(coordinate.x().to_float(), Type::Vec3), float(1.0));
    assert!(texture_store_wgsl(72, &coordinate, &color).is_ok());
    assert!(texture_store_wgsl(0, &coordinate, &color).is_err());
    for invalid in [
        uv(),
        uint(0),
        uvec2(float(0.0), uint(0)),
        coordinate.clone() + float(1.0),
    ] {
        assert!(texture_store_wgsl(72, &invalid, &color).is_err());
    }
    for invalid in [
        uv(),
        position_geometry(),
        Texture::Input.sample(uv()),
        uint(1) + float(1.0),
        uint(1).sin(),
        uniform(0, Type::Uint),
        coordinate.swizzle("xyz"),
        uint(1).pow(uint(2)),
    ] {
        assert!(texture_store_wgsl(72, &coordinate, &invalid).is_err());
    }
    assert!(effect_wgsl(&instance_index().to_float()).is_err());
    assert!(
        NodeMaterial::new(Texture::History.sample(uv()))
            .wgsl(0)
            .is_err()
    );
    assert!(effect_wgsl(&Texture::History.sample(uv())).is_ok());
    assert!(effect_wgsl_with_textures(&Texture::External(1).sample(uv()), 1).is_err());
    assert!(effect_wgsl_with_textures(&Texture::External(1).sample(uv()), 2).is_ok());
    // Scalar broadcast stays in the unsigned family until explicitly converted.
    let value = (coordinate.clone() + uint(1)).x().to_float();
    assert!(
        texture_store_wgsl(72, &coordinate, &vec4(splat(value, Type::Vec3), float(1.0))).is_ok()
    );
    assert!(effect_wgsl(&uv().cross(uv())).is_err());
    assert!(effect_wgsl(&uv().dot(splat(float(1.0), Type::Vec3))).is_err());
}

#[test]
fn graph_checks_types_stages_resources_and_shared_expressions() {
    let shared = (uv().x() * uniform(0, Type::Float)).sin();
    let source = NodeMaterial::new(vec3(shared.clone(), shared.clone(), shared))
        .wgsl(0)
        .unwrap();
    assert_eq!(source.matches("sin(").count(), 1);
    for color in [
        uv() + float(1.0),
        uv() + splat(float(1.0), Type::Vec3),
        uv().swizzle("z"),
        float(f32::NAN),
        uniform(16, Type::Float),
        uniform(0, Type::Bool),
        position_geometry(),
        Texture::Input.sample(uv()),
        Texture::External(1).sample(uv()),
        uv().equal(uv()),
        float(0.0).clamp(uv(), float(1.0)),
    ] {
        assert!(NodeMaterial::new(color).wgsl(0).is_err());
    }
    let mut material = NodeMaterial::new(float(1.0));
    material.position = Some(Texture::Map.sample(uv()).rgb());
    assert!(material.wgsl(0).is_err());
    material.position = Some(Texture::Map.sample_level(uv(), float(0.0)).rgb());
    assert!(material.wgsl(0).is_ok());
    assert!(effect_wgsl(&position_geometry()).is_err());
    assert!(effect_wgsl(&Texture::Map.sample(uv())).is_err());
    assert!(effect_wgsl(&Texture::Input.sample_grad(uv(), float(0.0), uv())).is_err());
}

#[test]
fn native_functions_are_typed_and_conflicts_are_rejected() {
    let f = WgslFn::new(
        "scale",
        "fn scale(x:f32)->f32{return x*2.0;}",
        &[Type::Float],
        Type::Float,
    )
    .unwrap();
    assert!(NodeMaterial::new(f.call(&[uv()])).wgsl(0).is_err());
    assert!(NodeMaterial::new(f.call(&[])).wgsl(0).is_err());
    assert!(WgslFn::new("a; injected", "", &[], Type::Float).is_err());
    let other = WgslFn::new(
        "scale",
        "fn scale(x:f32)->f32{return x*3.0;}",
        &[Type::Float],
        Type::Float,
    )
    .unwrap();
    assert!(
        NodeMaterial::new(f.call(&[float(1.0)]) + other.call(&[float(2.0)]))
            .wgsl(0)
            .is_err()
    );
    assert!(gaussian_blur(Texture::Input, uv(), uv(), 33).is_err());
}
