// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform9 : vec2<f32>,
	nodeUniform5 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform8 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) nodeVarying2 : vec2<f32>,
	@location( 1 ) nodeVarying3 : f32,
	@location( 2 ) v_clipSpace : vec4<f32>,
	@location( 3 ) nodeVarying5 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> v_positionWorld : vec3<f32>;
var<private> positionLocal : vec3<f32>;

// codes


@vertex
fn main( @location( 0 ) uv : vec2<f32>,
	@location( 1 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.nodeVarying5 = uv;
	varyings.nodeVarying2 = vec2<f32>( 0.0, 0.0 );
	varyings.nodeVarying3 = 0.0;
	positionLocal = position;
	v_positionWorld = ( object.nodeUniform8 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	nodeVar9 = ( ( render.cameraProjectionMatrix * render.cameraViewMatrix ) * vec4<f32>( v_positionWorld, 1.0 ) );
	varyings.nodeVarying2 = ( uv * vec2<f32>( nodeVar9.w ) );
	varyings.nodeVarying3 = nodeVar9.w;
	nodeVar10 = vec4<f32>( ( ( round( ( ( nodeVar9.xy / vec2<f32>( ( nodeVar9.w * 2.0 ) ) ) * render.nodeUniform9 ) ) / render.nodeUniform9 ) * vec2<f32>( ( nodeVar9.w * 2.0 ) ) ), nodeVar9.zw );

	// result

	varyings.builtinClipSpace = nodeVar10;

	return varyings;

}
