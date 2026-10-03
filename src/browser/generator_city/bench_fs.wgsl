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
var<private> Metalness : f32;
var<private> nodeVar2 : f32;
var<private> Roughness : f32;
var<private> nodeVar3 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar19 : f32;
var<private> normalWorld : vec3<f32>;
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
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec4<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec4<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec4<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec4<f32>;
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
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec4<f32>;
var<private> nodeVar113 : array< vec3<f32>, 9 >;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec2<f32>;
var<private> nodeVar126 : vec4<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : vec2<f32>;
var<private> nodeVar134 : vec4<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : vec2<f32>;
var<private> nodeVar144 : vec4<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : vec2<f32>;
var<private> nodeVar152 : vec4<f32>;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : f32;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : f32;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : f32;
var<private> nodeVar211 : f32;
var<private> nodeVar212 : f32;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> nodeVar233 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : f32;
var<private> nodeVar236 : f32;
var<private> nodeVar237 : f32;
var<private> nodeVar238 : f32;
var<private> nodeVar239 : f32;
var<private> nodeVar240 : f32;
var<private> nodeVar241 : f32;
var<private> nodeVar242 : f32;
var<private> nodeVar243 : f32;
var<private> nodeVar244 : f32;
var<private> nodeVar245 : f32;
var<private> nodeVar246 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar247 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar248 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar249 : vec3<f32>;
var<private> nodeVar250 : vec4<f32>;

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
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = ( nodeVarying3 == 0.0 );

	if ( nodeVar1 ) {

		nodeVar0 = mix( mix( vec3<f32>( 0.11193242782769693, 0.05286064701616471, 0.023153366173251363 ), vec3<f32>( 0.2541520943200296, 0.12213877222015301, 0.054480276435339814 ), ( ( ( ( ( mx_fractal_noise_float( ( nodeVarying8 * vec3<f32>( 3.0, 60.0, 60.0 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) * 0.7 ) + ( fract( ( sin( ( ( floor( ( nodeVarying8.z * 10.5 ) ) + ( floor( ( nodeVarying8.y * 8.0 ) ) * 7.3 ) ) * 17.53 ) ) * 43758.5453 ) ) * 0.3 ) ) ), vec3<f32>( 0.20155625378383743, 0.174647403645279, 0.13843161502267545 ), ( ( ( ( mx_fractal_noise_float( ( nodeVarying8 * vec3<f32>( 1.6 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) * 0.45 ) );

	} else {

		nodeVar0 = vec3<f32>( 0.01680737574872402, 0.01680737574872402, 0.015996293361446288 );

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;

	if ( nodeVar1 ) {

		nodeVar2 = 0.0;

	} else {

		nodeVar2 = 0.6;

	}

	Metalness = nodeVar2;

	if ( nodeVar1 ) {

		nodeVar3 = ( ( ( ( ( mx_fractal_noise_float( ( nodeVarying8 * vec3<f32>( 3.0, 60.0, 60.0 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) * 0.25 ) + 0.65 );

	} else {

		nodeVar3 = 0.5;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar3, 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeVar2 ) ) );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar5 = dot( normalView, positionViewDirection );
	nodeVar6 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( nodeVar5, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar6;
	nodeVar7 = ( dfg.x + dfg.y );
	nodeVar8 = ( 1.0 / nodeVar7 );
	nodeVar9 = nodeVar8;
	nodeVar10 = ( nodeVar9 - 1.0 );
	nodeVar11 = ( SpecularColorBlended * vec3<f32>( nodeVar10 ) );
	nodeVar12 = ( nodeVar11 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar12;
	nodeVar13 = ( render.nodeUniform9 - render.nodeUniform10 );
	nodeVar14 = vec4<f32>( nodeVar13, 0.0 );
	nodeVar15 = ( render.cameraViewMatrix * nodeVar14 );
	nodeVar16 = normalize( nodeVar15.xyz );
	nodeVar17 = nodeVar16;
	nodeVar18 = dot( normalView, nodeVar17 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar20 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform13 ) ) ), 1.0 ) );
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

	nodeVar35 = mix( 1.0, nodeVar19, render.nodeUniform18 );
	nodeVar36 = ( vec3<f32>( clamp( nodeVar18, 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar35 ) ) );
	nodeVar37 = nodeVar36;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar38 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar39 = ( nodeVar37 * nodeVar38 );
	nodeVar40 = ( nodeVar17 + positionViewDirection );
	nodeVar41 = normalize( nodeVar40 );
	nodeVar42 = dot( positionViewDirection, nodeVar41 );
	nodeVar43 = clamp( nodeVar42, 0.0, 1.0 );
	nodeVar44 = exp2( ( ( ( nodeVar43 * -5.55473 ) - 6.98316 ) * nodeVar43 ) );
	nodeVar45 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar44 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar44 ) ) );
	nodeVar46 = ( vec3<f32>( 1.0 ) - nodeVar45 );
	nodeVar47 = nodeVar46;
	nodeVar48 = ( nodeVar39 * nodeVar47 );
	nodeVar49 = ( directDiffuse + nodeVar48 );
	directDiffuse = nodeVar49;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar50 = normalize( ( nodeVar17 + positionViewDirection ) );
	nodeVar51 = clamp( dot( positionViewDirection, nodeVar50 ), 0.0, 1.0 );
	nodeVar52 = exp2( ( ( ( nodeVar51 * -5.55473 ) - 6.98316 ) * nodeVar51 ) );
	nodeVar53 = ( Roughness * Roughness );
	nodeVar54 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar52 ) ) ) + vec3<f32>( ( 1.0 * nodeVar52 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar53, clamp( dot( normalView, nodeVar17 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar53, clamp( dot( normalView, nodeVar50 ), 0.0, 1.0 ) ) ) );
	nodeVar55 = ( nodeVar37 * nodeVar54 );
	nodeVar56 = ( nodeVar55 * multiScatteringCompensation );
	nodeVar57 = ( directSpecular + nodeVar56 );
	directSpecular = nodeVar57;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar58 = ( object.nodeUniform20 - object.nodeUniform21 );
	nodeVar59 = ( object.nodeUniform22 - vec3<f32>( 1.0 ) );
	nodeVar60 = ( nodeVar58 / nodeVar59 );
	nodeVar61 = ( normalWorld * nodeVar60 );
	nodeVar62 = ( nodeVar61 * vec3<f32>( 0.5 ) );
	nodeVar63 = ( v_positionWorld + nodeVar62 );
	nodeVar64 = ( nodeVar63 - object.nodeUniform21 );
	nodeVar65 = ( nodeVar64 / nodeVar58 );
	nodeVar66 = ( clamp( nodeVar65, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar59 );
	nodeVar67 = ( nodeVar66 / object.nodeUniform22 );
	nodeVar68 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform22 );
	nodeVar69 = ( nodeVar67 + nodeVar68 );
	nodeVar70 = ( nodeVar69.z * object.nodeUniform22.z );
	nodeVar71 = ( nodeVar70 + 1.0 );
	nodeVar72 = ( object.nodeUniform22.z + 2.0 );
	nodeVar73 = ( nodeVar72 * 0.0 );
	nodeVar74 = ( nodeVar71 + nodeVar73 );
	nodeVar75 = ( nodeVar72 * 7.0 );
	nodeVar76 = ( nodeVar74 / nodeVar75 );
	nodeVar77 = vec3<f32>( nodeVar69.xy, nodeVar76 );
	nodeVar78 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar77 );
	nodeVar79 = ( nodeVar72 * 1.0 );
	nodeVar80 = ( nodeVar71 + nodeVar79 );
	nodeVar81 = ( nodeVar80 / nodeVar75 );
	nodeVar82 = vec3<f32>( nodeVar69.xy, nodeVar81 );
	nodeVar83 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar82 );
	nodeVar84 = vec3<f32>( nodeVar78.w, nodeVar83.xy );
	nodeVar85 = ( nodeVar72 * 2.0 );
	nodeVar86 = ( nodeVar71 + nodeVar85 );
	nodeVar87 = ( nodeVar86 / nodeVar75 );
	nodeVar88 = vec3<f32>( nodeVar69.xy, nodeVar87 );
	nodeVar89 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar88 );
	nodeVar90 = vec3<f32>( nodeVar83.zw, nodeVar89.x );
	nodeVar91 = ( nodeVar72 * 3.0 );
	nodeVar92 = ( nodeVar71 + nodeVar91 );
	nodeVar93 = ( nodeVar92 / nodeVar75 );
	nodeVar94 = vec3<f32>( nodeVar69.xy, nodeVar93 );
	nodeVar95 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar94 );
	nodeVar96 = ( nodeVar72 * 4.0 );
	nodeVar97 = ( nodeVar71 + nodeVar96 );
	nodeVar98 = ( nodeVar97 / nodeVar75 );
	nodeVar99 = vec3<f32>( nodeVar69.xy, nodeVar98 );
	nodeVar100 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar99 );
	nodeVar101 = vec3<f32>( nodeVar95.w, nodeVar100.xy );
	nodeVar102 = ( nodeVar72 * 5.0 );
	nodeVar103 = ( nodeVar71 + nodeVar102 );
	nodeVar104 = ( nodeVar103 / nodeVar75 );
	nodeVar105 = vec3<f32>( nodeVar69.xy, nodeVar104 );
	nodeVar106 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar105 );
	nodeVar107 = vec3<f32>( nodeVar100.zw, nodeVar106.x );
	nodeVar108 = ( nodeVar72 * 6.0 );
	nodeVar109 = ( nodeVar71 + nodeVar108 );
	nodeVar110 = ( nodeVar109 / nodeVar75 );
	nodeVar111 = vec3<f32>( nodeVar69.xy, nodeVar110 );
	nodeVar112 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar111 );
	nodeVar113 = array< vec3<f32>, 9 >( nodeVar78.xyz, nodeVar84, nodeVar90, nodeVar89.yzw, nodeVar95.xyz, nodeVar101, nodeVar107, nodeVar106.yzw, nodeVar112.xyz );
	nodeVar114 = ( ( ( ( ( ( ( ( ( nodeVar113[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar113[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar113[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar113[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar113[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar113[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar113[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar113[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar113[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar115 = max( nodeVar114, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar116 = ( nodeVar115 * vec3<f32>( object.nodeUniform23 ) );
	nodeVar117 = ( irradiance + nodeVar116 );
	irradiance = nodeVar117;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar118 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform24 );
	nodeVar119 = floor( nodeVar118 );
	nodeVar120 = nodeVar119;
	nodeVar121 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar122 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar121.x, ( - nodeVar121.y ), nodeVar121.z ), 1.0 ) ).xyz );
	nodeVar123 = max( ( 4.0 - nodeVar120 ), 0.0 );
	nodeVar120 = max( nodeVar120, 4.0 );
	nodeVar124 = exp2( nodeVar120 );
	nodeVar125 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar121.x, ( - nodeVar121.y ), nodeVar121.z ), 1.0 ) ).xyz, nodeVar122 ) * vec2<f32>( ( nodeVar124 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar122 > 2.0 ) ) {

		nodeVar125.y = ( nodeVar125.y + nodeVar124 );
		nodeVar122 = ( nodeVar122 - 3.0 );
		

	}

	nodeVar125.x = ( nodeVar125.x + ( nodeVar122 * nodeVar124 ) );
	nodeVar125.x = ( nodeVar125.x + ( nodeVar123 * ( 3.0 * 16.0 ) ) );
	nodeVar125.y = ( nodeVar125.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar124 ) ) );
	nodeVar125.x = ( nodeVar125.x * object.nodeUniform27 );
	nodeVar125.y = ( nodeVar125.y * object.nodeUniform28 );
	nodeVar126 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar125, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar127 = nodeVar126.xyz;
	nodeVar128 = fract( nodeVar118 );

	if ( ( nodeVar128 != 0.0 ) ) {

		nodeVar129 = ( nodeVar119 + 1.0 );
		nodeVar130 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar121.x, ( - nodeVar121.y ), nodeVar121.z ), 1.0 ) ).xyz );
		nodeVar131 = max( ( 4.0 - nodeVar129 ), 0.0 );
		nodeVar129 = max( nodeVar129, 4.0 );
		nodeVar132 = exp2( nodeVar129 );
		nodeVar133 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar121.x, ( - nodeVar121.y ), nodeVar121.z ), 1.0 ) ).xyz, nodeVar130 ) * vec2<f32>( ( nodeVar132 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar130 > 2.0 ) ) {

			nodeVar133.y = ( nodeVar133.y + nodeVar132 );
			nodeVar130 = ( nodeVar130 - 3.0 );
			

		}

		nodeVar133.x = ( nodeVar133.x + ( nodeVar130 * nodeVar132 ) );
		nodeVar133.x = ( nodeVar133.x + ( nodeVar131 * ( 3.0 * 16.0 ) ) );
		nodeVar133.y = ( nodeVar133.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar132 ) ) );
		nodeVar133.x = ( nodeVar133.x * object.nodeUniform27 );
		nodeVar133.y = ( nodeVar133.y * object.nodeUniform28 );
		nodeVar134 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar133, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar135 = nodeVar134.xyz;
		nodeVar127 = mix( nodeVar127, nodeVar135, nodeVar128 );
		

	}

	nodeVar136 = ( radiance + ( nodeVar127 * vec3<f32>( object.nodeUniform30 ) ) );
	radiance = nodeVar136;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar137 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform24 );
	nodeVar138 = floor( nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar141 = max( ( 4.0 - nodeVar139 ), 0.0 );
	nodeVar139 = max( nodeVar139, 4.0 );
	nodeVar142 = exp2( nodeVar139 );
	nodeVar143 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar140 ) * vec2<f32>( ( nodeVar142 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar140 > 2.0 ) ) {

		nodeVar143.y = ( nodeVar143.y + nodeVar142 );
		nodeVar140 = ( nodeVar140 - 3.0 );
		

	}

	nodeVar143.x = ( nodeVar143.x + ( nodeVar140 * nodeVar142 ) );
	nodeVar143.x = ( nodeVar143.x + ( nodeVar141 * ( 3.0 * 16.0 ) ) );
	nodeVar143.y = ( nodeVar143.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar142 ) ) );
	nodeVar143.x = ( nodeVar143.x * object.nodeUniform27 );
	nodeVar143.y = ( nodeVar143.y * object.nodeUniform28 );
	nodeVar144 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar143, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar145 = nodeVar144.xyz;
	nodeVar146 = fract( nodeVar137 );

	if ( ( nodeVar146 != 0.0 ) ) {

		nodeVar147 = ( nodeVar138 + 1.0 );
		nodeVar148 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar149 = max( ( 4.0 - nodeVar147 ), 0.0 );
		nodeVar147 = max( nodeVar147, 4.0 );
		nodeVar150 = exp2( nodeVar147 );
		nodeVar151 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar148 ) * vec2<f32>( ( nodeVar150 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar148 > 2.0 ) ) {

			nodeVar151.y = ( nodeVar151.y + nodeVar150 );
			nodeVar148 = ( nodeVar148 - 3.0 );
			

		}

		nodeVar151.x = ( nodeVar151.x + ( nodeVar148 * nodeVar150 ) );
		nodeVar151.x = ( nodeVar151.x + ( nodeVar149 * ( 3.0 * 16.0 ) ) );
		nodeVar151.y = ( nodeVar151.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar150 ) ) );
		nodeVar151.x = ( nodeVar151.x * object.nodeUniform27 );
		nodeVar151.y = ( nodeVar151.y * object.nodeUniform28 );
		nodeVar152 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar151, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar153 = nodeVar152.xyz;
		nodeVar145 = mix( nodeVar145, nodeVar153, nodeVar146 );
		

	}

	nodeVar154 = ( iblIrradiance + ( ( nodeVar145 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform30 ) ) );
	iblIrradiance = nodeVar154;
	nodeVar155 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar156 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar157 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar158 = ( SpecularF90 * dfg.y );
	nodeVar159 = ( nodeVar157 + vec3<f32>( nodeVar158 ) );
	nodeVar160 = ( nodeVar155 + nodeVar159 );
	nodeVar155 = nodeVar160;
	nodeVar161 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( nodeVar162 * vec3<f32>( 0.047619 ) );
	nodeVar164 = ( SpecularColor + nodeVar163 );
	nodeVar165 = ( nodeVar159 * nodeVar164 );
	nodeVar166 = ( dfg.x + dfg.y );
	nodeVar167 = ( 1.0 - nodeVar166 );
	nodeVar168 = nodeVar167;
	nodeVar169 = ( vec3<f32>( nodeVar168 ) * nodeVar164 );
	nodeVar170 = ( vec3<f32>( 1.0 ) - nodeVar169 );
	nodeVar171 = nodeVar170;
	nodeVar172 = ( nodeVar165 / nodeVar171 );
	nodeVar173 = ( nodeVar172 * vec3<f32>( nodeVar168 ) );
	nodeVar174 = ( nodeVar156 + nodeVar173 );
	nodeVar156 = nodeVar174;
	nodeVar175 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar176 = ( irradiance * nodeVar175 );
	nodeVar177 = ( nodeVar155 + nodeVar156 );
	nodeVar178 = ( vec3<f32>( 1.0 ) - nodeVar177 );
	nodeVar179 = nodeVar178;
	nodeVar180 = ( nodeVar176 * nodeVar179 );
	nodeVar181 = nodeVar180;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar182 = ( indirectDiffuse + nodeVar181 );
	indirectDiffuse = nodeVar182;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar183 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar184 = ( SpecularF90 * dfg.y );
	nodeVar185 = ( nodeVar183 + vec3<f32>( nodeVar184 ) );
	nodeVar186 = ( singleScatteringDielectric + nodeVar185 );
	singleScatteringDielectric = nodeVar186;
	nodeVar187 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar188 = nodeVar187;
	nodeVar189 = ( nodeVar188 * vec3<f32>( 0.047619 ) );
	nodeVar190 = ( SpecularColor + nodeVar189 );
	nodeVar191 = ( nodeVar185 * nodeVar190 );
	nodeVar192 = ( dfg.x + dfg.y );
	nodeVar193 = ( 1.0 - nodeVar192 );
	nodeVar194 = nodeVar193;
	nodeVar195 = ( vec3<f32>( nodeVar194 ) * nodeVar190 );
	nodeVar196 = ( vec3<f32>( 1.0 ) - nodeVar195 );
	nodeVar197 = nodeVar196;
	nodeVar198 = ( nodeVar191 / nodeVar197 );
	nodeVar199 = ( nodeVar198 * vec3<f32>( nodeVar194 ) );
	nodeVar200 = ( multiScatteringDielectric + nodeVar199 );
	multiScatteringDielectric = nodeVar200;
	nodeVar201 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar202 = ( SpecularF90 * dfg.y );
	nodeVar203 = ( nodeVar201 + vec3<f32>( nodeVar202 ) );
	nodeVar204 = ( singleScatteringMetallic + nodeVar203 );
	singleScatteringMetallic = nodeVar204;
	nodeVar205 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar206 = nodeVar205;
	nodeVar207 = ( nodeVar206 * vec3<f32>( 0.047619 ) );
	nodeVar208 = ( DiffuseColor.xyz + nodeVar207 );
	nodeVar209 = ( nodeVar203 * nodeVar208 );
	nodeVar210 = ( dfg.x + dfg.y );
	nodeVar211 = ( 1.0 - nodeVar210 );
	nodeVar212 = nodeVar211;
	nodeVar213 = ( vec3<f32>( nodeVar212 ) * nodeVar208 );
	nodeVar214 = ( vec3<f32>( 1.0 ) - nodeVar213 );
	nodeVar215 = nodeVar214;
	nodeVar216 = ( nodeVar209 / nodeVar215 );
	nodeVar217 = ( nodeVar216 * vec3<f32>( nodeVar212 ) );
	nodeVar218 = ( multiScatteringMetallic + nodeVar217 );
	multiScatteringMetallic = nodeVar218;
	nodeVar219 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar220 = ( radiance * nodeVar219 );
	nodeVar221 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar222 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar223 = ( nodeVar221 * nodeVar222 );
	nodeVar224 = ( nodeVar220 + nodeVar223 );
	nodeVar225 = nodeVar224;
	nodeVar226 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar227 = ( vec3<f32>( 1.0 ) - nodeVar226 );
	nodeVar228 = nodeVar227;
	nodeVar229 = ( DiffuseContribution * nodeVar228 );
	nodeVar230 = ( nodeVar229 * nodeVar222 );
	nodeVar231 = nodeVar230;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar232 = ( indirectSpecular + nodeVar225 );
	indirectSpecular = nodeVar232;
	nodeVar233 = ( indirectDiffuse + nodeVar231 );
	indirectDiffuse = nodeVar233;
	ambientOcclusion = 1.0;
	nodeVar234 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar234;
	nodeVar235 = dot( normalView, positionViewDirection );
	nodeVar236 = ( clamp( nodeVar235, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar237 = ( Roughness * -16.0 );
	nodeVar238 = ( 1.0 - nodeVar237 );
	nodeVar239 = nodeVar238;
	nodeVar240 = ( - nodeVar239 );
	nodeVar241 = exp2( nodeVar240 );
	nodeVar242 = pow( nodeVar236, nodeVar241 );
	nodeVar243 = ( 1.0 - nodeVar242 );
	nodeVar244 = nodeVar243;
	nodeVar245 = ( ambientOcclusion - nodeVar244 );
	nodeVar246 = ( indirectSpecular * vec3<f32>( clamp( nodeVar245, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar246;
	nodeVar247 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar247;
	nodeVar248 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar248;
	nodeVar249 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar249;
	nodeVar250 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar250;

	// result

	output.color = nodeVar250;

	return output;

}
