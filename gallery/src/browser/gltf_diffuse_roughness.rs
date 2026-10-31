//! webgpu_loader_gltf_diffuse_roughness: DiffuseRoughnessParameterSweep.glb's
//! 24 spheres under RoomEnvironment's PMREM ( fromScene, sigma 0.04, as the
//! background with blurriness 0.5 and the environment ), Neutral tone mapped.
//! GLTFLoader's port ( `crate::gltf` ) reads the materials: the four
//! without KHR_materials_diffuse_roughness are MeshStandardMaterials, the
//! others MeshPhysicalMaterials whose diffuseRoughness above 0 switches the
//! diffuse lobe to r187's BRDF_EON ( its directional albedo for the
//! environment's irradiance ). The 24 meshes share GLTFLoader's one cached
//! geometry, whose triangle winding the page reverses once at load. Every
//! stage runs the WGSL three.js r187 generates for the page: the room, the
//! PMREM and the background ( `pmrem/`, `compute_cloth/` ), the standard
//! spheres and the output pass ( the compute_cloth modules, byte-identical )
//! and the two physical variants ( `gltf_diffuse_roughness/` ).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, color, depth_attachment, sampled_pipeline, set};
use super::gltf_viewer::load_asset;
use super::lights_projector::{m3, m4, pack};
use super::pmrem::{Pmrem, room_environment};
use super::pmrem_cube_uv::{bind, raw_pipeline};
use super::retro::{target, uniform};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const BACKGROUND_VS: &str = include_str!("compute_cloth/background_vs.wgsl");
const BACKGROUND_FS: &str = include_str!("compute_cloth/background_fs.wgsl");
const OUTPUT_VS: &str = include_str!("compute_cloth/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("compute_cloth/output_fs.wgsl");
/// MeshStandardNodeMaterial, MeshPhysicalNodeMaterial without and with
/// diffuseRoughness.
const PROGRAMS: [(&str, &str); 3] = [
    (
        include_str!("compute_cloth/sphere_vs.wgsl"),
        include_str!("compute_cloth/sphere_fs.wgsl"),
    ),
    (
        include_str!("gltf_diffuse_roughness/physical_vs.wgsl"),
        include_str!("gltf_diffuse_roughness/physical_fs.wgsl"),
    ),
    (
        include_str!("gltf_diffuse_roughness/rough_vs.wgsl"),
        include_str!("gltf_diffuse_roughness/rough_fs.wgsl"),
    ),
];
const MODEL: &str = "assets/gltf/DiffuseRoughnessParameterSweep.glb";
const X3: wgpu::VertexFormat = wgpu::VertexFormat::Float32x3;

/// A geometry GLTFLoader caches for its primitives ( normal, position,
/// index and its count ), with its bounding sphere.
struct Geometry {
    normal: wgpu::Buffer,
    position: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    bounds: Sphere,
}
/// A glTF mesh: its geometry, world matrix, material program and object
/// uniforms ( written once: the meshes and their materials do not change ).
struct Item {
    geometry: usize,
    matrix: Matrix4,
    program: usize,
    object: wgpu::Buffer,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    background: Draw,
    /// Per item.
    items: Vec<Draw>,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    /// A frame was requested ( animate() runs: the damped controls update ).
    pending: bool,
    geometries: Vec<Geometry>,
    items: Vec<Item>,
    background: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    output_quad: wgpu::Buffer,
    pmrem: Pmrem,
    clamp: wgpu::Sampler,
    /// The PMREM's trilinear sampler.
    linear: wgpu::Sampler,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    /// The lit programs' camera uniforms.
    renders: [wgpu::Buffer; 3],
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    targets: Option<Targets>,
}
fn floats(g: &BufferGeometry, name: &str) -> Result<Vec<f32>> {
    let a = g
        .attributes
        .get(name)
        .ok_or(Error::Invalid("diffuse roughness attribute"))?;
    (0..a.count())
        .flat_map(|i| (0..3).map(move |k| (i, k)))
        .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
        .collect()
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 35.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 10.);
        n.quaternion = Quaternion::IDENTITY;
        // OrbitControls with enableDamping.
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        let sampler = |mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        let linear = sampler(wgpu::FilterMode::Linear);
        let clamp = sampler(wgpu::FilterMode::Nearest);
        // pmremGenerator.fromScene( new RoomEnvironment(), 0.04 ).
        let pmrem = room_environment(r, 0.04)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        // GLTFLoader.loadAsync( 'DiffuseRoughnessParameterSweep.glb' ).
        let (asset, buffers, images) = load_asset(MODEL).await?;
        let imported = crate::gltf::import_decoded(&asset, &buffers, &images)?;
        let sources = imported.sources.clone();
        let mut model = Scene::new();
        let meshes = imported.instantiate(&mut model)?;
        model.update()?;
        // GLTFLoader caches a primitive's geometry by its attribute and index
        // accessors: the spheres share one.
        let key = |(mesh, primitive): (usize, usize)| -> Result<Vec<usize>> {
            let p = asset
                .meshes()
                .nth(mesh)
                .and_then(|m| m.primitives().nth(primitive))
                .ok_or(Error::Invalid("diffuse roughness primitive"))?;
            let mut key: Vec<usize> = p.attributes().map(|(_, a)| a.index()).collect();
            key.sort_unstable();
            key.push(p.indices().map_or(usize::MAX, |a| a.index()));
            Ok(key)
        };
        let mut keys: Vec<Vec<usize>> = vec![];
        let mut geometries = vec![];
        let mut items = vec![];
        for (handle, source) in meshes.into_iter().zip(sources) {
            let node = model.get(handle)?;
            let NodeKind::Mesh(mesh) = &node.kind else {
                return Err(Error::Invalid("diffuse roughness mesh"));
            };
            let k = key(source)?;
            let geometry = match keys.iter().position(|other| *other == k) {
                Some(i) => i,
                None => {
                    let g = &mesh.geometry;
                    // The page reverses each geometry's triangle winding once
                    // ( the draft asset winds clockwise ).
                    let mut index = g
                        .index
                        .clone()
                        .ok_or(Error::Invalid("diffuse roughness index"))?;
                    for t in index.as_chunks_mut::<3>().0 {
                        t.swap(1, 2);
                    }
                    geometries.push(Geometry {
                        normal: init(
                            "diffuse roughness normal",
                            bytemuck::cast_slice(&floats(g, "normal")?),
                            vertex,
                        ),
                        position: init(
                            "diffuse roughness position",
                            bytemuck::cast_slice(&floats(g, "position")?),
                            vertex,
                        ),
                        index: init(
                            "diffuse roughness index",
                            bytemuck::cast_slice(&index),
                            wgpu::BufferUsages::INDEX,
                        ),
                        count: index.len() as u32,
                        bounds: g
                            .bounding_sphere
                            .ok_or(Error::Invalid("diffuse roughness bounds"))?,
                    });
                    keys.push(k);
                    geometries.len() - 1
                }
            };
            let matrix = node.matrix_world;
            let normal_matrix =
                Matrix4::from_mat3(Matrix3::from_mat4(matrix).inverse().transpose());
            let identity = m4(Matrix4::IDENTITY);
            let max_lod = vec![pmrem.max_lod as f64];
            let (program, values): (usize, Vec<(&str, Vec<f64>)>) = match &*mesh.materials[0] {
                Material::Standard(m) => (
                    0,
                    vec![
                        ("nodeUniform0", m.properties.color.0.to_array().to_vec()),
                        ("nodeUniform1", vec![m.properties.opacity]),
                        ("nodeUniform2", vec![m.metalness]),
                        ("nodeUniform3", vec![m.roughness]),
                        ("nodeUniform5", m3(normal_matrix)),
                        ("nodeUniform6", m.emissive.0.to_array().to_vec()),
                        ("nodeUniform7", vec![1.]),
                        ("nodeUniform9", m4(matrix)),
                        // PMREMNode: rotation, maxLod and environmentIntensity.
                        ("nodeUniform11", identity),
                        ("nodeUniform13", max_lod),
                        ("nodeUniform14", vec![1.]),
                    ],
                ),
                Material::Physical(p) => {
                    let m = &p.base;
                    let mut values = vec![
                        ("nodeUniform0", m.properties.color.0.to_array().to_vec()),
                        ("nodeUniform1", vec![m.properties.opacity]),
                        ("nodeUniform2", vec![m.metalness]),
                        ("nodeUniform3", vec![m.roughness]),
                        ("nodeUniform5", m3(normal_matrix)),
                        ("nodeUniform6", vec![p.ior]),
                        ("nodeUniform7", p.specular_color.0.to_array().to_vec()),
                        ("nodeUniform8", vec![p.specular_intensity]),
                    ];
                    // useDiffuseRoughness: diffuseRoughness > 0.
                    let rough = p.diffuse_roughness > 0.;
                    let u = |i: usize| -> &'static str {
                        [
                            "nodeUniform9",
                            "nodeUniform10",
                            "nodeUniform11",
                            "nodeUniform12",
                            "nodeUniform13",
                            "nodeUniform14",
                            "nodeUniform15",
                            "nodeUniform16",
                            "nodeUniform17",
                            "nodeUniform18",
                        ][i - 9 + usize::from(rough)]
                    };
                    if rough {
                        values.push(("nodeUniform9", vec![p.diffuse_roughness]));
                    }
                    values.extend([
                        (u(9), m.emissive.0.to_array().to_vec()),
                        (u(10), vec![1.]),
                        (u(12), m4(matrix)),
                        (u(14), identity),
                        (u(16), max_lod),
                        (u(17), vec![1.]),
                    ]);
                    (1 + usize::from(rough), values)
                }
                _ => return Err(Error::Invalid("diffuse roughness material")),
            };
            let source = PROGRAMS[program].1;
            let object = uniform(r, "diffuse roughness object", source, "objectStruct")?;
            let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            r.queue
                .write_buffer(&object, 0, &pack(source, "objectStruct", &values)?);
            items.push(Item {
                geometry,
                matrix,
                program,
                object,
            });
        }
        // The background mesh: SphereGeometry( 1, 32, 32 ).
        let ball = SphereGeometry::build(1., 32, 32)?;
        let ball_index = ball
            .index
            .clone()
            .ok_or(Error::Invalid("background index"))?;
        let background_object = uniform(
            r,
            "diffuse roughness background",
            BACKGROUND_FS,
            "objectStruct",
        )?;
        r.queue.write_buffer(
            &background_object,
            0,
            &pack(
                BACKGROUND_FS,
                "objectStruct",
                &[
                    ("nodeUniform1", &m4(Matrix4::IDENTITY)),
                    ("nodeUniform4", &m3(Matrix4::IDENTITY)),
                    ("nodeUniform5", &[pmrem.max_lod as f64]),
                    ("nodeUniform8", &[1.]),
                    ("nodeUniform10", &m4(Matrix4::IDENTITY)),
                ],
            )?,
        );
        Ok(Self {
            controls,
            pending: true,
            geometries,
            items,
            background: (
                init(
                    "diffuse roughness background",
                    bytemuck::cast_slice(&floats(&ball, "normal")?),
                    vertex,
                ),
                init(
                    "diffuse roughness background",
                    bytemuck::cast_slice(&floats(&ball, "position")?),
                    vertex,
                ),
                init(
                    "diffuse roughness background",
                    bytemuck::cast_slice(&ball_index),
                    wgpu::BufferUsages::INDEX,
                ),
                ball_index.len() as u32,
            ),
            output_quad: init(
                "diffuse roughness quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            pmrem,
            clamp,
            linear,
            background_render: uniform(
                r,
                "diffuse roughness background",
                BACKGROUND_FS,
                "renderStruct",
            )?,
            background_object,
            renders: [
                uniform(
                    r,
                    "diffuse roughness standard",
                    PROGRAMS[0].1,
                    "renderStruct",
                )?,
                uniform(
                    r,
                    "diffuse roughness physical",
                    PROGRAMS[1].1,
                    "renderStruct",
                )?,
                uniform(r, "diffuse roughness rough", PROGRAMS[2].1, "renderStruct")?,
            ],
            output_render: uniform(r, "diffuse roughness output", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "diffuse roughness output", OUTPUT_VS, "objectStruct")?,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let triangles = (samples, wgpu::PrimitiveTopology::TriangleList);
        let normal = [wgpu::VertexAttribute {
            format: X3,
            offset: 0,
            shader_location: 0,
        }];
        let position = [wgpu::VertexAttribute {
            format: X3,
            offset: 0,
            shader_location: 1,
        }];
        let mesh_layouts = [
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &normal,
            },
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &position,
            },
        ];
        let groups = |p: &wgpu::RenderPipeline,
                      render: &wgpu::Buffer,
                      object: Vec<(u32, wgpu::BindingResource)>| {
            vec![
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                ),
                bind(r, p.get_bind_group_layout(1), &object),
            ]
        };
        let background = sampled_pipeline(
            r,
            "diffuse roughness background",
            (BACKGROUND_VS, BACKGROUND_FS),
            &mesh_layouts,
            &[HALF],
            Some((wgpu::CompareFunction::Always, false)),
            (true, false),
            triangles,
        );
        let background_groups = groups(
            &background,
            &self.background_render,
            vec![
                (0, sampler(&self.linear)),
                (1, tex(&self.pmrem.view)),
                (2, self.background_object.as_entire_binding()),
            ],
        );
        let pipelines = PROGRAMS.map(|(vs, fs)| {
            sampled_pipeline(
                r,
                "diffuse roughness spheres",
                (vs, fs),
                &mesh_layouts,
                &[HALF],
                Some((wgpu::CompareFunction::LessEqual, true)),
                (false, false),
                triangles,
            )
        });
        // Each mesh's object, the DFG LUT and the PMREM.
        let items = self
            .items
            .iter()
            .map(|item| {
                let p = &pipelines[item.program];
                (
                    p.clone(),
                    groups(
                        p,
                        &self.renders[item.program],
                        vec![
                            (0, item.object.as_entire_binding()),
                            (1, sampler(&self.clamp)),
                            (2, tex(&r.dfg)),
                            (3, sampler(&self.linear)),
                            (4, tex(&self.pmrem.view)),
                        ],
                    ),
                )
            })
            .collect();
        let color_target = target(r, size, HALF, 1);
        let output = raw_pipeline(
            r,
            "diffuse roughness output",
            OUTPUT_VS,
            OUTPUT_FS,
            &mesh_layouts[..1],
            out.options.format,
            1,
            None,
            false,
        );
        let output_groups = vec![
            bind(
                r,
                output.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            bind(
                r,
                output.get_bind_group_layout(1),
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&color_target)),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            msaa: (samples > 1).then(|| {
                (
                    target(r, size, HALF, samples),
                    target(r, size, DEPTH, samples),
                )
            }),
            color: color_target,
            depth: target(r, size, DEPTH, 1),
            background: (background, background_groups),
            items,
            output: (output, output_groups),
            screen: RenderTarget::with_options(
                &r.device,
                size.0,
                size.1,
                RenderTargetOptions {
                    samples: 0,
                    depth_buffer: false,
                    ..out.options.clone()
                },
            )?,
        });
        Ok(())
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("diffuse roughness output"),
            color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
            ..Default::default()
        });
        set(&mut pass, &t.output);
        pass.set_vertex_buffer(0, self.output_quad.slice(..));
        pass.draw(0..3, 0..1);
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != if out.options.samples > 1 { 4 } else { 1 }
        }) {
            self.resize(r, out)?;
            self.pending = true;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("diffuse roughness targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate(): controls.update(), then renderer.render( scene, camera ).
        self.controls.frame_update(s, c)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let write = |buffer: &wgpu::Buffer,
                     source: &str,
                     name: &str,
                     values: &[(&str, Vec<f64>)]|
         -> Result<()> {
            let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            r.queue
                .write_buffer(buffer, 0, &pack(source, name, &values)?);
            Ok(())
        };
        let camera_values = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
        ];
        // backgroundBlurriness 0.5, backgroundIntensity and backgroundRotation.
        let mut values = camera_values.clone();
        values.extend([
            ("nodeUniform6", vec![0.5]),
            ("nodeUniform7", vec![1.]),
            ("nodeUniform2", m4(Matrix4::IDENTITY)),
        ]);
        write(
            &self.background_render,
            BACKGROUND_FS,
            "renderStruct",
            &values,
        )?;
        let mut lit = camera_values;
        lit.push(("cameraWorldMatrix", m4(world)));
        for (buffer, (_, fs)) in self.renders.iter().zip(PROGRAMS) {
            write(buffer, fs, "renderStruct", &lit)?;
        }
        write(
            &self.output_render,
            OUTPUT_FS,
            "renderStruct",
            &[
                (
                    "cameraProjectionMatrix",
                    vec![
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix", m4(Matrix4::IDENTITY)),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
                ("nodeUniform2", vec![1.]),
            ],
        )?;
        write(
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        // The opaque list: the meshes the frustum intersects, front to back
        // ( painterSortStable: z, then id ).
        let screen = projection * view;
        let frustum = Frustum::from_projection(screen);
        let mut opaque = vec![];
        for (i, item) in self.items.iter().enumerate() {
            let bounds = self.geometries[item.geometry].bounds;
            let center = item.matrix.transform_point3(bounds.center);
            let scale = item
                .matrix
                .x_axis
                .truncate()
                .length()
                .max(item.matrix.y_axis.truncate().length())
                .max(item.matrix.z_axis.truncate().length());
            if frustum.intersects_sphere(Sphere {
                center,
                radius: bounds.radius * scale,
            }) {
                opaque.push((screen.project_point3(item.matrix.w_axis.truncate()).z, i));
            }
        }
        opaque.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("diffuse roughness scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: t.msaa.as_ref().map_or(&t.color, |m| &m.0),
                    depth_slice: None,
                    resolve_target: t.msaa.as_ref().map(|_| &t.color),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: depth_attachment(
                    t.msaa.as_ref().map_or(&t.depth, |m| &m.1),
                    wgpu::LoadOp::Clear(1.),
                ),
                ..Default::default()
            });
            set(&mut pass, &t.background);
            pass.set_vertex_buffer(0, self.background.0.slice(..));
            pass.set_vertex_buffer(1, self.background.1.slice(..));
            pass.set_index_buffer(self.background.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.background.3, 0, 0..1);
            for (_, i) in opaque {
                let g = &self.geometries[self.items[i].geometry];
                set(&mut pass, &t.items[i]);
                pass.set_vertex_buffer(0, g.normal.slice(..));
                pass.set_vertex_buffer(1, g.position.slice(..));
                pass.set_index_buffer(g.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..g.count, 0, 0..1);
            }
        }
        self.present(&mut encoder, t);
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// The target `output` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_mut(&mut self) -> Option<&mut RenderTarget> {
        self.targets.as_mut().map(|t| &mut t.screen)
    }
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
        // OrbitControls' pointer handlers call update().
        self.controls.update(s, c)
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Option<crate::Result<bool>> {
        Some(self.render(renderer, scene, camera, target))
    }
    fn on_output_target(&self) -> Option<Option<&crate::renderer::RenderTarget>> {
        if let Some(target) = self.output() {
            return Some(Some(target));
        }
        None
    }
    fn on_output_target_mut(&mut self) -> Option<Option<&mut crate::renderer::RenderTarget>> {
        Some(self.output_mut())
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}
/// The program of three.js examples/webgpu_loader_gltf_diffuse_roughness.html: its renderer,
/// its scene and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_loader_gltf_diffuse_roughness(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            // WebGPURenderer( { antialias: true } ): the scene's 4× MSAA
            // targets follow the main target's samples.
            samples: 4,
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                100.,
                Box::new(Demo::create(scene, camera, 501, renderer).await?),
            ))
        },
    )
    .await
}
