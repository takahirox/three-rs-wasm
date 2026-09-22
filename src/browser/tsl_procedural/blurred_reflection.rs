use super::super::gltf_viewer::{decode_texture_image, fetch};
use super::*;
pub(in crate::browser) struct BlurredReflection {
    viewer: OrbitViewer,
    time: f64,
    mixer: crate::animation::AnimationMixer,
    objects: Vec<Object3D>,
    floor: Object3D,
    camera: Object3D,
    reflected: RenderTarget,
    sampler: wgpu::Sampler,
    depth_sampler: wgpu::Sampler,
    parameters: [f32; 3],
}
fn target(r: &Renderer, w: u32, h: u32) -> Result<RenderTarget> {
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
fn circle() -> tsl::Node {
    let p = position_world();
    let radius = p.swizzle("xz").length();
    let distance = (radius.clone() * float(0.1) * float(5.) - (time() * float(0.5)).fract())
        .fract()
        - float(0.5);
    let strength = (float(0.5) / distance.abs()).pow(float(0.8))
        * float(0.01)
        * (float(0.8) - distance.abs()).max(float(0.));
    let color = mix(
        rgb(0x74ccf4),
        rgb(0x7f00c5),
        (radius / float(10.)).clamp(float(0.), float(1.)),
    );
    hue(
        color * strength * (float(1.) - p.y() * float(0.7)).max(float(0.)),
        time(),
    )
}
fn light() -> tsl::Node {
    light_index()
        .less_than(uint(1))
        .select(light_color(), circle() * float(50.))
}
impl BlurredReflection {
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
        let center = Vector3::new(0., 0.5, 0.);
        let p = Vector3::new(-2.5, 2., 2.5) - center;
        let mut viewer = OrbitViewer::from_camera(center, p.length());
        viewer.fixture(p.x.atan2(p.z), (p.y / p.length()).asin(), 1.8);
        s.tone_mapping = ToneMapping::Neutral;
        s.exposure = 1.3;
        let sky = mesh(
            s,
            Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
            Material::Shader(
                background_material(
                    r,
                    vec4(
                        hue(rgb(0x0066ff) * sky_direction().y() * float(0.1), time()),
                        float(1.),
                    ),
                )
                .await?,
            ),
        );
        s.get_mut(sky)?.frustum_culled = false;
        s.get_mut(sky)?.render_order = -100;
        let h = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x0066ff),
            intensity: 10.,
        }));
        s.get_mut(h)?.position = Vector3::Y;
        s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 1.,
            distance: 0.,
            decay: 2.,
        }));
        let shared = Arc::new(
            SurfaceNodes {
                light_color: Some(light()),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        );
        let (a, b, i) = load_asset("/web/models/Michelle.glb").await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        let mut objects = instance.meshes;
        for &h in &objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for material in &mut m.materials {
                    let material = Arc::make_mut(material);
                    material.properties_mut().vertex_program = Some(shared.clone());
                    if let Material::Standard(m)
                    | Material::Physical(MeshPhysicalMaterial { base: m, .. }) = material
                    {
                        m.energy_conservation = true;
                    }
                }
            }
        }
        let mut texture = decode_texture_image(
            &fetch("/web/gallery/assets/tsl-procedural/uv_grid_directx.jpg").await?,
        )
        .await?;
        texture.srgb = true;
        texture.mipmap_filter = Some(Filter::Linear);
        let mut material = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        material.properties.map = Some(Arc::new(texture));
        material.properties.side = Side::Double;
        material.properties.vertex_program = Some(shared);
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
            Material::Standard(material),
        );
        s.get_mut(h)?.position = Vector3::new(0., 1., -3.);
        objects.push(h);
        objects.push(sky);
        let reflected = target(r, 1, 1)?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let depth_sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let screen = tsl::viewport::screen_uv();
        let coordinate = vec2(float(1.) - screen.x(), screen.y());
        let reflection = WgslFn::new(
            "blurred_reflection",
            include_str!("blurred_reflection.wgsl"),
            &[
                Type::Texture,
                Type::Sampler,
                Type::DepthTexture,
                Type::Vec2,
                Type::Float,
                Type::Float,
            ],
            Type::Vec3,
        )?
        .call(&[
            tsl::Texture::External(0).node(),
            tsl::Texture::External(0).sampler(),
            tsl::Texture::External(1).node(),
            coordinate,
            uniform(1, Type::Float),
            uniform(2, Type::Float),
        ]);
        let opacity = float(1.) - view_z().smoothstep(float(7.), float(25.));
        let program = SurfaceNodes {
            color: Some(vec4(circle() + reflection, opacity)),
            light_color: Some(light()),
            ..Default::default()
        }
        .build_with_texture_types(
            r,
            &[],
            &[
                (&reflected.view, &sampler, Type::Texture),
                (
                    reflected.depth_view.as_ref().unwrap(),
                    &depth_sampler,
                    Type::DepthTexture,
                ),
            ],
        )
        .await?;
        let mut m = MeshStandardMaterial {
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
        objects.push(floor);
        let camera = s.insert(s.get(c)?.kind.clone());
        s.get_mut(camera)?.matrix_auto_update = false;
        Ok(Self {
            viewer,
            time: 0.,
            mixer,
            objects,
            floor,
            camera,
            reflected,
            sampler,
            depth_sampler,
            parameters: [0.9, 0.2, 0.5],
        })
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i > 2 || !v.is_finite() {
            return Err(Error::Invalid("blurred reflection parameter"));
        }
        self.parameters[i] = v.clamp(if i == 2 { 0.25 } else { 0. }, 1.);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
        }
        self.viewer
            .limit_pitch(0., std::f64::consts::FRAC_PI_2 - 1e-6);
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.25;
            p.far = 30.;
        }
        for action in &mut self.mixer.actions {
            action.time = self.time;
        }
        self.mixer.update(s, 0.)?;
        for &h in &self.objects {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for material in &mut m.materials {
                    match Arc::make_mut(material) {
                        Material::Shader(m) => m.uniforms[0][0] = self.time as f32,
                        m => {
                            let u = &mut m.properties_mut().vertex_uniforms;
                            u[0][0] = self.time as f32;
                            u[1][0] = self.parameters[0];
                            u[2][0] = self.parameters[1];
                        }
                    }
                }
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
            ((out.width as f64 * self.parameters[2] as f64).round() as u32).max(1),
            ((out.height as f64 * self.parameters[2] as f64).round() as u32).max(1),
        );
        if self.reflected.width != w || self.reflected.height != h {
            self.reflected = target(r, w, h)?;
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
                        (
                            self.reflected.depth_view.as_ref().unwrap(),
                            &self.depth_sampler,
                        ),
                    ],
                )?;
            }
        }
        s.update_world_matrix(c, true, false)?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(camera) = camera else {
            return Err(Error::Invalid("reflector camera"));
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
