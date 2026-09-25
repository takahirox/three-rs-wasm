//! Clearcoat spheres, the FlyControls earth with film grain, alpha-tested
//! point-light shadows, the dynamic cube map and the light probe.
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::tsl::surface::SurfaceNodes as SurfaceNodesAlias;
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, environment::EnvironmentMap, geometry::*,
    material::*, math::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
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
fn repeated(mut t: Texture, x: f64, y: f64) -> Arc<Texture> {
    t.wrap_s = Wrapping::Repeat;
    t.wrap_t = Wrapping::Repeat;
    t.repeat = Vector2::new(x, y);
    Arc::new(t)
}
/// HDRCubeTextureLoader: six RGBE faces as a resident half-float cube.
async fn pisa_hdr(r: &Renderer) -> Result<crate::texture_gpu::GpuTexture> {
    let mut pixels = vec![];
    let mut size = 0;
    for face in ["px", "nx", "py", "ny", "pz", "nz"] {
        let bytes = fetch(&format!(
            "{ASSETS}/tsl-environment/textures/cube/pisaHDR/{face}.hdr"
        ))
        .await?;
        let im = image::load_from_memory(&bytes).map_err(|e| Error::Asset(e.to_string()))?;
        if size == 0 {
            size = im.width();
        }
        pixels.extend(
            im.to_rgba32f()
                .as_raw()
                .iter()
                .map(|v| half::f16::from_f32(v.clamp(0.0, 65504.0))),
        );
    }
    crate::texture_gpu::GpuTexture::from_cube_hdr(r, size, &pixels)
}
/// FlakesTexture: 4000 random normal-colored discs drawn on a 512² canvas.
fn flakes(seed: &mut u32) -> Result<Texture> {
    use wasm_bindgen::JsCast;
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or(Error::Invalid("document"))?;
    let canvas: web_sys::HtmlCanvasElement = document
        .create_element("canvas")
        .map_err(|_| Error::Invalid("canvas"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("canvas"))?;
    canvas.set_width(512);
    canvas.set_height(512);
    let context: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .ok_or(Error::Invalid("2d context"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("2d context"))?;
    context.set_fill_style_str("rgb(127,127,255)");
    context.fill_rect(0., 0., 512., 512.);
    for _ in 0..4000 {
        let x = random(seed) * 512.;
        let y = random(seed) * 512.;
        let r = random(seed) * 3. + 3.;
        let (mut nx, mut ny, mut nz) = (random(seed) * 2. - 1., random(seed) * 2. - 1., 1.5);
        let l = (nx * nx + ny * ny + nz * nz).sqrt();
        nx /= l;
        ny /= l;
        nz /= l;
        context.set_fill_style_str(&format!(
            "rgb({},{},{})",
            nx * 127. + 127.,
            ny * 127. + 127.,
            nz * 255.
        ));
        context.begin_path();
        context
            .arc(x, y, r, 0., PI * 2.)
            .map_err(|_| Error::Invalid("arc"))?;
        context.fill();
    }
    let data = context
        .get_image_data(0., 0., 512., 512.)
        .map_err(|_| Error::Invalid("flakes pixels"))?
        .data()
        .0;
    let mut t = Texture::from_rgba(512, 512, data, false)?;
    t.mipmap_filter = Some(Filter::Linear);
    t.anisotropy = 16;
    Ok(t)
}
/// webgpu_clearcoat.
struct Clearcoat {
    light: Object3D,
    spheres: Vec<Object3D>,
    rotation: f64,
}
/// misc_controls_fly: FlyControls' move state and the film pass.
struct Fly {
    planet: Object3D,
    clouds: Object3D,
    moon: Object3D,
    rotation: [f64; 2],
    /// up, down, left, right, forward, back, pitchUp, pitchDown, yawLeft,
    /// yawRight, rollLeft, rollRight.
    state: [f64; 12],
    input: RenderTarget,
    film: crate::postprocessing::Effect,
}
/// webgpu_lightprobe: the probe's SH coefficients and the intensities.
struct Probe {
    sphere: Object3D,
    helper: Object3D,
    light: Object3D,
    sh: [Vector3; 9],
    /// lightProbeIntensity, directionalLightIntensity, envMapIntensity.
    params: [f64; 3],
}
/// SphericalHarmonics3.getBasisAt.
fn sh_basis(d: Vector3) -> [f64; 9] {
    let (x, y, z) = (d.x, d.y, d.z);
    [
        0.282095,
        0.488603 * y,
        0.488603 * z,
        0.488603 * x,
        1.092548 * x * y,
        1.092548 * y * z,
        0.315392 * (3. * z * z - 1.),
        1.092548 * x * z,
        0.546274 * (x * x - y * y),
    ]
}
/// getShIrradianceAt( normal, coefficients ) in WGSL, coefficients in uniforms 0..8.
const SH_IRRADIANCE: &str = "fn sh_irradiance(n:vec3<f32>,c0:vec4<f32>,c1:vec4<f32>,c2:vec4<f32>,c3:vec4<f32>,c4:vec4<f32>,c5:vec4<f32>,c6:vec4<f32>,c7:vec4<f32>,c8:vec4<f32>)->vec3<f32>{let x=n.x;let y=n.y;let z=n.z;var r=c0.xyz*0.886227;r+=c1.xyz*(2.0*0.511664)*y;r+=c2.xyz*(2.0*0.511664)*z;r+=c3.xyz*(2.0*0.511664)*x;r+=c4.xyz*(2.0*0.429043)*x*y;r+=c5.xyz*(2.0*0.429043)*y*z;r+=c6.xyz*(z*z*0.743125-0.247708);r+=c7.xyz*(2.0*0.429043)*x*z;r+=c8.xyz*0.429043*(x*x-y*y);return r;}";
fn sh_call(scale_uniform: Option<usize>) -> Result<crate::tsl::Node> {
    use crate::tsl::*;
    let mut args = vec![normal_world()];
    args.extend((0..9).map(|i| uniform(i, Type::Vec4)));
    let irradiance = WgslFn::new(
        "sh_irradiance",
        SH_IRRADIANCE,
        &[
            Type::Vec3,
            Type::Vec4,
            Type::Vec4,
            Type::Vec4,
            Type::Vec4,
            Type::Vec4,
            Type::Vec4,
            Type::Vec4,
            Type::Vec4,
            Type::Vec4,
        ],
        Type::Vec3,
    )?
    .call(&args);
    Ok(match scale_uniform {
        Some(i) => irradiance * uniform(i, Type::Float) * float((1. / PI) as f32),
        None => irradiance,
    })
}
/// webgpu_tonemapping's select options, in GUI order.
const TONE_MAPPINGS: [ToneMapping; 7] = [
    ToneMapping::None,
    ToneMapping::Linear,
    ToneMapping::Reinhard,
    ToneMapping::Cineon,
    ToneMapping::Aces,
    ToneMapping::AgX,
    ToneMapping::Neutral,
];
/// webgpu_shadowmap_pointlight.
struct PointShadows {
    lights: [Object3D; 2],
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    controls: Option<Controls>,
    clearcoat: Option<Clearcoat>,
    fly: Option<Fly>,
    points: Option<PointShadows>,
    probe: Option<Probe>,
    /// The cube background sphere, kept centered on the camera.
    sky: Option<Object3D>,
    /// toneMapping, exposure, blurriness, background intensity.
    tone: Option<[f64; 4]>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            268 => (27., 0.25, 50., Vector3::new(0., 0., 10.)),
            269 => (25., 50., 1e7, Vector3::new(0., 0., 6371. * 5.)),
            270 => (45., 1., 1000., Vector3::new(0., 10., 40.)),
            271 => (40., 1., 1000., Vector3::new(0., 0., 30.)),
            _ => (45., 0.01, 10., Vector3::new(-0.02, 0.03, 0.05)),
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
            clearcoat: None,
            fly: None,
            points: None,
            probe: None,
            sky: None,
            tone: None,
        };
        match id {
            268 => d.clearcoat_scene(s, c, r).await?,
            269 => d.fly_scene(s, r).await?,
            270 => d.point_scene(s, c).await?,
            271 => d.probe_scene(s, c, r).await?,
            _ => d.tone_scene(s, c).await?,
        }
        Ok(d)
    }
    async fn clearcoat_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 1.25;
        // scene.environment and scene.background are the same HDR cube: its PMREM
        // lights, and the background samples the cube itself.
        let cube = pisa_hdr(r).await?;
        s.environment = Some(Arc::new(EnvironmentMap::from_cube_texture(r, &cube)?));
        self.sky = Some(cube_background(s, r, &cube).await?);
        let carbon = repeated(texture("lights-probes/Carbon.png", true).await?, 10., 10.);
        let carbon_normal = repeated(
            texture("lights-probes/Carbon_Normal.png", false).await?,
            10.,
            10.,
        );
        let water = Arc::new(texture("lights-probes/Water_1_M_Normal.jpg", false).await?);
        // CanvasTexture( FlakesTexture ): drawn after the loads start, from the seeded stream.
        let flakes = repeated(flakes(&mut self.seed)?, 10., 6.);
        let golf = Arc::new(texture("lights-probes/golfball.jpg", false).await?);
        let scratched =
            Arc::new(texture("lights-probes/Scratched_gold_01_1K_Normal.png", false).await?);
        let physical = |f: &dyn Fn(&mut MeshPhysicalMaterial)| {
            let mut m = MeshPhysicalMaterial::default();
            // WebGPU's PhysicalLightingModel conserves specular energy.
            m.base.energy_conservation = true;
            m.clearcoat = 1.;
            f(&mut m);
            Arc::new(Material::Physical(m))
        };
        let materials = [
            physical(&|m| {
                m.clearcoat_roughness = 0.1;
                m.base.metalness = 0.9;
                m.base.roughness = 0.5;
                m.base.properties.color = Color::from_hex(0x0000ff);
                m.base.normal_map = Some(flakes.clone());
                m.base.normal_scale = Vector2::splat(0.15);
            }),
            physical(&|m| {
                m.base.roughness = 0.5;
                m.clearcoat_roughness = 0.1;
                m.base.properties.map = Some(carbon.clone());
                m.base.normal_map = Some(carbon_normal.clone());
            }),
            physical(&|m| {
                m.base.metalness = 0.;
                m.base.roughness = 0.1;
                m.base.normal_map = Some(golf.clone());
                m.clearcoat_normal_map = Some(scratched.clone());
                m.clearcoat_normal_scale = Vector2::new(2., -2.);
            }),
            physical(&|m| {
                m.base.metalness = 1.;
                m.base.properties.color = Color::from_hex(0xff0000);
                m.base.normal_map = Some(water.clone());
                m.base.normal_scale = Vector2::splat(0.15);
                m.clearcoat_normal_map = Some(scratched.clone());
                m.clearcoat_normal_scale = Vector2::new(2., -2.);
            }),
        ];
        let geometry = Arc::new(SphereGeometry::build(0.8, 64, 32)?);
        let mut spheres = vec![];
        for (m, p) in materials
            .into_iter()
            .zip([[-1., 1.], [1., 1.], [-1., -1.], [1., -1.]])
        {
            let h = s.insert(NodeKind::Mesh(Mesh::new(geometry.clone(), m)));
            s.get_mut(h)?.position = Vector3::new(p[0], p[1], 0.);
            spheres.push(h);
        }
        let light = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.05, 8, 8)?),
            Arc::new(Material::Basic(MeshBasicMaterial::default())),
        )));
        let point = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 30.,
            distance: 0.,
            decay: 2.,
        }));
        s.add(light, point)?;
        let mut controls = Controls::new(None, (3., 30.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.clearcoat = Some(Clearcoat {
            light,
            spheres,
            rotation: 0.,
        });
        Ok(())
    }
    async fn fly_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let radius = 6371.;
        s.fog = Some(Fog::Exp2 {
            color: Color::BLACK,
            density: 0.00000025,
        });
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(-1., 0., 1.).normalize();
        let mut planet = MeshPhongMaterial {
            specular: Color::from_hex(0x7c7c7c),
            shininess: 15.,
            normal_map: Some(Arc::new(
                texture("lights-probes/earth_normal_2048.jpg", false).await?,
            )),
            normal_scale: Vector2::new(0.85, -0.85),
            specular_map: Some(Arc::new(texture("earth_specular_2048.jpg", false).await?)),
            ..Default::default()
        };
        planet.properties.map = Some(Arc::new(texture("earth_atmos_2048.jpg", true).await?));
        let geometry = Arc::new(SphereGeometry::build(radius, 100, 50)?);
        let planet = s.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(Material::Phong(planet)),
        )));
        s.get_mut(planet)?.quaternion = euler(0., 0., 0.41);
        let mut clouds = MeshLambertMaterial::default();
        clouds.properties.map = Some(Arc::new(
            texture("lights-probes/earth_clouds_1024.png", true).await?,
        ));
        clouds.properties.transparent = true;
        let clouds = s.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(Material::Lambert(clouds)),
        )));
        let n = s.get_mut(clouds)?;
        n.scale = Vector3::splat(1.005);
        n.quaternion = euler(0., 0., 0.41);
        let mut moon = MeshPhongMaterial::default();
        moon.properties.map = Some(Arc::new(
            texture("lights-probes/moon_1024.jpg", true).await?,
        ));
        let moon = s.insert(NodeKind::Mesh(Mesh::new(
            geometry,
            Arc::new(Material::Phong(moon)),
        )));
        let n = s.get_mut(moon)?;
        n.position = Vector3::new(radius * 5., 0., 0.);
        n.scale = Vector3::splat(0.23);
        let mut stars = vec![];
        for count in [250, 1500] {
            let v: Vec<f32> = (0..count * 3)
                .map(|_| ((random(&mut self.seed) * 2. - 1.) * radius) as f32)
                .collect();
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(v, 3, false)?),
            );
            stars.push(Arc::new(g));
        }
        // r186's WebGPU Points draw PointsMaterial as native one-pixel points
        // (`point-list`), whatever its size.
        let program = Arc::new(
            crate::shader::ShaderProgram::with_projection(
                r,
                &crate::tsl::NodeMaterial::new(crate::tsl::uniform(0, crate::tsl::Type::Vec4)).wgsl(0)?,
                &[],
                &[],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
            )
            .await?,
        );
        let materials: Vec<Arc<Material>> = [0x9c9c9c, 0x838383, 0x5a5a5a]
            .map(|hex| {
                let mut m = ShaderMaterial::new(program.clone());
                let c = Color::from_hex(hex).0;
                m.uniforms[0] = [c.x as f32, c.y as f32, c.z as f32, 1.];
                Arc::new(Material::Shader(m))
            })
            .to_vec();
        for i in 10..30 {
            let h = s.insert(NodeKind::Points(Points {
                geometry: stars[i % 2].clone(),
                material: materials[i % 3].clone(),
            }));
            let (x, y, z) = (
                random(&mut self.seed) * 6.,
                random(&mut self.seed) * 6.,
                random(&mut self.seed) * 6.,
            );
            let n = s.get_mut(h)?;
            n.quaternion = euler(x, y, z);
            n.scale = Vector3::splat(i as f64 * 10.);
        }
        let input = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples: 4,
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        // FilmNode: base + base × clamp( rand( fract( uv + time ) ) + 0.1 ), with
        // TSL's rand = fract( sin( dot( uv, ( 12.9898, 78.233 ) ) mod π ) × 43758.5453 ).
        let film = crate::postprocessing::Effect::new(
            r,
            wgpu::TextureFormat::Rgba16Float,
            "fn effect(uv:vec2<f32>)->vec4<f32>{let base=textureSample(input_texture,input_sampler,uv);let p=fract(uv+vec2(params[0].x));let dt=dot(p,vec2(12.9898,78.233));let sn=dt-3.141592653589793*floor(dt/3.141592653589793);let noise=fract(sin(sn)*43758.5453);return vec4(base.rgb+base.rgb*clamp(noise+0.1,0.0,1.0),base.a);}",
        )
        .await?;
        self.fly = Some(Fly {
            planet,
            clouds,
            moon,
            rotation: [0.; 2],
            state: [0.; 12],
            input,
            film,
        });
        Ok(())
    }
    async fn point_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x111122),
            intensity: 3.,
        }));
        // The 2 × 2 canvas: a transparent top row and a white bottom row. alphaMap's
        // green equals this texture's alpha, so a white map with that alpha and
        // alphaTest 0.5 discards the same fragments in the view and the shadow maps.
        let mut rgba = vec![255, 255, 255, 0, 255, 255, 255, 0];
        rgba.extend([255u8; 8]);
        let mut alpha = Texture::from_rgba(2, 2, rgba, false)?;
        alpha.wrap_s = Wrapping::Repeat;
        alpha.wrap_t = Wrapping::Repeat;
        alpha.repeat = Vector2::new(1., 4.5);
        alpha.filter = Filter::Nearest;
        alpha.min_filter = Some(Filter::Linear);
        alpha.mipmap_filter = Some(Filter::Linear);
        let mut sphere_material = MeshPhongMaterial::default();
        sphere_material.properties.side = Side::Double;
        sphere_material.properties.map = Some(Arc::new(alpha));
        sphere_material.properties.alpha_test = 0.5;
        let sphere_material = Arc::new(Material::Phong(sphere_material));
        let mut lights = vec![];
        for hex in [0x0088ff, 0xff8888] {
            let light = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(hex),
                intensity: 200.,
                distance: 20.,
                decay: 2.,
            }));
            let n = s.get_mut(light)?;
            n.cast_shadow = true;
            n.shadow = crate::shadow::Shadow {
                bias: -0.005,
                map_size: Some(128),
                radius: 10.,
                ..Default::default()
            };
            let mut bulb = MeshBasicMaterial::default();
            bulb.properties.color = Color(Color::from_hex(hex).0 * 200.);
            let b = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(SphereGeometry::build(0.3, 12, 6)?),
                Arc::new(Material::Basic(bulb)),
            )));
            s.add(light, b)?;
            let sphere = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(SphereGeometry::build(2., 32, 8)?),
                sphere_material.clone(),
            )));
            let n = s.get_mut(sphere)?;
            n.cast_shadow = true;
            n.receive_shadow = true;
            s.add(light, sphere)?;
            lights.push(light);
        }
        let mut room = MeshPhongMaterial {
            shininess: 10.,
            specular: Color::from_hex(0x111111),
            ..Default::default()
        };
        room.properties.color = Color::from_hex(0xa0adaf);
        room.properties.side = Side::Back;
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(30., 30., 30.)?),
            Arc::new(Material::Phong(room)),
        )));
        let n = s.get_mut(h)?;
        n.position.y = 10.;
        n.receive_shadow = true;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 10., 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.points = Some(PointShadows {
            lights: [lights[0], lights[1]],
        });
        Ok(())
    }
    /// CubeTextureLoader's sRGB pisa faces, LightProbeGenerator.fromCubeTexture,
    /// the environment-mapped sphere and the LightProbeHelper.
    async fn probe_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let mut pixels = vec![];
        let mut faces = vec![];
        let mut sh = [Vector3::ZERO; 9];
        let mut total = 0.;
        for (face, name) in ["px", "nx", "py", "ny", "pz", "nz"].into_iter().enumerate() {
            let mut t = decode_texture_image(
                &fetch(&format!("{ASSETS}/lights-probes/pisa/{name}.png")).await?,
            )
            .await?;
            t.srgb = true;
            let width = t.width as usize;
            let pixel = 2. / width as f64;
            for (i, p) in t.rgba.as_chunks::<4>().0.iter().enumerate() {
                // CubeTextureLoader marks the faces sRGB; fromCubeTexture converts them.
                let color = Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64)
                    .map(|v| srgb_to_linear(v / 255.));
                let col = -1. + ((i % width) as f64 + 0.5) * pixel;
                let row = 1. - ((i / width) as f64 + 0.5) * pixel;
                let coord = match face {
                    0 => Vector3::new(-1., row, -col),
                    1 => Vector3::new(1., row, col),
                    2 => Vector3::new(-col, 1., -row),
                    3 => Vector3::new(-col, -1., row),
                    4 => Vector3::new(-col, row, 1.),
                    _ => Vector3::new(col, row, -1.),
                };
                let length_sq = coord.length_squared();
                let weight = 4. / (length_sq.sqrt() * length_sq);
                total += weight;
                let basis = sh_basis(coord.normalize());
                for j in 0..9 {
                    sh[j] += color * (basis[j] * weight);
                }
            }
            pixels.extend(t.rgba.iter().enumerate().map(|(i, &v)| {
                half::f16::from_f64(if i % 4 == 3 {
                    1.
                } else {
                    srgb_to_linear(v as f64 / 255.)
                })
            }));
            faces.push(t);
        }
        let norm = 4. * PI / total;
        for c in &mut sh {
            *c *= norm;
        }
        // The PMREM of the cube lights the sphere as its envMap; the background samples
        // the cube itself.
        s.environment = Some(Arc::new(EnvironmentMap::from_cube_hdr(r, 256, &pixels)?));
        let cube = crate::texture_gpu::GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        self.sky = Some(cube_background(s, r, &cube).await?);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 0.6,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(10., 10., 10.);
        // LightProbeNode adds getShIrradianceAt( normalWorld ) × intensity to the irradiance.
        let program = SurfaceNodesAlias {
            light_map: Some(sh_call(None)?),
            ..Default::default()
        }
        .build(r, &[], &[])
        .await?;
        let mut m = MeshStandardMaterial {
            metalness: 0.,
            roughness: 0.,
            energy_conservation: true,
            ..Default::default()
        };
        m.properties.vertex_program = Some(Arc::new(program));
        let sphere = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(5., 64, 32)?),
            Arc::new(Material::Standard(m)),
        )));
        let helper_color = crate::tsl::vec4(sh_call(Some(9))?, crate::tsl::float(1.));
        let program = crate::shader::ShaderProgram::with_projection(
            r,
            &crate::tsl::NodeMaterial::new(helper_color).wgsl(0)?,
            &[],
            &[],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let helper = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(1., 32, 16)?),
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        s.get_mut(helper)?.position = Vector3::new(-10., 0., 0.);
        let mut controls = Controls::new(None, (10., 50.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.probe = Some(Probe {
            sphere,
            helper,
            light,
            sh,
            params: [1., 0.6, 1.],
        });
        Ok(())
    }
    async fn tone_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0xfff3ee),
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(1., 0.05, 0.7);
        s.environment = Some(Arc::new(EnvironmentMap::from_hdr(
            &fetch("/web/environments/venice_sunset_1k.hdr").await?,
        )?));
        s.background_environment = true;
        let (asset, buffers, images) =
            super::gltf_viewer::load_asset(&format!("{ASSETS}/lights-probes/venice_mask.glb"))
                .await?;
        let instance = crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        for root in instance.clone() {
            for h in s.traverse(root, false)? {
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(h)?.kind {
                    for m in &mut mesh.materials {
                        match Arc::make_mut(m) {
                            Material::Standard(m) => m.energy_conservation = true,
                            Material::Physical(m) => m.base.energy_conservation = true,
                            _ => {}
                        }
                    }
                }
            }
        }
        let mut controls = Controls::new(Some(0.05), (0.03, 0.2), PI, true);
        controls.set_target(Vector3::new(0., 0.03, 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.tone = Some([6., 1., 0.3, 1.]);
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
        let steps = (delta * 60.).round().max(0.) as usize;
        self.last = t;
        match self.id {
            268 => {
                let k = self.clearcoat.as_mut().ok_or(Error::Invalid("clearcoat"))?;
                let timer = t * 1000. * 0.00025;
                s.get_mut(k.light)?.position = Vector3::new(
                    (timer * 7.).sin() * 3.,
                    (timer * 5.).cos() * 4.,
                    (timer * 3.).cos() * 3.,
                );
                for _ in 0..steps {
                    k.rotation += 0.005;
                }
                for &h in &k.spheres {
                    s.get_mut(h)?.quaternion = Quaternion::from_rotation_y(k.rotation);
                }
            }
            269 => self.fly_step(s, c, delta)?,
            271 => {
                let k = self.probe.as_ref().ok_or(Error::Invalid("probe"))?;
                let [probe, directional, env] = k.params;
                s.environment_intensity = env;
                if let NodeKind::Light(Light::Directional { intensity, .. }) =
                    &mut s.get_mut(k.light)?.kind
                {
                    *intensity = directional;
                }
                for (h, scaled) in [(k.sphere, true), (k.helper, false)] {
                    if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                        let m = Arc::make_mut(&mut m.materials[0]);
                        let uniforms = match m {
                            Material::Shader(m) => &mut m.uniforms,
                            other => &mut other.properties_mut().vertex_uniforms,
                        };
                        for (i, c) in k.sh.iter().enumerate() {
                            let c = if scaled { *c * probe } else { *c };
                            uniforms[i] = [c.x as f32, c.y as f32, c.z as f32, 0.];
                        }
                        uniforms[9] = [probe as f32, 0., 0., 0.];
                    }
                }
            }
            272 => {
                let [tone, exposure, blur, intensity] = self.tone.ok_or(Error::Invalid("tone"))?;
                s.tone_mapping = TONE_MAPPINGS[tone as usize];
                s.exposure = exposure;
                s.background_blur = blur;
                s.background_intensity = intensity;
                if let Some(controls) = &mut self.controls {
                    for _ in 0..steps.max(1) {
                        controls.frame_update(s, c)?;
                    }
                }
            }
            _ => {
                let k = self.points.as_ref().ok_or(Error::Invalid("points"))?;
                for (i, &light) in k.lights.iter().enumerate() {
                    let time = t + if i == 1 { 10000. } else { 0. };
                    let n = s.get_mut(light)?;
                    n.position = Vector3::new(
                        (time * 0.6).sin() * 9.,
                        (time * 0.7).sin() * 9. + 6.,
                        (time * 0.8).sin() * 9.,
                    );
                    n.quaternion = euler(time, 0., time);
                }
            }
        }
        if let Some(sky) = self.sky {
            let p = s.get(c)?.position;
            s.get_mut(sky)?.position = p;
        }
        Ok(())
    }
    /// render(): the planet and clouds turn; FlyControls move at a third of the
    /// distance to the nearer surface.
    fn fly_step(&mut self, s: &mut Scene, c: Object3D, delta: f64) -> Result<()> {
        let k = self.fly.as_mut().ok_or(Error::Invalid("fly"))?;
        let radius = 6371.;
        k.rotation[0] += 0.02 * delta;
        k.rotation[1] += 1.25 * 0.02 * delta;
        s.get_mut(k.planet)?.quaternion = euler(0., k.rotation[0], 0.41);
        s.get_mut(k.clouds)?.quaternion = euler(0., k.rotation[1], 0.41);
        let camera = s.get(c)?.position;
        let d_planet = camera.length();
        let d_moon = (camera - s.get(k.moon)?.position).length();
        let d = if d_moon < d_planet {
            d_moon - radius * 0.23 * 1.01
        } else {
            d_planet - radius * 1.01
        };
        let st = k.state;
        let movement = Vector3::new(-st[2] + st[3], -st[1] + st[0], -st[4] + st[5]);
        let rotation = Vector3::new(-st[7] + st[6], -st[9] + st[8], -st[11] + st[10]);
        let (move_mult, rot_mult) = (delta * 0.33 * d, delta * PI / 24.);
        let n = s.get_mut(c)?;
        let q = n.quaternion;
        n.position += q * (movement * move_mult);
        let r = rotation * rot_mult;
        n.quaternion = q * Quaternion::from_xyzw(r.x, r.y, r.z, 1.).normalize();
        Ok(())
    }
    /// The fly example's film pass; the others render directly.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let Some(k) = self.fly.as_mut() else {
            return Ok(false);
        };
        if (k.input.width, k.input.height) != (out.width, out.height) {
            k.input.set_size(&r.device, out.width, out.height)?;
        }
        r.render(s, c, &k.input)?;
        k.film.parameters[0] = [self.time as f32, 0., 0., 0.];
        k.film.apply(r, &k.input, None, out)?;
        Ok(true)
    }
    /// FlyControls pointer events: the pointer's offset from the center turns the
    /// camera; the buttons move it forward or back.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(k) = &mut self.fly {
            let (w, h, _) = super::controls_attributes::viewport_css();
            match kind {
                10 => k.state[4] = 1.,
                12 => k.state[5] = 1.,
                20 => k.state[4] = 0.,
                22 => k.state[5] = 0.,
                0 => {
                    k.state[8] = -(x - w / 2.) / (w / 2.);
                    k.state[7] = (y - h / 2.) / (h / 2.);
                }
                _ => {}
            }
        }
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(k) = &mut self.fly {
            let v = f64::from(u8::from(down));
            let index = match code {
                87 => 4,
                83 => 5,
                65 => 2,
                68 => 3,
                82 => 0,
                70 => 1,
                38 => 6,
                40 => 7,
                37 => 8,
                39 => 9,
                81 => 10,
                69 => 11,
                _ => return,
            };
            k.state[index] = v;
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
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
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
        match (self.id, index) {
            (271, 0..=2) => self.probe.as_mut().ok_or(Error::Invalid("probe"))?.params[index] = v,
            (272, 0..=3) => {
                if index == 0 && !(0. ..7.).contains(&v) {
                    return Err(Error::Invalid("tone mapping"));
                }
                self.tone.as_mut().ok_or(Error::Invalid("tone"))?[index] = v;
            }
            _ => return Err(Error::Invalid("lights/probes parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}

/// `scene.background = cubeTexture`: as in r186, a camera-centered 32 × 32 sphere
/// at the far plane.
async fn cube_background(
    s: &mut Scene,
    r: &Renderer,
    env: &crate::texture_gpu::GpuTexture,
) -> Result<Object3D> {
    use crate::tsl::*;
    let graph = NodeMaterial::new(vec4(
        WgslFn::new(
            "sky_color",
            "fn sky_color(d:vec3<f32>)->vec3<f32>{return textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb;}",
            &[Type::Vec3],
            Type::Vec3,
        )?
        .call(&[position_local()]),
        float(1.),
    ));
    let source = graph.wgsl_with_texture_types(&[Type::TextureCube], &[])?;
    let mut m = ShaderMaterial::new(Arc::new(
        crate::shader::ShaderProgram::with_projection_and_dimensions(
            r,
            &source,
            &[(&env.view, &env.sampler)],
            &[wgpu::TextureViewDimension::Cube],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(out.clip.xy,out.clip.w,out.clip.w);return out;}",
        )
        .await?,
    ));
    m.properties.side = Side::Back;
    m.properties.depth_write = false;
    m.properties.depth_test = false;
    let sky = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(SphereGeometry::build(1., 32, 32)?),
        Arc::new(Material::Shader(m)),
    )));
    let n = s.get_mut(sky)?;
    n.frustum_culled = false;
    n.render_order = -10000;
    Ok(sky)
}
