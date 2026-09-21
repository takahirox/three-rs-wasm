//! Pinned r186 material, contact-shadow and instanced-line examples.
mod contact;
mod lines;
mod materials;
use super::gltf_viewer::{OrbitViewer, decode_texture_image, fetch};
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, *},
};
use std::sync::Arc;
const ASSETS: &str = "/web/gallery/assets/tsl-primitives";
fn mesh(s: &mut Scene, g: Arc<BufferGeometry>, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(m))))
}
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.0
}
async fn texture(r: &Renderer, name: &str) -> Result<GpuTexture> {
    let path = match name {
        "uv_grid_opengl.jpg" => "/web/gallery/assets/uv-grid.jpg".to_owned(),
        "transition1.png" => "/web/gallery/assets/transition1.png".to_owned(),
        _ => format!("{ASSETS}/{name}"),
    };
    let mut t = decode_texture_image(&fetch(&path).await?).await?;
    t.srgb = false;
    t.mipmap_filter = Some(Filter::Linear);
    t.wrap_s = Wrapping::Repeat;
    t.wrap_t = Wrapping::Repeat;
    r.upload_texture(&Arc::new(t))
}
pub(super) struct Demo {
    example: u32,
    time: f64,
    params: [f32; 8],
    viewer: OrbitViewer,
    objects: Vec<Object3D>,
    rotations: Vec<Vector3>,
    lines: Option<lines::Lines>,
    contact: Option<contact::Contact>,
    orbit_delta: Vector2,
    pan_delta: Vector3,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, example: u32, r: &Renderer) -> Result<Self> {
        let position = match example {
            123 => Vector3::new(1000., 200., 0.),
            124 => Vector3::new(0., 0., 4.),
            125 => Vector3::new(0.5, 1., 2.),
            126 => Vector3::new(-40., 0., 60.),
            _ => Vector3::new(-50., 0., 50.),
        };
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: match example {
                123 => 45.,
                124 => 70.,
                125 => 50.,
                _ => 40.,
            },
            near: if (124..=125).contains(&example) {
                0.1
            } else {
                1.
            },
            far: match example {
                123 => 2000.,
                124 => 10.,
                125 => 100.,
                _ => 1000.,
            },
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = position;
        s.look_at(c, Vector3::ZERO)?;
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
        viewer.fixture(
            position.x.atan2(position.z),
            (position.y / position.length()).asin(),
            1.8,
        );
        let mut d = Self {
            example,
            time: 0.,
            params: [0.; 8],
            viewer,
            objects: vec![],
            rotations: vec![],
            lines: None,
            contact: None,
            orbit_delta: Vector2::ZERO,
            pan_delta: Vector3::ZERO,
        };
        match example {
            123 => d.materials(s, r).await?,
            124 => d.sandbox(s, r).await?,
            125 => d.contact(s, r).await?,
            126 | 127 => d.lines(s, r).await?,
            _ => return Err(Error::Invalid("TSL primitives example")),
        };
        Ok(d)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.lines.as_ref().map(|x| x.output())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= match self.example {
            125 => 6,
            126 => 8,
            127 => 5,
            _ => 0,
        } || !v.is_finite()
        {
            return Err(Error::Invalid("TSL primitives parameter"));
        }
        self.params[i] = v;
        if let Some(lines) = &mut self.lines
            && i == if self.example == 126 { 5 } else { 3 }
        {
            lines.thin_scale = v;
        }
        if self.example == 126 && i == 1 {
            self.params[2] = if v > 0.5 { 0.5 } else { 10. };
        }
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
        }
        if self.example == 123 {
            s.get_mut(c)?.position = Vector3::new(
                (self.time * 0.1).cos() * 1000.,
                200.,
                (self.time * 0.1).sin() * 1000.,
            );
            s.look_at(c, Vector3::ZERO)?;
        }
        if self.example >= 125 {
            if self.example == 126 {
                self.viewer.orbit_pixels(
                    self.orbit_delta.x * 0.05,
                    self.orbit_delta.y * 0.05,
                    0.,
                    1.,
                    10.,
                    500.,
                );
                self.viewer.pan_world(self.pan_delta * 0.05);
                self.orbit_delta *= 0.95;
                self.pan_delta *= 0.95;
            }
            self.viewer.update(s, c)?;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
                p.near = if self.example == 125 { 0.1 } else { 1. };
                p.far = if self.example == 125 { 100. } else { 1000. };
            }
        }
        for (i, &h) in self.objects.iter().enumerate() {
            let n = s.get_mut(h)?;
            if i < self.rotations.len() {
                let a = self.rotations[i];
                n.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    a.x + self.time * 0.6,
                    a.y + self.time * if self.example == 123 { 0.3 } else { 1.2 },
                    a.z,
                );
            }
            if let NodeKind::Mesh(m) = &mut n.kind {
                for material in &mut m.materials {
                    if let Material::Shader(sh) = Arc::make_mut(material) {
                        sh.uniforms[0][0] = self.time as f32;
                    }
                }
            }
        }
        if let Some(lines) = &mut self.lines {
            lines.update(s, &self.params)?;
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
        if let Some(lines) = &mut self.lines {
            lines.render(r, s, c, out)?;
            Ok(true)
        } else if let Some(contact) = &mut self.contact {
            contact.render(r, s, c, out, &self.params)?;
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
        if self.example == 126 {
            if pan {
                self.pan_delta +=
                    OrbitViewer::pan_delta(s, c, self.viewer.radius(), dx, dy, height)?;
            } else {
                self.orbit_delta += Vector2::new(dx, dy) / height.max(1.);
                self.viewer.orbit_pixels(0., 0., wheel, height, 10., 500.);
            }
        } else if pan {
            self.viewer.pan_pixels(s, c, dx, dy, height)?;
        } else {
            self.viewer.orbit_pixels(
                dx,
                dy,
                wheel,
                height,
                if self.example == 125 { 0.01 } else { 10. },
                if self.example == 125 { 100. } else { 500. },
            );
        }
        Ok(())
    }
}
