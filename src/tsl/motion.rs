//! Motion-vector MRT. Clip transforms execute per vertex, as in r186 VelocityNode.
use super::*;

/// Current/previous unjittered homogeneous clip coordinates, evaluated on the GPU
/// before interpolation. For skin/morph animation `previous_clip` must read the
/// previous deformed position; reusing current deformation is only valid for
/// rigid objects. Camera jitter must not be included in either matrix.
pub struct MotionVectors {
    pub current_clip: Node,
    pub previous_clip: Node,
}
impl MotionVectors {
    pub async fn build(
        &self,
        renderer: &crate::renderer::Renderer,
        surface: &surface::SurfaceNodes,
        buffers: &[(&crate::compute::GpuBuffer, Type)],
        textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
    ) -> Result<crate::shader::ShaderProgram> {
        let mut compiler = Compiler::new(Stage::Vertex, textures.len());
        compiler.buffers = buffers.iter().map(|(_, t)| *t).collect();
        let (ct, current) = compiler.emit(&self.current_clip)?;
        let (pt, previous) = compiler.emit(&self.previous_clip)?;
        if ct != Type::Vec4 || pt != Type::Vec4 {
            return Err(Error::Invalid("motion clip coordinates must be vec4"));
        }
        let mut projection = String::new();
        let mut helpers = compiler.functions.into_iter().collect::<Vec<_>>();
        helpers.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, body) in helpers {
            projection.push_str(&body);
        }
        projection.push_str(&format!("\nfn project_motion(surface:VertexOut,position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->VertexOut{{var result=surface;\n{}result.motion_current={current};result.motion_previous={previous};return result;}}\nfn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{return project_motion(surface,position,surface.local_normal,surface.uv);}}",compiler.body));
        let v = Node::new(Expr::MotionVector);
        surface
            .mrt_program(
                renderer,
                &[output(), vec4(vec3(v.x(), v.y(), float(0.)), float(1.))],
                buffers,
                textures,
                &projection,
            )
            .await
    }
}
/// Transform a GPU position by four column nodes (uniform or storage backed).
pub fn clip_position(position: Node, columns: [Node; 4]) -> Node {
    let mut args = vec![position];
    args.extend(columns);
    WgslFn::new("tsl_motion_clip", "fn tsl_motion_clip(p:vec3<f32>,a:vec4<f32>,b:vec4<f32>,c:vec4<f32>,d:vec4<f32>)->vec4<f32>{return mat4x4(a,b,c,d)*vec4(p,1.0);}", &[Type::Vec3,Type::Vec4,Type::Vec4,Type::Vec4,Type::Vec4],Type::Vec4).unwrap().call(&args)
}

/// Previous skeletal/morph pose for a motion pass. Static influences and morph
/// deltas reuse Core's resident deformation storage; only bone matrices and
/// morph weights are transferred. Call `update` once per animation frame, after
/// updating scene matrices and before rendering any motion passes.
pub struct PreviousPose {
    pub buffer: crate::compute::GpuBuffer,
    object: crate::scene::Object3D,
    last: Vec<f32>,
    uploaded: Vec<f32>,
}
impl PreviousPose {
    pub fn new(
        r: &crate::renderer::Renderer,
        s: &crate::scene::Scene,
        object: crate::scene::Object3D,
    ) -> Result<Self> {
        let values = Self::values(s, object)?;
        let buffer = crate::compute::GpuBuffer::new(
            r,
            bytemuck::cast_slice(&values),
            crate::compute::BufferAccess::Read,
        )?;
        Ok(Self {
            buffer,
            object,
            last: values.clone(),
            uploaded: values,
        })
    }
    fn values(s: &crate::scene::Scene, object: crate::scene::Object3D) -> Result<Vec<f32>> {
        let n = s.get(object)?;
        let g = n
            .geometry()
            .ok_or(Error::Invalid("motion pose requires geometry"))?;
        let mut values = Vec::new();
        if let Some(skin) = &n.skin {
            if skin.joints.len() != skin.inverse_bind_matrices.len()
                || n.matrix_world.determinant() == 0.
            {
                return Err(Error::Invalid("motion skin matrices"));
            }
            let inverse = n.matrix_world.inverse();
            for (&joint, bind) in skin.joints.iter().zip(&skin.inverse_bind_matrices) {
                let matrix = inverse * s.get(joint)?.matrix_world * *bind;
                if !matrix.is_finite() {
                    return Err(Error::Invalid("motion bone matrix"));
                }
                values.extend(matrix.as_mat4().to_cols_array());
            }
        }
        let count = g.morph_attributes.values().map(Vec::len).max().unwrap_or(0);
        if n.morph_weights.len() > count || !n.morph_weights.iter().all(|w| w.is_finite()) {
            return Err(Error::Invalid("motion morph weights"));
        }
        values.extend((0..count).map(|i| n.morph_weights.get(i).copied().unwrap_or(0.) as f32));
        if values.is_empty() {
            values.resize(4, 0.);
        }
        Ok(values)
    }
    pub fn update(&mut self, r: &crate::renderer::Renderer, s: &crate::scene::Scene) -> Result<()> {
        let values = Self::values(s, self.object)?;
        if values.len() != self.last.len() {
            return Err(Error::Invalid("motion pose layout changed"));
        }
        if self.uploaded != self.last {
            self.buffer.write(r, 0, bytemuck::cast_slice(&self.last))?;
            self.uploaded.clone_from(&self.last);
        }
        self.last = values;
        Ok(())
    }
}
/// Previous deformed position for `MotionVectors::previous_clip`. Bind the
/// `PreviousPose::buffer` at `binding` as `Type::Float`. Evaluation is vertex-only.
/// Custom displacement must also be applied here using its previous parameters.
pub fn previous_position(binding: usize) -> Node {
    let name = format!("tsl_previous_position_{binding}");
    let body = include_str!("motion_pose.wgsl")
        .replace("POSE", &format!("tsl_attribute_{binding}"))
        .replace("FUNCTION", &name);
    WgslFn::new(&name, &body, &[Type::Float], Type::Vec3)
        .unwrap()
        .call(&[storage_element(binding, uint(0))])
}
