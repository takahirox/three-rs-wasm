// Three.js r186 - Node System

// directives

// structs

// uniforms

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionViewDirection : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> modelViewMatrix : mat4x4<f32>;
var<private> normalLocal : vec3<f32>;
var<private> VERTEX_nodeVar8 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes

@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) normal : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	positionLocal = positionLocal;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform2 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform3 * normalLocal ), 0.0 ) ).xyz );
	VERTEX_nodeVar8 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar8;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
