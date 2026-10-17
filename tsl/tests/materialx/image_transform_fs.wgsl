// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32
};
@binding( 2 ) @group( 1 )
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
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
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
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec4<f32>;

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
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( 30.0 * 0.017453292519943295 );
	nodeVar1 = cos( nodeVar0 );
	nodeVar2 = ( vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) ) - vec2<f32>( 0.5, 0.5 ) );
	nodeVar3 = sin( nodeVar0 );
	nodeVar4 = ( 30.0 * 0.017453292519943295 );
	nodeVar5 = cos( nodeVar4 );
	nodeVar6 = sin( nodeVar4 );
	nodeVar7 = textureSample( nodeUniform0, nodeUniform0_sampler, vec2<f32>( mix( ( ( vec2<f32>( ( ( nodeVar1 * ( nodeVar2 / vec2<f32>( 2.0, 1.0 ) ).x ) + ( nodeVar3 * ( nodeVar2 / vec2<f32>( 2.0, 1.0 ) ).y ) ), ( ( nodeVar1 * ( nodeVar2 / vec2<f32>( 2.0, 1.0 ) ).y ) - ( nodeVar3 * ( nodeVar2 / vec2<f32>( 2.0, 1.0 ) ).x ) ) ) - vec2<f32>( 0.0, 0.0 ) ) + vec2<f32>( 0.5, 0.5 ) ), ( ( vec2<f32>( ( ( nodeVar5 * ( nodeVar2 - vec2<f32>( 0.0, 0.0 ) ).x ) + ( nodeVar6 * ( nodeVar2 - vec2<f32>( 0.0, 0.0 ) ).y ) ), ( ( nodeVar5 * ( nodeVar2 - vec2<f32>( 0.0, 0.0 ) ).y ) - ( nodeVar6 * ( nodeVar2 - vec2<f32>( 0.0, 0.0 ) ).x ) ) ) / vec2<f32>( 2.0, 1.0 ) ) + vec2<f32>( 0.5, 0.5 ) ), step( 0.5, f32( 0 ) ) )[ 0u ], ( 1.0 - mix( ( ( vec2<f32>( ( ( nodeVar1 * ( nodeVar2 / vec2<f32>( 2.0, 1.0 ) ).x ) + ( nodeVar3 * ( nodeVar2 / vec2<f32>( 2.0, 1.0 ) ).y ) ), ( ( nodeVar1 * ( nodeVar2 / vec2<f32>( 2.0, 1.0 ) ).y ) - ( nodeVar3 * ( nodeVar2 / vec2<f32>( 2.0, 1.0 ) ).x ) ) ) - vec2<f32>( 0.0, 0.0 ) ) + vec2<f32>( 0.5, 0.5 ) ), ( ( vec2<f32>( ( ( nodeVar5 * ( nodeVar2 - vec2<f32>( 0.0, 0.0 ) ).x ) + ( nodeVar6 * ( nodeVar2 - vec2<f32>( 0.0, 0.0 ) ).y ) ), ( ( nodeVar5 * ( nodeVar2 - vec2<f32>( 0.0, 0.0 ) ).y ) - ( nodeVar6 * ( nodeVar2 - vec2<f32>( 0.0, 0.0 ) ).x ) ) ) / vec2<f32>( 2.0, 1.0 ) ) + vec2<f32>( 0.5, 0.5 ) ), step( 0.5, f32( 0 ) ) )[ 1u ] ) ) );
	DiffuseColor = vec4<f32>( nodeVar7.xyz, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar8 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.2, 0.0525 ) + max( max( nodeVar8.x, nodeVar8.y ), nodeVar8.z ) ), 1.0 );
	IOR = object.nodeUniform5;
	nodeVar9 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar9 * nodeVar9 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform6 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform6, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar10 = dot( normalView, positionViewDirection );
	nodeVar11 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( Roughness, clamp( nodeVar10, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar11;
	nodeVar12 = ( dfg.x + dfg.y );
	nodeVar13 = ( 1.0 / nodeVar12 );
	nodeVar14 = nodeVar13;
	nodeVar15 = ( nodeVar14 - 1.0 );
	nodeVar16 = ( SpecularColorBlended * vec3<f32>( nodeVar15 ) );
	nodeVar17 = ( nodeVar16 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar17;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar18 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform11 );
	nodeVar19 = floor( nodeVar18 );
	nodeVar20 = nodeVar19;
	nodeVar21 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar22 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar21.x, ( - nodeVar21.y ), nodeVar21.z ), 1.0 ) ).xyz );
	nodeVar23 = max( ( 4.0 - nodeVar20 ), 0.0 );
	nodeVar20 = max( nodeVar20, 4.0 );
	nodeVar24 = exp2( nodeVar20 );
	nodeVar25 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar21.x, ( - nodeVar21.y ), nodeVar21.z ), 1.0 ) ).xyz, nodeVar22 ) * vec2<f32>( ( nodeVar24 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar22 > 2.0 ) ) {

		nodeVar25.y = ( nodeVar25.y + nodeVar24 );
		nodeVar22 = ( nodeVar22 - 3.0 );
		

	}

	nodeVar25.x = ( nodeVar25.x + ( nodeVar22 * nodeVar24 ) );
	nodeVar25.x = ( nodeVar25.x + ( nodeVar23 * ( 3.0 * 16.0 ) ) );
	nodeVar25.y = ( nodeVar25.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar24 ) ) );
	nodeVar25.x = ( nodeVar25.x * object.nodeUniform14 );
	nodeVar25.y = ( nodeVar25.y * object.nodeUniform15 );
	nodeVar26 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar25, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar27 = nodeVar26.xyz;
	nodeVar28 = fract( nodeVar18 );

	if ( ( nodeVar28 != 0.0 ) ) {

		nodeVar29 = ( nodeVar19 + 1.0 );
		nodeVar30 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar21.x, ( - nodeVar21.y ), nodeVar21.z ), 1.0 ) ).xyz );
		nodeVar31 = max( ( 4.0 - nodeVar29 ), 0.0 );
		nodeVar29 = max( nodeVar29, 4.0 );
		nodeVar32 = exp2( nodeVar29 );
		nodeVar33 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar21.x, ( - nodeVar21.y ), nodeVar21.z ), 1.0 ) ).xyz, nodeVar30 ) * vec2<f32>( ( nodeVar32 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar30 > 2.0 ) ) {

			nodeVar33.y = ( nodeVar33.y + nodeVar32 );
			nodeVar30 = ( nodeVar30 - 3.0 );
			

		}

		nodeVar33.x = ( nodeVar33.x + ( nodeVar30 * nodeVar32 ) );
		nodeVar33.x = ( nodeVar33.x + ( nodeVar31 * ( 3.0 * 16.0 ) ) );
		nodeVar33.y = ( nodeVar33.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar32 ) ) );
		nodeVar33.x = ( nodeVar33.x * object.nodeUniform14 );
		nodeVar33.y = ( nodeVar33.y * object.nodeUniform15 );
		nodeVar34 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar33, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar35 = nodeVar34.xyz;
		nodeVar27 = mix( nodeVar27, nodeVar35, nodeVar28 );
		

	}

	nodeVar36 = ( radiance + ( nodeVar27 * vec3<f32>( object.nodeUniform17 ) ) );
	radiance = nodeVar36;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar37 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform11 );
	nodeVar38 = floor( nodeVar37 );
	nodeVar39 = nodeVar38;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar40 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar41 = max( ( 4.0 - nodeVar39 ), 0.0 );
	nodeVar39 = max( nodeVar39, 4.0 );
	nodeVar42 = exp2( nodeVar39 );
	nodeVar43 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar40 ) * vec2<f32>( ( nodeVar42 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar40 > 2.0 ) ) {

		nodeVar43.y = ( nodeVar43.y + nodeVar42 );
		nodeVar40 = ( nodeVar40 - 3.0 );
		

	}

	nodeVar43.x = ( nodeVar43.x + ( nodeVar40 * nodeVar42 ) );
	nodeVar43.x = ( nodeVar43.x + ( nodeVar41 * ( 3.0 * 16.0 ) ) );
	nodeVar43.y = ( nodeVar43.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar42 ) ) );
	nodeVar43.x = ( nodeVar43.x * object.nodeUniform14 );
	nodeVar43.y = ( nodeVar43.y * object.nodeUniform15 );
	nodeVar44 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar43, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar45 = nodeVar44.xyz;
	nodeVar46 = fract( nodeVar37 );

	if ( ( nodeVar46 != 0.0 ) ) {

		nodeVar47 = ( nodeVar38 + 1.0 );
		nodeVar48 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar49 = max( ( 4.0 - nodeVar47 ), 0.0 );
		nodeVar47 = max( nodeVar47, 4.0 );
		nodeVar50 = exp2( nodeVar47 );
		nodeVar51 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar48 ) * vec2<f32>( ( nodeVar50 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar48 > 2.0 ) ) {

			nodeVar51.y = ( nodeVar51.y + nodeVar50 );
			nodeVar48 = ( nodeVar48 - 3.0 );
			

		}

		nodeVar51.x = ( nodeVar51.x + ( nodeVar48 * nodeVar50 ) );
		nodeVar51.x = ( nodeVar51.x + ( nodeVar49 * ( 3.0 * 16.0 ) ) );
		nodeVar51.y = ( nodeVar51.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar50 ) ) );
		nodeVar51.x = ( nodeVar51.x * object.nodeUniform14 );
		nodeVar51.y = ( nodeVar51.y * object.nodeUniform15 );
		nodeVar52 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar51, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar53 = nodeVar52.xyz;
		nodeVar45 = mix( nodeVar45, nodeVar53, nodeVar46 );
		

	}

	nodeVar54 = ( iblIrradiance + ( ( nodeVar45 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform17 ) ) );
	iblIrradiance = nodeVar54;
	nodeVar55 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar56 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar57 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar58 = ( SpecularF90 * dfg.y );
	nodeVar59 = ( nodeVar57 + vec3<f32>( nodeVar58 ) );
	nodeVar60 = ( nodeVar55 + nodeVar59 );
	nodeVar55 = nodeVar60;
	nodeVar61 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar62 = nodeVar61;
	nodeVar63 = ( nodeVar62 * vec3<f32>( 0.047619 ) );
	nodeVar64 = ( SpecularColor + nodeVar63 );
	nodeVar65 = ( nodeVar59 * nodeVar64 );
	nodeVar66 = ( dfg.x + dfg.y );
	nodeVar67 = ( 1.0 - nodeVar66 );
	nodeVar68 = nodeVar67;
	nodeVar69 = ( vec3<f32>( nodeVar68 ) * nodeVar64 );
	nodeVar70 = ( vec3<f32>( 1.0 ) - nodeVar69 );
	nodeVar71 = nodeVar70;
	nodeVar72 = ( nodeVar65 / nodeVar71 );
	nodeVar73 = ( nodeVar72 * vec3<f32>( nodeVar68 ) );
	nodeVar74 = ( nodeVar56 + nodeVar73 );
	nodeVar56 = nodeVar74;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar75 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar76 = ( irradiance * nodeVar75 );
	nodeVar77 = ( nodeVar55 + nodeVar56 );
	nodeVar78 = ( vec3<f32>( 1.0 ) - nodeVar77 );
	nodeVar79 = nodeVar78;
	nodeVar80 = ( nodeVar76 * nodeVar79 );
	nodeVar81 = nodeVar80;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar82 = ( indirectDiffuse + nodeVar81 );
	indirectDiffuse = nodeVar82;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar83 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar84 = ( SpecularF90 * dfg.y );
	nodeVar85 = ( nodeVar83 + vec3<f32>( nodeVar84 ) );
	nodeVar86 = ( singleScatteringDielectric + nodeVar85 );
	singleScatteringDielectric = nodeVar86;
	nodeVar87 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar88 = nodeVar87;
	nodeVar89 = ( nodeVar88 * vec3<f32>( 0.047619 ) );
	nodeVar90 = ( SpecularColor + nodeVar89 );
	nodeVar91 = ( nodeVar85 * nodeVar90 );
	nodeVar92 = ( dfg.x + dfg.y );
	nodeVar93 = ( 1.0 - nodeVar92 );
	nodeVar94 = nodeVar93;
	nodeVar95 = ( vec3<f32>( nodeVar94 ) * nodeVar90 );
	nodeVar96 = ( vec3<f32>( 1.0 ) - nodeVar95 );
	nodeVar97 = nodeVar96;
	nodeVar98 = ( nodeVar91 / nodeVar97 );
	nodeVar99 = ( nodeVar98 * vec3<f32>( nodeVar94 ) );
	nodeVar100 = ( multiScatteringDielectric + nodeVar99 );
	multiScatteringDielectric = nodeVar100;
	nodeVar101 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar102 = ( SpecularF90 * dfg.y );
	nodeVar103 = ( nodeVar101 + vec3<f32>( nodeVar102 ) );
	nodeVar104 = ( singleScatteringMetallic + nodeVar103 );
	singleScatteringMetallic = nodeVar104;
	nodeVar105 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar106 = nodeVar105;
	nodeVar107 = ( nodeVar106 * vec3<f32>( 0.047619 ) );
	nodeVar108 = ( DiffuseColor.xyz + nodeVar107 );
	nodeVar109 = ( nodeVar103 * nodeVar108 );
	nodeVar110 = ( dfg.x + dfg.y );
	nodeVar111 = ( 1.0 - nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( vec3<f32>( nodeVar112 ) * nodeVar108 );
	nodeVar114 = ( vec3<f32>( 1.0 ) - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( nodeVar109 / nodeVar115 );
	nodeVar117 = ( nodeVar116 * vec3<f32>( nodeVar112 ) );
	nodeVar118 = ( multiScatteringMetallic + nodeVar117 );
	multiScatteringMetallic = nodeVar118;
	nodeVar119 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar120 = ( radiance * nodeVar119 );
	nodeVar121 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar122 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar123 = ( nodeVar121 * nodeVar122 );
	nodeVar124 = ( nodeVar120 + nodeVar123 );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar127 = ( vec3<f32>( 1.0 ) - nodeVar126 );
	nodeVar128 = nodeVar127;
	nodeVar129 = ( DiffuseContribution * nodeVar128 );
	nodeVar130 = ( nodeVar129 * nodeVar122 );
	nodeVar131 = nodeVar130;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar132 = ( indirectSpecular + nodeVar125 );
	indirectSpecular = nodeVar132;
	nodeVar133 = ( indirectDiffuse + nodeVar131 );
	indirectDiffuse = nodeVar133;
	ambientOcclusion = 1.0;
	nodeVar134 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar134;
	nodeVar135 = dot( normalView, positionViewDirection );
	nodeVar136 = ( clamp( nodeVar135, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar137 = ( Roughness * -16.0 );
	nodeVar138 = ( 1.0 - nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( - nodeVar139 );
	nodeVar141 = exp2( nodeVar140 );
	nodeVar142 = pow( nodeVar136, nodeVar141 );
	nodeVar143 = ( 1.0 - nodeVar142 );
	nodeVar144 = nodeVar143;
	nodeVar145 = ( ambientOcclusion - nodeVar144 );
	nodeVar146 = ( indirectSpecular * vec3<f32>( clamp( nodeVar145, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar146;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar147 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar147;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar148 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar148;
	nodeVar149 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar149;
	nodeVar150 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar150;

	// result

	output.color = nodeVar150;

	return output;

}
