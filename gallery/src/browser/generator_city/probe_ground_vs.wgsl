// Three.js r187 - Node System

// directives


// structs


// uniforms

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 4 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct cameraProjectionMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 5 ) @group( 0 )
var<uniform> cameraProjectionMatrices : cameraProjectionMatricesStruct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform28 : mat4x4<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : f32
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

struct cameraIndexStruct {
	u_cameraIndex : u32
};
@binding( 0 ) @group( 1 )
var<uniform> cameraIndex : cameraIndexStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) @interpolate(flat, either) v_cameraIndex : u32,
	@location( 1 ) v_positionView : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) normal : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	varyings.v_positionWorld = ( object.nodeUniform0 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_cameraIndex = cameraIndex.u_cameraIndex;
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( cameraViewMatrices.value[ varyings.v_cameraIndex ] * vec4<f32>( ( object.nodeUniform6 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( cameraViewMatrices.value[ varyings.v_cameraIndex ] * object.nodeUniform0 );
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - varyings.v_positionView );
	let VERTEX_nodeConst79 = ( cameraProjectionMatrices.value[ varyings.v_cameraIndex ] * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst79;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
