// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs

struct StructType0 {
	x : atomic< i32 >,
	y : atomic< i32 >,
	z : atomic< i32 >,
	mass : atomic< i32 >
};


// uniforms

struct NodeBuffer_994Struct {
	value : array< StructType0 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_994 : NodeBuffer_994Struct;

struct NodeBuffer_995Struct {
	value : array< vec4<f32> >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> NodeBuffer_995 : NodeBuffer_995Struct;

struct objectStruct {
	nodeUniform2 : u32
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : i32;
var<private> nodeVar4 : i32;
var<private> nodeVar5 : i32;

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


	// flow -> updateGridKernel
	if ( instanceIndex >= object.nodeUniform2 ) { return; }

	let nodeConst0 = atomicLoad( &NodeBuffer_994.value[ instanceIndex ].mass );
	let nodeConst1 = ( f32( nodeConst0 ) / 10000000.0 );

	if ( ( nodeConst1 <= 0.0 ) ) {

		return;
		

	}

	let nodeConst2 = atomicLoad( &NodeBuffer_994.value[ instanceIndex ].x );
	nodeVar0 = ( ( f32( nodeConst2 ) / 10000000.0 ) / nodeConst1 );
	let nodeConst3 = atomicLoad( &NodeBuffer_994.value[ instanceIndex ].y );
	nodeVar1 = ( ( f32( nodeConst3 ) / 10000000.0 ) / nodeConst1 );
	let nodeConst4 = atomicLoad( &NodeBuffer_994.value[ instanceIndex ].z );
	nodeVar2 = ( ( f32( nodeConst4 ) / 10000000.0 ) / nodeConst1 );
	nodeVar3 = ( i32( instanceIndex ) / 4096 );

	if ( ( ( nodeVar3 < 1 ) || ( nodeVar3 > ( 64 - 2 ) ) ) ) {

		nodeVar0 = 0.0;
		

	}

	nodeVar4 = ( ( i32( instanceIndex ) / 64 ) % 64 );

	if ( ( ( nodeVar4 < 1 ) || ( nodeVar4 > ( 64 - 2 ) ) ) ) {

		nodeVar1 = 0.0;
		

	}

	nodeVar5 = ( i32( instanceIndex ) % 64 );

	if ( ( ( nodeVar5 < 1 ) || ( nodeVar5 > ( 64 - 2 ) ) ) ) {

		nodeVar2 = 0.0;
		

	}

	NodeBuffer_995.value[ instanceIndex ] = vec4<f32>( nodeVar0, nodeVar1, nodeVar2, nodeConst1 );

	

}
