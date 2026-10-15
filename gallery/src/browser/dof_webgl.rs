//! webgl_postprocessing_dof: 14 × 9 × 14 spheres, each with its own
//! MeshBasicMaterial reflecting the SwedishRoyalCastle cube ( multiply ),
//! recolored every frame along the hue circle, viewed by a camera easing
//! toward the pointer, through RenderPass, BokehPass and OutputPass.
//! BokehPass renders the scene again with MeshDepthMaterial's RGBA depth
//! packing into a half-float target cleared white, then BokehShader's 41
//! taps around each pixel, spread by the distance to the focus.
use super::environment_materials::cube;
use super::ssao::{COPY, Pass, TEXEL, half, pass, target};
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::sync::Arc;

/// MeshBasicMaterial with the cube envMap: the reflection per vertex
/// ( envmap_vertex: the camera-to-vertex direction reflected about the world
/// normal, in the bitangent varying ), the cube sampled with flipEnvMap −1,
/// multiplied with the color ( u.custom[0].rgb ). u.custom[1].xyz is the
/// camera position.
const ENVMAP_VERTEX: &str = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let world_normal=normalize((vec4(surface.normal,0.0)*u.view).xyz);let to_vertex=normalize(surface.position-u.custom[1].xyz);out.bitangent=reflect(to_vertex,world_normal);return out;}";
const ENVMAP: &str = "fn dof_basic()->vec4<f32>{let r=fragment_surface.bitangent;let env=textureSample(tsl_texture_0,tsl_sampler_0,vec3(-r.x,r.y,r.z));return vec4(u.custom[0].rgb*env.rgb,1.0);}";
/// MeshDepthMaterial with RGBADepthPacking: the fragment's depth from its
/// view z ( u.custom[2]: near, far ), packed by packDepthToRGBA.
const DEPTH: &str = "fn dof_depth()->vec4<f32>{let near=u.custom[2].x;let far=u.custom[2].y;let z=fragment_surface.view_position.z;let v=(-far/(far-near)*z-far*near/(far-near))/(-z);if v<=0.0 {return vec4(0.0);}if v>=1.0 {return vec4(1.0);}var vuf=floor(v*16777216.0);let af=v*16777216.0-vuf;let b=vuf*(1.0/256.0);let bf=fract(b);vuf=floor(b);let g=vuf*(1.0/256.0);let gf=fract(g);vuf=floor(g);return vec4(vuf*(1.0/255.0),gf*(256.0/255.0),bf*(256.0/255.0),af);}";
const PROJECTION: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
/// BokehShader ( u.custom[0]: focus, aspect, aperture, maxblur;
/// [1]: nearClip, farClip ). Textures: tColor ( linear ), tDepth
/// ( nearest ). vUv is the fragment's uv with v up; the targets are stored
/// from their top row.
const BOKEH: &str = "fn dof_color(uv:vec2<f32>)->vec4<f32>{return textureSampleLevel(tsl_texture_0,tsl_sampler_0,vec2(uv.x,1.0-uv.y),0.0);}fn dof_bokeh()->vec4<f32>{let uv=fragment_surface.uv;let focus=u.custom[0].x;let aspect=u.custom[0].y;let aperture=u.custom[0].z;let maxblur=u.custom[0].w;let near=u.custom[1].x;let far=u.custom[1].y;let packed=textureSampleLevel(tsl_texture_1,tsl_sampler_1,vec2(uv.x,1.0-uv.y),0.0);let depth=dot(packed,vec4((255.0/256.0)/1.0,(255.0/256.0)/256.0,(255.0/256.0)/65536.0,1.0/16777216.0));let view_z=(near*far)/((far-near)*depth-far);let factor=focus+view_z;let a=vec2(1.0,aspect);let blur=vec2(clamp(factor*aperture,-maxblur,maxblur));let b9=blur*0.9;let b7=blur*0.7;let b4=blur*0.4;var col=vec4(0.0);
col+=dof_color(uv);
col+=dof_color(uv+(vec2(0.0,0.4)*a)*blur);col+=dof_color(uv+(vec2(0.15,0.37)*a)*blur);col+=dof_color(uv+(vec2(0.29,0.29)*a)*blur);col+=dof_color(uv+(vec2(-0.37,0.15)*a)*blur);col+=dof_color(uv+(vec2(0.40,0.0)*a)*blur);col+=dof_color(uv+(vec2(0.37,-0.15)*a)*blur);col+=dof_color(uv+(vec2(0.29,-0.29)*a)*blur);col+=dof_color(uv+(vec2(-0.15,-0.37)*a)*blur);col+=dof_color(uv+(vec2(0.0,-0.4)*a)*blur);col+=dof_color(uv+(vec2(-0.15,0.37)*a)*blur);col+=dof_color(uv+(vec2(-0.29,0.29)*a)*blur);col+=dof_color(uv+(vec2(0.37,0.15)*a)*blur);col+=dof_color(uv+(vec2(-0.4,0.0)*a)*blur);col+=dof_color(uv+(vec2(-0.37,-0.15)*a)*blur);col+=dof_color(uv+(vec2(-0.29,-0.29)*a)*blur);col+=dof_color(uv+(vec2(0.15,-0.37)*a)*blur);
col+=dof_color(uv+(vec2(0.15,0.37)*a)*b9);col+=dof_color(uv+(vec2(-0.37,0.15)*a)*b9);col+=dof_color(uv+(vec2(0.37,-0.15)*a)*b9);col+=dof_color(uv+(vec2(-0.15,-0.37)*a)*b9);col+=dof_color(uv+(vec2(-0.15,0.37)*a)*b9);col+=dof_color(uv+(vec2(0.37,0.15)*a)*b9);col+=dof_color(uv+(vec2(-0.37,-0.15)*a)*b9);col+=dof_color(uv+(vec2(0.15,-0.37)*a)*b9);
col+=dof_color(uv+(vec2(0.29,0.29)*a)*b7);col+=dof_color(uv+(vec2(0.40,0.0)*a)*b7);col+=dof_color(uv+(vec2(0.29,-0.29)*a)*b7);col+=dof_color(uv+(vec2(0.0,-0.4)*a)*b7);col+=dof_color(uv+(vec2(-0.29,0.29)*a)*b7);col+=dof_color(uv+(vec2(-0.4,0.0)*a)*b7);col+=dof_color(uv+(vec2(-0.29,-0.29)*a)*b7);col+=dof_color(uv+(vec2(0.0,0.4)*a)*b7);
col+=dof_color(uv+(vec2(0.29,0.29)*a)*b4);col+=dof_color(uv+(vec2(0.4,0.0)*a)*b4);col+=dof_color(uv+(vec2(0.29,-0.29)*a)*b4);col+=dof_color(uv+(vec2(0.0,-0.4)*a)*b4);col+=dof_color(uv+(vec2(-0.29,0.29)*a)*b4);col+=dof_color(uv+(vec2(-0.4,0.0)*a)*b4);col+=dof_color(uv+(vec2(-0.29,-0.29)*a)*b4);col+=dof_color(uv+(vec2(0.0,0.4)*a)*b4);
var result=col/41.0;result.a=1.0;return result;}";
/// Color.setHSL in the working ( linear ) color space.
fn hsl(h: f64, s: f64, l: f64) -> Color {
    let h = h.rem_euclid(1.);
    let hue = |p: f64, q: f64, mut t: f64| {
        if t < 0. {
            t += 1.;
        }
        if t > 1. {
            t -= 1.;
        }
        if t < 1. / 6. {
            p + (q - p) * 6. * t
        } else if t < 1. / 2. {
            q
        } else if t < 2. / 3. {
            p + (q - p) * 6. * (2. / 3. - t)
        } else {
            p
        }
    };
    if s == 0. {
        return Color(Vector3::splat(l));
    }
    let p = if l <= 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let q = 2. * l - p;
    Color(Vector3::new(
        hue(q, p, h + 1. / 3.),
        hue(q, p, h),
        hue(q, p, h - 1. / 3.),
    ))
}
struct Targets {
    width: u32,
    height: u32,
    /// The composer's read and write buffers, and BokehPass's depth target.
    read: RenderTarget,
    write: RenderTarget,
    depth: RenderTarget,
}
pub(super) struct Demo {
    time: f64,
    steps: u32,
    mouse: Vector2,
    camera_position: Vector2,
    spheres: Vec<Object3D>,
    /// BokehPass's override render: the same spheres with MeshDepthMaterial
    /// ( RGBADepthPacking ), cleared white, and its camera.
    depth_scene: Scene,
    depth_camera: Object3D,
    /// focus, aperture × 0.00001, maxblur.
    effect: [f64; 3],
    targets: Option<Targets>,
    bokeh: Pass,
    output: Pass,
    linear: wgpu::Sampler,
    nearest: wgpu::Sampler,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 1.,
            far: 3000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 200.);
        s.look_at(c, Vector3::ZERO)?;
        s.background = Color::BLACK;
        let environment = cube(r, "SwedishRoyalCastle").await?;
        let node = WgslFn::new("dof_basic", ENVMAP, &[], Type::Vec4)?.call(&[]);
        let program = Arc::new(
            ShaderProgram::with_projection_and_sample_types(
                r,
                &NodeMaterial::new(node).wgsl_with_texture_types(&[Type::TextureCube], &[])?,
                &[(&environment.view, &environment.sampler)],
                &[wgpu::TextureViewDimension::Cube],
                &[wgpu::TextureSampleType::Float { filterable: true }],
                ENVMAP_VERTEX,
            )
            .await?,
        );
        let depth = WgslFn::new("dof_depth", DEPTH, &[], Type::Vec4)?.call(&[]);
        let depth_program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(depth).wgsl(0)?,
            &[],
            &[],
            PROJECTION,
        )
        .await?;
        let mut depth_material = ShaderMaterial::new(Arc::new(depth_program));
        // NoBlending.
        depth_material.uniforms[2] = [1., 3000., 0., 0.];
        let depth_material = Arc::new(Material::Shader(depth_material));
        let mut depth_scene = Scene::new();
        depth_scene.background = Color::WHITE;
        let depth_camera =
            depth_scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                fov: 70.,
                near: 1.,
                far: 3000.,
                aspect,
                ..Default::default()
            })));
        let geometry = Arc::new(SphereGeometry::build(1., 20, 10)?);
        let (xgrid, ygrid, zgrid) = (14, 9, 14);
        let mut spheres = vec![];
        for i in 0..xgrid {
            for j in 0..ygrid {
                for k in 0..zgrid {
                    let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                        geometry.clone(),
                        Arc::new(Material::Shader(ShaderMaterial::new(program.clone()))),
                    )));
                    let n = s.get_mut(mesh)?;
                    n.position = Vector3::new(
                        200. * (i as f64 - xgrid as f64 / 2.),
                        200. * (j as f64 - ygrid as f64 / 2.),
                        200. * (k as f64 - zgrid as f64 / 2.),
                    );
                    n.scale = Vector3::splat(60.);
                    let (position, scale) = (n.position, n.scale);
                    spheres.push(mesh);
                    let depth = depth_scene.insert(NodeKind::Mesh(Mesh::new(
                        geometry.clone(),
                        depth_material.clone(),
                    )));
                    let n = depth_scene.get_mut(depth)?;
                    n.position = position;
                    n.scale = scale;
                }
            }
        }
        let nearest = r.device.create_sampler(&Default::default());
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let placeholder = target(r, 1, 1, false)?;
        let view = placeholder.texture.create_view(&Default::default());
        Ok(Self {
            time: 0.,
            steps: 1,
            mouse: Vector2::ZERO,
            camera_position: Vector2::ZERO,
            spheres,
            depth_scene,
            depth_camera,
            // matChanger(): focus 500, aperture 5 × 0.00001, maxblur 0.01.
            effect: [500., 5. * 0.00001, 0.01],
            targets: None,
            bokeh: pass(
                r,
                "dof_bokeh",
                BOKEH,
                &[
                    (&view, &linear, Type::Texture, half()),
                    (&view, &nearest, Type::Texture, half()),
                ],
            )
            .await?,
            output: pass(
                r,
                "ssao_copy",
                &format!("{TEXEL}{COPY}"),
                &[(&view, &nearest, Type::Texture, half())],
            )
            .await?,
            linear,
            nearest,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        self.steps = self.steps.max(u32::from(animate));
        Ok(())
    }
    /// render(): the camera eases toward the pointer and looks at the origin;
    /// every material takes its hue for Date.now() * 0.00005.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        for _ in 0..std::mem::take(&mut self.steps) {
            let p = &mut self.camera_position;
            p.x += (self.mouse.x - p.x) * 0.036;
            p.y += (-self.mouse.y - p.y) * 0.036;
        }
        let n = s.get_mut(c)?;
        n.position.x = self.camera_position.x;
        n.position.y = self.camera_position.y;
        s.look_at(c, Vector3::ZERO)?;
        let time = self.time * 1000. * 0.00005;
        let count = self.spheres.len() as f64;
        for (i, sphere) in self.spheres.iter().enumerate() {
            let h = (360. * (i as f64 / count + time) % 360.) / 360.;
            let color = hsl(h, 1., 0.5);
            if let NodeKind::Mesh(mesh) = &mut s.get_mut(*sphere)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
            {
                m.uniforms[0] = [color.0.x as f32, color.0.y as f32, color.0.z as f32, 1.];
            }
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, w: u32, h: u32) -> Result<()> {
        let read = target(r, w, h, true)?;
        let write = target(r, w, h, false)?;
        let depth = target(r, w, h, true)?;
        let (read_view, depth_view, write_view) = (
            read.texture.create_view(&Default::default()),
            depth.texture.create_view(&Default::default()),
            write.texture.create_view(&Default::default()),
        );
        Arc::make_mut(&mut self.bokeh.material()?.program).rebind(
            r,
            &[],
            &[(&read_view, &self.linear), (&depth_view, &self.nearest)],
        )?;
        Arc::make_mut(&mut self.output.material()?.program).rebind(
            r,
            &[],
            &[(&write_view, &self.nearest)],
        )?;
        self.targets = Some(Targets {
            width: w,
            height: h,
            read,
            write,
            depth,
        });
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .targets
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.resize(r, out.width, out.height)?;
        }
        let t = self.targets.as_ref().ok_or(Error::Invalid("dof targets"))?;
        for sphere in &self.spheres {
            if let NodeKind::Mesh(mesh) = &mut s.get_mut(*sphere)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
            {
                m.uniforms[1] = [
                    self.camera_position.x as f32,
                    self.camera_position.y as f32,
                    200.,
                    0.,
                ];
            }
        }
        // RenderPass.
        r.render(s, c, &t.read)?;
        // BokehPass: the depth render with the override material, cleared white.
        let (camera, transform) = {
            let n = s.get(c)?;
            (n.kind.clone(), (n.position, n.quaternion))
        };
        let n = self.depth_scene.get_mut(self.depth_camera)?;
        n.kind = camera;
        (n.position, n.quaternion) = transform;
        r.render(&mut self.depth_scene, self.depth_camera, &t.depth)?;
        let aspect = t.width as f64 / t.height as f64;
        let mut uniforms = [[0f32; 4]; 16];
        uniforms[0] = [
            self.effect[0] as f32,
            aspect as f32,
            self.effect[1] as f32,
            self.effect[2] as f32,
        ];
        uniforms[1] = [1., 3000., 0., 0.];
        self.bokeh.render(r, &t.write, &uniforms)?;
        // OutputPass.
        self.output.render(r, out, &uniforms)?;
        Ok(true)
    }
    /// onPointerMove: the pointer relative to the window's center.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if kind != 0 {
            return;
        }
        let (w, h, _) = super::controls_attributes::viewport_css();
        self.mouse = Vector2::new(x - w / 2., y - h / 2.);
    }
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
    /// The GUI: focus, aperture, maxblur.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let value = value as f64;
        match index {
            0 => self.effect[0] = value,
            1 => self.effect[1] = value * 0.00001,
            2 => self.effect[2] = value,
            _ => return Err(Error::Invalid("dof parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.steps += 1;
    }
}
