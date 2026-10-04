// Three.js r186 - Node System

// directives
enable subgroups;

// system
var<private> instanceIndex : u32;

// locals
var<workgroup> WorkgroupArray_2797: array< u32, 256 >;

// structs


// uniforms

struct Current_RightStruct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> Current_Right : Current_RightStruct;

// vars


// codes
fn pow2Ceil ( x : u32 ) -> u32 {

	var val : u32;


	if ( ( x == 0u ) ) {

		return 1u;

	}

	val = ( x - 1u );
	val = ( val | ( val >> 1u ) );
	val = ( val | ( val >> 2u ) );
	val = ( val | ( val >> 4u ) );
	val = ( val | ( val >> 8u ) );
	val = ( val | ( val >> 16u ) );

	return ( val + 1u );

}




@compute @workgroup_size( 256, 1, 1 )
fn main( @builtin( local_invocation_index ) invocationLocalIndex : u32,
	@builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32>,
	@builtin( subgroup_size ) subgroupSize : u32 ) {

	// local vars
	
	var k : u32;


	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 256 * numWorkgroups.x )
		+ globalId.z * ( 256 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code

	k = instanceIndex;
	WorkgroupArray_2797[ invocationLocalIndex ] = 0u;

	while ( ( k < 262144u ) ) {

		WorkgroupArray_2797[ invocationLocalIndex ] = ( WorkgroupArray_2797[ invocationLocalIndex ] + Current_Right.value[ k ] );
		k = ( k + 65536u );

	}

	workgroupBarrier();
	k = ( pow2Ceil( 256u ) / 2u );

	while ( ( f32( k ) > 0.0 ) ) {


		if ( ( ( invocationLocalIndex < k ) && ( f32( ( invocationLocalIndex + k ) ) < 256.0 ) ) ) {

			WorkgroupArray_2797[ invocationLocalIndex ] = ( WorkgroupArray_2797[ invocationLocalIndex ] + WorkgroupArray_2797[ ( invocationLocalIndex + k ) ] );
			

		}

		workgroupBarrier();
		k = ( k / 2u );

	}


	if ( ( invocationLocalIndex == 0u ) ) {

		Current_Right.value[ workgroupId.x ] = WorkgroupArray_2797[ 0u ];
		

	}


	

}
