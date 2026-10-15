//! webgl_shadowmap_pcss: twenty bouncing Phong spheres of random colors, a
//! column and a large ground in fog, lit by an ambient and a directional
//! light with a BasicShadowMap. The page splices its PCSS functions into
//! shadowmap_pars_fragment, but its second replacement (the call in
//! getShadow) no longer matches r186's chunk, which has a blank line after
//! `if ( frustumTest ) {`: the original draws the plain BasicShadowMap
//! comparison, and so does the port. The shadow camera's CameraHelper keeps
//! the projection it read when constructed (far 500: the page edits far
//! without updating it).
use super::controls_attributes::{Controls, camera_helper, camera_state, update_camera_helper};
use crate::shadow::ShadowFilter;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

pub(super) struct Demo {
    time: f64,
    /// The spheres and their phases.
    spheres: Vec<(Object3D, f64)>,
    shadow_camera: (Object3D, Object3D),
    controls: Controls,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 30.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(7., 13., 7.);
        n.quaternion = Quaternion::IDENTITY;
        // renderer.setClearColor( scene.fog.color ).
        let fog = Color::from_hex(0xcce0ff);
        s.background = fog;
        s.fog = Some(Fog::Linear {
            color: fog,
            near: 5.,
            far: 100.,
        });
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xaaaaaa),
            intensity: 3.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0xf0f6ff),
            intensity: 4.5,
            target: Vector3::ZERO,
        }));
        let light_position = Vector3::new(2., 8., 4.);
        let n = s.get_mut(light)?;
        n.position = light_position;
        n.cast_shadow = true;
        n.shadow.map_size = Some(1024);
        n.shadow.near = 0.5;
        n.shadow.far = 20.;
        n.shadow.filter = ShadowFilter::Basic;
        // CameraHelper( light.shadow.camera ): the default ±5 frustum with the
        // far plane of the matrix it was built with (500).
        let (geometry, program) = camera_helper(r).await?;
        let camera = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -5.,
            right: 5.,
            top: 5.,
            bottom: -5.,
            near: 0.5,
            far: 500.,
            zoom: 1.,
            ..Default::default()
        })));
        s.get_mut(camera)?.position = light_position;
        s.look_at(camera, Vector3::ZERO)?;
        let mut m = ShaderMaterial::new(program);
        m.properties.tone_mapped = false;
        m.properties.fog = false;
        let helper = s.insert(NodeKind::Line(Line {
            geometry,
            material: Arc::new(Material::Shader(m)),
            segments: true,
        }));
        s.get_mut(helper)?.frustum_culled = false;
        update_camera_helper(s, camera, helper)?;
        let phong = |color: Color| {
            let mut m = MeshPhongMaterial::default();
            m.properties.color = color;
            Arc::new(Material::Phong(m))
        };
        // The fixture's seeded Math.random, in the page's order: each
        // sphere's color, then x, z, distance and phase.
        let mut seed = 186u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.
        };
        let sphere = Arc::new(SphereGeometry::build(0.3, 20, 20)?);
        let mut spheres = vec![];
        for _ in 0..20 {
            // Color.setHex( Math.random() × 0xffffff ) floors the value.
            let color = Color::from_hex((random() * 16777215.).floor() as u32);
            let node = s.insert(NodeKind::Mesh(Mesh::new(sphere.clone(), phong(color))));
            let x = random() - 0.5;
            let z = random() - 0.5;
            let length = x.hypot(z);
            let (x, z) = if length > 0. {
                (x / length, z / length)
            } else {
                (0., 0.)
            };
            let distance = random() * 2. + 1.;
            let phase = random() * PI;
            let n = s.get_mut(node)?;
            n.position = Vector3::new(x * distance, 0., z * distance);
            n.cast_shadow = true;
            n.receive_shadow = true;
            spheres.push((node, phase));
        }
        let ground_material = phong(Color::from_hex(0x898989));
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(20000., 20000., 8, 8)?),
            ground_material.clone(),
        )));
        let n = s.get_mut(ground)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.receive_shadow = true;
        let column = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 4., 1.)?),
            ground_material,
        )));
        let n = s.get_mut(column)?;
        n.position.y = 2.;
        n.cast_shadow = true;
        n.receive_shadow = true;
        let mut controls = Controls::new(None, (10., 75.), PI * 0.5, true);
        controls.set_target(Vector3::new(0., 2.5, 0.));
        controls.update(s, c)?;
        let mut demo = Self {
            time: 0.,
            spheres,
            shadow_camera: (camera, helper),
            controls,
        };
        demo.place(s)?;
        Ok(demo)
    }
    /// animate(): each sphere bounces by |sin( time + phase )|.
    fn place(&mut self, s: &mut Scene) -> Result<()> {
        for &(node, phase) in &self.spheres {
            s.get_mut(node)?.position.y = (self.time + phase).sin().abs() * 4. + 0.3;
        }
        update_camera_helper(s, self.shadow_camera.0, self.shadow_camera.1)
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        self.place(s)
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("pcss has no parameters"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
