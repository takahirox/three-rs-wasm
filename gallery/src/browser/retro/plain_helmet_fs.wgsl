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
@binding( 5 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 12 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;
@binding( 13 ) @group( 1 ) var nodeUniform37_sampler : sampler;
@binding( 14 ) @group( 1 ) var nodeUniform37 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform21 : mat3x3<f32>,
	nodeUniform22 : vec2<f32>,
	nodeUniform32 : f32,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform38 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
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
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec2<f32>;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : vec2<f32>;
var<private> nodeVar99 : vec4<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : vec2<f32>;
var<private> nodeVar109 : vec4<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec2<f32>;
var<private> nodeVar117 : vec4<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
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
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : f32;
var<private> nodeVar203 : f32;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : f32;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : f32;
var<private> nodeVar209 : f32;
var<private> nodeVar210 : f32;
var<private> nodeVar211 : f32;
var<private> nodeVar212 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : vec4<f32>;

// codes
fn V_GGX_SmithCorrelated ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn D_GGX ( alpha : f32, dotNH : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;

	nodeVar0 = ( alpha * alpha );
	nodeVar1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeVar0 ) ) );

	return ( ( nodeVar0 / ( nodeVar1 * nodeVar1 ) ) * 0.3183098861837907 );

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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	AmbientOcclusion = ( ( ( nodeVar1.x - 1.0 ) * object.nodeUniform6 ) + 1.0 );
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform11 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform10 * nodeVar3.y ), 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	nodeVar5 = textureSample( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	EmissiveColor = ( vec4<f32>( ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) ), 1.0 ) * nodeVar5 ).xyz;
	NORMAL_normalView = normalViewGeometry;
	nodeVar6 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	nodeVar7 = dpdx( nodeVarying6 );
	nodeVar8 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	nodeVar9 = - dpdy( nodeVarying6 );
	nodeVar10 = ( ( nodeVar6 * vec3<f32>( nodeVar7.x ) ) + ( nodeVar8 * vec3<f32>( nodeVar9.x ) ) );
	nodeVar12 = ( ( nodeVar6 * vec3<f32>( nodeVar7.y ) ) + ( nodeVar8 * vec3<f32>( nodeVar9.y ) ) );
	nodeVar13 = max( dot( nodeVar10, nodeVar10 ), dot( nodeVar12, nodeVar12 ) );

	if ( ( nodeVar13 == 0.0 ) ) {

		nodeVar11 = 0.0;

	} else {

		nodeVar11 = inverseSqrt( nodeVar13 );

	}

	tangentViewFrame = ( nodeVar10 * vec3<f32>( nodeVar11 ) );
	NORMAL_tangentView = tangentViewFrame;
	bitangentViewFrame = ( nodeVar12 * nodeVar11 );
	NORMAL_bitangentView = bitangentViewFrame;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar14 = textureSample( nodeUniform20, nodeUniform20_sampler, ( object.nodeUniform21 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	nodeVar15 = ( ( nodeVar14 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar15.xy * object.nodeUniform22 ), nodeVar15.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar16 = dot( normalView, positionViewDirection );
	nodeVar17 = textureSample( nodeUniform18, nodeUniform18_sampler, vec2<f32>( Roughness, clamp( nodeVar16, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar17;
	nodeVar18 = ( dfg.x + dfg.y );
	nodeVar19 = ( 1.0 / nodeVar18 );
	nodeVar20 = nodeVar19;
	nodeVar21 = ( nodeVar20 - 1.0 );
	nodeVar22 = ( SpecularColorBlended * vec3<f32>( nodeVar21 ) );
	nodeVar23 = ( nodeVar22 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar23;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar24 = ( irradiance + render.nodeUniform23 );
	irradiance = nodeVar24;
	nodeVar25 = ( render.nodeUniform25 - render.nodeUniform26 );
	nodeVar26 = vec4<f32>( nodeVar25, 0.0 );
	nodeVar27 = ( render.cameraViewMatrix * nodeVar26 );
	nodeVar28 = normalize( nodeVar27.xyz );
	nodeVar29 = nodeVar28;
	nodeVar30 = dot( normalView, nodeVar29 );
	nodeVar31 = ( vec3<f32>( clamp( nodeVar30, 0.0, 1.0 ) ) * render.nodeUniform27 );
	nodeVar32 = nodeVar31;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar33 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar34 = ( nodeVar32 * nodeVar33 );
	nodeVar35 = ( nodeVar29 + positionViewDirection );
	nodeVar36 = normalize( nodeVar35 );
	nodeVar37 = dot( positionViewDirection, nodeVar36 );
	nodeVar38 = clamp( nodeVar37, 0.0, 1.0 );
	nodeVar39 = exp2( ( ( ( nodeVar38 * -5.55473 ) - 6.98316 ) * nodeVar38 ) );
	nodeVar40 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar39 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar39 ) ) );
	nodeVar41 = ( vec3<f32>( 1.0 ) - nodeVar40 );
	nodeVar42 = nodeVar41;
	nodeVar43 = ( nodeVar34 * nodeVar42 );
	nodeVar44 = ( directDiffuse + nodeVar43 );
	directDiffuse = nodeVar44;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar45 = normalize( ( nodeVar29 + positionViewDirection ) );
	nodeVar46 = clamp( dot( positionViewDirection, nodeVar45 ), 0.0, 1.0 );
	nodeVar47 = exp2( ( ( ( nodeVar46 * -5.55473 ) - 6.98316 ) * nodeVar46 ) );
	nodeVar48 = ( Roughness * Roughness );
	nodeVar49 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar47 ) ) ) + vec3<f32>( ( 1.0 * nodeVar47 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar48, clamp( dot( normalView, nodeVar29 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar48, clamp( dot( normalView, nodeVar45 ), 0.0, 1.0 ) ) ) );
	nodeVar50 = ( nodeVar32 * nodeVar49 );
	nodeVar51 = ( nodeVar50 * multiScatteringCompensation );
	nodeVar52 = ( directSpecular + nodeVar51 );
	directSpecular = nodeVar52;
	nodeVar53 = ( render.nodeUniform28 - v_positionView );
	nodeVar54 = normalize( nodeVar53 );
	nodeVar55 = dot( normalView, nodeVar54 );

	if ( ( render.nodeUniform30 > 0.0 ) ) {

		nodeVar57 = length( nodeVar53 );
		nodeVar58 = ( nodeVar57 / render.nodeUniform30 );
		nodeVar59 = clamp( ( 1.0 - ( ( ( nodeVar58 * nodeVar58 ) * nodeVar58 ) * nodeVar58 ) ), 0.0, 1.0 );
		nodeVar56 = ( ( 1.0 / max( pow( nodeVar57, render.nodeUniform31 ), 0.01 ) ) * ( nodeVar59 * nodeVar59 ) );

	} else {

		nodeVar56 = ( 1.0 / max( pow( length( nodeVar53 ), render.nodeUniform31 ), 0.01 ) );

	}

	nodeVar60 = ( render.nodeUniform29 * vec3<f32>( nodeVar56 ) );
	nodeVar61 = ( vec3<f32>( clamp( nodeVar55, 0.0, 1.0 ) ) * nodeVar60 );
	nodeVar62 = nodeVar61;
	nodeVar63 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar64 = ( nodeVar62 * nodeVar63 );
	nodeVar65 = ( nodeVar54 + positionViewDirection );
	nodeVar66 = normalize( nodeVar65 );
	nodeVar67 = dot( positionViewDirection, nodeVar66 );
	nodeVar68 = clamp( nodeVar67, 0.0, 1.0 );
	nodeVar69 = exp2( ( ( ( nodeVar68 * -5.55473 ) - 6.98316 ) * nodeVar68 ) );
	nodeVar70 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar69 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar69 ) ) );
	nodeVar71 = ( vec3<f32>( 1.0 ) - nodeVar70 );
	nodeVar72 = nodeVar71;
	nodeVar73 = ( nodeVar64 * nodeVar72 );
	nodeVar74 = ( directDiffuse + nodeVar73 );
	directDiffuse = nodeVar74;
	nodeVar75 = normalize( ( nodeVar54 + positionViewDirection ) );
	nodeVar76 = clamp( dot( positionViewDirection, nodeVar75 ), 0.0, 1.0 );
	nodeVar77 = exp2( ( ( ( nodeVar76 * -5.55473 ) - 6.98316 ) * nodeVar76 ) );
	nodeVar78 = ( Roughness * Roughness );
	nodeVar79 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar77 ) ) ) + vec3<f32>( ( 1.0 * nodeVar77 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar78, clamp( dot( normalView, nodeVar54 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar78, clamp( dot( normalView, nodeVar75 ), 0.0, 1.0 ) ) ) );
	nodeVar80 = ( nodeVar62 * nodeVar79 );
	nodeVar81 = ( nodeVar80 * multiScatteringCompensation );
	nodeVar82 = ( directSpecular + nodeVar81 );
	directSpecular = nodeVar82;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar83 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform32 );
	nodeVar84 = floor( nodeVar83 );
	nodeVar85 = nodeVar84;
	nodeVar86 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar87 = getFace( ( object.nodeUniform33 * vec4<f32>( vec3<f32>( nodeVar86.x, ( - nodeVar86.y ), nodeVar86.z ), 1.0 ) ).xyz );
	nodeVar88 = max( ( 4.0 - nodeVar85 ), 0.0 );
	nodeVar85 = max( nodeVar85, 4.0 );
	nodeVar89 = exp2( nodeVar85 );
	nodeVar90 = ( ( getUV( ( object.nodeUniform33 * vec4<f32>( vec3<f32>( nodeVar86.x, ( - nodeVar86.y ), nodeVar86.z ), 1.0 ) ).xyz, nodeVar87 ) * vec2<f32>( ( nodeVar89 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar87 > 2.0 ) ) {

		nodeVar90.y = ( nodeVar90.y + nodeVar89 );
		nodeVar87 = ( nodeVar87 - 3.0 );
		

	}

	nodeVar90.x = ( nodeVar90.x + ( nodeVar87 * nodeVar89 ) );
	nodeVar90.x = ( nodeVar90.x + ( nodeVar88 * ( 3.0 * 16.0 ) ) );
	nodeVar90.y = ( nodeVar90.y + ( 4.0 * ( exp2( object.nodeUniform32 ) - nodeVar89 ) ) );
	nodeVar90.x = ( nodeVar90.x * object.nodeUniform35 );
	nodeVar90.y = ( nodeVar90.y * object.nodeUniform36 );
	nodeVar91 = textureSampleGrad( nodeUniform37, nodeUniform37_sampler, nodeVar90, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar92 = nodeVar91.xyz;
	nodeVar93 = fract( nodeVar83 );

	if ( ( nodeVar93 != 0.0 ) ) {

		nodeVar94 = ( nodeVar84 + 1.0 );
		nodeVar95 = getFace( ( object.nodeUniform33 * vec4<f32>( vec3<f32>( nodeVar86.x, ( - nodeVar86.y ), nodeVar86.z ), 1.0 ) ).xyz );
		nodeVar96 = max( ( 4.0 - nodeVar94 ), 0.0 );
		nodeVar94 = max( nodeVar94, 4.0 );
		nodeVar97 = exp2( nodeVar94 );
		nodeVar98 = ( ( getUV( ( object.nodeUniform33 * vec4<f32>( vec3<f32>( nodeVar86.x, ( - nodeVar86.y ), nodeVar86.z ), 1.0 ) ).xyz, nodeVar95 ) * vec2<f32>( ( nodeVar97 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar95 > 2.0 ) ) {

			nodeVar98.y = ( nodeVar98.y + nodeVar97 );
			nodeVar95 = ( nodeVar95 - 3.0 );
			

		}

		nodeVar98.x = ( nodeVar98.x + ( nodeVar95 * nodeVar97 ) );
		nodeVar98.x = ( nodeVar98.x + ( nodeVar96 * ( 3.0 * 16.0 ) ) );
		nodeVar98.y = ( nodeVar98.y + ( 4.0 * ( exp2( object.nodeUniform32 ) - nodeVar97 ) ) );
		nodeVar98.x = ( nodeVar98.x * object.nodeUniform35 );
		nodeVar98.y = ( nodeVar98.y * object.nodeUniform36 );
		nodeVar99 = textureSampleGrad( nodeUniform37, nodeUniform37_sampler, nodeVar98, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar100 = nodeVar99.xyz;
		nodeVar92 = mix( nodeVar92, nodeVar100, nodeVar93 );
		

	}

	nodeVar101 = ( radiance + ( nodeVar92 * vec3<f32>( object.nodeUniform38 ) ) );
	radiance = nodeVar101;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar102 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform32 );
	nodeVar103 = floor( nodeVar102 );
	nodeVar104 = nodeVar103;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar105 = getFace( ( object.nodeUniform33 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar106 = max( ( 4.0 - nodeVar104 ), 0.0 );
	nodeVar104 = max( nodeVar104, 4.0 );
	nodeVar107 = exp2( nodeVar104 );
	nodeVar108 = ( ( getUV( ( object.nodeUniform33 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar105 ) * vec2<f32>( ( nodeVar107 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar105 > 2.0 ) ) {

		nodeVar108.y = ( nodeVar108.y + nodeVar107 );
		nodeVar105 = ( nodeVar105 - 3.0 );
		

	}

	nodeVar108.x = ( nodeVar108.x + ( nodeVar105 * nodeVar107 ) );
	nodeVar108.x = ( nodeVar108.x + ( nodeVar106 * ( 3.0 * 16.0 ) ) );
	nodeVar108.y = ( nodeVar108.y + ( 4.0 * ( exp2( object.nodeUniform32 ) - nodeVar107 ) ) );
	nodeVar108.x = ( nodeVar108.x * object.nodeUniform35 );
	nodeVar108.y = ( nodeVar108.y * object.nodeUniform36 );
	nodeVar109 = textureSampleGrad( nodeUniform37, nodeUniform37_sampler, nodeVar108, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar110 = nodeVar109.xyz;
	nodeVar111 = fract( nodeVar102 );

	if ( ( nodeVar111 != 0.0 ) ) {

		nodeVar112 = ( nodeVar103 + 1.0 );
		nodeVar113 = getFace( ( object.nodeUniform33 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar114 = max( ( 4.0 - nodeVar112 ), 0.0 );
		nodeVar112 = max( nodeVar112, 4.0 );
		nodeVar115 = exp2( nodeVar112 );
		nodeVar116 = ( ( getUV( ( object.nodeUniform33 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar113 ) * vec2<f32>( ( nodeVar115 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar113 > 2.0 ) ) {

			nodeVar116.y = ( nodeVar116.y + nodeVar115 );
			nodeVar113 = ( nodeVar113 - 3.0 );
			

		}

		nodeVar116.x = ( nodeVar116.x + ( nodeVar113 * nodeVar115 ) );
		nodeVar116.x = ( nodeVar116.x + ( nodeVar114 * ( 3.0 * 16.0 ) ) );
		nodeVar116.y = ( nodeVar116.y + ( 4.0 * ( exp2( object.nodeUniform32 ) - nodeVar115 ) ) );
		nodeVar116.x = ( nodeVar116.x * object.nodeUniform35 );
		nodeVar116.y = ( nodeVar116.y * object.nodeUniform36 );
		nodeVar117 = textureSampleGrad( nodeUniform37, nodeUniform37_sampler, nodeVar116, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar118 = nodeVar117.xyz;
		nodeVar110 = mix( nodeVar110, nodeVar118, nodeVar111 );
		

	}

	nodeVar119 = ( iblIrradiance + ( ( nodeVar110 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform38 ) ) );
	iblIrradiance = nodeVar119;
	ambientOcclusion = 1.0;
	nodeVar120 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar120;
	nodeVar121 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar122 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar123 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar124 = ( SpecularF90 * dfg.y );
	nodeVar125 = ( nodeVar123 + vec3<f32>( nodeVar124 ) );
	nodeVar126 = ( nodeVar121 + nodeVar125 );
	nodeVar121 = nodeVar126;
	nodeVar127 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar128 = nodeVar127;
	nodeVar129 = ( nodeVar128 * vec3<f32>( 0.047619 ) );
	nodeVar130 = ( SpecularColor + nodeVar129 );
	nodeVar131 = ( nodeVar125 * nodeVar130 );
	nodeVar132 = ( dfg.x + dfg.y );
	nodeVar133 = ( 1.0 - nodeVar132 );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( vec3<f32>( nodeVar134 ) * nodeVar130 );
	nodeVar136 = ( vec3<f32>( 1.0 ) - nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( nodeVar131 / nodeVar137 );
	nodeVar139 = ( nodeVar138 * vec3<f32>( nodeVar134 ) );
	nodeVar140 = ( nodeVar122 + nodeVar139 );
	nodeVar122 = nodeVar140;
	nodeVar141 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar142 = ( irradiance * nodeVar141 );
	nodeVar143 = ( nodeVar121 + nodeVar122 );
	nodeVar144 = ( vec3<f32>( 1.0 ) - nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( nodeVar142 * nodeVar145 );
	nodeVar147 = nodeVar146;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar148 = ( indirectDiffuse + nodeVar147 );
	indirectDiffuse = nodeVar148;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar149 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar150 = ( SpecularF90 * dfg.y );
	nodeVar151 = ( nodeVar149 + vec3<f32>( nodeVar150 ) );
	nodeVar152 = ( singleScatteringDielectric + nodeVar151 );
	singleScatteringDielectric = nodeVar152;
	nodeVar153 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar154 = nodeVar153;
	nodeVar155 = ( nodeVar154 * vec3<f32>( 0.047619 ) );
	nodeVar156 = ( SpecularColor + nodeVar155 );
	nodeVar157 = ( nodeVar151 * nodeVar156 );
	nodeVar158 = ( dfg.x + dfg.y );
	nodeVar159 = ( 1.0 - nodeVar158 );
	nodeVar160 = nodeVar159;
	nodeVar161 = ( vec3<f32>( nodeVar160 ) * nodeVar156 );
	nodeVar162 = ( vec3<f32>( 1.0 ) - nodeVar161 );
	nodeVar163 = nodeVar162;
	nodeVar164 = ( nodeVar157 / nodeVar163 );
	nodeVar165 = ( nodeVar164 * vec3<f32>( nodeVar160 ) );
	nodeVar166 = ( multiScatteringDielectric + nodeVar165 );
	multiScatteringDielectric = nodeVar166;
	nodeVar167 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar168 = ( SpecularF90 * dfg.y );
	nodeVar169 = ( nodeVar167 + vec3<f32>( nodeVar168 ) );
	nodeVar170 = ( singleScatteringMetallic + nodeVar169 );
	singleScatteringMetallic = nodeVar170;
	nodeVar171 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar172 = nodeVar171;
	nodeVar173 = ( nodeVar172 * vec3<f32>( 0.047619 ) );
	nodeVar174 = ( DiffuseColor.xyz + nodeVar173 );
	nodeVar175 = ( nodeVar169 * nodeVar174 );
	nodeVar176 = ( dfg.x + dfg.y );
	nodeVar177 = ( 1.0 - nodeVar176 );
	nodeVar178 = nodeVar177;
	nodeVar179 = ( vec3<f32>( nodeVar178 ) * nodeVar174 );
	nodeVar180 = ( vec3<f32>( 1.0 ) - nodeVar179 );
	nodeVar181 = nodeVar180;
	nodeVar182 = ( nodeVar175 / nodeVar181 );
	nodeVar183 = ( nodeVar182 * vec3<f32>( nodeVar178 ) );
	nodeVar184 = ( multiScatteringMetallic + nodeVar183 );
	multiScatteringMetallic = nodeVar184;
	nodeVar185 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar186 = ( radiance * nodeVar185 );
	nodeVar187 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar188 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar189 = ( nodeVar187 * nodeVar188 );
	nodeVar190 = ( nodeVar186 + nodeVar189 );
	nodeVar191 = nodeVar190;
	nodeVar192 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar193 = ( vec3<f32>( 1.0 ) - nodeVar192 );
	nodeVar194 = nodeVar193;
	nodeVar195 = ( DiffuseContribution * nodeVar194 );
	nodeVar196 = ( nodeVar195 * nodeVar188 );
	nodeVar197 = nodeVar196;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar198 = ( indirectSpecular + nodeVar191 );
	indirectSpecular = nodeVar198;
	nodeVar199 = ( indirectDiffuse + nodeVar197 );
	indirectDiffuse = nodeVar199;
	nodeVar200 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar200;
	nodeVar201 = dot( normalView, positionViewDirection );
	nodeVar202 = ( clamp( nodeVar201, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar203 = ( Roughness * -16.0 );
	nodeVar204 = ( 1.0 - nodeVar203 );
	nodeVar205 = nodeVar204;
	nodeVar206 = ( - nodeVar205 );
	nodeVar207 = exp2( nodeVar206 );
	nodeVar208 = pow( nodeVar202, nodeVar207 );
	nodeVar209 = ( 1.0 - nodeVar208 );
	nodeVar210 = nodeVar209;
	nodeVar211 = ( ambientOcclusion - nodeVar210 );
	nodeVar212 = ( indirectSpecular * vec3<f32>( clamp( nodeVar211, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar212;
	nodeVar213 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar213;
	nodeVar214 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar214;
	nodeVar215 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar215;
	nodeVar216 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar216;

	// result

	output.color = nodeVar216;

	return output;

}
