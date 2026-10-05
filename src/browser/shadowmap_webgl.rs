//! webgl_shadowmap: six galloping horses, a flamingo, a stork and a parrot
//! ( the glTF morph animations, each mesh with its own action phase ) running
//! along x past the extruded "THREE.JS" text and two blocks, under an
//! ambient light and a directional light's 2048 × 1024 PCF shadow map, in
//! linear fog, with OrbitControls. The T key toggles the ShadowMapViewer HUD
//! of the light's shadow map. The morph weights are sampled from the clips'
//! linear keyframes on the CPU, as AnimationMixer samples them.
use super::controls_attributes::{Controls, camera_state, viewport_css};
use super::gltf_viewer::{fetch, load_asset};
use super::shadowmap_performance::offset_hsl;
use super::text_clipping::text_geometry;
use super::text_shapes::{Extrude, Font};
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const FLOOR: f64 = -250.;
const MAP: (u32, u32) = (2048, 1024);
/// The fixture's Math.random: a 32-bit LCG seeded with 186.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(self.0) / 4294967296.
    }
}
/// A glTF morph clip: its keyframe times and weights.
struct Clip {
    times: Vec<f64>,
    values: Vec<Vec<f64>>,
    duration: f64,
}
impl Clip {
    /// LinearInterpolant over the keyframes.
    fn sample(&self, t: f64) -> Vec<f64> {
        let n = self.times.len();
        if t <= self.times[0] {
            return self.values[0].clone();
        }
        if t >= self.times[n - 1] {
            return self.values[n - 1].clone();
        }
        let k = self.times.partition_point(|&x| x <= t).max(1);
        let (t0, t1) = (self.times[k - 1], self.times[k]);
        let a = (t - t0) / (t1 - t0);
        self.values[k - 1]
            .iter()
            .zip(&self.values[k])
            .map(|(v0, v1)| v0 * (1. - a) + v1 * a)
            .collect()
    }
}
/// A running morph: its node, clip, speed, duration and start.
struct Morph {
    node: Object3D,
    clip: usize,
    speed: f64,
    duration: f64,
    start: f64,
}
pub(super) struct Demo {
    controls: Controls,
    random: Random,
    clips: Vec<Clip>,
    morphs: Vec<Morph>,
    time: f64,
    last: f64,
    show_hud: bool,
    hud: Scene,
    hud_camera: Object3D,
    hud_quad: Object3D,
    /// The canvas height when the HUD was created ( SCREEN_HEIGHT ).
    screen_height: f64,
    output: Option<RenderTarget>,
}
/// A glTF model's first mesh ( geometry, material ) and its morph clip.
async fn model(name: &str) -> Result<(Arc<BufferGeometry>, Arc<Material>, Clip)> {
    let (a, b, i) = load_asset(&format!("/web/gallery/assets/gltf/{name}.glb")).await?;
    let mut scratch = Scene::new();
    let template = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(&mut scratch)?;
    let first = *template
        .meshes
        .first()
        .ok_or(Error::Invalid("morph mesh"))?;
    let (geometry, material) = match &scratch.get(first)?.kind {
        NodeKind::Mesh(m) => (m.geometry.clone(), m.materials[0].clone()),
        _ => return Err(Error::Invalid("morph mesh")),
    };
    let animation = a.animations().next().ok_or(Error::Invalid("morph clip"))?;
    let channel = animation
        .channels()
        .next()
        .ok_or(Error::Invalid("morph track"))?;
    let reader = channel.reader(|buffer| b.get(buffer.index()).map(Vec::as_slice));
    let times: Vec<f64> = reader
        .read_inputs()
        .ok_or(Error::Invalid("morph times"))?
        .map(f64::from)
        .collect();
    let flat: Vec<f64> = match reader.read_outputs() {
        Some(gltf::animation::util::ReadOutputs::MorphTargetWeights(w)) => {
            w.into_f32().map(f64::from).collect()
        }
        _ => return Err(Error::Invalid("morph weights")),
    };
    let targets = flat.len() / times.len();
    let values = flat.chunks(targets).map(<[f64]>::to_vec).collect();
    let duration = times.last().copied().unwrap_or(1.);
    Ok((
        geometry,
        material,
        Clip {
            times,
            values,
            duration,
        },
    ))
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 23.,
            near: 10.,
            far: 3000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(700., 50., 1900.);
        s.background = Color::from_hex(0x59472b);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x59472b),
            near: 1000.,
            far: 3000.,
        });
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(0., 1500., 1000.);
        n.cast_shadow = true;
        n.shadow.extent = 2000.;
        n.shadow.near = 1200.;
        n.shadow.far = 2500.;
        // reversedDepthBuffer: the bias is added to the reversed depth, so it
        // moves the compared depth toward the light.
        n.shadow.bias = -0.0001;
        n.shadow.reversed_depth = true;
        n.shadow.map_size = Some(MAP.0);
        n.shadow.map_height = Some(MAP.1);
        // The ground and the two blocks share the plane's Phong material.
        let plane_material = Arc::new(Material::Phong(MeshPhongMaterial {
            properties: MaterialProperties {
                color: Color::from_hex(0xffdd99),
                ..Default::default()
            },
            ..Default::default()
        }));
        let mesh =
            |s: &mut Scene, g: BufferGeometry, m: Arc<Material>, cast: bool| -> Result<Object3D> {
                let h = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(g), m)));
                let n = s.get_mut(h)?;
                n.cast_shadow = cast;
                n.receive_shadow = true;
                Ok(h)
            };
        let ground = mesh(
            s,
            PlaneGeometry::build(100., 100., 1, 1)?,
            plane_material.clone(),
            false,
        )?;
        let n = s.get_mut(ground)?;
        n.position = Vector3::new(0., FLOOR, 0.);
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.scale = Vector3::splat(100.);
        // The text, centered by its bounding box.
        let font =
            Font::parse(&fetch("/web/gallery/assets/fonts/helvetiker_bold.typeface.json").await?)?;
        let text = text_geometry(
            &font,
            "THREE.JS",
            200.,
            &Extrude {
                curve_segments: 12,
                steps: 1,
                depth: 50.,
                bevel: Some((2., 5., 3)),
            },
        )?;
        let (min_x, max_x) = match text.attributes.get("position") {
            Some(Attribute::F32(a)) => a
                .array()
                .chunks(3)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
                    (lo.min(f64::from(p[0])), hi.max(f64::from(p[0])))
                }),
            _ => return Err(Error::Invalid("text positions")),
        };
        let text_material = Arc::new(Material::Phong(MeshPhongMaterial {
            properties: MaterialProperties {
                color: Color::from_hex(0xff0000),
                ..Default::default()
            },
            specular: Color::from_hex(0xffffff),
            ..Default::default()
        }));
        let text_mesh = mesh(s, text, text_material, true)?;
        s.get_mut(text_mesh)?.position = Vector3::new(-0.5 * (max_x - min_x), FLOOR + 67., 0.);
        for (w, h, d) in [(1500., 220., 150.), (1600., 170., 250.)] {
            let block = mesh(
                s,
                BoxGeometry::build(w, h, d)?,
                plane_material.clone(),
                true,
            )?;
            s.get_mut(block)?.position = Vector3::new(0., FLOOR - 50., 20.);
        }
        // The morphs, in the page's load order ( the fixture loads them in
        // sequence ): addMorph( mesh, clip, speed, duration, x, y, z, fudge ).
        let mut random = Random(186);
        let mut clips = vec![];
        let mut morphs = vec![];
        for (name, entries) in [
            (
                "Horse",
                vec![
                    (550., 1., FLOOR, 300., true),
                    (550., 1., FLOOR, 450., true),
                    (550., 1., FLOOR, 600., true),
                    (550., 1., FLOOR, -300., true),
                    (550., 1., FLOOR, -450., true),
                    (550., 1., FLOOR, -600., true),
                ],
            ),
            ("Flamingo", vec![(500., 1., FLOOR + 350., 40., false)]),
            ("Stork", vec![(350., 1., FLOOR + 350., 340., false)]),
            ("Parrot", vec![(450., 0.5, FLOOR + 300., 700., false)]),
        ] {
            let (geometry, material, clip) = model(name).await?;
            let targets = clip.values[0].len();
            for (speed, duration, y, z, fudge) in entries {
                let x = if name == "Horse" {
                    100. - random.next() * 1000.
                } else {
                    500. - random.next() * 500.
                };
                let mut cloned = material.as_ref().clone();
                if fudge {
                    let (ds, dl) = (random.next() * 0.5 - 0.25, random.next() * 0.5 - 0.25);
                    let p = cloned.properties_mut();
                    p.color = offset_hsl(p.color, ds, dl);
                }
                if let Material::Standard(m) = &mut cloned {
                    // r186's WebGL physical shading: the DFG LUT's multiple scattering.
                    m.energy_conservation = true;
                }
                let start = -duration * random.next();
                let node = s.insert(NodeKind::Mesh(Mesh::new(
                    geometry.clone(),
                    Arc::new(cloned),
                )));
                let n = s.get_mut(node)?;
                n.position = Vector3::new(x, y, z);
                n.quaternion = Quaternion::from_rotation_y(PI / 2.);
                n.cast_shadow = true;
                n.receive_shadow = true;
                n.morph_weights = vec![0.; targets];
                morphs.push(Morph {
                    node,
                    clip: clips.len(),
                    speed,
                    duration,
                    start,
                });
            }
            clips.push(clip);
        }
        let mut controls = Controls::new(None, (200., 2200.), PI / 2., true);
        controls.set_target(Vector3::new(0., -75., 25.));
        controls.update(s, c)?;
        // ShadowMapViewer: the light's shadow map at 512 × 256, its depth
        // shown as the 8-bit depth material color ( black where cleared ).
        let probe = RenderTarget::new(&r.device, 1, 1)?;
        r.render(s, c, &probe)?;
        let atlas = r.shadow_atlas().ok_or(Error::Invalid("shadow atlas"))?;
        let view = atlas.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::DepthOnly,
            base_array_layer: 0,
            array_layer_count: Some(1),
            ..Default::default()
        });
        let nearest = r.device.create_sampler(&wgpu::SamplerDescriptor::default());
        let color = WgslFn::new(
            "shadow_hud",
            &format!("fn shadow_hud()->vec4<f32>{{let size=vec2<f32>({:.1},{:.1});let uv=fragment_surface.uv;let p=vec2(uv.x,1.0-uv.y)*size-0.5;let i=floor(p);let f=p-i;var s=array<f32,4>();for(var k=0;k<4;k++){{let o=vec2(f32(k%2),f32(k/2));let c=vec2<i32>(clamp(i+o,vec2(0.0),size-1.0));let d=textureLoad(tsl_texture_0,c,0).r;s[k]=select(round(d*255.0)/255.0,0.0,d>=1.0);}}let v=vec3(mix(mix(s[0],s[1],f.x),mix(s[2],s[3],f.x),f.y));return vec4(select(v,srgb_input(v),ENCODE_SRGB),1.0);}}", MAP.0 as f64, MAP.1 as f64),
            &[],
            Type::Vec4,
        )?
        .call(&[]);
        let program = ShaderProgram::with_projection_and_sample_types(
            r,
            &NodeMaterial::new(color).wgsl_with_texture_types(&[Type::Texture], &[])?,
            &[(&view, &nearest)],
            &[wgpu::TextureViewDimension::D2],
            &[wgpu::TextureSampleType::Float { filterable: false }],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.properties.tone_mapped = false;
        let mut hud = Scene::new();
        let (w, h, _) = viewport_css();
        let hud_camera = hud.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -w / 2.,
            right: w / 2.,
            top: h / 2.,
            bottom: -h / 2.,
            near: 1.,
            far: 10.,
            zoom: 1.,
            ..Default::default()
        })));
        hud.get_mut(hud_camera)?.position.z = 2.;
        let hud_quad = hud.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(256., 256., 1, 1)?),
            Arc::new(Material::Shader(m)),
        )));
        let mut d = Self {
            controls,
            random,
            clips,
            morphs,
            time: 0.,
            last: 0.,
            show_hud: false,
            hud,
            hud_camera,
            hud_quad,
            screen_height: h,
            output: None,
        };
        d.layout()?;
        Ok(d)
    }
    /// ShadowMapViewer's position ( 10, SCREEN_HEIGHT − 256 − 10 ) and size
    /// ( 512, 256 ), in the HUD's CSS-pixel frustum.
    fn layout(&mut self) -> Result<()> {
        let (w, h, _) = viewport_css();
        if let NodeKind::Camera(Camera::Orthographic(o)) =
            &mut self.hud.get_mut(self.hud_camera)?.kind
        {
            o.left = -w / 2.;
            o.right = w / 2.;
            o.top = h / 2.;
            o.bottom = -h / 2.;
        }
        let (size_w, size_h) = (MAP.0 as f64 / 4., MAP.1 as f64 / 4.);
        let y = self.screen_height - size_h - 10.;
        let n = self.hud.get_mut(self.hud_quad)?;
        n.scale = Vector3::new(size_w / 256., size_h / 256., 1.);
        n.position = Vector3::new(-w / 2. + size_w / 2. + 10., h / 2. - size_h / 2. - y, 0.);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// render(): the mixer, the morphs' runs, then the controls.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let delta = self.time - self.last;
        self.last = self.time;
        // setDuration( d ): timeScale = clip duration / d; startAt( start ).
        // An update with a zero delta has no time direction, so the scheduled
        // start waits for the mixer's first non-zero step.
        let started = self.time > 0.;
        for morph in &self.morphs {
            let clip = &self.clips[morph.clip];
            let local = if started {
                ((self.time - morph.start) * (clip.duration / morph.duration))
                    .rem_euclid(clip.duration)
            } else {
                0.
            };
            let weights = clip.sample(local);
            let n = s.get_mut(morph.node)?;
            n.morph_weights = weights;
            n.position.x += morph.speed * delta;
            if n.position.x > 2000. {
                n.position.x = -1000. - self.random.next() * 500.;
            }
        }
        self.controls.update(s, c)
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if !self.show_hud {
            r.render(s, c, out)?;
            return Ok(true);
        }
        if self.output.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.options.samples != out.options.samples
        }) {
            let mut options = out.options.clone();
            options.store_multisampled_color_buffer = true;
            self.output = Some(RenderTarget::with_options(
                &r.device, out.width, out.height, options,
            )?);
            self.layout()?;
        }
        let target = self
            .output
            .as_mut()
            .ok_or(Error::Invalid("shadowmap target"))?;
        target.set_load_color(false);
        r.render(s, c, target)?;
        // lightShadowMapViewer.render(): no clear but depth.
        target.set_load_color(true);
        r.render(&mut self.hud, self.hud_camera, target)?;
        target.set_load_color(false);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.show_hud.then_some(()).and(self.output.as_ref())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    /// keydown: T ( keyCode 84 ) toggles the HUD.
    pub fn key(&mut self, code: u32, down: bool) {
        if down && code == 84 {
            self.show_hud = !self.show_hud;
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
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if !pan {
            // enablePan is false.
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("shadowmap parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
