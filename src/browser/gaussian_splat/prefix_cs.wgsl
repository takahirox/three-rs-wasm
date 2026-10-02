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
var<private> sum : u32;
var<private> count : u32;

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


	// flow -> CountingSortPrefix
	if ( instanceIndex >= object.nodeUniform2 ) { return; }

	sum = 0u;

	for ( var bin : u32 = 0u; bin < 4096u; bin ++ ) {

		let nodeConst0 = atomicLoad( &NodeBuffer_1007.value[ bin ] );
		count = nodeConst0;
		atomicStore( &NodeBuffer_1008.value[ bin ], sum );
		sum = ( sum + count );

	}


	

}
