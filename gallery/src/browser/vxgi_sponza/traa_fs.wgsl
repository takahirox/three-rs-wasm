// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct StructType0 {
	closestDepth : f32,
	closestPositionTexel : vec2<f32>,
	farthestDepth : f32
};

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_2d<f32>;
@binding( 2 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform1 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform2 : texture_depth_2d;
@binding( 6 ) @group( 0 ) var nodeUniform7 : texture_depth_2d;
@binding( 7 ) @group( 0 ) var nodeUniform9_sampler : sampler;
@binding( 8 ) @group( 0 ) var nodeUniform9 : texture_2d<f32>;

struct objectStruct {
	nodeUniform3 : vec2<f32>,
	nodeUniform4 : mat4x4<f32>,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : mat4x4<f32>,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform11 : f32
};
@binding( 5 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec2<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : StructType0;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec2<u32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec4<f32>;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : vec4<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : vec4<f32>;
var<private> nodeVar54 : vec4<f32>;
var<private> nodeVar55 : vec4<f32>;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec4<f32>;
var<private> nodeVar58 : vec4<f32>;
var<private> nodeVar59 : vec4<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec4<f32>;
var<private> nodeVar62 : vec4<f32>;
var<private> nodeVar63 : vec4<f32>;
var<private> nodeVar64 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar65 : vec4<f32>;

// codes
fn subpixelCorrection ( velocityUV : vec2<f32>, textureSize : vec2<i32> ) -> f32 {

	var nodeVar0 : vec2<f32>;
	var nodeVar1 : vec2<f32>;

	nodeVar0 = abs( fract( ( velocityUV * vec2<f32>( textureSize ) ) ) );
	nodeVar1 = max( nodeVar0, ( vec2<f32>( 1.0 ) - nodeVar0 ) );

	return ( ( 1.0 - ( nodeVar1.x * nodeVar1.y ) ) / 0.75 );

}


fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn clipAABB ( currentColor : vec4<f32>, historyColor : vec4<f32>, minColor : vec4<f32>, maxColor : vec4<f32> ) -> vec4<f32> {

	var nodeVar0 : vec4<f32>;

	let nodeConst0 = ( ( maxColor.xyz + minColor.xyz ) * vec3<f32>( 0.5 ) );
	let nodeConst1 = ( ( ( maxColor.xyz - minColor.xyz ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 1e-7 ) );
	let nodeConst2 = ( historyColor - vec4<f32>( nodeConst0, currentColor.w ) );
	let nodeConst3 = ( nodeConst2.xyz / nodeConst1 );
	let nodeConst4 = abs( nodeConst3 );
	let nodeConst5 = max( max( nodeConst4.x, nodeConst4.y ), nodeConst4.z );

	if ( ( nodeConst5 > 1.0 ) ) {

		nodeVar0 = ( vec4<f32>( nodeConst0, currentColor.w ) + ( nodeConst2 / vec4<f32>( nodeConst5 ) ) );

	} else {

		nodeVar0 = historyColor;

	}


	return nodeVar0;

}


fn flickerReduction ( currentColor : vec4<f32>, historyColor : vec4<f32>, currentWeight : f32 ) -> vec4<f32> {

	

	let nodeConst0 = ( currentColor * vec4<f32>( ( 1.0 / ( max( max( currentColor.x, currentColor.y ), currentColor.z ) + 1.0 ) ) ) );
	let nodeConst1 = ( historyColor * vec4<f32>( ( 1.0 / ( max( max( historyColor.x, historyColor.y ), historyColor.z ) + 1.0 ) ) ) );
	let nodeConst2 = dot( nodeConst0.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
	let nodeConst3 = dot( nodeConst1.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
	let nodeConst4 = ( currentWeight / ( nodeConst2 + 1.0 ) );
	let nodeConst5 = ( ( 1.0 - currentWeight ) / ( nodeConst3 + 1.0 ) );

	return ( ( ( currentColor * vec4<f32>( nodeConst4 ) ) + ( historyColor * vec4<f32>( nodeConst5 ) ) ) / vec4<f32>( max( ( nodeConst4 + nodeConst5 ), 0.00001 ) ) );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = 0.05;
	nodeVar1 = 2.0;
	nodeVar2 = vec2<f32>( 0.0, 0.0 );
	nodeVar3 = -1.0;
	nodeVar4 = ( nodeVarying0 * vec2<f32>( textureDimensions( nodeUniform1, 0 ) ) );
	nodeVar5 = ( nodeVar4 + vec2<f32>( -1.0, -1.0 ) );
	nodeVar6 = textureLoad( nodeUniform2, vec2<i32>( nodeVar5 ), u32( 0u ) );
	nodeVar7 = nodeVar6;

	if ( ( nodeVar7 < nodeVar1 ) ) {

		nodeVar1 = nodeVar7;
		nodeVar2 = nodeVar5;
		

	}


	if ( ( nodeVar7 > nodeVar3 ) ) {

		nodeVar3 = nodeVar7;
		

	}

	nodeVar8 = ( nodeVar4 + vec2<f32>( -1.0, 0.0 ) );
	nodeVar9 = textureLoad( nodeUniform2, vec2<i32>( nodeVar8 ), u32( 0u ) );
	nodeVar10 = nodeVar9;

	if ( ( nodeVar10 < nodeVar1 ) ) {

		nodeVar1 = nodeVar10;
		nodeVar2 = nodeVar8;
		

	}


	if ( ( nodeVar10 > nodeVar3 ) ) {

		nodeVar3 = nodeVar10;
		

	}

	nodeVar11 = ( nodeVar4 + vec2<f32>( -1.0, 1.0 ) );
	nodeVar12 = textureLoad( nodeUniform2, vec2<i32>( nodeVar11 ), u32( 0u ) );
	nodeVar13 = nodeVar12;

	if ( ( nodeVar13 < nodeVar1 ) ) {

		nodeVar1 = nodeVar13;
		nodeVar2 = nodeVar11;
		

	}


	if ( ( nodeVar13 > nodeVar3 ) ) {

		nodeVar3 = nodeVar13;
		

	}

	nodeVar14 = ( nodeVar4 + vec2<f32>( 0.0, -1.0 ) );
	nodeVar15 = textureLoad( nodeUniform2, vec2<i32>( nodeVar14 ), u32( 0u ) );
	nodeVar16 = nodeVar15;

	if ( ( nodeVar16 < nodeVar1 ) ) {

		nodeVar1 = nodeVar16;
		nodeVar2 = nodeVar14;
		

	}


	if ( ( nodeVar16 > nodeVar3 ) ) {

		nodeVar3 = nodeVar16;
		

	}

	nodeVar17 = ( nodeVar4 + vec2<f32>( 0.0, 0.0 ) );
	nodeVar18 = textureLoad( nodeUniform2, vec2<i32>( nodeVar17 ), u32( 0u ) );
	nodeVar19 = nodeVar18;

	if ( ( nodeVar19 < nodeVar1 ) ) {

		nodeVar1 = nodeVar19;
		nodeVar2 = nodeVar17;
		

	}


	if ( ( nodeVar19 > nodeVar3 ) ) {

		nodeVar3 = nodeVar19;
		

	}

	nodeVar20 = ( nodeVar4 + vec2<f32>( 0.0, 1.0 ) );
	nodeVar21 = textureLoad( nodeUniform2, vec2<i32>( nodeVar20 ), u32( 0u ) );
	nodeVar22 = nodeVar21;

	if ( ( nodeVar22 < nodeVar1 ) ) {

		nodeVar1 = nodeVar22;
		nodeVar2 = nodeVar20;
		

	}


	if ( ( nodeVar22 > nodeVar3 ) ) {

		nodeVar3 = nodeVar22;
		

	}

	nodeVar23 = ( nodeVar4 + vec2<f32>( 1.0, -1.0 ) );
	nodeVar24 = textureLoad( nodeUniform2, vec2<i32>( nodeVar23 ), u32( 0u ) );
	nodeVar25 = nodeVar24;

	if ( ( nodeVar25 < nodeVar1 ) ) {

		nodeVar1 = nodeVar25;
		nodeVar2 = nodeVar23;
		

	}


	if ( ( nodeVar25 > nodeVar3 ) ) {

		nodeVar3 = nodeVar25;
		

	}

	nodeVar26 = ( nodeVar4 + vec2<f32>( 1.0, 0.0 ) );
	nodeVar27 = textureLoad( nodeUniform2, vec2<i32>( nodeVar26 ), u32( 0u ) );
	nodeVar28 = nodeVar27;

	if ( ( nodeVar28 < nodeVar1 ) ) {

		nodeVar1 = nodeVar28;
		nodeVar2 = nodeVar26;
		

	}


	if ( ( nodeVar28 > nodeVar3 ) ) {

		nodeVar3 = nodeVar28;
		

	}

	nodeVar29 = ( nodeVar4 + vec2<f32>( 1.0, 1.0 ) );
	nodeVar30 = textureLoad( nodeUniform2, vec2<i32>( nodeVar29 ), u32( 0u ) );
	nodeVar31 = nodeVar30;

	if ( ( nodeVar31 < nodeVar1 ) ) {

		nodeVar1 = nodeVar31;
		nodeVar2 = nodeVar29;
		

	}


	if ( ( nodeVar31 > nodeVar3 ) ) {

		nodeVar3 = nodeVar31;
		

	}

	nodeVar32 = StructType0( nodeVar1, nodeVar2, nodeVar3 );
	nodeVar33 = textureLoad( nodeUniform0, vec2<i32>( nodeVar32.closestPositionTexel ), u32( 0u ) );
	nodeVar34 = ( nodeVar33.xy * vec2<f32>( 0.5, -0.5 ) );
	nodeVar35 = subpixelCorrection( nodeVar34, vec2<i32>( textureDimensions( nodeUniform1, 0 ) ) );
	nodeVar0 = ( nodeVar0 + ( nodeVar35 * 0.25 ) );
	nodeVar37 = ( nodeVarying0 - nodeVar34 );
	nodeVar39 = textureDimensions( nodeUniform7, u32( 0 ) );
	nodeVar38 = textureLoad( nodeUniform7, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( object.nodeUniform8 * vec3<f32>( nodeVar37, 1.0 ) ).xy ) * vec2<f32>( nodeVar39 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar39 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );

	if ( ( ( all( ( nodeVar37 >= vec2<f32>( 0.0 ) ) ) && all( ( nodeVar37 <= vec2<f32>( 1.0 ) ) ) ) && ( ( ( nodeVar32.farthestDepth - nodeVar32.closestDepth ) > 0.001 ) || ( ! ( ( nodeVar32.closestDepth - ( ( ( object.nodeUniform3.x + ( object.nodeUniform4 * vec4<f32>( ( object.nodeUniform5 * vec4<f32>( ( ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar37.x, ( 1.0 - nodeVar37.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar38 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar37.x, ( 1.0 - nodeVar37.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar38 ), 1.0 ) ).w ) ), 1.0 ) ).xyz, 1.0 ) ).z ) * object.nodeUniform3.y ) / ( ( object.nodeUniform3.y - object.nodeUniform3.x ) * ( object.nodeUniform4 * vec4<f32>( ( object.nodeUniform5 * vec4<f32>( ( ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar37.x, ( 1.0 - nodeVar37.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar38 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar37.x, ( 1.0 - nodeVar37.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar38 ), 1.0 ) ).w ) ), 1.0 ) ).xyz, 1.0 ) ).z ) ) ) > 0.0005 ) ) ) ) ) {

		nodeVar36 = clamp( ( nodeVar0 + clamp( ( length( ( ( nodeVarying0 - nodeVar37 ) * vec2<f32>( textureDimensions( nodeUniform1, 0 ) ) ) ) / 128.0 ), 0.0, 1.0 ) ), 0.0, 1.0 );

	} else {

		nodeVar36 = 1.0;

	}

	nodeVar0 = nodeVar36;
	nodeVar40 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	nodeVar41 = nodeVar40;
	nodeVar42 = ( nodeVar40 * nodeVar40 );
	nodeVar43 = textureLoad( nodeUniform1, vec2<i32>( nodeVar4 ) + vec2<i32>( -1, -1 ), u32( 0u ) );
	nodeVar44 = max( nodeVar43, vec4<f32>( 0.0 ) );
	nodeVar41 = ( nodeVar41 + nodeVar44 );
	nodeVar42 = ( nodeVar42 + ( nodeVar44 * nodeVar44 ) );
	nodeVar45 = textureLoad( nodeUniform1, vec2<i32>( nodeVar4 ) + vec2<i32>( -1, 1 ), u32( 0u ) );
	nodeVar46 = max( nodeVar45, vec4<f32>( 0.0 ) );
	nodeVar41 = ( nodeVar41 + nodeVar46 );
	nodeVar42 = ( nodeVar42 + ( nodeVar46 * nodeVar46 ) );
	nodeVar47 = textureLoad( nodeUniform1, vec2<i32>( nodeVar4 ) + vec2<i32>( 1, -1 ), u32( 0u ) );
	nodeVar48 = max( nodeVar47, vec4<f32>( 0.0 ) );
	nodeVar41 = ( nodeVar41 + nodeVar48 );
	nodeVar42 = ( nodeVar42 + ( nodeVar48 * nodeVar48 ) );
	nodeVar49 = textureLoad( nodeUniform1, vec2<i32>( nodeVar4 ) + vec2<i32>( 1, 1 ), u32( 0u ) );
	nodeVar50 = max( nodeVar49, vec4<f32>( 0.0 ) );
	nodeVar41 = ( nodeVar41 + nodeVar50 );
	nodeVar42 = ( nodeVar42 + ( nodeVar50 * nodeVar50 ) );
	nodeVar51 = textureLoad( nodeUniform1, vec2<i32>( nodeVar4 ) + vec2<i32>( 1, 0 ), u32( 0u ) );
	nodeVar52 = max( nodeVar51, vec4<f32>( 0.0 ) );
	nodeVar41 = ( nodeVar41 + nodeVar52 );
	nodeVar42 = ( nodeVar42 + ( nodeVar52 * nodeVar52 ) );
	nodeVar53 = textureLoad( nodeUniform1, vec2<i32>( nodeVar4 ) + vec2<i32>( 0, -1 ), u32( 0u ) );
	nodeVar54 = max( nodeVar53, vec4<f32>( 0.0 ) );
	nodeVar41 = ( nodeVar41 + nodeVar54 );
	nodeVar42 = ( nodeVar42 + ( nodeVar54 * nodeVar54 ) );
	nodeVar55 = textureLoad( nodeUniform1, vec2<i32>( nodeVar4 ) + vec2<i32>( 0, 1 ), u32( 0u ) );
	nodeVar56 = max( nodeVar55, vec4<f32>( 0.0 ) );
	nodeVar41 = ( nodeVar41 + nodeVar56 );
	nodeVar42 = ( nodeVar42 + ( nodeVar56 * nodeVar56 ) );
	nodeVar57 = textureLoad( nodeUniform1, vec2<i32>( nodeVar4 ) + vec2<i32>( -1, 0 ), u32( 0u ) );
	nodeVar58 = max( nodeVar57, vec4<f32>( 0.0 ) );
	nodeVar41 = ( nodeVar41 + nodeVar58 );
	nodeVar42 = ( nodeVar42 + ( nodeVar58 * nodeVar58 ) );
	nodeVar59 = ( nodeVar41 / vec4<f32>( 9.0 ) );
	nodeVar60 = ( 1.0 - clamp( ( length( ( ( nodeVarying0 - nodeVar37 ) * vec2<f32>( textureDimensions( nodeUniform1, 0 ) ) ) ) / 128.0 ), 0.0, 1.0 ) );
	nodeVar61 = ( sqrt( max( ( ( nodeVar42 / vec4<f32>( 9.0 ) ) - ( nodeVar59 * nodeVar59 ) ), vec4<f32>( 0.0 ) ) ) * vec4<f32>( mix( 0.5, 1.0, ( nodeVar60 * nodeVar60 ) ) ) );
	nodeVar62 = ( nodeVar59 - nodeVar61 );
	nodeVar63 = ( nodeVar59 + nodeVar61 );
	nodeVar64 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform10 * vec3<f32>( nodeVar37, 1.0 ) ).xy );
	DiffuseColor = flickerReduction( nodeVar40, clipAABB( clamp( nodeVar59, nodeVar62, nodeVar63 ), nodeVar64, nodeVar62, nodeVar63 ), nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform11 );
	DiffuseColor.w = 1.0;
	nodeVar65 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar65;

	// result

	output.color = nodeVar65;

	return output;

}
