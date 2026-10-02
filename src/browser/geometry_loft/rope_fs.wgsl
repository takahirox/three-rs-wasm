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
@binding( 3 ) @group( 1 ) var nodeUniform17_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform17 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform31_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform31 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform32 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform14 : vec4<f32>,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform20 : vec4<f32>,
	nodeUniform13 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : vec2<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : vec2<f32>,
	nodeUniform25 : f32,
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
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec2<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : f32;
var<private> shadowValue : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec2<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec2<f32>;
var<private> nodeVar85 : vec4<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : vec2<f32>;
var<private> nodeVar93 : vec4<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : vec2<f32>;
var<private> nodeVar103 : vec4<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec2<f32>;
var<private> nodeVar111 : vec4<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : f32;
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
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : f32;
var<private> nodeVar203 : f32;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec4<f32>;

// codes
fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


fn V_GGX_SmithCorrelated ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn D_GGX ( alpha : f32, dotNH : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;

	nodeVar0 = ( alpha * alpha );
	nodeVar1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeVar0 ) ) );

	return ( ( nodeVar0 / ( nodeVar1 * nodeVar1 ) ) * 0.3183098861837907 );

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
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying7 : vec2<f32>,
	@builtin( front_facing ) isFront : bool,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	nodeVar1 = normalize( dpdx( v_positionView ) );
	NORMAL_normalView = normalViewGeometry;
	nodeVar2 = cross( normalize( - dpdy( v_positionView ) ), NORMAL_normalView );
	nodeVar3 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	nodeVar4 = ( dot( nodeVar1, nodeVar2 ) * nodeVar3 );
	nodeVar5 = ( vec2<f32>( ( ( sin( ( ( nodeVarying7.x * 200.0 ) + ( nodeVarying7.y * 6.283185307179586 ) ) ) * 0.015 ) - ( sin( ( ( nodeVarying7.x * 200.0 ) + ( nodeVarying7.y * 6.283185307179586 ) ) ) * 0.015 ) ), ( ( sin( ( ( nodeVarying7.x * 200.0 ) + ( nodeVarying7.y * 6.283185307179586 ) ) ) * 0.015 ) - ( sin( ( ( nodeVarying7.x * 200.0 ) + ( nodeVarying7.y * 6.283185307179586 ) ) ) * 0.015 ) ) ) * vec2<f32>( 1.0 ) );
	normalView = normalize( ( ( vec3<f32>( abs( nodeVar4 ) ) * NORMAL_normalView ) - ( vec3<f32>( sign( nodeVar4 ) ) * ( ( vec3<f32>( nodeVar5.x ) * nodeVar2 ) + ( vec3<f32>( nodeVar5.y ) * cross( NORMAL_normalView, nodeVar1 ) ) ) ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = dot( normalView, positionViewDirection );
	nodeVar7 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar6, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar7;
	nodeVar8 = ( dfg.x + dfg.y );
	nodeVar9 = ( 1.0 / nodeVar8 );
	nodeVar10 = nodeVar9;
	nodeVar11 = ( nodeVar10 - 1.0 );
	nodeVar12 = ( SpecularColorBlended * vec3<f32>( nodeVar11 ) );
	nodeVar13 = ( nodeVar12 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar13;
	nodeVar14 = vec4<f32>( render.nodeUniform11, 0.0 );
	nodeVar15 = ( render.cameraViewMatrix * nodeVar14 );
	nodeVar16 = normalize( nodeVar15.xyz );
	nodeVar17 = nodeVar16;
	nodeVar18 = dot( normalView, nodeVar17 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar19 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform13 ) ) ), 1.0 );
	nodeVar20 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar20 >= render.nodeUniform14.x ) && ( nodeVar20 < render.nodeUniform14.y ) ) ) {

		nodeVar22 = ( render.nodeUniform15 * nodeVar19 );
		nodeVar23 = ( nodeVar22.xyz / vec3<f32>( nodeVar22.w ) );
		nodeVar24 = vec3<f32>( nodeVar23.x, ( 1.0 - nodeVar23.y ), ( nodeVar23.z + render.nodeUniform16 ) );

		if ( ( ( ( ( ( nodeVar24.x >= 0.0 ) && ( nodeVar24.x <= 1.0 ) ) && ( nodeVar24.y >= 0.0 ) ) && ( nodeVar24.y <= 1.0 ) ) && ( nodeVar24.z <= 1.0 ) ) ) {

			nodeVar25 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar26 = ( render.nodeUniform18 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ).x );
			nodeVar27 = ( nodeVar24.xy + ( vogelDiskSample( 0, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar28 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar27, nodeVar24.z );
			nodeVar29 = ( nodeVar24.xy + ( vogelDiskSample( 1, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar30 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar29, nodeVar24.z );
			nodeVar31 = ( nodeVar24.xy + ( vogelDiskSample( 2, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar32 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar31, nodeVar24.z );
			nodeVar33 = ( nodeVar24.xy + ( vogelDiskSample( 3, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar34 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar33, nodeVar24.z );
			nodeVar35 = ( nodeVar24.xy + ( vogelDiskSample( 4, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar36 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar35, nodeVar24.z );
			nodeVar21 = ( ( ( ( ( nodeVar28 + nodeVar30 ) + nodeVar32 ) + nodeVar34 ) + nodeVar36 ) * 0.2 );

		} else {

			nodeVar21 = 1.0;

		}

		shadowValue = mix( nodeVar21, shadowValue, smoothstep( render.nodeUniform14.z, render.nodeUniform14.y, nodeVar20 ) );
		

	}


	if ( ( ( nodeVar20 >= render.nodeUniform20.x ) && ( nodeVar20 < render.nodeUniform20.y ) ) ) {

		nodeVar38 = ( render.nodeUniform21 * nodeVar19 );
		nodeVar39 = ( nodeVar38.xyz / vec3<f32>( nodeVar38.w ) );
		nodeVar40 = vec3<f32>( nodeVar39.x, ( 1.0 - nodeVar39.y ), ( nodeVar39.z + render.nodeUniform22 ) );

		if ( ( ( ( ( ( nodeVar40.x >= 0.0 ) && ( nodeVar40.x <= 1.0 ) ) && ( nodeVar40.y >= 0.0 ) ) && ( nodeVar40.y <= 1.0 ) ) && ( nodeVar40.z <= 1.0 ) ) ) {

			nodeVar41 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar42 = ( render.nodeUniform23 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform24 ).x );
			nodeVar43 = ( nodeVar40.xy + ( vogelDiskSample( 0, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar44 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar43, nodeVar40.z );
			nodeVar45 = ( nodeVar40.xy + ( vogelDiskSample( 1, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar46 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar45, nodeVar40.z );
			nodeVar47 = ( nodeVar40.xy + ( vogelDiskSample( 2, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar48 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar47, nodeVar40.z );
			nodeVar49 = ( nodeVar40.xy + ( vogelDiskSample( 3, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar50 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar49, nodeVar40.z );
			nodeVar51 = ( nodeVar40.xy + ( vogelDiskSample( 4, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar52 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar51, nodeVar40.z );
			nodeVar37 = ( ( ( ( ( nodeVar44 + nodeVar46 ) + nodeVar48 ) + nodeVar50 ) + nodeVar52 ) * 0.2 );

		} else {

			nodeVar37 = 1.0;

		}

		shadowValue = mix( nodeVar37, shadowValue, smoothstep( render.nodeUniform20.z, render.nodeUniform20.y, nodeVar20 ) );
		

	}

	nodeVar53 = mix( 1.0, shadowValue, render.nodeUniform25 );
	nodeVar54 = ( render.nodeUniform12 * vec3<f32>( nodeVar53 ) );
	nodeVar55 = ( vec3<f32>( clamp( nodeVar18, 0.0, 1.0 ) ) * nodeVar54 );
	nodeVar56 = nodeVar55;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar57 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar58 = ( nodeVar56 * nodeVar57 );
	nodeVar59 = ( nodeVar17 + positionViewDirection );
	nodeVar60 = normalize( nodeVar59 );
	nodeVar61 = dot( positionViewDirection, nodeVar60 );
	nodeVar62 = clamp( nodeVar61, 0.0, 1.0 );
	nodeVar63 = exp2( ( ( ( nodeVar62 * -5.55473 ) - 6.98316 ) * nodeVar62 ) );
	nodeVar64 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar63 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar63 ) ) );
	nodeVar65 = ( vec3<f32>( 1.0 ) - nodeVar64 );
	nodeVar66 = nodeVar65;
	nodeVar67 = ( nodeVar58 * nodeVar66 );
	nodeVar68 = ( directDiffuse + nodeVar67 );
	directDiffuse = nodeVar68;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = normalize( ( nodeVar17 + positionViewDirection ) );
	nodeVar70 = clamp( dot( positionViewDirection, nodeVar69 ), 0.0, 1.0 );
	nodeVar71 = exp2( ( ( ( nodeVar70 * -5.55473 ) - 6.98316 ) * nodeVar70 ) );
	nodeVar72 = ( Roughness * Roughness );
	nodeVar73 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar71 ) ) ) + vec3<f32>( ( 1.0 * nodeVar71 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar72, clamp( dot( normalView, nodeVar17 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar72, clamp( dot( normalView, nodeVar69 ), 0.0, 1.0 ) ) ) );
	nodeVar74 = ( nodeVar56 * nodeVar73 );
	nodeVar75 = ( nodeVar74 * multiScatteringCompensation );
	nodeVar76 = ( directSpecular + nodeVar75 );
	directSpecular = nodeVar76;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar77 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform26 );
	nodeVar78 = floor( nodeVar77 );
	nodeVar79 = nodeVar78;
	nodeVar80 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar81 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar80.x, ( - nodeVar80.y ), nodeVar80.z ), 1.0 ) ).xyz );
	nodeVar82 = max( ( 4.0 - nodeVar79 ), 0.0 );
	nodeVar79 = max( nodeVar79, 4.0 );
	nodeVar83 = exp2( nodeVar79 );
	nodeVar84 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar80.x, ( - nodeVar80.y ), nodeVar80.z ), 1.0 ) ).xyz, nodeVar81 ) * vec2<f32>( ( nodeVar83 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar81 > 2.0 ) ) {

		nodeVar84.y = ( nodeVar84.y + nodeVar83 );
		nodeVar81 = ( nodeVar81 - 3.0 );
		

	}

	nodeVar84.x = ( nodeVar84.x + ( nodeVar81 * nodeVar83 ) );
	nodeVar84.x = ( nodeVar84.x + ( nodeVar82 * ( 3.0 * 16.0 ) ) );
	nodeVar84.y = ( nodeVar84.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar83 ) ) );
	nodeVar84.x = ( nodeVar84.x * object.nodeUniform29 );
	nodeVar84.y = ( nodeVar84.y * object.nodeUniform30 );
	nodeVar85 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar84, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar86 = nodeVar85.xyz;
	nodeVar87 = fract( nodeVar77 );

	if ( ( nodeVar87 != 0.0 ) ) {

		nodeVar88 = ( nodeVar78 + 1.0 );
		nodeVar89 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar80.x, ( - nodeVar80.y ), nodeVar80.z ), 1.0 ) ).xyz );
		nodeVar90 = max( ( 4.0 - nodeVar88 ), 0.0 );
		nodeVar88 = max( nodeVar88, 4.0 );
		nodeVar91 = exp2( nodeVar88 );
		nodeVar92 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar80.x, ( - nodeVar80.y ), nodeVar80.z ), 1.0 ) ).xyz, nodeVar89 ) * vec2<f32>( ( nodeVar91 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar89 > 2.0 ) ) {

			nodeVar92.y = ( nodeVar92.y + nodeVar91 );
			nodeVar89 = ( nodeVar89 - 3.0 );
			

		}

		nodeVar92.x = ( nodeVar92.x + ( nodeVar89 * nodeVar91 ) );
		nodeVar92.x = ( nodeVar92.x + ( nodeVar90 * ( 3.0 * 16.0 ) ) );
		nodeVar92.y = ( nodeVar92.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar91 ) ) );
		nodeVar92.x = ( nodeVar92.x * object.nodeUniform29 );
		nodeVar92.y = ( nodeVar92.y * object.nodeUniform30 );
		nodeVar93 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar92, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar94 = nodeVar93.xyz;
		nodeVar86 = mix( nodeVar86, nodeVar94, nodeVar87 );
		

	}

	nodeVar95 = ( radiance + ( nodeVar86 * vec3<f32>( object.nodeUniform32 ) ) );
	radiance = nodeVar95;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar96 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform26 );
	nodeVar97 = floor( nodeVar96 );
	nodeVar98 = nodeVar97;
	nodeVar99 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar100 = max( ( 4.0 - nodeVar98 ), 0.0 );
	nodeVar98 = max( nodeVar98, 4.0 );
	nodeVar101 = exp2( nodeVar98 );
	nodeVar102 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar99 ) * vec2<f32>( ( nodeVar101 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar99 > 2.0 ) ) {

		nodeVar102.y = ( nodeVar102.y + nodeVar101 );
		nodeVar99 = ( nodeVar99 - 3.0 );
		

	}

	nodeVar102.x = ( nodeVar102.x + ( nodeVar99 * nodeVar101 ) );
	nodeVar102.x = ( nodeVar102.x + ( nodeVar100 * ( 3.0 * 16.0 ) ) );
	nodeVar102.y = ( nodeVar102.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar101 ) ) );
	nodeVar102.x = ( nodeVar102.x * object.nodeUniform29 );
	nodeVar102.y = ( nodeVar102.y * object.nodeUniform30 );
	nodeVar103 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar102, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar104 = nodeVar103.xyz;
	nodeVar105 = fract( nodeVar96 );

	if ( ( nodeVar105 != 0.0 ) ) {

		nodeVar106 = ( nodeVar97 + 1.0 );
		nodeVar107 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar108 = max( ( 4.0 - nodeVar106 ), 0.0 );
		nodeVar106 = max( nodeVar106, 4.0 );
		nodeVar109 = exp2( nodeVar106 );
		nodeVar110 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar107 ) * vec2<f32>( ( nodeVar109 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar107 > 2.0 ) ) {

			nodeVar110.y = ( nodeVar110.y + nodeVar109 );
			nodeVar107 = ( nodeVar107 - 3.0 );
			

		}

		nodeVar110.x = ( nodeVar110.x + ( nodeVar107 * nodeVar109 ) );
		nodeVar110.x = ( nodeVar110.x + ( nodeVar108 * ( 3.0 * 16.0 ) ) );
		nodeVar110.y = ( nodeVar110.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar109 ) ) );
		nodeVar110.x = ( nodeVar110.x * object.nodeUniform29 );
		nodeVar110.y = ( nodeVar110.y * object.nodeUniform30 );
		nodeVar111 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar110, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar112 = nodeVar111.xyz;
		nodeVar104 = mix( nodeVar104, nodeVar112, nodeVar105 );
		

	}

	nodeVar113 = ( iblIrradiance + ( ( nodeVar104 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform32 ) ) );
	iblIrradiance = nodeVar113;
	nodeVar114 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar115 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar116 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar117 = ( SpecularF90 * dfg.y );
	nodeVar118 = ( nodeVar116 + vec3<f32>( nodeVar117 ) );
	nodeVar119 = ( nodeVar114 + nodeVar118 );
	nodeVar114 = nodeVar119;
	nodeVar120 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar121 = nodeVar120;
	nodeVar122 = ( nodeVar121 * vec3<f32>( 0.047619 ) );
	nodeVar123 = ( SpecularColor + nodeVar122 );
	nodeVar124 = ( nodeVar118 * nodeVar123 );
	nodeVar125 = ( dfg.x + dfg.y );
	nodeVar126 = ( 1.0 - nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( vec3<f32>( nodeVar127 ) * nodeVar123 );
	nodeVar129 = ( vec3<f32>( 1.0 ) - nodeVar128 );
	nodeVar130 = nodeVar129;
	nodeVar131 = ( nodeVar124 / nodeVar130 );
	nodeVar132 = ( nodeVar131 * vec3<f32>( nodeVar127 ) );
	nodeVar133 = ( nodeVar115 + nodeVar132 );
	nodeVar115 = nodeVar133;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar134 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar135 = ( irradiance * nodeVar134 );
	nodeVar136 = ( nodeVar114 + nodeVar115 );
	nodeVar137 = ( vec3<f32>( 1.0 ) - nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( nodeVar135 * nodeVar138 );
	nodeVar140 = nodeVar139;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar141 = ( indirectDiffuse + nodeVar140 );
	indirectDiffuse = nodeVar141;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar142 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar143 = ( SpecularF90 * dfg.y );
	nodeVar144 = ( nodeVar142 + vec3<f32>( nodeVar143 ) );
	nodeVar145 = ( singleScatteringDielectric + nodeVar144 );
	singleScatteringDielectric = nodeVar145;
	nodeVar146 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( nodeVar147 * vec3<f32>( 0.047619 ) );
	nodeVar149 = ( SpecularColor + nodeVar148 );
	nodeVar150 = ( nodeVar144 * nodeVar149 );
	nodeVar151 = ( dfg.x + dfg.y );
	nodeVar152 = ( 1.0 - nodeVar151 );
	nodeVar153 = nodeVar152;
	nodeVar154 = ( vec3<f32>( nodeVar153 ) * nodeVar149 );
	nodeVar155 = ( vec3<f32>( 1.0 ) - nodeVar154 );
	nodeVar156 = nodeVar155;
	nodeVar157 = ( nodeVar150 / nodeVar156 );
	nodeVar158 = ( nodeVar157 * vec3<f32>( nodeVar153 ) );
	nodeVar159 = ( multiScatteringDielectric + nodeVar158 );
	multiScatteringDielectric = nodeVar159;
	nodeVar160 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar161 = ( SpecularF90 * dfg.y );
	nodeVar162 = ( nodeVar160 + vec3<f32>( nodeVar161 ) );
	nodeVar163 = ( singleScatteringMetallic + nodeVar162 );
	singleScatteringMetallic = nodeVar163;
	nodeVar164 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( nodeVar165 * vec3<f32>( 0.047619 ) );
	nodeVar167 = ( DiffuseColor.xyz + nodeVar166 );
	nodeVar168 = ( nodeVar162 * nodeVar167 );
	nodeVar169 = ( dfg.x + dfg.y );
	nodeVar170 = ( 1.0 - nodeVar169 );
	nodeVar171 = nodeVar170;
	nodeVar172 = ( vec3<f32>( nodeVar171 ) * nodeVar167 );
	nodeVar173 = ( vec3<f32>( 1.0 ) - nodeVar172 );
	nodeVar174 = nodeVar173;
	nodeVar175 = ( nodeVar168 / nodeVar174 );
	nodeVar176 = ( nodeVar175 * vec3<f32>( nodeVar171 ) );
	nodeVar177 = ( multiScatteringMetallic + nodeVar176 );
	multiScatteringMetallic = nodeVar177;
	nodeVar178 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar179 = ( radiance * nodeVar178 );
	nodeVar180 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar181 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar182 = ( nodeVar180 * nodeVar181 );
	nodeVar183 = ( nodeVar179 + nodeVar182 );
	nodeVar184 = nodeVar183;
	nodeVar185 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar186 = ( vec3<f32>( 1.0 ) - nodeVar185 );
	nodeVar187 = nodeVar186;
	nodeVar188 = ( DiffuseContribution * nodeVar187 );
	nodeVar189 = ( nodeVar188 * nodeVar181 );
	nodeVar190 = nodeVar189;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar191 = ( indirectSpecular + nodeVar184 );
	indirectSpecular = nodeVar191;
	nodeVar192 = ( indirectDiffuse + nodeVar190 );
	indirectDiffuse = nodeVar192;
	ambientOcclusion = 1.0;
	nodeVar193 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar193;
	nodeVar194 = dot( normalView, positionViewDirection );
	nodeVar195 = ( clamp( nodeVar194, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar196 = ( Roughness * -16.0 );
	nodeVar197 = ( 1.0 - nodeVar196 );
	nodeVar198 = nodeVar197;
	nodeVar199 = ( - nodeVar198 );
	nodeVar200 = exp2( nodeVar199 );
	nodeVar201 = pow( nodeVar195, nodeVar200 );
	nodeVar202 = ( 1.0 - nodeVar201 );
	nodeVar203 = nodeVar202;
	nodeVar204 = ( ambientOcclusion - nodeVar203 );
	nodeVar205 = ( indirectSpecular * vec3<f32>( clamp( nodeVar204, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar205;
	nodeVar206 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar206;
	nodeVar207 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar207;
	nodeVar208 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar208;
	nodeVar209 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar209;

	// result

	output.color = nodeVar209;

	return output;

}
