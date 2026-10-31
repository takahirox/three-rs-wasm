// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< atomic<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform2Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> nodeUniform2 : nodeUniform2Struct;

struct nodeUniform3Struct {
	value : array< vec4<u32> >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> nodeUniform3 : nodeUniform3Struct;

struct nodeUniform4Struct {
	value : array< vec4<f32> >
};
@binding( 4 ) @group( 0 )
var<storage, read> nodeUniform4 : nodeUniform4Struct;

struct nodeUniform6Struct {
	value : array< mat4x4<f32> >
};
@binding( 5 ) @group( 0 )
var<storage, read> nodeUniform6 : nodeUniform6Struct;

struct nodeUniform9Struct {
	value : array< vec4<f32>, 6 >
};
@binding( 6 ) @group( 0 )
var<uniform> nodeUniform9 : nodeUniform9Struct;

struct nodeUniform10Struct {
	value : array< u32 >
};
@binding( 7 ) @group( 0 )
var<storage, read_write> nodeUniform10 : nodeUniform10Struct;

struct nodeUniform11Struct {
	value : array< vec4<u32> >
};
@binding( 8 ) @group( 0 )
var<storage, read_write> nodeUniform11 : nodeUniform11Struct;

struct objectStruct {
	nodeUniform1 : u32,
	nodeUniform5 : u32,
	nodeUniform7 : u32,
	nodeUniform8 : vec3<f32>
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : u32;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : vec4<u32>;
var<private> nodeVar3 : u32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : bool;
var<private> nodeVar9 : bool;
var<private> nodeVar10 : u32;

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


	// flow -> MeshletDrawingPipeline Cluster Cull
	nodeVar0 = ( ( ( ( workgroupId.y * 65535u ) + workgroupId.x ) * 64u ) + localId.x );
	let nodeConst0 = atomicLoad( &nodeUniform0.value[ 0u ] );

	if ( ( nodeVar0 >= min( nodeConst0, object.nodeUniform1 ) ) ) {

		return;
		

	}

	nodeVar1 = nodeUniform2.value[ nodeVar0 ];
	nodeVar2 = nodeUniform3.value[ nodeVar1 ];
	nodeVar3 = ( nodeVar0 - nodeVar2.x );
	nodeVar4 = ( nodeVar2.y >> 16u );
	nodeVar5 = ( nodeVar2.w >> 16u );
	let nodeConst1 = ( nodeVar2.y & 255u );
	let nodeConst2 = u32( nodeUniform4.value[ ( ( ( nodeVar5 * 8u ) + ( nodeConst1 + 2u ) ) + object.nodeUniform5 ) ].y );

	if ( ( nodeVar3 < ( nodeConst2 / 64u ) ) ) {

		let nodeConst3 = bitcast<vec4<u32>>( nodeUniform4.value[ ( ( ( nodeVar4 * 3u ) + 1u ) + object.nodeUniform7 ) ] );
		let nodeConst4 = bitcast<vec4<u32>>( nodeUniform4.value[ ( ( ( nodeVar4 * 3u ) + 0u ) + object.nodeUniform7 ) ] );
		let nodeConst5 = ( nodeVar1 - nodeConst4.x );
		let nodeConst6 = bitcast<vec4<u32>>( nodeUniform4.value[ ( ( ( nodeVar5 * 8u ) + 1u ) + object.nodeUniform5 ) ] );
		let nodeConst7 = ( ( nodeConst6.z + u32( nodeUniform4.value[ ( ( ( nodeVar5 * 8u ) + ( nodeConst1 + 2u ) ) + object.nodeUniform5 ) ].z ) ) + nodeVar3 );
		nodeVar6 = ( nodeUniform6.value[ ( nodeConst3.x + nodeConst5 ) ] * vec4<f32>( nodeUniform4.value[ ( nodeConst7 * 2u ) ].xyz, 1.0 ) ).xyz;
		let nodeConst8 = unpack2x16float(nodeVar2.z);
		nodeVar7 = ( ( nodeUniform4.value[ ( nodeConst7 * 2u ) ].w * abs( nodeConst8.x ) ) + nodeConst8.y );
		nodeVar8 = true;

		if ( ( ( nodeConst8.x > 0.0 ) && ( ( u32( nodeUniform4.value[ ( ( ( nodeVar4 * 3u ) + 2u ) + object.nodeUniform7 ) ].w ) & 1u ) != 0u ) ) ) {

			let nodeConst9 = ( nodeVar6 - object.nodeUniform8 );

			if ( ( dot( nodeConst9, normalize( ( nodeUniform6.value[ ( nodeConst3.x + nodeConst5 ) ] * vec4<f32>( nodeUniform4.value[ ( ( nodeConst7 * 2u ) + 1u ) ].xyz, 0.0 ) ).xyz ) ) >= ( ( nodeUniform4.value[ ( ( nodeConst7 * 2u ) + 1u ) ].w * length( nodeConst9 ) ) + nodeVar7 ) ) ) {

				nodeVar8 = false;
				

			}

			

		}


		for ( var plane : i32 = 0; plane < 6; plane ++ ) {

			let nodeConst10 = vec4<f32>( nodeVar6, nodeVar7 );

			if ( ( ( dot( nodeUniform9.value[ plane ].xyz, nodeConst10.xyz ) + nodeUniform9.value[ plane ].w ) < ( - nodeConst10.w ) ) ) {

				nodeVar8 = false;
				

			}


		}


		if ( nodeVar8 ) {

			nodeVar9 = ( nodeUniform10.value[ ( ( nodeConst3.z + ( nodeConst5 * nodeConst3.w ) ) + nodeVar3 ) ] == ( nodeConst1 + 1u ) );

			if ( nodeVar9 ) {


				if ( ( ( nodeVar2.y & 512u ) != 0u ) ) {

					nodeVar10 = 1u;

				} else {

					nodeVar10 = 0u;

				}

				let nodeConst11 = atomicAdd( &nodeUniform0.value[ ( ( ( nodeVar4 * 2u ) + 1u ) + nodeVar10 ) ], 1u );
				let nodeConst12 = nodeConst4.w;

				if ( ( nodeConst11 < nodeConst12 ) ) {

					nodeUniform11.value[ ( ( nodeConst4.z + ( nodeVar10 * nodeConst12 ) ) + nodeConst11 ) ] = vec4<u32>( nodeConst5, ( nodeConst6.y + u32( nodeUniform4.value[ ( ( ( nodeVar5 * 8u ) + ( nodeConst1 + 2u ) ) + object.nodeUniform5 ) ].x ) ), nodeConst2, nodeVar3 );
					

				}

				

			}

			

		}

		

	}


	

}
