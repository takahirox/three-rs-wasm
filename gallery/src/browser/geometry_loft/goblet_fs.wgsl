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
	nodeUniform0 : vec3<f32>,
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
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : i32;
var<private> nodeVar3 : i32;
var<private> nodeVar4 : i32;
var<private> nodeVar5 : i32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : i32;
var<private> nodeVar15 : i32;
var<private> nodeVar16 : i32;
var<private> nodeVar17 : u32;
var<private> nodeVar18 : u32;
var<private> nodeVar19 : u32;
var<private> nodeVar20 : u32;
var<private> nodeVar21 : vec3<u32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : f32;
var<private> shadowValue : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec2<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec2<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : vec2<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec2<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec4<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec2<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec2<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec2<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec2<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec2<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec2<f32>;
var<private> nodeVar108 : vec4<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : vec2<f32>;
var<private> nodeVar116 : vec4<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
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
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
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
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : f32;
var<private> nodeVar218 : f32;
var<private> nodeVar219 : f32;
var<private> nodeVar220 : f32;
var<private> nodeVar221 : f32;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : f32;
var<private> nodeVar224 : f32;
var<private> nodeVar225 : f32;
var<private> nodeVar226 : f32;
var<private> nodeVar227 : f32;
var<private> nodeVar228 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar229 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar230 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar231 : vec3<f32>;
var<private> nodeVar232 : vec4<f32>;

// codes
fn mx_floor ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn mx_rotl32 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn mx_bjmix ( a : u32, b : u32, c : u32 ) -> vec3<u32> {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = a;
	nodeVar1 = b;
	nodeVar2 = c;
	nodeVar0 = ( nodeVar0 - nodeVar2 );
	nodeVar0 = ( nodeVar0 ^ mx_rotl32( nodeVar2, 4 ) );
	nodeVar2 = ( nodeVar2 + nodeVar1 );
	nodeVar1 = ( nodeVar1 - nodeVar0 );
	nodeVar1 = ( nodeVar1 ^ mx_rotl32( nodeVar0, 6 ) );
	nodeVar0 = ( nodeVar0 + nodeVar2 );
	nodeVar2 = ( nodeVar2 - nodeVar1 );
	nodeVar2 = ( nodeVar2 ^ mx_rotl32( nodeVar1, 8 ) );
	nodeVar1 = ( nodeVar1 + nodeVar0 );
	nodeVar0 = ( nodeVar0 - nodeVar2 );
	nodeVar0 = ( nodeVar0 ^ mx_rotl32( nodeVar2, 16 ) );
	nodeVar2 = ( nodeVar2 + nodeVar1 );
	nodeVar1 = ( nodeVar1 - nodeVar0 );
	nodeVar1 = ( nodeVar1 ^ mx_rotl32( nodeVar0, 19 ) );
	nodeVar0 = ( nodeVar0 + nodeVar2 );
	nodeVar2 = ( nodeVar2 - nodeVar1 );
	nodeVar2 = ( nodeVar2 ^ mx_rotl32( nodeVar1, 4 ) );
	nodeVar1 = ( nodeVar1 + nodeVar0 );

	return vec3<u32>( nodeVar0, nodeVar1, nodeVar2 );

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


fn mx_bits_to_01 ( bits : u32 ) -> f32 {

	var nodeVar0 : u32;

	nodeVar0 = bits;

	return ( f32( nodeVar0 ) / 4294967295.0 );

}


fn mx_worley_distance_1 ( p : vec3<f32>, x : i32, y : i32, z : i32, xoff : i32, yoff : i32, zoff : i32, jitter : f32, metric : i32 ) -> f32 {

	var nodeVar0 : i32;
	var nodeVar1 : f32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : i32;
	var nodeVar5 : i32;
	var nodeVar6 : i32;
	var nodeVar7 : i32;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : i32;
	var nodeVar11 : i32;
	var nodeVar12 : i32;
	var nodeVar13 : u32;
	var nodeVar14 : u32;
	var nodeVar15 : u32;
	var nodeVar16 : u32;
	var nodeVar17 : vec3<u32>;
	var nodeVar18 : vec3<f32>;
	var nodeVar19 : vec3<f32>;
	var nodeVar20 : vec3<f32>;

	nodeVar0 = metric;
	nodeVar1 = jitter;
	nodeVar2 = zoff;
	nodeVar3 = yoff;
	nodeVar4 = xoff;
	nodeVar5 = z;
	nodeVar6 = y;
	nodeVar7 = x;
	nodeVar8 = p;
	nodeVar9 = vec3<f32>( f32( ( nodeVar7 + nodeVar4 ) ), f32( ( nodeVar6 + nodeVar3 ) ), f32( ( nodeVar5 + nodeVar2 ) ) );
	nodeVar10 = i32( floor( nodeVar9.x ) );
	nodeVar11 = i32( floor( nodeVar9.y ) );
	nodeVar12 = i32( floor( nodeVar9.z ) );
	nodeVar13 = 3735928588u;
	nodeVar14 = 0u;
	nodeVar15 = 0u;
	nodeVar16 = 0u;
	nodeVar16 = nodeVar13;
	nodeVar15 = nodeVar16;
	nodeVar14 = nodeVar15;
	nodeVar14 = ( nodeVar14 + u32( nodeVar10 ) );
	nodeVar15 = ( nodeVar15 + u32( nodeVar11 ) );
	nodeVar16 = ( nodeVar16 + u32( nodeVar12 ) );
	nodeVar17 = mx_bjmix( nodeVar14, nodeVar15, nodeVar16 );
	nodeVar18 = vec3<f32>( mx_bits_to_01( mx_bjfinal( nodeVar17.x, nodeVar17.y, nodeVar17.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar17.x + 1u ), nodeVar17.y, nodeVar17.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar17.x + 2u ), nodeVar17.y, nodeVar17.z ) ) );
	nodeVar18 = ( nodeVar18 - vec3<f32>( 0.5 ) );
	nodeVar18 = ( nodeVar18 * vec3<f32>( nodeVar1 ) );
	nodeVar18 = ( nodeVar18 + vec3<f32>( 0.5 ) );
	nodeVar19 = ( vec3<f32>( f32( nodeVar7 ), f32( nodeVar6 ), f32( nodeVar5 ) ) + nodeVar18 );
	nodeVar20 = ( nodeVar19 - nodeVar8 );

	if ( ( nodeVar0 == 2 ) ) {

		return ( ( abs( nodeVar20.x ) + abs( nodeVar20.y ) ) + abs( nodeVar20.z ) );

	}


	if ( ( nodeVar0 == 3 ) ) {

		return max( max( abs( nodeVar20.x ), abs( nodeVar20.y ) ), abs( nodeVar20.z ) );

	}


	return dot( nodeVar20, nodeVar20 );

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


fn mx_cell_noise_float_2 ( p : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;

	nodeVar0 = p;
	nodeVar1 = mx_floor( nodeVar0.x );
	nodeVar2 = mx_floor( nodeVar0.y );
	nodeVar3 = mx_floor( nodeVar0.z );

	return mx_bits_to_01( mx_hash_int_2( nodeVar1, nodeVar2, nodeVar3 ) );

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

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	nodeVar0 = ( positionLocal * vec3<f32>( 5.0 ) );
	nodeVar1 = 1.0;
	nodeVar2 = 0;
	nodeVar3 = 0;
	nodeVar4 = 0;
	nodeVar5 = 0;
	nodeVar6 = nodeVar0.x;
	nodeVar3 = mx_floor( nodeVar6 );
	nodeVar7 = nodeVar0.y;
	nodeVar4 = mx_floor( nodeVar7 );
	nodeVar8 = nodeVar0.z;
	nodeVar5 = mx_floor( nodeVar8 );
	nodeVar9 = vec3<f32>( ( nodeVar6 - f32( nodeVar3 ) ), ( nodeVar7 - f32( nodeVar4 ) ), ( nodeVar8 - f32( nodeVar5 ) ) );
	nodeVar10 = 1000000.0;
	nodeVar11 = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {


			for ( var z : i32 = -1; z <= 1; z ++ ) {

				nodeVar12 = mx_worley_distance_1( nodeVar9, x, y, z, nodeVar3, nodeVar4, nodeVar5, nodeVar1, 0 );
				nodeVar13 = vec3<f32>( f32( ( nodeVar3 + x ) ), f32( ( nodeVar4 + y ) ), f32( ( nodeVar5 + z ) ) );
				nodeVar14 = i32( floor( nodeVar13.x ) );
				nodeVar15 = i32( floor( nodeVar13.y ) );
				nodeVar16 = i32( floor( nodeVar13.z ) );
				nodeVar17 = 3735928588u;
				nodeVar18 = 0u;
				nodeVar19 = 0u;
				nodeVar20 = 0u;
				nodeVar20 = nodeVar17;
				nodeVar19 = nodeVar20;
				nodeVar18 = nodeVar19;
				nodeVar18 = ( nodeVar18 + u32( nodeVar14 ) );
				nodeVar19 = ( nodeVar19 + u32( nodeVar15 ) );
				nodeVar20 = ( nodeVar20 + u32( nodeVar16 ) );
				nodeVar21 = mx_bjmix( nodeVar18, nodeVar19, nodeVar20 );
				nodeVar22 = vec3<f32>( mx_bits_to_01( mx_bjfinal( nodeVar21.x, nodeVar21.y, nodeVar21.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar21.x + 1u ), nodeVar21.y, nodeVar21.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar21.x + 2u ), nodeVar21.y, nodeVar21.z ) ) );
				nodeVar22 = ( nodeVar22 - vec3<f32>( 0.5 ) );
				nodeVar22 = ( nodeVar22 * vec3<f32>( nodeVar1 ) );
				nodeVar22 = ( nodeVar22 + vec3<f32>( 0.5 ) );
				nodeVar23 = ( ( vec3<f32>( f32( x ), f32( y ), f32( z ) ) + nodeVar22 ) - nodeVar9 );

				if ( ( nodeVar12 < nodeVar10 ) ) {

					nodeVar10 = nodeVar12;
					nodeVar11 = nodeVar23;
					

				}


			}


		}


	}


	if ( ( nodeVar2 == 1 ) ) {

		nodeVar10 = mx_cell_noise_float_2( ( nodeVar11 + nodeVar0 ) );
		

	} else {

		nodeVar10 = sqrt( nodeVar10 );
		

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar24 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( nodeVar10 * 0.18 ) + 0.12 ), 0.0525 ) + max( max( nodeVar24.x, nodeVar24.y ), nodeVar24.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	nodeVar25 = normalize( dpdx( v_positionView ) );
	NORMAL_normalView = normalViewGeometry;
	nodeVar26 = cross( normalize( - dpdy( v_positionView ) ), NORMAL_normalView );
	nodeVar27 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	nodeVar28 = ( dot( nodeVar25, nodeVar26 ) * nodeVar27 );
	nodeVar29 = ( vec2<f32>( ( ( nodeVar10 * 0.1 ) - ( nodeVar10 * 0.1 ) ), ( ( nodeVar10 * 0.1 ) - ( nodeVar10 * 0.1 ) ) ) * vec2<f32>( 1.0 ) );
	normalView = normalize( ( ( vec3<f32>( abs( nodeVar28 ) ) * NORMAL_normalView ) - ( vec3<f32>( sign( nodeVar28 ) ) * ( ( vec3<f32>( nodeVar29.x ) * nodeVar26 ) + ( vec3<f32>( nodeVar29.y ) * cross( NORMAL_normalView, nodeVar25 ) ) ) ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar30 = dot( normalView, positionViewDirection );
	nodeVar31 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( nodeVar30, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar31;
	nodeVar32 = ( dfg.x + dfg.y );
	nodeVar33 = ( 1.0 / nodeVar32 );
	nodeVar34 = nodeVar33;
	nodeVar35 = ( nodeVar34 - 1.0 );
	nodeVar36 = ( SpecularColorBlended * vec3<f32>( nodeVar35 ) );
	nodeVar37 = ( nodeVar36 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar37;
	nodeVar38 = vec4<f32>( render.nodeUniform10, 0.0 );
	nodeVar39 = ( render.cameraViewMatrix * nodeVar38 );
	nodeVar40 = normalize( nodeVar39.xyz );
	nodeVar41 = nodeVar40;
	nodeVar42 = dot( normalView, nodeVar41 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar43 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform12 ) ) ), 1.0 );
	nodeVar44 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar44 >= render.nodeUniform13.x ) && ( nodeVar44 < render.nodeUniform13.y ) ) ) {

		nodeVar46 = ( render.nodeUniform14 * nodeVar43 );
		nodeVar47 = ( nodeVar46.xyz / vec3<f32>( nodeVar46.w ) );
		nodeVar48 = vec3<f32>( nodeVar47.x, ( 1.0 - nodeVar47.y ), ( nodeVar47.z + render.nodeUniform15 ) );

		if ( ( ( ( ( ( nodeVar48.x >= 0.0 ) && ( nodeVar48.x <= 1.0 ) ) && ( nodeVar48.y >= 0.0 ) ) && ( nodeVar48.y <= 1.0 ) ) && ( nodeVar48.z <= 1.0 ) ) ) {

			nodeVar49 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar50 = ( render.nodeUniform17 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ).x );
			nodeVar51 = ( nodeVar48.xy + ( vogelDiskSample( 0, 5, nodeVar49 ) * vec2<f32>( nodeVar50 ) ) );
			nodeVar52 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar51, nodeVar48.z );
			nodeVar53 = ( nodeVar48.xy + ( vogelDiskSample( 1, 5, nodeVar49 ) * vec2<f32>( nodeVar50 ) ) );
			nodeVar54 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar53, nodeVar48.z );
			nodeVar55 = ( nodeVar48.xy + ( vogelDiskSample( 2, 5, nodeVar49 ) * vec2<f32>( nodeVar50 ) ) );
			nodeVar56 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar55, nodeVar48.z );
			nodeVar57 = ( nodeVar48.xy + ( vogelDiskSample( 3, 5, nodeVar49 ) * vec2<f32>( nodeVar50 ) ) );
			nodeVar58 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar57, nodeVar48.z );
			nodeVar59 = ( nodeVar48.xy + ( vogelDiskSample( 4, 5, nodeVar49 ) * vec2<f32>( nodeVar50 ) ) );
			nodeVar60 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar59, nodeVar48.z );
			nodeVar45 = ( ( ( ( ( nodeVar52 + nodeVar54 ) + nodeVar56 ) + nodeVar58 ) + nodeVar60 ) * 0.2 );

		} else {

			nodeVar45 = 1.0;

		}

		shadowValue = mix( nodeVar45, shadowValue, smoothstep( render.nodeUniform13.z, render.nodeUniform13.y, nodeVar44 ) );
		

	}


	if ( ( ( nodeVar44 >= render.nodeUniform19.x ) && ( nodeVar44 < render.nodeUniform19.y ) ) ) {

		nodeVar62 = ( render.nodeUniform20 * nodeVar43 );
		nodeVar63 = ( nodeVar62.xyz / vec3<f32>( nodeVar62.w ) );
		nodeVar64 = vec3<f32>( nodeVar63.x, ( 1.0 - nodeVar63.y ), ( nodeVar63.z + render.nodeUniform21 ) );

		if ( ( ( ( ( ( nodeVar64.x >= 0.0 ) && ( nodeVar64.x <= 1.0 ) ) && ( nodeVar64.y >= 0.0 ) ) && ( nodeVar64.y <= 1.0 ) ) && ( nodeVar64.z <= 1.0 ) ) ) {

			nodeVar65 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar66 = ( render.nodeUniform22 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform23 ).x );
			nodeVar67 = ( nodeVar64.xy + ( vogelDiskSample( 0, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar68 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar67, nodeVar64.z );
			nodeVar69 = ( nodeVar64.xy + ( vogelDiskSample( 1, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar70 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar69, nodeVar64.z );
			nodeVar71 = ( nodeVar64.xy + ( vogelDiskSample( 2, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar72 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar71, nodeVar64.z );
			nodeVar73 = ( nodeVar64.xy + ( vogelDiskSample( 3, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar74 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar73, nodeVar64.z );
			nodeVar75 = ( nodeVar64.xy + ( vogelDiskSample( 4, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar76 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar75, nodeVar64.z );
			nodeVar61 = ( ( ( ( ( nodeVar68 + nodeVar70 ) + nodeVar72 ) + nodeVar74 ) + nodeVar76 ) * 0.2 );

		} else {

			nodeVar61 = 1.0;

		}

		shadowValue = mix( nodeVar61, shadowValue, smoothstep( render.nodeUniform19.z, render.nodeUniform19.y, nodeVar44 ) );
		

	}

	nodeVar77 = mix( 1.0, shadowValue, render.nodeUniform24 );
	nodeVar78 = ( vec3<f32>( clamp( nodeVar42, 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar77 ) ) );
	nodeVar79 = nodeVar78;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar80 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar81 = ( nodeVar79 * nodeVar80 );
	nodeVar82 = ( nodeVar41 + positionViewDirection );
	nodeVar83 = normalize( nodeVar82 );
	nodeVar84 = dot( positionViewDirection, nodeVar83 );
	nodeVar85 = clamp( nodeVar84, 0.0, 1.0 );
	nodeVar86 = exp2( ( ( ( nodeVar85 * -5.55473 ) - 6.98316 ) * nodeVar85 ) );
	nodeVar87 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar86 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar86 ) ) );
	nodeVar88 = ( vec3<f32>( 1.0 ) - nodeVar87 );
	nodeVar89 = nodeVar88;
	nodeVar90 = ( nodeVar81 * nodeVar89 );
	nodeVar91 = ( directDiffuse + nodeVar90 );
	directDiffuse = nodeVar91;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar92 = normalize( ( nodeVar41 + positionViewDirection ) );
	nodeVar93 = clamp( dot( positionViewDirection, nodeVar92 ), 0.0, 1.0 );
	nodeVar94 = exp2( ( ( ( nodeVar93 * -5.55473 ) - 6.98316 ) * nodeVar93 ) );
	nodeVar95 = ( Roughness * Roughness );
	nodeVar96 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar94 ) ) ) + vec3<f32>( ( 1.0 * nodeVar94 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar95, clamp( dot( normalView, nodeVar41 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar95, clamp( dot( normalView, nodeVar92 ), 0.0, 1.0 ) ) ) );
	nodeVar97 = ( nodeVar79 * nodeVar96 );
	nodeVar98 = ( nodeVar97 * multiScatteringCompensation );
	nodeVar99 = ( directSpecular + nodeVar98 );
	directSpecular = nodeVar99;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar100 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform25 );
	nodeVar101 = floor( nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar104 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar103.x, ( - nodeVar103.y ), nodeVar103.z ), 1.0 ) ).xyz );
	nodeVar105 = max( ( 4.0 - nodeVar102 ), 0.0 );
	nodeVar102 = max( nodeVar102, 4.0 );
	nodeVar106 = exp2( nodeVar102 );
	nodeVar107 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar103.x, ( - nodeVar103.y ), nodeVar103.z ), 1.0 ) ).xyz, nodeVar104 ) * vec2<f32>( ( nodeVar106 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar104 > 2.0 ) ) {

		nodeVar107.y = ( nodeVar107.y + nodeVar106 );
		nodeVar104 = ( nodeVar104 - 3.0 );
		

	}

	nodeVar107.x = ( nodeVar107.x + ( nodeVar104 * nodeVar106 ) );
	nodeVar107.x = ( nodeVar107.x + ( nodeVar105 * ( 3.0 * 16.0 ) ) );
	nodeVar107.y = ( nodeVar107.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar106 ) ) );
	nodeVar107.x = ( nodeVar107.x * object.nodeUniform28 );
	nodeVar107.y = ( nodeVar107.y * object.nodeUniform29 );
	nodeVar108 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar107, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar109 = nodeVar108.xyz;
	nodeVar110 = fract( nodeVar100 );

	if ( ( nodeVar110 != 0.0 ) ) {

		nodeVar111 = ( nodeVar101 + 1.0 );
		nodeVar112 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar103.x, ( - nodeVar103.y ), nodeVar103.z ), 1.0 ) ).xyz );
		nodeVar113 = max( ( 4.0 - nodeVar111 ), 0.0 );
		nodeVar111 = max( nodeVar111, 4.0 );
		nodeVar114 = exp2( nodeVar111 );
		nodeVar115 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( nodeVar103.x, ( - nodeVar103.y ), nodeVar103.z ), 1.0 ) ).xyz, nodeVar112 ) * vec2<f32>( ( nodeVar114 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar112 > 2.0 ) ) {

			nodeVar115.y = ( nodeVar115.y + nodeVar114 );
			nodeVar112 = ( nodeVar112 - 3.0 );
			

		}

		nodeVar115.x = ( nodeVar115.x + ( nodeVar112 * nodeVar114 ) );
		nodeVar115.x = ( nodeVar115.x + ( nodeVar113 * ( 3.0 * 16.0 ) ) );
		nodeVar115.y = ( nodeVar115.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar114 ) ) );
		nodeVar115.x = ( nodeVar115.x * object.nodeUniform28 );
		nodeVar115.y = ( nodeVar115.y * object.nodeUniform29 );
		nodeVar116 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar115, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar117 = nodeVar116.xyz;
		nodeVar109 = mix( nodeVar109, nodeVar117, nodeVar110 );
		

	}

	nodeVar118 = ( radiance + ( nodeVar109 * vec3<f32>( object.nodeUniform31 ) ) );
	radiance = nodeVar118;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar119 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform25 );
	nodeVar120 = floor( nodeVar119 );
	nodeVar121 = nodeVar120;
	nodeVar122 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar123 = max( ( 4.0 - nodeVar121 ), 0.0 );
	nodeVar121 = max( nodeVar121, 4.0 );
	nodeVar124 = exp2( nodeVar121 );
	nodeVar125 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar122 ) * vec2<f32>( ( nodeVar124 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar122 > 2.0 ) ) {

		nodeVar125.y = ( nodeVar125.y + nodeVar124 );
		nodeVar122 = ( nodeVar122 - 3.0 );
		

	}

	nodeVar125.x = ( nodeVar125.x + ( nodeVar122 * nodeVar124 ) );
	nodeVar125.x = ( nodeVar125.x + ( nodeVar123 * ( 3.0 * 16.0 ) ) );
	nodeVar125.y = ( nodeVar125.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar124 ) ) );
	nodeVar125.x = ( nodeVar125.x * object.nodeUniform28 );
	nodeVar125.y = ( nodeVar125.y * object.nodeUniform29 );
	nodeVar126 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar125, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar127 = nodeVar126.xyz;
	nodeVar128 = fract( nodeVar119 );

	if ( ( nodeVar128 != 0.0 ) ) {

		nodeVar129 = ( nodeVar120 + 1.0 );
		nodeVar130 = getFace( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar131 = max( ( 4.0 - nodeVar129 ), 0.0 );
		nodeVar129 = max( nodeVar129, 4.0 );
		nodeVar132 = exp2( nodeVar129 );
		nodeVar133 = ( ( getUV( ( object.nodeUniform26 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar130 ) * vec2<f32>( ( nodeVar132 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar130 > 2.0 ) ) {

			nodeVar133.y = ( nodeVar133.y + nodeVar132 );
			nodeVar130 = ( nodeVar130 - 3.0 );
			

		}

		nodeVar133.x = ( nodeVar133.x + ( nodeVar130 * nodeVar132 ) );
		nodeVar133.x = ( nodeVar133.x + ( nodeVar131 * ( 3.0 * 16.0 ) ) );
		nodeVar133.y = ( nodeVar133.y + ( 4.0 * ( exp2( object.nodeUniform25 ) - nodeVar132 ) ) );
		nodeVar133.x = ( nodeVar133.x * object.nodeUniform28 );
		nodeVar133.y = ( nodeVar133.y * object.nodeUniform29 );
		nodeVar134 = textureSampleGrad( nodeUniform30, nodeUniform30_sampler, nodeVar133, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar135 = nodeVar134.xyz;
		nodeVar127 = mix( nodeVar127, nodeVar135, nodeVar128 );
		

	}

	nodeVar136 = ( iblIrradiance + ( ( nodeVar127 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform31 ) ) );
	iblIrradiance = nodeVar136;
	nodeVar137 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar138 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar139 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar140 = ( SpecularF90 * dfg.y );
	nodeVar141 = ( nodeVar139 + vec3<f32>( nodeVar140 ) );
	nodeVar142 = ( nodeVar137 + nodeVar141 );
	nodeVar137 = nodeVar142;
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
	nodeVar156 = ( nodeVar138 + nodeVar155 );
	nodeVar138 = nodeVar156;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar157 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar158 = ( irradiance * nodeVar157 );
	nodeVar159 = ( nodeVar137 + nodeVar138 );
	nodeVar160 = ( vec3<f32>( 1.0 ) - nodeVar159 );
	nodeVar161 = nodeVar160;
	nodeVar162 = ( nodeVar158 * nodeVar161 );
	nodeVar163 = nodeVar162;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar164 = ( indirectDiffuse + nodeVar163 );
	indirectDiffuse = nodeVar164;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar165 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar166 = ( SpecularF90 * dfg.y );
	nodeVar167 = ( nodeVar165 + vec3<f32>( nodeVar166 ) );
	nodeVar168 = ( singleScatteringDielectric + nodeVar167 );
	singleScatteringDielectric = nodeVar168;
	nodeVar169 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar170 = nodeVar169;
	nodeVar171 = ( nodeVar170 * vec3<f32>( 0.047619 ) );
	nodeVar172 = ( SpecularColor + nodeVar171 );
	nodeVar173 = ( nodeVar167 * nodeVar172 );
	nodeVar174 = ( dfg.x + dfg.y );
	nodeVar175 = ( 1.0 - nodeVar174 );
	nodeVar176 = nodeVar175;
	nodeVar177 = ( vec3<f32>( nodeVar176 ) * nodeVar172 );
	nodeVar178 = ( vec3<f32>( 1.0 ) - nodeVar177 );
	nodeVar179 = nodeVar178;
	nodeVar180 = ( nodeVar173 / nodeVar179 );
	nodeVar181 = ( nodeVar180 * vec3<f32>( nodeVar176 ) );
	nodeVar182 = ( multiScatteringDielectric + nodeVar181 );
	multiScatteringDielectric = nodeVar182;
	nodeVar183 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar184 = ( SpecularF90 * dfg.y );
	nodeVar185 = ( nodeVar183 + vec3<f32>( nodeVar184 ) );
	nodeVar186 = ( singleScatteringMetallic + nodeVar185 );
	singleScatteringMetallic = nodeVar186;
	nodeVar187 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar188 = nodeVar187;
	nodeVar189 = ( nodeVar188 * vec3<f32>( 0.047619 ) );
	nodeVar190 = ( DiffuseColor.xyz + nodeVar189 );
	nodeVar191 = ( nodeVar185 * nodeVar190 );
	nodeVar192 = ( dfg.x + dfg.y );
	nodeVar193 = ( 1.0 - nodeVar192 );
	nodeVar194 = nodeVar193;
	nodeVar195 = ( vec3<f32>( nodeVar194 ) * nodeVar190 );
	nodeVar196 = ( vec3<f32>( 1.0 ) - nodeVar195 );
	nodeVar197 = nodeVar196;
	nodeVar198 = ( nodeVar191 / nodeVar197 );
	nodeVar199 = ( nodeVar198 * vec3<f32>( nodeVar194 ) );
	nodeVar200 = ( multiScatteringMetallic + nodeVar199 );
	multiScatteringMetallic = nodeVar200;
	nodeVar201 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar202 = ( radiance * nodeVar201 );
	nodeVar203 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar204 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar205 = ( nodeVar203 * nodeVar204 );
	nodeVar206 = ( nodeVar202 + nodeVar205 );
	nodeVar207 = nodeVar206;
	nodeVar208 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar209 = ( vec3<f32>( 1.0 ) - nodeVar208 );
	nodeVar210 = nodeVar209;
	nodeVar211 = ( DiffuseContribution * nodeVar210 );
	nodeVar212 = ( nodeVar211 * nodeVar204 );
	nodeVar213 = nodeVar212;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar214 = ( indirectSpecular + nodeVar207 );
	indirectSpecular = nodeVar214;
	nodeVar215 = ( indirectDiffuse + nodeVar213 );
	indirectDiffuse = nodeVar215;
	ambientOcclusion = 1.0;
	nodeVar216 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar216;
	nodeVar217 = dot( normalView, positionViewDirection );
	nodeVar218 = ( clamp( nodeVar217, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar219 = ( Roughness * -16.0 );
	nodeVar220 = ( 1.0 - nodeVar219 );
	nodeVar221 = nodeVar220;
	nodeVar222 = ( - nodeVar221 );
	nodeVar223 = exp2( nodeVar222 );
	nodeVar224 = pow( nodeVar218, nodeVar223 );
	nodeVar225 = ( 1.0 - nodeVar224 );
	nodeVar226 = nodeVar225;
	nodeVar227 = ( ambientOcclusion - nodeVar226 );
	nodeVar228 = ( indirectSpecular * vec3<f32>( clamp( nodeVar227, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar228;
	nodeVar229 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar229;
	nodeVar230 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar230;
	nodeVar231 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar231;
	nodeVar232 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar232;

	// result

	output.color = nodeVar232;

	return output;

}
