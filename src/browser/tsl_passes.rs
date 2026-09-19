//! Five official TSL examples: compute, RTT, composed passes, history and masks.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch};
use crate::{
    Error, Result,
    camera::*,
    geometry::*,
    material::{self, Filter, Material, MeshBasicMaterial, MeshPhongMaterial},
    math::*,
    postprocessing::Effect,
    renderer::{RenderTarget, RenderTargetOptions, Renderer},
    scene::*,
    tsl::{self, compute::TextureCompute, *},
};
use std::sync::Arc;
const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
fn target(
    renderer: &Renderer,
    w: u32,
    h: u32,
    format: wgpu::TextureFormat,
) -> Result<RenderTarget> {
    RenderTarget::with_options(
        &renderer.device,
        w,
        h,
        RenderTargetOptions {
            format,
            ..Default::default()
        },
    )
}
fn mesh(scene: &mut Scene, geometry: BufferGeometry, material: Material) -> Object3D {
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(geometry),
        Arc::new(material),
    )))
}
fn camera(
    scene: &mut Scene,
    handle: Object3D,
    fov: f64,
    near: f64,
    far: f64,
    position: Vector3,
) -> Result<()> {
    let aspect = match scene.camera(handle)?.0 {
        Camera::Perspective(c) => c.aspect,
        _ => 1.0,
    };
    scene.get_mut(handle)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov,
        aspect,
        near,
        far,
        ..Default::default()
    }));
    scene.get_mut(handle)?.position = position;
    scene.look_at(handle, Vector3::ZERO)
}
async fn map(url: &str, srgb: bool, mips: bool) -> Result<Arc<material::Texture>> {
    let mut map = decode_image(&fetch(url).await?).await?;
    map.srgb = srgb;
    map.mipmap_filter = mips.then_some(Filter::Linear);
    Ok(Arc::new(map))
}
struct Mask {
    scene: Scene,
    camera: Object3D,
    object: Object3D,
    target: RenderTarget,
}
pub(super) struct Demo {
    example: u32,
    time: f64,
    objects: Vec<Object3D>,
    targets: Vec<RenderTarget>,
    effects: Vec<Effect>,
    masks: Vec<Mask>,
    compute: Option<TextureCompute>,
    pictures: Vec<crate::texture_gpu::GpuTexture>,
    sampler: wgpu::Sampler,
    ping: usize,
    size: [u32; 2],
    pointer: Vector2,
    speed: f64,
    viewer: OrbitViewer,
    orbit_delta: Vector2,
    pan_delta: Vector3,
}
impl Demo {
    pub async fn create(
        scene: &mut Scene,
        cam: Object3D,
        example: u32,
        renderer: &Renderer,
    ) -> Result<Self> {
        let mut out = Self {
            example,
            time: 0.0,
            objects: Vec::new(),
            targets: Vec::new(),
            effects: Vec::new(),
            masks: Vec::new(),
            compute: None,
            pictures: Vec::new(),
            sampler: renderer.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            ping: 0,
            size: [1, 1],
            pointer: Vector2::ZERO,
            speed: 0.0,
            viewer: OrbitViewer::from_camera(Vector3::ZERO, 14.0f64.sqrt()),
            orbit_delta: Vector2::ZERO,
            pan_delta: Vector3::ZERO,
        };
        scene.background = Color::BLACK;
        match example {
            38 => {
                let index = instance_index();
                let x = index.clone().modulo(uint(512));
                let y = index / uint(512);
                let px = x.to_float() / float(50.0);
                let py = y.to_float() / float(50.0);
                let v = px.sin()
                    + py.sin()
                    + (px.clone() + py.clone()).sin()
                    + ((px.clone() * px + py.clone() * py).sqrt() + float(5.0)).sin();
                let color = vec4(
                    vec3(
                        v.sin(),
                        (v.clone() + float(std::f32::consts::PI)).sin(),
                        (v + float(std::f32::consts::PI) - float(0.5)).sin(),
                    ),
                    float(1.0),
                );
                let compute = TextureCompute::new(renderer, 512, 512, uvec2(x, y), color).await?;
                compute.dispatch(renderer)?;
                let material = NodeMaterial::new(tsl::Texture::External(0).sample(uv()))
                    .build(renderer, &[(&compute.view, &compute.sampler)])
                    .await?;
                out.objects.push(mesh(
                    scene,
                    PlaneGeometry::build(1.0, 1.0, 1, 1)?,
                    Material::Shader(material),
                ));
                scene.get_mut(cam)?.kind =
                    NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                        left: -1.0,
                        right: 1.0,
                        top: 1.0,
                        bottom: -1.0,
                        near: 0.0,
                        far: 2.0,
                        ..Default::default()
                    }));
                scene.get_mut(cam)?.position = Vector3::Z;
                out.compute = Some(compute);
            }
            39 => {
                camera(scene, cam, 70.0, 0.1, 10.0, Vector3::new(0.0, 0.0, 3.0))?;
                scene.background = Color::from_hex(0x0066ff);
                let mut material = MeshBasicMaterial::default();
                material.properties.map =
                    Some(map("/web/gallery/assets/uv-grid.jpg", false, true).await?);
                out.objects.push(mesh(
                    scene,
                    BoxGeometry::build(1.0, 1.0, 1.0)?,
                    Material::Basic(material),
                ));
                out.targets
                    .push(target(renderer, 1, 1, wgpu::TextureFormat::Rgba8Unorm)?);
                let color = hue(
                    saturation(
                        tsl::Texture::Input.sample(uv()).rgb(),
                        float(1.0) - uniform(0, Type::Vec2).x(),
                    ),
                    uniform(0, Type::Vec2).y(),
                );
                out.effects.push(effect(renderer, HDR, &color).await?);
            }
            40 => {
                camera(scene, cam, 70.0, 1.0, 1000.0, Vector3::new(0.0, 0.0, 400.0))?;
                scene.fog = Some(Fog::Linear {
                    color: Color::BLACK,
                    near: 1.0,
                    far: 1000.0,
                });
                let group = scene.insert(NodeKind::Group);
                out.objects.push(group);
                let geometry = Arc::new(SphereGeometry::build(1.0, 4, 4)?);
                let mut phong = MeshPhongMaterial::default();
                phong.properties.flat_shading = true;
                let material = Arc::new(Material::Phong(phong));
                let mut seed = 186_u32;
                let mut random = || {
                    seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                    seed as f64 / 4294967296.0
                };
                for _ in 0..100 {
                    let object = scene.insert(NodeKind::Mesh(Mesh::new(
                        geometry.clone(),
                        material.clone(),
                    )));
                    let node = scene.get_mut(object)?;
                    node.position = Vector3::new(random() - 0.5, random() - 0.5, random() - 0.5)
                        .normalize()
                        * random()
                        * 400.0;
                    node.quaternion = Quaternion::from_euler(
                        glam::EulerRot::XYZ,
                        random() * 2.0,
                        random() * 2.0,
                        random() * 2.0,
                    );
                    node.scale = Vector3::splat(random() * 50.0);
                    scene.add(group, object)?;
                }
                scene.insert(NodeKind::Light(Light::Ambient {
                    color: Color::from_hex(0xcccccc),
                    intensity: 1.0,
                }));
                let light = scene.insert(NodeKind::Light(Light::Directional {
                    color: Color::WHITE,
                    intensity: 3.0,
                    target: Vector3::ZERO,
                }));
                scene.get_mut(light)?.position = Vector3::ONE;
                out.targets.push(target(renderer, 1, 1, HDR)?);
                out.targets.push(target(renderer, 1, 1, HDR)?);
                let color = dot_screen(
                    tsl::Texture::Input.sample(uv()),
                    uv(),
                    uniform(0, Type::Vec2),
                    float(1.57),
                    float(0.3),
                );
                out.effects.push(effect(renderer, HDR, &color).await?);
                out.effects.push(
                    effect(
                        renderer,
                        HDR,
                        &rgb_shift(tsl::Texture::Input, uv(), float(0.001), float(0.0)),
                    )
                    .await?,
                );
            }
            41 => {
                camera(scene, cam, 50.0, 1.0, 100.0, Vector3::new(1.0, 2.0, 3.0))?;
                scene.background = Color::from_hex(0x0487e2);
                scene.fog = Some(Fog::Linear {
                    color: scene.background,
                    near: 7.0,
                    far: 25.0,
                });
                scene.tone_mapping = ToneMapping::Neutral;
                out.viewer
                    .fixture(1.0_f64.atan2(3.0), (2.0 / 14.0_f64.sqrt()).asin(), 1.8);
                let mut material = MeshBasicMaterial::default();
                material.properties.map = Some(map("/web/crate.gif", true, true).await?);
                out.objects.push(mesh(
                    scene,
                    BoxGeometry::build(1.0, 1.0, 1.0)?,
                    Material::Basic(material),
                ));
                out.targets.push(target(renderer, 1, 1, HDR)?);
                out.targets.push(target(renderer, 1, 1, HDR)?);
                let current = tsl::Texture::Input.sample(uv()).rgb();
                let previous = tsl::Texture::History.sample(uv()).rgb();
                let amount = (luminance((previous - current.clone()).abs()) * float(1000.0))
                    .clamp(float(0.0), float(3.0));
                out.effects
                    .push(effect(renderer, HDR, &saturation(current, amount)).await?);
            }
            42 => {
                camera(scene, cam, 50.0, 1.0, 1000.0, Vector3::new(0.0, 0.0, 10.0))?;
                scene.background = Color::from_hex(0xe0e0e0);
                out.targets.push(target(renderer, 1, 1, HDR)?);
                for geometry in [
                    BoxGeometry::build(4.0, 4.0, 4.0)?,
                    TorusGeometry::build(
                        3.0,
                        1.0,
                        16,
                        32,
                        std::f64::consts::TAU,
                        0.0,
                        std::f64::consts::TAU,
                    )?,
                ] {
                    let mut mask = Scene::new();
                    mask.background_alpha = 0.0;
                    let mc =
                        mask.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                            fov: 50.0,
                            near: 1.0,
                            far: 1000.0,
                            ..Default::default()
                        })));
                    mask.get_mut(mc)?.position.z = 10.0;
                    let object = mesh(
                        &mut mask,
                        geometry,
                        Material::Basic(MeshBasicMaterial::default()),
                    );
                    out.masks.push(Mask {
                        scene: mask,
                        camera: mc,
                        object,
                        target: target(renderer, 1, 1, HDR)?,
                    });
                }
                for (url, mips) in [
                    ("/web/gallery/assets/caravaggio.jpg", false),
                    ("/web/gallery/assets/panorama.jpg", true),
                ] {
                    let image = map(url, true, mips).await?;
                    out.pictures.push(renderer.upload_texture(&image)?);
                }
                let base = tsl::Texture::External(2).sample(uv());
                let first = mix(
                    base,
                    tsl::Texture::External(0).sample(uv()),
                    tsl::Texture::Input.sample(uv()).swizzle("w"),
                );
                let color = mix(
                    first,
                    tsl::Texture::External(1).sample(uv()),
                    tsl::Texture::History.sample(uv()).swizzle("w"),
                );
                out.effects.push(
                    effect_with_textures(
                        renderer,
                        HDR,
                        &color,
                        &[
                            (&out.pictures[0].view, &out.pictures[0].sampler),
                            (&out.pictures[1].view, &out.pictures[1].sampler),
                            (&out.targets[0].view, &out.sampler),
                        ],
                    )
                    .await?,
                );
            }
            _ => return Err(Error::Invalid("TSL pass example")),
        }
        Ok(out)
    }
    pub fn seek(&mut self, seconds: f64) {
        self.time = seconds;
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if self.example != 41 || index != 0 || !(0.0..=2.0).contains(&value) {
            return Err(Error::Invalid("TSL pass parameter"));
        }
        self.speed = value as f64;
        Ok(())
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new((x + 1.0) * 0.5, (1.0 - y) * 0.5);
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        scene: &Scene,
        cam: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if self.example == 41 {
            if pan {
                self.pan_delta +=
                    OrbitViewer::pan_delta(scene, cam, self.viewer.radius(), dx, dy, height)?;
            } else {
                self.orbit_delta += Vector2::new(dx, dy);
                self.viewer.orbit_pixels(0.0, 0.0, wheel, height, 2.0, 10.0);
            }
        }
        Ok(())
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        cam: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        if animate {
            self.time += if [39, 40].contains(&self.example) {
                1.0 / 60.0
            } else if self.example == 41 {
                delta * self.speed
            } else {
                delta
            };
        }
        match self.example {
            39 | 40 => {
                let amount = if self.example == 39 { 0.6 } else { 0.3 };
                scene.get_mut(self.objects[0])?.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    self.time * amount,
                    self.time * amount * 2.0,
                    0.0,
                );
            }
            41 => {
                scene.get_mut(self.objects[0])?.quaternion =
                    Quaternion::from_rotation_y(self.time * 5.0);
                let height = web_sys::window()
                    .unwrap()
                    .inner_height()
                    .unwrap()
                    .as_f64()
                    .unwrap_or(512.0);
                self.viewer.orbit_pixels(
                    self.orbit_delta.x * 0.01,
                    self.orbit_delta.y * 0.01,
                    0.0,
                    height,
                    2.0,
                    10.0,
                );
                self.orbit_delta *= 0.99;
                self.viewer.pan_world(self.pan_delta * 0.01);
                self.pan_delta *= 0.99;
                self.viewer.update(scene, cam)?;
            }
            42 => {
                let t = self.time + 6000.0;
                for (i, mask) in self.masks.iter_mut().enumerate() {
                    let node = mask.scene.get_mut(mask.object)?;
                    node.position = if i == 0 {
                        Vector3::new((t / 1.5).cos() * 2.0, t.sin() * 2.0, 0.0)
                    } else {
                        Vector3::new(t.cos() * 2.0, (t / 1.5).sin() * 2.0, 0.0)
                    };
                    node.quaternion = Quaternion::from_euler(glam::EulerRot::XYZ, t, t / 2.0, 0.0);
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        renderer: &Renderer,
        scene: &mut Scene,
        cam: Object3D,
        output: &RenderTarget,
    ) -> Result<bool> {
        let aspect = output.width as f64 / output.height as f64;
        if self.example == 38 {
            if let NodeKind::Camera(Camera::Orthographic(c)) = &mut scene.get_mut(cam)?.kind {
                c.left = -aspect;
                c.right = aspect;
            }
            return Ok(false);
        }
        if self.size != [output.width, output.height] {
            self.size = [output.width, output.height];
            for t in &mut self.targets {
                t.set_size(&renderer.device, output.width, output.height)?;
            }
            for mask in &mut self.masks {
                mask.target
                    .set_size(&renderer.device, output.width, output.height)?;
                if let NodeKind::Camera(Camera::Perspective(c)) =
                    &mut mask.scene.get_mut(mask.camera)?.kind
                {
                    c.aspect = aspect;
                }
            }
            if self.example == 42 {
                self.effects[0].set_textures(
                    renderer,
                    &[
                        (&self.pictures[0].view, &self.pictures[0].sampler),
                        (&self.pictures[1].view, &self.pictures[1].sampler),
                        (&self.targets[0].view, &self.sampler),
                    ],
                )?;
            }
        }
        match self.example {
            39 => {
                renderer.render(scene, cam, &self.targets[0])?;
                self.effects[0].parameters[0] =
                    [self.pointer.x as f32, self.pointer.y as f32, 0.0, 0.0];
                self.effects[0].apply(renderer, &self.targets[0], None, output)?;
            }
            40 => {
                renderer.render(scene, cam, &self.targets[0])?;
                self.effects[0].parameters[0] =
                    [output.width as f32, output.height as f32, 0.0, 0.0];
                self.effects[0].apply(renderer, &self.targets[0], None, &self.targets[1])?;
                self.effects[1].apply(renderer, &self.targets[1], None, output)?;
            }
            41 => {
                renderer.render(scene, cam, &self.targets[self.ping])?;
                self.effects[0].apply(
                    renderer,
                    &self.targets[self.ping],
                    Some(&self.targets[1 - self.ping]),
                    output,
                )?;
                self.ping = 1 - self.ping;
            }
            42 => {
                renderer.render(scene, cam, &self.targets[0])?;
                for mask in &mut self.masks {
                    renderer.render(&mut mask.scene, mask.camera, &mask.target)?;
                }
                self.effects[0].apply(
                    renderer,
                    &self.masks[0].target,
                    Some(&self.masks[1].target),
                    output,
                )?;
            }
            _ => {}
        }
        Ok(true)
    }
}
