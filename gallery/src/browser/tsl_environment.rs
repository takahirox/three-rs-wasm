//! r186 environment nodes, alpha hashing and chromatic aberration.
mod box_projection;
mod chromatic;
mod hash;
mod maps;
use super::gltf_viewer::{OrbitViewer, fetch};
use crate::{
    Error, Result,
    attribute::BufferAttribute,
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
const ASSETS: &str = "/web/gallery/assets/tsl-environment";
const NEXT_ASSETS: &str = "/web/gallery/assets/tsl-next";
const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.0
}
fn mesh(s: &mut Scene, g: Arc<BufferGeometry>, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(m))))
}
fn standard(color: u32, roughness: f64, metalness: f64) -> MeshStandardMaterial {
    let mut m = MeshStandardMaterial {
        roughness,
        metalness,
        energy_conservation: true,
        ..Default::default()
    };
    m.properties.color = Color::from_hex(color);
    m
}
pub(super) struct Demo {
    example: u32,
    time: f64,
    clock: f64,
    auto_residual: f64,
    dragging: bool,
    params: [f32; 8],
    viewer: OrbitViewer,
    objects: Vec<Object3D>,
    sky: Option<Object3D>,
    groups: Vec<Object3D>,
    programs: Vec<Arc<crate::shader::ShaderProgram>>,
    ssaa: Option<crate::postprocessing::ssaa::SsaaPass>,
    ca: Option<chromatic::Chromatic>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, example: u32, r: &Renderer) -> Result<Self> {
        let (position, target, fov, near, far) = match example {
            103 => (
                Vector3::new(-1.8, 0.6, 2.7),
                Vector3::ZERO,
                45.0,
                0.25,
                20.0,
            ),
            104 => (
                Vector3::new(-3.6, 1.2, 5.4),
                Vector3::ZERO,
                45.0,
                0.25,
                20.0,
            ),
            105 => (
                Vector3::new(0.0, 200.0, -200.0),
                Vector3::new(0.0, -10.0, 0.0),
                45.0,
                0.1,
                1000.0,
            ),
            106 => (Vector3::splat(3.0), Vector3::ZERO, 60.0, 0.1, 100.0),
            107 => (
                Vector3::new(0.0, 15.0, 40.0),
                Vector3::new(0.0, 0.5, 0.0),
                45.0,
                0.1,
                200.0,
            ),
            _ => return Err(Error::Invalid("TSL environment example")),
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
        let mut d = Self {
            example,
            time: 0.0,
            clock: 0.0,
            auto_residual: 0.0,
            dragging: false,
            params: [0.0; 8],
            viewer,
            objects: vec![],
            sky: None,
            groups: vec![],
            programs: vec![],
            ssaa: None,
            ca: None,
        };
        s.background = Color::BLACK;
        match example {
            103 | 104 => d.maps(s, r).await?,
            105 => d.box_projection(s, r).await?,
            106 => d.hash(s, r).await?,
            107 => d.chromatic(s, r).await?,
            _ => unreachable!(),
        };
        Ok(d)
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn dragging(&mut self, v: bool) {
        self.dragging = v;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        let count = match self.example {
            103 => 0,
            104 => 8,
            105 => 2,
            106 => 3,
            107 => 7,
            _ => 0,
        };
        if i >= count || !v.is_finite() {
            return Err(Error::Invalid("TSL environment parameter"));
        }
        self.params[i] = v;
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
        }
        if self.example == 107 {
            if self.params[6] > 0.5 && !self.dragging {
                self.auto_residual -= (self.time - self.clock) / 600.0;
            }
            self.viewer
                .orbit_pixels(self.auto_residual * 0.1, 0.0, 0.0, 1.0, 0.0, f64::INFINITY);
            self.auto_residual *= 0.9;
        }
        self.clock = self.time;
        self.viewer.update(s, c)?;
        let data = match self.example {
            103 => {
                let mix =
                    (((self.time * 0.1 + 0.75) * std::f64::consts::TAU).sin() * 0.5 + 0.5) as f32;
                [[mix, 0.5, 0.0, 0.0], [0.0; 4]]
            }
            104 => [
                self.params[..4].try_into().unwrap(),
                self.params[4..8].try_into().unwrap(),
            ],
            105 => [[self.params[1], 0.0, 0.0, 0.0], [0.0; 4]],
            106 => [[self.params[0], self.params[1], 0.0, 0.0], [0.0; 4]],
            _ => [[0.0; 4]; 2],
        };
        for h in &self.objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                for mat in &mut m.materials {
                    let p = Arc::make_mut(mat).properties_mut();
                    p.vertex_uniforms[..2].copy_from_slice(&data);
                    if self.example == 106 {
                        p.transparent = self.params[1] < 0.5;
                        p.depth_write = !p.transparent;
                    }
                    if self.example == 105 {
                        p.vertex_program =
                            Some(self.programs[usize::from(self.params[0] > 0.5)].clone());
                    }
                }
            }
        }
        if let Some(h) = self.sky {
            s.get_mut(h)?.position = s.get(c)?.position;
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind
                && let Material::Shader(mat) = Arc::make_mut(&mut m.materials[0])
            {
                mat.uniforms[..2].copy_from_slice(&data);
            }
        }
        if self.example == 107 && self.params[5] > 0.5 {
            for g in &self.groups {
                s.get_mut(*g)?.quaternion = Quaternion::from_rotation_y(self.time * 0.5);
                let children = s.get(*g)?.children().to_vec();
                for (i, h) in children.into_iter().enumerate() {
                    s.get_mut(h)?.quaternion = Quaternion::from_euler(
                        glam::EulerRot::XYZ,
                        self.time * (1.0 + i as f64 * 0.1),
                        0.0,
                        self.time * (1.0 - i as f64 * 0.1),
                    );
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
        if let Some(pass) = &mut self.ssaa {
            pass.sample_level = self.params[2] as u32;
            pass.render(r, s, c, out)?;
            return Ok(true);
        }
        if let Some(pass) = &mut self.ca {
            pass.render(r, s, c, out, &self.params)?;
            return Ok(true);
        }
        Ok(false)
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
        if self.example == 106 {
            if !pan {
                self.viewer
                    .orbit_pixels(dx, dy, 0.0, height, 0.0, f64::INFINITY);
            }
            return Ok(());
        }
        if pan {
            return self.viewer.pan_pixels(s, c, dx, dy, height);
        }
        let (min, max) = match self.example {
            103 => (2.0, 10.0),
            104 => (2.0, 10.0),
            105 => (10.0, 400.0),
            _ => (0.0, f64::INFINITY),
        };
        if self.example == 107 && self.params[6] > 0.5 && wheel != 0.0 && !self.dragging {
            self.viewer
                .orbit_pixels(-1.0 / 36000.0, 0.0, 0.0, 1.0, min, max);
        }
        self.viewer.orbit_pixels(dx, dy, wheel, height, min, max);
        Ok(())
    }
}
