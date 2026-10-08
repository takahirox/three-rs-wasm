//! CSS2DRenderer and CSS2DObject ( `examples/jsm/renderers/CSS2DRenderer.js` ):
//! HTML elements placed over the canvas at their objects' projected
//! positions, as three.js r186 places them. A CSS2DObject is a scene node of
//! kind `NodeKind::CSS2DObject`; the renderer writes each element's
//! `display`, `transform-origin`, `transform` and `z-index` with the same
//! strings as the original.
use crate::math::Vector2;
#[cfg(target_arch = "wasm32")]
use crate::{
    Error, Result,
    math::Vector3,
    scene::{NodeKind, Object3D, Scene},
};
use serde::Serialize;

/// An HTML element held by a CSS object. Without a DOM ( native builds ) it
/// holds nothing.
#[derive(Debug, Default)]
pub struct DomElement {
    #[cfg(target_arch = "wasm32")]
    pub element: Option<web_sys::HtmlElement>,
}
impl Clone for DomElement {
    /// Object3D.copy(): `source.element.cloneNode( true )`.
    fn clone(&self) -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            element: self.element.as_ref().and_then(|e| {
                use wasm_bindgen::JsCast;
                e.clone_node_with_deep(true)
                    .ok()
                    .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
            }),
        }
    }
}
impl Serialize for DomElement {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_none()
    }
}
impl DomElement {
    #[cfg(target_arch = "wasm32")]
    pub fn new(element: web_sys::HtmlElement) -> Self {
        Self {
            element: Some(element),
        }
    }
    /// `element.remove()`, as the `removed` listener does.
    pub fn detach(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(e) = &self.element {
            e.remove();
        }
    }
}

/// CSS2DObject: an element, its pivot ( `center` ) and its rotation.
#[derive(Clone, Debug, Serialize)]
pub struct CSS2DObject {
    pub element: DomElement,
    /// The pivot in the element, ( 0.5, 0.5 ) for its middle.
    pub center: Vector2,
    #[serde(rename = "rotation2D")]
    pub rotation_2d: f64,
}
impl CSS2DObject {
    /// `new CSS2DObject( element )`: absolutely positioned, not selectable or
    /// draggable.
    #[cfg(target_arch = "wasm32")]
    pub fn new(element: web_sys::HtmlElement) -> Self {
        let style = element.style();
        let _ = style.set_property("position", "absolute");
        let _ = style.set_property("user-select", "none");
        let _ = element.set_attribute("draggable", "false");
        Self {
            element: DomElement::new(element),
            center: Vector2::new(0.5, 0.5),
            rotation_2d: 0.,
        }
    }
}

/// A number as JavaScript prints it in a template literal: the shortest
/// round trip, exponents below 1e-6 and from 1e21, and −0 as "0".
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(crate) fn js_number(v: f64) -> String {
    if v == 0. {
        return "0".into();
    }
    if v.is_nan() {
        return "NaN".into();
    }
    if v.is_infinite() {
        return if v > 0. { "Infinity" } else { "-Infinity" }.into();
    }
    let a = v.abs();
    if !(1e-6..1e21).contains(&a) {
        let s = format!("{v:e}");
        // Rust prints 1e21 and 1e-7; JavaScript prints 1e+21 and 1e-7.
        return match s.split_once('e') {
            Some((m, e)) if !e.starts_with('-') => format!("{m}e+{e}"),
            _ => s,
        };
    }
    format!("{v}")
}

/// CSS2DRenderer: its element ( `domElement` ), its size and the depth
/// order of its objects.
#[cfg(target_arch = "wasm32")]
pub struct CSS2DRenderer {
    dom_element: web_sys::HtmlElement,
    width: f64,
    height: f64,
    pub sort_objects: bool,
}
/// The renderer's parameters: an element to use instead of a new `div`.
#[cfg(target_arch = "wasm32")]
#[derive(Default)]
pub struct CSS2DRendererParameters {
    pub element: Option<web_sys::HtmlElement>,
}
/// getSize(): the size set by setSize().
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}
#[cfg(target_arch = "wasm32")]
impl CSS2DRenderer {
    pub fn new(parameters: CSS2DRendererParameters) -> Result<Self> {
        use wasm_bindgen::JsCast;
        let dom_element = match parameters.element {
            Some(e) => e,
            None => web_sys::window()
                .and_then(|w| w.document())
                .ok_or(Error::Invalid("CSS2DRenderer document"))?
                .create_element("div")
                .map_err(|_| Error::Invalid("CSS2DRenderer element"))?
                .dyn_into::<web_sys::HtmlElement>()
                .map_err(|_| Error::Invalid("CSS2DRenderer element"))?,
        };
        let _ = dom_element.style().set_property("overflow", "hidden");
        Ok(Self {
            dom_element,
            width: 0.,
            height: 0.,
            sort_objects: true,
        })
    }
    /// The element that holds the objects' elements.
    pub fn dom_element(&self) -> &web_sys::HtmlElement {
        &self.dom_element
    }
    pub fn get_size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }
    /// setSize( width, height ), in CSS pixels.
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
        let style = self.dom_element.style();
        let _ = style.set_property("width", &format!("{}px", js_number(width)));
        let _ = style.set_property("height", &format!("{}px", js_number(height)));
    }
    /// render( scene, camera ): the objects in the scene's order, then their
    /// z-index by render order and distance to the camera.
    pub fn render(&mut self, scene: &mut Scene, camera: Object3D) -> Result<()> {
        scene.update()?;
        let node = scene.get(camera)?;
        let NodeKind::Camera(c) = &node.kind else {
            return Err(Error::Invalid("CSS2DRenderer camera"));
        };
        let view = node.matrix_world.inverse();
        let view_projection = c.projection_matrix()? * view;
        let camera_position = node.matrix_world.w_axis.truncate();
        let camera_layers = node.layers;
        let mut distances = std::collections::HashMap::new();
        for root in scene.roots() {
            self.render_object(
                scene,
                root,
                view_projection,
                camera_position,
                camera_layers,
                &mut distances,
            )?;
        }
        if self.sort_objects {
            self.z_order(scene, &distances)?;
        }
        Ok(())
    }
    fn hide_object(scene: &Scene, object: Object3D) -> Result<()> {
        let node = scene.get(object)?;
        if let NodeKind::CSS2DObject(o) = &node.kind
            && let Some(e) = &o.element.element
        {
            let _ = e.style().set_property("display", "none");
        }
        for &child in node.children() {
            Self::hide_object(scene, child)?;
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    fn render_object(
        &self,
        scene: &mut Scene,
        object: Object3D,
        view_projection: crate::math::Matrix4,
        camera_position: Vector3,
        camera_layers: crate::scene::Layers,
        distances: &mut std::collections::HashMap<Object3D, f64>,
    ) -> Result<()> {
        let node = scene.get(object)?;
        if !node.visible {
            return Self::hide_object(scene, object);
        }
        if let NodeKind::CSS2DObject(o) = &node.kind {
            let world = node.matrix_world.w_axis.truncate();
            let clip = view_projection * world.extend(1.);
            let v = clip.truncate() / clip.w;
            // WebGL's −1 ≤ z ≤ 1 is WebGPU's 0 ≤ z ≤ 1.
            let visible = (0. ..=1.).contains(&v.z) && node.layers.test(camera_layers);
            let hooks = node.render_hooks.clone();
            let (center, rotation) = (o.center, o.rotation_2d);
            let element = o.element.element.clone();
            if let Some(element) = &element {
                let style = element.style();
                let _ = style.set_property("display", if visible { "" } else { "none" });
                if visible {
                    if let Some(before) = &hooks.before {
                        before(scene, object);
                    }
                    let (cx, cy) = (100. * center.x, 100. * center.y);
                    let _ = style.set_property(
                        "transform-origin",
                        &format!("{}% {}%", js_number(cx), js_number(cy)),
                    );
                    let angle = -rotation;
                    let (half_w, half_h) = (self.width / 2., self.height / 2.);
                    let tx = v.x * half_w + half_w;
                    let ty = -v.y * half_h + half_h;
                    let _ = style.set_property(
                        "transform",
                        &format!(
                            "translate({}%, {}%) translate({}px, {}px) rotate({}rad)",
                            js_number(-cx),
                            js_number(-cy),
                            js_number(tx),
                            js_number(ty),
                            js_number(angle)
                        ),
                    );
                    let parent: Option<web_sys::Node> = element.parent_node();
                    let holder: &web_sys::Node = self.dom_element.as_ref();
                    if parent.as_ref() != Some(holder) {
                        let _ = self.dom_element.append_child(element);
                    }
                    if let Some(after) = &hooks.after {
                        after(scene, object);
                    }
                }
            }
            distances.insert(object, camera_position.distance_squared(world));
        }
        for child in scene.get(object)?.children().to_vec() {
            self.render_object(
                scene,
                child,
                view_projection,
                camera_position,
                camera_layers,
                distances,
            )?;
        }
        Ok(())
    }
    /// zOrder(): the visible objects, nearer and of higher render order on top.
    fn z_order(
        &self,
        scene: &Scene,
        distances: &std::collections::HashMap<Object3D, f64>,
    ) -> Result<()> {
        let mut sorted = vec![];
        for root in scene.roots() {
            for h in scene.traverse(root, true)? {
                let node = scene.get(h)?;
                if let NodeKind::CSS2DObject(o) = &node.kind {
                    sorted.push((h, node.render_order, o));
                }
            }
        }
        let distance = |h: &Object3D| distances.get(h).copied().unwrap_or(f64::NAN);
        sorted.sort_by(|a, b| {
            if a.1 != b.1 {
                return b.1.cmp(&a.1);
            }
            distance(&a.0)
                .partial_cmp(&distance(&b.0))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let z_max = sorted.len();
        for (i, (_, _, o)) in sorted.iter().enumerate() {
            if let Some(e) = &o.element.element {
                let _ = e.style().set_property("z-index", &(z_max - i).to_string());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::js_number;
    #[test]
    fn numbers_print_as_in_javascript() {
        assert_eq!(js_number(-0.), "0");
        assert_eq!(js_number(256.), "256");
        assert_eq!(js_number(-50.), "-50");
        assert_eq!(js_number(0.1 + 0.2), "0.30000000000000004");
        assert_eq!(js_number(1e-7), "1e-7");
        assert_eq!(js_number(1.5e-7), "1.5e-7");
        assert_eq!(js_number(0.000001), "0.000001");
        assert_eq!(js_number(1e21), "1e+21");
        assert_eq!(js_number(123456789012345680000.), "123456789012345680000");
    }
}
