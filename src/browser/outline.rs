//! webgpu_postprocessing_outline: the OBJ tree, 20 Lambert spheres, the floor
//! and the torus under a shadowing SunLight, with OutlineNode around the
//! object under the pointer: the non-selected depth pass, the selected mask
//! with its depth test, the half-resolution downsample and edge detection,
//! the half- and quarter-resolution separable blurs and the composite, added
//! to the scene pass with the edge colors, strength and pulse.
use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::fetch;
use super::ssao::{Pass, pass};
use super::terrain_loaders::parse_obj;
use crate::attribute::BufferAttribute;
use crate::raycast::Raycaster;
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

/// The pass's pixel as uv from the top-left (WebGPU's screenUV and quad uv);
/// u.custom[3].xy is the target size.
const UV: &str = "fn outline_uv()->vec2<f32>{return fragment_surface.clip.xy/u.custom[3].xy;}";
/// OutlineNode's prepareMask: ( 0, depthTest, 1 ), the fragment in front of
/// the non-selected depth (texture 0, read at screenUV). u.custom[0].xy is
/// (near, far).
const MASK: &str = "fn outline_mask()->vec4<f32>{let depth=textureLoad(tsl_texture_0,vec2<i32>(fragment_surface.clip.xy),0);let near=u.custom[0].x;let far=u.custom[0].y;let view_z=(near*far)/((far-near)*depth-far);let visible=select(0.0,1.0,fragment_surface.view_position.z<=view_z);return vec4(0.0,visible,1.0,1.0);}";
/// The downsample copy: the mask sampled bilinearly.
const COPY: &str =
    "fn outline_copy()->vec4<f32>{return textureSample(tsl_texture_0,tsl_sampler_0,outline_uv());}";
/// Edge detection on the downsampled mask: visible edges red, hidden green.
const EDGE: &str = "fn outline_edge()->vec4<f32>{let uv=outline_uv();let inv=1.0/vec2<f32>(textureDimensions(tsl_texture_0));let o=vec4(1.0,0.0,0.0,1.0)*vec4(inv,inv);let c1=textureSample(tsl_texture_0,tsl_sampler_0,uv+o.xy);let c2=textureSample(tsl_texture_0,tsl_sampler_0,uv-o.xy);let c3=textureSample(tsl_texture_0,tsl_sampler_0,uv+o.yw);let c4=textureSample(tsl_texture_0,tsl_sampler_0,uv-o.yw);let d=length(vec2((c1.r-c2.r)*0.5,(c3.r-c4.r)*0.5));let visibility=min(min(c1.g,c2.g),min(c3.g,c4.g));let color=select(vec3(0.0,1.0,0.0),vec3(1.0,0.0,0.0),1.0-visibility>0.001);return vec4(color,1.0)*d;}";
/// separableBlur: u.custom[0].xy is the half-resolution texel size (both
/// blurs use the downsampled mask's size), [0].zw the direction, [1].x the
/// kernel radius (MAX_RADIUS 4).
const BLUR: &str = "fn outline_pdf(x:f32,sigma:f32)->f32{return 0.39894*exp(-0.5*x*x/(sigma*sigma))/sigma;}fn outline_blur()->vec4<f32>{let uv=outline_uv();let radius=u.custom[1].x;let sigma=radius/2.0;var weight_sum=outline_pdf(0.0,sigma);var sum=textureSample(tsl_texture_0,tsl_sampler_0,uv)*weight_sum;let delta=u.custom[0].zw*u.custom[0].xy*radius/4.0;var offset=delta;for(var i=1;i<=4;i++){let x=radius*f32(i)/4.0;let w=outline_pdf(x,sigma);sum+=(textureSample(tsl_texture_0,tsl_sampler_0,uv+offset)+textureSample(tsl_texture_0,tsl_sampler_0,uv-offset))*w;weight_sum+=w*2.0;offset+=delta;}return sum/weight_sum;}";
/// The composite: mask.r × ( edge1 + edge2 × edgeGlow ); u.custom[1].y is
/// edgeGlow. Textures: edge 1, edge 2, the mask.
const COMPOSITE: &str = "fn outline_composite()->vec4<f32>{let uv=outline_uv();let e1=textureSample(tsl_texture_0,tsl_sampler_0,uv);let e2=textureSample(tsl_texture_1,tsl_sampler_1,uv);let m=textureSample(tsl_texture_2,tsl_sampler_2,uv);return m.r*(e1+e2*u.custom[1].y);}";
/// The output: ( visibleEdge × visibleEdgeColor + hiddenEdge ×
/// hiddenEdgeColor ) × edgeStrength, pulsed, plus the scene pass. u.custom[0]
/// is (edgeStrength, pulsePeriod, time), [1] and [2] the edge colors.
/// Textures: the composite, the scene.
const OUTPUT: &str = "fn outline_output()->vec4<f32>{let p=vec2<i32>(fragment_surface.clip.xy);let o=textureLoad(tsl_texture_0,p,0);let scene=textureLoad(tsl_texture_1,p,0);var color=(o.r*u.custom[1]+o.g*u.custom[2])*u.custom[0].x;if u.custom[0].y>0.0 {let t=u.custom[0].z/u.custom[0].y*2.0;color*=(sin((t+0.75)*6.283185307179586)*0.5+0.5)*0.5+0.5;}return color+scene;}";
fn target8(r: &Renderer, w: u32, h: u32, depth: bool) -> Result<RenderTarget> {
    RenderTarget::with_options(
        &r.device,
        w.max(1),
        h.max(1),
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            depth_buffer: depth,
            ..Default::default()
        },
    )
}
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
struct Targets {
    scene: RenderTarget,
    depth: RenderTarget,
    mask: RenderTarget,
    down: RenderTarget,
    edge1: RenderTarget,
    blur1: RenderTarget,
    edge2: RenderTarget,
    blur2: RenderTarget,
    composite: RenderTarget,
}
/// The override renders: the same static meshes with the depth or the mask
/// material, visibility following the selection.
struct Mirror {
    scene: Scene,
    camera: Object3D,
    meshes: Vec<(Object3D, Object3D)>,
}
pub(super) struct Demo {
    group: Object3D,
    depth: Mirror,
    mask: Mirror,
    /// The masks' downsample, edge, blur (×4: half X, half Y, quarter X,
    /// quarter Y), composite and output passes.
    down: Pass,
    edge: Pass,
    blurs: [Pass; 4],
    composite: Pass,
    output: Pass,
    targets: Option<Targets>,
    linear: wgpu::Sampler,
    nearest: wgpu::Sampler,
    selected: Option<Object3D>,
    last_selection: bool,
    /// The mirrors have drawn every mesh once, so selections find their draw
    /// data resident.
    warmed: bool,
    pointer: Option<Vector2>,
    controls: Controls,
    time: f64,
    /// edgeStrength, edgeGlow, edgeThickness, pulsePeriod, visibleEdgeColor,
    /// hiddenEdgeColor (sRGB hex).
    params: [f64; 6],
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
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
        n.position = Vector3::new(0., 0., 8.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xaaaaaa),
            intensity: 0.6,
        }));
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::from_hex(0xddffdd),
            intensity: 2.,
        }));
        let n = s.get_mut(sun)?;
        n.position = Vector3::new(5., 5., 5.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            map_size: Some(2048),
            far: 25.,
            ..Default::default()
        };
        let group = s.insert(NodeKind::Group);
        let holder = s.insert(NodeKind::Group);
        s.add(group, holder)?;
        // OBJLoader's tree: each mesh centered, the object scaled by
        // 1 / ( 0.2 × radius ) and raised by 1.
        let tree = s.insert(NodeKind::Group);
        s.add(holder, tree)?;
        let mut scale = 1.;
        for object in parse_obj(&String::from_utf8_lossy(
            &fetch("/web/gallery/assets/outline/tree.obj").await?,
        )) {
            let mut g = BufferGeometry::default();
            g.set_attribute("position", vec3s(object.positions)?);
            g.set_attribute("normal", vec3s(object.normals)?);
            if !object.uvs.is_empty() {
                g.set_attribute(
                    "uv",
                    Attribute::F32(BufferAttribute::new(object.uvs, 2, false)?),
                );
            }
            g.center()?;
            scale = 0.2 * g.compute_bounding_sphere()?.radius;
            let mut m = MeshPhongMaterial {
                specular: Color::from_hex(0x111111),
                shininess: 5.,
                ..Default::default()
            };
            m.properties.color = Color::WHITE;
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(g),
                Arc::new(Material::Phong(m)),
            )));
            let n = s.get_mut(h)?;
            n.receive_shadow = true;
            n.cast_shadow = true;
            s.add(tree, h)?;
        }
        let n = s.get_mut(tree)?;
        n.position.y = 1.;
        n.scale = Vector3::splat(1. / scale);
        let sphere = Arc::new(SphereGeometry::build(3., 48, 24)?);
        let mut seed = 186u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.
        };
        for _ in 0..20 {
            let mut m = MeshLambertMaterial::default();
            m.properties.color = Color::from_hsl(random(), 1., 0.3);
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                sphere.clone(),
                Arc::new(Material::Lambert(m)),
            )));
            let n = s.get_mut(h)?;
            n.position = Vector3::new(random() * 4. - 2., random() * 4. - 2., random() * 4. - 2.);
            n.receive_shadow = true;
            n.cast_shadow = true;
            n.scale = Vector3::splat(random() * 0.3 + 0.1);
            s.add(group, h)?;
        }
        let mut floor = MeshLambertMaterial::default();
        floor.properties.side = Side::Double;
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(12., 12., 1, 1)?),
            Arc::new(Material::Lambert(floor)),
        )));
        let n = s.get_mut(h)?;
        n.quaternion = Quaternion::from_rotation_x(-PI * 0.5);
        n.position.y = -1.5;
        n.receive_shadow = true;
        s.add(group, h)?;
        let mut torus = MeshPhongMaterial::default();
        torus.properties.color = Color::from_hex(0xffaaff);
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusGeometry::build(
                1.,
                0.3,
                16,
                100,
                2. * PI,
                0.,
                2. * PI,
            )?),
            Arc::new(Material::Phong(torus)),
        )));
        let n = s.get_mut(h)?;
        n.position.z = -4.;
        n.receive_shadow = true;
        n.cast_shadow = true;
        s.add(group, h)?;
        s.update_world_matrix(group, false, true)?;
        // The override materials and their mirror scenes.
        let depth_material = Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(
                    WgslFn::new(
                        "outline_black",
                        "fn outline_black()->vec4<f32>{return vec4(0.0,0.0,0.0,1.0);}",
                        &[],
                        Type::Vec4,
                    )?
                    .call(&[]),
                )
                .wgsl(0)?,
                &[],
                &[],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
            )
            .await?,
        ))));
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let nearest = r.device.create_sampler(&wgpu::SamplerDescriptor::default());
        let comparison = r.device.create_sampler(&wgpu::SamplerDescriptor {
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let placeholder = target8(r, 1, 1, true)?;
        let color = placeholder.texture.create_view(&Default::default());
        let depth_view = placeholder
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("outline depth view"))?;
        let mask_material = Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection_and_sample_types(
                r,
                &NodeMaterial::new(WgslFn::new("outline_mask", MASK, &[], Type::Vec4)?.call(&[]))
                    .wgsl_with_texture_types(&[Type::DepthTexture], &[])?,
                &[(depth_view, &comparison)],
                &[wgpu::TextureViewDimension::D2],
                &[wgpu::TextureSampleType::Depth],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
            )
            .await?,
        ))));
        let meshes: Vec<Object3D> = s
            .traverse(group, false)?
            .into_iter()
            .filter(|&h| matches!(s.get(h).map(|n| &n.kind), Ok(NodeKind::Mesh(_))))
            .collect();
        let mirror = |material: &Arc<Material>, clear: Color| -> Result<Mirror> {
            let mut scene = Scene::new();
            scene.background = clear;
            let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
                PerspectiveCamera::default(),
            )));
            let mut pairs = vec![];
            for &h in &meshes {
                let n = s.get(h)?;
                let NodeKind::Mesh(m) = &n.kind else { continue };
                let copy = scene.insert(NodeKind::Mesh(Mesh::new(
                    m.geometry.clone(),
                    material.clone(),
                )));
                let (scale, rotation, translation) = n.matrix_world.to_scale_rotation_translation();
                let c = scene.get_mut(copy)?;
                c.position = translation;
                c.quaternion = rotation;
                c.scale = scale;
                c.frustum_culled = false;
                pairs.push((h, copy));
            }
            Ok(Mirror {
                scene,
                camera,
                meshes: pairs,
            })
        };
        // Both renders clear to ( 0xffffff, 1 ).
        let depth = mirror(&depth_material, Color::WHITE)?;
        let mask = mirror(&mask_material, Color::WHITE)?;
        let filtered = |view| {
            (
                view,
                &linear,
                Type::Texture,
                wgpu::TextureSampleType::Float { filterable: true },
            )
        };
        let down = pass(
            r,
            "outline_copy",
            &format!("{UV}{COPY}"),
            &[filtered(&color)],
        )
        .await?;
        let edge = pass(
            r,
            "outline_edge",
            &format!("{UV}{EDGE}"),
            &[filtered(&color)],
        )
        .await?;
        let mut blurs = vec![];
        for _ in 0..4 {
            blurs.push(
                pass(
                    r,
                    "outline_blur",
                    &format!("{UV}{BLUR}"),
                    &[filtered(&color)],
                )
                .await?,
            );
        }
        let composite = pass(
            r,
            "outline_composite",
            &format!("{UV}{COMPOSITE}"),
            &[filtered(&color), filtered(&color), filtered(&color)],
        )
        .await?;
        let output = pass(
            r,
            "outline_output",
            OUTPUT,
            &[
                (
                    &color,
                    &nearest,
                    Type::Texture,
                    wgpu::TextureSampleType::Float { filterable: true },
                ),
                (
                    &color,
                    &nearest,
                    Type::Texture,
                    wgpu::TextureSampleType::Float { filterable: true },
                ),
            ],
        )
        .await?;
        // OrbitControls: distances 5–20, no pan, damping 0.05.
        let mut controls = Controls::new(Some(0.05), (5., 20.), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            group,
            depth,
            mask,
            down,
            edge,
            blurs: blurs
                .try_into()
                .map_err(|_| Error::Invalid("outline blurs"))?,
            composite,
            output,
            targets: None,
            linear,
            nearest,
            selected: None,
            last_selection: false,
            warmed: false,
            pointer: None,
            controls,
            time: 0.,
            params: [3., 0., 1., 0., 0xffffff as f64, 0x4e3636 as f64],
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// The pointermove raycast (against the camera as last rendered), then
    /// animate()'s controls.update().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if let Some(p) = self.pointer.take() {
            let (w, h, _) = viewport_css();
            let ndc = Vector2::new(p.x / w * 2. - 1., -(p.y / h) * 2. + 1.);
            let (camera, world) = s.camera(c)?;
            let (camera, world) = (camera.clone(), world);
            let mut raycaster = Raycaster::default();
            raycaster.set_from_camera(ndc, &camera, world)?;
            let hits = raycaster.intersect_object(s, self.group, true)?;
            if let Some(hit) = hits.first() {
                self.selected = Some(hit.object);
            }
        }
        self.controls.update(s, c)
    }
    fn resize(&mut self, r: &Renderer, w: u32, h: u32) -> Result<()> {
        let (hw, hh) = (
            (w as f64 / 2.).round() as u32,
            (h as f64 / 2.).round() as u32,
        );
        let (qw, qh) = (
            (hw as f64 / 2.).round() as u32,
            (hh as f64 / 2.).round() as u32,
        );
        let t = Targets {
            scene: RenderTarget::with_options(
                &r.device,
                w,
                h,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    ..Default::default()
                },
            )?,
            depth: target8(r, w, h, true)?,
            mask: target8(r, w, h, true)?,
            down: target8(r, hw, hh, false)?,
            edge1: target8(r, hw, hh, false)?,
            blur1: target8(r, hw, hh, false)?,
            edge2: target8(r, qw, qh, false)?,
            blur2: target8(r, qw, qh, false)?,
            composite: target8(r, w, h, false)?,
        };
        let view = |t: &RenderTarget| t.texture.create_view(&Default::default());
        let rebind =
            |p: &mut Pass, textures: &[(&wgpu::TextureView, &wgpu::Sampler)]| -> Result<()> {
                Arc::make_mut(&mut p.material()?.program).rebind(r, &[], textures)
            };
        let (mask, down, edge1, blur1, edge2, blur2, composite, scene) = (
            view(&t.mask),
            view(&t.down),
            view(&t.edge1),
            view(&t.blur1),
            view(&t.edge2),
            view(&t.blur2),
            view(&t.composite),
            view(&t.scene),
        );
        let depth_view = t
            .depth
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("outline depth view"))?;
        let comparison = r.device.create_sampler(&wgpu::SamplerDescriptor {
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        for &(_, copy) in &self.mask.meshes {
            if let NodeKind::Mesh(m) = &mut self.mask.scene.get_mut(copy)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                Arc::make_mut(&mut m.program).rebind(r, &[], &[(depth_view, &comparison)])?;
            }
        }
        let l = &self.linear;
        rebind(&mut self.down, &[(&mask, l)])?;
        rebind(&mut self.edge, &[(&down, l)])?;
        rebind(&mut self.blurs[0], &[(&edge1, l)])?;
        rebind(&mut self.blurs[1], &[(&blur1, l)])?;
        rebind(&mut self.blurs[2], &[(&edge1, l)])?;
        rebind(&mut self.blurs[3], &[(&blur2, l)])?;
        rebind(&mut self.composite, &[(&edge1, l), (&edge2, l), (&mask, l)])?;
        rebind(
            &mut self.output,
            &[(&composite, &self.nearest), (&scene, &self.nearest)],
        )?;
        self.targets = Some(t);
        Ok(())
    }
    fn uniforms(size: &RenderTarget) -> [[f32; 4]; 16] {
        let mut u = [[0f32; 4]; 16];
        u[3] = [size.width as f32, size.height as f32, 0., 0.];
        u
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
            .is_none_or(|t| t.scene.width != out.width || t.scene.height != out.height)
        {
            self.resize(r, out.width, out.height)?;
        }
        let Some(t) = self.targets.take() else {
            return Err(Error::Invalid("outline targets"));
        };
        let result = self.passes(r, s, c, out, &t);
        self.targets = Some(t);
        result?;
        Ok(true)
    }
    fn passes(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        t: &Targets,
    ) -> Result<()> {
        r.render(s, c, &t.scene)?;
        let n = s.get(c)?;
        let (kind, position, quaternion) = (n.kind.clone(), n.position, n.quaternion);
        let (near, far) = match &kind {
            NodeKind::Camera(Camera::Perspective(p)) => (p.near, p.far),
            _ => return Err(Error::Invalid("outline camera")),
        };
        if !self.warmed {
            self.warmed = true;
            for mirror in [&mut self.depth, &mut self.mask] {
                let camera = mirror.scene.get_mut(mirror.camera)?;
                camera.kind = kind.clone();
                camera.position = position;
                camera.quaternion = quaternion;
                for &(_, copy) in &mirror.meshes {
                    mirror.scene.get_mut(copy)?.visible = true;
                }
            }
            r.render(&mut self.depth.scene, self.depth.camera, &t.depth)?;
            r.render(&mut self.mask.scene, self.mask.camera, &t.mask)?;
        }
        if let Some(selected) = self.selected {
            self.last_selection = true;
            for mirror in [&mut self.depth, &mut self.mask] {
                let camera = mirror.scene.get_mut(mirror.camera)?;
                camera.kind = kind.clone();
                camera.position = position;
                camera.quaternion = quaternion;
            }
            for &(original, copy) in &self.depth.meshes {
                self.depth.scene.get_mut(copy)?.visible = original != selected;
            }
            for &(original, copy) in &self.mask.meshes {
                self.mask.scene.get_mut(copy)?.visible = original == selected;
            }
            r.render(&mut self.depth.scene, self.depth.camera, &t.depth)?;
            let mut u = Self::uniforms(&t.mask);
            u[0] = [near as f32, far as f32, 0., 0.];
            for &(_, copy) in &self.mask.meshes {
                if let NodeKind::Mesh(m) = &mut self.mask.scene.get_mut(copy)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms = u;
                }
            }
            r.render(&mut self.mask.scene, self.mask.camera, &t.mask)?;
            self.down.render(r, &t.down, &Self::uniforms(&t.down))?;
            self.edge.render(r, &t.edge1, &Self::uniforms(&t.edge1))?;
            let texel = [1. / t.down.width as f32, 1. / t.down.height as f32];
            let thickness = self.params[2] as f32;
            for (i, (target, direction, radius)) in [
                (&t.blur1, [1., 0.], thickness),
                (&t.edge1, [0., 1.], thickness),
                (&t.blur2, [1., 0.], 4.),
                (&t.edge2, [0., 1.], 4.),
            ]
            .into_iter()
            .enumerate()
            {
                let mut u = Self::uniforms(target);
                u[0] = [texel[0], texel[1], direction[0], direction[1]];
                u[1][0] = radius;
                self.blurs[i].render(r, target, &u)?;
            }
            let mut u = Self::uniforms(&t.composite);
            u[1][1] = self.params[1] as f32;
            self.composite.render(r, &t.composite, &u)?;
        } else if self.last_selection {
            // An emptied selection clears the composite once.
            self.last_selection = false;
            self.composite.render(r, &t.composite, &[[0.; 4]; 16])?;
        }
        let mut u = [[0f32; 4]; 16];
        let color = |hex: f64| {
            Color::from_hex(hex as u32)
                .0
                .as_vec3()
                .extend(0.)
                .to_array()
        };
        u[0] = [
            self.params[0] as f32,
            self.params[3] as f32,
            self.time as f32,
            0.,
        ];
        u[1] = color(self.params[4]);
        u[2] = color(self.params[5]);
        self.output.render(r, out, &u)
    }
    /// pointermove: the raycast runs on the next frame.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if kind == 0 {
            self.pointer = Some(Vector2::new(x, y));
        }
    }
    pub fn key(&mut self, _code: u32, _down: bool) {}
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
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if !pan {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// edgeStrength, edgeGlow, edgeThickness, pulsePeriod and the two colors.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let p = self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("outline parameter"))?;
        *p = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
