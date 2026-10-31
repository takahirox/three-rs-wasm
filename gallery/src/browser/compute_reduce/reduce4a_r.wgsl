// Three.js r187 - Node System

// directives
enable subgroups;

// system
var<private> instanceIndex : u32;

// locals
var<workgroup> WorkgroupArray_1555: array< u32, 64 >;

// structs


// uniforms

struct CurrentVectorized_RightStruct {
	value : array< vec4<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> CurrentVectorized_Right : CurrentVectorized_RightStruct;

struct WorkgroupSums_RightStruct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> WorkgroupSums_Right : WorkgroupSums_RightStruct;

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
	
	var nodeVar3 : vec4<u32>;
	var subgroupSizeLog : u32;
	var spineSizeLog : u32;
	var alignedSize : u32;
	var isValidSubgroupIndex : bool;
	var nodeVar6 : u32;
	var t : u32;


	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 256 * numWorkgroups.x )
		+ globalId.z * ( 256 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code

	let nodeConst0 = ( invocationLocalIndex / subgroupSize );
	var nodeVar0 : u32 = ( ( nodeConst0 * subgroupSize ) * 4u );
	nodeVar0 = ( nodeVar0 + invocationSubgroupIndex );
	var nodeVar1 : u32 = ( nodeVar0 + ( workgroupId.x * ( 256u * 4u ) ) );
	var nodeVar2 : u32 = 0u;

	if ( ( workgroupId.x < ( 64u - 1u ) ) ) {


		for ( var currentSubgroupInBlock : u32 = 0u; currentSubgroupInBlock < 4u; currentSubgroupInBlock ++ ) {

			nodeVar2 = u32( ( f32( nodeVar2 ) + dot( vec4<f32>( vec4<u32>( 1u, 1u, 1u, 1u ) ), vec4<f32>( CurrentVectorized_Right.value[ nodeVar1 ] ) ) ) );
			nodeVar1 = ( nodeVar1 + subgroupSize );

		}

		

	}


	if ( ( workgroupId.x == ( 64u - 1u ) ) ) {


		for ( var currentSubgroupInBlock : u32 = 0u; currentSubgroupInBlock < 4u; currentSubgroupInBlock ++ ) {

			nodeVar2 = u32( ( f32( nodeVar2 ) + dot( vec4<f32>( select( vec4<u32>( 0u, 0u, 0u, 0u ), CurrentVectorized_Right.value[ nodeVar1 ], ( nodeVar1 < 65536u ) ) ), vec4<f32>( vec4<u32>( 1u, 1u, 1u, 1u ) ) ) ) );
			nodeVar1 = ( nodeVar1 + subgroupSize );

		}

		

	}

	nodeVar2 = subgroupAdd( nodeVar2 );

	if ( ( invocationSubgroupIndex == 0u ) ) {

		WorkgroupArray_1555[ nodeConst0 ] = nodeVar2;
		

	}

	workgroupBarrier();
	subgroupSizeLog = countTrailingZeros( subgroupSize );
	let nodeConst1 = ( 256u >> subgroupSizeLog );
	spineSizeLog = countTrailingZeros( nodeConst1 );
	var nodeVar4 : u32 = ( ( spineSizeLog + subgroupSizeLog ) - 1u );
	nodeVar4 = ( nodeVar4 / subgroupSizeLog );
	nodeVar4 = ( nodeVar4 * subgroupSizeLog );
	alignedSize = ( 1u << nodeVar4 );
	var nodeVar5 : u32 = 0u;

	for ( var j : u32 = subgroupSize; j <= alignedSize; j <<= subgroupSizeLog ) {

		let nodeConst2 = ( ( ( invocationLocalIndex + 1u ) << nodeVar5 ) - 1u );
		isValidSubgroupIndex = ( nodeConst2 < nodeConst1 );
		t = subgroupAdd( select( 0u, WorkgroupArray_1555[ nodeConst2 ], isValidSubgroupIndex ) );

		if ( isValidSubgroupIndex ) {

			WorkgroupArray_1555[ nodeConst2 ] = t;
			

		}

		workgroupBarrier();
		nodeVar5 = ( nodeVar5 + subgroupSizeLog );

	}


	if ( ( invocationLocalIndex == 0u ) ) {

		WorkgroupSums_Right.value[ workgroupId.x ] = WorkgroupArray_1555[ ( nodeConst1 - 1u ) ];
		

	}


	

}
