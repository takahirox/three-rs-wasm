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
    Uint,
    UVec2,
    Float,
    Vec2,
    Vec3,
    Vec4,
    Bool,
    Texture,
    TextureArray,
    Texture3D,
    TextureCube,
    DepthTexture,
    ComparisonSampler,
    Sampler,
}
impl Type {
    fn lanes(self) -> usize {
        match self {
            Self::Float | Self::Uint => 1,
            Self::Vec2 | Self::UVec2 => 2,
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
            Self::Uint => "u32",
            Self::UVec2 => "vec2<u32>",
            Self::Float => "f32",
            Self::Vec2 => "vec2<f32>",
            Self::Vec3 => "vec3<f32>",
            Self::Vec4 => "vec4<f32>",
            Self::Bool => "bool",
            Self::Texture => "texture_2d<f32>",
            Self::TextureArray => "texture_2d_array<f32>",
            Self::Texture3D => "texture_3d<f32>",
            Self::TextureCube => "texture_cube<f32>",
            Self::DepthTexture => "texture_depth_2d",
            Self::ComparisonSampler => "sampler_comparison",
            Self::Sampler => "sampler",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Texture {
    Map,
    Input,
    History,
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
    /// Sample one resident 2D-array layer. Layer conversion truncates towards zero.
    pub fn sample_array(self, uv: Node, layer: Node) -> Node {
        Node::new(Expr::SampleArray(self, uv, layer))
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
    Uint(u32),
    InstanceIndex,
    VertexIndex,
    StorageElement(usize, Node),
    PositionLocal,
    NormalWorld,
    SurfaceVector(&'static str),
    PositionWorld,
    ViewZ,
    Output,
    BaseColor,
    LitProperty(&'static str, Type),
    ScreenCoordinate,
    ScreenSize,
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
    SampleArray(Texture, Node, Node),
    StorageLoad(usize, Node),
    DepthSample(Node),
    Call(Arc<WgslFn>, Vec<Node>),
}
/// Current render target dimensions in physical pixels.
pub fn screen_size() -> Node {
    Node::new(Expr::ScreenSize)
}
/// Fragment pixel coordinates, with top-left origin.
pub fn screen_coordinate() -> Node {
    Node::new(Expr::ScreenCoordinate)
}
/// Linear material output, available in output programs and MRT graphs.
pub fn output() -> Node {
    Node::new(Expr::Output)
}
pub async fn output_program(renderer: &Renderer, color: &Node) -> Result<ShaderProgram> {
    let mut compiler = Compiler::new(Stage::Output, 0);
    let (ty, value) = compiler.emit(color)?;
    let value = output_color(ty, value)?;
    let mut functions: Vec<_> = compiler.functions.values().cloned().collect();
    functions.sort();
    ShaderProgram::with_output(
        renderer,
        &format!(
            "{}\nfn transform_output(value:vec4<f32>)->vec4<f32>{{\n{}return {value};\n}}",
            functions.join("\n"),
            compiler.body
        ),
    )
    .await
}
/// Read a compute storage texture using signed texel coordinates (vec2).
pub fn texture_load(binding: usize, coordinate: Node) -> Node {
    Node::new(Expr::StorageLoad(binding, coordinate))
}
pub fn uint(value: u32) -> Node {
    Node::new(Expr::Uint(value))
}
/// Per-instance resident vec4 storage buffer, indexed on the GPU.
pub fn instanced_attribute(index: usize) -> Node {
    storage_element(index, instance_index())
}
/// Read a typed resident storage element. Its type comes from the binding layout.
pub fn storage_element(binding: usize, index: Node) -> Node {
    Node::new(Expr::StorageElement(binding, index))
}
pub fn position_local() -> Node {
    Node::new(Expr::PositionLocal)
}
pub fn tangent_view() -> Node {
    Node::new(Expr::SurfaceVector("tangent.xyz"))
}
pub fn bitangent_view() -> Node {
    Node::new(Expr::SurfaceVector("bitangent"))
}
pub fn position_view_direction() -> Node {
    Node::new(Expr::SurfaceVector("view_position"))
}
pub fn normal_world() -> Node {
    Node::new(Expr::NormalWorld)
}
/// Resolved material normal, available after lighting in output/MRT graphs.
pub fn normal_view() -> Node {
    Node::new(Expr::LitProperty("fragment_normal", Type::Vec3))
}
pub fn diffuse_color() -> Node {
    Node::new(Expr::LitProperty("fragment_diffuse", Type::Vec4))
}
pub fn emissive() -> Node {
    Node::new(Expr::LitProperty("fragment_emissive", Type::Vec3))
}
pub fn position_world() -> Node {
    Node::new(Expr::PositionWorld)
}
pub fn view_z() -> Node {
    Node::new(Expr::ViewZ)
}
/// Index of the current vertex, including indexed draws. Vertex stage only.
pub fn vertex_index() -> Node {
    Node::new(Expr::VertexIndex)
}
pub fn instance_index() -> Node {
    Node::new(Expr::InstanceIndex)
}
pub fn uvec2(x: Node, y: Node) -> Node {
    Node::new(Expr::Vector(Type::UVec2, vec![x, y]))
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
    pub fn normalize(&self) -> Self {
        self.unary("normalize")
    }
    pub fn fwidth(&self) -> Self {
        self.unary("fwidth")
    }
    pub fn exp(&self) -> Self {
        self.unary("exp")
    }
    pub fn sqrt(&self) -> Self {
        self.unary("sqrt")
    }
    pub fn dot(&self, rhs: Node) -> Self {
        self.binary("dot", rhs)
    }
    pub fn cross(&self, rhs: Node) -> Self {
        self.binary("cross", rhs)
    }
    pub fn max(&self, rhs: Node) -> Self {
        self.binary("max", rhs)
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
    /// Float mod is floor-based; unsigned mod uses integer remainder.
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
    /// Smooth Hermite interpolation between two edges.
    pub fn smoothstep(&self, low: Node, high: Node) -> Self {
        let t = ((self.clone() - low.clone()) / (high - low)).clamp(float(0.0), float(1.0));
        t.clone() * t.clone() * (float(3.0) - float(2.0) * t)
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
        self.wgsl_with_buffers(texture_count, 0)
    }
    pub fn wgsl_with_buffers(&self, texture_count: usize, buffer_count: usize) -> Result<String> {
        self.wgsl_with_storage(texture_count, &vec![Type::Vec4; buffer_count])
    }
    pub fn wgsl_with_storage(&self, texture_count: usize, types: &[Type]) -> Result<String> {
        self.wgsl_with_texture_types(&vec![Type::Texture; texture_count], types)
    }
    pub fn wgsl_with_texture_types(
        &self,
        texture_types: &[Type],
        types: &[Type],
    ) -> Result<String> {
        if texture_types.iter().any(|t| {
            !matches!(
                t,
                Type::Texture
                    | Type::TextureArray
                    | Type::Texture3D
                    | Type::TextureCube
                    | Type::DepthTexture
            )
        }) {
            return Err(Error::Invalid("TSL sampled texture type"));
        }
        let texture_count = texture_types.len();
        validate_storage_types(types)?;
        let buffer_count = types.len();
        let mut vertex = Compiler::new(Stage::Vertex, texture_count);
        vertex.buffers = types.to_vec();
        vertex.texture_types = texture_types.to_vec();
        let (ty, position) = vertex.emit(self.position.as_ref().unwrap_or(&position_geometry()))?;
        if ty != Type::Vec3 {
            return Err(Error::Invalid("TSL position must be vec3"));
        }
        let mut fragment = Compiler::new(Stage::Fragment, texture_count);
        fragment.buffers = types.to_vec();
        fragment.texture_types = texture_types.to_vec();
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
        let resources = (0..texture_count).map(|i| format!("@group(1) @binding({}) var tsl_texture_{i}:{};\n@group(1) @binding({}) var tsl_sampler_{i}:{};",buffer_count+i*2,texture_types[i].wgsl(),buffer_count+i*2+1,if texture_types[i]==Type::DepthTexture{"sampler_comparison"}else{"sampler"})).collect::<Vec<_>>().join("\n");
        let attributes = (0..buffer_count)
            .map(|i| {
                format!(
                    "@group(1) @binding({i}) var<storage,read> tsl_attribute_{i}:array<{}>;",
                    types[i].wgsl()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        Ok(format!(
            "{attributes}\n{resources}\n{functions}\nfn deform(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{{\n{}return {position};\n}}\nfn shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>{{\n{}return {color};\n}}",
            vertex.body, fragment.body
        ))
    }
    pub async fn build(
        &self,
        renderer: &Renderer,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<ShaderMaterial> {
        self.build_with_storage(renderer, &[], textures).await
    }
    /// Build with explicitly typed sampled texture views.
    pub async fn build_with_texture_types(
        &self,
        renderer: &Renderer,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler, Type)],
    ) -> Result<ShaderMaterial> {
        let types: Vec<_> = textures.iter().map(|x| x.2).collect();
        let source = self.wgsl_with_texture_types(&types, &[])?;
        let dimensions: Vec<_> = types
            .iter()
            .map(|t| {
                if *t == Type::TextureArray {
                    wgpu::TextureViewDimension::D2Array
                } else if *t == Type::TextureCube {
                    wgpu::TextureViewDimension::Cube
                } else if *t == Type::Texture3D {
                    wgpu::TextureViewDimension::D3
                } else {
                    wgpu::TextureViewDimension::D2
                }
            })
            .collect();
        let sample_types: Vec<_> = types
            .iter()
            .map(|t| {
                if *t == Type::DepthTexture {
                    wgpu::TextureSampleType::Depth
                } else {
                    wgpu::TextureSampleType::Float { filterable: true }
                }
            })
            .collect();
        let views: Vec<_> = textures.iter().map(|x| (x.0, x.1)).collect();
        Ok(ShaderMaterial::new(Arc::new(
            ShaderProgram::with_texture_dimensions(
                renderer,
                &source,
                &[],
                &views,
                &dimensions,
                &sample_types,
            )
            .await?,
        )))
    }
    /// Interpolation of the primary UV varying, useful for texture atlases with MSAA.
    pub async fn build_interpolated(
        &self,
        renderer: &Renderer,
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        interpolation: crate::shader::UvInterpolation,
    ) -> Result<ShaderMaterial> {
        let source = self.wgsl(textures.len())?;
        Ok(ShaderMaterial::new(Arc::new(
            ShaderProgram::with_uv_interpolation(renderer, &source, textures, interpolation)
                .await?,
        )))
    }
    pub async fn build_with_storage(
        &self,
        renderer: &Renderer,
        buffers: &[(&crate::compute::GpuBuffer, Type)],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<ShaderMaterial> {
        if buffers
            .iter()
            .any(|(b, _)| matches!(b.access, crate::compute::BufferAccess::Uniform))
        {
            return Err(Error::Invalid(
                "TSL storage binding requires storage buffer",
            ));
        }
        let types: Vec<_> = buffers.iter().map(|(_, t)| *t).collect();
        let refs: Vec<_> = buffers.iter().map(|(b, _)| *b).collect();
        let source = self.wgsl_with_storage(textures.len(), &types)?;
        Ok(ShaderMaterial::new(Arc::new(
            ShaderProgram::with_textures(renderer, &source, &refs, textures).await?,
        )))
    }
}
/// Compile the same expression API for a fullscreen GPU pass.
pub fn effect_wgsl(color: &Node) -> Result<String> {
    effect_wgsl_with_textures(color, 0)
}
pub fn effect_wgsl_with_textures(color: &Node, textures: usize) -> Result<String> {
    effect_source(color, textures, false)
}
fn effect_source(color: &Node, textures: usize, depth: bool) -> Result<String> {
    let mut compiler = Compiler::new(Stage::Effect, textures);
    compiler.depth = depth;
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
        "{}\n{functions}\nfn effect(uv:vec2<f32>)->vec4<f32>{{\n{}return {value};\n}}",
        if depth {
            "@group(1) @binding(0) var tsl_depth:texture_depth_2d;".into()
        } else {
            texture_declarations(textures)
        },
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
    Compute,
    Output,
}
struct Compiler {
    stage: Stage,
    textures: usize,
    texture_types: Vec<Type>,
    buffers: Vec<Type>,
    depth: bool,
    body: String,
    values: HashMap<usize, (Type, String)>,
    functions: HashMap<String, String>,
}
impl Compiler {
    fn new(stage: Stage, textures: usize) -> Self {
        Self {
            stage,
            textures,
            texture_types: vec![Type::Texture; textures],
            buffers: vec![],
            depth: false,
            body: String::new(),
            values: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    fn texture(&self, texture: Texture) -> Result<(String, String)> {
        match texture {
            Texture::Map if matches!(self.stage, Stage::Vertex | Stage::Fragment) => {
                Ok(("color_map".into(), "color_sampler".into()))
            }
            Texture::History if self.stage == Stage::Effect => {
                Ok(("history_texture".into(), "input_sampler".into()))
            }
            Texture::Input if self.stage == Stage::Effect => {
                Ok(("input_texture".into(), "input_sampler".into()))
            }
            Texture::External(i) if self.stage != Stage::Compute && i < self.textures => {
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
            Expr::Output => {
                if self.stage != Stage::Output {
                    return Err(Error::Invalid("TSL output stage"));
                }
                (Type::Vec4, "value".into())
            }
            Expr::Uint(x) => (Type::Uint, format!("{x}u")),
            Expr::InstanceIndex => (
                Type::Uint,
                match self.stage {
                    Stage::Compute => "tsl_index",
                    Stage::Vertex => "vertex_instance_index",
                    Stage::Fragment => "surface.instance_index",
                    _ => return Err(Error::Invalid("TSL instance index stage")),
                }
                .into(),
            ),
            Expr::VertexIndex => {
                if self.stage != Stage::Vertex {
                    return Err(Error::Invalid("TSL vertex index stage"));
                }
                (Type::Uint, "tsl_vertex_index".into())
            }
            Expr::StorageElement(i, index) => {
                let ty = *self
                    .buffers
                    .get(*i)
                    .ok_or(Error::Invalid("TSL storage binding"))?;
                if !matches!(self.stage, Stage::Vertex | Stage::Fragment | Stage::Compute) {
                    return Err(Error::Invalid("TSL storage stage"));
                }
                let (it, value) = self.emit(index)?;
                if it != Type::Uint {
                    return Err(Error::Invalid("TSL storage index requires uint"));
                }
                (ty, format!("tsl_attribute_{i}[{value}]"))
            }
            Expr::PositionLocal | Expr::NormalWorld => {
                if !matches!(self.stage, Stage::Fragment | Stage::Output) {
                    return Err(Error::Invalid("TSL fragment attribute stage"));
                }
                let surface = if self.stage == Stage::Output {
                    "fragment_surface"
                } else {
                    "surface"
                };
                (
                    Type::Vec3,
                    if matches!(&*node.0, Expr::PositionLocal) {
                        format!("{surface}.local_position")
                    } else {
                        format!(
                            "normalize(transpose(mat3x3(u.view[0].xyz,u.view[1].xyz,u.view[2].xyz))*{surface}.normal)"
                        )
                    },
                )
            }
            Expr::SurfaceVector(field) => {
                if !matches!(self.stage, Stage::Fragment | Stage::Output) {
                    return Err(Error::Invalid("TSL surface vector stage"));
                }
                let surface = if self.stage == Stage::Output {
                    "fragment_surface"
                } else {
                    "surface"
                };
                (
                    Type::Vec3,
                    format!(
                        "{}normalize({surface}.{field})",
                        if *field == "view_position" { "-" } else { "" }
                    ),
                )
            }
            Expr::LitProperty(name, ty) => {
                if self.stage != Stage::Output {
                    return Err(Error::Invalid("TSL material output property stage"));
                }
                (*ty, (*name).into())
            }
            Expr::PositionWorld | Expr::ViewZ => {
                if !matches!(self.stage, Stage::Output | Stage::Fragment) {
                    return Err(Error::Invalid("TSL fragment position stage"));
                }
                match (&*node.0, self.stage) {
                    (Expr::PositionWorld, Stage::Output) => {
                        (Type::Vec3, "fragment_position_world".into())
                    }
                    (Expr::PositionWorld, _) => (Type::Vec3, "surface.position".into()),
                    (_, Stage::Output) => (Type::Float, "fragment_view_z".into()),
                    _ => (Type::Float, "(-surface.view_position.z)".into()),
                }
            }
            Expr::ScreenSize => match self.stage {
                Stage::Vertex | Stage::Fragment | Stage::Output => {
                    (Type::Vec2, "u.point.xy".into())
                }
                _ => return Err(Error::Invalid("TSL screen size stage")),
            },
            Expr::ScreenCoordinate => match self.stage {
                Stage::Fragment => (Type::Vec2, "surface.clip.xy".into()),
                Stage::Output => (Type::Vec2, "fragment_surface.clip.xy".into()),
                _ => return Err(Error::Invalid("TSL screen coordinate stage")),
            },
            Expr::DepthSample(coordinate) => {
                if self.stage != Stage::Effect || !self.depth {
                    return Err(Error::Invalid("TSL depth texture binding"));
                }
                let (ty, value) = self.emit(coordinate)?;
                if ty != Type::Vec2 {
                    return Err(Error::Invalid("TSL depth UV type"));
                }
                (
                    Type::Float,
                    format!(
                        "textureLoad(tsl_depth,clamp(vec2<i32>({value}*vec2<f32>(textureDimensions(tsl_depth))),vec2(0),vec2<i32>(textureDimensions(tsl_depth))-1),0)"
                    ),
                )
            }
            Expr::BaseColor => {
                if self.stage != Stage::Fragment {
                    return Err(Error::Invalid("TSL base color stage"));
                }
                (Type::Vec4, "base".into())
            }
            Expr::Constant(x) => {
                if !x.is_finite() {
                    return Err(Error::Invalid("nonfinite TSL constant"));
                }
                (Type::Float, format!("{x:?}"))
            }
            Expr::Uniform(i, ty) => {
                if *i >= 16 || ty.lanes() == 0 || matches!(ty, Type::Uint | Type::UVec2) {
                    return Err(Error::Invalid("TSL uniform slot or type"));
                }
                let prefix = if matches!(self.stage, Stage::Effect | Stage::Compute) {
                    "params"
                } else {
                    "u.custom"
                };
                (*ty, format!("{prefix}[{i}].{}", &"xyzw"[..ty.lanes()]))
            }
            Expr::Uv if self.stage == Stage::Compute => {
                return Err(Error::Invalid("TSL UV is unavailable in this stage"));
            }
            Expr::Uv => (
                Type::Vec2,
                if self.stage == Stage::Fragment {
                    "surface.uv"
                } else if self.stage == Stage::Output {
                    "fragment_surface.uv"
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
                    if t.lanes() == 0
                        || matches!(ty, Type::UVec2) != matches!(t, Type::Uint | Type::UVec2)
                    {
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
                (
                    if t == Type::UVec2 {
                        match components.len() {
                            1 => Type::Uint,
                            2 => Type::UVec2,
                            _ => return Err(Error::Invalid("TSL unsigned swizzle width")),
                        }
                    } else {
                        Type::vector(components.len())?
                    },
                    format!("{v}.{components}"),
                )
            }
            Expr::Unary(op, input) => {
                let (t, v) = self.emit(input)?;
                if *op == "fwidth" && !matches!(self.stage, Stage::Fragment | Stage::Effect) {
                    return Err(Error::Invalid("TSL derivative stage"));
                }
                if *op == "f32" {
                    if ![Type::Float, Type::Bool, Type::Uint].contains(&t) {
                        return Err(Error::Invalid("TSL float conversion"));
                    }
                    (
                        Type::Float,
                        if t == Type::Bool {
                            format!("select(0.0,1.0,{v})")
                        } else if t == Type::Uint {
                            format!("f32({v})")
                        } else {
                            v
                        },
                    )
                } else {
                    if t.lanes() == 0 || matches!(t, Type::Uint | Type::UVec2) {
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
                } else if ["dot", "cross"].contains(op) {
                    if a.0 != b.0
                        || !matches!(a.0, Type::Vec2 | Type::Vec3 | Type::Vec4)
                        || (*op == "cross" && a.0 != Type::Vec3)
                    {
                        return Err(Error::Invalid("TSL vector operation"));
                    }
                    (
                        if *op == "dot" {
                            Type::Float
                        } else {
                            Type::Vec3
                        },
                        format!("{op}({},{})", a.1, b.1),
                    )
                } else if ["==", ">", "<"].contains(op) {
                    if a.0 != Type::Float || b.0 != Type::Float {
                        return Err(Error::Invalid("TSL comparison requires scalars"));
                    }
                    (Type::Bool, format!("({} {op} {})", a.1, b.1))
                } else {
                    let (t, a, b) = promote(a, b)?;
                    if matches!(t, Type::Uint | Type::UVec2)
                        && !["+", "-", "*", "/", "mod", "max"].contains(op)
                    {
                        return Err(Error::Invalid("TSL integer operation"));
                    }
                    (
                        t,
                        match *op {
                            "max" => format!("max({a},{b})"),
                            "pow" => format!("pow({a},{b})"),
                            "mod" if matches!(t, Type::Uint | Type::UVec2) => format!("({a}%{b})"),
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
            Expr::StorageLoad(i, coordinate) => {
                if self.stage != Stage::Compute || *i >= self.textures {
                    return Err(Error::Invalid("TSL storage texture binding or stage"));
                }
                let (ty, v) = self.emit(coordinate)?;
                if !matches!(ty, Type::Vec2 | Type::UVec2) {
                    return Err(Error::Invalid("TSL textureLoad coordinate"));
                }
                (
                    Type::Vec4,
                    format!("textureLoad(tsl_storage_{i},vec2<i32>({v}))"),
                )
            }
            Expr::Resource(texture, sampler) => {
                let (t, s) = self.texture(*texture)?;
                if *sampler {
                    (
                        if matches!(texture,Texture::External(i) if self.texture_types[*i]==Type::DepthTexture)
                        {
                            Type::ComparisonSampler
                        } else {
                            Type::Sampler
                        },
                        s,
                    )
                } else {
                    (
                        if let Texture::External(i) = texture {
                            self.texture_types[*i]
                        } else {
                            Type::Texture
                        },
                        t,
                    )
                }
            }
            Expr::SampleArray(texture, uv, layer) => {
                let (t, s) = self.texture(*texture)?;
                if !matches!(texture, Texture::External(i) if self.texture_types[*i]==Type::TextureArray)
                    || self.stage == Stage::Vertex
                {
                    return Err(Error::Invalid("TSL array sample binding or stage"));
                }
                let (uv_ty, uv) = self.emit(uv)?;
                let (layer_ty, layer) = self.emit(layer)?;
                if uv_ty != Type::Vec2 || !matches!(layer_ty, Type::Float | Type::Uint) {
                    return Err(Error::Invalid("TSL array sample coordinate or layer"));
                }
                (
                    Type::Vec4,
                    format!("textureSample({t},{s},{uv},i32({layer}))"),
                )
            }
            Expr::Sample(texture, args) => {
                let (t, s) = self.texture(*texture)?;
                if matches!(texture, Texture::External(i) if self.texture_types[*i]!=Type::Texture)
                {
                    return Err(Error::Invalid("TSL 2D sample requires a 2D texture"));
                }
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
        let value = if matches!(
            ty,
            Type::Texture
                | Type::TextureArray
                | Type::Texture3D
                | Type::TextureCube
                | Type::DepthTexture
                | Type::ComparisonSampler
                | Type::Sampler
        ) {
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
    if ta.lanes() == 0
        || tb.lanes() == 0
        || matches!(ta, Type::Uint | Type::UVec2) != matches!(tb, Type::Uint | Type::UVec2)
        || (ta != tb && ta.lanes() != 1 && tb.lanes() != 1)
    {
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

fn texture_declarations(count: usize) -> String {
    (0..count).map(|i|format!("@group(1) @binding({}) var tsl_texture_{i}:texture_2d<f32>;\n@group(1) @binding({}) var tsl_sampler_{i}:sampler;",i*2,i*2+1)).collect::<Vec<_>>().join("\n")
}
pub async fn effect_with_textures(
    renderer: &Renderer,
    format: wgpu::TextureFormat,
    color: &Node,
    textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
) -> Result<Effect> {
    Effect::with_textures(
        renderer,
        format,
        &effect_wgsl_with_textures(color, textures.len())?,
        textures,
    )
    .await
}
pub fn mix(a: Node, b: Node, factor: Node) -> Node {
    a.clone() + (b - a) * factor
}
pub fn luminance(color: Node) -> Node {
    color.dot(vec3(float(0.2126), float(0.7152), float(0.0722)))
}
pub fn saturation(color: Node, amount: Node) -> Node {
    mix(splat(luminance(color.clone()), Type::Vec3), color, amount).max(float(0.0))
}
pub fn hue(color: Node, angle: Node) -> Node {
    let k = splat(float(0.57735), Type::Vec3);
    let c = angle.cos();
    (color.clone() * c.clone()
        + k.cross(color.clone()) * angle.sin()
        + k.clone() * k.dot(color) * (float(1.0) - c))
        .max(float(0.0))
}
/// DotScreenNode r186. Supply the UVs used by the destination quad.
/// For a WebGPU render-to-texture pass these match Effect's top-left UVs.
pub fn dot_screen(color: Node, coordinate: Node, size: Node, angle: Node, scale: Node) -> Node {
    let p = coordinate * size;
    let s = angle.sin();
    let c = angle.cos();
    let point = vec2(c.clone() * p.x() - s.clone() * p.y(), s * p.x() + c * p.y()) * scale;
    let pattern = point.x().sin() * point.y().sin() * float(4.0);
    let avg = (color.x() + color.y() + color.swizzle("z")) / float(3.0);
    vec4(
        splat(avg * float(10.0) - float(5.0) + pattern, Type::Vec3),
        color.swizzle("w"),
    )
}
pub fn rgb_shift(texture: Texture, coordinate: Node, amount: Node, angle: Node) -> Node {
    let offset = vec2(angle.cos(), angle.sin()) * amount;
    let center = texture.sample(coordinate.clone());
    vec4(
        vec3(
            texture.sample(coordinate.clone() + offset.clone()).x(),
            center.y(),
            texture.sample(coordinate - offset).swizzle("z"),
        ),
        center.swizzle("w"),
    )
}

pub mod compute;

pub mod display;
pub mod lut;
pub mod smaa;

pub mod bloom;
pub mod sprites;
pub mod surface;

fn validate_storage_types(types: &[Type]) -> Result<()> {
    if types.iter().any(|t| {
        !matches!(
            t,
            Type::Float | Type::Vec2 | Type::Vec3 | Type::Vec4 | Type::Uint | Type::UVec2
        )
    }) {
        return Err(Error::Invalid("TSL storage element type"));
    }
    Ok(())
}
/// PCG hash used by Three.js. Executes integer arithmetic on the GPU.
pub fn hash(seed: Node) -> Node {
    WgslFn::new("tsl_hash", "fn tsl_hash(seed:u32)->f32 { let state=seed*747796405u+2891336453u; let word=((state>>((state>>28u)+4u))^state)*277803737u; return f32((word>>22u)^word)*(1.0/4294967296.0); }", &[Type::Uint], Type::Float).unwrap().call(&[seed])
}

/// Circle opacity with the same derivative smoothing as Three.js shapeCircle.
/// Enable the material's alpha_to_coverage when `antialias` is true.
pub fn shape_circle(antialias: bool) -> Node {
    let p = uv() * float(2.0) - float(1.0);
    let len = p.clone().dot(p);
    if antialias {
        let d = len.fwidth();
        let t = ((len - (float(1.0) - d.clone())) / (d * float(2.0))).clamp(float(0.0), float(1.0));
        float(1.0) - t.clone() * t.clone() * (float(3.0) - float(2.0) * t)
    } else {
        len.greater_than(float(1.0)).select(float(0.0), float(1.0))
    }
}

/// MaterialX scalar 3D Perlin noise. Evaluated on the GPU; signed, with the
/// pinned Three.js hash, gradient and amplitude normalization.
pub fn mx_noise_float3(position: Node) -> Node {
    WgslFn::new(
        "tsl_mx_noise3",
        include_str!("noise.wgsl"),
        &[Type::Vec3],
        Type::Float,
    )
    .unwrap()
    .call(&[position])
}

/// Nearest depth sampling from a single-sampled GPU depth attachment.
pub fn depth_texture(coordinate: Node) -> Node {
    Node::new(Expr::DepthSample(coordinate))
}
pub async fn depth_effect(
    renderer: &Renderer,
    format: wgpu::TextureFormat,
    color: &Node,
    depth: &wgpu::TextureView,
) -> Result<Effect> {
    Effect::with_depth(renderer, format, &effect_source(color, 0, true)?, depth).await
}

/// As `depth_effect`, with a multisampled depth input. `depth_texture` reads sample 0,
/// matching Three.js PassNode; it does not average or copy depth samples.
pub async fn multisampled_depth_effect(
    renderer: &Renderer,
    format: wgpu::TextureFormat,
    color: &Node,
    depth: &wgpu::TextureView,
) -> Result<Effect> {
    let source = effect_source(color, 0, true)?.replace(
        "tsl_depth:texture_depth_2d",
        "tsl_depth:texture_depth_multisampled_2d",
    );
    Effect::with_multisampled_depth(renderer, format, &source, depth).await
}

/// Rotate a position by XYZ Euler angles, matching r186's rotate(vec3, vec3).
/// Evaluated per vertex on the GPU; does not rebuild geometry.
pub fn rotate_euler(position: Node, angles: Node) -> Node {
    WgslFn::new("tsl_rotate_euler", "fn tsl_rotate_euler(p:vec3<f32>,a:vec3<f32>)->vec3<f32>{let c=cos(a);let s=sin(a);let z=vec3(c.z*p.x-s.z*p.y,s.z*p.x+c.z*p.y,p.z);let y=vec3(c.y*z.x+s.y*z.z,z.y,-s.y*z.x+c.y*z.z);return vec3(y.x,c.x*y.y-s.x*y.z,s.x*y.y+c.x*y.z);}", &[Type::Vec3,Type::Vec3],Type::Vec3).unwrap().call(&[position,angles])
}

/// Three.js triangle oscillator, in the range zero to one.
pub fn osc_triangle(t: Node) -> Node {
    ((t + float(0.5)).fract() * float(2.0) - float(1.0)).abs()
}

pub mod volume;

pub mod sampling;

pub mod dof;
