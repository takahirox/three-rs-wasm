//! BVH skeleton animation, framebuffer-to-texture copies, float render-target
//! readback, GPU picking and the instancing-performance methods from the pinned
//! WebGL examples.
use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::fetch;
use super::trackball_sprites::{Mode, SPRITE_VERTEX, Trackball};
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    tsl::*,
};
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
const PLAIN: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
/// `Color.setHSL` in the linear working color space.
fn hsl(h: f64, s: f64, l: f64) -> [f64; 3] {
    let h = (h % 1. + 1.) % 1.;
    let (s, l) = (s.clamp(0., 1.), l.clamp(0., 1.));
    if s == 0. {
        return [l; 3];
    }
    let p = if l <= 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let q = 2. * l - p;
    let f = |mut t: f64| {
        if t < 0. {
            t += 1.;
        }
        if t > 1. {
            t -= 1.;
        }
        if t < 1. / 6. {
            q + (p - q) * 6. * t
        } else if t < 0.5 {
            p
        } else if t < 2. / 3. {
            q + (p - q) * 6. * (2. / 3. - t)
        } else {
            q
        }
    };
    [f(h + 1. / 3.), f(h), f(h - 1. / 3.)]
}
/// `Quaternion.slerpFlat`.
fn slerp_flat(a: [f64; 4], b: [f64; 4], t: f64) -> [f64; 4] {
    let [mut x0, mut y0, mut z0, mut w0] = a;
    let [mut x1, mut y1, mut z1, mut w1] = b;
    if w0 != w1 || x0 != x1 || y0 != y1 || z0 != z1 {
        let mut dot = x0 * x1 + y0 * y1 + z0 * z1 + w0 * w1;
        if dot < 0. {
            x1 = -x1;
            y1 = -y1;
            z1 = -z1;
            w1 = -w1;
            dot = -dot;
        }
        let (mut s, mut t) = (1. - t, t);
        if dot < 0.9995 {
            let theta = dot.acos();
            let sin = theta.sin();
            s = (s * theta).sin() / sin;
            t = (t * theta).sin() / sin;
            x0 = x0 * s + x1 * t;
            y0 = y0 * s + y1 * t;
            z0 = z0 * s + z1 * t;
            w0 = w0 * s + w1 * t;
        } else {
            x0 = x0 * s + x1 * t;
            y0 = y0 * s + y1 * t;
            z0 = z0 * s + z1 * t;
            w0 = w0 * s + w1 * t;
            let f = 1. / (x0 * x0 + y0 * y0 + z0 * z0 + w0 * w0).sqrt();
            x0 *= f;
            y0 *= f;
            z0 *= f;
            w0 *= f;
        }
    }
    [x0, y0, z0, w0]
}
/// One BVH bone: its node, parent index, Float32 key times and position/quaternion values.
struct Bone {
    node: Object3D,
    parent: Option<usize>,
    times: Vec<f32>,
    positions: Vec<[f32; 3]>,
    rotations: Vec<[f32; 4]>,
    animated: bool,
}
/// BVHLoader.parse: the HIERARCHY as bones (end sites included) and MOTION as tracks.
fn parse_bvh(s: &mut Scene, text: &str) -> Result<Vec<Bone>> {
    let bad = |m: &'static str| Error::Asset(format!("BVH: {m}"));
    let mut lines = text
        .split(['\r', '\n'])
        .map(str::trim)
        .filter(|l| !l.is_empty());
    let mut next = || lines.next().ok_or(bad("truncated"));
    if next()? != "HIERARCHY" {
        return Err(bad("HIERARCHY expected"));
    }
    struct Node {
        offset: [f64; 3],
        channels: Vec<String>,
        parent: Option<usize>,
        end: bool,
        frames: Vec<([f64; 3], [f64; 4])>,
    }
    let mut nodes: Vec<Node> = vec![];
    // readNode, iteratively: every "{ ... }" block is one node in file order.
    let mut stack: Vec<usize> = vec![];
    let mut first = next()?.to_string();
    loop {
        let tokens: Vec<&str> = first.split_whitespace().collect();
        let end = tokens
            .first()
            .is_some_and(|t| t.eq_ignore_ascii_case("END"))
            && tokens
                .get(1)
                .is_some_and(|t| t.eq_ignore_ascii_case("SITE"));
        if next()? != "{" {
            return Err(bad("expected {"));
        }
        let offset_line: Vec<f64> = next()?
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse().unwrap_or(f64::NAN))
            .collect();
        let mut channels = vec![];
        if !end {
            let tokens: Vec<String> = next()?.split_whitespace().map(String::from).collect();
            let n: usize = tokens.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);
            channels = tokens.into_iter().skip(2).take(n).collect();
        }
        nodes.push(Node {
            offset: [offset_line[0], offset_line[1], offset_line[2]],
            channels,
            parent: stack.last().copied(),
            end,
            frames: vec![],
        });
        stack.push(nodes.len() - 1);
        // Children until "}"; closing braces pop back up the stack.
        loop {
            let line = next()?.to_string();
            if line == "}" {
                stack.pop();
                if stack.is_empty() {
                    break;
                }
            } else {
                first = line;
                break;
            }
        }
        if stack.is_empty() {
            break;
        }
    }
    if next()? != "MOTION" {
        return Err(bad("MOTION expected"));
    }
    let frames: usize = next()?
        .split_whitespace()
        .nth(1)
        .and_then(|v| v.parse().ok())
        .ok_or(bad("frames"))?;
    let frame_time: f64 = next()?
        .split_whitespace()
        .nth(2)
        .and_then(|v| v.parse().ok())
        .ok_or(bad("frame time"))?;
    let mut times = vec![];
    for i in 0..frames {
        let values: Vec<f64> = next()?
            .split_whitespace()
            .map(|v| v.parse().unwrap_or(f64::NAN))
            .collect();
        let mut values = values.into_iter();
        // readFrameData: depth-first file order, which is the node order.
        for node in nodes.iter_mut().filter(|n| !n.end) {
            let (mut p, mut q) = ([0.; 3], Quaternion::IDENTITY);
            for c in &node.channels {
                let v = values.next().unwrap_or(f64::NAN);
                match c.as_str() {
                    "Xposition" => p[0] = v,
                    "Yposition" => p[1] = v,
                    "Zposition" => p[2] = v,
                    "Xrotation" => q *= Quaternion::from_axis_angle(Vector3::X, v * PI / 180.),
                    "Yrotation" => q *= Quaternion::from_axis_angle(Vector3::Y, v * PI / 180.),
                    "Zrotation" => q *= Quaternion::from_axis_angle(Vector3::Z, v * PI / 180.),
                    _ => {}
                }
            }
            node.frames.push((p, [q.x, q.y, q.z, q.w]));
        }
        times.push((i as f64 * frame_time) as f32);
    }
    let mut bones: Vec<Bone> = Vec::with_capacity(nodes.len());
    for node in &nodes {
        let h = s.insert(NodeKind::Group);
        s.get_mut(h)?.position = Vector3::from_array(node.offset);
        if let Some(parent) = node.parent {
            s.add(bones[parent].node, h)?;
        }
        bones.push(Bone {
            node: h,
            parent: node.parent,
            times: if node.end { vec![] } else { times.clone() },
            positions: node
                .frames
                .iter()
                .map(|(p, _)| [0, 1, 2].map(|k| (p[k] + node.offset[k]) as f32))
                .collect(),
            rotations: node
                .frames
                .iter()
                .map(|(_, q)| q.map(|v| v as f32))
                .collect(),
            animated: !node.end,
        });
    }
    Ok(bones)
}
/// Resident async readback of one Rgba32Float texel.
struct FloatReadback {
    buffer: wgpu::Buffer,
    state: Arc<Mutex<Option<bool>>>,
}
impl FloatReadback {
    fn new(r: &Renderer) -> Self {
        Self {
            buffer: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("resident float readback"),
                size: 256,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            state: Arc::new(Mutex::new(None)),
        }
    }
    /// Copy texel ( x, y ) and map it, unless a previous read is still in flight.
    fn begin(&self, r: &Renderer, texture: &wgpu::Texture, x: u32, y: u32) {
        let mut state = self.state.lock().expect("readback state");
        if state.is_some() {
            return;
        }
        let mut encoder = r
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("float readback"),
            });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(1),
                },
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        r.queue.submit([encoder.finish()]);
        *state = Some(false);
        let done = self.state.clone();
        self.buffer
            .slice(..16)
            .map_async(wgpu::MapMode::Read, move |result| {
                *done.lock().expect("readback state") = Some(result.is_ok());
            });
    }
    /// The four floats of a finished read.
    fn take(&self) -> Option<[f32; 4]> {
        let mut state = self.state.lock().expect("readback state");
        if *state != Some(true) {
            return None;
        }
        let values = {
            let view = self.buffer.slice(..16).get_mapped_range();
            let v: &[f32] = bytemuck::cast_slice(&view);
            [v[0], v[1], v[2], v[3]]
        };
        self.buffer.unmap();
        *state = None;
        Some(values)
    }
}
struct Pick {
    scene: Scene,
    camera: Object3D,
    target: RenderTarget,
    readback: FloatReadback,
    data: Vec<(Vector3, Vector3, Vector3)>,
    highlight: Object3D,
}
struct Float {
    rtt: Scene,
    rtt_camera: Object3D,
    screen: Scene,
    screen_camera: Object3D,
    target: RenderTarget,
    quad: Object3D,
    meshes: [Object3D; 2],
    readback: FloatReadback,
    value: f64,
    delta: f64,
}
struct Framebuffer {
    hud: Scene,
    hud_camera: Object3D,
    sprite: Object3D,
    texture: wgpu::Texture,
    colors: GpuBuffer,
    count: usize,
    output: Option<RenderTarget>,
    offset: i64,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    controls: Option<Controls>,
    trackball: Option<Trackball>,
    pointer: Vector2,
    // BVH
    bones: Vec<Bone>,
    duration: f64,
    helper: Option<(GpuBuffer, Vec<(usize, usize)>)>,
    // Framebuffer texture
    framebuffer: Option<Framebuffer>,
    // Float readback
    float: Option<Float>,
    // GPU picking
    pick: Option<Pick>,
    // Instancing performance
    params: [f32; 2],
    built: Option<(usize, usize)>,
    suzanne: Option<Arc<BufferGeometry>>,
    meshes: Vec<Object3D>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            228 => (60., 1., 1000., Vector3::new(0., 200., 300.)),
            229 => (70., 1., 1000., Vector3::new(0., 0., 20.)),
            230 => (50., 1., 1000., Vector3::ZERO),
            231 => (70., 1., 10000., Vector3::new(0., 0., 1000.)),
            _ => (70., 1., 100., Vector3::new(0., 0., 30.)),
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
            seed: 186,
            controls: None,
            trackball: None,
            pointer: Vector2::ZERO,
            bones: vec![],
            duration: 0.,
            helper: None,
            framebuffer: None,
            float: None,
            pick: None,
            params: [0., 1000.],
            built: None,
            suzanne: None,
            meshes: vec![],
        };
        match id {
            228 => d.bvh(s, r).await?,
            229 => d.framebuffer_scene(s, r).await?,
            230 => d.float_scene(r).await?,
            231 => d.pick_scene(s, c, r).await?,
            _ => d.performance_scene(s).await?,
        }
        if let Some(controls) = &mut d.controls {
            controls.update(s, c)?;
        }
        Ok(d)
    }
    async fn bvh(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0xeeeeee);
        let grid = super::interactive_scenes::grid_helper(400., 10, 0x444444, 0x888888)?;
        s.insert(NodeKind::Line(grid));
        let text = String::from_utf8_lossy(&fetch("/web/gallery/assets/bvh/pirouette.bvh").await?)
            .into_owned();
        self.bones = parse_bvh(s, &text)?;
        self.duration = self
            .bones
            .iter()
            .filter_map(|b| b.times.last())
            .fold(0f64, |a, &t| a.max(t as f64));
        // SkeletonHelper: one segment per bone whose parent is a bone, child then parent,
        // colored 0x0000ff and 0x00ff00; depthTest and depthWrite off, transparent.
        let pairs: Vec<(usize, usize)> = self
            .bones
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.parent.map(|p| (i, p)))
            .collect();
        let buffer =
            GpuBuffer::zeroed(r, (pairs.len() * 2 * 12).max(12) as u64, BufferAccess::Read)?;
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let i=tsl_vertex_index;let p=vec3(tsl_attribute_0[i*3u],tsl_attribute_0[i*3u+1u],tsl_attribute_0[i*3u+2u]);out.clip=u.projection*u.view*vec4(p,1.0);out.local_normal=select(vec3(0.0,1.0,0.0),vec3(0.0,0.0,1.0),i%2u==0u);return out;}";
        let source = NodeMaterial::new(vec4(normal_local(), float(1.)))
            .wgsl_with_storage(0, &[Type::Float])?;
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(r, &source, &[&buffer], &[], projection).await?,
        ));
        m.properties.depth_test = false;
        m.properties.depth_write = false;
        m.properties.transparent = true;
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(vec![0.; pairs.len() * 6])?);
        let helper = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Shader(m)),
            segments: true,
        }));
        s.get_mut(helper)?.frustum_culled = false;
        self.helper = Some((buffer, pairs));
        self.controls = Some(Controls::new(None, (300., 700.), PI, true));
        Ok(())
    }
    fn animate_bvh(&mut self, r: &Renderer, s: &mut Scene) -> Result<()> {
        // AnimationMixer LoopRepeat over the clip duration; linear positions, slerped rotations.
        let mut time = self.time;
        if self.duration > 0. && time >= self.duration {
            time -= self.duration * (time / self.duration).floor();
        }
        for b in self.bones.iter().filter(|b| b.animated) {
            let times = &b.times;
            let (position, rotation) = match times.iter().position(|&k| k as f64 > time) {
                Some(0) => (
                    b.positions[0].map(|v| v as f64),
                    b.rotations[0].map(|v| v as f64),
                ),
                None => (
                    b.positions[times.len() - 1].map(|v| v as f64),
                    b.rotations[times.len() - 1].map(|v| v as f64),
                ),
                Some(i) => {
                    let (t0, t1) = (times[i - 1] as f64, times[i] as f64);
                    let a = (time - t0) / (t1 - t0);
                    let (p0, p1) = (b.positions[i - 1], b.positions[i]);
                    (
                        [0, 1, 2].map(|k| p0[k] as f64 * (1. - a) + p1[k] as f64 * a),
                        slerp_flat(
                            b.rotations[i - 1].map(|v| v as f64),
                            b.rotations[i].map(|v| v as f64),
                            a,
                        ),
                    )
                }
            };
            let n = s.get_mut(b.node)?;
            n.position = Vector3::from_array(position);
            n.quaternion =
                Quaternion::from_xyzw(rotation[0], rotation[1], rotation[2], rotation[3]);
        }
        if let Some(root) = self.bones.first() {
            s.update_world_matrix(root.node, false, true)?;
        }
        if let Some((buffer, pairs)) = &self.helper {
            let mut data = Vec::with_capacity(pairs.len() * 6);
            for &(child, parent) in pairs {
                for i in [child, parent] {
                    let p = s.get(self.bones[i].node)?.matrix_world.w_axis;
                    data.extend([p.x as f32, p.y as f32, p.z as f32]);
                }
            }
            // The original rewrites these positions on the CPU every frame as well.
            buffer.write(r, 0, bytemuck::cast_slice(&data))?;
        }
        Ok(())
    }
    async fn framebuffer_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        // GeometryUtils.gosper( 8 ): four L-system steps drawn as a centered line strip.
        let mut input = String::from("A");
        for _ in 0..4 {
            let mut output = String::new();
            for ch in input.chars() {
                match ch {
                    'A' => output.push_str("A+BF++BF-FA--FAFA-BF+"),
                    'B' => output.push_str("-FA+BFBF++BF+FA--FA-B"),
                    c => output.push(c),
                }
            }
            input = output;
        }
        let (mut x, mut y, mut angle) = (0f64, 0f64, 0f64);
        let mut points = vec![0f32, 0., 0.];
        for ch in input.chars() {
            match ch {
                '+' => angle += PI / 3.,
                '-' => angle -= PI / 3.,
                'F' => {
                    x += 8. * angle.cos();
                    y += -8. * angle.sin();
                    points.extend([x as f32, y as f32, 0.]);
                }
                _ => {}
            }
        }
        let count = points.len() / 3;
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(points)?);
        g.center()?;
        // The DynamicDrawUsage color attribute: three floats per vertex, rewritten each frame.
        let colors = GpuBuffer::zeroed(r, count as u64 * 12, BufferAccess::Read)?;
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let i=tsl_vertex_index*3u;out.local_normal=vec3(tsl_attribute_0[i],tsl_attribute_0[i+1u],tsl_attribute_0[i+2u]);return out;}";
        let source = NodeMaterial::new(vec4(normal_local(), float(1.)))
            .wgsl_with_storage(0, &[Type::Float])?;
        let m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(r, &source, &[&colors], &[], projection).await?,
        ));
        let line = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Shader(m)),
            segments: false,
        }));
        s.get_mut(line)?.scale = Vector3::splat(0.05);
        // FramebufferTexture( 128 × dpr ): nearest filtering, no mipmaps, raw copied values.
        let (_, _, dpr) = viewport_css();
        let size = (128. * dpr) as u32;
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("framebuffer texture"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("framebuffer texture sampler"),
            ..Default::default()
        });
        // The sprite samples the copy (rows top-down here, bottom-up in GL) and the output
        // encodes it again, as SpriteMaterial's colorspace_fragment does in the original.
        let color = WgslFn::new(
            "copied",
            "fn copied(t:vec4<f32>)->vec4<f32>{return t;}",
            &[Type::Vec4],
            Type::Vec4,
        )?
        .call(&[crate::tsl::Texture::External(0).sample(vec2(uv().x(), float(1.) - uv().y()))]);
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(1)?,
            &[],
            &[(&view, &sampler)],
            SPRITE_VERTEX,
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.properties.transparent = true;
        m.properties.fog = false;
        m.uniforms[0] = [0., 1., 0.5, 0.5];
        m.uniforms[1] = [1., 1., 1., 1.];
        m.uniforms[2] = [1., 1., 0., 0.];
        m.uniforms[3] = [0.; 4];
        let mut hud = Scene::new();
        hud.background = Color::BLACK;
        let hud_camera = hud.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            near: 1.,
            far: 10.,
            ..Default::default()
        })));
        hud.get_mut(hud_camera)?.position = Vector3::new(0., 0., 10.);
        let mut quad = BufferGeometry::default();
        quad.set_attribute(
            "position",
            vec3s(vec![
                -0.5, -0.5, 0., 0.5, -0.5, 0., 0.5, 0.5, 0., -0.5, 0.5, 0.,
            ])?,
        );
        quad.set_attribute("normal", vec3s([0., 0., 1.].repeat(4))?);
        quad.set_attribute(
            "uv",
            Attribute::F32(BufferAttribute::new(
                vec![0., 0., 1., 0., 1., 1., 0., 1.],
                2,
                false,
            )?),
        );
        quad.set_index(Some(vec![0, 1, 2, 0, 2, 3]));
        let sprite = hud.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(quad),
            Arc::new(Material::Shader(m)),
        )));
        let n = hud.get_mut(sprite)?;
        n.scale = Vector3::new(size as f64, size as f64, 1.);
        n.frustum_culled = false;
        self.framebuffer = Some(Framebuffer {
            hud,
            hud_camera,
            sprite,
            texture,
            colors,
            count,
            output: None,
            offset: 0,
        });
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::ZERO);
        self.controls = Some(controls);
        Ok(())
    }
    async fn float_scene(&mut self, r: &Renderer) -> Result<()> {
        let (w, h, _) = viewport_css();
        let mut rtt = Scene::new();
        rtt.background = Color::BLACK;
        let rtt_camera = rtt.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -w / 2.,
            right: w / 2.,
            top: h / 2.,
            bottom: -h / 2.,
            near: 1.,
            far: 1000.,
            ..Default::default()
        })));
        rtt.get_mut(rtt_camera)?.position = Vector3::new(0., 0., 500.);
        for (color, intensity, z) in [(0xffffff, 3., 1.), (0xffd5d5, 4.5, -1.)] {
            let light = rtt.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(color),
                intensity,
                target: Vector3::ZERO,
            }));
            rtt.get_mut(light)?.position = Vector3::new(0., 0., z);
        }
        // fragment_shader_pass_1: ( vUv.x ≥ 0.5 ? x : 0, vUv.y ≥ 0.5 ? y : 0, time ), raw.
        let pass = WgslFn::new(
            "pass_1",
            "fn pass_1(v:vec2<f32>)->vec4<f32>{let r=select(v.x,0.0,v.y<0.5);let g=select(v.y,0.0,v.x<0.5);return vec4(r,g,u.custom[0].x,1.0);}",
            &[Type::Vec2],
            Type::Vec4,
        )?
        .call(&[uv()]);
        let program =
            ShaderProgram::with_projection(r, &NodeMaterial::new(pass).wgsl(0)?, &[], &[], PLAIN)
                .await?;
        let plane = Arc::new(PlaneGeometry::build(w, h, 1, 1)?);
        let quad = rtt.insert(NodeKind::Mesh(Mesh::new(
            plane.clone(),
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        rtt.get_mut(quad)?.position.z = -100.;
        let torus = Arc::new(TorusGeometry::build(
            100.,
            25.,
            15,
            30,
            std::f64::consts::TAU,
            0.,
            std::f64::consts::TAU,
        )?);
        let mut meshes = vec![];
        for (color, specular, position, scale) in [
            (0x9c9c9c, 0xffaa00, Vector3::new(0., 0., 100.), 1.5),
            (0x9c0000, 0xff2200, Vector3::new(0., 150., 100.), 0.75),
        ] {
            let mut m = MeshPhongMaterial {
                specular: Color::from_hex(specular),
                shininess: 5.,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(color);
            let h = rtt.insert(NodeKind::Mesh(Mesh::new(
                torus.clone(),
                Arc::new(Material::Phong(m)),
            )));
            let n = rtt.get_mut(h)?;
            n.position = position;
            n.scale = Vector3::splat(scale);
            meshes.push(h);
        }
        let mut screen = Scene::new();
        screen.background = Color::BLACK;
        let screen_camera =
            screen.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                left: -w / 2.,
                right: w / 2.,
                top: h / 2.,
                bottom: -h / 2.,
                near: 1.,
                far: 1000.,
                ..Default::default()
            })));
        screen.get_mut(screen_camera)?.position = Vector3::new(0., 0., 500.);
        // WebGLRenderTarget( innerWidth, innerHeight ): CSS-pixel size, Float32. The example
        // has no resize handler, so the target, cameras and plane keep their initial size.
        let target = RenderTarget::with_options(
            &r.device,
            w as u32,
            h as u32,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba32Float,
                ..Default::default()
            },
        )?;
        self.screen_quad(r, &mut screen, plane, &target).await?;
        self.float = Some(Float {
            rtt,
            rtt_camera,
            screen,
            screen_camera,
            target,
            quad,
            meshes: [meshes[0], meshes[1]],
            readback: FloatReadback::new(r),
            value: 0.,
            delta: 0.01,
        });
        Ok(())
    }
    /// fragment_shader_screen: the float target at vUv, then colorspace_fragment's encoding.
    async fn screen_quad(
        &self,
        r: &Renderer,
        screen: &mut Scene,
        plane: Arc<BufferGeometry>,
        target: &RenderTarget,
    ) -> Result<Object3D> {
        let sample = crate::tsl::Texture::External(0).sample(vec2(uv().x(), float(1.) - uv().y()));
        // Float32 targets are not filterable: nearest sampling, as the magnification filter is.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("float target sampler"),
            ..Default::default()
        });
        let source = NodeMaterial::new(sample).wgsl_with_texture_types(&[Type::Texture], &[])?;
        let program = ShaderProgram::with_texture_dimensions(
            r,
            &source,
            &[],
            &[(&target.view, &sampler)],
            &[wgpu::TextureViewDimension::D2],
            &[wgpu::TextureSampleType::Float { filterable: false }],
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.properties.depth_write = false;
        let h = screen.insert(NodeKind::Mesh(Mesh::new(
            plane,
            Arc::new(Material::Shader(m)),
        )));
        screen.get_mut(h)?.position.z = -100.;
        Ok(h)
    }
    async fn pick_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.background = Color::WHITE;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xcccccc),
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0., 500., 2000.);
        let (mut positions, mut normals, mut uvs, mut colors, mut index) =
            (vec![], vec![], vec![], vec![], vec![]);
        let base = BoxGeometry::build(1., 1., 1.)?;
        let get = |g: &BufferGeometry, name: &str| match g.attributes.get(name) {
            Some(Attribute::F32(a)) => a.array().to_vec(),
            _ => vec![],
        };
        let (bp, bn, bu) = (
            get(&base, "position"),
            get(&base, "normal"),
            get(&base, "uv"),
        );
        let bi: Vec<u32> = base.index.clone().unwrap_or_default();
        let mut data = Vec::with_capacity(5000);
        for _ in 0..5000 {
            let p = Vector3::new(
                random(&mut self.seed) * 10000. - 5000.,
                random(&mut self.seed) * 6000. - 3000.,
                random(&mut self.seed) * 8000. - 4000.,
            );
            let e = Vector3::new(
                random(&mut self.seed) * 2. * PI,
                random(&mut self.seed) * 2. * PI,
                random(&mut self.seed) * 2. * PI,
            );
            let sc = Vector3::new(
                random(&mut self.seed) * 200. + 100.,
                random(&mut self.seed) * 200. + 100.,
                random(&mut self.seed) * 200. + 100.,
            );
            let q = Euler {
                angles: e,
                order: EulerOrder::XYZ,
            }
            .quaternion();
            let m = Matrix4::from_scale_rotation_translation(sc, q, p);
            let normal_matrix = Matrix3::from_mat4(m).inverse().transpose();
            let color = Color::from_hex((random(&mut self.seed) * 16777215.).floor() as u32);
            let offset = (positions.len() / 3) as u32;
            for v in bp.chunks(3) {
                let w = m.transform_point3(Vector3::new(v[0] as f64, v[1] as f64, v[2] as f64));
                positions.extend([w.x as f32, w.y as f32, w.z as f32]);
                colors.extend(color.0.to_array().map(|c| c as f32));
            }
            for v in bn.chunks(3) {
                let n = (normal_matrix * Vector3::new(v[0] as f64, v[1] as f64, v[2] as f64))
                    .normalize_or_zero();
                normals.extend([n.x as f32, n.y as f32, n.z as f32]);
            }
            uvs.extend_from_slice(&bu);
            index.extend(bi.iter().map(|i| i + offset));
            data.push((p, e, sc));
        }
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(positions)?);
        g.set_attribute("normal", vec3s(normals)?);
        g.set_attribute("uv", Attribute::F32(BufferAttribute::new(uvs, 2, false)?));
        g.set_attribute("color", vec3s(colors)?);
        g.set_index(Some(index));
        let g = Arc::new(g);
        let mut m = MeshPhongMaterial {
            shininess: 0.,
            ..Default::default()
        };
        m.properties.flat_shading = true;
        m.properties.vertex_colors = true;
        s.insert(NodeKind::Mesh(Mesh::new(
            g.clone(),
            Arc::new(Material::Phong(m)),
        )));
        let mut lambert = MeshLambertMaterial::default();
        lambert.properties.color = Color::from_hex(0xffff00);
        let highlight = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 1., 1.)?),
            Arc::new(Material::Lambert(lambert)),
        )));
        // The picking pass: the box id (24 vertices each) as a flat float, cleared to -1.
        let mut scene = Scene::new();
        scene.background = Color::linear(-1., -1., -1.);
        let camera = scene.insert(NodeKind::Camera(s.camera(c)?.0.clone()));
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.local_normal=vec3(f32(tsl_vertex_index/24u));return out;}";
        let id = NodeMaterial::new(vec4(normal_local(), float(1.)));
        let program = ShaderProgram::with_projection(r, &id.wgsl(0)?, &[], &[], projection).await?;
        let pick_mesh = scene.insert(NodeKind::Mesh(Mesh::new(
            g,
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        scene.get_mut(pick_mesh)?.frustum_culled = false;
        let target = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba32Float,
                ..Default::default()
            },
        )?;
        self.pick = Some(Pick {
            scene,
            camera,
            target,
            readback: FloatReadback::new(r),
            data,
            highlight,
        });
        let (w, h, _) = viewport_css();
        let mut t = Trackball::new(s, c, Vector2::new(w, h))?.with_pan_speed(0.8);
        t.static_moving = true;
        self.trackball = Some(t);
        Ok(())
    }
    async fn performance_scene(&mut self, s: &mut Scene) -> Result<()> {
        s.background = Color::WHITE;
        #[derive(serde::Deserialize)]
        struct Array<T> {
            array: Vec<T>,
        }
        #[derive(serde::Deserialize)]
        struct Attributes {
            position: Array<f32>,
        }
        #[derive(serde::Deserialize)]
        struct Data {
            attributes: Attributes,
            index: Array<u16>,
        }
        #[derive(serde::Deserialize)]
        struct Asset {
            data: Data,
        }
        let asset: Asset = serde_json::from_slice(
            &fetch("/web/gallery/assets/suzanne_buffergeometry.json").await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(asset.data.attributes.position.array)?);
        g.set_index(Some(
            asset.data.index.array.into_iter().map(u32::from).collect(),
        ));
        g.compute_vertex_normals()?;
        self.suzanne = Some(Arc::new(g));
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        // autoRotate at the default speed 2.
        controls.auto_rotate = Some(2.);
        self.controls = Some(controls);
        Ok(())
    }
    /// randomizeMatrix: position, Quaternion.random() and a uniform scale.
    fn random_matrix(&mut self) -> Matrix4 {
        let p = Vector3::new(
            random(&mut self.seed) * 40. - 20.,
            random(&mut self.seed) * 40. - 20.,
            random(&mut self.seed) * 40. - 20.,
        );
        let theta1 = 2. * PI * random(&mut self.seed);
        let theta2 = 2. * PI * random(&mut self.seed);
        let x0 = random(&mut self.seed);
        let (r1, r2) = ((1. - x0).sqrt(), x0.sqrt());
        let q = Quaternion::from_xyzw(
            r1 * theta1.sin(),
            r1 * theta1.cos(),
            r2 * theta2.sin(),
            r2 * theta2.cos(),
        );
        let sc = random(&mut self.seed);
        Matrix4::from_scale_rotation_translation(Vector3::splat(sc), q, p)
    }
    /// initMesh(): clean() and rebuild with the selected method, as the original does.
    fn rebuild(&mut self, s: &mut Scene) -> Result<()> {
        let method = self.params[0] as usize;
        let count = (self.params[1].max(1.) as usize).min(10000);
        if self.built == Some((method, count)) {
            return Ok(());
        }
        for h in self.meshes.drain(..) {
            s.dispose(h)?;
        }
        let g = self.suzanne.clone().ok_or(Error::Invalid("suzanne"))?;
        let material = Arc::new(Material::Normal(MeshNormalMaterial::default()));
        match method {
            0 => {
                let h = s.insert(NodeKind::Mesh(Mesh::new(g, material)));
                let instances = (0..count)
                    .map(|_| Instance {
                        matrix: self.random_matrix(),
                        color: Color::WHITE,
                    })
                    .collect();
                s.get_mut(h)?.instances = instances;
                self.meshes.push(h);
            }
            1 => {
                let p = match g.attributes.get("position") {
                    Some(Attribute::F32(a)) => a.array().to_vec(),
                    _ => vec![],
                };
                let n = match g.attributes.get("normal") {
                    Some(Attribute::F32(a)) => a.array().to_vec(),
                    _ => vec![],
                };
                let index = g.index.clone().unwrap_or_default();
                let (mut mp, mut mn, mut mi) = (vec![], vec![], vec![]);
                for _ in 0..count {
                    let m = self.random_matrix();
                    let nm = Matrix3::from_mat4(m).inverse().transpose();
                    let offset = (mp.len() / 3) as u32;
                    for v in p.chunks(3) {
                        let w =
                            m.transform_point3(Vector3::new(v[0] as f64, v[1] as f64, v[2] as f64));
                        mp.extend([w.x as f32, w.y as f32, w.z as f32]);
                    }
                    for v in n.chunks(3) {
                        let w = (nm * Vector3::new(v[0] as f64, v[1] as f64, v[2] as f64))
                            .normalize_or_zero();
                        mn.extend([w.x as f32, w.y as f32, w.z as f32]);
                    }
                    mi.extend(index.iter().map(|i| i + offset));
                }
                let mut merged = BufferGeometry::default();
                merged.set_attribute("position", vec3s(mp)?);
                merged.set_attribute("normal", vec3s(mn)?);
                merged.set_index(Some(mi));
                self.meshes
                    .push(s.insert(NodeKind::Mesh(Mesh::new(Arc::new(merged), material))));
            }
            _ => {
                for _ in 0..count {
                    let m = self.random_matrix();
                    let h = s.insert(NodeKind::Mesh(Mesh::new(g.clone(), material.clone())));
                    let (scale, rotation, translation) = m.to_scale_rotation_translation();
                    let n = s.get_mut(h)?;
                    n.position = translation;
                    n.quaternion = rotation;
                    n.scale = scale;
                    self.meshes.push(h);
                }
            }
        }
        self.built = Some((method, count));
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, r: &Renderer, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        match self.id {
            228 => self.animate_bvh(r, s)?,
            229 => {
                // updateColors(): setHSL( ( offset + i ) % l / l, 1, 0.5 ), then offset -= 25.
                let f = self
                    .framebuffer
                    .as_mut()
                    .ok_or(Error::Invalid("framebuffer"))?;
                let l = f.count as i64;
                let colors: Vec<[f32; 3]> = (0..l)
                    .map(|i| {
                        // JavaScript's remainder keeps the sign of the negative offset.
                        let h = ((f.offset + i) % l) as f64 / l as f64;
                        hsl(h, 1., 0.5).map(|v| v as f32)
                    })
                    .collect();
                f.colors.write(r, 0, bytemuck::cast_slice(&colors))?;
                f.offset -= 25 * steps as i64;
                let (w, h, _) = viewport_css();
                if let NodeKind::Camera(Camera::Orthographic(o)) =
                    &mut f.hud.get_mut(f.hud_camera)?.kind
                {
                    o.left = -w / 2.;
                    o.right = w / 2.;
                    o.top = h / 2.;
                    o.bottom = -h / 2.;
                }
                // updateSpritePosition(): the top-left corner, in CSS pixels.
                let half = f.hud.get(f.sprite)?.scale.x / 2.;
                f.hud.get_mut(f.sprite)?.position = Vector3::new(-w / 2. + half, h / 2. - half, 1.);
            }
            230 => {
                let fl = self.float.as_mut().ok_or(Error::Invalid("float"))?;
                for _ in 0..steps {
                    if fl.value > 1. || fl.value < 0. {
                        fl.delta *= -1.;
                    }
                    fl.value += fl.delta;
                }
                let time = t * 1.5;
                fl.rtt.get_mut(fl.meshes[0])?.quaternion = Quaternion::from_rotation_y(-time);
                fl.rtt.get_mut(fl.meshes[1])?.quaternion =
                    Quaternion::from_rotation_y(-time + PI / 2.);
                let quad = fl.quad;
                if let NodeKind::Mesh(m) = &mut fl.rtt.get_mut(quad)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0][0] = fl.value as f32;
                }
            }
            231 => {
                let (w, h, _) = viewport_css();
                if let Some(tb) = &mut self.trackball {
                    tb.screen = Vector2::new(w, h);
                    for _ in 0..steps {
                        tb.update(s)?;
                    }
                }
                // readRenderTargetPixelsAsync(): apply the latest finished pick.
                let pick = self.pick.as_mut().ok_or(Error::Invalid("pick"))?;
                if let Some(v) = pick.readback.take() {
                    let id = v[0].round() as i64;
                    let n = s.get_mut(pick.highlight)?;
                    if id >= 0 && (id as usize) < pick.data.len() {
                        let (p, e, sc) = pick.data[id as usize];
                        n.position = p;
                        n.quaternion = Euler {
                            angles: e,
                            order: EulerOrder::XYZ,
                        }
                        .quaternion();
                        n.scale = sc + Vector3::splat(10.);
                        n.visible = true;
                    } else {
                        n.visible = false;
                    }
                }
            }
            _ => {
                self.rebuild(s)?;
                if let Some(controls) = &mut self.controls {
                    for _ in 0..steps {
                        controls.frame_update(s, c)?;
                    }
                }
            }
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
        match self.id {
            229 => {
                let f = self
                    .framebuffer
                    .as_mut()
                    .ok_or(Error::Invalid("framebuffer"))?;
                if f.output.as_ref().is_none_or(|t| {
                    t.width != out.width
                        || t.height != out.height
                        || t.options.samples != out.options.samples
                }) {
                    let mut options = out.options.clone();
                    options.store_multisampled_color_buffer = true;
                    f.output = Some(RenderTarget::with_options(
                        &r.device, out.width, out.height, options,
                    )?);
                }
                let target = f.output.as_mut().expect("output");
                target.set_load_color(false);
                r.render(s, c, target)?;
                // copyFramebufferToTexture( texture, center ): the resolved canvas pixels.
                let size = f.texture.width();
                // The GL origin counts rows from the bottom: the top row is height - y - size.
                let x = (target.width as f64 / 2. - size as f64 / 2.).max(0.) as u32;
                let y_gl = (target.height as f64 / 2. - size as f64 / 2.).max(0.) as u32;
                let y = target.height.saturating_sub(y_gl + size);
                let mut encoder =
                    r.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("framebuffer copy"),
                        });
                encoder.copy_texture_to_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &target.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d { x, y, z: 0 },
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::TexelCopyTextureInfo {
                        texture: &f.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::Extent3d {
                        width: size.min(target.width),
                        height: size.min(target.height),
                        depth_or_array_layers: 1,
                    },
                );
                r.queue.submit([encoder.finish()]);
                // clearDepth(), then the sprite over the scene.
                target.set_load_color(true);
                r.render(&mut f.hud, f.hud_camera, target)?;
                target.set_load_color(false);
                Ok(true)
            }
            230 => {
                let fl = self.float.as_mut().ok_or(Error::Invalid("float"))?;
                let target = &fl.target;
                let (tw, th) = (target.width, target.height);
                r.render(&mut fl.rtt, fl.rtt_camera, target)?;
                r.render(&mut fl.screen, fl.screen_camera, out)?;
                // readRenderTargetPixels at the mouse, rows counted from the bottom in GL.
                // x = clientX and y = innerHeight - clientY, with the initial innerHeight.
                let (w, h, _) = viewport_css();
                let client = Vector2::new(
                    (self.pointer.x + 1.) / 2. * w,
                    (1. - self.pointer.y) / 2. * h,
                );
                let x = (client.x.max(0.) as u32).min(tw - 1);
                let y_gl = ((th as f64 - client.y).max(0.) as u32).min(th - 1);
                let y = th - 1 - y_gl;
                fl.readback.begin(r, &target.texture, x, y);
                if let Some(v) = fl.readback.take() {
                    show_values(v);
                }
                Ok(true)
            }
            231 => {
                let pick = self.pick.as_mut().ok_or(Error::Invalid("pick"))?;
                // camera.setViewOffset( width, height, floor( pointer × dpr ), 1, 1 ).
                let (_, _, dpr) = viewport_css();
                let camera = s.get(c)?.clone();
                let node = pick.scene.get_mut(pick.camera)?;
                node.position = camera.position;
                node.quaternion = camera.quaternion;
                node.up = camera.up;
                if let (
                    NodeKind::Camera(Camera::Perspective(p)),
                    NodeKind::Camera(Camera::Perspective(main)),
                ) = (&mut node.kind, &camera.kind)
                {
                    *p = main.clone();
                    p.view = Some(ViewOffset {
                        full_width: out.width as f64,
                        full_height: out.height as f64,
                        offset_x: (self.pointer.x * dpr).floor(),
                        offset_y: (self.pointer.y * dpr).floor(),
                        width: 1.,
                        height: 1.,
                    });
                }
                r.render(&mut pick.scene, pick.camera, &pick.target)?;
                pick.readback.begin(r, &pick.target.texture, 0, 0);
                Ok(false)
            }
            _ => Ok(false),
        }
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        match self.id {
            229 => self.framebuffer.as_ref().and_then(|f| f.output.as_ref()),
            _ => None,
        }
    }
    pub fn gpu_pointer(&mut self, x: f64, y: f64) {
        if self.id == 230 {
            self.pointer = Vector2::new(x, y);
        }
    }
    /// Absolute CSS-pixel pointer events: TrackballControls and the picking pointer.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if self.id != 231 {
            return;
        }
        if kind < 10 {
            self.pointer = Vector2::new(x, y);
        }
        if let Some(t) = &mut self.trackball {
            match kind {
                10..=19 => t.down(kind - 10, x, y),
                20..=29 => t.state = Mode::None,
                _ => t.moved(x, y),
            }
        }
    }
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
        if let Some(t) = &mut self.trackball {
            if wheel != 0. {
                t.zoom_start.y -= wheel * 0.00025;
            }
            return Ok(());
        }
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            // The framebuffer example sets enablePan = false.
            if self.id == 229 {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(t) = &mut self.trackball {
            if !down {
                t.key_state = Mode::None;
            } else if t.key_state == Mode::None {
                t.key_state = match code {
                    65 => Mode::Rotate,
                    83 => Mode::Zoom,
                    68 => Mode::Pan,
                    _ => Mode::None,
                };
            }
        }
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (232, 0) if (0. ..=2.).contains(&value) => self.params[0] = value,
            (232, 1) if (1. ..=10000.).contains(&value) => self.params[1] = value.round(),
            _ => return Err(Error::Invalid("picking/buffers parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// valueNode.innerHTML = 'r:' + read[ 0 ] + '<br/>g:' + read[ 1 ] + '<br/>b:' + read[ 2 ].
fn show_values(v: [f32; 4]) {
    if let Some(node) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("values"))
    {
        node.set_inner_html(&format!(
            "r:{}<br/>g:{}<br/>b:{}",
            v[0] as f64, v[1] as f64, v[2] as f64
        ));
    }
}
