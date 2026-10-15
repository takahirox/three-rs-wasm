// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1007Struct {
	value : array< atomic<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_1007 : NodeBuffer_1007Struct;

struct NodeBuffer_1008Struct {
	value : array< atomic<u32> >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> NodeBuffer_1008 : NodeBuffer_1008Struct;

struct objectStruct {
	nodeUniform2 : u32
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars


// codes


@compute @workgroup_size( 256, 1, 1 )
fn main( @builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32> ) {

	// local vars
	

	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 256 * numWorkgroups.x )
		+ globalId.z * ( 256 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code


	// flow -> CountingSortReset
	if ( instanceIndex >= object.nodeUniform2 ) { return; }

	atomicStore( &NodeBuffer_1007.value[ instanceIndex ], 0u );
	atomicStore( &NodeBuffer_1008.value[ instanceIndex ], 0u );

	

}
