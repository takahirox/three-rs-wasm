//! Resident point/instance attributes for the pinned WebGL buffer examples.
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::tsl::Node;
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
    tsl::{self, sprites::SpriteNodeMaterial, surface::SurfaceNodes, *},
};
use std::ops::Mul;
use std::sync::Arc;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn function(name: &str, source: &str, args: &[Type], output: Type, nodes: &[Node]) -> Result<Node> {
    Ok(WgslFn::new(name, source, args, output)?.call(nodes))
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    object: Object3D,
    visibility: Option<GpuBuffer>,
    visible: Vec<f32>,
    dirty: bool,
    seed: u32,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let (fov, near, far, z) = match id {
            158 | 159 => (27., 5., 3500., 2750.),
            160 => (40., 1., 10000., 300.),
            161 => (50., 1., 5000., 1400.),
            _ => (45., 0.01, 10., 3.5),
        };
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., z);
        s.background = Color::BLACK;
        let mut seed = 186;
        let mut visibility = None;
        let mut visible = vec![];
        let object = if id == 162 {
            let mut positions = vec![];
            let mut colors = vec![];
            let mut uvs = vec![];
            for i in 0..100 {
                for j in 0..200 {
                    let lat = random(&mut seed) * std::f64::consts::PI / 50.
                        + i as f64 / 100. * std::f64::consts::PI;
                    let lng = random(&mut seed) * std::f64::consts::PI / 50.
                        + j as f64 / 200. * std::f64::consts::TAU;
                    positions.extend([
                        0.,
                        0.,
                        0.,
                        (lat.sin() * lng.cos()) as f32,
                        lat.cos() as f32,
                        (lat.sin() * lng.sin()) as f32,
                    ]);
                    for lightness in [0.2, 0.7] {
                        let rgb = hsl(lat / std::f64::consts::PI, lightness);
                        colors.extend(rgb.map(|x| x as f32));
                        uvs.extend([(i * 200 + j) as f32, 0.]);
                    }
                }
            }
            let mut g = BufferGeometry::default();
            for (name, data, size) in [
                ("position", positions, 3),
                ("color", colors, 3),
                ("uv", uvs, 2),
            ] {
                g.set_attribute(
                    name,
                    Attribute::F32(BufferAttribute::new(data, size, false)?),
                );
            }
            visible = vec![1.; 20000];
            let buffer = GpuBuffer::new(r, bytemuck::cast_slice(&visible), BufferAccess::Read)?;
            let mask = storage_element(0, uv().x().to_uint()).greater_than(float(0.));
            let mut m = LineBasicMaterial::default();
            m.properties.vertex_colors = true;
            m.properties.vertex_program = Some(Arc::new(
                SurfaceNodes {
                    mask: Some(mask),
                    ..Default::default()
                }
                .build(r, &[(&buffer, Type::Float)], &[])
                .await?,
            ));
            visibility = Some(buffer);
            s.insert(NodeKind::Line(Line {
                geometry: Arc::new(g),
                material: Arc::new(Material::Line(m)),
                segments: true,
            }))
        } else {
            let count = match id {
                158 | 159 => 500000,
                160 => 100000,
                _ => 75000,
            };
            // Keep packed source records resident; the shared quad/circle has no per-particle copies.
            let stride = match id {
                158 => 6,
                159 => 4,
                _ => 3,
            };
            let mut words = Vec::<u32>::with_capacity(count * stride);
            for _ in 0..count {
                let radius = match id {
                    158 | 159 => 500.,
                    160 => 200.,
                    _ => 1.,
                };
                let p = [
                    random(&mut seed) * 2. * radius - radius,
                    random(&mut seed) * 2. * radius - radius,
                    random(&mut seed) * 2. * radius - radius,
                ];
                words.extend(p.map(|v| (v as f32).to_bits()));
                if id == 158 || id == 159 {
                    let color = p.map(|v| {
                        let x = v / 1000. + 0.5;
                        if x <= 0.04045 {
                            x / 12.92
                        } else {
                            ((x + 0.055) / 1.055).powf(2.4)
                        }
                    });
                    if id == 158 {
                        words.extend(color.map(|x| (x as f32).to_bits()));
                    } else {
                        words.push(
                            (color[0] * 255.) as u32
                                | ((color[1] * 255.) as u32) << 8
                                | ((color[2] * 255.) as u32) << 16,
                        );
                    }
                }
            }
            let buffer = GpuBuffer::new(r, bytemuck::cast_slice(&words), BufferAccess::Read)?;
            let pos = function(
                "particle_position",
                &format!(
                    "fn particle_position(i:u32)->vec3<f32>{{let b=i*{stride}u;return vec3(bitcast<f32>(tsl_attribute_0[b]),bitcast<f32>(tsl_attribute_0[b+1u]),bitcast<f32>(tsl_attribute_0[b+2u]));}}"
                ),
                &[Type::Uint],
                Type::Vec3,
                &[instance_index()],
            )?;
            let picture = if id >= 160 {
                let name = if id == 160 {
                    "spark1.png"
                } else {
                    "circle.png"
                };
                let mut image =
                    decode_texture_image(&fetch(&format!("/web/gallery/assets/{name}")).await?)
                        .await?;
                image.srgb = false;
                image.mipmap_filter = Some(Filter::Linear);
                Some(r.upload_texture(&Arc::new(image))?)
            } else {
                None
            };
            let textures: Vec<_> = picture.iter().map(|p| (&p.view, &p.sampler)).collect();
            let hsl_source = "fn particle_hue(h:f32)->vec3<f32>{let x=fract(h)*6.0;return clamp(vec3(abs(x-3.0)-1.0,2.0-abs(x-2.0),2.0-abs(x-4.0)),vec3(0.0),vec3(1.0));}";
            let wave = (pos.clone().x() + uniform(0, Type::Float))
                .mul(float(2.1))
                .sin()
                + (pos.clone().y() + uniform(0, Type::Float))
                    .mul(float(3.2))
                    .sin()
                + (pos.clone().swizzle("z") + uniform(0, Type::Float))
                    .mul(float(4.3))
                    .sin();
            let color = match id {
                158 => function(
                    "particle_color",
                    "fn particle_color(i:u32)->vec3<f32>{let b=i*6u+3u;return vec3(bitcast<f32>(tsl_attribute_0[b]),bitcast<f32>(tsl_attribute_0[b+1u]),bitcast<f32>(tsl_attribute_0[b+2u]));}",
                    &[Type::Uint],
                    Type::Vec3,
                    &[instance_index()],
                )?,
                159 => function(
                    "particle_color",
                    "fn particle_color(i:u32)->vec3<f32>{return unpack4x8unorm(tsl_attribute_0[i*4u+3u]).rgb;}",
                    &[Type::Uint],
                    Type::Vec3,
                    &[instance_index()],
                )?,
                160 => function(
                    "particle_hue",
                    hsl_source,
                    &[Type::Float],
                    Type::Vec3,
                    &[instance_index().to_float() / float(100000.)],
                )?,
                _ => function(
                    "particle_hue",
                    hsl_source,
                    &[Type::Float],
                    Type::Vec3,
                    &[wave.clone() / float(5.)],
                )?,
            };
            // WebGL PointsMaterial applies fog after output encoding; raw shaders do neither.
            let color = if id <= 159 {
                let fog = function(
                    "particle_fog",
                    "fn particle_fog(p:vec3<f32>)->f32{return smoothstep(2000.0,3500.0,-(u.view*u.model*vec4(p,1.0)).z);}",
                    &[Type::Vec3],
                    Type::Float,
                    std::slice::from_ref(&pos),
                )?;
                s.background = Color(Vector3::splat(5. / 255.));
                vec4(
                    mix(
                        function(
                            "particle_srgb",
                            "fn particle_srgb(c:vec3<f32>)->vec3<f32>{return srgb_output(c);}",
                            &[Type::Vec3],
                            Type::Vec3,
                            &[color],
                        )?,
                        splat(float(5. / 255.), Type::Vec3),
                        fog,
                    ),
                    float(1.),
                )
            } else {
                let tex = tsl::Texture::External(0).sample(if id == 160 {
                    uv()
                } else {
                    vec2(uv().x(), float(1.) - uv().y())
                });
                let out = vec4(color * tex.rgb(), tex.swizzle("w"));
                if id == 161 {
                    alpha_test(out, float(0.499999))
                } else {
                    out
                }
            };
            let mut sprite = SpriteNodeMaterial::new(color);
            sprite.position = pos.clone();
            if id == 161 {
                sprite.scale = (wave * float(10.) + float(10.)) / float(500.);
            } else {
                // The original CPU size loop is evaluated per instance in the vertex shader.
                let size = if id == 160 {
                    float(10.)
                        * (float(1.)
                            + (instance_index().to_float() * float(0.1) + uniform(0, Type::Float))
                                .sin())
                } else {
                    float(15.)
                };
                sprite.scale = function(
                    "particle_pixels",
                    if id == 160 {
                        "fn particle_pixels(p:vec3<f32>,size:f32)->f32{return clamp(size*300.0/(-(u.view*u.model*vec4(p,1.0)).z),1.0,511.0);}"
                    } else {
                        "fn particle_pixels(p:vec3<f32>,size:f32)->f32{return clamp(size*u.point.y*0.5/(-(u.view*u.model*vec4(p,1.0)).z),1.0,511.0);}"
                    },
                    &[Type::Vec3, Type::Float],
                    Type::Float,
                    &[pos, size],
                )?;
            }
            let mut m = if id == 161 {
                sprite
                    .build_with_storage(r, &[(&buffer, Type::Uint)], &textures)
                    .await?
            } else {
                sprite
                    .build_points(r, &[(&buffer, Type::Uint)], &textures)
                    .await?
            };
            m.properties.transparent = id == 160;
            if id == 160 {
                m.properties.depth_test = false;
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
            let mut g = if id == 161 {
                CircleGeometry::build(1., 6, 0., std::f64::consts::TAU)?
            } else {
                PlaneGeometry::build(1., 1., 1, 1)?
            };
            g.instance_count = Some(count as u32);
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(g),
                Arc::new(Material::Shader(m)),
            )));
            s.get_mut(h)?.frustum_culled = false;
            if id == 161 {
                s.get_mut(h)?.scale = Vector3::splat(500.);
            }
            h
        };
        Ok(Self {
            id,
            time: 0.,
            object,
            visibility,
            visible,
            dirty: false,
            seed,
        })
    }
    pub fn update(&mut self, s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        let t = self.time;
        let n = s.get_mut(self.object)?;
        n.quaternion = if self.id == 160 {
            Quaternion::from_rotation_z(t * 0.05)
        } else if self.id == 161 {
            Quaternion::from_euler(glam::EulerRot::XYZ, t * 0.1, t * 0.2, 0.)
        } else {
            Quaternion::from_euler(glam::EulerRot::XYZ, t * 0.25, t * 0.5, 0.)
        };
        if let NodeKind::Mesh(m) = &mut n.kind
            && let Material::Shader(material) = Arc::make_mut(&mut m.materials[0])
        {
            material.uniforms[0][0] = (t * if self.id == 160 { 5. } else { 0.5 }) as f32;
        }
        Ok(())
    }
    pub fn prepare(&mut self, r: &Renderer) -> Result<()> {
        if self.dirty {
            if let Some(b) = &self.visibility {
                b.write(r, 0, bytemuck::cast_slice(&self.visible))?;
            }
            self.dirty = false;
        }
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, _v: f32) -> Result<()> {
        if self.id != 162 || index > 1 {
            return Err(Error::Invalid("particle parameter"));
        }
        for value in &mut self.visible {
            if index == 1 {
                *value = 1.;
            } else if random(&mut self.seed) > 0.75 {
                *value = 0.;
            }
        }
        self.dirty = true;
        Ok(())
    }
    pub fn status(&self) -> String {
        if self.id == 162 {
            format!(
                "1 draw call, 20,000 lines, {} culled",
                self.visible.iter().filter(|&&x| x == 0.).count()
            )
        } else {
            String::new()
        }
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
fn hsl(h: f64, l: f64) -> [f64; 3] {
    let h = h.rem_euclid(1.);
    let c = 1. - (2. * l - 1.).abs();
    [0., 8., 4.].map(|offset| {
        let k = (h * 12. + offset) % 12.;
        l - c / 2. * (-1f64).max((k - 3.).min(9. - k).min(1.))
    })
}
