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
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform6 : mat3x3<f32>,
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
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_nodeVar11 : vec3<f32>;
var<private> NORMAL_nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar31 : vec4<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
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
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec4<f32>;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec4<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec4<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec4<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec4<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec4<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec4<f32>;
var<private> nodeVar124 : array< vec3<f32>, 9 >;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : vec2<f32>;
var<private> nodeVar137 : vec4<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : vec2<f32>;
var<private> nodeVar145 : vec4<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec2<f32>;
var<private> nodeVar155 : vec4<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : vec2<f32>;
var<private> nodeVar163 : vec4<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
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
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : f32;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : f32;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : vec3<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : f32;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : f32;
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
var<private> nodeVar239 : vec3<f32>;
var<private> nodeVar240 : vec3<f32>;
var<private> nodeVar241 : vec3<f32>;
var<private> nodeVar242 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar243 : vec3<f32>;
var<private> nodeVar244 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar245 : vec3<f32>;
var<private> nodeVar246 : f32;
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
var<private> nodeVar257 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar258 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar259 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar260 : vec3<f32>;
var<private> nodeVar261 : vec4<f32>;

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
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = fract( ( sin( ( ( floor( ( v_positionWorld.x / 1.5 ) ) * 127.1 ) + ( floor( ( v_positionWorld.z / 1.5 ) ) * 311.7 ) ) ) * 43758.5453 ) );
	nodeVar1 = 0.0;
	nodeVar2 = smoothstep( 200.0, 18.0, distance( v_positionWorld, render.cameraPosition ) );

	if ( ( nodeVar2 > 0.0 ) ) {

		nodeVar1 = ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 14.0 ) ) ) * 1.0 ) + 0.0 ) * 0.07 );
		

	}

	nodeVar3 = ( v_positionWorld.x / 1.5 );
	nodeVar4 = max( fwidth( nodeVar3 ), 0.0001 );
	nodeVar5 = ( v_positionWorld.z / 1.5 );
	nodeVar6 = max( fwidth( nodeVar5 ), 0.0001 );
	nodeVar7 = ( max( smoothstep( ( 0.03 + nodeVar4 ), ( 0.03 - nodeVar4 ), ( 0.5 - abs( ( fract( nodeVar3 ) - 0.5 ) ) ) ), smoothstep( ( 0.03 + nodeVar6 ), ( 0.03 - nodeVar6 ), ( 0.5 - abs( ( fract( nodeVar5 ) - 0.5 ) ) ) ) ) * nodeVar2 );
	DiffuseColor = vec4<f32>( ( ( ( mix( vec3<f32>( 0.158960835050774, 0.158960835050774, 0.13843161502267545 ), vec3<f32>( 0.26225065751888765, 0.26225065751888765, 0.22322795730611386 ), ( ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 0.5 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) ) * vec3<f32>( ( ( ( nodeVar0 - 0.5 ) * 0.16 ) + 1.0 ) ) ) + vec3<f32>( ( nodeVar1 * nodeVar2 ) ) ) * vec3<f32>( ( 1.0 - ( nodeVar7 * 0.45 ) ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar8 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( 0.92 - ( nodeVar0 * 0.05 ) ), 0.0525 ) + max( max( nodeVar8.x, nodeVar8.y ), nodeVar8.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	nodeVar9 = dpdx( v_positionView );
	nodeVar10 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_nodeVar11 = cross( nodeVar10, NORMAL_normalView );
	NORMAL_nodeVar12 = dot( nodeVar9, NORMAL_nodeVar11 );
	nodeVar13 = 0.0;

	if ( ( nodeVar2 > 0.0 ) ) {

		nodeVar13 = ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 3.0 ) ) ) * 1.0 ) + 0.0 ) * 0.003 );
		

	}

	nodeVar14 = ( ( nodeVar13 - ( nodeVar7 * 0.012 ) ) * nodeVar2 );
	nodeVar15 = ( vec3<f32>( sign( NORMAL_nodeVar12 ) ) * ( ( vec3<f32>( dpdx( nodeVar14 ) ) * NORMAL_nodeVar11 ) + ( vec3<f32>( - dpdy( nodeVar14 ) ) * cross( NORMAL_normalView, nodeVar9 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeVar12 ) ) * NORMAL_normalView ) - nodeVar15 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar16 = dot( normalView, positionViewDirection );
	nodeVar17 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( Roughness, clamp( nodeVar16, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar17;
	nodeVar18 = ( dfg.x + dfg.y );
	nodeVar19 = ( 1.0 / nodeVar18 );
	nodeVar20 = nodeVar19;
	nodeVar21 = ( nodeVar20 - 1.0 );
	nodeVar22 = ( SpecularColorBlended * vec3<f32>( nodeVar21 ) );
	nodeVar23 = ( nodeVar22 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar23;
	nodeVar24 = ( render.nodeUniform11 - render.nodeUniform12 );
	nodeVar25 = vec4<f32>( nodeVar24, 0.0 );
	nodeVar26 = ( render.cameraViewMatrix * nodeVar25 );
	nodeVar27 = normalize( nodeVar26.xyz );
	nodeVar28 = nodeVar27;
	nodeVar29 = dot( normalView, nodeVar28 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar31 = ( render.nodeUniform14 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
	nodeVar32 = ( nodeVar31.xyz / vec3<f32>( nodeVar31.w ) );
	nodeVar33 = vec3<f32>( nodeVar32.x, ( 1.0 - nodeVar32.y ), ( nodeVar32.z + render.nodeUniform16 ) );

	if ( ( ( ( ( ( nodeVar33.x >= 0.0 ) && ( nodeVar33.x <= 1.0 ) ) && ( nodeVar33.y >= 0.0 ) ) && ( nodeVar33.y <= 1.0 ) ) && ( nodeVar33.z <= 1.0 ) ) ) {

		nodeVar34 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar35 = ( render.nodeUniform18 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ).x );
		nodeVar36 = ( nodeVar33.xy + ( vogelDiskSample( 0, 5, nodeVar34 ) * vec2<f32>( nodeVar35 ) ) );
		nodeVar37 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar36, nodeVar33.z );
		nodeVar38 = ( nodeVar33.xy + ( vogelDiskSample( 1, 5, nodeVar34 ) * vec2<f32>( nodeVar35 ) ) );
		nodeVar39 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar38, nodeVar33.z );
		nodeVar40 = ( nodeVar33.xy + ( vogelDiskSample( 2, 5, nodeVar34 ) * vec2<f32>( nodeVar35 ) ) );
		nodeVar41 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar40, nodeVar33.z );
		nodeVar42 = ( nodeVar33.xy + ( vogelDiskSample( 3, 5, nodeVar34 ) * vec2<f32>( nodeVar35 ) ) );
		nodeVar43 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar42, nodeVar33.z );
		nodeVar44 = ( nodeVar33.xy + ( vogelDiskSample( 4, 5, nodeVar34 ) * vec2<f32>( nodeVar35 ) ) );
		nodeVar45 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar44, nodeVar33.z );
		nodeVar30 = ( ( ( ( ( nodeVar37 + nodeVar39 ) + nodeVar41 ) + nodeVar43 ) + nodeVar45 ) * 0.2 );

	} else {

		nodeVar30 = 1.0;

	}

	nodeVar46 = mix( 1.0, nodeVar30, render.nodeUniform20 );
	nodeVar47 = ( vec3<f32>( clamp( nodeVar29, 0.0, 1.0 ) ) * ( render.nodeUniform13 * vec3<f32>( nodeVar46 ) ) );
	nodeVar48 = nodeVar47;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar49 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar50 = ( nodeVar48 * nodeVar49 );
	nodeVar51 = ( nodeVar28 + positionViewDirection );
	nodeVar52 = normalize( nodeVar51 );
	nodeVar53 = dot( positionViewDirection, nodeVar52 );
	nodeVar54 = clamp( nodeVar53, 0.0, 1.0 );
	nodeVar55 = exp2( ( ( ( nodeVar54 * -5.55473 ) - 6.98316 ) * nodeVar54 ) );
	nodeVar56 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar55 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar55 ) ) );
	nodeVar57 = ( vec3<f32>( 1.0 ) - nodeVar56 );
	nodeVar58 = nodeVar57;
	nodeVar59 = ( nodeVar50 * nodeVar58 );
	nodeVar60 = ( directDiffuse + nodeVar59 );
	directDiffuse = nodeVar60;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar61 = normalize( ( nodeVar28 + positionViewDirection ) );
	nodeVar62 = clamp( dot( positionViewDirection, nodeVar61 ), 0.0, 1.0 );
	nodeVar63 = exp2( ( ( ( nodeVar62 * -5.55473 ) - 6.98316 ) * nodeVar62 ) );
	nodeVar64 = ( Roughness * Roughness );
	nodeVar65 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar63 ) ) ) + vec3<f32>( ( 1.0 * nodeVar63 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar64, clamp( dot( normalView, nodeVar28 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar64, clamp( dot( normalView, nodeVar61 ), 0.0, 1.0 ) ) ) );
	nodeVar66 = ( nodeVar48 * nodeVar65 );
	nodeVar67 = ( nodeVar66 * multiScatteringCompensation );
	nodeVar68 = ( directSpecular + nodeVar67 );
	directSpecular = nodeVar68;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = ( object.nodeUniform22 - object.nodeUniform23 );
	nodeVar70 = ( object.nodeUniform24 - vec3<f32>( 1.0 ) );
	nodeVar71 = ( nodeVar69 / nodeVar70 );
	nodeVar72 = ( normalWorld * nodeVar71 );
	nodeVar73 = ( nodeVar72 * vec3<f32>( 0.5 ) );
	nodeVar74 = ( v_positionWorld + nodeVar73 );
	nodeVar75 = ( nodeVar74 - object.nodeUniform23 );
	nodeVar76 = ( nodeVar75 / nodeVar69 );
	nodeVar77 = ( clamp( nodeVar76, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar70 );
	nodeVar78 = ( nodeVar77 / object.nodeUniform24 );
	nodeVar79 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform24 );
	nodeVar80 = ( nodeVar78 + nodeVar79 );
	nodeVar81 = ( nodeVar80.z * object.nodeUniform24.z );
	nodeVar82 = ( nodeVar81 + 1.0 );
	nodeVar83 = ( object.nodeUniform24.z + 2.0 );
	nodeVar84 = ( nodeVar83 * 0.0 );
	nodeVar85 = ( nodeVar82 + nodeVar84 );
	nodeVar86 = ( nodeVar83 * 7.0 );
	nodeVar87 = ( nodeVar85 / nodeVar86 );
	nodeVar88 = vec3<f32>( nodeVar80.xy, nodeVar87 );
	nodeVar89 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar88 );
	nodeVar90 = ( nodeVar83 * 1.0 );
	nodeVar91 = ( nodeVar82 + nodeVar90 );
	nodeVar92 = ( nodeVar91 / nodeVar86 );
	nodeVar93 = vec3<f32>( nodeVar80.xy, nodeVar92 );
	nodeVar94 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar93 );
	nodeVar95 = vec3<f32>( nodeVar89.w, nodeVar94.xy );
	nodeVar96 = ( nodeVar83 * 2.0 );
	nodeVar97 = ( nodeVar82 + nodeVar96 );
	nodeVar98 = ( nodeVar97 / nodeVar86 );
	nodeVar99 = vec3<f32>( nodeVar80.xy, nodeVar98 );
	nodeVar100 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar99 );
	nodeVar101 = vec3<f32>( nodeVar94.zw, nodeVar100.x );
	nodeVar102 = ( nodeVar83 * 3.0 );
	nodeVar103 = ( nodeVar82 + nodeVar102 );
	nodeVar104 = ( nodeVar103 / nodeVar86 );
	nodeVar105 = vec3<f32>( nodeVar80.xy, nodeVar104 );
	nodeVar106 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar105 );
	nodeVar107 = ( nodeVar83 * 4.0 );
	nodeVar108 = ( nodeVar82 + nodeVar107 );
	nodeVar109 = ( nodeVar108 / nodeVar86 );
	nodeVar110 = vec3<f32>( nodeVar80.xy, nodeVar109 );
	nodeVar111 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar110 );
	nodeVar112 = vec3<f32>( nodeVar106.w, nodeVar111.xy );
	nodeVar113 = ( nodeVar83 * 5.0 );
	nodeVar114 = ( nodeVar82 + nodeVar113 );
	nodeVar115 = ( nodeVar114 / nodeVar86 );
	nodeVar116 = vec3<f32>( nodeVar80.xy, nodeVar115 );
	nodeVar117 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar116 );
	nodeVar118 = vec3<f32>( nodeVar111.zw, nodeVar117.x );
	nodeVar119 = ( nodeVar83 * 6.0 );
	nodeVar120 = ( nodeVar82 + nodeVar119 );
	nodeVar121 = ( nodeVar120 / nodeVar86 );
	nodeVar122 = vec3<f32>( nodeVar80.xy, nodeVar121 );
	nodeVar123 = textureSample( nodeUniform21, nodeUniform21_sampler, nodeVar122 );
	nodeVar124 = array< vec3<f32>, 9 >( nodeVar89.xyz, nodeVar95, nodeVar101, nodeVar100.yzw, nodeVar106.xyz, nodeVar112, nodeVar118, nodeVar117.yzw, nodeVar123.xyz );
	nodeVar125 = ( ( ( ( ( ( ( ( ( nodeVar124[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar124[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar124[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar124[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar124[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar124[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar124[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar124[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar124[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar126 = max( nodeVar125, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar127 = ( nodeVar126 * vec3<f32>( object.nodeUniform25 ) );
	nodeVar128 = ( irradiance + nodeVar127 );
	irradiance = nodeVar128;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar129 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform26 );
	nodeVar130 = floor( nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar133 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar132.x, ( - nodeVar132.y ), nodeVar132.z ), 1.0 ) ).xyz );
	nodeVar134 = max( ( 4.0 - nodeVar131 ), 0.0 );
	nodeVar131 = max( nodeVar131, 4.0 );
	nodeVar135 = exp2( nodeVar131 );
	nodeVar136 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar132.x, ( - nodeVar132.y ), nodeVar132.z ), 1.0 ) ).xyz, nodeVar133 ) * vec2<f32>( ( nodeVar135 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar133 > 2.0 ) ) {

		nodeVar136.y = ( nodeVar136.y + nodeVar135 );
		nodeVar133 = ( nodeVar133 - 3.0 );
		

	}

	nodeVar136.x = ( nodeVar136.x + ( nodeVar133 * nodeVar135 ) );
	nodeVar136.x = ( nodeVar136.x + ( nodeVar134 * ( 3.0 * 16.0 ) ) );
	nodeVar136.y = ( nodeVar136.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar135 ) ) );
	nodeVar136.x = ( nodeVar136.x * object.nodeUniform29 );
	nodeVar136.y = ( nodeVar136.y * object.nodeUniform30 );
	nodeVar137 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar136, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar138 = nodeVar137.xyz;
	nodeVar139 = fract( nodeVar129 );

	if ( ( nodeVar139 != 0.0 ) ) {

		nodeVar140 = ( nodeVar130 + 1.0 );
		nodeVar141 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar132.x, ( - nodeVar132.y ), nodeVar132.z ), 1.0 ) ).xyz );
		nodeVar142 = max( ( 4.0 - nodeVar140 ), 0.0 );
		nodeVar140 = max( nodeVar140, 4.0 );
		nodeVar143 = exp2( nodeVar140 );
		nodeVar144 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( nodeVar132.x, ( - nodeVar132.y ), nodeVar132.z ), 1.0 ) ).xyz, nodeVar141 ) * vec2<f32>( ( nodeVar143 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar141 > 2.0 ) ) {

			nodeVar144.y = ( nodeVar144.y + nodeVar143 );
			nodeVar141 = ( nodeVar141 - 3.0 );
			

		}

		nodeVar144.x = ( nodeVar144.x + ( nodeVar141 * nodeVar143 ) );
		nodeVar144.x = ( nodeVar144.x + ( nodeVar142 * ( 3.0 * 16.0 ) ) );
		nodeVar144.y = ( nodeVar144.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar143 ) ) );
		nodeVar144.x = ( nodeVar144.x * object.nodeUniform29 );
		nodeVar144.y = ( nodeVar144.y * object.nodeUniform30 );
		nodeVar145 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar144, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar146 = nodeVar145.xyz;
		nodeVar138 = mix( nodeVar138, nodeVar146, nodeVar139 );
		

	}

	nodeVar147 = ( radiance + ( nodeVar138 * vec3<f32>( object.nodeUniform32 ) ) );
	radiance = nodeVar147;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar148 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform26 );
	nodeVar149 = floor( nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar152 = max( ( 4.0 - nodeVar150 ), 0.0 );
	nodeVar150 = max( nodeVar150, 4.0 );
	nodeVar153 = exp2( nodeVar150 );
	nodeVar154 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar151 ) * vec2<f32>( ( nodeVar153 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar151 > 2.0 ) ) {

		nodeVar154.y = ( nodeVar154.y + nodeVar153 );
		nodeVar151 = ( nodeVar151 - 3.0 );
		

	}

	nodeVar154.x = ( nodeVar154.x + ( nodeVar151 * nodeVar153 ) );
	nodeVar154.x = ( nodeVar154.x + ( nodeVar152 * ( 3.0 * 16.0 ) ) );
	nodeVar154.y = ( nodeVar154.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar153 ) ) );
	nodeVar154.x = ( nodeVar154.x * object.nodeUniform29 );
	nodeVar154.y = ( nodeVar154.y * object.nodeUniform30 );
	nodeVar155 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar154, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar156 = nodeVar155.xyz;
	nodeVar157 = fract( nodeVar148 );

	if ( ( nodeVar157 != 0.0 ) ) {

		nodeVar158 = ( nodeVar149 + 1.0 );
		nodeVar159 = getFace( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar160 = max( ( 4.0 - nodeVar158 ), 0.0 );
		nodeVar158 = max( nodeVar158, 4.0 );
		nodeVar161 = exp2( nodeVar158 );
		nodeVar162 = ( ( getUV( ( object.nodeUniform27 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar159 ) * vec2<f32>( ( nodeVar161 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar159 > 2.0 ) ) {

			nodeVar162.y = ( nodeVar162.y + nodeVar161 );
			nodeVar159 = ( nodeVar159 - 3.0 );
			

		}

		nodeVar162.x = ( nodeVar162.x + ( nodeVar159 * nodeVar161 ) );
		nodeVar162.x = ( nodeVar162.x + ( nodeVar160 * ( 3.0 * 16.0 ) ) );
		nodeVar162.y = ( nodeVar162.y + ( 4.0 * ( exp2( object.nodeUniform26 ) - nodeVar161 ) ) );
		nodeVar162.x = ( nodeVar162.x * object.nodeUniform29 );
		nodeVar162.y = ( nodeVar162.y * object.nodeUniform30 );
		nodeVar163 = textureSampleGrad( nodeUniform31, nodeUniform31_sampler, nodeVar162, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar164 = nodeVar163.xyz;
		nodeVar156 = mix( nodeVar156, nodeVar164, nodeVar157 );
		

	}

	nodeVar165 = ( iblIrradiance + ( ( nodeVar156 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform32 ) ) );
	iblIrradiance = nodeVar165;
	nodeVar166 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar167 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar168 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar169 = ( SpecularF90 * dfg.y );
	nodeVar170 = ( nodeVar168 + vec3<f32>( nodeVar169 ) );
	nodeVar171 = ( nodeVar166 + nodeVar170 );
	nodeVar166 = nodeVar171;
	nodeVar172 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar173 = nodeVar172;
	nodeVar174 = ( nodeVar173 * vec3<f32>( 0.047619 ) );
	nodeVar175 = ( SpecularColor + nodeVar174 );
	nodeVar176 = ( nodeVar170 * nodeVar175 );
	nodeVar177 = ( dfg.x + dfg.y );
	nodeVar178 = ( 1.0 - nodeVar177 );
	nodeVar179 = nodeVar178;
	nodeVar180 = ( vec3<f32>( nodeVar179 ) * nodeVar175 );
	nodeVar181 = ( vec3<f32>( 1.0 ) - nodeVar180 );
	nodeVar182 = nodeVar181;
	nodeVar183 = ( nodeVar176 / nodeVar182 );
	nodeVar184 = ( nodeVar183 * vec3<f32>( nodeVar179 ) );
	nodeVar185 = ( nodeVar167 + nodeVar184 );
	nodeVar167 = nodeVar185;
	nodeVar186 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar187 = ( irradiance * nodeVar186 );
	nodeVar188 = ( nodeVar166 + nodeVar167 );
	nodeVar189 = ( vec3<f32>( 1.0 ) - nodeVar188 );
	nodeVar190 = nodeVar189;
	nodeVar191 = ( nodeVar187 * nodeVar190 );
	nodeVar192 = nodeVar191;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar193 = ( indirectDiffuse + nodeVar192 );
	indirectDiffuse = nodeVar193;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar194 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar195 = ( SpecularF90 * dfg.y );
	nodeVar196 = ( nodeVar194 + vec3<f32>( nodeVar195 ) );
	nodeVar197 = ( singleScatteringDielectric + nodeVar196 );
	singleScatteringDielectric = nodeVar197;
	nodeVar198 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar199 = nodeVar198;
	nodeVar200 = ( nodeVar199 * vec3<f32>( 0.047619 ) );
	nodeVar201 = ( SpecularColor + nodeVar200 );
	nodeVar202 = ( nodeVar196 * nodeVar201 );
	nodeVar203 = ( dfg.x + dfg.y );
	nodeVar204 = ( 1.0 - nodeVar203 );
	nodeVar205 = nodeVar204;
	nodeVar206 = ( vec3<f32>( nodeVar205 ) * nodeVar201 );
	nodeVar207 = ( vec3<f32>( 1.0 ) - nodeVar206 );
	nodeVar208 = nodeVar207;
	nodeVar209 = ( nodeVar202 / nodeVar208 );
	nodeVar210 = ( nodeVar209 * vec3<f32>( nodeVar205 ) );
	nodeVar211 = ( multiScatteringDielectric + nodeVar210 );
	multiScatteringDielectric = nodeVar211;
	nodeVar212 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar213 = ( SpecularF90 * dfg.y );
	nodeVar214 = ( nodeVar212 + vec3<f32>( nodeVar213 ) );
	nodeVar215 = ( singleScatteringMetallic + nodeVar214 );
	singleScatteringMetallic = nodeVar215;
	nodeVar216 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar217 = nodeVar216;
	nodeVar218 = ( nodeVar217 * vec3<f32>( 0.047619 ) );
	nodeVar219 = ( DiffuseColor.xyz + nodeVar218 );
	nodeVar220 = ( nodeVar214 * nodeVar219 );
	nodeVar221 = ( dfg.x + dfg.y );
	nodeVar222 = ( 1.0 - nodeVar221 );
	nodeVar223 = nodeVar222;
	nodeVar224 = ( vec3<f32>( nodeVar223 ) * nodeVar219 );
	nodeVar225 = ( vec3<f32>( 1.0 ) - nodeVar224 );
	nodeVar226 = nodeVar225;
	nodeVar227 = ( nodeVar220 / nodeVar226 );
	nodeVar228 = ( nodeVar227 * vec3<f32>( nodeVar223 ) );
	nodeVar229 = ( multiScatteringMetallic + nodeVar228 );
	multiScatteringMetallic = nodeVar229;
	nodeVar230 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar231 = ( radiance * nodeVar230 );
	nodeVar232 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar233 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar234 = ( nodeVar232 * nodeVar233 );
	nodeVar235 = ( nodeVar231 + nodeVar234 );
	nodeVar236 = nodeVar235;
	nodeVar237 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar238 = ( vec3<f32>( 1.0 ) - nodeVar237 );
	nodeVar239 = nodeVar238;
	nodeVar240 = ( DiffuseContribution * nodeVar239 );
	nodeVar241 = ( nodeVar240 * nodeVar233 );
	nodeVar242 = nodeVar241;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar243 = ( indirectSpecular + nodeVar236 );
	indirectSpecular = nodeVar243;
	nodeVar244 = ( indirectDiffuse + nodeVar242 );
	indirectDiffuse = nodeVar244;
	ambientOcclusion = 1.0;
	nodeVar245 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar245;
	nodeVar246 = dot( normalView, positionViewDirection );
	nodeVar247 = ( clamp( nodeVar246, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar248 = ( Roughness * -16.0 );
	nodeVar249 = ( 1.0 - nodeVar248 );
	nodeVar250 = nodeVar249;
	nodeVar251 = ( - nodeVar250 );
	nodeVar252 = exp2( nodeVar251 );
	nodeVar253 = pow( nodeVar247, nodeVar252 );
	nodeVar254 = ( 1.0 - nodeVar253 );
	nodeVar255 = nodeVar254;
	nodeVar256 = ( ambientOcclusion - nodeVar255 );
	nodeVar257 = ( indirectSpecular * vec3<f32>( clamp( nodeVar256, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar257;
	nodeVar258 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar258;
	nodeVar259 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar259;
	nodeVar260 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar260;
	nodeVar261 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar261;

	// result

	output.color = nodeVar261;

	return output;

}
