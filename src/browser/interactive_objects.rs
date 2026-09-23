//! Instanced/sprite picking, orientation, atlas panorama and canvas-texture scenes
//! from the pinned WebGL examples.
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::{
    Error, Result,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    raycast::Raycaster,
    renderer::*,
    scene::*,
    tsl::{self, surface::SurfaceNodes, *},
};
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::sync::Arc;
use wasm_bindgen::JsCast;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
/// `Color.setHex( Math.random() * 0xffffff )`: floors, then decodes sRGB.
fn random_color(seed: &mut u32) -> Color {
    Color::from_hex((random(seed) * 16777215.).floor() as u32)
}
/// OrbitControls state and update step, expressed in CSS pixels.
struct Orbit {
    target: Vector3,
    theta: f64,
    phi: f64,
    radius: f64,
    delta: Vector2,
    pan: Vector3,
    scale: f64,
    damping: bool,
    rotate_speed: f64,
    distance: (f64, f64),
}
impl Orbit {
    fn new(position: Vector3, damping: bool, rotate_speed: f64, distance: (f64, f64)) -> Self {
        let radius = position.length();
        Self {
            target: Vector3::ZERO,
            theta: position.x.atan2(position.z),
            phi: (position.y / radius).clamp(-1., 1.).acos(),
            radius,
            delta: Vector2::ZERO,
            pan: Vector3::ZERO,
            scale: 1.,
            damping,
            rotate_speed,
            distance,
        }
    }
    fn rotate(&mut self, dx: f64, dy: f64, height: f64) {
        self.delta -= Vector2::new(dx, dy) * self.rotate_speed * TAU / height.max(1.);
    }
    fn dolly(&mut self, wheel: f64) {
        let scale = 0.95f64.powf((wheel * 0.01).abs());
        if wheel < 0. {
            self.scale *= scale;
        } else if wheel > 0. {
            self.scale /= scale;
        }
    }
    fn pan(&mut self, camera: &crate::scene::Node, fov: f64, dx: f64, dy: f64, height: f64) {
        let distance = self.radius * (fov / 2.).to_radians().tan();
        let m = Matrix4::from_quat(camera.quaternion);
        self.pan += m.x_axis.truncate() * (-2. * dx * distance / height.max(1.))
            + m.y_axis.truncate() * (2. * dy * distance / height.max(1.));
    }
    fn update(&mut self) {
        let f = if self.damping { 0.05 } else { 1. };
        self.theta += self.delta.x * f;
        self.phi = (self.phi + self.delta.y * f)
            .clamp(0., PI)
            .clamp(1e-6, PI - 1e-6);
        self.target += self.pan * f;
        self.radius = (self.radius * self.scale).clamp(self.distance.0, self.distance.1);
        if self.damping {
            self.delta *= 0.95;
            self.pan *= 0.95;
        } else {
            self.delta = Vector2::ZERO;
            self.pan = Vector3::ZERO;
        }
        self.scale = 1.;
    }
    fn apply(&self, s: &mut Scene, c: Object3D) -> Result<()> {
        let (sp, cp) = self.phi.sin_cos();
        let (st, ct) = self.theta.sin_cos();
        s.get_mut(c)?.position = self.target + Vector3::new(sp * st, cp, sp * ct) * self.radius;
        s.look_at(c, self.target)?;
        s.update_world_matrix(c, true, false)
    }
}
/// `Matrix4.lookAt` followed by `Quaternion.setFromRotationMatrix`.
fn look_rotation(eye: Vector3, target: Vector3, up: Vector3) -> Quaternion {
    let mut z = eye - target;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    z = z.normalize();
    let mut x = up.cross(z);
    if x.length_squared() == 0. {
        if up.z.abs() == 1. {
            z.x += 0.0001;
        } else {
            z.z += 0.0001;
        }
        z = z.normalize();
        x = up.cross(z);
    }
    x = x.normalize();
    let y = z.cross(x);
    let (m11, m12, m13) = (x.x, y.x, z.x);
    let (m21, m22, m23) = (x.y, y.y, z.y);
    let (m31, m32, m33) = (x.z, y.z, z.z);
    let trace = m11 + m22 + m33;
    if trace > 0. {
        let s = 0.5 / (trace + 1.).sqrt();
        Quaternion::from_xyzw((m32 - m23) * s, (m13 - m31) * s, (m21 - m12) * s, 0.25 / s)
    } else if m11 > m22 && m11 > m33 {
        let s = 2. * (1. + m11 - m22 - m33).sqrt();
        Quaternion::from_xyzw(0.25 * s, (m12 + m21) / s, (m13 + m31) / s, (m32 - m23) / s)
    } else if m22 > m33 {
        let s = 2. * (1. + m22 - m11 - m33).sqrt();
        Quaternion::from_xyzw((m12 + m21) / s, 0.25 * s, (m23 + m32) / s, (m13 - m31) / s)
    } else {
        let s = 2. * (1. + m33 - m11 - m22).sqrt();
        Quaternion::from_xyzw((m13 + m31) / s, (m23 + m32) / s, 0.25 * s, (m21 - m12) / s)
    }
}
/// `Quaternion.rotateTowards` with Three.js's `slerp`.
fn rotate_towards(a: Quaternion, b: Quaternion, step: f64) -> Quaternion {
    let angle = 2. * a.dot(b).clamp(-1., 1.).abs().acos();
    if angle == 0. {
        return a;
    }
    let t = (step / angle).min(1.);
    let (mut other, mut dot) = (b, a.dot(b));
    if dot < 0. {
        other = -other;
        dot = -dot;
    }
    let s = 1. - t;
    if dot < 0.9995 {
        let theta = dot.acos();
        let sin = theta.sin();
        a * ((s * theta).sin() / sin) + other * ((t * theta).sin() / sin)
    } else {
        (a * s + other * t).normalize()
    }
}
struct Drawing {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    start: Vector2,
    paint: bool,
    texture: GpuTexture,
    mips: crate::mipmap::MipGenerator,
    dirty: bool,
}
struct SpriteInfo {
    node: Object3D,
    center: Vector2,
    rotation: f64,
    attenuation: bool,
}
const SPRITE_BLUE: u32 = 0x6699ff;
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    pointer: Vector2,
    orbit: Orbit,
    objects: Vec<Object3D>,
    count: u32,
    target: Quaternion,
    next_target: f64,
    look_at: bool,
    drawing: Option<Drawing>,
    sprites: Vec<SpriteInfo>,
    selected: Option<usize>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            188 => (60., 0.1, 100., Vector3::splat(10.)),
            189 => (70., 0.01, 10., Vector3::new(0., 0., 5.)),
            190 => (90., 0.1, 100., Vector3::new(0., 0., 0.01)),
            191 => (50., 1., 2000., Vector3::new(0., 0., 500.)),
            _ => (50., 1., 1000., Vector3::splat(15.)),
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
            // webgl_instancing_raycast starts its mouse at (1, 1), off the grid.
            pointer: if id == 188 {
                Vector2::ONE
            } else {
                Vector2::ZERO
            },
            orbit: match id {
                188 => Orbit::new(position, true, 1., (0., f64::INFINITY)),
                190 => Orbit::new(position, true, -0.25, (0., f64::INFINITY)),
                _ => Orbit::new(position, false, 1., (15., 250.)),
            },
            objects: vec![],
            count: 1000,
            target: Quaternion::IDENTITY,
            next_target: 0.,
            look_at: false,
            drawing: None,
            sprites: vec![],
            selected: None,
        };
        match id {
            188 => {
                let light = s.insert(NodeKind::Light(Light::Hemisphere {
                    sky: Color::WHITE,
                    ground: Color::from_hex(0x888888),
                    intensity: 3.,
                }));
                s.get_mut(light)?.position = Vector3::Y;
                let mut m = MeshPhongMaterial::default();
                m.properties.color = Color::WHITE;
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(IcosahedronGeometry::build(0.5, 3)?),
                    Arc::new(Material::Phong(m)),
                )));
                let offset = 4.5;
                let mut instances = Vec::with_capacity(1000);
                for x in 0..10 {
                    for y in 0..10 {
                        for z in 0..10 {
                            instances.push(Instance {
                                matrix: Matrix4::from_translation(Vector3::new(
                                    offset - x as f64,
                                    offset - y as f64,
                                    offset - z as f64,
                                )),
                                color: Color::WHITE,
                            });
                        }
                    }
                }
                s.get_mut(h)?.instances = instances;
                d.objects.push(h);
            }
            189 => {
                let mut cone = CylinderGeometry::build(0., 0.1, 0.5, 8, 1, false, 0., TAU)?;
                cone.rotate_x(FRAC_PI_2)?;
                let cone = s.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(cone),
                    Arc::new(Material::Normal(MeshNormalMaterial::default())),
                )));
                let mut red = MeshBasicMaterial::default();
                red.properties.color = Color::from_hex(0xff0000);
                let target = s.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(SphereGeometry::build(0.05, 32, 16)?),
                    Arc::new(Material::Basic(red)),
                )));
                let mut wire = MeshBasicMaterial::default();
                // WebGL leaves MeshNormalMaterial unencoded, so this scene renders raw
                // display values; the wireframe blends its encoded 0xcccccc in display space.
                wire.properties.color = Color::linear(0.8, 0.8, 0.8);
                wire.properties.wireframe = true;
                wire.properties.transparent = true;
                wire.properties.opacity = 0.3;
                s.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(SphereGeometry::build(2., 32, 32)?),
                    Arc::new(Material::Basic(wire)),
                )));
                d.objects = vec![cone, target];
                // init() calls generateTarget before the first frame.
                d.generate_target(s)?;
            }
            190 => {
                let atlas = decode_texture_image(
                    &fetch("/web/gallery/assets/sun_temple_stripe.jpg").await?,
                )
                .await?;
                let (w, h) = (atlas.width, atlas.height);
                let pixels = &atlas.rgba;
                let mut materials = vec![];
                for i in 0..6 {
                    // drawImage( image, tileWidth * i, 0, tileWidth, tileWidth, ... ).
                    let mut tile = Vec::with_capacity((h * h * 4) as usize);
                    for row in 0..h {
                        let start = ((row * w + i * h) * 4) as usize;
                        tile.extend_from_slice(&pixels[start..start + (h * 4) as usize]);
                    }
                    let mut t = crate::material::Texture::from_rgba(h, h, tile, true)?;
                    t.mipmap_filter = Some(Filter::Linear);
                    let mut m = MeshBasicMaterial::default();
                    m.properties.map = Some(Arc::new(t));
                    materials.push(Arc::new(Material::Basic(m)));
                }
                let mut g = BoxGeometry::build(1., 1., 1.)?;
                g.scale(Vector3::new(1., 1., -1.))?;
                let mut mesh = Mesh::new(Arc::new(g), materials[0].clone());
                mesh.materials = materials;
                d.objects.push(s.insert(NodeKind::Mesh(mesh)));
            }
            191 => {
                // CanvasTexture: no color space, generated mipmaps, updated in place.
                let mut white =
                    crate::material::Texture::from_rgba(128, 128, vec![255; 128 * 128 * 4], false)?;
                white.mipmap_filter = Some(Filter::Linear);
                let texture = r.upload_texture(&Arc::new(white))?;
                let mips = crate::mipmap::MipGenerator::new(&r.device, &texture.texture)?;
                let mut m = MeshBasicMaterial::default();
                m.properties.vertex_program = Some(Arc::new(
                    SurfaceNodes {
                        color: Some(
                            tsl::Texture::External(0)
                                .sample(vec2(uv().x(), float(1.) - uv().y()))
                                .rgb(),
                        ),
                        ..Default::default()
                    }
                    .build(r, &[], &[(&texture.view, &texture.sampler)])
                    .await?,
                ));
                d.objects.push(s.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(BoxGeometry::build(200., 200., 200.)?),
                    Arc::new(Material::Basic(m)),
                ))));
                let document = web_sys::window()
                    .and_then(|w| w.document())
                    .ok_or(Error::Invalid("document"))?;
                let canvas: web_sys::HtmlCanvasElement = document
                    .get_element_by_id("drawing-canvas")
                    .ok_or(Error::Invalid("drawing canvas"))?
                    .dyn_into()
                    .map_err(|_| Error::Invalid("drawing canvas"))?;
                let context: web_sys::CanvasRenderingContext2d = canvas
                    .get_context("2d")
                    .ok()
                    .flatten()
                    .and_then(|c| c.dyn_into().ok())
                    .ok_or(Error::Invalid("2d context"))?;
                context.set_fill_style_str("#FFFFFF");
                context.fill_rect(0., 0., 128., 128.);
                d.drawing = Some(Drawing {
                    canvas,
                    context,
                    start: Vector2::ZERO,
                    paint: false,
                    texture,
                    mips,
                    dirty: true,
                });
            }
            _ => {
                s.background = Color::WHITE;
                let group = s.insert(NodeKind::Group);
                let group2 = s.insert(NodeKind::Group);
                s.add(group, group2)?;
                let n = s.get_mut(group2)?;
                n.scale = Vector3::new(1., 2., 1.);
                n.position = Vector3::new(-5., 0., 0.);
                n.quaternion = Quaternion::from_rotation_x(FRAC_PI_2);
                for (parent, position, scale, center, rotation, attenuation) in [
                    (
                        group,
                        Vector3::new(6., 5., 5.),
                        Vector3::new(2., 5., 1.),
                        Vector2::splat(0.5),
                        0.,
                        true,
                    ),
                    (
                        group,
                        Vector3::new(8., -2., 2.),
                        Vector3::new(0.1, 0.5, 0.1),
                        Vector2::new(0.5, 0.),
                        PI / 3. * 4.,
                        false,
                    ),
                    (
                        group2,
                        Vector3::new(0., 2., 5.),
                        Vector3::new(10., 2., 3.),
                        Vector2::new(-0.1, 0.),
                        PI / 3.,
                        true,
                    ),
                ] {
                    // The sprite shader's `position.xy - ( center - 0.5 )`, baked into the quad.
                    let mut g = PlaneGeometry::build(1., 1., 1, 1)?;
                    g.translate(Vector3::new(0.5 - center.x, 0.5 - center.y, 0.))?;
                    let mut sprite =
                        sprites::SpriteNodeMaterial::new(vec4(uniform(0, Type::Vec3), float(1.)));
                    sprite.rotation = float(rotation as f32);
                    sprite.size_attenuation = float(if attenuation { 1. } else { 0. });
                    let mut m = sprite.build(r, &[], &[]).await?;
                    set_sprite_color(&mut m, SPRITE_BLUE);
                    let h = s.insert(NodeKind::Mesh(Mesh::new(
                        Arc::new(g),
                        Arc::new(Material::Shader(m)),
                    )));
                    let n = s.get_mut(h)?;
                    n.position = position;
                    n.scale = scale;
                    n.frustum_culled = false;
                    s.add(parent, h)?;
                    d.sprites.push(SpriteInfo {
                        node: h,
                        center,
                        rotation,
                        attenuation,
                    });
                }
            }
        }
        Ok(d)
    }
    fn generate_target(&mut self, s: &mut Scene) -> Result<()> {
        let theta = random(&mut self.seed) * TAU;
        let phi = (2. * random(&mut self.seed) - 1.).acos();
        let position =
            Vector3::new(phi.sin() * theta.sin(), phi.cos(), phi.sin() * theta.cos()) * 2.;
        s.get_mut(self.objects[1])?.position = position;
        let mesh = s.get(self.objects[0])?.position;
        self.target = look_rotation(position, mesh, Vector3::Y);
        self.next_target += 2.;
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// One original animation frame, after the camera aspect is known.
    pub fn prepare(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        _aspect: f64,
    ) -> Result<()> {
        let delta = self.time - self.last;
        self.last = self.time;
        match self.id {
            188 => {
                self.orbit.update();
                self.orbit.apply(s, c)?;
                s.get_mut(self.objects[0])?.instance_count = Some(self.count);
                s.update()?;
                let (camera, world) = s.camera(c)?;
                let mut ray = Raycaster::default();
                ray.set_from_camera(self.pointer, camera, world)?;
                let hit = ray
                    .intersect_object(s, self.objects[0], false)?
                    .first()
                    .and_then(|h| h.instance_index);
                if let Some(i) = hit
                    && s.get(self.objects[0])?.instances[i].color == Color::WHITE
                {
                    let color = random_color(&mut self.seed);
                    s.get_mut(self.objects[0])?.instances[i].color = color;
                }
            }
            189 => {
                // setTimeout( generateTarget, 2000 ), on the example clock.
                while self.time >= self.next_target {
                    self.generate_target(s)?;
                }
                let mesh = s.get_mut(self.objects[0])?;
                if mesh.quaternion != self.target {
                    mesh.quaternion = if self.look_at {
                        self.target
                    } else {
                        rotate_towards(mesh.quaternion, self.target, FRAC_PI_2 * delta)
                    };
                }
            }
            190 => {
                self.orbit.update();
                self.orbit.apply(s, c)?;
            }
            191 => {
                // mesh.rotation.x/y += 0.01 per frame, as 60 fps time.
                s.get_mut(self.objects[0])?.quaternion = Euler {
                    angles: Vector3::new(self.time * 0.6, self.time * 0.6, 0.),
                    order: EulerOrder::XYZ,
                }
                .quaternion();
                if let Some(d) = &mut self.drawing
                    && d.dirty
                {
                    // A GPU copy, like WebGL's texImage2D( canvas ): CPU readbacks would
                    // switch Chrome's canvas to software rasterization and change the strokes.
                    r.queue.copy_external_image_to_texture(
                        &wgpu::CopyExternalImageSourceInfo {
                            source: wgpu::ExternalImageSource::HTMLCanvasElement(d.canvas.clone()),
                            origin: wgpu::Origin2d::ZERO,
                            flip_y: false,
                        },
                        wgpu::CopyExternalImageDestInfo {
                            texture: &d.texture.texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                            color_space: wgpu::PredefinedColorSpace::Srgb,
                            premultiplied_alpha: false,
                        },
                        wgpu::Extent3d {
                            width: 128,
                            height: 128,
                            depth_or_array_layers: 1,
                        },
                    );
                    d.mips.update(&r.device, &r.queue);
                    d.dirty = false;
                }
            }
            _ => self.orbit.apply(s, c)?,
        }
        Ok(())
    }
    /// Three.js `Sprite.raycast` for the three sprites, nearest first.
    fn raycast_sprites(&self, s: &Scene, c: Object3D) -> Result<Option<usize>> {
        let (camera, world) = s.camera(c)?;
        let mut ray = Raycaster::default();
        ray.set_from_camera(self.pointer, camera, world)?;
        let view = world.inverse();
        let mut hits = vec![];
        for (i, sprite) in self.sprites.iter().enumerate() {
            let matrix = s.get(sprite.node)?.matrix_world;
            let mut scale = Vector3::new(
                matrix.x_axis.truncate().length(),
                matrix.y_axis.truncate().length(),
                matrix.z_axis.truncate().length(),
            );
            let mv = (view * matrix).w_axis.truncate();
            if !sprite.attenuation {
                scale *= -mv.z;
            }
            let (sin, cos) = sprite.rotation.sin_cos();
            let vertex = |x: f64, y: f64| {
                let aligned = Vector2::new(
                    (x - sprite.center.x + 0.5) * scale.x,
                    (y - sprite.center.y + 0.5) * scale.y,
                );
                let rotated = Vector2::new(
                    cos * aligned.x - sin * aligned.y,
                    sin * aligned.x + cos * aligned.y,
                );
                world.transform_point3(mv + rotated.extend(0.))
            };
            let (a, b, cc) = (vertex(-0.5, -0.5), vertex(0.5, -0.5), vertex(0.5, 0.5));
            let point = ray
                .ray
                .intersect_triangle(a, b, cc, false)
                .or_else(|| ray.ray.intersect_triangle(a, cc, vertex(-0.5, 0.5), false));
            if let Some(point) = point {
                let distance = ray.ray.origin.distance(point);
                let Camera::Perspective(p) = camera else {
                    continue;
                };
                if distance >= p.near && distance <= p.far {
                    hits.push((distance, i));
                }
            }
        }
        hits.sort_by(|a, b| a.0.total_cmp(&b.0));
        Ok(hits.first().map(|h| h.1))
    }
    /// Pointer in NDC. The sprite example raycasts inside its pointermove handler.
    pub fn gpu_pointer(&mut self, s: &mut Scene, c: Object3D, x: f64, y: f64) -> Result<()> {
        self.pointer = Vector2::new(x, y);
        if self.id != 192 {
            return Ok(());
        }
        if let Some(i) = self.selected.take() {
            sprite_color(s, self.sprites[i].node, SPRITE_BLUE)?;
        }
        self.orbit.apply(s, c)?;
        s.update()?;
        if let Some(i) = self.raycast_sprites(s, c)? {
            sprite_color(s, self.sprites[i].node, 0xff0000)?;
            self.selected = Some(i);
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
        if ![188, 190, 192].contains(&self.id) {
            return Ok(());
        }
        if pan {
            if self.id != 192 {
                return Ok(());
            }
            let Camera::Perspective(p) = s.camera(c)?.0 else {
                return Ok(());
            };
            self.orbit.pan(s.get(c)?, p.fov, dx, dy, height);
        } else if wheel != 0. {
            if self.id != 192 {
                return Ok(());
            }
            self.orbit.dolly(wheel);
        } else {
            self.orbit.rotate(dx, dy, height);
        }
        // The pointer and wheel handlers call controls.update() themselves.
        self.orbit.update();
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (188, 0) if (0. ..=1000.).contains(&value) => self.count = value as u32,
            (189, 0) => self.look_at = value != 0.,
            _ => return Err(Error::Invalid("interactive object parameter")),
        }
        Ok(())
    }
    /// Canvas pointer events: 0 down, 1 move, 2 up or leave.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) -> Result<()> {
        let d = self
            .drawing
            .as_mut()
            .ok_or(Error::Invalid("drawing example"))?;
        match kind {
            0 => {
                d.paint = true;
                d.start = Vector2::new(x, y);
            }
            1 if d.paint => {
                // The original never begins a new path: each stroke redraws the whole path.
                d.context.move_to(d.start.x, d.start.y);
                d.context.set_stroke_style_str("#000000");
                d.context.line_to(x, y);
                d.context.stroke();
                d.start = Vector2::new(x, y);
                d.dirty = true;
            }
            _ => d.paint = false,
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
fn set_sprite_color(m: &mut ShaderMaterial, hex: u32) {
    // Raw sprite output: store the display (sRGB) value, as WebGL encodes before resolve.
    m.uniforms[0] = [
        ((hex >> 16) & 255) as f32 / 255.,
        ((hex >> 8) & 255) as f32 / 255.,
        (hex & 255) as f32 / 255.,
        1.,
    ];
}
fn sprite_color(s: &mut Scene, h: Object3D, hex: u32) -> Result<()> {
    if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind
        && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
    {
        set_sprite_color(m, hex);
    }
    Ok(())
}
