use three_rs_wasm::tsl::*;

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
