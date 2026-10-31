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
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform1Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read> nodeUniform1 : nodeUniform1Struct;

struct objectStruct {
	nodeUniform2 : u32,
	nodeUniform3 : u32
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


	// flow -> MeshletDrawingPipeline Cluster Args
	if ( instanceIndex >= object.nodeUniform3 ) { return; }

	let nodeConst0 = ( ( min( nodeUniform1.value[ 0u ], object.nodeUniform2 ) + 63u ) / 64u );
	nodeUniform0.value[ 0u ] = min( nodeConst0, 65535u );
	nodeUniform0.value[ 1u ] = ( ( nodeConst0 + 65534u ) / 65535u );
	nodeUniform0.value[ 2u ] = 1u;

	

}
