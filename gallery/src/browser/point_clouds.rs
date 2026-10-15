//! Point clouds retain compact attributes; wave deformation and animated sizes run on the GPU.
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::tsl::Node;
use crate::{
    Error, Result,
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, sprites::SpriteNodeMaterial, *},
};
use std::sync::Arc;
const ASSETS: &str = "/web/gallery/assets/point-clouds";
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn function(name: &str, code: &str, args: &[Type], output: Type, nodes: &[Node]) -> Result<Node> {
    Ok(WgslFn::new(name, code, args, output)?.call(nodes))
}
fn scalar(i: usize) -> Node {
    uniform(i, Type::Float)
}
fn hue(h: f64, s: f64, l: f64) -> [f64; 3] {
    let c = (1. - (2. * l - 1.).abs()) * s;
    [0., 8., 4.].map(|o| {
        let k = (h.rem_euclid(1.) * 12. + o) % 12.;
        l - c / 2. * (-1f64).max((k - 3.).min(9. - k).min(1.))
    })
}
fn hsl(h: Node, s: Node, l: Node) -> Result<Node> {
    function(
        "cloud_hsl",
        "fn cloud_hsl(h:f32,s:f32,l:f32)->vec3<f32>{let h6=fract(h)*6.0;let rgb=clamp(vec3(abs(h6-3.0)-1.0,2.0-abs(h6-2.0),2.0-abs(h6-4.0)),vec3(0.0),vec3(1.0));return (rgb-0.5)*((1.0-abs(2.0*l-1.0))*s)+l;}",
        &[Type::Float, Type::Float, Type::Float],
        Type::Vec3,
        &[h, s, l],
    )
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    objects: Vec<Object3D>,
    rotations: Vec<[f64; 3]>,
    pointer: Vector2,
    enabled: bool,
    inner: u32,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let (fov, near, far, z) = match id {
            163 => (55., 2., 2000., 1000.),
            164 => (75., 1., 2000., 1000.),
            165 => (75., 1., 10000., 1000.),
            166 => (40., 1., 10000., 300.),
            _ => (40., 1., 1000., 500.),
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
        let mut inner = 0;
        let (count, buffer) = if id == 165 {
            (2500, None)
        } else if id == 167 {
            let bytes = fetch(&format!("{ASSETS}/frame.bin")).await?;
            if bytes.len() < 8 {
                return Err(Error::Asset("point frame header".into()));
            }
            let count = u32::from_le_bytes(bytes[..4].try_into().unwrap());
            inner = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
            if bytes.len() != 8 + count as usize * 12 || inner > count {
                return Err(Error::Asset("point frame size".into()));
            }
            (
                count,
                Some(GpuBuffer::new(r, &bytes[8..], BufferAccess::Read)?),
            )
        } else {
            let count = if id == 166 { 100000 } else { 10000 };
            let radius = if id == 166 { 200. } else { 1000. };
            let positions = (0..count * 3)
                .map(|_| ((random(&mut seed) * 2. - 1.) * radius) as f32)
                .collect::<Vec<_>>();
            (
                count,
                Some(GpuBuffer::new(
                    r,
                    bytemuck::cast_slice(&positions),
                    BufferAccess::Read,
                )?),
            )
        };
        let buffers: Vec<_> = buffer.iter().map(|b| (b, Type::Float)).collect();
        let pos = if id == 165 {
            function(
                "cloud_position",
                "fn cloud_position(i:u32,t:f32)->vec3<f32>{let x=f32(i/50u);let z=f32(i%50u);return vec3(x*100.0-2500.0,sin((x+t)*0.3)*50.0+sin((z+t)*0.5)*50.0,z*100.0-2500.0);}",
                &[Type::Uint, Type::Float],
                Type::Vec3,
                &[instance_index(), scalar(0)],
            )?
        } else {
            function(
                "cloud_position",
                "fn cloud_position(i:u32)->vec3<f32>{let b=i*3u;return vec3(tsl_attribute_0[b],tsl_attribute_0[b+1u],tsl_attribute_0[b+2u]);}",
                &[Type::Uint],
                Type::Vec3,
                &[instance_index()],
            )?
        };
        let mut objects = vec![];
        let mut rotations = vec![];
        for group in 0..if id == 164 { 5 } else { 1 } {
            let picture = if id == 165 {
                None
            } else {
                let path = match id {
                    163 => format!("{ASSETS}/disc.png"),
                    164 => format!("{ASSETS}/snowflake{}.png", [2, 3, 1, 5, 4][group]),
                    166 => "/web/gallery/assets/spark1.png".into(),
                    _ => format!("{ASSETS}/ball.png"),
                };
                let mut t = decode_texture_image(&fetch(&path).await?).await?;
                t.srgb = id <= 164;
                t.mipmap_filter = Some(Filter::Linear);
                if id == 167 {
                    t.wrap_s = Wrapping::Repeat;
                    t.wrap_t = Wrapping::Repeat;
                }
                Some(r.upload_texture(&Arc::new(t))?)
            };
            let textures: Vec<_> = picture.iter().map(|p| (&p.view, &p.sampler)).collect();
            let tex = if id == 165 {
                vec4(splat(float(1.), Type::Vec3), float(1.))
            } else {
                tsl::Texture::External(0).sample(if id <= 164 {
                    vec2(uv().x(), float(1.) - uv().y())
                } else {
                    uv()
                })
            };
            let color = match id {
                163 => hsl(scalar(0) * float(0.05), float(0.5), float(0.5))?,
                164 => uniform(3, Type::Vec3),
                165 => splat(float(1.), Type::Vec3),
                166 => {
                    let negative = pos.clone().x().less_than(float(0.));
                    hsl(
                        negative.clone().select(float(0.5), float(0.))
                            + instance_index().to_float() * float(0.1 / 100000.),
                        negative.select(float(0.7), float(0.9)),
                        float(0.5),
                    )?
                }
                _ => hsl(
                    instance_index().to_float().less_than(scalar(2)).select(
                        float(0.5) + float(0.2) * instance_index().to_float() / scalar(2),
                        float(0.1),
                    ),
                    float(1.),
                    float(0.5),
                )?,
            };
            let tex = if id == 164 {
                mix(
                    vec4(splat(float(1.), Type::Vec3), float(1.)),
                    tex,
                    scalar(1),
                )
            } else {
                tex
            };
            let output = vec4(color * tex.rgb(), tex.swizzle("w"));
            let output = if id == 163 || id == 167 {
                alpha_test(output, float(0.499999))
            } else {
                output
            };
            let output = if id <= 164 {
                let fog = function(
                    "cloud_fog",
                    "fn cloud_fog(p:vec3<f32>,density:f32)->f32{let depth=-(u.view*u.model*vec4(p,1.0)).z;return exp(-density*density*depth*depth);}",
                    &[Type::Vec3, Type::Float],
                    Type::Float,
                    &[pos.clone(), float(if id == 163 { 0.001 } else { 0.0008 })],
                )?;
                let rgb = function(
                    "cloud_srgb",
                    "fn cloud_srgb(c:vec3<f32>)->vec3<f32>{return srgb_output(c);}",
                    &[Type::Vec3],
                    Type::Vec3,
                    &[output.clone().rgb()],
                )?;
                vec4(rgb * fog, output.swizzle("w"))
            } else if id == 167 {
                let fog = function(
                    "cloud_frame_fog",
                    "fn cloud_frame_fog(p:vec3<f32>)->f32{let clip=u.projection*u.view*u.model*vec4(p,1.0);return 1.0-smoothstep(200.0,600.0,clip.z);}",
                    &[Type::Vec3],
                    Type::Float,
                    std::slice::from_ref(&pos),
                )?;
                vec4(output.clone().rgb() * fog, output.swizzle("w"))
            } else if id == 165 {
                function(
                    "cloud_disc",
                    "fn cloud_disc(uv:vec2<f32>)->vec4<f32>{if length(uv-0.5)>0.475{discard;}return vec4(1.0);}",
                    &[Type::Vec2],
                    Type::Vec4,
                    &[uv()],
                )?
            } else {
                output
            };
            let size = match id {
                163 => float(35.),
                164 => float([20., 15., 10., 8., 5.][group]),
                165 => function(
                    "cloud_wave_size",
                    "fn cloud_wave_size(i:u32,t:f32)->f32{return (sin((f32(i/50u)+t)*0.3)+1.0)*20.0+(sin((f32(i%50u)+t)*0.5)+1.0)*20.0;}",
                    &[Type::Uint, Type::Float],
                    Type::Float,
                    &[instance_index(), scalar(0)],
                )?,
                166 => {
                    float(14.)
                        + float(13.)
                            * (instance_index().to_float() * float(0.1) + scalar(0) * float(5.))
                                .sin()
                }
                _ => instance_index().to_float().less_than(scalar(2)).select(
                    (float(26.)
                        + float(32.)
                            * (instance_index().to_float() * float(0.1) + scalar(0) * float(6.))
                                .sin())
                    .max(float(0.)),
                    float(40.),
                ),
            };
            let mut sprite = SpriteNodeMaterial::new(output);
            sprite.position = pos.clone();
            sprite.scale = function(
                "cloud_pixels",
                "fn cloud_pixels(p:vec3<f32>,size:f32,attenuate:f32,factor:f32,dpr:f32)->f32{let view=u.view*u.model*vec4(p,1.0);return clamp(select(size*dpr,size*factor/(-view.z),attenuate>0.5),1.0,511.0);}",
                &[
                    Type::Vec3,
                    Type::Float,
                    Type::Float,
                    Type::Float,
                    Type::Float,
                ],
                Type::Float,
                &[
                    pos.clone(),
                    size,
                    if id == 163 { scalar(1) } else { float(1.) },
                    if id <= 164 {
                        function(
                            "cloud_halfheight",
                            "fn cloud_halfheight()->f32{return u.point.y*0.5;}",
                            &[],
                            Type::Float,
                            &[],
                        )?
                    } else {
                        float(if id == 167 { 150. } else { 300. })
                    },
                    scalar(4),
                ],
            )?;
            let mut m = sprite.build_points(r, &buffers, &textures).await?;
            m.properties.transparent = matches!(id, 163 | 164 | 166);
            if id == 164 || id == 166 {
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
            let mut g = PlaneGeometry::build(1., 1., 1, 1)?;
            g.instance_count = Some(count);
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(g),
                Arc::new(Material::Shader(m)),
            )));
            s.get_mut(h)?.frustum_culled = false;
            objects.push(h);
            rotations.push(if id == 164 {
                [
                    random(&mut seed) * 6.,
                    random(&mut seed) * 6.,
                    random(&mut seed) * 6.,
                ]
            } else {
                [0.; 3]
            });
        }
        Ok(Self {
            id,
            time: 0.,
            objects,
            rotations,
            pointer: Vector2::ZERO,
            enabled: true,
            inner,
        })
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        if self.id <= 165 {
            let n = s.get_mut(c)?;
            n.position.x += (self.pointer.x - n.position.x) * 0.05;
            n.position.y += (-self.pointer.y - n.position.y) * 0.05;
            s.look_at(c, Vector3::ZERO)?;
        }
        for (i, &h) in self.objects.iter().enumerate() {
            let n = s.get_mut(h)?;
            n.quaternion = match self.id {
                164 => Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    self.rotations[i][0],
                    self.time
                        * 0.05
                        * if i < 4 {
                            (i + 1) as f64
                        } else {
                            -(i as f64 + 1.)
                        },
                    self.rotations[i][2],
                ),
                166 => Quaternion::from_rotation_z(self.time * 0.05),
                167 => Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    0.,
                    self.time * 0.2,
                    self.time * 0.2,
                ),
                _ => Quaternion::IDENTITY,
            };
            if let NodeKind::Mesh(mesh) = &mut n.kind
                && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
            {
                m.uniforms[0][0] = (self.time * if self.id == 165 { 6. } else { 1. }) as f32;
                m.uniforms[1][0] = f32::from(self.enabled);
                m.uniforms[2][0] = self.inner as f32;
                m.uniforms[4][0] = web_sys::window().unwrap().device_pixel_ratio() as f32;
                if self.id == 164 {
                    let h = 1. - i as f64 * 0.05 + self.time * 0.05;
                    let color = hue(h, [0.2, 0.1, 0.05, 0., 0.][i], 0.5);
                    for (k, v) in color.into_iter().enumerate() {
                        m.uniforms[3][k] = if v <= 0.04045 {
                            v / 12.92
                        } else {
                            ((v + 0.055) / 1.055).powf(2.4)
                        } as f32;
                    }
                }
            }
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new(x, y);
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if !matches!(self.id, 163 | 164) || index != 0 {
            return Err(Error::Invalid("point cloud parameter"));
        }
        self.enabled = value > 0.5;
        Ok(())
    }
}
