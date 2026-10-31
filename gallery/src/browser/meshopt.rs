//! meshoptimizer's simplifier and clusterizer, the builds the pages load:
//! three.js r187's bundled meshopt_simplifier / meshopt_clusterizer modules
//! ( MeshletGeometryUtils, which MeshletDrawingPipeline runs for
//! webgpu_lightprobes_sponza ) and meshoptimizer 1.1.1 from jsDelivr
//! ( webgl_batch_lod_bvh, through @three.ez/simplify-geometry ). Their wasm
//! modules have their own memory, so they are called through JavaScript.
use crate::{Error, Result};
use js_sys::{Array, Float32Array, Function, Reflect, Uint8Array, Uint32Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(inline_js = r#"
// The modules three.js bundles in examples/jsm/libs/, ready as
// MeshletGeometryUtils awaits them.
export async function meshoptThree( simplifierUrl, clusterizerUrl ) {
	const [ { MeshoptSimplifier }, { MeshoptClusterizer } ] = await Promise.all( [
		import( new URL( simplifierUrl, document.baseURI ).href ),
		import( new URL( clusterizerUrl, document.baseURI ).href ),
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

/// The meshlets of MeshoptClusterizer.buildMeshlets(): per meshlet its
/// vertex offset, triangle offset, vertex count and triangle count, the
/// meshlets' vertex indices and their local triangle indices.
pub(super) struct Meshlets {
    pub meshlets: Vec<u32>,
    pub vertices: Vec<u32>,
    pub triangles: Vec<u8>,
    pub count: usize,
}
impl Meshlets {
    /// MeshoptClusterizer.extractMeshlet( buffers, index ): the meshlet's
    /// vertices and triangles.
    pub(super) fn extract(&self, index: usize) -> (&[u32], &[u8]) {
        let m = &self.meshlets[index * 4..index * 4 + 4];
        let (vertex_offset, triangle_offset) = (m[0] as usize, m[1] as usize);
        (
            &self.vertices[vertex_offset..vertex_offset + m[2] as usize],
            &self.triangles[triangle_offset..triangle_offset + m[3] as usize * 3],
        )
    }
}
/// computeMeshletBounds()'s bounds: the sphere ( center and radius ) and the
/// normal cone ( axis and cutoff ).
#[derive(Clone, Copy)]
pub(super) struct Bounds {
    pub center: [f32; 3],
    pub radius: f32,
    pub cone_axis: [f32; 3],
    pub cone_cutoff: f32,
}

/// MeshoptSimplifier, and MeshoptClusterizer when the modules are three's.
pub(super) struct Meshopt {
    simplifier: JsValue,
    clusterizer: JsValue,
}

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
    fn result(r: &JsValue) -> Result<(Vec<u32>, f64)> {
        let index = Reflect::get(r, &0.into()).map_err(js)?;
        let index = Uint32Array::new(&index).to_vec();
        let error = Reflect::get(r, &1.into())
            .map_err(js)?
            .as_f64()
            .unwrap_or(f64::NAN);
        Ok((index, error))
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
    /// MeshoptSimplifier.getScale( positions, stride ).
    pub(super) fn get_scale(&self, positions: &[f32], stride: u32) -> Result<f64> {
        let v = call(
            &self.simplifier,
            "getScale",
            &[Float32Array::from(positions).into(), stride.into()],
        )?;
        v.as_f64().ok_or(Error::Invalid("meshopt scale"))
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
    /// MeshoptSimplifier.simplifySloppy( indices, positions, stride, null,
    /// target, error ).
    pub(super) fn simplify_sloppy(
        &self,
        indices: &[u32],
        positions: &[f32],
        stride: u32,
        target: f64,
        error: f64,
    ) -> Result<(Vec<u32>, f64)> {
        let r = call(
            &self.simplifier,
            "simplifySloppy",
            &[
                Uint32Array::from(indices).into(),
                Float32Array::from(positions).into(),
                stride.into(),
                JsValue::NULL,
                target.into(),
                error.into(),
            ],
        )?;
        Self::result(&r)
    }
    /// MeshoptClusterizer.buildMeshlets( indices, positions, stride,
    /// maxVertices, maxTriangles, coneWeight ) and computeMeshletBounds() of
    /// its meshlets.
    pub(super) fn build_meshlets(
        &self,
        indices: &[u32],
        positions: &[f32],
        stride: u32,
        max_vertices: u32,
        max_triangles: u32,
        cone_weight: f64,
    ) -> Result<(Meshlets, Vec<Bounds>)> {
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
        let count = get(&buffers, "meshletCount")?.as_f64().unwrap_or(0.) as usize;
        let meshlets = Meshlets {
            meshlets: Uint32Array::new(&get(&buffers, "meshlets")?).to_vec(),
            vertices: Uint32Array::new(&get(&buffers, "vertices")?).to_vec(),
            triangles: Uint8Array::new(&get(&buffers, "triangles")?).to_vec(),
            count,
        };
        let bounds = (0..count)
            .map(|m| {
                let b = Reflect::get(&bounds, &(m as u32).into()).map_err(js)?;
                let f = |k: &str| -> Result<f32> {
                    Ok(get(&b, k)?.as_f64().unwrap_or(f64::NAN) as f32)
                };
                Ok(Bounds {
                    center: [f("centerX")?, f("centerY")?, f("centerZ")?],
                    radius: f("radius")?,
                    cone_axis: [f("coneAxisX")?, f("coneAxisY")?, f("coneAxisZ")?],
                    cone_cutoff: f("coneCutoff")?,
                })
            })
            .collect::<Result<_>>()?;
        Ok((meshlets, bounds))
    }
}
