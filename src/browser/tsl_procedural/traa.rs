use super::super::gltf_viewer::{decode_texture_image, fetch};
use super::*;
use crate::{postprocessing::Effect, tsl::temporal::TemporalAA};
pub(in crate::browser) struct Traa {
    objects: [Object3D; 2],
    previous: [Option<Matrix4>; 2],
    target: RenderTarget,
    aa: TemporalAA,
    finish: Effect,
    index: u32,
    rotation: f64,
    seek: Option<f64>,
    started: bool,
}
impl Traa {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 0.1,
            far: 10.,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 2.5);
        s.background = Color::BLACK;
        s.background_outputs = vec![BackgroundOutput::Color, BackgroundOutput::Zero];
        let program = Arc::new(
            tsl::motion::MotionVectors {
                current_clip: tsl::motion::clip_position(
                    position_geometry(),
                    std::array::from_fn(|i| uniform(i, Type::Vec4)),
                ),
                previous_clip: tsl::motion::clip_position(
                    position_geometry(),
                    std::array::from_fn(|i| uniform(i + 4, Type::Vec4)),
                ),
            }
            .build(r, &SurfaceNodes::default(), &[], &[])
            .await?,
        );
        let geometry = Arc::new(BoxGeometry::build(1., 1., 1.)?);
        let mut a = MeshBasicMaterial::default();
        a.properties.wireframe = true;
        a.properties.vertex_program = Some(program.clone());
        let left = mesh(s, geometry.clone(), Material::Basic(a));
        s.get_mut(left)?.position.x = -1.;
        let mut texture = decode_texture_image(
            &fetch("/web/gallery/assets/tsl-next/textures/brick_diffuse.jpg").await?,
        )
        .await?;
        texture.srgb = true;
        texture.filter = Filter::Nearest;
        texture.min_filter = Some(Filter::Nearest);
        texture.mipmap_filter = None;
        let mut b = MeshBasicMaterial::default();
        b.properties.map = Some(Arc::new(texture));
        b.properties.vertex_program = Some(program);
        let right = mesh(s, geometry, Material::Basic(b));
        s.get_mut(right)?.position.x = 1.;
        Ok(Self {
            objects: [left, right],
            previous: [None, None],
            target: RenderTarget::with_options(
                &r.device,
                1,
                1,
                RenderTargetOptions {
                    format: HDR,
                    count: 2,
                    ..Default::default()
                },
            )?,
            aa: TemporalAA::new(r).await?,
            finish: effect(r, HDR, &tsl::Texture::Input.sample(uv())).await?,
            index: 0,
            rotation: 0.,
            seek: None,
            started: false,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.seek = Some(t);
    }
    pub fn update(&mut self, s: &mut Scene, _: Object3D, _: f64, animated: bool) -> Result<()> {
        if let Some(t) = self.seek.take() {
            let frames = (t * 60.).round().max(0.) as u32;
            let remainder = frames % 400;
            self.rotation =
                (frames / 400 * 200 + remainder.min(99) + remainder.saturating_sub(299)) as f64;
            self.index = frames;
        } else if animated {
            self.index += 1;
            if ((self.index as f64 / 200.).round() as u32).is_multiple_of(2) {
                self.rotation += 1.;
            }
        }
        for h in self.objects {
            s.get_mut(h)?.quaternion = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                self.rotation * 0.005,
                self.rotation * 0.01,
                0.,
            );
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
        self.target.set_size(&r.device, out.width, out.height)?;
        for root in s.roots() {
            s.update_matrix_world(root, true)?;
        }
        let world = s.get(c)?.matrix_world;
        let NodeKind::Camera(Camera::Perspective(camera)) = &mut s.get_mut(c)?.kind else {
            return Err(Error::Invalid("TRAA camera"));
        };
        camera.aspect = out.width as f64 / out.height as f64;
        camera.view = None;
        let projection = camera.projection_matrix()?;
        camera.view = self
            .started
            .then(|| self.aa.view_offset(out.width, out.height));
        self.started = true;
        let jittered = camera.projection_matrix()?;
        for (i, h) in self.objects.iter().enumerate() {
            let current = projection * world.inverse() * s.get(*h)?.matrix_world;
            let previous = self.previous[i].unwrap_or(current);
            if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind {
                let p = Arc::make_mut(&mut m.materials[0]).properties_mut();
                for (slot, value) in current
                    .to_cols_array_2d()
                    .into_iter()
                    .chain(previous.to_cols_array_2d())
                    .enumerate()
                {
                    p.vertex_uniforms[slot] = value.map(|v| v as f32);
                }
            }
            self.previous[i] = Some(current);
        }
        let rendered = r.render(s, c, &self.target);
        if let NodeKind::Camera(Camera::Perspective(camera)) = &mut s.get_mut(c)?.kind {
            camera.view = None;
        }
        rendered?;
        self.aa
            .apply(r, &self.target, world, jittered, [0.1, 10.], false)?;
        self.finish.apply(r, self.aa.output(), None, out)?;
        Ok(true)
    }
}
