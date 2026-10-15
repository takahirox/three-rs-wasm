// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1035Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_1035 : NodeBuffer_1035Struct;

struct NodeBuffer_1036Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> NodeBuffer_1036 : NodeBuffer_1036Struct;

struct NodeBuffer_1037Struct {
	value : array< u32 >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> NodeBuffer_1037 : NodeBuffer_1037Struct;

struct objectStruct {
	nodeUniform1 : u32,
	nodeUniform4 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : u32;

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

	if ( instanceIndex >= object.nodeUniform4 ) { return; }

	nodeVar0 = ( ( ( object.nodeUniform1 - 1u ) / 64u ) + 1u );
	NodeBuffer_1035.value[ 0u ] = nodeVar0;
	NodeBuffer_1036.value[ 0u ] = nodeVar0;
	NodeBuffer_1037.value[ 0u ] = nodeVar0;

	

}
