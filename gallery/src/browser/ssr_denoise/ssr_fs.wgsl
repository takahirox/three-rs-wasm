// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct StructType0 {
	reflectDir : vec3<f32>,
	sampleWeight : vec3<f32>,
	pdf : f32,
	NdotV : f32,
	alpha : f32,
	f0 : vec3<f32>
};

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_depth_2d;
@binding( 2 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform7_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform7 : texture_2d<f32>;
@binding( 6 ) @group( 0 ) var nodeUniform14_sampler : sampler;
@binding( 7 ) @group( 0 ) var nodeUniform14 : texture_2d<f32>;
@binding( 8 ) @group( 0 ) var nodeUniform15_sampler : sampler;
@binding( 9 ) @group( 0 ) var nodeUniform15 : texture_2d<f32>;
@binding( 10 ) @group( 0 ) var nodeUniform16_sampler : sampler;
@binding( 11 ) @group( 0 ) var nodeUniform16 : texture_2d<f32>;
@binding( 12 ) @group( 0 ) var nodeUniform18_sampler : sampler;
@binding( 13 ) @group( 0 ) var nodeUniform18 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform4 : vec2<f32>,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<u32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : i32;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : StructType0;
var<private> nodeVar69 : StructType0;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec4<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : StructType0;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec2<f32>;
var<private> nodeVar127 : vec4<f32>;
var<private> nodeVar128 : vec2<f32>;
var<private> nodeVar129 : vec2<f32>;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : vec2<f32>;
var<private> nodeVar136 : vec2<f32>;
var<private> nodeVar137 : vec2<f32>;
var<private> nodeVar138 : vec4<f32>;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : bool;
var<private> nodeVar141 : vec2<f32>;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : vec2<f32>;
var<private> nodeVar145 : vec2<f32>;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : vec2<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : vec4<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : vec4<f32>;
var<private> nodeVar162 : vec4<f32>;
var<private> nodeVar163 : vec4<f32>;
var<private> nodeVar164 : vec4<f32>;
var<private> nodeVar165 : vec4<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : vec4<f32>;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : f32;
var<private> nodeVar186 : f32;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : f32;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : vec4<f32>;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : f32;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : f32;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : f32;
var<private> nodeVar209 : f32;
var<private> nodeVar210 : f32;
var<private> nodeVar211 : f32;
var<private> nodeVar212 : f32;
var<private> nodeVar213 : f32;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn tsl_mod_vec2( x : vec2f, y : vec2f ) -> vec2f { return x - y * floor( x / y ); }
fn SampleGGXVNDF ( V : vec3<f32>, ax : f32, ay : f32, r1 : f32, r2 : f32 ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : vec3<f32>;
	var nodeVar11 : vec3<f32>;
	var nodeVar12 : vec3<f32>;

	nodeVar0 = normalize( vec3<f32>( ( ax * V.x ), ( ay * V.y ), V.z ) );
	nodeVar1 = min( ax, ay );
	nodeVar2 = ( 1.0 + length( V.xy ) );
	nodeVar3 = ( nodeVar1 * nodeVar1 );
	nodeVar4 = ( nodeVar2 * nodeVar2 );
	nodeVar5 = ( ( ( 1.0 - nodeVar3 ) * nodeVar4 ) / ( nodeVar4 + ( ( nodeVar3 * V.z ) * V.z ) ) );
	nodeVar6 = ( nodeVar0.z * nodeVar5 );
	nodeVar7 = ( 6.283185307179586 * r1 );
	nodeVar8 = ( ( ( 1.0 - r2 ) * ( 1.0 + nodeVar6 ) ) - nodeVar6 );
	nodeVar9 = sqrt( max( 0.0, ( 1.0 - ( nodeVar8 * nodeVar8 ) ) ) );
	nodeVar10 = vec3<f32>( ( nodeVar9 * cos( nodeVar7 ) ), ( nodeVar9 * sin( nodeVar7 ) ), nodeVar8 );
	nodeVar11 = ( nodeVar10 + nodeVar0 );
	nodeVar12 = normalize( vec3<f32>( ( ax * nodeVar11.x ), ( ay * nodeVar11.y ), max( 0.0, nodeVar11.z ) ) );

	return nodeVar12;

}


fn getSpecularDominantFactor ( NoV : f32, roughness : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = ( 0.298475 * log( ( 39.4115 - ( 39.0029 * roughness ) ) ) );

	return clamp( ( ( pow( ( 1.0 - NoV ), 10.8649 ) * ( 1.0 - nodeVar0 ) ) + nodeVar0 ), 0.0, 1.0 );

}


fn computeScreenBorderFactor ( uvCoord : vec2<f32>, borderWidth : f32 ) -> f32 {

	


	return pow( smoothstep( 0.0, 1.0, smoothstep( 0.0, max( borderWidth, 0.0001 ), min( min( uvCoord.x, ( 1.0 - uvCoord.x ) ), min( uvCoord.y, ( 1.0 - uvCoord.y ) ) ) ) ), 0.125 );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = nodeVarying0;
	nodeVar2 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar1 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar0 ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar3 = nodeVar1;

	if ( ( nodeVar3 >= 1.0 ) ) {

		discard;
		

	}

	nodeVar4 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar3 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar3 ), 1.0 ) ).w ) );
	nodeVar5 = ( object.nodeUniform2 * vec4<f32>( nodeVar4, 1.0 ) ).xyz;
	nodeVar6 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	nodeVar7 = normalize( ( ( nodeVar6.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) );
	nodeVar8 = normalize( nodeVar4 );
	nodeVar9 = normalize( ( - nodeVar8 ) );
	nodeVar10 = vec3<f32>( 1.0, 1.0, 1.0 );
	nodeVar11 = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
	nodeVar12 = ( i32( object.nodeUniform5 ) + 47 );
	nodeVar13 = tsl_mod_vec2( ( floor( ( nodeVarying0 * object.nodeUniform4 ) ) + floor( ( fract( vec2<f32>( ( f32( nodeVar12 ) * 0.7548776662 ), ( f32( nodeVar12 ) * 0.569840291 ) ) ) * vec2<f32>( 32.0 ) ) ) ), vec2<f32>( 32.0 ) );
	nodeVar14 = ( ( ( nodeVar13.x * 0.7548776662466927 ) + ( nodeVar13.y * 0.5698402909980532 ) ) + 47.0 );
	nodeVar11 = vec4<f32>( fract( ( ( nodeVar14 * 1.324717957244746 ) * 0.7548776662466927 ) ), fract( ( ( nodeVar14 * 2.649435914489492 ) * 0.5698402909980532 ) ), fract( ( ( nodeVar14 * 3.974153871734238 ) * 0.419875421 ) ), fract( ( ( nodeVar14 * 5.298871828978984 ) * 0.43015970900194667 ) ) );
	nodeVar15 = nodeVar11;
	nodeVar15.y = mix( nodeVar15.y, 0.0, ( object.nodeUniform6 * sqrt( nodeVar15.w ) ) );
	nodeVar16 = textureSample( nodeUniform7, nodeUniform7_sampler, nodeVar0 );
	nodeVar10 = nodeVar16.xyz;
	nodeVar17 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	nodeVar18 = max( ( nodeVar17.w * nodeVar17.w ), 0.001 );
	nodeVar19 = nodeVar18;
	nodeVar20 = nodeVar18;
	nodeVar21 = cross( vec3<f32>( 0.0, 0.0, 1.0 ), nodeVar7 );
	nodeVar22 = normalize( nodeVar21 );

	if ( ( length( nodeVar22 ) < 0.001 ) ) {

		nodeVar22 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), nodeVar7 ) );
		

	}

	nodeVar23 = normalize( cross( nodeVar7, nodeVar22 ) );
	nodeVar24 = vec3<f32>( dot( nodeVar22, nodeVar9 ), dot( nodeVar23, nodeVar9 ), dot( nodeVar7, nodeVar9 ) );
	nodeVar25 = SampleGGXVNDF( nodeVar24, nodeVar19, nodeVar20, nodeVar15.x, nodeVar15.y );

	if ( ( nodeVar25.z < 0.0 ) ) {

		nodeVar25 = ( - nodeVar25 );
		

	}

	nodeVar26 = normalize( ( ( ( nodeVar22 * vec3<f32>( nodeVar25.x ) ) + ( nodeVar23 * vec3<f32>( nodeVar25.y ) ) ) + ( nodeVar7 * vec3<f32>( nodeVar25.z ) ) ) );
	nodeVar27 = normalize( reflect( ( - nodeVar9 ), nodeVar26 ) );
	nodeVar28 = nodeVar27;
	nodeVar29 = normalize( ( nodeVar9 + nodeVar28 ) );
	nodeVar30 = max( 0.0, dot( nodeVar7, nodeVar9 ) );
	nodeVar31 = max( 0.0, dot( nodeVar7, nodeVar28 ) );
	nodeVar32 = max( 0.0, dot( nodeVar7, nodeVar29 ) );
	nodeVar33 = max( 0.0, dot( nodeVar9, nodeVar29 ) );
	nodeVar34 = textureSample( nodeUniform7, nodeUniform7_sampler, nodeVarying0 );
	nodeVar35 = mix( vec3<f32>( 0.04, 0.04, 0.04 ), nodeVar10, nodeVar34.w );
	nodeVar36 = ( 1.0 - nodeVar33 );
	nodeVar37 = ( nodeVar36 * nodeVar36 );
	nodeVar38 = ( ( nodeVar37 * nodeVar37 ) * nodeVar36 );
	nodeVar39 = ( nodeVar35 + ( ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar35 ) * vec3<f32>( nodeVar38 ) ) );
	nodeVar40 = nodeVar39;
	nodeVar41 = ( nodeVar19 * nodeVar19 );
	nodeVar42 = ( nodeVar32 * nodeVar32 );
	nodeVar43 = ( ( nodeVar42 * ( nodeVar41 - 1.0 ) ) + 1.0 );
	nodeVar44 = ( nodeVar41 / ( 3.141592653589793 * pow( nodeVar43, 2.0 ) ) );
	nodeVar45 = nodeVar44;
	nodeVar46 = ( nodeVar19 * nodeVar19 );
	nodeVar47 = max( 0.0, ( 1.0 - ( nodeVar30 * nodeVar30 ) ) );
	nodeVar48 = ( 1.0 + sqrt( nodeVar47 ) );
	nodeVar49 = ( nodeVar48 * nodeVar48 );
	nodeVar50 = ( ( ( 1.0 - nodeVar46 ) * nodeVar49 ) / ( nodeVar49 + ( ( nodeVar46 * nodeVar30 ) * nodeVar30 ) ) );
	nodeVar51 = sqrt( ( ( nodeVar46 * nodeVar47 ) + ( nodeVar30 * nodeVar30 ) ) );
	nodeVar52 = ( nodeVar45 / max( 0.000001, ( 2.0 * ( ( nodeVar50 * nodeVar30 ) + nodeVar51 ) ) ) );
	nodeVar53 = nodeVar52;
	nodeVar54 = ( nodeVar19 * nodeVar19 );
	nodeVar55 = max( ( 1.0 - ( nodeVar30 * nodeVar30 ) ), 0.0 );
	nodeVar56 = ( 1.0 + sqrt( nodeVar55 ) );
	nodeVar57 = ( nodeVar56 * nodeVar56 );
	nodeVar58 = ( ( ( 1.0 - nodeVar54 ) * nodeVar57 ) / ( nodeVar57 + ( ( nodeVar54 * nodeVar30 ) * nodeVar30 ) ) );
	nodeVar59 = sqrt( ( ( nodeVar54 * nodeVar55 ) + ( nodeVar30 * nodeVar30 ) ) );
	nodeVar60 = ( nodeVar19 * nodeVar19 );
	nodeVar61 = ( nodeVar30 * nodeVar30 );
	nodeVar62 = ( ( 2.0 * nodeVar30 ) / ( nodeVar30 + sqrt( ( nodeVar60 + ( ( 1.0 - nodeVar60 ) * nodeVar61 ) ) ) ) );
	nodeVar63 = ( nodeVar19 * nodeVar19 );
	nodeVar64 = ( nodeVar31 * nodeVar31 );
	nodeVar65 = ( ( 2.0 * nodeVar31 ) / ( nodeVar31 + sqrt( ( nodeVar63 + ( ( 1.0 - nodeVar63 ) * nodeVar64 ) ) ) ) );
	nodeVar66 = ( nodeVar62 * nodeVar65 );
	nodeVar67 = ( ( ( nodeVar40 * vec3<f32>( nodeVar66 ) ) * vec3<f32>( ( ( nodeVar58 * nodeVar30 ) + nodeVar59 ) ) ) / vec3<f32>( max( ( 2.0 * nodeVar30 ), 0.0001 ) ) );
	nodeVar68 = StructType0( nodeVar27, nodeVar67, nodeVar53, nodeVar30, nodeVar19, nodeVar35 );
	nodeVar69 = nodeVar68;

	if ( ( dot( nodeVar69.reflectDir, nodeVar7 ) < 0.0 ) ) {

		nodeVar70 = max( ( nodeVar17.w * nodeVar17.w ), 0.001 );
		nodeVar71 = nodeVar70;
		nodeVar72 = nodeVar70;
		nodeVar73 = cross( vec3<f32>( 0.0, 0.0, 1.0 ), nodeVar7 );
		nodeVar74 = normalize( nodeVar73 );

		if ( ( length( nodeVar74 ) < 0.001 ) ) {

			nodeVar74 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), nodeVar7 ) );
			

		}

		nodeVar75 = normalize( cross( nodeVar7, nodeVar74 ) );
		nodeVar76 = vec3<f32>( dot( nodeVar74, nodeVar9 ), dot( nodeVar75, nodeVar9 ), dot( nodeVar7, nodeVar9 ) );
		nodeVar77 = fract( ( nodeVar15 + ( nodeVar15 * vec4<f32>( 7.0 ) ) ) );
		nodeVar78 = SampleGGXVNDF( nodeVar76, nodeVar71, nodeVar72, nodeVar77.x, nodeVar77.y );

		if ( ( nodeVar78.z < 0.0 ) ) {

			nodeVar78 = ( - nodeVar78 );
			

		}

		nodeVar79 = normalize( ( ( ( nodeVar74 * vec3<f32>( nodeVar78.x ) ) + ( nodeVar75 * vec3<f32>( nodeVar78.y ) ) ) + ( nodeVar7 * vec3<f32>( nodeVar78.z ) ) ) );
		nodeVar80 = normalize( reflect( ( - nodeVar9 ), nodeVar79 ) );
		nodeVar81 = nodeVar80;
		nodeVar82 = normalize( ( nodeVar9 + nodeVar81 ) );
		nodeVar83 = max( 0.0, dot( nodeVar7, nodeVar9 ) );
		nodeVar84 = max( 0.0, dot( nodeVar7, nodeVar81 ) );
		nodeVar85 = max( 0.0, dot( nodeVar7, nodeVar82 ) );
		nodeVar86 = max( 0.0, dot( nodeVar9, nodeVar82 ) );
		nodeVar87 = mix( vec3<f32>( 0.04, 0.04, 0.04 ), nodeVar10, nodeVar34.w );
		nodeVar88 = ( 1.0 - nodeVar86 );
		nodeVar89 = ( nodeVar88 * nodeVar88 );
		nodeVar90 = ( ( nodeVar89 * nodeVar89 ) * nodeVar88 );
		nodeVar91 = ( nodeVar87 + ( ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar87 ) * vec3<f32>( nodeVar90 ) ) );
		nodeVar92 = nodeVar91;
		nodeVar93 = ( nodeVar71 * nodeVar71 );
		nodeVar94 = ( nodeVar85 * nodeVar85 );
		nodeVar95 = ( ( nodeVar94 * ( nodeVar93 - 1.0 ) ) + 1.0 );
		nodeVar96 = ( nodeVar93 / ( 3.141592653589793 * pow( nodeVar95, 2.0 ) ) );
		nodeVar97 = nodeVar96;
		nodeVar98 = ( nodeVar71 * nodeVar71 );
		nodeVar99 = max( 0.0, ( 1.0 - ( nodeVar83 * nodeVar83 ) ) );
		nodeVar100 = ( 1.0 + sqrt( nodeVar99 ) );
		nodeVar101 = ( nodeVar100 * nodeVar100 );
		nodeVar102 = ( ( ( 1.0 - nodeVar98 ) * nodeVar101 ) / ( nodeVar101 + ( ( nodeVar98 * nodeVar83 ) * nodeVar83 ) ) );
		nodeVar103 = sqrt( ( ( nodeVar98 * nodeVar99 ) + ( nodeVar83 * nodeVar83 ) ) );
		nodeVar104 = ( nodeVar97 / max( 0.000001, ( 2.0 * ( ( nodeVar102 * nodeVar83 ) + nodeVar103 ) ) ) );
		nodeVar105 = nodeVar104;
		nodeVar106 = ( nodeVar71 * nodeVar71 );
		nodeVar107 = max( ( 1.0 - ( nodeVar83 * nodeVar83 ) ), 0.0 );
		nodeVar108 = ( 1.0 + sqrt( nodeVar107 ) );
		nodeVar109 = ( nodeVar108 * nodeVar108 );
		nodeVar110 = ( ( ( 1.0 - nodeVar106 ) * nodeVar109 ) / ( nodeVar109 + ( ( nodeVar106 * nodeVar83 ) * nodeVar83 ) ) );
		nodeVar111 = sqrt( ( ( nodeVar106 * nodeVar107 ) + ( nodeVar83 * nodeVar83 ) ) );
		nodeVar112 = ( nodeVar71 * nodeVar71 );
		nodeVar113 = ( nodeVar83 * nodeVar83 );
		nodeVar114 = ( ( 2.0 * nodeVar83 ) / ( nodeVar83 + sqrt( ( nodeVar112 + ( ( 1.0 - nodeVar112 ) * nodeVar113 ) ) ) ) );
		nodeVar115 = ( nodeVar71 * nodeVar71 );
		nodeVar116 = ( nodeVar84 * nodeVar84 );
		nodeVar117 = ( ( 2.0 * nodeVar84 ) / ( nodeVar84 + sqrt( ( nodeVar115 + ( ( 1.0 - nodeVar115 ) * nodeVar116 ) ) ) ) );
		nodeVar118 = ( nodeVar114 * nodeVar117 );
		nodeVar119 = ( ( ( nodeVar92 * vec3<f32>( nodeVar118 ) ) * vec3<f32>( ( ( nodeVar110 * nodeVar83 ) + nodeVar111 ) ) ) / vec3<f32>( max( ( 2.0 * nodeVar83 ), 0.0001 ) ) );
		nodeVar120 = StructType0( nodeVar80, nodeVar119, nodeVar105, nodeVar83, nodeVar71, nodeVar87 );
		nodeVar69 = nodeVar120;
		

	}

	nodeVar121 = nodeVar69.reflectDir;
	nodeVar122 = nodeVar69.sampleWeight;
	nodeVar123 = getSpecularDominantFactor( nodeVar69.NdotV, nodeVar17.w );
	nodeVar124 = ( object.nodeUniform8 / dot( ( - nodeVar8 ), nodeVar7 ) );
	nodeVar125 = ( nodeVar4 + ( nodeVar121 * vec3<f32>( nodeVar124 ) ) );

	if ( ( nodeVar125.z > ( - object.nodeUniform9 ) ) ) {

		nodeVar125 = ( nodeVar4 + ( nodeVar121 * vec3<f32>( ( ( ( - object.nodeUniform9 ) - nodeVar4.z ) / nodeVar121.z ) ) ) );
		

	}

	nodeVar126 = ( nodeVar0 * object.nodeUniform4 );
	nodeVar127 = ( object.nodeUniform10 * vec4<f32>( nodeVar125, 1.0 ) );
	nodeVar128 = ( ( ( nodeVar127.xy / vec2<f32>( nodeVar127.w ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) );
	nodeVar129 = ( vec2<f32>( nodeVar128.x, ( 1.0 - nodeVar128.y ) ) * object.nodeUniform4 );
	nodeVar130 = ( nodeVar129.x - nodeVar126.x );
	nodeVar131 = ( nodeVar129.y - nodeVar126.y );
	nodeVar132 = max( max( abs( nodeVar130 ), abs( nodeVar131 ) ), 1.0 );
	let nodeConst0 = i32( max( ( clamp( object.nodeUniform11, 0.0, 1.0 ) * 64.0 ), 1.0 ) );
	nodeVar133 = ( nodeVar130 / f32( nodeConst0 ) );
	nodeVar134 = ( nodeVar131 / f32( nodeConst0 ) );
	nodeVar135 = vec2<f32>( nodeVar133, nodeVar134 );
	nodeVar136 = ( vec2<f32>( 1.0, 1.0 ) / object.nodeUniform4 );
	nodeVar137 = vec2<f32>( nodeVar136.x, 0.0 );
	nodeVar138 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );
	nodeVar139 = 0.0;
	let nodeConst1 = ( 1.0 / nodeVar4.z );
	let nodeConst2 = ( 1.0 / nodeVar125.z );
	nodeVar140 = false;
	nodeVar141 = vec2<f32>( 0.0, 0.0 );
	nodeVar142 = 0.0;

	for ( var i : i32 = 1; i < nodeConst0; i ++ ) {

		nodeVar143 = max( pow( ( ( f32( i ) + ( nodeVar11.z - 0.5 ) ) / f32( nodeConst0 ) ), 3.0 ), ( f32( i ) / nodeVar132 ) );
		nodeVar144 = ( nodeVar126 + ( nodeVar135 * vec2<f32>( ( nodeVar143 * f32( nodeConst0 ) ) ) ) );

		if ( ( ( ( ( nodeVar144.x < 0.0 ) || ( nodeVar144.x > object.nodeUniform4.x ) ) || ( nodeVar144.y < 0.0 ) ) || ( nodeVar144.y > object.nodeUniform4.y ) ) ) {

			break;
			

		}

		nodeVar145 = ( nodeVar144 * nodeVar136 );
		nodeVar146 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar145 ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar147 = nodeVar146;
		nodeVar148 = ( ( object.nodeUniform9 * object.nodeUniform12 ) / ( ( ( object.nodeUniform12 - object.nodeUniform9 ) * nodeVar147 ) - object.nodeUniform12 ) );
		nodeVar149 = ( 1.0 / ( nodeConst1 + ( nodeVar143 * ( nodeConst2 - nodeConst1 ) ) ) );

		if ( ( nodeVar149 <= nodeVar148 ) ) {

			nodeVar150 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar145.x, ( 1.0 - nodeVar145.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar147 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar145.x, ( 1.0 - nodeVar145.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar147 ), 1.0 ) ).w ) );
			nodeVar151 = ( length( cross( ( nodeVar150 - nodeVar4 ), ( nodeVar150 - nodeVar125 ) ) ) / length( ( nodeVar125 - nodeVar4 ) ) );
			nodeVar152 = ( nodeVar145 + nodeVar137 );
			nodeVar153 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar152.x, ( 1.0 - nodeVar152.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar147 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar152.x, ( 1.0 - nodeVar152.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar147 ), 1.0 ) ).w ) );
			nodeVar154 = ( ( nodeVar153.x - nodeVar150.x ) * 3.0 );
			nodeVar155 = max( nodeVar154, object.nodeUniform13 );

			if ( ( nodeVar151 <= nodeVar155 ) ) {

				nodeVar156 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVar145 );
				nodeVar157 = normalize( ( ( nodeVar156.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) );
				nodeVar140 = true;
				nodeVar141 = nodeVar145;
				nodeVar142 = nodeVar147;
				break;
				

			}

			

		}


	}


	if ( nodeVar140 ) {

		nodeVar158 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar141.x, ( 1.0 - nodeVar141.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar142 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar141.x, ( 1.0 - nodeVar141.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar142 ), 1.0 ) ).w ) );

		if ( ( 0.0 <= object.nodeUniform8 ) ) {

			nodeVar159 = ( object.nodeUniform2 * vec4<f32>( nodeVar158, 1.0 ) ).xyz;
			nodeVar160 = ( distance( nodeVar5, nodeVar159 ) * nodeVar123 );
			nodeVar161 = textureSample( nodeUniform14, nodeUniform14_sampler, nodeVar141 );
			nodeVar162 = nodeVar161;
			nodeVar163 = textureSample( nodeUniform16, nodeUniform16_sampler, nodeVar141 );
			nodeVar164 = textureSample( nodeUniform15, nodeUniform15_sampler, ( nodeVar141 - nodeVar163.xy ) );
			nodeVar165 = nodeVar164;
			nodeVar166 = ( nodeVar162.xyz + ( nodeVar165.xyz * vec3<f32>( ( 1.0 - nodeVar165.w ) ) ) );
			nodeVar162.x = nodeVar166[ 0 ];
			nodeVar162.y = nodeVar166[ 1 ];
			nodeVar162.z = nodeVar166[ 2 ];
			nodeVar167 = computeScreenBorderFactor( nodeVar141, ( object.nodeUniform17 * ( 1.0 - min( ( nodeVar17.w / 0.25 ), 1.0 ) ) ) );

			if ( ( nodeVar167 < 1.0 ) ) {

				nodeVar168 = vec3<f32>( 0.0, 0.0, 0.0 );
				nodeVar169 = normalize( ( object.nodeUniform2 * vec4<f32>( nodeVar7, 0.0 ) ).xyz );
				nodeVar170 = normalize( ( object.nodeUniform2 * vec4<f32>( nodeVar9, 0.0 ) ).xyz );
				nodeVar171 = max( 0.0, dot( nodeVar169, nodeVar170 ) );
				nodeVar172 = normalize( ( object.nodeUniform2 * vec4<f32>( nodeVar121, 0.0 ) ).xyz );
				nodeVar173 = normalize( ( nodeVar170 + nodeVar172 ) );
				nodeVar174 = max( 0.0, dot( nodeVar169, nodeVar172 ) );
				nodeVar175 = max( 0.0, dot( nodeVar170, nodeVar173 ) );
				nodeVar176 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, vec2<f32>( ( ( atan2( nodeVar172.z, nodeVar172.x ) * 0.15915494309189535 ) + 0.5 ), ( ( asin( clamp( nodeVar172.y, -1.0, 1.0 ) ) * 0.3183098861837907 ) + 0.5 ) ), 0.0 );
				nodeVar177 = ( 1.0 - nodeVar175 );
				nodeVar178 = ( nodeVar177 * nodeVar177 );
				nodeVar179 = ( ( nodeVar178 * nodeVar178 ) * nodeVar177 );
				nodeVar180 = ( nodeVar69.f0 + ( ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar69.f0 ) * vec3<f32>( nodeVar179 ) ) );
				nodeVar181 = ( nodeVar69.alpha * nodeVar69.alpha );
				nodeVar182 = ( nodeVar171 * nodeVar171 );
				nodeVar183 = ( ( 2.0 * nodeVar171 ) / ( nodeVar171 + sqrt( ( nodeVar181 + ( ( 1.0 - nodeVar181 ) * nodeVar182 ) ) ) ) );
				nodeVar184 = ( nodeVar69.alpha * nodeVar69.alpha );
				nodeVar185 = ( nodeVar174 * nodeVar174 );
				nodeVar186 = ( ( 2.0 * nodeVar174 ) / ( nodeVar174 + sqrt( ( nodeVar184 + ( ( 1.0 - nodeVar184 ) * nodeVar185 ) ) ) ) );
				nodeVar187 = ( nodeVar183 * nodeVar186 );
				nodeVar188 = ( nodeVar69.alpha * nodeVar69.alpha );
				nodeVar189 = ( nodeVar171 * nodeVar171 );
				nodeVar168 = ( ( nodeVar176.xyz * ( ( nodeVar180 * vec3<f32>( nodeVar187 ) ) / vec3<f32>( max( ( ( 2.0 * nodeVar171 ) / ( nodeVar171 + sqrt( ( nodeVar188 + ( ( 1.0 - nodeVar188 ) * nodeVar189 ) ) ) ) ), 0.0001 ) ) ) ) * vec3<f32>( object.nodeUniform19 ) );
				nodeVar190 = mix( ( nodeVar168 * vec3<f32>( object.nodeUniform20 ) ), nodeVar162.xyz, nodeVar167 );
				nodeVar162.x = nodeVar190[ 0 ];
				nodeVar162.y = nodeVar190[ 1 ];
				nodeVar162.z = nodeVar190[ 2 ];
				

			}

			nodeVar139 = 1.0;
			nodeVar138 = vec4<f32>( ( nodeVar162.xyz * nodeVar122 ), nodeVar160 );
			

		}

		

	}


	if ( ( nodeVar139 == 0.0 ) ) {

		nodeVar191 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar192 = normalize( ( object.nodeUniform2 * vec4<f32>( nodeVar7, 0.0 ) ).xyz );
		nodeVar193 = normalize( ( object.nodeUniform2 * vec4<f32>( nodeVar9, 0.0 ) ).xyz );
		nodeVar194 = max( 0.0, dot( nodeVar192, nodeVar193 ) );
		nodeVar195 = normalize( ( object.nodeUniform2 * vec4<f32>( nodeVar121, 0.0 ) ).xyz );
		nodeVar196 = normalize( ( nodeVar193 + nodeVar195 ) );
		nodeVar197 = max( 0.0, dot( nodeVar192, nodeVar195 ) );
		nodeVar198 = max( 0.0, dot( nodeVar193, nodeVar196 ) );
		nodeVar199 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, vec2<f32>( ( ( atan2( nodeVar195.z, nodeVar195.x ) * 0.15915494309189535 ) + 0.5 ), ( ( asin( clamp( nodeVar195.y, -1.0, 1.0 ) ) * 0.3183098861837907 ) + 0.5 ) ), 0.0 );
		nodeVar200 = ( 1.0 - nodeVar198 );
		nodeVar201 = ( nodeVar200 * nodeVar200 );
		nodeVar202 = ( ( nodeVar201 * nodeVar201 ) * nodeVar200 );
		nodeVar203 = ( nodeVar69.f0 + ( ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar69.f0 ) * vec3<f32>( nodeVar202 ) ) );
		nodeVar204 = ( nodeVar69.alpha * nodeVar69.alpha );
		nodeVar205 = ( nodeVar194 * nodeVar194 );
		nodeVar206 = ( ( 2.0 * nodeVar194 ) / ( nodeVar194 + sqrt( ( nodeVar204 + ( ( 1.0 - nodeVar204 ) * nodeVar205 ) ) ) ) );
		nodeVar207 = ( nodeVar69.alpha * nodeVar69.alpha );
		nodeVar208 = ( nodeVar197 * nodeVar197 );
		nodeVar209 = ( ( 2.0 * nodeVar197 ) / ( nodeVar197 + sqrt( ( nodeVar207 + ( ( 1.0 - nodeVar207 ) * nodeVar208 ) ) ) ) );
		nodeVar210 = ( nodeVar206 * nodeVar209 );
		nodeVar211 = ( nodeVar69.alpha * nodeVar69.alpha );
		nodeVar212 = ( nodeVar194 * nodeVar194 );
		nodeVar191 = ( ( nodeVar199.xyz * ( ( nodeVar203 * vec3<f32>( nodeVar210 ) ) / vec3<f32>( max( ( ( 2.0 * nodeVar194 ) / ( nodeVar194 + sqrt( ( nodeVar211 + ( ( 1.0 - nodeVar211 ) * nodeVar212 ) ) ) ) ), 0.0001 ) ) ) ) * vec3<f32>( object.nodeUniform19 ) );
		nodeVar138 = vec4<f32>( ( nodeVar191 * vec3<f32>( object.nodeUniform20 ) ), 10000.0 );
		

	}

	nodeVar213 = max( dot( nodeVar138.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ), 0.0001 );
	nodeVar214 = ( nodeVar138.xyz * vec3<f32>( min( ( object.nodeUniform21 / nodeVar213 ), 1.0 ) ) );
	nodeVar138.x = nodeVar214[ 0 ];
	nodeVar138.y = nodeVar214[ 1 ];
	nodeVar138.z = nodeVar214[ 2 ];
	nodeVar215 = ( nodeVar138.xyz * vec3<f32>( object.nodeUniform22 ) );
	nodeVar138.x = nodeVar215[ 0 ];
	nodeVar138.y = nodeVar215[ 1 ];
	nodeVar138.z = nodeVar215[ 2 ];

	// result

	output.color = max( nodeVar138, vec4<f32>( 0.0 ) );

	return output;

}
