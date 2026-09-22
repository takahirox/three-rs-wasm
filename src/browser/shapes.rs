//! Fixed official geometry examples and a physical-material white furnace.
use super::gltf_viewer::{OrbitViewer, decode_texture_image, fetch};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::sync::Arc;
const ASSETS: &str = "/web/gallery/assets/shapes";
#[derive(serde::Deserialize)]
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
    #[serde(default)]
    phong: bool,
    #[serde(default = "one")]
    size: f64,
    map: bool,
}
fn one() -> f64 {
    1.
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
            let data: Vec<Object> =
                serde_json::from_slice(&fetch(&format!("{ASSETS}/{kind}.json")).await?)
                    .map_err(|e| Error::Asset(e.to_string()))?;
            let geometries =
                super::tsl_materials::geometries(&fetch(&format!("{ASSETS}/{kind}.bin")).await?)?;
            if geometries.len() != data.len() {
                return Err(Error::Invalid("shape geometry metadata"));
            }
            let map = if id <= 155 || id == 175 {
                let path = if id == 154 {
                    format!("{ASSETS}/disc.png")
                } else {
                    "/web/gallery/assets/uv-grid.jpg".to_owned()
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
