// Three.js r187 - Node System

// directives


// structs


// uniforms

struct nodeUniform3Struct {
	value : array< vec4<f32> >
};
@binding( 11 ) @group( 1 )
var<storage, read> nodeUniform3 : nodeUniform3Struct;

struct nodeUniform4Struct {
	value : array< u32 >
};
@binding( 12 ) @group( 1 )
var<storage, read> nodeUniform4 : nodeUniform4Struct;

struct nodeUniform5Struct {
	value : array< vec4<u32> >
};
@binding( 13 ) @group( 1 )
var<storage, read> nodeUniform5 : nodeUniform5Struct;

struct nodeUniform14Struct {
	value : array< mat4x4<f32> >
};
@binding( 14 ) @group( 1 )
var<storage, read> nodeUniform14 : nodeUniform14Struct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform6 : u32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform15 : u32,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : f32,
	nodeUniform20 : mat3x3<f32>,
	nodeUniform21 : vec2<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform30 : f32,
	nodeUniform28 : f32,
	nodeUniform23 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) nodeVarying5 : vec2<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_tangentView : vec3<f32>,
	@location( 3 ) v_bitangentView : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) NORMAL_v_bitangentView : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> tangentLocal : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> VERTEX_normalView : vec3<f32>;
var<private> VERTEX_tangentView : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> vMeshletInstance : u32;
var<private> positionLocal : vec3<f32>;
var<private> nodeVarying4 : vec3<f32>;
var<private> nodeVarying7 : vec3<f32>;
var<private> nodeVarying9 : vec4<f32>;
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

	let nodeConst0 = vec2<f32>( nodeUniform3.value[ ( nodeUniform4.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 0u ) ].w, nodeUniform3.value[ ( nodeUniform4.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 1u ) ].w );
	varyings.nodeVarying5 = nodeConst0;
	vMeshletInstance = nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].x;
	nodeVarying7 = nodeUniform3.value[ ( nodeUniform4.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 1u ) ].xyz;
	normalLocal = nodeVarying7;
	let nodeConst1 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( transpose( tsl_inverse_mat3( mat3x3<f32>( nodeUniform14.value[ ( object.nodeUniform15 + vMeshletInstance ) ][ 0 ].xyz, nodeUniform14.value[ ( object.nodeUniform15 + vMeshletInstance ) ][ 1 ].xyz, nodeUniform14.value[ ( object.nodeUniform15 + vMeshletInstance ) ][ 2 ].xyz ) ) ) * normalLocal ), 0.0 ) ).xyz );
	varyings.v_normalViewGeometry = nodeConst1;
	modelViewMatrix = ( render.cameraViewMatrix * nodeUniform14.value[ ( object.nodeUniform15 + vMeshletInstance ) ] );
	nodeVarying9 = nodeUniform3.value[ ( nodeUniform4.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 2u ) ];
	tangentLocal = nodeVarying9.xyz;
	varyings.v_tangentView = ( modelViewMatrix * vec4<f32>( tangentLocal, 0.0 ) ).xyz;
	normalViewGeometry = normalize( varyings.v_normalViewGeometry );
	VERTEX_normalView = normalViewGeometry;
	VERTEX_tangentView = normalize( varyings.v_tangentView );
	varyings.NORMAL_v_bitangentView = ( cross( VERTEX_normalView, VERTEX_tangentView ) * vec3<f32>( nodeVarying9.w ) );
	nodeVarying4 = nodeUniform3.value[ ( nodeUniform4.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 0u ) ].xyz;
	positionLocal = nodeVarying4;
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	varyings.v_positionWorld = ( nodeUniform14.value[ ( object.nodeUniform15 + vMeshletInstance ) ] * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let VERTEX_nodeConst31 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst31;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
