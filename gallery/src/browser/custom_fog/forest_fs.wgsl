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
@binding( 3 ) @group( 1 ) var nodeUniform24_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform24 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform33_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform33 : texture_2d<f32>;

struct objectStruct {
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform37 : f32,
	nodeUniform38 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform27 : f32,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_nodeVar6 : vec3<f32>;
var<private> NORMAL_nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec2<f32>;
var<private> nodeVar71 : vec4<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec2<f32>;
var<private> nodeVar79 : vec4<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec2<f32>;
var<private> nodeVar89 : vec4<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec2<f32>;
var<private> nodeVar97 : vec4<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
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
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : f32;
var<private> nodeVar186 : f32;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : f32;
var<private> nodeVar190 : f32;
var<private> nodeVar191 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : vec4<f32>;

// codes
fn mx_floor ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn mx_fade ( t : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = t;

	return ( ( ( nodeVar0 * nodeVar0 ) * nodeVar0 ) * ( ( nodeVar0 * ( ( nodeVar0 * 6.0 ) - 15.0 ) ) + 10.0 ) );

}


fn mx_rotl32 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn mx_bjfinal ( a : u32, b : u32, c : u32 ) -> u32 {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = c;
	nodeVar1 = b;
	nodeVar2 = a;
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 14 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - mx_rotl32( nodeVar0, 11 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - mx_rotl32( nodeVar2, 25 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 16 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - mx_rotl32( nodeVar0, 4 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - mx_rotl32( nodeVar2, 14 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 24 ) );

	return nodeVar0;

}


fn mx_hash_int_2 ( x : i32, y : i32, z : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;
	var nodeVar6 : u32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = 3u;
	nodeVar4 = 0u;
	nodeVar5 = 0u;
	nodeVar6 = 0u;
	nodeVar6 = ( ( 3735928559u + ( nodeVar3 << 2u ) ) + 13u );
	nodeVar5 = nodeVar6;
	nodeVar4 = nodeVar5;
	nodeVar4 = ( nodeVar4 + u32( nodeVar2 ) );
	nodeVar5 = ( nodeVar5 + u32( nodeVar1 ) );
	nodeVar6 = ( nodeVar6 + u32( nodeVar0 ) );

	return mx_bjfinal( nodeVar4, nodeVar5, nodeVar6 );

}


fn mx_select ( b : bool, t : f32, f : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : bool;
	var nodeVar3 : f32;

	nodeVar0 = f;
	nodeVar1 = t;
	nodeVar2 = b;

	return select( nodeVar0, nodeVar1, nodeVar2 );

}


fn mx_negate_if ( val : f32, b : bool ) -> f32 {

	var nodeVar0 : bool;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = b;
	nodeVar1 = val;

	return select( nodeVar1, ( - nodeVar1 ), nodeVar0 );

}


fn mx_gradient_float_1 ( hash : u32, x : f32, y : f32, z : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = hash;
	nodeVar4 = ( nodeVar3 & 15u );
	nodeVar5 = mx_select( ( nodeVar4 < 8u ), nodeVar2, nodeVar1 );
	nodeVar6 = mx_select( ( nodeVar4 < 4u ), nodeVar1, mx_select( ( ( nodeVar4 == 12u ) || ( nodeVar4 == 14u ) ), nodeVar2, nodeVar0 ) );

	return ( mx_negate_if( nodeVar5, bool( ( nodeVar4 & 1u ) ) ) + mx_negate_if( nodeVar6, bool( ( nodeVar4 & 2u ) ) ) );

}


fn mx_trilerp_0 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, v4 : f32, v5 : f32, v6 : f32, v7 : f32, s : f32, t : f32, r : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : f32;

	nodeVar0 = r;
	nodeVar1 = t;
	nodeVar2 = s;
	nodeVar3 = v7;
	nodeVar4 = v6;
	nodeVar5 = v5;
	nodeVar6 = v4;
	nodeVar7 = v3;
	nodeVar8 = v2;
	nodeVar9 = v1;
	nodeVar10 = v0;
	nodeVar11 = ( 1.0 - nodeVar2 );
	nodeVar12 = ( 1.0 - nodeVar1 );
	nodeVar13 = ( 1.0 - nodeVar0 );

	return ( ( nodeVar13 * ( ( nodeVar12 * ( ( nodeVar10 * nodeVar11 ) + ( nodeVar9 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar8 * nodeVar11 ) + ( nodeVar7 * nodeVar2 ) ) ) ) ) + ( nodeVar0 * ( ( nodeVar12 * ( ( nodeVar6 * nodeVar11 ) + ( nodeVar5 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar4 * nodeVar11 ) + ( nodeVar3 * nodeVar2 ) ) ) ) ) );

}


fn mx_gradient_scale3d_0 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.982 * nodeVar0 );

}


fn mx_perlin_noise_float_1 ( p : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : f32;

	nodeVar0 = p;
	nodeVar1 = 0;
	nodeVar2 = 0;
	nodeVar3 = 0;
	nodeVar4 = nodeVar0.x;
	nodeVar1 = mx_floor( nodeVar4 );
	nodeVar5 = ( nodeVar4 - f32( nodeVar1 ) );
	nodeVar6 = nodeVar0.y;
	nodeVar2 = mx_floor( nodeVar6 );
	nodeVar7 = ( nodeVar6 - f32( nodeVar2 ) );
	nodeVar8 = nodeVar0.z;
	nodeVar3 = mx_floor( nodeVar8 );
	nodeVar9 = ( nodeVar8 - f32( nodeVar3 ) );
	nodeVar10 = mx_fade( nodeVar5 );
	nodeVar11 = mx_fade( nodeVar7 );
	nodeVar12 = mx_fade( nodeVar9 );
	nodeVar13 = mx_trilerp_0( mx_gradient_float_1( mx_hash_int_2( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return mx_gradient_scale3d_0( nodeVar13 );

}


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


fn tri ( x : f32 ) -> f32 {

	


	return abs( ( fract( x ) - 0.5 ) );

}


fn tri3 ( p : vec3<f32> ) -> vec3<f32> {

	


	return vec3<f32>( tri( ( p.z + tri( ( p.y * 1.0 ) ) ) ), tri( ( p.z + tri( ( p.x * 1.0 ) ) ) ), tri( ( p.y + tri( ( p.x * 1.0 ) ) ) ) );

}


fn triNoise3D ( position : vec3<f32>, speed : f32, time : f32 ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : vec3<f32>;
	var nodeVar5 : f32;

	nodeVar0 = position;
	nodeVar1 = 1.4;
	nodeVar2 = 0.0;
	nodeVar3 = nodeVar0;

	for ( var i : f32 = 0.0; i <= 3.0; i += 1. ) {

		nodeVar4 = tri3( ( nodeVar3 * vec3<f32>( 2.0 ) ) );
		nodeVar0 = ( nodeVar0 + ( nodeVar4 + vec3<f32>( ( time * ( 0.1 * speed ) ) ) ) );
		nodeVar3 = ( nodeVar3 * vec3<f32>( 1.8 ) );
		nodeVar1 = ( nodeVar1 * 1.5 );
		nodeVar0 = ( nodeVar0 * vec3<f32>( 1.2 ) );
		nodeVar5 = tri( ( nodeVar0.z + tri( ( nodeVar0.x + tri( nodeVar0.y ) ) ) ) );
		nodeVar2 = ( nodeVar2 + ( nodeVar5 / nodeVar1 ) );
		nodeVar3 = ( nodeVar3 + vec3<f32>( 0.14 ) );

	}


	return nodeVar2;

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) nodeVarying7 : f32,
	@location( 5 ) nodeVarying8 : f32,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = 0.0;
	nodeVar2 = smoothstep( 280.0, 25.0, distance( v_positionWorld, object.nodeUniform8 ) );

	if ( ( nodeVar2 > 0.01 ) ) {

		nodeVar1 = ( ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 0.9 ) ) ) * 1.0 ) + 0.0 ) + ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 3.1 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) ) * nodeVar2 );
		

	}

	DiffuseColor = vec4<f32>( mix( mix( vec3<f32>( 0.012286488353353374, 0.033104766565152086, 0.009134058699157796 ), vec3<f32>( 0.02732089163382382, 0.057805430183792694, 0.014443843592229466 ), nodeVarying7 ), mix( vec3<f32>( 0.0722718506743852, 0.14412847084818123, 0.02732089163382382 ), vec3<f32>( 0.15592646369776456, 0.2541520943200296, 0.05126945836711539 ), nodeVarying7 ), clamp( ( ( ( nodeVarying8 * 0.5 ) + 0.32 ) + ( nodeVar1 * 0.18 ) ), 0.0, 1.0 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform9 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform10;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar3 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform11, 0.0525 ) + max( max( nodeVar3.x, nodeVar3.y ), nodeVar3.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform10 ) ) );
	EmissiveColor = ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) );
	nodeVar4 = dpdx( v_positionView );
	nodeVar5 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_nodeVar6 = cross( nodeVar5, NORMAL_normalView );
	NORMAL_nodeVar7 = dot( nodeVar4, NORMAL_nodeVar6 );
	nodeVar8 = ( nodeVar1 * 0.22 );
	nodeVar9 = ( vec3<f32>( sign( NORMAL_nodeVar7 ) ) * ( ( vec3<f32>( dpdx( nodeVar8 ) ) * NORMAL_nodeVar6 ) + ( vec3<f32>( - dpdy( nodeVar8 ) ) * cross( NORMAL_normalView, nodeVar4 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeVar7 ) ) * NORMAL_normalView ) - nodeVar9 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar10 = dot( normalView, positionViewDirection );
	nodeVar11 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( nodeVar10, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar11;
	nodeVar12 = ( dfg.x + dfg.y );
	nodeVar13 = ( 1.0 / nodeVar12 );
	nodeVar14 = nodeVar13;
	nodeVar15 = ( nodeVar14 - 1.0 );
	nodeVar16 = ( SpecularColorBlended * vec3<f32>( nodeVar15 ) );
	nodeVar17 = ( nodeVar16 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar17;
	nodeVar18 = ( render.nodeUniform18 - render.nodeUniform19 );
	nodeVar19 = vec4<f32>( nodeVar18, 0.0 );
	nodeVar20 = ( render.cameraViewMatrix * nodeVar19 );
	nodeVar21 = normalize( nodeVar20.xyz );
	nodeVar22 = nodeVar21;
	nodeVar23 = dot( normalView, nodeVar22 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar25 = ( render.nodeUniform21 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform22 ) ) ), 1.0 ) );
	nodeVar26 = ( nodeVar25.xyz / vec3<f32>( nodeVar25.w ) );
	nodeVar27 = vec3<f32>( nodeVar26.x, ( 1.0 - nodeVar26.y ), ( nodeVar26.z + render.nodeUniform23 ) );

	if ( ( ( ( ( ( nodeVar27.x >= 0.0 ) && ( nodeVar27.x <= 1.0 ) ) && ( nodeVar27.y >= 0.0 ) ) && ( nodeVar27.y <= 1.0 ) ) && ( nodeVar27.z <= 1.0 ) ) ) {

		nodeVar28 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar29 = ( render.nodeUniform25 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform26 ).x );
		nodeVar30 = ( nodeVar27.xy + ( vogelDiskSample( 0, 5, nodeVar28 ) * vec2<f32>( nodeVar29 ) ) );
		nodeVar31 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeVar30, nodeVar27.z );
		nodeVar32 = ( nodeVar27.xy + ( vogelDiskSample( 1, 5, nodeVar28 ) * vec2<f32>( nodeVar29 ) ) );
		nodeVar33 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeVar32, nodeVar27.z );
		nodeVar34 = ( nodeVar27.xy + ( vogelDiskSample( 2, 5, nodeVar28 ) * vec2<f32>( nodeVar29 ) ) );
		nodeVar35 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeVar34, nodeVar27.z );
		nodeVar36 = ( nodeVar27.xy + ( vogelDiskSample( 3, 5, nodeVar28 ) * vec2<f32>( nodeVar29 ) ) );
		nodeVar37 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeVar36, nodeVar27.z );
		nodeVar38 = ( nodeVar27.xy + ( vogelDiskSample( 4, 5, nodeVar28 ) * vec2<f32>( nodeVar29 ) ) );
		nodeVar39 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeVar38, nodeVar27.z );
		nodeVar24 = ( ( ( ( ( nodeVar31 + nodeVar33 ) + nodeVar35 ) + nodeVar37 ) + nodeVar39 ) * 0.2 );

	} else {

		nodeVar24 = 1.0;

	}

	nodeVar40 = mix( 1.0, nodeVar24, render.nodeUniform27 );
	nodeVar41 = ( vec3<f32>( clamp( nodeVar23, 0.0, 1.0 ) ) * ( render.nodeUniform20 * vec3<f32>( nodeVar40 ) ) );
	nodeVar42 = nodeVar41;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar43 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar44 = ( nodeVar42 * nodeVar43 );
	nodeVar45 = ( nodeVar22 + positionViewDirection );
	nodeVar46 = normalize( nodeVar45 );
	nodeVar47 = dot( positionViewDirection, nodeVar46 );
	nodeVar48 = clamp( nodeVar47, 0.0, 1.0 );
	nodeVar49 = exp2( ( ( ( nodeVar48 * -5.55473 ) - 6.98316 ) * nodeVar48 ) );
	nodeVar50 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar49 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar49 ) ) );
	nodeVar51 = ( vec3<f32>( 1.0 ) - nodeVar50 );
	nodeVar52 = nodeVar51;
	nodeVar53 = ( nodeVar44 * nodeVar52 );
	nodeVar54 = ( directDiffuse + nodeVar53 );
	directDiffuse = nodeVar54;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = normalize( ( nodeVar22 + positionViewDirection ) );
	nodeVar56 = clamp( dot( positionViewDirection, nodeVar55 ), 0.0, 1.0 );
	nodeVar57 = exp2( ( ( ( nodeVar56 * -5.55473 ) - 6.98316 ) * nodeVar56 ) );
	nodeVar58 = ( Roughness * Roughness );
	nodeVar59 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar57 ) ) ) + vec3<f32>( ( 1.0 * nodeVar57 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar58, clamp( dot( normalView, nodeVar22 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar58, clamp( dot( normalView, nodeVar55 ), 0.0, 1.0 ) ) ) );
	nodeVar60 = ( nodeVar42 * nodeVar59 );
	nodeVar61 = ( nodeVar60 * multiScatteringCompensation );
	nodeVar62 = ( directSpecular + nodeVar61 );
	directSpecular = nodeVar62;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar63 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform28 );
	nodeVar64 = floor( nodeVar63 );
	nodeVar65 = nodeVar64;
	nodeVar66 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar67 = getFace( ( object.nodeUniform29 * vec4<f32>( vec3<f32>( nodeVar66.x, ( - nodeVar66.y ), nodeVar66.z ), 1.0 ) ).xyz );
	nodeVar68 = max( ( 4.0 - nodeVar65 ), 0.0 );
	nodeVar65 = max( nodeVar65, 4.0 );
	nodeVar69 = exp2( nodeVar65 );
	nodeVar70 = ( ( getUV( ( object.nodeUniform29 * vec4<f32>( vec3<f32>( nodeVar66.x, ( - nodeVar66.y ), nodeVar66.z ), 1.0 ) ).xyz, nodeVar67 ) * vec2<f32>( ( nodeVar69 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar67 > 2.0 ) ) {

		nodeVar70.y = ( nodeVar70.y + nodeVar69 );
		nodeVar67 = ( nodeVar67 - 3.0 );
		

	}

	nodeVar70.x = ( nodeVar70.x + ( nodeVar67 * nodeVar69 ) );
	nodeVar70.x = ( nodeVar70.x + ( nodeVar68 * ( 3.0 * 16.0 ) ) );
	nodeVar70.y = ( nodeVar70.y + ( 4.0 * ( exp2( object.nodeUniform28 ) - nodeVar69 ) ) );
	nodeVar70.x = ( nodeVar70.x * object.nodeUniform31 );
	nodeVar70.y = ( nodeVar70.y * object.nodeUniform32 );
	nodeVar71 = textureSampleGrad( nodeUniform33, nodeUniform33_sampler, nodeVar70, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar72 = nodeVar71.xyz;
	nodeVar73 = fract( nodeVar63 );

	if ( ( nodeVar73 != 0.0 ) ) {

		nodeVar74 = ( nodeVar64 + 1.0 );
		nodeVar75 = getFace( ( object.nodeUniform29 * vec4<f32>( vec3<f32>( nodeVar66.x, ( - nodeVar66.y ), nodeVar66.z ), 1.0 ) ).xyz );
		nodeVar76 = max( ( 4.0 - nodeVar74 ), 0.0 );
		nodeVar74 = max( nodeVar74, 4.0 );
		nodeVar77 = exp2( nodeVar74 );
		nodeVar78 = ( ( getUV( ( object.nodeUniform29 * vec4<f32>( vec3<f32>( nodeVar66.x, ( - nodeVar66.y ), nodeVar66.z ), 1.0 ) ).xyz, nodeVar75 ) * vec2<f32>( ( nodeVar77 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar75 > 2.0 ) ) {

			nodeVar78.y = ( nodeVar78.y + nodeVar77 );
			nodeVar75 = ( nodeVar75 - 3.0 );
			

		}

		nodeVar78.x = ( nodeVar78.x + ( nodeVar75 * nodeVar77 ) );
		nodeVar78.x = ( nodeVar78.x + ( nodeVar76 * ( 3.0 * 16.0 ) ) );
		nodeVar78.y = ( nodeVar78.y + ( 4.0 * ( exp2( object.nodeUniform28 ) - nodeVar77 ) ) );
		nodeVar78.x = ( nodeVar78.x * object.nodeUniform31 );
		nodeVar78.y = ( nodeVar78.y * object.nodeUniform32 );
		nodeVar79 = textureSampleGrad( nodeUniform33, nodeUniform33_sampler, nodeVar78, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar80 = nodeVar79.xyz;
		nodeVar72 = mix( nodeVar72, nodeVar80, nodeVar73 );
		

	}

	nodeVar81 = ( radiance + ( nodeVar72 * vec3<f32>( object.nodeUniform34 ) ) );
	radiance = nodeVar81;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar82 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform28 );
	nodeVar83 = floor( nodeVar82 );
	nodeVar84 = nodeVar83;
	nodeVar85 = getFace( ( object.nodeUniform29 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar86 = max( ( 4.0 - nodeVar84 ), 0.0 );
	nodeVar84 = max( nodeVar84, 4.0 );
	nodeVar87 = exp2( nodeVar84 );
	nodeVar88 = ( ( getUV( ( object.nodeUniform29 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar85 ) * vec2<f32>( ( nodeVar87 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar85 > 2.0 ) ) {

		nodeVar88.y = ( nodeVar88.y + nodeVar87 );
		nodeVar85 = ( nodeVar85 - 3.0 );
		

	}

	nodeVar88.x = ( nodeVar88.x + ( nodeVar85 * nodeVar87 ) );
	nodeVar88.x = ( nodeVar88.x + ( nodeVar86 * ( 3.0 * 16.0 ) ) );
	nodeVar88.y = ( nodeVar88.y + ( 4.0 * ( exp2( object.nodeUniform28 ) - nodeVar87 ) ) );
	nodeVar88.x = ( nodeVar88.x * object.nodeUniform31 );
	nodeVar88.y = ( nodeVar88.y * object.nodeUniform32 );
	nodeVar89 = textureSampleGrad( nodeUniform33, nodeUniform33_sampler, nodeVar88, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar90 = nodeVar89.xyz;
	nodeVar91 = fract( nodeVar82 );

	if ( ( nodeVar91 != 0.0 ) ) {

		nodeVar92 = ( nodeVar83 + 1.0 );
		nodeVar93 = getFace( ( object.nodeUniform29 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar94 = max( ( 4.0 - nodeVar92 ), 0.0 );
		nodeVar92 = max( nodeVar92, 4.0 );
		nodeVar95 = exp2( nodeVar92 );
		nodeVar96 = ( ( getUV( ( object.nodeUniform29 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar93 ) * vec2<f32>( ( nodeVar95 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar93 > 2.0 ) ) {

			nodeVar96.y = ( nodeVar96.y + nodeVar95 );
			nodeVar93 = ( nodeVar93 - 3.0 );
			

		}

		nodeVar96.x = ( nodeVar96.x + ( nodeVar93 * nodeVar95 ) );
		nodeVar96.x = ( nodeVar96.x + ( nodeVar94 * ( 3.0 * 16.0 ) ) );
		nodeVar96.y = ( nodeVar96.y + ( 4.0 * ( exp2( object.nodeUniform28 ) - nodeVar95 ) ) );
		nodeVar96.x = ( nodeVar96.x * object.nodeUniform31 );
		nodeVar96.y = ( nodeVar96.y * object.nodeUniform32 );
		nodeVar97 = textureSampleGrad( nodeUniform33, nodeUniform33_sampler, nodeVar96, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar98 = nodeVar97.xyz;
		nodeVar90 = mix( nodeVar90, nodeVar98, nodeVar91 );
		

	}

	nodeVar99 = ( iblIrradiance + ( ( nodeVar90 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform34 ) ) );
	iblIrradiance = nodeVar99;
	nodeVar100 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar101 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar102 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar103 = ( SpecularF90 * dfg.y );
	nodeVar104 = ( nodeVar102 + vec3<f32>( nodeVar103 ) );
	nodeVar105 = ( nodeVar100 + nodeVar104 );
	nodeVar100 = nodeVar105;
	nodeVar106 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( nodeVar107 * vec3<f32>( 0.047619 ) );
	nodeVar109 = ( SpecularColor + nodeVar108 );
	nodeVar110 = ( nodeVar104 * nodeVar109 );
	nodeVar111 = ( dfg.x + dfg.y );
	nodeVar112 = ( 1.0 - nodeVar111 );
	nodeVar113 = nodeVar112;
	nodeVar114 = ( vec3<f32>( nodeVar113 ) * nodeVar109 );
	nodeVar115 = ( vec3<f32>( 1.0 ) - nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( nodeVar110 / nodeVar116 );
	nodeVar118 = ( nodeVar117 * vec3<f32>( nodeVar113 ) );
	nodeVar119 = ( nodeVar101 + nodeVar118 );
	nodeVar101 = nodeVar119;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar120 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar121 = ( irradiance * nodeVar120 );
	nodeVar122 = ( nodeVar100 + nodeVar101 );
	nodeVar123 = ( vec3<f32>( 1.0 ) - nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( nodeVar121 * nodeVar124 );
	nodeVar126 = nodeVar125;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar127 = ( indirectDiffuse + nodeVar126 );
	indirectDiffuse = nodeVar127;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar128 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar129 = ( SpecularF90 * dfg.y );
	nodeVar130 = ( nodeVar128 + vec3<f32>( nodeVar129 ) );
	nodeVar131 = ( singleScatteringDielectric + nodeVar130 );
	singleScatteringDielectric = nodeVar131;
	nodeVar132 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( nodeVar133 * vec3<f32>( 0.047619 ) );
	nodeVar135 = ( SpecularColor + nodeVar134 );
	nodeVar136 = ( nodeVar130 * nodeVar135 );
	nodeVar137 = ( dfg.x + dfg.y );
	nodeVar138 = ( 1.0 - nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( vec3<f32>( nodeVar139 ) * nodeVar135 );
	nodeVar141 = ( vec3<f32>( 1.0 ) - nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( nodeVar136 / nodeVar142 );
	nodeVar144 = ( nodeVar143 * vec3<f32>( nodeVar139 ) );
	nodeVar145 = ( multiScatteringDielectric + nodeVar144 );
	multiScatteringDielectric = nodeVar145;
	nodeVar146 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar147 = ( SpecularF90 * dfg.y );
	nodeVar148 = ( nodeVar146 + vec3<f32>( nodeVar147 ) );
	nodeVar149 = ( singleScatteringMetallic + nodeVar148 );
	singleScatteringMetallic = nodeVar149;
	nodeVar150 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar151 = nodeVar150;
	nodeVar152 = ( nodeVar151 * vec3<f32>( 0.047619 ) );
	nodeVar153 = ( DiffuseColor.xyz + nodeVar152 );
	nodeVar154 = ( nodeVar148 * nodeVar153 );
	nodeVar155 = ( dfg.x + dfg.y );
	nodeVar156 = ( 1.0 - nodeVar155 );
	nodeVar157 = nodeVar156;
	nodeVar158 = ( vec3<f32>( nodeVar157 ) * nodeVar153 );
	nodeVar159 = ( vec3<f32>( 1.0 ) - nodeVar158 );
	nodeVar160 = nodeVar159;
	nodeVar161 = ( nodeVar154 / nodeVar160 );
	nodeVar162 = ( nodeVar161 * vec3<f32>( nodeVar157 ) );
	nodeVar163 = ( multiScatteringMetallic + nodeVar162 );
	multiScatteringMetallic = nodeVar163;
	nodeVar164 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar165 = ( radiance * nodeVar164 );
	nodeVar166 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar167 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar168 = ( nodeVar166 * nodeVar167 );
	nodeVar169 = ( nodeVar165 + nodeVar168 );
	nodeVar170 = nodeVar169;
	nodeVar171 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar172 = ( vec3<f32>( 1.0 ) - nodeVar171 );
	nodeVar173 = nodeVar172;
	nodeVar174 = ( DiffuseContribution * nodeVar173 );
	nodeVar175 = ( nodeVar174 * nodeVar167 );
	nodeVar176 = nodeVar175;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar177 = ( indirectSpecular + nodeVar170 );
	indirectSpecular = nodeVar177;
	nodeVar178 = ( indirectDiffuse + nodeVar176 );
	indirectDiffuse = nodeVar178;
	ambientOcclusion = 1.0;
	nodeVar179 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar179;
	nodeVar180 = dot( normalView, positionViewDirection );
	nodeVar181 = ( clamp( nodeVar180, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar182 = ( Roughness * -16.0 );
	nodeVar183 = ( 1.0 - nodeVar182 );
	nodeVar184 = nodeVar183;
	nodeVar185 = ( - nodeVar184 );
	nodeVar186 = exp2( nodeVar185 );
	nodeVar187 = pow( nodeVar181, nodeVar186 );
	nodeVar188 = ( 1.0 - nodeVar187 );
	nodeVar189 = nodeVar188;
	nodeVar190 = ( ambientOcclusion - nodeVar189 );
	nodeVar191 = ( indirectSpecular * vec3<f32>( clamp( nodeVar190, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar191;
	nodeVar192 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar192;
	nodeVar193 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar193;
	nodeVar194 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar194;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar195 = ( object.nodeUniform35 + ( ( ( triNoise3D( ( v_positionWorld * vec3<f32>( 0.005 ) ), 0.2, object.nodeUniform36 ) + triNoise3D( ( v_positionWorld * vec3<f32>( 0.01 ) ), 0.2, ( object.nodeUniform36 * 1.2 ) ) ) - 0.7 ) * 22.0 ) );
	nodeVar196 = ( - v_positionView.z );
	nodeVar197 = vec4<f32>( mix( Output.xyz, vec3<f32>( 0.6307571363387763, 0.7304607400847158, 0.7991027380100881 ), ( 1.0 - ( ( 1.0 - ( clamp( ( ( nodeVar195 - v_positionWorld.y ) / ( nodeVar195 - object.nodeUniform37 ) ), 0.0, 1.0 ) * 0.98 ) ) * ( 1.0 - ( 1.0 - exp( ( - ( ( ( object.nodeUniform38 * object.nodeUniform38 ) * nodeVar196 ) * nodeVar196 ) ) ) ) ) ) ) ), Output.w );
	Output = nodeVar197;

	// result

	output.color = nodeVar197;

	return output;

}
