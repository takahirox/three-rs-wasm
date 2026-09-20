//! Pinned official examples using GPU material-input graphs.
use super::gltf_viewer::OrbitViewer;
use crate::{
    Result,
    attribute::BufferAttribute,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, surface::SurfaceNodes},
};
use std::sync::Arc;
mod bloom;
mod helmet;
mod jelly;
mod mask;
mod procedural;
mod selective;
mod storage;
mod tornado;
pub(super) struct Demo {
    example: u32,
    time: f64,
    lights: Vec<Object3D>,
    viewer: OrbitViewer,
    objects: Vec<Object3D>,
    params: [[f32; 4]; 16],
    orbit: Vector2,
    pan: Vector3,
    mixer: Option<crate::animation::AnimationMixer>,
    depth: Option<(RenderTarget, crate::postprocessing::Effect)>,
    mrt_sampler: Option<wgpu::Sampler>,
    bloom: Option<bloom::BloomPass>,
    storage: Option<storage::Storage>,
    jelly: Option<jelly::Jelly>,
    mask: Option<mask::MaskPass>,
}
impl Demo {
    pub async fn create(
        scene: &mut Scene,
        cam: Object3D,
        example: u32,
        r: &Renderer,
    ) -> Result<Self> {
        if example == 77 {
            return Self::mask(scene, cam, r).await;
        }
        if example == 76 {
            return Self::tornado(scene, cam, r).await;
        }
        if example == 75 {
            return Self::jelly(scene, cam, r).await;
        }
        if example == 74 {
            return Self::storage(scene, cam, r).await;
        }
        if example == 73 {
            return Self::selective_bloom(scene, cam, r).await;
        }
        if [71, 72].contains(&example) {
            return Self::bloom_model(scene, cam, r, example).await;
        }
        if example == 70 {
            return Self::material_mrt(scene, cam, r).await;
        }
        if [68, 69].contains(&example) {
            return Self::selective(scene, cam, r, example).await;
        }
        if example == 67 {
            return Self::fog_background(scene, cam, r).await;
        }
        if example == 66 {
            return Self::flames(scene, cam, r).await;
        }
        if example == 65 {
            return Self::shadertoy(scene, cam, r).await;
        }
        if example == 64 {
            return Self::custom_lights(scene, cam, r).await;
        }
        if example == 63 {
            return Self::mrt(scene, cam, r).await;
        }
        if example == 62 {
            return Self::depth(scene, cam, r).await;
        }
        if example == 61 {
            return Self::skinning(scene, cam, r).await;
        }
        if example == 60 {
            return Self::halftone(scene, cam, r).await;
        }
        if example == 59 {
            return Self::sea(scene, cam, r).await;
        }
        if example != 58 {
            return Err(crate::Error::Invalid("TSL surface example"));
        }
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.0,
            aspect,
            near: 1.0,
            far: 1000.0,
            ..Default::default()
        }));
        let position = Vector3::new(0.0, 5.0, -15.0);
        let target = Vector3::new(0.0, 5.5, 0.0);
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, target)?;
        scene.background = Color::BLACK;
        scene.background_alpha = 0.0;
        let mut lights = Vec::new();
        for (x, hex) in [(-5.0, 0xff0000), (0.0, 0x00ff00), (5.0, 0x0000ff)] {
            let color = Color::from_hex(hex);
            let h = scene.insert(NodeKind::Light(Light::RectArea {
                color,
                intensity: 5.0,
                width: 4.0,
                height: 10.0,
            }));
            scene.get_mut(h)?.position = Vector3::new(x, 6.0, 5.0);
            lights.push(h);
            let mut m = MeshBasicMaterial::default();
            m.properties.color = color;
            m.properties.side = Side::Back;
            let helper = scene.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(4.0, 10.0, 1, 1)?),
                Arc::new(Material::Basic(m)),
            )));
            scene.add(h, helper)?;
            let mut geometry = BufferGeometry::default();
            geometry.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(
                    vec![
                        2.0, 5.0, 0.0, -2.0, 5.0, 0.0, -2.0, -5.0, 0.0, 2.0, -5.0, 0.0, 2.0, 5.0,
                        0.0,
                    ],
                    3,
                    false,
                )?),
            );
            let mut line = LineBasicMaterial::default();
            line.properties.color = color;
            let outline = scene.insert(NodeKind::Line(Line {
                geometry: Arc::new(geometry),
                material: Arc::new(Material::Line(line)),
                segments: false,
            }));
            scene.add(h, outline)?;
        }
        let graph = SurfaceNodes {
            roughness: Some(tsl::checker(tsl::uv() * tsl::float(400.0))),
            ..Default::default()
        };
        let mut floor = MeshStandardMaterial::default();
        floor.properties.color = Color::from_hex(0x444444);
        floor.properties.vertex_program = Some(Arc::new(graph.build(r, &[], &[]).await?));
        scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(2000.0, 0.1, 2000.0)?),
            Arc::new(Material::Standard(floor)),
        )));
        let knot = MeshStandardMaterial {
            roughness: 0.0,
            ..Default::default()
        };
        let h = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(1.5, 0.5, 200, 16, 2, 3)?),
            Arc::new(Material::Standard(knot)),
        )));
        scene.get_mut(h)?.position = target;
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        Ok(Self {
            example,
            time: 0.0,
            lights,
            viewer,
            objects: vec![],
            params: [[0.0; 4]; 16],
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
    async fn sea(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        use tsl::*;
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.0,
            aspect,
            near: 0.1,
            far: 10.0,
            ..Default::default()
        }));
        let position = Vector3::splat(1.25);
        let target = Vector3::new(0.0, -0.25, 0.0);
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, target)?;
        scene.background = Color::BLACK;
        let light = scene.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.0,
            target: Vector3::ZERO,
        }));
        scene.get_mut(light)?.position = Vector3::new(-4.0, 2.0, 0.0);
        let source = format!(
            "{}\nfn sea_wave(p:vec3<f32>,t:f32,large:vec4<f32>,small:vec4<f32>)->f32 {{var height=sin(p.x*large.x+t*large.z)*sin(p.z*large.y+t*large.z)*large.w;for(var i=1.0;i<small.x+1.0;i+=1.0){{height-=abs(tsl_mx_noise3(vec3((p.xz+2.0)*small.y*i,t*small.z))*small.w/i);}}return height;}}",
            include_str!("../tsl/noise.wgsl")
        );
        let wave = WgslFn::new(
            "sea_wave",
            &source,
            &[Type::Vec3, Type::Float, Type::Vec4, Type::Vec4],
            Type::Float,
        )?;
        let elevation = |p: tsl::Node| {
            wave.call(&[
                p,
                uniform(0, Type::Float),
                uniform(1, Type::Vec4),
                uniform(2, Type::Vec4),
            ])
        };
        let vertex = position_geometry();
        let zero = float(0.0);
        let displaced = vertex.clone() + vec3(zero.clone(), elevation(vertex), zero.clone());
        let p = position_local();
        let height = elevation(p.clone());
        let center = p.clone() + vec3(zero.clone(), height.clone(), zero.clone());
        let shift = uniform(3, Type::Vec4).x();
        let a = p.clone() + vec3(shift.clone(), zero.clone(), zero.clone());
        let b = p + vec3(zero.clone(), zero.clone(), -shift);
        let a = a.clone() + vec3(zero.clone(), elevation(a), zero.clone());
        let b = b.clone() + vec3(zero.clone(), elevation(b), zero);
        let normal = (a - center.clone())
            .normalize()
            .cross((b - center).normalize());
        let transform = WgslFn::new(
            "sea_normal",
            "fn sea_normal(n:vec3<f32>)->vec3<f32>{return normalize((u.view*u.normal*vec4(n,0.0)).xyz);}",
            &[Type::Vec3],
            Type::Vec3,
        )?;
        let settings = uniform(3, Type::Vec4);
        let emissive = ((height - settings.swizzle("z")) / (settings.y() - settings.swizzle("z")))
            .pow(settings.swizzle("w"));
        let graph = SurfaceNodes {
            position: Some(displaced),
            normal: Some(transform.call(&[normal])),
            roughness: Some(uniform(6, Type::Float)),
            color: Some(uniform(5, Type::Vec3)),
            emissive: Some(uniform(4, Type::Vec3) * emissive),
            ..Default::default()
        };
        let mut material = MeshStandardMaterial::default();
        material.properties.vertex_program = Some(Arc::new(graph.build(r, &[], &[]).await?));
        let mut geometry = PlaneGeometry::build(2.0, 2.0, 256, 256)?;
        geometry.rotate_x(-std::f64::consts::FRAC_PI_2)?;
        let object = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Standard(material)),
        )));
        scene.get_mut(object)?.frustum_culled = false;
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        let mut params = [[0.0; 4]; 16];
        params[1] = [3.0, 1.0, 1.25, 0.15];
        params[2] = [3.0, 2.0, 0.3, 0.18];
        params[3] = [0.01, -0.25, 0.2, 7.0];
        params[4] = Color::from_hex(0xff0a81).0.extend(0.0).as_vec4().to_array();
        params[5] = Color::from_hex(0x271442).0.extend(0.0).as_vec4().to_array();
        params[6][0] = 0.15;
        Ok(Self {
            example: 59,
            time: 0.0,
            lights: vec![],
            viewer,
            objects: vec![object],
            params,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
    async fn halftone(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        use tsl::*;
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 25.0,
            aspect,
            near: 0.1,
            far: 100.0,
            ..Default::default()
        }));
        let position = Vector3::new(6.0, 3.0, 10.0);
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, Vector3::ZERO)?;
        scene.background = Color::BLACK;
        let ambient = scene.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 3.0,
        }));
        let sun = scene.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 8.0,
            target: Vector3::ZERO,
        }));
        scene.get_mut(sun)?.position = Vector3::new(4.0, 3.0, 1.0);
        let mut color = output().rgb();
        for slot in [1, 5] {
            let settings = uniform(slot, Type::Vec4);
            let direction = uniform(slot + 1, Type::Vec3);
            let tint = uniform(slot + 2, Type::Vec3);
            let levels = uniform(slot + 3, Type::Vec4);
            let p = screen_coordinate() / screen_size().y() * settings.x();
            let grid = vec2(
                (p.x() - p.y()) * float(std::f32::consts::FRAC_1_SQRT_2),
                (p.x() + p.y()) * float(std::f32::consts::FRAC_1_SQRT_2),
            )
            .modulo(float(1.0));
            let strength = ((normal_world().dot(direction.normalize()) - settings.swizzle("z"))
                / (settings.y() - settings.swizzle("z")))
            .clamp(float(0.0), float(1.0));
            let radius = strength.clone() * settings.swizzle("w") * float(0.5);
            let mask = (grid - float(0.5))
                .length()
                .less_than(radius)
                .select(float(1.0), float(0.0))
                * mix(levels.x(), levels.y(), strength);
            color = mix(color, tint, mask);
        }
        let program = Arc::new(output_program(r, &vec4(color, output().swizzle("w"))).await?);
        let mut material = MeshStandardMaterial::default();
        material.properties.color = Color::from_hex(0xff622e);
        material.properties.vertex_program = Some(program.clone());
        let mut objects = Vec::new();
        for (g, x) in [
            (TorusKnotGeometry::build(0.6, 0.25, 128, 32, 2, 3)?, 3.0),
            (SphereGeometry::build(1.0, 64, 64)?, -3.0),
        ] {
            let h = scene.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(g),
                Arc::new(Material::Standard(material.clone())),
            )));
            scene.get_mut(h)?.position.x = x;
            objects.push(h);
        }
        let (asset, buffers, images) =
            super::gltf_viewer::load_asset("/web/models/Michelle.glb").await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
        let group = scene.insert(NodeKind::Group);
        scene.get_mut(group)?.position.y = -2.0;
        scene.get_mut(group)?.scale = Vector3::splat(2.5);
        for h in instance.roots {
            scene.add(group, h)?;
        }
        for &h in &instance.meshes {
            if let NodeKind::Mesh(mesh) = &mut scene.get_mut(h)?.kind {
                for m in &mut mesh.materials {
                    Arc::make_mut(m).properties_mut().vertex_program = Some(program.clone());
                }
            }
        }
        objects.extend(instance.meshes);
        let mut params = [[0.0; 4]; 16];
        params[1] = [140.0, 1.0, 0.0, 0.8];
        params[2] = [-0.4, -1.0, 0.5, 0.0];
        params[3] = Color::from_hex(0xfb00ff).0.extend(0.0).as_vec4().to_array();
        params[4] = [0.0, 0.5, 0.0, 0.0];
        params[5] = [180.0, 0.55, 0.2, 0.8];
        params[6] = [0.5, 0.5, -0.2, 0.0];
        params[7] = Color::from_hex(0x94ffd1).0.extend(0.0).as_vec4().to_array();
        params[8] = [0.5, 1.0, 0.0, 0.0];
        params[9] = [3.0, 8.0, 0xff622e as f32, 0.0];
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
        viewer.fixture(
            position.x.atan2(position.z),
            (position.y / position.length()).asin(),
            1.8,
        );
        Ok(Self {
            example: 60,
            time: 0.0,
            lights: vec![ambient, sun],
            viewer,
            objects,
            params,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
    async fn skinning(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        use tsl::*;
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.0,
            aspect,
            near: 0.01,
            far: 100.0,
            ..Default::default()
        }));
        let position = Vector3::new(1.0, 2.0, 3.0);
        let target = Vector3::Y;
        scene.get_mut(cam)?.position = position;
        scene.look_at(cam, target)?;
        scene.background = Color::BLACK;
        scene.tone_mapping = ToneMapping::Linear;
        scene.exposure = 0.4;
        let rgb = |hex| {
            let c = Color::from_hex(hex).0;
            vec3(float(c.x as f32), float(c.y as f32), float(c.z as f32))
        };
        let background = surface::background_material(
            r,
            mix(rgb(0x66bbff), rgb(0x4466ff), float(1.0) - uv().y()),
        )
        .await?;
        let h = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
            Arc::new(Material::Shader(background)),
        )));
        scene.get_mut(h)?.frustum_culled = false;
        scene.get_mut(h)?.render_order = i32::MIN;
        let light = scene.insert(NodeKind::Light(Light::Point {
            color: Color::WHITE,
            intensity: 2500.0 / (4.0 * std::f64::consts::PI),
            distance: 100.0,
            decay: 2.0,
        }));
        scene.add(cam, light)?;
        scene.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x4466ff),
            intensity: 1.0,
        }));
        let (asset, buffers, images) =
            super::gltf_viewer::load_asset("/web/models/Michelle.glb").await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        let offset = position - target;
        let mut viewer = OrbitViewer::from_camera(target, offset.length());
        viewer.fixture(
            offset.x.atan2(offset.z),
            (offset.y / offset.length()).asin(),
            1.8,
        );
        Ok(Self {
            example: 61,
            time: 0.0,
            lights: vec![],
            objects: vec![],
            params: [[0.0; 4]; 16],
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: Some(mixer),
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
    async fn depth(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        use tsl::*;
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.0,
            aspect,
            near: 1.0,
            far: 20.0,
            ..Default::default()
        }));
        scene.get_mut(cam)?.position = Vector3::new(0.0, 0.0, 4.0);
        scene.look_at(cam, Vector3::ZERO)?;
        scene.background = Color::from_hex(0x222222);
        let geometry = Arc::new(TorusKnotGeometry::build(1.0, 0.3, 128, 64, 2, 3)?);
        let material = Arc::new(Material::Basic(MeshBasicMaterial::default()));
        let mut seed = 186u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.0
        };
        for _ in 0..50 {
            let angle = random() * std::f64::consts::TAU;
            let z = random() * 2.0 - 1.0;
            let scale = (1.0 - z * z).sqrt() * 5.0;
            let h = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                material.clone(),
            )));
            scene.get_mut(h)?.position =
                Vector3::new(angle.cos() * scale, angle.sin() * scale, z * 5.0);
            scene.get_mut(h)?.quaternion =
                Quaternion::from_euler(glam::EulerRot::XYZ, random(), random(), random());
        }
        let target = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..Default::default()
            },
        )?;
        let view = target
            .depth_texture()
            .unwrap()
            .create_view(&Default::default());
        let effect = depth_effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &vec4(splat(depth_texture(uv()), Type::Vec3), float(1.0)),
            &view,
        )
        .await?;
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 4.0);
        viewer.fixture(0.0, 0.0, 1.8);
        Ok(Self {
            example: 62,
            time: 0.0,
            lights: vec![],
            objects: vec![],
            params: [[0.0; 4]; 16],
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: Some((target, effect)),
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        scene: &mut Scene,
        cam: Object3D,
        target: &RenderTarget,
    ) -> Result<bool> {
        if let Some(mask) = &mut self.mask {
            mask.render(r, scene, cam, target)?;
            return Ok(true);
        }
        if let Some(jelly) = &mut self.jelly {
            jelly.render(r, &self.params)?;
        }
        if let Some(storage) = &mut self.storage {
            storage.update(r, self.time)?;
        }
        if self.example == 74
            && let NodeKind::Camera(Camera::Orthographic(c)) = &mut scene.get_mut(cam)?.kind
        {
            c.left = -(target.width as f64) / (target.height as f64);
            c.right = -c.left;
        }
        if let Some(bloom) = &mut self.bloom {
            bloom.render(r, scene, cam, target)?;
            return Ok(true);
        }
        let Some((input, effect)) = &mut self.depth else {
            return Ok(false);
        };
        if input.width != target.width || input.height != target.height {
            input.set_size(&r.device, target.width, target.height)?;
            if let Some(sampler) = &self.mrt_sampler {
                let views: Vec<_> = input
                    .textures()
                    .iter()
                    .map(|t| t.create_view(&Default::default()))
                    .collect();
                effect.set_textures(
                    r,
                    &views.iter().map(|view| (view, sampler)).collect::<Vec<_>>(),
                )?;
            } else {
                effect.set_depth(
                    r,
                    &input
                        .depth_texture()
                        .unwrap()
                        .create_view(&Default::default()),
                )?;
            }
        }
        r.render(scene, cam, input)?;
        effect.apply(r, input, None, target)?;
        Ok(true)
    }
    async fn mrt(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        use tsl::*;
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.0,
            aspect,
            near: 0.1,
            far: 50.0,
            ..Default::default()
        }));
        scene.get_mut(cam)?.position = Vector3::new(0.0, 0.0, 4.0);
        scene.look_at(cam, Vector3::ZERO)?;
        scene.background = Color::from_hex(0x222222);
        let mut image = super::gltf_viewer::decode_image(
            &super::gltf_viewer::fetch("/web/gallery/assets/hardwood2_diffuse.jpg").await?,
        )
        .await?;
        image.srgb = true;
        image.wrap_s = Wrapping::Repeat;
        image.wrap_t = Wrapping::Repeat;
        image.mipmap_filter = Some(Filter::Linear);
        let image = r.upload_texture(&Arc::new(image))?;
        let uv = tsl::uv() * vec2(float(10.0), float(4.0));
        let graph = SurfaceNodes {
            color: Some(tsl::Texture::External(0).sample(vec2(uv.x(), float(1.0) - uv.y()))),
            ..Default::default()
        };
        let program = graph
            .build_mrt(
                r,
                &[output(), normal_world()],
                &[],
                &[(&image.view, &image.sampler)],
            )
            .await?;
        let mut material = MeshBasicMaterial::default();
        material.properties.vertex_program = Some(Arc::new(program));
        let object = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(TorusKnotGeometry::build(1.0, 0.3, 128, 32, 2, 3)?),
            Arc::new(Material::Basic(material)),
        )));
        let target = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                count: 2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..Default::default()
            },
        )?;
        let sampler = r.device.create_sampler(&Default::default());
        let views: Vec<_> = target
            .textures()
            .iter()
            .map(|t| t.create_view(&Default::default()))
            .collect();
        let color = tsl::uv().x().less_than(float(0.5)).select(
            tsl::Texture::External(0).sample(tsl::uv()),
            tsl::Texture::External(1).sample(tsl::uv()),
        );
        let effect = effect_with_textures(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &color,
            &[(&views[0], &sampler), (&views[1], &sampler)],
        )
        .await?;
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 4.0);
        viewer.fixture(0.0, 0.0, 1.8);
        Ok(Self {
            example: 63,
            time: 0.0,
            lights: vec![],
            objects: vec![object],
            params: [[0.0; 4]; 16],
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: Some((target, effect)),
            mrt_sampler: Some(sampler),
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
    async fn custom_lights(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        use tsl::*;
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.0,
            aspect,
            near: 0.1,
            far: 10.0,
            ..Default::default()
        }));
        scene.get_mut(cam)?.position = Vector3::new(0.0, 0.0, 1.5);
        scene.look_at(cam, Vector3::ZERO)?;
        scene.background = Color::BLACK;
        let group = scene.insert(NodeKind::Group);
        let mut lights = Vec::new();
        let geometry = Arc::new(SphereGeometry::build(0.02, 16, 8)?);
        let mut color = splat(float(0.0), Type::Vec3);
        for (i, hex) in [0xffaa00, 0x0040ff, 0x80ff80].into_iter().enumerate() {
            let rgb = Color::from_hex(hex);
            let mut material = MeshBasicMaterial::default();
            material.properties.color = rgb;
            let h = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Basic(material)),
            )));
            scene.add(group, h)?;
            lights.push(h);
            let distance = (position_world() - uniform(i + 1, Type::Vec3)).length();
            let attenuation = (float(1.0) - distance.clone().pow(float(4.0)))
                .clamp(float(0.0), float(1.0))
                .pow(float(2.0))
                / distance.pow(float(2.0)).max(float(0.01));
            color = color
                + vec3(
                    float(rgb.0.x as f32),
                    float(rgb.0.y as f32),
                    float(rgb.0.z as f32),
                ) * float(0.1)
                    * attenuation;
        }
        let mut seed = 186u32;
        let data: Vec<f32> = (0..500000 * 3)
            .map(|_| {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                ((seed as f64 / 4294967296.0 - 0.5) * 3.0) as f32
            })
            .collect();
        let buffer = crate::compute::GpuBuffer::new(
            r,
            bytemuck::cast_slice(&data),
            crate::compute::BufferAccess::Read,
        )?;
        let index = instance_index() * uint(3);
        let mut graph = NodeMaterial::new(color);
        graph.position = Some(vec3(
            storage_element(0, index.clone()),
            storage_element(0, index.clone() + uint(1)),
            storage_element(0, index + uint(2)),
        ));
        let material = graph
            .build_with_storage(r, &[(&buffer, Type::Float)], &[])
            .await?;
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(vec![0.0; 3], 3, false)?),
        );
        geometry.instance_count = Some(500000);
        let points = scene.insert(NodeKind::Points(Points {
            geometry: Arc::new(geometry),
            material: Arc::new(Material::Shader(material)),
        }));
        scene.get_mut(points)?.frustum_culled = false;
        scene.add(group, points)?;
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 1.5);
        viewer.fixture(0.0, 0.0, 1.8);
        Ok(Self {
            example: 64,
            time: 0.0,
            lights,
            objects: vec![group, points],
            params: [[0.0; 4]; 16],
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if self.example == 76 && index < 7 && value.is_finite() {
            match index {
                0 => {
                    self.params[3] = Color::from_hex(value as u32)
                        .0
                        .extend(0.0)
                        .as_vec4()
                        .to_array()
                }
                1..=4 => {
                    self.params[2][index - 1] = value.clamp(
                        if index == 1 { -1.0 } else { 0.0 },
                        if index == 2 || index == 4 { 2.0 } else { 1.0 },
                    )
                }
                5 => self.params[1][1] = value.clamp(0.0, 10.0),
                6 => self.params[1][2] = value.clamp(0.0, 1.0),
                _ => {}
            }
            return Ok(());
        }
        if self.example == 75 && index < 4 && value.is_finite() {
            self.params[1][index] =
                value.clamp([0.0, 0.9, 0.1, 0.1][index], [0.5, 0.98, 0.5, 0.3][index]);
            return Ok(());
        }
        if self.example == 72 && index < 3 && value.is_finite() {
            let (slot, max) = match index {
                0 => (1, 5.0),
                1 => (2, 1.0),
                _ => (3, 2.0),
            };
            self.params[1][slot] = value.clamp(0.0, max);
            return Ok(());
        }
        if [71, 73].contains(&self.example) && index < 4 && value.is_finite() {
            self.params[1][index] = value.clamp(
                0.0,
                if index == 1 {
                    3.0
                } else if index == 3 {
                    if self.example == 73 { 3.0 } else { 2.0 }
                } else {
                    1.0
                },
            );
            return Ok(());
        }
        if self.example == 68 && index < 2 && value.is_finite() {
            self.params[1][index] = value.clamp(0.0, 1.0);
            return Ok(());
        }
        if self.example == 60 && value.is_finite() {
            if index < 2 {
                self.params[9][index] = value.clamp(0.0, if index == 0 { 10.0 } else { 20.0 });
            } else if index == 22 {
                self.params[9][2] = value;
            } else if index < 22 {
                let group = (index - 2) / 10;
                let field = (index - 2) % 10;
                let slot = 1 + group * 4;
                match field {
                    0 => {
                        self.params[slot + 2] = Color::from_hex(value as u32)
                            .0
                            .extend(0.0)
                            .as_vec4()
                            .to_array()
                    }
                    1 => self.params[slot][0] = value.clamp(1.0, 200.0),
                    2..=4 => self.params[slot + 1][field - 2] = value.clamp(-1.0, 1.0),
                    5 => self.params[slot][1] = value.clamp(-1.0, 1.0),
                    6 => self.params[slot][2] = value.clamp(-1.0, 1.0),
                    7 => self.params[slot + 3][0] = value.clamp(0.0, 1.0),
                    8 => self.params[slot + 3][1] = value.clamp(0.0, 1.0),
                    9 => self.params[slot][3] = value.clamp(0.0, 1.0),
                    _ => unreachable!(),
                }
            } else {
                return Err(crate::Error::Invalid("surface parameter"));
            }
            return Ok(());
        }
        if self.example != 59 || !value.is_finite() {
            return Err(crate::Error::Invalid("surface parameter"));
        }
        match index {
            0 | 2 => {
                self.params[if index == 0 { 5 } else { 4 }] = Color::from_hex(value as u32)
                    .0
                    .extend(0.0)
                    .as_vec4()
                    .to_array()
            }
            1 => self.params[6][0] = value.clamp(0.0, 1.0),
            3 => self.params[3][1] = value.clamp(-1.0, 0.0),
            4 => self.params[3][2] = value.clamp(0.0, 1.0),
            5 => self.params[3][3] = value.clamp(1.0, 10.0),
            6 => self.params[1][2] = value.clamp(0.0, 5.0),
            7 => self.params[1][3] = value.clamp(0.0, 1.0),
            8 => self.params[1][0] = value.clamp(0.0, 10.0),
            9 => self.params[1][1] = value.clamp(0.0, 10.0),
            10 => self.params[2][0] = value.clamp(0.0, 5.0).floor(),
            11 => self.params[2][1] = value.clamp(0.0, 10.0),
            12 => self.params[2][2] = value.clamp(0.0, 1.0),
            13 => self.params[2][3] = value.clamp(0.0, 1.0),
            14 => self.params[3][0] = value.clamp(0.0, 0.1),
            _ => return Err(crate::Error::Invalid("surface parameter")),
        }
        Ok(())
    }
    fn limits(&self) -> (f64, f64) {
        match self.example {
            59 | 60 | 66 | 76 => (0.1, 50.0),
            64 => (0.001, 4.0),
            67 => (2.0, 5.0),
            70 | 72 => (2.0, 10.0),
            71 => (3.0, 8.0),
            73 => (1.0, 100.0),
            75 => (0.7, 2.0),
            68 | 69 => (3.0, 25.0),
            _ => (0.001, 1e6),
        }
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        cam: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        if animate {
            self.time += delta;
            if self.example == 77 && self.params[4][0] < 0.5 {
                self.params[4][1] += delta as f32 * 0.5;
            }
            if let Some(jelly) = &mut self.jelly {
                jelly.step();
            }
        }
        if let Some(bloom) = &mut self.bloom {
            bloom.bloom.threshold = self.params[1][0];
            bloom.bloom.strength = self.params[1][1];
            bloom.bloom.radius = self.params[1][2];
            scene.exposure = if self.example == 71 {
                (self.params[1][3] as f64).powi(4)
            } else {
                self.params[1][3] as f64
            };
        }

        if [68, 69].contains(&self.example) {
            let t = self.time * 0.5;
            let positions = [
                Vector3::new(
                    (t * 0.7).sin() * 3.0,
                    (t * 0.5).cos() * 4.0,
                    (t * 0.3).cos() * 3.0,
                ),
                Vector3::new(
                    (t * 0.3).cos() * 3.0,
                    (t * 0.5).sin() * 4.0,
                    (t * 0.7).sin() * 3.0,
                ),
                Vector3::new(
                    (t * 0.7).sin() * 3.0,
                    (t * 0.3).cos() * 4.0,
                    (t * 0.5).sin() * 3.0,
                ),
                Vector3::new(
                    (t * 0.3).sin() * 3.0,
                    (t * 0.7).cos() * 4.0,
                    (t * 0.5).sin() * 3.0,
                ),
            ];
            for (&light, position) in self.lights.iter().zip(positions) {
                scene.get_mut(light)?.position = position;
            }
            if self.example == 68
                && let NodeKind::Mesh(m) = &mut scene.get_mut(self.objects[1])?.kind
                && let Material::Standard(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.roughness = self.params[1][0] as f64;
                m.metalness = self.params[1][1] as f64;
            }
        }
        if self.example == 64 {
            let time = self.time;
            let positions = [
                Vector3::new((time * 0.7).sin(), (time * 0.5).cos(), (time * 0.3).cos()),
                Vector3::new((time * 0.3).cos(), (time * 0.5).sin(), (time * 0.7).sin()),
                Vector3::new((time * 0.7).sin(), (time * 0.3).cos(), (time * 0.5).sin()),
            ];
            let rotation = Quaternion::from_rotation_y(time * 0.1);
            scene.get_mut(self.objects[0])?.quaternion = rotation;
            for (i, &h) in self.lights.iter().enumerate() {
                let position = positions[i] * 0.5;
                scene.get_mut(h)?.position = position;
                self.params[i + 1] = (rotation * position).extend(0.0).as_vec4().to_array();
            }
        }
        if self.example == 77 {
            scene.get_mut(self.objects[0])?.quaternion =
                Quaternion::from_rotation_y(self.params[4][1] as f64);
        }
        if self.example == 63 {
            scene.get_mut(self.objects[0])?.quaternion =
                Quaternion::from_rotation_y(self.time * 0.4);
        }
        if self.example == 58 {
            for (&h, speed) in self.lights.iter().zip([-1.0, 0.5, 1.0]) {
                scene.get_mut(h)?.quaternion = Quaternion::from_rotation_y(self.time * speed);
            }
        }
        if self.example == 60 {
            self.params[6][0] = self.time.cos() as f32;
            self.params[6][1] = self.time.sin() as f32;
            for (i, &h) in self.lights.iter().enumerate() {
                match &mut scene.get_mut(h)?.kind {
                    NodeKind::Light(Light::Ambient { intensity, .. })
                    | NodeKind::Light(Light::Directional { intensity, .. }) => {
                        *intensity = self.params[9][i] as f64
                    }
                    _ => {}
                }
            }
            for &h in &self.objects[..2] {
                if let NodeKind::Mesh(m) = &mut scene.get_mut(h)?.kind {
                    Arc::make_mut(&mut m.materials[0]).properties_mut().color =
                        Color::from_hex(self.params[9][2] as u32);
                }
            }
        }
        let (min, max) = self.limits();
        let damping = if self.example == 62 {
            0.0
        } else if [59, 60, 66, 76].contains(&self.example) {
            0.05
        } else {
            1.0
        };
        let height = web_sys::window()
            .unwrap()
            .inner_height()
            .unwrap()
            .as_f64()
            .unwrap_or(512.0);
        self.viewer.orbit_pixels(
            self.orbit.x * damping,
            self.orbit.y * damping,
            0.0,
            height,
            min,
            max,
        );
        self.orbit *= 1.0 - damping;
        self.viewer.pan_world(self.pan * damping);
        self.pan *= 1.0 - damping;
        if [71, 73].contains(&self.example) {
            self.viewer
                .limit_pitch(0.0, std::f64::consts::FRAC_PI_2 - 1e-6);
        }
        if ![65, 74].contains(&self.example) {
            self.viewer.update(scene, cam)?;
        }
        if let Some(mixer) = &mut self.mixer {
            for action in &mut mixer.actions {
                action.time = self.time;
            }
            mixer.update(scene, 0.0)?;
        }
        self.params[0][0] = self.time as f32;
        self.params[0][1] = height as f32;
        for &h in &self.objects {
            match &mut scene.get_mut(h)?.kind {
                NodeKind::Mesh(m) => {
                    for material in &mut m.materials {
                        let material = Arc::make_mut(material);
                        if let Material::Shader(m) = material {
                            m.uniforms = self.params;
                        } else if self.example != 73 {
                            material.properties_mut().vertex_uniforms = self.params;
                        }
                    }
                }
                NodeKind::Points(p) => {
                    if let Material::Shader(m) = Arc::make_mut(&mut p.material) {
                        m.uniforms = self.params;
                    }
                }
                _ => {}
            }
        }

        if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(cam)?.kind {
            c.near = match self.example {
                61 | 68 | 69 | 77 => 0.01,
                67 | 70 | 72 => 0.25,
                62 | 73 => 1.0,
                63 | 64 => 0.1,
                59 | 60 | 66 | 75 | 76 => 0.1,
                _ => 1.0,
            };
            c.far = match self.example {
                59 | 64 | 75 => 10.0,
                62 | 67 | 70 | 72 => 20.0,
                63 | 76 => 50.0,
                73 => 200.0,
                60 | 61 | 66 | 68 | 69 | 71 | 77 => 100.0,
                _ => 1000.0,
            };
        }
        Ok(())
    }
    pub fn seek(&mut self, time: f64) {
        if self.example == 77 && self.params[4][0] < 0.5 {
            self.params[4][1] += (time - self.time) as f32 * 0.5;
        }
        self.time = time;
        if let Some(jelly) = &mut self.jelly {
            jelly.step();
        }
    }
    pub fn dragging(&mut self, value: bool) {
        if self.example == 77 {
            self.params[4][0] = if value { 1.0 } else { 0.0 };
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        scene: &Scene,
        cam: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let (min, max) = self.limits();
        if pan {
            self.pan += OrbitViewer::pan_delta(scene, cam, self.viewer.radius(), dx, dy, height)?;
        } else {
            self.orbit += Vector2::new(dx, dy);
            self.viewer.orbit_pixels(0.0, 0.0, wheel, height, min, max);
        }
        if self.example == 62 {
            self.viewer.orbit_pixels(
                self.orbit.x * 0.05,
                self.orbit.y * 0.05,
                0.0,
                height,
                0.001,
                1e6,
            );
            self.orbit *= 0.95;
            self.viewer.pan_world(self.pan * 0.05);
            self.pan *= 0.95;
        }
        Ok(())
    }
}
