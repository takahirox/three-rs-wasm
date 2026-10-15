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


// uniforms

struct NodeBuffer_992Struct {
	value : array< StructType0 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> NodeBuffer_992 : NodeBuffer_992Struct;

struct NodeBuffer_995Struct {
	value : array< vec4<f32> >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> NodeBuffer_995 : NodeBuffer_995Struct;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> particlePosition : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> B : mat3x3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;

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


	// flow -> g2pKernel
	if ( instanceIndex >= object.nodeUniform8 ) { return; }

	particlePosition = NodeBuffer_992.value[ instanceIndex ].position;
	nodeVar0 = ( particlePosition * object.nodeUniform1 );
	nodeVar1 = vec3<f32>( 0.0, 0.0, 0.0 );
	let cellIndex = ( vec3<i32>( nodeVar0 ) - vec3<i32>( i32( 1.0 ) ) );
	let cellDiff = ( fract( nodeVar0 ) - vec3<f32>( 0.5 ) );
	let weights = array< vec3<f32>, 3 >( ( ( vec3<f32>( 0.5 ) * ( vec3<f32>( 0.5 ) - cellDiff ) ) * ( vec3<f32>( 0.5 ) - cellDiff ) ), ( vec3<f32>( 0.75 ) - ( cellDiff * cellDiff ) ), ( ( vec3<f32>( 0.5 ) * ( vec3<f32>( 0.5 ) + cellDiff ) ) * ( vec3<f32>( 0.5 ) + cellDiff ) ) );
	B = mat3x3<f32>( 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 );

	for ( var gx : i32 = 0; gx < 3; gx ++ ) {


		for ( var gy : i32 = 0; gy < 3; gy ++ ) {


			for ( var gz : i32 = 0; gz < 3; gz ++ ) {

				let nodeConst0 = ( cellIndex + vec3<i32>( gx, gy, gz ) );
				let cellDist = ( ( vec3<f32>( nodeConst0 ) + vec3<f32>( 0.5 ) ) - nodeVar0 );
				let nodeConst1 = ( ( ( nodeConst0.x * 4096 ) + ( nodeConst0.y * 64 ) ) + nodeConst0.z );
				let weightedVelocity = ( NodeBuffer_995.value[ nodeConst1 ].xyz * vec3<f32>( ( ( weights[ gx ].x * weights[ gy ].y ) * weights[ gz ].z ) ) );
				B = ( B + mat3x3<f32>( ( weightedVelocity * vec3<f32>( cellDist.x ) ), ( weightedVelocity * vec3<f32>( cellDist.y ) ), ( weightedVelocity * vec3<f32>( cellDist.z ) ) ) );
				nodeVar1 = ( nodeVar1 + weightedVelocity );

			}


		}


	}

	NodeBuffer_992.value[ instanceIndex ].C = ( 4.0 * B );
	nodeVar1 = ( nodeVar1 + ( object.nodeUniform3 * vec3<f32>( object.nodeUniform4 ) ) );
	nodeVar1 = ( nodeVar1 / object.nodeUniform1 );
	nodeVar1 = ( nodeVar1 + ( object.nodeUniform5 * vec3<f32>( pow( max( ( 1.0 - ( length( cross( object.nodeUniform6, ( particlePosition - object.nodeUniform7 ) ) ) * 3.0 ) ), 0.0 ), 2.0 ) ) ) );
	particlePosition = ( particlePosition + ( nodeVar1 * vec3<f32>( object.nodeUniform4 ) ) );
	particlePosition = clamp( particlePosition, ( vec3<f32>( 1.0, 1.0, 1.0 ) / object.nodeUniform1 ), ( ( vec3<f32>( 64.0, 64.0, 64.0 ) - vec3<f32>( 1.0 ) ) / object.nodeUniform1 ) );
	nodeVar2 = ( ( ( object.nodeUniform1 * vec3<f32>( 0.5 ) ) - vec3<f32>( 9.0 ) ) / object.nodeUniform1 );
	let posNext = ( particlePosition + ( ( nodeVar1 * vec3<f32>( object.nodeUniform4 ) ) * vec3<f32>( 2.0 ) ) );
	nodeVar3 = ( posNext - vec3<f32>( 0.5 ) );
	nodeVar4 = ( step( nodeVar2, abs( nodeVar3 ) ) * ( nodeVar3 + ( ( - nodeVar2 ) * sign( nodeVar3 ) ) ) );
	nodeVar5 = length( nodeVar4 );
	nodeVar6 = ( nodeVar5 - ( 6.0 / object.nodeUniform1.x ) );

	if ( ( nodeVar6 > 0.0 ) ) {

		nodeVar3 = ( nodeVar3 - ( ( normalize( nodeVar4 ) * vec3<f32>( nodeVar6 ) ) * vec3<f32>( 1.3 ) ) );
		

	}

	nodeVar3 = ( nodeVar3 + vec3<f32>( 0.5 ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar3 - posNext ) );
	nodeVar1 = ( nodeVar1 * object.nodeUniform1 );
	NodeBuffer_992.value[ instanceIndex ].position = particlePosition;
	NodeBuffer_992.value[ instanceIndex ].velocity = nodeVar1;

	

}
