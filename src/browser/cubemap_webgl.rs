//! webgl_materials_cubemap_dynamic: a mirror sphere reflecting a CubeCamera's
//! capture of the scene every frame ( the cube render target's PMREM, as
//! WebGL regenerates it for the material's envMap ), with a turning box and
//! torus knot circling it, under the quarry HDR background and environment,
//! ACES Filmic, and auto-rotating OrbitControls. The scene is turned 0.5
//! about y; the cube camera, outside it, stays at the world origin. The
//! capture renders without tone mapping, as render targets do.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use crate::environment::{CubeCapture, EnvironmentMap};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

pub(super) struct Demo {
    controls: Controls,
    time: f64,
    steps: u32,
    sphere: Object3D,
    cube: Object3D,
    torus: Object3D,
    /// The cube and torus's accumulated rotation.
    rotation: Vector2,
    capture: CubeCapture,
    cube_camera: Object3D,
    /// Animation frames due at the next render.
    pending: u32,
    /// OrbitControls' wheel handler calls update(), which auto-rotates.
    wheels: u32,
    /// The GUI: the mirror's roughness and metalness, toneMappingExposure.
    params: [f64; 3],
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 75.);
        s.environment = Some(Arc::new(EnvironmentMap::from_hdr(
            &fetch("/web/gallery/assets/draco-variants/quarry_01_1k.hdr").await?,
        )?));
        s.background_environment = true;
        // cubeRenderTarget = new WebGLCubeRenderTarget( 256 ), half float.
        let capture = CubeCapture::new(r, 256)?;
        let root = s.insert(NodeKind::Group);
        s.get_mut(root)?.quaternion = Quaternion::from_rotation_y(0.5);
        let mut mirror = MeshStandardMaterial {
            roughness: 0.05,
            metalness: 1.,
            ..Default::default()
        };
        mirror.properties.env_map = Some(capture.environment.clone());
        let sphere = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(IcosahedronGeometry::build(15., 8)?),
            Arc::new(Material::Standard(mirror)),
        )));
        let plain = Arc::new(Material::Standard(MeshStandardMaterial {
            roughness: 0.1,
            metalness: 0.,
            ..Default::default()
        }));
        let cube = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(15., 15., 15.)?),
            plain.clone(),
        )));
        let torus = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(8., 3., 128, 16, 2, 3)?),
            plain,
        )));
        for node in [sphere, cube, torus] {
            s.add(root, node)?;
        }
        // CubeCamera( 1, 1000 ): not in the scene, at the origin.
        let cube_camera = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 90.,
            aspect: 1.,
            near: 1.,
            far: 1000.,
            ..Default::default()
        })));
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.auto_rotate = Some(2.);
        controls.update(s, c)?;
        Ok(Self {
            controls,
            time: 0.,
            steps: 1,
            sphere,
            cube,
            torus,
            rotation: Vector2::ZERO,
            capture,
            cube_camera,
            pending: 0,
            wheels: 0,
            params: [0.05, 1., 1.],
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        self.steps = self.steps.max(u32::from(animate));
        Ok(())
    }
    /// animate( msTime ): the box and knot circle by the time and turn per frame.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let steps = std::mem::take(&mut self.steps);
        self.rotation += Vector2::new(0.02, 0.03) * steps as f64;
        let time = self.time;
        for (node, phase) in [(self.cube, 0.), (self.torus, 10.)] {
            let n = s.get_mut(node)?;
            let t = time + phase;
            n.position = Vector3::new(t.cos() * 30., t.sin() * 30., t.sin() * 30.);
            n.quaternion = Euler {
                angles: Vector3::new(self.rotation.x, self.rotation.y, 0.),
                order: EulerOrder::XYZ,
            }
            .quaternion();
        }
        self.pending = steps;
        if let NodeKind::Mesh(mesh) = &mut s.get_mut(self.sphere)?.kind
            && let Material::Standard(m) = Arc::make_mut(&mut mesh.materials[0])
        {
            m.roughness = self.params[0];
            m.metalness = self.params[1];
        }
        s.exposure = self.params[2];
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        // cubeCamera.update( renderer, scene ), then controls.update().
        if std::mem::take(&mut self.pending) > 0 {
            let exposure = s.tone_mapping;
            s.tone_mapping = ToneMapping::None;
            let result = self.capture.update(r, s, self.cube_camera, Vector3::ZERO);
            s.tone_mapping = exposure;
            result?;
            self.controls.frame_update(s, c)?;
        } else {
            self.controls.update(s, c)?;
        }
        for _ in 0..std::mem::take(&mut self.wheels) {
            self.controls.frame_update(s, c)?;
        }
        s.tone_mapping = ToneMapping::Aces;
        r.render(s, c, out)?;
        Ok(true)
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
            self.wheels += 1;
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// The GUI: roughness, metalness ( the mirror's ), exposure.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("cubemap parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.steps += 1;
    }
}
