// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform16_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform16 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform30_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform30 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform13 : vec4<f32>,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform19 : vec4<f32>,
	nodeUniform12 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : vec2<f32>,
	nodeUniform24 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar0 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec2<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : f32;
var<private> shadowValue : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : vec2<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec2<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec2<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
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
	nodeVar0 = ( ( mx_perlin_noise_float_1( vec3<f32>( ( nodeVarying7.x * 6.0 ), ( nodeVarying7.y * 160.0 ), 0.0 ) ) * 1.0 ) + 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( nodeVar0 * 0.08 ) + 0.1 ), 0.0525 ) + max( max( nodeVar1.x, nodeVar1.y ), nodeVar1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	nodeVar2 = normalize( dpdx( v_positionView ) );
	nodeVar3 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeVar3 ) );
	nodeVar4 = cross( normalize( - dpdy( v_positionView ) ), NORMAL_normalView );
	nodeVar5 = ( dot( nodeVar2, nodeVar4 ) * nodeVar3 );
	nodeVar6 = ( vec2<f32>( ( ( nodeVar0 * 0.004 ) - ( nodeVar0 * 0.004 ) ), ( ( nodeVar0 * 0.004 ) - ( nodeVar0 * 0.004 ) ) ) * vec2<f32>( 1.0 ) );
	normalView = normalize( ( ( vec3<f32>( abs( nodeVar5 ) ) * NORMAL_normalView ) - ( vec3<f32>( sign( nodeVar5 ) ) * ( ( vec3<f32>( nodeVar6.x ) * nodeVar4 ) + ( vec3<f32>( nodeVar6.y ) * cross( NORMAL_normalView, nodeVar2 ) ) ) ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar7 = dot( normalView, positionViewDirection );
	nodeVar8 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( nodeVar7, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar8;
	nodeVar9 = ( dfg.x + dfg.y );
	nodeVar10 = ( 1.0 / nodeVar9 );
	nodeVar11 = nodeVar10;
	nodeVar12 = ( nodeVar11 - 1.0 );
	nodeVar13 = ( SpecularColorBlended * vec3<f32>( nodeVar12 ) );
	nodeVar14 = ( nodeVar13 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar14;
	nodeVar15 = vec4<f32>( render.nodeUniform10, 0.0 );
	nodeVar16 = ( render.cameraViewMatrix * nodeVar15 );
	nodeVar17 = normalize( nodeVar16.xyz );
	nodeVar18 = nodeVar17;
	nodeVar19 = dot( normalView, nodeVar18 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar20 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform12 ) ) ), 1.0 );
	nodeVar21 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar21 >= render.nodeUniform13.x ) && ( nodeVar21 < render.nodeUniform13.y ) ) ) {

		nodeVar23 = ( render.nodeUniform14 * nodeVar20 );
		nodeVar24 = ( nodeVar23.xyz / vec3<f32>( nodeVar23.w ) );
		nodeVar25 = vec3<f32>( nodeVar24.x, ( 1.0 - nodeVar24.y ), ( nodeVar24.z + render.nodeUniform15 ) );

		if ( ( ( ( ( ( nodeVar25.x >= 0.0 ) && ( nodeVar25.x <= 1.0 ) ) && ( nodeVar25.y >= 0.0 ) ) && ( nodeVar25.y <= 1.0 ) ) && ( nodeVar25.z <= 1.0 ) ) ) {

			nodeVar26 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar27 = ( render.nodeUniform17 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ).x );
			nodeVar28 = ( nodeVar25.xy + ( vogelDiskSample( 0, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
			nodeVar29 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar28, nodeVar25.z );
			nodeVar30 = ( nodeVar25.xy + ( vogelDiskSample( 1, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
			nodeVar31 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar30, nodeVar25.z );
			nodeVar32 = ( nodeVar25.xy + ( vogelDiskSample( 2, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
			nodeVar33 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar32, nodeVar25.z );
			nodeVar34 = ( nodeVar25.xy + ( vogelDiskSample( 3, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
			nodeVar35 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar34, nodeVar25.z );
			nodeVar36 = ( nodeVar25.xy + ( vogelDiskSample( 4, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
			nodeVar37 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar36, nodeVar25.z );
			nodeVar22 = ( ( ( ( ( nodeVar29 + nodeVar31 ) + nodeVar33 ) + nodeVar35 ) + nodeVar37 ) * 0.2 );

		} else {

			nodeVar22 = 1.0;

		}

		shadowValue = mix( nodeVar22, shadowValue, smoothstep( render.nodeUniform13.z, render.nodeUniform13.y, nodeVar21 ) );
		

	}


	if ( ( ( nodeVar21 >= render.nodeUniform19.x ) && ( nodeVar21 < render.nodeUniform19.y ) ) ) {

		nodeVar39 = ( render.nodeUniform20 * nodeVar20 );
		nodeVar40 = ( nodeVar39.xyz / vec3<f32>( nodeVar39.w ) );
		nodeVar41 = vec3<f32>( nodeVar40.x, ( 1.0 - nodeVar40.y ), ( nodeVar40.z + render.nodeUniform21 ) );

		if ( ( ( ( ( ( nodeVar41.x >= 0.0 ) && ( nodeVar41.x <= 1.0 ) ) && ( nodeVar41.y >= 0.0 ) ) && ( nodeVar41.y <= 1.0 ) ) && ( nodeVar41.z <= 1.0 ) ) ) {

			nodeVar42 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar43 = ( render.nodeUniform22 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform23 ).x );
			nodeVar44 = ( nodeVar41.xy + ( vogelDiskSample( 0, 5, nodeVar42 ) * vec2<f32>( nodeVar43 ) ) );
			nodeVar45 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar44, nodeVar41.z );
			nodeVar46 = ( nodeVar41.xy + ( vogelDiskSample( 1, 5, nodeVar42 ) * vec2<f32>( nodeVar43 ) ) );
			nodeVar47 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar46, nodeVar41.z );
			nodeVar48 = ( nodeVar41.xy + ( vogelDiskSample( 2, 5, nodeVar42 ) * vec2<f32>( nodeVar43 ) ) );
			nodeVar49 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar48, nodeVar41.z );
			nodeVar50 = ( nodeVar41.xy + ( vogelDiskSample( 3, 5, nodeVar42 ) * vec2<f32>( nodeVar43 ) ) );
			nodeVar51 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar50, nodeVar41.z );
			nodeVar52 = ( nodeVar41.xy + ( vogelDiskSample( 4, 5, nodeVar42 ) * vec2<f32>( nodeVar43 ) ) );
			nodeVar53 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar52, nodeVar41.z );
			nodeVar38 = ( ( ( ( ( nodeVar45 + nodeVar47 ) + nodeVar49 ) + nodeVar51 ) + nodeVar53 ) * 0.2 );

		} else {

			nodeVar38 = 1.0;

		}

		shadowValue = mix( nodeVar38, shadowValue, smoothstep( render.nodeUniform19.z, render.nodeUniform19.y, nodeVar21 ) );
		

	}

	nodeVar54 = mix( 1.0, shadowValue, render.nodeUniform24 );
	nodeVar55 = ( vec3<f32>( clamp( nodeVar19, 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar54 ) ) );
	nodeVar56 = nodeVar55;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar57 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar58 = ( nodeVar56 * nodeVar57 );
	nodeVar59 = ( nodeVar18 + positionViewDirection );
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
	nodeVar69 = normalize( ( nodeVar18 + positionViewDirection ) );
	nodeVar70 = clamp( dot( positionViewDirection, nodeVar69 ), 0.0, 1.0 );
	nodeVar71 = exp2( ( ( ( nodeVar70 * -5.55473 ) - 6.98316 ) * nodeVar70 ) );
	nodeVar72 = ( Roughness * Roughness );
	nodeVar73 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar71 ) ) ) + vec3<f32>( ( 1.0 * nodeVar71 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar72, clamp( dot( normalView, nodeVar18 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar72, clamp( dot( normalView, nodeVar69 ), 0.0, 1.0 ) ) ) );
	nodeVar74 = ( nodeVar56 * nodeVar73 );
	nodeVar75 = ( nodeVar74 * multiScatteringCompensation );
	nodeVar76 = ( directSpecular + nodeVar75 );
	directSpecular = nodeVar76;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar77 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform25 );
	nodeVar78 = floor( nodeVar77 );
	nodeVar79 = nodeVar78;
	nodeVar80 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar81 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar80.x, ( - nodeVar80.y ), nodeVar80.z ), 1.0 ) ).xyz );
	nodeVar82 = max( ( 4.0 - nodeVar79 ), 0.0 );
	nodeVar79 = max( nodeVar79, 4.0 );
	nodeVar83 = exp2( nodeVar79 );
	nodeVar84 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar80.x, ( - nodeVar80.y ), nodeVar80.z ), 1.0 ) ).xyz, nodeVar81 ) * vec2<f32>( ( nodeVar83 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar81 > 2.0 ) ) {

		nodeVar84.y = ( nodeVar84.y + nodeVar83 );
		nodeVar81 = ( nodeVar81 - 3.0 );
		

	}

	nodeVar84.x = ( nodeVar84.x + ( nodeVar81 * nodeVar83 ) );
	nodeVar84.x = ( nodeVar84.x + ( nodeVar82 * ( 3.0 * 16.0 ) ) );
	nodeVar84.y = ( nodeVar84.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar83 ) ) );
	nodeVar84.x = ( nodeVar84.x * object.nodeUniform28 );
	nodeVar84.y = ( nodeVar84.y * object.nodeUniform29 );
	nodeVar85 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar84, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar86 = nodeVar85.xyz;
	nodeVar87 = fract( nodeVar77 );

	if ( ( nodeVar87 != 0.0 ) ) {

		nodeVar88 = ( nodeVar78 + 1.0 );
		nodeVar89 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar80.x, ( - nodeVar80.y ), nodeVar80.z ), 1.0 ) ).xyz );
		nodeVar90 = max( ( 4.0 - nodeVar88 ), 0.0 );
		nodeVar88 = max( nodeVar88, 4.0 );
		nodeVar91 = exp2( nodeVar88 );
		nodeVar92 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar80.x, ( - nodeVar80.y ), nodeVar80.z ), 1.0 ) ).xyz, nodeVar89 ) * vec2<f32>( ( nodeVar91 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar89 > 2.0 ) ) {

			nodeVar92.y = ( nodeVar92.y + nodeVar91 );
			nodeVar89 = ( nodeVar89 - 3.0 );
			

		}

		nodeVar92.x = ( nodeVar92.x + ( nodeVar89 * nodeVar91 ) );
		nodeVar92.x = ( nodeVar92.x + ( nodeVar90 * ( 3.0 * 16.0 ) ) );
		nodeVar92.y = ( nodeVar92.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar91 ) ) );
		nodeVar92.x = ( nodeVar92.x * object.nodeUniform28 );
		nodeVar92.y = ( nodeVar92.y * object.nodeUniform29 );
		nodeVar93 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar92, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar94 = nodeVar93.xyz;
		nodeVar86 = mix( nodeVar86, nodeVar94, nodeVar87 );
		

	}

	nodeVar95 = ( radiance + ( nodeVar86 * vec3<f32>( object.nodeUniform31 ) ) );
	radiance = nodeVar95;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar96 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform25 );
	nodeVar97 = floor( nodeVar96 );
	nodeVar98 = nodeVar97;
	nodeVar99 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar100 = max( ( 4.0 - nodeVar98 ), 0.0 );
	nodeVar98 = max( nodeVar98, 4.0 );
	nodeVar101 = exp2( nodeVar98 );
	nodeVar102 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar99 ) * vec2<f32>( ( nodeVar101 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar99 > 2.0 ) ) {

		nodeVar102.y = ( nodeVar102.y + nodeVar101 );
		nodeVar99 = ( nodeVar99 - 3.0 );
		

	}

	nodeVar102.x = ( nodeVar102.x + ( nodeVar99 * nodeVar101 ) );
	nodeVar102.x = ( nodeVar102.x + ( nodeVar100 * ( 3.0 * 16.0 ) ) );
	nodeVar102.y = ( nodeVar102.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar101 ) ) );
	nodeVar102.x = ( nodeVar102.x * object.nodeUniform28 );
	nodeVar102.y = ( nodeVar102.y * object.nodeUniform29 );
	nodeVar103 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar102, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar104 = nodeVar103.xyz;
	nodeVar105 = fract( nodeVar96 );

	if ( ( nodeVar105 != 0.0 ) ) {

		nodeVar106 = ( nodeVar97 + 1.0 );
		nodeVar107 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar108 = max( ( 4.0 - nodeVar106 ), 0.0 );
		nodeVar106 = max( nodeVar106, 4.0 );
		nodeVar109 = exp2( nodeVar106 );
		nodeVar110 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar107 ) * vec2<f32>( ( nodeVar109 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar107 > 2.0 ) ) {

			nodeVar110.y = ( nodeVar110.y + nodeVar109 );
			nodeVar107 = ( nodeVar107 - 3.0 );
			

		}

		nodeVar110.x = ( nodeVar110.x + ( nodeVar107 * nodeVar109 ) );
		nodeVar110.x = ( nodeVar110.x + ( nodeVar108 * ( 3.0 * 16.0 ) ) );
		nodeVar110.y = ( nodeVar110.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar109 ) ) );
		nodeVar110.x = ( nodeVar110.x * object.nodeUniform28 );
		nodeVar110.y = ( nodeVar110.y * object.nodeUniform29 );
		nodeVar111 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar110, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar112 = nodeVar111.xyz;
		nodeVar104 = mix( nodeVar104, nodeVar112, nodeVar105 );
		

	}

	nodeVar113 = ( iblIrradiance + ( ( nodeVar104 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform31 ) ) );
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
