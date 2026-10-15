//! webgl_materials_physical_transmission_alpha: the DragonAttenuation glTF
//! ( its cloth backdrop and the transmissive, volume-attenuated dragon ) lit
//! only by the royal esplanade UltraHDR environment under ACES Filmic tone
//! mapping, drawn into an alpha canvas over the page's coloured table. The
//! transmission pass renders the backdrop over a transparent clear, so the
//! dragon's transmissionAlpha lets the page show through where it refracts
//! nothing. The GUI edits the dragon's material and the exposure; OrbitControls
//! render on demand.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{fetch, load_asset};
use super::probes_hdr::ultra_hdr;
use crate::{Error, Result, camera::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";

pub struct Demo {
    controls: Controls,
    dragon: Object3D,
    /// color, transmission, opacity, metalness, roughness, ior, thickness,
    /// attenuationColor, attenuationDistance, specularIntensity, specularColor,
    /// envMapIntensity, exposure.
    params: [f64; 13],
    dirty: bool,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 1.,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-5., 0.5, 0.);
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 1.;
        // WebGLRenderer( { alpha: true } ) without a background: a transparent clear.
        s.background = Color::BLACK;
        s.background_alpha = 0.;
        let env =
            ultra_hdr(&fetch(&format!("{ASSETS}/probes-hdr/royal_esplanade_2k.hdr.jpg")).await?)
                .await?;
        s.environment = Some(Arc::new(env));
        let (a, b, i) = load_asset(&format!(
            "{ASSETS}/tsl-next/models/gltf/DragonAttenuation.glb"
        ))
        .await?;
        let model = crate::gltf::import_decoded(&a, &b, &i)?.instantiate(s)?;
        let mut dragon = None;
        let mut params = [0.; 13];
        // WebGL's image-based lighting also scales the diffuse by the multiple-scattering
        // specular ( RE_IndirectSpecular_Physical ); no punctual lights are present.
        for &h in &model {
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
        for &h in &model {
            if let NodeKind::Mesh(mesh) = &s.get(h)?.kind
                && let Material::Physical(m) = mesh.materials[0].as_ref()
            {
                params = [
                    f64::from(m.base.properties.color.to_hex()),
                    m.transmission,
                    1.,
                    m.base.metalness,
                    m.base.roughness,
                    m.ior,
                    m.thickness,
                    f64::from(m.attenuation_color.to_hex()),
                    m.attenuation_distance,
                    m.specular_intensity,
                    f64::from(0xffffff),
                    1.,
                    1.,
                ];
                dragon = Some(h);
            }
        }
        let dragon = dragon.ok_or(Error::Invalid("DragonAttenuation physical mesh"))?;
        let mut controls = Controls::new(None, (5., 20.), PI, true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
        controls.update(s, c)?;
        Ok(Self {
            controls,
            dragon,
            params,
            dirty: false,
        })
    }
    fn apply(&self, s: &mut Scene) -> Result<()> {
        let p = self.params;
        s.exposure = p[12];
        if let NodeKind::Mesh(mesh) = &mut s.get_mut(self.dragon)?.kind
            && let Material::Physical(m) = Arc::make_mut(&mut mesh.materials[0])
        {
            m.base.properties.color = Color::from_hex(p[0] as u32);
            m.transmission = p[1];
            m.base.properties.opacity = p[2];
            // transparent follows opacity < 1, as the page's onChange sets it.
            m.base.properties.transparent = p[2] < 1.;
            m.base.metalness = p[3];
            m.base.roughness = p[4];
            m.ior = p[5];
            m.thickness = p[6];
            m.attenuation_color = Color::from_hex(p[7] as u32);
            m.attenuation_distance = p[8];
            m.specular_intensity = p[9];
            m.specular_color = Color::from_hex(p[10] as u32);
            m.base.env_map_intensity = p[11];
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.dirty) {
            self.apply(s)?;
        }
        self.controls.update(s, c)
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
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if let Some(p) = self.params.get_mut(index) {
            *p = f64::from(value);
            self.dirty = true;
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
