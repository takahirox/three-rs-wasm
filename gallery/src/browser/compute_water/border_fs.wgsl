// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform19 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform20 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
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
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec3<f32>;
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
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec2<f32>;
var<private> nodeVar63 : vec4<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec2<f32>;
var<private> nodeVar71 : vec4<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec4<f32>;

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
	@location( 1 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

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
	nodeVar9 = ( render.nodeUniform11 - render.nodeUniform12 );
	nodeVar10 = vec4<f32>( nodeVar9, 0.0 );
	nodeVar11 = ( render.cameraViewMatrix * nodeVar10 );
	nodeVar12 = normalize( nodeVar11.xyz );
	nodeVar13 = nodeVar12;
	nodeVar14 = dot( normalView, nodeVar13 );
	nodeVar15 = ( vec3<f32>( clamp( nodeVar14, 0.0, 1.0 ) ) * render.nodeUniform13 );
	nodeVar16 = nodeVar15;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar17 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar18 = ( nodeVar16 * nodeVar17 );
	nodeVar19 = ( nodeVar13 + positionViewDirection );
	nodeVar20 = normalize( nodeVar19 );
	nodeVar21 = dot( positionViewDirection, nodeVar20 );
	nodeVar22 = clamp( nodeVar21, 0.0, 1.0 );
	nodeVar23 = exp2( ( ( ( nodeVar22 * -5.55473 ) - 6.98316 ) * nodeVar22 ) );
	nodeVar24 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar23 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar23 ) ) );
	nodeVar25 = ( vec3<f32>( 1.0 ) - nodeVar24 );
	nodeVar26 = nodeVar25;
	nodeVar27 = ( nodeVar18 * nodeVar26 );
	nodeVar28 = ( directDiffuse + nodeVar27 );
	directDiffuse = nodeVar28;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar29 = normalize( ( nodeVar13 + positionViewDirection ) );
	nodeVar30 = clamp( dot( positionViewDirection, nodeVar29 ), 0.0, 1.0 );
	nodeVar31 = exp2( ( ( ( nodeVar30 * -5.55473 ) - 6.98316 ) * nodeVar30 ) );
	nodeVar32 = ( Roughness * Roughness );
	nodeVar33 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar31 ) ) ) + vec3<f32>( ( 1.0 * nodeVar31 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar32, clamp( dot( normalView, nodeVar13 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar32, clamp( dot( normalView, nodeVar29 ), 0.0, 1.0 ) ) ) );
	nodeVar34 = ( nodeVar16 * nodeVar33 );
	nodeVar35 = ( nodeVar34 * multiScatteringCompensation );
	nodeVar36 = ( directSpecular + nodeVar35 );
	directSpecular = nodeVar36;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar37 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform14 );
	nodeVar38 = floor( nodeVar37 );
	nodeVar39 = nodeVar38;
	nodeVar40 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar41 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar40.x, ( - nodeVar40.y ), nodeVar40.z ), 1.0 ) ).xyz );
	nodeVar42 = max( ( 4.0 - nodeVar39 ), 0.0 );
	nodeVar39 = max( nodeVar39, 4.0 );
	nodeVar43 = exp2( nodeVar39 );
	nodeVar44 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar40.x, ( - nodeVar40.y ), nodeVar40.z ), 1.0 ) ).xyz, nodeVar41 ) * vec2<f32>( ( nodeVar43 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar41 > 2.0 ) ) {

		nodeVar44.y = ( nodeVar44.y + nodeVar43 );
		nodeVar41 = ( nodeVar41 - 3.0 );
		

	}

	nodeVar44.x = ( nodeVar44.x + ( nodeVar41 * nodeVar43 ) );
	nodeVar44.x = ( nodeVar44.x + ( nodeVar42 * ( 3.0 * 16.0 ) ) );
	nodeVar44.y = ( nodeVar44.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar43 ) ) );
	nodeVar44.x = ( nodeVar44.x * object.nodeUniform17 );
	nodeVar44.y = ( nodeVar44.y * object.nodeUniform18 );
	nodeVar45 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar44, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar46 = nodeVar45.xyz;
	nodeVar47 = fract( nodeVar37 );

	if ( ( nodeVar47 != 0.0 ) ) {

		nodeVar48 = ( nodeVar38 + 1.0 );
		nodeVar49 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar40.x, ( - nodeVar40.y ), nodeVar40.z ), 1.0 ) ).xyz );
		nodeVar50 = max( ( 4.0 - nodeVar48 ), 0.0 );
		nodeVar48 = max( nodeVar48, 4.0 );
		nodeVar51 = exp2( nodeVar48 );
		nodeVar52 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar40.x, ( - nodeVar40.y ), nodeVar40.z ), 1.0 ) ).xyz, nodeVar49 ) * vec2<f32>( ( nodeVar51 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar49 > 2.0 ) ) {

			nodeVar52.y = ( nodeVar52.y + nodeVar51 );
			nodeVar49 = ( nodeVar49 - 3.0 );
			

		}

		nodeVar52.x = ( nodeVar52.x + ( nodeVar49 * nodeVar51 ) );
		nodeVar52.x = ( nodeVar52.x + ( nodeVar50 * ( 3.0 * 16.0 ) ) );
		nodeVar52.y = ( nodeVar52.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar51 ) ) );
		nodeVar52.x = ( nodeVar52.x * object.nodeUniform17 );
		nodeVar52.y = ( nodeVar52.y * object.nodeUniform18 );
		nodeVar53 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar52, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar54 = nodeVar53.xyz;
		nodeVar46 = mix( nodeVar46, nodeVar54, nodeVar47 );
		

	}

	nodeVar55 = ( radiance + ( nodeVar46 * vec3<f32>( object.nodeUniform20 ) ) );
	radiance = nodeVar55;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar56 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform14 );
	nodeVar57 = floor( nodeVar56 );
	nodeVar58 = nodeVar57;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar59 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar60 = max( ( 4.0 - nodeVar58 ), 0.0 );
	nodeVar58 = max( nodeVar58, 4.0 );
	nodeVar61 = exp2( nodeVar58 );
	nodeVar62 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar59 ) * vec2<f32>( ( nodeVar61 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar59 > 2.0 ) ) {

		nodeVar62.y = ( nodeVar62.y + nodeVar61 );
		nodeVar59 = ( nodeVar59 - 3.0 );
		

	}

	nodeVar62.x = ( nodeVar62.x + ( nodeVar59 * nodeVar61 ) );
	nodeVar62.x = ( nodeVar62.x + ( nodeVar60 * ( 3.0 * 16.0 ) ) );
	nodeVar62.y = ( nodeVar62.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar61 ) ) );
	nodeVar62.x = ( nodeVar62.x * object.nodeUniform17 );
	nodeVar62.y = ( nodeVar62.y * object.nodeUniform18 );
	nodeVar63 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar62, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar64 = nodeVar63.xyz;
	nodeVar65 = fract( nodeVar56 );

	if ( ( nodeVar65 != 0.0 ) ) {

		nodeVar66 = ( nodeVar57 + 1.0 );
		nodeVar67 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar68 = max( ( 4.0 - nodeVar66 ), 0.0 );
		nodeVar66 = max( nodeVar66, 4.0 );
		nodeVar69 = exp2( nodeVar66 );
		nodeVar70 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar67 ) * vec2<f32>( ( nodeVar69 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar67 > 2.0 ) ) {

			nodeVar70.y = ( nodeVar70.y + nodeVar69 );
			nodeVar67 = ( nodeVar67 - 3.0 );
			

		}

		nodeVar70.x = ( nodeVar70.x + ( nodeVar67 * nodeVar69 ) );
		nodeVar70.x = ( nodeVar70.x + ( nodeVar68 * ( 3.0 * 16.0 ) ) );
		nodeVar70.y = ( nodeVar70.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar69 ) ) );
		nodeVar70.x = ( nodeVar70.x * object.nodeUniform17 );
		nodeVar70.y = ( nodeVar70.y * object.nodeUniform18 );
		nodeVar71 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar70, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar72 = nodeVar71.xyz;
		nodeVar64 = mix( nodeVar64, nodeVar72, nodeVar65 );
		

	}

	nodeVar73 = ( iblIrradiance + ( ( nodeVar64 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform20 ) ) );
	iblIrradiance = nodeVar73;
	nodeVar74 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar75 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar76 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar77 = ( SpecularF90 * dfg.y );
	nodeVar78 = ( nodeVar76 + vec3<f32>( nodeVar77 ) );
	nodeVar79 = ( nodeVar74 + nodeVar78 );
	nodeVar74 = nodeVar79;
	nodeVar80 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar81 = nodeVar80;
	nodeVar82 = ( nodeVar81 * vec3<f32>( 0.047619 ) );
	nodeVar83 = ( SpecularColor + nodeVar82 );
	nodeVar84 = ( nodeVar78 * nodeVar83 );
	nodeVar85 = ( dfg.x + dfg.y );
	nodeVar86 = ( 1.0 - nodeVar85 );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( vec3<f32>( nodeVar87 ) * nodeVar83 );
	nodeVar89 = ( vec3<f32>( 1.0 ) - nodeVar88 );
	nodeVar90 = nodeVar89;
	nodeVar91 = ( nodeVar84 / nodeVar90 );
	nodeVar92 = ( nodeVar91 * vec3<f32>( nodeVar87 ) );
	nodeVar93 = ( nodeVar75 + nodeVar92 );
	nodeVar75 = nodeVar93;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar95 = ( irradiance * nodeVar94 );
	nodeVar96 = ( nodeVar74 + nodeVar75 );
	nodeVar97 = ( vec3<f32>( 1.0 ) - nodeVar96 );
	nodeVar98 = nodeVar97;
	nodeVar99 = ( nodeVar95 * nodeVar98 );
	nodeVar100 = nodeVar99;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar101 = ( indirectDiffuse + nodeVar100 );
	indirectDiffuse = nodeVar101;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar102 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar103 = ( SpecularF90 * dfg.y );
	nodeVar104 = ( nodeVar102 + vec3<f32>( nodeVar103 ) );
	nodeVar105 = ( singleScatteringDielectric + nodeVar104 );
	singleScatteringDielectric = nodeVar105;
	nodeVar106 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( nodeVar107 * vec3<f32>( 0.047619 ) );
	nodeVar109 = ( SpecularColor + nodeVar108 );
	nodeVar110 = ( nodeVar104 * nodeVar109 );
	nodeVar111 = ( dfg.x + dfg.y );
	nodeVar112 = ( 1.0 - nodeVar111 );
	nodeVar113 = nodeVar112;
	nodeVar114 = ( vec3<f32>( nodeVar113 ) * nodeVar109 );
	nodeVar115 = ( vec3<f32>( 1.0 ) - nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( nodeVar110 / nodeVar116 );
	nodeVar118 = ( nodeVar117 * vec3<f32>( nodeVar113 ) );
	nodeVar119 = ( multiScatteringDielectric + nodeVar118 );
	multiScatteringDielectric = nodeVar119;
	nodeVar120 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar121 = ( SpecularF90 * dfg.y );
	nodeVar122 = ( nodeVar120 + vec3<f32>( nodeVar121 ) );
	nodeVar123 = ( singleScatteringMetallic + nodeVar122 );
	singleScatteringMetallic = nodeVar123;
	nodeVar124 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( nodeVar125 * vec3<f32>( 0.047619 ) );
	nodeVar127 = ( DiffuseColor.xyz + nodeVar126 );
	nodeVar128 = ( nodeVar122 * nodeVar127 );
	nodeVar129 = ( dfg.x + dfg.y );
	nodeVar130 = ( 1.0 - nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( vec3<f32>( nodeVar131 ) * nodeVar127 );
	nodeVar133 = ( vec3<f32>( 1.0 ) - nodeVar132 );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( nodeVar128 / nodeVar134 );
	nodeVar136 = ( nodeVar135 * vec3<f32>( nodeVar131 ) );
	nodeVar137 = ( multiScatteringMetallic + nodeVar136 );
	multiScatteringMetallic = nodeVar137;
	nodeVar138 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar139 = ( radiance * nodeVar138 );
	nodeVar140 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar141 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar142 = ( nodeVar140 * nodeVar141 );
	nodeVar143 = ( nodeVar139 + nodeVar142 );
	nodeVar144 = nodeVar143;
	nodeVar145 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar146 = ( vec3<f32>( 1.0 ) - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( DiffuseContribution * nodeVar147 );
	nodeVar149 = ( nodeVar148 * nodeVar141 );
	nodeVar150 = nodeVar149;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar151 = ( indirectSpecular + nodeVar144 );
	indirectSpecular = nodeVar151;
	nodeVar152 = ( indirectDiffuse + nodeVar150 );
	indirectDiffuse = nodeVar152;
	ambientOcclusion = 1.0;
	nodeVar153 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar153;
	nodeVar154 = dot( normalView, positionViewDirection );
	nodeVar155 = ( clamp( nodeVar154, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar156 = ( Roughness * -16.0 );
	nodeVar157 = ( 1.0 - nodeVar156 );
	nodeVar158 = nodeVar157;
	nodeVar159 = ( - nodeVar158 );
	nodeVar160 = exp2( nodeVar159 );
	nodeVar161 = pow( nodeVar155, nodeVar160 );
	nodeVar162 = ( 1.0 - nodeVar161 );
	nodeVar163 = nodeVar162;
	nodeVar164 = ( ambientOcclusion - nodeVar163 );
	nodeVar165 = ( indirectSpecular * vec3<f32>( clamp( nodeVar164, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar165;
	nodeVar166 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar166;
	nodeVar167 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar167;
	nodeVar168 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar168;
	nodeVar169 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar169;

	// result

	output.color = nodeVar169;

	return output;

}
