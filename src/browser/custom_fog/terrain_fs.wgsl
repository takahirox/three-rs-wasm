// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform18_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform18 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform27_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform27 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
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
	cameraPosition : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform21 : f32,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_nodeVar4 : vec3<f32>;
var<private> NORMAL_nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> NORMAL_normalWorld : vec3<f32>;
var<private> NORMAL_nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> positionViewDirection : vec3<f32>;
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
var<private> nodeVar206 : f32;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : vec4<f32>;

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


fn mx_hash_int_1 ( x : i32, y : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;

	nodeVar0 = y;
	nodeVar1 = x;
	nodeVar2 = 2u;
	nodeVar3 = 0u;
	nodeVar4 = 0u;
	nodeVar5 = 0u;
	nodeVar5 = ( ( 3735928559u + ( nodeVar2 << 2u ) ) + 13u );
	nodeVar4 = nodeVar5;
	nodeVar3 = nodeVar4;
	nodeVar3 = ( nodeVar3 + u32( nodeVar1 ) );
	nodeVar4 = ( nodeVar4 + u32( nodeVar0 ) );

	return mx_bjfinal( nodeVar3, nodeVar4, nodeVar5 );

}


fn mx_gradient_float_0 ( hash : u32, x : f32, y : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;

	nodeVar0 = y;
	nodeVar1 = x;
	nodeVar2 = hash;
	nodeVar3 = ( nodeVar2 & 7u );
	nodeVar4 = mx_select( ( nodeVar3 < 4u ), nodeVar1, nodeVar0 );
	nodeVar5 = ( 2.0 * mx_select( ( nodeVar3 < 4u ), nodeVar0, nodeVar1 ) );

	return ( mx_negate_if( nodeVar4, bool( ( nodeVar3 & 1u ) ) ) + mx_negate_if( nodeVar5, bool( ( nodeVar3 & 2u ) ) ) );

}


fn mx_bilerp_0 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, s : f32, t : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;

	nodeVar0 = t;
	nodeVar1 = s;
	nodeVar2 = v3;
	nodeVar3 = v2;
	nodeVar4 = v1;
	nodeVar5 = v0;
	nodeVar6 = ( 1.0 - nodeVar1 );

	return ( ( ( 1.0 - nodeVar0 ) * ( ( nodeVar5 * nodeVar6 ) + ( nodeVar4 * nodeVar1 ) ) ) + ( nodeVar0 * ( ( nodeVar3 * nodeVar6 ) + ( nodeVar2 * nodeVar1 ) ) ) );

}


fn mx_gradient_scale2d_0 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.6616 * nodeVar0 );

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


fn tri ( x : f32 ) -> f32 {

	


	return abs( ( fract( x ) - 0.5 ) );

}


fn tri3 ( p : vec3<f32> ) -> vec3<f32> {

	


	return vec3<f32>( tri( ( p.z + tri( ( p.y * 1.0 ) ) ) ), tri( ( p.z + tri( ( p.x * 1.0 ) ) ) ), tri( ( p.y + tri( ( p.x * 1.0 ) ) ) ) );

}


fn mx_perlin_noise_float_0 ( p : vec2<f32> ) -> f32 {

	var nodeVar0 : vec2<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;

	nodeVar0 = p;
	nodeVar1 = 0;
	nodeVar2 = 0;
	nodeVar3 = nodeVar0.x;
	nodeVar1 = mx_floor( nodeVar3 );
	nodeVar4 = ( nodeVar3 - f32( nodeVar1 ) );
	nodeVar5 = nodeVar0.y;
	nodeVar2 = mx_floor( nodeVar5 );
	nodeVar6 = ( nodeVar5 - f32( nodeVar2 ) );
	nodeVar7 = mx_fade( nodeVar4 );
	nodeVar8 = mx_fade( nodeVar6 );
	nodeVar9 = mx_bilerp_0( mx_gradient_float_0( mx_hash_int_1( nodeVar1, nodeVar2 ), nodeVar4, nodeVar6 ), mx_gradient_float_0( mx_hash_int_1( ( nodeVar1 + 1 ), nodeVar2 ), ( nodeVar4 - 1.0 ), nodeVar6 ), mx_gradient_float_0( mx_hash_int_1( nodeVar1, ( nodeVar2 + 1 ) ), nodeVar4, ( nodeVar6 - 1.0 ) ), mx_gradient_float_0( mx_hash_int_1( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ) ), ( nodeVar4 - 1.0 ), ( nodeVar6 - 1.0 ) ), nodeVar7, nodeVar8 );

	return mx_gradient_scale2d_0( nodeVar9 );

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


fn triNoise3D ( position : vec3<f32>, speed : f32, time : f32 ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : vec3<f32>;
	var nodeVar5 : f32;

	nodeVar0 = position;
	nodeVar1 = 1.4;
	nodeVar2 = 0.0;
	nodeVar3 = nodeVar0;

	for ( var i : f32 = 0.0; i <= 3.0; i += 1. ) {

		nodeVar4 = tri3( ( nodeVar3 * vec3<f32>( 2.0 ) ) );
		nodeVar0 = ( nodeVar0 + ( nodeVar4 + vec3<f32>( ( time * ( 0.1 * speed ) ) ) ) );
		nodeVar3 = ( nodeVar3 * vec3<f32>( 1.8 ) );
		nodeVar1 = ( nodeVar1 * 1.5 );
		nodeVar0 = ( nodeVar0 * vec3<f32>( 1.2 ) );
		nodeVar5 = tri( ( nodeVar0.z + tri( ( nodeVar0.x + tri( nodeVar0.y ) ) ) ) );
		nodeVar2 = ( nodeVar2 + ( nodeVar5 / nodeVar1 ) );
		nodeVar3 = ( nodeVar3 + vec3<f32>( 0.14 ) );

	}


	return nodeVar2;

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( ( mx_perlin_noise_float_0( ( v_positionWorld.xz * vec2<f32>( 0.012 ) ) ) * 1.0 ) + 0.0 );
	nodeVar1 = clamp( ( ( v_positionWorld.y - object.nodeUniform1 ) / ( object.nodeUniform2 - object.nodeUniform1 ) ), 0.0, 1.0 );
	nodeVar2 = dpdx( v_positionView );
	nodeVar3 = - dpdy( v_positionView );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_nodeVar4 = cross( nodeVar3, NORMAL_normalView );
	NORMAL_nodeVar5 = dot( nodeVar2, NORMAL_nodeVar4 );
	nodeVar6 = 0.0;
	nodeVar7 = distance( v_positionWorld, render.cameraPosition );
	nodeVar8 = smoothstep( 420.0, 60.0, nodeVar7 );

	if ( ( nodeVar8 > 0.01 ) ) {

		nodeVar6 = ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 0.6 ) ) ) * 1.0 ) + 0.0 );
		nodeVar9 = smoothstep( 180.0, 40.0, nodeVar7 );

		if ( ( nodeVar9 > 0.01 ) ) {

			nodeVar6 = ( nodeVar6 + ( ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 1.7 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) * nodeVar9 ) );
			

		}

		

	}

	NORMAL_normalWorld = normalize( ( vec4<f32>( NORMAL_normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	NORMAL_nodeVar10 = clamp( NORMAL_normalWorld.y, 0.0, 1.0 );
	nodeVar11 = ( 1.0 - NORMAL_nodeVar10 );
	nodeVar12 = ( ( mx_perlin_noise_float_0( ( v_positionWorld.xz * vec2<f32>( 0.05 ) ) ) * 1.0 ) + 0.0 );
	nodeVar13 = ( ( mx_perlin_noise_float_0( ( v_positionWorld.xz * vec2<f32>( 0.18 ) ) ) * 1.0 ) + 0.0 );
	nodeVar14 = ( smoothstep( 0.56, 0.78, ( ( nodeVar1 + ( nodeVar12 * 0.08 ) ) + ( nodeVar13 * 0.05 ) ) ) * smoothstep( 0.45, 0.75, NORMAL_nodeVar10 ) );
	nodeVar15 = ( ( vec3<f32>( sign( NORMAL_nodeVar5 ) ) * ( ( vec3<f32>( dpdx( nodeVar6 ) ) * NORMAL_nodeVar4 ) + ( vec3<f32>( - dpdy( nodeVar6 ) ) * cross( NORMAL_normalView, nodeVar2 ) ) ) ) * vec3<f32>( ( ( mix( mix( 0.25, 0.55, nodeVar11 ), 0.04, nodeVar14 ) * nodeVar8 ) * 0.25 ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeVar5 ) ) * NORMAL_normalView ) - nodeVar15 ) );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar16 = clamp( normalWorld.y, 0.0, 1.0 );
	nodeVar17 = ( ( mix( vec3<f32>( 0.16513219449147767, 0.15592646369776456, 0.13843161502267545 ), vec3<f32>( 0.1499597898006365, 0.171441100722554, 0.09084171117479915 ), ( ( ( smoothstep( 0.45, 0.72, nodeVar13 ) * smoothstep( 0.62, 0.32, nodeVar11 ) ) * smoothstep( 0.66, 0.34, nodeVar1 ) ) * 0.45 ) ) * vec3<f32>( ( ( ( ( ( ( sin( ( ( ( ( ( v_positionWorld.y * 0.5 ) + ( v_positionWorld.x * 0.08 ) ) + ( v_positionWorld.z * 0.05 ) ) + ( nodeVar12 * 3.0 ) ) + ( nodeVar0 * 4.0 ) ) ) * 0.6 ) + ( sin( ( ( v_positionWorld.y * 1.4 ) + ( nodeVar13 * 2.0 ) ) ) * 0.4 ) ) * 0.5 ) + 0.5 ) * 0.36 ) + 0.8 ) ) ) * vec3<f32>( ( ( nodeVar13 * 0.18 ) + 1.0 ) ) );
	nodeVar18 = smoothstep( 180.0, 820.0, nodeVar7 );
	DiffuseColor = vec4<f32>( mix( max( mix( vec3<f32>( dot( ( ( ( mix( mix( mix( mix( mix( mix( vec3<f32>( 0.15592646369776456, 0.16826940017946088, 0.08650046202808521 ), vec3<f32>( 0.2541520943200296, 0.23455058215026167, 0.08021982030622662 ), ( smoothstep( 0.15, 0.75, nodeVar0 ) * smoothstep( 0.22, 0.5, nodeVar1 ) ) ), vec3<f32>( 0.040915196900556984, 0.05126945836711539, 0.028426039499072558 ), ( ( smoothstep( 0.16, 0.34, nodeVar1 ) * smoothstep( 0.5, 0.72, nodeVar16 ) ) * 0.75 ) ), nodeVar17, smoothstep( 0.46, 0.64, ( nodeVar1 + ( nodeVar12 * 0.06 ) ) ) ), nodeVar17, smoothstep( 0.34, 0.62, nodeVar11 ) ), vec3<f32>( 0.22696587349938613, 0.19461783043107173, 0.158960835050774 ), ( ( ( smoothstep( 0.42, 0.7, nodeVar11 ) * smoothstep( 0.35, 0.7, nodeVar16 ) ) * ( ( nodeVar12 * 0.5 ) + 0.5 ) ) * 0.5 ) ), mix( vec3<f32>( 0.8148465722120952, 0.8387990117372213, 0.8713671191959567 ), vec3<f32>( 0.6038273388475408, 0.6724431569510133, 0.76052450467022 ), ( smoothstep( 0.2, 0.7, nodeVar13 ) * 0.6 ) ), nodeVar14 ) * vec3<f32>( ( 1.0 - ( ( smoothstep( 0.24, 0.06, nodeVar1 ) * nodeVar16 ) * 0.32 ) ) ) ) * vec3<f32>( ( ( ( ( nodeVar0 * 0.5 ) + 0.5 ) * 0.3 ) + 0.84 ) ) ) * vec3<f32>( ( ( ( ( nodeVar13 * 0.5 ) + 0.5 ) * 0.12 ) + 0.94 ) ) ), vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), ( ( ( mix( mix( mix( mix( mix( mix( vec3<f32>( 0.15592646369776456, 0.16826940017946088, 0.08650046202808521 ), vec3<f32>( 0.2541520943200296, 0.23455058215026167, 0.08021982030622662 ), ( smoothstep( 0.15, 0.75, nodeVar0 ) * smoothstep( 0.22, 0.5, nodeVar1 ) ) ), vec3<f32>( 0.040915196900556984, 0.05126945836711539, 0.028426039499072558 ), ( ( smoothstep( 0.16, 0.34, nodeVar1 ) * smoothstep( 0.5, 0.72, nodeVar16 ) ) * 0.75 ) ), nodeVar17, smoothstep( 0.46, 0.64, ( nodeVar1 + ( nodeVar12 * 0.06 ) ) ) ), nodeVar17, smoothstep( 0.34, 0.62, nodeVar11 ) ), vec3<f32>( 0.22696587349938613, 0.19461783043107173, 0.158960835050774 ), ( ( ( smoothstep( 0.42, 0.7, nodeVar11 ) * smoothstep( 0.35, 0.7, nodeVar16 ) ) * ( ( nodeVar12 * 0.5 ) + 0.5 ) ) * 0.5 ) ), mix( vec3<f32>( 0.8148465722120952, 0.8387990117372213, 0.8713671191959567 ), vec3<f32>( 0.6038273388475408, 0.6724431569510133, 0.76052450467022 ), ( smoothstep( 0.2, 0.7, nodeVar13 ) * 0.6 ) ), nodeVar14 ) * vec3<f32>( ( 1.0 - ( ( smoothstep( 0.24, 0.06, nodeVar1 ) * nodeVar16 ) * 0.32 ) ) ) ) * vec3<f32>( ( ( ( ( nodeVar0 * 0.5 ) + 0.5 ) * 0.3 ) + 0.84 ) ) ) * vec3<f32>( ( ( ( ( nodeVar13 * 0.5 ) + 0.5 ) * 0.12 ) + 0.94 ) ) ), ( ( ( 1.0 - nodeVar18 ) * 0.5 ) + 0.5 ) ), vec3<f32>( 0.0 ) ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4910208498384856 ), ( nodeVar18 * 0.62 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform8;
	nodeVar19 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( mix( 0.95, 0.72, nodeVar14 ), 0.0525 ) + max( max( nodeVar19.x, nodeVar19.y ), nodeVar19.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform8 ) ) );
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar20 = dot( normalView, positionViewDirection );
	nodeVar21 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( nodeVar20, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar21;
	nodeVar22 = ( dfg.x + dfg.y );
	nodeVar23 = ( 1.0 / nodeVar22 );
	nodeVar24 = nodeVar23;
	nodeVar25 = ( nodeVar24 - 1.0 );
	nodeVar26 = ( SpecularColorBlended * vec3<f32>( nodeVar25 ) );
	nodeVar27 = ( nodeVar26 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar27;
	nodeVar28 = ( render.nodeUniform12 - render.nodeUniform13 );
	nodeVar29 = vec4<f32>( nodeVar28, 0.0 );
	nodeVar30 = ( render.cameraViewMatrix * nodeVar29 );
	nodeVar31 = normalize( nodeVar30.xyz );
	nodeVar32 = nodeVar31;
	nodeVar33 = dot( normalView, nodeVar32 );
	shadowPositionWorld = v_positionWorld;
	nodeVar35 = ( render.nodeUniform15 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform16 ) ) ), 1.0 ) );
	nodeVar36 = ( nodeVar35.xyz / vec3<f32>( nodeVar35.w ) );
	nodeVar37 = vec3<f32>( nodeVar36.x, ( 1.0 - nodeVar36.y ), ( nodeVar36.z + render.nodeUniform17 ) );

	if ( ( ( ( ( ( nodeVar37.x >= 0.0 ) && ( nodeVar37.x <= 1.0 ) ) && ( nodeVar37.y >= 0.0 ) ) && ( nodeVar37.y <= 1.0 ) ) && ( nodeVar37.z <= 1.0 ) ) ) {

		nodeVar38 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar39 = ( render.nodeUniform19 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform20 ).x );
		nodeVar40 = ( nodeVar37.xy + ( vogelDiskSample( 0, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar41 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar40, nodeVar37.z );
		nodeVar42 = ( nodeVar37.xy + ( vogelDiskSample( 1, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar43 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar42, nodeVar37.z );
		nodeVar44 = ( nodeVar37.xy + ( vogelDiskSample( 2, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar45 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar44, nodeVar37.z );
		nodeVar46 = ( nodeVar37.xy + ( vogelDiskSample( 3, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar47 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar46, nodeVar37.z );
		nodeVar48 = ( nodeVar37.xy + ( vogelDiskSample( 4, 5, nodeVar38 ) * vec2<f32>( nodeVar39 ) ) );
		nodeVar49 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar48, nodeVar37.z );
		nodeVar34 = ( ( ( ( ( nodeVar41 + nodeVar43 ) + nodeVar45 ) + nodeVar47 ) + nodeVar49 ) * 0.2 );

	} else {

		nodeVar34 = 1.0;

	}

	nodeVar50 = mix( 1.0, nodeVar34, render.nodeUniform21 );
	nodeVar51 = ( render.nodeUniform14 * vec3<f32>( nodeVar50 ) );
	nodeVar52 = ( vec3<f32>( clamp( nodeVar33, 0.0, 1.0 ) ) * nodeVar51 );
	nodeVar53 = nodeVar52;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar54 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar55 = ( nodeVar53 * nodeVar54 );
	nodeVar56 = ( nodeVar32 + positionViewDirection );
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
	nodeVar66 = normalize( ( nodeVar32 + positionViewDirection ) );
	nodeVar67 = clamp( dot( positionViewDirection, nodeVar66 ), 0.0, 1.0 );
	nodeVar68 = exp2( ( ( ( nodeVar67 * -5.55473 ) - 6.98316 ) * nodeVar67 ) );
	nodeVar69 = ( Roughness * Roughness );
	nodeVar70 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar68 ) ) ) + vec3<f32>( ( 1.0 * nodeVar68 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar69, clamp( dot( normalView, nodeVar32 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar69, clamp( dot( normalView, nodeVar66 ), 0.0, 1.0 ) ) ) );
	nodeVar71 = ( nodeVar53 * nodeVar70 );
	nodeVar72 = ( nodeVar71 * multiScatteringCompensation );
	nodeVar73 = ( directSpecular + nodeVar72 );
	directSpecular = nodeVar73;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar74 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform22 );
	nodeVar75 = floor( nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar78 = getFace( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( nodeVar77.x, ( - nodeVar77.y ), nodeVar77.z ), 1.0 ) ).xyz );
	nodeVar79 = max( ( 4.0 - nodeVar76 ), 0.0 );
	nodeVar76 = max( nodeVar76, 4.0 );
	nodeVar80 = exp2( nodeVar76 );
	nodeVar81 = ( ( getUV( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( nodeVar77.x, ( - nodeVar77.y ), nodeVar77.z ), 1.0 ) ).xyz, nodeVar78 ) * vec2<f32>( ( nodeVar80 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar78 > 2.0 ) ) {

		nodeVar81.y = ( nodeVar81.y + nodeVar80 );
		nodeVar78 = ( nodeVar78 - 3.0 );
		

	}

	nodeVar81.x = ( nodeVar81.x + ( nodeVar78 * nodeVar80 ) );
	nodeVar81.x = ( nodeVar81.x + ( nodeVar79 * ( 3.0 * 16.0 ) ) );
	nodeVar81.y = ( nodeVar81.y + ( 4.0 * ( exp2( object.nodeUniform22 ) - nodeVar80 ) ) );
	nodeVar81.x = ( nodeVar81.x * object.nodeUniform25 );
	nodeVar81.y = ( nodeVar81.y * object.nodeUniform26 );
	nodeVar82 = textureSampleGrad( nodeUniform27, nodeUniform27_sampler, nodeVar81, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar83 = nodeVar82.xyz;
	nodeVar84 = fract( nodeVar74 );

	if ( ( nodeVar84 != 0.0 ) ) {

		nodeVar85 = ( nodeVar75 + 1.0 );
		nodeVar86 = getFace( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( nodeVar77.x, ( - nodeVar77.y ), nodeVar77.z ), 1.0 ) ).xyz );
		nodeVar87 = max( ( 4.0 - nodeVar85 ), 0.0 );
		nodeVar85 = max( nodeVar85, 4.0 );
		nodeVar88 = exp2( nodeVar85 );
		nodeVar89 = ( ( getUV( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( nodeVar77.x, ( - nodeVar77.y ), nodeVar77.z ), 1.0 ) ).xyz, nodeVar86 ) * vec2<f32>( ( nodeVar88 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar86 > 2.0 ) ) {

			nodeVar89.y = ( nodeVar89.y + nodeVar88 );
			nodeVar86 = ( nodeVar86 - 3.0 );
			

		}

		nodeVar89.x = ( nodeVar89.x + ( nodeVar86 * nodeVar88 ) );
		nodeVar89.x = ( nodeVar89.x + ( nodeVar87 * ( 3.0 * 16.0 ) ) );
		nodeVar89.y = ( nodeVar89.y + ( 4.0 * ( exp2( object.nodeUniform22 ) - nodeVar88 ) ) );
		nodeVar89.x = ( nodeVar89.x * object.nodeUniform25 );
		nodeVar89.y = ( nodeVar89.y * object.nodeUniform26 );
		nodeVar90 = textureSampleGrad( nodeUniform27, nodeUniform27_sampler, nodeVar89, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar91 = nodeVar90.xyz;
		nodeVar83 = mix( nodeVar83, nodeVar91, nodeVar84 );
		

	}

	nodeVar92 = ( radiance + ( nodeVar83 * vec3<f32>( object.nodeUniform28 ) ) );
	radiance = nodeVar92;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar93 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform22 );
	nodeVar94 = floor( nodeVar93 );
	nodeVar95 = nodeVar94;
	nodeVar96 = getFace( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar97 = max( ( 4.0 - nodeVar95 ), 0.0 );
	nodeVar95 = max( nodeVar95, 4.0 );
	nodeVar98 = exp2( nodeVar95 );
	nodeVar99 = ( ( getUV( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar96 ) * vec2<f32>( ( nodeVar98 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar96 > 2.0 ) ) {

		nodeVar99.y = ( nodeVar99.y + nodeVar98 );
		nodeVar96 = ( nodeVar96 - 3.0 );
		

	}

	nodeVar99.x = ( nodeVar99.x + ( nodeVar96 * nodeVar98 ) );
	nodeVar99.x = ( nodeVar99.x + ( nodeVar97 * ( 3.0 * 16.0 ) ) );
	nodeVar99.y = ( nodeVar99.y + ( 4.0 * ( exp2( object.nodeUniform22 ) - nodeVar98 ) ) );
	nodeVar99.x = ( nodeVar99.x * object.nodeUniform25 );
	nodeVar99.y = ( nodeVar99.y * object.nodeUniform26 );
	nodeVar100 = textureSampleGrad( nodeUniform27, nodeUniform27_sampler, nodeVar99, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar101 = nodeVar100.xyz;
	nodeVar102 = fract( nodeVar93 );

	if ( ( nodeVar102 != 0.0 ) ) {

		nodeVar103 = ( nodeVar94 + 1.0 );
		nodeVar104 = getFace( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar105 = max( ( 4.0 - nodeVar103 ), 0.0 );
		nodeVar103 = max( nodeVar103, 4.0 );
		nodeVar106 = exp2( nodeVar103 );
		nodeVar107 = ( ( getUV( ( object.nodeUniform23 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar104 ) * vec2<f32>( ( nodeVar106 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar104 > 2.0 ) ) {

			nodeVar107.y = ( nodeVar107.y + nodeVar106 );
			nodeVar104 = ( nodeVar104 - 3.0 );
			

		}

		nodeVar107.x = ( nodeVar107.x + ( nodeVar104 * nodeVar106 ) );
		nodeVar107.x = ( nodeVar107.x + ( nodeVar105 * ( 3.0 * 16.0 ) ) );
		nodeVar107.y = ( nodeVar107.y + ( 4.0 * ( exp2( object.nodeUniform22 ) - nodeVar106 ) ) );
		nodeVar107.x = ( nodeVar107.x * object.nodeUniform25 );
		nodeVar107.y = ( nodeVar107.y * object.nodeUniform26 );
		nodeVar108 = textureSampleGrad( nodeUniform27, nodeUniform27_sampler, nodeVar107, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar109 = nodeVar108.xyz;
		nodeVar101 = mix( nodeVar101, nodeVar109, nodeVar102 );
		

	}

	nodeVar110 = ( iblIrradiance + ( ( nodeVar101 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform28 ) ) );
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
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar206 = ( object.nodeUniform29 + ( ( ( triNoise3D( ( v_positionWorld * vec3<f32>( 0.005 ) ), 0.2, object.nodeUniform30 ) + triNoise3D( ( v_positionWorld * vec3<f32>( 0.01 ) ), 0.2, ( object.nodeUniform30 * 1.2 ) ) ) - 0.7 ) * 22.0 ) );
	nodeVar207 = ( - v_positionView.z );
	nodeVar208 = vec4<f32>( mix( Output.xyz, vec3<f32>( 0.6307571363387763, 0.7304607400847158, 0.7991027380100881 ), ( 1.0 - ( ( 1.0 - ( clamp( ( ( nodeVar206 - v_positionWorld.y ) / ( nodeVar206 - object.nodeUniform31 ) ), 0.0, 1.0 ) * 0.98 ) ) * ( 1.0 - ( 1.0 - exp( ( - ( ( ( object.nodeUniform32 * object.nodeUniform32 ) * nodeVar207 ) * nodeVar207 ) ) ) ) ) ) ) ), Output.w );
	Output = nodeVar208;

	// result

	output.color = nodeVar208;

	return output;

}
