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
@binding( 3 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform16 : f32
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
var<private> nodeVar1 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec4<f32>;

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

	DiffuseColor = vec4<f32>( vec3<f32>( 0.2, 0.6, 0.8 ), 1.0 );
	nodeVar0 = vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) );
	nodeVar1 = textureSample( nodeUniform0, nodeUniform0_sampler, vec2<f32>( nodeVar0[ 0u ], ( 1.0 - nodeVar0[ 1u ] ) ) );
	DiffuseColor.w = ( DiffuseColor.w * nodeVar1.xyz[ 0u ] );
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar2 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.2, 0.0525 ) + max( max( nodeVar2.x, nodeVar2.y ), nodeVar2.z ) ), 1.0 );
	IOR = object.nodeUniform4;
	nodeVar3 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar3 * nodeVar3 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform5 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform5, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = dot( normalView, positionViewDirection );
	nodeVar5 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar4, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar5;
	nodeVar6 = ( dfg.x + dfg.y );
	nodeVar7 = ( 1.0 / nodeVar6 );
	nodeVar8 = nodeVar7;
	nodeVar9 = ( nodeVar8 - 1.0 );
	nodeVar10 = ( SpecularColorBlended * vec3<f32>( nodeVar9 ) );
	nodeVar11 = ( nodeVar10 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar11;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar12 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform10 );
	nodeVar13 = floor( nodeVar12 );
	nodeVar14 = nodeVar13;
	nodeVar15 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar16 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar15.x, ( - nodeVar15.y ), nodeVar15.z ), 1.0 ) ).xyz );
	nodeVar17 = max( ( 4.0 - nodeVar14 ), 0.0 );
	nodeVar14 = max( nodeVar14, 4.0 );
	nodeVar18 = exp2( nodeVar14 );
	nodeVar19 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar15.x, ( - nodeVar15.y ), nodeVar15.z ), 1.0 ) ).xyz, nodeVar16 ) * vec2<f32>( ( nodeVar18 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar16 > 2.0 ) ) {

		nodeVar19.y = ( nodeVar19.y + nodeVar18 );
		nodeVar16 = ( nodeVar16 - 3.0 );
		

	}

	nodeVar19.x = ( nodeVar19.x + ( nodeVar16 * nodeVar18 ) );
	nodeVar19.x = ( nodeVar19.x + ( nodeVar17 * ( 3.0 * 16.0 ) ) );
	nodeVar19.y = ( nodeVar19.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar18 ) ) );
	nodeVar19.x = ( nodeVar19.x * object.nodeUniform13 );
	nodeVar19.y = ( nodeVar19.y * object.nodeUniform14 );
	nodeVar20 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar19, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar21 = nodeVar20.xyz;
	nodeVar22 = fract( nodeVar12 );

	if ( ( nodeVar22 != 0.0 ) ) {

		nodeVar23 = ( nodeVar13 + 1.0 );
		nodeVar24 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar15.x, ( - nodeVar15.y ), nodeVar15.z ), 1.0 ) ).xyz );
		nodeVar25 = max( ( 4.0 - nodeVar23 ), 0.0 );
		nodeVar23 = max( nodeVar23, 4.0 );
		nodeVar26 = exp2( nodeVar23 );
		nodeVar27 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar15.x, ( - nodeVar15.y ), nodeVar15.z ), 1.0 ) ).xyz, nodeVar24 ) * vec2<f32>( ( nodeVar26 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar24 > 2.0 ) ) {

			nodeVar27.y = ( nodeVar27.y + nodeVar26 );
			nodeVar24 = ( nodeVar24 - 3.0 );
			

		}

		nodeVar27.x = ( nodeVar27.x + ( nodeVar24 * nodeVar26 ) );
		nodeVar27.x = ( nodeVar27.x + ( nodeVar25 * ( 3.0 * 16.0 ) ) );
		nodeVar27.y = ( nodeVar27.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar26 ) ) );
		nodeVar27.x = ( nodeVar27.x * object.nodeUniform13 );
		nodeVar27.y = ( nodeVar27.y * object.nodeUniform14 );
		nodeVar28 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar27, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar29 = nodeVar28.xyz;
		nodeVar21 = mix( nodeVar21, nodeVar29, nodeVar22 );
		

	}

	nodeVar30 = ( radiance + ( nodeVar21 * vec3<f32>( object.nodeUniform16 ) ) );
	radiance = nodeVar30;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar31 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform10 );
	nodeVar32 = floor( nodeVar31 );
	nodeVar33 = nodeVar32;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar34 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar35 = max( ( 4.0 - nodeVar33 ), 0.0 );
	nodeVar33 = max( nodeVar33, 4.0 );
	nodeVar36 = exp2( nodeVar33 );
	nodeVar37 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar34 ) * vec2<f32>( ( nodeVar36 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar34 > 2.0 ) ) {

		nodeVar37.y = ( nodeVar37.y + nodeVar36 );
		nodeVar34 = ( nodeVar34 - 3.0 );
		

	}

	nodeVar37.x = ( nodeVar37.x + ( nodeVar34 * nodeVar36 ) );
	nodeVar37.x = ( nodeVar37.x + ( nodeVar35 * ( 3.0 * 16.0 ) ) );
	nodeVar37.y = ( nodeVar37.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar36 ) ) );
	nodeVar37.x = ( nodeVar37.x * object.nodeUniform13 );
	nodeVar37.y = ( nodeVar37.y * object.nodeUniform14 );
	nodeVar38 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar37, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar39 = nodeVar38.xyz;
	nodeVar40 = fract( nodeVar31 );

	if ( ( nodeVar40 != 0.0 ) ) {

		nodeVar41 = ( nodeVar32 + 1.0 );
		nodeVar42 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar43 = max( ( 4.0 - nodeVar41 ), 0.0 );
		nodeVar41 = max( nodeVar41, 4.0 );
		nodeVar44 = exp2( nodeVar41 );
		nodeVar45 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar42 ) * vec2<f32>( ( nodeVar44 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar42 > 2.0 ) ) {

			nodeVar45.y = ( nodeVar45.y + nodeVar44 );
			nodeVar42 = ( nodeVar42 - 3.0 );
			

		}

		nodeVar45.x = ( nodeVar45.x + ( nodeVar42 * nodeVar44 ) );
		nodeVar45.x = ( nodeVar45.x + ( nodeVar43 * ( 3.0 * 16.0 ) ) );
		nodeVar45.y = ( nodeVar45.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar44 ) ) );
		nodeVar45.x = ( nodeVar45.x * object.nodeUniform13 );
		nodeVar45.y = ( nodeVar45.y * object.nodeUniform14 );
		nodeVar46 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar45, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar47 = nodeVar46.xyz;
		nodeVar39 = mix( nodeVar39, nodeVar47, nodeVar40 );
		

	}

	nodeVar48 = ( iblIrradiance + ( ( nodeVar39 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform16 ) ) );
	iblIrradiance = nodeVar48;
	nodeVar49 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar50 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar51 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar52 = ( SpecularF90 * dfg.y );
	nodeVar53 = ( nodeVar51 + vec3<f32>( nodeVar52 ) );
	nodeVar54 = ( nodeVar49 + nodeVar53 );
	nodeVar49 = nodeVar54;
	nodeVar55 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar56 = nodeVar55;
	nodeVar57 = ( nodeVar56 * vec3<f32>( 0.047619 ) );
	nodeVar58 = ( SpecularColor + nodeVar57 );
	nodeVar59 = ( nodeVar53 * nodeVar58 );
	nodeVar60 = ( dfg.x + dfg.y );
	nodeVar61 = ( 1.0 - nodeVar60 );
	nodeVar62 = nodeVar61;
	nodeVar63 = ( vec3<f32>( nodeVar62 ) * nodeVar58 );
	nodeVar64 = ( vec3<f32>( 1.0 ) - nodeVar63 );
	nodeVar65 = nodeVar64;
	nodeVar66 = ( nodeVar59 / nodeVar65 );
	nodeVar67 = ( nodeVar66 * vec3<f32>( nodeVar62 ) );
	nodeVar68 = ( nodeVar50 + nodeVar67 );
	nodeVar50 = nodeVar68;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar70 = ( irradiance * nodeVar69 );
	nodeVar71 = ( nodeVar49 + nodeVar50 );
	nodeVar72 = ( vec3<f32>( 1.0 ) - nodeVar71 );
	nodeVar73 = nodeVar72;
	nodeVar74 = ( nodeVar70 * nodeVar73 );
	nodeVar75 = nodeVar74;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar76 = ( indirectDiffuse + nodeVar75 );
	indirectDiffuse = nodeVar76;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar77 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar78 = ( SpecularF90 * dfg.y );
	nodeVar79 = ( nodeVar77 + vec3<f32>( nodeVar78 ) );
	nodeVar80 = ( singleScatteringDielectric + nodeVar79 );
	singleScatteringDielectric = nodeVar80;
	nodeVar81 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar82 = nodeVar81;
	nodeVar83 = ( nodeVar82 * vec3<f32>( 0.047619 ) );
	nodeVar84 = ( SpecularColor + nodeVar83 );
	nodeVar85 = ( nodeVar79 * nodeVar84 );
	nodeVar86 = ( dfg.x + dfg.y );
	nodeVar87 = ( 1.0 - nodeVar86 );
	nodeVar88 = nodeVar87;
	nodeVar89 = ( vec3<f32>( nodeVar88 ) * nodeVar84 );
	nodeVar90 = ( vec3<f32>( 1.0 ) - nodeVar89 );
	nodeVar91 = nodeVar90;
	nodeVar92 = ( nodeVar85 / nodeVar91 );
	nodeVar93 = ( nodeVar92 * vec3<f32>( nodeVar88 ) );
	nodeVar94 = ( multiScatteringDielectric + nodeVar93 );
	multiScatteringDielectric = nodeVar94;
	nodeVar95 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar96 = ( SpecularF90 * dfg.y );
	nodeVar97 = ( nodeVar95 + vec3<f32>( nodeVar96 ) );
	nodeVar98 = ( singleScatteringMetallic + nodeVar97 );
	singleScatteringMetallic = nodeVar98;
	nodeVar99 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar100 = nodeVar99;
	nodeVar101 = ( nodeVar100 * vec3<f32>( 0.047619 ) );
	nodeVar102 = ( DiffuseColor.xyz + nodeVar101 );
	nodeVar103 = ( nodeVar97 * nodeVar102 );
	nodeVar104 = ( dfg.x + dfg.y );
	nodeVar105 = ( 1.0 - nodeVar104 );
	nodeVar106 = nodeVar105;
	nodeVar107 = ( vec3<f32>( nodeVar106 ) * nodeVar102 );
	nodeVar108 = ( vec3<f32>( 1.0 ) - nodeVar107 );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( nodeVar103 / nodeVar109 );
	nodeVar111 = ( nodeVar110 * vec3<f32>( nodeVar106 ) );
	nodeVar112 = ( multiScatteringMetallic + nodeVar111 );
	multiScatteringMetallic = nodeVar112;
	nodeVar113 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar114 = ( radiance * nodeVar113 );
	nodeVar115 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar116 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar117 = ( nodeVar115 * nodeVar116 );
	nodeVar118 = ( nodeVar114 + nodeVar117 );
	nodeVar119 = nodeVar118;
	nodeVar120 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar121 = ( vec3<f32>( 1.0 ) - nodeVar120 );
	nodeVar122 = nodeVar121;
	nodeVar123 = ( DiffuseContribution * nodeVar122 );
	nodeVar124 = ( nodeVar123 * nodeVar116 );
	nodeVar125 = nodeVar124;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar126 = ( indirectSpecular + nodeVar119 );
	indirectSpecular = nodeVar126;
	nodeVar127 = ( indirectDiffuse + nodeVar125 );
	indirectDiffuse = nodeVar127;
	ambientOcclusion = 1.0;
	nodeVar128 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar128;
	nodeVar129 = dot( normalView, positionViewDirection );
	nodeVar130 = ( clamp( nodeVar129, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar131 = ( Roughness * -16.0 );
	nodeVar132 = ( 1.0 - nodeVar131 );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( - nodeVar133 );
	nodeVar135 = exp2( nodeVar134 );
	nodeVar136 = pow( nodeVar130, nodeVar135 );
	nodeVar137 = ( 1.0 - nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( ambientOcclusion - nodeVar138 );
	nodeVar140 = ( indirectSpecular * vec3<f32>( clamp( nodeVar139, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar140;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar141 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar141;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar142 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar142;
	nodeVar143 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar143;
	nodeVar144 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar144;

	// result

	output.color = nodeVar144;

	return output;

}
