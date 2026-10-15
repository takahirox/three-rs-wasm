// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_991Struct {
	value : array< vec3<f32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_991 : NodeBuffer_991Struct;

struct NodeBuffer_995Struct {
	value : array< vec2<u32> >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> NodeBuffer_995 : NodeBuffer_995Struct;

struct NodeBuffer_997Struct {
	value : array< vec3<f32> >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> NodeBuffer_997 : NodeBuffer_997Struct;

struct NodeBuffer_996Struct {
	value : array< f32 >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> NodeBuffer_996 : NodeBuffer_996Struct;

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform5 : u32
};
@binding( 4 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : f32;

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


	// flow -> Spring Forces
	if ( instanceIndex >= object.nodeUniform5 ) { return; }

	nodeVar0 = ( NodeBuffer_991.value[ NodeBuffer_995.value[ instanceIndex ].y ] - NodeBuffer_991.value[ NodeBuffer_995.value[ instanceIndex ].x ] );
	nodeVar1 = max( length( nodeVar0 ), 0.000001 );
	NodeBuffer_997.value[ instanceIndex ] = ( ( ( vec3<f32>( ( ( nodeVar1 - NodeBuffer_996.value[ instanceIndex ] ) * object.nodeUniform4 ) ) * nodeVar0 ) * vec3<f32>( 0.5 ) ) / vec3<f32>( nodeVar1 ) );

	

}
