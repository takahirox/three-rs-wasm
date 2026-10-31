//! Rhino3dmLoader ( examples/jsm/loaders/3DMLoader.js, r187 ) on rhino3dm
//! 8.32.1, the library the page loads from jsDelivr ( MIT, Robert McNeel &
//! Associates ). rhino3dm is an emscripten module with its own memory, so it
//! is loaded and called through JavaScript; the loader's code is ported: the
//! worker's decodeObjects ( extractObjectData, curveToPoints,
//! extractProperties, meshToThreejs ) and the main thread's _createGeometry,
//! _createObject, _createMaterial and _compareMaterials. The decode runs on
//! the page's thread, where the original posts it to a Web Worker. The
//! loader's userData ( views, named views, groups, strings, settings ) is
//! not built: nothing in the scene reads it.
use crate::attribute::BufferAttribute;
use crate::geometry::{Attribute, BufferGeometry};
use crate::{Error, Result};
use js_sys::{Array, Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

const LIBRARY_PATH: &str = "https://cdn.jsdelivr.net/npm/rhino3dm@8.32.1/";

#[wasm_bindgen(inline_js = r#"
// _initLibrary(): rhino3dm.js and rhino3dm.wasm from the library path, and
// the worker's init ( a module with wasmBinary, ready on
// onRuntimeInitialized ).
export async function rhino3dmInit( path ) {
	const [ js, wasmBinary ] = await Promise.all( [
		fetch( path + 'rhino3dm.js' ).then( r => r.text() ),
		fetch( path + 'rhino3dm.wasm' ).then( r => r.arrayBuffer() ),
	] );
	const rhino3dm = new Function( js + '\nreturn rhino3dm;' )();
	const RhinoModule = { wasmBinary };
	await new Promise( ( resolve ) => {
		RhinoModule.onRuntimeInitialized = resolve;
		rhino3dm( RhinoModule );
	} );
	return RhinoModule;
}
export function forInKeys( o ) { const keys = []; for ( const k in o ) keys.push( k ); return keys; }
export function typeOf( v ) { return typeof v; }
export function hasOwnConstructor( v ) { return v.hasOwnProperty( 'constructor' ); }
export function constructorName( v ) { return v.constructor.name; }
export function instanceOf( v, c ) { return v instanceof c; }
export function jsonStringify( v ) { return JSON.stringify( v ); }
export function toNumber( v ) { return Number( v ); }
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = rhino3dmInit)]
    fn rhino3dm_init(path: &str) -> std::result::Result<js_sys::Promise, JsValue>;
    #[wasm_bindgen(js_name = forInKeys)]
    fn for_in_keys(o: &JsValue) -> Array;
    #[wasm_bindgen(js_name = typeOf)]
    fn type_of(v: &JsValue) -> String;
    #[wasm_bindgen(catch, js_name = hasOwnConstructor)]
    fn has_own_constructor(v: &JsValue) -> std::result::Result<bool, JsValue>;
    #[wasm_bindgen(catch, js_name = constructorName)]
    fn constructor_name(v: &JsValue) -> std::result::Result<JsValue, JsValue>;
    #[wasm_bindgen(catch, js_name = instanceOf)]
    fn instance_of(v: &JsValue, c: &JsValue) -> std::result::Result<bool, JsValue>;
    #[wasm_bindgen(js_name = toNumber)]
    fn to_number(v: &JsValue) -> f64;
    #[wasm_bindgen(catch, js_name = jsonStringify)]
    fn json_stringify(v: &JsValue) -> std::result::Result<JsValue, JsValue>;
}

fn js_error(e: JsValue) -> Error {
    Error::Asset(format!("THREE.3DMLoader: {e:?}"))
}
/// `o[ key ]`.
fn get(o: &JsValue, key: &str) -> Result<JsValue> {
    Reflect::get(o, &JsValue::from_str(key)).map_err(js_error)
}
/// `o[ i ]`.
fn at(o: &JsValue, i: f64) -> Result<JsValue> {
    Reflect::get(o, &JsValue::from_f64(i)).map_err(js_error)
}
fn set(o: &JsValue, key: &str, v: &JsValue) -> Result<()> {
    Reflect::set(o, &JsValue::from_str(key), v)
        .map(|_| ())
        .map_err(js_error)
}
/// `o.method( …args )`.
fn call(o: &JsValue, method: &str, args: &[JsValue]) -> Result<JsValue> {
    let f: Function = get(o, method)?
        .dyn_into()
        .map_err(|_| Error::Asset(format!("THREE.3DMLoader: {method} is not a function")))?;
    f.apply(o, &args.iter().collect::<Array>())
        .map_err(js_error)
}
/// `new C( …args )`.
fn construct(c: &JsValue, args: &[JsValue]) -> Result<JsValue> {
    let c: Function = c
        .clone()
        .dyn_into()
        .map_err(|_| Error::Invalid("3dm constructor"))?;
    Reflect::construct(&c, &args.iter().collect::<Array>()).map_err(js_error)
}
/// The value as a number ( `Number( v )` ).
fn number(v: &JsValue) -> f64 {
    v.as_f64().unwrap_or_else(|| to_number(v))
}
fn object() -> JsValue {
    js_sys::Object::new().into()
}
fn truthy(v: &JsValue) -> bool {
    v.is_truthy()
}
fn same(a: &JsValue, b: &JsValue) -> bool {
    js_sys::Object::is(a, b)
}

/// rhino3dm, initialized as the loader's worker initializes it.
pub(super) async fn library() -> Result<JsValue> {
    JsFuture::from(rhino3dm_init(LIBRARY_PATH).map_err(js_error)?)
        .await
        .map_err(js_error)
}

/// extractProperties( object ).
fn extract_properties(o: &JsValue) -> Result<JsValue> {
    let result = object();
    for key in for_in_keys(o).iter() {
        let key = key.as_string().unwrap_or_default();
        let value = get(o, &key)?;
        let t = type_of(&value);
        if t == "function" {
            continue;
        }
        if t == "object" && !value.is_null() && has_own_constructor(&value).map_err(js_error)? {
            let v = object();
            set(&v, "name", &get(&get(&value, "constructor")?, "name")?)?;
            set(&v, "value", &get(&value, "value")?)?;
            set(&result, &key, &v)?;
        } else if t == "object" && !value.is_null() {
            set(&result, &key, &extract_properties(&value)?)?;
        } else {
            set(&result, &key, &value)?;
        }
    }
    Ok(result)
}

/// meshToThreejs( geom ).
fn mesh_to_threejs(geom: &JsValue) -> Result<JsValue> {
    if type_of(&get(geom, "toThreejsBuffers")?) == "function" {
        let b = call(geom, "toThreejsBuffers", &[JsValue::FALSE])?;
        let g = object();
        set(&g, "format", &JsValue::from_str("buffers"))?;
        set(&g, "position", &get(&b, "position")?)?;
        for key in ["normal", "index", "uv", "color"] {
            let v = get(&b, key)?;
            if truthy(&v) {
                set(&g, key, &v)?;
            }
        }
        return Ok(g);
    }
    call(geom, "toThreejsJSON", &[])
}

/// curveToPoints( curve, pointLimit ): the points as [ x, y, z ] arrays.
fn curve_to_points(rhino: &JsValue, curve: &JsValue, limit: f64) -> Result<Vec<JsValue>> {
    let is =
        |name: &str| -> Result<bool> { instance_of(curve, &get(rhino, name)?).map_err(js_error) };
    let mut count = limit;
    if is("LineCurve")? {
        return Ok(vec![get(curve, "pointAtStart")?, get(curve, "pointAtEnd")?]);
    }
    if is("PolylineCurve")? {
        let n = number(&get(curve, "pointCount")?);
        let mut rc = vec![];
        let mut i = 0.0;
        while i < n {
            rc.push(call(curve, "point", &[JsValue::from_f64(i)])?);
            i += 1.0;
        }
        return Ok(rc);
    }
    if is("PolyCurve")? {
        let n = number(&get(curve, "segmentCount")?);
        let mut rc = vec![];
        let mut i = 0.0;
        while i < n {
            let segment = call(curve, "segmentCurve", &[JsValue::from_f64(i)])?;
            rc.extend(curve_to_points(rhino, &segment, count)?);
            call(&segment, "delete", &[])?;
            i += 1.0;
        }
        return Ok(rc);
    }
    if is("ArcCurve")? {
        count = (number(&get(curve, "angleDegrees")?) / 5.0).floor();
        count = if count < 2.0 { 2.0 } else { count };
    }
    if is("NurbsCurve")? && number(&get(curve, "degree")?) == 1.0 {
        let line = call(curve, "tryGetPolyline", &[])?;
        let n = number(&get(&line, "count")?);
        let mut rc = vec![];
        let mut i = 0.0;
        while i < n {
            rc.push(call(&line, "get", &[JsValue::from_f64(i)])?);
            i += 1.0;
        }
        call(&line, "delete", &[])?;
        return Ok(rc);
    }
    let domain = get(curve, "domain")?;
    let (d0, d1) = (number(&at(&domain, 0.0)?), number(&at(&domain, 1.0)?));
    let divisions = count - 1.0;
    let mut ts: Vec<f64> = vec![];
    let mut j = 0.0;
    while j < count {
        let t = d0 + (j / divisions) * (d1 - d0);
        if t == d0 || t == d1 {
            ts.push(t);
            j += 1.0;
            continue;
        }
        let tan = call(curve, "tangentAt", &[JsValue::from_f64(t)])?;
        let prev = call(
            curve,
            "tangentAt",
            &[ts.last()
                .map_or(JsValue::UNDEFINED, |&v| JsValue::from_f64(v))],
        )?;
        let c = |v: &JsValue, i: f64| -> Result<f64> { Ok(number(&at(v, i)?)) };
        let ts_ = c(&tan, 0.0)? * c(&tan, 0.0)?
            + c(&tan, 1.0)? * c(&tan, 1.0)?
            + c(&tan, 2.0)? * c(&tan, 2.0)?;
        let pts = c(&prev, 0.0)? * c(&prev, 0.0)?
            + c(&prev, 1.0)? * c(&prev, 1.0)?
            + c(&prev, 2.0)? * c(&prev, 2.0)?;
        let denominator = (ts_ * pts).sqrt();
        let angle = if denominator == 0.0 {
            std::f64::consts::PI / 2.0
        } else {
            // tan.x … : the tangents are arrays, so these read undefined.
            let p = |v: &JsValue, k: &str| -> Result<f64> { Ok(number(&get(v, k)?)) };
            let theta = (p(&tan, "x")? * p(&prev, "x")?
                + p(&tan, "y")? * p(&prev, "y")?
                + p(&tan, "z")? * p(&prev, "z")?)
                / denominator;
            js_sys::Math::acos(js_sys::Math::max(-1.0, js_sys::Math::min(1.0, theta)))
        };
        if angle < 0.1 {
            j += 1.0;
            continue;
        }
        ts.push(t);
        j += 1.0;
    }
    ts.iter()
        .map(|&t| call(curve, "pointAt", &[JsValue::from_f64(t)]))
        .collect()
}

/// `objectType.constructor.name` past its prefix ( substring( n ) ).
fn type_name(v: &JsValue, prefix: usize) -> Result<String> {
    let name = constructor_name(v)
        .map_err(js_error)?
        .as_string()
        .unwrap_or_default();
    Ok(name.chars().skip(prefix).collect())
}

/// extractObjectData( object, doc, subdivisionLevel ).
fn extract_object_data(
    rhino: &JsValue,
    o: &JsValue,
    doc: &JsValue,
    subdivision: f64,
) -> Result<Option<JsValue>> {
    let geometry_ = call(o, "geometry", &[])?;
    let attributes_ = call(o, "attributes", &[])?;
    let object_type = get(&geometry_, "objectType")?;
    let types = get(rhino, "ObjectType")?;
    let is = |name: &str| -> Result<bool> { Ok(same(&object_type, &get(&types, name)?)) };
    let any = get(&get(rhino, "MeshType")?, "Any")?;
    let position_data = |array: Vec<f64>| -> Result<JsValue> {
        let position = object();
        set(&position, "itemSize", &JsValue::from_f64(3.0))?;
        set(&position, "type", &JsValue::from_str("Float32Array"))?;
        set(
            &position,
            "array",
            &array
                .iter()
                .map(|&v| JsValue::from_f64(v))
                .collect::<Array>(),
        )?;
        Ok(position)
    };
    let mut geometry = JsValue::UNDEFINED;
    if is("Curve")? {
        let pts = curve_to_points(rhino, &geometry_, 100.0)?;
        let mut array = vec![];
        for p in &pts {
            for k in 0..3 {
                array.push(number(&at(p, f64::from(k))?));
            }
        }
        let attributes = object();
        set(&attributes, "position", &position_data(array)?)?;
        let data = object();
        set(&data, "attributes", &attributes)?;
        geometry = object();
        set(&geometry, "data", &data)?;
    } else if is("Point")? {
        let pt = get(&geometry_, "location")?;
        let position = position_data(vec![
            number(&at(&pt, 0.0)?),
            number(&at(&pt, 1.0)?),
            number(&at(&pt, 2.0)?),
        ])?;
        let c = call(&attributes_, "drawColor", std::slice::from_ref(doc))?;
        let color = object();
        set(&color, "itemSize", &JsValue::from_f64(3.0))?;
        set(&color, "type", &JsValue::from_str("Float32Array"))?;
        let channel =
            |k: &str| -> Result<JsValue> { Ok(JsValue::from_f64(number(&get(&c, k)?) / 255.0)) };
        set(
            &color,
            "array",
            &[channel("r")?, channel("g")?, channel("b")?]
                .iter()
                .collect::<Array>(),
        )?;
        let attributes = object();
        set(&attributes, "position", &position)?;
        set(&attributes, "color", &color)?;
        let data = object();
        set(&data, "attributes", &attributes)?;
        geometry = object();
        set(&geometry, "data", &data)?;
    } else if is("PointSet")? || is("Mesh")? {
        geometry = mesh_to_threejs(&geometry_)?;
    } else if is("Brep")? {
        let faces = call(&geometry_, "faces", &[])?;
        let mesh = construct(&get(rhino, "Mesh")?, &[])?;
        let n = number(&get(&faces, "count")?);
        let mut i = 0.0;
        while i < n {
            let face = call(&faces, "get", &[JsValue::from_f64(i)])?;
            let m = call(&face, "getMesh", std::slice::from_ref(&any))?;
            if truthy(&m) {
                call(&mesh, "append", std::slice::from_ref(&m))?;
                call(&m, "delete", &[])?;
            }
            call(&face, "delete", &[])?;
            i += 1.0;
        }
        if number(&get(&call(&mesh, "faces", &[])?, "count")?) > 0.0 {
            call(&mesh, "compact", &[])?;
            geometry = mesh_to_threejs(&mesh)?;
            call(&faces, "delete", &[])?;
        }
        call(&mesh, "delete", &[])?;
    } else if is("Extrusion")? {
        let mesh = call(&geometry_, "getMesh", std::slice::from_ref(&any))?;
        if truthy(&mesh) {
            geometry = mesh_to_threejs(&mesh)?;
            call(&mesh, "delete", &[])?;
        }
    } else if is("TextDot")? || is("Light")? {
        geometry = extract_properties(&geometry_)?;
    } else if is("InstanceReference")? {
        geometry = extract_properties(&geometry_)?;
        let xform = get(&geometry_, "xform")?;
        let x = extract_properties(&xform)?;
        set(
            &x,
            "array",
            &call(&xform, "toFloatArray", &[JsValue::TRUE])?,
        )?;
        set(&geometry, "xform", &x)?;
    } else if is("SubD")? {
        call(&geometry_, "subdivide", &[JsValue::from_f64(subdivision)])?;
        let mesh = call(
            &get(rhino, "Mesh")?,
            "createFromSubDControlNet",
            &[geometry_.clone(), JsValue::FALSE],
        )?;
        if truthy(&mesh) {
            geometry = mesh_to_threejs(&mesh)?;
            call(&mesh, "delete", &[])?;
        }
    }
    if !truthy(&geometry) {
        return Ok(None);
    }
    let attributes = extract_properties(&attributes_)?;
    set(&attributes, "geometry", &extract_properties(&geometry_)?)?;
    if number(&get(&attributes_, "groupCount")?) > 0.0 {
        set(
            &attributes,
            "groupIds",
            &call(&attributes_, "getGroupList", &[])?,
        )?;
    }
    if number(&get(&attributes_, "userStringCount")?) > 0.0 {
        set(
            &attributes,
            "userStrings",
            &call(&attributes_, "getUserStrings", &[])?,
        )?;
    }
    if number(&get(&geometry_, "userStringCount")?) > 0.0 {
        set(
            &get(&attributes, "geometry")?,
            "userStrings",
            &call(&geometry_, "getUserStrings", &[])?,
        )?;
    }
    set(
        &attributes,
        "drawColor",
        &call(&attributes_, "drawColor", std::slice::from_ref(doc))?,
    )?;
    let result = object();
    set(&result, "geometry", &geometry)?;
    set(&result, "attributes", &attributes)?;
    set(
        &result,
        "objectType",
        &JsValue::from_str(&type_name(&object_type, 11)?),
    )?;
    Ok(Some(result))
}

/// extractTextures( m, tTypes, d ).
fn extract_textures(m: &JsValue, types: &[JsValue], doc: &JsValue) -> Result<Vec<JsValue>> {
    let mut out = vec![];
    for t in types {
        let texture = call(m, "getTexture", std::slice::from_ref(t))?;
        if truthy(&texture) {
            let tex = object();
            set(&tex, "type", &JsValue::from_str(&type_name(t, 12)?))?;
            let image = call(
                doc,
                "getEmbeddedFileAsBase64",
                &[get(&texture, "fileName")?],
            )?;
            for k in ["wrapU", "wrapV", "wrapW"] {
                set(&tex, k, &get(&texture, k)?)?;
            }
            let uvw = call(
                &get(&texture, "uvwTransform")?,
                "toFloatArray",
                &[JsValue::TRUE],
            )?;
            set(
                &tex,
                "repeat",
                &[at(&uvw, 0.0)?, at(&uvw, 5.0)?].iter().collect::<Array>(),
            )?;
            if truthy(&image) {
                set(
                    &tex,
                    "image",
                    &JsValue::from_str(&format!(
                        "data:image/png;base64,{}",
                        image.as_string().unwrap_or_default()
                    )),
                )?;
            } else {
                set(&tex, "image", &JsValue::NULL)?;
            }
            out.push(tex);
            call(&texture, "delete", &[])?;
        }
    }
    Ok(out)
}

/// The decoded file: decodeObjects' objects, materials, layers and render
/// environment.
pub(super) struct Decoded {
    pub objects: Vec<JsValue>,
    pub materials: Vec<JsValue>,
    pub layers: Vec<JsValue>,
    pub render_environment: bool,
}

/// decodeObjects( rhino, buffer, subdivisionLevel ).
pub(super) fn decode_objects(rhino: &JsValue, bytes: &[u8], subdivision: f64) -> Result<Decoded> {
    let doc = call(
        &get(rhino, "File3dm")?,
        "fromByteArray",
        &[js_sys::Uint8Array::from(bytes).into()],
    )?;
    let mut objects = vec![];
    let objs = call(&doc, "objects", &[])?;
    let count = number(&get(&objs, "count")?);
    let mut i = 0.0;
    while i < count {
        let o = call(&objs, "get", &[JsValue::from_f64(i)])?;
        let data = extract_object_data(rhino, &o, &doc, subdivision)?;
        call(&o, "delete", &[])?;
        if let Some(d) = data {
            objects.push(d);
        }
        i += 1.0;
    }
    let mut i = 0.0;
    while i < number(&get(&call(&doc, "instanceDefinitions", &[])?, "count")?) {
        let idef = call(
            &call(&doc, "instanceDefinitions", &[])?,
            "get",
            &[JsValue::from_f64(i)],
        )?;
        let attributes = extract_properties(&idef)?;
        set(&attributes, "objectIds", &call(&idef, "getObjectIds", &[])?)?;
        let o = object();
        set(&o, "geometry", &JsValue::NULL)?;
        set(&o, "attributes", &attributes)?;
        set(&o, "objectType", &JsValue::from_str("InstanceDefinition"))?;
        objects.push(o);
        i += 1.0;
    }
    let tt = get(rhino, "TextureType")?;
    let types =
        |names: &[&str]| -> Result<Vec<JsValue>> { names.iter().map(|n| get(&tt, n)).collect() };
    let texture_types = types(&["Diffuse", "Bump", "Transparency", "Opacity", "Emap"])?;
    let pbr_types = types(&[
        "PBR_BaseColor",
        "PBR_Subsurface",
        "PBR_SubsurfaceScattering",
        "PBR_SubsurfaceScatteringRadius",
        "PBR_Metallic",
        "PBR_Specular",
        "PBR_SpecularTint",
        "PBR_Roughness",
        "PBR_Anisotropic",
        "PBR_Anisotropic_Rotation",
        "PBR_Sheen",
        "PBR_SheenTint",
        "PBR_Clearcoat",
        "PBR_ClearcoatBump",
        "PBR_ClearcoatRoughness",
        "PBR_OpacityIor",
        "PBR_OpacityRoughness",
        "PBR_Emission",
        "PBR_AmbientOcclusion",
        "PBR_Displacement",
    ])?;
    let mut materials = vec![];
    let mut i = 0.0;
    while i < number(&get(&call(&doc, "materials", &[])?, "count")?) {
        let m = call(
            &call(&doc, "materials", &[])?,
            "get",
            &[JsValue::from_f64(i)],
        )?;
        let material = extract_properties(&m)?;
        let mut textures = extract_textures(&m, &texture_types, &doc)?;
        let pbr = call(&m, "physicallyBased", &[])?;
        let supported = get(&pbr, "supported")?;
        set(&material, "pbrSupported", &supported)?;
        if truthy(&supported) {
            textures.extend(extract_textures(&m, &pbr_types, &doc)?);
            set(
                &material,
                "pbr",
                &extract_properties(&call(&m, "physicallyBased", &[])?)?,
            )?;
        }
        set(&material, "textures", &textures.iter().collect::<Array>())?;
        materials.push(material);
        call(&m, "delete", &[])?;
        i += 1.0;
    }
    let mut layers = vec![];
    let mut i = 0.0;
    while i < number(&get(&call(&doc, "layers", &[])?, "count")?) {
        let l = call(&call(&doc, "layers", &[])?, "get", &[JsValue::from_f64(i)])?;
        layers.push(extract_properties(&l)?);
        call(&l, "delete", &[])?;
        i += 1.0;
    }
    // The render environment: the reflection environment's embedded texture.
    let settings = call(&doc, "settings", &[])?;
    let reflection = get(
        &get(
            &call(&settings, "renderSettings", &[])?,
            "renderEnvironments",
        )?,
        "reflectionId",
    )?;
    let rc = call(&doc, "renderContent", &[])?;
    let mut environment = false;
    let mut i = 0.0;
    while i < number(&get(&rc, "count")?) {
        let content = call(&rc, "get", &[JsValue::from_f64(i)])?;
        if get(&content, "kind")?.as_string().as_deref() == Some("environment")
            && same(&get(&content, "id")?, &reflection)
        {
            let file = get(
                &call(&content, "findChild", &[JsValue::from_str("texture")])?,
                "fileName",
            )?;
            let files = call(&doc, "embeddedFiles", &[])?;
            let mut j = 0.0;
            while j < number(&get(&files, "count")?) {
                if same(
                    &get(&call(&files, "get", &[JsValue::from_f64(j)])?, "fileName")?,
                    &file,
                ) {
                    environment = true;
                }
                j += 1.0;
            }
        }
        i += 1.0;
    }
    call(&doc, "delete", &[])?;
    Ok(Decoded {
        objects,
        materials,
        layers,
        render_environment: environment,
    })
}

/// A material the loader creates.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum MaterialType {
    Standard,
    Physical,
    LineBasic,
    Points,
}
#[derive(Clone, Debug)]
pub(super) struct LoaderMaterial {
    pub kind: MaterialType,
    pub name: String,
    pub color: [f64; 3],
    pub emissive: [f64; 3],
    pub specular_color: [f64; 3],
    pub sheen_color: [f64; 3],
    pub opacity: f64,
    pub transparent: bool,
    pub vertex_colors: bool,
    pub flat_shading: bool,
    pub double_sided: bool,
    pub metalness: f64,
    pub roughness: f64,
    pub ior: f64,
    pub clearcoat: f64,
    pub clearcoat_roughness: f64,
    pub sheen: f64,
    pub specular_intensity: f64,
    pub thickness: f64,
    pub anisotropy: f64,
    pub size: f64,
    pub size_attenuation: bool,
}
impl LoaderMaterial {
    fn new(kind: MaterialType) -> Self {
        LoaderMaterial {
            kind,
            name: String::new(),
            color: [1.0; 3],
            emissive: [0.0; 3],
            specular_color: [1.0; 3],
            sheen_color: [0.0; 3],
            opacity: 1.0,
            transparent: false,
            vertex_colors: false,
            flat_shading: false,
            double_sided: false,
            metalness: 0.0,
            roughness: 1.0,
            ior: 1.5,
            clearcoat: 0.0,
            clearcoat_roughness: 0.0,
            sheen: 0.0,
            specular_intensity: 1.0,
            thickness: 0.0,
            anisotropy: 0.0,
            size: 1.0,
            size_attenuation: true,
        }
    }
    /// The JSON _compareMaterials compares: name, color, type, vertexColors
    /// ( JSON.stringify writes NaN and the infinities as null, -0 as 0 ).
    fn key(&self) -> (String, [Option<u64>; 3], MaterialTypeKey, bool) {
        let n = |v: f64| {
            v.is_finite()
                .then(|| if v == 0.0 { 0 } else { v.to_bits() })
        };
        (
            self.name.clone(),
            self.color.map(n),
            MaterialTypeKey(self.kind.clone()),
            self.vertex_colors,
        )
    }
}
#[derive(Debug, PartialEq)]
pub(super) struct MaterialTypeKey(MaterialType);

/// The loader's objects; the page reads the lights' colour and intensity only.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(super) enum NodeKind {
    Object3D,
    Mesh,
    Line,
    Points,
    /// A text dot: text, font height and face, the dot colour ( r, g, b, a )
    /// and its point.
    Sprite {
        text: String,
        font_height: f64,
        font_face: String,
        color: [f64; 4],
    },
    PointLight,
    SpotLight {
        angle: f64,
        target: [f64; 3],
    },
    RectAreaLight {
        width: f64,
        height: f64,
    },
    DirectionalLight {
        target: [f64; 3],
    },
}

#[derive(Clone, Debug)]
pub(super) struct Node {
    pub kind: NodeKind,
    pub name: String,
    pub parent: Option<usize>,
    pub position: [f64; 3],
    pub quaternion: [f64; 4],
    pub scale: [f64; 3],
    pub visible: bool,
    pub layer_index: Option<f64>,
    pub geometry: Option<std::sync::Arc<BufferGeometry>>,
    pub material: Option<usize>,
    /// A light's colour and intensity.
    pub light: Option<([f64; 3], f64)>,
    pub children: Vec<usize>,
    /// The object's attributes ( userData.attributes ) id, for instance
    /// definitions.
    pub id: JsValue,
}
impl Node {
    fn new(kind: NodeKind) -> Self {
        Node {
            kind,
            name: String::new(),
            parent: None,
            position: [0.0; 3],
            quaternion: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
            visible: true,
            layer_index: None,
            geometry: None,
            material: None,
            light: None,
            children: vec![],
            id: JsValue::UNDEFINED,
        }
    }
}

/// The loader's object tree: nodes ( index 0 the root Object3D ) and the
/// materials _compareMaterials kept.
pub(super) struct Tree {
    pub nodes: Vec<Node>,
    pub materials: Vec<LoaderMaterial>,
}

fn rgb255(c: &JsValue) -> Result<[f64; 3]> {
    Ok([
        number(&get(c, "r")?) / 255.0,
        number(&get(c, "g")?) / 255.0,
        number(&get(c, "b")?) / 255.0,
    ])
}

/// _geometryFromData( data ).
fn geometry_from_data(data: &JsValue) -> Result<BufferGeometry> {
    if truthy(data) && get(data, "format")?.as_string().as_deref() == Some("buffers") {
        let mut g = BufferGeometry::default();
        for (key, size) in [("position", 3), ("normal", 3), ("uv", 2), ("color", 3)] {
            let v = get(data, key)?;
            if truthy(&v) && number(&get(&v, "length")?) > 0.0 {
                let array = js_sys::Float32Array::new(&v).to_vec();
                g.set_attribute(
                    key,
                    Attribute::F32(BufferAttribute::new(array, size, false)?),
                );
            }
        }
        let index = get(data, "index")?;
        if truthy(&index) && number(&get(&index, "length")?) > 0.0 {
            g.set_index(Some(js_sys::Uint32Array::new(&index).to_vec()));
        }
        return Ok(g);
    }
    let json = json_stringify(data)
        .map_err(js_error)?
        .as_string()
        .unwrap_or_default();
    crate::buffer_geometry_loader::parse(json.as_bytes())
}

/// Matrix4.lookAt( eye, target, up ) then Quaternion.setFromRotationMatrix.
fn look_at_quaternion(eye: [f64; 3], target: [f64; 3], up: [f64; 3]) -> [f64; 4] {
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let len_sq = |a: [f64; 3]| a[0] * a[0] + a[1] * a[1] + a[2] * a[2];
    // normalize(): divideScalar( length || 1 ), a multiply by the inverse.
    let normalize = |a: [f64; 3]| {
        let l = len_sq(a).sqrt();
        let l = 1.0 / if l == 0.0 || l.is_nan() { 1.0 } else { l };
        [a[0] * l, a[1] * l, a[2] * l]
    };
    let cross = |a: [f64; 3], b: [f64; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let mut z = sub(eye, target);
    if len_sq(z) == 0.0 {
        z[2] = 1.0;
    }
    z = normalize(z);
    let mut x = cross(up, z);
    if len_sq(x) == 0.0 {
        if up[2].abs() == 1.0 {
            z[0] += 0.0001;
        } else {
            z[2] += 0.0001;
        }
        z = normalize(z);
        x = cross(up, z);
    }
    x = normalize(x);
    let y = cross(z, x);
    // setFromRotationMatrix
    let (m11, m12, m13) = (x[0], y[0], z[0]);
    let (m21, m22, m23) = (x[1], y[1], z[1]);
    let (m31, m32, m33) = (x[2], y[2], z[2]);
    let trace = m11 + m22 + m33;
    if trace > 0.0 {
        let s = 0.5 / (trace + 1.0).sqrt();
        [(m32 - m23) * s, (m13 - m31) * s, (m21 - m12) * s, 0.25 / s]
    } else if m11 > m22 && m11 > m33 {
        let s = 2.0 * (1.0 + m11 - m22 - m33).sqrt();
        [0.25 * s, (m12 + m21) / s, (m13 + m31) / s, (m32 - m23) / s]
    } else if m22 > m33 {
        let s = 2.0 * (1.0 + m22 - m11 - m33).sqrt();
        [(m12 + m21) / s, 0.25 * s, (m23 + m32) / s, (m13 - m31) / s]
    } else {
        let s = 2.0 * (1.0 + m33 - m11 - m22).sqrt();
        [(m13 + m31) / s, (m23 + m32) / s, 0.25 * s, (m21 - m12) / s]
    }
}

struct Builder {
    tree: Tree,
}
impl Builder {
    /// _compareMaterials( material ).
    fn compare_materials(&mut self, m: LoaderMaterial) -> usize {
        let key = m.key();
        if let Some(i) = self.tree.materials.iter().position(|x| x.key() == key) {
            return i;
        }
        self.tree.materials.push(m);
        self.tree.materials.len() - 1
    }
    /// _createMaterial( material, renderEnvironment ).
    fn create_material(&mut self, material: &JsValue, environment: bool) -> Result<LoaderMaterial> {
        if material.is_undefined() {
            let mut m = LoaderMaterial::new(MaterialType::Standard);
            m.metalness = 0.8;
            m.name = "__DEFAULT".into();
            m.double_sided = true;
            return Ok(m);
        }
        let mut m = LoaderMaterial::new(MaterialType::Physical);
        m.color = rgb255(&get(material, "diffuseColor")?)?;
        m.emissive = rgb255(&get(material, "emissionColor")?)?;
        m.flat_shading = truthy(&get(material, "disableLighting")?);
        m.ior = number(&get(material, "indexOfRefraction")?);
        m.name = get(material, "name")?.as_string().unwrap_or_default();
        // The reflectivity setter, after ior in the parameters' order.
        let r = number(&get(material, "reflectivity")?);
        m.ior = (1.0 + 0.4 * r) / (1.0 - 0.4 * r);
        let transparency = number(&get(material, "transparency")?);
        m.opacity = 1.0 - transparency;
        m.double_sided = true;
        m.specular_color = rgb255(&get(material, "specularColor")?)?;
        m.transparent = transparency > 0.0;
        if truthy(&get(material, "pbrSupported")?) {
            let pbr = get(material, "pbr")?;
            let f = |k: &str| -> Result<f64> { Ok(number(&get(&pbr, k)?)) };
            m.anisotropy = f("anisotropic")?;
            let base = get(&pbr, "baseColor")?;
            m.color = [
                number(&get(&base, "r")?),
                number(&get(&base, "g")?),
                number(&get(&base, "b")?),
            ];
            m.clearcoat = f("clearcoat")?;
            m.clearcoat_roughness = f("clearcoatRoughness")?;
            m.metalness = f("metallic")?;
            m.roughness = f("roughness")?;
            m.sheen = f("sheen")?;
            m.specular_intensity = f("specular")?;
            m.thickness = f("subsurface")?;
            m.opacity = f("opacity")?;
            m.transparent = m.opacity < 1.0;
        }
        let textures = get(material, "textures")?;
        for t in Array::from(&textures).iter() {
            if !get(&t, "image")?.is_null() {
                return Err(Error::Asset(
                    "THREE.3DMLoader: textured materials are not supported by the port".into(),
                ));
            }
        }
        if environment {
            return Err(Error::Asset(
                "THREE.3DMLoader: the render environment is not supported by the port".into(),
            ));
        }
        Ok(m)
    }
    fn node(&mut self, n: Node) -> usize {
        self.tree.nodes.push(n);
        self.tree.nodes.len() - 1
    }
    fn add(&mut self, parent: usize, child: usize) {
        if let Some(old) = self.tree.nodes[child].parent.take() {
            self.tree.nodes[old].children.retain(|&c| c != child);
        }
        self.tree.nodes[child].parent = Some(parent);
        self.tree.nodes[parent].children.push(child);
    }
    /// _createObject( obj, mat ).
    fn create_object(
        &mut self,
        obj: &JsValue,
        mat: Option<LoaderMaterial>,
    ) -> Result<Option<usize>> {
        let attributes = get(obj, "attributes")?;
        let object_type = get(obj, "objectType")?.as_string().unwrap_or_default();
        let name = get(&attributes, "name")?;
        let name = if truthy(&name) {
            name.as_string().unwrap_or_default()
        } else {
            String::new()
        };
        let mut node = match object_type.as_str() {
            "Point" | "PointSet" => {
                let geometry = geometry_from_data(&get(obj, "geometry")?)?;
                let mut m = LoaderMaterial::new(MaterialType::Points);
                if geometry.attributes.contains_key("color") {
                    m.vertex_colors = true;
                } else {
                    m.color = rgb255(&get(&attributes, "drawColor")?)?;
                }
                m.size_attenuation = false;
                m.size = 2.0;
                let mut n = Node::new(NodeKind::Points);
                n.material = Some(self.compare_materials(m));
                n.geometry = Some(std::sync::Arc::new(geometry));
                n
            }
            "Mesh" | "Extrusion" | "SubD" | "Brep" => {
                let data = get(obj, "geometry")?;
                if data.is_null() {
                    return Ok(None);
                }
                let geometry = geometry_from_data(&data)?;
                let mut m = match mat {
                    Some(m) => m,
                    None => self.create_material(&JsValue::UNDEFINED, false)?,
                };
                if geometry.attributes.contains_key("color") {
                    m.vertex_colors = true;
                }
                let mut n = Node::new(NodeKind::Mesh);
                n.material = Some(self.compare_materials(m));
                n.geometry = Some(std::sync::Arc::new(geometry));
                n
            }
            "Curve" => {
                let geometry = geometry_from_data(&get(obj, "geometry")?)?;
                let mut m = LoaderMaterial::new(MaterialType::LineBasic);
                m.color = rgb255(&get(&attributes, "drawColor")?)?;
                let mut n = Node::new(NodeKind::Line);
                n.material = Some(self.compare_materials(m));
                n.geometry = Some(std::sync::Arc::new(geometry));
                n
            }
            "TextDot" => {
                let g = get(obj, "geometry")?;
                let c = get(&attributes, "drawColor")?;
                let ch = |k: &str| -> Result<f64> { Ok(number(&get(&c, k)?)) };
                let point = get(&g, "point")?;
                let mut n = Node::new(NodeKind::Sprite {
                    text: get(&g, "text")?.as_string().unwrap_or_default(),
                    font_height: number(&get(&g, "fontHeight")?),
                    font_face: get(&g, "fontFace")?.as_string().unwrap_or_default(),
                    color: [ch("r")?, ch("g")?, ch("b")?, ch("a")?],
                });
                n.position = [
                    number(&at(&point, 0.0)?),
                    number(&at(&point, 1.0)?),
                    number(&at(&point, 2.0)?),
                ];
                n
            }
            "Light" => {
                let g = get(obj, "geometry")?;
                let v3 = |k: &str| -> Result<[f64; 3]> {
                    let v = get(&g, k)?;
                    Ok([
                        number(&at(&v, 0.0)?),
                        number(&at(&v, 1.0)?),
                        number(&at(&v, 2.0)?),
                    ])
                };
                let style = get(&get(&g, "lightStyle")?, "name")?
                    .as_string()
                    .unwrap_or_default();
                let mut n = match style.as_str() {
                    "LightStyle_WorldPoint" => {
                        let mut n = Node::new(NodeKind::PointLight);
                        n.position = v3("location")?;
                        n
                    }
                    "LightStyle_WorldSpot" => {
                        let mut n = Node::new(NodeKind::SpotLight {
                            angle: number(&get(&g, "spotAngleRadians")?),
                            target: v3("direction")?,
                        });
                        n.position = v3("location")?;
                        n
                    }
                    "LightStyle_WorldRectangular" => {
                        let width = number(&at(&get(&g, "width")?, 2.0)?).abs();
                        let height = number(&at(&get(&g, "length")?, 0.0)?).abs();
                        let l = v3("location")?;
                        let mut n = Node::new(NodeKind::RectAreaLight { width, height });
                        n.position = [l[0] - height / 2.0, l[1], l[2] - width / 2.0];
                        // lookAt(): a light looks from its position.
                        n.quaternion =
                            look_at_quaternion(n.position, v3("direction")?, [0.0, 0.0, 1.0]);
                        n
                    }
                    "LightStyle_WorldDirectional" => {
                        let mut n = Node::new(NodeKind::DirectionalLight {
                            target: v3("direction")?,
                        });
                        n.position = v3("location")?;
                        n
                    }
                    _ => return Ok(None),
                };
                n.light = Some((
                    rgb255(&get(&g, "diffuse")?)?,
                    number(&get(&g, "intensity")?),
                ));
                // Light objects keep their default name.
                let _ = &name;
                n.id = get(&attributes, "id")?;
                let i = self.node(n);
                return Ok(Some(i));
            }
            _ => return Ok(None),
        };
        node.name = name;
        node.id = get(&attributes, "id")?;
        Ok(Some(self.node(node)))
    }
    /// A deep copy ( Object3D.clone( true ) ).
    fn clone_tree(&mut self, n: usize) -> usize {
        let mut copy = self.tree.nodes[n].clone();
        copy.parent = None;
        copy.children = vec![];
        let c = self.node(copy);
        for child in self.tree.nodes[n].children.clone() {
            let cc = self.clone_tree(child);
            self.add(c, cc);
        }
        c
    }
}

/// _createGeometry( data ): the object tree.
pub(super) fn create_geometry(data: &Decoded) -> Result<Tree> {
    let mut b = Builder {
        tree: Tree {
            nodes: vec![],
            materials: vec![],
        },
    };
    let root = b.node(Node::new(NodeKind::Object3D));
    let mut definition_objects = vec![];
    let mut definitions = vec![];
    let mut references = vec![];
    for obj in &data.objects {
        let attributes = get(obj, "attributes")?;
        match get(obj, "objectType")?.as_string().as_deref() {
            Some("InstanceDefinition") => definitions.push(obj.clone()),
            Some("InstanceReference") => references.push(obj.clone()),
            _ => {
                let source = get(&get(&attributes, "materialSource")?, "name")?
                    .as_string()
                    .unwrap_or_default();
                let layer_index = number(&get(&attributes, "layerIndex")?);
                // matId stays null for other sources, and null >= 0.
                let mut mat_id: Option<f64> = None;
                match source.as_str() {
                    "ObjectMaterialSource_MaterialFromLayer" if layer_index >= 0.0 => {
                        let layer = data
                            .layers
                            .get(layer_index as usize)
                            .cloned()
                            .unwrap_or(JsValue::UNDEFINED);
                        mat_id = Some(number(&get(&layer, "renderMaterialIndex")?));
                    }
                    "ObjectMaterialSource_MaterialFromObject" => {
                        let mi = number(&get(&attributes, "materialIndex")?);
                        if mi >= 0.0 {
                            mat_id = Some(mi);
                        }
                    }
                    _ => {}
                }
                let material = match mat_id {
                    Some(id) if id < 0.0 || id.is_nan() => None,
                    Some(id) => {
                        let r = data
                            .materials
                            .get(id as usize)
                            .cloned()
                            .unwrap_or(JsValue::UNDEFINED);
                        Some(b.create_material(&r, data.render_environment)?)
                    }
                    None => Some(b.create_material(&JsValue::UNDEFINED, data.render_environment)?),
                };
                let Some(o) = b.create_object(obj, material)? else {
                    continue;
                };
                let layer = (layer_index >= 0.0 && layer_index.fract() == 0.0)
                    .then(|| data.layers.get(layer_index as usize))
                    .flatten();
                b.tree.nodes[o].visible = match layer {
                    Some(l) => truthy(&get(l, "visible")?),
                    None => true,
                };
                b.tree.nodes[o].layer_index = Some(layer_index);
                if truthy(&get(&attributes, "isInstanceDefinitionObject")?) {
                    definition_objects.push(o);
                } else {
                    b.add(root, o);
                }
            }
        }
    }
    for def in &definitions {
        let ids = Array::from(&get(&get(def, "attributes")?, "objectIds")?);
        let mut objects = vec![];
        for id in ids.iter() {
            for &o in &definition_objects {
                if same(&id, &b.tree.nodes[o].id) {
                    objects.push(o);
                }
            }
        }
        let def_id = get(&get(def, "attributes")?, "id")?;
        for r in &references {
            if !same(&get(&get(r, "geometry")?, "parentIdefId")?, &def_id) {
                continue;
            }
            let xf = Array::from(&get(&get(&get(r, "geometry")?, "xform")?, "array")?);
            let e: Vec<f64> = xf.iter().map(|v| number(&v)).collect();
            if e.len() < 16 {
                return Err(Error::Invalid("3dm xform"));
            }
            // Matrix4.set( ...xf ) ( row-major ), then applyMatrix4's
            // decompose.
            let m = glam::DMat4::from_cols_array(&[
                e[0], e[4], e[8], e[12], e[1], e[5], e[9], e[13], e[2], e[6], e[10], e[14], e[3],
                e[7], e[11], e[15],
            ]);
            let (scale, rotation, translation) = m.to_scale_rotation_translation();
            let mut node = Node::new(NodeKind::Object3D);
            node.position = translation.to_array();
            node.quaternion = rotation.to_array();
            node.scale = scale.to_array();
            let n = b.node(node);
            for &o in &objects {
                let c = b.clone_tree(o);
                b.add(n, c);
            }
            b.add(root, n);
        }
    }
    Ok(b.tree)
}

/// The nodes in traverse() order ( parents before children ), as indices.
pub(super) fn traverse(tree: &Tree) -> Vec<usize> {
    let mut out = vec![];
    let mut stack = vec![0usize];
    while let Some(n) = stack.pop() {
        out.push(n);
        for &c in tree.nodes[n].children.iter().rev() {
            stack.push(c);
        }
    }
    out
}

/// A canonical text form of the tree, for comparing the port with the
/// original loader ( tests/browser/rhino3dm-loader.spec.js ): numbers as the
/// hexadecimal bits of their double, buffers as their FNV-1a hash, the
/// materials in first-reference order. Sprites keep their position, not
/// their canvas-measured scale.
pub(super) fn canonical(tree: &Tree) -> String {
    let num = |v: f64| {
        if v.is_nan() {
            "\"NaN\"".to_string()
        } else {
            format!("\"{:016x}\"", v.to_bits())
        }
    };
    let nums = |v: &[f64]| {
        format!(
            "[{}]",
            v.iter().map(|&x| num(x)).collect::<Vec<_>>().join(",")
        )
    };
    let hash = |bytes: &[u8], count: usize| {
        let mut h: u32 = 0x811c_9dc5;
        for &b in bytes {
            h ^= u32::from(b);
            h = h.wrapping_mul(0x0100_0193);
        }
        format!("\"{count}:{h:08x}\"")
    };
    let text = |s: &str| serde_json::to_string(s).unwrap_or_default();
    let order = traverse(tree);
    let mut materials: Vec<usize> = vec![];
    for &i in &order {
        if let Some(m) = tree.nodes[i].material
            && !materials.contains(&m)
        {
            materials.push(m);
        }
    }
    let position_of = |i: usize| order.iter().position(|&x| x == i);
    let mut out = String::from("{\"nodes\":[");
    for (k, &i) in order.iter().enumerate() {
        if k > 0 {
            out.push(',');
        }
        let n = &tree.nodes[i];
        let kind = match n.kind {
            NodeKind::Object3D => "Object3D",
            NodeKind::Mesh => "Mesh",
            NodeKind::Line => "Line",
            NodeKind::Points => "Points",
            NodeKind::Sprite { .. } => "Sprite",
            NodeKind::PointLight => "PointLight",
            NodeKind::SpotLight { .. } => "SpotLight",
            NodeKind::RectAreaLight { .. } => "RectAreaLight",
            NodeKind::DirectionalLight { .. } => "DirectionalLight",
        };
        let geometry = n.geometry.as_ref().map_or("null".to_string(), |g| {
            let attributes = g
                .attributes
                .iter()
                .map(|(name, a)| match a {
                    Attribute::F32(a) => format!(
                        "[{},{},{}]",
                        text(name),
                        a.item_size(),
                        hash(
                            &a.array()
                                .iter()
                                .flat_map(|x| x.to_le_bytes())
                                .collect::<Vec<_>>(),
                            a.array().len()
                        )
                    ),
                    _ => format!("[{},null]", text(name)),
                })
                .collect::<Vec<_>>()
                .join(",");
            let index = g.index.as_ref().map_or("null".to_string(), |ix| {
                hash(
                    &ix.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<_>>(),
                    ix.len(),
                )
            });
            format!("[[{attributes}],{index}]")
        });
        let scale = if matches!(n.kind, NodeKind::Sprite { .. }) {
            "null".to_string()
        } else {
            nums(&n.scale)
        };
        out.push_str(&format!(
            "[{},{},{},{},{},{},{},{},{},{},{}]",
            text(kind),
            text(&n.name),
            n.parent
                .and_then(position_of)
                .map_or("null".into(), |p| p.to_string()),
            nums(&n.position),
            nums(&n.quaternion),
            scale,
            n.visible,
            n.layer_index.map_or("null".into(), num),
            geometry,
            n.material
                .and_then(|m| materials.iter().position(|&x| x == m))
                .map_or("null".into(), |m| m.to_string()),
            n.light
                .map_or("null".into(), |(c, i)| format!("[{},{}]", nums(&c), num(i)))
        ));
    }
    out.push_str("],\"materials\":[");
    for (k, &m) in materials.iter().enumerate() {
        if k > 0 {
            out.push(',');
        }
        let m = &tree.materials[m];
        let kind = match m.kind {
            MaterialType::Standard => "MeshStandardMaterial",
            MaterialType::Physical => "MeshPhysicalMaterial",
            MaterialType::LineBasic => "LineBasicMaterial",
            MaterialType::Points => "PointsMaterial",
        };
        let physical = m.kind == MaterialType::Physical;
        let lit = physical || m.kind == MaterialType::Standard;
        let points = m.kind == MaterialType::Points;
        let or_null = |b: bool, v: String| if b { v } else { "null".into() };
        out.push_str(&format!(
            "[{},{},{},{},{},{},{},{},{},{},{},{},{}]",
            text(kind),
            text(&m.name),
            nums(&m.color),
            nums(&[m.opacity]),
            m.transparent,
            if m.double_sided { 2 } else { 0 },
            m.vertex_colors,
            m.flat_shading,
            or_null(
                lit,
                nums(&[
                    m.emissive[0],
                    m.emissive[1],
                    m.emissive[2],
                    m.metalness,
                    m.roughness
                ])
            ),
            or_null(
                physical,
                nums(&[
                    m.ior,
                    m.clearcoat,
                    m.clearcoat_roughness,
                    m.sheen,
                    m.specular_intensity,
                    m.thickness,
                    m.anisotropy
                ])
            ),
            or_null(physical, nums(&m.specular_color)),
            or_null(physical, nums(&m.sheen_color)),
            or_null(points, format!("[{},{}]", num(m.size), m.size_attenuation))
        ));
    }
    out.push_str("]}");
    out
}
