//! webgpu_lights_ies_spotlight: four IESSpotLights (red, green, blue, white)
//! with shadows, their targets swinging toward the center, over a Phong
//! floor and box. IESLoader's profiles are 180-texel half-float rows; as
//! IESSpotLightNode does, each light's cone falloff is replaced by its
//! profile sampled at acos( L · D ) / π, here in the Phong material's
//! per-light color hook with the core's cone disabled for these lights.
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

const ASSETS: &str = "/web/gallery/assets/ies";
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
/// IESLamp: the angles and the squared, normalized candela values.
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
    read(3, &mut line)?;
    let ver = read(num_ver, &mut line)?;
    let hor = read(num_hor, &mut line)?;
    let mut candela = vec![];
    for _ in 0..num_hor {
        candela.push(read(num_ver, &mut line)?);
    }
    // candelaValues[ i ][ j ] *= candelaValues[ i ][ j ] × multiplier.
    let mut max = -1f64;
    for row in &mut candela {
        for v in row.iter_mut() {
            let value = *v;
            *v = value * (value * multiplier);
            max = if max < *v { *v } else { max };
        }
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
/// _getIESValues' first 180 entries (horizontal angle 0), the texture's row.
fn profile(lamp: &Lamp) -> Vec<f64> {
    let (nh, nv) = (lamp.hor.len(), lamp.ver.len());
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
        let lerp = |x: f64, y: f64, t: f64| x + (y - x) * t;
        let c = &lamp.candela;
        let v1 = lerp(c[ti][pi], c[next][pi], a);
        let v2 = lerp(c[ti][pi + 1], c[next][pi + 1], a);
        lerp(v1, v2, b)
    };
    let (start, end) = (lamp.hor[0], lamp.hor[nh - 1]);
    (0..180)
        .map(|phi| {
            let mut theta = 0f64;
            if end - start != 0. && (theta < start || theta >= end) {
                theta %= end * 2.;
                if theta > end {
                    theta = end * 2. - theta;
                }
            }
            interpolate(phi as f64, theta)
        })
        .collect()
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
/// IESSpotLightNode.getSpotAttenuation: the profile of light i at the angle
/// between the fragment's light direction and the spot direction.
const IES: &str = "fn ies_color(index:u32,color:vec3<f32>,p:vec3<f32>)->vec3<f32>{let l=normalize(u.light_position[index].xyz-p);let c=dot(l,u.light_direction[index].xyz);let v=(f32(index)+0.5)/4.0;return color*textureSampleLevel(tsl_texture_0,tsl_sampler_0,vec2(acos(c)*(1.0/3.141592653589793),v),0.0).r;}";
pub(super) struct Demo {
    controls: Controls,
    lights: Vec<Object3D>,
    targets: Vec<Vector3>,
    cones: Vec<Object3D>,
    helpers: bool,
    time: f64,
    _texture: wgpu::Texture,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let mut texels = vec![];
        for file in FILES {
            let text = String::from_utf8(fetch(&format!("{ASSETS}/{file}")).await?)
                .map_err(|_| Error::Asset("IES text".into()))?;
            texels.extend(profile(&parse_lamp(&text)?).into_iter().map(to_half));
        }
        let size = wgpu::Extent3d {
            width: 180,
            height: 4,
            depth_or_array_layers: 1,
        };
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("IES profiles"),
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
                bytes_per_row: Some(360),
                rows_per_image: Some(4),
            },
            size,
        );
        let view = texture.create_view(&Default::default());
        // IESLoader: LinearFilter, clamped.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
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
        .build_with_texture_types(r, &[], &[(&view, &sampler, Type::Texture)])
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
            _texture: texture,
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
