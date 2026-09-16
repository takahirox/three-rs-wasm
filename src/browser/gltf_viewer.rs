//! Static glTF/GLB base-color viewer. gltf-rs parses accessors; rendering stays in Rust.
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    scene::*,
};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

async fn fetch(url: &str) -> Result<Vec<u8>> {
    let response = JsFuture::from(
        web_sys::window()
            .ok_or(Error::Invalid("window"))?
            .fetch_with_str(url),
    )
    .await
    .map_err(|e| Error::Asset(format!("{e:?}")))?
    .dyn_into::<web_sys::Response>()
    .map_err(|_| Error::Invalid("response"))?;
    if !response.ok() {
        return Err(Error::Asset(format!("HTTP {}: {url}", response.status())));
    }
    let data = JsFuture::from(
        response
            .array_buffer()
            .map_err(|e| Error::Asset(format!("{e:?}")))?,
    )
    .await
    .map_err(|e| Error::Asset(format!("{e:?}")))?;
    Ok(js_sys::Uint8Array::new(&data).to_vec())
}
fn external(base: &str, uri: &str) -> Result<String> {
    if uri.starts_with("data:") {
        return Err(Error::Invalid("data URI (use external files or GLB)"));
    }
    Ok(format!("{base}/{uri}"))
}
pub(super) struct GltfViewer {
    center: Vector3,
    radius: f64,
    initial_radius: f64,
    yaw: f64,
    pitch: f64,
    pub triangles: usize,
    pub meshes: usize,
}
impl GltfViewer {
    pub async fn create(
        scene: &mut Scene,
        camera: Object3D,
        placeholder: Object3D,
        example: u32,
    ) -> Result<Self> {
        let url = if example == 4 {
            "/web/models/DamagedHelmet/glTF/DamagedHelmet.gltf"
        } else {
            "/web/models/BoomBox.glb"
        };
        let base = url.rsplit_once('/').ok_or(Error::Invalid("asset URL"))?.0;
        let bytes = fetch(url).await?;
        let asset = gltf::Gltf::from_slice(&bytes).map_err(|e| Error::Asset(e.to_string()))?;
        if asset.extensions_required().next().is_some() || asset.skins().next().is_some() {
            return Err(Error::Invalid("required glTF extension or skin"));
        }
        let mut buffers = Vec::new();
        for buffer in asset.buffers() {
            let data = match buffer.source() {
                gltf::buffer::Source::Bin => {
                    asset.blob.clone().ok_or(Error::Invalid("GLB buffer"))?
                }
                gltf::buffer::Source::Uri(uri) => fetch(&external(base, uri)?).await?,
            };
            if data.len() < buffer.length() {
                return Err(Error::Invalid("short glTF buffer"));
            }
            buffers.push(data);
        }
        let mut images = Vec::new();
        for image in asset.images() {
            let data = match image.source() {
                gltf::image::Source::Uri { uri, .. } => fetch(&external(base, uri)?).await?,
                gltf::image::Source::View { view, .. } => {
                    let buffer = buffers
                        .get(view.buffer().index())
                        .ok_or(Error::Invalid("image buffer"))?;
                    let end = view
                        .offset()
                        .checked_add(view.length())
                        .ok_or(Error::Invalid("image range"))?;
                    buffer
                        .get(view.offset()..end)
                        .ok_or(Error::Invalid("image range"))?
                        .to_vec()
                }
            };
            images.push(Texture::from_image(&data)?);
        }
        let mut textures = Vec::new();
        for texture in asset.textures() {
            let mut image = images
                .get(texture.source().index())
                .ok_or(Error::Invalid("image index"))?
                .clone();
            image.flip_y = false;
            let wrap = |w| match w {
                gltf::texture::WrappingMode::ClampToEdge => Wrapping::Clamp,
                gltf::texture::WrappingMode::MirroredRepeat => Wrapping::Mirror,
                _ => Wrapping::Repeat,
            };
            image.wrap_s = wrap(texture.sampler().wrap_s());
            image.wrap_t = wrap(texture.sampler().wrap_t());
            image.filter =
                if texture.sampler().mag_filter() == Some(gltf::texture::MagFilter::Nearest) {
                    Filter::Nearest
                } else {
                    Filter::Linear
                };
            textures.push(Arc::new(image));
        }
        scene.dispose(placeholder)?;
        let selected = asset
            .default_scene()
            .or_else(|| asset.scenes().next())
            .ok_or(Error::Invalid("glTF scene"))?;
        let mut stack: Vec<_> = selected.nodes().map(|n| (n, Matrix4::IDENTITY)).collect();
        let mut bounds = Box3::default();
        let mut triangles = 0;
        let mut meshes = 0;
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
                    if source.alpha_mode() == gltf::material::AlphaMode::Mask {
                        return Err(Error::Invalid("glTF alpha mask"));
                    }
                    let pbr = source.pbr_metallic_roughness();
                    let factor = pbr.base_color_factor();
                    let mut material = Material::default();
                    let p = material.properties_mut();
                    p.color = Color(Vector3::new(
                        factor[0] as f64,
                        factor[1] as f64,
                        factor[2] as f64,
                    ));
                    p.opacity = factor[3] as f64;
                    p.transparent = source.alpha_mode() == gltf::material::AlphaMode::Blend;
                    p.side = if source.double_sided() {
                        Side::Double
                    } else {
                        Side::Front
                    };
                    if let Some(info) = pbr.base_color_texture() {
                        if info.tex_coord() != 0 {
                            return Err(Error::Invalid("glTF UV set"));
                        }
                        p.map = Some(
                            textures
                                .get(info.texture().index())
                                .ok_or(Error::Invalid("glTF texture"))?
                                .clone(),
                        );
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
                    triangles += geometry.draw_count() / 3;
                    meshes += 1;
                    let handle = scene.insert(NodeKind::Mesh(Mesh::new(
                        Arc::new(geometry),
                        Arc::new(material),
                    )));
                    let target = scene.get_mut(handle)?;
                    target.name = node.name().unwrap_or("glTF mesh").into();
                    target.matrix = world;
                    target.matrix_auto_update = false;
                    target.matrix_world_needs_update = true;
                }
            }
        }
        if meshes == 0 || bounds.is_empty() {
            return Err(Error::Invalid("empty glTF scene"));
        }
        let radius = bounds.size().max_element() * 1.8;
        scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.0,
            aspect: 1.0,
            near: radius / 100.0,
            far: radius * 100.0,
            ..Default::default()
        }));
        scene.background = Color::from_hex(0x101820);
        let viewer = Self {
            center: bounds.center(),
            radius,
            initial_radius: radius,
            yaw: -0.55,
            pitch: 0.18,
            triangles,
            meshes,
        };
        viewer.update(scene, camera)?;
        Ok(viewer)
    }
    pub fn orbit(&mut self, dx: f64, dy: f64, zoom: f64) {
        self.yaw -= dx * 0.008;
        self.pitch = (self.pitch + dy * 0.008).clamp(-1.4, 1.4);
        self.radius = (self.radius * (zoom * 0.001).exp())
            .clamp(self.initial_radius * 0.3, self.initial_radius * 3.0);
    }
    pub fn update(&self, scene: &mut Scene, camera: Object3D) -> Result<()> {
        scene.get_mut(camera)?.position = self.center
            + Vector3::new(
                self.yaw.sin() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.cos() * self.pitch.cos(),
            ) * self.radius;
        scene.look_at(camera, self.center)
    }
}
