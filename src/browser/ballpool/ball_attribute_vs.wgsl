// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform38 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform22 : vec3<f32>,
	nodeUniform24 : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform33 : f32,
	nodeUniform28 : f32,
	nodeUniform27 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform14 : mat3x3<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : f32,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform36 : mat4x4<f32>,
	nodeUniform37 : mat4x4<f32>,
	nodeUniform39 : mat4x4<f32>,
	nodeUniform40 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) positionPrevious : vec3<f32>,
	@location( 2 ) v_positionView : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) vInstanceColor : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : mat4x4<f32>;
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar167 : vec4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
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
	@location( 5 ) nodeAttribute3 : vec4<f32>,
	@location( 6 ) nodeAttribute4 : vec4<f32>,
	@location( 7 ) nodeAttribute5 : vec4<f32>,
	@location( 8 ) nodeAttribute6 : vec4<f32>,
	@location( 9 ) nodeAttribute7 : vec4<f32>,
	@location( 10 ) nodeAttribute8 : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.positionLocal = position;
	nodeVar0 = mat4x4<f32>( nodeAttribute0, nodeAttribute1, nodeAttribute2, nodeAttribute3 );
	varyings.positionLocal = ( nodeVar0 * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	varyings.positionPrevious = position;
	varyings.positionPrevious = ( mat4x4<f32>( nodeAttribute4, nodeAttribute5, nodeAttribute6, nodeAttribute7 ) * vec4<f32>( varyings.positionPrevious, 1.0 ) ).xyz;
	normalLocal = normal;
	normalLocal = normalize( ( transpose( tsl_inverse_mat3( mat3x3<f32>( nodeVar0[ 0 ].xyz, nodeVar0[ 1 ].xyz, nodeVar0[ 2 ].xyz ) ) ) * normalLocal ) );
	varyings.vInstanceColor = nodeAttribute8;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform14 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform21 );
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - varyings.v_positionView );
	varyings.v_positionWorld = ( object.nodeUniform21 * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar167 = ( render.cameraProjectionMatrix * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar167;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
