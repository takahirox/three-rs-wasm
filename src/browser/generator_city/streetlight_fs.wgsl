// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform10 : texture_3d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform21 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : bool;
var<private> Metalness : f32;
var<private> nodeVar2 : f32;
var<private> Roughness : f32;
var<private> nodeVar3 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec4<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec4<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec4<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec4<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec4<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec4<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec4<f32>;
var<private> nodeVar97 : array< vec3<f32>, 9 >;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : vec2<f32>;
var<private> nodeVar110 : vec4<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec2<f32>;
var<private> nodeVar118 : vec4<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : vec2<f32>;
var<private> nodeVar128 : vec4<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : vec2<f32>;
var<private> nodeVar136 : vec4<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar186 : f32;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar218 : vec3<f32>;
var<private> nodeVar219 : f32;
var<private> nodeVar220 : f32;
var<private> nodeVar221 : f32;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : f32;
var<private> nodeVar224 : f32;
var<private> nodeVar225 : f32;
var<private> nodeVar226 : f32;
var<private> nodeVar227 : f32;
var<private> nodeVar228 : f32;
var<private> nodeVar229 : f32;
var<private> nodeVar230 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar231 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar233 : vec3<f32>;
var<private> nodeVar234 : vec4<f32>;

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
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = ( nodeVarying3 == 1.0 );

	if ( nodeVar1 ) {

		nodeVar0 = vec3<f32>( 1.0, 0.8713671191959567, 0.6038273388475408 );

	} else {

		nodeVar0 = vec3<f32>( 0.05126945836711539, 0.05612849004241121, 0.04666508633021928 );

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;

	if ( nodeVar1 ) {

		nodeVar2 = 0.0;

	} else {

		nodeVar2 = 0.7;

	}

	Metalness = nodeVar2;

	if ( nodeVar1 ) {

		nodeVar3 = 0.3;

	} else {

		nodeVar3 = 0.5;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar3, 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeVar2 ) ) );

	if ( nodeVar1 ) {

		nodeVar5 = ( vec3<f32>( 1.0, 0.76052450467022, 0.3813260114221238 ) * vec3<f32>( 60.0 ) );

	} else {

		nodeVar5 = vec3<f32>( 0.0, 0.0, 0.0 );

	}

	EmissiveColor = nodeVar5;
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = dot( normalView, positionViewDirection );
	nodeVar7 = textureSample( nodeUniform4, nodeUniform4_sampler, vec2<f32>( Roughness, clamp( nodeVar6, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar7;
	nodeVar8 = ( dfg.x + dfg.y );
	nodeVar9 = ( 1.0 / nodeVar8 );
	nodeVar10 = nodeVar9;
	nodeVar11 = ( nodeVar10 - 1.0 );
	nodeVar12 = ( SpecularColorBlended * vec3<f32>( nodeVar11 ) );
	nodeVar13 = ( nodeVar12 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar13;
	nodeVar14 = ( render.nodeUniform7 - render.nodeUniform8 );
	nodeVar15 = vec4<f32>( nodeVar14, 0.0 );
	nodeVar16 = ( render.cameraViewMatrix * nodeVar15 );
	nodeVar17 = normalize( nodeVar16.xyz );
	nodeVar18 = nodeVar17;
	nodeVar19 = dot( normalView, nodeVar18 );
	nodeVar20 = ( vec3<f32>( clamp( nodeVar19, 0.0, 1.0 ) ) * render.nodeUniform9 );
	nodeVar21 = nodeVar20;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar22 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar23 = ( nodeVar21 * nodeVar22 );
	nodeVar24 = ( nodeVar18 + positionViewDirection );
	nodeVar25 = normalize( nodeVar24 );
	nodeVar26 = dot( positionViewDirection, nodeVar25 );
	nodeVar27 = clamp( nodeVar26, 0.0, 1.0 );
	nodeVar28 = exp2( ( ( ( nodeVar27 * -5.55473 ) - 6.98316 ) * nodeVar27 ) );
	nodeVar29 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar28 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar28 ) ) );
	nodeVar30 = ( vec3<f32>( 1.0 ) - nodeVar29 );
	nodeVar31 = nodeVar30;
	nodeVar32 = ( nodeVar23 * nodeVar31 );
	nodeVar33 = ( directDiffuse + nodeVar32 );
	directDiffuse = nodeVar33;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar34 = normalize( ( nodeVar18 + positionViewDirection ) );
	nodeVar35 = clamp( dot( positionViewDirection, nodeVar34 ), 0.0, 1.0 );
	nodeVar36 = exp2( ( ( ( nodeVar35 * -5.55473 ) - 6.98316 ) * nodeVar35 ) );
	nodeVar37 = ( Roughness * Roughness );
	nodeVar38 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar36 ) ) ) + vec3<f32>( ( 1.0 * nodeVar36 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar37, clamp( dot( normalView, nodeVar18 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar37, clamp( dot( normalView, nodeVar34 ), 0.0, 1.0 ) ) ) );
	nodeVar39 = ( nodeVar21 * nodeVar38 );
	nodeVar40 = ( nodeVar39 * multiScatteringCompensation );
	nodeVar41 = ( directSpecular + nodeVar40 );
	directSpecular = nodeVar41;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar42 = ( object.nodeUniform11 - object.nodeUniform12 );
	nodeVar43 = ( object.nodeUniform13 - vec3<f32>( 1.0 ) );
	nodeVar44 = ( nodeVar42 / nodeVar43 );
	nodeVar45 = ( normalWorld * nodeVar44 );
	nodeVar46 = ( nodeVar45 * vec3<f32>( 0.5 ) );
	nodeVar47 = ( v_positionWorld + nodeVar46 );
	nodeVar48 = ( nodeVar47 - object.nodeUniform12 );
	nodeVar49 = ( nodeVar48 / nodeVar42 );
	nodeVar50 = ( clamp( nodeVar49, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar43 );
	nodeVar51 = ( nodeVar50 / object.nodeUniform13 );
	nodeVar52 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform13 );
	nodeVar53 = ( nodeVar51 + nodeVar52 );
	nodeVar54 = ( nodeVar53.z * object.nodeUniform13.z );
	nodeVar55 = ( nodeVar54 + 1.0 );
	nodeVar56 = ( object.nodeUniform13.z + 2.0 );
	nodeVar57 = ( nodeVar56 * 0.0 );
	nodeVar58 = ( nodeVar55 + nodeVar57 );
	nodeVar59 = ( nodeVar56 * 7.0 );
	nodeVar60 = ( nodeVar58 / nodeVar59 );
	nodeVar61 = vec3<f32>( nodeVar53.xy, nodeVar60 );
	nodeVar62 = textureSample( nodeUniform10, nodeUniform10_sampler, nodeVar61 );
	nodeVar63 = ( nodeVar56 * 1.0 );
	nodeVar64 = ( nodeVar55 + nodeVar63 );
	nodeVar65 = ( nodeVar64 / nodeVar59 );
	nodeVar66 = vec3<f32>( nodeVar53.xy, nodeVar65 );
	nodeVar67 = textureSample( nodeUniform10, nodeUniform10_sampler, nodeVar66 );
	nodeVar68 = vec3<f32>( nodeVar62.w, nodeVar67.xy );
	nodeVar69 = ( nodeVar56 * 2.0 );
	nodeVar70 = ( nodeVar55 + nodeVar69 );
	nodeVar71 = ( nodeVar70 / nodeVar59 );
	nodeVar72 = vec3<f32>( nodeVar53.xy, nodeVar71 );
	nodeVar73 = textureSample( nodeUniform10, nodeUniform10_sampler, nodeVar72 );
	nodeVar74 = vec3<f32>( nodeVar67.zw, nodeVar73.x );
	nodeVar75 = ( nodeVar56 * 3.0 );
	nodeVar76 = ( nodeVar55 + nodeVar75 );
	nodeVar77 = ( nodeVar76 / nodeVar59 );
	nodeVar78 = vec3<f32>( nodeVar53.xy, nodeVar77 );
	nodeVar79 = textureSample( nodeUniform10, nodeUniform10_sampler, nodeVar78 );
	nodeVar80 = ( nodeVar56 * 4.0 );
	nodeVar81 = ( nodeVar55 + nodeVar80 );
	nodeVar82 = ( nodeVar81 / nodeVar59 );
	nodeVar83 = vec3<f32>( nodeVar53.xy, nodeVar82 );
	nodeVar84 = textureSample( nodeUniform10, nodeUniform10_sampler, nodeVar83 );
	nodeVar85 = vec3<f32>( nodeVar79.w, nodeVar84.xy );
	nodeVar86 = ( nodeVar56 * 5.0 );
	nodeVar87 = ( nodeVar55 + nodeVar86 );
	nodeVar88 = ( nodeVar87 / nodeVar59 );
	nodeVar89 = vec3<f32>( nodeVar53.xy, nodeVar88 );
	nodeVar90 = textureSample( nodeUniform10, nodeUniform10_sampler, nodeVar89 );
	nodeVar91 = vec3<f32>( nodeVar84.zw, nodeVar90.x );
	nodeVar92 = ( nodeVar56 * 6.0 );
	nodeVar93 = ( nodeVar55 + nodeVar92 );
	nodeVar94 = ( nodeVar93 / nodeVar59 );
	nodeVar95 = vec3<f32>( nodeVar53.xy, nodeVar94 );
	nodeVar96 = textureSample( nodeUniform10, nodeUniform10_sampler, nodeVar95 );
	nodeVar97 = array< vec3<f32>, 9 >( nodeVar62.xyz, nodeVar68, nodeVar74, nodeVar73.yzw, nodeVar79.xyz, nodeVar85, nodeVar91, nodeVar90.yzw, nodeVar96.xyz );
	nodeVar98 = ( ( ( ( ( ( ( ( ( nodeVar97[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar97[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar97[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar97[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar97[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar97[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar97[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar97[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar97[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar99 = max( nodeVar98, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar100 = ( nodeVar99 * vec3<f32>( object.nodeUniform14 ) );
	nodeVar101 = ( irradiance + nodeVar100 );
	irradiance = nodeVar101;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar102 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform15 );
	nodeVar103 = floor( nodeVar102 );
	nodeVar104 = nodeVar103;
	nodeVar105 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar106 = getFace( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( nodeVar105.x, ( - nodeVar105.y ), nodeVar105.z ), 1.0 ) ).xyz );
	nodeVar107 = max( ( 4.0 - nodeVar104 ), 0.0 );
	nodeVar104 = max( nodeVar104, 4.0 );
	nodeVar108 = exp2( nodeVar104 );
	nodeVar109 = ( ( getUV( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( nodeVar105.x, ( - nodeVar105.y ), nodeVar105.z ), 1.0 ) ).xyz, nodeVar106 ) * vec2<f32>( ( nodeVar108 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar106 > 2.0 ) ) {

		nodeVar109.y = ( nodeVar109.y + nodeVar108 );
		nodeVar106 = ( nodeVar106 - 3.0 );
		

	}

	nodeVar109.x = ( nodeVar109.x + ( nodeVar106 * nodeVar108 ) );
	nodeVar109.x = ( nodeVar109.x + ( nodeVar107 * ( 3.0 * 16.0 ) ) );
	nodeVar109.y = ( nodeVar109.y + ( 4.0 * ( exp2( object.nodeUniform15 ) - nodeVar108 ) ) );
	nodeVar109.x = ( nodeVar109.x * object.nodeUniform18 );
	nodeVar109.y = ( nodeVar109.y * object.nodeUniform19 );
	nodeVar110 = textureSampleGrad( nodeUniform20, nodeUniform20_sampler, nodeVar109, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar111 = nodeVar110.xyz;
	nodeVar112 = fract( nodeVar102 );

	if ( ( nodeVar112 != 0.0 ) ) {

		nodeVar113 = ( nodeVar103 + 1.0 );
		nodeVar114 = getFace( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( nodeVar105.x, ( - nodeVar105.y ), nodeVar105.z ), 1.0 ) ).xyz );
		nodeVar115 = max( ( 4.0 - nodeVar113 ), 0.0 );
		nodeVar113 = max( nodeVar113, 4.0 );
		nodeVar116 = exp2( nodeVar113 );
		nodeVar117 = ( ( getUV( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( nodeVar105.x, ( - nodeVar105.y ), nodeVar105.z ), 1.0 ) ).xyz, nodeVar114 ) * vec2<f32>( ( nodeVar116 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar114 > 2.0 ) ) {

			nodeVar117.y = ( nodeVar117.y + nodeVar116 );
			nodeVar114 = ( nodeVar114 - 3.0 );
			

		}

		nodeVar117.x = ( nodeVar117.x + ( nodeVar114 * nodeVar116 ) );
		nodeVar117.x = ( nodeVar117.x + ( nodeVar115 * ( 3.0 * 16.0 ) ) );
		nodeVar117.y = ( nodeVar117.y + ( 4.0 * ( exp2( object.nodeUniform15 ) - nodeVar116 ) ) );
		nodeVar117.x = ( nodeVar117.x * object.nodeUniform18 );
		nodeVar117.y = ( nodeVar117.y * object.nodeUniform19 );
		nodeVar118 = textureSampleGrad( nodeUniform20, nodeUniform20_sampler, nodeVar117, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar119 = nodeVar118.xyz;
		nodeVar111 = mix( nodeVar111, nodeVar119, nodeVar112 );
		

	}

	nodeVar120 = ( radiance + ( nodeVar111 * vec3<f32>( object.nodeUniform21 ) ) );
	radiance = nodeVar120;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar121 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform15 );
	nodeVar122 = floor( nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = getFace( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar125 = max( ( 4.0 - nodeVar123 ), 0.0 );
	nodeVar123 = max( nodeVar123, 4.0 );
	nodeVar126 = exp2( nodeVar123 );
	nodeVar127 = ( ( getUV( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar124 ) * vec2<f32>( ( nodeVar126 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar124 > 2.0 ) ) {

		nodeVar127.y = ( nodeVar127.y + nodeVar126 );
		nodeVar124 = ( nodeVar124 - 3.0 );
		

	}

	nodeVar127.x = ( nodeVar127.x + ( nodeVar124 * nodeVar126 ) );
	nodeVar127.x = ( nodeVar127.x + ( nodeVar125 * ( 3.0 * 16.0 ) ) );
	nodeVar127.y = ( nodeVar127.y + ( 4.0 * ( exp2( object.nodeUniform15 ) - nodeVar126 ) ) );
	nodeVar127.x = ( nodeVar127.x * object.nodeUniform18 );
	nodeVar127.y = ( nodeVar127.y * object.nodeUniform19 );
	nodeVar128 = textureSampleGrad( nodeUniform20, nodeUniform20_sampler, nodeVar127, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar129 = nodeVar128.xyz;
	nodeVar130 = fract( nodeVar121 );

	if ( ( nodeVar130 != 0.0 ) ) {

		nodeVar131 = ( nodeVar122 + 1.0 );
		nodeVar132 = getFace( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar133 = max( ( 4.0 - nodeVar131 ), 0.0 );
		nodeVar131 = max( nodeVar131, 4.0 );
		nodeVar134 = exp2( nodeVar131 );
		nodeVar135 = ( ( getUV( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar132 ) * vec2<f32>( ( nodeVar134 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar132 > 2.0 ) ) {

			nodeVar135.y = ( nodeVar135.y + nodeVar134 );
			nodeVar132 = ( nodeVar132 - 3.0 );
			

		}

		nodeVar135.x = ( nodeVar135.x + ( nodeVar132 * nodeVar134 ) );
		nodeVar135.x = ( nodeVar135.x + ( nodeVar133 * ( 3.0 * 16.0 ) ) );
		nodeVar135.y = ( nodeVar135.y + ( 4.0 * ( exp2( object.nodeUniform15 ) - nodeVar134 ) ) );
		nodeVar135.x = ( nodeVar135.x * object.nodeUniform18 );
		nodeVar135.y = ( nodeVar135.y * object.nodeUniform19 );
		nodeVar136 = textureSampleGrad( nodeUniform20, nodeUniform20_sampler, nodeVar135, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar137 = nodeVar136.xyz;
		nodeVar129 = mix( nodeVar129, nodeVar137, nodeVar130 );
		

	}

	nodeVar138 = ( iblIrradiance + ( ( nodeVar129 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform21 ) ) );
	iblIrradiance = nodeVar138;
	nodeVar139 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar141 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar142 = ( SpecularF90 * dfg.y );
	nodeVar143 = ( nodeVar141 + vec3<f32>( nodeVar142 ) );
	nodeVar144 = ( nodeVar139 + nodeVar143 );
	nodeVar139 = nodeVar144;
	nodeVar145 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar146 = nodeVar145;
	nodeVar147 = ( nodeVar146 * vec3<f32>( 0.047619 ) );
	nodeVar148 = ( SpecularColor + nodeVar147 );
	nodeVar149 = ( nodeVar143 * nodeVar148 );
	nodeVar150 = ( dfg.x + dfg.y );
	nodeVar151 = ( 1.0 - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( vec3<f32>( nodeVar152 ) * nodeVar148 );
	nodeVar154 = ( vec3<f32>( 1.0 ) - nodeVar153 );
	nodeVar155 = nodeVar154;
	nodeVar156 = ( nodeVar149 / nodeVar155 );
	nodeVar157 = ( nodeVar156 * vec3<f32>( nodeVar152 ) );
	nodeVar158 = ( nodeVar140 + nodeVar157 );
	nodeVar140 = nodeVar158;
	nodeVar159 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar160 = ( irradiance * nodeVar159 );
	nodeVar161 = ( nodeVar139 + nodeVar140 );
	nodeVar162 = ( vec3<f32>( 1.0 ) - nodeVar161 );
	nodeVar163 = nodeVar162;
	nodeVar164 = ( nodeVar160 * nodeVar163 );
	nodeVar165 = nodeVar164;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar166 = ( indirectDiffuse + nodeVar165 );
	indirectDiffuse = nodeVar166;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar167 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar168 = ( SpecularF90 * dfg.y );
	nodeVar169 = ( nodeVar167 + vec3<f32>( nodeVar168 ) );
	nodeVar170 = ( singleScatteringDielectric + nodeVar169 );
	singleScatteringDielectric = nodeVar170;
	nodeVar171 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar172 = nodeVar171;
	nodeVar173 = ( nodeVar172 * vec3<f32>( 0.047619 ) );
	nodeVar174 = ( SpecularColor + nodeVar173 );
	nodeVar175 = ( nodeVar169 * nodeVar174 );
	nodeVar176 = ( dfg.x + dfg.y );
	nodeVar177 = ( 1.0 - nodeVar176 );
	nodeVar178 = nodeVar177;
	nodeVar179 = ( vec3<f32>( nodeVar178 ) * nodeVar174 );
	nodeVar180 = ( vec3<f32>( 1.0 ) - nodeVar179 );
	nodeVar181 = nodeVar180;
	nodeVar182 = ( nodeVar175 / nodeVar181 );
	nodeVar183 = ( nodeVar182 * vec3<f32>( nodeVar178 ) );
	nodeVar184 = ( multiScatteringDielectric + nodeVar183 );
	multiScatteringDielectric = nodeVar184;
	nodeVar185 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar186 = ( SpecularF90 * dfg.y );
	nodeVar187 = ( nodeVar185 + vec3<f32>( nodeVar186 ) );
	nodeVar188 = ( singleScatteringMetallic + nodeVar187 );
	singleScatteringMetallic = nodeVar188;
	nodeVar189 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar190 = nodeVar189;
	nodeVar191 = ( nodeVar190 * vec3<f32>( 0.047619 ) );
	nodeVar192 = ( DiffuseColor.xyz + nodeVar191 );
	nodeVar193 = ( nodeVar187 * nodeVar192 );
	nodeVar194 = ( dfg.x + dfg.y );
	nodeVar195 = ( 1.0 - nodeVar194 );
	nodeVar196 = nodeVar195;
	nodeVar197 = ( vec3<f32>( nodeVar196 ) * nodeVar192 );
	nodeVar198 = ( vec3<f32>( 1.0 ) - nodeVar197 );
	nodeVar199 = nodeVar198;
	nodeVar200 = ( nodeVar193 / nodeVar199 );
	nodeVar201 = ( nodeVar200 * vec3<f32>( nodeVar196 ) );
	nodeVar202 = ( multiScatteringMetallic + nodeVar201 );
	multiScatteringMetallic = nodeVar202;
	nodeVar203 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar204 = ( radiance * nodeVar203 );
	nodeVar205 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar206 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar207 = ( nodeVar205 * nodeVar206 );
	nodeVar208 = ( nodeVar204 + nodeVar207 );
	nodeVar209 = nodeVar208;
	nodeVar210 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar211 = ( vec3<f32>( 1.0 ) - nodeVar210 );
	nodeVar212 = nodeVar211;
	nodeVar213 = ( DiffuseContribution * nodeVar212 );
	nodeVar214 = ( nodeVar213 * nodeVar206 );
	nodeVar215 = nodeVar214;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar216 = ( indirectSpecular + nodeVar209 );
	indirectSpecular = nodeVar216;
	nodeVar217 = ( indirectDiffuse + nodeVar215 );
	indirectDiffuse = nodeVar217;
	ambientOcclusion = 1.0;
	nodeVar218 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar218;
	nodeVar219 = dot( normalView, positionViewDirection );
	nodeVar220 = ( clamp( nodeVar219, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar221 = ( Roughness * -16.0 );
	nodeVar222 = ( 1.0 - nodeVar221 );
	nodeVar223 = nodeVar222;
	nodeVar224 = ( - nodeVar223 );
	nodeVar225 = exp2( nodeVar224 );
	nodeVar226 = pow( nodeVar220, nodeVar225 );
	nodeVar227 = ( 1.0 - nodeVar226 );
	nodeVar228 = nodeVar227;
	nodeVar229 = ( ambientOcclusion - nodeVar228 );
	nodeVar230 = ( indirectSpecular * vec3<f32>( clamp( nodeVar229, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar230;
	nodeVar231 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar231;
	nodeVar232 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar232;
	nodeVar233 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar233;
	nodeVar234 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar234;

	// result

	output.color = nodeVar234;

	return output;

}
