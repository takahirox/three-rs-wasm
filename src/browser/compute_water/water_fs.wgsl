// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform22 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec2<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : vec2<f32>;
var<private> nodeVar58 : vec4<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec2<f32>;
var<private> nodeVar68 : vec4<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec2<f32>;
var<private> nodeVar76 : vec4<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : vec3<f32>;
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
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
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
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec4<f32>;

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
	@location( 2 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform3, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	Metalness = object.nodeUniform5;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform6, 0.0525 ) + max( max( nodeVar1.x, nodeVar1.y ), nodeVar1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform5 ) ) );
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	normalView = nodeVarying4;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = dot( normalView, positionViewDirection );
	nodeVar7 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( nodeVar6, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar7;
	nodeVar8 = ( dfg.x + dfg.y );
	nodeVar9 = ( 1.0 / nodeVar8 );
	nodeVar10 = nodeVar9;
	nodeVar11 = ( nodeVar10 - 1.0 );
	nodeVar12 = ( SpecularColorBlended * vec3<f32>( nodeVar11 ) );
	nodeVar13 = ( nodeVar12 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar13;
	nodeVar14 = ( render.nodeUniform14 - render.nodeUniform15 );
	nodeVar15 = vec4<f32>( nodeVar14, 0.0 );
	nodeVar16 = ( render.cameraViewMatrix * nodeVar15 );
	nodeVar17 = normalize( nodeVar16.xyz );
	nodeVar18 = nodeVar17;
	nodeVar19 = dot( normalView, nodeVar18 );
	nodeVar20 = ( vec3<f32>( clamp( nodeVar19, 0.0, 1.0 ) ) * render.nodeUniform16 );
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
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar42 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform17 );
	nodeVar43 = floor( nodeVar42 );
	nodeVar44 = nodeVar43;
	nodeVar45 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar46 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar45.x, ( - nodeVar45.y ), nodeVar45.z ), 1.0 ) ).xyz );
	nodeVar47 = max( ( 4.0 - nodeVar44 ), 0.0 );
	nodeVar44 = max( nodeVar44, 4.0 );
	nodeVar48 = exp2( nodeVar44 );
	nodeVar49 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar45.x, ( - nodeVar45.y ), nodeVar45.z ), 1.0 ) ).xyz, nodeVar46 ) * vec2<f32>( ( nodeVar48 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar46 > 2.0 ) ) {

		nodeVar49.y = ( nodeVar49.y + nodeVar48 );
		nodeVar46 = ( nodeVar46 - 3.0 );
		

	}

	nodeVar49.x = ( nodeVar49.x + ( nodeVar46 * nodeVar48 ) );
	nodeVar49.x = ( nodeVar49.x + ( nodeVar47 * ( 3.0 * 16.0 ) ) );
	nodeVar49.y = ( nodeVar49.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar48 ) ) );
	nodeVar49.x = ( nodeVar49.x * object.nodeUniform20 );
	nodeVar49.y = ( nodeVar49.y * object.nodeUniform21 );
	nodeVar50 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar49, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar51 = nodeVar50.xyz;
	nodeVar52 = fract( nodeVar42 );

	if ( ( nodeVar52 != 0.0 ) ) {

		nodeVar53 = ( nodeVar43 + 1.0 );
		nodeVar54 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar45.x, ( - nodeVar45.y ), nodeVar45.z ), 1.0 ) ).xyz );
		nodeVar55 = max( ( 4.0 - nodeVar53 ), 0.0 );
		nodeVar53 = max( nodeVar53, 4.0 );
		nodeVar56 = exp2( nodeVar53 );
		nodeVar57 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar45.x, ( - nodeVar45.y ), nodeVar45.z ), 1.0 ) ).xyz, nodeVar54 ) * vec2<f32>( ( nodeVar56 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar54 > 2.0 ) ) {

			nodeVar57.y = ( nodeVar57.y + nodeVar56 );
			nodeVar54 = ( nodeVar54 - 3.0 );
			

		}

		nodeVar57.x = ( nodeVar57.x + ( nodeVar54 * nodeVar56 ) );
		nodeVar57.x = ( nodeVar57.x + ( nodeVar55 * ( 3.0 * 16.0 ) ) );
		nodeVar57.y = ( nodeVar57.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar56 ) ) );
		nodeVar57.x = ( nodeVar57.x * object.nodeUniform20 );
		nodeVar57.y = ( nodeVar57.y * object.nodeUniform21 );
		nodeVar58 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar57, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar59 = nodeVar58.xyz;
		nodeVar51 = mix( nodeVar51, nodeVar59, nodeVar52 );
		

	}

	nodeVar60 = ( radiance + ( nodeVar51 * vec3<f32>( object.nodeUniform23 ) ) );
	radiance = nodeVar60;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar61 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform17 );
	nodeVar62 = floor( nodeVar61 );
	nodeVar63 = nodeVar62;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar64 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar65 = max( ( 4.0 - nodeVar63 ), 0.0 );
	nodeVar63 = max( nodeVar63, 4.0 );
	nodeVar66 = exp2( nodeVar63 );
	nodeVar67 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar64 ) * vec2<f32>( ( nodeVar66 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar64 > 2.0 ) ) {

		nodeVar67.y = ( nodeVar67.y + nodeVar66 );
		nodeVar64 = ( nodeVar64 - 3.0 );
		

	}

	nodeVar67.x = ( nodeVar67.x + ( nodeVar64 * nodeVar66 ) );
	nodeVar67.x = ( nodeVar67.x + ( nodeVar65 * ( 3.0 * 16.0 ) ) );
	nodeVar67.y = ( nodeVar67.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar66 ) ) );
	nodeVar67.x = ( nodeVar67.x * object.nodeUniform20 );
	nodeVar67.y = ( nodeVar67.y * object.nodeUniform21 );
	nodeVar68 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar67, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar69 = nodeVar68.xyz;
	nodeVar70 = fract( nodeVar61 );

	if ( ( nodeVar70 != 0.0 ) ) {

		nodeVar71 = ( nodeVar62 + 1.0 );
		nodeVar72 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar73 = max( ( 4.0 - nodeVar71 ), 0.0 );
		nodeVar71 = max( nodeVar71, 4.0 );
		nodeVar74 = exp2( nodeVar71 );
		nodeVar75 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar72 ) * vec2<f32>( ( nodeVar74 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar72 > 2.0 ) ) {

			nodeVar75.y = ( nodeVar75.y + nodeVar74 );
			nodeVar72 = ( nodeVar72 - 3.0 );
			

		}

		nodeVar75.x = ( nodeVar75.x + ( nodeVar72 * nodeVar74 ) );
		nodeVar75.x = ( nodeVar75.x + ( nodeVar73 * ( 3.0 * 16.0 ) ) );
		nodeVar75.y = ( nodeVar75.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar74 ) ) );
		nodeVar75.x = ( nodeVar75.x * object.nodeUniform20 );
		nodeVar75.y = ( nodeVar75.y * object.nodeUniform21 );
		nodeVar76 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar75, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar77 = nodeVar76.xyz;
		nodeVar69 = mix( nodeVar69, nodeVar77, nodeVar70 );
		

	}

	nodeVar78 = ( iblIrradiance + ( ( nodeVar69 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform23 ) ) );
	iblIrradiance = nodeVar78;
	nodeVar79 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar80 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar81 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar82 = ( SpecularF90 * dfg.y );
	nodeVar83 = ( nodeVar81 + vec3<f32>( nodeVar82 ) );
	nodeVar84 = ( nodeVar79 + nodeVar83 );
	nodeVar79 = nodeVar84;
	nodeVar85 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar86 = nodeVar85;
	nodeVar87 = ( nodeVar86 * vec3<f32>( 0.047619 ) );
	nodeVar88 = ( SpecularColor + nodeVar87 );
	nodeVar89 = ( nodeVar83 * nodeVar88 );
	nodeVar90 = ( dfg.x + dfg.y );
	nodeVar91 = ( 1.0 - nodeVar90 );
	nodeVar92 = nodeVar91;
	nodeVar93 = ( vec3<f32>( nodeVar92 ) * nodeVar88 );
	nodeVar94 = ( vec3<f32>( 1.0 ) - nodeVar93 );
	nodeVar95 = nodeVar94;
	nodeVar96 = ( nodeVar89 / nodeVar95 );
	nodeVar97 = ( nodeVar96 * vec3<f32>( nodeVar92 ) );
	nodeVar98 = ( nodeVar80 + nodeVar97 );
	nodeVar80 = nodeVar98;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar99 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar100 = ( irradiance * nodeVar99 );
	nodeVar101 = ( nodeVar79 + nodeVar80 );
	nodeVar102 = ( vec3<f32>( 1.0 ) - nodeVar101 );
	nodeVar103 = nodeVar102;
	nodeVar104 = ( nodeVar100 * nodeVar103 );
	nodeVar105 = nodeVar104;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar106 = ( indirectDiffuse + nodeVar105 );
	indirectDiffuse = nodeVar106;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar107 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar108 = ( SpecularF90 * dfg.y );
	nodeVar109 = ( nodeVar107 + vec3<f32>( nodeVar108 ) );
	nodeVar110 = ( singleScatteringDielectric + nodeVar109 );
	singleScatteringDielectric = nodeVar110;
	nodeVar111 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( nodeVar112 * vec3<f32>( 0.047619 ) );
	nodeVar114 = ( SpecularColor + nodeVar113 );
	nodeVar115 = ( nodeVar109 * nodeVar114 );
	nodeVar116 = ( dfg.x + dfg.y );
	nodeVar117 = ( 1.0 - nodeVar116 );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( vec3<f32>( nodeVar118 ) * nodeVar114 );
	nodeVar120 = ( vec3<f32>( 1.0 ) - nodeVar119 );
	nodeVar121 = nodeVar120;
	nodeVar122 = ( nodeVar115 / nodeVar121 );
	nodeVar123 = ( nodeVar122 * vec3<f32>( nodeVar118 ) );
	nodeVar124 = ( multiScatteringDielectric + nodeVar123 );
	multiScatteringDielectric = nodeVar124;
	nodeVar125 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar126 = ( SpecularF90 * dfg.y );
	nodeVar127 = ( nodeVar125 + vec3<f32>( nodeVar126 ) );
	nodeVar128 = ( singleScatteringMetallic + nodeVar127 );
	singleScatteringMetallic = nodeVar128;
	nodeVar129 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar130 = nodeVar129;
	nodeVar131 = ( nodeVar130 * vec3<f32>( 0.047619 ) );
	nodeVar132 = ( DiffuseColor.xyz + nodeVar131 );
	nodeVar133 = ( nodeVar127 * nodeVar132 );
	nodeVar134 = ( dfg.x + dfg.y );
	nodeVar135 = ( 1.0 - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( vec3<f32>( nodeVar136 ) * nodeVar132 );
	nodeVar138 = ( vec3<f32>( 1.0 ) - nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( nodeVar133 / nodeVar139 );
	nodeVar141 = ( nodeVar140 * vec3<f32>( nodeVar136 ) );
	nodeVar142 = ( multiScatteringMetallic + nodeVar141 );
	multiScatteringMetallic = nodeVar142;
	nodeVar143 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar144 = ( radiance * nodeVar143 );
	nodeVar145 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar146 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar147 = ( nodeVar145 * nodeVar146 );
	nodeVar148 = ( nodeVar144 + nodeVar147 );
	nodeVar149 = nodeVar148;
	nodeVar150 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar151 = ( vec3<f32>( 1.0 ) - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( DiffuseContribution * nodeVar152 );
	nodeVar154 = ( nodeVar153 * nodeVar146 );
	nodeVar155 = nodeVar154;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar156 = ( indirectSpecular + nodeVar149 );
	indirectSpecular = nodeVar156;
	nodeVar157 = ( indirectDiffuse + nodeVar155 );
	indirectDiffuse = nodeVar157;
	ambientOcclusion = 1.0;
	nodeVar158 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar158;
	nodeVar159 = dot( normalView, positionViewDirection );
	nodeVar160 = ( clamp( nodeVar159, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar161 = ( Roughness * -16.0 );
	nodeVar162 = ( 1.0 - nodeVar161 );
	nodeVar163 = nodeVar162;
	nodeVar164 = ( - nodeVar163 );
	nodeVar165 = exp2( nodeVar164 );
	nodeVar166 = pow( nodeVar160, nodeVar165 );
	nodeVar167 = ( 1.0 - nodeVar166 );
	nodeVar168 = nodeVar167;
	nodeVar169 = ( ambientOcclusion - nodeVar168 );
	nodeVar170 = ( indirectSpecular * vec3<f32>( clamp( nodeVar169, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar170;
	nodeVar171 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar171;
	nodeVar172 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar172;
	nodeVar173 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar173;
	nodeVar174 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar174;

	// result

	output.color = nodeVar174;

	return output;

}
