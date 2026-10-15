//! What the css3d pages share: their CSS3DRenderer over the canvas, the
//! fixture's Math.random, Color.getStyle() and the window size.
use crate::css3d::{CSS3DRenderer, CSS3DRendererParameters};
use crate::{Error, Result};
use wasm_bindgen::JsCast;

/// The fixture's Math.random: a 32-bit LCG seeded with 186.
pub(super) struct Random(pub u32);
impl Random {
    pub fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(self.0) / 4294967296.
    }
}
pub(super) fn document() -> Result<web_sys::Document> {
    web_sys::window()
        .and_then(|w| w.document())
        .ok_or(Error::Invalid("document"))
}
pub(super) fn element(tag: &str) -> Result<web_sys::HtmlElement> {
    document()?
        .create_element(tag)
        .map_err(|_| Error::Invalid("element"))?
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| Error::Invalid("element"))
}
pub(super) fn window_size() -> (f64, f64) {
    let w = web_sys::window();
    let get = |v: Option<wasm_bindgen::JsValue>| v.and_then(|v| v.as_f64()).unwrap_or(0.);
    (
        get(w.as_ref().and_then(|w| w.inner_width().ok())),
        get(w.as_ref().and_then(|w| w.inner_height().ok())),
    )
}
/// A `<style>` with the page's rules ( the gallery page has its own ).
pub(super) fn add_style(css: &str) -> Result<()> {
    let style = element("style")?;
    style.set_text_content(Some(css));
    document()?
        .body()
        .ok_or(Error::Invalid("body"))?
        .append_child(&style)
        .map_err(|_| Error::Invalid("style"))?;
    Ok(())
}
/// The page's CSS3DRenderer: window-sized, absolute at the top, in English
/// like the page, and the target of the gallery's pointer input as the
/// page's controls are.
pub(super) fn overlay(id: &str) -> Result<CSS3DRenderer> {
    let mut renderer = CSS3DRenderer::new(CSS3DRendererParameters::default())?;
    let (w, h) = window_size();
    renderer.set_size(w, h);
    let e = renderer.dom_element();
    e.set_id(id);
    let _ = e.set_attribute("lang", "en");
    let _ = e.set_attribute("data-pointer-target", "");
    let _ = e.style().set_property("position", "absolute");
    let _ = e.style().set_property("top", "0px");
    document()?
        .body()
        .ok_or(Error::Invalid("body"))?
        .append_child(e)
        .map_err(|_| Error::Invalid("overlay"))?;
    Ok(renderer)
}
/// three.js's SRGBToLinear and LinearToSRGB.
fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 {
        c * 0.0773993808
    } else {
        (c * 0.9478672986 + 0.0521327014).powf(2.4)
    }
}
fn linear_to_srgb(c: f64) -> f64 {
    if c < 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(0.41666) - 0.055
    }
}
/// `new THREE.Color( hex ).getStyle()`: the floored hex through the linear
/// working space and back, as `rgb(r,g,b)`.
pub(super) fn color_style(hex: f64) -> String {
    let hex = hex.floor() as u32;
    let byte = |shift: u32| {
        let linear = srgb_to_linear(f64::from((hex >> shift) & 255) / 255.);
        (linear_to_srgb(linear) * 255. + 0.5).floor()
    };
    format!("rgb({},{},{})", byte(16), byte(8), byte(0))
}

/// The css3d pages' "camera setViewOffset" folder: setViewOffset ( 0 ), the
/// six values ( 1–6 ) and clearViewOffset ( 7 ), as their onChange and
/// updateCameraViewOffset() apply them.
pub(super) fn view_offset_gui(
    s: &mut crate::scene::Scene,
    c: crate::scene::Object3D,
    index: usize,
    value: f64,
) -> Result<()> {
    use crate::camera::{Camera, ViewOffset};
    use crate::scene::NodeKind;
    // updateCameraViewOffset(): each given value, or the current one ( the
    // window's before there is a view ); `0` counts as not given.
    let update = |s: &mut crate::scene::Scene, index: usize, value: f64| -> Result<()> {
        let (w, h) = window_size();
        let NodeKind::Camera(camera) = &mut s.get_mut(c)?.kind else {
            return Err(Error::Invalid("view offset camera"));
        };
        let view = match camera {
            Camera::Perspective(p) => &mut p.view,
            Camera::Orthographic(o) => &mut o.view,
        };
        let mut next = view.map_or([w, h, 0., 0., w, h], |v| {
            [
                v.full_width,
                v.full_height,
                v.offset_x,
                v.offset_y,
                v.width,
                v.height,
            ]
        });
        if value != 0. {
            next[index] = value;
        }
        *view = Some(ViewOffset {
            full_width: next[0],
            full_height: next[1],
            offset_x: next[2],
            offset_y: next[3],
            width: next[4],
            height: next[5],
        });
        Ok(())
    };
    match index {
        0 => {
            let (w, h) = window_size();
            for (i, v) in [w, h, 0., 0., w, h].into_iter().enumerate() {
                update(s, i, v)?;
            }
        }
        1..=6 => update(s, index - 1, value)?,
        _ => {
            let NodeKind::Camera(camera) = &mut s.get_mut(c)?.kind else {
                return Err(Error::Invalid("view offset camera"));
            };
            let view = match camera {
                Camera::Perspective(p) => &mut p.view,
                Camera::Orthographic(o) => &mut o.view,
            };
            // The values' setValue( 0 ) changes nothing, then clearViewOffset().
            *view = None;
        }
    }
    Ok(())
}

/// TWEEN.Easing.Exponential.InOut ( tween.js 23.1.1 ).
pub(super) fn exponential_in_out(amount: f64) -> f64 {
    if amount == 0. {
        return 0.;
    }
    if amount == 1. {
        return 1.;
    }
    let a = amount * 2.;
    if a < 1. {
        0.5 * 1024f64.powf(a - 1.)
    } else {
        0.5 * (-(2f64.powf(-10. * (a - 1.))) + 2.)
    }
}
/// A tween.js Tween of an object's position ( `object` ), or of nothing
/// with an onComplete ( `object` None ): its start, end and timing.
pub(super) struct Tween {
    pub object: Option<usize>,
    pub from: [f64; 3],
    pub to: [f64; 3],
    pub start: f64,
    pub duration: f64,
    pub easing: fn(f64) -> f64,
}
impl Tween {
    /// Tween.update( time ): the eased values, or None before its start;
    /// whether it completed.
    pub fn sample(&self, time: f64) -> (Option<[f64; 3]>, bool) {
        if time < self.start {
            return (None, false);
        }
        let elapsed = time - self.start;
        let portion = if self.duration == 0. || elapsed > self.duration {
            1.
        } else {
            let p = (elapsed / self.duration).min(1.);
            if p == 0. && elapsed == self.duration {
                1.
            } else {
                p
            }
        };
        let v = (self.easing)(portion);
        let values = std::array::from_fn(|i| self.from[i] + (self.to[i] - self.from[i]) * v);
        (
            Some(values),
            self.duration == 0. || elapsed >= self.duration,
        )
    }
}

/// The examples' main.css body as the CSS pages' elements inherit it, under
/// `scopes`, with the gallery's own `*` rules ( smoothed fonts, border-box
/// sizing ) undone and main.css's button rule.
pub(super) fn page_css(scopes: &[&str], color: &str) -> String {
    let all = scopes.join(",");
    let descendants = scopes
        .iter()
        .map(|s| format!("{s} *"))
        .collect::<Vec<_>>()
        .join(",");
    let buttons = scopes
        .iter()
        .map(|s| format!("{s} button"))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{all}{{color:{color};font-family:Monospace;font-size:13px;line-height:24px}}\
         {all},{descendants}{{-webkit-font-smoothing:auto;-moz-osx-font-smoothing:auto;box-sizing:content-box}}\
         {buttons}{{cursor:pointer;text-transform:uppercase;pointer-events:auto}}"
    )
}

pub(super) use crate::math::quaternion_from_rotation;

/// The svg pages' SVGRenderer: window-sized at the top of the page, over the
/// gallery's canvas and the target of its pointer input; `color_management`
/// is THREE.ColorManagement.enabled.
pub(super) fn svg_overlay(id: &str, color_management: bool) -> Result<crate::svg::SVGRenderer> {
    let mut renderer = crate::svg::SVGRenderer::new()?;
    renderer.color_management = color_management;
    let (w, h) = window_size();
    renderer.set_size(w, h);
    let e = renderer.dom_element();
    e.set_id(id);
    let _ = e.set_attribute("data-pointer-target", "");
    let _ = e.set_attribute("style", "position:absolute;top:0px;left:0px;display:block");
    document()?
        .body()
        .ok_or(Error::Invalid("body"))?
        .append_child(e)
        .map_err(|_| Error::Invalid("svg overlay"))?;
    Ok(renderer)
}
