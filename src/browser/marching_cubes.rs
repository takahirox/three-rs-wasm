//! webgl_marchingcubes: MarchingCubes' metaballs and planes, polygonized on
//! the CPU each frame into the mesh's dynamic position, normal, uv and color
//! attributes exactly as the original's object does (Float32 field, normal
//! cache and palette), under a directional, a point and an ambient light,
//! with thirteen materials: environment-mapped standard and Lambert
//! (reflection and refraction), Phong variants with a texture and vertex
//! colors, and the four ToonShader ShaderMaterials. The streamed attributes
//! are written into the mesh's resident buffers.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::marching_tables::{EDGE_TABLE, TRI_TABLE};
use crate::shader::ShaderProgram;
use crate::tsl::surface::SurfaceNodes;
use crate::tsl::{NodeMaterial, Type, WgslFn, normal_world, position_world};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, environment::EnvironmentMap, geometry::*,
    material::*, math::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;
/// One cube edge: its bit, the corner cells, the axis, the list offset, the
/// origin and the two corner fields.
type Edge = (u32, usize, usize, usize, usize, [f64; 3], usize, usize);

const ASSETS: &str = "/web/gallery/assets/environment-materials/textures";
const MATERIALS: [&str; 13] = [
    "shiny",
    "chrome",
    "liquid",
    "matte",
    "flat",
    "textured",
    "colors",
    "multiColors",
    "plastic",
    "toon1",
    "toon2",
    "hatching",
    "dotted",
];
const MAX_POLY_COUNT: usize = 100_000;

/// The MarchingCubes object's state: the Float32 field, normal cache and
/// palette, and the attribute arrays filled by polygonize.
struct Cubes {
    size: usize,
    halfsize: f64,
    delta: f64,
    isolation: f64,
    field: Vec<f32>,
    normal_cache: Vec<f32>,
    palette: Vec<f32>,
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    colors: Vec<f32>,
    count: usize,
    enable_uvs: bool,
    enable_colors: bool,
}
impl Cubes {
    fn new(resolution: usize) -> Self {
        let size = resolution;
        let size3 = size * size * size;
        let max = MAX_POLY_COUNT * 3;
        Self {
            size,
            halfsize: size as f64 / 2.,
            delta: 2. / size as f64,
            isolation: 80.,
            field: vec![0.; size3],
            normal_cache: vec![0.; size3 * 3],
            palette: vec![0.; size3 * 3],
            positions: vec![0.; max * 3],
            normals: vec![0.; max * 3],
            uvs: vec![0.; max * 2],
            colors: vec![0.; max * 3],
            count: 0,
            enable_uvs: false,
            enable_colors: false,
        }
    }
    /// init( resolution ): a new field; the attribute arrays are kept.
    fn init(&mut self, resolution: usize) {
        let size3 = resolution * resolution * resolution;
        self.size = resolution;
        self.halfsize = resolution as f64 / 2.;
        self.delta = 2. / resolution as f64;
        self.isolation = 80.;
        self.field = vec![0.; size3];
        self.normal_cache = vec![0.; size3 * 3];
        self.palette = vec![0.; size3 * 3];
        self.count = 0;
    }
    fn reset(&mut self) {
        self.field.fill(0.);
        for i in 0..self.field.len() {
            self.normal_cache[i * 3] = 0.;
        }
        self.palette.fill(0.);
    }
    fn add_ball(&mut self, ball: Vector3, strength: f64, subtract: f64, color: Vector3) {
        let sign = strength.signum();
        let strength = strength.abs();
        let size = self.size as f64;
        let radius = size * (strength / subtract).sqrt();
        let (xs, ys, zs) = (ball.x * size, ball.y * size, ball.z * size);
        let lo = |v: f64| ((v - radius).floor() as i64).max(1);
        let hi = |v: f64| ((v + radius).floor() as i64).min(self.size as i64 - 1);
        let (min_z, max_z, min_y, max_y, min_x, max_x) =
            (lo(zs), hi(zs), lo(ys), hi(ys), lo(xs), hi(xs));
        let size2 = self.size * self.size;
        for z in min_z..max_z {
            let z_offset = size2 * z as usize;
            let fz = z as f64 / size - ball.z;
            let fz2 = fz * fz;
            for y in min_y..max_y {
                let y_offset = z_offset + self.size * y as usize;
                let fy = y as f64 / size - ball.y;
                let fy2 = fy * fy;
                for x in min_x..max_x {
                    let fx = x as f64 / size - ball.x;
                    let val = strength / (0.000001 + fx * fx + fy2 + fz2) - subtract;
                    if val > 0. {
                        let i = y_offset + x as usize;
                        self.field[i] = (self.field[i] as f64 + val * sign) as f32;
                        let (dx, dy, dz) = (x as f64 - xs, y as f64 - ys, z as f64 - zs);
                        let ratio = (dx * dx + dy * dy + dz * dz).sqrt() / radius;
                        let contrib =
                            1. - ratio * ratio * ratio * (ratio * (ratio * 6. - 15.) + 10.);
                        for (k, c) in [color.x, color.y, color.z].into_iter().enumerate() {
                            let p = &mut self.palette[i * 3 + k];
                            *p = (*p as f64 + c * contrib) as f32;
                        }
                    }
                }
            }
        }
    }
    /// addPlaneX / addPlaneY / addPlaneZ along `axis` ( 0, 1, 2 ).
    fn add_plane(&mut self, axis: usize, strength: f64, subtract: f64) {
        let size = self.size;
        let (yd, zd) = (size, size * size);
        let dist = (size as f64 * (strength / subtract).sqrt()).min(size as f64);
        let mut d = 0usize;
        while (d as f64) < dist {
            let div = d as f64 / size as f64;
            let val = strength / (0.0001 + div * div) - subtract;
            if val > 0. {
                for a in 0..size {
                    for b in 0..size {
                        let i = match axis {
                            0 => zd * b + d + a * yd,
                            1 => zd * b + d * yd + a,
                            _ => zd * d + a * yd + b,
                        };
                        self.field[i] = (self.field[i] as f64 + val) as f32;
                    }
                }
            }
            d += 1;
        }
    }
    fn comp_norm(&mut self, q: usize) {
        let q3 = q * 3;
        if self.normal_cache[q3] == 0. {
            let (yd, zd) = (self.size, self.size * self.size);
            let f = &self.field;
            self.normal_cache[q3] = (f[q - 1] as f64 - f[q + 1] as f64) as f32;
            self.normal_cache[q3 + 1] = (f[q - yd] as f64 - f[q + yd] as f64) as f32;
            self.normal_cache[q3 + 2] = (f[q - zd] as f64 - f[q + zd] as f64) as f32;
        }
    }
    /// VIntX / VIntY / VIntZ: the edge vertex along `axis`, its normal and
    /// color, into the Float32 lists at `offset`.
    #[allow(clippy::too_many_arguments)]
    fn vint(
        &self,
        lists: &mut [[f32; 36]; 3],
        axis: usize,
        q: usize,
        offset: usize,
        p: [f64; 3],
        v1: f32,
        v2: f32,
        c1: usize,
        c2: usize,
    ) {
        let mu = (self.isolation - v1 as f64) / (v2 as f64 - v1 as f64);
        let mut v = p;
        v[axis] += mu * self.delta;
        let step = [1, self.size, self.size * self.size][axis] * 3;
        let lerp = |a: f32, b: f32| (a as f64 + (b as f64 - a as f64) * mu) as f32;
        for k in 0..3 {
            lists[0][offset + k] = v[k] as f32;
            lists[1][offset + k] = lerp(self.normal_cache[q + k], self.normal_cache[q + step + k]);
            lists[2][offset + k] = lerp(self.palette[c1 * 3 + k], self.palette[c2 * 3 + k]);
        }
    }
    fn polygonize(&mut self, fx: f64, fy: f64, fz: f64, q: usize) {
        let (yd, zd) = (self.size, self.size * self.size);
        let q1 = q + 1;
        let (qy, qz) = (q + yd, q + zd);
        let (q1y, q1z) = (q1 + yd, q1 + zd);
        let (qyz, q1yz) = (q + yd + zd, q1 + yd + zd);
        let f = [
            self.field[q],
            self.field[q1],
            self.field[qy],
            self.field[q1y],
            self.field[qz],
            self.field[q1z],
            self.field[qyz],
            self.field[q1yz],
        ];
        let mut cube = 0usize;
        for (bit, i) in [
            (1, 0),
            (2, 1),
            (8, 2),
            (4, 3),
            (16, 4),
            (32, 5),
            (128, 6),
            (64, 7),
        ] {
            if (f[i] as f64) < self.isolation {
                cube |= bit;
            }
        }
        let bits = EDGE_TABLE[cube];
        if bits == 0 {
            return;
        }
        let d = self.delta;
        let (fx2, fy2, fz2) = (fx + d, fy + d, fz + d);
        let mut lists = [[0f32; 36]; 3];
        // ( bit, first/second corner cells, axis, offset, origin, fields ).
        let edges: [Edge; 12] = [
            (1, q, q1, 0, 0, [fx, fy, fz], 0, 1),
            (2, q1, q1y, 1, 3, [fx2, fy, fz], 1, 3),
            (4, qy, q1y, 0, 6, [fx, fy2, fz], 2, 3),
            (8, q, qy, 1, 9, [fx, fy, fz], 0, 2),
            (16, qz, q1z, 0, 12, [fx, fy, fz2], 4, 5),
            (32, q1z, q1yz, 1, 15, [fx2, fy, fz2], 5, 7),
            (64, qyz, q1yz, 0, 18, [fx, fy2, fz2], 6, 7),
            (128, qz, qyz, 1, 21, [fx, fy, fz2], 4, 6),
            (256, q, qz, 2, 24, [fx, fy, fz], 0, 4),
            (512, q1, q1z, 2, 27, [fx2, fy, fz], 1, 5),
            (1024, q1y, q1yz, 2, 30, [fx2, fy2, fz], 3, 7),
            (2048, qy, qyz, 2, 33, [fx, fy2, fz], 2, 6),
        ];
        for (bit, a, b, axis, offset, origin, fa, fb) in edges {
            if bits & bit != 0 {
                self.comp_norm(a);
                self.comp_norm(b);
                self.vint(&mut lists, axis, a * 3, offset, origin, f[fa], f[fb], a, b);
            }
        }
        let base = cube << 4;
        let mut i = 0;
        while TRI_TABLE[base + i] != -1 {
            let o = [0, 1, 2].map(|k| 3 * TRI_TABLE[base + i + k] as usize);
            self.emit(&lists, o);
            i += 3;
        }
    }
    /// posnormtriv (smooth shading).
    fn emit(&mut self, lists: &[[f32; 36]; 3], o: [usize; 3]) {
        let c = self.count * 3;
        if c + 9 > self.positions.len() {
            return;
        }
        for (v, &at) in o.iter().enumerate() {
            for k in 0..3 {
                self.positions[c + v * 3 + k] = lists[0][at + k];
                self.normals[c + v * 3 + k] = lists[1][at + k];
            }
            if self.enable_uvs {
                let d = self.count * 2;
                self.uvs[d + v * 2] = lists[0][at];
                self.uvs[d + v * 2 + 1] = lists[0][at + 2];
            }
            if self.enable_colors {
                for k in 0..3 {
                    self.colors[c + v * 3 + k] = lists[2][at + k];
                }
            }
        }
        self.count += 3;
    }
    fn update(&mut self) {
        self.count = 0;
        let smin2 = self.size - 2;
        let (size, size2) = (self.size, self.size * self.size);
        for z in 1..smin2 {
            let z_offset = size2 * z;
            let fz = (z as f64 - self.halfsize) / self.halfsize;
            for y in 1..smin2 {
                let y_offset = z_offset + size * y;
                let fy = (y as f64 - self.halfsize) / self.halfsize;
                for x in 1..smin2 {
                    let fx = (x as f64 - self.halfsize) / self.halfsize;
                    self.polygonize(fx, fy, fz, y_offset + x);
                }
            }
        }
    }
}
/// ToonShader1's refraction vector, per vertex, in the unused local-normal
/// varying.
const TOON_REFRACT: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let world=(u.model*vec4(surface.local_position,1.0)).xyz;let n=normalize(mat3x3(u.model[0].xyz,u.model[1].xyz,u.model[2].xyz)*surface.local_normal);out.local_normal=refract(normalize(world-u.camera.xyz),n,1.02);return out;}";
const PLAIN: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
/// The ToonShader fragment shaders: custom[0] light position, [1] light
/// color, [2] ambient color, [3] base color, [4..8] line colors, [8].x the
/// drawing buffer height for WebGL's bottom-up gl_FragCoord.
fn toon_source(kind: usize) -> String {
    let head = "let n=normalize(fragment_surface.normal);let pos=u.custom[0].xyz;let light=u.custom[1].xyz;let ambient=u.custom[2].xyz;let base=u.custom[3].xyz;let fc=vec2(fragment_surface.clip.x,u.custom[8].x-fragment_surface.clip.y);";
    let body = match kind {
        0 => {
            "let w=ambient+light*max(dot(n,pos),0.0);var i=smoothstep(-0.5,1.0,pow(length(w),20.0));i+=length(w)*0.2;let cw=dot(n,fragment_surface.local_normal);i+=pow(1.0-length(cw),6.0);i=i*0.2+0.3;if i<0.5 {return vec4(2.0*i*base,1.0);}return vec4(1.0-2.0*(1.0-i)*(1.0-base),1.0);"
        }
        1 => {
            "let camera=max(dot(n,vec3(0.0,0.0,1.0)),0.4);let l=max(dot(n,pos),0.0);var c=vec4(base,1.0);if length(ambient+light*l)<1.0 {c*=vec4(u.custom[4].xyz,1.0);}if length(ambient+light*camera)<0.5 {c*=vec4(u.custom[5].xyz,1.0);}return c;"
        }
        2 => {
            "let w=length(ambient+light*max(dot(n,pos),0.0));var c=vec4(base,1.0);if w<1.0 && glsl_mod(fc.x+fc.y,10.0)==0.0 {c=vec4(u.custom[4].xyz,1.0);}if w<0.75 && glsl_mod(fc.x-fc.y,10.0)==0.0 {c=vec4(u.custom[5].xyz,1.0);}if w<0.5 && glsl_mod(fc.x+fc.y-5.0,10.0)==0.0 {c=vec4(u.custom[6].xyz,1.0);}if w<0.3465 && glsl_mod(fc.x-fc.y-5.0,10.0)==0.0 {c=vec4(u.custom[7].xyz,1.0);}return c;"
        }
        _ => {
            "let w=length(ambient+light*max(dot(n,pos),0.0));var c=vec4(base,1.0);if w<1.0 && glsl_mod(fc.x,4.001)+glsl_mod(fc.y,4.0)>6.0 {c=vec4(u.custom[4].xyz,1.0);}if w<0.5 && glsl_mod(fc.x+2.0,4.001)+glsl_mod(fc.y+2.0,4.0)>6.0 {c=vec4(u.custom[4].xyz,1.0);}return c;"
        }
    };
    format!(
        "fn glsl_mod(x:f32,y:f32)->f32{{return x-y*floor(x/y);}}fn toon_color()->vec4<f32>{{{head}{body}}}"
    )
}
pub(super) struct Demo {
    controls: Controls,
    cubes: Cubes,
    mesh: Object3D,
    materials: Vec<Arc<Material>>,
    environment: Arc<EnvironmentMap>,
    current: usize,
    /// speed, numBlobs, resolution, isolation, floor, wallx, wallz.
    params: [f64; 7],
    resolution: usize,
    time: f64,
    last: f64,
    pending: Option<f64>,
    pending_material: Option<usize>,
    _cube: crate::texture_gpu::GpuTexture,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-500., 500., 1500.);
        s.background = Color::from_hex(0x050505);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0.5, 0.5, 1.);
        let point = s.insert(NodeKind::Light(Light::Point {
            color: Color::from_hex(0xff7c00),
            intensity: 3.,
            distance: 0.,
            decay: 0.,
        }));
        s.get_mut(point)?.position = Vector3::new(0., 0., 100.);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x323232),
            intensity: 3.,
        }));
        let mut controls = Controls::new(None, (500., 5000.), PI, true);
        controls.update(s, c)?;
        // CubeTextureLoader: the sRGB SwedishRoyalCastle faces.
        let mut faces = vec![];
        for name in ["px", "nx", "py", "ny", "pz", "nz"] {
            let mut t = decode_texture_image(
                &fetch(&format!("{ASSETS}/cube/SwedishRoyalCastle/{name}.jpg")).await?,
            )
            .await?;
            t.srgb = true;
            faces.push(t);
        }
        let cube = crate::texture_gpu::GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        let environment = Arc::new(EnvironmentMap::from_cube_texture(r, &cube)?);
        // Lambert's envmap_fragment: outgoingLight × the cube, sampled with
        // flipEnvMap −1 along the per-fragment reflection or refraction; the
        // diffuse term scales linearly, so it multiplies the diffuse color.
        let env_lambert = async |refraction: Option<f64>| -> Result<Material> {
            let vector = match refraction {
                Some(ratio) => format!("refract(v,n,{ratio:?})"),
                None => "reflect(v,n)".into(),
            };
            let node = WgslFn::new(
                "envmap_color",
                &format!("fn envmap_color(n:vec3<f32>,p:vec3<f32>)->vec4<f32>{{let v=normalize(p-u.camera.xyz);let r={vector};return vec4(u.color.rgb*textureSample(tsl_texture_0,tsl_sampler_0,vec3(-r.x,r.yz)).rgb,u.color.a);}}"),
                &[Type::Vec3, Type::Vec3],
                Type::Vec4,
            )?
            .call(&[normal_world(), position_world()]);
            let program = SurfaceNodes {
                color: Some(node),
                ..Default::default()
            }
            .build_with_texture_types(r, &[], &[(&cube.view, &cube.sampler, Type::TextureCube)])
            .await?;
            let mut m = MeshLambertMaterial::default();
            m.properties.vertex_program = Some(Arc::new(program));
            Ok(Material::Lambert(m))
        };
        let mut uv_grid =
            decode_texture_image(&fetch(&format!("{ASSETS}/uv_grid_opengl.jpg")).await?).await?;
        uv_grid.srgb = true;
        uv_grid.wrap_s = Wrapping::Repeat;
        uv_grid.wrap_t = Wrapping::Repeat;
        uv_grid.mipmap_filter = Some(Filter::Linear);
        let phong = |color: u32, specular: u32, shininess: f64| {
            let mut m = MeshPhongMaterial {
                specular: Color::from_hex(specular),
                shininess,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(color);
            m
        };
        let toon = async |kind: usize| -> Result<Material> {
            let node = WgslFn::new("toon_color", &toon_source(kind), &[], Type::Vec4)?.call(&[]);
            let program = ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(node).wgsl(0)?,
                &[],
                &[],
                if kind == 0 { TOON_REFRACT } else { PLAIN },
            )
            .await?;
            let mut m = ShaderMaterial::new(Arc::new(program));
            let linear = |hex: u32| {
                let c = Color::from_hex(hex).0;
                [c.x as f32, c.y as f32, c.z as f32, 1.]
            };
            // uDirLightPos = light.position, uDirLightColor = light.color,
            // uAmbientLightColor = ambientLight.color, then the shader's own
            // base and line colors.
            m.uniforms[0] = [0.5, 0.5, 1., 0.];
            m.uniforms[1] = linear(0xffffff);
            m.uniforms[2] = linear(0x323232);
            m.uniforms[3] = linear(if kind == 1 { 0xeeeeee } else { 0xffffff });
            m.uniforms[4] = linear(if kind == 1 { 0x808080 } else { 0x000000 });
            for i in 5..8 {
                m.uniforms[i] = linear(0x000000);
            }
            Ok(Material::Shader(m))
        };
        let shiny = {
            let mut m = MeshStandardMaterial {
                roughness: 0.1,
                metalness: 1.,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(0x9c0000);
            Material::Standard(m)
        };
        let mut textured = phong(0xffffff, 0x111111, 1.);
        textured.properties.map = Some(Arc::new(uv_grid));
        let mut colors = phong(0xffffff, 0xffffff, 2.);
        colors.properties.vertex_colors = true;
        let mut multi = phong(0xffffff, 0x111111, 2.);
        multi.properties.vertex_colors = true;
        let materials = vec![
            shiny,
            env_lambert(None).await?,
            env_lambert(Some(0.85)).await?,
            Material::Phong(phong(0xffffff, 0x494949, 1.)),
            Material::Lambert(MeshLambertMaterial::default()),
            Material::Phong(textured),
            Material::Phong(colors),
            Material::Phong(multi),
            Material::Phong(phong(0xffffff, 0xc1c1c1, 250.)),
            toon(0).await?,
            toon(1).await?,
            toon(2).await?,
            toon(3).await?,
        ]
        .into_iter()
        .map(Arc::new)
        .collect::<Vec<_>>();
        let mut geometry = BufferGeometry::default();
        for (name, size) in [("position", 3), ("normal", 3), ("uv", 2), ("color", 3)] {
            let mut a = BufferAttribute::new(vec![0f32; size * 3], size, false)?;
            a.set_usage(crate::attribute::Usage::Dynamic);
            geometry.set_attribute(name, Attribute::F32(a));
        }
        geometry.bounding_sphere = Some(Sphere {
            center: Vector3::ZERO,
            radius: 1.,
        });
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            materials[0].clone(),
        )));
        s.get_mut(mesh)?.scale = Vector3::splat(700.);
        let mut demo = Self {
            controls,
            cubes: Cubes::new(28),
            mesh,
            materials,
            environment,
            current: 0,
            params: [1., 10., 28., 80., 1., 0., 0.],
            resolution: 28,
            time: 0.,
            last: 0.,
            pending: None,
            pending_material: None,
            _cube: cube,
        };
        demo.select(s, 0)?;
        demo.frame(s, 0.)?;
        Ok(demo)
    }
    /// createHandler( id ): the material, and the uv and color switches.
    fn select(&mut self, s: &mut Scene, index: usize) -> Result<()> {
        self.current = index;
        self.cubes.enable_uvs = MATERIALS[index] == "textured";
        self.cubes.enable_colors = matches!(MATERIALS[index], "colors" | "multiColors");
        s.environment = (index == 0).then(|| self.environment.clone());
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.mesh)?.kind {
            m.materials = vec![self.materials[index].clone()];
        }
        Ok(())
    }
    /// render(): the clock, the resolution and isolation, updateCubes().
    fn frame(&mut self, s: &mut Scene, delta: f64) -> Result<()> {
        let [speed, blobs, resolution, isolation, floor, wallx, wallz] = self.params;
        self.time += delta * speed * 0.5;
        if resolution as usize != self.resolution {
            self.resolution = resolution as usize;
            self.cubes.init(resolution.floor() as usize);
        }
        self.cubes.isolation = isolation;
        self.cubes.reset();
        let rainbow = [
            0xff0000, 0xffbb00, 0xffff00, 0x00ff00, 0x0000ff, 0x9400bd, 0xc800eb,
        ];
        let numblobs = blobs as usize;
        let subtract = 12.;
        let strength = 1.2 / (((numblobs as f64).sqrt() - 1.) / 4. + 1.);
        let time = self.time;
        for i in 0..numblobs {
            let fi = i as f64;
            let ball = Vector3::new(
                (fi + 1.26 * time * (1.03 + 0.5 * (0.21 * fi).cos())).sin() * 0.27 + 0.5,
                (fi + 1.12 * time * (1.22 + 0.1424 * fi).cos()).cos().abs() * 0.77,
                (fi + 1.32 * time * 0.1 * (0.92 + 0.53 * fi).sin()).cos() * 0.27 + 0.5,
            );
            let color = if MATERIALS[self.current] == "multiColors" {
                Color::from_hex(rainbow[i % 7]).0
            } else {
                ball
            };
            self.cubes.add_ball(ball, strength, subtract, color);
        }
        if floor > 0.5 {
            self.cubes.add_plane(1, 2., 12.);
        }
        if wallz > 0.5 {
            self.cubes.add_plane(2, 2., 12.);
        }
        if wallx > 0.5 {
            self.cubes.add_plane(0, 2., 12.);
        }
        self.cubes.update();
        // setDrawRange( 0, count ) and needsUpdate: the used part of each array
        // is written into the mesh's resident buffers.
        let count = self.cubes.count;
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.mesh)?.kind {
            let g = Arc::make_mut(&mut m.geometry);
            for (name, data, size) in [
                ("position", &self.cubes.positions, 3),
                ("normal", &self.cubes.normals, 3),
                ("uv", &self.cubes.uvs, 2),
                ("color", &self.cubes.colors, 3),
            ] {
                if let Some(Attribute::F32(a)) = g.attributes.get_mut(name) {
                    a.replace(data[..count * size].to_vec())?;
                }
            }
        }
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.frame(s, dt)?;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if let Some(index) = self.pending_material.take() {
            self.select(s, index)?;
        }
        if let Some(t) = self.pending.take() {
            let delta = t - self.last;
            self.last = t;
            self.frame(s, delta)?;
        }
        // The toon shaders read gl_FragCoord from the bottom of the drawing
        // buffer.
        let height = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.query_selector("canvas").ok().flatten())
            .and_then(|c| wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlCanvasElement>(c).ok())
            .map_or(1., |c| c.height() as f32);
        for m in &mut self.materials[9..] {
            if let Material::Shader(m) = Arc::make_mut(m) {
                m.uniforms[8] = [height, 0., 0., 0.];
            }
        }
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.mesh)?.kind {
            m.materials = vec![self.materials[self.current].clone()];
        }
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
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
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    /// The thirteen material buttons, then speed, numBlobs, resolution,
    /// isolation, floor, wallx and wallz.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0..13 => self.pending_material = Some(index),
            13..20 => self.params[index - 13] = value as f64,
            _ => return Err(Error::Invalid("marching cubes parameter")),
        }
        Ok(())
    }
    /// A still frame at time t: one render() with the elapsed delta.
    pub fn seek(&mut self, t: f64) {
        self.pending = Some(t);
    }
}
