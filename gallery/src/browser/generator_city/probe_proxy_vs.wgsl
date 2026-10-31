// Three.js r187 - Node System

// directives


// structs


// uniforms

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 3 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct cameraProjectionMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 4 ) @group( 0 )
var<uniform> cameraProjectionMatrices : cameraProjectionMatricesStruct;

struct objectStruct {
	nodeUniform4 : mat4x4<f32>,
	nodeUniform5 : u32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : f32,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32
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
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes

fn tsl_inverse_mat3( m : mat3x3<f32> ) -> mat3x3<f32> {

	let a00 = m[ 0 ][ 0 ]; let a01 = m[ 0 ][ 1 ]; let a02 = m[ 0 ][ 2 ];
	let a10 = m[ 1 ][ 0 ]; let a11 = m[ 1 ][ 1 ]; let a12 = m[ 1 ][ 2 ];
	let a20 = m[ 2 ][ 0 ]; let a21 = m[ 2 ][ 1 ]; let a22 = m[ 2 ][ 2 ];

	let b01 = a22 * a11 - a12 * a21;
	let b11 = - a22 * a10 + a12 * a20;
	let b21 = a21 * a10 - a11 * a20;

	let det = a00 * b01 + a01 * b11 + a02 * b21;

	return mat3x3<f32>(
		b01, ( - a22 * a01 + a02 * a21 ), ( a12 * a01 - a02 * a11 ),
		b11, ( a22 * a00 - a02 * a20 ), ( - a12 * a00 + a02 * a10 ),
		b21, ( - a21 * a00 + a01 * a20 ), ( a11 * a00 - a01 * a10 )
	) * ( 1.0 / det );

}



@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) normal : vec3<f32>,
	@location( 2 ) nodeAttribute0 : vec4<f32>,
	@location( 3 ) nodeAttribute1 : vec4<f32>,
	@location( 4 ) nodeAttribute2 : vec4<f32>,
	@location( 5 ) nodeAttribute3 : vec4<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	let nodeConst0 = mat4x4<f32>( nodeAttribute0, nodeAttribute1, nodeAttribute2, nodeAttribute3 );
	positionLocal = ( nodeConst0 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	normalLocal = normal;
	normalLocal = normalize( ( transpose( tsl_inverse_mat3( mat3x3<f32>( nodeConst0[ 0 ].xyz, nodeConst0[ 1 ].xyz, nodeConst0[ 2 ].xyz ) ) ) * normalLocal ) );
	varyings.v_positionWorld = ( object.nodeUniform4 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_cameraIndex = cameraIndex.u_cameraIndex;
	varyings.v_normalViewGeometry = normalize( ( cameraViewMatrices.value[ varyings.v_cameraIndex ] * vec4<f32>( ( object.nodeUniform11 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( cameraViewMatrices.value[ varyings.v_cameraIndex ] * object.nodeUniform4 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	let VERTEX_nodeConst49 = ( cameraProjectionMatrices.value[ varyings.v_cameraIndex ] * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst49;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
