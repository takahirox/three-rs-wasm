// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< vec4<f32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform1Struct {
	value : array< mat4x4<f32> >
};
@binding( 1 ) @group( 0 )
var<storage, read> nodeUniform1 : nodeUniform1Struct;

struct nodeUniform3Struct {
	value : array< mat4x4<f32> >
};
@binding( 3 ) @group( 0 )
var<storage, read> nodeUniform3 : nodeUniform3Struct;

struct nodeUniform4Struct {
	value : array< vec4<u32> >
};
@binding( 4 ) @group( 0 )
var<storage, read> nodeUniform4 : nodeUniform4Struct;

struct nodeUniform5Struct {
	value : array< vec4<f32> >
};
@binding( 5 ) @group( 0 )
var<storage, read> nodeUniform5 : nodeUniform5Struct;

struct nodeUniform7Struct {
	value : array< vec4<f32> >
};
@binding( 6 ) @group( 0 )
var<storage, read> nodeUniform7 : nodeUniform7Struct;

struct nodeUniform8Struct {
	value : array< f32 >
};
@binding( 7 ) @group( 0 )
var<storage, read> nodeUniform8 : nodeUniform8Struct;

struct objectStruct {
	nodeUniform2 : mat4x4<f32>,
	nodeUniform6 : mat4x4<f32>,
	nodeUniform9 : u32
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars


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



@compute @workgroup_size( 64, 1, 1 )
fn main( @builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32> ) {

	// local vars
	

	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 64 * numWorkgroups.x )
		+ globalId.z * ( 64 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code


	// flow -> Compute Instanced Skinning
	if ( instanceIndex >= object.nodeUniform9 ) { return; }

	let nodeConst0 = ( instanceIndex * 2u );
	let nodeConst1 = ( instanceIndex / 16340u );
	let nodeConst2 = ( nodeConst1 * 65u );
	let nodeConst3 = ( instanceIndex % 16340u );
	let nodeConst4 = ( nodeConst3 * 3u );
	let nodeConst5 = ( object.nodeUniform6 * vec4<f32>( ( nodeUniform7.value[ nodeConst4 ].xyz + ( nodeUniform7.value[ ( nodeConst4 + 2u ) ].xyz * vec3<f32>( nodeUniform8.value[ nodeConst1 ] ) ) ), 1.0 ) );
	nodeUniform0.value[ nodeConst0 ] = vec4<f32>( ( nodeUniform1.value[ nodeConst1 ] * vec4<f32>( ( object.nodeUniform2 * ( ( ( ( ( nodeUniform5.value[ nodeConst3 ].x * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].x ) ] ) * nodeConst5 ) + ( ( nodeUniform5.value[ nodeConst3 ].y * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].y ) ] ) * nodeConst5 ) ) + ( ( nodeUniform5.value[ nodeConst3 ].z * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].z ) ] ) * nodeConst5 ) ) + ( ( nodeUniform5.value[ nodeConst3 ].w * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].w ) ] ) * nodeConst5 ) ) ).xyz, 1.0 ) ).xyz, 1.0 );
	nodeUniform0.value[ ( nodeConst0 + 1u ) ] = vec4<f32>( normalize( ( transpose( tsl_inverse_mat3( mat3x3<f32>( nodeUniform1.value[ nodeConst1 ][ 0 ].xyz, nodeUniform1.value[ nodeConst1 ][ 1 ].xyz, nodeUniform1.value[ nodeConst1 ][ 2 ].xyz ) ) ) * ( mat3x3<f32>( ( ( object.nodeUniform2 * ( ( ( nodeUniform5.value[ nodeConst3 ].x * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].x ) ] + nodeUniform5.value[ nodeConst3 ].y * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].y ) ] ) + nodeUniform5.value[ nodeConst3 ].z * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].z ) ] ) + nodeUniform5.value[ nodeConst3 ].w * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].w ) ] ) ) * object.nodeUniform6 )[ 0 ].xyz, ( ( object.nodeUniform2 * ( ( ( nodeUniform5.value[ nodeConst3 ].x * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].x ) ] + nodeUniform5.value[ nodeConst3 ].y * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].y ) ] ) + nodeUniform5.value[ nodeConst3 ].z * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].z ) ] ) + nodeUniform5.value[ nodeConst3 ].w * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].w ) ] ) ) * object.nodeUniform6 )[ 1 ].xyz, ( ( object.nodeUniform2 * ( ( ( nodeUniform5.value[ nodeConst3 ].x * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].x ) ] + nodeUniform5.value[ nodeConst3 ].y * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].y ) ] ) + nodeUniform5.value[ nodeConst3 ].z * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].z ) ] ) + nodeUniform5.value[ nodeConst3 ].w * nodeUniform3.value[ ( nodeConst2 + nodeUniform4.value[ nodeConst3 ].w ) ] ) ) * object.nodeUniform6 )[ 2 ].xyz ) * nodeUniform7.value[ ( nodeConst4 + 1u ) ].xyz ) ) ), 0.0 );

	

}
