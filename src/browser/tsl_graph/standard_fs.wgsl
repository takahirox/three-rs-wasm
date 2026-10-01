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
@binding( 3 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar0 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
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
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec2<f32>;
var<private> nodeVar53 : vec4<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec2<f32>;
var<private> nodeVar63 : vec4<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec2<f32>;
var<private> nodeVar71 : vec4<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : f32;
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
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec4<f32>;

// codes
fn tri ( x : f32 ) -> f32 {

	


	return abs( ( fract( x ) - 0.5 ) );

}


fn tri3 ( p : vec3<f32> ) -> vec3<f32> {

	


	return vec3<f32>( tri( ( p.z + tri( ( p.y * 1.0 ) ) ) ), tri( ( p.z + tri( ( p.x * 1.0 ) ) ) ), tri( ( p.y + tri( ( p.x * 1.0 ) ) ) ) );

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
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	DiffuseColor = vec4<f32>( mix( vec3<f32>( triNoise3D( v_positionWorld, 1.0, 0.0 ) ), mix( vec3<f32>( 0.001214107934117647, 0.06480326668529614, 0.006512090790025684 ), vec3<f32>( 0.3712376804636741, 0.9646862478936612, 0.06301001764564068 ), clamp( ( mx_fractal_noise_float( ( v_positionWorld * vec3<f32>( 66.7 ) ), 4, 2.0, 0.5 ) * 0.5 ), 0.0, 1.0 ) ), clamp( normalWorld.y, 0.0, 1.0 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	Metalness = 0.0;
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 1.0, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - 0.0 ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = dot( normalView, positionViewDirection );
	nodeVar2 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( nodeVar1, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	nodeVar3 = ( dfg.x + dfg.y );
	nodeVar4 = ( 1.0 / nodeVar3 );
	nodeVar5 = nodeVar4;
	nodeVar6 = ( nodeVar5 - 1.0 );
	nodeVar7 = ( SpecularColorBlended * vec3<f32>( nodeVar6 ) );
	nodeVar8 = ( nodeVar7 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar8;
	nodeVar9 = ( render.nodeUniform8 - render.nodeUniform9 );
	nodeVar10 = vec4<f32>( nodeVar9, 0.0 );
	nodeVar11 = ( render.cameraViewMatrix * nodeVar10 );
	nodeVar12 = normalize( nodeVar11.xyz );
	nodeVar13 = nodeVar12;
	nodeVar14 = dot( normalView, nodeVar13 );
	nodeVar15 = ( vec3<f32>( clamp( nodeVar14, 0.0, 1.0 ) ) * render.nodeUniform10 );
	nodeVar16 = nodeVar15;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar17 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar18 = ( nodeVar16 * nodeVar17 );
	nodeVar19 = ( nodeVar13 + positionViewDirection );
	nodeVar20 = normalize( nodeVar19 );
	nodeVar21 = dot( positionViewDirection, nodeVar20 );
	nodeVar22 = clamp( nodeVar21, 0.0, 1.0 );
	nodeVar23 = exp2( ( ( ( nodeVar22 * -5.55473 ) - 6.98316 ) * nodeVar22 ) );
	nodeVar24 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar23 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar23 ) ) );
	nodeVar25 = ( vec3<f32>( 1.0 ) - nodeVar24 );
	nodeVar26 = nodeVar25;
	nodeVar27 = ( nodeVar18 * nodeVar26 );
	nodeVar28 = ( directDiffuse + nodeVar27 );
	directDiffuse = nodeVar28;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar29 = normalize( ( nodeVar13 + positionViewDirection ) );
	nodeVar30 = clamp( dot( positionViewDirection, nodeVar29 ), 0.0, 1.0 );
	nodeVar31 = exp2( ( ( ( nodeVar30 * -5.55473 ) - 6.98316 ) * nodeVar30 ) );
	nodeVar32 = ( Roughness * Roughness );
	nodeVar33 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar31 ) ) ) + vec3<f32>( ( 1.0 * nodeVar31 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar32, clamp( dot( normalView, nodeVar13 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar32, clamp( dot( normalView, nodeVar29 ), 0.0, 1.0 ) ) ) );
	nodeVar34 = ( nodeVar16 * nodeVar33 );
	nodeVar35 = ( nodeVar34 * multiScatteringCompensation );
	nodeVar36 = ( directSpecular + nodeVar35 );
	directSpecular = nodeVar36;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar37 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform11 );
	nodeVar38 = floor( nodeVar37 );
	nodeVar39 = nodeVar38;
	nodeVar40 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar41 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar40.x, ( - nodeVar40.y ), nodeVar40.z ), 1.0 ) ).xyz );
	nodeVar42 = max( ( 4.0 - nodeVar39 ), 0.0 );
	nodeVar39 = max( nodeVar39, 4.0 );
	nodeVar43 = exp2( nodeVar39 );
	nodeVar44 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar40.x, ( - nodeVar40.y ), nodeVar40.z ), 1.0 ) ).xyz, nodeVar41 ) * vec2<f32>( ( nodeVar43 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar41 > 2.0 ) ) {

		nodeVar44.y = ( nodeVar44.y + nodeVar43 );
		nodeVar41 = ( nodeVar41 - 3.0 );
		

	}

	nodeVar44.x = ( nodeVar44.x + ( nodeVar41 * nodeVar43 ) );
	nodeVar44.x = ( nodeVar44.x + ( nodeVar42 * ( 3.0 * 16.0 ) ) );
	nodeVar44.y = ( nodeVar44.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar43 ) ) );
	nodeVar44.x = ( nodeVar44.x * object.nodeUniform14 );
	nodeVar44.y = ( nodeVar44.y * object.nodeUniform15 );
	nodeVar45 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar44, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar46 = nodeVar45.xyz;
	nodeVar47 = fract( nodeVar37 );

	if ( ( nodeVar47 != 0.0 ) ) {

		nodeVar48 = ( nodeVar38 + 1.0 );
		nodeVar49 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar40.x, ( - nodeVar40.y ), nodeVar40.z ), 1.0 ) ).xyz );
		nodeVar50 = max( ( 4.0 - nodeVar48 ), 0.0 );
		nodeVar48 = max( nodeVar48, 4.0 );
		nodeVar51 = exp2( nodeVar48 );
		nodeVar52 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( nodeVar40.x, ( - nodeVar40.y ), nodeVar40.z ), 1.0 ) ).xyz, nodeVar49 ) * vec2<f32>( ( nodeVar51 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar49 > 2.0 ) ) {

			nodeVar52.y = ( nodeVar52.y + nodeVar51 );
			nodeVar49 = ( nodeVar49 - 3.0 );
			

		}

		nodeVar52.x = ( nodeVar52.x + ( nodeVar49 * nodeVar51 ) );
		nodeVar52.x = ( nodeVar52.x + ( nodeVar50 * ( 3.0 * 16.0 ) ) );
		nodeVar52.y = ( nodeVar52.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar51 ) ) );
		nodeVar52.x = ( nodeVar52.x * object.nodeUniform14 );
		nodeVar52.y = ( nodeVar52.y * object.nodeUniform15 );
		nodeVar53 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar52, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar54 = nodeVar53.xyz;
		nodeVar46 = mix( nodeVar46, nodeVar54, nodeVar47 );
		

	}

	nodeVar55 = ( radiance + ( nodeVar46 * vec3<f32>( object.nodeUniform17 ) ) );
	radiance = nodeVar55;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar56 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform11 );
	nodeVar57 = floor( nodeVar56 );
	nodeVar58 = nodeVar57;
	nodeVar59 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar60 = max( ( 4.0 - nodeVar58 ), 0.0 );
	nodeVar58 = max( nodeVar58, 4.0 );
	nodeVar61 = exp2( nodeVar58 );
	nodeVar62 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar59 ) * vec2<f32>( ( nodeVar61 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar59 > 2.0 ) ) {

		nodeVar62.y = ( nodeVar62.y + nodeVar61 );
		nodeVar59 = ( nodeVar59 - 3.0 );
		

	}

	nodeVar62.x = ( nodeVar62.x + ( nodeVar59 * nodeVar61 ) );
	nodeVar62.x = ( nodeVar62.x + ( nodeVar60 * ( 3.0 * 16.0 ) ) );
	nodeVar62.y = ( nodeVar62.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar61 ) ) );
	nodeVar62.x = ( nodeVar62.x * object.nodeUniform14 );
	nodeVar62.y = ( nodeVar62.y * object.nodeUniform15 );
	nodeVar63 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar62, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar64 = nodeVar63.xyz;
	nodeVar65 = fract( nodeVar56 );

	if ( ( nodeVar65 != 0.0 ) ) {

		nodeVar66 = ( nodeVar57 + 1.0 );
		nodeVar67 = getFace( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar68 = max( ( 4.0 - nodeVar66 ), 0.0 );
		nodeVar66 = max( nodeVar66, 4.0 );
		nodeVar69 = exp2( nodeVar66 );
		nodeVar70 = ( ( getUV( ( object.nodeUniform12 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar67 ) * vec2<f32>( ( nodeVar69 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar67 > 2.0 ) ) {

			nodeVar70.y = ( nodeVar70.y + nodeVar69 );
			nodeVar67 = ( nodeVar67 - 3.0 );
			

		}

		nodeVar70.x = ( nodeVar70.x + ( nodeVar67 * nodeVar69 ) );
		nodeVar70.x = ( nodeVar70.x + ( nodeVar68 * ( 3.0 * 16.0 ) ) );
		nodeVar70.y = ( nodeVar70.y + ( 4.0 * ( exp2( object.nodeUniform11 ) - nodeVar69 ) ) );
		nodeVar70.x = ( nodeVar70.x * object.nodeUniform14 );
		nodeVar70.y = ( nodeVar70.y * object.nodeUniform15 );
		nodeVar71 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar70, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar72 = nodeVar71.xyz;
		nodeVar64 = mix( nodeVar64, nodeVar72, nodeVar65 );
		

	}

	nodeVar73 = ( iblIrradiance + ( ( nodeVar64 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform17 ) ) );
	iblIrradiance = nodeVar73;
	nodeVar74 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar75 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar76 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar77 = ( SpecularF90 * dfg.y );
	nodeVar78 = ( nodeVar76 + vec3<f32>( nodeVar77 ) );
	nodeVar79 = ( nodeVar74 + nodeVar78 );
	nodeVar74 = nodeVar79;
	nodeVar80 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar81 = nodeVar80;
	nodeVar82 = ( nodeVar81 * vec3<f32>( 0.047619 ) );
	nodeVar83 = ( SpecularColor + nodeVar82 );
	nodeVar84 = ( nodeVar78 * nodeVar83 );
	nodeVar85 = ( dfg.x + dfg.y );
	nodeVar86 = ( 1.0 - nodeVar85 );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( vec3<f32>( nodeVar87 ) * nodeVar83 );
	nodeVar89 = ( vec3<f32>( 1.0 ) - nodeVar88 );
	nodeVar90 = nodeVar89;
	nodeVar91 = ( nodeVar84 / nodeVar90 );
	nodeVar92 = ( nodeVar91 * vec3<f32>( nodeVar87 ) );
	nodeVar93 = ( nodeVar75 + nodeVar92 );
	nodeVar75 = nodeVar93;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar95 = ( irradiance * nodeVar94 );
	nodeVar96 = ( nodeVar74 + nodeVar75 );
	nodeVar97 = ( vec3<f32>( 1.0 ) - nodeVar96 );
	nodeVar98 = nodeVar97;
	nodeVar99 = ( nodeVar95 * nodeVar98 );
	nodeVar100 = nodeVar99;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar101 = ( indirectDiffuse + nodeVar100 );
	indirectDiffuse = nodeVar101;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar102 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar103 = ( SpecularF90 * dfg.y );
	nodeVar104 = ( nodeVar102 + vec3<f32>( nodeVar103 ) );
	nodeVar105 = ( singleScatteringDielectric + nodeVar104 );
	singleScatteringDielectric = nodeVar105;
	nodeVar106 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( nodeVar107 * vec3<f32>( 0.047619 ) );
	nodeVar109 = ( SpecularColor + nodeVar108 );
	nodeVar110 = ( nodeVar104 * nodeVar109 );
	nodeVar111 = ( dfg.x + dfg.y );
	nodeVar112 = ( 1.0 - nodeVar111 );
	nodeVar113 = nodeVar112;
	nodeVar114 = ( vec3<f32>( nodeVar113 ) * nodeVar109 );
	nodeVar115 = ( vec3<f32>( 1.0 ) - nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( nodeVar110 / nodeVar116 );
	nodeVar118 = ( nodeVar117 * vec3<f32>( nodeVar113 ) );
	nodeVar119 = ( multiScatteringDielectric + nodeVar118 );
	multiScatteringDielectric = nodeVar119;
	nodeVar120 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar121 = ( SpecularF90 * dfg.y );
	nodeVar122 = ( nodeVar120 + vec3<f32>( nodeVar121 ) );
	nodeVar123 = ( singleScatteringMetallic + nodeVar122 );
	singleScatteringMetallic = nodeVar123;
	nodeVar124 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( nodeVar125 * vec3<f32>( 0.047619 ) );
	nodeVar127 = ( DiffuseColor.xyz + nodeVar126 );
	nodeVar128 = ( nodeVar122 * nodeVar127 );
	nodeVar129 = ( dfg.x + dfg.y );
	nodeVar130 = ( 1.0 - nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( vec3<f32>( nodeVar131 ) * nodeVar127 );
	nodeVar133 = ( vec3<f32>( 1.0 ) - nodeVar132 );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( nodeVar128 / nodeVar134 );
	nodeVar136 = ( nodeVar135 * vec3<f32>( nodeVar131 ) );
	nodeVar137 = ( multiScatteringMetallic + nodeVar136 );
	multiScatteringMetallic = nodeVar137;
	nodeVar138 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar139 = ( radiance * nodeVar138 );
	nodeVar140 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar141 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar142 = ( nodeVar140 * nodeVar141 );
	nodeVar143 = ( nodeVar139 + nodeVar142 );
	nodeVar144 = nodeVar143;
	nodeVar145 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar146 = ( vec3<f32>( 1.0 ) - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( DiffuseContribution * nodeVar147 );
	nodeVar149 = ( nodeVar148 * nodeVar141 );
	nodeVar150 = nodeVar149;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar151 = ( indirectSpecular + nodeVar144 );
	indirectSpecular = nodeVar151;
	nodeVar152 = ( indirectDiffuse + nodeVar150 );
	indirectDiffuse = nodeVar152;
	ambientOcclusion = 1.0;
	nodeVar153 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar153;
	nodeVar154 = dot( normalView, positionViewDirection );
	nodeVar155 = ( clamp( nodeVar154, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar156 = ( Roughness * -16.0 );
	nodeVar157 = ( 1.0 - nodeVar156 );
	nodeVar158 = nodeVar157;
	nodeVar159 = ( - nodeVar158 );
	nodeVar160 = exp2( nodeVar159 );
	nodeVar161 = pow( nodeVar155, nodeVar160 );
	nodeVar162 = ( 1.0 - nodeVar161 );
	nodeVar163 = nodeVar162;
	nodeVar164 = ( ambientOcclusion - nodeVar163 );
	nodeVar165 = ( indirectSpecular * vec3<f32>( clamp( nodeVar164, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar165;
	nodeVar166 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar166;
	nodeVar167 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar167;
	nodeVar168 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar168;
	nodeVar169 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar169;

	// result

	output.color = nodeVar169;

	return output;

}
