//! The PDB molecules with their CSS2D labels, the helper objects, the mesh
//! simplifier, the AMF loader and the TIFF loader from the pinned WebGL examples.
pub(super) mod formats;
use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::{fetch, load_asset};
use super::trackball_sprites::{Mode, Trackball};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use formats::{Molecule, parse_amf, parse_pdb};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
/// The example's MOLECULES menu, in its order.
const MOLECULES: [&str; 17] = [
    "ethanol.pdb",
    "aspirin.pdb",
    "caffeine.pdb",
    "nicotine.pdb",
    "lsd.pdb",
    "cocaine.pdb",
    "cholesterol.pdb",
    "lycopene.pdb",
    "glucose.pdb",
    "Al2O3.pdb",
    "cubane.pdb",
    "cu.pdb",
    "caf2.pdb",
    "nacl.pdb",
    "ybco.pdb",
    "buckyball.pdb",
    "graphite.pdb",
];
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
fn floats(g: &BufferGeometry, name: &str) -> Vec<f32> {
    match g.attributes.get(name) {
        Some(Attribute::F32(a)) => a.array().to_vec(),
        _ => vec![],
    }
}
fn points(data: &[f32]) -> impl Iterator<Item = Vector3> + '_ {
    data.chunks(3)
        .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
}
fn line_material(hex: u32) -> Arc<Material> {
    let mut m = LineBasicMaterial::default();
    m.properties.color = Color::from_hex(hex);
    Arc::new(Material::Line(m))
}
fn segments(g: BufferGeometry, material: Arc<Material>) -> NodeKind {
    NodeKind::Line(Line {
        geometry: Arc::new(g),
        material,
        segments: true,
    })
}
/// `BoxHelper`: the eight corners of a box and its 24-index edge list.
fn box_helper(b: Box3) -> Result<(BufferGeometry, Vec<f32>)> {
    let (n, x) = (b.min, b.max);
    let corners = [
        [x.x, x.y, x.z],
        [n.x, x.y, x.z],
        [n.x, n.y, x.z],
        [x.x, n.y, x.z],
        [x.x, x.y, n.z],
        [n.x, x.y, n.z],
        [n.x, n.y, n.z],
        [x.x, n.y, n.z],
    ];
    let data: Vec<f32> = corners.iter().flatten().map(|&v| v as f32).collect();
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(data.clone())?);
    g.set_index(Some(vec![
        0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
    ]));
    Ok((g, data))
}
/// A JavaScript number key: -0 and 0 print the same.
fn key(v: f32) -> u32 {
    if v == 0. { 0 } else { v.to_bits() }
}
/// `WireframeGeometry` of indexed triangles: each edge once, either direction.
fn wireframe_geometry(position: &[f32], index: &[u32]) -> Vec<f32> {
    let mut edges = std::collections::HashSet::new();
    let mut out = vec![];
    let at = |i: u32| {
        let i = i as usize * 3;
        [position[i], position[i + 1], position[i + 2]]
    };
    for t in index.as_chunks::<3>().0 {
        for j in 0..3 {
            let (a, b) = (at(t[j]), at(t[(j + 1) % 3]));
            let (ka, kb) = (a.map(key), b.map(key));
            if edges.contains(&(ka, kb)) || edges.contains(&(kb, ka)) {
                continue;
            }
            edges.insert((ka, kb));
            edges.insert((kb, ka));
            out.extend(a);
            out.extend(b);
        }
    }
    out
}
/// `EdgesGeometry( geometry, 1 )`: rounded position hashes, unpaired edges and
/// edges whose faces meet at more than a degree, in the original's order.
fn edges_geometry(position: &[f32], index: &[u32]) -> Vec<f32> {
    let threshold = (PI / 180.).cos();
    let at = |i: u32| {
        let i = i as usize * 3;
        Vector3::new(
            position[i] as f64,
            position[i + 1] as f64,
            position[i + 2] as f64,
        )
    };
    // Math.round: halves toward +∞.
    let hash = |v: Vector3| [v.x, v.y, v.z].map(|c| (c * 1e4 + 0.5).floor() as i64);
    type Hash = ([i64; 3], [i64; 3]);
    let mut order: Vec<Hash> = vec![];
    let mut data: HashMap<Hash, Option<(u32, u32, Vector3)>> = HashMap::new();
    let mut out = vec![];
    for t in index.as_chunks::<3>().0 {
        let v = [at(t[0]), at(t[1]), at(t[2])];
        // Triangle.getNormal: ( c - b ) × ( a - b ), normalized.
        let n = (v[2] - v[1]).cross(v[0] - v[1]);
        let l2 = n.length_squared();
        let normal = if l2 > 0. {
            n * (1. / l2.sqrt())
        } else {
            Vector3::ZERO
        };
        let h = v.map(hash);
        if h[0] == h[1] || h[1] == h[2] || h[2] == h[0] {
            continue;
        }
        for j in 0..3 {
            let k = (j + 1) % 3;
            let (forward, reverse) = ((h[j], h[k]), (h[k], h[j]));
            if let Some(Some((_, _, other))) = data.get(&reverse) {
                if normal.dot(*other) <= threshold {
                    out.extend(
                        [v[j], v[k]]
                            .iter()
                            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32]),
                    );
                }
                data.insert(reverse, None);
            } else if let std::collections::hash_map::Entry::Vacant(entry) = data.entry(forward) {
                order.push(forward);
                entry.insert(Some((t[j], t[k], normal)));
            }
        }
    }
    for h in order {
        if let Some(Some((a, b, _))) = data.get(&h) {
            for p in [at(*a), at(*b)] {
                out.extend([p.x as f32, p.y as f32, p.z as f32]);
            }
        }
    }
    out
}
/// `PolarGridHelper( radius, sectors, rings, divisions, color1, color2 )`.
fn polar_grid(
    radius: f64,
    sectors: u32,
    rings: u32,
    divisions: u32,
    c1: u32,
    c2: u32,
) -> Result<BufferGeometry> {
    let (c1, c2) = (Color::from_hex(c1).0, Color::from_hex(c2).0);
    let (mut v, mut c) = (vec![], vec![]);
    let mut push = |p: [f64; 3], color: Vector3| {
        v.extend(p.map(|x| x as f32));
        c.extend(color.to_array().map(|x| x as f32));
    };
    if sectors > 1 {
        for i in 0..sectors {
            let a = i as f64 / sectors as f64 * (PI * 2.);
            let color = if i & 1 == 1 { c1 } else { c2 };
            push([0., 0., 0.], color);
            push([a.sin() * radius, 0., a.cos() * radius], color);
        }
    }
    for i in 0..rings {
        let color = if i & 1 == 1 { c1 } else { c2 };
        let r = radius - radius / rings as f64 * i as f64;
        for j in 0..divisions {
            for k in [j, j + 1] {
                let a = k as f64 / divisions as f64 * (PI * 2.);
                push([a.sin() * r, 0., a.cos() * r], color);
            }
        }
    }
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(v)?);
    g.set_attribute("color", vec3s(c)?);
    Ok(g)
}
/// One loaded molecule: its meshes under the rotating root and its label elements.
struct MoleculeNodes {
    group: Object3D,
    labels: Vec<(Object3D, web_sys::Element)>,
}
struct Pdb {
    root: Object3D,
    texts: Vec<String>,
    molecules: Vec<Option<MoleculeNodes>>,
    current: usize,
    container: web_sys::Element,
    ico: Arc<BufferGeometry>,
    cube: Arc<BufferGeometry>,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    controls: Option<Controls>,
    trackball: Option<Trackball>,
    pdb: Option<Pdb>,
    // Helpers
    light: Option<(Object3D, Object3D)>,
    // Simplifier
    head: Option<Arc<BufferGeometry>>,
    simplified: HashMap<u32, Object3D>,
    ratio: u32,
    shown: Option<u32>,
    head_scale: Vector3,
    molecule: Option<usize>,
    flat: Option<Arc<Material>>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let _ = r;
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            233 => (70., 1., 5000., Vector3::new(0., 0., 1000.)),
            234 => (70., 1., 1000., Vector3::new(0., 0., 400.)),
            235 => (40., 1., 1000., Vector3::new(0., 0., 15.)),
            236 => (35., 1., 500., Vector3::new(0., -9., 6.)),
            _ => (45., 0.01, 10., Vector3::new(0., 0., 4.)),
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
            controls: None,
            trackball: None,
            pdb: None,
            light: None,
            head: None,
            simplified: HashMap::new(),
            ratio: 125,
            shown: None,
            head_scale: Vector3::ONE,
            molecule: None,
            flat: None,
        };
        match id {
            233 => d.pdb_scene(s, c).await?,
            234 => d.helpers_scene(s).await?,
            235 => d.simplifier_scene(s, c).await?,
            236 => d.amf_scene(s, c).await?,
            _ => d.tiff_scene(s).await?,
        }
        Ok(d)
    }
    async fn pdb_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.background = Color::from_hex(0x050505);
        for (p, intensity) in [
            (Vector3::new(1., 1., 1.), 2.5),
            (Vector3::new(-1., -1., 1.), 1.5),
        ] {
            let light = s.insert(NodeKind::Light(Light::Directional {
                color: Color::WHITE,
                intensity,
                target: Vector3::ZERO,
            }));
            s.get_mut(light)?.position = p;
        }
        let root = s.insert(NodeKind::Group);
        let mut texts = vec![];
        for name in MOLECULES {
            texts.push(
                String::from_utf8_lossy(&fetch(&format!("{ASSETS}/pdb/{name}")).await?)
                    .into_owned(),
            );
        }
        // CSS2DRenderer's domElement: an absolute, clipped, pointer-transparent layer.
        // The original page is English, which selects the generic monospace font.
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or(Error::Invalid("document"))?;
        let body = document
            .query_selector("body")
            .ok()
            .flatten()
            .ok_or(Error::Invalid("body"))?;
        body.insert_adjacent_html("beforeend", "<div id=\"pdb-labels\" lang=\"en\"></div>")
            .map_err(|_| Error::Invalid("label layer"))?;
        let container = document
            .get_element_by_id("pdb-labels")
            .ok_or(Error::Invalid("label layer"))?;
        let mut pdb = Pdb {
            root,
            texts,
            molecules: (0..MOLECULES.len()).map(|_| None).collect(),
            current: 2,
            container,
            ico: Arc::new(IcosahedronGeometry::build(1., 3)?),
            cube: Arc::new(BoxGeometry::build(1., 1., 1.)?),
        };
        Self::show_molecule(&mut pdb, s, 2)?;
        self.pdb = Some(pdb);
        let (w, h, _) = viewport_css();
        let mut t = Trackball::new(s, c, Vector2::new(w, h))?;
        t.distance = (500., 2000.);
        self.trackball = Some(t);
        Ok(())
    }
    /// loadMolecule: each molecule is built once and kept; switching hides the rest.
    fn show_molecule(pdb: &mut Pdb, s: &mut Scene, index: usize) -> Result<()> {
        if pdb.molecules[index].is_none() {
            let Molecule { atoms, bonds } = parse_pdb(&pdb.texts[index])?;
            let group = s.insert(NodeKind::Group);
            s.add(pdb.root, group)?;
            // The Float32 geometries, centered on their bounding box by `translate`.
            let f = |p: [f64; 3]| p.map(|v| v as f32);
            let mut b = Box3::default();
            for a in &atoms {
                let p = f(a.position);
                b.expand_by_point(Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64));
            }
            let offset = -b.center();
            let moved = |p: [f64; 3]| {
                let p = f(p);
                Vector3::new(
                    (p[0] as f64 + offset.x) as f32 as f64,
                    (p[1] as f64 + offset.y) as f32 as f64,
                    (p[2] as f64 + offset.z) as f32 as f64,
                )
            };
            let mut html = String::new();
            let mut anchors = vec![];
            for a in &atoms {
                // Color.setRGB( r / 255, … , SRGBColorSpace ), stored as Float32.
                let color = Color::from_srgb(
                    a.color[0] as f64 / 255.,
                    a.color[1] as f64 / 255.,
                    a.color[2] as f64 / 255.,
                )
                .0
                .map(|v| v as f32 as f64);
                let mut m = MeshPhongMaterial::default();
                m.properties.color = Color::linear(color.x, color.y, color.z);
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    pdb.ico.clone(),
                    Arc::new(Material::Phong(m)),
                )));
                let n = s.get_mut(h)?;
                n.position = moved(a.position) * 75.;
                n.scale = Vector3::splat(25.);
                s.add(group, h)?;
                let label = s.insert(NodeKind::Group);
                s.get_mut(label)?.position = moved(a.position) * 75.;
                s.add(group, label)?;
                anchors.push(label);
                html.push_str(&format!(
                    "<div id=\"pdb-label-{index}-{}\" data-color=\"color:rgb({},{},{});\">{}</div>",
                    anchors.len() - 1,
                    a.color[0],
                    a.color[1],
                    a.color[2],
                    a.label
                ));
            }
            for (start, end) in &bonds {
                let (start, end) = (moved(*start) * 75., moved(*end) * 75.);
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    pdb.cube.clone(),
                    Arc::new(Material::Phong(MeshPhongMaterial::default())),
                )));
                let n = s.get_mut(h)?;
                n.position = start + (end - start) * 0.5;
                n.scale = Vector3::new(5., 5., start.distance(end));
                // lookAt before root.add: the bond has no parent yet.
                s.look_at(h, end)?;
                s.add(group, h)?;
            }
            pdb.container
                .insert_adjacent_html("beforeend", &html)
                .map_err(|_| Error::Invalid("labels"))?;
            let document = web_sys::window()
                .and_then(|w| w.document())
                .ok_or(Error::Invalid("document"))?;
            let labels = anchors
                .into_iter()
                .enumerate()
                .map(|(i, anchor)| {
                    document
                        .get_element_by_id(&format!("pdb-label-{index}-{i}"))
                        .map(|e| (anchor, e))
                        .ok_or(Error::Invalid("label element"))
                })
                .collect::<Result<_>>()?;
            pdb.molecules[index] = Some(MoleculeNodes { group, labels });
        }
        for (i, m) in pdb.molecules.iter().enumerate() {
            if let Some(m) = m {
                s.get_mut(m.group)?.visible = i == index;
            }
        }
        pdb.current = index;
        Ok(())
    }
    /// CSS2DRenderer.render: project each label, then order by camera distance.
    fn place_labels(&self, s: &mut Scene, c: Object3D) -> Result<()> {
        let Some(pdb) = &self.pdb else {
            return Ok(());
        };
        let (w, h, _) = viewport_css();
        let _ = pdb.container.set_attribute(
            "style",
            &format!(
                "position:absolute;top:0px;left:0px;pointer-events:none;overflow:hidden;width:{w}px;height:{h}px;font-family:Monospace;font-size:13px;line-height:24px;color:#fff"
            ),
        );
        s.update()?;
        let world = s.get(c)?.matrix_world;
        let view_projection = s.camera(c)?.0.projection_matrix()? * world.inverse();
        let camera_position = world.w_axis.truncate();
        let mut placed = vec![];
        for (i, m) in pdb.molecules.iter().enumerate() {
            let Some(m) = m else { continue };
            for (anchor, element) in &m.labels {
                if i != pdb.current {
                    // Removed CSS2DObjects leave the DOM.
                    let _ = element.set_attribute("style", "display:none");
                    continue;
                }
                let p = s.get(*anchor)?.matrix_world.w_axis.truncate();
                let v = view_projection.project_point3(p);
                let distance = p.distance_squared(camera_position);
                placed.push((distance, element, v));
            }
        }
        // Array.prototype.sort is stable: ties keep scene order.
        let mut order: Vec<usize> = (0..placed.len()).collect();
        order.sort_by(|&a, &b| placed[a].0.total_cmp(&placed[b].0));
        let z_max = placed.len();
        let mut z = vec![0; placed.len()];
        for (rank, &i) in order.iter().enumerate() {
            z[i] = z_max - rank;
        }
        for (i, (_, element, v)) in placed.iter().enumerate() {
            let visible = v.z >= -1. && v.z <= 1.;
            let color: String = element.get_attribute("data-color").unwrap_or_default();
            // CSS2DObject's element style and the example's `.label` rule.
            let style = format!(
                "position:absolute;user-select:none;{color}text-shadow:-1px 1px 1px rgb(0,0,0);margin-left:25px;font-size:20px;-webkit-font-smoothing:auto;{}transform:translate(-50%,-50%) translate({}px,{}px);z-index:{}",
                if visible { "" } else { "display:none;" },
                v.x * w / 2. + w / 2.,
                -v.y * h / 2. + h / 2.,
                z[i]
            );
            let _ = element.set_attribute("style", &style);
        }
        Ok(())
    }
    async fn helpers_scene(&mut self, s: &mut Scene) -> Result<()> {
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 1.,
            distance: 0.,
            decay: 2.,
        }));
        let light_position = Vector3::new(200., 100., 150.);
        s.get_mut(light)?.position = light_position;
        // PointLightHelper( light, 15 ): a wireframe sphere on the light's matrix.
        let sphere = SphereGeometry::build(15., 4, 2)?;
        let sphere_box = Box3::from_points(points(&floats(&sphere, "position")));
        let mut basic = MeshBasicMaterial::default();
        basic.properties.wireframe = true;
        basic.properties.fog = false;
        let helper = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(sphere),
            Arc::new(Material::Basic(basic)),
        )));
        s.get_mut(helper)?.position = light_position;
        self.light = Some((light, helper));
        let mut boxes = vec![];
        let grid = super::interactive_scenes::grid_helper(400., 40, 0x0000ff, 0x808080)?;
        let grid_offset = Matrix4::from_translation(Vector3::new(-150., -150., 0.));
        boxes.push(
            Box3::from_points(points(&floats(&grid.geometry, "position"))).transformed(grid_offset),
        );
        let h = s.insert(NodeKind::Line(grid));
        s.get_mut(h)?.position = Vector3::new(-150., -150., 0.);
        let polar = polar_grid(200., 16, 8, 64, 0x0000ff, 0x808080)?;
        let polar_offset = Matrix4::from_translation(Vector3::new(200., -150., 0.));
        boxes
            .push(Box3::from_points(points(&floats(&polar, "position"))).transformed(polar_offset));
        let mut m = LineBasicMaterial::default();
        m.properties.vertex_colors = true;
        let h = s.insert(segments(polar, Arc::new(Material::Line(m))));
        s.get_mut(h)?.position = Vector3::new(200., -150., 0.);
        // The glTF head: tangents, then the helpers of its world-space vertices.
        let (mesh_geometry, mesh_scale) = head().await?;
        let mut g = (*mesh_geometry).clone();
        g.compute_tangents()?;
        let group_matrix = Matrix4::from_scale(Vector3::splat(50.));
        let world = group_matrix * Matrix4::from_scale(mesh_scale);
        let normal_matrix = Matrix3::from_mat4(world).inverse().transpose();
        let position = floats(&g, "position");
        let normal = floats(&g, "normal");
        let tangent = match g.attributes.get("tangent") {
            Some(Attribute::F32(a)) => a.array().to_vec(),
            _ => return Err(Error::Invalid("tangents")),
        };
        let (mut vn, mut vt) = (vec![], vec![]);
        for (j, p) in points(&position).enumerate() {
            let v1 = world.transform_point3(p);
            let n = Vector3::new(
                normal[j * 3] as f64,
                normal[j * 3 + 1] as f64,
                normal[j * 3 + 2] as f64,
            );
            let v2 = (normal_matrix * n).normalize_or_zero() * 5. + v1;
            vn.extend(
                [v1, v2]
                    .iter()
                    .flat_map(|v| [v.x as f32, v.y as f32, v.z as f32]),
            );
            let t = Vector3::new(
                tangent[j * 4] as f64,
                tangent[j * 4 + 1] as f64,
                tangent[j * 4 + 2] as f64,
            );
            let v2 = world.transform_vector3(t).normalize_or_zero() * 5. + v1;
            vt.extend(
                [v1, v2]
                    .iter()
                    .flat_map(|v| [v.x as f32, v.y as f32, v.z as f32]),
            );
        }
        let mesh_box = Box3::from_points(points(&position));
        let index: Vec<u32> = g.index.clone().unwrap_or_default();
        let group = s.insert(NodeKind::Group);
        s.get_mut(group)?.scale = Vector3::splat(50.);
        let standard = MeshStandardMaterial {
            energy_conservation: true,
            ..head_material()
        };
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Standard(standard)),
        )));
        s.get_mut(mesh)?.scale = mesh_scale;
        s.add(group, mesh)?;
        for (data, hex) in [(vn, 0xff0000), (vt, 0x00ffff)] {
            boxes.push(Box3::from_points(points(&data)));
            let mut lines = BufferGeometry::default();
            lines.set_attribute("position", vec3s(data)?);
            s.insert(segments(lines, line_material(hex)));
        }
        let add_box = |s: &mut Scene, b: Box3, boxes: &mut Vec<Box3>| -> Result<()> {
            let (g, data) = box_helper(b)?;
            boxes.push(Box3::from_points(points(&data)));
            s.insert(segments(g, line_material(0xffff00)));
            Ok(())
        };
        add_box(s, mesh_box.transformed(world), &mut boxes)?;
        boxes.push(mesh_box.transformed(world));
        let mut group_box = mesh_box.transformed(world);
        for (data, x) in [
            (wireframe_geometry(&position, &index), 4.),
            (edges_geometry(&position, &index), -4.),
        ] {
            let b = Box3::from_points(points(&data));
            let local = group_matrix * Matrix4::from_translation(Vector3::new(x, 0., 0.));
            let mut m = LineBasicMaterial::default();
            m.properties.depth_test = false;
            m.properties.opacity = 0.25;
            m.properties.transparent = true;
            let mut lines = BufferGeometry::default();
            lines.set_attribute("position", vec3s(data)?);
            let h = s.insert(segments(lines, Arc::new(Material::Line(m))));
            s.get_mut(h)?.position = Vector3::new(x, 0., 0.);
            s.add(group, h)?;
            add_box(s, b.transformed(local), &mut boxes)?;
            boxes.push(b.transformed(local));
            group_box.union(b.transformed(local));
        }
        add_box(s, group_box, &mut boxes)?;
        // BoxHelper( scene ): every geometry in the scene at that moment.
        boxes.push(sphere_box.transformed(Matrix4::from_translation(light_position)));
        let mut scene_box = Box3::default();
        for b in &boxes {
            scene_box.union(*b);
        }
        add_box(s, scene_box, &mut boxes)?;
        Ok(())
    }
    async fn simplifier_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.6,
        }));
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 400.,
            distance: 0.,
            decay: 2.,
        }));
        s.add(c, light)?;
        let (g, scale) = head().await?;
        let standard = MeshStandardMaterial {
            energy_conservation: true,
            ..head_material()
        };
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            g.clone(),
            Arc::new(Material::Standard(standard.clone())),
        )));
        let n = s.get_mut(mesh)?;
        n.position = Vector3::new(-3., 0., 0.);
        n.quaternion = Quaternion::from_rotation_y(PI / 2.);
        n.scale = scale;
        let mut flat = standard;
        flat.properties.flat_shading = true;
        self.flat = Some(Arc::new(Material::Standard(flat)));
        self.head = Some(g);
        self.head_scale = scale;
        self.simplify(s)?;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::ZERO);
        self.controls = Some(controls);
        Ok(())
    }
    /// SimplifyModifier.modify( geometry, count ) through meshoptimizer 1.1, each
    /// ratio built once and kept.
    fn simplify(&mut self, s: &mut Scene) -> Result<()> {
        let scale = self.head_scale;
        let key = self.ratio;
        if !self.simplified.contains_key(&key) {
            let g = self.head.clone().ok_or(Error::Invalid("head"))?;
            let ratio_param = key as f64 / 1000.;
            let position = floats(&g, "position");
            let (normal, uv) = (floats(&g, "normal"), floats(&g, "uv"));
            let count = position.len() / 3;
            let index: Vec<u32> = g.index.clone().ok_or(Error::Invalid("head index"))?;
            let remove = (count as f64 * (1. - ratio_param)).floor();
            let ratio = (1. - remove / count as f64).clamp(0., 1.);
            let target = (((index.len() as f64 * ratio / 3.).floor() * 3.) as usize).max(3);
            let mut attributes = Vec::with_capacity(count * 5);
            for i in 0..count {
                attributes.extend_from_slice(&normal[i * 3..i * 3 + 3]);
                attributes.extend_from_slice(&uv[i * 2..i * 2 + 2]);
            }
            let mut destination = vec![0u32; index.len()];
            let (n, _) = optimesh::simplifier::simplify_with_attributes(
                &mut destination,
                &index,
                &optimesh::simplifier::VertexData {
                    positions: &position,
                    count,
                    stride: 12,
                },
                &optimesh::simplifier::Attributes {
                    data: &attributes,
                    stride: 20,
                    weights: &[0.25, 0.25, 0.25, 0.5, 0.5],
                    count: 5,
                },
                None,
                &optimesh::simplifier::SimplifyTarget {
                    target_index_count: target,
                    target_error: 1.,
                    options: 0,
                },
            );
            destination.truncate(n);
            // compactMesh: remap over the highest used index, rewritten in place.
            let vertices = destination.iter().max().map_or(0, |&m| m as usize + 1);
            let mut remap = vec![0u32; vertices];
            let unique =
                optimesh::vfetchoptimizer::optimize_vertex_fetch_remap(&mut remap, &destination);
            for i in &mut destination {
                *i = remap[*i as usize];
            }
            let mut out = BufferGeometry::default();
            for (name, size, data) in [
                ("position", 3, &position),
                ("normal", 3, &normal),
                ("uv", 2, &uv),
            ] {
                let mut values = vec![0f32; unique * size];
                for (i, &t) in remap.iter().enumerate() {
                    if t != u32::MAX {
                        values[t as usize * size..(t as usize + 1) * size]
                            .copy_from_slice(&data[i * size..(i + 1) * size]);
                    }
                }
                out.set_attribute(
                    name,
                    Attribute::F32(BufferAttribute::new(values, size, false)?),
                );
            }
            out.set_index(Some(destination));
            let flat = self.flat.clone().ok_or(Error::Invalid("flat material"))?;
            let h = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(out), flat)));
            let n = s.get_mut(h)?;
            n.position = Vector3::new(3., 0., 0.);
            n.quaternion = Quaternion::from_rotation_y(-PI / 2.);
            n.scale = scale;
            self.simplified.insert(key, h);
        }
        for (&k, &h) in &self.simplified {
            s.get_mut(h)?.visible = k == key;
        }
        self.shown = Some(key);
        Ok(())
    }
    async fn amf_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.background = Color::from_hex(0x999999);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x999999),
            intensity: 1.,
        }));
        s.get_mut(c)?.up = Vector3::Z;
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 250.,
            distance: 0.,
            decay: 2.,
        }));
        s.add(c, light)?;
        let grid = super::interactive_scenes::grid_helper(50., 50, 0xffffff, 0x555555)?;
        let h = s.insert(NodeKind::Line(grid));
        s.get_mut(h)?.quaternion = Quaternion::from_axis_angle(Vector3::X, 90. * (PI / 180.));
        let objects = parse_amf(&fetch(&format!("{ASSETS}/amf/rook.amf")).await?)?;
        let root = s.insert(NodeKind::Group);
        for volumes in objects {
            let object = s.insert(NodeKind::Group);
            s.add(root, object)?;
            for v in volumes {
                let mut g = BufferGeometry::default();
                g.set_attribute("position", vec3s(v.positions)?);
                if let Some(normals) = v.normals {
                    g.set_attribute("normal", vec3s(normals)?);
                }
                g.set_index(Some(v.index));
                let mut m = MeshPhongMaterial::default();
                m.properties.flat_shading = true;
                m.properties.color = Color::from_hex(0xaaaaff);
                for source in [&v.object_color, &v.material].into_iter().flatten() {
                    m.properties.color =
                        Color::linear(source.color[0], source.color[1], source.color[2]);
                    m.properties.transparent = source.opacity.is_some();
                    m.properties.opacity = source.opacity.unwrap_or(1.);
                }
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(g),
                    Arc::new(Material::Phong(m)),
                )));
                s.add(object, h)?;
            }
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.up = Vector3::Z;
        controls.set_target(Vector3::new(0., 0., 2.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        Ok(())
    }
    async fn tiff_scene(&mut self, s: &mut Scene) -> Result<()> {
        let plane = Arc::new(PlaneGeometry::build(1., 1., 1, 1)?);
        for (name, x) in [("uncompressed", -1.5), ("lzw", 0.), ("jpeg", 1.5)] {
            let (w, h, rgba) =
                formats::decode_tiff(&fetch(&format!("{ASSETS}/tiff/crate_{name}.tif")).await?)?;
            // DataTexture: flipY, linear filters, no mipmaps; sRGB as the example sets.
            let texture = Texture::from_rgba(w, h, rgba, true)?;
            let mut m = MeshBasicMaterial::default();
            m.properties.map = Some(Arc::new(texture));
            let n = s.insert(NodeKind::Mesh(Mesh::new(
                plane.clone(),
                Arc::new(Material::Basic(m)),
            )));
            s.get_mut(n)?.position = Vector3::new(x, 0., 0.);
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        match self.id {
            233 => {
                if let (Some(index), Some(pdb)) = (self.molecule.take(), self.pdb.as_mut()) {
                    Self::show_molecule(pdb, s, index)?;
                }
                let (w, h, _) = viewport_css();
                if let Some(tb) = &mut self.trackball {
                    tb.screen = Vector2::new(w, h);
                    for _ in 0..steps {
                        tb.update(s)?;
                    }
                }
                if let Some(pdb) = &self.pdb {
                    let time = t * 0.4;
                    s.get_mut(pdb.root)?.quaternion = Euler {
                        angles: Vector3::new(time, time * 0.7, 0.),
                        order: EulerOrder::XYZ,
                    }
                    .quaternion();
                }
                self.place_labels(s, c)?;
            }
            234 => {
                let time = -t * 0.3;
                let n = s.get_mut(c)?;
                n.position = Vector3::new(400. * time.cos(), 0., 400. * time.sin());
                s.look_at(c, Vector3::ZERO)?;
                if let Some((light, helper)) = self.light {
                    let p = Vector3::new(
                        (time * 1.7).sin() * 300.,
                        (time * 1.5).cos() * 400.,
                        (time * 1.3).cos() * 300.,
                    );
                    s.get_mut(light)?.position = p;
                    s.get_mut(helper)?.position = p;
                }
            }
            235 if self.shown != Some(self.ratio) => self.simplify(s)?,
            _ => {}
        }
        Ok(())
    }
    /// Absolute CSS-pixel pointer events for TrackballControls.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
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
        // The simplifier disables zoom and pan; the AMF example disables zoom.
        if wheel != 0. {
            return Ok(());
        } else if pan {
            if self.id == 235 {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn key(&mut self, code: u32, down: bool) {
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
            (233, 0) if (value as usize) < MOLECULES.len() => self.molecule = Some(value as usize),
            // ratio: 0.01 to 1 in steps of 0.01, from the default 0.125.
            (235, 0) if (0.01..=1.).contains(&value) => {
                self.ratio = (value as f64 * 1000.).round() as u32
            }
            _ => return Err(Error::Invalid("helpers/formats parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// `gltf.scene.children[ 0 ]` of LeePerrySmith.glb: its geometry and node scale.
pub(super) async fn head() -> Result<(Arc<BufferGeometry>, Vector3)> {
    let (asset, buffers, images) = load_asset("/web/models/LeePerrySmith.glb").await?;
    let mut temp = Scene::default();
    crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(&mut temp)?;
    temp.handles()
        .find_map(|h| {
            let n = temp.get(h).ok()?;
            match &n.kind {
                NodeKind::Mesh(m) => Some((m.geometry.clone(), n.scale)),
                _ => None,
            }
        })
        .ok_or(Error::Invalid("head geometry"))
}
/// The head's glTF material: baseColorFactor, metallicFactor 0, roughness 1.
fn head_material() -> MeshStandardMaterial {
    let mut m = MeshStandardMaterial {
        metalness: 0.,
        roughness: 1.,
        ..Default::default()
    };
    m.properties.color = Color::linear(0.665386974811554, 0.6653873324394226, 0.8227859139442444);
    m
}
