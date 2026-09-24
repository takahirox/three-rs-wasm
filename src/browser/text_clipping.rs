//! The advanced clipping volume, spline tubes, 3D text, the tessellated text and
//! the custom-attribute text lines from the pinned WebGL examples.
mod tube;
use super::controls_attributes::{
    CameraState, Controls, additive, call, camera_helper, camera_state, set_uniform,
    update_camera_helper, viewport_css,
};
use super::gltf_viewer::fetch;
use super::shapes_lights::hsl;
use super::text_shapes::{Extrude, Font, extrude};
use super::trackball_sprites::{Mode, Trackball};
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    curve::Curve,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    tsl::*,
};
use std::f64::consts::PI;
use std::sync::Arc;
use tube::{Spline, splines};

const ASSETS: &str = "/web/gallery/assets";
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
fn f32s(g: &BufferGeometry, name: &str) -> Vec<f32> {
    match g.attributes.get(name) {
        Some(Attribute::F32(a)) => a.array().to_vec(),
        _ => vec![],
    }
}
/// ExtrudeGeometry positions, face normals from computeVertexNormals and the
/// lid/side groups.
fn text_geometry(font: &Font, text: &str, size: f64, o: &Extrude) -> Result<BufferGeometry> {
    let (positions, groups) = extrude(&font.shapes(text, size), o);
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(positions)?);
    g.compute_vertex_normals()?;
    g.groups = groups
        .into_iter()
        .map(|(start, count, material_index)| Group {
            start,
            count,
            material_index,
        })
        .collect();
    Ok(g)
}
/// The Float32 bounding box of xyz triples.
fn bounds(p: &[f32]) -> (Vector3, Vector3) {
    let mut min = Vector3::splat(f64::INFINITY);
    let mut max = Vector3::splat(f64::NEG_INFINITY);
    for v in p.as_chunks::<3>().0 {
        let v = Vector3::new(v[0] as f64, v[1] as f64, v[2] as f64);
        min = min.min(v);
        max = max.max(v);
    }
    (min, max)
}
/// BufferGeometry.center(): translate by the negated bounding-box center.
fn center(p: &mut [f32]) {
    let (min, max) = bounds(p);
    let c = ((min + max) * 0.5).to_array();
    for v in p.as_chunks_mut::<3>().0 {
        for k in 0..3 {
            v[k] = (v[k] as f64 - c[k]) as f32;
        }
    }
}
/// TessellateModifier( maxEdgeLength, maxIterations ).modify for positions and normals.
fn tessellate(
    positions: &[f32],
    normals: &[f32],
    max_edge: f64,
    max_iterations: usize,
) -> (Vec<f32>, Vec<f32>) {
    let mut p2: Vec<f64> = positions.iter().map(|&v| v as f64).collect();
    let mut n2: Vec<f64> = normals.iter().map(|&v| v as f64).collect();
    let max_sq = max_edge * max_edge;
    let lerp = |a: [f64; 3], b: [f64; 3]| [0, 1, 2].map(|k| a[k] + (b[k] - a[k]) * 0.5);
    let (mut iteration, mut tessellating) = (0, true);
    while tessellating && iteration < max_iterations {
        iteration += 1;
        tessellating = false;
        let (p, n) = (std::mem::take(&mut p2), std::mem::take(&mut n2));
        for (tp, tn) in p.as_chunks::<9>().0.iter().zip(n.as_chunks::<9>().0) {
            let at = |a: &[f64; 9], i: usize| [a[i * 3], a[i * 3 + 1], a[i * 3 + 2]];
            let mut vs = [at(tp, 0), at(tp, 1), at(tp, 2), [0.; 3]];
            let mut ns = [at(tn, 0), at(tn, 1), at(tn, 2), [0.; 3]];
            let d2 = |a: [f64; 3], b: [f64; 3]| {
                let (x, y, z) = (a[0] - b[0], a[1] - b[1], a[2] - b[2]);
                x * x + y * y + z * z
            };
            let (dab, dbc, dac) = (d2(vs[0], vs[1]), d2(vs[1], vs[2]), d2(vs[0], vs[2]));
            let faces: &[[usize; 3]] = if dab > max_sq || dbc > max_sq || dac > max_sq {
                tessellating = true;
                if dab >= dbc && dab >= dac {
                    vs[3] = lerp(vs[0], vs[1]);
                    ns[3] = lerp(ns[0], ns[1]);
                    &[[0, 3, 2], [3, 1, 2]]
                } else if dbc >= dab && dbc >= dac {
                    vs[3] = lerp(vs[1], vs[2]);
                    ns[3] = lerp(ns[1], ns[2]);
                    &[[0, 1, 3], [3, 2, 0]]
                } else {
                    vs[3] = lerp(vs[0], vs[2]);
                    ns[3] = lerp(ns[0], ns[2]);
                    &[[0, 1, 3], [3, 1, 2]]
                }
            } else {
                &[[0, 1, 2]]
            };
            for f in faces {
                for &k in f {
                    p2.extend(vs[k]);
                    n2.extend(ns[k]);
                }
            }
        }
    }
    let f = |v: Vec<f64>| v.into_iter().map(|x| x as f32).collect();
    (f(p2), f(n2))
}
/// Plane.setFromCoplanarPoints.
fn plane_from_points(a: Vector3, b: Vector3, c: Vector3) -> Plane {
    let normal = (c - b).cross(a - b).normalize();
    Plane {
        normal,
        constant: -a.dot(normal),
    }
}
/// Plane.applyMatrix4 with the matrix's normal matrix.
fn transform_plane(p: Plane, m: Matrix4) -> Plane {
    let point = m.transform_point3(p.normal * -p.constant);
    let normal_matrix = Matrix3::from_mat4(m).inverse().transpose();
    let normal = (normal_matrix * p.normal).normalize();
    Plane {
        normal,
        constant: -point.dot(normal),
    }
}
/// planeToMatrix: X/Y aligned to the plane, Hughes & Moeller's basis.
fn plane_matrix(p: Plane) -> Matrix4 {
    let z = p.normal;
    let y = if z.x.abs() > z.z.abs() {
        Vector3::new(-z.y, z.x, 0.)
    } else {
        Vector3::new(0., -z.z, z.y)
    }
    .normalize();
    let x = y.cross(z);
    let t = p.normal * -p.constant;
    Matrix4::from_cols(x.extend(0.), y.extend(0.), z.extend(0.), t.extend(1.))
}
/// Matrix4.lookAt( eye, target, up ) as a rotation.
fn look_rotation(eye: Vector3, target: Vector3, up: Vector3) -> Quaternion {
    let mut z = eye - target;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    z = z.normalize();
    let mut x = up.cross(z);
    if x.length_squared() == 0. {
        if up.z.abs() == 1. {
            z.x += 0.0001;
        } else {
            z.z += 0.0001;
        }
        z = z.normalize();
        x = up.cross(z);
    }
    x = x.normalize();
    let y = z.cross(x);
    Quaternion::from_mat3(&Matrix3::from_cols(x, y, z))
}
fn euler(x: f64, y: f64, z: f64) -> Quaternion {
    Euler {
        angles: Vector3::new(x, y, z),
        order: EulerOrder::XYZ,
    }
    .quaternion()
}
fn set_geometry(s: &mut Scene, h: Object3D, g: &Arc<BufferGeometry>) -> Result<()> {
    if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
        m.geometry = g.clone();
    }
    Ok(())
}
fn material_mut(s: &mut Scene, h: Object3D) -> Result<&mut MaterialProperties> {
    match &mut s.get_mut(h)?.kind {
        NodeKind::Mesh(m) => Ok(Arc::make_mut(&mut m.materials[0]).properties_mut()),
        _ => Err(Error::Invalid("material node")),
    }
}

/// webgl_clipping_advanced: the transformed tetrahedron planes and the rotating
/// cylindrical global planes.
struct Clipping {
    object: Object3D,
    visualization: Object3D,
    planes: [Object3D; 4],
    base: [Plane; 4],
    matrices: [Matrix4; 4],
    global: Vec<Plane>,
    /// Local Enabled, Shadows, Visualize, global Enabled.
    params: [bool; 4],
}
/// webgl_geometry_extrude_splines.
struct Splines {
    splines: Vec<Spline>,
    tube: Object3D,
    wire: Object3D,
    camera: Object3D,
    helper: Object3D,
    eye: Object3D,
    binormals: Vec<Vector3>,
    tangents: usize,
    /// spline, scale, extrusionSegments, radiusSegments, closed, animationView,
    /// lookAhead, cameraHelper.
    params: [f64; 8],
    built: Option<[f64; 4]>,
}
/// webgl_geometry_text.
struct Text {
    fonts: Vec<Vec<u8>>,
    parsed: Vec<Option<Font>>,
    font: usize,
    bold: bool,
    bevel: bool,
    text: String,
    first_letter: bool,
    group: Object3D,
    meshes: [Object3D; 2],
    light: Object3D,
    target_rotation: f64,
    rotation: f64,
    drag: Option<(f64, f64)>,
    /// refreshText() and changeColor, applied in prepare().
    dirty: bool,
    color: Option<Color>,
}
/// webgl_custom_attributes_lines: the displacement random walk, streamed each frame.
struct Lines {
    line: Object3D,
    displacement: Vec<f32>,
    buffer: GpuBuffer,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    controls: Option<Controls>,
    trackball: Option<Trackball>,
    clipping: Option<Clipping>,
    splines: Option<Splines>,
    text: Option<Text>,
    tessellated: Option<Object3D>,
    lines: Option<Lines>,
}
const FONTS: [&str; 5] = [
    "helvetiker",
    "optimer",
    "gentilis",
    "droid/droid_sans",
    "droid/droid_serif",
];
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            248 => (36., 0.25, 16., Vector3::new(0., 1.5, 3.)),
            249 => (50., 0.01, 10000., Vector3::new(0., 50., 500.)),
            250 => (30., 1., 1500., Vector3::new(0., 400., 700.)),
            251 => (40., 1., 10000., Vector3::new(-100., 100., 200.)),
            _ => (30., 1., 10000., Vector3::new(0., 0., 400.)),
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
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            controls: None,
            trackball: None,
            clipping: None,
            splines: None,
            text: None,
            tessellated: None,
            lines: None,
        };
        match id {
            248 => d.clipping_scene(s, c)?,
            249 => d.splines_scene(s, c, r, aspect).await?,
            250 => d.text_scene(s, c).await?,
            251 => d.tessellation_scene(s, c, r).await?,
            _ => d.lines_scene(s, r).await?,
        }
        Ok(d)
    }
    fn clipping_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.,
        }));
        let spot = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 60.,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 2.,
            angle: PI / 5.,
            penumbra: 0.2,
        }));
        let n = s.get_mut(spot)?;
        n.position = Vector3::new(2., 3., 3.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            map_size: Some(1024),
            near: 3.,
            far: 10.,
            ..Default::default()
        };
        let dir = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 1.5,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(dir)?;
        n.position = Vector3::new(0., 2., 0.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            map_size: Some(1024),
            near: 1.,
            far: 10.,
            extent: 1.,
            ..Default::default()
        };
        let v = Vector3::new;
        let h = std::f64::consts::FRAC_1_SQRT_2;
        let vertices = [v(1., 0., h), v(-1., 0., h), v(0., 1., -h), v(0., -1., -h)];
        let base = [[0, 1, 2], [0, 2, 3], [0, 3, 1], [1, 3, 2]]
            .map(|[a, b, c]| plane_from_points(vertices[a], vertices[b], vertices[c]));
        let matrices = base.map(plane_matrix);
        let global = (0..5)
            .map(|i| {
                let angle = i as f64 * PI * 2. / 5.;
                Plane {
                    normal: v(angle.cos(), 0., angle.sin()),
                    constant: 2.5,
                }
            })
            .collect();
        let mut clip = MeshPhongMaterial {
            shininess: 100.,
            ..Default::default()
        };
        clip.properties.color = Color::from_hex(0xee0a10);
        clip.properties.side = Side::Double;
        clip.properties.clip_shadows = true;
        let object = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(0.18, 0.18, 0.18)?),
            Arc::new(Material::Phong(clip)),
        )));
        let n = s.get_mut(object)?;
        n.cast_shadow = true;
        for z in -2..=2 {
            for y in -2..=2 {
                for x in -2..=2 {
                    n.instances.push(Instance {
                        matrix: Matrix4::from_translation(v(
                            x as f64 / 5.,
                            y as f64 / 5.,
                            z as f64 / 5.,
                        )),
                        ..Default::default()
                    });
                }
            }
        }
        let plane = Arc::new(PlaneGeometry::build(3., 3., 1, 1)?);
        let visualization = s.insert(NodeKind::Group);
        s.get_mut(visualization)?.visible = false;
        let mut planes = vec![];
        for i in 0..4 {
            // color.setHSL( i / n, 0.5, 0.5 ).getHex(): quantized through sRGB bytes.
            let c = hsl(i as f64 / 4., 0.5, 0.5).0.to_array();
            let byte = |v: f64| (linear_to_srgb(v) * 255.).clamp(0., 255.).round() as u32;
            let hex = (byte(c[0]) << 16) | (byte(c[1]) << 8) | byte(c[2]);
            let mut m = MeshBasicMaterial::default();
            m.properties.color = Color::from_hex(hex);
            m.properties.side = Side::Double;
            m.properties.opacity = 0.2;
            m.properties.transparent = true;
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                plane.clone(),
                Arc::new(Material::Basic(m)),
            )));
            s.add(visualization, mesh)?;
            planes.push(mesh);
        }
        let mut ground = MeshPhongMaterial {
            shininess: 10.,
            ..Default::default()
        };
        ground.properties.color = Color::from_hex(0xa0adaf);
        let g = s.insert(NodeKind::Mesh(Mesh::new(
            plane,
            Arc::new(Material::Phong(ground)),
        )));
        let n = s.get_mut(g)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.scale = Vector3::splat(3.);
        n.receive_shadow = true;
        // WebGLClipping never applies renderer.clippingPlanes to shadow maps.
        s.clipping_shadows = false;
        let mut controls = Controls::new(None, (1., 8.), PI, true);
        controls.set_target(v(0., 1., 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.clipping = Some(Clipping {
            object,
            visualization,
            planes: planes.try_into().map_err(|_| Error::Invalid("planes"))?,
            base,
            matrices,
            global,
            params: [true, true, false, false],
        });
        Ok(())
    }
    fn prepare_clipping(&mut self, s: &mut Scene, t: f64) -> Result<()> {
        let k = self.clipping.as_ref().ok_or(Error::Invalid("clipping"))?;
        let [local, shadows, visualize, global] = k.params;
        let q = euler(t * 0.5, t * 0.2, 0.);
        let n = s.get_mut(k.object)?;
        n.position.y = 1.;
        n.quaternion = q;
        let bouncy = (t * 0.5).cos() * 0.5 + 0.7;
        let transform = Matrix4::from_rotation_translation(q, Vector3::new(0., 1., 0.))
            * Matrix4::from_scale(Vector3::splat(bouncy));
        let planes = k.base.map(|p| transform_plane(p, transform));
        {
            let m = material_mut(s, k.object)?;
            m.clipping_planes = if local { planes.to_vec() } else { vec![] };
            m.clip_shadows = shadows;
        }
        for i in 0..4 {
            let others = (0..4).filter(|&j| j != i).map(|j| planes[j]).collect();
            material_mut(s, k.planes[i])?.clipping_planes = if local { others } else { vec![] };
            let (scale, rotation, translation) =
                (transform * k.matrices[i]).to_scale_rotation_translation();
            let n = s.get_mut(k.planes[i])?;
            n.position = translation;
            n.quaternion = rotation;
            n.scale = scale;
        }
        s.get_mut(k.visualization)?.visible = visualize;
        let rotation = Matrix4::from_rotation_y(t * 0.1);
        s.clipping_planes = if global {
            k.global
                .iter()
                .map(|&p| transform_plane(p, rotation))
                .collect()
        } else {
            vec![]
        };
        Ok(())
    }
    async fn splines_scene(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        r: &Renderer,
        aspect: f64,
    ) -> Result<()> {
        s.background = Color::from_hex(0xf0f0f0);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 1.5,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::Z;
        // The spline camera keeps the aspect it was created with.
        let camera = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 84.,
            near: 0.01,
            far: 1000.,
            aspect,
            ..Default::default()
        })));
        let (g, program) = camera_helper(r).await?;
        let helper = s.insert(NodeKind::Line(Line {
            geometry: g,
            material: Arc::new(Material::Shader(ShaderMaterial::new(program))),
            segments: true,
        }));
        let n = s.get_mut(helper)?;
        n.frustum_culled = false;
        n.visible = false;
        let mut lambert = MeshLambertMaterial::default();
        lambert.properties.color = Color::from_hex(0xff00ff);
        let empty = Arc::new(BufferGeometry::default());
        let tube = s.insert(NodeKind::Mesh(Mesh::new(
            empty.clone(),
            Arc::new(Material::Lambert(lambert)),
        )));
        let mut wire = MeshBasicMaterial::default();
        wire.properties.color = Color::BLACK;
        wire.properties.opacity = 0.3;
        wire.properties.wireframe = true;
        wire.properties.transparent = true;
        let wire = s.insert(NodeKind::Mesh(Mesh::new(
            empty,
            Arc::new(Material::Basic(wire)),
        )));
        s.add(tube, wire)?;
        let mut eye = MeshBasicMaterial::default();
        eye.properties.color = Color::from_hex(0xdddddd);
        let eye = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(5., 32, 16)?),
            Arc::new(Material::Basic(eye)),
        )));
        s.get_mut(eye)?.visible = false;
        let mut controls = Controls::new(None, (100., 2000.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.splines = Some(Splines {
            splines: splines(),
            tube,
            wire,
            camera,
            helper,
            eye,
            binormals: vec![],
            tangents: 0,
            params: [0., 4., 100., 3., 1., 0., 0., 0.],
            built: None,
        });
        Ok(())
    }
    fn prepare_splines(&mut self, s: &mut Scene, t: f64) -> Result<()> {
        let k = self.splines.as_mut().ok_or(Error::Invalid("splines"))?;
        let [
            spline,
            scale,
            segments,
            radial,
            closed,
            _,
            look_ahead,
            show_helper,
        ] = k.params;
        let key = [spline, segments, radial, closed];
        // addTube(): a new TubeGeometry whenever a geometry parameter changes.
        if k.built != Some(key) {
            let t = tube::tube(
                &k.splines[spline as usize],
                segments as u32,
                2.,
                radial as u32,
                closed > 0.5,
            )?;
            let mut g = BufferGeometry::default();
            g.set_attribute("position", vec3s(t.positions)?);
            g.set_attribute("normal", vec3s(t.normals)?);
            g.set_index(Some(t.index));
            let g = Arc::new(g);
            set_geometry(s, k.tube, &g)?;
            set_geometry(s, k.wire, &g)?;
            k.binormals = t.binormals;
            k.tangents = t.tangents;
            k.built = Some(key);
        }
        s.get_mut(k.tube)?.scale = Vector3::splat(scale);
        let path = &k.splines[spline as usize];
        let lengths = path.lengths(200)?;
        let at = |u: f64| path.u_to_t(u, &lengths);
        let u = (t * 1000. % 20000.) / 20000.;
        let mut position = path.point(at(u))? * scale;
        let pickt = u * k.tangents as f64;
        let pick = pickt.floor() as usize;
        let next = (pick + 1) % k.tangents;
        let binormal =
            (k.binormals[next] - k.binormals[pick]) * (pickt - pick as f64) + k.binormals[pick];
        let direction = path.tangent(at(u))?;
        let normal = binormal.cross(direction);
        position += normal * 15.;
        let length = lengths[lengths.len() - 1];
        let mut look_at = path.point(at((u + 30. / length) % 1.))? * scale;
        if look_ahead < 0.5 {
            look_at = position + direction;
        }
        let n = s.get_mut(k.camera)?;
        n.position = position;
        n.quaternion = look_rotation(position, look_at, normal);
        s.get_mut(k.eye)?.position = position;
        let visible = show_helper > 0.5;
        s.get_mut(k.eye)?.visible = visible;
        s.get_mut(k.helper)?.visible = visible;
        s.update_world_matrix(k.camera, true, false)?;
        update_camera_helper(s, k.camera, k.helper)
    }
    async fn text_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.fog = Some(Fog::Linear {
            color: Color::BLACK,
            near: 250.,
            far: 1400.,
        });
        let dir = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 0.4,
            target: Vector3::ZERO,
        }));
        s.get_mut(dir)?.position = Vector3::Z;
        let light = s.insert(NodeKind::Light(Light::Point {
            color: hsl(random(&mut self.seed), 1., 0.5),
            intensity: 4.5,
            distance: 0.,
            decay: 0.,
        }));
        s.get_mut(light)?.position = Vector3::new(0., 100., 90.);
        let mut front = MeshPhongMaterial::default();
        front.properties.flat_shading = true;
        let materials = vec![
            Arc::new(Material::Phong(front)),
            Arc::new(Material::Phong(MeshPhongMaterial::default())),
        ];
        let group = s.insert(NodeKind::Group);
        s.get_mut(group)?.position.y = 100.;
        let mut meshes = vec![];
        for _ in 0..2 {
            let mut mesh = Mesh::new(Arc::new(BufferGeometry::default()), materials[0].clone());
            mesh.materials = materials.clone();
            let h = s.insert(NodeKind::Mesh(mesh));
            s.add(group, h)?;
            meshes.push(h);
        }
        let mut basic = MeshBasicMaterial::default();
        basic.properties.opacity = 0.5;
        basic.properties.transparent = true;
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10000., 10000., 1, 1)?),
            Arc::new(Material::Basic(basic)),
        )));
        let n = s.get_mut(plane)?;
        n.position.y = 100.;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        // Every face the buttons can select is fetched once; each parses on first use.
        let mut fonts = vec![];
        for name in FONTS {
            for weight in ["regular", "bold"] {
                fonts.push(fetch(&format!("{ASSETS}/fonts/{name}_{weight}.typeface.json")).await?);
            }
        }
        s.look_at(c, Vector3::new(0., 150., 0.))?;
        self.text = Some(Text {
            parsed: fonts.iter().map(|_| None).collect(),
            fonts,
            font: 1,
            bold: true,
            bevel: true,
            text: "three.js".into(),
            first_letter: true,
            group,
            meshes: [meshes[0], meshes[1]],
            light,
            target_rotation: 0.,
            rotation: 0.,
            drag: None,
            dirty: true,
            color: None,
        });
        Ok(())
    }
    /// refreshText(): a new TextGeometry for both meshes, or none for empty text.
    fn refresh_text(&mut self, s: &mut Scene) -> Result<()> {
        let k = self.text.as_mut().ok_or(Error::Invalid("text"))?;
        k.dirty = false;
        let face = (k.font % FONTS.len()) * 2 + usize::from(k.bold);
        if k.parsed[face].is_none() {
            k.parsed[face] = Some(Font::parse(&k.fonts[face])?);
        }
        let font = k.parsed[face].as_ref().ok_or(Error::Invalid("font"))?;
        let empty = k.text.is_empty();
        let g = if empty {
            BufferGeometry::default()
        } else {
            text_geometry(
                font,
                &k.text,
                70.,
                &Extrude {
                    curve_segments: 4,
                    steps: 1,
                    depth: 20.,
                    bevel: k.bevel.then_some((2., 1.5, 3)),
                },
            )?
        };
        let (min, max) = bounds(&f32s(&g, "position"));
        let offset = -0.5 * (max.x - min.x);
        let g = Arc::new(g);
        for (i, &h) in k.meshes.iter().enumerate() {
            set_geometry(s, h, &g)?;
            let n = s.get_mut(h)?;
            n.visible = !empty && g.draw_count() > 0;
            if i == 0 {
                n.position = Vector3::new(offset, 30., 0.);
                n.quaternion = euler(0., PI * 2., 0.);
            } else {
                n.position = Vector3::new(offset, -30., 20.);
                n.quaternion = euler(PI, PI * 2., 0.);
            }
        }
        Ok(())
    }
    async fn tessellation_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        // The raw output clears with the background's display value, as WebGL's sRGB clear.
        s.background = Color::linear(5. / 255., 5. / 255., 5. / 255.);
        let font =
            Font::parse(&fetch(&format!("{ASSETS}/fonts/helvetiker_bold.typeface.json")).await?)?;
        let g = text_geometry(
            &font,
            "THREE.JS",
            40.,
            &Extrude {
                curve_segments: 3,
                steps: 1,
                depth: 5.,
                bevel: Some((2., 1., 3)),
            },
        )?;
        let mut positions = f32s(&g, "position");
        center(&mut positions);
        // center()'s applyMatrix4 renormalizes the normals.
        let normals: Vec<f32> = f32s(&g, "normal")
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|n| {
                Vector3::new(n[0] as f64, n[1] as f64, n[2] as f64)
                    .normalize_or_zero()
                    .to_array()
                    .map(|v| v as f32)
            })
            .collect();
        let (positions, normals) = tessellate(&positions, &normals, 8., 6);
        let faces = positions.len() / 9;
        let (mut colors, mut displacement) =
            (Vec::with_capacity(faces * 9), Vec::with_capacity(faces * 6));
        for _ in 0..faces {
            let h = 0.2 * random(&mut self.seed);
            let sat = 0.5 + 0.5 * random(&mut self.seed);
            let l = 0.5 + 0.5 * random(&mut self.seed);
            let c = hsl(h, sat, l).0.to_array().map(|v| v as f32);
            let d = (10. * (0.5 - random(&mut self.seed))) as f32;
            for _ in 0..3 {
                colors.extend(c);
                displacement.extend([d, 0.]);
            }
        }
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(positions)?);
        g.set_attribute("normal", vec3s(normals)?);
        g.set_attribute("color", vec3s(colors)?);
        // The per-face displacement rides in uv.x: resident, uploaded once.
        g.set_attribute(
            "uv",
            Attribute::F32(BufferAttribute::new(displacement, 2, false)?),
        );
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let p=position+surface.local_normal*u.custom[0].x*surface.uv.x;out.clip=u.projection*u.view*u.model*vec4(p,1.0);return out;}";
        let shade = call(
            "tessellated_color",
            "fn tessellated_color(a:f32)->vec4<f32>{let d=max(dot(fragment_surface.local_normal,normalize(vec3(1.0))),0.0);return vec4((d+a)*fragment_surface.color.rgb,1.0);}",
            &[Type::Float],
            Type::Vec4,
            &[float(0.4)],
        )?;
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(shade).wgsl(0)?,
            &[],
            &[],
            projection,
        )
        .await?;
        // customColor is the color attribute, read as the vColor varying.
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.properties.vertex_colors = true;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Shader(m)),
        )));
        self.tessellated = Some(mesh);
        let (w, h, _) = viewport_css();
        self.trackball = Some(Trackball::new(s, c, Vector2::new(w, h))?);
        Ok(())
    }
    async fn lines_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::linear(5. / 255., 5. / 255., 5. / 255.);
        let font =
            Font::parse(&fetch(&format!("{ASSETS}/fonts/helvetiker_bold.typeface.json")).await?)?;
        let (mut positions, _) = extrude(
            &font.shapes("three.js", 50.),
            &Extrude {
                curve_segments: 10,
                steps: 1,
                depth: 15.,
                bevel: Some((5., 1.5, 10)),
            },
        );
        center(&mut positions);
        let count = positions.len() / 3;
        let colors: Vec<f32> = (0..count)
            .flat_map(|i| {
                hsl(i as f64 / count as f64, 0.5, 0.5)
                    .0
                    .to_array()
                    .map(|v| v as f32)
            })
            .collect();
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(positions)?);
        g.set_attribute("color", vec3s(colors)?);
        let displacement = vec![0f32; count * 3];
        let buffer = GpuBuffer::new(r, bytemuck::cast_slice(&displacement), BufferAccess::Read)?;
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let i=3u*tsl_vertex_index;let d=vec3(tsl_attribute_0[i],tsl_attribute_0[i+1u],tsl_attribute_0[i+2u]);out.clip=u.projection*u.view*u.model*vec4(position+u.custom[0].x*d,1.0);return out;}";
        // gl_FragColor = vec4( vColor × color, opacity ), with color white and opacity 0.3.
        let shade = call(
            "line_color",
            "fn line_color(c:vec4<f32>)->vec4<f32>{return vec4(fragment_surface.color.rgb*c.rgb,c.a);}",
            &[Type::Vec4],
            Type::Vec4,
            &[vec4(vec3(float(1.), float(1.), float(1.)), float(0.3))],
        )?;
        let source = NodeMaterial::new(shade).wgsl_with_storage(0, &[Type::Float])?;
        let program =
            ShaderProgram::with_projection(r, &source, &[&buffer], &[], projection).await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.properties.vertex_colors = true;
        m.properties.blending = Some(additive());
        m.properties.depth_test = false;
        m.properties.transparent = true;
        let line = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Shader(m)),
            segments: false,
        }));
        self.lines = Some(Lines {
            line,
            displacement,
            buffer,
        });
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, r: &Renderer, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        match self.id {
            248 => self.prepare_clipping(s, t)?,
            249 => self.prepare_splines(s, t)?,
            250 => {
                if self.text.as_ref().is_some_and(|k| k.dirty) {
                    self.refresh_text(s)?;
                }
                let k = self.text.as_mut().ok_or(Error::Invalid("text"))?;
                if let Some(c) = k.color.take()
                    && let NodeKind::Light(Light::Point { color, .. }) =
                        &mut s.get_mut(k.light)?.kind
                {
                    *color = c;
                }
                for _ in 0..steps {
                    k.rotation += (k.target_rotation - k.rotation) * 0.05;
                }
                s.get_mut(k.group)?.quaternion = Quaternion::from_rotation_y(k.rotation);
                s.look_at(c, Vector3::new(0., 150., 0.))?;
            }
            251 => {
                let mesh = self.tessellated.ok_or(Error::Invalid("tessellated"))?;
                set_uniform(s, mesh, 0, [(1. + (t * 0.5).sin()) as f32, 0., 0., 0.])?;
                let (w, h, _) = viewport_css();
                if let Some(tb) = &mut self.trackball {
                    tb.screen = Vector2::new(w, h);
                    for _ in 0..steps {
                        tb.update(s)?;
                    }
                }
            }
            _ => {
                let k = self.lines.as_mut().ok_or(Error::Invalid("lines"))?;
                s.get_mut(k.line)?.quaternion = euler(0.2, 0.25 * t, 0.);
                for _ in 0..steps {
                    for v in k.displacement.iter_mut() {
                        *v = (*v as f64 + 0.3 * (0.5 - random(&mut self.seed))) as f32;
                    }
                }
                // attributes.displacement.needsUpdate: the whole array every frame, as there.
                k.buffer
                    .write(r, 0, bytemuck::cast_slice(&k.displacement))?;
                set_uniform(s, k.line, 0, [(0.5 * t).sin() as f32, 0., 0., 0.])?;
            }
        }
        Ok(())
    }
    /// renderer.render( scene, animationView ? splineCamera : camera ).
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        _c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        match &self.splines {
            Some(k) if k.params[5] > 0.5 => {
                r.render(s, k.camera, out)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
    /// Absolute CSS-pixel pointer events: TrackballControls, or the text's drag rotation.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(k) = &mut self.text {
            match kind {
                10..=19 => k.drag = Some((x, k.target_rotation)),
                20..=29 => k.drag = None,
                _ => {
                    if let Some((x0, r0)) = k.drag {
                        k.target_rotation = r0 + (x - x0) * 0.02;
                    }
                }
            }
            return;
        }
        if let Some(t) = &mut self.trackball {
            match kind {
                10..=19 => t.down(kind - 10, x, y),
                20..=29 => t.state = Mode::None,
                _ => t.moved(x, y),
            }
        }
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
        if let Some(t) = &mut self.trackball {
            if wheel != 0. {
                t.zoom_start.y -= wheel * 0.00025;
            }
            return Ok(());
        }
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    /// Trackball keys, or the text example's keydown (codes) and keypress (100000 + which).
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(k) = &mut self.text {
            if !down {
                return;
            }
            if let Some(which) = code.checked_sub(100000) {
                if which != 8 {
                    k.text.extend(char::from_u32(which));
                    k.dirty = true;
                }
            } else {
                if k.first_letter {
                    k.first_letter = false;
                    k.text.clear();
                }
                if code == 8 {
                    k.text.pop();
                    k.dirty = true;
                }
            }
            return;
        }
        if let Some(t) = &mut self.trackball {
            if !down {
                t.key_state = Mode::None;
            } else if t.key_state == Mode::None {
                t.key_state = match code {
                    65 => Mode::Rotate,
                    83 => Mode::Zoom,
                    68 => Mode::Pan,
                    _ => Mode::None,
                };
            }
        }
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (248, 0..=3) => {
                let k = self.clipping.as_mut().ok_or(Error::Invalid("clipping"))?;
                let on = value > 0.5;
                match index {
                    0 => {
                        k.params[0] = on;
                        if !on {
                            k.params[2] = false;
                        }
                    }
                    2 => {
                        if k.params[0] {
                            k.params[2] = on;
                        }
                    }
                    _ => k.params[index] = on,
                }
            }
            (249, 0..=7) => {
                let k = self.splines.as_mut().ok_or(Error::Invalid("splines"))?;
                let valid = match index {
                    0 => (0. ..16.).contains(&value),
                    1 => (2. ..=10.).contains(&value),
                    2 => (50. ..=500.).contains(&value),
                    3 => (2. ..=12.).contains(&value),
                    _ => true,
                };
                if !valid {
                    return Err(Error::Invalid("splines parameter"));
                }
                k.params[index] = if index < 4 {
                    value.round() as f64
                } else {
                    f64::from(u8::from(value > 0.5))
                };
            }
            (250, 0..=3) => {
                let k = self.text.as_mut().ok_or(Error::Invalid("text"))?;
                match index {
                    0 => k.color = Some(hsl(random(&mut self.seed), 1., 0.5)),
                    1 => k.font += 1,
                    2 => k.bold = !k.bold,
                    _ => k.bevel = !k.bevel,
                }
                k.dirty |= index > 0;
            }
            _ => return Err(Error::Invalid("text/clipping parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
