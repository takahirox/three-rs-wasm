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
@binding( 5 ) @group( 1 ) var nodeUniform27_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform27 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform2 : u32,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform28 : f32
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
var<private> radiance : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec2<f32>;
var<private> nodeVar73 : vec4<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : vec2<f32>;
var<private> nodeVar81 : vec4<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec2<f32>;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : vec2<f32>;
var<private> nodeVar99 : vec4<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
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
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : f32;
var<private> nodeVar186 : f32;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : f32;
var<private> nodeVar190 : f32;
var<private> nodeVar191 : f32;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec4<f32>;

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
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar65 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform22 );
	nodeVar66 = floor( nodeVar65 );
	nodeVar67 = nodeVar66;
	nodeVar68 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar69 = getFace( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( nodeVar68.x, ( - nodeVar68.y ), nodeVar68.z ), 1.0 ) ).xyz );
	nodeVar70 = max( ( 4.0 - nodeVar67 ), 0.0 );
	nodeVar67 = max( nodeVar67, 4.0 );
	nodeVar71 = exp2( nodeVar67 );
	nodeVar72 = ( ( getUV( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( nodeVar68.x, ( - nodeVar68.y ), nodeVar68.z ), 1.0 ) ).xyz, nodeVar69 ) * vec2<f32>( ( nodeVar71 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar69 > 2.0 ) ) {

		nodeVar72.y = ( nodeVar72.y + nodeVar71 );
		nodeVar69 = ( nodeVar69 - 3.0 );
		

	}

	nodeVar72.x = ( nodeVar72.x + ( nodeVar69 * nodeVar71 ) );
	nodeVar72.x = ( nodeVar72.x + ( nodeVar70 * ( 3.0 * 16.0 ) ) );
	nodeVar72.y = ( nodeVar72.y + ( 4.0 * ( exp2( object.nodeUniform22 ) - nodeVar71 ) ) );
	nodeVar72.x = ( nodeVar72.x * object.nodeUniform25 );
	nodeVar72.y = ( nodeVar72.y * object.nodeUniform26 );
	nodeVar73 = textureSampleGrad( nodeUniform27, nodeUniform27_sampler, nodeVar72, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar74 = nodeVar73.xyz;
	nodeVar75 = fract( nodeVar65 );

	if ( ( nodeVar75 != 0.0 ) ) {

		nodeVar76 = ( nodeVar66 + 1.0 );
		nodeVar77 = getFace( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( nodeVar68.x, ( - nodeVar68.y ), nodeVar68.z ), 1.0 ) ).xyz );
		nodeVar78 = max( ( 4.0 - nodeVar76 ), 0.0 );
		nodeVar76 = max( nodeVar76, 4.0 );
		nodeVar79 = exp2( nodeVar76 );
		nodeVar80 = ( ( getUV( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( nodeVar68.x, ( - nodeVar68.y ), nodeVar68.z ), 1.0 ) ).xyz, nodeVar77 ) * vec2<f32>( ( nodeVar79 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar77 > 2.0 ) ) {

			nodeVar80.y = ( nodeVar80.y + nodeVar79 );
			nodeVar77 = ( nodeVar77 - 3.0 );
			

		}

		nodeVar80.x = ( nodeVar80.x + ( nodeVar77 * nodeVar79 ) );
		nodeVar80.x = ( nodeVar80.x + ( nodeVar78 * ( 3.0 * 16.0 ) ) );
		nodeVar80.y = ( nodeVar80.y + ( 4.0 * ( exp2( object.nodeUniform22 ) - nodeVar79 ) ) );
		nodeVar80.x = ( nodeVar80.x * object.nodeUniform25 );
		nodeVar80.y = ( nodeVar80.y * object.nodeUniform26 );
		nodeVar81 = textureSampleGrad( nodeUniform27, nodeUniform27_sampler, nodeVar80, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar82 = nodeVar81.xyz;
		nodeVar74 = mix( nodeVar74, nodeVar82, nodeVar75 );
		

	}

	nodeVar83 = ( radiance + ( nodeVar74 * vec3<f32>( object.nodeUniform28 ) ) );
	radiance = nodeVar83;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar84 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform22 );
	nodeVar85 = floor( nodeVar84 );
	nodeVar86 = nodeVar85;
	nodeVar87 = getFace( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar88 = max( ( 4.0 - nodeVar86 ), 0.0 );
	nodeVar86 = max( nodeVar86, 4.0 );
	nodeVar89 = exp2( nodeVar86 );
	nodeVar90 = ( ( getUV( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar87 ) * vec2<f32>( ( nodeVar89 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar87 > 2.0 ) ) {

		nodeVar90.y = ( nodeVar90.y + nodeVar89 );
		nodeVar87 = ( nodeVar87 - 3.0 );
		

	}

	nodeVar90.x = ( nodeVar90.x + ( nodeVar87 * nodeVar89 ) );
	nodeVar90.x = ( nodeVar90.x + ( nodeVar88 * ( 3.0 * 16.0 ) ) );
	nodeVar90.y = ( nodeVar90.y + ( 4.0 * ( exp2( object.nodeUniform22 ) - nodeVar89 ) ) );
	nodeVar90.x = ( nodeVar90.x * object.nodeUniform25 );
	nodeVar90.y = ( nodeVar90.y * object.nodeUniform26 );
	nodeVar91 = textureSampleGrad( nodeUniform27, nodeUniform27_sampler, nodeVar90, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar92 = nodeVar91.xyz;
	nodeVar93 = fract( nodeVar84 );

	if ( ( nodeVar93 != 0.0 ) ) {

		nodeVar94 = ( nodeVar85 + 1.0 );
		nodeVar95 = getFace( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar96 = max( ( 4.0 - nodeVar94 ), 0.0 );
		nodeVar94 = max( nodeVar94, 4.0 );
		nodeVar97 = exp2( nodeVar94 );
		nodeVar98 = ( ( getUV( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar95 ) * vec2<f32>( ( nodeVar97 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar95 > 2.0 ) ) {

			nodeVar98.y = ( nodeVar98.y + nodeVar97 );
			nodeVar95 = ( nodeVar95 - 3.0 );
			

		}

		nodeVar98.x = ( nodeVar98.x + ( nodeVar95 * nodeVar97 ) );
		nodeVar98.x = ( nodeVar98.x + ( nodeVar96 * ( 3.0 * 16.0 ) ) );
		nodeVar98.y = ( nodeVar98.y + ( 4.0 * ( exp2( object.nodeUniform22 ) - nodeVar97 ) ) );
		nodeVar98.x = ( nodeVar98.x * object.nodeUniform25 );
		nodeVar98.y = ( nodeVar98.y * object.nodeUniform26 );
		nodeVar99 = textureSampleGrad( nodeUniform27, nodeUniform27_sampler, nodeVar98, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar100 = nodeVar99.xyz;
		nodeVar92 = mix( nodeVar92, nodeVar100, nodeVar93 );
		

	}

	nodeVar101 = ( iblIrradiance + ( ( nodeVar92 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform28 ) ) );
	iblIrradiance = nodeVar101;
	nodeVar102 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar103 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar104 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar105 = ( SpecularF90 * dfg.y );
	nodeVar106 = ( nodeVar104 + vec3<f32>( nodeVar105 ) );
	nodeVar107 = ( nodeVar102 + nodeVar106 );
	nodeVar102 = nodeVar107;
	nodeVar108 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( nodeVar109 * vec3<f32>( 0.047619 ) );
	nodeVar111 = ( SpecularColor + nodeVar110 );
	nodeVar112 = ( nodeVar106 * nodeVar111 );
	nodeVar113 = ( dfg.x + dfg.y );
	nodeVar114 = ( 1.0 - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( vec3<f32>( nodeVar115 ) * nodeVar111 );
	nodeVar117 = ( vec3<f32>( 1.0 ) - nodeVar116 );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar112 / nodeVar118 );
	nodeVar120 = ( nodeVar119 * vec3<f32>( nodeVar115 ) );
	nodeVar121 = ( nodeVar103 + nodeVar120 );
	nodeVar103 = nodeVar121;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar122 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar123 = ( irradiance * nodeVar122 );
	nodeVar124 = ( nodeVar102 + nodeVar103 );
	nodeVar125 = ( vec3<f32>( 1.0 ) - nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( nodeVar123 * nodeVar126 );
	nodeVar128 = nodeVar127;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar129 = ( indirectDiffuse + nodeVar128 );
	indirectDiffuse = nodeVar129;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar130 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar131 = ( SpecularF90 * dfg.y );
	nodeVar132 = ( nodeVar130 + vec3<f32>( nodeVar131 ) );
	nodeVar133 = ( singleScatteringDielectric + nodeVar132 );
	singleScatteringDielectric = nodeVar133;
	nodeVar134 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar135 = nodeVar134;
	nodeVar136 = ( nodeVar135 * vec3<f32>( 0.047619 ) );
	nodeVar137 = ( SpecularColor + nodeVar136 );
	nodeVar138 = ( nodeVar132 * nodeVar137 );
	nodeVar139 = ( dfg.x + dfg.y );
	nodeVar140 = ( 1.0 - nodeVar139 );
	nodeVar141 = nodeVar140;
	nodeVar142 = ( vec3<f32>( nodeVar141 ) * nodeVar137 );
	nodeVar143 = ( vec3<f32>( 1.0 ) - nodeVar142 );
	nodeVar144 = nodeVar143;
	nodeVar145 = ( nodeVar138 / nodeVar144 );
	nodeVar146 = ( nodeVar145 * vec3<f32>( nodeVar141 ) );
	nodeVar147 = ( multiScatteringDielectric + nodeVar146 );
	multiScatteringDielectric = nodeVar147;
	nodeVar148 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar149 = ( SpecularF90 * dfg.y );
	nodeVar150 = ( nodeVar148 + vec3<f32>( nodeVar149 ) );
	nodeVar151 = ( singleScatteringMetallic + nodeVar150 );
	singleScatteringMetallic = nodeVar151;
	nodeVar152 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar153 = nodeVar152;
	nodeVar154 = ( nodeVar153 * vec3<f32>( 0.047619 ) );
	nodeVar155 = ( DiffuseColor.xyz + nodeVar154 );
	nodeVar156 = ( nodeVar150 * nodeVar155 );
	nodeVar157 = ( dfg.x + dfg.y );
	nodeVar158 = ( 1.0 - nodeVar157 );
	nodeVar159 = nodeVar158;
	nodeVar160 = ( vec3<f32>( nodeVar159 ) * nodeVar155 );
	nodeVar161 = ( vec3<f32>( 1.0 ) - nodeVar160 );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( nodeVar156 / nodeVar162 );
	nodeVar164 = ( nodeVar163 * vec3<f32>( nodeVar159 ) );
	nodeVar165 = ( multiScatteringMetallic + nodeVar164 );
	multiScatteringMetallic = nodeVar165;
	nodeVar166 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar167 = ( radiance * nodeVar166 );
	nodeVar168 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar169 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar170 = ( nodeVar168 * nodeVar169 );
	nodeVar171 = ( nodeVar167 + nodeVar170 );
	nodeVar172 = nodeVar171;
	nodeVar173 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar174 = ( vec3<f32>( 1.0 ) - nodeVar173 );
	nodeVar175 = nodeVar174;
	nodeVar176 = ( DiffuseContribution * nodeVar175 );
	nodeVar177 = ( nodeVar176 * nodeVar169 );
	nodeVar178 = nodeVar177;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar179 = ( indirectSpecular + nodeVar172 );
	indirectSpecular = nodeVar179;
	nodeVar180 = ( indirectDiffuse + nodeVar178 );
	indirectDiffuse = nodeVar180;
	ambientOcclusion = 1.0;
	nodeVar181 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar181;
	nodeVar182 = dot( normalView, positionViewDirection );
	nodeVar183 = ( clamp( nodeVar182, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar184 = ( Roughness * -16.0 );
	nodeVar185 = ( 1.0 - nodeVar184 );
	nodeVar186 = nodeVar185;
	nodeVar187 = ( - nodeVar186 );
	nodeVar188 = exp2( nodeVar187 );
	nodeVar189 = pow( nodeVar183, nodeVar188 );
	nodeVar190 = ( 1.0 - nodeVar189 );
	nodeVar191 = nodeVar190;
	nodeVar192 = ( ambientOcclusion - nodeVar191 );
	nodeVar193 = ( indirectSpecular * vec3<f32>( clamp( nodeVar192, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar193;
	nodeVar194 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar194;
	nodeVar195 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar195;
	nodeVar196 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar196;
	nodeVar197 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar197;

	// result

	output.color = nodeVar197;

	return output;

}
