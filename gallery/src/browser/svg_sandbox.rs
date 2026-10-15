//! svg_sandbox: the QR code ( Lambert, vertex colors ), two cubes, a
//! plane, a cylinder, a field of 100 random triangles turning 0.01 rad per
//! frame, 50 sprites, 50 SVG circles and the hexagon SVG file, under ambient
//! and directional light, written as SVG paths by SVGRenderer ( Rust,
//! src/svg.rs, low quality ) without color management, with damped
//! OrbitControls on the SVG element.
use super::controls_attributes::{Controls, camera_state};
use super::css3d_common::{Random, document, svg_overlay, window_size};
use super::gltf_viewer::fetch;
use super::svg_lines::raw_hex;
use crate::geometry::{Attribute, BoxGeometry, BufferGeometry, CylinderGeometry, PlaneGeometry};
use crate::svg::{SVGObject, SVGRenderer};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, material::*, math::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

pub(super) struct Demo {
    controls: Controls,
    svg: SVGRenderer,
    objects: Scene,
    group: Object3D,
    rotation: f64,
    /// The camera's world matrix as the last controls update left it.
    stale: Option<Matrix4>,
    time: f64,
    last: f64,
    size: (f64, f64),
}
fn f32s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
/// `new MeshBasicMaterial( { color, ... } )` without color management.
fn basic(color: Color, opacity: f64, side: Side) -> Arc<Material> {
    Arc::new(Material::Basic(MeshBasicMaterial {
        properties: MaterialProperties {
            color,
            opacity,
            transparent: opacity < 1.,
            side,
            ..Default::default()
        },
    }))
}
/// BufferGeometryLoader: the QR code's positions, normals, colors and group.
fn qr_geometry(bytes: &[u8]) -> Result<BufferGeometry> {
    #[derive(serde::Deserialize)]
    struct Array {
        array: Vec<f32>,
    }
    #[derive(serde::Deserialize)]
    struct GroupJson {
        start: usize,
        count: usize,
        #[serde(rename = "materialIndex")]
        material_index: usize,
    }
    #[derive(serde::Deserialize)]
    struct Attributes {
        position: Array,
        normal: Array,
        color: Array,
    }
    #[derive(serde::Deserialize)]
    struct Data {
        attributes: Attributes,
        groups: Vec<GroupJson>,
    }
    #[derive(serde::Deserialize)]
    struct Asset {
        data: Data,
    }
    let asset: Asset = serde_json::from_slice(bytes).map_err(|e| Error::Asset(e.to_string()))?;
    let mut g = BufferGeometry::default();
    g.set_attribute("position", f32s(asset.data.attributes.position.array)?);
    g.set_attribute("normal", f32s(asset.data.attributes.normal.array)?);
    g.set_attribute("color", f32s(asset.data.attributes.color.array)?);
    for group in asset.data.groups {
        g.add_group(group.start, group.count, group.material_index);
    }
    Ok(g)
}
/// OrbitControls.update(): its camera.lookAt() updates the world matrix
/// before turning the camera, which keeps the earlier rotation at the new
/// position until the render updates the camera ( SVGRenderer places its
/// SVGObjects before that update ).
fn orbit_update(controls: &mut Controls, s: &mut Scene, c: Object3D) -> Result<Matrix4> {
    let before = s.get(c)?.quaternion;
    controls.update(s, c)?;
    let n = s.get(c)?;
    Ok(Matrix4::from_scale_rotation_translation(
        n.scale, before, n.position,
    ))
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 75.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 500.);
        let mut o = Scene::new();
        o.background = raw_hex(f64::from(0xf0f0f0));
        let scene = o.insert(NodeKind::Group);
        let mut random = Random(186);
        let add = |o: &mut Scene, kind: NodeKind| -> Result<Object3D> {
            let h = o.insert(kind);
            o.add(scene, h)?;
            Ok(h)
        };
        let euler = |x: f64, y: f64, z: f64| {
            Euler {
                angles: Vector3::new(x, y, z),
                order: EulerOrder::XYZ,
            }
            .quaternion()
        };
        // CUBES
        let box_geometry = Arc::new(BoxGeometry::build(100., 100., 100.)?);
        let mesh = add(
            &mut o,
            NodeKind::Mesh(Mesh::new(
                box_geometry.clone(),
                basic(raw_hex(f64::from(0x0000ff)), 0.5, Side::Front),
            )),
        )?;
        let (x, y) = (random.next(), random.next());
        let n = o.get_mut(mesh)?;
        n.position.x = 500.;
        n.quaternion = euler(x, y, 0.);
        n.scale = Vector3::splat(2.);
        let color = raw_hex(random.next() * f64::from(0xffffff));
        let mesh = add(
            &mut o,
            NodeKind::Mesh(Mesh::new(box_geometry, basic(color, 1., Side::Front))),
        )?;
        let (x, y) = (random.next(), random.next());
        let n = o.get_mut(mesh)?;
        n.position = Vector3::new(500., 500., 0.);
        n.quaternion = euler(x, y, 0.);
        n.scale = Vector3::splat(2.);
        // PLANE
        let color = raw_hex(random.next() * f64::from(0xffffff));
        let mesh = add(
            &mut o,
            NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(100., 100., 1, 1)?),
                basic(color, 1., Side::Double),
            )),
        )?;
        let n = o.get_mut(mesh)?;
        n.position.y = -500.;
        n.scale = Vector3::splat(2.);
        // CYLINDER
        let color = raw_hex(random.next() * f64::from(0xffffff));
        let mesh = add(
            &mut o,
            NodeKind::Mesh(Mesh::new(
                Arc::new(CylinderGeometry::build(
                    20.,
                    100.,
                    200.,
                    10,
                    1,
                    false,
                    0.,
                    2. * PI,
                )?),
                basic(color, 1., Side::Front),
            )),
        )?;
        let n = o.get_mut(mesh)?;
        n.position.x = -500.;
        n.quaternion = euler(-PI / 2., 0., 0.);
        n.scale = Vector3::splat(2.);
        // POLYFIELD
        let (mut vertices, mut colors) = (vec![], vec![]);
        for _ in 0..100 {
            let mut point = || {
                [
                    random.next() * 1000. - 500.,
                    random.next() * 1000. - 500.,
                    random.next() * 1000. - 500.,
                ]
            };
            let v = point();
            let mut corner = || {
                [
                    random.next() * 100. - 50.,
                    random.next() * 100. - 50.,
                    random.next() * 100. - 50.,
                ]
            };
            let (v0, v1, v2) = (corner(), corner(), corner());
            let color = raw_hex(random.next() * f64::from(0xffffff));
            for c in [v0, v1, v2] {
                vertices.extend([
                    (c[0] + v[0]) as f32,
                    (c[1] + v[1]) as f32,
                    (c[2] + v[2]) as f32,
                ]);
                colors.extend(color.0.to_array().map(|c| c as f32));
            }
        }
        let mut field = BufferGeometry::default();
        field.set_attribute("position", f32s(vertices)?);
        field.set_attribute("color", f32s(colors)?);
        let field_material = Arc::new(Material::Basic(MeshBasicMaterial {
            properties: MaterialProperties {
                vertex_colors: true,
                side: Side::Double,
                ..Default::default()
            },
        }));
        let group = add(
            &mut o,
            NodeKind::Mesh(Mesh::new(Arc::new(field), field_material)),
        )?;
        o.get_mut(group)?.scale = Vector3::splat(2.);
        // SPRITES
        for _ in 0..50 {
            let mut material = SpriteMaterial::default();
            material.properties.color = raw_hex(random.next() * f64::from(0xffffff));
            let sprite = add(
                &mut o,
                NodeKind::Sprite(Sprite {
                    material: Arc::new(material),
                }),
            )?;
            let position = Vector3::new(
                random.next() * 1000. - 500.,
                random.next() * 1000. - 500.,
                random.next() * 1000. - 500.,
            );
            let n = o.get_mut(sprite)?;
            n.position = position;
            n.scale = Vector3::new(64., 64., 1.);
        }
        // CUSTOM
        let doc = document()?;
        let ns = Some("http://www.w3.org/2000/svg");
        let node = doc
            .create_element_ns(ns, "circle")
            .map_err(|_| Error::Invalid("svg circle"))?;
        let _ = node.set_attribute("stroke", "black");
        let _ = node.set_attribute("fill", "red");
        let _ = node.set_attribute("r", "40");
        for _ in 0..50 {
            let clone = node
                .clone_node()
                .map_err(|_| Error::Invalid("svg circle"))?
                .dyn_into::<web_sys::Element>()
                .map_err(|_| Error::Invalid("svg circle"))?;
            let object = add(&mut o, NodeKind::SVGObject(SVGObject::new(clone)))?;
            o.get_mut(object)?.position = Vector3::new(
                random.next() * 1000. - 500.,
                random.next() * 1000. - 500.,
                random.next() * 1000. - 500.,
            );
        }
        // LIGHTS
        add(
            &mut o,
            NodeKind::Light(Light::Ambient {
                color: raw_hex(f64::from(0x80ffff)),
                intensity: 1.,
            }),
        )?;
        let directional = add(
            &mut o,
            NodeKind::Light(Light::Directional {
                color: raw_hex(f64::from(0xffff00)),
                intensity: 1.,
                target: Vector3::ZERO,
            }),
        )?;
        o.get_mut(directional)?.position = Vector3::new(-1., 0.5, 0.);
        // The loaders' callbacks: the QR code, then the hexagon.
        let qr = qr_geometry(
            &fetch("/web/gallery/assets/svg-renderer/QRCode_buffergeometry.json").await?,
        )?;
        let mesh = add(
            &mut o,
            NodeKind::Mesh(Mesh::new(
                Arc::new(qr),
                Arc::new(Material::Lambert(MeshLambertMaterial {
                    properties: MaterialProperties {
                        vertex_colors: true,
                        ..Default::default()
                    },
                    emissive: Color::BLACK,
                    ..Default::default()
                })),
            )),
        )?;
        o.get_mut(mesh)?.scale = Vector3::splat(2.);
        let text = String::from_utf8_lossy(&fetch("/web/gallery/assets/svg/hexagon.svg").await?)
            .into_owned();
        let node = doc
            .create_element_ns(ns, "g")
            .map_err(|_| Error::Invalid("svg group"))?;
        let parsed = web_sys::DomParser::new()
            .map_err(|_| Error::Invalid("DOMParser"))?
            .parse_from_string(&text, web_sys::SupportedType::ImageSvgXml)
            .map_err(|_| Error::Invalid("hexagon.svg"))?;
        if let Some(root) = parsed.document_element() {
            let _ = node.append_child(&root);
        }
        let object = add(&mut o, NodeKind::SVGObject(SVGObject::new(node)))?;
        o.get_mut(object)?.position.x = 500.;
        let mut svg = svg_overlay("svg-renderer", false)?;
        svg.set_quality("low");
        // The constructor's update().
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        let stale = Some(orbit_update(&mut controls, s, c)?);
        Ok(Self {
            controls,
            svg,
            objects: o,
            group,
            rotation: 0.,
            stale,
            time: 0.,
            last: 0.,
            size: window_size(),
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// render(): the field turns 0.01 and the controls update once per 60 fps
    /// step, then the SVG render.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let size = window_size();
        if size != self.size {
            self.size = size;
            self.svg.set_size(size.0, size.1);
        }
        let steps = ((self.time - self.last) * 60.).round().max(0.) as usize;
        self.last = self.time;
        for _ in 0..steps {
            self.rotation += 0.01;
            self.stale = Some(orbit_update(&mut self.controls, s, c)?);
        }
        self.objects.get_mut(self.group)?.quaternion = Euler {
            angles: Vector3::new(self.rotation, 0., 0.),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        if let Some(stale) = self.stale.take() {
            s.get_mut(c)?.matrix_world = stale;
        }
        self.svg.render_from(&mut self.objects, s, c)
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
        // The pointer and wheel handlers end in update(), damping or not.
        self.stale = Some(orbit_update(&mut self.controls, s, c)?);
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("svg_sandbox parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
