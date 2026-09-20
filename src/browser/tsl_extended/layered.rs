use super::*;
use crate::attribute::BufferAttribute;
use crate::render_target::RenderTarget3D;
/// Four resident views of the same head data, with native array/3D render slices.
pub(super) struct Layered {
    scenes: Vec<(Scene, Object3D)>,
    target: RenderTarget,
    array: RenderTarget,
    volume: RenderTarget3D,
    quad_scene: Scene,
    quad_camera: Object3D,
    quad: Object3D,
    slice: u64,
    viewer: OrbitViewer,
}
impl Layered {
    pub(super) async fn new(r: &Renderer) -> Result<Self> {
        let data = fetch("/web/gallery/assets/head256x256x109.raw").await?;
        if data.len() != 256 * 256 * 109 {
            return Err(Error::Invalid("head volume size"));
        }
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut maps = Vec::new();
        for dimension in [wgpu::TextureDimension::D2, wgpu::TextureDimension::D3] {
            let texture = r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("head texture"),
                size: wgpu::Extent3d {
                    width: 256,
                    height: 256,
                    depth_or_array_layers: 109,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            r.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(256),
                },
                texture.size(),
            );
            maps.push(texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(if dimension == wgpu::TextureDimension::D3 {
                    wgpu::TextureViewDimension::D3
                } else {
                    wgpu::TextureViewDimension::D2Array
                }),
                ..Default::default()
            }));
        }
        let options = RenderTargetOptions {
            depth: 109,
            depth_buffer: false,
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        };
        let array = RenderTarget::with_options(&r.device, 256, 256, options.clone())?;
        let mut volume = RenderTarget3D::new(&r.device, 256, 256, 109, options)?;
        volume.set_load_color(true);
        maps.push(array.texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        }));
        maps.push(volume.texture.create_view(&Default::default()));
        // The official TextureHelper merges 109 static planes, in increasing slice order.
        let mut positions = Vec::new();
        let mut uv_values = Vec::new();
        let mut indices = Vec::new();
        for i in 0..109 {
            let z = 5.45 * (i as f32 / 108.0 - 0.5);
            positions.extend_from_slice(&[-5.0, 5.0, z, 5.0, 5.0, z, -5.0, -5.0, z, 5.0, -5.0, z]);
            uv_values.extend_from_slice(&[0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0]);
            let j = i * 4;
            indices.extend_from_slice(&[j, j + 2, j + 1, j + 2, j + 3, j + 1]);
        }
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(positions, 3, false)?),
        );
        g.set_attribute(
            "uv",
            Attribute::F32(BufferAttribute::new(uv_values, 2, false)?),
        );
        g.set_index(Some(indices));
        let geometry = Arc::new(g);
        let mut scenes = Vec::new();
        for (i, map) in maps.iter().enumerate() {
            let mut s = Scene::new();
            let c = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                fov: 50.0,
                aspect: 1.0,
                near: 0.1,
                far: 100.0,
                ..Default::default()
            })));
            let z = position_local().swizzle("z") / float(5.45) + float(0.5);
            let uv = vec2(uv().x(), float(1.0) - uv().y());
            let sample = if i % 2 == 0 {
                tsl::Texture::External(0).sample_array(uv, z.clone() * float(108.0) + float(0.001))
            } else {
                tsl::sampling::volume(tsl::Texture::External(0), vec3(uv.x(), uv.y(), z.clone()))
            };
            let value = sample.x();
            let color = if i < 2 {
                value.clone() * value.clone() * z * float(if i == 0 { 108.0 / 109.0 } else { 1.0 })
            } else {
                value.clone()
            };
            let mut m = NodeMaterial::new(vec4(
                vec3(color.clone(), color.clone(), color),
                value * float(if i < 2 { 0.25 } else { 1.0 }),
            ))
            .build_with_texture_types(
                r,
                &[(
                    map,
                    &sampler,
                    if i % 2 == 0 {
                        Type::TextureArray
                    } else {
                        Type::Texture3D
                    },
                )],
            )
            .await?;
            m.properties.transparent = true;
            m.properties.side = Side::Double;
            m.properties.force_single_pass = false;
            mesh(&mut s, geometry.clone(), Material::Shader(m));
            let center = vec2(
                float(if i % 2 == 0 { 0.25 } else { 0.75 }),
                float(if i < 2 { 0.75 } else { 0.25 }),
            );
            let coordinate = screen_coordinate() / screen_size();
            let distance = (coordinate - center)
                .length()
                .smoothstep(float(0.0), float(0.2))
                .pow(float(0.3))
                * float(0.1);
            let bg = mesh(
                &mut s,
                Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
                Material::Shader(
                    tsl::surface::background_material(
                        r,
                        rgb(if i < 2 { 0x616161 } else { 0x212121 }) - distance,
                    )
                    .await?,
                ),
            );
            s.get_mut(bg)?.render_order = -100;
            s.get_mut(bg)?.frustum_culled = false;
            scenes.push((s, c));
        }
        let mut quad_scene = Scene::new();
        let quad_camera =
            quad_scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
                left: -1.0,
                right: 1.0,
                top: 1.0,
                bottom: -1.0,
                near: 0.0,
                far: 2.0,
                ..Default::default()
            })));
        quad_scene.get_mut(quad_camera)?.position.z = 1.0;
        let mut m = NodeMaterial::new(vec4(
            tsl::Texture::External(0)
                .sample_array(
                    vec2(uv().x(), float(1.0) - uv().y()),
                    uniform(0, Type::Float),
                )
                .rgb(),
            float(1.0),
        ))
        .build_with_texture_types(r, &[(&maps[0], &sampler, Type::TextureArray)])
        .await?;
        m.properties.depth_test = false;
        m.properties.depth_write = false;
        let quad = mesh(
            &mut quad_scene,
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
            Material::Shader(m),
        );
        let target = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 149.0_f64.sqrt());
        viewer.fixture((-7.0_f64).atan2(10.0), 0.0, 1.8);
        Ok(Self {
            scenes,
            target,
            array,
            volume,
            quad_scene,
            quad_camera,
            quad,
            slice: 0,
            viewer,
        })
    }
    pub(super) fn output(&self) -> &RenderTarget {
        &self.target
    }
    pub(super) fn pan(&mut self, dx: f64, dy: f64, height: f64) -> Result<()> {
        let (s, c) = &self.scenes[0];
        self.viewer.pan_pixels(s, *c, dx, dy, height)
    }
    pub(super) fn input(&mut self, dx: f64, dy: f64, wheel: f64, height: f64) {
        self.viewer.orbit_pixels(dx, dy, wheel, height, 1.0, 20.0);
        self.viewer
            .limit_yaw(-std::f64::consts::PI / 3.0, std::f64::consts::PI / 3.0);
        self.viewer.limit_pitch(
            std::f64::consts::FRAC_PI_2 - std::f64::consts::PI / 1.25,
            std::f64::consts::FRAC_PI_4,
        );
    }
    pub(super) fn render(&mut self, r: &Renderer, output: &RenderTarget, time: f64) -> Result<()> {
        let desired = (time.max(0.0) * 20.0).floor() as u64 + 1;
        // At most a full volume needs filling after a suspended tab resumes.
        if desired.saturating_sub(self.slice) > 109 {
            self.slice = desired - 109;
        }
        while self.slice < desired {
            let j = (self.slice % 109) as u32;
            if let NodeKind::Mesh(m) = &mut self.quad_scene.get_mut(self.quad)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.uniforms[0][0] = j as f32;
            }
            self.array.set_layer(j)?;
            self.volume.set_layer(j)?;
            r.render(&mut self.quad_scene, self.quad_camera, &self.array)?;
            r.render(&mut self.quad_scene, self.quad_camera, &self.volume)?;
            self.slice += 1;
        }
        if (self.target.width, self.target.height) != (output.width, output.height) {
            self.target
                .set_size(&r.device, output.width, output.height)?;
        }
        for (i, (s, c)) in self.scenes.iter_mut().enumerate() {
            self.viewer.update(s, *c)?;
            let width = output.width / 2;
            let height = output.height / 2;
            if let NodeKind::Camera(Camera::Perspective(cam)) = &mut s.get_mut(*c)?.kind {
                cam.aspect = width as f64 / height as f64;
                cam.near = 0.1;
                cam.far = 100.0;
            }
            self.target.viewport = [
                (i as u32 % 2) * width,
                if i < 2 { height } else { 0 },
                width,
                height,
            ];
            self.target.scissor = Some(self.target.viewport);
            self.target.options.load_color = i != 0;
            r.render(s, *c, &self.target)?;
        }
        Ok(())
    }
}
