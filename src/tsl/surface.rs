//! Node inputs for the existing lit materials. Lighting remains in the renderer;
//! node expressions replace selected inputs before BRDF evaluation on the GPU.
use super::*;

/// Normal is in view space. Unspecified inputs retain the material/map value.
/// Attach the resulting program to `MaterialProperties::vertex_program` and
/// update `vertex_uniforms` without rebuilding the program.
#[derive(Clone, Debug, Default)]
pub struct SurfaceNodes {
    pub position: Option<Node>,
    /// Linear HDR environment radiance, evaluated for each BRDF sampling direction.
    pub environment: Option<Node>,
    pub output: Option<Node>,
    /// Replace/mix diffuse lighting with framebuffer RGB; W is the mix factor.
    pub backdrop: Option<Node>,
    pub color: Option<Node>,
    pub normal: Option<Node>,
    pub roughness: Option<Node>,
    pub metalness: Option<Node>,
    pub emissive: Option<Node>,
    /// Baked indirect diffuse irradiance (linear RGB), before the Lambert BRDF.
    pub light_map: Option<Node>,
    /// Phong RGB specular color and shininess in W.
    pub specular: Option<Node>,
}

impl SurfaceNodes {
    pub fn wgsl(&self, texture_count: usize, types: &[Type]) -> Result<(String, String, String)> {
        let graph = NodeMaterial {
            position: self.position.clone(),
            color: self
                .color
                .clone()
                .unwrap_or_else(|| Node::new(Expr::BaseColor)),
        };
        let mut source = graph.wgsl_with_storage(texture_count, types)?;
        let mut fragment = Compiler::new(Stage::Fragment, texture_count);
        fragment.buffers = types.to_vec();
        let mut assignments = String::new();
        for (name, node, expected) in [
            ("backdrop", &self.backdrop, Type::Vec4),
            ("normal", &self.normal, Type::Vec3),
            ("roughness", &self.roughness, Type::Float),
            ("metalness", &self.metalness, Type::Float),
            ("emissive", &self.emissive, Type::Vec3),
            ("light_map", &self.light_map, Type::Vec3),
            ("specular", &self.specular, Type::Vec4),
        ] {
            if let Some(node) = node {
                let (ty, value) = fragment.emit(node)?;
                if ty != expected {
                    return Err(Error::Invalid("TSL surface input type"));
                }
                assignments.push_str(&format!("result.{name}={value};\n"));
            }
        }
        // Compile helpers jointly to reject same-name, different-body functions.
        let mut shared = Compiler::new(Stage::Fragment, texture_count);
        shared.buffers = types.to_vec();
        shared.emit(&graph.color)?;
        let mut vertex = Compiler::new(Stage::Vertex, texture_count);
        vertex.buffers = types.to_vec();
        if let Some(position) = &self.position {
            vertex.emit(position)?;
        }
        for (name, body) in vertex.functions {
            insert_function(&mut shared.functions, name, body)?;
        }
        let mut helpers: Vec<_> = fragment.functions.into_iter().collect();
        helpers.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, body) in helpers {
            if !shared.functions.contains_key(&name) {
                source.push_str(&format!("\n{body}"));
            }
            insert_function(&mut shared.functions, name, body)?;
        }
        if let Some(node) = &self.environment {
            let mut compiler = Compiler::new(Stage::Output, texture_count);
            compiler.environment = true;
            compiler.buffers = types.to_vec();
            let (ty, value) = compiler.emit(node)?;
            if ty != Type::Vec3 {
                return Err(Error::Invalid("TSL environment requires RGB"));
            }
            let mut helpers: Vec<_> = compiler.functions.into_iter().collect();
            helpers.sort_by(|a, b| a.0.cmp(&b.0));
            for (name, body) in helpers {
                if !shared.functions.contains_key(&name) {
                    source.push_str(&format!("\n{body}"));
                }
                insert_function(&mut shared.functions, name, body)?;
            }
            source.push_str(&format!("\nfn environment_sample(environment_direction:vec3<f32>,environment_roughness:f32)->vec3<f32>{{\n{}return {value}*u.environment.x;\n}}",compiler.body));
        }
        let output = if let Some(node) = &self.output {
            let mut compiler = Compiler::new(Stage::Output, texture_count);
            compiler.buffers = types.to_vec();
            let (ty, value) = compiler.emit(node)?;
            let value = output_color(ty, value)?;
            let mut helpers: Vec<_> = compiler.functions.into_iter().collect();
            helpers.sort_by(|a, b| a.0.cmp(&b.0));
            for (name, body) in helpers {
                if !shared.functions.contains_key(&name) {
                    source.push_str(&format!("\n{body}"));
                }
                insert_function(&mut shared.functions, name, body)?;
            }
            format!(
                "fn transform_output(value:vec4<f32>)->vec4<f32>{{\n{}return {value};\n}}",
                compiler.body
            )
        } else {
            crate::shader::DEFAULT_OUTPUT.into()
        };
        Ok((
            source,
            format!(
                "fn transform_surface(surface:VertexOut,value:LitSurface)->LitSurface{{\n{}var result=value;\n{assignments}return result;\n}}",
                fragment.body
            ),
            output,
        ))
    }

    /// Write named-by-index color attachments in one scene draw. Each graph is
    /// evaluated after lighting; `output()` references the ordinary material output.
    /// Attachment formats may differ, with floating-point fragment outputs.
    pub async fn build_mrt(
        &self,
        renderer: &Renderer,
        outputs: &[Node],
        buffers: &[(&crate::compute::GpuBuffer, Type)],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<ShaderProgram> {
        self.mrt_program(
            renderer,
            outputs,
            buffers,
            textures,
            crate::shader::DEFAULT_PROJECTION,
        )
        .await
    }
    /// Compile a full-screen background graph with multiple attachments.
    pub async fn build_background_mrt(
        &self,
        renderer: &Renderer,
        outputs: &[Node],
    ) -> Result<ShaderMaterial> {
        let program = self
            .mrt_program(renderer, outputs, &[], &[], BACKGROUND_PROJECTION)
            .await?;
        let mut material = ShaderMaterial::new(Arc::new(program));
        material.properties.depth_test = false;
        material.properties.depth_write = false;
        Ok(material)
    }
    async fn mrt_program(
        &self,
        renderer: &Renderer,
        outputs: &[Node],
        buffers: &[(&crate::compute::GpuBuffer, Type)],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        projection: &str,
    ) -> Result<ShaderProgram> {
        if outputs.is_empty()
            || outputs.len() > renderer.device.limits().max_color_attachments as usize
        {
            return Err(Error::Invalid("TSL MRT attachment count"));
        }
        if buffers
            .iter()
            .any(|(b, _)| matches!(b.access, crate::compute::BufferAccess::Uniform))
        {
            return Err(Error::Invalid(
                "TSL storage binding requires storage buffer",
            ));
        }
        let types: Vec<_> = buffers.iter().map(|(_, ty)| *ty).collect();
        let buffers: Vec<_> = buffers.iter().map(|(b, _)| *b).collect();
        let (mut source, surface, output) = self.wgsl(textures.len(), &types)?;
        let mut compiler = Compiler::new(Stage::Output, textures.len());
        compiler.buffers = types;
        let mut fields = String::new();
        let mut assignments = String::new();
        for (i, node) in outputs.iter().enumerate() {
            let (ty, value) = compiler.emit(node)?;
            let value = output_color(ty, value)?;
            fields.push_str(&format!("@location({i}) attachment_{i}:vec4<f32>,"));
            assignments.push_str(&format!("result.attachment_{i}={value};\n"));
        }
        let mut helpers: Vec<_> = compiler.functions.into_iter().collect();
        helpers.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, body) in helpers {
            if !source.contains(&body) {
                source.push_str(&format!("\n{body}"));
            }
        }
        let mrt = format!(
            "\nstruct TslMrtOutput{{{fields}}}\n@fragment fn fs_main(surface:VertexOut,@builtin(front_facing) front:bool)->TslMrtOutput{{let value=color_main(surface,front);\n{}var result:TslMrtOutput;\n{assignments}return result;}}",
            compiler.body
        );
        ShaderProgram::with_mrt(
            renderer,
            &source,
            &surface,
            &output,
            &buffers,
            textures,
            outputs.len() as u32,
            &mrt,
            projection,
        )
        .await
    }
    pub async fn build(
        &self,
        renderer: &Renderer,
        buffers: &[(&crate::compute::GpuBuffer, Type)],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<ShaderProgram> {
        if buffers
            .iter()
            .any(|(b, _)| matches!(b.access, crate::compute::BufferAccess::Uniform))
        {
            return Err(Error::Invalid(
                "TSL storage binding requires storage buffer",
            ));
        }
        let types: Vec<_> = buffers.iter().map(|(_, ty)| *ty).collect();
        let buffers: Vec<_> = buffers.iter().map(|(buffer, _)| *buffer).collect();
        let (source, surface, output) = self.wgsl(textures.len(), &types)?;
        ShaderProgram::with_surface(renderer, &source, &surface, &output, &buffers, textures).await
    }
}

/// A screen-space background quad material. Use a resident 2x2 plane, disable
/// frustum culling and draw before scene geometry. UV is bottom-left oriented.
const BACKGROUND_PROJECTION: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut {var result=surface;result.clip=vec4(position.xy,1.0,1.0);return result;}";
pub async fn background_material(renderer: &Renderer, color: Node) -> Result<ShaderMaterial> {
    // BackgroundNode passes through NodeMaterial's unsigned color output.
    let source = NodeMaterial::new(color.max(float(0.0))).wgsl(0)?;
    let projection = BACKGROUND_PROJECTION;
    let program = ShaderProgram::with_projection(renderer, &source, &[], &[], projection).await?;
    let mut material = ShaderMaterial::new(Arc::new(program));
    material.properties.depth_write = false;
    material.properties.depth_test = false;
    Ok(material)
}

/// Tangent-space normal mapping with a graph-supplied sample and untransformed UVs,
/// as in Three.js NormalMapNode. The result is in view space for `SurfaceNodes.normal`.
pub fn normal_map(sample: Node, coordinate: Node) -> Node {
    WgslFn::new("tsl_normal_map",r#"
fn tsl_normal_map(sample:vec3<f32>,uv:vec2<f32>,position:vec3<f32>,normal:vec3<f32>)->vec3<f32>{
 let n=normalize(normal);let q0=dpdx(position);let q1=-dpdy(position);let st0=dpdx(uv);let st1=-dpdy(uv);
 let t=cross(q1,n)*st0.x+cross(n,q0)*st1.x;let b=cross(q1,n)*st0.y+cross(n,q0)*st1.y;
 let det=max(dot(t,t),dot(b,b));let scale=select(inverseSqrt(max(det,1e-30)),0.0,det==0.0);let mapped=sample*2.0-1.0;
 return normalize((u.view*vec4(t*scale*mapped.x+b*scale*mapped.y+n*mapped.z,0.0)).xyz)*select(-1.0,1.0,fragment_front);
}"#,&[Type::Vec3,Type::Vec2,Type::Vec3,Type::Vec3],Type::Vec3).expect("static normal-map function").call(&[sample,coordinate,position_world(),normal_world()])
}

/// Forward-difference bump mapping. Re-evaluates the height graph at offset UVs,
/// matching texture-context substitution without reading height data on the CPU.
pub fn bump_map(height: impl Fn(Node) -> Node, coordinate: Node, scale: Node) -> Node {
    let dx = WgslFn::new(
        "tsl_uv_dx",
        "fn tsl_uv_dx(v:vec2<f32>)->vec2<f32>{return dpdx(v);}",
        &[Type::Vec2],
        Type::Vec2,
    )
    .expect("static derivative");
    let dy = WgslFn::new(
        "tsl_uv_dy",
        "fn tsl_uv_dy(v:vec2<f32>)->vec2<f32>{return -dpdy(v);}",
        &[Type::Vec2],
        Type::Vec2,
    )
    .expect("static derivative");
    let base = height(coordinate.clone());
    let differences = vec2(
        height(coordinate.clone() + dx.call(std::slice::from_ref(&coordinate))) - base.clone(),
        height(coordinate.clone() + dy.call(&[coordinate])) - base,
    ) * scale;
    WgslFn::new(
        "tsl_bump_normal",
        r#"
fn tsl_bump_normal(d:vec2<f32>,position:vec3<f32>,normal:vec3<f32>)->vec3<f32>{
 let p=(u.view*vec4(position,1.0)).xyz;
 let n=normalize((u.view*vec4(normal,0.0)).xyz)*select(-1.0,1.0,fragment_front);
 let sx=normalize(dpdx(p));let sy=normalize(-dpdy(p));
 let r1=cross(sy,n);let r2=cross(n,sx);
 let det=dot(sx,r1)*select(-1.0,1.0,fragment_front);
 return normalize(abs(det)*n-sign(det)*(d.x*r1+d.y*r2));
}"#,
        &[Type::Vec2, Type::Vec3, Type::Vec3],
        Type::Vec3,
    )
    .expect("static bump mapping")
    .call(&[differences, position_world(), normal_world()])
}

/// Offset UVs in the tangent plane using the normalized view direction, as in
/// Three.js parallaxUV. Geometry must provide tangents (compute once at upload).
pub fn parallax_uv(coordinate: Node, scale: Node) -> Node {
    coordinate
        - vec2(
            position_view_direction().dot(tangent_view()),
            position_view_direction().dot(bitangent_view()),
        ) * scale
}
/// Photoshop-style overlay blend in linear working color space.
pub fn blend_overlay(base: Node, blend: Node) -> Node {
    WgslFn::new("tsl_blend_overlay", "fn tsl_blend_overlay(a:vec3<f32>,b:vec3<f32>)->vec3<f32>{return select(1.0-2.0*(1.0-a)*(1.0-b),2.0*a*b,a<vec3(0.5));}", &[Type::Vec3,Type::Vec3],Type::Vec3).unwrap().call(&[base,blend])
}

/// Derivative-based parallax for geometry without vertex tangents. `normal` is
/// the material's view-space normal, while `geometry_uv` is the original UV set.
/// Matches r186's shared normal-map context: the UV derivatives, bitangent and
/// normalization factor come from the geometric normal; the tangent's first
/// cross product uses the mapped normal when color nodes are evaluated.
pub fn parallax_uv_frame(coordinate: Node, scale: Node, normal: Node, geometry_uv: Node) -> Node {
    WgslFn::new("tsl_parallax_frame",r#"
fn tsl_parallax_frame(uv:vec2<f32>,scale:vec2<f32>,normal:vec3<f32>,base:vec2<f32>,world:vec3<f32>,gn:vec3<f32>)->vec2<f32>{
 let p=(u.view*vec4(world,1.0)).xyz;let n=normalize((u.view*vec4(gn,0.0)).xyz);let q0=dpdx(p);let q1=-dpdy(p);let st0=dpdx(base);let st1=-dpdy(base);let first=cross(q1,n);let second=cross(n,q0);let t=first*st0.x+second*st1.x;let b=first*st0.y+second*st1.y;let det=max(dot(t,t),dot(b,b));let factor=select(inverseSqrt(max(det,1e-30)),0.0,det==0.0);let mapped_t=(cross(q1,normal)*st0.x+second*st1.x)*factor;let direction=normalize(-p);return uv-vec2(dot(direction,mapped_t),dot(direction,b*factor))*scale;
}"#,&[Type::Vec2,Type::Vec2,Type::Vec3,Type::Vec2,Type::Vec3,Type::Vec3],Type::Vec2).unwrap().call(&[coordinate,scale,normal,geometry_uv,position_world(),normal_world()])
}

/// Derivative-scaled alpha hashing; surviving fragments retain their opacity.
/// Supply local position after instance transforms, as in r186 NodeMaterial.
pub fn alpha_hash(opacity: Node, position: Node, enabled: Node) -> Node {
    WgslFn::new(
        "tsl_alpha_hash",
        include_str!("alpha_hash.wgsl"),
        &[Type::Float, Type::Vec3, Type::Bool],
        Type::Float,
    )
    .unwrap()
    .call(&[opacity, position, enabled])
}
