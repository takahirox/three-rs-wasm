// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_7764Struct {
	value : array< vec4<f32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_7764 : NodeBuffer_7764Struct;

struct NodeBuffer_7757Struct {
	value : array< mat4x4<f32> >
};
@binding( 1 ) @group( 0 )
var<storage, read> NodeBuffer_7757 : NodeBuffer_7757Struct;

struct NodeBuffer_7758Struct {
	value : array< mat4x4<f32> >
};
@binding( 3 ) @group( 0 )
var<storage, read> NodeBuffer_7758 : NodeBuffer_7758Struct;

struct NodeBuffer_7760Struct {
	value : array< vec4<u32> >
};
@binding( 4 ) @group( 0 )
var<storage, read> NodeBuffer_7760 : NodeBuffer_7760Struct;

struct NodeBuffer_7761Struct {
	value : array< vec4<f32> >
};
@binding( 5 ) @group( 0 )
var<storage, read> NodeBuffer_7761 : NodeBuffer_7761Struct;

struct NodeBuffer_7759Struct {
	value : array< vec4<f32> >
};
@binding( 6 ) @group( 0 )
var<storage, read> NodeBuffer_7759 : NodeBuffer_7759Struct;

struct NodeBuffer_7756Struct {
	value : array< f32 >
};
@binding( 7 ) @group( 0 )
var<storage, read> NodeBuffer_7756 : NodeBuffer_7756Struct;

struct objectStruct {
	nodeUniform2 : mat4x4<f32>,
	nodeUniform6 : mat4x4<f32>,
	nodeUniform9 : u32
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : u32;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : u32;
var<private> nodeVar3 : u32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : vec4<f32>;

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

	nodeVar0 = ( instanceIndex * 2u );
	nodeVar1 = ( instanceIndex / 16340u );
	nodeVar2 = ( nodeVar1 * 65u );
	nodeVar3 = ( instanceIndex % 16340u );
	nodeVar4 = ( nodeVar3 * 3u );
	nodeVar5 = ( object.nodeUniform6 * vec4<f32>( ( NodeBuffer_7759.value[ nodeVar4 ].xyz + ( NodeBuffer_7759.value[ ( nodeVar4 + 2u ) ].xyz * vec3<f32>( NodeBuffer_7756.value[ nodeVar1 ] ) ) ), 1.0 ) );
	NodeBuffer_7764.value[ nodeVar0 ] = vec4<f32>( ( NodeBuffer_7757.value[ nodeVar1 ] * vec4<f32>( ( object.nodeUniform2 * ( ( ( ( ( NodeBuffer_7761.value[ nodeVar3 ].x * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].x ) ] ) * nodeVar5 ) + ( ( NodeBuffer_7761.value[ nodeVar3 ].y * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].y ) ] ) * nodeVar5 ) ) + ( ( NodeBuffer_7761.value[ nodeVar3 ].z * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].z ) ] ) * nodeVar5 ) ) + ( ( NodeBuffer_7761.value[ nodeVar3 ].w * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].w ) ] ) * nodeVar5 ) ) ).xyz, 1.0 ) ).xyz, 1.0 );
	NodeBuffer_7764.value[ ( nodeVar0 + 1u ) ] = vec4<f32>( normalize( ( transpose( tsl_inverse_mat3( mat3x3<f32>( NodeBuffer_7757.value[ nodeVar1 ][ 0 ].xyz, NodeBuffer_7757.value[ nodeVar1 ][ 1 ].xyz, NodeBuffer_7757.value[ nodeVar1 ][ 2 ].xyz ) ) ) * ( mat3x3<f32>( ( ( object.nodeUniform2 * ( ( ( NodeBuffer_7761.value[ nodeVar3 ].x * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].x ) ] + NodeBuffer_7761.value[ nodeVar3 ].y * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].y ) ] ) + NodeBuffer_7761.value[ nodeVar3 ].z * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].z ) ] ) + NodeBuffer_7761.value[ nodeVar3 ].w * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].w ) ] ) ) * object.nodeUniform6 )[ 0 ].xyz, ( ( object.nodeUniform2 * ( ( ( NodeBuffer_7761.value[ nodeVar3 ].x * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].x ) ] + NodeBuffer_7761.value[ nodeVar3 ].y * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].y ) ] ) + NodeBuffer_7761.value[ nodeVar3 ].z * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].z ) ] ) + NodeBuffer_7761.value[ nodeVar3 ].w * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].w ) ] ) ) * object.nodeUniform6 )[ 1 ].xyz, ( ( object.nodeUniform2 * ( ( ( NodeBuffer_7761.value[ nodeVar3 ].x * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].x ) ] + NodeBuffer_7761.value[ nodeVar3 ].y * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].y ) ] ) + NodeBuffer_7761.value[ nodeVar3 ].z * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].z ) ] ) + NodeBuffer_7761.value[ nodeVar3 ].w * NodeBuffer_7758.value[ ( nodeVar2 + NodeBuffer_7760.value[ nodeVar3 ].w ) ] ) ) * object.nodeUniform6 )[ 2 ].xyz ) * NodeBuffer_7759.value[ ( nodeVar4 + 1u ) ].xyz ) ) ), 0.0 );

	

}
