//! Additional pinned r186 scenes. Static geometry and node programs stay on the GPU.
mod anamorphic;
mod cube;
mod dof;
mod earth;
mod elements;
mod gather;
mod indirect;
mod interpolation;
mod layered;
mod occlusion;
mod points;
mod volume;
use super::gltf_viewer::{OrbitViewer, decode_image, fetch};
use crate::{
    Error, Result,
    camera::*,
    compute::{BufferAccess, GpuBuffer},
    geometry::*,
    material::*,
    math::*,
    postprocessing::Effect,
    renderer::*,
    scene::*,
    tsl::{self, *},
};
use std::sync::Arc;
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.0
}
fn rgb(hex: u32) -> tsl::Node {
    let c = Color::from_hex(hex);
    vec3(
        float(c.0.x as f32),
        float(c.0.y as f32),
        float(c.0.z as f32),
    )
}
fn mesh(s: &mut Scene, g: Arc<BufferGeometry>, m: Material) -> Object3D {
    s.insert(NodeKind::Mesh(Mesh::new(g, Arc::new(m))))
}
pub(super) struct Demo {
    example: u32,
    time: f64,
    rotation: f64,
    objects: Vec<Object3D>,
    initial_rotations: Vec<[f64; 3]>,
    params: [f32; 16],
    viewer: OrbitViewer,
    post: Option<(RenderTarget, Effect)>,
    points: Option<points::PointsPass>,
    layered: Option<layered::Layered>,
    indirect: Option<indirect::Indirect>,
    elements: Option<elements::Elements>,
    dof: Option<dof::DofPass>,
    occlusion: Option<crate::occlusion::OcclusionQueries>,
    anamorphic: Option<anamorphic::Anamorphic>,
    interpolation: Option<interpolation::Comparison>,
    volume_compute: Option<(crate::compute::ComputeKernel, GpuBuffer)>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, example: u32, r: &Renderer) -> Result<Self> {
        let (fov, near, far, z) = match example {
            78 => (70.0, 0.1, 50.0, 3.0),
            79 => (60.0, 0.1, 100.0, 10.0),
            80 => (45.0, 0.1, 2000.0, 70.0),
            81 => (60.0, 0.1, 100.0, 2.0),
            82 => (60.0, 0.1, 100.0, 1.5),
            83 => (60.0, 0.1, 100.0, 1.5),
            84 => (45.0, 0.1, 2000.0, 70.0),
            85 => (60.0, 1.0, 2100.0, 50.0),
            96 => (50.0, 1.0, 10000.0, 500.0),
            97 => (50.0, 0.1, 100.0, 10.0),
            95 => (40.0, 1.0, 1000.0, 60.0),
            94 => (50.0, 0.1, 10000.0, 1.0),
            92 | 93 => (50.0, 1.0, 10.0, 2.0),
            91 => (70.0, 1.0, 3500.0, 200.0),
            90 => (45.0, 1.0, 4000.0, 1200.0),
            89 => (50.0, 0.01, 100.0, 7.0),
            88 => (25.0, 0.1, 100.0, 3.0),
            87 => (45.0, 0.25, 250.0, 20.0),
            86 => (50.0, 0.1, 2000.0, 2.0),
            _ => return Err(Error::Invalid("TSL extended example")),
        };
        let aspect = if let Camera::Perspective(c) = s.camera(c)?.0 {
            c.aspect
        } else {
            1.0
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            aspect,
            near,
            far,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0.0, 0.0, z);
        s.look_at(c, Vector3::ZERO)?;
        let mut out = Self {
            example,
            time: 0.0,
            rotation: 0.0,
            objects: vec![],
            initial_rotations: vec![],
            params: [1.0; 16],
            viewer: OrbitViewer::from_camera(Vector3::ZERO, z),
            post: None,
            interpolation: None,
            anamorphic: None,
            occlusion: None,
            dof: None,
            elements: None,
            indirect: None,
            points: None,
            layered: None,
            volume_compute: None,
        };
        if example == 96 {
            out.cube_mipmaps(s, r).await?;
        } else if example == 97 {
            out.layered = Some(layered::Layered::new(r).await?);
        } else if example == 78 {
            out.multisampled(s, r).await?;
        } else if example == 79 {
            out.layers(s, c, r).await?;
        } else if example == 80 {
            out.texture_array(s, r).await?;
        } else if example == 84 {
            out.compressed_array(s, r).await?;
        } else if example == 85 {
            out.interpolation(s, r).await?;
        } else if example == 95 {
            out.points(s, c, r).await?;
        } else if example == 94 {
            out.indirect(s, c, r).await?;
        } else if [92, 93].contains(&example) {
            out.elements = Some(elements::Elements::new(r).await?);
        } else if example == 91 {
            out.dof(s, r).await?;
        } else if example == 90 {
            out.instance_uniform(s, c, r).await?;
        } else if example == 89 {
            out.occlusion(s, r).await?;
        } else if example == 88 {
            out.earth(s, c, r).await?;
        } else if example == 87 {
            out.anamorphic(s, r).await?;
        } else if example == 86 {
            out.gather(s, c, r).await?;
        } else {
            out.volume(s, r).await?;
        }
        Ok(out)
    }
    async fn compressed_array(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let bytes = fetch("/web/gallery/assets/spiritedaway.ktx2").await?;
        let texture = GpuTexture::from_basis_array(r, &bytes, true)?;
        let phase = uniform(0, Type::Float) * float(10.0) + float(1.0);
        let layer = phase.clone() - (phase / float(5.0)).floor() * float(5.0);
        let graph = NodeMaterial::new(
            tsl::Texture::External(0).sample_array(vec2(uv().x(), float(1.0) - uv().y()), layer),
        );
        let material = graph
            .build_with_texture_types(r, &[(&texture.view, &texture.sampler, Type::TextureArray)])
            .await?;
        self.objects.push(mesh(
            s,
            Arc::new(PlaneGeometry::build(50.0, 25.0, 1, 1)?),
            Material::Shader(material),
        ));
        Ok(())
    }
    async fn texture_array(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let data = fetch("/web/gallery/assets/head256x256x109.raw").await?;
        if data.len() != 256 * 256 * 109 {
            return Err(Error::Invalid("head array texture length"));
        }
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("head volume array"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 109,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
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
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor::default());
        let layer = (tsl::osc_triangle(uniform(0, Type::Float) * float(0.5)) + float(1.0))
            * float(0.5 * 109.0);
        let value = tsl::Texture::External(0)
            .sample_array(vec2(uv().x(), float(1.0) - uv().y()), layer)
            .x()
            * float(1.9)
            - float(0.1);
        let material = NodeMaterial::new(value)
            .build_with_texture_types(r, &[(&view, &sampler, Type::TextureArray)])
            .await?;
        self.objects.push(mesh(
            s,
            Arc::new(PlaneGeometry::build(50.0, 50.0, 1, 1)?),
            Material::Shader(material),
        ));
        Ok(())
    }
    async fn multisampled(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0x111111);
        let g = Arc::new(BoxGeometry::segmented(7.0, 7.0, 7.0, 12, 12, 12)?);
        let mut seed = 186;
        let mut positions = vec![];
        for i in 0..50 {
            let radius = if i % 2 == 0 { 20.0 } else { 10.0 };
            let phi = (2.0 * random(&mut seed) - 1.0).acos() - std::f64::consts::FRAC_PI_2;
            let theta = std::f64::consts::TAU * random(&mut seed);
            positions.push(Vector3::new(
                radius * phi.cos() * theta.cos(),
                radius * phi.sin(),
                radius * phi.cos() * theta.sin(),
            ));
        }
        for wire in [true, false] {
            let mut m = MeshBasicMaterial::default();
            m.properties.wireframe = wire;
            if !wire {
                m.properties.color = Color::from_hex(0xff0000);
            }
            let h = mesh(s, g.clone(), Material::Basic(m));
            s.get_mut(h)?.instances = positions
                .iter()
                .map(|p| {
                    let mut matrix = Matrix4::IDENTITY;
                    if !wire {
                        matrix *= 0.996;
                    }
                    matrix.w_axis = p.extend(1.0);
                    Instance {
                        matrix,
                        color: Color::WHITE,
                    }
                })
                .collect();
            self.objects.push(h);
        }
        let rt = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                samples: 4,
                ..Default::default()
            },
        )?;
        let effect = tsl::effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &tsl::Texture::Input.sample(uv()),
        )
        .await?;
        self.post = Some((rt, effect));
        Ok(())
    }
    async fn layers(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.get_mut(c)?.layers.mask = 7;
        let coordinate = screen_coordinate() / screen_size();
        let bg = mix(rgb(0xf996ae), rgb(0xf6f0a3), coordinate.x())
            + rgb(0xd9b6fd) * (float(1.0) - (coordinate - vec2(float(0.5), float(1.0))).length());
        let source = NodeMaterial::new(bg).wgsl(0)?;
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var o=surface;o.clip=vec4(position.xy*2.0,1.0,1.0);return o;}";
        let mut m = ShaderMaterial::new(Arc::new(
            crate::shader::ShaderProgram::with_projection(r, &source, &[], &[], projection).await?,
        ));
        m.properties.depth_write = false;
        m.properties.depth_test = false;
        let bg = mesh(
            s,
            Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1)?),
            Material::Shader(m),
        );
        s.get_mut(bg)?.frustum_culled = false;
        s.get_mut(bg)?.render_order = -100;
        s.get_mut(bg)?.layers.mask = 1 << 31;
        let mut image = decode_image(&fetch("/web/gallery/assets/blossom.png").await?).await?;
        image.srgb = true;
        image.mipmap_filter = Some(Filter::Linear);
        let image = r.upload_texture(&Arc::new(image))?;
        let mut g = PlaneGeometry::build(0.25, 0.25, 1, 1)?;
        g.instance_count = Some(2500);
        let g = Arc::new(g);
        let mut seed = 186;
        for (layer, hex) in [0xd70654, 0xffd95f, 0xb8d576].into_iter().enumerate() {
            let mut positions = vec![];
            let mut rotations = vec![];
            let mut directions = vec![];
            for i in 0..2500 {
                positions.push([
                    (-25.0 + 5.0 * random(&mut seed)) as f32,
                    (-10.0 + 60.0 * random(&mut seed)) as f32,
                    (-5.0 + 10.0 * random(&mut seed)) as f32,
                    i as f32 / 2500.0,
                ]);
                let d = Vector3::new(
                    0.7 + 0.2 * random(&mut seed),
                    -0.3 + 0.15 * random(&mut seed),
                    0.0,
                )
                .normalize();
                rotations.push([
                    random(&mut seed) as f32,
                    random(&mut seed) as f32,
                    random(&mut seed) as f32,
                    0.0,
                ]);
                directions.push([d.x as f32, d.y as f32, d.z as f32, 0.0]);
            }
            let buffers = [positions, rotations, directions]
                .iter()
                .map(|v| GpuBuffer::new(r, bytemuck::cast_slice(v), BufferAccess::Read))
                .collect::<Result<Vec<_>>>()?;
            let local = (instanced_attribute(0).swizzle("w")
                + uniform(0, Type::Float) * float(0.02))
            .fract();
            let tex = tsl::Texture::External(0).sample(vec2(uv().x(), float(1.0) - uv().y()));
            // Alpha maps read green from the same sRGB texture, as the original node graph does.
            let alpha = tex.swizzle("w") * tex.y();
            let graph = NodeMaterial {
                position: Some(
                    tsl::rotate_euler(
                        position_geometry(),
                        instanced_attribute(1).rgb() * local.clone() * float(20.0),
                    ) + instanced_attribute(0).rgb()
                        + instanced_attribute(2).rgb() * local * float(50.0),
                ),
                color: vec4(rgb(hex) * tex.rgb(), alpha),
            };
            let mut m = graph
                .build_with_storage(
                    r,
                    &buffers.iter().map(|b| (b, Type::Vec4)).collect::<Vec<_>>(),
                    &[(&image.view, &image.sampler)],
                )
                .await?;
            m.properties.side = Side::Double;
            m.properties.alpha_test = 0.1;
            let h = mesh(s, g.clone(), Material::Shader(m));
            s.get_mut(h)?.layers.set(layer as u32);
            s.get_mut(h)?.frustum_culled = false;
            self.objects.push(h);
        }
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, delta: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += delta;
            if self.params[1] > 0.5 {
                self.rotation += delta * 60.0;
            }
        }
        if self.example == 90 {
            self.update_instance_uniform(s, c)?;
        }
        if self.example == 89 {
            self.update_occlusion(s, c)?;
        }
        if self.example == 88 {
            self.update_earth(s, c)?;
        }
        if self.example == 78 {
            for h in &self.objects {
                s.get_mut(*h)?.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    self.rotation * 0.001,
                    self.rotation * 0.002,
                    0.0,
                );
            }
        }
        if self.example == 79 {
            s.get_mut(c)?.layers.mask = (1 << 31)
                | (0..3)
                    .filter(|i| self.params[*i] > 0.5)
                    .map(|i| 1u32 << i)
                    .sum::<u32>();
            for h in &self.objects {
                if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0][0] = self.time as f32;
                }
            }
        }
        if [81, 82, 83].contains(&self.example) {
            self.update_volume(s, c)?;
        }
        if [87, 91, 94, 95, 96].contains(&self.example) {
            self.viewer.update(s, c)?;
        }
        if [80, 84, 87, 91, 94, 95].contains(&self.example) {
            for h in &self.objects {
                if let NodeKind::Mesh(m) = &mut s.get_mut(*h)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0][0] = self.time as f32;
                    if self.example == 87 {
                        m.uniforms[1][0] = self.params[5];
                    }
                    if self.example == 95 {
                        m.uniforms[1][0] = self.params[2];
                        m.uniforms[2][0] = self.params[0];
                        m.uniforms[3][0] =
                            web_sys::window().map_or(1.0, |w| w.device_pixel_ratio()) as f32;
                        m.properties.alpha_to_coverage = self.params[0] > 0.5;
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
        target: &RenderTarget,
    ) -> Result<bool> {
        if let Some(pass) = &mut self.layered {
            pass.render(r, target, self.time)?;
            return Ok(true);
        }
        if let Some(pass) = &mut self.points {
            pass.render(r, s, c, target, self.time, &self.params)?;
            return Ok(true);
        }
        if let Some(pass) = &self.indirect {
            pass.render(r, s, c, target, self.time)?;
            return Ok(true);
        }
        if let Some(elements) = &mut self.elements {
            elements.render(r, s, c, target, self.time)?;
            return Ok(true);
        }
        if let Some(pass) = &mut self.dof {
            pass.render(r, s, c, target, &self.params)?;
            return Ok(true);
        }
        if let Some(q) = &self.occlusion {
            r.render_with_occlusion(s, c, target, Some(q))?;
            return Ok(true);
        }
        if let Some(pass) = &mut self.anamorphic {
            pass.render(r, s, c, target, &self.params)?;
            return Ok(true);
        }
        if self.example == 86
            && let NodeKind::Camera(Camera::Orthographic(camera)) = &mut s.get_mut(c)?.kind
        {
            camera.left = -(target.width as f64) / (target.height as f64);
            camera.right = -camera.left;
        }
        if let Some(comparison) = &mut self.interpolation {
            comparison.render(r, s, c, target, self.params[0] as usize, &self.objects)?;
            return Ok(true);
        }
        if let Some((kernel, buffer)) = &self.volume_compute {
            buffer.write(
                r,
                0,
                bytemuck::cast_slice(&[self.time as f32, 0.0, 0.0, 0.0]),
            )?;
            let max = r.device.limits().max_compute_workgroups_per_dimension;
            kernel.dispatch(r, [max, 125000_u32.div_ceil(max), 1])?;
        }
        let Some((rt, fx)) = &mut self.post else {
            return Ok(false);
        };
        let samples = if self.params[0] > 0.5 { 4 } else { 1 };
        if rt.options.samples != samples {
            *rt = RenderTarget::with_options(
                &r.device,
                target.width,
                target.height,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    samples,
                    ..Default::default()
                },
            )?;
        } else if rt.width != target.width || rt.height != target.height {
            rt.set_size(&r.device, target.width, target.height)?;
        }
        r.render(s, c, rt)?;
        fx.apply(r, rt, None, target)?;
        Ok(true)
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        if self.params[1] > 0.5 {
            self.rotation = t * 60.0;
        }
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i == 0
            && v.is_finite()
            && v >= 0.0
            && let Some(e) = &mut self.elements
        {
            return e.select(v as usize);
        }
        if !v.is_finite()
            || i >= match self.example {
                78 => 2,
                79 | 81 => 3,
                82 | 83 => 4,
                85 => 1,
                88 => 4,
                87 => 6,
                91 => 3,
                95 => 4,
                _ => 0,
            }
        {
            return Err(Error::Invalid("TSL extended parameter"));
        }
        self.params[i] = v;
        Ok(())
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
        if let Some(e) = &mut self.elements {
            e.input(dx, dy);
            return Ok(());
        }
        if let Some(p) = &mut self.layered {
            if pan {
                return p.pan(dx, dy, height);
            }
            p.input(dx, dy, wheel, height);
            return Ok(());
        }
        if pan {
            return self.viewer.pan_pixels(s, c, dx, dy, height);
        }
        let (min, max) = match self.example {
            87 => (2.0, 25.0),
            88 => (0.1, 50.0),
            89 => (3.0, 25.0),
            90 => (400.0, 2000.0),
            95 => (10.0, 500.0),
            _ => (0.0, f64::INFINITY),
        };
        self.viewer.orbit_pixels(dx, dy, wheel, height, min, max);
        if self.example == 96 {
            self.viewer.limit_pitch(
                std::f64::consts::FRAC_PI_2 - std::f64::consts::PI / 1.5,
                std::f64::consts::FRAC_PI_4,
            );
        }
        Ok(())
    }
}

impl Demo {
    pub(super) fn viewport(&mut self, index: usize, rectangle: [f64; 4]) -> Result<()> {
        self.elements
            .as_mut()
            .ok_or(Error::Invalid("multiple elements example"))?
            .viewport(index, rectangle)
    }
    pub(super) fn output_target(&self) -> Option<&RenderTarget> {
        self.layered
            .as_ref()
            .map(|p| p.output())
            .or_else(|| self.points.as_ref().map(|p| p.output()))
            .or_else(|| self.elements.as_ref().map(|e| e.output()))
    }
}

impl Demo {
    pub(super) fn attach_canvases(&mut self, r: &Renderer, canvases: js_sys::Array) -> Result<()> {
        self.elements
            .as_mut()
            .ok_or(Error::Invalid("multiple canvases example"))?
            .attach_canvases(r, canvases)
    }
}
