//! Procedural and instanced geometry. Static records remain resident on the GPU.
use super::gltf_viewer::{OrbitViewer, decode_texture_image, fetch, load_asset};
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
    shader::ShaderProgram,
    tsl::{self, *},
};
use std::sync::Arc;
const ASSETS: &str = "/web/gallery/assets/shader-geometry";
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn call(name: &str, source: &str, args: &[Type], output: Type, nodes: &[Node]) -> Result<Node> {
    Ok(WgslFn::new(name, source, args, output)?.call(nodes))
}
fn attribute(g: &mut BufferGeometry, name: &str, data: Vec<f32>, size: usize) -> Result<()> {
    g.set_attribute(
        name,
        Attribute::F32(BufferAttribute::new(data, size, false)?),
    );
    Ok(())
}
const HASH: &str = "fn hash_u(v:u32)->u32{var x=v;x+=x<<10u;x^=x>>6u;x+=x<<3u;x^=x>>11u;x+=x<<15u;return x;}fn noise_u(i:f32,c:f32)->f32{return bitcast<f32>((hash_u(bitcast<u32>(i)^hash_u(bitcast<u32>(c)))&0x007fffffu)|0x3f800000u)-1.0;}fn random_v(i:u32)->vec3<f32>{return vec3(noise_u(f32(i),0.0),noise_u(f32(i),1.0),noise_u(f32(i),2.0));}";
const ROTATE: &str = "fn rotate_q(q:vec4<f32>,p:vec3<f32>)->vec3<f32>{let a=cross(q.xyz,p);return p+2.0*(q.w*a+cross(q.xyz,a));}";
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    rotation: Quaternion,
    objects: Vec<Object3D>,
    count: u32,
    viewer: OrbitViewer,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let (fov, near, far, z) = match id {
            168 => (27., 1., 3500., 4.),
            169 => (27., 1., 3500., 2500.),
            170 => (50., 1., 10., 2.),
            171 => (50., 1., 1000., 0.),
            _ => (27., 0.1, 100., 20.),
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
        s.background = match id {
            168 | 169 => Color::linear(5. / 255., 5. / 255., 5. / 255.),
            171 => Color::from_hex(0x101010),
            _ => Color::BLACK,
        };
        let mut objects = vec![];
        if id == 172 {
            let (asset, buffers, images) = load_asset("/web/models/LeePerrySmith.glb").await?;
            let mut temp = Scene::default();
            crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(&mut temp)?;
            let g = temp
                .handles()
                .find_map(|h| match &temp.get(h).ok()?.kind {
                    NodeKind::Mesh(m) => Some(m.geometry.clone()),
                    _ => None,
                })
                .ok_or(Error::Invalid("head geometry"))?;
            for (x, amount) in [(-3.5, 2.), (3.5, -2.)] {
                let position = call(
                    "twist",
                    "fn twist(p:vec3<f32>,t:f32,a:f32)->vec3<f32>{let theta=sin(t+p.y)/a;let c=cos(theta);let s=sin(theta);return vec3(c*p.x+s*p.z,p.y,-s*p.x+c*p.z);}",
                    &[Type::Vec3, Type::Float, Type::Float],
                    Type::Vec3,
                    &[position_geometry(), uniform(0, Type::Float), float(amount)],
                )?;
                let graph = NodeMaterial {
                    color: base_color(),
                    position: Some(position),
                };
                let projection = format!(
                    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{var out=surface;let theta=sin(u.custom[0].x+position.y)/{amount:?};let c=cos(theta);let s=sin(theta);let n=surface.normal;out.normal=vec3(c*n.x+s*n.z,n.y,-s*n.x+c*n.z);return out;}}"
                );
                let mut m = MeshNormalMaterial::default();
                m.properties.vertex_program = Some(Arc::new(
                    ShaderProgram::with_projection(r, &graph.wgsl(0)?, &[], &[], &projection)
                        .await?,
                ));
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    g.clone(),
                    Arc::new(Material::Normal(m)),
                )));
                s.get_mut(h)?.position = Vector3::new(x, -0.5, 0.);
                objects.push(h);
            }
        } else {
            let count = if id == 170 {
                50000
            } else if id == 171 {
                5000
            } else {
                10000
            };
            let mut seed = 186;
            let mut data = vec![];
            let mut g = BufferGeometry::default();
            if id == 171 {
                let v: serde_json::Value =
                    serde_json::from_slice(&fetch(&format!("{ASSETS}/cube.json")).await?)
                        .map_err(|e| Error::Asset(e.to_string()))?;
                let words = v["vertices"]
                    .as_array()
                    .ok_or(Error::Invalid("cube vertices"))?
                    .iter()
                    .map(|x| x.as_f64().unwrap_or(0.) as f32)
                    .collect::<Vec<_>>();
                attribute(
                    &mut g,
                    "position",
                    words
                        .as_chunks::<8>()
                        .0
                        .iter()
                        .flat_map(|p| p[..3].iter().copied())
                        .collect(),
                    3,
                )?;
                attribute(
                    &mut g,
                    "uv",
                    words
                        .as_chunks::<8>()
                        .0
                        .iter()
                        .flat_map(|p| p[4..6].iter().copied())
                        .collect(),
                    2,
                )?;
                g.set_index(Some(
                    v["indices"]
                        .as_array()
                        .ok_or(Error::Invalid("cube indices"))?
                        .iter()
                        .map(|i| i.as_u64().unwrap_or(0) as u32)
                        .collect(),
                ));
                for _ in 0..count {
                    let offset = Vector3::new(
                        random(&mut seed) * 100. - 50.,
                        random(&mut seed) * 100. - 50.,
                        random(&mut seed) * 100. - 50.,
                    );
                    let offset = offset + offset.normalize() * 5.;
                    let q = Quaternion::from_xyzw(
                        random(&mut seed) * 2. - 1.,
                        random(&mut seed) * 2. - 1.,
                        random(&mut seed) * 2. - 1.,
                        random(&mut seed) * 2. - 1.,
                    )
                    .normalize();
                    data.extend(
                        Matrix4::from_rotation_translation(q, offset)
                            .to_cols_array()
                            .map(|v| v as f32),
                    );
                }
            } else {
                attribute(
                    &mut g,
                    "position",
                    if id == 170 {
                        vec![0.025, -0.025, 0., -0.025, 0.025, 0., 0., 0., 0.025]
                    } else {
                        vec![0.; 9]
                    },
                    3,
                )?;
                attribute(&mut g, "uv", vec![0., 0., 0.5, 1., 1., 0.], 2)?;
                if id == 169 {
                    for _ in 0..count {
                        let p = [
                            random(&mut seed) * 800. - 400.,
                            random(&mut seed) * 800. - 400.,
                            random(&mut seed) * 800. - 400.,
                        ];
                        for _ in 0..3 {
                            for x in p {
                                data.push((x + random(&mut seed) * 50. - 25.) as f32);
                            }
                        }
                    }
                }
                if id == 170 {
                    for _ in 0..count {
                        for _ in 0..3 {
                            data.push((random(&mut seed) - 0.5) as f32);
                        }
                        for _ in 0..4 {
                            data.push(random(&mut seed) as f32);
                        }
                        for _ in 0..2 {
                            let q = Quaternion::from_xyzw(
                                random(&mut seed) * 2. - 1.,
                                random(&mut seed) * 2. - 1.,
                                random(&mut seed) * 2. - 1.,
                                random(&mut seed) * 2. - 1.,
                            )
                            .normalize();
                            data.extend(q.to_array().map(|v| v as f32));
                        }
                    }
                }
            }
            g.instance_count = Some(count);
            let buffer = if data.is_empty() {
                None
            } else {
                Some(GpuBuffer::new(
                    r,
                    bytemuck::cast_slice(&data),
                    BufferAccess::Read,
                )?)
            };
            let bindings: Vec<_> = buffer.iter().map(|b| (b, Type::Float)).collect();
            let mut textures = vec![];
            if id == 169 || id == 171 {
                for name in if id == 169 {
                    vec!["crate.gif", "checker.jpg", "grass.jpg"]
                } else {
                    vec!["crate.gif"]
                } {
                    let mut t =
                        decode_texture_image(&fetch(&format!("{ASSETS}/{name}")).await?).await?;
                    t.srgb = id == 171;
                    t.flip_y = id != 171;
                    t.mipmap_filter = Some(Filter::Linear);
                    textures.push(r.upload_texture(&Arc::new(t))?);
                }
            }
            let images: Vec<_> = textures.iter().map(|t| (&t.view, &t.sampler)).collect();
            let position = match id {
                168 => call(
                    "procedural_pos",
                    &format!(
                        "{HASH}fn procedural_pos(i:u32,v:u32)->vec3<f32>{{return random_v(i)*2.0-1.0+(random_v(i*3u+v)*2.0-1.0)/64.0;}}"
                    ),
                    &[Type::Uint, Type::Uint],
                    Type::Vec3,
                    &[instance_index(), vertex_index()],
                )?,
                169 => call(
                    "integer_pos",
                    "fn integer_pos(i:u32,v:u32)->vec3<f32>{let b=i*9u+v*3u;return vec3(tsl_attribute_0[b],tsl_attribute_0[b+1u],tsl_attribute_0[b+2u]);}",
                    &[Type::Uint, Type::Uint],
                    Type::Vec3,
                    &[instance_index(), vertex_index()],
                )?,
                170 => call(
                    "instance_pos",
                    &format!(
                        "{ROTATE}fn read4(b:u32)->vec4<f32>{{return vec4(tsl_attribute_0[b],tsl_attribute_0[b+1u],tsl_attribute_0[b+2u],tsl_attribute_0[b+3u]);}}fn instance_pos(i:u32,p:vec3<f32>,t:f32)->vec3<f32>{{let b=i*15u;let st=sin(t*0.05);let offset=read4(b).xyz;let q=normalize(mix(read4(b+7u),read4(b+11u),st));return rotate_q(q,offset*max(abs(st*2.0+1.0),0.5)+p);}}"
                    ),
                    &[Type::Uint, Type::Vec3, Type::Float],
                    Type::Vec3,
                    &[
                        instance_index(),
                        position_geometry(),
                        uniform(0, Type::Float),
                    ],
                )?,
                _ => call(
                    "cube_pos",
                    &format!(
                        "{ROTATE}fn column(b:u32)->vec4<f32>{{return vec4(tsl_attribute_0[b],tsl_attribute_0[b+1u],tsl_attribute_0[b+2u],tsl_attribute_0[b+3u]);}}fn cube_pos(i:u32,p:vec3<f32>,q:vec4<f32>)->vec3<f32>{{let b=i*16u;let m=mat4x4(column(b),column(b+4u),column(b+8u),column(b+12u));return (m*vec4(rotate_q(q,p),1.0)).xyz;}}"
                    ),
                    &[Type::Uint, Type::Vec3, Type::Vec4],
                    Type::Vec3,
                    &[
                        instance_index(),
                        position_geometry(),
                        uniform(1, Type::Vec4),
                    ],
                )?,
            };
            let color = match id {
                168 => vec4(
                    call(
                        "procedural_color",
                        &format!(
                            "{}fn procedural_color(i:u32)->vec3<f32>{{return 0.25+0.75*color_random_v(i);}}",
                            HASH.replace("hash_u", "color_hash_u")
                                .replace("noise_u", "color_noise_u")
                                .replace("random_v", "color_random_v")
                        ),
                        &[Type::Uint],
                        Type::Vec3,
                        &[instance_index()],
                    )?,
                    float(1.),
                ),
                169 => call(
                    "integer_color",
                    "fn integer_color(i:u32,uv:vec2<f32>)->vec4<f32>{let dx=dpdx(uv);let dy=dpdy(uv);switch i%3u{case 0u:{return textureSampleGrad(tsl_texture_0,tsl_sampler_0,uv,dx,dy);}case 1u:{return textureSampleGrad(tsl_texture_1,tsl_sampler_1,uv,dx,dy);}default:{return textureSampleGrad(tsl_texture_2,tsl_sampler_2,uv,dx,dy);}}}",
                    &[Type::Uint, Type::Vec2],
                    Type::Vec4,
                    &[instance_index(), vec2(uv().x(), float(1.) - uv().y())],
                )?,
                170 => call(
                    "instance_color",
                    "fn instance_color(i:u32,p:vec3<f32>,t:f32)->vec4<f32>{let b=i*15u+3u;return vec4(tsl_attribute_0[b]+sin(p.x*10.0+t)*0.5,tsl_attribute_0[b+1u],tsl_attribute_0[b+2u],tsl_attribute_0[b+3u]);}",
                    &[Type::Uint, Type::Vec3, Type::Float],
                    Type::Vec4,
                    &[instance_index(), position_local(), uniform(0, Type::Float)],
                )?,
                _ => tsl::Texture::External(0).sample(uv()),
            };
            let graph = NodeMaterial {
                color,
                position: Some(position),
            };
            let mut m = graph.build_with_storage(r, &bindings, &images).await?;
            m.properties.side = if id == 171 { Side::Front } else { Side::Double };
            m.properties.transparent = id == 170;
            m.uniforms[1] = [0., 0., 0., 1.];
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(g),
                Arc::new(Material::Shader(m)),
            )));
            s.get_mut(h)?.frustum_culled = false;
            objects.push(h);
        }
        Ok(Self {
            id,
            time: 0.,
            last: 0.,
            rotation: Quaternion::IDENTITY,
            objects,
            count: 50000,
            viewer: OrbitViewer::from_camera(Vector3::ZERO, 20.),
        })
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        let t = self.time;
        if self.id == 171 {
            let d = (t - self.last) / 5.;
            let a = d / 3f64.sqrt();
            self.rotation =
                (self.rotation * Quaternion::from_xyzw(a, a, a, 1.).normalize()).normalize();
            self.last = t;
        }
        if self.id == 172 {
            self.viewer.update(s, c)?;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
                p.near = 0.1;
                p.far = 100.;
            }
        }
        for h in &self.objects {
            let n = s.get_mut(*h)?;
            if self.id != 172 {
                n.quaternion = if self.id == 170 || self.id == 171 {
                    Quaternion::from_rotation_y(t * if self.id == 170 { 0.5 } else { 0.05 })
                } else {
                    Quaternion::from_euler(glam::EulerRot::XYZ, t * 0.25, t * 0.5, 0.)
                };
            }
            if self.id == 170 {
                n.instance_count = Some(self.count);
            }
            if let NodeKind::Mesh(m) = &mut n.kind {
                let material = Arc::make_mut(&mut m.materials[0]);
                if let Material::Shader(m) = material {
                    m.uniforms[0][0] = (t * if self.id == 170 { 5. } else { 1. }) as f32;
                    m.uniforms[1] = self.rotation.to_array().map(|x| x as f32);
                } else {
                    material.properties_mut().vertex_uniforms[0][0] = t as f32;
                }
            }
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if self.id != 170 || i != 0 || !v.is_finite() {
            return Err(Error::Invalid("geometry parameter"));
        }
        self.count = v.clamp(0., 50000.) as u32;
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
        if self.id == 172 {
            if pan {
                self.viewer.pan_pixels(s, c, dx, dy, height)?;
            } else {
                self.viewer.orbit_pixels(dx, dy, wheel, height, 10., 50.);
            }
        }
        Ok(())
    }
}
