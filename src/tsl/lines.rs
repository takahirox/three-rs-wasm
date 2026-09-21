//! Instanced GPU ribbons, matching Line2NodeMaterial's round caps and dash distances.
use crate::{
    Result,
    attribute::BufferAttribute,
    compute::{BufferAccess, GpuBuffer},
    geometry::{Attribute, BufferGeometry},
    material::ShaderMaterial,
    renderer::Renderer,
    shader::ShaderProgram,
};
use std::sync::Arc;
/// Shader variants follow Three.js material recompilation: dead discard paths
/// must disappear before compilation, including for derivative-based coverage.
#[derive(Clone, Copy, Default)]
pub struct LineOptions {
    pub world_units: bool,
    pub dashed: bool,
    pub alpha_to_coverage: bool,
}
/// Static start/distance, end/distance, start color and end color per segment.
/// One storage buffer can be shared across all material variants.
pub struct LineSegments {
    buffer: GpuBuffer,
}
impl LineSegments {
    pub fn new(renderer: &Renderer, segments: &[[[f32; 4]; 4]]) -> Result<Self> {
        Ok(Self {
            buffer: GpuBuffer::new(renderer, bytemuck::cast_slice(segments), BufferAccess::Read)?,
        })
    }
    /// Uniforms: [width, unused, unused, DPR], [scale, offset, dash, gap].
    /// Supports standard perspective/orthographic cameras with rigid camera transforms.
    pub async fn material(
        &self,
        renderer: &Renderer,
        options: LineOptions,
    ) -> Result<ShaderMaterial> {
        let specialize = |source: &str| {
            source
                .replace(
                    "u.custom[0].y>0.5",
                    if options.world_units { "true" } else { "false" },
                )
                .replace(
                    "u.custom[0].z>0.5",
                    if options.dashed { "true" } else { "false" },
                )
                .replace(
                    "u.custom[0].z<0.5",
                    if options.dashed { "false" } else { "true" },
                )
                .replace(
                    "u.custom[2].x>0.5",
                    if options.alpha_to_coverage {
                        "true"
                    } else {
                        "false"
                    },
                )
        };
        let program = ShaderProgram::with_projection(
            renderer,
            &specialize(include_str!("lines.wgsl")),
            &[&self.buffer],
            &[],
            &specialize(include_str!("lines_projection.wgsl")),
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.properties.alpha_to_coverage = options.alpha_to_coverage;
        Ok(m)
    }
}
pub fn geometry(count: u32) -> Result<BufferGeometry> {
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(
            vec![
                -1., 2., 0., 1., 2., 0., -1., 1., 0., 1., 1., 0., -1., 0., 0., 1., 0., 0., -1.,
                -1., 0., 1., -1., 0.,
            ],
            3,
            false,
        )?),
    );
    g.set_attribute(
        "uv",
        Attribute::F32(BufferAttribute::new(
            vec![
                -1., 2., 1., 2., -1., 1., 1., 1., -1., -1., 1., -1., -1., -2., 1., -2.,
            ],
            2,
            false,
        )?),
    );
    g.set_index(Some(vec![
        0, 2, 1, 2, 3, 1, 2, 4, 3, 4, 5, 3, 4, 6, 5, 6, 7, 5,
    ]));
    g.instance_count = Some(count);
    Ok(g)
}
