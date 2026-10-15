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
@binding( 3 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform4 : mat3x3<f32>,
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
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> Clearcoat : f32;
var<private> ClearcoatRoughness : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> clearcoatRadiance : vec3<f32>;
var<private> clearcoatSpecularDirect : vec3<f32>;
var<private> clearcoatSpecularIndirect : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : vec4<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : vec2<f32>;
var<private> nodeVar49 : vec4<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> clearcoatNormalView : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec2<f32>;
var<private> nodeVar60 : vec4<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec2<f32>;
var<private> nodeVar68 : vec4<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec2<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec4<f32>;

// codes
fn mx_srgb_texture_to_lin_rec709 ( color : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<bool>;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : vec3<f32>;

	nodeVar0 = color;
	nodeVar1 = ( nodeVar0 > vec3<f32>( 0.04045, 0.04045, 0.04045 ) );
	nodeVar2 = ( nodeVar0 / vec3<f32>( 12.92 ) );
	nodeVar3 = pow( ( max( ( nodeVar0 + vec3<f32>( 0.055, 0.055, 0.055 ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) / vec3<f32>( 1.055 ) ), vec3<f32>( 2.4, 2.4, 2.4 ) );

	return mix( nodeVar2, nodeVar3, vec3<f32>( nodeVar1 ) );

}


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

	nodeVar0 = ( ( vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) ) * vec2<f32>( 1.0, 1.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar1 = textureSample( nodeUniform0, nodeUniform0_sampler, vec2<f32>( nodeVar0[ 0u ], ( 1.0 - nodeVar0[ 1u ] ) ) );
	DiffuseColor = vec4<f32>( ( ( vec3<f32>( 1.0 ) * vec3<f32>( 1.0, 1.0, 1.0 ) ) * mx_srgb_texture_to_lin_rec709( nodeVar1.xyz ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = 1.0;
	nodeVar2 = ( ( vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) ) * vec2<f32>( 1.0, 1.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar3 = textureSample( nodeUniform2, nodeUniform2_sampler, vec2<f32>( nodeVar2[ 0u ], ( 1.0 - nodeVar2[ 1u ] ) ) );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar3.x, 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	IOR = object.nodeUniform5;
	nodeVar5 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar5 * nodeVar5 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( 0.0 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( 0.0, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - 1.0 ) ) );
	Clearcoat = 1.0;
	nodeVar6 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	ClearcoatRoughness = min( ( max( nodeVar3.x, 0.0525 ) + max( max( nodeVar6.x, nodeVar6.y ), nodeVar6.z ) ), 1.0 );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	clearcoatRadiance = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar7 = dot( normalView, positionViewDirection );
	nodeVar8 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar7, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar8;
	nodeVar9 = ( dfg.x + dfg.y );
	nodeVar10 = ( 1.0 / nodeVar9 );
	nodeVar11 = nodeVar10;
	nodeVar12 = ( nodeVar11 - 1.0 );
	nodeVar13 = ( SpecularColorBlended * vec3<f32>( nodeVar12 ) );
	nodeVar14 = ( nodeVar13 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar14;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar15 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform10 );
	nodeVar16 = floor( nodeVar15 );
	nodeVar17 = nodeVar16;
	nodeVar18 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar19 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar18.x, ( - nodeVar18.y ), nodeVar18.z ), 1.0 ) ).xyz );
	nodeVar20 = max( ( 4.0 - nodeVar17 ), 0.0 );
	nodeVar17 = max( nodeVar17, 4.0 );
	nodeVar21 = exp2( nodeVar17 );
	nodeVar22 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar18.x, ( - nodeVar18.y ), nodeVar18.z ), 1.0 ) ).xyz, nodeVar19 ) * vec2<f32>( ( nodeVar21 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar19 > 2.0 ) ) {

		nodeVar22.y = ( nodeVar22.y + nodeVar21 );
		nodeVar19 = ( nodeVar19 - 3.0 );
		

	}

	nodeVar22.x = ( nodeVar22.x + ( nodeVar19 * nodeVar21 ) );
	nodeVar22.x = ( nodeVar22.x + ( nodeVar20 * ( 3.0 * 16.0 ) ) );
	nodeVar22.y = ( nodeVar22.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar21 ) ) );
	nodeVar22.x = ( nodeVar22.x * object.nodeUniform13 );
	nodeVar22.y = ( nodeVar22.y * object.nodeUniform14 );
	nodeVar23 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar22, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar24 = nodeVar23.xyz;
	nodeVar25 = fract( nodeVar15 );

	if ( ( nodeVar25 != 0.0 ) ) {

		nodeVar26 = ( nodeVar16 + 1.0 );
		nodeVar27 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar18.x, ( - nodeVar18.y ), nodeVar18.z ), 1.0 ) ).xyz );
		nodeVar28 = max( ( 4.0 - nodeVar26 ), 0.0 );
		nodeVar26 = max( nodeVar26, 4.0 );
		nodeVar29 = exp2( nodeVar26 );
		nodeVar30 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar18.x, ( - nodeVar18.y ), nodeVar18.z ), 1.0 ) ).xyz, nodeVar27 ) * vec2<f32>( ( nodeVar29 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar27 > 2.0 ) ) {

			nodeVar30.y = ( nodeVar30.y + nodeVar29 );
			nodeVar27 = ( nodeVar27 - 3.0 );
			

		}

		nodeVar30.x = ( nodeVar30.x + ( nodeVar27 * nodeVar29 ) );
		nodeVar30.x = ( nodeVar30.x + ( nodeVar28 * ( 3.0 * 16.0 ) ) );
		nodeVar30.y = ( nodeVar30.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar29 ) ) );
		nodeVar30.x = ( nodeVar30.x * object.nodeUniform13 );
		nodeVar30.y = ( nodeVar30.y * object.nodeUniform14 );
		nodeVar31 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar30, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar32 = nodeVar31.xyz;
		nodeVar24 = mix( nodeVar24, nodeVar32, nodeVar25 );
		

	}

	nodeVar33 = ( radiance + ( nodeVar24 * vec3<f32>( object.nodeUniform16 ) ) );
	radiance = nodeVar33;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar34 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform10 );
	nodeVar35 = floor( nodeVar34 );
	nodeVar36 = nodeVar35;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar37 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar38 = max( ( 4.0 - nodeVar36 ), 0.0 );
	nodeVar36 = max( nodeVar36, 4.0 );
	nodeVar39 = exp2( nodeVar36 );
	nodeVar40 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar37 ) * vec2<f32>( ( nodeVar39 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar37 > 2.0 ) ) {

		nodeVar40.y = ( nodeVar40.y + nodeVar39 );
		nodeVar37 = ( nodeVar37 - 3.0 );
		

	}

	nodeVar40.x = ( nodeVar40.x + ( nodeVar37 * nodeVar39 ) );
	nodeVar40.x = ( nodeVar40.x + ( nodeVar38 * ( 3.0 * 16.0 ) ) );
	nodeVar40.y = ( nodeVar40.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar39 ) ) );
	nodeVar40.x = ( nodeVar40.x * object.nodeUniform13 );
	nodeVar40.y = ( nodeVar40.y * object.nodeUniform14 );
	nodeVar41 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar40, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar42 = nodeVar41.xyz;
	nodeVar43 = fract( nodeVar34 );

	if ( ( nodeVar43 != 0.0 ) ) {

		nodeVar44 = ( nodeVar35 + 1.0 );
		nodeVar45 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar46 = max( ( 4.0 - nodeVar44 ), 0.0 );
		nodeVar44 = max( nodeVar44, 4.0 );
		nodeVar47 = exp2( nodeVar44 );
		nodeVar48 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar45 ) * vec2<f32>( ( nodeVar47 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar45 > 2.0 ) ) {

			nodeVar48.y = ( nodeVar48.y + nodeVar47 );
			nodeVar45 = ( nodeVar45 - 3.0 );
			

		}

		nodeVar48.x = ( nodeVar48.x + ( nodeVar45 * nodeVar47 ) );
		nodeVar48.x = ( nodeVar48.x + ( nodeVar46 * ( 3.0 * 16.0 ) ) );
		nodeVar48.y = ( nodeVar48.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar47 ) ) );
		nodeVar48.x = ( nodeVar48.x * object.nodeUniform13 );
		nodeVar48.y = ( nodeVar48.y * object.nodeUniform14 );
		nodeVar49 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar48, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar50 = nodeVar49.xyz;
		nodeVar42 = mix( nodeVar42, nodeVar50, nodeVar43 );
		

	}

	nodeVar51 = ( iblIrradiance + ( ( nodeVar42 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform16 ) ) );
	iblIrradiance = nodeVar51;
	nodeVar52 = clamp( roughnessToMip( ClearcoatRoughness ), -2.0, object.nodeUniform10 );
	nodeVar53 = floor( nodeVar52 );
	nodeVar54 = nodeVar53;
	clearcoatNormalView = NORMAL_normalView;
	nodeVar55 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), clearcoatNormalView ), clearcoatNormalView, ( ( ( ClearcoatRoughness * ClearcoatRoughness ) * ClearcoatRoughness ) * ClearcoatRoughness ) ) ), 0.0 ) ).xyz );
	nodeVar56 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar55.x, ( - nodeVar55.y ), nodeVar55.z ), 1.0 ) ).xyz );
	nodeVar57 = max( ( 4.0 - nodeVar54 ), 0.0 );
	nodeVar54 = max( nodeVar54, 4.0 );
	nodeVar58 = exp2( nodeVar54 );
	nodeVar59 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar55.x, ( - nodeVar55.y ), nodeVar55.z ), 1.0 ) ).xyz, nodeVar56 ) * vec2<f32>( ( nodeVar58 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar56 > 2.0 ) ) {

		nodeVar59.y = ( nodeVar59.y + nodeVar58 );
		nodeVar56 = ( nodeVar56 - 3.0 );
		

	}

	nodeVar59.x = ( nodeVar59.x + ( nodeVar56 * nodeVar58 ) );
	nodeVar59.x = ( nodeVar59.x + ( nodeVar57 * ( 3.0 * 16.0 ) ) );
	nodeVar59.y = ( nodeVar59.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar58 ) ) );
	nodeVar59.x = ( nodeVar59.x * object.nodeUniform13 );
	nodeVar59.y = ( nodeVar59.y * object.nodeUniform14 );
	nodeVar60 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar59, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar61 = nodeVar60.xyz;
	nodeVar62 = fract( nodeVar52 );

	if ( ( nodeVar62 != 0.0 ) ) {

		nodeVar63 = ( nodeVar53 + 1.0 );
		nodeVar64 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar55.x, ( - nodeVar55.y ), nodeVar55.z ), 1.0 ) ).xyz );
		nodeVar65 = max( ( 4.0 - nodeVar63 ), 0.0 );
		nodeVar63 = max( nodeVar63, 4.0 );
		nodeVar66 = exp2( nodeVar63 );
		nodeVar67 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar55.x, ( - nodeVar55.y ), nodeVar55.z ), 1.0 ) ).xyz, nodeVar64 ) * vec2<f32>( ( nodeVar66 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar64 > 2.0 ) ) {

			nodeVar67.y = ( nodeVar67.y + nodeVar66 );
			nodeVar64 = ( nodeVar64 - 3.0 );
			

		}

		nodeVar67.x = ( nodeVar67.x + ( nodeVar64 * nodeVar66 ) );
		nodeVar67.x = ( nodeVar67.x + ( nodeVar65 * ( 3.0 * 16.0 ) ) );
		nodeVar67.y = ( nodeVar67.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar66 ) ) );
		nodeVar67.x = ( nodeVar67.x * object.nodeUniform13 );
		nodeVar67.y = ( nodeVar67.y * object.nodeUniform14 );
		nodeVar68 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar67, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar69 = nodeVar68.xyz;
		nodeVar61 = mix( nodeVar61, nodeVar69, nodeVar62 );
		

	}

	nodeVar70 = ( clearcoatRadiance + ( nodeVar61 * vec3<f32>( object.nodeUniform16 ) ) );
	clearcoatRadiance = nodeVar70;
	nodeVar71 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar72 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar73 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar74 = ( SpecularF90 * dfg.y );
	nodeVar75 = ( nodeVar73 + vec3<f32>( nodeVar74 ) );
	nodeVar76 = ( nodeVar71 + nodeVar75 );
	nodeVar71 = nodeVar76;
	nodeVar77 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar78 = nodeVar77;
	nodeVar79 = ( nodeVar78 * vec3<f32>( 0.047619 ) );
	nodeVar80 = ( SpecularColor + nodeVar79 );
	nodeVar81 = ( nodeVar75 * nodeVar80 );
	nodeVar82 = ( dfg.x + dfg.y );
	nodeVar83 = ( 1.0 - nodeVar82 );
	nodeVar84 = nodeVar83;
	nodeVar85 = ( vec3<f32>( nodeVar84 ) * nodeVar80 );
	nodeVar86 = ( vec3<f32>( 1.0 ) - nodeVar85 );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( nodeVar81 / nodeVar87 );
	nodeVar89 = ( nodeVar88 * vec3<f32>( nodeVar84 ) );
	nodeVar90 = ( nodeVar72 + nodeVar89 );
	nodeVar72 = nodeVar90;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar91 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar92 = ( irradiance * nodeVar91 );
	nodeVar93 = ( nodeVar71 + nodeVar72 );
	nodeVar94 = ( vec3<f32>( 1.0 ) - nodeVar93 );
	nodeVar95 = nodeVar94;
	nodeVar96 = ( nodeVar92 * nodeVar95 );
	nodeVar97 = nodeVar96;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar98 = ( indirectDiffuse + nodeVar97 );
	indirectDiffuse = nodeVar98;
	nodeVar99 = dot( clearcoatNormalView, positionViewDirection );
	nodeVar100 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( ClearcoatRoughness, clamp( nodeVar99, 0.0, 1.0 ) ) ).xy;
	nodeVar101 = ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( nodeVar100.x ) ) + vec3<f32>( ( 1.0 * nodeVar100.y ) ) );
	nodeVar102 = ( clearcoatRadiance * nodeVar101 );
	nodeVar103 = ( clearcoatSpecularIndirect + nodeVar102 );
	clearcoatSpecularIndirect = nodeVar103;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar104 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar105 = ( SpecularF90 * dfg.y );
	nodeVar106 = ( nodeVar104 + vec3<f32>( nodeVar105 ) );
	nodeVar107 = ( singleScatteringDielectric + nodeVar106 );
	singleScatteringDielectric = nodeVar107;
	nodeVar108 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( nodeVar109 * vec3<f32>( 0.047619 ) );
	nodeVar111 = ( SpecularColor + nodeVar110 );
	nodeVar112 = ( nodeVar106 * nodeVar111 );
	nodeVar113 = ( dfg.x + dfg.y );
	nodeVar114 = ( 1.0 - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( vec3<f32>( nodeVar115 ) * nodeVar111 );
	nodeVar117 = ( vec3<f32>( 1.0 ) - nodeVar116 );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar112 / nodeVar118 );
	nodeVar120 = ( nodeVar119 * vec3<f32>( nodeVar115 ) );
	nodeVar121 = ( multiScatteringDielectric + nodeVar120 );
	multiScatteringDielectric = nodeVar121;
	nodeVar122 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar123 = ( SpecularF90 * dfg.y );
	nodeVar124 = ( nodeVar122 + vec3<f32>( nodeVar123 ) );
	nodeVar125 = ( singleScatteringMetallic + nodeVar124 );
	singleScatteringMetallic = nodeVar125;
	nodeVar126 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( nodeVar127 * vec3<f32>( 0.047619 ) );
	nodeVar129 = ( DiffuseColor.xyz + nodeVar128 );
	nodeVar130 = ( nodeVar124 * nodeVar129 );
	nodeVar131 = ( dfg.x + dfg.y );
	nodeVar132 = ( 1.0 - nodeVar131 );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( vec3<f32>( nodeVar133 ) * nodeVar129 );
	nodeVar135 = ( vec3<f32>( 1.0 ) - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( nodeVar130 / nodeVar136 );
	nodeVar138 = ( nodeVar137 * vec3<f32>( nodeVar133 ) );
	nodeVar139 = ( multiScatteringMetallic + nodeVar138 );
	multiScatteringMetallic = nodeVar139;
	nodeVar140 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar141 = ( radiance * nodeVar140 );
	nodeVar142 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar143 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar144 = ( nodeVar142 * nodeVar143 );
	nodeVar145 = ( nodeVar141 + nodeVar144 );
	nodeVar146 = nodeVar145;
	nodeVar147 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar148 = ( vec3<f32>( 1.0 ) - nodeVar147 );
	nodeVar149 = nodeVar148;
	nodeVar150 = ( DiffuseContribution * nodeVar149 );
	nodeVar151 = ( nodeVar150 * nodeVar143 );
	nodeVar152 = nodeVar151;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar153 = ( indirectSpecular + nodeVar146 );
	indirectSpecular = nodeVar153;
	nodeVar154 = ( indirectDiffuse + nodeVar152 );
	indirectDiffuse = nodeVar154;
	ambientOcclusion = 1.0;
	nodeVar155 = ( clearcoatSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	clearcoatSpecularIndirect = nodeVar155;
	nodeVar156 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar156;
	nodeVar157 = dot( normalView, positionViewDirection );
	nodeVar158 = ( clamp( nodeVar157, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar159 = ( Roughness * -16.0 );
	nodeVar160 = ( 1.0 - nodeVar159 );
	nodeVar161 = nodeVar160;
	nodeVar162 = ( - nodeVar161 );
	nodeVar163 = exp2( nodeVar162 );
	nodeVar164 = pow( nodeVar158, nodeVar163 );
	nodeVar165 = ( 1.0 - nodeVar164 );
	nodeVar166 = nodeVar165;
	nodeVar167 = ( ambientOcclusion - nodeVar166 );
	nodeVar168 = ( indirectSpecular * vec3<f32>( clamp( nodeVar167, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar168;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar169 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar169;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar170 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar170;
	nodeVar171 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar171;
	nodeVar172 = dot( clearcoatNormalView, positionViewDirection );
	nodeVar173 = clamp( nodeVar172, 0.0, 1.0 );
	nodeVar174 = exp2( ( ( ( nodeVar173 * -5.55473 ) - 6.98316 ) * nodeVar173 ) );
	nodeVar175 = ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( ( 1.0 - nodeVar174 ) ) ) + vec3<f32>( ( 1.0 * nodeVar174 ) ) );
	nodeVar176 = ( vec3<f32>( Clearcoat ) * nodeVar175 );
	nodeVar177 = ( vec3<f32>( 1.0 ) - nodeVar176 );
	nodeVar178 = nodeVar177;
	nodeVar179 = ( outgoingLight * nodeVar178 );
	nodeVar180 = ( clearcoatSpecularDirect + clearcoatSpecularIndirect );
	nodeVar181 = ( nodeVar180 * vec3<f32>( Clearcoat ) );
	nodeVar182 = ( nodeVar179 + nodeVar181 );
	outgoingLight = nodeVar182;
	nodeVar183 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar183;

	// result

	output.color = nodeVar183;

	return output;

}
