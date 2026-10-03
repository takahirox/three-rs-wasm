// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform17_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform17 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform37_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform37 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform42_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform42 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32,
	nodeUniform46 : f32,
	nodeUniform47 : f32,
	nodeUniform50 : f32,
	nodeUniform51 : f32,
	nodeUniform36 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform35 : vec3<f32>,
	nodeUniform48 : vec3<f32>,
	nodeUniform49 : vec3<f32>,
	nodeUniform38 : mat4x4<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform43 : f32,
	nodeUniform44 : vec2<f32>,
	nodeUniform45 : f32
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
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
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
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar64 : vec4<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec4<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec2<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec2<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec2<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec2<f32>;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec2<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec4<f32>;
var<private> nodeVar82 : vec4<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec4<f32>;
var<private> nodeVar85 : vec4<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec4<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec4<f32>;
var<private> nodeVar96 : vec4<f32>;
var<private> nodeVar97 : vec4<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec4<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec4<f32>;
var<private> nodeVar109 : vec4<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec4<f32>;
var<private> nodeVar116 : vec4<f32>;
var<private> nodeVar117 : vec4<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : f32;
var<private> nodeVar203 : f32;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : f32;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : f32;
var<private> nodeVar209 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec4<f32>;

// codes
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


fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

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
	nodeVar1 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar2 = ( positionLocal * vec3<f32>( 0.5 ) );
	nodeVar3 = vec3<f32>( 0.0, ( - object.nodeUniform7 ), 0.0 );
	nodeVar4 = ( ( ( ( mx_perlin_noise_float_1( ( nodeVar2 + nodeVar3 ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 );
	nodeVar5 = ( clamp( pow( ( ( ( nodeVar4 * 0.5 ) + ( ( ( ( ( mx_perlin_noise_float_1( ( ( ( nodeVar2 * vec3<f32>( 2.0 ) ) - ( nodeVar3 * vec3<f32>( 1.5 ) ) ) + vec3<f32>( ( nodeVar4 * 0.4 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.35 ) ) + ( ( ( ( ( mx_perlin_noise_float_1( ( ( nodeVar2 * vec3<f32>( 4.0 ) ) + ( nodeVar3 * vec3<f32>( 2.5 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.15 ) ), 2.5 ), 0.0, 1.0 ) + 0.1 );
	nodeVar1 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), object.nodeUniform6, smoothstep( 0.05, 0.35, nodeVar5 ) );
	nodeVar1 = mix( nodeVar1, object.nodeUniform8, smoothstep( 0.35, 0.65, nodeVar5 ) );
	nodeVar1 = mix( nodeVar1, object.nodeUniform9, smoothstep( 0.65, 1.0, nodeVar5 ) );
	nodeVar6 = cos( object.nodeUniform11 );
	EmissiveColor = ( ( ( ( ( ( max( ( ( max( mix( vec3<f32>( dot( nodeVar1, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar1, object.nodeUniform10 ), vec3<f32>( 0.0 ) ) * vec3<f32>( nodeVar6 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar1, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar1, object.nodeUniform10 ), vec3<f32>( 0.0 ) ) ) * vec3<f32>( sin( object.nodeUniform11 ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar1, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar1, object.nodeUniform10 ), vec3<f32>( 0.0 ) ) ) * ( 1.0 - nodeVar6 ) ) ) ) ) ), vec3<f32>( 0.0 ) ) * vec3<f32>( pow( max( ( object.nodeUniform12 / 8.34 ), 0.0 ), 4.0 ) ) ) * vec3<f32>( max( ( object.nodeUniform13 / 11.02 ), 0.0 ) ) ) * vec3<f32>( object.nodeUniform14 ) ) * vec3<f32>( object.nodeUniform15 ) ) * vec3<f32>( smoothstep( 0.0, 3.0, object.nodeUniform7 ) ) ) * vec3<f32>( object.nodeUniform16 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar7 = dot( normalView, positionViewDirection );
	nodeVar8 = textureSample( nodeUniform17, nodeUniform17_sampler, vec2<f32>( Roughness, clamp( nodeVar7, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar8;
	nodeVar9 = ( dfg.x + dfg.y );
	nodeVar10 = ( 1.0 / nodeVar9 );
	nodeVar11 = nodeVar10;
	nodeVar12 = ( nodeVar11 - 1.0 );
	nodeVar13 = ( SpecularColorBlended * vec3<f32>( nodeVar12 ) );
	nodeVar14 = ( nodeVar13 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar14;
	nodeVar15 = ( render.nodeUniform19 - v_positionView );
	nodeVar16 = normalize( nodeVar15 );
	nodeVar17 = dot( normalView, nodeVar16 );

	if ( ( 0.0 == 1.0 ) ) {

		nodeVar19 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar20 = clamp( ( ( ( ( object.nodeUniform12 / 8.34 ) * 0.5 ) + 0.2 ) + object.nodeUniform20 ), 0.0, 1.0 );
		nodeVar19 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), object.nodeUniform6, smoothstep( 0.05, 0.35, nodeVar20 ) );
		nodeVar19 = mix( nodeVar19, object.nodeUniform8, smoothstep( 0.35, 0.65, nodeVar20 ) );
		nodeVar19 = mix( nodeVar19, object.nodeUniform9, smoothstep( 0.65, 1.0, nodeVar20 ) );
		nodeVar21 = cos( object.nodeUniform11 );
		nodeVar18 = max( ( ( max( mix( vec3<f32>( dot( nodeVar19, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar19, object.nodeUniform10 ), vec3<f32>( 0.0 ) ) * vec3<f32>( nodeVar21 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar19, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar19, object.nodeUniform10 ), vec3<f32>( 0.0 ) ) ) * vec3<f32>( sin( object.nodeUniform11 ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar19, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar19, object.nodeUniform10 ), vec3<f32>( 0.0 ) ) ) * ( 1.0 - nodeVar21 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

	} else {

		nodeVar22 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar23 = vec3<f32>( 0.0, object.nodeUniform23, 0.0 );
		nodeVar24 = length( ( v_positionWorld - ( ( object.nodeUniform21 + object.nodeUniform22 ) + ( nodeVar23 * vec3<f32>( clamp( ( dot( ( v_positionWorld - object.nodeUniform21 ), nodeVar23 ) / dot( nodeVar23, nodeVar23 ) ), 0.0, 1.0 ) ) ) ) ) );
		nodeVar25 = ( v_positionWorld.xz - object.nodeUniform21.xz );
		nodeVar26 = atan2( nodeVar25.y, nodeVar25.x );
		nodeVar27 = clamp( ( clamp( ( 1.0 - ( nodeVar24 / object.nodeUniform24 ) ), 0.0, 1.0 ) * mix( ( ( ( ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( v_positionWorld.x * ( 0.6 * object.nodeUniform25 ) ), ( object.nodeUniform7 * 1.2 ), ( v_positionWorld.z * ( 0.6 * object.nodeUniform25 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.65 ) + ( ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( v_positionWorld.x * ( 1.5 * object.nodeUniform25 ) ), ( object.nodeUniform7 * 2.5 ), ( v_positionWorld.z * ( 1.5 * object.nodeUniform25 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.35 ) ) * ( ( mix( 1.0, ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( cos( nodeVar26 ) * ( 1.5 * object.nodeUniform25 ) ), ( sin( nodeVar26 ) * ( 1.5 * object.nodeUniform25 ) ), ( object.nodeUniform7 * 0.6 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ), smoothstep( 0.0, object.nodeUniform26, length( nodeVar25 ) ) ) * 0.5 ) + 0.5 ) ), 1.0, clamp( ( nodeVar24 / object.nodeUniform27 ), 0.0, 1.0 ) ) ), 0.0, 1.0 );
		nodeVar22 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), object.nodeUniform6, smoothstep( 0.05, 0.35, nodeVar27 ) );
		nodeVar22 = mix( nodeVar22, object.nodeUniform8, smoothstep( 0.35, 0.65, nodeVar27 ) );
		nodeVar22 = mix( nodeVar22, object.nodeUniform9, smoothstep( 0.65, 1.0, nodeVar27 ) );
		nodeVar28 = cos( object.nodeUniform11 );
		nodeVar18 = max( ( ( max( mix( vec3<f32>( dot( nodeVar22, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar22, object.nodeUniform10 ), vec3<f32>( 0.0 ) ) * vec3<f32>( nodeVar28 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar22, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar22, object.nodeUniform10 ), vec3<f32>( 0.0 ) ) ) * vec3<f32>( sin( object.nodeUniform11 ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar22, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar22, object.nodeUniform10 ), vec3<f32>( 0.0 ) ) ) * ( 1.0 - nodeVar28 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

	}


	if ( ( 0.0 == 1.0 ) ) {

		nodeVar29 = object.nodeUniform28;

	} else {

		nodeVar29 = object.nodeUniform29;

	}


	if ( ( 0.0 == 1.0 ) ) {

		nodeVar31 = vec3<f32>( 0.0, object.nodeUniform23, 0.0 );
		nodeVar32 = length( ( v_positionWorld - ( ( object.nodeUniform21 + object.nodeUniform22 ) + ( nodeVar31 * vec3<f32>( clamp( ( dot( ( v_positionWorld - object.nodeUniform21 ), nodeVar31 ) / dot( nodeVar31, nodeVar31 ) ), 0.0, 1.0 ) ) ) ) ) );
		nodeVar30 = ( ( 1.0 / ( pow( nodeVar32, 2.0 ) + pow( 1.2, 2.0 ) ) ) / ( 1.0 / max( pow( length( ( v_positionWorld - object.nodeUniform21 ) ), 2.0 ), 0.01 ) ) );

	} else {

		nodeVar30 = 1.0;

	}


	if ( ( 0.0 == 1.0 ) ) {

		nodeVar33 = mix( object.nodeUniform30, object.nodeUniform31, smoothstep( 0.0, 1.0, clamp( ( nodeVar32 / object.nodeUniform32 ), 0.0, 1.0 ) ) );

	} else {

		nodeVar33 = 1.0;

	}


	if ( ( render.nodeUniform33 > 0.0 ) ) {

		nodeVar35 = length( nodeVar15 );
		nodeVar36 = ( nodeVar35 / render.nodeUniform33 );
		nodeVar37 = clamp( ( 1.0 - ( ( ( nodeVar36 * nodeVar36 ) * nodeVar36 ) * nodeVar36 ) ), 0.0, 1.0 );
		nodeVar34 = ( ( 1.0 / max( pow( nodeVar35, render.nodeUniform34 ), 0.01 ) ) * ( nodeVar37 * nodeVar37 ) );

	} else {

		nodeVar34 = ( 1.0 / max( pow( length( nodeVar15 ), render.nodeUniform34 ), 0.01 ) );

	}

	nodeVar38 = ( ( ( ( ( ( ( ( ( nodeVar18 * vec3<f32>( pow( max( ( object.nodeUniform12 / 8.34 ), 0.0 ), 4.0 ) ) ) * vec3<f32>( max( ( object.nodeUniform13 / 11.02 ), 0.0 ) ) ) * vec3<f32>( object.nodeUniform14 ) ) * vec3<f32>( nodeVar29 ) ) * vec3<f32>( object.nodeUniform15 ) ) * vec3<f32>( smoothstep( 0.0, 3.0, object.nodeUniform7 ) ) ) * vec3<f32>( nodeVar30 ) ) * vec3<f32>( nodeVar33 ) ) * vec3<f32>( nodeVar34 ) );
	nodeVar39 = ( vec3<f32>( clamp( nodeVar17, 0.0, 1.0 ) ) * nodeVar38 );
	nodeVar40 = nodeVar39;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar41 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar42 = ( nodeVar40 * nodeVar41 );
	nodeVar43 = ( nodeVar16 + positionViewDirection );
	nodeVar44 = normalize( nodeVar43 );
	nodeVar45 = dot( positionViewDirection, nodeVar44 );
	nodeVar46 = clamp( nodeVar45, 0.0, 1.0 );
	nodeVar47 = exp2( ( ( ( nodeVar46 * -5.55473 ) - 6.98316 ) * nodeVar46 ) );
	nodeVar48 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar47 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar47 ) ) );
	nodeVar49 = ( vec3<f32>( 1.0 ) - nodeVar48 );
	nodeVar50 = nodeVar49;
	nodeVar51 = ( nodeVar42 * nodeVar50 );
	nodeVar52 = ( directDiffuse + nodeVar51 );
	directDiffuse = nodeVar52;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar53 = normalize( ( nodeVar16 + positionViewDirection ) );
	nodeVar54 = clamp( dot( positionViewDirection, nodeVar53 ), 0.0, 1.0 );
	nodeVar55 = exp2( ( ( ( nodeVar54 * -5.55473 ) - 6.98316 ) * nodeVar54 ) );
	nodeVar56 = ( Roughness * Roughness );
	nodeVar57 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar55 ) ) ) + vec3<f32>( ( 1.0 * nodeVar55 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar56, clamp( dot( normalView, nodeVar16 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar56, clamp( dot( normalView, nodeVar53 ), 0.0, 1.0 ) ) ) );
	nodeVar58 = ( nodeVar40 * nodeVar57 );
	nodeVar59 = ( nodeVar58 * multiScatteringCompensation );
	nodeVar60 = ( directSpecular + nodeVar59 );
	directSpecular = nodeVar60;
	nodeVar61 = ( render.nodeUniform35 - v_positionView );
	nodeVar62 = normalize( nodeVar61 );
	nodeVar63 = dot( normalView, nodeVar62 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar64 = ( render.nodeUniform38 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform40 ) ) ), 1.0 ) );
	nodeVar65 = ( nodeVar64.xyz / vec3<f32>( nodeVar64.w ) );
	nodeVar66 = vec3<f32>( nodeVar65.x, ( 1.0 - nodeVar65.y ), ( nodeVar65.z + render.nodeUniform41 ) );
	nodeVar67 = textureSample( nodeUniform37, nodeUniform37_sampler, nodeVar66.xy );

	if ( ( ( ( ( ( nodeVar66.x >= 0.0 ) && ( nodeVar66.x <= 1.0 ) ) && ( nodeVar66.y >= 0.0 ) ) && ( nodeVar66.y <= 1.0 ) ) && ( nodeVar66.z <= 1.0 ) ) ) {

		nodeVar69 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar70 = ( render.nodeUniform43 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform44 ).x );
		nodeVar71 = ( nodeVar66.xy + ( vogelDiskSample( 0, 5, nodeVar69 ) * vec2<f32>( nodeVar70 ) ) );
		nodeVar72 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeVar71, nodeVar66.z );
		nodeVar73 = ( nodeVar66.xy + ( vogelDiskSample( 1, 5, nodeVar69 ) * vec2<f32>( nodeVar70 ) ) );
		nodeVar74 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeVar73, nodeVar66.z );
		nodeVar75 = ( nodeVar66.xy + ( vogelDiskSample( 2, 5, nodeVar69 ) * vec2<f32>( nodeVar70 ) ) );
		nodeVar76 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeVar75, nodeVar66.z );
		nodeVar77 = ( nodeVar66.xy + ( vogelDiskSample( 3, 5, nodeVar69 ) * vec2<f32>( nodeVar70 ) ) );
		nodeVar78 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeVar77, nodeVar66.z );
		nodeVar79 = ( nodeVar66.xy + ( vogelDiskSample( 4, 5, nodeVar69 ) * vec2<f32>( nodeVar70 ) ) );
		nodeVar80 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeVar79, nodeVar66.z );
		nodeVar68 = ( ( ( ( ( nodeVar72 + nodeVar74 ) + nodeVar76 ) + nodeVar78 ) + nodeVar80 ) * 0.2 );

	} else {

		nodeVar68 = 1.0;

	}

	nodeVar81 = mix( vec4<f32>( 1.0 ), mix( nodeVar67, vec4<f32>( 1.0 ), vec4<f32>( nodeVar68 ) ), ( render.nodeUniform45 * nodeVar67.w ) );
	nodeVar82 = ( vec4<f32>( render.nodeUniform36, 1.0 ) * nodeVar81 );
	nodeVar83 = ( render.nodeUniform48 - render.nodeUniform49 );
	nodeVar84 = vec4<f32>( nodeVar83, 0.0 );
	nodeVar85 = ( render.cameraViewMatrix * nodeVar84 );
	nodeVar86 = normalize( nodeVar85.xyz );
	nodeVar87 = nodeVar86;
	nodeVar88 = dot( nodeVar62, nodeVar87 );
	nodeVar89 = smoothstep( render.nodeUniform46, render.nodeUniform47, nodeVar88 );
	nodeVar90 = ( nodeVar82 * vec4<f32>( nodeVar89 ) );

	if ( ( render.nodeUniform50 > 0.0 ) ) {

		nodeVar92 = length( nodeVar61 );
		nodeVar93 = ( nodeVar92 / render.nodeUniform50 );
		nodeVar94 = clamp( ( 1.0 - ( ( ( nodeVar93 * nodeVar93 ) * nodeVar93 ) * nodeVar93 ) ), 0.0, 1.0 );
		nodeVar91 = ( ( 1.0 / max( pow( nodeVar92, render.nodeUniform51 ), 0.01 ) ) * ( nodeVar94 * nodeVar94 ) );

	} else {

		nodeVar91 = ( 1.0 / max( pow( length( nodeVar61 ), render.nodeUniform51 ), 0.01 ) );

	}

	nodeVar95 = ( nodeVar90 * vec4<f32>( nodeVar91 ) );
	nodeVar96 = ( vec4<f32>( clamp( nodeVar63, 0.0, 1.0 ) ) * nodeVar95 );
	nodeVar97 = nodeVar96;
	nodeVar98 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar99 = ( nodeVar97 * vec4<f32>( nodeVar98, 1.0 ) );
	nodeVar100 = ( nodeVar62 + positionViewDirection );
	nodeVar101 = normalize( nodeVar100 );
	nodeVar102 = dot( positionViewDirection, nodeVar101 );
	nodeVar103 = clamp( nodeVar102, 0.0, 1.0 );
	nodeVar104 = exp2( ( ( ( nodeVar103 * -5.55473 ) - 6.98316 ) * nodeVar103 ) );
	nodeVar105 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar104 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar104 ) ) );
	nodeVar106 = ( vec3<f32>( 1.0 ) - nodeVar105 );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( nodeVar99 * vec4<f32>( nodeVar107, 1.0 ) );
	nodeVar109 = ( vec4<f32>( directDiffuse, 1.0 ) + nodeVar108 );
	directDiffuse = nodeVar109.xyz;
	nodeVar110 = normalize( ( nodeVar62 + positionViewDirection ) );
	nodeVar111 = clamp( dot( positionViewDirection, nodeVar110 ), 0.0, 1.0 );
	nodeVar112 = exp2( ( ( ( nodeVar111 * -5.55473 ) - 6.98316 ) * nodeVar111 ) );
	nodeVar113 = ( Roughness * Roughness );
	nodeVar114 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar112 ) ) ) + vec3<f32>( ( 1.0 * nodeVar112 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar113, clamp( dot( normalView, nodeVar62 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar113, clamp( dot( normalView, nodeVar110 ), 0.0, 1.0 ) ) ) );
	nodeVar115 = ( nodeVar97 * vec4<f32>( nodeVar114, 1.0 ) );
	nodeVar116 = ( nodeVar115 * vec4<f32>( multiScatteringCompensation, 1.0 ) );
	nodeVar117 = ( vec4<f32>( directSpecular, 1.0 ) + nodeVar116 );
	directSpecular = nodeVar117.xyz;
	nodeVar118 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar119 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar120 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar121 = ( SpecularF90 * dfg.y );
	nodeVar122 = ( nodeVar120 + vec3<f32>( nodeVar121 ) );
	nodeVar123 = ( nodeVar118 + nodeVar122 );
	nodeVar118 = nodeVar123;
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
	nodeVar137 = ( nodeVar119 + nodeVar136 );
	nodeVar119 = nodeVar137;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar138 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar139 = ( irradiance * nodeVar138 );
	nodeVar140 = ( nodeVar118 + nodeVar119 );
	nodeVar141 = ( vec3<f32>( 1.0 ) - nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( nodeVar139 * nodeVar142 );
	nodeVar144 = nodeVar143;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar145 = ( indirectDiffuse + nodeVar144 );
	indirectDiffuse = nodeVar145;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar146 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar147 = ( SpecularF90 * dfg.y );
	nodeVar148 = ( nodeVar146 + vec3<f32>( nodeVar147 ) );
	nodeVar149 = ( singleScatteringDielectric + nodeVar148 );
	singleScatteringDielectric = nodeVar149;
	nodeVar150 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar151 = nodeVar150;
	nodeVar152 = ( nodeVar151 * vec3<f32>( 0.047619 ) );
	nodeVar153 = ( SpecularColor + nodeVar152 );
	nodeVar154 = ( nodeVar148 * nodeVar153 );
	nodeVar155 = ( dfg.x + dfg.y );
	nodeVar156 = ( 1.0 - nodeVar155 );
	nodeVar157 = nodeVar156;
	nodeVar158 = ( vec3<f32>( nodeVar157 ) * nodeVar153 );
	nodeVar159 = ( vec3<f32>( 1.0 ) - nodeVar158 );
	nodeVar160 = nodeVar159;
	nodeVar161 = ( nodeVar154 / nodeVar160 );
	nodeVar162 = ( nodeVar161 * vec3<f32>( nodeVar157 ) );
	nodeVar163 = ( multiScatteringDielectric + nodeVar162 );
	multiScatteringDielectric = nodeVar163;
	nodeVar164 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar165 = ( SpecularF90 * dfg.y );
	nodeVar166 = ( nodeVar164 + vec3<f32>( nodeVar165 ) );
	nodeVar167 = ( singleScatteringMetallic + nodeVar166 );
	singleScatteringMetallic = nodeVar167;
	nodeVar168 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar169 = nodeVar168;
	nodeVar170 = ( nodeVar169 * vec3<f32>( 0.047619 ) );
	nodeVar171 = ( DiffuseColor.xyz + nodeVar170 );
	nodeVar172 = ( nodeVar166 * nodeVar171 );
	nodeVar173 = ( dfg.x + dfg.y );
	nodeVar174 = ( 1.0 - nodeVar173 );
	nodeVar175 = nodeVar174;
	nodeVar176 = ( vec3<f32>( nodeVar175 ) * nodeVar171 );
	nodeVar177 = ( vec3<f32>( 1.0 ) - nodeVar176 );
	nodeVar178 = nodeVar177;
	nodeVar179 = ( nodeVar172 / nodeVar178 );
	nodeVar180 = ( nodeVar179 * vec3<f32>( nodeVar175 ) );
	nodeVar181 = ( multiScatteringMetallic + nodeVar180 );
	multiScatteringMetallic = nodeVar181;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar182 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar183 = ( radiance * nodeVar182 );
	nodeVar184 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar185 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar186 = ( nodeVar184 * nodeVar185 );
	nodeVar187 = ( nodeVar183 + nodeVar186 );
	nodeVar188 = nodeVar187;
	nodeVar189 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar190 = ( vec3<f32>( 1.0 ) - nodeVar189 );
	nodeVar191 = nodeVar190;
	nodeVar192 = ( DiffuseContribution * nodeVar191 );
	nodeVar193 = ( nodeVar192 * nodeVar185 );
	nodeVar194 = nodeVar193;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar195 = ( indirectSpecular + nodeVar188 );
	indirectSpecular = nodeVar195;
	nodeVar196 = ( indirectDiffuse + nodeVar194 );
	indirectDiffuse = nodeVar196;
	ambientOcclusion = 1.0;
	nodeVar197 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar197;
	nodeVar198 = dot( normalView, positionViewDirection );
	nodeVar199 = ( clamp( nodeVar198, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar200 = ( Roughness * -16.0 );
	nodeVar201 = ( 1.0 - nodeVar200 );
	nodeVar202 = nodeVar201;
	nodeVar203 = ( - nodeVar202 );
	nodeVar204 = exp2( nodeVar203 );
	nodeVar205 = pow( nodeVar199, nodeVar204 );
	nodeVar206 = ( 1.0 - nodeVar205 );
	nodeVar207 = nodeVar206;
	nodeVar208 = ( ambientOcclusion - nodeVar207 );
	nodeVar209 = ( indirectSpecular * vec3<f32>( clamp( nodeVar208, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar209;
	nodeVar210 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar210;
	nodeVar211 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar211;
	nodeVar212 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar212;
	nodeVar213 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar213;

	// result

	output.color = nodeVar213;

	return output;

}
