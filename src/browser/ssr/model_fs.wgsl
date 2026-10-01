// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform17_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform17 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform25_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform25 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform19 : vec2<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform26 : f32
};
@binding( 0 ) @group( 1 )
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
var<private> nodeVar0 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : vec4<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec2<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar53 : f32;
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
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec2<f32>;

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
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_bitangentView : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 5 ) nodeVarying9 : vec2<f32> ) -> OutputType {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	AmbientOcclusion = ( ( ( nodeVar1.x - 1.0 ) * object.nodeUniform6 ) + 1.0 );
	nodeVar2 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform10 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar5 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform9 * nodeVar3.y ), 0.0525 ) + max( max( nodeVar5.x, nodeVar5.y ), nodeVar5.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	EmissiveColor = ( object.nodeUniform13 * vec3<f32>( object.nodeUniform14 ) );
	NORMAL_tangentView = normalize( v_tangentView );
	NORMAL_bitangentView = normalize( NORMAL_v_bitangentView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar6 = textureSample( nodeUniform17, nodeUniform17_sampler, ( object.nodeUniform18 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	nodeVar7 = ( ( nodeVar6 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar7.xy * object.nodeUniform19 ), nodeVar7.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar8 = dot( normalView, positionViewDirection );
	nodeVar9 = textureSample( nodeUniform15, nodeUniform15_sampler, vec2<f32>( Roughness, clamp( nodeVar8, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar9;
	nodeVar10 = ( dfg.x + dfg.y );
	nodeVar11 = ( 1.0 / nodeVar10 );
	nodeVar12 = nodeVar11;
	nodeVar13 = ( nodeVar12 - 1.0 );
	nodeVar14 = ( SpecularColorBlended * vec3<f32>( nodeVar13 ) );
	nodeVar15 = ( nodeVar14 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar15;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar16 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform20 );
	nodeVar17 = floor( nodeVar16 );
	nodeVar18 = nodeVar17;
	nodeVar19 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar20 = getFace( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz );
	nodeVar21 = max( ( 4.0 - nodeVar18 ), 0.0 );
	nodeVar18 = max( nodeVar18, 4.0 );
	nodeVar22 = exp2( nodeVar18 );
	nodeVar23 = ( ( getUV( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz, nodeVar20 ) * vec2<f32>( ( nodeVar22 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar20 > 2.0 ) ) {

		nodeVar23.y = ( nodeVar23.y + nodeVar22 );
		nodeVar20 = ( nodeVar20 - 3.0 );
		

	}

	nodeVar23.x = ( nodeVar23.x + ( nodeVar20 * nodeVar22 ) );
	nodeVar23.x = ( nodeVar23.x + ( nodeVar21 * ( 3.0 * 16.0 ) ) );
	nodeVar23.y = ( nodeVar23.y + ( 4.0 * ( exp2( object.nodeUniform20 ) - nodeVar22 ) ) );
	nodeVar23.x = ( nodeVar23.x * object.nodeUniform23 );
	nodeVar23.y = ( nodeVar23.y * object.nodeUniform24 );
	nodeVar24 = textureSampleGrad( nodeUniform25, nodeUniform25_sampler, nodeVar23, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar25 = nodeVar24.xyz;
	nodeVar26 = fract( nodeVar16 );

	if ( ( nodeVar26 != 0.0 ) ) {

		nodeVar27 = ( nodeVar17 + 1.0 );
		nodeVar28 = getFace( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz );
		nodeVar29 = max( ( 4.0 - nodeVar27 ), 0.0 );
		nodeVar27 = max( nodeVar27, 4.0 );
		nodeVar30 = exp2( nodeVar27 );
		nodeVar31 = ( ( getUV( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz, nodeVar28 ) * vec2<f32>( ( nodeVar30 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar28 > 2.0 ) ) {

			nodeVar31.y = ( nodeVar31.y + nodeVar30 );
			nodeVar28 = ( nodeVar28 - 3.0 );
			

		}

		nodeVar31.x = ( nodeVar31.x + ( nodeVar28 * nodeVar30 ) );
		nodeVar31.x = ( nodeVar31.x + ( nodeVar29 * ( 3.0 * 16.0 ) ) );
		nodeVar31.y = ( nodeVar31.y + ( 4.0 * ( exp2( object.nodeUniform20 ) - nodeVar30 ) ) );
		nodeVar31.x = ( nodeVar31.x * object.nodeUniform23 );
		nodeVar31.y = ( nodeVar31.y * object.nodeUniform24 );
		nodeVar32 = textureSampleGrad( nodeUniform25, nodeUniform25_sampler, nodeVar31, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar33 = nodeVar32.xyz;
		nodeVar25 = mix( nodeVar25, nodeVar33, nodeVar26 );
		

	}

	nodeVar34 = ( radiance + ( nodeVar25 * vec3<f32>( object.nodeUniform26 ) ) );
	radiance = nodeVar34;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar35 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform20 );
	nodeVar36 = floor( nodeVar35 );
	nodeVar37 = nodeVar36;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar38 = getFace( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar39 = max( ( 4.0 - nodeVar37 ), 0.0 );
	nodeVar37 = max( nodeVar37, 4.0 );
	nodeVar40 = exp2( nodeVar37 );
	nodeVar41 = ( ( getUV( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar38 ) * vec2<f32>( ( nodeVar40 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar38 > 2.0 ) ) {

		nodeVar41.y = ( nodeVar41.y + nodeVar40 );
		nodeVar38 = ( nodeVar38 - 3.0 );
		

	}

	nodeVar41.x = ( nodeVar41.x + ( nodeVar38 * nodeVar40 ) );
	nodeVar41.x = ( nodeVar41.x + ( nodeVar39 * ( 3.0 * 16.0 ) ) );
	nodeVar41.y = ( nodeVar41.y + ( 4.0 * ( exp2( object.nodeUniform20 ) - nodeVar40 ) ) );
	nodeVar41.x = ( nodeVar41.x * object.nodeUniform23 );
	nodeVar41.y = ( nodeVar41.y * object.nodeUniform24 );
	nodeVar42 = textureSampleGrad( nodeUniform25, nodeUniform25_sampler, nodeVar41, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar43 = nodeVar42.xyz;
	nodeVar44 = fract( nodeVar35 );

	if ( ( nodeVar44 != 0.0 ) ) {

		nodeVar45 = ( nodeVar36 + 1.0 );
		nodeVar46 = getFace( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar47 = max( ( 4.0 - nodeVar45 ), 0.0 );
		nodeVar45 = max( nodeVar45, 4.0 );
		nodeVar48 = exp2( nodeVar45 );
		nodeVar49 = ( ( getUV( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar46 ) * vec2<f32>( ( nodeVar48 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar46 > 2.0 ) ) {

			nodeVar49.y = ( nodeVar49.y + nodeVar48 );
			nodeVar46 = ( nodeVar46 - 3.0 );
			

		}

		nodeVar49.x = ( nodeVar49.x + ( nodeVar46 * nodeVar48 ) );
		nodeVar49.x = ( nodeVar49.x + ( nodeVar47 * ( 3.0 * 16.0 ) ) );
		nodeVar49.y = ( nodeVar49.y + ( 4.0 * ( exp2( object.nodeUniform20 ) - nodeVar48 ) ) );
		nodeVar49.x = ( nodeVar49.x * object.nodeUniform23 );
		nodeVar49.y = ( nodeVar49.y * object.nodeUniform24 );
		nodeVar50 = textureSampleGrad( nodeUniform25, nodeUniform25_sampler, nodeVar49, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar51 = nodeVar50.xyz;
		nodeVar43 = mix( nodeVar43, nodeVar51, nodeVar44 );
		

	}

	nodeVar52 = ( iblIrradiance + ( ( nodeVar43 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform26 ) ) );
	iblIrradiance = nodeVar52;
	ambientOcclusion = 1.0;
	nodeVar53 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar53;
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
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	nodeVar149 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m1 = vec4<f32>( nodeVar149, 1.0 );
	nodeVar150 = vec2<f32>( Metalness, Roughness );
	output.m2 = vec4<f32>( vec3<f32>( nodeVar150, 0.0 ), 1.0 );

	// result

	return output;

}
