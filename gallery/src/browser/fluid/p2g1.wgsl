// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs

struct StructType0 {
	position : vec3<f32>,
	velocity : vec3<f32>,
	C : mat3x3<f32>
};

struct StructType1 {
	x : atomic< i32 >,
	y : atomic< i32 >,
	z : atomic< i32 >,
	mass : atomic< i32 >
};


// uniforms

struct NodeBuffer_992Struct {
	value : array< StructType0 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_992 : NodeBuffer_992Struct;

struct NodeBuffer_994Struct {
	value : array< StructType1 >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> NodeBuffer_994 : NodeBuffer_994Struct;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform3 : u32
};
@binding( 1 ) @group( 0 )
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


	// flow -> p2g1Kernel
	if ( instanceIndex >= object.nodeUniform3 ) { return; }

	let particlePosition = NodeBuffer_992.value[ instanceIndex ].position;
	let particleVelocity = NodeBuffer_992.value[ instanceIndex ].velocity;
	let C = NodeBuffer_992.value[ instanceIndex ].C;
	nodeVar0 = ( particlePosition * object.nodeUniform1 );
	let cellIndex = ( vec3<i32>( nodeVar0 ) - vec3<i32>( i32( 1.0 ) ) );
	let cellDiff = ( fract( nodeVar0 ) - vec3<f32>( 0.5 ) );
	let weights = array< vec3<f32>, 3 >( ( ( vec3<f32>( 0.5 ) * ( vec3<f32>( 0.5 ) - cellDiff ) ) * ( vec3<f32>( 0.5 ) - cellDiff ) ), ( vec3<f32>( 0.75 ) - ( cellDiff * cellDiff ) ), ( ( vec3<f32>( 0.5 ) * ( vec3<f32>( 0.5 ) + cellDiff ) ) * ( vec3<f32>( 0.5 ) + cellDiff ) ) );

	for ( var gx : i32 = 0; gx < 3; gx ++ ) {


		for ( var gy : i32 = 0; gy < 3; gy ++ ) {


			for ( var gz : i32 = 0; gz < 3; gz ++ ) {

				let nodeConst0 = ( cellIndex + vec3<i32>( gx, gy, gz ) );
				let cellDist = ( ( vec3<f32>( nodeConst0 ) + vec3<f32>( 0.5 ) ) - nodeVar0 );
				nodeVar1 = ( ( weights[ gx ].x * weights[ gy ].y ) * weights[ gz ].z );
				let velContrib = ( vec3<f32>( nodeVar1 ) * ( particleVelocity + ( C * cellDist ) ) );
				let nodeConst1 = ( ( ( nodeConst0.x * 4096 ) + ( nodeConst0.y * 64 ) ) + nodeConst0.z );
				atomicAdd( &NodeBuffer_994.value[ nodeConst1 ].x, i32( ( velContrib.x * 10000000.0 ) ) );
				atomicAdd( &NodeBuffer_994.value[ nodeConst1 ].y, i32( ( velContrib.y * 10000000.0 ) ) );
				atomicAdd( &NodeBuffer_994.value[ nodeConst1 ].z, i32( ( velContrib.z * 10000000.0 ) ) );
				atomicAdd( &NodeBuffer_994.value[ nodeConst1 ].mass, i32( ( nodeVar1 * 10000000.0 ) ) );

			}


		}


	}


	

}
