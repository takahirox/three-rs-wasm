//! Static glTF 2.0 import. Callers resolve buffer/image bytes; no network or GPU state is owned here.
use crate::{
    Error, Result, attribute::BufferAttribute, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;
mod animation;
pub use animation::{AnimatedGltf, GltfInstance, import_animated, import_animated_decoded};
/// KHR_materials_variants mappings of one mesh: ( variant indices, material ).
pub type VariantMaterials = Vec<(Vec<usize>, Arc<Material>)>;
#[derive(Clone)]
pub struct ImportedGltf {
    pub bounds: Box3,
    pub triangles: usize,
    meshes: Vec<(String, Matrix4, Mesh)>,
    /// Per mesh, in `instantiate` order: KHR_materials_variants mappings as
    /// ( variant indices, material ).
    pub variant_materials: Vec<VariantMaterials>,
    mesh_nodes: Vec<usize>,
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
    import_internal(asset, buffers, decoded_images, false)
}
fn import_internal(
    asset: &gltf::Gltf,
    buffers: &[Vec<u8>],
    decoded_images: &[Texture],
    animated: bool,
) -> Result<ImportedGltf> {
    let extensions = asset
        .extensions_required()
        .filter(|name| {
            !((animated && *name == "EXT_mesh_gpu_instancing")
                || matches!(
                    *name,
                    "KHR_texture_transform"
                        | "KHR_materials_unlit"
                        | "KHR_materials_emissive_strength"
                        | "KHR_materials_ior"
                        | "KHR_materials_specular"
                        | "KHR_materials_clearcoat"
                        | "KHR_materials_sheen"
                        | "KHR_materials_anisotropy"
                        | "KHR_materials_transmission"
                        | "KHR_materials_volume"
                        | "KHR_materials_dispersion"
                        | "KHR_materials_iridescence"
                ))
        })
        .collect::<Vec<_>>();
    if !extensions.is_empty() {
        return Err(Error::Asset(format!(
            "Unsupported required glTF extensions: {}. Decode compressed geometry with compression::prepare_gltf and use supported material extensions.",
            extensions.join(", ")
        )));
    }
    if !animated
        && asset
            .nodes()
            .any(|n| n.extension_value("EXT_mesh_gpu_instancing").is_some())
    {
        return Err(Error::Invalid(
            "GPU instances require animated glTF importer",
        ));
    }
    if !animated && asset.skins().next().is_some() {
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
    let mut variant_materials = Vec::new();
    let mut mesh_nodes = Vec::new();
    while let Some((node, parent)) = stack.pop() {
        let local = local_transform(node.transform());
        let world = parent * local;
        for child in node.children() {
            stack.push((child, world));
        }
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                if primitive.mode() != gltf::mesh::Mode::Triangles
                    || (!animated && primitive.morph_targets().next().is_some())
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
                if let Some(uv) = reader.read_tex_coords(1) {
                    geometry.set_attribute(
                        "uv1",
                        Attribute::F32(BufferAttribute::new(
                            uv.into_f32().flatten().collect(),
                            2,
                            false,
                        )?),
                    );
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
                if animated {
                    if let Some(joints) = reader.read_joints(0) {
                        geometry.set_attribute(
                            "skinIndex",
                            Attribute::U16(BufferAttribute::new(
                                joints.into_u16().flatten().collect(),
                                4,
                                false,
                            )?),
                        );
                    }
                    if let Some(weights) = reader.read_weights(0) {
                        let values = weights
                            .into_f32()
                            .flat_map(|mut v| {
                                let sum = v.iter().sum::<f32>();
                                if sum > 0.0 {
                                    for w in &mut v {
                                        *w /= sum;
                                    }
                                } else {
                                    v = [1.0, 0.0, 0.0, 0.0];
                                }
                                v
                            })
                            .collect();
                        geometry.set_attribute(
                            "skinWeight",
                            Attribute::F32(BufferAttribute::new(values, 4, false)?),
                        );
                    }
                    geometry.morph_targets_relative = true;
                    for (positions, normals, tangents) in reader.read_morph_targets() {
                        for (name, values) in [
                            (
                                "position",
                                positions.map(|v| v.flatten().collect::<Vec<_>>()),
                            ),
                            ("normal", normals.map(|v| v.flatten().collect::<Vec<_>>())),
                            ("tangent", tangents.map(|v| v.flatten().collect::<Vec<_>>())),
                        ] {
                            if name != "position" && !geometry.attributes.contains_key(name) {
                                continue;
                            }
                            let values =
                                values.unwrap_or_else(|| vec![0.0; geometry.vertex_count() * 3]);
                            geometry
                                .morph_attributes
                                .entry(name.into())
                                .or_default()
                                .push(Attribute::F32(BufferAttribute::new(values, 3, false)?));
                        }
                    }
                }
                // Builds a primitive's material: its own, or a KHR_materials_variants one.
                let build = |source: gltf::Material<'_>,
                             geometry: &mut BufferGeometry|
                 -> Result<Material> {
                    let pbr = source.pbr_metallic_roughness();
                    let factor = pbr.base_color_factor();
                    let get_texture = |index: usize, uv: u32, srgb: bool| -> Result<Arc<Texture>> {
                        if uv > 1 {
                            return Err(Error::Invalid(
                                "glTF texture coordinate set (only TEXCOORD_0/1 supported)",
                            ));
                        }
                        let pair = textures.get(index).ok_or(Error::Invalid("glTF texture"))?;
                        let mut texture = if srgb { pair.0.clone() } else { pair.1.clone() };
                        if uv != 0 {
                            Arc::make_mut(&mut texture).tex_coord = uv;
                        }
                        Ok(texture)
                    };
                    let get_info =
                        |info: &gltf::texture::Info<'_>, srgb: bool| -> Result<Arc<Texture>> {
                            let transform = info.texture_transform();
                            let mut texture = get_texture(
                                info.texture().index(),
                                transform
                                    .as_ref()
                                    .and_then(|t| t.tex_coord())
                                    .unwrap_or(info.tex_coord()),
                                srgb,
                            )?;
                            if let Some(t) = transform {
                                let texture = Arc::make_mut(&mut texture);
                                texture.offset = Vector2::from_array(t.offset().map(f64::from));
                                texture.repeat = Vector2::from_array(t.scale().map(f64::from));
                                texture.rotation = t.rotation() as f64;
                                let (s, c) = texture.rotation.sin_cos();
                                let scale = texture.repeat;
                                let offset = texture.offset;
                                texture.matrix = Some(Matrix3::from_cols_array(&[
                                    scale.x * c,
                                    -scale.x * s,
                                    0.0,
                                    scale.y * s,
                                    scale.y * c,
                                    0.0,
                                    offset.x,
                                    offset.y,
                                    1.0,
                                ]));
                            }
                            Ok(texture)
                        };
                    let mut standard = MeshStandardMaterial {
                        roughness: pbr.roughness_factor() as f64,
                        metalness: pbr.metallic_factor() as f64,
                        emissive: Color(
                            Vector3::from_array(source.emissive_factor().map(f64::from))
                                * source.emissive_strength().unwrap_or(1.0) as f64,
                        ),
                        ..Default::default()
                    };
                    if let Some(info) = pbr.metallic_roughness_texture() {
                        standard.metallic_roughness_map = Some(get_info(&info, false)?);
                    }
                    if let Some(info) = source.normal_texture() {
                        standard.normal_map = Some(transform_texture(
                            get_texture(info.texture().index(), info.tex_coord(), false)?,
                            info.extension_value("KHR_texture_transform"),
                        )?);
                        standard.normal_scale = Vector2::splat(info.scale() as f64);
                    }
                    if let Some(info) = source.occlusion_texture() {
                        standard.occlusion_map = Some(transform_texture(
                            get_texture(info.texture().index(), info.tex_coord(), false)?,
                            info.extension_value("KHR_texture_transform"),
                        )?);
                        standard.occlusion_strength = info.strength() as f64;
                    }
                    if let Some(info) = source.emissive_texture() {
                        standard.emissive_map = Some(get_info(&info, true)?);
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
                        p.map = Some(get_info(&info, true)?);
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
                    if source.unlit() {
                        material = Material::Basic(MeshBasicMaterial {
                            properties: material.properties().clone(),
                        });
                    } else if source.ior().is_some()
                        || source.specular().is_some()
                        || [
                            "KHR_materials_clearcoat",
                            "KHR_materials_sheen",
                            "KHR_materials_anisotropy",
                            "KHR_materials_transmission",
                            "KHR_materials_volume",
                            "KHR_materials_dispersion",
                            "KHR_materials_iridescence",
                        ]
                        .iter()
                        .any(|name| source.extension_value(name).is_some())
                    {
                        let Material::Standard(base) = material else {
                            unreachable!()
                        };
                        let mut physical = MeshPhysicalMaterial {
                            base,
                            ior: source.ior().unwrap_or(1.5) as f64,
                            ..Default::default()
                        };
                        if let Some(specular) = source.specular() {
                            physical.specular_intensity_map = specular
                                .specular_texture()
                                .map(|info| get_info(&info, false))
                                .transpose()?;
                            physical.specular_color_map = specular
                                .specular_color_texture()
                                .map(|info| get_info(&info, true))
                                .transpose()?;
                            physical.specular_color = Color(Vector3::from_array(
                                specular.specular_color_factor().map(f64::from),
                            ));
                            physical.specular_intensity = specular.specular_factor() as f64;
                        }
                        for name in [
                            "KHR_materials_clearcoat",
                            "KHR_materials_sheen",
                            "KHR_materials_anisotropy",
                            "KHR_materials_transmission",
                            "KHR_materials_volume",
                            "KHR_materials_dispersion",
                            "KHR_materials_iridescence",
                        ] {
                            if let Some(extension) = source.extension_value(name) {
                                let map = |key: &str, srgb: bool| -> Result<Option<Arc<Texture>>> {
                                    let Some(info) = extension.get(key) else {
                                        return Ok(None);
                                    };
                                    let index = info["index"]
                                        .as_u64()
                                        .ok_or(Error::Invalid("physical texture index"))?
                                        as usize;
                                    let uv = info
                                        .get("texCoord")
                                        .map(|v| {
                                            v.as_u64().ok_or(Error::Invalid("physical texture UV"))
                                        })
                                        .transpose()?
                                        .unwrap_or(0);
                                    if uv > 1 {
                                        return Err(Error::Invalid("physical texture UV"));
                                    }
                                    Ok(Some(transform_texture(
                                        get_texture(index, uv as u32, srgb)?,
                                        info.get("extensions")
                                            .and_then(|e| e.get("KHR_texture_transform")),
                                    )?))
                                };
                                let value = |key: &str, default: f64| {
                                    extension[key].as_f64().unwrap_or(default)
                                };
                                match name {
                                    "KHR_materials_clearcoat" => {
                                        physical.clearcoat_map = map("clearcoatTexture", false)?;
                                        physical.clearcoat_roughness_map =
                                            map("clearcoatRoughnessTexture", false)?;
                                        physical.clearcoat_normal_map =
                                            map("clearcoatNormalTexture", false)?;
                                        physical.clearcoat_normal_scale = Vector2::splat(
                                            extension["clearcoatNormalTexture"]["scale"]
                                                .as_f64()
                                                .unwrap_or(1.0),
                                        );
                                        if !geometry.attributes.contains_key("tangent") {
                                            physical.clearcoat_normal_scale.y *= -1.0;
                                        }

                                        physical.clearcoat = value("clearcoatFactor", 0.0);
                                        physical.clearcoat_roughness =
                                            value("clearcoatRoughnessFactor", 0.0);
                                    }
                                    "KHR_materials_sheen" => {
                                        physical.sheen_color_map = map("sheenColorTexture", true)?;
                                        physical.sheen_roughness_map =
                                            map("sheenRoughnessTexture", false)?;

                                        physical.sheen = 1.0;
                                        physical.sheen_roughness =
                                            value("sheenRoughnessFactor", 0.0);
                                        if let Some(color) =
                                            extension["sheenColorFactor"].as_array()
                                        {
                                            if color.len() != 3 {
                                                return Err(Error::Invalid("sheen color"));
                                            }
                                            physical.sheen_color = Color::linear(
                                                color[0].as_f64().unwrap_or(0.0),
                                                color[1].as_f64().unwrap_or(0.0),
                                                color[2].as_f64().unwrap_or(0.0),
                                            );
                                        }
                                    }
                                    "KHR_materials_iridescence" => {
                                        physical.iridescence = value("iridescenceFactor", 0.0);
                                        physical.iridescence_ior = value("iridescenceIor", 1.3);
                                        physical.iridescence_thickness_range = [
                                            value("iridescenceThicknessMinimum", 100.0),
                                            value("iridescenceThicknessMaximum", 400.0),
                                        ];
                                        physical.iridescence_map =
                                            map("iridescenceTexture", false)?;
                                        physical.iridescence_thickness_map =
                                            map("iridescenceThicknessTexture", false)?;
                                    }
                                    "KHR_materials_transmission" => {
                                        physical.transmission = value("transmissionFactor", 0.0);
                                        physical.transmission_map =
                                            map("transmissionTexture", false)?;
                                    }
                                    "KHR_materials_volume" => {
                                        physical.thickness = value("thicknessFactor", 0.0);
                                        physical.thickness_map = map("thicknessTexture", false)?;
                                        physical.attenuation_distance =
                                            value("attenuationDistance", f64::INFINITY);
                                        if let Some(color) = extension
                                            .get("attenuationColor")
                                            .and_then(|v| v.as_array())
                                        {
                                            if color.len() != 3 {
                                                return Err(Error::Invalid("attenuation color"));
                                            }
                                            physical.attenuation_color = Color::linear(
                                                color[0].as_f64().unwrap_or(1.0),
                                                color[1].as_f64().unwrap_or(1.0),
                                                color[2].as_f64().unwrap_or(1.0),
                                            );
                                        }
                                    }
                                    "KHR_materials_dispersion" => {
                                        physical.dispersion = value("dispersion", 0.0);
                                    }
                                    _ => {
                                        physical.anisotropy_map = map("anisotropyTexture", false)?;
                                        physical.anisotropy = value("anisotropyStrength", 0.0);
                                        physical.anisotropy_rotation =
                                            value("anisotropyRotation", 0.0);
                                    }
                                }
                            }
                        }
                        material = Material::Physical(physical);
                    }
                    Ok(material)
                };
                let mut material = build(primitive.material(), &mut geometry)?;
                let mut variants = Vec::new();
                if let Some(mappings) = primitive
                    .extension_value("KHR_materials_variants")
                    .and_then(|v| v.get("mappings"))
                    .and_then(|v| v.as_array())
                {
                    for mapping in mappings {
                        let index = mapping["material"]
                            .as_u64()
                            .ok_or(Error::Invalid("variant material"))?
                            as usize;
                        let source = asset
                            .materials()
                            .nth(index)
                            .ok_or(Error::Invalid("variant material"))?;
                        let mut variant = build(source, &mut geometry)?;
                        if !geometry.attributes.contains_key("normal") {
                            variant.properties_mut().flat_shading = true;
                        }
                        let indices = mapping["variants"]
                            .as_array()
                            .ok_or(Error::Invalid("variant indices"))?
                            .iter()
                            .filter_map(|v| v.as_u64().map(|v| v as usize))
                            .collect();
                        variants.push((indices, Arc::new(variant)));
                    }
                }
                if !geometry.attributes.contains_key("normal") {
                    // GLTFLoader derives flat normals from the deformed surface
                    // in the fragment shader. Baking rest-pose normals here
                    // produces incorrect lighting when position-only morphs move.
                    material.properties_mut().flat_shading = true;
                }
                triangles += geometry.draw_count() / 3;
                mesh_nodes.push(node.index());
                meshes.push((
                    node.name().unwrap_or("glTF mesh").to_owned(),
                    world,
                    Mesh::new(Arc::new(geometry), Arc::new(material)),
                ));
                variant_materials.push(variants);
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
        variant_materials,
        mesh_nodes,
    })
}

// Normal/occlusion texture infos expose extensions through the generic JSON API.
fn transform_texture(
    mut texture: Arc<Texture>,
    extension: Option<&serde_json::Value>,
) -> Result<Arc<Texture>> {
    let Some(t) = extension else {
        return Ok(texture);
    };
    let texture_mut = Arc::make_mut(&mut texture);
    let pair = |key: &str, default: [f64; 2]| -> Result<[f64; 2]> {
        let Some(value) = t.get(key) else {
            return Ok(default);
        };
        let a = value
            .as_array()
            .ok_or(Error::Invalid("glTF texture transform"))?;
        if a.len() != 2 {
            return Err(Error::Invalid("glTF texture transform"));
        }
        Ok([
            a[0].as_f64()
                .ok_or(Error::Invalid("glTF texture transform"))?,
            a[1].as_f64()
                .ok_or(Error::Invalid("glTF texture transform"))?,
        ])
    };
    texture_mut.offset = Vector2::from_array(pair("offset", [0.0, 0.0])?);
    texture_mut.repeat = Vector2::from_array(pair("scale", [1.0, 1.0])?);
    texture_mut.rotation = match t.get("rotation") {
        Some(v) => v.as_f64().ok_or(Error::Invalid("glTF texture rotation"))?,
        None => 0.0,
    };
    if let Some(uv) = t.get("texCoord") {
        let uv = uv
            .as_u64()
            .filter(|v| *v <= 1)
            .ok_or(Error::Invalid("glTF texture coordinates"))?;
        texture_mut.tex_coord = uv as u32;
    }
    let (s, c) = texture_mut.rotation.sin_cos();
    let scale = texture_mut.repeat;
    let offset = texture_mut.offset;
    texture_mut.matrix = Some(Matrix3::from_cols_array(&[
        scale.x * c,
        -scale.x * s,
        0.0,
        scale.y * s,
        scale.y * c,
        0.0,
        offset.x,
        offset.y,
        1.0,
    ]));
    Ok(texture)
}

// Three composes glTF TRS in double precision and converts to f32 only on GPU upload.
fn local_transform(transform: gltf::scene::Transform) -> Matrix4 {
    match transform {
        gltf::scene::Transform::Matrix { matrix } => {
            Matrix4::from_cols_array_2d(&matrix.map(|v| v.map(f64::from)))
        }
        gltf::scene::Transform::Decomposed {
            translation,
            rotation,
            scale,
        } => Matrix4::from_scale_rotation_translation(
            Vector3::from_array(scale.map(f64::from)),
            Quaternion::from_array(rotation.map(f64::from)),
            Vector3::from_array(translation.map(f64::from)),
        ),
    }
}
