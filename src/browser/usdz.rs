//! webgl_loader_usdz: the saeukkang USDZ model lit only by the
//! venice_sunset HDR environment, with the same HDR as a background blurred
//! to roughness 0.5, ACES Filmic tone mapping at exposure 2 and OrbitControls.
//! The model is USDLoader's parse of the archive, baked by
//! tools/tsl/prepare-usdz.mjs: one mesh with its non-indexed attributes and
//! MeshPhysicalMaterial ( roughness 1, IOR 1.5, the archive's base color
//! image ).
//!
//! WebGLRenderer prefilters the equirectangular HDR into a PMREM cubeUV
//! texture ( PMREMGenerator.fromEquirectangular, GGX VNDF importance
//! sampling ) for both the environment and the blurred background; the port
//! runs r186's PMREM generator as the WebGPU renderer generates it ( the same
//! algorithm and layout, stored with WebGPU's vertical orientation, which its
//! lookups account for ). The model's program is a WGSL port of the GLSL
//! WebGLRenderer builds for MeshPhysicalMaterial with a CUBE_UV environment
//! and no lights: the IBL radiance and irradiance through the r186 DFG LUT's
//! multiple scattering. The background is the cubeUV lookup along each
//! pixel's view ray. The canvas is tone mapped and sRGB-encoded in the shader
//! and multisampled when the page's antialias is.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::lights_projector::{m4, pack};
use super::pmrem_cube_uv::{GGX_8, Pmrem, bind, flat_camera, source_pipeline};
use super::retro::mipmapped;
use super::shadowmap_opacity::Mipmaps;
use super::trackball_sprites::parse_rgbe;
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use wgpu::util::DeviceExt;

const ASSETS: &str = "/web/gallery/assets/usdz";
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const SHADER: &str = r#"
struct Frame {
	view: mat4x4<f32>,
	projection: mat4x4<f32>,
	model: mat4x4<f32>,
	inverse_view_projection: mat4x4<f32>,
	camera: vec4<f32>,
	// toneMappingExposure, environmentIntensity, backgroundBlurriness,
	// backgroundIntensity.
	params: vec4<f32>,
};
@group( 0 ) @binding( 0 ) var<uniform> frame: Frame;
@group( 0 ) @binding( 1 ) var env_map: texture_2d<f32>;
@group( 0 ) @binding( 2 ) var env_sampler: sampler;
@group( 0 ) @binding( 3 ) var map: texture_2d<f32>;
@group( 0 ) @binding( 4 ) var map_sampler: sampler;
@group( 0 ) @binding( 5 ) var dfg_lut: texture_2d<f32>;
// CUBEUV_MAX_MIP 8, CUBEUV_TEXEL_WIDTH 1 / 768, CUBEUV_TEXEL_HEIGHT 1 / 1024.
const MAX_MIP: f32 = 8.0;
const TEXEL: vec2<f32> = vec2( 1.0 / 768.0, 1.0 / 1024.0 );
fn roughness_to_mip( roughness: f32 ) -> f32 {
	if ( roughness >= 0.8 ) { return ( 1.0 - roughness ) * ( -1.0 - -2.0 ) / ( 1.0 - 0.8 ) + -2.0; }
	if ( roughness >= 0.4 ) { return ( 0.8 - roughness ) * ( 2.0 - -1.0 ) / ( 0.8 - 0.4 ) + -1.0; }
	if ( roughness >= 0.305 ) { return ( 0.4 - roughness ) * ( 3.0 - 2.0 ) / ( 0.4 - 0.305 ) + 2.0; }
	if ( roughness >= 0.21 ) { return ( 0.305 - roughness ) * ( 4.0 - 3.0 ) / ( 0.305 - 0.21 ) + 3.0; }
	return -2.0 * log2( 1.16 * roughness );
}
fn get_face( direction: vec3<f32> ) -> f32 {
	let a = abs( direction );
	if ( a.x > a.z ) {
		if ( a.x > a.y ) { return select( 3.0, 0.0, direction.x > 0.0 ); }
		return select( 4.0, 1.0, direction.y > 0.0 );
	}
	if ( a.z > a.y ) { return select( 5.0, 2.0, direction.z > 0.0 ); }
	return select( 4.0, 1.0, direction.y > 0.0 );
}
fn get_uv( direction: vec3<f32>, face: f32 ) -> vec2<f32> {
	var uv: vec2<f32>;
	if ( face == 0.0 ) { uv = vec2( direction.z, direction.y ) / abs( direction.x ); }
	else if ( face == 1.0 ) { uv = vec2( -direction.x, -direction.z ) / abs( direction.y ); }
	else if ( face == 2.0 ) { uv = vec2( -direction.x, direction.y ) / abs( direction.z ); }
	else if ( face == 3.0 ) { uv = vec2( -direction.z, direction.y ) / abs( direction.x ); }
	else if ( face == 4.0 ) { uv = vec2( -direction.x, direction.z ) / abs( direction.y ); }
	else { uv = vec2( direction.x, direction.y ) / abs( direction.z ); }
	return 0.5 * ( uv + 1.0 );
}
fn bilinear_cube_uv( direction: vec3<f32>, mip_in: f32 ) -> vec3<f32> {
	var face = get_face( direction );
	let filter_int = max( 4.0 - mip_in, 0.0 );
	let mip = max( mip_in, 4.0 );
	let face_size = exp2( mip );
	var uv = get_uv( direction, face ) * ( face_size - 2.0 ) + 1.0;
	if ( face > 2.0 ) {
		uv.y += face_size;
		face -= 3.0;
	}
	uv.x += face * face_size;
	uv.x += filter_int * 3.0 * 16.0;
	uv.y += 4.0 * ( exp2( MAX_MIP ) - face_size );
	return textureSampleLevel( env_map, env_sampler, uv * TEXEL, 0.0 ).rgb;
}
// textureCubeUV on the WebGPU-oriented layout ( y flipped ).
fn texture_cube_uv( world: vec3<f32>, roughness: f32 ) -> vec3<f32> {
	let direction = vec3( world.x, -world.y, world.z );
	let mip = clamp( roughness_to_mip( roughness ), -2.0, MAX_MIP );
	let mip_f = fract( mip );
	let mip_int = floor( mip );
	let color0 = bilinear_cube_uv( direction, mip_int );
	if ( mip_f == 0.0 ) { return color0; }
	return mix( color0, bilinear_cube_uv( direction, mip_int + 1.0 ), mip_f );
}
fn output( color: vec3<f32> ) -> vec4<f32> {
	// ACESFilmicToneMapping.
	let input = mat3x3<f32>( vec3( 0.59719, 0.07600, 0.02840 ), vec3( 0.35458, 0.90834, 0.13383 ), vec3( 0.04823, 0.01566, 0.83777 ) );
	let out = mat3x3<f32>( vec3( 1.60475, -0.10208, -0.00327 ), vec3( -0.53108, 1.10813, -0.07276 ), vec3( -0.07367, -0.00605, 1.07602 ) );
	var c = input * ( color * frame.params.x / 0.6 );
	let a = c * ( c + 0.0245786 ) - 0.000090537;
	let b = c * ( 0.983729 * c + 0.4329510 ) + 0.238081;
	c = saturate( out * ( a / b ) );
	return vec4( select( pow( c, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), c * 12.92, c <= vec3( 0.0031308 ) ), 1.0 );
}
struct Varyings {
	@builtin( position ) position: vec4<f32>,
	@location( 0 ) view_position: vec3<f32>,
	@location( 1 ) normal: vec3<f32>,
	@location( 2 ) uv: vec2<f32>,
};
@vertex fn model_vs( @location( 0 ) position: vec3<f32>, @location( 1 ) normal: vec3<f32>, @location( 2 ) uv: vec2<f32> ) -> Varyings {
	let mv = frame.view * frame.model * vec4( position, 1.0 );
	var out: Varyings;
	out.position = frame.projection * mv;
	out.view_position = -mv.xyz;
	out.normal = normalize( ( frame.view * frame.model * vec4( normal, 0.0 ) ).xyz );
	out.uv = uv;
	return out;
}
const RECIPROCAL_PI: f32 = 0.3183098861837907;
fn multiscattering( fab: vec2<f32>, specular: vec3<f32>, f90: f32 ) -> array<vec3<f32>, 2> {
	let fss_ess = specular * fab.x + f90 * fab.y;
	let ess = fab.x + fab.y;
	let ems = 1.0 - ess;
	let favg = specular + ( 1.0 - specular ) * 0.047619;
	let fms = fss_ess * favg / ( 1.0 - ems * favg );
	return array( fss_ess, fms * ems );
}
@fragment fn model_fs( in: Varyings ) -> @location( 0 ) vec4<f32> {
	// The map is uploaded with flipY.
	let diffuse = textureSample( map, map_sampler, vec2( in.uv.x, 1.0 - in.uv.y ) ).rgb;
	let normal = normalize( in.normal );
	let dxy = max( abs( dpdx( normal ) ), abs( dpdy( normal ) ) );
	let roughness = min( max( 1.0, 0.0525 ) + max( max( dxy.x, dxy.y ), dxy.z ), 1.0 );
	let view = normalize( in.view_position );
	let dot_nv = saturate( dot( normal, view ) );
	let dfg = textureSampleLevel( dfg_lut, env_sampler, vec2( roughness, dot_nv ), 0.0 ).rg;
	// IOR 1.5: specularColor 0.04, specularF90 1.
	let specular = vec3( pow( ( 1.5 - 1.0 ) / ( 1.5 + 1.0 ), 2.0 ) );
	let to_world = transpose( mat3x3<f32>( frame.view[ 0 ].xyz, frame.view[ 1 ].xyz, frame.view[ 2 ].xyz ) );
	let reflected = normalize( mix( reflect( -view, normal ), normal, roughness * roughness * roughness * roughness ) );
	let radiance = texture_cube_uv( normalize( to_world * reflected ), roughness ) * frame.params.y;
	let irradiance = 3.141592653589793 * texture_cube_uv( normalize( to_world * normal ), 1.0 ) * frame.params.y;
	let scattering = multiscattering( dfg, specular, 1.0 );
	let diffuse_layer = diffuse * ( 1.0 - scattering[ 0 ] - scattering[ 1 ] );
	let cosine = irradiance * RECIPROCAL_PI;
	let light = radiance * scattering[ 0 ] + scattering[ 1 ] * cosine + diffuse_layer * cosine;
	return output( light );
}
struct Background { @builtin( position ) position: vec4<f32>, @location( 0 ) ndc: vec2<f32> };
@vertex fn background_vs( @builtin( vertex_index ) index: u32 ) -> Background {
	let ndc = vec2( f32( index & 1u ) * 4.0 - 1.0, f32( index >> 1u ) * 4.0 - 1.0 );
	return Background( vec4( ndc, 1.0, 1.0 ), ndc );
}
@fragment fn background_fs( in: Background ) -> @location( 0 ) vec4<f32> {
	let far = frame.inverse_view_projection * vec4( in.ndc, 1.0, 1.0 );
	let direction = normalize( far.xyz / far.w - frame.camera.xyz );
	return output( texture_cube_uv( direction, frame.params.z ) * frame.params.w );
}
"#;
#[derive(serde::Deserialize)]
struct Bake {
    vertices: u32,
    #[serde(rename = "matrixWorld")]
    matrix_world: [f64; 16],
}
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
    model: Matrix4,
    vertices: (wgpu::Buffer, u32),
    frame: wgpu::Buffer,
    group: wgpu::BindGroup,
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
            fov: 60.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0.75, -1.5);
        let mut controls = Controls::new(None, (1., 8.), std::f64::consts::PI, true);
        controls.update(s, c)?;
        let init = |label, contents: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        let bake: Bake = serde_json::from_slice(&fetch(&format!("{ASSETS}/saeukkang.json")).await?)
            .map_err(|e| Error::Asset(e.to_string()))?;
        let bytes = fetch(&format!("{ASSETS}/saeukkang.bin")).await?;
        let n = bake.vertices as usize;
        if bytes.len() != n * 8 * 4 {
            return Err(Error::Invalid("usdz bake"));
        }
        let floats: Vec<f32> = bytemuck::pod_collect_to_vec(&bytes);
        let (positions, rest) = floats.split_at(n * 3);
        let (normals, uvs) = rest.split_at(n * 3);
        let mut data = Vec::with_capacity(n * 8);
        for k in 0..n {
            data.extend_from_slice(&positions[k * 3..k * 3 + 3]);
            data.extend_from_slice(&normals[k * 3..k * 3 + 3]);
            data.extend_from_slice(&uvs[k * 2..k * 2 + 2]);
        }
        let mut mipmaps = Mipmaps::new(r);
        let image = decode_texture_image(&fetch(&format!("{ASSETS}/saeukkang.png")).await?).await?;
        let map = mipmapped(r, &mut mipmaps, &image, wgpu::TextureFormat::Rgba8UnormSrgb);
        let map_sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // HDRLoader's DataTexture: flipY, linear, no mipmaps.
        let (width, height, texels) =
            parse_rgbe(&fetch(&format!("{ASSETS}/venice_sunset_1k.hdr")).await?)?;
        let row = width as usize * 4;
        let texels: Vec<u16> = texels.chunks(row).rev().flatten().copied().collect();
        let equirect = r
            .device
            .create_texture_with_data(
                &r.queue,
                &wgpu::TextureDescriptor {
                    label: Some("usdz environment"),
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
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                bytemuck::cast_slice(&texels),
            )
            .create_view(&Default::default());
        let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // PMREMGenerator.fromEquirectangular: a 256 cube ( width / 4 ).
        let pmrem = Pmrem::new(r, &clamp, 8, GGX_8)?;
        {
            let source = source_pipeline(
                r,
                "usdz PMREM equirect",
                include_str!("retro/pmrem_equirect_vs.wgsl"),
                include_str!("retro/pmrem_equirect_fs.wgsl"),
            );
            let object = init(
                "usdz PMREM",
                &pack(
                    include_str!("retro/pmrem_equirect_vs.wgsl"),
                    "objectStruct",
                    &[("nodeUniform3", &m4(Matrix4::IDENTITY))],
                )?,
                wgpu::BufferUsages::UNIFORM,
            );
            let source_groups = [
                bind(
                    r,
                    source.get_bind_group_layout(0),
                    &[(
                        0,
                        flat_camera(r, include_str!("retro/pmrem_equirect_vs.wgsl"))?
                            .as_entire_binding(),
                    )],
                ),
                bind(
                    r,
                    source.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::Sampler(&clamp)),
                        (1, wgpu::BindingResource::TextureView(&equirect)),
                        (2, object.as_entire_binding()),
                    ],
                ),
            ];
            let mut encoder = r.device.create_command_encoder(&Default::default());
            pmrem.encode(
                &mut encoder,
                &source,
                [&source_groups[0], &source_groups[1]],
            );
            r.queue.submit([encoder.finish()]);
        }
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty,
            count: None,
        };
        let texture = wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        };
        let filtering = wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering);
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("usdz"),
                entries: &[
                    entry(
                        0,
                        wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                    ),
                    entry(1, texture),
                    entry(2, filtering),
                    entry(3, texture),
                    entry(4, filtering),
                    entry(5, texture),
                ],
            });
        let frame = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("usdz frame"),
            size: 64 * 4 + 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("usdz"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: frame.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&pmrem.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&clamp),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&map),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(&map_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(&r.dfg),
                },
            ],
        });
        // The page offsets the model to ( 0, 0.25, -0.25 ).
        let model = Matrix4::from_translation(Vector3::new(0., 0.25, -0.25))
            * Matrix4::from_cols_array(&bake.matrix_world);
        Ok(Self {
            controls,
            pending: true,
            model,
            vertices: (
                init(
                    "usdz vertices",
                    bytemuck::cast_slice(&data),
                    wgpu::BufferUsages::VERTEX,
                ),
                bake.vertices,
            ),
            frame,
            group,
            layout,
            targets: None,
        })
    }
    fn pipeline(
        &self,
        r: &Renderer,
        model: bool,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> wgpu::RenderPipeline {
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("usdz"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("usdz"),
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
        let buffers = [wgpu::VertexBufferLayout {
            array_stride: 32,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes,
        }];
        r.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("usdz"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some(if model { "model_vs" } else { "background_vs" }),
                    compilation_options: Default::default(),
                    buffers: if model { &buffers } else { &[] },
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some(if model { "model_fs" } else { "background_fs" }),
                    compilation_options: Default::default(),
                    targets: &[Some(format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    cull_mode: model.then_some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH,
                    depth_write_enabled: model,
                    depth_compare: if model {
                        wgpu::CompareFunction::LessEqual
                    } else {
                        wgpu::CompareFunction::Always
                    },
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
                color: (samples > 1).then(|| attachment(out.options.format, "usdz color")),
                depth: attachment(DEPTH, "usdz depth"),
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
        let projection = camera.projection_matrix()?;
        let mut data: Vec<f32> = vec![];
        for m in [view, projection, self.model, (projection * view).inverse()] {
            data.extend(m.to_cols_array().iter().map(|&v| v as f32));
        }
        let eye = world.w_axis;
        data.extend([eye.x as f32, eye.y as f32, eye.z as f32, 1.]);
        data.extend([2., 1., 0.5, 1.]);
        r.queue
            .write_buffer(&self.frame, 0, bytemuck::cast_slice(&data));
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("usdz targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("usdz"),
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
            pass.set_bind_group(0, &self.group, &[]);
            // The background box first, then the model.
            pass.set_pipeline(&t.pipelines[1]);
            pass.draw(0..3, 0..1);
            pass.set_pipeline(&t.pipelines[0]);
            pass.set_vertex_buffer(0, self.vertices.0.slice(..));
            pass.draw(0..self.vertices.1, 0..1);
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
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("usdz parameter"))
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
