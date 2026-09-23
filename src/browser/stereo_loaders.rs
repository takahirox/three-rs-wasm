//! Stereo, anaglyph and parallax-barrier effects, and the PCD and ImageBitmap
//! loader examples from the pinned WebGL examples.
use super::gltf_viewer::{decode_texture_image, fetch};
use super::interactive_objects::Orbit;
use crate::tsl::Node;
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*, shader::ShaderProgram, tsl::*,
};
use std::sync::Arc;
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
const FULLSCREEN: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.xy*2.0,1.0,1.0);return out;}";
/// A WebGPU-range off-axis frustum, Three.js's `makePerspective( l, r, t, b, n, f )`.
fn frustum(l: f64, r: f64, t: f64, b: f64, n: f64, f: f64) -> Matrix4 {
    Matrix4::from_cols_array(&[
        2. * n / (r - l),
        0.,
        0.,
        0.,
        0.,
        2. * n / (t - b),
        0.,
        0.,
        (r + l) / (r - l),
        (t + b) / (t - b),
        -f / (f - n),
        -1.,
        0.,
        0.,
        -f * n / (f - n),
        0.,
    ])
}
/// CubeTextureLoader: six sRGB faces with mipmaps.
async fn cube(r: &Renderer, base: &str, ext: &str) -> Result<GpuTexture> {
    let mut faces = vec![];
    for face in ["px", "nx", "py", "ny", "pz", "nz"] {
        let mut t = decode_texture_image(&fetch(&format!("{base}/{face}.{ext}")).await?).await?;
        t.srgb = true;
        faces.push(t);
    }
    GpuTexture::from_cube_rgba(
        r,
        &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
    )
}
/// Per-vertex envMap direction of MeshBasicMaterial (`vReflect`) from the world normal, in local_normal.
fn env_projection(refraction: Option<f32>) -> String {
    let dir = match refraction {
        Some(ratio) => format!("refract(v,n,{ratio:?})"),
        None => "reflect(v,n)".into(),
    };
    format!(
        "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{var out=surface;let v=normalize(surface.position-u.camera.xyz);let n=normalize((u.normal*vec4(surface.local_normal,0.0)).xyz);out.local_normal={dir};return out;}}"
    )
}
/// PCDLoader.parse: ascii, binary and LZF binary_compressed; x/y/z and packed rgb.
fn parse_pcd(data: &[u8]) -> Result<(Vec<f32>, Vec<f32>)> {
    let bad = |m: &'static str| Error::Asset(format!("PCD: {m}"));
    // The header ends after the line starting with DATA.
    let mut end = 0;
    let mut line_start = 0;
    for (i, &c) in data.iter().enumerate() {
        if c == b'\n' || c == b'\r' {
            let line = String::from_utf8_lossy(&data[line_start..i]);
            if line.trim().to_lowercase().starts_with("data") {
                end = i + 1;
                break;
            }
            line_start = i + 1;
        }
    }
    if end == 0 {
        return Err(bad("header"));
    }
    let header = String::from_utf8_lossy(&data[..end]).to_string();
    let field = |key: &str| -> Vec<String> {
        header
            .lines()
            .filter(|l| !l.starts_with('#'))
            .find(|l| l.to_uppercase().starts_with(&format!("{key} ")))
            .map(|l| {
                l[key.len() + 1..]
                    .split_whitespace()
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    };
    let fields = field("FIELDS");
    let size: Vec<usize> = field("SIZE")
        .iter()
        .map(|x| x.parse().unwrap_or(4))
        .collect();
    let types = field("TYPE");
    let count: Vec<usize> = field("COUNT")
        .iter()
        .map(|x| x.parse().unwrap_or(1))
        .collect();
    let count = if count.is_empty() {
        vec![1; fields.len()]
    } else {
        count
    };
    let mode = field("DATA").first().cloned().unwrap_or_default();
    let points = field("POINTS")
        .first()
        .and_then(|x| x.parse::<usize>().ok())
        .or_else(|| {
            let w: usize = field("WIDTH").first()?.parse().ok()?;
            let h: usize = field("HEIGHT").first()?.parse().ok()?;
            Some(w * h)
        })
        .ok_or(bad("points"))?;
    let index = |name: &str| fields.iter().position(|f| f == name);
    let mut offsets = vec![0; fields.len()];
    let mut row = 0;
    for (i, offset) in offsets.iter_mut().enumerate() {
        *offset = if mode == "ascii" { i } else { row };
        row += size.get(i).copied().unwrap_or(4) * count.get(i).copied().unwrap_or(1);
    }
    let read = |bytes: &[u8], at: usize, i: usize| -> f64 {
        let (t, s) = (types.get(i).map(String::as_str).unwrap_or("F"), size[i]);
        let b = |n: usize| bytes.get(at..at + n);
        match (t, s) {
            ("F", 8) => b(8).map(|v| f64::from_le_bytes(v.try_into().unwrap())),
            ("F", _) => b(4).map(|v| f32::from_le_bytes(v.try_into().unwrap()) as f64),
            ("I", 1) => b(1).map(|v| v[0] as i8 as f64),
            ("I", 2) => b(2).map(|v| i16::from_le_bytes(v.try_into().unwrap()) as f64),
            ("I", _) => b(4).map(|v| i32::from_le_bytes(v.try_into().unwrap()) as f64),
            (_, 1) => b(1).map(|v| v[0] as f64),
            (_, 2) => b(2).map(|v| u16::from_le_bytes(v.try_into().unwrap()) as f64),
            _ => b(4).map(|v| u32::from_le_bytes(v.try_into().unwrap()) as f64),
        }
        .unwrap_or(f64::NAN)
    };
    let (mut position, mut color) = (vec![], vec![]);
    let push_rgb = |color: &mut Vec<f32>, b: u8, g: u8, r: u8| {
        // c.setRGB( r, g, b, SRGBColorSpace ): decoded to the working space.
        color.extend([r, g, b].map(|v| srgb_to_linear_f(v as f64 / 255.) as f32));
    };
    let (x, y, z, rgb) = (index("x"), index("y"), index("z"), index("rgb"));
    match mode.as_str() {
        "ascii" => {
            let text = String::from_utf8_lossy(&data[end..]);
            for line in text.split('\n') {
                if line.is_empty() {
                    continue;
                }
                let v: Vec<&str> = line.split(' ').collect();
                let num = |i: usize| {
                    v.get(i)
                        .and_then(|s| s.trim().parse::<f64>().ok())
                        .unwrap_or(f64::NAN)
                };
                if let (Some(x), Some(y), Some(z)) = (x, y, z) {
                    position.extend(
                        [num(offsets[x]), num(offsets[y]), num(offsets[z])].map(|v| v as f32),
                    );
                }
                if let Some(i) = rgb {
                    let f = num(offsets[i]);
                    let packed = if types.get(i).map(String::as_str) == Some("F") {
                        (f as f32).to_bits() as i32
                    } else {
                        f as i32
                    };
                    push_rgb(
                        &mut color,
                        packed as u8,
                        (packed >> 8) as u8,
                        (packed >> 16) as u8,
                    );
                }
            }
        }
        "binary" => {
            let body = &data[end..];
            for p in 0..points {
                let base = p * row;
                if let (Some(xi), Some(yi), Some(zi)) = (x, y, z) {
                    for i in [xi, yi, zi] {
                        position.push(read(body, base + offsets[i], i) as f32);
                    }
                }
                if let Some(i) = rgb {
                    let at = base + offsets[i];
                    let c = body.get(at..at + 3).ok_or(bad("rgb"))?;
                    push_rgb(&mut color, c[0], c[1], c[2]);
                }
            }
        }
        "binary_compressed" => {
            let sizes = data.get(end..end + 8).ok_or(bad("sizes"))?;
            let compressed = u32::from_le_bytes(sizes[..4].try_into().unwrap()) as usize;
            let decompressed = u32::from_le_bytes(sizes[4..].try_into().unwrap()) as usize;
            let body = lzf(
                data.get(end + 8..end + 8 + compressed).ok_or(bad("body"))?,
                decompressed,
            )?;
            for p in 0..points {
                if let (Some(xi), Some(yi), Some(zi)) = (x, y, z) {
                    for i in [xi, yi, zi] {
                        position.push(read(&body, points * offsets[i] + size[i] * p, i) as f32);
                    }
                }
                if let Some(i) = rgb {
                    let at = points * offsets[i] + size[i] * p;
                    let c = body.get(at..at + 3).ok_or(bad("rgb"))?;
                    push_rgb(&mut color, c[0], c[1], c[2]);
                }
            }
        }
        _ => return Err(bad("data type")),
    }
    Ok((position, color))
}
fn srgb_to_linear_f(v: f64) -> f64 {
    crate::math::srgb_to_linear(v)
}
/// PCDLoader's `decompressLZF`.
fn lzf(input: &[u8], out_len: usize) -> Result<Vec<u8>> {
    let bad = || Error::Asset("PCD: invalid compressed data".into());
    let mut out = vec![0u8; out_len];
    let (mut ip, mut op) = (0, 0);
    while ip < input.len() {
        let ctrl = input[ip] as usize;
        ip += 1;
        if ctrl < 32 {
            let n = ctrl + 1;
            if op + n > out_len || ip + n > input.len() {
                return Err(bad());
            }
            out[op..op + n].copy_from_slice(&input[ip..ip + n]);
            op += n;
            ip += n;
        } else {
            let mut len = ctrl >> 5;
            let mut reference = op as isize - (((ctrl & 0x1f) << 8) as isize) - 1;
            if ip >= input.len() {
                return Err(bad());
            }
            if len == 7 {
                len += input[ip] as usize;
                ip += 1;
                if ip >= input.len() {
                    return Err(bad());
                }
            }
            reference -= input[ip] as isize;
            ip += 1;
            if op + len + 2 > out_len || reference < 0 || reference as usize >= op {
                return Err(bad());
            }
            // Byte by byte: a back-reference may overlap the bytes it produces.
            for reference in reference as usize..reference as usize + len + 2 {
                out[op] = out[reference];
                op += 1;
            }
        }
    }
    Ok(out)
}
const PCD_FILES: [&str; 4] = [
    "ascii/simple.pcd",
    "binary/Zaghetto.pcd",
    "binary/Zaghetto_8bit.pcd",
    "binary_compressed/pcl_logo.pcd",
];
struct Eyes {
    left: Object3D,
    right: Object3D,
    targets: Option<(RenderTarget, RenderTarget)>,
    composite: Option<(Scene, Object3D, Object3D)>,
    output: Option<RenderTarget>,
    sampler: wgpu::Sampler,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    pointer: Vector2,
    orbit: Orbit,
    objects: Vec<Object3D>,
    sky: Option<Object3D>,
    eyes: Option<Eyes>,
    points: Option<Object3D>,
    /// One resident node per PCD file, uploaded on first selection.
    clouds: Vec<Option<Object3D>>,
    params: [f32; 3],
    loaded: usize,
    dirty: bool,
    cubes: Vec<(f64, bool)>,
    earth: Option<Arc<crate::material::Texture>>,
    group: Option<Object3D>,
    seed: u32,
    pcd: Vec<Vec<u8>>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            203 => (60., 0.1, 100., Vector3::new(0., 0., 3.)),
            204 | 205 => (60., 0.01, 100., Vector3::new(0., 0., 3.)),
            206 => (30., 0.01, 40., Vector3::new(0., 0., 1.)),
            _ => (30., 1., 1500., Vector3::new(0., 4., 7.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = position;
        s.look_at(c, Vector3::ZERO)?;
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            pointer: Vector2::ZERO,
            orbit: Orbit::new(position, false, 1., (0.5, 10.)),
            objects: vec![],
            sky: None,
            eyes: None,
            points: None,
            clouds: vec![None; PCD_FILES.len()],
            params: [0.005, 16777215., 1.],
            loaded: usize::MAX,
            dirty: true,
            cubes: vec![],
            earth: None,
            group: None,
            seed: 186,
            pcd: vec![],
        };
        match id {
            203..=205 => d.stereo_scene(s, r).await?,
            206 => {
                for file in PCD_FILES {
                    d.pcd
                        .push(fetch(&format!("/web/gallery/assets/pcd/{file}")).await?);
                }
            }
            _ => {
                let group = s.insert(NodeKind::Group);
                let grid = super::interactive_scenes::grid_helper(4., 12, 0x888888, 0x444444)?;
                let g = s.insert(NodeKind::Line(grid));
                s.add(group, g)?;
                d.group = Some(group);
                let mut t =
                    decode_texture_image(&fetch("/web/gallery/assets/earth_atmos_2048.jpg").await?)
                        .await?;
                t.srgb = true;
                t.mipmap_filter = Some(Filter::Linear);
                d.earth = Some(Arc::new(t));
                // setTimeout( addImage, 300/600/900 ), setTimeout( addImageBitmap, 1300/1600/1900 ).
                d.cubes = [0.3, 0.6, 0.9, 1.3, 1.6, 1.9]
                    .iter()
                    .enumerate()
                    .map(|(i, &t)| (t, i < 3))
                    .collect();
            }
        }
        Ok(d)
    }
    async fn stereo_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let (base, ext) = if self.id == 203 {
            (
                "/web/gallery/assets/tsl-lighting/textures/cube/Park3Med",
                "jpg",
            )
        } else {
            (
                "/web/gallery/assets/environment-materials/textures/cube/pisa",
                "png",
            )
        };
        let env = cube(r, base, ext).await?;
        let sample = |d: Node| {
            call(
                "cube_color",
                "fn cube_color(d:vec3<f32>)->vec3<f32>{return textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb;}",
                &[Type::Vec3],
                Type::Vec3,
                &[d],
            )
        };
        // Background box: the world direction, recentered on each camera before rendering.
        let graph = NodeMaterial::new(vec4(sample(position_local())?, float(1.)));
        let source = graph.wgsl_with_texture_types(&[Type::TextureCube], &[])?;
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_texture_dimensions(
                r,
                &source,
                &[],
                &[(&env.view, &env.sampler)],
                &[wgpu::TextureViewDimension::Cube],
                &[wgpu::TextureSampleType::Float { filterable: true }],
            )
            .await?,
        ));
        m.properties.side = Side::Back;
        m.properties.depth_write = false;
        m.properties.depth_test = false;
        let sky = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 1., 1.)?),
            Arc::new(Material::Shader(m)),
        )));
        s.get_mut(sky)?.frustum_culled = false;
        s.get_mut(sky)?.render_order = -10000;
        self.sky = Some(sky);
        // MeshBasicMaterial white × envMap (reflection, or refraction with ratio 0.95).
        let graph = NodeMaterial::new(vec4(sample(normal_local())?, float(1.)));
        let source = graph.wgsl_with_texture_types(&[Type::TextureCube], &[])?;
        let projection = env_projection((self.id == 203).then_some(0.95));
        let m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection_and_dimensions(
                r,
                &source,
                &[(&env.view, &env.sampler)],
                &[wgpu::TextureViewDimension::Cube],
                &projection,
            )
            .await?,
        ));
        let (g, m) = (
            Arc::new(SphereGeometry::build(0.1, 32, 16)?),
            Arc::new(Material::Shader(m)),
        );
        let mut seed = 186;
        for _ in 0..500 {
            let h = s.insert(NodeKind::Mesh(Mesh::new(g.clone(), m.clone())));
            let p = [0; 3].map(|_| random(&mut seed) * 10. - 5.);
            let n = s.get_mut(h)?;
            n.position = Vector3::from_array(p);
            n.scale = Vector3::splat(random(&mut seed) * 3. + 1.);
            self.objects.push(h);
        }
        let eye = || NodeKind::Camera(Camera::Perspective(PerspectiveCamera::default()));
        let (left, right) = (s.insert(eye()), s.insert(eye()));
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("stereo target sampler"),
            min_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let placeholder = || {
            RenderTarget::with_options(
                &r.device,
                1,
                1,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    ..Default::default()
                },
            )
        };
        self.eyes = Some(Eyes {
            left,
            right,
            targets: Some((placeholder()?, placeholder()?)),
            composite: None,
            output: None,
            sampler,
        });
        if self.id != 203 {
            let composite = self.composite(r).await?;
            self.eyes.as_mut().expect("eyes").composite = Some(composite);
        }
        Ok(())
    }
    fn load_pcd(&mut self, s: &mut Scene) -> Result<()> {
        let file = self.params[2] as usize;
        // The type switch destroys the GUI and loads a fresh PointsMaterial.
        if self.loaded != usize::MAX {
            self.params[0] = 0.005;
            self.params[1] = 16777215.;
        }
        if let Some(old) = self.points.take() {
            s.get_mut(old)?.visible = false;
        }
        // The original re-parses and re-uploads the file on every switch; each parsed
        // cloud stays resident here and is shown again with fresh material defaults.
        let h = match self.clouds[file] {
            Some(h) => h,
            None => {
                let (p, colors) = parse_pcd(&self.pcd[file])?;
                let mut g = BufferGeometry::default();
                g.set_attribute("position", vec3s(p)?);
                let has_colors = !colors.is_empty();
                if has_colors {
                    g.set_attribute("color", vec3s(colors)?);
                }
                g.center()?;
                g.rotate_x(std::f64::consts::PI)?;
                let mut m = PointsMaterial::default();
                m.properties.vertex_colors = has_colors;
                let h = s.insert(NodeKind::Points(Points {
                    geometry: Arc::new(g),
                    material: Arc::new(Material::Points(m)),
                }));
                self.clouds[file] = Some(h);
                h
            }
        };
        let n = s.get_mut(h)?;
        n.visible = true;
        if let NodeKind::Points(p) = &mut n.kind
            && let Material::Points(m) = Arc::make_mut(&mut p.material)
        {
            m.size = self.params[0] as f64;
            m.properties.color = Color::WHITE;
        }
        self.points = Some(h);
        self.loaded = file;
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// One original animation frame.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let step = (t - self.last) * 60.;
        self.last = t;
        match self.id {
            203..=205 => {
                // camera.position += ( mouse - position ) * .05 per frame, as 60 fps steps.
                let keep = 0.95f64.powf(step);
                let n = s.get_mut(c)?;
                n.position.x = self.pointer.x + (n.position.x - self.pointer.x) * keep;
                n.position.y = -self.pointer.y + (n.position.y + self.pointer.y) * keep;
                s.look_at(c, Vector3::ZERO)?;
                let timer = 0.1 * t;
                for (i, h) in self.objects.iter().enumerate() {
                    let n = s.get_mut(*h)?;
                    n.position.x = 5. * (timer + i as f64).cos();
                    n.position.y = 5. * (timer + i as f64 * 1.1).sin();
                }
            }
            206 => {
                if self.loaded != self.params[2] as usize {
                    self.load_pcd(s)?;
                }
                if self.dirty
                    && let Some(h) = self.points
                    && let NodeKind::Points(p) = &mut s.get_mut(h)?.kind
                    && let Material::Points(m) = Arc::make_mut(&mut p.material)
                {
                    m.size = self.params[0] as f64;
                    // lil-gui's addColor writes the picked bytes to r, g, b without conversion.
                    let hex = self.params[1] as u32;
                    m.properties.color = Color::linear(
                        ((hex >> 16) & 255) as f64 / 255.,
                        ((hex >> 8) & 255) as f64 / 255.,
                        (hex & 255) as f64 / 255.,
                    );
                }
                self.dirty = false;
                self.orbit.apply(s, c)?;
            }
            _ => {
                let group = self.group.ok_or(Error::Invalid("group"))?;
                // group.rotation.y = performance.now() / 3000.
                s.get_mut(group)?.quaternion = Quaternion::from_rotation_y(t / 3.);
                while let Some(&(due, image)) = self.cubes.first() {
                    if t < due {
                        break;
                    }
                    self.cubes.remove(0);
                    let mut m = MeshBasicMaterial::default();
                    m.properties.map = self.earth.clone();
                    if image {
                        m.properties.color = Color::from_hex(0xff8888);
                    }
                    let h = s.insert(NodeKind::Mesh(Mesh::new(
                        Arc::new(BoxGeometry::build(1., 1., 1.)?),
                        Arc::new(Material::Basic(m)),
                    )));
                    let p = [0; 3].map(|_| random(&mut self.seed) * 2. - 1.);
                    let a = [0; 3].map(|_| random(&mut self.seed) * std::f64::consts::TAU);
                    let n = s.get_mut(h)?;
                    n.position = Vector3::from_array(p);
                    n.quaternion = Euler {
                        angles: Vector3::from_array(a),
                        order: EulerOrder::XYZ,
                    }
                    .quaternion();
                    s.add(group, h)?;
                }
            }
        }
        Ok(())
    }
    /// The eye cameras: StereoCamera for the stereo and parallax effects,
    /// frameCorners around a zero-parallax plane for the anaglyph.
    fn place_eyes(&self, s: &mut Scene, c: Object3D) -> Result<()> {
        let eyes = self.eyes.as_ref().ok_or(Error::Invalid("eyes"))?;
        let node = s.get(c)?.clone();
        let Camera::Perspective(camera) = s.camera(c)?.0.clone() else {
            return Err(Error::Invalid("stereo camera"));
        };
        let (n, f) = (camera.near, camera.far);
        let right = node.quaternion * Vector3::X;
        let up = node.quaternion * Vector3::Y;
        let forward = node.quaternion * Vector3::Z;
        for (h, sign) in [(eyes.left, -1.), (eyes.right, 1.)] {
            let eye = node.position + right * (sign * 0.032);
            let projection = if self.id == 204 {
                // AnaglyphEffect: frameCorners( eye, plane corners at planeDistance 3 ).
                let plane = 3.;
                let center = node.position - forward * plane;
                let half_h = plane * (camera.fov.to_radians() / 2.).tan();
                let half_w = half_h * camera.aspect;
                let pa = center - right * half_w - up * half_h;
                let pb = center + right * half_w - up * half_h;
                let pc = center - right * half_w + up * half_h;
                let (vr, vu) = ((pb - pa).normalize(), (pc - pa).normalize());
                let vn = vr.cross(vu).normalize();
                let (va, vb, vc) = (pa - eye, pb - eye, pc - eye);
                let d = -va.dot(vn);
                frustum(
                    vr.dot(va) * n / d,
                    vr.dot(vb) * n / d,
                    vu.dot(vc) * n / d,
                    vu.dot(va) * n / d,
                    n,
                    f,
                )
            } else {
                // StereoCamera.update: focus 10, aspect 0.5 (stereo) or 1 (parallax).
                let aspect = camera.aspect * if self.id == 203 { 0.5 } else { 1. };
                let ymax = n * (camera.fov.to_radians() * 0.5).tan();
                let shift = -sign * 0.032 * n / 10.;
                frustum(
                    -ymax * aspect + shift,
                    ymax * aspect + shift,
                    ymax,
                    -ymax,
                    n,
                    f,
                )
            };
            let e = s.get_mut(h)?;
            e.position = eye;
            e.quaternion = node.quaternion;
            e.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                fov: camera.fov,
                near: n,
                far: f,
                aspect: camera.aspect,
                projection_override: Some(projection),
                ..Default::default()
            }));
        }
        Ok(())
    }
    fn center_sky(&self, s: &mut Scene, camera: Object3D) -> Result<()> {
        if let Some(sky) = self.sky {
            let p = s.get(camera)?.position;
            s.get_mut(sky)?.position = p;
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if !(203..=205).contains(&self.id) {
            return Ok(false);
        }
        self.place_eyes(s, c)?;
        let (left, right) = {
            let e = self.eyes.as_ref().ok_or(Error::Invalid("eyes"))?;
            (e.left, e.right)
        };
        let dpr = web_sys::window()
            .map(|w| w.device_pixel_ratio())
            .unwrap_or(1.);
        if self.id == 203 {
            // StereoEffect: one clear, then the two halves under scissor and viewport.
            let eyes = self.eyes.as_mut().ok_or(Error::Invalid("eyes"))?;
            if eyes.output.as_ref().is_none_or(|t| {
                t.width != out.width
                    || t.height != out.height
                    || t.options.samples != out.options.samples
            }) {
                let mut options = out.options.clone();
                options.store_multisampled_color_buffer = true;
                eyes.output = Some(RenderTarget::with_options(
                    &r.device, out.width, out.height, options,
                )?);
            }
            let half = ((out.width as f64 / dpr / 2.) * dpr).round() as u32;
            for (i, camera) in [left, right].into_iter().enumerate() {
                let (x, w) = if i == 0 {
                    (0, half)
                } else {
                    (half, out.width - half)
                };
                let sky = self.sky;
                if let Some(sky) = sky {
                    let p = s.get(camera)?.position;
                    s.get_mut(sky)?.position = p;
                }
                let target = self
                    .eyes
                    .as_mut()
                    .and_then(|e| e.output.as_mut())
                    .expect("stereo target");
                target.viewport = [x, 0, w, out.height];
                target.scissor = Some([x, 0, w, out.height]);
                target.set_load_color(i != 0);
                r.render(s, camera, target)?;
            }
            let target = self
                .eyes
                .as_mut()
                .and_then(|e| e.output.as_mut())
                .expect("stereo target");
            target.scissor = None;
            target.viewport = [0, 0, out.width, out.height];
            return Ok(true);
        }
        // Anaglyph and parallax barrier: linear 8-bit eye targets, then a composite.
        let eyes = self.eyes.as_mut().ok_or(Error::Invalid("eyes"))?;
        if eyes
            .targets
            .as_ref()
            .is_none_or(|(l, _)| l.width != out.width || l.height != out.height)
        {
            let options = RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..Default::default()
            };
            let (l, rt) = (
                RenderTarget::with_options(&r.device, out.width, out.height, options.clone())?,
                RenderTarget::with_options(&r.device, out.width, out.height, options)?,
            );
            if let Some((scene, _, quad)) = &mut eyes.composite
                && let NodeKind::Mesh(m) = &mut scene.get_mut(*quad)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                Arc::make_mut(&mut m.program).rebind(
                    r,
                    &[],
                    &[(&l.view, &eyes.sampler), (&rt.view, &eyes.sampler)],
                )?;
            }
            eyes.targets = Some((l, rt));
        }
        for (camera, first) in [(left, true), (right, false)] {
            self.center_sky(s, camera)?;
            let eyes = self.eyes.as_ref().expect("eyes");
            let (l, rt) = eyes.targets.as_ref().expect("eye targets");
            r.render(s, camera, if first { l } else { rt })?;
        }
        let eyes = self.eyes.as_mut().expect("eyes");
        let (scene, camera, _) = eyes.composite.as_mut().ok_or(Error::Invalid("composite"))?;
        r.render(scene, *camera, out)?;
        Ok(true)
    }
    async fn composite(&mut self, r: &Renderer) -> Result<(Scene, Object3D, Object3D)> {
        let eyes = self.eyes.as_ref().ok_or(Error::Invalid("eyes"))?;
        let (l, rt) = eyes.targets.as_ref().ok_or(Error::Invalid("eye targets"))?;
        // Eye targets are stored top-down; the original samples its y-up targets at vUv.
        let uv = vec2(uv().x(), float(1.) - uv().y());
        let color = if self.id == 204 {
            call(
                "anaglyph",
                "fn anaglyph(uv:vec2<f32>)->vec4<f32>{let l=textureSample(tsl_texture_0,tsl_sampler_0,uv);let r=textureSample(tsl_texture_1,tsl_sampler_1,uv);let ml=mat3x3(vec3(0.456100,-0.0400822,-0.0152161),vec3(0.500484,-0.0378246,-0.0205971),vec3(0.176381,-0.0157589,-0.00546856));let mr=mat3x3(vec3(-0.0434706,0.378476,-0.0721527),vec3(-0.0879388,0.73364,-0.112961),vec3(-0.00155529,-0.0184503,1.2264));return vec4(clamp(ml*l.rgb+mr*r.rgb,vec3(0.0),vec3(1.0)),max(l.a,r.a));}",
                &[Type::Vec2],
                Type::Vec4,
                &[uv],
            )?
        } else {
            // mod( gl_FragCoord.y, 2.0 ) > 1.0 with WebGL's bottom-up fragment rows.
            call(
                "barrier",
                "fn barrier(uv:vec2<f32>,frag:vec2<f32>)->vec4<f32>{let y=u.point.y-frag.y;let l=textureSample(tsl_texture_0,tsl_sampler_0,uv);let r=textureSample(tsl_texture_1,tsl_sampler_1,uv);return select(r,l,y-2.0*floor(y/2.0)>1.0);}",
                &[Type::Vec2, Type::Vec2],
                Type::Vec4,
                &[uv, screen_coordinate()],
            )?
        };
        let graph = NodeMaterial::new(color);
        let program = Arc::new(
            ShaderProgram::with_projection(
                r,
                &graph.wgsl(2)?,
                &[],
                &[(&l.view, &eyes.sampler), (&rt.view, &eyes.sampler)],
                FULLSCREEN,
            )
            .await?,
        );
        let mut m = ShaderMaterial::new(program);
        m.properties.depth_test = false;
        m.properties.depth_write = false;
        let mut scene = Scene::new();
        scene.background = Color::BLACK;
        let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
            PerspectiveCamera::default(),
        )));
        // FullScreenQuad: one triangle, uv (0,0), (2,0), (0,2) at clip (-1,-1), (3,-1), (-1,3).
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            vec3s(vec![-0.5, -0.5, 0., 1.5, -0.5, 0., -0.5, 1.5, 0.])?,
        );
        g.set_attribute("normal", vec3s(vec![0., 0., 1., 0., 0., 1., 0., 0., 1.])?);
        g.set_attribute(
            "uv",
            Attribute::F32(BufferAttribute::new(
                vec![0., 0., 2., 0., 0., 2.],
                2,
                false,
            )?),
        );
        let quad = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Shader(m)),
        )));
        scene.get_mut(quad)?.frustum_culled = false;
        Ok((scene, camera, quad))
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.eyes.as_ref().and_then(|e| e.output.as_ref())
    }
    /// The stereo effects use `( clientX - innerWidth / 2 ) * 0.01`.
    pub fn gpu_pointer(&mut self, x: f64, y: f64) {
        let window = web_sys::window();
        let w = window
            .as_ref()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(512.);
        let h = window
            .as_ref()
            .and_then(|w| w.inner_height().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(512.);
        self.pointer = Vector2::new(x * w / 2. * 0.01, -y * h / 2. * 0.01);
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
        if self.id != 206 {
            return Ok(());
        }
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
        // The controls' change events re-render the on-demand scene.
        self.orbit.update();
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (206, 0) if (0.001..=0.01).contains(&value) => self.params[0] = value,
            (206, 1) if value >= 0. => self.params[1] = value,
            (206, 2) if (0. ..=3.).contains(&value) => self.params[2] = value,
            _ => return Err(Error::Invalid("stereo/loader parameter")),
        }
        self.dirty = true;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
