// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct StructType0 {
	reflectDir : vec3<f32>,
	sampleWeight : vec3<f32>,
	pdf : f32,
	NdotV : f32,
	alpha : f32,
	f90 : f32,
	f0 : vec3<f32>
};

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 0 ) var nodeUniform2 : texture_depth_2d;
@binding( 2 ) @group( 0 ) var nodeUniform5_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform5 : texture_2d<f32>;
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
	nodeUniform0 : vec2<f32>,
	nodeUniform1 : f32,
	nodeUniform3 : mat4x4<f32>,
	nodeUniform4 : mat4x4<f32>,
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
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec2<u32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec4<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : StructType0;
var<private> nodeVar43 : StructType0;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : StructType0;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec2<f32>;
var<private> nodeVar76 : vec2<f32>;
var<private> nodeVar77 : vec2<f32>;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : vec2<f32>;
var<private> nodeVar86 : vec2<f32>;
var<private> nodeVar87 : vec2<f32>;
var<private> nodeVar88 : vec4<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : bool;
var<private> nodeVar91 : vec2<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : bool;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec2<f32>;
var<private> nodeVar97 : vec2<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : bool;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec2<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec4<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : bool;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec4<f32>;
var<private> nodeVar117 : vec4<f32>;
var<private> nodeVar118 : vec4<f32>;
var<private> nodeVar119 : vec4<f32>;
var<private> nodeVar120 : vec4<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : vec4<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec4<f32>;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;

// codes
fn fn131 ( roughness : f32, NoH : f32, k : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;

	nodeVar0 = ( roughness * roughness );
	nodeVar1 = ( NoH * NoH );
	nodeVar2 = ( ( nodeVar1 * ( nodeVar0 - 1.0 ) ) + 1.0 );
	nodeVar3 = ( nodeVar0 / ( 3.141592653589793 * pow( nodeVar2, k ) ) );

	return nodeVar3;

}


fn fn132 ( NDotX : f32, alpha : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;

	nodeVar0 = ( alpha * alpha );
	nodeVar1 = ( NDotX * NDotX );

	return ( ( 2.0 * NDotX ) / ( NDotX + sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * nodeVar1 ) ) ) ) );

}


fn tsl_mod_vec2( x : vec2f, y : vec2f ) -> vec2f { return x - y * floor( x / y ); }
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn130 ( V : vec3<f32>, ax : f32, ay : f32, r1 : f32, r2 : f32 ) -> vec3<f32> {

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


fn fn135 ( f0 : vec3<f32>, theta : f32, f90 : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : vec3<f32>;

	nodeVar0 = ( 1.0 - theta );
	nodeVar1 = ( nodeVar0 * nodeVar0 );
	nodeVar2 = ( ( nodeVar1 * nodeVar1 ) * nodeVar0 );
	nodeVar3 = ( f0 + ( ( vec3<f32>( f90 ) - f0 ) * vec3<f32>( nodeVar2 ) ) );

	return nodeVar3;

}


fn fn134 ( NoH : f32, NoV : f32, roughness : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;

	nodeVar0 = fn131( roughness, NoH, 2.0 );
	nodeVar1 = ( roughness * roughness );
	nodeVar2 = max( 0.0, ( 1.0 - ( NoV * NoV ) ) );
	nodeVar3 = ( 1.0 + sqrt( nodeVar2 ) );
	nodeVar4 = ( nodeVar3 * nodeVar3 );
	nodeVar5 = ( ( ( 1.0 - nodeVar1 ) * nodeVar4 ) / ( nodeVar4 + ( ( nodeVar1 * NoV ) * NoV ) ) );
	nodeVar6 = sqrt( ( ( nodeVar1 * nodeVar2 ) + ( NoV * NoV ) ) );
	nodeVar7 = ( nodeVar0 / max( 0.000001, ( 2.0 * ( ( nodeVar5 * NoV ) + nodeVar6 ) ) ) );

	return nodeVar7;

}


fn fn133 ( NoL : f32, NoV : f32, alphaG : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = fn132( NoV, alphaG );
	nodeVar1 = fn132( NoL, alphaG );
	nodeVar2 = ( nodeVar0 * nodeVar1 );

	return nodeVar2;

}


fn fn136 ( NoV : f32, roughness : f32 ) -> f32 {

	

	let nodeConst0 = ( 0.298475 * log( ( 39.4115 - ( 39.0029 * roughness ) ) ) );

	return clamp( ( ( pow( ( 1.0 - NoV ), 10.8649 ) * ( 1.0 - nodeConst0 ) ) + nodeConst0 ), 0.0, 1.0 );

}


fn fn22 ( viewPosition : vec3<f32>, projectionMatrix : mat4x4<f32> ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	let nodeConst0 = ( projectionMatrix * vec4<f32>( viewPosition, 1.0 ) );
	nodeVar0 = ( ( ( nodeConst0.xy / vec2<f32>( nodeConst0.w ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) );

	return vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) );

}


fn fn164 ( point : vec3<f32>, linePointA : vec3<f32>, linePointB : vec3<f32> ) -> f32 {

	


	return ( length( cross( ( point - linePointA ), ( point - linePointB ) ) ) / length( ( linePointB - linePointA ) ) );

}


fn fn166 ( uvCoord : vec2<f32>, borderWidth : f32 ) -> f32 {

	


	return pow( smoothstep( 0.0, 1.0, smoothstep( 0.0, max( borderWidth, 0.0001 ), min( min( uvCoord.x, ( 1.0 - uvCoord.x ) ), min( uvCoord.y, ( 1.0 - uvCoord.y ) ) ) ) ), 0.125 );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
	let nodeConst0 = ( i32( object.nodeUniform1 ) + 47 );
	let nodeConst1 = tsl_mod_vec2( ( floor( ( nodeVarying0 * object.nodeUniform0 ) ) + floor( ( fract( vec2<f32>( ( f32( nodeConst0 ) * 0.7548776662 ), ( f32( nodeConst0 ) * 0.569840291 ) ) ) * vec2<f32>( 32.0 ) ) ) ), vec2<f32>( 32.0 ) );
	let nodeConst2 = ( ( ( nodeConst1.x * 0.7548776662466927 ) + ( nodeConst1.y * 0.5698402909980532 ) ) + 47.0 );
	nodeVar0 = vec4<f32>( fract( ( ( nodeConst2 * 1.324717957244746 ) * 0.7548776662466927 ) ), fract( ( ( nodeConst2 * 2.649435914489492 ) * 0.5698402909980532 ) ), fract( ( ( nodeConst2 * 3.974153871734238 ) * 0.419875421 ) ), fract( ( ( nodeConst2 * 5.298871828978984 ) * 0.43015970900194667 ) ) );
	var nodeVar1 : vec4<f32> = nodeVar0;
	let nodeConst3 = vec2<f32>( textureDimensions( nodeUniform2, 0 ) );
	nodeVar2 = ( ( floor( ( nodeVarying0 * nodeConst3 ) ) + vec2<f32>( 0.5 ) ) / nodeConst3 );
	nodeVar4 = textureDimensions( nodeUniform2, u32( 0 ) );
	nodeVar3 = textureLoad( nodeUniform2, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar2 ) * vec2<f32>( nodeVar4 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar4 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar5 = nodeVar3;

	if ( ( nodeVar5 >= 1.0 ) ) {

		discard;
		

	}

	let nodeConst4 = ( object.nodeUniform3 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar2.x, ( 1.0 - nodeVar2.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar5 ), 1.0 ) );
	nodeVar6 = ( nodeConst4.xyz / vec3<f32>( nodeConst4.w ) );
	nodeVar7 = ( object.nodeUniform4 * vec4<f32>( nodeVar6, 1.0 ) ).xyz;
	nodeVar8 = textureSample( nodeUniform5, nodeUniform5_sampler, nodeVarying0 );
	nodeVar9 = normalize( ( ( nodeVar8.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) );
	nodeVar10 = normalize( nodeVar6 );
	nodeVar11 = normalize( ( - nodeVar10 ) );
	nodeVar12 = nodeVar1;
	nodeVar12.y = mix( nodeVar12.y, 0.0, ( object.nodeUniform6 * sqrt( nodeVar12.w ) ) );
	nodeVar13 = textureSample( nodeUniform7, nodeUniform7_sampler, nodeVar2 );
	nodeVar14 = nodeVar13.xyz;
	nodeVar15 = textureSample( nodeUniform5, nodeUniform5_sampler, nodeVarying0 );
	let nodeConst5 = nodeVar15;
	let nodeConst6 = nodeConst5.w;
	nodeVar16 = max( ( nodeConst6 * nodeConst6 ), 0.001 );
	nodeVar17 = nodeVar16;
	nodeVar18 = nodeVar16;

	if ( ( abs( nodeVar9.z ) < 0.999 ) ) {

		nodeVar19 = vec3<f32>( 0.0, 0.0, 1.0 );

	} else {

		nodeVar19 = vec3<f32>( 1.0, 0.0, 0.0 );

	}

	nodeVar20 = normalize( cross( nodeVar19, nodeVar9 ) );
	nodeVar21 = normalize( cross( nodeVar9, nodeVar20 ) );
	nodeVar22 = vec3<f32>( dot( nodeVar20, nodeVar11 ), dot( nodeVar21, nodeVar11 ), dot( nodeVar9, nodeVar11 ) );
	nodeVar23 = fn130( nodeVar22, nodeVar17, nodeVar18, nodeVar12.x, nodeVar12.y );

	if ( ( nodeVar23.z < 0.0 ) ) {

		nodeVar23 = ( - nodeVar23 );
		

	}

	nodeVar24 = normalize( ( ( ( nodeVar20 * vec3<f32>( nodeVar23.x ) ) + ( nodeVar21 * vec3<f32>( nodeVar23.y ) ) ) + ( nodeVar9 * vec3<f32>( nodeVar23.z ) ) ) );
	nodeVar25 = normalize( reflect( ( - nodeVar11 ), nodeVar24 ) );
	nodeVar26 = nodeVar25;
	nodeVar27 = normalize( ( nodeVar11 + nodeVar26 ) );
	nodeVar28 = max( 0.0, dot( nodeVar9, nodeVar11 ) );
	nodeVar29 = max( 0.0, dot( nodeVar9, nodeVar26 ) );
	nodeVar30 = max( 0.0, dot( nodeVar9, nodeVar27 ) );
	nodeVar31 = max( 0.0, dot( nodeVar11, nodeVar27 ) );
	nodeVar32 = textureSample( nodeUniform7, nodeUniform7_sampler, nodeVarying0 );
	let nodeConst7 = nodeVar32;
	let nodeConst8 = nodeConst7.w;
	let nodeConst9 = mix( vec3<f32>( 0.04, 0.04, 0.04 ), nodeVar14, nodeConst8 );
	nodeVar33 = fn135( nodeConst9, nodeVar31, 1.0 );
	nodeVar34 = fn134( nodeVar30, nodeVar28, nodeVar17 );
	nodeVar35 = ( nodeVar17 * nodeVar17 );
	nodeVar36 = max( ( 1.0 - ( nodeVar28 * nodeVar28 ) ), 0.0 );
	nodeVar37 = ( 1.0 + sqrt( nodeVar36 ) );
	nodeVar38 = ( nodeVar37 * nodeVar37 );
	nodeVar39 = ( ( ( 1.0 - nodeVar35 ) * nodeVar38 ) / ( nodeVar38 + ( ( nodeVar35 * nodeVar28 ) * nodeVar28 ) ) );
	nodeVar40 = sqrt( ( ( nodeVar35 * nodeVar36 ) + ( nodeVar28 * nodeVar28 ) ) );
	nodeVar41 = ( ( ( nodeVar33 * vec3<f32>( fn133( nodeVar29, nodeVar28, nodeVar17 ) ) ) * vec3<f32>( ( ( nodeVar39 * nodeVar28 ) + nodeVar40 ) ) ) / vec3<f32>( max( ( 2.0 * nodeVar28 ), 0.0001 ) ) );
	nodeVar42 = StructType0( nodeVar25, nodeVar41, nodeVar34, nodeVar28, nodeVar17, 1.0, nodeConst9 );
	nodeVar43 = nodeVar42;

	if ( ( dot( nodeVar43.reflectDir, nodeVar9 ) < 0.0 ) ) {

		nodeVar44 = max( ( nodeConst6 * nodeConst6 ), 0.001 );
		nodeVar45 = nodeVar44;
		nodeVar46 = nodeVar44;

		if ( ( abs( nodeVar9.z ) < 0.999 ) ) {

			nodeVar47 = vec3<f32>( 0.0, 0.0, 1.0 );

		} else {

			nodeVar47 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar48 = normalize( cross( nodeVar47, nodeVar9 ) );
		nodeVar49 = normalize( cross( nodeVar9, nodeVar48 ) );
		nodeVar50 = vec3<f32>( dot( nodeVar48, nodeVar11 ), dot( nodeVar49, nodeVar11 ), dot( nodeVar9, nodeVar11 ) );
		let nodeConst10 = fract( ( nodeVar12 + ( nodeVar12 * vec4<f32>( 7.0 ) ) ) );
		nodeVar51 = fn130( nodeVar50, nodeVar45, nodeVar46, nodeConst10.x, nodeConst10.y );

		if ( ( nodeVar51.z < 0.0 ) ) {

			nodeVar51 = ( - nodeVar51 );
			

		}

		nodeVar52 = normalize( ( ( ( nodeVar48 * vec3<f32>( nodeVar51.x ) ) + ( nodeVar49 * vec3<f32>( nodeVar51.y ) ) ) + ( nodeVar9 * vec3<f32>( nodeVar51.z ) ) ) );
		nodeVar53 = normalize( reflect( ( - nodeVar11 ), nodeVar52 ) );
		nodeVar54 = nodeVar53;
		nodeVar55 = normalize( ( nodeVar11 + nodeVar54 ) );
		nodeVar56 = max( 0.0, dot( nodeVar9, nodeVar11 ) );
		nodeVar57 = max( 0.0, dot( nodeVar9, nodeVar54 ) );
		nodeVar58 = max( 0.0, dot( nodeVar9, nodeVar55 ) );
		nodeVar59 = max( 0.0, dot( nodeVar11, nodeVar55 ) );
		let nodeConst11 = mix( vec3<f32>( 0.04, 0.04, 0.04 ), nodeVar14, nodeConst8 );
		nodeVar60 = fn135( nodeConst11, nodeVar59, 1.0 );
		nodeVar61 = fn134( nodeVar58, nodeVar56, nodeVar45 );
		nodeVar62 = ( nodeVar45 * nodeVar45 );
		nodeVar63 = max( ( 1.0 - ( nodeVar56 * nodeVar56 ) ), 0.0 );
		nodeVar64 = ( 1.0 + sqrt( nodeVar63 ) );
		nodeVar65 = ( nodeVar64 * nodeVar64 );
		nodeVar66 = ( ( ( 1.0 - nodeVar62 ) * nodeVar65 ) / ( nodeVar65 + ( ( nodeVar62 * nodeVar56 ) * nodeVar56 ) ) );
		nodeVar67 = sqrt( ( ( nodeVar62 * nodeVar63 ) + ( nodeVar56 * nodeVar56 ) ) );
		nodeVar68 = ( ( ( nodeVar60 * vec3<f32>( fn133( nodeVar57, nodeVar56, nodeVar45 ) ) ) * vec3<f32>( ( ( nodeVar66 * nodeVar56 ) + nodeVar67 ) ) ) / vec3<f32>( max( ( 2.0 * nodeVar56 ), 0.0001 ) ) );
		nodeVar69 = StructType0( nodeVar53, nodeVar68, nodeVar61, nodeVar56, nodeVar45, 1.0, nodeConst11 );
		nodeVar43 = nodeVar69;
		

	}

	nodeVar70 = nodeVar43.reflectDir;
	nodeVar71 = nodeVar43.sampleWeight;
	nodeVar72 = fn136( nodeVar43.NdotV, nodeConst6 );
	nodeVar73 = ( object.nodeUniform8 / max( dot( ( - nodeVar10 ), nodeVar9 ), 0.0001 ) );
	nodeVar74 = ( nodeVar6 + ( nodeVar70 * vec3<f32>( nodeVar73 ) ) );

	if ( ( nodeVar74.z > ( - object.nodeUniform9 ) ) ) {

		nodeVar74 = ( nodeVar6 + ( nodeVar70 * vec3<f32>( ( ( ( - object.nodeUniform9 ) - nodeVar6.z ) / nodeVar70.z ) ) ) );
		

	}

	nodeVar75 = ( nodeVar2 * object.nodeUniform0 );
	nodeVar76 = ( fn22( nodeVar74, object.nodeUniform10 ) * object.nodeUniform0 );
	nodeVar77 = ( nodeVar76 - nodeVar75 );

	if ( ( nodeVar77.x >= 0.0 ) ) {

		nodeVar78 = ( object.nodeUniform0.x - nodeVar75.x );

	} else {

		nodeVar78 = nodeVar75.x;

	}


	if ( ( nodeVar77.y >= 0.0 ) ) {

		nodeVar79 = ( object.nodeUniform0.y - nodeVar75.y );

	} else {

		nodeVar79 = nodeVar75.y;

	}

	let nodeConst12 = ( vec2<f32>( nodeVar78, nodeVar79 ) / max( abs( nodeVar77 ), vec2<f32>( 0.000001 ) ) );
	let nodeConst13 = min( min( nodeConst12.x, nodeConst12.y ), 1.0 );

	if ( ( nodeConst13 < 1.0 ) ) {

		nodeVar76 = ( nodeVar75 + ( nodeVar77 * vec2<f32>( nodeConst13 ) ) );
		nodeVar74 = ( mix( ( nodeVar6 / vec3<f32>( nodeVar6.z ) ), ( nodeVar74 / vec3<f32>( nodeVar74.z ) ), nodeConst13 ) / vec3<f32>( mix( ( 1.0 / nodeVar6.z ), ( 1.0 / nodeVar74.z ), nodeConst13 ) ) );
		

	}

	nodeVar80 = ( nodeVar76.x - nodeVar75.x );
	nodeVar81 = ( nodeVar76.y - nodeVar75.y );
	nodeVar82 = max( max( abs( nodeVar80 ), abs( nodeVar81 ) ), 1.0 );
	let nodeConst14 = i32( max( ( clamp( object.nodeUniform11, 0.0, 1.0 ) * 64.0 ), 1.0 ) );
	nodeVar83 = ( nodeVar80 / f32( nodeConst14 ) );
	nodeVar84 = ( nodeVar81 / f32( nodeConst14 ) );
	nodeVar85 = vec2<f32>( nodeVar83, nodeVar84 );
	nodeVar86 = ( vec2<f32>( 1.0, 1.0 ) / object.nodeUniform0 );
	nodeVar87 = vec2<f32>( nodeVar86.x, 0.0 );
	nodeVar88 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );
	nodeVar89 = 0.0;
	let nodeConst15 = ( 1.0 / nodeVar6.z );
	let nodeConst16 = ( 1.0 / nodeVar74.z );
	nodeVar90 = false;
	nodeVar91 = vec2<f32>( 0.0, 0.0 );
	nodeVar92 = 0.0;
	nodeVar93 = 0.0;
	nodeVar94 = true;

	for ( var i : i32 = 1; i < nodeConst14; i ++ ) {

		let nodeConst17 = f32( i );
		nodeVar95 = max( pow( ( ( nodeConst17 + ( nodeVar1.z - 0.5 ) ) / f32( nodeConst14 ) ), 3.0 ), ( nodeConst17 / nodeVar82 ) );

		if ( ( nodeVar95 > 1.0 ) ) {

			break;
			

		}

		nodeVar96 = ( nodeVar75 + ( nodeVar85 * vec2<f32>( ( nodeVar95 * f32( nodeConst14 ) ) ) ) );

		if ( ( ( ( ( nodeVar96.x < 0.0 ) || ( nodeVar96.x > object.nodeUniform0.x ) ) || ( nodeVar96.y < 0.0 ) ) || ( nodeVar96.y > object.nodeUniform0.y ) ) ) {

			break;
			

		}

		nodeVar97 = ( nodeVar96 * nodeVar86 );
		nodeVar98 = textureLoad( nodeUniform2, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar97 ) * vec2<f32>( nodeVar4 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar4 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar99 = nodeVar98;
		nodeVar100 = ( ( object.nodeUniform9 * object.nodeUniform12 ) / ( ( ( object.nodeUniform12 - object.nodeUniform9 ) * nodeVar99 ) - object.nodeUniform12 ) );
		nodeVar101 = ( 1.0 / ( nodeConst15 + ( nodeVar95 * ( nodeConst16 - nodeConst15 ) ) ) );
		nodeVar102 = nodeVar93;
		nodeVar103 = nodeVar94;
		nodeVar93 = nodeVar95;
		nodeVar94 = ( nodeVar101 > nodeVar100 );

		if ( ( ( ! ( nodeVar99 >= 1.0 ) ) && ( nodeVar101 <= nodeVar100 ) ) ) {

			let nodeConst18 = ( object.nodeUniform3 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar97.x, ( 1.0 - nodeVar97.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar99 ), 1.0 ) );
			nodeVar104 = ( nodeConst18.xyz / vec3<f32>( nodeConst18.w ) );
			nodeVar105 = fn164( nodeVar104, nodeVar6, nodeVar74 );
			nodeVar106 = ( nodeVar97 + nodeVar87 );
			let nodeConst19 = ( object.nodeUniform3 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar106.x, ( 1.0 - nodeVar106.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar99 ), 1.0 ) );
			nodeVar107 = ( nodeConst19.xyz / vec3<f32>( nodeConst19.w ) );
			nodeVar108 = ( ( nodeVar107.x - nodeVar104.x ) * 3.0 );
			nodeVar109 = max( nodeVar108, object.nodeUniform13 );

			if ( ( nodeVar105 <= nodeVar109 ) ) {

				nodeVar110 = textureSample( nodeUniform5, nodeUniform5_sampler, nodeVar97 );
				nodeVar111 = normalize( ( ( nodeVar110.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) );

				if ( ( dot( nodeVar70, nodeVar111 ) >= 0.0 ) ) {

					continue;
					

				}

				nodeVar90 = true;
				nodeVar91 = nodeVar97;
				nodeVar92 = nodeVar99;
				break;
				

			}

			

		}


	}


	if ( nodeVar90 ) {

		let nodeConst20 = ( object.nodeUniform3 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar91.x, ( 1.0 - nodeVar91.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar92 ), 1.0 ) );
		nodeVar112 = ( nodeConst20.xyz / vec3<f32>( nodeConst20.w ) );
		nodeVar113 = ( 0.0 <= object.nodeUniform8 );

		if ( nodeVar113 ) {

			nodeVar114 = ( object.nodeUniform4 * vec4<f32>( nodeVar112, 1.0 ) ).xyz;
			nodeVar115 = ( distance( nodeVar7, nodeVar114 ) * nodeVar72 );
			nodeVar116 = textureSample( nodeUniform14, nodeUniform14_sampler, nodeVar91 );
			nodeVar117 = nodeVar116;
			nodeVar118 = textureSample( nodeUniform16, nodeUniform16_sampler, nodeVar91 );
			nodeVar119 = textureSample( nodeUniform15, nodeUniform15_sampler, ( nodeVar91 - nodeVar118.xy ) );
			nodeVar120 = nodeVar119;
			nodeVar121 = ( nodeVar117.xyz + ( nodeVar120.xyz * vec3<f32>( ( 1.0 - nodeVar120.w ) ) ) );
			nodeVar117.x = nodeVar121[ 0 ];
			nodeVar117.y = nodeVar121[ 1 ];
			nodeVar117.z = nodeVar121[ 2 ];
			let nodeConst21 = fn166( nodeVar91, ( object.nodeUniform17 * ( 1.0 - min( ( nodeConst6 / 0.25 ), 1.0 ) ) ) );

			if ( ( nodeConst21 < 1.0 ) ) {

				nodeVar122 = vec3<f32>( 0.0, 0.0, 0.0 );
				nodeVar123 = normalize( ( object.nodeUniform4 * vec4<f32>( nodeVar9, 0.0 ) ).xyz );
				nodeVar124 = normalize( ( object.nodeUniform4 * vec4<f32>( nodeVar11, 0.0 ) ).xyz );
				nodeVar125 = max( 0.0, dot( nodeVar123, nodeVar124 ) );
				nodeVar126 = normalize( ( object.nodeUniform4 * vec4<f32>( nodeVar70, 0.0 ) ).xyz );
				nodeVar127 = normalize( ( nodeVar124 + nodeVar126 ) );
				nodeVar128 = max( 0.0, dot( nodeVar123, nodeVar126 ) );
				nodeVar129 = max( 0.0, dot( nodeVar124, nodeVar127 ) );
				nodeVar130 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, vec2<f32>( ( ( atan2( nodeVar126.z, nodeVar126.x ) * 0.15915494309189535 ) + 0.5 ), ( ( asin( clamp( nodeVar126.y, -1.0, 1.0 ) ) * 0.3183098861837907 ) + 0.5 ) ), 0.0 );
				nodeVar122 = ( ( nodeVar130.xyz * ( ( fn135( nodeVar43.f0, nodeVar129, nodeVar43.f90 ) * vec3<f32>( fn133( nodeVar128, nodeVar125, nodeVar43.alpha ) ) ) / vec3<f32>( max( fn132( nodeVar125, nodeVar43.alpha ), 0.0001 ) ) ) ) * vec3<f32>( object.nodeUniform19 ) );
				nodeVar131 = mix( ( nodeVar122 * vec3<f32>( object.nodeUniform20 ) ), nodeVar117.xyz, nodeConst21 );
				nodeVar117.x = nodeVar131[ 0 ];
				nodeVar117.y = nodeVar131[ 1 ];
				nodeVar117.z = nodeVar131[ 2 ];
				

			}

			nodeVar89 = 1.0;
			nodeVar88 = vec4<f32>( ( nodeVar117.xyz * nodeVar71 ), nodeVar115 );
			

		}

		

	}


	if ( ( nodeVar89 == 0.0 ) ) {

		nodeVar132 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar133 = normalize( ( object.nodeUniform4 * vec4<f32>( nodeVar9, 0.0 ) ).xyz );
		nodeVar134 = normalize( ( object.nodeUniform4 * vec4<f32>( nodeVar11, 0.0 ) ).xyz );
		nodeVar135 = max( 0.0, dot( nodeVar133, nodeVar134 ) );
		nodeVar136 = normalize( ( object.nodeUniform4 * vec4<f32>( nodeVar70, 0.0 ) ).xyz );
		nodeVar137 = normalize( ( nodeVar134 + nodeVar136 ) );
		nodeVar138 = max( 0.0, dot( nodeVar133, nodeVar136 ) );
		nodeVar139 = max( 0.0, dot( nodeVar134, nodeVar137 ) );
		nodeVar140 = textureSampleLevel( nodeUniform18, nodeUniform18_sampler, vec2<f32>( ( ( atan2( nodeVar136.z, nodeVar136.x ) * 0.15915494309189535 ) + 0.5 ), ( ( asin( clamp( nodeVar136.y, -1.0, 1.0 ) ) * 0.3183098861837907 ) + 0.5 ) ), 0.0 );
		nodeVar132 = ( ( nodeVar140.xyz * ( ( fn135( nodeVar43.f0, nodeVar139, nodeVar43.f90 ) * vec3<f32>( fn133( nodeVar138, nodeVar135, nodeVar43.alpha ) ) ) / vec3<f32>( max( fn132( nodeVar135, nodeVar43.alpha ), 0.0001 ) ) ) ) * vec3<f32>( object.nodeUniform19 ) );
		nodeVar88 = vec4<f32>( ( nodeVar132 * vec3<f32>( object.nodeUniform20 ) ), 10000.0 );
		

	}

	nodeVar141 = max( dot( nodeVar88.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ), 0.0001 );
	nodeVar142 = ( nodeVar88.xyz * vec3<f32>( min( ( object.nodeUniform21 / nodeVar141 ), 1.0 ) ) );
	nodeVar88.x = nodeVar142[ 0 ];
	nodeVar88.y = nodeVar142[ 1 ];
	nodeVar88.z = nodeVar142[ 2 ];
	nodeVar143 = ( nodeVar88.xyz * vec3<f32>( object.nodeUniform22 ) );
	nodeVar88.x = nodeVar143[ 0 ];
	nodeVar88.y = nodeVar143[ 1 ];
	nodeVar88.z = nodeVar143[ 2 ];

	// result

	output.color = max( nodeVar88, vec4<f32>( 0.0 ) );

	return output;

}
