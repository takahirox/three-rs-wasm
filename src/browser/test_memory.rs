//! webgl_test_memory: every frame builds a sphere with random segment counts
//! and a 256 × 256 canvas texture of a random color, draws it as a wireframe,
//! then removes and disposes all three. The port does the same: each frame's
//! geometry and texture are new, uploaded once, drawn and released, so GPU
//! residency stays flat while uploads recur exactly as in the original.
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::sync::Arc;

pub(super) struct Demo {
    seed: u32,
    mesh: Option<Object3D>,
    /// A still frame was requested: the next render is one animate() call.
    pending: bool,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 200.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::WHITE;
        let mut demo = Self {
            seed: 186,
            mesh: None,
            pending: false,
        };
        demo.frame(s)?;
        Ok(demo)
    }
    fn random(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed as f64 / 4294967296.
    }
    /// animate(): scene.remove and dispose last frame's mesh, then a new
    /// SphereGeometry( 50, random × 64, random × 32 ) with a CanvasTexture
    /// filled with rgb( ⌊random × 256⌋ … ) and a wireframe MeshBasicMaterial.
    fn frame(&mut self, s: &mut Scene) -> Result<()> {
        if let Some(mesh) = self.mesh.take() {
            s.dispose(mesh)?;
        }
        let width = (self.random() * 64.).floor().max(3.) as u32;
        let height = (self.random() * 32.).floor().max(2.) as u32;
        let geometry = Arc::new(SphereGeometry::build(50., width, height)?);
        let rgb = [0; 3].map(|_| (self.random() * 256.).floor() as u8);
        let pixels: Vec<u8> = (0..256 * 256)
            .flat_map(|_| [rgb[0], rgb[1], rgb[2], 255])
            .collect();
        let mut texture = Texture::from_rgba(256, 256, pixels, false)?;
        texture.mipmap_filter = Some(Filter::Linear);
        let mut m = MeshBasicMaterial::default();
        m.properties.map = Some(Arc::new(texture));
        m.properties.wireframe = true;
        self.mesh = Some(s.insert(NodeKind::Mesh(Mesh::new(
            geometry,
            Arc::new(Material::Basic(m)),
        ))));
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.frame(s)?;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.pending) {
            self.frame(s)?;
        }
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("test memory parameter"))
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
