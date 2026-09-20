use super::*;
use crate::{postprocessing::Effect, tsl::*};
pub(super) struct MaskPass {
    input: RenderTarget,
    h: RenderTarget,
    v: RenderTarget,
    blur_h: Effect,
    blur_v: Effect,
    compose: Effect,
    sampler: wgpu::Sampler,
}
fn display(color: tsl::Node) -> Result<tsl::Node> {
    Ok(WgslFn::new("mask_display",&format!("{}\nfn mask_display(value:vec4<f32>)->vec4<f32>{{return vec4(mask_srgb_output(mask_tone_output(value.rgb,0.4,3.0)),clamp(value.a,0.0,1.0));}}",include_str!("../../shaders/output.wgsl").replace("aces_output", "mask_aces_output").replace("tone_output", "mask_tone_output").replace("srgb_output", "mask_srgb_output")),&[Type::Vec4],Type::Vec4)?.call(&[color]))
}
impl MaskPass {
    async fn new(r: &Renderer) -> Result<Self> {
        let input = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples: 4,
                count: 2,

                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let make = || {
            RenderTarget::with_options(
                &r.device,
                1,
                1,
                RenderTargetOptions {
                    depth_buffer: false,
                    format: wgpu::TextureFormat::Rgba16Float,
                    ..Default::default()
                },
            )
        };
        let h = make()?;
        let v = make()?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let blur_h = effect_with_textures(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &gaussian_blur(tsl::Texture::External(0), uv(), uniform(0, Type::Vec2), 20)?,
            &[(
                &input.textures()[1].create_view(&Default::default()),
                &sampler,
            )],
        )
        .await?;
        let blur_v = effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &gaussian_blur(tsl::Texture::Input, uv(), uniform(0, Type::Vec2), 20)?,
        )
        .await?;
        let compose = effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &display(
                tsl::Texture::Input.sample(uv()) + tsl::Texture::History.sample(uv()) * float(0.3),
            )?,
        )
        .await?;
        Ok(Self {
            input,
            h,
            v,
            blur_h,
            blur_v,
            compose,
            sampler,
        })
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        scene: &mut Scene,
        cam: Object3D,
        target: &RenderTarget,
    ) -> Result<()> {
        if (self.input.width, self.input.height) != (target.width, target.height) {
            for rt in [&mut self.input, &mut self.h, &mut self.v] {
                rt.set_size(&r.device, target.width, target.height)?;
            }
            self.blur_h.set_textures(
                r,
                &[(
                    &self.input.textures()[1].create_view(&Default::default()),
                    &self.sampler,
                )],
            )?;
        }
        r.render(scene, cam, &self.input)?;
        self.blur_h.parameters[0] = [1.0 / target.width as f32, 0.0, 0.0, 0.0];
        self.blur_h.apply(r, &self.input, None, &self.h)?;
        self.blur_v.parameters[0] = [0.0, 1.0 / target.height as f32, 0.0, 0.0];
        self.blur_v.apply(r, &self.h, None, &self.v)?;
        self.compose.apply(r, &self.input, Some(&self.v), target)
    }
}
impl Demo {
    pub(super) async fn mask(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.0,
            aspect,
            near: 0.01,
            far: 100.0,
            ..Default::default()
        }));
        let position = Vector3::new(1.0, 2.0, 3.0);
        let target = Vector3::new(0.0, 1.0, 0.0);
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, target)?;
        scene.background = Color::BLACK;
        let rgb = |hex| {
            let c = Color::from_hex(hex).0;
            vec3(float(c.x as f32), float(c.y as f32), float(c.z as f32))
        };
        let graph = SurfaceNodes {
            color: Some(mix(rgb(0x66bbff), rgb(0x4466ff), float(1.0) - uv().y()) * float(0.05)),
            ..Default::default()
        };
        let background = graph
            .build_background_mrt(r, &[output(), splat(float(0.0), Type::Vec4)])
            .await?;
        let h = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
            Arc::new(Material::Shader(background)),
        )));
        scene.get_mut(h)?.frustum_culled = false;
        scene.get_mut(h)?.render_order = i32::MIN;
        let light = scene.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 2000.0 / std::f64::consts::PI,
            target: Vector3::ZERO,
            distance: 0.0,
            decay: 2.0,
            angle: std::f64::consts::PI / 3.0,
            penumbra: 0.0,
        }));
        scene.get_mut(light)?.position = Vector3::Y;
        scene.add(cam, light)?;
        let (asset, buffers, images) =
            super::super::gltf_viewer::load_asset("/web/models/Michelle.glb").await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
        let default_program = Arc::new(
            SurfaceNodes::default()
                .build_mrt(r, &[output(), splat(float(0.0), Type::Vec4)], &[], &[])
                .await?,
        );
        let glow_program = Arc::new(
            SurfaceNodes::default()
                .build_mrt(r, &[output(), output() + float(1.0)], &[], &[])
                .await?,
        );
        for (i, h) in instance.meshes.iter().enumerate() {
            if let NodeKind::Mesh(m) = &mut scene.get_mut(*h)?.kind {
                for material in &mut m.materials {
                    match Arc::make_mut(material) {
                        Material::Standard(m)
                        | Material::Physical(MeshPhysicalMaterial { base: m, .. }) => {
                            m.energy_conservation = true
                        }
                        _ => {}
                    }
                    Arc::make_mut(material).properties_mut().vertex_program = Some(if i == 0 {
                        glow_program.clone()
                    } else {
                        default_program.clone()
                    });
                }
            }
        }
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        let blue = Arc::new(
            SurfaceNodes::default()
                .build_mrt(r, &[output(), output()], &[], &[])
                .await?,
        );
        let geometry = Arc::new(SphereGeometry::build(0.3, 32, 16)?);
        let group = scene.insert(NodeKind::Group);
        for (i, color) in [0x0000ff, 0x00ff00, 0xff0000, 0x00ffff]
            .into_iter()
            .enumerate()
        {
            let mut m = MeshStandardMaterial {
                energy_conservation: true,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(color);
            m.properties.vertex_program = Some(if i == 0 {
                blue.clone()
            } else {
                default_program.clone()
            });
            let h = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Standard(m)),
            )));
            let a = i as f64 * std::f64::consts::FRAC_PI_2;
            scene.get_mut(h)?.position = Vector3::new(a.cos(), 1.0, a.sin());
            scene.add(group, h)?;
        }
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        Ok(Self {
            example: 77,
            time: 0.0,
            lights: vec![],
            objects: vec![group],
            params: [[0.0; 4]; 16],
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: Some(mixer),
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: Some(MaskPass::new(r).await?),
        })
    }
}
