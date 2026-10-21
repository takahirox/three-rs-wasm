//! webgl_loader_fbx_nurbs: nurbs.fbx through the FBXLoader port, its five
//! NurbsCurve geometries (open, closed and periodic, orders 3 and 4) sampled
//! as NURBSCurve.getPoints( controlPoints × 12 ) into lines with the loader's
//! 0x3300ff LineBasicMaterial, the models' transforms under the loader's
//! group, a GridHelper and OrbitControls. The curves are evaluated once at
//! load, as the loader does.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use super::interactive_scenes::grid_helper;
use crate::fbx::FbxNodeKind;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

pub(super) struct Demo {
    controls: Controls,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(2., 18., 28.);
        s.background = Color::BLACK;
        s.insert(NodeKind::Line(grid_helper(28., 28, 0x303030, 0x303030)?));
        // FBXLoader.load( 'models/fbx/nurbs.fbx' ): the curves as lines under
        // the loader's group.
        let fbx = crate::fbx::parse(
            &fetch("/web/gallery/assets/fbx/nurbs.fbx").await?,
            &crate::fbx::Options::default(),
        )?;
        let geometries: Vec<Arc<BufferGeometry>> = fbx
            .geometries
            .iter()
            .map(|g| Arc::new(g.geometry.clone()))
            .collect();
        let mut handles: Vec<Object3D> = vec![];
        for n in &fbx.nodes {
            let h = match &n.kind {
                FbxNodeKind::Line { geometry, material } => {
                    let mut m = LineBasicMaterial::default();
                    m.properties.color = fbx.materials[*material].color;
                    s.insert(NodeKind::Line(Line {
                        geometry: geometry
                            .map(|g| geometries[g].clone())
                            .unwrap_or_else(|| Arc::new(BufferGeometry::default())),
                        material: Arc::new(Material::Line(m)),
                        segments: false,
                    }))
                }
                _ => s.insert(NodeKind::Group),
            };
            let node = s.get_mut(h)?;
            node.name = n.name.clone();
            node.position = n.position;
            node.quaternion = n.quaternion;
            node.scale = n.scale;
            if let Some(p) = n.parent {
                s.add(handles[p], h)?;
            }
            handles.push(h);
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 12., 0.));
        controls.update(s, c)?;
        Ok(Self { controls })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("fbx nurbs parameter"))
    }
    pub fn seek(&mut self, _t: f64) {}
}

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/webgl_loader_fbx_nurbs.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_fbx_nurbs(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 1,
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                2000.,
                Box::new(Demo::create(scene, camera, 363, renderer).await?),
            ))
        },
    )
    .await
}
