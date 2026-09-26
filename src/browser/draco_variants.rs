//! Uniform-buffer light arrays, the Draco bunny, keyframed Littlest Tokyo, glTF
//! material variants and the facecap morph targets.
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::gltf_viewer::{fetch, load_asset};
use super::sky_water::sky::{Sky, sky_mesh, sky_program};
use crate::compute::{BufferAccess, GpuBuffer};
use crate::environment::EnvironmentMap;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn, uniform};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
const PROJECT: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
const LIGHTS: usize = 300;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
/// webgl_ubo_arrays: the shared LightingData block as resident storage.
struct LightArrays {
    buffer: GpuBuffer,
    centers: Vec<(f64, f64)>,
    data: Vec<f32>,
    meshes: Vec<Object3D>,
    count: f64,
}
/// webgl_loader_gltf_variants: each mesh's original material and its mappings.
struct Variants {
    meshes: Vec<(Object3D, Arc<Material>, crate::gltf::VariantMaterials)>,
    names: Vec<String>,
    selected: usize,
    applied: Option<usize>,
}
pub struct Demo {
    id: u32,
    time: f64,
    last: f64,
    pending: f64,
    controls: Option<Controls>,
    lights: Option<LightArrays>,
    mixer: Option<crate::animation::AnimationMixer>,
    variants: Option<Variants>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            293 => (45., 0.1, 100., Vector3::new(0., 50., 50.)),
            294 => (35., 0.1, 15., Vector3::new(3., 0.25, 3.)),
            295 => (40., 1., 100., Vector3::new(5., 2., 8.)),
            296 => (45., 0.25, 20., Vector3::new(2.5, 1.5, 3.)),
            _ => (45., 1., 20., Vector3::new(-1.8, 0.8, 3.)),
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
            pending: 0.,
            controls: None,
            lights: None,
            mixer: None,
            variants: None,
        };
        match id {
            293 => d.light_arrays(s, c, r).await?,
            294 => d.bunny(s).await?,
            295 => d.keyframes(s, c, r).await?,
            296 => d.shoe(s, c, r).await?,
            _ => d.face(s, c, r).await?,
        }
        Ok(d)
    }
    async fn light_arrays(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.look_at(c, Vector3::ZERO)?;
        let mut seed = 186;
        let (mut positions, mut colors, mut centers) = (vec![], vec![], vec![]);
        for _ in 0..LIGHTS {
            // new THREE.Color( 0xffffff * Math.random() ).toArray(): the floored hex in
            // linear working space.
            let color = Color::from_hex((16777215. * random(&mut seed)).floor() as u32).0;
            let x = random(&mut seed) * 50. - 25.;
            let z = random(&mut seed) * 50. - 25.;
            positions.extend([x as f32, 1., z as f32, 0.]);
            colors.extend([color.x as f32, color.y as f32, color.z as f32, 0.]);
            centers.push((x, z));
        }
        let data = [positions, colors].concat();
        let buffer = GpuBuffer::new(r, bytemuck::cast_slice(&data), BufferAccess::Read)?;
        // The RawShaderMaterial: vPositionEye is the world position and the loop
        // adds each light's color times getDistanceAttenuation( d, 4, 0.7 ).
        let color = WgslFn::new(
            "ubo_lights",
            "fn ubo_lights(position:vec3<f32>,count:f32)->vec4<f32>{
 var color=vec3(0.0);
 for(var i=0;i<i32(count);i++){
  let d=length(tsl_attribute_0[i].xyz-position);
  var falloff=1.0/max(pow(d,0.7),0.01);
  let r=d/4.0;falloff*=pow(clamp(1.0-r*r*r*r,0.0,1.0),2.0);
  color+=tsl_attribute_0[i+300].xyz*falloff;
 }
 return vec4(color,1.0);
}",
            &[Type::Vec3, Type::Float],
            Type::Vec4,
        )?
        .call(&[crate::tsl::position_world(), uniform(0, Type::Vec4).x()]);
        let program = Arc::new(
            ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(color).wgsl_with_storage(0, &[Type::Vec4])?,
                &[&buffer],
                &[],
                PROJECT,
            )
            .await?,
        );
        let mut material = ShaderMaterial::new(program);
        // The block's count starts at POINTLIGHTS_MAX; the GUI's 200 applies on change.
        material.uniforms[0] = [300., 0., 0., 0.];
        let material = Arc::new(Material::Shader(material));
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(100., 100., 1, 1)?),
            material.clone(),
        )));
        let n = s.get_mut(plane)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.position.y = -1.;
        let sphere = Arc::new(SphereGeometry::build(1., 32, 16)?);
        let mut meshes = vec![plane];
        for i in 0..10 {
            for k in 0..10 {
                let h = s.insert(NodeKind::Mesh(Mesh::new(sphere.clone(), material.clone())));
                s.get_mut(h)?.position = Vector3::new(i as f64 * 6. - 30., 0., k as f64 * 6. - 30.);
                meshes.push(h);
            }
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.lights = Some(LightArrays {
            buffer,
            centers,
            data,
            meshes,
            count: 300.,
        });
        Ok(())
    }
    async fn bunny(&mut self, s: &mut Scene) -> Result<()> {
        s.background = Color::from_hex(0x443333);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x443333),
            near: 1.,
            far: 4.,
        });
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(8., 8., 1, 1)?),
            Arc::new(Material::Lambert(MeshLambertMaterial {
                properties: MaterialProperties {
                    color: Color::from_hex(0xcbcbcb),
                    ..Default::default()
                },
                ..Default::default()
            })),
        )));
        let n = s.get_mut(plane)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.position.y = 0.03;
        n.receive_shadow = true;
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0x8d7c7c),
            ground: Color::from_hex(0x494966),
            intensity: 3.,
        }));
        // HemisphereLight starts at Object3D.DEFAULT_UP.
        s.get_mut(hemi)?.position = Vector3::Y;
        let spot = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 7.,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 2.,
            angle: PI / 16.,
            penumbra: 0.5,
        }));
        let n = s.get_mut(spot)?;
        n.position = Vector3::new(-1., 1., 1.);
        n.cast_shadow = true;
        n.shadow.radius = 8.;
        // DRACOLoader, then computeVertexNormals().
        let mut geometry = crate::compression::decode_draco(
            &fetch(&format!("{ASSETS}/draco-variants/bunny.drc")).await?,
        )?;
        geometry.compute_vertex_normals()?;
        let mut material = MeshStandardMaterial::default();
        material.properties.color = Color::from_hex(0xa5a5a5);
        let bunny = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Standard(material)),
        )));
        let n = s.get_mut(bunny)?;
        n.cast_shadow = true;
        n.receive_shadow = true;
        Ok(())
    }
    async fn keyframes(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let sky_uniforms = Sky {
            turbidity: 0.,
            rayleigh: 3.,
            mie_directional_g: 0.7,
            cloud_elevation: 1.,
            sun: Vector3::new(-0.8, 0.19, 0.56),
            ..Default::default()
        };
        let program = sky_program(r).await?;
        let sky = sky_mesh(s, &program, 10000.)?;
        sky_uniforms.apply(s, sky, 0.)?;
        // PMREMGenerator.fromScene( sky ): the sky alone, captured and prefiltered once.
        let mut capture = Scene::new();
        let captured = sky_mesh(&mut capture, &program, 10000.)?;
        sky_uniforms.apply(&mut capture, captured, 0.)?;
        s.environment = Some(Arc::new(EnvironmentMap::from_scene(
            r,
            &mut capture,
            256,
            0.,
        )?));
        let (a, b, i) = load_asset(&format!(
            "{ASSETS}/tsl-viewport/models/gltf/LittlestTokyo.glb"
        ))
        .await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let group = s.insert(NodeKind::Group);
        let n = s.get_mut(group)?;
        n.position = Vector3::new(1., 1., 0.);
        n.scale = Vector3::splat(0.01);
        for h in instance.roots {
            s.add(group, h)?;
        }
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        self.mixer = Some(mixer);
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 0.7, 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        Ok(())
    }
    async fn shoe(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        let mut env = EnvironmentMap::from_hdr(
            &fetch(&format!("{ASSETS}/draco-variants/quarry_01_1k.hdr")).await?,
        )?;
        env.prefilter(r)?;
        s.environment = Some(Arc::new(env));
        s.background_environment = true;
        let (asset, buffers, images) = load_asset(&format!(
            "{ASSETS}/draco-variants/MaterialsVariantsShoe/MaterialsVariantsShoe.gltf"
        ))
        .await?;
        let names = asset
            .extension_value("KHR_materials_variants")
            .and_then(|v| v.get("variants"))
            .and_then(|v| v.as_array())
            .ok_or(Error::Invalid("KHR_materials_variants"))?
            .iter()
            .map(|v| v["name"].as_str().unwrap_or_default().to_owned())
            .collect::<Vec<_>>();
        let imported = crate::gltf::import_decoded(&asset, &buffers, &images)?;
        let mappings = imported.variant_materials.clone();
        let handles = imported.instantiate(s)?;
        let group = s.insert(NodeKind::Group);
        s.get_mut(group)?.scale = Vector3::splat(10.);
        let mut meshes = vec![];
        for (h, mapping) in handles.into_iter().zip(mappings) {
            s.add(group, h)?;
            if let NodeKind::Mesh(m) = &s.get(h)?.kind
                && !mapping.is_empty()
            {
                meshes.push((h, m.materials[0].clone(), mapping));
            }
        }
        let mut controls = Controls::new(None, (2., 10.), PI, true);
        controls.set_target(Vector3::new(0., 0.5, -0.2));
        controls.update(s, c)?;
        self.controls = Some(controls);
        // state.variant = 'midnight'.
        let selected = names
            .iter()
            .position(|n| n.contains("midnight"))
            .unwrap_or(0);
        self.variants = Some(Variants {
            meshes,
            names,
            selected,
            applied: None,
        });
        Ok(())
    }
    async fn face(&mut self, s: &mut Scene, _c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        s.background = Color::from_hex(0x666666);
        s.environment = Some(super::room_environment::environment(r)?);
        let (a, b, i) = load_asset(&format!("{ASSETS}/draco-variants/facecap.glb")).await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        // WebGPURenderer's standard materials conserve energy.
        for &h in &instance.meshes {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for m in &mut m.materials {
                    if let Material::Standard(m)
                    | Material::Physical(MeshPhysicalMaterial { base: m, .. }) = Arc::make_mut(m)
                    {
                        m.energy_conservation = true;
                    }
                }
            }
        }
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        self.mixer = Some(mixer);
        let mut controls = Controls::new(Some(0.05), (2.5, 5.), PI / 1.8, true);
        controls.azimuth = Some((-PI / 2., PI / 2.));
        controls.set_target(Vector3::new(0., 0.15, -0.2));
        self.controls = Some(controls);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let t = self.time;
        self.last = t;
        match self.id {
            293 => {
                let k = self.lights.as_mut().ok_or(Error::Invalid("lights"))?;
                // animate(): each light circles its center at 0.5 rad/s with a 0.5 phase step.
                for (i, (x, z)) in k.centers.iter().enumerate() {
                    let angle = 0.5 * t + i as f64 * 0.5;
                    k.data[i * 4] = (x + angle.sin() * 5.) as f32;
                    k.data[i * 4 + 2] = (z + angle.cos() * 5.) as f32;
                }
                k.buffer
                    .write(r, 0, bytemuck::cast_slice(&k.data[..LIGHTS * 4]))?;
                // One shared material, as the clones share their program and block.
                let count = k.count;
                if let NodeKind::Mesh(m) = &s.get(k.meshes[0])?.kind
                    && let Material::Shader(current) = m.materials[0].as_ref()
                    && current.uniforms[0][0] as f64 != count
                {
                    let mut material = current.clone();
                    material.uniforms[0] = [count as f32, 0., 0., 0.];
                    let material = Arc::new(Material::Shader(material));
                    for &h in &k.meshes {
                        if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                            m.materials[0] = material.clone();
                        }
                    }
                }
            }
            294 => {
                // animate(): Date.now() × 0.0003.
                let a = t * 1000. * 0.0003;
                let n = s.get_mut(c)?;
                n.position.x = a.sin() * 0.5;
                n.position.z = a.cos() * 0.5;
                s.look_at(c, Vector3::new(0., 0.1, 0.))?;
            }
            295 | 297 => {
                if let Some(m) = &mut self.mixer {
                    for a in &mut m.actions {
                        a.time = t;
                    }
                    m.update(s, 0.)?;
                }
                // One controls.update() per animation frame: Littlest Tokyo updates
                // before rendering, the facecap example after, so its frame shows
                // the updates of the frames before.
                let update = if self.id == 297 {
                    std::mem::replace(&mut self.pending, 1.) > 0.
                } else {
                    true
                };
                if update && let Some(controls) = &mut self.controls {
                    controls.update(s, c)?;
                }
            }
            296 => {
                let k = self.variants.as_mut().ok_or(Error::Invalid("variants"))?;
                if k.applied != Some(k.selected) {
                    k.applied = Some(k.selected);
                    for (h, original, mapping) in &k.meshes {
                        let material = mapping
                            .iter()
                            .find(|(variants, _)| variants.contains(&k.selected))
                            .map_or(original.clone(), |(_, m)| m.clone());
                        if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                            m.materials[0] = material;
                        }
                    }
                }
            }
            _ => {}
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
            // webgl_ubo_arrays: enablePan = false.
            if self.id == 293 {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        // Damped controls apply the input over the following frames.
        if matches!(self.id, 295 | 297) {
            return Ok(());
        }
        controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (293, 0) => {
                self.lights.as_mut().ok_or(Error::Invalid("lights"))?.count = (value as f64).round()
            }
            (296, 0) => {
                let k = self.variants.as_mut().ok_or(Error::Invalid("variants"))?;
                if (value as usize) < k.names.len() {
                    k.selected = value as usize;
                }
            }
            _ => return Err(Error::Invalid("draco/variants parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
