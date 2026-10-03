// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform17_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform17 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform21_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform21 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform31_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform31 : texture_2d<f32>;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
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

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
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

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_nodeVar12 : vec3<f32>;
var<private> NORMAL_nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec2<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar32 : vec4<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec2<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec4<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec4<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec4<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec4<f32>;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec4<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec4<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec4<f32>;
var<private> nodeVar125 : array< vec3<f32>, 9 >;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : vec2<f32>;
var<private> nodeVar138 : vec4<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : vec2<f32>;
var<private> nodeVar146 : vec4<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
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
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
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
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : f32;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar244 : vec3<f32>;
var<private> nodeVar245 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar246 : vec3<f32>;
var<private> nodeVar247 : f32;
var<private> nodeVar248 : f32;
var<private> nodeVar249 : f32;
var<private> nodeVar250 : f32;
var<private> nodeVar251 : f32;
var<private> nodeVar252 : f32;
var<private> nodeVar253 : f32;
var<private> nodeVar254 : f32;
var<private> nodeVar255 : f32;
var<private> nodeVar256 : f32;
var<private> nodeVar257 : f32;
var<private> nodeVar258 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar259 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar260 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar261 : vec3<f32>;
var<private> nodeVar262 : vec4<f32>;

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
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalWorldGeometry : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 0.6 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 );
	nodeVar1 = 0.0;
	nodeVar2 = smoothstep( 200.0, 18.0, distance( v_positionWorld, render.cameraPosition ) );

	if ( ( nodeVar2 > 0.0 ) ) {

		nodeVar1 = ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 18.0 ) ) ) * 1.0 ) + 0.0 ) * 0.05 );
		

	}

	nodeVar3 = ( mix( vec3<f32>( 0.061246054224174035, 0.061246054224174035, 0.04970656597728775 ), vec3<f32>( 0.10702310296918527, 0.10702310296918527, 0.08865558627723595 ), nodeVar0 ) + vec3<f32>( ( nodeVar1 * nodeVar2 ) ) );
	normalWorldGeometry = normalize( v_normalWorldGeometry );
	nodeVar5 = ( v_positionWorld.x / 1.5 );
	nodeVar6 = max( fwidth( nodeVar5 ), 0.0001 );
	nodeVar7 = ( v_positionWorld.z / 1.5 );
	nodeVar8 = max( fwidth( nodeVar7 ), 0.0001 );
	DiffuseColor = vec4<f32>( ( mix( ( nodeVar3 * vec3<f32>( 0.7 ) ), nodeVar3, smoothstep( 0.5, 0.85, normalWorldGeometry.y ) ) * vec3<f32>( ( 1.0 - ( ( max( smoothstep( ( 0.02666666666666667 + nodeVar6 ), ( 0.02666666666666667 - nodeVar6 ), ( 0.5 - abs( ( fract( nodeVar5 ) - 0.5 ) ) ) ), smoothstep( ( 0.02666666666666667 + nodeVar8 ), ( 0.02666666666666667 - nodeVar8 ), ( 0.5 - abs( ( fract( nodeVar7 ) - 0.5 ) ) ) ) ) * nodeVar2 ) * 0.4 ) ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform5 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform6;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar9 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( 0.7 + ( nodeVar0 * 0.1 ) ), 0.0525 ) + max( max( nodeVar9.x, nodeVar9.y ), nodeVar9.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform6 ) ) );
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	nodeVar10 = dpdx( v_positionView );
	nodeVar11 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_nodeVar12 = cross( nodeVar11, NORMAL_normalView );
	NORMAL_nodeVar13 = dot( nodeVar10, NORMAL_nodeVar12 );
	nodeVar14 = 0.0;

	if ( ( nodeVar2 > 0.0 ) ) {

		nodeVar14 = ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 4.0 ) ) ) * 1.0 ) + 0.0 ) * 0.002 );
		

	}

	nodeVar15 = ( nodeVar14 * nodeVar2 );
	nodeVar16 = ( vec3<f32>( sign( NORMAL_nodeVar13 ) ) * ( ( vec3<f32>( dpdx( nodeVar15 ) ) * NORMAL_nodeVar12 ) + ( vec3<f32>( - dpdy( nodeVar15 ) ) * cross( NORMAL_normalView, nodeVar10 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeVar13 ) ) * NORMAL_normalView ) - nodeVar16 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar17 = dot( normalView, positionViewDirection );
	nodeVar18 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( Roughness, clamp( nodeVar17, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar18;
	nodeVar19 = ( dfg.x + dfg.y );
	nodeVar20 = ( 1.0 / nodeVar19 );
	nodeVar21 = nodeVar20;
	nodeVar22 = ( nodeVar21 - 1.0 );
	nodeVar23 = ( SpecularColorBlended * vec3<f32>( nodeVar22 ) );
	nodeVar24 = ( nodeVar23 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar24;
	nodeVar25 = ( render.nodeUniform11 - render.nodeUniform12 );
	nodeVar26 = vec4<f32>( nodeVar25, 0.0 );
	nodeVar27 = ( render.cameraViewMatrix * nodeVar26 );
	nodeVar28 = normalize( nodeVar27.xyz );
	nodeVar29 = nodeVar28;
	nodeVar30 = dot( normalView, nodeVar29 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar32 = ( render.nodeUniform14 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
	nodeVar33 = ( nodeVar32.xyz / vec3<f32>( nodeVar32.w ) );
	nodeVar34 = vec3<f32>( nodeVar33.x, ( 1.0 - nodeVar33.y ), ( nodeVar33.z + render.nodeUniform16 ) );

	if ( ( ( ( ( ( nodeVar34.x >= 0.0 ) && ( nodeVar34.x <= 1.0 ) ) && ( nodeVar34.y >= 0.0 ) ) && ( nodeVar34.y <= 1.0 ) ) && ( nodeVar34.z <= 1.0 ) ) ) {

		nodeVar35 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar36 = ( render.nodeUniform18 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ).x );
		nodeVar37 = ( nodeVar34.xy + ( vogelDiskSample( 0, 5, nodeVar35 ) * vec2<f32>( nodeVar36 ) ) );
		nodeVar38 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar37, nodeVar34.z );
		nodeVar39 = ( nodeVar34.xy + ( vogelDiskSample( 1, 5, nodeVar35 ) * vec2<f32>( nodeVar36 ) ) );
		nodeVar40 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar39, nodeVar34.z );
		nodeVar41 = ( nodeVar34.xy + ( vogelDiskSample( 2, 5, nodeVar35 ) * vec2<f32>( nodeVar36 ) ) );
		nodeVar42 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar41, nodeVar34.z );
		nodeVar43 = ( nodeVar34.xy + ( vogelDiskSample( 3, 5, nodeVar35 ) * vec2<f32>( nodeVar36 ) ) );
		nodeVar44 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar43, nodeVar34.z );
		nodeVar45 = ( nodeVar34.xy + ( vogelDiskSample( 4, 5, nodeVar35 ) * vec2<f32>( nodeVar36 ) ) );
		nodeVar46 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar45, nodeVar34.z );
		nodeVar31 = ( ( ( ( ( nodeVar38 + nodeVar40 ) + nodeVar42 ) + nodeVar44 ) + nodeVar46 ) * 0.2 );

	} else {

		nodeVar31 = 1.0;

	}

	nodeVar47 = mix( 1.0, nodeVar31, render.nodeUniform20 );
	nodeVar48 = ( vec3<f32>( clamp( nodeVar30, 0.0, 1.0 ) ) * ( render.nodeUniform13 * vec3<f32>( nodeVar47 ) ) );
	nodeVar49 = nodeVar48;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar50 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar51 = ( nodeVar49 * nodeVar50 );
	nodeVar52 = ( nodeVar29 + positionViewDirection );
	nodeVar53 = normalize( nodeVar52 );
	nodeVar54 = dot( positionViewDirection, nodeVar53 );
	nodeVar55 = clamp( nodeVar54, 0.0, 1.0 );
	nodeVar56 = exp2( ( ( ( nodeVar55 * -5.55473 ) - 6.98316 ) * nodeVar55 ) );
	nodeVar57 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar56 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar56 ) ) );
	nodeVar58 = ( vec3<f32>( 1.0 ) - nodeVar57 );
	nodeVar59 = nodeVar58;
	nodeVar60 = ( nodeVar51 * nodeVar59 );
	nodeVar61 = ( directDiffuse + nodeVar60 );
	directDiffuse = nodeVar61;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar62 = normalize( ( nodeVar29 + positionViewDirection ) );
	nodeVar63 = clamp( dot( positionViewDirection, nodeVar62 ), 0.0, 1.0 );
	nodeVar64 = exp2( ( ( ( nodeVar63 * -5.55473 ) - 6.98316 ) * nodeVar63 ) );
	nodeVar65 = ( Roughness * Roughness );
	nodeVar66 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar64 ) ) ) + vec3<f32>( ( 1.0 * nodeVar64 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar65, clamp( dot( normalView, nodeVar29 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar65, clamp( dot( normalView, nodeVar62 ), 0.0, 1.0 ) ) ) );
	nodeVar67 = ( nodeVar49 * nodeVar66 );
	nodeVar68 = ( nodeVar67 * multiScatteringCompensation );
	nodeVar69 = ( directSpecular + nodeVar68 );
	directSpecular = nodeVar69;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar70 = ( object.nodeUniform22 - object.nodeUniform23 );
	nodeVar71 = ( object.nodeUniform24 - vec3<f32>( 1.0 ) );
	nodeVar72 = ( nodeVar70 / nodeVar71 );
	nodeVar73 = ( normalWorld * nodeVar72 );
	nodeVar74 = ( nodeVar73 * vec3<f32>( 0.5 ) );
	nodeVar75 = ( v_positionWorld + nodeVar74 );
	nodeVar76 = ( nodeVar75 - object.nodeUniform23 );
	nodeVar77 = ( nodeVar76 / nodeVar70 );
	nodeVar78 = ( clamp( nodeVar77, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar71 );
	nodeVar79 = ( nodeVar78 / object.nodeUniform24 );
	nodeVar80 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform24 );
	nodeVar81 = ( nodeVar79 + nodeVar80 );
	nodeVar82 = ( nodeVar81.z * object.nodeUniform24.z );
	nodeVar83 = ( nodeVar82 + 1.0 );
	nodeVar84 = ( object.nodeUniform24.z + 2.0 );
	nodeVar85 = ( nodeVar84 * 0.0 );
	nodeVar86 = ( nodeVar83 + nodeVar85 );
	nodeVar87 = ( nodeVar84 * 7.0 );
	nodeVar88 = ( nodeVar86 / nodeVar87 );
	nodeVar89 = vec3<f32>( nodeVar81.xy, nodeVar88 );
	nodeVar90 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar89 );
	nodeVar91 = ( nodeVar84 * 1.0 );
	nodeVar92 = ( nodeVar83 + nodeVar91 );
	nodeVar93 = ( nodeVar92 / nodeVar87 );
	nodeVar94 = vec3<f32>( nodeVar81.xy, nodeVar93 );
	nodeVar95 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar94 );
	nodeVar96 = vec3<f32>( nodeVar90.w, nodeVar95.xy );
	nodeVar97 = ( nodeVar84 * 2.0 );
	nodeVar98 = ( nodeVar83 + nodeVar97 );
	nodeVar99 = ( nodeVar98 / nodeVar87 );
	nodeVar100 = vec3<f32>( nodeVar81.xy, nodeVar99 );
	nodeVar101 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar100 );
	nodeVar102 = vec3<f32>( nodeVar95.zw, nodeVar101.x );
	nodeVar103 = ( nodeVar84 * 3.0 );
	nodeVar104 = ( nodeVar83 + nodeVar103 );
	nodeVar105 = ( nodeVar104 / nodeVar87 );
	nodeVar106 = vec3<f32>( nodeVar81.xy, nodeVar105 );
	nodeVar107 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar106 );
	nodeVar108 = ( nodeVar84 * 4.0 );
	nodeVar109 = ( nodeVar83 + nodeVar108 );
	nodeVar110 = ( nodeVar109 / nodeVar87 );
	nodeVar111 = vec3<f32>( nodeVar81.xy, nodeVar110 );
	nodeVar112 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar111 );
	nodeVar113 = vec3<f32>( nodeVar107.w, nodeVar112.xy );
	nodeVar114 = ( nodeVar84 * 5.0 );
	nodeVar115 = ( nodeVar83 + nodeVar114 );
	nodeVar116 = ( nodeVar115 / nodeVar87 );
	nodeVar117 = vec3<f32>( nodeVar81.xy, nodeVar116 );
	nodeVar118 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar117 );
	nodeVar119 = vec3<f32>( nodeVar112.zw, nodeVar118.x );
	nodeVar120 = ( nodeVar84 * 6.0 );
	nodeVar121 = ( nodeVar83 + nodeVar120 );
	nodeVar122 = ( nodeVar121 / nodeVar87 );
	nodeVar123 = vec3<f32>( nodeVar81.xy, nodeVar122 );
	nodeVar124 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar123 );
	nodeVar125 = array< vec3<f32>, 9 >( nodeVar90.xyz, nodeVar96, nodeVar102, nodeVar101.yzw, nodeVar107.xyz, nodeVar113, nodeVar119, nodeVar118.yzw, nodeVar124.xyz );
	nodeVar126 = ( ( ( ( ( ( ( ( ( nodeVar125[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar125[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar125[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar125[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar125[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar125[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar125[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar125[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar125[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar127 = max( nodeVar126, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar128 = ( nodeVar127 * vec3<f32>( object.nodeUniform25 ) );
	nodeVar129 = ( irradiance + nodeVar128 );
	irradiance = nodeVar129;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar130 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform26 );
	nodeVar131 = floor( nodeVar130 );
	nodeVar132 = nodeVar131;
	nodeVar133 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar134 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar133.x, ( - nodeVar133.y ), nodeVar133.z ), 1.0 ) ).xyz );
	nodeVar135 = max( ( 4.0 - nodeVar132 ), 0.0 );
	nodeVar132 = max( nodeVar132, 4.0 );
	nodeVar136 = exp2( nodeVar132 );
	nodeVar137 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar133.x, ( - nodeVar133.y ), nodeVar133.z ), 1.0 ) ).xyz, nodeVar134 ) * vec2<f32>( ( nodeVar136 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar134 > 2.0 ) ) {

		nodeVar137.y = ( nodeVar137.y + nodeVar136 );
		nodeVar134 = ( nodeVar134 - 3.0 );
		

	}

	nodeVar137.x = ( nodeVar137.x + ( nodeVar134 * nodeVar136 ) );
	nodeVar137.x = ( nodeVar137.x + ( nodeVar135 * ( 3.0 * 16.0 ) ) );
	nodeVar137.y = ( nodeVar137.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar136 ) ) );
	nodeVar137.x = ( nodeVar137.x * object.nodeUniform29 );
	nodeVar137.y = ( nodeVar137.y * object.nodeUniform30 );
	nodeVar138 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar137, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar139 = nodeVar138.xyz;
	nodeVar140 = fract( nodeVar130 );

	if ( ( nodeVar140 != 0.0 ) ) {

		nodeVar141 = ( nodeVar131 + 1.0 );
		nodeVar142 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar133.x, ( - nodeVar133.y ), nodeVar133.z ), 1.0 ) ).xyz );
		nodeVar143 = max( ( 4.0 - nodeVar141 ), 0.0 );
		nodeVar141 = max( nodeVar141, 4.0 );
		nodeVar144 = exp2( nodeVar141 );
		nodeVar145 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar133.x, ( - nodeVar133.y ), nodeVar133.z ), 1.0 ) ).xyz, nodeVar142 ) * vec2<f32>( ( nodeVar144 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar142 > 2.0 ) ) {

			nodeVar145.y = ( nodeVar145.y + nodeVar144 );
			nodeVar142 = ( nodeVar142 - 3.0 );
			

		}

		nodeVar145.x = ( nodeVar145.x + ( nodeVar142 * nodeVar144 ) );
		nodeVar145.x = ( nodeVar145.x + ( nodeVar143 * ( 3.0 * 16.0 ) ) );
		nodeVar145.y = ( nodeVar145.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar144 ) ) );
		nodeVar145.x = ( nodeVar145.x * object.nodeUniform29 );
		nodeVar145.y = ( nodeVar145.y * object.nodeUniform30 );
		nodeVar146 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar145, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar147 = nodeVar146.xyz;
		nodeVar139 = mix( nodeVar139, nodeVar147, nodeVar140 );
		

	}

	nodeVar148 = ( radiance + ( nodeVar139 * vec3<f32>( object.nodeUniform32 ) ) );
	radiance = nodeVar148;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar149 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform26 );
	nodeVar150 = floor( nodeVar149 );
	nodeVar151 = nodeVar150;
	nodeVar152 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar153 = max( ( 4.0 - nodeVar151 ), 0.0 );
	nodeVar151 = max( nodeVar151, 4.0 );
	nodeVar154 = exp2( nodeVar151 );
	nodeVar155 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar152 ) * vec2<f32>( ( nodeVar154 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

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
	nodeVar158 = fract( nodeVar149 );

	if ( ( nodeVar158 != 0.0 ) ) {

		nodeVar159 = ( nodeVar150 + 1.0 );
		nodeVar160 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar161 = max( ( 4.0 - nodeVar159 ), 0.0 );
		nodeVar159 = max( nodeVar159, 4.0 );
		nodeVar162 = exp2( nodeVar159 );
		nodeVar163 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar160 ) * vec2<f32>( ( nodeVar162 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

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

	nodeVar166 = ( iblIrradiance + ( ( nodeVar157 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform32 ) ) );
	iblIrradiance = nodeVar166;
	nodeVar167 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar168 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar169 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar170 = ( SpecularF90 * dfg.y );
	nodeVar171 = ( nodeVar169 + vec3<f32>( nodeVar170 ) );
	nodeVar172 = ( nodeVar167 + nodeVar171 );
	nodeVar167 = nodeVar172;
	nodeVar173 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar174 = nodeVar173;
	nodeVar175 = ( nodeVar174 * vec3<f32>( 0.047619 ) );
	nodeVar176 = ( SpecularColor + nodeVar175 );
	nodeVar177 = ( nodeVar171 * nodeVar176 );
	nodeVar178 = ( dfg.x + dfg.y );
	nodeVar179 = ( 1.0 - nodeVar178 );
	nodeVar180 = nodeVar179;
	nodeVar181 = ( vec3<f32>( nodeVar180 ) * nodeVar176 );
	nodeVar182 = ( vec3<f32>( 1.0 ) - nodeVar181 );
	nodeVar183 = nodeVar182;
	nodeVar184 = ( nodeVar177 / nodeVar183 );
	nodeVar185 = ( nodeVar184 * vec3<f32>( nodeVar180 ) );
	nodeVar186 = ( nodeVar168 + nodeVar185 );
	nodeVar168 = nodeVar186;
	nodeVar187 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar188 = ( irradiance * nodeVar187 );
	nodeVar189 = ( nodeVar167 + nodeVar168 );
	nodeVar190 = ( vec3<f32>( 1.0 ) - nodeVar189 );
	nodeVar191 = nodeVar190;
	nodeVar192 = ( nodeVar188 * nodeVar191 );
	nodeVar193 = nodeVar192;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar194 = ( indirectDiffuse + nodeVar193 );
	indirectDiffuse = nodeVar194;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar195 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar196 = ( SpecularF90 * dfg.y );
	nodeVar197 = ( nodeVar195 + vec3<f32>( nodeVar196 ) );
	nodeVar198 = ( singleScatteringDielectric + nodeVar197 );
	singleScatteringDielectric = nodeVar198;
	nodeVar199 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar200 = nodeVar199;
	nodeVar201 = ( nodeVar200 * vec3<f32>( 0.047619 ) );
	nodeVar202 = ( SpecularColor + nodeVar201 );
	nodeVar203 = ( nodeVar197 * nodeVar202 );
	nodeVar204 = ( dfg.x + dfg.y );
	nodeVar205 = ( 1.0 - nodeVar204 );
	nodeVar206 = nodeVar205;
	nodeVar207 = ( vec3<f32>( nodeVar206 ) * nodeVar202 );
	nodeVar208 = ( vec3<f32>( 1.0 ) - nodeVar207 );
	nodeVar209 = nodeVar208;
	nodeVar210 = ( nodeVar203 / nodeVar209 );
	nodeVar211 = ( nodeVar210 * vec3<f32>( nodeVar206 ) );
	nodeVar212 = ( multiScatteringDielectric + nodeVar211 );
	multiScatteringDielectric = nodeVar212;
	nodeVar213 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar214 = ( SpecularF90 * dfg.y );
	nodeVar215 = ( nodeVar213 + vec3<f32>( nodeVar214 ) );
	nodeVar216 = ( singleScatteringMetallic + nodeVar215 );
	singleScatteringMetallic = nodeVar216;
	nodeVar217 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar218 = nodeVar217;
	nodeVar219 = ( nodeVar218 * vec3<f32>( 0.047619 ) );
	nodeVar220 = ( DiffuseColor.xyz + nodeVar219 );
	nodeVar221 = ( nodeVar215 * nodeVar220 );
	nodeVar222 = ( dfg.x + dfg.y );
	nodeVar223 = ( 1.0 - nodeVar222 );
	nodeVar224 = nodeVar223;
	nodeVar225 = ( vec3<f32>( nodeVar224 ) * nodeVar220 );
	nodeVar226 = ( vec3<f32>( 1.0 ) - nodeVar225 );
	nodeVar227 = nodeVar226;
	nodeVar228 = ( nodeVar221 / nodeVar227 );
	nodeVar229 = ( nodeVar228 * vec3<f32>( nodeVar224 ) );
	nodeVar230 = ( multiScatteringMetallic + nodeVar229 );
	multiScatteringMetallic = nodeVar230;
	nodeVar231 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar232 = ( radiance * nodeVar231 );
	nodeVar233 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar234 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar235 = ( nodeVar233 * nodeVar234 );
	nodeVar236 = ( nodeVar232 + nodeVar235 );
	nodeVar237 = nodeVar236;
	nodeVar238 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar239 = ( vec3<f32>( 1.0 ) - nodeVar238 );
	nodeVar240 = nodeVar239;
	nodeVar241 = ( DiffuseContribution * nodeVar240 );
	nodeVar242 = ( nodeVar241 * nodeVar234 );
	nodeVar243 = nodeVar242;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar244 = ( indirectSpecular + nodeVar237 );
	indirectSpecular = nodeVar244;
	nodeVar245 = ( indirectDiffuse + nodeVar243 );
	indirectDiffuse = nodeVar245;
	ambientOcclusion = 1.0;
	nodeVar246 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar246;
	nodeVar247 = dot( normalView, positionViewDirection );
	nodeVar248 = ( clamp( nodeVar247, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar249 = ( Roughness * -16.0 );
	nodeVar250 = ( 1.0 - nodeVar249 );
	nodeVar251 = nodeVar250;
	nodeVar252 = ( - nodeVar251 );
	nodeVar253 = exp2( nodeVar252 );
	nodeVar254 = pow( nodeVar248, nodeVar253 );
	nodeVar255 = ( 1.0 - nodeVar254 );
	nodeVar256 = nodeVar255;
	nodeVar257 = ( ambientOcclusion - nodeVar256 );
	nodeVar258 = ( indirectSpecular * vec3<f32>( clamp( nodeVar257, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar258;
	nodeVar259 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar259;
	nodeVar260 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar260;
	nodeVar261 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar261;
	nodeVar262 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar262;

	// result

	output.color = nodeVar262;

	return output;

}
