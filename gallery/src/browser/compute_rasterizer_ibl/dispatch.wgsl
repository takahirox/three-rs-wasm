// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1016Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_1016 : NodeBuffer_1016Struct;

struct NodeBuffer_1015Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read> NodeBuffer_1015 : NodeBuffer_1015Struct;

struct objectStruct {
	nodeUniform2 : u32
};
@binding( 2 ) @group( 0 )
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


	// flow -> Compute Dispatch
	if ( instanceIndex >= object.nodeUniform2 ) { return; }

	NodeBuffer_1016.value[ 0u ] = min( NodeBuffer_1015.value[ 0u ], 65535u );
	NodeBuffer_1016.value[ 1u ] = ( ( ( NodeBuffer_1015.value[ 0u ] + 65535u ) - 1u ) / 65535u );
	NodeBuffer_1016.value[ 2u ] = 1u;

	

}
