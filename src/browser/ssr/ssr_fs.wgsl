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
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;

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
	nodeVar27 = vec2<f32>( 0.0, 0.0 );
	nodeVar28 = 0.0;

	for ( var i : i32 = 1; i < nodeConst1; i ++ ) {

		nodeVar29 = ( f32( i ) / f32( nodeConst1 ) );
		nodeVar30 = ( nodeVar13 + ( nodeVar21 * vec2<f32>( ( nodeVar29 * f32( nodeConst1 ) ) ) ) );

		if ( ( ( ( ( nodeVar30.x < 0.0 ) || ( nodeVar30.x > object.nodeUniform7.x ) ) || ( nodeVar30.y < 0.0 ) ) || ( nodeVar30.y > object.nodeUniform7.y ) ) ) {

			break;
			

		}

		nodeVar31 = ( nodeVar30 * nodeVar22 );
		nodeVar32 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar31 ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar33 = nodeVar32;
		nodeVar34 = ( ( object.nodeUniform6 * object.nodeUniform10 ) / ( ( ( object.nodeUniform10 - object.nodeUniform6 ) * nodeVar33 ) - object.nodeUniform10 ) );
		nodeVar35 = ( 1.0 / ( nodeConst2 + ( nodeVar29 * ( nodeConst3 - nodeConst2 ) ) ) );

		if ( ( nodeVar35 <= nodeVar34 ) ) {

			nodeVar36 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar31.x, ( 1.0 - nodeVar31.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar33 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar31.x, ( 1.0 - nodeVar31.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar33 ), 1.0 ) ).w ) );
			nodeVar37 = ( length( cross( ( nodeVar36 - nodeVar4 ), ( nodeVar36 - nodeVar12 ) ) ) / length( ( nodeVar12 - nodeVar4 ) ) );
			nodeVar38 = ( nodeVar31 + nodeVar23 );
			nodeVar39 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar38.x, ( 1.0 - nodeVar38.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar33 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar38.x, ( 1.0 - nodeVar38.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar33 ), 1.0 ) ).w ) );
			nodeVar40 = ( ( nodeVar39.x - nodeVar36.x ) * 3.0 );
			nodeVar41 = max( nodeVar40, object.nodeUniform11 );

			if ( ( nodeVar37 <= nodeVar41 ) ) {

				nodeVar42 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVar31 );
				nodeVar43 = normalize( ( ( nodeVar42 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );

				if ( ( dot( nodeVar10, nodeVar43 ) >= 0.0 ) ) {

					continue;
					

				}

				nodeVar44 = ( - ( ( ( nodeVar7.x * nodeVar4.x ) + ( nodeVar7.y * nodeVar4.y ) ) + ( nodeVar7.z * nodeVar4.z ) ) );
				nodeVar45 = ( ( ( ( nodeVar7.x * nodeVar36.x ) + ( nodeVar7.y * nodeVar36.y ) ) + ( nodeVar7.z * nodeVar36.z ) ) + nodeVar44 );

				if ( ( nodeVar45 > object.nodeUniform5 ) ) {

					break;
					

				}

				nodeVar26 = true;
				nodeVar27 = nodeVar31;
				nodeVar28 = nodeVar33;
				break;
				

			}

			

		}


	}


	if ( nodeVar26 ) {

		nodeVar46 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar27.x, ( 1.0 - nodeVar27.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar28 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar27.x, ( 1.0 - nodeVar27.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar28 ), 1.0 ) ).w ) );
		nodeVar47 = ( - ( ( ( nodeVar7.x * nodeVar4.x ) + ( nodeVar7.y * nodeVar4.y ) ) + ( nodeVar7.z * nodeVar4.z ) ) );
		nodeVar48 = ( ( ( ( nodeVar7.x * nodeVar46.x ) + ( nodeVar7.y * nodeVar46.y ) ) + ( nodeVar7.z * nodeVar46.z ) ) + nodeVar47 );

		if ( ( nodeVar48 <= object.nodeUniform5 ) ) {

			nodeVar49 = ( object.nodeUniform2 * vec4<f32>( nodeVar46, 1.0 ) ).xyz;
			nodeVar50 = ( distance( nodeVar5, nodeVar49 ) * 1.0 );
			nodeVar51 = textureSample( nodeUniform12, nodeUniform12_sampler, nodeVar27 );
			nodeVar52 = nodeVar51;
			nodeVar53 = nodeVar52.xyz;
			nodeVar52.x = nodeVar53[ 0 ];
			nodeVar52.y = nodeVar53[ 1 ];
			nodeVar52.z = nodeVar53[ 2 ];
			nodeVar54 = ( 1.0 - ( nodeVar48 / object.nodeUniform5 ) );
			nodeVar55 = ( nodeVar54 * nodeVar54 );
			nodeVar56 = ( ( dot( nodeVar8, nodeVar10 ) + 1.0 ) / 2.0 );
			nodeVar25 = 1.0;
			nodeVar24 = vec4<f32>( ( ( nodeVar52.xyz * vec3<f32>( nodeVar9.x ) ) * vec3<f32>( ( nodeVar55 * nodeVar56 ) ) ), nodeVar50 );
			

		}

		

	}


	if ( ( nodeVar25 == 0.0 ) ) {

		

	}

	nodeVar57 = max( dot( nodeVar24.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ), 0.0001 );
	nodeVar58 = ( nodeVar24.xyz * vec3<f32>( min( ( object.nodeUniform13 / nodeVar57 ), 1.0 ) ) );
	nodeVar24.x = nodeVar58[ 0 ];
	nodeVar24.y = nodeVar58[ 1 ];
	nodeVar24.z = nodeVar58[ 2 ];
	nodeVar59 = ( nodeVar24.xyz * vec3<f32>( object.nodeUniform14 ) );
	nodeVar24.x = nodeVar59[ 0 ];
	nodeVar24.y = nodeVar59[ 1 ];
	nodeVar24.z = nodeVar59[ 2 ];

	// result

	output.color = max( nodeVar24, vec4<f32>( 0.0 ) );

	return output;

}
