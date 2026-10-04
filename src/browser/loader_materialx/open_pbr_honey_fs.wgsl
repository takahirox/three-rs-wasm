// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform14 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform11 : mat4x4<f32>,
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
	cameraWorldMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform13 : vec2<f32>
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
var<private> nodeVar2 : vec2<f32>;
var<private> Anisotropy : f32;
var<private> AlphaT : f32;
var<private> AnisotropyT : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> tangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> bitangentView : vec3<f32>;
var<private> TBNViewMatrix : mat3x3<f32>;
var<private> AnisotropyB : vec3<f32>;
var<private> Transmission : f32;
var<private> Thickness : f32;
var<private> AttenuationDistance : f32;
var<private> AttenuationColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : vec2<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec2<f32>;
var<private> nodeVar66 : vec4<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec2<f32>;
var<private> nodeVar74 : vec4<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec2<f32>;
var<private> nodeVar84 : vec4<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec2<f32>;
var<private> nodeVar92 : vec4<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
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
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : f32;
var<private> nodeVar186 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec4<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec4<f32>;

// codes
fn getVolumeTransmissionRay ( n : vec3<f32>, v : vec3<f32>, thickness : f32, ior : f32, modelMatrix : mat4x4<f32> ) -> vec3<f32> {

	


	return ( normalize( refract( ( - v ), normalize( n ), ( 1.0 / ior ) ) ) * ( vec3<f32>( thickness ) * vec3<f32>( length( modelMatrix[ 0u ].xyz ), length( modelMatrix[ 1u ].xyz ), length( modelMatrix[ 2u ].xyz ) ) ) );

}


fn volumeAttenuation ( transmissionDistance : f32, attenuationColor : vec3<f32>, attenuationDistance : f32 ) -> vec3<f32> {

	


	if ( ( attenuationDistance != 0.0 ) ) {

		return exp( ( ( - ( ( - log( attenuationColor ) ) / vec3<f32>( attenuationDistance ) ) ) * vec3<f32>( transmissionDistance ) ) );

	}


	return vec3<f32>( 1.0, 1.0, 1.0 );

}


fn applyIorToRoughness ( roughness : f32, ior : f32 ) -> f32 {

	


	return ( roughness * clamp( ( ( ior * 2.0 ) - 2.0 ), 0.0, 1.0 ) );

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
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec4<f32>,
	@builtin( front_facing ) isFront : bool ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 1.0 ) * vec3<f32>( 0.8, 0.8, 0.8 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.0, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	IOR = 1.504;
	nodeVar1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar1 * nodeVar1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform4 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform4, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	nodeVar2 = ( vec2<f32>( cos( 0.0 ), sin( 0.0 ) ) * vec2<f32>( 0.0 ) );
	Anisotropy = length( nodeVar2 );

	if ( ( Anisotropy == 0.0 ) ) {

		nodeVar2 = vec2<f32>( 1.0, 0.0 );
		

	} else {

		nodeVar2 = ( nodeVar2 / vec2<f32>( Anisotropy ) );
		Anisotropy = clamp( Anisotropy, 0.0, 1.0 );
		

	}

	AlphaT = mix( ( Roughness * Roughness ), 1.0, ( Anisotropy * Anisotropy ) );
	nodeVar3 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	tangentView = ( normalize( v_tangentView ) * vec3<f32>( nodeVar3 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeVar3 ) );
	normalView = NORMAL_normalView;
	bitangentView = ( normalize( ( cross( normalView, tangentView ) * vec3<f32>( nodeVarying8.w ) ) ) * vec3<f32>( nodeVar3 ) );
	TBNViewMatrix = mat3x3<f32>( tangentView, bitangentView, normalView );
	AnisotropyT = ( ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar2.x ) ) + ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar2.y ) ) );
	AnisotropyB = ( ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar2.x ) ) - ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar2.y ) ) );
	Transmission = 1.0;
	Thickness = 2.0;
	AttenuationDistance = 2.0;
	AttenuationColor = vec3<f32>( 0.83, 0.4, 0.04 );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar4 = ( render.cameraPosition - v_positionWorld );
	nodeVar5 = normalize( nodeVar4 );
	nodeVar6 = getVolumeTransmissionRay( normalWorld, nodeVar5, Thickness, IOR, object.nodeUniform11 );
	nodeVar7 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeVar6 ), 1.0 ) ) );
	nodeVar8 = ( nodeVar7.xy / vec2<f32>( nodeVar7.w ) );
	nodeVar8 = ( nodeVar8 + vec2<f32>( 1.0 ) );
	nodeVar8 = ( nodeVar8 / vec2<f32>( 2.0 ) );
	nodeVar8 = vec2<f32>( nodeVar8.x, ( 1.0 - nodeVar8.y ) );
	nodeVar9 = textureSample( nodeUniform12, nodeUniform12_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeVar5 ), 0.0, 1.0 ) ) ).xy;
	nodeVar10 = ( DiffuseContribution * volumeAttenuation( length( nodeVar6 ), AttenuationColor, AttenuationDistance ) );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform13.x, render.nodeUniform13.y );
	nodeVar11 = ( ( ( nodeVar8 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform13 );
	nodeVar12 = ( log2( cameraViewport.z ) * applyIorToRoughness( Roughness, IOR ) );
	nodeVar13 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform14, i32( nodeVar12 ) ) ) ), vec2<f32>( textureDimensions( nodeUniform14, i32( nodeVar12 ) ) ) );
	nodeVar14 = ( ( nodeVar11 * nodeVar13.zw ) + vec2<f32>( 0.5 ) );
	nodeVar15 = fract( nodeVar14 );
	nodeVar16 = ( ( 0.16666666666666666 * ( ( nodeVar15.x * ( ( nodeVar15.x * ( ( - nodeVar15.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar15.x * ( nodeVar15.x * ( ( 3.0 * nodeVar15.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar17 = floor( nodeVar14 );
	nodeVar18 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar15.x * ( nodeVar15.x * ( ( 3.0 * nodeVar15.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar15.x * ( ( nodeVar15.x * ( ( - nodeVar15.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar15.x * ( nodeVar15.x * ( ( 3.0 * nodeVar15.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar19 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar15.y * ( nodeVar15.y * ( ( 3.0 * nodeVar15.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar15.y * ( ( nodeVar15.y * ( ( - nodeVar15.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar15.y * ( nodeVar15.y * ( ( 3.0 * nodeVar15.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar20 = floor( nodeVar12 );
	nodeVar21 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, ( ( vec2<f32>( ( nodeVar17.x + nodeVar18 ), ( nodeVar17.y + nodeVar19 ) ) - vec2<f32>( 0.5 ) ) * nodeVar13.xy ), nodeVar20 );
	nodeVar22 = ( ( 0.16666666666666666 * ( ( nodeVar15.x * ( ( nodeVar15.x * ( ( -3.0 * nodeVar15.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar15.x, 3.0 ) ) );
	nodeVar23 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar15.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar15.x * ( ( nodeVar15.x * ( ( -3.0 * nodeVar15.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar15.x, 3.0 ) ) ) ) );
	nodeVar24 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, ( ( vec2<f32>( ( nodeVar17.x + nodeVar23 ), ( nodeVar17.y + nodeVar19 ) ) - vec2<f32>( 0.5 ) ) * nodeVar13.xy ), nodeVar20 );
	nodeVar25 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar15.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar15.y * ( ( nodeVar15.y * ( ( -3.0 * nodeVar15.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar15.y, 3.0 ) ) ) ) );
	nodeVar26 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, ( ( vec2<f32>( ( nodeVar17.x + nodeVar18 ), ( nodeVar17.y + nodeVar25 ) ) - vec2<f32>( 0.5 ) ) * nodeVar13.xy ), nodeVar20 );
	nodeVar27 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, ( ( vec2<f32>( ( nodeVar17.x + nodeVar23 ), ( nodeVar17.y + nodeVar25 ) ) - vec2<f32>( 0.5 ) ) * nodeVar13.xy ), nodeVar20 );
	nodeVar28 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform14, i32( ( nodeVar12 + 1.0 ) ) ) ) ), vec2<f32>( textureDimensions( nodeUniform14, i32( ( nodeVar12 + 1.0 ) ) ) ) );
	nodeVar29 = ( ( nodeVar11 * nodeVar28.zw ) + vec2<f32>( 0.5 ) );
	nodeVar30 = fract( nodeVar29 );
	nodeVar31 = ( ( 0.16666666666666666 * ( ( nodeVar30.x * ( ( nodeVar30.x * ( ( - nodeVar30.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar30.x * ( nodeVar30.x * ( ( 3.0 * nodeVar30.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar32 = floor( nodeVar29 );
	nodeVar33 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar30.x * ( nodeVar30.x * ( ( 3.0 * nodeVar30.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar30.x * ( ( nodeVar30.x * ( ( - nodeVar30.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar30.x * ( nodeVar30.x * ( ( 3.0 * nodeVar30.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar34 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar30.y * ( nodeVar30.y * ( ( 3.0 * nodeVar30.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar30.y * ( ( nodeVar30.y * ( ( - nodeVar30.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar30.y * ( nodeVar30.y * ( ( 3.0 * nodeVar30.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar35 = ceil( nodeVar12 );
	nodeVar36 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, ( ( vec2<f32>( ( nodeVar32.x + nodeVar33 ), ( nodeVar32.y + nodeVar34 ) ) - vec2<f32>( 0.5 ) ) * nodeVar28.xy ), nodeVar35 );
	nodeVar37 = ( ( 0.16666666666666666 * ( ( nodeVar30.x * ( ( nodeVar30.x * ( ( -3.0 * nodeVar30.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar30.x, 3.0 ) ) );
	nodeVar38 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar30.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar30.x * ( ( nodeVar30.x * ( ( -3.0 * nodeVar30.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar30.x, 3.0 ) ) ) ) );
	nodeVar39 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, ( ( vec2<f32>( ( nodeVar32.x + nodeVar38 ), ( nodeVar32.y + nodeVar34 ) ) - vec2<f32>( 0.5 ) ) * nodeVar28.xy ), nodeVar35 );
	nodeVar40 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar30.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar30.y * ( ( nodeVar30.y * ( ( -3.0 * nodeVar30.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar30.y, 3.0 ) ) ) ) );
	nodeVar41 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, ( ( vec2<f32>( ( nodeVar32.x + nodeVar33 ), ( nodeVar32.y + nodeVar40 ) ) - vec2<f32>( 0.5 ) ) * nodeVar28.xy ), nodeVar35 );
	nodeVar42 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, ( ( vec2<f32>( ( nodeVar32.x + nodeVar38 ), ( nodeVar32.y + nodeVar40 ) ) - vec2<f32>( 0.5 ) ) * nodeVar28.xy ), nodeVar35 );
	nodeVar43 = mix( ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar15.y * ( ( nodeVar15.y * ( ( - nodeVar15.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar15.y * ( nodeVar15.y * ( ( 3.0 * nodeVar15.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar16 ) * nodeVar21 ) + ( vec4<f32>( nodeVar22 ) * nodeVar24 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar15.y * ( ( nodeVar15.y * ( ( -3.0 * nodeVar15.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar15.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar16 ) * nodeVar26 ) + ( vec4<f32>( nodeVar22 ) * nodeVar27 ) ) ) ), ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar30.y * ( ( nodeVar30.y * ( ( - nodeVar30.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar30.y * ( nodeVar30.y * ( ( 3.0 * nodeVar30.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar31 ) * nodeVar36 ) + ( vec4<f32>( nodeVar37 ) * nodeVar39 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar30.y * ( ( nodeVar30.y * ( ( -3.0 * nodeVar30.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar30.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar31 ) * nodeVar41 ) + ( vec4<f32>( nodeVar37 ) * nodeVar42 ) ) ) ), fract( nodeVar12 ) );
	nodeVar44 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar9.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar9.y ) ) ) ) * ( nodeVar10 * nodeVar43.xyz ) ), ( 1.0 - ( ( 1.0 - nodeVar43.w ) * ( ( ( nodeVar10.x + nodeVar10.y ) + nodeVar10.z ) / 3.0 ) ) ) );
	nodeVar45 = mix( 1.0, nodeVar44.w, Transmission );
	nodeVar46 = ( DiffuseColor.w * nodeVar45 );
	DiffuseColor.w = nodeVar46;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar47 = dot( normalView, positionViewDirection );
	nodeVar48 = textureSample( nodeUniform12, nodeUniform12_sampler, vec2<f32>( Roughness, clamp( nodeVar47, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar48;
	nodeVar49 = ( dfg.x + dfg.y );
	nodeVar50 = ( 1.0 / nodeVar49 );
	nodeVar51 = nodeVar50;
	nodeVar52 = ( nodeVar51 - 1.0 );
	nodeVar53 = ( SpecularColorBlended * vec3<f32>( nodeVar52 ) );
	nodeVar54 = ( nodeVar53 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar54;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform15 );
	nodeVar56 = floor( nodeVar55 );
	nodeVar57 = nodeVar56;
	nodeVar58 = ( 1.0 - ( Anisotropy * ( 1.0 - Roughness ) ) );
	nodeVar59 = ( nodeVar58 * nodeVar58 );
	nodeVar60 = normalize( mix( normalize( cross( cross( AnisotropyB, positionViewDirection ), AnisotropyB ) ), normalView, ( nodeVar59 * nodeVar59 ) ) );
	nodeVar61 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), nodeVar60 ), nodeVar60, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar62 = getFace( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( nodeVar61.x, ( - nodeVar61.y ), nodeVar61.z ), 1.0 ) ).xyz );
	nodeVar63 = max( ( 4.0 - nodeVar57 ), 0.0 );
	nodeVar57 = max( nodeVar57, 4.0 );
	nodeVar64 = exp2( nodeVar57 );
	nodeVar65 = ( ( getUV( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( nodeVar61.x, ( - nodeVar61.y ), nodeVar61.z ), 1.0 ) ).xyz, nodeVar62 ) * vec2<f32>( ( nodeVar64 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar62 > 2.0 ) ) {

		nodeVar65.y = ( nodeVar65.y + nodeVar64 );
		nodeVar62 = ( nodeVar62 - 3.0 );
		

	}

	nodeVar65.x = ( nodeVar65.x + ( nodeVar62 * nodeVar64 ) );
	nodeVar65.x = ( nodeVar65.x + ( nodeVar63 * ( 3.0 * 16.0 ) ) );
	nodeVar65.y = ( nodeVar65.y + ( 4.0 * ( exp2( object.nodeUniform15 ) - nodeVar64 ) ) );
	nodeVar65.x = ( nodeVar65.x * object.nodeUniform18 );
	nodeVar65.y = ( nodeVar65.y * object.nodeUniform19 );
	nodeVar66 = textureSampleGrad( nodeUniform20, nodeUniform20_sampler, nodeVar65, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar67 = nodeVar66.xyz;
	nodeVar68 = fract( nodeVar55 );

	if ( ( nodeVar68 != 0.0 ) ) {

		nodeVar69 = ( nodeVar56 + 1.0 );
		nodeVar70 = getFace( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( nodeVar61.x, ( - nodeVar61.y ), nodeVar61.z ), 1.0 ) ).xyz );
		nodeVar71 = max( ( 4.0 - nodeVar69 ), 0.0 );
		nodeVar69 = max( nodeVar69, 4.0 );
		nodeVar72 = exp2( nodeVar69 );
		nodeVar73 = ( ( getUV( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( nodeVar61.x, ( - nodeVar61.y ), nodeVar61.z ), 1.0 ) ).xyz, nodeVar70 ) * vec2<f32>( ( nodeVar72 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar70 > 2.0 ) ) {

			nodeVar73.y = ( nodeVar73.y + nodeVar72 );
			nodeVar70 = ( nodeVar70 - 3.0 );
			

		}

		nodeVar73.x = ( nodeVar73.x + ( nodeVar70 * nodeVar72 ) );
		nodeVar73.x = ( nodeVar73.x + ( nodeVar71 * ( 3.0 * 16.0 ) ) );
		nodeVar73.y = ( nodeVar73.y + ( 4.0 * ( exp2( object.nodeUniform15 ) - nodeVar72 ) ) );
		nodeVar73.x = ( nodeVar73.x * object.nodeUniform18 );
		nodeVar73.y = ( nodeVar73.y * object.nodeUniform19 );
		nodeVar74 = textureSampleGrad( nodeUniform20, nodeUniform20_sampler, nodeVar73, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar75 = nodeVar74.xyz;
		nodeVar67 = mix( nodeVar67, nodeVar75, nodeVar68 );
		

	}

	nodeVar76 = ( radiance + ( nodeVar67 * vec3<f32>( object.nodeUniform21 ) ) );
	radiance = nodeVar76;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar77 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform15 );
	nodeVar78 = floor( nodeVar77 );
	nodeVar79 = nodeVar78;
	nodeVar80 = getFace( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar81 = max( ( 4.0 - nodeVar79 ), 0.0 );
	nodeVar79 = max( nodeVar79, 4.0 );
	nodeVar82 = exp2( nodeVar79 );
	nodeVar83 = ( ( getUV( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar80 ) * vec2<f32>( ( nodeVar82 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar80 > 2.0 ) ) {

		nodeVar83.y = ( nodeVar83.y + nodeVar82 );
		nodeVar80 = ( nodeVar80 - 3.0 );
		

	}

	nodeVar83.x = ( nodeVar83.x + ( nodeVar80 * nodeVar82 ) );
	nodeVar83.x = ( nodeVar83.x + ( nodeVar81 * ( 3.0 * 16.0 ) ) );
	nodeVar83.y = ( nodeVar83.y + ( 4.0 * ( exp2( object.nodeUniform15 ) - nodeVar82 ) ) );
	nodeVar83.x = ( nodeVar83.x * object.nodeUniform18 );
	nodeVar83.y = ( nodeVar83.y * object.nodeUniform19 );
	nodeVar84 = textureSampleGrad( nodeUniform20, nodeUniform20_sampler, nodeVar83, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar85 = nodeVar84.xyz;
	nodeVar86 = fract( nodeVar77 );

	if ( ( nodeVar86 != 0.0 ) ) {

		nodeVar87 = ( nodeVar78 + 1.0 );
		nodeVar88 = getFace( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar89 = max( ( 4.0 - nodeVar87 ), 0.0 );
		nodeVar87 = max( nodeVar87, 4.0 );
		nodeVar90 = exp2( nodeVar87 );
		nodeVar91 = ( ( getUV( ( object.nodeUniform16 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar88 ) * vec2<f32>( ( nodeVar90 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar88 > 2.0 ) ) {

			nodeVar91.y = ( nodeVar91.y + nodeVar90 );
			nodeVar88 = ( nodeVar88 - 3.0 );
			

		}

		nodeVar91.x = ( nodeVar91.x + ( nodeVar88 * nodeVar90 ) );
		nodeVar91.x = ( nodeVar91.x + ( nodeVar89 * ( 3.0 * 16.0 ) ) );
		nodeVar91.y = ( nodeVar91.y + ( 4.0 * ( exp2( object.nodeUniform15 ) - nodeVar90 ) ) );
		nodeVar91.x = ( nodeVar91.x * object.nodeUniform18 );
		nodeVar91.y = ( nodeVar91.y * object.nodeUniform19 );
		nodeVar92 = textureSampleGrad( nodeUniform20, nodeUniform20_sampler, nodeVar91, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar93 = nodeVar92.xyz;
		nodeVar85 = mix( nodeVar85, nodeVar93, nodeVar86 );
		

	}

	nodeVar94 = ( iblIrradiance + ( ( nodeVar85 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform21 ) ) );
	iblIrradiance = nodeVar94;
	nodeVar95 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar96 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar97 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar98 = ( SpecularF90 * dfg.y );
	nodeVar99 = ( nodeVar97 + vec3<f32>( nodeVar98 ) );
	nodeVar100 = ( nodeVar95 + nodeVar99 );
	nodeVar95 = nodeVar100;
	nodeVar101 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( nodeVar102 * vec3<f32>( 0.047619 ) );
	nodeVar104 = ( SpecularColor + nodeVar103 );
	nodeVar105 = ( nodeVar99 * nodeVar104 );
	nodeVar106 = ( dfg.x + dfg.y );
	nodeVar107 = ( 1.0 - nodeVar106 );
	nodeVar108 = nodeVar107;
	nodeVar109 = ( vec3<f32>( nodeVar108 ) * nodeVar104 );
	nodeVar110 = ( vec3<f32>( 1.0 ) - nodeVar109 );
	nodeVar111 = nodeVar110;
	nodeVar112 = ( nodeVar105 / nodeVar111 );
	nodeVar113 = ( nodeVar112 * vec3<f32>( nodeVar108 ) );
	nodeVar114 = ( nodeVar96 + nodeVar113 );
	nodeVar96 = nodeVar114;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar115 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar116 = ( irradiance * nodeVar115 );
	nodeVar117 = ( nodeVar95 + nodeVar96 );
	nodeVar118 = ( vec3<f32>( 1.0 ) - nodeVar117 );
	nodeVar119 = nodeVar118;
	nodeVar120 = ( nodeVar116 * nodeVar119 );
	nodeVar121 = nodeVar120;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar122 = ( indirectDiffuse + nodeVar121 );
	indirectDiffuse = nodeVar122;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar123 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar124 = ( SpecularF90 * dfg.y );
	nodeVar125 = ( nodeVar123 + vec3<f32>( nodeVar124 ) );
	nodeVar126 = ( singleScatteringDielectric + nodeVar125 );
	singleScatteringDielectric = nodeVar126;
	nodeVar127 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar128 = nodeVar127;
	nodeVar129 = ( nodeVar128 * vec3<f32>( 0.047619 ) );
	nodeVar130 = ( SpecularColor + nodeVar129 );
	nodeVar131 = ( nodeVar125 * nodeVar130 );
	nodeVar132 = ( dfg.x + dfg.y );
	nodeVar133 = ( 1.0 - nodeVar132 );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( vec3<f32>( nodeVar134 ) * nodeVar130 );
	nodeVar136 = ( vec3<f32>( 1.0 ) - nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( nodeVar131 / nodeVar137 );
	nodeVar139 = ( nodeVar138 * vec3<f32>( nodeVar134 ) );
	nodeVar140 = ( multiScatteringDielectric + nodeVar139 );
	multiScatteringDielectric = nodeVar140;
	nodeVar141 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar142 = ( SpecularF90 * dfg.y );
	nodeVar143 = ( nodeVar141 + vec3<f32>( nodeVar142 ) );
	nodeVar144 = ( singleScatteringMetallic + nodeVar143 );
	singleScatteringMetallic = nodeVar144;
	nodeVar145 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar146 = nodeVar145;
	nodeVar147 = ( nodeVar146 * vec3<f32>( 0.047619 ) );
	nodeVar148 = ( DiffuseColor.xyz + nodeVar147 );
	nodeVar149 = ( nodeVar143 * nodeVar148 );
	nodeVar150 = ( dfg.x + dfg.y );
	nodeVar151 = ( 1.0 - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( vec3<f32>( nodeVar152 ) * nodeVar148 );
	nodeVar154 = ( vec3<f32>( 1.0 ) - nodeVar153 );
	nodeVar155 = nodeVar154;
	nodeVar156 = ( nodeVar149 / nodeVar155 );
	nodeVar157 = ( nodeVar156 * vec3<f32>( nodeVar152 ) );
	nodeVar158 = ( multiScatteringMetallic + nodeVar157 );
	multiScatteringMetallic = nodeVar158;
	nodeVar159 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar160 = ( radiance * nodeVar159 );
	nodeVar161 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar162 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar163 = ( nodeVar161 * nodeVar162 );
	nodeVar164 = ( nodeVar160 + nodeVar163 );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar167 = ( vec3<f32>( 1.0 ) - nodeVar166 );
	nodeVar168 = nodeVar167;
	nodeVar169 = ( DiffuseContribution * nodeVar168 );
	nodeVar170 = ( nodeVar169 * nodeVar162 );
	nodeVar171 = nodeVar170;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar172 = ( indirectSpecular + nodeVar165 );
	indirectSpecular = nodeVar172;
	nodeVar173 = ( indirectDiffuse + nodeVar171 );
	indirectDiffuse = nodeVar173;
	ambientOcclusion = 1.0;
	nodeVar174 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar174;
	nodeVar175 = dot( normalView, positionViewDirection );
	nodeVar176 = ( clamp( nodeVar175, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar177 = ( Roughness * -16.0 );
	nodeVar178 = ( 1.0 - nodeVar177 );
	nodeVar179 = nodeVar178;
	nodeVar180 = ( - nodeVar179 );
	nodeVar181 = exp2( nodeVar180 );
	nodeVar182 = pow( nodeVar176, nodeVar181 );
	nodeVar183 = ( 1.0 - nodeVar182 );
	nodeVar184 = nodeVar183;
	nodeVar185 = ( ambientOcclusion - nodeVar184 );
	nodeVar186 = ( indirectSpecular * vec3<f32>( clamp( nodeVar185, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar186;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar187 = ( directDiffuse + indirectDiffuse );
	nodeVar188 = mix( vec4<f32>( nodeVar187, 1.0 ), nodeVar44, Transmission );
	totalDiffuse = nodeVar188.xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar189 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar189;
	nodeVar190 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar190;
	nodeVar191 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar191;

	// result

	output.color = nodeVar191;

	return output;

}
