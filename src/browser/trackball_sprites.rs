//! Swapped point buffers, the voxel terrain, TrackballControls, sprites and the
//! fly-through LOD field from the pinned WebGL examples.
use super::controls_attributes::viewport_css;
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    tsl::{self, *},
};
use std::f64::consts::{PI, TAU};
use std::sync::Arc;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
async fn texture(url: &str, srgb: bool) -> Result<Arc<crate::material::Texture>> {
    let mut t = decode_texture_image(&fetch(url).await?).await?;
    t.srgb = srgb;
    t.mipmap_filter = Some(Filter::Linear);
    Ok(Arc::new(t))
}
/// The instanced city of the controls examples, with fog and three lights.
fn cones(s: &mut Scene, seed: &mut u32) -> Result<()> {
    s.background = Color::from_hex(0xcccccc);
    s.fog = Some(Fog::Exp2 {
        color: Color::from_hex(0xcccccc),
        density: 0.002,
    });
    let mut m = MeshPhongMaterial::default();
    m.properties.flat_shading = true;
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(CylinderGeometry::build(0., 10., 30., 4, 1, false, 0., TAU)?),
        Arc::new(Material::Phong(m)),
    )));
    s.get_mut(h)?.instances = (0..500)
        .map(|_| Instance {
            matrix: Matrix4::from_translation(Vector3::from_array(
                [0; 3].map(|_| (random(seed) - 0.5) * 1000.),
            )),
            color: Color::WHITE,
        })
        .collect();
    for (color, p) in [(0xffffff, 1.), (0x002288, -1.)] {
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(color),
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::splat(p);
    }
    s.insert(NodeKind::Light(Light::Ambient {
        color: Color::from_hex(0x555555),
        intensity: 1.,
    }));
    Ok(())
}
#[derive(Clone, Copy, PartialEq)]
enum Mode {
    None,
    Rotate,
    Zoom,
    Pan,
}
/// TrackballControls with its screen-space pointer state, stepped once per 60 fps step.
struct Trackball {
    camera: Object3D,
    target: Vector3,
    eye: Vector3,
    move_prev: Vector2,
    move_curr: Vector2,
    last_axis: Vector3,
    last_angle: f64,
    zoom_start: Vector2,
    zoom_end: Vector2,
    pan_start: Vector2,
    pan_end: Vector2,
    state: Mode,
    key_state: Mode,
    screen: Vector2,
}
impl Trackball {
    const ROTATE_SPEED: f64 = 1.0;
    const ZOOM_SPEED: f64 = 1.2;
    const PAN_SPEED: f64 = 0.8;
    const DAMPING: f64 = 0.2;
    fn new(s: &mut Scene, camera: Object3D, screen: Vector2) -> Result<Self> {
        let mut t = Self {
            camera,
            target: Vector3::ZERO,
            eye: Vector3::ZERO,
            move_prev: Vector2::ZERO,
            move_curr: Vector2::ZERO,
            last_axis: Vector3::ZERO,
            last_angle: 0.,
            zoom_start: Vector2::ZERO,
            zoom_end: Vector2::ZERO,
            pan_start: Vector2::ZERO,
            pan_end: Vector2::ZERO,
            state: Mode::None,
            key_state: Mode::None,
            screen,
        };
        // The constructor's update().
        t.update(s)?;
        Ok(t)
    }
    fn on_screen(&self, x: f64, y: f64) -> Vector2 {
        Vector2::new(x / self.screen.x, y / self.screen.y)
    }
    fn on_circle(&self, x: f64, y: f64) -> Vector2 {
        Vector2::new(
            (x - self.screen.x * 0.5) / (self.screen.x * 0.5),
            (self.screen.y - 2. * y) / self.screen.x,
        )
    }
    fn down(&mut self, button: u32, x: f64, y: f64) {
        // mouseButtons: LEFT rotate, MIDDLE dolly, RIGHT pan.
        self.state = match button {
            0 => Mode::Rotate,
            1 => Mode::Zoom,
            2 => Mode::Pan,
            _ => Mode::None,
        };
        let state = if self.key_state != Mode::None {
            self.key_state
        } else {
            self.state
        };
        match state {
            Mode::Rotate => {
                self.move_curr = self.on_circle(x, y);
                self.move_prev = self.move_curr;
            }
            Mode::Zoom => {
                self.zoom_start = self.on_screen(x, y);
                self.zoom_end = self.zoom_start;
            }
            Mode::Pan => {
                self.pan_start = self.on_screen(x, y);
                self.pan_end = self.pan_start;
            }
            Mode::None => {}
        }
    }
    fn moved(&mut self, x: f64, y: f64) {
        let state = if self.key_state != Mode::None {
            self.key_state
        } else {
            self.state
        };
        match state {
            Mode::Rotate => self.move_curr = self.on_circle(x, y),
            Mode::Zoom => self.zoom_end = self.on_screen(x, y),
            Mode::Pan => self.pan_end = self.on_screen(x, y),
            Mode::None => {}
        }
    }
    fn update(&mut self, s: &mut Scene) -> Result<()> {
        let c = self.camera;
        let n = s.get(c)?;
        let (mut position, mut up) = (n.position, n.up);
        let perspective = matches!(n.kind, NodeKind::Camera(Camera::Perspective(_)));
        self.eye = position - self.target;
        // _rotateCamera
        let delta = self.move_curr - self.move_prev;
        let angle = delta.length();
        if angle != 0. {
            self.eye = position - self.target;
            let eye_direction = self.eye.normalize();
            let up_direction = up.normalize();
            let sideways = up_direction.cross(eye_direction).normalize();
            let direction = up_direction * delta.y + sideways * delta.x;
            let axis = direction.cross(self.eye).normalize();
            let angle = angle * Self::ROTATE_SPEED;
            let q = Quaternion::from_axis_angle(axis, angle);
            self.eye = q * self.eye;
            up = q * up;
            self.last_axis = axis;
            self.last_angle = angle;
        } else if self.last_angle != 0. {
            self.last_angle *= (1. - Self::DAMPING).sqrt();
            self.eye = position - self.target;
            let q = Quaternion::from_axis_angle(self.last_axis, self.last_angle);
            self.eye = q * self.eye;
            up = q * up;
        }
        self.move_prev = self.move_curr;
        // _zoomCamera
        let factor = 1. + (self.zoom_end.y - self.zoom_start.y) * Self::ZOOM_SPEED;
        if factor != 1. && factor > 0. {
            if perspective {
                self.eye *= factor;
            } else if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(c)?.kind {
                o.zoom = (o.zoom / factor).max(0.);
            }
        }
        self.zoom_start.y += (self.zoom_end.y - self.zoom_start.y) * Self::DAMPING;
        // _panCamera
        let mut change = self.pan_end - self.pan_start;
        if change.length_squared() != 0. {
            if let NodeKind::Camera(Camera::Orthographic(o)) = &s.get(c)?.kind {
                let width = self.screen.x;
                change.x *= (o.right - o.left) / o.zoom / width;
                change.y *= (o.top - o.bottom) / o.zoom / width;
            }
            change *= self.eye.length() * Self::PAN_SPEED;
            let set_length = |v: Vector3, l: f64| v.normalize_or_zero() * l;
            let pan = set_length(self.eye.cross(up), change.x) + set_length(up, change.y);
            position += pan;
            self.target += pan;
            self.pan_start += (self.pan_end - self.pan_start) * Self::DAMPING;
        }
        position = self.target + self.eye;
        let n = s.get_mut(c)?;
        n.position = position;
        n.up = up;
        s.look_at(c, self.target)?;
        s.update_world_matrix(c, true, false)
    }
}
/// `webgl_sprites`: one textured billboard per sprite, as SpriteMaterial draws them.
struct Sprite {
    node: Object3D,
    rotation: f64,
    opacity: f64,
    center: Vector2,
    width: f64,
    faded: bool,
}
/// The sprite vertex stage of `sprite.glsl.js`: u.custom[0] = (rotation, opacity,
/// center.x, center.y), [1] = color, [2] = uv repeat and offset, [3] = fog (near, far, on).
/// The view depth for fog travels in local_normal.x.
const SPRITE_VERTEX: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;var mv=u.view*u.model*vec4(0.0,0.0,0.0,1.0);let scale=vec2(length(u.model[0].xyz),length(u.model[1].xyz));let aligned=(position.xy-(u.custom[0].zw-vec2(0.5)))*scale;let r=u.custom[0].x;mv=vec4(mv.xy+vec2(cos(r)*aligned.x-sin(r)*aligned.y,sin(r)*aligned.x+cos(r)*aligned.y),mv.zw);out.clip=u.projection*mv;out.local_normal=vec3(-mv.z,0.0,0.0);out.uv=surface.uv*u.custom[2].xy+u.custom[2].zw;return out;}";
/// The sprite HUD scene, its orthographic camera and the sprites with their centers.
type Hud = (Scene, Object3D, Vec<(Object3D, Vector2)>);
/// `webgl_lod`: FlyControls state.
#[derive(Default)]
struct Fly {
    keys: [bool; 12],
    buttons: [bool; 2],
    yaw_left: f64,
    pitch_down: f64,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    params: [f32; 2],
    // The HDR quad and its half-float texture.
    points: Vec<Object3D>,
    hdr_texture: Option<wgpu::Texture>,
    // Voxel terrain
    walker: FirstPerson,
    // Trackball
    trackball: Option<Trackball>,
    cameras: Vec<Object3D>,
    // Sprites
    sprites: Vec<Sprite>,
    group: Option<Object3D>,
    hud: Option<Hud>,
    output: Option<RenderTarget>,
    // LOD
    lods: Vec<(Object3D, [Object3D; 5])>,
    /// The 1×1 target of the one-time pass that prepares every LOD level's draw data.
    warm: Option<RenderTarget>,
    fly: Fly,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            213 => (50., 0.1, 1., Vector3::ZERO),
            214 => (60., 1., 20000., Vector3::ZERO),
            215 => (60., 1., 1000., Vector3::new(0., 0., 500.)),
            216 => (60., 1., 2100., Vector3::new(0., 0., 1500.)),
            _ => (45., 1., 15000., Vector3::new(0., 0., 1000.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = position;
        if id != 214 && id != 213 {
            s.look_at(c, Vector3::ZERO)?;
        }
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            params: [0., 0.],
            points: vec![],
            hdr_texture: None,
            walker: FirstPerson::default(),
            trackball: None,
            cameras: vec![],
            sprites: vec![],
            group: None,
            hud: None,
            output: None,
            lods: vec![],
            warm: None,
            fly: Fly::default(),
        };
        match id {
            213 => d.hdr(s, c, r).await?,
            214 => d.terrain(s, c).await?,
            215 => d.trackball_scene(s, c)?,
            216 => d.sprite_scene(s, r).await?,
            _ => d.lod_scene(s)?,
        }
        Ok(d)
    }
    async fn hdr(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.background = Color::BLACK;
        let (width, height, texels) =
            parse_rgbe(&fetch("/web/gallery/assets/memorial.hdr").await?)?;
        // HDRLoader: HalfFloatType, linear filtering, no mipmaps, flipY.
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("HDR texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        r.queue.write_texture(
            texture.as_image_copy(),
            bytemuck::cast_slice(&texels),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 8),
                rows_per_image: Some(height),
            },
            texture.size(),
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("HDR sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // MeshBasicMaterial map with ReinhardToneMapping( exposure ) and the sRGB output,
        // written raw; u.custom[0].x is toneMappingExposure.
        let color = WgslFn::new(
            "hdr_color",
            "fn hdr_color(t:vec4<f32>)->vec4<f32>{let c=u.custom[0].x*t.rgb;let m=clamp(c/(vec3(1.0)+c),vec3(0.0),vec3(1.0));let e=select(1.055*pow(m,vec3(0.41666))-vec3(0.055),m*12.92,m<=vec3(0.0031308));return vec4(e,1.0);}",
            &[Type::Vec4],
            Type::Vec4,
        )?
        .call(&[tsl::Texture::External(0).sample(vec2(uv().x(), float(1.) - uv().y()))]);
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(1)?,
            &[],
            &[(&view, &sampler)],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.uniforms[0] = [2., 0., 0., 0.];
        let quad = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(
                1.5 * width as f64 / height as f64,
                1.5,
                1,
                1,
            )?),
            Arc::new(Material::Shader(m)),
        )));
        self.points.push(quad);
        self.params[0] = 2.;
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -aspect,
            right: aspect,
            top: 1.,
            bottom: -1.,
            near: 0.,
            far: 1.,
            ..Default::default()
        }));
        self.hdr_texture = Some(texture);
        Ok(())
    }
    async fn terrain(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.background = Color::from_hex(0xbfd1e5);
        let (width, depth) = (128i64, 128i64);
        // generateHeight runs first, so its Math.random() is the first draw.
        let data = generate_height(width as usize, depth as usize, &mut self.seed);
        let get_y = |x: i64, z: i64| -> i64 {
            let i = x + z * width;
            // Out-of-range reads are undefined in the original: ( NaN * 0.15 ) | 0 = 0.
            if i < 0 || i as usize >= data.len() {
                0
            } else {
                (data[i as usize] * 0.15) as i64
            }
        };
        s.get_mut(c)?.position.y = (get_y(width / 2, depth / 2) * 100 + 100) as f64;
        // The five face templates: PlaneGeometry( 100, 100 ) with edited uvs, rotated and moved.
        let f = |v: f64| v as f32 as f64;
        let face = |uv_rows: [usize; 2], rotation: Option<(char, f64)>, offset: Vector3| {
            let mut p = [
                [-50., 50., 0.],
                [50., 50., 0.],
                [-50., -50., 0.],
                [50., -50., 0.],
            ];
            let mut n = [[0., 0., 1.]; 4];
            let mut uv = [[0., 1.], [1., 1.], [0., 0.], [1., 0.]];
            for row in uv_rows {
                uv[row][1] = 0.5;
            }
            if let Some((axis, angle)) = rotation {
                let (sn, cs) = angle.sin_cos();
                let turn = |v: [f64; 3]| match axis {
                    'y' => [cs * v[0] + sn * v[2], v[1], -sn * v[0] + cs * v[2]],
                    _ => [v[0], cs * v[1] - sn * v[2], sn * v[1] + cs * v[2]],
                };
                for v in &mut p {
                    *v = turn(*v).map(f);
                }
                for v in &mut n {
                    let t = turn(*v);
                    let l = (t[0] * t[0] + t[1] * t[1] + t[2] * t[2]).sqrt();
                    *v = t.map(|x| f(x / l));
                }
            }
            for v in &mut p {
                *v = [f(v[0] + offset.x), f(v[1] + offset.y), f(v[2] + offset.z)];
            }
            (p, n, uv)
        };
        let half = PI / 2.;
        let px = face([0, 1], Some(('y', half)), Vector3::new(50., 0., 0.));
        let nx = face([0, 1], Some(('y', -half)), Vector3::new(-50., 0., 0.));
        let py = face([2, 3], Some(('x', -half)), Vector3::new(0., 50., 0.));
        let pz = face([0, 1], None, Vector3::new(0., 0., 50.));
        let nz = face([0, 1], Some(('y', PI)), Vector3::new(0., 0., -50.));
        let (mut positions, mut normals, mut uvs, mut index) = (vec![], vec![], vec![], vec![]);
        let mut push = |(p, n, uv): &Face, t: [f64; 3]| {
            let base = (positions.len() / 3) as u32;
            for k in 0..4 {
                positions.extend([
                    f(p[k][0] + t[0]) as f32,
                    f(p[k][1] + t[1]) as f32,
                    f(p[k][2] + t[2]) as f32,
                ]);
                normals.extend(n[k].map(|v| v as f32));
                uvs.extend(uv[k].map(|v| v as f32));
            }
            index.extend([0, 2, 1, 2, 3, 1].map(|i| base + i));
        };
        for z in 0..depth {
            for x in 0..width {
                let h = get_y(x, z);
                let t = [
                    (x * 100 - width / 2 * 100) as f64,
                    (h * 100) as f64,
                    (z * 100 - depth / 2 * 100) as f64,
                ];
                let (px_h, nx_h, pz_h, nz_h) = (
                    get_y(x + 1, z),
                    get_y(x - 1, z),
                    get_y(x, z + 1),
                    get_y(x, z - 1),
                );
                let open = |v: i64| v != h && v != h + 1;
                push(&py, t);
                if open(px_h) || x == 0 {
                    push(&px, t);
                }
                if open(nx_h) || x == width - 1 {
                    push(&nx, t);
                }
                if open(pz_h) || z == depth - 1 {
                    push(&pz, t);
                }
                if open(nz_h) || z == 0 {
                    push(&nz, t);
                }
            }
        }
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(positions)?);
        g.set_attribute("normal", vec3s(normals)?);
        g.set_attribute("uv", Attribute::F32(BufferAttribute::new(uvs, 2, false)?));
        g.set_index(Some(index));
        let mut atlas =
            decode_texture_image(&fetch("/web/gallery/assets/minecraft/atlas.png").await?).await?;
        atlas.srgb = true;
        atlas.filter = Filter::Nearest;
        atlas.min_filter = Some(Filter::Linear);
        atlas.mipmap_filter = Some(Filter::Linear);
        let mut m = MeshLambertMaterial::default();
        m.properties.map = Some(Arc::new(atlas));
        m.properties.side = Side::Double;
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Lambert(m)),
        )));
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xeeeeee),
            intensity: 3.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 12.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(1., 1., 0.5).normalize();
        // FirstPersonControls._setOrientation() from the camera's initial quaternion.
        let look = s.get(c)?.quaternion * -Vector3::Z;
        self.walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
        self.walker.lon = look.x.atan2(look.z).to_degrees();
        Ok(())
    }
    fn trackball_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        cones(s, &mut self.seed)?;
        let orthographic = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            near: 1.,
            far: 1000.,
            ..Default::default()
        })));
        s.get_mut(orthographic)?.position = Vector3::new(0., 0., 500.);
        self.cameras = vec![c, orthographic];
        self.set_frustum(s)?;
        let (w, h, _) = viewport_css();
        self.trackball = Some(Trackball::new(s, c, Vector2::new(w, h))?);
        Ok(())
    }
    fn set_frustum(&self, s: &mut Scene) -> Result<()> {
        let (w, h, _) = viewport_css();
        let aspect = w / h.max(1.);
        let frustum = 400.;
        if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(self.cameras[1])?.kind {
            o.left = -frustum * aspect / 2.;
            o.right = frustum * aspect / 2.;
            o.top = frustum / 2.;
            o.bottom = -frustum / 2.;
        }
        Ok(())
    }
    async fn sprite_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::BLACK;
        s.fog = Some(Fog::Linear {
            color: Color::BLACK,
            near: 1500.,
            far: 2100.,
        });
        let base = "/web/gallery/assets";
        let map_b = texture(&format!("{base}/sprite1.png"), true).await?;
        let map_c = texture(&format!("{base}/sprite2.png"), true).await?;
        let hud = texture(
            &format!("{base}/environment-materials/textures/sprite0.png"),
            true,
        )
        .await?;
        // The shared Sprite geometry: a unit quad with uv.
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            vec3s(vec![
                -0.5, -0.5, 0., 0.5, -0.5, 0., 0.5, 0.5, 0., -0.5, 0.5, 0.,
            ])?,
        );
        g.set_attribute("normal", vec3s([0., 0., 1.].repeat(4))?);
        g.set_attribute(
            "uv",
            Attribute::F32(BufferAttribute::new(
                vec![0., 0., 1., 0., 1., 1., 0., 1.],
                2,
                false,
            )?),
        );
        g.set_index(Some(vec![0, 1, 2, 0, 2, 3]));
        let g = Arc::new(g);
        // Fragment: map × diffuse/opacity, sRGB-encoded, then fog toward black after the
        // encoding (fog_fragment follows colorspace_fragment in the sprite shader). The
        // flipY upload puts v = 0 at the image bottom.
        let color = WgslFn::new(
            "sprite_color",
            "fn sprite_color(t:vec4<f32>,depth:f32)->vec4<f32>{let c=t*vec4(u.custom[1].rgb,u.custom[0].y);let e=select(1.055*pow(c.rgb,vec3(0.41666))-vec3(0.055),c.rgb*12.92,c.rgb<=vec3(0.0031308));let f=select(0.0,smoothstep(u.custom[3].x,u.custom[3].y,depth),u.custom[3].z>0.5);return vec4(mix(e,vec3(0.0),f),c.a);}",
            &[Type::Vec4, Type::Float],
            Type::Vec4,
        )?
        .call(&[
            tsl::Texture::External(0).sample(vec2(uv().x(), float(1.) - uv().y())),
            normal_local().x(),
        ]);
        let graph = NodeMaterial::new(color);
        let source = graph.wgsl(1)?;
        let mut programs = vec![];
        for map in [&map_b, &map_c, &hud] {
            let t = r.upload_texture(map)?;
            programs.push(Arc::new(
                ShaderProgram::with_projection(
                    r,
                    &source,
                    &[],
                    &[(&t.view, &t.sampler)],
                    SPRITE_VERTEX,
                )
                .await?,
            ));
        }
        let material = |program: &Arc<ShaderProgram>, color: Color, uv: [f32; 4], fog: bool| {
            let mut m = ShaderMaterial::new(program.clone());
            m.properties.transparent = true;
            // NormalBlending of a non-premultiplied material.
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
            m.properties.side = Side::Double;
            // Fog is applied in the fragment function, after the sRGB encoding.
            m.properties.fog = false;
            m.uniforms[0] = [0., 1., 0.5, 0.5];
            m.uniforms[1] = color.0.as_vec3().extend(1.).to_array();
            m.uniforms[2] = uv;
            m.uniforms[3] = [1500., 2100., f32::from(u8::from(fog)), 0.];
            m
        };
        let group = s.insert(NodeKind::Group);
        let identity = [1., 1., 0., 0.];
        for _ in 0..200 {
            let x = random(&mut self.seed) - 0.5;
            let y = random(&mut self.seed) - 0.5;
            let z = random(&mut self.seed) - 0.5;
            let (m, faded) = if z < 0. {
                (material(&programs[0], Color::WHITE, identity, true), true)
            } else {
                let hue = 0.5 * random(&mut self.seed);
                // map.offset ( -0.5, -0.5 ), map.repeat ( 2, 2 ) on the shared mapC.
                (
                    material(
                        &programs[1],
                        hsl(hue, 0.75, 0.5),
                        [2., 2., -0.5, -0.5],
                        true,
                    ),
                    false,
                )
            };
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                g.clone(),
                Arc::new(Material::Shader(m)),
            )));
            s.get_mut(h)?.position = Vector3::new(x, y, z).normalize() * 500.;
            s.get_mut(h)?.frustum_culled = false;
            s.add(group, h)?;
            self.sprites.push(Sprite {
                node: h,
                rotation: 0.,
                opacity: 1.,
                center: Vector2::splat(0.5),
                width: 128.,
                faded,
            });
        }
        self.group = Some(group);
        // The HUD scene: five sprites at the corners and center, sharing one material.
        let mut scene = Scene::new();
        let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            near: 1.,
            far: 10.,
            ..Default::default()
        })));
        scene.get_mut(camera)?.position = Vector3::new(0., 0., 10.);
        let mut corners = vec![];
        for center in [(0., 1.), (1., 1.), (0., 0.), (1., 0.), (0.5, 0.5)] {
            let mut m = material(&programs[2], Color::WHITE, identity, false);
            m.uniforms[0] = [0., 1., center.0, center.1];
            let h = scene.insert(NodeKind::Mesh(Mesh::new(
                g.clone(),
                Arc::new(Material::Shader(m)),
            )));
            let n = scene.get_mut(h)?;
            n.scale = Vector3::new(128., 128., 1.);
            n.frustum_culled = false;
            corners.push((h, Vector2::new(center.0 as f64, center.1 as f64)));
        }
        self.hud = Some((scene, camera, corners));
        Ok(())
    }
    fn lod_scene(&mut self, s: &mut Scene) -> Result<()> {
        s.background = Color::BLACK;
        s.fog = Some(Fog::Linear {
            color: Color::BLACK,
            near: 1.,
            far: 15000.,
        });
        s.insert(NodeKind::Light(Light::Point {
            color: Color::from_hex(0xff2200),
            intensity: 3.,
            distance: 0.,
            decay: 0.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::Z;
        let geometries =
            [16, 8, 4, 2, 1].map(|detail| IcosahedronGeometry::build(100., detail).map(Arc::new));
        let mut m = MeshLambertMaterial::default();
        m.properties.wireframe = true;
        let m = Arc::new(Material::Lambert(m));
        for _ in 0..1000 {
            let lod = s.insert(NodeKind::Group);
            let mut levels = Vec::with_capacity(5);
            for g in &geometries {
                let g = g.as_ref().map_err(|_| Error::Invalid("LOD geometry"))?;
                let h = s.insert(NodeKind::Mesh(Mesh::new(g.clone(), m.clone())));
                s.get_mut(h)?.scale = Vector3::splat(1.5);
                s.add(lod, h)?;
                levels.push(h);
            }
            let p = Vector3::new(
                10000. * (0.5 - random(&mut self.seed)),
                7500. * (0.5 - random(&mut self.seed)),
                10000. * (0.5 - random(&mut self.seed)),
            );
            s.get_mut(lod)?.position = p;
            self.lods
                .push((lod, [levels[0], levels[1], levels[2], levels[3], levels[4]]));
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// One original animation frame; per-frame increments run as 60 fps steps of example time.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let delta = t - self.last;
        let step = delta * 60.;
        self.last = t;
        match self.id {
            213 => {
                // renderer.toneMappingExposure = params.exposure; the resize keeps the
                // frustum height and follows the window aspect.
                let (w, h, _) = viewport_css();
                if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(c)?.kind {
                    let height = o.top - o.bottom;
                    o.left = -height * (w / h.max(1.)) / 2.;
                    o.right = height * (w / h.max(1.)) / 2.;
                }
                if let Some(&quad) = self.points.first()
                    && let NodeKind::Mesh(m) = &mut s.get_mut(quad)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0][0] = self.params[0];
                }
            }
            // Zero-length frames do not step the controls (see docs/trackball-sprites.md).
            214 if delta > 0. => self.walker.update(s, c, delta)?,
            214 => {}
            215 => {
                self.set_frustum(s)?;
                // createControls( camera ) after the orthographic toggle.
                let camera = self.cameras[usize::from(self.params[0] > 0.5)];
                let (w, h, _) = viewport_css();
                if self.trackball.as_ref().is_none_or(|t| t.camera != camera) {
                    self.trackball = Some(Trackball::new(s, camera, Vector2::new(w, h))?);
                }
                // handleResize() on window resize.
                if let Some(t) = &mut self.trackball {
                    t.screen = Vector2::new(w, h);
                }
                let steps = step.round().max(0.) as usize;
                if let Some(trackball) = &mut self.trackball {
                    for _ in 0..steps {
                        trackball.update(s)?;
                    }
                }
            }
            216 => self.prepare_sprites(s, t, step)?,
            _ => self.prepare_lod(s, c, delta)?,
        }
        Ok(())
    }
    fn prepare_sprites(&mut self, s: &mut Scene, t: f64, step: f64) -> Result<()> {
        let count = self.sprites.len();
        for (i, sprite) in self.sprites.iter_mut().enumerate() {
            let x = s.get(sprite.node)?.position.x;
            let scale = (t + x * 0.01).sin() * 0.3 + 1.;
            sprite.rotation += 0.1 * (i as f64 / count as f64) * step;
            if sprite.faded {
                sprite.opacity = (t + x * 0.01).sin() * 0.4 + 0.6;
            }
            let n = s.get_mut(sprite.node)?;
            n.scale = Vector3::new(scale * sprite.width, scale * sprite.width, 1.);
            if let NodeKind::Mesh(m) = &mut n.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.uniforms[0] = [
                    sprite.rotation as f32,
                    sprite.opacity as f32,
                    sprite.center.x as f32,
                    sprite.center.y as f32,
                ];
            }
        }
        if let Some(group) = self.group {
            s.get_mut(group)?.quaternion = Euler {
                angles: Vector3::new(t * 0.5, t * 0.75, t),
                order: EulerOrder::XYZ,
            }
            .quaternion();
        }
        // updateHUDSprites and the orthographic HUD camera, in CSS pixels.
        let (w, h, _) = viewport_css();
        if let Some((scene, camera, corners)) = &mut self.hud {
            if let NodeKind::Camera(Camera::Orthographic(o)) = &mut scene.get_mut(*camera)?.kind {
                o.left = -w / 2.;
                o.right = w / 2.;
                o.top = h / 2.;
                o.bottom = -h / 2.;
            }
            for (h_node, center) in corners.iter() {
                let x = if center.x == 0.5 {
                    0.
                } else {
                    (center.x * 2. - 1.) * w / 2.
                };
                let y = if center.y == 0.5 {
                    0.
                } else {
                    (center.y * 2. - 1.) * h / 2.
                };
                scene.get_mut(*h_node)?.position = Vector3::new(x, y, 1.);
            }
        }
        Ok(())
    }
    fn prepare_lod(&mut self, s: &mut Scene, c: Object3D, delta: f64) -> Result<()> {
        // FlyControls.update( delta ): movementSpeed 1000, rollSpeed PI / 10.
        let k = |i: usize| f64::from(u8::from(self.fly.keys[i]));
        let [
            forward,
            back,
            left,
            right,
            up,
            down,
            pitch_up,
            pitch_down,
            yaw_left,
            yaw_right,
            roll_left,
            roll_right,
        ] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(k);
        let forward = forward.max(f64::from(u8::from(self.fly.buttons[0])));
        let back = back.max(f64::from(u8::from(self.fly.buttons[1])));
        let movement = Vector3::new(-left + right, -down + up, -forward + back);
        let rotation = Vector3::new(
            -(pitch_down + self.fly.pitch_down) + pitch_up,
            -yaw_right + yaw_left + self.fly.yaw_left,
            -roll_right + roll_left,
        );
        let (move_mult, rot_mult) = (delta * 1000., delta * PI / 10.);
        let n = s.get_mut(c)?;
        let q = n.quaternion;
        n.position += q * Vector3::X * (movement.x * move_mult);
        n.position += q * Vector3::Y * (movement.y * move_mult);
        n.position += q * Vector3::Z * (movement.z * move_mult);
        let r = rotation * rot_mult;
        let tmp = Quaternion::from_xyzw(r.x, r.y, r.z, 1.).normalize();
        n.quaternion = q * tmp;
        let camera = n.position;
        // LOD.update( camera ): levels at 50, 300, 1000, 2000 and 8000.
        let distances = [50., 300., 1000., 2000., 8000.];
        for (lod, levels) in &self.lods {
            let distance = camera.distance(s.get(*lod)?.position);
            let mut current = 0;
            for (i, d) in distances.iter().enumerate().skip(1) {
                if distance >= *d {
                    current = i;
                } else {
                    break;
                }
            }
            for (i, h) in levels.iter().enumerate() {
                s.get_mut(*h)?.visible = i == current;
            }
        }
        Ok(())
    }
    /// The trackball's orthographic camera and the sprite HUD pass.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        match self.id {
            215 if self.params[0] > 0.5 => {
                self.ensure_output(r, out)?;
                let target = self.output.as_mut().expect("output");
                target.set_load_color(false);
                r.render(s, self.cameras[1], target)?;
                Ok(true)
            }
            216 => {
                self.ensure_output(r, out)?;
                let target = self.output.as_mut().expect("output");
                target.set_load_color(false);
                r.render(s, c, target)?;
                // renderer.clearDepth(), then the HUD over the sprites.
                target.set_load_color(true);
                let (scene, camera, _) = self.hud.as_mut().ok_or(Error::Invalid("hud"))?;
                scene.background = Color::BLACK;
                r.render(scene, *camera, target)?;
                target.set_load_color(false);
                Ok(true)
            }
            217 if self.warm.is_none() => {
                // Per-object draw data is created on first draw: prepare every level once,
                // unculled, into a 1×1 target, so flying reveals no new GPU resources.
                let warm = RenderTarget::with_options(&r.device, 1, 1, out.options.clone())?;
                let mut state = vec![];
                for (_, levels) in &self.lods {
                    for &h in levels {
                        let n = s.get_mut(h)?;
                        state.push((h, n.visible));
                        n.visible = true;
                        n.frustum_culled = false;
                    }
                }
                r.render(s, c, &warm)?;
                for (h, visible) in state {
                    let n = s.get_mut(h)?;
                    n.visible = visible;
                    n.frustum_culled = true;
                }
                self.warm = Some(warm);
                Ok(false)
            }
            _ => Ok(false),
        }
    }
    fn ensure_output(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
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
        Ok(())
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        match self.id {
            215 if self.params[0] > 0.5 => self.output.as_ref(),
            216 => self.output.as_ref(),
            _ => None,
        }
    }
    /// Absolute CSS-pixel pointer events: kind 0 move, 10 + button down, 20 + button up.
    pub fn pointer(&mut self, kind: u32, x: f64, y: f64) -> Result<()> {
        match self.id {
            214 => self.walker.pointer(kind, x, y),
            215 => {
                if let Some(t) = &mut self.trackball {
                    match kind {
                        10..=19 => t.down(kind - 10, x, y),
                        20..=29 => t.state = Mode::None,
                        _ => t.moved(x, y),
                    }
                }
            }
            217 => match kind {
                10 => self.fly.buttons[0] = true,
                12 => self.fly.buttons[1] = true,
                20 => self.fly.buttons[0] = false,
                22 => self.fly.buttons[1] = false,
                0 => {
                    let (w, h, _) = viewport_css();
                    let (hw, hh) = (w / 2., h / 2.);
                    self.fly.yaw_left = -(x - hw) / hw;
                    self.fly.pitch_down = (y - hh) / hh;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
    /// Wheel input: TrackballControls' deltaMode 0 zoom, zoomStart.y -= deltaY × 0.00025.
    pub fn wheel(&mut self, wheel: f64) {
        if let (215, Some(t)) = (self.id, &mut self.trackball) {
            t.zoom_start.y -= wheel * 0.00025;
        }
    }
    pub fn key(&mut self, code: u32, down: bool) {
        match self.id {
            214 => {
                // W / ArrowUp, S / ArrowDown, A / ArrowLeft, D / ArrowRight, R, F.
                let index = match code {
                    87 | 38 => 0,
                    83 | 40 => 1,
                    65 | 37 => 2,
                    68 | 39 => 3,
                    82 => 4,
                    70 => 5,
                    _ => return,
                };
                self.walker.keys[index] = down;
            }
            215 => {
                if let Some(t) = &mut self.trackball {
                    if !down {
                        t.key_state = Mode::None;
                    } else if t.key_state == Mode::None {
                        // keys [ 'KeyA', 'KeyS', 'KeyD' ]: rotate, zoom, pan.
                        t.key_state = match code {
                            65 => Mode::Rotate,
                            83 => Mode::Zoom,
                            68 => Mode::Pan,
                            _ => Mode::None,
                        };
                    }
                }
            }
            217 => {
                // W S A D R F, arrows, Q E.
                let index = match code {
                    87 => 0,
                    83 => 1,
                    65 => 2,
                    68 => 3,
                    82 => 4,
                    70 => 5,
                    38 => 6,
                    40 => 7,
                    37 => 8,
                    39 => 9,
                    81 => 10,
                    69 => 11,
                    _ => return,
                };
                self.fly.keys[index] = down;
            }
            _ => {}
        }
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (213, 0) if (0. ..=4.).contains(&value) => self.params[0] = value,
            // The orthographic toggle recreates the controls on the next frame;
            // multiTouchRoll only affects two-finger touch.
            (215, 0 | 1) => self.params[index] = value,
            _ => return Err(Error::Invalid("trackball/sprites parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// `Color.setHSL` in the linear working color space.
fn hsl(h: f64, s: f64, l: f64) -> Color {
    let h = (h % 1. + 1.) % 1.;
    if s == 0. {
        return Color::linear(l, l, l);
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
    Color::linear(f(h + 1. / 3.), f(h), f(h - 1. / 3.))
}
/// One face template: four positions, normals and uvs.
type Face = ([[f64; 3]; 4], [[f64; 3]; 4], [[f64; 2]; 4]);
/// ImprovedNoise's permutation, doubled.
const PERMUTATION: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];
/// `ImprovedNoise.noise( x, y, z )`.
fn noise(x: f64, y: f64, z: f64) -> f64 {
    let p = |i: usize| PERMUTATION[i & 255] as usize;
    let fade = |t: f64| t * t * t * (t * (t * 6. - 15.) + 10.);
    let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
    let grad = |hash: usize, x: f64, y: f64, z: f64| {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            z
        };
        (if h & 1 == 0 { u } else { -u }) + if h & 2 == 0 { v } else { -v }
    };
    let (fx, fy, fz) = (x.floor(), y.floor(), z.floor());
    let (xi, yi, zi) = (
        (fx as i64 & 255) as usize,
        (fy as i64 & 255) as usize,
        (fz as i64 & 255) as usize,
    );
    let (x, y, z) = (x - fx, y - fy, z - fz);
    let (u, v, w) = (fade(x), fade(y), fade(z));
    // _p holds 512 entries: indices below 512 read the doubled table directly.
    let a = p(xi) + yi;
    let (aa, ab) = (p(a) + zi, p(a + 1) + zi);
    let b = p(xi + 1) + yi;
    let (ba, bb) = (p(b) + zi, p(b + 1) + zi);
    lerp(
        lerp(
            lerp(grad(p(aa), x, y, z), grad(p(ba), x - 1., y, z), u),
            lerp(grad(p(ab), x, y - 1., z), grad(p(bb), x - 1., y - 1., z), u),
            v,
        ),
        lerp(
            lerp(
                grad(p(aa + 1), x, y, z - 1.),
                grad(p(ba + 1), x - 1., y, z - 1.),
                u,
            ),
            lerp(
                grad(p(ab + 1), x, y - 1., z - 1.),
                grad(p(bb + 1), x - 1., y - 1., z - 1.),
                u,
            ),
            v,
        ),
        w,
    )
}
/// `generateHeight( width, height )` of the voxel example.
fn generate_height(width: usize, height: usize, seed: &mut u32) -> Vec<f64> {
    let size = width * height;
    let z = random(seed) * 100.;
    let mut data = vec![0.; size];
    let mut quality = 2.;
    for _ in 0..4 {
        for (i, d) in data.iter_mut().enumerate() {
            let (x, y) = ((i % width) as f64, (i / width) as f64);
            *d += noise(x / quality, y / quality, z) * quality;
        }
        quality *= 4.;
    }
    data
}
/// FirstPersonControls (movementSpeed 1000, lookSpeed 0.2, damping 0.1).
#[derive(Default)]
struct FirstPerson {
    velocity: Vector3,
    lon_velocity: f64,
    lat_velocity: f64,
    lon: f64,
    lat: f64,
    pointer: Vector2,
    down: Vector2,
    count: u32,
    drag: bool,
    pointer_forward: bool,
    pointer_backward: bool,
    /// forward, backward, left, right, up, down.
    keys: [bool; 6],
}
impl FirstPerson {
    fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64) -> Result<()> {
        let (speed, look, damping) = (1000., 0.2, 0.1);
        let b = |v: bool| f64::from(u8::from(v));
        let mut drive = b(self.keys[0]) - b(self.keys[1]);
        let look_move = b(self.pointer_forward) - b(self.pointer_backward);
        let yaw = self.lon.to_radians();
        let (sin_yaw, cos_yaw) = yaw.sin_cos();
        let mut strafe = b(self.keys[3]) - b(self.keys[2]);
        let mut climb = b(self.keys[4]) - b(self.keys[5]);
        let key_scale = 1.
            / (strafe * strafe + climb * climb + drive * drive)
                .sqrt()
                .max(1.);
        strafe *= speed * key_scale;
        climb *= speed * key_scale;
        drive *= speed * key_scale;
        let mut target = Vector3::new(
            sin_yaw * drive - cos_yaw * strafe,
            climb,
            cos_yaw * drive + sin_yaw * strafe,
        );
        let n = s.get_mut(c)?;
        if look_move != 0. {
            target += (n.quaternion * -Vector3::Z) * (look_move * speed);
        }
        self.velocity += (target - self.velocity) * damping;
        n.position += self.velocity * delta;
        let target_lon = if self.drag {
            -self.pointer.x * look
        } else {
            0.
        };
        let target_lat = if self.drag {
            -self.pointer.y * look
        } else {
            0.
        };
        self.lon_velocity += (target_lon - self.lon_velocity) * damping;
        self.lat_velocity += (target_lat - self.lat_velocity) * damping;
        self.lon += self.lon_velocity * delta;
        self.lat += self.lat_velocity * delta;
        self.lat = self.lat.clamp(-85., 85.);
        let (phi, theta) = ((90. - self.lat).to_radians(), self.lon.to_radians());
        // setFromSphericalCoords( 1, phi, theta ) around the camera.
        let position = n.position;
        let target =
            position + Vector3::new(phi.sin() * theta.sin(), phi.cos(), phi.sin() * theta.cos());
        s.look_at(c, target)
    }
    fn pointer(&mut self, kind: u32, x: f64, y: f64) {
        match kind {
            10..=19 => {
                let idle = !self.keys[0] && !self.keys[1];
                match kind - 10 {
                    0 if idle => self.pointer_forward = true,
                    2 if idle => self.pointer_backward = true,
                    _ => {}
                }
                self.count += 1;
                self.down = Vector2::new(x, y);
                self.pointer = Vector2::ZERO;
                self.drag = true;
            }
            20..=29 => {
                if !self.drag {
                    return;
                }
                self.count = self.count.saturating_sub(1);
                match kind - 20 {
                    0 => self.pointer_forward = false,
                    2 => self.pointer_backward = false,
                    _ => {}
                }
                self.pointer = Vector2::ZERO;
                if self.count == 0 {
                    self.drag = false;
                }
            }
            _ => {
                if self.drag {
                    self.pointer = Vector2::new(x, y) - self.down;
                }
            }
        }
    }
}
/// Radiance RGBE (new-style RLE or flat scanlines) as `HDRLoader` reads it, converted with
/// `RGBEByteToRGBHalf`: channel × 2^( e - 128 ) / 255, truncated to half floats.
fn parse_rgbe(data: &[u8]) -> Result<(u32, u32, Vec<u16>)> {
    let bad = |m: &'static str| Error::Asset(format!("HDR: {m}"));
    let mut pos = 0;
    let mut line = || -> Option<String> {
        let start = pos;
        let end = start + data.get(start..)?.iter().position(|&c| c == b'\n')?;
        pos = end + 1;
        Some(String::from_utf8_lossy(&data[start..end]).into_owned())
    };
    let (mut width, mut height) = (0usize, 0usize);
    while let Some(l) = line() {
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts.len() == 4 && parts[0] == "-Y" && parts[2] == "+X" {
            height = parts[1].parse().map_err(|_| bad("height"))?;
            width = parts[3].parse().map_err(|_| bad("width"))?;
            break;
        }
    }
    if width == 0 || height == 0 {
        return Err(bad("resolution"));
    }
    let mut rgbe = vec![0u8; width * height * 4];
    let mut p = pos;
    let byte = |i: usize| data.get(i).copied().ok_or(bad("truncated"));
    for y in 0..height {
        let row = &mut rgbe[y * width * 4..(y + 1) * width * 4];
        if !(8..=0x7fff).contains(&width)
            || byte(p)? != 2
            || byte(p + 1)? != 2
            || byte(p + 2)? & 0x80 != 0
        {
            // Flat scanline.
            row.copy_from_slice(data.get(p..p + width * 4).ok_or(bad("flat scanline"))?);
            p += width * 4;
            continue;
        }
        if ((byte(p + 2)? as usize) << 8 | byte(p + 3)? as usize) != width {
            return Err(bad("scanline width"));
        }
        p += 4;
        for channel in 0..4 {
            let mut x = 0;
            while x < width {
                let count = byte(p)? as usize;
                p += 1;
                if count > 128 {
                    let value = byte(p)?;
                    p += 1;
                    for _ in 0..count - 128 {
                        *row.get_mut(x * 4 + channel).ok_or(bad("run"))? = value;
                        x += 1;
                    }
                } else {
                    for _ in 0..count {
                        *row.get_mut(x * 4 + channel).ok_or(bad("literal"))? = byte(p)?;
                        p += 1;
                        x += 1;
                    }
                }
            }
        }
    }
    let half = |v: f64| -> u16 {
        // DataUtils.toHalfFloat: clamp, round to Float32, then truncate the mantissa.
        let f = (v.clamp(-65504., 65504.) as f32).to_bits();
        let e = (f >> 23) & 0x1ff;
        let (base, shift) = half_table(e);
        (base as u32 + ((f & 0x007fffff) >> shift)) as u16
    };
    let mut out = Vec::with_capacity(width * height * 4);
    for px in rgbe.chunks(4) {
        let scale = 2f64.powf(px[3] as f64 - 128.) / 255.;
        for &c in &px[..3] {
            out.push(half((c as f64 * scale).min(65504.)));
        }
        out.push(half(1.));
    }
    Ok((width as u32, height as u32, out))
}
/// `DataUtils` base and shift tables for a 9-bit sign/exponent index.
fn half_table(i: u32) -> (u16, u32) {
    let sign = if i & 0x100 != 0 { 0x8000u16 } else { 0 };
    let e = (i & 0xff) as i32 - 127;
    if e < -27 {
        (sign, 24)
    } else if e < -14 {
        (sign | (0x0400 >> (-e - 14)) as u16, (-e - 1) as u32)
    } else if e <= 15 {
        (sign | (((e + 15) as u16) << 10), 13)
    } else if e < 128 {
        (sign | 0x7c00, 24)
    } else {
        (sign | 0x7c00, 13)
    }
}
