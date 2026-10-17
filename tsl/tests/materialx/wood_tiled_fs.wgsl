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
@binding( 3 ) @group( 1 ) var nodeUniform3_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform3 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
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
var<private> nodeVar7 : vec2<f32>;
var<private> Anisotropy : f32;
var<private> AlphaT : f32;
var<private> AnisotropyT : vec3<f32>;
var<private> tangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> bitangentView : vec3<f32>;
var<private> TBNViewMatrix : mat3x3<f32>;
var<private> AnisotropyB : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> clearcoatRadiance : vec3<f32>;
var<private> clearcoatSpecularDirect : vec3<f32>;
var<private> clearcoatSpecularIndirect : vec3<f32>;
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
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec2<f32>;
var<private> nodeVar53 : vec4<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> clearcoatNormalView : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec2<f32>;
var<private> nodeVar64 : vec4<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec2<f32>;
var<private> nodeVar72 : vec4<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec2<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
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
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec4<f32>;

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
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec2<f32>,
	@location( 4 ) nodeVarying8 : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( ( vec2<f32>( nodeVarying7[ 0u ], ( 1.0 - nodeVarying7[ 1u ] ) ) * vec2<f32>( 4.0, 4.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar1 = textureSample( nodeUniform0, nodeUniform0_sampler, vec2<f32>( nodeVar0[ 0u ], ( 1.0 - nodeVar0[ 1u ] ) ) );
	DiffuseColor = vec4<f32>( ( vec3<f32>( 1.0 ) * mx_srgb_texture_to_lin_rec709( nodeVar1.xyz ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	nodeVar2 = ( ( vec2<f32>( nodeVarying7[ 0u ], ( 1.0 - nodeVarying7[ 1u ] ) ) * vec2<f32>( 4.0, 4.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar3 = textureSample( nodeUniform3, nodeUniform3_sampler, vec2<f32>( nodeVar2[ 0u ], ( 1.0 - nodeVar2[ 1u ] ) ) );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar3.x, 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	IOR = object.nodeUniform6;
	nodeVar5 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar5 * nodeVar5 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( 0.4 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( 0.4, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	Clearcoat = 0.1;
	nodeVar6 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	ClearcoatRoughness = min( ( max( 0.2, 0.0525 ) + max( max( nodeVar6.x, nodeVar6.y ), nodeVar6.z ) ), 1.0 );
	nodeVar7 = ( vec2<f32>( cos( 0.0 ), sin( 0.0 ) ) * vec2<f32>( 0.5 ) );
	Anisotropy = length( nodeVar7 );

	if ( ( Anisotropy == 0.0 ) ) {

		nodeVar7 = vec2<f32>( 1.0, 0.0 );
		

	} else {

		nodeVar7 = ( nodeVar7 / vec2<f32>( Anisotropy ) );
		Anisotropy = clamp( Anisotropy, 0.0, 1.0 );
		

	}

	AlphaT = mix( ( Roughness * Roughness ), 1.0, ( Anisotropy * Anisotropy ) );
	tangentView = normalize( v_tangentView );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	bitangentView = normalize( ( cross( normalView, tangentView ) * vec3<f32>( nodeVarying8.w ) ) );
	TBNViewMatrix = mat3x3<f32>( tangentView, bitangentView, normalView );
	AnisotropyT = ( ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar7.x ) ) + ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar7.y ) ) );
	AnisotropyB = ( ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar7.x ) ) - ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar7.y ) ) );
	EmissiveColor = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	clearcoatRadiance = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar8 = dot( normalView, positionViewDirection );
	nodeVar9 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( Roughness, clamp( nodeVar8, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar9;
	nodeVar10 = ( dfg.x + dfg.y );
	nodeVar11 = ( 1.0 / nodeVar10 );
	nodeVar12 = nodeVar11;
	nodeVar13 = ( nodeVar12 - 1.0 );
	nodeVar14 = ( SpecularColorBlended * vec3<f32>( nodeVar13 ) );
	nodeVar15 = ( nodeVar14 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar15;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar16 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform11 );
	nodeVar17 = floor( nodeVar16 );
	nodeVar18 = nodeVar17;
	nodeVar19 = ( 1.0 - ( Anisotropy * ( 1.0 - Roughness ) ) );
	nodeVar20 = ( nodeVar19 * nodeVar19 );
	nodeVar21 = normalize( mix( normalize( cross( cross( AnisotropyB, positionViewDirection ), AnisotropyB ) ), normalView, ( nodeVar20 * nodeVar20 ) ) );
	nodeVar22 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), nodeVar21 ), nodeVar21, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar23 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar22.x, ( - nodeVar22.y ), nodeVar22.z ), 1.0 ) ).xyz );
	nodeVar24 = max( ( 4.0 - nodeVar18 ), 0.0 );
	nodeVar18 = max( nodeVar18, 4.0 );
	nodeVar25 = exp2( nodeVar18 );
	nodeVar26 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar22.x, ( - nodeVar22.y ), nodeVar22.z ), 1.0 ) ).xyz, nodeVar23 ) * vec2<f32>( ( nodeVar25 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar23 > 2.0 ) ) {

		nodeVar26.y = ( nodeVar26.y + nodeVar25 );
		nodeVar23 = ( nodeVar23 - 3.0 );
		

	}

	nodeVar26.x = ( nodeVar26.x + ( nodeVar23 * nodeVar25 ) );
	nodeVar26.x = ( nodeVar26.x + ( nodeVar24 * ( 3.0 * 16.0 ) ) );
	nodeVar26.y = ( nodeVar26.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar25 ) ) );
	nodeVar26.x = ( nodeVar26.x * object.nodeUniform14 );
	nodeVar26.y = ( nodeVar26.y * object.nodeUniform15 );
	nodeVar27 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar26, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar28 = nodeVar27.xyz;
	nodeVar29 = fract( nodeVar16 );

	if ( ( nodeVar29 != 0.0 ) ) {

		nodeVar30 = ( nodeVar17 + 1.0 );
		nodeVar31 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar22.x, ( - nodeVar22.y ), nodeVar22.z ), 1.0 ) ).xyz );
		nodeVar32 = max( ( 4.0 - nodeVar30 ), 0.0 );
		nodeVar30 = max( nodeVar30, 4.0 );
		nodeVar33 = exp2( nodeVar30 );
		nodeVar34 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar22.x, ( - nodeVar22.y ), nodeVar22.z ), 1.0 ) ).xyz, nodeVar31 ) * vec2<f32>( ( nodeVar33 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar31 > 2.0 ) ) {

			nodeVar34.y = ( nodeVar34.y + nodeVar33 );
			nodeVar31 = ( nodeVar31 - 3.0 );
			

		}

		nodeVar34.x = ( nodeVar34.x + ( nodeVar31 * nodeVar33 ) );
		nodeVar34.x = ( nodeVar34.x + ( nodeVar32 * ( 3.0 * 16.0 ) ) );
		nodeVar34.y = ( nodeVar34.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar33 ) ) );
		nodeVar34.x = ( nodeVar34.x * object.nodeUniform14 );
		nodeVar34.y = ( nodeVar34.y * object.nodeUniform15 );
		nodeVar35 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar34, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar36 = nodeVar35.xyz;
		nodeVar28 = mix( nodeVar28, nodeVar36, nodeVar29 );
		

	}

	nodeVar37 = ( radiance + ( nodeVar28 * vec3<f32>( object.nodeUniform17 ) ) );
	radiance = nodeVar37;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar38 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform11 );
	nodeVar39 = floor( nodeVar38 );
	nodeVar40 = nodeVar39;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar41 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar42 = max( ( 4.0 - nodeVar40 ), 0.0 );
	nodeVar40 = max( nodeVar40, 4.0 );
	nodeVar43 = exp2( nodeVar40 );
	nodeVar44 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar41 ) * vec2<f32>( ( nodeVar43 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar41 > 2.0 ) ) {

		nodeVar44.y = ( nodeVar44.y + nodeVar43 );
		nodeVar41 = ( nodeVar41 - 3.0 );
		

	}

	nodeVar44.x = ( nodeVar44.x + ( nodeVar41 * nodeVar43 ) );
	nodeVar44.x = ( nodeVar44.x + ( nodeVar42 * ( 3.0 * 16.0 ) ) );
	nodeVar44.y = ( nodeVar44.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar43 ) ) );
	nodeVar44.x = ( nodeVar44.x * object.nodeUniform14 );
	nodeVar44.y = ( nodeVar44.y * object.nodeUniform15 );
	nodeVar45 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar44, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar46 = nodeVar45.xyz;
	nodeVar47 = fract( nodeVar38 );

	if ( ( nodeVar47 != 0.0 ) ) {

		nodeVar48 = ( nodeVar39 + 1.0 );
		nodeVar49 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar50 = max( ( 4.0 - nodeVar48 ), 0.0 );
		nodeVar48 = max( nodeVar48, 4.0 );
		nodeVar51 = exp2( nodeVar48 );
		nodeVar52 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar49 ) * vec2<f32>( ( nodeVar51 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar49 > 2.0 ) ) {

			nodeVar52.y = ( nodeVar52.y + nodeVar51 );
			nodeVar49 = ( nodeVar49 - 3.0 );
			

		}

		nodeVar52.x = ( nodeVar52.x + ( nodeVar49 * nodeVar51 ) );
		nodeVar52.x = ( nodeVar52.x + ( nodeVar50 * ( 3.0 * 16.0 ) ) );
		nodeVar52.y = ( nodeVar52.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar51 ) ) );
		nodeVar52.x = ( nodeVar52.x * object.nodeUniform14 );
		nodeVar52.y = ( nodeVar52.y * object.nodeUniform15 );
		nodeVar53 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar52, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar54 = nodeVar53.xyz;
		nodeVar46 = mix( nodeVar46, nodeVar54, nodeVar47 );
		

	}

	nodeVar55 = ( iblIrradiance + ( ( nodeVar46 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform17 ) ) );
	iblIrradiance = nodeVar55;
	nodeVar56 = clamp( roughnessToMip( ClearcoatRoughness ), -2.0, object.nodeUniform11 );
	nodeVar57 = floor( nodeVar56 );
	nodeVar58 = nodeVar57;
	clearcoatNormalView = NORMAL_normalView;
	nodeVar59 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), clearcoatNormalView ), clearcoatNormalView, ( ( ( ClearcoatRoughness * ClearcoatRoughness ) * ClearcoatRoughness ) * ClearcoatRoughness ) ) ), 0.0 ) ).xyz );
	nodeVar60 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar59.x, ( - nodeVar59.y ), nodeVar59.z ), 1.0 ) ).xyz );
	nodeVar61 = max( ( 4.0 - nodeVar58 ), 0.0 );
	nodeVar58 = max( nodeVar58, 4.0 );
	nodeVar62 = exp2( nodeVar58 );
	nodeVar63 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar59.x, ( - nodeVar59.y ), nodeVar59.z ), 1.0 ) ).xyz, nodeVar60 ) * vec2<f32>( ( nodeVar62 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar60 > 2.0 ) ) {

		nodeVar63.y = ( nodeVar63.y + nodeVar62 );
		nodeVar60 = ( nodeVar60 - 3.0 );
		

	}

	nodeVar63.x = ( nodeVar63.x + ( nodeVar60 * nodeVar62 ) );
	nodeVar63.x = ( nodeVar63.x + ( nodeVar61 * ( 3.0 * 16.0 ) ) );
	nodeVar63.y = ( nodeVar63.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar62 ) ) );
	nodeVar63.x = ( nodeVar63.x * object.nodeUniform14 );
	nodeVar63.y = ( nodeVar63.y * object.nodeUniform15 );
	nodeVar64 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar63, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar65 = nodeVar64.xyz;
	nodeVar66 = fract( nodeVar56 );

	if ( ( nodeVar66 != 0.0 ) ) {

		nodeVar67 = ( nodeVar57 + 1.0 );
		nodeVar68 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar59.x, ( - nodeVar59.y ), nodeVar59.z ), 1.0 ) ).xyz );
		nodeVar69 = max( ( 4.0 - nodeVar67 ), 0.0 );
		nodeVar67 = max( nodeVar67, 4.0 );
		nodeVar70 = exp2( nodeVar67 );
		nodeVar71 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar59.x, ( - nodeVar59.y ), nodeVar59.z ), 1.0 ) ).xyz, nodeVar68 ) * vec2<f32>( ( nodeVar70 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar68 > 2.0 ) ) {

			nodeVar71.y = ( nodeVar71.y + nodeVar70 );
			nodeVar68 = ( nodeVar68 - 3.0 );
			

		}

		nodeVar71.x = ( nodeVar71.x + ( nodeVar68 * nodeVar70 ) );
		nodeVar71.x = ( nodeVar71.x + ( nodeVar69 * ( 3.0 * 16.0 ) ) );
		nodeVar71.y = ( nodeVar71.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar70 ) ) );
		nodeVar71.x = ( nodeVar71.x * object.nodeUniform14 );
		nodeVar71.y = ( nodeVar71.y * object.nodeUniform15 );
		nodeVar72 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar71, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar73 = nodeVar72.xyz;
		nodeVar65 = mix( nodeVar65, nodeVar73, nodeVar66 );
		

	}

	nodeVar74 = ( clearcoatRadiance + ( nodeVar65 * vec3<f32>( object.nodeUniform17 ) ) );
	clearcoatRadiance = nodeVar74;
	nodeVar75 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar76 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar77 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar78 = ( SpecularF90 * dfg.y );
	nodeVar79 = ( nodeVar77 + vec3<f32>( nodeVar78 ) );
	nodeVar80 = ( nodeVar75 + nodeVar79 );
	nodeVar75 = nodeVar80;
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
	nodeVar94 = ( nodeVar76 + nodeVar93 );
	nodeVar76 = nodeVar94;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar95 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar96 = ( irradiance * nodeVar95 );
	nodeVar97 = ( nodeVar75 + nodeVar76 );
	nodeVar98 = ( vec3<f32>( 1.0 ) - nodeVar97 );
	nodeVar99 = nodeVar98;
	nodeVar100 = ( nodeVar96 * nodeVar99 );
	nodeVar101 = nodeVar100;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar102 = ( indirectDiffuse + nodeVar101 );
	indirectDiffuse = nodeVar102;
	nodeVar103 = dot( clearcoatNormalView, positionViewDirection );
	nodeVar104 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( ClearcoatRoughness, clamp( nodeVar103, 0.0, 1.0 ) ) ).xy;
	nodeVar105 = ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( nodeVar104.x ) ) + vec3<f32>( ( 1.0 * nodeVar104.y ) ) );
	nodeVar106 = ( clearcoatRadiance * nodeVar105 );
	nodeVar107 = ( clearcoatSpecularIndirect + nodeVar106 );
	clearcoatSpecularIndirect = nodeVar107;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar108 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar109 = ( SpecularF90 * dfg.y );
	nodeVar110 = ( nodeVar108 + vec3<f32>( nodeVar109 ) );
	nodeVar111 = ( singleScatteringDielectric + nodeVar110 );
	singleScatteringDielectric = nodeVar111;
	nodeVar112 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar113 = nodeVar112;
	nodeVar114 = ( nodeVar113 * vec3<f32>( 0.047619 ) );
	nodeVar115 = ( SpecularColor + nodeVar114 );
	nodeVar116 = ( nodeVar110 * nodeVar115 );
	nodeVar117 = ( dfg.x + dfg.y );
	nodeVar118 = ( 1.0 - nodeVar117 );
	nodeVar119 = nodeVar118;
	nodeVar120 = ( vec3<f32>( nodeVar119 ) * nodeVar115 );
	nodeVar121 = ( vec3<f32>( 1.0 ) - nodeVar120 );
	nodeVar122 = nodeVar121;
	nodeVar123 = ( nodeVar116 / nodeVar122 );
	nodeVar124 = ( nodeVar123 * vec3<f32>( nodeVar119 ) );
	nodeVar125 = ( multiScatteringDielectric + nodeVar124 );
	multiScatteringDielectric = nodeVar125;
	nodeVar126 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar127 = ( SpecularF90 * dfg.y );
	nodeVar128 = ( nodeVar126 + vec3<f32>( nodeVar127 ) );
	nodeVar129 = ( singleScatteringMetallic + nodeVar128 );
	singleScatteringMetallic = nodeVar129;
	nodeVar130 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( nodeVar131 * vec3<f32>( 0.047619 ) );
	nodeVar133 = ( DiffuseColor.xyz + nodeVar132 );
	nodeVar134 = ( nodeVar128 * nodeVar133 );
	nodeVar135 = ( dfg.x + dfg.y );
	nodeVar136 = ( 1.0 - nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( vec3<f32>( nodeVar137 ) * nodeVar133 );
	nodeVar139 = ( vec3<f32>( 1.0 ) - nodeVar138 );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( nodeVar134 / nodeVar140 );
	nodeVar142 = ( nodeVar141 * vec3<f32>( nodeVar137 ) );
	nodeVar143 = ( multiScatteringMetallic + nodeVar142 );
	multiScatteringMetallic = nodeVar143;
	nodeVar144 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar145 = ( radiance * nodeVar144 );
	nodeVar146 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar147 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar148 = ( nodeVar146 * nodeVar147 );
	nodeVar149 = ( nodeVar145 + nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar152 = ( vec3<f32>( 1.0 ) - nodeVar151 );
	nodeVar153 = nodeVar152;
	nodeVar154 = ( DiffuseContribution * nodeVar153 );
	nodeVar155 = ( nodeVar154 * nodeVar147 );
	nodeVar156 = nodeVar155;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar157 = ( indirectSpecular + nodeVar150 );
	indirectSpecular = nodeVar157;
	nodeVar158 = ( indirectDiffuse + nodeVar156 );
	indirectDiffuse = nodeVar158;
	ambientOcclusion = 1.0;
	nodeVar159 = ( clearcoatSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	clearcoatSpecularIndirect = nodeVar159;
	nodeVar160 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar160;
	nodeVar161 = dot( normalView, positionViewDirection );
	nodeVar162 = ( clamp( nodeVar161, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar163 = ( Roughness * -16.0 );
	nodeVar164 = ( 1.0 - nodeVar163 );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( - nodeVar165 );
	nodeVar167 = exp2( nodeVar166 );
	nodeVar168 = pow( nodeVar162, nodeVar167 );
	nodeVar169 = ( 1.0 - nodeVar168 );
	nodeVar170 = nodeVar169;
	nodeVar171 = ( ambientOcclusion - nodeVar170 );
	nodeVar172 = ( indirectSpecular * vec3<f32>( clamp( nodeVar171, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar172;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar173 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar173;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar174 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar174;
	nodeVar175 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar175;
	nodeVar176 = dot( clearcoatNormalView, positionViewDirection );
	nodeVar177 = clamp( nodeVar176, 0.0, 1.0 );
	nodeVar178 = exp2( ( ( ( nodeVar177 * -5.55473 ) - 6.98316 ) * nodeVar177 ) );
	nodeVar179 = ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( ( 1.0 - nodeVar178 ) ) ) + vec3<f32>( ( 1.0 * nodeVar178 ) ) );
	nodeVar180 = ( vec3<f32>( Clearcoat ) * nodeVar179 );
	nodeVar181 = ( vec3<f32>( 1.0 ) - nodeVar180 );
	nodeVar182 = nodeVar181;
	nodeVar183 = ( outgoingLight * nodeVar182 );
	nodeVar184 = ( clearcoatSpecularDirect + clearcoatSpecularIndirect );
	nodeVar185 = ( nodeVar184 * vec3<f32>( Clearcoat ) );
	nodeVar186 = ( nodeVar183 + nodeVar185 );
	outgoingLight = nodeVar186;
	nodeVar187 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar187;

	// result

	output.color = nodeVar187;

	return output;

}
