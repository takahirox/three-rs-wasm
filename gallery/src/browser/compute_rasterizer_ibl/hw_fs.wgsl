// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 12 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 13 ) @group( 1 ) var nodeUniform21_sampler : sampler;
@binding( 14 ) @group( 1 ) var nodeUniform21 : texture_2d<f32>;

struct objectStruct {
	nodeUniform7 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform22 : f32
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
var<private> nodeVar11 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar12 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar13 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec3<f32>;
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
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec2<f32>;
var<private> nodeVar55 : vec4<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec2<f32>;
var<private> nodeVar63 : vec4<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar66 : f32;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec4<f32>;

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
fn main( @location( 0 ) v_positionViewDirection : vec3<f32>,
	@location( 1 ) @interpolate(flat, either) vInstId : u32,
	@location( 2 ) @interpolate(flat, either) vMegaTriIdx : u32,
	@location( 3 ) vUv : vec2<f32>,
	@location( 4 ) vNormal : vec3<f32>,
	@location( 5 ) vTangent : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar11 = textureSample( nodeUniform6, nodeUniform6_sampler, vUv );
	DiffuseColor = nodeVar11;
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	DiffuseColor.w = 1.0;
	nodeVar12 = textureSample( nodeUniform8, nodeUniform8_sampler, vUv );
	AmbientOcclusion = nodeVar12.x;
	nodeVar13 = textureSample( nodeUniform9, nodeUniform9_sampler, vUv );
	Metalness = nodeVar13.z;
	nodeVar14 = normalize( vNormal );
	nodeVar15 = dpdx( nodeVar14 );
	nodeVar16 = - dpdy( nodeVar14 );
	Roughness = min( ( max( sqrt( ( ( nodeVar13.y * nodeVar13.y ) + min( ( ( dot( nodeVar15, nodeVar15 ) + dot( nodeVar16, nodeVar16 ) ) * 2.0 ), 0.2 ) ) ), 0.0525 ) + 0.0 ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeVar13.z ) ) );
	nodeVar17 = textureSample( nodeUniform10, nodeUniform10_sampler, vUv );
	EmissiveColor = nodeVar17.xyz;
	nodeVar18 = normalize( vTangent );
	nodeVar19 = textureSample( nodeUniform13, nodeUniform13_sampler, vUv );
	nodeVar20 = ( ( nodeVar19.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) );
	normalView = normalize( ( render.cameraViewMatrix * vec4<f32>( normalize( ( ( ( nodeVar18 * vec3<f32>( nodeVar20.x ) ) + ( cross( nodeVar14, nodeVar18 ) * vec3<f32>( nodeVar20.y ) ) ) + ( nodeVar14 * vec3<f32>( nodeVar20.z ) ) ) ), 0.0 ) ).xyz );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar21 = dot( normalView, positionViewDirection );
	nodeVar22 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( nodeVar21, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar22;
	nodeVar23 = ( dfg.x + dfg.y );
	nodeVar24 = ( 1.0 / nodeVar23 );
	nodeVar25 = nodeVar24;
	nodeVar26 = ( nodeVar25 - 1.0 );
	nodeVar27 = ( SpecularColorBlended * vec3<f32>( nodeVar26 ) );
	nodeVar28 = ( nodeVar27 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar28;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar29 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform16 );
	nodeVar30 = floor( nodeVar29 );
	nodeVar31 = nodeVar30;
	nodeVar32 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar33 = getFace( ( object.nodeUniform17 * vec4<f32>( vec3<f32>( nodeVar32.x, ( - nodeVar32.y ), nodeVar32.z ), 1.0 ) ).xyz );
	nodeVar34 = max( ( 4.0 - nodeVar31 ), 0.0 );
	nodeVar31 = max( nodeVar31, 4.0 );
	nodeVar35 = exp2( nodeVar31 );
	nodeVar36 = ( ( getUV( ( object.nodeUniform17 * vec4<f32>( vec3<f32>( nodeVar32.x, ( - nodeVar32.y ), nodeVar32.z ), 1.0 ) ).xyz, nodeVar33 ) * vec2<f32>( ( nodeVar35 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar33 > 2.0 ) ) {

		nodeVar36.y = ( nodeVar36.y + nodeVar35 );
		nodeVar33 = ( nodeVar33 - 3.0 );
		

	}

	nodeVar36.x = ( nodeVar36.x + ( nodeVar33 * nodeVar35 ) );
	nodeVar36.x = ( nodeVar36.x + ( nodeVar34 * ( 3.0 * 16.0 ) ) );
	nodeVar36.y = ( nodeVar36.y + ( 4.0 * ( exp2( object.nodeUniform16 ) - nodeVar35 ) ) );
	nodeVar36.x = ( nodeVar36.x * object.nodeUniform19 );
	nodeVar36.y = ( nodeVar36.y * object.nodeUniform20 );
	nodeVar37 = textureSampleGrad( nodeUniform21, nodeUniform21_sampler, nodeVar36, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar38 = nodeVar37.xyz;
	nodeVar39 = fract( nodeVar29 );

	if ( ( nodeVar39 != 0.0 ) ) {

		nodeVar40 = ( nodeVar30 + 1.0 );
		nodeVar41 = getFace( ( object.nodeUniform17 * vec4<f32>( vec3<f32>( nodeVar32.x, ( - nodeVar32.y ), nodeVar32.z ), 1.0 ) ).xyz );
		nodeVar42 = max( ( 4.0 - nodeVar40 ), 0.0 );
		nodeVar40 = max( nodeVar40, 4.0 );
		nodeVar43 = exp2( nodeVar40 );
		nodeVar44 = ( ( getUV( ( object.nodeUniform17 * vec4<f32>( vec3<f32>( nodeVar32.x, ( - nodeVar32.y ), nodeVar32.z ), 1.0 ) ).xyz, nodeVar41 ) * vec2<f32>( ( nodeVar43 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar41 > 2.0 ) ) {

			nodeVar44.y = ( nodeVar44.y + nodeVar43 );
			nodeVar41 = ( nodeVar41 - 3.0 );
			

		}

		nodeVar44.x = ( nodeVar44.x + ( nodeVar41 * nodeVar43 ) );
		nodeVar44.x = ( nodeVar44.x + ( nodeVar42 * ( 3.0 * 16.0 ) ) );
		nodeVar44.y = ( nodeVar44.y + ( 4.0 * ( exp2( object.nodeUniform16 ) - nodeVar43 ) ) );
		nodeVar44.x = ( nodeVar44.x * object.nodeUniform19 );
		nodeVar44.y = ( nodeVar44.y * object.nodeUniform20 );
		nodeVar45 = textureSampleGrad( nodeUniform21, nodeUniform21_sampler, nodeVar44, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar46 = nodeVar45.xyz;
		nodeVar38 = mix( nodeVar38, nodeVar46, nodeVar39 );
		

	}

	nodeVar47 = ( radiance + ( nodeVar38 * vec3<f32>( object.nodeUniform22 ) ) );
	radiance = nodeVar47;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar48 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform16 );
	nodeVar49 = floor( nodeVar48 );
	nodeVar50 = nodeVar49;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar51 = getFace( ( object.nodeUniform17 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar52 = max( ( 4.0 - nodeVar50 ), 0.0 );
	nodeVar50 = max( nodeVar50, 4.0 );
	nodeVar53 = exp2( nodeVar50 );
	nodeVar54 = ( ( getUV( ( object.nodeUniform17 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar51 ) * vec2<f32>( ( nodeVar53 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar51 > 2.0 ) ) {

		nodeVar54.y = ( nodeVar54.y + nodeVar53 );
		nodeVar51 = ( nodeVar51 - 3.0 );
		

	}

	nodeVar54.x = ( nodeVar54.x + ( nodeVar51 * nodeVar53 ) );
	nodeVar54.x = ( nodeVar54.x + ( nodeVar52 * ( 3.0 * 16.0 ) ) );
	nodeVar54.y = ( nodeVar54.y + ( 4.0 * ( exp2( object.nodeUniform16 ) - nodeVar53 ) ) );
	nodeVar54.x = ( nodeVar54.x * object.nodeUniform19 );
	nodeVar54.y = ( nodeVar54.y * object.nodeUniform20 );
	nodeVar55 = textureSampleGrad( nodeUniform21, nodeUniform21_sampler, nodeVar54, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar56 = nodeVar55.xyz;
	nodeVar57 = fract( nodeVar48 );

	if ( ( nodeVar57 != 0.0 ) ) {

		nodeVar58 = ( nodeVar49 + 1.0 );
		nodeVar59 = getFace( ( object.nodeUniform17 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar60 = max( ( 4.0 - nodeVar58 ), 0.0 );
		nodeVar58 = max( nodeVar58, 4.0 );
		nodeVar61 = exp2( nodeVar58 );
		nodeVar62 = ( ( getUV( ( object.nodeUniform17 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar59 ) * vec2<f32>( ( nodeVar61 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar59 > 2.0 ) ) {

			nodeVar62.y = ( nodeVar62.y + nodeVar61 );
			nodeVar59 = ( nodeVar59 - 3.0 );
			

		}

		nodeVar62.x = ( nodeVar62.x + ( nodeVar59 * nodeVar61 ) );
		nodeVar62.x = ( nodeVar62.x + ( nodeVar60 * ( 3.0 * 16.0 ) ) );
		nodeVar62.y = ( nodeVar62.y + ( 4.0 * ( exp2( object.nodeUniform16 ) - nodeVar61 ) ) );
		nodeVar62.x = ( nodeVar62.x * object.nodeUniform19 );
		nodeVar62.y = ( nodeVar62.y * object.nodeUniform20 );
		nodeVar63 = textureSampleGrad( nodeUniform21, nodeUniform21_sampler, nodeVar62, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar64 = nodeVar63.xyz;
		nodeVar56 = mix( nodeVar56, nodeVar64, nodeVar57 );
		

	}

	nodeVar65 = ( iblIrradiance + ( ( nodeVar56 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform22 ) ) );
	iblIrradiance = nodeVar65;
	ambientOcclusion = 1.0;
	nodeVar66 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar66;
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
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar144 = ( indirectSpecular + nodeVar137 );
	indirectSpecular = nodeVar144;
	nodeVar145 = ( indirectDiffuse + nodeVar143 );
	indirectDiffuse = nodeVar145;
	nodeVar146 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar146;
	nodeVar147 = dot( normalView, positionViewDirection );
	nodeVar148 = ( clamp( nodeVar147, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar149 = ( Roughness * -16.0 );
	nodeVar150 = ( 1.0 - nodeVar149 );
	nodeVar151 = nodeVar150;
	nodeVar152 = ( - nodeVar151 );
	nodeVar153 = exp2( nodeVar152 );
	nodeVar154 = pow( nodeVar148, nodeVar153 );
	nodeVar155 = ( 1.0 - nodeVar154 );
	nodeVar156 = nodeVar155;
	nodeVar157 = ( ambientOcclusion - nodeVar156 );
	nodeVar158 = ( indirectSpecular * vec3<f32>( clamp( nodeVar157, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar158;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar159 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar159;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar160 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar160;
	nodeVar161 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar161;
	nodeVar162 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar162;

	// result

	output.color = nodeVar162;

	return output;

}
