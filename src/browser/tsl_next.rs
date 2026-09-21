mod lut;
mod path;
mod room;
mod smaa;
mod sobel;
// Five r186 node scenes: GPU path deformation, Sobel, SMAA, LUT and parallax.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch};
use crate::{
    Error, Result,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, surface::*, *},
};
use std::sync::Arc;
const ASSETS: &str = "/web/gallery/assets/tsl-next";
pub(super) struct Demo {
    lut: Option<lut::Lut>,
    sobel: Option<sobel::Sobel>,
    smaa: Option<smaa::Smaa>,
    example: u32,
    time: f64,
    clock: f64,
    rotation: f64,
    dragging: bool,
    params: [f32; 16],
    objects: Vec<Object3D>,
    viewer: OrbitViewer,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, example: u32, r: &Renderer) -> Result<Self> {
        let (fov, near, far, position, target) = match example {
            98 => (
                60.0,
                0.01,
                100.0,
                Vector3::new(0.0, 0.0, 15.0),
                Vector3::ZERO,
            ),
            99 => (
                70.0,
                0.1,
                100.0,
                Vector3::new(0.0, 1.0, 3.0),
                Vector3::new(0.0, 0.5, 0.0),
            ),
            100 => (
                70.0,
                1.0,
                1000.0,
                Vector3::new(0.0, 0.0, 300.0),
                Vector3::ZERO,
            ),
            101 => (
                25.0,
                0.1,
                100.0,
                Vector3::new(8.0, 10.0, 12.0),
                Vector3::new(0.0, 3.0, 0.0),
            ),
            102 => (
                45.0,
                0.1,
                100.0,
                Vector3::new(15.0, 7.0, 15.0),
                Vector3::ZERO,
            ),
            _ => return Err(Error::Invalid("TSL next example")),
        };
        let aspect = if let Camera::Perspective(p) = s.camera(c)?.0 {
            p.aspect
        } else {
            1.0
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = position;
        s.look_at(c, target)?;
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        let mut out = Self {
            lut: None,
            sobel: None,
            smaa: None,
            example,
            time: 0.0,
            clock: 0.0,
            rotation: 0.0,
            dragging: false,
            params: [1.0; 16],
            objects: vec![],
            viewer,
        };
        s.background = Color::BLACK;
        match example {
            98 => out.path(s, r).await?,
            99 => out.sobel(s, r).await?,
            100 => out.smaa(s, r).await?,
            101 => out.lut(s, r).await?,
            102 => out.parallax(s, r).await?,
            _ => return Err(Error::Invalid("TSL scene not built")),
        };
        Ok(out)
    }
    async fn parallax(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_hdr(
            &fetch(&format!(
                "{ASSETS}/textures/equirectangular/752-hdri-skies-com_1k.hdr"
            ))
            .await?,
        )?));
        s.background_environment = true;
        s.background_blur = 0.4;
        s.tone_mapping = ToneMapping::Reinhard;
        s.exposure = 6.0;
        let mut textures = vec![];
        let mut top = None;
        for (name, srgb) in [
            ("Ice002_1K-JPG_Color.jpg", true),
            ("Ice003_1K-JPG_Color.jpg", true),
            ("Ice002_1K-JPG_Roughness.jpg", false),
            ("Ice002_1K-JPG_NormalGL.jpg", false),
            ("Ice002_1K-JPG_Displacement.jpg", false),
        ] {
            let mut image =
                decode_image(&fetch(&format!("{ASSETS}/textures/ambientcg/{name}")).await?).await?;
            image.srgb = srgb;
            image.wrap_s = Wrapping::Repeat;
            image.wrap_t = Wrapping::Repeat;
            image.mipmap_filter = Some(Filter::Linear);
            let image = Arc::new(image);
            if name == "Ice002_1K-JPG_Color.jpg" {
                top = Some(image.clone());
            }
            textures.push(r.upload_texture(&image)?);
        }
        let bindings = textures[1..]
            .iter()
            .map(|t| (&t.view, &t.sampler))
            .collect::<Vec<_>>();
        let tex = |i: usize, p: tsl::Node| {
            (if i == 0 {
                tsl::Texture::Map
            } else {
                tsl::Texture::External(i - 1)
            })
            .sample(vec2(p.x(), float(1.0) - p.y()))
        };
        let scaled = uv() * uniform(0, Type::Vec2).y();
        let mapped_normal = normal_map(tex(3, scaled.clone()).rgb(), uv());
        let offset = parallax_uv_frame(
            scaled.clone(),
            tex(4, scaled.clone()).swizzle("xy") * uniform(0, Type::Vec2).x(),
            mapped_normal.clone(),
            uv(),
        );
        let graph = SurfaceNodes {
            color: Some(
                blend_overlay(tex(0, scaled.clone()).rgb(), tex(1, offset).rgb()) * float(5.0),
            ),
            roughness: Some(tex(2, scaled.clone()).x()),
            normal: Some(mapped_normal),
            ..Default::default()
        };
        let mut mat = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        mat.properties.map = top;
        mat.properties.vertex_program = Some(Arc::new(graph.build(r, &[], &bindings).await?));
        let geometry = CircleGeometry::build(25.0, 64, 0.0, std::f64::consts::TAU)?;
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Standard(mat)),
        )));
        s.get_mut(h)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        self.objects.push(h);
        self.params[..3].copy_from_slice(&[0.4, 0.5, 3.0]);
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
            if self.params[1] > 0.5 {
                self.rotation += delta;
            }
        }
        if self.example == 100 && self.params[1] > 0.5 {
            for h in &self.objects {
                s.get_mut(*h)?.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    self.rotation * 0.3,
                    self.rotation * 0.6,
                    0.0,
                );
            }
        }
        if self.example == 101 {
            for h in &self.objects {
                if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind
                    && let Material::Shader(mat) = Arc::make_mut(&mut m.materials[0])
                {
                    mat.uniforms[0][0] = self.time as f32;
                }
            }
        }
        if self.example == 98 {
            for h in &self.objects {
                if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                    Arc::make_mut(&mut m.materials[0])
                        .properties_mut()
                        .vertex_uniforms[0][0] = self.time as f32;
                }
            }
        }
        if self.example == 102 {
            // OrbitControls autoRotateSpeed=-1 advances 2pi/60 radians per second.
            if !self.dragging {
                self.viewer.orbit_pixels(
                    -(self.time - self.clock) / 60.0,
                    0.0,
                    0.0,
                    1.0,
                    10.0,
                    40.0,
                );
            }
            s.background_blur = self.params[0] as f64;
            for h in &self.objects {
                if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                    Arc::make_mut(&mut m.materials[0])
                        .properties_mut()
                        .vertex_uniforms[0] = [self.params[1], self.params[2], 0.0, 0.0];
                }
            }
        }
        self.clock = self.time;
        self.viewer.update(s, c)
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        if self.params[1] > 0.5 {
            self.rotation = t;
        }
    }
    pub fn dragging(&mut self, value: bool) {
        self.dragging = value;
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= match self.example {
            98 => 0,
            99 => 1,
            100 | 101 => 2,
            102 => 3,
            _ => 0,
        } || !v.is_finite()
        {
            return Err(Error::Invalid("TSL next parameter"));
        }
        self.params[i] = v;
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        target: &RenderTarget,
    ) -> Result<bool> {
        if let Some(p) = &mut self.lut {
            p.render(r, s, c, target, self.params[0] as usize, self.params[1])?;
            return Ok(true);
        }
        if let Some(p) = &mut self.smaa {
            p.render(r, s, c, target, self.params[0] > 0.5)?;
            return Ok(true);
        }
        if let Some(p) = &mut self.sobel {
            p.render(r, s, c, target, self.params[0] > 0.5)?;
            return Ok(true);
        }
        Ok(false)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if pan {
            return self.viewer.pan_pixels(s, c, dx, dy, height);
        }
        let (min, max) = match self.example {
            102 => (10.0, 40.0),
            101 => (0.1, 50.0),
            _ => (0.0, f64::INFINITY),
        };
        // OrbitControls' wheel handler also calls update() without a delta.
        // With auto-rotation enabled this contributes its default 1/60 second.
        if self.example == 102 && wheel != 0.0 && !self.dragging {
            self.viewer
                .orbit_pixels(-1.0 / 3600.0, 0.0, 0.0, 1.0, min, max);
        }
        self.viewer.orbit_pixels(
            dx,
            dy,
            if self.example == 99 { 0.0 } else { wheel },
            height,
            min,
            max,
        );
        Ok(())
    }
}
