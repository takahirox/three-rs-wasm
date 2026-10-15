// Three.js r186 - Node System

// directives
enable subgroups;

// system
var<private> instanceIndex : u32;

// locals
var<workgroup> WorkgroupArray_3780: array< u32, 64 >;

// structs


// uniforms

struct CurrentVectorized_LeftStruct {
	value : array< vec4<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> CurrentVectorized_Left : CurrentVectorized_LeftStruct;

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
	
	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var nodeVar4 : vec4<u32>;
	var subgroupSizeLog : u32;
	var nodeVar5 : u32;
	var spineSizeLog : u32;
	var nodeVar6 : u32;
	var alignedSize : u32;
	var nodeVar7 : u32;
	var nodeVar8 : u32;
	var isValidSubgroupIndex : bool;
	var nodeVar9 : u32;
	var t : u32;


	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 256 * numWorkgroups.x )
		+ globalId.z * ( 256 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code

	nodeVar0 = ( invocationLocalIndex / subgroupSize );
	nodeVar1 = ( ( nodeVar0 * subgroupSize ) * 4u );
	nodeVar1 = ( nodeVar1 + invocationSubgroupIndex );
	nodeVar2 = ( nodeVar1 + ( workgroupId.x * ( 256u * 4u ) ) );
	nodeVar3 = 0u;

	if ( ( workgroupId.x < ( 64u - 1u ) ) ) {


		for ( var currentSubgroupInBlock : u32 = 0u; currentSubgroupInBlock < 4u; currentSubgroupInBlock ++ ) {

			nodeVar3 = ( nodeVar3 + u32( dot( vec4<u32>( 1u, 1u, 1u, 1u ), CurrentVectorized_Left.value[ nodeVar2 ] ) ) );
			nodeVar2 = ( nodeVar2 + subgroupSize );

		}

		

	}


	if ( ( workgroupId.x == ( 64u - 1u ) ) ) {


		for ( var currentSubgroupInBlock : u32 = 0u; currentSubgroupInBlock < 4u; currentSubgroupInBlock ++ ) {

			nodeVar3 = ( nodeVar3 + u32( dot( select( vec4<u32>( 0u, 0u, 0u, 0u ), CurrentVectorized_Left.value[ nodeVar2 ], ( nodeVar2 < 65536u ) ), vec4<u32>( 1u, 1u, 1u, 1u ) ) ) );
			nodeVar2 = ( nodeVar2 + subgroupSize );

		}

		

	}

	nodeVar3 = subgroupAdd( nodeVar3 );

	if ( ( invocationSubgroupIndex == 0u ) ) {

		WorkgroupArray_3780[ nodeVar0 ] = nodeVar3;
		

	}

	workgroupBarrier();
	subgroupSizeLog = countTrailingZeros( subgroupSize );
	nodeVar5 = ( 256u >> subgroupSizeLog );
	spineSizeLog = countTrailingZeros( nodeVar5 );
	nodeVar6 = ( ( spineSizeLog + subgroupSizeLog ) - 1u );
	nodeVar6 = ( nodeVar6 / subgroupSizeLog );
	nodeVar6 = ( nodeVar6 * subgroupSizeLog );
	alignedSize = ( 1u << nodeVar6 );
	nodeVar7 = 0u;

	for ( var j : u32 = subgroupSize; j <= alignedSize; j <<= subgroupSizeLog ) {

		nodeVar8 = ( ( ( invocationLocalIndex + 1u ) << nodeVar7 ) - 1u );
		isValidSubgroupIndex = ( nodeVar8 < nodeVar5 );
		t = subgroupAdd( select( 0u, WorkgroupArray_3780[ nodeVar8 ], isValidSubgroupIndex ) );

		if ( isValidSubgroupIndex ) {

			WorkgroupArray_3780[ nodeVar8 ] = t;
			

		}

		workgroupBarrier();
		nodeVar7 = ( nodeVar7 + subgroupSizeLog );

	}


	if ( ( invocationLocalIndex == 0u ) ) {

		WorkgroupSums_Left.value[ workgroupId.x ] = WorkgroupArray_3780[ ( nodeVar5 - 1u ) ];
		

	}


	

}
