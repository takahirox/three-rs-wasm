// Three.js r186 - Node System

// directives


// structs


// uniforms
@binding( 3 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;

struct renderStruct {
	nodeUniform1 : f32,
	nodeUniform3 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform5 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform8 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) nodeVarying4 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar17 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes
fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn tsl_mod_vec2( x : vec2f, y : vec2f ) -> vec2f { return x - y * floor( x / y ); }


@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) uv : vec2<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	nodeVar0 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, vec2<f32>( 0.5, tsl_mod_float( ( ( uv.y * 0.2 ) - ( render.nodeUniform1 * 0.005 ) ), 1.0 ) ), 0 );
	nodeVar1 = ( nodeVar0.x * 10.0 );
	nodeVar2 = cos( nodeVar1 );
	nodeVar3 = sin( nodeVar1 );
	nodeVar4 = ( ( mat2x2<f32>( nodeVar2, nodeVar3, ( - nodeVar3 ), nodeVar2 ) * ( positionLocal.xz - vec2<f32>( 0.0, 0.0 ) ) ) + vec2<f32>( 0.0, 0.0 ) );
	positionLocal.x = nodeVar4[ 0 ];
	positionLocal.z = nodeVar4[ 1 ];
	nodeVar5 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, tsl_mod_vec2( vec2<f32>( 0.25, ( render.nodeUniform1 * 0.01 ) ), vec2<f32>( 1.0 ) ), 0 );
	nodeVar6 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, tsl_mod_vec2( vec2<f32>( 0.75, ( render.nodeUniform1 * 0.01 ) ), vec2<f32>( 1.0 ) ), 0 );
	positionLocal = ( positionLocal + vec3<f32>( ( vec2<f32>( ( nodeVar5.x - 0.5 ), ( nodeVar6.x - 0.5 ) ) * vec2<f32>( ( pow( uv.y, 2.0 ) * 10.0 ) ) ), 0.0 ) );
	positionLocal = positionLocal;
	varyings.nodeVarying4 = uv;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform8 );
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar17 = ( render.cameraProjectionMatrix * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar17;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
