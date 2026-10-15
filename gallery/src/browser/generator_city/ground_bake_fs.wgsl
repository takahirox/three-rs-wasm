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
@binding( 5 ) @group( 1 ) var nodeUniform25_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform25 : texture_2d<f32>;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>,
	nodeUniform19 : f32,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform26 : f32
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
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_nodeVar37 : vec3<f32>;
var<private> NORMAL_nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec2<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec2<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec2<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec2<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec2<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : vec2<f32>;
var<private> nodeVar103 : vec4<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec2<f32>;
var<private> nodeVar111 : vec4<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : vec2<f32>;
var<private> nodeVar121 : vec4<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : vec2<f32>;
var<private> nodeVar129 : vec4<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar179 : f32;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : f32;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : f32;
var<private> nodeVar213 : f32;
var<private> nodeVar214 : f32;
var<private> nodeVar215 : f32;
var<private> nodeVar216 : f32;
var<private> nodeVar217 : f32;
var<private> nodeVar218 : f32;
var<private> nodeVar219 : f32;
var<private> nodeVar220 : f32;
var<private> nodeVar221 : f32;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar224 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar225 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar226 : vec3<f32>;
var<private> nodeVar227 : vec4<f32>;

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


fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
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

	nodeVar0 = 0.0;
	nodeVar1 = 0.0;
	nodeVar2 = 1.0;
	nodeVar3 = 0.0;
	nodeVar4 = distance( v_positionWorld, render.cameraPosition );
	nodeVar5 = smoothstep( 240.0, 25.0, nodeVar4 );

	if ( ( nodeVar5 > 0.0 ) ) {

		nodeVar0 = ( ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 7.0 ) ) ) * 1.0 ) + 0.0 ) + ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 23.0 ) ) ) * 1.0 ) + 0.0 ) ) * 0.5 );
		nodeVar1 = smoothstep( 0.5, 0.85, ( ( ( mx_fractal_noise_float( ( v_positionWorld * vec3<f32>( 0.45 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) );
		nodeVar2 = ( ( smoothstep( 0.25, 0.7, ( ( ( mx_fractal_noise_float( ( v_positionWorld * vec3<f32>( 0.7 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) ) * 0.55 ) + 0.35 );
		

	}

	nodeVar6 = smoothstep( 22.0, 4.0, nodeVar4 );

	if ( ( nodeVar6 > 0.0 ) ) {

		nodeVar3 = ( ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 45.0 ) ) ) * 1.0 ) + 0.0 ) * 0.6 ) + ( ( ( mx_perlin_noise_float_1( ( v_positionWorld * vec3<f32>( 80.0 ) ) ) * 1.0 ) + 0.0 ) * 0.4 ) );
		

	}

	nodeVar7 = vec4<f32>( nodeVar0, nodeVar1, nodeVar2, nodeVar3 );
	nodeVar8 = ( mix( vec3<f32>( 0.01764195448412081, 0.019382360952473074, 0.024157632443547246 ), vec3<f32>( 0.04373502925049377, 0.04970656597728775, 0.061246054224174035 ), ( ( ( mx_fractal_noise_float( ( v_positionWorld * vec3<f32>( 0.2 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) ) * vec3<f32>( ( ( ( nodeVar7.x * 0.22 ) * nodeVar5 ) + 1.0 ) ) );
	nodeVar9 = mix( nodeVar8, ( nodeVar8 * vec3<f32>( 0.5 ) ), ( ( nodeVar7.y * 0.5 ) * nodeVar5 ) );
	nodeVar10 = smoothstep( 0.6, 0.85, ( ( ( mx_fractal_noise_float( ( v_positionWorld * vec3<f32>( 0.14 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) );
	nodeVar11 = tsl_mod_float( ( v_positionWorld.x + 101.0 ), 112.0 );
	nodeVar12 = ( nodeVar11 - 90.0 );
	nodeVar13 = ( nodeVar12 - 11.0 );
	nodeVar14 = max( fwidth( nodeVar13 ), 0.0001 );
	nodeVar15 = ( nodeVar12 - 5.5 );
	nodeVar16 = max( fwidth( nodeVar15 ), 0.0001 );
	nodeVar17 = ( nodeVar12 - 16.5 );
	nodeVar18 = max( fwidth( nodeVar17 ), 0.0001 );
	nodeVar19 = step( 90.0, nodeVar11 );
	nodeVar20 = tsl_mod_float( ( v_positionWorld.z + 71.0 ), 82.0 );
	nodeVar21 = step( 60.0, nodeVar20 );
	nodeVar22 = ( nodeVar20 - 60.0 );
	nodeVar23 = ( nodeVar22 - 11.0 );
	nodeVar24 = max( fwidth( nodeVar23 ), 0.0001 );
	nodeVar25 = ( nodeVar22 - 5.5 );
	nodeVar26 = max( fwidth( nodeVar25 ), 0.0001 );
	nodeVar27 = ( nodeVar22 - 16.5 );
	nodeVar28 = max( fwidth( nodeVar27 ), 0.0001 );
	nodeVar29 = ( nodeVar12 / 1.2 );
	nodeVar30 = max( fwidth( nodeVar29 ), 0.0001 );
	nodeVar31 = ( nodeVar22 / 1.2 );
	nodeVar32 = max( fwidth( nodeVar31 ), 0.0001 );
	nodeVar33 = ( ( max( max( max( ( ( max( smoothstep( ( 0.12 + nodeVar14 ), ( 0.12 - nodeVar14 ), abs( nodeVar13 ) ), ( max( smoothstep( ( 0.1 + nodeVar16 ), ( 0.1 - nodeVar16 ), abs( nodeVar15 ) ), smoothstep( ( 0.1 + nodeVar18 ), ( 0.1 - nodeVar18 ), abs( nodeVar17 ) ) ) * step( fract( ( v_positionWorld.z / 7.0 ) ), 0.5 ) ) ) * nodeVar19 ) * ( 1.0 - nodeVar21 ) ), ( ( max( smoothstep( ( 0.12 + nodeVar24 ), ( 0.12 - nodeVar24 ), abs( nodeVar23 ) ), ( max( smoothstep( ( 0.1 + nodeVar26 ), ( 0.1 - nodeVar26 ), abs( nodeVar25 ) ), smoothstep( ( 0.1 + nodeVar28 ), ( 0.1 - nodeVar28 ), abs( nodeVar27 ) ) ) * step( fract( ( v_positionWorld.x / 7.0 ) ), 0.5 ) ) ) * nodeVar21 ) * ( 1.0 - nodeVar19 ) ) ), ( ( ( smoothstep( ( 0.3166666666666667 + nodeVar30 ), ( 0.3166666666666667 - nodeVar30 ), ( 0.5 - abs( ( fract( nodeVar29 ) - 0.5 ) ) ) ) * nodeVar19 ) * ( 1.0 - nodeVar21 ) ) * max( step( nodeVar20, 5.0 ), step( 55.0, nodeVar20 ) ) ) ), ( ( ( smoothstep( ( 0.3166666666666667 + nodeVar32 ), ( 0.3166666666666667 - nodeVar32 ), ( 0.5 - abs( ( fract( nodeVar31 ) - 0.5 ) ) ) ) * nodeVar21 ) * ( 1.0 - nodeVar19 ) ) * max( step( nodeVar11, 5.0 ), step( 85.0, nodeVar11 ) ) ) ) * nodeVar5 ) * nodeVar7.z );
	DiffuseColor = vec4<f32>( mix( mix( nodeVar9, ( nodeVar9 * vec3<f32>( 0.6 ) ), nodeVar10 ), vec3<f32>( 0.6307571363387763, 0.6038273388475408, 0.5271151256969157 ), nodeVar33 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform3;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar34 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( mix( ( 0.95 - ( nodeVar33 * 0.2 ) ), 0.32, nodeVar10 ), 0.0525 ) + max( max( nodeVar34.x, nodeVar34.y ), nodeVar34.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform3 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	nodeVar35 = dpdx( v_positionView );
	nodeVar36 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_nodeVar37 = cross( nodeVar36, NORMAL_normalView );
	NORMAL_nodeVar38 = dot( nodeVar35, NORMAL_nodeVar37 );
	nodeVar39 = ( ( ( nodeVar7.x * 0.003 ) * nodeVar5 ) + ( ( nodeVar7.w * 0.0016 ) * nodeVar6 ) );
	nodeVar40 = ( vec3<f32>( sign( NORMAL_nodeVar38 ) ) * ( ( vec3<f32>( dpdx( nodeVar39 ) ) * NORMAL_nodeVar37 ) + ( vec3<f32>( - dpdy( nodeVar39 ) ) * cross( NORMAL_normalView, nodeVar35 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeVar38 ) ) * NORMAL_normalView ) - nodeVar40 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar41 = dot( normalView, positionViewDirection );
	nodeVar42 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar41, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar42;
	nodeVar43 = ( dfg.x + dfg.y );
	nodeVar44 = ( 1.0 / nodeVar43 );
	nodeVar45 = nodeVar44;
	nodeVar46 = ( nodeVar45 - 1.0 );
	nodeVar47 = ( SpecularColorBlended * vec3<f32>( nodeVar46 ) );
	nodeVar48 = ( nodeVar47 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar48;
	nodeVar49 = ( render.nodeUniform10 - render.nodeUniform11 );
	nodeVar50 = vec4<f32>( nodeVar49, 0.0 );
	nodeVar51 = ( render.cameraViewMatrix * nodeVar50 );
	nodeVar52 = normalize( nodeVar51.xyz );
	nodeVar53 = nodeVar52;
	nodeVar54 = dot( normalView, nodeVar53 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar56 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	nodeVar57 = ( nodeVar56.xyz / vec3<f32>( nodeVar56.w ) );
	nodeVar58 = vec3<f32>( nodeVar57.x, ( 1.0 - nodeVar57.y ), ( nodeVar57.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeVar58.x >= 0.0 ) && ( nodeVar58.x <= 1.0 ) ) && ( nodeVar58.y >= 0.0 ) ) && ( nodeVar58.y <= 1.0 ) ) && ( nodeVar58.z <= 1.0 ) ) ) {

		nodeVar59 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar60 = ( render.nodeUniform17 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ).x );
		nodeVar61 = ( nodeVar58.xy + ( vogelDiskSample( 0, 5, nodeVar59 ) * vec2<f32>( nodeVar60 ) ) );
		nodeVar62 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar61, nodeVar58.z );
		nodeVar63 = ( nodeVar58.xy + ( vogelDiskSample( 1, 5, nodeVar59 ) * vec2<f32>( nodeVar60 ) ) );
		nodeVar64 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar63, nodeVar58.z );
		nodeVar65 = ( nodeVar58.xy + ( vogelDiskSample( 2, 5, nodeVar59 ) * vec2<f32>( nodeVar60 ) ) );
		nodeVar66 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar65, nodeVar58.z );
		nodeVar67 = ( nodeVar58.xy + ( vogelDiskSample( 3, 5, nodeVar59 ) * vec2<f32>( nodeVar60 ) ) );
		nodeVar68 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar67, nodeVar58.z );
		nodeVar69 = ( nodeVar58.xy + ( vogelDiskSample( 4, 5, nodeVar59 ) * vec2<f32>( nodeVar60 ) ) );
		nodeVar70 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar69, nodeVar58.z );
		nodeVar55 = ( ( ( ( ( nodeVar62 + nodeVar64 ) + nodeVar66 ) + nodeVar68 ) + nodeVar70 ) * 0.2 );

	} else {

		nodeVar55 = 1.0;

	}

	nodeVar71 = mix( 1.0, nodeVar55, render.nodeUniform19 );
	nodeVar72 = ( render.nodeUniform12 * vec3<f32>( nodeVar71 ) );
	nodeVar73 = ( vec3<f32>( clamp( nodeVar54, 0.0, 1.0 ) ) * nodeVar72 );
	nodeVar74 = nodeVar73;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar75 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar76 = ( nodeVar74 * nodeVar75 );
	nodeVar77 = ( nodeVar53 + positionViewDirection );
	nodeVar78 = normalize( nodeVar77 );
	nodeVar79 = dot( positionViewDirection, nodeVar78 );
	nodeVar80 = clamp( nodeVar79, 0.0, 1.0 );
	nodeVar81 = exp2( ( ( ( nodeVar80 * -5.55473 ) - 6.98316 ) * nodeVar80 ) );
	nodeVar82 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar81 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar81 ) ) );
	nodeVar83 = ( vec3<f32>( 1.0 ) - nodeVar82 );
	nodeVar84 = nodeVar83;
	nodeVar85 = ( nodeVar76 * nodeVar84 );
	nodeVar86 = ( directDiffuse + nodeVar85 );
	directDiffuse = nodeVar86;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar87 = normalize( ( nodeVar53 + positionViewDirection ) );
	nodeVar88 = clamp( dot( positionViewDirection, nodeVar87 ), 0.0, 1.0 );
	nodeVar89 = exp2( ( ( ( nodeVar88 * -5.55473 ) - 6.98316 ) * nodeVar88 ) );
	nodeVar90 = ( Roughness * Roughness );
	nodeVar91 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar89 ) ) ) + vec3<f32>( ( 1.0 * nodeVar89 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar90, clamp( dot( normalView, nodeVar53 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar90, clamp( dot( normalView, nodeVar87 ), 0.0, 1.0 ) ) ) );
	nodeVar92 = ( nodeVar74 * nodeVar91 );
	nodeVar93 = ( nodeVar92 * multiScatteringCompensation );
	nodeVar94 = ( directSpecular + nodeVar93 );
	directSpecular = nodeVar94;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar95 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform20 );
	nodeVar96 = floor( nodeVar95 );
	nodeVar97 = nodeVar96;
	nodeVar98 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar99 = getFace( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( nodeVar98.x, ( - nodeVar98.y ), nodeVar98.z ), 1.0 ) ).xyz );
	nodeVar100 = max( ( 4.0 - nodeVar97 ), 0.0 );
	nodeVar97 = max( nodeVar97, 4.0 );
	nodeVar101 = exp2( nodeVar97 );
	nodeVar102 = ( ( getUV( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( nodeVar98.x, ( - nodeVar98.y ), nodeVar98.z ), 1.0 ) ).xyz, nodeVar99 ) * vec2<f32>( ( nodeVar101 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar99 > 2.0 ) ) {

		nodeVar102.y = ( nodeVar102.y + nodeVar101 );
		nodeVar99 = ( nodeVar99 - 3.0 );
		

	}

	nodeVar102.x = ( nodeVar102.x + ( nodeVar99 * nodeVar101 ) );
	nodeVar102.x = ( nodeVar102.x + ( nodeVar100 * ( 3.0 * 16.0 ) ) );
	nodeVar102.y = ( nodeVar102.y + ( 4.0 * ( exp2( object.nodeUniform20 ) - nodeVar101 ) ) );
	nodeVar102.x = ( nodeVar102.x * object.nodeUniform23 );
	nodeVar102.y = ( nodeVar102.y * object.nodeUniform24 );
	nodeVar103 = textureSampleGrad( nodeUniform25, nodeUniform25_sampler, nodeVar102, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar104 = nodeVar103.xyz;
	nodeVar105 = fract( nodeVar95 );

	if ( ( nodeVar105 != 0.0 ) ) {

		nodeVar106 = ( nodeVar96 + 1.0 );
		nodeVar107 = getFace( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( nodeVar98.x, ( - nodeVar98.y ), nodeVar98.z ), 1.0 ) ).xyz );
		nodeVar108 = max( ( 4.0 - nodeVar106 ), 0.0 );
		nodeVar106 = max( nodeVar106, 4.0 );
		nodeVar109 = exp2( nodeVar106 );
		nodeVar110 = ( ( getUV( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( nodeVar98.x, ( - nodeVar98.y ), nodeVar98.z ), 1.0 ) ).xyz, nodeVar107 ) * vec2<f32>( ( nodeVar109 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar107 > 2.0 ) ) {

			nodeVar110.y = ( nodeVar110.y + nodeVar109 );
			nodeVar107 = ( nodeVar107 - 3.0 );
			

		}

		nodeVar110.x = ( nodeVar110.x + ( nodeVar107 * nodeVar109 ) );
		nodeVar110.x = ( nodeVar110.x + ( nodeVar108 * ( 3.0 * 16.0 ) ) );
		nodeVar110.y = ( nodeVar110.y + ( 4.0 * ( exp2( object.nodeUniform20 ) - nodeVar109 ) ) );
		nodeVar110.x = ( nodeVar110.x * object.nodeUniform23 );
		nodeVar110.y = ( nodeVar110.y * object.nodeUniform24 );
		nodeVar111 = textureSampleGrad( nodeUniform25, nodeUniform25_sampler, nodeVar110, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar112 = nodeVar111.xyz;
		nodeVar104 = mix( nodeVar104, nodeVar112, nodeVar105 );
		

	}

	nodeVar113 = ( radiance + ( nodeVar104 * vec3<f32>( object.nodeUniform26 ) ) );
	radiance = nodeVar113;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar114 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform20 );
	nodeVar115 = floor( nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = getFace( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar118 = max( ( 4.0 - nodeVar116 ), 0.0 );
	nodeVar116 = max( nodeVar116, 4.0 );
	nodeVar119 = exp2( nodeVar116 );
	nodeVar120 = ( ( getUV( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar117 ) * vec2<f32>( ( nodeVar119 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar117 > 2.0 ) ) {

		nodeVar120.y = ( nodeVar120.y + nodeVar119 );
		nodeVar117 = ( nodeVar117 - 3.0 );
		

	}

	nodeVar120.x = ( nodeVar120.x + ( nodeVar117 * nodeVar119 ) );
	nodeVar120.x = ( nodeVar120.x + ( nodeVar118 * ( 3.0 * 16.0 ) ) );
	nodeVar120.y = ( nodeVar120.y + ( 4.0 * ( exp2( object.nodeUniform20 ) - nodeVar119 ) ) );
	nodeVar120.x = ( nodeVar120.x * object.nodeUniform23 );
	nodeVar120.y = ( nodeVar120.y * object.nodeUniform24 );
	nodeVar121 = textureSampleGrad( nodeUniform25, nodeUniform25_sampler, nodeVar120, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar122 = nodeVar121.xyz;
	nodeVar123 = fract( nodeVar114 );

	if ( ( nodeVar123 != 0.0 ) ) {

		nodeVar124 = ( nodeVar115 + 1.0 );
		nodeVar125 = getFace( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar126 = max( ( 4.0 - nodeVar124 ), 0.0 );
		nodeVar124 = max( nodeVar124, 4.0 );
		nodeVar127 = exp2( nodeVar124 );
		nodeVar128 = ( ( getUV( ( object.nodeUniform21 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar125 ) * vec2<f32>( ( nodeVar127 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar125 > 2.0 ) ) {

			nodeVar128.y = ( nodeVar128.y + nodeVar127 );
			nodeVar125 = ( nodeVar125 - 3.0 );
			

		}

		nodeVar128.x = ( nodeVar128.x + ( nodeVar125 * nodeVar127 ) );
		nodeVar128.x = ( nodeVar128.x + ( nodeVar126 * ( 3.0 * 16.0 ) ) );
		nodeVar128.y = ( nodeVar128.y + ( 4.0 * ( exp2( object.nodeUniform20 ) - nodeVar127 ) ) );
		nodeVar128.x = ( nodeVar128.x * object.nodeUniform23 );
		nodeVar128.y = ( nodeVar128.y * object.nodeUniform24 );
		nodeVar129 = textureSampleGrad( nodeUniform25, nodeUniform25_sampler, nodeVar128, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar130 = nodeVar129.xyz;
		nodeVar122 = mix( nodeVar122, nodeVar130, nodeVar123 );
		

	}

	nodeVar131 = ( iblIrradiance + ( ( nodeVar122 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform26 ) ) );
	iblIrradiance = nodeVar131;
	nodeVar132 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar133 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar134 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar135 = ( SpecularF90 * dfg.y );
	nodeVar136 = ( nodeVar134 + vec3<f32>( nodeVar135 ) );
	nodeVar137 = ( nodeVar132 + nodeVar136 );
	nodeVar132 = nodeVar137;
	nodeVar138 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( nodeVar139 * vec3<f32>( 0.047619 ) );
	nodeVar141 = ( SpecularColor + nodeVar140 );
	nodeVar142 = ( nodeVar136 * nodeVar141 );
	nodeVar143 = ( dfg.x + dfg.y );
	nodeVar144 = ( 1.0 - nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( vec3<f32>( nodeVar145 ) * nodeVar141 );
	nodeVar147 = ( vec3<f32>( 1.0 ) - nodeVar146 );
	nodeVar148 = nodeVar147;
	nodeVar149 = ( nodeVar142 / nodeVar148 );
	nodeVar150 = ( nodeVar149 * vec3<f32>( nodeVar145 ) );
	nodeVar151 = ( nodeVar133 + nodeVar150 );
	nodeVar133 = nodeVar151;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar152 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar153 = ( irradiance * nodeVar152 );
	nodeVar154 = ( nodeVar132 + nodeVar133 );
	nodeVar155 = ( vec3<f32>( 1.0 ) - nodeVar154 );
	nodeVar156 = nodeVar155;
	nodeVar157 = ( nodeVar153 * nodeVar156 );
	nodeVar158 = nodeVar157;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar159 = ( indirectDiffuse + nodeVar158 );
	indirectDiffuse = nodeVar159;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar160 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar161 = ( SpecularF90 * dfg.y );
	nodeVar162 = ( nodeVar160 + vec3<f32>( nodeVar161 ) );
	nodeVar163 = ( singleScatteringDielectric + nodeVar162 );
	singleScatteringDielectric = nodeVar163;
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
	nodeVar177 = ( multiScatteringDielectric + nodeVar176 );
	multiScatteringDielectric = nodeVar177;
	nodeVar178 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar179 = ( SpecularF90 * dfg.y );
	nodeVar180 = ( nodeVar178 + vec3<f32>( nodeVar179 ) );
	nodeVar181 = ( singleScatteringMetallic + nodeVar180 );
	singleScatteringMetallic = nodeVar181;
	nodeVar182 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar183 = nodeVar182;
	nodeVar184 = ( nodeVar183 * vec3<f32>( 0.047619 ) );
	nodeVar185 = ( DiffuseColor.xyz + nodeVar184 );
	nodeVar186 = ( nodeVar180 * nodeVar185 );
	nodeVar187 = ( dfg.x + dfg.y );
	nodeVar188 = ( 1.0 - nodeVar187 );
	nodeVar189 = nodeVar188;
	nodeVar190 = ( vec3<f32>( nodeVar189 ) * nodeVar185 );
	nodeVar191 = ( vec3<f32>( 1.0 ) - nodeVar190 );
	nodeVar192 = nodeVar191;
	nodeVar193 = ( nodeVar186 / nodeVar192 );
	nodeVar194 = ( nodeVar193 * vec3<f32>( nodeVar189 ) );
	nodeVar195 = ( multiScatteringMetallic + nodeVar194 );
	multiScatteringMetallic = nodeVar195;
	nodeVar196 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar197 = ( radiance * nodeVar196 );
	nodeVar198 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar199 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar200 = ( nodeVar198 * nodeVar199 );
	nodeVar201 = ( nodeVar197 + nodeVar200 );
	nodeVar202 = nodeVar201;
	nodeVar203 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar204 = ( vec3<f32>( 1.0 ) - nodeVar203 );
	nodeVar205 = nodeVar204;
	nodeVar206 = ( DiffuseContribution * nodeVar205 );
	nodeVar207 = ( nodeVar206 * nodeVar199 );
	nodeVar208 = nodeVar207;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar209 = ( indirectSpecular + nodeVar202 );
	indirectSpecular = nodeVar209;
	nodeVar210 = ( indirectDiffuse + nodeVar208 );
	indirectDiffuse = nodeVar210;
	ambientOcclusion = 1.0;
	nodeVar211 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar211;
	nodeVar212 = dot( normalView, positionViewDirection );
	nodeVar213 = ( clamp( nodeVar212, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar214 = ( Roughness * -16.0 );
	nodeVar215 = ( 1.0 - nodeVar214 );
	nodeVar216 = nodeVar215;
	nodeVar217 = ( - nodeVar216 );
	nodeVar218 = exp2( nodeVar217 );
	nodeVar219 = pow( nodeVar213, nodeVar218 );
	nodeVar220 = ( 1.0 - nodeVar219 );
	nodeVar221 = nodeVar220;
	nodeVar222 = ( ambientOcclusion - nodeVar221 );
	nodeVar223 = ( indirectSpecular * vec3<f32>( clamp( nodeVar222, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar223;
	nodeVar224 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar224;
	nodeVar225 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar225;
	nodeVar226 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar226;
	nodeVar227 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar227;

	// result

	output.color = nodeVar227;

	return output;

}
