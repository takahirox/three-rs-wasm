// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : mat4x4<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform16 : f32
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
var<private> Sheen : vec3<f32>;
var<private> SheenRoughness : f32;
var<private> nodeVar2 : vec2<f32>;
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
var<private> sheenSpecularDirect : vec3<f32>;
var<private> sheenSpecularIndirect : vec3<f32>;
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
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : vec4<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec2<f32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<f32>;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> irradiance : vec3<f32>;
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
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec3<f32>;
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
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : f32;
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
var<private> nodeVar169 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec4<f32>;

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
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 1.0 ) * vec3<f32>( 0.02, 0.02, 0.02 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.8, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	IOR = object.nodeUniform4;
	nodeVar1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar1 * nodeVar1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform5 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform5, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	Sheen = ( vec3<f32>( 1.0 ) * vec3<f32>( 0.4, 0.4, 0.4 ) );
	SheenRoughness = pow( 0.5, 1.5 );
	nodeVar2 = ( vec2<f32>( cos( 0.0 ), sin( 0.0 ) ) * vec2<f32>( 0.0 ) );
	Anisotropy = length( nodeVar2 );

	if ( ( Anisotropy == 0.0 ) ) {

		nodeVar2 = vec2<f32>( 1.0, 0.0 );
		

	} else {

		nodeVar2 = ( nodeVar2 / vec2<f32>( Anisotropy ) );
		Anisotropy = clamp( Anisotropy, 0.0, 1.0 );
		

	}

	AlphaT = mix( ( Roughness * Roughness ), 1.0, ( Anisotropy * Anisotropy ) );
	tangentView = normalize( v_tangentView );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	bitangentView = normalize( ( cross( normalView, tangentView ) * vec3<f32>( nodeVarying7.w ) ) );
	TBNViewMatrix = mat3x3<f32>( tangentView, bitangentView, normalView );
	AnisotropyT = ( ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar2.x ) ) + ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar2.y ) ) );
	AnisotropyB = ( ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar2.x ) ) - ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar2.y ) ) );
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	sheenSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	sheenSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar3 = dot( normalView, positionViewDirection );
	nodeVar4 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( Roughness, clamp( nodeVar3, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	nodeVar5 = ( dfg.x + dfg.y );
	nodeVar6 = ( 1.0 / nodeVar5 );
	nodeVar7 = nodeVar6;
	nodeVar8 = ( nodeVar7 - 1.0 );
	nodeVar9 = ( SpecularColorBlended * vec3<f32>( nodeVar8 ) );
	nodeVar10 = ( nodeVar9 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar10;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar11 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform10 );
	nodeVar12 = floor( nodeVar11 );
	nodeVar13 = nodeVar12;
	nodeVar14 = ( 1.0 - ( Anisotropy * ( 1.0 - Roughness ) ) );
	nodeVar15 = ( nodeVar14 * nodeVar14 );
	nodeVar16 = normalize( mix( normalize( cross( cross( AnisotropyB, positionViewDirection ), AnisotropyB ) ), normalView, ( nodeVar15 * nodeVar15 ) ) );
	nodeVar17 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), nodeVar16 ), nodeVar16, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar18 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar17.x, ( - nodeVar17.y ), nodeVar17.z ), 1.0 ) ).xyz );
	nodeVar19 = max( ( 4.0 - nodeVar13 ), 0.0 );
	nodeVar13 = max( nodeVar13, 4.0 );
	nodeVar20 = exp2( nodeVar13 );
	nodeVar21 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar17.x, ( - nodeVar17.y ), nodeVar17.z ), 1.0 ) ).xyz, nodeVar18 ) * vec2<f32>( ( nodeVar20 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar18 > 2.0 ) ) {

		nodeVar21.y = ( nodeVar21.y + nodeVar20 );
		nodeVar18 = ( nodeVar18 - 3.0 );
		

	}

	nodeVar21.x = ( nodeVar21.x + ( nodeVar18 * nodeVar20 ) );
	nodeVar21.x = ( nodeVar21.x + ( nodeVar19 * ( 3.0 * 16.0 ) ) );
	nodeVar21.y = ( nodeVar21.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar20 ) ) );
	nodeVar21.x = ( nodeVar21.x * object.nodeUniform13 );
	nodeVar21.y = ( nodeVar21.y * object.nodeUniform14 );
	nodeVar22 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar21, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar23 = nodeVar22.xyz;
	nodeVar24 = fract( nodeVar11 );

	if ( ( nodeVar24 != 0.0 ) ) {

		nodeVar25 = ( nodeVar12 + 1.0 );
		nodeVar26 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar17.x, ( - nodeVar17.y ), nodeVar17.z ), 1.0 ) ).xyz );
		nodeVar27 = max( ( 4.0 - nodeVar25 ), 0.0 );
		nodeVar25 = max( nodeVar25, 4.0 );
		nodeVar28 = exp2( nodeVar25 );
		nodeVar29 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar17.x, ( - nodeVar17.y ), nodeVar17.z ), 1.0 ) ).xyz, nodeVar26 ) * vec2<f32>( ( nodeVar28 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar26 > 2.0 ) ) {

			nodeVar29.y = ( nodeVar29.y + nodeVar28 );
			nodeVar26 = ( nodeVar26 - 3.0 );
			

		}

		nodeVar29.x = ( nodeVar29.x + ( nodeVar26 * nodeVar28 ) );
		nodeVar29.x = ( nodeVar29.x + ( nodeVar27 * ( 3.0 * 16.0 ) ) );
		nodeVar29.y = ( nodeVar29.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar28 ) ) );
		nodeVar29.x = ( nodeVar29.x * object.nodeUniform13 );
		nodeVar29.y = ( nodeVar29.y * object.nodeUniform14 );
		nodeVar30 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar29, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar31 = nodeVar30.xyz;
		nodeVar23 = mix( nodeVar23, nodeVar31, nodeVar24 );
		

	}

	nodeVar32 = ( radiance + ( nodeVar23 * vec3<f32>( object.nodeUniform16 ) ) );
	radiance = nodeVar32;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar33 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform10 );
	nodeVar34 = floor( nodeVar33 );
	nodeVar35 = nodeVar34;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar36 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar37 = max( ( 4.0 - nodeVar35 ), 0.0 );
	nodeVar35 = max( nodeVar35, 4.0 );
	nodeVar38 = exp2( nodeVar35 );
	nodeVar39 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar36 ) * vec2<f32>( ( nodeVar38 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar36 > 2.0 ) ) {

		nodeVar39.y = ( nodeVar39.y + nodeVar38 );
		nodeVar36 = ( nodeVar36 - 3.0 );
		

	}

	nodeVar39.x = ( nodeVar39.x + ( nodeVar36 * nodeVar38 ) );
	nodeVar39.x = ( nodeVar39.x + ( nodeVar37 * ( 3.0 * 16.0 ) ) );
	nodeVar39.y = ( nodeVar39.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar38 ) ) );
	nodeVar39.x = ( nodeVar39.x * object.nodeUniform13 );
	nodeVar39.y = ( nodeVar39.y * object.nodeUniform14 );
	nodeVar40 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar39, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar41 = nodeVar40.xyz;
	nodeVar42 = fract( nodeVar33 );

	if ( ( nodeVar42 != 0.0 ) ) {

		nodeVar43 = ( nodeVar34 + 1.0 );
		nodeVar44 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar45 = max( ( 4.0 - nodeVar43 ), 0.0 );
		nodeVar43 = max( nodeVar43, 4.0 );
		nodeVar46 = exp2( nodeVar43 );
		nodeVar47 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar44 ) * vec2<f32>( ( nodeVar46 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar44 > 2.0 ) ) {

			nodeVar47.y = ( nodeVar47.y + nodeVar46 );
			nodeVar44 = ( nodeVar44 - 3.0 );
			

		}

		nodeVar47.x = ( nodeVar47.x + ( nodeVar44 * nodeVar46 ) );
		nodeVar47.x = ( nodeVar47.x + ( nodeVar45 * ( 3.0 * 16.0 ) ) );
		nodeVar47.y = ( nodeVar47.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar46 ) ) );
		nodeVar47.x = ( nodeVar47.x * object.nodeUniform13 );
		nodeVar47.y = ( nodeVar47.y * object.nodeUniform14 );
		nodeVar48 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar47, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar49 = nodeVar48.xyz;
		nodeVar41 = mix( nodeVar41, nodeVar49, nodeVar42 );
		

	}

	nodeVar50 = ( iblIrradiance + ( ( nodeVar41 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform16 ) ) );
	iblIrradiance = nodeVar50;
	nodeVar51 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar52 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar53 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar54 = ( SpecularF90 * dfg.y );
	nodeVar55 = ( nodeVar53 + vec3<f32>( nodeVar54 ) );
	nodeVar56 = ( nodeVar51 + nodeVar55 );
	nodeVar51 = nodeVar56;
	nodeVar57 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar58 = nodeVar57;
	nodeVar59 = ( nodeVar58 * vec3<f32>( 0.047619 ) );
	nodeVar60 = ( SpecularColor + nodeVar59 );
	nodeVar61 = ( nodeVar55 * nodeVar60 );
	nodeVar62 = ( dfg.x + dfg.y );
	nodeVar63 = ( 1.0 - nodeVar62 );
	nodeVar64 = nodeVar63;
	nodeVar65 = ( vec3<f32>( nodeVar64 ) * nodeVar60 );
	nodeVar66 = ( vec3<f32>( 1.0 ) - nodeVar65 );
	nodeVar67 = nodeVar66;
	nodeVar68 = ( nodeVar61 / nodeVar67 );
	nodeVar69 = ( nodeVar68 * vec3<f32>( nodeVar64 ) );
	nodeVar70 = ( nodeVar52 + nodeVar69 );
	nodeVar52 = nodeVar70;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar71 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar72 = ( irradiance * nodeVar71 );
	nodeVar73 = ( nodeVar51 + nodeVar52 );
	nodeVar74 = ( vec3<f32>( 1.0 ) - nodeVar73 );
	nodeVar75 = nodeVar74;
	nodeVar76 = ( nodeVar72 * nodeVar75 );
	nodeVar77 = nodeVar76;
	nodeVar78 = ( SheenRoughness * SheenRoughness );
	nodeVar79 = ( 1.0 / ( SheenRoughness + 0.1 ) );
	nodeVar80 = clamp( exp( ( ( ( ( ( -1.9362 + ( SheenRoughness * 1.0678 ) ) + ( nodeVar78 * 0.4573 ) ) - ( nodeVar79 * 0.8469 ) ) * clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( SheenRoughness * 0.5538 ) ) - ( nodeVar78 * 0.467 ) ) - ( nodeVar79 * 0.1255 ) ) ) ), 0.0, 1.0 );
	nodeVar81 = ( ( ( irradiance * Sheen ) * vec3<f32>( nodeVar80 ) ) * vec3<f32>( 0.3183098861837907 ) );
	nodeVar82 = ( sheenSpecularIndirect + nodeVar81 );
	sheenSpecularIndirect = nodeVar82;
	nodeVar83 = max( Sheen.x, Sheen.y );
	nodeVar84 = max( nodeVar83, Sheen.z );
	nodeVar85 = ( nodeVar84 * nodeVar80 );
	nodeVar86 = ( 1.0 - nodeVar85 );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( nodeVar77 * vec3<f32>( nodeVar87 ) );
	nodeVar77 = nodeVar88;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar89 = ( indirectDiffuse + nodeVar77 );
	indirectDiffuse = nodeVar89;
	nodeVar90 = ( SheenRoughness * SheenRoughness );
	nodeVar91 = ( 1.0 / ( SheenRoughness + 0.1 ) );
	nodeVar92 = clamp( exp( ( ( ( ( ( -1.9362 + ( SheenRoughness * 1.0678 ) ) + ( nodeVar90 * 0.4573 ) ) - ( nodeVar91 * 0.8469 ) ) * clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( SheenRoughness * 0.5538 ) ) - ( nodeVar90 * 0.467 ) ) - ( nodeVar91 * 0.1255 ) ) ) ), 0.0, 1.0 );
	nodeVar93 = ( ( ( iblIrradiance * Sheen ) * vec3<f32>( nodeVar92 ) ) * vec3<f32>( 0.3183098861837907 ) );
	nodeVar94 = ( sheenSpecularIndirect + nodeVar93 );
	sheenSpecularIndirect = nodeVar94;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar95 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar96 = ( SpecularF90 * dfg.y );
	nodeVar97 = ( nodeVar95 + vec3<f32>( nodeVar96 ) );
	nodeVar98 = ( singleScatteringDielectric + nodeVar97 );
	singleScatteringDielectric = nodeVar98;
	nodeVar99 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar100 = nodeVar99;
	nodeVar101 = ( nodeVar100 * vec3<f32>( 0.047619 ) );
	nodeVar102 = ( SpecularColor + nodeVar101 );
	nodeVar103 = ( nodeVar97 * nodeVar102 );
	nodeVar104 = ( dfg.x + dfg.y );
	nodeVar105 = ( 1.0 - nodeVar104 );
	nodeVar106 = nodeVar105;
	nodeVar107 = ( vec3<f32>( nodeVar106 ) * nodeVar102 );
	nodeVar108 = ( vec3<f32>( 1.0 ) - nodeVar107 );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( nodeVar103 / nodeVar109 );
	nodeVar111 = ( nodeVar110 * vec3<f32>( nodeVar106 ) );
	nodeVar112 = ( multiScatteringDielectric + nodeVar111 );
	multiScatteringDielectric = nodeVar112;
	nodeVar113 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar114 = ( SpecularF90 * dfg.y );
	nodeVar115 = ( nodeVar113 + vec3<f32>( nodeVar114 ) );
	nodeVar116 = ( singleScatteringMetallic + nodeVar115 );
	singleScatteringMetallic = nodeVar116;
	nodeVar117 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar118 * vec3<f32>( 0.047619 ) );
	nodeVar120 = ( DiffuseColor.xyz + nodeVar119 );
	nodeVar121 = ( nodeVar115 * nodeVar120 );
	nodeVar122 = ( dfg.x + dfg.y );
	nodeVar123 = ( 1.0 - nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( vec3<f32>( nodeVar124 ) * nodeVar120 );
	nodeVar126 = ( vec3<f32>( 1.0 ) - nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( nodeVar121 / nodeVar127 );
	nodeVar129 = ( nodeVar128 * vec3<f32>( nodeVar124 ) );
	nodeVar130 = ( multiScatteringMetallic + nodeVar129 );
	multiScatteringMetallic = nodeVar130;
	nodeVar131 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar132 = ( radiance * nodeVar131 );
	nodeVar133 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar134 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar135 = ( nodeVar133 * nodeVar134 );
	nodeVar136 = ( nodeVar132 + nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar139 = ( vec3<f32>( 1.0 ) - nodeVar138 );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( DiffuseContribution * nodeVar140 );
	nodeVar142 = ( nodeVar141 * nodeVar134 );
	nodeVar143 = nodeVar142;
	nodeVar144 = max( Sheen.x, Sheen.y );
	nodeVar145 = max( nodeVar144, Sheen.z );
	nodeVar146 = ( SheenRoughness * SheenRoughness );
	nodeVar147 = ( 1.0 / ( SheenRoughness + 0.1 ) );
	nodeVar148 = clamp( exp( ( ( ( ( ( -1.9362 + ( SheenRoughness * 1.0678 ) ) + ( nodeVar146 * 0.4573 ) ) - ( nodeVar147 * 0.8469 ) ) * clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( SheenRoughness * 0.5538 ) ) - ( nodeVar146 * 0.467 ) ) - ( nodeVar147 * 0.1255 ) ) ) ), 0.0, 1.0 );
	nodeVar149 = ( nodeVar145 * nodeVar148 );
	nodeVar150 = ( 1.0 - nodeVar149 );
	nodeVar151 = nodeVar150;
	nodeVar152 = ( nodeVar137 * vec3<f32>( nodeVar151 ) );
	nodeVar137 = nodeVar152;
	nodeVar153 = ( nodeVar143 * vec3<f32>( nodeVar151 ) );
	nodeVar143 = nodeVar153;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar154 = ( indirectSpecular + nodeVar137 );
	indirectSpecular = nodeVar154;
	nodeVar155 = ( indirectDiffuse + nodeVar143 );
	indirectDiffuse = nodeVar155;
	ambientOcclusion = 1.0;
	nodeVar156 = ( sheenSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	sheenSpecularIndirect = nodeVar156;
	nodeVar157 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar157;
	nodeVar158 = dot( normalView, positionViewDirection );
	nodeVar159 = ( clamp( nodeVar158, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar160 = ( Roughness * -16.0 );
	nodeVar161 = ( 1.0 - nodeVar160 );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( - nodeVar162 );
	nodeVar164 = exp2( nodeVar163 );
	nodeVar165 = pow( nodeVar159, nodeVar164 );
	nodeVar166 = ( 1.0 - nodeVar165 );
	nodeVar167 = nodeVar166;
	nodeVar168 = ( ambientOcclusion - nodeVar167 );
	nodeVar169 = ( indirectSpecular * vec3<f32>( clamp( nodeVar168, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar169;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar170 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar170;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar171 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar171;
	nodeVar172 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar172;
	nodeVar173 = ( ( outgoingLight + sheenSpecularDirect ) + sheenSpecularIndirect );
	outgoingLight = nodeVar173;
	nodeVar174 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar174;

	// result

	output.color = nodeVar174;

	return output;

}
