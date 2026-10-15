// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	@location( 3 ) m3 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform22_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform22 : texture_depth_2d;
@binding( 7 ) @group( 1 ) var nodeUniform36_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform36 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : mat4x4<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform37 : f32,
	nodeUniform38 : mat4x4<f32>,
	nodeUniform39 : mat4x4<f32>,
	nodeUniform41 : mat4x4<f32>,
	nodeUniform42 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform40 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform19 : vec4<f32>,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform25 : vec4<f32>,
	nodeUniform18 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : vec2<f32>,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : vec2<f32>,
	nodeUniform30 : f32,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar0 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : f32;
var<private> shadowValue : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec2<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec2<f32>;
var<private> nodeVar83 : vec4<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec2<f32>;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec2<f32>;
var<private> nodeVar101 : vec4<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : vec2<f32>;
var<private> nodeVar109 : vec4<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar160 : f32;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
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
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : f32;
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
var<private> nodeVar204 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec4<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar209 : vec4<f32>;
var<private> nodeVar210 : vec4<f32>;
var<private> nodeVar211 : vec2<f32>;
var<private> nodeVar212 : vec4<f32>;

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
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) positionPrevious : vec3<f32>,
	@location( 6 ) nodeVarying8 : vec2<f32>,
	@builtin( front_facing ) isFront : bool,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	nodeVar0 = textureSample( nodeUniform2, nodeUniform2_sampler, ( object.nodeUniform3 * vec3<f32>( nodeVarying8, 1.0 ) ).xy );
	AmbientOcclusion = ( ( ( nodeVar0.x - 1.0 ) * object.nodeUniform4 ) + 1.0 );
	nodeVar1 = textureSample( nodeUniform2, nodeUniform2_sampler, ( object.nodeUniform6 * vec3<f32>( nodeVarying8, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform5 * nodeVar1.z );
	nodeVar2 = textureSample( nodeUniform2, nodeUniform2_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying8, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar3 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform7 * nodeVar2.y ), 0.0525 ) + max( max( nodeVar3.x, nodeVar3.y ), nodeVar3.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform5 * nodeVar1.z ) ) ) );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( ( ( f32( isFront ) * 2.0 ) - 1.0 ) ) );
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = dot( normalView, positionViewDirection );
	nodeVar5 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( nodeVar4, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar5;
	nodeVar6 = ( dfg.x + dfg.y );
	nodeVar7 = ( 1.0 / nodeVar6 );
	nodeVar8 = nodeVar7;
	nodeVar9 = ( nodeVar8 - 1.0 );
	nodeVar10 = ( SpecularColorBlended * vec3<f32>( nodeVar9 ) );
	nodeVar11 = ( nodeVar10 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar11;
	nodeVar12 = vec4<f32>( render.nodeUniform16, 0.0 );
	nodeVar13 = ( render.cameraViewMatrix * nodeVar12 );
	nodeVar14 = normalize( nodeVar13.xyz );
	nodeVar15 = nodeVar14;
	nodeVar16 = dot( normalView, nodeVar15 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar17 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform18 ) ) ), 1.0 );
	nodeVar18 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar18 >= render.nodeUniform19.x ) && ( nodeVar18 < render.nodeUniform19.y ) ) ) {

		nodeVar20 = ( render.nodeUniform20 * nodeVar17 );
		nodeVar21 = ( nodeVar20.xyz / vec3<f32>( nodeVar20.w ) );
		nodeVar22 = vec3<f32>( nodeVar21.x, ( 1.0 - nodeVar21.y ), ( nodeVar21.z + render.nodeUniform21 ) );

		if ( ( ( ( ( ( nodeVar22.x >= 0.0 ) && ( nodeVar22.x <= 1.0 ) ) && ( nodeVar22.y >= 0.0 ) ) && ( nodeVar22.y <= 1.0 ) ) && ( nodeVar22.z <= 1.0 ) ) ) {

			nodeVar23 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar24 = ( render.nodeUniform23 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform24 ).x );
			nodeVar25 = ( nodeVar22.xy + ( vogelDiskSample( 0, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar26 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar25, nodeVar22.z );
			nodeVar27 = ( nodeVar22.xy + ( vogelDiskSample( 1, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar28 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar27, nodeVar22.z );
			nodeVar29 = ( nodeVar22.xy + ( vogelDiskSample( 2, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar30 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar29, nodeVar22.z );
			nodeVar31 = ( nodeVar22.xy + ( vogelDiskSample( 3, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar32 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar31, nodeVar22.z );
			nodeVar33 = ( nodeVar22.xy + ( vogelDiskSample( 4, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar34 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar33, nodeVar22.z );
			nodeVar19 = ( ( ( ( ( nodeVar26 + nodeVar28 ) + nodeVar30 ) + nodeVar32 ) + nodeVar34 ) * 0.2 );

		} else {

			nodeVar19 = 1.0;

		}

		shadowValue = mix( nodeVar19, shadowValue, smoothstep( render.nodeUniform19.z, render.nodeUniform19.y, nodeVar18 ) );
		

	}


	if ( ( ( nodeVar18 >= render.nodeUniform25.x ) && ( nodeVar18 < render.nodeUniform25.y ) ) ) {

		nodeVar36 = ( render.nodeUniform26 * nodeVar17 );
		nodeVar37 = ( nodeVar36.xyz / vec3<f32>( nodeVar36.w ) );
		nodeVar38 = vec3<f32>( nodeVar37.x, ( 1.0 - nodeVar37.y ), ( nodeVar37.z + render.nodeUniform27 ) );

		if ( ( ( ( ( ( nodeVar38.x >= 0.0 ) && ( nodeVar38.x <= 1.0 ) ) && ( nodeVar38.y >= 0.0 ) ) && ( nodeVar38.y <= 1.0 ) ) && ( nodeVar38.z <= 1.0 ) ) ) {

			nodeVar39 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar40 = ( render.nodeUniform28 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform29 ).x );
			nodeVar41 = ( nodeVar38.xy + ( vogelDiskSample( 0, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar42 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar41, nodeVar38.z );
			nodeVar43 = ( nodeVar38.xy + ( vogelDiskSample( 1, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar44 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar43, nodeVar38.z );
			nodeVar45 = ( nodeVar38.xy + ( vogelDiskSample( 2, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar46 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar45, nodeVar38.z );
			nodeVar47 = ( nodeVar38.xy + ( vogelDiskSample( 3, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar48 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar47, nodeVar38.z );
			nodeVar49 = ( nodeVar38.xy + ( vogelDiskSample( 4, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar50 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeVar49, nodeVar38.z );
			nodeVar35 = ( ( ( ( ( nodeVar42 + nodeVar44 ) + nodeVar46 ) + nodeVar48 ) + nodeVar50 ) * 0.2 );

		} else {

			nodeVar35 = 1.0;

		}

		shadowValue = mix( nodeVar35, shadowValue, smoothstep( render.nodeUniform25.z, render.nodeUniform25.y, nodeVar18 ) );
		

	}

	nodeVar51 = mix( 1.0, shadowValue, render.nodeUniform30 );
	nodeVar52 = ( render.nodeUniform17 * vec3<f32>( nodeVar51 ) );
	nodeVar53 = ( vec3<f32>( clamp( nodeVar16, 0.0, 1.0 ) ) * nodeVar52 );
	nodeVar54 = nodeVar53;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar56 = ( nodeVar54 * nodeVar55 );
	nodeVar57 = ( nodeVar15 + positionViewDirection );
	nodeVar58 = normalize( nodeVar57 );
	nodeVar59 = dot( positionViewDirection, nodeVar58 );
	nodeVar60 = clamp( nodeVar59, 0.0, 1.0 );
	nodeVar61 = exp2( ( ( ( nodeVar60 * -5.55473 ) - 6.98316 ) * nodeVar60 ) );
	nodeVar62 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar61 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar61 ) ) );
	nodeVar63 = ( vec3<f32>( 1.0 ) - nodeVar62 );
	nodeVar64 = nodeVar63;
	nodeVar65 = ( nodeVar56 * nodeVar64 );
	nodeVar66 = ( directDiffuse + nodeVar65 );
	directDiffuse = nodeVar66;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar67 = normalize( ( nodeVar15 + positionViewDirection ) );
	nodeVar68 = clamp( dot( positionViewDirection, nodeVar67 ), 0.0, 1.0 );
	nodeVar69 = exp2( ( ( ( nodeVar68 * -5.55473 ) - 6.98316 ) * nodeVar68 ) );
	nodeVar70 = ( Roughness * Roughness );
	nodeVar71 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar69 ) ) ) + vec3<f32>( ( 1.0 * nodeVar69 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar70, clamp( dot( normalView, nodeVar15 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar70, clamp( dot( normalView, nodeVar67 ), 0.0, 1.0 ) ) ) );
	nodeVar72 = ( nodeVar54 * nodeVar71 );
	nodeVar73 = ( nodeVar72 * multiScatteringCompensation );
	nodeVar74 = ( directSpecular + nodeVar73 );
	directSpecular = nodeVar74;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar75 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform31 );
	nodeVar76 = floor( nodeVar75 );
	nodeVar77 = nodeVar76;
	nodeVar78 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar79 = getFace( ( object.nodeUniform32 * vec4<f32>( vec3<f32>( nodeVar78.x, ( - nodeVar78.y ), nodeVar78.z ), 1.0 ) ).xyz );
	nodeVar80 = max( ( 4.0 - nodeVar77 ), 0.0 );
	nodeVar77 = max( nodeVar77, 4.0 );
	nodeVar81 = exp2( nodeVar77 );
	nodeVar82 = ( ( getUV( ( object.nodeUniform32 * vec4<f32>( vec3<f32>( nodeVar78.x, ( - nodeVar78.y ), nodeVar78.z ), 1.0 ) ).xyz, nodeVar79 ) * vec2<f32>( ( nodeVar81 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar79 > 2.0 ) ) {

		nodeVar82.y = ( nodeVar82.y + nodeVar81 );
		nodeVar79 = ( nodeVar79 - 3.0 );
		

	}

	nodeVar82.x = ( nodeVar82.x + ( nodeVar79 * nodeVar81 ) );
	nodeVar82.x = ( nodeVar82.x + ( nodeVar80 * ( 3.0 * 16.0 ) ) );
	nodeVar82.y = ( nodeVar82.y + ( 4.0 * ( exp2( object.nodeUniform31 ) - nodeVar81 ) ) );
	nodeVar82.x = ( nodeVar82.x * object.nodeUniform34 );
	nodeVar82.y = ( nodeVar82.y * object.nodeUniform35 );
	nodeVar83 = textureSampleGrad( nodeUniform36, nodeUniform36_sampler, nodeVar82, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar84 = nodeVar83.xyz;
	nodeVar85 = fract( nodeVar75 );

	if ( ( nodeVar85 != 0.0 ) ) {

		nodeVar86 = ( nodeVar76 + 1.0 );
		nodeVar87 = getFace( ( object.nodeUniform32 * vec4<f32>( vec3<f32>( nodeVar78.x, ( - nodeVar78.y ), nodeVar78.z ), 1.0 ) ).xyz );
		nodeVar88 = max( ( 4.0 - nodeVar86 ), 0.0 );
		nodeVar86 = max( nodeVar86, 4.0 );
		nodeVar89 = exp2( nodeVar86 );
		nodeVar90 = ( ( getUV( ( object.nodeUniform32 * vec4<f32>( vec3<f32>( nodeVar78.x, ( - nodeVar78.y ), nodeVar78.z ), 1.0 ) ).xyz, nodeVar87 ) * vec2<f32>( ( nodeVar89 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar87 > 2.0 ) ) {

			nodeVar90.y = ( nodeVar90.y + nodeVar89 );
			nodeVar87 = ( nodeVar87 - 3.0 );
			

		}

		nodeVar90.x = ( nodeVar90.x + ( nodeVar87 * nodeVar89 ) );
		nodeVar90.x = ( nodeVar90.x + ( nodeVar88 * ( 3.0 * 16.0 ) ) );
		nodeVar90.y = ( nodeVar90.y + ( 4.0 * ( exp2( object.nodeUniform31 ) - nodeVar89 ) ) );
		nodeVar90.x = ( nodeVar90.x * object.nodeUniform34 );
		nodeVar90.y = ( nodeVar90.y * object.nodeUniform35 );
		nodeVar91 = textureSampleGrad( nodeUniform36, nodeUniform36_sampler, nodeVar90, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar92 = nodeVar91.xyz;
		nodeVar84 = mix( nodeVar84, nodeVar92, nodeVar85 );
		

	}

	nodeVar93 = ( radiance + ( nodeVar84 * vec3<f32>( object.nodeUniform37 ) ) );
	radiance = nodeVar93;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform31 );
	nodeVar95 = floor( nodeVar94 );
	nodeVar96 = nodeVar95;
	nodeVar97 = getFace( ( object.nodeUniform32 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar98 = max( ( 4.0 - nodeVar96 ), 0.0 );
	nodeVar96 = max( nodeVar96, 4.0 );
	nodeVar99 = exp2( nodeVar96 );
	nodeVar100 = ( ( getUV( ( object.nodeUniform32 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar97 ) * vec2<f32>( ( nodeVar99 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar97 > 2.0 ) ) {

		nodeVar100.y = ( nodeVar100.y + nodeVar99 );
		nodeVar97 = ( nodeVar97 - 3.0 );
		

	}

	nodeVar100.x = ( nodeVar100.x + ( nodeVar97 * nodeVar99 ) );
	nodeVar100.x = ( nodeVar100.x + ( nodeVar98 * ( 3.0 * 16.0 ) ) );
	nodeVar100.y = ( nodeVar100.y + ( 4.0 * ( exp2( object.nodeUniform31 ) - nodeVar99 ) ) );
	nodeVar100.x = ( nodeVar100.x * object.nodeUniform34 );
	nodeVar100.y = ( nodeVar100.y * object.nodeUniform35 );
	nodeVar101 = textureSampleGrad( nodeUniform36, nodeUniform36_sampler, nodeVar100, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar102 = nodeVar101.xyz;
	nodeVar103 = fract( nodeVar94 );

	if ( ( nodeVar103 != 0.0 ) ) {

		nodeVar104 = ( nodeVar95 + 1.0 );
		nodeVar105 = getFace( ( object.nodeUniform32 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar106 = max( ( 4.0 - nodeVar104 ), 0.0 );
		nodeVar104 = max( nodeVar104, 4.0 );
		nodeVar107 = exp2( nodeVar104 );
		nodeVar108 = ( ( getUV( ( object.nodeUniform32 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar105 ) * vec2<f32>( ( nodeVar107 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar105 > 2.0 ) ) {

			nodeVar108.y = ( nodeVar108.y + nodeVar107 );
			nodeVar105 = ( nodeVar105 - 3.0 );
			

		}

		nodeVar108.x = ( nodeVar108.x + ( nodeVar105 * nodeVar107 ) );
		nodeVar108.x = ( nodeVar108.x + ( nodeVar106 * ( 3.0 * 16.0 ) ) );
		nodeVar108.y = ( nodeVar108.y + ( 4.0 * ( exp2( object.nodeUniform31 ) - nodeVar107 ) ) );
		nodeVar108.x = ( nodeVar108.x * object.nodeUniform34 );
		nodeVar108.y = ( nodeVar108.y * object.nodeUniform35 );
		nodeVar109 = textureSampleGrad( nodeUniform36, nodeUniform36_sampler, nodeVar108, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar110 = nodeVar109.xyz;
		nodeVar102 = mix( nodeVar102, nodeVar110, nodeVar103 );
		

	}

	nodeVar111 = ( iblIrradiance + ( ( nodeVar102 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform37 ) ) );
	iblIrradiance = nodeVar111;
	ambientOcclusion = 1.0;
	nodeVar112 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar112;
	nodeVar113 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar114 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar115 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar116 = ( SpecularF90 * dfg.y );
	nodeVar117 = ( nodeVar115 + vec3<f32>( nodeVar116 ) );
	nodeVar118 = ( nodeVar113 + nodeVar117 );
	nodeVar113 = nodeVar118;
	nodeVar119 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( nodeVar120 * vec3<f32>( 0.047619 ) );
	nodeVar122 = ( SpecularColor + nodeVar121 );
	nodeVar123 = ( nodeVar117 * nodeVar122 );
	nodeVar124 = ( dfg.x + dfg.y );
	nodeVar125 = ( 1.0 - nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( vec3<f32>( nodeVar126 ) * nodeVar122 );
	nodeVar128 = ( vec3<f32>( 1.0 ) - nodeVar127 );
	nodeVar129 = nodeVar128;
	nodeVar130 = ( nodeVar123 / nodeVar129 );
	nodeVar131 = ( nodeVar130 * vec3<f32>( nodeVar126 ) );
	nodeVar132 = ( nodeVar114 + nodeVar131 );
	nodeVar114 = nodeVar132;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar133 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar134 = ( irradiance * nodeVar133 );
	nodeVar135 = ( nodeVar113 + nodeVar114 );
	nodeVar136 = ( vec3<f32>( 1.0 ) - nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( nodeVar134 * nodeVar137 );
	nodeVar139 = nodeVar138;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = ( indirectDiffuse + nodeVar139 );
	indirectDiffuse = nodeVar140;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar141 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar142 = ( SpecularF90 * dfg.y );
	nodeVar143 = ( nodeVar141 + vec3<f32>( nodeVar142 ) );
	nodeVar144 = ( singleScatteringDielectric + nodeVar143 );
	singleScatteringDielectric = nodeVar144;
	nodeVar145 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar146 = nodeVar145;
	nodeVar147 = ( nodeVar146 * vec3<f32>( 0.047619 ) );
	nodeVar148 = ( SpecularColor + nodeVar147 );
	nodeVar149 = ( nodeVar143 * nodeVar148 );
	nodeVar150 = ( dfg.x + dfg.y );
	nodeVar151 = ( 1.0 - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( vec3<f32>( nodeVar152 ) * nodeVar148 );
	nodeVar154 = ( vec3<f32>( 1.0 ) - nodeVar153 );
	nodeVar155 = nodeVar154;
	nodeVar156 = ( nodeVar149 / nodeVar155 );
	nodeVar157 = ( nodeVar156 * vec3<f32>( nodeVar152 ) );
	nodeVar158 = ( multiScatteringDielectric + nodeVar157 );
	multiScatteringDielectric = nodeVar158;
	nodeVar159 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar160 = ( SpecularF90 * dfg.y );
	nodeVar161 = ( nodeVar159 + vec3<f32>( nodeVar160 ) );
	nodeVar162 = ( singleScatteringMetallic + nodeVar161 );
	singleScatteringMetallic = nodeVar162;
	nodeVar163 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar164 = nodeVar163;
	nodeVar165 = ( nodeVar164 * vec3<f32>( 0.047619 ) );
	nodeVar166 = ( DiffuseColor.xyz + nodeVar165 );
	nodeVar167 = ( nodeVar161 * nodeVar166 );
	nodeVar168 = ( dfg.x + dfg.y );
	nodeVar169 = ( 1.0 - nodeVar168 );
	nodeVar170 = nodeVar169;
	nodeVar171 = ( vec3<f32>( nodeVar170 ) * nodeVar166 );
	nodeVar172 = ( vec3<f32>( 1.0 ) - nodeVar171 );
	nodeVar173 = nodeVar172;
	nodeVar174 = ( nodeVar167 / nodeVar173 );
	nodeVar175 = ( nodeVar174 * vec3<f32>( nodeVar170 ) );
	nodeVar176 = ( multiScatteringMetallic + nodeVar175 );
	multiScatteringMetallic = nodeVar176;
	nodeVar177 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar178 = ( vec3<f32>( 0.0, 0.0, 0.0 ) * nodeVar177 );
	nodeVar179 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar180 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar181 = ( nodeVar179 * nodeVar180 );
	nodeVar182 = ( nodeVar178 + nodeVar181 );
	nodeVar183 = nodeVar182;
	nodeVar184 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar185 = ( vec3<f32>( 1.0 ) - nodeVar184 );
	nodeVar186 = nodeVar185;
	nodeVar187 = ( DiffuseContribution * nodeVar186 );
	nodeVar188 = ( nodeVar187 * nodeVar180 );
	nodeVar189 = nodeVar188;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar190 = ( indirectSpecular + nodeVar183 );
	indirectSpecular = nodeVar190;
	nodeVar191 = ( indirectDiffuse + nodeVar189 );
	indirectDiffuse = nodeVar191;
	nodeVar192 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar192;
	nodeVar193 = dot( normalView, positionViewDirection );
	nodeVar194 = ( clamp( nodeVar193, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar195 = ( Roughness * -16.0 );
	nodeVar196 = ( 1.0 - nodeVar195 );
	nodeVar197 = nodeVar196;
	nodeVar198 = ( - nodeVar197 );
	nodeVar199 = exp2( nodeVar198 );
	nodeVar200 = pow( nodeVar194, nodeVar199 );
	nodeVar201 = ( 1.0 - nodeVar200 );
	nodeVar202 = nodeVar201;
	nodeVar203 = ( ambientOcclusion - nodeVar202 );
	nodeVar204 = ( indirectSpecular * vec3<f32>( clamp( nodeVar203, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar204;
	nodeVar205 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar205;
	nodeVar206 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar206;
	nodeVar207 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar207;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	nodeVar208 = vec4<f32>( ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ), ( object.nodeUniform7 * nodeVar2.y ) );
	output.m1 = nodeVar208;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform39 );
	nodeVar209 = ( ( object.nodeUniform38 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar210 = ( ( render.nodeUniform40 * ( object.nodeUniform41 * object.nodeUniform42 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar211 = ( ( nodeVar209.xy / vec2<f32>( nodeVar209.w ) ) - ( nodeVar210.xy / vec2<f32>( nodeVar210.w ) ) );
	output.m2 = vec4<f32>( vec3<f32>( nodeVar211, 0.0 ), 1.0 );
	nodeVar212 = vec4<f32>( DiffuseColor.xyz, ( object.nodeUniform5 * nodeVar1.z ) );
	output.m3 = nodeVar212;

	// result

	return output;

}
