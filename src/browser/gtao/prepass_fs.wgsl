// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform40_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform40 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : mat4x4<f32>,
	nodeUniform38 : f32,
	nodeUniform39 : f32,
	nodeUniform41 : f32,
	nodeUniform42 : mat4x4<f32>,
	nodeUniform43 : mat4x4<f32>,
	nodeUniform45 : mat4x4<f32>,
	nodeUniform46 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform44 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform20 : vec3<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform33 : f32,
	nodeUniform34 : f32,
	nodeUniform28 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec4<f32>;
var<private> nodeVar90 : vec4<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : vec2<f32>;
var<private> nodeVar131 : vec4<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : vec2<f32>;
var<private> nodeVar139 : vec4<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : vec2<f32>;
var<private> nodeVar149 : vec4<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : vec2<f32>;
var<private> nodeVar157 : vec4<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : f32;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : f32;
var<private> nodeVar216 : f32;
var<private> nodeVar217 : f32;
var<private> nodeVar218 : vec3<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : vec3<f32>;
var<private> nodeVar222 : vec3<f32>;
var<private> nodeVar223 : vec3<f32>;
var<private> nodeVar224 : vec3<f32>;
var<private> nodeVar225 : vec3<f32>;
var<private> nodeVar226 : vec3<f32>;
var<private> nodeVar227 : vec3<f32>;
var<private> nodeVar228 : vec3<f32>;
var<private> nodeVar229 : vec3<f32>;
var<private> nodeVar230 : vec3<f32>;
var<private> nodeVar231 : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> nodeVar233 : vec3<f32>;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : vec3<f32>;
var<private> nodeVar236 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar237 : vec3<f32>;
var<private> nodeVar238 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar239 : vec3<f32>;
var<private> nodeVar240 : f32;
var<private> nodeVar241 : f32;
var<private> nodeVar242 : f32;
var<private> nodeVar243 : f32;
var<private> nodeVar244 : f32;
var<private> nodeVar245 : f32;
var<private> nodeVar246 : f32;
var<private> nodeVar247 : f32;
var<private> nodeVar248 : f32;
var<private> nodeVar249 : f32;
var<private> nodeVar250 : f32;
var<private> nodeVar251 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar252 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar253 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar254 : vec3<f32>;
var<private> nodeVar255 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar256 : vec4<f32>;
var<private> nodeVar257 : vec4<f32>;
var<private> nodeVar258 : vec2<f32>;

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
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) positionPrevious : vec3<f32> ) -> OutputType {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = dot( normalView, positionViewDirection );
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar1, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	nodeVar3 = ( dfg.x + dfg.y );
	nodeVar4 = ( 1.0 / nodeVar3 );
	nodeVar5 = nodeVar4;
	nodeVar6 = ( nodeVar5 - 1.0 );
	nodeVar7 = ( SpecularColorBlended * vec3<f32>( nodeVar6 ) );
	nodeVar8 = ( nodeVar7 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar8;
	nodeVar9 = ( render.nodeUniform10 - v_positionView );
	nodeVar10 = normalize( nodeVar9 );
	nodeVar11 = dot( normalView, nodeVar10 );
	nodeVar12 = ( render.nodeUniform15 - render.nodeUniform16 );
	nodeVar13 = vec4<f32>( nodeVar12, 0.0 );
	nodeVar14 = ( render.cameraViewMatrix * nodeVar13 );
	nodeVar15 = normalize( nodeVar14.xyz );
	nodeVar16 = nodeVar15;
	nodeVar17 = dot( nodeVar10, nodeVar16 );
	nodeVar18 = smoothstep( render.nodeUniform12, render.nodeUniform13, nodeVar17 );
	nodeVar19 = ( render.nodeUniform11 * vec3<f32>( nodeVar18 ) );

	if ( ( render.nodeUniform17 > 0.0 ) ) {

		nodeVar21 = length( nodeVar9 );
		nodeVar22 = ( nodeVar21 / render.nodeUniform17 );
		nodeVar23 = clamp( ( 1.0 - ( ( ( nodeVar22 * nodeVar22 ) * nodeVar22 ) * nodeVar22 ) ), 0.0, 1.0 );
		nodeVar20 = ( ( 1.0 / max( pow( nodeVar21, render.nodeUniform18 ), 0.01 ) ) * ( nodeVar23 * nodeVar23 ) );

	} else {

		nodeVar20 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform18 ), 0.01 ) );

	}

	nodeVar24 = ( nodeVar19 * vec3<f32>( nodeVar20 ) );
	nodeVar25 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar24 );
	nodeVar26 = nodeVar25;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar27 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar28 = ( nodeVar26 * nodeVar27 );
	nodeVar29 = ( nodeVar10 + positionViewDirection );
	nodeVar30 = normalize( nodeVar29 );
	nodeVar31 = dot( positionViewDirection, nodeVar30 );
	nodeVar32 = clamp( nodeVar31, 0.0, 1.0 );
	nodeVar33 = exp2( ( ( ( nodeVar32 * -5.55473 ) - 6.98316 ) * nodeVar32 ) );
	nodeVar34 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar33 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar33 ) ) );
	nodeVar35 = ( vec3<f32>( 1.0 ) - nodeVar34 );
	nodeVar36 = nodeVar35;
	nodeVar37 = ( nodeVar28 * nodeVar36 );
	nodeVar38 = ( directDiffuse + nodeVar37 );
	directDiffuse = nodeVar38;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar39 = normalize( ( nodeVar10 + positionViewDirection ) );
	nodeVar40 = clamp( dot( positionViewDirection, nodeVar39 ), 0.0, 1.0 );
	nodeVar41 = exp2( ( ( ( nodeVar40 * -5.55473 ) - 6.98316 ) * nodeVar40 ) );
	nodeVar42 = ( Roughness * Roughness );
	nodeVar43 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar41 ) ) ) + vec3<f32>( ( 1.0 * nodeVar41 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar42, clamp( dot( normalView, nodeVar10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar42, clamp( dot( normalView, nodeVar39 ), 0.0, 1.0 ) ) ) );
	nodeVar44 = ( nodeVar26 * nodeVar43 );
	nodeVar45 = ( nodeVar44 * multiScatteringCompensation );
	nodeVar46 = ( directSpecular + nodeVar45 );
	directSpecular = nodeVar46;
	nodeVar47 = ( render.nodeUniform19 - v_positionView );
	nodeVar48 = normalize( nodeVar47 );
	nodeVar49 = dot( normalView, nodeVar48 );
	nodeVar50 = ( render.nodeUniform23 - render.nodeUniform24 );
	nodeVar51 = vec4<f32>( nodeVar50, 0.0 );
	nodeVar52 = ( render.cameraViewMatrix * nodeVar51 );
	nodeVar53 = normalize( nodeVar52.xyz );
	nodeVar54 = nodeVar53;
	nodeVar55 = dot( nodeVar48, nodeVar54 );
	nodeVar56 = smoothstep( render.nodeUniform21, render.nodeUniform22, nodeVar55 );
	nodeVar57 = ( render.nodeUniform20 * vec3<f32>( nodeVar56 ) );

	if ( ( render.nodeUniform25 > 0.0 ) ) {

		nodeVar59 = length( nodeVar47 );
		nodeVar60 = ( nodeVar59 / render.nodeUniform25 );
		nodeVar61 = clamp( ( 1.0 - ( ( ( nodeVar60 * nodeVar60 ) * nodeVar60 ) * nodeVar60 ) ), 0.0, 1.0 );
		nodeVar58 = ( ( 1.0 / max( pow( nodeVar59, render.nodeUniform26 ), 0.01 ) ) * ( nodeVar61 * nodeVar61 ) );

	} else {

		nodeVar58 = ( 1.0 / max( pow( length( nodeVar47 ), render.nodeUniform26 ), 0.01 ) );

	}

	nodeVar62 = ( nodeVar57 * vec3<f32>( nodeVar58 ) );
	nodeVar63 = ( vec3<f32>( clamp( nodeVar49, 0.0, 1.0 ) ) * nodeVar62 );
	nodeVar64 = nodeVar63;
	nodeVar65 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar66 = ( nodeVar64 * nodeVar65 );
	nodeVar67 = ( nodeVar48 + positionViewDirection );
	nodeVar68 = normalize( nodeVar67 );
	nodeVar69 = dot( positionViewDirection, nodeVar68 );
	nodeVar70 = clamp( nodeVar69, 0.0, 1.0 );
	nodeVar71 = exp2( ( ( ( nodeVar70 * -5.55473 ) - 6.98316 ) * nodeVar70 ) );
	nodeVar72 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar71 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar71 ) ) );
	nodeVar73 = ( vec3<f32>( 1.0 ) - nodeVar72 );
	nodeVar74 = nodeVar73;
	nodeVar75 = ( nodeVar66 * nodeVar74 );
	nodeVar76 = ( directDiffuse + nodeVar75 );
	directDiffuse = nodeVar76;
	nodeVar77 = normalize( ( nodeVar48 + positionViewDirection ) );
	nodeVar78 = clamp( dot( positionViewDirection, nodeVar77 ), 0.0, 1.0 );
	nodeVar79 = exp2( ( ( ( nodeVar78 * -5.55473 ) - 6.98316 ) * nodeVar78 ) );
	nodeVar80 = ( Roughness * Roughness );
	nodeVar81 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar79 ) ) ) + vec3<f32>( ( 1.0 * nodeVar79 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar80, clamp( dot( normalView, nodeVar48 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar80, clamp( dot( normalView, nodeVar77 ), 0.0, 1.0 ) ) ) );
	nodeVar82 = ( nodeVar64 * nodeVar81 );
	nodeVar83 = ( nodeVar82 * multiScatteringCompensation );
	nodeVar84 = ( directSpecular + nodeVar83 );
	directSpecular = nodeVar84;
	nodeVar85 = ( render.nodeUniform27 - v_positionView );
	nodeVar86 = normalize( nodeVar85 );
	nodeVar87 = dot( normalView, nodeVar86 );
	nodeVar88 = ( render.nodeUniform31 - render.nodeUniform32 );
	nodeVar89 = vec4<f32>( nodeVar88, 0.0 );
	nodeVar90 = ( render.cameraViewMatrix * nodeVar89 );
	nodeVar91 = normalize( nodeVar90.xyz );
	nodeVar92 = nodeVar91;
	nodeVar93 = dot( nodeVar86, nodeVar92 );
	nodeVar94 = smoothstep( render.nodeUniform29, render.nodeUniform30, nodeVar93 );
	nodeVar95 = ( render.nodeUniform28 * vec3<f32>( nodeVar94 ) );

	if ( ( render.nodeUniform33 > 0.0 ) ) {

		nodeVar97 = length( nodeVar85 );
		nodeVar98 = ( nodeVar97 / render.nodeUniform33 );
		nodeVar99 = clamp( ( 1.0 - ( ( ( nodeVar98 * nodeVar98 ) * nodeVar98 ) * nodeVar98 ) ), 0.0, 1.0 );
		nodeVar96 = ( ( 1.0 / max( pow( nodeVar97, render.nodeUniform34 ), 0.01 ) ) * ( nodeVar99 * nodeVar99 ) );

	} else {

		nodeVar96 = ( 1.0 / max( pow( length( nodeVar85 ), render.nodeUniform34 ), 0.01 ) );

	}

	nodeVar100 = ( nodeVar95 * vec3<f32>( nodeVar96 ) );
	nodeVar101 = ( vec3<f32>( clamp( nodeVar87, 0.0, 1.0 ) ) * nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar104 = ( nodeVar102 * nodeVar103 );
	nodeVar105 = ( nodeVar86 + positionViewDirection );
	nodeVar106 = normalize( nodeVar105 );
	nodeVar107 = dot( positionViewDirection, nodeVar106 );
	nodeVar108 = clamp( nodeVar107, 0.0, 1.0 );
	nodeVar109 = exp2( ( ( ( nodeVar108 * -5.55473 ) - 6.98316 ) * nodeVar108 ) );
	nodeVar110 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar109 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar109 ) ) );
	nodeVar111 = ( vec3<f32>( 1.0 ) - nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( nodeVar104 * nodeVar112 );
	nodeVar114 = ( directDiffuse + nodeVar113 );
	directDiffuse = nodeVar114;
	nodeVar115 = normalize( ( nodeVar86 + positionViewDirection ) );
	nodeVar116 = clamp( dot( positionViewDirection, nodeVar115 ), 0.0, 1.0 );
	nodeVar117 = exp2( ( ( ( nodeVar116 * -5.55473 ) - 6.98316 ) * nodeVar116 ) );
	nodeVar118 = ( Roughness * Roughness );
	nodeVar119 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar117 ) ) ) + vec3<f32>( ( 1.0 * nodeVar117 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar118, clamp( dot( normalView, nodeVar86 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar118, clamp( dot( normalView, nodeVar115 ), 0.0, 1.0 ) ) ) );
	nodeVar120 = ( nodeVar102 * nodeVar119 );
	nodeVar121 = ( nodeVar120 * multiScatteringCompensation );
	nodeVar122 = ( directSpecular + nodeVar121 );
	directSpecular = nodeVar122;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar123 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform35 );
	nodeVar124 = floor( nodeVar123 );
	nodeVar125 = nodeVar124;
	nodeVar126 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar127 = getFace( ( object.nodeUniform36 * vec4<f32>( vec3<f32>( nodeVar126.x, ( - nodeVar126.y ), nodeVar126.z ), 1.0 ) ).xyz );
	nodeVar128 = max( ( 4.0 - nodeVar125 ), 0.0 );
	nodeVar125 = max( nodeVar125, 4.0 );
	nodeVar129 = exp2( nodeVar125 );
	nodeVar130 = ( ( getUV( ( object.nodeUniform36 * vec4<f32>( vec3<f32>( nodeVar126.x, ( - nodeVar126.y ), nodeVar126.z ), 1.0 ) ).xyz, nodeVar127 ) * vec2<f32>( ( nodeVar129 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar127 > 2.0 ) ) {

		nodeVar130.y = ( nodeVar130.y + nodeVar129 );
		nodeVar127 = ( nodeVar127 - 3.0 );
		

	}

	nodeVar130.x = ( nodeVar130.x + ( nodeVar127 * nodeVar129 ) );
	nodeVar130.x = ( nodeVar130.x + ( nodeVar128 * ( 3.0 * 16.0 ) ) );
	nodeVar130.y = ( nodeVar130.y + ( 4.0 * ( exp2( object.nodeUniform35 ) - nodeVar129 ) ) );
	nodeVar130.x = ( nodeVar130.x * object.nodeUniform38 );
	nodeVar130.y = ( nodeVar130.y * object.nodeUniform39 );
	nodeVar131 = textureSampleGrad( nodeUniform40, nodeUniform40_sampler, nodeVar130, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar132 = nodeVar131.xyz;
	nodeVar133 = fract( nodeVar123 );

	if ( ( nodeVar133 != 0.0 ) ) {

		nodeVar134 = ( nodeVar124 + 1.0 );
		nodeVar135 = getFace( ( object.nodeUniform36 * vec4<f32>( vec3<f32>( nodeVar126.x, ( - nodeVar126.y ), nodeVar126.z ), 1.0 ) ).xyz );
		nodeVar136 = max( ( 4.0 - nodeVar134 ), 0.0 );
		nodeVar134 = max( nodeVar134, 4.0 );
		nodeVar137 = exp2( nodeVar134 );
		nodeVar138 = ( ( getUV( ( object.nodeUniform36 * vec4<f32>( vec3<f32>( nodeVar126.x, ( - nodeVar126.y ), nodeVar126.z ), 1.0 ) ).xyz, nodeVar135 ) * vec2<f32>( ( nodeVar137 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar135 > 2.0 ) ) {

			nodeVar138.y = ( nodeVar138.y + nodeVar137 );
			nodeVar135 = ( nodeVar135 - 3.0 );
			

		}

		nodeVar138.x = ( nodeVar138.x + ( nodeVar135 * nodeVar137 ) );
		nodeVar138.x = ( nodeVar138.x + ( nodeVar136 * ( 3.0 * 16.0 ) ) );
		nodeVar138.y = ( nodeVar138.y + ( 4.0 * ( exp2( object.nodeUniform35 ) - nodeVar137 ) ) );
		nodeVar138.x = ( nodeVar138.x * object.nodeUniform38 );
		nodeVar138.y = ( nodeVar138.y * object.nodeUniform39 );
		nodeVar139 = textureSampleGrad( nodeUniform40, nodeUniform40_sampler, nodeVar138, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar140 = nodeVar139.xyz;
		nodeVar132 = mix( nodeVar132, nodeVar140, nodeVar133 );
		

	}

	nodeVar141 = ( radiance + ( nodeVar132 * vec3<f32>( object.nodeUniform41 ) ) );
	radiance = nodeVar141;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar142 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform35 );
	nodeVar143 = floor( nodeVar142 );
	nodeVar144 = nodeVar143;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar145 = getFace( ( object.nodeUniform36 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar146 = max( ( 4.0 - nodeVar144 ), 0.0 );
	nodeVar144 = max( nodeVar144, 4.0 );
	nodeVar147 = exp2( nodeVar144 );
	nodeVar148 = ( ( getUV( ( object.nodeUniform36 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar145 ) * vec2<f32>( ( nodeVar147 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar145 > 2.0 ) ) {

		nodeVar148.y = ( nodeVar148.y + nodeVar147 );
		nodeVar145 = ( nodeVar145 - 3.0 );
		

	}

	nodeVar148.x = ( nodeVar148.x + ( nodeVar145 * nodeVar147 ) );
	nodeVar148.x = ( nodeVar148.x + ( nodeVar146 * ( 3.0 * 16.0 ) ) );
	nodeVar148.y = ( nodeVar148.y + ( 4.0 * ( exp2( object.nodeUniform35 ) - nodeVar147 ) ) );
	nodeVar148.x = ( nodeVar148.x * object.nodeUniform38 );
	nodeVar148.y = ( nodeVar148.y * object.nodeUniform39 );
	nodeVar149 = textureSampleGrad( nodeUniform40, nodeUniform40_sampler, nodeVar148, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar150 = nodeVar149.xyz;
	nodeVar151 = fract( nodeVar142 );

	if ( ( nodeVar151 != 0.0 ) ) {

		nodeVar152 = ( nodeVar143 + 1.0 );
		nodeVar153 = getFace( ( object.nodeUniform36 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar154 = max( ( 4.0 - nodeVar152 ), 0.0 );
		nodeVar152 = max( nodeVar152, 4.0 );
		nodeVar155 = exp2( nodeVar152 );
		nodeVar156 = ( ( getUV( ( object.nodeUniform36 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar153 ) * vec2<f32>( ( nodeVar155 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar153 > 2.0 ) ) {

			nodeVar156.y = ( nodeVar156.y + nodeVar155 );
			nodeVar153 = ( nodeVar153 - 3.0 );
			

		}

		nodeVar156.x = ( nodeVar156.x + ( nodeVar153 * nodeVar155 ) );
		nodeVar156.x = ( nodeVar156.x + ( nodeVar154 * ( 3.0 * 16.0 ) ) );
		nodeVar156.y = ( nodeVar156.y + ( 4.0 * ( exp2( object.nodeUniform35 ) - nodeVar155 ) ) );
		nodeVar156.x = ( nodeVar156.x * object.nodeUniform38 );
		nodeVar156.y = ( nodeVar156.y * object.nodeUniform39 );
		nodeVar157 = textureSampleGrad( nodeUniform40, nodeUniform40_sampler, nodeVar156, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar158 = nodeVar157.xyz;
		nodeVar150 = mix( nodeVar150, nodeVar158, nodeVar151 );
		

	}

	nodeVar159 = ( iblIrradiance + ( ( nodeVar150 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform41 ) ) );
	iblIrradiance = nodeVar159;
	nodeVar160 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar161 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar162 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar163 = ( SpecularF90 * dfg.y );
	nodeVar164 = ( nodeVar162 + vec3<f32>( nodeVar163 ) );
	nodeVar165 = ( nodeVar160 + nodeVar164 );
	nodeVar160 = nodeVar165;
	nodeVar166 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar167 = nodeVar166;
	nodeVar168 = ( nodeVar167 * vec3<f32>( 0.047619 ) );
	nodeVar169 = ( SpecularColor + nodeVar168 );
	nodeVar170 = ( nodeVar164 * nodeVar169 );
	nodeVar171 = ( dfg.x + dfg.y );
	nodeVar172 = ( 1.0 - nodeVar171 );
	nodeVar173 = nodeVar172;
	nodeVar174 = ( vec3<f32>( nodeVar173 ) * nodeVar169 );
	nodeVar175 = ( vec3<f32>( 1.0 ) - nodeVar174 );
	nodeVar176 = nodeVar175;
	nodeVar177 = ( nodeVar170 / nodeVar176 );
	nodeVar178 = ( nodeVar177 * vec3<f32>( nodeVar173 ) );
	nodeVar179 = ( nodeVar161 + nodeVar178 );
	nodeVar161 = nodeVar179;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar180 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar181 = ( irradiance * nodeVar180 );
	nodeVar182 = ( nodeVar160 + nodeVar161 );
	nodeVar183 = ( vec3<f32>( 1.0 ) - nodeVar182 );
	nodeVar184 = nodeVar183;
	nodeVar185 = ( nodeVar181 * nodeVar184 );
	nodeVar186 = nodeVar185;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar187 = ( indirectDiffuse + nodeVar186 );
	indirectDiffuse = nodeVar187;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar188 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar189 = ( SpecularF90 * dfg.y );
	nodeVar190 = ( nodeVar188 + vec3<f32>( nodeVar189 ) );
	nodeVar191 = ( singleScatteringDielectric + nodeVar190 );
	singleScatteringDielectric = nodeVar191;
	nodeVar192 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar193 = nodeVar192;
	nodeVar194 = ( nodeVar193 * vec3<f32>( 0.047619 ) );
	nodeVar195 = ( SpecularColor + nodeVar194 );
	nodeVar196 = ( nodeVar190 * nodeVar195 );
	nodeVar197 = ( dfg.x + dfg.y );
	nodeVar198 = ( 1.0 - nodeVar197 );
	nodeVar199 = nodeVar198;
	nodeVar200 = ( vec3<f32>( nodeVar199 ) * nodeVar195 );
	nodeVar201 = ( vec3<f32>( 1.0 ) - nodeVar200 );
	nodeVar202 = nodeVar201;
	nodeVar203 = ( nodeVar196 / nodeVar202 );
	nodeVar204 = ( nodeVar203 * vec3<f32>( nodeVar199 ) );
	nodeVar205 = ( multiScatteringDielectric + nodeVar204 );
	multiScatteringDielectric = nodeVar205;
	nodeVar206 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar207 = ( SpecularF90 * dfg.y );
	nodeVar208 = ( nodeVar206 + vec3<f32>( nodeVar207 ) );
	nodeVar209 = ( singleScatteringMetallic + nodeVar208 );
	singleScatteringMetallic = nodeVar209;
	nodeVar210 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar211 = nodeVar210;
	nodeVar212 = ( nodeVar211 * vec3<f32>( 0.047619 ) );
	nodeVar213 = ( DiffuseColor.xyz + nodeVar212 );
	nodeVar214 = ( nodeVar208 * nodeVar213 );
	nodeVar215 = ( dfg.x + dfg.y );
	nodeVar216 = ( 1.0 - nodeVar215 );
	nodeVar217 = nodeVar216;
	nodeVar218 = ( vec3<f32>( nodeVar217 ) * nodeVar213 );
	nodeVar219 = ( vec3<f32>( 1.0 ) - nodeVar218 );
	nodeVar220 = nodeVar219;
	nodeVar221 = ( nodeVar214 / nodeVar220 );
	nodeVar222 = ( nodeVar221 * vec3<f32>( nodeVar217 ) );
	nodeVar223 = ( multiScatteringMetallic + nodeVar222 );
	multiScatteringMetallic = nodeVar223;
	nodeVar224 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar225 = ( radiance * nodeVar224 );
	nodeVar226 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar227 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar228 = ( nodeVar226 * nodeVar227 );
	nodeVar229 = ( nodeVar225 + nodeVar228 );
	nodeVar230 = nodeVar229;
	nodeVar231 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar232 = ( vec3<f32>( 1.0 ) - nodeVar231 );
	nodeVar233 = nodeVar232;
	nodeVar234 = ( DiffuseContribution * nodeVar233 );
	nodeVar235 = ( nodeVar234 * nodeVar227 );
	nodeVar236 = nodeVar235;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar237 = ( indirectSpecular + nodeVar230 );
	indirectSpecular = nodeVar237;
	nodeVar238 = ( indirectDiffuse + nodeVar236 );
	indirectDiffuse = nodeVar238;
	ambientOcclusion = 1.0;
	nodeVar239 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar239;
	nodeVar240 = dot( normalView, positionViewDirection );
	nodeVar241 = ( clamp( nodeVar240, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar242 = ( Roughness * -16.0 );
	nodeVar243 = ( 1.0 - nodeVar242 );
	nodeVar244 = nodeVar243;
	nodeVar245 = ( - nodeVar244 );
	nodeVar246 = exp2( nodeVar245 );
	nodeVar247 = pow( nodeVar241, nodeVar246 );
	nodeVar248 = ( 1.0 - nodeVar247 );
	nodeVar249 = nodeVar248;
	nodeVar250 = ( ambientOcclusion - nodeVar249 );
	nodeVar251 = ( indirectSpecular * vec3<f32>( clamp( nodeVar250, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar251;
	nodeVar252 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar252;
	nodeVar253 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar253;
	nodeVar254 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar254;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar255 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m0 = vec4<f32>( nodeVar255, 1.0 );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform43 );
	nodeVar256 = ( ( object.nodeUniform42 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar257 = ( ( render.nodeUniform44 * ( object.nodeUniform45 * object.nodeUniform46 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar258 = ( ( nodeVar256.xy / vec2<f32>( nodeVar256.w ) ) - ( nodeVar257.xy / vec2<f32>( nodeVar257.w ) ) );
	output.m1 = vec4<f32>( vec3<f32>( nodeVar258, 0.0 ), 1.0 );

	// result

	return output;

}
