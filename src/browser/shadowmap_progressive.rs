//! webgpu_shadowmap_progressive: ProgressiveLightMapGPU accumulating the soft
//! shadows of four randomly placed directional lights into a 1024² float
//! lightmap over the ground and the ShadowmappableMesh model, with OrbitControls
//! and two TransformControls ( the light origin and the model ). Each frame, as
//! the page's animate(): the controls update, the lightmap pass renders the
//! four shadow maps and draws the seam-blurring plane and both objects through
//! their packed second uv set into the active ping-pong target, blending with
//! the previous one by the blend window; the lights then move by the fixture's
//! seeded Math.random ( toward the light origin, or over the hemisphere for
//! ambient occlusion ); and the main pass draws the ground and model with the
//! lightmap under the linear fog, the gizmos over them ( scene meshes ) and
//! the output pass. potpack's layout of the two uv boxes is computed at load,
//! as addObjectsToLightMap() does. Every stage runs the WGSL three.js r186
//! generates for the page ( in `shadowmap_progressive/` ); the debug lightmap
//! plane is not reproduced.
use super::controls_attributes::{Controls, camera_state, viewport_css};
use super::gltf_viewer::load_asset;
use super::lights_projector::{m3, m4};
use super::transform_controls::{Event, TransformControls};
use super::wgsl_bind::{Spec, Uniforms, attribute};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const FLOAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SCENE_DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const RESOLUTION: u32 = 1024;
const SHADOW: u32 = 1024;
const LIGHTS: usize = 4;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("shadowmap_progressive/", $name, ".wgsl"))
    };
}
const BLUR: (&str, &str) = (wgsl!("blur_vs"), wgsl!("blur_fs"));
const SHADOW_SHADERS: (&str, &str) = (
    wgsl!("shadow_vs"),
    include_str!("fog_volume/shadow_fs.wgsl"),
);
const LIGHTMAP: (&str, &str) = (wgsl!("lightmap_vs"), wgsl!("lightmap_fs"));
const MAIN: (&str, &str) = (wgsl!("main_vs"), wgsl!("main_fs"));
const OUTPUT: (&str, &str) = (wgsl!("output_vs"), wgsl!("output_fs"));
/// The page's fixture Math.random: a 32-bit LCG from 186.
struct FixtureRandom(u32);
impl FixtureRandom {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(self.0) / 4294967296.
    }
}
/// A lightmapped object: normal, position, uv and its packed uv1, the index.
struct Object {
    buffers: [wgpu::Buffer; 4],
    index: wgpu::Buffer,
    count: u32,
    /// Its shadow, lightmap and main object uniforms.
    shadow: Uniforms,
    lightmap: Uniforms,
    main: Uniforms,
}
/// potpack( boxes ) for the page's equal boxes: their ( x, y ) and the
/// container's ( w, h ).
fn potpack(sizes: &[f64]) -> (Vec<(f64, f64)>, (f64, f64)) {
    let area: f64 = sizes.iter().map(|s| s * s).sum();
    let max_width = sizes.iter().copied().fold(0., f64::max);
    let mut order: Vec<usize> = (0..sizes.len()).collect();
    // boxes.sort( ( a, b ) => b.h - a.h ): a stable sort.
    order.sort_by(|&a, &b| sizes[b].total_cmp(&sizes[a]));
    let start = (area / 0.95).sqrt().ceil().max(max_width);
    let mut spaces = vec![(0., 0., start, f64::INFINITY)];
    let (mut width, mut height) = (0f64, 0f64);
    let mut out = vec![(0., 0.); sizes.len()];
    for &b in &order {
        let (w, h) = (sizes[b], sizes[b]);
        for i in (0..spaces.len()).rev() {
            let space = spaces[i];
            if w > space.2 || h > space.3 {
                continue;
            }
            out[b] = (space.0, space.1);
            height = height.max(space.1 + h);
            width = width.max(space.0 + w);
            if w == space.2 && h == space.3 {
                let last = spaces.pop().unwrap_or(space);
                if i < spaces.len() {
                    spaces[i] = last;
                }
            } else if h == space.3 {
                spaces[i].0 += w;
                spaces[i].2 -= w;
            } else if w == space.2 {
                spaces[i].1 += h;
                spaces[i].3 -= h;
            } else {
                spaces.push((space.0 + w, space.1, space.2 - w, h));
                spaces[i].1 += h;
                spaces[i].3 -= h;
            }
            break;
        }
    }
    (out, (width, height))
}
struct Targets {
    width: u32,
    height: u32,
    samples: u32,
    format: wgpu::TextureFormat,
    main_pipeline: wgpu::RenderPipeline,
    main_groups: Vec<Vec<wgpu::BindGroup>>,
    output_pipeline: wgpu::RenderPipeline,
    output_groups: Vec<wgpu::BindGroup>,
    scene: RenderTarget,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    orbit_enabled: bool,
    gizmos: [TransformControls; 2],
    /// The light origin and the model's group ( with the lights' target ).
    origin: Object3D,
    model: Object3D,
    /// Enable, Blur Edges, Blend Window, Light Radius, Ambient Weight and
    /// Debug Lightmap.
    params: [f64; 6],
    random: FixtureRandom,
    lights: [Vector3; LIGHTS],
    /// The lights' target as its world matrix last stood: the shadow cameras
    /// look at it before the lightmap pass refreshes it ( the origin at first ).
    shadow_target: Vector3,
    /// The ground and the model's faces.
    objects: [Object; 2],
    blur_plane: (wgpu::Buffer, wgpu::Buffer),
    /// The output pass's fullscreen triangle.
    fullscreen: wgpu::Buffer,
    /// The two ping-pong lightmaps and their shared depth.
    maps: [wgpu::TextureView; 2],
    map_depth: wgpu::TextureView,
    buffer1_active: bool,
    shadows: [(wgpu::TextureView, wgpu::TextureView); LIGHTS],
    shadow_uniforms: [Uniforms; LIGHTS],
    lightmap_render: Uniforms,
    _blur: Uniforms,
    main_render: Uniforms,
    output: Uniforms,
    shadow_pipeline: wgpu::RenderPipeline,
    blur_pipeline: wgpu::RenderPipeline,
    lightmap_pipeline: wgpu::RenderPipeline,
    /// Bind groups: per light and object the shadow pass's, per previous
    /// map the blur plane's and per previous map and object the lightmap's.
    shadow_groups: Vec<Vec<Vec<wgpu::BindGroup>>>,
    blur_groups: Vec<Vec<wgpu::BindGroup>>,
    lightmap_groups: Vec<Vec<Vec<wgpu::BindGroup>>>,
    clamp: wgpu::Sampler,
    queue: Vec<(u32, f64, f64)>,
    /// A frame of animate() is due ( the clock advanced ); other renders
    /// present the last frame again.
    pending: bool,
    targets: Option<Targets>,
}
fn vertex(r: &Renderer, label: &str, data: &[f32]) -> wgpu::Buffer {
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX,
        })
}
fn texture(r: &Renderer, label: &str, size: u32, format: wgpu::TextureFormat) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
        .create_view(&Default::default())
}
/// sRGB hex to linear components.
fn linear(hex: u32) -> [f64; 3] {
    [16, 8, 0].map(|shift| {
        let c = f64::from((hex >> shift) & 255) / 255.;
        if c < 0.04045 {
            c * 0.0773993808
        } else {
            (c * 0.9478672986 + 0.0521327014).powf(2.4)
        }
    })
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 100., 200.);
        n.quaternion = Quaternion::IDENTITY;
        // OrbitControls: damped ( 0.05 ), screen-space panning, 100 – 500, a
        // maximum polar angle of π / 1.5, around ( 0, 100, 0 ).
        let mut controls = Controls::new(Some(0.05), (100., 500.), PI / 1.5, true);
        controls.set_target(Vector3::new(0., 100., 0.));
        controls.update(s, c)?;
        let origin = s.insert(NodeKind::Group);
        s.get_mut(origin)?.position = Vector3::new(60., 150., 100.);
        let model = s.insert(NodeKind::Group);
        let n = s.get_mut(model)?;
        n.scale = Vector3::splat(2.);
        n.position = Vector3::new(0., -16., 0.);
        let mut gizmos = [TransformControls::new(s, c)?, TransformControls::new(s, c)?];
        gizmos[0].attach(s, origin)?;
        gizmos[1].attach(s, model)?;
        for g in &mut gizmos {
            g.events.clear();
            g.update(s)?;
        }
        // The model: ShadowmappableMesh's faces ( its edges render on no layer ).
        let (asset, buffers, _) =
            load_asset("/web/gallery/assets/gltf/ShadowmappableMesh.glb").await?;
        let primitive = asset
            .meshes()
            .next()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("lightmap model"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let positions: Vec<f32> = reader
            .read_positions()
            .ok_or(Error::Invalid("model positions"))?
            .flatten()
            .collect();
        let normals: Vec<f32> = reader
            .read_normals()
            .ok_or(Error::Invalid("model normals"))?
            .flatten()
            .collect();
        let uvs: Vec<f32> = reader
            .read_tex_coords(0)
            .ok_or(Error::Invalid("model uv"))?
            .into_f32()
            .flatten()
            .collect();
        let indices: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("model index"))?
            .into_u32()
            .collect();
        let plane = PlaneGeometry::build(600., 600., 1, 1)?;
        let read = |g: &BufferGeometry, name: &str| -> Result<Vec<f32>> {
            match g.attributes.get(name) {
                Some(Attribute::F32(a)) => Ok(a.array().to_vec()),
                _ => Err(Error::Invalid("plane attribute")),
            }
        };
        let ground = (
            read(&plane, "normal")?,
            read(&plane, "position")?,
            read(&plane, "uv")?,
            plane.index.clone().ok_or(Error::Invalid("plane index"))?,
        );
        // addObjectsToLightMap(): the uv boxes packed into one lightmap.
        let padding = 3. / f64::from(RESOLUTION);
        let size = 1. + padding * 2.;
        let (boxes, (w, h)) = potpack(&[size, size]);
        let uv1 = |uvs: &[f32], (bx, by): (f64, f64)| -> Vec<f32> {
            uvs.chunks(2)
                .flat_map(|uv| {
                    [
                        ((f64::from(uv[0]) + bx + padding) / w) as f32,
                        (1. - (f64::from(uv[1]) + by + padding) / h) as f32,
                    ]
                })
                .collect()
        };
        let object = |label: &str,
                      (normal, position, uv, index): (Vec<f32>, Vec<f32>, Vec<f32>, Vec<u32>),
                      packed: (f64, f64)|
         -> Result<Object> {
            Ok(Object {
                buffers: [
                    vertex(r, label, &normal),
                    vertex(r, label, &position),
                    vertex(r, label, &uv),
                    vertex(r, label, &uv1(&uv, packed)),
                ],
                index: r
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(label),
                        contents: bytemuck::cast_slice(&index),
                        usage: wgpu::BufferUsages::INDEX,
                    }),
                count: index.len() as u32,
                shadow: Uniforms::new(r, label, &[SHADOW_SHADERS.0, SHADOW_SHADERS.1])?,
                lightmap: Uniforms::new(r, label, &[LIGHTMAP.0, LIGHTMAP.1])?,
                main: Uniforms::new(r, label, &[MAIN.0, MAIN.1])?,
            })
        };
        let objects = [
            object("ground", ground, boxes[0])?,
            object(
                "lightmap model",
                (normals, positions, uvs, indices),
                boxes[1],
            )?,
        ];
        let blur_geometry = PlaneGeometry::build(1., 1., 1, 1)?;
        let blur_plane = (
            vertex(r, "blur plane", &read(&blur_geometry, "uv")?),
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("blur plane"),
                    contents: bytemuck::cast_slice(
                        &blur_geometry
                            .index
                            .clone()
                            .ok_or(Error::Invalid("blur index"))?,
                    ),
                    usage: wgpu::BufferUsages::INDEX,
                }),
        );
        let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let compare = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let maps = [
            texture(r, "progressive lightmap 1", RESOLUTION, FLOAT),
            texture(r, "progressive lightmap 2", RESOLUTION, FLOAT),
        ];
        let shadows = std::array::from_fn(|_| {
            (
                texture(r, "shadow color", SHADOW, wgpu::TextureFormat::Rgba8Unorm),
                texture(r, "shadow depth", SHADOW, DEPTH),
            )
        });
        let f3 = |location| {
            [wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: location,
            }]
        };
        let f2 = |location| {
            [wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: location,
            }]
        };
        let (p0, uv0) = (f3(0), f2(0));
        let (n0, p1, uv1_2) = (f3(0), f3(1), f2(2));
        let shadow_pipeline = Spec {
            label: "progressive shadow",
            shaders: SHADOW_SHADERS,
            buffers: &[attribute(&p0, 12)],
            format: wgpu::TextureFormat::Rgba8Unorm,
            blend: false,
            depth: Some((DEPTH, wgpu::CompareFunction::LessEqual, true, (0, 0.))),
            cw: true,
            cull: Some(wgpu::Face::Back),
            samples: 1,
        }
        .build(r);
        let blur_pipeline = Spec {
            label: "blurring plane",
            shaders: BLUR,
            buffers: &[attribute(&uv0, 8)],
            format: FLOAT,
            blend: false,
            depth: Some((DEPTH, wgpu::CompareFunction::LessEqual, false, (3, -1.))),
            cw: false,
            cull: Some(wgpu::Face::Back),
            samples: 1,
        }
        .build(r);
        let lightmap_pipeline = Spec {
            label: "lightmap surface",
            shaders: LIGHTMAP,
            buffers: &[attribute(&n0, 12), attribute(&p1, 12), attribute(&uv1_2, 8)],
            format: FLOAT,
            blend: false,
            depth: Some((DEPTH, wgpu::CompareFunction::LessEqual, true, (0, 0.))),
            cw: false,
            cull: Some(wgpu::Face::Back),
            samples: 1,
        }
        .build(r);
        let shadow_uniforms: [Uniforms; LIGHTS] = [
            Uniforms::new(r, "shadow", &[SHADOW_SHADERS.0, SHADOW_SHADERS.1])?,
            Uniforms::new(r, "shadow", &[SHADOW_SHADERS.0, SHADOW_SHADERS.1])?,
            Uniforms::new(r, "shadow", &[SHADOW_SHADERS.0, SHADOW_SHADERS.1])?,
            Uniforms::new(r, "shadow", &[SHADOW_SHADERS.0, SHADOW_SHADERS.1])?,
        ];
        let lightmap_render = Uniforms::new(r, "lightmap", &[LIGHTMAP.0, LIGHTMAP.1])?;
        let blur = Uniforms::new(r, "blurring plane", &[BLUR.0, BLUR.1])?;
        let identity3 = m3(Matrix4::IDENTITY);
        let names: Vec<String> = (1..=8).map(|i| format!("nodeUniform{i}")).collect();
        let values: Vec<(&str, &[f64])> =
            names.iter().map(|n| (n.as_str(), &identity3[..])).collect();
        blur.write(r, "objectStruct", &values)?;
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        // Per light: each object's shadow-pass groups ( the light's render
        // struct, the object's object struct ).
        let mut shadow_groups = vec![];
        for light in &shadow_uniforms {
            let mut per_object = vec![];
            for o in &objects {
                let mut groups =
                    light.bind(r, |g| shadow_pipeline.get_bind_group_layout(g), &[])?;
                groups.truncate(1);
                groups.extend(
                    o.shadow
                        .bind(r, |g| shadow_pipeline.get_bind_group_layout(g), &[])?
                        .into_iter()
                        .skip(1),
                );
                per_object.push(groups);
            }
            shadow_groups.push(per_object);
        }
        let blur_groups = maps
            .iter()
            .map(|previous| {
                blur.bind(
                    r,
                    |g| blur_pipeline.get_bind_group_layout(g),
                    &[
                        ("nodeUniform0", tex(previous)),
                        ("nodeUniform0_sampler", sampler(&clamp)),
                    ],
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let shadow_names = [
            "nodeUniform16",
            "nodeUniform26",
            "nodeUniform36",
            "nodeUniform46",
        ];
        let samplers: Vec<String> = shadow_names
            .iter()
            .map(|n| format!("{n}_sampler"))
            .collect();
        let mut lightmap_groups = vec![];
        for previous in &maps {
            let mut per_object = vec![];
            for o in &objects {
                let mut resources: Vec<(&str, wgpu::BindingResource)> = vec![
                    ("nodeUniform50", tex(previous)),
                    ("nodeUniform50_sampler", sampler(&clamp)),
                ];
                for (i, (_, depth)) in shadows.iter().enumerate() {
                    resources.push((shadow_names[i], tex(depth)));
                    resources.push((&samplers[i], sampler(&compare)));
                }
                let mut groups = lightmap_render.bind(
                    r,
                    |g| lightmap_pipeline.get_bind_group_layout(g),
                    &resources,
                )?;
                groups.truncate(1);
                groups.extend(
                    o.lightmap
                        .bind(
                            r,
                            |g| lightmap_pipeline.get_bind_group_layout(g),
                            &resources,
                        )?
                        .into_iter()
                        .skip(1),
                );
                per_object.push(groups);
            }
            lightmap_groups.push(per_object);
        }
        let demo = Self {
            controls,
            orbit_enabled: true,
            gizmos,
            origin,
            model,
            params: [1., 1., 200., 50., 0.5, 0.],
            random: FixtureRandom(186),
            lights: [Vector3::splat(200.); LIGHTS],
            shadow_target: Vector3::ZERO,
            objects,
            blur_plane,
            fullscreen: vertex(
                r,
                "output triangle",
                &[-1., 3., 0., -1., -1., 0., 3., -1., 0.],
            ),
            maps,
            map_depth: texture(r, "lightmap depth", RESOLUTION, DEPTH),
            buffer1_active: false,
            shadows,
            shadow_uniforms,
            lightmap_render,
            _blur: blur,
            main_render: Uniforms::new(r, "progressive main", &[MAIN.0, MAIN.1])?,
            output: Uniforms::new(r, "progressive output", &[OUTPUT.0, OUTPUT.1])?,
            shadow_pipeline,
            blur_pipeline,
            lightmap_pipeline,
            shadow_groups,
            blur_groups,
            lightmap_groups,
            clamp,
            queue: vec![],
            pending: true,
            targets: None,
        };
        demo.output.write(
            r,
            "objectStruct",
            &[("nodeUniform4", &m4(Matrix4::IDENTITY))],
        )?;
        Ok(demo)
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        self.pending |= animate;
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        for (kind, x, y) in std::mem::take(&mut self.queue) {
            self.pointer(s, kind, x, y)?;
        }
        Ok(())
    }
    /// The object's world matrix and normal matrix.
    fn object_matrix(&self, s: &Scene, i: usize) -> Result<Matrix4> {
        Ok(if i == 0 {
            // The ground: rotation.x = −π / 2 at y = −0.1.
            Matrix4::from_translation(Vector3::new(0., -0.1, 0.))
                * Matrix4::from_rotation_x(-PI / 2.)
        } else {
            let n = s.get(self.model)?;
            Matrix4::from_scale_rotation_translation(n.scale, n.quaternion, n.position)
        })
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let scene = RenderTarget::with_options(
            &r.device,
            out.width,
            out.height,
            RenderTargetOptions {
                format: HALF,
                samples,
                ..Default::default()
            },
        )?;
        let f3 = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 1,
        }];
        let f2 = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        }];
        let main_pipeline = Spec {
            label: "progressive main",
            shaders: MAIN,
            buffers: &[attribute(&f2, 8), attribute(&f3, 12)],
            format: HALF,
            blend: false,
            depth: Some((SCENE_DEPTH, wgpu::CompareFunction::LessEqual, true, (0, 0.))),
            cw: false,
            cull: Some(wgpu::Face::Back),
            samples,
        }
        .build(r);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let mut main_groups = vec![];
        for o in &self.objects {
            let resources = [
                ("nodeUniform6", tex(&self.maps[1])),
                ("nodeUniform6_sampler", sampler(&self.clamp)),
            ];
            let mut groups =
                self.main_render
                    .bind(r, |g| main_pipeline.get_bind_group_layout(g), &resources)?;
            groups.truncate(1);
            groups.extend(
                o.main
                    .bind(r, |g| main_pipeline.get_bind_group_layout(g), &resources)?
                    .into_iter()
                    .skip(1),
            );
            main_groups.push(groups);
        }
        let p0 = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        }];
        let output_pipeline = Spec {
            label: "progressive output",
            shaders: OUTPUT,
            buffers: &[attribute(&p0, 12)],
            format: out.options.format,
            blend: false,
            depth: None,
            cw: false,
            cull: Some(wgpu::Face::Back),
            samples: 1,
        }
        .build(r);
        let output_groups = self.output.bind(
            r,
            |g| output_pipeline.get_bind_group_layout(g),
            &[
                ("nodeUniform0", tex(&scene.view)),
                ("nodeUniform0_sampler", sampler(&self.clamp)),
            ],
        )?;
        let (w, h) = (f64::from(out.width), f64::from(out.height));
        self.output.write(
            r,
            "renderStruct",
            &[
                (
                    "cameraProjectionMatrix",
                    &[
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix", &m4(Matrix4::IDENTITY)),
                ("nodeUniform1", &[w, h]),
            ],
        )?;
        self.targets = Some(Targets {
            width: out.width,
            height: out.height,
            samples,
            format: out.options.format,
            main_pipeline,
            main_groups,
            output_pipeline,
            output_groups,
            scene,
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
        Ok(())
    }
    /// The light shadow's camera: an orthographic box ( ±150, 100 – 5000 )
    /// at the light looking at its target, in WebGPU's clip space.
    fn shadow_camera(position: Vector3, target: Vector3) -> (Matrix4, Matrix4) {
        let view = Matrix4::look_at_rh(position, target, Vector3::Y);
        let (l, r, t, b, n, f) = (-150., 150., 150., -150., 100., 5000.);
        let projection = Matrix4::from_cols_array(&[
            2. / (r - l),
            0.,
            0.,
            0.,
            0.,
            2. / (t - b),
            0.,
            0.,
            0.,
            0.,
            -1. / (f - n),
            0.,
            -(r + l) / (r - l),
            -(t + b) / (t - b),
            -n / (f - n),
            1.,
        ]);
        (projection, view)
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        // A resize redraws the frame without advancing the animation.
        let mut resized = false;
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != samples
        }) {
            self.resize(r, out)?;
            resized = true;
        }
        let advance = std::mem::take(&mut self.pending);
        if !advance && !resized {
            return self.present(r);
        }
        if advance {
            // animate(): controls.update(), the orbit's inertia.
            self.controls.frame_update(s, c)?;
        }
        for g in &mut self.gizmos {
            g.update(s)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let models = [self.object_matrix(s, 0)?, self.object_matrix(s, 1)?];
        let model_world = models[1];
        let object = s.get(self.model)?.position;
        let target = model_world.transform_point3(Vector3::new(0., 20., 0.));
        let [enable, blur_edges, window, radius, ambient, _] = self.params;
        let color = [1., 1., 1.];
        let specular = linear(0x111111);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        if advance && enable > 0.5 {
            // progressiveSurfacemap.update( camera, blendWindow, blurEdges ).
            let mut lightmap_render: Vec<(String, Vec<f64>)> = vec![];
            let ids = [
                (9, 10, 11, 13),
                (20, 21, 22, 23),
                (30, 31, 32, 33),
                (40, 41, 42, 43),
            ];
            for (i, &position) in self.lights.iter().enumerate() {
                let (sp, sv) = Self::shadow_camera(position, self.shadow_target);
                self.shadow_uniforms[i].write(
                    r,
                    "renderStruct",
                    &[
                        ("cameraViewMatrix", &m4(sv)),
                        ("cameraProjectionMatrix", &m4(sp)),
                    ],
                )?;
                // LightShadow's matrix: the [0, 1] bias on x and y only.
                let bias = Matrix4::from_cols_array(&[
                    0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
                ]);
                let (pos, tgt, col, mat) = ids[i];
                let k = mat;
                lightmap_render.extend([
                    (
                        format!("nodeUniform{pos}"),
                        vec![position.x, position.y, position.z],
                    ),
                    (
                        format!("nodeUniform{tgt}"),
                        vec![target.x, target.y, target.z],
                    ),
                    (format!("nodeUniform{col}"), vec![PI / 4.; 3]),
                    (format!("nodeUniform{k}"), m4(bias * sp * sv)),
                    (format!("nodeUniform{}", k + 1), vec![0.]),
                    (format!("nodeUniform{}", k + 2), vec![0.]),
                    (format!("nodeUniform{}", k + 4), vec![1.]),
                    (format!("nodeUniform{}", k + 5), vec![f64::from(SHADOW); 2]),
                    (format!("nodeUniform{}", k + 6), vec![1.]),
                ]);
            }
            lightmap_render.push(("cameraViewMatrix".into(), m4(view)));
            let values: Vec<(&str, &[f64])> = lightmap_render
                .iter()
                .map(|(n, v)| (n.as_str(), &v[..]))
                .collect();
            self.lightmap_render.write(r, "renderStruct", &values)?;
            for (o, model) in self.objects.iter().zip(models) {
                o.shadow.write(
                    r,
                    "objectStruct",
                    &[("nodeUniform0", &[1.]), ("nodeUniform3", &m4(model))],
                )?;
                o.lightmap.write(
                    r,
                    "objectStruct",
                    &[
                        ("nodeUniform0", &color),
                        ("nodeUniform1", &[1.]),
                        ("nodeUniform2", &[30.]),
                        ("nodeUniform3", &specular),
                        ("nodeUniform4", &[0.; 3]),
                        ("nodeUniform5", &[1.]),
                        ("nodeUniform7", &m3(model.inverse().transpose())),
                        ("nodeUniform12", &m4(model)),
                        ("nodeUniform51", &m3(Matrix4::IDENTITY)),
                        ("nodeUniform52", &[window]),
                    ],
                )?;
            }
            for (i, (color, depth)) in self.shadows.iter().enumerate() {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("progressive shadow"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: color,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                pass.set_pipeline(&self.shadow_pipeline);
                for (o, groups) in self.objects.iter().zip(&self.shadow_groups[i]) {
                    for (g, group) in groups.iter().enumerate() {
                        pass.set_bind_group(g as u32, group, &[]);
                    }
                    pass.set_vertex_buffer(0, o.buffers[1].slice(..));
                    pass.set_index_buffer(o.index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..o.count, 0, 0..1);
                }
            }
            // Ping-pong: render into the active map, reading the other.
            let (active, previous) = if self.buffer1_active { (0, 1) } else { (1, 0) };
            self.buffer1_active = !self.buffer1_active;
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("progressive lightmap"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.maps[active],
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.map_depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                if blur_edges > 0.5 {
                    pass.set_pipeline(&self.blur_pipeline);
                    for (g, group) in self.blur_groups[previous].iter().enumerate() {
                        pass.set_bind_group(g as u32, group, &[]);
                    }
                    pass.set_vertex_buffer(0, self.blur_plane.0.slice(..));
                    pass.set_index_buffer(self.blur_plane.1.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..6, 0, 0..1);
                }
                pass.set_pipeline(&self.lightmap_pipeline);
                for (o, groups) in self.objects.iter().zip(&self.lightmap_groups[previous]) {
                    for (g, group) in groups.iter().enumerate() {
                        pass.set_bind_group(g as u32, group, &[]);
                    }
                    pass.set_vertex_buffer(0, o.buffers[0].slice(..));
                    pass.set_vertex_buffer(1, o.buffers[1].slice(..));
                    pass.set_vertex_buffer(2, o.buffers[3].slice(..));
                    pass.set_index_buffer(o.index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..o.count, 0, 0..1);
                }
            }
        }
        // The target's world matrix is current once a render has updated it.
        self.shadow_target = target;
        // The lights move for the next frame: toward the light origin or over
        // the upper hemisphere around the model.
        let origin = s.get(self.origin)?.position;
        for light in self.lights.iter_mut().filter(|_| advance) {
            *light = if self.random.next() > ambient {
                let x = origin.x + self.random.next() * radius;
                let y = origin.y + self.random.next() * radius;
                let z = origin.z + self.random.next() * radius;
                Vector3::new(x, y, z)
            } else {
                // The page's own 3.14159, not π.
                #[allow(clippy::approx_constant)]
                let (half_turn, turn) = (3.14159 / 2., 2. * 3.14159);
                let lambda = (2. * self.random.next() - 1.).acos() - half_turn;
                let phi = turn * self.random.next();
                Vector3::new(
                    lambda.cos() * phi.cos() * 300. + object.x,
                    (lambda.cos() * phi.sin() * 300.).abs() + object.y + 20.,
                    lambda.sin() * 300. + object.z,
                )
            };
        }
        // renderer.render( scene, camera ).
        let t = self
            .targets
            .as_mut()
            .ok_or(Error::Invalid("progressive targets"))?;
        self.main_render.write(
            r,
            "renderStruct",
            &[
                ("cameraViewMatrix", &m4(view)),
                ("cameraProjectionMatrix", &m4(projection)),
                ("nodeUniform9", &linear(0x949494)),
                ("nodeUniform10", &[1000.]),
                ("nodeUniform11", &[3000.]),
            ],
        )?;
        for (o, model) in self.objects.iter().zip(models) {
            o.main.write(
                r,
                "objectStruct",
                &[
                    ("nodeUniform0", &color),
                    ("nodeUniform1", &[1.]),
                    ("nodeUniform2", &[30.]),
                    ("nodeUniform3", &specular),
                    ("nodeUniform4", &[0.; 3]),
                    ("nodeUniform5", &[1.]),
                    ("nodeUniform7", &m3(Matrix4::IDENTITY)),
                    ("nodeUniform8", &[1.]),
                    ("nodeUniform13", &m4(model)),
                ],
            )?;
        }
        let background = linear(0x949494);
        let multisampled = t.scene.options.samples > 1;
        let depth = t
            .scene
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("progressive depth"))?;
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("progressive main"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: if multisampled {
                        &t.scene.multisampled_views[0]
                    } else {
                        &t.scene.view
                    },
                    depth_slice: None,
                    resolve_target: multisampled.then_some(&t.scene.view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background[0],
                            g: background[1],
                            b: background[2],
                            a: 1.,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(&t.main_pipeline);
            for (o, groups) in self.objects.iter().zip(&t.main_groups) {
                for (g, group) in groups.iter().enumerate() {
                    pass.set_bind_group(g as u32, group, &[]);
                }
                pass.set_vertex_buffer(0, o.buffers[3].slice(..));
                pass.set_vertex_buffer(1, o.buffers[1].slice(..));
                pass.set_index_buffer(o.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..o.count, 0, 0..1);
            }
        }
        r.queue.submit([encoder.finish()]);
        // The gizmos over the scene.
        t.scene.set_load_color(true);
        t.scene.options.load_depth = true;
        let gizmos = r.render(s, c, &t.scene);
        t.scene.set_load_color(false);
        t.scene.options.load_depth = false;
        gizmos?;
        self.present(r)
    }
    /// The output pass from the scene target.
    fn present(&self, r: &Renderer) -> Result<bool> {
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("progressive targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("progressive output"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&t.output_pipeline);
            for (g, group) in t.output_groups.iter().enumerate() {
                pass.set_bind_group(g as u32, group, &[]);
            }
            pass.set_vertex_buffer(0, self.fullscreen.slice(..));
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    fn events(&mut self) {
        for g in &mut self.gizmos {
            for e in std::mem::take(&mut g.events) {
                if let Event::DraggingChanged(v) = e {
                    self.orbit_enabled = !v;
                }
            }
        }
    }
    /// The gizmos' pointer listeners, in the order the page adds them.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.queue.push((kind, x, y));
    }
    fn pointer(&mut self, s: &mut Scene, kind: u32, x: f64, y: f64) -> Result<()> {
        let (w, h, _) = viewport_css();
        let ndc = Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.);
        for g in &mut self.gizmos {
            if !g.enabled {
                continue;
            }
            match kind {
                10..=12 => {
                    g.pointer_hover(s, ndc)?;
                    g.pointer_down(s, ndc, kind as i32 - 10)?;
                }
                0 => {
                    g.pointer_hover(s, ndc)?;
                    g.pointer_move(s, ndc)?;
                }
                20..=22 => g.pointer_up(s, kind as i32 - 20)?,
                _ => {}
            }
        }
        self.events();
        Ok(())
    }
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
        // The gizmos' listeners see the pointer before the orbit controls.
        for (kind, x, y) in std::mem::take(&mut self.queue) {
            self.pointer(s, kind, x, y)?;
        }
        if !self.orbit_enabled {
            return Ok(());
        }
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        // The pointer handlers call controls.update().
        self.controls.update(s, c)
    }
    /// Enable, Blur Edges, Blend Window, Light Radius, Ambient Weight and
    /// Debug Lightmap ( not reproduced ).
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("progressive parameter"))? = f64::from(value);
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
