//! Pinned buffergeometry and rawshader scenes. Both share resident geometry
//! between Phong back/front passes; RawShaderMaterial defaults to a single pass.
use crate::{
    Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;

pub(super) async fn create(
    scene: &mut Scene,
    camera: Object3D,
    aspect: f64,
    raw: bool,
    renderer: &crate::renderer::Renderer,
) -> Result<Vec<Object3D>> {
    scene.background = if raw {
        Color::linear(16.0 / 255.0, 16.0 / 255.0, 16.0 / 255.0)
    } else {
        Color::from_hex(0x050505)
    };
    scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: if raw { 50.0 } else { 27.0 },
        aspect,
        near: 1.0,
        far: if raw { 10.0 } else { 3500.0 },
        ..Default::default()
    }));
    scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, if raw { 2.0 } else { 2750.0 });
    let mut seed = 186_u32;
    let mut random = || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        seed as f64 / 4294967296.0
    };
    let mut geometry = BufferGeometry::default();
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();
    let material = if raw {
        let mut bytes = Vec::new();
        let mut uv = Vec::new();
        for _ in 0..600 {
            let x = random() - 0.5;
            positions.extend([x as f32, (random() - 0.5) as f32, (random() - 0.5) as f32]);
            // Preserve the local x varying through the existing UV channel.
            uv.extend([x as f32, 0.0]);
            for _ in 0..4 {
                bytes.push((random() * 255.0) as u8);
            }
        }
        geometry.set_attribute("uv", Attribute::F32(BufferAttribute::new(uv, 2, false)?));
        geometry.set_attribute(
            "color",
            Attribute::U8(BufferAttribute::new(bytes, 4, true)?),
        );
        let program = crate::shader::ShaderProgram::new(
            renderer,
            r#"
fn deform(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{return position;}
fn shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>{
 // WebGL clamps source values before blending into its UNORM framebuffer.
 return clamp(vec4(base.r+sin(surface.uv.x*10.0+u.custom[0].x)*0.5,base.gba),vec4(0.0),vec4(1.0));
}"#,
            &[],
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.properties.fog = false;
        Material::Shader(m)
    } else {
        scene.fog = Some(Fog::Linear {
            color: scene.background,
            near: 2000.0,
            far: 3500.0,
        });
        scene.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xcccccc),
            intensity: 1.0,
        }));
        for (position, intensity) in [(Vector3::ONE, 1.5), (Vector3::new(0.0, -1.0, 0.0), 4.5)] {
            let light = scene.insert(NodeKind::Light(Light::Directional {
                color: Color::WHITE,
                intensity,
                target: Vector3::ZERO,
            }));
            scene.get_mut(light)?.position = position;
        }
        for _ in 0..160000 {
            let center = Vector3::new(
                random() * 800.0 - 400.0,
                random() * 800.0 - 400.0,
                random() * 800.0 - 400.0,
            );
            let vertices = std::array::from_fn::<_, 3, _>(|_| {
                center
                    + Vector3::new(
                        random() * 12.0 - 6.0,
                        random() * 12.0 - 6.0,
                        random() * 12.0 - 6.0,
                    )
            });
            let normal = (vertices[2] - vertices[1])
                .cross(vertices[0] - vertices[1])
                .normalize_or_zero();
            let color = center / 800.0 + Vector3::splat(0.5);
            let alpha = random() as f32;
            for p in vertices {
                positions.extend(p.as_vec3().to_array());
                normals.extend(normal.as_vec3().to_array());
                colors.extend([color.x as f32, color.y as f32, color.z as f32, alpha]);
            }
        }
        geometry.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(normals, 3, false)?),
        );
        geometry.set_attribute(
            "color",
            Attribute::F32(BufferAttribute::new(colors, 4, false)?),
        );
        let mut m = MeshPhongMaterial::default();
        m.properties.color = Color::from_hex(0xd5d5d5);
        m.specular = Color::WHITE;
        m.shininess = 250.0;
        Material::Phong(m)
    };
    geometry.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(positions, 3, false)?),
    );
    geometry.compute_bounding_sphere()?;
    let geometry = Arc::new(geometry);
    let mut objects = Vec::new();
    let sides: &[Side] = if raw {
        &[Side::Double]
    } else {
        &[Side::Back, Side::Front]
    };
    for &side in sides {
        let mut material = material.clone();
        let p = material.properties_mut();
        p.side = side;
        p.transparent = true;
        p.vertex_colors = true;
        objects.push(scene.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(material),
        ))));
    }
    Ok(objects)
}

pub(super) fn update(
    scene: &mut Scene,
    objects: &[Object3D],
    seconds: f64,
    raw: bool,
) -> Result<()> {
    for &object in objects {
        let node = scene.get_mut(object)?;
        node.quaternion = Quaternion::from_euler(
            glam::EulerRot::XYZ,
            if raw { 0.0 } else { seconds * 0.25 },
            seconds * 0.5,
            0.0,
        );
        if raw
            && let NodeKind::Mesh(mesh) = &mut node.kind
            && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
        {
            m.uniforms[0][0] = (seconds * 5.0) as f32;
        }
    }
    Ok(())
}
