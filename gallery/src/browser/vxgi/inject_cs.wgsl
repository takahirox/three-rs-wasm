// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 5 ) @group( 0 ) var nodeUniform8_sampler : sampler;
@binding( 6 ) @group( 0 ) var nodeUniform8 : texture_depth_cube;
@binding( 7 ) @group( 0 ) var nodeUniform9 : texture_storage_3d<rgba16float, write>;
@binding( 8 ) @group( 0 ) var nodeUniform10 : texture_storage_3d<rgba16float, write>;

struct nodeUniform1Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read> nodeUniform1 : nodeUniform1Struct;

struct nodeUniform2Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read> nodeUniform2 : nodeUniform2Struct;

struct nodeUniform3Struct {
	value : array< vec4<f32> >
};
@binding( 3 ) @group( 0 )
var<storage, read> nodeUniform3 : nodeUniform3Struct;

struct nodeUniform6Struct {
	value : array< vec4<f32>, 32 >
};
@binding( 4 ) @group( 0 )
var<uniform> nodeUniform6 : nodeUniform6Struct;

struct objectStruct {
	nodeUniform0 : u32,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : vec4<f32>,
	nodeUniform11 : u32
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

// codes
fn fn44 ( lightDistance : f32, cutoffDistance : f32, decayExponent : f32 ) -> f32 {

	var nodeVar0 : f32;


	if ( ( cutoffDistance > 0.0 ) ) {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		let nodeConst1 = ( lightDistance / cutoffDistance );
		let nodeConst2 = clamp( ( 1.0 - ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * nodeConst1 ) ), 0.0, 1.0 );
		nodeVar0 = ( nodeConst0 * ( nodeConst2 * nodeConst2 ) );

	} else {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		nodeVar0 = nodeConst0;

	}


	return nodeVar0;

}




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
	if ( instanceIndex >= object.nodeUniform11 ) { return; }


	if ( ( instanceIndex < object.nodeUniform0 ) ) {

		let nodeConst0 = vec3<u32>( ( instanceIndex % 72u ), ( ( instanceIndex / 72u ) % 56u ), ( instanceIndex / 4032u ) );
		let nodeConst1 = nodeUniform1.value[ instanceIndex ];
		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

		if ( ( nodeConst1 != 0u ) ) {

			let nodeConst2 = ( ( nodeUniform2.value[ instanceIndex ] - 1u ) * 5u );
			let nodeConst3 = nodeUniform3.value[ nodeConst2 ].xyz;
			let nodeConst4 = nodeUniform3.value[ ( nodeConst2 + 1u ) ].xyz;
			let nodeConst5 = nodeUniform3.value[ ( nodeConst2 + 2u ) ].xyz;
			let nodeConst6 = nodeUniform3.value[ ( nodeConst2 + 3u ) ];
			let nodeConst7 = nodeUniform3.value[ ( nodeConst2 + 4u ) ].xyz;
			nodeVar1 = normalize( cross( ( nodeConst4 - nodeConst3 ), ( nodeConst5 - nodeConst3 ) ) );

			if ( ( nodeConst6.w == 1.0 ) ) {

				nodeVar2 = ( - nodeVar1 );

			} else {

				nodeVar2 = nodeVar1;

			}

			nodeVar1 = nodeVar2;
			let nodeConst8 = ( object.nodeUniform4 + ( ( vec3<f32>( nodeConst0 ) + vec3<f32>( 0.5 ) ) * vec3<f32>( object.nodeUniform5 ) ) );
			nodeVar3 = vec3<f32>( 0.0, 0.0, 0.0 );
			let nodeConst9 = nodeUniform6.value[ 0u ];
			let nodeConst10 = nodeUniform6.value[ 1u ];
			let nodeConst11 = nodeUniform6.value[ 2u ];
			let nodeConst12 = nodeUniform6.value[ 3u ];
			nodeVar4 = vec3<f32>( 0.0, 0.0, 0.0 );
			nodeVar5 = 10000000000.0;
			nodeVar6 = 1.0;
			let nodeConst13 = ( nodeConst9.xyz - nodeConst8 );
			nodeVar5 = length( nodeConst13 );
			nodeVar4 = ( nodeConst13 / vec3<f32>( nodeVar5 ) );
			nodeVar6 = fn44( nodeVar5, nodeConst10.w, nodeConst11.w );
			nodeVar7 = dot( nodeVar1, nodeVar4 );

			if ( ( nodeConst6.w == 2.0 ) ) {

				nodeVar8 = abs( nodeVar7 );

			} else {

				nodeVar8 = max( nodeVar7, 0.0 );

			}

			nodeVar7 = nodeVar8;

			if ( ( ( nodeVar7 > 0.0 ) && ( nodeVar6 > 0.0 ) ) ) {

				nodeVar9 = 1.0;
				let nodeConst14 = ( nodeConst8 + ( nodeVar1 * vec3<f32>( object.nodeUniform5 ) ) );
				let nodeConst15 = ( nodeConst14 - nodeConst9.xyz );
				let nodeConst16 = abs( nodeConst15 );
				let nodeConst17 = max( max( nodeConst16.x, nodeConst16.y ), nodeConst16.z );

				if ( ( ( nodeConst17 >= object.nodeUniform7.y ) && ( nodeConst17 <= object.nodeUniform7.z ) ) ) {

					let nodeConst18 = ( - nodeConst17 );
					let nodeConst19 = normalize( nodeConst15 );
					nodeVar11 = textureSampleLevel( nodeUniform8, nodeUniform8_sampler, vec3<f32>( ( - nodeConst19.x ), nodeConst19.yz ), 0 );

					if ( ( ( ( ( ( object.nodeUniform7.y + nodeConst18 ) * object.nodeUniform7.z ) / ( ( object.nodeUniform7.z - object.nodeUniform7.y ) * nodeConst18 ) ) + object.nodeUniform7.x ) <= nodeVar11 ) ) {

						nodeVar10 = 1.0;

					} else {

						nodeVar10 = 0.0;

					}

					nodeVar9 = nodeVar10;
					

				}

				nodeVar3 = ( nodeVar3 + ( nodeConst11.xyz * vec3<f32>( ( ( nodeVar6 * nodeVar7 ) * nodeVar9 ) ) ) );
				

			}

			let nodeConst20 = ( f32( countOneBits( nodeConst1 ) ) / 8.0 );
			nodeVar0 = vec4<f32>( ( ( ( ( nodeConst6.xyz * nodeVar3 ) / vec3<f32>( 3.141592653589793 ) ) + nodeConst7 ) * vec3<f32>( nodeConst20 ) ), nodeConst20 );
			

		}

		textureStore( nodeUniform9, nodeConst0, nodeVar0 );
		textureStore( nodeUniform10, nodeConst0, nodeVar0 );
		

	}


	

}
