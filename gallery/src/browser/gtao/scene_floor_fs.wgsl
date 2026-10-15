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
@binding( 5 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform44_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform44 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : f32,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform39 : f32,
	nodeUniform40 : mat4x4<f32>,
	nodeUniform42 : f32,
	nodeUniform43 : f32,
	nodeUniform45 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform24 : vec3<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32,
	nodeUniform37 : f32,
	nodeUniform38 : f32,
	nodeUniform32 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	nodeUniform35 : vec3<f32>,
	nodeUniform36 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform5 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar1 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec4<f32>;
var<private> nodeVar54 : vec4<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec4<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
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
var<private> nodeVar118 : f32;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : vec2<f32>;
var<private> nodeVar133 : vec4<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec2<f32>;
var<private> nodeVar141 : vec4<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : vec2<f32>;
var<private> nodeVar151 : vec4<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : vec2<f32>;
var<private> nodeVar159 : vec4<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : f32;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : f32;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : f32;
var<private> nodeVar219 : f32;
var<private> nodeVar220 : f32;
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
var<private> nodeVar238 : vec3<f32>;
var<private> nodeVar239 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar240 : vec3<f32>;
var<private> nodeVar241 : vec3<f32>;
var<private> nodeVar242 : vec3<f32>;
var<private> nodeVar243 : f32;
var<private> nodeVar244 : f32;
var<private> nodeVar245 : f32;
var<private> nodeVar246 : f32;
var<private> nodeVar247 : f32;
var<private> nodeVar248 : f32;
var<private> nodeVar249 : f32;
var<private> nodeVar250 : f32;
var<private> nodeVar251 : f32;
var<private> nodeVar252 : f32;
var<private> nodeVar253 : f32;
var<private> nodeVar254 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar255 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar256 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar257 : vec3<f32>;
var<private> nodeVar258 : vec4<f32>;

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
	@location( 3 ) nodeVarying6 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( fragCoord.xy / render.nodeUniform5 ) ).x;
	AmbientOcclusion = nodeVar1;
	Metalness = object.nodeUniform6;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar2 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform7, 0.0525 ) + max( max( nodeVar2.x, nodeVar2.y ), nodeVar2.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform6 ) ) );
	EmissiveColor = ( object.nodeUniform10 * vec3<f32>( object.nodeUniform11 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar3 = dot( normalView, positionViewDirection );
	nodeVar4 = textureSample( nodeUniform12, nodeUniform12_sampler, vec2<f32>( Roughness, clamp( nodeVar3, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	nodeVar5 = ( dfg.x + dfg.y );
	nodeVar6 = ( 1.0 / nodeVar5 );
	nodeVar7 = nodeVar6;
	nodeVar8 = ( nodeVar7 - 1.0 );
	nodeVar9 = ( SpecularColorBlended * vec3<f32>( nodeVar8 ) );
	nodeVar10 = ( nodeVar9 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar10;
	nodeVar11 = ( render.nodeUniform14 - v_positionView );
	nodeVar12 = normalize( nodeVar11 );
	nodeVar13 = dot( normalView, nodeVar12 );
	nodeVar14 = ( render.nodeUniform19 - render.nodeUniform20 );
	nodeVar15 = vec4<f32>( nodeVar14, 0.0 );
	nodeVar16 = ( render.cameraViewMatrix * nodeVar15 );
	nodeVar17 = normalize( nodeVar16.xyz );
	nodeVar18 = nodeVar17;
	nodeVar19 = dot( nodeVar12, nodeVar18 );
	nodeVar20 = smoothstep( render.nodeUniform16, render.nodeUniform17, nodeVar19 );
	nodeVar21 = ( render.nodeUniform15 * vec3<f32>( nodeVar20 ) );

	if ( ( render.nodeUniform21 > 0.0 ) ) {

		nodeVar23 = length( nodeVar11 );
		nodeVar24 = ( nodeVar23 / render.nodeUniform21 );
		nodeVar25 = clamp( ( 1.0 - ( ( ( nodeVar24 * nodeVar24 ) * nodeVar24 ) * nodeVar24 ) ), 0.0, 1.0 );
		nodeVar22 = ( ( 1.0 / max( pow( nodeVar23, render.nodeUniform22 ), 0.01 ) ) * ( nodeVar25 * nodeVar25 ) );

	} else {

		nodeVar22 = ( 1.0 / max( pow( length( nodeVar11 ), render.nodeUniform22 ), 0.01 ) );

	}

	nodeVar26 = ( nodeVar21 * vec3<f32>( nodeVar22 ) );
	nodeVar27 = ( vec3<f32>( clamp( nodeVar13, 0.0, 1.0 ) ) * nodeVar26 );
	nodeVar28 = nodeVar27;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar29 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar30 = ( nodeVar28 * nodeVar29 );
	nodeVar31 = ( nodeVar12 + positionViewDirection );
	nodeVar32 = normalize( nodeVar31 );
	nodeVar33 = dot( positionViewDirection, nodeVar32 );
	nodeVar34 = clamp( nodeVar33, 0.0, 1.0 );
	nodeVar35 = exp2( ( ( ( nodeVar34 * -5.55473 ) - 6.98316 ) * nodeVar34 ) );
	nodeVar36 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar35 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar35 ) ) );
	nodeVar37 = ( vec3<f32>( 1.0 ) - nodeVar36 );
	nodeVar38 = nodeVar37;
	nodeVar39 = ( nodeVar30 * nodeVar38 );
	nodeVar40 = ( directDiffuse + nodeVar39 );
	directDiffuse = nodeVar40;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar41 = normalize( ( nodeVar12 + positionViewDirection ) );
	nodeVar42 = clamp( dot( positionViewDirection, nodeVar41 ), 0.0, 1.0 );
	nodeVar43 = exp2( ( ( ( nodeVar42 * -5.55473 ) - 6.98316 ) * nodeVar42 ) );
	nodeVar44 = ( Roughness * Roughness );
	nodeVar45 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar43 ) ) ) + vec3<f32>( ( 1.0 * nodeVar43 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar44, clamp( dot( normalView, nodeVar12 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar44, clamp( dot( normalView, nodeVar41 ), 0.0, 1.0 ) ) ) );
	nodeVar46 = ( nodeVar28 * nodeVar45 );
	nodeVar47 = ( nodeVar46 * multiScatteringCompensation );
	nodeVar48 = ( directSpecular + nodeVar47 );
	directSpecular = nodeVar48;
	nodeVar49 = ( render.nodeUniform23 - v_positionView );
	nodeVar50 = normalize( nodeVar49 );
	nodeVar51 = dot( normalView, nodeVar50 );
	nodeVar52 = ( render.nodeUniform27 - render.nodeUniform28 );
	nodeVar53 = vec4<f32>( nodeVar52, 0.0 );
	nodeVar54 = ( render.cameraViewMatrix * nodeVar53 );
	nodeVar55 = normalize( nodeVar54.xyz );
	nodeVar56 = nodeVar55;
	nodeVar57 = dot( nodeVar50, nodeVar56 );
	nodeVar58 = smoothstep( render.nodeUniform25, render.nodeUniform26, nodeVar57 );
	nodeVar59 = ( render.nodeUniform24 * vec3<f32>( nodeVar58 ) );

	if ( ( render.nodeUniform29 > 0.0 ) ) {

		nodeVar61 = length( nodeVar49 );
		nodeVar62 = ( nodeVar61 / render.nodeUniform29 );
		nodeVar63 = clamp( ( 1.0 - ( ( ( nodeVar62 * nodeVar62 ) * nodeVar62 ) * nodeVar62 ) ), 0.0, 1.0 );
		nodeVar60 = ( ( 1.0 / max( pow( nodeVar61, render.nodeUniform30 ), 0.01 ) ) * ( nodeVar63 * nodeVar63 ) );

	} else {

		nodeVar60 = ( 1.0 / max( pow( length( nodeVar49 ), render.nodeUniform30 ), 0.01 ) );

	}

	nodeVar64 = ( nodeVar59 * vec3<f32>( nodeVar60 ) );
	nodeVar65 = ( vec3<f32>( clamp( nodeVar51, 0.0, 1.0 ) ) * nodeVar64 );
	nodeVar66 = nodeVar65;
	nodeVar67 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar68 = ( nodeVar66 * nodeVar67 );
	nodeVar69 = ( nodeVar50 + positionViewDirection );
	nodeVar70 = normalize( nodeVar69 );
	nodeVar71 = dot( positionViewDirection, nodeVar70 );
	nodeVar72 = clamp( nodeVar71, 0.0, 1.0 );
	nodeVar73 = exp2( ( ( ( nodeVar72 * -5.55473 ) - 6.98316 ) * nodeVar72 ) );
	nodeVar74 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar73 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar73 ) ) );
	nodeVar75 = ( vec3<f32>( 1.0 ) - nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = ( nodeVar68 * nodeVar76 );
	nodeVar78 = ( directDiffuse + nodeVar77 );
	directDiffuse = nodeVar78;
	nodeVar79 = normalize( ( nodeVar50 + positionViewDirection ) );
	nodeVar80 = clamp( dot( positionViewDirection, nodeVar79 ), 0.0, 1.0 );
	nodeVar81 = exp2( ( ( ( nodeVar80 * -5.55473 ) - 6.98316 ) * nodeVar80 ) );
	nodeVar82 = ( Roughness * Roughness );
	nodeVar83 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar81 ) ) ) + vec3<f32>( ( 1.0 * nodeVar81 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar82, clamp( dot( normalView, nodeVar50 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar82, clamp( dot( normalView, nodeVar79 ), 0.0, 1.0 ) ) ) );
	nodeVar84 = ( nodeVar66 * nodeVar83 );
	nodeVar85 = ( nodeVar84 * multiScatteringCompensation );
	nodeVar86 = ( directSpecular + nodeVar85 );
	directSpecular = nodeVar86;
	nodeVar87 = ( render.nodeUniform31 - v_positionView );
	nodeVar88 = normalize( nodeVar87 );
	nodeVar89 = dot( normalView, nodeVar88 );
	nodeVar90 = ( render.nodeUniform35 - render.nodeUniform36 );
	nodeVar91 = vec4<f32>( nodeVar90, 0.0 );
	nodeVar92 = ( render.cameraViewMatrix * nodeVar91 );
	nodeVar93 = normalize( nodeVar92.xyz );
	nodeVar94 = nodeVar93;
	nodeVar95 = dot( nodeVar88, nodeVar94 );
	nodeVar96 = smoothstep( render.nodeUniform33, render.nodeUniform34, nodeVar95 );
	nodeVar97 = ( render.nodeUniform32 * vec3<f32>( nodeVar96 ) );

	if ( ( render.nodeUniform37 > 0.0 ) ) {

		nodeVar99 = length( nodeVar87 );
		nodeVar100 = ( nodeVar99 / render.nodeUniform37 );
		nodeVar101 = clamp( ( 1.0 - ( ( ( nodeVar100 * nodeVar100 ) * nodeVar100 ) * nodeVar100 ) ), 0.0, 1.0 );
		nodeVar98 = ( ( 1.0 / max( pow( nodeVar99, render.nodeUniform38 ), 0.01 ) ) * ( nodeVar101 * nodeVar101 ) );

	} else {

		nodeVar98 = ( 1.0 / max( pow( length( nodeVar87 ), render.nodeUniform38 ), 0.01 ) );

	}

	nodeVar102 = ( nodeVar97 * vec3<f32>( nodeVar98 ) );
	nodeVar103 = ( vec3<f32>( clamp( nodeVar89, 0.0, 1.0 ) ) * nodeVar102 );
	nodeVar104 = nodeVar103;
	nodeVar105 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar106 = ( nodeVar104 * nodeVar105 );
	nodeVar107 = ( nodeVar88 + positionViewDirection );
	nodeVar108 = normalize( nodeVar107 );
	nodeVar109 = dot( positionViewDirection, nodeVar108 );
	nodeVar110 = clamp( nodeVar109, 0.0, 1.0 );
	nodeVar111 = exp2( ( ( ( nodeVar110 * -5.55473 ) - 6.98316 ) * nodeVar110 ) );
	nodeVar112 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar111 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar111 ) ) );
	nodeVar113 = ( vec3<f32>( 1.0 ) - nodeVar112 );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar106 * nodeVar114 );
	nodeVar116 = ( directDiffuse + nodeVar115 );
	directDiffuse = nodeVar116;
	nodeVar117 = normalize( ( nodeVar88 + positionViewDirection ) );
	nodeVar118 = clamp( dot( positionViewDirection, nodeVar117 ), 0.0, 1.0 );
	nodeVar119 = exp2( ( ( ( nodeVar118 * -5.55473 ) - 6.98316 ) * nodeVar118 ) );
	nodeVar120 = ( Roughness * Roughness );
	nodeVar121 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar119 ) ) ) + vec3<f32>( ( 1.0 * nodeVar119 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar120, clamp( dot( normalView, nodeVar88 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar120, clamp( dot( normalView, nodeVar117 ), 0.0, 1.0 ) ) ) );
	nodeVar122 = ( nodeVar104 * nodeVar121 );
	nodeVar123 = ( nodeVar122 * multiScatteringCompensation );
	nodeVar124 = ( directSpecular + nodeVar123 );
	directSpecular = nodeVar124;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar125 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform39 );
	nodeVar126 = floor( nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar129 = getFace( ( object.nodeUniform40 * vec4<f32>( vec3<f32>( nodeVar128.x, ( - nodeVar128.y ), nodeVar128.z ), 1.0 ) ).xyz );
	nodeVar130 = max( ( 4.0 - nodeVar127 ), 0.0 );
	nodeVar127 = max( nodeVar127, 4.0 );
	nodeVar131 = exp2( nodeVar127 );
	nodeVar132 = ( ( getUV( ( object.nodeUniform40 * vec4<f32>( vec3<f32>( nodeVar128.x, ( - nodeVar128.y ), nodeVar128.z ), 1.0 ) ).xyz, nodeVar129 ) * vec2<f32>( ( nodeVar131 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar129 > 2.0 ) ) {

		nodeVar132.y = ( nodeVar132.y + nodeVar131 );
		nodeVar129 = ( nodeVar129 - 3.0 );
		

	}

	nodeVar132.x = ( nodeVar132.x + ( nodeVar129 * nodeVar131 ) );
	nodeVar132.x = ( nodeVar132.x + ( nodeVar130 * ( 3.0 * 16.0 ) ) );
	nodeVar132.y = ( nodeVar132.y + ( 4.0 * ( exp2( object.nodeUniform39 ) - nodeVar131 ) ) );
	nodeVar132.x = ( nodeVar132.x * object.nodeUniform42 );
	nodeVar132.y = ( nodeVar132.y * object.nodeUniform43 );
	nodeVar133 = textureSampleGrad( nodeUniform44, nodeUniform44_sampler, nodeVar132, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar134 = nodeVar133.xyz;
	nodeVar135 = fract( nodeVar125 );

	if ( ( nodeVar135 != 0.0 ) ) {

		nodeVar136 = ( nodeVar126 + 1.0 );
		nodeVar137 = getFace( ( object.nodeUniform40 * vec4<f32>( vec3<f32>( nodeVar128.x, ( - nodeVar128.y ), nodeVar128.z ), 1.0 ) ).xyz );
		nodeVar138 = max( ( 4.0 - nodeVar136 ), 0.0 );
		nodeVar136 = max( nodeVar136, 4.0 );
		nodeVar139 = exp2( nodeVar136 );
		nodeVar140 = ( ( getUV( ( object.nodeUniform40 * vec4<f32>( vec3<f32>( nodeVar128.x, ( - nodeVar128.y ), nodeVar128.z ), 1.0 ) ).xyz, nodeVar137 ) * vec2<f32>( ( nodeVar139 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar137 > 2.0 ) ) {

			nodeVar140.y = ( nodeVar140.y + nodeVar139 );
			nodeVar137 = ( nodeVar137 - 3.0 );
			

		}

		nodeVar140.x = ( nodeVar140.x + ( nodeVar137 * nodeVar139 ) );
		nodeVar140.x = ( nodeVar140.x + ( nodeVar138 * ( 3.0 * 16.0 ) ) );
		nodeVar140.y = ( nodeVar140.y + ( 4.0 * ( exp2( object.nodeUniform39 ) - nodeVar139 ) ) );
		nodeVar140.x = ( nodeVar140.x * object.nodeUniform42 );
		nodeVar140.y = ( nodeVar140.y * object.nodeUniform43 );
		nodeVar141 = textureSampleGrad( nodeUniform44, nodeUniform44_sampler, nodeVar140, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar142 = nodeVar141.xyz;
		nodeVar134 = mix( nodeVar134, nodeVar142, nodeVar135 );
		

	}

	nodeVar143 = ( radiance + ( nodeVar134 * vec3<f32>( object.nodeUniform45 ) ) );
	radiance = nodeVar143;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar144 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform39 );
	nodeVar145 = floor( nodeVar144 );
	nodeVar146 = nodeVar145;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar147 = getFace( ( object.nodeUniform40 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar148 = max( ( 4.0 - nodeVar146 ), 0.0 );
	nodeVar146 = max( nodeVar146, 4.0 );
	nodeVar149 = exp2( nodeVar146 );
	nodeVar150 = ( ( getUV( ( object.nodeUniform40 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar147 ) * vec2<f32>( ( nodeVar149 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar147 > 2.0 ) ) {

		nodeVar150.y = ( nodeVar150.y + nodeVar149 );
		nodeVar147 = ( nodeVar147 - 3.0 );
		

	}

	nodeVar150.x = ( nodeVar150.x + ( nodeVar147 * nodeVar149 ) );
	nodeVar150.x = ( nodeVar150.x + ( nodeVar148 * ( 3.0 * 16.0 ) ) );
	nodeVar150.y = ( nodeVar150.y + ( 4.0 * ( exp2( object.nodeUniform39 ) - nodeVar149 ) ) );
	nodeVar150.x = ( nodeVar150.x * object.nodeUniform42 );
	nodeVar150.y = ( nodeVar150.y * object.nodeUniform43 );
	nodeVar151 = textureSampleGrad( nodeUniform44, nodeUniform44_sampler, nodeVar150, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar152 = nodeVar151.xyz;
	nodeVar153 = fract( nodeVar144 );

	if ( ( nodeVar153 != 0.0 ) ) {

		nodeVar154 = ( nodeVar145 + 1.0 );
		nodeVar155 = getFace( ( object.nodeUniform40 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar156 = max( ( 4.0 - nodeVar154 ), 0.0 );
		nodeVar154 = max( nodeVar154, 4.0 );
		nodeVar157 = exp2( nodeVar154 );
		nodeVar158 = ( ( getUV( ( object.nodeUniform40 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar155 ) * vec2<f32>( ( nodeVar157 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar155 > 2.0 ) ) {

			nodeVar158.y = ( nodeVar158.y + nodeVar157 );
			nodeVar155 = ( nodeVar155 - 3.0 );
			

		}

		nodeVar158.x = ( nodeVar158.x + ( nodeVar155 * nodeVar157 ) );
		nodeVar158.x = ( nodeVar158.x + ( nodeVar156 * ( 3.0 * 16.0 ) ) );
		nodeVar158.y = ( nodeVar158.y + ( 4.0 * ( exp2( object.nodeUniform39 ) - nodeVar157 ) ) );
		nodeVar158.x = ( nodeVar158.x * object.nodeUniform42 );
		nodeVar158.y = ( nodeVar158.y * object.nodeUniform43 );
		nodeVar159 = textureSampleGrad( nodeUniform44, nodeUniform44_sampler, nodeVar158, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar160 = nodeVar159.xyz;
		nodeVar152 = mix( nodeVar152, nodeVar160, nodeVar153 );
		

	}

	nodeVar161 = ( iblIrradiance + ( ( nodeVar152 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform45 ) ) );
	iblIrradiance = nodeVar161;
	ambientOcclusion = 1.0;
	nodeVar162 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar162;
	nodeVar163 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar164 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar165 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar166 = ( SpecularF90 * dfg.y );
	nodeVar167 = ( nodeVar165 + vec3<f32>( nodeVar166 ) );
	nodeVar168 = ( nodeVar163 + nodeVar167 );
	nodeVar163 = nodeVar168;
	nodeVar169 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar170 = nodeVar169;
	nodeVar171 = ( nodeVar170 * vec3<f32>( 0.047619 ) );
	nodeVar172 = ( SpecularColor + nodeVar171 );
	nodeVar173 = ( nodeVar167 * nodeVar172 );
	nodeVar174 = ( dfg.x + dfg.y );
	nodeVar175 = ( 1.0 - nodeVar174 );
	nodeVar176 = nodeVar175;
	nodeVar177 = ( vec3<f32>( nodeVar176 ) * nodeVar172 );
	nodeVar178 = ( vec3<f32>( 1.0 ) - nodeVar177 );
	nodeVar179 = nodeVar178;
	nodeVar180 = ( nodeVar173 / nodeVar179 );
	nodeVar181 = ( nodeVar180 * vec3<f32>( nodeVar176 ) );
	nodeVar182 = ( nodeVar164 + nodeVar181 );
	nodeVar164 = nodeVar182;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar183 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar184 = ( irradiance * nodeVar183 );
	nodeVar185 = ( nodeVar163 + nodeVar164 );
	nodeVar186 = ( vec3<f32>( 1.0 ) - nodeVar185 );
	nodeVar187 = nodeVar186;
	nodeVar188 = ( nodeVar184 * nodeVar187 );
	nodeVar189 = nodeVar188;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar190 = ( indirectDiffuse + nodeVar189 );
	indirectDiffuse = nodeVar190;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar191 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar192 = ( SpecularF90 * dfg.y );
	nodeVar193 = ( nodeVar191 + vec3<f32>( nodeVar192 ) );
	nodeVar194 = ( singleScatteringDielectric + nodeVar193 );
	singleScatteringDielectric = nodeVar194;
	nodeVar195 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar196 = nodeVar195;
	nodeVar197 = ( nodeVar196 * vec3<f32>( 0.047619 ) );
	nodeVar198 = ( SpecularColor + nodeVar197 );
	nodeVar199 = ( nodeVar193 * nodeVar198 );
	nodeVar200 = ( dfg.x + dfg.y );
	nodeVar201 = ( 1.0 - nodeVar200 );
	nodeVar202 = nodeVar201;
	nodeVar203 = ( vec3<f32>( nodeVar202 ) * nodeVar198 );
	nodeVar204 = ( vec3<f32>( 1.0 ) - nodeVar203 );
	nodeVar205 = nodeVar204;
	nodeVar206 = ( nodeVar199 / nodeVar205 );
	nodeVar207 = ( nodeVar206 * vec3<f32>( nodeVar202 ) );
	nodeVar208 = ( multiScatteringDielectric + nodeVar207 );
	multiScatteringDielectric = nodeVar208;
	nodeVar209 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar210 = ( SpecularF90 * dfg.y );
	nodeVar211 = ( nodeVar209 + vec3<f32>( nodeVar210 ) );
	nodeVar212 = ( singleScatteringMetallic + nodeVar211 );
	singleScatteringMetallic = nodeVar212;
	nodeVar213 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar214 = nodeVar213;
	nodeVar215 = ( nodeVar214 * vec3<f32>( 0.047619 ) );
	nodeVar216 = ( DiffuseColor.xyz + nodeVar215 );
	nodeVar217 = ( nodeVar211 * nodeVar216 );
	nodeVar218 = ( dfg.x + dfg.y );
	nodeVar219 = ( 1.0 - nodeVar218 );
	nodeVar220 = nodeVar219;
	nodeVar221 = ( vec3<f32>( nodeVar220 ) * nodeVar216 );
	nodeVar222 = ( vec3<f32>( 1.0 ) - nodeVar221 );
	nodeVar223 = nodeVar222;
	nodeVar224 = ( nodeVar217 / nodeVar223 );
	nodeVar225 = ( nodeVar224 * vec3<f32>( nodeVar220 ) );
	nodeVar226 = ( multiScatteringMetallic + nodeVar225 );
	multiScatteringMetallic = nodeVar226;
	nodeVar227 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar228 = ( radiance * nodeVar227 );
	nodeVar229 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar230 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar231 = ( nodeVar229 * nodeVar230 );
	nodeVar232 = ( nodeVar228 + nodeVar231 );
	nodeVar233 = nodeVar232;
	nodeVar234 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar235 = ( vec3<f32>( 1.0 ) - nodeVar234 );
	nodeVar236 = nodeVar235;
	nodeVar237 = ( DiffuseContribution * nodeVar236 );
	nodeVar238 = ( nodeVar237 * nodeVar230 );
	nodeVar239 = nodeVar238;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar240 = ( indirectSpecular + nodeVar233 );
	indirectSpecular = nodeVar240;
	nodeVar241 = ( indirectDiffuse + nodeVar239 );
	indirectDiffuse = nodeVar241;
	nodeVar242 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar242;
	nodeVar243 = dot( normalView, positionViewDirection );
	nodeVar244 = ( clamp( nodeVar243, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar245 = ( Roughness * -16.0 );
	nodeVar246 = ( 1.0 - nodeVar245 );
	nodeVar247 = nodeVar246;
	nodeVar248 = ( - nodeVar247 );
	nodeVar249 = exp2( nodeVar248 );
	nodeVar250 = pow( nodeVar244, nodeVar249 );
	nodeVar251 = ( 1.0 - nodeVar250 );
	nodeVar252 = nodeVar251;
	nodeVar253 = ( ambientOcclusion - nodeVar252 );
	nodeVar254 = ( indirectSpecular * vec3<f32>( clamp( nodeVar253, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar254;
	nodeVar255 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar255;
	nodeVar256 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar256;
	nodeVar257 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar257;
	nodeVar258 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar258;

	// result

	output.color = nodeVar258;

	return output;

}
