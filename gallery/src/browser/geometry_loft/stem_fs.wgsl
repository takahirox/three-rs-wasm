// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform29_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform29 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : mat4x4<f32>,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform30 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform12 : vec4<f32>,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform18 : vec4<f32>,
	nodeUniform11 : f32,
	nodeUniform14 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : vec2<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : vec2<f32>,
	nodeUniform23 : f32
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
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : f32;
var<private> shadowValue : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec3<f32>;
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
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec2<f32>;
var<private> nodeVar97 : vec4<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec2<f32>;
var<private> nodeVar105 : vec4<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar155 : f32;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
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
var<private> nodeVar184 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar187 : vec3<f32>;
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
var<private> nodeVar198 : f32;
var<private> nodeVar199 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec4<f32>;

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




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) nodeVarying7 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 0.7835377915215659, 0.6653872982754769, 0.46207699964472876 ) * vec3<f32>( ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( nodeVarying7.y * 24.0 ), ( nodeVarying7.x * 2.0 ), 0.0 ) ) * 1.0 ) + 0.0 ) * 0.1 ) + 0.94 ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( ( ( mx_perlin_noise_float_1( ( positionLocal * vec3<f32>( 12.0 ) ) ) * 1.0 ) + 0.0 ) * 0.15 ) + 0.55 ), 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = dot( normalView, positionViewDirection );
	nodeVar2 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( nodeVar1, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	nodeVar3 = ( dfg.x + dfg.y );
	nodeVar4 = ( 1.0 / nodeVar3 );
	nodeVar5 = nodeVar4;
	nodeVar6 = ( nodeVar5 - 1.0 );
	nodeVar7 = ( SpecularColorBlended * vec3<f32>( nodeVar6 ) );
	nodeVar8 = ( nodeVar7 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar8;
	nodeVar9 = vec4<f32>( render.nodeUniform9, 0.0 );
	nodeVar10 = ( render.cameraViewMatrix * nodeVar9 );
	nodeVar11 = normalize( nodeVar10.xyz );
	nodeVar12 = nodeVar11;
	nodeVar13 = dot( normalView, nodeVar12 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar14 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform11 ) ) ), 1.0 );
	nodeVar15 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar15 >= render.nodeUniform12.x ) && ( nodeVar15 < render.nodeUniform12.y ) ) ) {

		nodeVar17 = ( render.nodeUniform13 * nodeVar14 );
		nodeVar18 = ( nodeVar17.xyz / vec3<f32>( nodeVar17.w ) );
		nodeVar19 = vec3<f32>( nodeVar18.x, ( 1.0 - nodeVar18.y ), ( nodeVar18.z + render.nodeUniform14 ) );

		if ( ( ( ( ( ( nodeVar19.x >= 0.0 ) && ( nodeVar19.x <= 1.0 ) ) && ( nodeVar19.y >= 0.0 ) ) && ( nodeVar19.y <= 1.0 ) ) && ( nodeVar19.z <= 1.0 ) ) ) {

			nodeVar20 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar21 = ( render.nodeUniform16 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ).x );
			nodeVar22 = ( nodeVar19.xy + ( vogelDiskSample( 0, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar23 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar22, nodeVar19.z );
			nodeVar24 = ( nodeVar19.xy + ( vogelDiskSample( 1, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar25 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar24, nodeVar19.z );
			nodeVar26 = ( nodeVar19.xy + ( vogelDiskSample( 2, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar27 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar26, nodeVar19.z );
			nodeVar28 = ( nodeVar19.xy + ( vogelDiskSample( 3, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar29 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar28, nodeVar19.z );
			nodeVar30 = ( nodeVar19.xy + ( vogelDiskSample( 4, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar31 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar30, nodeVar19.z );
			nodeVar16 = ( ( ( ( ( nodeVar23 + nodeVar25 ) + nodeVar27 ) + nodeVar29 ) + nodeVar31 ) * 0.2 );

		} else {

			nodeVar16 = 1.0;

		}

		shadowValue = mix( nodeVar16, shadowValue, smoothstep( render.nodeUniform12.z, render.nodeUniform12.y, nodeVar15 ) );
		

	}


	if ( ( ( nodeVar15 >= render.nodeUniform18.x ) && ( nodeVar15 < render.nodeUniform18.y ) ) ) {

		nodeVar33 = ( render.nodeUniform19 * nodeVar14 );
		nodeVar34 = ( nodeVar33.xyz / vec3<f32>( nodeVar33.w ) );
		nodeVar35 = vec3<f32>( nodeVar34.x, ( 1.0 - nodeVar34.y ), ( nodeVar34.z + render.nodeUniform20 ) );

		if ( ( ( ( ( ( nodeVar35.x >= 0.0 ) && ( nodeVar35.x <= 1.0 ) ) && ( nodeVar35.y >= 0.0 ) ) && ( nodeVar35.y <= 1.0 ) ) && ( nodeVar35.z <= 1.0 ) ) ) {

			nodeVar36 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar37 = ( render.nodeUniform21 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform22 ).x );
			nodeVar38 = ( nodeVar35.xy + ( vogelDiskSample( 0, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar39 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar38, nodeVar35.z );
			nodeVar40 = ( nodeVar35.xy + ( vogelDiskSample( 1, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar41 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar40, nodeVar35.z );
			nodeVar42 = ( nodeVar35.xy + ( vogelDiskSample( 2, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar43 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar42, nodeVar35.z );
			nodeVar44 = ( nodeVar35.xy + ( vogelDiskSample( 3, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar45 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar44, nodeVar35.z );
			nodeVar46 = ( nodeVar35.xy + ( vogelDiskSample( 4, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar47 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar46, nodeVar35.z );
			nodeVar32 = ( ( ( ( ( nodeVar39 + nodeVar41 ) + nodeVar43 ) + nodeVar45 ) + nodeVar47 ) * 0.2 );

		} else {

			nodeVar32 = 1.0;

		}

		shadowValue = mix( nodeVar32, shadowValue, smoothstep( render.nodeUniform18.z, render.nodeUniform18.y, nodeVar15 ) );
		

	}

	nodeVar48 = mix( 1.0, shadowValue, render.nodeUniform23 );
	nodeVar49 = ( vec3<f32>( clamp( nodeVar13, 0.0, 1.0 ) ) * ( render.nodeUniform10 * vec3<f32>( nodeVar48 ) ) );
	nodeVar50 = nodeVar49;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar51 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar52 = ( nodeVar50 * nodeVar51 );
	nodeVar53 = ( nodeVar12 + positionViewDirection );
	nodeVar54 = normalize( nodeVar53 );
	nodeVar55 = dot( positionViewDirection, nodeVar54 );
	nodeVar56 = clamp( nodeVar55, 0.0, 1.0 );
	nodeVar57 = exp2( ( ( ( nodeVar56 * -5.55473 ) - 6.98316 ) * nodeVar56 ) );
	nodeVar58 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar57 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar57 ) ) );
	nodeVar59 = ( vec3<f32>( 1.0 ) - nodeVar58 );
	nodeVar60 = nodeVar59;
	nodeVar61 = ( nodeVar52 * nodeVar60 );
	nodeVar62 = ( directDiffuse + nodeVar61 );
	directDiffuse = nodeVar62;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar63 = normalize( ( nodeVar12 + positionViewDirection ) );
	nodeVar64 = clamp( dot( positionViewDirection, nodeVar63 ), 0.0, 1.0 );
	nodeVar65 = exp2( ( ( ( nodeVar64 * -5.55473 ) - 6.98316 ) * nodeVar64 ) );
	nodeVar66 = ( Roughness * Roughness );
	nodeVar67 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar65 ) ) ) + vec3<f32>( ( 1.0 * nodeVar65 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar66, clamp( dot( normalView, nodeVar12 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar66, clamp( dot( normalView, nodeVar63 ), 0.0, 1.0 ) ) ) );
	nodeVar68 = ( nodeVar50 * nodeVar67 );
	nodeVar69 = ( nodeVar68 * multiScatteringCompensation );
	nodeVar70 = ( directSpecular + nodeVar69 );
	directSpecular = nodeVar70;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar71 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform24 );
	nodeVar72 = floor( nodeVar71 );
	nodeVar73 = nodeVar72;
	nodeVar74 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar75 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar74.x, ( - nodeVar74.y ), nodeVar74.z ), 1.0 ) ).xyz );
	nodeVar76 = max( ( 4.0 - nodeVar73 ), 0.0 );
	nodeVar73 = max( nodeVar73, 4.0 );
	nodeVar77 = exp2( nodeVar73 );
	nodeVar78 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar74.x, ( - nodeVar74.y ), nodeVar74.z ), 1.0 ) ).xyz, nodeVar75 ) * vec2<f32>( ( nodeVar77 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar75 > 2.0 ) ) {

		nodeVar78.y = ( nodeVar78.y + nodeVar77 );
		nodeVar75 = ( nodeVar75 - 3.0 );
		

	}

	nodeVar78.x = ( nodeVar78.x + ( nodeVar75 * nodeVar77 ) );
	nodeVar78.x = ( nodeVar78.x + ( nodeVar76 * ( 3.0 * 16.0 ) ) );
	nodeVar78.y = ( nodeVar78.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar77 ) ) );
	nodeVar78.x = ( nodeVar78.x * object.nodeUniform27 );
	nodeVar78.y = ( nodeVar78.y * object.nodeUniform28 );
	nodeVar79 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar78, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar80 = nodeVar79.xyz;
	nodeVar81 = fract( nodeVar71 );

	if ( ( nodeVar81 != 0.0 ) ) {

		nodeVar82 = ( nodeVar72 + 1.0 );
		nodeVar83 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar74.x, ( - nodeVar74.y ), nodeVar74.z ), 1.0 ) ).xyz );
		nodeVar84 = max( ( 4.0 - nodeVar82 ), 0.0 );
		nodeVar82 = max( nodeVar82, 4.0 );
		nodeVar85 = exp2( nodeVar82 );
		nodeVar86 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar74.x, ( - nodeVar74.y ), nodeVar74.z ), 1.0 ) ).xyz, nodeVar83 ) * vec2<f32>( ( nodeVar85 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar83 > 2.0 ) ) {

			nodeVar86.y = ( nodeVar86.y + nodeVar85 );
			nodeVar83 = ( nodeVar83 - 3.0 );
			

		}

		nodeVar86.x = ( nodeVar86.x + ( nodeVar83 * nodeVar85 ) );
		nodeVar86.x = ( nodeVar86.x + ( nodeVar84 * ( 3.0 * 16.0 ) ) );
		nodeVar86.y = ( nodeVar86.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar85 ) ) );
		nodeVar86.x = ( nodeVar86.x * object.nodeUniform27 );
		nodeVar86.y = ( nodeVar86.y * object.nodeUniform28 );
		nodeVar87 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar86, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar88 = nodeVar87.xyz;
		nodeVar80 = mix( nodeVar80, nodeVar88, nodeVar81 );
		

	}

	nodeVar89 = ( radiance + ( nodeVar80 * vec3<f32>( object.nodeUniform30 ) ) );
	radiance = nodeVar89;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar90 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform24 );
	nodeVar91 = floor( nodeVar90 );
	nodeVar92 = nodeVar91;
	nodeVar93 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar94 = max( ( 4.0 - nodeVar92 ), 0.0 );
	nodeVar92 = max( nodeVar92, 4.0 );
	nodeVar95 = exp2( nodeVar92 );
	nodeVar96 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar93 ) * vec2<f32>( ( nodeVar95 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar93 > 2.0 ) ) {

		nodeVar96.y = ( nodeVar96.y + nodeVar95 );
		nodeVar93 = ( nodeVar93 - 3.0 );
		

	}

	nodeVar96.x = ( nodeVar96.x + ( nodeVar93 * nodeVar95 ) );
	nodeVar96.x = ( nodeVar96.x + ( nodeVar94 * ( 3.0 * 16.0 ) ) );
	nodeVar96.y = ( nodeVar96.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar95 ) ) );
	nodeVar96.x = ( nodeVar96.x * object.nodeUniform27 );
	nodeVar96.y = ( nodeVar96.y * object.nodeUniform28 );
	nodeVar97 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar96, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar98 = nodeVar97.xyz;
	nodeVar99 = fract( nodeVar90 );

	if ( ( nodeVar99 != 0.0 ) ) {

		nodeVar100 = ( nodeVar91 + 1.0 );
		nodeVar101 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar102 = max( ( 4.0 - nodeVar100 ), 0.0 );
		nodeVar100 = max( nodeVar100, 4.0 );
		nodeVar103 = exp2( nodeVar100 );
		nodeVar104 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar101 ) * vec2<f32>( ( nodeVar103 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar101 > 2.0 ) ) {

			nodeVar104.y = ( nodeVar104.y + nodeVar103 );
			nodeVar101 = ( nodeVar101 - 3.0 );
			

		}

		nodeVar104.x = ( nodeVar104.x + ( nodeVar101 * nodeVar103 ) );
		nodeVar104.x = ( nodeVar104.x + ( nodeVar102 * ( 3.0 * 16.0 ) ) );
		nodeVar104.y = ( nodeVar104.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar103 ) ) );
		nodeVar104.x = ( nodeVar104.x * object.nodeUniform27 );
		nodeVar104.y = ( nodeVar104.y * object.nodeUniform28 );
		nodeVar105 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar104, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar106 = nodeVar105.xyz;
		nodeVar98 = mix( nodeVar98, nodeVar106, nodeVar99 );
		

	}

	nodeVar107 = ( iblIrradiance + ( ( nodeVar98 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform30 ) ) );
	iblIrradiance = nodeVar107;
	nodeVar108 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar109 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar110 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar111 = ( SpecularF90 * dfg.y );
	nodeVar112 = ( nodeVar110 + vec3<f32>( nodeVar111 ) );
	nodeVar113 = ( nodeVar108 + nodeVar112 );
	nodeVar108 = nodeVar113;
	nodeVar114 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( nodeVar115 * vec3<f32>( 0.047619 ) );
	nodeVar117 = ( SpecularColor + nodeVar116 );
	nodeVar118 = ( nodeVar112 * nodeVar117 );
	nodeVar119 = ( dfg.x + dfg.y );
	nodeVar120 = ( 1.0 - nodeVar119 );
	nodeVar121 = nodeVar120;
	nodeVar122 = ( vec3<f32>( nodeVar121 ) * nodeVar117 );
	nodeVar123 = ( vec3<f32>( 1.0 ) - nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( nodeVar118 / nodeVar124 );
	nodeVar126 = ( nodeVar125 * vec3<f32>( nodeVar121 ) );
	nodeVar127 = ( nodeVar109 + nodeVar126 );
	nodeVar109 = nodeVar127;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar128 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar129 = ( irradiance * nodeVar128 );
	nodeVar130 = ( nodeVar108 + nodeVar109 );
	nodeVar131 = ( vec3<f32>( 1.0 ) - nodeVar130 );
	nodeVar132 = nodeVar131;
	nodeVar133 = ( nodeVar129 * nodeVar132 );
	nodeVar134 = nodeVar133;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar135 = ( indirectDiffuse + nodeVar134 );
	indirectDiffuse = nodeVar135;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar136 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar137 = ( SpecularF90 * dfg.y );
	nodeVar138 = ( nodeVar136 + vec3<f32>( nodeVar137 ) );
	nodeVar139 = ( singleScatteringDielectric + nodeVar138 );
	singleScatteringDielectric = nodeVar139;
	nodeVar140 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar141 = nodeVar140;
	nodeVar142 = ( nodeVar141 * vec3<f32>( 0.047619 ) );
	nodeVar143 = ( SpecularColor + nodeVar142 );
	nodeVar144 = ( nodeVar138 * nodeVar143 );
	nodeVar145 = ( dfg.x + dfg.y );
	nodeVar146 = ( 1.0 - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( vec3<f32>( nodeVar147 ) * nodeVar143 );
	nodeVar149 = ( vec3<f32>( 1.0 ) - nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( nodeVar144 / nodeVar150 );
	nodeVar152 = ( nodeVar151 * vec3<f32>( nodeVar147 ) );
	nodeVar153 = ( multiScatteringDielectric + nodeVar152 );
	multiScatteringDielectric = nodeVar153;
	nodeVar154 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar155 = ( SpecularF90 * dfg.y );
	nodeVar156 = ( nodeVar154 + vec3<f32>( nodeVar155 ) );
	nodeVar157 = ( singleScatteringMetallic + nodeVar156 );
	singleScatteringMetallic = nodeVar157;
	nodeVar158 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar159 = nodeVar158;
	nodeVar160 = ( nodeVar159 * vec3<f32>( 0.047619 ) );
	nodeVar161 = ( DiffuseColor.xyz + nodeVar160 );
	nodeVar162 = ( nodeVar156 * nodeVar161 );
	nodeVar163 = ( dfg.x + dfg.y );
	nodeVar164 = ( 1.0 - nodeVar163 );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( vec3<f32>( nodeVar165 ) * nodeVar161 );
	nodeVar167 = ( vec3<f32>( 1.0 ) - nodeVar166 );
	nodeVar168 = nodeVar167;
	nodeVar169 = ( nodeVar162 / nodeVar168 );
	nodeVar170 = ( nodeVar169 * vec3<f32>( nodeVar165 ) );
	nodeVar171 = ( multiScatteringMetallic + nodeVar170 );
	multiScatteringMetallic = nodeVar171;
	nodeVar172 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar173 = ( radiance * nodeVar172 );
	nodeVar174 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar175 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar176 = ( nodeVar174 * nodeVar175 );
	nodeVar177 = ( nodeVar173 + nodeVar176 );
	nodeVar178 = nodeVar177;
	nodeVar179 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar180 = ( vec3<f32>( 1.0 ) - nodeVar179 );
	nodeVar181 = nodeVar180;
	nodeVar182 = ( DiffuseContribution * nodeVar181 );
	nodeVar183 = ( nodeVar182 * nodeVar175 );
	nodeVar184 = nodeVar183;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar185 = ( indirectSpecular + nodeVar178 );
	indirectSpecular = nodeVar185;
	nodeVar186 = ( indirectDiffuse + nodeVar184 );
	indirectDiffuse = nodeVar186;
	ambientOcclusion = 1.0;
	nodeVar187 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar187;
	nodeVar188 = dot( normalView, positionViewDirection );
	nodeVar189 = ( clamp( nodeVar188, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar190 = ( Roughness * -16.0 );
	nodeVar191 = ( 1.0 - nodeVar190 );
	nodeVar192 = nodeVar191;
	nodeVar193 = ( - nodeVar192 );
	nodeVar194 = exp2( nodeVar193 );
	nodeVar195 = pow( nodeVar189, nodeVar194 );
	nodeVar196 = ( 1.0 - nodeVar195 );
	nodeVar197 = nodeVar196;
	nodeVar198 = ( ambientOcclusion - nodeVar197 );
	nodeVar199 = ( indirectSpecular * vec3<f32>( clamp( nodeVar198, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar199;
	nodeVar200 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar200;
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
