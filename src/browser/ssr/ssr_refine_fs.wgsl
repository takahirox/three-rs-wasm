// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_depth_2d;
@binding( 2 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform4_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform4 : texture_2d<f32>;
@binding( 6 ) @group( 0 ) var nodeUniform12_sampler : sampler;
@binding( 7 ) @group( 0 ) var nodeUniform12 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : vec2<f32>,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : f32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<u32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : bool;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec4<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}



@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = nodeVarying0;
	nodeVar2 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar1 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar0 ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar3 = nodeVar1;

	if ( ( nodeVar3 >= 1.0 ) ) {

		discard;
		

	}

	nodeVar4 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar3 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar3 ), 1.0 ) ).w ) );
	nodeVar5 = ( object.nodeUniform2 * vec4<f32>( nodeVar4, 1.0 ) ).xyz;
	nodeVar6 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	nodeVar7 = normalize( ( ( nodeVar6 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );
	nodeVar8 = normalize( nodeVar4 );
	nodeVar9 = textureSample( nodeUniform4, nodeUniform4_sampler, nodeVarying0 );

	if ( ( nodeVar9.x <= 0.0 ) ) {

		discard;
		

	}

	nodeVar10 = normalize( reflect( nodeVar8, nodeVar7 ) );
	nodeVar11 = ( object.nodeUniform5 / dot( ( - nodeVar8 ), nodeVar7 ) );
	nodeVar12 = ( nodeVar4 + ( nodeVar10 * vec3<f32>( nodeVar11 ) ) );

	if ( ( nodeVar12.z > ( - object.nodeUniform6 ) ) ) {

		nodeVar12 = ( nodeVar4 + ( nodeVar10 * vec3<f32>( ( ( ( - object.nodeUniform6 ) - nodeVar4.z ) / nodeVar10.z ) ) ) );
		

	}

	nodeVar13 = ( nodeVar0 * object.nodeUniform7 );
	nodeVar14 = ( object.nodeUniform8 * vec4<f32>( nodeVar12, 1.0 ) );
	nodeVar15 = ( ( ( nodeVar14.xy / vec2<f32>( nodeVar14.w ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) );
	nodeVar16 = ( vec2<f32>( nodeVar15.x, ( 1.0 - nodeVar15.y ) ) * object.nodeUniform7 );
	nodeVar17 = ( nodeVar16.x - nodeVar13.x );
	nodeVar18 = ( nodeVar16.y - nodeVar13.y );
	let nodeConst0 = max( i32( trunc( ( max( abs( nodeVar17 ), abs( nodeVar18 ) ) * clamp( object.nodeUniform9, 0.0, 1.0 ) ) ) ), 1 );
	let nodeConst1 = nodeConst0;
	nodeVar19 = ( nodeVar17 / f32( nodeConst1 ) );
	nodeVar20 = ( nodeVar18 / f32( nodeConst1 ) );
	nodeVar21 = vec2<f32>( nodeVar19, nodeVar20 );
	nodeVar22 = ( vec2<f32>( 1.0, 1.0 ) / object.nodeUniform7 );
	nodeVar23 = vec2<f32>( nodeVar22.x, 0.0 );
	nodeVar24 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );
	nodeVar25 = 0.0;
	let nodeConst2 = ( 1.0 / nodeVar4.z );
	let nodeConst3 = ( 1.0 / nodeVar12.z );
	nodeVar26 = false;
	nodeVar27 = 0.0;
	nodeVar28 = 0.0;
	nodeVar29 = vec2<f32>( 0.0, 0.0 );
	nodeVar30 = 0.0;

	for ( var i : i32 = 1; i < nodeConst1; i ++ ) {

		nodeVar31 = ( f32( i ) / f32( nodeConst1 ) );
		nodeVar32 = ( nodeVar13 + ( nodeVar21 * vec2<f32>( ( nodeVar31 * f32( nodeConst1 ) ) ) ) );

		if ( ( ( ( ( nodeVar32.x < 0.0 ) || ( nodeVar32.x > object.nodeUniform7.x ) ) || ( nodeVar32.y < 0.0 ) ) || ( nodeVar32.y > object.nodeUniform7.y ) ) ) {

			break;
			

		}

		nodeVar33 = ( nodeVar32 * nodeVar22 );
		nodeVar34 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar33 ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar35 = nodeVar34;
		nodeVar36 = ( ( object.nodeUniform6 * object.nodeUniform10 ) / ( ( ( object.nodeUniform10 - object.nodeUniform6 ) * nodeVar35 ) - object.nodeUniform10 ) );
		nodeVar37 = ( 1.0 / ( nodeConst2 + ( nodeVar31 * ( nodeConst3 - nodeConst2 ) ) ) );

		if ( ( nodeVar37 <= nodeVar36 ) ) {

			nodeVar38 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar33.x, ( 1.0 - nodeVar33.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar35 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar33.x, ( 1.0 - nodeVar33.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar35 ), 1.0 ) ).w ) );
			nodeVar39 = ( length( cross( ( nodeVar38 - nodeVar4 ), ( nodeVar38 - nodeVar12 ) ) ) / length( ( nodeVar12 - nodeVar4 ) ) );
			nodeVar40 = ( nodeVar33 + nodeVar23 );
			nodeVar41 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar40.x, ( 1.0 - nodeVar40.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar35 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar40.x, ( 1.0 - nodeVar40.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar35 ), 1.0 ) ).w ) );
			nodeVar42 = ( ( nodeVar41.x - nodeVar38.x ) * 3.0 );
			nodeVar43 = max( nodeVar42, object.nodeUniform11 );

			if ( ( nodeVar39 <= nodeVar43 ) ) {

				nodeVar44 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVar33 );
				nodeVar45 = normalize( ( ( nodeVar44 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );

				if ( ( dot( nodeVar10, nodeVar45 ) >= 0.0 ) ) {

					continue;
					

				}

				nodeVar46 = ( - ( ( ( nodeVar7.x * nodeVar4.x ) + ( nodeVar7.y * nodeVar4.y ) ) + ( nodeVar7.z * nodeVar4.z ) ) );
				nodeVar47 = ( ( ( ( nodeVar7.x * nodeVar38.x ) + ( nodeVar7.y * nodeVar38.y ) ) + ( nodeVar7.z * nodeVar38.z ) ) + nodeVar46 );

				if ( ( nodeVar47 > object.nodeUniform5 ) ) {

					break;
					

				}

				nodeVar26 = true;
				nodeVar29 = nodeVar33;
				nodeVar30 = nodeVar35;
				nodeVar27 = ( ( f32( i ) - 1.0 ) / f32( nodeConst1 ) );
				nodeVar28 = nodeVar31;
				break;
				

			}

			

		}


	}


	if ( nodeVar26 ) {


		for ( var i : i32 = 0; i < 8; i ++ ) {

			nodeVar48 = ( ( nodeVar27 + nodeVar28 ) * 0.5 );
			nodeVar49 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( ( nodeVar13 + ( nodeVar21 * vec2<f32>( ( nodeVar48 * f32( nodeConst1 ) ) ) ) ) * nodeVar22 ) ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );

			if ( ( ( 1.0 / ( nodeConst2 + ( nodeVar48 * ( nodeConst3 - nodeConst2 ) ) ) ) <= ( ( object.nodeUniform6 * object.nodeUniform10 ) / ( ( ( object.nodeUniform10 - object.nodeUniform6 ) * nodeVar49 ) - object.nodeUniform10 ) ) ) ) {

				nodeVar28 = nodeVar48;
				

			} else {

				nodeVar27 = nodeVar48;
				

			}


		}

		nodeVar29 = ( ( nodeVar13 + ( nodeVar21 * vec2<f32>( ( nodeVar28 * f32( nodeConst1 ) ) ) ) ) * nodeVar22 );
		nodeVar50 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar29 ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar30 = nodeVar50;
		nodeVar51 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar29.x, ( 1.0 - nodeVar29.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar30 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar29.x, ( 1.0 - nodeVar29.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar30 ), 1.0 ) ).w ) );
		nodeVar52 = ( - ( ( ( nodeVar7.x * nodeVar4.x ) + ( nodeVar7.y * nodeVar4.y ) ) + ( nodeVar7.z * nodeVar4.z ) ) );
		nodeVar53 = ( ( ( ( nodeVar7.x * nodeVar51.x ) + ( nodeVar7.y * nodeVar51.y ) ) + ( nodeVar7.z * nodeVar51.z ) ) + nodeVar52 );

		if ( ( nodeVar53 <= object.nodeUniform5 ) ) {

			nodeVar54 = ( object.nodeUniform2 * vec4<f32>( nodeVar51, 1.0 ) ).xyz;
			nodeVar55 = ( distance( nodeVar5, nodeVar54 ) * 1.0 );
			nodeVar56 = textureSample( nodeUniform12, nodeUniform12_sampler, nodeVar29 );
			nodeVar57 = nodeVar56;
			nodeVar58 = nodeVar57.xyz;
			nodeVar57.x = nodeVar58[ 0 ];
			nodeVar57.y = nodeVar58[ 1 ];
			nodeVar57.z = nodeVar58[ 2 ];
			nodeVar59 = ( 1.0 - ( nodeVar53 / object.nodeUniform5 ) );
			nodeVar60 = ( nodeVar59 * nodeVar59 );
			nodeVar61 = ( ( dot( nodeVar8, nodeVar10 ) + 1.0 ) / 2.0 );
			nodeVar25 = 1.0;
			nodeVar24 = vec4<f32>( ( ( nodeVar57.xyz * vec3<f32>( nodeVar9.x ) ) * vec3<f32>( ( nodeVar60 * nodeVar61 ) ) ), nodeVar55 );
			

		}

		

	}


	if ( ( nodeVar25 == 0.0 ) ) {

		

	}

	nodeVar62 = max( dot( nodeVar24.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ), 0.0001 );
	nodeVar63 = ( nodeVar24.xyz * vec3<f32>( min( ( object.nodeUniform13 / nodeVar62 ), 1.0 ) ) );
	nodeVar24.x = nodeVar63[ 0 ];
	nodeVar24.y = nodeVar63[ 1 ];
	nodeVar24.z = nodeVar63[ 2 ];
	nodeVar64 = ( nodeVar24.xyz * vec3<f32>( object.nodeUniform14 ) );
	nodeVar24.x = nodeVar64[ 0 ];
	nodeVar24.y = nodeVar64[ 1 ];
	nodeVar24.z = nodeVar64[ 2 ];

	// result

	output.color = max( nodeVar24, vec4<f32>( 0.0 ) );

	return output;

}
