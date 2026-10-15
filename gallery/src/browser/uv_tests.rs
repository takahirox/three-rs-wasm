//! misc_uv_tests: UVsDebug's 1024 × 1024 canvas unwrapping of nine
//! built-in geometries, drawn with the page's Canvas 2D API from the Rust
//! geometries' uv attributes and indices.
use crate::{Error, Result, geometry::*, math::*, renderer::*, scene::*};

const SIZE: f64 = 1024.;
/// UVsDebug( geometry ): face outlines, face numbers and the a/b/c vertex
/// labels, with the same double-precision arithmetic (Vector2.divideScalar
/// multiplies by the reciprocal).
fn uvs_debug(
    document: &web_sys::Document,
    geometry: &BufferGeometry,
) -> Result<web_sys::HtmlCanvasElement> {
    use wasm_bindgen::JsCast;
    let fail = |_| Error::Invalid("uv debug canvas");
    let canvas: web_sys::HtmlCanvasElement = document
        .create_element("canvas")
        .map_err(fail)?
        .dyn_into()
        .map_err(|_| Error::Invalid("uv debug canvas"))?;
    let (width, height) = (SIZE, SIZE);
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
    let ctx: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .map_err(fail)?
        .ok_or(Error::Invalid("uv debug context"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("uv debug context"))?;
    ctx.set_line_width(1.);
    ctx.set_stroke_style_str("rgb( 63, 63, 63 )");
    ctx.set_text_align("center");
    ctx.set_fill_style_str("rgb( 255, 255, 255 )");
    ctx.fill_rect(0., 0., width, height);
    let Some(Attribute::F32(uv)) = geometry.attributes.get("uv") else {
        return Err(Error::Invalid("uv attribute"));
    };
    let uv = uv.array();
    let count = uv.len() / 2;
    let faces: Vec<[u32; 3]> = match &geometry.index {
        Some(index) => index.as_chunks::<3>().0.to_vec(),
        None => (0..count as u32 / 3)
            .map(|i| [i * 3, i * 3 + 1, i * 3 + 2])
            .collect(),
    };
    let abc = ['a', 'b', 'c'];
    for (index, face) in faces.iter().enumerate() {
        let uvs = face.map(|v| {
            let i = v as usize * 2;
            (uv[i] as f64, uv[i + 1] as f64)
        });
        ctx.begin_path();
        let (mut ax, mut ay) = (0., 0.);
        for (j, &(x, y)) in uvs.iter().enumerate() {
            ax += x;
            ay += y;
            let (px, py) = (x * (width - 2.) + 0.5, (1. - y) * (height - 2.) + 0.5);
            if j == 0 {
                ctx.move_to(px, py);
            } else {
                ctx.line_to(px, py);
            }
        }
        ctx.close_path();
        ctx.stroke();
        let scale = 1. / 3.;
        let (ax, ay) = (ax * scale, ay * scale);
        ctx.set_font("18px Arial");
        ctx.set_fill_style_str("rgb( 63, 63, 63 )");
        let label = index.to_string();
        ctx.fill_text(&label, ax * width, (1. - ay) * height)
            .map_err(fail)?;
        if ax > 0.95 {
            ctx.fill_text(&label, (ax % 1.) * width, (1. - ay) * height)
                .map_err(fail)?;
        }
        ctx.set_font("12px Arial");
        ctx.set_fill_style_str("rgb( 191, 191, 191 )");
        for (j, &(x, y)) in uvs.iter().enumerate() {
            let (bx, by) = ((ax + x) * 0.5, (ay + y) * 0.5);
            let label = format!("{}{}", abc[j], face[j]);
            ctx.fill_text(&label, bx * width, (1. - by) * height)
                .map_err(fail)?;
            if bx > 0.95 {
                ctx.fill_text(&label, (bx % 1.) * width, (1. - by) * height)
                    .map_err(fail)?;
            }
        }
    }
    Ok(canvas)
}
pub(super) struct Demo;
impl Demo {
    /// The nine test( name, geometry ) sections, appended to the page.
    pub async fn create(_s: &mut Scene, _c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or(Error::Invalid("document"))?;
        let body = document.body().ok_or(Error::Invalid("body"))?;
        let fail = |_| Error::Invalid("uv tests page");
        let container = document.create_element("div").map_err(fail)?;
        container.set_id("uv-tests");
        let points: Vec<Vector2> = (0..10)
            .map(|i| Vector2::new((i as f64 * 0.2).sin() * 15. + 50., (i as f64 - 5.) * 2.))
            .collect();
        let tau = std::f64::consts::TAU;
        let tests = [
            (
                "new THREE.PlaneGeometry( 100, 100, 4, 4 )",
                PlaneGeometry::build(100., 100., 4, 4)?,
            ),
            (
                "new THREE.SphereGeometry( 75, 12, 6 )",
                SphereGeometry::build(75., 12, 6)?,
            ),
            (
                "new THREE.IcosahedronGeometry( 30, 1 )",
                IcosahedronGeometry::build(30., 1)?,
            ),
            (
                "new THREE.OctahedronGeometry( 30, 2 )",
                OctahedronGeometry::build(30., 2)?,
            ),
            (
                "new THREE.CylinderGeometry( 25, 75, 100, 10, 5 )",
                CylinderGeometry::build(25., 75., 100., 10, 5, false, 0., tau)?,
            ),
            (
                "new THREE.BoxGeometry( 100, 100, 100, 4, 4, 4 )",
                BoxGeometry::segmented(100., 100., 100., 4, 4, 4)?,
            ),
            (
                "new THREE.LatheGeometry( points, 8 )",
                LatheGeometry::build(&points, 8, 0., tau)?,
            ),
            (
                "new THREE.TorusGeometry( 50, 20, 8, 8 )",
                TorusGeometry::build(50., 20., 8, 8, tau, 0., tau)?,
            ),
            (
                "new THREE.TorusKnotGeometry( 50, 10, 12, 6 )",
                TorusKnotGeometry::build(50., 10., 12, 6, 2, 3)?,
            ),
        ];
        for (name, geometry) in &tests {
            let d = document.create_element("div").map_err(fail)?;
            let title = document.create_element("h3").map_err(fail)?;
            title.set_text_content(Some(name));
            d.append_child(&title).map_err(fail)?;
            let canvas = uvs_debug(&document, geometry)?;
            d.append_child(&canvas).map_err(fail)?;
            container.append_child(&d).map_err(fail)?;
        }
        body.append_child(&container).map_err(fail)?;
        Ok(Self)
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
        Err(Error::Invalid("uv tests parameter"))
    }
    pub fn seek(&mut self, _t: f64) {}
}
