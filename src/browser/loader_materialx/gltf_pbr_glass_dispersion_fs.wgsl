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
@binding( 3 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform19 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform10 : mat4x4<f32>,
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
	cameraWorldMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform11 : vec2<f32>
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
var<private> Dispersion : f32;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
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
var<private> nodeVar58 : vec3<f32>;
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
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : vec2<f32>;
var<private> nodeVar81 : vec4<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec2<f32>;
var<private> nodeVar89 : vec4<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
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
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec4<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec4<f32>;

// codes
fn getVolumeTransmissionRay ( n : vec3<f32>, v : vec3<f32>, thickness : f32, ior : f32, modelMatrix : mat4x4<f32> ) -> vec3<f32> {

	


	return ( normalize( refract( ( - v ), normalize( n ), ( 1.0 / ior ) ) ) * ( vec3<f32>( thickness ) * vec3<f32>( length( modelMatrix[ 0u ].xyz ), length( modelMatrix[ 1u ].xyz ), length( modelMatrix[ 2u ].xyz ) ) ) );

}


fn applyIorToRoughness ( roughness : f32, ior : f32 ) -> f32 {

	


	return ( roughness * clamp( ( ( ior * 2.0 ) - 2.0 ), 0.0, 1.0 ) );

}


fn volumeAttenuation ( transmissionDistance : f32, attenuationColor : vec3<f32>, attenuationDistance : f32 ) -> vec3<f32> {

	


	if ( ( attenuationDistance != 0.0 ) ) {

		return exp( ( ( - ( ( - log( attenuationColor ) ) / vec3<f32>( attenuationDistance ) ) ) * vec3<f32>( transmissionDistance ) ) );

	}


	return vec3<f32>( 1.0, 1.0, 1.0 );

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

	DiffuseColor = vec4<f32>( vec3<f32>( 1.0, 1.0, 1.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	Metalness = 0.0;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.02, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	IOR = 1.52;
	nodeVar1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar1 * nodeVar1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform3 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform3, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - 0.0 ) ) );
	Transmission = 1.0;
	Thickness = 0.8;
	AttenuationDistance = 0.75;
	AttenuationColor = vec3<f32>( 0.85, 0.95, 1.0 );
	Dispersion = 0.35;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	nodeVar2 = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
	nodeVar3 = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var i : i32 = 0; i < 3; i ++ ) {

		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( ( ( f32( isFront ) * 2.0 ) - 1.0 ) ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		nodeVar4 = ( render.cameraPosition - v_positionWorld );
		nodeVar5 = normalize( nodeVar4 );
		nodeVar6 = ( ( IOR - 1.0 ) * ( Dispersion * 0.025 ) );
		nodeVar7 = getVolumeTransmissionRay( normalWorld, nodeVar5, Thickness, vec3<f32>( ( IOR - nodeVar6 ), IOR, ( IOR + nodeVar6 ) )[ i ], object.nodeUniform10 );
		nodeVar8 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeVar7 ), 1.0 ) ) );
		nodeVar9 = ( nodeVar8.xy / vec2<f32>( nodeVar8.w ) );
		nodeVar9 = ( nodeVar9 + vec2<f32>( 1.0 ) );
		nodeVar9 = ( nodeVar9 / vec2<f32>( 2.0 ) );
		nodeVar9 = vec2<f32>( nodeVar9.x, ( 1.0 - nodeVar9.y ) );
		let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform11.x, render.nodeUniform11.y );
		nodeVar10 = ( ( ( nodeVar9 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform11 );
		nodeVar11 = ( log2( cameraViewport.z ) * applyIorToRoughness( Roughness, vec3<f32>( ( IOR - nodeVar6 ), IOR, ( IOR + nodeVar6 ) )[ i ] ) );
		nodeVar12 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform12, i32( nodeVar11 ) ) ) ), vec2<f32>( textureDimensions( nodeUniform12, i32( nodeVar11 ) ) ) );
		nodeVar13 = ( ( nodeVar10 * nodeVar12.zw ) + vec2<f32>( 0.5 ) );
		nodeVar14 = fract( nodeVar13 );
		nodeVar15 = ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( ( nodeVar14.x * ( ( - nodeVar14.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar14.x * ( nodeVar14.x * ( ( 3.0 * nodeVar14.x ) - 6.0 ) ) ) + 4.0 ) ) );
		nodeVar16 = floor( nodeVar13 );
		nodeVar17 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( nodeVar14.x * ( ( 3.0 * nodeVar14.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( ( nodeVar14.x * ( ( - nodeVar14.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar14.x * ( nodeVar14.x * ( ( 3.0 * nodeVar14.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
		nodeVar18 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( nodeVar14.y * ( ( 3.0 * nodeVar14.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( ( nodeVar14.y * ( ( - nodeVar14.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar14.y * ( nodeVar14.y * ( ( 3.0 * nodeVar14.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
		nodeVar19 = floor( nodeVar11 );
		nodeVar20 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, ( ( vec2<f32>( ( nodeVar16.x + nodeVar17 ), ( nodeVar16.y + nodeVar18 ) ) - vec2<f32>( 0.5 ) ) * nodeVar12.xy ), nodeVar19 );
		nodeVar21 = ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( ( nodeVar14.x * ( ( -3.0 * nodeVar14.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar14.x, 3.0 ) ) );
		nodeVar22 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar14.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( ( nodeVar14.x * ( ( -3.0 * nodeVar14.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar14.x, 3.0 ) ) ) ) );
		nodeVar23 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, ( ( vec2<f32>( ( nodeVar16.x + nodeVar22 ), ( nodeVar16.y + nodeVar18 ) ) - vec2<f32>( 0.5 ) ) * nodeVar12.xy ), nodeVar19 );
		nodeVar24 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar14.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( ( nodeVar14.y * ( ( -3.0 * nodeVar14.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar14.y, 3.0 ) ) ) ) );
		nodeVar25 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, ( ( vec2<f32>( ( nodeVar16.x + nodeVar17 ), ( nodeVar16.y + nodeVar24 ) ) - vec2<f32>( 0.5 ) ) * nodeVar12.xy ), nodeVar19 );
		nodeVar26 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, ( ( vec2<f32>( ( nodeVar16.x + nodeVar22 ), ( nodeVar16.y + nodeVar24 ) ) - vec2<f32>( 0.5 ) ) * nodeVar12.xy ), nodeVar19 );
		nodeVar27 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform12, i32( ( nodeVar11 + 1.0 ) ) ) ) ), vec2<f32>( textureDimensions( nodeUniform12, i32( ( nodeVar11 + 1.0 ) ) ) ) );
		nodeVar28 = ( ( nodeVar10 * nodeVar27.zw ) + vec2<f32>( 0.5 ) );
		nodeVar29 = fract( nodeVar28 );
		nodeVar30 = ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( - nodeVar29.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.x * ( nodeVar29.x * ( ( 3.0 * nodeVar29.x ) - 6.0 ) ) ) + 4.0 ) ) );
		nodeVar31 = floor( nodeVar28 );
		nodeVar32 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( nodeVar29.x * ( ( 3.0 * nodeVar29.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( - nodeVar29.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.x * ( nodeVar29.x * ( ( 3.0 * nodeVar29.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
		nodeVar33 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( nodeVar29.y * ( ( 3.0 * nodeVar29.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( - nodeVar29.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.y * ( nodeVar29.y * ( ( 3.0 * nodeVar29.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
		nodeVar34 = ceil( nodeVar11 );
		nodeVar35 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar32 ), ( nodeVar31.y + nodeVar33 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
		nodeVar36 = ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( -3.0 * nodeVar29.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.x, 3.0 ) ) );
		nodeVar37 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar29.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( -3.0 * nodeVar29.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.x, 3.0 ) ) ) ) );
		nodeVar38 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar37 ), ( nodeVar31.y + nodeVar33 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
		nodeVar39 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar29.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( -3.0 * nodeVar29.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.y, 3.0 ) ) ) ) );
		nodeVar40 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar32 ), ( nodeVar31.y + nodeVar39 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
		nodeVar41 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar37 ), ( nodeVar31.y + nodeVar39 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
		nodeVar42 = mix( ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( ( nodeVar14.y * ( ( - nodeVar14.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar14.y * ( nodeVar14.y * ( ( 3.0 * nodeVar14.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar15 ) * nodeVar20 ) + ( vec4<f32>( nodeVar21 ) * nodeVar23 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( ( nodeVar14.y * ( ( -3.0 * nodeVar14.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar14.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar15 ) * nodeVar25 ) + ( vec4<f32>( nodeVar21 ) * nodeVar26 ) ) ) ), ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( - nodeVar29.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.y * ( nodeVar29.y * ( ( 3.0 * nodeVar29.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar30 ) * nodeVar35 ) + ( vec4<f32>( nodeVar36 ) * nodeVar38 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( -3.0 * nodeVar29.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar30 ) * nodeVar40 ) + ( vec4<f32>( nodeVar36 ) * nodeVar41 ) ) ) ), fract( nodeVar11 ) );
		nodeVar2[ i ] = nodeVar42[ i ];
		nodeVar2.w = ( nodeVar2.w + nodeVar42.w );
		nodeVar3[ i ] = ( DiffuseContribution[ i ] * volumeAttenuation( length( nodeVar7 ), AttenuationColor, AttenuationDistance )[ i ] );

	}

	nodeVar2.w = ( nodeVar2.w / 3.0 );
	nodeVar43 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeVar5 ), 0.0, 1.0 ) ) ).xy;
	nodeVar44 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar43.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar43.y ) ) ) ) * ( nodeVar3 * nodeVar2.xyz ) ), ( 1.0 - ( ( 1.0 - nodeVar2.w ) * ( ( ( nodeVar3.x + nodeVar3.y ) + nodeVar3.z ) / 3.0 ) ) ) );
	nodeVar45 = mix( 1.0, nodeVar44.w, Transmission );
	nodeVar46 = ( DiffuseColor.w * nodeVar45 );
	DiffuseColor.w = nodeVar46;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar47 = dot( normalView, positionViewDirection );
	nodeVar48 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( nodeVar47, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar48;
	nodeVar49 = ( dfg.x + dfg.y );
	nodeVar50 = ( 1.0 / nodeVar49 );
	nodeVar51 = nodeVar50;
	nodeVar52 = ( nodeVar51 - 1.0 );
	nodeVar53 = ( SpecularColorBlended * vec3<f32>( nodeVar52 ) );
	nodeVar54 = ( nodeVar53 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar54;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform14 );
	nodeVar56 = floor( nodeVar55 );
	nodeVar57 = nodeVar56;
	nodeVar58 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar59 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar58.x, ( - nodeVar58.y ), nodeVar58.z ), 1.0 ) ).xyz );
	nodeVar60 = max( ( 4.0 - nodeVar57 ), 0.0 );
	nodeVar57 = max( nodeVar57, 4.0 );
	nodeVar61 = exp2( nodeVar57 );
	nodeVar62 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar58.x, ( - nodeVar58.y ), nodeVar58.z ), 1.0 ) ).xyz, nodeVar59 ) * vec2<f32>( ( nodeVar61 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

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
	nodeVar65 = fract( nodeVar55 );

	if ( ( nodeVar65 != 0.0 ) ) {

		nodeVar66 = ( nodeVar56 + 1.0 );
		nodeVar67 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar58.x, ( - nodeVar58.y ), nodeVar58.z ), 1.0 ) ).xyz );
		nodeVar68 = max( ( 4.0 - nodeVar66 ), 0.0 );
		nodeVar66 = max( nodeVar66, 4.0 );
		nodeVar69 = exp2( nodeVar66 );
		nodeVar70 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( nodeVar58.x, ( - nodeVar58.y ), nodeVar58.z ), 1.0 ) ).xyz, nodeVar67 ) * vec2<f32>( ( nodeVar69 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

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

	nodeVar73 = ( radiance + ( nodeVar64 * vec3<f32>( object.nodeUniform20 ) ) );
	radiance = nodeVar73;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar74 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform14 );
	nodeVar75 = floor( nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar78 = max( ( 4.0 - nodeVar76 ), 0.0 );
	nodeVar76 = max( nodeVar76, 4.0 );
	nodeVar79 = exp2( nodeVar76 );
	nodeVar80 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar77 ) * vec2<f32>( ( nodeVar79 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar77 > 2.0 ) ) {

		nodeVar80.y = ( nodeVar80.y + nodeVar79 );
		nodeVar77 = ( nodeVar77 - 3.0 );
		

	}

	nodeVar80.x = ( nodeVar80.x + ( nodeVar77 * nodeVar79 ) );
	nodeVar80.x = ( nodeVar80.x + ( nodeVar78 * ( 3.0 * 16.0 ) ) );
	nodeVar80.y = ( nodeVar80.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar79 ) ) );
	nodeVar80.x = ( nodeVar80.x * object.nodeUniform17 );
	nodeVar80.y = ( nodeVar80.y * object.nodeUniform18 );
	nodeVar81 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar80, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar82 = nodeVar81.xyz;
	nodeVar83 = fract( nodeVar74 );

	if ( ( nodeVar83 != 0.0 ) ) {

		nodeVar84 = ( nodeVar75 + 1.0 );
		nodeVar85 = getFace( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar86 = max( ( 4.0 - nodeVar84 ), 0.0 );
		nodeVar84 = max( nodeVar84, 4.0 );
		nodeVar87 = exp2( nodeVar84 );
		nodeVar88 = ( ( getUV( ( object.nodeUniform15 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar85 ) * vec2<f32>( ( nodeVar87 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar85 > 2.0 ) ) {

			nodeVar88.y = ( nodeVar88.y + nodeVar87 );
			nodeVar85 = ( nodeVar85 - 3.0 );
			

		}

		nodeVar88.x = ( nodeVar88.x + ( nodeVar85 * nodeVar87 ) );
		nodeVar88.x = ( nodeVar88.x + ( nodeVar86 * ( 3.0 * 16.0 ) ) );
		nodeVar88.y = ( nodeVar88.y + ( 4.0 * ( exp2( object.nodeUniform14 ) - nodeVar87 ) ) );
		nodeVar88.x = ( nodeVar88.x * object.nodeUniform17 );
		nodeVar88.y = ( nodeVar88.y * object.nodeUniform18 );
		nodeVar89 = textureSampleGrad( nodeUniform19, nodeUniform19_sampler, nodeVar88, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar90 = nodeVar89.xyz;
		nodeVar82 = mix( nodeVar82, nodeVar90, nodeVar83 );
		

	}

	nodeVar91 = ( iblIrradiance + ( ( nodeVar82 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform20 ) ) );
	iblIrradiance = nodeVar91;
	nodeVar92 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar93 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar95 = ( SpecularF90 * dfg.y );
	nodeVar96 = ( nodeVar94 + vec3<f32>( nodeVar95 ) );
	nodeVar97 = ( nodeVar92 + nodeVar96 );
	nodeVar92 = nodeVar97;
	nodeVar98 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar99 = nodeVar98;
	nodeVar100 = ( nodeVar99 * vec3<f32>( 0.047619 ) );
	nodeVar101 = ( SpecularColor + nodeVar100 );
	nodeVar102 = ( nodeVar96 * nodeVar101 );
	nodeVar103 = ( dfg.x + dfg.y );
	nodeVar104 = ( 1.0 - nodeVar103 );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( vec3<f32>( nodeVar105 ) * nodeVar101 );
	nodeVar107 = ( vec3<f32>( 1.0 ) - nodeVar106 );
	nodeVar108 = nodeVar107;
	nodeVar109 = ( nodeVar102 / nodeVar108 );
	nodeVar110 = ( nodeVar109 * vec3<f32>( nodeVar105 ) );
	nodeVar111 = ( nodeVar93 + nodeVar110 );
	nodeVar93 = nodeVar111;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar112 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar113 = ( irradiance * nodeVar112 );
	nodeVar114 = ( nodeVar92 + nodeVar93 );
	nodeVar115 = ( vec3<f32>( 1.0 ) - nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( nodeVar113 * nodeVar116 );
	nodeVar118 = nodeVar117;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar119 = ( indirectDiffuse + nodeVar118 );
	indirectDiffuse = nodeVar119;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar120 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar121 = ( SpecularF90 * dfg.y );
	nodeVar122 = ( nodeVar120 + vec3<f32>( nodeVar121 ) );
	nodeVar123 = ( singleScatteringDielectric + nodeVar122 );
	singleScatteringDielectric = nodeVar123;
	nodeVar124 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( nodeVar125 * vec3<f32>( 0.047619 ) );
	nodeVar127 = ( SpecularColor + nodeVar126 );
	nodeVar128 = ( nodeVar122 * nodeVar127 );
	nodeVar129 = ( dfg.x + dfg.y );
	nodeVar130 = ( 1.0 - nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( vec3<f32>( nodeVar131 ) * nodeVar127 );
	nodeVar133 = ( vec3<f32>( 1.0 ) - nodeVar132 );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( nodeVar128 / nodeVar134 );
	nodeVar136 = ( nodeVar135 * vec3<f32>( nodeVar131 ) );
	nodeVar137 = ( multiScatteringDielectric + nodeVar136 );
	multiScatteringDielectric = nodeVar137;
	nodeVar138 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar139 = ( SpecularF90 * dfg.y );
	nodeVar140 = ( nodeVar138 + vec3<f32>( nodeVar139 ) );
	nodeVar141 = ( singleScatteringMetallic + nodeVar140 );
	singleScatteringMetallic = nodeVar141;
	nodeVar142 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar143 = nodeVar142;
	nodeVar144 = ( nodeVar143 * vec3<f32>( 0.047619 ) );
	nodeVar145 = ( DiffuseColor.xyz + nodeVar144 );
	nodeVar146 = ( nodeVar140 * nodeVar145 );
	nodeVar147 = ( dfg.x + dfg.y );
	nodeVar148 = ( 1.0 - nodeVar147 );
	nodeVar149 = nodeVar148;
	nodeVar150 = ( vec3<f32>( nodeVar149 ) * nodeVar145 );
	nodeVar151 = ( vec3<f32>( 1.0 ) - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( nodeVar146 / nodeVar152 );
	nodeVar154 = ( nodeVar153 * vec3<f32>( nodeVar149 ) );
	nodeVar155 = ( multiScatteringMetallic + nodeVar154 );
	multiScatteringMetallic = nodeVar155;
	nodeVar156 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar157 = ( radiance * nodeVar156 );
	nodeVar158 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar159 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar160 = ( nodeVar158 * nodeVar159 );
	nodeVar161 = ( nodeVar157 + nodeVar160 );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar164 = ( vec3<f32>( 1.0 ) - nodeVar163 );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( DiffuseContribution * nodeVar165 );
	nodeVar167 = ( nodeVar166 * nodeVar159 );
	nodeVar168 = nodeVar167;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar169 = ( indirectSpecular + nodeVar162 );
	indirectSpecular = nodeVar169;
	nodeVar170 = ( indirectDiffuse + nodeVar168 );
	indirectDiffuse = nodeVar170;
	ambientOcclusion = 1.0;
	nodeVar171 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar171;
	nodeVar172 = dot( normalView, positionViewDirection );
	nodeVar173 = ( clamp( nodeVar172, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar174 = ( Roughness * -16.0 );
	nodeVar175 = ( 1.0 - nodeVar174 );
	nodeVar176 = nodeVar175;
	nodeVar177 = ( - nodeVar176 );
	nodeVar178 = exp2( nodeVar177 );
	nodeVar179 = pow( nodeVar173, nodeVar178 );
	nodeVar180 = ( 1.0 - nodeVar179 );
	nodeVar181 = nodeVar180;
	nodeVar182 = ( ambientOcclusion - nodeVar181 );
	nodeVar183 = ( indirectSpecular * vec3<f32>( clamp( nodeVar182, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar183;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar184 = ( directDiffuse + indirectDiffuse );
	nodeVar185 = mix( vec4<f32>( nodeVar184, 1.0 ), nodeVar44, Transmission );
	totalDiffuse = nodeVar185.xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar186 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar186;
	nodeVar187 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar187;
	nodeVar188 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar188;

	// result

	output.color = nodeVar188;

	return output;

}
