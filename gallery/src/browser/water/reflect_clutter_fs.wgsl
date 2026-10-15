// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform17_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform17 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform24 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform29_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform29 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform34_sampler : sampler;
@binding( 12 ) @group( 1 ) var nodeUniform34 : texture_2d<f32>;
@binding( 13 ) @group( 1 ) var nodeUniform36_sampler : sampler;
@binding( 14 ) @group( 1 ) var nodeUniform36 : texture_2d<f32>;
@binding( 15 ) @group( 1 ) var nodeUniform42_sampler : sampler;
@binding( 16 ) @group( 1 ) var nodeUniform42 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : f32,
	nodeUniform25 : mat3x3<f32>,
	nodeUniform28 : mat4x4<f32>,
	nodeUniform30 : mat3x3<f32>,
	nodeUniform31 : vec2<f32>,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform37 : f32,
	nodeUniform38 : mat4x4<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform43 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform35 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> Transmission : f32;
var<private> nodeVar6 : vec4<f32>;
var<private> Thickness : f32;
var<private> AttenuationDistance : f32;
var<private> AttenuationColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec4<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec4<f32>;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec4<f32>;
var<private> nodeVar58 : vec4<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec2<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : vec2<f32>;
var<private> nodeVar77 : vec4<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec2<f32>;
var<private> nodeVar85 : vec4<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : vec2<f32>;
var<private> nodeVar95 : vec4<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : vec2<f32>;
var<private> nodeVar103 : vec4<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar106 : f32;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : f32;
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
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : f32;
var<private> nodeVar190 : f32;
var<private> nodeVar191 : f32;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : f32;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec4<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec4<f32>;

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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) nodeVarying7 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	AmbientOcclusion = ( ( ( nodeVar1.x - 1.0 ) * object.nodeUniform6 ) + 1.0 );
	nodeVar2 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform10 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform9 * nodeVar3.y ), 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	IOR = object.nodeUniform13;
	nodeVar5 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar5 * nodeVar5 ) ) * object.nodeUniform14 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform15 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform15, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	nodeVar6 = textureSample( nodeUniform17, nodeUniform17_sampler, ( object.nodeUniform18 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	Transmission = ( object.nodeUniform16 * nodeVar6.x );
	Thickness = object.nodeUniform19;
	AttenuationDistance = object.nodeUniform20;
	AttenuationColor = object.nodeUniform21;
	nodeVar7 = textureSample( nodeUniform24, nodeUniform24_sampler, ( object.nodeUniform25 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	EmissiveColor = ( vec4<f32>( ( object.nodeUniform22 * vec3<f32>( object.nodeUniform23 ) ), 1.0 ) * nodeVar7 ).xyz;
	NORMAL_normalView = normalViewGeometry;
	nodeVar8 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	nodeVar9 = dpdx( nodeVarying7 );
	nodeVar10 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	nodeVar11 = - dpdy( nodeVarying7 );
	nodeVar12 = ( ( nodeVar8 * vec3<f32>( nodeVar9.x ) ) + ( nodeVar10 * vec3<f32>( nodeVar11.x ) ) );
	nodeVar14 = ( ( nodeVar8 * vec3<f32>( nodeVar9.y ) ) + ( nodeVar10 * vec3<f32>( nodeVar11.y ) ) );
	nodeVar15 = max( dot( nodeVar12, nodeVar12 ), dot( nodeVar14, nodeVar14 ) );

	if ( ( nodeVar15 == 0.0 ) ) {

		nodeVar13 = 0.0;

	} else {

		nodeVar13 = inverseSqrt( nodeVar15 );

	}

	tangentViewFrame = ( nodeVar12 * vec3<f32>( nodeVar13 ) );
	NORMAL_tangentView = tangentViewFrame;
	bitangentViewFrame = ( nodeVar14 * nodeVar13 );
	NORMAL_bitangentView = bitangentViewFrame;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar16 = textureSample( nodeUniform29, nodeUniform29_sampler, ( object.nodeUniform30 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	nodeVar17 = ( ( nodeVar16 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar17.xy * object.nodeUniform31 ), nodeVar17.z ) ) );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar18 = ( render.cameraPosition - v_positionWorld );
	nodeVar19 = normalize( nodeVar18 );
	nodeVar20 = getVolumeTransmissionRay( normalWorld, nodeVar19, Thickness, IOR, object.nodeUniform33 );
	nodeVar21 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeVar20 ), 1.0 ) ) );
	nodeVar22 = ( nodeVar21.xy / vec2<f32>( nodeVar21.w ) );
	nodeVar22 = ( nodeVar22 + vec2<f32>( 1.0 ) );
	nodeVar22 = ( nodeVar22 / vec2<f32>( 2.0 ) );
	nodeVar22 = vec2<f32>( nodeVar22.x, ( 1.0 - nodeVar22.y ) );
	nodeVar23 = textureSample( nodeUniform34, nodeUniform34_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeVar19 ), 0.0, 1.0 ) ) ).xy;
	nodeVar24 = ( DiffuseContribution * volumeAttenuation( length( nodeVar20 ), AttenuationColor, AttenuationDistance ) );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform35.x, render.nodeUniform35.y );
	nodeVar25 = ( ( ( nodeVar22 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform35 );
	nodeVar26 = ( log2( cameraViewport.z ) * applyIorToRoughness( Roughness, IOR ) );
	nodeVar27 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform36, i32( nodeVar26 ) ) ) ), vec2<f32>( textureDimensions( nodeUniform36, i32( nodeVar26 ) ) ) );
	nodeVar28 = ( ( nodeVar25 * nodeVar27.zw ) + vec2<f32>( 0.5 ) );
	nodeVar29 = fract( nodeVar28 );
	nodeVar30 = ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( - nodeVar29.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.x * ( nodeVar29.x * ( ( 3.0 * nodeVar29.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar31 = floor( nodeVar28 );
	nodeVar32 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( nodeVar29.x * ( ( 3.0 * nodeVar29.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( - nodeVar29.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.x * ( nodeVar29.x * ( ( 3.0 * nodeVar29.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar33 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( nodeVar29.y * ( ( 3.0 * nodeVar29.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( - nodeVar29.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.y * ( nodeVar29.y * ( ( 3.0 * nodeVar29.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar34 = floor( nodeVar26 );
	nodeVar35 = textureSampleLevel( nodeUniform36, nodeUniform36_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar32 ), ( nodeVar31.y + nodeVar33 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
	nodeVar36 = ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( -3.0 * nodeVar29.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.x, 3.0 ) ) );
	nodeVar37 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar29.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( -3.0 * nodeVar29.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.x, 3.0 ) ) ) ) );
	nodeVar38 = textureSampleLevel( nodeUniform36, nodeUniform36_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar37 ), ( nodeVar31.y + nodeVar33 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
	nodeVar39 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar29.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( -3.0 * nodeVar29.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.y, 3.0 ) ) ) ) );
	nodeVar40 = textureSampleLevel( nodeUniform36, nodeUniform36_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar32 ), ( nodeVar31.y + nodeVar39 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
	nodeVar41 = textureSampleLevel( nodeUniform36, nodeUniform36_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar37 ), ( nodeVar31.y + nodeVar39 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
	nodeVar42 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform36, i32( ( nodeVar26 + 1.0 ) ) ) ) ), vec2<f32>( textureDimensions( nodeUniform36, i32( ( nodeVar26 + 1.0 ) ) ) ) );
	nodeVar43 = ( ( nodeVar25 * nodeVar42.zw ) + vec2<f32>( 0.5 ) );
	nodeVar44 = fract( nodeVar43 );
	nodeVar45 = ( ( 0.16666666666666666 * ( ( nodeVar44.x * ( ( nodeVar44.x * ( ( - nodeVar44.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar44.x * ( nodeVar44.x * ( ( 3.0 * nodeVar44.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar46 = floor( nodeVar43 );
	nodeVar47 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar44.x * ( nodeVar44.x * ( ( 3.0 * nodeVar44.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar44.x * ( ( nodeVar44.x * ( ( - nodeVar44.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar44.x * ( nodeVar44.x * ( ( 3.0 * nodeVar44.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar48 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar44.y * ( nodeVar44.y * ( ( 3.0 * nodeVar44.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar44.y * ( ( nodeVar44.y * ( ( - nodeVar44.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar44.y * ( nodeVar44.y * ( ( 3.0 * nodeVar44.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar49 = ceil( nodeVar26 );
	nodeVar50 = textureSampleLevel( nodeUniform36, nodeUniform36_sampler, ( ( vec2<f32>( ( nodeVar46.x + nodeVar47 ), ( nodeVar46.y + nodeVar48 ) ) - vec2<f32>( 0.5 ) ) * nodeVar42.xy ), nodeVar49 );
	nodeVar51 = ( ( 0.16666666666666666 * ( ( nodeVar44.x * ( ( nodeVar44.x * ( ( -3.0 * nodeVar44.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar44.x, 3.0 ) ) );
	nodeVar52 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar44.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar44.x * ( ( nodeVar44.x * ( ( -3.0 * nodeVar44.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar44.x, 3.0 ) ) ) ) );
	nodeVar53 = textureSampleLevel( nodeUniform36, nodeUniform36_sampler, ( ( vec2<f32>( ( nodeVar46.x + nodeVar52 ), ( nodeVar46.y + nodeVar48 ) ) - vec2<f32>( 0.5 ) ) * nodeVar42.xy ), nodeVar49 );
	nodeVar54 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar44.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar44.y * ( ( nodeVar44.y * ( ( -3.0 * nodeVar44.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar44.y, 3.0 ) ) ) ) );
	nodeVar55 = textureSampleLevel( nodeUniform36, nodeUniform36_sampler, ( ( vec2<f32>( ( nodeVar46.x + nodeVar47 ), ( nodeVar46.y + nodeVar54 ) ) - vec2<f32>( 0.5 ) ) * nodeVar42.xy ), nodeVar49 );
	nodeVar56 = textureSampleLevel( nodeUniform36, nodeUniform36_sampler, ( ( vec2<f32>( ( nodeVar46.x + nodeVar52 ), ( nodeVar46.y + nodeVar54 ) ) - vec2<f32>( 0.5 ) ) * nodeVar42.xy ), nodeVar49 );
	nodeVar57 = mix( ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( - nodeVar29.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.y * ( nodeVar29.y * ( ( 3.0 * nodeVar29.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar30 ) * nodeVar35 ) + ( vec4<f32>( nodeVar36 ) * nodeVar38 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( -3.0 * nodeVar29.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar30 ) * nodeVar40 ) + ( vec4<f32>( nodeVar36 ) * nodeVar41 ) ) ) ), ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar44.y * ( ( nodeVar44.y * ( ( - nodeVar44.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar44.y * ( nodeVar44.y * ( ( 3.0 * nodeVar44.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar45 ) * nodeVar50 ) + ( vec4<f32>( nodeVar51 ) * nodeVar53 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar44.y * ( ( nodeVar44.y * ( ( -3.0 * nodeVar44.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar44.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar45 ) * nodeVar55 ) + ( vec4<f32>( nodeVar51 ) * nodeVar56 ) ) ) ), fract( nodeVar26 ) );
	nodeVar58 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar23.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar23.y ) ) ) ) * ( nodeVar24 * nodeVar57.xyz ) ), ( 1.0 - ( ( 1.0 - nodeVar57.w ) * ( ( ( nodeVar24.x + nodeVar24.y ) + nodeVar24.z ) / 3.0 ) ) ) );
	nodeVar59 = mix( 1.0, nodeVar58.w, Transmission );
	nodeVar60 = ( DiffuseColor.w * nodeVar59 );
	DiffuseColor.w = nodeVar60;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar61 = dot( normalView, positionViewDirection );
	nodeVar62 = textureSample( nodeUniform34, nodeUniform34_sampler, vec2<f32>( Roughness, clamp( nodeVar61, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar62;
	nodeVar63 = ( dfg.x + dfg.y );
	nodeVar64 = ( 1.0 / nodeVar63 );
	nodeVar65 = nodeVar64;
	nodeVar66 = ( nodeVar65 - 1.0 );
	nodeVar67 = ( SpecularColorBlended * vec3<f32>( nodeVar66 ) );
	nodeVar68 = ( nodeVar67 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar68;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform37 );
	nodeVar70 = floor( nodeVar69 );
	nodeVar71 = nodeVar70;
	nodeVar72 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar73 = getFace( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( nodeVar72.x, ( - nodeVar72.y ), nodeVar72.z ), 1.0 ) ).xyz );
	nodeVar74 = max( ( 4.0 - nodeVar71 ), 0.0 );
	nodeVar71 = max( nodeVar71, 4.0 );
	nodeVar75 = exp2( nodeVar71 );
	nodeVar76 = ( ( getUV( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( nodeVar72.x, ( - nodeVar72.y ), nodeVar72.z ), 1.0 ) ).xyz, nodeVar73 ) * vec2<f32>( ( nodeVar75 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar73 > 2.0 ) ) {

		nodeVar76.y = ( nodeVar76.y + nodeVar75 );
		nodeVar73 = ( nodeVar73 - 3.0 );
		

	}

	nodeVar76.x = ( nodeVar76.x + ( nodeVar73 * nodeVar75 ) );
	nodeVar76.x = ( nodeVar76.x + ( nodeVar74 * ( 3.0 * 16.0 ) ) );
	nodeVar76.y = ( nodeVar76.y + ( 4.0 * ( exp2( object.nodeUniform37 ) - nodeVar75 ) ) );
	nodeVar76.x = ( nodeVar76.x * object.nodeUniform40 );
	nodeVar76.y = ( nodeVar76.y * object.nodeUniform41 );
	nodeVar77 = textureSampleGrad( nodeUniform42, nodeUniform42_sampler, nodeVar76, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar78 = nodeVar77.xyz;
	nodeVar79 = fract( nodeVar69 );

	if ( ( nodeVar79 != 0.0 ) ) {

		nodeVar80 = ( nodeVar70 + 1.0 );
		nodeVar81 = getFace( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( nodeVar72.x, ( - nodeVar72.y ), nodeVar72.z ), 1.0 ) ).xyz );
		nodeVar82 = max( ( 4.0 - nodeVar80 ), 0.0 );
		nodeVar80 = max( nodeVar80, 4.0 );
		nodeVar83 = exp2( nodeVar80 );
		nodeVar84 = ( ( getUV( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( nodeVar72.x, ( - nodeVar72.y ), nodeVar72.z ), 1.0 ) ).xyz, nodeVar81 ) * vec2<f32>( ( nodeVar83 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar81 > 2.0 ) ) {

			nodeVar84.y = ( nodeVar84.y + nodeVar83 );
			nodeVar81 = ( nodeVar81 - 3.0 );
			

		}

		nodeVar84.x = ( nodeVar84.x + ( nodeVar81 * nodeVar83 ) );
		nodeVar84.x = ( nodeVar84.x + ( nodeVar82 * ( 3.0 * 16.0 ) ) );
		nodeVar84.y = ( nodeVar84.y + ( 4.0 * ( exp2( object.nodeUniform37 ) - nodeVar83 ) ) );
		nodeVar84.x = ( nodeVar84.x * object.nodeUniform40 );
		nodeVar84.y = ( nodeVar84.y * object.nodeUniform41 );
		nodeVar85 = textureSampleGrad( nodeUniform42, nodeUniform42_sampler, nodeVar84, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar86 = nodeVar85.xyz;
		nodeVar78 = mix( nodeVar78, nodeVar86, nodeVar79 );
		

	}

	nodeVar87 = ( radiance + ( nodeVar78 * vec3<f32>( object.nodeUniform43 ) ) );
	radiance = nodeVar87;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar88 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform37 );
	nodeVar89 = floor( nodeVar88 );
	nodeVar90 = nodeVar89;
	nodeVar91 = getFace( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar92 = max( ( 4.0 - nodeVar90 ), 0.0 );
	nodeVar90 = max( nodeVar90, 4.0 );
	nodeVar93 = exp2( nodeVar90 );
	nodeVar94 = ( ( getUV( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar91 ) * vec2<f32>( ( nodeVar93 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar91 > 2.0 ) ) {

		nodeVar94.y = ( nodeVar94.y + nodeVar93 );
		nodeVar91 = ( nodeVar91 - 3.0 );
		

	}

	nodeVar94.x = ( nodeVar94.x + ( nodeVar91 * nodeVar93 ) );
	nodeVar94.x = ( nodeVar94.x + ( nodeVar92 * ( 3.0 * 16.0 ) ) );
	nodeVar94.y = ( nodeVar94.y + ( 4.0 * ( exp2( object.nodeUniform37 ) - nodeVar93 ) ) );
	nodeVar94.x = ( nodeVar94.x * object.nodeUniform40 );
	nodeVar94.y = ( nodeVar94.y * object.nodeUniform41 );
	nodeVar95 = textureSampleGrad( nodeUniform42, nodeUniform42_sampler, nodeVar94, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar96 = nodeVar95.xyz;
	nodeVar97 = fract( nodeVar88 );

	if ( ( nodeVar97 != 0.0 ) ) {

		nodeVar98 = ( nodeVar89 + 1.0 );
		nodeVar99 = getFace( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar100 = max( ( 4.0 - nodeVar98 ), 0.0 );
		nodeVar98 = max( nodeVar98, 4.0 );
		nodeVar101 = exp2( nodeVar98 );
		nodeVar102 = ( ( getUV( ( object.nodeUniform38 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar99 ) * vec2<f32>( ( nodeVar101 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar99 > 2.0 ) ) {

			nodeVar102.y = ( nodeVar102.y + nodeVar101 );
			nodeVar99 = ( nodeVar99 - 3.0 );
			

		}

		nodeVar102.x = ( nodeVar102.x + ( nodeVar99 * nodeVar101 ) );
		nodeVar102.x = ( nodeVar102.x + ( nodeVar100 * ( 3.0 * 16.0 ) ) );
		nodeVar102.y = ( nodeVar102.y + ( 4.0 * ( exp2( object.nodeUniform37 ) - nodeVar101 ) ) );
		nodeVar102.x = ( nodeVar102.x * object.nodeUniform40 );
		nodeVar102.y = ( nodeVar102.y * object.nodeUniform41 );
		nodeVar103 = textureSampleGrad( nodeUniform42, nodeUniform42_sampler, nodeVar102, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar104 = nodeVar103.xyz;
		nodeVar96 = mix( nodeVar96, nodeVar104, nodeVar97 );
		

	}

	nodeVar105 = ( iblIrradiance + ( ( nodeVar96 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform43 ) ) );
	iblIrradiance = nodeVar105;
	ambientOcclusion = 1.0;
	nodeVar106 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar106;
	nodeVar107 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar108 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar109 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar110 = ( SpecularF90 * dfg.y );
	nodeVar111 = ( nodeVar109 + vec3<f32>( nodeVar110 ) );
	nodeVar112 = ( nodeVar107 + nodeVar111 );
	nodeVar107 = nodeVar112;
	nodeVar113 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar114 * vec3<f32>( 0.047619 ) );
	nodeVar116 = ( SpecularColor + nodeVar115 );
	nodeVar117 = ( nodeVar111 * nodeVar116 );
	nodeVar118 = ( dfg.x + dfg.y );
	nodeVar119 = ( 1.0 - nodeVar118 );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( vec3<f32>( nodeVar120 ) * nodeVar116 );
	nodeVar122 = ( vec3<f32>( 1.0 ) - nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( nodeVar117 / nodeVar123 );
	nodeVar125 = ( nodeVar124 * vec3<f32>( nodeVar120 ) );
	nodeVar126 = ( nodeVar108 + nodeVar125 );
	nodeVar108 = nodeVar126;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar127 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar128 = ( irradiance * nodeVar127 );
	nodeVar129 = ( nodeVar107 + nodeVar108 );
	nodeVar130 = ( vec3<f32>( 1.0 ) - nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( nodeVar128 * nodeVar131 );
	nodeVar133 = nodeVar132;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar134 = ( indirectDiffuse + nodeVar133 );
	indirectDiffuse = nodeVar134;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar135 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar136 = ( SpecularF90 * dfg.y );
	nodeVar137 = ( nodeVar135 + vec3<f32>( nodeVar136 ) );
	nodeVar138 = ( singleScatteringDielectric + nodeVar137 );
	singleScatteringDielectric = nodeVar138;
	nodeVar139 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( nodeVar140 * vec3<f32>( 0.047619 ) );
	nodeVar142 = ( SpecularColor + nodeVar141 );
	nodeVar143 = ( nodeVar137 * nodeVar142 );
	nodeVar144 = ( dfg.x + dfg.y );
	nodeVar145 = ( 1.0 - nodeVar144 );
	nodeVar146 = nodeVar145;
	nodeVar147 = ( vec3<f32>( nodeVar146 ) * nodeVar142 );
	nodeVar148 = ( vec3<f32>( 1.0 ) - nodeVar147 );
	nodeVar149 = nodeVar148;
	nodeVar150 = ( nodeVar143 / nodeVar149 );
	nodeVar151 = ( nodeVar150 * vec3<f32>( nodeVar146 ) );
	nodeVar152 = ( multiScatteringDielectric + nodeVar151 );
	multiScatteringDielectric = nodeVar152;
	nodeVar153 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar154 = ( SpecularF90 * dfg.y );
	nodeVar155 = ( nodeVar153 + vec3<f32>( nodeVar154 ) );
	nodeVar156 = ( singleScatteringMetallic + nodeVar155 );
	singleScatteringMetallic = nodeVar156;
	nodeVar157 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar158 = nodeVar157;
	nodeVar159 = ( nodeVar158 * vec3<f32>( 0.047619 ) );
	nodeVar160 = ( DiffuseColor.xyz + nodeVar159 );
	nodeVar161 = ( nodeVar155 * nodeVar160 );
	nodeVar162 = ( dfg.x + dfg.y );
	nodeVar163 = ( 1.0 - nodeVar162 );
	nodeVar164 = nodeVar163;
	nodeVar165 = ( vec3<f32>( nodeVar164 ) * nodeVar160 );
	nodeVar166 = ( vec3<f32>( 1.0 ) - nodeVar165 );
	nodeVar167 = nodeVar166;
	nodeVar168 = ( nodeVar161 / nodeVar167 );
	nodeVar169 = ( nodeVar168 * vec3<f32>( nodeVar164 ) );
	nodeVar170 = ( multiScatteringMetallic + nodeVar169 );
	multiScatteringMetallic = nodeVar170;
	nodeVar171 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar172 = ( radiance * nodeVar171 );
	nodeVar173 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar174 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar175 = ( nodeVar173 * nodeVar174 );
	nodeVar176 = ( nodeVar172 + nodeVar175 );
	nodeVar177 = nodeVar176;
	nodeVar178 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar179 = ( vec3<f32>( 1.0 ) - nodeVar178 );
	nodeVar180 = nodeVar179;
	nodeVar181 = ( DiffuseContribution * nodeVar180 );
	nodeVar182 = ( nodeVar181 * nodeVar174 );
	nodeVar183 = nodeVar182;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar184 = ( indirectSpecular + nodeVar177 );
	indirectSpecular = nodeVar184;
	nodeVar185 = ( indirectDiffuse + nodeVar183 );
	indirectDiffuse = nodeVar185;
	nodeVar186 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar186;
	nodeVar187 = dot( normalView, positionViewDirection );
	nodeVar188 = ( clamp( nodeVar187, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar189 = ( Roughness * -16.0 );
	nodeVar190 = ( 1.0 - nodeVar189 );
	nodeVar191 = nodeVar190;
	nodeVar192 = ( - nodeVar191 );
	nodeVar193 = exp2( nodeVar192 );
	nodeVar194 = pow( nodeVar188, nodeVar193 );
	nodeVar195 = ( 1.0 - nodeVar194 );
	nodeVar196 = nodeVar195;
	nodeVar197 = ( ambientOcclusion - nodeVar196 );
	nodeVar198 = ( indirectSpecular * vec3<f32>( clamp( nodeVar197, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar198;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar199 = ( directDiffuse + indirectDiffuse );
	nodeVar200 = mix( vec4<f32>( nodeVar199, 1.0 ), nodeVar58, Transmission );
	totalDiffuse = nodeVar200.xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar201 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar201;
	nodeVar202 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar202;
	nodeVar203 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar203;

	// result

	output.color = nodeVar203;

	return output;

}
