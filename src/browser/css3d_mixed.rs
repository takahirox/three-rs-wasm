//! css3d_mixed: an iframe of the gallery ( `./#webgl_animation_keyframes` )
//! placed by CSS3DRenderer behind the canvas, seen through a NoBlending,
//! premultiplied, zero-opacity cutout plane in a framed room. The canvas
//! ( alpha, NeutralToneMapping ) lies over the CSS layer without pointer
//! events, and damped OrbitControls run on a full-window element beneath
//! it, the iframe taking the pointer where it shows.
use super::controls_attributes::{Controls, camera_state};
use super::css3d_common::{add_style, document, element, page_css, window_size};
use super::helpers_formats::edges_geometry;
use super::text_shapes::{Extrude, Path, Shape, extrude};
use crate::attribute::BufferAttribute;
use crate::css3d::{CSS3DObject, CSS3DRenderer, CSS3DRendererParameters};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

pub(super) struct Demo {
    controls: Controls,
    css: CSS3DRenderer,
    size: (f64, f64),
    /// The controls element's start and end listeners.
    _listeners: Vec<Closure<dyn FnMut()>>,
}
fn vec3s(data: Vec<f32>) -> Result<crate::geometry::Attribute> {
    Ok(crate::geometry::Attribute::F32(BufferAttribute::new(
        data, 3, false,
    )?))
}
fn floats(g: &BufferGeometry) -> Vec<f32> {
    match g.attributes.get("position") {
        Some(crate::geometry::Attribute::F32(a)) => a.array().to_vec(),
        _ => vec![],
    }
}
/// buildFrame(): the border with the picture's hole, extruded `thickness`,
/// and its back plane, in one MeshStandardMaterial.
fn build_frame(s: &mut Scene, width: f64, height: f64, thickness: f64) -> Result<Object3D> {
    let group = s.insert(NodeKind::Group);
    let material = Arc::new(Material::Standard(MeshStandardMaterial {
        properties: MaterialProperties {
            color: Color::from_hex(0x2200ff),
            ..Default::default()
        },
        ..Default::default()
    }));
    let (ow, oh) = (width / 2. + thickness, height / 2. + thickness);
    let shape = Shape {
        outline: Path::polygon(&[[-ow, -oh], [ow, -oh], [ow, oh], [-ow, oh]]),
        holes: vec![Path::polygon(&[
            [-width / 2., -height / 2.],
            [width / 2., -height / 2.],
            [width / 2., height / 2.],
            [-width / 2., height / 2.],
        ])],
    };
    let (positions, _) = extrude(
        &[shape],
        &Extrude {
            curve_segments: 12,
            steps: 1,
            depth: thickness,
            bevel: None,
        },
    );
    let mut frame = BufferGeometry::default();
    frame.set_attribute("position", vec3s(positions)?);
    frame.compute_vertex_normals()?;
    let frame_mesh = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(frame), material.clone())));
    s.get_mut(frame_mesh)?.position.z = -thickness / 2.;
    s.add(group, frame_mesh)?;
    let back = PlaneGeometry::build(width + thickness * 2., height + thickness * 2., 1, 1)?;
    let back_mesh = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(back), material)));
    let n = s.get_mut(back_mesh)?;
    n.position = Vector3::new(0., 0., -thickness / 2.);
    n.quaternion = Quaternion::from_rotation_y(PI);
    s.add(group, back_mesh)?;
    Ok(group)
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        // The gallery's index rules place iframes: the page has none of them.
        add_style(&format!(
            "{}#css3d-renderer iframe{{top:auto;left:auto;right:auto}}",
            page_css(&["#css3d-renderer"], "#000")
        ))?;
        let body = document()?.body().ok_or(Error::Invalid("body"))?;
        let append = |e: &web_sys::HtmlElement| {
            body.append_child(e)
                .map(|_| ())
                .map_err(|_| Error::Invalid("css3d_mixed element"))
        };
        // The controls' element, full-window beneath the CSS layer.
        let controls_element = element("div")?;
        let style = controls_element.style();
        let _ = style.set_property("position", "absolute");
        let _ = style.set_property("top", "0");
        let _ = style.set_property("width", "100%");
        let _ = style.set_property("height", "100%");
        let _ = controls_element.set_attribute("data-pointer-target", "");
        append(&controls_element)?;
        // The CSS layer: the page's static renderer element passes the
        // pointer on to the controls; only its elements take it.
        let mut css = CSS3DRenderer::new(CSS3DRendererParameters::default())?;
        let size = window_size();
        css.set_size(size.0, size.1);
        let e = css.dom_element();
        e.set_id("css3d-renderer");
        let _ = e.set_attribute("lang", "en");
        let _ = e.style().set_property("position", "absolute");
        let _ = e.style().set_property("top", "0px");
        let _ = e.style().set_property("pointer-events", "none");
        append(e)?;
        // The WebGL canvas over both, without pointer events.
        if let Some(canvas) = document()?.query_selector("canvas").ok().flatten() {
            let canvas = canvas
                .dyn_into::<web_sys::HtmlElement>()
                .map_err(|_| Error::Invalid("canvas"))?;
            let _ = canvas.style().set_property("position", "absolute");
            let _ = canvas.style().set_property("top", "0");
            let _ = canvas.style().set_property("pointer-events", "none");
            append(&canvas)?;
        }
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-1000., 500., 1500.);
        s.background = Color::from_hex(0xf0f0f0);
        s.tone_mapping = ToneMapping::Neutral;
        // Add room
        let room = BoxGeometry::segmented(4000., 2000., 4000., 10, 5, 10)?;
        let mut edges = BufferGeometry::default();
        edges.set_attribute(
            "position",
            vec3s(edges_geometry(
                &floats(&room),
                room.index.as_deref().unwrap_or(&[]),
            ))?,
        );
        let mut line = LineBasicMaterial::default();
        line.properties.color = Color::BLACK;
        line.properties.opacity = 0.2;
        line.properties.transparent = true;
        s.insert(NodeKind::Line(Line {
            geometry: Arc::new(edges),
            material: Arc::new(Material::Line(line)),
            segments: true,
        }));
        // Add light
        let light = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x444444),
            intensity: 4.,
        }));
        s.get_mut(light)?.position = Vector3::new(-25., 100., 50.);
        // Add cutout mesh: NoBlending writes its premultiplied zero alpha.
        let cutout = Arc::new(Material::Basic(MeshBasicMaterial {
            properties: MaterialProperties {
                color: Color::from_hex(0xff0000),
                blending: Some(wgpu::BlendState::REPLACE),
                opacity: 0.,
                premultiplied_alpha: true,
                ..Default::default()
            },
        }));
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(1024., 768., 1, 1)?),
            cutout,
        )));
        // Add frame
        build_frame(s, 1024., 768., 50.)?;
        // Add CSS3D element
        let iframe = element("iframe")?;
        let style = iframe.style();
        let _ = style.set_property("width", "1028px");
        let _ = style.set_property("height", "768px");
        let _ = style.set_property("border", "0px");
        let _ = style.set_property("backface-visibility", "hidden");
        let _ = iframe.set_attribute("src", "./#webgl_animation_keyframes");
        s.insert(NodeKind::CSS3DObject(CSS3DObject::new(iframe.clone())));
        // Add controls: the iframe leaves the pointer to them while they run
        // ( a wheel's start and end come together ).
        let mut listeners = vec![];
        for (events, value) in [
            (&["pointerdown"][..], "none"),
            (&["pointerup", "pointercancel"][..], "auto"),
        ] {
            let iframe = iframe.clone();
            let listener = Closure::<dyn FnMut()>::new(move || {
                let _ = iframe.style().set_property("pointer-events", value);
            });
            for event in events {
                let _ = controls_element
                    .add_event_listener_with_callback(event, listener.as_ref().unchecked_ref());
            }
            listeners.push(listener);
        }
        let controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        Ok(Self {
            controls,
            css,
            size,
            _listeners: listeners,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// animate(): controls.update(), the canvas, then the CSS layer.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let size = window_size();
        if size != self.size {
            self.size = size;
            self.css.set_size(size.0, size.1);
        }
        self.controls.update(s, c)?;
        s.update()?;
        self.css.render(s, c)
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
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("css3d_mixed parameter"))
    }
    pub fn seek(&mut self, _t: f64) {}
}
