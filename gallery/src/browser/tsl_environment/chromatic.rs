use super::*;
use crate::postprocessing::Effect;
pub(super) struct Chromatic {
    scene: RenderTarget,
    encoded: RenderTarget,
    encode: Effect,
    aberration: Effect,
}
impl Chromatic {
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        params: &[f32; 8],
    ) -> Result<()> {
        for target in [&mut self.scene, &mut self.encoded] {
            target.set_size(&r.device, out.width, out.height)?;
        }
        r.render(s, c, &self.scene)?;
        if params[0] > 0.5 {
            self.encode.apply(r, &self.scene, None, &self.encoded)?;
            self.aberration.parameters[0] = [params[1], params[2], params[3], params[4]];
            self.aberration.apply(r, &self.encoded, None, out)?;
        } else {
            self.encode.apply(r, &self.scene, None, out)?;
        }
        Ok(())
    }
}
impl Demo {
    pub(super) async fn chromatic(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        use std::f64::consts::TAU;
        s.background = Color::from_hex(0x0a0a0a);
        s.environment = Some(super::super::room_environment::environment(r)?);
        let torus = |radius, tube, radial, tubular| {
            TorusGeometry::build(radius, tube, radial, tubular, TAU, 0.0, TAU)
        };
        let geometries = [
            BoxGeometry::build(3.0, 3.0, 3.0)?,
            SphereGeometry::build(2.0, 32, 16)?,
            CylinderGeometry::build(0.0, 2.0, 4.0, 8, 1, false, 0.0, TAU)?,
            CylinderGeometry::build(1.5, 1.5, 4.0, 8, 1, false, 0.0, TAU)?,
            torus(2.0, 0.8, 8, 16)?,
            OctahedronGeometry::build(2.5, 0)?,
            IcosahedronGeometry::build(2.5, 0)?,
            TorusKnotGeometry::build(1.5, 0.5, 64, 8, 2, 3)?,
        ]
        .map(Arc::new);
        let colors = [
            0xff0000, 0x00ff00, 0x0000ff, 0xffff00, 0xff00ff, 0x00ffff, 0xffffff, 0xff8800,
        ];
        let central = s.insert(NodeKind::Group);
        let mut m = standard(0xffffff, 0.1, 1.0);
        m.emissive = Color::from_hex(0x222222);
        let h = mesh(s, Arc::new(torus(5.0, 1.5, 16, 32)?), Material::Standard(m));
        s.add(central, h)?;
        for i in 0..6 {
            let angle = i as f64 / 6.0 * TAU;
            let h = mesh(
                s,
                geometries[i].clone(),
                Material::Standard(standard(colors[i], 0.2, 0.8)),
            );
            s.get_mut(h)?.position = Vector3::new(angle.cos() * 3.0, 0.0, angle.sin() * 3.0);
            s.get_mut(h)?.scale = Vector3::splat(0.5);
            s.add(central, h)?;
        }
        self.groups.push(central);
        for i in 0..12 {
            let angle = i as f64 / 12.0 * TAU;
            let g = s.insert(NodeKind::Group);
            s.get_mut(g)?.position = Vector3::new(
                angle.cos() * 15.0,
                (i as f64 * 0.5).sin() * 2.0,
                angle.sin() * 15.0,
            );
            let h = mesh(
                s,
                geometries[i % 8].clone(),
                Material::Standard(standard(colors[i % 8], 0.2, 0.8)),
            );
            s.add(g, h)?;
            self.groups.push(g);
        }
        let mut seed = 186;
        let mut positions = Vec::with_capacity(600);
        for _ in 0..200 {
            let radius = 25.0 + random(&mut seed) * 10.0;
            let theta = random(&mut seed) * TAU;
            let phi = random(&mut seed) * std::f64::consts::PI;
            positions.extend(
                [
                    radius * phi.sin() * theta.cos(),
                    radius * phi.cos(),
                    radius * phi.sin() * theta.sin(),
                ]
                .map(|v| v as f32),
            );
        }
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(positions, 3, false)?),
        );
        s.insert(NodeKind::Points(Points {
            geometry: Arc::new(g),
            material: Arc::new(Material::Shader(
                NodeMaterial::new(splat(float(1.0), Type::Vec3))
                    .build(r, &[])
                    .await?,
            )),
        }));
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        for i in 0..=20 {
            let k = -20.0 + i as f32 * 2.0;
            positions.extend([-20.0, 0.0, k, 20.0, 0.0, k, k, 0.0, -20.0, k, 0.0, 20.0]);
            let c = Color::from_hex(if i == 10 { 0x444444 } else { 0x222222 });
            for _ in 0..4 {
                colors.extend([c.0.x as f32, c.0.y as f32, c.0.z as f32]);
            }
        }
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(positions, 3, false)?),
        );
        g.set_attribute(
            "color",
            Attribute::F32(BufferAttribute::new(colors, 3, false)?),
        );
        let h = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Line(LineBasicMaterial {
                properties: MaterialProperties {
                    vertex_colors: true,
                    ..Default::default()
                },
                ..Default::default()
            })),
            segments: true,
        }));
        s.get_mut(h)?.position.y = -10.0;
        let options = RenderTargetOptions {
            samples: 4,
            format: HDR,
            ..Default::default()
        };
        let scene = RenderTarget::with_options(&r.device, 1, 1, options.clone())?;
        let encoded = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                samples: 1,
                ..options
            },
        )?;
        let encode = tsl::effect(
            r,
            HDR,
            &vec4(
                tsl::display::srgb(tsl::Texture::Input.sample(uv()).rgb()),
                float(1.0),
            ),
        )
        .await?;
        let u = uniform(0, Type::Vec4);
        let aberration = tsl::effect(
            r,
            HDR,
            &tsl::display::chromatic_aberration(
                tsl::Texture::Input,
                uv(),
                u.x(),
                vec2(u.y(), u.swizzle("z")),
                u.swizzle("w"),
            ),
        )
        .await?;
        self.ca = Some(Chromatic {
            scene,
            encoded,
            encode,
            aberration,
        });
        self.params = [1.0, 1.5, 0.5, 0.5, 1.2, 1.0, 1.0, 0.0];
        Ok(())
    }
}
