use super::super::gltf_viewer::fetch;
use super::*;
use crate::environment::EnvironmentMap;
use tsl::materialx::{
    self as mx,
    Dimension::{D2, D3},
};
pub(in crate::browser) struct Noise {
    time: f64,
    viewer: OrbitViewer,
    spheres: Vec<Object3D>,
    light: Object3D,
    sky: Object3D,
}
fn graph(i: usize) -> tsl::Node {
    let offset2 = vec2(time() * float(0.35), time() * float(0.19));
    let offset3 = vec3(
        time() * float(0.35),
        time() * float(0.19),
        time() * float(0.27),
    );
    let p2 = uv() * float(12.) + offset2.clone();
    let p3 = normal_world() * float(4.) + offset3.clone();
    let dimension = if i.is_multiple_of(2) { D2 } else { D3 };
    let p = if i.is_multiple_of(2) {
        p2.clone()
    } else {
        p3.clone()
    };
    match i {
        0 | 1 => mx::perlin_vec3(p, dimension),
        2 | 3 => splat(mx::cell(p, dimension), Type::Vec3),
        4..=7 => splat(
            mx::worley(p, dimension, float(1.), float(if i >= 6 { 1. } else { 0. })),
            Type::Vec3,
        ),
        8 | 9 => {
            let p = if i == 8 {
                vec3(p2.x(), p2.y(), float(0.))
            } else {
                p3
            };
            mx::fractal_vec3(p * float(0.35), D3, float(3.), float(2.), float(0.5))
        }
        10..=19 => {
            let p = if i.is_multiple_of(2) {
                p2 + offset2
            } else {
                p3 + offset3
            };
            let v = match i {
                10 | 11 => mx::perlin(p, dimension) * float(0.5) + float(0.5),
                12 | 13 => mx::cell(p, dimension),
                14..=17 => mx::worley(
                    p,
                    dimension,
                    float(1.),
                    float(if i >= 16 { 1. } else { 0. }),
                ),
                _ => {
                    let p = if i == 18 {
                        vec3(p.x(), p.y(), float(0.))
                    } else {
                        p
                    };
                    mx::fractal(p, D3, float(3.), float(2.), float(0.5))
                }
            };
            splat(v, Type::Vec3)
        }
        _ => WgslFn::new(
            "hex_checker",
            include_str!("noise_hex.wgsl"),
            &[Type::Vec2],
            Type::Vec3,
        )
        .unwrap()
        .call(&[uv() * float(3.) + vec2(time() * float(0.08), time() * float(0.04))]),
    }
}
impl Noise {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 27.,
            aspect,
            near: 1.,
            far: 1000.,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 190.);
        s.look_at(c, Vector3::ZERO)?;
        let viewer = OrbitViewer::from_camera(Vector3::ZERO, 190.);
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 1.25;
        let mut pixels = vec![];
        let mut size = 0;
        for face in ["px", "nx", "py", "ny", "pz", "nz"] {
            let bytes = fetch(&format!(
                "/web/gallery/assets/tsl-environment/textures/cube/pisaHDR/{face}.hdr"
            ))
            .await?;
            let im = image::load_from_memory(&bytes).map_err(|e| Error::Asset(e.to_string()))?;
            size = im.width();
            pixels.extend(
                im.to_rgba32f()
                    .as_raw()
                    .iter()
                    .map(|v| half::f16::from_f32(v.clamp(0., 65504.))),
            );
        }
        let cube = GpuTexture::from_cube_hdr(r, size, &pixels)?;
        s.environment = Some(Arc::new(EnvironmentMap::from_cube_texture(r, &cube)?));
        let direction = normal_world();
        let color=WgslFn::new("noise_sky_cube","fn noise_sky_cube(t:texture_cube<f32>,s:sampler,d:vec3<f32>)->vec4<f32>{return textureSampleLevel(t,s,d,0.0);}",&[Type::TextureCube,Type::Sampler,Type::Vec3],Type::Vec4)?.call(&[tsl::Texture::External(0).node(),tsl::Texture::External(0).sampler(),vec3(-direction.x(),direction.y(),direction.swizzle("z"))]);
        let mut m = NodeMaterial::new(color)
            .build_with_texture_types(r, &[(&cube.view, &cube.sampler, Type::TextureCube)])
            .await?;
        m.properties.side = Side::Back;
        m.properties.depth_write = false;
        m.properties.depth_test = false;
        let sky = mesh(
            s,
            Arc::new(SphereGeometry::build(1., 32, 32)?),
            Material::Shader(m),
        );
        s.get_mut(sky)?.scale = Vector3::splat(10.);
        s.get_mut(sky)?.frustum_culled = false;
        s.get_mut(sky)?.render_order = -10000;
        let geometry = Arc::new(SphereGeometry::build(5., 64, 32)?);
        let mut spheres = vec![];
        for i in 0..21 {
            let program = SurfaceNodes {
                color: Some(graph(i)),
                output: Some(output().max(float(0.))),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?;
            let mut m = MeshPhysicalMaterial::default();
            m.base.energy_conservation = true;
            m.base.properties.vertex_program = Some(Arc::new(program));
            let h = mesh(s, geometry.clone(), Material::Physical(m));
            s.get_mut(h)?.position =
                Vector3::new((i % 7) as f64 * 15.5 - 46.5, 18. - (i / 7) as f64 * 18., 0.);
            spheres.push(h);
        }
        let labels = super::super::tsl_materials::geometries(
            &fetch("/web/gallery/assets/tsl-procedural/noise-labels.bin").await?,
        )?;
        for label in labels {
            mesh(
                s,
                Arc::new(label),
                Material::Basic(MeshBasicMaterial::default()),
            );
        }
        let light = mesh(
            s,
            Arc::new(SphereGeometry::build(0.4, 8, 8)?),
            Material::Basic(MeshBasicMaterial::default()),
        );
        let h = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 1000.,
            distance: 0.,
            decay: 2.,
        }));
        s.add(light, h)?;
        Ok(Self {
            time: 0.,
            viewer,
            spheres,
            light,
            sky,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
        }
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 1.;
            p.far = 1000.;
        }
        s.get_mut(self.sky)?.position = s.get(c)?.position;
        let t = self.time * 0.25;
        s.get_mut(self.light)?.position = Vector3::new(
            (t * 7.).sin() * 30.,
            (t * 5.).cos() * 40.,
            (t * 3.).cos() * 30.,
        );
        for &h in &self.spheres {
            let n = s.get_mut(h)?;
            n.quaternion = Quaternion::from_rotation_y(self.time * 0.3);
            if let NodeKind::Mesh(m) = &mut n.kind {
                Arc::make_mut(&mut m.materials[0])
                    .properties_mut()
                    .vertex_uniforms[0][0] = self.time as f32;
            }
        }
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
            self.viewer.orbit_pixels(dx, dy, w, h, 0., f64::INFINITY);
        }
        Ok(())
    }
}
