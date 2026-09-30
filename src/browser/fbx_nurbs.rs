//! webgl_loader_fbx_nurbs: FBXLoader's ASCII parser on nurbs.fbx, its five
//! NurbsCurve geometries (open, closed and periodic, orders 3 and 4) sampled
//! as NURBSCurve.getPoints( controlPoints × 12 ) into lines with the loader's
//! 0x3300ff LineBasicMaterial, the models' Lcl transforms under the layer
//! group, a GridHelper and OrbitControls. The curves are evaluated once at
//! load, as the loader does.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use super::interactive_scenes::grid_helper;
use crate::attribute::BufferAttribute;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;

/// A node of FBXLoader's TextParser tree: its attributes, its properties
/// (`Name: value`, `P:` and `C:` lines) and its child nodes.
#[derive(Default)]
struct Node {
    name: String,
    attrs: Vec<String>,
    props: Vec<(String, Vec<String>)>,
    children: Vec<Node>,
}
impl Node {
    fn prop(&self, name: &str) -> Option<&Vec<String>> {
        self.props.iter().find(|(n, _)| n == name).map(|(_, v)| v)
    }
    fn child(&self, name: &str) -> Option<&Node> {
        self.children.iter().find(|c| c.name == name)
    }
    /// An array node's `a:` values.
    fn array(&self, name: &str) -> Result<Vec<f64>> {
        self.child(name)
            .and_then(|c| c.prop("a"))
            .ok_or(Error::Invalid("fbx array"))?
            .iter()
            .map(|v| v.parse::<f64>().map_err(|_| Error::Invalid("fbx number")))
            .collect()
    }
}
/// Split a line's values at commas outside quotes, unquoting them.
fn values(text: &str) -> Vec<String> {
    let mut out = vec![];
    let mut current = String::new();
    let mut quoted = false;
    for ch in text.chars() {
        match ch {
            '"' => quoted = !quoted,
            ',' if !quoted => out.push(std::mem::take(&mut current).trim().to_string()),
            _ => current.push(ch),
        }
    }
    let last = current.trim();
    if !last.is_empty() || !out.is_empty() {
        out.push(last.to_string());
    }
    out
}
/// The ASCII FBX tree: `Name: attrs {` opens a node, `}` closes it, and any
/// other `Name: values` line is a property of the open node.
fn parse(text: &str) -> Result<Node> {
    let mut stack = vec![Node::default()];
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        if line == "}" {
            let node = stack.pop().ok_or(Error::Invalid("fbx braces"))?;
            stack
                .last_mut()
                .ok_or(Error::Invalid("fbx braces"))?
                .children
                .push(node);
            continue;
        }
        let (name, rest) = line.split_once(':').ok_or(Error::Invalid("fbx line"))?;
        let rest = rest.trim();
        if let Some(attrs) = rest.strip_suffix('{') {
            stack.push(Node {
                name: name.trim().to_string(),
                attrs: values(attrs),
                ..Default::default()
            });
        } else {
            stack
                .last_mut()
                .ok_or(Error::Invalid("fbx braces"))?
                .props
                .push((name.trim().to_string(), values(rest)));
        }
    }
    if stack.len() != 1 {
        return Err(Error::Invalid("fbx braces"));
    }
    stack.pop().ok_or(Error::Invalid("fbx tree"))
}
/// NURBSUtils.findSpan.
fn find_span(p: usize, u: f64, knots: &[f64]) -> usize {
    let n = knots.len() - p - 1;
    if u >= knots[n] {
        return n - 1;
    }
    if u <= knots[p] {
        return p;
    }
    let (mut low, mut high) = (p, n);
    let mut mid = (low + high) / 2;
    while u < knots[mid] || u >= knots[mid + 1] {
        if u < knots[mid] {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }
    mid
}
/// NURBSUtils.calcBasisFunctions.
fn basis(span: usize, u: f64, p: usize, knots: &[f64]) -> Vec<f64> {
    let mut n = vec![0.; p + 1];
    let (mut left, mut right) = (vec![0.; p + 1], vec![0.; p + 1]);
    n[0] = 1.;
    for j in 1..=p {
        left[j] = u - knots[span + 1 - j];
        right[j] = knots[span + j] - u;
        let mut saved = 0.;
        for r in 0..j {
            let rv = right[r + 1];
            let lv = left[j - r];
            let temp = n[r] / (rv + lv);
            n[r] = saved + rv * temp;
            saved = lv * temp;
        }
        n[j] = saved;
    }
    n
}
/// NURBSCurve( degree, knots, controlPoints, startKnot, endKnot ).getPoints( divisions ).
fn curve_points(
    degree: usize,
    knots: &[f64],
    points: &[Vector4],
    start: usize,
    end: usize,
    divisions: usize,
) -> Vec<f32> {
    let mut out = Vec::with_capacity((divisions + 1) * 3);
    for d in 0..=divisions {
        let t = d as f64 / divisions as f64;
        let u = knots[start] + t * (knots[end] - knots[start]);
        let span = find_span(degree, u, knots);
        let n = basis(span, u, degree, knots);
        let mut c = Vector4::ZERO;
        for (j, nj) in n.iter().enumerate() {
            let point = points[span - degree + j];
            let w = point.w * nj;
            c.x += point.x * w;
            c.y += point.y * w;
            c.z += point.z * w;
            c.w += point.w * nj;
        }
        if c.w != 1. {
            c /= c.w;
        }
        out.extend([c.x as f32, c.y as f32, c.z as f32]);
    }
    out
}
/// FBXLoader.parseNurbsGeometry.
fn nurbs_geometry(node: &Node) -> Result<BufferGeometry> {
    let order: usize = node
        .prop("Order")
        .and_then(|v| v.first())
        .and_then(|v| v.parse().ok())
        .ok_or(Error::Invalid("fbx nurbs order"))?;
    let degree = order - 1;
    let knots = node.array("KnotVector")?;
    let mut points: Vec<Vector4> = node
        .array("Points")?
        .chunks(4)
        .map(|p| Vector4::new(p[0], p[1], p[2], p[3]))
        .collect();
    let form = node
        .prop("Form")
        .and_then(|v| v.first())
        .map(String::as_str);
    let (mut start, mut end) = (0, knots.len() - 1);
    if form == Some("Closed") {
        points.push(points[0]);
    } else if form == Some("Periodic") {
        start = degree;
        end = knots.len() - 1 - start;
        for i in 0..degree {
            points.push(points[i]);
        }
    }
    if knots.len() < degree + 2 || points.len() + degree + 1 > knots.len() {
        return Err(Error::Invalid("fbx nurbs knots"));
    }
    let divisions = points.len() * 12;
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(
            curve_points(degree, &knots, &points, start, end, divisions),
            3,
            false,
        )?),
    );
    Ok(g)
}
pub(super) struct Demo {
    controls: Controls,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(2., 18., 28.);
        s.background = Color::BLACK;
        s.insert(NodeKind::Line(grid_helper(28., 28, 0x303030, 0x303030)?));
        let text = fetch("/web/gallery/assets/fbx/nurbs.fbx").await?;
        let tree = parse(&String::from_utf8_lossy(&text))?;
        let objects = tree.child("Objects").ok_or(Error::Invalid("fbx objects"))?;
        // Connections: child → parents.
        let mut parents: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut children: HashMap<i64, Vec<i64>> = HashMap::new();
        for (name, v) in &tree
            .child("Connections")
            .ok_or(Error::Invalid("fbx connections"))?
            .props
        {
            if name != "C" || v.len() < 3 {
                continue;
            }
            let (child, parent) = (
                v[1].parse::<i64>().map_err(|_| Error::Invalid("fbx id"))?,
                v[2].parse::<i64>().map_err(|_| Error::Invalid("fbx id"))?,
            );
            parents.entry(child).or_default().push(parent);
            children.entry(parent).or_default().push(child);
        }
        let id = |n: &Node| -> Result<i64> {
            n.attrs
                .first()
                .and_then(|v| v.parse().ok())
                .ok_or(Error::Invalid("fbx id"))
        };
        let mut geometries = HashMap::new();
        for node in objects.children.iter().filter(|n| n.name == "Geometry") {
            if node.attrs.get(2).map(String::as_str) == Some("NurbsCurve") {
                geometries.insert(id(node)?, Arc::new(nurbs_geometry(node)?));
            }
        }
        // parseModels: curves as lines, others as groups; then their
        // transforms and the hierarchy (roots under the loader's group).
        let mut color = LineBasicMaterial::default();
        color.properties.color = Color::from_hex(0x3300ff);
        let material = Arc::new(Material::Line(color));
        let mut models = vec![];
        for node in objects.children.iter().filter(|n| n.name == "Model") {
            let model_id = id(node)?;
            let h = if node.attrs.get(2).map(String::as_str) == Some("NurbsCurve") {
                let geometry = children
                    .get(&model_id)
                    .into_iter()
                    .flatten()
                    .filter_map(|c| geometries.get(c))
                    .next_back()
                    .cloned()
                    .unwrap_or_else(|| Arc::new(BufferGeometry::default()));
                s.insert(NodeKind::Line(Line {
                    geometry,
                    material: material.clone(),
                    segments: false,
                }))
            } else {
                s.insert(NodeKind::Group)
            };
            let n = s.get_mut(h)?;
            n.name = node
                .attrs
                .get(1)
                .and_then(|v| v.split("::").nth(1))
                .unwrap_or("")
                .to_string();
            if let Some(properties) = node.child("Properties70") {
                for (_, p) in properties.props.iter().filter(|(k, _)| k == "P") {
                    let vector = || -> Result<Vector3> {
                        let v: Vec<f64> = p[p.len().saturating_sub(3)..]
                            .iter()
                            .map(|v| v.parse::<f64>().map_err(|_| Error::Invalid("fbx number")))
                            .collect::<Result<_>>()?;
                        Ok(Vector3::new(v[0], v[1], v[2]))
                    };
                    match p.first().map(String::as_str) {
                        Some("Lcl Translation") => n.position = vector()?,
                        Some("Lcl Scaling") => n.scale = vector()?,
                        // The file's models are translated and scaled only.
                        Some("Lcl Rotation" | "PreRotation" | "PostRotation") => {
                            return Err(Error::Invalid("fbx rotation"));
                        }
                        _ => {}
                    }
                }
            }
            models.push((model_id, h));
        }
        let root = s.insert(NodeKind::Group);
        for &(model_id, h) in &models {
            let parent = parents
                .get(&model_id)
                .into_iter()
                .flatten()
                .find_map(|p| models.iter().find(|(m, _)| m == p).map(|&(_, h)| h));
            s.add(parent.unwrap_or(root), h)?;
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 12., 0.));
        controls.update(s, c)?;
        Ok(Self { controls })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
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
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("fbx nurbs parameter"))
    }
    pub fn seek(&mut self, _t: f64) {}
}
