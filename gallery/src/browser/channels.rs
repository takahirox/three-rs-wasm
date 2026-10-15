//! webgl_materials_channels: the displaced ninja head through
//! MeshStandardMaterial, MeshNormalMaterial (tangent-space normal map), the
//! VelocityShader and MeshDepthMaterial's four packings, with front, back or
//! double sides, a perspective and an orthographic camera and one damped
//! OrbitControls per camera. Displacement stays on the GPU in every material.
use super::controls_attributes::{
    CameraState, Controls, camera_state, viewport_css, webgl_orthographic, webgl_perspective,
};
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::shader::ShaderProgram;
use crate::tsl::{
    self, NodeMaterial, Type, WgslFn, float, normal_geometry, position_geometry,
    surface::SurfaceNodes, uniform, uv, vec2,
};
use crate::{Error, Result, camera::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets/environment-materials";
const SCALE: f64 = 2.436143;
const BIAS: f64 = -0.428408;
/// displacementmap_vertex: the image is stored from its top row, so the
/// flipped texture's v is 1 − uv.y; `texture2D` in the vertex stage reads
/// level 0.
const DISPLACE: &str = "fn displaced(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{let h=textureSampleLevel(tsl_texture_0,tsl_sampler_0,vec2(uv.x,1.0-uv.y),0.0).x;return position+normalize(normal)*(h*2.436143+-0.428408);}";
/// MeshNormalMaterial's vertex stage: vNormal (normalMatrix × objectNormal,
/// negated for BackSide) in `normal`, vViewPosition in `view_position`.
/// u.custom[15].x is the side (0 front, 1 back, 2 double).
fn normal_projection() -> String {
    format!(
        "{DISPLACE}fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{var out=surface;let p=displaced(position,surface.local_normal,surface.uv);let mv=u.view*u.model*vec4(p,1.0);out.clip=u.projection*mv;out.view_position=mv.xyz;var n=normalize((u.view*vec4((u.normal*vec4(surface.local_normal,0.0)).xyz,0.0)).xyz);out.normal=n;return out;}}"
    )
}
/// MeshDepthMaterial: gl_Position = projectionMatrix × modelViewMatrix ×
/// transformed with WebGL's -1..1 depth range (u.custom[4..8] and
/// u.custom[0..4], composed in f64 as three does) for vHighPrecisionZW,
/// carried in local_position.xy.
fn depth_projection() -> String {
    format!(
        "{DISPLACE}fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{var out=surface;let p=displaced(position,surface.local_normal,surface.uv);let mv=mat4x4<f32>(u.custom[0],u.custom[1],u.custom[2],u.custom[3])*vec4(p,1.0);let gl=mat4x4<f32>(u.custom[4],u.custom[5],u.custom[6],u.custom[7])*mv;out.clip=u.projection*(u.view*u.model*vec4(p,1.0));out.local_position=vec3(gl.zw,0.0);return out;}}"
    )
}
/// VelocityShader: the current and previous clip positions from
/// currentProjectionViewMatrix × modelMatrix and previousProjectionViewMatrix
/// × modelMatrixPrev (u.custom[0..4], [4..8], [8..12]), in local_position
/// (x, y, w) and view_position (x, y, w).
fn velocity_projection() -> String {
    format!(
        "{DISPLACE}fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{{var out=surface;let p=displaced(position,surface.local_normal,surface.uv);out.clip=u.projection*(u.view*u.model*vec4(p,1.0));let current=mat4x4<f32>(u.custom[0],u.custom[1],u.custom[2],u.custom[3])*u.model*vec4(p,1.0);let previous=mat4x4<f32>(u.custom[4],u.custom[5],u.custom[6],u.custom[7])*mat4x4<f32>(u.custom[8],u.custom[9],u.custom[10],u.custom[11])*vec4(p,1.0);out.local_position=current.xyw;out.view_position=previous.xyw;return out;}}"
    )
}
/// The outputs are written as they are (none of these shaders includes
/// colorspace_fragment): the target's sRGB encoding is undone first.
const RAW: &str = "fn raw(c:vec4<f32>)->vec4<f32>{return vec4(srgb_input(c.rgb),c.a);}";
/// meshnormal.glsl: normal_fragment_begin and normal_fragment_maps with
/// getTangentFrame (WebGL's dFdy is −dpdy); the map is sampled at the
/// flipped v, the frame uses the geometry's uv as vNormalMapUv; u.custom[12].xy
/// is the normalScale uniform.
const NORMAL_FRAGMENT: &str = "fn channel_normal()->vec4<f32>{let s=fragment_surface;let face=select(-1.0,1.0,fragment_front);var normal=normalize(s.normal);if u.custom[15].x==1.0 {normal=-normal;}let double=u.custom[15].x==2.0;if double {normal*=face;}let eye=s.view_position;let q0=dpdx(eye);let q1=-dpdy(eye);let st0=dpdx(s.uv);let st1=-dpdy(s.uv);let q1perp=cross(q1,normal);let q0perp=cross(normal,q0);var t=q1perp*st0.x+q0perp*st1.x;var b=q1perp*st0.y+q0perp*st1.y;let det=max(dot(t,t),dot(b,b));let scale=select(inverseSqrt(det),0.0,det==0.0);t*=scale;b*=scale;if double {t*=face;b*=face;}var map=textureSample(tsl_texture_1,tsl_sampler_1,vec2(s.uv.x,1.0-s.uv.y)).xyz*2.0-1.0;map=vec3(map.xy*u.custom[12].xy,map.z);normal=normalize(mat3x3<f32>(t,b,normal)*map);return raw(vec4(normalize(normal)*0.5+0.5,1.0));}";
/// depth.glsl with packing.glsl's packDepthToRGBA/RGB/RG; u.custom[14].x
/// is the packing (0 basic, 1 RGBA, 2 RGB, 3 RG).
const DEPTH_FRAGMENT: &str = "fn channel_depth()->vec4<f32>{let zw=fragment_surface.local_position.xy;let v=0.5*zw.x/zw.y+0.5;let packing=u.custom[14].x;var c=vec4(vec3(1.0-v),1.0);if packing==1.0 {if v<=0.0 {c=vec4(0.0);} else if v>=1.0 {c=vec4(1.0);} else {var vuf:f32;let a=modf(v*16777216.0);vuf=a.whole;let bb=modf(vuf*(1.0/256.0));vuf=bb.whole;let g=modf(vuf*(1.0/256.0));vuf=g.whole;c=vec4(vuf*(1.0/255.0),g.fract*(256.0/255.0),bb.fract*(256.0/255.0),a.fract);}} else if packing==2.0 {if v<=0.0 {c=vec4(0.0,0.0,0.0,1.0);} else if v>=1.0 {c=vec4(1.0);} else {var vuf:f32;let bb=modf(v*65536.0);vuf=bb.whole;let g=modf(vuf*(1.0/256.0));vuf=g.whole;c=vec4(vuf*(1.0/255.0),g.fract*(256.0/255.0),bb.fract,1.0);}} else if packing==3.0 {if v<=0.0 {c=vec4(0.0,0.0,0.0,1.0);} else if v>=1.0 {c=vec4(1.0);} else {let gg=modf(v*256.0);c=vec4(gg.whole*(1.0/255.0),gg.fract,0.0,1.0);}}return raw(c);}";
/// VelocityShader's fragment: each NDC velocity component packed to RG.
const VELOCITY_FRAGMENT: &str = "fn pack_rg(v:f32)->vec2<f32>{if v<=0.0 {return vec2(0.0);}if v>=1.0 {return vec2(1.0);}let gg=modf(v*256.0);return vec2(gg.whole*(1.0/255.0),gg.fract);}fn channel_velocity()->vec4<f32>{let current=fragment_surface.local_position;let previous=fragment_surface.view_position;var vel=(current.xy/current.z-previous.xy/previous.z)*0.5;vel=vel*0.5+0.5;let v1=pack_rg(vel.x);let v2=pack_rg(vel.y);return raw(vec4(v1.x,v1.y,v2.x,v2.y));}";
async fn image(path: &str) -> Result<Texture> {
    let mut t = decode_texture_image(&fetch(&format!("{ASSETS}/{path}")).await?).await?;
    t.srgb = false;
    t.mipmap_filter = Some(Filter::Linear);
    Ok(t)
}
async fn program(
    r: &Renderer,
    name: &str,
    fragment: &str,
    projection: &str,
    textures: &[(&wgpu::TextureView, &wgpu::Sampler)],
) -> Result<ShaderMaterial> {
    let node = WgslFn::new(name, &format!("{RAW}{fragment}"), &[], Type::Vec4)?.call(&[]);
    let types = vec![Type::Texture; textures.len()];
    let dimensions = vec![wgpu::TextureViewDimension::D2; textures.len()];
    Ok(ShaderMaterial::new(Arc::new(
        ShaderProgram::with_projection_and_dimensions(
            r,
            &NodeMaterial::new(node).wgsl_with_texture_types(&types, &[])?,
            textures,
            &dimensions,
            projection,
        )
        .await?,
    )))
}
fn columns(m: Matrix4) -> [[f32; 4]; 4] {
    m.to_cols_array_2d().map(|c| c.map(|v| v as f32))
}
pub(super) struct Demo {
    mesh: Object3D,
    perspective: Object3D,
    orthographic: Object3D,
    /// controlsPerspective and controlsOrtho, both driven by the canvas.
    controls: [Controls; 2],
    /// standard, normal, velocity, depthBasic, depthRGBA, depthRGB, depthRG.
    materials: Vec<Material>,
    /// material, camera (0 perspective, 1 ortho), side (0 front, 1 back, 2 double).
    params: [f64; 3],
    selected: Option<(usize, usize)>,
    current_pv: Matrix4,
    previous_pv: Matrix4,
    /// The mesh's matrixWorldPrevious: identity until its first render.
    model_previous: Matrix4,
    viewport: (f64, f64),
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let (w, h, _) = viewport_css();
        let aspect = w / h.max(1.);
        let perspective = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 500.,
            far: 3000.,
            aspect,
            ..Default::default()
        })));
        s.get_mut(perspective)?.position = Vector3::new(0., 0., 1500.);
        let orthographic = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -500. * aspect,
            right: 500. * aspect,
            top: 500.,
            bottom: -500.,
            near: 1000.,
            far: 2500.,
            zoom: 1.,
            ..Default::default()
        })));
        s.get_mut(orthographic)?.position = Vector3::new(0., 0., 1500.);
        s.background = Color::BLACK;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.3,
        }));
        for (color, intensity, position, parent) in [
            (0xff0000, 1.5, Vector3::new(0., 0., 2500.), false),
            (0xff6666, 3., Vector3::ZERO, true),
            (0x0000ff, 1.5, Vector3::new(-1000., 0., 1000.), false),
        ] {
            let light = s.insert(NodeKind::Light(Light::Point {
                color: Color::from_hex(color),
                intensity,
                distance: 0.,
                decay: 0.,
            }));
            s.get_mut(light)?.position = position;
            if parent {
                s.add(perspective, light)?;
            }
        }
        let normal_map = Arc::new(image("models/obj/ninja/normal.png").await?);
        let ao_map = Arc::new(image("models/obj/ninja/ao.jpg").await?);
        let displacement =
            r.upload_texture(&Arc::new(image("models/obj/ninja/displacement.jpg").await?))?;
        let normal = r.upload_texture(&normal_map)?;
        let geometry = Arc::new(
            super::tsl_materials::geometries(&fetch(&format!("{ASSETS}/ninja.bin")).await?)?
                .remove(0),
        );
        // MeshStandardMaterial: the displacement through the shared vertex program.
        let mut standard = MeshStandardMaterial {
            energy_conservation: true,
            roughness: 0.6,
            metalness: 0.5,
            normal_scale: Vector2::new(1., -1.),
            normal_map: Some(normal_map),
            occlusion_map: Some(ao_map),
            ..Default::default()
        };
        standard.properties.color = Color::WHITE;
        let coordinate = vec2(uv().x(), float(1.) - uv().y());
        let height = tsl::Texture::External(0)
            .sample_level(coordinate, float(0.))
            .x();
        let position = position_geometry()
            + normal_geometry().normalize()
                * (height * uniform(0, Type::Float) + float(BIAS as f32));
        standard.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                position: Some(position),
                ..Default::default()
            }
            .build(r, &[], &[(&displacement.view, &displacement.sampler)])
            .await?,
        ));
        standard.properties.vertex_uniforms[0][0] = SCALE as f32;
        let displacement_only = [(&displacement.view, &displacement.sampler)];
        let normal_material = program(
            r,
            "channel_normal",
            NORMAL_FRAGMENT,
            &normal_projection(),
            &[
                (&displacement.view, &displacement.sampler),
                (&normal.view, &normal.sampler),
            ],
        )
        .await?;
        let velocity = program(
            r,
            "channel_velocity",
            VELOCITY_FRAGMENT,
            &velocity_projection(),
            &displacement_only,
        )
        .await?;
        let depth = program(
            r,
            "channel_depth",
            DEPTH_FRAGMENT,
            &depth_projection(),
            &displacement_only,
        )
        .await?;
        let mut materials = vec![
            Material::Standard(standard),
            Material::Shader(normal_material),
            Material::Shader(velocity),
        ];
        for packing in 0..4 {
            let mut m = depth.clone();
            m.uniforms[14][0] = packing as f32;
            materials.push(Material::Shader(m));
        }
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            geometry,
            Arc::new(materials[1].clone()),
        )));
        let n = s.get_mut(mesh)?;
        n.scale = Vector3::splat(25.);
        n.frustum_culled = false;
        // OrbitControls: distances 1000–2400 and zoom 0.5–1.5, damped.
        let mut controls = [
            Controls::new(Some(0.05), (1000., 2400.), PI, true),
            Controls::new(Some(0.05), (0., f64::INFINITY), PI, true),
        ];
        controls[0].update(s, perspective)?;
        controls[1].update(s, orthographic)?;
        let _ = c;
        Ok(Self {
            mesh,
            perspective,
            orthographic,
            controls,
            materials,
            params: [1., 0., 2.],
            selected: None,
            current_pv: Matrix4::IDENTITY,
            previous_pv: Matrix4::IDENTITY,
            model_previous: Matrix4::IDENTITY,
            viewport: (w, h),
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    /// The WebGL projection matrix of a camera node.
    fn gl_projection(camera: &Camera) -> Matrix4 {
        match camera {
            Camera::Perspective(p) => webgl_perspective(p.fov, p.aspect, p.near, p.far),
            Camera::Orthographic(o) => {
                let (dx, dy) = (
                    (o.right - o.left) / (2. * o.zoom),
                    (o.top - o.bottom) / (2. * o.zoom),
                );
                let (cx, cy) = ((o.right + o.left) / 2., (o.top + o.bottom) / 2.);
                webgl_orthographic(cx - dx, cx + dx, cy + dy, cy - dy, o.near, o.far)
            }
        }
    }
    /// render(): the selected material and side, both controls' damped
    /// update, the velocity matrices, then the active camera renders.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        // onWindowResize changes only the camera active at the time: the
        // orthographic frustum becomes ± innerHeight × aspect by ± innerHeight.
        let (w, h, _) = viewport_css();
        if (w, h) != self.viewport {
            self.viewport = (w, h);
            let aspect = w / h.max(1.);
            let active = if self.params[1] > 0.5 {
                self.orthographic
            } else {
                self.perspective
            };
            match &mut s.get_mut(active)?.kind {
                NodeKind::Camera(Camera::Perspective(p)) => p.aspect = aspect,
                NodeKind::Camera(Camera::Orthographic(o)) => {
                    o.left = -h * aspect;
                    o.right = h * aspect;
                    o.top = h;
                    o.bottom = -h;
                }
                _ => {}
            }
        }
        let material = (self.params[0].round() as usize).min(6);
        let side = (self.params[2].round() as usize).min(2);
        if self.selected != Some((material, side)) {
            self.selected = Some((material, side));
            let mut m = self.materials[material].clone();
            m.properties_mut().side = [Side::Front, Side::Back, Side::Double][side];
            // WebGLMaterials negates the normalScale uniform of a BackSide material.
            let scale = if side == 1 {
                Vector2::new(-1., 1.)
            } else {
                Vector2::new(1., -1.)
            };
            match &mut m {
                Material::Shader(m) => {
                    m.uniforms[15][0] = side as f32;
                    m.uniforms[12] = [scale.x as f32, scale.y as f32, 0., 0.];
                }
                Material::Standard(m) => m.normal_scale = scale,
                _ => {}
            }
            if let NodeKind::Mesh(mesh) = &mut s.get_mut(self.mesh)?.kind {
                mesh.materials[0] = Arc::new(m);
            }
        }
        self.controls[0].update(s, self.perspective)?;
        let scale = self.controls[1].take_scale();
        if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(self.orthographic)?.kind {
            o.zoom = (o.zoom / scale).clamp(0.5, 1.5);
        }
        self.controls[1].update(s, self.orthographic)?;
        let active = if self.params[1] > 0.5 {
            self.orthographic
        } else {
            self.perspective
        };
        let n = s.get(active)?;
        let (kind, position, quaternion) = (n.kind.clone(), n.position, n.quaternion);
        let camera = match &kind {
            NodeKind::Camera(camera) => camera.clone(),
            _ => return Err(Error::Invalid("channels camera")),
        };
        let n = s.get_mut(c)?;
        n.kind = kind;
        n.position = position;
        n.quaternion = quaternion;
        let projection = Self::gl_projection(&camera);
        let view = Matrix4::from_rotation_translation(quaternion, position).inverse();
        self.previous_pv = self.current_pv;
        self.current_pv = projection * view;
        let model = Matrix4::from_scale(Vector3::splat(25.));
        let (current, previous, model_previous, model_view) = (
            columns(self.current_pv),
            columns(self.previous_pv),
            columns(self.model_previous),
            columns(view * model),
        );
        let gl = columns(projection);
        if let NodeKind::Mesh(mesh) = &mut s.get_mut(self.mesh)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
        {
            if material == 2 {
                m.uniforms[0..4].copy_from_slice(&current);
                m.uniforms[4..8].copy_from_slice(&previous);
                m.uniforms[8..12].copy_from_slice(&model_previous);
            } else {
                m.uniforms[0..4].copy_from_slice(&model_view);
                m.uniforms[4..8].copy_from_slice(&gl);
            }
        }
        // matrixWorldPrevious is copied after the render.
        self.model_previous = model;
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    /// Both OrbitControls listen to the canvas: rotate, pan (per camera
    /// type) and the wheel's dolly or zoom.
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &mut Scene,
        _c: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if wheel != 0. {
            for controls in &mut self.controls {
                controls.wheel_scale(wheel);
            }
        } else if pan {
            let camera: CameraState = camera_state(s, self.perspective)?;
            self.controls[0].pan(&camera, dx, dy, height);
            let n = s.get(self.orthographic)?;
            if let NodeKind::Camera(Camera::Orthographic(o)) = &n.kind {
                let (width, _, _) = viewport_css();
                let left = dx * (o.right - o.left) / o.zoom / width;
                let up = dy * (o.top - o.bottom) / o.zoom / height;
                self.controls[1].pan_axes(n.quaternion, left, up);
            }
        } else {
            for controls in &mut self.controls {
                controls.rotate(dx, dy, height);
            }
        }
        Ok(())
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let p = self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("channels parameter"))?;
        *p = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
