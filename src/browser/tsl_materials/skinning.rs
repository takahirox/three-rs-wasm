use super::*;
use crate::{
    compute::{BufferAccess, GpuBuffer},
    postprocessing::Effect,
};
const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
pub(super) struct Blur {
    scene: RenderTarget,
    intermediate: RenderTarget,
    horizontal: Effect,
    vertical: Effect,
}
impl Demo {
    pub(super) async fn skinning(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        r: &Renderer,
    ) -> Result<()> {
        s.background = Color::BLACK;
        s.background_alpha = 0.0;
        for (color, p, parent) in [
            (0xff9900, Vector3::new(0.0, 4.5, -2.0), None),
            (0x0099ff, Vector3::ZERO, Some(c)),
        ] {
            let h = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(color),
                intensity: 400.0 / (4.0 * std::f64::consts::PI),
                distance: 100.0,
                decay: 2.0,
            }));
            s.get_mut(h)?.position = p;
            if let Some(parent) = parent {
                s.add(parent, h)?;
            }
        }
        let mut m = MeshBasicMaterial::default();
        m.properties.color = Color::BLACK;
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(1000.0, 1000.0, 1, 1)?),
            Material::Basic(m),
        );
        s.get_mut(h)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        let (a, b, i) = super::super::gltf_viewer::load_asset(
            "/web/gallery/assets/tsl-viewport/models/gltf/Michelle.glb",
        )
        .await?;
        let imported = crate::gltf::import_animated_decoded(&a, &b, &i)?;
        let instance = imported.instantiate(s)?;
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        self.mixer = Some(mixer);
        let mut colors = 186u32;
        let mut metals = 187u32;
        let mut random = |_| {
            let mut v = [0.0; 4];
            for value in &mut v[..4] {
                colors = colors.wrapping_mul(1664525).wrapping_add(1013904223);
                *value = (colors as f64 / 4294967296.0) as f32;
            }
            metals = metals.wrapping_mul(1664525).wrapping_add(1013904223);
            v[3] = (metals as f64 / 4294967296.0) as f32;
            for _ in 0..3 {
                metals = metals.wrapping_mul(1664525).wrapping_add(1013904223);
            }
            v
        };
        let values: Vec<[f32; 4]> = (0..30).map(&mut random).collect();
        let buffer = GpuBuffer::new(r, bytemuck::cast_slice(&values), BufferAccess::Read)?;
        let osc = ((uniform(0, Type::Float) * float(0.1) + float(0.75))
            * float(std::f32::consts::TAU))
        .sin()
            * float(0.5)
            + float(0.5);
        let program = Arc::new(
            SurfaceNodes {
                color: Some(mix(
                    splat(float(1.0), Type::Vec3),
                    instanced_attribute(0).rgb(),
                    osc.clone(),
                )),
                metalness: Some(instanced_attribute(0).swizzle("w") * osc),
                ..Default::default()
            }
            .build(r, &[(&buffer, Type::Vec4)], &[])
            .await?,
        );
        for h in instance.meshes {
            let node = s.get_mut(h)?;
            node.instances = (0..30)
                .map(|i| Instance {
                    matrix: Matrix4::from_translation(Vector3::new(
                        -200.0 + (i % 5) as f64 * 70.0,
                        (i / 5) as f64 * -200.0,
                        0.0,
                    )),
                    ..Default::default()
                })
                .collect();
            if let NodeKind::Mesh(m) = &mut node.kind {
                let mut material = MeshStandardMaterial {
                    roughness: 0.1,
                    energy_conservation: true,
                    ..Default::default()
                };
                material.properties.vertex_program = Some(program.clone());
                m.materials = vec![Arc::new(Material::Standard(material))];
            }
            self.objects.push(h);
        }
        let scene = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: HDR,
                samples: 4,
                ..Default::default()
            },
        )?;
        let intermediate = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: HDR,
                depth_buffer: false,
                ..Default::default()
            },
        )?;
        let z = tsl::viewport::perspective_depth_to_view_z(
            depth_texture(uv()),
            float(0.01),
            float(40.0),
        );
        let depth = ((-z - float(0.01)) / float(39.99) - float(0.15)) / float(0.15);
        let color = gaussian_blur(
            tsl::Texture::Input,
            uv(),
            uniform(0, Type::Vec2) * depth.clamp(float(0.0), float(1.0)),
            4,
        )?;
        let view = scene
            .depth_texture()
            .unwrap()
            .create_view(&Default::default());
        self.blur = Some(Blur {
            horizontal: multisampled_depth_effect(r, HDR, &color, &view).await?,
            vertical: multisampled_depth_effect(r, HDR, &color, &view).await?,
            scene,
            intermediate,
        });
        Ok(())
    }
}
impl Blur {
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<()> {
        if self.scene.width != out.width || self.scene.height != out.height {
            self.scene.set_size(&r.device, out.width, out.height)?;
            self.intermediate
                .set_size(&r.device, out.width, out.height)?;
            let view = self
                .scene
                .depth_texture()
                .unwrap()
                .create_view(&Default::default());
            self.horizontal.set_depth(r, &view)?;
            self.vertical.set_depth(r, &view)?;
        }
        r.render(s, c, &self.scene)?;
        self.horizontal.parameters[0] = [1.0 / out.width as f32, 0.0, 0.0, 0.0];
        self.vertical.parameters[0] = [0.0, 1.0 / out.height as f32, 0.0, 0.0];
        self.horizontal
            .apply(r, &self.scene, None, &self.intermediate)?;
        self.vertical.apply(r, &self.intermediate, None, out)?;
        Ok(())
    }
}
