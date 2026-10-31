// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform1Struct {
	value : array< atomic<u32> >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> nodeUniform1 : nodeUniform1Struct;

struct nodeUniform2Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> nodeUniform2 : nodeUniform2Struct;

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

	bin = nodeUniform0.value[ instanceIndex ];
	let nodeConst0 = atomicAdd( &nodeUniform1.value[ bin ], 1u );
	targetIndex = nodeConst0;
	nodeUniform2.value[ targetIndex ] = instanceIndex;

	

}
