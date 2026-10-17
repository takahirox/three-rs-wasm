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
@binding( 3 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform19 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat4x4<f32>,
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
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_tangentWorld : vec3<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> NORMAL_bitangentWorld : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_normalWorld : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : vec4<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec2<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar99 : vec3<f32>;
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
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec4<f32>;

// codes
fn mx_srgb_texture_to_lin_rec709 ( color : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<bool>;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : vec3<f32>;

	nodeVar0 = color;
	nodeVar1 = ( nodeVar0 > vec3<f32>( 0.04045, 0.04045, 0.04045 ) );
	nodeVar2 = ( nodeVar0 / vec3<f32>( 12.92 ) );
	nodeVar3 = pow( ( max( ( nodeVar0 + vec3<f32>( 0.055, 0.055, 0.055 ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) / vec3<f32>( 1.055 ) ), vec3<f32>( 2.4, 2.4, 2.4 ) );

	return mix( nodeVar2, nodeVar3, vec3<f32>( nodeVar1 ) );

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
	@location( 1 ) v_tangentWorld : vec3<f32>,
	@location( 2 ) v_bitangentWorld : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) NORMAL_v_tangentWorld : vec3<f32>,
	@location( 5 ) NORMAL_v_bitangentWorld : vec3<f32>,
	@location( 6 ) nodeVarying12 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.8, 0.8, 0.8 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.2, 0.0525 ) + max( max( nodeVar1.x, nodeVar1.y ), nodeVar1.z ) ), 1.0 );
	IOR = object.nodeUniform4;
	nodeVar2 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar2 * nodeVar2 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform5 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform5, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_tangentWorld = normalize( NORMAL_v_tangentWorld );
	nodeVar4 = vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) );
	nodeVar5 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( nodeVar4[ 0u ], ( 1.0 - nodeVar4[ 1u ] ) ) );
	nodeVar6 = vec3<f32>( mx_srgb_texture_to_lin_rec709( nodeVar5.xyz )[ 0u ], mx_srgb_texture_to_lin_rec709( nodeVar5.xyz )[ 1u ], mx_srgb_texture_to_lin_rec709( nodeVar5.xyz )[ 2u ] );
	nodeVar7 = mix( ( ( nodeVar6 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 0.0, 0.0, 1.0 ), f32( ( dot( nodeVar6, nodeVar6 ) == 0.0 ) ) );
	NORMAL_bitangentWorld = normalize( NORMAL_v_bitangentWorld );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_normalWorld = normalize( ( vec4<f32>( NORMAL_normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	normalView = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform10 * normalize( ( ( ( NORMAL_tangentWorld * vec3<f32>( ( nodeVar7[ 0u ] * vec2<f32>( 1.5 )[ 0u ] ) ) ) + ( NORMAL_bitangentWorld * vec3<f32>( ( nodeVar7[ 1u ] * vec2<f32>( 1.5 )[ 1u ] ) ) ) ) + ( NORMAL_normalWorld * vec3<f32>( nodeVar7[ 2u ] ) ) ) ) ), 0.0 ) ).xyz );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar8 = dot( normalView, positionViewDirection );
	nodeVar9 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar8, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar9;
	nodeVar10 = ( dfg.x + dfg.y );
	nodeVar11 = ( 1.0 / nodeVar10 );
	nodeVar12 = nodeVar11;
	nodeVar13 = ( nodeVar12 - 1.0 );
	nodeVar14 = ( SpecularColorBlended * vec3<f32>( nodeVar13 ) );
	nodeVar15 = ( nodeVar14 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar15;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar16 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform14 );
	nodeVar17 = floor( nodeVar16 );
	nodeVar18 = nodeVar17;
	nodeVar19 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar20 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz );
	nodeVar21 = max( ( 4.0 - nodeVar18 ), 0.0 );
	nodeVar18 = max( nodeVar18, 4.0 );
	nodeVar22 = exp2( nodeVar18 );
	nodeVar23 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz, nodeVar20 ) * vec2<f32>( ( nodeVar22 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar20 > 2.0 ) ) {

		nodeVar23.y = ( nodeVar23.y + nodeVar22 );
		nodeVar20 = ( nodeVar20 - 3.0 );
		

	}

	nodeVar23.x = ( nodeVar23.x + ( nodeVar20 * nodeVar22 ) );
	nodeVar23.x = ( nodeVar23.x + ( nodeVar21 * ( 3.0 * 16.0 ) ) );
	nodeVar23.y = ( nodeVar23.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar22 ) ) );
	nodeVar23.x = ( nodeVar23.x * object.nodeUniform17 );
	nodeVar23.y = ( nodeVar23.y * object.nodeUniform18 );
	nodeVar24 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar23, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar25 = nodeVar24.xyz;
	nodeVar26 = fract( nodeVar16 );

	if ( ( nodeVar26 != 0.0 ) ) {

		nodeVar27 = ( nodeVar17 + 1.0 );
		nodeVar28 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz );
		nodeVar29 = max( ( 4.0 - nodeVar27 ), 0.0 );
		nodeVar27 = max( nodeVar27, 4.0 );
		nodeVar30 = exp2( nodeVar27 );
		nodeVar31 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz, nodeVar28 ) * vec2<f32>( ( nodeVar30 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar28 > 2.0 ) ) {

			nodeVar31.y = ( nodeVar31.y + nodeVar30 );
			nodeVar28 = ( nodeVar28 - 3.0 );
			

		}

		nodeVar31.x = ( nodeVar31.x + ( nodeVar28 * nodeVar30 ) );
		nodeVar31.x = ( nodeVar31.x + ( nodeVar29 * ( 3.0 * 16.0 ) ) );
		nodeVar31.y = ( nodeVar31.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar30 ) ) );
		nodeVar31.x = ( nodeVar31.x * object.nodeUniform17 );
		nodeVar31.y = ( nodeVar31.y * object.nodeUniform18 );
		nodeVar32 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar31, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar33 = nodeVar32.xyz;
		nodeVar25 = mix( nodeVar25, nodeVar33, nodeVar26 );
		

	}

	nodeVar34 = ( radiance + ( nodeVar25 * vec3<f32>( object.nodeUniform20 ) ) );
	radiance = nodeVar34;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar35 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform14 );
	nodeVar36 = floor( nodeVar35 );
	nodeVar37 = nodeVar36;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar38 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar39 = max( ( 4.0 - nodeVar37 ), 0.0 );
	nodeVar37 = max( nodeVar37, 4.0 );
	nodeVar40 = exp2( nodeVar37 );
	nodeVar41 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar38 ) * vec2<f32>( ( nodeVar40 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar38 > 2.0 ) ) {

		nodeVar41.y = ( nodeVar41.y + nodeVar40 );
		nodeVar38 = ( nodeVar38 - 3.0 );
		

	}

	nodeVar41.x = ( nodeVar41.x + ( nodeVar38 * nodeVar40 ) );
	nodeVar41.x = ( nodeVar41.x + ( nodeVar39 * ( 3.0 * 16.0 ) ) );
	nodeVar41.y = ( nodeVar41.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar40 ) ) );
	nodeVar41.x = ( nodeVar41.x * object.nodeUniform17 );
	nodeVar41.y = ( nodeVar41.y * object.nodeUniform18 );
	nodeVar42 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar41, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar43 = nodeVar42.xyz;
	nodeVar44 = fract( nodeVar35 );

	if ( ( nodeVar44 != 0.0 ) ) {

		nodeVar45 = ( nodeVar36 + 1.0 );
		nodeVar46 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar47 = max( ( 4.0 - nodeVar45 ), 0.0 );
		nodeVar45 = max( nodeVar45, 4.0 );
		nodeVar48 = exp2( nodeVar45 );
		nodeVar49 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar46 ) * vec2<f32>( ( nodeVar48 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar46 > 2.0 ) ) {

			nodeVar49.y = ( nodeVar49.y + nodeVar48 );
			nodeVar46 = ( nodeVar46 - 3.0 );
			

		}

		nodeVar49.x = ( nodeVar49.x + ( nodeVar46 * nodeVar48 ) );
		nodeVar49.x = ( nodeVar49.x + ( nodeVar47 * ( 3.0 * 16.0 ) ) );
		nodeVar49.y = ( nodeVar49.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar48 ) ) );
		nodeVar49.x = ( nodeVar49.x * object.nodeUniform17 );
		nodeVar49.y = ( nodeVar49.y * object.nodeUniform18 );
		nodeVar50 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar49, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar51 = nodeVar50.xyz;
		nodeVar43 = mix( nodeVar43, nodeVar51, nodeVar44 );
		

	}

	nodeVar52 = ( iblIrradiance + ( ( nodeVar43 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform20 ) ) );
	iblIrradiance = nodeVar52;
	nodeVar53 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar54 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar56 = ( SpecularF90 * dfg.y );
	nodeVar57 = ( nodeVar55 + vec3<f32>( nodeVar56 ) );
	nodeVar58 = ( nodeVar53 + nodeVar57 );
	nodeVar53 = nodeVar58;
	nodeVar59 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar60 = nodeVar59;
	nodeVar61 = ( nodeVar60 * vec3<f32>( 0.047619 ) );
	nodeVar62 = ( SpecularColor + nodeVar61 );
	nodeVar63 = ( nodeVar57 * nodeVar62 );
	nodeVar64 = ( dfg.x + dfg.y );
	nodeVar65 = ( 1.0 - nodeVar64 );
	nodeVar66 = nodeVar65;
	nodeVar67 = ( vec3<f32>( nodeVar66 ) * nodeVar62 );
	nodeVar68 = ( vec3<f32>( 1.0 ) - nodeVar67 );
	nodeVar69 = nodeVar68;
	nodeVar70 = ( nodeVar63 / nodeVar69 );
	nodeVar71 = ( nodeVar70 * vec3<f32>( nodeVar66 ) );
	nodeVar72 = ( nodeVar54 + nodeVar71 );
	nodeVar54 = nodeVar72;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar73 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar74 = ( irradiance * nodeVar73 );
	nodeVar75 = ( nodeVar53 + nodeVar54 );
	nodeVar76 = ( vec3<f32>( 1.0 ) - nodeVar75 );
	nodeVar77 = nodeVar76;
	nodeVar78 = ( nodeVar74 * nodeVar77 );
	nodeVar79 = nodeVar78;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar80 = ( indirectDiffuse + nodeVar79 );
	indirectDiffuse = nodeVar80;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar81 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar82 = ( SpecularF90 * dfg.y );
	nodeVar83 = ( nodeVar81 + vec3<f32>( nodeVar82 ) );
	nodeVar84 = ( singleScatteringDielectric + nodeVar83 );
	singleScatteringDielectric = nodeVar84;
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
	nodeVar98 = ( multiScatteringDielectric + nodeVar97 );
	multiScatteringDielectric = nodeVar98;
	nodeVar99 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar100 = ( SpecularF90 * dfg.y );
	nodeVar101 = ( nodeVar99 + vec3<f32>( nodeVar100 ) );
	nodeVar102 = ( singleScatteringMetallic + nodeVar101 );
	singleScatteringMetallic = nodeVar102;
	nodeVar103 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar104 = nodeVar103;
	nodeVar105 = ( nodeVar104 * vec3<f32>( 0.047619 ) );
	nodeVar106 = ( DiffuseColor.xyz + nodeVar105 );
	nodeVar107 = ( nodeVar101 * nodeVar106 );
	nodeVar108 = ( dfg.x + dfg.y );
	nodeVar109 = ( 1.0 - nodeVar108 );
	nodeVar110 = nodeVar109;
	nodeVar111 = ( vec3<f32>( nodeVar110 ) * nodeVar106 );
	nodeVar112 = ( vec3<f32>( 1.0 ) - nodeVar111 );
	nodeVar113 = nodeVar112;
	nodeVar114 = ( nodeVar107 / nodeVar113 );
	nodeVar115 = ( nodeVar114 * vec3<f32>( nodeVar110 ) );
	nodeVar116 = ( multiScatteringMetallic + nodeVar115 );
	multiScatteringMetallic = nodeVar116;
	nodeVar117 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar118 = ( radiance * nodeVar117 );
	nodeVar119 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar120 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar121 = ( nodeVar119 * nodeVar120 );
	nodeVar122 = ( nodeVar118 + nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar125 = ( vec3<f32>( 1.0 ) - nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( DiffuseContribution * nodeVar126 );
	nodeVar128 = ( nodeVar127 * nodeVar120 );
	nodeVar129 = nodeVar128;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar130 = ( indirectSpecular + nodeVar123 );
	indirectSpecular = nodeVar130;
	nodeVar131 = ( indirectDiffuse + nodeVar129 );
	indirectDiffuse = nodeVar131;
	ambientOcclusion = 1.0;
	nodeVar132 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar132;
	nodeVar133 = dot( normalView, positionViewDirection );
	nodeVar134 = ( clamp( nodeVar133, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar135 = ( Roughness * -16.0 );
	nodeVar136 = ( 1.0 - nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( - nodeVar137 );
	nodeVar139 = exp2( nodeVar138 );
	nodeVar140 = pow( nodeVar134, nodeVar139 );
	nodeVar141 = ( 1.0 - nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( ambientOcclusion - nodeVar142 );
	nodeVar144 = ( indirectSpecular * vec3<f32>( clamp( nodeVar143, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar144;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar145 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar145;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar146 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar146;
	nodeVar147 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar147;
	nodeVar148 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar148;

	// result

	output.color = nodeVar148;

	return output;

}
