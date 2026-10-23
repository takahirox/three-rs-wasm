//! The convex, NURBS, text shapes, text stroke and shapes geometry examples,
//! their objects built at load as the pages build them ( ShapeGeometry,
//! ExtrudeGeometry, Path points, NURBS, ConvexGeometry, SVGLoader strokes ),
//! and a physical-material white furnace.
use super::gltf_viewer::{OrbitViewer, decode_texture_image, fetch};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::sync::Arc;
const ASSETS: &str = "assets/shapes";
struct Object {
    kind: String,
    position: [f64; 3],
    quaternion: [f64; 4],
    scale: [f64; 3],
    color: [f64; 3],
    opacity: f64,
    transparent: bool,
    double: bool,
    lambert: bool,
    phong: bool,
    size: f64,
    map: bool,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    viewer: OrbitViewer,
    root: Object3D,
    target_rotation: f64,
    rotation: f64,
    tint: bool,
    objects: Vec<Object3D>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let (fov, near, far, position) = match id {
            175 => (50., 1., 1000., Vector3::new(0., 150., 500.)),
            153 => (40., 1., 30., Vector3::new(0., 0., 18.)),
            154 => (40., 1., 1000., Vector3::new(15., 20., 30.)),
            155 => (50., 1., 2000., Vector3::new(0., 150., 750.)),
            156 => (45., 1., 10000., Vector3::new(0., -400., 600.)),
            _ => (45., 1., 10000., Vector3::new(0., -400., 1000.)),
        };
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = position;
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
        viewer.fixture(
            position.x.atan2(position.z),
            (position.y / position.length()).asin(),
            1.8,
        );
        if id != 155 && id != 175 {
            s.look_at(c, Vector3::ZERO)?;
        }
        let root = s.insert(NodeKind::Group);
        let mut d = Self {
            id,
            time: 0.,
            viewer,
            root,
            target_rotation: 0.,
            rotation: 0.,
            tint: false,
            objects: vec![],
        };
        if id == 153 {
            s.background = Color::from_hex(0xcccccc);
            let mut environment = Scene::new();
            environment.background = s.background;
            s.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_scene(
                r,
                &mut environment,
                256,
                0.,
            )?));
            let geometry = Arc::new(SphereGeometry::build(0.4, 32, 16)?);
            for x in 0..=10 {
                for y in 0..=10 {
                    let mut m = MeshPhysicalMaterial::default();
                    m.base.roughness = x as f64 / 10.;
                    m.base.metalness = y as f64 / 10.;
                    m.base.energy_conservation = true;
                    let h = s.insert(NodeKind::Mesh(Mesh::new(
                        geometry.clone(),
                        Arc::new(Material::Physical(m)),
                    )));
                    s.get_mut(h)?.position = Vector3::new(x as f64 - 5., 5. - y as f64, 0.);
                    d.objects.push(h);
                }
            }
        } else {
            let kind = match id {
                175 => "shapes",
                154 => "convex",
                155 => "nurbs",
                156 => "text_shapes",
                _ => "text_stroke",
            };
            let (data, geometries): (Vec<Object>, Vec<BufferGeometry>) =
                page(kind).await?.into_iter().unzip();
            let map = if id <= 155 || id == 175 {
                let path = if id == 154 {
                    format!("{ASSETS}/disc.png")
                } else {
                    "assets/uv-grid.jpg".to_owned()
                };
                let mut t = decode_texture_image(&fetch(&path).await?).await?;
                t.mipmap_filter = Some(Filter::Linear);
                if id == 175 {
                    t.wrap_s = Wrapping::Repeat;
                    t.wrap_t = Wrapping::Repeat;
                    t.repeat = Vector2::splat(0.008);
                }
                if id == 155 {
                    t.anisotropy = 16;
                    t.wrap_s = Wrapping::Repeat;
                    t.wrap_t = Wrapping::Repeat;
                }
                Some(Arc::new(t))
            } else {
                None
            };
            for (o, mut g) in data.into_iter().zip(geometries) {
                if o.kind == "points" {
                    g.delete_attribute("uv");
                }
                let mut m = match o.kind.as_str() {
                    "points" => Material::Points(PointsMaterial {
                        size: o.size,
                        ..Default::default()
                    }),
                    "line" => Material::Line(LineBasicMaterial::default()),
                    _ => {
                        if o.phong {
                            Material::Phong(MeshPhongMaterial::default())
                        } else if o.lambert {
                            Material::Lambert(MeshLambertMaterial::default())
                        } else {
                            Material::Basic(MeshBasicMaterial::default())
                        }
                    }
                };
                let p = m.properties_mut();
                p.color = Color(Vector3::from_array(o.color));
                p.opacity = o.opacity;
                p.transparent = o.transparent;
                p.side = if o.double { Side::Double } else { Side::Front };
                if o.map {
                    p.map = map.clone();
                }
                if o.kind == "points" && id != 175 {
                    p.alpha_test = 0.5;
                }
                let g = Arc::new(g);
                let m = Arc::new(m);
                let node = match o.kind.as_str() {
                    "points" => NodeKind::Points(Points {
                        geometry: g,
                        material: m,
                    }),
                    "line" => NodeKind::Line(Line {
                        geometry: g,
                        material: m,
                        segments: false,
                    }),
                    _ => NodeKind::Mesh(Mesh::new(g, m)),
                };
                let h = s.insert(node);
                s.add(root, h)?;
                let n = s.get_mut(h)?;
                n.position = Vector3::from_array(o.position);
                n.quaternion = Quaternion::from_array(o.quaternion);
                n.scale = Vector3::from_array(o.scale);
            }
            if id == 154 {
                s.insert(NodeKind::Light(Light::Ambient {
                    color: Color::from_hex(0x666666),
                    intensity: 1.,
                }));
                let light = s.insert(NodeKind::Light(Light::Point {
                    color: Color::WHITE,
                    intensity: 3.,
                    distance: 0.,
                    decay: 0.,
                }));
                s.add(c, light)?;
                let mut g = BufferGeometry::default();
                g.set_attribute(
                    "position",
                    Attribute::F32(BufferAttribute::new(
                        vec![
                            0., 0., 0., 20., 0., 0., 0., 0., 0., 0., 20., 0., 0., 0., 0., 0., 0.,
                            20.,
                        ],
                        3,
                        false,
                    )?),
                );
                g.set_attribute(
                    "color",
                    Attribute::F32(BufferAttribute::new(
                        vec![
                            1., 0., 0., 1., 0.6, 0., 0., 1., 0., 0.6, 1., 0., 0., 0., 1., 0., 0.6,
                            1.,
                        ],
                        3,
                        false,
                    )?),
                );
                let mut m = LineBasicMaterial::default();
                m.properties.vertex_colors = true;
                s.insert(NodeKind::Line(Line {
                    geometry: Arc::new(g),
                    material: Arc::new(Material::Line(m)),
                    segments: true,
                }));
            } else {
                s.background = Color::from_hex(0xf0f0f0);
            }
            if id == 175 {
                s.get_mut(root)?.position.y = 50.;
                let light = s.insert(NodeKind::Light(Light::Point {
                    color: Color::WHITE,
                    intensity: 2.5,
                    distance: 0.,
                    decay: 0.,
                }));
                s.add(c, light)?;
            }
            if id == 155 {
                s.get_mut(root)?.position.y = 50.;
                s.insert(NodeKind::Light(Light::Ambient {
                    color: Color::WHITE,
                    intensity: 1.,
                }));
                let h = s.insert(NodeKind::Light(Light::Directional {
                    color: Color::WHITE,
                    intensity: 3.,
                    target: Vector3::ZERO,
                }));
                s.get_mut(h)?.position = Vector3::ONE;
            }
        }
        Ok(d)
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if self.id != 153 || i != 0 || !v.is_finite() {
            return Err(Error::Invalid("furnace parameter"));
        }
        self.tint = v > 0.5;
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
        }
        if self.id == 154 || (156..=157).contains(&self.id) {
            self.viewer.update(s, c)?;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
                p.near = 1.;
                p.far = if self.id == 154 { 1000. } else { 10000. };
            }
        }
        if self.id == 154 {
            s.get_mut(self.root)?.quaternion = Quaternion::from_rotation_y(self.time * 0.3);
        }
        if self.id == 155 || self.id == 175 {
            self.rotation += (self.target_rotation - self.rotation) * 0.05;
            s.get_mut(self.root)?.quaternion = Quaternion::from_rotation_y(self.rotation);
        }
        for &h in &self.objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                let color = Color::from_hex(if self.tint { 0xccccff } else { 0xffffff });
                if m.materials[0].properties().color != color {
                    Arc::make_mut(&mut m.materials[0]).properties_mut().color = color;
                }
            }
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        w: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if self.id == 153 {
            return Ok(());
        }
        if self.id == 155 || self.id == 175 {
            if !pan && w == 0. {
                self.target_rotation += dx * 0.02;
            }
            return Ok(());
        }
        if pan {
            self.viewer.pan_pixels(s, c, dx, dy, height)?;
        } else {
            self.viewer.orbit_pixels(
                dx,
                dy,
                w,
                height,
                if self.id == 154 { 20. } else { 0. },
                if self.id == 154 { 50. } else { f64::INFINITY },
            );
            if self.id == 154 {
                self.viewer.limit_pitch(0., std::f64::consts::FRAC_PI_2);
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------- the pages' objects

/// An object as the page leaves it: its kind, world transform and material.
#[allow(clippy::too_many_arguments)]
fn object(
    kind: &str,
    position: [f64; 3],
    rotation: [f64; 3],
    scale: f64,
    color: u32,
    opacity: f64,
    double: bool,
    material: &str,
    size: f64,
    map: bool,
) -> Object {
    let q = Euler {
        angles: Vector3::from_array(rotation),
        order: EulerOrder::XYZ,
    }
    .quaternion();
    Object {
        kind: kind.into(),
        position,
        quaternion: q.to_array(),
        scale: [scale; 3],
        color: Color::from_hex(color).0.to_array(),
        opacity,
        transparent: opacity < 1.,
        double,
        lambert: material == "lambert",
        phong: material == "phong",
        size,
        map,
    }
}
fn points_geometry(points: &[Vector3]) -> Result<BufferGeometry> {
    let mut g = BufferGeometry::default();
    g.set_from_points(points)?;
    Ok(g)
}
fn points2(points: &[[f64; 2]]) -> Vec<Vector3> {
    points
        .iter()
        .map(|p| Vector3::new(p[0], p[1], 0.))
        .collect()
}
/// ExtrudeGeometry: positions, uvs, groups and computeVertexNormals().
fn extruded(shapes: &[crate::shapes::Shape], o: &crate::shapes::Extrude) -> Result<BufferGeometry> {
    let (p, uv, groups) = crate::shapes::extrude_uv(shapes, o);
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(p, 3, false)?),
    );
    g.set_attribute("uv", Attribute::F32(BufferAttribute::new(uv, 2, false)?));
    for (start, count, material) in groups {
        g.add_group(start, count, material);
    }
    g.compute_vertex_normals()?;
    Ok(g)
}

async fn page(kind: &str) -> Result<Vec<(Object, BufferGeometry)>> {
    match kind {
        "convex" => convex(),
        "nurbs" => nurbs(),
        "text_shapes" => text_shapes().await,
        "text_stroke" => text_stroke().await,
        _ => shapes(),
    }
}

/// webgl_geometry_convex: the merged dodecahedron's vertices as points and
/// their ConvexGeometry.
fn convex() -> Result<Vec<(Object, BufferGeometry)>> {
    let mut d = DodecahedronGeometry::build(10., 0)?;
    d.delete_attribute("normal");
    d.delete_attribute("uv");
    let d = crate::geometry_utils::merge_vertices(&d, 1e-4)?;
    let vertices = d.positions()?;
    Ok(vec![
        (
            object(
                "points", [0.; 3], [0.; 3], 1., 0x0080ff, 1., false, "points", 1., true,
            ),
            points_geometry(&vertices)?,
        ),
        (
            object(
                "mesh", [0.; 3], [0.; 3], 1., 0xffffff, 0.5, true, "lambert", 1., false,
            ),
            super::convex_hull::convex_geometry(&vertices)?,
        ),
    ])
}

/// webgl_geometry_nurbs: the curve ( with the gallery's seeded Math.random )
/// and its control polygon, the surface and the volume's five faces.
fn nurbs() -> Result<Vec<(Object, BufferGeometry)>> {
    use crate::nurbs::{NurbsCurve, NurbsSurface, NurbsVolume};
    let mut seed: u32 = 186;
    let mut random = || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(seed) / 4294967296.
    };
    let degree = 3;
    let mut knots = vec![0.; degree + 1];
    let mut control = vec![];
    for i in 0..20 {
        control.push([
            random() * 400. - 200.,
            random() * 400.,
            random() * 400. - 200.,
            1.,
        ]);
        knots.push(((i + 1) as f64 / (20 - degree) as f64).clamp(0., 1.));
    }
    let curve = NurbsCurve::new(degree, knots, control.clone(), None, None);
    let control_points: Vec<Vector3> = control
        .iter()
        .map(|p| Vector3::new(p[0], p[1], p[2]))
        .collect();
    let mut out = vec![
        (
            object(
                "line",
                [0., -100., 0.],
                [0.; 3],
                1.,
                0x333333,
                1.,
                false,
                "line",
                1.,
                false,
            ),
            points_geometry(&curve.points(200))?,
        ),
        (
            object(
                "line",
                [0., -100., 0.],
                [0.; 3],
                1.,
                0x333333,
                0.25,
                false,
                "line",
                1.,
                false,
            ),
            points_geometry(&control_points)?,
        ),
    ];
    let v = |x: f64, y: f64, z: f64, w: f64| [x, y, z, w];
    let surface = NurbsSurface {
        degrees: [2, 3],
        knots: [
            vec![0., 0., 0., 1., 1., 1.],
            vec![0., 0., 0., 0., 1., 1., 1., 1.],
        ],
        control_points: vec![
            vec![
                v(-200., -200., 100., 1.),
                v(-200., -100., -200., 1.),
                v(-200., 100., 250., 1.),
                v(-200., 200., -100., 1.),
            ],
            vec![
                v(0., -200., 0., 1.),
                v(0., -100., -100., 5.),
                v(0., 100., 150., 5.),
                v(0., 200., 0., 1.),
            ],
            vec![
                v(200., -200., -100., 1.),
                v(200., -100., 200., 1.),
                v(200., 100., -250., 1.),
                v(200., 200., 100., 1.),
            ],
        ],
    };
    out.push((
        object(
            "mesh",
            [-400., 100., 0.],
            [0.; 3],
            1.,
            0xffffff,
            1.,
            true,
            "lambert",
            1.,
            true,
        ),
        ParametricGeometry::build(|a, b| surface.point(a, b), 20, 20)?,
    ));
    let volume = NurbsVolume {
        degrees: [2, 3, 1],
        knots: [
            vec![0., 0., 0., 1., 1., 1.],
            vec![0., 0., 0., 0., 1., 1., 1., 1.],
            vec![0., 0., 1., 1.],
        ],
        control_points: vec![
            vec![
                vec![v(-200., -200., -200., 1.), v(-200., -200., 200., 1.)],
                vec![v(-200., -100., -200., 1.), v(-200., -100., 200., 1.)],
                vec![v(-200., 100., -200., 1.), v(-200., 100., 200., 1.)],
                vec![v(-200., 200., -200., 1.), v(-200., 200., 200., 1.)],
            ],
            vec![
                vec![v(0., -200., -200., 1.), v(0., -200., 200., 1.)],
                vec![v(0., -100., -200., 1.), v(0., -100., 200., 1.)],
                vec![v(0., 100., -200., 1.), v(0., 100., 200., 1.)],
                vec![v(0., 200., -200., 1.), v(0., 200., 200., 1.)],
            ],
            vec![
                vec![v(200., -200., -200., 1.), v(200., -200., 200., 1.)],
                vec![v(200., -100., 0., 1.), v(200., -100., 100., 1.)],
                vec![v(200., 100., 0., 1.), v(200., 100., 100., 1.)],
                vec![v(200., 200., 0., 1.), v(200., 200., 100., 1.)],
            ],
        ],
    };
    let faces: [&dyn Fn(f64, f64) -> Vector3; 5] = [
        &|a, b| volume.point(a, b, 0.),
        &|a, b| volume.point(a, b, 0.5),
        &|a, b| volume.point(a, b, 1.),
        &|a, b| volume.point(a, 1., b),
        &|a, b| volume.point(0., a, b),
    ];
    for f in faces {
        out.push((
            object(
                "mesh",
                [400., 100., 0.],
                [0.; 3],
                0.5,
                0xffffff,
                1.,
                true,
                "lambert",
                1.,
                true,
            ),
            ParametricGeometry::build(f, 20, 20)?,
        ));
    }
    Ok(out)
}

/// The shapes with their holes appended, as the text pages outline them.
fn outlines(shapes: &[crate::shapes::Shape]) -> Vec<Vec<[f64; 2]>> {
    let mut out: Vec<Vec<[f64; 2]>> = shapes.iter().map(|s| s.outline.points(12)).collect();
    for s in shapes {
        out.extend(s.holes.iter().map(|h| h.points(12)));
    }
    out
}
/// ShapeGeometry( shapes ) centered in x ( computeBoundingBox, translate ).
fn centered_text(shapes: &[crate::shapes::Shape]) -> Result<(BufferGeometry, f64)> {
    let mut g = crate::shapes::shape_geometry(shapes, 12, true)?;
    let b = g.compute_bounding_box()?;
    let x_mid = -0.5 * (b.max.x - b.min.x);
    g.translate(Vector3::new(x_mid, 0., 0.))?;
    Ok((g, x_mid))
}

/// webgl_geometry_text_shapes: the text's ShapeGeometry and its outlines as lines.
async fn text_shapes() -> Result<Vec<(Object, BufferGeometry)>> {
    let font =
        crate::shapes::Font::parse(&fetch("assets/fonts/helvetiker_regular.typeface.json").await?)?;
    let shapes = font.shapes("   Three.js\nSimple text.", 100.);
    let (g, x_mid) = centered_text(&shapes)?;
    let mut out = vec![(
        object(
            "mesh",
            [0., 0., -150.],
            [0.; 3],
            1.,
            0x006699,
            0.4,
            true,
            "basic",
            1.,
            false,
        ),
        g,
    )];
    for points in outlines(&shapes) {
        let mut g = points_geometry(&points2(&points))?;
        g.translate(Vector3::new(x_mid, 0., 0.))?;
        out.push((
            object(
                "line", [0.; 3], [0.; 3], 1., 0x006699, 1., true, "line", 1., false,
            ),
            g,
        ));
    }
    Ok(out)
}

/// webgl_geometry_text_stroke: generateStrokeText() for the English ( ltr ),
/// Hebrew ( rtl ) and Chinese ( tb ) messages.
async fn text_stroke() -> Result<Vec<(Object, BufferGeometry)>> {
    let font = crate::shapes::Font::parse(
        &fetch("assets/shapes/MPLUSRounded1c-Regular.typeface.json").await?,
    )?;
    let mut out = vec![];
    for (message, direction, x, y) in [
        ("   Three.js\nStroke text.", "ltr", -100., 0.),
        ("טקסט קו", "rtl", -100., -300.),
        ("文字描邊", "tb", 300., -300.),
    ] {
        let shapes = font.shapes_in(message, 80., direction);
        let (g, x_mid) = centered_text(&shapes)?;
        out.push((
            object(
                "mesh",
                [x, y, -150.],
                [0.; 3],
                1.,
                0x006699,
                0.4,
                true,
                "basic",
                1.,
                false,
            ),
            g,
        ));
        for points in outlines(&shapes) {
            let Some(mut g) = super::svg_loader::stroke(&points, 5.)? else {
                return Err(Error::Invalid("stroke of a degenerate outline"));
            };
            g.translate(Vector3::new(x_mid, 0., 0.))?;
            out.push((
                object(
                    "mesh",
                    [x, y, 0.],
                    [0.; 3],
                    1.,
                    0x006699,
                    1.,
                    true,
                    "basic",
                    1.,
                    false,
                ),
                g,
            ));
        }
    }
    Ok(out)
}

/// webgl_geometry_shapes: addShape() and addLineShape() for each shape.
fn shapes() -> Result<Vec<(Object, BufferGeometry)>> {
    use crate::shapes::{Extrude, Path, Shape};
    use std::f64::consts::PI;
    let shape = |p: Path| Shape {
        outline: p,
        holes: vec![],
    };
    let california: Vec<[f64; 2]> = [
        [610., 320.],
        [450., 300.],
        [392., 392.],
        [266., 438.],
        [190., 570.],
        [190., 600.],
        [160., 620.],
        [160., 650.],
        [180., 640.],
        [165., 680.],
        [150., 670.],
        [90., 737.],
        [80., 795.],
        [50., 835.],
        [64., 870.],
        [60., 945.],
        [300., 945.],
        [300., 743.],
        [600., 473.],
        [626., 425.],
        [600., 370.],
        [610., 320.],
    ]
    .map(|[x, y]| [x * 0.25, y * 0.25])
    .to_vec();
    let polyline = |first: [f64; 2], rest: &[[f64; 2]]| {
        let mut p = Path::default();
        p.move_to(first);
        for &q in rest {
            p.line_to(q);
        }
        p
    };
    let mut heart = Path::default();
    heart.move_to([25., 25.]);
    heart.cubic_to([25., 25.], [20., 0.], [0., 0.]);
    heart.cubic_to([-30., 0.], [-30., 35.], [-30., 35.]);
    heart.cubic_to([-30., 55.], [-10., 77.], [25., 95.]);
    heart.cubic_to([60., 77.], [80., 55.], [80., 35.]);
    heart.cubic_to([80., 35.], [80., 0.], [50., 0.]);
    heart.cubic_to([35., 0.], [25., 25.], [25., 25.]);
    // roundedRect( ctx, 0, 0, 50, 50, 20 )
    let mut rounded = Path::default();
    let (x, y, w, h, r) = (0., 0., 50., 50., 20.);
    rounded.move_to([x, y + r]);
    rounded.line_to([x, y + h - r]);
    rounded.quadratic_to([x, y + h], [x + r, y + h]);
    rounded.line_to([x + w - r, y + h]);
    rounded.quadratic_to([x + w, y + h], [x + w, y + h - r]);
    rounded.line_to([x + w, y + r]);
    rounded.quadratic_to([x + w, y], [x + w - r, y]);
    rounded.line_to([x + r, y]);
    rounded.quadratic_to([x, y], [x, y + r]);
    let mut track = polyline([40., 40.], &[[40., 160.]]);
    track.absarc([60., 160.], 20., PI, 0., true);
    track.line_to([80., 40.]);
    track.absarc([60., 40.], 20., 2. * PI, PI, true);
    let mut circle = Path::default();
    let cr = 40.;
    circle.move_to([0., cr]);
    circle.quadratic_to([cr, cr], [cr, 0.]);
    circle.quadratic_to([cr, -cr], [0., -cr]);
    circle.quadratic_to([-cr, -cr], [-cr, 0.]);
    circle.quadratic_to([-cr, cr], [0., cr]);
    let mut fish = Path::default();
    fish.move_to([0., 0.]);
    fish.quadratic_to([50., -80.], [90., -10.]);
    fish.quadratic_to([100., -10.], [115., -40.]);
    fish.quadratic_to([115., 0.], [115., 40.]);
    fish.quadratic_to([100., 10.], [90., 10.]);
    fish.quadratic_to([50., 80.], [0., 0.]);
    let mut arc = Path::default();
    arc.move_to([50., 10.]);
    arc.absarc([10., 10.], 40., 0., PI * 2., false);
    let mut arc_hole = Path::default();
    arc_hole.move_to([20., 10.]);
    arc_hole.absarc([10., 10.], 10., 0., PI * 2., true);
    let mut smiley = Path::default();
    smiley.move_to([80., 40.]);
    smiley.absarc([40., 40.], 40., 0., PI * 2., false);
    let mut eye1 = Path::default();
    eye1.move_to([35., 20.]);
    eye1.absellipse([25., 20.], [10., 10.], 0., PI * 2., true, 0.);
    let mut eye2 = Path::default();
    eye2.move_to([65., 20.]);
    eye2.absarc([55., 20.], 10., 0., PI * 2., true);
    let mut mouth = Path::default();
    mouth.move_to([20., 40.]);
    mouth.quadratic_to([40., 60.], [60., 40.]);
    mouth.cubic_to([70., 45.], [70., 50.], [60., 60.]);
    mouth.quadratic_to([40., 80.], [20., 60.]);
    mouth.quadratic_to([5., 50.], [20., 40.]);
    let mut spline = Path::default();
    spline.move_to([0., 0.]);
    spline.spline_thru(&[[70., 20.], [80., 90.], [-30., 70.], [0., 0.]]);
    let extrude = Extrude {
        curve_segments: 12,
        steps: 2,
        depth: 8.,
        bevel: Some((1., 1., 2)),
    };
    let mut out = vec![];
    let line_shape = |out: &mut Vec<(Object, BufferGeometry)>,
                      path: &Path,
                      color: u32,
                      [x, y, z]: [f64; 3],
                      rotation: [f64; 3]|
     -> Result<()> {
        let mut path = path.clone();
        path.auto_close = true;
        let points = points2(&path.points(12));
        let spaced = points2(&path.spaced_points(50));
        for (kind, z, p, size) in [
            ("line", z - 25., &points, 1.),
            ("line", z + 25., &spaced, 1.),
            ("points", z + 75., &points, 4.),
            ("points", z + 125., &spaced, 4.),
        ] {
            let material = if kind == "line" { "line" } else { "points" };
            out.push((
                object(
                    kind,
                    [x, y, z],
                    rotation,
                    1.,
                    color,
                    1.,
                    false,
                    material,
                    size,
                    false,
                ),
                points_geometry(p)?,
            ));
        }
        Ok(())
    };
    let list: Vec<(Shape, u32, [f64; 3], [f64; 3])> = vec![
        (
            shape(Path::from_points(&california)),
            0xf08000,
            [-300., -100., 0.],
            [0.; 3],
        ),
        (
            shape(polyline([80., 20.], &[[40., 80.], [120., 80.], [80., 20.]])),
            0x8080f0,
            [-180., 0., 0.],
            [0.; 3],
        ),
        (shape(rounded), 0x008000, [-150., 150., 0.], [0.; 3]),
        (shape(track), 0x008080, [200., -100., 0.], [0.; 3]),
        (
            shape(polyline(
                [0., 0.],
                &[[0., 80.], [80., 80.], [80., 0.], [0., 0.]],
            )),
            0x0040f0,
            [150., 100., 0.],
            [0.; 3],
        ),
        (shape(heart), 0xf00000, [60., 100., 0.], [0., 0., PI]),
        (shape(circle), 0x00f000, [120., 250., 0.], [0.; 3]),
        (shape(fish), 0x404040, [-60., 200., 0.], [0.; 3]),
        (
            Shape {
                outline: smiley,
                holes: vec![eye1, eye2, mouth],
            },
            0xf000f0,
            [-200., 250., 0.],
            [0., 0., PI],
        ),
        (
            Shape {
                outline: arc,
                holes: vec![arc_hole],
            },
            0x804000,
            [150., 0., 0.],
            [0.; 3],
        ),
        (shape(spline), 0x808080, [-50., -100., 0.], [0.; 3]),
    ];
    for (s, color, [x, y, z], rotation) in &list {
        let one = std::slice::from_ref(s);
        out.push((
            object(
                "mesh",
                [*x, *y, z - 175.],
                *rotation,
                1.,
                0xffffff,
                1.,
                true,
                "phong",
                1.,
                true,
            ),
            crate::shapes::shape_geometry(one, 12, false)?,
        ));
        out.push((
            object(
                "mesh",
                [*x, *y, z - 125.],
                *rotation,
                1.,
                *color,
                1.,
                true,
                "phong",
                1.,
                false,
            ),
            crate::shapes::shape_geometry(one, 12, false)?,
        ));
        out.push((
            object(
                "mesh",
                [*x, *y, z - 75.],
                *rotation,
                1.,
                *color,
                1.,
                false,
                "phong",
                1.,
                false,
            ),
            extruded(one, &extrude)?,
        ));
        line_shape(&mut out, &s.outline, *color, [*x, *y, *z], *rotation)?;
    }
    // The arc's hole and the smiley's holes as lines.
    let (arc, smiley) = (&list[9], &list[8]);
    line_shape(&mut out, &arc.0.holes[0], arc.1, arc.2, arc.3)?;
    for h in &smiley.0.holes {
        line_shape(&mut out, h, smiley.1, smiley.2, smiley.3)?;
    }
    Ok(out)
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
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
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

/// The program of three.js examples/webgpu_furnace_test.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_furnace_test(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            on_demand: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 153, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_geometry_convex.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_geometry_convex(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 154, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_geometry_nurbs.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_geometry_nurbs(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 155, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_geometry_text_shapes.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_geometry_text_shapes(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            on_demand: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 156, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_geometry_text_stroke.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_geometry_text_stroke(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            on_demand: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 157, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_geometry_shapes.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_geometry_shapes(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                5000.,
                Box::new(Demo::create(scene, camera, 175, renderer).await?),
            ))
        },
    )
    .await
}
