//! Static glTF 2.0 import. Callers resolve buffer/image bytes; no network or GPU state is owned here.
use crate::{
    Error, Result, attribute::BufferAttribute, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;
pub struct ImportedGltf {
    pub bounds: Box3,
    pub triangles: usize,
    meshes: Vec<(String, Matrix4, Mesh)>,
}
impl ImportedGltf {
    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }
    pub fn instantiate(self, scene: &mut Scene) -> Result<Vec<Object3D>> {
        self.meshes
            .into_iter()
            .map(|(name, matrix, mesh)| {
                let handle = scene.insert(NodeKind::Mesh(mesh));
                let node = scene.get_mut(handle)?;
                node.name = name;
                node.matrix = matrix;
                node.matrix_auto_update = false;
                node.matrix_world_needs_update = true;
                Ok(handle)
            })
            .collect()
    }
}
pub fn import(asset: &gltf::Gltf, buffers: &[Vec<u8>], images: &[Vec<u8>]) -> Result<ImportedGltf> {
    let decoded_images = images
        .iter()
        .map(|bytes| Texture::from_image(bytes))
        .collect::<Result<Vec<_>>>()?;
    import_decoded(asset, buffers, &decoded_images)
}
/// Import caller-decoded images, allowing platform codecs without duplicating scene import.
pub fn import_decoded(
    asset: &gltf::Gltf,
    buffers: &[Vec<u8>],
    decoded_images: &[Texture],
) -> Result<ImportedGltf> {
    let extensions = asset.extensions_required().collect::<Vec<_>>();
    if !extensions.is_empty() {
        return Err(Error::Asset(format!(
            "Unsupported required glTF extensions: {}. Use uncompressed static metallic-roughness glTF.",
            extensions.join(", ")
        )));
    }
    if asset.skins().next().is_some() {
        return Err(Error::Invalid(
            "skinned glTF is outside the static importer",
        ));
    }
    for buffer in asset.buffers() {
        if buffers
            .get(buffer.index())
            .is_none_or(|b| b.len() < buffer.length())
        {
            return Err(Error::Invalid("short glTF buffer"));
        }
    }
    let mut textures = Vec::new();
    for texture in asset.textures() {
        let mut image = decoded_images
            .get(texture.source().index())
            .ok_or(Error::Invalid("image index"))?
            .clone();
        image.flip_y = false;
        image.srgb = true;
        let wrap = |w| match w {
            gltf::texture::WrappingMode::ClampToEdge => Wrapping::Clamp,
            gltf::texture::WrappingMode::MirroredRepeat => Wrapping::Mirror,
            _ => Wrapping::Repeat,
        };
        image.wrap_s = wrap(texture.sampler().wrap_s());
        image.wrap_t = wrap(texture.sampler().wrap_t());
        image.filter = if texture.sampler().mag_filter() == Some(gltf::texture::MagFilter::Nearest)
        {
            Filter::Nearest
        } else {
            Filter::Linear
        };
        use gltf::texture::MinFilter as M;
        let min = texture
            .sampler()
            .min_filter()
            .unwrap_or(M::LinearMipmapLinear);
        image.min_filter = Some(match min {
            M::Nearest | M::NearestMipmapNearest | M::NearestMipmapLinear => Filter::Nearest,
            _ => Filter::Linear,
        });
        image.mipmap_filter = match min {
            M::Nearest | M::Linear => None,
            M::NearestMipmapNearest | M::LinearMipmapNearest => Some(Filter::Nearest),
            _ => Some(Filter::Linear),
        };
        let mut linear = image.clone();
        linear.srgb = false;
        textures.push((Arc::new(image), Arc::new(linear)));
    }

    let selected = asset
        .default_scene()
        .or_else(|| asset.scenes().next())
        .ok_or(Error::Invalid("glTF scene"))?;
    let mut stack: Vec<_> = selected.nodes().map(|n| (n, Matrix4::IDENTITY)).collect();
    let mut bounds = Box3::default();
    let mut triangles = 0;
    let mut meshes = Vec::new();
    while let Some((node, parent)) = stack.pop() {
        let local = Matrix4::from_cols_array_2d(
            &node
                .transform()
                .matrix()
                .map(|column| column.map(f64::from)),
        );
        let world = parent * local;
        for child in node.children() {
            stack.push((child, world));
        }
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                if primitive.mode() != gltf::mesh::Mode::Triangles
                    || primitive.morph_targets().next().is_some()
                {
                    return Err(Error::Invalid("non-triangle or morph glTF primitive"));
                }
                let reader =
                    primitive.reader(|buffer| buffers.get(buffer.index()).map(Vec::as_slice));
                let positions: Vec<_> = reader
                    .read_positions()
                    .ok_or(Error::Invalid("glTF positions"))?
                    .collect();
                for p in &positions {
                    bounds.expand_by_point(
                        world.transform_point3(Vector3::from_array(p.map(f64::from))),
                    );
                }
                let mut geometry = BufferGeometry::default();
                geometry.set_attribute(
                    "position",
                    Attribute::F32(BufferAttribute::new(
                        positions.into_iter().flatten().collect(),
                        3,
                        false,
                    )?),
                );
                if let Some(indices) = reader.read_indices() {
                    geometry.set_index(Some(indices.into_u32().collect()));
                }
                if let Some(normals) = reader.read_normals() {
                    geometry.set_attribute(
                        "normal",
                        Attribute::F32(BufferAttribute::new(
                            normals.flatten().collect(),
                            3,
                            false,
                        )?),
                    );
                }
                if let Some(uv) = reader.read_tex_coords(0) {
                    geometry.set_attribute(
                        "uv",
                        Attribute::F32(BufferAttribute::new(
                            uv.into_f32().flatten().collect(),
                            2,
                            false,
                        )?),
                    );
                }
                let source = primitive.material();
                let pbr = source.pbr_metallic_roughness();
                let factor = pbr.base_color_factor();
                let get_texture = |index: usize, uv: u32, srgb: bool| -> Result<Arc<Texture>> {
                    if uv != 0 {
                        return Err(Error::Invalid(
                            "glTF texture coordinate set (only TEXCOORD_0 supported)",
                        ));
                    }
                    let pair = textures.get(index).ok_or(Error::Invalid("glTF texture"))?;
                    Ok(if srgb { pair.0.clone() } else { pair.1.clone() })
                };
                let mut standard = MeshStandardMaterial {
                    roughness: pbr.roughness_factor() as f64,
                    metalness: pbr.metallic_factor() as f64,
                    emissive: Color(Vector3::from_array(source.emissive_factor().map(f64::from))),
                    ..Default::default()
                };
                if let Some(info) = pbr.metallic_roughness_texture() {
                    standard.metallic_roughness_map = Some(get_texture(
                        info.texture().index(),
                        info.tex_coord(),
                        false,
                    )?);
                }
                if let Some(info) = source.normal_texture() {
                    standard.normal_map = Some(get_texture(
                        info.texture().index(),
                        info.tex_coord(),
                        false,
                    )?);
                    standard.normal_scale = Vector2::splat(info.scale() as f64);
                }
                if let Some(info) = source.occlusion_texture() {
                    standard.occlusion_map = Some(get_texture(
                        info.texture().index(),
                        info.tex_coord(),
                        false,
                    )?);
                    standard.occlusion_strength = info.strength() as f64;
                }
                if let Some(info) = source.emissive_texture() {
                    standard.emissive_map =
                        Some(get_texture(info.texture().index(), info.tex_coord(), true)?);
                }
                if let Some(tangents) = reader.read_tangents() {
                    geometry.set_attribute(
                        "tangent",
                        Attribute::F32(BufferAttribute::new(
                            tangents.flatten().collect(),
                            4,
                            false,
                        )?),
                    );
                }
                // Match GLTFLoader's derivative-tangent convention for glTF UVs.
                if !geometry.attributes.contains_key("tangent") {
                    standard.normal_scale.y *= -1.0;
                }
                let mut material = Material::Standard(standard);
                let p = material.properties_mut();
                p.color = Color(Vector3::new(
                    factor[0] as f64,
                    factor[1] as f64,
                    factor[2] as f64,
                ));
                p.opacity = factor[3] as f64;
                p.transparent = source.alpha_mode() == gltf::material::AlphaMode::Blend;
                p.depth_write = !p.transparent;
                p.alpha_test = if source.alpha_mode() == gltf::material::AlphaMode::Mask {
                    source.alpha_cutoff().unwrap_or(0.5) as f64
                } else {
                    0.0
                };
                p.side = if source.double_sided() {
                    Side::Double
                } else {
                    Side::Front
                };
                if let Some(info) = pbr.base_color_texture() {
                    if info.tex_coord() != 0 {
                        return Err(Error::Invalid("glTF UV set"));
                    }
                    p.map = Some(get_texture(info.texture().index(), info.tex_coord(), true)?);
                }
                if let Some(colors) = reader.read_colors(0) {
                    geometry.set_attribute(
                        "color",
                        Attribute::F32(BufferAttribute::new(
                            colors.into_rgba_f32().flatten().collect(),
                            4,
                            false,
                        )?),
                    );
                    p.vertex_colors = true;
                }
                if !geometry.attributes.contains_key("normal") {
                    geometry = geometry.to_non_indexed()?;
                    geometry.compute_vertex_normals()?;
                }
                triangles += geometry.draw_count() / 3;
                meshes.push((
                    node.name().unwrap_or("glTF mesh").to_owned(),
                    world,
                    Mesh::new(Arc::new(geometry), Arc::new(material)),
                ));
            }
        }
    }
    if meshes.is_empty() || bounds.is_empty() {
        return Err(Error::Invalid("empty glTF scene"));
    }

    Ok(ImportedGltf {
        bounds,
        triangles,
        meshes,
    })
}
