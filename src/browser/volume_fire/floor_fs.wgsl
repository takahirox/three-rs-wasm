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
@binding( 3 ) @group( 1 ) var nodeUniform38_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform38 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform43_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform43 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform33 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform47 : f32,
	nodeUniform48 : f32,
	nodeUniform51 : f32,
	nodeUniform52 : f32,
	nodeUniform37 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform36 : vec3<f32>,
	nodeUniform49 : vec3<f32>,
	nodeUniform50 : vec3<f32>,
	nodeUniform39 : mat4x4<f32>,
	nodeUniform41 : f32,
	nodeUniform42 : f32,
	nodeUniform46 : f32,
	nodeUniform44 : f32,
	nodeUniform45 : vec2<f32>
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
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar58 : vec4<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec4<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec2<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec2<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec2<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec2<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec2<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec4<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec4<f32>;
var<private> nodeVar78 : vec4<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec4<f32>;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec4<f32>;
var<private> nodeVar89 : vec4<f32>;
var<private> nodeVar90 : vec4<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec4<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec4<f32>;
var<private> nodeVar102 : vec4<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec4<f32>;
var<private> nodeVar109 : vec4<f32>;
var<private> nodeVar110 : vec4<f32>;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
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
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
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
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = dot( normalView, positionViewDirection );
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar1, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	nodeVar3 = ( dfg.x + dfg.y );
	nodeVar4 = ( 1.0 / nodeVar3 );
	nodeVar5 = nodeVar4;
	nodeVar6 = ( nodeVar5 - 1.0 );
	nodeVar7 = ( SpecularColorBlended * vec3<f32>( nodeVar6 ) );
	nodeVar8 = ( nodeVar7 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar8;
	nodeVar9 = ( render.nodeUniform10 - v_positionView );
	nodeVar10 = normalize( nodeVar9 );
	nodeVar11 = dot( normalView, nodeVar10 );

	if ( ( 0.0 == 1.0 ) ) {

		nodeVar13 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar14 = clamp( ( ( ( ( object.nodeUniform12 / 8.34 ) * 0.5 ) + 0.2 ) + object.nodeUniform13 ), 0.0, 1.0 );
		nodeVar13 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), object.nodeUniform11, smoothstep( 0.05, 0.35, nodeVar14 ) );
		nodeVar13 = mix( nodeVar13, object.nodeUniform14, smoothstep( 0.35, 0.65, nodeVar14 ) );
		nodeVar13 = mix( nodeVar13, object.nodeUniform15, smoothstep( 0.65, 1.0, nodeVar14 ) );
		nodeVar15 = cos( object.nodeUniform17 );
		nodeVar12 = max( ( ( max( mix( vec3<f32>( dot( nodeVar13, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar13, object.nodeUniform16 ), vec3<f32>( 0.0 ) ) * vec3<f32>( nodeVar15 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar13, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar13, object.nodeUniform16 ), vec3<f32>( 0.0 ) ) ) * vec3<f32>( sin( object.nodeUniform17 ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar13, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar13, object.nodeUniform16 ), vec3<f32>( 0.0 ) ) ) * ( 1.0 - nodeVar15 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

	} else {

		nodeVar16 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar17 = vec3<f32>( 0.0, object.nodeUniform20, 0.0 );
		nodeVar18 = length( ( v_positionWorld - ( ( object.nodeUniform18 + object.nodeUniform19 ) + ( nodeVar17 * vec3<f32>( clamp( ( dot( ( v_positionWorld - object.nodeUniform18 ), nodeVar17 ) / dot( nodeVar17, nodeVar17 ) ), 0.0, 1.0 ) ) ) ) ) );
		nodeVar19 = ( v_positionWorld.xz - object.nodeUniform18.xz );
		nodeVar20 = atan2( nodeVar19.y, nodeVar19.x );
		nodeVar21 = clamp( ( clamp( ( 1.0 - ( nodeVar18 / object.nodeUniform21 ) ), 0.0, 1.0 ) * mix( ( ( ( ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( v_positionWorld.x * ( 0.6 * object.nodeUniform22 ) ), ( object.nodeUniform23 * 1.2 ), ( v_positionWorld.z * ( 0.6 * object.nodeUniform22 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.65 ) + ( ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( v_positionWorld.x * ( 1.5 * object.nodeUniform22 ) ), ( object.nodeUniform23 * 2.5 ), ( v_positionWorld.z * ( 1.5 * object.nodeUniform22 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.35 ) ) * ( ( mix( 1.0, ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( cos( nodeVar20 ) * ( 1.5 * object.nodeUniform22 ) ), ( sin( nodeVar20 ) * ( 1.5 * object.nodeUniform22 ) ), ( object.nodeUniform23 * 0.6 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ), smoothstep( 0.0, object.nodeUniform24, length( nodeVar19 ) ) ) * 0.5 ) + 0.5 ) ), 1.0, clamp( ( nodeVar18 / object.nodeUniform25 ), 0.0, 1.0 ) ) ), 0.0, 1.0 );
		nodeVar16 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), object.nodeUniform11, smoothstep( 0.05, 0.35, nodeVar21 ) );
		nodeVar16 = mix( nodeVar16, object.nodeUniform14, smoothstep( 0.35, 0.65, nodeVar21 ) );
		nodeVar16 = mix( nodeVar16, object.nodeUniform15, smoothstep( 0.65, 1.0, nodeVar21 ) );
		nodeVar22 = cos( object.nodeUniform17 );
		nodeVar12 = max( ( ( max( mix( vec3<f32>( dot( nodeVar16, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar16, object.nodeUniform16 ), vec3<f32>( 0.0 ) ) * vec3<f32>( nodeVar22 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar16, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar16, object.nodeUniform16 ), vec3<f32>( 0.0 ) ) ) * vec3<f32>( sin( object.nodeUniform17 ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar16, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar16, object.nodeUniform16 ), vec3<f32>( 0.0 ) ) ) * ( 1.0 - nodeVar22 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

	}


	if ( ( 0.0 == 1.0 ) ) {

		nodeVar23 = object.nodeUniform28;

	} else {

		nodeVar23 = object.nodeUniform29;

	}


	if ( ( 0.0 == 1.0 ) ) {

		nodeVar25 = vec3<f32>( 0.0, object.nodeUniform20, 0.0 );
		nodeVar26 = length( ( v_positionWorld - ( ( object.nodeUniform18 + object.nodeUniform19 ) + ( nodeVar25 * vec3<f32>( clamp( ( dot( ( v_positionWorld - object.nodeUniform18 ), nodeVar25 ) / dot( nodeVar25, nodeVar25 ) ), 0.0, 1.0 ) ) ) ) ) );
		nodeVar24 = ( ( 1.0 / ( pow( nodeVar26, 2.0 ) + pow( 1.2, 2.0 ) ) ) / ( 1.0 / max( pow( length( ( v_positionWorld - object.nodeUniform18 ) ), 2.0 ), 0.01 ) ) );

	} else {

		nodeVar24 = 1.0;

	}


	if ( ( 0.0 == 1.0 ) ) {

		nodeVar27 = mix( object.nodeUniform31, object.nodeUniform32, smoothstep( 0.0, 1.0, clamp( ( nodeVar26 / object.nodeUniform33 ), 0.0, 1.0 ) ) );

	} else {

		nodeVar27 = 1.0;

	}


	if ( ( render.nodeUniform34 > 0.0 ) ) {

		nodeVar29 = length( nodeVar9 );
		nodeVar30 = ( nodeVar29 / render.nodeUniform34 );
		nodeVar31 = clamp( ( 1.0 - ( ( ( nodeVar30 * nodeVar30 ) * nodeVar30 ) * nodeVar30 ) ), 0.0, 1.0 );
		nodeVar28 = ( ( 1.0 / max( pow( nodeVar29, render.nodeUniform35 ), 0.01 ) ) * ( nodeVar31 * nodeVar31 ) );

	} else {

		nodeVar28 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform35 ), 0.01 ) );

	}

	nodeVar32 = ( ( ( ( ( ( ( ( ( nodeVar12 * vec3<f32>( pow( max( ( object.nodeUniform12 / 8.34 ), 0.0 ), 4.0 ) ) ) * vec3<f32>( max( ( object.nodeUniform26 / 11.02 ), 0.0 ) ) ) * vec3<f32>( object.nodeUniform27 ) ) * vec3<f32>( nodeVar23 ) ) * vec3<f32>( object.nodeUniform30 ) ) * vec3<f32>( smoothstep( 0.0, 3.0, object.nodeUniform23 ) ) ) * vec3<f32>( nodeVar24 ) ) * vec3<f32>( nodeVar27 ) ) * vec3<f32>( nodeVar28 ) );
	nodeVar33 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar32 );
	nodeVar34 = nodeVar33;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar35 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar36 = ( nodeVar34 * nodeVar35 );
	nodeVar37 = ( nodeVar10 + positionViewDirection );
	nodeVar38 = normalize( nodeVar37 );
	nodeVar39 = dot( positionViewDirection, nodeVar38 );
	nodeVar40 = clamp( nodeVar39, 0.0, 1.0 );
	nodeVar41 = exp2( ( ( ( nodeVar40 * -5.55473 ) - 6.98316 ) * nodeVar40 ) );
	nodeVar42 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar41 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar41 ) ) );
	nodeVar43 = ( vec3<f32>( 1.0 ) - nodeVar42 );
	nodeVar44 = nodeVar43;
	nodeVar45 = ( nodeVar36 * nodeVar44 );
	nodeVar46 = ( directDiffuse + nodeVar45 );
	directDiffuse = nodeVar46;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar47 = normalize( ( nodeVar10 + positionViewDirection ) );
	nodeVar48 = clamp( dot( positionViewDirection, nodeVar47 ), 0.0, 1.0 );
	nodeVar49 = exp2( ( ( ( nodeVar48 * -5.55473 ) - 6.98316 ) * nodeVar48 ) );
	nodeVar50 = ( Roughness * Roughness );
	nodeVar51 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar49 ) ) ) + vec3<f32>( ( 1.0 * nodeVar49 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar50, clamp( dot( normalView, nodeVar10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar50, clamp( dot( normalView, nodeVar47 ), 0.0, 1.0 ) ) ) );
	nodeVar52 = ( nodeVar34 * nodeVar51 );
	nodeVar53 = ( nodeVar52 * multiScatteringCompensation );
	nodeVar54 = ( directSpecular + nodeVar53 );
	directSpecular = nodeVar54;
	nodeVar55 = ( render.nodeUniform36 - v_positionView );
	nodeVar56 = normalize( nodeVar55 );
	nodeVar57 = dot( normalView, nodeVar56 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar58 = ( render.nodeUniform39 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform41 ) ) ), 1.0 ) );
	nodeVar59 = ( nodeVar58.xyz / vec3<f32>( nodeVar58.w ) );
	nodeVar60 = vec3<f32>( nodeVar59.x, ( 1.0 - nodeVar59.y ), ( nodeVar59.z + render.nodeUniform42 ) );
	nodeVar61 = textureSample( nodeUniform38, nodeUniform38_sampler, nodeVar60.xy );

	if ( ( ( ( ( ( nodeVar60.x >= 0.0 ) && ( nodeVar60.x <= 1.0 ) ) && ( nodeVar60.y >= 0.0 ) ) && ( nodeVar60.y <= 1.0 ) ) && ( nodeVar60.z <= 1.0 ) ) ) {

		nodeVar63 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar64 = ( render.nodeUniform44 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform45 ).x );
		nodeVar65 = ( nodeVar60.xy + ( vogelDiskSample( 0, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
		nodeVar66 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeVar65, nodeVar60.z );
		nodeVar67 = ( nodeVar60.xy + ( vogelDiskSample( 1, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
		nodeVar68 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeVar67, nodeVar60.z );
		nodeVar69 = ( nodeVar60.xy + ( vogelDiskSample( 2, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
		nodeVar70 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeVar69, nodeVar60.z );
		nodeVar71 = ( nodeVar60.xy + ( vogelDiskSample( 3, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
		nodeVar72 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeVar71, nodeVar60.z );
		nodeVar73 = ( nodeVar60.xy + ( vogelDiskSample( 4, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
		nodeVar74 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeVar73, nodeVar60.z );
		nodeVar62 = ( ( ( ( ( nodeVar66 + nodeVar68 ) + nodeVar70 ) + nodeVar72 ) + nodeVar74 ) * 0.2 );

	} else {

		nodeVar62 = 1.0;

	}

	nodeVar75 = mix( vec4<f32>( 1.0 ), mix( nodeVar61, vec4<f32>( 1.0 ), vec4<f32>( nodeVar62 ) ), ( render.nodeUniform46 * nodeVar61.w ) );
	nodeVar76 = ( render.nodeUniform49 - render.nodeUniform50 );
	nodeVar77 = vec4<f32>( nodeVar76, 0.0 );
	nodeVar78 = ( render.cameraViewMatrix * nodeVar77 );
	nodeVar79 = normalize( nodeVar78.xyz );
	nodeVar80 = nodeVar79;
	nodeVar81 = dot( nodeVar56, nodeVar80 );
	nodeVar82 = smoothstep( render.nodeUniform47, render.nodeUniform48, nodeVar81 );
	nodeVar83 = ( ( vec4<f32>( render.nodeUniform37, 1.0 ) * nodeVar75 ) * vec4<f32>( nodeVar82 ) );

	if ( ( render.nodeUniform51 > 0.0 ) ) {

		nodeVar85 = length( nodeVar55 );
		nodeVar86 = ( nodeVar85 / render.nodeUniform51 );
		nodeVar87 = clamp( ( 1.0 - ( ( ( nodeVar86 * nodeVar86 ) * nodeVar86 ) * nodeVar86 ) ), 0.0, 1.0 );
		nodeVar84 = ( ( 1.0 / max( pow( nodeVar85, render.nodeUniform52 ), 0.01 ) ) * ( nodeVar87 * nodeVar87 ) );

	} else {

		nodeVar84 = ( 1.0 / max( pow( length( nodeVar55 ), render.nodeUniform52 ), 0.01 ) );

	}

	nodeVar88 = ( nodeVar83 * vec4<f32>( nodeVar84 ) );
	nodeVar89 = ( vec4<f32>( clamp( nodeVar57, 0.0, 1.0 ) ) * nodeVar88 );
	nodeVar90 = nodeVar89;
	nodeVar91 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar92 = ( nodeVar90 * vec4<f32>( nodeVar91, 1.0 ) );
	nodeVar93 = ( nodeVar56 + positionViewDirection );
	nodeVar94 = normalize( nodeVar93 );
	nodeVar95 = dot( positionViewDirection, nodeVar94 );
	nodeVar96 = clamp( nodeVar95, 0.0, 1.0 );
	nodeVar97 = exp2( ( ( ( nodeVar96 * -5.55473 ) - 6.98316 ) * nodeVar96 ) );
	nodeVar98 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar97 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar97 ) ) );
	nodeVar99 = ( vec3<f32>( 1.0 ) - nodeVar98 );
	nodeVar100 = nodeVar99;
	nodeVar101 = ( nodeVar92 * vec4<f32>( nodeVar100, 1.0 ) );
	nodeVar102 = ( vec4<f32>( directDiffuse, 1.0 ) + nodeVar101 );
	directDiffuse = nodeVar102.xyz;
	nodeVar103 = normalize( ( nodeVar56 + positionViewDirection ) );
	nodeVar104 = clamp( dot( positionViewDirection, nodeVar103 ), 0.0, 1.0 );
	nodeVar105 = exp2( ( ( ( nodeVar104 * -5.55473 ) - 6.98316 ) * nodeVar104 ) );
	nodeVar106 = ( Roughness * Roughness );
	nodeVar107 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar105 ) ) ) + vec3<f32>( ( 1.0 * nodeVar105 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar106, clamp( dot( normalView, nodeVar56 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar106, clamp( dot( normalView, nodeVar103 ), 0.0, 1.0 ) ) ) );
	nodeVar108 = ( nodeVar90 * vec4<f32>( nodeVar107, 1.0 ) );
	nodeVar109 = ( nodeVar108 * vec4<f32>( multiScatteringCompensation, 1.0 ) );
	nodeVar110 = ( vec4<f32>( directSpecular, 1.0 ) + nodeVar109 );
	directSpecular = nodeVar110.xyz;
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
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar175 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar176 = ( radiance * nodeVar175 );
	nodeVar177 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
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
