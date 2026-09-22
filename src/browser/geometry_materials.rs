//! Barycentric edges, scientific LUT colors and compact normalized integer attributes.
use super::gltf_viewer::{OrbitViewer, fetch};
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
    tsl::{self, surface::SurfaceNodes, *},
};
use std::sync::Arc;
const ASSETS: &str = "/web/gallery/assets/geometry-materials";
fn call(name: &str, source: &str, args: &[Type], out: Type, nodes: &[Node]) -> Result<Node> {
    Ok(WgslFn::new(name, source, args, out)?.call(nodes))
}
fn mesh(s: &mut Scene, g: Arc<BufferGeometry>, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(m))))
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    viewer: OrbitViewer,
    objects: Vec<Object3D>,
    value: f32,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let (fov, near, far, z) = match id {
            173 => (40., 1., 500., 200.),
            176 => (60., 1., 100., 10.),
            _ => (27., 1., 3500., 2750.),
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
        s.background = if id == 176 {
            Color::WHITE
        } else if id == 177 {
            Color::linear(5. / 255., 5. / 255., 5. / 255.)
        } else {
            Color::BLACK
        };
        let mut d = Self {
            id,
            time: 0.,
            viewer: OrbitViewer::from_camera(Vector3::ZERO, z),
            objects: vec![],
            value: if id == 173 { 1. } else { 0. },
        };
        if id == 173 {
            let g = Arc::new(
                super::tsl_materials::geometries(
                    &fetch(&format!("{ASSETS}/wireframe.bin")).await?,
                )?
                .remove(0),
            );
            let mut m = NodeMaterial::new(vec3(float(224. / 255.), float(224. / 255.), float(1.)))
                .build(r, &[])
                .await?;
            m.properties.wireframe = true;
            let h = mesh(s, g.clone(), Material::Shader(m));
            s.get_mut(h)?.position.x = -40.;
            let rgba = call(
                "edge_color",
                "fn edge_color(uv:vec2<f32>,thickness:f32,front:bool)->vec4<f32>{let center=vec3(uv,1.0-uv.x-uv.y);let width=fwidth(center);let edge=smoothstep((thickness-1.0)*width,thickness*width,center);return vec4(select(vec3(0.4,0.4,0.5),vec3(0.9,0.9,1.0),front),1.0-min(min(edge.x,edge.y),edge.z));}",
                &[Type::Vec2, Type::Float, Type::Bool],
                Type::Vec4,
                &[uv(), uniform(0, Type::Float), front_facing()],
            )?;
            let mut m = NodeMaterial::new(rgba).build(r, &[]).await?;
            m.properties.side = Side::Double;
            m.properties.alpha_to_coverage = true;
            m.uniforms[0][0] = 1.;
            let h = mesh(s, g, Material::Shader(m));
            s.get_mut(h)?.position.x = 40.;
            d.objects.push(h);
        } else if id == 176 {
            let g = Arc::new(
                super::tsl_materials::geometries(&fetch(&format!("{ASSETS}/pressure.bin")).await?)?
                    .remove(0),
            );
            let colors = GpuBuffer::new(
                r,
                &fetch(&format!("{ASSETS}/lut.bin")).await?,
                BufferAccess::Read,
            )?;
            let graph = NodeMaterial::new(base_color());
            let code = graph.wgsl_with_storage(0, &[Type::Vec4])?;
            let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let index=u32(floor(clamp(surface.uv.x/2000.0,0.0,1.0)*32.0+0.5));out.color=tsl_attribute_0[u32(u.custom[0].x)*33u+index];return out;}";
            let mut m = MeshLambertMaterial::default();
            m.properties.color = Color::from_hex(0xf5f5f5);
            m.properties.side = Side::Double;
            m.properties.vertex_colors = true;
            m.properties.vertex_program = Some(Arc::new(
                ShaderProgram::with_projection(r, &code, &[&colors], &[], projection).await?,
            ));
            d.objects.push(mesh(s, g, Material::Lambert(m)));
            let light = s.insert(NodeKind::Light(Light::Point {
                color: Color::WHITE,
                intensity: 3.,
                distance: 0.,
                decay: 0.,
            }));
            s.add(c, light)?;
            let mut texture = crate::material::Texture::from_rgba(
                1,
                128,
                fetch(&format!("{ASSETS}/lut.rgba")).await?,
                true,
            )?;
            texture.mipmap_filter = None;
            let t = r.upload_texture(&Arc::new(texture))?;
            let y = (float(1.) - uv().y()) * float(32.);
            let sample = tsl::Texture::External(0).sample(vec2(
                float(0.5),
                (y.clamp(float(0.5), float(31.5)) + uniform(0, Type::Float) * float(32.))
                    / float(128.),
            ));
            let graph = NodeMaterial::new(sample);
            let mut m=ShaderMaterial::new(Arc::new(ShaderProgram::with_projection(r,&graph.wgsl(1)?,&[],&[(&t.view,&t.sampler)],"fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.x*0.125-0.5,position.y,0.0,1.0);return out;}").await?));
            m.properties.depth_test = false;
            m.properties.depth_write = false;
            let h = mesh(
                s,
                Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
                Material::Shader(m),
            );
            s.get_mut(h)?.frustum_culled = false;
            s.get_mut(h)?.render_order = 1;
            d.objects.push(h);
        } else {
            let mut seed = 186u32;
            let mut rand = || {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                seed as f64 / 4294967296.
            };
            let mut words = Vec::<u32>::with_capacity(500000 * 12);
            for _ in 0..500000 {
                let p = Vector3::new(
                    rand() * 800. - 400.,
                    rand() * 800. - 400.,
                    rand() * 800. - 400.,
                );
                let vertices = std::array::from_fn::<_, 3, _>(|_| {
                    p + Vector3::new(rand() * 12. - 6., rand() * 12. - 6., rand() * 12. - 6.)
                });
                for v in vertices {
                    words.extend(v.to_array().map(|x| (x as f32).to_bits()));
                }
                let n = (vertices[2] - vertices[1])
                    .cross(vertices[0] - vertices[1])
                    .normalize_or_zero()
                    .to_array()
                    .map(|x| (x * 32767.) as i16 as u16 as u32);
                words.extend([n[0] | n[1] << 16, n[2]]);
                let rgb = (p / 800. + Vector3::splat(0.5))
                    .to_array()
                    .map(|x| (x * 255.) as u32);
                words.push(rgb[0] | rgb[1] << 8 | rgb[2] << 16);
            }
            let b = GpuBuffer::new(r, bytemuck::cast_slice(&words), BufferAccess::Read)?;
            let p = call(
                "uint_position",
                "fn uint_position(i:u32,v:u32)->vec3<f32>{let b=i*12u+v*3u;return vec3(bitcast<f32>(tsl_attribute_0[b]),bitcast<f32>(tsl_attribute_0[b+1u]),bitcast<f32>(tsl_attribute_0[b+2u]));}",
                &[Type::Uint, Type::Uint],
                Type::Vec3,
                &[instance_index(), vertex_index()],
            )?;
            let normal = call(
                "uint_normal",
                "fn uint_normal(i:u32)->vec3<f32>{let b=i*12u;let xy=tsl_attribute_0[b+9u];let z=tsl_attribute_0[b+10u];return vec3(f32(bitcast<i32>(xy<<16u)>>16),f32(bitcast<i32>(xy)>>16),f32(bitcast<i32>(z<<16u)>>16))/32767.0;}",
                &[Type::Uint],
                Type::Vec3,
                &[instance_index()],
            )?;
            let color = call(
                "uint_color",
                "fn uint_color(i:u32)->vec3<f32>{let x=tsl_attribute_0[i*12u+11u];return vec3(f32(x&255u),f32((x>>8u)&255u),f32((x>>16u)&255u))/255.0;}",
                &[Type::Uint],
                Type::Vec3,
                &[instance_index()],
            )?;
            let tint = Color::from_hex(0xd5d5d5).0;
            let output = call(
                "uint_output",
                "fn uint_output(c:vec4<f32>)->vec4<f32>{return vec4(mix(srgb_output(c.rgb),vec3(5.0/255.0),smoothstep(2000.0,3500.0,fragment_view_z)),c.a);}",
                &[Type::Vec4],
                Type::Vec4,
                &[output()],
            )?;
            let mut m = MeshPhongMaterial::default();
            m.properties.side = Side::Double;
            m.specular = Color::WHITE;
            m.shininess = 250.;
            m.properties.vertex_program = Some(Arc::new(
                SurfaceNodes {
                    position: Some(p),
                    vertex_normal: Some(normal),
                    color: Some(
                        color
                            * vec3(
                                float(tint.x as f32),
                                float(tint.y as f32),
                                float(tint.z as f32),
                            ),
                    ),
                    output: Some(output),
                    ..Default::default()
                }
                .build(r, &[(&b, Type::Uint)], &[])
                .await?,
            ));
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(vec![0.; 9], 3, false)?),
            );
            g.instance_count = Some(500000);
            let h = mesh(s, Arc::new(g), Material::Phong(m));
            s.get_mut(h)?.frustum_culled = false;
            d.objects.push(h);
            s.insert(NodeKind::Light(Light::Ambient {
                color: Color::from_hex(0xcccccc),
                intensity: 1.,
            }));
            for (position, intensity) in [(Vector3::ONE, 1.5), (Vector3::new(0., -1., 0.), 4.5)] {
                let h = s.insert(NodeKind::Light(Light::Directional {
                    color: Color::WHITE,
                    intensity,
                    target: Vector3::ZERO,
                }));
                s.get_mut(h)?.position = position;
            }
        }
        Ok(d)
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        if self.id != 177 {
            self.viewer.update(s, c)?;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
                p.near = 1.;
                p.far = if self.id == 173 { 500. } else { 100. };
            }
        }
        for &h in &self.objects {
            let n = s.get_mut(h)?;
            if self.id == 177 {
                n.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    self.time * 0.25,
                    self.time * 0.5,
                    0.,
                );
            }
            if let NodeKind::Mesh(m) = &mut n.kind {
                let m = Arc::make_mut(&mut m.materials[0]);
                if let Material::Shader(m) = m {
                    m.uniforms[0][0] = self.value;
                } else {
                    m.properties_mut().vertex_uniforms[0][0] = self.value;
                }
            }
        }
        Ok(())
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i != 0 || self.id == 177 || !v.is_finite() {
            return Err(Error::Invalid("geometry material parameter"));
        }
        self.value = if self.id == 173 {
            v.clamp(0., 4.)
        } else {
            v.round().clamp(0., 3.)
        };
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        w: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if self.id == 177 {
            return Ok(());
        }
        if pan {
            if self.id == 176 {
                self.viewer.pan_pixels(s, c, dx, dy, height)?;
            }
        } else {
            self.viewer.orbit_pixels(
                dx,
                dy,
                if self.id == 173 { 0. } else { w },
                height,
                0.,
                f64::INFINITY,
            );
        }
        Ok(())
    }
}
