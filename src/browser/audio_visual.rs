//! webaudio_visualizer and webaudio_orientation. The visualizer draws the
//! analyser's 64 frequency bins, uploaded each frame as a red DataTexture, as
//! a line in a raw ShaderMaterial. The orientation scene places the
//! BoomBox's directional PositionalAudio, its PositionalAudioHelper cone and
//! the damping wall. The sandbox places three positional sources and turns
//! their analysers' average frequency into emissive blue, under
//! FirstPersonControls. The page runs the Web Audio graph (web/gallery/audio.js).
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::environment_materials::cube;
use super::gltf_viewer::load_asset;
use super::interactive_scenes::grid_helper;
use super::trackball_sprites::FirstPerson;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const BINS: u32 = 64;
/// PositionalAudioHelper( audio, 0.1 ): the directional cone's three line
/// strips (outer, inner, outer), as its geometry groups draw them.
fn cone_segments(inner: f64, outer: f64, range: f64) -> Vec<(Vec<f32>, u32)> {
    let (inner, outer) = (inner.to_radians() / 2., outer.to_radians() / 2.);
    let segment = |from: f64, to: f64, divisions: f64| {
        let step = (to - from) / divisions;
        let mut v = vec![0f32; 3];
        let mut i = from;
        while i < to {
            let next = (i + step).min(to);
            v.extend([(i.sin() * range) as f32, 0., (i.cos() * range) as f32]);
            v.extend([(next.sin() * range) as f32, 0., (next.cos() * range) as f32]);
            v.extend([0., 0., 0.]);
            i += step;
        }
        v
    };
    vec![
        (segment(-outer, -inner, 2.), 0xffff00),
        (segment(-inner, inner, 16.), 0x00ff00),
        (segment(inner, outer, 2.), 0xffff00),
    ]
}
struct Visualizer {
    texture: wgpu::Texture,
    data: Option<Vec<u8>>,
}
/// webaudio_sandbox: the sound spheres, their materials and the GUI's audio
/// settings (master, the three spheres, ambient, frequency, wave type).
struct Sandbox {
    spheres: [Object3D; 3],
    averages: [f64; 3],
    settings: [f64; 7],
    walker: FirstPerson,
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    sandbox: Option<Sandbox>,
    root: Object3D,
    started: bool,
    start_requested: bool,
    visualizer: Option<Visualizer>,
    /// The orientation scene's audio source (the BoomBox's child).
    source: Option<Object3D>,
    controls: Option<Controls>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let root = s.insert(NodeKind::Group);
        let mut d = Self {
            id,
            time: 0.,
            last: 0.,
            sandbox: None,
            root,
            started: false,
            start_requested: false,
            visualizer: None,
            source: None,
            controls: None,
        };
        s.background = Color::BLACK;
        if id == 328 {
            d.visualizer_scene(s, r).await?;
        } else if id == 330 {
            d.sandbox_scene(s, c)?;
        } else {
            d.orientation_scene(s, c, r).await?;
        }
        // Nothing is shown until the Play button runs init().
        s.get_mut(root)?.visible = false;
        Ok(d)
    }
    async fn visualizer_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        // DataTexture( analyser.data, 64, 1, RedFormat ): nearest filtering.
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("audio frequency data"),
            size: wgpu::Extent3d {
                width: BINS,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        let sampler = r.device.create_sampler(&Default::default());
        let color = WgslFn::new(
            "spectrum",
            "fn spectrum()->vec4<f32>{let uv=fragment_surface.uv;let f=textureSample(tsl_texture_0,tsl_sampler_0,vec2(uv.x,0.0)).r;let i=step(uv.y,f)*step(f-0.0125,uv.y);return vec4(mix(vec3(0.125),vec3(1.0,1.0,0.0),i),1.0);}",
            &[],
            Type::Vec4,
        )?
        .call(&[]);
        // `gl_Position = vec4( position, 1.0 )`: no camera transform.
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(1)?,
            &[],
            &[(&view, &sampler)],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(position,1.0);return out;}",
        )
        .await?;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
            Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
        )));
        s.get_mut(mesh)?.frustum_culled = false;
        s.add(self.root, mesh)?;
        self.visualizer = Some(Visualizer {
            texture,
            data: Some(vec![0; BINS as usize]),
        });
        Ok(())
    }
    async fn orientation_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(3., 2., 3.);
        n.quaternion = Quaternion::IDENTITY;
        let root = self.root;
        let add = |s: &mut Scene, kind: NodeKind| -> Result<Object3D> {
            let h = s.insert(kind);
            s.add(root, h)?;
            Ok(h)
        };
        let hemi = add(
            s,
            NodeKind::Light(Light::Hemisphere {
                sky: Color::WHITE,
                ground: Color::from_hex(0x8d8d8d),
                intensity: 3.,
            }),
        )?;
        s.get_mut(hemi)?.position = Vector3::new(0., 20., 0.);
        let light = add(
            s,
            NodeKind::Light(Light::Directional {
                color: Color::WHITE,
                intensity: 3.,
                target: Vector3::ZERO,
            }),
        )?;
        let n = s.get_mut(light)?;
        n.position = Vector3::new(5., 5., 0.);
        n.cast_shadow = true;
        n.shadow.extent = 1.;
        n.shadow.near = 0.1;
        n.shadow.far = 20.;
        let mut phong = MeshPhongMaterial::default();
        phong.properties.color = Color::from_hex(0xcbcbcb);
        phong.properties.depth_write = false;
        let ground = add(
            s,
            NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(50., 50., 1, 1)?),
                Arc::new(Material::Phong(phong)),
            )),
        )?;
        let n = s.get_mut(ground)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.receive_shadow = true;
        add(s, NodeKind::Line(grid_helper(50., 50, 0xc1c1c1, 0xc1c1c1)?))?;
        // The BoomBox: each mesh's geometry turned by −π about y, the Swedish
        // Royal Castle cube as its environment map.
        let (a, b, images) = load_asset("/web/gallery/assets/webaudio/BoomBox.glb").await?;
        let mut temp = Scene::new();
        let meshes = crate::gltf::import_decoded(&a, &b, &images)?.instantiate(&mut temp)?;
        let boombox = add(s, NodeKind::Group)?;
        let n = s.get_mut(boombox)?;
        n.position = Vector3::new(0., 0.2, 0.);
        n.scale = Vector3::splat(20.);
        for h in meshes {
            let node = temp.get(h)?;
            let (matrix, kind) = (node.matrix, node.kind.clone());
            let NodeKind::Mesh(mut mesh) = kind else {
                continue;
            };
            let mut g = (*mesh.geometry).clone();
            g.apply_quaternion(Quaternion::from_rotation_y(-PI))?;
            mesh.geometry = Arc::new(g);
            let m = s.insert(NodeKind::Mesh(mesh));
            let n = s.get_mut(m)?;
            n.matrix = matrix;
            n.matrix_auto_update = false;
            n.cast_shadow = true;
            s.add(boombox, m)?;
        }
        let cubemap = cube(r, "SwedishRoyalCastle").await?;
        s.environment = Some(Arc::new(
            crate::environment::EnvironmentMap::from_cube_texture(r, &cubemap)?,
        ));
        let source = s.insert(NodeKind::Group);
        s.add(boombox, source)?;
        // setDirectionalCone( 180, 230, 0.1 ) and the helper at range 0.1.
        for (points, hex) in cone_segments(180., 230., 0.1) {
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(points, 3, false)?),
            );
            let mut m = LineBasicMaterial::default();
            m.properties.color = Color::from_hex(hex);
            let line = s.insert(NodeKind::Line(Line {
                geometry: Arc::new(g),
                material: Arc::new(Material::Line(m)),
                segments: false,
            }));
            s.add(source, line)?;
        }
        let mut wall = MeshBasicMaterial::default();
        wall.properties.color = Color::from_hex(0xff0000);
        wall.properties.transparent = true;
        wall.properties.opacity = 0.5;
        let wall = add(
            s,
            NodeKind::Mesh(Mesh::new(
                Arc::new(BoxGeometry::build(2., 1., 0.1)?),
                Arc::new(Material::Basic(wall)),
            )),
        )?;
        s.get_mut(wall)?.position = Vector3::new(0., 0.5, -0.5);
        s.background = Color::from_hex(0xa0a0a0);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0xa0a0a0),
            near: 2.,
            far: 20.,
        });
        // OrbitControls: target (0, 0.1, 0), distances 0.5–10, maxPolarAngle π/2.
        let mut controls = Controls::new(None, (0.5, 10.), PI / 2., true);
        controls.set_target(Vector3::new(0., 0.1, 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.source = Some(source);
        Ok(())
    }
    fn sandbox_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 25., 0.);
        n.quaternion = Quaternion::IDENTITY;
        s.fog = Some(Fog::Exp2 {
            color: Color::BLACK,
            density: 0.0025,
        });
        let root = self.root;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0., 0.5, 1.).normalize();
        s.add(root, light)?;
        let sphere = Arc::new(SphereGeometry::build(20., 32, 16)?);
        let mut spheres = vec![];
        for (hex, position) in [
            (0xffaa00, Vector3::new(-250., 30., 0.)),
            (0xff2200, Vector3::new(250., 30., 0.)),
            (0x6622aa, Vector3::new(0., 30., -250.)),
        ] {
            let mut m = MeshPhongMaterial::default();
            m.properties.color = Color::from_hex(hex);
            m.properties.flat_shading = true;
            m.shininess = 0.;
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                sphere.clone(),
                Arc::new(Material::Phong(m)),
            )));
            s.get_mut(h)?.position = position;
            s.add(root, h)?;
            spheres.push(h);
        }
        let mut grid = grid_helper(1000., 10, 0x444444, 0x444444)?;
        grid.segments = true;
        let grid = s.insert(NodeKind::Line(grid));
        s.get_mut(grid)?.position.y = 0.1;
        s.add(root, grid)?;
        // FirstPersonControls: movementSpeed 70, lookVertical false, and the
        // orientation taken from the camera's initial quaternion.
        let mut walker = FirstPerson::default();
        walker.speed = 70.;
        walker.fixed_latitude = true;
        let look = s.get(c)?.quaternion * -Vector3::Z;
        walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
        walker.lon = look.x.atan2(look.z).to_degrees();
        self.sandbox = Some(Sandbox {
            spheres: [spheres[0], spheres[1], spheres[2]],
            averages: [0.; 3],
            // master, firstSphere, secondSphere, thirdSphere, Ambient,
            // frequency, wavetype (sine).
            settings: [1., 1., 1., 0.5, 0.5, 144., 0.],
            walker,
        });
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// The Play button's scene, and the analyser data uploaded each frame, as
    /// `needsUpdate` uploads it.
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.start_requested) && !self.started {
            self.started = true;
            s.get_mut(self.root)?.visible = true;
        }
        let delta = self.time - self.last;
        self.last = self.time;
        if let Some(k) = &mut self.sandbox
            && self.started
        {
            // controls.update( delta ); emissive.b = getAverageFrequency() / 256.
            k.walker.update(s, _c, delta)?;
            for i in 0..3 {
                let emissive = Color::linear(0., 0., k.averages[i] / 256.);
                if let NodeKind::Mesh(mesh) = &mut s.get_mut(k.spheres[i])?.kind
                    && let Material::Phong(m) = Arc::make_mut(&mut mesh.materials[0])
                {
                    m.emissive = emissive;
                }
            }
        }
        if let Some(v) = &mut self.visualizer
            && self.started
            && let Some(data) = &v.data
        {
            r.queue.write_texture(
                v.texture.as_image_copy(),
                data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(BINS),
                    rows_per_image: Some(1),
                },
                wgpu::Extent3d {
                    width: BINS,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            );
        }
        Ok(())
    }
    /// analyser.getFrequencyData(): the latest bins from the page.
    pub fn audio_data(&mut self, bytes: &[u8]) -> Result<()> {
        if let Some(k) = &mut self.sandbox {
            // Three analysers of 16 bins each (fftSize 32).
            if bytes.len() != 48 {
                return Err(Error::Invalid("analyser bins"));
            }
            for (i, bins) in bytes.chunks(16).enumerate() {
                k.averages[i] = bins.iter().map(|&b| f64::from(b)).sum::<f64>() / 16.;
            }
            return Ok(());
        }
        let v = self
            .visualizer
            .as_mut()
            .ok_or(Error::Invalid("not an analyser example"))?;
        if bytes.len() != BINS as usize {
            return Err(Error::Invalid("analyser bins"));
        }
        v.data = Some(bytes.to_vec());
        Ok(())
    }
    /// The listener (camera) and the source's world placement:
    /// [position, forward, up] + [position, orientation, 0].
    pub fn audio_frame(&mut self, s: &mut Scene, c: Object3D) -> Result<Vec<f32>> {
        if let Some(k) = &self.sandbox {
            // [listener] + per sphere [position, orientation, 0] + the settings.
            s.update_world_matrix(c, true, false)?;
            let (_, q, p) = s.get(c)?.matrix_world.to_scale_rotation_translation();
            let mut out = vec![];
            for v in [p, q * Vector3::NEG_Z, q * Vector3::Y] {
                out.extend([v.x as f32, v.y as f32, v.z as f32]);
            }
            for &h in &k.spheres {
                s.update_world_matrix(h, true, false)?;
                let (_, q, p) = s.get(h)?.matrix_world.to_scale_rotation_translation();
                let o = q * Vector3::Z;
                out.extend([p.x, p.y, p.z, o.x, o.y, o.z, 0.].map(|v| v as f32));
            }
            out.extend(k.settings.map(|v| v as f32));
            return Ok(out);
        }
        let source = self
            .source
            .ok_or(Error::Invalid("not a positional audio example"))?;
        s.update_world_matrix(c, true, false)?;
        s.update_world_matrix(source, true, false)?;
        let (_, q, p) = s.get(c)?.matrix_world.to_scale_rotation_translation();
        let mut out = vec![];
        for v in [p, q * Vector3::NEG_Z, q * Vector3::Y] {
            out.extend([v.x as f32, v.y as f32, v.z as f32]);
        }
        let (_, q, p) = s.get(source)?.matrix_world.to_scale_rotation_translation();
        let o = q * Vector3::Z;
        out.extend([p.x, p.y, p.z, o.x, o.y, o.z, 0.].map(|v| v as f32));
        Ok(out)
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(k) = &mut self.sandbox {
            k.walker.pointer(kind, x, y);
        }
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(k) = &mut self.sandbox {
            k.walker.key(code, down);
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
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    /// The Play button (0, or 7 after the sandbox's GUI settings 0–6, which
    /// the page reads each frame), applied at the next frame.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index, &mut self.sandbox) {
            (330, 0..=6, Some(k)) => k.settings[index] = value as f64,
            (330, 7, _) | (328 | 329, 0, _) => self.start_requested = true,
            _ => return Err(Error::Invalid("audio parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
