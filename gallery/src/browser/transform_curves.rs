//! TransformControls on a crate, the curve-modifier text flows whose curve
//! handles the gizmo drags, and the CCD IK arm whose target it drags.
use super::controls_attributes::{Controls, camera_state, viewport_css};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::interactive_scenes::grid_helper;
use super::text_clipping::text_geometry;
use super::text_shapes::{Extrude, Font};
use super::transform_controls::{Event, Mode, TransformControls};
use crate::compute::{BufferAccess, GpuBuffer};
use crate::curve::{CatmullRomCurve3, CatmullRomType, Curve};
use crate::raycast::Raycaster;
use crate::shader::ShaderProgram;
use crate::tsl::{self, NodeMaterial, Type, uniform};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
/// CurveModifier's spline texture: 1024 points per curve, four rows each.
const TEXTURE_WIDTH: u32 = 1024;
const TEXTURE_HEIGHT: u32 = 4;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
/// `DataUtils.toHalfFloat`: the table conversion, which truncates the mantissa.
fn to_half(value: f64) -> u16 {
    let f = (value.clamp(-65504., 65504.) as f32).to_bits();
    let e = ((f >> 23) & 0x1ff) as usize;
    let (base, shift) = {
        let sign: u32 = if e & 0x100 != 0 { 0x8000 } else { 0 };
        let x = (e & 0xff) as i32 - 127;
        if x < -27 {
            (sign, 24)
        } else if x < -14 {
            ((0x0400u32 >> (-x - 14)) | sign, (-x - 1) as u32)
        } else if x <= 15 {
            ((((x + 15) << 10) as u32) | sign, 13)
        } else if x < 128 {
            (0x7c00 | sign, 24)
        } else {
            (0x7c00 | sign, 13)
        }
    };
    (base + ((f & 0x007f_ffff) >> shift)) as u16
}
/// A line whose vertices live in a storage buffer, rewritten in place, as
/// `setFromPoints` rewrites the line's position attribute.
struct DynamicLine {
    node: Object3D,
    buffer: GpuBuffer,
}
impl DynamicLine {
    async fn new(s: &mut Scene, r: &Renderer, count: usize, hex: u32) -> Result<Self> {
        let buffer = GpuBuffer::zeroed(r, (count * 12) as u64, BufferAccess::Read)?;
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let i=tsl_vertex_index;let p=vec3(tsl_attribute_0[i*3u],tsl_attribute_0[i*3u+1u],tsl_attribute_0[i*3u+2u]);out.clip=u.projection*u.view*u.model*vec4(p,1.0);return out;}";
        let color = tsl::vec4(uniform(0, Type::Vec4).rgb(), tsl::float(1.));
        let source = NodeMaterial::new(color).wgsl_with_storage(0, &[Type::Float])?;
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(r, &source, &[&buffer], &[], projection).await?,
        ));
        let c = Color::from_hex(hex).0;
        m.uniforms[0] = [c.x as f32, c.y as f32, c.z as f32, 1.];
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(vec![0.; count * 3], 3, false)?),
        );
        let node = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Shader(m)),
            segments: false,
        }));
        s.get_mut(node)?.frustum_culled = false;
        Ok(Self { node, buffer })
    }
    fn set_from_points(&self, r: &Renderer, points: &[Vector3]) -> Result<()> {
        let data: Vec<f32> = points
            .iter()
            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();
        self.buffer.write(r, 0, bytemuck::cast_slice(&data))
    }
}
/// CurveModifier `Flow` / `InstancedFlow`: the spline texture and the curves' state.
struct Flow {
    texture: wgpu::Texture,
    node: Object3D,
    data: Vec<u16>,
    /// curveLengthArray.
    lengths: Vec<f64>,
    /// Each curve's cached arc lengths: after the first update, getLength()
    /// returns the cache of the previous update (the curve is not marked
    /// needsUpdate when its handles move).
    cached: Vec<Option<Vec<f64>>>,
    path_offset: f64,
    spine_length: f64,
}
/// initSplineTexture: the RGBA half-float spline texture and its sampler.
fn spline_texture(r: &Renderer, curves: u32) -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("flow spline texture"),
        size: wgpu::Extent3d {
            width: TEXTURE_WIDTH,
            height: TEXTURE_HEIGHT * curves,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    // wrapS = RepeatWrapping; `wrapY` is not a texture property, so T clamps.
    let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    (texture, view, sampler)
}
impl Flow {
    fn new(texture: wgpu::Texture, node: Object3D, curves: u32) -> Self {
        let n = curves as usize;
        Self {
            texture,
            node,
            data: vec![0; (TEXTURE_WIDTH * TEXTURE_HEIGHT * curves * 4) as usize],
            lengths: vec![0.; n],
            cached: vec![None; n],
            path_offset: 0.,
            spine_length: 400.,
        }
    }
    /// updateCurve( index, curve ): the length, then updateSplineTexture.
    fn update_curve(&mut self, r: &Renderer, index: usize, curve: &CatmullRomCurve3) -> Result<()> {
        let length = match &self.cached[index] {
            Some(cached) => *cached.last().ok_or(Error::Invalid("arc lengths"))?,
            None => *curve
                .lengths(200)?
                .last()
                .ok_or(Error::Invalid("arc lengths"))?,
        };
        self.spine_length = length;
        self.lengths[index] = length;
        let points = (TEXTURE_WIDTH * (TEXTURE_HEIGHT / 4)) as usize;
        let lengths = curve.lengths(points as u32 / 2)?;
        let spaced = curve.spaced_points_with(points as u32, &lengths)?;
        let (tangents, normals, binormals) =
            curve.frenet_frames_with(points as u32, true, &lengths)?;
        for i in 0..points {
            let row_offset = i / TEXTURE_WIDTH as usize;
            let column = i % TEXTURE_WIDTH as usize;
            for (k, v) in [spaced[i], tangents[i], normals[i], binormals[i]]
                .into_iter()
                .enumerate()
            {
                let row = k + row_offset + TEXTURE_HEIGHT as usize * index;
                let at = 4 * TEXTURE_WIDTH as usize * row + column * 4;
                self.data[at..at + 4].copy_from_slice(&[
                    to_half(v.x),
                    to_half(v.y),
                    to_half(v.z),
                    to_half(1.),
                ]);
            }
        }
        self.cached[index] = Some(lengths);
        let size = self.texture.size();
        r.queue.write_texture(
            self.texture.as_image_copy(),
            bytemuck::cast_slice(&self.data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size.width * 8),
                rows_per_image: Some(size.height),
            },
            size,
        );
        Ok(())
    }
}
/// The flow's vertex stage: the spine texture's position and Frenet basis.
/// `instanced` reads the per-instance curve, length and offset from the
/// instance translation, as InstancedFlow's shader reads instanceMatrix.
fn flow_projection(curves: u32, instanced: bool) -> String {
    let layers = TEXTURE_HEIGHT * curves;
    let (length, offset, row) = if instanced {
        (
            "t.x",
            "u.custom[0].x+t.z",
            &*format!("floor(mt)+t.y*{TEXTURE_HEIGHT}.0"),
        )
    } else {
        ("u.custom[1].x", "u.custom[0].x", "floor(mt)")
    };
    // WebGL transforms the curve normal into view space; the WebGPU example's
    // normalNode takes basis × normalLocal as the view normal unchanged.
    let normal = if instanced {
        "normalize((u.view*vec4((u.normal*vec4(basis*surface.local_normal,0.0)).xyz,0.0)).xyz)"
    } else {
        "basis*surface.local_normal"
    };
    format!(
        "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{var out=surface;let t=surface.local_position-position;\
let world=u.model*vec4(position,1.0);let bend=u.custom[0].w>0.0;let xw=select(1.0,0.0,bend);\
let portion=select(0.0,(world.x+u.custom[0].z)/{length},bend);var mt=portion*u.custom[0].y+{offset};mt=mt-floor(mt);\
let row={row};\
let sp=textureSampleLevel(tsl_texture_0,tsl_sampler_0,vec2(mt,(row+0.5)/{layers}.0),0.0).xyz;\
let a=textureSampleLevel(tsl_texture_0,tsl_sampler_0,vec2(mt,(row+1.5)/{layers}.0),0.0).xyz;\
let b=textureSampleLevel(tsl_texture_0,tsl_sampler_0,vec2(mt,(row+2.5)/{layers}.0),0.0).xyz;\
let c=textureSampleLevel(tsl_texture_0,tsl_sampler_0,vec2(mt,(row+3.5)/{layers}.0),0.0).xyz;\
let basis=mat3x3(a,b,c);let transformed=basis*vec3(world.x*xw,world.y,world.z)+sp;\
out.position=(u.model*vec4(transformed,1.0)).xyz;out.view_position=(u.view*vec4(out.position,1.0)).xyz;\
out.clip=u.projection*vec4(out.view_position,1.0);out.normal={normal};return out;}}"
    )
}
/// webgpu_modifier_curve and webgl_modifier_curve_instanced.
struct Curves {
    handles: Vec<Object3D>,
    /// ( handle indices, line ) per curve.
    curves: Vec<(Vec<usize>, DynamicLine)>,
    flow: Flow,
    select: Option<Vector2>,
    added: bool,
}
impl Curves {
    fn curve(&self, s: &Scene, index: usize) -> Result<CatmullRomCurve3> {
        let points = self.curves[index]
            .0
            .iter()
            .map(|&i| Ok(s.get(self.handles[i])?.position))
            .collect::<Result<Vec<_>>>()?;
        let mut curve = CatmullRomCurve3::new(points);
        curve.curve_type = CatmullRomType::Centripetal;
        curve.closed = true;
        Ok(curve)
    }
    /// The 'dragging-changed' listener on drag end, and the initial setup.
    fn refresh(&mut self, s: &Scene, r: &Renderer) -> Result<()> {
        for i in 0..self.curves.len() {
            let curve = self.curve(s, i)?;
            self.curves[i].1.set_from_points(r, &curve.points(50)?)?;
            self.flow.update_curve(r, i, &curve)?;
        }
        Ok(())
    }
}
/// The orbit camera pair of misc_controls_transform.
struct Cameras {
    perspective: bool,
    fov: f64,
    perspective_zoom: f64,
    top: f64,
    bottom: f64,
    orthographic_zoom: f64,
}
enum Input {
    Pointer(u32, f64, f64),
    Key(u32, bool),
    Orbit(f64, f64, f64, bool, f64),
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    controls: Option<Controls>,
    orbit_enabled: bool,
    cameras: Option<Cameras>,
    tc: TransformControls,
    /// TransformControls' pointermove listener, added on pointerdown.
    moving: bool,
    shift: bool,
    queue: Vec<Input>,
    curves: Option<Curves>,
    ik: Option<Box<Ik>>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            308 => (50., 0.1, 100., Vector3::new(5., 2.5, 5.)),
            312 => (
                55.,
                0.001,
                5000.,
                Vector3::new(0.9728517749133652, 1.1044765132727201, 0.7316689528482836),
            ),
            _ => (40., 1., 1000., Vector3::new(2., 2., 4.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = position;
        n.quaternion = Quaternion::IDENTITY;
        s.look_at(c, Vector3::ZERO)?;
        s.background = Color::BLACK;
        let tc = TransformControls::new(s, c)?;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            controls: None,
            orbit_enabled: true,
            cameras: None,
            tc,
            moving: false,
            shift: false,
            queue: vec![],
            curves: None,
            ik: None,
        };
        match id {
            308 => d.transform_scene(s, c).await?,
            312 => d.ik_scene(s, r).await?,
            _ => d.curve_scene(s, r).await?,
        }
        Ok(d)
    }
    async fn transform_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let grid = s.insert(NodeKind::Line(grid_helper(5., 10, 0x888888, 0x444444)?));
        if let NodeKind::Line(l) = &mut s.get_mut(grid)?.kind {
            Arc::make_mut(&mut l.material).properties_mut().tone_mapped = false;
        }
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 4.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(1., 1., 1.);
        let mut texture =
            decode_texture_image(&fetch(&format!("{ASSETS}/passes-decals/crate.gif")).await?)
                .await?;
        texture.srgb = true;
        texture.mipmap_filter = Some(Filter::Linear);
        texture.anisotropy = 16;
        let mut m = MeshLambertMaterial::default();
        m.properties.map = Some(Arc::new(texture));
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 1., 1.)?),
            Arc::new(Material::Lambert(m)),
        )));
        // OrbitControls: orbit.update() looks at the origin.
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, false);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.cameras = Some(Cameras {
            perspective: true,
            fov: 50.,
            perspective_zoom: 1.,
            top: 5.,
            bottom: -5.,
            orthographic_zoom: 1.,
        });
        // The 'change' listener renders: the gizmo updates at each change.
        self.tc.render_on_change = true;
        self.tc.attach(s, mesh)?;
        self.tc.events.clear();
        self.tc.update(s)
    }
    async fn curve_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let instanced = self.id == 310;
        let sets: Vec<Vec<[f64; 3]>> = if instanced {
            vec![
                vec![
                    [1., -0.5, -1.],
                    [1., -0.5, 1.],
                    [-1., -0.5, 1.],
                    [-1., -0.5, -1.],
                ],
                vec![
                    [-1., 0.5, -1.],
                    [-1., 0.5, 1.],
                    [1., 0.5, 1.],
                    [1., 0.5, -1.],
                ],
            ]
        } else {
            vec![vec![
                [1., 0., -1.],
                [1., 0., 1.],
                [-1., 0., 1.],
                [-1., 0., -1.],
            ]]
        };
        let box_geometry = Arc::new(BoxGeometry::build(0.1, 0.1, 0.1)?);
        let box_material = Arc::new(Material::Basic(MeshBasicMaterial::default()));
        let mut handles = vec![];
        let mut curves = vec![];
        for set in sets {
            let mut indices = vec![];
            for p in set {
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    box_geometry.clone(),
                    box_material.clone(),
                )));
                s.get_mut(h)?.position = Vector3::from_array(p);
                indices.push(handles.len());
                handles.push(h);
            }
            let line = DynamicLine::new(s, r, 51, 0x00ff00).await?;
            curves.push((indices, line));
        }
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0xffaa33),
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(-10., 10., 10.);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x003973),
            intensity: 3.,
        }));
        let font = Font::parse(
            &fetch(&format!("{ASSETS}/fonts/helvetiker_regular.typeface.json")).await?,
        )?;
        let mut g = text_geometry(
            &font,
            "Hello three.js!",
            0.2,
            &Extrude {
                curve_segments: 12,
                steps: 1,
                depth: 0.05,
                bevel: Some((0.02, 0.01, 5)),
            },
        )?;
        g.groups.clear();
        g.rotate_x(PI)?;
        let count = sets_len(instanced);
        let (texture, view, sampler) = spline_texture(r, count);
        let graph = NodeMaterial::new(tsl::base_color());
        let program = ShaderProgram::with_projection(
            r,
            &graph.wgsl(1)?,
            &[],
            &[(&view, &sampler)],
            &flow_projection(count, instanced),
        )
        .await?;
        let mut m = MeshStandardMaterial {
            energy_conservation: !instanced,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0x99ffff);
        m.properties.vertex_program = Some(Arc::new(program));
        let node = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Standard(m)),
        )));
        let mesh = s.get_mut(node)?;
        if instanced {
            mesh.frustum_culled = false;
            mesh.instances = vec![
                Instance {
                    matrix: Matrix4::IDENTITY,
                    color: Color::WHITE,
                };
                8
            ];
        }
        let mut k = Curves {
            handles,
            curves,
            flow: Flow::new(texture, node, count),
            select: None,
            added: false,
        };
        k.refresh(s, r)?;
        if instanced {
            // setCurve, moveIndividualAlongCurve and setColorAt per instance.
            let lengths = k.flow.lengths.clone();
            let n = s.get_mut(k.flow.node)?;
            for (i, instance) in n.instances.iter_mut().enumerate() {
                let curve = i % 2;
                let offset = i as f64 / 8.;
                instance.matrix =
                    Matrix4::from_translation(Vector3::new(lengths[curve], curve as f64, offset));
                instance.color =
                    Color::from_hex((16777215. * random(&mut self.seed)).floor() as u32);
            }
        }
        self.curves = Some(k);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    fn ndc(x: f64, y: f64) -> Vector2 {
        let (w, h, _) = viewport_css();
        Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.)
    }
    fn drain(&mut self, s: &Scene, r: &Renderer) -> Result<()> {
        for e in std::mem::take(&mut self.tc.events) {
            match e {
                Event::DraggingChanged(v) => {
                    if self.id == 308 {
                        self.orbit_enabled = !v;
                    }
                    if !v && let Some(k) = &mut self.curves {
                        k.refresh(s, r)?;
                    }
                }
                Event::MouseDown if self.id == 312 => self.orbit_enabled = false,
                Event::MouseUp if self.id == 312 => self.orbit_enabled = true,
                _ => {}
            }
        }
        Ok(())
    }
    fn pointer(&mut self, s: &mut Scene, r: &Renderer, kind: u32, x: f64, y: f64) -> Result<()> {
        let ndc = Self::ndc(x, y);
        match kind {
            10..=12 => {
                // The curve examples' own pointerdown listener comes first.
                if let Some(k) = &mut self.curves {
                    k.select = Some(ndc);
                }
                if self.tc.enabled {
                    self.moving = true;
                    self.tc.pointer_hover(s, ndc)?;
                    self.drain(s, r)?;
                    self.tc.pointer_down(s, ndc, kind as i32 - 10)?;
                }
            }
            0 => {
                if self.tc.enabled {
                    self.tc.pointer_hover(s, ndc)?;
                    self.drain(s, r)?;
                    if self.moving {
                        self.tc.pointer_move(s, ndc)?;
                    }
                }
            }
            20..=22 if self.tc.enabled => {
                self.moving = false;
                self.tc.pointer_up(s, kind as i32 - 20)?;
            }
            _ => {}
        }
        self.drain(s, r)
    }
    fn orthographic(&self, s: &mut Scene, c: Object3D) -> Result<Option<OrthographicCamera>> {
        Ok(match &s.get(c)?.kind {
            NodeKind::Camera(Camera::Orthographic(o)) => Some(o.clone()),
            _ => None,
        })
    }
    /// onWindowResize for the camera pair: the frustum follows the aspect.
    fn resize(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let Some(k) = &self.cameras else {
            return Ok(());
        };
        let (w, h, _) = viewport_css();
        let aspect = w / h;
        match &mut s.get_mut(c)?.kind {
            NodeKind::Camera(Camera::Perspective(p)) => {
                p.fov = k.fov;
                p.zoom = k.perspective_zoom;
                p.aspect = aspect;
            }
            NodeKind::Camera(Camera::Orthographic(o)) => {
                (o.top, o.bottom, o.zoom) = (k.top, k.bottom, k.orthographic_zoom);
                (o.left, o.right) = (k.bottom * aspect, k.top * aspect);
            }
            _ => {}
        }
        Ok(())
    }
    fn orbit(&mut self, s: &mut Scene, c: Object3D, input: [f64; 4], pan: bool) -> Result<()> {
        let [dx, dy, wheel, height] = input;
        let ortho = self.orthographic(s, c)?;
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        if !self.orbit_enabled {
            return Ok(());
        }
        if wheel != 0. {
            match ortho {
                Some(_) => controls.wheel_scale(wheel),
                None => controls.dolly(wheel, &camera_state(s, c)?, Vector2::ZERO),
            }
        } else if pan {
            match &ortho {
                Some(o) => {
                    let (w, h, _) = viewport_css();
                    let q = s.get(c)?.quaternion;
                    controls.pan_axes(
                        q,
                        dx * (o.right - o.left) / o.zoom / w,
                        dy * (o.top - o.bottom) / o.zoom / h,
                    );
                }
                None => controls.pan(&camera_state(s, c)?, dx, dy, height),
            }
        } else {
            controls.rotate(dx, dy, height);
        }
        if ortho.is_some() {
            let scale = controls.take_scale();
            if let (Some(k), NodeKind::Camera(Camera::Orthographic(o))) =
                (&mut self.cameras, &mut s.get_mut(c)?.kind)
            {
                o.zoom /= scale;
                k.orthographic_zoom = o.zoom;
            }
        }
        controls.update(s, c)?;
        // misc_controls_transform's orbit 'change' listener renders.
        if self.tc.render_on_change {
            self.tc.update(s)?;
        }
        Ok(())
    }
    /// misc_controls_transform's keydown and keyup listeners (event.key).
    fn keyboard(&mut self, s: &mut Scene, c: Object3D, code: u32, down: bool) -> Result<()> {
        if code == 16 {
            self.shift = down;
            let snaps = if down {
                [Some(1.), Some(15f64.to_radians()), Some(0.25)]
            } else {
                [None; 3]
            };
            return self.tc.set_snaps(s, snaps);
        }
        if !down {
            return Ok(());
        }
        // With Shift held, letters arrive as capitals and match no case.
        let letter = (65..=90).contains(&code);
        if letter && self.shift {
            return Ok(());
        }
        match code {
            81 => self.tc.set_space_local(s, !self.tc.local)?,
            87 => self.tc.set_mode(s, Mode::Translate)?,
            69 => self.tc.set_mode(s, Mode::Rotate)?,
            82 => self.tc.set_mode(s, Mode::Scale)?,
            67 => {
                let target = self
                    .controls
                    .as_ref()
                    .map(|c| c.target())
                    .unwrap_or_default();
                let Some(k) = &mut self.cameras else {
                    return Ok(());
                };
                k.perspective = !k.perspective;
                let n = s.get_mut(c)?;
                n.kind = NodeKind::Camera(if k.perspective {
                    Camera::Perspective(PerspectiveCamera {
                        fov: k.fov,
                        near: 0.1,
                        far: 100.,
                        zoom: k.perspective_zoom,
                        ..Default::default()
                    })
                } else {
                    Camera::Orthographic(OrthographicCamera {
                        top: k.top,
                        bottom: k.bottom,
                        near: 0.1,
                        far: 100.,
                        zoom: k.orthographic_zoom,
                        ..Default::default()
                    })
                });
                self.resize(s, c)?;
                self.tc.camera_changed(s)?;
                s.look_at(c, target)?;
                self.resize(s, c)?;
                self.tc.update(s)?;
            }
            86 => {
                let fov = random(&mut self.seed) + 0.1;
                let zoom = random(&mut self.seed) + 0.1;
                if let Some(k) = &mut self.cameras {
                    k.fov = fov * 160.;
                    k.bottom = -fov * 500.;
                    k.top = fov * 500.;
                    k.perspective_zoom = zoom * 5.;
                    k.orthographic_zoom = zoom * 5.;
                }
                self.resize(s, c)?;
                self.tc.update(s)?;
            }
            187 | 61 | 107 => {
                let size = self.tc.size + 0.1;
                self.tc.set_size(s, size)?;
            }
            189 | 173 | 109 => {
                let size = (self.tc.size - 0.1).max(0.1);
                self.tc.set_size(s, size)?;
            }
            88..=90 => {
                let i = (code - 88) as usize;
                let shown = !self.tc.show[i];
                self.tc.set_show(s, i, shown)?;
            }
            32 => {
                let enabled = !self.tc.enabled;
                self.tc.set_enabled(s, enabled)?;
            }
            27 => self.tc.reset(s)?,
            _ => {}
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let t = self.time;
        let delta = t - self.last;
        self.last = t;
        self.resize(s, c)?;
        for input in std::mem::take(&mut self.queue) {
            match input {
                Input::Pointer(kind, x, y) => self.pointer(s, r, kind, x, y)?,
                Input::Key(code, down) => {
                    if self.id == 308 {
                        self.keyboard(s, c, code, down)?;
                    }
                }
                Input::Orbit(dx, dy, wheel, pan, height) => {
                    self.orbit(s, c, [dx, dy, wheel, height], pan)?
                }
            }
            self.drain(s, r)?;
        }
        if let Some(k) = &mut self.curves {
            // animate(): the click's raycast against the handles attaches the gizmo.
            if let Some(ndc) = k.select.take() {
                let (camera, world) = s.camera(c)?;
                let mut ray = Raycaster::default();
                ray.set_from_camera(ndc, camera, world)?;
                if let Some(hit) = ray.intersect_objects(s, &k.handles, false)?.first() {
                    self.tc.attach(s, hit.object)?;
                    k.added = true;
                }
            }
            // moveAlongCurve( 0.001 ) once per animation frame, as 60 fps steps.
            let steps = (delta * 60.).round().max(0.);
            k.flow.path_offset += 0.001 * steps;
            let uniforms = [
                [k.flow.path_offset as f32, 1., 161., 1.],
                [k.flow.spine_length as f32, 0., 0., 0.],
            ];
            if let NodeKind::Mesh(m) = &mut s.get_mut(k.flow.node)?.kind {
                let p = m.materials[0].properties();
                if p.vertex_uniforms[0] != uniforms[0] || p.vertex_uniforms[1] != uniforms[1] {
                    let p = Arc::make_mut(&mut m.materials[0]).properties_mut();
                    p.vertex_uniforms[0] = uniforms[0];
                    p.vertex_uniforms[1] = uniforms[1];
                }
            }
            if !k.added {
                return Ok(());
            }
        }
        self.tc.update(s)?;
        self.tc.events.clear();
        Ok(())
    }
    /// Absolute CSS pointer events: 0 move, 10 + button down, 20 + button up.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.queue.push(Input::Pointer(kind, x, y));
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.queue.push(Input::Key(code, down));
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if matches!(self.id, 308 | 312) {
            self.queue.push(Input::Orbit(dx, dy, wheel, pan, height));
        }
        Ok(())
    }
    /// webgl_animation_skinning_ik's GUI: follow sphere, turn head, IK auto
    /// update and the manual update() button.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let k = self
            .ik
            .as_mut()
            .ok_or(Error::Invalid("transform/curve parameter"))?;
        match index {
            0..=2 => k.conf[index] = value > 0.5,
            3 => k.manual = true,
            _ => return Err(Error::Invalid("IK parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
fn sets_len(instanced: bool) -> u32 {
    if instanced { 2 } else { 1 }
}
/// webgl_animation_skinning_ik: Kira, the CCD IK arm chain, its helper, the
/// mirror sphere's CubeCamera and the example's settings.
pub(super) struct Ik {
    bones: Vec<Object3D>,
    kira: Object3D,
    head: Object3D,
    sphere: Object3D,
    cube: RenderTarget,
    cube_camera: Object3D,
    helper: Object3D,
    helper_spheres: [Object3D; 4],
    helper_line: DynamicLine,
    skinned: Vec<Object3D>,
    /// followSphere, turnHead, ik_solver.
    conf: [bool; 3],
    manual: bool,
}
/// The chain: target 22 ( target_hand_l ), effector 6 ( hand_l ), then the
/// links lowerarm_l and Upperarm_l with their Euler limits.
const IK_TARGET: usize = 22;
const IK_EFFECTOR: usize = 6;
const IK_LINKS: [(usize, [f64; 3], [f64; 3]); 2] = [
    (5, [1.2, -1.8, -0.4], [1.7, -1.1, 0.3]),
    (4, [0.1, -0.7, -1.8], [1.1, 0., -1.4]),
];
fn world_position(s: &Scene, h: Object3D) -> Result<Vector3> {
    Ok(s.get(h)?.matrix_world.w_axis.truncate())
}
impl Ik {
    /// CCDIKSolver.update(): one iteration over the links.
    fn solve(&self, s: &mut Scene) -> Result<()> {
        let target = world_position(s, self.bones[IK_TARGET])?;
        let effector = self.bones[IK_EFFECTOR];
        for (index, min, max) in IK_LINKS {
            let link = self.bones[index];
            let (_, q, _) = s.get(link)?.matrix_world.to_scale_rotation_translation();
            let link_position = world_position(s, link)?;
            let inverse = q.inverse();
            let effector_vec = norm(inverse * (world_position(s, effector)? - link_position));
            let target_vec = norm(inverse * (target - link_position));
            let angle = target_vec.dot(effector_vec).clamp(-1., 1.).acos();
            if angle < 1e-5 {
                continue;
            }
            let axis = norm(effector_vec.cross(target_vec));
            let n = s.get_mut(link)?;
            n.quaternion *= axis_angle(axis, angle);
            // rotation.setFromVector3( rotation.max( rotationMin ) ), then .min( rotationMax ).
            n.quaternion = quaternion_xyz(euler_xyz(n.quaternion).max(Vector3::from_array(min)));
            n.quaternion = quaternion_xyz(euler_xyz(n.quaternion).min(Vector3::from_array(max)));
            s.update_world_matrix(link, false, true)?;
        }
        Ok(())
    }
    /// CCDIKHelper.updateMatrixWorld: spheres and the line at the chain's bones,
    /// in the skinned mesh's frame.
    fn update_helper(&self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let world = s.get(self.kira)?.matrix_world;
        let inverse = world.inverse();
        let mut points = vec![];
        for index in [IK_TARGET, IK_EFFECTOR, IK_LINKS[0].0, IK_LINKS[1].0] {
            points.push(inverse.transform_point3(world_position(s, self.bones[index])?));
        }
        for (h, p) in self.helper_spheres.iter().zip(&points) {
            s.get_mut(*h)?.position = *p;
        }
        let (scale, rotation, translation) = world.to_scale_rotation_translation();
        let n = s.get_mut(self.helper)?;
        (n.position, n.quaternion, n.scale) = (translation, rotation, scale);
        s.update_world_matrix(self.helper, false, true)?;
        self.helper_line.set_from_points(r, &points)
    }
}
fn axis_angle(axis: Vector3, angle: f64) -> Quaternion {
    let s = (angle / 2.).sin();
    Quaternion::from_xyzw(axis.x * s, axis.y * s, axis.z * s, (angle / 2.).cos())
}
/// `Euler.setFromQuaternion( q, 'XYZ' )`, through the rotation matrix.
fn euler_xyz(q: Quaternion) -> Vector3 {
    let (x, y, z, w) = (q.x, q.y, q.z, q.w);
    let m11 = 1. - 2. * (y * y + z * z);
    let m12 = 2. * (x * y - w * z);
    let m13 = 2. * (x * z + w * y);
    let m22 = 1. - 2. * (x * x + z * z);
    let m23 = 2. * (y * z - w * x);
    let m32 = 2. * (y * z + w * x);
    let m33 = 1. - 2. * (x * x + y * y);
    let ey = m13.clamp(-1., 1.).asin();
    if m13.abs() < 0.9999999 {
        Vector3::new((-m23).atan2(m33), ey, (-m12).atan2(m11))
    } else {
        Vector3::new(m32.atan2(m22), ey, 0.)
    }
}
/// `Quaternion.setFromEuler` for the XYZ order.
fn quaternion_xyz(e: Vector3) -> Quaternion {
    let (c1, c2, c3) = ((e.x / 2.).cos(), (e.y / 2.).cos(), (e.z / 2.).cos());
    let (s1, s2, s3) = ((e.x / 2.).sin(), (e.y / 2.).sin(), (e.z / 2.).sin());
    Quaternion::from_xyzw(
        s1 * c2 * c3 + c1 * s2 * s3,
        c1 * s2 * c3 - s1 * c2 * s3,
        c1 * c2 * s3 + s1 * s2 * c3,
        c1 * c2 * c3 - s1 * s2 * s3,
    )
}
fn norm(v: Vector3) -> Vector3 {
    let l = v.length();
    if l == 0. { v } else { v / l }
}
impl Demo {
    async fn ik_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.fog = Some(Fog::Exp2 {
            color: Color::WHITE,
            density: 0.17,
        });
        s.background = Color::WHITE;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 8.,
        }));
        let (a, b, i) =
            super::gltf_viewer::load_asset(&format!("{ASSETS}/transform-curves/kira.glb")).await?;
        let joints: Vec<usize> = a
            .skins()
            .next()
            .ok_or(Error::Invalid("kira skin"))?
            .joints()
            .map(|j| j.index())
            .collect();
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let group = s.insert(NodeKind::Group);
        for &h in &instance.roots {
            s.add(group, h)?;
        }
        // r186 WebGL physical lighting also attenuates the ambient diffuse by the
        // specular multiple scattering.
        for &h in &instance.meshes {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for m in &mut m.materials {
                    if let Material::Standard(m)
                    | Material::Physical(MeshPhysicalMaterial { base: m, .. }) = Arc::make_mut(m)
                    {
                        m.energy_conservation = true;
                    }
                }
            }
        }
        // three's nodes recompose their matrices from position, quaternion and
        // scale: the IK, the head's lookAt and the gizmo all move them.
        for &h in &instance.nodes {
            s.get_mut(h)?.matrix_auto_update = true;
        }
        let bones: Vec<Object3D> = joints.iter().map(|&j| instance.nodes[j]).collect();
        // The glTF nodes by name (a node's primitives may be named children).
        let named = |_: &Scene, name: &str| -> Result<Object3D> {
            a.nodes()
                .filter(|n| n.name() == Some(name))
                .last()
                .map(|n| instance.nodes[n.index()])
                .ok_or(Error::Invalid("named kira node"))
        };
        let head = named(s, "head")?;
        let hand = named(s, "hand_l")?;
        let target = named(s, "target_hand_l")?;
        let sphere = named(s, "boule")?;
        let kira = named(s, "Kira_Shirt_left")?;
        let target_position = s.get(sphere)?.position;
        s.update_world_matrix(group, false, true)?;
        s.attach(hand, sphere)?;
        // WebGLCubeRenderTarget( 1024 ): 8-bit, linear, no mipmaps.
        let cube = RenderTarget::with_options(
            &r.device,
            1024,
            1024,
            RenderTargetOptions {
                depth: 6,
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..Default::default()
            },
        )?;
        let view = cube.texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let cube_camera = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 90.,
            aspect: 1.,
            near: 0.05,
            far: 50.,
            ..Default::default()
        })));
        // MeshBasicMaterial( { envMap } ): the reflected view ray into the cube.
        let env = tsl::WgslFn::new(
            "mirror_env",
            "fn mirror_env(n:vec3<f32>,p:vec3<f32>)->vec4<f32>{let v=normalize(p-u.camera.xyz);let d=reflect(v,normalize(n));return vec4(textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb,1.0);}",
            &[Type::Vec3, Type::Vec3],
            Type::Vec4,
        )?
        .call(&[tsl::normal_world(), tsl::position_world()]);
        let source = NodeMaterial::new(env).wgsl_with_texture_types(&[Type::TextureCube], &[])?;
        let program = ShaderProgram::with_projection_and_dimensions(
            r,
            &source,
            &[(&view, &sampler)],
            &[wgpu::TextureViewDimension::Cube],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let mirror = Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program))));
        let mirror_mesh = if matches!(s.get(sphere)?.kind, NodeKind::Mesh(_)) {
            sphere
        } else {
            *s.get(sphere)?
                .children()
                .first()
                .ok_or(Error::Invalid("sphere mesh"))?
        };
        if let NodeKind::Mesh(m) = &mut s.get_mut(mirror_mesh)?.kind {
            for material in m.materials.iter_mut() {
                *material = mirror.clone();
            }
        }
        // OOI.kira.add( skeleton.bones[ 0 ] ): the local transform is kept.
        s.add(kira, bones[0])?;
        // CCDIKHelper( mesh, iks, 0.01 ).
        let helper = s.insert(NodeKind::Group);
        let sphere_geometry = Arc::new(SphereGeometry::build(0.01, 16, 8)?);
        let helper_material = |hex: u32, line: bool| {
            let mut p = MaterialProperties {
                color: Color::from_hex(hex),
                ..Default::default()
            };
            p.depth_test = false;
            p.depth_write = false;
            p.transparent = true;
            Arc::new(if line {
                Material::Line(LineBasicMaterial {
                    properties: p,
                    ..Default::default()
                })
            } else {
                Material::Basic(MeshBasicMaterial { properties: p })
            })
        };
        let mut helper_spheres = vec![];
        let link_material = helper_material(0x8888ff, false);
        for m in [
            helper_material(0xff8888, false),
            helper_material(0x88ff88, false),
            link_material.clone(),
            link_material,
        ] {
            let h = s.insert(NodeKind::Mesh(Mesh::new(sphere_geometry.clone(), m)));
            s.add(helper, h)?;
            helper_spheres.push(h);
        }
        let helper_line = DynamicLine::new(s, r, 4, 0xff0000).await?;
        if let NodeKind::Line(l) = &mut s.get_mut(helper_line.node)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut l.material)
        {
            m.properties.depth_test = false;
            m.properties.depth_write = false;
            m.properties.transparent = true;
        }
        s.add(helper, helper_line.node)?;
        // The Line's bounding sphere is computed once, from the first helper
        // update (the rest pose); the positions then stream to the GPU only.
        s.update()?;
        let world = s.get(kira)?.matrix_world.inverse();
        let mut first = vec![];
        for index in [IK_TARGET, IK_EFFECTOR, IK_LINKS[0].0, IK_LINKS[1].0] {
            let p = world.transform_point3(world_position(s, bones[index])?);
            first.extend([p.x as f32, p.y as f32, p.z as f32]);
        }
        let n = s.get_mut(helper_line.node)?;
        n.frustum_culled = true;
        if let NodeKind::Line(l) = &mut n.kind {
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(first, 3, false)?),
            );
            l.geometry = Arc::new(g);
        }
        let skinned: Vec<Object3D> = s
            .traverse(group, false)?
            .into_iter()
            .filter(|&h| s.get(h).is_ok_and(|n| n.skin.is_some()))
            .collect();
        // OrbitControls: minDistance 0.2, maxDistance 1.5, damping.
        let mut controls = Controls::new(Some(0.05), (0.2, 1.5), PI, false);
        controls.set_target(target_position);
        self.controls = Some(controls);
        // TransformControls: size 0.75, showX false, world space, on target_hand_l.
        self.tc.size = 0.75;
        self.tc.show[0] = false;
        self.tc.attach(s, target)?;
        self.tc.events.clear();
        self.ik = Some(Box::new(Ik {
            bones,
            kira,
            head,
            sphere,
            cube,
            cube_camera,
            helper,
            helper_spheres: helper_spheres
                .try_into()
                .map_err(|_| Error::Invalid("helper spheres"))?,
            helper_line,
            skinned,
            conf: [false, true, true],
            manual: false,
        }));
        Ok(())
    }
    /// animate(): the mirror sphere's CubeCamera, then follow, head turn, IK and
    /// the damped orbit, then the view.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let Some(k) = self.ik.as_mut() else {
            return Ok(false);
        };
        s.update()?;
        s.get_mut(k.sphere)?.visible = false;
        let position = world_position(s, k.sphere)?;
        s.get_mut(k.cube_camera)?.position = position;
        self.tc.update(s)?;
        k.update_helper(s, r)?;
        for (i, (direction, up)) in [
            (Vector3::NEG_X, Vector3::Y),
            (Vector3::X, Vector3::Y),
            (Vector3::Y, Vector3::NEG_Z),
            (Vector3::NEG_Y, Vector3::Z),
            (Vector3::Z, Vector3::Y),
            (Vector3::NEG_Z, Vector3::Y),
        ]
        .into_iter()
        .enumerate()
        {
            s.get_mut(k.cube_camera)?.up = up;
            s.look_at(k.cube_camera, position + direction)?;
            k.cube.set_layer(i as u32)?;
            r.render(s, k.cube_camera, &k.cube)?;
        }
        s.get_mut(k.sphere)?.visible = true;
        let controls = self
            .controls
            .as_mut()
            .ok_or(Error::Invalid("IK controls"))?;
        if k.conf[0] {
            let v = world_position(s, k.sphere)?;
            let target = controls.target();
            controls.set_target(target + (v - target) * 0.1);
        }
        if k.conf[1] {
            // head.lookAt( sphere ), then rotation.y += π.
            let v = world_position(s, k.sphere)?;
            s.look_at(k.head, v)?;
            let n = s.get_mut(k.head)?;
            let mut euler = euler_xyz(n.quaternion);
            euler.y += PI;
            n.quaternion = quaternion_xyz(euler);
        }
        if k.conf[2] || std::mem::take(&mut k.manual) {
            // updateIK(): the solver, then computeBoundingSphere() per SkinnedMesh.
            k.solve(s)?;
            s.update()?;
            for &h in &k.skinned {
                r.compute_bounding_sphere(s, h)?;
            }
        }
        controls.update(s, c)?;
        s.update()?;
        self.tc.update(s)?;
        k.update_helper(s, r)?;
        r.render(s, c, out)?;
        Ok(true)
    }
}
