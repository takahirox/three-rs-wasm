// Three.js r187 - Node System

// directives
enable subgroups;

// system
var<private> instanceIndex : u32;

// locals
var<workgroup> WorkgroupArray_1420: array< u32, 64 >;

// structs


// uniforms

struct Current_LeftStruct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> Current_Left : Current_LeftStruct;

struct WorkgroupSums_LeftStruct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> WorkgroupSums_Left : WorkgroupSums_LeftStruct;

// vars


// codes


@compute @workgroup_size( 256, 1, 1 )
fn main( @builtin( local_invocation_index ) invocationLocalIndex : u32,
	@builtin( subgroup_invocation_id ) invocationSubgroupIndex : u32,
	@builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32>,
	@builtin( subgroup_size ) subgroupSize : u32 ) {

	// local vars
	
	var total : u32;
	var block : u32;
	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var blockLimiter : u32;
	var localThreadOffset : u32;
	var delta : u32;
	var nodeVar2 : u32;


	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 256 * numWorkgroups.x )
		+ globalId.z * ( 256 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code

	total = 0u;
	block = 0u;
	let nodeConst0 = ( workgroupId.x * 2048u );
	let nodeConst1 = ( 256u * 4u );
	blockLimiter = ( select( 2048u, select( 0u, ( 262144u - nodeConst0 ), ( 262144u > nodeConst0 ) ), ( ( nodeConst0 + 2048u ) > 262144u ) ) / nodeConst1 );

	while ( ( block < blockLimiter ) ) {

		localThreadOffset = 0u;

		while ( ( localThreadOffset < 4u ) ) {

			total = ( total + Current_Left.value[ ( ( nodeConst0 + ( ( block * nodeConst1 ) + ( invocationLocalIndex * 4u ) ) ) + localThreadOffset ) ] );
			localThreadOffset = ( localThreadOffset + 1u );

		}

		block = ( block + 1u );

	}

	total = subgroupAdd( total );
	delta = ( 256u / subgroupSize );

	while ( ( f32( delta ) > 1.0 ) ) {


		if ( ( invocationSubgroupIndex == 0u ) ) {

			WorkgroupArray_1420[ ( invocationLocalIndex / subgroupSize ) ] = total;
			

		}

		workgroupBarrier();
		total = select( 0u, WorkgroupArray_1420[ invocationLocalIndex ], ( invocationLocalIndex < delta ) );
		total = subgroupAdd( total );
		delta = ( delta / subgroupSize );

	}


	if ( ( invocationLocalIndex == 0u ) ) {

		WorkgroupSums_Left.value[ workgroupId.x ] = total;
		

	}


	

}
