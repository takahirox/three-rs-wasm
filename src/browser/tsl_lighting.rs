//! Pinned r186 PMREM, baked lighting and depth/bloom postprocessing examples.
mod lightmap;
mod maps;
mod post;
use super::gltf_viewer::{OrbitViewer, decode_texture_image, fetch, load_asset};
use crate::{
    Error, Result,
    camera::*,
    environment::EnvironmentMap,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, surface::*, *},
};
use std::sync::Arc;
const ASSETS: &str = "/web/gallery/assets/tsl-lighting";
const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
fn mesh(s: &mut Scene, g: Arc<BufferGeometry>, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(m))))
}
pub(super) struct Demo {
    example: u32,
    time: f64,
    params: [f32; 8],
    viewer: OrbitViewer,
    objects: Vec<Object3D>,
    sky: Option<Object3D>,
    mixer: Option<crate::animation::AnimationMixer>,
    post: Option<post::Post>,
    focus: Vector3,
    focus_from: Vector3,
    focus_to: Vector3,
    focus_start: f64,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, example: u32, r: &Renderer) -> Result<Self> {
        let (position, target, fov, near, far) = match example {
            108 => (Vector3::new(0.0, 0.0, 8.0), Vector3::ZERO, 45.0, 0.25, 20.0),
            109 => (
                Vector3::new(-1.8, 0.6, 2.7),
                Vector3::ZERO,
                45.0,
                0.25,
                20.0,
            ),
            110 => (
                Vector3::new(700.0, 200.0, -500.0),
                Vector3::ZERO,
                40.0,
                1.0,
                10000.0,
            ),
            111 => (
                Vector3::new(-6.0, 5.0, 6.0),
                Vector3::new(0.0, 2.0, 0.0),
                60.0,
                0.1,
                100.0,
            ),
            112 => (
                Vector3::new(0.0, 0.5, -0.5),
                Vector3::new(0.0, 0.5, -0.51),
                45.0,
                0.1,
                100.0,
            ),
            _ => return Err(Error::Invalid("TSL lighting example")),
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
        s.get_mut(c)?.position = position;
        s.look_at(c, target)?;
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        let focus = Vector3::new(1.0, 1.75, -0.4);
        let mut d = Self {
            example,
            time: 0.0,
            params: [0.0; 8],
            viewer,
            objects: vec![],
            sky: None,
            mixer: None,
            post: None,
            focus,
            focus_from: focus,
            focus_to: focus,
            focus_start: 0.0,
        };
        s.background = Color::BLACK;
        match example {
            108 | 109 => d.maps(s, r).await?,
            110 => d.lightmap(s, r).await?,
            111 | 112 => d.post(s, r).await?,
            _ => unreachable!(),
        };
        Ok(d)
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        let count = match self.example {
            109 | 110 => 1,
            111 => 4,
            112 => 6,
            _ => 0,
        };
        if i >= count || !v.is_finite() {
            return Err(Error::Invalid("TSL lighting parameter"));
        }
        self.params[i] = v;
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
        }
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            (p.near, p.far) = match self.example {
                108 | 109 => (0.25, 20.0),
                110 => (1.0, 10000.0),
                _ => (0.1, 100.0),
            };
        }
        if let Some(sky) = self.sky {
            s.get_mut(sky)?.position = s.get(c)?.position;
        }
        if let Some(mixer) = &mut self.mixer {
            for action in &mut mixer.actions {
                action.time = self.time;
            }
            mixer.update(s, 0.0)?;
        }
        if matches!(self.example, 109 | 110) {
            for h in &self.objects {
                if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                    for m in &mut m.materials {
                        match Arc::make_mut(m) {
                            Material::Shader(m) => m.uniforms[0][0] = self.params[0],
                            m => m.properties_mut().vertex_uniforms[0][0] = self.params[0],
                        }
                    }
                }
            }
        }
        let t = ((self.time - self.focus_start) / 0.5).clamp(0.0, 1.0);
        let t = if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        };
        self.focus = self.focus_from.lerp(self.focus_to, t);
        if self.example == 112 {
            s.exposure = self.params[5] as f64;
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
        if let Some(post) = &mut self.post {
            post.render(r, s, c, out, &self.params, self.focus)?;
            return Ok(true);
        }
        Ok(false)
    }
    pub fn select(&mut self, s: &mut Scene, c: Object3D, x: f64, y: f64) -> Result<()> {
        if self.example != 111 {
            return Ok(());
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let mut ray = crate::raycast::Raycaster::default();
        ray.set_from_camera(Vector2::new(x, y), camera, world)?;
        if let Some(hit) = ray.intersect_objects(s, &self.objects, false)?.first() {
            self.focus_from = self.focus;
            self.focus_to = hit.point;
            self.focus_start = self.time;
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
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if self.example == 112 {
            if !pan {
                self.viewer.orbit_pixels(dx, dy, 0.0, height, 0.01, 0.01);
            }
            return Ok(());
        }
        if pan {
            return self.viewer.pan_pixels(s, c, dx, dy, height);
        }
        let (min, max) = if self.example <= 109 {
            (2.0, 10.0)
        } else {
            (0.0, f64::INFINITY)
        };
        self.viewer.orbit_pixels(
            dx,
            dy,
            if self.example == 110 { 0.0 } else { wheel },
            height,
            min,
            max,
        );
        if self.example == 110 {
            self.viewer.limit_pitch(
                std::f64::consts::PI * 0.05,
                std::f64::consts::FRAC_PI_2 - 1e-6,
            );
        }
        Ok(())
    }
}
