use super::*;
use crate::{attribute::BufferAttribute, tsl::sprites::*};
fn lucy(bytes: &[u8]) -> Result<BufferGeometry> {
    let marker = b"end_header\n";
    let offset = bytes
        .windows(marker.len())
        .position(|s| s == marker)
        .ok_or(Error::Invalid("PLY header"))?
        + marker.len();
    let header = std::str::from_utf8(&bytes[..offset]).map_err(|_| Error::Invalid("PLY header"))?;
    if !header.contains("format binary_little_endian 1.0") {
        return Err(Error::Invalid("Lucy PLY format"));
    }
    let count = |kind: &str| -> Result<usize> {
        header
            .lines()
            .find_map(|s| s.strip_prefix(&format!("element {kind} ")))
            .and_then(|s| s.parse().ok())
            .ok_or(Error::Invalid("PLY count"))
    };
    let vertices = count("vertex")?;
    let faces = count("face")?;
    if bytes.len() != offset + vertices * 12 + faces * 13 {
        return Err(Error::Invalid("Lucy PLY payload"));
    }
    let mut g = BufferGeometry::default();
    let positions = bytes[offset..offset + vertices * 12]
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| f32::from_le_bytes(*b))
        .collect();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(positions, 3, false)?),
    );
    let mut indices = Vec::with_capacity(faces * 3);
    for f in bytes[offset + vertices * 12..].as_chunks::<13>().0 {
        if f[0] != 3 {
            return Err(Error::Invalid("Lucy PLY non-triangle"));
        }
        for b in f[1..].as_chunks::<4>().0 {
            indices.push(u32::from_le_bytes(*b));
        }
    }
    g.set_index(Some(indices));
    g.compute_vertex_normals()?;
    Ok(g)
}
impl Demo {
    pub(super) async fn smoke(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        self.params[..3].copy_from_slice(&[1.0, 1.0, 2.0]);
        s.fog = Some(Fog::Exp2 {
            color: Color::BLACK,
            density: 0.025,
        });
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.25,
        }));
        let h = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 800.0,
            // Upstream target is not attached to the scene; its world matrix stays identity.
            target: Vector3::ZERO,
            angle: 0.5,
            penumbra: 0.7,
            distance: 0.0,
            decay: 2.0,
            ies: false,
        }));
        s.get_mut(h)?.position = Vector3::new(10.0, 15.0, 8.0);
        let g = lucy(&fetch(&format!("{ASSETS}/models/ply/binary/Lucy100k.ply")).await?)?;
        let h = mesh(
            s,
            Arc::new(g),
            Material::Standard(MeshStandardMaterial {
                energy_conservation: true,
                ..Default::default()
            }),
        );
        let n = s.get_mut(h)?;
        n.position.y = 4.0;
        n.quaternion = Quaternion::from_rotation_y(std::f64::consts::PI);
        n.scale = Vector3::splat(0.005);
        let mut m = MeshStandardMaterial {
            roughness: 1.0,
            energy_conservation: true,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0x444444);
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(100.0, 100.0, 1, 1)?),
            Material::Standard(m),
        );
        s.get_mut(h)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        let mut tex = decode_texture_image(
            &fetch(&format!("{ASSETS}/textures/opengameart/smoke1.png")).await?,
        )
        .await?;
        tex.srgb = true;
        tex.mipmap_filter = Some(Filter::Linear);
        let tex = r.upload_texture(&Arc::new(tex))?;
        // r187's range() nodes, hashed per instance with the Sprite's id ( Object3D 14 on
        // the page ): lifeRange, offsetRange, scaleRange and rotateRange are range
        // nodes 0 to 3.
        let ranged = |id, low, high| tsl::range(14, id, low, high);
        let life_range = ranged(0, [0.1; 4], [1.0; 4]).x();
        let scaled = (time() + float(20.0)) * float(0.2);
        let lifetime = (scaled.clone() * life_range.clone()).fract();
        let life = lifetime.clone() / life_range;
        let rotate = scaled * ranged(3, [0.1; 4], [2.0; 4]).x();
        let p = uv() - float(0.5);
        let coord = vec2(
            rotate.cos() * p.x() - rotate.sin() * p.y(),
            rotate.sin() * p.x() + rotate.cos() * p.y(),
        ) + float(0.5);
        let alpha = tsl::Texture::External(0)
            .sample(vec2(coord.x(), float(1.0) - coord.y()))
            .swizzle("w")
            * life.smoothstep(float(0.0), float(0.3))
            * life.smoothstep(float(1.0), float(0.7));
        let soft = vp::soft_particles(
            alpha.clone(),
            uniform(1, Type::Vec4).y(),
            uniform(1, Type::Vec4).swizzle("z"),
            float(0.1),
            float(100.0),
        );
        let color = mix(
            rgb(0x6f6f6f),
            rgb(0x303030),
            (position_local().y() * float(0.1)).clamp(float(0.0), float(1.0)),
        );
        let mut sprite =
            SpriteNodeMaterial::new(vec4(color, mix(alpha, soft, uniform(1, Type::Float))));
        sprite.position =
            ranged(1, [-2.0, 0.0, -2.0, 0.0], [2.0, 7.0, 2.0, 0.0]).rgb() * lifetime.clone();
        sprite.scale = ranged(2, [6.0; 4], [8.0; 4]).x() * lifetime.max(float(0.3));
        let mut m = sprite.build(r, &[], &[(&tex.view, &tex.sampler)]).await?;
        m.properties.depth_write = false;
        let mut g = PlaneGeometry::build(1.0, 1.0, 1, 1)?;
        g.instance_count = Some(50);
        let h = mesh(s, Arc::new(g), Material::Shader(m));
        s.get_mut(h)?.frustum_culled = false;
        self.objects.push(h);
        Ok(())
    }
}
