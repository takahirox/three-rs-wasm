//! Pinned r186 TSL material and transparency scenes.
mod ground;
mod oit;
mod skinning;
mod sss;
mod toon;
use super::gltf_viewer::{OrbitViewer, decode_texture_image, fetch};
use crate::{
    Error, Result,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, surface::*, *},
};
use std::sync::Arc;
const ASSETS: &str = "/web/gallery/assets/tsl-materials";
fn mesh(s: &mut Scene, g: Arc<BufferGeometry>, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(m))))
}
fn geometries(bytes: &[u8]) -> Result<Vec<BufferGeometry>> {
    let (words, remainder) = bytes.as_chunks::<4>();
    if !remainder.is_empty() {
        return Err(Error::Invalid("geometry byte length"));
    }
    let mut words = words.iter();
    let mut next = || {
        words
            .next()
            .map(|b| u32::from_le_bytes(*b))
            .ok_or(Error::Invalid("truncated geometry"))
    };
    let count = next()?;
    let mut result = vec![];
    for _ in 0..count {
        let vertices = next()? as usize;
        let indices = next()? as usize;
        let mut g = BufferGeometry::default();
        for (name, size) in [("position", 3), ("normal", 3), ("uv", 2)] {
            let mut values = vec![];
            for _ in 0..vertices * size {
                values.push(f32::from_bits(next()?));
            }
            g.set_attribute(
                name,
                Attribute::F32(crate::attribute::BufferAttribute::new(values, size, false)?),
            );
        }
        if indices > 0 {
            let mut values = vec![];
            for _ in 0..indices {
                values.push(next()?);
            }
            g.set_index(Some(values));
        }
        result.push(g);
    }
    if words.next().is_some() {
        return Err(Error::Invalid("trailing geometry"));
    }
    Ok(result)
}
pub(super) struct Demo {
    example: u32,
    time: f64,
    params: [f32; 8],
    viewer: OrbitViewer,
    objects: Vec<Object3D>,
    animated: Option<Object3D>,
    mixer: Option<crate::animation::AnimationMixer>,
    blur: Option<skinning::Blur>,
    oit: Option<oit::Oit>,
    sky: Option<Object3D>,
    orbit_delta: Vector2,
    pan_delta: Vector3,
    dragging: bool,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, example: u32, r: &Renderer) -> Result<Self> {
        let position = match example {
            121 => Vector3::new(8.0, 4.0, 10.0),
            122 => Vector3::new(-20.0, 7.0, 20.0),
            120 => Vector3::new(1.0, 2.0, 3.0),
            118 => Vector3::new(0.0, 300.0, 1600.0),
            _ => Vector3::new(0.0, 400.0, 1400.0),
        };
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.0,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: if example == 120 {
                50.0
            } else if example == 121 {
                45.0
            } else {
                40.0
            },
            near: if example == 120 {
                0.01
            } else if example == 121 {
                0.1
            } else {
                1.0
            },
            far: match example {
                118 => 5000.0,
                119 => 2500.0,
                120 => 40.0,
                121 => 100.0,
                _ => 1000.0,
            },
            aspect,
            ..Default::default()
        }));
        let target = match example {
            120 => Vector3::Y,
            122 => Vector3::new(0.0, 2.0, 0.0),
            _ => Vector3::ZERO,
        };
        s.get_mut(c)?.position = position;
        s.look_at(c, target)?;
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        let mut d = Self {
            example,
            time: 0.0,
            params: [0.0; 8],
            viewer,
            objects: vec![],
            animated: None,
            mixer: None,
            blur: None,
            oit: None,
            sky: None,
            orbit_delta: Vector2::ZERO,
            pan_delta: Vector3::ZERO,
            dragging: false,
        };
        match example {
            118 => d.sss(s, r).await?,
            119 => d.toon(s, r).await?,
            120 => d.skinning(s, c, r).await?,
            121 => d.oit(s, r).await?,
            122 => d.ground(s, r).await?,
            _ => return Err(Error::Invalid("TSL material example")),
        };
        Ok(d)
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= match self.example {
            118 => 5,
            121 => 2,
            122 => 1,
            _ => 0,
        } || !v.is_finite()
        {
            return Err(Error::Invalid("TSL material parameter"));
        }
        self.params[i] = v;
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
        }
        if self.example == 121 {
            if animate && !self.dragging {
                self.orbit_delta.x += 1.0 / 1800.0;
            }
            self.viewer.orbit_pixels(
                self.orbit_delta.x * 0.05,
                self.orbit_delta.y * 0.05,
                0.0,
                1.0,
                5.0,
                25.0,
            );
            self.viewer.pan_world(self.pan_delta * 0.05);
            self.orbit_delta *= 0.95;
            self.pan_delta *= 0.95;
        }
        if self.example == 122 {
            self.viewer
                .limit_pitch(0.0, std::f64::consts::FRAC_PI_2 - 1e-6);
        }
        if self.example != 120 {
            self.viewer.update(s, c)?;
        }
        if let Some(sky) = self.sky {
            s.get_mut(sky)?.visible = self.params[0] > 0.5;
            s.background_environment = self.params[0] <= 0.5;
        }
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = match self.example {
                120 => 0.01,
                121 => 0.1,
                _ => 1.0,
            };
            p.far = match self.example {
                118 => 5000.0,
                119 => 2500.0,
                120 => 40.0,
                121 => 100.0,
                _ => 1000.0,
            };
        }
        if let Some(h) = self.animated {
            let n = s.get_mut(h)?;
            if self.example == 118 {
                n.quaternion = Quaternion::from_rotation_y(self.time / 5.0);
            } else {
                let t = self.time * 0.25;
                n.position = Vector3::new(
                    (t * 7.0).sin() * 300.0,
                    (t * 5.0).cos() * 400.0,
                    (t * 3.0).cos() * 300.0,
                );
            }
        }
        if let Some(mixer) = &mut self.mixer {
            for a in &mut mixer.actions {
                a.time = self.time;
            }
            mixer.update(s, 0.0)?;
        }
        for &h in &self.objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for m in &mut m.materials {
                    let p = Arc::make_mut(m).properties_mut();
                    p.vertex_uniforms[0] = self.params[..4].try_into().unwrap();
                    p.vertex_uniforms[1][0] = self.params[4];
                    if self.example == 120 {
                        p.vertex_uniforms[0][0] = self.time as f32;
                    }
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
        if let Some(oit) = &mut self.oit {
            oit.render(r, s, c, out, &self.params)?;
            return Ok(true);
        }
        if let Some(blur) = &mut self.blur {
            blur.render(r, s, c, out)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn dragging(&mut self, value: bool) {
        self.dragging = value;
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
        if self.example == 121 {
            if pan {
                self.pan_delta +=
                    OrbitViewer::pan_delta(s, c, self.viewer.radius(), dx, dy, height)?;
            } else {
                self.orbit_delta += Vector2::new(dx, dy) / height.max(1.0);
                self.viewer.orbit_pixels(0.0, 0.0, wheel, height, 5.0, 25.0);
            }
        } else if pan {
            if self.example != 122 {
                self.viewer.pan_pixels(s, c, dx, dy, height)?;
            }
        } else {
            let (min, max) = match self.example {
                118 => (500.0, 3000.0),
                122 => (20.0, 80.0),
                _ => (200.0, 2000.0),
            };
            self.viewer.orbit_pixels(dx, dy, wheel, height, min, max);
        }
        Ok(())
    }
}
