// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_999Struct {
	value : array< atomic<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_999 : NodeBuffer_999Struct;

struct NodeBuffer_1001Struct {
	value : array< atomic<u32> >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> NodeBuffer_1001 : NodeBuffer_1001Struct;

struct NodeBuffer_1007Struct {
	value : array< atomic<u32> >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> NodeBuffer_1007 : NodeBuffer_1007Struct;

struct NodeBuffer_1011Struct {
	value : array< atomic<u32> >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> NodeBuffer_1011 : NodeBuffer_1011Struct;

struct objectStruct {
	nodeUniform4 : u32
};
@binding( 4 ) @group( 0 )
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


	// flow -> Compute Clear
	if ( instanceIndex >= object.nodeUniform4 ) { return; }

	atomicStore( &NodeBuffer_999.value[ instanceIndex ], 0u );
	atomicStore( &NodeBuffer_1001.value[ instanceIndex ], 0u );

	if ( ( f32( instanceIndex ) == 0.0 ) ) {

		atomicStore( &NodeBuffer_1007.value[ 0u ], 0u );
		atomicStore( &NodeBuffer_1011.value[ 0u ], 0u );
		

	}


	

}
