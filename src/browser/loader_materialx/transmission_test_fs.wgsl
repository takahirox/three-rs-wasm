// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform24 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform25 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform17 : vec2<f32>
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
var<private> Transmission : f32;
var<private> Thickness : f32;
var<private> AttenuationDistance : f32;
var<private> AttenuationColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec4<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec2<f32>;
var<private> nodeVar61 : vec4<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec2<f32>;
var<private> nodeVar69 : vec4<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec2<f32>;
var<private> nodeVar79 : vec4<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec2<f32>;
var<private> nodeVar87 : vec4<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar137 : f32;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec4<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec4<f32>;

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
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@builtin( front_facing ) isFront : bool ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( mix( vec3<f32>( 0.9, 0.95, 1.0 ), vec3<f32>( 0.95, 0.98, 1.0 ), 0.9 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.1, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	IOR = object.nodeUniform4;
	nodeVar1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar1 * nodeVar1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform5 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform5, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	Transmission = 0.9;
	Thickness = object.nodeUniform6;
	AttenuationDistance = object.nodeUniform7;
	AttenuationColor = object.nodeUniform8;
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( ( ( f32( isFront ) * 2.0 ) - 1.0 ) ) );
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar2 = ( render.cameraPosition - v_positionWorld );
	nodeVar3 = normalize( nodeVar2 );
	nodeVar4 = getVolumeTransmissionRay( normalWorld, nodeVar3, Thickness, IOR, object.nodeUniform15 );
	nodeVar5 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeVar4 ), 1.0 ) ) );
	nodeVar6 = ( nodeVar5.xy / vec2<f32>( nodeVar5.w ) );
	nodeVar6 = ( nodeVar6 + vec2<f32>( 1.0 ) );
	nodeVar6 = ( nodeVar6 / vec2<f32>( 2.0 ) );
	nodeVar6 = vec2<f32>( nodeVar6.x, ( 1.0 - nodeVar6.y ) );
	nodeVar7 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeVar3 ), 0.0, 1.0 ) ) ).xy;
	nodeVar8 = ( DiffuseContribution * volumeAttenuation( length( nodeVar4 ), AttenuationColor, AttenuationDistance ) );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform17.x, render.nodeUniform17.y );
	nodeVar9 = ( ( ( nodeVar6 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform17 );
	nodeVar10 = ( log2( cameraViewport.z ) * applyIorToRoughness( Roughness, IOR ) );
	nodeVar11 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform18, i32( nodeVar10 ) ) ) ), vec2<f32>( textureDimensions( nodeUniform18, i32( nodeVar10 ) ) ) );
	nodeVar12 = ( ( nodeVar9 * nodeVar11.zw ) + vec2<f32>( 0.5 ) );
	nodeVar13 = fract( nodeVar12 );
	nodeVar14 = ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( ( nodeVar13.x * ( ( - nodeVar13.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar13.x * ( nodeVar13.x * ( ( 3.0 * nodeVar13.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar15 = floor( nodeVar12 );
	nodeVar16 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( nodeVar13.x * ( ( 3.0 * nodeVar13.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( ( nodeVar13.x * ( ( - nodeVar13.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar13.x * ( nodeVar13.x * ( ( 3.0 * nodeVar13.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar17 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( nodeVar13.y * ( ( 3.0 * nodeVar13.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( ( nodeVar13.y * ( ( - nodeVar13.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar13.y * ( nodeVar13.y * ( ( 3.0 * nodeVar13.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar18 = floor( nodeVar10 );
	nodeVar19 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, ( ( vec2<f32>( ( nodeVar15.x + nodeVar16 ), ( nodeVar15.y + nodeVar17 ) ) - vec2<f32>( 0.5 ) ) * nodeVar11.xy ), nodeVar18 );
	nodeVar20 = ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( ( nodeVar13.x * ( ( -3.0 * nodeVar13.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar13.x, 3.0 ) ) );
	nodeVar21 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar13.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( ( nodeVar13.x * ( ( -3.0 * nodeVar13.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar13.x, 3.0 ) ) ) ) );
	nodeVar22 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, ( ( vec2<f32>( ( nodeVar15.x + nodeVar21 ), ( nodeVar15.y + nodeVar17 ) ) - vec2<f32>( 0.5 ) ) * nodeVar11.xy ), nodeVar18 );
	nodeVar23 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar13.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( ( nodeVar13.y * ( ( -3.0 * nodeVar13.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar13.y, 3.0 ) ) ) ) );
	nodeVar24 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, ( ( vec2<f32>( ( nodeVar15.x + nodeVar16 ), ( nodeVar15.y + nodeVar23 ) ) - vec2<f32>( 0.5 ) ) * nodeVar11.xy ), nodeVar18 );
	nodeVar25 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, ( ( vec2<f32>( ( nodeVar15.x + nodeVar21 ), ( nodeVar15.y + nodeVar23 ) ) - vec2<f32>( 0.5 ) ) * nodeVar11.xy ), nodeVar18 );
	nodeVar26 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform18, i32( ( nodeVar10 + 1.0 ) ) ) ) ), vec2<f32>( textureDimensions( nodeUniform18, i32( ( nodeVar10 + 1.0 ) ) ) ) );
	nodeVar27 = ( ( nodeVar9 * nodeVar26.zw ) + vec2<f32>( 0.5 ) );
	nodeVar28 = fract( nodeVar27 );
	nodeVar29 = ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( ( nodeVar28.x * ( ( - nodeVar28.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar28.x * ( nodeVar28.x * ( ( 3.0 * nodeVar28.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar30 = floor( nodeVar27 );
	nodeVar31 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( nodeVar28.x * ( ( 3.0 * nodeVar28.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( ( nodeVar28.x * ( ( - nodeVar28.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar28.x * ( nodeVar28.x * ( ( 3.0 * nodeVar28.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar32 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( nodeVar28.y * ( ( 3.0 * nodeVar28.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( ( nodeVar28.y * ( ( - nodeVar28.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar28.y * ( nodeVar28.y * ( ( 3.0 * nodeVar28.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar33 = ceil( nodeVar10 );
	nodeVar34 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, ( ( vec2<f32>( ( nodeVar30.x + nodeVar31 ), ( nodeVar30.y + nodeVar32 ) ) - vec2<f32>( 0.5 ) ) * nodeVar26.xy ), nodeVar33 );
	nodeVar35 = ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( ( nodeVar28.x * ( ( -3.0 * nodeVar28.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar28.x, 3.0 ) ) );
	nodeVar36 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar28.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( ( nodeVar28.x * ( ( -3.0 * nodeVar28.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar28.x, 3.0 ) ) ) ) );
	nodeVar37 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, ( ( vec2<f32>( ( nodeVar30.x + nodeVar36 ), ( nodeVar30.y + nodeVar32 ) ) - vec2<f32>( 0.5 ) ) * nodeVar26.xy ), nodeVar33 );
	nodeVar38 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar28.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( ( nodeVar28.y * ( ( -3.0 * nodeVar28.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar28.y, 3.0 ) ) ) ) );
	nodeVar39 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, ( ( vec2<f32>( ( nodeVar30.x + nodeVar31 ), ( nodeVar30.y + nodeVar38 ) ) - vec2<f32>( 0.5 ) ) * nodeVar26.xy ), nodeVar33 );
	nodeVar40 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, ( ( vec2<f32>( ( nodeVar30.x + nodeVar36 ), ( nodeVar30.y + nodeVar38 ) ) - vec2<f32>( 0.5 ) ) * nodeVar26.xy ), nodeVar33 );
	nodeVar41 = mix( ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( ( nodeVar13.y * ( ( - nodeVar13.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar13.y * ( nodeVar13.y * ( ( 3.0 * nodeVar13.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar14 ) * nodeVar19 ) + ( vec4<f32>( nodeVar20 ) * nodeVar22 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( ( nodeVar13.y * ( ( -3.0 * nodeVar13.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar13.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar14 ) * nodeVar24 ) + ( vec4<f32>( nodeVar20 ) * nodeVar25 ) ) ) ), ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( ( nodeVar28.y * ( ( - nodeVar28.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar28.y * ( nodeVar28.y * ( ( 3.0 * nodeVar28.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar29 ) * nodeVar34 ) + ( vec4<f32>( nodeVar35 ) * nodeVar37 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( ( nodeVar28.y * ( ( -3.0 * nodeVar28.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar28.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar29 ) * nodeVar39 ) + ( vec4<f32>( nodeVar35 ) * nodeVar40 ) ) ) ), fract( nodeVar10 ) );
	nodeVar42 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar7.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar7.y ) ) ) ) * ( nodeVar8 * nodeVar41.xyz ) ), ( 1.0 - ( ( 1.0 - nodeVar41.w ) * ( ( ( nodeVar8.x + nodeVar8.y ) + nodeVar8.z ) / 3.0 ) ) ) );
	nodeVar43 = mix( 1.0, nodeVar42.w, Transmission );
	nodeVar44 = ( DiffuseColor.w * nodeVar43 );
	DiffuseColor.w = nodeVar44;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar45 = dot( normalView, positionViewDirection );
	nodeVar46 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( nodeVar45, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar46;
	nodeVar47 = ( dfg.x + dfg.y );
	nodeVar48 = ( 1.0 / nodeVar47 );
	nodeVar49 = nodeVar48;
	nodeVar50 = ( nodeVar49 - 1.0 );
	nodeVar51 = ( SpecularColorBlended * vec3<f32>( nodeVar50 ) );
	nodeVar52 = ( nodeVar51 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar52;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar53 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform19 );
	nodeVar54 = floor( nodeVar53 );
	nodeVar55 = nodeVar54;
	nodeVar56 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar57 = getFace( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( nodeVar56.x, ( - nodeVar56.y ), nodeVar56.z ), 1.0 ) ).xyz );
	nodeVar58 = max( ( 4.0 - nodeVar55 ), 0.0 );
	nodeVar55 = max( nodeVar55, 4.0 );
	nodeVar59 = exp2( nodeVar55 );
	nodeVar60 = ( ( getUV( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( nodeVar56.x, ( - nodeVar56.y ), nodeVar56.z ), 1.0 ) ).xyz, nodeVar57 ) * vec2<f32>( ( nodeVar59 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar57 > 2.0 ) ) {

		nodeVar60.y = ( nodeVar60.y + nodeVar59 );
		nodeVar57 = ( nodeVar57 - 3.0 );
		

	}

	nodeVar60.x = ( nodeVar60.x + ( nodeVar57 * nodeVar59 ) );
	nodeVar60.x = ( nodeVar60.x + ( nodeVar58 * ( 3.0 * 16.0 ) ) );
	nodeVar60.y = ( nodeVar60.y + ( 4.0 * ( exp2( object.nodeUniform19 ) - nodeVar59 ) ) );
	nodeVar60.x = ( nodeVar60.x * object.nodeUniform22 );
	nodeVar60.y = ( nodeVar60.y * object.nodeUniform23 );
	nodeVar61 = textureSampleGrad( nodeUniform24, nodeUniform24_sampler, nodeVar60, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar62 = nodeVar61.xyz;
	nodeVar63 = fract( nodeVar53 );

	if ( ( nodeVar63 != 0.0 ) ) {

		nodeVar64 = ( nodeVar54 + 1.0 );
		nodeVar65 = getFace( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( nodeVar56.x, ( - nodeVar56.y ), nodeVar56.z ), 1.0 ) ).xyz );
		nodeVar66 = max( ( 4.0 - nodeVar64 ), 0.0 );
		nodeVar64 = max( nodeVar64, 4.0 );
		nodeVar67 = exp2( nodeVar64 );
		nodeVar68 = ( ( getUV( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( nodeVar56.x, ( - nodeVar56.y ), nodeVar56.z ), 1.0 ) ).xyz, nodeVar65 ) * vec2<f32>( ( nodeVar67 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar65 > 2.0 ) ) {

			nodeVar68.y = ( nodeVar68.y + nodeVar67 );
			nodeVar65 = ( nodeVar65 - 3.0 );
			

		}

		nodeVar68.x = ( nodeVar68.x + ( nodeVar65 * nodeVar67 ) );
		nodeVar68.x = ( nodeVar68.x + ( nodeVar66 * ( 3.0 * 16.0 ) ) );
		nodeVar68.y = ( nodeVar68.y + ( 4.0 * ( exp2( object.nodeUniform19 ) - nodeVar67 ) ) );
		nodeVar68.x = ( nodeVar68.x * object.nodeUniform22 );
		nodeVar68.y = ( nodeVar68.y * object.nodeUniform23 );
		nodeVar69 = textureSampleGrad( nodeUniform24, nodeUniform24_sampler, nodeVar68, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar70 = nodeVar69.xyz;
		nodeVar62 = mix( nodeVar62, nodeVar70, nodeVar63 );
		

	}

	nodeVar71 = ( radiance + ( nodeVar62 * vec3<f32>( object.nodeUniform25 ) ) );
	radiance = nodeVar71;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar72 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform19 );
	nodeVar73 = floor( nodeVar72 );
	nodeVar74 = nodeVar73;
	nodeVar75 = getFace( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar76 = max( ( 4.0 - nodeVar74 ), 0.0 );
	nodeVar74 = max( nodeVar74, 4.0 );
	nodeVar77 = exp2( nodeVar74 );
	nodeVar78 = ( ( getUV( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar75 ) * vec2<f32>( ( nodeVar77 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar75 > 2.0 ) ) {

		nodeVar78.y = ( nodeVar78.y + nodeVar77 );
		nodeVar75 = ( nodeVar75 - 3.0 );
		

	}

	nodeVar78.x = ( nodeVar78.x + ( nodeVar75 * nodeVar77 ) );
	nodeVar78.x = ( nodeVar78.x + ( nodeVar76 * ( 3.0 * 16.0 ) ) );
	nodeVar78.y = ( nodeVar78.y + ( 4.0 * ( exp2( object.nodeUniform19 ) - nodeVar77 ) ) );
	nodeVar78.x = ( nodeVar78.x * object.nodeUniform22 );
	nodeVar78.y = ( nodeVar78.y * object.nodeUniform23 );
	nodeVar79 = textureSampleGrad( nodeUniform24, nodeUniform24_sampler, nodeVar78, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar80 = nodeVar79.xyz;
	nodeVar81 = fract( nodeVar72 );

	if ( ( nodeVar81 != 0.0 ) ) {

		nodeVar82 = ( nodeVar73 + 1.0 );
		nodeVar83 = getFace( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar84 = max( ( 4.0 - nodeVar82 ), 0.0 );
		nodeVar82 = max( nodeVar82, 4.0 );
		nodeVar85 = exp2( nodeVar82 );
		nodeVar86 = ( ( getUV( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar83 ) * vec2<f32>( ( nodeVar85 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar83 > 2.0 ) ) {

			nodeVar86.y = ( nodeVar86.y + nodeVar85 );
			nodeVar83 = ( nodeVar83 - 3.0 );
			

		}

		nodeVar86.x = ( nodeVar86.x + ( nodeVar83 * nodeVar85 ) );
		nodeVar86.x = ( nodeVar86.x + ( nodeVar84 * ( 3.0 * 16.0 ) ) );
		nodeVar86.y = ( nodeVar86.y + ( 4.0 * ( exp2( object.nodeUniform19 ) - nodeVar85 ) ) );
		nodeVar86.x = ( nodeVar86.x * object.nodeUniform22 );
		nodeVar86.y = ( nodeVar86.y * object.nodeUniform23 );
		nodeVar87 = textureSampleGrad( nodeUniform24, nodeUniform24_sampler, nodeVar86, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar88 = nodeVar87.xyz;
		nodeVar80 = mix( nodeVar80, nodeVar88, nodeVar81 );
		

	}

	nodeVar89 = ( iblIrradiance + ( ( nodeVar80 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform25 ) ) );
	iblIrradiance = nodeVar89;
	nodeVar90 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar91 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar92 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar93 = ( SpecularF90 * dfg.y );
	nodeVar94 = ( nodeVar92 + vec3<f32>( nodeVar93 ) );
	nodeVar95 = ( nodeVar90 + nodeVar94 );
	nodeVar90 = nodeVar95;
	nodeVar96 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar97 = nodeVar96;
	nodeVar98 = ( nodeVar97 * vec3<f32>( 0.047619 ) );
	nodeVar99 = ( SpecularColor + nodeVar98 );
	nodeVar100 = ( nodeVar94 * nodeVar99 );
	nodeVar101 = ( dfg.x + dfg.y );
	nodeVar102 = ( 1.0 - nodeVar101 );
	nodeVar103 = nodeVar102;
	nodeVar104 = ( vec3<f32>( nodeVar103 ) * nodeVar99 );
	nodeVar105 = ( vec3<f32>( 1.0 ) - nodeVar104 );
	nodeVar106 = nodeVar105;
	nodeVar107 = ( nodeVar100 / nodeVar106 );
	nodeVar108 = ( nodeVar107 * vec3<f32>( nodeVar103 ) );
	nodeVar109 = ( nodeVar91 + nodeVar108 );
	nodeVar91 = nodeVar109;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar110 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar111 = ( irradiance * nodeVar110 );
	nodeVar112 = ( nodeVar90 + nodeVar91 );
	nodeVar113 = ( vec3<f32>( 1.0 ) - nodeVar112 );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar111 * nodeVar114 );
	nodeVar116 = nodeVar115;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar117 = ( indirectDiffuse + nodeVar116 );
	indirectDiffuse = nodeVar117;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar118 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar119 = ( SpecularF90 * dfg.y );
	nodeVar120 = ( nodeVar118 + vec3<f32>( nodeVar119 ) );
	nodeVar121 = ( singleScatteringDielectric + nodeVar120 );
	singleScatteringDielectric = nodeVar121;
	nodeVar122 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( nodeVar123 * vec3<f32>( 0.047619 ) );
	nodeVar125 = ( SpecularColor + nodeVar124 );
	nodeVar126 = ( nodeVar120 * nodeVar125 );
	nodeVar127 = ( dfg.x + dfg.y );
	nodeVar128 = ( 1.0 - nodeVar127 );
	nodeVar129 = nodeVar128;
	nodeVar130 = ( vec3<f32>( nodeVar129 ) * nodeVar125 );
	nodeVar131 = ( vec3<f32>( 1.0 ) - nodeVar130 );
	nodeVar132 = nodeVar131;
	nodeVar133 = ( nodeVar126 / nodeVar132 );
	nodeVar134 = ( nodeVar133 * vec3<f32>( nodeVar129 ) );
	nodeVar135 = ( multiScatteringDielectric + nodeVar134 );
	multiScatteringDielectric = nodeVar135;
	nodeVar136 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar137 = ( SpecularF90 * dfg.y );
	nodeVar138 = ( nodeVar136 + vec3<f32>( nodeVar137 ) );
	nodeVar139 = ( singleScatteringMetallic + nodeVar138 );
	singleScatteringMetallic = nodeVar139;
	nodeVar140 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar141 = nodeVar140;
	nodeVar142 = ( nodeVar141 * vec3<f32>( 0.047619 ) );
	nodeVar143 = ( DiffuseColor.xyz + nodeVar142 );
	nodeVar144 = ( nodeVar138 * nodeVar143 );
	nodeVar145 = ( dfg.x + dfg.y );
	nodeVar146 = ( 1.0 - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( vec3<f32>( nodeVar147 ) * nodeVar143 );
	nodeVar149 = ( vec3<f32>( 1.0 ) - nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( nodeVar144 / nodeVar150 );
	nodeVar152 = ( nodeVar151 * vec3<f32>( nodeVar147 ) );
	nodeVar153 = ( multiScatteringMetallic + nodeVar152 );
	multiScatteringMetallic = nodeVar153;
	nodeVar154 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar155 = ( radiance * nodeVar154 );
	nodeVar156 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar157 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar158 = ( nodeVar156 * nodeVar157 );
	nodeVar159 = ( nodeVar155 + nodeVar158 );
	nodeVar160 = nodeVar159;
	nodeVar161 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar162 = ( vec3<f32>( 1.0 ) - nodeVar161 );
	nodeVar163 = nodeVar162;
	nodeVar164 = ( DiffuseContribution * nodeVar163 );
	nodeVar165 = ( nodeVar164 * nodeVar157 );
	nodeVar166 = nodeVar165;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar167 = ( indirectSpecular + nodeVar160 );
	indirectSpecular = nodeVar167;
	nodeVar168 = ( indirectDiffuse + nodeVar166 );
	indirectDiffuse = nodeVar168;
	ambientOcclusion = 1.0;
	nodeVar169 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar169;
	nodeVar170 = dot( normalView, positionViewDirection );
	nodeVar171 = ( clamp( nodeVar170, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar172 = ( Roughness * -16.0 );
	nodeVar173 = ( 1.0 - nodeVar172 );
	nodeVar174 = nodeVar173;
	nodeVar175 = ( - nodeVar174 );
	nodeVar176 = exp2( nodeVar175 );
	nodeVar177 = pow( nodeVar171, nodeVar176 );
	nodeVar178 = ( 1.0 - nodeVar177 );
	nodeVar179 = nodeVar178;
	nodeVar180 = ( ambientOcclusion - nodeVar179 );
	nodeVar181 = ( indirectSpecular * vec3<f32>( clamp( nodeVar180, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar181;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar182 = ( directDiffuse + indirectDiffuse );
	nodeVar183 = mix( vec4<f32>( nodeVar182, 1.0 ), nodeVar42, Transmission );
	totalDiffuse = nodeVar183.xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar184 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar184;
	nodeVar185 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar185;
	nodeVar186 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar186;

	// result

	output.color = nodeVar186;

	return output;

}
