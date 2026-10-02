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
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform42_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform42 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform37 : f32,
	nodeUniform38 : mat4x4<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform43 : f32,
	nodeUniform44 : mat4x4<f32>,
	nodeUniform45 : mat4x4<f32>,
	nodeUniform47 : mat4x4<f32>,
	nodeUniform48 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform46 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform22 : vec3<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform30 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform33 : vec3<f32>,
	nodeUniform34 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : vec4<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec4<f32>;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : vec2<f32>;
var<private> nodeVar132 : vec4<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : vec2<f32>;
var<private> nodeVar140 : vec4<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : vec2<f32>;
var<private> nodeVar150 : vec4<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : vec2<f32>;
var<private> nodeVar158 : vec4<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : f32;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : f32;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : f32;
var<private> nodeVar217 : f32;
var<private> nodeVar218 : f32;
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
var<private> nodeVar237 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar238 : vec3<f32>;
var<private> nodeVar239 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar240 : vec3<f32>;
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
var<private> nodeVar251 : f32;
var<private> nodeVar252 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar253 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar254 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar255 : vec3<f32>;
var<private> nodeVar256 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar257 : vec4<f32>;
var<private> nodeVar258 : vec4<f32>;
var<private> nodeVar259 : vec2<f32>;

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
	@location( 4 ) positionPrevious : vec3<f32>,
	@location( 5 ) nodeVarying7 : vec2<f32> ) -> OutputType {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform5, 0.0525 ) + max( max( nodeVar1.x, nodeVar1.y ), nodeVar1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = dot( normalView, positionViewDirection );
	nodeVar3 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( Roughness, clamp( nodeVar2, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	nodeVar4 = ( dfg.x + dfg.y );
	nodeVar5 = ( 1.0 / nodeVar4 );
	nodeVar6 = nodeVar5;
	nodeVar7 = ( nodeVar6 - 1.0 );
	nodeVar8 = ( SpecularColorBlended * vec3<f32>( nodeVar7 ) );
	nodeVar9 = ( nodeVar8 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar9;
	nodeVar10 = ( render.nodeUniform12 - v_positionView );
	nodeVar11 = normalize( nodeVar10 );
	nodeVar12 = dot( normalView, nodeVar11 );
	nodeVar13 = ( render.nodeUniform17 - render.nodeUniform18 );
	nodeVar14 = vec4<f32>( nodeVar13, 0.0 );
	nodeVar15 = ( render.cameraViewMatrix * nodeVar14 );
	nodeVar16 = normalize( nodeVar15.xyz );
	nodeVar17 = nodeVar16;
	nodeVar18 = dot( nodeVar11, nodeVar17 );
	nodeVar19 = smoothstep( render.nodeUniform14, render.nodeUniform15, nodeVar18 );
	nodeVar20 = ( render.nodeUniform13 * vec3<f32>( nodeVar19 ) );

	if ( ( render.nodeUniform19 > 0.0 ) ) {

		nodeVar22 = length( nodeVar10 );
		nodeVar23 = ( nodeVar22 / render.nodeUniform19 );
		nodeVar24 = clamp( ( 1.0 - ( ( ( nodeVar23 * nodeVar23 ) * nodeVar23 ) * nodeVar23 ) ), 0.0, 1.0 );
		nodeVar21 = ( ( 1.0 / max( pow( nodeVar22, render.nodeUniform20 ), 0.01 ) ) * ( nodeVar24 * nodeVar24 ) );

	} else {

		nodeVar21 = ( 1.0 / max( pow( length( nodeVar10 ), render.nodeUniform20 ), 0.01 ) );

	}

	nodeVar25 = ( nodeVar20 * vec3<f32>( nodeVar21 ) );
	nodeVar26 = ( vec3<f32>( clamp( nodeVar12, 0.0, 1.0 ) ) * nodeVar25 );
	nodeVar27 = nodeVar26;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar28 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar29 = ( nodeVar27 * nodeVar28 );
	nodeVar30 = ( nodeVar11 + positionViewDirection );
	nodeVar31 = normalize( nodeVar30 );
	nodeVar32 = dot( positionViewDirection, nodeVar31 );
	nodeVar33 = clamp( nodeVar32, 0.0, 1.0 );
	nodeVar34 = exp2( ( ( ( nodeVar33 * -5.55473 ) - 6.98316 ) * nodeVar33 ) );
	nodeVar35 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar34 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar34 ) ) );
	nodeVar36 = ( vec3<f32>( 1.0 ) - nodeVar35 );
	nodeVar37 = nodeVar36;
	nodeVar38 = ( nodeVar29 * nodeVar37 );
	nodeVar39 = ( directDiffuse + nodeVar38 );
	directDiffuse = nodeVar39;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar40 = normalize( ( nodeVar11 + positionViewDirection ) );
	nodeVar41 = clamp( dot( positionViewDirection, nodeVar40 ), 0.0, 1.0 );
	nodeVar42 = exp2( ( ( ( nodeVar41 * -5.55473 ) - 6.98316 ) * nodeVar41 ) );
	nodeVar43 = ( Roughness * Roughness );
	nodeVar44 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar42 ) ) ) + vec3<f32>( ( 1.0 * nodeVar42 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar43, clamp( dot( normalView, nodeVar11 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar43, clamp( dot( normalView, nodeVar40 ), 0.0, 1.0 ) ) ) );
	nodeVar45 = ( nodeVar27 * nodeVar44 );
	nodeVar46 = ( nodeVar45 * multiScatteringCompensation );
	nodeVar47 = ( directSpecular + nodeVar46 );
	directSpecular = nodeVar47;
	nodeVar48 = ( render.nodeUniform21 - v_positionView );
	nodeVar49 = normalize( nodeVar48 );
	nodeVar50 = dot( normalView, nodeVar49 );
	nodeVar51 = ( render.nodeUniform25 - render.nodeUniform26 );
	nodeVar52 = vec4<f32>( nodeVar51, 0.0 );
	nodeVar53 = ( render.cameraViewMatrix * nodeVar52 );
	nodeVar54 = normalize( nodeVar53.xyz );
	nodeVar55 = nodeVar54;
	nodeVar56 = dot( nodeVar49, nodeVar55 );
	nodeVar57 = smoothstep( render.nodeUniform23, render.nodeUniform24, nodeVar56 );
	nodeVar58 = ( render.nodeUniform22 * vec3<f32>( nodeVar57 ) );

	if ( ( render.nodeUniform27 > 0.0 ) ) {

		nodeVar60 = length( nodeVar48 );
		nodeVar61 = ( nodeVar60 / render.nodeUniform27 );
		nodeVar62 = clamp( ( 1.0 - ( ( ( nodeVar61 * nodeVar61 ) * nodeVar61 ) * nodeVar61 ) ), 0.0, 1.0 );
		nodeVar59 = ( ( 1.0 / max( pow( nodeVar60, render.nodeUniform28 ), 0.01 ) ) * ( nodeVar62 * nodeVar62 ) );

	} else {

		nodeVar59 = ( 1.0 / max( pow( length( nodeVar48 ), render.nodeUniform28 ), 0.01 ) );

	}

	nodeVar63 = ( nodeVar58 * vec3<f32>( nodeVar59 ) );
	nodeVar64 = ( vec3<f32>( clamp( nodeVar50, 0.0, 1.0 ) ) * nodeVar63 );
	nodeVar65 = nodeVar64;
	nodeVar66 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar67 = ( nodeVar65 * nodeVar66 );
	nodeVar68 = ( nodeVar49 + positionViewDirection );
	nodeVar69 = normalize( nodeVar68 );
	nodeVar70 = dot( positionViewDirection, nodeVar69 );
	nodeVar71 = clamp( nodeVar70, 0.0, 1.0 );
	nodeVar72 = exp2( ( ( ( nodeVar71 * -5.55473 ) - 6.98316 ) * nodeVar71 ) );
	nodeVar73 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar72 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar72 ) ) );
	nodeVar74 = ( vec3<f32>( 1.0 ) - nodeVar73 );
	nodeVar75 = nodeVar74;
	nodeVar76 = ( nodeVar67 * nodeVar75 );
	nodeVar77 = ( directDiffuse + nodeVar76 );
	directDiffuse = nodeVar77;
	nodeVar78 = normalize( ( nodeVar49 + positionViewDirection ) );
	nodeVar79 = clamp( dot( positionViewDirection, nodeVar78 ), 0.0, 1.0 );
	nodeVar80 = exp2( ( ( ( nodeVar79 * -5.55473 ) - 6.98316 ) * nodeVar79 ) );
	nodeVar81 = ( Roughness * Roughness );
	nodeVar82 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar80 ) ) ) + vec3<f32>( ( 1.0 * nodeVar80 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar81, clamp( dot( normalView, nodeVar49 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar81, clamp( dot( normalView, nodeVar78 ), 0.0, 1.0 ) ) ) );
	nodeVar83 = ( nodeVar65 * nodeVar82 );
	nodeVar84 = ( nodeVar83 * multiScatteringCompensation );
	nodeVar85 = ( directSpecular + nodeVar84 );
	directSpecular = nodeVar85;
	nodeVar86 = ( render.nodeUniform29 - v_positionView );
	nodeVar87 = normalize( nodeVar86 );
	nodeVar88 = dot( normalView, nodeVar87 );
	nodeVar89 = ( render.nodeUniform33 - render.nodeUniform34 );
	nodeVar90 = vec4<f32>( nodeVar89, 0.0 );
	nodeVar91 = ( render.cameraViewMatrix * nodeVar90 );
	nodeVar92 = normalize( nodeVar91.xyz );
	nodeVar93 = nodeVar92;
	nodeVar94 = dot( nodeVar87, nodeVar93 );
	nodeVar95 = smoothstep( render.nodeUniform31, render.nodeUniform32, nodeVar94 );
	nodeVar96 = ( render.nodeUniform30 * vec3<f32>( nodeVar95 ) );

	if ( ( render.nodeUniform35 > 0.0 ) ) {

		nodeVar98 = length( nodeVar86 );
		nodeVar99 = ( nodeVar98 / render.nodeUniform35 );
		nodeVar100 = clamp( ( 1.0 - ( ( ( nodeVar99 * nodeVar99 ) * nodeVar99 ) * nodeVar99 ) ), 0.0, 1.0 );
		nodeVar97 = ( ( 1.0 / max( pow( nodeVar98, render.nodeUniform36 ), 0.01 ) ) * ( nodeVar100 * nodeVar100 ) );

	} else {

		nodeVar97 = ( 1.0 / max( pow( length( nodeVar86 ), render.nodeUniform36 ), 0.01 ) );

	}

	nodeVar101 = ( nodeVar96 * vec3<f32>( nodeVar97 ) );
	nodeVar102 = ( vec3<f32>( clamp( nodeVar88, 0.0, 1.0 ) ) * nodeVar101 );
	nodeVar103 = nodeVar102;
	nodeVar104 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar105 = ( nodeVar103 * nodeVar104 );
	nodeVar106 = ( nodeVar87 + positionViewDirection );
	nodeVar107 = normalize( nodeVar106 );
	nodeVar108 = dot( positionViewDirection, nodeVar107 );
	nodeVar109 = clamp( nodeVar108, 0.0, 1.0 );
	nodeVar110 = exp2( ( ( ( nodeVar109 * -5.55473 ) - 6.98316 ) * nodeVar109 ) );
	nodeVar111 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar110 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar110 ) ) );
	nodeVar112 = ( vec3<f32>( 1.0 ) - nodeVar111 );
	nodeVar113 = nodeVar112;
	nodeVar114 = ( nodeVar105 * nodeVar113 );
	nodeVar115 = ( directDiffuse + nodeVar114 );
	directDiffuse = nodeVar115;
	nodeVar116 = normalize( ( nodeVar87 + positionViewDirection ) );
	nodeVar117 = clamp( dot( positionViewDirection, nodeVar116 ), 0.0, 1.0 );
	nodeVar118 = exp2( ( ( ( nodeVar117 * -5.55473 ) - 6.98316 ) * nodeVar117 ) );
	nodeVar119 = ( Roughness * Roughness );
	nodeVar120 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar118 ) ) ) + vec3<f32>( ( 1.0 * nodeVar118 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar119, clamp( dot( normalView, nodeVar87 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar119, clamp( dot( normalView, nodeVar116 ), 0.0, 1.0 ) ) ) );
	nodeVar121 = ( nodeVar103 * nodeVar120 );
	nodeVar122 = ( nodeVar121 * multiScatteringCompensation );
	nodeVar123 = ( directSpecular + nodeVar122 );
	directSpecular = nodeVar123;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar124 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform37 );
	nodeVar125 = floor( nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar128 = getFace( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( nodeVar127.x, ( - nodeVar127.y ), nodeVar127.z ), 1.0 ) ).xyz );
	nodeVar129 = max( ( 4.0 - nodeVar126 ), 0.0 );
	nodeVar126 = max( nodeVar126, 4.0 );
	nodeVar130 = exp2( nodeVar126 );
	nodeVar131 = ( ( getUV( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( nodeVar127.x, ( - nodeVar127.y ), nodeVar127.z ), 1.0 ) ).xyz, nodeVar128 ) * vec2<f32>( ( nodeVar130 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar128 > 2.0 ) ) {

		nodeVar131.y = ( nodeVar131.y + nodeVar130 );
		nodeVar128 = ( nodeVar128 - 3.0 );
		

	}

	nodeVar131.x = ( nodeVar131.x + ( nodeVar128 * nodeVar130 ) );
	nodeVar131.x = ( nodeVar131.x + ( nodeVar129 * ( 3.0 * 16.0 ) ) );
	nodeVar131.y = ( nodeVar131.y + ( 4.0 * ( exp2( object.nodeUniform37 ) - nodeVar130 ) ) );
	nodeVar131.x = ( nodeVar131.x * object.nodeUniform40 );
	nodeVar131.y = ( nodeVar131.y * object.nodeUniform41 );
	nodeVar132 = textureSampleGrad( nodeUniform42, nodeUniform42_sampler, nodeVar131, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar133 = nodeVar132.xyz;
	nodeVar134 = fract( nodeVar124 );

	if ( ( nodeVar134 != 0.0 ) ) {

		nodeVar135 = ( nodeVar125 + 1.0 );
		nodeVar136 = getFace( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( nodeVar127.x, ( - nodeVar127.y ), nodeVar127.z ), 1.0 ) ).xyz );
		nodeVar137 = max( ( 4.0 - nodeVar135 ), 0.0 );
		nodeVar135 = max( nodeVar135, 4.0 );
		nodeVar138 = exp2( nodeVar135 );
		nodeVar139 = ( ( getUV( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( nodeVar127.x, ( - nodeVar127.y ), nodeVar127.z ), 1.0 ) ).xyz, nodeVar136 ) * vec2<f32>( ( nodeVar138 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar136 > 2.0 ) ) {

			nodeVar139.y = ( nodeVar139.y + nodeVar138 );
			nodeVar136 = ( nodeVar136 - 3.0 );
			

		}

		nodeVar139.x = ( nodeVar139.x + ( nodeVar136 * nodeVar138 ) );
		nodeVar139.x = ( nodeVar139.x + ( nodeVar137 * ( 3.0 * 16.0 ) ) );
		nodeVar139.y = ( nodeVar139.y + ( 4.0 * ( exp2( object.nodeUniform37 ) - nodeVar138 ) ) );
		nodeVar139.x = ( nodeVar139.x * object.nodeUniform40 );
		nodeVar139.y = ( nodeVar139.y * object.nodeUniform41 );
		nodeVar140 = textureSampleGrad( nodeUniform42, nodeUniform42_sampler, nodeVar139, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar141 = nodeVar140.xyz;
		nodeVar133 = mix( nodeVar133, nodeVar141, nodeVar134 );
		

	}

	nodeVar142 = ( radiance + ( nodeVar133 * vec3<f32>( object.nodeUniform43 ) ) );
	radiance = nodeVar142;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar143 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform37 );
	nodeVar144 = floor( nodeVar143 );
	nodeVar145 = nodeVar144;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar146 = getFace( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar147 = max( ( 4.0 - nodeVar145 ), 0.0 );
	nodeVar145 = max( nodeVar145, 4.0 );
	nodeVar148 = exp2( nodeVar145 );
	nodeVar149 = ( ( getUV( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar146 ) * vec2<f32>( ( nodeVar148 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar146 > 2.0 ) ) {

		nodeVar149.y = ( nodeVar149.y + nodeVar148 );
		nodeVar146 = ( nodeVar146 - 3.0 );
		

	}

	nodeVar149.x = ( nodeVar149.x + ( nodeVar146 * nodeVar148 ) );
	nodeVar149.x = ( nodeVar149.x + ( nodeVar147 * ( 3.0 * 16.0 ) ) );
	nodeVar149.y = ( nodeVar149.y + ( 4.0 * ( exp2( object.nodeUniform37 ) - nodeVar148 ) ) );
	nodeVar149.x = ( nodeVar149.x * object.nodeUniform40 );
	nodeVar149.y = ( nodeVar149.y * object.nodeUniform41 );
	nodeVar150 = textureSampleGrad( nodeUniform42, nodeUniform42_sampler, nodeVar149, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar151 = nodeVar150.xyz;
	nodeVar152 = fract( nodeVar143 );

	if ( ( nodeVar152 != 0.0 ) ) {

		nodeVar153 = ( nodeVar144 + 1.0 );
		nodeVar154 = getFace( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar155 = max( ( 4.0 - nodeVar153 ), 0.0 );
		nodeVar153 = max( nodeVar153, 4.0 );
		nodeVar156 = exp2( nodeVar153 );
		nodeVar157 = ( ( getUV( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar154 ) * vec2<f32>( ( nodeVar156 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar154 > 2.0 ) ) {

			nodeVar157.y = ( nodeVar157.y + nodeVar156 );
			nodeVar154 = ( nodeVar154 - 3.0 );
			

		}

		nodeVar157.x = ( nodeVar157.x + ( nodeVar154 * nodeVar156 ) );
		nodeVar157.x = ( nodeVar157.x + ( nodeVar155 * ( 3.0 * 16.0 ) ) );
		nodeVar157.y = ( nodeVar157.y + ( 4.0 * ( exp2( object.nodeUniform37 ) - nodeVar156 ) ) );
		nodeVar157.x = ( nodeVar157.x * object.nodeUniform40 );
		nodeVar157.y = ( nodeVar157.y * object.nodeUniform41 );
		nodeVar158 = textureSampleGrad( nodeUniform42, nodeUniform42_sampler, nodeVar157, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar159 = nodeVar158.xyz;
		nodeVar151 = mix( nodeVar151, nodeVar159, nodeVar152 );
		

	}

	nodeVar160 = ( iblIrradiance + ( ( nodeVar151 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform43 ) ) );
	iblIrradiance = nodeVar160;
	nodeVar161 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar162 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar163 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar164 = ( SpecularF90 * dfg.y );
	nodeVar165 = ( nodeVar163 + vec3<f32>( nodeVar164 ) );
	nodeVar166 = ( nodeVar161 + nodeVar165 );
	nodeVar161 = nodeVar166;
	nodeVar167 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar168 = nodeVar167;
	nodeVar169 = ( nodeVar168 * vec3<f32>( 0.047619 ) );
	nodeVar170 = ( SpecularColor + nodeVar169 );
	nodeVar171 = ( nodeVar165 * nodeVar170 );
	nodeVar172 = ( dfg.x + dfg.y );
	nodeVar173 = ( 1.0 - nodeVar172 );
	nodeVar174 = nodeVar173;
	nodeVar175 = ( vec3<f32>( nodeVar174 ) * nodeVar170 );
	nodeVar176 = ( vec3<f32>( 1.0 ) - nodeVar175 );
	nodeVar177 = nodeVar176;
	nodeVar178 = ( nodeVar171 / nodeVar177 );
	nodeVar179 = ( nodeVar178 * vec3<f32>( nodeVar174 ) );
	nodeVar180 = ( nodeVar162 + nodeVar179 );
	nodeVar162 = nodeVar180;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar181 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar182 = ( irradiance * nodeVar181 );
	nodeVar183 = ( nodeVar161 + nodeVar162 );
	nodeVar184 = ( vec3<f32>( 1.0 ) - nodeVar183 );
	nodeVar185 = nodeVar184;
	nodeVar186 = ( nodeVar182 * nodeVar185 );
	nodeVar187 = nodeVar186;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar188 = ( indirectDiffuse + nodeVar187 );
	indirectDiffuse = nodeVar188;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar189 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar190 = ( SpecularF90 * dfg.y );
	nodeVar191 = ( nodeVar189 + vec3<f32>( nodeVar190 ) );
	nodeVar192 = ( singleScatteringDielectric + nodeVar191 );
	singleScatteringDielectric = nodeVar192;
	nodeVar193 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar194 = nodeVar193;
	nodeVar195 = ( nodeVar194 * vec3<f32>( 0.047619 ) );
	nodeVar196 = ( SpecularColor + nodeVar195 );
	nodeVar197 = ( nodeVar191 * nodeVar196 );
	nodeVar198 = ( dfg.x + dfg.y );
	nodeVar199 = ( 1.0 - nodeVar198 );
	nodeVar200 = nodeVar199;
	nodeVar201 = ( vec3<f32>( nodeVar200 ) * nodeVar196 );
	nodeVar202 = ( vec3<f32>( 1.0 ) - nodeVar201 );
	nodeVar203 = nodeVar202;
	nodeVar204 = ( nodeVar197 / nodeVar203 );
	nodeVar205 = ( nodeVar204 * vec3<f32>( nodeVar200 ) );
	nodeVar206 = ( multiScatteringDielectric + nodeVar205 );
	multiScatteringDielectric = nodeVar206;
	nodeVar207 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar208 = ( SpecularF90 * dfg.y );
	nodeVar209 = ( nodeVar207 + vec3<f32>( nodeVar208 ) );
	nodeVar210 = ( singleScatteringMetallic + nodeVar209 );
	singleScatteringMetallic = nodeVar210;
	nodeVar211 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar212 = nodeVar211;
	nodeVar213 = ( nodeVar212 * vec3<f32>( 0.047619 ) );
	nodeVar214 = ( DiffuseColor.xyz + nodeVar213 );
	nodeVar215 = ( nodeVar209 * nodeVar214 );
	nodeVar216 = ( dfg.x + dfg.y );
	nodeVar217 = ( 1.0 - nodeVar216 );
	nodeVar218 = nodeVar217;
	nodeVar219 = ( vec3<f32>( nodeVar218 ) * nodeVar214 );
	nodeVar220 = ( vec3<f32>( 1.0 ) - nodeVar219 );
	nodeVar221 = nodeVar220;
	nodeVar222 = ( nodeVar215 / nodeVar221 );
	nodeVar223 = ( nodeVar222 * vec3<f32>( nodeVar218 ) );
	nodeVar224 = ( multiScatteringMetallic + nodeVar223 );
	multiScatteringMetallic = nodeVar224;
	nodeVar225 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar226 = ( radiance * nodeVar225 );
	nodeVar227 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar228 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar229 = ( nodeVar227 * nodeVar228 );
	nodeVar230 = ( nodeVar226 + nodeVar229 );
	nodeVar231 = nodeVar230;
	nodeVar232 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar233 = ( vec3<f32>( 1.0 ) - nodeVar232 );
	nodeVar234 = nodeVar233;
	nodeVar235 = ( DiffuseContribution * nodeVar234 );
	nodeVar236 = ( nodeVar235 * nodeVar228 );
	nodeVar237 = nodeVar236;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar238 = ( indirectSpecular + nodeVar231 );
	indirectSpecular = nodeVar238;
	nodeVar239 = ( indirectDiffuse + nodeVar237 );
	indirectDiffuse = nodeVar239;
	ambientOcclusion = 1.0;
	nodeVar240 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar240;
	nodeVar241 = dot( normalView, positionViewDirection );
	nodeVar242 = ( clamp( nodeVar241, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar243 = ( Roughness * -16.0 );
	nodeVar244 = ( 1.0 - nodeVar243 );
	nodeVar245 = nodeVar244;
	nodeVar246 = ( - nodeVar245 );
	nodeVar247 = exp2( nodeVar246 );
	nodeVar248 = pow( nodeVar242, nodeVar247 );
	nodeVar249 = ( 1.0 - nodeVar248 );
	nodeVar250 = nodeVar249;
	nodeVar251 = ( ambientOcclusion - nodeVar250 );
	nodeVar252 = ( indirectSpecular * vec3<f32>( clamp( nodeVar251, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar252;
	nodeVar253 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar253;
	nodeVar254 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar254;
	nodeVar255 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar255;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar256 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m0 = vec4<f32>( nodeVar256, 1.0 );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform45 );
	nodeVar257 = ( ( object.nodeUniform44 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar258 = ( ( render.nodeUniform46 * ( object.nodeUniform47 * object.nodeUniform48 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar259 = ( ( nodeVar257.xy / vec2<f32>( nodeVar257.w ) ) - ( nodeVar258.xy / vec2<f32>( nodeVar258.w ) ) );
	output.m1 = vec4<f32>( vec3<f32>( nodeVar259, 0.0 ), 1.0 );

	// result

	return output;

}
