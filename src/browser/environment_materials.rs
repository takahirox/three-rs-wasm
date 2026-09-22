//! Official environment, surface-detail and blend examples.
use super::gltf_viewer::{OrbitViewer, decode_texture_image, fetch, load_asset};
use crate::material::Texture;
use crate::tsl::Node;
use crate::{
    Error, Result,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    tsl::{self, surface::SurfaceNodes, *},
};
use std::sync::Arc;
mod blending;
mod reflections;
mod surfaces;
const ASSETS: &str = "/web/gallery/assets/environment-materials";
fn mesh(s: &mut Scene, g: Arc<BufferGeometry>, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(m))))
}
fn call(name: &str, source: &str, args: &[Type], out: Type, nodes: &[Node]) -> Result<Node> {
    Ok(WgslFn::new(name, source, args, out)?.call(nodes))
}
async fn image(path: &str, srgb: bool) -> Result<Texture> {
    let mut t = decode_texture_image(&fetch(&format!("{ASSETS}/{path}")).await?).await?;
    t.srgb = srgb;
    t.mipmap_filter = Some(Filter::Linear);
    Ok(t)
}
async fn cube(r: &Renderer, name: &str) -> Result<GpuTexture> {
    let names = if name == "Bridge2" {
        ["posx", "negx", "posy", "negy", "posz", "negz"]
    } else {
        ["px", "nx", "py", "ny", "pz", "nz"]
    };
    let mut faces = vec![];
    for face in names {
        faces.push(
            image(
                &format!(
                    "textures/cube/{name}/{face}.{}",
                    if name == "pisa" { "png" } else { "jpg" }
                ),
                true,
            )
            .await?,
        );
    }
    GpuTexture::from_cube_rgba(
        r,
        &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
    )
}
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    viewer: OrbitViewer,
    objects: Vec<Object3D>,
    sky: Option<Object3D>,
    params: [f32; 8],
    pointer: Vector2,
    orbit: Vector2,
    pan: Vector3,
    rotation: Vector3,
    material_rotation: Vector3,
    light: Option<Object3D>,
    ambient: Option<Object3D>,
    dirty: bool,
    bump_program: Option<Arc<ShaderProgram>>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let (fov, near, far, z) = match id {
            178 => (60., 0.01, 100., 3.),
            179 => (70., 0.1, 100., 2.5),
            180 => (45., 1., 10000., 1500.),
            181 => (27., 0.1, 100., 12.),
            _ => (70., 1., 1000., 600.),
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
        s.get_mut(c)?.position = Vector3::new(0., 0., z);
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            viewer: OrbitViewer::from_camera(Vector3::ZERO, z),
            objects: vec![],
            sky: None,
            params: [0.; 8],
            pointer: Vector2::ZERO,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            rotation: Vector3::ZERO,
            material_rotation: Vector3::ZERO,
            light: None,
            ambient: None,
            dirty: true,
            bump_program: None,
        };
        match id {
            178 | 179 => d.reflections(s, r).await?,
            180 | 181 => d.surfaces(s, c, r, aspect).await?,
            182 => d.blending(s, r).await?,
            _ => return Err(Error::Invalid("environment example")),
        }
        Ok(d)
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new(x / 100., y / 100.);
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        let count = match self.id {
            178 => 5,
            179 => 6,
            180 => 7,
            181 => 2,
            _ => 0,
        };
        if i >= count || !v.is_finite() {
            return Err(Error::Invalid("environment parameter"));
        }
        self.params[i] = match (self.id, i) {
            (178, 0) => v.clamp(0., 16777215.),
            (178, 1) | (179, 0) => v.round().clamp(0., 1.),
            (178, 2) | (178, 4) | (180, 0..=3) => v.clamp(0., 1.),
            (180, 4 | 5) => v.clamp(0., 3.),
            (180, 6) => v.clamp(-1., 1.),
            (181, 1) => v.clamp(0., 40.),
            _ => f32::from(v > 0.5),
        };
        self.dirty = true;
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        let delta = self.time - self.last;
        self.last = self.time;
        if self.id == 178 {
            let n = s.get_mut(c)?;
            n.position.x += (self.pointer.x - n.position.x) * 0.05;
            n.position.y += (-self.pointer.y - n.position.y) * 0.05;
            s.look_at(c, Vector3::ZERO)?;
        } else if (179..=181).contains(&self.id) {
            let damp = if self.id >= 180 { 0.05 } else { 1. };
            self.viewer.orbit_pixels(
                self.orbit.x * damp,
                self.orbit.y * damp,
                0.,
                1.,
                0.,
                f64::INFINITY,
            );
            self.viewer.pan_world(self.pan * damp);
            self.orbit *= 1. - damp;
            self.pan *= 1. - damp;
            self.viewer.update(s, c)?;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
                p.near = 0.1;
                p.far = 100.;
            }
        }
        match self.id {
            178 | 179 => self.update_reflections(s, c, delta)?,
            180 | 181 => self.update_surfaces(s, c)?,
            182 => self.update_blending(s)?,
            _ => {}
        }
        self.dirty = false;
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
        if !(179..=181).contains(&self.id) {
            return Ok(());
        }
        if pan {
            if self.id != 181 {
                self.pan += OrbitViewer::pan_delta(s, c, self.viewer.radius(), dx, dy, height)?;
            }
        } else {
            self.orbit += Vector2::new(dx, dy) / height.max(1.);
        }
        self.viewer.orbit_pixels(
            0.,
            0.,
            if self.id == 180 { 0. } else { w },
            height,
            if self.id == 179 {
                1.5
            } else if self.id == 180 {
                0.
            } else {
                8.
            },
            if self.id == 179 {
                6.
            } else if self.id == 180 {
                f64::INFINITY
            } else {
                50.
            },
        );
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, aspect: f64) -> Result<()> {
        if self.id == 180
            && let NodeKind::Camera(Camera::Orthographic(p)) = &mut s.get_mut(c)?.kind
        {
            p.left = -500. * aspect;
            p.right = 500. * aspect;
        }
        Ok(())
    }
}
