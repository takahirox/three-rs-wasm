//! webgl_loader_vrml: the sixteen VRML samples ( house by default ) under an
//! ambient and a directional light, with damped OrbitControls that reset
//! when an asset loads. The scenes are VRMLLoader's parse of each file, baked
//! by tools/tsl/prepare-vrml.mjs as the loader leaves them: Phong and basic
//! meshes ( the Background node's sky and ground spheres drawn first ), line
//! segments and points, with their world matrices, geometries, materials and
//! textures ( map.gif and the PixelTexture data ). All sixteen are uploaded
//! at load, so switching assets creates nothing.
//!
//! Each frame culls and orders the renderables as WebGLRenderer does
//! ( opaque by group order, render order, material id, clip-space z of the
//! bounding sphere center and id; transparent back to front, double-sided
//! ones as a back then a front pass ). The shaders are WGSL ports of the GLSL
//! WebGLRenderer builds for MeshPhongMaterial, MeshBasicMaterial,
//! LineBasicMaterial and PointsMaterial ( points as squares of the attenuated
//! gl_PointSize ); the canvas is sRGB-encoded in the shader and blended
//! there, as WebGL's canvas is.
use super::controls_attributes::{Controls, camera_state, viewport_css, webgl_perspective};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::retro::{mipmapped, mipmapped_raw};
use super::shadowmap_opacity::Mipmaps;
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

const ASSETS: &str = "/web/gallery/assets/vrml";
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const NAMES: [&str; 16] = [
    "creaseAngle",
    "crystal",
    "house",
    "elevationGrid1",
    "elevationGrid2",
    "extrusion1",
    "extrusion2",
    "extrusion3",
    "lines",
    "linesTransparent",
    "meshWithLines",
    "meshWithTexture",
    "pixelTexture",
    "points",
    "camera",
    "multilineString",
];
/// Object uniform slots ( dynamic offsets ).
const SLOT: u64 = 256;
const SHADER: &str = r#"
struct Frame {
	view: mat4x4<f32>,
	projection: mat4x4<f32>,
	// The ambient irradiance; the directional light's view-space direction
	// and color; the viewport size and the points' scale ( height / 2 ) and
	// pixel ratio.
	ambient: vec4<f32>,
	light_direction: vec4<f32>,
	light_color: vec4<f32>,
	viewport: vec4<f32>,
};
struct Object {
	model_view: mat4x4<f32>,
	normal0: vec4<f32>,
	normal1: vec4<f32>,
	normal2: vec4<f32>,
	// diffuse, opacity; emissive, shininess; specular, -; uv transform rows.
	color: vec4<f32>,
	emissive: vec4<f32>,
	specular: vec4<f32>,
	uv0: vec4<f32>,
	uv1: vec4<f32>,
	// vertexColors, map, flipY, opaque.
	flags: vec4<f32>,
	// size.
	point: vec4<f32>,
};
@group( 0 ) @binding( 0 ) var<uniform> frame: Frame;
@group( 1 ) @binding( 0 ) var<uniform> object: Object;
@group( 2 ) @binding( 0 ) var map: texture_2d<f32>;
@group( 2 ) @binding( 1 ) var map_sampler: sampler;
struct Varyings {
	@builtin( position ) position: vec4<f32>,
	@location( 0 ) view_position: vec3<f32>,
	@location( 1 ) normal: vec3<f32>,
	@location( 2 ) uv: vec2<f32>,
	@location( 3 ) color: vec3<f32>,
};
fn varyings( position: vec3<f32>, normal: vec3<f32>, uv: vec2<f32>, color: vec3<f32> ) -> Varyings {
	let mv = object.model_view * vec4( position, 1.0 );
	var out: Varyings;
	out.position = frame.projection * mv;
	out.view_position = -mv.xyz;
	out.normal = normalize( mat3x3<f32>( object.normal0.xyz, object.normal1.xyz, object.normal2.xyz ) * normal );
	out.uv = ( mat3x3<f32>( object.uv0.xyz, object.uv1.xyz, vec3( object.uv0.w, object.uv1.w, 1.0 ) ) * vec3( uv, 1.0 ) ).xy;
	out.color = color;
	return out;
}
@vertex fn vs( @location( 0 ) position: vec3<f32>, @location( 1 ) normal: vec3<f32>, @location( 2 ) uv: vec2<f32>, @location( 3 ) color: vec3<f32> ) -> Varyings {
	return varyings( position, normal, uv, color );
}
// gl_PointSize = size × scale / -mvPosition.z ( at least a pixel ), as a
// square around the point.
@vertex fn points( @builtin( vertex_index ) corner: u32, @location( 0 ) position: vec3<f32>, @location( 3 ) color: vec3<f32> ) -> Varyings {
	var out = varyings( position, vec3( 0.0, 0.0, 1.0 ), vec2( 0.0 ), color );
	let mv = object.model_view * vec4( position, 1.0 );
	let size = clamp( object.point.x * frame.viewport.w * ( frame.viewport.z / -mv.z ), 1.0, 511.0 );
	let corners = array( vec2( -1.0, -1.0 ), vec2( 1.0, -1.0 ), vec2( -1.0, 1.0 ), vec2( -1.0, 1.0 ), vec2( 1.0, -1.0 ), vec2( 1.0, 1.0 ) );
	out.position = vec4( out.position.xy + corners[ corner ] * size / frame.viewport.xy * out.position.w, out.position.zw );
	return out;
}
fn encode( c: vec3<f32>, alpha: f32 ) -> vec4<f32> {
	return vec4( select( pow( c, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), c * 12.92, c <= vec3( 0.0031308 ) ), alpha );
}
fn diffuse_color( in: Varyings ) -> vec4<f32> {
	var diffuse = object.color;
	if ( object.flags.y > 0.5 ) {
		let uv = select( in.uv, vec2( in.uv.x, 1.0 - in.uv.y ), object.flags.z > 0.5 );
		diffuse *= textureSample( map, map_sampler, uv );
	}
	if ( object.flags.x > 0.5 ) { diffuse = vec4( diffuse.rgb * in.color, diffuse.a ); }
	return diffuse;
}
const RECIPROCAL_PI: f32 = 0.3183098861837907;
@fragment fn phong( in: Varyings, @builtin( front_facing ) front: bool ) -> @location( 0 ) vec4<f32> {
	let diffuse = diffuse_color( in );
	// DOUBLE_SIDED flips back faces; single-sided programs see front faces only.
	let normal = normalize( in.normal ) * select( -1.0, 1.0, front );
	let view = normalize( in.view_position );
	let direction = frame.light_direction.xyz;
	let irradiance = saturate( dot( normal, direction ) ) * frame.light_color.rgb;
	let half_dir = normalize( direction + view );
	let dot_nh = saturate( dot( normal, half_dir ) );
	let dot_vh = saturate( dot( view, half_dir ) );
	let fresnel = exp2( ( -5.55473 * dot_vh - 6.98316 ) * dot_vh );
	let f = object.specular.rgb * ( 1.0 - fresnel ) + vec3( fresnel );
	let shininess = object.emissive.w;
	let d = RECIPROCAL_PI * ( shininess * 0.5 + 1.0 ) * pow( dot_nh, shininess );
	var light = irradiance * RECIPROCAL_PI * diffuse.rgb + irradiance * f * ( 0.25 * d );
	light += frame.ambient.rgb * RECIPROCAL_PI * diffuse.rgb;
	light += object.emissive.rgb;
	return encode( light, select( diffuse.a, 1.0, object.flags.w > 0.5 ) );
}
@fragment fn basic( in: Varyings ) -> @location( 0 ) vec4<f32> {
	let diffuse = diffuse_color( in );
	return encode( diffuse.rgb, select( diffuse.a, 1.0, object.flags.w > 0.5 ) );
}
"#;
#[derive(serde::Deserialize)]
struct Blob {
    offset: usize,
    length: usize,
    #[serde(rename = "type")]
    kind: String,
}
#[derive(serde::Deserialize)]
struct BakedAttribute {
    offset: usize,
    length: usize,
    #[serde(rename = "itemSize")]
    item_size: usize,
}
#[derive(serde::Deserialize)]
struct BakedGeometry {
    attributes: HashMap<String, BakedAttribute>,
    index: Option<Blob>,
    sphere: [f64; 4],
}
#[derive(serde::Deserialize)]
struct BakedData {
    offset: usize,
    length: usize,
    width: u32,
    height: u32,
}
#[derive(serde::Deserialize)]
struct BakedTexture {
    url: Option<String>,
    data: Option<BakedData>,
    wrap: [u32; 2],
    filter: [u32; 2],
    #[serde(rename = "flipY")]
    flip_y: bool,
    matrix: [f64; 9],
}
#[derive(serde::Deserialize)]
struct BakedMaterial {
    id: u32,
    #[serde(rename = "type")]
    kind: String,
    color: [f64; 3],
    emissive: Option<[f64; 3]>,
    specular: Option<[f64; 3]>,
    shininess: Option<f64>,
    opacity: f64,
    transparent: bool,
    side: u32,
    #[serde(rename = "vertexColors")]
    vertex_colors: bool,
    #[serde(rename = "depthWrite")]
    depth_write: bool,
    #[serde(rename = "depthTest")]
    depth_test: bool,
    size: Option<f64>,
    map: Option<usize>,
}
#[derive(serde::Deserialize)]
struct BakedObject {
    id: u32,
    kind: String,
    #[serde(rename = "renderOrder")]
    render_order: f64,
    #[serde(rename = "groupOrder")]
    group_order: f64,
    #[serde(rename = "frustumCulled")]
    frustum_culled: bool,
    #[serde(rename = "matrixWorld")]
    matrix_world: [f64; 16],
    geometry: usize,
    material: BakedMaterial,
}
#[derive(serde::Deserialize)]
struct BakedAsset {
    geometries: Vec<BakedGeometry>,
    textures: Vec<BakedTexture>,
    objects: Vec<BakedObject>,
}
#[derive(serde::Deserialize)]
struct Bake {
    assets: Vec<BakedAsset>,
}
/// A geometry's buffers: position, normal, uv and color ( zeros, or white
/// for the color, when absent ), index, and its counts.
struct Geometry {
    buffers: [wgpu::Buffer; 4],
    index: Option<(wgpu::Buffer, u32)>,
    vertices: u32,
    center: Vector3,
    radius: f64,
}
/// The pipeline variant: program ( 0 phong, 1 basic, 2 lines, 3 points ),
/// culled face, blending, depth write and test.
type Key = (u8, Option<wgpu::Face>, bool, bool, bool);
struct Renderable {
    object: BakedObject,
    world: Matrix4,
    slot: u32,
    texture: usize,
    /// The map's uv transform ( Texture.matrix ) and flipY.
    uv: [f32; 8],
    flip_y: bool,
}
struct Asset {
    geometries: Vec<Geometry>,
    /// Texture bind groups ( the first is the white stand-in ).
    textures: Vec<wgpu::BindGroup>,
    renderables: Vec<Renderable>,
}
struct Targets {
    size: (u32, u32),
    format: wgpu::TextureFormat,
    depth: wgpu::TextureView,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    pending: bool,
    asset: usize,
    assets: Vec<Asset>,
    frame: wgpu::Buffer,
    frame_group: wgpu::BindGroup,
    objects: wgpu::Buffer,
    object_group: wgpu::BindGroup,
    /// Whether the next frame resets the controls ( an asset loaded ).
    reset: bool,
    layouts: [wgpu::BindGroupLayout; 3],
    pipelines: HashMap<(Key, wgpu::TextureFormat), wgpu::RenderPipeline>,
    targets: Option<Targets>,
}
/// The renderer's sampler for a three.js filter and wrap pair.
fn sampler(r: &Renderer, t: &BakedTexture) -> wgpu::Sampler {
    let wrap = |w| match w {
        1001 => wgpu::AddressMode::ClampToEdge,
        1002 => wgpu::AddressMode::MirrorRepeat,
        _ => wgpu::AddressMode::Repeat,
    };
    let linear = |f| {
        if f >= 1006 {
            wgpu::FilterMode::Linear
        } else {
            wgpu::FilterMode::Nearest
        }
    };
    r.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wrap(t.wrap[0]),
        address_mode_v: wrap(t.wrap[1]),
        mag_filter: linear(t.filter[0]),
        min_filter: linear(t.filter[1]),
        mipmap_filter: if t.filter[1] == 1008 || t.filter[1] == 1005 {
            wgpu::FilterMode::Linear
        } else {
            wgpu::FilterMode::Nearest
        },
        ..Default::default()
    })
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.1,
            far: 1e10,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-10., 5., 10.);
        let mut controls = Controls::new(Some(0.05), (1., 200.), std::f64::consts::PI, true);
        controls.update(s, c)?;
        let bake: Bake = serde_json::from_slice(&fetch(&format!("{ASSETS}/scenes.json")).await?)
            .map_err(|e| Error::Asset(e.to_string()))?;
        let bin = fetch(&format!("{ASSETS}/scenes.bin")).await?;
        let slice = |offset: usize, bytes: usize| -> Result<&[u8]> {
            bin.get(offset..offset + bytes)
                .ok_or(Error::Invalid("vrml bake range"))
        };
        let entry = |binding, visibility, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty,
            count: None,
        };
        let uniform = |dynamic| wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: dynamic,
            min_binding_size: None,
        };
        let both = wgpu::ShaderStages::VERTEX_FRAGMENT;
        let layouts = [
            r.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("vrml frame"),
                    entries: &[entry(0, both, uniform(false))],
                }),
            r.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("vrml object"),
                    entries: &[entry(0, both, uniform(true))],
                }),
            r.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("vrml map"),
                    entries: &[
                        entry(
                            0,
                            wgpu::ShaderStages::FRAGMENT,
                            wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                        ),
                        entry(
                            1,
                            wgpu::ShaderStages::FRAGMENT,
                            wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        ),
                    ],
                }),
        ];
        let init = |label, contents: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        let mut mipmaps = Mipmaps::new(r);
        let white = mipmapped_raw(
            r,
            &mut mipmaps,
            &[255; 4],
            (1, 1),
            4,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        );
        let white_sampler = r.device.create_sampler(&Default::default());
        let texture_group = |view: &wgpu::TextureView, sampler: &wgpu::Sampler| {
            r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("vrml map"),
                layout: &layouts[2],
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
            })
        };
        let mut assets = vec![];
        let mut slots = 0u32;
        for baked in bake.assets {
            let mut geometries = vec![];
            for g in &baked.geometries {
                let position = g
                    .attributes
                    .get("position")
                    .ok_or(Error::Invalid("vrml position"))?;
                let vertices = position.length / position.item_size;
                let mut buffers = vec![];
                for (name, size, fill) in [
                    ("position", 3, 0f32),
                    ("normal", 3, 0.),
                    ("uv", 2, 0.),
                    ("color", 3, 1.),
                ] {
                    buffers.push(match g.attributes.get(name) {
                        Some(a) => init(
                            name,
                            slice(a.offset, a.length * 4)?,
                            wgpu::BufferUsages::VERTEX,
                        ),
                        None => init(
                            name,
                            bytemuck::cast_slice(&vec![fill; vertices * size]),
                            wgpu::BufferUsages::VERTEX,
                        ),
                    });
                }
                let buffers: [wgpu::Buffer; 4] = buffers
                    .try_into()
                    .map_err(|_| Error::Invalid("vrml buffers"))?;
                let vertices = vertices as u32;
                let index = match &g.index {
                    Some(b) => {
                        let indices: Vec<u32> = if b.kind == "Uint16Array" {
                            slice(b.offset, b.length * 2)?
                                .as_chunks::<2>()
                                .0
                                .iter()
                                .map(|&c| u32::from(u16::from_le_bytes(c)))
                                .collect()
                        } else {
                            bytemuck::pod_collect_to_vec(slice(b.offset, b.length * 4)?)
                        };
                        Some((
                            init(
                                "vrml index",
                                bytemuck::cast_slice(&indices),
                                wgpu::BufferUsages::INDEX,
                            ),
                            b.length as u32,
                        ))
                    }
                    None => None,
                };
                geometries.push(Geometry {
                    buffers,
                    index,
                    vertices,
                    center: Vector3::new(g.sphere[0], g.sphere[1], g.sphere[2]),
                    radius: g.sphere[3],
                });
            }
            let mut textures = vec![texture_group(&white, &white_sampler)];
            for t in &baked.textures {
                let view = if let Some(url) = &t.url {
                    let image =
                        decode_texture_image(&fetch(&format!("{ASSETS}/{url}")).await?).await?;
                    mipmapped(r, &mut mipmaps, &image, wgpu::TextureFormat::Rgba8UnormSrgb)
                } else if let Some(d) = &t.data {
                    // A PixelTexture's DataTexture: no mipmaps, nearest.
                    let texture = r.device.create_texture_with_data(
                        &r.queue,
                        &wgpu::TextureDescriptor {
                            label: Some("vrml pixel texture"),
                            size: wgpu::Extent3d {
                                width: d.width,
                                height: d.height,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgpu::TextureDimension::D2,
                            format: wgpu::TextureFormat::Rgba8UnormSrgb,
                            usage: wgpu::TextureUsages::TEXTURE_BINDING
                                | wgpu::TextureUsages::COPY_DST,
                            view_formats: &[],
                        },
                        wgpu::util::TextureDataOrder::LayerMajor,
                        slice(d.offset, d.length)?,
                    );
                    texture.create_view(&Default::default())
                } else {
                    return Err(Error::Invalid("vrml texture"));
                };
                textures.push(texture_group(&view, &sampler(r, t)));
            }
            let mut renderables = vec![];
            for object in baked.objects {
                let world = Matrix4::from_cols_array(&object.matrix_world);
                let map = object.material.map.and_then(|m| baked.textures.get(m));
                let e = map.map_or([1., 0., 0., 0., 1., 0., 0., 0., 1.], |t| t.matrix);
                let uv = [e[0], e[1], e[2], e[6], e[3], e[4], e[5], e[7]].map(|v| v as f32);
                renderables.push(Renderable {
                    texture: object.material.map.map_or(0, |m| m + 1),
                    flip_y: map.is_some_and(|t| t.flip_y),
                    object,
                    world,
                    slot: slots,
                    uv,
                });
                slots += 1;
            }
            assets.push(Asset {
                geometries,
                textures,
                renderables,
            });
        }
        // The fixed parts of every object's uniforms; the frame writes the
        // model-view and normal matrices of the drawn ones.
        let objects = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vrml objects"),
            size: SLOT * u64::from(slots.max(1)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        for asset in &assets {
            for item in &asset.renderables {
                let m = &item.object.material;
                let mut data = vec![0f32; 28];
                data[..3].copy_from_slice(&m.color.map(|v| v as f32));
                data[3] = m.opacity as f32;
                let emissive = m.emissive.unwrap_or([0.; 3]);
                data[4..7].copy_from_slice(&emissive.map(|v| v as f32));
                data[7] = m.shininess.unwrap_or(30.) as f32;
                let specular = m.specular.unwrap_or([0.; 3]);
                data[8..11].copy_from_slice(&specular.map(|v| v as f32));
                data[12..20].copy_from_slice(&item.uv);
                data[20] = f32::from(m.vertex_colors);
                data[21] = f32::from(m.map.is_some());
                data[22] = f32::from(item.flip_y);
                // OPAQUE: an opaque material writes alpha 1.
                data[23] = f32::from(!m.transparent);
                data[24] = m.size.unwrap_or(1.) as f32;
                r.queue.write_buffer(
                    &objects,
                    SLOT * u64::from(item.slot) + 112,
                    bytemuck::cast_slice(&data),
                );
            }
        }
        let frame = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vrml frame"),
            size: 64 * 2 + 16 * 4,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let frame_group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("vrml frame"),
            layout: &layouts[0],
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: frame.as_entire_binding(),
            }],
        });
        let object_group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("vrml object"),
            layout: &layouts[1],
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &objects,
                    offset: 0,
                    size: wgpu::BufferSize::new(224),
                }),
            }],
        });
        Ok(Self {
            controls,
            pending: true,
            asset: 2,
            assets,
            frame,
            frame_group,
            objects,
            object_group,
            reset: false,
            layouts,
            pipelines: HashMap::new(),
            targets: None,
        })
    }
    fn pipeline(
        &mut self,
        r: &Renderer,
        key: Key,
        format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        if let Some(p) = self.pipelines.get(&(key, format)) {
            return p.clone();
        }
        let (program, cull, blend, depth_write, depth_test) = key;
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("vrml"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("vrml"),
                bind_group_layouts: &[&self.layouts[0], &self.layouts[1], &self.layouts[2]],
                push_constant_ranges: &[],
            });
        let attribute = |location, format| {
            [wgpu::VertexAttribute {
                format,
                offset: 0,
                shader_location: location,
            }]
        };
        let attributes = [
            attribute(0, wgpu::VertexFormat::Float32x3),
            attribute(1, wgpu::VertexFormat::Float32x3),
            attribute(2, wgpu::VertexFormat::Float32x2),
            attribute(3, wgpu::VertexFormat::Float32x3),
        ];
        // Points step their attributes per square.
        let step = if program == 3 {
            wgpu::VertexStepMode::Instance
        } else {
            wgpu::VertexStepMode::Vertex
        };
        let strides = [12, 12, 8, 12];
        let buffers: Vec<_> = (0..4)
            .map(|k| wgpu::VertexBufferLayout {
                array_stride: strides[k],
                step_mode: step,
                attributes: &attributes[k],
            })
            .collect();
        let pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("vrml"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some(if program == 3 { "points" } else { "vs" }),
                    compilation_options: Default::default(),
                    buffers: &buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some(if program == 0 { "phong" } else { "basic" }),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        // NormalBlending ( not premultiplied ).
                        blend: blend.then_some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: if program == 2 {
                        wgpu::PrimitiveTopology::LineList
                    } else {
                        wgpu::PrimitiveTopology::TriangleList
                    },
                    cull_mode: cull,
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH,
                    depth_write_enabled: depth_write,
                    depth_compare: if depth_test {
                        wgpu::CompareFunction::LessEqual
                    } else {
                        wgpu::CompareFunction::Always
                    },
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        self.pipelines.insert((key, format), pipeline.clone());
        pipeline
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        self.pending |= animate;
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let resized = self
            .targets
            .as_ref()
            .is_none_or(|t| t.size != (out.width, out.height) || t.format != out.options.format);
        if resized {
            let depth = r
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("vrml depth"),
                    size: wgpu::Extent3d {
                        width: out.width,
                        height: out.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: DEPTH,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                })
                .create_view(&Default::default());
            self.targets = Some(Targets {
                size: (out.width, out.height),
                format: out.options.format,
                depth,
                screen: RenderTarget::with_options(
                    &r.device,
                    out.width,
                    out.height,
                    RenderTargetOptions {
                        samples: 0,
                        depth_buffer: false,
                        ..out.options.clone()
                    },
                )?,
            });
        }
        if !std::mem::take(&mut self.pending) && !resized {
            return Ok(true);
        }
        // The load callback's controls.reset(): the saved camera and
        // target, then update().
        if std::mem::take(&mut self.reset) {
            s.get_mut(c)?.position = Vector3::new(-10., 5., 10.);
            self.controls.set_target(Vector3::ZERO);
            self.controls.update(s, c)?;
        }
        // animate(): controls.update() ( damping ), then the render.
        self.controls.frame_update(s, c)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (fov, near, far, aspect) = match camera {
            Camera::Perspective(p) => (p.fov, p.near, p.far, p.aspect),
            _ => (60., 0.1, 1e10, 1.),
        };
        let view = world.inverse();
        let projection = camera.projection_matrix()?;
        // WebGLRenderer's sort keys use its own clip space.
        let sort_projection = webgl_perspective(fov, aspect, near, far) * view;
        let frustum = Frustum::from_projection(projection * view);
        let (css_w, css_h, dpr) = viewport_css();
        let _ = css_w;
        let mut data: Vec<f32> = vec![];
        for m in [view, projection] {
            data.extend(m.to_cols_array().iter().map(|&v| v as f32));
        }
        data.extend([1.2, 1.2, 1.2, 1.]);
        let d = view
            .transform_vector3(Vector3::new(200., 200., 200.).normalize())
            .normalize();
        data.extend([d.x as f32, d.y as f32, d.z as f32, 0.]);
        data.extend([2., 2., 2., 1.]);
        data.extend([
            out.width as f32,
            out.height as f32,
            (css_h * 0.5) as f32,
            dpr as f32,
        ]);
        r.queue
            .write_buffer(&self.frame, 0, bytemuck::cast_slice(&data));
        let asset = &self.assets[self.asset];
        // projectObject: culled renderables into the opaque and transparent
        // lists with their sort depth.
        let mut opaque = vec![];
        let mut transparent = vec![];
        for (k, item) in asset.renderables.iter().enumerate() {
            let g = &asset.geometries[item.object.geometry];
            if item.object.frustum_culled {
                let scale = item
                    .world
                    .to_scale_rotation_translation()
                    .0
                    .abs()
                    .max_element();
                if !frustum.intersects_sphere(Sphere {
                    center: item.world.transform_point3(g.center),
                    radius: g.radius * scale,
                }) {
                    continue;
                }
            }
            let z = (sort_projection * item.world * g.center.extend(1.)).z;
            if item.object.material.transparent {
                transparent.push((k, z));
            } else {
                opaque.push((k, z));
            }
        }
        let key = |k: usize| {
            let o = &asset.renderables[k].object;
            (o.group_order, o.render_order, o.material.id, o.id)
        };
        opaque.sort_by(|&(a, za), &(b, zb)| {
            let (ka, kb) = (key(a), key(b));
            ka.0.total_cmp(&kb.0)
                .then(ka.1.total_cmp(&kb.1))
                .then(ka.2.cmp(&kb.2))
                .then(za.total_cmp(&zb))
                .then(ka.3.cmp(&kb.3))
        });
        transparent.sort_by(|&(a, za), &(b, zb)| {
            let (ka, kb) = (key(a), key(b));
            ka.0.total_cmp(&kb.0)
                .then(ka.1.total_cmp(&kb.1))
                .then(zb.total_cmp(&za))
                .then(ka.3.cmp(&kb.3))
        });
        // The drawn objects' model-view and normal matrices.
        for &(k, _) in opaque.iter().chain(&transparent) {
            let item = &asset.renderables[k];
            let model_view = view * item.world;
            let normal = Matrix3::from_mat4(model_view).inverse().transpose();
            let mut data: Vec<f32> = model_view
                .to_cols_array()
                .iter()
                .map(|&v| v as f32)
                .collect();
            for col in [normal.x_axis, normal.y_axis, normal.z_axis] {
                data.extend([col.x as f32, col.y as f32, col.z as f32, 0.]);
            }
            r.queue.write_buffer(
                &self.objects,
                SLOT * u64::from(item.slot),
                bytemuck::cast_slice(&data),
            );
        }
        // Each draw: its pipeline key and its passes ( a transparent
        // double-sided object draws its back, then its front ).
        let mut draws = vec![];
        for (list, blend) in [(&opaque, false), (&transparent, true)] {
            for &(k, _) in list.iter() {
                let o = &asset.renderables[k].object;
                let m = &o.material;
                let program = match (o.kind.as_str(), m.kind.as_str()) {
                    ("lines", _) => 2,
                    ("points", _) => 3,
                    (_, "MeshPhongMaterial") => 0,
                    _ => 1,
                };
                let faces: Vec<Option<wgpu::Face>> = if program >= 2 {
                    vec![None]
                } else {
                    match m.side {
                        1 => vec![Some(wgpu::Face::Front)],
                        2 if blend => vec![Some(wgpu::Face::Front), Some(wgpu::Face::Back)],
                        2 => vec![None],
                        _ => vec![Some(wgpu::Face::Back)],
                    }
                };
                for cull in faces {
                    draws.push((k, (program, cull, blend, m.depth_write, m.depth_test)));
                }
            }
        }
        let format = out.options.format;
        let pipelines: Vec<wgpu::RenderPipeline> = draws
            .iter()
            .map(|&(_, key)| self.pipeline(r, key, format))
            .collect();
        let asset = &self.assets[self.asset];
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("vrml targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("vrml"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_bind_group(0, &self.frame_group, &[]);
            for (&(k, (program, _, _, _, _)), pipeline) in draws.iter().zip(&pipelines) {
                let item = &asset.renderables[k];
                let g = &asset.geometries[item.object.geometry];
                pass.set_pipeline(pipeline);
                pass.set_bind_group(1, &self.object_group, &[item.slot * SLOT as u32]);
                pass.set_bind_group(2, &asset.textures[item.texture], &[]);
                for (slot, buffer) in g.buffers.iter().enumerate() {
                    pass.set_vertex_buffer(slot as u32, buffer.slice(..));
                }
                if program == 3 {
                    pass.draw(0..6, 0..g.vertices);
                } else if let Some((index, count)) = &g.index {
                    pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..*count, 0, 0..1);
                } else {
                    pass.draw(0..g.vertices, 0..1);
                }
            }
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// The asset ( by index ): the loaded scene replaces the last and the
    /// controls reset to their saved state.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 || value < 0. || value as usize >= NAMES.len() {
            return Err(Error::Invalid("vrml parameter"));
        }
        self.asset = value as usize;
        self.reset = true;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
