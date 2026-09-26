//! SkyMesh with a dynamic cube reflection, the cascaded sun light over the sky
//! environment, the pointer-lock walk, the video panorama and the ocean.
pub(super) mod sky;
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::trackball_sprites::FirstPerson;
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, environment::EnvironmentMap, geometry::*,
    material::*, math::*, raycast::Raycaster, renderer::*, scene::*, shader::ShaderProgram, tsl::*,
};
use sky::{Sky, sky_mesh, sky_program};
use std::f64::consts::PI;
use std::sync::Arc;

fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
/// MathUtils.seededRandom (mulberry32).
fn seeded_random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_add(0x6D2B79F5);
    let mut t = *seed;
    t = (t ^ (t >> 15)).wrapping_mul(t | 1);
    t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
    (t ^ (t >> 14)) as f64 / 4294967296.
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
/// `Color.setHSL( h, s, l, SRGBColorSpace )`: HSL in sRGB, stored linear.
fn hsl_srgb(h: f64, s: f64, l: f64) -> Color {
    let c = super::shapes_lights::hsl(h, s, l).0;
    let f = crate::math::srgb_to_linear;
    Color::linear(f(c.x), f(c.y), f(c.z))
}
/// `Vector3.setFromSphericalCoords( 1, phi, theta )`.
fn spherical(phi: f64, theta: f64) -> Vector3 {
    Vector3::new(phi.sin() * theta.sin(), phi.cos(), phi.sin() * theta.cos())
}
/// webgpu_sky: the sky, its six-face cube capture and the reflecting sphere.
struct SkyScene {
    sky: Object3D,
    sphere: Object3D,
    cube_camera: Object3D,
    cube: RenderTarget,
    /// turbidity, rayleigh, mieCoefficient, mieDirectionalG, elevation, azimuth,
    /// exposure, showSunDisc, cloudCoverage, cloudDensity, cloudElevation.
    params: [f64; 11],
}
/// webgpu_lights_sunlight.
struct Sunlight {
    sky: Object3D,
    sun: Object3D,
    walker: FirstPerson,
    program: Arc<ShaderProgram>,
    /// showCascades, far, resolution, azimuth, elevation.
    params: [f64; 5],
    built: Option<[f64; 2]>,
}
/// misc_controls_pointerlock.
struct Walk {
    objects: Vec<Object3D>,
    locked: bool,
    velocity: Vector3,
    keys: [bool; 4],
    can_jump: bool,
    jump: bool,
    look: Vector2,
}
/// webgpu_video_panorama: the inside-out sphere with the VideoTexture.
struct Panorama {
    video: web_sys::HtmlVideoElement,
    texture: wgpu::Texture,
    /// The video time of the last uploaded frame.
    uploaded: Option<f64>,
    lon: f64,
    lat: f64,
    drag: Option<(f64, f64, f64, f64)>,
}
/// webgpu_ocean: WaterMesh with its half-resolution reflector, the sky and bloom.
struct Ocean {
    water: Object3D,
    sky: Object3D,
    cube: Object3D,
    mirror_camera: Object3D,
    reflected: RenderTarget,
    sampler: wgpu::Sampler,
    normals: crate::texture_gpu::GpuTexture,
    input: RenderTarget,
    bloom: crate::tsl::bloom::Bloom,
    combine: crate::postprocessing::Effect,
    program: Arc<ShaderProgram>,
    /// elevation, azimuth, exposure, distortionScale, size, bloom strength, bloom
    /// radius, cloud coverage, cloud density, cloud elevation.
    params: [f64; 10],
    built: Option<[f64; 2]>,
}
/// WaterMesh's color node, sampling the reflector (texture 0) and the normals (texture 1,
/// flipped as TextureLoader uploads it).
const WATER: &str = r#"
fn water_sample(c:vec2<f32>)->vec4<f32>{return textureSample(tsl_texture_1,tsl_sampler_1,vec2(c.x,1.0-c.y));}
fn water_color(world:vec3<f32>,screen:vec2<f32>,a:vec4<f32>,b:vec4<f32>,c:vec4<f32>)->vec4<f32>{
 let t=a.w;let uv=world.xz*b.w;
 let uv0=uv/103.0+vec2(t/17.0,t/29.0);
 let uv1=uv/107.0-vec2(t/-19.0,t/31.0);
 let uv2=uv/vec2(8907.0,9803.0)+vec2(t/101.0,t/97.0);
 let uv3=uv/vec2(1091.0,1027.0)-vec2(t/109.0,t/-113.0);
 let noise=(water_sample(uv0)+water_sample(uv1)+water_sample(uv2)+water_sample(uv3))*0.5-1.0;
 // TSL's mul( 1.5, 1.0, 1.5 ) multiplies by each scalar in turn: a uniform 2.25.
 let surfaceNormal=normalize(noise.xzy*2.25);
 let worldToEye=u.camera.xyz-world;
 let eyeDirection=normalize(worldToEye);
 let reflection=normalize(reflect(-a.xyz,surfaceNormal));
 let direction=max(0.0,dot(eyeDirection,reflection));
 let specularLight=pow(direction,100.0)*b.rgb*2.0;
 let diffuseLight=max(dot(a.xyz,surfaceNormal),0.0)*b.rgb*0.5;
 let distortion=surfaceNormal.xz*(0.001+1.0/length(worldToEye))*c.w;
 let mirror=textureSample(tsl_texture_0,tsl_sampler_0,vec2(1.0-screen.x,screen.y)+distortion).rgb;
 let theta=max(dot(eyeDirection,surfaceNormal),0.0);
 let reflectance=pow(1.0-theta,5.0)*(1.0-0.02)+0.02;
 let scatter=max(0.0,dot(surfaceNormal,eyeDirection))*c.rgb;
 return vec4(mix(b.rgb*diffuseLight*0.3+scatter,mirror+specularLight,vec3(reflectance)),1.0);
}"#;
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    controls: Option<Controls>,
    sky: Option<SkyScene>,
    sunlight: Option<Sunlight>,
    walk: Option<Walk>,
    panorama: Option<Panorama>,
    ocean: Option<Ocean>,
}
const RESOLUTIONS: [u32; 5] = [256, 512, 1024, 2048, 4096];
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            263 => (60., 100., 2000000., Vector3::new(0., 100., 2000.)),
            264 => (60., 0.5, 5000., Vector3::new(60., 8., 0.)),
            266 => (75., 0.25, 10., Vector3::ZERO),
            267 => (55., 1., 20000., Vector3::new(30., 30., 100.)),
            _ => (75., 1., 1000., Vector3::new(0., 10., 0.)),
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
            sky: None,
            sunlight: None,
            walk: None,
            panorama: None,
            ocean: None,
        };
        match id {
            263 => d.sky_scene(s, c, r).await?,
            264 => d.sunlight_scene(s, c, r).await?,
            266 => d.panorama_scene(s, r).await?,
            267 => d.ocean_scene(s, c, r).await?,
            _ => d.walk_scene(s)?,
        }
        Ok(d)
    }
    async fn sky_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let program = sky_program(r).await?;
        let sky = sky_mesh(s, &program, 450000.)?;
        // CubeRenderTarget( 256, HalfFloatType ), captured by a CubeCamera at the origin.
        let cube = RenderTarget::with_options(
            &r.device,
            256,
            256,
            RenderTargetOptions {
                depth: 6,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let view = cube.texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // MeshBasicNodeMaterial( { envMap } ): the reflected view ray, x flipped as
        // CubeTextureNode samples it.
        let env = WgslFn::new(
            "sphere_env",
            "fn sphere_env(n:vec3<f32>,p:vec3<f32>)->vec4<f32>{let v=normalize(p-u.camera.xyz);let d=reflect(v,normalize(n));return vec4(textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb,1.0);}",
            &[Type::Vec3, Type::Vec3],
            Type::Vec4,
        )?
        .call(&[normal_world(), position_world()]);
        let source = NodeMaterial::new(env).wgsl_with_texture_types(&[Type::TextureCube], &[])?;
        let program = ShaderProgram::with_projection_and_dimensions(
            r,
            &source,
            &[(&view, &sampler)],
            &[wgpu::TextureViewDimension::Cube],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let sphere = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(400., 64, 32)?),
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        let cube_camera = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 90.,
            aspect: 1.,
            near: 1.,
            far: 1000.,
            ..Default::default()
        })));
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.sky = Some(SkyScene {
            sky,
            sphere,
            cube_camera,
            cube,
            params: [10., 3., 0.005, 0.7, 65., 0., 0.05, 1., 0.4, 0.4, 0.5],
        });
        Ok(())
    }
    fn sky_uniforms(p: &[f64; 11]) -> Sky {
        Sky {
            turbidity: p[0],
            rayleigh: p[1],
            mie_coefficient: p[2],
            mie_directional_g: p[3],
            sun: spherical((90. - p[4]).to_radians(), p[5].to_radians()),
            cloud_coverage: p[8],
            cloud_density: p[9],
            cloud_elevation: p[10],
            show_sun_disc: p[7] > 0.5,
            ..Default::default()
        }
    }
    async fn sunlight_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 0.6;
        s.environment_intensity = 0.5;
        s.fog = Some(Fog::Linear {
            color: Color::BLACK,
            near: 500.,
            far: 4000.,
        });
        s.look_at(c, Vector3::new(-60., 8., 0.))?;
        let program = sky_program(r).await?;
        let sky = sky_mesh(s, &program, 9000.)?;
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 3.,
        }));
        let n = s.get_mut(sun)?;
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            far: 1000.,
            map_size: Some(2048),
            normal_bias: 0.05,
            ..Default::default()
        };
        // WebGPU's PhysicalLightingModel conserves specular energy.
        let standard = |hex: u32| {
            let mut m = MeshStandardMaterial {
                energy_conservation: true,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(hex);
            Arc::new(Material::Standard(m))
        };
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10000., 10000., 1, 1)?),
            standard(0xa39f8e),
        )));
        let n = s.get_mut(ground)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.receive_shadow = true;
        let posts = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 10., 1.)?),
            standard(0x475161),
        )));
        let n = s.get_mut(posts)?;
        n.cast_shadow = true;
        n.receive_shadow = true;
        for i in 0..60 {
            for side in [-1., 1.] {
                n.instances.push(Instance {
                    matrix: Matrix4::from_translation(Vector3::new(
                        30. - i as f64 * 17.,
                        5.,
                        side * 13.,
                    )),
                    ..Default::default()
                });
            }
        }
        let towers = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(10., 1., 10.)?),
            standard(0xffffff),
        )));
        let colors = [Color::from_hex(0x08d9d6), Color::from_hex(0xff2e63)];
        let mut seed = 6u32;
        seeded_random(&mut seed);
        let n = s.get_mut(towers)?;
        n.cast_shadow = true;
        n.receive_shadow = true;
        for i in 0..30 {
            for (k, side) in [-1., 1.].into_iter().enumerate() {
                let height = 20. + seeded_random(&mut seed) * 40.;
                let x = -40. - i as f64 * 32. - seeded_random(&mut seed) * 10.;
                let z = side * (38. + seeded_random(&mut seed) * 55.);
                n.instances.push(Instance {
                    matrix: Matrix4::from_scale_rotation_translation(
                        Vector3::new(1., height, 1.),
                        Quaternion::IDENTITY,
                        Vector3::new(x, height / 2., z),
                    ),
                    color: colors[(i + k) % 2],
                });
            }
        }
        let mut walker = FirstPerson::default();
        walker.speed = 40.;
        // FirstPersonControls._setOrientation() from the camera's initial quaternion.
        let look = s.get(c)?.quaternion * -Vector3::Z;
        walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
        walker.lon = look.x.atan2(look.z).to_degrees();
        self.sunlight = Some(Sunlight {
            sky,
            sun,
            walker,
            program,
            params: [0., 1000., 3., 135., 10.],
            built: None,
        });
        Ok(())
    }
    /// updateSun(): light direction, color and intensity, fog color, the sky and
    /// a new PMREM environment from a scene holding only the sky.
    fn update_sun(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let k = self.sunlight.as_mut().ok_or(Error::Invalid("sunlight"))?;
        let [_, far, resolution, azimuth, elevation] = k.params;
        let n = s.get_mut(k.sun)?;
        n.shadow.far = far;
        n.shadow.map_size = Some(RESOLUTIONS[resolution as usize]);
        if k.built == Some([azimuth, elevation]) {
            return Ok(());
        }
        k.built = Some([azimuth, elevation]);
        let position = spherical((90. - elevation).to_radians(), azimuth.to_radians());
        let daylight = (elevation / 30.).min(1.);
        let lerp = |a: u32, b: u32| {
            let (a, b) = (Color::from_hex(a).0, Color::from_hex(b).0);
            Color(a + (b - a) * daylight)
        };
        let n = s.get_mut(k.sun)?;
        n.position = position;
        if let NodeKind::Light(Light::Sun { color, intensity }) = &mut n.kind {
            *color = lerp(0xff8a3d, 0xfff2e3);
            *intensity = 3. + daylight * 2.;
        }
        if let Some(Fog::Linear { color, .. }) = &mut s.fog {
            *color = lerp(0xd9a273, 0xd8e2ea);
        }
        let uniforms = Sky {
            turbidity: 3.,
            rayleigh: 2.,
            cloud_speed: 0.,
            show_sun_disc: false,
            sun: position,
            ..Default::default()
        };
        uniforms.apply(s, k.sky, self.time)?;
        let mut env = Scene::new();
        let h = sky_mesh(&mut env, &k.program, 9000.)?;
        uniforms.apply(&mut env, h, self.time)?;
        s.environment = Some(Arc::new(EnvironmentMap::from_scene(r, &mut env, 256, 0.)?));
        Ok(())
    }
    /// The page's `#video` (loop, muted, playsinline), playing, once its first frame
    /// is available.
    async fn panorama_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        use wasm_bindgen::JsCast;
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or(Error::Invalid("document"))?;
        let video: web_sys::HtmlVideoElement = document
            .get_element_by_id("video")
            .ok_or(Error::Invalid("video element"))?
            .dyn_into()
            .map_err(|_| Error::Invalid("video element"))?;
        while video.ready_state() < 2 {
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                let _ = web_sys::window()
                    .map(|w| w.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 20));
            });
            wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|_| Error::Invalid("video wait"))?;
        }
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("video texture"),
            size: wgpu::Extent3d {
                width: video.video_width(),
                height: video.video_height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        // VideoTexture: linear filtering without mipmaps, clamped, flipped on upload.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let color = crate::tsl::Texture::External(0).sample(uv());
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(1)?,
            &[],
            &[(&view, &sampler)],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let mut geometry = SphereGeometry::build(5., 60, 40)?;
        geometry.scale(Vector3::new(-1., 1., 1.))?;
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        self.panorama = Some(Panorama {
            video,
            texture,
            uploaded: None,
            lon: 0.,
            lat: 0.,
            drag: None,
        });
        Ok(())
    }
    /// VideoTexture uploads each newly presented frame; the camera orbits by lon/lat.
    fn panorama_step(&mut self, r: &Renderer, s: &mut Scene, c: Object3D) -> Result<()> {
        let k = self.panorama.as_mut().ok_or(Error::Invalid("panorama"))?;
        let time = k.video.current_time();
        if k.video.ready_state() >= 2 && k.uploaded != Some(time) {
            r.queue.copy_external_image_to_texture(
                &wgpu::CopyExternalImageSourceInfo {
                    source: wgpu::ExternalImageSource::HTMLVideoElement(k.video.clone()),
                    origin: wgpu::Origin2d::ZERO,
                    flip_y: true,
                },
                wgpu::CopyExternalImageDestInfo {
                    texture: &k.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                    color_space: wgpu::PredefinedColorSpace::Srgb,
                    premultiplied_alpha: false,
                },
                k.texture.size(),
            );
            k.uploaded = Some(time);
        }
        k.lat = k.lat.clamp(-85., 85.);
        let (phi, theta) = ((90. - k.lat).to_radians(), k.lon.to_radians());
        s.get_mut(c)?.position =
            Vector3::new(phi.sin() * theta.cos(), phi.cos(), phi.sin() * theta.sin()) * 0.5;
        s.look_at(c, Vector3::ZERO)
    }
    async fn ocean_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let mut t = super::gltf_viewer::decode_texture_image(
            &super::gltf_viewer::fetch("/web/gallery/assets/waternormals.jpg").await?,
        )
        .await?;
        // TextureLoader leaves the normals in NoColorSpace.
        t.srgb = false;
        t.wrap_s = Wrapping::Repeat;
        t.wrap_t = Wrapping::Repeat;
        t.mipmap_filter = Some(Filter::Linear);
        let normals = r.upload_texture(&Arc::new(t))?;
        let reflected = Self::reflector(r, 1, 1)?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let color = WgslFn::new(
            "water_color",
            WATER,
            &[Type::Vec3, Type::Vec2, Type::Vec4, Type::Vec4, Type::Vec4],
            Type::Vec4,
        )?
        .call(&[
            position_world(),
            crate::tsl::viewport::screen_uv(),
            uniform(0, Type::Vec4),
            uniform(1, Type::Vec4),
            uniform(2, Type::Vec4),
        ]);
        let reflected_view = reflected.texture.create_view(&Default::default());
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(2)?,
            &[],
            &[
                (&reflected_view, &sampler),
                (&normals.view, &normals.sampler),
            ],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.properties.transparent = true;
        let water = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10000., 10000., 1, 1)?),
            Arc::new(Material::Shader(m)),
        )));
        s.get_mut(water)?.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        let program = sky_program(r).await?;
        let sky = sky_mesh(s, &program, 10000.)?;
        let mut box_material = MeshStandardMaterial {
            roughness: 0.,
            energy_conservation: true,
            ..Default::default()
        };
        box_material.properties.color = Color::WHITE;
        let cube = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(30., 30., 30.)?),
            Arc::new(Material::Standard(box_material)),
        )));
        let mirror_camera = s.insert(s.get(c)?.kind.clone());
        s.get_mut(mirror_camera)?.matrix_auto_update = false;
        let input = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples: 1,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let mut bloom = crate::tsl::bloom::Bloom::new(r).await?;
        bloom.threshold = 0.;
        bloom.strength = 0.1;
        bloom.radius = 0.;
        let combine = crate::postprocessing::Effect::new(
            r,
            wgpu::TextureFormat::Rgba16Float,
            "fn effect(uv:vec2<f32>)->vec4<f32>{return textureSample(input_texture,input_sampler,uv)+textureSample(history_texture,input_sampler,uv);}",
        )
        .await?;
        let mut controls = Controls::new(None, (40., 200.), PI * 0.495, true);
        controls.set_target(Vector3::new(0., 10., 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.ocean = Some(Ocean {
            water,
            sky,
            cube,
            mirror_camera,
            reflected,
            sampler,
            normals,
            input,
            bloom,
            combine,
            program,
            params: [2., 180., 0.1, 3.7, 1., 0.1, 0., 0.4, 0.5, 0.5],
            built: None,
        });
        Ok(())
    }
    /// ReflectorNode's HalfFloat target.
    fn reflector(r: &Renderer, w: u32, h: u32) -> Result<RenderTarget> {
        RenderTarget::with_options(
            &r.device,
            w,
            h,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )
    }
    fn ocean_sky(p: &[f64; 10]) -> Sky {
        Sky {
            turbidity: 10.,
            rayleigh: 2.,
            mie_coefficient: 0.005,
            mie_directional_g: 0.8,
            sun: spherical((90. - p[0]).to_radians(), p[1].to_radians()),
            cloud_coverage: p[7],
            cloud_density: p[8],
            cloud_elevation: p[9],
            ..Default::default()
        }
    }
    fn ocean_step(&mut self, r: &Renderer, s: &mut Scene, t: f64) -> Result<()> {
        let k = self.ocean.as_mut().ok_or(Error::Invalid("ocean"))?;
        let sky = Self::ocean_sky(&k.params);
        s.exposure = k.params[2];
        k.bloom.strength = k.params[5] as f32;
        k.bloom.radius = k.params[6] as f32;
        sky.apply(s, k.sky, t)?;
        // updateSun(): a new PMREM environment when the sun moves.
        if k.built != Some([k.params[0], k.params[1]]) {
            k.built = Some([k.params[0], k.params[1]]);
            let mut env = Scene::new();
            let h = sky_mesh(&mut env, &k.program, 10000.)?;
            sky.apply(&mut env, h, t)?;
            s.environment = Some(Arc::new(EnvironmentMap::from_scene(r, &mut env, 256, 0.)?));
        }
        let sun = sky.sun.normalize();
        if let NodeKind::Mesh(m) = &mut s.get_mut(k.water)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            let water = Color::from_hex(0x001e0f).0;
            m.uniforms[0] = [sun.x as f32, sun.y as f32, sun.z as f32, t as f32];
            m.uniforms[1] = [1., 1., 1., k.params[4] as f32];
            m.uniforms[2] = [
                water.x as f32,
                water.y as f32,
                water.z as f32,
                k.params[3] as f32,
            ];
        }
        let n = s.get_mut(k.cube)?;
        n.position.y = t.sin() * 20. + 5.;
        n.quaternion = Euler {
            angles: Vector3::new(t * 0.5, 0., t * 0.51),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        Ok(())
    }
    fn walk_scene(&mut self, s: &mut Scene) -> Result<()> {
        s.background = Color::WHITE;
        s.fog = Some(Fog::Linear {
            color: Color::WHITE,
            near: 0.,
            far: 750.,
        });
        let light = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0xeeeeff),
            ground: Color::from_hex(0x777788),
            intensity: 2.5,
        }));
        s.get_mut(light)?.position = Vector3::new(0.5, 1., 0.75);
        // The floor: rotated, jittered in Float32, made non-indexed, then colored.
        let mut floor = PlaneGeometry::build(2000., 2000., 100, 100)?;
        floor.rotate_x(-PI / 2.)?;
        if let Some(Attribute::F32(a)) = floor.attributes.get("position") {
            let mut p = a.array().to_vec();
            for v in p.as_chunks_mut::<3>().0 {
                v[0] = (v[0] as f64 + random(&mut self.seed) * 20. - 10.) as f32;
                v[1] = (v[1] as f64 + random(&mut self.seed) * 2.) as f32;
                v[2] = (v[2] as f64 + random(&mut self.seed) * 20. - 10.) as f32;
            }
            floor.set_attribute("position", vec3s(p)?);
        }
        let mut floor = floor.to_non_indexed()?;
        floor.attributes.remove("normal");
        floor.attributes.remove("uv");
        let count = floor.draw_count();
        let mut colors = Vec::with_capacity(count * 3);
        for _ in 0..count {
            let h = random(&mut self.seed) * 0.3 + 0.5;
            let l = random(&mut self.seed) * 0.25 + 0.75;
            colors.extend(hsl_srgb(h, 0.75, l).0.to_array().map(|v| v as f32));
        }
        floor.set_attribute("color", vec3s(colors)?);
        let mut basic = MeshBasicMaterial::default();
        basic.properties.vertex_colors = true;
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(floor),
            Arc::new(Material::Basic(basic)),
        )));
        let mut boxes = BoxGeometry::build(20., 20., 20.)?.to_non_indexed()?;
        let count = boxes.draw_count();
        let mut colors = Vec::with_capacity(count * 3);
        for _ in 0..count {
            let h = random(&mut self.seed) * 0.3 + 0.5;
            let l = random(&mut self.seed) * 0.25 + 0.75;
            colors.extend(hsl_srgb(h, 0.75, l).0.to_array().map(|v| v as f32));
        }
        boxes.set_attribute("color", vec3s(colors)?);
        let boxes = Arc::new(boxes);
        let mut objects = vec![];
        for _ in 0..500 {
            let h = random(&mut self.seed) * 0.2 + 0.5;
            let l = random(&mut self.seed) * 0.25 + 0.75;
            let mut m = MeshPhongMaterial {
                specular: Color::WHITE,
                ..Default::default()
            };
            m.properties.flat_shading = true;
            m.properties.vertex_colors = true;
            m.properties.color = hsl_srgb(h, 0.75, l);
            let b = s.insert(NodeKind::Mesh(Mesh::new(
                boxes.clone(),
                Arc::new(Material::Phong(m)),
            )));
            let x = (random(&mut self.seed) * 20. - 10.).floor() * 20.;
            let y = (random(&mut self.seed) * 20.).floor() * 20. + 10.;
            let z = (random(&mut self.seed) * 20. - 10.).floor() * 20.;
            s.get_mut(b)?.position = Vector3::new(x, y, z);
            objects.push(b);
        }
        self.walk = Some(Walk {
            objects,
            locked: false,
            velocity: Vector3::ZERO,
            keys: [false; 4],
            can_jump: false,
            jump: false,
            look: Vector2::ZERO,
        });
        Ok(())
    }
    /// animate(): the locked walk with gravity, landing on boxes by a downward ray.
    fn walk_step(&mut self, s: &mut Scene, c: Object3D, delta: f64) -> Result<()> {
        let k = self.walk.as_mut().ok_or(Error::Invalid("walk"))?;
        // PointerLockControls.onMouseMove: YXZ Euler, 0.002 rad per pixel.
        if k.look != Vector2::ZERO {
            let n = s.get_mut(c)?;
            let (y, x, z) = n.quaternion.to_euler(glam::EulerRot::YXZ);
            let x = (x - k.look.y * 0.002).clamp(-PI / 2., PI / 2.);
            let y = y - k.look.x * 0.002;
            n.quaternion = Quaternion::from_euler(glam::EulerRot::YXZ, y, x, z);
            k.look = Vector2::ZERO;
        }
        if k.jump {
            if k.can_jump {
                k.velocity.y += 350.;
            }
            k.can_jump = false;
            k.jump = false;
        }
        if !k.locked {
            return Ok(());
        }
        s.update()?;
        let mut ray = Raycaster::default();
        let origin = s.get(c)?.position - Vector3::new(0., 10., 0.);
        ray.set(origin, Vector3::NEG_Y);
        ray.near = 0.;
        ray.far = 10.;
        let on_object = !ray.intersect_objects(s, &k.objects, false)?.is_empty();
        k.velocity.x -= k.velocity.x * 10. * delta;
        k.velocity.z -= k.velocity.z * 10. * delta;
        k.velocity.y -= 9.8 * 100. * delta;
        let b = |v: bool| f64::from(u8::from(v));
        let [forward, backward, left, right] = k.keys;
        let direction = Vector3::new(b(right) - b(left), 0., b(forward) - b(backward));
        let direction = if direction.length() > 0. {
            direction / direction.length()
        } else {
            direction
        };
        if forward || backward {
            k.velocity.z -= direction.z * 400. * delta;
        }
        if left || right {
            k.velocity.x -= direction.x * 400. * delta;
        }
        if on_object {
            k.velocity.y = k.velocity.y.max(0.);
            k.can_jump = true;
        }
        // moveRight: the camera matrix's x column; moveForward: up × that column.
        let n = s.get_mut(c)?;
        let column = n.quaternion * Vector3::X;
        n.position += column * (-k.velocity.x * delta);
        n.position += n.up.cross(column) * (-k.velocity.z * delta);
        n.position.y += k.velocity.y * delta;
        if n.position.y < 10. {
            k.velocity.y = 0.;
            n.position.y = 10.;
            k.can_jump = true;
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, r: &Renderer, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let delta = t - self.last;
        self.last = t;
        match self.id {
            263 => {
                let k = self.sky.as_ref().ok_or(Error::Invalid("sky"))?;
                s.exposure = k.params[6];
                Self::sky_uniforms(&k.params).apply(s, k.sky, t)?;
            }
            266 => self.panorama_step(r, s, c)?,
            267 => self.ocean_step(r, s, t)?,
            264 => {
                self.update_sun(s, r)?;
                let k = self.sunlight.as_mut().ok_or(Error::Invalid("sunlight"))?;
                if delta > 0. {
                    k.walker.update(s, c, delta)?;
                }
            }
            _ => self.walk_step(s, c, delta.max(0.))?,
        }
        Ok(())
    }
    /// The sky example renders the CubeCamera's six faces with the sphere hidden; the
    /// ocean renders its reflector, then the scene pass with bloom added.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if let Some(k) = self.ocean.as_mut() {
            let (w, h) = (
                ((out.width as f64) * 0.5).round().max(1.) as u32,
                ((out.height as f64) * 0.5).round().max(1.) as u32,
            );
            if (k.reflected.width, k.reflected.height) != (w, h) {
                k.reflected = Self::reflector(r, w, h)?;
                let view = k.reflected.texture.create_view(&Default::default());
                if let NodeKind::Mesh(m) = &mut s.get_mut(k.water)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    Arc::make_mut(&mut m.program).rebind(
                        r,
                        &[],
                        &[(&view, &k.sampler), (&k.normals.view, &k.normals.sampler)],
                    )?;
                }
            }
            s.update_world_matrix(c, true, false)?;
            let (camera, world) = s.camera(c)?;
            let Camera::Perspective(camera) = camera else {
                return Err(Error::Invalid("ocean camera"));
            };
            if let Some((camera, matrix)) =
                crate::reflection::planar_camera(camera, world, Vector4::new(0., 1., 0., 0.))?
            {
                let n = s.get_mut(k.mirror_camera)?;
                n.kind = NodeKind::Camera(Camera::Perspective(camera));
                n.matrix = matrix;
                n.matrix_world_needs_update = true;
                s.get_mut(k.water)?.visible = false;
                let result = r.render(s, k.mirror_camera, &k.reflected);
                s.get_mut(k.water)?.visible = true;
                result?;
            }
            if (k.input.width, k.input.height) != (out.width, out.height) {
                k.input.set_size(&r.device, out.width, out.height)?;
            }
            r.render(s, c, &k.input)?;
            let bloom = k.bloom.render(r, &k.input)?;
            k.combine.apply(r, &k.input, Some(bloom), out)?;
            return Ok(true);
        }
        let Some(k) = self.sky.as_mut() else {
            return Ok(false);
        };
        s.get_mut(k.sphere)?.visible = false;
        for (i, (direction, up)) in [
            (Vector3::NEG_X, Vector3::Y),
            (Vector3::X, Vector3::Y),
            (Vector3::Y, Vector3::NEG_Z),
            (Vector3::NEG_Y, Vector3::Z),
            (Vector3::Z, Vector3::Y),
            (Vector3::NEG_Z, Vector3::Y),
        ]
        .into_iter()
        .enumerate()
        {
            s.get_mut(k.cube_camera)?.up = up;
            s.look_at(k.cube_camera, direction)?;
            k.cube.set_layer(i as u32)?;
            r.render(s, k.cube_camera, &k.cube)?;
        }
        s.get_mut(k.sphere)?.visible = true;
        r.render(s, c, out)?;
        Ok(true)
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(k) = &mut self.panorama {
            match kind {
                10..=19 => k.drag = Some((x, y, k.lon, k.lat)),
                20..=29 => k.drag = None,
                _ => {
                    if let Some((x0, y0, lon, lat)) = k.drag {
                        k.lon = (x0 - x) * 0.1 + lon;
                        k.lat = (y0 - y) * 0.1 + lat;
                    }
                }
            }
        }
        if let Some(k) = &mut self.sunlight {
            k.walker.pointer(kind, x, y);
        }
    }
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
        if let Some(k) = &mut self.walk {
            if k.locked {
                k.look += Vector2::new(dx, dy);
            }
            return Ok(());
        }
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. || pan {
            // The sky example disables zoom and pan.
            if self.id == 263 {
                return Ok(());
            }
            if wheel != 0. {
                controls.dolly(wheel, &camera, Vector2::ZERO);
            } else {
                controls.pan(&camera, dx, dy, height);
            }
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(k) = &mut self.sunlight {
            k.walker.key(code, down);
        }
        if let Some(k) = &mut self.walk {
            match code {
                38 | 87 => k.keys[0] = down,
                37 | 65 => k.keys[2] = down,
                40 | 83 => k.keys[1] = down,
                39 | 68 => k.keys[3] = down,
                32 if down => k.jump = true,
                _ => {}
            }
        }
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        match (self.id, index) {
            (263, 0..=10) => {
                self.sky.as_mut().ok_or(Error::Invalid("sky"))?.params[index] = v;
            }
            (264, 0..=4) => {
                if index == 2 && !(0. ..5.).contains(&v) {
                    return Err(Error::Invalid("shadow resolution"));
                }
                self.sunlight
                    .as_mut()
                    .ok_or(Error::Invalid("sunlight"))?
                    .params[index] = v;
            }
            (265, 0) => self.walk.as_mut().ok_or(Error::Invalid("walk"))?.locked = v > 0.5,
            (267, 0..=9) => self.ocean.as_mut().ok_or(Error::Invalid("ocean"))?.params[index] = v,
            _ => return Err(Error::Invalid("sky/water parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
