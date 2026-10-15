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
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec3<f32>;
var<private> density : f32;
var<private> stress : mat3x3<f32>;

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


	// flow -> p2g2Kernel
	if ( instanceIndex >= object.nodeUniform7 ) { return; }

	let particlePosition = NodeBuffer_992.value[ instanceIndex ].position;
	nodeVar0 = ( particlePosition * object.nodeUniform1 );
	let cellIndex = ( vec3<i32>( nodeVar0 ) - vec3<i32>( i32( 1.0 ) ) );
	let cellDiff = ( fract( nodeVar0 ) - vec3<f32>( 0.5 ) );
	let weights = array< vec3<f32>, 3 >( ( ( vec3<f32>( 0.5 ) * ( vec3<f32>( 0.5 ) - cellDiff ) ) * ( vec3<f32>( 0.5 ) - cellDiff ) ), ( vec3<f32>( 0.75 ) - ( cellDiff * cellDiff ) ), ( ( vec3<f32>( 0.5 ) * ( vec3<f32>( 0.5 ) + cellDiff ) ) * ( vec3<f32>( 0.5 ) + cellDiff ) ) );
	density = 0.0;

	for ( var gx : i32 = 0; gx < 3; gx ++ ) {


		for ( var gy : i32 = 0; gy < 3; gy ++ ) {


			for ( var gz : i32 = 0; gz < 3; gz ++ ) {

				let nodeConst0 = ( cellIndex + vec3<i32>( gx, gy, gz ) );
				let nodeConst1 = ( ( ( nodeConst0.x * 4096 ) + ( nodeConst0.y * 64 ) ) + nodeConst0.z );
				let nodeConst2 = atomicLoad( &NodeBuffer_994.value[ nodeConst1 ].mass );
				density = ( density + ( ( f32( nodeConst2 ) / 10000000.0 ) * ( ( weights[ gx ].x * weights[ gy ].y ) * weights[ gz ].z ) ) );

			}


		}


	}

	let pressure = max( 0.0, ( ( pow( ( density / object.nodeUniform3 ), 5.0 ) - 1.0 ) * object.nodeUniform4 ) );
	stress = mat3x3<f32>( ( - pressure ), 0.0, 0.0, 0.0, ( - pressure ), 0.0, 0.0, 0.0, ( - pressure ) );
	let C = NodeBuffer_992.value[ instanceIndex ].C;
	stress = ( stress + ( object.nodeUniform5 * ( C + transpose( C ) ) ) );

	for ( var gx : i32 = 0; gx < 3; gx ++ ) {


		for ( var gy : i32 = 0; gy < 3; gy ++ ) {


			for ( var gz : i32 = 0; gz < 3; gz ++ ) {

				let nodeConst3 = ( cellIndex + vec3<i32>( gx, gy, gz ) );
				let cellDist = ( ( vec3<f32>( nodeConst3 ) + vec3<f32>( 0.5 ) ) - nodeVar0 );
				let momentum = ( ( ( ( weights[ gx ].x * weights[ gy ].y ) * weights[ gz ].z ) * ( object.nodeUniform6 * ( ( 1.0 / density ) * -4.0 ) * stress ) ) * cellDist );
				let nodeConst4 = ( ( ( nodeConst3.x * 4096 ) + ( nodeConst3.y * 64 ) ) + nodeConst3.z );
				atomicAdd( &NodeBuffer_994.value[ nodeConst4 ].x, i32( ( momentum.x * 10000000.0 ) ) );
				atomicAdd( &NodeBuffer_994.value[ nodeConst4 ].y, i32( ( momentum.y * 10000000.0 ) ) );
				atomicAdd( &NodeBuffer_994.value[ nodeConst4 ].z, i32( ( momentum.z * 10000000.0 ) ) );

			}


		}


	}


	

}
