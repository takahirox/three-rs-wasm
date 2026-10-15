//! webgl_postprocessing_ssao: 100 instanced Lambert boxes through
//! EffectComposer's RenderPass, SSAOPass (view normals and depth, the
//! 32-sample kernel with the 4 × 4 simplex rotation noise, the 5 × 5 blur and
//! the multiplied composite, or the SSAO, blur, depth and normal outputs) and
//! OutputPass. SSAOPass and SimplexNoise draw from Math.random; the port
//! consumes the fixture's sequence in the same order.
use crate::attribute::BufferAttribute;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::sync::Arc;

/// The fixture's Math.random: a 32-bit LCG seeded with 186.
pub(super) struct Random(pub(super) u32);
impl Random {
    pub(super) fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 as f64 / 4294967296.
    }
}
/// SimplexNoise( r ): the permutation drawn from r.random(), and noise3d.
pub(super) struct Simplex {
    perm: [usize; 512],
}
impl Simplex {
    const GRAD3: [[f64; 3]; 12] = [
        [1., 1., 0.],
        [-1., 1., 0.],
        [1., -1., 0.],
        [-1., -1., 0.],
        [1., 0., 1.],
        [-1., 0., 1.],
        [1., 0., -1.],
        [-1., 0., -1.],
        [0., 1., 1.],
        [0., -1., 1.],
        [0., 1., -1.],
        [0., -1., -1.],
    ];
    pub(super) fn new(random: &mut Random) -> Self {
        Self::from_random(|| random.next())
    }
    /// SimplexNoise( { random } ) over any random source.
    pub(super) fn from_random(mut random: impl FnMut() -> f64) -> Self {
        let p: Vec<usize> = (0..256)
            .map(|_| (random() * 256.).floor() as usize)
            .collect();
        let mut perm = [0; 512];
        for (i, v) in perm.iter_mut().enumerate() {
            *v = p[i & 255];
        }
        Self { perm }
    }
    /// noise( xin, yin ): 2D simplex noise.
    pub(super) fn noise(&self, xin: f64, yin: f64) -> f64 {
        let f2 = 0.5 * (3f64.sqrt() - 1.0);
        let s = (xin + yin) * f2;
        let (i, j) = ((xin + s).floor(), (yin + s).floor());
        let g2 = (3.0 - 3f64.sqrt()) / 6.0;
        let t = (i + j) * g2;
        let (x0, y0) = (xin - (i - t), yin - (j - t));
        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };
        let (x1, y1) = (x0 - i1 as f64 + g2, y0 - j1 as f64 + g2);
        let (x2, y2) = (x0 - 1.0 + 2.0 * g2, y0 - 1.0 + 2.0 * g2);
        let (ii, jj) = ((i as i64 & 255) as usize, (j as i64 & 255) as usize);
        let p = &self.perm;
        let gi = [
            p[ii + p[jj]] % 12,
            p[ii + i1 + p[jj + j1]] % 12,
            p[ii + 1 + p[jj + 1]] % 12,
        ];
        let corner = |t: f64, g: usize, x: f64, y: f64| {
            if t < 0. {
                0.
            } else {
                let t = t * t;
                t * t * (Self::GRAD3[g][0] * x + Self::GRAD3[g][1] * y)
            }
        };
        70.0 * (corner(0.5 - x0 * x0 - y0 * y0, gi[0], x0, y0)
            + corner(0.5 - x1 * x1 - y1 * y1, gi[1], x1, y1)
            + corner(0.5 - x2 * x2 - y2 * y2, gi[2], x2, y2))
    }
    fn noise3d(&self, xin: f64, yin: f64, zin: f64) -> f64 {
        let f3 = 1.0 / 3.0;
        let s = (xin + yin + zin) * f3;
        let (i, j, k) = ((xin + s).floor(), (yin + s).floor(), (zin + s).floor());
        let g3 = 1.0 / 6.0;
        let t = (i + j + k) * g3;
        let (x0, y0, z0) = (xin - (i - t), yin - (j - t), zin - (k - t));
        let (i1, j1, k1, i2, j2, k2) = if x0 >= y0 {
            if y0 >= z0 {
                (1, 0, 0, 1, 1, 0)
            } else if x0 >= z0 {
                (1, 0, 0, 1, 0, 1)
            } else {
                (0, 0, 1, 1, 0, 1)
            }
        } else if y0 < z0 {
            (0, 0, 1, 0, 1, 1)
        } else if x0 < z0 {
            (0, 1, 0, 0, 1, 1)
        } else {
            (0, 1, 0, 1, 1, 0)
        };
        let corners = [
            (x0, y0, z0),
            (
                x0 - i1 as f64 + g3,
                y0 - j1 as f64 + g3,
                z0 - k1 as f64 + g3,
            ),
            (
                x0 - i2 as f64 + 2.0 * g3,
                y0 - j2 as f64 + 2.0 * g3,
                z0 - k2 as f64 + 2.0 * g3,
            ),
            (
                x0 - 1.0 + 3.0 * g3,
                y0 - 1.0 + 3.0 * g3,
                z0 - 1.0 + 3.0 * g3,
            ),
        ];
        let (ii, jj, kk) = (
            (i as i64 & 255) as usize,
            (j as i64 & 255) as usize,
            (k as i64 & 255) as usize,
        );
        let p = &self.perm;
        let offsets = [(0, 0, 0), (i1, j1, k1), (i2, j2, k2), (1, 1, 1)];
        let mut n = 0.;
        for (&(x, y, z), &(a, b, c)) in corners.iter().zip(&offsets) {
            let gi = p[ii + a + p[jj + b + p[kk + c]]] % 12;
            let mut t = 0.6 - x * x - y * y - z * z;
            if t >= 0. {
                t *= t;
                let g = Self::GRAD3[gi];
                n += t * t * (g[0] * x + g[1] * y + g[2] * z);
            }
        }
        32.0 * n
    }
}
pub(super) const FULLSCREEN: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position.xy,0.0,1.0);return out;}";
/// A render target texel at the GL screen position uv (rows counted from
/// the bottom; the texture is stored from its top row), clamped.
pub(super) const TEXEL: &str = "fn gl_texel(size:vec2<u32>,uv:vec2<f32>)->vec2<i32>{let s=vec2<f32>(size);let p=clamp(floor(uv*s),vec2(0.0),s-1.0);return vec2<i32>(i32(p.x),i32(s.y-1.0-p.y));}";
/// packing.glsl's depth conversions.
pub(super) const PACKING: &str = "fn perspective_depth_to_view_z(depth:f32,near:f32,far:f32)->f32{return (near*far)/((far-near)*depth-far);}fn view_z_to_orthographic_depth(view_z:f32,near:f32,far:f32)->f32{return (view_z+near)/(near-far);}";
/// SSAOShader: u.custom[0..4] cameraProjectionMatrix, [4..8] its inverse,
/// [8] (resolution, cameraNear, cameraFar), [9] (kernelRadius, minDistance,
/// maxDistance). Textures: the view normals, the depth, the rotation noise.
fn ssao_source(kernel: &[[f32; 3]]) -> String {
    let kernel = kernel
        .iter()
        .map(|k| format!("vec3<f32>({:?},{:?},{:?})", k[0], k[1], k[2]))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{TEXEL}{PACKING}fn ssao()->vec4<f32>{{const KERNEL=array<vec3<f32>,32>({kernel});let uv=fragment_surface.uv;let near=u.custom[8].z;let far=u.custom[8].w;let projection=mat4x4<f32>(u.custom[0],u.custom[1],u.custom[2],u.custom[3]);let inverse=mat4x4<f32>(u.custom[4],u.custom[5],u.custom[6],u.custom[7]);let depth=textureLoad(tsl_texture_1,gl_texel(textureDimensions(tsl_texture_1),uv),0);if depth==1.0 {{return vec4(1.0);}}let view_z=perspective_depth_to_view_z(depth,near,far);let clip_w=projection[2][3]*view_z+projection[3][3];let clip=vec4((vec3(uv,depth)-0.5)*2.0,1.0)*clip_w;let view_position=(inverse*clip).xyz;let view_normal=2.0*textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),uv),0).xyz-1.0;let pixel=vec2<i32>(floor(uv*u.custom[8].xy));let random=vec3(textureLoad(tsl_texture_2,vec2(pixel.x%4,pixel.y%4),0).r);let tangent=normalize(random-view_normal*dot(random,view_normal));let bitangent=cross(view_normal,tangent);let kernel_matrix=mat3x3<f32>(tangent,bitangent,view_normal);var occlusion=0.0;for(var i=0;i<32;i++){{let sample_vector=kernel_matrix*KERNEL[i];let sample_point=view_position+sample_vector*u.custom[9].x;var ndc=projection*vec4(sample_point,1.0);ndc/=ndc.w;let sample_uv=ndc.xy*0.5+0.5;let fragment_z=textureLoad(tsl_texture_1,gl_texel(textureDimensions(tsl_texture_1),sample_uv),0);let real_depth=view_z_to_orthographic_depth(perspective_depth_to_view_z(fragment_z,near,far),near,far);let sample_depth=view_z_to_orthographic_depth(sample_point.z,near,far);let delta=sample_depth-real_depth;if delta>u.custom[9].y && delta<u.custom[9].z {{occlusion+=1.0;}}}}occlusion=clamp(occlusion/32.0,0.0,1.0);return vec4(vec3(1.0-occlusion),1.0);}}"
    )
}
/// SSAOBlurShader: the 5 × 5 mean of texel centers.
const BLUR: &str = "fn ssao_blur()->vec4<f32>{let size=vec2<i32>(textureDimensions(tsl_texture_0));let center=gl_texel(vec2<u32>(size),fragment_surface.uv);var result=0.0;for(var i=-2;i<=2;i++){for(var j=-2;j<=2;j++){let p=clamp(center+vec2(i,-j),vec2(0),size-1);result+=textureLoad(tsl_texture_0,p,0).r;}}return vec4(vec3(result/25.0),1.0);}";
/// SSAODepthShader: 1 − linear depth; u.custom[8].zw is (near, far).
const DEPTH: &str = "fn ssao_depth()->vec4<f32>{let d=textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv),0);let depth=view_z_to_orthographic_depth(perspective_depth_to_view_z(d,u.custom[8].z,u.custom[8].w),u.custom[8].z,u.custom[8].w);return vec4(vec3(1.0-depth),1.0);}";
/// CopyShader (and OutputPass's read): the texel under the fragment.
pub(super) const COPY: &str = "fn ssao_copy()->vec4<f32>{return textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv),0);}";
/// MeshNormalMaterial for the override render: the view normal packed.
pub(super) const NORMAL: &str =
    "fn ssao_normal()->vec4<f32>{return vec4(normalize(fragment_surface.normal)*0.5+0.5,1.0);}";
/// One full-screen pass: its own scene, camera and quad.
pub(super) struct Pass {
    pub(super) scene: Scene,
    camera: Object3D,
    quad: Object3D,
}
impl Pass {
    pub(super) fn new(material: ShaderMaterial) -> Result<Self> {
        let mut scene = Scene::new();
        scene.background = Color::BLACK;
        let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.,
            right: 1.,
            top: 1.,
            bottom: -1.,
            near: 0.,
            far: 1.,
            zoom: 1.,
            ..Default::default()
        })));
        let quad = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(fullscreen_triangle()?),
            Arc::new(Material::Shader(material)),
        )));
        scene.get_mut(quad)?.frustum_culled = false;
        Ok(Self {
            scene,
            camera,
            quad,
        })
    }
    pub(super) fn material(&mut self) -> Result<&mut ShaderMaterial> {
        match &mut self.scene.get_mut(self.quad)?.kind {
            NodeKind::Mesh(m) => match Arc::make_mut(&mut m.materials[0]) {
                Material::Shader(m) => Ok(m),
                _ => Err(Error::Invalid("ssao pass material")),
            },
            _ => Err(Error::Invalid("ssao pass quad")),
        }
    }
    pub(super) fn render(
        &mut self,
        r: &Renderer,
        target: &RenderTarget,
        uniforms: &[[f32; 4]; 16],
    ) -> Result<()> {
        self.material()?.uniforms = *uniforms;
        r.render(&mut self.scene, self.camera, target)
    }
}
pub(super) async fn pass(
    r: &Renderer,
    name: &str,
    source: &str,
    textures: &[(
        &wgpu::TextureView,
        &wgpu::Sampler,
        Type,
        wgpu::TextureSampleType,
    )],
) -> Result<Pass> {
    let node = WgslFn::new(name, source, &[], Type::Vec4)?.call(&[]);
    let types: Vec<Type> = textures.iter().map(|t| t.2).collect();
    let sample_types: Vec<_> = textures.iter().map(|t| t.3).collect();
    let bindings: Vec<_> = textures.iter().map(|t| (t.0, t.1)).collect();
    let mut m = ShaderMaterial::new(Arc::new(
        ShaderProgram::with_projection_and_sample_types(
            r,
            &NodeMaterial::new(node).wgsl_with_texture_types(&types, &[])?,
            &bindings,
            &vec![wgpu::TextureViewDimension::D2; textures.len()],
            &sample_types,
            FULLSCREEN,
        )
        .await?,
    ));
    m.properties.depth_test = false;
    m.properties.depth_write = false;
    Pass::new(m)
}
/// The composer's targets: the read buffer (RenderPass), the normal and
/// depth target, and the SSAO and blur targets.
struct Targets {
    read: RenderTarget,
    normal: RenderTarget,
    ssao: RenderTarget,
    blur: RenderTarget,
}
/// FullScreenQuad's geometry: one triangle over the viewport, uv 0..2.
pub(super) fn fullscreen_triangle() -> Result<BufferGeometry> {
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(
            vec![-1., 3., 0., -1., -1., 0., 3., -1., 0.],
            3,
            false,
        )?),
    );
    g.set_attribute(
        "uv",
        Attribute::F32(BufferAttribute::new(
            vec![0., 2., 0., 0., 2., 0.],
            2,
            false,
        )?),
    );
    g.set_attribute(
        "normal",
        Attribute::F32(BufferAttribute::new(
            vec![0., 0., 1., 0., 0., 1., 0., 0., 1.],
            3,
            false,
        )?),
    );
    Ok(g)
}
pub(super) struct Demo {
    group: Object3D,
    normal_scene: Scene,
    normal_group: Object3D,
    normal_camera: Object3D,
    time: f64,
    targets: Option<Targets>,
    ssao: Pass,
    blur: Pass,
    depth: Pass,
    /// CopyShader from the SSAO, blur (replace or multiply) and normal targets.
    copies: [Pass; 4],
    output: Pass,
    /// The 1 × 1 targets the passes are created against, the noise texture
    /// and the samplers.
    noise: wgpu::Texture,
    nearest: wgpu::Sampler,
    comparison: wgpu::Sampler,
    /// output, kernelRadius, minDistance, maxDistance, enabled.
    params: [f64; 5],
}
pub(super) fn target(r: &Renderer, w: u32, h: u32, depth: bool) -> Result<RenderTarget> {
    RenderTarget::with_options(
        &r.device,
        w,
        h,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba16Float,
            depth_buffer: depth,
            ..Default::default()
        },
    )
}
pub(super) fn half() -> wgpu::TextureSampleType {
    wgpu::TextureSampleType::Float { filterable: true }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let camera = Camera::Perspective(PerspectiveCamera {
            fov: 65.,
            near: 100.,
            far: 700.,
            aspect,
            ..Default::default()
        });
        s.get_mut(c)?.kind = NodeKind::Camera(camera.clone());
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 500.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::from_hex(0xaaaaaa);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 4.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0., 1., 0.);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.,
        }));
        let mut random = Random(186);
        let mut instances = vec![];
        for _ in 0..100 {
            let position = Vector3::new(
                random.next() * 400. - 200.,
                random.next() * 400. - 200.,
                random.next() * 400. - 200.,
            );
            let rotation = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                random.next(),
                random.next(),
                random.next(),
            );
            let scale = random.next() * 10. + 2.;
            let hex = (random.next() * 0xffffff as f64).floor() as u32;
            instances.push(Instance {
                matrix: Matrix4::from_scale_rotation_translation(
                    Vector3::splat(scale),
                    rotation,
                    position,
                ),
                color: Color::from_hex(hex),
            });
        }
        let geometry = Arc::new(BoxGeometry::build(10., 10., 10.)?);
        let group = s.insert(NodeKind::Group);
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(Material::Lambert(MeshLambertMaterial::default())),
        )));
        let n = s.get_mut(mesh)?;
        n.instances = instances.clone();
        n.frustum_culled = false;
        s.add(group, mesh)?;
        // SSAOPass( scene, camera ): the kernel, then SimplexNoise and the
        // 4 × 4 rotation noise.
        let kernel: Vec<[f32; 3]> = (0..32)
            .map(|i| {
                let v = Vector3::new(
                    random.next() * 2. - 1.,
                    random.next() * 2. - 1.,
                    random.next(),
                )
                .normalize();
                let scale = i as f64 / 32.;
                let scale = 0.1 + (1. - 0.1) * (scale * scale);
                (v * scale).as_vec3().to_array()
            })
            .collect();
        let simplex = Simplex::new(&mut random);
        let noise_data: Vec<f32> = (0..16)
            .map(|_| {
                let x = random.next() * 2. - 1.;
                let y = random.next() * 2. - 1.;
                simplex.noise3d(x, y, 0.) as f32
            })
            .collect();
        let size = wgpu::Extent3d {
            width: 4,
            height: 4,
            depth_or_array_layers: 1,
        };
        let noise = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ssao rotation noise"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let bytes: Vec<u8> = noise_data.iter().flat_map(|v| v.to_le_bytes()).collect();
        r.queue.write_texture(
            noise.as_image_copy(),
            &bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(16),
                rows_per_image: Some(4),
            },
            size,
        );
        // The override render: the same instances through MeshNormalMaterial.
        // Its 0x7777ff clear is replaced by the scene's Color background, which
        // WebGLBackground always clears with.
        let mut normal_scene = Scene::new();
        normal_scene.background = Color::from_hex(0xaaaaaa);
        let normal_camera = normal_scene.insert(NodeKind::Camera(camera));
        normal_scene.get_mut(normal_camera)?.position = Vector3::new(0., 0., 500.);
        let node = WgslFn::new("ssao_normal", NORMAL, &[], Type::Vec4)?.call(&[]);
        let normal_material = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(node).wgsl(0)?,
                &[],
                &[],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
            )
            .await?,
        ));
        let normal_group = normal_scene.insert(NodeKind::Group);
        let normal_mesh = normal_scene.insert(NodeKind::Mesh(Mesh::new(
            geometry,
            Arc::new(Material::Shader(normal_material)),
        )));
        let n = normal_scene.get_mut(normal_mesh)?;
        n.instances = instances;
        n.frustum_culled = false;
        normal_scene.add(normal_group, normal_mesh)?;
        let nearest = r.device.create_sampler(&wgpu::SamplerDescriptor::default());
        let comparison = r.device.create_sampler(&wgpu::SamplerDescriptor {
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let initial = target(r, 1, 1, true)?;
        let color = initial.texture.create_view(&Default::default());
        let depth_view = initial
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("ssao depth view"))?;
        let noise_view = noise.create_view(&Default::default());
        let float = wgpu::TextureSampleType::Float { filterable: false };
        let ssao = pass(
            r,
            "ssao",
            &ssao_source(&kernel),
            &[
                (&color, &nearest, Type::Texture, half()),
                (
                    depth_view,
                    &comparison,
                    Type::DepthTexture,
                    wgpu::TextureSampleType::Depth,
                ),
                (&noise_view, &nearest, Type::Texture, float),
            ],
        )
        .await?;
        let blur = pass(
            r,
            "ssao_blur",
            &format!("{TEXEL}{BLUR}"),
            &[(&color, &nearest, Type::Texture, half())],
        )
        .await?;
        let depth = pass(
            r,
            "ssao_depth",
            &format!("{TEXEL}{PACKING}{DEPTH}"),
            &[(
                depth_view,
                &comparison,
                Type::DepthTexture,
                wgpu::TextureSampleType::Depth,
            )],
        )
        .await?;
        let mut copies = vec![];
        for multiply in [false, false, true, false] {
            let mut copy = pass(
                r,
                "ssao_copy",
                &format!("{TEXEL}{COPY}"),
                &[(&color, &nearest, Type::Texture, half())],
            )
            .await?;
            if multiply {
                // Default: CustomBlending ( DstColor, Zero ), alpha ( DstAlpha, Zero ).
                let m = copy.material()?;
                m.properties.transparent = true;
                m.properties.blending = Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::Dst,
                        dst_factor: wgpu::BlendFactor::Zero,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::DstAlpha,
                        dst_factor: wgpu::BlendFactor::Zero,
                        operation: wgpu::BlendOperation::Add,
                    },
                });
            }
            copies.push(copy);
        }
        let output = pass(
            r,
            "ssao_copy",
            &format!("{TEXEL}{COPY}"),
            &[(&color, &nearest, Type::Texture, half())],
        )
        .await?;
        Ok(Self {
            group,
            normal_scene,
            normal_group,
            normal_camera,
            time: 0.,
            targets: None,
            ssao,
            blur,
            depth,
            copies: copies
                .try_into()
                .map_err(|_| Error::Invalid("ssao copies"))?,
            output,
            noise,
            nearest,
            comparison,
            params: [0., 8., 0.005, 0.1, 1.],
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// render(): the group turns with performance.now().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let timer = self.time * 1000.;
        let q = Quaternion::from_euler(glam::EulerRot::XYZ, timer * 0.0002, timer * 0.0001, 0.);
        s.get_mut(self.group)?.quaternion = q;
        self.normal_scene.get_mut(self.normal_group)?.quaternion = q;
        // The override render uses the composer camera.
        let kind = s.get(c)?.kind.clone();
        self.normal_scene.get_mut(self.normal_camera)?.kind = kind;
        Ok(())
    }
    /// Rebind every pass to the targets at the output size.
    fn resize(&mut self, r: &Renderer, w: u32, h: u32) -> Result<()> {
        let targets = Targets {
            read: target(r, w, h, true)?,
            normal: target(r, w, h, true)?,
            ssao: target(r, w, h, false)?,
            blur: target(r, w, h, false)?,
        };
        let view = |t: &RenderTarget| t.texture.create_view(&Default::default());
        let depth_view = targets
            .normal
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("ssao depth view"))?;
        let noise = self.noise.create_view(&Default::default());
        let rebind =
            |p: &mut Pass, textures: &[(&wgpu::TextureView, &wgpu::Sampler)]| -> Result<()> {
                Arc::make_mut(&mut p.material()?.program).rebind(r, &[], textures)
            };
        let (normal, ssao, blur, read) = (
            view(&targets.normal),
            view(&targets.ssao),
            view(&targets.blur),
            view(&targets.read),
        );
        rebind(
            &mut self.ssao,
            &[
                (&normal, &self.nearest),
                (depth_view, &self.comparison),
                (&noise, &self.nearest),
            ],
        )?;
        rebind(&mut self.blur, &[(&ssao, &self.nearest)])?;
        rebind(&mut self.depth, &[(depth_view, &self.comparison)])?;
        rebind(&mut self.output, &[(&read, &self.nearest)])?;
        for (copy, source) in self.copies.iter_mut().zip([&ssao, &blur, &blur, &normal]) {
            rebind(copy, &[(source, &self.nearest)])?;
        }
        self.targets = Some(targets);
        Ok(())
    }
    /// composer.render(): RenderPass, SSAOPass (when enabled), OutputPass.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .targets
            .as_ref()
            .is_none_or(|t| t.read.width != out.width || t.read.height != out.height)
        {
            self.resize(r, out.width, out.height)?;
        }
        let Some(mut targets) = self.targets.take() else {
            return Err(Error::Invalid("ssao targets"));
        };
        let result = self.passes(r, s, c, out, &mut targets);
        self.targets = Some(targets);
        result?;
        Ok(true)
    }
    fn passes(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        t: &mut Targets,
    ) -> Result<()> {
        r.render(s, c, &t.read)?;
        let [output, kernel_radius, min_distance, max_distance, enabled] = self.params;
        if enabled > 0.5 {
            r.render(&mut self.normal_scene, self.normal_camera, &t.normal)?;
            let (near, far, projection) = match s.camera(c)?.0 {
                Camera::Perspective(p) => (
                    p.near,
                    p.far,
                    super::controls_attributes::webgl_perspective(p.fov, p.aspect, p.near, p.far),
                ),
                _ => return Err(Error::Invalid("ssao camera")),
            };
            let mut uniforms = [[0f32; 4]; 16];
            let columns = |m: Matrix4| m.to_cols_array_2d().map(|c| c.map(|v| v as f32));
            uniforms[0..4].copy_from_slice(&columns(projection));
            uniforms[4..8].copy_from_slice(&columns(projection.inverse()));
            uniforms[8] = [out.width as f32, out.height as f32, near as f32, far as f32];
            uniforms[9] = [
                kernel_radius as f32,
                min_distance as f32,
                max_distance as f32,
                0.,
            ];
            self.ssao.render(r, &t.ssao, &uniforms)?;
            self.blur.render(r, &t.blur, &uniforms)?;
            // The output into the read buffer, drawn over it without a clear:
            // Default multiplies the blurred occlusion in; the others replace it.
            t.read.set_load_color(true);
            let result = match output.round() as u32 {
                1 => self.copies[0].render(r, &t.read, &uniforms),
                2 => self.copies[1].render(r, &t.read, &uniforms),
                3 => self.depth.render(r, &t.read, &uniforms),
                4 => self.copies[3].render(r, &t.read, &uniforms),
                _ => self.copies[2].render(r, &t.read, &uniforms),
            };
            t.read.set_load_color(false);
            result?;
        }
        self.output.render(r, out, &[[0.; 4]; 16])
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    /// The GUI: output, kernelRadius, minDistance, maxDistance and enabled.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let p = self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("ssao parameter"))?;
        *p = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
