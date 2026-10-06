//! webgl_postprocessing_dof2: 100 falling double-sided leaves, 20 flat-shaded
//! Suzanne heads and 20 colored balls, all Phong with the Bridge2 cube as a
//! multiplied reflection, under an ambient and two directional lights. The
//! camera circles the target by the clock; each frame the page raycasts the
//! scene through the pointer on the CPU ( the original's autofocus, before
//! the leaves move ), eases the focus distance toward the hit by 3 %, then
//! renders the scene ( with the cube background ) into a half-float target,
//! renders it again with BokehDepthShader as the override material ( the
//! background keeps its own material ) and composites BokehShader2 on a
//! full-screen quad. The targets are sized in CSS pixels, as the page sizes
//! them, and the shaders are WGSL ports of the GLSL programs WebGLRenderer
//! builds for them ( MeshPhongMaterial with a cube environment, the
//! background cube, the depth shader and BokehShader2 with its RINGS and
//! SAMPLES defines as pipeline constants ). Each leaf, head and ball in view
//! is drawn individually after frustum culling, as WebGLRenderer draws them,
//! from one per-object storage buffer; only the leaves' matrices change each
//! frame.
use super::controls_attributes::viewport_css;
use super::environment_materials::cube;
use super::gltf_viewer::fetch;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const LEAVES: usize = 100;
const MONKEYS: usize = 20;
const BALLS: usize = 20;
const NEAR: f64 = 1.;
const FAR: f64 = 3000.;
const TARGET: Vector3 = Vector3::new(0., 20., -50.);
/// Frame uniforms shared by the scene, background and depth programs.
const COMMON: &str = r#"
struct Frame {
	view: mat4x4<f32>,
	projection: mat4x4<f32>,
	inverse_view_projection: mat4x4<f32>,
	camera: vec4<f32>,
	// View-space directions to the lights, and their colors.
	light0: vec4<f32>,
	color0: vec4<f32>,
	light1: vec4<f32>,
	color1: vec4<f32>,
	ambient: vec4<f32>,
	// mNear, mFar.
	depth: vec4<f32>,
};
struct Object { model: mat4x4<f32>, color: vec4<f32> };
@group( 0 ) @binding( 0 ) var<uniform> frame: Frame;
@group( 0 ) @binding( 1 ) var<storage, read> objects: array<Object>;
@group( 0 ) @binding( 2 ) var env: texture_cube<f32>;
@group( 0 ) @binding( 3 ) var env_sampler: sampler;
override ENCODE: bool = false;
// sRGBTransferOETF.
fn output( c: vec3<f32> ) -> vec4<f32> {
	if ( ENCODE ) {
		return vec4( select( pow( c, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), c * 12.92, c <= vec3( 0.0031308 ) ), 1.0 );
	}
	return vec4( c, 1.0 );
}
"#;
/// MeshPhongMaterial ( BlinnPhong, envMap with MultiplyOperation and
/// reflectivity 1 ); FLAT derives the normal from the view position's
/// derivatives ( WebGL's dFdy runs up the screen ), DOUBLE flips back faces.
const PHONG: &str = r#"
override FLAT: bool = false;
override DOUBLE: bool = false;
struct Varyings {
	@builtin( position ) position: vec4<f32>,
	@location( 0 ) view_position: vec3<f32>,
	@location( 1 ) normal: vec3<f32>,
	@location( 2 ) world: vec3<f32>,
	@location( 3 ) @interpolate( flat ) object: u32,
};
@vertex fn vs( @location( 0 ) position: vec3<f32>, @location( 1 ) normal: vec3<f32>, @builtin( instance_index ) object: u32 ) -> Varyings {
	let model = objects[ object ].model;
	let world = model * vec4( position, 1.0 );
	let mv = frame.view * world;
	var out: Varyings;
	out.position = frame.projection * mv;
	out.view_position = -mv.xyz;
	out.normal = ( frame.view * model * vec4( normal, 0.0 ) ).xyz;
	out.world = world.xyz;
	out.object = object;
	return out;
}
const RECIPROCAL_PI: f32 = 0.3183098861837907;
fn blinn_phong( light: vec3<f32>, view: vec3<f32>, normal: vec3<f32>, specular: vec3<f32>, shininess: f32 ) -> vec3<f32> {
	let half_dir = normalize( light + view );
	let dot_nh = saturate( dot( normal, half_dir ) );
	let dot_vh = saturate( dot( view, half_dir ) );
	let fresnel = exp2( ( -5.55473 * dot_vh - 6.98316 ) * dot_vh );
	let f = specular * ( 1.0 - fresnel ) + vec3( fresnel );
	let d = RECIPROCAL_PI * ( shininess * 0.5 + 1.0 ) * pow( dot_nh, shininess );
	return f * ( 0.25 * d );
}
@fragment fn fs( in: Varyings, @builtin( front_facing ) front: bool ) -> @location( 0 ) vec4<f32> {
	let object = objects[ in.object ];
	let diffuse = object.color.rgb;
	let shininess = object.color.w;
	var normal: vec3<f32>;
	if ( FLAT ) {
		normal = normalize( cross( dpdx( in.view_position ), -dpdy( in.view_position ) ) );
	} else {
		normal = normalize( in.normal );
		if ( DOUBLE && !front ) { normal = -normal; }
	}
	let view = normalize( in.view_position );
	var light = vec3( 0.0 );
	for ( var k = 0; k < 2; k++ ) {
		let direction = select( frame.light1.xyz, frame.light0.xyz, k == 0 );
		let color = select( frame.color1.rgb, frame.color0.rgb, k == 0 );
		let irradiance = saturate( dot( normal, direction ) ) * color;
		light += irradiance * RECIPROCAL_PI * diffuse + irradiance * blinn_phong( direction, view, normal, vec3( 1.0 ), shininess );
	}
	light += frame.ambient.rgb * RECIPROCAL_PI * diffuse;
	let to_fragment = normalize( in.world - frame.camera.xyz );
	let world_normal = normalize( ( vec4( normal, 0.0 ) * frame.view ).xyz );
	let r = reflect( to_fragment, world_normal );
	let env_color = textureSample( env, env_sampler, vec3( -r.x, r.yz ) );
	return output( light * env_color.rgb );
}
"#;
/// BokehDepthShader.
const DEPTH_SHADER: &str = r#"
struct Varyings { @builtin( position ) position: vec4<f32>, @location( 0 ) depth: f32 };
@vertex fn vs( @location( 0 ) position: vec3<f32>, @builtin( instance_index ) object: u32 ) -> Varyings {
	let mv = frame.view * objects[ object ].model * vec4( position, 1.0 );
	return Varyings( frame.projection * mv, -mv.z );
}
@fragment fn fs( in: Varyings ) -> @location( 0 ) vec4<f32> {
	return vec4( vec3( 1.0 - smoothstep( frame.depth.x, frame.depth.y, in.depth ) ), 1.0 );
}
"#;
/// The background cube ( flipEnvMap -1 ), as a full-screen triangle along
/// each pixel's view ray.
const BACKGROUND: &str = r#"
struct Varyings { @builtin( position ) position: vec4<f32>, @location( 0 ) ndc: vec2<f32> };
@vertex fn vs( @builtin( vertex_index ) index: u32 ) -> Varyings {
	let ndc = vec2( f32( index & 1u ) * 4.0 - 1.0, f32( index >> 1u ) * 4.0 - 1.0 );
	return Varyings( vec4( ndc, 1.0, 1.0 ), ndc );
}
@fragment fn fs( in: Varyings ) -> @location( 0 ) vec4<f32> {
	let far = frame.inverse_view_projection * vec4( in.ndc, 1.0, 1.0 );
	let direction = far.xyz / far.w - frame.camera.xyz;
	return output( textureSample( env, env_sampler, vec3( -direction.x, direction.yz ) ).rgb );
}
"#;
/// BokehShader2's fragment program. vUv runs up the screen as WebGL's does;
/// the targets are sampled with v flipped.
const BOKEH: &str = r#"
struct Bokeh {
	size: vec2<f32>,
	focal_depth: f32,
	focal_length: f32,
	fstop: f32,
	maxblur: f32,
	threshold: f32,
	gain: f32,
	bias: f32,
	fringe: f32,
	znear: f32,
	zfar: f32,
	dithering: f32,
	show_focus: u32,
	manualdof: u32,
	vignetting: u32,
	depthblur: u32,
	pentagon: u32,
	shader_focus: u32,
	pad: u32,
	focus_coords: vec2<f32>,
	screen: vec2<f32>,
};
@group( 0 ) @binding( 0 ) var<uniform> u: Bokeh;
@group( 0 ) @binding( 1 ) var t_color: texture_2d<f32>;
@group( 0 ) @binding( 2 ) var t_depth: texture_2d<f32>;
@group( 0 ) @binding( 3 ) var linear_sampler: sampler;
override RINGS: i32 = 3;
override SAMPLES: i32 = 4;
const PI: f32 = 3.141592653589793;
const ndofstart = 1.0;
const ndofdist = 2.0;
const fdofstart = 1.0;
const fdofdist = 3.0;
const CoC = 0.03;
const vignout = 1.3;
const vignin = 0.0;
const vignfade = 22.0;
const dbsize = 1.25;
const feather = 0.4;
fn gl( t: texture_2d<f32>, uv: vec2<f32> ) -> vec4<f32> {
	return textureSampleLevel( t, linear_sampler, vec2( uv.x, 1.0 - uv.y ), 0.0 );
}
fn rand( uv: vec2<f32> ) -> f32 {
	let dt = dot( uv, vec2( 12.9898, 78.233 ) );
	let sn = dt - PI * floor( dt / PI );
	return fract( sin( sn ) * 43758.5453 );
}
fn penta( coords: vec2<f32> ) -> f32 {
	let scale = f32( RINGS ) - 1.3;
	let HS0 = vec4( 1.0, 0.0, 0.0, 1.0 );
	let HS1 = vec4( 0.309016994, 0.951056516, 0.0, 1.0 );
	let HS2 = vec4( -0.809016994, 0.587785252, 0.0, 1.0 );
	let HS3 = vec4( -0.809016994, -0.587785252, 0.0, 1.0 );
	let HS4 = vec4( 0.309016994, -0.951056516, 0.0, 1.0 );
	let HS5 = vec4( 0.0, 0.0, 1.0, 1.0 );
	let P = vec4( coords, vec2( scale, scale ) );
	var dist = vec4( dot( P, HS0 ), dot( P, HS1 ), dot( P, HS2 ), dot( P, HS3 ) );
	dist = smoothstep( vec4( -feather ), vec4( feather ), dist );
	var inorout = -4.0 + dot( dist, vec4( 1.0 ) );
	dist = vec4( dot( P, HS4 ), HS5.w - abs( P.z ), dist.z, dist.w );
	dist = smoothstep( vec4( -feather ), vec4( feather ), dist );
	inorout += dist.x;
	return clamp( inorout, 0.0, 1.0 );
}
fn bdepth( coords: vec2<f32> ) -> f32 {
	let wh = vec2( 1.0 / u.size.x, 1.0 / u.size.y ) * dbsize;
	// offset[ 2 ] is vec2( wh.x - wh.y ) in the original.
	let offsets = array<vec2<f32>, 9>( vec2( -wh.x, -wh.y ), vec2( 0.0, -wh.y ), vec2( wh.x - wh.y ), vec2( -wh.x, 0.0 ), vec2( 0.0 ), vec2( wh.x, 0.0 ), vec2( -wh.x, wh.y ), vec2( 0.0, wh.y ), vec2( wh.x, wh.y ) );
	let kernel = array<f32, 9>( 1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0, 2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0 );
	var d = 0.0;
	for ( var i = 0; i < 9; i++ ) {
		d += gl( t_depth, coords + offsets[ i ] ).r * kernel[ i ];
	}
	return d;
}
fn color( coords: vec2<f32>, blur: f32 ) -> vec3<f32> {
	let texel = vec2( 1.0 / u.size.x, 1.0 / u.size.y );
	var col = vec3( 0.0 );
	col.r = gl( t_color, coords + vec2( 0.0, 1.0 ) * texel * u.fringe * blur ).r;
	col.g = gl( t_color, coords + vec2( -0.866, -0.5 ) * texel * u.fringe * blur ).g;
	col.b = gl( t_color, coords + vec2( 0.866, -0.5 ) * texel * u.fringe * blur ).b;
	let lum = dot( col, vec3( 0.299, 0.587, 0.114 ) );
	let thresh = max( ( lum - u.threshold ) * u.gain, 0.0 );
	return col + mix( vec3( 0.0 ), col, thresh * blur );
}
fn debug_focus( c: vec3<f32>, blur: f32, depth: f32 ) -> vec3<f32> {
	let edge = 0.002 * depth;
	let m = clamp( smoothstep( 0.0, edge, blur ), 0.0, 1.0 );
	let e = clamp( smoothstep( 1.0 - edge, 1.0, blur ), 0.0, 1.0 );
	var col = mix( c, vec3( 1.0, 0.5, 0.0 ), ( 1.0 - m ) * 0.6 );
	col = mix( col, vec3( 0.0, 0.5, 1.0 ), ( ( 1.0 - e ) - ( 1.0 - m ) ) * 0.2 );
	return col;
}
fn linearize( depth: f32 ) -> f32 {
	return -u.zfar * u.znear / ( depth * ( u.zfar - u.znear ) - u.zfar );
}
@vertex fn vs( @builtin( vertex_index ) index: u32 ) -> @builtin( position ) vec4<f32> {
	return vec4( f32( index & 1u ) * 4.0 - 1.0, f32( index >> 1u ) * 4.0 - 1.0, 0.0, 1.0 );
}
@fragment fn fs( @builtin( position ) position: vec4<f32> ) -> @location( 0 ) vec4<f32> {
	let uv = vec2( position.x / u.screen.x, 1.0 - position.y / u.screen.y );
	var depth = linearize( gl( t_depth, uv ).x );
	if ( u.depthblur != 0u ) { depth = linearize( bdepth( uv ) ); }
	var f_depth = u.focal_depth;
	if ( u.shader_focus != 0u ) { f_depth = linearize( gl( t_depth, u.focus_coords ).x ); }
	var blur = 0.0;
	if ( u.manualdof != 0u ) {
		let a = depth - f_depth;
		let b = ( a - fdofstart ) / fdofdist;
		let c = ( -a - ndofstart ) / ndofdist;
		blur = select( c, b, a > 0.0 );
	} else {
		let f = u.focal_length;
		let d = f_depth * 1000.0;
		let o = depth * 1000.0;
		let a = ( o * f ) / ( o - f );
		let b = ( d * f ) / ( d - f );
		let c = ( d - f ) / ( d * u.fstop * CoC );
		blur = abs( a - b ) * c;
	}
	blur = clamp( blur, 0.0, 1.0 );
	let noise = vec2( rand( uv ), rand( uv + vec2( 0.4, 0.6 ) ) ) * u.dithering * blur;
	let w = ( 1.0 / u.size.x ) * blur * u.maxblur + noise.x;
	let h = ( 1.0 / u.size.y ) * blur * u.maxblur + noise.y;
	var col = gl( t_color, uv ).rgb;
	if ( blur >= 0.05 ) {
		var s = 1.0;
		let rings = f32( RINGS );
		for ( var i = 1; i <= RINGS; i++ ) {
			let ringsamples = i * SAMPLES;
			for ( var j = 0; j < RINGS * SAMPLES; j++ ) {
				if ( j >= ringsamples ) { break; }
				let step = PI * 2.0 / f32( ringsamples );
				let pw = cos( f32( j ) * step ) * f32( i );
				let ph = sin( f32( j ) * step ) * f32( i );
				var p = 1.0;
				if ( u.pentagon != 0u ) { p = penta( vec2( pw, ph ) ); }
				let weight = mix( 1.0, f32( i ) / rings, u.bias ) * p;
				col += color( uv + vec2( pw * w, ph * h ), blur ) * weight;
				s += weight;
			}
		}
		col /= s;
	}
	if ( u.show_focus != 0u ) { col = debug_focus( col, blur, depth ); }
	if ( u.vignetting != 0u ) {
		let dist = smoothstep( vignout + ( u.fstop / vignfade ), vignin + ( u.fstop / vignfade ), distance( uv, vec2( 0.5 ) ) );
		col *= clamp( dist, 0.0, 1.0 );
	}
	return vec4( select( pow( col, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), col * 12.92, col <= vec3( 0.0031308 ) ), 1.0 );
}
"#;
/// The fixture's Math.random: a 32-bit LCG seeded with 186.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(self.0) / 4294967296.
    }
}
/// `Color.setHex` with color management: sRGB bytes to linear.
fn linear(hex: f64) -> [f32; 3] {
    let hex = hex.floor() as u32;
    [16, 8, 0].map(|shift| {
        let c = f64::from((hex >> shift) & 255) / 255.;
        (if c < 0.04045 {
            c * 0.0773993808
        } else {
            (c * 0.9478672986 + 0.0521327014).powf(2.4)
        }) as f32
    })
}
/// `Matrix4.lookAt` for a camera at `eye` facing `target` ( Y up ).
fn look_at(eye: Vector3, target: Vector3) -> Matrix4 {
    let mut z = eye - target;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    z = z.normalize();
    let mut x = Vector3::Y.cross(z);
    if x.length_squared() == 0. {
        z.z += 0.0001;
        z = z.normalize();
        x = Vector3::Y.cross(z);
    }
    let x = x.normalize();
    Matrix4::from_cols(
        x.extend(0.),
        z.cross(x).extend(0.),
        z.extend(0.),
        eye.extend(1.),
    )
}
/// `Euler` ( XYZ ) to a rotation matrix.
fn euler(x: f64, y: f64, z: f64) -> Matrix4 {
    Matrix4::from_rotation_x(x) * Matrix4::from_rotation_y(y) * Matrix4::from_rotation_z(z)
}
/// The camera's projection: x and y as `Matrix4.makePerspective`, depth 0..1.
fn perspective(fov: f64, aspect: f64) -> Matrix4 {
    let top = NEAR * (fov.to_radians() * 0.5).tan();
    let (height, width) = (2. * top, 2. * top * aspect);
    Matrix4::from_cols_array(&[
        2. * NEAR / width,
        0.,
        0.,
        0.,
        0.,
        2. * NEAR / height,
        0.,
        0.,
        0.,
        0.,
        -FAR / (FAR - NEAR),
        -1.,
        0.,
        0.,
        -FAR * NEAR / (FAR - NEAR),
        0.,
    ])
}
/// A mesh shape: interleaved position and normal, index, and the CPU copy
/// the raycast reads.
struct Shape {
    vertices: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    positions: Vec<Vector3>,
    triangles: Vec<u32>,
    /// computeBoundingSphere: the box center and the farthest vertex.
    sphere: (Vector3, f64),
}
/// A leaf: its Euler angles and position, and their per-frame steps.
struct Leaf {
    rotation: [f64; 3],
    spin: [f64; 3],
    position: Vector3,
    drift: [f64; 2],
}
struct Targets {
    size: (u32, u32),
    css: (u32, u32),
    format: wgpu::TextureFormat,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    scene_depth: wgpu::TextureView,
    screen_depth: wgpu::TextureView,
    bokeh_group: wgpu::BindGroup,
    screen: RenderTarget,
    /// The phong ( smooth double-sided, smooth, flat ) and background
    /// pipelines into the screen, for the disabled effect.
    direct: [wgpu::RenderPipeline; 4],
}
pub(super) struct Demo {
    pending: bool,
    elapsed: f64,
    fov: f64,
    /// The GUI values ( see `parameter` ).
    params: [f64; 20],
    pointer: Vector2,
    focus: Vector2,
    distance: f64,
    /// The objects' world matrices as of the last render ( identity before
    /// the first one, as the raycast sees them ).
    rendered: bool,
    leaves: Vec<Leaf>,
    /// World matrices: leaves, heads, balls.
    worlds: Vec<Matrix4>,
    colors: Vec<[f32; 4]>,
    shapes: [Shape; 3],
    frame: wgpu::Buffer,
    objects: wgpu::Buffer,
    bokeh: wgpu::Buffer,
    group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
    /// Phong ( smooth double-sided, smooth, flat ), background, depth.
    pipelines: [wgpu::RenderPipeline; 5],
    /// The bokeh programs by RINGS, SAMPLES and output format ( the page
    /// recompiles on a change; a revisited setting reuses its program ).
    bokeh_pipelines:
        std::collections::HashMap<(i32, i32, wgpu::TextureFormat), wgpu::RenderPipeline>,
    bokeh_layout: wgpu::BindGroupLayout,
    targets: Option<Targets>,
}
fn texture(
    r: &Renderer,
    label: &str,
    (width, height): (u32, u32),
    format: wgpu::TextureFormat,
) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
        .create_view(&Default::default())
}
/// A scene program: `kind` 0..2 Phong ( double-sided, smooth, flat ), 3 the
/// background, 4 the depth shader.
fn scene_pipeline(
    r: &Renderer,
    layout: &wgpu::BindGroupLayout,
    kind: usize,
    format: wgpu::TextureFormat,
    encode: bool,
) -> wgpu::RenderPipeline {
    let source = [PHONG, PHONG, PHONG, BACKGROUND, DEPTH_SHADER][kind];
    let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("dof2"),
        source: wgpu::ShaderSource::Wgsl(format!("{COMMON}{source}").into()),
    });
    let pipeline_layout = r
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("dof2"),
            bind_group_layouts: &[layout],
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
    ];
    let buffers = [wgpu::VertexBufferLayout {
        array_stride: 24,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: if kind == 4 {
            &attributes[..1]
        } else {
            &attributes
        },
    }];
    let mut constants = vec![];
    if kind < 3 {
        constants.push(("FLAT", f64::from(kind == 2)));
        constants.push(("DOUBLE", f64::from(kind == 0)));
    }
    if kind < 4 {
        constants.push(("ENCODE", f64::from(encode)));
    }
    let options = wgpu::PipelineCompilationOptions {
        constants: &constants,
        ..Default::default()
    };
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("dof2"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs"),
                compilation_options: options.clone(),
                buffers: if kind == 3 { &[] } else { &buffers },
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: options,
                targets: &[Some(format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                // The leaves are double-sided; the depth override is front-sided.
                cull_mode: (kind != 0 && kind != 3).then_some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH,
                depth_write_enabled: kind != 3,
                depth_compare: if kind == 3 {
                    wgpu::CompareFunction::Always
                } else {
                    wgpu::CompareFunction::LessEqual
                },
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}
fn shape(r: &Renderer, g: &BufferGeometry) -> Result<Shape> {
    let read = |name: &str| -> Result<Vec<f32>> {
        match g.attributes.get(name) {
            Some(Attribute::F32(a)) => Ok(a.array().to_vec()),
            _ => Err(Error::Invalid("dof2 attribute")),
        }
    };
    let (positions, normals) = (read("position")?, read("normal")?);
    let mut data = Vec::with_capacity(positions.len() * 2);
    for (p, n) in positions.chunks(3).zip(normals.chunks(3)) {
        data.extend_from_slice(p);
        data.extend_from_slice(n);
    }
    let triangles = g.index.clone().ok_or(Error::Invalid("dof2 index"))?;
    let init = |label, contents: &[u8], usage| {
        r.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents,
                usage,
            })
    };
    Ok(Shape {
        vertices: init(
            "dof2 vertices",
            bytemuck::cast_slice(&data),
            wgpu::BufferUsages::VERTEX,
        ),
        index: init(
            "dof2 index",
            bytemuck::cast_slice(&triangles),
            wgpu::BufferUsages::INDEX,
        ),
        count: triangles.len() as u32,
        sphere: {
            let points: Vec<Vector3> = positions
                .chunks(3)
                .map(|p| Vector3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2])))
                .collect();
            let (lo, hi) = points.iter().fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(a, b), p| (a.min(*p), b.max(*p)),
            );
            let center = (lo + hi) * 0.5;
            let radius = points
                .iter()
                .map(|p| p.distance_squared(center))
                .fold(0., f64::max)
                .sqrt();
            (center, radius)
        },
        positions: positions
            .chunks(3)
            .map(|p| Vector3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2])))
            .collect(),
        triangles,
    })
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let (w, h, _) = viewport_css();
        // camera.setFocalLength( 35 ) with the initial aspect ( filmGauge 35 ).
        let fov = (2. * (0.5 * 35. / (w / h).max(1.) / 35_f64).atan()).to_degrees();
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov,
            near: NEAR,
            far: FAR,
            aspect,
            ..Default::default()
        }));
        let mut random = Random(186);
        let mut rand = || random.next();
        let mut leaves = vec![];
        for _ in 0..LEAVES {
            let rotation = [rand(), rand(), rand()];
            let spin = [rand() * 0.1, rand() * 0.1, rand() * 0.1];
            let position = Vector3::new(rand() * 150., rand() * 300., rand() * 150.);
            let drift = [rand() - 0.5, rand() - 0.5];
            leaves.push(Leaf {
                rotation,
                spin,
                position,
                drift,
            });
        }
        let mut worlds = vec![Matrix4::IDENTITY; LEAVES];
        let mut colors = vec![
            {
                let [r, g, b] = linear(f64::from(0xffffff) * 0.4);
                [r, g, b, 0.5]
            };
            LEAVES
        ];
        for i in 0..MONKEYS {
            let a = i as f64 / MONKEYS as f64 * std::f64::consts::PI;
            let position = Vector3::new(
                (a * 2.).sin() * 200.,
                (a * 3.).sin() * 20.,
                (a * 2.).cos() * 200.,
            );
            worlds.push(
                Matrix4::from_translation(position)
                    * euler(0., a * 2., 0.)
                    * Matrix4::from_scale(Vector3::splat(30.)),
            );
            colors.push([1., 1., 1., 50.]);
        }
        for _ in 0..BALLS {
            let [r, g, b] = linear(f64::from(0xffffff) * rand());
            let x = (rand() - 0.5) * 200.;
            let y = rand() * 50.;
            let z = (rand() - 0.5) * 200.;
            worlds.push(
                Matrix4::from_translation(Vector3::new(x, y, z))
                    * Matrix4::from_scale(Vector3::splat(10.)),
            );
            colors.push([r, g, b, 0.5]);
        }
        // Suzanne: BufferGeometryLoader and computeVertexNormals ( flat
        // shading ignores them ).
        #[derive(serde::Deserialize)]
        struct Array<T> {
            array: Vec<T>,
        }
        #[derive(serde::Deserialize)]
        struct Attributes {
            position: Array<f32>,
        }
        #[derive(serde::Deserialize)]
        struct Data {
            attributes: Attributes,
            index: Array<u16>,
        }
        #[derive(serde::Deserialize)]
        struct Asset {
            data: Data,
        }
        let asset: Asset = serde_json::from_slice(
            &fetch("/web/gallery/assets/suzanne_buffergeometry.json").await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let mut suzanne = BufferGeometry::default();
        suzanne.set_attribute(
            "position",
            Attribute::F32(crate::attribute::BufferAttribute::new(
                asset.data.attributes.position.array,
                3,
                false,
            )?),
        );
        suzanne.set_index(Some(
            asset.data.index.array.into_iter().map(u32::from).collect(),
        ));
        suzanne.compute_vertex_normals()?;
        let shapes = [
            shape(r, &PlaneGeometry::build(10., 10., 1, 1)?)?,
            shape(r, &suzanne)?,
            shape(r, &SphereGeometry::build(1., 20, 20)?)?,
        ];
        let env = cube(r, "Bridge2").await?;
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("dof2"),
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
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::Cube,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });
        let frame = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("dof2 frame"),
            size: 64 * 3 + 16 * 7,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let objects = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("resident draw data"),
            size: 80 * (LEAVES + MONKEYS + BALLS) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("dof2"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: frame.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: objects.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&env.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        let pipelines = [0, 1, 2, 3, 4].map(|k| scene_pipeline(r, &layout, k, HALF, false));
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty,
            count: None,
        };
        let target = wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        };
        let bokeh_layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("dof2 bokeh"),
                entries: &[
                    entry(
                        0,
                        wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                    ),
                    entry(1, target),
                    entry(2, target),
                    entry(
                        3,
                        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    ),
                ],
            });
        let bokeh = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("dof2 bokeh"),
            size: 96,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let demo = Self {
            pending: true,
            elapsed: 0.,
            fov,
            // effectController and shaderSettings in the GUI's order.
            params: [
                1., 1., 0., 2.8, 2.2, 1., 0., 0., 0., 0., 0.5, 2., 0.5, 0.7, 35., 1., 0.0001, 0.,
                3., 4.,
            ],
            pointer: Vector2::ZERO,
            focus: Vector2::ZERO,
            distance: 100.,
            rendered: false,
            leaves,
            worlds,
            colors,
            shapes,
            frame,
            objects,
            bokeh,
            group,
            layout,
            pipelines,
            bokeh_pipelines: Default::default(),
            bokeh_layout,
            targets: None,
        };
        // The heads and balls never move: their objects are written once.
        demo.write_objects(r, LEAVES..LEAVES + MONKEYS + BALLS);
        Ok(demo)
    }
    fn write_objects(&self, r: &Renderer, range: std::ops::Range<usize>) {
        let mut data: Vec<f32> = Vec::with_capacity(range.len() * 20);
        for k in range.clone() {
            data.extend(self.worlds[k].to_cols_array().iter().map(|&v| v as f32));
            data.extend_from_slice(&self.colors[k]);
        }
        r.queue.write_buffer(
            &self.objects,
            range.start as u64 * 80,
            bytemuck::cast_slice(&data),
        );
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        // The clock advances on animation frames; a seek sets it.
        if animate {
            self.elapsed += dt;
        }
        self.pending |= animate;
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    /// Raycaster.intersectObjects( scene.children ): the nearest hit along
    /// the ray, by world distance ( FrontSide meshes skip back faces ).
    fn raycast(&self, ray: Ray) -> Option<f64> {
        let mut nearest: Option<f64> = None;
        for (k, world) in self.worlds.iter().enumerate() {
            let (shape, double) = if k < LEAVES {
                (&self.shapes[0], true)
            } else if k < LEAVES + MONKEYS {
                (&self.shapes[1], false)
            } else {
                (&self.shapes[2], false)
            };
            let world = if self.rendered {
                *world
            } else {
                Matrix4::IDENTITY
            };
            let inverse = world.inverse();
            let origin = inverse.transform_point3(ray.origin);
            let direction =
                (inverse.transform_point3(ray.origin + ray.direction) - origin).normalize();
            let local = Ray { origin, direction };
            for t in shape.triangles.chunks(3) {
                let [a, b, c] = [0, 1, 2].map(|i| shape.positions[t[i] as usize]);
                if let Some(point) = local.intersect_triangle(a, b, c, !double) {
                    let distance = ray.origin.distance(world.transform_point3(point));
                    nearest = Some(nearest.map_or(distance, |n| n.min(distance)));
                }
            }
        }
        nearest
    }
    fn draw_scene(
        &self,
        pass: &mut wgpu::RenderPass,
        pipelines: [&wgpu::RenderPipeline; 4],
        depth: Option<&wgpu::RenderPipeline>,
        visible: &[bool],
    ) {
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_pipeline(pipelines[3]);
        pass.draw(0..3, 0..1);
        let ranges = [
            (0, LEAVES),
            (LEAVES, LEAVES + MONKEYS),
            (LEAVES + MONKEYS, LEAVES + MONKEYS + BALLS),
        ];
        for (k, (start, end)) in ranges.into_iter().enumerate() {
            // Leaves double-sided, heads flat, balls smooth.
            pass.set_pipeline(depth.unwrap_or(pipelines[[0, 2, 1][k]]));
            let shape = &self.shapes[k];
            pass.set_vertex_buffer(0, shape.vertices.slice(..));
            pass.set_index_buffer(shape.index.slice(..), wgpu::IndexFormat::Uint32);
            // One draw per object in view, as WebGLRenderer renders them.
            for object in (start..end).filter(|&o| visible[o]) {
                pass.draw_indexed(0..shape.count, 0, object as u32..object as u32 + 1);
            }
        }
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let (css_w, css_h, _) = viewport_css();
        let css = (css_w.round().max(1.) as u32, css_h.round().max(1.) as u32);
        if self.targets.as_ref().is_none_or(|t| {
            t.size != (out.width, out.height) || t.css != css || t.format != out.options.format
        }) {
            let color = texture(r, "dof2 color", css, HALF);
            let depth = texture(r, "dof2 depth", css, HALF);
            let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });
            let bokeh_group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("dof2 bokeh"),
                layout: &self.bokeh_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.bokeh.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&color),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&depth),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&linear),
                    },
                ],
            });
            self.targets = Some(Targets {
                size: (out.width, out.height),
                css,
                format: out.options.format,
                color,
                depth,
                scene_depth: texture(r, "dof2 scene depth", css, DEPTH),
                screen_depth: texture(r, "dof2 screen depth", (out.width, out.height), DEPTH),
                bokeh_group,
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
                direct: [0, 1, 2, 3]
                    .map(|k| scene_pipeline(r, &self.layout, k, out.options.format, true)),
            });
        }
        if !std::mem::take(&mut self.pending) {
            return Ok(true);
        }
        // render(): the camera on its orbit.
        let time = self.elapsed * 1000. * 0.00015;
        let eye = Vector3::new(
            time.cos() * 400.,
            (time / 1.4).sin() * 100.,
            time.sin() * 500.,
        );
        let world = look_at(eye, TARGET);
        {
            let node = s.get_mut(c)?;
            node.position = eye;
        }
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        let projection = perspective(self.fov, aspect);
        // The autofocus: Raycaster.setFromCamera through the pointer.
        if self.params[1] > 0.5 {
            let ndc = projection.inverse().project_point3(Vector3::new(
                self.pointer.x,
                self.pointer.y,
                0.5,
            ));
            let ray = Ray {
                origin: eye,
                direction: (world.transform_point3(ndc) - eye).normalize(),
            };
            let target = self.raycast(ray).unwrap_or(1000.);
            self.distance += (target - self.distance) * 0.03;
            let x = ((self.distance - NEAR) / (FAR - NEAR)).clamp(0., 1.);
            let sdistance = x * x * (3. - 2. * x);
            let depth = 1. - sdistance;
            self.params[3] = -FAR * NEAR / (depth * (FAR - NEAR) - FAR);
        }
        for (k, leaf) in self.leaves.iter_mut().enumerate() {
            for a in 0..3 {
                leaf.rotation[a] += leaf.spin[a];
            }
            leaf.position.y -= 2.;
            leaf.position.x += leaf.drift[0];
            leaf.position.z += leaf.drift[1];
            if leaf.position.y < 0. {
                leaf.position.y += 300.;
            }
            let [x, y, z] = leaf.rotation;
            self.worlds[k] = Matrix4::from_translation(leaf.position) * euler(x, y, z);
        }
        self.rendered = true;
        self.write_objects(r, 0..LEAVES);
        // Frustum culling by each object's world bounding sphere.
        let frustum = Frustum::from_projection(projection * world.inverse());
        let visible: Vec<bool> = self
            .worlds
            .iter()
            .enumerate()
            .map(|(k, w)| {
                let (center, radius) = self.shapes[if k < LEAVES {
                    0
                } else if k < LEAVES + MONKEYS {
                    1
                } else {
                    2
                }]
                .sphere;
                let scale = w.to_scale_rotation_translation().0.abs().max_element();
                frustum.intersects_sphere(Sphere {
                    center: w.transform_point3(center),
                    radius: radius * scale,
                })
            })
            .collect();
        let view = world.inverse();
        let mut frame: Vec<f32> = vec![];
        let mut put = |m: Matrix4| frame.extend(m.to_cols_array().iter().map(|&v| v as f32));
        put(view);
        put(projection);
        put((projection * view).inverse());
        let mut vec4 = |v: [f64; 4]| frame.extend(v.map(|x| x as f32));
        vec4([eye.x, eye.y, eye.z, 1.]);
        // DirectionalLight directions in view space ( toward the light ).
        for (position, intensity) in [
            (Vector3::new(2., 1.2, 10.), 6.),
            (Vector3::new(-2., 1.2, -10.), 3.),
        ] {
            let d = view.transform_vector3(position.normalize()).normalize();
            vec4([d.x, d.y, d.z, 0.]);
            vec4([intensity, intensity, intensity, 1.]);
        }
        let ambient = f64::from(linear(f64::from(0xcccccc))[0]);
        vec4([ambient, ambient, ambient, 1.]);
        vec4([NEAR, FAR, 0., 0.]);
        r.queue
            .write_buffer(&self.frame, 0, bytemuck::cast_slice(&frame));
        let p = self.params;
        let key = (p[18] as i32, p[19] as i32, out.options.format);
        if !self.bokeh_pipelines.contains_key(&key) {
            let pipeline = self.bokeh(r, key);
            self.bokeh_pipelines.insert(key, pipeline);
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("dof2 targets"))?;
        let mut bokeh: Vec<f32> = vec![
            t.css.0 as f32,
            t.css.1 as f32,
            p[3] as f32,
            p[14] as f32,
            p[4] as f32,
            p[5] as f32,
            p[10] as f32,
            p[11] as f32,
            p[12] as f32,
            p[13] as f32,
            NEAR as f32,
            FAR as f32,
            p[16] as f32,
        ];
        for k in [6, 7, 8, 9, 17, 2] {
            bokeh.push(f32::from_bits(u32::from(p[k] > 0.5)));
        }
        bokeh.push(0.);
        bokeh.extend([self.focus.x as f32, self.focus.y as f32]);
        bokeh.extend([t.size.0 as f32, t.size.1 as f32]);
        r.queue
            .write_buffer(&self.bokeh, 0, bytemuck::cast_slice(&bokeh));
        let pass = |encoder: &mut wgpu::CommandEncoder,
                    view: &wgpu::TextureView,
                    depth: &wgpu::TextureView| {
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("dof2"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Discard,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                })
                .forget_lifetime()
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let phong = [
            &self.pipelines[0],
            &self.pipelines[1],
            &self.pipelines[2],
            &self.pipelines[3],
        ];
        if p[0] > 0.5 {
            {
                let mut scene = pass(&mut encoder, &t.color, &t.scene_depth);
                self.draw_scene(&mut scene, phong, None, &visible);
            }
            {
                let mut depth = pass(&mut encoder, &t.depth, &t.scene_depth);
                self.draw_scene(&mut depth, phong, Some(&self.pipelines[4]), &visible);
            }
            let pipeline = &self.bokeh_pipelines[&key];
            let mut composite = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("dof2 bokeh"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            composite.set_pipeline(pipeline);
            composite.set_bind_group(0, &t.bokeh_group, &[]);
            composite.draw(0..3, 0..1);
        } else {
            let direct = [&t.direct[0], &t.direct[1], &t.direct[2], &t.direct[3]];
            let mut scene = pass(&mut encoder, &t.screen.view, &t.screen_depth);
            self.draw_scene(&mut scene, direct, None, &visible);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    /// The bokeh program for the current rings and samples ( the page
    /// recompiles it when they change ).
    fn bokeh(
        &self,
        r: &Renderer,
        (rings, samples, format): (i32, i32, wgpu::TextureFormat),
    ) -> wgpu::RenderPipeline {
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("dof2 bokeh"),
            source: wgpu::ShaderSource::Wgsl(BOKEH.into()),
        });
        let constants = [("RINGS", f64::from(rings)), ("SAMPLES", f64::from(samples))];
        let options = wgpu::PipelineCompilationOptions {
            constants: &constants,
            ..Default::default()
        };
        r.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("dof2 bokeh"),
                layout: Some(
                    &r.device
                        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                            label: Some("dof2 bokeh"),
                            bind_group_layouts: &[&self.bokeh_layout],
                            push_constant_ranges: &[],
                        }),
                ),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    compilation_options: options.clone(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs"),
                    compilation_options: options,
                    targets: &[Some(format.into())],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            })
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// onPointerMove: the ray's pointer and the shader focus coordinates.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if kind != 0 {
            return;
        }
        let (w, h, _) = viewport_css();
        self.pointer = Vector2::new((x - w / 2.) / (w / 2.), -(y - h / 2.) / (h / 2.));
        self.focus = Vector2::new(x / w, 1. - y / h);
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
    /// The GUI: enabled, jsDepthCalculation, shaderFocus, focalDepth, fstop,
    /// maxblur, showFocus, manualdof, vignetting, depthblur, threshold, gain,
    /// bias, fringe, focalLength ( camera.setFocalLength with the current
    /// aspect ), noise, dithering, pentagon, rings, samples.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("dof2 parameter"))? = f64::from(value);
        if index == 14 {
            let (w, h, _) = viewport_css();
            self.fov = (2. * (0.5 * 35. / (w / h).max(1.) / f64::from(value)).atan()).to_degrees();
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.elapsed = t;
        self.pending = true;
    }
}
