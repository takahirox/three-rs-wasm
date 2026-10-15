// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1002Struct {
	value : array< vec4<f32>, 16 >
};
@binding( 0 ) @group( 0 )
var<uniform> NodeBuffer_1002 : NodeBuffer_1002Struct;

struct NodeBuffer_1008Struct {
	value : array< f32 >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> NodeBuffer_1008 : NodeBuffer_1008Struct;

struct objectStruct {
	nodeUniform2 : u32
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : u32;
var<private> nodeVar3 : u32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : u32;

// codes


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


	// flow -> HZB Level 1
	if ( instanceIndex >= object.nodeUniform2 ) { return; }


	if ( ( instanceIndex < ( u32( NodeBuffer_1002.value[ 1u ].y ) * u32( NodeBuffer_1002.value[ 1u ].z ) ) ) ) {

		nodeVar0 = 0.0;
		nodeVar1 = ( instanceIndex / u32( NodeBuffer_1002.value[ 1u ].y ) );
		nodeVar2 = ( nodeVar1 * 2u );
		nodeVar3 = ( u32( NodeBuffer_1002.value[ 0u ].z ) - 1u );
		nodeVar4 = ( instanceIndex % u32( NodeBuffer_1002.value[ 1u ].y ) );
		nodeVar5 = ( nodeVar4 * 2u );
		nodeVar6 = ( u32( NodeBuffer_1002.value[ 0u ].y ) - 1u );
		nodeVar0 = max( nodeVar0, NodeBuffer_1008.value[ ( ( u32( NodeBuffer_1002.value[ 0u ].x ) + ( min( ( nodeVar2 + 0u ), nodeVar3 ) * u32( NodeBuffer_1002.value[ 0u ].y ) ) ) + min( ( nodeVar5 + 0u ), nodeVar6 ) ) ] );
		nodeVar0 = max( nodeVar0, NodeBuffer_1008.value[ ( ( u32( NodeBuffer_1002.value[ 0u ].x ) + ( min( ( nodeVar2 + 0u ), nodeVar3 ) * u32( NodeBuffer_1002.value[ 0u ].y ) ) ) + min( ( nodeVar5 + 1u ), nodeVar6 ) ) ] );
		nodeVar0 = max( nodeVar0, NodeBuffer_1008.value[ ( ( u32( NodeBuffer_1002.value[ 0u ].x ) + ( min( ( nodeVar2 + 1u ), nodeVar3 ) * u32( NodeBuffer_1002.value[ 0u ].y ) ) ) + min( ( nodeVar5 + 0u ), nodeVar6 ) ) ] );
		nodeVar0 = max( nodeVar0, NodeBuffer_1008.value[ ( ( u32( NodeBuffer_1002.value[ 0u ].x ) + ( min( ( nodeVar2 + 1u ), nodeVar3 ) * u32( NodeBuffer_1002.value[ 0u ].y ) ) ) + min( ( nodeVar5 + 1u ), nodeVar6 ) ) ] );
		NodeBuffer_1008.value[ ( ( u32( NodeBuffer_1002.value[ 1u ].x ) + ( nodeVar1 * u32( NodeBuffer_1002.value[ 1u ].y ) ) ) + nodeVar4 ) ] = nodeVar0;
		

	}


	

}
