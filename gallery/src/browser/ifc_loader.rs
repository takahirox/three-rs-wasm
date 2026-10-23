//! webgl_loader_ifc: the Revit advanced sample project as web-ifc streams it.
//! The page's web-ifc 0.0.77 is loaded from jsDelivr at run time, as the page
//! loads it: the model is opened with COORDINATE_TO_ORIGIN and StreamAllMeshes
//! hands each placed geometry to the page's loadAllGeometry, ported here —
//! getBufferGeometry's GetGeometry / GetVertexArray / GetIndexArray per
//! placement, the sRGB colour converted once per placement into RGBA vertex
//! colours, applyMatrix4 on positions and normals, and mergeGeometries into
//! one opaque and one transparent double-sided vertex-coloured Phong mesh.
//! web-ifc's wasm module has its own memory, so the library is called
//! through JavaScript. Two directional lights and an ambient light;
//! OrbitControls render on demand.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use crate::attribute::BufferAttribute;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

const ASSETS: &str = "assets/ifc-loader";

#[wasm_bindgen(inline_js = r#"
const WEB_IFC_VERSION = '0.0.77';
const WEB_IFC_WASM_PATH = `https://cdn.jsdelivr.net/npm/web-ifc@${ WEB_IFC_VERSION }/`;
// IfcAPI.Init() with the page's wasm path; the API object is the library's.
export async function ifcOpen() {
	const { IfcAPI } = await import( `${ WEB_IFC_WASM_PATH }web-ifc-api.js` );
	const ifcAPI = new IfcAPI();
	ifcAPI.SetWasmPath( WEB_IFC_WASM_PATH );
	await ifcAPI.Init();
	return ifcAPI;
}
// The library calls the page makes: OpenModel, StreamAllMeshes with each
// placed geometry handed to `placed( color, flatTransformation, vertexData,
// indexData )` ( getBufferGeometry's GetGeometry / GetVertexArray /
// GetIndexArray, the geometry released after ), and CloseModel.
export function ifcStream( ifcAPI, data, placed ) {
	const modelID = ifcAPI.OpenModel( data, { COORDINATE_TO_ORIGIN: true } );
	ifcAPI.StreamAllMeshes( modelID, ( flatMesh ) => {
		const placedGeometries = flatMesh.geometries;
		for ( let i = 0; i < placedGeometries.size(); i ++ ) {
			const placedGeometry = placedGeometries.get( i );
			const geometry = ifcAPI.GetGeometry( modelID, placedGeometry.geometryExpressID );
			const vertexData = ifcAPI.GetVertexArray( geometry.GetVertexData(), geometry.GetVertexDataSize() );
			const indexData = ifcAPI.GetIndexArray( geometry.GetIndexData(), geometry.GetIndexDataSize() );
			const c = placedGeometry.color;
			placed( new Float64Array( [ c.x, c.y, c.z, c.w ] ), new Float64Array( placedGeometry.flatTransformation ), vertexData, indexData );
			geometry.delete();
		}
	} );
	ifcAPI.CloseModel( modelID );
}
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = ifcOpen)]
    fn ifc_open() -> std::result::Result<js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, js_name = ifcStream)]
    fn ifc_stream(
        api: &JsValue,
        data: &js_sys::Uint8Array,
        placed: &js_sys::Function,
    ) -> std::result::Result<(), JsValue>;
}
/// Color.setRGB( …, SRGBColorSpace ): SRGBToLinear.
fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 {
        c * 0.0773993808
    } else {
        (c * 0.9478672986 + 0.0521327014).powf(2.4)
    }
}

/// One merged mesh's attributes and index, as mergeGeometries concatenates them.
#[derive(Default)]
struct Merged {
    position: Vec<f32>,
    normal: Vec<f32>,
    color: Vec<f32>,
    index: Vec<u32>,
}
impl Merged {
    fn geometry(self) -> Result<BufferGeometry> {
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(self.position, 3, false)?),
        );
        g.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(self.normal, 3, false)?),
        );
        g.set_attribute(
            "color",
            Attribute::F32(BufferAttribute::new(self.color, 4, false)?),
        );
        g.set_index(Some(self.index));
        Ok(g)
    }
}

/// One placed geometry merged into its mesh: getPlacedGeometry's colour,
/// ifcGeometryToBuffer's attributes, applyMatrix4 with the flat
/// transformation and mergeGeometries' concatenation.
fn place(target: &mut Merged, c: &[f64], e: &[f64], vertex: &[f32], index: &[u32]) {
    let color = [
        srgb_to_linear(c[0]) as f32,
        srgb_to_linear(c[1]) as f32,
        srgb_to_linear(c[2]) as f32,
        c[3] as f32,
    ];
    // getNormalMatrix: the inverse transpose of the upper 3 × 3.
    let normal_matrix = Matrix3::from_cols(
        Vector3::new(e[0], e[1], e[2]),
        Vector3::new(e[4], e[5], e[6]),
        Vector3::new(e[8], e[9], e[10]),
    )
    .inverse()
    .transpose();
    let base = (target.position.len() / 3) as u32;
    for v in vertex.as_chunks::<6>().0 {
        let (x, y, z) = (f64::from(v[0]), f64::from(v[1]), f64::from(v[2]));
        let w = 1. / (e[3] * x + e[7] * y + e[11] * z + e[15]);
        target.position.extend([
            ((e[0] * x + e[4] * y + e[8] * z + e[12]) * w) as f32,
            ((e[1] * x + e[5] * y + e[9] * z + e[13]) * w) as f32,
            ((e[2] * x + e[6] * y + e[10] * z + e[14]) * w) as f32,
        ]);
        let n = normal_matrix * Vector3::new(f64::from(v[3]), f64::from(v[4]), f64::from(v[5]));
        let length = n.length();
        let n = if length > 0. { n / length } else { n };
        target.normal.extend([n.x as f32, n.y as f32, n.z as f32]);
        target.color.extend(color);
    }
    target.index.extend(index.iter().map(|i| i + base));
}

pub struct Demo {
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
            near: 0.1,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(82.48, 22.09, -45.24);
        s.background = Color::from_hex(0x8cc7de);
        for (color, position) in [
            (0xffeeff, Vector3::new(1., 1., 1.)),
            (0xffffff, Vector3::new(-1., 0.5, -1.)),
        ] {
            let h = s.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(color),
                intensity: 2.5,
                target: Vector3::ZERO,
            }));
            s.get_mut(h)?.position = position;
        }
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xffffee),
            intensity: 0.75,
        }));
        let js = |e: JsValue| Error::Asset(format!("web-ifc: {e:?}"));
        let api = JsFuture::from(ifc_open().map_err(js)?).await.map_err(js)?;
        let data = js_sys::Uint8Array::from(
            fetch(&format!("{ASSETS}/rac_advanced_sample_project.ifc"))
                .await?
                .as_slice(),
        );
        // loadAllGeometry(): each placement merged as it streams.
        let merged = std::rc::Rc::new(std::cell::RefCell::new((
            Merged::default(),
            Merged::default(),
        )));
        let target = merged.clone();
        let placed = Closure::<
            dyn FnMut(
                js_sys::Float64Array,
                js_sys::Float64Array,
                js_sys::Float32Array,
                js_sys::Uint32Array,
            ),
        >::new(
            move |color: js_sys::Float64Array,
                  matrix: js_sys::Float64Array,
                  vertex: js_sys::Float32Array,
                  index: js_sys::Uint32Array| {
                let (c, e, vertex, index) = (
                    color.to_vec(),
                    matrix.to_vec(),
                    vertex.to_vec(),
                    index.to_vec(),
                );
                let mut m = target.borrow_mut();
                let (opaque, transparent) = &mut *m;
                place(
                    if c[3] != 1. { transparent } else { opaque },
                    &c,
                    &e,
                    &vertex,
                    &index,
                );
            },
        );
        ifc_stream(&api, &data, placed.as_ref().unchecked_ref()).map_err(js)?;
        drop(placed);
        let (opaque, transparent) = std::rc::Rc::try_unwrap(merged)
            .map_err(|_| Error::Invalid("ifc stream"))?
            .into_inner();
        for (merged, is_transparent) in [(opaque, false), (transparent, true)] {
            if merged.index.is_empty() {
                continue;
            }
            let mut material = MeshPhongMaterial::default();
            material.properties.side = Side::Double;
            material.properties.vertex_colors = true;
            material.properties.transparent = is_transparent;
            s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(merged.geometry()?),
                Arc::new(Material::Phong(material)),
            )));
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(30.86, 7.73, 0.15));
        controls.update(s, c)?;
        Ok(Self { controls })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.update(s, c)
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
        Ok(())
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

/// The program of three.js examples/webgl_loader_ifc.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_ifc(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                1000.,
                Box::new(Demo::create(scene, camera, 441, renderer).await?),
            ))
        },
    )
    .await
}
