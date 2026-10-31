// Three.js r187 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform7 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform12 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) @interpolate(flat, either) nodeVarying4 : u32,
	@location( 1 ) nodeVarying5 : vec4<f32>,
	@location( 2 ) nodeVarying6 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec4<f32>;
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
fn main( @builtin( instance_index ) instanceIndex : u32,
	@location( 0 ) position : vec3<f32>,
	@location( 1 ) normal : vec3<f32>,
	@location( 2 ) uv : vec2<f32>,
	@location( 3 ) nodeAttribute0 : vec4<f32>,
	@location( 4 ) nodeAttribute1 : vec4<f32>,
	@location( 5 ) nodeAttribute2 : vec4<f32>,
	@location( 6 ) nodeAttribute3 : vec4<f32>,
	@location( 7 ) nodeAttribute4 : vec4<f32>,
	@location( 8 ) nodeAttribute11 : vec4<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	let nodeConst0 = mat4x4<f32>( nodeAttribute0, nodeAttribute1, nodeAttribute2, nodeAttribute3 );
	positionLocal = ( nodeConst0 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	normalLocal = normal;
	normalLocal = normalize( ( transpose( tsl_inverse_mat3( mat3x3<f32>( nodeConst0[ 0 ].xyz, nodeConst0[ 1 ].xyz, nodeConst0[ 2 ].xyz ) ) ) * normalLocal ) );
	positionLocal = nodeAttribute4.xyz;
	varyings.nodeVarying4 = instanceIndex;
	varyings.nodeVarying5 = nodeAttribute4;
	varyings.nodeVarying6 = uv;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform10 );
	let nodeConst4 = ( modelViewMatrix * vec4<f32>( nodeAttribute4.xyz, 1.0 ) );
	let nodeConst5 = atan2( nodeAttribute11.y, nodeAttribute11.x );
	let nodeConst6 = cos( nodeConst5 );
	let nodeConst7 = sin( nodeConst5 );
	v_positionView = vec4<f32>( ( nodeConst4.xy + ( mat2x2<f32>( nodeConst6, nodeConst7, ( - nodeConst7 ), nodeConst6 ) * ( position.xy * ( vec2<f32>( length( object.nodeUniform10[ 0u ].xyz ), length( object.nodeUniform10[ 1u ].xyz ) ) * vec2<f32>( object.nodeUniform12 ) ) ) ) ), nodeConst4.zw );
	let VERTEX_nodeConst8 = ( render.cameraProjectionMatrix * v_positionView );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst8;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
