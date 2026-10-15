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
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
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
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> sheenSpecularDirect : vec3<f32>;
var<private> sheenSpecularIndirect : vec3<f32>;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : vec3<f32>;
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
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar152 : vec3<f32>;
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
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec4<f32>;

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

	DiffuseColor = vec4<f32>( vec3<f32>( 0.2, 0.4, 0.8 ), 1.0 );
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
	Sheen = ( vec3<f32>( 0.8 ) * vec3<f32>( 1.0, 0.9, 0.7 ) );
	SheenRoughness = 0.3;
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	sheenSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	sheenSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = dot( normalView, positionViewDirection );
	nodeVar3 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar2, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	nodeVar4 = ( dfg.x + dfg.y );
	nodeVar5 = ( 1.0 / nodeVar4 );
	nodeVar6 = nodeVar5;
	nodeVar7 = ( nodeVar6 - 1.0 );
	nodeVar8 = ( SpecularColorBlended * vec3<f32>( nodeVar7 ) );
	nodeVar9 = ( nodeVar8 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar9;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar10 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform10 );
	nodeVar11 = floor( nodeVar10 );
	nodeVar12 = nodeVar11;
	nodeVar13 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar14 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar13.x, ( - nodeVar13.y ), nodeVar13.z ), 1.0 ) ).xyz );
	nodeVar15 = max( ( 4.0 - nodeVar12 ), 0.0 );
	nodeVar12 = max( nodeVar12, 4.0 );
	nodeVar16 = exp2( nodeVar12 );
	nodeVar17 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar13.x, ( - nodeVar13.y ), nodeVar13.z ), 1.0 ) ).xyz, nodeVar14 ) * vec2<f32>( ( nodeVar16 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar14 > 2.0 ) ) {

		nodeVar17.y = ( nodeVar17.y + nodeVar16 );
		nodeVar14 = ( nodeVar14 - 3.0 );
		

	}

	nodeVar17.x = ( nodeVar17.x + ( nodeVar14 * nodeVar16 ) );
	nodeVar17.x = ( nodeVar17.x + ( nodeVar15 * ( 3.0 * 16.0 ) ) );
	nodeVar17.y = ( nodeVar17.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar16 ) ) );
	nodeVar17.x = ( nodeVar17.x * object.nodeUniform13 );
	nodeVar17.y = ( nodeVar17.y * object.nodeUniform14 );
	nodeVar18 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar17, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar19 = nodeVar18.xyz;
	nodeVar20 = fract( nodeVar10 );

	if ( ( nodeVar20 != 0.0 ) ) {

		nodeVar21 = ( nodeVar11 + 1.0 );
		nodeVar22 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar13.x, ( - nodeVar13.y ), nodeVar13.z ), 1.0 ) ).xyz );
		nodeVar23 = max( ( 4.0 - nodeVar21 ), 0.0 );
		nodeVar21 = max( nodeVar21, 4.0 );
		nodeVar24 = exp2( nodeVar21 );
		nodeVar25 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar13.x, ( - nodeVar13.y ), nodeVar13.z ), 1.0 ) ).xyz, nodeVar22 ) * vec2<f32>( ( nodeVar24 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar22 > 2.0 ) ) {

			nodeVar25.y = ( nodeVar25.y + nodeVar24 );
			nodeVar22 = ( nodeVar22 - 3.0 );
			

		}

		nodeVar25.x = ( nodeVar25.x + ( nodeVar22 * nodeVar24 ) );
		nodeVar25.x = ( nodeVar25.x + ( nodeVar23 * ( 3.0 * 16.0 ) ) );
		nodeVar25.y = ( nodeVar25.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar24 ) ) );
		nodeVar25.x = ( nodeVar25.x * object.nodeUniform13 );
		nodeVar25.y = ( nodeVar25.y * object.nodeUniform14 );
		nodeVar26 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar25, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar27 = nodeVar26.xyz;
		nodeVar19 = mix( nodeVar19, nodeVar27, nodeVar20 );
		

	}

	nodeVar28 = ( radiance + ( nodeVar19 * vec3<f32>( object.nodeUniform16 ) ) );
	radiance = nodeVar28;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar29 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform10 );
	nodeVar30 = floor( nodeVar29 );
	nodeVar31 = nodeVar30;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar32 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar33 = max( ( 4.0 - nodeVar31 ), 0.0 );
	nodeVar31 = max( nodeVar31, 4.0 );
	nodeVar34 = exp2( nodeVar31 );
	nodeVar35 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar32 ) * vec2<f32>( ( nodeVar34 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar32 > 2.0 ) ) {

		nodeVar35.y = ( nodeVar35.y + nodeVar34 );
		nodeVar32 = ( nodeVar32 - 3.0 );
		

	}

	nodeVar35.x = ( nodeVar35.x + ( nodeVar32 * nodeVar34 ) );
	nodeVar35.x = ( nodeVar35.x + ( nodeVar33 * ( 3.0 * 16.0 ) ) );
	nodeVar35.y = ( nodeVar35.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar34 ) ) );
	nodeVar35.x = ( nodeVar35.x * object.nodeUniform13 );
	nodeVar35.y = ( nodeVar35.y * object.nodeUniform14 );
	nodeVar36 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar35, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar37 = nodeVar36.xyz;
	nodeVar38 = fract( nodeVar29 );

	if ( ( nodeVar38 != 0.0 ) ) {

		nodeVar39 = ( nodeVar30 + 1.0 );
		nodeVar40 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar41 = max( ( 4.0 - nodeVar39 ), 0.0 );
		nodeVar39 = max( nodeVar39, 4.0 );
		nodeVar42 = exp2( nodeVar39 );
		nodeVar43 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar40 ) * vec2<f32>( ( nodeVar42 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar40 > 2.0 ) ) {

			nodeVar43.y = ( nodeVar43.y + nodeVar42 );
			nodeVar40 = ( nodeVar40 - 3.0 );
			

		}

		nodeVar43.x = ( nodeVar43.x + ( nodeVar40 * nodeVar42 ) );
		nodeVar43.x = ( nodeVar43.x + ( nodeVar41 * ( 3.0 * 16.0 ) ) );
		nodeVar43.y = ( nodeVar43.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar42 ) ) );
		nodeVar43.x = ( nodeVar43.x * object.nodeUniform13 );
		nodeVar43.y = ( nodeVar43.y * object.nodeUniform14 );
		nodeVar44 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar43, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar45 = nodeVar44.xyz;
		nodeVar37 = mix( nodeVar37, nodeVar45, nodeVar38 );
		

	}

	nodeVar46 = ( iblIrradiance + ( ( nodeVar37 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform16 ) ) );
	iblIrradiance = nodeVar46;
	nodeVar47 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar48 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar49 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar50 = ( SpecularF90 * dfg.y );
	nodeVar51 = ( nodeVar49 + vec3<f32>( nodeVar50 ) );
	nodeVar52 = ( nodeVar47 + nodeVar51 );
	nodeVar47 = nodeVar52;
	nodeVar53 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar54 = nodeVar53;
	nodeVar55 = ( nodeVar54 * vec3<f32>( 0.047619 ) );
	nodeVar56 = ( SpecularColor + nodeVar55 );
	nodeVar57 = ( nodeVar51 * nodeVar56 );
	nodeVar58 = ( dfg.x + dfg.y );
	nodeVar59 = ( 1.0 - nodeVar58 );
	nodeVar60 = nodeVar59;
	nodeVar61 = ( vec3<f32>( nodeVar60 ) * nodeVar56 );
	nodeVar62 = ( vec3<f32>( 1.0 ) - nodeVar61 );
	nodeVar63 = nodeVar62;
	nodeVar64 = ( nodeVar57 / nodeVar63 );
	nodeVar65 = ( nodeVar64 * vec3<f32>( nodeVar60 ) );
	nodeVar66 = ( nodeVar48 + nodeVar65 );
	nodeVar48 = nodeVar66;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar67 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar68 = ( irradiance * nodeVar67 );
	nodeVar69 = ( nodeVar47 + nodeVar48 );
	nodeVar70 = ( vec3<f32>( 1.0 ) - nodeVar69 );
	nodeVar71 = nodeVar70;
	nodeVar72 = ( nodeVar68 * nodeVar71 );
	nodeVar73 = nodeVar72;
	nodeVar74 = ( SheenRoughness * SheenRoughness );
	nodeVar75 = ( 1.0 / ( SheenRoughness + 0.1 ) );
	nodeVar76 = clamp( exp( ( ( ( ( ( -1.9362 + ( SheenRoughness * 1.0678 ) ) + ( nodeVar74 * 0.4573 ) ) - ( nodeVar75 * 0.8469 ) ) * clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( SheenRoughness * 0.5538 ) ) - ( nodeVar74 * 0.467 ) ) - ( nodeVar75 * 0.1255 ) ) ) ), 0.0, 1.0 );
	nodeVar77 = ( ( ( irradiance * Sheen ) * vec3<f32>( nodeVar76 ) ) * vec3<f32>( 0.3183098861837907 ) );
	nodeVar78 = ( sheenSpecularIndirect + nodeVar77 );
	sheenSpecularIndirect = nodeVar78;
	nodeVar79 = max( Sheen.x, Sheen.y );
	nodeVar80 = max( nodeVar79, Sheen.z );
	nodeVar81 = ( nodeVar80 * nodeVar76 );
	nodeVar82 = ( 1.0 - nodeVar81 );
	nodeVar83 = nodeVar82;
	nodeVar84 = ( nodeVar73 * vec3<f32>( nodeVar83 ) );
	nodeVar73 = nodeVar84;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar85 = ( indirectDiffuse + nodeVar73 );
	indirectDiffuse = nodeVar85;
	nodeVar86 = ( SheenRoughness * SheenRoughness );
	nodeVar87 = ( 1.0 / ( SheenRoughness + 0.1 ) );
	nodeVar88 = clamp( exp( ( ( ( ( ( -1.9362 + ( SheenRoughness * 1.0678 ) ) + ( nodeVar86 * 0.4573 ) ) - ( nodeVar87 * 0.8469 ) ) * clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( SheenRoughness * 0.5538 ) ) - ( nodeVar86 * 0.467 ) ) - ( nodeVar87 * 0.1255 ) ) ) ), 0.0, 1.0 );
	nodeVar89 = ( ( ( iblIrradiance * Sheen ) * vec3<f32>( nodeVar88 ) ) * vec3<f32>( 0.3183098861837907 ) );
	nodeVar90 = ( sheenSpecularIndirect + nodeVar89 );
	sheenSpecularIndirect = nodeVar90;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar91 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar92 = ( SpecularF90 * dfg.y );
	nodeVar93 = ( nodeVar91 + vec3<f32>( nodeVar92 ) );
	nodeVar94 = ( singleScatteringDielectric + nodeVar93 );
	singleScatteringDielectric = nodeVar94;
	nodeVar95 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar96 = nodeVar95;
	nodeVar97 = ( nodeVar96 * vec3<f32>( 0.047619 ) );
	nodeVar98 = ( SpecularColor + nodeVar97 );
	nodeVar99 = ( nodeVar93 * nodeVar98 );
	nodeVar100 = ( dfg.x + dfg.y );
	nodeVar101 = ( 1.0 - nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( vec3<f32>( nodeVar102 ) * nodeVar98 );
	nodeVar104 = ( vec3<f32>( 1.0 ) - nodeVar103 );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( nodeVar99 / nodeVar105 );
	nodeVar107 = ( nodeVar106 * vec3<f32>( nodeVar102 ) );
	nodeVar108 = ( multiScatteringDielectric + nodeVar107 );
	multiScatteringDielectric = nodeVar108;
	nodeVar109 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar110 = ( SpecularF90 * dfg.y );
	nodeVar111 = ( nodeVar109 + vec3<f32>( nodeVar110 ) );
	nodeVar112 = ( singleScatteringMetallic + nodeVar111 );
	singleScatteringMetallic = nodeVar112;
	nodeVar113 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar114 * vec3<f32>( 0.047619 ) );
	nodeVar116 = ( DiffuseColor.xyz + nodeVar115 );
	nodeVar117 = ( nodeVar111 * nodeVar116 );
	nodeVar118 = ( dfg.x + dfg.y );
	nodeVar119 = ( 1.0 - nodeVar118 );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( vec3<f32>( nodeVar120 ) * nodeVar116 );
	nodeVar122 = ( vec3<f32>( 1.0 ) - nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( nodeVar117 / nodeVar123 );
	nodeVar125 = ( nodeVar124 * vec3<f32>( nodeVar120 ) );
	nodeVar126 = ( multiScatteringMetallic + nodeVar125 );
	multiScatteringMetallic = nodeVar126;
	nodeVar127 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar128 = ( radiance * nodeVar127 );
	nodeVar129 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar130 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar131 = ( nodeVar129 * nodeVar130 );
	nodeVar132 = ( nodeVar128 + nodeVar131 );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar135 = ( vec3<f32>( 1.0 ) - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( DiffuseContribution * nodeVar136 );
	nodeVar138 = ( nodeVar137 * nodeVar130 );
	nodeVar139 = nodeVar138;
	nodeVar140 = max( Sheen.x, Sheen.y );
	nodeVar141 = max( nodeVar140, Sheen.z );
	nodeVar142 = ( SheenRoughness * SheenRoughness );
	nodeVar143 = ( 1.0 / ( SheenRoughness + 0.1 ) );
	nodeVar144 = clamp( exp( ( ( ( ( ( -1.9362 + ( SheenRoughness * 1.0678 ) ) + ( nodeVar142 * 0.4573 ) ) - ( nodeVar143 * 0.8469 ) ) * clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( SheenRoughness * 0.5538 ) ) - ( nodeVar142 * 0.467 ) ) - ( nodeVar143 * 0.1255 ) ) ) ), 0.0, 1.0 );
	nodeVar145 = ( nodeVar141 * nodeVar144 );
	nodeVar146 = ( 1.0 - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( nodeVar133 * vec3<f32>( nodeVar147 ) );
	nodeVar133 = nodeVar148;
	nodeVar149 = ( nodeVar139 * vec3<f32>( nodeVar147 ) );
	nodeVar139 = nodeVar149;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar150 = ( indirectSpecular + nodeVar133 );
	indirectSpecular = nodeVar150;
	nodeVar151 = ( indirectDiffuse + nodeVar139 );
	indirectDiffuse = nodeVar151;
	ambientOcclusion = 1.0;
	nodeVar152 = ( sheenSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	sheenSpecularIndirect = nodeVar152;
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
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar166 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar166;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar167 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar167;
	nodeVar168 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar168;
	nodeVar169 = ( ( outgoingLight + sheenSpecularDirect ) + sheenSpecularIndirect );
	outgoingLight = nodeVar169;
	nodeVar170 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar170;

	// result

	output.color = nodeVar170;

	return output;

}
