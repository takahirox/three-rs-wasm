use super::super::gltf_viewer::{decode_texture_image, fetch};
use super::*;
use crate::{
    compute::{BufferAccess, GpuBuffer},
    postprocessing::Effect,
};
pub(in crate::browser) struct Tree {
    viewer: OrbitViewer,
    time: f64,
    previous: f64,
    auto_delta: f64,
    pub dragging: bool,
    tree: Object3D,
    floor: Object3D,
    camera: Object3D,
    reflected: RenderTarget,
    scene: RenderTarget,
    horizontal: RenderTarget,
    vertical: RenderTarget,
    blur: Effect,
    finish: Effect,
    normal: GpuTexture,
    sampler: wgpu::Sampler,
}
fn reflection_target(r: &Renderer, w: u32, h: u32) -> Result<RenderTarget> {
    RenderTarget::with_options(
        &r.device,
        w,
        h,
        RenderTargetOptions {
            format: HDR,
            ..Default::default()
        },
    )
}
impl Tree {
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
        let center = Vector3::new(0., 1., 0.);
        let p = Vector3::new(4., 2., 4.) - center;
        let mut viewer = OrbitViewer::from_camera(center, p.length());
        viewer.fixture(p.x.atan2(p.z), (p.y / p.length()).asin(), 1.8);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x4195a4),
            near: 1.,
            far: 20.,
        });
        s.tone_mapping = ToneMapping::Aces;
        s.shadow_map_size = 1024;
        let sky = mesh(
            s,
            Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
            Material::Shader(
                background_material(
                    r,
                    vec4(
                        mix(rgb(0x4195a4), rgb(0x0066ff), -sky_direction().y()),
                        float(1.),
                    ),
                )
                .await?,
            ),
        );
        s.get_mut(sky)?.frustum_culled = false;
        s.get_mut(sky)?.render_order = -100;
        for (pos, color, intensity, shadow) in [
            (Vector3::new(7., 5., 7.), 0xffe499, 2., true),
            (Vector3::new(7., -5., 7.), 0x0487e2, 0.5, false),
        ] {
            let h = s.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(color),
                intensity,
                target: Vector3::ZERO,
            }));
            let n = s.get_mut(h)?;
            n.position = pos;
            n.cast_shadow = shadow;
            n.shadow.extent = 5. / 1.5;
            n.shadow.bias = -0.0001;
        }
        let data = fetch("/web/gallery/assets/tsl-procedural/reflection-tree.bin").await?;
        let count = u32::from_le_bytes(data[..4].try_into().unwrap());
        let buffer = GpuBuffer::new(r, &data[4..], BufferAccess::Read)?;
        let i = instance_index() * uint(4);
        let origin = storage_element(0, i.clone()).rgb();
        let normal = storage_element(0, i.clone() + uint(1)).rgb();
        let color = storage_element(0, i.clone() + uint(2)).rgb();
        let data = storage_element(0, i + uint(3));
        let size = data.x();
        let phase = data.y();
        let seed = data.swizzle("z");
        let dif1 = (phase.clone() - uniform(1, Type::Float)).abs();
        let dif2 = (phase.clone() - uniform(2, Type::Float)).abs();
        let effect1 = dif1.less_than(float(0.15)).select(
            (float(0.15) - dif1) * (float(1.7) - phase.clone()) * float(10.),
            float(0.),
        );
        let effect2 = dif2.less_than(float(0.15)).select(
            (float(0.15) - dif2) * (float(1.7) - phase) * float(10.),
            effect1.clone(),
        );
        let direction = position_geometry().normalize();
        let position = position_geometry() + origin + direction.clone() * (effect2.clone() + size)
            - direction * effect2.clone()
            + normal.clone() * effect2.clone()
            + normal * ((time() + seed * float(2.)).sin() * float(1.5)).abs();
        let local = uv() - float(0.5);
        let square = local.x().abs().max(local.y().abs()) / float(0.5);
        let mut material = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        material.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                position: Some(position.clone()),
                color: Some(
                    (square.clamp(float(0.85), float(1.)) - float(0.5)) * float(2.) - color.clone(),
                ),
                emissive: Some(vec3(effect1, float(0.), effect2).pow(float(2.)) * color),
                ..Default::default()
            }
            .build(r, &[(&buffer, Type::Vec4)], &[])
            .await?,
        ));
        material.properties.shadow_program = Some(Arc::new(
            shadow_program_with_storage(r, Some(position), None, &[(&buffer, Type::Vec4)]).await?,
        ));
        let mut g = BoxGeometry::build(1., 1., 1.)?;
        g.instance_count = Some(count);
        let tree = mesh(s, Arc::new(g), Material::Standard(material));
        let n = s.get_mut(tree)?;
        n.scale = Vector3::splat(0.05);
        n.frustum_culled = false;
        n.cast_shadow = true;
        n.receive_shadow = true;
        let mut textures = Vec::new();
        for (name, srgb) in [
            ("FloorsCheckerboard_S_Diffuse.jpg", true),
            ("FloorsCheckerboard_S_Normal.jpg", false),
        ] {
            let mut t = decode_texture_image(
                &fetch(&format!("/web/gallery/assets/tsl-procedural/{name}")).await?,
            )
            .await?;
            t.srgb = srgb;
            t.wrap_s = Wrapping::Repeat;
            t.wrap_t = Wrapping::Repeat;
            t.mipmap_filter = Some(Filter::Linear);
            t.repeat = Vector2::splat(15.);
            textures.push(Arc::new(t));
        }
        let normal = r.upload_texture(&textures[1])?;
        let reflected = reflection_target(r, 1, 1)?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let screen = tsl::viewport::screen_uv();
        let texuv = uv() * float(15.);
        let offset = (tsl::Texture::External(1)
            .sample(vec2(texuv.x(), float(1.) - texuv.y()))
            .swizzle("xy")
            * float(2.)
            - float(1.))
            * float(0.02);
        let coordinate = vec2(float(1.) - screen.x(), screen.y()) + offset;
        let mut floor_material = MeshPhongMaterial {
            normal_map: Some(textures[1].clone()),
            normal_scale: Vector2::new(0.2, -0.2),
            ..Default::default()
        };
        floor_material.properties.map = Some(textures[0].clone());
        floor_material.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                emissive: Some(tsl::Texture::External(0).sample(coordinate).rgb() * float(0.25)),
                ..Default::default()
            }
            .build(
                r,
                &[],
                &[(&reflected.view, &sampler), (&normal.view, &normal.sampler)],
            )
            .await?,
        ));
        let floor = mesh(
            s,
            Arc::new(BoxGeometry::build(50., 0.001, 50.)?),
            Material::Phong(floor_material),
        );
        s.get_mut(floor)?.receive_shadow = true;
        let scene = target(r, 1, 1)?;
        let horizontal = reflection_target(r, 1, 1)?;
        let vertical = reflection_target(r, 1, 1)?;
        let depth = depth_texture(uv());
        let viewz = float(0.25 * 30.) / (float(30.) - depth * float(30. - 0.25));
        let linear = (viewz - float(0.25)) / float(30. - 0.25);
        let strength = ((linear - float(0.3)) / float(0.4)).clamp(float(0.), float(1.));
        let blur = tsl::multisampled_depth_effect(
            r,
            HDR,
            &gaussian_blur(
                tsl::Texture::Input,
                uv(),
                uniform(0, Type::Vec2) * strength,
                4,
            )?,
            scene.depth_view.as_ref().unwrap(),
        )
        .await?;
        let vignette = (float(1.)
            - ((uv() - float(0.5)).length() * float(1.25)).clamp(float(0.), float(1.)))
            - float(0.2);
        let overlay=WgslFn::new("tree_overlay","fn tree_overlay(base:vec3<f32>,blend:f32)->vec3<f32>{return select(1.0-2.0*(1.0-base)*(1.0-blend),2.0*base*blend,base<vec3(0.5));}",&[Type::Vec3,Type::Float],Type::Vec3)?.call(&[tsl::Texture::Input.sample(uv()).rgb(),vignette]);
        let finish = tsl::effect(r, HDR, &vec4(overlay, float(1.))).await?;
        let camera = s.insert(s.get(c)?.kind.clone());
        s.get_mut(camera)?.matrix_auto_update = false;
        Ok(Self {
            viewer,
            time: 0.,
            previous: 0.,
            auto_delta: 0.,
            dragging: false,
            tree,
            floor,
            camera,
            reflected,
            scene,
            horizontal,
            vertical,
            blur,
            finish,
            normal,
            sampler,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
        }
        if !self.dragging {
            self.auto_delta += (self.time - self.previous) / 60.;
        }
        self.viewer
            .orbit_pixels(self.auto_delta * 0.05, 0., 0., 1., 1., 10.);
        self.auto_delta *= 0.95;
        self.previous = self.time;
        self.viewer
            .limit_pitch(0., std::f64::consts::FRAC_PI_2 - 1e-6);
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.25;
            p.far = 30.;
        }
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.tree)?.kind {
            let u = &mut Arc::make_mut(&mut m.materials[0])
                .properties_mut()
                .vertex_uniforms;
            u[0][0] = self.time as f32;
            for (i, delay) in [0.8, 0.].into_iter().enumerate() {
                let phase = ((self.time - delay).rem_euclid(3. + delay) / 3.).min(1.);
                u[i + 1][0] = if self.time < delay {
                    -0.2
                } else {
                    (-0.2 + 1.4 * (0.5 - 0.5 * (phase * std::f64::consts::PI).cos())) as f32
                };
            }
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
        let (w, h) = (
            (out.width as f64 * 0.2).round() as u32,
            (out.height as f64 * 0.2).round() as u32,
        );
        let (w, h) = (w.max(1), h.max(1));
        if self.reflected.width != w || self.reflected.height != h {
            self.reflected = reflection_target(r, w, h)?;
            if let NodeKind::Mesh(m) = &mut s.get_mut(self.floor)?.kind {
                Arc::make_mut(
                    Arc::make_mut(&mut m.materials[0])
                        .properties_mut()
                        .vertex_program
                        .as_mut()
                        .unwrap(),
                )
                .rebind(
                    r,
                    &[],
                    &[
                        (&self.reflected.view, &self.sampler),
                        (&self.normal.view, &self.normal.sampler),
                    ],
                )?;
            }
        }
        if self.scene.width != out.width || self.scene.height != out.height {
            self.scene = target(r, out.width, out.height)?;
            self.horizontal = reflection_target(r, out.width, out.height)?;
            self.vertical = reflection_target(r, out.width, out.height)?;
            self.blur
                .set_depth(r, self.scene.depth_view.as_ref().unwrap())?;
        }
        s.update_world_matrix(c, true, false)?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(camera) = camera else {
            return Err(Error::Invalid("tree camera"));
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
        }
        r.render(s, c, &self.scene)?;
        self.blur.parameters[0] = [1. / out.width as f32, 0., 0., 0.];
        self.blur.apply(r, &self.scene, None, &self.horizontal)?;
        self.blur.parameters[0] = [0., 1. / out.height as f32, 0., 0.];
        self.blur.apply(r, &self.horizontal, None, &self.vertical)?;
        self.finish.apply(r, &self.vertical, None, out)?;
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
        // OrbitControls calls update() without delta on a wheel event.
        if w != 0. && !self.dragging {
            self.auto_delta += 1. / 3600.;
        }
        if p {
            self.viewer.pan_pixels(s, c, dx, dy, h)?;
        } else {
            self.viewer.orbit_pixels(dx, dy, w, h, 1., 10.);
        }
        Ok(())
    }
}
