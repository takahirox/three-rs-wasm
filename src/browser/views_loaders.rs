//! Multiple viewports, OBB collisions, depth-sorted custom points and the XYZ/TGA
//! loader examples from the pinned WebGL examples.
use super::gltf_viewer::{decode_texture_image, fetch};
use super::interactive_objects::Orbit;
use crate::tsl::Node;
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    geometry::*,
    material::*,
    math::*,
    raycast::Raycaster,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    tsl::{self, *},
};
use std::f64::consts::{PI, TAU};
use std::sync::Arc;
use wasm_bindgen::JsCast;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn call(name: &str, source: &str, args: &[Type], out: Type, nodes: &[Node]) -> Result<Node> {
    Ok(WgslFn::new(name, source, args, out)?.call(nodes))
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
/// `Color.setHSL( h, s, l )` components in the given (not converted) space.
fn hsl(h: f64, s: f64, l: f64) -> [f64; 3] {
    let hue = |p: f64, q: f64, t: f64| {
        let t = t.rem_euclid(1.);
        if t < 1. / 6. {
            p + (q - p) * 6. * t
        } else if t < 0.5 {
            q
        } else if t < 2. / 3. {
            p + (q - p) * 6. * (2. / 3. - t)
        } else {
            p
        }
    };
    let (s, l) = (s.clamp(0., 1.), l.clamp(0., 1.));
    let q = if l <= 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let p = 2. * l - q;
    let h = h.rem_euclid(1.);
    [hue(p, q, h + 1. / 3.), hue(p, q, h), hue(p, q, h - 1. / 3.)]
}
/// `BufferGeometryUtils.mergeVertices` for position-only geometry: first index
/// occurrence wins, keyed by the 1e-4 tolerance hash.
fn merge_vertices(g: &BufferGeometry) -> Result<Vec<Vector3>> {
    let positions = g.positions()?;
    let count = g.index.as_ref().map_or(positions.len(), |i| i.len());
    let mut seen = std::collections::HashMap::new();
    let mut out = vec![];
    for i in 0..count {
        let p = positions[g.vertex_index(i)?];
        let key = [p.x, p.y, p.z].map(|v| (v as f32 as f64 * 1e4 + 0.5).trunc() as i64);
        seen.entry(key).or_insert_with(|| {
            out.push(p);
            out.len() - 1
        });
    }
    Ok(out)
}
/// Three.js `OBB` (examples/jsm/math/OBB.js), as used by `webgl_math_obb`.
#[derive(Clone, Copy)]
struct Obb {
    center: Vector3,
    half: Vector3,
    rotation: Matrix3,
}
impl Obb {
    fn transformed(half: Vector3, m: Matrix4) -> Self {
        let mut sx = m.x_axis.truncate().length();
        let sy = m.y_axis.truncate().length();
        let sz = m.z_axis.truncate().length();
        if m.determinant() < 0. {
            sx = -sx;
        }
        let rotation = Matrix3::from_cols(
            m.x_axis.truncate() / sx,
            m.y_axis.truncate() / sy,
            m.z_axis.truncate() / sz,
        );
        Self {
            center: m.w_axis.truncate(),
            half: half * Vector3::new(sx, sy, sz),
            rotation,
        }
    }
    fn intersects(&self, o: &Obb) -> bool {
        let e = f64::EPSILON;
        let (a, b) = (self, o);
        let au = [a.rotation.x_axis, a.rotation.y_axis, a.rotation.z_axis];
        let bu = [b.rotation.x_axis, b.rotation.y_axis, b.rotation.z_axis];
        let (ae, be) = (a.half.to_array(), b.half.to_array());
        let r: [[f64; 3]; 3] = std::array::from_fn(|i| std::array::from_fn(|j| au[i].dot(bu[j])));
        let v = b.center - a.center;
        let t = [v.dot(au[0]), v.dot(au[1]), v.dot(au[2])];
        let ar: [[f64; 3]; 3] = std::array::from_fn(|i| std::array::from_fn(|j| r[i][j].abs() + e));
        for i in 0..3 {
            let rb = be[0] * ar[i][0] + be[1] * ar[i][1] + be[2] * ar[i][2];
            if t[i].abs() > ae[i] + rb {
                return false;
            }
        }
        for i in 0..3 {
            let ra = ae[0] * ar[0][i] + ae[1] * ar[1][i] + ae[2] * ar[2][i];
            if (t[0] * r[0][i] + t[1] * r[1][i] + t[2] * r[2][i]).abs() > ra + be[i] {
                return false;
            }
        }
        // The nine cross-product axes, in the original's order.
        let tests = [
            (
                ae[1] * ar[2][0] + ae[2] * ar[1][0],
                be[1] * ar[0][2] + be[2] * ar[0][1],
                t[2] * r[1][0] - t[1] * r[2][0],
            ),
            (
                ae[1] * ar[2][1] + ae[2] * ar[1][1],
                be[0] * ar[0][2] + be[2] * ar[0][0],
                t[2] * r[1][1] - t[1] * r[2][1],
            ),
            (
                ae[1] * ar[2][2] + ae[2] * ar[1][2],
                be[0] * ar[0][1] + be[1] * ar[0][0],
                t[2] * r[1][2] - t[1] * r[2][2],
            ),
            (
                ae[0] * ar[2][0] + ae[2] * ar[0][0],
                be[1] * ar[1][2] + be[2] * ar[1][1],
                t[0] * r[2][0] - t[2] * r[0][0],
            ),
            (
                ae[0] * ar[2][1] + ae[2] * ar[0][1],
                be[0] * ar[1][2] + be[2] * ar[1][0],
                t[0] * r[2][1] - t[2] * r[0][1],
            ),
            (
                ae[0] * ar[2][2] + ae[2] * ar[0][2],
                be[0] * ar[1][1] + be[1] * ar[1][0],
                t[0] * r[2][2] - t[2] * r[0][2],
            ),
            (
                ae[0] * ar[1][0] + ae[1] * ar[0][0],
                be[1] * ar[2][2] + be[2] * ar[2][1],
                t[1] * r[0][0] - t[0] * r[1][0],
            ),
            (
                ae[0] * ar[1][1] + ae[1] * ar[0][1],
                be[0] * ar[2][2] + be[2] * ar[2][0],
                t[1] * r[0][1] - t[0] * r[1][1],
            ),
            (
                ae[0] * ar[1][2] + ae[1] * ar[0][2],
                be[0] * ar[2][1] + be[1] * ar[2][0],
                t[1] * r[0][2] - t[0] * r[1][2],
            ),
        ];
        tests.iter().all(|&(ra, rb, d)| d.abs() <= ra + rb)
    }
    /// `intersectRay`: the ray in the box's local frame against its AABB.
    fn intersect_ray(&self, ray: Ray) -> Option<Vector3> {
        let m = Matrix4::from_mat3(self.rotation) * Matrix4::IDENTITY;
        let m = Matrix4::from_cols(m.x_axis, m.y_axis, m.z_axis, self.center.extend(1.));
        let inverse = m.inverse();
        let origin = inverse.transform_point3(ray.origin);
        let direction = (inverse.transform_point3(ray.origin + ray.direction) - origin).normalize();
        Ray { origin, direction }
            .intersect_box(Box3 {
                min: -self.half,
                max: self.half,
            })
            .map(|p| m.transform_point3(p))
    }
}
/// `TGALoader.parse`: indexed, true-color and grey images, raw or RLE, returned as
/// top-down RGBA rows (the texture then uses flipY, as TGALoader's does).
fn tga_rgba(data: &[u8]) -> Result<(u32, u32, Vec<u8>)> {
    let bad = |m: &'static str| Error::Asset(format!("TGA: {m}"));
    if data.len() < 18 {
        return Err(bad("header"));
    }
    let u16_at = |i: usize| u16::from_le_bytes([data[i], data[i + 1]]) as usize;
    let (id_length, colormap_type, image_type) = (data[0] as usize, data[1], data[2]);
    let (colormap_length, colormap_size) = (u16_at(5), data[7]);
    let (width, height, pixel_size, flags) = (u16_at(12), u16_at(14), data[16], data[17]);
    let (rle, pal, grey) = match image_type {
        1 => (false, true, false),
        2 => (false, false, false),
        3 => (false, false, true),
        9 => (true, true, false),
        10 => (true, false, false),
        11 => (true, false, true),
        _ => return Err(bad("type")),
    };
    if pal && (colormap_length > 256 || colormap_size != 24 || colormap_type != 1)
        || !pal && colormap_type != 0
    {
        return Err(bad("colormap"));
    }
    if width == 0 || height == 0 || ![8, 16, 24, 32].contains(&pixel_size) {
        return Err(bad("size"));
    }
    let bytes = (pixel_size >> 3) as usize;
    let mut offset = 18 + id_length;
    let palette = if pal {
        let end = offset + colormap_length * 3;
        let p = data.get(offset..end).ok_or(bad("palette"))?.to_vec();
        offset = end;
        p
    } else {
        vec![]
    };
    let total = width * height * bytes;
    let pixels = if rle {
        let mut out = vec![0u8; total];
        let mut shift = 0;
        while shift < total {
            let c = *data.get(offset).ok_or(bad("rle"))?;
            offset += 1;
            let count = (c & 0x7f) as usize + 1;
            if c & 0x80 != 0 {
                let px = data.get(offset..offset + bytes).ok_or(bad("rle"))?.to_vec();
                offset += bytes;
                for i in 0..count {
                    let at = shift + i * bytes;
                    if at + bytes <= total {
                        out[at..at + bytes].copy_from_slice(&px);
                    }
                }
                shift += bytes * count;
            } else {
                let n = (count * bytes).min(total - shift);
                out[shift..shift + n]
                    .copy_from_slice(data.get(offset..offset + n).ok_or(bad("rle"))?);
                offset += count * bytes;
                shift += count * bytes;
            }
        }
        out
    } else {
        let n = if pal { width * height } else { total };
        data.get(offset..offset + n).ok_or(bad("pixels"))?.to_vec()
    };
    let origin = (flags & 0x30) >> 4;
    let (flip_x, flip_y) = (origin == 1 || origin == 3, origin == 0 || origin == 1);
    let mut rgba = vec![0u8; width * height * 4];
    let mut i = 0;
    for row in 0..height {
        let y = if flip_y { height - 1 - row } else { row };
        for col in 0..width {
            let x = if flip_x { width - 1 - col } else { col };
            let o = (x + width * y) * 4;
            let px: [u8; 4] = match (grey, pixel_size) {
                (true, 8) => [pixels[i], pixels[i], pixels[i], 255],
                (true, 16) => [pixels[i], pixels[i], pixels[i], pixels[i + 1]],
                (false, 8) => {
                    let c = pixels[i] as usize * 3;
                    [palette[c + 2], palette[c + 1], palette[c], 255]
                }
                (false, 16) => {
                    let c = pixels[i] as u16 + ((pixels[i + 1] as u16) << 8);
                    [
                        ((c & 0x7c00) >> 7) as u8,
                        ((c & 0x03e0) >> 2) as u8,
                        ((c & 0x001f) << 3) as u8,
                        if c & 0x8000 != 0 { 0 } else { 255 },
                    ]
                }
                (false, 24) => [pixels[i + 2], pixels[i + 1], pixels[i], 255],
                (false, 32) => [pixels[i + 2], pixels[i + 1], pixels[i], pixels[i + 3]],
                _ => return Err(bad("format")),
            };
            rgba[o..o + 4].copy_from_slice(&px);
            i += if pal { 1 } else { bytes };
        }
    }
    Ok((width as u32, height as u32, rgba))
}
/// Three.js's column-major Matrix4 arithmetic, operation for operation, so the
/// CPU depth sort breaks ties on exactly the same rounded values.
mod three {
    pub type M = [f64; 16];
    pub fn multiply(a: &M, b: &M) -> M {
        let mut t = [0.; 16];
        for r in 0..4 {
            for c in 0..4 {
                t[c * 4 + r] = a[r] * b[c * 4]
                    + a[4 + r] * b[c * 4 + 1]
                    + a[8 + r] * b[c * 4 + 2]
                    + a[12 + r] * b[c * 4 + 3];
            }
        }
        t
    }
    /// `Quaternion.setFromEuler` (XYZ) followed by `Matrix4.compose` at the origin.
    pub fn rotation(x: f64, y: f64, z: f64) -> M {
        let (c1, c2, c3) = ((x / 2.).cos(), (y / 2.).cos(), (z / 2.).cos());
        let (s1, s2, s3) = ((x / 2.).sin(), (y / 2.).sin(), (z / 2.).sin());
        let qx = s1 * c2 * c3 + c1 * s2 * s3;
        let qy = c1 * s2 * c3 - s1 * c2 * s3;
        let qz = c1 * c2 * s3 + s1 * s2 * c3;
        let qw = c1 * c2 * c3 - s1 * s2 * s3;
        let (x2, y2, z2) = (qx + qx, qy + qy, qz + qz);
        let (xx, xy, xz) = (qx * x2, qx * y2, qx * z2);
        let (yy, yz, zz) = (qy * y2, qy * z2, qz * z2);
        let (wx, wy, wz) = (qw * x2, qw * y2, qw * z2);
        [
            1. - (yy + zz),
            xy + wz,
            xz - wy,
            0.,
            xy - wz,
            1. - (xx + zz),
            yz + wx,
            0.,
            xz + wy,
            yz - wx,
            1. - (xx + yy),
            0.,
            0.,
            0.,
            0.,
            1.,
        ]
    }
    /// `PerspectiveCamera.updateProjectionMatrix` without zoom, view or film offset.
    pub fn perspective(fov: f64, aspect: f64, near: f64, far: f64) -> M {
        let top = near * (std::f64::consts::PI / 180. * 0.5 * fov).tan();
        let height = 2. * top;
        let width = aspect * height;
        let left = -0.5 * width;
        let (right, bottom) = (left + width, top - height);
        let x = 2. * near / (right - left);
        let y = 2. * near / (top - bottom);
        let a = (right + left) / (right - left);
        let b = (top + bottom) / (top - bottom);
        let c = -(far + near) / (far - near);
        let d = -2. * far * near / (far - near);
        [x, 0., 0., 0., 0., y, 0., 0., a, b, c, -1., 0., 0., d, 0.]
    }
    /// `Vector3.applyMatrix4`'s z, with its reciprocal-w multiply.
    pub fn projected_z(e: &M, x: f64, y: f64, z: f64) -> f64 {
        let w = 1. / (e[3] * x + e[7] * y + e[11] * z + e[15]);
        (e[2] * x + e[6] * y + e[10] * z + e[14]) * w
    }
}
const FULLSCREEN: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.xy*2.0,1.0,1.0);return out;}";
/// A viewport-filling triangle: WebGL's scissored clear color for one view.
async fn clear_triangle(s: &mut Scene, r: &Renderer) -> Result<Object3D> {
    let graph = NodeMaterial::new(vec4(uniform(0, Type::Vec3), float(1.)));
    let mut m = ShaderMaterial::new(Arc::new(
        ShaderProgram::with_projection(r, &graph.wgsl(0)?, &[], &[], FULLSCREEN).await?,
    ));
    m.properties.depth_test = false;
    m.properties.depth_write = false;
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        vec3s(vec![-0.5, -0.5, 0., 1.5, -0.5, 0., -0.5, 1.5, 0.])?,
    );
    g.set_attribute("normal", vec3s(vec![0., 0., 1., 0., 0., 1., 0., 0., 1.])?);
    g.set_attribute(
        "uv",
        Attribute::F32(BufferAttribute::new(vec![0.; 6], 2, false)?),
    );
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(g),
        Arc::new(Material::Shader(m)),
    )));
    s.get_mut(h)?.frustum_culled = false;
    s.get_mut(h)?.render_order = -10000;
    Ok(h)
}
struct View {
    camera: Object3D,
    rect: [f64; 4],
    background: Color,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    pointer: Vector2,
    orbit: Orbit,
    objects: Vec<Object3D>,
    views: Vec<View>,
    clear: Option<Object3D>,
    output: Option<RenderTarget>,
    angles: Vec<Vector3>,
    hitbox: Option<Object3D>,
    points: Option<Points2>,
}
struct Points2 {
    positions: Vec<Vector3>,
    /// sphere.matrixWorld as of the previous render (identity before the first).
    world: three::M,
    sphere: usize,
    sizes: Vec<f32>,
    size_buffer: GpuBuffer,
    order_buffer: GpuBuffer,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            198 => (30., 1., 10000., Vector3::new(0., 300., 1800.)),
            199 => (70., 1., 1000., Vector3::new(0., 0., 75.)),
            200 => (45., 1., 10000., Vector3::new(0., 0., 300.)),
            201 => (50., 0.1, 100., Vector3::new(10., 7., 10.)),
            _ => (45., 0.1, 100., Vector3::new(0., 1., 5.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = position;
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            pointer: Vector2::ZERO,
            orbit: match id {
                199 => Orbit::new(position, true, 1., (0., f64::INFINITY)),
                _ => Orbit::new(position, false, 1., (0., f64::INFINITY)),
            },
            objects: vec![],
            views: vec![],
            clear: None,
            output: None,
            angles: vec![],
            hitbox: None,
            points: None,
        };
        match id {
            198 => d.views_scene(s, c, r).await?,
            199 => {
                s.background = Color::WHITE;
                let light = s.insert(NodeKind::Light(Light::Hemisphere {
                    sky: Color::WHITE,
                    ground: Color::from_hex(0x222222),
                    intensity: 4.,
                }));
                s.get_mut(light)?.position = Vector3::ONE;
                let g = Arc::new(BoxGeometry::build(10., 5., 6.)?);
                for _ in 0..100 {
                    let mut m = MeshLambertMaterial::default();
                    m.properties.color = Color::from_hex(0x00ff00);
                    let h = s.insert(NodeKind::Mesh(Mesh::new(
                        g.clone(),
                        Arc::new(Material::Lambert(m)),
                    )));
                    let p = [0; 3].map(|_| random(&mut d.seed) * 80. - 40.);
                    let a = [0; 3].map(|_| random(&mut d.seed) * TAU);
                    let k = [0; 3].map(|_| random(&mut d.seed) + 0.5);
                    let n = s.get_mut(h)?;
                    n.position = Vector3::from_array(p);
                    n.scale = Vector3::from_array(k);
                    d.angles.push(Vector3::from_array(a));
                    d.objects.push(h);
                }
                let mut m = MeshBasicMaterial::default();
                m.properties.color = Color::BLACK;
                m.properties.wireframe = true;
                let hitbox = s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(Material::Basic(m)))));
                s.get_mut(hitbox)?.visible = false;
                d.hitbox = Some(hitbox);
            }
            200 => d.points_scene(s, r).await?,
            201 => {
                let text = String::from_utf8(fetch("/web/gallery/assets/helix_201.xyz").await?)
                    .map_err(|e| Error::Asset(e.to_string()))?;
                // XYZLoader: three values per vertex, six with an sRGB byte color.
                let (mut p, mut colors) = (vec![], vec![]);
                for line in text.split('\n') {
                    let line = line.trim();
                    if line.starts_with('#') {
                        continue;
                    }
                    let v: Vec<&str> = line.split_whitespace().collect();
                    let num = |s: &str| s.parse::<f64>().unwrap_or(f64::NAN);
                    if v.len() == 3 || v.len() == 6 {
                        p.extend(v[..3].iter().map(|x| num(x) as f32));
                    }
                    if v.len() == 6 {
                        colors.extend(
                            v[3..]
                                .iter()
                                .map(|x| crate::math::srgb_to_linear(num(x) / 255.) as f32),
                        );
                    }
                }
                let mut g = BufferGeometry::default();
                g.set_attribute("position", vec3s(p)?);
                let has_colors = !colors.is_empty();
                if has_colors {
                    g.set_attribute("color", vec3s(colors)?);
                }
                g.center()?;
                let mut m = PointsMaterial {
                    size: 0.1,
                    ..Default::default()
                };
                m.properties.vertex_colors = has_colors;
                d.objects.push(s.insert(NodeKind::Points(Points {
                    geometry: Arc::new(g),
                    material: Arc::new(Material::Points(m)),
                })));
            }
            _ => {
                let g = Arc::new(BoxGeometry::build(1., 1., 1.)?);
                for (name, x) in [("crate_grey8.tga", -1.), ("crate_color8.tga", 1.)] {
                    let (w, h, rgba) =
                        tga_rgba(&fetch(&format!("/web/gallery/assets/{name}")).await?)?;
                    let mut t = crate::material::Texture::from_rgba(w, h, rgba, true)?;
                    // TGALoader: flipY, generated mipmaps, sRGB set by the example.
                    t.srgb = true;
                    t.mipmap_filter = Some(Filter::Linear);
                    let mut m = MeshPhongMaterial::default();
                    m.properties.map = Some(Arc::new(t));
                    let h = s.insert(NodeKind::Mesh(Mesh::new(
                        g.clone(),
                        Arc::new(Material::Phong(m)),
                    )));
                    s.get_mut(h)?.position.x = x;
                }
                s.insert(NodeKind::Light(Light::Ambient {
                    color: Color::WHITE,
                    intensity: 1.5,
                }));
                let light = s.insert(NodeKind::Light(Light::Directional {
                    color: Color::WHITE,
                    intensity: 2.5,
                    target: Vector3::ZERO,
                }));
                s.get_mut(light)?.position = Vector3::ONE;
            }
        }
        if id == 201 || id == 198 {
            s.look_at(c, Vector3::ZERO)?;
        } else {
            // OrbitControls' constructor update() looks at its target.
            d.orbit.apply(s, c)?;
        }
        Ok(d)
    }
    async fn views_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::Z;
        // The original's radial-gradient canvas, rasterized by the same Canvas2D.
        let fail = |e: wasm_bindgen::JsValue| Error::Asset(format!("shadow canvas: {e:?}"));
        let canvas = web_sys::OffscreenCanvas::new(128, 128).map_err(fail)?;
        let context = canvas
            .get_context("2d")
            .map_err(fail)?
            .ok_or(Error::Invalid("shadow context"))?
            .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
            .map_err(|e| fail(e.into()))?;
        let gradient = context
            .create_radial_gradient(64., 64., 0., 64., 64., 64.)
            .map_err(fail)?;
        gradient
            .add_color_stop(0.1, "rgba(0,0,0,0.15)")
            .map_err(fail)?;
        gradient.add_color_stop(1., "rgba(0,0,0,0)").map_err(fail)?;
        context.set_fill_style_canvas_gradient(&gradient);
        context.fill_rect(0., 0., 128., 128.);
        let data = context
            .get_image_data(0., 0., 128., 128.)
            .map_err(fail)?
            .data()
            .to_vec();
        let mut t = crate::material::Texture::from_rgba(128, 128, data, false)?;
        t.mipmap_filter = Some(Filter::Linear);
        let mut shadow = MeshBasicMaterial::default();
        shadow.properties.map = Some(Arc::new(t));
        shadow.properties.transparent = true;
        let (shadow, plane) = (
            Arc::new(Material::Basic(shadow)),
            Arc::new(PlaneGeometry::build(300., 300., 1, 1)?),
        );
        for x in [0., -400., 400.] {
            let h = s.insert(NodeKind::Mesh(Mesh::new(plane.clone(), shadow.clone())));
            let n = s.get_mut(h)?;
            n.position = Vector3::new(x, -250., 0.);
            n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        }
        let base = IcosahedronGeometry::build(200., 1)?;
        let p = base.positions()?;
        let mut m = MeshPhongMaterial::default();
        m.properties.flat_shading = true;
        m.properties.vertex_colors = true;
        m.shininess = 0.;
        let material = Arc::new(Material::Phong(m));
        let mut w = MeshBasicMaterial::default();
        w.properties.color = Color::BLACK;
        w.properties.wireframe = true;
        w.properties.transparent = true;
        let wire = Arc::new(Material::Basic(w));
        for (k, (x, rx)) in [(-400., -1.87), (400., 0.), (0., 0.)]
            .into_iter()
            .enumerate()
        {
            let colors = p
                .iter()
                .flat_map(|v| {
                    let y = (v.y / 200. + 1.) / 2.;
                    // setHSL / setRGB with SRGBColorSpace convert to the working space.
                    let rgb = match k {
                        0 => hsl(y, 1., 0.5),
                        1 => hsl(0., y, 0.5),
                        _ => [1., 0.8 - y, 0.],
                    };
                    rgb.map(|c| crate::math::srgb_to_linear(c) as f32)
                })
                .collect();
            let mut g = base.clone();
            g.set_attribute("color", vec3s(colors)?);
            let g = Arc::new(g);
            let h = s.insert(NodeKind::Mesh(Mesh::new(g.clone(), material.clone())));
            let wireframe = s.insert(NodeKind::Mesh(Mesh::new(g, wire.clone())));
            s.add(h, wireframe)?;
            let n = s.get_mut(h)?;
            n.position.x = x;
            n.quaternion = Quaternion::from_rotation_x(rx);
        }
        let camera = |fov: f64| {
            NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                fov,
                near: 1.,
                far: 10000.,
                ..Default::default()
            }))
        };
        for (i, (rect, rgb, eye, up, fov)) in [
            (
                [0., 0., 0.5, 1.],
                [0.5, 0.5, 0.7],
                Vector3::new(0., 300., 1800.),
                Vector3::Y,
                30.,
            ),
            (
                [0.5, 0., 0.5, 0.5],
                [0.7, 0.5, 0.5],
                Vector3::new(0., 1800., 0.),
                Vector3::Z,
                45.,
            ),
            (
                [0.5, 0.5, 0.5, 0.5],
                [0.5, 0.7, 0.7],
                Vector3::new(1400., 800., 1400.),
                Vector3::Y,
                60.,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let h = if i == 0 { c } else { s.insert(camera(fov)) };
            let n = s.get_mut(h)?;
            n.position = eye;
            n.up = up;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut n.kind {
                p.fov = fov;
            }
            self.views.push(View {
                camera: h,
                rect,
                background: Color::from_srgb(rgb[0], rgb[1], rgb[2]),
            });
        }
        self.clear = Some(clear_triangle(s, r).await?);
        Ok(())
    }
    async fn points_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let sphere = merge_vertices(&SphereGeometry::build(100., 68, 38)?)?;
        let cube = merge_vertices(&BoxGeometry::segmented(80., 80., 80., 10, 10, 10)?)?;
        let length1 = sphere.len();
        let positions: Vec<Vector3> = sphere.into_iter().chain(cube).collect();
        let n = positions.len();
        let mut records = Vec::with_capacity(n * 8);
        let mut sizes = Vec::with_capacity(n);
        for (i, v) in positions.iter().enumerate() {
            // Working-space HSL, no conversion; the raw shader outputs these values.
            let rgb = if i < length1 {
                hsl(
                    0.01 + 0.1 * (i as f64 / length1 as f64),
                    0.99,
                    (v.y + 100.) / 400.,
                )
            } else {
                hsl(0.6, 0.75, 0.25 + v.y / 200.)
            };
            records.extend([v.x as f32, v.y as f32, v.z as f32, 0.]);
            records.extend(rgb.map(|c| c as f32));
            records.push(0.);
            sizes.push(if i < length1 { 10. } else { 40. });
        }
        let attributes = GpuBuffer::new(r, bytemuck::cast_slice(&records), BufferAccess::Read)?;
        let size_buffer = GpuBuffer::new(r, bytemuck::cast_slice(&sizes), BufferAccess::Read)?;
        let order: Vec<u32> = (0..n as u32).collect();
        let order_buffer = GpuBuffer::new(r, bytemuck::cast_slice(&order), BufferAccess::Read)?;
        let mut image =
            decode_texture_image(&fetch("/web/gallery/assets/point-clouds/disc.png").await?)
                .await?;
        image.srgb = false;
        image.mipmap_filter = Some(Filter::Linear);
        let disc = r.upload_texture(&Arc::new(image))?;
        // Draw order follows the sorted index: instance i draws point order[i].
        // The vertex stage mirrors the original shader with CPU-computed f64
        // modelView/projection matrices (u.custom[0..8]), as WebGL uploads them:
        // near-tied depths then round like the original's gl_Position.
        let color = call(
            "point_color",
            "fn point_color(i:u32,t:vec4<f32>)->vec4<f32>{let j=tsl_attribute_2[i];return vec4(tsl_attribute_0[j*2u+1u].rgb,1.0)*t;}",
            &[Type::Uint, Type::Vec4],
            Type::Vec4,
            &[instance_index(), tsl::Texture::External(0).sample(uv())],
        )?;
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let j=tsl_attribute_2[surface.instance_index];let model_view=mat4x4(u.custom[0],u.custom[1],u.custom[2],u.custom[3]);let projection=mat4x4(u.custom[4],u.custom[5],u.custom[6],u.custom[7]);let mv=model_view*vec4(tsl_attribute_0[j*2u].xyz,1.0);let size=clamp(tsl_attribute_1[j]*(300.0/-mv.z),1.0,511.0);out.clip=projection*mv;out.clip.x+=position.x*size*2.0/(u.point.x*u.transmission[2].z)*out.clip.w;out.clip.y+=position.y*size*2.0/(u.point.y*u.transmission[2].w)*out.clip.w;return out;}";
        let graph = NodeMaterial::new(color);
        let source = graph.wgsl_with_storage(1, &[Type::Vec4, Type::Float, Type::Uint])?;
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(
                r,
                &source,
                &[&attributes, &size_buffer, &order_buffer],
                &[(&disc.view, &disc.sampler)],
                projection,
            )
            .await?,
        ));
        m.properties.transparent = true;
        // ShaderMaterial transparent: NormalBlending without premultiplied alpha.
        m.properties.blending = Some(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        });
        let mut g = PlaneGeometry::build(1., 1., 1, 1)?;
        g.instance_count = Some(n as u32);
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Shader(m)),
        )));
        s.get_mut(h)?.frustum_culled = false;
        self.objects.push(h);
        self.points = Some(Points2 {
            positions,
            world: three::rotation(0., 0., 0.),
            sphere: length1,
            sizes,
            size_buffer,
            order_buffer,
        });
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// One original animation frame.
    pub fn prepare(&mut self, r: &Renderer, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        // Per-frame increments of the original, as 60 fps steps of example time.
        let step = (t - self.last) * 60.;
        self.last = t;
        match self.id {
            198 => {
                let mouse = self.pointer.x;
                for (i, v) in self.views.iter().enumerate() {
                    let n = s.get_mut(v.camera)?;
                    match i {
                        0 => {
                            n.position.x = (n.position.x + mouse * 0.05 * step).clamp(-2000., 2000.)
                        }
                        1 => {
                            n.position.x = (n.position.x - mouse * 0.05 * step).clamp(-2000., 2000.)
                        }
                        _ => {
                            n.position.y = (n.position.y - mouse * 0.05 * step).clamp(-1600., 1600.)
                        }
                    }
                    let target = if i == 1 {
                        Vector3::new(n.position.x, 0., n.position.z)
                    } else {
                        Vector3::ZERO
                    };
                    s.look_at(v.camera, target)?;
                }
            }
            199 => {
                self.orbit.update();
                self.orbit.apply(s, c)?;
                // rotation.x/y += delta * PI * (0.2, 0.1) under the example Timer.
                for (h, a) in self.objects.iter().zip(&self.angles) {
                    s.get_mut(*h)?.quaternion = Euler {
                        angles: *a + Vector3::new(t * PI * 0.2, t * PI * 0.1, 0.),
                        order: EulerOrder::XYZ,
                    }
                    .quaternion();
                }
                s.update()?;
                let half = Vector3::new(5., 2.5, 3.);
                let obbs: Vec<Obb> = self
                    .objects
                    .iter()
                    .map(|h| s.get(*h).map(|n| Obb::transformed(half, n.matrix_world)))
                    .collect::<Result<_>>()?;
                let mut hit = vec![false; obbs.len()];
                for i in 0..obbs.len() {
                    for j in i + 1..obbs.len() {
                        if obbs[i].intersects(&obbs[j]) {
                            hit[i] = true;
                            hit[j] = true;
                        }
                    }
                }
                for (h, hit) in self.objects.iter().zip(hit) {
                    if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                        Arc::make_mut(&mut m.materials[0]).properties_mut().color =
                            Color::from_hex(if hit { 0xff0000 } else { 0x00ff00 });
                    }
                }
            }
            200 => {
                let mesh = self.objects[0];
                let time = t * 5.;
                s.get_mut(mesh)?.quaternion = Euler {
                    angles: Vector3::new(0., 0.02 * time, 0.02 * time),
                    order: EulerOrder::XYZ,
                }
                .quaternion();
                s.update()?;
                let p = self.points.as_mut().ok_or(Error::Invalid("points"))?;
                for i in 0..p.sphere {
                    p.sizes[i] = (16. + 12. * (0.1 * i as f64 + time).sin()) as f32;
                }
                // sortPoints(): projected depth, farthest first, stable as Array.sort.
                // It runs before render() refreshes matrixWorld, so it uses the
                // previous frame's rotation, with Three.js's own float arithmetic.
                let Camera::Perspective(camera) = s.camera(c)?.0 else {
                    return Err(Error::Invalid("points camera"));
                };
                let z = s.get(c)?.position.z;
                let projection =
                    three::perspective(camera.fov, camera.aspect, camera.near, camera.far);
                let view = [
                    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., -z, 1.,
                ];
                let matrix = three::multiply(&three::multiply(&projection, &view), &p.world);
                let mut depth: Vec<(f64, u32)> = p
                    .positions
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (three::projected_z(&matrix, v.x, v.y, v.z), i as u32))
                    .collect();
                p.world = three::rotation(0., 0.02 * time, 0.02 * time);
                let model_view = three::multiply(&view, &p.world);
                if let NodeKind::Mesh(m) = &mut s.get_mut(mesh)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    for (k, column) in model_view.chunks(4).chain(projection.chunks(4)).enumerate()
                    {
                        m.uniforms[k] = std::array::from_fn(|i| column[i] as f32);
                    }
                }
                depth.sort_by(|a, b| b.0.total_cmp(&a.0));
                let order: Vec<u32> = depth.into_iter().map(|x| x.1).collect();
                p.size_buffer.write(r, 0, bytemuck::cast_slice(&p.sizes))?;
                p.order_buffer.write(r, 0, bytemuck::cast_slice(&order))?;
            }
            201 => {
                s.get_mut(self.objects[0])?.quaternion = Euler {
                    angles: Vector3::new(t * 0.2, t * 0.5, 0.),
                    order: EulerOrder::XYZ,
                }
                .quaternion();
            }
            _ => {
                self.orbit.update();
                self.orbit.apply(s, c)?;
            }
        }
        Ok(())
    }
    /// The three viewports, each with its scissored clear color.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        _c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.id != 198 {
            return Ok(false);
        }
        if self.output.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.options.samples != out.options.samples
        }) {
            let mut options = out.options.clone();
            options.store_multisampled_color_buffer = true;
            self.output = Some(RenderTarget::with_options(
                &r.device, out.width, out.height, options,
            )?);
        }
        let target = self.output.as_mut().expect("views target");
        let window = web_sys::window().ok_or(Error::Invalid("window"))?;
        let dpr = window.device_pixel_ratio();
        let (w, h) = (out.width as f64 / dpr, out.height as f64 / dpr);
        for (i, v) in self.views.iter().enumerate() {
            // Math.floor in CSS pixels, then WebGL's round( × pixelRatio ).
            let [l, b, vw, vh] = [
                (w * v.rect[0]).floor(),
                (h * v.rect[1]).floor(),
                (w * v.rect[2]).floor(),
                (h * v.rect[3]).floor(),
            ]
            .map(|x| (x * dpr).round());
            let (x, width, height) = (l as u32, vw as u32, vh as u32);
            let y = (out.height as f64 - b - vh).max(0.) as u32;
            if width == 0 || height == 0 {
                continue;
            }
            let width = width.min(out.width - x.min(out.width));
            let height = height.min(out.height - y.min(out.height));
            target.viewport = [x, y, width, height];
            target.scissor = Some([x, y, width, height]);
            target.set_load_color(i != 0);
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(v.camera)?.kind {
                p.aspect = width as f64 / height as f64;
            }
            if let Some(q) = self.clear
                && let NodeKind::Mesh(m) = &mut s.get_mut(q)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.uniforms[0] = v.background.0.as_vec3().extend(1.).to_array();
            }
            r.render(s, v.camera, target)?;
        }
        target.scissor = None;
        target.viewport = [0, 0, out.width, out.height];
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.output.as_ref()
    }
    /// NDC pointer; the multiple-views example uses `clientX - innerWidth / 2`.
    pub fn gpu_pointer(&mut self, x: f64, y: f64) {
        let width = web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|w| w.as_f64())
            .unwrap_or(512.);
        self.pointer = if self.id == 198 {
            Vector2::new(x * width / 2., y)
        } else {
            Vector2::new(x, y)
        };
    }
    /// `webgl_math_obb` click: nearest OBB ray hit receives the wireframe hitbox.
    pub fn select(&mut self, s: &mut Scene, c: Object3D, x: f64, y: f64) -> Result<()> {
        if self.id != 199 {
            return Ok(());
        }
        let (camera, world) = s.camera(c)?;
        let mut ray = Raycaster::default();
        ray.set_from_camera(Vector2::new(x, y), camera, world)?;
        let half = Vector3::new(5., 2.5, 3.);
        let mut hits = vec![];
        for &h in &self.objects {
            let obb = Obb::transformed(half, s.get(h)?.matrix_world);
            if let Some(p) = obb.intersect_ray(ray.ray) {
                hits.push((ray.ray.origin.distance(p), h));
            }
        }
        hits.sort_by(|a, b| a.0.total_cmp(&b.0));
        let hitbox = self.hitbox.ok_or(Error::Invalid("hitbox"))?;
        if let Some(&(_, h)) = hits.first() {
            s.add(h, hitbox)?;
            s.get_mut(hitbox)?.visible = true;
        } else {
            s.remove_from_parent(hitbox)?;
            s.get_mut(hitbox)?.visible = false;
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        match self.id {
            199 => {
                if pan {
                    let Camera::Perspective(p) = s.camera(c)?.0 else {
                        return Ok(());
                    };
                    self.orbit.pan(s.get(c)?, p.fov, dx, dy, height);
                } else if wheel != 0. {
                    self.orbit.dolly(wheel);
                } else {
                    self.orbit.rotate(dx, dy, height);
                }
            }
            202 => {
                // enableZoom = false; panning stays enabled.
                if pan {
                    let Camera::Perspective(p) = s.camera(c)?.0 else {
                        return Ok(());
                    };
                    self.orbit.pan(s.get(c)?, p.fov, dx, dy, height);
                } else if wheel == 0. {
                    self.orbit.rotate(dx, dy, height);
                } else {
                    return Ok(());
                }
            }
            _ => return Ok(()),
        }
        // The controls' pointer and wheel handlers call update() themselves.
        self.orbit.update();
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
