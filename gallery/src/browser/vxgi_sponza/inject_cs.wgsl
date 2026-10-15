// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 5 ) @group( 0 ) var nodeUniform9_sampler : sampler;
@binding( 6 ) @group( 0 ) var nodeUniform9 : texture_depth_2d;
@binding( 7 ) @group( 0 ) var nodeUniform10 : texture_storage_3d<rgba16float, write>;
@binding( 8 ) @group( 0 ) var nodeUniform11 : texture_storage_3d<rgba16float, write>;

struct NodeBuffer_15215Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read> NodeBuffer_15215 : NodeBuffer_15215Struct;

struct NodeBuffer_15216Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read> NodeBuffer_15216 : NodeBuffer_15216Struct;

struct NodeBuffer_9863Struct {
	value : array< vec4<f32> >
};
@binding( 3 ) @group( 0 )
var<storage, read> NodeBuffer_9863 : NodeBuffer_9863Struct;

struct NodeBuffer_1063Struct {
	value : array< vec4<f32>, 32 >
};
@binding( 4 ) @group( 0 )
var<uniform> NodeBuffer_1063 : NodeBuffer_1063Struct;

struct objectStruct {
	nodeUniform0 : u32,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform8 : vec4<f32>,
	nodeUniform12 : u32
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;

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


	// flow -> VXGI.Inject
	if ( instanceIndex >= object.nodeUniform12 ) { return; }


	if ( ( instanceIndex < object.nodeUniform0 ) ) {

		let nodeConst0 = vec3<u32>( ( instanceIndex % 144u ), ( ( instanceIndex / 144u ) % 64u ), ( instanceIndex / 9216u ) );
		let nodeConst1 = NodeBuffer_15215.value[ instanceIndex ];
		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

		if ( ( nodeConst1 != 0u ) ) {

			let nodeConst2 = ( ( NodeBuffer_15216.value[ instanceIndex ] - 1u ) * 5u );
			let nodeConst3 = NodeBuffer_9863.value[ nodeConst2 ].xyz;
			let nodeConst4 = NodeBuffer_9863.value[ ( nodeConst2 + 1u ) ].xyz;
			let nodeConst5 = NodeBuffer_9863.value[ ( nodeConst2 + 2u ) ].xyz;
			let nodeConst6 = NodeBuffer_9863.value[ ( nodeConst2 + 3u ) ];
			let nodeConst7 = NodeBuffer_9863.value[ ( nodeConst2 + 4u ) ].xyz;
			nodeVar1 = normalize( cross( ( nodeConst4 - nodeConst3 ), ( nodeConst5 - nodeConst3 ) ) );

			if ( ( nodeConst6.w == 1.0 ) ) {

				nodeVar2 = ( - nodeVar1 );

			} else {

				nodeVar2 = nodeVar1;

			}

			nodeVar1 = nodeVar2;
			let nodeConst8 = ( object.nodeUniform4 + ( ( vec3<f32>( nodeConst0 ) + vec3<f32>( 0.5 ) ) * vec3<f32>( object.nodeUniform5 ) ) );
			nodeVar3 = vec3<f32>( 0.0, 0.0, 0.0 );
			let nodeConst9 = NodeBuffer_1063.value[ 0u ];
			let nodeConst10 = NodeBuffer_1063.value[ 1u ];
			let nodeConst11 = NodeBuffer_1063.value[ 2u ];
			let nodeConst12 = NodeBuffer_1063.value[ 3u ];
			nodeVar4 = vec3<f32>( 0.0, 0.0, 0.0 );
			nodeVar5 = 10000000000.0;
			nodeVar6 = 1.0;
			nodeVar4 = nodeConst10.xyz;
			nodeVar7 = dot( nodeVar1, nodeVar4 );

			if ( ( nodeConst6.w == 2.0 ) ) {

				nodeVar8 = abs( nodeVar7 );

			} else {

				nodeVar8 = max( nodeVar7, 0.0 );

			}

			nodeVar7 = nodeVar8;

			if ( ( ( nodeVar7 > 0.0 ) && ( nodeVar6 > 0.0 ) ) ) {

				nodeVar9 = 1.0;
				let nodeConst13 = ( nodeConst8 + ( nodeVar1 * vec3<f32>( object.nodeUniform5 ) ) );
				let nodeConst14 = ( object.nodeUniform7 * vec4<f32>( nodeConst13, 1.0 ) );
				let nodeConst15 = ( nodeConst14.xyz / vec3<f32>( nodeConst14.w ) );
				let nodeConst16 = vec2<f32>( nodeConst15.x, ( 1.0 - nodeConst15.y ) );
				let nodeConst17 = ( ( ( ( ( ( nodeConst16.x >= 0.0 ) && ( nodeConst16.x <= 1.0 ) ) && ( nodeConst16.y >= 0.0 ) ) && ( nodeConst16.y <= 1.0 ) ) && ( nodeConst15.z >= 0.0 ) ) && ( nodeConst15.z <= 1.0 ) );

				if ( nodeConst17 ) {

					nodeVar11 = textureSampleLevel( nodeUniform9, nodeUniform9_sampler, nodeConst16, 0 );

					if ( ( ( nodeConst15.z + object.nodeUniform8.x ) <= nodeVar11 ) ) {

						nodeVar10 = 1.0;

					} else {

						nodeVar10 = 0.0;

					}

					nodeVar9 = nodeVar10;
					

				}

				nodeVar3 = ( nodeVar3 + ( nodeConst11.xyz * vec3<f32>( ( ( nodeVar6 * nodeVar7 ) * nodeVar9 ) ) ) );
				

			}

			nodeVar12 = ( f32( countOneBits( nodeConst1 ) ) / 8.0 );
			nodeVar0 = vec4<f32>( ( ( ( ( nodeConst6.xyz * nodeVar3 ) / vec3<f32>( 3.141592653589793 ) ) + nodeConst7 ) * vec3<f32>( nodeVar12 ) ), nodeVar12 );
			

		}

		textureStore( nodeUniform10, nodeConst0, nodeVar0 );
		textureStore( nodeUniform11, nodeConst0, nodeVar0 );
		

	}


	

}
