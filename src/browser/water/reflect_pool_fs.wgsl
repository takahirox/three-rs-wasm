// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform28_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform28 : texture_2d<f32>;

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
	nodeUniform13 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : f32,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform21 : mat3x3<f32>,
	nodeUniform22 : vec2<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform29 : f32
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
var<private> nodeVar4 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar6 : f32;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec2<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec2<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec2<f32>;
var<private> nodeVar59 : vec4<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
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
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec4<f32>;

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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying6 : vec2<f32>,
	@builtin( front_facing ) isFront : bool ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	AmbientOcclusion = ( ( ( nodeVar1.x - 1.0 ) * object.nodeUniform6 ) + 1.0 );
	nodeVar2 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform10 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform9 * nodeVar3.y ), 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	IOR = object.nodeUniform13;
	nodeVar5 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar5 * nodeVar5 ) ) * object.nodeUniform14 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform15 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform15, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	EmissiveColor = ( object.nodeUniform16 * vec3<f32>( object.nodeUniform17 ) );
	nodeVar6 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeVar6 ) );
	nodeVar7 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	nodeVar8 = dpdx( nodeVarying6 );
	nodeVar9 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	nodeVar10 = - dpdy( nodeVarying6 );
	nodeVar11 = ( ( nodeVar7 * vec3<f32>( nodeVar8.x ) ) + ( nodeVar9 * vec3<f32>( nodeVar10.x ) ) );
	nodeVar13 = ( ( nodeVar7 * vec3<f32>( nodeVar8.y ) ) + ( nodeVar9 * vec3<f32>( nodeVar10.y ) ) );
	nodeVar14 = max( dot( nodeVar11, nodeVar11 ), dot( nodeVar13, nodeVar13 ) );

	if ( ( nodeVar14 == 0.0 ) ) {

		nodeVar12 = 0.0;

	} else {

		nodeVar12 = inverseSqrt( nodeVar14 );

	}

	tangentViewFrame = ( nodeVar11 * vec3<f32>( nodeVar12 ) );
	NORMAL_tangentView = ( tangentViewFrame * vec3<f32>( nodeVar6 ) );
	bitangentViewFrame = ( nodeVar13 * nodeVar12 );
	NORMAL_bitangentView = ( bitangentViewFrame * vec3<f32>( nodeVar6 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar15 = textureSample( nodeUniform20, nodeUniform20_sampler, ( object.nodeUniform21 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	nodeVar16 = ( ( nodeVar15 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar16.xy * object.nodeUniform22 ), nodeVar16.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar17 = dot( normalView, positionViewDirection );
	nodeVar18 = textureSample( nodeUniform18, nodeUniform18_sampler, vec2<f32>( Roughness, clamp( nodeVar17, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar18;
	nodeVar19 = ( dfg.x + dfg.y );
	nodeVar20 = ( 1.0 / nodeVar19 );
	nodeVar21 = nodeVar20;
	nodeVar22 = ( nodeVar21 - 1.0 );
	nodeVar23 = ( SpecularColorBlended * vec3<f32>( nodeVar22 ) );
	nodeVar24 = ( nodeVar23 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar24;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar25 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform23 );
	nodeVar26 = floor( nodeVar25 );
	nodeVar27 = nodeVar26;
	nodeVar28 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar29 = getFace( ( object.nodeUniform24 * vec4<f32>( vec3<f32>( nodeVar28.x, ( - nodeVar28.y ), nodeVar28.z ), 1.0 ) ).xyz );
	nodeVar30 = max( ( 4.0 - nodeVar27 ), 0.0 );
	nodeVar27 = max( nodeVar27, 4.0 );
	nodeVar31 = exp2( nodeVar27 );
	nodeVar32 = ( ( getUV( ( object.nodeUniform24 * vec4<f32>( vec3<f32>( nodeVar28.x, ( - nodeVar28.y ), nodeVar28.z ), 1.0 ) ).xyz, nodeVar29 ) * vec2<f32>( ( nodeVar31 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar29 > 2.0 ) ) {

		nodeVar32.y = ( nodeVar32.y + nodeVar31 );
		nodeVar29 = ( nodeVar29 - 3.0 );
		

	}

	nodeVar32.x = ( nodeVar32.x + ( nodeVar29 * nodeVar31 ) );
	nodeVar32.x = ( nodeVar32.x + ( nodeVar30 * ( 3.0 * 16.0 ) ) );
	nodeVar32.y = ( nodeVar32.y + ( 4.0 * ( exp2( object.nodeUniform23 ) - nodeVar31 ) ) );
	nodeVar32.x = ( nodeVar32.x * object.nodeUniform26 );
	nodeVar32.y = ( nodeVar32.y * object.nodeUniform27 );
	nodeVar33 = textureSampleGrad( nodeUniform28, nodeUniform28_sampler, nodeVar32, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar34 = nodeVar33.xyz;
	nodeVar35 = fract( nodeVar25 );

	if ( ( nodeVar35 != 0.0 ) ) {

		nodeVar36 = ( nodeVar26 + 1.0 );
		nodeVar37 = getFace( ( object.nodeUniform24 * vec4<f32>( vec3<f32>( nodeVar28.x, ( - nodeVar28.y ), nodeVar28.z ), 1.0 ) ).xyz );
		nodeVar38 = max( ( 4.0 - nodeVar36 ), 0.0 );
		nodeVar36 = max( nodeVar36, 4.0 );
		nodeVar39 = exp2( nodeVar36 );
		nodeVar40 = ( ( getUV( ( object.nodeUniform24 * vec4<f32>( vec3<f32>( nodeVar28.x, ( - nodeVar28.y ), nodeVar28.z ), 1.0 ) ).xyz, nodeVar37 ) * vec2<f32>( ( nodeVar39 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar37 > 2.0 ) ) {

			nodeVar40.y = ( nodeVar40.y + nodeVar39 );
			nodeVar37 = ( nodeVar37 - 3.0 );
			

		}

		nodeVar40.x = ( nodeVar40.x + ( nodeVar37 * nodeVar39 ) );
		nodeVar40.x = ( nodeVar40.x + ( nodeVar38 * ( 3.0 * 16.0 ) ) );
		nodeVar40.y = ( nodeVar40.y + ( 4.0 * ( exp2( object.nodeUniform23 ) - nodeVar39 ) ) );
		nodeVar40.x = ( nodeVar40.x * object.nodeUniform26 );
		nodeVar40.y = ( nodeVar40.y * object.nodeUniform27 );
		nodeVar41 = textureSampleGrad( nodeUniform28, nodeUniform28_sampler, nodeVar40, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar42 = nodeVar41.xyz;
		nodeVar34 = mix( nodeVar34, nodeVar42, nodeVar35 );
		

	}

	nodeVar43 = ( radiance + ( nodeVar34 * vec3<f32>( object.nodeUniform29 ) ) );
	radiance = nodeVar43;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar44 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform23 );
	nodeVar45 = floor( nodeVar44 );
	nodeVar46 = nodeVar45;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar47 = getFace( ( object.nodeUniform24 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar48 = max( ( 4.0 - nodeVar46 ), 0.0 );
	nodeVar46 = max( nodeVar46, 4.0 );
	nodeVar49 = exp2( nodeVar46 );
	nodeVar50 = ( ( getUV( ( object.nodeUniform24 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar47 ) * vec2<f32>( ( nodeVar49 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar47 > 2.0 ) ) {

		nodeVar50.y = ( nodeVar50.y + nodeVar49 );
		nodeVar47 = ( nodeVar47 - 3.0 );
		

	}

	nodeVar50.x = ( nodeVar50.x + ( nodeVar47 * nodeVar49 ) );
	nodeVar50.x = ( nodeVar50.x + ( nodeVar48 * ( 3.0 * 16.0 ) ) );
	nodeVar50.y = ( nodeVar50.y + ( 4.0 * ( exp2( object.nodeUniform23 ) - nodeVar49 ) ) );
	nodeVar50.x = ( nodeVar50.x * object.nodeUniform26 );
	nodeVar50.y = ( nodeVar50.y * object.nodeUniform27 );
	nodeVar51 = textureSampleGrad( nodeUniform28, nodeUniform28_sampler, nodeVar50, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar52 = nodeVar51.xyz;
	nodeVar53 = fract( nodeVar44 );

	if ( ( nodeVar53 != 0.0 ) ) {

		nodeVar54 = ( nodeVar45 + 1.0 );
		nodeVar55 = getFace( ( object.nodeUniform24 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar56 = max( ( 4.0 - nodeVar54 ), 0.0 );
		nodeVar54 = max( nodeVar54, 4.0 );
		nodeVar57 = exp2( nodeVar54 );
		nodeVar58 = ( ( getUV( ( object.nodeUniform24 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar55 ) * vec2<f32>( ( nodeVar57 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar55 > 2.0 ) ) {

			nodeVar58.y = ( nodeVar58.y + nodeVar57 );
			nodeVar55 = ( nodeVar55 - 3.0 );
			

		}

		nodeVar58.x = ( nodeVar58.x + ( nodeVar55 * nodeVar57 ) );
		nodeVar58.x = ( nodeVar58.x + ( nodeVar56 * ( 3.0 * 16.0 ) ) );
		nodeVar58.y = ( nodeVar58.y + ( 4.0 * ( exp2( object.nodeUniform23 ) - nodeVar57 ) ) );
		nodeVar58.x = ( nodeVar58.x * object.nodeUniform26 );
		nodeVar58.y = ( nodeVar58.y * object.nodeUniform27 );
		nodeVar59 = textureSampleGrad( nodeUniform28, nodeUniform28_sampler, nodeVar58, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar60 = nodeVar59.xyz;
		nodeVar52 = mix( nodeVar52, nodeVar60, nodeVar53 );
		

	}

	nodeVar61 = ( iblIrradiance + ( ( nodeVar52 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform29 ) ) );
	iblIrradiance = nodeVar61;
	ambientOcclusion = 1.0;
	nodeVar62 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar62;
	nodeVar63 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar64 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar65 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar66 = ( SpecularF90 * dfg.y );
	nodeVar67 = ( nodeVar65 + vec3<f32>( nodeVar66 ) );
	nodeVar68 = ( nodeVar63 + nodeVar67 );
	nodeVar63 = nodeVar68;
	nodeVar69 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar70 = nodeVar69;
	nodeVar71 = ( nodeVar70 * vec3<f32>( 0.047619 ) );
	nodeVar72 = ( SpecularColor + nodeVar71 );
	nodeVar73 = ( nodeVar67 * nodeVar72 );
	nodeVar74 = ( dfg.x + dfg.y );
	nodeVar75 = ( 1.0 - nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = ( vec3<f32>( nodeVar76 ) * nodeVar72 );
	nodeVar78 = ( vec3<f32>( 1.0 ) - nodeVar77 );
	nodeVar79 = nodeVar78;
	nodeVar80 = ( nodeVar73 / nodeVar79 );
	nodeVar81 = ( nodeVar80 * vec3<f32>( nodeVar76 ) );
	nodeVar82 = ( nodeVar64 + nodeVar81 );
	nodeVar64 = nodeVar82;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar83 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar84 = ( irradiance * nodeVar83 );
	nodeVar85 = ( nodeVar63 + nodeVar64 );
	nodeVar86 = ( vec3<f32>( 1.0 ) - nodeVar85 );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( nodeVar84 * nodeVar87 );
	nodeVar89 = nodeVar88;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar90 = ( indirectDiffuse + nodeVar89 );
	indirectDiffuse = nodeVar90;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar91 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar92 = ( SpecularF90 * dfg.y );
	nodeVar93 = ( nodeVar91 + vec3<f32>( nodeVar92 ) );
	nodeVar94 = ( singleScatteringDielectric + nodeVar93 );
	singleScatteringDielectric = nodeVar94;
	nodeVar95 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar96 = nodeVar95;
	nodeVar97 = ( nodeVar96 * vec3<f32>( 0.047619 ) );
	nodeVar98 = ( SpecularColor + nodeVar97 );
	nodeVar99 = ( nodeVar93 * nodeVar98 );
	nodeVar100 = ( dfg.x + dfg.y );
	nodeVar101 = ( 1.0 - nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( vec3<f32>( nodeVar102 ) * nodeVar98 );
	nodeVar104 = ( vec3<f32>( 1.0 ) - nodeVar103 );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( nodeVar99 / nodeVar105 );
	nodeVar107 = ( nodeVar106 * vec3<f32>( nodeVar102 ) );
	nodeVar108 = ( multiScatteringDielectric + nodeVar107 );
	multiScatteringDielectric = nodeVar108;
	nodeVar109 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar110 = ( SpecularF90 * dfg.y );
	nodeVar111 = ( nodeVar109 + vec3<f32>( nodeVar110 ) );
	nodeVar112 = ( singleScatteringMetallic + nodeVar111 );
	singleScatteringMetallic = nodeVar112;
	nodeVar113 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar114 * vec3<f32>( 0.047619 ) );
	nodeVar116 = ( DiffuseColor.xyz + nodeVar115 );
	nodeVar117 = ( nodeVar111 * nodeVar116 );
	nodeVar118 = ( dfg.x + dfg.y );
	nodeVar119 = ( 1.0 - nodeVar118 );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( vec3<f32>( nodeVar120 ) * nodeVar116 );
	nodeVar122 = ( vec3<f32>( 1.0 ) - nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( nodeVar117 / nodeVar123 );
	nodeVar125 = ( nodeVar124 * vec3<f32>( nodeVar120 ) );
	nodeVar126 = ( multiScatteringMetallic + nodeVar125 );
	multiScatteringMetallic = nodeVar126;
	nodeVar127 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar128 = ( radiance * nodeVar127 );
	nodeVar129 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar130 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar131 = ( nodeVar129 * nodeVar130 );
	nodeVar132 = ( nodeVar128 + nodeVar131 );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar135 = ( vec3<f32>( 1.0 ) - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( DiffuseContribution * nodeVar136 );
	nodeVar138 = ( nodeVar137 * nodeVar130 );
	nodeVar139 = nodeVar138;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = ( indirectSpecular + nodeVar133 );
	indirectSpecular = nodeVar140;
	nodeVar141 = ( indirectDiffuse + nodeVar139 );
	indirectDiffuse = nodeVar141;
	nodeVar142 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar142;
	nodeVar143 = dot( normalView, positionViewDirection );
	nodeVar144 = ( clamp( nodeVar143, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar145 = ( Roughness * -16.0 );
	nodeVar146 = ( 1.0 - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( - nodeVar147 );
	nodeVar149 = exp2( nodeVar148 );
	nodeVar150 = pow( nodeVar144, nodeVar149 );
	nodeVar151 = ( 1.0 - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( ambientOcclusion - nodeVar152 );
	nodeVar154 = ( indirectSpecular * vec3<f32>( clamp( nodeVar153, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar154;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar155 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar155;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar156 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar156;
	nodeVar157 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar157;
	nodeVar158 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar158;

	// result

	output.color = nodeVar158;

	return output;

}
