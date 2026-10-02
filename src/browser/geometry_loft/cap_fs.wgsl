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
@binding( 5 ) @group( 1 ) var nodeUniform29_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform29 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat4x4<f32>,
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
	nodeUniform10 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform12 : vec4<f32>,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform18 : vec4<f32>,
	nodeUniform11 : f32,
	nodeUniform14 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : vec2<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : vec2<f32>,
	nodeUniform23 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : i32;
var<private> nodeVar5 : i32;
var<private> nodeVar6 : i32;
var<private> nodeVar7 : i32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : i32;
var<private> nodeVar17 : i32;
var<private> nodeVar18 : i32;
var<private> nodeVar19 : u32;
var<private> nodeVar20 : u32;
var<private> nodeVar21 : u32;
var<private> nodeVar22 : u32;
var<private> nodeVar23 : vec3<u32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : f32;
var<private> shadowValue : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec4<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec2<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec2<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec2<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec2<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec2<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec4<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec2<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec2<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec2<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : vec2<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec2<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec2<f32>;
var<private> nodeVar111 : vec4<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : vec2<f32>;
var<private> nodeVar119 : vec4<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
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
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : f32;
var<private> nodeVar221 : f32;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : f32;
var<private> nodeVar224 : f32;
var<private> nodeVar225 : f32;
var<private> nodeVar226 : f32;
var<private> nodeVar227 : f32;
var<private> nodeVar228 : f32;
var<private> nodeVar229 : f32;
var<private> nodeVar230 : f32;
var<private> nodeVar231 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar233 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : vec4<f32>;

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
	@location( 5 ) nodeVarying7 : vec2<f32>,
	@builtin( front_facing ) isFront : bool,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = smoothstep( 0.42, 0.58, nodeVarying7.x );
	nodeVar1 = ( ( ( sin( ( nodeVarying7.y * 376.99111843077515 ) ) * 0.5 ) + 0.5 ) * ( 1.0 - nodeVar0 ) );
	nodeVar2 = ( positionLocal * vec3<f32>( 2.4 ) );
	nodeVar3 = 1.0;
	nodeVar4 = 0;
	nodeVar5 = 0;
	nodeVar6 = 0;
	nodeVar7 = 0;
	nodeVar8 = nodeVar2.x;
	nodeVar5 = mx_floor( nodeVar8 );
	nodeVar9 = nodeVar2.y;
	nodeVar6 = mx_floor( nodeVar9 );
	nodeVar10 = nodeVar2.z;
	nodeVar7 = mx_floor( nodeVar10 );
	nodeVar11 = vec3<f32>( ( nodeVar8 - f32( nodeVar5 ) ), ( nodeVar9 - f32( nodeVar6 ) ), ( nodeVar10 - f32( nodeVar7 ) ) );
	nodeVar12 = 1000000.0;
	nodeVar13 = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {


			for ( var z : i32 = -1; z <= 1; z ++ ) {

				nodeVar14 = mx_worley_distance_1( nodeVar11, x, y, z, nodeVar5, nodeVar6, nodeVar7, nodeVar3, 0 );
				nodeVar15 = vec3<f32>( f32( ( nodeVar5 + x ) ), f32( ( nodeVar6 + y ) ), f32( ( nodeVar7 + z ) ) );
				nodeVar16 = i32( floor( nodeVar15.x ) );
				nodeVar17 = i32( floor( nodeVar15.y ) );
				nodeVar18 = i32( floor( nodeVar15.z ) );
				nodeVar19 = 3735928588u;
				nodeVar20 = 0u;
				nodeVar21 = 0u;
				nodeVar22 = 0u;
				nodeVar22 = nodeVar19;
				nodeVar21 = nodeVar22;
				nodeVar20 = nodeVar21;
				nodeVar20 = ( nodeVar20 + u32( nodeVar16 ) );
				nodeVar21 = ( nodeVar21 + u32( nodeVar17 ) );
				nodeVar22 = ( nodeVar22 + u32( nodeVar18 ) );
				nodeVar23 = mx_bjmix( nodeVar20, nodeVar21, nodeVar22 );
				nodeVar24 = vec3<f32>( mx_bits_to_01( mx_bjfinal( nodeVar23.x, nodeVar23.y, nodeVar23.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar23.x + 1u ), nodeVar23.y, nodeVar23.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar23.x + 2u ), nodeVar23.y, nodeVar23.z ) ) );
				nodeVar24 = ( nodeVar24 - vec3<f32>( 0.5 ) );
				nodeVar24 = ( nodeVar24 * vec3<f32>( nodeVar3 ) );
				nodeVar24 = ( nodeVar24 + vec3<f32>( 0.5 ) );
				nodeVar25 = ( ( vec3<f32>( f32( x ), f32( y ), f32( z ) ) + nodeVar24 ) - nodeVar11 );

				if ( ( nodeVar14 < nodeVar12 ) ) {

					nodeVar12 = nodeVar14;
					nodeVar13 = nodeVar25;
					

				}


			}


		}


	}


	if ( ( nodeVar4 == 1 ) ) {

		nodeVar12 = mx_cell_noise_float_2( ( nodeVar13 + nodeVar2 ) );
		

	} else {

		nodeVar12 = sqrt( nodeVar12 );
		

	}

	nodeVar26 = ( ( 1.0 - smoothstep( 0.18, 0.38, nodeVar12 ) ) * nodeVar0 );
	DiffuseColor = vec4<f32>( mix( mix( vec3<f32>( 0.8069522576650873, 0.7156935005005721, 0.55201140150344 ), vec3<f32>( 0.5209955731953768, 0.4232676699760063, 0.2704977910022518 ), nodeVar1 ), mix( vec3<f32>( 0.36625259558833256, 0.026241221889696346, 0.014443843592229466 ), vec3<f32>( 0.8879231178794776, 0.8148465722120952, 0.6866853124288864 ), nodeVar26 ), nodeVar0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar27 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( 0.55 - ( nodeVar0 * 0.2 ) ) + ( nodeVar26 * 0.25 ) ), 0.0525 ) + max( max( nodeVar27.x, nodeVar27.y ), nodeVar27.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	nodeVar28 = normalize( dpdx( v_positionView ) );
	NORMAL_normalView = normalViewGeometry;
	nodeVar29 = cross( normalize( - dpdy( v_positionView ) ), NORMAL_normalView );
	nodeVar30 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	nodeVar31 = ( dot( nodeVar28, nodeVar29 ) * nodeVar30 );
	nodeVar32 = ( vec2<f32>( ( ( ( nodeVar26 * 0.08 ) - ( nodeVar1 * 0.015 ) ) - ( ( nodeVar26 * 0.08 ) - ( nodeVar1 * 0.015 ) ) ), ( ( ( nodeVar26 * 0.08 ) - ( nodeVar1 * 0.015 ) ) - ( ( nodeVar26 * 0.08 ) - ( nodeVar1 * 0.015 ) ) ) ) * vec2<f32>( 1.0 ) );
	normalView = normalize( ( ( vec3<f32>( abs( nodeVar31 ) ) * NORMAL_normalView ) - ( vec3<f32>( sign( nodeVar31 ) ) * ( ( vec3<f32>( nodeVar32.x ) * nodeVar29 ) + ( vec3<f32>( nodeVar32.y ) * cross( NORMAL_normalView, nodeVar28 ) ) ) ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar33 = dot( normalView, positionViewDirection );
	nodeVar34 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( nodeVar33, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar34;
	nodeVar35 = ( dfg.x + dfg.y );
	nodeVar36 = ( 1.0 / nodeVar35 );
	nodeVar37 = nodeVar36;
	nodeVar38 = ( nodeVar37 - 1.0 );
	nodeVar39 = ( SpecularColorBlended * vec3<f32>( nodeVar38 ) );
	nodeVar40 = ( nodeVar39 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar40;
	nodeVar41 = vec4<f32>( render.nodeUniform9, 0.0 );
	nodeVar42 = ( render.cameraViewMatrix * nodeVar41 );
	nodeVar43 = normalize( nodeVar42.xyz );
	nodeVar44 = nodeVar43;
	nodeVar45 = dot( normalView, nodeVar44 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar46 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform11 ) ) ), 1.0 );
	nodeVar47 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar47 >= render.nodeUniform12.x ) && ( nodeVar47 < render.nodeUniform12.y ) ) ) {

		nodeVar49 = ( render.nodeUniform13 * nodeVar46 );
		nodeVar50 = ( nodeVar49.xyz / vec3<f32>( nodeVar49.w ) );
		nodeVar51 = vec3<f32>( nodeVar50.x, ( 1.0 - nodeVar50.y ), ( nodeVar50.z + render.nodeUniform14 ) );

		if ( ( ( ( ( ( nodeVar51.x >= 0.0 ) && ( nodeVar51.x <= 1.0 ) ) && ( nodeVar51.y >= 0.0 ) ) && ( nodeVar51.y <= 1.0 ) ) && ( nodeVar51.z <= 1.0 ) ) ) {

			nodeVar52 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar53 = ( render.nodeUniform16 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ).x );
			nodeVar54 = ( nodeVar51.xy + ( vogelDiskSample( 0, 5, nodeVar52 ) * vec2<f32>( nodeVar53 ) ) );
			nodeVar55 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar54, nodeVar51.z );
			nodeVar56 = ( nodeVar51.xy + ( vogelDiskSample( 1, 5, nodeVar52 ) * vec2<f32>( nodeVar53 ) ) );
			nodeVar57 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar56, nodeVar51.z );
			nodeVar58 = ( nodeVar51.xy + ( vogelDiskSample( 2, 5, nodeVar52 ) * vec2<f32>( nodeVar53 ) ) );
			nodeVar59 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar58, nodeVar51.z );
			nodeVar60 = ( nodeVar51.xy + ( vogelDiskSample( 3, 5, nodeVar52 ) * vec2<f32>( nodeVar53 ) ) );
			nodeVar61 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar60, nodeVar51.z );
			nodeVar62 = ( nodeVar51.xy + ( vogelDiskSample( 4, 5, nodeVar52 ) * vec2<f32>( nodeVar53 ) ) );
			nodeVar63 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar62, nodeVar51.z );
			nodeVar48 = ( ( ( ( ( nodeVar55 + nodeVar57 ) + nodeVar59 ) + nodeVar61 ) + nodeVar63 ) * 0.2 );

		} else {

			nodeVar48 = 1.0;

		}

		shadowValue = mix( nodeVar48, shadowValue, smoothstep( render.nodeUniform12.z, render.nodeUniform12.y, nodeVar47 ) );
		

	}


	if ( ( ( nodeVar47 >= render.nodeUniform18.x ) && ( nodeVar47 < render.nodeUniform18.y ) ) ) {

		nodeVar65 = ( render.nodeUniform19 * nodeVar46 );
		nodeVar66 = ( nodeVar65.xyz / vec3<f32>( nodeVar65.w ) );
		nodeVar67 = vec3<f32>( nodeVar66.x, ( 1.0 - nodeVar66.y ), ( nodeVar66.z + render.nodeUniform20 ) );

		if ( ( ( ( ( ( nodeVar67.x >= 0.0 ) && ( nodeVar67.x <= 1.0 ) ) && ( nodeVar67.y >= 0.0 ) ) && ( nodeVar67.y <= 1.0 ) ) && ( nodeVar67.z <= 1.0 ) ) ) {

			nodeVar68 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar69 = ( render.nodeUniform21 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform22 ).x );
			nodeVar70 = ( nodeVar67.xy + ( vogelDiskSample( 0, 5, nodeVar68 ) * vec2<f32>( nodeVar69 ) ) );
			nodeVar71 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar70, nodeVar67.z );
			nodeVar72 = ( nodeVar67.xy + ( vogelDiskSample( 1, 5, nodeVar68 ) * vec2<f32>( nodeVar69 ) ) );
			nodeVar73 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar72, nodeVar67.z );
			nodeVar74 = ( nodeVar67.xy + ( vogelDiskSample( 2, 5, nodeVar68 ) * vec2<f32>( nodeVar69 ) ) );
			nodeVar75 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar74, nodeVar67.z );
			nodeVar76 = ( nodeVar67.xy + ( vogelDiskSample( 3, 5, nodeVar68 ) * vec2<f32>( nodeVar69 ) ) );
			nodeVar77 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar76, nodeVar67.z );
			nodeVar78 = ( nodeVar67.xy + ( vogelDiskSample( 4, 5, nodeVar68 ) * vec2<f32>( nodeVar69 ) ) );
			nodeVar79 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar78, nodeVar67.z );
			nodeVar64 = ( ( ( ( ( nodeVar71 + nodeVar73 ) + nodeVar75 ) + nodeVar77 ) + nodeVar79 ) * 0.2 );

		} else {

			nodeVar64 = 1.0;

		}

		shadowValue = mix( nodeVar64, shadowValue, smoothstep( render.nodeUniform18.z, render.nodeUniform18.y, nodeVar47 ) );
		

	}

	nodeVar80 = mix( 1.0, shadowValue, render.nodeUniform23 );
	nodeVar81 = ( vec3<f32>( clamp( nodeVar45, 0.0, 1.0 ) ) * ( render.nodeUniform10 * vec3<f32>( nodeVar80 ) ) );
	nodeVar82 = nodeVar81;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar83 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar84 = ( nodeVar82 * nodeVar83 );
	nodeVar85 = ( nodeVar44 + positionViewDirection );
	nodeVar86 = normalize( nodeVar85 );
	nodeVar87 = dot( positionViewDirection, nodeVar86 );
	nodeVar88 = clamp( nodeVar87, 0.0, 1.0 );
	nodeVar89 = exp2( ( ( ( nodeVar88 * -5.55473 ) - 6.98316 ) * nodeVar88 ) );
	nodeVar90 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar89 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar89 ) ) );
	nodeVar91 = ( vec3<f32>( 1.0 ) - nodeVar90 );
	nodeVar92 = nodeVar91;
	nodeVar93 = ( nodeVar84 * nodeVar92 );
	nodeVar94 = ( directDiffuse + nodeVar93 );
	directDiffuse = nodeVar94;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar95 = normalize( ( nodeVar44 + positionViewDirection ) );
	nodeVar96 = clamp( dot( positionViewDirection, nodeVar95 ), 0.0, 1.0 );
	nodeVar97 = exp2( ( ( ( nodeVar96 * -5.55473 ) - 6.98316 ) * nodeVar96 ) );
	nodeVar98 = ( Roughness * Roughness );
	nodeVar99 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar97 ) ) ) + vec3<f32>( ( 1.0 * nodeVar97 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar98, clamp( dot( normalView, nodeVar44 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar98, clamp( dot( normalView, nodeVar95 ), 0.0, 1.0 ) ) ) );
	nodeVar100 = ( nodeVar82 * nodeVar99 );
	nodeVar101 = ( nodeVar100 * multiScatteringCompensation );
	nodeVar102 = ( directSpecular + nodeVar101 );
	directSpecular = nodeVar102;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar103 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform24 );
	nodeVar104 = floor( nodeVar103 );
	nodeVar105 = nodeVar104;
	nodeVar106 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar107 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar106.x, ( - nodeVar106.y ), nodeVar106.z ), 1.0 ) ).xyz );
	nodeVar108 = max( ( 4.0 - nodeVar105 ), 0.0 );
	nodeVar105 = max( nodeVar105, 4.0 );
	nodeVar109 = exp2( nodeVar105 );
	nodeVar110 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar106.x, ( - nodeVar106.y ), nodeVar106.z ), 1.0 ) ).xyz, nodeVar107 ) * vec2<f32>( ( nodeVar109 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar107 > 2.0 ) ) {

		nodeVar110.y = ( nodeVar110.y + nodeVar109 );
		nodeVar107 = ( nodeVar107 - 3.0 );
		

	}

	nodeVar110.x = ( nodeVar110.x + ( nodeVar107 * nodeVar109 ) );
	nodeVar110.x = ( nodeVar110.x + ( nodeVar108 * ( 3.0 * 16.0 ) ) );
	nodeVar110.y = ( nodeVar110.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar109 ) ) );
	nodeVar110.x = ( nodeVar110.x * object.nodeUniform27 );
	nodeVar110.y = ( nodeVar110.y * object.nodeUniform28 );
	nodeVar111 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar110, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar112 = nodeVar111.xyz;
	nodeVar113 = fract( nodeVar103 );

	if ( ( nodeVar113 != 0.0 ) ) {

		nodeVar114 = ( nodeVar104 + 1.0 );
		nodeVar115 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar106.x, ( - nodeVar106.y ), nodeVar106.z ), 1.0 ) ).xyz );
		nodeVar116 = max( ( 4.0 - nodeVar114 ), 0.0 );
		nodeVar114 = max( nodeVar114, 4.0 );
		nodeVar117 = exp2( nodeVar114 );
		nodeVar118 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar106.x, ( - nodeVar106.y ), nodeVar106.z ), 1.0 ) ).xyz, nodeVar115 ) * vec2<f32>( ( nodeVar117 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar115 > 2.0 ) ) {

			nodeVar118.y = ( nodeVar118.y + nodeVar117 );
			nodeVar115 = ( nodeVar115 - 3.0 );
			

		}

		nodeVar118.x = ( nodeVar118.x + ( nodeVar115 * nodeVar117 ) );
		nodeVar118.x = ( nodeVar118.x + ( nodeVar116 * ( 3.0 * 16.0 ) ) );
		nodeVar118.y = ( nodeVar118.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar117 ) ) );
		nodeVar118.x = ( nodeVar118.x * object.nodeUniform27 );
		nodeVar118.y = ( nodeVar118.y * object.nodeUniform28 );
		nodeVar119 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar118, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar120 = nodeVar119.xyz;
		nodeVar112 = mix( nodeVar112, nodeVar120, nodeVar113 );
		

	}

	nodeVar121 = ( radiance + ( nodeVar112 * vec3<f32>( object.nodeUniform30 ) ) );
	radiance = nodeVar121;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar122 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform24 );
	nodeVar123 = floor( nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar126 = max( ( 4.0 - nodeVar124 ), 0.0 );
	nodeVar124 = max( nodeVar124, 4.0 );
	nodeVar127 = exp2( nodeVar124 );
	nodeVar128 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar125 ) * vec2<f32>( ( nodeVar127 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

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
	nodeVar131 = fract( nodeVar122 );

	if ( ( nodeVar131 != 0.0 ) ) {

		nodeVar132 = ( nodeVar123 + 1.0 );
		nodeVar133 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar134 = max( ( 4.0 - nodeVar132 ), 0.0 );
		nodeVar132 = max( nodeVar132, 4.0 );
		nodeVar135 = exp2( nodeVar132 );
		nodeVar136 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar133 ) * vec2<f32>( ( nodeVar135 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

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

	nodeVar139 = ( iblIrradiance + ( ( nodeVar130 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform30 ) ) );
	iblIrradiance = nodeVar139;
	nodeVar140 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar141 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar142 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar143 = ( SpecularF90 * dfg.y );
	nodeVar144 = ( nodeVar142 + vec3<f32>( nodeVar143 ) );
	nodeVar145 = ( nodeVar140 + nodeVar144 );
	nodeVar140 = nodeVar145;
	nodeVar146 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( nodeVar147 * vec3<f32>( 0.047619 ) );
	nodeVar149 = ( SpecularColor + nodeVar148 );
	nodeVar150 = ( nodeVar144 * nodeVar149 );
	nodeVar151 = ( dfg.x + dfg.y );
	nodeVar152 = ( 1.0 - nodeVar151 );
	nodeVar153 = nodeVar152;
	nodeVar154 = ( vec3<f32>( nodeVar153 ) * nodeVar149 );
	nodeVar155 = ( vec3<f32>( 1.0 ) - nodeVar154 );
	nodeVar156 = nodeVar155;
	nodeVar157 = ( nodeVar150 / nodeVar156 );
	nodeVar158 = ( nodeVar157 * vec3<f32>( nodeVar153 ) );
	nodeVar159 = ( nodeVar141 + nodeVar158 );
	nodeVar141 = nodeVar159;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar160 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar161 = ( irradiance * nodeVar160 );
	nodeVar162 = ( nodeVar140 + nodeVar141 );
	nodeVar163 = ( vec3<f32>( 1.0 ) - nodeVar162 );
	nodeVar164 = nodeVar163;
	nodeVar165 = ( nodeVar161 * nodeVar164 );
	nodeVar166 = nodeVar165;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar167 = ( indirectDiffuse + nodeVar166 );
	indirectDiffuse = nodeVar167;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar168 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar169 = ( SpecularF90 * dfg.y );
	nodeVar170 = ( nodeVar168 + vec3<f32>( nodeVar169 ) );
	nodeVar171 = ( singleScatteringDielectric + nodeVar170 );
	singleScatteringDielectric = nodeVar171;
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
	nodeVar185 = ( multiScatteringDielectric + nodeVar184 );
	multiScatteringDielectric = nodeVar185;
	nodeVar186 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar187 = ( SpecularF90 * dfg.y );
	nodeVar188 = ( nodeVar186 + vec3<f32>( nodeVar187 ) );
	nodeVar189 = ( singleScatteringMetallic + nodeVar188 );
	singleScatteringMetallic = nodeVar189;
	nodeVar190 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar191 = nodeVar190;
	nodeVar192 = ( nodeVar191 * vec3<f32>( 0.047619 ) );
	nodeVar193 = ( DiffuseColor.xyz + nodeVar192 );
	nodeVar194 = ( nodeVar188 * nodeVar193 );
	nodeVar195 = ( dfg.x + dfg.y );
	nodeVar196 = ( 1.0 - nodeVar195 );
	nodeVar197 = nodeVar196;
	nodeVar198 = ( vec3<f32>( nodeVar197 ) * nodeVar193 );
	nodeVar199 = ( vec3<f32>( 1.0 ) - nodeVar198 );
	nodeVar200 = nodeVar199;
	nodeVar201 = ( nodeVar194 / nodeVar200 );
	nodeVar202 = ( nodeVar201 * vec3<f32>( nodeVar197 ) );
	nodeVar203 = ( multiScatteringMetallic + nodeVar202 );
	multiScatteringMetallic = nodeVar203;
	nodeVar204 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar205 = ( radiance * nodeVar204 );
	nodeVar206 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar207 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar208 = ( nodeVar206 * nodeVar207 );
	nodeVar209 = ( nodeVar205 + nodeVar208 );
	nodeVar210 = nodeVar209;
	nodeVar211 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar212 = ( vec3<f32>( 1.0 ) - nodeVar211 );
	nodeVar213 = nodeVar212;
	nodeVar214 = ( DiffuseContribution * nodeVar213 );
	nodeVar215 = ( nodeVar214 * nodeVar207 );
	nodeVar216 = nodeVar215;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar217 = ( indirectSpecular + nodeVar210 );
	indirectSpecular = nodeVar217;
	nodeVar218 = ( indirectDiffuse + nodeVar216 );
	indirectDiffuse = nodeVar218;
	ambientOcclusion = 1.0;
	nodeVar219 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar219;
	nodeVar220 = dot( normalView, positionViewDirection );
	nodeVar221 = ( clamp( nodeVar220, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar222 = ( Roughness * -16.0 );
	nodeVar223 = ( 1.0 - nodeVar222 );
	nodeVar224 = nodeVar223;
	nodeVar225 = ( - nodeVar224 );
	nodeVar226 = exp2( nodeVar225 );
	nodeVar227 = pow( nodeVar221, nodeVar226 );
	nodeVar228 = ( 1.0 - nodeVar227 );
	nodeVar229 = nodeVar228;
	nodeVar230 = ( ambientOcclusion - nodeVar229 );
	nodeVar231 = ( indirectSpecular * vec3<f32>( clamp( nodeVar230, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar231;
	nodeVar232 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar232;
	nodeVar233 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar233;
	nodeVar234 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar234;
	nodeVar235 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar235;

	// result

	output.color = nodeVar235;

	return output;

}
