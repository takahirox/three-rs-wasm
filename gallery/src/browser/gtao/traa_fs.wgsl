// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


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
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<u32>;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : vec4<f32>;
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
var<private> Output : vec4<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn130 ( currentColor : vec4<f32>, historyColor : vec4<f32>, minColor : vec4<f32>, maxColor : vec4<f32> ) -> vec4<f32> {

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




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = 0.05;
	nodeVar2 = 2.0;
	nodeVar3 = vec2<f32>( 0.0, 0.0 );
	nodeVar4 = -1.0;
	let nodeConst0 = ( nodeVarying0 * vec2<f32>( textureDimensions( nodeUniform1, 0 ) ) );
	nodeVar5 = ( nodeConst0 + vec2<f32>( -1.0, -1.0 ) );
	nodeVar6 = textureLoad( nodeUniform2, vec2<i32>( nodeVar5 ), u32( 0u ) );
	nodeVar7 = nodeVar6;

	if ( ( nodeVar7 < nodeVar2 ) ) {

		nodeVar2 = nodeVar7;
		nodeVar3 = nodeVar5;
		

	}


	if ( ( nodeVar7 > nodeVar4 ) ) {

		nodeVar4 = nodeVar7;
		

	}

	nodeVar8 = ( nodeConst0 + vec2<f32>( -1.0, 0.0 ) );
	nodeVar9 = textureLoad( nodeUniform2, vec2<i32>( nodeVar8 ), u32( 0u ) );
	nodeVar10 = nodeVar9;

	if ( ( nodeVar10 < nodeVar2 ) ) {

		nodeVar2 = nodeVar10;
		nodeVar3 = nodeVar8;
		

	}


	if ( ( nodeVar10 > nodeVar4 ) ) {

		nodeVar4 = nodeVar10;
		

	}

	nodeVar11 = ( nodeConst0 + vec2<f32>( -1.0, 1.0 ) );
	nodeVar12 = textureLoad( nodeUniform2, vec2<i32>( nodeVar11 ), u32( 0u ) );
	nodeVar13 = nodeVar12;

	if ( ( nodeVar13 < nodeVar2 ) ) {

		nodeVar2 = nodeVar13;
		nodeVar3 = nodeVar11;
		

	}


	if ( ( nodeVar13 > nodeVar4 ) ) {

		nodeVar4 = nodeVar13;
		

	}

	nodeVar14 = ( nodeConst0 + vec2<f32>( 0.0, -1.0 ) );
	nodeVar15 = textureLoad( nodeUniform2, vec2<i32>( nodeVar14 ), u32( 0u ) );
	nodeVar16 = nodeVar15;

	if ( ( nodeVar16 < nodeVar2 ) ) {

		nodeVar2 = nodeVar16;
		nodeVar3 = nodeVar14;
		

	}


	if ( ( nodeVar16 > nodeVar4 ) ) {

		nodeVar4 = nodeVar16;
		

	}

	nodeVar17 = ( nodeConst0 + vec2<f32>( 0.0, 0.0 ) );
	nodeVar18 = textureLoad( nodeUniform2, vec2<i32>( nodeVar17 ), u32( 0u ) );
	nodeVar19 = nodeVar18;

	if ( ( nodeVar19 < nodeVar2 ) ) {

		nodeVar2 = nodeVar19;
		nodeVar3 = nodeVar17;
		

	}


	if ( ( nodeVar19 > nodeVar4 ) ) {

		nodeVar4 = nodeVar19;
		

	}

	nodeVar20 = ( nodeConst0 + vec2<f32>( 0.0, 1.0 ) );
	nodeVar21 = textureLoad( nodeUniform2, vec2<i32>( nodeVar20 ), u32( 0u ) );
	nodeVar22 = nodeVar21;

	if ( ( nodeVar22 < nodeVar2 ) ) {

		nodeVar2 = nodeVar22;
		nodeVar3 = nodeVar20;
		

	}


	if ( ( nodeVar22 > nodeVar4 ) ) {

		nodeVar4 = nodeVar22;
		

	}

	nodeVar23 = ( nodeConst0 + vec2<f32>( 1.0, -1.0 ) );
	nodeVar24 = textureLoad( nodeUniform2, vec2<i32>( nodeVar23 ), u32( 0u ) );
	nodeVar25 = nodeVar24;

	if ( ( nodeVar25 < nodeVar2 ) ) {

		nodeVar2 = nodeVar25;
		nodeVar3 = nodeVar23;
		

	}


	if ( ( nodeVar25 > nodeVar4 ) ) {

		nodeVar4 = nodeVar25;
		

	}

	nodeVar26 = ( nodeConst0 + vec2<f32>( 1.0, 0.0 ) );
	nodeVar27 = textureLoad( nodeUniform2, vec2<i32>( nodeVar26 ), u32( 0u ) );
	nodeVar28 = nodeVar27;

	if ( ( nodeVar28 < nodeVar2 ) ) {

		nodeVar2 = nodeVar28;
		nodeVar3 = nodeVar26;
		

	}


	if ( ( nodeVar28 > nodeVar4 ) ) {

		nodeVar4 = nodeVar28;
		

	}

	nodeVar29 = ( nodeConst0 + vec2<f32>( 1.0, 1.0 ) );
	nodeVar30 = textureLoad( nodeUniform2, vec2<i32>( nodeVar29 ), u32( 0u ) );
	nodeVar31 = nodeVar30;

	if ( ( nodeVar31 < nodeVar2 ) ) {

		nodeVar2 = nodeVar31;
		nodeVar3 = nodeVar29;
		

	}


	if ( ( nodeVar31 > nodeVar4 ) ) {

		nodeVar4 = nodeVar31;
		

	}

	nodeVar32 = StructType0( nodeVar2, nodeVar3, nodeVar4 );
	nodeVar33 = textureLoad( nodeUniform0, vec2<i32>( nodeVar32.closestPositionTexel ), u32( 0u ) );
	let nodeConst1 = ( nodeVarying0 - ( nodeVar33.xy * vec2<f32>( 0.5, -0.5 ) ) );
	nodeVar35 = textureDimensions( nodeUniform7, u32( 0 ) );
	nodeVar34 = textureLoad( nodeUniform7, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( object.nodeUniform8 * vec3<f32>( nodeConst1, 1.0 ) ).xy ) * vec2<f32>( nodeVar35 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar35 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst2 = ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst1.x, ( 1.0 - nodeConst1.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar34 ), 1.0 ) );

	if ( ( ( all( ( nodeConst1 >= vec2<f32>( 0.0 ) ) ) && all( ( nodeConst1 <= vec2<f32>( 1.0 ) ) ) ) && ( ( ( nodeVar32.farthestDepth - nodeVar32.closestDepth ) > 0.001 ) || ( ! ( ( nodeVar32.closestDepth - ( ( ( object.nodeUniform3.x + ( object.nodeUniform4 * vec4<f32>( ( object.nodeUniform5 * vec4<f32>( ( nodeConst2.xyz / vec3<f32>( nodeConst2.w ) ), 1.0 ) ).xyz, 1.0 ) ).z ) * object.nodeUniform3.y ) / ( ( object.nodeUniform3.y - object.nodeUniform3.x ) * ( object.nodeUniform4 * vec4<f32>( ( object.nodeUniform5 * vec4<f32>( ( nodeConst2.xyz / vec3<f32>( nodeConst2.w ) ), 1.0 ) ).xyz, 1.0 ) ).z ) ) ) > 0.0005 ) ) ) ) ) {

		let nodeConst3 = clamp( ( length( ( ( nodeVarying0 - nodeConst1 ) * vec2<f32>( textureDimensions( nodeUniform1, 0 ) ) ) ) / 128.0 ), 0.0, 1.0 );
		nodeVar1 = clamp( ( nodeVar0 + nodeConst3 ), 0.0, 1.0 );

	} else {

		nodeVar1 = 1.0;

	}

	nodeVar0 = nodeVar1;
	nodeVar36 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	let nodeConst4 = nodeVar36;
	let nodeConst5 = ( nodeConst4 * vec4<f32>( ( 1.0 / ( max( max( nodeConst4.x, nodeConst4.y ), nodeConst4.z ) + 1.0 ) ) ) );
	nodeVar37 = nodeConst4;
	nodeVar38 = ( nodeConst4 * nodeConst4 );
	nodeVar39 = textureLoad( nodeUniform1, vec2<i32>( nodeConst0 ) + vec2<i32>( -1, -1 ), u32( 0u ) );
	let nodeConst6 = max( nodeVar39, vec4<f32>( 0.0 ) );
	nodeVar37 = ( nodeVar37 + nodeConst6 );
	nodeVar38 = ( nodeVar38 + ( nodeConst6 * nodeConst6 ) );
	nodeVar40 = textureLoad( nodeUniform1, vec2<i32>( nodeConst0 ) + vec2<i32>( -1, 1 ), u32( 0u ) );
	let nodeConst7 = max( nodeVar40, vec4<f32>( 0.0 ) );
	nodeVar37 = ( nodeVar37 + nodeConst7 );
	nodeVar38 = ( nodeVar38 + ( nodeConst7 * nodeConst7 ) );
	nodeVar41 = textureLoad( nodeUniform1, vec2<i32>( nodeConst0 ) + vec2<i32>( 1, -1 ), u32( 0u ) );
	let nodeConst8 = max( nodeVar41, vec4<f32>( 0.0 ) );
	nodeVar37 = ( nodeVar37 + nodeConst8 );
	nodeVar38 = ( nodeVar38 + ( nodeConst8 * nodeConst8 ) );
	nodeVar42 = textureLoad( nodeUniform1, vec2<i32>( nodeConst0 ) + vec2<i32>( 1, 1 ), u32( 0u ) );
	let nodeConst9 = max( nodeVar42, vec4<f32>( 0.0 ) );
	nodeVar37 = ( nodeVar37 + nodeConst9 );
	nodeVar38 = ( nodeVar38 + ( nodeConst9 * nodeConst9 ) );
	nodeVar43 = textureLoad( nodeUniform1, vec2<i32>( nodeConst0 ) + vec2<i32>( 1, 0 ), u32( 0u ) );
	let nodeConst10 = max( nodeVar43, vec4<f32>( 0.0 ) );
	nodeVar37 = ( nodeVar37 + nodeConst10 );
	nodeVar38 = ( nodeVar38 + ( nodeConst10 * nodeConst10 ) );
	nodeVar44 = textureLoad( nodeUniform1, vec2<i32>( nodeConst0 ) + vec2<i32>( 0, -1 ), u32( 0u ) );
	let nodeConst11 = max( nodeVar44, vec4<f32>( 0.0 ) );
	nodeVar37 = ( nodeVar37 + nodeConst11 );
	nodeVar38 = ( nodeVar38 + ( nodeConst11 * nodeConst11 ) );
	nodeVar45 = textureLoad( nodeUniform1, vec2<i32>( nodeConst0 ) + vec2<i32>( 0, 1 ), u32( 0u ) );
	let nodeConst12 = max( nodeVar45, vec4<f32>( 0.0 ) );
	nodeVar37 = ( nodeVar37 + nodeConst12 );
	nodeVar38 = ( nodeVar38 + ( nodeConst12 * nodeConst12 ) );
	nodeVar46 = textureLoad( nodeUniform1, vec2<i32>( nodeConst0 ) + vec2<i32>( -1, 0 ), u32( 0u ) );
	let nodeConst13 = max( nodeVar46, vec4<f32>( 0.0 ) );
	nodeVar37 = ( nodeVar37 + nodeConst13 );
	nodeVar38 = ( nodeVar38 + ( nodeConst13 * nodeConst13 ) );
	let nodeConst14 = ( nodeVar37 / vec4<f32>( 9.0 ) );
	let nodeConst3 = clamp( ( length( ( ( nodeVarying0 - nodeConst1 ) * vec2<f32>( textureDimensions( nodeUniform1, 0 ) ) ) ) / 128.0 ), 0.0, 1.0 );
	let nodeConst15 = ( 1.0 - nodeConst3 );
	let nodeConst16 = ( sqrt( max( ( ( nodeVar38 / vec4<f32>( 9.0 ) ) - ( nodeConst14 * nodeConst14 ) ), vec4<f32>( 0.0 ) ) ) * vec4<f32>( mix( 0.5, 1.0, ( nodeConst15 * nodeConst15 ) ) ) );
	let nodeConst17 = ( nodeConst14 - nodeConst16 );
	let nodeConst18 = ( nodeConst14 + nodeConst16 );
	nodeVar47 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform10 * vec3<f32>( nodeConst1, 1.0 ) ).xy );
	let nodeConst19 = fn130( clamp( nodeConst14, nodeConst17, nodeConst18 ), nodeVar47, nodeConst17, nodeConst18 );
	let nodeConst20 = ( nodeConst19 * vec4<f32>( ( 1.0 / ( max( max( nodeConst19.x, nodeConst19.y ), nodeConst19.z ) + 1.0 ) ) ) );
	let nodeConst21 = dot( nodeConst5.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
	let nodeConst22 = dot( nodeConst20.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
	let nodeConst23 = ( nodeVar0 / ( nodeConst21 + 1.0 ) );
	let nodeConst24 = ( ( 1.0 - nodeVar0 ) / ( nodeConst22 + 1.0 ) );
	DiffuseColor = ( ( ( nodeConst4 * vec4<f32>( nodeConst23 ) ) + ( nodeConst19 * vec4<f32>( nodeConst24 ) ) ) / vec4<f32>( max( ( nodeConst23 + nodeConst24 ), 0.00001 ) ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform11 );
	DiffuseColor.w = 1.0;
	let nodeConst25 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst25;

	// result

	output.color = nodeConst25;

	return output;

}
