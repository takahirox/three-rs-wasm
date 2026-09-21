//! r186 framebuffer effects, soft particles and spatial upscaling.
mod backdrop;
mod refraction;
mod smoke;
mod upscale;
use super::gltf_viewer::{OrbitViewer, decode_texture_image, fetch, load_asset};
use crate::{
    Error, Result,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, surface::*, viewport as vp, *},
};
use std::sync::Arc;
const ASSETS: &str = "/web/gallery/assets/tsl-viewport";
const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
fn rgb(hex: u32) -> tsl::Node {
    let c = Color::from_hex(hex).0;
    vec3(float(c.x as f32), float(c.y as f32), float(c.z as f32))
}
fn mesh(s: &mut Scene, g: Arc<BufferGeometry>, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(m))))
}
fn time() -> tsl::Node {
    uniform(0, Type::Float)
}
fn osc(t: tsl::Node) -> tsl::Node {
    ((t + float(0.75)) * float(std::f32::consts::TAU)).sin() * float(0.5) + float(0.5)
}
pub(super) struct Demo {
    example: u32,
    time: f64,
    portal_time: f64,
    dragging: bool,
    orbit_delta: Vector2,
    pan_delta: Vector3,
    params: [f32; 8],
    viewer: OrbitViewer,
    objects: Vec<Object3D>,
    group: Option<Object3D>,
    mixer: Option<crate::animation::AnimationMixer>,
    materials: Vec<Arc<Material>>,
    post: Option<upscale::Upscale>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, example: u32, r: &Renderer) -> Result<Self> {
        let (fov, near, far, p, target) = match example {
            113 => (
                50.0,
                0.01,
                100.0,
                Vector3::new(1.0, 2.0, 3.0),
                Vector3::new(0.0, 1.0, 0.0),
            ),
            114 => (
                50.0,
                0.25,
                25.0,
                Vector3::new(3.0, 2.0, 3.0),
                Vector3::new(0.0, 1.0, 0.0),
            ),
            115 => (
                45.0,
                1.0,
                500.0,
                Vector3::new(0.0, 50.0, 160.0),
                Vector3::new(0.0, 50.0, 0.0),
            ),
            116 => (
                60.0,
                0.1,
                100.0,
                Vector3::new(6.0, 8.0, 8.0),
                Vector3::new(0.0, 4.0, 0.0),
            ),
            _ => (
                25.0,
                0.1,
                100.0,
                Vector3::new(-0.5, 0.0, 12.0),
                Vector3::new(-0.5, 0.0, 0.0),
            ),
        };
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.0,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = p;
        s.look_at(c, target)?;
        let offset = p - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        let mut d = Self {
            example,
            time: 0.0,
            portal_time: 0.0,
            dragging: false,
            orbit_delta: Vector2::ZERO,
            pan_delta: Vector3::ZERO,
            params: [1.0; 8],
            viewer,
            objects: vec![],
            group: None,
            mixer: None,
            materials: vec![],
            post: None,
        };
        s.background = Color::BLACK;
        match example {
            113 | 114 => d.backdrop(s, c, r).await?,
            115 => d.refraction(s, r).await?,
            116 => d.smoke(s, r).await?,
            117 => d.upscale(s, r).await?,
            _ => return Err(Error::Invalid("TSL viewport example")),
        };
        Ok(d)
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        let n = match self.example {
            114 | 116 => 3,
            117 => 2,
            _ => 0,
        };
        if i >= n || !v.is_finite() {
            return Err(Error::Invalid("viewport parameter"));
        }
        self.params[i] = v;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.portal_time = t;
    }
    pub fn dragging(&mut self, value: bool) {
        self.dragging = value;
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
            if !self.dragging {
                self.portal_time += delta;
            }
        }
        if matches!(self.example, 116 | 117) {
            self.viewer.orbit_pixels(
                self.orbit_delta.x * 0.05,
                self.orbit_delta.y * 0.05,
                0.0,
                1.0,
                0.0,
                f64::INFINITY,
            );
            self.viewer.pan_world(self.pan_delta * 0.05);
            self.orbit_delta *= 0.95;
            self.pan_delta *= 0.95;
        }
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            (p.near, p.far) = match self.example {
                113 => (0.01, 100.0),
                114 => (0.25, 25.0),
                115 => (1.0, 500.0),
                _ => (0.1, 100.0),
            };
        }
        if let Some(m) = &mut self.mixer {
            for a in &mut m.actions {
                a.time = self.time;
            }
            m.update(s, 0.0)?;
        }
        if self.example == 113
            && let Some(g) = self.group
        {
            s.get_mut(g)?.quaternion = Quaternion::from_rotation_y(self.portal_time * 0.5);
        }
        if self.example == 114 {
            let n = s.get_mut(self.group.unwrap())?;
            n.scale = Vector3::new(self.params[0] as f64, self.params[1] as f64, 1.0);
            if let NodeKind::Mesh(m) = &mut n.kind {
                m.materials[0] = self.materials[(self.params[2] as usize).min(3)].clone();
            }
        }
        if self.example == 115 {
            let n = s.get_mut(self.group.unwrap())?;
            let t = self.time;
            n.position = Vector3::new(
                t.cos() * 30.0,
                (t * 2.0).cos().abs() * 20.0 + 5.0,
                t.sin() * 30.0,
            );
            n.quaternion = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                0.0,
                std::f64::consts::FRAC_PI_2 - t,
                t * 8.0,
            );
        }
        for &h in &self.objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for material in &mut m.materials {
                    let m = Arc::make_mut(material);
                    let values = if let Material::Shader(m) = m {
                        &mut m.uniforms
                    } else {
                        &mut m.properties_mut().vertex_uniforms
                    };
                    values[0][0] = self.time as f32;
                    values[1] = self.params[..4].try_into().unwrap();
                }
            }
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if let Some(p) = &mut self.post {
            p.render(r, s, c, out, &self.params)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let damped = matches!(self.example, 116 | 117);
        if pan {
            if damped {
                self.pan_delta +=
                    OrbitViewer::pan_delta(s, c, self.viewer.radius(), dx, dy, height)?;
            } else {
                self.viewer.pan_pixels(s, c, dx, dy, height)?;
            }
        } else {
            let (min, max) = match self.example {
                116 => (6.0, 20.0),
                _ => (0.0, f64::INFINITY),
            };
            if damped {
                self.orbit_delta += Vector2::new(dx, dy) / height.max(1.0);
                self.viewer.orbit_pixels(0.0, 0.0, wheel, height, min, max);
            } else {
                self.viewer.orbit_pixels(dx, dy, wheel, height, min, max);
            }
        }
        Ok(())
    }
}
