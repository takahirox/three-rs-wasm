//! webgl_materials_envmaps_exr: the PIZ-compressed EXR and the PNG
//! equirectangular maps, each prefiltered once into a PMREM for the
//! reflective torus knot and shown as the background, with ACES exposure,
//! roughness, metalness and the debug plane showing the PMREM atlas.
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::refraction_loaders::formats::decode_exr;
use crate::environment::EnvironmentMap;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/envmap-exr";
/// The debug plane's MeshBasicMaterial with the PMREM render target as its
/// map: the port's own cube-UV atlas at the plane's uv. Its layout is not
/// three's, so the debug view is not compared.
async fn atlas_material(r: &Renderer, env: &mut EnvironmentMap) -> Result<Arc<Material>> {
    let gpu = env.prefilter(r)?;
    let color = WgslFn::new(
        "atlas",
        "fn atlas()->vec4<f32>{return vec4(textureSample(tsl_texture_0,tsl_sampler_0,fragment_surface.uv).rgb,1.0);}",
        &[],
        Type::Vec4,
    )?
    .call(&[]);
    let program = ShaderProgram::with_projection(
        r,
        &NodeMaterial::new(color).wgsl(1)?,
        &[],
        &[(gpu.view(), gpu.sampler())],
        "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
    )
    .await?;
    Ok(Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(
        program,
    )))))
}
pub(super) struct Demo {
    time: f64,
    last: f64,
    torus: Object3D,
    plane: Object3D,
    /// EXR, PNG: the environments and the debug plane materials.
    environments: [Arc<EnvironmentMap>; 2],
    planes: [Arc<Material>; 2],
    /// envMap (0 EXR, 1 PNG), roughness, metalness, exposure, debug.
    params: [f64; 5],
    rotation: f64,
    controls: Controls,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 120.);
        n.quaternion = Quaternion::IDENTITY;
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 1.;
        let torus = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(18., 8., 150, 20, 2, 3)?),
            Arc::new(Material::Standard(MeshStandardMaterial {
                metalness: 0.,
                roughness: 0.,
                ..Default::default()
            })),
        )));
        // EXRLoader: HalfFloat RGBA rows from the bottom; the environment
        // stores rows from the top, as an image does.
        let (width, height, texels) =
            decode_exr(&fetch(&format!("{ASSETS}/piz_compressed.exr")).await?)?;
        let row = width as usize * 4;
        let rgba = texels
            .chunks(row)
            .rev()
            .flatten()
            .map(|&h| half::f16::from_bits(h))
            .collect();
        let mut exr = EnvironmentMap {
            width,
            height,
            rgba,
            gpu: None,
        };
        let mut png =
            decode_texture_image(&fetch(&format!("{ASSETS}/equirectangular.png")).await?).await?;
        png.srgb = true;
        let mut png = EnvironmentMap::from_texture(&png)?;
        let planes = [
            atlas_material(r, &mut exr).await?,
            atlas_material(r, &mut png).await?,
        ];
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(200., 200., 1, 1)?),
            planes[0].clone(),
        )));
        let n = s.get_mut(plane)?;
        n.position.y = -50.;
        n.quaternion = Quaternion::from_rotation_x(-PI * 0.5);
        n.visible = false;
        let environments = [Arc::new(exr), Arc::new(png)];
        s.environment = Some(environments[0].clone());
        s.background_environment = true;
        // OrbitControls: distances 50–300.
        let mut controls = Controls::new(None, (50., 300.), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            time: 0.,
            last: 0.,
            torus,
            plane,
            environments,
            planes,
            params: [0., 0., 0., 1., 0.],
            rotation: 0.,
            controls,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// render(): the selected environment and background, the material
    /// settings, the knot's 0.005 per-frame turn (as 60 fps steps) and the
    /// debug plane.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let steps = ((self.time - self.last) * 60.).round().max(0.) as usize;
        self.last = self.time;
        self.rotation += 0.005 * steps as f64;
        let [map, roughness, metalness, exposure, debug] = self.params;
        let index = usize::from(map > 0.5);
        if !s
            .environment
            .as_ref()
            .is_some_and(|e| Arc::ptr_eq(e, &self.environments[index]))
        {
            s.environment = Some(self.environments[index].clone());
        }
        s.exposure = exposure;
        // The sRGB PNG background is not tone mapped; the linear EXR is.
        s.background_tone_mapped = index == 0;
        let n = s.get_mut(self.torus)?;
        n.quaternion = Quaternion::from_rotation_y(self.rotation);
        if let NodeKind::Mesh(m) = &mut n.kind
            && let Material::Standard(m) = Arc::make_mut(&mut m.materials[0])
            && (m.roughness != roughness || m.metalness != metalness)
        {
            m.roughness = roughness;
            m.metalness = metalness;
        }
        let n = s.get_mut(self.plane)?;
        n.visible = debug > 0.5;
        if let NodeKind::Mesh(m) = &mut n.kind
            && !Arc::ptr_eq(&m.materials[0], &self.planes[index])
        {
            m.materials[0] = self.planes[index].clone();
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
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let p = self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("envmap parameter"))?;
        *p = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
