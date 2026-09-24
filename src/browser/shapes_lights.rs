//! The environment-mapped heads, the STL loader with shadows, extruded shapes,
//! tweened spot lights and the hemisphere-light flamingo from the pinned WebGL
//! examples.
mod formats;
use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use super::terrain_loaders::parse_obj;
use super::trackball_sprites::{Mode, Trackball};
use crate::{
    Error, Result,
    animation::AnimationMixer,
    attribute::BufferAttribute,
    camera::*,
    curve::{CatmullRomCurve3, CatmullRomType},
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    texture_gpu::GpuTexture,
    tsl::{surface::SurfaceNodes, *},
};
use formats::{ExtrudeOptions, extrude, parse_stl};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
/// `Color.setHSL` in the linear working space.
fn hsl(h: f64, s: f64, l: f64) -> Color {
    let h = (h % 1. + 1.) % 1.;
    if s == 0. {
        return Color::linear(l, l, l);
    }
    let p = if l <= 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let q = 2. * l - p;
    let f = |mut t: f64| {
        if t < 0. {
            t += 1.;
        }
        if t > 1. {
            t -= 1.;
        }
        if t < 1. / 6. {
            q + (p - q) * 6. * t
        } else if t < 0.5 {
            p
        } else if t < 2. / 3. {
            q + (p - q) * 6. * (2. / 3. - t)
        } else {
            q
        }
    };
    Color::linear(f(h + 1. / 3.), f(h), f(h - 1. / 3.))
}
/// A camera-centered background box that samples a cube with x flipped.
async fn sky_box(s: &mut Scene, r: &Renderer, env: &GpuTexture) -> Result<Object3D> {
    let graph = NodeMaterial::new(vec4(
        WgslFn::new(
            "sky_color",
            "fn sky_color(d:vec3<f32>)->vec3<f32>{return textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb;}",
            &[Type::Vec3],
            Type::Vec3,
        )?
        .call(&[position_local()]),
        float(1.),
    ));
    let source = graph.wgsl_with_texture_types(&[Type::TextureCube], &[])?;
    let mut m = ShaderMaterial::new(Arc::new(
        ShaderProgram::with_projection_and_dimensions(
            r,
            &source,
            &[(&env.view, &env.sampler)],
            &[wgpu::TextureViewDimension::Cube],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(out.clip.xy,out.clip.w,out.clip.w);return out;}",
        )
        .await?,
    ));
    m.properties.side = Side::Back;
    m.properties.depth_write = false;
    m.properties.depth_test = false;
    let sky = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(BoxGeometry::build(1., 1., 1.)?),
        Arc::new(Material::Shader(m)),
    )));
    let n = s.get_mut(sky)?;
    n.frustum_culled = false;
    n.render_order = -10000;
    Ok(sky)
}
/// One running TWEEN.Tween: the light's values captured at start, the targets,
/// the start time and duration in milliseconds.
struct Tween {
    light: usize,
    position: bool,
    start: f64,
    duration: f64,
    from: [f64; 3],
    to: [f64; 3],
}
struct Spot {
    light: Object3D,
    cone: Object3D,
    angle: f64,
    penumbra: f64,
    position: Vector3,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    seed: u32,
    controls: Option<Controls>,
    trackball: Option<Trackball>,
    sky: Option<Object3D>,
    spots: Vec<Spot>,
    tweens: Vec<Tween>,
    next_tweens: f64,
    mixer: Option<AnimationMixer>,
    hemisphere: Option<[Object3D; 2]>,
    directional: Option<[Object3D; 3]>,
    visible: [bool; 2],
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            243 => (50., 0.1, 100., Vector3::new(0., 0., 13.)),
            244 => (35., 1., 15., Vector3::new(3., 0.15, 3.)),
            245 => (45., 1., 1000., Vector3::new(0., 0., 500.)),
            246 => (35., 0.1, 100., Vector3::new(4.6, 2.2, -2.1)),
            _ => (30., 1., 5000., Vector3::new(0., 0., 250.)),
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near,
            far,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = position;
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            seed: 186,
            controls: None,
            trackball: None,
            sky: None,
            spots: vec![],
            tweens: vec![],
            next_tweens: 0.,
            mixer: None,
            hemisphere: None,
            directional: None,
            visible: [true; 2],
        };
        match id {
            243 => d.cubemap(s, r).await?,
            244 => d.stl(s).await?,
            245 => d.extrusions(s, c)?,
            246 => d.spotlights(s, c)?,
            _ => d.hemisphere_scene(s, r).await?,
        }
        Ok(d)
    }
    async fn cubemap(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let mut faces = vec![];
        for face in ["px", "nx", "py", "ny", "pz", "nz"] {
            let mut t =
                decode_texture_image(&fetch(&format!("{ASSETS}/castle-{face}.jpg")).await?).await?;
            t.srgb = true;
            faces.push(t);
        }
        let env = GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        self.sky = Some(sky_box(s, r, &env).await?);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 3.,
        }));
        s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 200.,
            distance: 0.,
            decay: 2.,
        }));
        // envmap_fragment on Lambert: reflection or refraction of the view ray about
        // the world normal, x flipped, then Multiply or Mix by the reflectivity.
        let head = parse_obj(&String::from_utf8_lossy(
            &fetch("/web/models/WaltHead.obj").await?,
        ))
        .into_iter()
        .next()
        .ok_or(Error::Asset("WaltHead: no object".into()))?;
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(head.positions)?);
        g.set_attribute("normal", vec3s(head.normals)?);
        let g = Arc::new(g);
        for (x, hex, ray, combine) in [
            (0., 0xffffff, "reflect(v,w)", "value.rgb*e"),
            (-6., 0xfff700, "refract(v,w,0.95)", "value.rgb*e"),
            (6., 0xffaa00, "reflect(v,w)", "mix(value.rgb,e,0.3)"),
        ] {
            let env_fn = WgslFn::new(
                "cube_env",
                &format!(
                    "fn cube_env(value:vec4<f32>)->vec4<f32>{{let n=normalize(fragment_surface.normal)*select(-1.0,1.0,fragment_front);let w=normalize(transpose(mat3x3(u.view[0].xyz,u.view[1].xyz,u.view[2].xyz))*n);let v=normalize(fragment_surface.position-u.camera.xyz);let d={ray};let e=textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb;return vec4({combine},value.a);}}"
                ),
                &[Type::Vec4],
                Type::Vec4,
            )?
            .call(&[output()]);
            let program = SurfaceNodes {
                output: Some(env_fn),
                ..Default::default()
            }
            .build_with_texture_types(r, &[], &[(&env.view, &env.sampler, Type::TextureCube)])
            .await?;
            let mut m = MeshLambertMaterial::default();
            m.properties.color = Color::from_hex(hex);
            m.properties.vertex_program = Some(Arc::new(program));
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                g.clone(),
                Arc::new(Material::Lambert(m)),
            )));
            let n = s.get_mut(h)?;
            n.scale = Vector3::splat(0.1);
            n.position = Vector3::new(x, -3., 0.);
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI / 1.5, true);
        controls.min_polar = PI / 4.;
        controls.set_target(Vector3::ZERO);
        self.controls = Some(controls);
        Ok(())
    }
    async fn stl(&mut self, s: &mut Scene) -> Result<()> {
        s.background = Color::from_hex(0x72645b);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x72645b),
            near: 2.,
            far: 15.,
        });
        let mut ground = MeshPhongMaterial {
            specular: Color::from_hex(0x474747),
            ..Default::default()
        };
        ground.properties.color = Color::from_hex(0xcbcbcb);
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(40., 40., 1, 1)?),
            Arc::new(Material::Phong(ground)),
        )));
        let n = s.get_mut(plane)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.position.y = -0.5;
        n.receive_shadow = true;
        let phong = |hex: u32| {
            let mut m = MeshPhongMaterial {
                specular: Color::from_hex(0x494949),
                shininess: 200.,
                ..Default::default()
            };
            m.properties.color = Color::from_hex(hex);
            Arc::new(Material::Phong(m))
        };
        let shared = phong(0xd5d5d5);
        for (path, material, position, rotation, scale) in [
            (
                "stl/ascii/slotted_disk.stl",
                Some(phong(0xff9c7c)),
                [0., -0.25, 0.6],
                [0., -PI / 2., 0.],
                0.5,
            ),
            (
                "stl/binary/pr2_head_pan.stl",
                Some(shared.clone()),
                [0., -0.37, -0.6],
                [-PI / 2., 0., 0.],
                2.,
            ),
            (
                "stl/binary/pr2_head_tilt.stl",
                Some(shared.clone()),
                [0.136, -0.37, -0.6],
                [-PI / 2., 0.3, 0.],
                2.,
            ),
            (
                "stl/binary/colored.stl",
                None,
                [0.5, 0.2, 0.],
                [-PI / 2., PI / 2., 0.],
                0.3,
            ),
        ] {
            let stl = parse_stl(&fetch(&format!("{ASSETS}/{path}")).await?)?;
            let mut g = BufferGeometry::default();
            g.set_attribute("position", vec3s(stl.positions)?);
            g.set_attribute("normal", vec3s(stl.normals)?);
            let material = match (material, stl.colors) {
                (_, Some((colors, alpha))) => {
                    g.set_attribute("color", vec3s(colors)?);
                    let mut m = MeshPhongMaterial::default();
                    m.properties.opacity = alpha;
                    m.properties.vertex_colors = true;
                    Arc::new(Material::Phong(m))
                }
                (Some(m), None) => m,
                (None, None) => shared.clone(),
            };
            let h = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(g), material)));
            let n = s.get_mut(h)?;
            n.position = Vector3::from_array(position);
            n.quaternion = Euler {
                angles: Vector3::from_array(rotation),
                order: EulerOrder::XYZ,
            }
            .quaternion();
            n.scale = Vector3::splat(scale);
            n.cast_shadow = true;
            n.receive_shadow = true;
        }
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0x8d7c7c),
            ground: Color::from_hex(0x494966),
            intensity: 3.,
        }));
        s.get_mut(hemi)?.position = Vector3::Y;
        // addShadowedLight: a 2 × 2 orthographic shadow camera from 1 to 4.
        for (p, hex, intensity) in [
            (Vector3::new(1., 1., 1.), 0xffffff, 3.5),
            (Vector3::new(0.5, 1., -1.), 0xffd500, 3.),
        ] {
            let light = s.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(hex),
                intensity,
                target: Vector3::ZERO,
            }));
            let n = s.get_mut(light)?;
            n.position = p;
            n.cast_shadow = true;
            n.shadow = crate::shadow::Shadow {
                near: 1.,
                far: 4.,
                extent: 1.,
                bias: -0.002,
                ..Default::default()
            };
        }
        Ok(())
    }
    fn extrusions(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.background = Color::from_hex(0x222222);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x666666),
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 3.,
            distance: 0.,
            decay: 0.,
        }));
        s.get_mut(light)?.position = Vector3::new(0., 0., 500.);
        let v = Vector3::new;
        let mut closed = CatmullRomCurve3::new(vec![
            v(-60., -100., 60.),
            v(-60., 20., 60.),
            v(-60., 120., 60.),
            v(60., 20., -60.),
            v(60., -100., -60.),
        ]);
        closed.curve_type = CatmullRomType::Uniform { tension: 0.5 };
        closed.closed = true;
        let triangle: Vec<[f64; 2]> = (0..3)
            .map(|i| {
                let a = 2. * i as f64 / 3. * PI;
                [a.cos() * 20., a.sin() * 20.]
            })
            .collect();
        // THREE.MathUtils.randFloat( - 50, 50 ): low + Math.random() × ( high − low ).
        let random_points: Vec<Vector3> = (0..10)
            .map(|i| {
                let y = -50. + random(&mut self.seed) * 100.;
                let z = -50. + random(&mut self.seed) * 100.;
                v((i as f64 - 4.5) * 50., y, z)
            })
            .collect();
        let spline = CatmullRomCurve3::new(random_points);
        let star: Vec<[f64; 2]> = (0..10)
            .map(|i| {
                let l = if i % 2 == 1 { 10. } else { 20. };
                let a = i as f64 / 5. * PI;
                [a.cos() * l, a.sin() * l]
            })
            .collect();
        let lambert = |hex: u32| {
            let mut m = MeshLambertMaterial::default();
            m.properties.color = Color::from_hex(hex);
            Arc::new(Material::Lambert(m))
        };
        let (m1, m2) = (lambert(0xb00000), lambert(0xff8000));
        for (shape, options, materials, position) in [
            (
                &triangle,
                ExtrudeOptions {
                    steps: 100,
                    depth: 1.,
                    bevel: None,
                    path: Some(&closed),
                },
                vec![m1.clone()],
                Vector3::ZERO,
            ),
            (
                &star,
                ExtrudeOptions {
                    steps: 200,
                    depth: 1.,
                    bevel: None,
                    path: Some(&spline),
                },
                vec![m2.clone()],
                Vector3::ZERO,
            ),
            (
                &star,
                ExtrudeOptions {
                    steps: 1,
                    depth: 20.,
                    bevel: Some((2., 4., 1)),
                    path: None,
                },
                vec![m1.clone(), m2.clone()],
                v(50., 100., 50.),
            ),
        ] {
            let (positions, groups) = extrude(shape, &options)?;
            let mut g = BufferGeometry::default();
            g.set_attribute("position", vec3s(positions)?);
            g.compute_vertex_normals()?;
            g.groups = groups
                .iter()
                .enumerate()
                .map(|(i, &(start, count))| Group {
                    start,
                    count,
                    material_index: i,
                })
                .collect();
            let mut mesh = Mesh::new(Arc::new(g), materials[0].clone());
            if materials.len() > 1 {
                mesh.materials = materials;
            }
            let h = s.insert(NodeKind::Mesh(mesh));
            s.get_mut(h)?.position = position;
        }
        let (w, h, _) = viewport_css();
        let mut t = Trackball::new(s, c, Vector2::new(w, h))?;
        t.distance = (200., 500.);
        self.trackball = Some(t);
        Ok(())
    }
    fn spotlights(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let mut floor = MeshPhongMaterial::default();
        floor.properties.color = Color::from_hex(0x808080);
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(100., 100., 1, 1)?),
            Arc::new(Material::Phong(floor)),
        )));
        let n = s.get_mut(h)?;
        n.quaternion = Quaternion::from_rotation_x(-PI * 0.5);
        n.position = Vector3::new(0., -0.05, 0.);
        n.receive_shadow = true;
        let mut box_material = MeshPhongMaterial::default();
        box_material.properties.color = Color::from_hex(0xaaaaaa);
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(0.3, 0.1, 0.2)?),
            Arc::new(Material::Phong(box_material)),
        )));
        let n = s.get_mut(h)?;
        n.position = Vector3::new(0., 0.5, 0.);
        n.cast_shadow = true;
        n.receive_shadow = true;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x444444),
            intensity: 1.,
        }));
        // SpotLightHelper's cone: five rays and a 32-segment rim, as line segments.
        let mut cone = vec![
            0., 0., 0., 0., 0., 1., 0., 0., 0., 1., 0., 1., 0., 0., 0., -1., 0., 1., 0., 0., 0.,
            0., 1., 1., 0., 0., 0., 0., -1., 1.,
        ];
        for i in 0..32 {
            let (p1, p2) = (i as f64 / 32. * PI * 2., (i + 1) as f64 / 32. * PI * 2.);
            cone.extend([p1.cos(), p1.sin(), 1., p2.cos(), p2.sin(), 1.]);
        }
        let mut cone_geometry = BufferGeometry::default();
        cone_geometry.set_attribute(
            "position",
            vec3s(cone.into_iter().map(|v| v as f32).collect())?,
        );
        let cone_geometry = Arc::new(cone_geometry);
        for (hex, position) in [
            (0xff7f00, [1.5, 4., 4.5]),
            (0x00ff7f, [0., 4., 3.5]),
            (0x7f00ff, [-1.5, 4., 4.5]),
        ] {
            let light = s.insert(NodeKind::Light(Light::Spot {
                color: Color::from_hex(hex),
                intensity: 10.,
                target: Vector3::ZERO,
                distance: 50.,
                decay: 2.,
                angle: 0.3,
                penumbra: 0.2,
            }));
            let n = s.get_mut(light)?;
            n.position = Vector3::from_array(position);
            n.cast_shadow = true;
            // SpotLightShadow: fov from the angle, near 0.5, far = light.distance.
            n.shadow = crate::shadow::Shadow {
                near: 0.5,
                far: 50.,
                ..Default::default()
            };
            let mut m = LineBasicMaterial::default();
            m.properties.color = Color::from_hex(hex);
            m.properties.fog = false;
            let cone = s.insert(NodeKind::Line(Line {
                geometry: cone_geometry.clone(),
                material: Arc::new(Material::Line(m)),
                segments: true,
            }));
            self.spots.push(Spot {
                light,
                cone,
                angle: 0.3,
                penumbra: 0.2,
                position: Vector3::from_array(position),
            });
        }
        let mut controls = Controls::new(None, (1., 10.), PI / 2., true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.update_spots(s)?;
        Ok(())
    }
    /// The lights' tweened state, and SpotLightHelper.update() for each cone.
    fn update_spots(&mut self, s: &mut Scene) -> Result<()> {
        for spot in &self.spots {
            if let NodeKind::Light(Light::Spot {
                angle, penumbra, ..
            }) = &mut s.get_mut(spot.light)?.kind
            {
                *angle = spot.angle;
                *penumbra = spot.penumbra;
            }
            s.get_mut(spot.light)?.position = spot.position;
            let width = 50. * spot.angle.tan();
            let n = s.get_mut(spot.cone)?;
            n.position = spot.position;
            n.scale = Vector3::new(width, width, 50.);
            s.look_at(spot.cone, Vector3::ZERO)?;
        }
        Ok(())
    }
    /// updateTweens: every light gets an angle/penumbra tween and a position tween.
    fn start_tweens(&mut self, at: f64) {
        for (i, spot) in self.spots.iter().enumerate() {
            let angle = random(&mut self.seed) * 0.7 + 0.1;
            let penumbra = random(&mut self.seed) + 1.;
            let duration = random(&mut self.seed) * 3000. + 2000.;
            self.tweens.push(Tween {
                light: i,
                position: false,
                start: at,
                duration,
                from: [spot.angle, spot.penumbra, 0.],
                to: [angle, penumbra, 0.],
            });
            let x = random(&mut self.seed) * 3. - 1.5;
            let y = random(&mut self.seed) + 1.5;
            let z = random(&mut self.seed) * 3. - 1.5;
            let duration = random(&mut self.seed) * 3000. + 2000.;
            self.tweens.push(Tween {
                light: i,
                position: true,
                start: at,
                duration,
                from: spot.position.to_array(),
                to: [x, y, z],
            });
        }
    }
    async fn hemisphere_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let background = hsl(0.6, 0., 1.);
        s.background = background;
        let sky_color = hsl(0.6, 1., 0.6);
        let ground_color = hsl(0.095, 1., 0.75);
        // scene.fog takes the sky shader's bottomColor, white.
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0xffffff),
            near: 1.,
            far: 5000.,
        });
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: sky_color,
            ground: ground_color,
            intensity: 2.,
        }));
        s.get_mut(hemi)?.position = Vector3::new(0., 50., 0.);
        // HemisphereLightHelper( hemiLight, 10 ): a wireframe octahedron turned by
        // rotateY( π / 2 ), sky-colored in its first half, facing away from the light.
        let mut octa = OctahedronGeometry::build(10., 0)?;
        let positions = match octa.attributes.get("position") {
            Some(Attribute::F32(a)) => a.array().to_vec(),
            _ => vec![],
        };
        let rotated: Vec<f32> = positions
            .chunks(3)
            .flat_map(|p| {
                let q = Quaternion::from_rotation_y(PI * 0.5)
                    * Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64);
                [q.x as f32, q.y as f32, q.z as f32]
            })
            .collect();
        let count = rotated.len() / 3;
        let colors: Vec<f32> = (0..count)
            .flat_map(|i| {
                let c = if (i as f64) < count as f64 / 2. {
                    sky_color
                } else {
                    ground_color
                }
                .0;
                [c.x as f32, c.y as f32, c.z as f32]
            })
            .collect();
        octa.set_attribute("position", vec3s(rotated)?);
        octa.set_attribute("color", vec3s(colors)?);
        let mut basic = MeshBasicMaterial::default();
        basic.properties.wireframe = true;
        basic.properties.fog = false;
        basic.properties.vertex_colors = true;
        let helper = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(octa),
            Arc::new(Material::Basic(basic)),
        )));
        s.get_mut(helper)?.position = Vector3::new(0., 50., 0.);
        s.look_at(helper, Vector3::new(0., -50., 0.))?;
        let dir_color = hsl(0.1, 1., 0.95);
        let light_position = Vector3::new(-1., 1.75, 1.) * 30.;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: dir_color,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = light_position;
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            map_size: Some(2048),
            near: 0.5,
            far: 3500.,
            extent: 50.,
            bias: -0.0001,
            ..Default::default()
        };
        // DirectionalLightHelper( dirLight, 10 ): the light plane and the target line.
        let mut line = LineBasicMaterial::default();
        line.properties.color = dir_color;
        line.properties.fog = false;
        let line = Arc::new(Material::Line(line));
        let mut plane = BufferGeometry::default();
        plane.set_attribute(
            "position",
            vec3s(vec![
                -10., 10., 0., 10., 10., 0., 10., -10., 0., -10., -10., 0., -10., 10., 0.,
            ])?,
        );
        let mut target = BufferGeometry::default();
        target.set_attribute("position", vec3s(vec![0., 0., 0., 0., 0., 1.])?);
        let mut helpers = vec![];
        for (g, length) in [(plane, 1.), (target, light_position.length())] {
            let h = s.insert(NodeKind::Line(Line {
                geometry: Arc::new(g),
                material: line.clone(),
                segments: false,
            }));
            let n = s.get_mut(h)?;
            n.position = light_position;
            n.scale.z = length;
            s.look_at(h, Vector3::ZERO)?;
            helpers.push(h);
        }
        let mut ground_material = MeshLambertMaterial::default();
        ground_material.properties.color = ground_color;
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10000., 10000., 1, 1)?),
            Arc::new(Material::Lambert(ground_material)),
        )));
        let n = s.get_mut(ground)?;
        n.position.y = -33.;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.receive_shadow = true;
        // The sky ShaderMaterial writes its gradient raw; the encoded output decodes it first.
        let top = sky_color.0;
        let gradient = WgslFn::new(
            "sky_gradient",
            &format!(
                "fn sky_gradient(p:vec3<f32>)->vec4<f32>{{let h=normalize(p+vec3(33.0)).y;let c=mix(vec3(1.0),vec3({:?},{:?},{:?}),max(pow(max(h,0.0),0.6),0.0));let d=select(pow((c+vec3(0.055))/1.055,vec3(2.4)),c/12.92,c<=vec3(0.04045));return vec4(d,1.0);}}",
                top.x as f32, top.y as f32, top.z as f32
            ),
            &[Type::Vec3],
            Type::Vec4,
        )?
        .call(&[position_world()]);
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(gradient).wgsl(0)?,
            &[],
            &[],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let mut sky = ShaderMaterial::new(Arc::new(program));
        sky.properties.side = Side::Back;
        sky.properties.fog = false;
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(4000., 32, 15)?),
            Arc::new(Material::Shader(sky)),
        )));
        // The flamingo: scale 0.35, y 15, rotation.y −1, its clip set to one second.
        let (asset, buffers, images) = load_asset(&format!("{ASSETS}/gltf/Flamingo.glb")).await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        for h in s.traverse(instance.roots[0], false)? {
            let n = s.get_mut(h)?;
            if let NodeKind::Mesh(mesh) = &mut n.kind {
                n.cast_shadow = true;
                n.receive_shadow = true;
                for m in &mut mesh.materials {
                    match Arc::make_mut(m) {
                        Material::Standard(m) => m.energy_conservation = true,
                        Material::Physical(m) => m.base.energy_conservation = true,
                        _ => {}
                    }
                }
            }
        }
        let mesh = s
            .traverse(instance.roots[0], false)?
            .into_iter()
            .find(|&h| matches!(s.get(h).map(|n| &n.kind), Ok(NodeKind::Mesh(_))))
            .ok_or(Error::Asset("Flamingo: no mesh".into()))?;
        let n = s.get_mut(mesh)?;
        n.scale = Vector3::splat(0.35);
        n.position.y = 15.;
        n.quaternion = Quaternion::from_rotation_y(-1.);
        n.matrix_auto_update = true;
        let mut mixer = AnimationMixer::default();
        let action = mixer.play(instance.clips[0].clone())?;
        mixer.actions[action].time_scale = instance.clips[0].duration();
        self.mixer = Some(mixer);
        self.hemisphere = Some([hemi, helper]);
        self.directional = Some([light, helpers[0], helpers[1]]);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        match self.id {
            243 => {
                if let Some(sky) = self.sky {
                    let p = s.get(c)?.position;
                    s.get_mut(sky)?.position = p;
                }
            }
            244 => {
                let timer = t * 0.5;
                s.get_mut(c)?.position = Vector3::new(timer.cos() * 3., 0.15, timer.sin() * 3.);
                s.look_at(c, Vector3::new(0., -0.25, 0.))?;
            }
            245 => {
                let (w, h, _) = viewport_css();
                if let Some(tb) = &mut self.trackball {
                    tb.screen = Vector2::new(w, h);
                    for _ in 0..steps {
                        tb.update(s)?;
                    }
                }
            }
            246 => {
                // setTimeout( updateTweens, 5000 ) timers, then TWEEN.update( now ).
                let now = t * 1000.;
                while self.next_tweens <= now {
                    let at = self.next_tweens;
                    self.start_tweens(at);
                    self.next_tweens += 5000.;
                }
                let mut i = 0;
                while i < self.tweens.len() {
                    let tw = &self.tweens[i];
                    if now < tw.start {
                        i += 1;
                        continue;
                    }
                    let elapsed = now - tw.start;
                    let portion = (elapsed / tw.duration).min(1.);
                    // Easing.Quadratic.Out
                    let k = portion * (2. - portion);
                    let v = [0, 1, 2].map(|j| tw.from[j] + (tw.to[j] - tw.from[j]) * k);
                    let spot = &mut self.spots[tw.light];
                    if tw.position {
                        spot.position = Vector3::from_array(v);
                    } else {
                        spot.angle = v[0];
                        spot.penumbra = v[1];
                    }
                    if elapsed >= tw.duration {
                        self.tweens.remove(i);
                    } else {
                        i += 1;
                    }
                }
                self.update_spots(s)?;
            }
            _ => {
                for (i, hs) in [
                    self.hemisphere.map(|h| h.to_vec()),
                    self.directional.map(|h| h.to_vec()),
                ]
                .into_iter()
                .enumerate()
                {
                    for h in hs.unwrap_or_default() {
                        s.get_mut(h)?.visible = self.visible[i];
                    }
                }
                if let Some(mixer) = &mut self.mixer {
                    mixer.actions[0].time = t * mixer.actions[0].time_scale;
                    mixer.update(s, 0.)?;
                }
            }
        }
        Ok(())
    }
    /// Absolute CSS-pixel pointer events for TrackballControls.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(t) = &mut self.trackball {
            match kind {
                10..=19 => t.down(kind - 10, x, y),
                20..=29 => t.state = Mode::None,
                _ => t.moved(x, y),
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if let Some(t) = &mut self.trackball {
            if wheel != 0. {
                t.zoom_start.y -= wheel * 0.00025;
            }
            return Ok(());
        }
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            // The cube-map example disables zoom.
            if self.id == 243 {
                return Ok(());
            }
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            if self.id == 243 {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(t) = &mut self.trackball {
            if !down {
                t.key_state = Mode::None;
            } else if t.key_state == Mode::None {
                t.key_state = match code {
                    65 => Mode::Rotate,
                    83 => Mode::Zoom,
                    68 => Mode::Pan,
                    _ => Mode::None,
                };
            }
        }
    }
    /// toggleHemisphereLight and toggleDirectionalLight, applied in prepare().
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (247, 0 | 1) => self.visible[index] = value > 0.5,
            _ => return Err(Error::Invalid("shapes/lights parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
