//! The raster fallback WebGLPathTracer draws until the path tracer has its
//! samples and while its quad fades in: renderer.render( scene, camera ) over
//! the page's scene ( the merged X-wing meshes with their page materials, the
//! floor, the blurred environment ) and the GradientEquirectTexture
//! background.
//!
//! The GradientEquirectTexture is the scene's own background map ( half
//! floats of the Float32 gradient ), so it also backs the transmission pass.
use super::scene::{self as page, Side as PageSide};
use super::textures;
use crate::Result;
use crate::attribute::BufferAttribute;
use crate::environment::EnvironmentMap;
use crate::geometry::{Attribute, BufferGeometry};
use crate::material::*;
use crate::math::{Color, Matrix4, Vector3};
use crate::scene::{Mesh, NodeKind, Object3D, Scene};
use std::sync::Arc;

/// The read-back blurred halfs ( GL rows, bottom first ) as the equirect
/// DataTexture the page sets as scene.environment.
pub(crate) fn environment((w, h): (u32, u32), halfs: &[u16]) -> EnvironmentMap {
    let row = w as usize * 4;
    let mut rgba = Vec::with_capacity(halfs.len());
    for y in (0..h as usize).rev() {
        rgba.extend(
            halfs[y * row..(y + 1) * row]
                .iter()
                .map(|&b| half::f16::from_bits(b)),
        );
    }
    EnvironmentMap {
        width: w,
        height: h,
        rgba,
        gpu: None,
    }
}

pub(crate) fn material(m: &page::Material, floor: &[u8]) -> Result<Material> {
    let mut properties = MaterialProperties {
        color: Color(Vector3::from_array(m.color)),
        opacity: m.opacity,
        transparent: m.transparent,
        side: match m.side {
            PageSide::Front => Side::Front,
            PageSide::Double => Side::Double,
        },
        polygon_offset: m.polygon_offset.then_some((1., 0)),
        ..Default::default()
    };
    if m.map {
        // generateRadialFloorTexture: a DataTexture, linear, repeated, no mips.
        let mut t = Texture::from_rgba(1024, 1024, floor.to_vec(), false)?;
        t.flip_y = false;
        t.wrap_s = Wrapping::Repeat;
        t.wrap_t = Wrapping::Repeat;
        t.filter = Filter::Linear;
        t.min_filter = Some(Filter::Linear);
        t.mipmap_filter = None;
        properties.map = Some(Arc::new(t));
    }
    let base = MeshStandardMaterial {
        properties,
        roughness: m.roughness,
        metalness: m.metalness,
        emissive: Color(Vector3::from_array(m.emissive) * m.emissive_intensity),
        ..Default::default()
    };
    Ok(if m.physical {
        Material::Physical(MeshPhysicalMaterial {
            base,
            ior: m.ior,
            transmission: m.transmission,
            thickness: m.thickness,
            ..Default::default()
        })
    } else {
        Material::Standard(base)
    })
}

/// The page's scene in the host scene: the meshes in merge order, the
/// environment, the gradient background and ACES Filmic tone mapping.
pub(crate) fn install(
    s: &mut Scene,
    p: &super::Prepared,
    environment: EnvironmentMap,
) -> Result<(Object3D, Arc<EnvironmentMap>)> {
    s.environment = Some(Arc::new(environment));
    s.tone_mapping = crate::scene::ToneMapping::Aces;
    s.background_alpha = 0.;
    // GradientEquirectTexture rows from v = 0 ( the bottom color ).
    let size = 512;
    let gradient = textures::gradient(
        size,
        Color::from_hex(0xeeeeee).0.to_array(),
        Color::from_hex(0xeaeaea).0.to_array(),
    );
    let mut rgba = Vec::with_capacity(gradient.len());
    for y in (0..size).rev() {
        rgba.extend(
            gradient[y * size * 4..(y + 1) * size * 4]
                .iter()
                .map(|&v| half::f16::from_f32(v)),
        );
    }
    let background = Arc::new(EnvironmentMap {
        width: size as u32,
        height: size as u32,
        rgba,
        gpu: None,
    });
    s.background_map = Some(background.clone());
    let mut last = None;
    for m in &p.meshes {
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(m.positions.clone(), 3, false)?),
        );
        g.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(m.normals.clone(), 3, false)?),
        );
        if let Some(uvs) = &m.uvs {
            g.set_attribute(
                "uv",
                Attribute::F32(BufferAttribute::new(uvs.clone(), 2, false)?),
            );
        }
        g.set_index(m.index.clone());
        let h = s.insert(NodeKind::Mesh(Mesh {
            geometry: Arc::new(g),
            materials: vec![Arc::new(material(&m.material, &p.floor_map)?)],
        }));
        let n = s.get_mut(h)?;
        n.matrix = Matrix4::from_cols_array(&m.matrix_world);
        n.matrix_auto_update = false;
        n.matrix_world_needs_update = true;
        last = Some(h);
    }
    // The floor is the last mesh.
    Ok((
        last.ok_or(crate::Error::Invalid("pathtracer scene"))?,
        background,
    ))
}
