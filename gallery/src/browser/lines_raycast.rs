//! webgpu_lines_fat_raycasting: a colored spiral of fat lines (LineSegments2
//! over LineSegmentsGeometry, or Line2 over LineGeometry) turning slowly, with
//! the pointer raycast against the displayed line each frame: a red sphere at
//! the ray's closest point and a green one on the line, colored from the hit
//! segment's start color. The GPU ribbons stay resident; the raycast is the
//! explicit CPU query of LineSegments2.raycast.
use super::controls_attributes::{CameraState, Controls, camera_state};
use crate::tsl::lines::{LineOptions, LineSegments, geometry};
use crate::{
    Error, Result, camera::*, curve::CatmullRomCurve3, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

/// One displayed line: the segments ( start, end, start color, end color ),
/// its ribbon and threshold meshes and their program variants by
/// ( worldUnits, alphaToCoverage ).
struct Fat {
    segments: Vec<[Vector3; 4]>,
    line: Object3D,
    threshold: Object3D,
    variants: Vec<Arc<crate::shader::ShaderProgram>>,
    _data: LineSegments,
}
pub(super) struct Demo {
    fats: [Fat; 2],
    inter: Object3D,
    on_line: Object3D,
    controls: Controls,
    pointer: Vector2,
    /// line type, world units, visualize threshold, width, alphaToCoverage,
    /// threshold, translation, animate.
    params: [f64; 8],
    rotation: f64,
    /// The rotation and translation of the previous render: the threshold
    /// line copies the line's transform before this frame's turn, and the
    /// raycast reads the matrixWorld that render left.
    last: (f64, f64),
    /// The drag's last pointer position and button.
    drag: Option<(Vector2, u32)>,
}
/// setHSL( h, s, l ) in a color's own space.
fn hsl_components(h: f64, s: f64, l: f64) -> Vector3 {
    Color::from_hsl(h, s, l).0
}
/// Color.getHSL() in the working (linear) space.
fn get_hsl(c: Vector3) -> (f64, f64, f64) {
    let (r, g, b) = (c.x, c.y, c.z);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let lightness = (min + max) / 2.;
    if min == max {
        return (0., 0., lightness);
    }
    let delta = max - min;
    let saturation = if lightness <= 0.5 {
        delta / (max + min)
    } else {
        delta / (2. - max - min)
    };
    let hue = if max == r {
        (g - b) / delta + if g < b { 6. } else { 0. }
    } else if max == g {
        (b - r) / delta + 2.
    } else {
        (r - g) / delta + 4.
    };
    (hue / 6., saturation, lightness)
}
/// Ray.distanceSqToSegment( v0, v1, pointOnRay, pointOnSegment ).
fn distance_sq_to_segment(ray: Ray, v0: Vector3, v1: Vector3) -> (f64, Vector3, Vector3) {
    let seg_center = (v0 + v1) * 0.5;
    let seg_dir = (v1 - v0).normalize();
    let diff = ray.origin - seg_center;
    let seg_extent = v0.distance(v1) * 0.5;
    let a01 = -ray.direction.dot(seg_dir);
    let b0 = diff.dot(ray.direction);
    let b1 = -diff.dot(seg_dir);
    let c = diff.length_squared();
    let det = (1. - a01 * a01).abs();
    let (s0, s1, sqr_dist);
    if det > 0. {
        let mut t0 = a01 * b1 - b0;
        let mut t1 = a01 * b0 - b1;
        let ext_det = seg_extent * det;
        if t0 >= 0. {
            if t1 >= -ext_det {
                if t1 <= ext_det {
                    let inv_det = 1. / det;
                    t0 *= inv_det;
                    t1 *= inv_det;
                    s0 = t0;
                    s1 = t1;
                    sqr_dist = t0 * (t0 + a01 * t1 + 2. * b0) + t1 * (a01 * t0 + t1 + 2. * b1) + c;
                } else {
                    s1 = seg_extent;
                    s0 = (-(a01 * s1 + b0)).max(0.);
                    sqr_dist = -s0 * s0 + s1 * (s1 + 2. * b1) + c;
                }
            } else {
                s1 = -seg_extent;
                s0 = (-(a01 * s1 + b0)).max(0.);
                sqr_dist = -s0 * s0 + s1 * (s1 + 2. * b1) + c;
            }
        } else if t1 <= -ext_det {
            let s0a = (-(-a01 * seg_extent + b0)).max(0.);
            s1 = if s0a > 0. {
                -seg_extent
            } else {
                (-b1).clamp(-seg_extent, seg_extent)
            };
            s0 = s0a;
            sqr_dist = -s0 * s0 + s1 * (s1 + 2. * b1) + c;
        } else if t1 <= ext_det {
            s0 = 0.;
            s1 = (-b1).clamp(-seg_extent, seg_extent);
            sqr_dist = s1 * (s1 + 2. * b1) + c;
        } else {
            let s0a = (-(a01 * seg_extent + b0)).max(0.);
            s1 = if s0a > 0. {
                seg_extent
            } else {
                (-b1).clamp(-seg_extent, seg_extent)
            };
            s0 = s0a;
            sqr_dist = -s0 * s0 + s1 * (s1 + 2. * b1) + c;
        }
    } else {
        s1 = if a01 > 0. { -seg_extent } else { seg_extent };
        s0 = (-(a01 * s1 + b0)).max(0.);
        sqr_dist = -s0 * s0 + s1 * (s1 + 2. * b1) + c;
    }
    (
        sqr_dist,
        ray.origin + ray.direction * s0,
        seg_center + seg_dir * s1,
    )
}
fn f32v(v: Vector3) -> Vector3 {
    v.as_vec3().as_dvec3()
}
impl Fat {
    async fn new(s: &mut Scene, r: &Renderer, segments: Vec<[Vector3; 4]>) -> Result<Self> {
        // computeLineDistances(): Float32 cumulative distances.
        let mut distance = 0f32;
        let data: Vec<[[f32; 4]; 4]> = segments
            .iter()
            .map(|[a, b, ca, cb]| {
                let start = distance;
                distance = (start as f64 + a.distance(*b)) as f32;
                [
                    [a.x as f32, a.y as f32, a.z as f32, start],
                    [b.x as f32, b.y as f32, b.z as f32, distance],
                    [ca.x as f32, ca.y as f32, ca.z as f32, 1.],
                    [cb.x as f32, cb.y as f32, cb.z as f32, 1.],
                ]
            })
            .collect();
        let buffer = LineSegments::new(r, &data)?;
        let mut variants = vec![];
        for mask in 0..4 {
            variants.push(
                buffer
                    .material(
                        r,
                        LineOptions {
                            world_units: mask & 1 != 0,
                            dashed: false,
                            alpha_to_coverage: mask & 2 != 0,
                        },
                    )
                    .await?
                    .program,
            );
        }
        let g = Arc::new(geometry(segments.len() as u32)?);
        let insert = |s: &mut Scene, material: ShaderMaterial| -> Result<Object3D> {
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                g.clone(),
                Arc::new(Material::Shader(material)),
            )));
            s.get_mut(h)?.frustum_culled = false;
            Ok(h)
        };
        let line = insert(s, ShaderMaterial::new(variants[3].clone()))?;
        // Line2NodeMaterial( { color: 0xffffff, transparent, opacity: 0.2,
        // depthTest: false } ) without vertex colors.
        let mut m = ShaderMaterial::new(variants[3].clone());
        m.properties.transparent = true;
        m.properties.opacity = 0.2;
        m.properties.depth_test = false;
        let threshold = insert(s, m)?;
        Ok(Self {
            segments,
            line,
            threshold,
            variants,
            _data: buffer,
        })
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.background = Color::BLACK;
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            aspect,
            near: 1.,
            far: 1000.,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-40., 0., 60.);
        let mut controls = Controls::new(None, (10., 500.), PI, true);
        controls.update(s, c)?;
        let points: Vec<Vector3> = (-50..50)
            .map(|i| {
                let t = i as f64 / 3.;
                Vector3::new(t * (2. * t).sin(), t, t * (2. * t).cos())
            })
            .collect();
        let divisions = 3 * points.len();
        let spline = CatmullRomCurve3::new(points);
        let mut positions = vec![];
        let mut colors = vec![];
        for i in 0..divisions {
            let t = i as f64 / divisions as f64;
            positions.push(f32v(spline.point(t)?));
            // setHSL( t, 1.0, 0.5, SRGBColorSpace ) in the linear working space.
            colors.push(f32v(hsl_components(t, 1., 0.5).map(srgb_to_linear)));
        }
        // LineSegmentsGeometry pairs the points; LineGeometry chains them.
        let pairs = (0..divisions / 2)
            .map(|i| {
                [
                    positions[2 * i],
                    positions[2 * i + 1],
                    colors[2 * i],
                    colors[2 * i + 1],
                ]
            })
            .collect();
        let chain = (0..divisions - 1)
            .map(|i| [positions[i], positions[i + 1], colors[i], colors[i + 1]])
            .collect();
        let chain = Fat::new(s, r, chain).await?;
        let pairs = Fat::new(s, r, pairs).await?;
        let sphere = Arc::new(SphereGeometry::build(0.25, 8, 4)?);
        let spheres = [0xff0000, 0x00ff00].map(|hex| {
            let mut m = MeshBasicMaterial::default();
            m.properties.color = Color::from_hex(hex);
            m.properties.depth_test = false;
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                sphere.clone(),
                Arc::new(Material::Basic(m)),
            )));
            if let Ok(n) = s.get_mut(h) {
                n.visible = false;
                n.render_order = 10;
            }
            h
        });
        Ok(Self {
            fats: [chain, pairs],
            inter: spheres[0],
            on_line: spheres[1],
            controls,
            pointer: Vector2::splat(f64::INFINITY),
            params: [1., 1., 0., 1., 1., 0., 0., 1.],
            rotation: 0.,
            last: (0., 0.),
            drag: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate && self.params[7] > 0.5 {
            self.rotation += dt * 0.1;
        }
        Ok(())
    }
    /// animate(): the line state, then the pointer's raycast against the
    /// displayed line and the two spheres.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.update(s, c)?;
        let [
            kind,
            world,
            visualize,
            width,
            coverage,
            threshold,
            translation,
            _,
        ] = self.params;
        let shown = usize::from(kind > 0.5);
        let dpr = web_sys::window().map_or(1., |w| w.device_pixel_ratio()) as f32;
        for (i, fat) in self.fats.iter().enumerate() {
            let visible = i == shown;
            for (h, main) in [(fat.line, true), (fat.threshold, false)] {
                let n = s.get_mut(h)?;
                n.visible = visible && (main || visualize > 0.5);
                n.position = Vector3::new(translation, 0., 0.);
                n.quaternion =
                    Quaternion::from_rotation_y(if main { self.rotation } else { self.last.0 });
                if let NodeKind::Mesh(m) = &mut n.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    // Line2NodeMaterial defaults to alphaToCoverage: the threshold
                    // material keeps it.
                    let a2c = !main || coverage > 0.5;
                    m.program =
                        fat.variants[usize::from(world > 0.5) | (usize::from(a2c) * 2)].clone();
                    let linewidth = if main { width } else { width + threshold };
                    m.uniforms[0] = [linewidth as f32, world as f32, 0., dpr];
                    m.uniforms[1] = [1., 0., 1., 1.];
                    m.uniforms[2] = [f32::from(a2c), f32::from(!main), 0., 0.];
                    m.properties.alpha_to_coverage = a2c;
                }
            }
        }
        s.update()?;
        let (camera, world_matrix) = s.camera(c)?;
        let hit = if self.pointer.is_finite() {
            let ray = camera.ray(self.pointer, world_matrix)?;
            let fat = &self.fats[shown];
            let matrix = Matrix4::from_rotation_translation(
                Quaternion::from_rotation_y(self.last.0),
                Vector3::new(self.last.1, 0., 0.),
            );
            self.raycast(
                ray,
                camera,
                world_matrix,
                matrix,
                fat,
                width + threshold,
                world > 0.5,
            )?
        } else {
            None
        };
        self.last = (self.rotation, translation);
        s.get_mut(self.inter)?.visible = hit.is_some();
        s.get_mut(self.on_line)?.visible = hit.is_some();
        if let Some((point, on_line, index)) = hit {
            let color = self.fats[shown].segments[index][2];
            let (h, sat, l) = get_hsl(color);
            for (sphere, position, offset) in
                [(self.inter, point, 0.3), (self.on_line, on_line, 0.7)]
            {
                let n = s.get_mut(sphere)?;
                n.position = position;
                if let NodeKind::Mesh(m) = &mut n.kind
                    && let Material::Basic(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.properties.color = Color::from_hsl(h + offset, sat, l);
                }
            }
        }
        Ok(())
    }
    /// LineSegments2.raycast: the nearest hit's point, point on the line and
    /// segment index, in world units or in CSS-pixel screen space.
    #[allow(clippy::too_many_arguments)]
    fn raycast(
        &self,
        ray: Ray,
        camera: &Camera,
        camera_world: Matrix4,
        matrix: Matrix4,
        fat: &Fat,
        line_width: f64,
        world_units: bool,
    ) -> Result<Option<(Vector3, Vector3, usize)>> {
        let mut hits = vec![];
        let world = |p: Vector3| matrix.transform_point3(p);
        if world_units {
            for (i, [a, b, ..]) in fat.segments.iter().enumerate() {
                let (_, point, on_line) = distance_sq_to_segment(ray, world(*a), world(*b));
                if point.distance(on_line) < line_width * 0.5 {
                    hits.push((ray.origin.distance(point), point, on_line, i));
                }
            }
        } else {
            let Camera::Perspective(p) = camera else {
                return Err(Error::Invalid("line raycast camera"));
            };
            let window = web_sys::window().ok_or(Error::Invalid("window"))?;
            let resolution = Vector2::new(
                window
                    .inner_width()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.),
                window
                    .inner_height()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.),
            );
            let projection = camera.projection_matrix()?;
            let view = camera_world.inverse();
            let near = -p.near;
            let mut origin = projection * (view * ray.at(1.).extend(1.));
            origin /= origin.w;
            let origin = Vector3::new(
                origin.x * resolution.x / 2.,
                origin.y * resolution.y / 2.,
                0.,
            );
            let mv = view * matrix;
            for (i, [a, b, ..]) in fat.segments.iter().enumerate() {
                let mut start = mv * a.extend(1.);
                let mut end = mv * b.extend(1.);
                if start.z > near && end.z > near {
                    continue;
                }
                if start.z > near {
                    let t = (start.z - near) / (start.z - end.z);
                    start = start.lerp(end, t);
                } else if end.z > near {
                    let t = (end.z - near) / (end.z - start.z);
                    end = end.lerp(start, t);
                }
                let mut start = projection * start;
                let mut end = projection * end;
                start /= start.w;
                end /= end.w;
                let s2 = Vector3::new(start.x * resolution.x / 2., start.y * resolution.y / 2., 0.);
                let e2 = Vector3::new(end.x * resolution.x / 2., end.y * resolution.y / 2., 0.);
                // Line3.closestPointToPointParameter( origin, clampToLine ).
                let d = e2 - s2;
                let param = ((origin - s2).dot(d) / d.dot(d)).clamp(0., 1.);
                let closest = s2 + d * param;
                let z = start.z + (end.z - start.z) * param;
                if (-1. ..=1.).contains(&z) && origin.distance(closest) < line_width * 0.5 {
                    let (_, point, on_line) = distance_sq_to_segment(ray, world(*a), world(*b));
                    hits.push((ray.origin.distance(point), point, on_line, i));
                }
            }
        }
        hits.sort_by(|a, b| a.0.total_cmp(&b.0));
        Ok(hits.first().map(|&(_, p, q, i)| (p, q, i)))
    }
    /// Pointer events in CSS pixels: kind 0 moves, 10 + button presses and
    /// 20 + button releases.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(window) = web_sys::window() {
            let w = window
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1.);
            let h = window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1.);
            self.pointer = Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.);
        }
        match kind {
            10..=19 => self.drag = Some((Vector2::new(x, y), kind - 10)),
            20..=29 => self.drag = None,
            _ => {}
        }
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan || self.drag.is_some_and(|(_, b)| b == 2) {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("lines raycast parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.rotation = t * 0.1;
    }
}
