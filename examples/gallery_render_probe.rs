//! Offline diagnostic: render captured upstream scene data with this Rust renderer.
//! This imports one observed frame, not the example's behavior or JavaScript code.
use serde::Deserialize;
use serde_json::{Value, json};
use std::{fs, path::Path, sync::Arc};
use three_rs_wasm::{
    Error, Result, attribute::BufferAttribute, camera::*, environment::EnvironmentMap, geometry::*,
    material::*, math::*, renderer::*, scene::*,
};

#[derive(Deserialize)]
struct ImageSpec {
    width: u32,
    height: u32,
    rgba: Vec<u8>,
    srgb: bool,
    flip_y: bool,
    wrap_s: u32,
    wrap_t: u32,
    mag: u32,
    min: u32,
    anisotropy: u16,
    offset: [f64; 2],
    repeat: [f64; 2],
    center: [f64; 2],
    rotation: f64,
}
#[derive(Deserialize)]
struct EnvironmentSpec {
    width: u32,
    height: u32,
    rgba: Vec<f32>,
}
#[derive(Deserialize)]
struct MaterialSpec {
    kind: String,
    color: [f64; 3],
    opacity: f64,
    transparent: bool,
    side: u32,
    alpha_test: f64,
    depth_test: bool,
    depth_write: bool,
    vertex_colors: bool,
    roughness: f64,
    metalness: f64,
    emissive: [f64; 3],
    normal_scale: [f64; 2],
    ao_intensity: f64,
    map: Option<usize>,
    mr_map: Option<usize>,
    normal_map: Option<usize>,
    emissive_map: Option<usize>,
    ao_map: Option<usize>,
    size: f64,
    size_attenuation: bool,
}
#[derive(Deserialize)]
struct GroupSpec {
    start: usize,
    count: usize,
    #[serde(rename = "materialIndex")]
    material_index: usize,
}
#[derive(Deserialize)]
struct ObjectSpec {
    kind: String,
    matrix: [f64; 16],
    position: Vec<f32>,
    normal: Option<Vec<f32>>,
    uv: Option<Vec<f32>>,
    color: Option<Vec<f32>>,
    color_size: usize,
    tangent: Option<Vec<f32>>,
    index: Option<Vec<u32>>,
    groups: Vec<GroupSpec>,
    draw_start: usize,
    draw_count: Option<usize>,
    materials: Vec<MaterialSpec>,
}
#[derive(Deserialize)]
struct CameraSpec {
    kind: String,
    matrix: [f64; 16],
    fov: f64,
    aspect: f64,
    near: f64,
    far: f64,
    zoom: f64,
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}
#[derive(Deserialize)]
struct LightSpec {
    kind: String,
    color: [f64; 3],
    intensity: f64,
    position: [f64; 3],
    target: [f64; 3],
    distance: f64,
    decay: f64,
}
#[derive(Deserialize)]
struct Snapshot {
    objects: Vec<ObjectSpec>,
    textures: Vec<ImageSpec>,
    environment: Option<EnvironmentSpec>,
    background: [f64; 3],
    background_environment: bool,
    background_blur: f64,
    background_intensity: f64,
    environment_intensity: f64,
    exposure: f64,
    aces: bool,
    camera: CameraSpec,
    lights: Vec<LightSpec>,
}
fn color(v: [f64; 3]) -> Color {
    Color(Vector3::from_array(v))
}
fn map(textures: &[Arc<Texture>], index: Option<usize>) -> Result<Option<Arc<Texture>>> {
    index
        .map(|i| {
            textures
                .get(i)
                .cloned()
                .ok_or(Error::Invalid("captured texture index"))
        })
        .transpose()
}
fn material(m: MaterialSpec, textures: &[Arc<Texture>]) -> Result<Arc<Material>> {
    let properties = MaterialProperties {
        color: color(m.color),
        opacity: m.opacity,
        alpha_test: m.alpha_test,
        transparent: m.transparent,
        side: match m.side {
            0 => Side::Front,
            1 => Side::Back,
            2 => Side::Double,
            _ => return Err(Error::Invalid("captured side")),
        },
        depth_test: m.depth_test,
        depth_write: m.depth_write,
        vertex_colors: m.vertex_colors,
        map: map(textures, m.map)?,
        ..Default::default()
    };
    Ok(Arc::new(match m.kind.as_str() {
        "basic" => Material::Basic(MeshBasicMaterial { properties }),
        "line" => Material::Line(LineBasicMaterial {
            properties,
            ..Default::default()
        }),
        "points" => Material::Points(PointsMaterial {
            properties,
            size: m.size,
            size_attenuation: m.size_attenuation,
        }),
        "standard" => Material::Standard(MeshStandardMaterial {
            properties,
            energy_conservation: false,
            roughness: m.roughness,
            metalness: m.metalness,
            emissive: color(m.emissive),
            normal_scale: Vector2::from_array(m.normal_scale),
            occlusion_strength: m.ao_intensity,
            metallic_roughness_map: map(textures, m.mr_map)?,
            normal_map: map(textures, m.normal_map)?,
            emissive_map: map(textures, m.emissive_map)?,
            occlusion_map: map(textures, m.ao_map)?,
        }),
        _ => return Err(Error::Invalid("captured material")),
    }))
}
fn import(snapshot: Snapshot) -> Result<(Scene, Object3D)> {
    let mut textures = Vec::new();
    for image in snapshot.textures {
        let mut texture = Texture::from_rgba(image.width, image.height, image.rgba, image.srgb)?;
        texture.flip_y = image.flip_y;
        texture.anisotropy = image.anisotropy;
        let wrap = |v| match v {
            1000 => Wrapping::Repeat,
            1002 => Wrapping::Mirror,
            _ => Wrapping::Clamp,
        };
        texture.wrap_s = wrap(image.wrap_s);
        texture.wrap_t = wrap(image.wrap_t);
        texture.filter = if image.mag == 1003 {
            Filter::Nearest
        } else {
            Filter::Linear
        };
        texture.min_filter = Some(if [1003, 1004, 1005].contains(&image.min) {
            Filter::Nearest
        } else {
            Filter::Linear
        });
        texture.mipmap_filter = match image.min {
            1004 | 1007 => Some(Filter::Nearest),
            1005 | 1008 => Some(Filter::Linear),
            _ => None,
        };
        texture.offset = Vector2::from_array(image.offset);
        texture.repeat = Vector2::from_array(image.repeat);
        texture.center = Vector2::from_array(image.center);
        texture.rotation = image.rotation;
        textures.push(Arc::new(texture));
    }
    let mut scene = Scene::new();
    scene.background = color(snapshot.background);
    scene.background_environment = snapshot.background_environment;
    scene.background_blur = snapshot.background_blur;
    scene.background_intensity = snapshot.background_intensity;
    scene.environment_intensity = snapshot.environment_intensity;
    scene.exposure = snapshot.exposure;
    scene.aces_tone_mapping = snapshot.aces;
    if let Some(env) = snapshot.environment {
        if env.width < 64
            || env.width > 8192
            || env.height == 0
            || env.height > 8192
            || env.rgba.len() != env.width as usize * env.height as usize * 4
            || env.rgba.iter().any(|v| !v.is_finite())
        {
            return Err(Error::Invalid("captured environment"));
        }
        scene.environment = Some(Arc::new(EnvironmentMap {
            gpu: None,
            width: env.width,
            height: env.height,
            rgba: env.rgba.into_iter().map(half::f16::from_f32).collect(),
        }));
    }
    let c = snapshot.camera;
    let camera = scene.insert(NodeKind::Camera(if c.kind == "perspective" {
        Camera::Perspective(PerspectiveCamera {
            fov: c.fov,
            aspect: c.aspect,
            near: c.near,
            far: c.far,
            zoom: c.zoom,
            ..Default::default()
        })
    } else {
        Camera::Orthographic(OrthographicCamera {
            left: c.left,
            right: c.right,
            top: c.top,
            bottom: c.bottom,
            near: c.near,
            far: c.far,
            zoom: c.zoom,
            ..Default::default()
        })
    }));
    scene.get_mut(camera)?.matrix = Matrix4::from_cols_array(&c.matrix);
    scene.get_mut(camera)?.matrix_auto_update = false;
    scene.get_mut(camera)?.matrix_world_needs_update = true;
    for l in snapshot.lights {
        let light = scene.insert(NodeKind::Light(match l.kind.as_str() {
            "ambient" => Light::Ambient {
                color: color(l.color),
                intensity: l.intensity,
            },
            "directional" => Light::Directional {
                color: color(l.color),
                intensity: l.intensity,
                target: Vector3::from_array(l.target),
            },
            "point" => Light::Point {
                color: color(l.color),
                intensity: l.intensity,
                distance: l.distance,
                decay: l.decay,
            },
            _ => return Err(Error::Invalid("captured light")),
        }));
        scene.get_mut(light)?.position = Vector3::from_array(l.position);
    }
    for object in snapshot.objects {
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(object.position, 3, false)?),
        );
        for (name, values, size) in [
            ("normal", object.normal, 3),
            ("uv", object.uv, 2),
            ("color", object.color, object.color_size),
            ("tangent", object.tangent, 4),
        ] {
            if let Some(values) = values {
                geometry.set_attribute(
                    name,
                    Attribute::F32(BufferAttribute::new(values, size, false)?),
                );
            }
        }
        geometry.set_index(object.index);
        geometry.set_draw_range(object.draw_start, object.draw_count);
        for group in object.groups {
            geometry.add_group(group.start, group.count, group.material_index);
        }
        if object.kind == "loop" && geometry.vertex_count() > 0 {
            let mut indices = geometry
                .get_index()
                .map(<[u32]>::to_vec)
                .unwrap_or_else(|| (0..geometry.vertex_count() as u32).collect());
            if let Some(&first) = indices.first() {
                indices.push(first);
            }
            geometry.set_index(Some(indices));
        }
        let materials = object
            .materials
            .into_iter()
            .map(|m| material(m, &textures))
            .collect::<Result<Vec<_>>>()?;
        let first = materials
            .first()
            .cloned()
            .ok_or(Error::Invalid("captured materials"))?;
        let kind = match object.kind.as_str() {
            "mesh" => NodeKind::Mesh(Mesh {
                geometry: Arc::new(geometry),
                materials,
            }),
            "line" | "segments" | "loop" => NodeKind::Line(Line {
                geometry: Arc::new(geometry),
                material: first,
                segments: object.kind == "segments",
            }),
            "points" => NodeKind::Points(Points {
                geometry: Arc::new(geometry),
                material: first,
            }),
            _ => return Err(Error::Invalid("captured object")),
        };
        let handle = scene.insert(kind);
        scene.get_mut(handle)?.matrix = Matrix4::from_cols_array(&object.matrix);
        scene.get_mut(handle)?.matrix_auto_update = false;
        scene.get_mut(handle)?.matrix_world_needs_update = true;
    }
    Ok((scene, camera))
}
fn render(
    renderer: &Renderer,
    snapshot: Snapshot,
    path: &Path,
) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let objects = snapshot.objects.len();
    let (mut scene, camera) = import(snapshot)?;
    let target = RenderTarget::with_options(
        &renderer.device,
        256,
        256,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba16Float,
            samples: 4,
            ..Default::default()
        },
    )?;
    renderer.render(&mut scene, camera, &target)?;
    let output = RenderTarget::new(&renderer.device, 256, 256)?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
        ..Default::default()
    });
    renderer.blit_tone_mapped(
        &target,
        &view,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        scene.exposure,
        scene.aces_tone_mapping,
    );
    let pixels = renderer.read_rgba(&output)?;
    let colored = pixels
        .chunks_exact(4)
        .filter(|p| p[0] != p[1] || p[1] != p[2])
        .count();
    image::save_buffer(path, &pixels, 256, 256, image::ColorType::Rgba8)?;
    let distinct = pixels
        .chunks_exact(4)
        .map(|p| [p[0], p[1], p[2]])
        .collect::<std::collections::HashSet<_>>()
        .len();
    Ok(
        json!({"stage":if distinct<2 {"rust-empty-frame"} else if objects==0 {"rust-background-frame-rendered"} else {"rust-static-frame-rendered"},"distinct_colors":distinct,"objects":objects,"colored_pixels":colored,"fidelity":"Not compared with upstream pixels; this is one captured frame, not a behavioral port."}),
    )
}
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::args()
        .nth(1)
        .ok_or("snapshot directory required")?;
    let mut paths = fs::read_dir(&directory)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    paths.sort();
    let mut renderer = None;
    for path in paths {
        let Some(name) = path
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|s| s.strip_suffix(".scene.json"))
        else {
            continue;
        };
        let result_path = Path::new(&directory).join(format!("{name}.rust.json"));
        #[derive(Deserialize)]
        struct Header {
            blockers: Vec<String>,
        }
        let result = (|| -> std::result::Result<Value, Box<dyn std::error::Error>> {
            let header: Header = serde_json::from_reader(std::io::BufReader::new(fs::File::open(&path)?))?;
            if !header.blockers.is_empty() {
                return Ok(json!({"stage":"rust-port-prerequisite","blockers":header.blockers}));
            }
            if renderer.is_none() {
                renderer = Some(pollster::block_on(Renderer::new())?);
            }
            render(renderer.as_ref().unwrap(),serde_json::from_reader(std::io::BufReader::new(fs::File::open(&path)?))?,&Path::new(&directory).join(format!("{name}.png")))
        })().unwrap_or_else(|e|json!({"stage":"rust-render-error","error":e.to_string().chars().take(500).collect::<String>()}));
        fs::write(result_path, serde_json::to_string_pretty(&result)? + "\n")?;
        println!("{name}: {}", result["stage"]);
        if let Some(renderer) = &renderer {
            renderer.collect_resources();
        }
    }
    Ok(())
}
