//! The STL, PLY and OBJ exporter scenes, the matcap head and the physically
//! based lights from the pinned examples.
mod formats;
use super::controls_attributes::{CameraState, Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::interactive_scenes::grid_helper;
use super::refraction_loaders::formats::decode_exr;
use crate::{
    Error, Result,
    attribute::BufferAttribute,
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{surface::SurfaceNodes, *},
};
use formats::{Colors, Item};
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";
fn vec3s(data: Vec<f32>) -> Result<Attribute> {
    Ok(Attribute::F32(BufferAttribute::new(data, 3, false)?))
}
fn bind(t: &[crate::texture_gpu::GpuTexture]) -> Vec<(&wgpu::TextureView, &wgpu::Sampler)> {
    t.iter().map(|t| (&t.view, &t.sampler)).collect()
}
fn f32s(g: &BufferGeometry, name: &str) -> Option<Vec<f32>> {
    match g.attributes.get(name) {
        Some(Attribute::F32(a)) => Some(a.array().to_vec()),
        _ => None,
    }
}
/// The exported object's geometry data and world matrix.
struct Exported {
    node: Object3D,
    /// matrixWorld, refreshed in prepare().
    world: [f64; 16],
    positions: Vec<f32>,
    normals: Option<Vec<f32>>,
    uvs: Option<Vec<f32>>,
    colors: Option<Vec<u8>>,
    float_colors: Option<Vec<f32>>,
    index: Option<Vec<u32>>,
    name: String,
    points: bool,
}
impl Exported {
    fn new(s: &Scene, node: Object3D) -> Result<Self> {
        let n = s.get(node)?;
        let g = n.geometry().ok_or(Error::Invalid("exported geometry"))?;
        let points = matches!(n.kind, NodeKind::Points(_));
        Ok(Self {
            node,
            world: n.matrix_world.to_cols_array(),
            positions: f32s(g, "position").ok_or(Error::Invalid("positions"))?,
            normals: f32s(g, "normal"),
            uvs: f32s(g, "uv"),
            colors: None,
            float_colors: if points { f32s(g, "color") } else { None },
            index: g.index.clone(),
            name: String::new(),
            points,
        })
    }
    fn item(&self) -> Item<'_> {
        Item {
            world: self.world,
            positions: &self.positions,
            normals: self.normals.as_deref(),
            uvs: self.uvs.as_deref(),
            colors: self
                .colors
                .as_deref()
                .map(Colors::Unorm8)
                .or(self.float_colors.as_deref().map(Colors::F32)),
            index: self.index.as_deref(),
            name: &self.name,
            points: self.points,
        }
    }
}
/// webgl_lights_physical's luminous powers (lm) and irradiances (lx), in GUI order.
const BULB_POWERS: [f64; 8] = [110000., 3500., 1700., 800., 400., 180., 20., 0.];
const HEMI_IRRADIANCES: [f64; 11] = [
    0.0001, 0.002, 0.5, 3.4, 50., 100., 350., 400., 1000., 18000., 50000.,
];
struct Physical {
    bulb: Object3D,
    bulb_mesh: Object3D,
    hemi: Object3D,
    /// hemiIrradiance index, bulbPower index, exposure, shadows.
    params: [f64; 4],
}
pub(super) struct Demo {
    id: u32,
    time: f64,
    controls: Option<Controls>,
    exported: Vec<Exported>,
    export: Option<(String, Vec<u8>)>,
    material: Option<Arc<Material>>,
    matcap: Option<(Object3D, [f64; 3])>,
    physical: Option<Physical>,
    /// addGeometry( type ) requested by a button, applied in prepare().
    pending: Option<u32>,
    exposure: Option<f64>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            258 | 259 => (45., 0.1, 100., Vector3::new(4., 2., 4.)),
            260 => (70., 1., 1000., Vector3::new(0., 0., 400.)),
            261 => (40., 1., 100., Vector3::new(0., 0., 13.)),
            _ => (50., 0.1, 100., Vector3::new(-4., 2., 4.)),
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
            controls: None,
            exported: vec![],
            export: None,
            material: None,
            matcap: None,
            physical: None,
            pending: None,
            exposure: None,
        };
        match id {
            258 | 259 => d.box_scene(s, c)?,
            260 => {
                s.insert(NodeKind::Light(Light::Ambient {
                    color: Color::WHITE,
                    intensity: 1.,
                }));
                let light = s.insert(NodeKind::Light(Light::Directional {
                    color: Color::WHITE,
                    intensity: 2.5,
                    target: Vector3::ZERO,
                }));
                s.get_mut(light)?.position = Vector3::new(0., 1., 1.);
                let mut m = MeshLambertMaterial::default();
                m.properties.color = Color::from_hex(0x00cc00);
                d.material = Some(Arc::new(Material::Lambert(m)));
                d.add_geometry(s, 1)?;
                let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
                controls.update(s, c)?;
                d.controls = Some(controls);
            }
            261 => d.matcap_scene(s, c, r).await?,
            _ => d.physical_scene(s, c, r).await?,
        }
        Ok(d)
    }
    /// The STL and PLY exporters' box on a shadowed, fogged ground with a grid.
    fn box_scene(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let ply = self.id == 259;
        s.background = Color::from_hex(0xa0a0a0);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0xa0a0a0),
            near: 4.,
            far: 20.,
        });
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x444444),
            intensity: 3.,
        }));
        s.get_mut(hemi)?.position = Vector3::new(0., 20., 0.);
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(0., 20., 10.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            near: 0.5,
            far: 500.,
            extent: 2.,
            ..Default::default()
        };
        let mut ground = MeshPhongMaterial::default();
        ground.properties.color = Color::from_hex(if ply { 0xcbcbcb } else { 0xbbbbbb });
        ground.properties.depth_write = false;
        let g = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(40., 40., 1, 1)?),
            Arc::new(Material::Phong(ground)),
        )));
        let n = s.get_mut(g)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.receive_shadow = true;
        let mut grid = grid_helper(40., 20, 0x000000, 0x000000)?;
        if let Material::Line(m) = Arc::make_mut(&mut grid.material) {
            m.properties.opacity = 0.2;
            m.properties.transparent = true;
        }
        s.insert(NodeKind::Line(grid));
        let mut geometry = BoxGeometry::build(1., 1., 1.)?;
        let mut material = MeshPhongMaterial::default();
        let mut colors = None;
        if ply {
            // Uint8BufferAttribute( positions, 3, true ): setX( 0.5 ) stores round( 127.5 ).
            let positions = f32s(&geometry, "position").ok_or(Error::Invalid("box"))?;
            let bytes: Vec<u8> = positions
                .iter()
                .map(|&v| if v > 0. { 128 } else { 0 })
                .collect();
            geometry.set_attribute(
                "color",
                vec3s(bytes.iter().map(|&b| b as f32 / 255.).collect())?,
            );
            material.properties.vertex_colors = true;
            colors = Some(bytes);
        } else {
            material.properties.color = Color::from_hex(0x00ff00);
        }
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            Arc::new(Material::Phong(material)),
        )));
        let n = s.get_mut(mesh)?;
        n.cast_shadow = true;
        n.position.y = 0.5;
        s.update()?;
        let mut exported = Exported::new(s, mesh)?;
        exported.colors = colors;
        self.exported = vec![exported];
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        Ok(())
    }
    /// addGeometry( type ): the OBJ exporter's selectable scene contents.
    fn add_geometry(&mut self, s: &mut Scene, kind: u32) -> Result<()> {
        for e in std::mem::take(&mut self.exported) {
            s.dispose(e.node)?;
        }
        let material = self
            .material
            .clone()
            .ok_or(Error::Invalid("obj material"))?;
        let triangle = || -> Result<Arc<BufferGeometry>> {
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                vec3s(vec![-50., -50., 0., 50., -50., 0., 50., 50., 0.])?,
            );
            g.compute_vertex_normals()?;
            Ok(Arc::new(g))
        };
        let cube = || -> Result<Arc<BufferGeometry>> {
            Ok(Arc::new(BoxGeometry::build(100., 100., 100.)?))
        };
        let cylinder = || -> Result<Arc<BufferGeometry>> {
            Ok(Arc::new(CylinderGeometry::build(
                50., 50., 100., 30, 1, false, 0., TAU,
            )?))
        };
        let mut nodes = vec![];
        match kind {
            1 => nodes.push((triangle()?, 0.)),
            2 => nodes.push((cube()?, 0.)),
            3 => nodes.push((cylinder()?, 0.)),
            4 | 5 => {
                nodes.push((triangle()?, -200.));
                nodes.push((cube()?, 0.));
                nodes.push((cylinder()?, 200.));
            }
            _ => {
                let mut g = BufferGeometry::default();
                g.set_attribute(
                    "position",
                    vec3s(vec![0., 0., 0., 100., 0., 0., 100., 100., 0., 0., 100., 0.])?,
                );
                g.set_attribute(
                    "color",
                    vec3s(vec![0.5, 0., 0., 0.5, 0., 0., 0., 0.5, 0., 0., 0.5, 0.])?,
                );
                let mut m = PointsMaterial {
                    size: 10.,
                    ..Default::default()
                };
                m.properties.vertex_colors = true;
                let h = s.insert(NodeKind::Points(Points {
                    geometry: Arc::new(g),
                    material: Arc::new(Material::Points(m)),
                }));
                s.update()?;
                s.update()?;
                let mut e = Exported::new(s, h)?;
                e.name = "point cloud".into();
                self.exported.push(e);
                return Ok(());
            }
        }
        for (g, x) in nodes {
            let h = s.insert(NodeKind::Mesh(Mesh::new(g, material.clone())));
            let n = s.get_mut(h)?;
            n.position.x = x;
            if kind == 5 {
                n.quaternion = Quaternion::from_rotation_y(PI / 4.);
            }
            s.update()?;
            self.exported.push(Exported::new(s, h)?);
        }
        Ok(())
    }
    /// The head with MeshMatcapNodeMaterial: the EXR matcap sampled by matcapUV
    /// from the normal-mapped view normal, times the color, unlit.
    async fn matcap_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 1.;
        let (width, height, texels) =
            decode_exr(&fetch(&format!("{ASSETS}/matcaps/040full.exr")).await?)?;
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("EXR matcap"),
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
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        let view = texture.create_view(&Default::default());
        // EXRLoader: linear filters, no mipmaps, clamped.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let matcap = WgslFn::new(
            "matcap_color",
            "fn matcap_color(n:vec3<f32>,color:vec4<f32>)->vec4<f32>{let v=normalize(-fragment_surface.view_position);let x=normalize(vec3(v.z,0.0,-v.x));let y=cross(v,x);let uv=vec2(dot(x,n),dot(y,n))*0.495+0.5;return vec4(textureSample(tsl_texture_0,tsl_sampler_0,uv).rgb*color.rgb,color.a);}",
            &[Type::Vec3, Type::Vec4],
            Type::Vec4,
        )?
        .call(&[normal_view(), uniform(1, Type::Vec4)]);
        let program = SurfaceNodes {
            output: Some(matcap),
            ..Default::default()
        }
        .build_with_texture_types(r, &[], &[(&view, &sampler, Type::Texture)])
        .await?;
        let mut normal = decode_texture_image(
            &fetch(&format!(
                "{ASSETS}/LeePerrySmith/Infinite-Level_02_Tangent_SmoothUV.jpg"
            ))
            .await?,
        )
        .await?;
        normal.srgb = false;
        normal.flip_y = true;
        normal.mipmap_filter = Some(Filter::Linear);
        let mut m = MeshLambertMaterial {
            normal_map: Some(Arc::new(normal)),
            ..Default::default()
        };
        m.properties.vertex_program = Some(Arc::new(program));
        m.properties.vertex_uniforms[1] = [1., 1., 1., 1.];
        let (geometry, scale) = super::helpers_formats::head().await?;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            geometry,
            Arc::new(Material::Lambert(m)),
        )));
        let n = s.get_mut(mesh)?;
        n.scale = scale;
        n.position.y = -0.25;
        self.matcap = Some((mesh, [1.; 3]));
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        Ok(())
    }
    async fn physical_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Reinhard;
        let bulb = s.insert(NodeKind::Light(Light::Point {
            color: Color::from_hex(0xffee88),
            intensity: 1.,
            distance: 100.,
            decay: 2.,
        }));
        let n = s.get_mut(bulb)?;
        n.position = Vector3::new(0., 2., 0.);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            near: 0.5,
            far: 500.,
            ..Default::default()
        };
        let mut bulb_material = MeshStandardMaterial {
            emissive: Color::from_hex(0xffffee),
            energy_conservation: true,
            ..Default::default()
        };
        bulb_material.properties.color = Color::BLACK;
        let bulb_mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.02, 16, 8)?),
            Arc::new(Material::Standard(bulb_material)),
        )));
        s.add(bulb, bulb_mesh)?;
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0xddeeff),
            ground: Color::from_hex(0x0f0e0d),
            intensity: 0.02,
        }));
        s.get_mut(hemi)?.position = Vector3::Y;
        // Textures: anisotropy 4, mipmapped, repeat-wrapped where the example repeats.
        let texture = |path: &'static str, srgb: bool, repeat: bool| async move {
            let mut t = decode_texture_image(&fetch(&format!("{ASSETS}/{path}")).await?).await?;
            t.srgb = srgb;
            t.anisotropy = 4;
            t.mipmap_filter = Some(Filter::Linear);
            if repeat {
                t.wrap_s = Wrapping::Repeat;
                t.wrap_t = Wrapping::Repeat;
            }
            r.upload_texture(&Arc::new(t))
        };
        let floor = [
            texture("hardwood2_diffuse.jpg", true, true).await?,
            texture("hardwood2_bump.jpg", false, true).await?,
            texture("hardwood2_roughness.jpg", false, true).await?,
        ];
        let brick = [
            texture("tsl-next/textures/brick_diffuse.jpg", true, true).await?,
            texture("tsl-environment/textures/brick_bump.jpg", false, true).await?,
        ];
        let earth = [
            texture("earth_atmos_2048.jpg", true, false).await?,
            texture("earth_specular_2048.jpg", true, false).await?,
        ];
        // TextureLoader images are flipped on upload: sample ( u, 1 − v ) of the repeated UV.
        let tex = |i: usize, repeat: [f32; 2], coordinate: crate::tsl::Node| {
            crate::tsl::Texture::External(i).sample(vec2(
                coordinate.x() * float(repeat[0]),
                float(1.) - coordinate.y() * float(repeat[1]),
            ))
        };
        let floor_graph = SurfaceNodes {
            color: Some(tex(0, [10., 24.], uv()).rgb()),
            roughness: Some(float(0.8) * tex(2, [10., 24.], uv()).y()),
            normal: Some(crate::tsl::surface::bump_map(
                |c| tex(1, [1., 1.], c).x(),
                vec2(uv().x() * float(10.), uv().y() * float(24.)),
                float(1.),
            )),
            ..Default::default()
        };
        let brick_graph = SurfaceNodes {
            color: Some(tex(0, [1., 1.], uv()).rgb()),
            normal: Some(crate::tsl::surface::bump_map(
                |c| tex(1, [1., 1.], c).x(),
                uv(),
                float(1.),
            )),
            ..Default::default()
        };
        let earth_graph = SurfaceNodes {
            color: Some(tex(0, [1., 1.], uv()).rgb()),
            metalness: Some(tex(1, [1., 1.], uv()).swizzle("z")),
            ..Default::default()
        };
        // The surface graphs leave positions unchanged: casters use a plain shadow program.
        let shadow = Arc::new(crate::tsl::surface::shadow_program(r, None, None).await?);
        let standard = |roughness: f64, metalness: f64, program: crate::shader::ShaderProgram| {
            // WebGPU's PhysicalLightingModel conserves specular energy, direct and indirect.
            let mut m = MeshStandardMaterial {
                roughness,
                metalness,
                energy_conservation: true,
                ..Default::default()
            };
            m.properties.vertex_program = Some(Arc::new(program));
            m.properties.shadow_program = Some(shadow.clone());
            Arc::new(Material::Standard(m))
        };

        let floor_material = standard(0.8, 0.2, floor_graph.build(r, &[], &bind(&floor)).await?);
        let cube_material = standard(0.7, 0.2, brick_graph.build(r, &[], &bind(&brick)).await?);
        let ball_material = standard(0.5, 1., earth_graph.build(r, &[], &bind(&earth)).await?);
        let floor_mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(20., 20., 1, 1)?),
            floor_material,
        )));
        let n = s.get_mut(floor_mesh)?;
        n.receive_shadow = true;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        let ball = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(0.25, 32, 32)?),
            ball_material,
        )));
        let n = s.get_mut(ball)?;
        n.position = Vector3::new(1., 0.25, 1.);
        n.quaternion = Quaternion::from_rotation_y(PI);
        n.cast_shadow = true;
        let box_geometry = Arc::new(BoxGeometry::build(0.5, 0.5, 0.5)?);
        for p in [
            Vector3::new(-0.5, 0.25, -1.),
            Vector3::new(0., 0.25, -5.),
            Vector3::new(7., 0.25, 0.),
        ] {
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                box_geometry.clone(),
                cube_material.clone(),
            )));
            let n = s.get_mut(h)?;
            n.position = p;
            n.cast_shadow = true;
        }
        self.physical = Some(Physical {
            bulb,
            bulb_mesh,
            hemi,
            params: [0., 4., 0.68, 1.],
        });
        let mut controls = Controls::new(None, (1., 20.), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D) -> Result<()> {
        if let Some(e) = self.exposure.take() {
            s.exposure = e;
        }
        if let Some(kind) = self.pending.take() {
            self.add_geometry(s, kind)?;
        }
        s.update()?;
        for e in &mut self.exported {
            e.world = s.get(e.node)?.matrix_world.to_cols_array();
        }
        if let Some((mesh, color)) = self.matcap
            && let NodeKind::Mesh(m) = &mut s.get_mut(mesh)?.kind
        {
            Arc::make_mut(&mut m.materials[0])
                .properties_mut()
                .vertex_uniforms[1] = [color[0] as f32, color[1] as f32, color[2] as f32, 1.];
        }
        if let Some(p) = &self.physical {
            let [hemi, bulb, exposure, shadows] = p.params;
            s.exposure = exposure.powf(5.);
            // PointLight.power = intensity × 4π.
            let intensity = BULB_POWERS[bulb as usize] / (4. * PI);
            let n = s.get_mut(p.bulb)?;
            n.cast_shadow = shadows > 0.5;
            n.position.y = (self.time * 0.5).cos() * 0.75 + 1.25;
            if let NodeKind::Light(Light::Point { intensity: i, .. }) = &mut n.kind {
                *i = intensity;
            }
            if let NodeKind::Mesh(m) = &mut s.get_mut(p.bulb_mesh)?.kind
                && let Material::Standard(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.emissive = Color(Color::from_hex(0xffffee).0 * (intensity / 0.02f64.powi(2)));
            }
            if let NodeKind::Light(Light::Hemisphere { intensity: i, .. }) =
                &mut s.get_mut(p.hemi)?.kind
            {
                *i = HEMI_IRRADIANCES[hemi as usize];
            }
        }
        Ok(())
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
            // The matcap example disables zoom.
            if self.id == 261 {
                return Ok(());
            }
            controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            // The OBJ and matcap examples disable panning.
            if matches!(self.id, 260 | 261) {
                return Ok(());
            }
            controls.pan(&camera, dx, dy, height);
        } else {
            controls.rotate(dx, dy, height);
        }
        controls.update(s, c)
    }
    /// Export buttons store the file for `take_export`; other controls change the scene.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match (self.id, index) {
            (258, 0 | 1) | (259, 0..=2) | (260, 6) => {
                let items = self.exported.iter().map(Exported::item).collect::<Vec<_>>();
                self.export = Some(match (self.id, index) {
                    (258, 0) => ("box.stl".into(), formats::stl(&items, false)),
                    (258, _) => ("box.stl".into(), formats::stl(&items, true)),
                    (259, 0) => ("box.ply".into(), formats::ply(&items, false, false)),
                    (259, 1) => ("box.ply".into(), formats::ply(&items, true, false)),
                    (259, _) => ("box.ply".into(), formats::ply(&items, true, true)),
                    _ => ("object.obj".into(), formats::obj(&items)),
                });
            }
            (260, 0..=5) => self.pending = Some(index as u32 + 1),
            (261, 0) => {
                let c = Color::from_hex(value as u32).0;
                if let Some((_, color)) = &mut self.matcap {
                    *color = c.to_array();
                }
            }
            (261, 1) => self.exposure = Some(value as f64),
            (262, 0..=3) => {
                let p = self.physical.as_mut().ok_or(Error::Invalid("physical"))?;
                p.params[index] = value as f64;
            }
            _ => return Err(Error::Invalid("exporters/matcap parameter")),
        }
        Ok(())
    }
    pub fn take_export(&mut self) -> Option<(String, Vec<u8>)> {
        self.export.take()
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
