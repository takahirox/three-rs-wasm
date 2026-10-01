// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform14 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform19 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform35_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform35 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : f32,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : mat4x4<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32,
	nodeUniform36 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
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
var<private> nodeVar6 : vec4<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
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
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : f32;
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
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : vec4<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec2<f32>;
var<private> nodeVar83 : vec4<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec2<f32>;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec2<f32>;
var<private> nodeVar101 : vec4<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : vec2<f32>;
var<private> nodeVar109 : vec4<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : f32;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : f32;
var<private> nodeVar203 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec4<f32>;

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
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) nodeVarying4 : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureSample( nodeUniform2, nodeUniform2_sampler, ( object.nodeUniform3 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform1, 1.0 ) * nodeVar1 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	nodeVar2 = textureSample( nodeUniform6, nodeUniform6_sampler, ( object.nodeUniform7 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform5 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform6, nodeUniform6_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform8 * nodeVar3.y ), 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	IOR = object.nodeUniform12;
	nodeVar5 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	nodeVar6 = textureSample( nodeUniform14, nodeUniform14_sampler, ( object.nodeUniform15 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar5 * nodeVar5 ) ) * ( object.nodeUniform13 * nodeVar6.xyz ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform16 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform16, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform5 * nodeVar2.z ) ) ) );
	EmissiveColor = ( object.nodeUniform17 * vec3<f32>( object.nodeUniform18 ) );
	normalView = nodeVarying4;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar7 = dot( normalView, positionViewDirection );
	nodeVar8 = textureSample( nodeUniform19, nodeUniform19_sampler, vec2<f32>( Roughness, clamp( nodeVar7, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar8;
	nodeVar9 = ( dfg.x + dfg.y );
	nodeVar10 = ( 1.0 / nodeVar9 );
	nodeVar11 = nodeVar10;
	nodeVar12 = ( nodeVar11 - 1.0 );
	nodeVar13 = ( SpecularColorBlended * vec3<f32>( nodeVar12 ) );
	nodeVar14 = ( nodeVar13 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar14;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar15 = dot( normalWorld, normalize( render.nodeUniform24 ) );
	nodeVar16 = ( nodeVar15 * 0.5 );
	nodeVar17 = ( nodeVar16 + 0.5 );
	nodeVar18 = mix( render.nodeUniform21, render.nodeUniform22, nodeVar17 );
	nodeVar19 = ( irradiance + nodeVar18 );
	irradiance = nodeVar19;
	nodeVar20 = vec4<f32>( render.nodeUniform25, 0.0 );
	nodeVar21 = ( render.cameraViewMatrix * nodeVar20 );
	nodeVar22 = normalize( nodeVar21.xyz );
	nodeVar23 = nodeVar22;
	nodeVar24 = dot( normalView, nodeVar23 );
	nodeVar25 = ( vec3<f32>( clamp( nodeVar24, 0.0, 1.0 ) ) * render.nodeUniform26 );
	nodeVar26 = nodeVar25;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar27 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar28 = ( nodeVar26 * nodeVar27 );
	nodeVar29 = ( nodeVar23 + positionViewDirection );
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
	nodeVar39 = normalize( ( nodeVar23 + positionViewDirection ) );
	nodeVar40 = clamp( dot( positionViewDirection, nodeVar39 ), 0.0, 1.0 );
	nodeVar41 = exp2( ( ( ( nodeVar40 * -5.55473 ) - 6.98316 ) * nodeVar40 ) );
	nodeVar42 = ( Roughness * Roughness );
	nodeVar43 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar41 ) ) ) + vec3<f32>( ( 1.0 * nodeVar41 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar42, clamp( dot( normalView, nodeVar23 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar42, clamp( dot( normalView, nodeVar39 ), 0.0, 1.0 ) ) ) );
	nodeVar44 = ( nodeVar26 * nodeVar43 );
	nodeVar45 = ( nodeVar44 * multiScatteringCompensation );
	nodeVar46 = ( directSpecular + nodeVar45 );
	directSpecular = nodeVar46;
	nodeVar47 = ( render.nodeUniform27 - render.nodeUniform28 );
	nodeVar48 = vec4<f32>( nodeVar47, 0.0 );
	nodeVar49 = ( render.cameraViewMatrix * nodeVar48 );
	nodeVar50 = normalize( nodeVar49.xyz );
	nodeVar51 = nodeVar50;
	nodeVar52 = dot( normalView, nodeVar51 );
	nodeVar53 = ( vec3<f32>( clamp( nodeVar52, 0.0, 1.0 ) ) * render.nodeUniform29 );
	nodeVar54 = nodeVar53;
	nodeVar55 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar56 = ( nodeVar54 * nodeVar55 );
	nodeVar57 = ( nodeVar51 + positionViewDirection );
	nodeVar58 = normalize( nodeVar57 );
	nodeVar59 = dot( positionViewDirection, nodeVar58 );
	nodeVar60 = clamp( nodeVar59, 0.0, 1.0 );
	nodeVar61 = exp2( ( ( ( nodeVar60 * -5.55473 ) - 6.98316 ) * nodeVar60 ) );
	nodeVar62 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar61 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar61 ) ) );
	nodeVar63 = ( vec3<f32>( 1.0 ) - nodeVar62 );
	nodeVar64 = nodeVar63;
	nodeVar65 = ( nodeVar56 * nodeVar64 );
	nodeVar66 = ( directDiffuse + nodeVar65 );
	directDiffuse = nodeVar66;
	nodeVar67 = normalize( ( nodeVar51 + positionViewDirection ) );
	nodeVar68 = clamp( dot( positionViewDirection, nodeVar67 ), 0.0, 1.0 );
	nodeVar69 = exp2( ( ( ( nodeVar68 * -5.55473 ) - 6.98316 ) * nodeVar68 ) );
	nodeVar70 = ( Roughness * Roughness );
	nodeVar71 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar69 ) ) ) + vec3<f32>( ( 1.0 * nodeVar69 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar70, clamp( dot( normalView, nodeVar51 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar70, clamp( dot( normalView, nodeVar67 ), 0.0, 1.0 ) ) ) );
	nodeVar72 = ( nodeVar54 * nodeVar71 );
	nodeVar73 = ( nodeVar72 * multiScatteringCompensation );
	nodeVar74 = ( directSpecular + nodeVar73 );
	directSpecular = nodeVar74;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar75 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform30 );
	nodeVar76 = floor( nodeVar75 );
	nodeVar77 = nodeVar76;
	nodeVar78 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar79 = getFace( ( object.nodeUniform31 * vec4<f32>( vec3<f32>( nodeVar78.x, ( - nodeVar78.y ), nodeVar78.z ), 1.0 ) ).xyz );
	nodeVar80 = max( ( 4.0 - nodeVar77 ), 0.0 );
	nodeVar77 = max( nodeVar77, 4.0 );
	nodeVar81 = exp2( nodeVar77 );
	nodeVar82 = ( ( getUV( ( object.nodeUniform31 * vec4<f32>( vec3<f32>( nodeVar78.x, ( - nodeVar78.y ), nodeVar78.z ), 1.0 ) ).xyz, nodeVar79 ) * vec2<f32>( ( nodeVar81 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar79 > 2.0 ) ) {

		nodeVar82.y = ( nodeVar82.y + nodeVar81 );
		nodeVar79 = ( nodeVar79 - 3.0 );
		

	}

	nodeVar82.x = ( nodeVar82.x + ( nodeVar79 * nodeVar81 ) );
	nodeVar82.x = ( nodeVar82.x + ( nodeVar80 * ( 3.0 * 16.0 ) ) );
	nodeVar82.y = ( nodeVar82.y + ( 4.0 * ( exp2( object.nodeUniform30 ) - nodeVar81 ) ) );
	nodeVar82.x = ( nodeVar82.x * object.nodeUniform33 );
	nodeVar82.y = ( nodeVar82.y * object.nodeUniform34 );
	nodeVar83 = textureSampleGrad( nodeUniform35, nodeUniform35_sampler, nodeVar82, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar84 = nodeVar83.xyz;
	nodeVar85 = fract( nodeVar75 );

	if ( ( nodeVar85 != 0.0 ) ) {

		nodeVar86 = ( nodeVar76 + 1.0 );
		nodeVar87 = getFace( ( object.nodeUniform31 * vec4<f32>( vec3<f32>( nodeVar78.x, ( - nodeVar78.y ), nodeVar78.z ), 1.0 ) ).xyz );
		nodeVar88 = max( ( 4.0 - nodeVar86 ), 0.0 );
		nodeVar86 = max( nodeVar86, 4.0 );
		nodeVar89 = exp2( nodeVar86 );
		nodeVar90 = ( ( getUV( ( object.nodeUniform31 * vec4<f32>( vec3<f32>( nodeVar78.x, ( - nodeVar78.y ), nodeVar78.z ), 1.0 ) ).xyz, nodeVar87 ) * vec2<f32>( ( nodeVar89 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar87 > 2.0 ) ) {

			nodeVar90.y = ( nodeVar90.y + nodeVar89 );
			nodeVar87 = ( nodeVar87 - 3.0 );
			

		}

		nodeVar90.x = ( nodeVar90.x + ( nodeVar87 * nodeVar89 ) );
		nodeVar90.x = ( nodeVar90.x + ( nodeVar88 * ( 3.0 * 16.0 ) ) );
		nodeVar90.y = ( nodeVar90.y + ( 4.0 * ( exp2( object.nodeUniform30 ) - nodeVar89 ) ) );
		nodeVar90.x = ( nodeVar90.x * object.nodeUniform33 );
		nodeVar90.y = ( nodeVar90.y * object.nodeUniform34 );
		nodeVar91 = textureSampleGrad( nodeUniform35, nodeUniform35_sampler, nodeVar90, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar92 = nodeVar91.xyz;
		nodeVar84 = mix( nodeVar84, nodeVar92, nodeVar85 );
		

	}

	nodeVar93 = ( radiance + ( nodeVar84 * vec3<f32>( object.nodeUniform36 ) ) );
	radiance = nodeVar93;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform30 );
	nodeVar95 = floor( nodeVar94 );
	nodeVar96 = nodeVar95;
	nodeVar97 = getFace( ( object.nodeUniform31 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar98 = max( ( 4.0 - nodeVar96 ), 0.0 );
	nodeVar96 = max( nodeVar96, 4.0 );
	nodeVar99 = exp2( nodeVar96 );
	nodeVar100 = ( ( getUV( ( object.nodeUniform31 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar97 ) * vec2<f32>( ( nodeVar99 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar97 > 2.0 ) ) {

		nodeVar100.y = ( nodeVar100.y + nodeVar99 );
		nodeVar97 = ( nodeVar97 - 3.0 );
		

	}

	nodeVar100.x = ( nodeVar100.x + ( nodeVar97 * nodeVar99 ) );
	nodeVar100.x = ( nodeVar100.x + ( nodeVar98 * ( 3.0 * 16.0 ) ) );
	nodeVar100.y = ( nodeVar100.y + ( 4.0 * ( exp2( object.nodeUniform30 ) - nodeVar99 ) ) );
	nodeVar100.x = ( nodeVar100.x * object.nodeUniform33 );
	nodeVar100.y = ( nodeVar100.y * object.nodeUniform34 );
	nodeVar101 = textureSampleGrad( nodeUniform35, nodeUniform35_sampler, nodeVar100, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar102 = nodeVar101.xyz;
	nodeVar103 = fract( nodeVar94 );

	if ( ( nodeVar103 != 0.0 ) ) {

		nodeVar104 = ( nodeVar95 + 1.0 );
		nodeVar105 = getFace( ( object.nodeUniform31 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar106 = max( ( 4.0 - nodeVar104 ), 0.0 );
		nodeVar104 = max( nodeVar104, 4.0 );
		nodeVar107 = exp2( nodeVar104 );
		nodeVar108 = ( ( getUV( ( object.nodeUniform31 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar105 ) * vec2<f32>( ( nodeVar107 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar105 > 2.0 ) ) {

			nodeVar108.y = ( nodeVar108.y + nodeVar107 );
			nodeVar105 = ( nodeVar105 - 3.0 );
			

		}

		nodeVar108.x = ( nodeVar108.x + ( nodeVar105 * nodeVar107 ) );
		nodeVar108.x = ( nodeVar108.x + ( nodeVar106 * ( 3.0 * 16.0 ) ) );
		nodeVar108.y = ( nodeVar108.y + ( 4.0 * ( exp2( object.nodeUniform30 ) - nodeVar107 ) ) );
		nodeVar108.x = ( nodeVar108.x * object.nodeUniform33 );
		nodeVar108.y = ( nodeVar108.y * object.nodeUniform34 );
		nodeVar109 = textureSampleGrad( nodeUniform35, nodeUniform35_sampler, nodeVar108, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar110 = nodeVar109.xyz;
		nodeVar102 = mix( nodeVar102, nodeVar110, nodeVar103 );
		

	}

	nodeVar111 = ( iblIrradiance + ( ( nodeVar102 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform36 ) ) );
	iblIrradiance = nodeVar111;
	nodeVar112 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar113 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar114 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar115 = ( SpecularF90 * dfg.y );
	nodeVar116 = ( nodeVar114 + vec3<f32>( nodeVar115 ) );
	nodeVar117 = ( nodeVar112 + nodeVar116 );
	nodeVar112 = nodeVar117;
	nodeVar118 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar119 = nodeVar118;
	nodeVar120 = ( nodeVar119 * vec3<f32>( 0.047619 ) );
	nodeVar121 = ( SpecularColor + nodeVar120 );
	nodeVar122 = ( nodeVar116 * nodeVar121 );
	nodeVar123 = ( dfg.x + dfg.y );
	nodeVar124 = ( 1.0 - nodeVar123 );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( vec3<f32>( nodeVar125 ) * nodeVar121 );
	nodeVar127 = ( vec3<f32>( 1.0 ) - nodeVar126 );
	nodeVar128 = nodeVar127;
	nodeVar129 = ( nodeVar122 / nodeVar128 );
	nodeVar130 = ( nodeVar129 * vec3<f32>( nodeVar125 ) );
	nodeVar131 = ( nodeVar113 + nodeVar130 );
	nodeVar113 = nodeVar131;
	nodeVar132 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar133 = ( irradiance * nodeVar132 );
	nodeVar134 = ( nodeVar112 + nodeVar113 );
	nodeVar135 = ( vec3<f32>( 1.0 ) - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( nodeVar133 * nodeVar136 );
	nodeVar138 = nodeVar137;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar139 = ( indirectDiffuse + nodeVar138 );
	indirectDiffuse = nodeVar139;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar141 = ( SpecularF90 * dfg.y );
	nodeVar142 = ( nodeVar140 + vec3<f32>( nodeVar141 ) );
	nodeVar143 = ( singleScatteringDielectric + nodeVar142 );
	singleScatteringDielectric = nodeVar143;
	nodeVar144 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( nodeVar145 * vec3<f32>( 0.047619 ) );
	nodeVar147 = ( SpecularColor + nodeVar146 );
	nodeVar148 = ( nodeVar142 * nodeVar147 );
	nodeVar149 = ( dfg.x + dfg.y );
	nodeVar150 = ( 1.0 - nodeVar149 );
	nodeVar151 = nodeVar150;
	nodeVar152 = ( vec3<f32>( nodeVar151 ) * nodeVar147 );
	nodeVar153 = ( vec3<f32>( 1.0 ) - nodeVar152 );
	nodeVar154 = nodeVar153;
	nodeVar155 = ( nodeVar148 / nodeVar154 );
	nodeVar156 = ( nodeVar155 * vec3<f32>( nodeVar151 ) );
	nodeVar157 = ( multiScatteringDielectric + nodeVar156 );
	multiScatteringDielectric = nodeVar157;
	nodeVar158 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar159 = ( SpecularF90 * dfg.y );
	nodeVar160 = ( nodeVar158 + vec3<f32>( nodeVar159 ) );
	nodeVar161 = ( singleScatteringMetallic + nodeVar160 );
	singleScatteringMetallic = nodeVar161;
	nodeVar162 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar163 = nodeVar162;
	nodeVar164 = ( nodeVar163 * vec3<f32>( 0.047619 ) );
	nodeVar165 = ( DiffuseColor.xyz + nodeVar164 );
	nodeVar166 = ( nodeVar160 * nodeVar165 );
	nodeVar167 = ( dfg.x + dfg.y );
	nodeVar168 = ( 1.0 - nodeVar167 );
	nodeVar169 = nodeVar168;
	nodeVar170 = ( vec3<f32>( nodeVar169 ) * nodeVar165 );
	nodeVar171 = ( vec3<f32>( 1.0 ) - nodeVar170 );
	nodeVar172 = nodeVar171;
	nodeVar173 = ( nodeVar166 / nodeVar172 );
	nodeVar174 = ( nodeVar173 * vec3<f32>( nodeVar169 ) );
	nodeVar175 = ( multiScatteringMetallic + nodeVar174 );
	multiScatteringMetallic = nodeVar175;
	nodeVar176 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar177 = ( radiance * nodeVar176 );
	nodeVar178 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar179 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar180 = ( nodeVar178 * nodeVar179 );
	nodeVar181 = ( nodeVar177 + nodeVar180 );
	nodeVar182 = nodeVar181;
	nodeVar183 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar184 = ( vec3<f32>( 1.0 ) - nodeVar183 );
	nodeVar185 = nodeVar184;
	nodeVar186 = ( DiffuseContribution * nodeVar185 );
	nodeVar187 = ( nodeVar186 * nodeVar179 );
	nodeVar188 = nodeVar187;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar189 = ( indirectSpecular + nodeVar182 );
	indirectSpecular = nodeVar189;
	nodeVar190 = ( indirectDiffuse + nodeVar188 );
	indirectDiffuse = nodeVar190;
	ambientOcclusion = 1.0;
	nodeVar191 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar191;
	nodeVar192 = dot( normalView, positionViewDirection );
	nodeVar193 = ( clamp( nodeVar192, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar194 = ( Roughness * -16.0 );
	nodeVar195 = ( 1.0 - nodeVar194 );
	nodeVar196 = nodeVar195;
	nodeVar197 = ( - nodeVar196 );
	nodeVar198 = exp2( nodeVar197 );
	nodeVar199 = pow( nodeVar193, nodeVar198 );
	nodeVar200 = ( 1.0 - nodeVar199 );
	nodeVar201 = nodeVar200;
	nodeVar202 = ( ambientOcclusion - nodeVar201 );
	nodeVar203 = ( indirectSpecular * vec3<f32>( clamp( nodeVar202, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar203;
	nodeVar204 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar204;
	nodeVar205 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar205;
	nodeVar206 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar206;
	nodeVar207 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar207;

	// result

	output.color = nodeVar207;

	return output;

}
