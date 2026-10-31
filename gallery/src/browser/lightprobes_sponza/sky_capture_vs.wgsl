// Three.js r187 - Node System

// directives


// structs


// uniforms

struct cameraProjectionMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 2 ) @group( 0 )
var<uniform> cameraProjectionMatrices : cameraProjectionMatricesStruct;

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 3 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : u32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform15 : f32
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
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) nodeVarying6 : f32,
	@location( 3 ) nodeVarying7 : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32>,
	@location( 5 ) nodeVarying9 : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> modelViewMatrix : mat4x4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> positionLocal : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	varyings.v_positionWorld = ( object.nodeUniform0 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_cameraIndex = cameraIndex.u_cameraIndex;
	let nodeConst12 = normalize( object.nodeUniform12 );
	varyings.nodeVarying8 = nodeConst12;
	varyings.nodeVarying6 = ( 1000.0 * max( 0.0, ( 1.0 - pow( 2.718281828459045, ( - ( ( 1.6110731556870734 - acos( clamp( nodeConst12.y, -1.0, 1.0 ) ) ) / 1.5 ) ) ) ) ) );
	varyings.nodeVarying7 = ( vec3<f32>( 0.000005804542996261093, 0.000013562911419845635, 0.000030265902468824876 ) * vec3<f32>( ( object.nodeUniform13 - ( 1.0 * ( 1.0 - ( 1.0 - clamp( ( 1.0 - exp( ( object.nodeUniform12.y / 450000.0 ) ) ), 0.0, 1.0 ) ) ) ) ) ) );
	varyings.nodeVarying9 = ( ( vec3<f32>( ( 0.434 * ( ( 0.2 * object.nodeUniform14 ) * 1e-17 ) ) ) * vec3<f32>( 183999185144339.78, 277980239196605.28, 407904795438610.94 ) ) * vec3<f32>( object.nodeUniform15 ) );
	modelViewMatrix = ( cameraViewMatrices.value[ varyings.v_cameraIndex ] * object.nodeUniform0 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let VERTEX_nodeConst13 = ( cameraProjectionMatrices.value[ varyings.v_cameraIndex ] * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst13;
	VERTEX_v_modelViewProjection.z = VERTEX_v_modelViewProjection.w;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
