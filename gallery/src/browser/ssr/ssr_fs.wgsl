// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


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
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : bool;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : bool;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : bool;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec4<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : bool;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec4<f32>;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn22 ( viewPosition : vec3<f32>, projectionMatrix : mat4x4<f32> ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	let nodeConst0 = ( projectionMatrix * vec4<f32>( viewPosition, 1.0 ) );
	nodeVar0 = ( ( ( nodeConst0.xy / vec2<f32>( nodeConst0.w ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) );

	return vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) );

}


fn fn140 ( point : vec3<f32>, linePointA : vec3<f32>, linePointB : vec3<f32> ) -> f32 {

	


	return ( length( cross( ( point - linePointA ), ( point - linePointB ) ) ) / length( ( linePointB - linePointA ) ) );

}


fn fn141 ( point : vec3<f32>, planePoint : vec3<f32>, planeNormal : vec3<f32> ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = ( - ( ( ( planeNormal.x * planePoint.x ) + ( planeNormal.y * planePoint.y ) ) + ( planeNormal.z * planePoint.z ) ) );

	return ( ( ( ( planeNormal.x * point.x ) + ( planeNormal.y * point.y ) ) + ( planeNormal.z * point.z ) ) + nodeVar0 );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = vec2<f32>( textureDimensions( nodeUniform0, 0 ) );
	nodeVar0 = ( ( floor( ( nodeVarying0 * nodeConst0 ) ) + vec2<f32>( 0.5 ) ) / nodeConst0 );
	nodeVar2 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar1 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar0 ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar3 = nodeVar1;

	if ( ( nodeVar3 >= 1.0 ) ) {

		discard;
		

	}

	let nodeConst1 = ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar3 ), 1.0 ) );
	nodeVar4 = ( nodeConst1.xyz / vec3<f32>( nodeConst1.w ) );
	nodeVar5 = ( object.nodeUniform2 * vec4<f32>( nodeVar4, 1.0 ) ).xyz;
	nodeVar6 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	nodeVar7 = normalize( ( ( nodeVar6 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );
	nodeVar8 = normalize( nodeVar4 );
	nodeVar9 = textureSample( nodeUniform4, nodeUniform4_sampler, nodeVarying0 );
	let nodeConst2 = nodeVar9.x;

	if ( ( nodeConst2 <= 0.0 ) ) {

		discard;
		

	}

	nodeVar10 = normalize( reflect( nodeVar8, nodeVar7 ) );
	nodeVar11 = ( object.nodeUniform5 / max( dot( ( - nodeVar8 ), nodeVar7 ), 0.0001 ) );
	nodeVar12 = ( nodeVar4 + ( nodeVar10 * vec3<f32>( nodeVar11 ) ) );

	if ( ( nodeVar12.z > ( - object.nodeUniform6 ) ) ) {

		nodeVar12 = ( nodeVar4 + ( nodeVar10 * vec3<f32>( ( ( ( - object.nodeUniform6 ) - nodeVar4.z ) / nodeVar10.z ) ) ) );
		

	}

	nodeVar13 = ( nodeVar0 * object.nodeUniform7 );
	nodeVar14 = ( fn22( nodeVar12, object.nodeUniform8 ) * object.nodeUniform7 );
	nodeVar15 = ( nodeVar14 - nodeVar13 );

	if ( ( nodeVar15.x >= 0.0 ) ) {

		nodeVar16 = ( object.nodeUniform7.x - nodeVar13.x );

	} else {

		nodeVar16 = nodeVar13.x;

	}


	if ( ( nodeVar15.y >= 0.0 ) ) {

		nodeVar17 = ( object.nodeUniform7.y - nodeVar13.y );

	} else {

		nodeVar17 = nodeVar13.y;

	}

	let nodeConst3 = ( vec2<f32>( nodeVar16, nodeVar17 ) / max( abs( nodeVar15 ), vec2<f32>( 0.000001 ) ) );
	let nodeConst4 = min( min( nodeConst3.x, nodeConst3.y ), 1.0 );

	if ( ( nodeConst4 < 1.0 ) ) {

		nodeVar14 = ( nodeVar13 + ( nodeVar15 * vec2<f32>( nodeConst4 ) ) );
		nodeVar12 = ( mix( ( nodeVar4 / vec3<f32>( nodeVar4.z ) ), ( nodeVar12 / vec3<f32>( nodeVar12.z ) ), nodeConst4 ) / vec3<f32>( mix( ( 1.0 / nodeVar4.z ), ( 1.0 / nodeVar12.z ), nodeConst4 ) ) );
		

	}

	nodeVar18 = ( nodeVar14.x - nodeVar13.x );
	nodeVar19 = ( nodeVar14.y - nodeVar13.y );
	let nodeConst5 = max( trunc( ( max( abs( nodeVar18 ), abs( nodeVar19 ) ) * clamp( object.nodeUniform9, 0.0, 1.0 ) ) ), 1.0 );
	let nodeConst6 = i32( nodeConst5 );
	nodeVar20 = ( nodeVar18 / f32( nodeConst6 ) );
	nodeVar21 = ( nodeVar19 / f32( nodeConst6 ) );
	nodeVar22 = vec2<f32>( nodeVar20, nodeVar21 );
	nodeVar23 = ( vec2<f32>( 1.0, 1.0 ) / object.nodeUniform7 );
	nodeVar24 = vec2<f32>( nodeVar23.x, 0.0 );
	nodeVar25 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );
	nodeVar26 = 0.0;
	let nodeConst7 = ( 1.0 / nodeVar4.z );
	let nodeConst8 = ( 1.0 / nodeVar12.z );
	nodeVar27 = false;
	nodeVar28 = vec2<f32>( 0.0, 0.0 );
	nodeVar29 = 0.0;
	nodeVar30 = 0.0;
	nodeVar31 = true;

	for ( var i : i32 = 1; i < nodeConst6; i ++ ) {

		nodeVar32 = ( f32( i ) / f32( nodeConst6 ) );

		if ( ( nodeVar32 > 1.0 ) ) {

			break;
			

		}

		nodeVar33 = ( nodeVar13 + ( nodeVar22 * vec2<f32>( ( nodeVar32 * f32( nodeConst6 ) ) ) ) );

		if ( ( ( ( ( nodeVar33.x < 0.0 ) || ( nodeVar33.x > object.nodeUniform7.x ) ) || ( nodeVar33.y < 0.0 ) ) || ( nodeVar33.y > object.nodeUniform7.y ) ) ) {

			break;
			

		}

		nodeVar34 = ( nodeVar33 * nodeVar23 );
		nodeVar35 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar34 ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar36 = nodeVar35;
		nodeVar37 = ( ( object.nodeUniform6 * object.nodeUniform10 ) / ( ( ( object.nodeUniform10 - object.nodeUniform6 ) * nodeVar36 ) - object.nodeUniform10 ) );
		nodeVar38 = ( 1.0 / ( nodeConst7 + ( nodeVar32 * ( nodeConst8 - nodeConst7 ) ) ) );
		nodeVar39 = nodeVar30;
		nodeVar40 = nodeVar31;
		nodeVar30 = nodeVar32;
		nodeVar31 = ( nodeVar38 > nodeVar37 );

		if ( ( ( ! ( nodeVar36 >= 1.0 ) ) && ( nodeVar38 <= nodeVar37 ) ) ) {

			let nodeConst9 = ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar34.x, ( 1.0 - nodeVar34.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) );
			nodeVar41 = ( nodeConst9.xyz / vec3<f32>( nodeConst9.w ) );
			nodeVar42 = fn140( nodeVar41, nodeVar4, nodeVar12 );
			nodeVar43 = ( nodeVar34 + nodeVar24 );
			let nodeConst10 = ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar43.x, ( 1.0 - nodeVar43.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) );
			nodeVar44 = ( nodeConst10.xyz / vec3<f32>( nodeConst10.w ) );
			nodeVar45 = ( ( nodeVar44.x - nodeVar41.x ) * 3.0 );
			nodeVar46 = max( nodeVar45, object.nodeUniform11 );

			if ( ( nodeVar42 <= nodeVar46 ) ) {

				nodeVar47 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVar34 );
				nodeVar48 = normalize( ( ( nodeVar47 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );

				if ( ( dot( nodeVar10, nodeVar48 ) >= 0.0 ) ) {

					continue;
					

				}

				nodeVar49 = fn141( nodeVar41, nodeVar4, nodeVar7 );

				if ( ( nodeVar49 > object.nodeUniform5 ) ) {

					break;
					

				}

				nodeVar27 = true;
				nodeVar28 = nodeVar34;
				nodeVar29 = nodeVar36;
				break;
				

			}

			

		}


	}


	if ( nodeVar27 ) {

		let nodeConst11 = ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar28.x, ( 1.0 - nodeVar28.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar29 ), 1.0 ) );
		nodeVar50 = ( nodeConst11.xyz / vec3<f32>( nodeConst11.w ) );
		nodeVar51 = fn141( nodeVar50, nodeVar4, nodeVar7 );
		nodeVar52 = ( nodeVar51 <= object.nodeUniform5 );

		if ( nodeVar52 ) {

			nodeVar53 = ( object.nodeUniform2 * vec4<f32>( nodeVar50, 1.0 ) ).xyz;
			nodeVar54 = ( distance( nodeVar5, nodeVar53 ) * 1.0 );
			nodeVar55 = textureSample( nodeUniform12, nodeUniform12_sampler, nodeVar28 );
			nodeVar56 = nodeVar55;
			nodeVar57 = nodeVar56.xyz;
			nodeVar56.x = nodeVar57[ 0 ];
			nodeVar56.y = nodeVar57[ 1 ];
			nodeVar56.z = nodeVar57[ 2 ];
			nodeVar58 = ( 1.0 - ( nodeVar51 / object.nodeUniform5 ) );
			nodeVar59 = ( nodeVar58 * nodeVar58 );
			nodeVar60 = ( ( dot( nodeVar8, nodeVar10 ) + 1.0 ) / 2.0 );
			nodeVar26 = 1.0;
			nodeVar25 = vec4<f32>( ( ( nodeVar56.xyz * vec3<f32>( nodeConst2 ) ) * vec3<f32>( ( nodeVar59 * nodeVar60 ) ) ), nodeVar54 );
			

		}

		

	}


	if ( ( nodeVar26 == 0.0 ) ) {

		

	}

	nodeVar61 = max( dot( nodeVar25.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ), 0.0001 );
	nodeVar62 = ( nodeVar25.xyz * vec3<f32>( min( ( object.nodeUniform13 / nodeVar61 ), 1.0 ) ) );
	nodeVar25.x = nodeVar62[ 0 ];
	nodeVar25.y = nodeVar62[ 1 ];
	nodeVar25.z = nodeVar62[ 2 ];
	nodeVar63 = ( nodeVar25.xyz * vec3<f32>( object.nodeUniform14 ) );
	nodeVar25.x = nodeVar63[ 0 ];
	nodeVar25.y = nodeVar63[ 1 ];
	nodeVar25.z = nodeVar63[ 2 ];

	// result

	output.color = max( nodeVar25, vec4<f32>( 0.0 ) );

	return output;

}
