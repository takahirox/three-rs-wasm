//! webgl_materials_subsurface_scattering: the Stanford bunny turning by the
//! clock with SubsurfaceScatteringShader ( MeshPhongMaterial's program with
//! a translucency term added to every direct light: the thickness map's red
//! channel tinted by thicknessColor, lit through the surface by a distorted
//! half vector ), under an ambient light, a faint directional light and two
//! range-limited point lights marked by small basic spheres. The bunny is
//! FBXLoader's parse of the page's FBX, baked by
//! tools/tsl/prepare-bunny.mjs ( the loader's non-indexed attributes ). The
//! shaders are WGSL ports of the GLSL WebGLRenderer builds for the
//! ShaderMaterial ( lights: true, no map: the material defines no USE_MAP )
//! and for MeshBasicMaterial; the canvas is sRGB-encoded in the shader and
//! multisampled when the page's antialias is.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::retro::mipmapped;
use super::shadowmap_opacity::Mipmaps;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use wgpu::util::DeviceExt;

const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const ASSETS: &str = "/web/gallery/assets/subsurface-scattering";
const SHADER: &str = r#"
struct Frame {
	view_projection: mat4x4<f32>,
	view: mat4x4<f32>,
	// Point lights: view-space position and cutoff distance, color.
	point0: vec4<f32>,
	point_color0: vec4<f32>,
	point1: vec4<f32>,
	point_color1: vec4<f32>,
	// The directional light's view-space direction and color.
	direction: vec4<f32>,
	direction_color: vec4<f32>,
	ambient: vec4<f32>,
	// diffuse, shininess; specular.
	diffuse: vec4<f32>,
	specular: vec4<f32>,
	// thicknessColor, thicknessDistortion; ambient, attenuation, power, scale.
	thickness_color: vec4<f32>,
	thickness: vec4<f32>,
};
struct Object { model: mat4x4<f32>, color: vec4<f32> };
@group( 0 ) @binding( 0 ) var<uniform> frame: Frame;
@group( 0 ) @binding( 1 ) var<uniform> object: Object;
@group( 0 ) @binding( 2 ) var thickness_map: texture_2d<f32>;
@group( 0 ) @binding( 3 ) var thickness_sampler: sampler;
fn encode( c: vec3<f32> ) -> vec4<f32> {
	return vec4( select( pow( c, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), c * 12.92, c <= vec3( 0.0031308 ) ), 1.0 );
}
struct Varyings {
	@builtin( position ) position: vec4<f32>,
	@location( 0 ) view_position: vec3<f32>,
	@location( 1 ) normal: vec3<f32>,
	@location( 2 ) uv: vec2<f32>,
};
@vertex fn bunny( @location( 0 ) position: vec3<f32>, @location( 1 ) normal: vec3<f32>, @location( 2 ) uv: vec2<f32> ) -> Varyings {
	let mv = frame.view * object.model * vec4( position, 1.0 );
	var out: Varyings;
	out.position = frame.view_projection * object.model * vec4( position, 1.0 );
	out.view_position = -mv.xyz;
	out.normal = normalize( ( frame.view * object.model * vec4( normal, 0.0 ) ).xyz );
	out.uv = uv;
	return out;
}
const RECIPROCAL_PI: f32 = 0.3183098861837907;
fn blinn_phong( light: vec3<f32>, view: vec3<f32>, normal: vec3<f32> ) -> vec3<f32> {
	let half_dir = normalize( light + view );
	let dot_nh = saturate( dot( normal, half_dir ) );
	let dot_vh = saturate( dot( view, half_dir ) );
	let fresnel = exp2( ( -5.55473 * dot_vh - 6.98316 ) * dot_vh );
	let f = frame.specular.rgb * ( 1.0 - fresnel ) + vec3( fresnel );
	let d = RECIPROCAL_PI * ( frame.diffuse.w * 0.5 + 1.0 ) * pow( dot_nh, frame.diffuse.w );
	return f * ( 0.25 * d );
}
// RE_Direct_BlinnPhong, then RE_Direct_Scattering.
fn direct( direction: vec3<f32>, color: vec3<f32>, normal: vec3<f32>, view: vec3<f32>, thickness: vec3<f32> ) -> vec3<f32> {
	let irradiance = saturate( dot( normal, direction ) ) * color;
	var light = irradiance * RECIPROCAL_PI * frame.diffuse.rgb + irradiance * blinn_phong( direction, view, normal );
	let half_dir = normalize( direction + normal * frame.thickness_color.w );
	let scattering = pow( saturate( dot( view, -half_dir ) ), frame.thickness.z ) * frame.thickness.w;
	light += ( scattering + frame.thickness.x ) * thickness * frame.thickness.y * color;
	return light;
}
// getDistanceAttenuation with decay 0.
fn point( light: vec4<f32>, color: vec3<f32>, position: vec3<f32>, normal: vec3<f32>, view: vec3<f32>, thickness: vec3<f32> ) -> vec3<f32> {
	let l = light.xyz - position;
	let distance = length( l );
	var falloff = 1.0;
	if ( light.w > 0.0 ) {
		let x = distance / light.w;
		let window = saturate( 1.0 - x * x * x * x );
		falloff *= window * window;
	}
	return direct( normalize( l ), color * falloff, normal, view, thickness );
}
@fragment fn bunny_fs( in: Varyings ) -> @location( 0 ) vec4<f32> {
	let normal = normalize( in.normal );
	let view = normalize( in.view_position );
	let position = -in.view_position;
	// The page's texture is uploaded with flipY.
	let sample = textureSample( thickness_map, thickness_sampler, vec2( in.uv.x, 1.0 - in.uv.y ) ).r;
	let thickness = frame.thickness_color.rgb * sample;
	var light = point( frame.point0, frame.point_color0.rgb, position, normal, view, thickness );
	light += point( frame.point1, frame.point_color1.rgb, position, normal, view, thickness );
	light += direct( frame.direction.xyz, frame.direction_color.rgb, normal, view, thickness );
	light += frame.ambient.rgb * RECIPROCAL_PI * frame.diffuse.rgb;
	return encode( light );
}
@vertex fn basic( @location( 0 ) position: vec3<f32> ) -> @builtin( position ) vec4<f32> {
	return frame.view_projection * object.model * vec4( position, 1.0 );
}
@fragment fn basic_fs() -> @location( 0 ) vec4<f32> {
	return encode( object.color.rgb );
}
"#;
/// `Color.setHex` with color management: sRGB bytes to linear.
fn linear(hex: u32) -> [f64; 3] {
    [16, 8, 0].map(|shift| {
        let c = f64::from((hex >> shift) & 255) / 255.;
        if c < 0.04045 {
            c * 0.0773993808
        } else {
            (c * 0.9478672986 + 0.0521327014).powf(2.4)
        }
    })
}
/// The point lights: position, color, intensity, distance.
const POINTS: [([f64; 3], u32, f64, f64); 2] = [
    ([0., -50., 350.], 0xc1c1c1, 4., 300.),
    ([-100., 20., -260.], 0xc1c100, 0.75, 500.),
];
struct Targets {
    size: (u32, u32),
    samples: u32,
    format: wgpu::TextureFormat,
    color: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    pipelines: [wgpu::RenderPipeline; 2],
}
pub(super) struct Demo {
    controls: Controls,
    pending: bool,
    elapsed: f64,
    /// distortion, ambient, attenuation, power, scale.
    params: [f64; 5],
    bunny: (wgpu::Buffer, u32),
    sphere: (wgpu::Buffer, wgpu::Buffer, u32),
    frame: wgpu::Buffer,
    /// The bunny's and the two spheres' object uniforms.
    objects: [wgpu::Buffer; 3],
    groups: [wgpu::BindGroup; 3],
    layout: wgpu::BindGroupLayout,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 1.,
            far: 5000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 300., 1600.);
        let mut controls = Controls::new(None, (500., 3000.), std::f64::consts::PI, true);
        controls.update(s, c)?;
        let init = |label, contents: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        #[derive(serde::Deserialize)]
        struct Bake {
            vertices: u32,
        }
        let bake: Bake = serde_json::from_slice(&fetch(&format!("{ASSETS}/bunny.json")).await?)
            .map_err(|e| Error::Asset(e.to_string()))?;
        let bytes = fetch(&format!("{ASSETS}/bunny.bin")).await?;
        let n = bake.vertices as usize;
        if bytes.len() != n * 8 * 4 {
            return Err(Error::Invalid("bunny bake"));
        }
        let floats: Vec<f32> = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|&b| f32::from_le_bytes(b))
            .collect();
        let (positions, rest) = floats.split_at(n * 3);
        let (normals, uvs) = rest.split_at(n * 3);
        let mut data = Vec::with_capacity(n * 8);
        for k in 0..n {
            data.extend_from_slice(&positions[k * 3..k * 3 + 3]);
            data.extend_from_slice(&normals[k * 3..k * 3 + 3]);
            data.extend_from_slice(&uvs[k * 2..k * 2 + 2]);
        }
        let vertex = wgpu::BufferUsages::VERTEX;
        let sphere = SphereGeometry::build(4., 8, 8)?;
        let sphere_positions = match sphere.attributes.get("position") {
            Some(Attribute::F32(a)) => a.array().to_vec(),
            _ => return Err(Error::Invalid("sphere positions")),
        };
        let sphere_index = sphere.index.clone().ok_or(Error::Invalid("sphere index"))?;
        let image =
            decode_texture_image(&fetch(&format!("{ASSETS}/bunny_thickness.jpg")).await?).await?;
        let thickness = mipmapped(
            r,
            &mut Mipmaps::new(r),
            &image,
            wgpu::TextureFormat::Rgba8Unorm,
        );
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty,
            count: None,
        };
        let uniform = wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        };
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("subsurface"),
                entries: &[
                    entry(0, uniform),
                    entry(1, uniform),
                    entry(
                        2,
                        wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                    ),
                    entry(
                        3,
                        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    ),
                ],
            });
        let frame = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("subsurface frame"),
            size: 64 * 2 + 16 * 11,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        // The light markers: MeshBasicMaterial colors at the lights.
        let object = |label, model: Matrix4, color: [f64; 3]| {
            let mut data: Vec<f32> = model.to_cols_array().iter().map(|&v| v as f32).collect();
            data.extend([color[0] as f32, color[1] as f32, color[2] as f32, 1.]);
            init(
                label,
                bytemuck::cast_slice(&data),
                wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            )
        };
        let objects = [
            object("bunny", Matrix4::IDENTITY, [1.; 3]),
            object(
                "light marker",
                Matrix4::from_translation(Vector3::from_array(POINTS[0].0)),
                linear(POINTS[0].1),
            ),
            object(
                "light marker",
                Matrix4::from_translation(Vector3::from_array(POINTS[1].0)),
                linear(POINTS[1].1),
            ),
        ];
        let groups = [0, 1, 2].map(|k| {
            r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("subsurface"),
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: frame.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: objects[k].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&thickness),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            })
        });
        Ok(Self {
            controls,
            pending: true,
            elapsed: 0.,
            params: [0.1, 0.4, 0.8, 2., 16.],
            bunny: (
                init("bunny", bytemuck::cast_slice(&data), vertex),
                bake.vertices,
            ),
            sphere: (
                init(
                    "light marker",
                    bytemuck::cast_slice(&sphere_positions),
                    vertex,
                ),
                init(
                    "light marker index",
                    bytemuck::cast_slice(&sphere_index),
                    wgpu::BufferUsages::INDEX,
                ),
                sphere_index.len() as u32,
            ),
            frame,
            objects,
            groups,
            layout,
            targets: None,
        })
    }
    fn pipeline(
        &self,
        r: &Renderer,
        bunny: bool,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> wgpu::RenderPipeline {
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("subsurface"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("subsurface"),
                bind_group_layouts: &[&self.layout],
                push_constant_ranges: &[],
            });
        let attributes = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 12,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 24,
                shader_location: 2,
            },
        ];
        let buffers = [if bunny {
            wgpu::VertexBufferLayout {
                array_stride: 32,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &attributes,
            }
        } else {
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &attributes[..1],
            }
        }];
        r.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("subsurface"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some(if bunny { "bunny" } else { "basic" }),
                    compilation_options: Default::default(),
                    buffers: &buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some(if bunny { "bunny_fs" } else { "basic_fs" }),
                    compilation_options: Default::default(),
                    targets: &[Some(format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: samples,
                    ..Default::default()
                },
                multiview: None,
                cache: None,
            })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        self.pending |= animate;
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let resized = self.targets.as_ref().is_none_or(|t| {
            t.size != (out.width, out.height)
                || t.samples != samples
                || t.format != out.options.format
        });
        if resized {
            let attachment = |format, label| {
                r.device
                    .create_texture(&wgpu::TextureDescriptor {
                        label: Some(label),
                        size: wgpu::Extent3d {
                            width: out.width,
                            height: out.height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: samples,
                        dimension: wgpu::TextureDimension::D2,
                        format,
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    })
                    .create_view(&Default::default())
            };
            self.targets = Some(Targets {
                size: (out.width, out.height),
                samples,
                format: out.options.format,
                color: (samples > 1).then(|| attachment(out.options.format, "subsurface color")),
                depth: attachment(DEPTH, "subsurface depth"),
                screen: RenderTarget::with_options(
                    &r.device,
                    out.width,
                    out.height,
                    RenderTargetOptions {
                        samples: 0,
                        depth_buffer: false,
                        ..out.options.clone()
                    },
                )?,
                pipelines: [
                    self.pipeline(r, true, out.options.format, samples),
                    self.pipeline(r, false, out.options.format, samples),
                ],
            });
        }
        if !std::mem::take(&mut self.pending) && !resized {
            return Ok(true);
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let view = world.inverse();
        let view_projection = camera.projection_matrix()? * view;
        // render(): model.rotation.y = performance.now() / 5000.
        let model = Matrix4::from_translation(Vector3::new(0., 0., 10.))
            * Matrix4::from_rotation_y(self.elapsed * 1000. / 5000.);
        let mut bunny: Vec<f32> = model.to_cols_array().iter().map(|&v| v as f32).collect();
        bunny.extend([1., 1., 1., 1.]);
        r.queue
            .write_buffer(&self.objects[0], 0, bytemuck::cast_slice(&bunny));
        let mut data: Vec<f32> = vec![];
        for m in [view_projection, view] {
            data.extend(m.to_cols_array().iter().map(|&v| v as f32));
        }
        let mut vec4 = |v: [f64; 4]| data.extend(v.map(|x| x as f32));
        for (position, color, intensity, distance) in POINTS {
            let p = view.transform_point3(Vector3::from_array(position));
            vec4([p.x, p.y, p.z, distance]);
            let [r, g, b] = linear(color);
            vec4([r * intensity, g * intensity, b * intensity, 1.]);
        }
        let d = view
            .transform_vector3(Vector3::new(0., 0.5, 0.5).normalize())
            .normalize();
        vec4([d.x, d.y, d.z, 0.]);
        vec4([0.03, 0.03, 0.03, 1.]);
        let [a, _, _] = linear(0xc1c1c1);
        vec4([a, a, a, 1.]);
        // diffuse ( a Vector3, not color-managed ) and shininess; specular 0x111111.
        vec4([1., 0.2, 0.2, 500.]);
        let [sr, sg, sb] = linear(0x111111);
        vec4([sr, sg, sb, 1.]);
        let p = self.params;
        vec4([0.5, 0.3, 0., p[0]]);
        vec4([p[1], p[2], p[3], p[4]]);
        r.queue
            .write_buffer(&self.frame, 0, bytemuck::cast_slice(&data));
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("subsurface targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("subsurface"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: t.color.as_ref().unwrap_or(&t.screen.view),
                    depth_slice: None,
                    resolve_target: t.color.as_ref().map(|_| &t.screen.view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            // The scene's order: the two light markers, then the bunny ( added
            // when the FBX loads ).
            pass.set_pipeline(&t.pipelines[1]);
            pass.set_vertex_buffer(0, self.sphere.0.slice(..));
            pass.set_index_buffer(self.sphere.1.slice(..), wgpu::IndexFormat::Uint32);
            for k in 1..3 {
                pass.set_bind_group(0, &self.groups[k], &[]);
                pass.draw_indexed(0..self.sphere.2, 0, 0..1);
            }
            pass.set_pipeline(&t.pipelines[0]);
            pass.set_bind_group(0, &self.groups[0], &[]);
            pass.set_vertex_buffer(0, self.bunny.0.slice(..));
            pass.draw(0..self.bunny.1, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
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
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        self.controls.update(s, c)
    }
    /// The GUI: distortion, ambient, attenuation, power, scale.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("subsurface parameter"))? = f64::from(value);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.elapsed = t;
        self.pending = true;
    }
}
