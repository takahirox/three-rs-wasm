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
@binding( 5 ) @group( 1 ) var nodeUniform21_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform21 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform31_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform31 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : f32,
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
	nodeUniform13 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform20 : f32,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : bool;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : bool;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec2<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : vec2<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec2<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec2<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec2<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
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
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec4<f32>;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec4<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec4<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec4<f32>;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec4<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec4<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec4<f32>;
var<private> nodeVar143 : array< vec3<f32>, 9 >;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : vec2<f32>;
var<private> nodeVar156 : vec4<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : vec2<f32>;
var<private> nodeVar164 : vec4<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : vec2<f32>;
var<private> nodeVar174 : vec4<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : vec2<f32>;
var<private> nodeVar182 : vec4<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : f32;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : vec3<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : vec3<f32>;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : f32;
var<private> nodeVar224 : f32;
var<private> nodeVar225 : vec3<f32>;
var<private> nodeVar226 : vec3<f32>;
var<private> nodeVar227 : vec3<f32>;
var<private> nodeVar228 : vec3<f32>;
var<private> nodeVar229 : vec3<f32>;
var<private> nodeVar230 : vec3<f32>;
var<private> nodeVar231 : vec3<f32>;
var<private> nodeVar232 : f32;
var<private> nodeVar233 : vec3<f32>;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : vec3<f32>;
var<private> nodeVar236 : vec3<f32>;
var<private> nodeVar237 : vec3<f32>;
var<private> nodeVar238 : vec3<f32>;
var<private> nodeVar239 : vec3<f32>;
var<private> nodeVar240 : f32;
var<private> nodeVar241 : f32;
var<private> nodeVar242 : f32;
var<private> nodeVar243 : vec3<f32>;
var<private> nodeVar244 : vec3<f32>;
var<private> nodeVar245 : vec3<f32>;
var<private> nodeVar246 : vec3<f32>;
var<private> nodeVar247 : vec3<f32>;
var<private> nodeVar248 : vec3<f32>;
var<private> nodeVar249 : vec3<f32>;
var<private> nodeVar250 : vec3<f32>;
var<private> nodeVar251 : vec3<f32>;
var<private> nodeVar252 : vec3<f32>;
var<private> nodeVar253 : vec3<f32>;
var<private> nodeVar254 : vec3<f32>;
var<private> nodeVar255 : vec3<f32>;
var<private> nodeVar256 : vec3<f32>;
var<private> nodeVar257 : vec3<f32>;
var<private> nodeVar258 : vec3<f32>;
var<private> nodeVar259 : vec3<f32>;
var<private> nodeVar260 : vec3<f32>;
var<private> nodeVar261 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar262 : vec3<f32>;
var<private> nodeVar263 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar264 : vec3<f32>;
var<private> nodeVar265 : f32;
var<private> nodeVar266 : f32;
var<private> nodeVar267 : f32;
var<private> nodeVar268 : f32;
var<private> nodeVar269 : f32;
var<private> nodeVar270 : f32;
var<private> nodeVar271 : f32;
var<private> nodeVar272 : f32;
var<private> nodeVar273 : f32;
var<private> nodeVar274 : f32;
var<private> nodeVar275 : f32;
var<private> nodeVar276 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar277 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar278 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar279 : vec3<f32>;
var<private> nodeVar280 : vec4<f32>;

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
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying8 : f32,
	@location( 5 ) nodeVarying9 : vec2<f32>,
	@location( 6 ) nodeVarying10 : vec3<f32>,
	@location( 7 ) nodeVarying11 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = ( nodeVarying3 == 0.0 );

	if ( nodeVar1 ) {

		nodeVar0 = array< vec3<f32>, 5 >( vec3<f32>( 0.5647115056965487, 0.24620132669705552, 0.12477181755144427 ), vec3<f32>( 0.3967552307153359, 0.158960835050774, 0.0722718506743852 ), vec3<f32>( 0.2541520943200296, 0.09084171117479915, 0.035601314869097636 ), vec3<f32>( 0.14702726648767014, 0.04666508633021928, 0.01764195448412081 ), vec3<f32>( 0.6938717612856897, 0.3515325994898463, 0.1844749944900301 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 17.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ];

	} else {


		if ( ( nodeVarying3 == 1.0 ) ) {

			nodeVar3 = ( nodeVarying9.y * 6.283185307179586 );
			nodeVar4 = vec3<f32>( ( sin( nodeVar3 ) * 0.09 ), nodeVarying9.x, ( cos( nodeVar3 ) * 0.09 ) );
			nodeVar5 = smoothstep( 0.013, 0.006, abs( ( abs( nodeVar4.x ) - 0.036 ) ) );
			nodeVar6 = smoothstep( 0.045, 0.075, nodeVar4.z );
			nodeVar7 = fract( ( sin( ( ( nodeVarying8 + 13.0 ) * 12.9898 ) ) * 43758.5453 ) );
			nodeVar8 = ( mix( mix( 1.55, 1.61, nodeVar7 ), mix( 1.67, 1.71, nodeVar7 ), smoothstep( -0.025, 0.07, nodeVar4.z ) ) + ( ( nodeVar4.x * ( nodeVar7 - 0.5 ) ) * 0.35 ) );
			nodeVar2 = mix( ( array< vec3<f32>, 5 >( vec3<f32>( 0.5647115056965487, 0.24620132669705552, 0.12477181755144427 ), vec3<f32>( 0.3967552307153359, 0.158960835050774, 0.0722718506743852 ), vec3<f32>( 0.2541520943200296, 0.09084171117479915, 0.035601314869097636 ), vec3<f32>( 0.14702726648767014, 0.04666508633021928, 0.01764195448412081 ), vec3<f32>( 0.6938717612856897, 0.3515325994898463, 0.1844749944900301 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 17.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( ( ( ( nodeVar5 * smoothstep( 0.005, 0.0015, abs( ( nodeVar4.y - 1.651 ) ) ) ) * nodeVar6 ) * 0.65 ) + ( ( ( nodeVar5 * smoothstep( 0.0035, 0.001, abs( ( nodeVar4.y - 1.665 ) ) ) ) * nodeVar6 ) * 0.35 ) ) + ( ( ( smoothstep( 0.03, 0.016, abs( nodeVar4.x ) ) * smoothstep( 0.004, 0.001, abs( ( nodeVar4.y - 1.58 ) ) ) ) * nodeVar6 ) * 0.25 ) ) ) ) ), array< vec3<f32>, 5 >( vec3<f32>( 0.010329823026364548, 0.007499032040460618, 0.006048833020386069 ), vec3<f32>( 0.04231141061442144, 0.023153366173251363, 0.010329823026364548 ), vec3<f32>( 0.09758734713304495, 0.05126945836711539, 0.015996293361446288 ), vec3<f32>( 0.15592646369776456, 0.13843161502267545, 0.12213877222015301 ), vec3<f32>( 0.023153366173251363, 0.019382360952473074, 0.01764195448412081 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 3.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ], smoothstep( ( nodeVar8 - 0.003 ), ( nodeVar8 + 0.003 ), nodeVar4.y ) );

		} else {


			if ( ( nodeVarying3 == 2.0 ) ) {


				if ( ( fract( ( sin( ( ( nodeVarying8 + 89.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.42 ) ) {

					nodeVar11 = ( nodeVarying10 * vec3<f32>( object.nodeUniform1 ) );
					nodeVar12 = ( ( nodeVar11.y - 1.33 ) * 0.42 );
					nodeVar13 = smoothstep( 0.03, 0.07, nodeVar11.z );
					nodeVar14 = ( ( ( smoothstep( 1.33, 1.35, nodeVar11.y ) * smoothstep( 1.5, 1.48, nodeVar11.y ) ) * smoothstep( 0.004, 0.0, ( abs( nodeVar11.x ) - nodeVar12 ) ) ) * nodeVar13 );
					nodeVar10 = mix( ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( ( ( ( smoothstep( 0.025, 0.009, abs( ( abs( nodeVar11.x ) - nodeVar12 ) ) ) * ( 1.0 - nodeVar14 ) ) * smoothstep( 1.32, 1.37, nodeVar11.y ) ) * nodeVar13 ) * 0.25 ) + ( ( smoothstep( 0.008, 0.003, abs( nodeVar11.x ) ) * nodeVar13 ) * 0.35 ) ) ) ) ), array< vec3<f32>, 4 >( vec3<f32>( 0.8069522576650873, 0.7912979403281551, 0.7454042095350284 ), vec3<f32>( 0.4793201830913402, 0.55201140150344, 0.6866853124288864 ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4793201830913402 ), vec3<f32>( 0.3231432091022285, 0.38642943377667954, 0.4793201830913402 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 71.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 4.0 ) ), 3.0 ) ) ], nodeVar14 );

				} else {

					nodeVar15 = ( nodeVarying10 * vec3<f32>( object.nodeUniform1 ) );
					nodeVar10 = ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( max( smoothstep( 1.065, 1.035, nodeVar15.y ), smoothstep( 1.455, 1.48, nodeVar15.y ) ) * 0.18 ) + ( ( smoothstep( 0.008, 0.003, abs( nodeVar15.x ) ) * smoothstep( 0.03, 0.07, nodeVar15.z ) ) * 0.3 ) ) ) ) );

				}

				nodeVar9 = nodeVar10;

			} else {


				if ( ( nodeVarying3 == 6.0 ) ) {


					if ( ( fract( ( sin( ( ( nodeVarying8 + 89.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.42 ) ) {

						nodeVar17 = array< vec3<f32>, 4 >( vec3<f32>( 0.8069522576650873, 0.7912979403281551, 0.7454042095350284 ), vec3<f32>( 0.4793201830913402, 0.55201140150344, 0.6866853124288864 ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4793201830913402 ), vec3<f32>( 0.3231432091022285, 0.38642943377667954, 0.4793201830913402 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 71.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 4.0 ) ), 3.0 ) ) ];

					} else {

						nodeVar17 = ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( 0.75 ) );

					}

					nodeVar16 = mix( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ], nodeVar17, smoothstep( 0.94, 0.985, nodeVarying9.x ) );

				} else {


					if ( ( nodeVarying3 == 3.0 ) ) {

						nodeVar18 = array< vec3<f32>, 5 >( vec3<f32>( 0.015996293361446288, 0.01764195448412081, 0.025186859622305935 ), vec3<f32>( 0.035601314869097636, 0.06480326668529614, 0.10461648408208657 ), vec3<f32>( 0.08228270712149792, 0.06301001764564068, 0.04666508633021928 ), vec3<f32>( 0.20155625378383743, 0.16513219449147767, 0.11697066774917994 ), vec3<f32>( 0.018500220124016652, 0.018500220124016652, 0.019382360952473074 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 47.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ];

					} else {


						if ( ( nodeVarying3 == 4.0 ) ) {

							nodeVar21 = ( fract( ( sin( ( ( nodeVarying8 + 5.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.4 );

							if ( nodeVar21 ) {

								nodeVar20 = mix( vec3<f32>( 0.018500220124016652, 0.024157632443547246, 0.031896033067374104 ), vec3<f32>( 0.5775804404214573, 0.5457244613615395, 0.4793201830913402 ), step( 0.55, fract( ( sin( ( ( nodeVarying8 + 31.0 ) * 12.9898 ) ) * 43758.5453 ) ) ) );

							} else {

								nodeVar20 = mix( vec3<f32>( 0.014443843592229466, 0.012983032338510335, 0.011612245176281512 ), vec3<f32>( 0.06847816983662762, 0.035601314869097636, 0.01764195448412081 ), fract( ( sin( ( ( nodeVarying8 + 23.0 ) * 12.9898 ) ) * 43758.5453 ) ) );

							}


							if ( nodeVar21 ) {

								nodeVar22 = vec3<f32>( 0.6653872982754769, 0.6375968739867731, 0.5775804404214573 );

							} else {

								nodeVar22 = vec3<f32>( 0.00972121731707524, 0.009134058699157796, 0.008023192982520563 );

							}

							nodeVar23 = ( nodeVarying9.y * 110.0 );
							nodeVar19 = mix( mix( nodeVar20, nodeVar22, smoothstep( -0.064, -0.074, nodeVarying9.x ) ), vec3<f32>( 0.407240211891531, 0.3915724777393922, 0.34670405634441115 ), ( ( ( ( smoothstep( 0.68, 0.78, fract( nodeVar23 ) ) * clamp( ( 1.0 - fwidth( nodeVar23 ) ), 0.0, 1.0 ) ) * smoothstep( -0.015, 0.005, nodeVarying9.x ) ) * smoothstep( -0.01, 0.015, nodeVarying9.y ) ) * 0.45 ) );

						} else {


							if ( ( nodeVarying3 == 5.0 ) ) {

								nodeVar25 = min( nodeVarying9, ( vec2<f32>( 1.0 ) - nodeVarying9 ) );

								if ( ( abs( nodeVarying11.x ) > 0.8 ) ) {

									nodeVar26 = 1.0;

								} else {

									nodeVar26 = 0.0;

								}

								nodeVar24 = mix( ( vec3<f32>( 0.06662593863608139, 0.038204371589236, 0.023153366173251363 ) * vec3<f32>( ( 1.0 - ( max( smoothstep( 0.045, 0.02, min( nodeVar25.x, nodeVar25.y ) ), smoothstep( 0.02, 0.008, abs( ( nodeVarying9.y - 0.72 ) ) ) ) * 0.35 ) ) ) ), vec3<f32>( 0.2746773120495699, 0.24620132669705552, 0.1878207722902346 ), ( ( smoothstep( 0.06, 0.045, abs( ( nodeVarying9.x - 0.5 ) ) ) * smoothstep( 0.07, 0.05, abs( ( nodeVarying9.y - 0.72 ) ) ) ) * nodeVar26 ) );

							} else {

								nodeVar24 = vec3<f32>( 0.02955683443236377, 0.019382360952473074, 0.013702083043526807 );

							}

							nodeVar19 = nodeVar24;

						}

						nodeVar18 = nodeVar19;

					}

					nodeVar16 = nodeVar18;

				}

				nodeVar9 = nodeVar16;

			}

			nodeVar2 = nodeVar9;

		}

		nodeVar0 = nodeVar2;

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform3;

	if ( nodeVar1 ) {

		nodeVar27 = 0.6;

	} else {


		if ( ( nodeVarying3 == 1.0 ) ) {

			nodeVar29 = fract( ( sin( ( ( nodeVarying8 + 13.0 ) * 12.9898 ) ) * 43758.5453 ) );
			nodeVar30 = ( nodeVarying9.y * 6.283185307179586 );
			nodeVar31 = vec3<f32>( ( sin( nodeVar30 ) * 0.09 ), nodeVarying9.x, ( cos( nodeVar30 ) * 0.09 ) );
			nodeVar32 = ( mix( mix( 1.55, 1.61, nodeVar29 ), mix( 1.67, 1.71, nodeVar29 ), smoothstep( -0.025, 0.07, nodeVar31.z ) ) + ( ( nodeVar31.x * ( nodeVar29 - 0.5 ) ) * 0.35 ) );
			nodeVar28 = mix( 0.6, 0.85, smoothstep( ( nodeVar32 - 0.003 ), ( nodeVar32 + 0.003 ), nodeVar31.y ) );

		} else {


			if ( ( ( nodeVarying3 == 5.0 ) || ( nodeVarying3 == 7.0 ) ) ) {

				nodeVar33 = 0.55;

			} else {

				nodeVar33 = 0.85;

			}

			nodeVar28 = nodeVar33;

		}

		nodeVar27 = nodeVar28;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar34 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar27, 0.0525 ) + max( max( nodeVar34.x, nodeVar34.y ), nodeVar34.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform3 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar35 = dot( normalView, positionViewDirection );
	nodeVar36 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar35, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar36;
	nodeVar37 = ( dfg.x + dfg.y );
	nodeVar38 = ( 1.0 / nodeVar37 );
	nodeVar39 = nodeVar38;
	nodeVar40 = ( nodeVar39 - 1.0 );
	nodeVar41 = ( SpecularColorBlended * vec3<f32>( nodeVar40 ) );
	nodeVar42 = ( nodeVar41 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar42;
	nodeVar43 = ( render.nodeUniform11 - render.nodeUniform12 );
	nodeVar44 = vec4<f32>( nodeVar43, 0.0 );
	nodeVar45 = ( render.cameraViewMatrix * nodeVar44 );
	nodeVar46 = normalize( nodeVar45.xyz );
	nodeVar47 = nodeVar46;
	nodeVar48 = dot( normalView, nodeVar47 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar50 = ( render.nodeUniform14 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
	nodeVar51 = ( nodeVar50.xyz / vec3<f32>( nodeVar50.w ) );
	nodeVar52 = vec3<f32>( nodeVar51.x, ( 1.0 - nodeVar51.y ), ( nodeVar51.z + render.nodeUniform16 ) );

	if ( ( ( ( ( ( nodeVar52.x >= 0.0 ) && ( nodeVar52.x <= 1.0 ) ) && ( nodeVar52.y >= 0.0 ) ) && ( nodeVar52.y <= 1.0 ) ) && ( nodeVar52.z <= 1.0 ) ) ) {

		nodeVar53 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar54 = ( render.nodeUniform18 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ).x );
		nodeVar55 = ( nodeVar52.xy + ( vogelDiskSample( 0, 5, nodeVar53 ) * vec2<f32>( nodeVar54 ) ) );
		nodeVar56 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar55, nodeVar52.z );
		nodeVar57 = ( nodeVar52.xy + ( vogelDiskSample( 1, 5, nodeVar53 ) * vec2<f32>( nodeVar54 ) ) );
		nodeVar58 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar57, nodeVar52.z );
		nodeVar59 = ( nodeVar52.xy + ( vogelDiskSample( 2, 5, nodeVar53 ) * vec2<f32>( nodeVar54 ) ) );
		nodeVar60 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar59, nodeVar52.z );
		nodeVar61 = ( nodeVar52.xy + ( vogelDiskSample( 3, 5, nodeVar53 ) * vec2<f32>( nodeVar54 ) ) );
		nodeVar62 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar61, nodeVar52.z );
		nodeVar63 = ( nodeVar52.xy + ( vogelDiskSample( 4, 5, nodeVar53 ) * vec2<f32>( nodeVar54 ) ) );
		nodeVar64 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar63, nodeVar52.z );
		nodeVar49 = ( ( ( ( ( nodeVar56 + nodeVar58 ) + nodeVar60 ) + nodeVar62 ) + nodeVar64 ) * 0.2 );

	} else {

		nodeVar49 = 1.0;

	}

	nodeVar65 = mix( 1.0, nodeVar49, render.nodeUniform20 );
	nodeVar66 = ( vec3<f32>( clamp( nodeVar48, 0.0, 1.0 ) ) * ( render.nodeUniform13 * vec3<f32>( nodeVar65 ) ) );
	nodeVar67 = nodeVar66;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar68 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar69 = ( nodeVar67 * nodeVar68 );
	nodeVar70 = ( nodeVar47 + positionViewDirection );
	nodeVar71 = normalize( nodeVar70 );
	nodeVar72 = dot( positionViewDirection, nodeVar71 );
	nodeVar73 = clamp( nodeVar72, 0.0, 1.0 );
	nodeVar74 = exp2( ( ( ( nodeVar73 * -5.55473 ) - 6.98316 ) * nodeVar73 ) );
	nodeVar75 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar74 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar74 ) ) );
	nodeVar76 = ( vec3<f32>( 1.0 ) - nodeVar75 );
	nodeVar77 = nodeVar76;
	nodeVar78 = ( nodeVar69 * nodeVar77 );
	nodeVar79 = ( directDiffuse + nodeVar78 );
	directDiffuse = nodeVar79;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar80 = normalize( ( nodeVar47 + positionViewDirection ) );
	nodeVar81 = clamp( dot( positionViewDirection, nodeVar80 ), 0.0, 1.0 );
	nodeVar82 = exp2( ( ( ( nodeVar81 * -5.55473 ) - 6.98316 ) * nodeVar81 ) );
	nodeVar83 = ( Roughness * Roughness );
	nodeVar84 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar82 ) ) ) + vec3<f32>( ( 1.0 * nodeVar82 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar83, clamp( dot( normalView, nodeVar47 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar83, clamp( dot( normalView, nodeVar80 ), 0.0, 1.0 ) ) ) );
	nodeVar85 = ( nodeVar67 * nodeVar84 );
	nodeVar86 = ( nodeVar85 * multiScatteringCompensation );
	nodeVar87 = ( directSpecular + nodeVar86 );
	directSpecular = nodeVar87;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar88 = ( object.nodeUniform22 - object.nodeUniform23 );
	nodeVar89 = ( object.nodeUniform24 - vec3<f32>( 1.0 ) );
	nodeVar90 = ( nodeVar88 / nodeVar89 );
	nodeVar91 = ( normalWorld * nodeVar90 );
	nodeVar92 = ( nodeVar91 * vec3<f32>( 0.5 ) );
	nodeVar93 = ( v_positionWorld + nodeVar92 );
	nodeVar94 = ( nodeVar93 - object.nodeUniform23 );
	nodeVar95 = ( nodeVar94 / nodeVar88 );
	nodeVar96 = ( clamp( nodeVar95, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar89 );
	nodeVar97 = ( nodeVar96 / object.nodeUniform24 );
	nodeVar98 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform24 );
	nodeVar99 = ( nodeVar97 + nodeVar98 );
	nodeVar100 = ( nodeVar99.z * object.nodeUniform24.z );
	nodeVar101 = ( nodeVar100 + 1.0 );
	nodeVar102 = ( object.nodeUniform24.z + 2.0 );
	nodeVar103 = ( nodeVar102 * 0.0 );
	nodeVar104 = ( nodeVar101 + nodeVar103 );
	nodeVar105 = ( nodeVar102 * 7.0 );
	nodeVar106 = ( nodeVar104 / nodeVar105 );
	nodeVar107 = vec3<f32>( nodeVar99.xy, nodeVar106 );
	nodeVar108 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar107 );
	nodeVar109 = ( nodeVar102 * 1.0 );
	nodeVar110 = ( nodeVar101 + nodeVar109 );
	nodeVar111 = ( nodeVar110 / nodeVar105 );
	nodeVar112 = vec3<f32>( nodeVar99.xy, nodeVar111 );
	nodeVar113 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar112 );
	nodeVar114 = vec3<f32>( nodeVar108.w, nodeVar113.xy );
	nodeVar115 = ( nodeVar102 * 2.0 );
	nodeVar116 = ( nodeVar101 + nodeVar115 );
	nodeVar117 = ( nodeVar116 / nodeVar105 );
	nodeVar118 = vec3<f32>( nodeVar99.xy, nodeVar117 );
	nodeVar119 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar118 );
	nodeVar120 = vec3<f32>( nodeVar113.zw, nodeVar119.x );
	nodeVar121 = ( nodeVar102 * 3.0 );
	nodeVar122 = ( nodeVar101 + nodeVar121 );
	nodeVar123 = ( nodeVar122 / nodeVar105 );
	nodeVar124 = vec3<f32>( nodeVar99.xy, nodeVar123 );
	nodeVar125 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar124 );
	nodeVar126 = ( nodeVar102 * 4.0 );
	nodeVar127 = ( nodeVar101 + nodeVar126 );
	nodeVar128 = ( nodeVar127 / nodeVar105 );
	nodeVar129 = vec3<f32>( nodeVar99.xy, nodeVar128 );
	nodeVar130 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar129 );
	nodeVar131 = vec3<f32>( nodeVar125.w, nodeVar130.xy );
	nodeVar132 = ( nodeVar102 * 5.0 );
	nodeVar133 = ( nodeVar101 + nodeVar132 );
	nodeVar134 = ( nodeVar133 / nodeVar105 );
	nodeVar135 = vec3<f32>( nodeVar99.xy, nodeVar134 );
	nodeVar136 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar135 );
	nodeVar137 = vec3<f32>( nodeVar130.zw, nodeVar136.x );
	nodeVar138 = ( nodeVar102 * 6.0 );
	nodeVar139 = ( nodeVar101 + nodeVar138 );
	nodeVar140 = ( nodeVar139 / nodeVar105 );
	nodeVar141 = vec3<f32>( nodeVar99.xy, nodeVar140 );
	nodeVar142 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar141 );
	nodeVar143 = array< vec3<f32>, 9 >( nodeVar108.xyz, nodeVar114, nodeVar120, nodeVar119.yzw, nodeVar125.xyz, nodeVar131, nodeVar137, nodeVar136.yzw, nodeVar142.xyz );
	nodeVar144 = ( ( ( ( ( ( ( ( ( nodeVar143[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar143[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar143[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar143[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar143[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar143[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar143[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar143[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar143[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar145 = max( nodeVar144, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar146 = ( nodeVar145 * vec3<f32>( object.nodeUniform25 ) );
	nodeVar147 = ( irradiance + nodeVar146 );
	irradiance = nodeVar147;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar148 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform26 );
	nodeVar149 = floor( nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar152 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar151.x, ( - nodeVar151.y ), nodeVar151.z ), 1.0 ) ).xyz );
	nodeVar153 = max( ( 4.0 - nodeVar150 ), 0.0 );
	nodeVar150 = max( nodeVar150, 4.0 );
	nodeVar154 = exp2( nodeVar150 );
	nodeVar155 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar151.x, ( - nodeVar151.y ), nodeVar151.z ), 1.0 ) ).xyz, nodeVar152 ) * vec2<f32>( ( nodeVar154 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar152 > 2.0 ) ) {

		nodeVar155.y = ( nodeVar155.y + nodeVar154 );
		nodeVar152 = ( nodeVar152 - 3.0 );
		

	}

	nodeVar155.x = ( nodeVar155.x + ( nodeVar152 * nodeVar154 ) );
	nodeVar155.x = ( nodeVar155.x + ( nodeVar153 * ( 3.0 * 16.0 ) ) );
	nodeVar155.y = ( nodeVar155.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar154 ) ) );
	nodeVar155.x = ( nodeVar155.x * object.nodeUniform29 );
	nodeVar155.y = ( nodeVar155.y * object.nodeUniform30 );
	nodeVar156 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar155, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar157 = nodeVar156.xyz;
	nodeVar158 = fract( nodeVar148 );

	if ( ( nodeVar158 != 0.0 ) ) {

		nodeVar159 = ( nodeVar149 + 1.0 );
		nodeVar160 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar151.x, ( - nodeVar151.y ), nodeVar151.z ), 1.0 ) ).xyz );
		nodeVar161 = max( ( 4.0 - nodeVar159 ), 0.0 );
		nodeVar159 = max( nodeVar159, 4.0 );
		nodeVar162 = exp2( nodeVar159 );
		nodeVar163 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar151.x, ( - nodeVar151.y ), nodeVar151.z ), 1.0 ) ).xyz, nodeVar160 ) * vec2<f32>( ( nodeVar162 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar160 > 2.0 ) ) {

			nodeVar163.y = ( nodeVar163.y + nodeVar162 );
			nodeVar160 = ( nodeVar160 - 3.0 );
			

		}

		nodeVar163.x = ( nodeVar163.x + ( nodeVar160 * nodeVar162 ) );
		nodeVar163.x = ( nodeVar163.x + ( nodeVar161 * ( 3.0 * 16.0 ) ) );
		nodeVar163.y = ( nodeVar163.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar162 ) ) );
		nodeVar163.x = ( nodeVar163.x * object.nodeUniform29 );
		nodeVar163.y = ( nodeVar163.y * object.nodeUniform30 );
		nodeVar164 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar163, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar165 = nodeVar164.xyz;
		nodeVar157 = mix( nodeVar157, nodeVar165, nodeVar158 );
		

	}

	nodeVar166 = ( radiance + ( nodeVar157 * vec3<f32>( object.nodeUniform32 ) ) );
	radiance = nodeVar166;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar167 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform26 );
	nodeVar168 = floor( nodeVar167 );
	nodeVar169 = nodeVar168;
	nodeVar170 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar171 = max( ( 4.0 - nodeVar169 ), 0.0 );
	nodeVar169 = max( nodeVar169, 4.0 );
	nodeVar172 = exp2( nodeVar169 );
	nodeVar173 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar170 ) * vec2<f32>( ( nodeVar172 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar170 > 2.0 ) ) {

		nodeVar173.y = ( nodeVar173.y + nodeVar172 );
		nodeVar170 = ( nodeVar170 - 3.0 );
		

	}

	nodeVar173.x = ( nodeVar173.x + ( nodeVar170 * nodeVar172 ) );
	nodeVar173.x = ( nodeVar173.x + ( nodeVar171 * ( 3.0 * 16.0 ) ) );
	nodeVar173.y = ( nodeVar173.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar172 ) ) );
	nodeVar173.x = ( nodeVar173.x * object.nodeUniform29 );
	nodeVar173.y = ( nodeVar173.y * object.nodeUniform30 );
	nodeVar174 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar173, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar175 = nodeVar174.xyz;
	nodeVar176 = fract( nodeVar167 );

	if ( ( nodeVar176 != 0.0 ) ) {

		nodeVar177 = ( nodeVar168 + 1.0 );
		nodeVar178 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar179 = max( ( 4.0 - nodeVar177 ), 0.0 );
		nodeVar177 = max( nodeVar177, 4.0 );
		nodeVar180 = exp2( nodeVar177 );
		nodeVar181 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar178 ) * vec2<f32>( ( nodeVar180 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar178 > 2.0 ) ) {

			nodeVar181.y = ( nodeVar181.y + nodeVar180 );
			nodeVar178 = ( nodeVar178 - 3.0 );
			

		}

		nodeVar181.x = ( nodeVar181.x + ( nodeVar178 * nodeVar180 ) );
		nodeVar181.x = ( nodeVar181.x + ( nodeVar179 * ( 3.0 * 16.0 ) ) );
		nodeVar181.y = ( nodeVar181.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar180 ) ) );
		nodeVar181.x = ( nodeVar181.x * object.nodeUniform29 );
		nodeVar181.y = ( nodeVar181.y * object.nodeUniform30 );
		nodeVar182 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar181, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar183 = nodeVar182.xyz;
		nodeVar175 = mix( nodeVar175, nodeVar183, nodeVar176 );
		

	}

	nodeVar184 = ( iblIrradiance + ( ( nodeVar175 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform32 ) ) );
	iblIrradiance = nodeVar184;
	nodeVar185 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar186 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar187 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar188 = ( SpecularF90 * dfg.y );
	nodeVar189 = ( nodeVar187 + vec3<f32>( nodeVar188 ) );
	nodeVar190 = ( nodeVar185 + nodeVar189 );
	nodeVar185 = nodeVar190;
	nodeVar191 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar192 = nodeVar191;
	nodeVar193 = ( nodeVar192 * vec3<f32>( 0.047619 ) );
	nodeVar194 = ( SpecularColor + nodeVar193 );
	nodeVar195 = ( nodeVar189 * nodeVar194 );
	nodeVar196 = ( dfg.x + dfg.y );
	nodeVar197 = ( 1.0 - nodeVar196 );
	nodeVar198 = nodeVar197;
	nodeVar199 = ( vec3<f32>( nodeVar198 ) * nodeVar194 );
	nodeVar200 = ( vec3<f32>( 1.0 ) - nodeVar199 );
	nodeVar201 = nodeVar200;
	nodeVar202 = ( nodeVar195 / nodeVar201 );
	nodeVar203 = ( nodeVar202 * vec3<f32>( nodeVar198 ) );
	nodeVar204 = ( nodeVar186 + nodeVar203 );
	nodeVar186 = nodeVar204;
	nodeVar205 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar206 = ( irradiance * nodeVar205 );
	nodeVar207 = ( nodeVar185 + nodeVar186 );
	nodeVar208 = ( vec3<f32>( 1.0 ) - nodeVar207 );
	nodeVar209 = nodeVar208;
	nodeVar210 = ( nodeVar206 * nodeVar209 );
	nodeVar211 = nodeVar210;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar212 = ( indirectDiffuse + nodeVar211 );
	indirectDiffuse = nodeVar212;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar213 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar214 = ( SpecularF90 * dfg.y );
	nodeVar215 = ( nodeVar213 + vec3<f32>( nodeVar214 ) );
	nodeVar216 = ( singleScatteringDielectric + nodeVar215 );
	singleScatteringDielectric = nodeVar216;
	nodeVar217 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar218 = nodeVar217;
	nodeVar219 = ( nodeVar218 * vec3<f32>( 0.047619 ) );
	nodeVar220 = ( SpecularColor + nodeVar219 );
	nodeVar221 = ( nodeVar215 * nodeVar220 );
	nodeVar222 = ( dfg.x + dfg.y );
	nodeVar223 = ( 1.0 - nodeVar222 );
	nodeVar224 = nodeVar223;
	nodeVar225 = ( vec3<f32>( nodeVar224 ) * nodeVar220 );
	nodeVar226 = ( vec3<f32>( 1.0 ) - nodeVar225 );
	nodeVar227 = nodeVar226;
	nodeVar228 = ( nodeVar221 / nodeVar227 );
	nodeVar229 = ( nodeVar228 * vec3<f32>( nodeVar224 ) );
	nodeVar230 = ( multiScatteringDielectric + nodeVar229 );
	multiScatteringDielectric = nodeVar230;
	nodeVar231 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar232 = ( SpecularF90 * dfg.y );
	nodeVar233 = ( nodeVar231 + vec3<f32>( nodeVar232 ) );
	nodeVar234 = ( singleScatteringMetallic + nodeVar233 );
	singleScatteringMetallic = nodeVar234;
	nodeVar235 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar236 = nodeVar235;
	nodeVar237 = ( nodeVar236 * vec3<f32>( 0.047619 ) );
	nodeVar238 = ( DiffuseColor.xyz + nodeVar237 );
	nodeVar239 = ( nodeVar233 * nodeVar238 );
	nodeVar240 = ( dfg.x + dfg.y );
	nodeVar241 = ( 1.0 - nodeVar240 );
	nodeVar242 = nodeVar241;
	nodeVar243 = ( vec3<f32>( nodeVar242 ) * nodeVar238 );
	nodeVar244 = ( vec3<f32>( 1.0 ) - nodeVar243 );
	nodeVar245 = nodeVar244;
	nodeVar246 = ( nodeVar239 / nodeVar245 );
	nodeVar247 = ( nodeVar246 * vec3<f32>( nodeVar242 ) );
	nodeVar248 = ( multiScatteringMetallic + nodeVar247 );
	multiScatteringMetallic = nodeVar248;
	nodeVar249 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar250 = ( radiance * nodeVar249 );
	nodeVar251 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar252 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar253 = ( nodeVar251 * nodeVar252 );
	nodeVar254 = ( nodeVar250 + nodeVar253 );
	nodeVar255 = nodeVar254;
	nodeVar256 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar257 = ( vec3<f32>( 1.0 ) - nodeVar256 );
	nodeVar258 = nodeVar257;
	nodeVar259 = ( DiffuseContribution * nodeVar258 );
	nodeVar260 = ( nodeVar259 * nodeVar252 );
	nodeVar261 = nodeVar260;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar262 = ( indirectSpecular + nodeVar255 );
	indirectSpecular = nodeVar262;
	nodeVar263 = ( indirectDiffuse + nodeVar261 );
	indirectDiffuse = nodeVar263;
	ambientOcclusion = 1.0;
	nodeVar264 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar264;
	nodeVar265 = dot( normalView, positionViewDirection );
	nodeVar266 = ( clamp( nodeVar265, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar267 = ( Roughness * -16.0 );
	nodeVar268 = ( 1.0 - nodeVar267 );
	nodeVar269 = nodeVar268;
	nodeVar270 = ( - nodeVar269 );
	nodeVar271 = exp2( nodeVar270 );
	nodeVar272 = pow( nodeVar266, nodeVar271 );
	nodeVar273 = ( 1.0 - nodeVar272 );
	nodeVar274 = nodeVar273;
	nodeVar275 = ( ambientOcclusion - nodeVar274 );
	nodeVar276 = ( indirectSpecular * vec3<f32>( clamp( nodeVar275, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar276;
	nodeVar277 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar277;
	nodeVar278 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar278;
	nodeVar279 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar279;
	nodeVar280 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar280;

	// result

	output.color = nodeVar280;

	return output;

}
