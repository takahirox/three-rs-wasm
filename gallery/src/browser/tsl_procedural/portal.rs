use super::*;
pub(in crate::browser) struct Portal {
    time: f64,
    viewer: OrbitViewer,
    portal: Scene,
    portal_camera: Object3D,
    portal_target: RenderTarget,
    sampler: wgpu::Sampler,
    plane: Object3D,
    objects: [Vec<Object3D>; 2],
    mixers: [crate::animation::AnimationMixer; 2],
}
impl Portal {
    pub async fn create(s: &mut Scene, c: Object3D, _example: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            aspect,
            near: 0.01,
            far: 30.,
            ..Default::default()
        }));
        let position = Vector3::new(2., 0.8, 2.4);
        let center = Vector3::new(0., 1., 0.);
        s.get_mut(c)?.position = position;
        s.look_at(c, center)?;
        let offset = position - center;
        let mut viewer = OrbitViewer::from_camera(center, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        s.tone_mapping = ToneMapping::Linear;
        s.exposure = 0.15;
        let mut portal = Scene::new();
        let portal_camera = portal.insert(s.get(c)?.kind.clone());
        let mut objects = [vec![], vec![]];
        for (i, scene) in [&mut *s, &mut portal].into_iter().enumerate() {
            let direction = sky_direction();
            let color = if i == 0 {
                mix(rgb(0x0066ff), rgb(0xff0066), direction.y())
            } else {
                tsl::materialx::worley(
                    direction * float(20.) + vec3(float(0.), float(1.) - time(), float(0.)),
                    tsl::materialx::Dimension::D3,
                    float(1.),
                    float(0.),
                ) * rgb(0x0066ff)
            };
            let h = mesh(
                scene,
                Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
                Material::Shader(background_material(r, vec4(color, float(1.))).await?),
            );
            scene.get_mut(h)?.frustum_culled = false;
            scene.get_mut(h)?.render_order = -100;
            objects[i].push(h);
            let light = scene.insert(NodeKind::Light(Light::Point {
                color: Color::WHITE,
                intensity: 17000. / (4. * std::f64::consts::PI),
                distance: 0.,
                decay: 2.,
            }));
            scene.get_mut(light)?.position = Vector3::new(0., 1., 5.);
        }
        let h = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0xff0066),
            ground: Color::from_hex(0x0066ff),
            intensity: 7.,
        }));
        s.get_mut(h)?.position = Vector3::Y;
        let (a, b, i) = load_asset("/web/gallery/assets/tsl-procedural/Xbot.glb").await?;
        let imported = crate::gltf::import_animated_decoded(&a, &b, &i)?;
        let p = uv() * float(20.) + time();
        let color = tsl::materialx::fractal_vec3(
            vec3(p.x(), p.y(), float(0.)),
            tsl::materialx::Dimension::D3,
            float(3.),
            float(2.),
            float(0.5),
        );
        let program = Arc::new(
            SurfaceNodes {
                color: Some(color),
                output: Some(output().max(float(0.))),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        );
        let mut mixers = [
            crate::animation::AnimationMixer::default(),
            crate::animation::AnimationMixer::default(),
        ];
        for (i, scene) in [&mut *s, &mut portal].into_iter().enumerate() {
            let instance = imported.clone().instantiate(scene)?;
            mixers[i].play(instance.clips[6].clone())?;
            for h in instance.meshes {
                if let NodeKind::Mesh(mesh) = &mut scene.get_mut(h)?.kind {
                    for material in &mut mesh.materials {
                        match Arc::make_mut(material) {
                            Material::Standard(m)
                            | Material::Physical(MeshPhysicalMaterial { base: m, .. }) => {
                                m.energy_conservation = true;
                                if i == 1 {
                                    m.properties.vertex_program = Some(program.clone());
                                    m.properties.wireframe = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                objects[i].push(h);
            }
        }
        let portal_target = target(r, 1, 1)?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let alpha = float(1.) - ((uv() - float(0.5)).length() - float(0.3)) / float(0.2);
        let graph = NodeMaterial::new(vec4(
            tsl::Texture::External(0)
                .sample(tsl::viewport::screen_uv())
                .rgb(),
            alpha.clamp(float(0.), float(1.)),
        ));
        let mut material = graph.build(r, &[(&portal_target.view, &sampler)]).await?;
        material.properties.transparent = true;
        material.properties.side = Side::Double;
        let plane = mesh(
            s,
            Arc::new(PlaneGeometry::build(1.7, 2., 1, 1)?),
            Material::Shader(material),
        );
        s.get_mut(plane)?.position = Vector3::new(0., 1., 0.8);
        Ok(Self {
            time: 0.,
            viewer,
            portal,
            portal_camera,
            portal_target,
            sampler,
            plane,
            objects,
            mixers,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
        }
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.01;
            p.far = 30.;
        }
        let camera = s.get(c)?;
        let copy = self.portal.get_mut(self.portal_camera)?;
        copy.kind = camera.kind.clone();
        copy.position = camera.position;
        copy.quaternion = camera.quaternion;
        for (i, scene) in [&mut *s, &mut self.portal].into_iter().enumerate() {
            for a in &mut self.mixers[i].actions {
                a.time = self.time;
            }
            self.mixers[i].update(scene, 0.)?;
            for &h in &self.objects[i] {
                if let NodeKind::Mesh(m) = &mut scene.get_mut(h)?.kind {
                    for material in &mut m.materials {
                        match Arc::make_mut(material) {
                            Material::Shader(m) => m.uniforms[0][0] = self.time as f32,
                            _ => {
                                Arc::make_mut(material).properties_mut().vertex_uniforms[0][0] =
                                    self.time as f32
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub fn parameter(&mut self, _i: usize, _v: f32) -> Result<()> {
        Err(Error::Invalid("procedural parameter"))
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.portal_target.width != out.width || self.portal_target.height != out.height {
            self.portal_target = target(r, out.width, out.height)?;
            if let NodeKind::Mesh(m) = &mut s.get_mut(self.plane)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                Arc::make_mut(&mut m.program).rebind(
                    r,
                    &[],
                    &[(&self.portal_target.view, &self.sampler)],
                )?;
            }
        }
        r.render(&mut self.portal, self.portal_camera, &self.portal_target)?;
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
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if pan {
            self.viewer.pan_pixels(s, c, dx, dy, height)?;
        } else {
            self.viewer
                .orbit_pixels(dx, dy, wheel, height, 0., f64::INFINITY);
        }
        Ok(())
    }
}
