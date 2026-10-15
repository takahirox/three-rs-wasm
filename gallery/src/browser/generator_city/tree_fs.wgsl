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
@binding( 3 ) @group( 1 ) var nodeUniform16_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform16 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform20 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform30_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform30 : texture_2d<f32>;

struct objectStruct {
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : mat4x4<f32>,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform1 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform19 : f32,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : bool;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> Metalness : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : bool;
var<private> Roughness : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_nodeVar14 : vec3<f32>;
var<private> NORMAL_nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : vec4<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : vec2<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec4<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec4<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec4<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec4<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec4<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec4<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec4<f32>;
var<private> nodeVar128 : array< vec3<f32>, 9 >;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec2<f32>;
var<private> nodeVar141 : vec4<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : vec2<f32>;
var<private> nodeVar149 : vec4<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : vec2<f32>;
var<private> nodeVar159 : vec4<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : vec2<f32>;
var<private> nodeVar167 : vec4<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : vec3<f32>;
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
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : f32;
var<private> nodeVar209 : f32;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : f32;
var<private> nodeVar218 : vec3<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : vec3<f32>;
var<private> nodeVar222 : vec3<f32>;
var<private> nodeVar223 : vec3<f32>;
var<private> nodeVar224 : vec3<f32>;
var<private> nodeVar225 : f32;
var<private> nodeVar226 : f32;
var<private> nodeVar227 : f32;
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
var<private> nodeVar239 : vec3<f32>;
var<private> nodeVar240 : vec3<f32>;
var<private> nodeVar241 : vec3<f32>;
var<private> nodeVar242 : vec3<f32>;
var<private> nodeVar243 : vec3<f32>;
var<private> nodeVar244 : vec3<f32>;
var<private> nodeVar245 : vec3<f32>;
var<private> nodeVar246 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar247 : vec3<f32>;
var<private> nodeVar248 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar249 : vec3<f32>;
var<private> nodeVar250 : f32;
var<private> nodeVar251 : f32;
var<private> nodeVar252 : f32;
var<private> nodeVar253 : f32;
var<private> nodeVar254 : f32;
var<private> nodeVar255 : f32;
var<private> nodeVar256 : f32;
var<private> nodeVar257 : f32;
var<private> nodeVar258 : f32;
var<private> nodeVar259 : f32;
var<private> nodeVar260 : f32;
var<private> nodeVar261 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar262 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar263 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar264 : vec3<f32>;
var<private> nodeVar265 : vec4<f32>;

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
	@location( 1 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 2 ) v_normalWorldGeometry : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) v_positionViewDirection : vec3<f32>,
	@location( 6 ) @interpolate(flat, either) nodeVarying9 : u32,
	@location( 7 ) nodeVarying10 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar4 = ( nodeVarying3 == 1.0 );

	if ( nodeVar4 ) {

		normalWorldGeometry = normalize( v_normalWorldGeometry );
		nodeVar3 = ( ( ( mix( mix( mix( vec3<f32>( 0.033104766565152086, 0.08228270712149792, 0.013702083043526807 ), vec3<f32>( 0.10946171076915331, 0.20155625378383743, 0.033104766565152086 ), ( ( normalWorldGeometry.y * 0.5 ) + 0.5 ) ), vec3<f32>( 0.1844749944900301, 0.22322795730611386, 0.028426039499072558 ), ( ( ( ( mx_fractal_noise_float( ( v_positionWorld * vec3<f32>( 0.55 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) * 0.55 ) ), vec3<f32>( 0.20507873637973145, 0.20155625378383743, 0.025186859622305935 ), ( fract( ( sin( ( f32( nodeVarying9 ) * 12.9898 ) ) * 43758.5453 ) ) * 0.3 ) ) * vec3<f32>( ( ( ( ( ( mx_fractal_noise_float( ( v_positionWorld * vec3<f32>( 2.2 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) * 0.35 ) + 0.75 ) ) ) * vec3<f32>( ( ( ( ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 9.0 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.24 ) + 0.88 ) ) ) * vec3<f32>( ( ( smoothstep( 0.1, 1.05, ( length( ( nodeVarying10 - vec3<f32>( 0.0, 3.9, 0.1 ) ) ) / 3.1 ) ) * 0.5 ) + 0.5 ) ) );

	} else {


		if ( ( nodeVarying3 == 2.0 ) ) {

			nodeVar6 = mix( vec3<f32>( 0.006048833020386069, 0.005605391621829108, 0.005181516700061659 ), vec3<f32>( 0.04231141061442144, 0.03954623527052923, 0.035601314869097636 ), smoothstep( 0.35, 0.5, fract( ( length( nodeVarying10.xz ) * 11.0 ) ) ) );

		} else {

			nodeVar6 = mix( vec3<f32>( 0.06847816983662762, 0.04231141061442144, 0.023153366173251363 ), vec3<f32>( 0.158960835050774, 0.11193242782769693, 0.061246054224174035 ), ( smoothstep( 0.55, 1.0, ( 1.0 - abs( ( mx_fractal_noise_float( vec3<f32>( ( nodeVarying10.x * 9.0 ), ( nodeVarying10.y * 1.4 ), ( nodeVarying10.z * 9.0 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) ) ) * 0.7 ) );

		}

		nodeVar3 = nodeVar6;

	}

	DiffuseColor = vec4<f32>( nodeVar3, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform5 );
	DiffuseColor.w = 1.0;
	nodeVar8 = ( nodeVarying3 == 2.0 );

	if ( nodeVar8 ) {

		nodeVar7 = 0.4;

	} else {

		nodeVar7 = 0.0;

	}

	Metalness = nodeVar7;

	if ( nodeVar8 ) {

		nodeVar9 = 0.7;

	} else {


		if ( nodeVar4 ) {

			nodeVar10 = ( ( ( ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 9.0 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.15 ) + 0.8 );

		} else {

			nodeVar10 = ( ( ( 1.0 - abs( ( mx_fractal_noise_float( vec3<f32>( ( nodeVarying10.x * 9.0 ), ( nodeVarying10.y * 1.4 ), ( nodeVarying10.z * 9.0 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) ) * 0.12 ) + 0.82 );

		}

		nodeVar9 = nodeVar10;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar11 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar9, 0.0525 ) + max( max( nodeVar11.x, nodeVar11.y ), nodeVar11.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeVar7 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	nodeVar12 = dpdx( v_positionView );
	nodeVar13 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_nodeVar14 = cross( nodeVar13, NORMAL_normalView );
	NORMAL_nodeVar15 = dot( nodeVar12, NORMAL_nodeVar14 );

	if ( nodeVar4 ) {

		nodeVar16 = ( ( ( ( mx_fractal_noise_float( ( v_positionWorld * vec3<f32>( 2.2 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) * 0.07 );

	} else {


		if ( nodeVar8 ) {

			nodeVar17 = 0.0;

		} else {

			nodeVar17 = ( ( 1.0 - abs( ( mx_fractal_noise_float( vec3<f32>( ( nodeVarying10.x * 9.0 ), ( nodeVarying10.y * 1.4 ), ( nodeVarying10.z * 9.0 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) ) * 0.02 );

		}

		nodeVar16 = nodeVar17;

	}

	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar18 = ( nodeVar16 * smoothstep( 0.08, 0.35, abs( dot( NORMAL_normalView, positionViewDirection ) ) ) );
	nodeVar19 = ( vec3<f32>( sign( NORMAL_nodeVar15 ) ) * ( ( vec3<f32>( dpdx( nodeVar18 ) ) * NORMAL_nodeVar14 ) + ( vec3<f32>( - dpdy( nodeVar18 ) ) * cross( NORMAL_normalView, nodeVar12 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeVar15 ) ) * NORMAL_normalView ) - nodeVar19 ) );
	nodeVar20 = dot( normalView, positionViewDirection );
	nodeVar21 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar20, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar21;
	nodeVar22 = ( dfg.x + dfg.y );
	nodeVar23 = ( 1.0 / nodeVar22 );
	nodeVar24 = nodeVar23;
	nodeVar25 = ( nodeVar24 - 1.0 );
	nodeVar26 = ( SpecularColorBlended * vec3<f32>( nodeVar25 ) );
	nodeVar27 = ( nodeVar26 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar27;
	nodeVar28 = ( render.nodeUniform10 - render.nodeUniform11 );
	nodeVar29 = vec4<f32>( nodeVar28, 0.0 );
	nodeVar30 = ( render.cameraViewMatrix * nodeVar29 );
	nodeVar31 = normalize( nodeVar30.xyz );
	nodeVar32 = nodeVar31;
	nodeVar33 = dot( normalView, nodeVar32 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar35 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	nodeVar36 = ( nodeVar35.xyz / vec3<f32>( nodeVar35.w ) );
	nodeVar37 = vec3<f32>( nodeVar36.x, ( 1.0 - nodeVar36.y ), ( nodeVar36.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeVar37.x >= 0.0 ) && ( nodeVar37.x <= 1.0 ) ) && ( nodeVar37.y >= 0.0 ) ) && ( nodeVar37.y <= 1.0 ) ) && ( nodeVar37.z <= 1.0 ) ) ) {

		nodeVar38 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar39 = ( render.nodeUniform17 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ).x );
		nodeVar40 = ( nodeVar37.xy + ( vogelDiskSample( 0, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar41 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar40, nodeVar37.z );
		nodeVar42 = ( nodeVar37.xy + ( vogelDiskSample( 1, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar43 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar42, nodeVar37.z );
		nodeVar44 = ( nodeVar37.xy + ( vogelDiskSample( 2, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar45 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar44, nodeVar37.z );
		nodeVar46 = ( nodeVar37.xy + ( vogelDiskSample( 3, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar47 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar46, nodeVar37.z );
		nodeVar48 = ( nodeVar37.xy + ( vogelDiskSample( 4, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar49 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar48, nodeVar37.z );
		nodeVar34 = ( ( ( ( ( nodeVar41 + nodeVar43 ) + nodeVar45 ) + nodeVar47 ) + nodeVar49 ) * 0.2 );

	} else {

		nodeVar34 = 1.0;

	}

	nodeVar50 = mix( 1.0, nodeVar34, render.nodeUniform19 );
	nodeVar51 = ( vec3<f32>( clamp( nodeVar33, 0.0, 1.0 ) ) * ( render.nodeUniform12 * vec3<f32>( nodeVar50 ) ) );
	nodeVar52 = nodeVar51;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar53 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar54 = ( nodeVar52 * nodeVar53 );
	nodeVar55 = ( nodeVar32 + positionViewDirection );
	nodeVar56 = normalize( nodeVar55 );
	nodeVar57 = dot( positionViewDirection, nodeVar56 );
	nodeVar58 = clamp( nodeVar57, 0.0, 1.0 );
	nodeVar59 = exp2( ( ( ( nodeVar58 * -5.55473 ) - 6.98316 ) * nodeVar58 ) );
	nodeVar60 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar59 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar59 ) ) );
	nodeVar61 = ( vec3<f32>( 1.0 ) - nodeVar60 );
	nodeVar62 = nodeVar61;
	nodeVar63 = ( nodeVar54 * nodeVar62 );
	nodeVar64 = ( directDiffuse + nodeVar63 );
	directDiffuse = nodeVar64;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar65 = normalize( ( nodeVar32 + positionViewDirection ) );
	nodeVar66 = clamp( dot( positionViewDirection, nodeVar65 ), 0.0, 1.0 );
	nodeVar67 = exp2( ( ( ( nodeVar66 * -5.55473 ) - 6.98316 ) * nodeVar66 ) );
	nodeVar68 = ( Roughness * Roughness );
	nodeVar69 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar67 ) ) ) + vec3<f32>( ( 1.0 * nodeVar67 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar68, clamp( dot( normalView, nodeVar32 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar68, clamp( dot( normalView, nodeVar65 ), 0.0, 1.0 ) ) ) );
	nodeVar70 = ( nodeVar52 * nodeVar69 );
	nodeVar71 = ( nodeVar70 * multiScatteringCompensation );
	nodeVar72 = ( directSpecular + nodeVar71 );
	directSpecular = nodeVar72;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar73 = ( object.nodeUniform21 - object.nodeUniform22 );
	nodeVar74 = ( object.nodeUniform23 - vec3<f32>( 1.0 ) );
	nodeVar75 = ( nodeVar73 / nodeVar74 );
	nodeVar76 = ( normalWorld * nodeVar75 );
	nodeVar77 = ( nodeVar76 * vec3<f32>( 0.5 ) );
	nodeVar78 = ( v_positionWorld + nodeVar77 );
	nodeVar79 = ( nodeVar78 - object.nodeUniform22 );
	nodeVar80 = ( nodeVar79 / nodeVar73 );
	nodeVar81 = ( clamp( nodeVar80, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar74 );
	nodeVar82 = ( nodeVar81 / object.nodeUniform23 );
	nodeVar83 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform23 );
	nodeVar84 = ( nodeVar82 + nodeVar83 );
	nodeVar85 = ( nodeVar84.z * object.nodeUniform23.z );
	nodeVar86 = ( nodeVar85 + 1.0 );
	nodeVar87 = ( object.nodeUniform23.z + 2.0 );
	nodeVar88 = ( nodeVar87 * 0.0 );
	nodeVar89 = ( nodeVar86 + nodeVar88 );
	nodeVar90 = ( nodeVar87 * 7.0 );
	nodeVar91 = ( nodeVar89 / nodeVar90 );
	nodeVar92 = vec3<f32>( nodeVar84.xy, nodeVar91 );
	nodeVar93 = textureSample( nodeUniform20, nodeUniform20_sampler, nodeVar92 );
	nodeVar94 = ( nodeVar87 * 1.0 );
	nodeVar95 = ( nodeVar86 + nodeVar94 );
	nodeVar96 = ( nodeVar95 / nodeVar90 );
	nodeVar97 = vec3<f32>( nodeVar84.xy, nodeVar96 );
	nodeVar98 = textureSample( nodeUniform20, nodeUniform20_sampler, nodeVar97 );
	nodeVar99 = vec3<f32>( nodeVar93.w, nodeVar98.xy );
	nodeVar100 = ( nodeVar87 * 2.0 );
	nodeVar101 = ( nodeVar86 + nodeVar100 );
	nodeVar102 = ( nodeVar101 / nodeVar90 );
	nodeVar103 = vec3<f32>( nodeVar84.xy, nodeVar102 );
	nodeVar104 = textureSample( nodeUniform20, nodeUniform20_sampler, nodeVar103 );
	nodeVar105 = vec3<f32>( nodeVar98.zw, nodeVar104.x );
	nodeVar106 = ( nodeVar87 * 3.0 );
	nodeVar107 = ( nodeVar86 + nodeVar106 );
	nodeVar108 = ( nodeVar107 / nodeVar90 );
	nodeVar109 = vec3<f32>( nodeVar84.xy, nodeVar108 );
	nodeVar110 = textureSample( nodeUniform20, nodeUniform20_sampler, nodeVar109 );
	nodeVar111 = ( nodeVar87 * 4.0 );
	nodeVar112 = ( nodeVar86 + nodeVar111 );
	nodeVar113 = ( nodeVar112 / nodeVar90 );
	nodeVar114 = vec3<f32>( nodeVar84.xy, nodeVar113 );
	nodeVar115 = textureSample( nodeUniform20, nodeUniform20_sampler, nodeVar114 );
	nodeVar116 = vec3<f32>( nodeVar110.w, nodeVar115.xy );
	nodeVar117 = ( nodeVar87 * 5.0 );
	nodeVar118 = ( nodeVar86 + nodeVar117 );
	nodeVar119 = ( nodeVar118 / nodeVar90 );
	nodeVar120 = vec3<f32>( nodeVar84.xy, nodeVar119 );
	nodeVar121 = textureSample( nodeUniform20, nodeUniform20_sampler, nodeVar120 );
	nodeVar122 = vec3<f32>( nodeVar115.zw, nodeVar121.x );
	nodeVar123 = ( nodeVar87 * 6.0 );
	nodeVar124 = ( nodeVar86 + nodeVar123 );
	nodeVar125 = ( nodeVar124 / nodeVar90 );
	nodeVar126 = vec3<f32>( nodeVar84.xy, nodeVar125 );
	nodeVar127 = textureSample( nodeUniform20, nodeUniform20_sampler, nodeVar126 );
	nodeVar128 = array< vec3<f32>, 9 >( nodeVar93.xyz, nodeVar99, nodeVar105, nodeVar104.yzw, nodeVar110.xyz, nodeVar116, nodeVar122, nodeVar121.yzw, nodeVar127.xyz );
	nodeVar129 = ( ( ( ( ( ( ( ( ( nodeVar128[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar128[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar128[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar128[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar128[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar128[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar128[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar128[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar128[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar130 = max( nodeVar129, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar131 = ( nodeVar130 * vec3<f32>( object.nodeUniform24 ) );
	nodeVar132 = ( irradiance + nodeVar131 );
	irradiance = nodeVar132;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar133 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform25 );
	nodeVar134 = floor( nodeVar133 );
	nodeVar135 = nodeVar134;
	nodeVar136 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar137 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar136.x, ( - nodeVar136.y ), nodeVar136.z ), 1.0 ) ).xyz );
	nodeVar138 = max( ( 4.0 - nodeVar135 ), 0.0 );
	nodeVar135 = max( nodeVar135, 4.0 );
	nodeVar139 = exp2( nodeVar135 );
	nodeVar140 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar136.x, ( - nodeVar136.y ), nodeVar136.z ), 1.0 ) ).xyz, nodeVar137 ) * vec2<f32>( ( nodeVar139 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar137 > 2.0 ) ) {

		nodeVar140.y = ( nodeVar140.y + nodeVar139 );
		nodeVar137 = ( nodeVar137 - 3.0 );
		

	}

	nodeVar140.x = ( nodeVar140.x + ( nodeVar137 * nodeVar139 ) );
	nodeVar140.x = ( nodeVar140.x + ( nodeVar138 * ( 3.0 * 16.0 ) ) );
	nodeVar140.y = ( nodeVar140.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar139 ) ) );
	nodeVar140.x = ( nodeVar140.x * object.nodeUniform28 );
	nodeVar140.y = ( nodeVar140.y * object.nodeUniform29 );
	nodeVar141 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar140, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar142 = nodeVar141.xyz;
	nodeVar143 = fract( nodeVar133 );

	if ( ( nodeVar143 != 0.0 ) ) {

		nodeVar144 = ( nodeVar134 + 1.0 );
		nodeVar145 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar136.x, ( - nodeVar136.y ), nodeVar136.z ), 1.0 ) ).xyz );
		nodeVar146 = max( ( 4.0 - nodeVar144 ), 0.0 );
		nodeVar144 = max( nodeVar144, 4.0 );
		nodeVar147 = exp2( nodeVar144 );
		nodeVar148 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar136.x, ( - nodeVar136.y ), nodeVar136.z ), 1.0 ) ).xyz, nodeVar145 ) * vec2<f32>( ( nodeVar147 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar145 > 2.0 ) ) {

			nodeVar148.y = ( nodeVar148.y + nodeVar147 );
			nodeVar145 = ( nodeVar145 - 3.0 );
			

		}

		nodeVar148.x = ( nodeVar148.x + ( nodeVar145 * nodeVar147 ) );
		nodeVar148.x = ( nodeVar148.x + ( nodeVar146 * ( 3.0 * 16.0 ) ) );
		nodeVar148.y = ( nodeVar148.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar147 ) ) );
		nodeVar148.x = ( nodeVar148.x * object.nodeUniform28 );
		nodeVar148.y = ( nodeVar148.y * object.nodeUniform29 );
		nodeVar149 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar148, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar150 = nodeVar149.xyz;
		nodeVar142 = mix( nodeVar142, nodeVar150, nodeVar143 );
		

	}

	nodeVar151 = ( radiance + ( nodeVar142 * vec3<f32>( object.nodeUniform31 ) ) );
	radiance = nodeVar151;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar152 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform25 );
	nodeVar153 = floor( nodeVar152 );
	nodeVar154 = nodeVar153;
	nodeVar155 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar156 = max( ( 4.0 - nodeVar154 ), 0.0 );
	nodeVar154 = max( nodeVar154, 4.0 );
	nodeVar157 = exp2( nodeVar154 );
	nodeVar158 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar155 ) * vec2<f32>( ( nodeVar157 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar155 > 2.0 ) ) {

		nodeVar158.y = ( nodeVar158.y + nodeVar157 );
		nodeVar155 = ( nodeVar155 - 3.0 );
		

	}

	nodeVar158.x = ( nodeVar158.x + ( nodeVar155 * nodeVar157 ) );
	nodeVar158.x = ( nodeVar158.x + ( nodeVar156 * ( 3.0 * 16.0 ) ) );
	nodeVar158.y = ( nodeVar158.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar157 ) ) );
	nodeVar158.x = ( nodeVar158.x * object.nodeUniform28 );
	nodeVar158.y = ( nodeVar158.y * object.nodeUniform29 );
	nodeVar159 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar158, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar160 = nodeVar159.xyz;
	nodeVar161 = fract( nodeVar152 );

	if ( ( nodeVar161 != 0.0 ) ) {

		nodeVar162 = ( nodeVar153 + 1.0 );
		nodeVar163 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar164 = max( ( 4.0 - nodeVar162 ), 0.0 );
		nodeVar162 = max( nodeVar162, 4.0 );
		nodeVar165 = exp2( nodeVar162 );
		nodeVar166 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar163 ) * vec2<f32>( ( nodeVar165 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar163 > 2.0 ) ) {

			nodeVar166.y = ( nodeVar166.y + nodeVar165 );
			nodeVar163 = ( nodeVar163 - 3.0 );
			

		}

		nodeVar166.x = ( nodeVar166.x + ( nodeVar163 * nodeVar165 ) );
		nodeVar166.x = ( nodeVar166.x + ( nodeVar164 * ( 3.0 * 16.0 ) ) );
		nodeVar166.y = ( nodeVar166.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar165 ) ) );
		nodeVar166.x = ( nodeVar166.x * object.nodeUniform28 );
		nodeVar166.y = ( nodeVar166.y * object.nodeUniform29 );
		nodeVar167 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar166, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar168 = nodeVar167.xyz;
		nodeVar160 = mix( nodeVar160, nodeVar168, nodeVar161 );
		

	}

	nodeVar169 = ( iblIrradiance + ( ( nodeVar160 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform31 ) ) );
	iblIrradiance = nodeVar169;
	nodeVar170 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar171 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar172 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar173 = ( SpecularF90 * dfg.y );
	nodeVar174 = ( nodeVar172 + vec3<f32>( nodeVar173 ) );
	nodeVar175 = ( nodeVar170 + nodeVar174 );
	nodeVar170 = nodeVar175;
	nodeVar176 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar177 = nodeVar176;
	nodeVar178 = ( nodeVar177 * vec3<f32>( 0.047619 ) );
	nodeVar179 = ( SpecularColor + nodeVar178 );
	nodeVar180 = ( nodeVar174 * nodeVar179 );
	nodeVar181 = ( dfg.x + dfg.y );
	nodeVar182 = ( 1.0 - nodeVar181 );
	nodeVar183 = nodeVar182;
	nodeVar184 = ( vec3<f32>( nodeVar183 ) * nodeVar179 );
	nodeVar185 = ( vec3<f32>( 1.0 ) - nodeVar184 );
	nodeVar186 = nodeVar185;
	nodeVar187 = ( nodeVar180 / nodeVar186 );
	nodeVar188 = ( nodeVar187 * vec3<f32>( nodeVar183 ) );
	nodeVar189 = ( nodeVar171 + nodeVar188 );
	nodeVar171 = nodeVar189;
	nodeVar190 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar191 = ( irradiance * nodeVar190 );
	nodeVar192 = ( nodeVar170 + nodeVar171 );
	nodeVar193 = ( vec3<f32>( 1.0 ) - nodeVar192 );
	nodeVar194 = nodeVar193;
	nodeVar195 = ( nodeVar191 * nodeVar194 );
	nodeVar196 = nodeVar195;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar197 = ( indirectDiffuse + nodeVar196 );
	indirectDiffuse = nodeVar197;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar198 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar199 = ( SpecularF90 * dfg.y );
	nodeVar200 = ( nodeVar198 + vec3<f32>( nodeVar199 ) );
	nodeVar201 = ( singleScatteringDielectric + nodeVar200 );
	singleScatteringDielectric = nodeVar201;
	nodeVar202 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar203 = nodeVar202;
	nodeVar204 = ( nodeVar203 * vec3<f32>( 0.047619 ) );
	nodeVar205 = ( SpecularColor + nodeVar204 );
	nodeVar206 = ( nodeVar200 * nodeVar205 );
	nodeVar207 = ( dfg.x + dfg.y );
	nodeVar208 = ( 1.0 - nodeVar207 );
	nodeVar209 = nodeVar208;
	nodeVar210 = ( vec3<f32>( nodeVar209 ) * nodeVar205 );
	nodeVar211 = ( vec3<f32>( 1.0 ) - nodeVar210 );
	nodeVar212 = nodeVar211;
	nodeVar213 = ( nodeVar206 / nodeVar212 );
	nodeVar214 = ( nodeVar213 * vec3<f32>( nodeVar209 ) );
	nodeVar215 = ( multiScatteringDielectric + nodeVar214 );
	multiScatteringDielectric = nodeVar215;
	nodeVar216 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar217 = ( SpecularF90 * dfg.y );
	nodeVar218 = ( nodeVar216 + vec3<f32>( nodeVar217 ) );
	nodeVar219 = ( singleScatteringMetallic + nodeVar218 );
	singleScatteringMetallic = nodeVar219;
	nodeVar220 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar221 = nodeVar220;
	nodeVar222 = ( nodeVar221 * vec3<f32>( 0.047619 ) );
	nodeVar223 = ( DiffuseColor.xyz + nodeVar222 );
	nodeVar224 = ( nodeVar218 * nodeVar223 );
	nodeVar225 = ( dfg.x + dfg.y );
	nodeVar226 = ( 1.0 - nodeVar225 );
	nodeVar227 = nodeVar226;
	nodeVar228 = ( vec3<f32>( nodeVar227 ) * nodeVar223 );
	nodeVar229 = ( vec3<f32>( 1.0 ) - nodeVar228 );
	nodeVar230 = nodeVar229;
	nodeVar231 = ( nodeVar224 / nodeVar230 );
	nodeVar232 = ( nodeVar231 * vec3<f32>( nodeVar227 ) );
	nodeVar233 = ( multiScatteringMetallic + nodeVar232 );
	multiScatteringMetallic = nodeVar233;
	nodeVar234 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar235 = ( radiance * nodeVar234 );
	nodeVar236 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar237 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar238 = ( nodeVar236 * nodeVar237 );
	nodeVar239 = ( nodeVar235 + nodeVar238 );
	nodeVar240 = nodeVar239;
	nodeVar241 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar242 = ( vec3<f32>( 1.0 ) - nodeVar241 );
	nodeVar243 = nodeVar242;
	nodeVar244 = ( DiffuseContribution * nodeVar243 );
	nodeVar245 = ( nodeVar244 * nodeVar237 );
	nodeVar246 = nodeVar245;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar247 = ( indirectSpecular + nodeVar240 );
	indirectSpecular = nodeVar247;
	nodeVar248 = ( indirectDiffuse + nodeVar246 );
	indirectDiffuse = nodeVar248;
	ambientOcclusion = 1.0;
	nodeVar249 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar249;
	nodeVar250 = dot( normalView, positionViewDirection );
	nodeVar251 = ( clamp( nodeVar250, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar252 = ( Roughness * -16.0 );
	nodeVar253 = ( 1.0 - nodeVar252 );
	nodeVar254 = nodeVar253;
	nodeVar255 = ( - nodeVar254 );
	nodeVar256 = exp2( nodeVar255 );
	nodeVar257 = pow( nodeVar251, nodeVar256 );
	nodeVar258 = ( 1.0 - nodeVar257 );
	nodeVar259 = nodeVar258;
	nodeVar260 = ( ambientOcclusion - nodeVar259 );
	nodeVar261 = ( indirectSpecular * vec3<f32>( clamp( nodeVar260, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar261;
	nodeVar262 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar262;
	nodeVar263 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar263;
	nodeVar264 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar264;
	nodeVar265 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar265;

	// result

	output.color = nodeVar265;

	return output;

}
