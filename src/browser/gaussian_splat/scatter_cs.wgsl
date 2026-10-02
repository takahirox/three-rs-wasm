// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1005Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read> NodeBuffer_1005 : NodeBuffer_1005Struct;

struct NodeBuffer_1008Struct {
	value : array< atomic<u32> >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> NodeBuffer_1008 : NodeBuffer_1008Struct;

struct NodeBuffer_1004Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> NodeBuffer_1004 : NodeBuffer_1004Struct;

struct objectStruct {
	nodeUniform3 : u32
};
@binding( 3 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> bin : u32;
var<private> targetIndex : u32;

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


	// flow -> CountingSortScatter
	if ( instanceIndex >= object.nodeUniform3 ) { return; }

	bin = NodeBuffer_1005.value[ instanceIndex ];
	let nodeConst0 = atomicAdd( &NodeBuffer_1008.value[ bin ], 1u );
	targetIndex = nodeConst0;
	NodeBuffer_1004.value[ targetIndex ] = instanceIndex;

	

}
