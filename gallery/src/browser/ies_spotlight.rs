//! webgpu_lights_ies_spotlight: four IESSpotLights (red, green, blue, white)
//! with shadows, their targets swinging toward the center, over a Phong
//! floor and box. IESLoader's profiles are half-float textures of 180
//! vertical angles by 360 horizontal ones ( one for a symmetric profile ); as
//! IESSpotLightNode does, each light's cone falloff is replaced by its
//! profile sampled at the tilt and twist angles of the fragment's light
//! direction, here in the Phong material's per-light color hook with the
//! core's cone disabled for these lights.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use crate::tsl::surface::SurfaceNodes;
use crate::tsl::{Type, WgslFn, light_color, light_index, position_world};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "assets/ies";
const FILES: [&str; 4] = [
    "007cfb11e343e2f42e3b476be4ab684e.ies",
    "06b4cfdc8805709e767b5e2e904be8ad.ies",
    "02a7562c650498ebb301153dbbf59207.ies",
    "1a936937a49c63374e6d4fbed9252b29.ies",
];
const LIGHTS: [(u32, [f64; 3]); 4] = [
    (0xff0000, [6.5, 3., 6.5]),
    (0x00ff00, [-6.5, 3., 6.5]),
    (0x0000ff, [-6.5, 3., -6.5]),
    (0xffffff, [6.5, 3., -6.5]),
];
/// IESLamp: the angles ( the horizontal ones shifted to start at 0 ) and the
/// scaled, normalized candela values.
struct Lamp {
    ver: Vec<f64>,
    hor: Vec<f64>,
    candela: Vec<Vec<f64>>,
}
fn parse_lamp(text: &str) -> Result<Lamp> {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut line = 0;
    let tokens = |l: &str| -> Vec<f64> {
        let t = l.trim().replace(',', " ");
        let mut v = vec![];
        for s in t.split(' ').filter(|s| !s.is_empty()) {
            v.push(s.parse::<f64>().unwrap_or(f64::NAN));
        }
        if v.is_empty() {
            // Number( '' ) of an empty line.
            v.push(0.);
        }
        v
    };
    let read = |count: usize, line: &mut usize| -> Result<Vec<f64>> {
        let mut out = vec![];
        while out.len() < count {
            let l = lines
                .get(*line)
                .ok_or(Error::Asset("IES: truncated".into()))?;
            *line += 1;
            out.extend(tokens(l));
        }
        Ok(out)
    };
    let tilt = loop {
        let l = lines.get(line).ok_or(Error::Asset("IES: no TILT".into()))?;
        line += 1;
        if l.contains("TILT") {
            break *l;
        }
    };
    if !tilt.contains("NONE") && tilt.contains("INCLUDE") {
        line += 1;
        let count = tokens(lines.get(line).copied().unwrap_or(""))[0] as usize;
        line += 1;
        read(count, &mut line)?;
        read(count, &mut line)?;
    }
    let values = read(10, &mut line)?;
    let multiplier = values[2];
    let (num_ver, num_hor) = (values[3] as usize, values[4] as usize);
    let factors = read(3, &mut line)?;
    let (ball, blp) = (factors[0], factors[1]);
    let ver = read(num_ver, &mut line)?;
    let mut hor = read(num_hor, &mut line)?;
    let mut candela = vec![];
    for _ in 0..num_hor {
        candela.push(read(num_ver, &mut line)?);
    }
    // candelaValues[ i ][ j ] *= multiplier × ballFactor × blpFactor.
    for row in &mut candela {
        for v in row.iter_mut() {
            *v *= multiplier * ball * blp;
        }
    }
    // The measured range shifted to start at 0, mirrored across the full
    // cycle when sampling.
    let base = hor.first().copied().unwrap_or(f64::NAN);
    let mut max = -1f64;
    for (angle, row) in hor.iter_mut().zip(&candela) {
        *angle -= base;
        for &v in row {
            max = if max < v { v } else { max };
        }
    }
    // A range wider than a half cycle is asymmetric: it wraps back around to
    // the first angle ( files that skip the final 360 entry ).
    let end = hor.last().copied().unwrap_or(f64::NAN);
    if end > 180. && end < 360. {
        hor.push(360.);
        candela.push(candela[0].clone());
    }
    if max > 0. {
        for row in &mut candela {
            for v in row.iter_mut() {
                *v /= max;
            }
        }
    }
    Ok(Lamp { ver, hor, candela })
}
/// _getIESValues' vertical angles: a row's 180 texels.
const HEIGHT: usize = 180;
/// _getIESValues: the texture's rows ( 360 horizontal angles, or one for a
/// symmetric profile ) of 180 vertical angles from 0 to 180 degrees.
fn profile(lamp: &Lamp) -> (usize, Vec<f64>) {
    let (nh, nv) = (lamp.hor.len(), lamp.ver.len());
    let width = if nh == 1 { 1 } else { 360 };
    let interpolate = |phi: f64, theta: f64| -> f64 {
        let (mut ti, mut t0, mut t1) = (0, 0., 0.);
        for i in 0..nh.saturating_sub(1) {
            if theta < lamp.hor[i + 1] || i + 2 == nh {
                (ti, t0, t1) = (i, lamp.hor[i], lamp.hor[i + 1]);
                break;
            }
        }
        let (mut pi, mut p0, mut p1) = (0, 0., 0.);
        for i in 0..nv.saturating_sub(1) {
            if phi < lamp.ver[i + 1] || i + 2 == nv {
                (pi, p0, p1) = (i, lamp.ver[i], lamp.ver[i + 1]);
                break;
            }
        }
        let (dt, dp) = (t1 - t0, p1 - p0);
        if dp == 0. {
            return 0.;
        }
        let a = if dt == 0. { 0. } else { (theta - t0) / dt };
        let b = (phi - p0) / dp;
        let next = if dt == 0. { ti } else { ti + 1 };
        // MathUtils.lerp.
        let lerp = |x: f64, y: f64, t: f64| (1. - t) * x + t * y;
        let c = &lamp.candela;
        let v1 = lerp(c[ti][pi], c[next][pi], a);
        let v2 = lerp(c[ti][pi + 1], c[next][pi + 1], a);
        lerp(v1, v2, b)
    };
    let end = lamp.hor[nh - 1];
    let mut data = vec![0.; width * HEIGHT];
    for (i, d) in data.iter_mut().enumerate() {
        // The texel at ( phi, theta ) is data[ phi + theta × 180 ].
        let (phi, theta) = (i % HEIGHT, i / HEIGHT);
        // The measured range mirrored around the cycle.
        let mut sample_theta = theta as f64;
        if end != 0. && sample_theta > end {
            sample_theta %= end * 2.;
            if sample_theta > end {
                sample_theta = end * 2. - sample_theta;
            }
        }
        // The vertical angles span [ 0, 180 ].
        let sample_phi = phi as f64 * 180. / (HEIGHT - 1) as f64;
        *d = interpolate(sample_phi, sample_theta);
    }
    (width, data)
}
/// DataUtils.toHalfFloat: the base and shift tables, truncating the mantissa.
fn to_half(value: f64) -> u16 {
    let f = (value.clamp(-65504., 65504.) as f32).to_bits();
    let i = ((f >> 23) & 0x1ff) as usize;
    let e = (i & 0xff) as i32 - 127;
    let sign = if i & 0x100 != 0 { 0x8000 } else { 0 };
    let (base, shift) = if e < -27 {
        (0, 24)
    } else if e < -14 {
        (0x0400 >> (-e - 14), (-e - 1) as u32)
    } else if e <= 15 {
        (((e + 15) << 10) as u32, 13)
    } else if e < 128 {
        (0x7c00, 24)
    } else {
        (0x7c00, 13)
    };
    ((base | sign) + ((f & 0x007f_ffff) >> shift)) as u16
}
/// IESSpotLightNode.getSpotAttenuation: light i's profile ( texture i ) at
/// the tilt angle off the spot direction, inset by half a texel on each side,
/// and the twist angle around it, offset by half a texel. The twist is
/// atan( y, x ) of the light's projected coordinate ( lightProjectionUV
/// − 0.5 ): its shadow camera's lookAt( target ) basis ( up +Y ) divided by
/// w, the positive projection and bias scales of its square map cancelling.
const IES: &str = "fn ies_color(index:u32,color:vec3<f32>,p:vec3<f32>)->vec3<f32>{let position=u.light_position[index].xyz;let l=normalize(position-p);var z=u.light_direction[index].xyz;let c=dot(l,z);let up=vec3(0.0,1.0,0.0);var x=cross(up,z);if dot(x,x)==0.0{z=normalize(z+vec3(0.0,0.0,0.0001));x=cross(up,z);}x=normalize(x);let y=cross(z,x);let d=p-position;let w=-dot(d,z);let coord=vec2(dot(d,x),dot(d,y))/w;var size=vec2<u32>(1u);switch index{case 0u:{size=textureDimensions(tsl_texture_0);}case 1u:{size=textureDimensions(tsl_texture_1);}case 2u:{size=textureDimensions(tsl_texture_2);}default:{size=textureDimensions(tsl_texture_3);}}let inset=vec2(0.5)/vec2<f32>(size);let twist=(atan2(coord.y,coord.x)+3.141592653589793)/(2.0*3.141592653589793)+inset.y;let tilt=acos(c)/3.141592653589793*((1.0-inset.x)-inset.x)+inset.x;let uv=vec2(tilt,twist);var a=0.0;switch index{case 0u:{a=textureSampleLevel(tsl_texture_0,tsl_sampler_0,uv,0.0).r;}case 1u:{a=textureSampleLevel(tsl_texture_1,tsl_sampler_1,uv,0.0).r;}case 2u:{a=textureSampleLevel(tsl_texture_2,tsl_sampler_2,uv,0.0).r;}default:{a=textureSampleLevel(tsl_texture_3,tsl_sampler_3,uv,0.0).r;}}return color*a;}";
pub(super) struct Demo {
    controls: Controls,
    lights: Vec<Object3D>,
    targets: Vec<Vector3>,
    cones: Vec<Object3D>,
    helpers: bool,
    time: f64,
    _textures: Vec<(wgpu::Texture, wgpu::TextureView)>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        // IESLoader: each light's 180 × ( 360 or 1 ) HalfFloat DataTexture,
        // LinearFilter, clamped in the vertical angle and repeating in the
        // horizontal one.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut textures = vec![];
        for file in FILES {
            let text = String::from_utf8(fetch(&format!("{ASSETS}/{file}")).await?)
                .map_err(|_| Error::Asset("IES text".into()))?;
            let (rows, data) = profile(&parse_lamp(&text)?);
            let texels: Vec<u16> = data.into_iter().map(to_half).collect();
            let size = wgpu::Extent3d {
                width: HEIGHT as u32,
                height: rows as u32,
                depth_or_array_layers: 1,
            };
            let texture = r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("IES profile"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R16Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            r.queue.write_texture(
                texture.as_image_copy(),
                bytemuck::cast_slice(&texels),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(HEIGHT as u32 * 2),
                    rows_per_image: Some(rows as u32),
                },
                size,
            );
            let view = texture.create_view(&Default::default());
            textures.push((texture, view));
        }
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 35.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(16., 4., 1.);
        s.background = Color::BLACK;
        // SpotLightHelper's cone: five rays and a 32-segment rim.
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
            Attribute::F32(BufferAttribute::new(
                cone.into_iter().map(|v| v as f32).collect(),
                3,
                false,
            )?),
        );
        let cone_geometry = Arc::new(cone_geometry);
        let (mut lights, mut targets, mut cones) = (vec![], vec![], vec![]);
        for (hex, position) in LIGHTS {
            let light = s.insert(NodeKind::Light(Light::Spot {
                color: Color::from_hex(hex),
                intensity: 500.,
                target: Vector3::ZERO,
                distance: 20.,
                decay: 2.,
                angle: PI / 8.,
                penumbra: 0.7,
                ies: true,
            }));
            let n = s.get_mut(light)?;
            n.position = Vector3::from_array(position);
            n.cast_shadow = true;
            // SpotLightShadow: 512², near 0.5, far = distance.
            n.shadow = crate::shadow::Shadow {
                map_size: Some(512),
                near: 0.5,
                far: 20.,
                ..Default::default()
            };
            lights.push(light);
            targets.push(Vector3::ZERO);
            let mut m = LineBasicMaterial::default();
            m.properties.color = Color::from_hex(hex);
            let cone = s.insert(NodeKind::Line(Line {
                geometry: cone_geometry.clone(),
                material: Arc::new(Material::Line(m)),
                segments: true,
            }));
            s.get_mut(cone)?.visible = false;
            cones.push(cone);
        }
        let hook = WgslFn::new(
            "ies_color",
            IES,
            &[Type::Uint, Type::Vec3, Type::Vec3],
            Type::Vec3,
        )?
        .call(&[light_index(), light_color(), position_world()]);
        let program = SurfaceNodes {
            light_color: Some(hook),
            ..Default::default()
        }
        .build_with_texture_types(
            r,
            &[],
            &textures
                .iter()
                .map(|(_, view)| (view, &sampler, Type::Texture))
                .collect::<Vec<_>>(),
        )
        .await?;
        let mut phong = MeshPhongMaterial::default();
        phong.properties.color = Color::from_hex(0x999999);
        phong.properties.vertex_program = Some(Arc::new(program));
        // The hook shades only: casters draw the plain depth program.
        phong.properties.shadow_program = Some(Arc::new(
            crate::shadow::ShadowProgram::new(r, crate::shader::DEFAULT_HOOKS).await?,
        ));
        let material = Arc::new(Material::Phong(phong));
        let floor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(200., 200., 1, 1)?),
            material.clone(),
        )));
        let n = s.get_mut(floor)?;
        n.quaternion = Quaternion::from_rotation_x(-PI * 0.5);
        n.receive_shadow = true;
        let cube = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(2., 2., 2.)?),
            material,
        )));
        let n = s.get_mut(cube)?;
        n.position.y = 1.;
        n.cast_shadow = true;
        let mut controls = Controls::new(None, (2., 50.), PI, true);
        controls.update(s, c)?;
        let mut demo = Self {
            controls,
            lights,
            targets,
            cones,
            helpers: false,
            time: 0.,
            _textures: textures,
        };
        demo.frame(s)?;
        Ok(demo)
    }
    /// render( time ): each light's target moves toward the center and
    /// SpotLightHelper.update() follows.
    fn frame(&mut self, s: &mut Scene) -> Result<()> {
        for (i, (&light, target)) in self.lights.iter().zip(&mut self.targets).enumerate() {
            let t = (((self.time + i as f64) * (PI / 2.)).sin() + 1.) / 2.;
            let position = s.get(light)?.position;
            *target = Vector3::new(
                position.x + (0. - position.x) * t,
                0.,
                position.z + (0. - position.z) * t,
            );
            if let NodeKind::Light(Light::Spot { target: t, .. }) = &mut s.get_mut(light)?.kind {
                *t = *target;
            }
            let cone = self.cones[i];
            let width = 20. * (PI / 8.).tan();
            let n = s.get_mut(cone)?;
            n.position = position;
            n.scale = Vector3::new(width, width, 20.);
            n.visible = self.helpers;
            s.look_at(cone, *target)?;
        }
        Ok(())
    }
    pub fn update(&mut self, s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        self.frame(s)
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        self.frame(s)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        _pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        // enablePan = false.
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    /// The helper switch.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("ies parameter"));
        }
        self.helpers = value > 0.5;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/webgpu_lights_ies_spotlight.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_lights_ies_spotlight(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                100.,
                Box::new(Demo::create(scene, camera, 360, renderer).await?),
            ))
        },
    )
    .await
}
