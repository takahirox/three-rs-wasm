//! webgpu_sculpt: the Sculptor addon's dynamic-topology sculpting of an
//! IcosahedronGeometry( 1, 50 ) sphere in clay MeshStandardMaterial under
//! two directional lights and an ambient light, with damped OrbitControls
//! and the brush cursor ( a ring and a dot, drawn over the mesh ). Strokes
//! run on the CPU as the addon's do ( the welded mesh, its octree, the
//! subdivision and decimation passes and the nine tools, in `sculpt/
//! sculptor.rs`, matching the addon's output bit for bit ), and the changed
//! vertices reach the GPU as the addon's attribute update ranges: partial
//! position and normal writes, the index rewritten when the topology
//! changes, and new buffers only when the addon reallocates its arrays.
//! Every stage runs the WGSL three.js r186 generates for the page ( in
//! `sculpt/`; the output is the compute_rasterizer module's, byte-identical ).
mod sculptor;
use super::controls_attributes::{Controls, camera_state, viewport_css};
use super::deferred::{Draw, culled_pipeline, set};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::uniform;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use sculptor::{Sculptor, TOOLS, View, icosahedron, invert, update_ranges};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("sculpt/", $name, ".wgsl"))
    };
}
const OUTPUT_VS: &str = include_str!("compute_rasterizer/8.wgsl");
const OUTPUT_FS: &str = include_str!("compute_rasterizer/9.wgsl");
const LOCATION0: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
const LOCATION1: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];
/// The cursor dot's radius in pixels of the brush size.
const CURSOR_DOT_RADIUS: f64 = 2.5;
/// The resident sculpt geometry: buffers sized to the addon's array
/// capacities.
struct Gpu {
    position: wgpu::Buffer,
    normal: wgpu::Buffer,
    index: wgpu::Buffer,
    vertex_capacity: usize,
    face_capacity: usize,
    version: u64,
}
/// The cursor ( ring and dot ) and its last placement.
struct Cursor {
    ring: (wgpu::Buffer, wgpu::Buffer, u32),
    dot: (wgpu::Buffer, wgpu::Buffer, u32),
    visible: bool,
    ring_visible: bool,
    color: u32,
    position: Vector3,
    quaternion: Quaternion,
    scale: f64,
    dot_scale: f64,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    mesh: Draw,
    ring: Draw,
    dot: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    sculptor: Sculptor,
    gpu: Gpu,
    cursor: Cursor,
    /// The last pointer position ( _pointer ) and the pressed buttons.
    pointer: Vector2,
    buttons: u32,
    orbit: Option<(Vector2, bool)>,
    /// Pointer events since the last frame, handled before it renders.
    events: Vec<(u32, f64, f64)>,
    /// The camera placement at the controls' last change event.
    last_camera: Option<(Vector3, Quaternion)>,
    mesh_render: wgpu::Buffer,
    mesh_object: wgpu::Buffer,
    cursor_render: wgpu::Buffer,
    ring_object: wgpu::Buffer,
    dot_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    linear: wgpu::Sampler,
    targets: Option<Targets>,
}
fn write(
    r: &Renderer,
    buffer: &wgpu::Buffer,
    source: &str,
    name: &str,
    values: &[(&str, Vec<f64>)],
) -> Result<()> {
    let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
    r.queue
        .write_buffer(buffer, 0, &pack(source, name, &values)?);
    Ok(())
}
fn buffer(r: &Renderer, label: &str, contents: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents,
            usage: usage | wgpu::BufferUsages::COPY_DST,
        })
}
/// A cursor geometry's positions and index.
fn upload_shape(r: &Renderer, g: &BufferGeometry) -> Result<(wgpu::Buffer, wgpu::Buffer, u32)> {
    let a = g
        .attributes
        .get("position")
        .ok_or(Error::Invalid("cursor positions"))?;
    let positions: Vec<f32> = (0..a.count())
        .flat_map(|i| (0..3).map(move |k| (i, k)))
        .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
        .collect::<Result<_>>()?;
    let index = g.index.clone().ok_or(Error::Invalid("cursor index"))?;
    Ok((
        buffer(
            r,
            "cursor position",
            bytemuck::cast_slice(&positions),
            wgpu::BufferUsages::VERTEX,
        ),
        buffer(
            r,
            "cursor index",
            bytemuck::cast_slice(&index),
            wgpu::BufferUsages::INDEX,
        ),
        index.len() as u32,
    ))
}
/// Quaternion.setFromUnitVectors( from, to ).
fn from_unit_vectors(from: Vector3, to: Vector3) -> Quaternion {
    let r = from.dot(to) + 1.;
    let (x, y, z, w) = if r < 1e-8 {
        if from.x.abs() > from.z.abs() {
            (-from.y, from.x, 0., 0.)
        } else {
            (0., -from.z, from.y, 0.)
        }
    } else {
        (
            from.y * to.z - from.z * to.y,
            from.z * to.x - from.x * to.z,
            from.x * to.y - from.y * to.x,
            r,
        )
    };
    Quaternion::from_xyzw(x, y, z, w).normalize()
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 4.);
        let mut controls = Controls::new(Some(0.1), (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        let sculptor = Sculptor::new(&icosahedron(1., 50));
        let m = &sculptor.mesh;
        let gpu = Gpu {
            position: buffer(
                r,
                "sculpt position",
                bytemuck::cast_slice(&m.vertices),
                wgpu::BufferUsages::VERTEX,
            ),
            normal: buffer(
                r,
                "sculpt normal",
                bytemuck::cast_slice(&m.render_normals),
                wgpu::BufferUsages::VERTEX,
            ),
            index: buffer(
                r,
                "sculpt index",
                bytemuck::cast_slice(&m.triangles),
                wgpu::BufferUsages::INDEX,
            ),
            vertex_capacity: m.vertices.len() / 3,
            face_capacity: m.triangles.len() / 3,
            version: m.topology_version(),
        };
        let cursor = Cursor {
            ring: upload_shape(r, &RingGeometry::build(0.95, 1., 48, 1, 0., PI * 2.)?)?,
            dot: upload_shape(r, &CircleGeometry::build(1., 16, 0., PI * 2.)?)?,
            visible: false,
            ring_visible: false,
            color: 0xcc0000,
            position: Vector3::ZERO,
            quaternion: Quaternion::IDENTITY,
            scale: 1.,
            dot_scale: 1.,
        };
        Ok(Self {
            controls,
            sculptor,
            gpu,
            cursor,
            pointer: Vector2::ZERO,
            buttons: 0,
            orbit: None,
            events: vec![],
            last_camera: None,
            mesh_render: uniform(r, "sculpt", wgsl!("mesh_fs"), "renderStruct")?,
            mesh_object: uniform(r, "sculpt", wgsl!("mesh_fs"), "objectStruct")?,
            cursor_render: uniform(r, "cursor", wgsl!("cursor_fs"), "renderStruct")?,
            ring_object: uniform(r, "cursor ring", wgsl!("cursor_fs"), "objectStruct")?,
            dot_object: uniform(r, "cursor dot", wgsl!("cursor_fs"), "objectStruct")?,
            output_render: uniform(r, "sculpt output", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "sculpt output", OUTPUT_VS, "objectStruct")?,
            output_quad: buffer(
                r,
                "sculpt quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            linear: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            targets: None,
        })
    }
    /// The stroke's camera ( three.js Matrix4 inverses ) and canvas.
    fn view(s: &Scene, c: Object3D) -> Result<View> {
        let (camera, world) = s.camera(c)?;
        let projection = camera.projection_matrix()?.to_cols_array();
        let world = world.to_cols_array();
        let (width, height, _) = viewport_css();
        Ok(View {
            projection,
            projection_inverse: invert(&projection),
            world,
            world_inverse: invert(&world),
            rect: [0., 0., width, height],
        })
    }
    /// _syncGeometry(): new buffers when the addon's arrays reallocated,
    /// otherwise the dirty vertices' update ranges and, after a topology
    /// change, the index.
    fn sync(&mut self, r: &Renderer) {
        let m = &self.sculptor.mesh;
        let (vertex_capacity, face_capacity) = (m.vertices.len() / 3, m.triangles.len() / 3);
        if vertex_capacity != self.gpu.vertex_capacity || face_capacity != self.gpu.face_capacity {
            self.gpu = Gpu {
                position: buffer(
                    r,
                    "sculpt position",
                    bytemuck::cast_slice(&m.vertices),
                    wgpu::BufferUsages::VERTEX,
                ),
                normal: buffer(
                    r,
                    "sculpt normal",
                    bytemuck::cast_slice(&m.render_normals),
                    wgpu::BufferUsages::VERTEX,
                ),
                index: buffer(
                    r,
                    "sculpt index",
                    bytemuck::cast_slice(&m.triangles),
                    wgpu::BufferUsages::INDEX,
                ),
                vertex_capacity,
                face_capacity,
                version: m.topology_version(),
            };
            self.sculptor.dirty.clear();
            return;
        }
        let mut dirty = std::mem::take(&mut self.sculptor.dirty);
        dirty.sort_unstable();
        dirty.dedup();
        dirty.retain(|&v| (v as usize) < m.nb_vertices());
        if !dirty.is_empty() {
            for (start, count) in update_ranges(&dirty) {
                let bytes =
                    |a: &[f32]| bytemuck::cast_slice::<f32, u8>(&a[start..start + count]).to_vec();
                r.queue
                    .write_buffer(&self.gpu.position, start as u64 * 4, &bytes(&m.vertices));
                r.queue.write_buffer(
                    &self.gpu.normal,
                    start as u64 * 4,
                    &bytes(&m.render_normals),
                );
            }
        }
        if m.topology_version() != self.gpu.version {
            let length = m.nb_triangles() * 3;
            r.queue.write_buffer(
                &self.gpu.index,
                0,
                bytemuck::cast_slice(&m.triangles[..length]),
            );
            self.gpu.version = m.topology_version();
        }
    }
    /// updateCursorPosition(): the hit point and normal while hovering or
    /// sculpting, otherwise the pointer unprojected at mid depth.
    fn update_cursor(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let view = Self::view(s, c)?;
        let hovering = !self.sculptor.sculpting;
        if hovering {
            self.sculptor.pick(&view, self.pointer.x, self.pointer.y);
        }
        let hit = self.sculptor.has_hit();
        if hit {
            let h = self.sculptor.hit_point;
            self.cursor.position = Vector3::new(h[0], h[1], h[2]);
            let n = self.sculptor.hit_normal;
            let normal = Vector3::new(n[0], n[1], n[2]);
            let length = normal.length();
            let normal = normal * (1. / if length == 0. { 1. } else { length });
            self.cursor.quaternion = from_unit_vectors(Vector3::Z, normal);
            self.cursor.scale = self.sculptor.world_radius2.sqrt();
        } else {
            let r = view.rect;
            let x = ((self.pointer.x - r[0]) / r[2]) * 2. - 1.;
            let offset_x = ((self.pointer.x + self.sculptor.size - r[0]) / r[2]) * 2. - 1.;
            let y = -((self.pointer.y - r[1]) / r[3]) * 2. + 1.;
            let unproject = |x: f64| {
                Matrix4::from_cols_array(&view.world).project_point3(
                    Matrix4::from_cols_array(&view.projection_inverse)
                        .project_point3(Vector3::new(x, y, 0.5)),
                )
            };
            let (mouse, offset) = (unproject(x), unproject(offset_x));
            self.cursor.position = mouse;
            self.cursor.quaternion = s.get(c)?.quaternion;
            self.cursor.scale = mouse.distance(offset);
        }
        self.cursor.dot_scale = CURSOR_DOT_RADIUS / self.sculptor.size;
        self.cursor.color = if hit && hovering { 0xcc0000 } else { 0xcc6600 };
        self.cursor.ring_visible = hovering;
        self.cursor.visible = true;
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let target = |format, samples| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("sculpt target"),
                    size: wgpu::Extent3d {
                        width: size.0,
                        height: size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let color = target(HALF, samples);
        let resolve = (samples > 1).then(|| target(HALF, 1));
        let depth = target(DEPTH, samples);
        let layout = |attributes: &'static [wgpu::VertexAttribute]| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes,
        };
        let (v3, v3b): (
            &'static [wgpu::VertexAttribute],
            &'static [wgpu::VertexAttribute],
        ) = (&LOCATION0, &LOCATION1);
        let triangles = (samples, wgpu::PrimitiveTopology::TriangleList);
        let mesh_pipeline = culled_pipeline(
            r,
            "sculpt mesh",
            (wgsl!("mesh_vs"), wgsl!("mesh_fs")),
            &[layout(v3), layout(v3b)],
            &[HALF],
            Some((wgpu::CompareFunction::LessEqual, true)),
            (false, false),
            triangles,
            Some(wgpu::Face::Back),
        );
        let mesh = (
            mesh_pipeline.clone(),
            vec![
                bind(
                    r,
                    mesh_pipeline.get_bind_group_layout(0),
                    &[(0, self.mesh_render.as_entire_binding())],
                ),
                bind(
                    r,
                    mesh_pipeline.get_bind_group_layout(1),
                    &[
                        (0, self.mesh_object.as_entire_binding()),
                        (1, wgpu::BindingResource::Sampler(&self.linear)),
                        (2, wgpu::BindingResource::TextureView(&r.dfg)),
                    ],
                ),
            ],
        );
        let cursor_pipeline = culled_pipeline(
            r,
            "sculpt cursor",
            (wgsl!("cursor_vs"), wgsl!("cursor_fs")),
            &[layout(v3)],
            &[HALF],
            Some((wgpu::CompareFunction::Always, false)),
            (false, false),
            triangles,
            None,
        );
        let cursor = |object: &wgpu::Buffer| {
            (
                cursor_pipeline.clone(),
                vec![
                    bind(
                        r,
                        cursor_pipeline.get_bind_group_layout(0),
                        &[(0, self.cursor_render.as_entire_binding())],
                    ),
                    bind(
                        r,
                        cursor_pipeline.get_bind_group_layout(1),
                        &[(0, object.as_entire_binding())],
                    ),
                ],
            )
        };
        let (ring, dot) = (cursor(&self.ring_object), cursor(&self.dot_object));
        let output_pipeline = culled_pipeline(
            r,
            "sculpt output",
            (OUTPUT_VS, OUTPUT_FS),
            &[layout(v3)],
            &[out.options.format],
            None,
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
            Some(wgpu::Face::Back),
        );
        let output = (
            output_pipeline.clone(),
            vec![
                bind(
                    r,
                    output_pipeline.get_bind_group_layout(0),
                    &[(0, self.output_render.as_entire_binding())],
                ),
                bind(
                    r,
                    output_pipeline.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::Sampler(&self.linear)),
                        (
                            1,
                            wgpu::BindingResource::TextureView(resolve.as_ref().unwrap_or(&color)),
                        ),
                        (2, self.output_object.as_entire_binding()),
                    ],
                ),
            ],
        );
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            color,
            resolve,
            depth,
            mesh,
            ring,
            dot,
            output,
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
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
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
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != samples
        }) {
            self.resize(r, out)?;
        }
        for (kind, x, y) in std::mem::take(&mut self.events) {
            self.draw_pointer(s, c, kind, x, y)?;
        }
        // animate(): controls.update(); a camera change re-places a
        // visible hover cursor ( the controls' change listener ).
        self.controls.update(s, c)?;
        s.update()?;
        // OrbitControls dispatches change only past its EPS from the last
        // dispatched placement.
        let (position, quaternion) = s.get(c).map(|n| (n.position, n.quaternion))?;
        let (last_position, last_quaternion) =
            *self.last_camera.get_or_insert((position, quaternion));
        let moved = last_position.distance_squared(position) > 1e-6
            || 8. * (1. - last_quaternion.dot(quaternion)) > 1e-6;
        if moved {
            self.last_camera = Some((position, quaternion));
        }
        if moved && self.cursor.visible && !self.sculptor.sculpting {
            self.update_cursor(s, c)?;
        }
        self.sync(r);
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("sculpt targets"))?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let id4 = m4(Matrix4::IDENTITY);
        let ambient = Color::from_hex(0x404040).0.to_array().to_vec();
        write(
            r,
            &self.mesh_render,
            wgsl!("mesh_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform10", ambient.clone()),
                ("nodeUniform14", vec![2.; 3]),
                ("nodeUniform12", vec![1., 1.5, 2.]),
                ("nodeUniform13", vec![0.; 3]),
                (
                    "nodeUniform17",
                    (Color::from_hex(0x88aaff).0 * 0.5).to_array().to_vec(),
                ),
                ("nodeUniform15", vec![-1., -0.5, -1.]),
                ("nodeUniform16", vec![0.; 3]),
            ],
        )?;
        write(
            r,
            &self.mesh_object,
            wgsl!("mesh_fs"),
            "objectStruct",
            &[
                (
                    "nodeUniform0",
                    Color::from_hex(0xcc6644).0.to_array().to_vec(),
                ),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.1]),
                ("nodeUniform3", vec![0.4]),
                ("nodeUniform5", m3(Matrix4::IDENTITY)),
                ("nodeUniform6", vec![0.; 3]),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", id4.clone()),
            ],
        )?;
        write(
            r,
            &self.cursor_render,
            wgsl!("cursor_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform2", ambient),
            ],
        )?;
        let cursor_color = Color::from_hex(self.cursor.color).0.to_array().to_vec();
        let group = Matrix4::from_scale_rotation_translation(
            Vector3::splat(self.cursor.scale),
            self.cursor.quaternion,
            self.cursor.position,
        );
        for (object, model) in [
            (&self.ring_object, group),
            (
                &self.dot_object,
                group * Matrix4::from_scale(Vector3::splat(self.cursor.dot_scale)),
            ),
        ] {
            write(
                r,
                object,
                wgsl!("cursor_fs"),
                "objectStruct",
                &[
                    ("nodeUniform0", cursor_color.clone()),
                    ("nodeUniform1", vec![1.]),
                    ("nodeUniform5", m4(model)),
                ],
            )?;
        }
        write(
            r,
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
                ("cameraViewMatrix", id4.clone()),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
            ],
        )?;
        write(
            r,
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform4", id4)],
        )?;
        let background = Color::from_hex(0x222222).0;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sculpt scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: t.resolve.as_ref(),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background.x,
                            g: background.y,
                            b: background.z,
                            a: 1.,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            set(&mut pass, &t.mesh);
            pass.set_vertex_buffer(0, self.gpu.normal.slice(..));
            pass.set_vertex_buffer(1, self.gpu.position.slice(..));
            pass.set_index_buffer(self.gpu.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.sculptor.mesh.nb_triangles() as u32 * 3, 0, 0..1);
            if self.cursor.visible {
                for (draw, shape, shown) in [
                    (&t.ring, &self.cursor.ring, self.cursor.ring_visible),
                    (&t.dot, &self.cursor.dot, true),
                ] {
                    if !shown {
                        continue;
                    }
                    set(&mut pass, draw);
                    pass.set_vertex_buffer(0, shape.0.slice(..));
                    pass.set_index_buffer(shape.1.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..shape.2, 0, 0..1);
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sculpt output"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            set(&mut pass, &t.output);
            pass.set_vertex_buffer(0, self.output_quad.slice(..));
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// The canvas pointer: down ( 10 + button ), move ( 0 ), up ( 20 +
    /// button ), in CSS pixels. The Sculptor's listener runs first, then the
    /// page's cursor listener, then OrbitControls'.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.events.push((kind, x, y));
    }
    fn draw_pointer(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        kind: u32,
        x: f64,
        y: f64,
    ) -> Result<()> {
        let view = Self::view(s, c)?;
        let (_, height, _) = viewport_css();
        let position = Vector2::new(x, y);
        match kind {
            10..=19 => {
                let button = kind - 10;
                self.buttons |= 1 << button;
                if button == 0 && self.sculptor.pointer_down(&view, x, y) {
                    // The stroke's start listener disables the controls.
                    return Ok(());
                }
                self.orbit = Some((position, button == 2));
            }
            20..=29 => {
                let button = kind - 20;
                self.buttons &= !(1 << button);
                if button == 0 && self.sculptor.sculpting {
                    self.sculptor.end_stroke();
                }
                self.orbit = None;
                // handlePointerUp: a hover cursor at the release.
                self.pointer = position;
                self.update_cursor(s, c)?;
            }
            _ => {
                if self.sculptor.sculpting {
                    self.sculptor.pointer_move(&view, x, y);
                }
                self.pointer = position;
                if self.buttons > 0 && !self.sculptor.sculpting {
                    self.cursor.visible = false;
                } else {
                    self.update_cursor(s, c)?;
                }
                if let Some((last, pan)) = self.orbit {
                    let (dx, dy) = (x - last.x, y - last.y);
                    let camera = camera_state(s, c)?;
                    if pan {
                        self.controls.pan(&camera, dx, dy, height);
                    } else {
                        self.controls.rotate(dx, dy, height);
                    }
                    self.orbit = Some((position, pan));
                }
            }
        }
        Ok(())
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        _dx: f64,
        _dy: f64,
        wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        if wheel != 0. {
            let camera = camera_state(s, c)?;
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        }
        Ok(())
    }
    /// tool, size, strength, negative and detail.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let value = value as f64;
        match index {
            0 => {
                let tool = *TOOLS
                    .get(value as usize)
                    .ok_or(Error::Invalid("sculpt tool"))?;
                self.sculptor.set_tool(tool);
            }
            1 => self.sculptor.set_size(value),
            2 => self.sculptor.set_strength(value),
            3 => self.sculptor.set_negative(value > 0.5),
            4 => self.sculptor.detail = value,
            _ => return Err(Error::Invalid("sculpt parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
