// Three.js r186 - Node System

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

struct NodeBuffer_9974Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read> NodeBuffer_9974 : NodeBuffer_9974Struct;

struct NodeBuffer_9975Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read> NodeBuffer_9975 : NodeBuffer_9975Struct;

struct NodeBuffer_9863Struct {
	value : array< vec4<f32> >
};
@binding( 3 ) @group( 0 )
var<storage, read> NodeBuffer_9863 : NodeBuffer_9863Struct;

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
var<private> nodeVar5 : u32;
var<private> nodeVar6 : u32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;

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


	// flow -> VXGI.Bounce
	if ( instanceIndex >= object.nodeUniform15 ) { return; }


	if ( ( instanceIndex < object.nodeUniform0 ) ) {

		let nodeConst0 = vec3<u32>( ( instanceIndex % 144u ), ( ( instanceIndex / 144u ) % 64u ), ( instanceIndex / 9216u ) );
		let nodeConst1 = NodeBuffer_9974.value[ instanceIndex ];
		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

		if ( ( nodeConst1 != 0u ) ) {

			let nodeConst2 = ( ( NodeBuffer_9975.value[ instanceIndex ] - 1u ) * 5u );
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
			nodeVar3 = textureSampleLevel( nodeUniform6, nodeUniform6_sampler, ( ( vec3<f32>( nodeConst0 ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 144.0, 64.0, 96.0 ) ), 0.0 );
			let nodeConst9 = nodeVar3;

			if ( ( abs( nodeVar1.y ) < 0.99 ) ) {

				nodeVar4 = vec3<f32>( 0.0, 1.0, 0.0 );

			} else {

				nodeVar4 = vec3<f32>( 1.0, 0.0, 0.0 );

			}

			let nodeConst10 = normalize( cross( nodeVar1, nodeVar4 ) );
			let nodeConst11 = cross( nodeVar1, nodeConst10 );
			nodeVar5 = ( ( instanceIndex * 747796405u ) + 2891336453u );
			nodeVar6 = ( ( ( nodeVar5 >> ( ( nodeVar5 >> 28u ) + 4u ) ) ^ nodeVar5 ) * 277803737u );
			let nodeConst12 = ( f32( ( ( nodeVar6 >> 22u ) ^ nodeVar6 ) ) * 2.3283064365386963e-10 );
			nodeVar7 = vec3<f32>( 0.0, 0.0, 0.0 );

			for ( var c : i32 = 0; c < 8; c ++ ) {

				let nodeConst13 = ( ( f32( c ) + 0.5 ) / 8.0 );
				let nodeConst14 = fract( ( ( f32( c ) * 0.618034 ) + nodeConst12 ) );
				let nodeConst15 = sqrt( nodeConst13 );
				let nodeConst16 = sqrt( ( 1.0 - nodeConst13 ) );
				let nodeConst17 = ( nodeConst14 * ( 3.141592653589793 * 2.0 ) );
				let nodeConst18 = normalize( ( ( ( nodeConst10 * vec3<f32>( ( cos( nodeConst17 ) * nodeConst15 ) ) ) + ( nodeConst11 * vec3<f32>( ( sin( nodeConst17 ) * nodeConst15 ) ) ) ) + ( nodeVar1 * vec3<f32>( nodeConst16 ) ) ) );
				let nodeConst19 = ( nodeConst8 + ( nodeVar1 * vec3<f32>( ( object.nodeUniform5 * 1.5 ) ) ) );
				nodeVar8 = vec3<f32>( 0.0, 0.0, 0.0 );
				nodeVar9 = 0.0;
				nodeVar10 = 0.0;
				let nodeConst20 = ( object.nodeUniform4 + object.nodeUniform7 );

				if ( ( abs( nodeConst18.x ) < 0.000001 ) ) {

					nodeVar11 = 0.000001;

				} else {

					nodeVar11 = nodeConst18.x;

				}


				if ( ( abs( nodeConst18.y ) < 0.000001 ) ) {

					nodeVar12 = 0.000001;

				} else {

					nodeVar12 = nodeConst18.y;

				}


				if ( ( abs( nodeConst18.z ) < 0.000001 ) ) {

					nodeVar13 = 0.000001;

				} else {

					nodeVar13 = nodeConst18.z;

				}

				let nodeConst21 = vec3<f32>( nodeVar11, nodeVar12, nodeVar13 );
				let nodeConst22 = ( vec3<f32>( 1.0 ) / nodeConst21 );
				let nodeConst23 = ( ( object.nodeUniform4 - nodeConst19 ) * nodeConst22 );
				let nodeConst24 = ( ( nodeConst20 - nodeConst19 ) * nodeConst22 );
				let nodeConst25 = min( nodeConst23, nodeConst24 );
				let nodeConst26 = max( nodeConst23, nodeConst24 );
				let nodeConst27 = max( max( nodeConst25.x, nodeConst25.y ), max( nodeConst25.z, 0.0 ) );
				let nodeConst28 = min( min( nodeConst26.x, nodeConst26.y ), nodeConst26.z );
				nodeVar14 = max( nodeConst27, object.nodeUniform5 );

				if ( ( object.nodeUniform8 > 0.0 ) ) {

					nodeVar15 = object.nodeUniform8;

				} else {

					nodeVar15 = 10000000000.0;

				}

				let nodeConst29 = min( nodeConst28, nodeVar15 );
				let nodeConst30 = ( nodeConst18 * nodeConst18 );

				if ( ( nodeConst28 > nodeVar14 ) ) {


					for ( var s : i32 = 0; s < 128; s ++ ) {


						if ( ( ( nodeVar14 >= nodeConst29 ) || ( nodeVar9 >= 0.98 ) ) ) {

							break;
							

						}

						let nodeConst31 = max( ( ( nodeVar14 * 2.0 ) * tan( radians( ( object.nodeUniform9 * 0.5 ) ) ) ), object.nodeUniform5 );
						let nodeConst32 = clamp( log2( ( nodeConst31 / object.nodeUniform5 ) ), 0.0, object.nodeUniform10 );
						let nodeConst33 = ( nodeConst19 + ( nodeConst18 * vec3<f32>( nodeVar14 ) ) );
						let nodeConst34 = ( ( nodeConst33 - object.nodeUniform4 ) / object.nodeUniform7 );
						nodeVar16 = textureSampleLevel( nodeUniform11, nodeUniform11_sampler, nodeConst34, nodeConst32 );
						let nodeConst35 = nodeVar16;
						let nodeConst36 = ( 1.0 - pow( ( 1.0 - clamp( dot( nodeConst35.xyz, nodeConst30 ), 0.0, 1.0 ) ), object.nodeUniform12 ) );
						let nodeConst37 = ( nodeConst36 * ( 1.0 - nodeVar9 ) );
						nodeVar17 = textureSampleLevel( nodeUniform13, nodeUniform13_sampler, nodeConst34, nodeConst32 );
						let nodeConst38 = nodeVar17;
						nodeVar8 = ( nodeVar8 + ( ( nodeConst38.xyz / vec3<f32>( max( nodeConst38.w, 0.0001 ) ) ) * vec3<f32>( nodeConst37 ) ) );
						nodeVar9 = ( nodeVar9 + nodeConst37 );
						nodeVar14 = ( nodeVar14 + ( ( object.nodeUniform5 * exp2( nodeConst32 ) ) * object.nodeUniform12 ) );

					}

					

				}

				nodeVar7 = ( nodeVar7 + nodeVar8 );

			}

			nodeVar0 = vec4<f32>( ( nodeConst9.xyz + ( ( nodeConst6.xyz * ( nodeVar7 / vec3<f32>( 8.0 ) ) ) * vec3<f32>( nodeConst9.w ) ) ), nodeConst9.w );
			

		}

		textureStore( nodeUniform14, nodeConst0, nodeVar0 );
		

	}


	

}
