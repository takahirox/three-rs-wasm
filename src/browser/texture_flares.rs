//! Raycast-painted canvas textures, a partially updated 3D cloud, cube mips
//! rendered in place, lens flares and the car materials.
use super::controls_attributes::{CameraState, Controls, camera_state, viewport_css};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use crate::mipmap::MipGenerator;
use crate::shader::ShaderProgram;
use crate::texture_gpu::GpuTexture;
use crate::tsl::{NodeMaterial, Type, WgslFn, uniform, uv};
use crate::{
    Error, Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*,
    renderer::*, scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::JsCast;

const ASSETS: &str = "/web/gallery/assets";
const PROJECT: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
fn random(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed as f64 / 4294967296.
}
fn euler(x: f64, y: f64, z: f64) -> Quaternion {
    Euler {
        angles: Vector3::new(x, y, z),
        order: EulerOrder::XYZ,
    }
    .quaternion()
}
fn document() -> Result<web_sys::Document> {
    web_sys::window()
        .and_then(|w| w.document())
        .ok_or(Error::Invalid("document"))
}
fn canvas_2d(
    width: u32,
    height: u32,
) -> Result<(
    web_sys::HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
)> {
    let canvas: web_sys::HtmlCanvasElement = document()?
        .create_element("canvas")
        .map_err(|_| Error::Invalid("canvas"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("canvas"))?;
    canvas.set_width(width);
    canvas.set_height(height);
    let context = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|c| c.dyn_into().ok())
        .ok_or(Error::Invalid("2d context"))?;
    Ok((canvas, context))
}
/// three's Wrapping for the WRAPPING menu: Repeat, ClampToEdge, MirroredRepeat.
fn wrapping(index: usize) -> Wrapping {
    [Wrapping::Repeat, Wrapping::Clamp, Wrapping::Mirror][index.min(2)]
}
/// `Texture.setUvTransform( offset, repeat, rotation, center 0 )` as two columns.
fn uv_transform(p: &[f64; 7]) -> [[f32; 4]; 2] {
    let (s, c) = p[6].sin_cos();
    let (sx, sy) = (p[4], p[5]);
    [
        [
            (sx * c) as f32,
            (-sy * s) as f32,
            (sx * s) as f32,
            (sy * c) as f32,
        ],
        [p[2] as f32, p[3] as f32, 0., 0.],
    ]
}
/// One texture sharing the drawing canvas: GPU copies from the canvas, as
/// WebGL's texImage2D( canvas ), with regenerated mipmaps.
struct CanvasTexture {
    texture: GpuTexture,
    mips: MipGenerator,
    /// One program per sampler: the circle's nine wrapS × wrapT pairs share the
    /// texture, as changing a Texture's wrapping keeps its image.
    materials: Vec<Arc<Material>>,
    stale: bool,
}
/// webgl_raycaster_texture.
struct Painter {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    image: web_sys::HtmlImageElement,
    /// crossRadius, crossMax, crossMin, crossThickness.
    cross: [f64; 4],
    position: (f64, f64),
    meshes: [Object3D; 3],
    /// The cube's, the plane's and the circle's textures.
    textures: [CanvasTexture; 3],
    pointer: Option<(f64, f64)>,
    /// wrapS, wrapT, offsetX, offsetY, repeatX, repeatY, rotation.
    params: [f64; 7],
}
/// webgl_texture3d_partialupdate.
struct Cloud {
    mesh: Object3D,
    texture: wgpu::Texture,
    seed: u32,
    current: usize,
    previous: f64,
    frame: f64,
    /// threshold, opacity, range, steps.
    params: [f64; 4],
}
/// webgpu_lensflares: one LensflareMesh on a point light.
struct Flare {
    light: Object3D,
    color: Color,
    converted: bool,
    temp: wgpu::Texture,
    occlusion: wgpu::Texture,
    buffers: Vec<wgpu::Buffer>,
    binds: Vec<wgpu::BindGroup>,
}
struct Flares {
    flares: Vec<Flare>,
    pipelines: [wgpu::RenderPipeline; 3],
    /// Element sizes and distances: textureFlare0 then four textureFlare3.
    elements: [(f64, f64); 5],
    /// up, down, left, right, forward, back, pitchUp, pitchDown, yawLeft,
    /// yawRight, rollLeft, rollRight.
    state: [f64; 12],
    format: wgpu::TextureFormat,
}
/// webgl_materials_car.
struct Car {
    body: Object3D,
    details: Vec<Object3D>,
    glass: Object3D,
    wheels: Vec<(Object3D, Quaternion)>,
    grid: Object3D,
    colors: [u32; 3],
    applied: [u32; 3],
}
pub struct Demo {
    id: u32,
    time: f64,
    last: f64,
    controls: Option<Controls>,
    painter: Option<Painter>,
    cloud: Option<Cloud>,
    flares: Option<Flares>,
    car: Option<Car>,
    _cube: Option<(wgpu::Texture, GpuTexture)>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let (fov, near, far, position) = match id {
            298 => (45., 1., 1000., Vector3::new(-30., 40., 50.)),
            299 => (60., 0.1, 100., Vector3::new(0., 0., 1.5)),
            300 => (50., 1., 10000., Vector3::new(0., 0., 500.)),
            301 => (40., 1., 15000., Vector3::new(0., 0., 250.)),
            _ => (40., 0.1, 100., Vector3::new(4.25, 1.4, -4.5)),
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
            painter: None,
            cloud: None,
            flares: None,
            car: None,
            _cube: None,
        };
        match id {
            298 => d.painter_scene(s, c, r).await?,
            299 => d.cloud_scene(s, c, r).await?,
            300 => d.mipmap_scene(s, c, r).await?,
            301 => d.flare_scene(s, r).await?,
            _ => d.car_scene(s, c, r).await?,
        }
        Ok(d)
    }
    /// A canvas texture, its mip generator and the MeshBasicMaterial-like program
    /// sampling it through the map's UV transform (uniforms 0 and 1).
    async fn canvas_texture(
        r: &Renderer,
        width: u32,
        height: u32,
        wraps: &[(Wrapping, Wrapping)],
    ) -> Result<CanvasTexture> {
        let mut image =
            Texture::from_rgba(width, height, vec![0; (width * height * 4) as usize], true)?;
        image.mipmap_filter = Some(Filter::Linear);
        let texture = r.upload_texture(&Arc::new(image))?;
        let mips = MipGenerator::new(&r.device, &texture.texture)?;
        let address = |w: Wrapping| match w {
            Wrapping::Repeat => wgpu::AddressMode::Repeat,
            Wrapping::Clamp => wgpu::AddressMode::ClampToEdge,
            Wrapping::Mirror => wgpu::AddressMode::MirrorRepeat,
        };
        let mut materials = vec![];
        for &(wrap_s, wrap_t) in wraps {
            let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address(wrap_s),
                address_mode_v: address(wrap_t),
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                lod_max_clamp: 32.,
                ..Default::default()
            });
            let color = WgslFn::new(
                "canvas_map",
                "fn canvas_map(uv:vec2<f32>,a:vec4<f32>,b:vec4<f32>)->vec4<f32>{let t=mat3x3(vec3(a.xy,0.0),vec3(a.zw,0.0),vec3(b.xy,1.0))*vec3(uv,1.0);return textureSample(tsl_texture_0,tsl_sampler_0,t.xy);}",
                &[Type::Vec2, Type::Vec4, Type::Vec4],
                Type::Vec4,
            )?
            .call(&[uv(), uniform(0, Type::Vec4), uniform(1, Type::Vec4)]);
            let program = ShaderProgram::with_projection(
                r,
                &NodeMaterial::new(color).wgsl(1)?,
                &[],
                &[(&texture.view, &sampler)],
                PROJECT,
            )
            .await?;
            let mut m = ShaderMaterial::new(Arc::new(program));
            m.uniforms[0] = [1., 0., 0., 1.];
            m.uniforms[1] = [0., 0., 0., 0.];
            materials.push(Arc::new(Material::Shader(m)));
        }
        Ok(CanvasTexture {
            texture,
            mips,
            materials,
            stale: true,
        })
    }
    async fn painter_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.background = Color::from_hex(0xeeeeee);
        s.look_at(c, Vector3::ZERO)?;
        let image = web_sys::HtmlImageElement::new().map_err(|_| Error::Invalid("image"))?;
        image.set_cross_origin(Some(""));
        image.set_src(&super::asset_url(&format!(
            "{ASSETS}/tsl-procedural/uv_grid_opengl.jpg"
        ))?);
        wasm_bindgen_futures::JsFuture::from(image.decode())
            .await
            .map_err(|_| Error::Asset("uv_grid_opengl.jpg".into()))?;
        let (width, height) = (image.natural_width(), image.natural_height());
        let (canvas, context) = canvas_2d(width, height)?;
        let radius = (width as f64).min(height as f64 / 30.).ceil();
        // The original's literal, not FRAC_1_SQRT_2.
        #[allow(clippy::approx_constant)]
        let max = (0.70710678 * radius).ceil();
        let (min, thickness) = ((max / 10.).ceil(), (max / 10.).ceil());
        let cube_texture =
            Self::canvas_texture(r, width, height, &[(Wrapping::Repeat, Wrapping::Repeat)]).await?;
        let plane_texture =
            Self::canvas_texture(r, width, height, &[(Wrapping::Mirror, Wrapping::Mirror)]).await?;
        let wraps: Vec<_> = (0..9).map(|i| (wrapping(i / 3), wrapping(i % 3))).collect();
        let circle_texture = Self::canvas_texture(r, width, height, &wraps).await?;
        // The geometries' UVs are scaled as in the original.
        let scale_uv =
            |mut g: BufferGeometry, f: &dyn Fn(f32) -> f32| -> Result<Arc<BufferGeometry>> {
                if let Some(Attribute::F32(a)) = g.attributes.get_mut("uv") {
                    for v in a.array_mut() {
                        *v = f(*v);
                    }
                }
                Ok(Arc::new(g))
            };
        let mut meshes = vec![];
        for (geometry, material, position) in [
            (
                scale_uv(BoxGeometry::build(20., 20., 20.)?, &|v| v * 2.)?,
                cube_texture.materials[0].clone(),
                Vector3::new(4., -5., 0.),
            ),
            (
                scale_uv(PlaneGeometry::build(25., 25., 1, 1)?, &|v| v * 2.)?,
                plane_texture.materials[0].clone(),
                Vector3::new(-16., -5., 0.),
            ),
            (
                scale_uv(CircleGeometry::build(25., 40, 0., PI * 2.)?, &|v| {
                    (v - 0.25) * 2.
                })?,
                circle_texture.materials[0].clone(),
                Vector3::new(24., -5., 0.),
            ),
        ] {
            let h = s.insert(NodeKind::Mesh(Mesh::new(geometry, material)));
            s.get_mut(h)?.position = position;
            meshes.push(h);
        }
        let mut painter = Painter {
            canvas,
            context,
            image,
            cross: [radius, max, min, thickness],
            position: (0., 0.),
            meshes: [meshes[0], meshes[1], meshes[2]],
            textures: [cube_texture, plane_texture, circle_texture],
            pointer: None,
            params: [0., 0., 0., 0., 1., 1., 0.],
        };
        painter.draw()?;
        self.painter = Some(painter);
        Ok(())
    }
    async fn cloud_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        // The sky: a 1 × 32 canvas gradient, drawn by the browser.
        let (_, context) = canvas_2d(1, 32)?;
        let gradient = context.create_linear_gradient(0., 0., 0., 32.);
        for (stop, color) in [(0., "#014a84"), (0.5, "#0561a0"), (1., "#437ab6")] {
            gradient
                .add_color_stop(stop, color)
                .map_err(|_| Error::Invalid("gradient"))?;
        }
        context.set_fill_style_canvas_gradient(&gradient);
        context.fill_rect(0., 0., 1., 32.);
        let pixels = context
            .get_image_data(0., 0., 1., 32.)
            .map_err(|_| Error::Invalid("gradient pixels"))?
            .data()
            .0;
        let mut sky = Texture::from_rgba(1, 32, pixels, true)?;
        sky.mipmap_filter = Some(Filter::Linear);
        let mut m = MeshBasicMaterial::default();
        m.properties.map = Some(Arc::new(sky));
        m.properties.side = Side::Back;
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(10., 32, 16)?),
            Arc::new(Material::Basic(m)),
        )));
        // Data3DTexture( 128³, RedFormat ), zero-filled, linear filtering.
        let size = 128u32;
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("cloud texture"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: size,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // The raymarching RawShaderMaterial. gl_FragCoord counts rows from the
        // bottom; the frame seeds the per-pixel start jitter.
        let color = WgslFn::new(
            "cloud_march",
            CLOUD,
            &[Type::Vec4, Type::Vec4, Type::Vec4, Type::Vec2],
            Type::Vec4,
        )?
        .call(&[
            uniform(0, Type::Vec4),
            uniform(1, Type::Vec4),
            uniform(2, Type::Vec4),
            crate::tsl::screen_size(),
        ]);
        let program = ShaderProgram::with_projection_and_dimensions(
            r,
            &NodeMaterial::new(color).wgsl_with_texture_types(&[Type::Texture3D], &[])?,
            &[(&view, &sampler)],
            &[wgpu::TextureViewDimension::D3],
            PROJECT,
        )
        .await?;
        let mut m = ShaderMaterial::new(Arc::new(program));
        let base = Color::from_hex(0x798aa0).0;
        m.uniforms[0] = [0.25, 0.25, 0.1, 100.];
        m.uniforms[1] = [base.x as f32, base.y as f32, base.z as f32, 0.];
        m.properties.side = Side::Back;
        m.properties.transparent = true;
        let mesh = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 1., 1.)?),
            Arc::new(Material::Shader(m)),
        )));
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        self.controls = Some(controls);
        self.cloud = Some(Cloud {
            mesh,
            texture,
            seed: 186,
            current: 0,
            previous: 0.,
            frame: 0.,
            params: [0.25, 0.25, 0.1, 100.],
        });
        Ok(())
    }
    async fn mipmap_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let mut faces = vec![];
        for name in ["px", "nx", "py", "ny", "pz", "nz"] {
            let mut t =
                decode_texture_image(&fetch(&format!("{ASSETS}/cube/Park3Med/{name}.jpg")).await?)
                    .await?;
            t.srgb = true;
            faces.push(t);
        }
        let source = GpuTexture::from_cube_rgba(
            r,
            &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
        )?;
        // allocateCubemapRenderTarget( 512 ): HalfFloat, ten levels; renderToCubeTexture
        // fills levels 0 to 8, each face and level tinted by the level.
        let size = 512u32;
        let levels = 10;
        let cube = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("mip-tinted cube"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 6,
            },
            mip_level_count: levels,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cube mip filter"),
            source: wgpu::ShaderSource::Wgsl(CUBE_FILTER.into()),
        });
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::Cube,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        let pipeline =
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("cube mip filter"),
                    layout: Some(&r.device.create_pipeline_layout(
                        &wgpu::PipelineLayoutDescriptor {
                            label: None,
                            bind_group_layouts: &[&layout],
                            push_constant_ranges: &[],
                        },
                    )),
                    vertex: wgpu::VertexState {
                        module: &module,
                        entry_point: Some("vs"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module,
                        entry_point: Some("fs"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::TextureFormat::Rgba16Float.into())],
                    }),
                    primitive: Default::default(),
                    depth_stencil: None,
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                });
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let mut buffers = vec![];
        for mip in 0..(size as f64).log2().floor() as u32 {
            for face in 0..6u32 {
                let buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
                    label: None,
                    size: 16,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                r.queue.write_buffer(
                    &buffer,
                    0,
                    bytemuck::cast_slice(&[face as f32, mip as f32, (size >> mip) as f32, 0.]),
                );
                let bind = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&source.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&source.sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: buffer.as_entire_binding(),
                        },
                    ],
                });
                let view = cube.create_view(&wgpu::TextureViewDescriptor {
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_mip_level: mip,
                    mip_level_count: Some(1),
                    base_array_layer: face,
                    array_layer_count: Some(1),
                    ..Default::default()
                });
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("cube mip filter"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_pipeline(&pipeline);
                pass.set_bind_group(0, &bind, &[]);
                pass.draw(0..3, 0..1);
                drop(pass);
                buffers.push(buffer);
            }
        }
        r.queue.submit([encoder.finish()]);
        let cube_view = cube.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let cube_sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // MeshBasicMaterial( { envMap } ): the reflection vector is computed per
        // vertex, and CubeTextures flip x.
        let sphere = Arc::new(SphereGeometry::build(100., 128, 128)?);
        for (view, sampler, flip, x) in [
            (&source.view, &source.sampler, -1f32, -100.),
            (&cube_view, &cube_sampler, 1., 100.),
        ] {
            let color = WgslFn::new(
                "env_reflect",
                "fn env_reflect(flip:f32)->vec4<f32>{let r=fragment_surface.tangent.xyz;return vec4(textureSample(tsl_texture_0,tsl_sampler_0,vec3(flip*r.x,r.yz)).rgb,1.0);}",
                &[Type::Float],
                Type::Vec4,
            )?
            .call(&[crate::tsl::float(flip)]);
            let program = ShaderProgram::with_projection_and_dimensions(
                r,
                &NodeMaterial::new(color).wgsl_with_texture_types(&[Type::TextureCube], &[])?,
                &[(view, sampler)],
                &[wgpu::TextureViewDimension::Cube],
                "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;let n=normalize((transpose(u.view)*vec4(surface.normal,0.0)).xyz);out.tangent=vec4(reflect(normalize(surface.position-u.camera.xyz),n),0.0);return out;}",
            )
            .await?;
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                sphere.clone(),
                Arc::new(Material::Shader(ShaderMaterial::new(Arc::new(program)))),
            )));
            s.get_mut(h)?.position.x = x;
        }
        drop(buffers);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI / 1.5, true);
        controls.min_polar = PI / 4.;
        self.controls = Some(controls);
        let _ = c;
        self._cube = Some((cube, source));
        Ok(())
    }
    async fn flare_scene(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        // setHSL( 0.51, 0.4, 0.01, SRGBColorSpace ), then the fog.
        let background = {
            let c = super::shapes_lights::hsl(0.51, 0.4, 0.01).0;
            let f = crate::math::srgb_to_linear;
            Color::linear(f(c.x), f(c.y), f(c.z))
        };
        s.background = background;
        s.fog = Some(Fog::Linear {
            color: background,
            near: 3500.,
            far: 15000.,
        });
        let box_geometry = Arc::new(BoxGeometry::build(250., 250., 250.)?);
        let material = Arc::new(Material::Phong(MeshPhongMaterial {
            specular: Color::WHITE,
            shininess: 50.,
            ..Default::default()
        }));
        let mut seed = 186;
        for _ in 0..3000 {
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                box_geometry.clone(),
                material.clone(),
            )));
            let position = Vector3::new(
                8000. * (2. * random(&mut seed) - 1.),
                8000. * (2. * random(&mut seed) - 1.),
                8000. * (2. * random(&mut seed) - 1.),
            );
            let rotation = euler(
                random(&mut seed) * PI,
                random(&mut seed) * PI,
                random(&mut seed) * PI,
            );
            let n = s.get_mut(h)?;
            n.position = position;
            n.quaternion = rotation;
        }
        let dir = s.insert(NodeKind::Light(Light::Directional {
            color: super::shapes_lights::hsl(0.1, 0.7, 0.5),
            intensity: 0.15,
            target: Vector3::ZERO,
        }));
        s.get_mut(dir)?.position = Vector3::new(0., -1., 0.);
        let flare0 = {
            let mut t = decode_texture_image(
                &fetch(&format!(
                    "{ASSETS}/environment-materials/textures/lensflare/lensflare0.png"
                ))
                .await?,
            )
            .await?;
            t.srgb = true;
            t.mipmap_filter = Some(Filter::Linear);
            r.upload_texture(&Arc::new(t))?
        };
        let flare3 = {
            let mut t = decode_texture_image(
                &fetch(&format!("{ASSETS}/texture-flares/lensflare3.png")).await?,
            )
            .await?;
            t.srgb = true;
            t.mipmap_filter = Some(Filter::Linear);
            r.upload_texture(&Arc::new(t))?
        };
        let format = wgpu::TextureFormat::Rgba16Float;
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("lensflare"),
            source: wgpu::ShaderSource::Wgsl(FLARE.into()),
        });
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });
        let pipeline_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
        let additive = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        };
        let pipeline = |fragment: &str, test: bool, blend: Option<wgpu::BlendState>| {
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("lensflare"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &module,
                        entry_point: Some("vs"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module,
                        entry_point: Some(fragment),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format,
                            blend,
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: Default::default(),
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: false,
                        depth_compare: if test {
                            wgpu::CompareFunction::LessEqual
                        } else {
                            wgpu::CompareFunction::Always
                        },
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                })
        };
        let pipelines = [
            pipeline("magenta", true, None),
            pipeline("restore", false, None),
            pipeline("element", false, Some(additive)),
        ];
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let mut flares = vec![];
        for (h, sat, l, position) in [
            (0.55, 0.95, 0.6, Vector3::new(5000., 0., -1000.)),
            (0.1, 0.85, 0.65, Vector3::new(0., 0., -1000.)),
            (0.995, 0.5, 0.95, Vector3::new(5000., 5000., -1000.)),
        ] {
            let color = super::shapes_lights::hsl(h, sat, l);
            let light = s.insert(NodeKind::Light(Light::Point {
                color,
                intensity: 1.5,
                distance: 2000.,
                decay: 0.,
            }));
            s.get_mut(light)?.position = position;
            // The LensflareMesh itself: its unit quad, fully transparent, drawn last.
            let mut quad = BufferGeometry::default();
            quad.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(
                    vec![-1., -1., 0., 1., -1., 0., 1., 1., 0., -1., 1., 0.],
                    3,
                    false,
                )?),
            );
            quad.set_attribute(
                "uv",
                Attribute::F32(BufferAttribute::new(
                    vec![0., 0., 1., 0., 1., 1., 0., 1.],
                    2,
                    false,
                )?),
            );
            quad.set_index(Some(vec![0, 1, 2, 0, 2, 3]));
            let mut m = MeshBasicMaterial::default();
            m.properties.opacity = 0.;
            m.properties.transparent = true;
            // The original draws it after the occlusion probe; its depth must not
            // occlude the probe here, where the probe follows the scene.
            m.properties.depth_write = false;
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(quad),
                Arc::new(Material::Basic(m)),
            )));
            let n = s.get_mut(mesh)?;
            n.frustum_culled = false;
            n.render_order = i32::MAX;
            s.add(light, mesh)?;
            let small = |label| {
                r.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: 16,
                        height: 16,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                })
            };
            let temp = small("lensflare temp");
            let occlusion = small("lensflare occlusion");
            let temp_view = temp.create_view(&Default::default());
            let occlusion_view = occlusion.create_view(&Default::default());
            let (mut buffers, mut binds) = (vec![], vec![]);
            // The shared quad ( magenta and restore ), then the five elements.
            for map in [
                &temp_view,
                &flare0.view,
                &flare3.view,
                &flare3.view,
                &flare3.view,
                &flare3.view,
            ] {
                let buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("lensflare quad"),
                    size: 48,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                binds.push(r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(map),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::Sampler(
                                if std::ptr::eq(map, &temp_view) {
                                    &linear
                                } else {
                                    &flare0.sampler
                                },
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: wgpu::BindingResource::TextureView(&occlusion_view),
                        },
                    ],
                }));
                buffers.push(buffer);
            }
            flares.push(Flare {
                light,
                color,
                converted: false,
                temp,
                occlusion,
                buffers,
                binds,
            });
        }
        self.flares = Some(Flares {
            flares,
            pipelines,
            elements: [(700., 0.), (60., 0.6), (70., 0.7), (120., 0.9), (70., 1.)],
            state: [0.; 12],
            format,
        });
        Ok(())
    }
    async fn car_scene(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        s.tone_mapping = ToneMapping::Aces;
        s.exposure = 0.85;
        s.background = Color::from_hex(0x333333);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x333333),
            near: 10.,
            far: 15.,
        });
        let mut env = crate::environment::EnvironmentMap::from_hdr(
            &fetch("/web/environments/venice_sunset_1k.hdr").await?,
        )?;
        env.prefilter(r)?;
        s.environment = Some(Arc::new(env));
        // GridHelper( 20, 40, 0xffffff, 0xffffff ), 20% opaque without depth writes.
        let mut grid = super::interactive_scenes::grid_helper(20., 40, 0xffffff, 0xffffff)?;
        if let Material::Line(m) = Arc::make_mut(&mut grid.material) {
            m.properties.opacity = 0.2;
            m.properties.transparent = true;
            m.properties.depth_write = false;
            // GridHelper's material is not tone mapped.
            m.properties.tone_mapped = false;
        }
        let grid = s.insert(NodeKind::Line(grid));
        let (a, b, i) =
            load_asset(&format!("{ASSETS}/tsl-materials/models/gltf/ferrari.glb")).await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let root = instance.roots[0];
        let (mut body, mut glass, mut details, mut wheels) = (None, None, vec![], vec![]);
        // getObjectByName: the materials belong to the named meshes; the wheels
        // are the named nodes that carry them.
        for &h in &instance.meshes {
            match s.get(h)?.name.as_str() {
                "body" => body = Some(h),
                "glass" => glass = Some(h),
                "rim_fl" | "rim_fr" | "rim_rr" | "rim_rl" | "trim" => details.push(h),
                _ => {}
            }
        }
        for &h in &instance.nodes {
            let n = s.get(h)?;
            if ["wheel_fl", "wheel_fr", "wheel_rl", "wheel_rr"].contains(&n.name.as_str()) {
                wheels.push((h, n.quaternion));
            }
        }
        let mut shadow = decode_texture_image(
            &fetch(&format!(
                "{ASSETS}/tsl-materials/models/gltf/ferrari_ao.png"
            ))
            .await?,
        )
        .await?;
        shadow.srgb = false;
        shadow.mipmap_filter = Some(Filter::Linear);
        let mut m = MeshBasicMaterial::default();
        m.properties.map = Some(Arc::new(shadow));
        m.properties.transparent = true;
        m.properties.tone_mapped = false;
        // MultiplyBlending with premultipliedAlpha: ( ZERO, SRC_COLOR, ZERO, SRC_ALPHA ).
        let blend = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::Zero,
            dst_factor: wgpu::BlendFactor::Src,
            operation: wgpu::BlendOperation::Add,
        };
        m.properties.blending = Some(wgpu::BlendState {
            color: blend,
            alpha: blend,
        });
        let plane = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(0.655 * 4., 1.3 * 4., 1, 1)?),
            Arc::new(Material::Basic(m)),
        )));
        let n = s.get_mut(plane)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.render_order = 2;
        s.add(root, plane)?;
        let mut controls = Controls::new(None, (0., 9.), PI / 2., true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
        controls.update(s, c)?;
        self.controls = Some(controls);
        let car = Car {
            body: body.ok_or(Error::Invalid("car body"))?,
            details,
            glass: glass.ok_or(Error::Invalid("car glass"))?,
            wheels,
            grid,
            colors: [0xff0000, 0xffffff, 0xffffff],
            applied: [u32::MAX; 3],
        };
        self.car = Some(car);
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
            298 => self.painter_step(s, c, r)?,
            299 => {
                let k = self.cloud.as_mut().ok_or(Error::Invalid("cloud"))?;
                // animate(): a new 30³ block every 1.5 s, generated on the CPU and
                // copied into its cell, as copyTextureToTexture does.
                const PADDED: u32 = 30;
                let now = t * 1000.;
                if now - k.previous > 1500. && k.current < 64 {
                    let per = (127 / 4) as f64;
                    let i = k.current;
                    let origin = [
                        ((i % 4) as f64 * per + 4.).floor() as u32,
                        (((i % 16) / 4) as f64 * per + 4.).floor() as u32,
                        ((i / 16) as f64 * per + 4.).floor() as u32,
                    ];
                    let scale_factor = (random(&mut k.seed) + 0.5) * 0.5;
                    let data = cloud_block(PADDED as usize, scale_factor);
                    r.queue.write_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture: &k.texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d {
                                x: origin[0],
                                y: origin[1],
                                z: origin[2],
                            },
                            aspect: wgpu::TextureAspect::All,
                        },
                        &data,
                        wgpu::TexelCopyBufferLayout {
                            offset: 0,
                            bytes_per_row: Some(PADDED),
                            rows_per_image: Some(PADDED),
                        },
                        wgpu::Extent3d {
                            width: PADDED,
                            height: PADDED,
                            depth_or_array_layers: PADDED,
                        },
                    );
                    k.previous = now;
                    k.current += 1;
                }
                k.frame += 1.;
                let (frame, p) = (k.frame, k.params);
                if let NodeKind::Mesh(m) = &mut s.get_mut(k.mesh)?.kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0] = p.map(|v| v as f32);
                    m.uniforms[2] = [frame as f32, 0., 0., 0.];
                }
            }
            301 => self.fly_step(s, c, delta)?,
            302 => {
                let k = self.car.as_mut().ok_or(Error::Invalid("car"))?;
                // time = - performance.now() / 1000.
                let time = -t;
                for (h, rest) in &k.wheels {
                    let mut e = Euler::from_quaternion(*rest, EulerOrder::XYZ);
                    e.angles.x = time * PI * 2.;
                    let n = s.get_mut(*h)?;
                    n.quaternion = e.quaternion();
                    n.matrix_auto_update = true;
                }
                // grid.position.z = - ( time ) % 1, JavaScript's remainder.
                s.get_mut(k.grid)?.position.z = -((-time) % 1.);
                if k.applied != k.colors {
                    k.applied = k.colors;
                    let [body, detail, glass] = k.colors.map(Color::from_hex);
                    let mut b = MeshPhysicalMaterial {
                        clearcoat: 1.,
                        clearcoat_roughness: 0.03,
                        ..Default::default()
                    };
                    b.base.properties.color = body;
                    b.base.metalness = 1.;
                    b.base.roughness = 0.5;
                    let mut d = MeshStandardMaterial {
                        metalness: 1.,
                        roughness: 0.5,
                        ..Default::default()
                    };
                    d.properties.color = detail;
                    let mut g = MeshPhysicalMaterial {
                        transmission: 1.,
                        ..Default::default()
                    };
                    g.base.properties.color = glass;
                    g.base.metalness = 0.25;
                    g.base.roughness = 0.;
                    let d = Arc::new(Material::Standard(d));
                    for (h, m) in [
                        (k.body, Arc::new(Material::Physical(b))),
                        (k.glass, Arc::new(Material::Physical(g))),
                    ]
                    .into_iter()
                    .chain(k.details.iter().map(|&h| (h, d.clone())))
                    {
                        if let NodeKind::Mesh(mesh) = &mut s.get_mut(h)?.kind {
                            mesh.materials = vec![m];
                        }
                    }
                }
                if let Some(controls) = &mut self.controls {
                    controls.update(s, c)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    /// The pointer's hit: `transformUv` by the hit object's map, then the cross
    /// is redrawn and every parent texture copies the canvas again.
    fn painter_step(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let k = self.painter.as_mut().ok_or(Error::Invalid("painter"))?;
        // animate(): the circle texture's offset, repeat and rotation.
        let wrap = (k.params[0] as usize, k.params[1] as usize);
        let transform = uv_transform(&k.params);
        let circle = &mut k.textures[2].materials[wrap.0 * 3 + wrap.1];
        if let Material::Shader(current) = circle.as_ref()
            && (current.uniforms[0] != transform[0] || current.uniforms[1] != transform[1])
            && let Material::Shader(m) = Arc::make_mut(circle)
        {
            m.uniforms[0] = transform[0];
            m.uniforms[1] = transform[1];
        }
        let circle = circle.clone();
        if let NodeKind::Mesh(m) = &mut s.get_mut(k.meshes[2])?.kind
            && !Arc::ptr_eq(&m.materials[0], &circle)
        {
            m.materials[0] = circle;
        }
        if let Some((x, y)) = k.pointer.take() {
            let (w, h, _) = viewport_css();
            let (camera, world) = s.camera(c)?;
            let mut ray = crate::raycast::Raycaster::default();
            ray.set_from_camera(
                Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.),
                camera,
                world,
            )?;
            let hits = ray.intersect_objects(s, &k.meshes, false)?;
            if let Some(hit) = hits.first()
                && let Some(mut uv) = hit.uv
            {
                let index = k.meshes.iter().position(|&m| m == hit.object).unwrap_or(0);
                let (matrix, wraps) = match index {
                    0 => (None, (Wrapping::Repeat, Wrapping::Repeat)),
                    1 => (None, (Wrapping::Mirror, Wrapping::Mirror)),
                    _ => (Some(transform), (wrapping(wrap.0), wrapping(wrap.1))),
                };
                if let Some([a, b]) = matrix {
                    uv = Vector2::new(
                        a[0] as f64 * uv.x + a[2] as f64 * uv.y + b[0] as f64,
                        a[1] as f64 * uv.x + a[3] as f64 * uv.y + b[1] as f64,
                    );
                }
                let fold = |v: f64, mode: Wrapping| {
                    if (0. ..=1.).contains(&v) {
                        return v;
                    }
                    match mode {
                        Wrapping::Repeat => v - v.floor(),
                        Wrapping::Clamp => {
                            if v < 0. {
                                0.
                            } else {
                                1.
                            }
                        }
                        Wrapping::Mirror => {
                            if (v.floor() % 2.).abs() == 1. {
                                v.ceil() - v
                            } else {
                                v - v.floor()
                            }
                        }
                    }
                };
                uv = Vector2::new(fold(uv.x, wraps.0), 1. - fold(uv.y, wraps.1));
                k.position = (
                    uv.x * k.canvas.width() as f64,
                    uv.y * k.canvas.height() as f64,
                );
                k.draw()?;
            }
        }
        // needsUpdate: the textures in use copy the canvas and regenerate mips.
        for t in &mut k.textures {
            if t.stale {
                r.queue.copy_external_image_to_texture(
                    &wgpu::CopyExternalImageSourceInfo {
                        source: wgpu::ExternalImageSource::HTMLCanvasElement(k.canvas.clone()),
                        origin: wgpu::Origin2d::ZERO,
                        flip_y: true,
                    },
                    wgpu::CopyExternalImageDestInfo {
                        texture: &t.texture.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                        color_space: wgpu::PredefinedColorSpace::Srgb,
                        premultiplied_alpha: false,
                    },
                    wgpu::Extent3d {
                        width: k.canvas.width(),
                        height: k.canvas.height(),
                        depth_or_array_layers: 1,
                    },
                );
                t.mips.update(&r.device, &r.queue);
                t.stale = false;
            }
        }
        Ok(())
    }
    /// FlyControls.update( delta ): movementSpeed 2500, rollSpeed π / 6.
    fn fly_step(&mut self, s: &mut Scene, c: Object3D, delta: f64) -> Result<()> {
        let k = self.flares.as_mut().ok_or(Error::Invalid("flares"))?;
        // A light whose lens flare was drawn keeps the sRGB-to-linear color that
        // its first element converted in place.
        for f in &k.flares {
            if f.converted
                && let NodeKind::Light(Light::Point { color, .. }) = &mut s.get_mut(f.light)?.kind
            {
                *color = f.color;
            }
        }
        let st = k.state;
        let movement = Vector3::new(-st[2] + st[3], -st[1] + st[0], -st[4] + st[5]);
        let rotation = Vector3::new(-st[7] + st[6], -st[9] + st[8], -st[11] + st[10]);
        let (move_mult, rot_mult) = (delta * 2500., delta * PI / 6.);
        let n = s.get_mut(c)?;
        let q = n.quaternion;
        n.position += q * (movement * move_mult);
        let r = rotation * rot_mult;
        n.quaternion = q * Quaternion::from_xyzw(r.x, r.y, r.z, 1.).normalize();
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let Some(k) = self.flares.as_mut() else {
            return Ok(false);
        };
        r.render(s, c, out)?;
        if out.options().format != k.format
            || out.depth_format() != Some(wgpu::TextureFormat::Depth32Float)
        {
            return Err(Error::Invalid("lensflare target format"));
        }
        let depth = out
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("lensflare depth"))?;
        let (camera, world) = s.camera(c)?;
        let projection = camera.projection_matrix()?;
        let view = world.inverse();
        let (vw, vh) = (out.width as f64, out.height as f64);
        let inv_aspect = vh / vw;
        let size = 16. / vh;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let pass = |encoder: &mut wgpu::CommandEncoder,
                    draws: &[(&wgpu::RenderPipeline, &wgpu::BindGroup)]| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("lensflare"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &out.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for (pipeline, bind) in draws {
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, *bind, &[]);
                pass.draw(0..6, 0..1);
            }
        };
        for f in &mut k.flares {
            // onBeforeRender(): the light's view and screen positions.
            let world_position = s.get(f.light)?.matrix_world.w_axis.truncate();
            let position_view = view.transform_point3(world_position);
            if position_view.z > 0. {
                continue;
            }
            let screen = projection.project_point3(position_view);
            let px = screen.x * vw / 2. + vw / 2. - 8.;
            let py = -screen.y * vh / 2. + vh / 2. - 8.;
            if !(0. ..=vw - 16.).contains(&px) || !(0. ..=vh - 16.).contains(&py) {
                continue;
            }
            let write =
                |buffer: &wgpu::Buffer, position: [f64; 3], scale: [f64; 2], color: Vector3| {
                    r.queue.write_buffer(
                        buffer,
                        0,
                        bytemuck::cast_slice(&[
                            position[0] as f32,
                            position[1] as f32,
                            position[2] as f32,
                            0.,
                            scale[0] as f32,
                            scale[1] as f32,
                            0.,
                            0.,
                            color.x as f32,
                            color.y as f32,
                            color.z as f32,
                            1.,
                        ]),
                    );
                };
            write(
                &f.buffers[0],
                [screen.x, screen.y, screen.z],
                [size * inv_aspect, size],
                Vector3::ONE,
            );
            // mesh2.color = element.color.convertSRGBToLinear(): the light's own color
            // for the first element, converted in place once.
            if !f.converted {
                f.converted = true;
                let c = f.color.0.map(crate::math::srgb_to_linear);
                f.color = Color(c);
            }
            let (vx, vy) = (-screen.x * 2., -screen.y * 2.);
            for (i, (element_size, distance)) in k.elements.iter().enumerate() {
                let color = if i == 0 { f.color.0 } else { Vector3::ONE };
                let s = element_size / vh;
                write(
                    &f.buffers[i + 1],
                    [screen.x + vx * distance, screen.y - vy * distance, screen.z],
                    [s * inv_aspect, s],
                    color,
                );
            }

            let source = wgpu::TexelCopyTextureInfo {
                texture: &out.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: px as u32,
                    y: py as u32,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            };
            let extent = wgpu::Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1,
            };
            encoder.copy_texture_to_texture(source, region(&f.temp), extent);
            pass(&mut encoder, &[(&k.pipelines[0], &f.binds[0])]);
            encoder.copy_texture_to_texture(source, region(&f.occlusion), extent);
            let mut draws = vec![(&k.pipelines[1], &f.binds[0])];
            draws.extend(f.binds[1..].iter().map(|b| (&k.pipelines[2], b)));
            pass(&mut encoder, &draws);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    /// Absolute CSS pointer events: 0 move, 10 + button down, 20 + button up.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if let Some(k) = &mut self.painter
            && kind == 0
        {
            k.pointer = Some((x, y));
        }
        if let Some(k) = &mut self.flares {
            let (w, h, _) = viewport_css();
            match kind {
                10 => k.state[4] = 1.,
                12 => k.state[5] = 1.,
                20 => k.state[4] = 0.,
                22 => k.state[5] = 0.,
                0 => {
                    k.state[8] = -(x - w / 2.) / (w / 2.);
                    k.state[7] = (y - h / 2.) / (h / 2.);
                }
                _ => {}
            }
        }
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let Some(k) = &mut self.flares {
            let index = match code {
                87 => 4,
                83 => 5,
                65 => 2,
                68 => 3,
                82 => 0,
                70 => 1,
                38 => 6,
                40 => 7,
                37 => 8,
                39 => 9,
                81 => 10,
                69 => 11,
                _ => return,
            };
            k.state[index] = f64::from(u8::from(down));
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
        // The car's controls update in the animation loop.
        if self.id == 302 {
            return Ok(());
        }
        controls.update(s, c)
    }
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        match self.id {
            298 if index < 7 => {
                self.painter
                    .as_mut()
                    .ok_or(Error::Invalid("painter"))?
                    .params[index] = v
            }
            299 if index < 4 => {
                self.cloud.as_mut().ok_or(Error::Invalid("cloud"))?.params[index] = v
            }
            302 if index < 3 => {
                self.car.as_mut().ok_or(Error::Invalid("car"))?.colors[index] = value as u32
            }
            _ => return Err(Error::Invalid("texture/flare parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
impl Painter {
    /// CanvasTexture._draw(): the grid image and the yellow cross.
    fn draw(&mut self) -> Result<()> {
        let [_, max, min, thickness] = self.cross;
        let (x, y) = self.position;
        let c = &self.context;
        let (w, h) = (self.canvas.width() as f64, self.canvas.height() as f64);
        c.clear_rect(0., 0., w, h);
        c.draw_image_with_html_image_element(&self.image, 0., 0.)
            .map_err(|_| Error::Invalid("drawImage"))?;
        c.set_line_width(thickness * 3.);
        c.set_stroke_style_str("#FFFF00");
        c.begin_path();
        c.move_to(x - max - 2., y - max - 2.);
        c.line_to(x - min, y - min);
        c.move_to(x + min, y + min);
        c.line_to(x + max + 2., y + max + 2.);
        c.move_to(x - max - 2., y + max + 2.);
        c.line_to(x - min, y + min);
        c.move_to(x + min, y - min);
        c.line_to(x + max + 2., y - max - 2.);
        c.stroke();
        for t in &mut self.textures {
            t.stale = true;
        }
        Ok(())
    }
}
/// A whole-texture copy destination.
fn region(texture: &wgpu::Texture) -> wgpu::TexelCopyTextureInfo<'_> {
    wgpu::TexelCopyTextureInfo {
        texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
    }
}
/// generateCloudTexture( size, scaleFactor ): faded ImprovedNoise stored as bytes.
fn cloud_block(size: usize, scale_factor: f64) -> Vec<u8> {
    let mut data = vec![0u8; size * size * size];
    let scale = scale_factor * 10. / size as f64;
    let mut i = 0;
    for z in 0..size {
        for y in 0..size {
            for x in 0..size {
                let half = size as f64 / 2.;
                let d =
                    Vector3::new(x as f64 - half, y as f64 - half, z as f64 - half) / size as f64;
                let fading = (1. - d.length()) * (1. - d.length());
                let value = (128.
                    + 128.
                        * super::trackball_sprites::noise(
                            x as f64 * scale / 1.5,
                            y as f64 * scale,
                            z as f64 * scale / 1.5,
                        ))
                    * fading;
                // Uint8Array stores ToUint8: truncated, modulo 256.
                data[i] = (value.trunc() as i64).rem_euclid(256) as u8;
                i += 1;
            }
        }
    }
    data
}
/// The cloud RawShaderMaterial's fragment: a jittered raymarch through the box.
const CLOUD: &str = r#"fn cloud_march(p0:vec4<f32>,base:vec4<f32>,f:vec4<f32>,screen:vec2<f32>)->vec4<f32>{
 let threshold=p0.x;let opacity=p0.y;let range=p0.z;let steps=p0.w;
 let origin=u.camera.xyz;let ray=normalize(fragment_surface.local_position-origin);
 let inv=1.0/ray;let t0s=(vec3(-0.5)-origin)*inv;let t1s=(vec3(0.5)-origin)*inv;
 let tmin=min(t0s,t1s);let tmax=max(t0s,t1s);
 var bounds=vec2(max(tmin.x,max(tmin.y,tmin.z)),min(tmax.x,min(tmax.y,tmax.z)));
 if bounds.x>bounds.y {discard;}
 bounds.x=max(bounds.x,0.0);
 var p=origin+bounds.x*ray;
 let incr=1.0/abs(ray);var delta=min(incr.x,min(incr.y,incr.z));delta/=steps;
 let frag=fragment_surface.clip.xy;let gl_y=screen.y-frag.y;
 var seed=u32(frag.x)*1973u+u32(gl_y)*9277u+u32(f.x)*26699u;
 seed=(seed^61u)^(seed>>16u);seed*=9u;seed=seed^(seed>>4u);seed*=0x27d4eb2du;seed=seed^(seed>>15u);
 let rand=f32(seed)/4294967296.0*2.0-1.0;
 p+=ray*rand*(1.0/128.0);
 var ac=vec4(base.rgb,0.0);
 for(var t=bounds.x;t<bounds.y;t+=delta){
  var d=textureSampleLevel(tsl_texture_0,tsl_sampler_0,p+0.5,0.0).r;
  d=smoothstep(threshold-range,threshold+range,d)*opacity;
  let s=0.01;
  let shade=textureSampleLevel(tsl_texture_0,tsl_sampler_0,p+0.5+vec3(-s),0.0).r-textureSampleLevel(tsl_texture_0,tsl_sampler_0,p+0.5+vec3(s),0.0).r;
  let col=shade*3.0+((p.x+p.y)*0.25)+0.2;
  ac=vec4(ac.rgb+(1.0-ac.a)*d*col,ac.a+(1.0-ac.a)*d);
  if ac.a>=0.95 {break;}
  p+=ray*delta;
 }
 if ac.a==0.0 {discard;}
 return ac;
}"#;
/// renderToCubeTexture's CubemapFilterShader, one face and level per draw.
const CUBE_FILTER: &str = r#"
@group(0) @binding(0) var source:texture_cube<f32>;
@group(0) @binding(1) var source_sampler:sampler;
@group(0) @binding(2) var<uniform> params:vec4<f32>;
@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=vec2(f32((i<<1u)&2u),f32(i&2u));return vec4(p*2.0-1.0,0.0,1.0);
}
@fragment fn fs(@builtin(position) position:vec4<f32>)->@location(0) vec4<f32>{
 let a=position.x/params.z*2.0-1.0;let b=position.y/params.z*2.0-1.0;
 var d=vec3(-a,-b,-1.0);
 switch u32(params.x) {
  case 0u:{d=vec3(1.0,-b,-a);}
  case 1u:{d=vec3(-1.0,-b,a);}
  case 2u:{d=vec3(a,1.0,b);}
  case 3u:{d=vec3(a,-1.0,-b);}
  case 4u:{d=vec3(a,-b,1.0);}
  default:{}
 }
 let mip=params.y;var tint=vec3(1.0,0.0,0.0);
 if mip==0.0 {tint=vec3(1.0);} else if mip==1.0 {tint=vec3(0.0,0.0,1.0);}
 else if mip==2.0 {tint=vec3(0.0,1.0,1.0);} else if mip==3.0 {tint=vec3(0.0,1.0,0.0);}
 else if mip==4.0 {tint=vec3(1.0,1.0,0.0);}
 return textureSample(source,source_sampler,normalize(d))*vec4(tint,1.0);
}"#;
/// LensflareMesh's quads: the magenta occlusion probe, the restored framebuffer
/// patch and the additive elements, whose vertices read nine occlusion texels.
const FLARE: &str = r#"
struct Quad{position:vec4<f32>,scale:vec4<f32>,color:vec4<f32>};
@group(0) @binding(0) var<uniform> quad:Quad;
@group(0) @binding(1) var map:texture_2d<f32>;
@group(0) @binding(2) var map_sampler:sampler;
@group(0) @binding(3) var occlusion:texture_2d<f32>;
struct Out{@builtin(position) clip:vec4<f32>,@location(0) uv:vec2<f32>,@location(1) visibility:f32};
@vertex fn vs(@builtin(vertex_index) i:u32)->Out{
 var corners=array(vec2(-1.0,-1.0),vec2(1.0,-1.0),vec2(1.0,1.0),vec2(-1.0,-1.0),vec2(1.0,1.0),vec2(-1.0,1.0));
 let p=corners[i];var out:Out;
 out.clip=vec4(p*quad.scale.xy+quad.position.xy,quad.position.z,1.0);
 out.uv=p*0.5+0.5;
 var v=textureLoad(occlusion,vec2(2,2),0)+textureLoad(occlusion,vec2(8,2),0)+textureLoad(occlusion,vec2(14,2),0)
  +textureLoad(occlusion,vec2(14,8),0)+textureLoad(occlusion,vec2(14,14),0)+textureLoad(occlusion,vec2(8,14),0)
  +textureLoad(occlusion,vec2(2,14),0)+textureLoad(occlusion,vec2(2,8),0)+textureLoad(occlusion,vec2(8,8),0);
 out.visibility=v.r/9.0*(1.0-v.g/9.0)*(v.b/9.0);
 return out;
}
@fragment fn magenta()->@location(0) vec4<f32>{return vec4(1.0,0.0,1.0,1.0);}
@fragment fn restore(in:Out)->@location(0) vec4<f32>{return textureSample(map,map_sampler,vec2(in.uv.x,1.0-in.uv.y));}
@fragment fn element(in:Out)->@location(0) vec4<f32>{
 var c=textureSample(map,map_sampler,vec2(in.uv.x,1.0-in.uv.y));c.a*=in.visibility;return vec4(c.rgb*quad.color.rgb,c.a);
}"#;
