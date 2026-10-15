// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1018Struct {
	value : array< atomic<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_1018 : NodeBuffer_1018Struct;

struct NodeBuffer_1020Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> NodeBuffer_1020 : NodeBuffer_1020Struct;

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


	// flow -> Compute HW Args
	if ( instanceIndex >= object.nodeUniform2 ) { return; }

	let nodeConst0 = atomicLoad( &NodeBuffer_1018.value[ 0u ] );
	NodeBuffer_1020.value[ 0u ] = ( nodeConst0 * 3u );
	NodeBuffer_1020.value[ 1u ] = 1u;
	NodeBuffer_1020.value[ 2u ] = 0u;
	NodeBuffer_1020.value[ 3u ] = 0u;

	

}
