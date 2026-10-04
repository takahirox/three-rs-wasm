// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform14 : f32
};
@binding( 0 ) @group( 1 )
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
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> Clearcoat : f32;
var<private> ClearcoatRoughness : f32;
var<private> nodeVar2 : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> clearcoatRadiance : vec3<f32>;
var<private> clearcoatSpecularDirect : vec3<f32>;
var<private> clearcoatSpecularIndirect : vec3<f32>;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec2<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : vec4<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> clearcoatNormalView : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec2<f32>;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec2<f32>;
var<private> nodeVar64 : vec4<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec3<f32>;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec2<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
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
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : f32;
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
var<private> nodeVar164 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec4<f32>;

// codes
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

	DiffuseColor = vec4<f32>( ( ( vec3<f32>( 1.0 ) * vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( 0.96467984, 0.37626296, 0.25818297 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = 1.0;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.25, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	IOR = object.nodeUniform3;
	nodeVar1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar1 * nodeVar1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( 0.0 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( 0.0, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - 1.0 ) ) );
	Clearcoat = 1.0;
	nodeVar2 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	ClearcoatRoughness = min( ( max( 0.20000000298023224, 0.0525 ) + max( max( nodeVar2.x, nodeVar2.y ), nodeVar2.z ) ), 1.0 );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	clearcoatRadiance = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar3 = dot( normalView, positionViewDirection );
	nodeVar4 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( nodeVar3, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	nodeVar5 = ( dfg.x + dfg.y );
	nodeVar6 = ( 1.0 / nodeVar5 );
	nodeVar7 = nodeVar6;
	nodeVar8 = ( nodeVar7 - 1.0 );
	nodeVar9 = ( SpecularColorBlended * vec3<f32>( nodeVar8 ) );
	nodeVar10 = ( nodeVar9 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar10;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar11 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform8 );
	nodeVar12 = floor( nodeVar11 );
	nodeVar13 = nodeVar12;
	nodeVar14 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar15 = getFace( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( nodeVar14.x, ( - nodeVar14.y ), nodeVar14.z ), 1.0 ) ).xyz );
	nodeVar16 = max( ( 4.0 - nodeVar13 ), 0.0 );
	nodeVar13 = max( nodeVar13, 4.0 );
	nodeVar17 = exp2( nodeVar13 );
	nodeVar18 = ( ( getUV( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( nodeVar14.x, ( - nodeVar14.y ), nodeVar14.z ), 1.0 ) ).xyz, nodeVar15 ) * vec2<f32>( ( nodeVar17 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar15 > 2.0 ) ) {

		nodeVar18.y = ( nodeVar18.y + nodeVar17 );
		nodeVar15 = ( nodeVar15 - 3.0 );
		

	}

	nodeVar18.x = ( nodeVar18.x + ( nodeVar15 * nodeVar17 ) );
	nodeVar18.x = ( nodeVar18.x + ( nodeVar16 * ( 3.0 * 16.0 ) ) );
	nodeVar18.y = ( nodeVar18.y + ( 4.0 * ( exp2( object.nodeUniform8 ) - nodeVar17 ) ) );
	nodeVar18.x = ( nodeVar18.x * object.nodeUniform11 );
	nodeVar18.y = ( nodeVar18.y * object.nodeUniform12 );
	nodeVar19 = textureSampleGrad( nodeUniform13, nodeUniform13_sampler, nodeVar18, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar20 = nodeVar19.xyz;
	nodeVar21 = fract( nodeVar11 );

	if ( ( nodeVar21 != 0.0 ) ) {

		nodeVar22 = ( nodeVar12 + 1.0 );
		nodeVar23 = getFace( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( nodeVar14.x, ( - nodeVar14.y ), nodeVar14.z ), 1.0 ) ).xyz );
		nodeVar24 = max( ( 4.0 - nodeVar22 ), 0.0 );
		nodeVar22 = max( nodeVar22, 4.0 );
		nodeVar25 = exp2( nodeVar22 );
		nodeVar26 = ( ( getUV( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( nodeVar14.x, ( - nodeVar14.y ), nodeVar14.z ), 1.0 ) ).xyz, nodeVar23 ) * vec2<f32>( ( nodeVar25 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar23 > 2.0 ) ) {

			nodeVar26.y = ( nodeVar26.y + nodeVar25 );
			nodeVar23 = ( nodeVar23 - 3.0 );
			

		}

		nodeVar26.x = ( nodeVar26.x + ( nodeVar23 * nodeVar25 ) );
		nodeVar26.x = ( nodeVar26.x + ( nodeVar24 * ( 3.0 * 16.0 ) ) );
		nodeVar26.y = ( nodeVar26.y + ( 4.0 * ( exp2( object.nodeUniform8 ) - nodeVar25 ) ) );
		nodeVar26.x = ( nodeVar26.x * object.nodeUniform11 );
		nodeVar26.y = ( nodeVar26.y * object.nodeUniform12 );
		nodeVar27 = textureSampleGrad( nodeUniform13, nodeUniform13_sampler, nodeVar26, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar28 = nodeVar27.xyz;
		nodeVar20 = mix( nodeVar20, nodeVar28, nodeVar21 );
		

	}

	nodeVar29 = ( radiance + ( nodeVar20 * vec3<f32>( object.nodeUniform14 ) ) );
	radiance = nodeVar29;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar30 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform8 );
	nodeVar31 = floor( nodeVar30 );
	nodeVar32 = nodeVar31;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar33 = getFace( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar34 = max( ( 4.0 - nodeVar32 ), 0.0 );
	nodeVar32 = max( nodeVar32, 4.0 );
	nodeVar35 = exp2( nodeVar32 );
	nodeVar36 = ( ( getUV( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar33 ) * vec2<f32>( ( nodeVar35 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar33 > 2.0 ) ) {

		nodeVar36.y = ( nodeVar36.y + nodeVar35 );
		nodeVar33 = ( nodeVar33 - 3.0 );
		

	}

	nodeVar36.x = ( nodeVar36.x + ( nodeVar33 * nodeVar35 ) );
	nodeVar36.x = ( nodeVar36.x + ( nodeVar34 * ( 3.0 * 16.0 ) ) );
	nodeVar36.y = ( nodeVar36.y + ( 4.0 * ( exp2( object.nodeUniform8 ) - nodeVar35 ) ) );
	nodeVar36.x = ( nodeVar36.x * object.nodeUniform11 );
	nodeVar36.y = ( nodeVar36.y * object.nodeUniform12 );
	nodeVar37 = textureSampleGrad( nodeUniform13, nodeUniform13_sampler, nodeVar36, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar38 = nodeVar37.xyz;
	nodeVar39 = fract( nodeVar30 );

	if ( ( nodeVar39 != 0.0 ) ) {

		nodeVar40 = ( nodeVar31 + 1.0 );
		nodeVar41 = getFace( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar42 = max( ( 4.0 - nodeVar40 ), 0.0 );
		nodeVar40 = max( nodeVar40, 4.0 );
		nodeVar43 = exp2( nodeVar40 );
		nodeVar44 = ( ( getUV( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar41 ) * vec2<f32>( ( nodeVar43 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar41 > 2.0 ) ) {

			nodeVar44.y = ( nodeVar44.y + nodeVar43 );
			nodeVar41 = ( nodeVar41 - 3.0 );
			

		}

		nodeVar44.x = ( nodeVar44.x + ( nodeVar41 * nodeVar43 ) );
		nodeVar44.x = ( nodeVar44.x + ( nodeVar42 * ( 3.0 * 16.0 ) ) );
		nodeVar44.y = ( nodeVar44.y + ( 4.0 * ( exp2( object.nodeUniform8 ) - nodeVar43 ) ) );
		nodeVar44.x = ( nodeVar44.x * object.nodeUniform11 );
		nodeVar44.y = ( nodeVar44.y * object.nodeUniform12 );
		nodeVar45 = textureSampleGrad( nodeUniform13, nodeUniform13_sampler, nodeVar44, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar46 = nodeVar45.xyz;
		nodeVar38 = mix( nodeVar38, nodeVar46, nodeVar39 );
		

	}

	nodeVar47 = ( iblIrradiance + ( ( nodeVar38 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform14 ) ) );
	iblIrradiance = nodeVar47;
	nodeVar48 = clamp( roughnessToMip( ClearcoatRoughness ), -2.0, object.nodeUniform8 );
	nodeVar49 = floor( nodeVar48 );
	nodeVar50 = nodeVar49;
	clearcoatNormalView = NORMAL_normalView;
	nodeVar51 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), clearcoatNormalView ), clearcoatNormalView, ( ( ( ClearcoatRoughness * ClearcoatRoughness ) * ClearcoatRoughness ) * ClearcoatRoughness ) ) ), 0.0 ) ).xyz );
	nodeVar52 = getFace( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( nodeVar51.x, ( - nodeVar51.y ), nodeVar51.z ), 1.0 ) ).xyz );
	nodeVar53 = max( ( 4.0 - nodeVar50 ), 0.0 );
	nodeVar50 = max( nodeVar50, 4.0 );
	nodeVar54 = exp2( nodeVar50 );
	nodeVar55 = ( ( getUV( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( nodeVar51.x, ( - nodeVar51.y ), nodeVar51.z ), 1.0 ) ).xyz, nodeVar52 ) * vec2<f32>( ( nodeVar54 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar52 > 2.0 ) ) {

		nodeVar55.y = ( nodeVar55.y + nodeVar54 );
		nodeVar52 = ( nodeVar52 - 3.0 );
		

	}

	nodeVar55.x = ( nodeVar55.x + ( nodeVar52 * nodeVar54 ) );
	nodeVar55.x = ( nodeVar55.x + ( nodeVar53 * ( 3.0 * 16.0 ) ) );
	nodeVar55.y = ( nodeVar55.y + ( 4.0 * ( exp2( object.nodeUniform8 ) - nodeVar54 ) ) );
	nodeVar55.x = ( nodeVar55.x * object.nodeUniform11 );
	nodeVar55.y = ( nodeVar55.y * object.nodeUniform12 );
	nodeVar56 = textureSampleGrad( nodeUniform13, nodeUniform13_sampler, nodeVar55, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar57 = nodeVar56.xyz;
	nodeVar58 = fract( nodeVar48 );

	if ( ( nodeVar58 != 0.0 ) ) {

		nodeVar59 = ( nodeVar49 + 1.0 );
		nodeVar60 = getFace( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( nodeVar51.x, ( - nodeVar51.y ), nodeVar51.z ), 1.0 ) ).xyz );
		nodeVar61 = max( ( 4.0 - nodeVar59 ), 0.0 );
		nodeVar59 = max( nodeVar59, 4.0 );
		nodeVar62 = exp2( nodeVar59 );
		nodeVar63 = ( ( getUV( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( nodeVar51.x, ( - nodeVar51.y ), nodeVar51.z ), 1.0 ) ).xyz, nodeVar60 ) * vec2<f32>( ( nodeVar62 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar60 > 2.0 ) ) {

			nodeVar63.y = ( nodeVar63.y + nodeVar62 );
			nodeVar60 = ( nodeVar60 - 3.0 );
			

		}

		nodeVar63.x = ( nodeVar63.x + ( nodeVar60 * nodeVar62 ) );
		nodeVar63.x = ( nodeVar63.x + ( nodeVar61 * ( 3.0 * 16.0 ) ) );
		nodeVar63.y = ( nodeVar63.y + ( 4.0 * ( exp2( object.nodeUniform8 ) - nodeVar62 ) ) );
		nodeVar63.x = ( nodeVar63.x * object.nodeUniform11 );
		nodeVar63.y = ( nodeVar63.y * object.nodeUniform12 );
		nodeVar64 = textureSampleGrad( nodeUniform13, nodeUniform13_sampler, nodeVar63, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar65 = nodeVar64.xyz;
		nodeVar57 = mix( nodeVar57, nodeVar65, nodeVar58 );
		

	}

	nodeVar66 = ( clearcoatRadiance + ( nodeVar57 * vec3<f32>( object.nodeUniform14 ) ) );
	clearcoatRadiance = nodeVar66;
	nodeVar67 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar68 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar70 = ( SpecularF90 * dfg.y );
	nodeVar71 = ( nodeVar69 + vec3<f32>( nodeVar70 ) );
	nodeVar72 = ( nodeVar67 + nodeVar71 );
	nodeVar67 = nodeVar72;
	nodeVar73 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar74 = nodeVar73;
	nodeVar75 = ( nodeVar74 * vec3<f32>( 0.047619 ) );
	nodeVar76 = ( SpecularColor + nodeVar75 );
	nodeVar77 = ( nodeVar71 * nodeVar76 );
	nodeVar78 = ( dfg.x + dfg.y );
	nodeVar79 = ( 1.0 - nodeVar78 );
	nodeVar80 = nodeVar79;
	nodeVar81 = ( vec3<f32>( nodeVar80 ) * nodeVar76 );
	nodeVar82 = ( vec3<f32>( 1.0 ) - nodeVar81 );
	nodeVar83 = nodeVar82;
	nodeVar84 = ( nodeVar77 / nodeVar83 );
	nodeVar85 = ( nodeVar84 * vec3<f32>( nodeVar80 ) );
	nodeVar86 = ( nodeVar68 + nodeVar85 );
	nodeVar68 = nodeVar86;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar87 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar88 = ( irradiance * nodeVar87 );
	nodeVar89 = ( nodeVar67 + nodeVar68 );
	nodeVar90 = ( vec3<f32>( 1.0 ) - nodeVar89 );
	nodeVar91 = nodeVar90;
	nodeVar92 = ( nodeVar88 * nodeVar91 );
	nodeVar93 = nodeVar92;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( indirectDiffuse + nodeVar93 );
	indirectDiffuse = nodeVar94;
	nodeVar95 = dot( clearcoatNormalView, positionViewDirection );
	nodeVar96 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( ClearcoatRoughness, clamp( nodeVar95, 0.0, 1.0 ) ) ).xy;
	nodeVar97 = ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( nodeVar96.x ) ) + vec3<f32>( ( 1.0 * nodeVar96.y ) ) );
	nodeVar98 = ( clearcoatRadiance * nodeVar97 );
	nodeVar99 = ( clearcoatSpecularIndirect + nodeVar98 );
	clearcoatSpecularIndirect = nodeVar99;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar100 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar101 = ( SpecularF90 * dfg.y );
	nodeVar102 = ( nodeVar100 + vec3<f32>( nodeVar101 ) );
	nodeVar103 = ( singleScatteringDielectric + nodeVar102 );
	singleScatteringDielectric = nodeVar103;
	nodeVar104 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( nodeVar105 * vec3<f32>( 0.047619 ) );
	nodeVar107 = ( SpecularColor + nodeVar106 );
	nodeVar108 = ( nodeVar102 * nodeVar107 );
	nodeVar109 = ( dfg.x + dfg.y );
	nodeVar110 = ( 1.0 - nodeVar109 );
	nodeVar111 = nodeVar110;
	nodeVar112 = ( vec3<f32>( nodeVar111 ) * nodeVar107 );
	nodeVar113 = ( vec3<f32>( 1.0 ) - nodeVar112 );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar108 / nodeVar114 );
	nodeVar116 = ( nodeVar115 * vec3<f32>( nodeVar111 ) );
	nodeVar117 = ( multiScatteringDielectric + nodeVar116 );
	multiScatteringDielectric = nodeVar117;
	nodeVar118 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar119 = ( SpecularF90 * dfg.y );
	nodeVar120 = ( nodeVar118 + vec3<f32>( nodeVar119 ) );
	nodeVar121 = ( singleScatteringMetallic + nodeVar120 );
	singleScatteringMetallic = nodeVar121;
	nodeVar122 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( nodeVar123 * vec3<f32>( 0.047619 ) );
	nodeVar125 = ( DiffuseColor.xyz + nodeVar124 );
	nodeVar126 = ( nodeVar120 * nodeVar125 );
	nodeVar127 = ( dfg.x + dfg.y );
	nodeVar128 = ( 1.0 - nodeVar127 );
	nodeVar129 = nodeVar128;
	nodeVar130 = ( vec3<f32>( nodeVar129 ) * nodeVar125 );
	nodeVar131 = ( vec3<f32>( 1.0 ) - nodeVar130 );
	nodeVar132 = nodeVar131;
	nodeVar133 = ( nodeVar126 / nodeVar132 );
	nodeVar134 = ( nodeVar133 * vec3<f32>( nodeVar129 ) );
	nodeVar135 = ( multiScatteringMetallic + nodeVar134 );
	multiScatteringMetallic = nodeVar135;
	nodeVar136 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar137 = ( radiance * nodeVar136 );
	nodeVar138 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar139 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar140 = ( nodeVar138 * nodeVar139 );
	nodeVar141 = ( nodeVar137 + nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar144 = ( vec3<f32>( 1.0 ) - nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( DiffuseContribution * nodeVar145 );
	nodeVar147 = ( nodeVar146 * nodeVar139 );
	nodeVar148 = nodeVar147;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar149 = ( indirectSpecular + nodeVar142 );
	indirectSpecular = nodeVar149;
	nodeVar150 = ( indirectDiffuse + nodeVar148 );
	indirectDiffuse = nodeVar150;
	ambientOcclusion = 1.0;
	nodeVar151 = ( clearcoatSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	clearcoatSpecularIndirect = nodeVar151;
	nodeVar152 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar152;
	nodeVar153 = dot( normalView, positionViewDirection );
	nodeVar154 = ( clamp( nodeVar153, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar155 = ( Roughness * -16.0 );
	nodeVar156 = ( 1.0 - nodeVar155 );
	nodeVar157 = nodeVar156;
	nodeVar158 = ( - nodeVar157 );
	nodeVar159 = exp2( nodeVar158 );
	nodeVar160 = pow( nodeVar154, nodeVar159 );
	nodeVar161 = ( 1.0 - nodeVar160 );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( ambientOcclusion - nodeVar162 );
	nodeVar164 = ( indirectSpecular * vec3<f32>( clamp( nodeVar163, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar164;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar165 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar165;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar166 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar166;
	nodeVar167 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar167;
	nodeVar168 = dot( clearcoatNormalView, positionViewDirection );
	nodeVar169 = clamp( nodeVar168, 0.0, 1.0 );
	nodeVar170 = exp2( ( ( ( nodeVar169 * -5.55473 ) - 6.98316 ) * nodeVar169 ) );
	nodeVar171 = ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( ( 1.0 - nodeVar170 ) ) ) + vec3<f32>( ( 1.0 * nodeVar170 ) ) );
	nodeVar172 = ( vec3<f32>( Clearcoat ) * nodeVar171 );
	nodeVar173 = ( vec3<f32>( 1.0 ) - nodeVar172 );
	nodeVar174 = nodeVar173;
	nodeVar175 = ( outgoingLight * nodeVar174 );
	nodeVar176 = ( clearcoatSpecularDirect + clearcoatSpecularIndirect );
	nodeVar177 = ( nodeVar176 * vec3<f32>( Clearcoat ) );
	nodeVar178 = ( nodeVar175 + nodeVar177 );
	outgoingLight = nodeVar178;
	nodeVar179 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar179;

	// result

	output.color = nodeVar179;

	return output;

}
