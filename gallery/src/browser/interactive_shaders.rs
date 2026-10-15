//! Fullscreen raw shaders and CPU-raycast selection scenes from the pinned WebGL examples.
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::tsl::Node;
use crate::{
    Error, Result,
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    geometry::*,
    material::*,
    math::*,
    raycast::Raycaster,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    tsl::{self, sprites::SpriteNodeMaterial, *},
};
use std::sync::Arc;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn call(name: &str, source: &str, args: &[Type], out: Type, nodes: &[Node]) -> Result<Node> {
    Ok(WgslFn::new(name, source, args, out)?.call(nodes))
}
/// Three.js's `rand` shader chunk; GLSL `mod` floors, unlike WGSL `%`.
const RAND: &str = "fn procedural_rand(uv:vec2<f32>)->f32{let dt=dot(uv,vec2(12.9898,78.233));let sn=dt-3.141592653589793*floor(dt/3.141592653589793);return fract(sin(sn)*43758.5453);}";
/// The original `webgl_shader` fragment, statement for statement.
const MONJORI: &str = "fn glsl_mod(x:f32,y:f32)->f32{return x-y*floor(x/y);}
fn monjori(uv:vec2<f32>,time:f32)->vec4<f32>{
let p=-1.0+2.0*uv;let a=time*40.0;let g=1.0/40.0;
var e=400.0*(p.x*0.5+0.5);var f=400.0*(p.y*0.5+0.5);
var i=200.0+sin(e*g+a/150.0)*20.0;
var d=200.0+cos(f*g/2.0)*18.0+cos(e*g)*7.0;
let r=sqrt(pow(abs(i-e),2.0)+pow(abs(d-f),2.0));
let q=f/r;
e=(r*cos(q))-a/2.0;f=(r*sin(q))-a/2.0;
d=sin(e*g)*176.0+sin(e*g)*164.0+r;
var h=((f+d)+a/2.0)*g;
i=cos(h+r*p.x/1.3)*(e+e+a)+cos(q*g*6.0)*(r+h/3.0);
h=sin(f*g)*144.0-sin(e*g)*212.0*p.x;
h=(h+(f-e)*q+sin(r-(a+h)/7.0)*10.0+i/4.0)*g;
i+=cos(h*2.3*sin(a/350.0-q))*184.0*sin(q-(r*4.3+a/12.0)*g)+tan(r*g+h)*184.0*cos(r*g+h);
i=glsl_mod(i/5.6,256.0)/64.0;
if i<0.0 {i+=4.0;}
if i>=2.0 {i=4.0-i;}
d=r/350.0;d+=sin(d*d*8.0)*0.52;
f=(sin(a*g)+1.0)/2.0;
return vec4(vec3(f*i/1.6,i/2.0+d/13.0,i)*d*p.x+vec3(i/1.3+d/8.0,i/2.0+d/18.0,i)*d*(1.0-p.x),1.0);}";
/// `gl_Position = vec4( position, 1.0 )`, and the identity orthographic camera.
const FULLSCREEN: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.xy,0.0,1.0);return out;}";
const PARTICLE_SIZE: f32 = 20.;
pub(super) struct Demo {
    id: u32,
    time: f64,
    pointer: Vector2,
    objects: Vec<Object3D>,
    materials: Vec<Arc<Material>>,
    procedure: usize,
    intersected: Option<usize>,
    /// JavaScript's initial `undefined`, which differs from the later `null`.
    undefined: bool,
    /// Raycast-only Points node holding the resident sprite positions.
    query: Option<(Scene, Object3D)>,
    sizes: Vec<f32>,
    size_buffer: Option<GpuBuffer>,
    dirty: bool,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let mut d = Self {
            id,
            time: 0.,
            pointer: Vector2::ZERO,
            objects: vec![],
            materials: vec![],
            procedure: 2,
            intersected: None,
            undefined: true,
            query: None,
            sizes: vec![],
            size_buffer: None,
            dirty: false,
        };
        s.background = Color::BLACK;
        match id {
            183 | 184 => {
                let sources: Vec<Node> = if id == 183 {
                    vec![call(
                        "monjori",
                        MONJORI,
                        &[Type::Vec2, Type::Float],
                        Type::Vec4,
                        &[uv(), uniform(0, Type::Float)],
                    )?]
                } else {
                    let rand = |offset: [f32; 2]| {
                        call(
                            "procedural_rand",
                            RAND,
                            &[Type::Vec2],
                            Type::Float,
                            &[uv() + vec2(float(offset[0]), float(offset[1]))],
                        )
                    };
                    let r0 = call("procedural_rand", RAND, &[Type::Vec2], Type::Float, &[uv()])?;
                    let r1 = rand([0.4, 0.6])?;
                    let r2 = rand([0.6, 0.4])?;
                    vec![
                        vec4(splat(r0.clone(), Type::Vec3), float(1.)),
                        vec4(
                            mix(
                                mix(
                                    vec3(float(1.), float(1.), float(1.)),
                                    vec3(float(0.), float(0.), float(1.)),
                                    r0.clone(),
                                ),
                                splat(float(0.), Type::Vec3),
                                r1.clone(),
                            ),
                            float(1.),
                        ),
                        vec4(vec3(r0, r1, r2), float(1.)),
                    ]
                };
                for color in sources {
                    let graph = NodeMaterial::new(color);
                    let mut m = ShaderMaterial::new(Arc::new(
                        ShaderProgram::with_projection(r, &graph.wgsl(0)?, &[], &[], FULLSCREEN)
                            .await?,
                    ));
                    m.properties.side = Side::Front;
                    d.materials.push(Arc::new(Material::Shader(m)));
                }
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
                    d.materials
                        .last()
                        .cloned()
                        .ok_or(Error::Invalid("shader"))?,
                )));
                s.get_mut(h)?.frustum_culled = false;
                d.objects.push(h);
            }
            185 | 186 => {
                s.get_mut(c)?.kind = NodeKind::Camera(if id == 185 {
                    Camera::Perspective(PerspectiveCamera {
                        fov: 70.,
                        near: 0.1,
                        far: 100.,
                        aspect: match s.camera(c)?.0 {
                            Camera::Perspective(p) => p.aspect,
                            _ => 1.,
                        },
                        ..Default::default()
                    })
                } else {
                    Camera::Orthographic(OrthographicCamera {
                        left: -25.,
                        right: 25.,
                        top: 25.,
                        bottom: -25.,
                        near: 0.1,
                        far: 100.,
                        ..Default::default()
                    })
                });
                s.background = Color::from_hex(0xf0f0f0);
                let light = s.insert(NodeKind::Light(Light::Directional {
                    color: Color::WHITE,
                    intensity: 3.,
                    target: Vector3::ZERO,
                }));
                s.get_mut(light)?.position = Vector3::ONE.normalize();
                let g = Arc::new(BoxGeometry::build(1., 1., 1.)?);
                let mut seed = 186;
                for _ in 0..2000 {
                    let mut m = MeshLambertMaterial::default();
                    // `Color.setHex` floors the scaled random value and decodes sRGB.
                    m.properties.color =
                        Color::from_hex((random(&mut seed) * 16777215.).floor() as u32);
                    let position = Vector3::new(
                        random(&mut seed) * 40. - 20.,
                        random(&mut seed) * 40. - 20.,
                        random(&mut seed) * 40. - 20.,
                    );
                    let angles = Vector3::new(
                        random(&mut seed) * std::f64::consts::TAU,
                        random(&mut seed) * std::f64::consts::TAU,
                        random(&mut seed) * std::f64::consts::TAU,
                    );
                    let scale = Vector3::new(
                        random(&mut seed) + 0.5,
                        random(&mut seed) + 0.5,
                        random(&mut seed) + 0.5,
                    );
                    let h = s.insert(NodeKind::Mesh(Mesh::new(
                        g.clone(),
                        Arc::new(Material::Lambert(m)),
                    )));
                    let n = s.get_mut(h)?;
                    n.position = position;
                    n.quaternion = Euler {
                        angles,
                        order: EulerOrder::XYZ,
                    }
                    .quaternion();
                    n.scale = scale;
                    d.objects.push(h);
                }
            }
            187 => {
                s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                    fov: 45.,
                    near: 1.,
                    far: 10000.,
                    aspect: match s.camera(c)?.0 {
                        Camera::Perspective(p) => p.aspect,
                        _ => 1.,
                    },
                    ..Default::default()
                }));
                s.get_mut(c)?.position = Vector3::new(0., 0., 250.);
                let positions =
                    merged_box_vertices(&BoxGeometry::segmented(200., 200., 200., 16, 16, 16)?)?;
                let count = positions.len();
                let mut records = Vec::with_capacity(count * 8);
                for (i, p) in positions.iter().enumerate() {
                    // `Color.setHSL` defaults to the working color space: no sRGB decoding.
                    let rgb = hsl(0.01 + 0.1 * (i as f64 / count as f64), 1., 0.5);
                    records.extend([p.x as f32, p.y as f32, p.z as f32, 0.]);
                    records.extend(rgb.map(|x| x as f32));
                    records.push(0.);
                }
                d.sizes = vec![PARTICLE_SIZE * 0.5; count];
                let attributes =
                    GpuBuffer::new(r, bytemuck::cast_slice(&records), BufferAccess::Read)?;
                let sizes = GpuBuffer::new(r, bytemuck::cast_slice(&d.sizes), BufferAccess::Read)?;
                let mut image = decode_texture_image(
                    &fetch("/web/gallery/assets/point-clouds/disc.png").await?,
                )
                .await?;
                image.srgb = false;
                // TextureLoader defaults: generated mipmaps, trilinear minification.
                image.mipmap_filter = Some(Filter::Linear);
                let disc = r.upload_texture(&Arc::new(image))?;
                let position = call(
                    "point_position",
                    "fn point_position(i:u32)->vec3<f32>{return tsl_attribute_0[i*2u].xyz;}",
                    &[Type::Uint],
                    Type::Vec3,
                    &[instance_index()],
                )?;
                let color = call(
                    "point_color",
                    "fn point_color(i:u32,t:vec4<f32>)->vec4<f32>{let c=vec4(tsl_attribute_0[i*2u+1u].rgb,1.0)*t;if c.a<0.9 {discard;}return c;}",
                    &[Type::Uint, Type::Vec4],
                    Type::Vec4,
                    &[
                        instance_index(),
                        // flipY upload + top-down gl_PointCoord: the quad's upward uv, unflipped.
                        tsl::Texture::External(0).sample(uv()),
                    ],
                )?;
                let mut sprite = SpriteNodeMaterial::new(color);
                sprite.position = position.clone();
                // `gl_PointSize = size * ( 300.0 / -mvPosition.z )`, in framebuffer pixels.
                sprite.scale = call(
                    "point_pixels",
                    "fn point_pixels(p:vec3<f32>,i:u32)->f32{return clamp(tsl_attribute_1[i]*300.0/(-(u.view*u.model*vec4(p,1.0)).z),1.0,511.0);}",
                    &[Type::Vec3, Type::Uint],
                    Type::Float,
                    &[position, instance_index()],
                )?;
                let mut m = sprite
                    .build_points(
                        r,
                        &[(&attributes, Type::Vec4), (&sizes, Type::Float)],
                        &[(&disc.view, &disc.sampler)],
                    )
                    .await?;
                m.properties.transparent = false;
                let mut g = PlaneGeometry::build(1., 1., 1, 1)?;
                g.instance_count = Some(count as u32);
                let h = s.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(g),
                    Arc::new(Material::Shader(m)),
                )));
                s.get_mut(h)?.frustum_culled = false;
                d.objects.push(h);
                d.size_buffer = Some(sizes);
                let mut g = BufferGeometry::default();
                g.set_attribute(
                    "position",
                    Attribute::F32(crate::attribute::BufferAttribute::new(
                        positions
                            .iter()
                            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
                            .collect(),
                        3,
                        false,
                    )?),
                );
                let mut query = Scene::default();
                let points = query.insert(NodeKind::Points(Points {
                    geometry: Arc::new(g),
                    material: Arc::new(Material::Points(PointsMaterial::default())),
                }));
                d.query = Some((query, points));
            }
            _ => return Err(Error::Invalid("interactive shader example")),
        }
        Ok(d)
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// Runs after the camera aspect is known, matching each original `render()` order.
    pub fn prepare(&mut self, r: &Renderer, s: &mut Scene, c: Object3D, aspect: f64) -> Result<()> {
        let t = self.time;
        match self.id {
            183 => {
                if let NodeKind::Mesh(m) = &mut s.get_mut(self.objects[0])?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0][0] = t as f32;
                }
            }
            184 => {
                if let NodeKind::Mesh(m) = &mut s.get_mut(self.objects[0])?.kind {
                    m.materials[0] = self.materials[self.procedure].clone();
                }
            }
            185 | 186 => {
                // The original advances 0.1 degree per frame; the port uses 60 fps time.
                let theta = (t * 6.).to_radians();
                let radius = if self.id == 185 { 5. } else { 25. };
                if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(c)?.kind {
                    o.left = -25. * aspect;
                    o.right = 25. * aspect;
                }
                s.get_mut(c)?.position =
                    Vector3::new(theta.sin(), theta.sin(), theta.cos()) * radius;
                s.look_at(c, Vector3::ZERO)?;
                s.update()?;
                let hit = self.raycast(s, c, &self.objects)?.map(|(h, _)| h);
                let previous = self.intersected.map(|i| self.objects[i]);
                if hit != previous {
                    if let Some(h) = previous {
                        set_emissive(s, h, Color::BLACK)?;
                    }
                    self.intersected = hit.and_then(|h| self.objects.iter().position(|x| *x == h));
                    if let Some(h) = hit {
                        set_emissive(s, h, Color::from_hex(0xff0000))?;
                    }
                }
            }
            _ => {
                // The original increments 0.0005 / 0.001 radians per frame; use 60 fps time.
                s.get_mut(self.objects[0])?.quaternion =
                    Quaternion::from_euler(glam::EulerRot::XYZ, t * 0.03, t * 0.06, 0.);
                s.update()?;
                let hit = self.raycast_points(s, c)?;
                if let Some(index) = hit {
                    if self.intersected != Some(index) {
                        if let Some(previous) = self.intersected {
                            self.sizes[previous] = PARTICLE_SIZE;
                        }
                        self.intersected = Some(index);
                        self.sizes[index] = PARTICLE_SIZE * 1.25;
                        self.dirty = true;
                    }
                } else if self.intersected.is_some() || self.undefined {
                    if let Some(previous) = self.intersected {
                        self.sizes[previous] = PARTICLE_SIZE;
                    }
                    self.dirty = true;
                    self.intersected = None;
                }
                self.undefined = false;
                if self.dirty
                    && let Some(b) = &self.size_buffer
                {
                    b.write(r, 0, bytemuck::cast_slice(&self.sizes))?;
                    self.dirty = false;
                }
            }
        }
        Ok(())
    }
    fn raycast(
        &self,
        s: &Scene,
        c: Object3D,
        objects: &[Object3D],
    ) -> Result<Option<(Object3D, Option<usize>)>> {
        let (camera, world) = s.camera(c)?;
        let mut ray = Raycaster::default();
        ray.set_from_camera(self.pointer, camera, world)?;
        let hits = ray.intersect_objects(s, objects, false)?;
        Ok(hits.first().map(|h| (h.object, h.index)))
    }
    /// The rendered sprites have no CPU vertices; query a raycast-only Points node.
    fn raycast_points(&mut self, s: &Scene, c: Object3D) -> Result<Option<usize>> {
        let (camera, world) = s.camera(c)?;
        let mut ray = Raycaster::default();
        ray.set_from_camera(self.pointer, camera, world)?;
        let quaternion = s.get(self.objects[0])?.quaternion;
        let (query, points) = self.query.as_mut().ok_or(Error::Invalid("points query"))?;
        query.get_mut(*points)?.quaternion = quaternion;
        query.update()?;
        Ok(ray
            .intersect_object(query, *points, false)?
            .first()
            .and_then(|hit| hit.index))
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new(x, y);
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if self.id != 184 || index != 0 || !(0. ..=2.).contains(&value) {
            return Err(Error::Invalid("procedure parameter"));
        }
        self.procedure = value as usize;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
fn set_emissive(s: &mut Scene, h: Object3D, color: Color) -> Result<()> {
    if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind
        && let Material::Lambert(m) = Arc::make_mut(&mut m.materials[0])
    {
        m.emissive = color;
    }
    Ok(())
}
/// `BufferGeometryUtils.mergeVertices` for position-only geometry: first index occurrence wins.
fn merged_box_vertices(g: &BufferGeometry) -> Result<Vec<Vector3>> {
    let positions = g.positions()?;
    let count = g.index.as_ref().map_or(positions.len(), |i| i.len());
    let mut seen = std::collections::HashMap::new();
    let mut out = vec![];
    for i in 0..count {
        let p = positions[g.vertex_index(i)?];
        let key = [p.x, p.y, p.z].map(|v| (v as f32 as f64 * 1e4 + 0.5).trunc() as i64);
        seen.entry(key).or_insert_with(|| {
            out.push(p);
            out.len() - 1
        });
    }
    Ok(out)
}
fn hsl(h: f64, s: f64, l: f64) -> [f64; 3] {
    // Color.setHSL / hue2rgb.
    let hue = |p: f64, q: f64, t: f64| {
        let t = t.rem_euclid(1.);
        if t < 1. / 6. {
            p + (q - p) * 6. * t
        } else if t < 0.5 {
            q
        } else if t < 2. / 3. {
            p + (q - p) * 6. * (2. / 3. - t)
        } else {
            p
        }
    };
    let h = h.rem_euclid(1.);
    let q = if l <= 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let p = 2. * l - q;
    [hue(p, q, h + 1. / 3.), hue(p, q, h), hue(p, q, h - 1. / 3.)]
}
