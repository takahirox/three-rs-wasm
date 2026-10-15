// Three.js r186 - Node System

// directives


// structs


// uniforms

struct cameraProjectionMatricesStruct {
	value : array< mat4x4<f32>, 4 >
};
@binding( 0 ) @group( 0 )
var<uniform> cameraProjectionMatrices : cameraProjectionMatricesStruct;

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 4 >
};
@binding( 1 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct cameraIndexStruct {
	u_cameraIndex : u32
};
@binding( 0 ) @group( 1 )
var<uniform> cameraIndex : cameraIndexStruct;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform4 : mat4x4<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar1 : vec4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_cameraIndex : u32;
var<private> v_positionView : vec3<f32>;
var<private> positionLocal : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	v_cameraIndex = cameraIndex.u_cameraIndex;
	modelViewMatrix = ( cameraViewMatrices.value[ v_cameraIndex ] * object.nodeUniform4 );
	positionLocal = position;
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar1 = ( cameraProjectionMatrices.value[ v_cameraIndex ] * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar1;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
