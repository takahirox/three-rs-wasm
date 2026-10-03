// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform18_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform18 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform22 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform32_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform32 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform2 : u32,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : mat4x4<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform33 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform21 : f32,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : u32;
var<private> nodeVar7 : u32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : u32;
var<private> nodeVar10 : u32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec4<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec4<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec4<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec4<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec4<f32>;
var<private> nodeVar108 : vec3<f32>;
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
var<private> nodeVar120 : array< vec3<f32>, 9 >;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : vec2<f32>;
var<private> nodeVar133 : vec4<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec2<f32>;
var<private> nodeVar141 : vec4<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : vec2<f32>;
var<private> nodeVar151 : vec4<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : vec2<f32>;
var<private> nodeVar159 : vec4<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : f32;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : f32;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : f32;
var<private> nodeVar218 : f32;
var<private> nodeVar219 : f32;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : vec3<f32>;
var<private> nodeVar222 : vec3<f32>;
var<private> nodeVar223 : vec3<f32>;
var<private> nodeVar224 : vec3<f32>;
var<private> nodeVar225 : vec3<f32>;
var<private> nodeVar226 : vec3<f32>;
var<private> nodeVar227 : vec3<f32>;
var<private> nodeVar228 : vec3<f32>;
var<private> nodeVar229 : vec3<f32>;
var<private> nodeVar230 : vec3<f32>;
var<private> nodeVar231 : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> nodeVar233 : vec3<f32>;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : vec3<f32>;
var<private> nodeVar236 : vec3<f32>;
var<private> nodeVar237 : vec3<f32>;
var<private> nodeVar238 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar239 : vec3<f32>;
var<private> nodeVar240 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar241 : vec3<f32>;
var<private> nodeVar242 : f32;
var<private> nodeVar243 : f32;
var<private> nodeVar244 : f32;
var<private> nodeVar245 : f32;
var<private> nodeVar246 : f32;
var<private> nodeVar247 : f32;
var<private> nodeVar248 : f32;
var<private> nodeVar249 : f32;
var<private> nodeVar250 : f32;
var<private> nodeVar251 : f32;
var<private> nodeVar252 : f32;
var<private> nodeVar253 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar254 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar255 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar256 : vec3<f32>;
var<private> nodeVar257 : vec4<f32>;

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
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( v_positionWorld.x + 101.0 );
	nodeVar1 = floor( ( nodeVar0 / 112.0 ) );
	nodeVar2 = ( v_positionWorld.z + 71.0 );
	nodeVar3 = floor( ( nodeVar2 / 82.0 ) );
	nodeVar4 = object.nodeUniform2;
	nodeVar5 = ( ( ( u32( ( ( ( nodeVar1 * 3.0 ) + clamp( floor( ( ( ( nodeVar0 - ( nodeVar1 * 112.0 ) ) - 5.0 ) / 26.666666666666668 ) ), 0.0, 2.0 ) ) + 4096.0 ) ) * 73856093u ) ^ ( u32( ( ( ( nodeVar3 * 2.0 ) + clamp( floor( ( ( ( nodeVar2 - ( nodeVar3 * 82.0 ) ) - 5.0 ) / 25.0 ) ), 0.0, 1.0 ) ) + 4096.0 ) ) * 19349663u ) ) ^ ( nodeVar4 * 2654435761u ) );
	nodeVar6 = ( ( ( nodeVar5 + 230900u ) * 747796405u ) + 2891336453u );
	nodeVar7 = ( ( ( nodeVar6 >> ( ( nodeVar6 >> 28u ) + 4u ) ) ^ nodeVar6 ) * 277803737u );
	nodeVar8 = ( f32( ( ( nodeVar7 >> 22u ) ^ nodeVar7 ) ) * 2.3283064365386963e-10 );
	nodeVar9 = ( ( ( nodeVar5 + 155260u ) * 747796405u ) + 2891336453u );
	nodeVar10 = ( ( ( nodeVar9 >> ( ( nodeVar9 >> 28u ) + 4u ) ) ^ nodeVar9 ) * 277803737u );
	DiffuseColor = vec4<f32>( ( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( vec3<f32>( 0.3915724777393922, 0.09084171117479915, 0.04518620437910499 ), vec3<f32>( 0.33245153633549385, 0.06847816983662762, 0.0343398068028541 ), step( 0.058823529411764705, nodeVar8 ) ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.08437621153575764 ), step( 0.11764705882352941, nodeVar8 ) ), vec3<f32>( 0.20507873637973145, 0.12743768042608497, 0.08021982030622662 ), step( 0.17647058823529413, nodeVar8 ) ), vec3<f32>( 0.55201140150344, 0.36625259558833256, 0.16202937562896222 ), step( 0.23529411764705882, nodeVar8 ) ), vec3<f32>( 0.4793201830913402, 0.3231432091022285, 0.158960835050774 ), step( 0.29411764705882354, nodeVar8 ) ), vec3<f32>( 0.5394794890033748, 0.4396571738310091, 0.22696587349938613 ), step( 0.35294117647058826, nodeVar8 ) ), vec3<f32>( 0.5647115056965487, 0.5271151256969157, 0.4452011945063733 ), step( 0.4117647058823529, nodeVar8 ) ), vec3<f32>( 0.5647115056965487, 0.5271151256969157, 0.4452011945063733 ), step( 0.47058823529411764, nodeVar8 ) ), vec3<f32>( 0.508881320845802, 0.4735314961384573, 0.3915724777393922 ), step( 0.5294117647058824, nodeVar8 ) ), vec3<f32>( 0.6375968739867731, 0.6038273388475408, 0.514917665367466 ), step( 0.5882352941176471, nodeVar8 ) ), vec3<f32>( 0.45641102317066595, 0.4286904966038916, 0.35640014413537763 ), step( 0.6470588235294118, nodeVar8 ) ), vec3<f32>( 0.3231432091022285, 0.3139887133649649, 0.2746773120495699 ), step( 0.7058823529411765, nodeVar8 ) ), vec3<f32>( 0.25818285291079235, 0.2501582847191642, 0.22696587349938613 ), step( 0.7647058823529411, nodeVar8 ) ), vec3<f32>( 0.37626212298046485, 0.36625259558833256, 0.3231432091022285 ), step( 0.8235294117647058, nodeVar8 ) ), vec3<f32>( 0.7083757798856457, 0.6724431569510133, 0.5972017883558645 ), step( 0.8823529411764706, nodeVar8 ) ), vec3<f32>( 0.20155625378383743, 0.23839757380151394, 0.2663556047920505 ), step( 0.9411764705882353, nodeVar8 ) ) * vec3<f32>( ( ( ( f32( ( ( nodeVar10 >> 22u ) ^ nodeVar10 ) ) * 2.3283064365386963e-10 ) * 0.12 ) + 0.94 ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar11 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform5, 0.0525 ) + max( max( nodeVar11.x, nodeVar11.y ), nodeVar11.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar12 = dot( normalView, positionViewDirection );
	nodeVar13 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( Roughness, clamp( nodeVar12, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar13;
	nodeVar14 = ( dfg.x + dfg.y );
	nodeVar15 = ( 1.0 / nodeVar14 );
	nodeVar16 = nodeVar15;
	nodeVar17 = ( nodeVar16 - 1.0 );
	nodeVar18 = ( SpecularColorBlended * vec3<f32>( nodeVar17 ) );
	nodeVar19 = ( nodeVar18 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar19;
	nodeVar20 = ( render.nodeUniform12 - render.nodeUniform13 );
	nodeVar21 = vec4<f32>( nodeVar20, 0.0 );
	nodeVar22 = ( render.cameraViewMatrix * nodeVar21 );
	nodeVar23 = normalize( nodeVar22.xyz );
	nodeVar24 = nodeVar23;
	nodeVar25 = dot( normalView, nodeVar24 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar27 = ( render.nodeUniform15 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform16 ) ) ), 1.0 ) );
	nodeVar28 = ( nodeVar27.xyz / vec3<f32>( nodeVar27.w ) );
	nodeVar29 = vec3<f32>( nodeVar28.x, ( 1.0 - nodeVar28.y ), ( nodeVar28.z + render.nodeUniform17 ) );

	if ( ( ( ( ( ( nodeVar29.x >= 0.0 ) && ( nodeVar29.x <= 1.0 ) ) && ( nodeVar29.y >= 0.0 ) ) && ( nodeVar29.y <= 1.0 ) ) && ( nodeVar29.z <= 1.0 ) ) ) {

		nodeVar30 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar31 = ( render.nodeUniform19 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform20 ).x );
		nodeVar32 = ( nodeVar29.xy + ( vogelDiskSample( 0, 5, nodeVar30 ) * vec2<f32>( nodeVar31 ) ) );
		nodeVar33 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar32, nodeVar29.z );
		nodeVar34 = ( nodeVar29.xy + ( vogelDiskSample( 1, 5, nodeVar30 ) * vec2<f32>( nodeVar31 ) ) );
		nodeVar35 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar34, nodeVar29.z );
		nodeVar36 = ( nodeVar29.xy + ( vogelDiskSample( 2, 5, nodeVar30 ) * vec2<f32>( nodeVar31 ) ) );
		nodeVar37 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar36, nodeVar29.z );
		nodeVar38 = ( nodeVar29.xy + ( vogelDiskSample( 3, 5, nodeVar30 ) * vec2<f32>( nodeVar31 ) ) );
		nodeVar39 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar38, nodeVar29.z );
		nodeVar40 = ( nodeVar29.xy + ( vogelDiskSample( 4, 5, nodeVar30 ) * vec2<f32>( nodeVar31 ) ) );
		nodeVar41 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar40, nodeVar29.z );
		nodeVar26 = ( ( ( ( ( nodeVar33 + nodeVar35 ) + nodeVar37 ) + nodeVar39 ) + nodeVar41 ) * 0.2 );

	} else {

		nodeVar26 = 1.0;

	}

	nodeVar42 = mix( 1.0, nodeVar26, render.nodeUniform21 );
	nodeVar43 = ( vec3<f32>( clamp( nodeVar25, 0.0, 1.0 ) ) * ( render.nodeUniform14 * vec3<f32>( nodeVar42 ) ) );
	nodeVar44 = nodeVar43;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar45 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar46 = ( nodeVar44 * nodeVar45 );
	nodeVar47 = ( nodeVar24 + positionViewDirection );
	nodeVar48 = normalize( nodeVar47 );
	nodeVar49 = dot( positionViewDirection, nodeVar48 );
	nodeVar50 = clamp( nodeVar49, 0.0, 1.0 );
	nodeVar51 = exp2( ( ( ( nodeVar50 * -5.55473 ) - 6.98316 ) * nodeVar50 ) );
	nodeVar52 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar51 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar51 ) ) );
	nodeVar53 = ( vec3<f32>( 1.0 ) - nodeVar52 );
	nodeVar54 = nodeVar53;
	nodeVar55 = ( nodeVar46 * nodeVar54 );
	nodeVar56 = ( directDiffuse + nodeVar55 );
	directDiffuse = nodeVar56;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar57 = normalize( ( nodeVar24 + positionViewDirection ) );
	nodeVar58 = clamp( dot( positionViewDirection, nodeVar57 ), 0.0, 1.0 );
	nodeVar59 = exp2( ( ( ( nodeVar58 * -5.55473 ) - 6.98316 ) * nodeVar58 ) );
	nodeVar60 = ( Roughness * Roughness );
	nodeVar61 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar59 ) ) ) + vec3<f32>( ( 1.0 * nodeVar59 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar60, clamp( dot( normalView, nodeVar24 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar60, clamp( dot( normalView, nodeVar57 ), 0.0, 1.0 ) ) ) );
	nodeVar62 = ( nodeVar44 * nodeVar61 );
	nodeVar63 = ( nodeVar62 * multiScatteringCompensation );
	nodeVar64 = ( directSpecular + nodeVar63 );
	directSpecular = nodeVar64;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar65 = ( object.nodeUniform23 - object.nodeUniform24 );
	nodeVar66 = ( object.nodeUniform25 - vec3<f32>( 1.0 ) );
	nodeVar67 = ( nodeVar65 / nodeVar66 );
	nodeVar68 = ( normalWorld * nodeVar67 );
	nodeVar69 = ( nodeVar68 * vec3<f32>( 0.5 ) );
	nodeVar70 = ( v_positionWorld + nodeVar69 );
	nodeVar71 = ( nodeVar70 - object.nodeUniform24 );
	nodeVar72 = ( nodeVar71 / nodeVar65 );
	nodeVar73 = ( clamp( nodeVar72, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar66 );
	nodeVar74 = ( nodeVar73 / object.nodeUniform25 );
	nodeVar75 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform25 );
	nodeVar76 = ( nodeVar74 + nodeVar75 );
	nodeVar77 = ( nodeVar76.z * object.nodeUniform25.z );
	nodeVar78 = ( nodeVar77 + 1.0 );
	nodeVar79 = ( object.nodeUniform25.z + 2.0 );
	nodeVar80 = ( nodeVar79 * 0.0 );
	nodeVar81 = ( nodeVar78 + nodeVar80 );
	nodeVar82 = ( nodeVar79 * 7.0 );
	nodeVar83 = ( nodeVar81 / nodeVar82 );
	nodeVar84 = vec3<f32>( nodeVar76.xy, nodeVar83 );
	nodeVar85 = textureSample( nodeUniform22, nodeUniform22_sampler, nodeVar84 );
	nodeVar86 = ( nodeVar79 * 1.0 );
	nodeVar87 = ( nodeVar78 + nodeVar86 );
	nodeVar88 = ( nodeVar87 / nodeVar82 );
	nodeVar89 = vec3<f32>( nodeVar76.xy, nodeVar88 );
	nodeVar90 = textureSample( nodeUniform22, nodeUniform22_sampler, nodeVar89 );
	nodeVar91 = vec3<f32>( nodeVar85.w, nodeVar90.xy );
	nodeVar92 = ( nodeVar79 * 2.0 );
	nodeVar93 = ( nodeVar78 + nodeVar92 );
	nodeVar94 = ( nodeVar93 / nodeVar82 );
	nodeVar95 = vec3<f32>( nodeVar76.xy, nodeVar94 );
	nodeVar96 = textureSample( nodeUniform22, nodeUniform22_sampler, nodeVar95 );
	nodeVar97 = vec3<f32>( nodeVar90.zw, nodeVar96.x );
	nodeVar98 = ( nodeVar79 * 3.0 );
	nodeVar99 = ( nodeVar78 + nodeVar98 );
	nodeVar100 = ( nodeVar99 / nodeVar82 );
	nodeVar101 = vec3<f32>( nodeVar76.xy, nodeVar100 );
	nodeVar102 = textureSample( nodeUniform22, nodeUniform22_sampler, nodeVar101 );
	nodeVar103 = ( nodeVar79 * 4.0 );
	nodeVar104 = ( nodeVar78 + nodeVar103 );
	nodeVar105 = ( nodeVar104 / nodeVar82 );
	nodeVar106 = vec3<f32>( nodeVar76.xy, nodeVar105 );
	nodeVar107 = textureSample( nodeUniform22, nodeUniform22_sampler, nodeVar106 );
	nodeVar108 = vec3<f32>( nodeVar102.w, nodeVar107.xy );
	nodeVar109 = ( nodeVar79 * 5.0 );
	nodeVar110 = ( nodeVar78 + nodeVar109 );
	nodeVar111 = ( nodeVar110 / nodeVar82 );
	nodeVar112 = vec3<f32>( nodeVar76.xy, nodeVar111 );
	nodeVar113 = textureSample( nodeUniform22, nodeUniform22_sampler, nodeVar112 );
	nodeVar114 = vec3<f32>( nodeVar107.zw, nodeVar113.x );
	nodeVar115 = ( nodeVar79 * 6.0 );
	nodeVar116 = ( nodeVar78 + nodeVar115 );
	nodeVar117 = ( nodeVar116 / nodeVar82 );
	nodeVar118 = vec3<f32>( nodeVar76.xy, nodeVar117 );
	nodeVar119 = textureSample( nodeUniform22, nodeUniform22_sampler, nodeVar118 );
	nodeVar120 = array< vec3<f32>, 9 >( nodeVar85.xyz, nodeVar91, nodeVar97, nodeVar96.yzw, nodeVar102.xyz, nodeVar108, nodeVar114, nodeVar113.yzw, nodeVar119.xyz );
	nodeVar121 = ( ( ( ( ( ( ( ( ( nodeVar120[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar120[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar120[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar120[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar120[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar120[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar120[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar120[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar120[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar122 = max( nodeVar121, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar123 = ( nodeVar122 * vec3<f32>( object.nodeUniform26 ) );
	nodeVar124 = ( irradiance + nodeVar123 );
	irradiance = nodeVar124;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar125 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform27 );
	nodeVar126 = floor( nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar129 = getFace( ( object.nodeUniform28 * vec4<f32>( vec3<f32>( nodeVar128.x, ( - nodeVar128.y ), nodeVar128.z ), 1.0 ) ).xyz );
	nodeVar130 = max( ( 4.0 - nodeVar127 ), 0.0 );
	nodeVar127 = max( nodeVar127, 4.0 );
	nodeVar131 = exp2( nodeVar127 );
	nodeVar132 = ( ( getUV( ( object.nodeUniform28 * vec4<f32>( vec3<f32>( nodeVar128.x, ( - nodeVar128.y ), nodeVar128.z ), 1.0 ) ).xyz, nodeVar129 ) * vec2<f32>( ( nodeVar131 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar129 > 2.0 ) ) {

		nodeVar132.y = ( nodeVar132.y + nodeVar131 );
		nodeVar129 = ( nodeVar129 - 3.0 );
		

	}

	nodeVar132.x = ( nodeVar132.x + ( nodeVar129 * nodeVar131 ) );
	nodeVar132.x = ( nodeVar132.x + ( nodeVar130 * ( 3.0 * 16.0 ) ) );
	nodeVar132.y = ( nodeVar132.y + ( 4.0 * ( exp2( object.nodeUniform27 ) - nodeVar131 ) ) );
	nodeVar132.x = ( nodeVar132.x * object.nodeUniform30 );
	nodeVar132.y = ( nodeVar132.y * object.nodeUniform31 );
	nodeVar133 = textureSampleGrad( nodeUniform32, nodeUniform32_sampler, nodeVar132, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar134 = nodeVar133.xyz;
	nodeVar135 = fract( nodeVar125 );

	if ( ( nodeVar135 != 0.0 ) ) {

		nodeVar136 = ( nodeVar126 + 1.0 );
		nodeVar137 = getFace( ( object.nodeUniform28 * vec4<f32>( vec3<f32>( nodeVar128.x, ( - nodeVar128.y ), nodeVar128.z ), 1.0 ) ).xyz );
		nodeVar138 = max( ( 4.0 - nodeVar136 ), 0.0 );
		nodeVar136 = max( nodeVar136, 4.0 );
		nodeVar139 = exp2( nodeVar136 );
		nodeVar140 = ( ( getUV( ( object.nodeUniform28 * vec4<f32>( vec3<f32>( nodeVar128.x, ( - nodeVar128.y ), nodeVar128.z ), 1.0 ) ).xyz, nodeVar137 ) * vec2<f32>( ( nodeVar139 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar137 > 2.0 ) ) {

			nodeVar140.y = ( nodeVar140.y + nodeVar139 );
			nodeVar137 = ( nodeVar137 - 3.0 );
			

		}

		nodeVar140.x = ( nodeVar140.x + ( nodeVar137 * nodeVar139 ) );
		nodeVar140.x = ( nodeVar140.x + ( nodeVar138 * ( 3.0 * 16.0 ) ) );
		nodeVar140.y = ( nodeVar140.y + ( 4.0 * ( exp2( object.nodeUniform27 ) - nodeVar139 ) ) );
		nodeVar140.x = ( nodeVar140.x * object.nodeUniform30 );
		nodeVar140.y = ( nodeVar140.y * object.nodeUniform31 );
		nodeVar141 = textureSampleGrad( nodeUniform32, nodeUniform32_sampler, nodeVar140, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar142 = nodeVar141.xyz;
		nodeVar134 = mix( nodeVar134, nodeVar142, nodeVar135 );
		

	}

	nodeVar143 = ( radiance + ( nodeVar134 * vec3<f32>( object.nodeUniform33 ) ) );
	radiance = nodeVar143;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar144 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform27 );
	nodeVar145 = floor( nodeVar144 );
	nodeVar146 = nodeVar145;
	nodeVar147 = getFace( ( object.nodeUniform28 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar148 = max( ( 4.0 - nodeVar146 ), 0.0 );
	nodeVar146 = max( nodeVar146, 4.0 );
	nodeVar149 = exp2( nodeVar146 );
	nodeVar150 = ( ( getUV( ( object.nodeUniform28 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar147 ) * vec2<f32>( ( nodeVar149 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar147 > 2.0 ) ) {

		nodeVar150.y = ( nodeVar150.y + nodeVar149 );
		nodeVar147 = ( nodeVar147 - 3.0 );
		

	}

	nodeVar150.x = ( nodeVar150.x + ( nodeVar147 * nodeVar149 ) );
	nodeVar150.x = ( nodeVar150.x + ( nodeVar148 * ( 3.0 * 16.0 ) ) );
	nodeVar150.y = ( nodeVar150.y + ( 4.0 * ( exp2( object.nodeUniform27 ) - nodeVar149 ) ) );
	nodeVar150.x = ( nodeVar150.x * object.nodeUniform30 );
	nodeVar150.y = ( nodeVar150.y * object.nodeUniform31 );
	nodeVar151 = textureSampleGrad( nodeUniform32, nodeUniform32_sampler, nodeVar150, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar152 = nodeVar151.xyz;
	nodeVar153 = fract( nodeVar144 );

	if ( ( nodeVar153 != 0.0 ) ) {

		nodeVar154 = ( nodeVar145 + 1.0 );
		nodeVar155 = getFace( ( object.nodeUniform28 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar156 = max( ( 4.0 - nodeVar154 ), 0.0 );
		nodeVar154 = max( nodeVar154, 4.0 );
		nodeVar157 = exp2( nodeVar154 );
		nodeVar158 = ( ( getUV( ( object.nodeUniform28 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar155 ) * vec2<f32>( ( nodeVar157 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar155 > 2.0 ) ) {

			nodeVar158.y = ( nodeVar158.y + nodeVar157 );
			nodeVar155 = ( nodeVar155 - 3.0 );
			

		}

		nodeVar158.x = ( nodeVar158.x + ( nodeVar155 * nodeVar157 ) );
		nodeVar158.x = ( nodeVar158.x + ( nodeVar156 * ( 3.0 * 16.0 ) ) );
		nodeVar158.y = ( nodeVar158.y + ( 4.0 * ( exp2( object.nodeUniform27 ) - nodeVar157 ) ) );
		nodeVar158.x = ( nodeVar158.x * object.nodeUniform30 );
		nodeVar158.y = ( nodeVar158.y * object.nodeUniform31 );
		nodeVar159 = textureSampleGrad( nodeUniform32, nodeUniform32_sampler, nodeVar158, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar160 = nodeVar159.xyz;
		nodeVar152 = mix( nodeVar152, nodeVar160, nodeVar153 );
		

	}

	nodeVar161 = ( iblIrradiance + ( ( nodeVar152 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform33 ) ) );
	iblIrradiance = nodeVar161;
	nodeVar162 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar163 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar164 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar165 = ( SpecularF90 * dfg.y );
	nodeVar166 = ( nodeVar164 + vec3<f32>( nodeVar165 ) );
	nodeVar167 = ( nodeVar162 + nodeVar166 );
	nodeVar162 = nodeVar167;
	nodeVar168 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar169 = nodeVar168;
	nodeVar170 = ( nodeVar169 * vec3<f32>( 0.047619 ) );
	nodeVar171 = ( SpecularColor + nodeVar170 );
	nodeVar172 = ( nodeVar166 * nodeVar171 );
	nodeVar173 = ( dfg.x + dfg.y );
	nodeVar174 = ( 1.0 - nodeVar173 );
	nodeVar175 = nodeVar174;
	nodeVar176 = ( vec3<f32>( nodeVar175 ) * nodeVar171 );
	nodeVar177 = ( vec3<f32>( 1.0 ) - nodeVar176 );
	nodeVar178 = nodeVar177;
	nodeVar179 = ( nodeVar172 / nodeVar178 );
	nodeVar180 = ( nodeVar179 * vec3<f32>( nodeVar175 ) );
	nodeVar181 = ( nodeVar163 + nodeVar180 );
	nodeVar163 = nodeVar181;
	nodeVar182 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar183 = ( irradiance * nodeVar182 );
	nodeVar184 = ( nodeVar162 + nodeVar163 );
	nodeVar185 = ( vec3<f32>( 1.0 ) - nodeVar184 );
	nodeVar186 = nodeVar185;
	nodeVar187 = ( nodeVar183 * nodeVar186 );
	nodeVar188 = nodeVar187;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar189 = ( indirectDiffuse + nodeVar188 );
	indirectDiffuse = nodeVar189;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar190 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar191 = ( SpecularF90 * dfg.y );
	nodeVar192 = ( nodeVar190 + vec3<f32>( nodeVar191 ) );
	nodeVar193 = ( singleScatteringDielectric + nodeVar192 );
	singleScatteringDielectric = nodeVar193;
	nodeVar194 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar195 = nodeVar194;
	nodeVar196 = ( nodeVar195 * vec3<f32>( 0.047619 ) );
	nodeVar197 = ( SpecularColor + nodeVar196 );
	nodeVar198 = ( nodeVar192 * nodeVar197 );
	nodeVar199 = ( dfg.x + dfg.y );
	nodeVar200 = ( 1.0 - nodeVar199 );
	nodeVar201 = nodeVar200;
	nodeVar202 = ( vec3<f32>( nodeVar201 ) * nodeVar197 );
	nodeVar203 = ( vec3<f32>( 1.0 ) - nodeVar202 );
	nodeVar204 = nodeVar203;
	nodeVar205 = ( nodeVar198 / nodeVar204 );
	nodeVar206 = ( nodeVar205 * vec3<f32>( nodeVar201 ) );
	nodeVar207 = ( multiScatteringDielectric + nodeVar206 );
	multiScatteringDielectric = nodeVar207;
	nodeVar208 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar209 = ( SpecularF90 * dfg.y );
	nodeVar210 = ( nodeVar208 + vec3<f32>( nodeVar209 ) );
	nodeVar211 = ( singleScatteringMetallic + nodeVar210 );
	singleScatteringMetallic = nodeVar211;
	nodeVar212 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar213 = nodeVar212;
	nodeVar214 = ( nodeVar213 * vec3<f32>( 0.047619 ) );
	nodeVar215 = ( DiffuseColor.xyz + nodeVar214 );
	nodeVar216 = ( nodeVar210 * nodeVar215 );
	nodeVar217 = ( dfg.x + dfg.y );
	nodeVar218 = ( 1.0 - nodeVar217 );
	nodeVar219 = nodeVar218;
	nodeVar220 = ( vec3<f32>( nodeVar219 ) * nodeVar215 );
	nodeVar221 = ( vec3<f32>( 1.0 ) - nodeVar220 );
	nodeVar222 = nodeVar221;
	nodeVar223 = ( nodeVar216 / nodeVar222 );
	nodeVar224 = ( nodeVar223 * vec3<f32>( nodeVar219 ) );
	nodeVar225 = ( multiScatteringMetallic + nodeVar224 );
	multiScatteringMetallic = nodeVar225;
	nodeVar226 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar227 = ( radiance * nodeVar226 );
	nodeVar228 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar229 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar230 = ( nodeVar228 * nodeVar229 );
	nodeVar231 = ( nodeVar227 + nodeVar230 );
	nodeVar232 = nodeVar231;
	nodeVar233 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar234 = ( vec3<f32>( 1.0 ) - nodeVar233 );
	nodeVar235 = nodeVar234;
	nodeVar236 = ( DiffuseContribution * nodeVar235 );
	nodeVar237 = ( nodeVar236 * nodeVar229 );
	nodeVar238 = nodeVar237;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar239 = ( indirectSpecular + nodeVar232 );
	indirectSpecular = nodeVar239;
	nodeVar240 = ( indirectDiffuse + nodeVar238 );
	indirectDiffuse = nodeVar240;
	ambientOcclusion = 1.0;
	nodeVar241 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar241;
	nodeVar242 = dot( normalView, positionViewDirection );
	nodeVar243 = ( clamp( nodeVar242, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar244 = ( Roughness * -16.0 );
	nodeVar245 = ( 1.0 - nodeVar244 );
	nodeVar246 = nodeVar245;
	nodeVar247 = ( - nodeVar246 );
	nodeVar248 = exp2( nodeVar247 );
	nodeVar249 = pow( nodeVar243, nodeVar248 );
	nodeVar250 = ( 1.0 - nodeVar249 );
	nodeVar251 = nodeVar250;
	nodeVar252 = ( ambientOcclusion - nodeVar251 );
	nodeVar253 = ( indirectSpecular * vec3<f32>( clamp( nodeVar252, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar253;
	nodeVar254 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar254;
	nodeVar255 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar255;
	nodeVar256 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar256;
	nodeVar257 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar257;

	// result

	output.color = nodeVar257;

	return output;

}
