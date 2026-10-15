//! Three pinned examples built with the shared Rust TSL graph compiler.
use super::gltf_viewer::{decode_image, fetch};
use crate::tsl::Node;
use crate::{
    Error, Result,
    camera::*,
    geometry::PlaneGeometry,
    material::{Filter, Material, ShaderMaterial, Wrapping},
    math::*,
    postprocessing::Effect,
    renderer::{RenderTarget, RenderTargetOptions, Renderer},
    scene::*,
    tsl::{self, *},
};
use std::sync::Arc;

pub(super) struct Demo {
    example: u32,
    objects: Vec<Object3D>,
    time: f64,
    parameters: [[f32; 4]; 16],
    procedural: Option<Procedural>,
}
struct Procedural {
    dummy: RenderTarget,
    generated: RenderTarget,
    horizontal: RenderTarget,
    output: RenderTarget,
    generate: Effect,
    blur_x: Effect,
    blur_y: Effect,
    auto_update: bool,
    initialized: bool,
}
fn scalar(index: usize) -> Node {
    uniform(index, Type::Float)
}
fn gradient() -> Node {
    function(|| {
        let p = uv();
        let blur = ((float(0.0625) - (p.x() * float(20.0) + scalar(0)).cos()) * float(0.0625))
            .pow(float(2.0));
        let grad = p.y().greater_than(float(0.5)).select(
            splat(float(0.0), Type::Vec2),
            splat(blur.clone(), Type::Vec2),
        );
        let mut color = splat(float(0.0), Type::Vec4);
        for (x, y) in [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
            let offset = vec2(blur.clone() * float(x), blur.clone() * float(y)) * float(0.5);
            color = color
                + tsl::Texture::Map.sample_grad(p.clone() + offset, grad.clone(), grad.clone())
                    * float(0.25);
        }
        p.y()
            .greater_than(float(0.497))
            .and(p.y().less_than(float(0.503)))
            .select(splat(float(1.0), Type::Vec4), color)
    })
}
fn crt() -> Node {
    function(|| {
        let dimensions = vec2(float(1608.0), float(1608.0));
        let pixel = (uv() * float(0.5) + float(0.5)) * dimensions.clone();
        let coord = pixel.clone() / scalar(1);
        let sub = coord.clone() * vec2(float(3.0), float(1.0));
        let offset = vec2(float(0.0), (coord.x().floor() * scalar(2)).fract());
        let point = (coord + offset.clone()).floor() * scalar(1) / dimensions;
        let point = vec2(
            point.x() + (scalar(0) * scalar(7) / float(20.0)).fract(),
            point.y() - float(1.5),
        );
        let index = sub.x().floor().modulo(float(3.0));
        let mask = vec3(
            index.equal(float(0.0)).to_float(),
            index.equal(float(1.0)).to_float(),
            index.equal(float(2.0)).to_float(),
        ) * float(3.0);
        let cell = (sub + offset).fract() * float(2.0) - float(1.0);
        let border = float(1.0) - cell.clone() * cell * scalar(3);
        let mask = mask
            * border.x().clamp(float(0.0), float(1.0))
            * border.y().clamp(float(0.0), float(1.0));
        let pulse = (pixel.y() / scalar(5) + scalar(0) * float(20.0)).sin() * scalar(4);
        tsl::Texture::Map.sample(point).rgb() * mask * (float(1.0) + pulse)
    })
}
async fn image_map(url: &str, repeat: bool) -> Result<Arc<crate::material::Texture>> {
    let mut image = decode_image(&fetch(url).await?).await?;
    image.srgb = false;
    image.mipmap_filter = Some(Filter::Linear);
    // WebGPU TextureLoader flips images on upload. Core's usual map path flips
    // coordinates; raw WGSL texture/sampler arguments instead require that same
    // storage orientation. One-time image preparation, never per-frame work.
    let row = image.width as usize * 4;
    for y in 0..image.height as usize / 2 {
        let other = (image.height as usize - 1 - y) * row;
        for x in 0..row {
            image.rgba.swap(y * row + x, other + x);
        }
    }
    image.flip_y = false;
    if repeat {
        image.wrap_s = Wrapping::Repeat;
        image.wrap_t = Wrapping::Repeat;
    }
    Ok(Arc::new(image))
}
impl Demo {
    pub async fn create(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &Renderer,
    ) -> Result<Self> {
        scene.background = if example == 36 {
            Color::from_hex(0x313131)
        } else {
            Color::BLACK
        };
        let aspect = match scene.camera(camera)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: if example == 35 { -1.0 } else { -aspect },
            right: if example == 35 { 1.0 } else { aspect },
            top: 1.0,
            bottom: -1.0,
            near: if example == 36 { 0.1 } else { 0.0 },
            far: if example == 35 {
                1.0
            } else if example == 36 {
                2000.0
            } else {
                2.0
            },
            ..Default::default()
        }));
        scene.get_mut(camera)?.position = Vector3::new(
            0.0,
            0.0,
            if example == 35 {
                0.0
            } else if example == 36 {
                2.0
            } else {
                1.0
            },
        );
        let mut result = Self {
            example,
            objects: Vec::new(),
            time: 0.0,
            parameters: [[0.0; 4]; 16],
            procedural: None,
        };
        let mut materials = Vec::new();
        match example {
            35 => {
                let function = WgslFn::new(
                    "crtFragment",
                    include_str!("tsl_crt.wgsl"),
                    &[
                        Type::Vec2,
                        Type::Texture,
                        Type::Sampler,
                        Type::Float,
                        Type::Float,
                        Type::Float,
                        Type::Float,
                        Type::Float,
                        Type::Float,
                        Type::Float,
                        Type::Float,
                        Type::Float,
                        Type::Float,
                    ],
                    Type::Vec3,
                )?;
                let color = function.call(&[
                    uv(),
                    tsl::Texture::Map.node(),
                    tsl::Texture::Map.sampler(),
                    float(1608.0),
                    float(1608.0),
                    scalar(2),
                    scalar(1),
                    scalar(3),
                    scalar(0),
                    scalar(6),
                    scalar(4),
                    scalar(5),
                    float(20.0),
                ]);
                let map = image_map("/web/gallery/assets/earth-lights.png", true).await?;
                for color in [color, crt()] {
                    let mut material = NodeMaterial::new(color).build(renderer, &[]).await?;
                    material.properties.map = Some(map.clone());
                    materials.push(material);
                }
                for (i, v) in [0.0, 6.0, 0.5, 1.0, 0.06, 60.0, 1.0, 1.0]
                    .into_iter()
                    .enumerate()
                {
                    result.parameters[i][0] = v;
                }
            }
            36 => {
                let mut material = NodeMaterial::new(gradient()).build(renderer, &[]).await?;
                material.properties.map =
                    Some(image_map("/web/gallery/assets/uv-grid.jpg", false).await?);
                materials.push(material);
            }
            37 => {
                let options = RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    depth_buffer: false,
                    ..Default::default()
                };
                let target = |size| {
                    RenderTarget::with_options(&renderer.device, size, size, options.clone())
                };
                let generated = target(512)?;
                let horizontal = target(512)?;
                let output = target(512)?;
                // Three's fullscreen QuadMesh UV starts at y=1 at the top;
                // Effect's UV is top-left based.
                let quad_uv = vec2(uv().x(), float(1.0) - uv().y());
                let generate =
                    effect(renderer, options.format, &checker(quad_uv * scalar(1))).await?;
                let mut blurs = Vec::new();
                for direction in [(1.0, 0.0), (0.0, 1.0)] {
                    let step =
                        vec2(float(direction.0), float(direction.1)) * scalar(2) / float(512.0);
                    blurs.push(
                        effect(
                            renderer,
                            options.format,
                            &gaussian_blur(tsl::Texture::Input, uv(), step, 20)?,
                        )
                        .await?,
                    );
                }
                let sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor {
                    min_filter: wgpu::FilterMode::Linear,
                    mag_filter: wgpu::FilterMode::Linear,
                    ..Default::default()
                });
                // Render-target sampling follows the same top-left UV convention as Effect.
                let color = tsl::Texture::External(0).sample(vec2(uv().x(), float(1.0) - uv().y()));
                materials.push(
                    NodeMaterial::new(color)
                        .build(renderer, &[(&output.view, &sampler)])
                        .await?,
                );
                result.procedural = Some(Procedural {
                    dummy: target(1)?,
                    generated,
                    horizontal,
                    output,
                    generate,
                    blur_x: blurs.remove(0),
                    blur_y: blurs.remove(0),
                    auto_update: true,
                    initialized: false,
                });
                result.parameters[1][0] = 4.0;
                result.parameters[2][0] = 0.5;
            }
            _ => return Err(Error::Invalid("TSL example")),
        }
        let geometry = Arc::new(PlaneGeometry::build(
            if example == 35 { 2.0 } else { 1.0 },
            1.0,
            1,
            1,
        )?);
        for (i, mut material) in materials.into_iter().enumerate() {
            material.properties.fog = false;
            let object = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Shader(material)),
            )));
            if example == 35 {
                scene.get_mut(object)?.position.y = 0.5 - i as f64;
            }
            result.objects.push(object);
        }
        Ok(result)
    }
    pub fn seek(&mut self, seconds: f64) {
        self.time = seconds;
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let valid = if self.example == 35 {
            match index {
                1 => (6.0..=50.0).contains(&value),
                2 => (0.0..=1.0).contains(&value),
                3 => (0.0..=5.0).contains(&value),
                4 => (0.0..=0.5).contains(&value),
                5 => (10.0..=100.0).contains(&value),
                6 | 7 => (1.0..=10.0).contains(&value),
                _ => false,
            }
        } else if self.example == 37 {
            match index {
                1 => (1.0..=10.0).contains(&value),
                2 => (0.0..=2.0).contains(&value),
                3 => value == 0.0 || value == 1.0,
                _ => false,
            }
        } else {
            false
        };
        if !valid {
            return Err(Error::Invalid("TSL example parameter"));
        }
        if self.example == 37 && index == 3 {
            self.procedural.as_mut().unwrap().auto_update = value != 0.0;
        } else {
            self.parameters[index][0] = value;
        }
        Ok(())
    }
    pub fn update(&mut self, scene: &mut Scene, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
        }
        if self.example != 37 {
            self.parameters[0][0] = self.time as f32;
        }
        for &object in &self.objects {
            if let NodeKind::Mesh(mesh) = &mut scene.get_mut(object)?.kind
                && let Material::Shader(ShaderMaterial { uniforms, .. }) =
                    Arc::make_mut(&mut mesh.materials[0])
            {
                *uniforms = self.parameters;
            }
        }
        Ok(())
    }
    pub fn prepare(
        &mut self,
        renderer: &Renderer,
        scene: &mut Scene,
        camera: Object3D,
        aspect: f64,
    ) -> Result<()> {
        if self.example != 35
            && let NodeKind::Camera(Camera::Orthographic(c)) = &mut scene.get_mut(camera)?.kind
        {
            c.left = -aspect;
            c.right = aspect;
        }
        if let Some(p) = &mut self.procedural {
            p.generate.parameters = self.parameters;
            p.blur_x.parameters = self.parameters;
            p.blur_y.parameters = self.parameters;
            if p.auto_update || !p.initialized {
                p.generate.apply(renderer, &p.dummy, None, &p.generated)?;
                p.initialized = true;
            }
            p.blur_x
                .apply(renderer, &p.generated, None, &p.horizontal)?;
            p.blur_y.apply(renderer, &p.horizontal, None, &p.output)?;
        }
        Ok(())
    }
}
