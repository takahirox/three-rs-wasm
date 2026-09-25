//! Composer background passes, the lava shader with BloomPass, uniform-buffer
//! shaded meshes, the RGB halftone pass and decals projected onto the head.
use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use crate::postprocessing::Effect;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn, float, uv, vec4};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
const PROJECT: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn euler(x: f64, y: f64, z: f64) -> Quaternion {
    Euler {
        angles: Vector3::new(x, y, z),
        order: EulerOrder::XYZ,
    }
    .quaternion()
}
async fn texture(path: &str, srgb: bool) -> Result<Texture> {
    let mut t = decode_texture_image(&fetch(&format!("{ASSETS}/{path}")).await?).await?;
    t.srgb = srgb;
    t.mipmap_filter = Some(Filter::Linear);
    Ok(t)
}
fn half_target(r: &Renderer, depth: bool) -> Result<RenderTarget> {
    RenderTarget::with_options(
        &r.device,
        1,
        1,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba16Float,
            depth_buffer: depth,
            ..Default::default()
        },
    )
}
/// OutputPass: no tone mapping, then the sRGB transfer, written to the canvas.
const OUTPUT: &str = "fn effect(uv:vec2<f32>)->vec4<f32>{let c=textureSample(input_texture,input_sampler,uv);let s=select(c.rgb*12.92,pow(c.rgb,vec3(0.41666))*1.055-vec3(0.055),c.rgb>vec3(0.0031308));return vec4(s,c.a);}";

/// webgl_postprocessing_backgrounds: the composer's clear, texture and cube
/// passes before the render pass.
struct Backgrounds {
    /// The composer's read and write buffers: OutputPass swaps them every frame,
    /// so a disabled ClearPass shows the buffer drawn two frames earlier.
    targets: [RenderTarget; 2],
    current: usize,
    dummy: RenderTarget,
    clear: Effect,
    texture: Effect,
    cube_scene: Scene,
    cube_camera: Object3D,
    cube: Object3D,
    output: Effect,
    /// clearPass, clearColor, clearAlpha, texturePass, texturePassOpacity,
    /// cubeTexturePass, cubeTexturePassOpacity, renderPass.
    params: [f64; 8],
}
/// webgl_shader_lava: the torus, its uniforms and the BloomPass.
struct Lava {
    torus: Object3D,
    time: f64,
    rotation: [f64; 2],
    targets: [RenderTarget; 3],
    blur_x: Effect,
    blur_y: Effect,
    combine: Effect,
    output: Effect,
}
/// webgl_postprocessing_rgb_halftone: the rotating group and the pass.
struct Halftone {
    group: Object3D,
    rotation: f64,
    target: RenderTarget,
    pass: Effect,
    /// shape, radius, rotateR, rotateG, rotateB (degrees), scatter, greyscale,
    /// blending, blendingMode, disable.
    params: [f64; 10],
}
/// webgl_decals: the head, the helper line, the decals and their state.
struct Decals {
    head: Object3D,
    triangles: Vec<[Vector3; 3]>,
    line: Object3D,
    helper_rotation: Matrix3,
    intersection: Option<(Vector3, Vector3)>,
    decals: Vec<Object3D>,
    material: MeshPhongMaterial,
    moved: bool,
    shoot: Option<(f64, f64)>,
    pointer: Option<(f64, f64)>,
    /// minScale, maxScale, rotate.
    params: [f64; 3],
    clear: bool,
}
pub struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    controls: Option<Controls>,
    backgrounds: Option<Backgrounds>,
    lava: Option<Lava>,
    ubo: Vec<Object3D>,
    halftone: Option<Halftone>,
    decals: Option<Decals>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            288 => (65., 1., 10., Vector3::new(0., 0., 7.)),
            289 => (35., 1., 3000., Vector3::new(0., 0., 4.)),
            290 => (45., 0.1, 100., Vector3::new(0., 0., 25.)),
            291 => (75., 1., 1000., Vector3::new(0., 0., 12.)),
            _ => (45., 1., 1000., Vector3::new(0., 0., 120.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = position;
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            controls: None,
            backgrounds: None,
            lava: None,
            ubo: vec![],
            halftone: None,
            decals: None,
        };
        match id {
            288 => d.backgrounds_scene(s, c, r).await?,
            289 => d.lava_scene(s, r).await?,
            290 => d.ubo_scene(s, r).await?,
            291 => d.halftone_scene(s, c, r).await?,
            _ => d.decals_scene(s, c, r).await?,
        }
        Ok(d)
    }
    async fn backgrounds_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        for (hex, position) in [
            (0xefffef, Vector3::new(-10., -10., 10.)),
            (0xffefef, Vector3::new(-10., 10., 10.)),
            (0xefefff, Vector3::new(10., -10., 10.)),
        ] {
            let light = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(hex),
                intensity: 500.,
                distance: 0.,
                decay: 2.,
            }));
            s.get_mut(light)?.position = position;
        }
        let roughness = 0.5 * random(&mut self.seed) + 0.25;
        let mut material = MeshStandardMaterial {
            roughness,
            metalness: 0.,
            ..Default::default()
        };
        material.properties.color = Color::from_hsl(random(&mut self.seed), 1., 0.3);
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(1., 48, 24)?),
            Arc::new(Material::Standard(material)),
        )));
        let format = wgpu::TextureFormat::Rgba16Float;
        // ClearPass: the clear color and alpha fill the read buffer.
        let clear = Effect::new(
            r,
            format,
            "fn effect(uv:vec2<f32>)->vec4<f32>{return params[0];}",
        )
        .await?;
        // TexturePass: CopyShader ( opacity × texel ) with premultiplied blending.
        let wood = r.upload_texture(&Arc::new(texture("hardwood2_diffuse.jpg", true).await?))?;
        let premultiplied = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        };
        let texture_pass = Effect::with_blend_and_textures(
            r,
            format,
            "@group(1) @binding(0) var wood_texture:texture_2d<f32>;@group(1) @binding(1) var wood_sampler:sampler;
fn effect(uv:vec2<f32>)->vec4<f32>{return params[0].x*textureSample(wood_texture,wood_sampler,uv);}",
            premultiplied,
            &[(&wood.view, &wood.sampler)],
        )
        .await?;
        // CubeTexturePass: a 10-unit back-sided box around a camera that copies the
        // view camera's projection and rotation.
        // CubeTextureLoader loads sRGB faces; the cube shader flips x and scales
        // the alpha by the opacity.
        let faces = super::lights_probes::pisa_faces().await?;
        let cube_texture = crate::texture_gpu::GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        let mut cube_scene = Scene::new();
        cube_scene.background = Color::BLACK;
        let graph = NodeMaterial::new(
            WgslFn::new(
                "cube_color",
                "fn cube_color(d:vec3<f32>,opacity:f32)->vec4<f32>{let c=textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz));return vec4(c.rgb,c.a*opacity);}",
                &[Type::Vec3, Type::Float],
                Type::Vec4,
            )?
            .call(&[crate::tsl::position_local(), crate::tsl::uniform(0, Type::Vec4).x()]),
        );
        let source = graph.wgsl_with_texture_types(&[Type::TextureCube], &[])?;
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection_and_dimensions(
                r,
                &source,
                &[(&cube_texture.view, &cube_texture.sampler)],
                &[wgpu::TextureViewDimension::Cube],
                PROJECT,
            )
            .await?,
        ));
        m.uniforms[0] = [1., 0., 0., 0.];
        m.properties.side = Side::Back;
        m.properties.depth_write = false;
        m.properties.depth_test = false;
        let cube = cube_scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(10., 10., 10.)?),
            Arc::new(Material::Shader(m)),
        )));
        let cube_camera = cube_scene.insert(NodeKind::Camera(Camera::Perspective(
            PerspectiveCamera::default(),
        )));
        let output = Effect::new(r, wgpu::TextureFormat::Rgba8Unorm, OUTPUT).await?;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.backgrounds = Some(Backgrounds {
            targets: [half_target(r, true)?, half_target(r, true)?],
            current: 0,
            dummy: half_target(r, false)?,
            clear,
            texture: texture_pass,
            cube_scene,
            cube_camera,
            cube,
            output,
            params: [1., 1., 1., 1., 1., 1., 1., 1.],
        });
        Ok(())
    }
    async fn lava_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let mut cloud = texture("passes-decals/cloud.png", false).await?;
        cloud.wrap_s = Wrapping::Repeat;
        cloud.wrap_t = Wrapping::Repeat;
        // lavatile.jpg carries an Adobe RGB profile that the reference WebGL upload
        // does not apply: decode it without color management.
        let mut lava = super::gltf_viewer::decode_image(
            &fetch(&format!("{ASSETS}/passes-decals/lavatile.jpg")).await?,
        )
        .await?;
        lava.srgb = true;
        lava.mipmap_filter = Some(Filter::Linear);
        lava.wrap_s = Wrapping::Repeat;
        lava.wrap_t = Wrapping::Repeat;
        let cloud = r.upload_texture(&Arc::new(cloud))?;
        let lava = r.upload_texture(&Arc::new(lava))?;
        // The lava fragment shader. TextureLoader flips its images: v samples 1 − v.
        // The fog depth is gl_FragCoord.z / gl_FragCoord.w: the GL window depth
        // times w, from the camera's near 1 and far 3000.
        let color = WgslFn::new(
            "lava_color",
            "fn lava_color(uv_in:vec2<f32>,time:f32)->vec4<f32>{
 let flip=vec2(1.0,-1.0);let lift=vec2(0.0,1.0);
 let noise=textureSample(tsl_texture_0,tsl_sampler_0,uv_in*flip+lift);
 var t1=uv_in+vec2(1.5,-1.5)*time*0.02;var t2=uv_in+vec2(-0.5,2.0)*time*0.01;
 t1.x+=noise.x*2.0;t1.y+=noise.y*2.0;t2.x-=noise.y*0.2;t2.y+=noise.z*0.2;
 let p=textureSample(tsl_texture_0,tsl_sampler_0,(t1*2.0)*flip+lift).a;
 let color=textureSample(tsl_texture_1,tsl_sampler_1,(t2*2.0)*flip+lift);
 var temp=color*(vec4(p)*2.0)+(color*color-0.1);
 if temp.r>1.0 {let a=clamp(temp.r-2.0,0.0,100.0);temp.b+=a;temp.g+=a;}
 if temp.g>1.0 {temp.r+=temp.g-1.0;temp.b+=temp.g-1.0;}
 if temp.b>1.0 {temp.r+=temp.b-1.0;temp.g+=temp.b-1.0;}
 let d=-fragment_surface.view_position.z;let n=1.0;let f=3000.0;
 let depth=0.5*((f+n)/(f-n)*d-2.0*f*n/(f-n)+d);
 let density=0.45;let fog=1.0-clamp(exp2(-density*density*depth*depth*1.442695),0.0,1.0);
 return mix(temp,vec4(vec3(0.0),temp.w),fog);
}",
            &[Type::Vec2, Type::Float],
            Type::Vec4,
        )?
        .call(&[
            uv() * crate::tsl::vec2(float(3.), float(1.)),
            crate::tsl::uniform(0, Type::Vec4).x(),
        ]);
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(2)?,
            &[],
            &[(&cloud.view, &cloud.sampler), (&lava.view, &lava.sampler)],
            PROJECT,
        )
        .await?;
        let torus = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusGeometry::build(
                0.65,
                0.3,
                30,
                30,
                std::f64::consts::TAU,
                0.,
                std::f64::consts::TAU,
            )?),
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        s.get_mut(torus)?.quaternion = euler(0.3, 0., 0.);
        // BloomPass( 1.25 ): a 25-tap Gaussian (sigma 4) in steps of 1/512 of the
        // texture, horizontally then vertically, added back at 1.25.
        let kernel = {
            let half = 12.;
            let mut values: Vec<f64> = (0..25)
                .map(|i| (-(i as f64 - half).powi(2) / 32.).exp())
                .collect();
            let sum: f64 = values.iter().sum();
            values.iter_mut().for_each(|v| *v /= sum);
            values
        };
        let kernel_wgsl = kernel
            .iter()
            .map(|v| format!("{:?}", *v as f32))
            .collect::<Vec<_>>()
            .join(",");
        let blur = |dx: f32, dy: f32| {
            format!(
                "fn effect(uv:vec2<f32>)->vec4<f32>{{let k=array<f32,25>({kernel_wgsl});let step=vec2({dx:?},{dy:?});var coord=uv-12.0*step;var sum=vec4(0.0);for(var i=0;i<25;i++){{sum+=textureSample(input_texture,input_sampler,coord)*k[i];coord+=step;}}return sum;}}"
            )
        };
        let format = wgpu::TextureFormat::Rgba16Float;
        let blur_x = Effect::new(r, format, &blur(0.001953125, 0.)).await?;
        // Effect UVs grow downward: the vertical step is negated.
        let blur_y = Effect::new(r, format, &blur(0., -0.001953125)).await?;
        let additive = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        };
        let combine = Effect::with_blend(
            r,
            format,
            "fn effect(uv:vec2<f32>)->vec4<f32>{return 1.25*textureSample(input_texture,input_sampler,uv);}",
            additive,
        )
        .await?;
        let output = Effect::new(r, wgpu::TextureFormat::Rgba8Unorm, OUTPUT).await?;
        self.lava = Some(Lava {
            torus,
            time: 1.,
            rotation: [0.3, 0.],
            targets: [
                half_target(r, true)?,
                half_target(r, false)?,
                half_target(r, false)?,
            ],
            blur_x,
            blur_y,
            combine,
            output,
        });
        Ok(())
    }
    /// webgl_ubo: 200 meshes whose raw shaders share the ViewData and LightingData
    /// blocks: Phong lighting in eye space, the light at ( 0, 0, 10 ) in eye space.
    async fn ubo_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let crate_texture =
            r.upload_texture(&Arc::new(texture("passes-decals/crate.gif", true).await?))?;
        let lighting = "let l=normalize(vec3(0.0,0.0,10.0)-fragment_surface.view_position);let n=normalize(fragment_surface.normal);let e=-normalize(fragment_surface.view_position);let r=normalize(reflect(-l,n));
 let diffuse=max(dot(n,l),0.0);let specular=pow(max(dot(r,e),0.0),64.0);
 let weighting=vec3(0.2015,0.2015,0.2015)+vec3(0.6654,0.6654,0.6654)*diffuse+vec3(0.7991,0.7991,0.7991)*specular;";
        // LightingData colors ( 0x7c7c7c, 0xd5d5d5, 0xe7e7e7 ) in linear working space.
        let lighting = lighting
            .replace("vec3(0.2015,0.2015,0.2015)", &linear(0x7c7c7c))
            .replace("vec3(0.6654,0.6654,0.6654)", &linear(0xd5d5d5))
            .replace("vec3(0.7991,0.7991,0.7991)", &linear(0xe7e7e7));
        let colored = WgslFn::new(
            "ubo_colored",
            &format!("fn ubo_colored(color:vec4<f32>)->vec4<f32>{{{lighting}return vec4(color.rgb*weighting,1.0);}}"),
            &[Type::Vec4],
            Type::Vec4,
        )?
        .call(&[crate::tsl::uniform(0, Type::Vec4)]);
        let textured = WgslFn::new(
            "ubo_textured",
            &format!("fn ubo_textured(uv:vec2<f32>)->vec4<f32>{{{lighting}return vec4(textureSample(tsl_texture_0,tsl_sampler_0,vec2(uv.x,1.0-uv.y)).rgb*weighting,1.0);}}"),
            &[Type::Vec2],
            Type::Vec4,
        )?
        .call(&[uv()]);
        let program1 = Arc::new(
            ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(colored).wgsl(0)?,
                &[],
                &[],
                PROJECT,
            )
            .await?,
        );
        let program2 = Arc::new(
            ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(textured).wgsl(1)?,
                &[],
                &[(&crate_texture.view, &crate_texture.sampler)],
                PROJECT,
            )
            .await?,
        );
        let tetrahedron = Arc::new(TetrahedronGeometry::build(1., 0)?);
        let cube = Arc::new(BoxGeometry::build(1., 1., 1.)?);
        let textured_material = Arc::new(Material::Shader(ShaderMaterial::new(program2)));
        for i in 0..200 {
            let (geometry, material) = if i % 2 == 0 {
                // new THREE.Color( 0xffffff * Math.random() ): setHex floors the value.
                let hex = (16777215. * random(&mut self.seed)).floor() as u32;
                let c = Color::from_hex(hex).0;
                let mut m = ShaderMaterial::new(program1.clone());
                m.uniforms[0] = [c.x as f32, c.y as f32, c.z as f32, 1.];
                (tetrahedron.clone(), Arc::new(Material::Shader(m)))
            } else {
                (cube.clone(), textured_material.clone())
            };
            let h = s.insert(NodeKind::Mesh(Mesh::new(geometry, material)));
            let scale = 1. + random(&mut self.seed) * 0.5;
            let rotation = [
                random(&mut self.seed) * PI,
                random(&mut self.seed) * PI,
                random(&mut self.seed) * PI,
            ];
            let position = Vector3::new(
                random(&mut self.seed) * 40. - 20.,
                random(&mut self.seed) * 40. - 20.,
                random(&mut self.seed) * 20. - 10.,
            );
            let n = s.get_mut(h)?;
            n.scale = Vector3::splat(scale);
            n.position = position;
            n.user_data.insert("rx".into(), rotation[0].into());
            n.user_data.insert("ry".into(), rotation[1].into());
            n.user_data.insert("rz".into(), rotation[2].into());
            n.quaternion = euler(rotation[0], rotation[1], rotation[2]);
            self.ubo.push(h);
        }
        Ok(())
    }
    async fn halftone_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0x444444);
        let group = s.insert(NodeKind::Group);
        let floor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(100., 1., 100.)?),
            Arc::new(Material::Phong(MeshPhongMaterial::default())),
        )));
        s.get_mut(floor)?.position.y = -10.;
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 250.,
            distance: 0.,
            decay: 2.,
        }));
        s.get_mut(light)?.position.y = 2.;
        s.add(group, floor)?;
        s.add(group, light)?;
        // abs( object normal ) + ( uv, 0 ), alpha 0, written raw to the linear target.
        let color = vec4(
            crate::tsl::normal_local().abs() + crate::tsl::vec3(uv().x(), uv().y(), float(0.)),
            float(0.),
        );
        let program = Arc::new(
            ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(color).wgsl(0)?,
                &[],
                &[],
                PROJECT,
            )
            .await?,
        );
        let material = Arc::new(Material::Shader(ShaderMaterial::new(program)));
        let cube = Arc::new(BoxGeometry::build(2., 2., 2.)?);
        for _ in 0..50 {
            let h = s.insert(NodeKind::Mesh(Mesh::new(cube.clone(), material.clone())));
            let position = Vector3::new(
                random(&mut self.seed) * 16. - 8.,
                random(&mut self.seed) * 16. - 8.,
                random(&mut self.seed) * 16. - 8.,
            );
            let rotation = [
                random(&mut self.seed) * PI * 2.,
                random(&mut self.seed) * PI * 2.,
                random(&mut self.seed) * PI * 2.,
            ];
            let n = s.get_mut(h)?;
            n.position = position;
            n.quaternion = euler(rotation[0], rotation[1], rotation[2]);
            s.add(group, h)?;
        }
        let pass = Effect::new(r, wgpu::TextureFormat::Rgba8Unorm, HALFTONE).await?;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.halftone = Some(Halftone {
            group,
            rotation: 0.,
            target: half_target(r, true)?,
            pass,
            params: [1., 4., 15., 45., 30., 0., 0., 1., 1., 0.],
        });
        Ok(())
    }
    async fn decals_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let _ = r;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x666666),
            intensity: 1.,
        }));
        for (hex, position) in [
            (0xffddcc, Vector3::new(1., 0.75, 0.5)),
            (0xccccff, Vector3::new(-1., 0.75, -0.5)),
        ] {
            let light = s.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(hex),
                intensity: 3.,
                target: Vector3::ZERO,
            }));
            s.get_mut(light)?.position = position;
        }
        // The helper line from p to n is a resident unit segment placed by its
        // transform; the original's zero-length initial line draws nothing.
        let mut line_geometry = BufferGeometry::default();
        line_geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(
                vec![0., 0., 0., 0., 0., 1.],
                3,
                false,
            )?),
        );
        let line = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(line_geometry),
            material: Arc::new(Material::Line(LineBasicMaterial::default())),
            segments: false,
        }));
        let n = s.get_mut(line)?;
        n.visible = false;
        n.frustum_culled = false;
        let (asset, buffers, images) = load_asset("/web/models/LeePerrySmith.glb").await?;
        let meshes = crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        let geometry = match &s.get(meshes[0])?.kind {
            NodeKind::Mesh(m) => m.geometry.clone(),
            _ => return Err(Error::Invalid("LeePerrySmith mesh")),
        };
        for h in meshes {
            s.dispose(h)?;
        }
        let mut head_material = MeshPhongMaterial {
            specular: Color::from_hex(0x111111),
            shininess: 25.,
            specular_map: Some(Arc::new(texture("shadow-rtt/Map-SPEC.jpg", false).await?)),
            normal_map: Some(Arc::new(
                texture(
                    "LeePerrySmith/Infinite-Level_02_Tangent_SmoothUV.jpg",
                    false,
                )
                .await?,
            )),
            ..Default::default()
        };
        head_material.properties.map =
            Some(Arc::new(texture("shadow-rtt/Map-COL.jpg", true).await?));
        let head = s.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(Material::Phong(head_material)),
        )));
        s.get_mut(head)?.scale = Vector3::splat(10.);
        // The raycast works on the object-space triangles with front-face culling.
        let positions = geometry
            .attributes
            .get("position")
            .ok_or(Error::Invalid("positions"))?;
        let mut triangles = vec![];
        for i in (0..geometry.draw_count()).step_by(3) {
            let v =
                |k: usize| -> Result<Vector3> { positions.vector3(geometry.vertex_index(i + k)?) };
            triangles.push([v(0)?, v(1)?, v(2)?]);
        }
        let mut material = MeshPhongMaterial {
            specular: Color::from_hex(0x444444),
            shininess: 30.,
            normal_map: Some(Arc::new(
                texture("passes-decals/decal-normal.jpg", false).await?,
            )),
            ..Default::default()
        };
        material.properties.map = Some(Arc::new(
            texture("passes-decals/decal-diffuse.png", true).await?,
        ));
        material.properties.transparent = true;
        material.properties.depth_write = false;
        material.properties.polygon_offset = Some((-4., 0));
        let mut controls = Controls::new(None, (50., 200.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.decals = Some(Decals {
            head,
            triangles,
            line,
            helper_rotation: Matrix3::IDENTITY,
            intersection: None,
            decals: vec![],
            material,
            moved: false,
            shoot: None,
            pointer: None,
            params: [10., 20., 1.],
            clear: false,
        });
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let delta = t - self.last;
        self.last = t;
        match self.id {
            288 => {
                let k = self
                    .backgrounds
                    .as_mut()
                    .ok_or(Error::Invalid("backgrounds"))?;
                // The cube camera copies the view camera's projection and rotation.
                let (camera, _) = s.camera(c)?;
                let camera = camera.clone();
                let rotation = s.get(c)?.quaternion;
                k.cube_scene.get_mut(k.cube_camera)?.kind = NodeKind::Camera(camera);
                k.cube_scene.get_mut(k.cube_camera)?.quaternion = rotation;
                let opacity = k.params[6];
                if let NodeKind::Mesh(m) = &mut k.cube_scene.get_mut(k.cube)?.kind
                    && let Material::Shader(current) = m.materials[0].as_ref()
                    && (current.uniforms[0][0] as f64 != opacity
                        || current.properties.transparent != (opacity < 1.))
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0] = [opacity as f32, 0., 0., 0.];
                    m.properties.transparent = opacity < 1.;
                }
            }
            289 => {
                // animate(): delta × 5 drives time and the rotation.
                let k = self.lava.as_mut().ok_or(Error::Invalid("lava"))?;
                let d = 5. * delta;
                k.time += 0.2 * d;
                k.rotation[1] += 0.0125 * d;
                k.rotation[0] += 0.05 * d;
                s.get_mut(k.torus)?.quaternion = euler(k.rotation[0], k.rotation[1], 0.);
                if let NodeKind::Mesh(m) = &mut s.get_mut(k.torus)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0] = [k.time as f32, 0., 0., 0.];
                }
            }
            290 => {
                for &h in &self.ubo {
                    let n = s.get_mut(h)?;
                    let mut get = |key: &str, step: f64| {
                        let v = n.user_data.get(key).and_then(|v| v.as_f64()).unwrap_or(0.) + step;
                        n.user_data.insert(key.into(), v.into());
                        v
                    };
                    let (x, y) = (get("rx", delta * 0.5), get("ry", delta * 0.3));
                    let z = get("rz", 0.);
                    n.quaternion = euler(x, y, z);
                }
            }
            291 => {
                let k = self.halftone.as_mut().ok_or(Error::Invalid("halftone"))?;
                k.rotation += delta * PI / 64.;
                s.get_mut(k.group)?.quaternion = Quaternion::from_rotation_y(k.rotation);
            }
            292 => self.decals_step(s, c)?,
            _ => {}
        }
        Ok(())
    }
    /// The pointer handlers: move updates the intersection, the helper and the
    /// line; a click without an orbit change shoots a decal.
    fn decals_step(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let k = self.decals.as_mut().ok_or(Error::Invalid("decals"))?;
        if std::mem::take(&mut k.clear) {
            for d in k.decals.drain(..) {
                s.dispose(d)?;
            }
        }
        let head_world = s.get(k.head)?.matrix_world;
        let pointer = k.pointer.take();
        let shoot = k.shoot.take();
        for (x, y) in pointer.into_iter().chain(shoot) {
            k.intersection = None;
            let (w, h, _) = viewport_css();
            let mouse = Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.);
            let (camera, _) = s.camera(c)?;
            let camera_world = s.get(c)?.matrix_world;
            let unprojected = camera_world.transform_point3(
                camera
                    .projection_matrix()?
                    .inverse()
                    .project_point3(mouse.extend(0.5)),
            );
            let origin = camera_world.w_axis.truncate();
            let ray = Ray {
                origin,
                direction: (unprojected - origin).normalize(),
            };
            let local = ray.transformed(head_world.inverse());
            let mut best: Option<(f64, Vector3, Vector3)> = None;
            for [a, b, cc] in &k.triangles {
                if let Some(hit) = local.intersect_triangle(*a, *b, *cc, true) {
                    let world = head_world.transform_point3(hit);
                    let distance = origin.distance(world);
                    if best.is_none_or(|(d, ..)| distance < d) {
                        best = Some((distance, world, (*cc - *b).cross(*a - *b).normalize()));
                    }
                }
            }
            if let Some((_, p, face_normal)) = best {
                let normal_matrix = Matrix3::from_mat4(head_world).inverse().transpose();
                let n = p + (normal_matrix * face_normal).normalize() * 10.;
                // mouseHelper.lookAt( n ): +z toward n, up +Y.
                let z = (n - p).normalize();
                let mut x = Vector3::Y.cross(z);
                if x.length_squared() == 0. {
                    x = Vector3::X;
                }
                let x = x.normalize();
                k.helper_rotation = Matrix3::from_cols(x, z.cross(x), z);
                let line = s.get_mut(k.line)?;
                line.position = p;
                line.quaternion = Quaternion::from_rotation_arc(Vector3::Z, z);
                line.scale = Vector3::new(1., 1., (n - p).length());
                line.visible = true;
                k.intersection = Some((p, face_normal));
            }
        }
        if shoot.is_some()
            && let Some((position, _)) = k.intersection
        {
            // Euler XYZ from the helper's rotation, as mouseHelper.rotation.
            let m = k.helper_rotation;
            let m13 = m.z_axis.x;
            let y = m13.clamp(-1., 1.).asin();
            let (x, mut z) = if m13.abs() < 0.9999999 {
                (
                    (-m.z_axis.y).atan2(m.z_axis.z),
                    (-m.y_axis.x).atan2(m.x_axis.x),
                )
            } else {
                (m.y_axis.z.atan2(m.y_axis.y), 0.)
            };
            if k.params[2] > 0.5 {
                z = random(&mut self.seed) * 2. * PI;
            }
            let [min, max, _] = k.params;
            let scale = min + random(&mut self.seed) * (max - min);
            let hex = (random(&mut self.seed) * 16777215.).floor() as u32;
            let geometry =
                decal_geometry(s, k.head, position, euler(x, y, z), Vector3::splat(scale))?;
            let mut material = k.material.clone();
            material.properties.color = Color::from_hex(hex);
            let decal = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(geometry),
                Arc::new(Material::Phong(material)),
            )));
            s.get_mut(decal)?.render_order = k.decals.len() as i32;
            k.decals.push(decal);
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        match self.id {
            288 => {
                let k = self
                    .backgrounds
                    .as_mut()
                    .ok_or(Error::Invalid("backgrounds"))?;
                for t in &mut k.targets {
                    if (t.width, t.height) != (out.width, out.height) {
                        t.set_size(&r.device, out.width, out.height)?;
                    }
                }
                let target = &mut k.targets[k.current];
                let p = k.params;
                if p[0] > 0.5 {
                    let hex = [0x000000, 0xffffff, 0x0000ff, 0x00ff00, 0xff0000][p[1] as usize];
                    let color = Color::from_hex(hex).0;
                    k.clear.parameters[0] =
                        [color.x as f32, color.y as f32, color.z as f32, p[2] as f32];
                    k.clear.apply(r, &k.dummy, None, target)?;
                }
                if p[3] > 0.5 {
                    k.texture.parameters[0] = [p[4] as f32, 0., 0., 0.];
                    k.texture.apply_with_load(r, &k.dummy, None, target, true)?;
                }
                target.set_load_color(true);
                if p[5] > 0.5 {
                    r.render(&mut k.cube_scene, k.cube_camera, target)?;
                }
                if p[7] > 0.5 {
                    r.render(s, c, target)?;
                }
                target.set_load_color(false);
                k.output.apply(r, target, None, out)?;
                k.current = 1 - k.current;
                Ok(true)
            }
            289 => {
                let k = self.lava.as_mut().ok_or(Error::Invalid("lava"))?;
                for t in &mut k.targets {
                    if (t.width, t.height) != (out.width, out.height) {
                        t.set_size(&r.device, out.width, out.height)?;
                    }
                }
                let [a, b, c2] = &k.targets;
                r.render(s, c, a)?;
                k.blur_x.apply(r, a, None, b)?;
                k.blur_y.apply(r, b, None, c2)?;
                k.combine.apply_with_load(r, c2, None, a, true)?;
                k.output.apply(r, a, None, out)?;
                Ok(true)
            }
            291 => {
                let k = self.halftone.as_mut().ok_or(Error::Invalid("halftone"))?;
                if (k.target.width, k.target.height) != (out.width, out.height) {
                    k.target.set_size(&r.device, out.width, out.height)?;
                }
                r.render(s, c, &k.target)?;
                let p = k.params;
                let deg = PI / 180.;
                k.pass.parameters[0] = [
                    p[1] as f32,
                    (p[2] * deg) as f32,
                    (p[3] * deg) as f32,
                    (p[4] * deg) as f32,
                ];
                k.pass.parameters[1] = [
                    p[5] as f32,
                    out.width as f32,
                    out.height as f32,
                    p[0] as f32,
                ];
                k.pass.parameters[2] = [p[7] as f32, p[8] as f32, p[6] as f32, p[9] as f32];
                k.pass.apply(r, &k.target, None, out)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
    /// Absolute CSS pointer events (decals): 0 move, 10 + button down, 20 + button up.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(k) = &mut self.decals {
            match kind {
                0 => k.pointer = Some((x, y)),
                10 => k.moved = false,
                20 if !k.moved => k.shoot = Some((x, y)),
                _ => {}
            }
        }
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        // controls 'change' marks the pointer as moved.
        if let Some(k) = &mut self.decals
            && (dx != 0. || dy != 0. || wheel != 0.)
        {
            k.moved = true;
        }
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            if self.id == 288 {
                return Ok(());
            }
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        match self.id {
            288 if index < 8 => {
                self.backgrounds
                    .as_mut()
                    .ok_or(Error::Invalid("backgrounds"))?
                    .params[index] = v
            }
            // The shape and blendingMode menus list the values 1 to 5.
            291 if index < 10 => {
                self.halftone
                    .as_mut()
                    .ok_or(Error::Invalid("halftone"))?
                    .params[index] = if index == 0 || index == 8 { v + 1. } else { v }
            }
            292 if index < 3 => {
                self.decals.as_mut().ok_or(Error::Invalid("decals"))?.params[index] = v
            }
            292 if index == 3 => self.decals.as_mut().ok_or(Error::Invalid("decals"))?.clear = true,
            _ => return Err(Error::Invalid("passes/decals parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// A color hex in the linear working space, as a WGSL vec3.
fn linear(hex: u32) -> String {
    let c = Color::from_hex(hex).0;
    format!("vec3({:?},{:?},{:?})", c.x as f32, c.y as f32, c.z as f32)
}
/// DecalGeometry: the mesh's world-space triangles in the projector's frame,
/// clipped by the six box planes, then back to world space with box UVs.
fn decal_geometry(
    s: &Scene,
    mesh: Object3D,
    position: Vector3,
    orientation: Quaternion,
    size: Vector3,
) -> Result<BufferGeometry> {
    let n = s.get(mesh)?;
    let world = n.matrix_world;
    let geometry = n.geometry().ok_or(Error::Invalid("decal target"))?;
    let normal_matrix = Matrix3::from_mat4(world).inverse().transpose();
    let projector = Matrix4::from_rotation_translation(orientation, position);
    let inverse = projector.inverse();
    let positions = geometry
        .attributes
        .get("position")
        .ok_or(Error::Invalid("positions"))?;
    let normals = geometry.attributes.get("normal");
    let mut vertices: Vec<(Vector3, Option<Vector3>)> = vec![];
    for i in 0..geometry.draw_count() {
        let index = geometry.vertex_index(i)?;
        let p = inverse.transform_point3(world.transform_point3(positions.vector3(index)?));
        let normal = normals
            .map(|a| a.vector3(index))
            .transpose()?
            .map(|v| (normal_matrix * v).normalize());
        vertices.push((p, normal));
    }
    let clip = |v0: &(Vector3, Option<Vector3>),
                v1: &(Vector3, Option<Vector3>),
                plane: Vector3,
                s: f64| {
        let d0 = v0.0.dot(plane) - s;
        let d1 = v1.0.dot(plane) - s;
        let t = d0 / (d0 - d1);
        let position = v0.0 + (v1.0 - v0.0) * t;
        let normal = v0.1.zip(v1.1).map(|(a, b)| a + (b - a) * t);
        (position, normal)
    };
    for plane in [
        Vector3::X,
        Vector3::NEG_X,
        Vector3::Y,
        Vector3::NEG_Y,
        Vector3::Z,
        Vector3::NEG_Z,
    ] {
        let half = 0.5 * size.dot(plane).abs();
        let mut out = vec![];
        for tri in vertices.as_chunks::<3>().0 {
            let d = [
                tri[0].0.dot(plane) - half,
                tri[1].0.dot(plane) - half,
                tri[2].0.dot(plane) - half,
            ];
            let outside = d.map(|v| v > 0.);
            match outside.iter().filter(|&&o| o).count() {
                0 => out.extend_from_slice(tri),
                1 => {
                    if outside[1] {
                        let (v1, v2) = (tri[0], tri[2]);
                        let (v3, v4) = (
                            clip(&tri[1], &v1, plane, half),
                            clip(&tri[1], &v2, plane, half),
                        );
                        out.extend([v3, v2, v1, v2, v3, v4]);
                    } else {
                        let (v1, v2, from) = if outside[0] {
                            (tri[1], tri[2], tri[0])
                        } else {
                            (tri[0], tri[1], tri[2])
                        };
                        let (v3, v4) =
                            (clip(&from, &v1, plane, half), clip(&from, &v2, plane, half));
                        out.extend([v1, v2, v3, v4, v3, v2]);
                    }
                }
                2 => {
                    for (inside, a, b) in [(0, 1, 2), (1, 2, 0), (2, 0, 1)] {
                        if !outside[inside] {
                            let v1 = tri[inside];
                            out.extend([
                                v1,
                                clip(&v1, &tri[a], plane, half),
                                clip(&v1, &tri[b], plane, half),
                            ]);
                        }
                    }
                }
                _ => {}
            }
        }
        vertices = out;
    }
    let (mut p, mut uv, mut normal) = (vec![], vec![], vec![]);
    for (position, n) in &vertices {
        uv.extend([
            (0.5 + position.x / size.x) as f32,
            (0.5 + position.y / size.y) as f32,
        ]);
        let world = projector.transform_point3(*position);
        p.extend([world.x as f32, world.y as f32, world.z as f32]);
        if let Some(n) = n {
            normal.extend([n.x as f32, n.y as f32, n.z as f32]);
        }
    }
    let mut g = BufferGeometry::default();
    g.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(p, 3, false)?),
    );
    g.set_attribute("uv", Attribute::F32(BufferAttribute::new(uv, 2, false)?));
    if !normal.is_empty() {
        g.set_attribute(
            "normal",
            Attribute::F32(BufferAttribute::new(normal, 3, false)?),
        );
    }
    Ok(g)
}
/// HalftoneShader, with GL's bottom-up pixel coordinates. params[0]: radius,
/// rotateR, rotateG, rotateB; params[1]: scatter, width, height, shape;
/// params[2]: blending, blendingMode, greyscale, disable.
const HALFTONE: &str = r#"
fn ht_blend(a:f32,b:f32,t:f32)->f32{return a*(1.0-t)+b*t;}
fn ht_hypot(x:f32,y:f32)->f32{return sqrt(x*x+y*y);}
fn ht_rand(seed:vec2<f32>)->f32{return fract(sin(dot(seed,vec2(12.9898,78.233)))*43758.5453);}
fn ht_tex(point:vec2<f32>)->vec4<f32>{let size=params[1].yz;return textureSampleLevel(input_texture,input_sampler,vec2(point.x/size.x,1.0-point.y/size.y),0.0);}
fn ht_distance(channel:f32,coord:vec2<f32>,normal:vec2<f32>,p:vec2<f32>,angle:f32,rad_max:f32)->f32{
 let shape=i32(params[1].w);var dist=ht_hypot(coord.x-p.x,coord.y-p.y);var rad=channel;
 if shape==1 {rad=pow(abs(rad),1.125)*rad_max;}
 else if shape==2 {rad=pow(abs(rad),1.125)*rad_max;if dist!=0.0 {let dot_p=abs((p.x-coord.x)/dist*normal.x+(p.y-coord.y)/dist*normal.y);dist=(dist*(1.0-0.20710678))+dot_p*dist*0.41421356;}}
 else if shape==3 {rad=pow(abs(rad),1.5)*rad_max;let dot_p=(p.x-coord.x)*normal.x+(p.y-coord.y)*normal.y;dist=ht_hypot(normal.x*dot_p,normal.y*dot_p);}
 else if shape==4 {let theta=atan2(p.y-coord.y,p.x-coord.x)-angle;let sin_t=abs(sin(theta));let cos_t=abs(cos(theta));rad=pow(abs(rad),1.4);rad=rad_max*(rad+select(rad-cos_t*rad,rad-sin_t*rad,sin_t>cos_t));}
 else if shape==5 {let theta=atan2(p.y-coord.y,p.x-coord.x)-angle-3.14159265/4.0;let sin_t=abs(sin(theta));let cos_t=abs(cos(theta));rad=pow(abs(rad),1.4);rad=rad_max*(rad+select(rad-cos_t*rad,rad-sin_t*rad,sin_t>cos_t));}
 return rad-dist;
}
fn ht_sample(point:vec2<f32>)->vec4<f32>{
 var tex=ht_tex(point);let base=ht_rand(floor(point))*6.28318531;let step=6.28318531/8.0;let dist=params[0].x*0.66;
 for(var i=0;i<8;i++){let r=base+step*f32(i);tex+=ht_tex(point+vec2(cos(r)*dist,sin(r)*dist));}
 return tex/9.0;
}
fn ht_mod(x:f32,y:f32)->f32{return x-y*floor(x/y);}
fn ht_dot(p:vec2<f32>,grid_angle:f32,channel:i32,aa:f32)->f32{
 let step=params[0].x;let n=vec2(cos(grid_angle),sin(grid_angle));let threshold=step*0.5;
 let dot_normal=n.x*p.x+n.y*p.y;let dot_line=-n.y*p.x+n.x*p.y;let offset=n*dot_normal;
 let offset_normal=ht_mod(ht_hypot(offset.x,offset.y),step);let normal_dir=select(-1.0,1.0,dot_normal<0.0);
 let normal_scale=select(step-offset_normal,-offset_normal,offset_normal<threshold)*normal_dir;
 let offset_line=ht_mod(ht_hypot(p.x-offset.x,p.y-offset.y),step);let line_dir=select(-1.0,1.0,dot_line<0.0);
 let line_scale=select(step-offset_line,-offset_line,offset_line<threshold)*line_dir;
 var p1=vec2(p.x-n.x*normal_scale+n.y*line_scale,p.y-n.y*normal_scale-n.x*line_scale);
 if params[1].x!=0.0 {let off_mag=params[1].x*threshold*0.5;let off_angle=ht_rand(floor(p1))*6.28318531;p1+=vec2(cos(off_angle),sin(off_angle))*off_mag;}
 let normal_step=normal_dir*select(-step,step,offset_normal<threshold);let line_step=line_dir*select(-step,step,offset_line<threshold);
 let p2=vec2(p1.x-n.x*normal_step,p1.y-n.y*normal_step);let p3=vec2(p1.x+n.y*line_step,p1.y-n.x*line_step);
 let p4=vec2(p1.x-n.x*normal_step+n.y*line_step,p1.y-n.y*normal_step-n.x*line_step);
 let s1=ht_sample(p1)[channel];let s2=ht_sample(p2)[channel];let s3=ht_sample(p3)[channel];let s4=ht_sample(p4)[channel];
 let r=params[0].x;
 let d1=ht_distance(s1,p1,n,p,grid_angle,r);let d2=ht_distance(s2,p2,n,p,grid_angle,r);let d3=ht_distance(s3,p3,n,p,grid_angle,r);let d4=ht_distance(s4,p4,n,p,grid_angle,r);
 var res=select(0.0,clamp(d1/aa,0.0,1.0),d1>0.0);res+=select(0.0,clamp(d2/aa,0.0,1.0),d2>0.0);res+=select(0.0,clamp(d3/aa,0.0,1.0),d3>0.0);res+=select(0.0,clamp(d4/aa,0.0,1.0),d4>0.0);
 return clamp(res,0.0,1.0);
}
fn ht_blend_colour(a:f32,b:f32,t:f32)->f32{
 let mode=i32(params[2].y);
 if mode==1 {return ht_blend(a,b,1.0-t);} if mode==3 {return ht_blend(a,min(1.0,a+b),t);}
 if mode==2 {return ht_blend(a,max(0.0,a*b),t);} if mode==4 {return ht_blend(a,max(a,b),t);}
 if mode==5 {return ht_blend(a,min(a,b),t);} return ht_blend(a,b,1.0-t);
}
fn effect(uv:vec2<f32>)->vec4<f32>{
 if params[2].w>0.5 {return textureSampleLevel(input_texture,input_sampler,uv,0.0);}
 let size=params[1].yz;let p=vec2(uv.x*size.x,(1.0-uv.y)*size.y);
 let radius=params[0].x;let aa=select(1.25,radius*0.5,radius<2.5);
 var r=ht_dot(p,params[0].y,0,aa);var g=ht_dot(p,params[0].z,1,aa);var b=ht_dot(p,params[0].w,2,aa);
 let colour=textureSampleLevel(input_texture,input_sampler,uv,0.0);
 r=ht_blend_colour(r,colour.r,params[2].x);g=ht_blend_colour(g,colour.g,params[2].x);b=ht_blend_colour(b,colour.b,params[2].x);
 if params[2].z>0.5 {let m=(r+b+g)/3.0;r=m;g=m;b=m;}
 return vec4(r,g,b,1.0);
}
"#;
