//! meshoptimizer's simplifier and clusterizer, the builds the pages load:
//! three.js r186's bundled meshopt_simplifier / meshopt_clusterizer modules
//! ( webgpu_compute_rasterizer_ibl ) and meshoptimizer 1.1.1 from jsDelivr
//! ( webgl_batch_lod_bvh, through @three.ez/simplify-geometry ). Their wasm
//! modules have their own memory, so they are called through JavaScript.
use crate::{Error, Result};
use js_sys::{Array, Float32Array, Function, Reflect, Uint32Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(inline_js = r#"
// The page's imports, ready as the page awaits them.
export async function meshoptThree( simplifierUrl, clusterizerUrl ) {
	const [ { MeshoptSimplifier }, { MeshoptClusterizer } ] = await Promise.all( [
		import( new URL( simplifierUrl, location.href ).href ),
		import( new URL( clusterizerUrl, location.href ).href ),
	] );
	await Promise.all( [ MeshoptClusterizer.ready, MeshoptSimplifier.ready ] );
	return { MeshoptSimplifier, MeshoptClusterizer };
}
export async function meshoptimizer( url ) {
	const { MeshoptSimplifier } = await import( url );
	await MeshoptSimplifier.ready;
	return { MeshoptSimplifier };
}
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = meshoptThree)]
    fn meshopt_three(
        simplifier: &str,
        clusterizer: &str,
    ) -> std::result::Result<js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, js_name = meshoptimizer)]
    fn meshoptimizer_module(url: &str) -> std::result::Result<js_sys::Promise, JsValue>;
}

fn js(e: JsValue) -> Error {
    Error::Asset(format!("meshoptimizer: {e:?}"))
}
fn get(o: &JsValue, key: &str) -> Result<JsValue> {
    Reflect::get(o, &JsValue::from_str(key)).map_err(js)
}
fn call(o: &JsValue, method: &str, args: &[JsValue]) -> Result<JsValue> {
    let f: Function = get(o, method)?
        .dyn_into()
        .map_err(|_| Error::Asset(format!("meshoptimizer: {method} is not a function")))?;
    f.apply(o, &args.iter().collect::<Array>()).map_err(js)
}
fn strings(v: &[&str]) -> JsValue {
    v.iter()
        .map(|s| JsValue::from_str(s))
        .collect::<Array>()
        .into()
}
fn numbers(v: &[f64]) -> JsValue {
    v.iter()
        .map(|&n| JsValue::from_f64(n))
        .collect::<Array>()
        .into()
}

/// MeshoptSimplifier and MeshoptClusterizer.
pub(super) struct Meshopt {
    simplifier: JsValue,
    clusterizer: JsValue,
}
/// A meshlet: its vertex indices and its triangles' local indices.
pub(super) struct Meshlet {
    pub vertices: Vec<u32>,
    pub triangles: Vec<u32>,
}
/// computeMeshletBounds()'s sphere: centerX, centerY, centerZ, radius.
pub(super) type Bounds = [f64; 4];

impl Meshopt {
    /// three's bundled modules, at their gallery URLs.
    pub(super) async fn three(simplifier: &str, clusterizer: &str) -> Result<Self> {
        let m = JsFuture::from(meshopt_three(simplifier, clusterizer).map_err(js)?)
            .await
            .map_err(js)?;
        Ok(Meshopt {
            simplifier: get(&m, "MeshoptSimplifier")?,
            clusterizer: get(&m, "MeshoptClusterizer")?,
        })
    }
    /// meshoptimizer's ESM bundle ( the simplifier only ).
    pub(super) async fn package(url: &str) -> Result<Self> {
        let m = JsFuture::from(meshoptimizer_module(url).map_err(js)?)
            .await
            .map_err(js)?;
        Ok(Meshopt {
            simplifier: get(&m, "MeshoptSimplifier")?,
            clusterizer: JsValue::UNDEFINED,
        })
    }
    /// MeshoptSimplifier.getScale( positions, stride ).
    pub(super) fn get_scale(&self, positions: &[f32], stride: u32) -> Result<f64> {
        let v = call(
            &self.simplifier,
            "getScale",
            &[Float32Array::from(positions).into(), stride.into()],
        )?;
        v.as_f64().ok_or(Error::Invalid("meshopt scale"))
    }
    /// MeshoptSimplifier.simplify( indices, positions, stride, target, error,
    /// flags ): the index and the error.
    pub(super) fn simplify(
        &self,
        indices: &[u32],
        positions: &[f32],
        stride: u32,
        target: f64,
        error: f64,
        flags: &[&str],
    ) -> Result<(Vec<u32>, f64)> {
        let r = call(
            &self.simplifier,
            "simplify",
            &[
                Uint32Array::from(indices).into(),
                Float32Array::from(positions).into(),
                stride.into(),
                target.into(),
                error.into(),
                strings(flags),
            ],
        )?;
        Self::result(&r)
    }
    fn result(r: &JsValue) -> Result<(Vec<u32>, f64)> {
        let index = Reflect::get(r, &0.into()).map_err(js)?;
        let index = Uint32Array::new(&index).to_vec();
        let error = Reflect::get(r, &1.into())
            .map_err(js)?
            .as_f64()
            .unwrap_or(f64::NAN);
        Ok((index, error))
    }
    /// MeshoptSimplifier.simplifyWithAttributes( … ) with no vertex lock.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn simplify_with_attributes(
        &self,
        indices: &[u32],
        positions: &[f32],
        stride: u32,
        attributes: &[f32],
        attribute_stride: u32,
        weights: &[f64],
        target: f64,
        error: f64,
        flags: &[&str],
    ) -> Result<(Vec<u32>, f64)> {
        let r = call(
            &self.simplifier,
            "simplifyWithAttributes",
            &[
                Uint32Array::from(indices).into(),
                Float32Array::from(positions).into(),
                stride.into(),
                Float32Array::from(attributes).into(),
                attribute_stride.into(),
                numbers(weights),
                JsValue::NULL,
                target.into(),
                error.into(),
                strings(flags),
            ],
        )?;
        Self::result(&r)
    }
    /// MeshoptClusterizer.buildMeshlets( … ), computeMeshletBounds and
    /// extractMeshlet for each meshlet.
    pub(super) fn meshlets(
        &self,
        indices: &[u32],
        positions: &[f32],
        stride: u32,
        max_vertices: u32,
        max_triangles: u32,
        cone_weight: f64,
    ) -> Result<Vec<(Meshlet, Bounds)>> {
        let positions_js: JsValue = Float32Array::from(positions).into();
        let buffers = call(
            &self.clusterizer,
            "buildMeshlets",
            &[
                Uint32Array::from(indices).into(),
                positions_js.clone(),
                stride.into(),
                max_vertices.into(),
                max_triangles.into(),
                cone_weight.into(),
            ],
        )?;
        let bounds = call(
            &self.clusterizer,
            "computeMeshletBounds",
            &[buffers.clone(), positions_js, stride.into()],
        )?;
        let count = get(&buffers, "meshletCount")?.as_f64().unwrap_or(0.) as u32;
        let mut out = vec![];
        for m in 0..count {
            let meshlet = call(
                &self.clusterizer,
                "extractMeshlet",
                &[buffers.clone(), m.into()],
            )?;
            // Typed arrays ( Array.from reads their elements ).
            let list = |k: &str| -> Result<Vec<u32>> {
                Ok(Array::from(&get(&meshlet, k)?)
                    .iter()
                    .map(|x| x.as_f64().unwrap_or(0.) as u32)
                    .collect())
            };
            let b = Reflect::get(&bounds, &m.into()).map_err(js)?;
            let f = |k: &str| -> Result<f64> { Ok(get(&b, k)?.as_f64().unwrap_or(f64::NAN)) };
            out.push((
                Meshlet {
                    vertices: list("vertices")?,
                    triangles: list("triangles")?,
                },
                [f("centerX")?, f("centerY")?, f("centerZ")?, f("radius")?],
            ));
        }
        Ok(out)
    }
}
