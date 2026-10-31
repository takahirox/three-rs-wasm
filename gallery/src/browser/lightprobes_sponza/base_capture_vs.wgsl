// Three.js r187 - Node System

// directives


// structs


// uniforms

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 2 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct cameraProjectionMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 3 ) @group( 0 )
var<uniform> cameraProjectionMatrices : cameraProjectionMatricesStruct;

struct cameraIndexStruct {
	u_cameraIndex : u32
};
@binding( 0 ) @group( 1 )
var<uniform> cameraIndex : cameraIndexStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : vec3<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) @interpolate(flat, either) v_cameraIndex : u32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> positionLocal : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) uv : vec2<f32>,
	@location( 1 ) normal : vec3<f32>,
	@location( 2 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.nodeVarying8 = uv;
	varyings.v_cameraIndex = cameraIndex.u_cameraIndex;
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( cameraViewMatrices.value[ varyings.v_cameraIndex ] * vec4<f32>( ( object.nodeUniform8 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( cameraViewMatrices.value[ varyings.v_cameraIndex ] * object.nodeUniform12 );
	positionLocal = position;
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	varyings.v_positionWorld = ( object.nodeUniform12 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let VERTEX_nodeConst28 = ( cameraProjectionMatrices.value[ varyings.v_cameraIndex ] * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst28;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
