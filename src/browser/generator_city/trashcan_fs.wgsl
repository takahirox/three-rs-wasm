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
@binding( 5 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform19 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform29_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform29 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : f32,
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
	nodeUniform11 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform18 : f32,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : bool;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> Metalness : f32;
var<private> nodeVar5 : f32;
var<private> Roughness : f32;
var<private> nodeVar6 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> normalWorld : vec3<f32>;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec4<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec4<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec4<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec4<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec4<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec4<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec4<f32>;
var<private> nodeVar116 : array< vec3<f32>, 9 >;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : vec2<f32>;
var<private> nodeVar129 : vec4<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : vec2<f32>;
var<private> nodeVar137 : vec4<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : vec2<f32>;
var<private> nodeVar147 : vec4<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec2<f32>;
var<private> nodeVar155 : vec4<f32>;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : f32;
var<private> nodeVar214 : f32;
var<private> nodeVar215 : f32;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : vec3<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : vec3<f32>;
var<private> nodeVar222 : vec3<f32>;
var<private> nodeVar223 : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar235 : vec3<f32>;
var<private> nodeVar236 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar237 : vec3<f32>;
var<private> nodeVar238 : f32;
var<private> nodeVar239 : f32;
var<private> nodeVar240 : f32;
var<private> nodeVar241 : f32;
var<private> nodeVar242 : f32;
var<private> nodeVar243 : f32;
var<private> nodeVar244 : f32;
var<private> nodeVar245 : f32;
var<private> nodeVar246 : f32;
var<private> nodeVar247 : f32;
var<private> nodeVar248 : f32;
var<private> nodeVar249 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar250 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar251 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar252 : vec3<f32>;
var<private> nodeVar253 : vec4<f32>;

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
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32>,
	@location( 5 ) nodeVarying9 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = ( nodeVarying3 == 2.0 );

	if ( nodeVar1 ) {

		nodeVar0 = mix( vec3<f32>( 0.01764195448412081, 0.015996293361446288, 0.012286488353353374 ), vec3<f32>( 0.3915724777393922, 0.34191442489801843, 0.24620132669705552 ), ( step( 0.82, ( ( ( mx_fractal_noise_float( ( nodeVarying8 * vec3<f32>( 34.0 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) ) * smoothstep( 0.6, 0.75, nodeVarying8.y ) ) );

	} else {


		if ( ( nodeVarying3 == 1.0 ) ) {

			nodeVar2 = ( ( vec3<f32>( 0.040915196900556984, 0.08865558627723595, 0.04231141061442144 ) * vec3<f32>( ( 1.0 - ( smoothstep( 0.45, 0.05, nodeVarying8.y ) * 0.5 ) ) ) ) * vec3<f32>( 1.15 ) );

		} else {

			nodeVar3 = ( nodeVarying9.x * 26.0 );
			nodeVar4 = ( nodeVarying9.y * 9.0 );
			nodeVar2 = mix( vec3<f32>( 0.005181516700061659, 0.006048833020386069, 0.005181516700061659 ), ( vec3<f32>( 0.040915196900556984, 0.08865558627723595, 0.04231141061442144 ) * vec3<f32>( ( 1.0 - ( smoothstep( 0.45, 0.05, nodeVarying8.y ) * 0.5 ) ) ) ), smoothstep( 0.3, 0.2, min( abs( ( fract( ( nodeVar3 + nodeVar4 ) ) - 0.5 ) ), abs( ( fract( ( nodeVar3 - nodeVar4 ) ) - 0.5 ) ) ) ) );

		}

		nodeVar0 = nodeVar2;

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;

	if ( nodeVar1 ) {

		nodeVar5 = 0.0;

	} else {

		nodeVar5 = 0.5;

	}

	Metalness = nodeVar5;

	if ( nodeVar1 ) {

		nodeVar6 = 0.9;

	} else {

		nodeVar6 = 0.55;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar7 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar6, 0.0525 ) + max( max( nodeVar7.x, nodeVar7.y ), nodeVar7.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeVar5 ) ) );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar8 = dot( normalView, positionViewDirection );
	nodeVar9 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( nodeVar8, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar9;
	nodeVar10 = ( dfg.x + dfg.y );
	nodeVar11 = ( 1.0 / nodeVar10 );
	nodeVar12 = nodeVar11;
	nodeVar13 = ( nodeVar12 - 1.0 );
	nodeVar14 = ( SpecularColorBlended * vec3<f32>( nodeVar13 ) );
	nodeVar15 = ( nodeVar14 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar15;
	nodeVar16 = ( render.nodeUniform9 - render.nodeUniform10 );
	nodeVar17 = vec4<f32>( nodeVar16, 0.0 );
	nodeVar18 = ( render.cameraViewMatrix * nodeVar17 );
	nodeVar19 = normalize( nodeVar18.xyz );
	nodeVar20 = nodeVar19;
	nodeVar21 = dot( normalView, nodeVar20 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar23 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform13 ) ) ), 1.0 ) );
	nodeVar24 = ( nodeVar23.xyz / vec3<f32>( nodeVar23.w ) );
	nodeVar25 = vec3<f32>( nodeVar24.x, ( 1.0 - nodeVar24.y ), ( nodeVar24.z + render.nodeUniform14 ) );

	if ( ( ( ( ( ( nodeVar25.x >= 0.0 ) && ( nodeVar25.x <= 1.0 ) ) && ( nodeVar25.y >= 0.0 ) ) && ( nodeVar25.y <= 1.0 ) ) && ( nodeVar25.z <= 1.0 ) ) ) {

		nodeVar26 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar27 = ( render.nodeUniform16 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ).x );
		nodeVar28 = ( nodeVar25.xy + ( vogelDiskSample( 0, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
		nodeVar29 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar28, nodeVar25.z );
		nodeVar30 = ( nodeVar25.xy + ( vogelDiskSample( 1, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
		nodeVar31 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar30, nodeVar25.z );
		nodeVar32 = ( nodeVar25.xy + ( vogelDiskSample( 2, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
		nodeVar33 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar32, nodeVar25.z );
		nodeVar34 = ( nodeVar25.xy + ( vogelDiskSample( 3, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
		nodeVar35 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar34, nodeVar25.z );
		nodeVar36 = ( nodeVar25.xy + ( vogelDiskSample( 4, 5, nodeVar26 ) * vec2<f32>( nodeVar27 ) ) );
		nodeVar37 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar36, nodeVar25.z );
		nodeVar22 = ( ( ( ( ( nodeVar29 + nodeVar31 ) + nodeVar33 ) + nodeVar35 ) + nodeVar37 ) * 0.2 );

	} else {

		nodeVar22 = 1.0;

	}

	nodeVar38 = mix( 1.0, nodeVar22, render.nodeUniform18 );
	nodeVar39 = ( vec3<f32>( clamp( nodeVar21, 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar38 ) ) );
	nodeVar40 = nodeVar39;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar41 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar42 = ( nodeVar40 * nodeVar41 );
	nodeVar43 = ( nodeVar20 + positionViewDirection );
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
	nodeVar53 = normalize( ( nodeVar20 + positionViewDirection ) );
	nodeVar54 = clamp( dot( positionViewDirection, nodeVar53 ), 0.0, 1.0 );
	nodeVar55 = exp2( ( ( ( nodeVar54 * -5.55473 ) - 6.98316 ) * nodeVar54 ) );
	nodeVar56 = ( Roughness * Roughness );
	nodeVar57 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar55 ) ) ) + vec3<f32>( ( 1.0 * nodeVar55 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar56, clamp( dot( normalView, nodeVar20 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar56, clamp( dot( normalView, nodeVar53 ), 0.0, 1.0 ) ) ) );
	nodeVar58 = ( nodeVar40 * nodeVar57 );
	nodeVar59 = ( nodeVar58 * multiScatteringCompensation );
	nodeVar60 = ( directSpecular + nodeVar59 );
	directSpecular = nodeVar60;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar61 = ( object.nodeUniform20 - object.nodeUniform21 );
	nodeVar62 = ( object.nodeUniform22 - vec3<f32>( 1.0 ) );
	nodeVar63 = ( nodeVar61 / nodeVar62 );
	nodeVar64 = ( normalWorld * nodeVar63 );
	nodeVar65 = ( nodeVar64 * vec3<f32>( 0.5 ) );
	nodeVar66 = ( v_positionWorld + nodeVar65 );
	nodeVar67 = ( nodeVar66 - object.nodeUniform21 );
	nodeVar68 = ( nodeVar67 / nodeVar61 );
	nodeVar69 = ( clamp( nodeVar68, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar62 );
	nodeVar70 = ( nodeVar69 / object.nodeUniform22 );
	nodeVar71 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform22 );
	nodeVar72 = ( nodeVar70 + nodeVar71 );
	nodeVar73 = ( nodeVar72.z * object.nodeUniform22.z );
	nodeVar74 = ( nodeVar73 + 1.0 );
	nodeVar75 = ( object.nodeUniform22.z + 2.0 );
	nodeVar76 = ( nodeVar75 * 0.0 );
	nodeVar77 = ( nodeVar74 + nodeVar76 );
	nodeVar78 = ( nodeVar75 * 7.0 );
	nodeVar79 = ( nodeVar77 / nodeVar78 );
	nodeVar80 = vec3<f32>( nodeVar72.xy, nodeVar79 );
	nodeVar81 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar80 );
	nodeVar82 = ( nodeVar75 * 1.0 );
	nodeVar83 = ( nodeVar74 + nodeVar82 );
	nodeVar84 = ( nodeVar83 / nodeVar78 );
	nodeVar85 = vec3<f32>( nodeVar72.xy, nodeVar84 );
	nodeVar86 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar85 );
	nodeVar87 = vec3<f32>( nodeVar81.w, nodeVar86.xy );
	nodeVar88 = ( nodeVar75 * 2.0 );
	nodeVar89 = ( nodeVar74 + nodeVar88 );
	nodeVar90 = ( nodeVar89 / nodeVar78 );
	nodeVar91 = vec3<f32>( nodeVar72.xy, nodeVar90 );
	nodeVar92 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar91 );
	nodeVar93 = vec3<f32>( nodeVar86.zw, nodeVar92.x );
	nodeVar94 = ( nodeVar75 * 3.0 );
	nodeVar95 = ( nodeVar74 + nodeVar94 );
	nodeVar96 = ( nodeVar95 / nodeVar78 );
	nodeVar97 = vec3<f32>( nodeVar72.xy, nodeVar96 );
	nodeVar98 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar97 );
	nodeVar99 = ( nodeVar75 * 4.0 );
	nodeVar100 = ( nodeVar74 + nodeVar99 );
	nodeVar101 = ( nodeVar100 / nodeVar78 );
	nodeVar102 = vec3<f32>( nodeVar72.xy, nodeVar101 );
	nodeVar103 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar102 );
	nodeVar104 = vec3<f32>( nodeVar98.w, nodeVar103.xy );
	nodeVar105 = ( nodeVar75 * 5.0 );
	nodeVar106 = ( nodeVar74 + nodeVar105 );
	nodeVar107 = ( nodeVar106 / nodeVar78 );
	nodeVar108 = vec3<f32>( nodeVar72.xy, nodeVar107 );
	nodeVar109 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar108 );
	nodeVar110 = vec3<f32>( nodeVar103.zw, nodeVar109.x );
	nodeVar111 = ( nodeVar75 * 6.0 );
	nodeVar112 = ( nodeVar74 + nodeVar111 );
	nodeVar113 = ( nodeVar112 / nodeVar78 );
	nodeVar114 = vec3<f32>( nodeVar72.xy, nodeVar113 );
	nodeVar115 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar114 );
	nodeVar116 = array< vec3<f32>, 9 >( nodeVar81.xyz, nodeVar87, nodeVar93, nodeVar92.yzw, nodeVar98.xyz, nodeVar104, nodeVar110, nodeVar109.yzw, nodeVar115.xyz );
	nodeVar117 = ( ( ( ( ( ( ( ( ( nodeVar116[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar116[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar116[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar116[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar116[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar116[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar116[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar116[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar116[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar118 = max( nodeVar117, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar119 = ( nodeVar118 * vec3<f32>( object.nodeUniform23 ) );
	nodeVar120 = ( irradiance + nodeVar119 );
	irradiance = nodeVar120;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar121 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform24 );
	nodeVar122 = floor( nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar125 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar124.x, ( - nodeVar124.y ), nodeVar124.z ), 1.0 ) ).xyz );
	nodeVar126 = max( ( 4.0 - nodeVar123 ), 0.0 );
	nodeVar123 = max( nodeVar123, 4.0 );
	nodeVar127 = exp2( nodeVar123 );
	nodeVar128 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar124.x, ( - nodeVar124.y ), nodeVar124.z ), 1.0 ) ).xyz, nodeVar125 ) * vec2<f32>( ( nodeVar127 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar125 > 2.0 ) ) {

		nodeVar128.y = ( nodeVar128.y + nodeVar127 );
		nodeVar125 = ( nodeVar125 - 3.0 );
		

	}

	nodeVar128.x = ( nodeVar128.x + ( nodeVar125 * nodeVar127 ) );
	nodeVar128.x = ( nodeVar128.x + ( nodeVar126 * ( 3.0 * 16.0 ) ) );
	nodeVar128.y = ( nodeVar128.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar127 ) ) );
	nodeVar128.x = ( nodeVar128.x * object.nodeUniform27 );
	nodeVar128.y = ( nodeVar128.y * object.nodeUniform28 );
	nodeVar129 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar128, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar130 = nodeVar129.xyz;
	nodeVar131 = fract( nodeVar121 );

	if ( ( nodeVar131 != 0.0 ) ) {

		nodeVar132 = ( nodeVar122 + 1.0 );
		nodeVar133 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar124.x, ( - nodeVar124.y ), nodeVar124.z ), 1.0 ) ).xyz );
		nodeVar134 = max( ( 4.0 - nodeVar132 ), 0.0 );
		nodeVar132 = max( nodeVar132, 4.0 );
		nodeVar135 = exp2( nodeVar132 );
		nodeVar136 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar124.x, ( - nodeVar124.y ), nodeVar124.z ), 1.0 ) ).xyz, nodeVar133 ) * vec2<f32>( ( nodeVar135 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar133 > 2.0 ) ) {

			nodeVar136.y = ( nodeVar136.y + nodeVar135 );
			nodeVar133 = ( nodeVar133 - 3.0 );
			

		}

		nodeVar136.x = ( nodeVar136.x + ( nodeVar133 * nodeVar135 ) );
		nodeVar136.x = ( nodeVar136.x + ( nodeVar134 * ( 3.0 * 16.0 ) ) );
		nodeVar136.y = ( nodeVar136.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar135 ) ) );
		nodeVar136.x = ( nodeVar136.x * object.nodeUniform27 );
		nodeVar136.y = ( nodeVar136.y * object.nodeUniform28 );
		nodeVar137 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar136, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar138 = nodeVar137.xyz;
		nodeVar130 = mix( nodeVar130, nodeVar138, nodeVar131 );
		

	}

	nodeVar139 = ( radiance + ( nodeVar130 * vec3<f32>( object.nodeUniform30 ) ) );
	radiance = nodeVar139;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform24 );
	nodeVar141 = floor( nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar144 = max( ( 4.0 - nodeVar142 ), 0.0 );
	nodeVar142 = max( nodeVar142, 4.0 );
	nodeVar145 = exp2( nodeVar142 );
	nodeVar146 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar143 ) * vec2<f32>( ( nodeVar145 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar143 > 2.0 ) ) {

		nodeVar146.y = ( nodeVar146.y + nodeVar145 );
		nodeVar143 = ( nodeVar143 - 3.0 );
		

	}

	nodeVar146.x = ( nodeVar146.x + ( nodeVar143 * nodeVar145 ) );
	nodeVar146.x = ( nodeVar146.x + ( nodeVar144 * ( 3.0 * 16.0 ) ) );
	nodeVar146.y = ( nodeVar146.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar145 ) ) );
	nodeVar146.x = ( nodeVar146.x * object.nodeUniform27 );
	nodeVar146.y = ( nodeVar146.y * object.nodeUniform28 );
	nodeVar147 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar146, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar148 = nodeVar147.xyz;
	nodeVar149 = fract( nodeVar140 );

	if ( ( nodeVar149 != 0.0 ) ) {

		nodeVar150 = ( nodeVar141 + 1.0 );
		nodeVar151 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar152 = max( ( 4.0 - nodeVar150 ), 0.0 );
		nodeVar150 = max( nodeVar150, 4.0 );
		nodeVar153 = exp2( nodeVar150 );
		nodeVar154 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar151 ) * vec2<f32>( ( nodeVar153 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar151 > 2.0 ) ) {

			nodeVar154.y = ( nodeVar154.y + nodeVar153 );
			nodeVar151 = ( nodeVar151 - 3.0 );
			

		}

		nodeVar154.x = ( nodeVar154.x + ( nodeVar151 * nodeVar153 ) );
		nodeVar154.x = ( nodeVar154.x + ( nodeVar152 * ( 3.0 * 16.0 ) ) );
		nodeVar154.y = ( nodeVar154.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar153 ) ) );
		nodeVar154.x = ( nodeVar154.x * object.nodeUniform27 );
		nodeVar154.y = ( nodeVar154.y * object.nodeUniform28 );
		nodeVar155 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar154, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar156 = nodeVar155.xyz;
		nodeVar148 = mix( nodeVar148, nodeVar156, nodeVar149 );
		

	}

	nodeVar157 = ( iblIrradiance + ( ( nodeVar148 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform30 ) ) );
	iblIrradiance = nodeVar157;
	nodeVar158 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar159 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar160 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar161 = ( SpecularF90 * dfg.y );
	nodeVar162 = ( nodeVar160 + vec3<f32>( nodeVar161 ) );
	nodeVar163 = ( nodeVar158 + nodeVar162 );
	nodeVar158 = nodeVar163;
	nodeVar164 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( nodeVar165 * vec3<f32>( 0.047619 ) );
	nodeVar167 = ( SpecularColor + nodeVar166 );
	nodeVar168 = ( nodeVar162 * nodeVar167 );
	nodeVar169 = ( dfg.x + dfg.y );
	nodeVar170 = ( 1.0 - nodeVar169 );
	nodeVar171 = nodeVar170;
	nodeVar172 = ( vec3<f32>( nodeVar171 ) * nodeVar167 );
	nodeVar173 = ( vec3<f32>( 1.0 ) - nodeVar172 );
	nodeVar174 = nodeVar173;
	nodeVar175 = ( nodeVar168 / nodeVar174 );
	nodeVar176 = ( nodeVar175 * vec3<f32>( nodeVar171 ) );
	nodeVar177 = ( nodeVar159 + nodeVar176 );
	nodeVar159 = nodeVar177;
	nodeVar178 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar179 = ( irradiance * nodeVar178 );
	nodeVar180 = ( nodeVar158 + nodeVar159 );
	nodeVar181 = ( vec3<f32>( 1.0 ) - nodeVar180 );
	nodeVar182 = nodeVar181;
	nodeVar183 = ( nodeVar179 * nodeVar182 );
	nodeVar184 = nodeVar183;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar185 = ( indirectDiffuse + nodeVar184 );
	indirectDiffuse = nodeVar185;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar186 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar187 = ( SpecularF90 * dfg.y );
	nodeVar188 = ( nodeVar186 + vec3<f32>( nodeVar187 ) );
	nodeVar189 = ( singleScatteringDielectric + nodeVar188 );
	singleScatteringDielectric = nodeVar189;
	nodeVar190 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar191 = nodeVar190;
	nodeVar192 = ( nodeVar191 * vec3<f32>( 0.047619 ) );
	nodeVar193 = ( SpecularColor + nodeVar192 );
	nodeVar194 = ( nodeVar188 * nodeVar193 );
	nodeVar195 = ( dfg.x + dfg.y );
	nodeVar196 = ( 1.0 - nodeVar195 );
	nodeVar197 = nodeVar196;
	nodeVar198 = ( vec3<f32>( nodeVar197 ) * nodeVar193 );
	nodeVar199 = ( vec3<f32>( 1.0 ) - nodeVar198 );
	nodeVar200 = nodeVar199;
	nodeVar201 = ( nodeVar194 / nodeVar200 );
	nodeVar202 = ( nodeVar201 * vec3<f32>( nodeVar197 ) );
	nodeVar203 = ( multiScatteringDielectric + nodeVar202 );
	multiScatteringDielectric = nodeVar203;
	nodeVar204 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar205 = ( SpecularF90 * dfg.y );
	nodeVar206 = ( nodeVar204 + vec3<f32>( nodeVar205 ) );
	nodeVar207 = ( singleScatteringMetallic + nodeVar206 );
	singleScatteringMetallic = nodeVar207;
	nodeVar208 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar209 = nodeVar208;
	nodeVar210 = ( nodeVar209 * vec3<f32>( 0.047619 ) );
	nodeVar211 = ( DiffuseColor.xyz + nodeVar210 );
	nodeVar212 = ( nodeVar206 * nodeVar211 );
	nodeVar213 = ( dfg.x + dfg.y );
	nodeVar214 = ( 1.0 - nodeVar213 );
	nodeVar215 = nodeVar214;
	nodeVar216 = ( vec3<f32>( nodeVar215 ) * nodeVar211 );
	nodeVar217 = ( vec3<f32>( 1.0 ) - nodeVar216 );
	nodeVar218 = nodeVar217;
	nodeVar219 = ( nodeVar212 / nodeVar218 );
	nodeVar220 = ( nodeVar219 * vec3<f32>( nodeVar215 ) );
	nodeVar221 = ( multiScatteringMetallic + nodeVar220 );
	multiScatteringMetallic = nodeVar221;
	nodeVar222 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar223 = ( radiance * nodeVar222 );
	nodeVar224 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar225 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar226 = ( nodeVar224 * nodeVar225 );
	nodeVar227 = ( nodeVar223 + nodeVar226 );
	nodeVar228 = nodeVar227;
	nodeVar229 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar230 = ( vec3<f32>( 1.0 ) - nodeVar229 );
	nodeVar231 = nodeVar230;
	nodeVar232 = ( DiffuseContribution * nodeVar231 );
	nodeVar233 = ( nodeVar232 * nodeVar225 );
	nodeVar234 = nodeVar233;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar235 = ( indirectSpecular + nodeVar228 );
	indirectSpecular = nodeVar235;
	nodeVar236 = ( indirectDiffuse + nodeVar234 );
	indirectDiffuse = nodeVar236;
	ambientOcclusion = 1.0;
	nodeVar237 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar237;
	nodeVar238 = dot( normalView, positionViewDirection );
	nodeVar239 = ( clamp( nodeVar238, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar240 = ( Roughness * -16.0 );
	nodeVar241 = ( 1.0 - nodeVar240 );
	nodeVar242 = nodeVar241;
	nodeVar243 = ( - nodeVar242 );
	nodeVar244 = exp2( nodeVar243 );
	nodeVar245 = pow( nodeVar239, nodeVar244 );
	nodeVar246 = ( 1.0 - nodeVar245 );
	nodeVar247 = nodeVar246;
	nodeVar248 = ( ambientOcclusion - nodeVar247 );
	nodeVar249 = ( indirectSpecular * vec3<f32>( clamp( nodeVar248, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar249;
	nodeVar250 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar250;
	nodeVar251 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar251;
	nodeVar252 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar252;
	nodeVar253 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar253;

	// result

	output.color = nodeVar253;

	return output;

}
