// Three.js r187 - Node System

// directives
enable subgroups;

// system
var<private> instanceIndex : u32;

// locals
var<workgroup> WorkgroupArray_1566: array< u32, 8 >;

// structs


// uniforms

struct WorkgroupSums_LeftStruct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> WorkgroupSums_Left : WorkgroupSums_LeftStruct;

struct Current_LeftStruct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> Current_Left : Current_LeftStruct;

// vars


// codes


@compute @workgroup_size( 32, 1, 1 )
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
		+ globalId.y * ( 32 * numWorkgroups.x )
		+ globalId.z * ( 32 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code

	total = 0u;
	block = 0u;
	let nodeConst0 = ( workgroupId.x * 2048u );
	let nodeConst1 = ( 32u * 4u );
	blockLimiter = ( select( 2048u, select( 0u, ( 128u - nodeConst0 ), ( 128u > nodeConst0 ) ), ( ( nodeConst0 + 2048u ) > 128u ) ) / nodeConst1 );

	while ( ( block < blockLimiter ) ) {

		localThreadOffset = 0u;

		while ( ( localThreadOffset < 4u ) ) {

			total = ( total + WorkgroupSums_Left.value[ ( ( nodeConst0 + ( ( block * nodeConst1 ) + ( invocationLocalIndex * 4u ) ) ) + localThreadOffset ) ] );
			localThreadOffset = ( localThreadOffset + 1u );

		}

		block = ( block + 1u );

	}

	total = subgroupAdd( total );
	delta = ( 32u / subgroupSize );

	while ( ( f32( delta ) > 1.0 ) ) {


		if ( ( invocationSubgroupIndex == 0u ) ) {

			WorkgroupArray_1566[ ( invocationLocalIndex / subgroupSize ) ] = total;
			

		}

		workgroupBarrier();
		total = select( 0u, WorkgroupArray_1566[ invocationLocalIndex ], ( invocationLocalIndex < delta ) );
		total = subgroupAdd( total );
		delta = ( delta / subgroupSize );

	}


	if ( ( invocationLocalIndex == 0u ) ) {

		Current_Left.value[ workgroupId.x ] = total;
		

	}


	

}
