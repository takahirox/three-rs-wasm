//! The mapped spotlight, and the Soldier and Xbot skinning examples: multiple
//! clones, blending, additive blending and the walking character.
use super::controls_attributes::{
    CameraState, Controls, camera_helper, camera_state, update_camera_helper,
};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use super::three_mixer::{ThreeMixer, make_additive, subclip};
use crate::compute::{BufferAccess, GpuBuffer};
use crate::shader::ShaderProgram;
use crate::tsl::surface::SurfaceNodes;
use crate::tsl::{self, NodeMaterial, Type, WgslFn, uniform};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
/// SkeletonHelper: a segment from each bone to its parent bone, child vertex
/// colored `color1`, parent vertex `color2`; the positions are rewritten on
/// the CPU each rendered frame, as the helper's geometry is.
struct SkeletonHelper {
    line: Object3D,
    root: Object3D,
    buffer: GpuBuffer,
    pairs: Vec<(Object3D, Object3D)>,
}
impl SkeletonHelper {
    async fn new(
        s: &mut Scene,
        r: &Renderer,
        root: Object3D,
        bones: &[Object3D],
        colors: [u32; 2],
    ) -> Result<Self> {
        let mut pairs = vec![];
        for &b in bones {
            if let Some(p) = s.get(b)?.parent()
                && bones.contains(&p)
            {
                pairs.push((b, p));
            }
        }
        let buffer = GpuBuffer::zeroed(r, (pairs.len() * 24).max(24) as u64, BufferAccess::Read)?;
        let projection = "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let i=tsl_vertex_index;let p=vec3(tsl_attribute_0[i*3u],tsl_attribute_0[i*3u+1u],tsl_attribute_0[i*3u+2u]);out.clip=u.projection*u.view*vec4(p,1.0);out.local_normal=vec3(select(0.0,1.0,i%2u==0u),0.0,0.0);return out;}";
        let color = crate::tsl::vec4(
            tsl::mix(
                uniform(1, Type::Vec4).rgb(),
                uniform(0, Type::Vec4).rgb(),
                tsl::normal_local().x(),
            ),
            tsl::float(1.),
        );
        let source = NodeMaterial::new(color).wgsl_with_storage(0, &[Type::Float])?;
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection(r, &source, &[&buffer], &[], projection).await?,
        ));
        for (i, hex) in colors.into_iter().enumerate() {
            let c = Color::from_hex(hex).0;
            m.uniforms[i] = [c.x as f32, c.y as f32, c.z as f32, 1.];
        }
        m.properties.depth_test = false;
        m.properties.depth_write = false;
        m.properties.transparent = true;
        m.properties.tone_mapped = false;
        m.properties.fog = false;
        let mut g = BufferGeometry::default();
        g.set_attribute("position", vec3s(vec![0.; pairs.len() * 6])?);
        let line = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Shader(m)),
            segments: true,
        }));
        let n = s.get_mut(line)?;
        n.frustum_culled = false;
        n.visible = false;
        Ok(Self {
            line,
            root,
            buffer,
            pairs,
        })
    }
    fn update(&self, s: &mut Scene, r: &Renderer) -> Result<()> {
        if !s.get(self.line)?.visible {
            return Ok(());
        }
        // updateMatrixWorld: the bones' world matrices after this frame's mixer.
        s.update_world_matrix(self.root, true, true)?;
        let mut data = Vec::with_capacity(self.pairs.len() * 6);
        for &(child, parent) in &self.pairs {
            for h in [child, parent] {
                let p = s.get(h)?.matrix_world.w_axis;
                data.extend([p.x as f32, p.y as f32, p.z as f32]);
            }
        }
        self.buffer.write(r, 0, bytemuck::cast_slice(&data))
    }
}
/// The Soldier or Xbot glTF instantiated under a group.
struct Model {
    group: Object3D,
    instance: crate::gltf::GltfInstance,
    bones: Vec<Object3D>,
}
async fn model(s: &mut Scene, path: &str) -> Result<Model> {
    let (a, b, i) = load_asset(path).await?;
    let bones: Vec<usize> = a
        .skins()
        .flat_map(|skin| skin.joints().map(|j| j.index()).collect::<Vec<_>>())
        .collect();
    let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
    let group = s.insert(NodeKind::Group);
    for &h in &instance.roots {
        s.add(group, h)?;
    }
    let mut unique = vec![];
    for j in bones {
        let h = instance.nodes[j];
        if !unique.contains(&h) {
            unique.push(h);
        }
    }
    Ok(Model {
        group,
        instance,
        bones: unique,
    })
}
fn find(s: &Scene, handles: &[Object3D], name: &str) -> Result<Object3D> {
    for &h in handles {
        if s.get(h)?.name == name {
            return Ok(h);
        }
    }
    Err(Error::Invalid("named glTF node"))
}
/// The shared scene of the Soldier examples: hemisphere and sun lights and the
/// shadow-receiving ground.
fn soldier_stage(s: &mut Scene, sun: Vector3, ground: f64) -> Result<()> {
    s.background = Color::from_hex(0xa0a0a0);
    s.fog = Some(Fog::Linear {
        color: Color::from_hex(0xa0a0a0),
        near: 10.,
        far: 50.,
    });
    let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
        sky: Color::WHITE,
        ground: Color::from_hex(0x8d8d8d),
        intensity: 3.,
    }));
    s.get_mut(hemi)?.position = Vector3::new(0., 20., 0.);
    let light = s.insert(NodeKind::Light(Light::Sun {
        color: Color::WHITE,
        intensity: 3.,
    }));
    let n = s.get_mut(light)?;
    n.position = sun;
    n.cast_shadow = true;
    // SunLightShadow: a 1024² map; camera.far = 20.
    n.shadow = crate::shadow::Shadow {
        map_size: Some(1024),
        far: 20.,
        ..Default::default()
    };
    let mut m = MeshPhongMaterial::default();
    m.properties.color = Color::from_hex(0xcbcbcb);
    m.properties.depth_write = false;
    let plane = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(ground, ground, 1, 1)?),
        Arc::new(Material::Phong(m)),
    )));
    let n = s.get_mut(plane)?;
    n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
    n.receive_shadow = true;
    Ok(())
}
/// webgpu_lights_spotlight.
struct Spot {
    light: Object3D,
    shadow_camera: Object3D,
    cone: Object3D,
    frustum: Object3D,
    receivers: [Object3D; 2],
    /// Per receiver: the plain material and one per map.
    materials: [Vec<Arc<Material>>; 2],
    /// map, color, intensity, distance, angle, penumbra, decay, focus,
    /// shadowIntensity, helpers.
    params: [f64; 10],
}
/// webgl_animation_multiple.
struct Multiple {
    default: Vec<(Object3D, crate::animation::AnimationMixer)>,
    shared: Vec<(Object3D, crate::animation::AnimationMixer)>,
    shared_mode: bool,
    start: f64,
}
/// webgl_animation_skinning_blending.
struct Blending {
    group: Object3D,
    helper: SkeletonHelper,
    mixer: ThreeMixer,
    /// idle, walk, run.
    actions: [usize; 3],
    single_step: bool,
    next_step: f64,
    sync: Vec<(usize, usize, f64)>,
    /// Deactivate/activate-all presses, applied with the scene before the next update.
    commands: Vec<usize>,
    /// show model, show skeleton, step size, use default duration, custom
    /// duration, time scale.
    settings: [f64; 6],
}
/// webgl_animation_skinning_additive_blending.
struct Additive {
    helper: SkeletonHelper,
    mixer: ThreeMixer,
    /// idle, walk, run.
    base: [usize; 3],
    /// sneak_pose, sad_pose, agree, headShake.
    additive: [usize; 4],
    /// None, idle, walk, run.
    current: usize,
    sync: Vec<(usize, usize, f64)>,
}
/// webgl_animation_walk.
struct Walk {
    group: Object3D,
    floor: Object3D,
    helper: SkeletonHelper,
    mixer: ThreeMixer,
    /// Idle, Walk, Run.
    actions: [usize; 3],
    current: usize,
    key: [f64; 3],
    position: Vector3,
    floor_decale: f64,
    /// show_skeleton, fixe_transition.
    settings: [f64; 2],
}
pub struct Demo {
    id: u32,
    time: f64,
    last: f64,
    controls: Option<Controls>,
    spot: Option<Spot>,
    multiple: Option<Multiple>,
    blending: Option<Blending>,
    additive: Option<Additive>,
    walk: Option<Walk>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            303 => (40., 0.1, 100., Vector3::new(7., 4., 1.)),
            304 => (45., 1., 1000., Vector3::new(2., 3., -6.)),
            305 => (45., 1., 100., Vector3::new(1., 2., -3.)),
            306 => (45., 1., 100., Vector3::new(-1., 2., 3.)),
            _ => (45., 0.1, 100., Vector3::new(0., 2., -5.)),
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
            controls: None,
            spot: None,
            multiple: None,
            blending: None,
            additive: None,
            walk: None,
        };
        match id {
            303 => d.spot_scene(s, c, r).await?,
            304 => d.multiple_scene(s, c).await?,
            305 => d.blending_scene(s, c, r).await?,
            306 => d.additive_scene(s, c, r).await?,
            _ => d.walk_scene(s, c, r).await?,
        }
        Ok(d)
    }
    async fn spot_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Neutral;
        let mut controls = Controls::new(None, (2., 10.), PI / 2., true);
        controls.set_target(Vector3::new(0., 1., 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        let mut maps = vec![];
        for name in ["disturb.jpg", "colors.png", "uv_grid_opengl.jpg"] {
            let mut t =
                decode_texture_image(&fetch(&format!("{ASSETS}/spot-skinning/{name}")).await?)
                    .await?;
            t.srgb = true;
            t.mipmap_filter = None;
            maps.push(r.upload_texture(&Arc::new(t))?);
        }
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x8d8d8d),
            intensity: 0.25,
        }));
        s.get_mut(hemi)?.position = Vector3::Y;
        let light = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 100.,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 2.,
            angle: PI / 6.,
            penumbra: 1.,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(2.5, 5., 2.5);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            map_size: Some(1024),
            near: 2.,
            far: 10.,
            ..Default::default()
        };
        // SpotLightHelper's cone and the shadow camera's CameraHelper, hidden.
        let mut positions = vec![
            0., 0., 0., 0., 0., 1., 0., 0., 0., 1., 0., 1., 0., 0., 0., -1., 0., 1., 0., 0., 0.,
            0., 1., 1., 0., 0., 0., 0., -1., 1.,
        ];
        for i in 0..32 {
            let (p1, p2) = (i as f64 / 32. * 2. * PI, (i + 1) as f64 / 32. * 2. * PI);
            positions.extend([p1.cos(), p1.sin(), 1., p2.cos(), p2.sin(), 1.]);
        }
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            vec3s(positions.into_iter().map(|v| v as f32).collect())?,
        );
        let mut m = LineBasicMaterial::default();
        m.properties.fog = false;
        m.properties.tone_mapped = false;
        let cone = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Line(m)),
            segments: true,
        }));
        s.get_mut(cone)?.visible = false;
        let shadow_camera = s.insert(NodeKind::Camera(Camera::Perspective(
            PerspectiveCamera::default(),
        )));
        let (geometry, program) = camera_helper(r).await?;
        let mut m = ShaderMaterial::new(program);
        m.properties.tone_mapped = false;
        m.properties.fog = false;
        let frustum = s.insert(NodeKind::Line(Line {
            geometry,
            material: Arc::new(Material::Shader(m)),
            segments: true,
        }));
        let n = s.get_mut(frustum)?;
        n.visible = false;
        n.frustum_culled = false;
        // The receivers: MeshLambertMaterial whose spotlight color takes the map
        // at lightProjectionUV, as SpotLightNode does. With the shadow on, the
        // r186 WebGPU reference leaves no spotlight outside the projected map
        // as a shadow of 0 there, weighted by the shadow intensity (measured: lit
        // without the map or without the shadow).
        let hook = |view: &wgpu::TextureView, sampler: &wgpu::Sampler| {
            let view = view.clone();
            let sampler = sampler.clone();
            async move {
                let node = WgslFn::new(
                    "spot_map",
                    "fn spot_map(index:u32,color:vec3<f32>,world:vec3<f32>,m0:vec4<f32>,m1:vec4<f32>,m2:vec4<f32>,m3:vec4<f32>,outside:f32)->vec3<f32>{
 if index!=1u {return color;}
 let c=mat4x4(m0,m1,m2,m3)*vec4(world,1.0);let uvw=c.xyz/c.w;
 if all(abs(uvw*2.0-1.0)<vec3(1.0)) {return color*textureSampleLevel(tsl_texture_0,tsl_sampler_0,vec2(uvw.x,1.0-uvw.y),0.0).rgb;}
 return color*outside;
}",
                    &[Type::Uint, Type::Vec3, Type::Vec3, Type::Vec4, Type::Vec4, Type::Vec4, Type::Vec4, Type::Float],
                    Type::Vec3,
                )?
                .call(&[
                    tsl::light_index(),
                    tsl::light_color(),
                    tsl::position_world(),
                    uniform(0, Type::Vec4),
                    uniform(1, Type::Vec4),
                    uniform(2, Type::Vec4),
                    uniform(3, Type::Vec4),
                    uniform(4, Type::Vec4).x(),
                ]);
                let program = SurfaceNodes {
                    light_color: Some(node),
                    ..Default::default()
                }
                .build(r, &[], &[(&view, &sampler)])
                .await?;
                Ok::<_, Error>(Arc::new(program))
            }
        };
        // The map changes only light color: casters keep the plain depth pass.
        let shadow =
            Arc::new(crate::shadow::ShadowProgram::new(r, crate::shader::DEFAULT_HOOKS).await?);
        let mut programs = vec![];
        for map in &maps {
            programs.push(hook(&map.view, &map.sampler).await?);
        }
        let lambert = |hex: u32, program: Option<&Arc<ShaderProgram>>| {
            let mut m = MeshLambertMaterial::default();
            m.properties.color = Color::from_hex(hex);
            m.properties.vertex_program = program.cloned();
            if program.is_some() {
                m.properties.shadow_program = Some(shadow.clone());
            }
            Arc::new(Material::Lambert(m))
        };
        let floor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10., 10., 1, 1)?),
            lambert(0xbcbcbc, Some(&programs[0])),
        )));
        let n = s.get_mut(floor)?;
        n.position.y = -1.;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.receive_shadow = true;
        let (positions, index) = super::refraction_loaders::formats::parse_ply(
            &fetch(&format!("{ASSETS}/ply/binary/Lucy100k.ply")).await?,
        )?;
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            vec3s(positions.iter().map(|v| v * 0.0024).collect())?,
        );
        g.set_index(Some(index));
        g.compute_vertex_normals()?;
        let lucy = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            lambert(0xffffff, Some(&programs[0])),
        )));
        let n = s.get_mut(lucy)?;
        n.quaternion = Quaternion::from_rotation_y(-PI / 2.);
        n.position.y = 0.8;
        n.cast_shadow = true;
        n.receive_shadow = true;
        let materials = [0xbcbcbc, 0xffffff].map(|hex| {
            std::iter::once(lambert(hex, None))
                .chain(programs.iter().map(|p| lambert(hex, Some(p))))
                .collect::<Vec<_>>()
        });
        self.spot = Some(Spot {
            light,
            shadow_camera,
            cone,
            frustum,
            receivers: [floor, lucy],
            materials,
            params: [1., 16777215., 100., 0., PI / 6., 1., 2., 1., 1., 0.],
        });
        Ok(())
    }
    async fn multiple_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.look_at(c, Vector3::new(0., 1., 0.))?;
        soldier_stage(s, Vector3::new(-3., 10., -10.), 200.)?;
        let path = format!("{ASSETS}/spot-skinning/Soldier.glb");
        let mut default = vec![];
        let mut shared = vec![];
        // setupDefaultScene(): three SkeletonUtils clones playing idle, run and walk.
        for (x, clip) in [(-2., 0), (0., 1), (2., 3)] {
            let m = model(s, &path).await?;
            for &h in &m.instance.meshes {
                s.get_mut(h)?.cast_shadow = true;
            }
            s.get_mut(m.group)?.position.x = x;
            let mut mixer = crate::animation::AnimationMixer::default();
            mixer.play(m.instance.clips[clip].clone())?;
            default.push((m.group, mixer));
        }
        // setupSharedSkeletonScene(): the vanguard mesh bound to the shared hips at
        // ( x, 0, 0 ), scale 0.01, rotation.x −π/2, in DetachedBindMode. Each copy
        // keeps a skeleton under that transform, driven by the same clip and time.
        for x in [-2., 0., 2.] {
            let m = model(s, &path).await?;
            let group = s.insert(NodeKind::Group);
            let n = s.get_mut(group)?;
            n.position.x = x;
            n.scale = Vector3::splat(0.01);
            n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
            let hips = find(s, &m.instance.nodes, "mixamorig:Hips")?;
            let mesh = find(s, &m.instance.meshes, "vanguard_Mesh")?;
            s.get_mut(mesh)?.cast_shadow = true;
            s.add(group, hips)?;
            s.add(group, mesh)?;
            s.get_mut(m.group)?.visible = false;
            s.get_mut(group)?.visible = false;
            let mut mixer = crate::animation::AnimationMixer::default();
            mixer.play(m.instance.clips[1].clone())?;
            shared.push((group, mixer));
        }
        self.multiple = Some(Multiple {
            default,
            shared,
            shared_mode: false,
            start: 0.,
        });
        Ok(())
    }
    async fn blending_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.look_at(c, Vector3::new(0., 1., 0.))?;
        soldier_stage(s, Vector3::new(-3., 10., -10.), 100.)?;
        let m = model(s, &format!("{ASSETS}/spot-skinning/Soldier.glb")).await?;
        for &h in &m.instance.meshes {
            s.get_mut(h)?.cast_shadow = true;
        }
        let helper = SkeletonHelper::new(s, r, m.group, &m.bones, [0x0000ff, 0x00ff00]).await?;
        let mut mixer = ThreeMixer::new();
        let idle = mixer.clip_action(s, m.instance.clips[0].clone(), false)?;
        let walk = mixer.clip_action(s, m.instance.clips[3].clone(), false)?;
        let run = mixer.clip_action(s, m.instance.clips[1].clone(), false)?;
        let mut b = Blending {
            group: m.group,
            helper,
            mixer,
            actions: [idle, walk, run],
            single_step: false,
            next_step: 0.,
            sync: vec![],
            commands: vec![],
            settings: [1., 0., 0.05, 1., 3.5, 1.],
        };
        b.activate_all(s, [0., 1., 0.])?;
        self.blending = Some(b);
        Ok(())
    }
    async fn additive_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        soldier_stage(s, Vector3::new(3., 10., 10.), 100.)?;
        let m = model(s, &format!("{ASSETS}/tsl-procedural/Xbot.glb")).await?;
        for &h in &m.instance.meshes {
            s.get_mut(h)?.cast_shadow = true;
        }
        let helper = SkeletonHelper::new(s, r, m.group, &m.bones, [0x0000ff, 0x00ff00]).await?;
        let mut mixer = ThreeMixer::new();
        // The clip actions in gltf.animations order: their activation order sets
        // the (non-commutative) order of additive accumulation.
        let (mut base, mut additive) = ([0; 3], [0; 4]);
        let base_names = ["idle", "walk", "run"];
        let additive_names = ["sneak_pose", "sad_pose", "agree", "headShake"];
        for source in m.instance.clips.clone() {
            let name = source.name.clone();
            if let Some(i) = base_names.iter().position(|n| *n == name) {
                base[i] = mixer.clip_action(s, source, false)?;
                set_weight(&mut mixer, base[i], if i == 0 { 1. } else { 0. });
                mixer.play(s, base[i])?;
            } else if let Some(i) = additive_names.iter().position(|n| *n == name) {
                // makeClipAdditive, then the poses keep frames [ 2, 3 ) at 30 fps.
                let mut c = make_additive(&source);
                if name.ends_with("_pose") {
                    c = subclip(&c, 2., 3., 30.);
                }
                additive[i] = mixer.clip_action(s, Arc::new(c), true)?;
                set_weight(&mut mixer, additive[i], 0.);
                mixer.play(s, additive[i])?;
            }
        }
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 1., 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.additive = Some(Additive {
            helper,
            mixer,
            base,
            additive,
            current: 1,
            sync: vec![],
        });
        Ok(())
    }
    async fn walk_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0x5e5d5d);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x5e5d5d),
            near: 2.,
            far: 20.,
        });
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 0.5;
        let group = s.insert(NodeKind::Group);
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 5.,
        }));
        let n = s.get_mut(sun)?;
        n.position = Vector3::new(-2., 5., -3.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            map_size: Some(1024),
            far: 20.,
            ..Default::default()
        };
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI / 2. - 0.05, true);
        controls.set_target(Vector3::new(0., 1., 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        let mut env = crate::environment::EnvironmentMap::from_hdr(
            &fetch(&format!("{ASSETS}/spot-skinning/lobe.hdr")).await?,
        )?;
        env.prefilter(r)?;
        s.environment = Some(Arc::new(env));
        s.environment_intensity = 1.5;
        // addFloor(): 16 repeats, anisotropic maps, no depth writes.
        let texture = |name: &str, srgb: bool| {
            let name = name.to_owned();
            async move {
                let mut t =
                    decode_texture_image(&fetch(&format!("{ASSETS}/tsl-procedural/{name}")).await?)
                        .await?;
                t.srgb = srgb;
                t.mipmap_filter = Some(Filter::Linear);
                t.wrap_s = Wrapping::Repeat;
                t.wrap_t = Wrapping::Repeat;
                t.repeat = Vector2::splat(16.);
                t.anisotropy = 16;
                Ok::<_, Error>(Arc::new(t))
            }
        };
        let mut m = MeshStandardMaterial {
            roughness: 0.85,
            normal_map: Some(texture("FloorsCheckerboard_S_Normal.jpg", false).await?),
            normal_scale: Vector2::splat(0.5),
            ..Default::default()
        };
        m.properties.map = Some(texture("FloorsCheckerboard_S_Diffuse.jpg", true).await?);
        m.properties.color = Color::from_hex(0x404040);
        m.properties.depth_write = false;
        let mut g = PlaneGeometry::build(50., 50., 50, 50)?;
        g.rotate_x(-PI / 2.)?;
        let floor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(g),
            Arc::new(Material::Standard(m)),
        )));
        s.get_mut(floor)?.receive_shadow = true;
        let bulb = s.insert(NodeKind::Light(Light::Point {
            color: Color::from_hex(0xffee88),
            intensity: 2.,
            distance: 500.,
            decay: 2.,
        }));
        let n = s.get_mut(bulb)?;
        n.position = Vector3::new(1., 0.1, -3.);
        n.cast_shadow = true;
        let mut m = MeshStandardMaterial {
            emissive: Color::from_hex(0xffffee),
            ..Default::default()
        };
        m.properties.color = Color::BLACK;
        let sphere = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.05, 16, 8)?),
            Arc::new(Material::Standard(m)),
        )));
        s.add(bulb, sphere)?;
        s.add(floor, bulb)?;
        let m = model(s, &format!("{ASSETS}/spot-skinning/Soldier.glb")).await?;
        // model.rotation.y = π inside group.rotation.y = π.
        s.get_mut(m.group)?.quaternion = Quaternion::from_rotation_y(PI);
        s.get_mut(group)?.quaternion = Quaternion::from_rotation_y(PI);
        s.add(group, m.group)?;
        for &h in &m.instance.meshes {
            let n = s.get_mut(h)?;
            let body = n.name == "vanguard_Mesh";
            if body {
                n.cast_shadow = true;
                n.receive_shadow = true;
            }
            if let NodeKind::Mesh(mesh) = &mut n.kind {
                for material in &mut mesh.materials {
                    if let Material::Standard(m) = Arc::make_mut(material) {
                        m.properties.color = Color::WHITE;
                        m.metalness = 1.;
                        if body {
                            m.roughness = 0.2;
                        } else {
                            m.roughness = 0.;
                            m.properties.transparent = true;
                            m.properties.opacity = 0.8;
                        }
                    }
                }
            }
        }
        // metalnessMap = map: the body's metalness takes the map's blue channel.
        let body = find(s, &m.instance.meshes, "vanguard_Mesh")?;
        if let NodeKind::Mesh(mesh) = &mut s.get_mut(body)?.kind
            && let Material::Standard(sm) = Arc::make_mut(&mut mesh.materials[0])
            && let Some(map) = sm.properties.map.clone()
        {
            let gpu = r.upload_texture(&map)?;
            let program = SurfaceNodes {
                metalness: Some(
                    tsl::Texture::External(0)
                        .sample(crate::tsl::uv())
                        .swizzle("z"),
                ),
                ..Default::default()
            }
            .build(r, &[], &[(&gpu.view, &gpu.sampler)])
            .await?;
            sm.properties.vertex_program = Some(Arc::new(program));
            sm.properties.shadow_program = Some(Arc::new(
                crate::shadow::ShadowProgram::new(r, crate::shader::DEFAULT_HOOKS).await?,
            ));
        }
        let helper = SkeletonHelper::new(s, r, m.group, &m.bones, [0xe000ff, 0x00e0ff]).await?;
        let mut mixer = ThreeMixer::new();
        let idle = mixer.clip_action(s, m.instance.clips[0].clone(), false)?;
        let walk = mixer.clip_action(s, m.instance.clips[3].clone(), false)?;
        let run = mixer.clip_action(s, m.instance.clips[1].clone(), false)?;
        for &a in &[walk, run] {
            mixer.actions[a].enabled = true;
            mixer.set_effective_time_scale(a, 1.);
            mixer.set_effective_weight(a, 0.);
        }
        mixer.actions[idle].enabled = true;
        mixer.set_effective_time_scale(idle, 1.);
        mixer.play(s, idle)?;
        self.walk = Some(Walk {
            group,
            floor,
            helper,
            mixer,
            actions: [idle, walk, run],
            current: 0,
            key: [0.; 3],
            position: Vector3::ZERO,
            floor_decale: 50. / 16. * 4.,
            settings: [0., 1.],
        });
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let t = self.time;
        let delta = t - self.last;
        self.last = t;
        match self.id {
            303 => self.spot_step(s, t)?,
            304 => {
                let k = self.multiple.as_mut().ok_or(Error::Invalid("multiple"))?;
                let time = t - k.start;
                for (i, (group, mixer)) in
                    k.default.iter_mut().chain(k.shared.iter_mut()).enumerate()
                {
                    let shown = (i >= 3) == k.shared_mode;
                    s.get_mut(*group)?.visible = shown;
                    if shown {
                        for a in &mut mixer.actions {
                            a.time = time;
                        }
                        mixer.update(s, 0.)?;
                    }
                }
            }
            305 => {
                let k = self.blending.as_mut().ok_or(Error::Invalid("blending"))?;
                for command in std::mem::take(&mut k.commands) {
                    k.scene_control(s, command, 0.)?;
                }
                let mut delta = delta;
                if k.single_step {
                    delta = k.next_step;
                    k.next_step = 0.;
                }
                k.mixer.time_scale = k.settings[5];
                k.mixer.update_synced(s, delta, &mut k.sync)?;
                s.get_mut(k.group)?.visible = k.settings[0] > 0.5;
                s.get_mut(k.helper.line)?.visible = k.settings[1] > 0.5;
                k.helper.update(s, r)?;
            }
            306 => {
                let k = self.additive.as_mut().ok_or(Error::Invalid("additive"))?;
                k.mixer.update_synced(s, delta, &mut k.sync)?;
                k.helper.update(s, r)?;
            }
            307 => self.walk_step(s, c, r, delta)?,
            _ => {}
        }
        Ok(())
    }
    fn spot_step(&mut self, s: &mut Scene, t: f64) -> Result<()> {
        let k = self.spot.as_mut().ok_or(Error::Invalid("spot"))?;
        // animate(): performance.now() / 3000.
        let time = t * 1000. / 3000.;
        let p = k.params;
        let position = Vector3::new(time.cos() * 2.5, 5., time.sin() * 2.5);
        let n = s.get_mut(k.light)?;
        n.position = position;
        n.shadow.focus = p[7];
        n.shadow.intensity = p[8];
        if let NodeKind::Light(Light::Spot {
            color,
            intensity,
            distance,
            angle,
            penumbra,
            decay,
            ..
        }) = &mut n.kind
        {
            *color = Color::from_hex(p[1] as u32);
            *intensity = p[2];
            *distance = p[3];
            *angle = p[4];
            *penumbra = p[5];
            *decay = p[6];
        }
        let far = if p[3] > 0. { p[3] } else { 10. };
        // SpotLightShadow.updateMatrices: fov 2 × angle × focus, aspect 1.
        let camera = PerspectiveCamera {
            fov: (2. * p[4] * p[7]).to_degrees(),
            aspect: 1.,
            near: 2.,
            far,
            ..Default::default()
        };
        let view = Matrix4::look_at_rh(position, Vector3::ZERO, Vector3::Y);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let matrix = bias * camera.projection_matrix()? * view;
        let columns = matrix.to_cols_array();
        for h in k.receivers {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                let properties = Arc::make_mut(&mut m.materials[0]).properties_mut();
                for (i, column) in columns.chunks(4).enumerate() {
                    properties.vertex_uniforms[i] = std::array::from_fn(|j| column[j] as f32);
                }
                properties.vertex_uniforms[4] = [(1. - p[8]) as f32, 0., 0., 0.];
            }
        }
        // spotLight.map: none or one of the three textures.
        let map = p[0] as usize;
        for (i, h) in k.receivers.into_iter().enumerate() {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                let uniforms = m.materials[0].properties().vertex_uniforms;
                let chosen = &k.materials[i][map.min(3)];
                if !Arc::ptr_eq(&m.materials[0], chosen) {
                    m.materials[0] = chosen.clone();
                }
                if m.materials[0].properties().vertex_uniforms != uniforms {
                    Arc::make_mut(&mut m.materials[0])
                        .properties_mut()
                        .vertex_uniforms = uniforms;
                }
            }
        }
        // lightHelper.update() and the shadow camera helper.
        let shown = p[9] > 0.5;
        let length = if p[3] > 0. { p[3] } else { 1000. };
        let width = length * p[4].tan();
        let direction = (Vector3::ZERO - position).normalize();
        let color = Color::from_hex(p[1] as u32);
        let n = s.get_mut(k.cone)?;
        n.visible = shown;
        n.position = position;
        // cone.lookAt( target ): +z toward the target with the up (0, 1, 0) roll.
        let x = Vector3::Y.cross(direction).normalize();
        n.quaternion = Quaternion::from_mat3(&Matrix3::from_cols(x, direction.cross(x), direction));
        n.scale = Vector3::new(width, width, length);
        if let NodeKind::Line(l) = &mut n.kind
            && let Material::Line(m) = Arc::make_mut(&mut l.material)
            && m.properties.color != color
        {
            m.properties.color = color;
        }
        let n = s.get_mut(k.shadow_camera)?;
        // CameraHelper keeps the lines computed before the shadow camera's first
        // update: fov 50 with the example's near 2 and far 10 (measured); it
        // follows only the camera's world matrix.
        n.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            aspect: 1.,
            near: 2.,
            far: 10.,
            ..Default::default()
        }));
        n.position = position;
        s.look_at(k.shadow_camera, Vector3::ZERO)?;
        s.update_world_matrix(k.shadow_camera, true, false)?;
        s.get_mut(k.frustum)?.visible = shown;
        if shown {
            update_camera_helper(s, k.shadow_camera, k.frustum)?;
        }
        Ok(())
    }
    fn walk_step(&mut self, s: &mut Scene, c: Object3D, r: &Renderer, delta: f64) -> Result<()> {
        let k = self.walk.as_mut().ok_or(Error::Invalid("walk"))?;
        let controls = self.controls.as_mut().ok_or(Error::Invalid("controls"))?;
        let fade = 0.5;
        let key = k.key;
        // getAzimuthalAngle(): the camera offset's angle around +y.
        let target = s.get(c)?.position - controls_target(controls, s, c)?;
        let azimuth = target.x.atan2(target.z);
        let active = key[0] != 0. || key[1] != 0.;
        let play = if active {
            if key[2] > 0. { 2 } else { 1 }
        } else {
            0
        };
        if k.current != play {
            let (current, old) = (k.actions[play], k.actions[k.current]);
            k.current = play;
            let m = &mut k.mixer;
            if k.settings[1] > 0.5 {
                m.reset(current);
                m.actions[current].weight = 1.;
                m.stop_fading(current);
                m.stop_fading(old);
                if play != 0 {
                    m.actions[current].time = m.actions[old].time
                        * (m.actions[current].clip.duration() / m.actions[old].clip.duration());
                }
                let (wo, wc) = (m.effective_weight(old), m.effective_weight(current));
                m.schedule_fading(old, fade, wo, 0.);
                m.schedule_fading(current, fade, wc, 1.);
                m.play(s, current)?;
            } else {
                set_weight(m, current, 1.);
                m.fade_out(old, fade);
                m.reset(current);
                m.fade_in(current, fade);
                m.play(s, current)?;
            }
        }
        if k.current != 0 {
            let velocity = if k.current == 2 { 5. } else { 1.8 };
            let mut ease = Vector3::new(key[1], 0., key[0]) * (velocity * delta);
            let angle = {
                let a = ease.x.atan2(ease.z) + azimuth;
                a.sin().atan2(a.cos())
            };
            let rotate = Quaternion::from_axis_angle(Vector3::Y, angle);
            ease = Quaternion::from_axis_angle(Vector3::Y, azimuth) * ease;
            k.position += ease;
            s.get_mut(c)?.position += ease;
            let n = s.get_mut(k.group)?;
            n.position = k.position;
            // Quaternion.rotateTowards( rotate, 0.05 ).
            let q = n.quaternion;
            let between = 2. * q.dot(rotate).abs().min(1.).acos();
            if between != 0. {
                let t = (0.05 / between).min(1.);
                n.quaternion = slerp(q, rotate, t);
            }
            controls.set_target(k.position + Vector3::Y);
            let floor = s.get_mut(k.floor)?;
            let (dx, dz) = (
                k.position.x - floor.position.x,
                k.position.z - floor.position.z,
            );
            if dx.abs() > k.floor_decale {
                floor.position.x += dx;
            }
            if dz.abs() > k.floor_decale {
                floor.position.z += dz;
            }
        }
        k.mixer.update(s, delta)?;
        controls.update(s, c)?;
        s.get_mut(k.helper.line)?.visible = k.settings[0] > 0.5;
        k.helper.update(s, r)
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(k) = &mut self.walk {
            let key = &mut k.key;
            match (code, down) {
                (38 | 87 | 90, true) => key[0] = -1.,
                (40 | 83, true) => key[0] = 1.,
                (37 | 65 | 81, true) => key[1] = -1.,
                (39 | 68, true) => key[1] = 1.,
                (16, true) => key[2] = 1.,
                (38 | 87 | 90, false) => key[0] = if key[0] < 0. { 0. } else { key[0] },
                (40 | 83, false) => key[0] = if key[0] > 0. { 0. } else { key[0] },
                (37 | 65 | 81, false) => key[1] = if key[1] < 0. { 0. } else { key[1] },
                (39 | 68, false) => key[1] = if key[1] > 0. { 0. } else { key[1] },
                (16, false) => key[2] = 0.,
                _ => {}
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
        let Some(controls) = &mut self.controls else {
            return Ok(());
        };
        let camera: CameraState = camera_state(s, c)?;
        if wheel != 0. {
            // The additive example disables zoom.
            if self.id == 306 {
                return Ok(());
            }
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            if matches!(self.id, 306 | 307) {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        // The walk's damped controls update in its animation loop.
        if self.id == 307 {
            return Ok(());
        }
        controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        match self.id {
            303 if index < 10 => {
                self.spot.as_mut().ok_or(Error::Invalid("spot"))?.params[index] = v
            }
            304 if index == 0 => {
                let k = self.multiple.as_mut().ok_or(Error::Invalid("multiple"))?;
                // clearScene(), then the other setup with new mixers.
                if (v > 0.5) != k.shared_mode {
                    k.shared_mode = v > 0.5;
                    k.start = self.time;
                }
            }
            305 => {
                let k = self.blending.as_mut().ok_or(Error::Invalid("blending"))?;
                k.control(index, v)?;
            }
            306 => {
                let k = self.additive.as_mut().ok_or(Error::Invalid("additive"))?;
                k.control(index, v)?;
            }
            307 if index < 2 => {
                self.walk.as_mut().ok_or(Error::Invalid("walk"))?.settings[index] = v
            }
            _ => return Err(Error::Invalid("spot/skinning parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
fn controls_target(controls: &Controls, s: &Scene, c: Object3D) -> Result<Vector3> {
    let _ = (s, c);
    Ok(controls.target())
}
fn slerp(a: Quaternion, b: Quaternion, t: f64) -> Quaternion {
    let r = super::three_mixer::slerp_quaternion(a, b, t);
    Quaternion::from_xyzw(r[0], r[1], r[2], r[3])
}
/// setWeight(): enable, effective time scale 1, effective weight.
fn set_weight(m: &mut ThreeMixer, a: usize, weight: f64) {
    m.actions[a].enabled = true;
    m.set_effective_time_scale(a, 1.);
    m.set_effective_weight(a, weight);
}
impl Blending {
    fn activate_all(&mut self, s: &Scene, weights: [f64; 3]) -> Result<()> {
        for (i, &a) in self.actions.iter().enumerate() {
            set_weight(&mut self.mixer, a, weights[i]);
        }
        for &a in &self.actions {
            self.mixer.play(s, a)?;
        }
        Ok(())
    }
    /// The cross-fade buttons are enabled by the effective weights, as
    /// updateCrossFadeControls() does each frame.
    fn enabled(&self, button: usize) -> bool {
        let w = self.actions.map(|a| self.mixer.effective_weight(a));
        match button {
            0 => w == [0., 1., 0.],
            1 => w == [1., 0., 0.],
            2 => w == [0., 1., 0.],
            _ => w == [0., 0., 1.],
        }
    }
    fn prepare_cross_fade(&mut self, start: usize, end: usize, default: f64) {
        let duration = if self.settings[3] > 0.5 {
            default
        } else {
            self.settings[4]
        };
        self.single_step = false;
        for &a in &self.actions {
            self.mixer.actions[a].paused = false;
        }
        let (start, end) = (self.actions[start], self.actions[end]);
        if start == self.actions[0] {
            execute_cross_fade(&mut self.mixer, start, end, duration);
        } else {
            self.sync.push((start, end, duration));
        }
    }
    fn control(&mut self, index: usize, v: f64) -> Result<()> {
        match index {
            0 => self.settings[0] = v,
            1 => self.settings[1] = v,
            2 | 3 => self.commands.push(index),
            4 => {
                // pauseContinue().
                if self.single_step {
                    self.single_step = false;
                    self.unpause();
                } else if self.mixer.actions[self.actions[0]].paused {
                    self.unpause();
                } else {
                    for &a in &self.actions {
                        self.mixer.actions[a].paused = true;
                    }
                }
            }
            5 => {
                self.unpause();
                self.single_step = true;
                self.next_step = self.settings[2];
            }
            6 => self.settings[2] = v,
            7..=10 => {
                if self.enabled(index - 7) {
                    let (start, end, duration) =
                        [(1, 0, 1.), (0, 1, 0.5), (1, 2, 2.5), (2, 1, 5.)][index - 7];
                    self.prepare_cross_fade(start, end, duration);
                }
            }
            11 => self.settings[3] = v,
            12 => self.settings[4] = v,
            13..=15 => set_weight(&mut self.mixer, self.actions[index - 13], v),
            16 => self.settings[5] = v,
            _ => return Err(Error::Invalid("blending control")),
        }
        Ok(())
    }
    fn scene_control(&mut self, s: &mut Scene, index: usize, v: f64) -> Result<()> {
        match index {
            // deactivateAllActions(): stop.
            2 => {
                for i in 0..3 {
                    self.mixer.stop(s, self.actions[i])?;
                }
                Ok(())
            }
            // activateAllActions(): the sliders' weights, then play.
            3 => {
                let w = self.actions.map(|a| self.mixer.effective_weight(a));
                self.activate_all(s, w)
            }
            _ => self.control(index, v),
        }
    }
    fn unpause(&mut self) {
        for &a in &self.actions {
            self.mixer.actions[a].paused = false;
        }
    }
}
/// executeCrossFade(): the end action at full weight from time 0, warped.
fn execute_cross_fade(m: &mut ThreeMixer, start: usize, end: usize, duration: f64) {
    set_weight(m, end, 1.);
    m.actions[end].time = 0.;
    m.cross_fade(start, end, duration, true);
}
impl Additive {
    fn control(&mut self, index: usize, v: f64) -> Result<()> {
        match index {
            // None, idle, walk, run: prepareCrossFade( current, chosen, 0.35 ).
            0..=3 => {
                let action = |i: usize| (i > 0).then(|| self.base[i - 1]);
                let (start, end) = (action(self.current), action(index));
                if start != end {
                    let m = &mut self.mixer;
                    if self.current == 1 || start.is_none() || end.is_none() {
                        match (start, end) {
                            (Some(a), Some(b)) => execute_cross_fade(m, a, b, 0.35),
                            (None, Some(b)) => {
                                set_weight(m, b, 1.);
                                m.actions[b].time = 0.;
                                m.fade_in(b, 0.35);
                            }
                            (Some(a), None) => m.fade_out(a, 0.35),
                            _ => {}
                        }
                    } else if let (Some(a), Some(b)) = (start, end) {
                        self.sync.push((a, b, 0.35));
                    }
                    self.current = index;
                }
            }
            4..=7 => set_weight(&mut self.mixer, self.additive[index - 4], v),
            8 => self.mixer.time_scale = v,
            _ => return Err(Error::Invalid("additive control")),
        }
        Ok(())
    }
}
impl ThreeMixer {
    /// `update()` with the examples' synchronizeCrossFade listeners, which
    /// cross-fade when their start action loops.
    pub(super) fn update_synced(
        &mut self,
        s: &mut Scene,
        delta: f64,
        sync: &mut Vec<(usize, usize, f64)>,
    ) -> Result<()> {
        let pending = std::mem::take(sync);
        let mut remaining = pending.clone();
        self.update_with(s, delta, &mut |m: &mut ThreeMixer, a: usize| {
            if let Some(i) = remaining.iter().position(|(start, ..)| *start == a) {
                let (start, end, duration) = remaining.remove(i);
                execute_cross_fade(m, start, end, duration);
            }
        })?;
        *sync = remaining;
        Ok(())
    }
}
