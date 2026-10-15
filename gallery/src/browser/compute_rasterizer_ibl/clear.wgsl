// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1004Struct {
	value : array< atomic<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_1004 : NodeBuffer_1004Struct;

struct NodeBuffer_1006Struct {
	value : array< atomic<u32> >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> NodeBuffer_1006 : NodeBuffer_1006Struct;

struct NodeBuffer_1014Struct {
	value : array< atomic<u32> >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> NodeBuffer_1014 : NodeBuffer_1014Struct;

struct NodeBuffer_1018Struct {
	value : array< atomic<u32> >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> NodeBuffer_1018 : NodeBuffer_1018Struct;

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

	atomicStore( &NodeBuffer_1004.value[ instanceIndex ], 0u );
	atomicStore( &NodeBuffer_1006.value[ instanceIndex ], 0u );

	if ( ( f32( instanceIndex ) == 0.0 ) ) {

		atomicStore( &NodeBuffer_1014.value[ 0u ], 0u );
		atomicStore( &NodeBuffer_1018.value[ 0u ], 0u );
		

	}


	

}
