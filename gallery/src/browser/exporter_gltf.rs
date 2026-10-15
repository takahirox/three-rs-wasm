//! misc_exporter_gltf: the page's export test scene under ACES Filmic tone
//! mapping, circled by the camera on the clock: basic, wireframe and
//! translucent polyhedra, a flat-shaded metal sphere with the gradient
//! DataTexture and the uv grid as its bump map, standard and Lambert
//! primitives, the textured box hierarchy and nested groups, a line strip,
//! a line loop and random points, the hidden box, the quantized ShaderBall,
//! fifty HSL-coloured instanced boxes, the meshopt / KTX2 coffeemat and the
//! canvas-drawn box, with the grid and axes helpers, an ambient and a
//! directional light. The page's GLTFExporter buttons are not ported.
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use super::interactive_scenes::grid_helper;
use crate::attribute::BufferAttribute;
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn positions(array: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(array, 3, false)?))
}
fn mesh(
    s: &mut Scene,
    geometry: BufferGeometry,
    material: &Arc<Material>,
    position: [f64; 3],
) -> Object3D {
    let h = s.insert(NodeKind::Mesh(Mesh {
        geometry: Arc::new(geometry),
        materials: vec![material.clone()],
    }));
    if let Ok(n) = s.get_mut(h) {
        n.position = Vector3::from_array(position);
    }
    h
}
fn standard(f: impl FnOnce(&mut MeshStandardMaterial)) -> Arc<Material> {
    let mut m = MeshStandardMaterial {
        energy_conservation: true,
        ..Default::default()
    };
    f(&mut m);
    Arc::new(Material::Standard(m))
}

pub(super) struct Demo {
    time: f64,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, _r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 2000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(600., 400., 0.);
        s.tone_mapping = ToneMapping::Aces;
        let mut seed = 186;
        // The linear gradient DataTexture: red up the rows, green down.
        let mut data = vec![0u8; 100 * 100 * 4];
        for y in 0..100 {
            for x in 0..100 {
                let i = 4 * (100 * y + x);
                data[i] = (255. * y as f64 / 99.).round() as u8;
                data[i + 1] = (255. - 255. * y as f64 / 99.).round() as u8;
                data[i + 3] = 255;
            }
        }
        let mut gradient = Texture::from_rgba(100, 100, data, false)?;
        gradient.flip_y = false;
        gradient.filter = Filter::Linear;
        gradient.min_filter = Some(Filter::Linear);
        gradient.mipmap_filter = None;
        let gradient = Arc::new(gradient);
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xcccccc),
            intensity: 1.,
        }));
        // DirectionalLight at its default ( 0, 1, 0 ), turned by lookAt( -1, -1, 0 )
        // with its target a child one unit ahead: light along ( −1, −2, 0 ).
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(1., 2., 0.).normalize();
        let mut grid = grid_helper(2000., 20, 0xc1c1c1, 0x8d8d8d)?;
        Arc::make_mut(&mut grid.material)
            .properties_mut()
            .tone_mapped = false;
        let grid = s.insert(NodeKind::Line(grid));
        s.get_mut(grid)?.position.y = -50.;
        // AxesHelper( 500 ): raw vertex colours, not tone mapped.
        let mut axes = BufferGeometry::default();
        axes.set_attribute(
            "position",
            positions(vec![
                0., 0., 0., 500., 0., 0., 0., 0., 0., 0., 500., 0., 0., 0., 0., 0., 0., 500.,
            ])?,
        );
        axes.set_attribute(
            "color",
            positions(vec![
                1., 0., 0., 1., 0.6, 0., 0., 1., 0., 0.6, 1., 0., 0., 0., 1., 0., 0.6, 1.,
            ])?,
        );
        let mut axes_material = LineBasicMaterial::default();
        axes_material.properties.vertex_colors = true;
        axes_material.properties.tone_mapped = false;
        s.insert(NodeKind::Line(Line {
            geometry: Arc::new(axes),
            material: Arc::new(Material::Line(axes_material)),
            segments: true,
        }));
        let mut grid_map = decode_texture_image(
            &fetch("/web/gallery/assets/environment-materials/textures/uv_grid_opengl.jpg").await?,
        )
        .await?;
        grid_map.srgb = true;
        grid_map.wrap_s = Wrapping::Repeat;
        grid_map.wrap_t = Wrapping::Repeat;
        grid_map.mipmap_filter = Some(Filter::Linear);
        let grid_map = Arc::new(grid_map);
        let mut basic = MeshBasicMaterial::default();
        basic.properties.map = Some(grid_map.clone());
        mesh(
            s,
            IcosahedronGeometry::build(75., 0)?,
            &Arc::new(Material::Basic(basic)),
            [-200., 0., 200.],
        );
        let mut wire = MeshBasicMaterial::default();
        wire.properties.color = Color::from_hex(0x0000ff);
        wire.properties.wireframe = true;
        mesh(
            s,
            OctahedronGeometry::build(75., 1)?,
            &Arc::new(Material::Basic(wire)),
            [0., 0., 200.],
        );
        let mut translucent = MeshBasicMaterial::default();
        translucent.properties.color = Color::from_hex(0xff0000);
        translucent.properties.transparent = true;
        translucent.properties.opacity = 0.5;
        mesh(
            s,
            TetrahedronGeometry::build(75., 0)?,
            &Arc::new(Material::Basic(translucent)),
            [200., 0., 200.],
        );
        let sphere = standard(|m| {
            m.properties.color = Color::from_hex(0xffff00);
            m.metalness = 0.5;
            m.roughness = 1.;
            m.properties.flat_shading = true;
            m.properties.map = Some(gradient.clone());
            m.bump_map = Some(grid_map.clone());
        });
        mesh(
            s,
            SphereGeometry::build(70., 10, 10)?,
            &sphere,
            [0., 0., 0.],
        );
        let cylinder = standard(|m| {
            m.properties.color = Color::from_hex(0xff00ff);
            m.properties.flat_shading = true;
        });
        mesh(
            s,
            CylinderGeometry::build(10., 80., 100., 32, 1, false, 0., TAU)?,
            &cylinder,
            [200., 0., 0.],
        );
        let knot = standard(|m| m.properties.color = Color::from_hex(0xff0000));
        mesh(
            s,
            TorusKnotGeometry::build(50., 15., 40, 10, 2, 3)?,
            &knot,
            [-200., 0., 0.],
        );
        // The hierarchy: hardwood2_diffuse without a colour space.
        let mut wood =
            decode_texture_image(&fetch("/web/gallery/assets/hardwood2_diffuse.jpg").await?)
                .await?;
        wood.srgb = false;
        wood.mipmap_filter = Some(Filter::Linear);
        let wood = Arc::new(wood);
        let wood = standard(|m| {
            m.properties.map = Some(wood.clone());
            m.properties.side = Side::Double;
        });
        let cube = mesh(
            s,
            BoxGeometry::build(40., 100., 100.)?,
            &wood,
            [-200., 0., 400.],
        );
        let sub = mesh(
            s,
            BoxGeometry::segmented(40., 40., 40., 2, 2, 2)?,
            &wood,
            [0., 0., 50.],
        );
        s.get_mut(sub)?.quaternion = Quaternion::from_rotation_y(45.);
        s.add(cube, sub)?;
        let group = s.insert(NodeKind::Group);
        let sub_group = s.insert(NodeKind::Group);
        s.get_mut(sub_group)?.position = Vector3::new(0., 50., 0.);
        s.add(group, sub_group)?;
        let boxed = mesh(s, BoxGeometry::build(30., 30., 30.)?, &wood, [0., 0., 400.]);
        s.add(sub_group, boxed)?;
        // The line strip and the line loop ( closed by its first point ).
        let yellow = {
            let mut m = LineBasicMaterial::default();
            m.properties.color = Color::from_hex(0xffff00);
            Arc::new(Material::Line(m))
        };
        let strip: Vec<f32> = (0..100)
            .flat_map(|i| [i as f32, ((i as f64 / 2.).sin() * 20.) as f32, 0.])
            .collect();
        let mut loop_points: Vec<f32> = (0..5)
            .flat_map(|i| {
                let a = i as f64 * PI * 2. / 5.;
                [(70. * a.sin()) as f32, (70. * a.cos()) as f32, 0.]
            })
            .collect();
        loop_points.extend_from_within(..3);
        for (points, position) in [(strip, [-50., 0., -200.]), (loop_points, [0., 0., -200.])] {
            let mut g = BufferGeometry::default();
            g.set_attribute("position", positions(points)?);
            let h = s.insert(NodeKind::Line(Line {
                geometry: Arc::new(g),
                material: yellow.clone(),
                segments: false,
            }));
            s.get_mut(h)?.position = Vector3::from_array(position);
        }
        let points: Vec<f32> = (0..100)
            .flat_map(|_| {
                let x = -50. + random(&mut seed) * 100.;
                let y = random(&mut seed) * 100.;
                let z = -50. + random(&mut seed) * 100.;
                [x as f32, y as f32, z as f32]
            })
            .collect();
        let mut g = BufferGeometry::default();
        g.set_attribute("position", positions(points)?);
        let mut points_material = PointsMaterial::default();
        points_material.properties.color = Color::from_hex(0xffff00);
        points_material.size = 5.;
        points_material.size_attenuation = true;
        let cloud = s.insert(NodeKind::Points(Points {
            geometry: Arc::new(g),
            material: Arc::new(Material::Points(points_material)),
        }));
        s.get_mut(cloud)?.position = Vector3::new(-200., 0., -200.);
        let lambert = {
            let mut m = MeshLambertMaterial::default();
            m.properties.color = Color::from_hex(0xffff00);
            m.properties.side = Side::Double;
            Arc::new(Material::Lambert(m))
        };
        mesh(
            s,
            CircleGeometry::build(50., 20, 0., TAU)?,
            &lambert,
            [200., 0., -400.],
        );
        mesh(
            s,
            RingGeometry::build(10., 50., 20, 5, 0., TAU)?,
            &lambert,
            [0., 0., -400.],
        );
        mesh(
            s,
            CylinderGeometry::build(25., 75., 100., 40, 5, false, 0., TAU)?,
            &lambert,
            [-200., 0., -400.],
        );
        let profile: Vec<Vector2> = (0..50)
            .map(|i| {
                let i = f64::from(i);
                Vector2::new((i * 0.2).sin() * (i * 0.1).sin() * 15. + 50., (i - 5.) * 2.)
            })
            .collect();
        mesh(
            s,
            LatheGeometry::build(&profile, 20, 0., TAU)?,
            &lambert,
            [200., 0., 400.],
        );
        // CubeHidden: invisible.
        let mut red = MeshBasicMaterial::default();
        red.properties.color = Color::from_hex(0xff0000);
        let hidden = mesh(
            s,
            BoxGeometry::build(200., 200., 200.)?,
            &Arc::new(Material::Basic(red)),
            [0., 0., 0.],
        );
        s.get_mut(hidden)?.visible = false;
        // ShaderBall ( KHR_mesh_quantization ).
        let (a, b, i) = load_asset("/web/gallery/assets/gltf/ShaderBall.glb").await?;
        let model = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let ball = s.insert(NodeKind::Group);
        for h in model.roots {
            s.add(ball, h)?;
        }
        let n = s.get_mut(ball)?;
        n.scale = Vector3::splat(50.);
        n.position = Vector3::new(200., -40., -200.);
        // The instanced boxes: setPosition only, HSL colours.
        let white = Arc::new(Material::Basic(MeshBasicMaterial::default()));
        let instanced = mesh(
            s,
            BoxGeometry::segmented(10., 10., 10., 2, 2, 2)?,
            &white,
            [400., 0., 200.],
        );
        let instances = (0..50)
            .map(|i| {
                let x = random(&mut seed) * 100. - 50.;
                let y = random(&mut seed) * 100. - 50.;
                let z = random(&mut seed) * 100. - 50.;
                Instance {
                    matrix: Matrix4::from_translation(Vector3::new(x, y, z)),
                    color: Color::from_hsl(f64::from(i) / 50., 1., 0.5),
                }
            })
            .collect();
        s.get_mut(instanced)?.instances = instances;
        // coffeemat ( KTX2 textures, meshopt geometry ).
        let (a, b, i) = load_asset("/web/gallery/assets/gltf/coffeemat.glb").await?;
        let coffeemat = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let coffee = s.insert(NodeKind::Group);
        for h in coffeemat.roots {
            s.add(coffee, h)?;
        }
        s.get_mut(coffee)?.position = Vector3::new(400., 0., -200.);
        // The 64 × 64 canvas: #005BBB with a #FFD500 square from 16 to 48.
        let mut pixels = vec![0u8; 64 * 64 * 4];
        for y in 0..64 {
            for x in 0..64 {
                let inside = (16..48).contains(&x) && (16..48).contains(&y);
                let c = if inside {
                    [0xff, 0xd5, 0x00]
                } else {
                    [0x00, 0x5b, 0xbb]
                };
                pixels[(y * 64 + x) * 4..(y * 64 + x) * 4 + 4]
                    .copy_from_slice(&[c[0], c[1], c[2], 255]);
            }
        }
        let mut canvas = Texture::from_rgba(64, 64, pixels, true)?;
        canvas.flip_y = true;
        canvas.mipmap_filter = Some(Filter::Linear);
        let mut webp = MeshBasicMaterial::default();
        webp.properties.map = Some(Arc::new(canvas));
        mesh(
            s,
            BoxGeometry::build(100., 100., 100.)?,
            &Arc::new(Material::Basic(webp)),
            [400., 0., 0.],
        );
        Ok(Self { time: 0. })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the camera circles at radius 800 by Date.now() × 0.0001.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let t = self.time * 1000. * 0.0001;
        let n = s.get_mut(c)?;
        n.position.x = t.cos() * 800.;
        n.position.z = t.sin() * 800.;
        s.look_at(c, Vector3::ZERO)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
