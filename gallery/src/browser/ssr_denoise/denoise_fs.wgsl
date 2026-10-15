// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 0 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 0 ) var nodeUniform3 : texture_depth_2d;
@binding( 4 ) @group( 0 ) var nodeUniform4_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform4 : texture_2d<f32>;
@binding( 6 ) @group( 0 ) var nodeUniform6_sampler : sampler;
@binding( 7 ) @group( 0 ) var nodeUniform6 : texture_2d<f32>;
@binding( 8 ) @group( 0 ) var nodeUniform8_sampler : sampler;
@binding( 9 ) @group( 0 ) var nodeUniform8 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : vec2<f32>,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : u32,
	nodeUniform21 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec2<u32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : bool;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : i32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : bool;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec4<f32>;

// codes
fn computeFrustumSize ( viewZ : f32, tanHalfFovY : f32 ) -> f32 {

	


	return ( ( 2.0 * viewZ ) * tanHalfFovY );

}


fn temporalWeight ( x : f32, strength : f32 ) -> f32 {

	


	return ( 1.0 / pow( x, strength ) );

}


fn specularLobeTanHalfAngle ( roughness : f32, percent : f32 ) -> f32 {

	


	return ( ( roughness * roughness ) * sqrt( ( percent / max( ( 1.0 - percent ), 0.000001 ) ) ) );

}


fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn getNeighborhoodStats ( uvCoord : vec2<f32>, centerSample : vec4<f32> ) -> vec4<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : bool;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : vec4<f32>;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : vec4<f32>;
	var nodeVar14 : f32;
	var nodeVar15 : f32;
	var nodeVar16 : f32;
	var nodeVar17 : vec4<f32>;
	var nodeVar18 : f32;
	var nodeVar19 : f32;
	var nodeVar20 : f32;
	var nodeVar21 : vec4<f32>;
	var nodeVar22 : f32;
	var nodeVar23 : f32;
	var nodeVar24 : f32;

	nodeVar0 = 0.0;
	nodeVar1 = 0.0;
	nodeVar2 = 0.0;
	nodeVar3 = 0.0;
	nodeVar4 = 0.0;
	nodeVar5 = false;
	nodeVar6 = centerSample.w;

	if ( ( nodeVar6 > 1000.0 ) ) {

		nodeVar6 = 0.25;
		nodeVar5 = true;
		

	}

	nodeVar7 = ( 1.0 / ( nodeVar6 + 0.001 ) );
	nodeVar0 = ( nodeVar0 + ( nodeVar6 * nodeVar7 ) );
	nodeVar1 = ( nodeVar1 + nodeVar7 );

	if ( ( object.nodeUniform0 > 0.0 ) ) {

		nodeVar4 = ( nodeVar4 + 1.0 );
		nodeVar8 = dot( centerSample.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
		let nodeConst0 = ( nodeVar8 - nodeVar2 );
		nodeVar2 = ( nodeVar2 + ( nodeConst0 / nodeVar4 ) );
		nodeVar3 = ( nodeVar3 + ( nodeConst0 * ( nodeVar8 - nodeVar2 ) ) );
		

	}

	nodeVar9 = textureSample( nodeUniform1, nodeUniform1_sampler, ( uvCoord + ( vec2<f32>( -1.0, 0.0 ) / object.nodeUniform2 ) ) );
	let nodeConst1 = max( nodeVar9, vec4<f32>( 0.0 ) );
	nodeVar10 = nodeConst1.w;

	if ( ( nodeVar10 > 1000.0 ) ) {

		nodeVar10 = 0.25;
		nodeVar5 = true;
		

	}

	nodeVar11 = ( 1.0 / ( nodeVar10 + 0.001 ) );
	nodeVar0 = ( nodeVar0 + ( nodeVar10 * nodeVar11 ) );
	nodeVar1 = ( nodeVar1 + nodeVar11 );

	if ( ( object.nodeUniform0 > 0.0 ) ) {

		nodeVar4 = ( nodeVar4 + 1.0 );
		nodeVar12 = dot( nodeConst1.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
		let nodeConst2 = ( nodeVar12 - nodeVar2 );
		nodeVar2 = ( nodeVar2 + ( nodeConst2 / nodeVar4 ) );
		nodeVar3 = ( nodeVar3 + ( nodeConst2 * ( nodeVar12 - nodeVar2 ) ) );
		

	}

	nodeVar13 = textureSample( nodeUniform1, nodeUniform1_sampler, ( uvCoord + ( vec2<f32>( 1.0, 0.0 ) / object.nodeUniform2 ) ) );
	let nodeConst3 = max( nodeVar13, vec4<f32>( 0.0 ) );
	nodeVar14 = nodeConst3.w;

	if ( ( nodeVar14 > 1000.0 ) ) {

		nodeVar14 = 0.25;
		nodeVar5 = true;
		

	}

	nodeVar15 = ( 1.0 / ( nodeVar14 + 0.001 ) );
	nodeVar0 = ( nodeVar0 + ( nodeVar14 * nodeVar15 ) );
	nodeVar1 = ( nodeVar1 + nodeVar15 );

	if ( ( object.nodeUniform0 > 0.0 ) ) {

		nodeVar4 = ( nodeVar4 + 1.0 );
		nodeVar16 = dot( nodeConst3.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
		let nodeConst4 = ( nodeVar16 - nodeVar2 );
		nodeVar2 = ( nodeVar2 + ( nodeConst4 / nodeVar4 ) );
		nodeVar3 = ( nodeVar3 + ( nodeConst4 * ( nodeVar16 - nodeVar2 ) ) );
		

	}

	nodeVar17 = textureSample( nodeUniform1, nodeUniform1_sampler, ( uvCoord + ( vec2<f32>( 0.0, -1.0 ) / object.nodeUniform2 ) ) );
	let nodeConst5 = max( nodeVar17, vec4<f32>( 0.0 ) );
	nodeVar18 = nodeConst5.w;

	if ( ( nodeVar18 > 1000.0 ) ) {

		nodeVar18 = 0.25;
		nodeVar5 = true;
		

	}

	nodeVar19 = ( 1.0 / ( nodeVar18 + 0.001 ) );
	nodeVar0 = ( nodeVar0 + ( nodeVar18 * nodeVar19 ) );
	nodeVar1 = ( nodeVar1 + nodeVar19 );

	if ( ( object.nodeUniform0 > 0.0 ) ) {

		nodeVar4 = ( nodeVar4 + 1.0 );
		nodeVar20 = dot( nodeConst5.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
		let nodeConst6 = ( nodeVar20 - nodeVar2 );
		nodeVar2 = ( nodeVar2 + ( nodeConst6 / nodeVar4 ) );
		nodeVar3 = ( nodeVar3 + ( nodeConst6 * ( nodeVar20 - nodeVar2 ) ) );
		

	}

	nodeVar21 = textureSample( nodeUniform1, nodeUniform1_sampler, ( uvCoord + ( vec2<f32>( 0.0, 1.0 ) / object.nodeUniform2 ) ) );
	let nodeConst7 = max( nodeVar21, vec4<f32>( 0.0 ) );
	nodeVar22 = nodeConst7.w;

	if ( ( nodeVar22 > 1000.0 ) ) {

		nodeVar22 = 0.25;
		nodeVar5 = true;
		

	}

	nodeVar23 = ( 1.0 / ( nodeVar22 + 0.001 ) );
	nodeVar0 = ( nodeVar0 + ( nodeVar22 * nodeVar23 ) );
	nodeVar1 = ( nodeVar1 + nodeVar23 );

	if ( ( object.nodeUniform0 > 0.0 ) ) {

		nodeVar4 = ( nodeVar4 + 1.0 );
		nodeVar24 = dot( nodeConst7.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
		let nodeConst8 = ( nodeVar24 - nodeVar2 );
		nodeVar2 = ( nodeVar2 + ( nodeConst8 / nodeVar4 ) );
		nodeVar3 = ( nodeVar3 + ( nodeConst8 * ( nodeVar24 - nodeVar2 ) ) );
		

	}


	return vec4<f32>( ( nodeVar0 / nodeVar1 ), nodeVar2, sqrt( ( nodeVar3 / max( nodeVar4, 1.0 ) ) ), f32( nodeVar5 ) );

}


fn computeHitDistFactor ( worldRayLength : f32, viewZ : f32, tanHalfFovY : f32 ) -> f32 {

	


	return clamp( ( worldRayLength / max( computeFrustumSize( viewZ, tanHalfFovY ), 0.000001 ) ), 0.0, 1.0 );

}


fn getTemporalVarianceFactor ( frameNum : f32, strength : f32 ) -> f32 {

	


	return max( temporalWeight( frameNum, strength ), 0.05 );

}


fn getSpecularDominantDirection ( N : vec3<f32>, V : vec3<f32>, roughness : f32 ) -> vec3<f32> {

	


	return normalize( mix( N, reflect( ( - V ), N ), ( 1.0 - roughness ) ) );

}


fn lobeNormalFalloff ( roughness : f32, aggressivity : f32, invNormalPhi : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = ( 1.0 / max( atan( specularLobeTanHalfAngle( roughness, clamp( mix( ( invNormalPhi * invNormalPhi ), 0.0, sqrt( aggressivity ) ), 0.1, 0.99 ) ) ), 0.0058823529411764705 ) );

	return ( ( nodeVar0 * nodeVar0 ) * 8.0 );

}


fn vogelDisk ( i : f32, radius : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( i + 0.5 ) * 2.399827721492203 );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( ( radius * sqrt( ( ( i + 0.5 ) / 8.0 ) ) ) ) );

}


fn tsl_mod_vec2( x : vec2f, y : vec2f ) -> vec2f { return x - y * floor( x / y ); }
fn planeDistance ( position : vec3<f32>, nPosition : vec3<f32>, normal : vec3<f32> ) -> f32 {

	


	return abs( dot( ( position - nPosition ), normal ) );

}


fn lobeNormalWeight ( viewNormal : vec3<f32>, nNormalV : vec3<f32>, lobeFalloff : f32 ) -> f32 {

	


	return exp( ( ( dot( viewNormal, nNormalV ) - 1.0 ) * lobeFalloff ) );

}


fn karisTemporalBlend ( denoisedRgb : vec3<f32>, denoisedRaw : vec3<f32>, a : f32, flickerSuppression : f32, adaptiveTrust : f32, nbhdMeanLuma : f32, nbhdStddevLuma : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;

	nodeVar0 = ( nbhdStddevLuma / max( nbhdMeanLuma, 0.0001 ) );
	nodeVar1 = ( a * ( 1.0 - clamp( ( ( nodeVar0 * adaptiveTrust ) * ( 1.0 - a ) ), 0.0, 0.9 ) ) );
	nodeVar2 = ( flickerSuppression * mix( ( 1.0 - adaptiveTrust ), 1.0, smoothstep( 0.1, 2.0, nodeVar0 ) ) );
	nodeVar3 = ( ( 1.0 - nodeVar1 ) / ( ( ( dot( denoisedRgb, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * nodeVar2 ) * 10.0 ) + 1.0 ) );
	nodeVar4 = ( nodeVar1 / ( ( ( dot( denoisedRaw, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * nodeVar2 ) * 10.0 ) + 1.0 ) );

	return ( ( ( denoisedRgb * vec3<f32>( nodeVar3 ) ) + ( denoisedRaw * vec3<f32>( nodeVar4 ) ) ) / vec3<f32>( max( ( nodeVar3 + nodeVar4 ), 0.000001 ) ) );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureDimensions( nodeUniform3, u32( 0 ) );
	nodeVar0 = textureLoad( nodeUniform3, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst0 = nodeVar0;

	if ( ( nodeConst0 >= 1.0 ) ) {

		discard;
		

	} else {

		nodeVar2 = textureSample( nodeUniform4, nodeUniform4_sampler, nodeVarying0 );
		let nodeConst1 = ( ( nodeVar2.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) );
		let nodeConst2 = normalize( ( vec4<f32>( nodeConst1, 0.0 ) * object.nodeUniform5 ).xyz );
		nodeVar3 = textureSample( nodeUniform6, nodeUniform6_sampler, nodeVarying0 );
		let nodeConst3 = max( max( nodeVar3, vec4<f32>( 0.0 ) ), vec4<f32>( 0.0 ) );
		let nodeConst4 = ( ( object.nodeUniform7 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst0 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform7 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst0 ), 1.0 ) ).w ) );
		nodeVar4 = textureSample( nodeUniform8, nodeUniform8_sampler, nodeVarying0 );
		nodeVar5 = textureSample( nodeUniform4, nodeUniform4_sampler, nodeVarying0 );
		let nodeConst5 = vec2<f32>( nodeVar4.w, nodeVar5.w );
		nodeVar6 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
		let nodeConst6 = max( nodeVar6, vec4<f32>( 0.0 ) );
		nodeVar7 = 1.0;
		nodeVar8 = 0.0;
		nodeVar9 = 0.0;
		nodeVar10 = false;
		nodeVar11 = getNeighborhoodStats( nodeVarying0, nodeConst6 );
		nodeVar7 = nodeVar11.x;
		nodeVar8 = nodeVar11.y;
		nodeVar9 = nodeVar11.z;
		nodeVar10 = ( nodeVar11.w > 0.5 );
		let nodeConst7 = tan( ( object.nodeUniform9 * 0.5 ) );
		let nodeConst8 = computeHitDistFactor( nodeVar7, abs( nodeConst4.z ), nodeConst7 );
		nodeVar12 = nodeConst3.xyz;
		nodeVar13 = 1.0;
		nodeVar14 = ( 1.0 / nodeConst3.w );
		nodeVar15 = nodeVar14;
		nodeVar16 = 1.0;
		nodeVar17 = nodeConst6.xyz;
		nodeVar18 = 1.0;

		if ( ( length( nodeConst6.xyz ) < 0.0001 ) ) {

			nodeVar17 = vec3<f32>( 0.0, 0.0, 0.0 );
			nodeVar18 = 0.0;
			

		}

		nodeVar19 = ( object.nodeUniform10 * 0.1 );
		nodeVar19 = ( nodeVar19 * ( nodeVar7 * abs( nodeConst4.z ) ) );
		nodeVar19 = ( nodeVar19 * max( sqrt( nodeConst5.y ), 0.01 ) );
		nodeVar20 = ( 1.0 - getTemporalVarianceFactor( nodeVar14, ( 1.0 - object.nodeUniform11 ) ) );
		nodeVar19 = ( nodeVar19 * mix( 1.0, 0.001, nodeVar20 ) );
		nodeVar21 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar22 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar23 = reflect( ( - getSpecularDominantDirection( nodeConst1, ( - normalize( nodeConst4 ) ), nodeConst5.y ) ), nodeConst1 );
		nodeVar24 = normalize( cross( nodeConst1, nodeVar23 ) );
		nodeVar21 = ( nodeVar24 * vec3<f32>( mix( 1.0, nodeConst5.y, clamp( ( acos( abs( nodeConst1.z ) ) / 1.5707963267948966 ), 0.0, 1.0 ) ) ) );
		nodeVar22 = cross( nodeVar23, nodeVar24 );
		nodeVar21 = ( nodeVar21 * vec3<f32>( nodeVar19 ) );
		nodeVar22 = ( nodeVar22 * vec3<f32>( nodeVar19 ) );
		let nodeConst9 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar25 = 1.0;
		nodeVar26 = vec2<f32>( 0.0, 0.0 );
		let nodeConst10 = lobeNormalFalloff( nodeConst5.y, nodeVar20, ( 1.0 - object.nodeUniform12 ) );

		for ( var i : i32 = 0; i < 8; i ++ ) {

			nodeVar27 = vogelDisk( f32( i ), 1.0 );
			let nodeConst11 = normalize( nodeVar27 );
			nodeVar28 = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
			nodeVar29 = ( i32( object.nodeUniform13 ) + 83 );
			nodeVar30 = tsl_mod_vec2( ( floor( ( nodeVarying0 * object.nodeUniform2 ) ) + floor( ( fract( vec2<f32>( ( f32( nodeVar29 ) * 0.7548776662 ), ( f32( nodeVar29 ) * 0.569840291 ) ) ) * vec2<f32>( 32.0 ) ) ) ), vec2<f32>( 32.0 ) );
			nodeVar31 = ( ( ( nodeVar30.x * 0.7548776662466927 ) + ( nodeVar30.y * 0.5698402909980532 ) ) + 83.0 );
			nodeVar28 = vec4<f32>( fract( ( ( nodeVar31 * 1.324717957244746 ) * 0.7548776662466927 ) ), fract( ( ( nodeVar31 * 2.649435914489492 ) * 0.5698402909980532 ) ), fract( ( ( nodeVar31 * 3.974153871734238 ) * 0.419875421 ) ), fract( ( ( nodeVar31 * 5.298871828978984 ) * 0.43015970900194667 ) ) );
			nodeVar32 = ( ( nodeVar28.x * 2.0 ) * 3.141592653589793 );

			if ( ( dot( nodeVar26, nodeVar26 ) > 0.001 ) ) {

				nodeVar33 = 1.0;

			} else {

				nodeVar33 = 0.0;

			}

			nodeVar34 = ( mat2x2<f32>( cos( nodeVar32 ), ( - sin( nodeVar32 ) ), sin( nodeVar32 ), cos( nodeVar32 ) ) * ( mix( nodeConst11, normalize( max( nodeVar26, vec2<f32>( 0.000001 ) ) ), ( ( object.nodeUniform14 * nodeVar20 ) * nodeVar33 ) ) * vec2<f32>( ( length( nodeVar27 ) * nodeVar25 ) ) ) );
			nodeVar35 = ( object.nodeUniform15 * vec4<f32>( ( nodeConst4 + ( ( nodeVar22 * vec3<f32>( nodeVar34.x ) ) + ( nodeVar21 * vec3<f32>( nodeVar34.y ) ) ) ), 1.0 ) );
			nodeVar36 = ( ( ( nodeVar35.xy / vec2<f32>( nodeVar35.w ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) );
			nodeVar37 = vec2<f32>( nodeVar36.x, ( 1.0 - nodeVar36.y ) );
			nodeVar37 = clamp( ( vec2<f32>( 1.0 ) - abs( ( vec2<f32>( 1.0 ) - abs( nodeVar37 ) ) ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) );
			nodeVar38 = textureSample( nodeUniform6, nodeUniform6_sampler, nodeVar37 );
			let nodeConst12 = max( max( nodeVar38, vec4<f32>( 0.0 ) ), vec4<f32>( 0.0 ) );
			nodeVar39 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar37 );
			nodeVar40 = max( max( nodeVar39, vec4<f32>( 0.0 ) ), vec4<f32>( 0.0 ) );
			nodeVar41 = textureLoad( nodeUniform3, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar37 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst13 = ( ( object.nodeUniform7 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar37.x, ( 1.0 - nodeVar37.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar41 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform7 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar37.x, ( 1.0 - nodeVar37.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar41 ), 1.0 ) ).w ) );
			let nodeConst14 = abs( nodeConst13.z );
			nodeVar42 = 0.0;
			nodeVar42 = ( nodeVar42 + ( ( abs( ( dot( nodeVar40.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - dot( nodeConst6.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ) ) * object.nodeUniform16 ) * 10.0 ) );

			if ( ( ( nodeVar40.w > 1000.0 ) && nodeVar10 ) ) {

				nodeVar43 = 1.0;

			} else {

				nodeVar43 = ( ( abs( ( nodeConst8 - computeHitDistFactor( nodeVar40.w, nodeConst14, nodeConst7 ) ) ) * object.nodeUniform17 ) / abs( nodeConst4.z ) );

			}

			nodeVar42 = ( nodeVar42 + nodeVar43 );
			nodeVar44 = textureSample( nodeUniform8, nodeUniform8_sampler, nodeVar37 );
			nodeVar45 = textureSample( nodeUniform4, nodeUniform4_sampler, nodeVar37 );
			nodeVar42 = ( nodeVar42 + ( abs( ( nodeConst5.y - vec2<f32>( nodeVar44.w, nodeVar45.w ).y ) ) * object.nodeUniform18 ) );
			nodeVar46 = textureSample( nodeUniform4, nodeUniform4_sampler, nodeVar37 );
			nodeVar47 = ( exp( ( - ( ( nodeVar42 * nodeVar20 ) + ( planeDistance( nodeConst4, nodeConst13, nodeConst1 ) * ( ( ( object.nodeUniform19 * 500.0 ) * abs( nodeConst1.z ) ) / abs( nodeConst4.z ) ) ) ) ) ) * lobeNormalWeight( nodeConst2, normalize( ( vec4<f32>( ( ( nodeVar46.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform5 ).xyz ), nodeConst10 ) );
			nodeVar25 = mix( nodeVar25, nodeVar47, object.nodeUniform14 );
			nodeVar26 = mix( nodeVar26, ( nodeConst11 * vec2<f32>( ( nodeVar47 - 0.5 ) ) ), 0.5 );
			nodeVar47 = ( nodeVar47 * mix( ( 1.0 / ( pow( dot( nodeVar40.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ), 2.0 ) + 0.01 ) ), 1.0, min( ( nodeVar14 / 5.0 ), 1.0 ) ) );
			nodeVar17 = ( nodeVar17 + ( nodeVar40.xyz * vec3<f32>( nodeVar47 ) ) );
			nodeVar18 = ( nodeVar18 + nodeVar47 );
			nodeVar12 = ( nodeVar12 + ( nodeConst12.xyz * vec3<f32>( nodeVar47 ) ) );
			nodeVar13 = ( nodeVar13 + nodeVar47 );
			nodeVar48 = bool( object.nodeUniform20 );

			if ( nodeVar48 ) {


				if ( ( nodeConst12.w > nodeConst3.w ) ) {

					nodeVar49 = ( nodeVar47 * 0.33 );

				} else {

					nodeVar49 = 0.0;

				}

				nodeVar15 = ( nodeVar15 + ( ( 1.0 / nodeConst12.w ) * nodeVar49 ) );
				nodeVar16 = ( nodeVar16 + nodeVar49 );
				

			}


		}

		nodeVar12 = ( nodeVar12 / vec3<f32>( max( nodeVar13, 0.000001 ) ) );
		nodeVar12 = max( nodeVar12, vec3<f32>( 0.000001 ) );
		nodeVar17 = ( nodeVar17 / vec3<f32>( max( nodeVar18, 0.000001 ) ) );
		let nodeConst15 = ( 1.0 / max( ( nodeVar15 / max( nodeVar16, 0.000001 ) ), 0.000001 ) );
		nodeVar50 = vec4<f32>( karisTemporalBlend( nodeVar12, nodeVar17, nodeConst15, object.nodeUniform21, object.nodeUniform0, nodeVar8, nodeVar9 ), nodeConst15 );
		

	}


	// result

	output.color = nodeVar50;

	return output;

}
