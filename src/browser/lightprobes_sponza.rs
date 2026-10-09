//! webgpu_lightprobes_sponza: Sponza, fetched at run time from
//! glTF-Sample-Assets as the page does ( its license does not grant
//! redistribution ), lit by a shadowed SunLight under SkyMesh with ACES,
//! FirstPersonControls and diffuse GI from a LightProbeGrid baked a few
//! probes per frame.
use super::gltf_viewer::{fetch, load_asset_bytes};
use super::sky_water::sky::{Sky, sky_mesh, sky_program};
use super::trackball_sprites::FirstPerson;
use crate::light_probe_grid::{
    BakeOptions, GridBaker, LightProbeGrid, bake, helper, helper_program,
};
use crate::{Error, Result, camera::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const MODEL_INDEX_URL: &str = "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/model-index.json";
const SAMPLE_ASSETS_BASE_URL: &str =
    "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/";

/// The page's params.
struct Params {
    enabled: bool,
    bounds: Vector3,
    size: Vector3,
    count: [u32; 3],
    bounces: u32,
    show_probes: bool,
    probe_size: f64,
    light_azimuth: f64,
    light_elevation: f64,
    light_intensity: f64,
    shadows: bool,
}
pub(super) struct Demo {
    controls: FirstPerson,
    baker: GridBaker,
    probes: Option<Object3D>,
    probes_helper: Option<Object3D>,
    helper_program: Arc<crate::shader::ShaderProgram>,
    probe_far: f64,
    bake_index: u32,
    bake_pass: u32,
    sun: Object3D,
    sky: Object3D,
    sky_uniforms: Sky,
    params: Params,
    /// Parameters whose onChange runs before the next frame.
    pending: Vec<usize>,
    time: f64,
    last: f64,
}
/// getSponzaModelURL(): the Sponza entry of the sample model index.
async fn sponza_url() -> Result<String> {
    let index: serde_json::Value = serde_json::from_slice(&fetch(MODEL_INDEX_URL).await?)
        .map_err(|e| Error::Asset(e.to_string()))?;
    let sponza = index
        .as_array()
        .and_then(|models| models.iter().find(|m| m["name"] == "Sponza"))
        .ok_or(Error::Asset(
            "Sponza entry was not found in the glTF sample model index.".into(),
        ))?;
    let variants = &sponza["variants"];
    let variant = ["glTF-Binary", "glTF", "glTF-Embedded"]
        .iter()
        .find_map(|k| variants[k].as_str())
        .or_else(|| variants.as_object()?.values().find_map(|v| v.as_str()))
        .ok_or(Error::Asset(
            "Sponza has no supported glTF variant in the model index.".into(),
        ))?;
    let folder = if variant.ends_with(".glb") {
        "glTF-Binary"
    } else {
        "glTF"
    };
    let name = sponza["name"].as_str().unwrap_or("Sponza");
    Ok(format!("{SAMPLE_ASSETS_BASE_URL}{name}/{folder}/{variant}"))
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.1,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(-10.25, 4.99, 0.40);
        n.quaternion = Euler {
            angles: Vector3::new(1.6505, -1.5008, 1.6507),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        let program = sky_program(r).await?;
        let sky = sky_mesh(s, &program, 450000.)?;
        let sky_uniforms = Sky {
            turbidity: 10.,
            rayleigh: 2.,
            mie_coefficient: 0.005,
            mie_directional_g: 0.8,
            // Keep the clouds static, like the WebGL version; the sun is the light.
            cloud_speed: 0.,
            show_sun_disc: false,
            ..Default::default()
        };
        s.tone_mapping = ToneMapping::Aces;
        // FirstPersonControls( camera ): the orientation from the camera's look.
        let direction = s.get(c)?.quaternion * -Vector3::Z;
        let radius = direction.length();
        let phi = (direction.y / radius).clamp(-1., 1.).acos();
        let theta = direction.x.atan2(direction.z);
        let mut controls = FirstPerson::default();
        controls.speed = 2.;
        controls.lat = 90. - phi.to_degrees();
        controls.lon = theta.to_degrees();
        // The model, its meshes casting and receiving shadows.
        let url = sponza_url().await?;
        let base = url.rsplit_once('/').ok_or(Error::Invalid("Sponza URL"))?.0;
        let (asset, buffers, images) = load_asset_bytes(&fetch(&url).await?, base).await?;
        let model = crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        // _box.setFromObject( model ): the meshes' world boxes.
        s.update()?;
        let (mut lo, mut hi) = (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        );
        for h in model {
            let n = s.get_mut(h)?;
            n.cast_shadow = true;
            n.receive_shadow = true;
            // WebGPURenderer's standard materials conserve energy.
            if let NodeKind::Mesh(m) = &mut n.kind {
                for m in &mut m.materials {
                    if let crate::material::Material::Standard(m) = Arc::make_mut(m) {
                        m.energy_conservation = true;
                    }
                }
            }
            if let NodeKind::Mesh(m) = &n.kind {
                let local = match m.geometry.bounding_box {
                    Some(b) => b,
                    None => Box3::from_points(m.geometry.positions()?),
                };
                let world = local.transformed(n.matrix_world);
                lo = lo.min(world.min);
                hi = hi.max(world.max);
            }
        }
        let size = hi - lo;
        let probe_far = size.x.max(size.y).max(size.z) * 2.;
        // SunLight( 0xfff2dc, 100 ), shadow far 50 and a 2048² map.
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::from_hex(0xfff2dc),
            intensity: 100.,
        }));
        let n = s.get_mut(sun)?;
        n.cast_shadow = true;
        n.shadow.far = 50.;
        n.shadow.map_size = Some(2048);
        let mut demo = Self {
            controls,
            baker: GridBaker::new(r),
            probes: None,
            probes_helper: None,
            helper_program: helper_program(r).await?,
            probe_far,
            bake_index: 0,
            bake_pass: 0,
            sun,
            sky,
            sky_uniforms,
            params: Params {
                enabled: true,
                bounds: Vector3::new(-0.5, 6., -0.3),
                size: Vector3::new(21., 11., 9.),
                count: [10, 7, 7],
                bounces: 1,
                show_probes: false,
                probe_size: 0.2,
                light_azimuth: -75.,
                light_elevation: 60.,
                light_intensity: 100.,
                shadows: true,
            },
            pending: vec![],
            time: 0.,
            last: 0.,
        };
        demo.update_light_position(s)?;
        demo.create_probes(s)?;
        Ok(demo)
    }
    /// createProbes(): a new grid of the page's size and counts, then a rebake.
    fn create_probes(&mut self, s: &mut Scene) -> Result<()> {
        if let Some(h) = self.probes_helper.take() {
            s.dispose(h)?;
        }
        if let Some(h) = self.probes.take() {
            s.dispose(h)?;
        }
        let p = &self.params;
        let grid = LightProbeGrid::new(p.size.x, p.size.y, p.size.z, Some(p.count));
        let h = s.insert(NodeKind::LightProbeGrid(Box::new(grid)));
        let n = s.get_mut(h)?;
        n.position = p.bounds;
        n.visible = p.enabled;
        self.probes = Some(h);
        self.reset_probe_bake();
        Ok(())
    }
    fn reset_probe_bake(&mut self) {
        self.bake_index = 0;
        self.bake_pass = 0;
    }
    /// updateProbes(): four probes per frame, each pass finished before the next bounce.
    fn update_probes(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let Some(grid) = self.probes else {
            return Ok(());
        };
        if self.bake_pass > self.params.bounces {
            return Ok(());
        }
        let total: u32 = self.params.count.iter().product();
        let count = 4.min(total - self.bake_index);
        // Keep the helper spheres out of the cubemap captures.
        if let Some(h) = self.probes_helper {
            s.get_mut(h)?.visible = false;
        }
        let baked = bake(
            &mut self.baker,
            r,
            s,
            grid,
            BakeOptions {
                cubemap_size: 32,
                near: 0.05,
                far: self.probe_far,
                start: self.bake_index,
                count: Some(count),
                pass: self.bake_pass,
                ..Default::default()
            },
        );
        if let Some(h) = self.probes_helper {
            s.get_mut(h)?.visible = self.params.show_probes;
        }
        baked?;
        if self.probes_helper.is_none() {
            let h = helper(s, grid, self.params.probe_size, &self.helper_program)?;
            s.get_mut(h)?.visible = self.params.show_probes;
            self.probes_helper = Some(h);
        }
        self.bake_index += count;
        if self.bake_index == total {
            self.bake_index = 0;
            self.bake_pass += 1;
        }
        Ok(())
    }
    /// updateLightPosition(): the sun and the sky's sun on the sphere.
    fn update_light_position(&mut self, s: &mut Scene) -> Result<()> {
        let elevation = self.params.light_elevation.to_radians();
        let azimuth = self.params.light_azimuth.to_radians();
        let phi = PI / 2. - elevation;
        let position = Vector3::new(
            phi.sin() * azimuth.sin(),
            phi.cos(),
            phi.sin() * azimuth.cos(),
        );
        s.get_mut(self.sun)?.position = position;
        self.sky_uniforms.sun = position;
        self.sky_uniforms.apply(s, self.sky, 0.)
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the timer's delta for the controls, then the render.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let delta = (self.time - self.last).max(0.);
        self.last = self.time;
        for index in std::mem::take(&mut self.pending) {
            match index {
                0 => {
                    if let Some(h) = self.probes {
                        s.get_mut(h)?.visible = self.params.enabled;
                    }
                }
                1 | 2 => {
                    self.update_light_position(s)?;
                    self.reset_probe_bake();
                }
                3 | 4 | 8 => self.reset_probe_bake(),
                9 => {
                    if let Some(h) = self.probes_helper {
                        s.get_mut(h)?.visible = self.params.show_probes;
                    }
                }
                10 => {
                    if let (Some(old), Some(grid)) = (self.probes_helper.take(), self.probes) {
                        s.dispose(old)?;
                        let h = helper(s, grid, self.params.probe_size, &self.helper_program)?;
                        s.get_mut(h)?.visible = self.params.show_probes;
                        self.probes_helper = Some(h);
                    }
                }
                _ => self.create_probes(s)?,
            }
        }
        self.controls.update(s, c, delta)?;
        let n = s.get_mut(self.sun)?;
        n.cast_shadow = self.params.shadows;
        if let NodeKind::Light(Light::Sun { intensity, .. }) = &mut n.kind {
            *intensity = self.params.light_intensity;
        }
        self.update_probes(s, r)
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.controls.pointer(kind, x, y);
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.controls.key(code, down);
    }
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
    /// The GUI: GI, light azimuth, elevation and intensity, shadows, the
    /// probe counts, the bounces, and the helper's visibility and size.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = f64::from(value);
        match index {
            0 => self.params.enabled = v > 0.5,
            1 => self.params.light_azimuth = v,
            2 => self.params.light_elevation = v,
            3 => self.params.light_intensity = v,
            4 => self.params.shadows = v > 0.5,
            5..=7 => self.params.count[index - 5] = v as u32,
            8 => self.params.bounces = v as u32,
            9 => self.params.show_probes = v > 0.5,
            10 => self.params.probe_size = v,
            _ => return Err(Error::Invalid("webgpu_lightprobes_sponza parameter")),
        }
        self.pending.push(index);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
