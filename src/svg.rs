//! SVGRenderer and SVGObject ( `examples/jsm/renderers/SVGRenderer.js` ) with
//! the Projector it draws through ( `examples/jsm/renderers/Projector.js` ):
//! the scene's faces, lines, sprites and points projected and clipped on the
//! CPU, painter-sorted and written as SVG paths, as three.js r186 writes
//! them. SVG output is CPU-projected in the original too; nothing here
//! replaces GPU work. The projection uses WebGL clip space ( z in −1 to 1 )
//! as the original's camera does.
use crate::math::Color;
use serde::Serialize;

/// An SVG element held by an SVGObject. Without a DOM ( native builds ) it
/// holds nothing.
#[derive(Debug, Default)]
pub struct SvgNode {
    #[cfg(target_arch = "wasm32")]
    pub element: Option<web_sys::Element>,
}
impl Clone for SvgNode {
    /// Object3D.copy() keeps the same node; SVGObject has no copy of its own.
    fn clone(&self) -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            element: self.element.clone(),
        }
    }
}
impl Serialize for SvgNode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_none()
    }
}
/// SVGObject: an SVG element placed at the node's projected position.
#[derive(Clone, Debug, Default, Serialize)]
pub struct SVGObject {
    pub node: SvgNode,
}
impl SVGObject {
    #[cfg(target_arch = "wasm32")]
    pub fn new(node: web_sys::Element) -> Self {
        Self {
            node: SvgNode {
                element: Some(node),
            },
        }
    }
}

/// `Color.getStyle( SRGBColorSpace )`: the working color in sRGB as
/// `rgb(r,g,b)`; without color management ( `ColorManagement.enabled = false` )
/// the values are written as they are.
pub fn color_style(color: Color, color_management: bool) -> String {
    let c = color.0.to_array().map(|v| {
        let v = if color_management {
            crate::math::linear_to_srgb(v)
        } else {
            v
        };
        // Math.round: halves toward +∞.
        (v * 255. + 0.5).floor()
    });
    format!(
        "rgb({},{},{})",
        crate::css2d::js_number(c[0]),
        crate::css2d::js_number(c[1]),
        crate::css2d::js_number(c[2])
    )
}

#[cfg(target_arch = "wasm32")]
pub use renderer::{SVGRenderInfo, SVGRenderer};

#[cfg(target_arch = "wasm32")]
mod renderer {
    use super::color_style;
    use crate::camera::Camera;
    use crate::css2d::js_number;
    use crate::geometry::{Attribute, BufferGeometry};
    use crate::material::{Material, Side};
    use crate::math::Color;
    use crate::scene::{Light, NodeKind, Object3D, Scene};
    use crate::{Error, Result};
    use std::collections::HashMap;
    use std::sync::Arc;

    type V3 = [f64; 3];
    type V4 = [f64; 4];
    const SVG_NS: &str = "http://www.w3.org/2000/svg";

    /// Vector3.applyMatrix4: the projective divide by w.
    fn apply3(e: &[f64; 16], p: V3) -> V3 {
        let w = 1. / (e[3] * p[0] + e[7] * p[1] + e[11] * p[2] + e[15]);
        [
            (e[0] * p[0] + e[4] * p[1] + e[8] * p[2] + e[12]) * w,
            (e[1] * p[0] + e[5] * p[1] + e[9] * p[2] + e[13]) * w,
            (e[2] * p[0] + e[6] * p[1] + e[10] * p[2] + e[14]) * w,
        ]
    }
    /// Vector4.applyMatrix4.
    fn apply4(e: &[f64; 16], v: V4) -> V4 {
        let [x, y, z, w] = v;
        [
            e[0] * x + e[4] * y + e[8] * z + e[12] * w,
            e[1] * x + e[5] * y + e[9] * z + e[13] * w,
            e[2] * x + e[6] * y + e[10] * z + e[14] * w,
            e[3] * x + e[7] * y + e[11] * z + e[15] * w,
        ]
    }
    /// Matrix4.multiplyMatrices( a, b ).
    fn multiply(a: &[f64; 16], b: &[f64; 16]) -> [f64; 16] {
        let mut te = [0.; 16];
        for c in 0..4 {
            for r in 0..4 {
                te[c * 4 + r] = a[r] * b[c * 4]
                    + a[4 + r] * b[c * 4 + 1]
                    + a[8 + r] * b[c * 4 + 2]
                    + a[12 + r] * b[c * 4 + 3];
            }
        }
        te
    }
    fn sub(a: V3, b: V3) -> V3 {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }
    fn dot(a: V3, b: V3) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }
    /// Vector3.normalize(): divideScalar( length || 1 ).
    fn normalize(v: V3) -> V3 {
        let length = dot(v, v).sqrt();
        let s = 1. / if length == 0. { 1. } else { length };
        [v[0] * s, v[1] * s, v[2] * s]
    }
    /// Vector3.applyMatrix3.
    fn apply_matrix3(e: &[f64; 9], v: V3) -> V3 {
        [
            e[0] * v[0] + e[3] * v[1] + e[6] * v[2],
            e[1] * v[0] + e[4] * v[1] + e[7] * v[2],
            e[2] * v[0] + e[5] * v[1] + e[8] * v[2],
        ]
    }
    /// Matrix3.getNormalMatrix( m ): the inverse transpose of its upper 3×3,
    /// with Matrix3.invert()'s operations.
    fn normal_matrix(m: &[f64; 16]) -> [f64; 9] {
        let te = [m[0], m[1], m[2], m[4], m[5], m[6], m[8], m[9], m[10]];
        let (n11, n21, n31) = (te[0], te[1], te[2]);
        let (n12, n22, n32) = (te[3], te[4], te[5]);
        let (n13, n23, n33) = (te[6], te[7], te[8]);
        let t11 = n33 * n22 - n32 * n23;
        let t12 = n32 * n13 - n33 * n12;
        let t13 = n23 * n12 - n22 * n13;
        let det = n11 * t11 + n21 * t12 + n31 * t13;
        if det == 0. {
            return [0.; 9];
        }
        let d = 1. / det;
        let inv = [
            t11 * d,
            (n31 * n23 - n33 * n21) * d,
            (n32 * n21 - n31 * n22) * d,
            t12 * d,
            (n33 * n11 - n31 * n13) * d,
            (n31 * n12 - n32 * n11) * d,
            t13 * d,
            (n21 * n13 - n23 * n11) * d,
            (n22 * n11 - n21 * n12) * d,
        ];
        // transpose()
        [
            inv[0], inv[3], inv[6], inv[1], inv[4], inv[7], inv[2], inv[5], inv[8],
        ]
    }
    /// Matrix4.invert() ( gl-matrix's cofactors, as r186 computes them ).
    fn invert(te: &[f64; 16]) -> [f64; 16] {
        let (n11, n21, n31, n41) = (te[0], te[1], te[2], te[3]);
        let (n12, n22, n32, n42) = (te[4], te[5], te[6], te[7]);
        let (n13, n23, n33, n43) = (te[8], te[9], te[10], te[11]);
        let (n14, n24, n34, n44) = (te[12], te[13], te[14], te[15]);
        let t1 = n11 * n22 - n21 * n12;
        let t2 = n11 * n32 - n31 * n12;
        let t3 = n11 * n42 - n41 * n12;
        let t4 = n21 * n32 - n31 * n22;
        let t5 = n21 * n42 - n41 * n22;
        let t6 = n31 * n42 - n41 * n32;
        let t7 = n13 * n24 - n23 * n14;
        let t8 = n13 * n34 - n33 * n14;
        let t9 = n13 * n44 - n43 * n14;
        let t10 = n23 * n34 - n33 * n24;
        let t11 = n23 * n44 - n43 * n24;
        let t12 = n33 * n44 - n43 * n34;
        let det = t1 * t12 - t2 * t11 + t3 * t10 + t4 * t9 - t5 * t8 + t6 * t7;
        if det == 0. {
            return [0.; 16];
        }
        let d = 1. / det;
        [
            (n22 * t12 - n32 * t11 + n42 * t10) * d,
            (n31 * t11 - n21 * t12 - n41 * t10) * d,
            (n24 * t6 - n34 * t5 + n44 * t4) * d,
            (n33 * t5 - n23 * t6 - n43 * t4) * d,
            (n32 * t9 - n12 * t12 - n42 * t8) * d,
            (n11 * t12 - n31 * t9 + n41 * t8) * d,
            (n34 * t3 - n14 * t6 - n44 * t2) * d,
            (n13 * t6 - n33 * t3 + n43 * t2) * d,
            (n12 * t11 - n22 * t9 + n42 * t7) * d,
            (n21 * t9 - n11 * t11 - n41 * t7) * d,
            (n14 * t5 - n24 * t3 + n44 * t1) * d,
            (n23 * t3 - n13 * t5 - n43 * t1) * d,
            (n22 * t8 - n12 * t10 - n32 * t7) * d,
            (n11 * t10 - n21 * t8 + n31 * t7) * d,
            (n24 * t2 - n14 * t4 - n34 * t1) * d,
            (n13 * t4 - n23 * t2 + n33 * t1) * d,
        ]
    }
    /// Camera.updateMatrixWorld()'s matrixWorldInverse: the world matrix
    /// inverted, or, unless its scale is exactly one, its decomposed position
    /// and rotation recomposed without scale and inverted.
    fn view_matrix(te: &[f64; 16]) -> [f64; 16] {
        let length = |a: f64, b: f64, c: f64| (a * a + b * b + c * c).sqrt();
        let mut sx = length(te[0], te[1], te[2]);
        let sy = length(te[4], te[5], te[6]);
        let sz = length(te[8], te[9], te[10]);
        if sx == 1. && sy == 1. && sz == 1. {
            return invert(te);
        }
        if glam::DMat4::from_cols_array(te).determinant() < 0. {
            sx = -sx;
        }
        let (ix, iy, iz) = (1. / sx, 1. / sy, 1. / sz);
        let q = crate::math::quaternion_from_rotation(
            crate::math::Vector3::new(te[0] * ix, te[1] * ix, te[2] * ix),
            crate::math::Vector3::new(te[4] * iy, te[5] * iy, te[6] * iy),
            crate::math::Vector3::new(te[8] * iz, te[9] * iz, te[10] * iz),
        );
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        let (x2, y2, z2) = (x + x, y + y, z + z);
        let (xx, xy, xz) = (x * x2, x * y2, x * z2);
        let (yy, yz, zz) = (y * y2, y * z2, z * z2);
        let (wx, wy, wz) = (w * x2, w * y2, w * z2);
        invert(&[
            1. - (yy + zz),
            xy + wz,
            xz - wy,
            0.,
            xy - wz,
            1. - (xx + zz),
            yz + wx,
            0.,
            xz + wy,
            yz - wx,
            1. - (xx + yy),
            0.,
            te[12],
            te[13],
            te[14],
            1.,
        ])
    }
    /// The camera's projectionMatrix in WebGL clip space, as
    /// updateProjectionMatrix() makes it.
    fn projection(camera: &Camera) -> [f64; 16] {
        let mut te = [0.; 16];
        match camera {
            Camera::Perspective(p) => {
                let near = p.near;
                let mut top = near * (std::f64::consts::PI / 180. * 0.5 * p.fov).tan() / p.zoom;
                let mut height = 2. * top;
                let mut width = p.aspect * height;
                let mut left = -0.5 * width;
                if let Some(v) = p.view {
                    left += v.offset_x * width / v.full_width;
                    top -= v.offset_y * height / v.full_height;
                    width *= v.width / v.full_width;
                    height *= v.height / v.full_height;
                }
                if p.film_offset != 0. {
                    // getFilmWidth(): filmGauge × min( aspect, 1 ).
                    left += near * p.film_offset / (p.film_gauge * p.aspect.min(1.));
                }
                let (right, bottom, far) = (left + width, top - height, p.far);
                te[0] = 2. * near / (right - left);
                te[5] = 2. * near / (top - bottom);
                te[8] = (right + left) / (right - left);
                te[9] = (top + bottom) / (top - bottom);
                te[10] = -(far + near) / (far - near);
                te[14] = (-2. * far * near) / (far - near);
                te[11] = -1.;
            }
            Camera::Orthographic(o) => {
                let dx = (o.right - o.left) / (2. * o.zoom);
                let dy = (o.top - o.bottom) / (2. * o.zoom);
                let cx = (o.right + o.left) / 2.;
                let cy = (o.top + o.bottom) / 2.;
                let (mut left, mut right, mut top, mut bottom) =
                    (cx - dx, cx + dx, cy + dy, cy - dy);
                if let Some(v) = o.view {
                    let scale_w = (o.right - o.left) / v.full_width / o.zoom;
                    let scale_h = (o.top - o.bottom) / v.full_height / o.zoom;
                    left += scale_w * v.offset_x;
                    right = left + scale_w * v.width;
                    top -= scale_h * v.offset_y;
                    bottom = top - scale_h * v.height;
                }
                let w = 1. / (right - left);
                let h = 1. / (top - bottom);
                let p = 1. / (o.far - o.near);
                te[0] = 2. * w;
                te[12] = -((right + left) * w);
                te[5] = 2. * h;
                te[13] = -((top + bottom) * h);
                te[10] = -2. * p;
                te[14] = -((o.far + o.near) * p);
                te[15] = 1.;
            }
        }
        te
    }
    /// Frustum.setFromProjectionMatrix( m ) in WebGL clip space.
    fn frustum(me: &[f64; 16]) -> [V4; 6] {
        let plane = |a: f64, b: f64, c: f64, d: f64| {
            let inverse = 1. / (a * a + b * b + c * c).sqrt();
            [a * inverse, b * inverse, c * inverse, d * inverse]
        };
        [
            plane(
                me[3] - me[0],
                me[7] - me[4],
                me[11] - me[8],
                me[15] - me[12],
            ),
            plane(
                me[3] + me[0],
                me[7] + me[4],
                me[11] + me[8],
                me[15] + me[12],
            ),
            plane(
                me[3] + me[1],
                me[7] + me[5],
                me[11] + me[9],
                me[15] + me[13],
            ),
            plane(
                me[3] - me[1],
                me[7] - me[5],
                me[11] - me[9],
                me[15] - me[13],
            ),
            plane(
                me[3] - me[2],
                me[7] - me[6],
                me[11] - me[10],
                me[15] - me[14],
            ),
            plane(
                me[3] + me[2],
                me[7] + me[6],
                me[11] + me[10],
                me[15] + me[14],
            ),
        ]
    }
    /// Frustum.intersectsSphere() of a sphere moved by Sphere.applyMatrix4().
    fn intersects_sphere(planes: &[V4; 6], center: V3, radius: f64, m: &[f64; 16]) -> bool {
        let center = apply3(m, center);
        let sx = m[0] * m[0] + m[1] * m[1] + m[2] * m[2];
        let sy = m[4] * m[4] + m[5] * m[5] + m[6] * m[6];
        let sz = m[8] * m[8] + m[9] * m[9] + m[10] * m[10];
        let radius = radius * sx.max(sy).max(sz).sqrt();
        planes.iter().all(|p| {
            let distance = p[0] * center[0] + p[1] * center[1] + p[2] * center[2] + p[3];
            distance >= -radius
        })
    }
    /// An attribute's array as numbers, as `attribute.array` reads.
    fn numbers(a: &Attribute) -> Vec<f64> {
        match a {
            Attribute::F32(a) => a.array().iter().map(|&v| f64::from(v)).collect(),
            Attribute::I8(a) => a.array().iter().map(|&v| f64::from(v)).collect(),
            Attribute::U8(a) => a.array().iter().map(|&v| f64::from(v)).collect(),
            Attribute::I16(a) => a.array().iter().map(|&v| f64::from(v)).collect(),
            Attribute::U16(a) => a.array().iter().map(|&v| f64::from(v)).collect(),
            Attribute::I32(a) => a.array().iter().map(|&v| f64::from(v)).collect(),
            Attribute::U32(a) => a.array().iter().map(|&v| f64::from(v)).collect(),
            _ => vec![],
        }
    }
    /// BufferGeometry.computeBoundingSphere(): the box center and the
    /// farthest position from it.
    fn bounding_sphere(positions: &[f64]) -> (V3, f64) {
        let (mut min, mut max) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for p in positions.as_chunks::<3>().0 {
            for k in 0..3 {
                min[k] = min[k].min(p[k]);
                max[k] = max[k].max(p[k]);
            }
        }
        let center = if max[0] < min[0] || max[1] < min[1] || max[2] < min[2] {
            [0.; 3]
        } else {
            [
                (min[0] + max[0]) * 0.5,
                (min[1] + max[1]) * 0.5,
                (min[2] + max[2]) * 0.5,
            ]
        };
        let mut max_radius_sq = 0f64;
        for p in positions.as_chunks::<3>().0 {
            let d = sub(center, [p[0], p[1], p[2]]);
            max_radius_sq = max_radius_sq.max(dot(d, d));
        }
        (center, max_radius_sq.sqrt())
    }

    /// RenderableVertex.
    #[derive(Clone, Copy, Default)]
    struct Vertex {
        position: V3,
        world: V3,
        screen: V4,
        visible: bool,
    }
    /// How a sprite or point is filled.
    #[derive(Clone)]
    struct Fill {
        color: Color,
        opacity: f64,
        /// PointsMaterial's size.
        size: Option<f64>,
    }
    enum Item {
        Face {
            v: [Vertex; 3],
            normal_model: V3,
            color: Color,
            material: Arc<Material>,
        },
        Line {
            v: [Vertex; 2],
            material: Arc<Material>,
        },
        Sprite {
            x: f64,
            y: f64,
            scale: [f64; 2],
            fill: Fill,
        },
        Object {
            #[allow(dead_code)]
            object: Object3D,
            x: f64,
            y: f64,
        },
    }
    struct Element {
        id: usize,
        z: f64,
        render_order: i32,
        item: Item,
    }
    impl Element {
        fn opacity(&self) -> Option<f64> {
            match &self.item {
                Item::Face { material, .. } | Item::Line { material, .. } => {
                    Some(material.properties().opacity)
                }
                Item::Sprite { fill, .. } => Some(fill.opacity),
                Item::Object { .. } => None,
            }
        }
    }
    /// painterSort: render order, then far to near, then object id.
    fn painter(a: &Element, b: &Element) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        if a.render_order != b.render_order {
            a.render_order.cmp(&b.render_order)
        } else if a.z != b.z {
            b.z.partial_cmp(&a.z).unwrap_or(Ordering::Equal)
        } else {
            a.id.cmp(&b.id)
        }
    }
    /// A clip vertex: one of the triangle's three, or a pooled intersection
    /// ( the pool's entries are reused within a clip, as the original's are ).
    #[derive(Clone, Copy)]
    enum ClipRef {
        Input(usize),
        Pool(usize),
    }

    /// The number of projected vertices and drawn faces of the last render.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct SVGRenderInfo {
        pub vertices: usize,
        pub faces: usize,
    }

    /// SVGRenderer: an `<svg>` element ( `domElement` ) the scene is drawn into
    /// as paths, its size, quality and the original's options.
    pub struct SVGRenderer {
        svg: web_sys::Element,
        width: f64,
        height: f64,
        precision: Option<usize>,
        quality: u32,
        clear_color: Color,
        /// The last face color: a face of an unlit-unknown material keeps it.
        color: Color,
        path_pool: Vec<web_sys::Element>,
        clip_pool: Vec<V4>,
        spheres: HashMap<usize, (Arc<BufferGeometry>, V3, f64)>,
        pub auto_clear: bool,
        pub sort_objects: bool,
        pub sort_elements: bool,
        /// Fractional pixels faces grow by against anti-aliasing gaps.
        pub overdraw: f64,
        /// `THREE.ColorManagement.enabled`: false writes colors unconverted.
        pub color_management: bool,
        pub info: SVGRenderInfo,
    }
    impl SVGRenderer {
        pub fn new() -> Result<Self> {
            let svg = web_sys::window()
                .and_then(|w| w.document())
                .ok_or(Error::Invalid("document"))?
                .create_element_ns(Some(SVG_NS), "svg")
                .map_err(|_| Error::Invalid("svg element"))?;
            Ok(Self {
                svg,
                width: 0.,
                height: 0.,
                precision: None,
                quality: 1,
                clear_color: Color::BLACK,
                color: Color::WHITE,
                path_pool: vec![],
                clip_pool: vec![],
                spheres: HashMap::new(),
                auto_clear: true,
                sort_objects: true,
                sort_elements: true,
                overdraw: 0.5,
                color_management: true,
                info: SVGRenderInfo::default(),
            })
        }
        pub fn dom_element(&self) -> &web_sys::Element {
            &self.svg
        }
        /// setQuality( 'high' | 'low' ): low draws crisp edges.
        pub fn set_quality(&mut self, quality: &str) {
            match quality {
                "high" => self.quality = 1,
                "low" => self.quality = 0,
                _ => {}
            }
        }
        pub fn set_clear_color(&mut self, color: Color) {
            self.clear_color = color;
        }
        pub fn set_pixel_ratio(&mut self, _ratio: f64) {}
        pub fn set_size(&mut self, width: f64, height: f64) {
            self.width = width;
            self.height = height;
            let (hw, hh) = (width / 2., height / 2.);
            let _ = self.svg.set_attribute(
                "viewBox",
                &format!(
                    "{} {} {} {}",
                    js_number(-hw),
                    js_number(-hh),
                    js_number(width),
                    js_number(height)
                ),
            );
            let _ = self.svg.set_attribute("width", &js_number(width));
            let _ = self.svg.set_attribute("height", &js_number(height));
        }
        pub fn get_size(&self) -> crate::css2d::Size {
            crate::css2d::Size {
                width: self.width,
                height: self.height,
            }
        }
        /// setPrecision( digits ): path numbers with that many decimals.
        pub fn set_precision(&mut self, precision: Option<usize>) {
            self.precision = precision;
        }
        fn remove_child_nodes(&self) {
            while let Some(child) = self.svg.first_child() {
                let _ = self.svg.remove_child(&child);
            }
        }
        fn set_background(&self, color: Color) {
            use wasm_bindgen::JsCast;
            if let Some(svg) = self.svg.dyn_ref::<web_sys::SvgElement>() {
                let _ = svg.style().set_property(
                    "background-color",
                    &color_style(color, self.color_management),
                );
            }
        }
        /// clear(): no paths, the clear color behind.
        pub fn clear(&mut self) {
            self.remove_child_nodes();
            self.set_background(self.clear_color);
        }
        fn convert(&self, c: f64) -> String {
            match self.precision {
                Some(p) => format!("{c:.p$}"),
                None => js_number(c),
            }
        }
        /// render( scene, camera ): the scene's roots projected through the camera.
        /// As in the original, SVGObjects are placed with the camera's world
        /// matrix as it was before the render updates it; faces, lines and
        /// sprites use the updated one.
        pub fn render(&mut self, scene: &mut Scene, camera: Object3D) -> Result<()> {
            let before = scene.get(camera)?.matrix_world.to_cols_array();
            scene.update()?;
            let n = scene.get(camera)?;
            let NodeKind::Camera(kind) = &n.kind else {
                return Err(Error::Invalid("SVGRenderer.render: not a camera"));
            };
            let (kind, world) = (kind.clone(), n.matrix_world.to_cols_array());
            self.render_with(scene, &kind, &before, &world, Some(camera))
        }
        /// render( scene, camera ) with a camera of another scene, which the
        /// render updates ( camera.updateMatrixWorld() ) as the original does.
        pub fn render_from(
            &mut self,
            scene: &mut Scene,
            camera_scene: &mut Scene,
            camera: Object3D,
        ) -> Result<()> {
            let before = camera_scene.get(camera)?.matrix_world.to_cols_array();
            scene.update()?;
            camera_scene.update_world_matrix(camera, false, true)?;
            let n = camera_scene.get(camera)?;
            let NodeKind::Camera(kind) = &n.kind else {
                return Err(Error::Invalid("SVGRenderer.render: not a camera"));
            };
            self.render_with(scene, kind, &before, &n.matrix_world.to_cols_array(), None)
        }
        fn render_with(
            &mut self,
            scene: &Scene,
            camera_kind: &Camera,
            camera_before: &[f64; 16],
            camera_world: &[f64; 16],
            camera: Option<Object3D>,
        ) -> Result<()> {
            if scene.background_alpha > 0. {
                self.remove_child_nodes();
                self.set_background(scene.background);
            } else if self.auto_clear {
                self.clear();
            }
            self.info = SVGRenderInfo::default();
            let p = projection(camera_kind);
            // render()'s _viewProjectionMatrix, from matrixWorldInverse before
            // projectScene() updates the camera.
            let svg_view_projection = multiply(&p, &view_matrix(camera_before));
            let view = view_matrix(camera_world);
            let view_projection = multiply(&p, &view);
            let normal_view = normal_matrix(&view);
            let (mut elements, lights) = self.project(scene, camera, &p, &view_projection)?;
            let (hw, hh) = (self.width / 2., self.height / 2.);
            let mut items: Vec<Element> = elements
                .drain(..)
                .filter(|e| e.opacity().is_some_and(|o| o != 0.))
                .collect();
            // SVGObjects, in traverseVisible() order.
            let mut svg_objects = 0;
            for root in scene.roots() {
                for h in scene.traverse(root, true)? {
                    let n = scene.get(h)?;
                    if !matches!(n.kind, NodeKind::SVGObject(_)) {
                        continue;
                    }
                    let w = n.matrix_world.to_cols_array();
                    let v = apply3(&svg_view_projection, [w[12], w[13], w[14]]);
                    if v[2] < -1. || v[2] > 1. {
                        continue;
                    }
                    svg_objects += 1;
                    items.push(Element {
                        id: h.index(),
                        z: v[2],
                        render_order: n.render_order,
                        item: Item::Object {
                            object: h,
                            x: v[0] * hw,
                            y: -v[1] * hh,
                        },
                    });
                }
            }
            if self.sort_elements && svg_objects > 0 {
                // renderSort: render order, then far to near.
                items.sort_by(|a, b| {
                    if a.render_order != b.render_order {
                        a.render_order.cmp(&b.render_order)
                    } else {
                        b.z.partial_cmp(&a.z).unwrap_or(std::cmp::Ordering::Equal)
                    }
                });
            }
            let mut path = String::new();
            let mut style = String::new();
            let mut path_count = 0;
            let (min, max) = ([-hw, -hh], [hw, hh]);
            let intersects = |points: &[[f64; 2]]| {
                let mut lo = [f64::INFINITY; 2];
                let mut hi = [f64::NEG_INFINITY; 2];
                for p in points {
                    for k in 0..2 {
                        lo[k] = lo[k].min(p[k]);
                        hi[k] = hi[k].max(p[k]);
                    }
                }
                !(hi[0] < min[0] || lo[0] > max[0] || hi[1] < min[1] || lo[1] > max[1])
            };
            for item in items {
                let (new_style, new_path) = match item.item {
                    Item::Object { object, x, y } => {
                        self.flush(&mut path, &mut style, &mut path_count)?;
                        if let NodeKind::SVGObject(o) = &scene.get(object)?.kind
                            && let Some(node) = &o.node.element
                        {
                            let _ = node.set_attribute(
                                "transform",
                                &format!("translate({},{})", js_number(x), js_number(y)),
                            );
                            let _ = self.svg.append_child(node);
                        }
                        continue;
                    }
                    Item::Sprite { x, y, scale, fill } => {
                        let (x, y) = (x * hw, y * -hh);
                        let (mut sx, mut sy) = (scale[0] * hw, scale[1] * hh);
                        if let Some(size) = fill.size {
                            sx *= size;
                            sy *= size;
                        }
                        let path = format!(
                            "M{},{}h{}v{}h{}z",
                            self.convert(x - sx * 0.5),
                            self.convert(y - sy * 0.5),
                            self.convert(sx),
                            self.convert(sy),
                            self.convert(-sx)
                        );
                        let style = format!(
                            "fill:{};fill-opacity:{}",
                            color_style(fill.color, self.color_management),
                            js_number(fill.opacity)
                        );
                        (style, path)
                    }
                    Item::Line { mut v, material } => {
                        for vertex in &mut v {
                            vertex.screen[0] *= hw;
                            vertex.screen[1] *= -hh;
                        }
                        let (a, b) = (v[0].screen, v[1].screen);
                        if !intersects(&[[a[0], a[1]], [b[0], b[1]]]) {
                            continue;
                        }
                        let Material::Line(m) = &*material else {
                            continue;
                        };
                        let path = format!(
                            "M{},{}L{},{}",
                            self.convert(a[0]),
                            self.convert(a[1]),
                            self.convert(b[0]),
                            self.convert(b[1])
                        );
                        let mut style = format!(
                            "fill:none;stroke:{};stroke-opacity:{};stroke-width:{};stroke-linecap:round",
                            color_style(m.properties.color, self.color_management),
                            js_number(m.properties.opacity),
                            js_number(m.linewidth)
                        );
                        if let Some(dash) = &m.dash {
                            style.push_str(&format!(
                                ";stroke-dasharray:{},{}",
                                js_number(dash.size),
                                js_number(dash.gap)
                            ));
                        }
                        (style, path)
                    }
                    Item::Face {
                        mut v,
                        normal_model,
                        color,
                        material,
                    } => {
                        for vertex in &mut v {
                            vertex.screen[0] *= hw;
                            vertex.screen[1] *= -hh;
                        }
                        if self.overdraw > 0. {
                            for (i, j) in [(0, 1), (1, 2), (2, 0)] {
                                let mut x = v[j].screen[0] - v[i].screen[0];
                                let mut y = v[j].screen[1] - v[i].screen[1];
                                let det = x * x + y * y;
                                if det == 0. {
                                    continue;
                                }
                                let idet = self.overdraw / det.sqrt();
                                x *= idet;
                                y *= idet;
                                v[j].screen[0] += x;
                                v[j].screen[1] += y;
                                v[i].screen[0] -= x;
                                v[i].screen[1] -= y;
                            }
                        }
                        let points = v.map(|v| [v.screen[0], v.screen[1]]);
                        if !intersects(&points) {
                            continue;
                        }
                        self.face(&v, normal_model, color, &material, &lights, &normal_view)
                    }
                };
                if style == new_style {
                    path.push_str(&new_path);
                } else {
                    self.flush(&mut path, &mut style, &mut path_count)?;
                    style = new_style;
                    path = new_path;
                }
            }
            self.flush(&mut path, &mut style, &mut path_count)?;
            Ok(())
        }
        /// flushPath(): the accumulated path as the next pooled `<path>`.
        fn flush(
            &mut self,
            path: &mut String,
            style: &mut String,
            count: &mut usize,
        ) -> Result<()> {
            if !path.is_empty() {
                if *count == self.path_pool.len() {
                    let node = web_sys::window()
                        .and_then(|w| w.document())
                        .ok_or(Error::Invalid("document"))?
                        .create_element_ns(Some(SVG_NS), "path")
                        .map_err(|_| Error::Invalid("svg path"))?;
                    if self.quality == 0 {
                        let _ = node.set_attribute("shape-rendering", "crispEdges");
                    }
                    self.path_pool.push(node);
                }
                let node = &self.path_pool[*count];
                *count += 1;
                let _ = node.set_attribute("d", path);
                let _ = node.set_attribute("style", style);
                let _ = self.svg.append_child(node);
            }
            path.clear();
            style.clear();
            Ok(())
        }
        /// renderFace3(): the face's path and its material's fill or stroke.
        fn face(
            &mut self,
            v: &[Vertex; 3],
            normal_model: V3,
            element_color: Color,
            material: &Material,
            lights: &[(Light, V3)],
            normal_view: &[f64; 9],
        ) -> (String, String) {
            self.info.vertices += 3;
            self.info.faces += 1;
            let path = format!(
                "M{},{}L{},{}L{},{}z",
                self.convert(v[0].screen[0]),
                self.convert(v[0].screen[1]),
                self.convert(v[1].screen[0]),
                self.convert(v[1].screen[1]),
                self.convert(v[2].screen[0]),
                self.convert(v[2].screen[1])
            );
            let properties = material.properties();
            let emissive = match material {
                Material::Lambert(m) => Some(m.emissive),
                Material::Phong(m) => Some(m.emissive),
                Material::Standard(m) => Some(m.emissive),
                Material::Physical(m) => Some(m.base.emissive),
                _ => None,
            };
            if let Material::Basic(_) = material {
                self.color = properties.color;
                if properties.vertex_colors {
                    self.color = Color(self.color.0 * element_color.0);
                }
            } else if let Some(emissive) = emissive {
                let mut diffuse = properties.color;
                if properties.vertex_colors {
                    diffuse = Color(diffuse.0 * element_color.0);
                }
                // calculateLights(): the ambient sum, then calculateLight().
                let mut color = [0.; 3];
                for (light, _) in lights {
                    if let Light::Ambient { color: c, .. } = light {
                        color[0] += c.0.x;
                        color[1] += c.0.y;
                        color[2] += c.0.z;
                    }
                }
                let centroid = {
                    let s = [
                        v[0].world[0] + v[1].world[0] + v[2].world[0],
                        v[0].world[1] + v[1].world[1] + v[2].world[1],
                        v[0].world[2] + v[1].world[2] + v[2].world[2],
                    ];
                    [s[0] / 3., s[1] / 3., s[2] / 3.]
                };
                for (light, position) in lights {
                    match light {
                        Light::Directional {
                            color: c,
                            intensity,
                            ..
                        } => {
                            let mut amount = dot(normal_model, normalize(*position));
                            if amount <= 0. {
                                continue;
                            }
                            amount *= intensity;
                            color[0] += c.0.x * amount;
                            color[1] += c.0.y * amount;
                            color[2] += c.0.z * amount;
                        }
                        Light::Point {
                            color: c,
                            intensity,
                            distance,
                            ..
                        } => {
                            let mut amount = dot(normal_model, normalize(sub(*position, centroid)));
                            if amount <= 0. {
                                continue;
                            }
                            let d = sub(centroid, *position);
                            amount *= if *distance == 0. {
                                1.
                            } else {
                                1. - (dot(d, d).sqrt() / distance).min(1.)
                            };
                            if amount == 0. {
                                continue;
                            }
                            amount *= intensity;
                            color[0] += c.0.x * amount;
                            color[1] += c.0.y * amount;
                            color[2] += c.0.z * amount;
                        }
                        _ => {}
                    }
                }
                self.color = Color(
                    crate::math::Vector3::new(
                        color[0] * diffuse.0.x,
                        color[1] * diffuse.0.y,
                        color[2] * diffuse.0.z,
                    ) + emissive.0,
                );
            } else if let Material::Normal(_) = material {
                let n = normalize(apply_matrix3(normal_view, normal_model));
                self.color = Color::linear(n[0] * 0.5 + 0.5, n[1] * 0.5 + 0.5, n[2] * 0.5 + 0.5);
            }
            let style = if properties.wireframe {
                format!(
                    "fill:none;stroke:{};stroke-opacity:{};stroke-width:1;stroke-linecap:round;stroke-linejoin:round",
                    color_style(self.color, self.color_management),
                    js_number(properties.opacity)
                )
            } else {
                format!(
                    "fill:{};fill-opacity:{}",
                    color_style(self.color, self.color_management),
                    js_number(properties.opacity)
                )
            };
            (style, path)
        }
        /// Projector.projectScene(): the visible objects sorted, then their
        /// faces, lines and sprites, and the lights in scene order.
        #[allow(clippy::type_complexity)]
        fn project(
            &mut self,
            scene: &Scene,
            camera: Option<Object3D>,
            p: &[f64; 16],
            view_projection: &[f64; 16],
        ) -> Result<(Vec<Element>, Vec<(Light, V3)>)> {
            let planes = frustum(view_projection);
            let mut objects = vec![];
            let mut lights = vec![];
            // projectObject(): an invisible object, or one whose material is
            // invisible or that lies outside the frustum, ends its branch.
            let mut stack: Vec<Object3D> = scene.roots().into_iter().rev().collect();
            while let Some(h) = stack.pop() {
                if Some(h) == camera {
                    continue;
                }
                let n = scene.get(h)?;
                if !n.visible {
                    continue;
                }
                let m = n.matrix_world.to_cols_array();
                let (drawn, descend) = match &n.kind {
                    NodeKind::Light(light) => {
                        lights.push((light.clone(), [m[12], m[13], m[14]]));
                        (false, true)
                    }
                    NodeKind::Mesh(mesh) => {
                        let shown = mesh
                            .materials
                            .first()
                            .is_none_or(|m| m.properties().visible)
                            && (!n.frustum_culled
                                || self.intersects_object(&mesh.geometry, &planes, &m));
                        (shown, shown)
                    }
                    NodeKind::Line(line) => {
                        let shown = line.material.properties().visible
                            && (!n.frustum_culled
                                || self.intersects_object(&line.geometry, &planes, &m));
                        (shown, shown)
                    }
                    NodeKind::Points(points) => {
                        let shown = points.material.properties().visible
                            && (!n.frustum_culled
                                || self.intersects_object(&points.geometry, &planes, &m));
                        (shown, shown)
                    }
                    NodeKind::Sprite(sprite) => {
                        let shown = sprite.material.properties.visible
                            && (!n.frustum_culled
                                || intersects_sphere(
                                    &planes,
                                    [0.; 3],
                                    std::f64::consts::FRAC_1_SQRT_2,
                                    &m,
                                ));
                        (shown, shown)
                    }
                    _ => (false, true),
                };
                if drawn {
                    let z = apply3(view_projection, [m[12], m[13], m[14]])[2];
                    objects.push(Element {
                        id: h.index(),
                        z,
                        render_order: n.render_order,
                        item: Item::Object {
                            object: h,
                            x: 0.,
                            y: 0.,
                        },
                    });
                }
                if descend {
                    stack.extend(n.children().iter().rev());
                }
            }
            if self.sort_objects {
                objects.sort_by(painter);
            }
            let mut elements = vec![];
            for o in &objects {
                let Item::Object { object: h, .. } = o.item else {
                    continue;
                };
                let n = scene.get(h)?;
                let model = n.matrix_world.to_cols_array();
                let id = h.index();
                let render_order = n.render_order;
                match &n.kind {
                    NodeKind::Mesh(mesh) => {
                        let g = &*mesh.geometry;
                        let Some(position) = g.attributes.get("position") else {
                            continue;
                        };
                        let mut positions = numbers(position);
                        if let Some(targets) = g.morph_attributes.get("position") {
                            let base = positions.clone();
                            for (t, target) in targets.iter().enumerate() {
                                let influence = n.morph_weights.get(t).copied().unwrap_or(0.);
                                if influence == 0. {
                                    continue;
                                }
                                let target = numbers(target);
                                for (i, value) in positions.iter_mut().enumerate() {
                                    let offset = target.get(i).copied().unwrap_or(0.);
                                    *value += if g.morph_targets_relative {
                                        offset * influence
                                    } else {
                                        (offset - base[i]) * influence
                                    };
                                }
                            }
                        }
                        let colors = g.attributes.get("color").map(numbers).unwrap_or_default();
                        let normal_matrix = normal_matrix(&model);
                        let vertices: Vec<Vertex> = positions
                            .as_chunks::<3>()
                            .0
                            .iter()
                            .map(|p| {
                                let position = [p[0], p[1], p[2]];
                                let world = apply3(&model, position);
                                let mut screen =
                                    apply4(view_projection, [world[0], world[1], world[2], 1.]);
                                let inv_w = 1. / screen[3];
                                screen[0] *= inv_w;
                                screen[1] *= inv_w;
                                screen[2] *= inv_w;
                                let visible = (-1. ..=1.).contains(&screen[0])
                                    && (-1. ..=1.).contains(&screen[1])
                                    && (-1. ..=1.).contains(&screen[2]);
                                Vertex {
                                    position,
                                    world,
                                    screen,
                                    visible,
                                }
                            })
                            .collect();
                        let material_for = |group: Option<usize>| -> Option<Arc<Material>> {
                            if mesh.materials.len() > 1 {
                                group.and_then(|i| mesh.materials.get(i).cloned())
                            } else {
                                mesh.materials.first().cloned()
                            }
                        };
                        let mut triangles: Vec<([usize; 3], Arc<Material>)> = vec![];
                        let count = positions.len() / 3;
                        let mut push =
                            |range: std::ops::Range<usize>,
                             material: Arc<Material>,
                             index: Option<&Vec<u32>>| {
                                let mut i = range.start;
                                while i < range.end {
                                    let t = match index {
                                        Some(index) => [
                                            index.get(i).copied().unwrap_or(0) as usize,
                                            index.get(i + 1).copied().unwrap_or(0) as usize,
                                            index.get(i + 2).copied().unwrap_or(0) as usize,
                                        ],
                                        None => [i, i + 1, i + 2],
                                    };
                                    triangles.push((t, material.clone()));
                                    i += 3;
                                }
                            };
                        let index = g.index.as_ref();
                        if !g.groups.is_empty() {
                            for group in &g.groups {
                                if let Some(material) = material_for(Some(group.material_index)) {
                                    push(group.start..group.start + group.count, material, index);
                                }
                            }
                        } else if let Some(material) = material_for(None) {
                            let end = index.map_or(count, |i| i.len());
                            push(0..end, material, index);
                        }
                        for (t, material) in triangles {
                            if t.iter().any(|&i| i >= vertices.len()) {
                                continue;
                            }
                            self.push_triangle(
                                &mut elements,
                                [vertices[t[0]], vertices[t[1]], vertices[t[2]]],
                                &colors,
                                t[0],
                                &material,
                                &normal_matrix,
                                id,
                                render_order,
                            );
                        }
                    }
                    NodeKind::Line(line) => {
                        let mvp = multiply(view_projection, &model);
                        let g = &*line.geometry;
                        let Some(position) = g.attributes.get("position") else {
                            continue;
                        };
                        let positions = numbers(position);
                        let vertices: Vec<V3> = positions
                            .as_chunks::<3>()
                            .0
                            .iter()
                            .map(|p| [p[0], p[1], p[2]])
                            .collect();
                        let worlds: Vec<V3> = vertices.iter().map(|&p| apply3(&model, p)).collect();
                        let mut pairs = vec![];
                        if let Some(index) = &g.index {
                            let mut i = 0;
                            while i + 1 < index.len() {
                                pairs.push((index[i] as usize, index[i + 1] as usize));
                                i += 2;
                            }
                        } else {
                            let step = if line.segments { 2 } else { 1 };
                            let mut i = 0;
                            while i + 1 < vertices.len() {
                                pairs.push((i, i + 1));
                                i += step;
                            }
                        }
                        for (a, b) in pairs {
                            if a >= vertices.len() || b >= vertices.len() {
                                continue;
                            }
                            let pa = vertices[a];
                            let pb = vertices[b];
                            let mut s1 = apply4(&mvp, [pa[0], pa[1], pa[2], 1.]);
                            let mut s2 = apply4(&mvp, [pb[0], pb[1], pb[2], 1.]);
                            if !clip_line(&mut s1, &mut s2) {
                                continue;
                            }
                            let i1 = 1. / s1[3];
                            let i2 = 1. / s2[3];
                            let s1 = s1.map(|c| c * i1);
                            let s2 = s2.map(|c| c * i2);
                            elements.push(Element {
                                id,
                                z: s1[2].max(s2[2]),
                                render_order,
                                item: Item::Line {
                                    v: [
                                        Vertex {
                                            world: worlds[a],
                                            screen: s1,
                                            ..Default::default()
                                        },
                                        Vertex {
                                            world: worlds[b],
                                            screen: s2,
                                            ..Default::default()
                                        },
                                    ],
                                    material: line.material.clone(),
                                },
                            });
                        }
                    }
                    NodeKind::Points(points) => {
                        let mvp = multiply(view_projection, &model);
                        let Some(position) = points.geometry.attributes.get("position") else {
                            continue;
                        };
                        let (color, opacity, size) = match &*points.material {
                            Material::Points(m) => {
                                (m.properties.color, m.properties.opacity, Some(m.size))
                            }
                            m => (m.properties().color, m.properties().opacity, None),
                        };
                        for q in numbers(position).as_chunks::<3>().0 {
                            let v = apply4(&mvp, [q[0], q[1], q[2], 1.]);
                            push_point(
                                &mut elements,
                                v,
                                n.scale.to_array(),
                                p,
                                Fill {
                                    color,
                                    opacity,
                                    size,
                                },
                                id,
                                render_order,
                            );
                        }
                    }
                    NodeKind::Sprite(sprite) => {
                        let v = apply4(view_projection, [model[12], model[13], model[14], 1.]);
                        push_point(
                            &mut elements,
                            v,
                            n.scale.to_array(),
                            p,
                            Fill {
                                color: sprite.material.properties.color,
                                opacity: sprite.material.properties.opacity,
                                size: None,
                            },
                            id,
                            render_order,
                        );
                    }
                    _ => {}
                }
            }
            if self.sort_elements {
                elements.sort_by(painter);
            }
            Ok((elements, lights))
        }
        fn intersects_object(
            &mut self,
            geometry: &Arc<BufferGeometry>,
            planes: &[V4; 6],
            m: &[f64; 16],
        ) -> bool {
            let key = Arc::as_ptr(geometry) as usize;
            let (center, radius) = match self.spheres.get(&key) {
                Some((_, c, r)) => (*c, *r),
                None => {
                    let positions = geometry
                        .attributes
                        .get("position")
                        .map(numbers)
                        .unwrap_or_default();
                    let (c, r) = bounding_sphere(&positions);
                    self.spheres.insert(key, (geometry.clone(), c, r));
                    (c, r)
                }
            };
            intersects_sphere(planes, center, radius, m)
        }
        /// RenderList.pushTriangle(): the triangle whole, clipped against the
        /// near and far planes into a fan, or not at all.
        #[allow(clippy::too_many_arguments)]
        fn push_triangle(
            &mut self,
            elements: &mut Vec<Element>,
            v: [Vertex; 3],
            colors: &[f64],
            a: usize,
            material: &Arc<Material>,
            normal_matrix: &[f64; 9],
            id: usize,
            render_order: i32,
        ) {
            let w = v.map(|v| v.screen[3]);
            let near = [0, 1, 2].map(|i| w[i] * (v[i].screen[2] + 1.));
            let far = [0, 1, 2].map(|i| w[i] * (1. - v[i].screen[2]));
            if near.iter().all(|&d| d < 0.) || far.iter().all(|&d| d < 0.) {
                return;
            }
            let double = material.properties().side == Side::Double;
            // face normal: ( v3 − v2 ) × ( v1 − v2 ).
            let normal_model = {
                let a = sub(v[2].position, v[1].position);
                let b = sub(v[0].position, v[1].position);
                let c = [
                    a[1] * b[2] - a[2] * b[1],
                    a[2] * b[0] - a[0] * b[2],
                    a[0] * b[1] - a[1] * b[0],
                ];
                normalize(apply_matrix3(normal_matrix, c))
            };
            let color = if material.properties().vertex_colors {
                Color::linear(
                    colors.get(a * 3).copied().unwrap_or(f64::NAN),
                    colors.get(a * 3 + 1).copied().unwrap_or(f64::NAN),
                    colors.get(a * 3 + 2).copied().unwrap_or(f64::NAN),
                )
            } else {
                Color::WHITE
            };
            let mut push = |face: [Vertex; 3]| {
                let z = (face[0].screen[2] + face[1].screen[2] + face[2].screen[2]) / 3.;
                elements.push(Element {
                    id,
                    z,
                    render_order,
                    item: Item::Face {
                        v: face,
                        normal_model,
                        color,
                        material: material.clone(),
                    },
                });
            };
            if near.iter().chain(&far).all(|&d| d >= 0.) {
                if !visible(&v) {
                    return;
                }
                if double || back_facing(&v) {
                    push(v);
                }
                return;
            }
            let clip = v.map(|v| {
                let w = v.screen[3];
                [v.screen[0] * w, v.screen[1] * w, v.screen[2] * w, w]
            });
            let polygon = self.clip_triangle(clip);
            if polygon.len() < 3 {
                return;
            }
            let screen: Vec<Vertex> = polygon
                .iter()
                .map(|c| {
                    let inv_w = 1. / c[3];
                    Vertex {
                        position: [0.; 3],
                        world: v[0].world,
                        screen: [c[0] * inv_w, c[1] * inv_w, c[2] * inv_w, 1.],
                        visible: true,
                    }
                })
                .collect();
            for i in 1..screen.len() - 1 {
                let face = [screen[0], screen[i], screen[i + 1]];
                if double || back_facing(&face) {
                    push(face);
                }
            }
        }
        /// clipTriangle(): Sutherland–Hodgman against w ± z ≥ 0, the
        /// intersections in a pool reused across passes as the original's.
        fn clip_triangle(&mut self, input: [V4; 3]) -> Vec<V4> {
            let mut current = vec![ClipRef::Input(0), ClipRef::Input(1), ClipRef::Input(2)];
            for sign in [1., -1.] {
                if current.is_empty() {
                    break;
                }
                let n = current.len();
                let mut output = vec![];
                for i in 0..n {
                    let get = |r: ClipRef, pool: &Vec<V4>| match r {
                        ClipRef::Input(k) => input[k],
                        ClipRef::Pool(k) => pool[k],
                    };
                    let v1 = get(current[i], &self.clip_pool);
                    let v2 = get(current[(i + 1) % n], &self.clip_pool);
                    let d1 = sign * v1[2] + v1[3];
                    let d2 = sign * v2[2] + v2[3];
                    let (in1, in2) = (d1 >= 0., d2 >= 0.);
                    if in1 {
                        output.push(current[i]);
                    }
                    if in1 != in2 {
                        let t = d1 / (d1 - d2);
                        let k = output.len();
                        while self.clip_pool.len() <= k {
                            self.clip_pool.push([0.; 4]);
                        }
                        // lerpVectors( v1, v2, t ), component by component.
                        self.clip_pool[k] = std::array::from_fn(|c| v1[c] + (v2[c] - v1[c]) * t);
                        output.push(ClipRef::Pool(k));
                    }
                }
                current = output;
            }
            current
                .into_iter()
                .map(|r| match r {
                    ClipRef::Input(k) => input[k],
                    ClipRef::Pool(k) => self.clip_pool[k],
                })
                .collect()
        }
    }
    /// checkTriangleVisibility(): a vertex inside, or the screen box meeting
    /// the clip cube.
    fn visible(v: &[Vertex; 3]) -> bool {
        if v.iter().any(|v| v.visible) {
            return true;
        }
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];
        for v in v {
            for k in 0..3 {
                lo[k] = lo[k].min(v.screen[k]);
                hi[k] = hi[k].max(v.screen[k]);
            }
        }
        (0..3).all(|k| !(hi[k] < -1. || lo[k] > 1.))
    }
    /// checkBackfaceCulling(): true for a front ( counter-clockwise ) face.
    fn back_facing(v: &[Vertex; 3]) -> bool {
        let (a, b, c) = (v[0].screen, v[1].screen, v[2].screen);
        ((c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])) < 0.
    }
    /// clipLine(): the segment cut to the near and far planes.
    fn clip_line(s1: &mut V4, s2: &mut V4) -> bool {
        let (mut alpha1, mut alpha2) = (0f64, 1f64);
        let bc1near = s1[2] + s1[3];
        let bc2near = s2[2] + s2[3];
        let bc1far = -s1[2] + s1[3];
        let bc2far = -s2[2] + s2[3];
        if bc1near >= 0. && bc2near >= 0. && bc1far >= 0. && bc2far >= 0. {
            return true;
        }
        if (bc1near < 0. && bc2near < 0.) || (bc1far < 0. && bc2far < 0.) {
            return false;
        }
        if bc1near < 0. {
            alpha1 = alpha1.max(bc1near / (bc1near - bc2near));
        } else if bc2near < 0. {
            alpha2 = alpha2.min(bc1near / (bc1near - bc2near));
        }
        if bc1far < 0. {
            alpha1 = alpha1.max(bc1far / (bc1far - bc2far));
        } else if bc2far < 0. {
            alpha2 = alpha2.min(bc1far / (bc1far - bc2far));
        }
        if alpha2 < alpha1 {
            return false;
        }
        // s1.lerp( s2, alpha1 ); s2.lerp( s1, 1 − alpha2 ) with the moved s1.
        for c in 0..4 {
            s1[c] += (s2[c] - s1[c]) * alpha1;
        }
        for c in 0..4 {
            s2[c] += (s1[c] - s2[c]) * (1. - alpha2);
        }
        true
    }
    /// pushPoint(): a sprite or point inside the depth range, sized by its
    /// scale over the projection's unit offsets.
    #[allow(clippy::too_many_arguments)]
    fn push_point(
        elements: &mut Vec<Element>,
        v: V4,
        scale: V3,
        p: &[f64; 16],
        fill: Fill,
        id: usize,
        render_order: i32,
    ) {
        let inv_w = 1. / v[3];
        let z = v[2] * inv_w;
        if !(-1. ..=1.).contains(&z) {
            return;
        }
        let x = v[0] * inv_w;
        let y = v[1] * inv_w;
        let sx = scale[0] * (x - (v[0] + p[0]) / (v[3] + p[12])).abs();
        let sy = scale[1] * (y - (v[1] + p[5]) / (v[3] + p[13])).abs();
        elements.push(Element {
            id,
            z,
            render_order,
            item: Item::Sprite {
                x,
                y,
                scale: [sx, sy],
                fill,
            },
        });
    }
}
