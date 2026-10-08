//! CSS3DRenderer, CSS3DObject and CSS3DSprite
//! ( `examples/jsm/renderers/CSS3DRenderer.js` ): HTML elements transformed
//! in 3D by CSS, under a camera element whose `perspective` and
//! `matrix3d` follow the camera, as three.js r186 writes them.
use crate::css2d::DomElement;
#[cfg(target_arch = "wasm32")]
use crate::{
    Error, Result,
    camera::Camera,
    css2d::js_number,
    math::{Matrix4, Quaternion, Vector3},
    scene::{NodeKind, Object3D, Scene},
};
use serde::Serialize;

/// CSS3DObject: an element placed by its object's world matrix.
#[derive(Clone, Debug, Serialize)]
pub struct CSS3DObject {
    pub element: DomElement,
}
/// CSS3DSprite: a CSS3DObject that faces the camera, turned by `rotation2D`.
#[derive(Clone, Debug, Serialize)]
pub struct CSS3DSprite {
    pub element: DomElement,
    #[serde(rename = "rotation2D")]
    pub rotation_2d: f64,
}
#[cfg(target_arch = "wasm32")]
fn prepare_element(element: &web_sys::HtmlElement) {
    let style = element.style();
    let _ = style.set_property("position", "absolute");
    let _ = style.set_property("pointer-events", "auto");
    let _ = style.set_property("user-select", "none");
    let _ = element.set_attribute("draggable", "false");
}
impl CSS3DObject {
    /// `new CSS3DObject( element )`.
    #[cfg(target_arch = "wasm32")]
    pub fn new(element: web_sys::HtmlElement) -> Self {
        prepare_element(&element);
        Self {
            element: DomElement::new(element),
        }
    }
}
impl CSS3DSprite {
    /// `new CSS3DSprite( element )`.
    #[cfg(target_arch = "wasm32")]
    pub fn new(element: web_sys::HtmlElement) -> Self {
        prepare_element(&element);
        Self {
            element: DomElement::new(element),
            rotation_2d: 0.,
        }
    }
}

/// CSS3DRenderer: its element ( `domElement` ) holding the view and camera
/// elements, and its size.
#[cfg(target_arch = "wasm32")]
pub struct CSS3DRenderer {
    dom_element: web_sys::HtmlElement,
    view_element: web_sys::HtmlElement,
    camera_element: web_sys::HtmlElement,
    width: f64,
    height: f64,
    /// The camera's and the objects' last styles, written only on change.
    camera_style: String,
    object_styles: std::collections::HashMap<Object3D, String>,
}
/// The renderer's parameters: an element to use instead of a new `div`.
#[cfg(target_arch = "wasm32")]
#[derive(Default)]
pub struct CSS3DRendererParameters {
    pub element: Option<web_sys::HtmlElement>,
}
/// `epsilon( value )`: magnitudes under 1e-10 print as 0.
#[cfg(target_arch = "wasm32")]
fn epsilon(v: f64) -> String {
    js_number(if v.abs() < 1e-10 { 0. } else { v })
}
/// getCameraCSSMatrix(): the view matrix with y flipped, column-major.
#[cfg(target_arch = "wasm32")]
fn camera_css_matrix(m: Matrix4) -> String {
    let e = m.to_cols_array();
    let signs = [
        1., -1., 1., 1., 1., -1., 1., 1., 1., -1., 1., 1., 1., -1., 1., 1.,
    ];
    let values: Vec<String> = e.iter().zip(signs).map(|(v, s)| epsilon(v * s)).collect();
    format!("matrix3d({})", values.join(","))
}
/// getObjectCSSMatrix(): the world matrix with its y column flipped,
/// centered on the element.
#[cfg(target_arch = "wasm32")]
fn object_css_matrix(m: Matrix4) -> String {
    let e = m.to_cols_array();
    let signs = [
        1., 1., 1., 1., -1., -1., -1., -1., 1., 1., 1., 1., 1., 1., 1., 1.,
    ];
    let values: Vec<String> = e.iter().zip(signs).map(|(v, s)| epsilon(v * s)).collect();
    format!("translate(-50%,-50%)matrix3d({})", values.join(","))
}
#[cfg(target_arch = "wasm32")]
impl CSS3DRenderer {
    pub fn new(parameters: CSS3DRendererParameters) -> Result<Self> {
        use wasm_bindgen::JsCast;
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or(Error::Invalid("CSS3DRenderer document"))?;
        let div = || -> Result<web_sys::HtmlElement> {
            document
                .create_element("div")
                .map_err(|_| Error::Invalid("CSS3DRenderer element"))?
                .dyn_into::<web_sys::HtmlElement>()
                .map_err(|_| Error::Invalid("CSS3DRenderer element"))
        };
        let dom_element = match parameters.element {
            Some(e) => e,
            None => div()?,
        };
        let _ = dom_element.style().set_property("overflow", "hidden");
        let view_element = div()?;
        let _ = view_element.style().set_property("transform-origin", "0 0");
        let _ = view_element.style().set_property("pointer-events", "none");
        dom_element
            .append_child(&view_element)
            .map_err(|_| Error::Invalid("CSS3DRenderer element"))?;
        let camera_element = div()?;
        let _ = camera_element
            .style()
            .set_property("transform-style", "preserve-3d");
        view_element
            .append_child(&camera_element)
            .map_err(|_| Error::Invalid("CSS3DRenderer element"))?;
        Ok(Self {
            dom_element,
            view_element,
            camera_element,
            width: 0.,
            height: 0.,
            camera_style: String::new(),
            object_styles: Default::default(),
        })
    }
    pub fn dom_element(&self) -> &web_sys::HtmlElement {
        &self.dom_element
    }
    pub fn get_size(&self) -> crate::css2d::Size {
        crate::css2d::Size {
            width: self.width,
            height: self.height,
        }
    }
    /// setSize( width, height ), in CSS pixels, for all three elements.
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
        for e in [&self.dom_element, &self.view_element, &self.camera_element] {
            let style = e.style();
            let _ = style.set_property("width", &format!("{}px", js_number(width)));
            let _ = style.set_property("height", &format!("{}px", js_number(height)));
        }
    }
    /// render( scene, camera ): the camera element's transform, then every
    /// object's.
    pub fn render(&mut self, scene: &mut Scene, camera: Object3D) -> Result<()> {
        scene.update()?;
        let node = scene.get(camera)?.clone();
        self.render_with(scene, &node)
    }
    /// render( scene, camera ) with a camera from another scene, as three.js
    /// renders a scene with any camera ( css3d_sandbox's scene2 ). The
    /// camera's scene must be up to date.
    pub fn render_from(
        &mut self,
        scene: &mut Scene,
        camera_scene: &Scene,
        camera: Object3D,
    ) -> Result<()> {
        scene.update()?;
        let node = camera_scene.get(camera)?.clone();
        self.render_with(scene, &node)
    }
    fn render_with(&mut self, scene: &mut Scene, node: &crate::scene::Node) -> Result<()> {
        let (half_w, half_h) = (self.width / 2., self.height / 2.);
        let NodeKind::Camera(c) = &node.kind else {
            return Err(Error::Invalid("CSS3DRenderer camera"));
        };
        let projection = c.projection_matrix()?;
        // projectionMatrix.elements[ 5 ], the same in WebGL and WebGPU clip space.
        let fov = projection.y_axis.y * half_h;
        let view = match c {
            Camera::Perspective(p) => p.view,
            Camera::Orthographic(o) => o.view,
        };
        let view_style = match view {
            Some(v) => format!(
                "translate( {}px, {}px )scale( {}, {} )",
                js_number(-v.offset_x * (self.width / v.width)),
                js_number(-v.offset_y * (self.height / v.height)),
                js_number(v.full_width / v.width),
                js_number(v.full_height / v.height)
            ),
            None => String::new(),
        };
        let _ = self
            .view_element
            .style()
            .set_property("transform", &view_style);
        let inverse = node.matrix_world.inverse();
        let scale_by_view_offset = view.map_or(1., |v| v.height / v.full_height);
        let camera_css = match c {
            Camera::Orthographic(o) => {
                let tx = -(o.right + o.left) / 2.;
                let ty = (o.top + o.bottom) / 2.;
                format!(
                    "scale( {} )scale({})translate({}px,{}px){}",
                    js_number(scale_by_view_offset),
                    js_number(fov),
                    epsilon(tx),
                    epsilon(ty),
                    camera_css_matrix(inverse)
                )
            }
            Camera::Perspective(_) => format!(
                "scale( {} )translateZ({}px){}",
                js_number(scale_by_view_offset),
                js_number(fov),
                camera_css_matrix(inverse)
            ),
        };
        let perspective = match c {
            Camera::Perspective(_) => format!("perspective({}px) ", js_number(fov)),
            Camera::Orthographic(_) => String::new(),
        };
        let style = format!(
            "{perspective}{camera_css}translate({}px,{}px)",
            js_number(half_w),
            js_number(half_h)
        );
        if self.camera_style != style {
            let _ = self
                .camera_element
                .style()
                .set_property("transform", &style);
            self.camera_style = style;
        }
        let camera_layers = node.layers;
        for root in scene.roots() {
            self.render_object(scene, root, inverse, camera_layers)?;
        }
        Ok(())
    }
    fn element(kind: &NodeKind) -> Option<&web_sys::HtmlElement> {
        match kind {
            NodeKind::CSS3DObject(o) => o.element.element.as_ref(),
            NodeKind::CSS3DSprite(o) => o.element.element.as_ref(),
            _ => None,
        }
    }
    fn hide_object(scene: &Scene, object: Object3D) -> Result<()> {
        let node = scene.get(object)?;
        if let Some(e) = Self::element(&node.kind) {
            let _ = e.style().set_property("display", "none");
        }
        for &child in node.children() {
            Self::hide_object(scene, child)?;
        }
        Ok(())
    }
    fn render_object(
        &mut self,
        scene: &mut Scene,
        object: Object3D,
        view: Matrix4,
        camera_layers: crate::scene::Layers,
    ) -> Result<()> {
        let node = scene.get(object)?;
        if !node.visible {
            return Self::hide_object(scene, object);
        }
        if let Some(element) = Self::element(&node.kind).cloned() {
            let visible = node.layers.test(camera_layers);
            let _ = element
                .style()
                .set_property("display", if visible { "" } else { "none" });
            if visible {
                let hooks = node.render_hooks.clone();
                if let Some(before) = &hooks.before {
                    before(scene, object);
                }
                let node = scene.get(object)?;
                let style = match &node.kind {
                    NodeKind::CSS3DSprite(sprite) => {
                        // The billboard: the view's rotation transposed, then the
                        // object's position and scale.
                        let mut m = view.transpose();
                        if sprite.rotation_2d != 0. {
                            m *= Matrix4::from_rotation_z(sprite.rotation_2d);
                        }
                        let (scale, _, position): (Vector3, Quaternion, Vector3) =
                            node.matrix_world.to_scale_rotation_translation();
                        m.w_axis = position.extend(m.w_axis.w);
                        m.x_axis *= scale.x;
                        m.y_axis *= scale.y;
                        m.z_axis *= scale.z;
                        m.x_axis.w = 0.;
                        m.y_axis.w = 0.;
                        m.z_axis.w = 0.;
                        m.w_axis.w = 1.;
                        object_css_matrix(m)
                    }
                    _ => object_css_matrix(node.matrix_world),
                };
                if self.object_styles.get(&object) != Some(&style) {
                    let _ = element.style().set_property("transform", &style);
                    self.object_styles.insert(object, style);
                }
                let parent: Option<web_sys::Node> = element.parent_node();
                let holder: &web_sys::Node = self.camera_element.as_ref();
                if parent.as_ref() != Some(holder) {
                    let _ = self.camera_element.append_child(&element);
                }
                if let Some(after) = &hooks.after {
                    after(scene, object);
                }
            }
        }
        for child in scene.get(object)?.children().to_vec() {
            self.render_object(scene, child, view, camera_layers)?;
        }
        Ok(())
    }
}
