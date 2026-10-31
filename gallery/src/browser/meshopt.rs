//! meshoptimizer's simplifier, the build webgl_batch_lod_bvh loads:
//! meshoptimizer 1.1.1 from jsDelivr ( through @three.ez/simplify-geometry ).
//! Its wasm module has its own memory, so it is called through JavaScript.
use crate::{Error, Result};
use js_sys::{Array, Float32Array, Function, Reflect, Uint32Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(inline_js = r#"
export async function meshoptimizer( url ) {
	const { MeshoptSimplifier } = await import( url );
	await MeshoptSimplifier.ready;
	return { MeshoptSimplifier };
}
"#)]
extern "C" {
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

/// MeshoptSimplifier.
pub(super) struct Meshopt {
    simplifier: JsValue,
}

impl Meshopt {
    /// meshoptimizer's ESM bundle ( the simplifier only ).
    pub(super) async fn package(url: &str) -> Result<Self> {
        let m = JsFuture::from(meshoptimizer_module(url).map_err(js)?)
            .await
            .map_err(js)?;
        Ok(Meshopt {
            simplifier: get(&m, "MeshoptSimplifier")?,
        })
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
        let index = Reflect::get(&r, &0.into()).map_err(js)?;
        let index = Uint32Array::new(&index).to_vec();
        let error = Reflect::get(&r, &1.into())
            .map_err(js)?
            .as_f64()
            .unwrap_or(f64::NAN);
        Ok((index, error))
    }
}
