//! misc_exporter_gcode: the page's Z-up print bed ( the rotated GridHelper,
//! an ambient and a directional light, OrbitControls around the Z-up
//! camera ) with the selected Lambert cube, cylinder, cone, sphere or torus
//! rested on the XY plane by its bounding box, as placeOnXYPlane does. The
//! printer, filament and slicer settings and the Polyslice slicing behind
//! Export G-code are not ported.
use super::controls_attributes::{Controls, camera_state};
use super::interactive_scenes::grid_helper;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

pub(super) struct Demo {
    controls: Controls,
    mesh: Option<Object3D>,
    pending: Option<usize>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(42., 42., 42.);
        n.up = Vector3::Z;
        s.background = Color::from_hex(0xa0a0a0);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.5,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 2.5,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0., 200., 100.);
        let grid = s.insert(NodeKind::Line(grid_helper(220., 10, 0x444444, 0x888888)?));
        s.get_mut(grid)?.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.up = Vector3::Z;
        controls.update(s, c)?;
        let mut demo = Self {
            controls,
            mesh: None,
            pending: None,
        };
        demo.add(s, 0)?;
        Ok(demo)
    }
    /// addCube() … addTorus(): clearScene(), the new mesh, placeOnXYPlane().
    fn add(&mut self, s: &mut Scene, kind: usize) -> Result<()> {
        if let Some(mesh) = self.mesh.take() {
            s.dispose(mesh)?;
        }
        let (geometry, upright) = match kind {
            0 => (BoxGeometry::build(10., 10., 10.)?, false),
            1 => (
                CylinderGeometry::build(5., 5., 10., 42, 1, false, 0., TAU)?,
                true,
            ),
            2 => (
                CylinderGeometry::build(0., 5., 10., 42, 1, false, 0., TAU)?,
                true,
            ),
            3 => (SphereGeometry::build(5., 42, 42)?, false),
            _ => (TorusGeometry::build(5., 2., 24, 100, TAU, 0., TAU)?, false),
        };
        let mut material = MeshLambertMaterial::default();
        material.properties.color = Color::from_hex(0x00cc00);
        // Box3.setFromObject: the geometry box's corners in the mesh's matrix.
        let quaternion = if upright {
            Quaternion::from_rotation_x(PI / 2.)
        } else {
            Quaternion::IDENTITY
        };
        let Some(Attribute::F32(p)) = geometry.get_attribute("position") else {
            return Err(Error::Invalid("gcode geometry"));
        };
        let (mut lo, mut hi) = (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        );
        for v in p.array().chunks(3) {
            let v = Vector3::new(f64::from(v[0]), f64::from(v[1]), f64::from(v[2]));
            lo = lo.min(v);
            hi = hi.max(v);
        }
        let mut min_z = f64::INFINITY;
        for k in 0..8 {
            let corner = Vector3::new(
                if k & 1 == 0 { lo.x } else { hi.x },
                if k & 2 == 0 { lo.y } else { hi.y },
                if k & 4 == 0 { lo.z } else { hi.z },
            );
            min_z = min_z.min((quaternion * corner).z);
        }
        let mesh = s.insert(NodeKind::Mesh(Mesh {
            geometry: Arc::new(geometry),
            materials: vec![Arc::new(Material::Lambert(material))],
        }));
        let n = s.get_mut(mesh)?;
        n.quaternion = quaternion;
        if min_z.is_finite() {
            n.position.z -= min_z;
        }
        self.mesh = Some(mesh);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if let Some(kind) = self.pending.take() {
            self.add(s, kind)?;
        }
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
    pub fn parameter(&mut self, index: usize, _value: f32) -> Result<()> {
        match index {
            // The printer, filament and slicer settings, and Export G-code: the
            // Polyslice slicer they parameterize is not ported.
            0..=4 | 10 => {}
            5..=9 => self.pending = Some(index - 5),
            _ => return Err(Error::Invalid("gcode parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
