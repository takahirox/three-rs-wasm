//! css3d_molecules: a PDB molecule as CSS3DSprite atoms ( the ball sprite
//! tinted per element on a canvas ) and CSS3DObject bonds ( each a crossed
//! pair of thin divs ), turning under the root, with TrackballControls on
//! the CSS renderer's element and the GUI's visualisation and molecule. The
//! page has no canvas: the gallery's canvas clears to transparent over the
//! page's radial-gradient body.
use super::css3d_common::{
    add_style, element, overlay, page_css, quaternion_from_rotation, window_size,
};
use super::gltf_viewer::fetch;
use super::helpers_formats::formats::parse_pdb;
use super::trackball_sprites::{Mode, Trackball};
use crate::css2d::js_number;
use crate::css3d::{CSS3DObject, CSS3DRenderer, CSS3DSprite};
use crate::{Error, Result, camera::*, math::*, renderer::*, scene::*};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;

const MOLECULES: [&str; 17] = [
    "ethanol.pdb",
    "aspirin.pdb",
    "caffeine.pdb",
    "nicotine.pdb",
    "lsd.pdb",
    "cocaine.pdb",
    "cholesterol.pdb",
    "lycopene.pdb",
    "glucose.pdb",
    "Al2O3.pdb",
    "cubane.pdb",
    "cu.pdb",
    "caf2.pdb",
    "nacl.pdb",
    "ybco.pdb",
    "buckyball.pdb",
    "graphite.pdb",
];
/// The page's body and `.bond` rules.
const STYLE: &str = "body{background-color:#050505;background:radial-gradient(ellipse at center, rgba(43,45,48,1) 0%,rgba(0,0,0,1) 100%)}.bond{width:5px;height:10px;background:#eee;display:block}";
/// One of the objects: an atom, or a bond with its two heights.
enum Part {
    Atom,
    Bond { short: String, full: String },
}
type Loaded = Rc<RefCell<Option<Result<String>>>>;
pub(super) struct Demo {
    controls: Trackball,
    css: CSS3DRenderer,
    objects: Scene,
    root: Object3D,
    parts: Vec<(Object3D, Part)>,
    base: web_sys::HtmlImageElement,
    sprites: HashMap<String, String>,
    viz: u32,
    molecule: usize,
    requested: Option<usize>,
    loaded: Loaded,
    time: f64,
    size: (f64, f64),
}
/// Matrix4.makeRotationAxis( axis, angle ), as columns.
fn rotation_axis(axis: Vector3, angle: f64) -> (Vector3, Vector3, Vector3) {
    let (c, s) = (angle.cos(), angle.sin());
    let t = 1. - c;
    let (x, y, z) = (axis.x, axis.y, axis.z);
    let (tx, ty) = (t * x, t * y);
    (
        Vector3::new(tx * x + c, tx * y + s * z, tx * z - s * y),
        Vector3::new(tx * y - s * z, ty * y + c, ty * z + s * x),
        Vector3::new(tx * z + s * y, ty * z - s * x, t * z * z + c),
    )
}
/// Vector3.normalize(): a zero vector stays zero.
fn normalize(v: Vector3) -> Vector3 {
    let length = v.length();
    v / if length == 0. { 1. } else { length }
}
/// SRGBToLinear, as Color.setRGB( r, g, b, SRGBColorSpace ) converts.
fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 {
        c * 0.0773993808
    } else {
        (c * 0.9478672986 + 0.0521327014).powf(2.4)
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        add_style(&format!(
            "{}{STYLE}",
            page_css(&["#css3d-renderer"], "#fff")
        ))?;
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 1.,
            far: 5000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 1000.);
        // The page has no canvas: the gallery's clears to nothing over the body.
        s.background = Color::BLACK;
        s.background_alpha = 0.;
        let mut objects = Scene::new();
        let root = objects.insert(NodeKind::Group);
        let css = overlay("css3d-renderer")?;
        let size = window_size();
        let mut controls = Trackball::new(s, c, Vector2::new(size.0, size.1))?;
        controls.rotate_speed = 0.5;
        // baseSprite.onload: the first molecule.
        let base = element("img")?
            .dyn_into::<web_sys::HtmlImageElement>()
            .map_err(|_| Error::Invalid("ball image"))?;
        base.set_src("/web/gallery/assets/point-clouds/ball.png");
        wasm_bindgen_futures::JsFuture::from(base.decode())
            .await
            .map_err(|_| Error::Invalid("ball image"))?;
        let text = String::from_utf8_lossy(&fetch(&url(2)).await?).into_owned();
        let mut d = Self {
            controls,
            css,
            objects,
            root,
            parts: vec![],
            base,
            sprites: HashMap::new(),
            viz: 2,
            molecule: 2,
            requested: Some(2),
            loaded: Rc::new(RefCell::new(None)),
            time: 0.,
            size,
        };
        d.load(&text)?;
        Ok(d)
    }
    /// The element's tinted ball: the image on a canvas, each channel times
    /// the atom's linear color ( Uint8ClampedArray rounds half to even ).
    fn sprite(&mut self, label: &str, color: [f64; 3]) -> Result<String> {
        if let Some(url) = self.sprites.get(label) {
            return Ok(url.clone());
        }
        let fail = |_| Error::Invalid("sprite canvas");
        let (w, h) = (self.base.width(), self.base.height());
        let canvas = element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| Error::Invalid("sprite canvas"))?;
        canvas.set_width(w);
        canvas.set_height(h);
        let context = canvas
            .get_context("2d")
            .map_err(fail)?
            .ok_or(Error::Invalid("sprite context"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|_| Error::Invalid("sprite context"))?;
        context
            .draw_image_with_html_image_element_and_dw_and_dh(
                &self.base,
                0.,
                0.,
                f64::from(w),
                f64::from(h),
            )
            .map_err(fail)?;
        let image = context
            .get_image_data(0., 0., f64::from(w), f64::from(h))
            .map_err(fail)?;
        let mut data = image.data().0;
        for px in data.chunks_mut(4) {
            for (k, value) in px.iter_mut().take(3).enumerate() {
                *value = (f64::from(*value) * color[k])
                    .clamp(0., 255.)
                    .round_ties_even() as u8;
            }
        }
        let tinted = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&data),
            w,
            h,
        )
        .map_err(fail)?;
        context.put_image_data(&tinted, 0., 0.).map_err(fail)?;
        let url = canvas.to_data_url().map_err(fail)?;
        self.sprites.insert(label.to_owned(), url.clone());
        Ok(url)
    }
    /// loadMolecule(): the old objects out, then the atoms and bonds of the
    /// centred molecule at 75 px per Å.
    fn load(&mut self, text: &str) -> Result<()> {
        for (h, _) in std::mem::take(&mut self.parts) {
            self.objects.dispose(h)?;
        }
        let molecule = parse_pdb(text)?;
        // The Float32 position attributes, centred on the atoms' box.
        let f = |v: f64| f64::from(v as f32);
        let (mut min, mut max) = (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        );
        for atom in &molecule.atoms {
            let p = Vector3::from_array(atom.position.map(f));
            min = min.min(p);
            max = max.max(p);
        }
        let offset = -((min + max) * 0.5);
        let shift = |p: [f64; 3]| -> Vector3 {
            let p = Vector3::from_array(p.map(f));
            Vector3::new(f(p.x + offset.x), f(p.y + offset.y), f(p.z + offset.z))
        };
        let node = |objects: &mut Scene,
                    kind: NodeKind,
                    matrix: Matrix4,
                    position: Vector3,
                    quaternion: Quaternion|
         -> Result<Object3D> {
            let h = objects.insert(kind);
            let n = objects.get_mut(h)?;
            n.position = position;
            n.quaternion = quaternion;
            n.matrix_auto_update = false;
            n.matrix = matrix;
            Ok(h)
        };
        for atom in &molecule.atoms {
            let position = shift(atom.position) * 75.;
            let color = atom
                .color
                .map(|c| f64::from(srgb_to_linear(f64::from(c) / 255.) as f32));
            let src = self.sprite(&atom.label, color)?;
            let img = element("img")?;
            let _ = img.set_attribute("src", &src);
            let h = node(
                &mut self.objects,
                NodeKind::CSS3DSprite(CSS3DSprite::new(img)),
                Matrix4::from_translation(position),
                position,
                Quaternion::IDENTITY,
            )?;
            self.objects.add(self.root, h)?;
            self.parts.push((h, Part::Atom));
        }
        for (start, end) in &molecule.bonds {
            let (start, end) = (shift(*start) * 75., shift(*end) * 75.);
            let v = end - start;
            let length = v.length() - 50.;
            let short = format!("{}px", js_number(length));
            let full = format!("{}px", js_number(length + 55.));
            let axis = Vector3::Y.cross(v);
            let radians = Vector3::Y.dot(normalize(v)).acos();
            let (cx, cy, cz) = rotation_axis(normalize(axis), radians);
            let quaternion = quaternion_from_rotation(cx, cy, cz);
            let middle = start.lerp(end, 0.5);
            let compose = Matrix4::from_rotation_translation(quaternion, middle);
            let bond = || -> Result<web_sys::HtmlElement> {
                let div = element("div")?;
                div.set_class_name("bond");
                let _ = div.style().set_property("height", &short);
                Ok(div)
            };
            let h = node(
                &mut self.objects,
                NodeKind::CSS3DObject(CSS3DObject::new(bond()?)),
                compose,
                middle,
                quaternion,
            )?;
            self.objects.add(self.root, h)?;
            self.parts.push((
                h,
                Part::Bond {
                    short: short.clone(),
                    full: full.clone(),
                },
            ));
            // The crossing bond, turned a quarter about y under a joint.
            let joint = node(
                &mut self.objects,
                NodeKind::Group,
                compose,
                middle,
                quaternion,
            )?;
            let turn = Quaternion::from_rotation_y(std::f64::consts::PI / 2.);
            let cross = node(
                &mut self.objects,
                NodeKind::CSS3DObject(CSS3DObject::new(bond()?)),
                Matrix4::from_quat(turn),
                Vector3::ZERO,
                turn,
            )?;
            self.objects.add(joint, cross)?;
            self.objects.add(self.root, joint)?;
            self.parts.push((cross, Part::Bond { short, full }));
        }
        self.visualise()
    }
    /// changeVizType(): showAtoms(), showBonds() or showAtomsBonds().
    fn visualise(&mut self) -> Result<()> {
        for (h, part) in &self.parts {
            let n = self.objects.get_mut(*h)?;
            let element = match &n.kind {
                NodeKind::CSS3DObject(o) => o.element.element.clone(),
                NodeKind::CSS3DSprite(o) => o.element.element.clone(),
                _ => None,
            };
            let Some(element) = element else { continue };
            let style = element.style();
            let atom = matches!(part, Part::Atom);
            let visible = match self.viz {
                0 => atom,
                1 => !atom,
                _ => true,
            };
            let _ = style.set_property("display", if visible { "" } else { "none" });
            n.visible = visible;
            if let Part::Bond { short, full } = part {
                match self.viz {
                    1 => {
                        let _ = style.set_property("height", full);
                    }
                    2 => {
                        let _ = style.set_property("height", short);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): a requested molecule's load, controls.update(), the root's
    /// turn by Date.now() × 0.0004, then render().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let size = window_size();
        if size != self.size {
            self.size = size;
            self.css.set_size(size.0, size.1);
            self.controls.screen = Vector2::new(size.0, size.1);
        }
        if self.requested != Some(self.molecule) {
            self.requested = Some(self.molecule);
            // The old objects leave at once; the new ones arrive with the file.
            for (h, _) in std::mem::take(&mut self.parts) {
                self.objects.dispose(h)?;
            }
            let (slot, url) = (self.loaded.clone(), url(self.molecule));
            wasm_bindgen_futures::spawn_local(async move {
                let result = fetch(&url)
                    .await
                    .map(|b| String::from_utf8_lossy(&b).into_owned());
                *slot.borrow_mut() = Some(result);
            });
        }
        let loaded = self.loaded.borrow_mut().take();
        if let Some(text) = loaded {
            self.load(&text?)?;
        }
        self.controls.update(s)?;
        let time = self.time * 1000. * 0.0004;
        self.objects.get_mut(self.root)?.quaternion = Euler {
            angles: Vector3::new(time, time * 0.7, 0.),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        s.update()?;
        self.css.render_from(&mut self.objects, s, c)
    }
    /// TrackballControls on the CSS renderer's element: absolute pointer events.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        match kind {
            10..=19 => self.controls.down(kind - 10, x, y),
            20..=29 => self.controls.state = Mode::None,
            _ => self.controls.moved(x, y),
        }
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        if wheel != 0. {
            self.controls.zoom_start.y -= wheel * 0.00025;
        }
        Ok(())
    }
    /// The GUI: vizType, molecule.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => {
                self.viz = (value as u32).min(2);
                self.visualise()
            }
            1 => {
                self.molecule = (value as usize).min(MOLECULES.len() - 1);
                Ok(())
            }
            _ => Err(Error::Invalid("css3d_molecules parameter")),
        }
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
fn url(index: usize) -> String {
    format!("/web/gallery/assets/pdb/{}", MOLECULES[index])
}
