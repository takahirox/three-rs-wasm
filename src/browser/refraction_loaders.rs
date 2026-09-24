//! Cube-map refraction, the PLY loader with shadows, the KMZ and Collada loaders
//! and the EXR loader from the pinned WebGL examples.
pub(super) mod formats;
use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::helpers_formats::formats::{parse_xml, unzip};
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    shader::ShaderProgram,
    texture_gpu::GpuTexture,
    tsl::{surface::SurfaceNodes, *},
};
use formats::{DaeScene, Shading, decode_exr, parse_collada, parse_ply};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
/// PLYLoader, then the examples' `computeVertexNormals`.
async fn ply(path: &str) -> Result<Arc<BufferGeometry>> {
    let (positions, index) = parse_ply(&fetch(&format!("{ASSETS}/{path}")).await?)?;
    let mut g = BufferGeometry::default();
    g.set_attribute("position", vec3s(positions)?);
    g.set_index(Some(index));
    g.compute_vertex_normals()?;
    Ok(Arc::new(g))
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    last: f64,
    controls: Option<Controls>,
    pointer: Option<Vector2>,
    elf: Option<Object3D>,
    sky: Option<Object3D>,
    exr_quad: Option<Object3D>,
    exr_texture: Option<wgpu::Texture>,
    exposure: f32,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            238 => (50., 1., 100000., Vector3::new(0., 0., -4000.)),
            239 => (35., 1., 15., Vector3::new(3., 0.15, 3.)),
            240 => (35., 1., 500., Vector3::new(0., 5., 10.)),
            _ => (45., 0.1, 2000., Vector3::new(8., 10., 8.)),
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
            pointer: None,
            elf: None,
            sky: None,
            exr_quad: None,
            exr_texture: None,
            exposure: 2.,
        };
        match id {
            238 => d.refraction(s, c, r).await?,
            239 => d.ply_scene(s, c).await?,
            240 => d.kmz(s, c).await?,
            241 => d.collada(s, c).await?,
            _ => d.exr(s, c, r).await?,
        }
        Ok(d)
    }
    async fn refraction(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        // CubeTextureLoader: six sRGB faces with mipmaps.
        let mut faces = vec![];
        for face in ["px", "nx", "py", "ny", "pz", "nz"] {
            let mut t =
                decode_texture_image(&fetch(&format!("{ASSETS}/cube/Park3Med/{face}.jpg")).await?)
                    .await?;
            t.srgb = true;
            faces.push(t);
        }
        let env = GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 3.5,
        }));
        // envmap_fragment with CubeRefractionMapping and MultiplyOperation: the view ray
        // refracted about the world normal, flipped in x, mixed in by the reflectivity.
        let mut materials = vec![];
        for (hex, ratio, reflectivity) in [
            (0xffffff, 0.98, 1.),
            (0xccfffd, 0.985, 1.),
            (0xccddff, 0.98, 0.9),
        ] {
            let refract = WgslFn::new(
                "cube_refract",
                &format!(
                    "fn cube_refract(value:vec4<f32>)->vec4<f32>{{let n=normalize(fragment_surface.normal)*select(-1.0,1.0,fragment_front);let w=normalize(transpose(mat3x3(u.view[0].xyz,u.view[1].xyz,u.view[2].xyz))*n);let d=refract(normalize(fragment_surface.position-u.camera.xyz),w,{ratio:?});let e=textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb;return vec4(mix(value.rgb,value.rgb*e,{reflectivity:?}),value.a);}}"
                ),
                &[Type::Vec4],
                Type::Vec4,
            )?
            .call(&[output()]);
            let program = SurfaceNodes {
                output: Some(refract),
                ..Default::default()
            }
            .build_with_texture_types(r, &[], &[(&env.view, &env.sampler, Type::TextureCube)])
            .await?;
            let mut m = MeshPhongMaterial::default();
            m.properties.color = Color::from_hex(hex);
            m.properties.vertex_program = Some(Arc::new(program));
            materials.push(Arc::new(Material::Phong(m)));
        }
        // scene.background = textureCube: a camera-centered box at the far plane.
        let graph = NodeMaterial::new(vec4(
            WgslFn::new(
                "sky_color",
                "fn sky_color(d:vec3<f32>)->vec3<f32>{return textureSample(tsl_texture_0,tsl_sampler_0,vec3(-d.x,d.yz)).rgb;}",
                &[Type::Vec3],
                Type::Vec3,
            )?
            .call(&[position_local()]),
            float(1.),
        ));
        let source = graph.wgsl_with_texture_types(&[Type::TextureCube], &[])?;
        let mut m = ShaderMaterial::new(Arc::new(
            ShaderProgram::with_projection_and_dimensions(
                r,
                &source,
                &[(&env.view, &env.sampler)],
                &[wgpu::TextureViewDimension::Cube],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(out.clip.xy,out.clip.w,out.clip.w);return out;}",
            )
            .await?,
        ));
        m.properties.side = Side::Back;
        m.properties.depth_write = false;
        m.properties.depth_test = false;
        let sky = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 1., 1.)?),
            Arc::new(Material::Shader(m)),
        )));
        let n = s.get_mut(sky)?;
        n.frustum_culled = false;
        n.render_order = -10000;
        self.sky = Some(sky);
        let lucy = ply("ply/binary/Lucy100k.ply").await?;
        for (material, x) in materials.into_iter().zip([0., -1500., 1500.]) {
            let h = s.insert(NodeKind::Mesh(Mesh::new(lucy.clone(), material)));
            let n = s.get_mut(h)?;
            n.position.x = x;
            n.scale = Vector3::splat(1.5);
        }
        s.look_at(c, Vector3::ZERO)?;
        self.pointer = Some(Vector2::ZERO);
        Ok(())
    }
    async fn ply_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.background = Color::from_hex(0x72645b);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x72645b),
            near: 2.,
            far: 15.,
        });
        let mut ground = MeshPhongMaterial {
            specular: Color::from_hex(0x474747),
            ..Default::default()
        };
        ground.properties.color = Color::from_hex(0xcbcbcb);
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(40., 40., 1, 1)?),
            Arc::new(Material::Phong(ground)),
        )));
        let n = s.get_mut(plane)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.position.y = -0.5;
        n.receive_shadow = true;
        let mut standard = MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        };
        standard.properties.color = Color::from_hex(0x009cff);
        standard.properties.flat_shading = true;
        let standard = Arc::new(Material::Standard(standard));
        for (path, position, rotation, scale) in [
            (
                "ply/ascii/dolphins.ply",
                Vector3::new(0., -0.2, 0.3),
                -PI / 2.,
                0.001,
            ),
            (
                "ply/binary/Lucy100k.ply",
                Vector3::new(-0.2, -0.02, -0.2),
                0.,
                0.0006,
            ),
        ] {
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                ply(path).await?,
                standard.clone(),
            )));
            let n = s.get_mut(h)?;
            n.position = position;
            n.quaternion = Quaternion::from_rotation_x(rotation);
            n.scale = Vector3::splat(scale);
            n.cast_shadow = true;
            n.receive_shadow = true;
        }
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0x8d7c7c),
            ground: Color::from_hex(0x494966),
            intensity: 3.,
        }));
        s.get_mut(hemi)?.position = Vector3::Y;
        // addShadowedLight: a 2 × 2 orthographic shadow camera from 1 to 4.
        for (p, hex, intensity) in [
            (Vector3::new(1., 1., 1.), 0xffffff, 3.5),
            (Vector3::new(0.5, 1., -1.), 0xffd500, 3.),
        ] {
            let light = s.insert(NodeKind::Light(Light::Directional {
                color: Color::from_hex(hex),
                intensity,
                target: Vector3::ZERO,
            }));
            let n = s.get_mut(light)?;
            n.position = p;
            n.cast_shadow = true;
            n.shadow = crate::shadow::Shadow {
                map_size: Some(1024),
                near: 1.,
                far: 4.,
                extent: 1.,
                bias: -0.001,
                ..Default::default()
            };
        }
        let _ = c;
        Ok(())
    }
    /// Materials of a Collada scene, converted as ColladaComposer.buildMaterial does.
    async fn collada_nodes(
        s: &mut Scene,
        dae: DaeScene,
        base: &str,
        files: &[(String, Vec<u8>)],
    ) -> Result<Object3D> {
        let root = s.insert(NodeKind::Group);
        let mut textures: std::collections::HashMap<String, Arc<crate::material::Texture>> =
            Default::default();
        for mesh in dae.meshes {
            let mut g = BufferGeometry::default();
            g.set_attribute("position", vec3s(mesh.positions)?);
            if !mesh.normals.is_empty() {
                g.set_attribute("normal", vec3s(mesh.normals)?);
            }
            if !mesh.uvs.is_empty() {
                g.set_attribute(
                    "uv",
                    Attribute::F32(BufferAttribute::new(mesh.uvs, 2, false)?),
                );
            }
            g.groups = mesh
                .groups
                .iter()
                .map(|&(start, count, material_index)| Group {
                    start,
                    count,
                    material_index,
                })
                .collect();
            let mut materials = vec![];
            for m in &mesh.materials {
                let mut properties = MaterialProperties {
                    color: Color::from_srgb(m.color[0], m.color[1], m.color[2]),
                    opacity: m.opacity,
                    transparent: m.transparent,
                    side: if m.double_sided {
                        Side::Double
                    } else {
                        Side::Front
                    },
                    ..Default::default()
                };
                if let Some(path) = &m.map {
                    if !textures.contains_key(path) {
                        let bytes =
                            match files.iter().find(|(name, _)| name.ends_with(path.as_str())) {
                                Some((_, b)) => b.clone(),
                                None => fetch(&format!("{base}/{path}")).await?,
                            };
                        let mut t = decode_texture_image(&bytes).await?;
                        t.srgb = true;
                        t.wrap_s = Wrapping::Repeat;
                        t.wrap_t = Wrapping::Repeat;
                        t.mipmap_filter = Some(Filter::Linear);
                        textures.insert(path.clone(), Arc::new(t));
                    }
                    properties.map = textures.get(path).cloned();
                }
                let srgb = |c: [f64; 3]| Color::from_srgb(c[0], c[1], c[2]);
                materials.push(Arc::new(match m.shading {
                    Shading::Phong => {
                        let mut p = MeshPhongMaterial::default();
                        if let Some(c) = m.specular {
                            p.specular = srgb(c);
                        }
                        if let Some(c) = m.emissive {
                            p.emissive = srgb(c);
                        }
                        if let Some(v) = m.shininess {
                            p.shininess = v;
                        }
                        p.properties = properties;
                        Material::Phong(p)
                    }
                    Shading::Lambert => {
                        let mut p = MeshLambertMaterial::default();
                        if let Some(c) = m.emissive {
                            p.emissive = srgb(c);
                        }
                        p.properties = properties;
                        Material::Lambert(p)
                    }
                    Shading::Basic => Material::Basic(MeshBasicMaterial { properties }),
                }));
            }
            if materials.is_empty() {
                materials.push(Arc::new(Material::Phong(MeshPhongMaterial::default())));
            }
            let mut node_mesh = Mesh::new(Arc::new(g), materials[0].clone());
            node_mesh.materials = materials;
            let h = s.insert(NodeKind::Mesh(node_mesh));
            let (scale, rotation, translation) =
                Matrix4::from_cols_array(&mesh.matrix).to_scale_rotation_translation();
            let n = s.get_mut(h)?;
            n.position = translation;
            n.quaternion = rotation;
            n.scale = scale;
            s.add(root, h)?;
        }
        // Z_UP assets are turned upright by the scene's rotation; the unit scales it.
        let n = s.get_mut(root)?;
        if dae.z_up {
            n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        }
        n.scale = Vector3::splat(dae.unit);
        Ok(root)
    }
    async fn kmz(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.background = Color::from_hex(0x999999);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(0.5, 1., 0.5).normalize();
        let grid = super::interactive_scenes::grid_helper(50., 50, 0xffffff, 0x7b7b7b)?;
        s.insert(NodeKind::Line(grid));
        // KMZLoader: doc.kml's model link inside the zip, parsed by ColladaLoader.
        let files = unzip(&fetch(&format!("{ASSETS}/kmz/Box.kmz")).await?)?;
        let kml = files
            .iter()
            .find(|(name, _)| name == "doc.kml")
            .ok_or(Error::Asset("KMZ: no doc.kml".into()))?;
        let doc = parse_xml(&String::from_utf8_lossy(&kml.1))?;
        let href = doc
            .first_descendant("Placemark")
            .and_then(|p| p.first_descendant("Model"))
            .and_then(|m| m.first_descendant("Link"))
            .and_then(|l| l.first_descendant("href"))
            .map(|h| h.text())
            .ok_or(Error::Asset("KMZ: no model link".into()))?;
        let dae = files
            .iter()
            .find(|(name, _)| *name == href)
            .ok_or(Error::Asset("KMZ: missing model".into()))?;
        let scene = parse_collada(&String::from_utf8_lossy(&dae.1))?;
        let root = Self::collada_nodes(s, scene, "", &files).await?;
        s.get_mut(root)?.position.y = 0.5;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::ZERO);
        controls.update(s, c)?;
        self.controls = Some(controls);
        Ok(())
    }
    async fn collada(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.look_at(c, Vector3::new(0., 3., 0.))?;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 1.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 2.5,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(1., 1., 0.).normalize();
        let base = format!("{ASSETS}/collada/elf");
        let scene = parse_collada(&String::from_utf8_lossy(
            &fetch(&format!("{base}/elf.dae")).await?,
        ))?;
        self.elf = Some(Self::collada_nodes(s, scene, &base, &[]).await?);
        Ok(())
    }
    async fn exr(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let (width, height, texels) = decode_exr(&fetch(&format!("{ASSETS}/memorial.exr")).await?)?;
        // EXRLoader: HalfFloatType RGBA rows from the bottom, flipY false, linear filters.
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("EXR texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        r.queue.write_texture(
            texture.as_image_copy(),
            bytemuck::cast_slice(&texels),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 8),
                rows_per_image: Some(height),
            },
            texture.size(),
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("EXR sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // MeshBasicMaterial map with ReinhardToneMapping( exposure ) and the sRGB output,
        // written raw; u.custom[0].x is toneMappingExposure.
        let color = WgslFn::new(
            "exr_color",
            "fn exr_color(t:vec4<f32>)->vec4<f32>{let c=u.custom[0].x*t.rgb;let m=clamp(c/(vec3(1.0)+c),vec3(0.0),vec3(1.0));let e=select(1.055*pow(m,vec3(0.41666))-vec3(0.055),m*12.92,m<=vec3(0.0031308));return vec4(e,1.0);}",
            &[Type::Vec4],
            Type::Vec4,
        )?
        .call(&[crate::tsl::Texture::External(0).sample(uv())]);
        let program = ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(1)?,
            &[],
            &[(&view, &sampler)],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}",
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        m.uniforms[0] = [self.exposure, 0., 0., 0.];
        let quad = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(
                1.5 * width as f64 / height as f64,
                1.5,
                1,
                1,
            )?),
            Arc::new(Material::Shader(m)),
        )));
        self.exr_quad = Some(quad);
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -aspect,
            right: aspect,
            top: 1.,
            bottom: -1.,
            near: 0.,
            far: 1.,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::ZERO;
        self.exr_texture = Some(texture);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let t = self.time;
        let steps = ((t - self.last) * 60.).round().max(0.) as usize;
        self.last = t;
        match self.id {
            238 => {
                // mouseX = ( clientX - windowHalfX ) × 4, eased at 0.05 per frame.
                let (w, h, _) = viewport_css();
                let p = self.pointer.unwrap_or(Vector2::ZERO);
                let mouse = Vector2::new(
                    ((p.x + 1.) / 2. * w - w / 2.) * 4.,
                    ((1. - p.y) / 2. * h - h / 2.) * 4.,
                );
                let n = s.get_mut(c)?;
                for _ in 0..steps {
                    n.position.x += (mouse.x - n.position.x) * 0.05;
                    n.position.y += (-mouse.y - n.position.y) * 0.05;
                }
                s.look_at(c, Vector3::ZERO)?;
                // The background box stays centered on the camera, unrotated.
                if let Some(sky) = self.sky {
                    let p = s.get(c)?.position;
                    s.get_mut(sky)?.position = p;
                }
            }
            239 => {
                let timer = t * 0.5;
                s.get_mut(c)?.position = Vector3::new(timer.sin() * 2.5, 0.15, timer.cos() * 2.5);
                s.look_at(c, Vector3::new(0., -0.1, 0.))?;
            }
            241 => {
                if let Some(elf) = self.elf {
                    s.get_mut(elf)?.quaternion = Euler {
                        angles: Vector3::new(-PI / 2., 0., t * 0.5),
                        order: EulerOrder::XYZ,
                    }
                    .quaternion();
                }
            }
            242 => {
                // onWindowResize keeps the frustum height and follows the aspect.
                let (w, h, _) = viewport_css();
                if let NodeKind::Camera(Camera::Orthographic(o)) = &mut s.get_mut(c)?.kind {
                    let height = o.top - o.bottom;
                    o.left = -height * (w / h) / 2.;
                    o.right = height * (w / h) / 2.;
                }
                if let Some(quad) = self.exr_quad
                    && let NodeKind::Mesh(m) = &mut s.get_mut(quad)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0][0] = self.exposure;
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn gpu_pointer(&mut self, x: f64, y: f64) {
        if let Some(p) = &mut self.pointer {
            *p = Vector2::new(x, y);
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
        // The 'change' listener renders after each update().
        controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (242, 0) if (0. ..=4.).contains(&value) => self.exposure = value,
            _ => return Err(Error::Invalid("refraction/loaders parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
