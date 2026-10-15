use super::super::gltf_viewer::{decode_texture_image, fetch};
use super::*;
use crate::{environment::EnvironmentMap, mipmap::MipGenerator};
pub(in crate::browser) struct Reflection {
    time: f64,
    previous: f64,
    viewer: OrbitViewer,
    floor: Object3D,
    camera: Object3D,
    reflected: RenderTarget,
    mips: MipGenerator,
    sampled: wgpu::TextureView,
    sampler: wgpu::Sampler,
    perlin: GpuTexture,
}
fn reflection_target(
    r: &Renderer,
    w: u32,
    h: u32,
) -> Result<(RenderTarget, MipGenerator, wgpu::TextureView)> {
    let mut target = RenderTarget::with_options(
        &r.device,
        w,
        h,
        RenderTargetOptions {
            format: HDR,
            ..Default::default()
        },
    )?;
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("mipped planar reflection"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: w.max(h).ilog2() + 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: HDR,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let mips = MipGenerator::new(&r.device, &texture)?;
    let view = texture.create_view(&Default::default());
    target.set_texture(0, texture)?;
    Ok((target, mips, view))
}
impl Reflection {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            aspect,
            near: 0.25,
            far: 30.,
            ..Default::default()
        }));
        let p = Vector3::new(-4., 1., 4.);
        let center = Vector3::new(0., 0.75, 0.);
        s.get_mut(c)?.position = p;
        s.look_at(c, center)?;
        let offset = p - center;
        let mut viewer = OrbitViewer::from_camera(center, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        s.tone_mapping = ToneMapping::Neutral;
        s.exposure = 1.5;
        let bytes =
            fetch("/web/gallery/assets/tsl-lighting/spruit_sunrise_2k.hdr.jpg.rgba16f.png").await?;
        let im = image::load_from_memory(&bytes)
            .map_err(|e| Error::Asset(e.to_string()))?
            .to_rgba16();
        let (width, height) = im.dimensions();
        s.environment = Some(Arc::new(EnvironmentMap {
            width,
            height,
            rgba: im
                .as_raw()
                .iter()
                .map(|v| half::f16::from_bits(*v))
                .collect(),
            gpu: None,
        }));
        s.background_environment = true;
        let mut grid = decode_texture_image(
            &fetch("/web/gallery/assets/tsl-procedural/uv_grid_directx.jpg").await?,
        )
        .await?;
        grid.srgb = true;
        grid.mipmap_filter = Some(Filter::Linear);
        let grid = Arc::new(grid);
        let mut m = MeshStandardMaterial {
            emissive: Color::WHITE,
            energy_conservation: true,
            ..Default::default()
        };
        m.properties.map = Some(grid.clone());
        m.metallic_roughness_map = Some(grid.clone());
        m.emissive_map = Some(grid);
        let h = mesh(
            s,
            Arc::new(BoxGeometry::build(1., 1., 1.)?),
            Material::Standard(m),
        );
        s.get_mut(h)?.position.y = 1.25;
        s.get_mut(h)?.scale = Vector3::splat(2.);
        let mut t =
            decode_texture_image(&fetch("/web/gallery/assets/flames-rgb-256x256.png").await?)
                .await?;
        t.srgb = true;
        t.wrap_s = Wrapping::Repeat;
        t.wrap_t = Wrapping::Repeat;
        t.mipmap_filter = Some(Filter::Linear);
        let perlin = r.upload_texture(&Arc::new(t))?;
        let (reflected, mips, sampled) = reflection_target(r, 1, 1)?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let coordinate = uv() * float(10.) + vec2(time() * float(0.1), float(0.));
        let roughness = (tsl::Texture::External(1)
            .sample(vec2(coordinate.x(), float(1.) - coordinate.y()))
            .x()
            * float(2.))
        .clamp(float(0.), float(1.));
        let screen = tsl::viewport::screen_uv();
        let reflection = tsl::sampling::texture_bicubic_level(
            tsl::Texture::External(0),
            vec2(float(1.) - screen.x(), screen.y()),
            roughness.clone() * float(0.9) * uniform(1, Type::Float),
        );
        let opacity = float(1.) - view_z().smoothstep(float(7.), float(25.));
        let program = SurfaceNodes {
            color: Some(vec4(reflection.rgb(), opacity)),
            roughness: Some(roughness * float(0.2)),
            output: Some(output().max(float(0.))),
            ..Default::default()
        }
        .build(
            r,
            &[],
            &[(&sampled, &sampler), (&perlin.view, &perlin.sampler)],
        )
        .await?;
        let mut m = MeshStandardMaterial {
            metalness: 1.,
            energy_conservation: true,
            ..Default::default()
        };
        m.properties.transparent = true;
        m.properties.vertex_program = Some(Arc::new(program));
        let floor = mesh(
            s,
            Arc::new(BoxGeometry::build(50., 0.001, 50.)?),
            Material::Standard(m),
        );
        let camera = s.insert(s.get(c)?.kind.clone());
        s.get_mut(camera)?.matrix_auto_update = false;
        Ok(Self {
            time: 0.,
            previous: 0.,
            viewer,
            floor,
            camera,
            reflected,
            mips,
            sampled,
            sampler,
            perlin,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
        }
        self.viewer
            .orbit_pixels(-(self.time - self.previous) / 600., 0., 0., 1., 1., 10.);
        self.previous = self.time;
        self.viewer
            .limit_pitch(0., std::f64::consts::FRAC_PI_2 - 1e-6);
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.25;
            p.far = 30.;
        }
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.floor)?.kind {
            Arc::make_mut(&mut m.materials[0])
                .properties_mut()
                .vertex_uniforms[0][0] = self.time as f32;
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
        let (w, h) = (out.width.div_ceil(2).max(1), out.height.div_ceil(2).max(1));
        if self.reflected.width != w || self.reflected.height != h {
            (self.reflected, self.mips, self.sampled) = reflection_target(r, w, h)?;
            if let NodeKind::Mesh(m) = &mut s.get_mut(self.floor)?.kind {
                let p = Arc::make_mut(&mut m.materials[0]).properties_mut();
                p.vertex_uniforms[1][0] = w.max(h).ilog2() as f32;
                Arc::make_mut(p.vertex_program.as_mut().unwrap()).rebind(
                    r,
                    &[],
                    &[
                        (&self.sampled, &self.sampler),
                        (&self.perlin.view, &self.perlin.sampler),
                    ],
                )?;
            }
        }
        s.update_world_matrix(c, true, false)?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(camera) = camera else {
            return Err(Error::Invalid("reflection camera"));
        };
        if let Some((camera, matrix)) =
            crate::reflection::planar_camera(camera, world, Vector4::new(0., 1., 0., 0.))?
        {
            let n = s.get_mut(self.camera)?;
            n.kind = NodeKind::Camera(Camera::Perspective(camera));
            n.matrix = matrix;
            n.matrix_world_needs_update = true;
            s.get_mut(self.floor)?.visible = false;
            let result = r.render(s, self.camera, &self.reflected);
            s.get_mut(self.floor)?.visible = true;
            result?;
            self.mips.update(&r.device, &r.queue);
        }
        r.render(s, c, out)?;
        Ok(true)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        w: f64,
        p: bool,
        h: f64,
    ) -> Result<()> {
        if p {
            self.viewer.pan_pixels(s, c, dx, dy, h)?;
        } else {
            self.viewer.orbit_pixels(dx, dy, w, h, 1., 10.);
        }
        Ok(())
    }
}
