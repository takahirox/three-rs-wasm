// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs

struct StructType0 {
	x : atomic< i32 >,
	y : atomic< i32 >,
	z : atomic< i32 >,
	mass : atomic< i32 >
};


// uniforms

struct NodeBuffer_994Struct {
	value : array< StructType0 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_994 : NodeBuffer_994Struct;

struct objectStruct {
	nodeUniform1 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars


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


	// flow -> clearGridKernel
	if ( instanceIndex >= object.nodeUniform1 ) { return; }

	atomicStore( &NodeBuffer_994.value[ instanceIndex ].x, 0 );
	atomicStore( &NodeBuffer_994.value[ instanceIndex ].y, 0 );
	atomicStore( &NodeBuffer_994.value[ instanceIndex ].z, 0 );
	atomicStore( &NodeBuffer_994.value[ instanceIndex ].mass, 0 );

	

}
