//! Procedural meshes following the pinned Three.js r186 geometry conventions.
use super::*;
use std::f64::consts::{PI, TAU};

#[derive(Default)]
struct MeshData {
    p: Vec<f32>,
    n: Vec<f32>,
    uv: Vec<f32>,
    indices: Vec<u32>,
}
impl MeshData {
    fn vertex(&mut self, p: Vector3, n: Vector3, uv: [f64; 2]) {
        self.p.extend(p.to_array().map(|v| v as f32));
        self.n.extend(n.to_array().map(|v| v as f32));
        self.uv.extend(uv.map(|v| v as f32));
    }
    fn finish(self) -> Result<BufferGeometry> {
        primitive(self.p, self.n, self.uv, self.indices)
    }
}

pub struct CircleGeometry;
impl CircleGeometry {
    pub fn build(radius: f64, segments: u32, start: f64, length: f64) -> Result<BufferGeometry> {
        let segments = segments.max(3);
        let mut m = MeshData::default();
        m.vertex(Vector3::ZERO, Vector3::Z, [0.5, 0.5]);
        for i in 0..=segments {
            let a = start + i as f64 / segments as f64 * length;
            m.vertex(
                Vector3::new(radius * a.cos(), radius * a.sin(), 0.0),
                Vector3::Z,
                [(a.cos() + 1.0) / 2.0, (a.sin() + 1.0) / 2.0],
            );
        }
        for i in 1..=segments {
            m.indices.extend([i, i + 1, 0]);
        }
        m.finish()
    }
}
pub struct RingGeometry;
impl RingGeometry {
    pub fn build(
        inner: f64,
        outer: f64,
        theta_segments: u32,
        phi_segments: u32,
        start: f64,
        length: f64,
    ) -> Result<BufferGeometry> {
        if outer == 0.0 {
            return Err(Error::Invalid("ring outer radius"));
        }
        let ts = theta_segments.max(3);
        let ps = phi_segments.max(1);
        let mut m = MeshData::default();
        let mut radius = inner;
        for j in 0..=ps {
            for i in 0..=ts {
                let a = start + i as f64 / ts as f64 * length;
                let p = Vector3::new(radius * a.cos(), radius * a.sin(), 0.0);
                m.vertex(
                    p,
                    Vector3::Z,
                    [(p.x / outer + 1.0) / 2.0, (p.y / outer + 1.0) / 2.0],
                );
                if j < ps && i < ts {
                    let a = j * (ts + 1) + i;
                    let b = a + ts + 1;
                    m.indices.extend([a, b, a + 1, b, b + 1, a + 1]);
                }
            }
            radius += (outer - inner) / ps as f64;
        }
        m.finish()
    }
}
pub struct TorusGeometry;
impl TorusGeometry {
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        radius: f64,
        tube: f64,
        radial: u32,
        tubular: u32,
        arc: f64,
        theta_start: f64,
        theta_length: f64,
    ) -> Result<BufferGeometry> {
        if radial == 0 || tubular == 0 {
            return Err(Error::Invalid("torus segments"));
        }
        let mut m = MeshData::default();
        for j in 0..=radial {
            let v = theta_start + j as f64 / radial as f64 * theta_length;
            for i in 0..=tubular {
                let u = i as f64 / tubular as f64 * arc;
                let p = Vector3::new(
                    (radius + tube * v.cos()) * u.cos(),
                    (radius + tube * v.cos()) * u.sin(),
                    tube * v.sin(),
                );
                let center = Vector3::new(radius * u.cos(), radius * u.sin(), 0.0);
                m.vertex(
                    p,
                    (p - center).normalize_or_zero(),
                    [i as f64 / tubular as f64, j as f64 / radial as f64],
                );
                if j > 0 && i > 0 {
                    let a = (tubular + 1) * j + i - 1;
                    let b = a - tubular - 1;
                    m.indices.extend([a, b, a + 1, b, b + 1, a + 1]);
                }
            }
        }
        m.finish()
    }
}
pub struct TorusKnotGeometry;
impl TorusKnotGeometry {
    pub fn build(
        radius: f64,
        tube: f64,
        tubular: u32,
        radial: u32,
        p: u32,
        q: u32,
    ) -> Result<BufferGeometry> {
        if tubular == 0 || radial == 0 || p == 0 {
            return Err(Error::Invalid("torus knot segments/winding"));
        }
        let mut m = MeshData::default();
        let curve = |u: f64| {
            let qu = q as f64 / p as f64 * u;
            Vector3::new(
                radius * (2.0 + qu.cos()) * 0.5 * u.cos(),
                radius * (2.0 + qu.cos()) * 0.5 * u.sin(),
                radius * qu.sin() * 0.5,
            )
        };
        for i in 0..=tubular {
            let u = i as f64 / tubular as f64 * p as f64 * TAU;
            let a = curve(u);
            let b = curve(u + 0.01);
            let tangent = b - a;
            let binormal = tangent.cross(b + a).normalize_or_zero();
            let normal = binormal.cross(tangent).normalize_or_zero();
            for j in 0..=radial {
                let v = j as f64 / radial as f64 * TAU;
                let position = a - tube * v.cos() * normal + tube * v.sin() * binormal;
                m.vertex(
                    position,
                    (position - a).normalize_or_zero(),
                    [i as f64 / tubular as f64, j as f64 / radial as f64],
                );
                if i > 0 && j > 0 {
                    let a = (radial + 1) * (i - 1) + j - 1;
                    let b = (radial + 1) * i + j - 1;
                    m.indices.extend([a, b, a + 1, b, b + 1, a + 1]);
                }
            }
        }
        m.finish()
    }
}
pub struct LatheGeometry;
impl LatheGeometry {
    pub fn build(
        points: &[Vector2],
        segments: u32,
        start: f64,
        length: f64,
    ) -> Result<BufferGeometry> {
        if points.len() < 2 || segments == 0 {
            return Err(Error::Invalid("lathe points/segments"));
        }
        let length = length.clamp(0.0, TAU);
        let mut m = MeshData::default();
        let mut normals = Vec::new();
        let mut previous = Vector3::ZERO;
        for j in 0..points.len() {
            if j == points.len() - 1 {
                normals.push(previous);
                break;
            }
            let d = points[j + 1] - points[j];
            let current = Vector3::new(d.y, -d.x, 0.0);
            normals.push((current + previous).normalize_or_zero());
            previous = current;
        }
        let count = points.len() as u32;
        for i in 0..=segments {
            let a = start + i as f64 / segments as f64 * length;
            for (j, p) in points.iter().enumerate() {
                let n = normals[j];
                m.vertex(
                    Vector3::new(p.x * a.sin(), p.y, p.x * a.cos()),
                    Vector3::new(n.x * a.sin(), n.y, n.x * a.cos()),
                    [
                        i as f64 / segments as f64,
                        j as f64 / (points.len() - 1) as f64,
                    ],
                );
                if i < segments && j + 1 < points.len() {
                    let a = i * count + j as u32;
                    let b = a + count;
                    m.indices.extend([a, b, a + 1, b + 1, a + 1, b]);
                }
            }
        }
        m.finish()
    }
}
pub struct CylinderGeometry;
impl CylinderGeometry {
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        top: f64,
        bottom: f64,
        height: f64,
        radial: u32,
        vertical: u32,
        open: bool,
        start: f64,
        length: f64,
    ) -> Result<BufferGeometry> {
        if radial == 0 || vertical == 0 || height == 0.0 {
            return Err(Error::Invalid("cylinder dimensions/segments"));
        }
        let mut m = MeshData::default();
        let mut groups = Vec::new();
        let slope = (bottom - top) / height;
        for y in 0..=vertical {
            let v = y as f64 / vertical as f64;
            let radius = v * (bottom - top) + top;
            for x in 0..=radial {
                let u = x as f64 / radial as f64;
                let a = start + u * length;
                m.vertex(
                    Vector3::new(
                        radius * a.sin(),
                        -v * height + height / 2.0,
                        radius * a.cos(),
                    ),
                    Vector3::new(a.sin(), slope, a.cos()).normalize_or_zero(),
                    [u, 1.0 - v],
                );
            }
        }
        for x in 0..radial {
            for y in 0..vertical {
                let a = y * (radial + 1) + x;
                let b = a + radial + 1;
                if top > 0.0 || y != 0 {
                    m.indices.extend([a, b, a + 1]);
                }
                if bottom > 0.0 || y != vertical - 1 {
                    m.indices.extend([b, b + 1, a + 1]);
                }
            }
        }
        groups.push(Group {
            start: 0,
            count: m.indices.len(),
            material_index: 0,
        });
        if !open {
            for (radius, sign, material_index) in [(top, 1.0, 1), (bottom, -1.0, 2)] {
                if radius <= 0.0 {
                    continue;
                }
                let first = (m.p.len() / 3) as u32;
                for _ in 0..radial {
                    m.vertex(
                        Vector3::new(0.0, height / 2.0 * sign, 0.0),
                        Vector3::new(0.0, sign, 0.0),
                        [0.5, 0.5],
                    );
                }
                let rim = (m.p.len() / 3) as u32;
                for x in 0..=radial {
                    let a = start + x as f64 / radial as f64 * length;
                    m.vertex(
                        Vector3::new(radius * a.sin(), height / 2.0 * sign, radius * a.cos()),
                        Vector3::new(0.0, sign, 0.0),
                        [a.cos() * 0.5 + 0.5, a.sin() * 0.5 * sign + 0.5],
                    );
                }
                let begin = m.indices.len();
                for x in 0..radial {
                    if sign > 0.0 {
                        m.indices.extend([rim + x, rim + x + 1, first + x]);
                    } else {
                        m.indices.extend([rim + x + 1, rim + x, first + x]);
                    }
                }
                groups.push(Group {
                    start: begin,
                    count: m.indices.len() - begin,
                    material_index,
                });
            }
        }
        let mut geometry = m.finish()?;
        geometry.groups = groups;
        Ok(geometry)
    }
}

pub struct ParametricGeometry;
impl ParametricGeometry {
    pub fn build(
        function: impl Fn(f64, f64) -> Vector3,
        slices: u32,
        stacks: u32,
    ) -> Result<BufferGeometry> {
        if slices == 0 || stacks == 0 {
            return Err(Error::Invalid("parametric segments"));
        }
        let mut m = MeshData::default();
        let epsilon = 0.00001;
        for i in 0..=stacks {
            let v = i as f64 / stacks as f64;
            for j in 0..=slices {
                let u = j as f64 / slices as f64;
                let p = function(u, v);
                let pu = if u - epsilon >= 0.0 {
                    p - function(u - epsilon, v)
                } else {
                    function(u + epsilon, v) - p
                };
                let pv = if v - epsilon >= 0.0 {
                    p - function(u, v - epsilon)
                } else {
                    function(u, v + epsilon) - p
                };
                m.vertex(p, pu.cross(pv).normalize_or_zero(), [u, v]);
                if i < stacks && j < slices {
                    let a = i * (slices + 1) + j;
                    let b = a + 1;
                    let d = a + slices + 1;
                    m.indices.extend([a, b, d, b, d + 1, d]);
                }
            }
        }
        m.finish()
    }
}

pub struct CapsuleGeometry;
impl CapsuleGeometry {
    pub fn build(
        radius: f64,
        height: f64,
        cap_segments: u32,
        radial_segments: u32,
        height_segments: u32,
    ) -> Result<BufferGeometry> {
        let cap = cap_segments.max(1);
        let radial = radial_segments.max(3);
        let vertical = height_segments.max(1);
        let rows = cap * 2 + vertical;
        if radius <= 0.0 || height < 0.0 {
            return Err(Error::Invalid("capsule dimensions"));
        }
        let mut m = MeshData::default();
        let cap_arc = PI * radius / 2.0;
        let total = 2.0 * cap_arc + height;
        for y in 0..=rows {
            let (py, pr, ny, arc) = if y <= cap {
                let f = y as f64 / cap as f64;
                let a = f * PI / 2.0;
                (
                    -height / 2.0 - radius * a.cos(),
                    radius * a.sin(),
                    -radius * a.cos(),
                    f * cap_arc,
                )
            } else if y <= cap + vertical {
                let f = (y - cap) as f64 / vertical as f64;
                (
                    -height / 2.0 + f * height,
                    radius,
                    0.0,
                    cap_arc + f * height,
                )
            } else {
                let f = (y - cap - vertical) as f64 / cap as f64;
                let a = f * PI / 2.0;
                (
                    height / 2.0 + radius * a.sin(),
                    radius * a.cos(),
                    radius * a.sin(),
                    cap_arc + height + f * cap_arc,
                )
            };
            let offset = if y == 0 {
                0.5 / radial as f64
            } else if y == rows {
                -0.5 / radial as f64
            } else {
                0.0
            };
            for x in 0..=radial {
                let u = x as f64 / radial as f64;
                let a = u * TAU;
                m.vertex(
                    Vector3::new(-pr * a.cos(), py, pr * a.sin()),
                    Vector3::new(-pr * a.cos(), ny, pr * a.sin()).normalize_or_zero(),
                    [u + offset, (arc / total).clamp(0.0, 1.0)],
                );
                if y > 0 && x < radial {
                    let a = (y - 1) * (radial + 1) + x;
                    let c = a + radial + 1;
                    m.indices.extend([a, a + 1, c, a + 1, c + 1, c]);
                }
            }
        }
        m.finish()
    }
}

pub struct PolyhedronGeometry;
impl PolyhedronGeometry {
    pub fn build(
        vertices: &[Vector3],
        indices: &[u32],
        radius: f64,
        detail: u32,
    ) -> Result<BufferGeometry> {
        if !indices.len().is_multiple_of(3)
            || indices.iter().any(|&i| i as usize >= vertices.len())
            || detail > 128
        {
            return Err(Error::Invalid("polyhedron topology/detail"));
        }
        let mut positions = Vec::new();
        let columns = detail as usize + 1;
        for face in indices.chunks_exact(3) {
            let [a, b, c] = [
                vertices[face[0] as usize],
                vertices[face[1] as usize],
                vertices[face[2] as usize],
            ];
            let mut grid = Vec::new();
            for i in 0..=columns {
                let aj = a.lerp(c, i as f64 / columns as f64);
                let bj = b.lerp(c, i as f64 / columns as f64);
                let rows = columns - i;
                grid.push(
                    (0..=rows)
                        .map(|j| {
                            if rows == 0 {
                                aj
                            } else {
                                aj.lerp(bj, j as f64 / rows as f64)
                            }
                        })
                        .collect::<Vec<_>>(),
                );
            }
            for i in 0..columns {
                for j in 0..2 * (columns - i) - 1 {
                    let k = j / 2;
                    let triangle = if j % 2 == 0 {
                        [grid[i][k + 1], grid[i + 1][k], grid[i][k]]
                    } else {
                        [grid[i][k + 1], grid[i + 1][k + 1], grid[i + 1][k]]
                    };
                    positions.extend(triangle.map(|v| v.normalize_or_zero() * radius));
                }
            }
        }
        let mut m = MeshData::default();
        for face in positions.chunks_exact(3) {
            let center = (face[0] + face[1] + face[2]) / 3.0;
            let azimuth = center.z.atan2(-center.x);
            let mut uv = face
                .iter()
                .map(|v| {
                    let mut u = v.z.atan2(-v.x) / TAU + 0.5;
                    if azimuth < 0.0 && u == 1.0 {
                        u -= 1.0;
                    }
                    if v.x == 0.0 && v.z == 0.0 {
                        u = azimuth / TAU + 0.5;
                    }
                    [u, 0.5 - (-v.y).atan2((v.x * v.x + v.z * v.z).sqrt()) / PI]
                })
                .collect::<Vec<_>>();
            let minimum = uv.iter().map(|v| v[0]).fold(f64::INFINITY, f64::min);
            let maximum = uv.iter().map(|v| v[0]).fold(f64::NEG_INFINITY, f64::max);
            if maximum > 0.9 && minimum < 0.1 {
                for uv in &mut uv {
                    if uv[0] < 0.2 {
                        uv[0] += 1.0;
                    }
                }
            }
            for (v, uv) in face.iter().zip(uv) {
                m.vertex(*v, *v, uv);
            }
        }
        let mut g = m.finish()?;
        g.index = None;
        if detail == 0 {
            g.compute_vertex_normals()?;
        } else {
            g.normalize_normals()?;
        }
        Ok(g)
    }
}
pub struct IcosahedronGeometry;
impl IcosahedronGeometry {
    pub fn build(radius: f64, detail: u32) -> Result<BufferGeometry> {
        let t = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let vertices = [
            [-1.0, t, 0.0],
            [1.0, t, 0.0],
            [-1.0, -t, 0.0],
            [1.0, -t, 0.0],
            [0.0, -1.0, t],
            [0.0, 1.0, t],
            [0.0, -1.0, -t],
            [0.0, 1.0, -t],
            [t, 0.0, -1.0],
            [t, 0.0, 1.0],
            [-t, 0.0, -1.0],
            [-t, 0.0, 1.0],
        ]
        .map(Vector3::from_array);
        PolyhedronGeometry::build(
            &vertices,
            &[
                0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, 1, 5, 9, 5, 11, 4, 11, 10, 2, 10,
                7, 6, 7, 1, 8, 3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, 4, 9, 5, 2, 4, 11, 6,
                2, 10, 8, 6, 7, 9, 8, 1,
            ],
            radius,
            detail,
        )
    }
}
pub struct OctahedronGeometry;
impl OctahedronGeometry {
    pub fn build(radius: f64, detail: u32) -> Result<BufferGeometry> {
        PolyhedronGeometry::build(
            &[
                Vector3::X,
                Vector3::NEG_X,
                Vector3::Y,
                Vector3::NEG_Y,
                Vector3::Z,
                Vector3::NEG_Z,
            ],
            &[
                0, 2, 4, 0, 4, 3, 0, 3, 5, 0, 5, 2, 1, 2, 5, 1, 5, 3, 1, 3, 4, 1, 4, 2,
            ],
            radius,
            detail,
        )
    }
}
pub struct TetrahedronGeometry;
impl TetrahedronGeometry {
    pub fn build(radius: f64, detail: u32) -> Result<BufferGeometry> {
        let vertices = [
            [1.0, 1.0, 1.0],
            [-1.0, -1.0, 1.0],
            [-1.0, 1.0, -1.0],
            [1.0, -1.0, -1.0],
        ]
        .map(Vector3::from_array);
        PolyhedronGeometry::build(
            &vertices,
            &[2, 1, 0, 0, 3, 2, 1, 3, 0, 2, 3, 1],
            radius,
            detail,
        )
    }
}
