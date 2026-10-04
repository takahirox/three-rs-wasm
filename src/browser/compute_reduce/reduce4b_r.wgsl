// Three.js r186 - Node System

// directives
enable subgroups;

// system
var<private> instanceIndex : u32;

// locals
var<workgroup> WorkgroupArray_4572: array< u32, 8 >;

// structs


// uniforms

struct WorkgroupSums_RightStruct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> WorkgroupSums_Right : WorkgroupSums_RightStruct;

struct Current_RightStruct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> Current_Right : Current_RightStruct;

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
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var blockLimiter : u32;
	var localThreadOffset : u32;
	var delta : u32;
	var nodeVar4 : u32;


	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 32 * numWorkgroups.x )
		+ globalId.z * ( 32 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code

	total = 0u;
	block = 0u;
	nodeVar1 = ( workgroupId.x * 2048u );
	nodeVar3 = ( 32u * 4u );
	blockLimiter = ( select( 2048u, select( 0u, ( 128u - nodeVar1 ), ( 128u > nodeVar1 ) ), ( ( nodeVar1 + 2048u ) > 128u ) ) / nodeVar3 );

	while ( ( block < blockLimiter ) ) {

		localThreadOffset = 0u;

		while ( ( f32( localThreadOffset ) < 4.0 ) ) {

			total = ( total + WorkgroupSums_Right.value[ ( ( nodeVar1 + ( ( block * nodeVar3 ) + ( invocationLocalIndex * 4u ) ) ) + localThreadOffset ) ] );
			localThreadOffset = ( localThreadOffset + 1u );

		}

		block = ( block + 1u );

	}

	total = subgroupAdd( total );
	delta = ( 32u / subgroupSize );

	while ( ( f32( delta ) > 1.0 ) ) {


		if ( ( f32( invocationSubgroupIndex ) == 0.0 ) ) {

			WorkgroupArray_4572[ ( invocationLocalIndex / subgroupSize ) ] = total;
			

		}

		workgroupBarrier();
		total = select( 0u, WorkgroupArray_4572[ invocationLocalIndex ], ( invocationLocalIndex < delta ) );
		total = subgroupAdd( total );
		delta = ( delta / subgroupSize );

	}


	if ( ( f32( invocationLocalIndex ) == 0.0 ) ) {

		Current_Right.value[ workgroupId.x ] = total;
		

	}


	

}
