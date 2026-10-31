// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 4 ) @group( 0 ) var nodeUniform6_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform6 : texture_3d<f32>;
@binding( 6 ) @group( 0 ) var nodeUniform11_sampler : sampler;
@binding( 7 ) @group( 0 ) var nodeUniform11 : texture_3d<f32>;
@binding( 8 ) @group( 0 ) var nodeUniform13_sampler : sampler;
@binding( 9 ) @group( 0 ) var nodeUniform13 : texture_3d<f32>;
@binding( 10 ) @group( 0 ) var nodeUniform14 : texture_storage_3d<rgba16float, write>;

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

struct objectStruct {
	nodeUniform0 : u32,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform12 : f32,
	nodeUniform15 : u32
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;

// codes
fn fn5 ( seed : u32 ) -> f32 {

	

	let nodeConst0 = ( ( seed * 747796405u ) + 2891336453u );
	let nodeConst1 = ( ( ( nodeConst0 >> ( ( nodeConst0 >> 28u ) + 4u ) ) ^ nodeConst0 ) * 277803737u );

	return ( f32( ( ( nodeConst1 >> 22u ) ^ nodeConst1 ) ) * 2.3283064365386963e-10 );

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


	// flow -> VXGI.Bounce
	if ( instanceIndex >= object.nodeUniform15 ) { return; }


	if ( ( instanceIndex < object.nodeUniform0 ) ) {

		let nodeConst0 = vec3<u32>( ( instanceIndex % 144u ), ( ( instanceIndex / 144u ) % 64u ), ( instanceIndex / 9216u ) );
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
			nodeVar3 = textureSampleLevel( nodeUniform6, nodeUniform6_sampler, ( ( vec3<f32>( nodeConst0 ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 144.0, 64.0, 96.0 ) ), 0.0 );
			let nodeConst9 = nodeVar3;
			let nodeConst10 = fn5( instanceIndex );
			let nodeConst11 = ( nodeConst8 + ( nodeVar1 * vec3<f32>( ( object.nodeUniform5 * 1.5 ) ) ) );

			if ( ( abs( nodeVar1.y ) < 0.99 ) ) {

				nodeVar4 = vec3<f32>( 0.0, 1.0, 0.0 );

			} else {

				nodeVar4 = vec3<f32>( 1.0, 0.0, 0.0 );

			}

			let nodeConst12 = normalize( cross( nodeVar1, nodeVar4 ) );
			let nodeConst13 = cross( nodeVar1, nodeConst12 );
			nodeVar5 = vec3<f32>( 0.0, 0.0, 0.0 );
			nodeVar6 = 0.0;

			for ( var c : u32 = 0u; c < 8u; c ++ ) {

				let nodeConst14 = ( ( f32( c ) + 0.5 ) / 8.0 );
				let nodeConst15 = fract( ( ( f32( c ) * 0.618034 ) + nodeConst10 ) );
				let nodeConst16 = sqrt( nodeConst14 );
				let nodeConst17 = sqrt( ( 1.0 - nodeConst14 ) );
				let nodeConst18 = ( nodeConst15 * ( 3.141592653589793 * 2.0 ) );
				let nodeConst19 = normalize( ( ( ( nodeConst12 * vec3<f32>( ( cos( nodeConst18 ) * nodeConst16 ) ) ) + ( nodeConst13 * vec3<f32>( ( sin( nodeConst18 ) * nodeConst16 ) ) ) ) + ( nodeVar1 * vec3<f32>( nodeConst17 ) ) ) );
				nodeVar7 = vec3<f32>( 0.0, 0.0, 0.0 );
				nodeVar8 = 0.0;
				nodeVar9 = 0.0;
				let nodeConst20 = ( object.nodeUniform4 + object.nodeUniform7 );

				if ( ( abs( nodeConst19.x ) < 0.000001 ) ) {

					nodeVar10 = 0.000001;

				} else {

					nodeVar10 = nodeConst19.x;

				}


				if ( ( abs( nodeConst19.y ) < 0.000001 ) ) {

					nodeVar11 = 0.000001;

				} else {

					nodeVar11 = nodeConst19.y;

				}


				if ( ( abs( nodeConst19.z ) < 0.000001 ) ) {

					nodeVar12 = 0.000001;

				} else {

					nodeVar12 = nodeConst19.z;

				}

				let nodeConst21 = vec3<f32>( nodeVar10, nodeVar11, nodeVar12 );
				let nodeConst22 = ( vec3<f32>( 1.0 ) / nodeConst21 );
				let nodeConst23 = ( ( object.nodeUniform4 - nodeConst11 ) * nodeConst22 );
				let nodeConst24 = ( ( nodeConst20 - nodeConst11 ) * nodeConst22 );
				let nodeConst25 = min( nodeConst23, nodeConst24 );
				let nodeConst26 = max( nodeConst23, nodeConst24 );
				let nodeConst27 = max( max( nodeConst25.x, nodeConst25.y ), max( nodeConst25.z, 0.0 ) );
				let nodeConst28 = min( min( nodeConst26.x, nodeConst26.y ), nodeConst26.z );
				nodeVar13 = max( nodeConst27, object.nodeUniform5 );

				if ( ( object.nodeUniform8 > 0.0 ) ) {

					nodeVar14 = object.nodeUniform8;

				} else {

					nodeVar14 = 10000000000.0;

				}

				let nodeConst29 = min( nodeConst28, nodeVar14 );
				let nodeConst30 = ( nodeConst19 * nodeConst19 );

				if ( ( nodeConst28 > nodeVar13 ) ) {


					for ( var s : i32 = 0; s < 128; s ++ ) {


						if ( ( ( nodeVar13 >= nodeConst29 ) || ( nodeVar8 >= 0.98 ) ) ) {

							break;
							

						}

						let nodeConst31 = max( ( ( nodeVar13 * 2.0 ) * tan( radians( ( object.nodeUniform9 * 0.5 ) ) ) ), object.nodeUniform5 );
						let nodeConst32 = clamp( log2( ( nodeConst31 / object.nodeUniform5 ) ), 0.0, object.nodeUniform10 );
						let nodeConst33 = ( nodeConst11 + ( nodeConst19 * vec3<f32>( nodeVar13 ) ) );
						let nodeConst34 = ( ( nodeConst33 - object.nodeUniform4 ) / object.nodeUniform7 );
						nodeVar15 = textureSampleLevel( nodeUniform11, nodeUniform11_sampler, nodeConst34, nodeConst32 );
						let nodeConst35 = nodeVar15;
						let nodeConst36 = ( 1.0 - pow( ( 1.0 - clamp( dot( nodeConst35.xyz, nodeConst30 ), 0.0, 1.0 ) ), object.nodeUniform12 ) );
						let nodeConst37 = ( nodeConst36 * ( 1.0 - nodeVar8 ) );
						nodeVar16 = textureSampleLevel( nodeUniform13, nodeUniform13_sampler, nodeConst34, nodeConst32 );
						let nodeConst38 = nodeVar16;
						nodeVar7 = ( nodeVar7 + ( ( nodeConst38.xyz / vec3<f32>( max( nodeConst38.w, 0.0001 ) ) ) * vec3<f32>( nodeConst37 ) ) );
						nodeVar8 = ( nodeVar8 + nodeConst37 );
						nodeVar13 = ( nodeVar13 + ( ( object.nodeUniform5 * exp2( nodeConst32 ) ) * object.nodeUniform12 ) );

					}

					

				}

				nodeVar5 = ( nodeVar5 + nodeVar7 );

			}

			nodeVar0 = vec4<f32>( ( nodeConst9.xyz + ( ( nodeConst6.xyz * ( nodeVar5 / vec3<f32>( 8.0 ) ) ) * vec3<f32>( nodeConst9.w ) ) ), nodeConst9.w );
			

		}

		textureStore( nodeUniform14, nodeConst0, nodeVar0 );
		

	}


	

}
