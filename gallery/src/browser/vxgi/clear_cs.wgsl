// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_5319Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> NodeBuffer_5319 : NodeBuffer_5319Struct;

struct NodeBuffer_5320Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> NodeBuffer_5320 : NodeBuffer_5320Struct;

struct objectStruct {
	nodeUniform0 : u32,
	nodeUniform3 : u32
};
@binding( 0 ) @group( 0 )
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


	// flow -> VXGI.Clear
	if ( instanceIndex >= object.nodeUniform3 ) { return; }


	if ( ( instanceIndex < object.nodeUniform0 ) ) {

		NodeBuffer_5319.value[ instanceIndex ] = 0u;
		NodeBuffer_5320.value[ instanceIndex ] = 0u;
		

	}


	

}
