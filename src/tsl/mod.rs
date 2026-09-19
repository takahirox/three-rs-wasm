//! Rust expression nodes following the Three.js Shading Language model.
//!
//! Graphs are validated and compiled once to WGSL; uniforms change without
//! rebuilding the graph. This is a subset, not a JavaScript TSL interpreter.
//! See `docs/tsl.md` for supported nodes and limits.
use crate::{
    Error, Result, material::ShaderMaterial, postprocessing::Effect, renderer::Renderer,
    shader::ShaderProgram,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Bool,
    Texture,
    Sampler,
}
impl Type {
    fn lanes(self) -> usize {
        match self {
            Self::Float => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 => 4,
            _ => 0,
        }
    }
    fn vector(n: usize) -> Result<Self> {
        match n {
            1 => Ok(Self::Float),
            2 => Ok(Self::Vec2),
            3 => Ok(Self::Vec3),
            4 => Ok(Self::Vec4),
            _ => Err(Error::Invalid("TSL vector width")),
        }
    }
    fn wgsl(self) -> &'static str {
        match self {
            Self::Float => "f32",
            Self::Vec2 => "vec2<f32>",
            Self::Vec3 => "vec3<f32>",
            Self::Vec4 => "vec4<f32>",
            Self::Bool => "bool",
            Self::Texture => "texture_2d<f32>",
            Self::Sampler => "sampler",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Texture {
    Map,
    Input,
    External(usize),
}
impl Texture {
    pub fn node(self) -> Node {
        Node::new(Expr::Resource(self, false))
    }
    pub fn sampler(self) -> Node {
        Node::new(Expr::Resource(self, true))
    }
    pub fn sample(self, uv: Node) -> Node {
        Node::new(Expr::Sample(self, vec![uv]))
    }
    pub fn sample_grad(self, uv: Node, dx: Node, dy: Node) -> Node {
        Node::new(Expr::Sample(self, vec![uv, dx, dy]))
    }
    pub fn sample_level(self, uv: Node, level: Node) -> Node {
        Node::new(Expr::Sample(self, vec![uv, level]))
    }
}
/// A native WGSL function. Signature arguments are checked by the graph compiler;
/// the device validates the function body before creating a material/effect.
#[derive(Clone, Debug)]
pub struct WgslFn {
    name: String,
    source: String,
    arguments: Vec<Type>,
    result: Type,
}
impl WgslFn {
    pub fn new(name: &str, source: &str, arguments: &[Type], result: Type) -> Result<Self> {
        if name.is_empty()
            || name.starts_with("__")
            || !name
                .chars()
                .enumerate()
                .all(|(i, c)| c.is_ascii_alphabetic() || c == '_' || (i > 0 && c.is_ascii_digit()))
            || result.lanes() == 0 && result != Type::Bool
        {
            return Err(Error::Invalid("TSL WGSL function signature"));
        }
        Ok(Self {
            name: name.into(),
            source: source.into(),
            arguments: arguments.into(),
            result,
        })
    }
    pub fn call(&self, arguments: &[Node]) -> Node {
        Node::new(Expr::Call(Arc::new(self.clone()), arguments.to_vec()))
    }
}
#[derive(Clone, Debug)]
pub struct Node(Arc<Expr>);
#[derive(Debug)]
enum Expr {
    Constant(f32),
    Uniform(usize, Type),
    Uv,
    Position,
    Normal,
    Vector(Type, Vec<Node>),
    Swizzle(Node, String),
    Binary(&'static str, Node, Node),
    Unary(&'static str, Node),
    Clamp(Node, Node, Node),
    Select(Node, Node, Node),
    Resource(Texture, bool),
    Sample(Texture, Vec<Node>),
    Call(Arc<WgslFn>, Vec<Node>),
}
pub fn float(value: f32) -> Node {
    Node::new(Expr::Constant(value))
}
pub fn uv() -> Node {
    Node::new(Expr::Uv)
}
pub fn position_geometry() -> Node {
    Node::new(Expr::Position)
}
pub fn normal_geometry() -> Node {
    Node::new(Expr::Normal)
}
/// Index refers to a vec4 slot in ShaderMaterial.uniforms / Effect.parameters.
/// The compiler rejects indices >=16 and nonnumeric types.
pub fn uniform(index: usize, ty: Type) -> Node {
    Node::new(Expr::Uniform(index, ty))
}
pub fn vec2(x: Node, y: Node) -> Node {
    Node::new(Expr::Vector(Type::Vec2, vec![x, y]))
}
pub fn vec3(x: Node, y: Node, z: Node) -> Node {
    Node::new(Expr::Vector(Type::Vec3, vec![x, y, z]))
}
pub fn vec4(rgb: Node, alpha: Node) -> Node {
    Node::new(Expr::Vector(Type::Vec4, vec![rgb, alpha]))
}
pub fn splat(value: Node, ty: Type) -> Node {
    Node::new(Expr::Vector(ty, vec![value]))
}
/// Fn-style composition is a Rust closure returning a node. Shared return values
/// retain their identity and are emitted only once per shader stage.
pub fn function(build: impl FnOnce() -> Node) -> Node {
    build()
}
pub fn checker(coordinate: Node) -> Node {
    let p = (coordinate * float(2.0)).floor();
    // Three.js Checker: a 2x2 pattern per unit coordinate.
    (p.x() + p.y()).modulo(float(2.0)).sign()
}
impl Node {
    fn new(expr: Expr) -> Self {
        Self(Arc::new(expr))
    }
    pub fn swizzle(&self, components: &str) -> Self {
        Self::new(Expr::Swizzle(self.clone(), components.into()))
    }
    pub fn x(&self) -> Self {
        self.swizzle("x")
    }
    pub fn y(&self) -> Self {
        self.swizzle("y")
    }
    pub fn rgb(&self) -> Self {
        self.swizzle("xyz")
    }
    fn unary(&self, op: &'static str) -> Self {
        Self::new(Expr::Unary(op, self.clone()))
    }
    fn binary(&self, op: &'static str, rhs: Node) -> Self {
        Self::new(Expr::Binary(op, self.clone(), rhs))
    }
    pub fn sin(&self) -> Self {
        self.unary("sin")
    }
    pub fn cos(&self) -> Self {
        self.unary("cos")
    }
    pub fn floor(&self) -> Self {
        self.unary("floor")
    }
    pub fn fract(&self) -> Self {
        self.unary("fract")
    }
    pub fn sign(&self) -> Self {
        self.unary("sign")
    }
    pub fn abs(&self) -> Self {
        self.unary("abs")
    }
    pub fn length(&self) -> Self {
        self.unary("length")
    }
    pub fn pow(&self, rhs: Node) -> Self {
        self.binary("pow", rhs)
    }
    /// TSL mod is floor-based, unlike WGSL's remainder for negative inputs.
    pub fn modulo(&self, rhs: Node) -> Self {
        self.binary("mod", rhs)
    }
    pub fn equal(&self, rhs: Node) -> Self {
        self.binary("==", rhs)
    }
    pub fn greater_than(&self, rhs: Node) -> Self {
        self.binary(">", rhs)
    }
    pub fn less_than(&self, rhs: Node) -> Self {
        self.binary("<", rhs)
    }
    pub fn and(&self, rhs: Node) -> Self {
        self.binary("&&", rhs)
    }
    pub fn clamp(&self, low: Node, high: Node) -> Self {
        Self::new(Expr::Clamp(self.clone(), low, high))
    }
    /// Select evaluates both branches, keeping derivative texture samples uniform.
    pub fn select(&self, when_true: Node, when_false: Node) -> Self {
        Self::new(Expr::Select(self.clone(), when_true, when_false))
    }
    pub fn to_float(&self) -> Self {
        self.unary("f32")
    }
}
macro_rules! operator {
    ($trait:ident,$method:ident,$op:literal) => {
        impl std::ops::$trait for Node {
            type Output = Node;
            fn $method(self, rhs: Node) -> Node {
                self.binary($op, rhs)
            }
        }
    };
}
operator!(Add, add, "+");
operator!(Sub, sub, "-");
operator!(Mul, mul, "*");
operator!(Div, div, "/");
impl std::ops::Neg for Node {
    type Output = Node;
    fn neg(self) -> Node {
        self.unary("negate")
    }
}

/// Unlit node material; position is evaluated on the GPU before the model transform.
#[derive(Clone, Debug)]
pub struct NodeMaterial {
    pub color: Node,
    pub position: Option<Node>,
}
impl NodeMaterial {
    pub fn new(color: Node) -> Self {
        Self {
            color,
            position: None,
        }
    }
    pub fn wgsl(&self, texture_count: usize) -> Result<String> {
        let mut vertex = Compiler::new(Stage::Vertex, texture_count);
        let (ty, position) = vertex.emit(self.position.as_ref().unwrap_or(&position_geometry()))?;
        if ty != Type::Vec3 {
            return Err(Error::Invalid("TSL position must be vec3"));
        }
        let mut fragment = Compiler::new(Stage::Fragment, texture_count);
        let (ty, color) = fragment.emit(&self.color)?;
        let color = output_color(ty, color)?;
        let mut functions = vertex.functions;
        for (name, source) in fragment.functions {
            insert_function(&mut functions, name, source)?;
        }
        let mut names: Vec<_> = functions.keys().collect();
        names.sort();
        let functions = names
            .into_iter()
            .map(|key| functions[key].as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let resources = (0..texture_count).map(|i| format!("@group(1) @binding({}) var tsl_texture_{i}:texture_2d<f32>;\n@group(1) @binding({}) var tsl_sampler_{i}:sampler;",i*2,i*2+1)).collect::<Vec<_>>().join("\n");
        Ok(format!(
            "{resources}\n{functions}\nfn deform(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{{\n{}return {position};\n}}\nfn shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>{{\n{}return {color};\n}}",
            vertex.body, fragment.body
        ))
    }
    pub async fn build(
        &self,
        renderer: &Renderer,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<ShaderMaterial> {
        let source = self.wgsl(textures.len())?;
        Ok(ShaderMaterial::new(Arc::new(
            ShaderProgram::with_textures(renderer, &source, &[], textures).await?,
        )))
    }
}
/// Compile the same expression API for a fullscreen GPU pass.
pub fn effect_wgsl(color: &Node) -> Result<String> {
    let mut compiler = Compiler::new(Stage::Effect, 0);
    let (ty, value) = compiler.emit(color)?;
    let value = output_color(ty, value)?;
    let mut names: Vec<_> = compiler.functions.keys().collect();
    names.sort();
    let functions = names
        .into_iter()
        .map(|key| compiler.functions[key].as_str())
        .collect::<Vec<_>>()
        .join("\n");
    Ok(format!(
        "{functions}\nfn effect(uv:vec2<f32>)->vec4<f32>{{\n{}return {value};\n}}",
        compiler.body
    ))
}
pub async fn effect(
    renderer: &Renderer,
    format: wgpu::TextureFormat,
    color: &Node,
) -> Result<Effect> {
    Effect::new(renderer, format, &effect_wgsl(color)?).await
}
fn output_color(ty: Type, value: String) -> Result<String> {
    match ty {
        Type::Float => Ok(format!("vec4(vec3({value}),1.0)")),
        Type::Vec3 => Ok(format!("vec4({value},1.0)")),
        Type::Vec4 => Ok(value),
        _ => Err(Error::Invalid("TSL color must be float, vec3 or vec4")),
    }
}
fn insert_function(
    functions: &mut HashMap<String, String>,
    name: String,
    source: String,
) -> Result<()> {
    if let Some(previous) = functions.get(&name)
        && previous != &source
    {
        return Err(Error::Invalid("conflicting TSL WGSL functions"));
    }
    functions.insert(name, source);
    Ok(())
}
#[derive(Clone, Copy, PartialEq)]
enum Stage {
    Vertex,
    Fragment,
    Effect,
}
struct Compiler {
    stage: Stage,
    textures: usize,
    body: String,
    values: HashMap<usize, (Type, String)>,
    functions: HashMap<String, String>,
}
impl Compiler {
    fn new(stage: Stage, textures: usize) -> Self {
        Self {
            stage,
            textures,
            body: String::new(),
            values: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    fn texture(&self, texture: Texture) -> Result<(String, String)> {
        match texture {
            Texture::Map if self.stage != Stage::Effect => {
                Ok(("color_map".into(), "color_sampler".into()))
            }
            Texture::Input if self.stage == Stage::Effect => {
                Ok(("input_texture".into(), "input_sampler".into()))
            }
            Texture::External(i) if self.stage != Stage::Effect && i < self.textures => {
                Ok((format!("tsl_texture_{i}"), format!("tsl_sampler_{i}")))
            }
            _ => Err(Error::Invalid("TSL texture binding for this stage")),
        }
    }
    fn emit(&mut self, node: &Node) -> Result<(Type, String)> {
        let key = Arc::as_ptr(&node.0) as usize;
        if let Some(value) = self.values.get(&key) {
            return Ok(value.clone());
        }
        let (ty, expression) = match &*node.0 {
            Expr::Constant(x) => {
                if !x.is_finite() {
                    return Err(Error::Invalid("nonfinite TSL constant"));
                }
                (Type::Float, format!("{x:?}"))
            }
            Expr::Uniform(i, ty) => {
                if *i >= 16 || ty.lanes() == 0 {
                    return Err(Error::Invalid("TSL uniform slot or type"));
                }
                let prefix = if self.stage == Stage::Effect {
                    "params"
                } else {
                    "u.custom"
                };
                (*ty, format!("{prefix}[{i}].{}", &"xyzw"[..ty.lanes()]))
            }
            Expr::Uv => (
                Type::Vec2,
                if self.stage == Stage::Fragment {
                    "surface.uv"
                } else {
                    "uv"
                }
                .into(),
            ),
            Expr::Position | Expr::Normal => {
                if self.stage != Stage::Vertex {
                    return Err(Error::Invalid(
                        "TSL geometry attribute requires vertex stage",
                    ));
                }
                (
                    Type::Vec3,
                    if matches!(&*node.0, Expr::Position) {
                        "position"
                    } else {
                        "normal"
                    }
                    .into(),
                )
            }
            Expr::Vector(ty, args) => {
                if ty.lanes() < 2 {
                    return Err(Error::Invalid("TSL vector constructor type"));
                }
                let mut values = Vec::new();
                let mut lanes = 0;
                for arg in args {
                    let (t, v) = self.emit(arg)?;
                    if t.lanes() == 0 {
                        return Err(Error::Invalid("TSL vector argument type"));
                    }
                    lanes += t.lanes();
                    values.push(v);
                }
                if lanes != ty.lanes() && !(args.len() == 1 && lanes == 1) {
                    return Err(Error::Invalid("TSL vector constructor width"));
                }
                (*ty, format!("{}({})", ty.wgsl(), values.join(",")))
            }
            Expr::Swizzle(input, components) => {
                let (t, v) = self.emit(input)?;
                if t.lanes() < 2
                    || components.is_empty()
                    || components.len() > 4
                    || !components
                        .chars()
                        .all(|c| "xyzw".find(c).is_some_and(|i| i < t.lanes()))
                {
                    return Err(Error::Invalid("TSL swizzle"));
                }
                (Type::vector(components.len())?, format!("{v}.{components}"))
            }
            Expr::Unary(op, input) => {
                let (t, v) = self.emit(input)?;
                if *op == "f32" {
                    if ![Type::Float, Type::Bool].contains(&t) {
                        return Err(Error::Invalid("TSL float conversion"));
                    }
                    (
                        Type::Float,
                        if t == Type::Bool {
                            format!("select(0.0,1.0,{v})")
                        } else {
                            v
                        },
                    )
                } else {
                    if t.lanes() == 0 {
                        return Err(Error::Invalid("TSL numeric unary operand"));
                    }
                    (
                        if *op == "length" { Type::Float } else { t },
                        if *op == "negate" {
                            format!("(-{v})")
                        } else {
                            format!("{op}({v})")
                        },
                    )
                }
            }
            Expr::Binary(op, a, b) => {
                let a = self.emit(a)?;
                let b = self.emit(b)?;
                if *op == "&&" {
                    if a.0 != Type::Bool || b.0 != Type::Bool {
                        return Err(Error::Invalid("TSL boolean operands"));
                    }
                    (Type::Bool, format!("({} && {})", a.1, b.1))
                } else if ["==", ">", "<"].contains(op) {
                    if a.0 != Type::Float || b.0 != Type::Float {
                        return Err(Error::Invalid("TSL comparison requires scalars"));
                    }
                    (Type::Bool, format!("({} {op} {})", a.1, b.1))
                } else {
                    let (t, a, b) = promote(a, b)?;
                    (
                        t,
                        match *op {
                            "pow" => format!("pow({a},{b})"),
                            "mod" => format!("({a}-{b}*floor({a}/{b}))"),
                            _ => format!("({a} {op} {b})"),
                        },
                    )
                }
            }
            Expr::Clamp(a, low, high) => {
                let a = self.emit(a)?;
                let low = self.emit(low)?;
                let high = self.emit(high)?;
                let original = a.0;
                let (t, a, low) = promote(a, low)?;
                let (t, a, high) = promote((t, a), high)?;
                if t != original {
                    return Err(Error::Invalid("TSL clamp bounds wider than value"));
                }
                (t, format!("clamp({a},{low},{high})"))
            }
            Expr::Select(condition, a, b) => {
                let c = self.emit(condition)?;
                if c.0 != Type::Bool {
                    return Err(Error::Invalid("TSL select condition"));
                }
                let a = self.emit(a)?;
                let b = self.emit(b)?;
                let (t, a, b) = promote(a, b)?;
                (t, format!("select({b},{a},{})", c.1))
            }
            Expr::Resource(texture, sampler) => {
                let (t, s) = self.texture(*texture)?;
                if *sampler {
                    (Type::Sampler, s)
                } else {
                    (Type::Texture, t)
                }
            }
            Expr::Sample(texture, args) => {
                let (t, s) = self.texture(*texture)?;
                if self.stage == Stage::Vertex && args.len() != 2 {
                    return Err(Error::Invalid("TSL vertex texture requires explicit level"));
                }
                let mut values = Vec::new();
                for (i, arg) in args.iter().enumerate() {
                    let (ty, v) = self.emit(arg)?;
                    let expected = if args.len() == 2 && i == 1 {
                        Type::Float
                    } else {
                        Type::Vec2
                    };
                    if ty != expected {
                        return Err(Error::Invalid("TSL texture coordinate, level or gradient"));
                    }
                    values.push(v);
                }
                let op = match args.len() {
                    1 => "textureSample",
                    2 => "textureSampleLevel",
                    _ => "textureSampleGrad",
                };
                (Type::Vec4, format!("{op}({t},{s},{})", values.join(",")))
            }
            Expr::Call(function, args) => {
                if function.arguments.len() != args.len() {
                    return Err(Error::Invalid("TSL WGSL function arity"));
                }
                let mut values = Vec::new();
                for (arg, expected) in args.iter().zip(&function.arguments) {
                    let (t, v) = self.emit(arg)?;
                    if t != *expected {
                        return Err(Error::Invalid("TSL WGSL function argument type"));
                    }
                    values.push(v);
                }
                insert_function(
                    &mut self.functions,
                    function.name.clone(),
                    function.source.clone(),
                )?;
                (
                    function.result,
                    format!("{}({})", function.name, values.join(",")),
                )
            }
        };
        // Resource handles cannot be assigned to WGSL let declarations.
        let value = if matches!(ty, Type::Texture | Type::Sampler) {
            expression
        } else {
            let name = format!("tsl_n{}", self.values.len());
            self.body
                .push_str(&format!("let {name}:{} = {expression};\n", ty.wgsl()));
            name
        };
        self.values.insert(key, (ty, value.clone()));
        Ok((ty, value))
    }
}
fn promote(a: (Type, String), b: (Type, String)) -> Result<(Type, String, String)> {
    let (ta, a) = a;
    let (tb, b) = b;
    if ta.lanes() == 0 || tb.lanes() == 0 || (ta != tb && ta != Type::Float && tb != Type::Float) {
        return Err(Error::Invalid("TSL incompatible numeric operands"));
    }
    let ty = if ta.lanes() >= tb.lanes() { ta } else { tb };
    Ok((
        ty,
        if ta != ty {
            format!("{}({a})", ty.wgsl())
        } else {
            a
        },
        if tb != ty {
            format!("{}({b})", ty.wgsl())
        } else {
            b
        },
    ))
}

/// Separable GaussianBlurNode kernel (Three.js r186, MIT). Build once for each
/// direction; `step` is direction * blur amount / input resolution.
pub fn gaussian_blur(texture: Texture, coordinate: Node, step: Node, sigma: u32) -> Result<Node> {
    if sigma > 32 {
        return Err(Error::Invalid("TSL Gaussian sigma exceeds 32"));
    }
    let radius = 3 + 2 * sigma;
    let width = radius as f64 / 3.0;
    let weights: Vec<f64> = (0..radius)
        .map(|i| (-0.5 * (i * i) as f64 / (width * width)).exp())
        .collect();
    let sum = 1.0 + 2.0 * weights.iter().skip(1).sum::<f64>();
    let mut color = texture.sample(coordinate.clone()) * float((weights[0] / sum) as f32);
    for (i, weight) in weights.into_iter().enumerate().skip(1) {
        let offset = step.clone() * float(i as f32);
        color = color
            + (texture.sample(coordinate.clone() + offset.clone())
                + texture.sample(coordinate.clone() - offset))
                * float((weight / sum) as f32);
    }
    Ok(color)
}
