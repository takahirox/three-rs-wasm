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
var<private> nodeVar0 : vec3<f32>;
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
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec2<f32>;
var<private> nodeVar82 : vec4<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec2<f32>;
var<private> nodeVar90 : vec4<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec2<f32>;
var<private> nodeVar100 : vec4<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec2<f32>;
var<private> nodeVar108 : vec4<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
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
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : f32;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : f32;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec4<f32>;

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


fn mx_fractal_noise_float ( p : vec3<f32>, octaves : i32, lacunarity : f32, diminish : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : i32;

	nodeVar0 = diminish;
	nodeVar1 = lacunarity;
	nodeVar2 = p;
	nodeVar3 = 0.0;
	nodeVar4 = 1.0;
	nodeVar5 = octaves;

	for ( var i : i32 = 0; i < nodeVar5; i ++ ) {

		nodeVar3 = ( nodeVar3 + ( nodeVar4 * mx_perlin_noise_float_1( nodeVar2 ) ) );
		nodeVar4 = ( nodeVar4 * nodeVar0 );
		nodeVar2 = ( nodeVar2 * vec3<f32>( nodeVar1 ) );

	}


	return nodeVar3;

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
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( positionLocal * vec3<f32>( 0.9 ) );
	nodeVar1 = ( 1.0 - abs( ( mx_fractal_noise_float( ( nodeVar0 + vec3<f32>( ( ( mx_fractal_noise_float( ( nodeVar0 * vec3<f32>( 0.4 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 2.0 ) ) ), 4, 2.0, 0.5 ) * 1.0 ) ) );
	nodeVar2 = ( ( ( pow( nodeVar1, 4.0 ) * 0.3 ) + ( pow( nodeVar1, 12.0 ) * 0.7 ) ) + ( pow( ( 1.0 - abs( ( mx_fractal_noise_float( ( ( nodeVar0 * vec3<f32>( 3.0 ) ) + vec3<f32>( 11.0 ) ), 3, 2.0, 0.5 ) * 1.0 ) ) ), 14.0 ) * 0.2 ) );
	DiffuseColor = vec4<f32>( mix( mix( vec3<f32>( 0.9046611743890203, 0.9046611743890203, 0.9301108583738498 ), vec3<f32>( 0.85499260812105, 0.85499260812105, 0.8879231178794776 ), ( ( ( ( mx_perlin_noise_float_1( ( nodeVar0 * vec3<f32>( 0.5 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) ), vec3<f32>( 0.6307571363387763, 0.6307571363387763, 0.6653872982754769 ), nodeVar2 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar3 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( nodeVar2 * 0.14 ) + 0.07 ), 0.0525 ) + max( max( nodeVar3.x, nodeVar3.y ), nodeVar3.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = dot( normalView, positionViewDirection );
	nodeVar5 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( nodeVar4, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar5;
	nodeVar6 = ( dfg.x + dfg.y );
	nodeVar7 = ( 1.0 / nodeVar6 );
	nodeVar8 = nodeVar7;
	nodeVar9 = ( nodeVar8 - 1.0 );
	nodeVar10 = ( SpecularColorBlended * vec3<f32>( nodeVar9 ) );
	nodeVar11 = ( nodeVar10 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar11;
	nodeVar12 = vec4<f32>( render.nodeUniform9, 0.0 );
	nodeVar13 = ( render.cameraViewMatrix * nodeVar12 );
	nodeVar14 = normalize( nodeVar13.xyz );
	nodeVar15 = nodeVar14;
	nodeVar16 = dot( normalView, nodeVar15 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar17 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform11 ) ) ), 1.0 );
	nodeVar18 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar18 >= render.nodeUniform12.x ) && ( nodeVar18 < render.nodeUniform12.y ) ) ) {

		nodeVar20 = ( render.nodeUniform13 * nodeVar17 );
		nodeVar21 = ( nodeVar20.xyz / vec3<f32>( nodeVar20.w ) );
		nodeVar22 = vec3<f32>( nodeVar21.x, ( 1.0 - nodeVar21.y ), ( nodeVar21.z + render.nodeUniform14 ) );

		if ( ( ( ( ( ( nodeVar22.x >= 0.0 ) && ( nodeVar22.x <= 1.0 ) ) && ( nodeVar22.y >= 0.0 ) ) && ( nodeVar22.y <= 1.0 ) ) && ( nodeVar22.z <= 1.0 ) ) ) {

			nodeVar23 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar24 = ( render.nodeUniform16 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ).x );
			nodeVar25 = ( nodeVar22.xy + ( vogelDiskSample( 0, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar26 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar25, nodeVar22.z );
			nodeVar27 = ( nodeVar22.xy + ( vogelDiskSample( 1, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar28 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar27, nodeVar22.z );
			nodeVar29 = ( nodeVar22.xy + ( vogelDiskSample( 2, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar30 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar29, nodeVar22.z );
			nodeVar31 = ( nodeVar22.xy + ( vogelDiskSample( 3, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar32 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar31, nodeVar22.z );
			nodeVar33 = ( nodeVar22.xy + ( vogelDiskSample( 4, 5, nodeVar23 ) * vec2<f32>( nodeVar24 ) ) );
			nodeVar34 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar33, nodeVar22.z );
			nodeVar19 = ( ( ( ( ( nodeVar26 + nodeVar28 ) + nodeVar30 ) + nodeVar32 ) + nodeVar34 ) * 0.2 );

		} else {

			nodeVar19 = 1.0;

		}

		shadowValue = mix( nodeVar19, shadowValue, smoothstep( render.nodeUniform12.z, render.nodeUniform12.y, nodeVar18 ) );
		

	}


	if ( ( ( nodeVar18 >= render.nodeUniform18.x ) && ( nodeVar18 < render.nodeUniform18.y ) ) ) {

		nodeVar36 = ( render.nodeUniform19 * nodeVar17 );
		nodeVar37 = ( nodeVar36.xyz / vec3<f32>( nodeVar36.w ) );
		nodeVar38 = vec3<f32>( nodeVar37.x, ( 1.0 - nodeVar37.y ), ( nodeVar37.z + render.nodeUniform20 ) );

		if ( ( ( ( ( ( nodeVar38.x >= 0.0 ) && ( nodeVar38.x <= 1.0 ) ) && ( nodeVar38.y >= 0.0 ) ) && ( nodeVar38.y <= 1.0 ) ) && ( nodeVar38.z <= 1.0 ) ) ) {

			nodeVar39 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar40 = ( render.nodeUniform21 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform22 ).x );
			nodeVar41 = ( nodeVar38.xy + ( vogelDiskSample( 0, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar42 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar41, nodeVar38.z );
			nodeVar43 = ( nodeVar38.xy + ( vogelDiskSample( 1, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar44 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar43, nodeVar38.z );
			nodeVar45 = ( nodeVar38.xy + ( vogelDiskSample( 2, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar46 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar45, nodeVar38.z );
			nodeVar47 = ( nodeVar38.xy + ( vogelDiskSample( 3, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar48 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar47, nodeVar38.z );
			nodeVar49 = ( nodeVar38.xy + ( vogelDiskSample( 4, 5, nodeVar39 ) * vec2<f32>( nodeVar40 ) ) );
			nodeVar50 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar49, nodeVar38.z );
			nodeVar35 = ( ( ( ( ( nodeVar42 + nodeVar44 ) + nodeVar46 ) + nodeVar48 ) + nodeVar50 ) * 0.2 );

		} else {

			nodeVar35 = 1.0;

		}

		shadowValue = mix( nodeVar35, shadowValue, smoothstep( render.nodeUniform18.z, render.nodeUniform18.y, nodeVar18 ) );
		

	}

	nodeVar51 = mix( 1.0, shadowValue, render.nodeUniform23 );
	nodeVar52 = ( vec3<f32>( clamp( nodeVar16, 0.0, 1.0 ) ) * ( render.nodeUniform10 * vec3<f32>( nodeVar51 ) ) );
	nodeVar53 = nodeVar52;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar54 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar55 = ( nodeVar53 * nodeVar54 );
	nodeVar56 = ( nodeVar15 + positionViewDirection );
	nodeVar57 = normalize( nodeVar56 );
	nodeVar58 = dot( positionViewDirection, nodeVar57 );
	nodeVar59 = clamp( nodeVar58, 0.0, 1.0 );
	nodeVar60 = exp2( ( ( ( nodeVar59 * -5.55473 ) - 6.98316 ) * nodeVar59 ) );
	nodeVar61 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar60 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar60 ) ) );
	nodeVar62 = ( vec3<f32>( 1.0 ) - nodeVar61 );
	nodeVar63 = nodeVar62;
	nodeVar64 = ( nodeVar55 * nodeVar63 );
	nodeVar65 = ( directDiffuse + nodeVar64 );
	directDiffuse = nodeVar65;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar66 = normalize( ( nodeVar15 + positionViewDirection ) );
	nodeVar67 = clamp( dot( positionViewDirection, nodeVar66 ), 0.0, 1.0 );
	nodeVar68 = exp2( ( ( ( nodeVar67 * -5.55473 ) - 6.98316 ) * nodeVar67 ) );
	nodeVar69 = ( Roughness * Roughness );
	nodeVar70 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar68 ) ) ) + vec3<f32>( ( 1.0 * nodeVar68 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar69, clamp( dot( normalView, nodeVar15 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar69, clamp( dot( normalView, nodeVar66 ), 0.0, 1.0 ) ) ) );
	nodeVar71 = ( nodeVar53 * nodeVar70 );
	nodeVar72 = ( nodeVar71 * multiScatteringCompensation );
	nodeVar73 = ( directSpecular + nodeVar72 );
	directSpecular = nodeVar73;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar74 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform24 );
	nodeVar75 = floor( nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar78 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar77.x, ( - nodeVar77.y ), nodeVar77.z ), 1.0 ) ).xyz );
	nodeVar79 = max( ( 4.0 - nodeVar76 ), 0.0 );
	nodeVar76 = max( nodeVar76, 4.0 );
	nodeVar80 = exp2( nodeVar76 );
	nodeVar81 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar77.x, ( - nodeVar77.y ), nodeVar77.z ), 1.0 ) ).xyz, nodeVar78 ) * vec2<f32>( ( nodeVar80 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar78 > 2.0 ) ) {

		nodeVar81.y = ( nodeVar81.y + nodeVar80 );
		nodeVar78 = ( nodeVar78 - 3.0 );
		

	}

	nodeVar81.x = ( nodeVar81.x + ( nodeVar78 * nodeVar80 ) );
	nodeVar81.x = ( nodeVar81.x + ( nodeVar79 * ( 3.0 * 16.0 ) ) );
	nodeVar81.y = ( nodeVar81.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar80 ) ) );
	nodeVar81.x = ( nodeVar81.x * object.nodeUniform27 );
	nodeVar81.y = ( nodeVar81.y * object.nodeUniform28 );
	nodeVar82 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar81, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar83 = nodeVar82.xyz;
	nodeVar84 = fract( nodeVar74 );

	if ( ( nodeVar84 != 0.0 ) ) {

		nodeVar85 = ( nodeVar75 + 1.0 );
		nodeVar86 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar77.x, ( - nodeVar77.y ), nodeVar77.z ), 1.0 ) ).xyz );
		nodeVar87 = max( ( 4.0 - nodeVar85 ), 0.0 );
		nodeVar85 = max( nodeVar85, 4.0 );
		nodeVar88 = exp2( nodeVar85 );
		nodeVar89 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar77.x, ( - nodeVar77.y ), nodeVar77.z ), 1.0 ) ).xyz, nodeVar86 ) * vec2<f32>( ( nodeVar88 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar86 > 2.0 ) ) {

			nodeVar89.y = ( nodeVar89.y + nodeVar88 );
			nodeVar86 = ( nodeVar86 - 3.0 );
			

		}

		nodeVar89.x = ( nodeVar89.x + ( nodeVar86 * nodeVar88 ) );
		nodeVar89.x = ( nodeVar89.x + ( nodeVar87 * ( 3.0 * 16.0 ) ) );
		nodeVar89.y = ( nodeVar89.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar88 ) ) );
		nodeVar89.x = ( nodeVar89.x * object.nodeUniform27 );
		nodeVar89.y = ( nodeVar89.y * object.nodeUniform28 );
		nodeVar90 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar89, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar91 = nodeVar90.xyz;
		nodeVar83 = mix( nodeVar83, nodeVar91, nodeVar84 );
		

	}

	nodeVar92 = ( radiance + ( nodeVar83 * vec3<f32>( object.nodeUniform30 ) ) );
	radiance = nodeVar92;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar93 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform24 );
	nodeVar94 = floor( nodeVar93 );
	nodeVar95 = nodeVar94;
	nodeVar96 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar97 = max( ( 4.0 - nodeVar95 ), 0.0 );
	nodeVar95 = max( nodeVar95, 4.0 );
	nodeVar98 = exp2( nodeVar95 );
	nodeVar99 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar96 ) * vec2<f32>( ( nodeVar98 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar96 > 2.0 ) ) {

		nodeVar99.y = ( nodeVar99.y + nodeVar98 );
		nodeVar96 = ( nodeVar96 - 3.0 );
		

	}

	nodeVar99.x = ( nodeVar99.x + ( nodeVar96 * nodeVar98 ) );
	nodeVar99.x = ( nodeVar99.x + ( nodeVar97 * ( 3.0 * 16.0 ) ) );
	nodeVar99.y = ( nodeVar99.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar98 ) ) );
	nodeVar99.x = ( nodeVar99.x * object.nodeUniform27 );
	nodeVar99.y = ( nodeVar99.y * object.nodeUniform28 );
	nodeVar100 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar99, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar101 = nodeVar100.xyz;
	nodeVar102 = fract( nodeVar93 );

	if ( ( nodeVar102 != 0.0 ) ) {

		nodeVar103 = ( nodeVar94 + 1.0 );
		nodeVar104 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar105 = max( ( 4.0 - nodeVar103 ), 0.0 );
		nodeVar103 = max( nodeVar103, 4.0 );
		nodeVar106 = exp2( nodeVar103 );
		nodeVar107 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar104 ) * vec2<f32>( ( nodeVar106 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar104 > 2.0 ) ) {

			nodeVar107.y = ( nodeVar107.y + nodeVar106 );
			nodeVar104 = ( nodeVar104 - 3.0 );
			

		}

		nodeVar107.x = ( nodeVar107.x + ( nodeVar104 * nodeVar106 ) );
		nodeVar107.x = ( nodeVar107.x + ( nodeVar105 * ( 3.0 * 16.0 ) ) );
		nodeVar107.y = ( nodeVar107.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar106 ) ) );
		nodeVar107.x = ( nodeVar107.x * object.nodeUniform27 );
		nodeVar107.y = ( nodeVar107.y * object.nodeUniform28 );
		nodeVar108 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar107, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar109 = nodeVar108.xyz;
		nodeVar101 = mix( nodeVar101, nodeVar109, nodeVar102 );
		

	}

	nodeVar110 = ( iblIrradiance + ( ( nodeVar101 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform30 ) ) );
	iblIrradiance = nodeVar110;
	nodeVar111 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar112 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar113 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar114 = ( SpecularF90 * dfg.y );
	nodeVar115 = ( nodeVar113 + vec3<f32>( nodeVar114 ) );
	nodeVar116 = ( nodeVar111 + nodeVar115 );
	nodeVar111 = nodeVar116;
	nodeVar117 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar118 * vec3<f32>( 0.047619 ) );
	nodeVar120 = ( SpecularColor + nodeVar119 );
	nodeVar121 = ( nodeVar115 * nodeVar120 );
	nodeVar122 = ( dfg.x + dfg.y );
	nodeVar123 = ( 1.0 - nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( vec3<f32>( nodeVar124 ) * nodeVar120 );
	nodeVar126 = ( vec3<f32>( 1.0 ) - nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( nodeVar121 / nodeVar127 );
	nodeVar129 = ( nodeVar128 * vec3<f32>( nodeVar124 ) );
	nodeVar130 = ( nodeVar112 + nodeVar129 );
	nodeVar112 = nodeVar130;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar131 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar132 = ( irradiance * nodeVar131 );
	nodeVar133 = ( nodeVar111 + nodeVar112 );
	nodeVar134 = ( vec3<f32>( 1.0 ) - nodeVar133 );
	nodeVar135 = nodeVar134;
	nodeVar136 = ( nodeVar132 * nodeVar135 );
	nodeVar137 = nodeVar136;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar138 = ( indirectDiffuse + nodeVar137 );
	indirectDiffuse = nodeVar138;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar139 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar140 = ( SpecularF90 * dfg.y );
	nodeVar141 = ( nodeVar139 + vec3<f32>( nodeVar140 ) );
	nodeVar142 = ( singleScatteringDielectric + nodeVar141 );
	singleScatteringDielectric = nodeVar142;
	nodeVar143 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar144 = nodeVar143;
	nodeVar145 = ( nodeVar144 * vec3<f32>( 0.047619 ) );
	nodeVar146 = ( SpecularColor + nodeVar145 );
	nodeVar147 = ( nodeVar141 * nodeVar146 );
	nodeVar148 = ( dfg.x + dfg.y );
	nodeVar149 = ( 1.0 - nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( vec3<f32>( nodeVar150 ) * nodeVar146 );
	nodeVar152 = ( vec3<f32>( 1.0 ) - nodeVar151 );
	nodeVar153 = nodeVar152;
	nodeVar154 = ( nodeVar147 / nodeVar153 );
	nodeVar155 = ( nodeVar154 * vec3<f32>( nodeVar150 ) );
	nodeVar156 = ( multiScatteringDielectric + nodeVar155 );
	multiScatteringDielectric = nodeVar156;
	nodeVar157 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar158 = ( SpecularF90 * dfg.y );
	nodeVar159 = ( nodeVar157 + vec3<f32>( nodeVar158 ) );
	nodeVar160 = ( singleScatteringMetallic + nodeVar159 );
	singleScatteringMetallic = nodeVar160;
	nodeVar161 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( nodeVar162 * vec3<f32>( 0.047619 ) );
	nodeVar164 = ( DiffuseColor.xyz + nodeVar163 );
	nodeVar165 = ( nodeVar159 * nodeVar164 );
	nodeVar166 = ( dfg.x + dfg.y );
	nodeVar167 = ( 1.0 - nodeVar166 );
	nodeVar168 = nodeVar167;
	nodeVar169 = ( vec3<f32>( nodeVar168 ) * nodeVar164 );
	nodeVar170 = ( vec3<f32>( 1.0 ) - nodeVar169 );
	nodeVar171 = nodeVar170;
	nodeVar172 = ( nodeVar165 / nodeVar171 );
	nodeVar173 = ( nodeVar172 * vec3<f32>( nodeVar168 ) );
	nodeVar174 = ( multiScatteringMetallic + nodeVar173 );
	multiScatteringMetallic = nodeVar174;
	nodeVar175 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar176 = ( radiance * nodeVar175 );
	nodeVar177 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar178 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar179 = ( nodeVar177 * nodeVar178 );
	nodeVar180 = ( nodeVar176 + nodeVar179 );
	nodeVar181 = nodeVar180;
	nodeVar182 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar183 = ( vec3<f32>( 1.0 ) - nodeVar182 );
	nodeVar184 = nodeVar183;
	nodeVar185 = ( DiffuseContribution * nodeVar184 );
	nodeVar186 = ( nodeVar185 * nodeVar178 );
	nodeVar187 = nodeVar186;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar188 = ( indirectSpecular + nodeVar181 );
	indirectSpecular = nodeVar188;
	nodeVar189 = ( indirectDiffuse + nodeVar187 );
	indirectDiffuse = nodeVar189;
	ambientOcclusion = 1.0;
	nodeVar190 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar190;
	nodeVar191 = dot( normalView, positionViewDirection );
	nodeVar192 = ( clamp( nodeVar191, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar193 = ( Roughness * -16.0 );
	nodeVar194 = ( 1.0 - nodeVar193 );
	nodeVar195 = nodeVar194;
	nodeVar196 = ( - nodeVar195 );
	nodeVar197 = exp2( nodeVar196 );
	nodeVar198 = pow( nodeVar192, nodeVar197 );
	nodeVar199 = ( 1.0 - nodeVar198 );
	nodeVar200 = nodeVar199;
	nodeVar201 = ( ambientOcclusion - nodeVar200 );
	nodeVar202 = ( indirectSpecular * vec3<f32>( clamp( nodeVar201, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar202;
	nodeVar203 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar203;
	nodeVar204 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar204;
	nodeVar205 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar205;
	nodeVar206 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar206;

	// result

	output.color = nodeVar206;

	return output;

}
