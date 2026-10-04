// Three.js r186 - Node System

// directives
enable subgroups;

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct Current_LeftStruct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> Current_Left : Current_LeftStruct;

struct objectStruct {
	nodeUniform1 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> dispatchSize : u32;
var<private> nodeVar0 : u32;
var<private> k : u32;

// codes


@compute @workgroup_size( 1, 1, 1 )
fn main( @builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32>,
	@builtin( subgroup_size ) subgroupSize : u32 ) {

	// local vars
	

	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 1 * numWorkgroups.x )
		+ globalId.z * ( 1 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code

	if ( instanceIndex >= object.nodeUniform1 ) { return; }

	dispatchSize = 1u;
	nodeVar0 = 0u;
	k = instanceIndex;

	while ( ( k < 256u ) ) {

		nodeVar0 = ( nodeVar0 + Current_Left.value[ k ] );
		k = ( k + dispatchSize );

	}

	Current_Left.value[ instanceIndex ] = nodeVar0;

	

}
