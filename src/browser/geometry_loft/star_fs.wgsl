// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform16_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform16 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform30_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform30 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform13 : vec4<f32>,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform19 : vec4<f32>,
	nodeUniform12 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : vec2<f32>,
	nodeUniform24 : f32
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
var<private> nodeVar1 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec2<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : f32;
var<private> shadowValue : f32;
var<private> nodeVar21 : f32;
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
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec2<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec2<f32>;
var<private> nodeVar84 : vec4<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec2<f32>;
var<private> nodeVar92 : vec4<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec2<f32>;
var<private> nodeVar102 : vec4<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : vec2<f32>;
var<private> nodeVar110 : vec4<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
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
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : f32;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : f32;
var<private> nodeVar203 : f32;
var<private> nodeVar204 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec4<f32>;

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
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@builtin( front_facing ) isFront : bool,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 0.6038273388475408, 0.09084171117479915, 0.057805430183792694 ) * vec3<f32>( ( ( ( ( mx_perlin_noise_float_1( ( positionLocal * vec3<f32>( 3.0 ) ) ) * 1.0 ) + 0.0 ) * 0.12 ) + 0.94 ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform2, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	nodeVar1 = normalize( dpdx( v_positionView ) );
	NORMAL_normalView = normalViewGeometry;
	nodeVar2 = cross( normalize( - dpdy( v_positionView ) ), NORMAL_normalView );
	nodeVar3 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	nodeVar4 = ( dot( nodeVar1, nodeVar2 ) * nodeVar3 );
	nodeVar5 = ( vec2<f32>( ( ( ( ( mx_perlin_noise_float_1( ( positionLocal * vec3<f32>( 50.0 ) ) ) * 1.0 ) + 0.0 ) * 0.008 ) - ( ( ( mx_perlin_noise_float_1( ( positionLocal * vec3<f32>( 50.0 ) ) ) * 1.0 ) + 0.0 ) * 0.008 ) ), ( ( ( ( mx_perlin_noise_float_1( ( positionLocal * vec3<f32>( 50.0 ) ) ) * 1.0 ) + 0.0 ) * 0.008 ) - ( ( ( mx_perlin_noise_float_1( ( positionLocal * vec3<f32>( 50.0 ) ) ) * 1.0 ) + 0.0 ) * 0.008 ) ) ) * vec2<f32>( 1.0 ) );
	normalView = normalize( ( ( vec3<f32>( abs( nodeVar4 ) ) * NORMAL_normalView ) - ( vec3<f32>( sign( nodeVar4 ) ) * ( ( vec3<f32>( nodeVar5.x ) * nodeVar2 ) + ( vec3<f32>( nodeVar5.y ) * cross( NORMAL_normalView, nodeVar1 ) ) ) ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = dot( normalView, positionViewDirection );
	nodeVar7 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( nodeVar6, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar7;
	nodeVar8 = ( dfg.x + dfg.y );
	nodeVar9 = ( 1.0 / nodeVar8 );
	nodeVar10 = nodeVar9;
	nodeVar11 = ( nodeVar10 - 1.0 );
	nodeVar12 = ( SpecularColorBlended * vec3<f32>( nodeVar11 ) );
	nodeVar13 = ( nodeVar12 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar13;
	nodeVar14 = vec4<f32>( render.nodeUniform10, 0.0 );
	nodeVar15 = ( render.cameraViewMatrix * nodeVar14 );
	nodeVar16 = normalize( nodeVar15.xyz );
	nodeVar17 = nodeVar16;
	nodeVar18 = dot( normalView, nodeVar17 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar19 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform12 ) ) ), 1.0 );
	nodeVar20 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar20 >= render.nodeUniform13.x ) && ( nodeVar20 < render.nodeUniform13.y ) ) ) {

		nodeVar22 = ( render.nodeUniform14 * nodeVar19 );
		nodeVar23 = ( nodeVar22.xyz / vec3<f32>( nodeVar22.w ) );
		nodeVar24 = vec3<f32>( nodeVar23.x, ( 1.0 - nodeVar23.y ), ( nodeVar23.z + render.nodeUniform15 ) );

		if ( ( ( ( ( ( nodeVar24.x >= 0.0 ) && ( nodeVar24.x <= 1.0 ) ) && ( nodeVar24.y >= 0.0 ) ) && ( nodeVar24.y <= 1.0 ) ) && ( nodeVar24.z <= 1.0 ) ) ) {

			nodeVar25 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar26 = ( render.nodeUniform17 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ).x );
			nodeVar27 = ( nodeVar24.xy + ( vogelDiskSample( 0, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar28 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar27, nodeVar24.z );
			nodeVar29 = ( nodeVar24.xy + ( vogelDiskSample( 1, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar30 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar29, nodeVar24.z );
			nodeVar31 = ( nodeVar24.xy + ( vogelDiskSample( 2, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar32 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar31, nodeVar24.z );
			nodeVar33 = ( nodeVar24.xy + ( vogelDiskSample( 3, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar34 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar33, nodeVar24.z );
			nodeVar35 = ( nodeVar24.xy + ( vogelDiskSample( 4, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar36 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar35, nodeVar24.z );
			nodeVar21 = ( ( ( ( ( nodeVar28 + nodeVar30 ) + nodeVar32 ) + nodeVar34 ) + nodeVar36 ) * 0.2 );

		} else {

			nodeVar21 = 1.0;

		}

		shadowValue = mix( nodeVar21, shadowValue, smoothstep( render.nodeUniform13.z, render.nodeUniform13.y, nodeVar20 ) );
		

	}


	if ( ( ( nodeVar20 >= render.nodeUniform19.x ) && ( nodeVar20 < render.nodeUniform19.y ) ) ) {

		nodeVar38 = ( render.nodeUniform20 * nodeVar19 );
		nodeVar39 = ( nodeVar38.xyz / vec3<f32>( nodeVar38.w ) );
		nodeVar40 = vec3<f32>( nodeVar39.x, ( 1.0 - nodeVar39.y ), ( nodeVar39.z + render.nodeUniform21 ) );

		if ( ( ( ( ( ( nodeVar40.x >= 0.0 ) && ( nodeVar40.x <= 1.0 ) ) && ( nodeVar40.y >= 0.0 ) ) && ( nodeVar40.y <= 1.0 ) ) && ( nodeVar40.z <= 1.0 ) ) ) {

			nodeVar41 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar42 = ( render.nodeUniform22 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform23 ).x );
			nodeVar43 = ( nodeVar40.xy + ( vogelDiskSample( 0, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar44 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar43, nodeVar40.z );
			nodeVar45 = ( nodeVar40.xy + ( vogelDiskSample( 1, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar46 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar45, nodeVar40.z );
			nodeVar47 = ( nodeVar40.xy + ( vogelDiskSample( 2, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar48 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar47, nodeVar40.z );
			nodeVar49 = ( nodeVar40.xy + ( vogelDiskSample( 3, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar50 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar49, nodeVar40.z );
			nodeVar51 = ( nodeVar40.xy + ( vogelDiskSample( 4, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar52 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar51, nodeVar40.z );
			nodeVar37 = ( ( ( ( ( nodeVar44 + nodeVar46 ) + nodeVar48 ) + nodeVar50 ) + nodeVar52 ) * 0.2 );

		} else {

			nodeVar37 = 1.0;

		}

		shadowValue = mix( nodeVar37, shadowValue, smoothstep( render.nodeUniform19.z, render.nodeUniform19.y, nodeVar20 ) );
		

	}

	nodeVar53 = mix( 1.0, shadowValue, render.nodeUniform24 );
	nodeVar54 = ( vec3<f32>( clamp( nodeVar18, 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar53 ) ) );
	nodeVar55 = nodeVar54;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar56 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar57 = ( nodeVar55 * nodeVar56 );
	nodeVar58 = ( nodeVar17 + positionViewDirection );
	nodeVar59 = normalize( nodeVar58 );
	nodeVar60 = dot( positionViewDirection, nodeVar59 );
	nodeVar61 = clamp( nodeVar60, 0.0, 1.0 );
	nodeVar62 = exp2( ( ( ( nodeVar61 * -5.55473 ) - 6.98316 ) * nodeVar61 ) );
	nodeVar63 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar62 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar62 ) ) );
	nodeVar64 = ( vec3<f32>( 1.0 ) - nodeVar63 );
	nodeVar65 = nodeVar64;
	nodeVar66 = ( nodeVar57 * nodeVar65 );
	nodeVar67 = ( directDiffuse + nodeVar66 );
	directDiffuse = nodeVar67;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar68 = normalize( ( nodeVar17 + positionViewDirection ) );
	nodeVar69 = clamp( dot( positionViewDirection, nodeVar68 ), 0.0, 1.0 );
	nodeVar70 = exp2( ( ( ( nodeVar69 * -5.55473 ) - 6.98316 ) * nodeVar69 ) );
	nodeVar71 = ( Roughness * Roughness );
	nodeVar72 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar70 ) ) ) + vec3<f32>( ( 1.0 * nodeVar70 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar71, clamp( dot( normalView, nodeVar17 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar71, clamp( dot( normalView, nodeVar68 ), 0.0, 1.0 ) ) ) );
	nodeVar73 = ( nodeVar55 * nodeVar72 );
	nodeVar74 = ( nodeVar73 * multiScatteringCompensation );
	nodeVar75 = ( directSpecular + nodeVar74 );
	directSpecular = nodeVar75;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar76 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform25 );
	nodeVar77 = floor( nodeVar76 );
	nodeVar78 = nodeVar77;
	nodeVar79 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar80 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar79.x, ( - nodeVar79.y ), nodeVar79.z ), 1.0 ) ).xyz );
	nodeVar81 = max( ( 4.0 - nodeVar78 ), 0.0 );
	nodeVar78 = max( nodeVar78, 4.0 );
	nodeVar82 = exp2( nodeVar78 );
	nodeVar83 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar79.x, ( - nodeVar79.y ), nodeVar79.z ), 1.0 ) ).xyz, nodeVar80 ) * vec2<f32>( ( nodeVar82 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar80 > 2.0 ) ) {

		nodeVar83.y = ( nodeVar83.y + nodeVar82 );
		nodeVar80 = ( nodeVar80 - 3.0 );
		

	}

	nodeVar83.x = ( nodeVar83.x + ( nodeVar80 * nodeVar82 ) );
	nodeVar83.x = ( nodeVar83.x + ( nodeVar81 * ( 3.0 * 16.0 ) ) );
	nodeVar83.y = ( nodeVar83.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar82 ) ) );
	nodeVar83.x = ( nodeVar83.x * object.nodeUniform28 );
	nodeVar83.y = ( nodeVar83.y * object.nodeUniform29 );
	nodeVar84 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar83, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar85 = nodeVar84.xyz;
	nodeVar86 = fract( nodeVar76 );

	if ( ( nodeVar86 != 0.0 ) ) {

		nodeVar87 = ( nodeVar77 + 1.0 );
		nodeVar88 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar79.x, ( - nodeVar79.y ), nodeVar79.z ), 1.0 ) ).xyz );
		nodeVar89 = max( ( 4.0 - nodeVar87 ), 0.0 );
		nodeVar87 = max( nodeVar87, 4.0 );
		nodeVar90 = exp2( nodeVar87 );
		nodeVar91 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar79.x, ( - nodeVar79.y ), nodeVar79.z ), 1.0 ) ).xyz, nodeVar88 ) * vec2<f32>( ( nodeVar90 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar88 > 2.0 ) ) {

			nodeVar91.y = ( nodeVar91.y + nodeVar90 );
			nodeVar88 = ( nodeVar88 - 3.0 );
			

		}

		nodeVar91.x = ( nodeVar91.x + ( nodeVar88 * nodeVar90 ) );
		nodeVar91.x = ( nodeVar91.x + ( nodeVar89 * ( 3.0 * 16.0 ) ) );
		nodeVar91.y = ( nodeVar91.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar90 ) ) );
		nodeVar91.x = ( nodeVar91.x * object.nodeUniform28 );
		nodeVar91.y = ( nodeVar91.y * object.nodeUniform29 );
		nodeVar92 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar91, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar93 = nodeVar92.xyz;
		nodeVar85 = mix( nodeVar85, nodeVar93, nodeVar86 );
		

	}

	nodeVar94 = ( radiance + ( nodeVar85 * vec3<f32>( object.nodeUniform31 ) ) );
	radiance = nodeVar94;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar95 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform25 );
	nodeVar96 = floor( nodeVar95 );
	nodeVar97 = nodeVar96;
	nodeVar98 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar99 = max( ( 4.0 - nodeVar97 ), 0.0 );
	nodeVar97 = max( nodeVar97, 4.0 );
	nodeVar100 = exp2( nodeVar97 );
	nodeVar101 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar98 ) * vec2<f32>( ( nodeVar100 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar98 > 2.0 ) ) {

		nodeVar101.y = ( nodeVar101.y + nodeVar100 );
		nodeVar98 = ( nodeVar98 - 3.0 );
		

	}

	nodeVar101.x = ( nodeVar101.x + ( nodeVar98 * nodeVar100 ) );
	nodeVar101.x = ( nodeVar101.x + ( nodeVar99 * ( 3.0 * 16.0 ) ) );
	nodeVar101.y = ( nodeVar101.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar100 ) ) );
	nodeVar101.x = ( nodeVar101.x * object.nodeUniform28 );
	nodeVar101.y = ( nodeVar101.y * object.nodeUniform29 );
	nodeVar102 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar101, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar103 = nodeVar102.xyz;
	nodeVar104 = fract( nodeVar95 );

	if ( ( nodeVar104 != 0.0 ) ) {

		nodeVar105 = ( nodeVar96 + 1.0 );
		nodeVar106 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar107 = max( ( 4.0 - nodeVar105 ), 0.0 );
		nodeVar105 = max( nodeVar105, 4.0 );
		nodeVar108 = exp2( nodeVar105 );
		nodeVar109 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar106 ) * vec2<f32>( ( nodeVar108 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar106 > 2.0 ) ) {

			nodeVar109.y = ( nodeVar109.y + nodeVar108 );
			nodeVar106 = ( nodeVar106 - 3.0 );
			

		}

		nodeVar109.x = ( nodeVar109.x + ( nodeVar106 * nodeVar108 ) );
		nodeVar109.x = ( nodeVar109.x + ( nodeVar107 * ( 3.0 * 16.0 ) ) );
		nodeVar109.y = ( nodeVar109.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar108 ) ) );
		nodeVar109.x = ( nodeVar109.x * object.nodeUniform28 );
		nodeVar109.y = ( nodeVar109.y * object.nodeUniform29 );
		nodeVar110 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar109, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar111 = nodeVar110.xyz;
		nodeVar103 = mix( nodeVar103, nodeVar111, nodeVar104 );
		

	}

	nodeVar112 = ( iblIrradiance + ( ( nodeVar103 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform31 ) ) );
	iblIrradiance = nodeVar112;
	nodeVar113 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar114 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar115 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar116 = ( SpecularF90 * dfg.y );
	nodeVar117 = ( nodeVar115 + vec3<f32>( nodeVar116 ) );
	nodeVar118 = ( nodeVar113 + nodeVar117 );
	nodeVar113 = nodeVar118;
	nodeVar119 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( nodeVar120 * vec3<f32>( 0.047619 ) );
	nodeVar122 = ( SpecularColor + nodeVar121 );
	nodeVar123 = ( nodeVar117 * nodeVar122 );
	nodeVar124 = ( dfg.x + dfg.y );
	nodeVar125 = ( 1.0 - nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( vec3<f32>( nodeVar126 ) * nodeVar122 );
	nodeVar128 = ( vec3<f32>( 1.0 ) - nodeVar127 );
	nodeVar129 = nodeVar128;
	nodeVar130 = ( nodeVar123 / nodeVar129 );
	nodeVar131 = ( nodeVar130 * vec3<f32>( nodeVar126 ) );
	nodeVar132 = ( nodeVar114 + nodeVar131 );
	nodeVar114 = nodeVar132;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar133 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar134 = ( irradiance * nodeVar133 );
	nodeVar135 = ( nodeVar113 + nodeVar114 );
	nodeVar136 = ( vec3<f32>( 1.0 ) - nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( nodeVar134 * nodeVar137 );
	nodeVar139 = nodeVar138;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = ( indirectDiffuse + nodeVar139 );
	indirectDiffuse = nodeVar140;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar141 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar142 = ( SpecularF90 * dfg.y );
	nodeVar143 = ( nodeVar141 + vec3<f32>( nodeVar142 ) );
	nodeVar144 = ( singleScatteringDielectric + nodeVar143 );
	singleScatteringDielectric = nodeVar144;
	nodeVar145 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar146 = nodeVar145;
	nodeVar147 = ( nodeVar146 * vec3<f32>( 0.047619 ) );
	nodeVar148 = ( SpecularColor + nodeVar147 );
	nodeVar149 = ( nodeVar143 * nodeVar148 );
	nodeVar150 = ( dfg.x + dfg.y );
	nodeVar151 = ( 1.0 - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( vec3<f32>( nodeVar152 ) * nodeVar148 );
	nodeVar154 = ( vec3<f32>( 1.0 ) - nodeVar153 );
	nodeVar155 = nodeVar154;
	nodeVar156 = ( nodeVar149 / nodeVar155 );
	nodeVar157 = ( nodeVar156 * vec3<f32>( nodeVar152 ) );
	nodeVar158 = ( multiScatteringDielectric + nodeVar157 );
	multiScatteringDielectric = nodeVar158;
	nodeVar159 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar160 = ( SpecularF90 * dfg.y );
	nodeVar161 = ( nodeVar159 + vec3<f32>( nodeVar160 ) );
	nodeVar162 = ( singleScatteringMetallic + nodeVar161 );
	singleScatteringMetallic = nodeVar162;
	nodeVar163 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar164 = nodeVar163;
	nodeVar165 = ( nodeVar164 * vec3<f32>( 0.047619 ) );
	nodeVar166 = ( DiffuseColor.xyz + nodeVar165 );
	nodeVar167 = ( nodeVar161 * nodeVar166 );
	nodeVar168 = ( dfg.x + dfg.y );
	nodeVar169 = ( 1.0 - nodeVar168 );
	nodeVar170 = nodeVar169;
	nodeVar171 = ( vec3<f32>( nodeVar170 ) * nodeVar166 );
	nodeVar172 = ( vec3<f32>( 1.0 ) - nodeVar171 );
	nodeVar173 = nodeVar172;
	nodeVar174 = ( nodeVar167 / nodeVar173 );
	nodeVar175 = ( nodeVar174 * vec3<f32>( nodeVar170 ) );
	nodeVar176 = ( multiScatteringMetallic + nodeVar175 );
	multiScatteringMetallic = nodeVar176;
	nodeVar177 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar178 = ( radiance * nodeVar177 );
	nodeVar179 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar180 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar181 = ( nodeVar179 * nodeVar180 );
	nodeVar182 = ( nodeVar178 + nodeVar181 );
	nodeVar183 = nodeVar182;
	nodeVar184 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar185 = ( vec3<f32>( 1.0 ) - nodeVar184 );
	nodeVar186 = nodeVar185;
	nodeVar187 = ( DiffuseContribution * nodeVar186 );
	nodeVar188 = ( nodeVar187 * nodeVar180 );
	nodeVar189 = nodeVar188;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar190 = ( indirectSpecular + nodeVar183 );
	indirectSpecular = nodeVar190;
	nodeVar191 = ( indirectDiffuse + nodeVar189 );
	indirectDiffuse = nodeVar191;
	ambientOcclusion = 1.0;
	nodeVar192 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar192;
	nodeVar193 = dot( normalView, positionViewDirection );
	nodeVar194 = ( clamp( nodeVar193, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar195 = ( Roughness * -16.0 );
	nodeVar196 = ( 1.0 - nodeVar195 );
	nodeVar197 = nodeVar196;
	nodeVar198 = ( - nodeVar197 );
	nodeVar199 = exp2( nodeVar198 );
	nodeVar200 = pow( nodeVar194, nodeVar199 );
	nodeVar201 = ( 1.0 - nodeVar200 );
	nodeVar202 = nodeVar201;
	nodeVar203 = ( ambientOcclusion - nodeVar202 );
	nodeVar204 = ( indirectSpecular * vec3<f32>( clamp( nodeVar203, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar204;
	nodeVar205 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar205;
	nodeVar206 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar206;
	nodeVar207 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar207;
	nodeVar208 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar208;

	// result

	output.color = nodeVar208;

	return output;

}
