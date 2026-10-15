//! Official direct, radial blur, FXAA, SSAA and transition examples.
use super::gltf_viewer::{decode_image, fetch};
use crate::{
    Error, Result,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    postprocessing::{Effect, ssaa::SsaaPass},
    renderer::*,
    scene::*,
    tsl::{self, display::*, *},
};
use std::sync::Arc;
const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
fn target(r: &Renderer, samples: u32) -> Result<RenderTarget> {
    RenderTarget::with_options(
        &r.device,
        1,
        1,
        RenderTargetOptions {
            format: HDR,
            samples,
            ..Default::default()
        },
    )
}
fn mesh(s: &mut Scene, g: BufferGeometry, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(Arc::new(g), Arc::new(m))))
}
fn camera(s: &mut Scene, c: Object3D, fov: f64, near: f64, far: f64, z: f64) -> Result<()> {
    s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov,
        near,
        far,
        ..Default::default()
    }));
    s.get_mut(c)?.position = Vector3::new(0.0, 0.0, z);
    Ok(())
}
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.0
}
fn directional(s: &mut Scene, position: Vector3, intensity: f64) -> Result<()> {
    let h = s.insert(NodeKind::Light(Light::Directional {
        color: Color::WHITE,
        intensity,
        target: Vector3::ZERO,
    }));
    s.get_mut(h)?.position = position;
    Ok(())
}
struct Secondary {
    scene: Scene,
    camera: Object3D,
    mesh: Object3D,
}
pub(super) struct Demo {
    example: u32,
    object: Object3D,
    time: f64,
    clock: f64,
    params: [f32; 7],
    effects: Vec<Effect>,
    targets: Vec<RenderTarget>,
    ssaa: Option<SsaaPass>,
    secondary: Option<Secondary>,
    pictures: Vec<GpuTexture>,
    texture_index: usize,
    tween_start: f64,
    tween_reversed: bool,
}
impl Demo {
    pub async fn create(
        scene: &mut Scene,
        cam: Object3D,
        example: u32,
        renderer: &Renderer,
    ) -> Result<Self> {
        let object = scene.insert(NodeKind::Group);
        let mut out = Self {
            example,
            object,
            time: 0.0,
            clock: 0.0,
            params: [0.0; 7],
            effects: vec![],
            targets: vec![],
            ssaa: None,
            secondary: None,
            pictures: vec![],
            texture_index: 5,
            tween_start: 2.0,
            tween_reversed: false,
        };
        scene.background = Color::BLACK;
        let mut seed = 186;
        match example {
            43 => {
                camera(scene, cam, 70.0, 1.0, 1000.0, 400.0)?;
                scene.tone_mapping = ToneMapping::Neutral;
                let program = Arc::new(
                    output_program(
                        renderer,
                        &vec4(
                            saturation(output().rgb(), uniform(0, Type::Float)),
                            output().swizzle("w"),
                        ),
                    )
                    .await?,
                );
                let geometry = Arc::new(SphereGeometry::build(1.0, 4, 4)?);
                for _ in 0..100 {
                    let mut mat = MeshPhongMaterial::default();
                    mat.properties.color =
                        Color::from_hex((random(&mut seed) * 0xffffff as f64) as u32);
                    mat.properties.flat_shading = true;
                    mat.properties.vertex_program = Some(program.clone());
                    let h = scene.insert(NodeKind::Mesh(Mesh::new(
                        geometry.clone(),
                        Arc::new(Material::Phong(mat)),
                    )));
                    let n = scene.get_mut(h)?;
                    n.position = Vector3::new(
                        random(&mut seed) - 0.5,
                        random(&mut seed) - 0.5,
                        random(&mut seed) - 0.5,
                    )
                    .normalize()
                        * random(&mut seed)
                        * 400.0;
                    n.quaternion = Quaternion::from_euler(
                        glam::EulerRot::XYZ,
                        random(&mut seed) * 2.0,
                        random(&mut seed) * 2.0,
                        random(&mut seed) * 2.0,
                    );
                    n.scale = Vector3::splat(random(&mut seed) * 50.0);
                    scene.add(object, h)?;
                }
                scene.insert(NodeKind::Light(Light::Ambient {
                    color: Color::from_hex(0xcccccc),
                    intensity: 1.0,
                }));
                directional(scene, Vector3::ONE, 3.0)?;
            }
            44 | 45 => {
                camera(scene, cam, 45.0, 0.1, 200.0, 50.0)?;
                let light = scene.insert(NodeKind::Light(Light::Hemisphere {
                    sky: Color::WHITE,
                    ground: Color::from_hex(0x8d8d8d),
                    intensity: 1.0,
                }));
                scene.get_mut(light)?.position = Vector3::new(0.0, 1000.0, 0.0);
                if example == 44 {
                    scene.tone_mapping = ToneMapping::Neutral;
                    scene.insert(NodeKind::Light(Light::Point {
                        color: Color::WHITE,
                        intensity: 1000.0,
                        distance: 0.0,
                        decay: 2.0,
                    }));
                    out.params = [0.9, 0.95, 32.0, 5.0, 1.0, 1.0, 0.0];
                } else {
                    scene.background = Color::WHITE;
                    directional(scene, Vector3::new(-3000.0, 1000.0, -1000.0), 3.0)?;
                    out.params = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
                }
                let mut mat = MeshStandardMaterial {
                    energy_conservation: true,
                    ..Default::default()
                };
                mat.properties.flat_shading = true;
                if example == 45 {
                    mat.properties.color = Color::from_hex(0xf73232);
                }
                let h = mesh(
                    scene,
                    TetrahedronGeometry::build(1.0, 0)?,
                    Material::Standard(mat),
                );
                scene.add(object, h)?;
                for i in 0..100 {
                    let mut position = Vector3::new(
                        random(&mut seed) * 50.0 - 25.0,
                        random(&mut seed) * 50.0 - 25.0,
                        random(&mut seed) * 50.0 - 25.0,
                    );
                    if example == 44 {
                        let xy = Vector2::new(position.x, position.y);
                        if xy.length() < 6.0 {
                            let xy = xy.normalize() * 6.0;
                            position.x = xy.x;
                            position.y = xy.y;
                        }
                    }
                    let scale = Vector3::splat(random(&mut seed) * 2.0 + 1.0);
                    let rotation = Quaternion::from_euler(
                        glam::EulerRot::XYZ,
                        random(&mut seed) * std::f64::consts::PI,
                        random(&mut seed) * std::f64::consts::PI,
                        random(&mut seed) * std::f64::consts::PI,
                    );
                    scene.get_mut(h)?.instances.push(Instance {
                        matrix: Matrix4::from_scale_rotation_translation(scale, rotation, position),
                        color: if example == 44 {
                            Color::from_hsl(0.55 + i as f64 / 100.0 * 0.15, 1.0, 0.2)
                        } else {
                            Color::WHITE
                        },
                    });
                }
                out.targets.push(target(renderer, 1)?);
                if example == 44 {
                    out.effects.push(
                        effect(
                            renderer,
                            HDR,
                            &radial_blur(
                                tsl::Texture::Input,
                                uv(),
                                uniform(1, Type::Vec2),
                                uniform(0, Type::Vec4),
                            ),
                        )
                        .await?,
                    );
                } else {
                    out.targets.push(target(renderer, 1)?);
                    out.effects.push(
                        effect(
                            renderer,
                            HDR,
                            &vec4(srgb(tsl::Texture::Input.sample(uv()).rgb()), float(1.0)),
                        )
                        .await?,
                    );
                    out.effects.push(
                        effect(
                            renderer,
                            HDR,
                            &fxaa(tsl::Texture::Input, uv(), uniform(0, Type::Vec2)),
                        )
                        .await?,
                    );
                }
            }
            46 => {
                camera(scene, cam, 65.0, 3.0, 10.0, 7.0)?;
                out.params = [3.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0];
                for (color, x, y) in [
                    (0xefffef, -10.0, -10.0),
                    (0xffefef, -10.0, 10.0),
                    (0xefefff, 10.0, -10.0),
                ] {
                    let h = scene.insert(NodeKind::Light(Light::Point {
                        color: Color::from_hex(color),
                        intensity: 500.0,
                        distance: 0.0,
                        decay: 2.0,
                    }));
                    scene.get_mut(h)?.position = Vector3::new(x, y, 10.0);
                }
                scene.insert(NodeKind::Light(Light::Ambient {
                    color: Color::WHITE,
                    intensity: 0.2,
                }));
                let h = mesh(
                    scene,
                    SphereGeometry::build(3.0, 48, 24)?,
                    Material::Standard(MeshStandardMaterial {
                        energy_conservation: true,
                        ..Default::default()
                    }),
                );
                out.object = h;
                for _ in 0..120 {
                    let position = Vector3::new(
                        random(&mut seed) * 4.0 - 2.0,
                        random(&mut seed) * 4.0 - 2.0,
                        random(&mut seed) * 4.0 - 2.0,
                    );
                    let rotation = Quaternion::from_euler(
                        glam::EulerRot::XYZ,
                        random(&mut seed),
                        random(&mut seed),
                        random(&mut seed),
                    );
                    let scale = Vector3::splat(random(&mut seed) * 0.2 + 0.05);
                    scene.get_mut(h)?.instances.push(Instance {
                        matrix: Matrix4::from_scale_rotation_translation(scale, rotation, position),
                        color: Color::from_hsl(random(&mut seed), 1.0, 0.3),
                    });
                }
                out.ssaa = Some(SsaaPass::new(renderer).await?);
                out.targets.push(target(renderer, 1)?);
                out.effects.push(
                    effect(
                        renderer,
                        HDR,
                        &premultiplied_srgb(tsl::Texture::Input.sample(uv())),
                    )
                    .await?,
                );
            }
            47 => {
                out.params = [1.0, 1.0, 0.0, 1.0, 5.0, 1.0, 0.1];
                out.object = transition_scene(scene, cam, true, &mut seed)?;
                let mut second = Scene::new();
                let c = second.insert(NodeKind::Camera(Camera::Perspective(
                    PerspectiveCamera::default(),
                )));
                let h = transition_scene(&mut second, c, false, &mut seed)?;
                out.secondary = Some(Secondary {
                    scene: second,
                    camera: c,
                    mesh: h,
                });
                out.targets = vec![target(renderer, 4)?, target(renderer, 4)?];
                for i in 1..=6 {
                    let mut texture = decode_image(
                        &fetch(&format!("/web/gallery/assets/transition{i}.png")).await?,
                    )
                    .await?;
                    texture.srgb = false;
                    texture.mipmap_filter = Some(Filter::Linear);
                    out.pictures
                        .push(renderer.upload_texture(&Arc::new(texture))?);
                }
                let node = transition(
                    tsl::Texture::Input.sample(uv()),
                    tsl::Texture::History.sample(uv()),
                    tsl::Texture::External(0)
                        .sample(vec2(uv().x(), float(1.0) - uv().y()))
                        .x(),
                    uniform(0, Type::Vec3).x(),
                    uniform(0, Type::Vec3).y(),
                    uniform(0, Type::Vec3).swizzle("z"),
                );
                out.effects.push(
                    effect_with_textures(
                        renderer,
                        HDR,
                        &node,
                        &[(&out.pictures[5].view, &out.pictures[5].sampler)],
                    )
                    .await?,
                );
                out.effects
                    .push(effect(renderer, HDR, &tsl::Texture::Input.sample(uv())).await?);
            }
            _ => return Err(Error::Invalid("TSL filter example")),
        }
        Ok(out)
    }
    pub fn seek(&mut self, time: f64) {
        self.time = time;
        self.clock = time;
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let ranges: &[(f32, f32)] = match self.example {
            43 => &[(0.0, 1.0)],
            44 => &[
                (0.0, 1.0),
                (0.0, 1.0),
                (16.0, 64.0),
                (1.0, 10.0),
                (0.0, 1.0),
                (0.0, 1.0),
            ],
            45 => &[(0.0, 1.0), (0.0, 1.0)],
            46 => &[
                (0.0, 5.0),
                (0.0, 4.0),
                (0.0, 1.0),
                (-100.0, 100.0),
                (0.0, 1.0),
            ],
            47 => &[
                (0.0, 1.0),
                (0.0, 1.0),
                (0.0, 1.0),
                (0.0, 1.0),
                (0.0, 5.0),
                (0.0, 1.0),
                (0.0, 1.0),
            ],
            _ => &[],
        };
        if !value.is_finite()
            || ranges
                .get(index)
                .is_none_or(|(lo, hi)| value < *lo || value > *hi)
        {
            return Err(Error::Invalid("TSL filter parameter"));
        }
        self.params[index] = value;
        Ok(())
    }
    pub fn update(&mut self, scene: &mut Scene, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.clock += delta;
            let enabled = match self.example {
                43 => true,
                44 => self.params[5] > 0.5,
                45 => self.params[1] > 0.5,
                46 => self.params[4] > 0.5,
                _ => self.params[0] > 0.5,
            };
            if enabled {
                self.time += if self.example == 43 {
                    1.0 / 60.0
                } else {
                    delta
                };
            }
        }
        let (x, y, z) = match self.example {
            43 => (self.time * 0.3, self.time * 0.6, 0.0),
            44 | 45 => (0.0, self.time * 0.1, 0.0),
            46 => (self.time * 0.25, self.time * 0.5, 0.0),
            _ => (0.0, -self.time * 0.4, 0.0),
        };
        scene.get_mut(self.object)?.quaternion =
            Quaternion::from_euler(glam::EulerRot::XYZ, x, y, z);
        if self.example == 43 {
            for h in scene.traverse(self.object, true)? {
                if let NodeKind::Mesh(m) = &mut scene.get_mut(h)?.kind {
                    Arc::make_mut(&mut m.materials[0])
                        .properties_mut()
                        .vertex_uniforms[0][0] = self.params[0];
                }
            }
        }
        if let Some(second) = &mut self.secondary {
            second.scene.get_mut(second.mesh)?.quaternion =
                Quaternion::from_euler(glam::EulerRot::XYZ, 0.0, self.time * 0.2, self.time * 0.1);
            // Match the upstream tween's delay, yoyo and endpoint callbacks,
            // including a frame landing exactly on the next transition's start.
            if self.params[1] > 0.5 && self.clock >= self.tween_start {
                let elapsed = self.clock - self.tween_start;
                let t = ((elapsed % 3.5) / 1.5).min(1.0);
                self.params[2] = if self.tween_reversed { 1.0 - t } else { t } as f32;
                if self.params[5] > 0.5 && (t == 0.0 || t == 1.0) {
                    self.params[4] = (self.params[4] + 1.0) % 6.0;
                }
                if elapsed >= 1.5 {
                    self.tween_reversed = !self.tween_reversed;
                    self.tween_start += 3.5 * (((elapsed - 1.5) / 3.5).floor() + 1.0);
                }
            }
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
        for t in &mut self.targets {
            t.set_size(&renderer.device, output.width, output.height)?;
        }
        match self.example {
            43 => return Ok(false),
            44 => {
                if self.params[4] <= 0.5 {
                    renderer.render(scene, cam, output)?;
                    return Ok(true);
                }
                renderer.render(scene, cam, &self.targets[0])?;
                self.effects[0].parameters[0] = self.params[..4].try_into().unwrap();
                self.effects[0].parameters[1] =
                    [output.width as f32, output.height as f32, 0.0, 0.0];
                self.effects[0].apply(renderer, &self.targets[0], None, output)?;
            }
            45 => {
                renderer.render(scene, cam, &self.targets[0])?;
                self.effects[0].apply(
                    renderer,
                    &self.targets[0],
                    None,
                    if self.params[0] > 0.5 {
                        &self.targets[1]
                    } else {
                        output
                    },
                )?;
                if self.params[0] > 0.5 {
                    self.effects[1].parameters[0] = [
                        1.0 / output.width as f32,
                        1.0 / output.height as f32,
                        0.0,
                        0.0,
                    ];
                    self.effects[1].apply(renderer, &self.targets[1], None, output)?;
                }
            }
            46 => {
                scene.background = Color::from_hex(
                    [0, 0xffffff, 0x0000ff, 0x00ff00, 0xff0000][self.params[1] as usize],
                );
                scene.background_alpha = self.params[2] as f64;
                if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(cam)?.kind {
                    c.view = Some(ViewOffset {
                        full_width: output.width as f64,
                        full_height: output.height as f64,
                        offset_x: self.params[3] as f64,
                        offset_y: 0.0,
                        width: output.width as f64,
                        height: output.height as f64,
                    });
                }
                let ssaa = self.ssaa.as_mut().unwrap();
                ssaa.sample_level = self.params[0] as u32;
                ssaa.render(renderer, scene, cam, &self.targets[0])?;
                self.effects[0].apply(renderer, &self.targets[0], None, output)?;
            }
            47 => {
                let second = self.secondary.as_mut().unwrap();
                if let NodeKind::Camera(Camera::Perspective(c)) =
                    &mut second.scene.get_mut(second.camera)?.kind
                {
                    c.aspect = output.width as f64 / output.height as f64;
                }
                let ratio = self.params[2];
                if ratio == 0.0 {
                    renderer.render(&mut second.scene, second.camera, &self.targets[1])?;
                    self.effects[1].apply(renderer, &self.targets[1], None, output)?;
                } else if ratio == 1.0 {
                    renderer.render(scene, cam, &self.targets[0])?;
                    self.effects[1].apply(renderer, &self.targets[0], None, output)?;
                } else {
                    renderer.render(scene, cam, &self.targets[0])?;
                    renderer.render(&mut second.scene, second.camera, &self.targets[1])?;
                    let index = self.params[4] as usize;
                    if self.texture_index != index {
                        let pic = &self.pictures[index];
                        self.effects[0].set_textures(renderer, &[(&pic.view, &pic.sampler)])?;
                        self.texture_index = index;
                    }
                    self.effects[0].parameters[0] = [ratio, self.params[6], self.params[3], 0.0];
                    self.effects[0].apply(
                        renderer,
                        &self.targets[0],
                        Some(&self.targets[1]),
                        output,
                    )?;
                }
            }
            _ => {}
        }
        Ok(true)
    }
}
fn transition_scene(
    scene: &mut Scene,
    cam: Object3D,
    boxes: bool,
    seed: &mut u32,
) -> Result<Object3D> {
    camera(scene, cam, 50.0, 0.1, 100.0, 20.0)?;
    scene.background = if boxes { Color::WHITE } else { Color::BLACK };
    scene.insert(NodeKind::Light(Light::Ambient {
        color: Color::from_hex(0xaaaaaa),
        intensity: 3.0,
    }));
    directional(scene, Vector3::new(0.0, 1.0, 4.0), 3.0)?;
    let mut mat = MeshPhongMaterial::default();
    mat.properties.color = Color::from_hex(if boxes { 0x0000ff } else { 0xff0000 });
    mat.properties.flat_shading = true;
    let h = mesh(
        scene,
        if boxes {
            BoxGeometry::build(2.0, 2.0, 2.0)?
        } else {
            IcosahedronGeometry::build(1.0, 1)?
        },
        Material::Phong(mat),
    );
    for _ in 0..500 {
        let pos = Vector3::new(
            random(seed) * 100.0 - 50.0,
            random(seed) * 60.0 - 30.0,
            random(seed) * 80.0 - 40.0,
        );
        let rot = Quaternion::from_euler(
            glam::EulerRot::XYZ,
            random(seed) * std::f64::consts::TAU,
            random(seed) * std::f64::consts::TAU,
            random(seed) * std::f64::consts::TAU,
        );
        let x = random(seed) * 2.0 + 1.0;
        let scale = if boxes {
            Vector3::new(x, random(seed) * 2.0 + 1.0, random(seed) * 2.0 + 1.0)
        } else {
            Vector3::splat(x)
        };
        let c = 0.1 + 0.9 * random(seed);
        scene.get_mut(h)?.instances.push(Instance {
            matrix: Matrix4::from_scale_rotation_translation(scale, rot, pos),
            color: Color::linear(c, c, c),
        });
    }
    Ok(h)
}
