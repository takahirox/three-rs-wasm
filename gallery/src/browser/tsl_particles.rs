//! Pinned TSL height fog, sprites, instanced sprites, galaxy and afterimage scenes.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch};
use crate::tsl::Node;
use crate::{
    Error, Result,
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    geometry::*,
    material::*,
    math::*,
    postprocessing::afterimage::AfterImagePass,
    renderer::*,
    scene::*,
    tsl::{self, sprites::*, *},
};
use std::sync::Arc;
fn rand(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.0
}
fn rgb(c: Color) -> Node {
    vec3(
        float(c.0.x as f32),
        float(c.0.y as f32),
        float(c.0.z as f32),
    )
}
fn scalar(i: usize) -> Node {
    uniform(i, Type::Float)
}
fn add_mesh(scene: &mut Scene, g: Arc<BufferGeometry>, m: Arc<Material>) -> Object3D {
    scene.insert(NodeKind::Mesh(Mesh::new(g, m)))
}
fn attribute(renderer: &Renderer, data: &[[f32; 4]]) -> Result<GpuBuffer> {
    GpuBuffer::new(renderer, bytemuck::cast_slice(data), BufferAccess::Read)
}
fn additive(m: &mut ShaderMaterial) {
    m.properties.depth_write = false;
    m.properties.blending = Some(wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
    });
}
pub(super) struct Demo {
    example: u32,
    time: f64,
    frames: f64,
    objects: Vec<Object3D>,
    group: Object3D,
    params: [f32; 4],
    size: Vector2,
    viewer: OrbitViewer,
    orbit: Vector2,
    pan: Vector3,
    pointer: Vector2,
    history: Option<AfterImagePass>,
    enabled: bool,
}
impl Demo {
    pub async fn create(
        scene: &mut Scene,
        cam: Object3D,
        example: u32,
        r: &Renderer,
    ) -> Result<Self> {
        let (fov, near, far, position) = match example {
            48 => (45.0, 1.0, 600.0, Vector3::new(20.0, 10.0, 25.0)),
            49 => (60.0, 1.0, 2100.0, Vector3::new(0.0, 0.0, 1500.0)),
            50 => (55.0, 2.0, 2000.0, Vector3::new(0.0, 0.0, 1000.0)),
            51 => (50.0, 0.1, 100.0, Vector3::new(4.0, 2.0, 5.0)),
            _ => (60.0, 1.0, 10000.0, Vector3::new(0.0, 0.0, 1000.0)),
        };
        let aspect = if let Camera::Perspective(c) = scene.camera(cam)?.0 {
            c.aspect
        } else {
            1.0
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            aspect,
            near,
            far,
            ..Default::default()
        }));
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, Vector3::ZERO)?;
        scene.background = Color::BLACK;
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
        viewer.fixture(
            position.x.atan2(position.z),
            (position.y / position.length()).asin(),
            1.8,
        );
        let group = scene.insert(NodeKind::Group);
        let mut out = Self {
            example,
            time: 0.0,
            frames: 0.0,
            objects: vec![],
            group,
            params: [0.0; 4],
            size: Vector2::ONE,
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            pointer: Vector2::ZERO,
            history: None,
            enabled: true,
        };
        let mut seed = 186;
        if example == 48 {
            let fog = Color::from_hex(0xffdfc1);
            scene.background = fog;
            let factor = exponential_height_fog_factor(scalar(0), scalar(1));
            let program = Arc::new(
                output_program(
                    r,
                    &vec4(mix(output().rgb(), rgb(fog), factor), output().swizzle("w")),
                )
                .await?,
            );
            let mut m = MeshPhongMaterial::default();
            m.properties.color = Color::from_hex(0xcd959a);
            m.properties.vertex_program = Some(program);
            let h = add_mesh(
                scene,
                Arc::new(BoxGeometry::build(1.0, 25.0, 1.0)?),
                Arc::new(Material::Phong(m)),
            );
            scene.get_mut(h)?.position.y = -10.0;
            for i in 0..10 {
                for j in 0..10 {
                    scene.get_mut(h)?.instances.push(Instance {
                        matrix: Matrix4::from_translation(Vector3::new(
                            -18.0 + i as f64 * 4.0,
                            0.0,
                            -18.0 + j as f64 * 4.0,
                        )),
                        color: Color::WHITE,
                    });
                }
            }
            let light = scene.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(0xffc0cb),
                intensity: 2.0,
                target: Vector3::ZERO,
            }));
            scene.get_mut(light)?.position = Vector3::new(-10.0, 10.0, 10.0);
            scene.insert(NodeKind::Light(Light::Ambient {
                color: Color::from_hex(0xcccccc),
                intensity: 1.0,
            }));
            out.objects.push(h);
            out.params = [0.04, 2.0, 0.0, 0.0];
            return Ok(out);
        }
        let mut geometry = PlaneGeometry::build(1.0, 1.0, 1, 1)?;
        let picture = if example == 51 {
            None
        } else {
            let name = match example {
                49 => "sprite1.png",
                50 => "snowflake1.png",
                _ => "circle.png",
            };
            let mut image = decode_image(&fetch(&format!("assets/{name}")).await?).await?;
            out.size = Vector2::new(image.width as f64, image.height as f64);
            image.srgb = example == 50;
            image.mipmap_filter = Some(Filter::Linear);
            Some(r.upload_texture(&Arc::new(image))?)
        };
        let textures: Vec<_> = picture.iter().map(|p| (&p.view, &p.sampler)).collect();
        let tex = tsl::Texture::External(0).sample(vec2(uv().x(), float(1.0) - uv().y()));
        let mut buffers = vec![];
        let mut sprite = match example {
            49 => {
                scene.fog = Some(Fog::Linear {
                    color: Color::from_hex(0x0000ff),
                    near: 1500.0,
                    far: 2100.0,
                });
                let color = (tex.clone()
                    * vec4(vec3(uv().x(), uv().y(), float(0.0)), float(1.0))
                    * float(2.0))
                .clamp(float(0.0), float(1.0));
                let mut s = SpriteNodeMaterial::new(vec4(
                    color.rgb(),
                    color.swizzle("w") * tex.swizzle("w"),
                ));
                s.rotation = scalar(1);
                s
            }
            50 => {
                scene.background_alpha = 0.0;
                geometry.instance_count = Some(10000);
                let data = (0..10000)
                    .map(|_| {
                        [
                            (rand(&mut seed) * 2000.0 - 1000.0) as f32,
                            (rand(&mut seed) * 2000.0 - 1000.0) as f32,
                            (rand(&mut seed) * 2000.0 - 1000.0) as f32,
                            0.0,
                        ]
                    })
                    .collect::<Vec<_>>();
                buffers.push(attribute(r, &data)?);
                scene.fog = Some(Fog::Exp2 {
                    color: Color::BLACK,
                    density: 0.001,
                });
                let mut s = SpriteNodeMaterial::new(vec4(
                    tex.rgb() * uniform(2, Type::Vec3),
                    tex.swizzle("w") * tex.x(),
                ));
                s.position = instanced_attribute(0).rgb();
                s.rotation = (scalar(0) + instance_index().to_float()).sin();
                s.scale = scalar(1);
                s.size_attenuation = scalar(3);
                out.params[0] = 1.0;
                s
            }
            51 => {
                geometry.instance_count = Some(20000);
                scene.background = Color::from_hex(0x201919);
                // r187's range() nodes, hashed per instance with the InstancedMesh's id
                // ( Object3D 9 on the page ): the scale, radiusRatio, the branch and the
                // offset are range nodes 0 to 3.
                let ranged = |id, low: f32, high: f32| tsl::range(9, id, [low; 4], [high; 4]);
                let ratio = ranged(1, 0.0, 1.0).x();
                let radius = ratio.pow(float(1.5)) * float(5.0);
                let angle = ranged(2, 0.0, 3.0).x().floor() * float(std::f32::consts::TAU / 3.0)
                    + scalar(0) * (float(1.0) - ratio.clone());
                let offset = tsl::range(9, 3, [-1.0, -1.0, -1.0, 0.0], [1.0, 1.0, 1.0, 0.0]).rgb();
                let offset = offset.clone() * offset.clone() * offset * ratio.clone() + float(0.2);
                let position = vec3(angle.cos(), float(0.0), angle.sin()) * radius + offset;
                let amount = float(1.0) - (float(1.0) - ratio).pow(float(2.0));
                let alpha = float(0.1) / (uv() - float(0.5)).length() - float(0.2);
                let mut s = SpriteNodeMaterial::new(vec4(
                    mix(uniform(2, Type::Vec3), uniform(3, Type::Vec3), amount),
                    alpha,
                ));
                s.position = position;
                s.scale = ranged(0, 0.0, 1.0).x() * scalar(1);
                out.params = [0.08, 0xffa575 as f32, 0x311599 as f32, 0.0];
                s
            }
            _ => {
                geometry.instance_count = Some(50000);
                let mut positions = Vec::with_capacity(50000);
                let mut colors = Vec::with_capacity(50000);
                for i in 0..50000 {
                    let angle = rand(&mut seed) * std::f64::consts::TAU;
                    let u = rand(&mut seed) * 2.0 - 1.0;
                    let radius = (1.0 - u * u).sqrt() * 600.0;
                    positions.push([
                        (angle.cos() * radius) as f32,
                        (angle.sin() * radius) as f32,
                        (u * 600.0) as f32,
                        i as f32 / 50000.0,
                    ]);
                    let c = Color::from_hsl(i as f64 / 50000.0, 0.7, 0.7).0;
                    let c = Color::from_srgb(c.x, c.y, c.z);
                    colors.push(c.0.extend(1.0).as_vec4().to_array());
                }
                buffers.push(attribute(r, &positions)?);
                buffers.push(attribute(r, &colors)?);
                let data = instanced_attribute(0);
                let t = (data.swizzle("w") + scalar(0) * float(0.1)).modulo(float(1.0));
                let acc = t.clone() * t.clone();
                let angle = acc.clone() * float(40.0);
                let position = vec3(
                    data.x() * acc.clone() + angle.sin() * float(20.0),
                    data.y() * acc.clone() + angle.cos() * float(20.0),
                    data.swizzle("z") * acc * float(1.75),
                );
                let mut s = SpriteNodeMaterial::new(
                    tex * vec4(instanced_attribute(1).rgb(), (float(1.0) - t) * float(2.0)),
                );
                s.position = position;
                s.scale = float(2.0);
                out.history = Some(AfterImagePass::new(r).await?);
                out.params = [0.8, 1.0, 0.0, 0.0];
                s
            }
        };
        // Keep this a node expression even for non-instanced sprites.
        if example == 49 {
            sprite.scale = float(1.0);
        }
        let refs = buffers.iter().collect::<Vec<_>>();
        let mut material = sprite.build(r, &refs, &textures).await?;
        if example >= 51 {
            additive(&mut material);
        }
        if example == 50 {
            material.properties.alpha_test = 0.1;
        }
        let geometry = Arc::new(geometry);
        let material = Arc::new(Material::Shader(material));
        for _ in 0..if example == 49 { 200 } else { 1 } {
            let h = add_mesh(scene, geometry.clone(), material.clone());
            scene.get_mut(h)?.frustum_culled = false;
            if example == 49 {
                scene.get_mut(h)?.position = Vector3::new(
                    rand(&mut seed) - 0.5,
                    rand(&mut seed) - 0.5,
                    rand(&mut seed) - 0.5,
                )
                .normalize()
                    * 500.0;
                scene.add(group, h)?;
            }
            out.objects.push(h);
        }
        Ok(out)
    }
    pub fn seek(&mut self, time: f64) {
        self.time = time;
        self.frames = time * 60.0;
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let ranges: &[(f32, f32)] = match self.example {
            48 => &[(0.001, 0.1), (-5.0, 5.0)],
            50 => &[(0.0, 1.0)],
            51 => &[(0.0, 1.0), (0.0, 16777215.0), (0.0, 16777215.0)],
            52 => &[(0.25, 1.0), (0.0, 1.0)],
            _ => &[],
        };
        if !value.is_finite()
            || ranges
                .get(index)
                .is_none_or(|(a, b)| value < *a || value > *b)
        {
            return Err(Error::Invalid("TSL particle parameter"));
        }
        self.params[index] = value;
        Ok(())
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new(x, y);
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
        if ![48, 51].contains(&self.example) {
            return Ok(());
        }
        if pan {
            self.pan += OrbitViewer::pan_delta(scene, cam, self.viewer.radius(), dx, dy, height)?;
        } else {
            self.orbit += Vector2::new(dx, dy);
            let (min, max) = if self.example == 48 {
                (7.0, 100.0)
            } else {
                (0.1, 50.0)
            };
            self.viewer.orbit_pixels(0.0, 0.0, wheel, height, min, max);
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
            self.time += delta;
            self.frames += 1.0;
        }
        let window = web_sys::window().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap_or(512.0);
        if [48, 51].contains(&self.example) {
            let (min, max, near, far) = if self.example == 48 {
                (7.0, 100.0, 1.0, 600.0)
            } else {
                (0.1, 50.0, 0.1, 100.0)
            };
            self.viewer.orbit_pixels(
                self.orbit.x * 0.05,
                self.orbit.y * 0.05,
                0.0,
                height,
                min,
                max,
            );
            self.orbit *= 0.95;
            self.viewer.pan_world(self.pan * 0.05);
            self.pan *= 0.95;
            if self.example == 48 {
                self.viewer
                    .limit_pitch(0.0, std::f64::consts::FRAC_PI_2 - 1e-6);
            }
            self.viewer.update(scene, cam)?;
            if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(cam)?.kind {
                c.near = near;
                c.far = far;
            }
        }
        if self.example == 50 {
            let p = &mut scene.get_mut(cam)?.position;
            p.x += (self.pointer.x - p.x) * 0.05;
            p.y += (-self.pointer.y - p.y) * 0.05;
            scene.look_at(cam, Vector3::ZERO)?;
        }
        if self.example == 49 {
            scene.get_mut(self.group)?.quaternion = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                self.time * 0.5,
                self.time * 0.75,
                self.time,
            );
        }
        if self.example == 52 {
            scene.get_mut(self.objects[0])?.quaternion = Quaternion::from_rotation_z(self.time);
            self.enabled = self.params[1] > 0.5;
            self.history.as_mut().unwrap().damp = self.params[0];
        }
        for (i, &h) in self.objects.iter().enumerate() {
            let n = scene.get_mut(h)?;
            if self.example == 49 {
                let scale = (self.time + n.position.x * 0.01).sin() * 0.3 + 1.0;
                n.scale = Vector3::new(scale * self.size.x, scale * self.size.y, 1.0);
            }
            if let NodeKind::Mesh(mesh) = &mut n.kind {
                let m = Arc::make_mut(&mut mesh.materials[0]);
                if self.example == 48 {
                    m.properties_mut().vertex_uniforms[0][0] = self.params[0];
                    m.properties_mut().vertex_uniforms[1][0] = self.params[1];
                }
                if let Material::Shader(m) = m {
                    m.uniforms[0][0] = self.time as f32;
                    match self.example {
                        49 => m.uniforms[1][0] = (self.frames * 0.1 * i as f64 / 200.0) as f32,
                        50 => {
                            m.uniforms[1][0] = if self.params[0] > 0.5 { 15.0 } else { 0.03 };
                            m.uniforms[3][0] = self.params[0];
                            m.uniforms[2] = Color::from_hsl((self.time * 0.05) % 1.0, 0.5, 0.5)
                                .0
                                .extend(1.0)
                                .as_vec4()
                                .to_array();
                        }
                        51 => {
                            m.uniforms[1][0] = self.params[0];
                            m.uniforms[2] = Color::from_hex(self.params[1] as u32)
                                .0
                                .extend(1.0)
                                .as_vec4()
                                .to_array();
                            m.uniforms[3] = Color::from_hex(self.params[2] as u32)
                                .0
                                .extend(1.0)
                                .as_vec4()
                                .to_array();
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        scene: &mut Scene,
        cam: Object3D,
        target: &RenderTarget,
    ) -> Result<bool> {
        if self.example != 52 {
            return Ok(false);
        }
        r.render(scene, cam, target)?;
        if self.enabled {
            self.history.as_mut().unwrap().render(r, target)?;
        }
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        if self.enabled {
            self.history.as_ref().map(|h| h.output())
        } else {
            None
        }
    }
    /// The target `output` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_mut(&mut self) -> Option<&mut RenderTarget> {
        // Its history is read again on the next frame.
        None
    }
}

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_pointer(&mut self, x: f64, y: f64) {
        self.pointer(x, y);
    }
    fn on_render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Option<crate::Result<bool>> {
        Some(self.render(renderer, scene, camera, target))
    }
    fn on_output_target(&self) -> Option<Option<&crate::renderer::RenderTarget>> {
        Some(self.output())
    }
    fn on_output_target_mut(&mut self) -> Option<Option<&mut crate::renderer::RenderTarget>> {
        Some(self.output_mut())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/webgpu_fog_height.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_fog_height(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.0),
                0.0,
                2.0,
                Box::new(Demo::create(scene, camera, 48, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_sprites.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_sprites(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.0),
                0.0,
                2.0,
                Box::new(Demo::create(scene, camera, 49, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_instance_sprites.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_instance_sprites(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            premultiplied_canvas: true,
            samples: 1,
            raw_output: true,
            premultiplied_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.0),
                0.0,
                2.0,
                Box::new(Demo::create(scene, camera, 50, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_tsl_galaxy.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_tsl_galaxy(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.0),
                0.0,
                2.0,
                Box::new(Demo::create(scene, camera, 51, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_postprocessing_afterimage.html: its renderer, its scene
/// and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_postprocessing_afterimage(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 1,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.0),
                0.0,
                2.0,
                Box::new(Demo::create(scene, camera, 52, renderer).await?),
            ))
        },
    )
    .await
}
