//! The cube-camera light probe, HDR/LDR/generated environment maps, UltraHDR
//! gain-map environments, the transmissive sphere and the dungeon scene.
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::gltf_viewer::{fetch, load_asset};
use super::lights_probes::{cube_background, cube_sh, pisa_faces, pisa_hdr, sh_call};
pub(super) use super::ultra_hdr::ultra_hdr;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, float, vec4};
use crate::{
    Error, Result, camera::*, environment::EnvironmentMap, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
const PROJECT: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";

/// The LightProbeHelper sphere: SH irradiance in uniforms 0..8, intensity in 9.
async fn probe_helper(
    s: &mut Scene,
    r: &Renderer,
    sh: &[Vector3; 9],
    size: f64,
) -> Result<Object3D> {
    let color = vec4(sh_call(Some(9))?, float(1.));
    let program =
        ShaderProgram::with_projection(r, &NodeMaterial::new(color).wgsl(0)?, &[], &[], PROJECT)
            .await?;
    let mut m = ShaderMaterial::new(Arc::new(program));
    for (i, c) in sh.iter().enumerate() {
        m.uniforms[i] = [c.x as f32, c.y as f32, c.z as f32, 0.];
    }
    m.uniforms[9] = [1., 0., 0., 0.];
    let helper = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(SphereGeometry::build(1., 32, 16)?),
        Arc::new(Material::Shader(m)),
    )));
    s.get_mut(helper)?.scale = Vector3::splat(size);
    Ok(helper)
}

/// webgl_materials_envmaps_hdr: the three PMREMs, the two raw cube backgrounds
/// and the debug plane showing the chosen atlas.
struct Envmaps {
    torus: Object3D,
    plane: Object3D,
    /// Generated, LDR, HDR.
    environments: [Arc<EnvironmentMap>; 3],
    backgrounds: [Object3D; 2],
    /// envMap, roughness, metalness, exposure, debug.
    params: [f64; 5],
    rotation: f64,
    planes: [Arc<Material>; 3],
}
/// The physical torus knot or sphere with its roughness/metalness controls.
struct Hdr {
    mesh: Object3D,
    /// autoRotate, metalness, roughness, exposure, resolution (0 2k, 1 4k), type.
    params: [f64; 6],
    rotation: f64,
    /// The resolution whose environment is shown.
    loaded: usize,
    /// Environments by resolution; a load in flight fills its slot.
    environments: [std::rc::Rc<std::cell::RefCell<Option<Arc<EnvironmentMap>>>>; 2],
    requested: [bool; 2],
}
/// webgpu_materials_transmission's material controls.
struct Transmission {
    mesh: Object3D,
    /// color, transmission, opacity, metalness, roughness, ior, thickness,
    /// specularIntensity, specularColor, envMapIntensity, exposure.
    params: [f64; 11],
}
pub struct Demo {
    id: u32,
    time: f64,
    last: f64,
    controls: Option<Controls>,
    envmaps: Option<Envmaps>,
    hdr: Option<Hdr>,
    transmission: Option<Transmission>,
    model: Vec<Object3D>,
    /// webgpu_performance's `static` control, applied before the next frame.
    static_meshes: bool,
    /// The cube background, kept centered on the camera.
    sky: Option<Object3D>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        // The page's DRACOLoader.
        crate::compression::install_draco();
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            283 => (40., 1., 1000., Vector3::new(0., 0., 30.)),
            284 => (40., 1., 1000., Vector3::new(0., 0., 120.)),
            285 => (50., 1., 500., Vector3::new(0., 0., -6.)),
            286 => (40., 1., 2000., Vector3::new(0., 0., 120.)),
            _ => (45., 0.1, 100., Vector3::new(60., 60., 60.)),
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
            controls: None,
            envmaps: None,
            hdr: None,
            transmission: None,
            model: vec![],
            static_meshes: true,
            sky: None,
        };
        match id {
            283 => d.cube_camera_scene(s, c, r).await?,
            284 => d.envmaps_scene(s, c, r).await?,
            285 => d.ultrahdr_scene(s, c).await?,
            286 => d.transmission_scene(s, c).await?,
            _ => d.performance_scene(s, c).await?,
        }
        Ok(d)
    }
    /// The pisa background captured by a 256² CubeCamera into an 8-bit
    /// NoColorSpace cube, then LightProbeGenerator.fromCubeRenderTarget. The
    /// target's texels are the faces' texels (same layout, 256² each), stored as
    /// 8-bit linear values; the SH is computed from them on the CPU, once.
    async fn cube_camera_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let faces = pisa_faces().await?;
        let sh = cube_sh(&faces, true);
        let cube = crate::texture_gpu::GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        self.sky = Some(cube_background(s, r, &cube).await?);
        probe_helper(s, r, &sh, 5.).await?;
        let mut controls = Controls::new(None, (10., 50.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        Ok(())
    }
    async fn envmaps_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let torus = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(18., 8., 150, 20, 2, 3)?),
            Arc::new(Material::Standard(MeshStandardMaterial {
                metalness: 0.,
                roughness: 0.,
                ..Default::default()
            })),
        )));
        // DebugEnvironment: a back-sided room, a point light and three emissive panels.
        let mut debug = Scene::new();
        debug.background = Color::BLACK;
        let mut box_geometry = BoxGeometry::build(1., 1., 1.)?;
        box_geometry.delete_attribute("uv");
        let box_geometry = Arc::new(box_geometry);
        let mut room = MeshStandardMaterial {
            metalness: 0.,
            ..Default::default()
        };
        room.properties.side = Side::Back;
        let h = debug.insert(NodeKind::Mesh(Mesh::new(
            box_geometry.clone(),
            Arc::new(Material::Standard(room)),
        )));
        debug.get_mut(h)?.scale = Vector3::splat(10.);
        debug.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 50.,
            distance: 0.,
            decay: 2.,
        }));
        for (hex, position, scale) in [
            (
                0xff0000,
                Vector3::new(-5., 2., 0.),
                Vector3::new(0.1, 1., 1.),
            ),
            (
                0x00ff00,
                Vector3::new(0., 5., 0.),
                Vector3::new(1., 0.1, 1.),
            ),
            (
                0x0000ff,
                Vector3::new(2., 1., 5.),
                Vector3::new(1.5, 2., 0.1),
            ),
        ] {
            let mut m = MeshLambertMaterial {
                emissive: Color(Vector3::splat(10.)),
                ..Default::default()
            };
            m.properties.color = Color::from_hex(hex);
            let h = debug.insert(NodeKind::Mesh(Mesh::new(
                box_geometry.clone(),
                Arc::new(Material::Lambert(m)),
            )));
            let n = debug.get_mut(h)?;
            n.position = position;
            n.scale = scale;
        }
        let generated = EnvironmentMap::from_scene(r, &mut debug, 256, 0.)?;
        let faces = pisa_faces().await?;
        let ldr_cube = crate::texture_gpu::GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        let ldr = EnvironmentMap::from_cube_texture(r, &ldr_cube)?;
        let hdr_cube = pisa_hdr(r).await?;
        let hdr = EnvironmentMap::from_cube_texture(r, &hdr_cube)?;
        // WebGL draws cube backgrounds on a unit box.
        let unit_box = Arc::new(BoxGeometry::build(1., 1., 1.)?);
        let ldr_background = cube_background(s, r, &ldr_cube).await?;
        let hdr_background = cube_background(s, r, &hdr_cube).await?;
        for h in [ldr_background, hdr_background] {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                m.geometry = unit_box.clone();
                // WebGLBackground tone-maps a cube background unless it is sRGB.
                if h == ldr_background {
                    Arc::make_mut(&mut m.materials[0])
                        .properties_mut()
                        .tone_mapped = false;
                }
            }
        }
        let environments = [Arc::new(generated), Arc::new(ldr), Arc::new(hdr)];
        // The debug plane maps the chosen PMREM atlas in three's texel layout: the
        // port's atlas stores each face region upside down with ±Y exchanged.
        let mut planes = vec![];
        for env in &environments {
            let gpu = env.gpu.as_ref().ok_or(Error::Invalid("PMREM"))?;
            let atlas = crate::tsl::WgslFn::new(
                "three_atlas",
                ATLAS,
                &[crate::tsl::Type::Vec2],
                crate::tsl::Type::Vec4,
            )?
            .call(&[crate::tsl::uv()]);
            let program = ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(atlas).wgsl(1)?,
                &[],
                &[(gpu.view(), gpu.sampler())],
                PROJECT,
            )
            .await?;
            planes.push(Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(
                program,
            )))));
        }
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(200., 200., 1, 1)?),
            planes[2].clone(),
        )));
        let n = s.get_mut(plane)?;
        n.position.y = -50.;
        n.quaternion = Quaternion::from_rotation_x(-PI * 0.5);
        n.visible = false;
        let mut controls = Controls::new(None, (50., 300.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.envmaps = Some(Envmaps {
            torus,
            plane,
            environments,
            backgrounds: [ldr_background, hdr_background],
            params: [2., 0., 0., 1., 0.],
            rotation: 0.,
            planes: planes.try_into().map_err(|_| Error::Invalid("planes"))?,
        });
        Ok(())
    }
    async fn ultrahdr_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let env = Arc::new(
            ultra_hdr(&fetch(&format!("{ASSETS}/probes-hdr/spruit_sunrise_2k.hdr.jpg")).await?)
                .await?,
        );
        s.environment = Some(env.clone());
        s.background_environment = true;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(1., 0.4, 128, 128, 1, 3)?),
            Arc::new(Material::Standard(MeshStandardMaterial {
                metalness: 1.,
                roughness: 0.,
                ..Default::default()
            })),
        )));
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.hdr = Some(Hdr {
            mesh,
            params: [1., 1., 0., 1., 0., 0.],
            rotation: 0.,
            loaded: 0,
            environments: [
                std::rc::Rc::new(std::cell::RefCell::new(Some(env))),
                Default::default(),
            ],
            requested: [true, false],
        });
        Ok(())
    }
    async fn transmission_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let env =
            ultra_hdr(&fetch(&format!("{ASSETS}/probes-hdr/royal_esplanade_2k.hdr.jpg")).await?)
                .await?;
        s.environment = Some(Arc::new(env));
        s.background_environment = true;
        // generateTexture(): a 2 × 2 canvas, white on the bottom row. alphaMap reads
        // its green channel: a white map with that alpha carries the same values.
        let mut rgba = vec![255, 255, 255, 0, 255, 255, 255, 0];
        rgba.extend([255u8; 8]);
        let mut alpha = Texture::from_rgba(2, 2, rgba, false)?;
        alpha.wrap_s = Wrapping::Repeat;
        alpha.wrap_t = Wrapping::Repeat;
        alpha.repeat = Vector2::new(1., 3.5);
        alpha.filter = Filter::Nearest;
        alpha.min_filter = Some(Filter::Linear);
        alpha.mipmap_filter = Some(Filter::Linear);
        let mut m = MeshPhysicalMaterial::default();
        m.base.metalness = 0.;
        m.base.roughness = 0.;
        m.base.energy_conservation = true;
        m.ior = 1.5;
        m.transmission = 1.;
        m.thickness = 0.01;
        m.specular_intensity = 1.;
        m.base.properties.map = Some(Arc::new(alpha));
        m.base.properties.side = Side::Double;
        m.base.properties.transparent = true;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(20., 64, 32)?),
            Arc::new(Material::Physical(m)),
        )));
        let mut controls = Controls::new(None, (10., 150.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.transmission = Some(Transmission {
            mesh,
            params: [
                f64::from(0xffffff),
                1.,
                1.,
                0.,
                0.,
                1.5,
                0.01,
                1.,
                f64::from(0xffffff),
                1.,
                1.,
            ],
        });
        Ok(())
    }
    async fn performance_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let env =
            ultra_hdr(&fetch(&format!("{ASSETS}/probes-hdr/royal_esplanade_2k.hdr.jpg")).await?)
                .await?;
        s.environment = Some(Arc::new(env));
        let (asset, buffers, images) =
            load_asset(&format!("{ASSETS}/probes-hdr/dungeon_warkarma.glb")).await?;
        self.model = crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        for &h in &self.model {
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
        let mut controls = Controls::new(None, (2., 60.), PI, true);
        controls.set_target(Vector3::new(0., 0., -0.2));
        controls.update(s, c)?;
        self.controls = Some(controls);
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
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        if let Some(sky) = self.sky {
            let p = s.get(c)?.position;
            s.get_mut(sky)?.position = p;
        }
        match self.id {
            284 => {
                let k = self.envmaps.as_mut().ok_or(Error::Invalid("envmaps"))?;
                let [env, roughness, metalness, exposure, debug] = k.params;
                let env = env as usize;
                for _ in 0..steps {
                    k.rotation += 0.005;
                }
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(k.torus)?.kind
                    && let Material::Standard(current) = mesh.materials[0].as_ref()
                    && (current.roughness, current.metalness) != (roughness, metalness)
                    && let Material::Standard(m) = Arc::make_mut(&mut mesh.materials[0])
                {
                    m.roughness = roughness;
                    m.metalness = metalness;
                }
                s.get_mut(k.torus)?.quaternion = Quaternion::from_rotation_y(k.rotation);
                if s.environment
                    .as_ref()
                    .is_none_or(|e| !Arc::ptr_eq(e, &k.environments[env]))
                {
                    s.environment = Some(k.environments[env].clone());
                    if let NodeKind::Mesh(m) = &mut s.get_mut(k.plane)?.kind {
                        m.materials[0] = k.planes[env].clone();
                    }
                }
                s.get_mut(k.plane)?.visible = debug > 0.5;
                // Generated uses the PMREM itself as the background; LDR and HDR the cubes.
                s.background_environment = env == 0;
                s.background_pmrem = env == 0;
                s.get_mut(k.backgrounds[0])?.visible = env == 1;
                s.get_mut(k.backgrounds[1])?.visible = env == 2;
                s.exposure = exposure;
                let p = s.get(c)?.position;
                for b in k.backgrounds {
                    s.get_mut(b)?.position = p;
                }
            }
            285 => {
                let k = self.hdr.as_mut().ok_or(Error::Invalid("hdr"))?;
                let [auto, metalness, roughness, exposure, _, _] = k.params;
                if auto > 0.5 {
                    for _ in 0..steps {
                        k.rotation += 0.005;
                    }
                }
                s.get_mut(k.mesh)?.quaternion = Quaternion::from_rotation_y(k.rotation);
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(k.mesh)?.kind
                    && let Material::Standard(current) = mesh.materials[0].as_ref()
                    && (current.roughness, current.metalness) != (roughness, metalness)
                    && let Material::Standard(m) = Arc::make_mut(&mut mesh.materials[0])
                {
                    m.roughness = roughness;
                    m.metalness = metalness;
                }
                // loadEnvironment( resolution ): the new texture replaces the old once loaded.
                let index = k.params[4] as usize;
                if !k.requested[index] {
                    k.requested[index] = true;
                    let slot = k.environments[index].clone();
                    let name = ["spruit_sunrise_2k", "spruit_sunrise_4k"][index];
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(bytes) =
                            fetch(&format!("{ASSETS}/probes-hdr/{name}.hdr.jpg")).await
                            && let Ok(env) = ultra_hdr(&bytes).await
                        {
                            *slot.borrow_mut() = Some(Arc::new(env));
                        }
                    });
                }
                if k.environments[index].borrow().is_some() {
                    k.loaded = index;
                }
                if let Some(env) = k.environments[k.loaded].borrow().as_ref()
                    && s.environment.as_ref().is_none_or(|e| !Arc::ptr_eq(e, env))
                {
                    s.environment = Some(env.clone());
                }
                s.exposure = exposure;
                if let Some(controls) = &mut self.controls {
                    controls.frame_update(s, c)?;
                }
            }
            286 => {
                let k = self
                    .transmission
                    .as_ref()
                    .ok_or(Error::Invalid("transmission"))?;
                let p = k.params;
                s.exposure = p[10];
                s.environment_intensity = p[9];
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(k.mesh)?.kind
                    && let Material::Physical(current) = mesh.materials[0].as_ref()
                {
                    let color = Color::from_hex(p[0] as u32);
                    let specular = Color::from_hex(p[8] as u32);
                    let changed = current.base.properties.color != color
                        || current.transmission != p[1]
                        || current.base.properties.opacity != p[2]
                        || current.base.metalness != p[3]
                        || current.base.roughness != p[4]
                        || current.ior != p[5]
                        || current.thickness != p[6]
                        || current.specular_intensity != p[7]
                        || current.specular_color != specular;
                    if changed && let Material::Physical(m) = Arc::make_mut(&mut mesh.materials[0])
                    {
                        m.base.properties.color = color;
                        m.transmission = p[1];
                        m.base.properties.opacity = p[2];
                        m.base.metalness = p[3];
                        m.base.roughness = p[4];
                        m.ior = p[5];
                        m.thickness = p[6];
                        m.specular_intensity = p[7];
                        m.specular_color = specular;
                    }
                }
            }
            _ => {
                // setStatic(): no visual change; static meshes skip per-frame updates.
                for &h in &self.model {
                    s.get_mut(h)?.is_static = self.static_meshes;
                }
            }
        }
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
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
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            if self.id == 283 {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        match self.id {
            284 if index < 5 => {
                self.envmaps
                    .as_mut()
                    .ok_or(Error::Invalid("envmaps"))?
                    .params[index] = v
            }
            285 if index < 6 => self.hdr.as_mut().ok_or(Error::Invalid("hdr"))?.params[index] = v,
            286 if index < 11 => {
                self.transmission
                    .as_mut()
                    .ok_or(Error::Invalid("transmission"))?
                    .params[index] = v
            }
            287 if index == 0 => self.static_meshes = v > 0.5,
            _ => return Err(Error::Invalid("probes/hdr parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// Sample the port's PMREM atlas at a uv of three's atlas (GL rows, bottom up):
/// each face region is mirrored vertically and faces 1 and 4 trade places.
const ATLAS: &str = "fn three_atlas(uv:vec2<f32>)->vec4<f32>{
 let size=vec2<f32>(textureDimensions(tsl_texture_0));let max_mip=log2(size.y/4.0);
 let p=uv*size;var q=p;
 for(var level=i32(max_mip);level>=-2;level--){
  let extra=f32(max(4-level,0));let s=exp2(f32(max(level,4)));
  let x0=extra*48.0;let y0=4.0*(exp2(max_mip)-s);
  if p.y>=y0 && p.y<y0+2.0*s && p.x>=x0 && p.x<x0+3.0*s {
   let col=floor((p.x-x0)/s);let row=floor((p.y-y0)/s);var face=u32(col+3.0*row);
   if face==1u {face=4u;} else if face==4u {face=1u;}
   let local=vec2(p.x-x0-col*s,p.y-y0-row*s);
   q=vec2(x0+f32(face%3u)*s+local.x,y0+select(0.0,s,face>2u)+(s-local.y));
   break;
  }
 }
 return vec4(textureSampleLevel(tsl_texture_0,tsl_sampler_0,q/size,0.0).rgb,1.0);
}";

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/webgpu_lightprobe_cubecamera.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_lightprobe_cubecamera(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            on_demand: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                5000.,
                Box::new(Demo::create(scene, camera, 283, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_materials_envmaps_hdr.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_materials_envmaps_hdr(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 1,
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                5000.,
                Box::new(Demo::create(scene, camera, 284, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_loader_texture_ultrahdr.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_texture_ultrahdr(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                5000.,
                Box::new(Demo::create(scene, camera, 285, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_materials_transmission.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_materials_transmission(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                5000.,
                Box::new(Demo::create(scene, camera, 286, renderer).await?),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_performance.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_performance(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                5000.,
                Box::new(Demo::create(scene, camera, 287, renderer).await?),
            ))
        },
    )
    .await
}
