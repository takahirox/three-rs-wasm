// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	cameraProjectionMatrixInverse : mat4x4<f32>,
	nodeUniform5 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_clipSpace : vec4<f32>,
	@location( 1 ) nodeVarying2 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec2<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> positionLocal : vec3<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) uv : vec2<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	nodeVar0 = ( render.nodeUniform5.x / render.nodeUniform5.y );
	nodeVar1 = vec2<f32>( ( positionLocal.x * ( 0.4 / nodeVar0 ) ), ( positionLocal.y * 0.4 ) );
	nodeVar2 = vec2<f32>( ( nodeVar1.x + ( ( ( -1.0 + 0.05 ) + ( ( 0.4 / 2.0 ) / nodeVar0 ) ) + ( 0.0 * ( ( 0.4 / nodeVar0 ) + 0.01 ) ) ) ), ( nodeVar1.y + ( ( ( 1.0 - 0.05 ) - ( 0.4 / 2.0 ) ) - ( 1.0 * ( 0.4 + 0.01 ) ) ) ) );
	nodeVar3 = vec4<f32>( nodeVar2.x, nodeVar2.y, 0.0, 1.0 );
	varyings.v_clipSpace = nodeVar3;
	varyings.nodeVarying2 = uv;

	// result

	varyings.builtinClipSpace = nodeVar3;

	return varyings;

}
