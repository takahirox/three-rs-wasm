//! webgpu_loader_materialx's shaders, built as the page builds them: its
//! scene ( the white background, the san_giuseppe_bridge environment, the
//! LinearToneMapping renderer at exposure 0.5 ), the TSL ground grid, and
//! each MaterialX document through the transpiled MaterialXLoader with the
//! page's options, its material on the ShaderBall's non-indexed meshes. The
//! renderer runs headless; the WGSL is `renderer.debug.getShaderAsync`'s.
//! Images are not loaded ( the shaders do not depend on them ): a loading
//! manager handler stands for the document's texture loader and hands each
//! texture a placeholder image naming its URI, so the page knows which
//! image each of the material's texture bindings samples.
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

/// A material's shaders, whether it is transparent and its textures.
pub struct Shaders {
    pub vertex: String,
    pub fragment: String,
    pub transparent: bool,
    pub textures: Vec<Binding>,
}

/// The ground grid's shaders and each document's.
pub struct PageShaders {
    pub grid: Shaders,
    pub materials: Vec<Result<Shaders, String>>,
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

/// The page's host objects inside the port: a document whose canvases have
/// no context ( the renderer runs headless ), and loaders whose requests
/// never settle.
fn install_host() {
    let doc = object(&[]);
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

/// Evaluate the modules once ( three's, then the addons ).
pub fn init() -> R<()> {
    if READY.with(|r| r.replace(true)) {
        return Ok(());
    }
    install_host();
    crate::init_three()?;
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
    })
}

/// The object group's texture bindings of `mesh`'s render object, found as
/// `getShaderAsync` finds it.
fn textures(renderer: &Val, scene: &Val, camera: &Val, mesh: &Val) -> R<Vec<Binding>> {
    let three = three();
    let frame_buffer = get(renderer, "needsFrameBufferTarget")?.truthy() && get(renderer, "_renderTarget")?.is_null();
    let render_target = if frame_buffer {
        m(renderer, "_getFrameBufferTarget", &[])?
    } else {
        let t = get(renderer, "_renderTarget")?;
        if t.truthy() { t } else { get(renderer, "_outputRenderTarget")? }
    };
    let render_list = m(&get(renderer, "_renderLists")?, "get", &[scene.clone(), camera.clone(), get(renderer, "lighting")?])?;
    let render_context = m(&get(renderer, "_renderContexts")?, "get", &[render_target, get(renderer, "_mrt")?])?;
    let material = get(mesh, "material")?;
    let render_object = m(
        &get(renderer, "_objects")?,
        "get",
        &[
            mesh.clone(),
            material,
            scene.clone(),
            camera.clone(),
            get(&render_list, "lightsNode")?,
            render_context.clone(),
            get(&render_context, "clippingContext")?,
        ],
    )?;
    let state = m(&render_object, "getNodeBuilderState", &[])?;
    let groups = get(&state, "bindings")?;
    let cube_uv = get(&three, "CubeUVReflectionMapping")?;
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
            } else if get(&texture, "mapping")?.strict_eq(&cube_uv) {
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
    // The texture loader's stand-in: the image's URI on a placeholder.
    let handler = object(&[(
        "load",
        func("load", 4, |_t, a| {
            let image = object(&[("width", num(1.0)), ("height", num(1.0)), ("materialxURI", arg(a, 0))]);
            call(&arg(a, 1), &Val::Undef, &[image])?;
            Ok(Val::Undef)
        }),
    )]);
    let handler_pattern = construct(&global("RegExp"), &[s("")])?;
    let mut materials = vec![];
    for d in documents {
        let one = || -> R<Shaders> {
            let loader = construct(&get(&loader_ns, "MaterialXLoader")?, &[])?;
            m(&get(&loader, "manager")?, "addHandler", &[handler_pattern.clone(), handler.clone()])?;
            let loader = m(&loader, "setPath", &[s(&d.path)])?;
            let options = object(&[
                ("uvSpace", s("top-left")),
                ("interfaceValidator", validator.clone()),
                ("throwOnErrors", Val::Bool(false)),
            ]);
            let result = m(&loader, "parse", &[s(&d.text), options])?;
            // Object.values( materials ).pop()
            let all = get(&result, "materials")?;
            let names = keys(&all);
            let material = match names.last() {
                Some(n) => get(&all, n)?,
                None => return Err(type_error("no material")),
            };
            let transparent = get(&material, "transparent")?.truthy();
            let mesh = new("Mesh", &[geometry.clone(), material])?;
            m(&scene, "add", &[mesh.clone()])?;
            let shaders = shaders_of(&renderer, &scene, &camera, &mesh, transparent)
                .and_then(|mut sh| {
                    sh.textures = textures(&renderer, &scene, &camera, &mesh)?;
                    Ok(sh)
                });
            m(&scene, "remove", &[mesh])?;
            m(&get(&loader, "manager")?, "removeHandler", &[handler_pattern.clone()])?;
            shaders
        };
        materials.push(one().map_err(|e| error_message(&e.0)));
    }
    Ok(PageShaders { grid, materials })
}
