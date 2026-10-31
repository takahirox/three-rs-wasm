//! webgpu_loader_materialx's shaders, built as the page builds them: its
//! scene ( the white background, the san_giuseppe_bridge environment, the
//! LinearToneMapping renderer at exposure 0.5 ), the TSL ground grid, and
//! each MaterialX document through the transpiled MaterialXLoader with the
//! page's options, its material on the ShaderBall's non-indexed meshes ( and
//! the hidden UV plane ), compiled with `compileAsync` before the model joins
//! the scene. The renderer runs headless; the WGSL is the compiled render
//! objects' ( as `renderer.debug.getShaderAsync` finds them ). three's
//! modules evaluate in the order of the build the page loads
//! ( `init_three_bundled` ), which names the shaders' layout Fns.
//! Images are not loaded ( the shaders do not depend on their texels ): a
//! loading manager handler stands for the document's texture loader and the
//! page's EXR, TIFF and HDR handlers, and hands each texture a placeholder
//! naming its URI ( an image, or the DataTexture the format's loader makes ),
//! so the page knows which image each of the material's bindings samples.
use crate::reg::*;
use crate::rt::*;

/// One document: its text and the loader's path.
pub struct Document {
    pub text: String,
    pub path: String,
}

/// What a texture binding of the object group samples.
#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    /// A document's image: its resolved URI.
    Image(String),
    /// The DFG LUT.
    Dfg,
    /// The environment's PMREM.
    Pmrem,
    /// The transmission ( viewport mip ) texture.
    Transmission,
}

/// A texture binding of the object group: its binding ( the sampler's is
/// the one before ), what it samples and the texture's wrapS and wrapT.
#[derive(Clone, Debug, PartialEq)]
pub struct Binding {
    pub binding: u32,
    pub source: Source,
    pub wrap: (u32, u32),
}

/// A material's shaders, whether it is transparent and its textures; for a
/// transparent double-sided material ( transmission ), the front side's,
/// and its back side's vertex and fragment shaders in `back` ( the renderer
/// draws the back faces first ).
pub struct Shaders {
    pub vertex: String,
    pub fragment: String,
    pub transparent: bool,
    pub textures: Vec<Binding>,
    pub back: Option<(String, String)>,
}

/// The ground grid's shaders and each document's.
pub struct PageShaders {
    pub grid: Shaders,
    pub materials: Vec<Result<Shaders, String>>,
    /// Per document, its material's shaders on the UV plane ( the page's
    /// 'UV Plane' geometry, built when the planes show ).
    pub planes: Vec<Result<Shaders, String>>,
}

fn element(name: &str) -> Val {
    let o = object(&[
        ("nodeName", Val::from(name.to_uppercase())),
        ("style", object(&[])),
        ("width", Val::Num(300.0)),
        ("height", Val::Num(150.0)),
    ]);
    for f in ["getContext", "addEventListener", "removeEventListener", "setAttribute", "getAttribute"] {
        let _ = set(&o, f, func(f, 0, |_t, _a| Ok(Val::Null)));
    }
    o
}

/// The page's host objects inside the port: a hidden document whose canvases
/// have no context ( the renderer runs headless and renders no frames, so an
/// async compilation yields with `scheduler.yield()` ), and loaders whose
/// requests never settle.
fn install_host() {
    let doc = object(&[("hidden", Val::Bool(true))]);
    let _ = set(&doc, "createElementNS", func("createElementNS", 2, |_t, a| Ok(element(&display(&arg(a, 1))))));
    let _ = set(&doc, "createElement", func("createElement", 1, |_t, a| Ok(element(&display(&arg(a, 0))))));
    define_global("document", doc);
    let signal = object(&[("aborted", Val::Bool(false))]);
    let any = signal.clone();
    define_global(
        "AbortController",
        func_ctor("AbortController", 0, move |this, _a| {
            set(this, "signal", signal.clone())?;
            set(this, "abort", func("abort", 0, |_t, _a| Ok(Val::Undef)))?;
            Ok(Val::Undef)
        }),
    );
    define_global(
        "AbortSignal",
        object(&[("any", func("any", 1, move |_t, _a| Ok(any.clone())))]),
    );
    define_global(
        "fetch",
        func("fetch", 2, |_t, _a| construct(&global("Promise"), &[func("executor", 2, |_t, _a| Ok(Val::Undef))])),
    );
}

thread_local! {
    static READY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Evaluate the modules once ( three's as the page's build evaluates them,
/// then the addons ).
pub fn init() -> R<()> {
    if READY.with(|r| r.replace(true)) {
        return Ok(());
    }
    install_host();
    crate::init_three_bundled()?;
    crate::init_addons()
}

fn three() -> Val {
    module_namespace("Three.WebGPU.js")
}
fn tsl() -> Val {
    module_namespace("Three.TSL.js")
}
fn new(name: &str, args: &[Val]) -> R<Val> {
    construct(&get(&three(), name)?, args)
}
fn t(name: &str, args: &[Val]) -> R<Val> {
    call(&get(&tsl(), name)?, &Val::Undef, args)
}
fn num(v: f64) -> Val {
    Val::Num(v)
}

/// The page's ground material: the grid of lines and dots faded with the
/// distance, as its `grid` and `fade` Fns build it.
fn ground_material() -> R<Val> {
    let material = new("MeshBasicNodeMaterial", &[])?;
    // const grid = Fn( ( [ coord, lineWidth = float( 0.01 ), dotSize = float( 0.03 ) ] ) => … )
    let grid_body = func("", 1, |_this, args| {
        let inputs = arg(args, 0);
        let coord = get_index(&inputs, 0)?;
        let mut line_width = get_index(&inputs, 1)?;
        if line_width.is_undef() {
            line_width = t("float", &[num(0.01)])?;
        }
        let mut dot_size = get_index(&inputs, 2)?;
        if dot_size.is_undef() {
            dot_size = t("float", &[num(0.03)])?;
        }
        let g = t("fract", &[coord.clone()])?;
        let fw = t("fwidth", &[coord])?;
        let gx = t("abs", &[m(&get(&g, "x")?, "sub", &[num(0.5)])?])?;
        let gy = t("abs", &[m(&get(&g, "y")?, "sub", &[num(0.5)])?])?;
        let line_x = t("saturate", &[m(&m(&m(&line_width, "sub", &[gx.clone()])?, "div", &[get(&fw, "x")?])?, "add", &[num(0.5)])?])?;
        let line_y = t("saturate", &[m(&m(&m(&line_width, "sub", &[gy.clone()])?, "div", &[get(&fw, "y")?])?, "add", &[num(0.5)])?])?;
        let lines = t("max", &[line_x, line_y])?;
        let square_dist = t("max", &[gx, gy])?;
        let aa = t("max", &[get(&fw, "x")?, get(&fw, "y")?])?;
        let dots = t(
            "smoothstep",
            &[m(&dot_size, "add", &[aa.clone()])?, m(&dot_size, "sub", &[aa])?, square_dist],
        )?;
        t("max", &[dots, lines])
    });
    let grid = t("Fn", &[grid_body])?;
    // const fade = Fn( ( [ radius = float( 10.0 ), falloff = float( 1.0 ) ] ) => … )
    let fade_body = func("", 1, |_this, args| {
        let inputs = arg(args, 0);
        let mut radius = get_index(&inputs, 0)?;
        if radius.is_undef() {
            radius = t("float", &[num(10.0)])?;
        }
        let mut falloff = get_index(&inputs, 1)?;
        if falloff.is_undef() {
            falloff = t("float", &[num(1.0)])?;
        }
        t(
            "smoothstep",
            &[radius.clone(), m(&radius, "sub", &[falloff])?, t("length", &[get(&tsl(), "positionWorld")?])?],
        )
    });
    let fade = t("Fn", &[fade_body])?;
    let grid_color = t("vec4", &[num(0.5), num(0.5), num(0.5), num(1.0)])?;
    let base_color = t("vec4", &[num(1.0), num(1.0), num(1.0), num(0.0)])?;
    let xz = get(&get(&tsl(), "positionWorld")?, "xz")?;
    let lines = call(&grid, &Val::Undef, &[xz, num(0.007), num(0.03)])?;
    let faded = call(&fade, &Val::Undef, &[num(30.0), num(20.0)])?;
    let color = m(&m(&lines, "mix", &[base_color, grid_color])?, "mul", &[faded])?;
    set(&material, "colorNode", color)?;
    set(&material, "transparent", Val::Bool(true))?;
    Ok(material)
}

fn shaders_of(renderer: &Val, scene: &Val, camera: &Val, mesh: &Val, transparent: bool) -> R<Shaders> {
    let raw = await_val(m(&get(renderer, "debug")?, "getShaderAsync", &[scene.clone(), camera.clone(), mesh.clone()])?)?;
    Ok(Shaders {
        vertex: display(&get(&raw, "vertexShader")?),
        fragment: display(&get(&raw, "fragmentShader")?),
        transparent,
        textures: vec![],
        back: None,
    })
}

/// The node builder state of `mesh`'s render object for the pass `pass`
/// ( undefined: the default one ), found as `getShaderAsync` finds it.
fn builder_state(renderer: &Val, scene: &Val, camera: &Val, mesh: &Val, pass: Val) -> R<Val> {
    let frame_buffer = get(renderer, "needsFrameBufferTarget")?.truthy() && get(renderer, "_renderTarget")?.is_null();
    let render_target = if frame_buffer {
        m(renderer, "_getFrameBufferTarget", &[])?
    } else {
        let t = get(renderer, "_renderTarget")?;
        if t.truthy() { t } else { get(renderer, "_outputRenderTarget")? }
    };
    let render_list = m(&get(renderer, "_renderLists")?, "get", &[scene.clone(), camera.clone(), get(renderer, "lighting")?])?;
    let render_context = m(
        &get(renderer, "_renderContexts")?,
        "get",
        &[render_target, get(renderer, "_mrt")?, num(0.0), camera.clone()],
    )?;
    let render_object = m(
        &get(renderer, "_objects")?,
        "get",
        &[
            mesh.clone(),
            get(mesh, "material")?,
            scene.clone(),
            camera.clone(),
            get(&render_list, "lightsNode")?,
            render_context.clone(),
            get(&render_context, "clippingContext")?,
            pass,
        ],
    )?;
    m(&render_object, "getNodeBuilderState", &[])
}

/// A compiled material's shaders on `mesh`: a transparent double-sided
/// material's front-side pass, with its back-side pass ( `compileAsync`
/// builds both, as the renderer draws them ), otherwise its one pass.
fn compiled_shaders(renderer: &Val, scene: &Val, camera: &Val, mesh: &Val) -> R<Shaders> {
    let three = three();
    let material = get(mesh, "material")?;
    let transparent = get(&material, "transparent")?.truthy();
    let double_side = get(&three, "DoubleSide")?;
    let double = transparent && get(&material, "side")?.strict_eq(&double_side) && !get(&material, "forceSinglePass")?.truthy();
    let stages = |state: &Val| -> R<(String, String)> {
        Ok((display(&get(state, "vertexShader")?), display(&get(state, "fragmentShader")?)))
    };
    let mut back = None;
    if double {
        set(&material, "side", get(&three, "BackSide")?)?;
        let state = builder_state(renderer, scene, camera, mesh, s("backSide"));
        set(&material, "side", get(&three, "FrontSide")?)?;
        let state = state?;
        back = Some((stages(&state)?, textures(&state)?));
    }
    let state = builder_state(renderer, scene, camera, mesh, Val::Undef);
    if double {
        set(&material, "side", double_side)?;
    }
    let state = state?;
    let ((vertex, fragment), textures) = (stages(&state)?, textures(&state)?);
    let back = match back {
        Some((shaders, back_textures)) if back_textures == textures => Some(shaders),
        Some(_) => return Err(type_error("the back side's texture bindings differ")),
        None => None,
    };
    Ok(Shaders { vertex, fragment, transparent, textures, back })
}

/// The object group's texture bindings of a node builder state.
fn textures(state: &Val) -> R<Vec<Binding>> {
    let three = three();
    let groups = get(state, "bindings")?;
    let mut out = vec![];
    for g in 0..to_number(&get(&groups, "length")?) as usize {
        let group = get_index(&groups, g)?;
        if display(&get(&group, "name")?) != "object" {
            continue;
        }
        let bindings = get(&group, "bindings")?;
        for i in 0..to_number(&get(&bindings, "length")?) as usize {
            let binding = get_index(&bindings, i)?;
            if !get(&binding, "isSampledTexture")?.truthy() {
                continue;
            }
            let texture = get(&binding, "texture")?;
            let image = get(&texture, "image")?;
            let uri = if !image.is_nullish() { get(&image, "materialxURI")? } else { Val::Undef };
            let source = if get(&texture, "isFramebufferTexture")?.truthy() {
                Source::Transmission
            } else if get(&texture, "isPMREMTexture")?.truthy() {
                Source::Pmrem
            } else if !uri.is_undef() {
                Source::Image(display(&uri))
            } else if get(&texture, "isDataTexture")?.truthy() && get(&texture, "format")?.strict_eq(&get(&three, "RGFormat")?) {
                Source::Dfg
            } else {
                return Err(type_error(&format!("unrecognized texture binding {}", display(&get(&binding, "name")?))));
            };
            out.push(Binding {
                binding: i as u32,
                source,
                wrap: (to_number(&get(&texture, "wrapS")?) as u32, to_number(&get(&texture, "wrapT")?) as u32),
            });
        }
    }
    Ok(out)
}

/// What the page's EXRLoader, HDRLoader and TIFFLoader hand MaterialXLoader
/// for `uri` ( lower-cased in `lower` ): a DataTexture as DataTextureLoader
/// configures it from the loader's texture data, one texel here.
fn data_texture(lower: &str, uri: Val) -> R<Val> {
    let three = three();
    let tiff = lower.ends_with(".tif") || lower.ends_with(".tiff");
    let texture = if tiff {
        // TIFFLoader: RGBA8, flipY, linear with mipmaps.
        new("DataTexture", &[typed(TypedKind::Uint8, vec![0.0; 4]), num(1.0), num(1.0)])?
    } else {
        // EXRLoader and HDRLoader: half-float RGBA, linear, no mipmaps.
        let t = new(
            "DataTexture",
            &[typed(TypedKind::Uint16, vec![0.0; 4]), num(1.0), num(1.0), get(&three, "RGBAFormat")?, get(&three, "HalfFloatType")?],
        )?;
        set(&t, "colorSpace", get(&three, "LinearSRGBColorSpace")?)?;
        set(&t, "generateMipmaps", Val::Bool(false))?;
        t
    };
    set(&texture, "wrapS", get(&three, "ClampToEdgeWrapping")?)?;
    set(&texture, "wrapT", get(&three, "ClampToEdgeWrapping")?)?;
    set(&texture, "magFilter", get(&three, "LinearFilter")?)?;
    let min = if tiff { "LinearMipmapLinearFilter" } else { "LinearFilter" };
    set(&texture, "minFilter", get(&three, min)?)?;
    // EXRLoader keeps its rows bottom-first; HDRLoader and TIFFLoader flip.
    set(&texture, "flipY", Val::Bool(!lower.ends_with(".exr")))?;
    set(&texture, "needsUpdate", Val::Bool(true))?;
    set(&get(&texture, "image")?, "materialxURI", uri)?;
    Ok(texture)
}

/// The page's shaders for `documents` ( in its order ) on a `width` ×
/// `height` canvas.
pub fn page_shaders(documents: &[Document], width: f64, height: f64) -> R<PageShaders> {
    init()?;
    let three = three();
    let renderer = new("WebGPURenderer", &[object(&[("antialias", Val::Bool(true))])])?;
    m(&renderer, "setPixelRatio", &[num(1.0)])?;
    m(&renderer, "setSize", &[num(width), num(height)])?;
    set(&renderer, "toneMapping", get(&three, "LinearToneMapping")?)?;
    set(&renderer, "toneMappingExposure", num(0.5))?;
    let camera = new("PerspectiveCamera", &[num(45.0), num(width / height), num(0.25), num(200.0)])?;
    // camera.position.set( 10, 10, 20 ), toward the OrbitControls' target.
    m(&get(&camera, "position")?, "set", &[num(10.0), num(10.0), num(20.0)])?;
    m(&camera, "lookAt", &[num(0.0), num(0.0), num(0.0)])?;
    let scene = new("Scene", &[])?;
    set(&scene, "background", new("Color", &[num(0xffffff as f64)])?)?;
    let ground = new("Mesh", &[new("CircleGeometry", &[num(40.0)])?, ground_material()?])?;
    set(&get(&ground, "rotation")?, "x", num(-std::f64::consts::PI / 2.0))?;
    set(&ground, "renderOrder", num(-1.0))?;
    m(&scene, "add", &[ground.clone()])?;
    // HDRLoader's texture: half-float RGBA, equirectangular, linear, no mipmaps.
    let env = new(
        "DataTexture",
        &[
            typed(TypedKind::Uint16, vec![0.0; 64 * 32 * 4]),
            num(64.0),
            num(32.0),
            get(&three, "RGBAFormat")?,
            get(&three, "HalfFloatType")?,
        ],
    )?;
    set(&env, "mapping", get(&three, "EquirectangularReflectionMapping")?)?;
    set(&env, "colorSpace", get(&three, "LinearSRGBColorSpace")?)?;
    set(&env, "minFilter", get(&three, "LinearFilter")?)?;
    set(&env, "magFilter", get(&three, "LinearFilter")?)?;
    set(&env, "generateMipmaps", Val::Bool(false))?;
    set(&env, "flipY", Val::Bool(true))?;
    set(&env, "needsUpdate", Val::Bool(true))?;
    set(&scene, "environment", env)?;
    await_val(m(&renderer, "init", &[])?)?;
    let grid = shaders_of(&renderer, &scene, &camera, &ground, true)?;
    // computePrefabTangents leaves the ShaderBall's meshes non-indexed with
    // position, normal, uv and tangent.
    let geometry = new("BufferGeometry", &[])?;
    for (name, size) in [("position", 3.0), ("normal", 3.0), ("uv", 2.0), ("tangent", 4.0)] {
        let attribute = new(
            "BufferAttribute",
            &[typed(TypedKind::Float32, vec![0.0; 3 * size as usize]), num(size)],
        )?;
        m(&geometry, "setAttribute", &[s(name), attribute])?;
    }
    let loader_ns = module_namespace("../examples/jsm/loaders/MaterialXLoader.js");
    let validation = module_namespace("../examples/jsm/loaders/materialx/MaterialXInterfaceValidation.js");
    let validator = call(&get(&validation, "createStrictInterfaceValidator")?, &Val::Undef, &[])?;
    // The page's loaders for <image> files: TextureLoader's image, and the
    // DefaultLoadingManager handlers it adds for .exr, .tif( f ) and .hdr,
    // whose DataTextures MaterialXLoader clones. Each stand-in names the URI
    // on its image ( the texture's data is not loaded ).
    let handler = object(&[(
        "load",
        func("load", 4, |_t, a| {
            let uri = arg(a, 0);
            let lower = display(&uri).to_lowercase();
            let loaded = if [".exr", ".hdr", ".tif", ".tiff"].iter().any(|e| lower.ends_with(e)) {
                data_texture(&lower, uri)?
            } else {
                object(&[("width", num(1.0)), ("height", num(1.0)), ("materialxURI", uri)])
            };
            call(&arg(a, 1), &Val::Undef, &[loaded])?;
            Ok(Val::Undef)
        }),
    )]);
    let handler_pattern = construct(&global("RegExp"), &[s("")])?;
    let mut materials = vec![];
    let mut models = vec![];
    for d in documents {
        let one = || -> R<(Shaders, [Val; 3])> {
            let loader = construct(&get(&loader_ns, "MaterialXLoader")?, &[])?;
            m(&get(&loader, "manager")?, "addHandler", &[handler_pattern.clone(), handler.clone()])?;
            let loader = m(&loader, "setPath", &[s(&d.path)])?;
            let options = object(&[
                ("uvSpace", s("top-left")),
                ("interfaceValidator", validator.clone()),
                ("throwOnErrors", Val::Bool(false)),
            ]);
            let result = m(&loader, "parse", &[s(&d.text), options]);
            m(&get(&loader, "manager")?, "removeHandler", &[handler_pattern.clone()])?;
            // Object.values( materials ).pop()
            let all = get(&result?, "materials")?;
            let names = keys(&all);
            let material = match names.last() {
                Some(n) => get(&all, n)?,
                None => return Err(type_error("no material")),
            };
            let transparent = get(&material, "transparent")?.truthy();
            // The ShaderBall's clone: its calibration and preview meshes, and
            // the hidden UV plane, all with the material.
            let model = new("Group", &[])?;
            let calibration = new("Mesh", &[geometry.clone(), material.clone()])?;
            let preview = new("Mesh", &[geometry.clone(), material.clone()])?;
            let plane = new("Mesh", &[new("PlaneGeometry", &[num(2.0), num(2.0)])?, material])?;
            set(&get(&plane, "rotation")?, "x", num(-std::f64::consts::PI / 2.0))?;
            set(&get(&plane, "position")?, "y", num(0.05))?;
            set(&plane, "visible", Val::Bool(false))?;
            for mesh in [&calibration, &preview, &plane] {
                m(&model, "add", &[mesh.clone()])?;
            }
            if transparent {
                set(&calibration, "renderOrder", num(1.0))?;
                set(&preview, "renderOrder", num(2.0))?;
            }
            // await renderer.compileAsync( model, camera, scene ); scene.add( model );
            await_val(m(&renderer, "compileAsync", &[model.clone(), camera.clone(), scene.clone()])?)?;
            m(&scene, "add", &[model])?;
            Ok((compiled_shaders(&renderer, &scene, &camera, &calibration)?, [calibration, preview, plane]))
        };
        match one() {
            Ok((shaders, meshes)) => {
                materials.push(Ok(shaders));
                models.push(Some(meshes));
            }
            Err(e) => {
                materials.push(Err(error_message(&e.0)));
                models.push(None);
            }
        }
    }
    // The Geometry selector's 'UV Plane': the planes show instead of the
    // ShaderBall's meshes, and the next frame builds their render objects.
    let mut planes = vec![];
    for meshes in &models {
        let plane = || -> R<Shaders> {
            let [calibration, preview, plane] = meshes.as_ref().ok_or_else(|| type_error("no material"))?;
            set(calibration, "visible", Val::Bool(false))?;
            set(preview, "visible", Val::Bool(false))?;
            set(plane, "visible", Val::Bool(true))?;
            compiled_shaders(&renderer, &scene, &camera, plane)
        };
        planes.push(plane().map_err(|e| error_message(&e.0)));
    }
    Ok(PageShaders { grid, materials, planes })
}
