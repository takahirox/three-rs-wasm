// Three.js r187 - Node System

// directives


// structs


// uniforms

struct nodeUniform3Struct {
	value : array< vec4<f32> >
};
@binding( 9 ) @group( 1 )
var<storage, read> nodeUniform3 : nodeUniform3Struct;

struct nodeUniform4Struct {
	value : array< u32 >
};
@binding( 10 ) @group( 1 )
var<storage, read> nodeUniform4 : nodeUniform4Struct;

struct nodeUniform5Struct {
	value : array< vec4<u32> >
};
@binding( 11 ) @group( 1 )
var<storage, read> nodeUniform5 : nodeUniform5Struct;

struct nodeUniform11Struct {
	value : array< mat4x4<f32> >
};
@binding( 12 ) @group( 1 )
var<storage, read> nodeUniform11 : nodeUniform11Struct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform6 : u32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform12 : u32,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform30 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	nodeUniform32 : vec3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform35 : f32,
	nodeUniform33 : f32,
	nodeUniform17 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : vec2<f32>,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform20 : vec4<f32>,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform26 : vec4<f32>,
	nodeUniform19 : f32,
	nodeUniform22 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) nodeVarying5 : vec2<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> vMeshletInstance : u32;
var<private> positionLocal : vec3<f32>;
var<private> nodeVarying4 : vec3<f32>;
var<private> nodeVarying7 : vec3<f32>;
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
	@builtin( vertex_index ) vertexIndex : u32 ) -> VaryingsStruct {

	// flow
	// code

	varyings.nodeVarying5 = vec2<f32>( nodeUniform3.value[ ( nodeUniform4.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 0u ) ].w, nodeUniform3.value[ ( nodeUniform4.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 1u ) ].w );
	vMeshletInstance = nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].x;
	nodeVarying7 = nodeUniform3.value[ ( nodeUniform4.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 1u ) ].xyz;
	normalLocal = nodeVarying7;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( transpose( tsl_inverse_mat3( mat3x3<f32>( nodeUniform11.value[ ( object.nodeUniform12 + vMeshletInstance ) ][ 0 ].xyz, nodeUniform11.value[ ( object.nodeUniform12 + vMeshletInstance ) ][ 1 ].xyz, nodeUniform11.value[ ( object.nodeUniform12 + vMeshletInstance ) ][ 2 ].xyz ) ) ) * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * nodeUniform11.value[ ( object.nodeUniform12 + vMeshletInstance ) ] );
	nodeVarying4 = nodeUniform3.value[ ( nodeUniform4.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 0u ) ].xyz;
	positionLocal = nodeVarying4;
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - varyings.v_positionView );
	varyings.v_positionWorld = ( nodeUniform11.value[ ( object.nodeUniform12 + vMeshletInstance ) ] * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let VERTEX_nodeConst48 = ( render.cameraProjectionMatrix * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst48;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
