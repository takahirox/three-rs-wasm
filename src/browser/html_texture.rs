//! webgpu_materials_texture_html: an HTML element ( text, emoji, RTL and
//! vertical text, an image, SVG, an input and a button ) as the map of a
//! rotating rounded box. HTMLTexture's element is a child of the renderer's
//! canvas with `layoutsubtree`; the browser's HTML-in-Canvas rasterization
//! ( the page's three-html-render polyfill ) records its paints and each
//! paint is copied into the map's GPU texture in place, as the WebGPU
//! backend's copyElementImageToTexture does. InteractionManager's CSS
//! matrix3d keeps the live element over the box's front face, so the
//! browser hit-tests the input and the button.
use crate::{Result, camera::*, material::*, math::*, renderer::*, scene::*};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r##"
// three-html-render, pinned. The native HTML-in-Canvas API has no path to this
// crate's wgpu queue, so the polyfill's rasterization is always installed.
const POLYFILL = 'https://cdn.jsdelivr.net/npm/three-html-render@0.1.2/dist/polyfill.mjs';
// The page's styles, after the rules of three's examples/main.css that reach the element.
// The gallery's own `* { box-sizing: border-box }` is undone for the element's subtree.
const STYLE = `
#draw_element, #draw_element * {
	box-sizing: content-box;
	-webkit-font-smoothing: auto;
}
#draw_element button {
	cursor: pointer;
	text-transform: uppercase;
}
#draw_element a, #draw_element button, #draw_element input, #draw_element select {
	pointer-events: auto;
}
#draw_element {
	width: 600px;
	background-color: #aaaaaa;
	color: #000000;
	font-family: sans-serif;
	font-size: 30px;
	line-height: 1.5;
	text-align: center;
	padding: 30px;
}
#draw_element img {
  animation: swing 1s ease-in-out infinite alternate;
}
#draw_element input[type="text"] {
	font-size: 24px;
	padding: 8px 12px;
	border: 2px solid #888;
	border-radius: 6px;
	width: 80%;
	margin-top: 10px;
}
#draw_element button {
	font-size: 24px;
	padding: 8px 20px;
	margin-top: 10px;
	border: none;
	border-radius: 6px;
	background-color: #4CAF50;
	color: white;
	cursor: pointer;
}
#draw_element button:hover {
	background-color: #2196F3;
}
@keyframes swing {
	from { transform: rotate(-15deg); }
	to { transform: rotate(15deg); }
}`;
export async function htmlTextureLoad() {
	if ( ! globalThis.galleryHtmlRender ) {
		const module = await import( POLYFILL );
		module.installHtmlInCanvasPolyfill( { force: true } );
		globalThis.galleryHtmlRender = module;
	}
}
export class HtmlTextureSource {
	constructor() {
		const style = document.createElement( 'style' );
		style.textContent = STYLE;
		document.head.appendChild( style );
		const element = document.createElement( 'div' );
		element.id = 'draw_element';
		element.innerHTML = `
					Hello world!<br>I'm multi-line, <b>formatted</b>,
					rotated text with emoji (&#128512;), RTL text
					<span dir=rtl>من فارسی صحبت میکنم</span>,
					vertical text,
					<p style="writing-mode: vertical-rl;">
					这是垂直文本
					</p>
					an inline image (<img width="150" src="assets/material-textures/758px-Canestra_di_frutta_(Caravaggio).jpg">), and
					<svg width="50" height="50">
					<circle cx="25" cy="25" r="20" fill="green" />
					<text x="25" y="30" font-size="15" text-anchor="middle" fill="#fff">
						SVG
					</text>
					</svg>!
					<br>
					<input type="text" placeholder="Type here...">
					<button>Click me</button>
				`;
		element.querySelector( 'button' ).addEventListener( 'click', function () {
			this.textContent = 'Clicked!';
		} );
		this.element = element;
		this.canvas = document.querySelector( 'canvas' );
		this.dirty = false;
		this.attached = false;
	}
	// Textures: the element joins the canvas' layout subtree, with the shared paint callback.
	attach() {
		const canvas = this.canvas;
		if ( ! canvas.hasAttribute( 'layoutsubtree' ) ) canvas.setAttribute( 'layoutsubtree', 'true' );
		if ( this.element.parentNode !== canvas ) canvas.appendChild( this.element );
		canvas.onpaint = ( event ) => {
			const changed = event && event.changedElements;
			if ( ! changed || changed.includes( this.element ) ) this.dirty = true;
		};
		this.attached = true;
	}
	width() {
		return this.element.offsetWidth || 1;
	}
	height() {
		return this.element.offsetHeight || 1;
	}
	requestPaint() {
		this.canvas.requestPaint();
	}
	takeDirty() {
		const dirty = this.dirty;
		this.dirty = false;
		return dirty;
	}
	// copyElementImageToTexture's source: the recorded paint, resampled to the texture size.
	snapshot( width, height ) {
		const source = globalThis.galleryHtmlRender.getHtmlRenderer().getCanvas( this.element );
		if ( ! source ) return null;
		if ( source.width === width && source.height === height ) return source;
		if ( ! this.resized ) this.resized = document.createElement( 'canvas' );
		const r = this.resized;
		if ( r.width !== width ) r.width = width;
		if ( r.height !== height ) r.height = height;
		const context = r.getContext( '2d' );
		context.clearRect( 0, 0, width, height );
		context.imageSmoothingEnabled = true;
		context.imageSmoothingQuality = 'high';
		context.drawImage( source, 0, 0, width, height );
		return r;
	}
	// InteractionManager.update(): returns false when the element has no size yet.
	place( elements ) {
		const element = this.element;
		element.style.position = 'absolute';
		element.style.left = '0';
		element.style.top = '0';
		element.style.transformOrigin = '0 0';
		element.style.transform = 'matrix3d(' + Array.from( elements ).join( ',' ) + ')';
	}
	offsetWidth() {
		return this.element.offsetWidth;
	}
	offsetHeight() {
		return this.element.offsetHeight;
	}
	cssSize() {
		return [ this.canvas.clientWidth, this.canvas.clientHeight ];
	}
}
"##)]
extern "C" {
    #[wasm_bindgen(catch, js_name = htmlTextureLoad)]
    async fn html_texture_load() -> std::result::Result<JsValue, JsValue>;

    type HtmlTextureSource;
    #[wasm_bindgen(constructor)]
    fn new() -> HtmlTextureSource;
    #[wasm_bindgen(method)]
    fn attach(this: &HtmlTextureSource);
    #[wasm_bindgen(method, getter)]
    fn attached(this: &HtmlTextureSource) -> bool;
    #[wasm_bindgen(method)]
    fn width(this: &HtmlTextureSource) -> u32;
    #[wasm_bindgen(method)]
    fn height(this: &HtmlTextureSource) -> u32;
    #[wasm_bindgen(method, js_name = requestPaint)]
    fn request_paint(this: &HtmlTextureSource);
    #[wasm_bindgen(method, js_name = takeDirty)]
    fn take_dirty(this: &HtmlTextureSource) -> bool;
    #[wasm_bindgen(method)]
    fn snapshot(
        this: &HtmlTextureSource,
        width: u32,
        height: u32,
    ) -> Option<web_sys::HtmlCanvasElement>;
    #[wasm_bindgen(method)]
    fn place(this: &HtmlTextureSource, elements: &[f64]);
    #[wasm_bindgen(method, js_name = offsetWidth)]
    fn offset_width(this: &HtmlTextureSource) -> f64;
    #[wasm_bindgen(method, js_name = offsetHeight)]
    fn offset_height(this: &HtmlTextureSource) -> f64;
    #[wasm_bindgen(method, js_name = cssSize)]
    fn css_size(this: &HtmlTextureSource) -> Vec<f64>;
}

/// NeutralToneMapping at exposure 1, on a linear color.
fn neutral_tone_mapping(mut color: Vector3) -> Vector3 {
    let start_compression = 0.8 - 0.04;
    let desaturation = 0.15;
    let x = color.x.min(color.y.min(color.z));
    let offset = if x < 0.08 { x - 6.25 * x * x } else { 0.04 };
    color -= Vector3::splat(offset);
    let peak = color.x.max(color.y.max(color.z));
    if peak < start_compression {
        return color;
    }
    let d = 1. - start_compression;
    let new_peak = 1. - d * d / (peak + d - start_compression);
    color *= new_peak / peak;
    let g = 1. - 1. / (desaturation * (peak - new_peak) + 1.);
    color.lerp(Vector3::splat(new_peak), g)
}

/// The map's GPU state: none until the first render, then the texture
/// waiting for its first paint record, then painted.
enum MapState {
    Pending,
    WaitingForPaint(wgpu::Texture),
    Ready(wgpu::Texture),
}

pub(super) struct Demo {
    mesh: Object3D,
    source: HtmlTextureSource,
    map: Arc<Texture>,
    state: MapState,
    needs_update: bool,
    /// The geometry's bounding box ( min, max ).
    bounds: (Vector3, Vector3),
    time: f64,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        html_texture_load()
            .await
            .map_err(|e| crate::Error::Asset(format!("three-html-render: {e:?}")))?;
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 1.,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 500.);
        s.tone_mapping = ToneMapping::Neutral;
        // WebGPURenderer tone-maps the whole frame in its output pass, the
        // background color included: the clear color is that result.
        s.background = Color(neutral_tone_mapping(Color::from_hex(0xaaaaaa).0));
        s.environment = Some(super::room_environment::environment_with_sigma(r, 0.02)?);
        let source = HtmlTextureSource::new();
        let geometry = super::gtao::rounded_box(200., 200., 200., 10, 10.)?;
        let positions = geometry.positions()?;
        let bounds = positions.iter().fold(
            (
                Vector3::splat(f64::INFINITY),
                Vector3::splat(f64::NEG_INFINITY),
            ),
            |(lo, hi), p| (lo.min(*p), hi.max(*p)),
        );
        // HTMLTexture: no mipmaps, sampled without a color space, sized at its first upload.
        let mut map = Texture::from_rgba(1, 1, vec![0; 4], false)?;
        map.mipmap_filter = None;
        let map = Arc::new(map);
        let mut material = MeshStandardMaterial {
            roughness: 0.,
            metalness: 0.5,
            ..Default::default()
        };
        material.properties.map = Some(map.clone());
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Standard(material)),
        )));
        Ok(Self {
            mesh,
            source,
            map,
            state: MapState::Pending,
            needs_update: true,
            bounds,
            time: 0.,
        })
    }

    /// The map's texture at the element's size, as the renderer first meets it.
    fn init_map(&mut self, s: &mut Scene, r: &Renderer) -> Result<wgpu::Texture> {
        self.source.attach();
        let (width, height) = (self.source.width(), self.source.height());
        let mut map = (*self.map).clone();
        map.width = width;
        map.height = height;
        map.rgba = vec![0; (width * height * 4) as usize];
        self.map = Arc::new(map);
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.mesh)?.kind {
            for material in &mut m.materials {
                if let Material::Standard(material) = Arc::make_mut(material) {
                    material.properties.map = Some(self.map.clone());
                }
            }
        }
        Ok(r.upload_texture(&self.map)?.texture)
    }

    /// InteractionManager.update() with the box's matrixWorld from the last render.
    fn update_interactions(&self, s: &Scene, c: Object3D) -> Result<()> {
        let css = self.source.css_size();
        let (css_w, css_h) = (css[0], css[1]);
        let viewport = Matrix4::from_cols_array(&[
            css_w / 2.,
            0.,
            0.,
            0.,
            0.,
            -css_h / 2.,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            css_w / 2.,
            css_h / 2.,
            0.,
            1.,
        ]);
        let (elem_w, elem_h) = (self.source.offset_width(), self.source.offset_height());
        let (lo, hi) = self.bounds;
        let size = hi - lo;
        let pixel_to_local = Matrix4::from_cols_array(&[
            size.x / elem_w,
            0.,
            0.,
            0.,
            0.,
            -size.y / elem_h,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            -size.x / 2.,
            size.y / 2.,
            hi.z,
            1.,
        ]);
        let (camera, world) = s.camera(c)?;
        let mvp = viewport
            * camera.projection_matrix()?
            * world.inverse()
            * s.get(self.mesh)?.matrix_world
            * pixel_to_local;
        self.source.place(&mvp.to_cols_array());
        Ok(())
    }

    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt * 1000.;
        }
        Ok(())
    }
    /// animate( time ): the rotation, interactions.update(), then the map's upload.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let t = self.time;
        self.update_interactions(s, c)?;
        s.get_mut(self.mesh)?.quaternion = Euler {
            angles: Vector3::new((t * 0.0005).sin() * 0.5, (t * 0.0008).cos() * 0.5, 0.),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        if self.source.attached() && self.source.take_dirty() {
            self.needs_update = true;
        }
        if !std::mem::take(&mut self.needs_update) {
            return Ok(());
        }
        self.state = match std::mem::replace(&mut self.state, MapState::Pending) {
            // Skip the first upload: the element needs a paint record first.
            MapState::Pending => {
                let texture = self.init_map(s, r)?;
                self.source.request_paint();
                MapState::WaitingForPaint(texture)
            }
            MapState::WaitingForPaint(texture) | MapState::Ready(texture) => {
                let (width, height) = (texture.width(), texture.height());
                if let Some(canvas) = self.source.snapshot(width, height) {
                    r.queue.copy_external_image_to_texture(
                        &wgpu::CopyExternalImageSourceInfo {
                            source: wgpu::ExternalImageSource::HTMLCanvasElement(canvas),
                            origin: wgpu::Origin2d::ZERO,
                            flip_y: false,
                        },
                        wgpu::CopyExternalImageDestInfo {
                            texture: &texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                            color_space: wgpu::PredefinedColorSpace::Srgb,
                            premultiplied_alpha: false,
                        },
                        texture.size(),
                    );
                }
                MapState::Ready(texture)
            }
        };
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
        Ok(())
    }
    /// The page's clock set directly: the animation loop's timestamp.
    pub fn seek(&mut self, t: f64) {
        self.time = t * 1000.;
    }
}
