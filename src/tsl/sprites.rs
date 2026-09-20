//! GPU billboard material with resident, explicitly bound instance attributes.
use super::*;
use crate::compute::{BufferAccess, GpuBuffer};

pub struct SpriteNodeMaterial {
    pub color: Node,
    pub position: Node,
    pub scale: Node,
    pub rotation: Node,
    pub size_attenuation: Node,
    /// Rotate around world Y to face the camera, preserving the world up axis.
    pub horizontal_rotation: bool,
}
impl SpriteNodeMaterial {
    pub fn new(color: Node) -> Self {
        Self {
            color,
            position: splat(float(0.0), Type::Vec3),
            scale: float(1.0),
            rotation: float(0.0),
            size_attenuation: float(1.0),
            horizontal_rotation: false,
        }
    }
    /// Attributes are read-only arrays of vec4s. They are uploaded by the caller
    /// once, and can be shared by multiple materials. Geometry.instance_count
    /// controls the draw count without allocating identity instance matrices.
    pub async fn build(
        &self,
        renderer: &Renderer,
        buffers: &[&GpuBuffer],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<ShaderMaterial> {
        let buffers: Vec<_> = buffers.iter().map(|b| (*b, Type::Vec4)).collect();
        self.build_with_storage(renderer, &buffers, textures).await
    }
    pub async fn build_with_storage(
        &self,
        renderer: &Renderer,
        buffers: &[(&GpuBuffer, Type)],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<ShaderMaterial> {
        self.build_projected(renderer, buffers, textures, false)
            .await
    }
    /// Billboard sizes in physical viewport pixels, matching PointsNodeMaterial.
    pub async fn build_points(
        &self,
        renderer: &Renderer,
        buffers: &[(&GpuBuffer, Type)],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<ShaderMaterial> {
        self.build_projected(renderer, buffers, textures, true)
            .await
    }
    async fn build_projected(
        &self,
        renderer: &Renderer,
        buffers: &[(&GpuBuffer, Type)],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
        pixel_size: bool,
    ) -> Result<ShaderMaterial> {
        let types: Vec<_> = buffers.iter().map(|(_, t)| *t).collect();
        let refs: Vec<_> = buffers.iter().map(|(b, _)| *b).collect();
        if buffers
            .iter()
            .any(|(b, _)| matches!(b.access, BufferAccess::Uniform))
        {
            return Err(Error::Invalid("sprite attributes require storage buffers"));
        }
        let graph = NodeMaterial::new(self.color.clone().max(float(0.0)));
        let source = graph.wgsl_with_storage(textures.len(), &types)?;
        let mut c = Compiler::new(Stage::Vertex, textures.len());
        c.buffers = types;
        let (p, position) = c.emit(&self.position)?;
        let (s, scale) = c.emit(&self.scale)?;
        let (r, rotation) = c.emit(&self.rotation)?;
        let (a, attenuation) = c.emit(&self.size_attenuation)?;
        if p != Type::Vec3
            || !matches!(s, Type::Float | Type::Vec2)
            || r != Type::Float
            || a != Type::Float
        {
            return Err(Error::Invalid("sprite node types"));
        }
        let mut functions: Vec<_> = c.functions.values().cloned().collect();
        functions.sort();
        // Merge native functions with the fragment graph, rejecting conflicting definitions.
        let mut projection = String::new();
        for function in functions {
            if !source.contains(&function) {
                projection.push_str(&function);
                projection.push('\n');
            }
        }
        projection.push_str(&format!("fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{\nlet uv=surface.uv;let normal=surface.normal;\n{}\nlet center=u.model*vec4({position},1.0);let view=u.view*center;\nvar scale=vec2(length(u.model[0].xyz),length(u.model[1].xyz))*vec2({scale});\nif {attenuation}<0.5 && u.projection[3][3]==0.0 {{scale*=-view.z;}}\nlet point=position.xy*scale;let angle={rotation};let rotated=vec2(cos(angle)*point.x-sin(angle)*point.y,sin(angle)*point.x+cos(angle)*point.y);\nvar out=surface;out.local_position={position};out.position=center.xyz;out.view_position=view.xyz;out.clip=u.projection*vec4(view.xy+rotated,view.zw);return out;\n}}",c.body));
        if pixel_size {
            let old =
                format!("if {attenuation}<0.5 && u.projection[3][3]==0.0 {{scale*=-view.z;}}");
            projection=projection.replace(&old,"").replace("out.clip=u.projection*vec4(view.xy+rotated,view.zw);","out.clip=u.projection*view;out.clip.x+=rotated.x*2.0/(u.point.x*u.transmission[2].z)*out.clip.w;out.clip.y+=rotated.y*2.0/(u.point.y*u.transmission[2].w)*out.clip.w;");
        }
        if self.horizontal_rotation {
            projection=projection.replace("out.clip=u.projection*vec4(view.xy+rotated,view.zw);", "let look=normalize(vec3(u.camera.x-center.x,0.0,u.camera.z-center.z));let right=vec3(look.z,0.0,-look.x);out.clip=u.projection*(view+u.view*vec4(right*rotated.x+vec3(0.0,rotated.y,0.0),0.0));");
        }
        let program =
            ShaderProgram::with_projection(renderer, &source, &refs, textures, &projection).await?;
        let mut material = ShaderMaterial::new(Arc::new(program));
        material.properties.transparent = true;
        Ok(material)
    }
}
/// Three.js's exponential height fog factor in fragment space.
pub fn exponential_height_fog_factor(density: Node, height: Node) -> Node {
    let m = (height - position_world().y()).max(float(0.0)) * view_z();
    float(1.0) - (-(density.clone() * density * m.clone() * m)).exp()
}
