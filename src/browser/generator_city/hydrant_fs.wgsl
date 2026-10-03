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
var<private> nodeVar2 : f32;
var<private> Metalness : f32;
var<private> nodeVar3 : f32;
var<private> Roughness : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
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
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> irradiance : vec3<f32>;
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
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec4<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec4<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec4<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec4<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec4<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec4<f32>;
var<private> nodeVar115 : array< vec3<f32>, 9 >;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : vec2<f32>;
var<private> nodeVar128 : vec4<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : vec2<f32>;
var<private> nodeVar136 : vec4<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : vec2<f32>;
var<private> nodeVar146 : vec4<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : vec2<f32>;
var<private> nodeVar154 : vec4<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : f32;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : f32;
var<private> nodeVar213 : f32;
var<private> nodeVar214 : f32;
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
var<private> nodeVar232 : vec3<f32>;
var<private> nodeVar233 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar236 : vec3<f32>;
var<private> nodeVar237 : f32;
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
var<private> nodeVar248 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar249 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar250 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar251 : vec3<f32>;
var<private> nodeVar252 : vec4<f32>;

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

	nodeVar1 = ( nodeVarying3 == 1.0 );

	if ( nodeVar1 ) {

		nodeVar0 = vec3<f32>( 0.2541520943200296, 0.2541520943200296, 0.21586050010324417 );

	} else {

		nodeVar2 = ( ( ( mx_fractal_noise_float( ( nodeVarying8 * vec3<f32>( 9.0 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 );
		nodeVar0 = mix( mix( vec3<f32>( 0.2746773120495699, 0.028426039499072558, 0.012983032338510335 ), vec3<f32>( 0.4793201830913402, 0.10224173307914941, 0.03954623527052923 ), ( nodeVar2 * 0.6 ) ), vec3<f32>( 0.06847816983662762, 0.025186859622305935, 0.010329823026364548 ), ( max( smoothstep( 0.28, 0.02, nodeVarying8.y ), ( smoothstep( 0.55, 0.85, nodeVar2 ) * 0.5 ) ) * 0.8 ) );

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;

	if ( nodeVar1 ) {

		nodeVar3 = 0.8;

	} else {

		nodeVar3 = 0.15;

	}

	Metalness = nodeVar3;

	if ( nodeVar1 ) {

		nodeVar4 = 0.45;

	} else {

		nodeVar5 = ( ( ( mx_fractal_noise_float( ( nodeVarying8 * vec3<f32>( 9.0 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 );
		nodeVar4 = ( ( ( nodeVar5 * 0.3 ) + ( max( smoothstep( 0.28, 0.02, nodeVarying8.y ), ( smoothstep( 0.55, 0.85, nodeVar5 ) * 0.5 ) ) * 0.25 ) ) + 0.5 );

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar6 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar4, 0.0525 ) + max( max( nodeVar6.x, nodeVar6.y ), nodeVar6.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeVar3 ) ) );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar7 = dot( normalView, positionViewDirection );
	nodeVar8 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( nodeVar7, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar8;
	nodeVar9 = ( dfg.x + dfg.y );
	nodeVar10 = ( 1.0 / nodeVar9 );
	nodeVar11 = nodeVar10;
	nodeVar12 = ( nodeVar11 - 1.0 );
	nodeVar13 = ( SpecularColorBlended * vec3<f32>( nodeVar12 ) );
	nodeVar14 = ( nodeVar13 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar14;
	nodeVar15 = ( render.nodeUniform9 - render.nodeUniform10 );
	nodeVar16 = vec4<f32>( nodeVar15, 0.0 );
	nodeVar17 = ( render.cameraViewMatrix * nodeVar16 );
	nodeVar18 = normalize( nodeVar17.xyz );
	nodeVar19 = nodeVar18;
	nodeVar20 = dot( normalView, nodeVar19 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar22 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform13 ) ) ), 1.0 ) );
	nodeVar23 = ( nodeVar22.xyz / vec3<f32>( nodeVar22.w ) );
	nodeVar24 = vec3<f32>( nodeVar23.x, ( 1.0 - nodeVar23.y ), ( nodeVar23.z + render.nodeUniform14 ) );

	if ( ( ( ( ( ( nodeVar24.x >= 0.0 ) && ( nodeVar24.x <= 1.0 ) ) && ( nodeVar24.y >= 0.0 ) ) && ( nodeVar24.y <= 1.0 ) ) && ( nodeVar24.z <= 1.0 ) ) ) {

		nodeVar25 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar26 = ( render.nodeUniform16 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ).x );
		nodeVar27 = ( nodeVar24.xy + ( vogelDiskSample( 0, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar28 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar27, nodeVar24.z );
		nodeVar29 = ( nodeVar24.xy + ( vogelDiskSample( 1, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar30 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar29, nodeVar24.z );
		nodeVar31 = ( nodeVar24.xy + ( vogelDiskSample( 2, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar32 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar31, nodeVar24.z );
		nodeVar33 = ( nodeVar24.xy + ( vogelDiskSample( 3, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar34 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar33, nodeVar24.z );
		nodeVar35 = ( nodeVar24.xy + ( vogelDiskSample( 4, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar36 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar35, nodeVar24.z );
		nodeVar21 = ( ( ( ( ( nodeVar28 + nodeVar30 ) + nodeVar32 ) + nodeVar34 ) + nodeVar36 ) * 0.2 );

	} else {

		nodeVar21 = 1.0;

	}

	nodeVar37 = mix( 1.0, nodeVar21, render.nodeUniform18 );
	nodeVar38 = ( vec3<f32>( clamp( nodeVar20, 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar37 ) ) );
	nodeVar39 = nodeVar38;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar40 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar41 = ( nodeVar39 * nodeVar40 );
	nodeVar42 = ( nodeVar19 + positionViewDirection );
	nodeVar43 = normalize( nodeVar42 );
	nodeVar44 = dot( positionViewDirection, nodeVar43 );
	nodeVar45 = clamp( nodeVar44, 0.0, 1.0 );
	nodeVar46 = exp2( ( ( ( nodeVar45 * -5.55473 ) - 6.98316 ) * nodeVar45 ) );
	nodeVar47 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar46 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar46 ) ) );
	nodeVar48 = ( vec3<f32>( 1.0 ) - nodeVar47 );
	nodeVar49 = nodeVar48;
	nodeVar50 = ( nodeVar41 * nodeVar49 );
	nodeVar51 = ( directDiffuse + nodeVar50 );
	directDiffuse = nodeVar51;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar52 = normalize( ( nodeVar19 + positionViewDirection ) );
	nodeVar53 = clamp( dot( positionViewDirection, nodeVar52 ), 0.0, 1.0 );
	nodeVar54 = exp2( ( ( ( nodeVar53 * -5.55473 ) - 6.98316 ) * nodeVar53 ) );
	nodeVar55 = ( Roughness * Roughness );
	nodeVar56 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar54 ) ) ) + vec3<f32>( ( 1.0 * nodeVar54 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar55, clamp( dot( normalView, nodeVar19 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar55, clamp( dot( normalView, nodeVar52 ), 0.0, 1.0 ) ) ) );
	nodeVar57 = ( nodeVar39 * nodeVar56 );
	nodeVar58 = ( nodeVar57 * multiScatteringCompensation );
	nodeVar59 = ( directSpecular + nodeVar58 );
	directSpecular = nodeVar59;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar60 = ( object.nodeUniform20 - object.nodeUniform21 );
	nodeVar61 = ( object.nodeUniform22 - vec3<f32>( 1.0 ) );
	nodeVar62 = ( nodeVar60 / nodeVar61 );
	nodeVar63 = ( normalWorld * nodeVar62 );
	nodeVar64 = ( nodeVar63 * vec3<f32>( 0.5 ) );
	nodeVar65 = ( v_positionWorld + nodeVar64 );
	nodeVar66 = ( nodeVar65 - object.nodeUniform21 );
	nodeVar67 = ( nodeVar66 / nodeVar60 );
	nodeVar68 = ( clamp( nodeVar67, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar61 );
	nodeVar69 = ( nodeVar68 / object.nodeUniform22 );
	nodeVar70 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform22 );
	nodeVar71 = ( nodeVar69 + nodeVar70 );
	nodeVar72 = ( nodeVar71.z * object.nodeUniform22.z );
	nodeVar73 = ( nodeVar72 + 1.0 );
	nodeVar74 = ( object.nodeUniform22.z + 2.0 );
	nodeVar75 = ( nodeVar74 * 0.0 );
	nodeVar76 = ( nodeVar73 + nodeVar75 );
	nodeVar77 = ( nodeVar74 * 7.0 );
	nodeVar78 = ( nodeVar76 / nodeVar77 );
	nodeVar79 = vec3<f32>( nodeVar71.xy, nodeVar78 );
	nodeVar80 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar79 );
	nodeVar81 = ( nodeVar74 * 1.0 );
	nodeVar82 = ( nodeVar73 + nodeVar81 );
	nodeVar83 = ( nodeVar82 / nodeVar77 );
	nodeVar84 = vec3<f32>( nodeVar71.xy, nodeVar83 );
	nodeVar85 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar84 );
	nodeVar86 = vec3<f32>( nodeVar80.w, nodeVar85.xy );
	nodeVar87 = ( nodeVar74 * 2.0 );
	nodeVar88 = ( nodeVar73 + nodeVar87 );
	nodeVar89 = ( nodeVar88 / nodeVar77 );
	nodeVar90 = vec3<f32>( nodeVar71.xy, nodeVar89 );
	nodeVar91 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar90 );
	nodeVar92 = vec3<f32>( nodeVar85.zw, nodeVar91.x );
	nodeVar93 = ( nodeVar74 * 3.0 );
	nodeVar94 = ( nodeVar73 + nodeVar93 );
	nodeVar95 = ( nodeVar94 / nodeVar77 );
	nodeVar96 = vec3<f32>( nodeVar71.xy, nodeVar95 );
	nodeVar97 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar96 );
	nodeVar98 = ( nodeVar74 * 4.0 );
	nodeVar99 = ( nodeVar73 + nodeVar98 );
	nodeVar100 = ( nodeVar99 / nodeVar77 );
	nodeVar101 = vec3<f32>( nodeVar71.xy, nodeVar100 );
	nodeVar102 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar101 );
	nodeVar103 = vec3<f32>( nodeVar97.w, nodeVar102.xy );
	nodeVar104 = ( nodeVar74 * 5.0 );
	nodeVar105 = ( nodeVar73 + nodeVar104 );
	nodeVar106 = ( nodeVar105 / nodeVar77 );
	nodeVar107 = vec3<f32>( nodeVar71.xy, nodeVar106 );
	nodeVar108 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar107 );
	nodeVar109 = vec3<f32>( nodeVar102.zw, nodeVar108.x );
	nodeVar110 = ( nodeVar74 * 6.0 );
	nodeVar111 = ( nodeVar73 + nodeVar110 );
	nodeVar112 = ( nodeVar111 / nodeVar77 );
	nodeVar113 = vec3<f32>( nodeVar71.xy, nodeVar112 );
	nodeVar114 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeVar113 );
	nodeVar115 = array< vec3<f32>, 9 >( nodeVar80.xyz, nodeVar86, nodeVar92, nodeVar91.yzw, nodeVar97.xyz, nodeVar103, nodeVar109, nodeVar108.yzw, nodeVar114.xyz );
	nodeVar116 = ( ( ( ( ( ( ( ( ( nodeVar115[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar115[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar115[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar115[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar115[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar115[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar115[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar115[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar115[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar117 = max( nodeVar116, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar118 = ( nodeVar117 * vec3<f32>( object.nodeUniform23 ) );
	nodeVar119 = ( irradiance + nodeVar118 );
	irradiance = nodeVar119;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar120 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform24 );
	nodeVar121 = floor( nodeVar120 );
	nodeVar122 = nodeVar121;
	nodeVar123 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar124 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar123.x, ( - nodeVar123.y ), nodeVar123.z ), 1.0 ) ).xyz );
	nodeVar125 = max( ( 4.0 - nodeVar122 ), 0.0 );
	nodeVar122 = max( nodeVar122, 4.0 );
	nodeVar126 = exp2( nodeVar122 );
	nodeVar127 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar123.x, ( - nodeVar123.y ), nodeVar123.z ), 1.0 ) ).xyz, nodeVar124 ) * vec2<f32>( ( nodeVar126 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar124 > 2.0 ) ) {

		nodeVar127.y = ( nodeVar127.y + nodeVar126 );
		nodeVar124 = ( nodeVar124 - 3.0 );
		

	}

	nodeVar127.x = ( nodeVar127.x + ( nodeVar124 * nodeVar126 ) );
	nodeVar127.x = ( nodeVar127.x + ( nodeVar125 * ( 3.0 * 16.0 ) ) );
	nodeVar127.y = ( nodeVar127.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar126 ) ) );
	nodeVar127.x = ( nodeVar127.x * object.nodeUniform27 );
	nodeVar127.y = ( nodeVar127.y * object.nodeUniform28 );
	nodeVar128 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar127, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar129 = nodeVar128.xyz;
	nodeVar130 = fract( nodeVar120 );

	if ( ( nodeVar130 != 0.0 ) ) {

		nodeVar131 = ( nodeVar121 + 1.0 );
		nodeVar132 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar123.x, ( - nodeVar123.y ), nodeVar123.z ), 1.0 ) ).xyz );
		nodeVar133 = max( ( 4.0 - nodeVar131 ), 0.0 );
		nodeVar131 = max( nodeVar131, 4.0 );
		nodeVar134 = exp2( nodeVar131 );
		nodeVar135 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar123.x, ( - nodeVar123.y ), nodeVar123.z ), 1.0 ) ).xyz, nodeVar132 ) * vec2<f32>( ( nodeVar134 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar132 > 2.0 ) ) {

			nodeVar135.y = ( nodeVar135.y + nodeVar134 );
			nodeVar132 = ( nodeVar132 - 3.0 );
			

		}

		nodeVar135.x = ( nodeVar135.x + ( nodeVar132 * nodeVar134 ) );
		nodeVar135.x = ( nodeVar135.x + ( nodeVar133 * ( 3.0 * 16.0 ) ) );
		nodeVar135.y = ( nodeVar135.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar134 ) ) );
		nodeVar135.x = ( nodeVar135.x * object.nodeUniform27 );
		nodeVar135.y = ( nodeVar135.y * object.nodeUniform28 );
		nodeVar136 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar135, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar137 = nodeVar136.xyz;
		nodeVar129 = mix( nodeVar129, nodeVar137, nodeVar130 );
		

	}

	nodeVar138 = ( radiance + ( nodeVar129 * vec3<f32>( object.nodeUniform30 ) ) );
	radiance = nodeVar138;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar139 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform24 );
	nodeVar140 = floor( nodeVar139 );
	nodeVar141 = nodeVar140;
	nodeVar142 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar143 = max( ( 4.0 - nodeVar141 ), 0.0 );
	nodeVar141 = max( nodeVar141, 4.0 );
	nodeVar144 = exp2( nodeVar141 );
	nodeVar145 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar142 ) * vec2<f32>( ( nodeVar144 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar142 > 2.0 ) ) {

		nodeVar145.y = ( nodeVar145.y + nodeVar144 );
		nodeVar142 = ( nodeVar142 - 3.0 );
		

	}

	nodeVar145.x = ( nodeVar145.x + ( nodeVar142 * nodeVar144 ) );
	nodeVar145.x = ( nodeVar145.x + ( nodeVar143 * ( 3.0 * 16.0 ) ) );
	nodeVar145.y = ( nodeVar145.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar144 ) ) );
	nodeVar145.x = ( nodeVar145.x * object.nodeUniform27 );
	nodeVar145.y = ( nodeVar145.y * object.nodeUniform28 );
	nodeVar146 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar145, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar147 = nodeVar146.xyz;
	nodeVar148 = fract( nodeVar139 );

	if ( ( nodeVar148 != 0.0 ) ) {

		nodeVar149 = ( nodeVar140 + 1.0 );
		nodeVar150 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar151 = max( ( 4.0 - nodeVar149 ), 0.0 );
		nodeVar149 = max( nodeVar149, 4.0 );
		nodeVar152 = exp2( nodeVar149 );
		nodeVar153 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar150 ) * vec2<f32>( ( nodeVar152 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar150 > 2.0 ) ) {

			nodeVar153.y = ( nodeVar153.y + nodeVar152 );
			nodeVar150 = ( nodeVar150 - 3.0 );
			

		}

		nodeVar153.x = ( nodeVar153.x + ( nodeVar150 * nodeVar152 ) );
		nodeVar153.x = ( nodeVar153.x + ( nodeVar151 * ( 3.0 * 16.0 ) ) );
		nodeVar153.y = ( nodeVar153.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar152 ) ) );
		nodeVar153.x = ( nodeVar153.x * object.nodeUniform27 );
		nodeVar153.y = ( nodeVar153.y * object.nodeUniform28 );
		nodeVar154 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar153, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar155 = nodeVar154.xyz;
		nodeVar147 = mix( nodeVar147, nodeVar155, nodeVar148 );
		

	}

	nodeVar156 = ( iblIrradiance + ( ( nodeVar147 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform30 ) ) );
	iblIrradiance = nodeVar156;
	nodeVar157 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar158 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar159 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar160 = ( SpecularF90 * dfg.y );
	nodeVar161 = ( nodeVar159 + vec3<f32>( nodeVar160 ) );
	nodeVar162 = ( nodeVar157 + nodeVar161 );
	nodeVar157 = nodeVar162;
	nodeVar163 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar164 = nodeVar163;
	nodeVar165 = ( nodeVar164 * vec3<f32>( 0.047619 ) );
	nodeVar166 = ( SpecularColor + nodeVar165 );
	nodeVar167 = ( nodeVar161 * nodeVar166 );
	nodeVar168 = ( dfg.x + dfg.y );
	nodeVar169 = ( 1.0 - nodeVar168 );
	nodeVar170 = nodeVar169;
	nodeVar171 = ( vec3<f32>( nodeVar170 ) * nodeVar166 );
	nodeVar172 = ( vec3<f32>( 1.0 ) - nodeVar171 );
	nodeVar173 = nodeVar172;
	nodeVar174 = ( nodeVar167 / nodeVar173 );
	nodeVar175 = ( nodeVar174 * vec3<f32>( nodeVar170 ) );
	nodeVar176 = ( nodeVar158 + nodeVar175 );
	nodeVar158 = nodeVar176;
	nodeVar177 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar178 = ( irradiance * nodeVar177 );
	nodeVar179 = ( nodeVar157 + nodeVar158 );
	nodeVar180 = ( vec3<f32>( 1.0 ) - nodeVar179 );
	nodeVar181 = nodeVar180;
	nodeVar182 = ( nodeVar178 * nodeVar181 );
	nodeVar183 = nodeVar182;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar184 = ( indirectDiffuse + nodeVar183 );
	indirectDiffuse = nodeVar184;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar185 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar186 = ( SpecularF90 * dfg.y );
	nodeVar187 = ( nodeVar185 + vec3<f32>( nodeVar186 ) );
	nodeVar188 = ( singleScatteringDielectric + nodeVar187 );
	singleScatteringDielectric = nodeVar188;
	nodeVar189 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar190 = nodeVar189;
	nodeVar191 = ( nodeVar190 * vec3<f32>( 0.047619 ) );
	nodeVar192 = ( SpecularColor + nodeVar191 );
	nodeVar193 = ( nodeVar187 * nodeVar192 );
	nodeVar194 = ( dfg.x + dfg.y );
	nodeVar195 = ( 1.0 - nodeVar194 );
	nodeVar196 = nodeVar195;
	nodeVar197 = ( vec3<f32>( nodeVar196 ) * nodeVar192 );
	nodeVar198 = ( vec3<f32>( 1.0 ) - nodeVar197 );
	nodeVar199 = nodeVar198;
	nodeVar200 = ( nodeVar193 / nodeVar199 );
	nodeVar201 = ( nodeVar200 * vec3<f32>( nodeVar196 ) );
	nodeVar202 = ( multiScatteringDielectric + nodeVar201 );
	multiScatteringDielectric = nodeVar202;
	nodeVar203 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar204 = ( SpecularF90 * dfg.y );
	nodeVar205 = ( nodeVar203 + vec3<f32>( nodeVar204 ) );
	nodeVar206 = ( singleScatteringMetallic + nodeVar205 );
	singleScatteringMetallic = nodeVar206;
	nodeVar207 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar208 = nodeVar207;
	nodeVar209 = ( nodeVar208 * vec3<f32>( 0.047619 ) );
	nodeVar210 = ( DiffuseColor.xyz + nodeVar209 );
	nodeVar211 = ( nodeVar205 * nodeVar210 );
	nodeVar212 = ( dfg.x + dfg.y );
	nodeVar213 = ( 1.0 - nodeVar212 );
	nodeVar214 = nodeVar213;
	nodeVar215 = ( vec3<f32>( nodeVar214 ) * nodeVar210 );
	nodeVar216 = ( vec3<f32>( 1.0 ) - nodeVar215 );
	nodeVar217 = nodeVar216;
	nodeVar218 = ( nodeVar211 / nodeVar217 );
	nodeVar219 = ( nodeVar218 * vec3<f32>( nodeVar214 ) );
	nodeVar220 = ( multiScatteringMetallic + nodeVar219 );
	multiScatteringMetallic = nodeVar220;
	nodeVar221 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar222 = ( radiance * nodeVar221 );
	nodeVar223 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar224 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar225 = ( nodeVar223 * nodeVar224 );
	nodeVar226 = ( nodeVar222 + nodeVar225 );
	nodeVar227 = nodeVar226;
	nodeVar228 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar229 = ( vec3<f32>( 1.0 ) - nodeVar228 );
	nodeVar230 = nodeVar229;
	nodeVar231 = ( DiffuseContribution * nodeVar230 );
	nodeVar232 = ( nodeVar231 * nodeVar224 );
	nodeVar233 = nodeVar232;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar234 = ( indirectSpecular + nodeVar227 );
	indirectSpecular = nodeVar234;
	nodeVar235 = ( indirectDiffuse + nodeVar233 );
	indirectDiffuse = nodeVar235;
	ambientOcclusion = 1.0;
	nodeVar236 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar236;
	nodeVar237 = dot( normalView, positionViewDirection );
	nodeVar238 = ( clamp( nodeVar237, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar239 = ( Roughness * -16.0 );
	nodeVar240 = ( 1.0 - nodeVar239 );
	nodeVar241 = nodeVar240;
	nodeVar242 = ( - nodeVar241 );
	nodeVar243 = exp2( nodeVar242 );
	nodeVar244 = pow( nodeVar238, nodeVar243 );
	nodeVar245 = ( 1.0 - nodeVar244 );
	nodeVar246 = nodeVar245;
	nodeVar247 = ( ambientOcclusion - nodeVar246 );
	nodeVar248 = ( indirectSpecular * vec3<f32>( clamp( nodeVar247, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar248;
	nodeVar249 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar249;
	nodeVar250 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar250;
	nodeVar251 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar251;
	nodeVar252 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar252;

	// result

	output.color = nodeVar252;

	return output;

}
