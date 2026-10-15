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
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec2<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : StructType0;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<u32>;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : vec4<f32>;
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
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec4<f32>;
var<private> nodeVar60 : vec4<f32>;
var<private> nodeVar61 : vec4<f32>;
var<private> nodeVar62 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar63 : vec4<f32>;

// codes
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
	nodeVar2 = 2.0;
	nodeVar3 = vec2<f32>( 0.0, 0.0 );
	nodeVar4 = -1.0;
	nodeVar5 = ( nodeVarying0 * vec2<f32>( textureDimensions( nodeUniform1, 0 ) ) );
	nodeVar6 = ( nodeVar5 + vec2<f32>( -1.0, -1.0 ) );
	nodeVar7 = textureLoad( nodeUniform2, vec2<i32>( nodeVar6 ), u32( 0u ) );
	nodeVar8 = nodeVar7;

	if ( ( nodeVar8 < nodeVar2 ) ) {

		nodeVar2 = nodeVar8;
		nodeVar3 = nodeVar6;
		

	}


	if ( ( nodeVar8 > nodeVar4 ) ) {

		nodeVar4 = nodeVar8;
		

	}

	nodeVar9 = ( nodeVar5 + vec2<f32>( -1.0, 0.0 ) );
	nodeVar10 = textureLoad( nodeUniform2, vec2<i32>( nodeVar9 ), u32( 0u ) );
	nodeVar11 = nodeVar10;

	if ( ( nodeVar11 < nodeVar2 ) ) {

		nodeVar2 = nodeVar11;
		nodeVar3 = nodeVar9;
		

	}


	if ( ( nodeVar11 > nodeVar4 ) ) {

		nodeVar4 = nodeVar11;
		

	}

	nodeVar12 = ( nodeVar5 + vec2<f32>( -1.0, 1.0 ) );
	nodeVar13 = textureLoad( nodeUniform2, vec2<i32>( nodeVar12 ), u32( 0u ) );
	nodeVar14 = nodeVar13;

	if ( ( nodeVar14 < nodeVar2 ) ) {

		nodeVar2 = nodeVar14;
		nodeVar3 = nodeVar12;
		

	}


	if ( ( nodeVar14 > nodeVar4 ) ) {

		nodeVar4 = nodeVar14;
		

	}

	nodeVar15 = ( nodeVar5 + vec2<f32>( 0.0, -1.0 ) );
	nodeVar16 = textureLoad( nodeUniform2, vec2<i32>( nodeVar15 ), u32( 0u ) );
	nodeVar17 = nodeVar16;

	if ( ( nodeVar17 < nodeVar2 ) ) {

		nodeVar2 = nodeVar17;
		nodeVar3 = nodeVar15;
		

	}


	if ( ( nodeVar17 > nodeVar4 ) ) {

		nodeVar4 = nodeVar17;
		

	}

	nodeVar18 = ( nodeVar5 + vec2<f32>( 0.0, 0.0 ) );
	nodeVar19 = textureLoad( nodeUniform2, vec2<i32>( nodeVar18 ), u32( 0u ) );
	nodeVar20 = nodeVar19;

	if ( ( nodeVar20 < nodeVar2 ) ) {

		nodeVar2 = nodeVar20;
		nodeVar3 = nodeVar18;
		

	}


	if ( ( nodeVar20 > nodeVar4 ) ) {

		nodeVar4 = nodeVar20;
		

	}

	nodeVar21 = ( nodeVar5 + vec2<f32>( 0.0, 1.0 ) );
	nodeVar22 = textureLoad( nodeUniform2, vec2<i32>( nodeVar21 ), u32( 0u ) );
	nodeVar23 = nodeVar22;

	if ( ( nodeVar23 < nodeVar2 ) ) {

		nodeVar2 = nodeVar23;
		nodeVar3 = nodeVar21;
		

	}


	if ( ( nodeVar23 > nodeVar4 ) ) {

		nodeVar4 = nodeVar23;
		

	}

	nodeVar24 = ( nodeVar5 + vec2<f32>( 1.0, -1.0 ) );
	nodeVar25 = textureLoad( nodeUniform2, vec2<i32>( nodeVar24 ), u32( 0u ) );
	nodeVar26 = nodeVar25;

	if ( ( nodeVar26 < nodeVar2 ) ) {

		nodeVar2 = nodeVar26;
		nodeVar3 = nodeVar24;
		

	}


	if ( ( nodeVar26 > nodeVar4 ) ) {

		nodeVar4 = nodeVar26;
		

	}

	nodeVar27 = ( nodeVar5 + vec2<f32>( 1.0, 0.0 ) );
	nodeVar28 = textureLoad( nodeUniform2, vec2<i32>( nodeVar27 ), u32( 0u ) );
	nodeVar29 = nodeVar28;

	if ( ( nodeVar29 < nodeVar2 ) ) {

		nodeVar2 = nodeVar29;
		nodeVar3 = nodeVar27;
		

	}


	if ( ( nodeVar29 > nodeVar4 ) ) {

		nodeVar4 = nodeVar29;
		

	}

	nodeVar30 = ( nodeVar5 + vec2<f32>( 1.0, 1.0 ) );
	nodeVar31 = textureLoad( nodeUniform2, vec2<i32>( nodeVar30 ), u32( 0u ) );
	nodeVar32 = nodeVar31;

	if ( ( nodeVar32 < nodeVar2 ) ) {

		nodeVar2 = nodeVar32;
		nodeVar3 = nodeVar30;
		

	}


	if ( ( nodeVar32 > nodeVar4 ) ) {

		nodeVar4 = nodeVar32;
		

	}

	nodeVar33 = StructType0( nodeVar2, nodeVar3, nodeVar4 );
	nodeVar34 = textureLoad( nodeUniform0, vec2<i32>( nodeVar33.closestPositionTexel ), u32( 0u ) );
	nodeVar35 = ( nodeVarying0 - ( nodeVar34.xy * vec2<f32>( 0.5, -0.5 ) ) );
	nodeVar37 = textureDimensions( nodeUniform7, u32( 0 ) );
	nodeVar36 = textureLoad( nodeUniform7, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( object.nodeUniform8 * vec3<f32>( nodeVar35, 1.0 ) ).xy ) * vec2<f32>( nodeVar37 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar37 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );

	if ( ( ( all( ( nodeVar35 >= vec2<f32>( 0.0 ) ) ) && all( ( nodeVar35 <= vec2<f32>( 1.0 ) ) ) ) && ( ( ( nodeVar33.farthestDepth - nodeVar33.closestDepth ) > 0.001 ) || ( ! ( ( nodeVar33.closestDepth - ( ( ( object.nodeUniform3.x + ( object.nodeUniform4 * vec4<f32>( ( object.nodeUniform5 * vec4<f32>( ( ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar35.x, ( 1.0 - nodeVar35.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar35.x, ( 1.0 - nodeVar35.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) ).w ) ), 1.0 ) ).xyz, 1.0 ) ).z ) * object.nodeUniform3.y ) / ( ( object.nodeUniform3.y - object.nodeUniform3.x ) * ( object.nodeUniform4 * vec4<f32>( ( object.nodeUniform5 * vec4<f32>( ( ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar35.x, ( 1.0 - nodeVar35.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar35.x, ( 1.0 - nodeVar35.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) ).w ) ), 1.0 ) ).xyz, 1.0 ) ).z ) ) ) > 0.0005 ) ) ) ) ) {

		nodeVar1 = clamp( ( nodeVar0 + clamp( ( length( ( ( nodeVarying0 - nodeVar35 ) * vec2<f32>( textureDimensions( nodeUniform1, 0 ) ) ) ) / 128.0 ), 0.0, 1.0 ) ), 0.0, 1.0 );

	} else {

		nodeVar1 = 1.0;

	}

	nodeVar0 = nodeVar1;
	nodeVar38 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	nodeVar39 = nodeVar38;
	nodeVar40 = ( nodeVar38 * nodeVar38 );
	nodeVar41 = textureLoad( nodeUniform1, vec2<i32>( nodeVar5 ) + vec2<i32>( -1, -1 ), u32( 0u ) );
	nodeVar42 = max( nodeVar41, vec4<f32>( 0.0 ) );
	nodeVar39 = ( nodeVar39 + nodeVar42 );
	nodeVar40 = ( nodeVar40 + ( nodeVar42 * nodeVar42 ) );
	nodeVar43 = textureLoad( nodeUniform1, vec2<i32>( nodeVar5 ) + vec2<i32>( -1, 1 ), u32( 0u ) );
	nodeVar44 = max( nodeVar43, vec4<f32>( 0.0 ) );
	nodeVar39 = ( nodeVar39 + nodeVar44 );
	nodeVar40 = ( nodeVar40 + ( nodeVar44 * nodeVar44 ) );
	nodeVar45 = textureLoad( nodeUniform1, vec2<i32>( nodeVar5 ) + vec2<i32>( 1, -1 ), u32( 0u ) );
	nodeVar46 = max( nodeVar45, vec4<f32>( 0.0 ) );
	nodeVar39 = ( nodeVar39 + nodeVar46 );
	nodeVar40 = ( nodeVar40 + ( nodeVar46 * nodeVar46 ) );
	nodeVar47 = textureLoad( nodeUniform1, vec2<i32>( nodeVar5 ) + vec2<i32>( 1, 1 ), u32( 0u ) );
	nodeVar48 = max( nodeVar47, vec4<f32>( 0.0 ) );
	nodeVar39 = ( nodeVar39 + nodeVar48 );
	nodeVar40 = ( nodeVar40 + ( nodeVar48 * nodeVar48 ) );
	nodeVar49 = textureLoad( nodeUniform1, vec2<i32>( nodeVar5 ) + vec2<i32>( 1, 0 ), u32( 0u ) );
	nodeVar50 = max( nodeVar49, vec4<f32>( 0.0 ) );
	nodeVar39 = ( nodeVar39 + nodeVar50 );
	nodeVar40 = ( nodeVar40 + ( nodeVar50 * nodeVar50 ) );
	nodeVar51 = textureLoad( nodeUniform1, vec2<i32>( nodeVar5 ) + vec2<i32>( 0, -1 ), u32( 0u ) );
	nodeVar52 = max( nodeVar51, vec4<f32>( 0.0 ) );
	nodeVar39 = ( nodeVar39 + nodeVar52 );
	nodeVar40 = ( nodeVar40 + ( nodeVar52 * nodeVar52 ) );
	nodeVar53 = textureLoad( nodeUniform1, vec2<i32>( nodeVar5 ) + vec2<i32>( 0, 1 ), u32( 0u ) );
	nodeVar54 = max( nodeVar53, vec4<f32>( 0.0 ) );
	nodeVar39 = ( nodeVar39 + nodeVar54 );
	nodeVar40 = ( nodeVar40 + ( nodeVar54 * nodeVar54 ) );
	nodeVar55 = textureLoad( nodeUniform1, vec2<i32>( nodeVar5 ) + vec2<i32>( -1, 0 ), u32( 0u ) );
	nodeVar56 = max( nodeVar55, vec4<f32>( 0.0 ) );
	nodeVar39 = ( nodeVar39 + nodeVar56 );
	nodeVar40 = ( nodeVar40 + ( nodeVar56 * nodeVar56 ) );
	nodeVar57 = ( nodeVar39 / vec4<f32>( 9.0 ) );
	nodeVar58 = ( 1.0 - clamp( ( length( ( ( nodeVarying0 - nodeVar35 ) * vec2<f32>( textureDimensions( nodeUniform1, 0 ) ) ) ) / 128.0 ), 0.0, 1.0 ) );
	nodeVar59 = ( sqrt( max( ( ( nodeVar40 / vec4<f32>( 9.0 ) ) - ( nodeVar57 * nodeVar57 ) ), vec4<f32>( 0.0 ) ) ) * vec4<f32>( mix( 0.5, 1.0, ( nodeVar58 * nodeVar58 ) ) ) );
	nodeVar60 = ( nodeVar57 - nodeVar59 );
	nodeVar61 = ( nodeVar57 + nodeVar59 );
	nodeVar62 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform10 * vec3<f32>( nodeVar35, 1.0 ) ).xy );
	DiffuseColor = flickerReduction( nodeVar38, clipAABB( clamp( nodeVar57, nodeVar60, nodeVar61 ), nodeVar62, nodeVar60, nodeVar61 ), nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform11 );
	DiffuseColor.w = 1.0;
	nodeVar63 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar63;

	// result

	output.color = nodeVar63;

	return output;

}
