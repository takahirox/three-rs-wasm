// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec2<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec4<f32>;

// codes
fn roughnessToMip ( roughness : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = 0.0;

	if ( ( roughness >= 0.8 ) ) {

		nodeVar0 = ( ( ( ( 1.0 - roughness ) * ( -1.0 - -2.0 ) ) / ( 1.0 - 0.8 ) ) + -2.0 );
		

	} else {


		if ( ( roughness >= 0.4 ) ) {

			nodeVar0 = ( ( ( ( 0.8 - roughness ) * ( 2.0 - -1.0 ) ) / ( 0.8 - 0.4 ) ) + -1.0 );
			

		} else {


			if ( ( roughness >= 0.305 ) ) {

				nodeVar0 = ( ( ( ( 0.4 - roughness ) * ( 3.0 - 2.0 ) ) / ( 0.4 - 0.305 ) ) + 2.0 );
				

			} else {


				if ( ( roughness >= 0.21 ) ) {

					nodeVar0 = ( ( ( ( 0.305 - roughness ) * ( 4.0 - 3.0 ) ) / ( 0.305 - 0.21 ) ) + 3.0 );
					

				} else {

					nodeVar0 = ( -2.0 * log2( ( 1.16 * roughness ) ) );
					

				}

				

			}

			

		}

		

	}


	return nodeVar0;

}


fn getFace ( direction : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;

	nodeVar0 = abs( direction );
	nodeVar1 = -1.0;

	if ( ( nodeVar0.x > nodeVar0.z ) ) {


		if ( ( nodeVar0.x > nodeVar0.y ) ) {


			if ( ( direction.x > 0.0 ) ) {

				nodeVar2 = 0.0;

			} else {

				nodeVar2 = 3.0;

			}

			nodeVar1 = nodeVar2;
			

		} else {


			if ( ( direction.y > 0.0 ) ) {

				nodeVar3 = 1.0;

			} else {

				nodeVar3 = 4.0;

			}

			nodeVar1 = nodeVar3;
			

		}

		

	} else {


		if ( ( nodeVar0.z > nodeVar0.y ) ) {


			if ( ( direction.z > 0.0 ) ) {

				nodeVar4 = 2.0;

			} else {

				nodeVar4 = 5.0;

			}

			nodeVar1 = nodeVar4;
			

		} else {


			if ( ( direction.y > 0.0 ) ) {

				nodeVar5 = 1.0;

			} else {

				nodeVar5 = 4.0;

			}

			nodeVar1 = nodeVar5;
			

		}

		

	}


	return nodeVar1;

}


fn getUV ( direction : vec3<f32>, face : f32 ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	nodeVar0 = vec2<f32>( 0.0, 0.0 );

	if ( ( face == 0.0 ) ) {

		nodeVar0 = ( vec2<f32>( direction.z, direction.y ) / vec2<f32>( abs( direction.x ) ) );
		

	} else {


		if ( ( face == 1.0 ) ) {

			nodeVar0 = ( vec2<f32>( ( - direction.x ), ( - direction.z ) ) / vec2<f32>( abs( direction.y ) ) );
			

		} else {


			if ( ( face == 2.0 ) ) {

				nodeVar0 = ( vec2<f32>( ( - direction.x ), direction.y ) / vec2<f32>( abs( direction.z ) ) );
				

			} else {


				if ( ( face == 3.0 ) ) {

					nodeVar0 = ( vec2<f32>( ( - direction.z ), direction.y ) / vec2<f32>( abs( direction.x ) ) );
					

				} else {


					if ( ( face == 4.0 ) ) {

						nodeVar0 = ( vec2<f32>( ( - direction.x ), direction.z ) / vec2<f32>( abs( direction.y ) ) );
						

					} else {

						nodeVar0 = ( vec2<f32>( direction.x, direction.y ) / vec2<f32>( abs( direction.z ) ) );
						

					}

					

				}

				

			}

			

		}

		

	}


	return ( vec2<f32>( 0.5 ) * ( nodeVar0 + vec2<f32>( 1.0 ) ) );

}




@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = vec2<f32>( dpdx( vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) ).x ), - dpdy( vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) ).x ) );
	nodeVar1 = vec2<f32>( dpdx( vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) ).y ), - dpdy( vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) ).y ) );
	nodeVar2 = vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) );
	nodeVar3 = textureSample( nodeUniform0, nodeUniform0_sampler, vec2<f32>( nodeVar2[ 0u ], ( 1.0 - nodeVar2[ 1u ] ) ) );
	nodeVar4 = ( ( vec2<f32>( dpdx( nodeVar3.x ), - dpdy( nodeVar3.x ) ) * vec2<f32>( 1.0 ) ) * vec2<f32>( 0.0625 ) );
	nodeVar5 = cross( vec3<f32>( nodeVar0.x, nodeVar1.x, nodeVar4.x ), vec3<f32>( nodeVar0.y, nodeVar1.y, nodeVar4.y ) );
	nodeVar6 = mix( nodeVar5, vec3<f32>( 0.0, 0.0, 1.0 ), f32( ( dot( nodeVar5, nodeVar5 ) < 1e-12 ) ) );
	DiffuseColor = vec4<f32>( ( ( normalize( mix( nodeVar6, ( nodeVar6 * vec3<f32>( -1.0 ) ), f32( ( nodeVar6.z < 0.0 ) ) ) ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar7 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.2, 0.0525 ) + max( max( nodeVar7.x, nodeVar7.y ), nodeVar7.z ) ), 1.0 );
	IOR = object.nodeUniform5;
	nodeVar8 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar8 * nodeVar8 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform6 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform6, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar9 = dot( normalView, positionViewDirection );
	nodeVar10 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( Roughness, clamp( nodeVar9, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar10;
	nodeVar11 = ( dfg.x + dfg.y );
	nodeVar12 = ( 1.0 / nodeVar11 );
	nodeVar13 = nodeVar12;
	nodeVar14 = ( nodeVar13 - 1.0 );
	nodeVar15 = ( SpecularColorBlended * vec3<f32>( nodeVar14 ) );
	nodeVar16 = ( nodeVar15 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar16;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar17 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform11 );
	nodeVar18 = floor( nodeVar17 );
	nodeVar19 = nodeVar18;
	nodeVar20 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar21 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar20.x, ( - nodeVar20.y ), nodeVar20.z ), 1.0 ) ).xyz );
	nodeVar22 = max( ( 4.0 - nodeVar19 ), 0.0 );
	nodeVar19 = max( nodeVar19, 4.0 );
	nodeVar23 = exp2( nodeVar19 );
	nodeVar24 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar20.x, ( - nodeVar20.y ), nodeVar20.z ), 1.0 ) ).xyz, nodeVar21 ) * vec2<f32>( ( nodeVar23 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar21 > 2.0 ) ) {

		nodeVar24.y = ( nodeVar24.y + nodeVar23 );
		nodeVar21 = ( nodeVar21 - 3.0 );
		

	}

	nodeVar24.x = ( nodeVar24.x + ( nodeVar21 * nodeVar23 ) );
	nodeVar24.x = ( nodeVar24.x + ( nodeVar22 * ( 3.0 * 16.0 ) ) );
	nodeVar24.y = ( nodeVar24.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar23 ) ) );
	nodeVar24.x = ( nodeVar24.x * object.nodeUniform14 );
	nodeVar24.y = ( nodeVar24.y * object.nodeUniform15 );
	nodeVar25 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar24, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar26 = nodeVar25.xyz;
	nodeVar27 = fract( nodeVar17 );

	if ( ( nodeVar27 != 0.0 ) ) {

		nodeVar28 = ( nodeVar18 + 1.0 );
		nodeVar29 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar20.x, ( - nodeVar20.y ), nodeVar20.z ), 1.0 ) ).xyz );
		nodeVar30 = max( ( 4.0 - nodeVar28 ), 0.0 );
		nodeVar28 = max( nodeVar28, 4.0 );
		nodeVar31 = exp2( nodeVar28 );
		nodeVar32 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar20.x, ( - nodeVar20.y ), nodeVar20.z ), 1.0 ) ).xyz, nodeVar29 ) * vec2<f32>( ( nodeVar31 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar29 > 2.0 ) ) {

			nodeVar32.y = ( nodeVar32.y + nodeVar31 );
			nodeVar29 = ( nodeVar29 - 3.0 );
			

		}

		nodeVar32.x = ( nodeVar32.x + ( nodeVar29 * nodeVar31 ) );
		nodeVar32.x = ( nodeVar32.x + ( nodeVar30 * ( 3.0 * 16.0 ) ) );
		nodeVar32.y = ( nodeVar32.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar31 ) ) );
		nodeVar32.x = ( nodeVar32.x * object.nodeUniform14 );
		nodeVar32.y = ( nodeVar32.y * object.nodeUniform15 );
		nodeVar33 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar32, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar34 = nodeVar33.xyz;
		nodeVar26 = mix( nodeVar26, nodeVar34, nodeVar27 );
		

	}

	nodeVar35 = ( radiance + ( nodeVar26 * vec3<f32>( object.nodeUniform17 ) ) );
	radiance = nodeVar35;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar36 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform11 );
	nodeVar37 = floor( nodeVar36 );
	nodeVar38 = nodeVar37;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar39 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar40 = max( ( 4.0 - nodeVar38 ), 0.0 );
	nodeVar38 = max( nodeVar38, 4.0 );
	nodeVar41 = exp2( nodeVar38 );
	nodeVar42 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar39 ) * vec2<f32>( ( nodeVar41 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar39 > 2.0 ) ) {

		nodeVar42.y = ( nodeVar42.y + nodeVar41 );
		nodeVar39 = ( nodeVar39 - 3.0 );
		

	}

	nodeVar42.x = ( nodeVar42.x + ( nodeVar39 * nodeVar41 ) );
	nodeVar42.x = ( nodeVar42.x + ( nodeVar40 * ( 3.0 * 16.0 ) ) );
	nodeVar42.y = ( nodeVar42.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar41 ) ) );
	nodeVar42.x = ( nodeVar42.x * object.nodeUniform14 );
	nodeVar42.y = ( nodeVar42.y * object.nodeUniform15 );
	nodeVar43 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar42, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar44 = nodeVar43.xyz;
	nodeVar45 = fract( nodeVar36 );

	if ( ( nodeVar45 != 0.0 ) ) {

		nodeVar46 = ( nodeVar37 + 1.0 );
		nodeVar47 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar48 = max( ( 4.0 - nodeVar46 ), 0.0 );
		nodeVar46 = max( nodeVar46, 4.0 );
		nodeVar49 = exp2( nodeVar46 );
		nodeVar50 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar47 ) * vec2<f32>( ( nodeVar49 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar47 > 2.0 ) ) {

			nodeVar50.y = ( nodeVar50.y + nodeVar49 );
			nodeVar47 = ( nodeVar47 - 3.0 );
			

		}

		nodeVar50.x = ( nodeVar50.x + ( nodeVar47 * nodeVar49 ) );
		nodeVar50.x = ( nodeVar50.x + ( nodeVar48 * ( 3.0 * 16.0 ) ) );
		nodeVar50.y = ( nodeVar50.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar49 ) ) );
		nodeVar50.x = ( nodeVar50.x * object.nodeUniform14 );
		nodeVar50.y = ( nodeVar50.y * object.nodeUniform15 );
		nodeVar51 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar50, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar52 = nodeVar51.xyz;
		nodeVar44 = mix( nodeVar44, nodeVar52, nodeVar45 );
		

	}

	nodeVar53 = ( iblIrradiance + ( ( nodeVar44 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform17 ) ) );
	iblIrradiance = nodeVar53;
	nodeVar54 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar56 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar57 = ( SpecularF90 * dfg.y );
	nodeVar58 = ( nodeVar56 + vec3<f32>( nodeVar57 ) );
	nodeVar59 = ( nodeVar54 + nodeVar58 );
	nodeVar54 = nodeVar59;
	nodeVar60 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar61 = nodeVar60;
	nodeVar62 = ( nodeVar61 * vec3<f32>( 0.047619 ) );
	nodeVar63 = ( SpecularColor + nodeVar62 );
	nodeVar64 = ( nodeVar58 * nodeVar63 );
	nodeVar65 = ( dfg.x + dfg.y );
	nodeVar66 = ( 1.0 - nodeVar65 );
	nodeVar67 = nodeVar66;
	nodeVar68 = ( vec3<f32>( nodeVar67 ) * nodeVar63 );
	nodeVar69 = ( vec3<f32>( 1.0 ) - nodeVar68 );
	nodeVar70 = nodeVar69;
	nodeVar71 = ( nodeVar64 / nodeVar70 );
	nodeVar72 = ( nodeVar71 * vec3<f32>( nodeVar67 ) );
	nodeVar73 = ( nodeVar55 + nodeVar72 );
	nodeVar55 = nodeVar73;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar74 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar75 = ( irradiance * nodeVar74 );
	nodeVar76 = ( nodeVar54 + nodeVar55 );
	nodeVar77 = ( vec3<f32>( 1.0 ) - nodeVar76 );
	nodeVar78 = nodeVar77;
	nodeVar79 = ( nodeVar75 * nodeVar78 );
	nodeVar80 = nodeVar79;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar81 = ( indirectDiffuse + nodeVar80 );
	indirectDiffuse = nodeVar81;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar82 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar83 = ( SpecularF90 * dfg.y );
	nodeVar84 = ( nodeVar82 + vec3<f32>( nodeVar83 ) );
	nodeVar85 = ( singleScatteringDielectric + nodeVar84 );
	singleScatteringDielectric = nodeVar85;
	nodeVar86 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( nodeVar87 * vec3<f32>( 0.047619 ) );
	nodeVar89 = ( SpecularColor + nodeVar88 );
	nodeVar90 = ( nodeVar84 * nodeVar89 );
	nodeVar91 = ( dfg.x + dfg.y );
	nodeVar92 = ( 1.0 - nodeVar91 );
	nodeVar93 = nodeVar92;
	nodeVar94 = ( vec3<f32>( nodeVar93 ) * nodeVar89 );
	nodeVar95 = ( vec3<f32>( 1.0 ) - nodeVar94 );
	nodeVar96 = nodeVar95;
	nodeVar97 = ( nodeVar90 / nodeVar96 );
	nodeVar98 = ( nodeVar97 * vec3<f32>( nodeVar93 ) );
	nodeVar99 = ( multiScatteringDielectric + nodeVar98 );
	multiScatteringDielectric = nodeVar99;
	nodeVar100 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar101 = ( SpecularF90 * dfg.y );
	nodeVar102 = ( nodeVar100 + vec3<f32>( nodeVar101 ) );
	nodeVar103 = ( singleScatteringMetallic + nodeVar102 );
	singleScatteringMetallic = nodeVar103;
	nodeVar104 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( nodeVar105 * vec3<f32>( 0.047619 ) );
	nodeVar107 = ( DiffuseColor.xyz + nodeVar106 );
	nodeVar108 = ( nodeVar102 * nodeVar107 );
	nodeVar109 = ( dfg.x + dfg.y );
	nodeVar110 = ( 1.0 - nodeVar109 );
	nodeVar111 = nodeVar110;
	nodeVar112 = ( vec3<f32>( nodeVar111 ) * nodeVar107 );
	nodeVar113 = ( vec3<f32>( 1.0 ) - nodeVar112 );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar108 / nodeVar114 );
	nodeVar116 = ( nodeVar115 * vec3<f32>( nodeVar111 ) );
	nodeVar117 = ( multiScatteringMetallic + nodeVar116 );
	multiScatteringMetallic = nodeVar117;
	nodeVar118 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar119 = ( radiance * nodeVar118 );
	nodeVar120 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar121 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar122 = ( nodeVar120 * nodeVar121 );
	nodeVar123 = ( nodeVar119 + nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar126 = ( vec3<f32>( 1.0 ) - nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( DiffuseContribution * nodeVar127 );
	nodeVar129 = ( nodeVar128 * nodeVar121 );
	nodeVar130 = nodeVar129;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar131 = ( indirectSpecular + nodeVar124 );
	indirectSpecular = nodeVar131;
	nodeVar132 = ( indirectDiffuse + nodeVar130 );
	indirectDiffuse = nodeVar132;
	ambientOcclusion = 1.0;
	nodeVar133 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar133;
	nodeVar134 = dot( normalView, positionViewDirection );
	nodeVar135 = ( clamp( nodeVar134, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar136 = ( Roughness * -16.0 );
	nodeVar137 = ( 1.0 - nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( - nodeVar138 );
	nodeVar140 = exp2( nodeVar139 );
	nodeVar141 = pow( nodeVar135, nodeVar140 );
	nodeVar142 = ( 1.0 - nodeVar141 );
	nodeVar143 = nodeVar142;
	nodeVar144 = ( ambientOcclusion - nodeVar143 );
	nodeVar145 = ( indirectSpecular * vec3<f32>( clamp( nodeVar144, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar145;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar146 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar146;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar147 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar147;
	nodeVar148 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar148;
	nodeVar149 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar149;

	// result

	output.color = nodeVar149;

	return output;

}
