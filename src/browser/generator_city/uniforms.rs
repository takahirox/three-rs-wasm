//! The city materials' uniform values, recognized by how each captured
//! shader uses its struct fields: the object's matrices, the standard
//! material's constants, the probe grid, the PMREM environment, the
//! material's own uniforms ( in field order ), and the sun light with its
//! shadow.
use super::geo::{IDENTITY, M4};

type Values = Vec<(String, Vec<f64>)>;
/// The cube-UV texel sizes at lodMax 8.
pub(super) const TEXEL: [f64; 2] = [1. / 768., 1. / 1024.];
/// scene.environmentIntensity.
pub(super) const ENVIRONMENT: f64 = 0.05;
/// A probe grid's bounds ( max, min ), resolution and intensity.
pub(super) type Grid = ([f64; 3], [f64; 3], [f64; 3], f64);
/// One object's inputs.
pub(super) struct Object<'a> {
    pub(super) model: M4,
    pub(super) seed: u32,
    pub(super) roughness: f64,
    pub(super) metalness: f64,
    /// The probe grid's bounds ( max, min ), resolution and intensity.
    pub(super) grid: Option<Grid>,
    /// The material's own uniforms, in field order.
    pub(super) custom: &'a [Vec<f64>],
}
/// The camera, the sun and its shadow.
pub(super) struct Render {
    pub(super) projection: M4,
    pub(super) view: M4,
    pub(super) world: M4,
    pub(super) light_color: [f64; 3],
    pub(super) light_position: [f64; 3],
    pub(super) shadow_matrix: M4,
    pub(super) normal_bias: f64,
    pub(super) map_size: f64,
    pub(super) time: f64,
}
/// A struct's fields ( name, type ) in declaration order.
pub(crate) fn fields(source: &str, name: &str) -> Vec<(String, String)> {
    let Some(start) = source.find(&format!("struct {name} {{")) else {
        return vec![];
    };
    let body = &source[start..];
    let body = &body[body.find('{').unwrap_or(0) + 1..body.find('}').unwrap_or(0)];
    body.split(",\n")
        .filter_map(|f| {
            let (n, t) = f.split_once(':')?;
            Some((
                n.trim().to_string(),
                t.trim().trim_end_matches(',').to_string(),
            ))
        })
        .collect()
}
/// The lines using `<group>.<name>` ( not a longer name ).
pub(crate) fn uses<'a>(sources: &[&'a str], group: &str, name: &str) -> Vec<&'a str> {
    let needle = format!("{group}.{name}");
    let mut out = vec![];
    for source in sources {
        for line in source.lines() {
            let mut from = 0;
            while let Some(i) = line[from..].find(&needle) {
                let end = from + i + needle.len();
                if !line[end..].starts_with(|c: char| c.is_ascii_digit()) {
                    out.push(line);
                    break;
                }
                from = end;
            }
        }
    }
    out
}
pub(crate) fn inverse(m: &M4) -> M4 {
    // Matrix4.invert(), as three.js r186 computes it.
    let (n11, n21, n31, n41) = (m[0], m[1], m[2], m[3]);
    let (n12, n22, n32, n42) = (m[4], m[5], m[6], m[7]);
    let (n13, n23, n33, n43) = (m[8], m[9], m[10], m[11]);
    let (n14, n24, n34, n44) = (m[12], m[13], m[14], m[15]);
    let t1 = n11 * n22 - n21 * n12;
    let t2 = n11 * n32 - n31 * n12;
    let t3 = n11 * n42 - n41 * n12;
    let t4 = n21 * n32 - n31 * n22;
    let t5 = n21 * n42 - n41 * n22;
    let t6 = n31 * n42 - n41 * n32;
    let t7 = n13 * n24 - n23 * n14;
    let t8 = n13 * n34 - n33 * n14;
    let t9 = n13 * n44 - n43 * n14;
    let t10 = n23 * n34 - n33 * n24;
    let t11 = n23 * n44 - n43 * n24;
    let t12 = n33 * n44 - n43 * n34;
    let det = t1 * t12 - t2 * t11 + t3 * t10 + t4 * t9 - t5 * t8 + t6 * t7;
    if det == 0. {
        return [0.; 16];
    }
    let d = 1. / det;
    [
        (n22 * t12 - n32 * t11 + n42 * t10) * d,
        (n31 * t11 - n21 * t12 - n41 * t10) * d,
        (n24 * t6 - n34 * t5 + n44 * t4) * d,
        (n33 * t5 - n23 * t6 - n43 * t4) * d,
        (n32 * t9 - n12 * t12 - n42 * t8) * d,
        (n11 * t12 - n31 * t9 + n41 * t8) * d,
        (n34 * t3 - n14 * t6 - n44 * t2) * d,
        (n13 * t6 - n33 * t3 + n43 * t2) * d,
        (n12 * t11 - n22 * t9 + n42 * t7) * d,
        (n21 * t9 - n11 * t11 - n41 * t7) * d,
        (n14 * t5 - n24 * t3 + n44 * t1) * d,
        (n23 * t3 - n13 * t5 - n43 * t1) * d,
        (n22 * t8 - n12 * t10 - n32 * t7) * d,
        (n11 * t10 - n21 * t8 + n31 * t7) * d,
        (n24 * t2 - n14 * t4 - n34 * t1) * d,
        (n13 * t4 - n23 * t2 + n33 * t1) * d,
    ]
}
/// The normal matrix's columns: the model's inverse transpose ( 3 × 3 ).
fn normal_matrix(m: &M4) -> Vec<f64> {
    let i = inverse(m);
    vec![i[0], i[4], i[8], i[1], i[5], i[9], i[2], i[6], i[10]]
}
/// What an object struct field holds.
#[derive(Clone, Copy, Debug)]
pub(super) enum ObjectSlot {
    Model,
    ModelInverse,
    NormalMatrix,
    Identity,
    Seed,
    One,
    Zero3,
    Metalness,
    Roughness,
    GridMax,
    GridMin,
    GridResolution,
    GridIntensity,
    LodMax,
    TexelX,
    TexelY,
    Environment,
    /// The material's n-th own uniform.
    Custom(usize),
}
/// What a render struct field holds.
#[derive(Clone, Copy, Debug)]
pub(super) enum RenderSlot {
    Projection,
    View,
    World,
    Position,
    LightPosition,
    Zero3,
    ShadowMatrix,
    NormalBias,
    Zero,
    One,
    MapSize,
    LightColor,
    Time,
}
/// A shader pair's struct fields and what each holds.
pub(super) type Layout<T> = Vec<(String, T)>;
fn struct_fields(vs: &str, fs: &str, name: &str) -> Vec<(String, String)> {
    let f = fields(fs, name);
    if f.is_empty() { fields(vs, name) } else { f }
}
/// The object struct's fields by how the stages use them.
pub(super) fn object_layout(vs: &str, fs: &str) -> Result<Layout<ObjectSlot>, String> {
    let sources = [fs, vs];
    let mut out = vec![];
    let mut custom = 0;
    let mut grid_next = 0;
    for (name, ty) in struct_fields(vs, fs, "objectStruct") {
        let lines = uses(&sources, "object", &name);
        let any = |p: &str| {
            let p = p.replace('#', &format!("object.{name}"));
            lines.iter().any(|l| l.contains(&p))
        };
        let slot = if grid_next > 0 {
            grid_next -= 1;
            match grid_next {
                2 => ObjectSlot::GridMin,
                1 => ObjectSlot::GridResolution,
                _ => ObjectSlot::GridIntensity,
            }
        } else {
            match ty.as_str() {
                "mat3x3<f32>" => ObjectSlot::NormalMatrix,
                "mat4x4<f32>" => {
                    if any("( # * vec4<f32>( render.cameraPosition") {
                        ObjectSlot::ModelInverse
                    } else if any("getFace( ( #") {
                        ObjectSlot::Identity
                    } else {
                        ObjectSlot::Model
                    }
                }
                "u32" => ObjectSlot::Seed,
                _ => {
                    if any("DiffuseColor.w * #") {
                        ObjectSlot::One
                    } else if any("EmissiveColor = ( # *") {
                        ObjectSlot::Zero3
                    } else if any("vec3<f32>( # ) )")
                        && lines.iter().any(|l| l.contains("EmissiveColor"))
                    {
                        ObjectSlot::One
                    } else if any("Metalness = #") {
                        ObjectSlot::Metalness
                    } else if any("max( #, 0.0525 )") || any("Roughness = #") {
                        ObjectSlot::Roughness
                    } else if any("( # - object.") && ty == "vec3<f32>" {
                        grid_next = 3;
                        ObjectSlot::GridMax
                    } else if any("-2.0, # )") {
                        ObjectSlot::LodMax
                    } else if any(".x * # )") {
                        ObjectSlot::TexelX
                    } else if any(".y * # )") {
                        ObjectSlot::TexelY
                    } else if lines
                        .iter()
                        .any(|l| l.contains("radiance + (") || l.contains("iblIrradiance + ("))
                    {
                        ObjectSlot::Environment
                    } else {
                        custom += 1;
                        ObjectSlot::Custom(custom - 1)
                    }
                }
            }
        };
        out.push((name, slot));
    }
    Ok(out)
}
/// The render struct's fields by how the stages use them.
pub(super) fn render_layout(vs: &str, fs: &str) -> Result<Layout<RenderSlot>, String> {
    let sources = [fs, vs];
    let mut out = vec![];
    for (name, _) in struct_fields(vs, fs, "renderStruct") {
        let lines = uses(&sources, "render", &name);
        let any = |p: &str| {
            let p = p.replace('#', &format!("render.{name}"));
            lines.iter().any(|l| l.contains(&p))
        };
        let slot = match name.as_str() {
            "cameraProjectionMatrix" => RenderSlot::Projection,
            "cameraViewMatrix" => RenderSlot::View,
            "cameraWorldMatrix" => RenderSlot::World,
            "cameraPosition" => RenderSlot::Position,
            _ => {
                if any("( # - render.") {
                    RenderSlot::LightPosition
                } else if any("- #") {
                    RenderSlot::Zero3
                } else if any("( # * vec4<f32>( ( shadowPositionWorld") {
                    RenderSlot::ShadowMatrix
                } else if any("( normalWorld * vec3<f32>( # ) )") {
                    RenderSlot::NormalBias
                } else if any(".z + # )") {
                    RenderSlot::Zero
                } else if any("( # * ( vec2<f32>( 1.0, 1.0 ) /") {
                    RenderSlot::One
                } else if any("/ # ).x") {
                    RenderSlot::MapSize
                } else if lines.iter().any(|l| l.contains("mix( 1.0, nodeVar")) {
                    RenderSlot::One
                } else if any("0.0, 1.0 ) ) * #")
                    || any("0.0, 1.0 ) ) * ( #")
                    || any("( # * vec3<f32>( nodeVar")
                {
                    RenderSlot::LightColor
                } else if any("( # *") && lines.iter().any(|l| l.contains("sin(")) {
                    RenderSlot::Time
                } else {
                    return Err(format!("render uniform {name}"));
                }
            }
        };
        out.push((name, slot));
    }
    Ok(out)
}
/// An object struct's values.
pub(super) fn object_values(layout: &Layout<ObjectSlot>, o: &Object) -> Values {
    let grid = o.grid.unwrap_or(([0.; 3], [0.; 3], [1.; 3], 0.));
    layout
        .iter()
        .map(|(name, slot)| {
            let v = match *slot {
                ObjectSlot::Model => o.model.to_vec(),
                ObjectSlot::ModelInverse => inverse(&o.model).to_vec(),
                ObjectSlot::NormalMatrix => normal_matrix(&o.model),
                ObjectSlot::Identity => IDENTITY.to_vec(),
                ObjectSlot::Seed => vec![o.seed as f64],
                ObjectSlot::One => vec![1.],
                ObjectSlot::Zero3 => vec![0.; 3],
                ObjectSlot::Metalness => vec![o.metalness],
                ObjectSlot::Roughness => vec![o.roughness],
                ObjectSlot::GridMax => grid.0.to_vec(),
                ObjectSlot::GridMin => grid.1.to_vec(),
                ObjectSlot::GridResolution => grid.2.to_vec(),
                ObjectSlot::GridIntensity => vec![grid.3],
                ObjectSlot::LodMax => vec![8.],
                ObjectSlot::TexelX => vec![TEXEL[0]],
                ObjectSlot::TexelY => vec![TEXEL[1]],
                ObjectSlot::Environment => vec![ENVIRONMENT],
                ObjectSlot::Custom(i) => o.custom.get(i).cloned().unwrap_or_default(),
            };
            (name.clone(), v)
        })
        .collect()
}
/// A render struct's values.
pub(super) fn render_values(layout: &Layout<RenderSlot>, r: &Render) -> Values {
    layout
        .iter()
        .map(|(name, slot)| {
            let v = match *slot {
                RenderSlot::Projection => r.projection.to_vec(),
                RenderSlot::View => r.view.to_vec(),
                RenderSlot::World => r.world.to_vec(),
                RenderSlot::Position => r.world[12..15].to_vec(),
                RenderSlot::LightPosition => r.light_position.to_vec(),
                RenderSlot::Zero3 => vec![0.; 3],
                RenderSlot::ShadowMatrix => r.shadow_matrix.to_vec(),
                RenderSlot::NormalBias => vec![r.normal_bias],
                RenderSlot::Zero => vec![0.],
                RenderSlot::One => vec![1.],
                RenderSlot::MapSize => vec![r.map_size; 2],
                RenderSlot::LightColor => r.light_color.to_vec(),
                RenderSlot::Time => vec![r.time],
            };
            (name.clone(), v)
        })
        .collect()
}
