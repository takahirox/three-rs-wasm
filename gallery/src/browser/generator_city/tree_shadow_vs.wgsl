// Three.js r186 - Node System

// directives


// structs


// uniforms

struct NodeBuffer_41854Struct {
	value : array< mat4x4<f32>, 64 >
};
@binding( 1 ) @group( 1 )
var<uniform> NodeBuffer_41854 : NodeBuffer_41854Struct;

struct renderStruct {
	nodeUniform1 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : mat4x4<f32>,
	nodeUniform5 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) v_normalWorldGeometry : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) @interpolate(flat, either) nodeVarying8 : u32,
	@location( 4 ) nodeVarying9 : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar6 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> v_normalViewGeometry : vec3<f32>;
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
	@location( 2 ) partId : f32 ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	positionLocal = ( NodeBuffer_41854.value[ instanceIndex ] * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	normalLocal = normal;
	normalLocal = normalize( ( transpose( tsl_inverse_mat3( mat3x3<f32>( NodeBuffer_41854.value[ instanceIndex ][ 0 ].xyz, NodeBuffer_41854.value[ instanceIndex ][ 1 ].xyz, NodeBuffer_41854.value[ instanceIndex ][ 2 ].xyz ) ) ) * normalLocal ) );

	if ( ( partId == 1.0 ) ) {

		nodeVar1 = ( f32( instanceIndex ) * 1.7 );
		nodeVar2 = ( sin( ( ( ( render.nodeUniform1 * 3.1 ) + ( position.y * 2.4 ) ) + ( position.x * 1.9 ) ) ) * 0.015 );
		nodeVar0 = ( ( vec3<f32>( ( sin( ( ( render.nodeUniform1 * 0.8 ) + nodeVar1 ) ) * 0.05 ), 0.0, ( sin( ( ( render.nodeUniform1 * 0.61 ) + ( nodeVar1 * 1.3 ) ) ) * 0.04 ) ) + vec3<f32>( nodeVar2, ( - nodeVar2 ), 0.0 ) ) * vec3<f32>( smoothstep( 2.0, 5.0, position.y ) ) );

	} else {

		nodeVar0 = vec3<f32>( 0.0, 0.0, 0.0 );

	}

	positionLocal = ( positionLocal + nodeVar0 );
	varyings.nodeVarying3 = partId;
	v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform3 * normalLocal ), 0.0 ) ).xyz );
	normalViewGeometry = normalize( v_normalViewGeometry );
	varyings.v_normalWorldGeometry = normalize( ( vec4<f32>( normalViewGeometry, 0.0 ) * render.cameraViewMatrix ).xyz );
	varyings.v_positionWorld = ( object.nodeUniform4 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.nodeVarying8 = instanceIndex;
	varyings.nodeVarying9 = position;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform4 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar6 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar6;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
