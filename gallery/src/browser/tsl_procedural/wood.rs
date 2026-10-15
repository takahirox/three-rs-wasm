use super::super::gltf_viewer::fetch;
use super::*;
use tsl::wood::WoodNodes;
#[derive(serde::Deserialize)]
struct Preset {
    parameters: [f32; 16],
    dark: u32,
    light: u32,
}
pub(in crate::browser) struct Wood {
    viewer: OrbitViewer,
    custom: Object3D,
    pending: [Option<f32>; 20],
}
fn nodes() -> WoodNodes {
    let p = |i: usize| uniform(i / 4, Type::Vec4).swizzle(["x", "y", "z", "w"][i % 4]);
    WoodNodes {
        center_size: p(0),
        large_warp_scale: p(1),
        large_grain_stretch: p(2),
        small_warp_strength: p(3),
        small_warp_scale: p(4),
        fine_warp_strength: p(5),
        fine_warp_scale: p(6),
        ring_thickness: p(7),
        ring_bias: p(8),
        ring_size_variance: p(9),
        ring_variance_scale: p(10),
        bark_thickness: p(11),
        splotch_scale: p(12),
        splotch_intensity: p(13),
        cell_scale: p(14),
        cell_size: p(15),
        dark: uniform(4, Type::Vec3),
        light: uniform(5, Type::Vec3),
    }
}
impl Wood {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 75.,
            aspect,
            near: 0.1,
            far: 1000.,
            ..Default::default()
        }));
        let p = Vector3::new(-0.1, 5., 0.548);
        let center = Vector3::new(0., 0., 0.548);
        s.get_mut(c)?.position = p;
        s.look_at(c, center)?;
        let offset = p - center;
        let mut viewer = OrbitViewer::from_camera(center, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        s.background = Color::WHITE;
        s.tone_mapping = ToneMapping::Neutral;
        s.environment_intensity = 2.;
        s.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_hdr(
            &fetch("/web/gallery/assets/tsl-procedural/san_giuseppe_bridge_2k.hdr").await?,
        )?));
        let geometries = super::super::tsl_materials::geometries(
            &fetch("/web/gallery/assets/tsl-procedural/wood-geometry.bin").await?,
        )?;
        let mut geometries = geometries.into_iter();
        let block = Arc::new(
            geometries
                .next()
                .ok_or(Error::Asset("wood geometry".into()))?,
        );
        let base = s.insert(NodeKind::Group);
        s.get_mut(base)?.quaternion = Quaternion::from_rotation_z(-std::f64::consts::FRAC_PI_2);
        s.get_mut(base)?.position.z = 0.548;
        let position = |x: f64, y: f64| Vector3::new(0., y - 2., x - 5. + 0.45);
        let mut text = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        text.properties.color = Color::BLACK;
        for (i, g) in geometries.enumerate() {
            let h = mesh(s, Arc::new(g), Material::Standard(text.clone()));
            s.add(base, h)?;
            s.get_mut(h)?.quaternion = Quaternion::from_rotation_y(-std::f64::consts::FRAC_PI_2);
            s.get_mut(h)?.position = if i < 4 {
                position(-1., i as f64)
            } else if i < 14 {
                position((i - 4) as f64, -1.)
            } else {
                position(4., 5.)
            };
        }
        let view_distance = WgslFn::new(
            "wood_view_distance",
            "fn wood_view_distance()->f32{return length(fragment_surface.view_position);}",
            &[],
            Type::Float,
        )
        .unwrap()
        .call(&[]);
        let color = nodes().color(position_local() + uniform(6, Type::Vec3), view_distance);
        let program = Arc::new(
            SurfaceNodes {
                color: Some(color),
                output: Some(output().max(float(0.))),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        );
        let presets: Vec<Preset> = serde_json::from_slice(
            &fetch("/web/gallery/assets/tsl-procedural/wood-presets.json").await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let mut seed = 186u32;
        let mut custom = None;
        for (x, y) in (0..10)
            .flat_map(|x| (0..4).map(move |y| (x, y)))
            .chain(std::iter::once((5, 5)))
        {
            let preset = &presets[if y == 5 { 0 } else { x }];
            let mut m = MeshPhysicalMaterial::default();
            m.base.energy_conservation = true;
            m.clearcoat = if y == 0 { 0. } else { 1. };
            m.clearcoat_roughness = if y == 5 { 0.2 } else { [0., 1., 0.4, 0.1][y] };
            m.base.properties.vertex_program = Some(program.clone());
            let u = &mut m.base.properties.vertex_uniforms;
            for (i, v) in preset.parameters.iter().enumerate() {
                u[i / 4][i % 4] = *v;
            }
            for (i, color) in [(4, preset.dark), (5, preset.light)] {
                let c = Color::from_hex(color).0;
                u[i] = [c.x as f32, c.y as f32, c.z as f32, 0.];
            }
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            u[6] = [-0.1, 0., (seed as f64 / 4294967296.) as f32, 0.];
            let h = mesh(s, block.clone(), Material::Physical(m));
            s.add(base, h)?;
            s.get_mut(h)?.position = position(x as f64, y as f64);
            if y == 5 {
                custom = Some(h);
            }
        }
        let graph=WgslFn::new("wood_grid",r#"fn wood_grid(p:vec3<f32>)->vec4<f32>{let coord=p.xz;let grid=fract(coord);let fw=fwidth(coord);let smoothing=max(fw.x,fw.y)*0.5;let square=max(abs(grid.x-0.5),abs(grid.y-0.5));let dots=smoothstep(0.03+smoothing,0.03-smoothing,square);let lines=max(smoothstep(0.005+smoothing,0.005-smoothing,abs(grid.x-0.5)),smoothstep(0.005+smoothing,0.005-smoothing,abs(grid.y-0.5)));return mix(vec4(1.0,1.0,1.0,0.0),vec4(0.5,0.5,0.5,1.0),max(dots,lines))*smoothstep(30.0,10.0,length(p));}"#,&[Type::Vec3],Type::Vec4).unwrap().call(&[position_world()]);
        let mut material = NodeMaterial::new(graph).build(r, &[]).await?;
        material.properties.transparent = true;
        let h = mesh(
            s,
            Arc::new(CircleGeometry::build(40., 32, 0., std::f64::consts::TAU)?),
            Material::Shader(material),
        );
        s.get_mut(h)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        s.get_mut(h)?.render_order = -1;
        Ok(Self {
            viewer,
            custom: custom.unwrap(),
            pending: [None; 20],
        })
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, _d: f64, _a: bool) -> Result<()> {
        for i in 0..20 {
            if let Some(v) = self.pending[i].take() {
                self.apply_parameter(s, i, v)?;
            }
        }
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.1;
            p.far = 1000.;
        }
        Ok(())
    }
    fn apply_parameter(&mut self, s: &mut Scene, i: usize, v: f32) -> Result<()> {
        let NodeKind::Mesh(m) = &mut s.get_mut(self.custom)?.kind else {
            return Err(Error::Invalid("wood mesh"));
        };
        let Material::Physical(m) = Arc::make_mut(&mut m.materials[0]) else {
            return Err(Error::Invalid("wood material"));
        };
        match i {
            0..=15 => m.base.properties.vertex_uniforms[i / 4][i % 4] = v,
            16 | 17 => {
                let c = Color::from_hex(v as u32).0;
                m.base.properties.vertex_uniforms[i - 12] =
                    [c.x as f32, c.y as f32, c.z as f32, 0.];
            }
            18 => {}
            19 => m.clearcoat_roughness = v as f64,
            _ => return Err(Error::Invalid("wood parameter")),
        }
        Ok(())
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= 20 || !v.is_finite() {
            return Err(Error::Invalid("wood parameter"));
        }
        self.pending[i] = Some(v);
        Ok(())
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
            self.viewer.orbit_pixels(dx, dy, w, h, 0.01, 1000.);
        }
        Ok(())
    }
}
