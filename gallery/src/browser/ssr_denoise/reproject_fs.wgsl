// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct StructType0 {
	color : vec4<f32>,
	weight : f32,
	confidence : f32
};

struct StructType1 {
	color : vec4<f32>,
	tapConfidence : f32,
	minConfidence : f32
};

struct StructType2 {
	mean : vec3<f32>,
	stdColor : vec3<f32>,
	rayLength : f32,
	envProbability : f32,
	stdDevRayLength : f32
};

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_depth_2d;
@binding( 1 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 0 ) var nodeUniform1 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;
@binding( 6 ) @group( 0 ) var nodeUniform8_sampler : sampler;
@binding( 7 ) @group( 0 ) var nodeUniform8 : texture_2d<f32>;
@binding( 8 ) @group( 0 ) var nodeUniform11 : texture_depth_2d;
@binding( 9 ) @group( 0 ) var nodeUniform13_sampler : sampler;
@binding( 10 ) @group( 0 ) var nodeUniform13 : texture_2d<f32>;
@binding( 11 ) @group( 0 ) var nodeUniform15_sampler : sampler;
@binding( 12 ) @group( 0 ) var nodeUniform15 : texture_2d<f32>;

struct objectStruct {
	nodeUniform2 : vec2<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : mat4x4<f32>,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform14 : mat3x3<f32>,
	nodeUniform16 : mat3x3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform20 : u32,
	nodeUniform21 : f32,
	nodeUniform22 : f32
};
@binding( 3 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec4<f32>;
var<private> nodeVar32 : vec4<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : StructType2;
var<private> nodeVar38 : f32;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : StructType1;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : StructType0;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : vec4<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : StructType0;
var<private> nodeVar57 : vec4<f32>;
var<private> nodeVar58 : vec4<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : StructType0;
var<private> nodeVar62 : vec4<f32>;
var<private> nodeVar63 : vec4<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : StructType0;
var<private> nodeVar67 : vec4<f32>;
var<private> nodeVar68 : vec4<f32>;
var<private> nodeVar69 : vec4<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec2<f32>;
var<private> nodeVar73 : vec2<f32>;
var<private> nodeVar74 : StructType1;
var<private> nodeVar75 : vec4<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : StructType0;
var<private> nodeVar79 : vec4<f32>;
var<private> nodeVar80 : vec4<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : StructType0;
var<private> nodeVar84 : vec4<f32>;
var<private> nodeVar85 : vec4<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : StructType0;
var<private> nodeVar89 : vec4<f32>;
var<private> nodeVar90 : vec4<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : StructType0;
var<private> nodeVar94 : vec4<f32>;
var<private> nodeVar95 : vec4<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : bool;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec4<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;

// codes
fn fn140 ( screenTexel : vec2<i32>, beautySize : vec2<f32>, resolveSize : vec2<f32> ) -> vec2<i32> {

	


	return vec2<i32>( floor( ( ( vec2<f32>( screenTexel ) * beautySize ) / resolveSize ) ) );

}


fn fn145 ( velocity : vec2<f32> ) -> vec2<f32> {

	


	return ( velocity * vec2<f32>( 0.5, -0.5 ) );

}


fn fn141 ( worldPos : vec3<f32>, previousViewMatrix : mat4x4<f32>, previousProjectionMatrix : mat4x4<f32> ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;
	var nodeVar1 : vec4<f32>;
	var nodeVar2 : f32;

	nodeVar0 = vec2<f32>( -1.0, -1.0 );
	nodeVar1 = ( previousProjectionMatrix * ( previousViewMatrix * vec4<f32>( worldPos, 1.0 ) ) );
	nodeVar2 = nodeVar1.w;

	if ( ( abs( nodeVar2 ) > 0.00001 ) ) {

		nodeVar0 = ( ( ( nodeVar1.xyz / vec3<f32>( nodeVar2 ) ).xy * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) );
		nodeVar0.y = ( 1.0 - nodeVar0.y );
		

	}


	return nodeVar0;

}


fn fn144 ( rayOrig : vec3<f32>, rayLength : f32, cameraWorldPosition : vec3<f32>, previousViewMatrix : mat4x4<f32>, previousProjectionMatrix : mat4x4<f32> ) -> vec2<f32> {

	var nodeVar0 : vec3<f32>;

	nodeVar0 = normalize( ( rayOrig - cameraWorldPosition ) );

	return fn141( ( rayOrig + ( nodeVar0 * vec3<f32>( rayLength ) ) ), previousViewMatrix, previousProjectionMatrix );

}


fn fn143 ( historyUV : vec2<f32>, resolution : vec2<f32> ) -> f32 {

	var nodeVar0 : vec2<f32>;
	var nodeVar1 : vec2<f32>;

	nodeVar0 = ( dpdx( historyUV ) * resolution );
	nodeVar1 = ( - dpdy( historyUV ) * resolution );
	let nodeConst0 = ( dot( nodeVar0, nodeVar0 ) + dot( nodeVar1, nodeVar1 ) );
	let nodeConst1 = ( ( nodeVar0.x * nodeVar1.y ) - ( nodeVar0.y * nodeVar1.x ) );

	return clamp( sqrt( max( ( ( nodeConst0 * 0.5 ) - sqrt( max( ( ( ( nodeConst0 * nodeConst0 ) * 0.25 ) - ( nodeConst1 * nodeConst1 ) ), 0.0 ) ) ), 0.0 ) ), 0.0, 1.0 );

}


fn fn142 ( history : vec3<f32>, boxMin : vec3<f32>, boxMax : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;

	let nodeConst0 = ( ( boxMax + boxMin ) * vec3<f32>( 0.5 ) );
	let nodeConst1 = ( history - nodeConst0 );
	let nodeConst2 = abs( ( nodeConst1 / ( ( ( boxMax - boxMin ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 1e-7 ) ) ) );
	let nodeConst3 = max( max( nodeConst2.x, nodeConst2.y ), nodeConst2.z );

	if ( ( nodeConst3 > 1.0 ) ) {

		nodeVar0 = ( nodeConst0 + ( nodeConst1 / vec3<f32>( nodeConst3 ) ) );

	} else {

		nodeVar0 = history;

	}


	return nodeVar0;

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = vec2<i32>( floor( ( fragCoord.xy - vec2<f32>( 0.5 ) ) ) );
	nodeVar0 = textureLoad( nodeUniform0, nodeConst0, u32( 0u ) );
	nodeVar1 = nodeVar0;

	if ( ( nodeVar1 >= 1.0 ) ) {

		discard;
		

	}

	let nodeConst1 = fn140( nodeConst0, vec2<f32>( textureDimensions( nodeUniform1, 0 ) ), object.nodeUniform2 );
	nodeVar2 = textureLoad( nodeUniform1, nodeConst1, u32( 0u ) );
	nodeVar3 = max( nodeVar2, vec4<f32>( 0.0 ) );
	nodeVar4 = textureLoad( nodeUniform3, nodeConst0, u32( 0u ) );
	nodeVar5 = ( ( nodeVar4.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) );
	let nodeConst2 = ( nodeVar3.xyz / vec3<f32>( ( ( ( dot( nodeVar3.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 ) ) );
	let nodeConst3 = vec3<f32>( dot( nodeConst2, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst2, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst2, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar6 = nodeConst3;
	nodeVar7 = ( nodeConst3 * nodeConst3 );
	nodeVar8 = 0.0;
	nodeVar9 = 0.0;
	nodeVar10 = 0.0;
	nodeVar11 = 0.0;

	if ( ( nodeVar3.w < 1000.0 ) ) {

		nodeVar8 = ( nodeVar8 + nodeVar3.w );
		nodeVar9 = ( nodeVar9 + 1.0 );
		nodeVar12 = ( nodeVar3.w - nodeVar10 );
		nodeVar10 = ( nodeVar10 + ( nodeVar12 / nodeVar9 ) );
		nodeVar11 = ( nodeVar11 + ( nodeVar12 * ( nodeVar3.w - nodeVar10 ) ) );
		

	}

	nodeVar13 = textureLoad( nodeUniform1, ( nodeConst1 + vec2<i32>( -1, -1 ) ), u32( 0u ) );
	nodeVar14 = max( nodeVar13, vec4<f32>( 0.0 ) );
	let nodeConst4 = ( nodeVar14.xyz / vec3<f32>( ( ( ( dot( nodeVar14.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 ) ) );
	let nodeConst5 = vec3<f32>( dot( nodeConst4, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst4, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst4, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar6 = ( nodeVar6 + nodeConst5 );
	nodeVar7 = ( nodeVar7 + ( nodeConst5 * nodeConst5 ) );

	if ( ( nodeVar14.w < 1000.0 ) ) {

		nodeVar8 = ( nodeVar8 + nodeVar14.w );
		nodeVar9 = ( nodeVar9 + 1.0 );
		nodeVar15 = ( nodeVar14.w - nodeVar10 );
		nodeVar10 = ( nodeVar10 + ( nodeVar15 / nodeVar9 ) );
		nodeVar11 = ( nodeVar11 + ( nodeVar15 * ( nodeVar14.w - nodeVar10 ) ) );
		

	}

	nodeVar16 = textureLoad( nodeUniform1, ( nodeConst1 + vec2<i32>( -1, 1 ) ), u32( 0u ) );
	nodeVar17 = max( nodeVar16, vec4<f32>( 0.0 ) );
	let nodeConst6 = ( nodeVar17.xyz / vec3<f32>( ( ( ( dot( nodeVar17.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 ) ) );
	let nodeConst7 = vec3<f32>( dot( nodeConst6, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst6, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst6, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar6 = ( nodeVar6 + nodeConst7 );
	nodeVar7 = ( nodeVar7 + ( nodeConst7 * nodeConst7 ) );

	if ( ( nodeVar17.w < 1000.0 ) ) {

		nodeVar8 = ( nodeVar8 + nodeVar17.w );
		nodeVar9 = ( nodeVar9 + 1.0 );
		nodeVar18 = ( nodeVar17.w - nodeVar10 );
		nodeVar10 = ( nodeVar10 + ( nodeVar18 / nodeVar9 ) );
		nodeVar11 = ( nodeVar11 + ( nodeVar18 * ( nodeVar17.w - nodeVar10 ) ) );
		

	}

	nodeVar19 = textureLoad( nodeUniform1, ( nodeConst1 + vec2<i32>( 1, -1 ) ), u32( 0u ) );
	nodeVar20 = max( nodeVar19, vec4<f32>( 0.0 ) );
	let nodeConst8 = ( nodeVar20.xyz / vec3<f32>( ( ( ( dot( nodeVar20.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 ) ) );
	let nodeConst9 = vec3<f32>( dot( nodeConst8, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst8, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst8, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar6 = ( nodeVar6 + nodeConst9 );
	nodeVar7 = ( nodeVar7 + ( nodeConst9 * nodeConst9 ) );

	if ( ( nodeVar20.w < 1000.0 ) ) {

		nodeVar8 = ( nodeVar8 + nodeVar20.w );
		nodeVar9 = ( nodeVar9 + 1.0 );
		nodeVar21 = ( nodeVar20.w - nodeVar10 );
		nodeVar10 = ( nodeVar10 + ( nodeVar21 / nodeVar9 ) );
		nodeVar11 = ( nodeVar11 + ( nodeVar21 * ( nodeVar20.w - nodeVar10 ) ) );
		

	}

	nodeVar22 = textureLoad( nodeUniform1, ( nodeConst1 + vec2<i32>( 1, 1 ) ), u32( 0u ) );
	nodeVar23 = max( nodeVar22, vec4<f32>( 0.0 ) );
	let nodeConst10 = ( nodeVar23.xyz / vec3<f32>( ( ( ( dot( nodeVar23.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 ) ) );
	let nodeConst11 = vec3<f32>( dot( nodeConst10, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst10, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst10, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar6 = ( nodeVar6 + nodeConst11 );
	nodeVar7 = ( nodeVar7 + ( nodeConst11 * nodeConst11 ) );

	if ( ( nodeVar23.w < 1000.0 ) ) {

		nodeVar8 = ( nodeVar8 + nodeVar23.w );
		nodeVar9 = ( nodeVar9 + 1.0 );
		nodeVar24 = ( nodeVar23.w - nodeVar10 );
		nodeVar10 = ( nodeVar10 + ( nodeVar24 / nodeVar9 ) );
		nodeVar11 = ( nodeVar11 + ( nodeVar24 * ( nodeVar23.w - nodeVar10 ) ) );
		

	}

	nodeVar25 = textureLoad( nodeUniform1, ( nodeConst1 + vec2<i32>( 1, 0 ) ), u32( 0u ) );
	nodeVar26 = max( nodeVar25, vec4<f32>( 0.0 ) );
	let nodeConst12 = ( nodeVar26.xyz / vec3<f32>( ( ( ( dot( nodeVar26.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 ) ) );
	let nodeConst13 = vec3<f32>( dot( nodeConst12, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst12, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst12, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar6 = ( nodeVar6 + nodeConst13 );
	nodeVar7 = ( nodeVar7 + ( nodeConst13 * nodeConst13 ) );

	if ( ( nodeVar26.w < 1000.0 ) ) {

		nodeVar8 = ( nodeVar8 + nodeVar26.w );
		nodeVar9 = ( nodeVar9 + 1.0 );
		nodeVar27 = ( nodeVar26.w - nodeVar10 );
		nodeVar10 = ( nodeVar10 + ( nodeVar27 / nodeVar9 ) );
		nodeVar11 = ( nodeVar11 + ( nodeVar27 * ( nodeVar26.w - nodeVar10 ) ) );
		

	}

	nodeVar28 = textureLoad( nodeUniform1, ( nodeConst1 + vec2<i32>( 0, -1 ) ), u32( 0u ) );
	nodeVar29 = max( nodeVar28, vec4<f32>( 0.0 ) );
	let nodeConst14 = ( nodeVar29.xyz / vec3<f32>( ( ( ( dot( nodeVar29.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 ) ) );
	let nodeConst15 = vec3<f32>( dot( nodeConst14, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst14, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst14, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar6 = ( nodeVar6 + nodeConst15 );
	nodeVar7 = ( nodeVar7 + ( nodeConst15 * nodeConst15 ) );

	if ( ( nodeVar29.w < 1000.0 ) ) {

		nodeVar8 = ( nodeVar8 + nodeVar29.w );
		nodeVar9 = ( nodeVar9 + 1.0 );
		nodeVar30 = ( nodeVar29.w - nodeVar10 );
		nodeVar10 = ( nodeVar10 + ( nodeVar30 / nodeVar9 ) );
		nodeVar11 = ( nodeVar11 + ( nodeVar30 * ( nodeVar29.w - nodeVar10 ) ) );
		

	}

	nodeVar31 = textureLoad( nodeUniform1, ( nodeConst1 + vec2<i32>( 0, 1 ) ), u32( 0u ) );
	nodeVar32 = max( nodeVar31, vec4<f32>( 0.0 ) );
	let nodeConst16 = ( nodeVar32.xyz / vec3<f32>( ( ( ( dot( nodeVar32.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 ) ) );
	let nodeConst17 = vec3<f32>( dot( nodeConst16, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst16, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst16, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar6 = ( nodeVar6 + nodeConst17 );
	nodeVar7 = ( nodeVar7 + ( nodeConst17 * nodeConst17 ) );

	if ( ( nodeVar32.w < 1000.0 ) ) {

		nodeVar8 = ( nodeVar8 + nodeVar32.w );
		nodeVar9 = ( nodeVar9 + 1.0 );
		nodeVar33 = ( nodeVar32.w - nodeVar10 );
		nodeVar10 = ( nodeVar10 + ( nodeVar33 / nodeVar9 ) );
		nodeVar11 = ( nodeVar11 + ( nodeVar33 * ( nodeVar32.w - nodeVar10 ) ) );
		

	}

	nodeVar34 = textureLoad( nodeUniform1, ( nodeConst1 + vec2<i32>( -1, 0 ) ), u32( 0u ) );
	nodeVar35 = max( nodeVar34, vec4<f32>( 0.0 ) );
	let nodeConst18 = ( nodeVar35.xyz / vec3<f32>( ( ( ( dot( nodeVar35.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 ) ) );
	let nodeConst19 = vec3<f32>( dot( nodeConst18, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst18, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst18, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar6 = ( nodeVar6 + nodeConst19 );
	nodeVar7 = ( nodeVar7 + ( nodeConst19 * nodeConst19 ) );

	if ( ( nodeVar35.w < 1000.0 ) ) {

		nodeVar8 = ( nodeVar8 + nodeVar35.w );
		nodeVar9 = ( nodeVar9 + 1.0 );
		nodeVar36 = ( nodeVar35.w - nodeVar10 );
		nodeVar10 = ( nodeVar10 + ( nodeVar36 / nodeVar9 ) );
		nodeVar11 = ( nodeVar11 + ( nodeVar36 * ( nodeVar35.w - nodeVar10 ) ) );
		

	}

	let nodeConst20 = ( nodeVar6 / vec3<f32>( 9.0 ) );

	if ( ( nodeVar9 < 0.5 ) ) {

		nodeVar38 = 10000.0;

	} else {

		nodeVar38 = ( nodeVar8 / max( nodeVar9, 0.0001 ) );

	}

	nodeVar37 = StructType2( nodeConst20, sqrt( max( ( ( nodeVar7 / vec3<f32>( 9.0 ) ) - ( nodeConst20 * nodeConst20 ) ), vec3<f32>( 0.0 ) ) ), nodeVar38, ( 1.0 - ( nodeVar9 / 9.0 ) ), max( sqrt( ( nodeVar11 / max( nodeVar9, 1.0 ) ) ), 0.001 ) );
	var nodeVar39 : StructType2 = nodeVar37;
	nodeVar40 = normalize( ( vec4<f32>( nodeVar5, 0.0 ) * object.nodeUniform5 ).xyz );
	let nodeConst21 = ( object.nodeUniform6 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar1 ), 1.0 ) );
	nodeVar41 = ( nodeConst21.xyz / vec3<f32>( nodeConst21.w ) );
	nodeVar42 = ( object.nodeUniform7 * vec4<f32>( nodeVar41, 1.0 ) ).xyz;
	nodeVar43 = textureLoad( nodeUniform8, nodeConst0, u32( 0u ) );
	nodeVar44 = fn145( nodeVar43.xy );
	nodeVar45 = ( nodeVarying0 - nodeVar44 );
	nodeVar46 = ( ( nodeVar45 * object.nodeUniform2 ) - vec2<f32>( 0.5 ) );
	let nodeConst22 = vec2<i32>( floor( nodeVar46 ) );
	let nodeConst23 = ( nodeConst22 + vec2<i32>( 0, 0 ) );
	let nodeConst24 = ( ( vec2<f32>( nodeConst23 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar49 = textureLoad( nodeUniform11, vec2<i32>( ( object.nodeUniform12 * vec3<f32>( vec2<f32>( nodeConst23 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst25 = ( object.nodeUniform10 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst24.x, ( 1.0 - nodeConst24.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar49 ), 1.0 ) );
	let nodeConst26 = ( nodeConst25.xyz / vec3<f32>( nodeConst25.w ) );
	nodeVar50 = abs( dot( ( ( object.nodeUniform9 * vec4<f32>( nodeConst26, 1.0 ) ).xyz - nodeVar42 ), nodeVar40 ) );
	nodeVar50 = ( nodeVar50 / abs( nodeConst26.z ) );
	nodeVar52 = textureLoad( nodeUniform13, vec2<i32>( ( object.nodeUniform14 * vec3<f32>( vec2<f32>( nodeConst23 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst27 = fract( nodeVar46 );
	nodeVar53 = textureLoad( nodeUniform15, vec2<i32>( ( object.nodeUniform16 * vec3<f32>( vec2<f32>( nodeConst23 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst28 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar50 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar53.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform17 ).xyz ), nodeVar40 ) ) );
	let nodeConst29 = ( ( ( 1.0 - nodeConst27.x ) * ( 1.0 - nodeConst27.y ) ) * nodeConst28 );
	nodeVar51 = StructType0( ( max( nodeVar52, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeConst29 ) ), nodeConst29, nodeConst28 );
	let nodeConst30 = ( nodeConst22 + vec2<i32>( 1, 0 ) );
	let nodeConst31 = ( ( vec2<f32>( nodeConst30 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar54 = textureLoad( nodeUniform11, vec2<i32>( ( object.nodeUniform12 * vec3<f32>( vec2<f32>( nodeConst30 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst32 = ( object.nodeUniform10 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst31.x, ( 1.0 - nodeConst31.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar54 ), 1.0 ) );
	let nodeConst33 = ( nodeConst32.xyz / vec3<f32>( nodeConst32.w ) );
	nodeVar55 = abs( dot( ( ( object.nodeUniform9 * vec4<f32>( nodeConst33, 1.0 ) ).xyz - nodeVar42 ), nodeVar40 ) );
	nodeVar55 = ( nodeVar55 / abs( nodeConst33.z ) );
	nodeVar57 = textureLoad( nodeUniform13, vec2<i32>( ( object.nodeUniform14 * vec3<f32>( vec2<f32>( nodeConst30 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar58 = textureLoad( nodeUniform15, vec2<i32>( ( object.nodeUniform16 * vec3<f32>( vec2<f32>( nodeConst30 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst34 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar55 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar58.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform17 ).xyz ), nodeVar40 ) ) );
	let nodeConst35 = ( ( nodeConst27.x * ( 1.0 - nodeConst27.y ) ) * nodeConst34 );
	nodeVar56 = StructType0( ( max( nodeVar57, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeConst35 ) ), nodeConst35, nodeConst34 );
	let nodeConst36 = ( nodeConst22 + vec2<i32>( 0, 1 ) );
	let nodeConst37 = ( ( vec2<f32>( nodeConst36 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar59 = textureLoad( nodeUniform11, vec2<i32>( ( object.nodeUniform12 * vec3<f32>( vec2<f32>( nodeConst36 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst38 = ( object.nodeUniform10 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst37.x, ( 1.0 - nodeConst37.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar59 ), 1.0 ) );
	let nodeConst39 = ( nodeConst38.xyz / vec3<f32>( nodeConst38.w ) );
	nodeVar60 = abs( dot( ( ( object.nodeUniform9 * vec4<f32>( nodeConst39, 1.0 ) ).xyz - nodeVar42 ), nodeVar40 ) );
	nodeVar60 = ( nodeVar60 / abs( nodeConst39.z ) );
	nodeVar62 = textureLoad( nodeUniform13, vec2<i32>( ( object.nodeUniform14 * vec3<f32>( vec2<f32>( nodeConst36 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar63 = textureLoad( nodeUniform15, vec2<i32>( ( object.nodeUniform16 * vec3<f32>( vec2<f32>( nodeConst36 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst40 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar60 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar63.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform17 ).xyz ), nodeVar40 ) ) );
	let nodeConst41 = ( ( ( 1.0 - nodeConst27.x ) * nodeConst27.y ) * nodeConst40 );
	nodeVar61 = StructType0( ( max( nodeVar62, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeConst41 ) ), nodeConst41, nodeConst40 );
	let nodeConst42 = ( nodeConst22 + vec2<i32>( 1, 1 ) );
	let nodeConst43 = ( ( vec2<f32>( nodeConst42 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar64 = textureLoad( nodeUniform11, vec2<i32>( ( object.nodeUniform12 * vec3<f32>( vec2<f32>( nodeConst42 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst44 = ( object.nodeUniform10 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst43.x, ( 1.0 - nodeConst43.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar64 ), 1.0 ) );
	let nodeConst45 = ( nodeConst44.xyz / vec3<f32>( nodeConst44.w ) );
	nodeVar65 = abs( dot( ( ( object.nodeUniform9 * vec4<f32>( nodeConst45, 1.0 ) ).xyz - nodeVar42 ), nodeVar40 ) );
	nodeVar65 = ( nodeVar65 / abs( nodeConst45.z ) );
	nodeVar67 = textureLoad( nodeUniform13, vec2<i32>( ( object.nodeUniform14 * vec3<f32>( vec2<f32>( nodeConst42 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar68 = textureLoad( nodeUniform15, vec2<i32>( ( object.nodeUniform16 * vec3<f32>( vec2<f32>( nodeConst42 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst46 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar65 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar68.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform17 ).xyz ), nodeVar40 ) ) );
	let nodeConst47 = ( ( nodeConst27.x * nodeConst27.y ) * nodeConst46 );
	nodeVar66 = StructType0( ( max( nodeVar67, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeConst47 ) ), nodeConst47, nodeConst46 );
	let nodeConst48 = ( ( ( nodeVar51.weight + nodeVar56.weight ) + nodeVar61.weight ) + nodeVar66.weight );

	if ( ( nodeConst48 > 0.01 ) ) {

		nodeVar48 = ( ( ( ( nodeVar51.color + nodeVar56.color ) + nodeVar61.color ) + nodeVar66.color ) / vec4<f32>( nodeConst48 ) );

	} else {

		nodeVar48 = vec4<f32>( nodeVar3.xyz, 1.0 );

	}

	nodeVar47 = StructType1( nodeVar48, max( max( nodeVar51.confidence, nodeVar56.confidence ), max( nodeVar61.confidence, nodeVar66.confidence ) ), min( min( nodeVar51.confidence, nodeVar56.confidence ), min( nodeVar61.confidence, nodeVar66.confidence ) ) );
	nodeVar69 = nodeVar47.color;
	nodeVar70 = 1.0;
	nodeVar71 = 0.0;
	let nodeConst49 = fn144( nodeVar42, nodeVar39.rayLength, object.nodeUniform18, object.nodeUniform17, object.nodeUniform19 );
	nodeVar72 = nodeConst49;
	let nodeConst50 = clamp( ( length( ( nodeVar44 * object.nodeUniform2 ) ) / 128.0 ), 0.0, 1.0 );
	nodeVar39.stdDevRayLength = ( 1.0 - min( ( ( nodeVar39.stdDevRayLength * min( ( nodeConst50 * 100.0 ), 1.0 ) ) * 3.5 ), 1.0 ) );
	nodeVar73 = ( ( nodeVar72 * object.nodeUniform2 ) - vec2<f32>( 0.5 ) );
	let nodeConst51 = vec2<i32>( floor( nodeVar73 ) );
	let nodeConst52 = ( nodeConst51 + vec2<i32>( 0, 0 ) );
	let nodeConst53 = ( ( vec2<f32>( nodeConst52 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar76 = textureLoad( nodeUniform11, vec2<i32>( ( object.nodeUniform12 * vec3<f32>( vec2<f32>( nodeConst52 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst54 = ( object.nodeUniform10 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst53.x, ( 1.0 - nodeConst53.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar76 ), 1.0 ) );
	let nodeConst55 = ( nodeConst54.xyz / vec3<f32>( nodeConst54.w ) );
	nodeVar77 = abs( dot( ( ( object.nodeUniform9 * vec4<f32>( nodeConst55, 1.0 ) ).xyz - nodeVar42 ), nodeVar40 ) );
	nodeVar77 = ( nodeVar77 / abs( nodeConst55.z ) );
	nodeVar79 = textureLoad( nodeUniform13, vec2<i32>( ( object.nodeUniform14 * vec3<f32>( vec2<f32>( nodeConst52 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst56 = fract( nodeVar73 );
	nodeVar80 = textureLoad( nodeUniform15, vec2<i32>( ( object.nodeUniform16 * vec3<f32>( vec2<f32>( nodeConst52 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst57 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar77 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar80.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform17 ).xyz ), nodeVar40 ) ) );
	let nodeConst58 = ( ( ( 1.0 - nodeConst56.x ) * ( 1.0 - nodeConst56.y ) ) * nodeConst57 );
	nodeVar78 = StructType0( ( max( nodeVar79, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeConst58 ) ), nodeConst58, nodeConst57 );
	let nodeConst59 = ( nodeConst51 + vec2<i32>( 1, 0 ) );
	let nodeConst60 = ( ( vec2<f32>( nodeConst59 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar81 = textureLoad( nodeUniform11, vec2<i32>( ( object.nodeUniform12 * vec3<f32>( vec2<f32>( nodeConst59 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst61 = ( object.nodeUniform10 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst60.x, ( 1.0 - nodeConst60.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar81 ), 1.0 ) );
	let nodeConst62 = ( nodeConst61.xyz / vec3<f32>( nodeConst61.w ) );
	nodeVar82 = abs( dot( ( ( object.nodeUniform9 * vec4<f32>( nodeConst62, 1.0 ) ).xyz - nodeVar42 ), nodeVar40 ) );
	nodeVar82 = ( nodeVar82 / abs( nodeConst62.z ) );
	nodeVar84 = textureLoad( nodeUniform13, vec2<i32>( ( object.nodeUniform14 * vec3<f32>( vec2<f32>( nodeConst59 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar85 = textureLoad( nodeUniform15, vec2<i32>( ( object.nodeUniform16 * vec3<f32>( vec2<f32>( nodeConst59 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst63 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar82 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar85.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform17 ).xyz ), nodeVar40 ) ) );
	let nodeConst64 = ( ( nodeConst56.x * ( 1.0 - nodeConst56.y ) ) * nodeConst63 );
	nodeVar83 = StructType0( ( max( nodeVar84, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeConst64 ) ), nodeConst64, nodeConst63 );
	let nodeConst65 = ( nodeConst51 + vec2<i32>( 0, 1 ) );
	let nodeConst66 = ( ( vec2<f32>( nodeConst65 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar86 = textureLoad( nodeUniform11, vec2<i32>( ( object.nodeUniform12 * vec3<f32>( vec2<f32>( nodeConst65 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst67 = ( object.nodeUniform10 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst66.x, ( 1.0 - nodeConst66.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar86 ), 1.0 ) );
	let nodeConst68 = ( nodeConst67.xyz / vec3<f32>( nodeConst67.w ) );
	nodeVar87 = abs( dot( ( ( object.nodeUniform9 * vec4<f32>( nodeConst68, 1.0 ) ).xyz - nodeVar42 ), nodeVar40 ) );
	nodeVar87 = ( nodeVar87 / abs( nodeConst68.z ) );
	nodeVar89 = textureLoad( nodeUniform13, vec2<i32>( ( object.nodeUniform14 * vec3<f32>( vec2<f32>( nodeConst65 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar90 = textureLoad( nodeUniform15, vec2<i32>( ( object.nodeUniform16 * vec3<f32>( vec2<f32>( nodeConst65 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst69 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar87 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar90.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform17 ).xyz ), nodeVar40 ) ) );
	let nodeConst70 = ( ( ( 1.0 - nodeConst56.x ) * nodeConst56.y ) * nodeConst69 );
	nodeVar88 = StructType0( ( max( nodeVar89, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeConst70 ) ), nodeConst70, nodeConst69 );
	let nodeConst71 = ( nodeConst51 + vec2<i32>( 1, 1 ) );
	let nodeConst72 = ( ( vec2<f32>( nodeConst71 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar91 = textureLoad( nodeUniform11, vec2<i32>( ( object.nodeUniform12 * vec3<f32>( vec2<f32>( nodeConst71 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst73 = ( object.nodeUniform10 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst72.x, ( 1.0 - nodeConst72.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar91 ), 1.0 ) );
	let nodeConst74 = ( nodeConst73.xyz / vec3<f32>( nodeConst73.w ) );
	nodeVar92 = abs( dot( ( ( object.nodeUniform9 * vec4<f32>( nodeConst74, 1.0 ) ).xyz - nodeVar42 ), nodeVar40 ) );
	nodeVar92 = ( nodeVar92 / abs( nodeConst74.z ) );
	nodeVar94 = textureLoad( nodeUniform13, vec2<i32>( ( object.nodeUniform14 * vec3<f32>( vec2<f32>( nodeConst71 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar95 = textureLoad( nodeUniform15, vec2<i32>( ( object.nodeUniform16 * vec3<f32>( vec2<f32>( nodeConst71 ), 1.0 ) ).xy ), u32( 0u ) );
	let nodeConst75 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar92 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar95.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform17 ).xyz ), nodeVar40 ) ) );
	let nodeConst76 = ( ( nodeConst56.x * nodeConst56.y ) * nodeConst75 );
	nodeVar93 = StructType0( ( max( nodeVar94, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeConst76 ) ), nodeConst76, nodeConst75 );
	let nodeConst77 = ( ( ( nodeVar78.weight + nodeVar83.weight ) + nodeVar88.weight ) + nodeVar93.weight );

	if ( ( nodeConst77 > 0.01 ) ) {

		nodeVar75 = ( ( ( ( nodeVar78.color + nodeVar83.color ) + nodeVar88.color ) + nodeVar93.color ) / vec4<f32>( nodeConst77 ) );

	} else {

		nodeVar75 = vec4<f32>( nodeVar3.xyz, 1.0 );

	}

	nodeVar74 = StructType1( nodeVar75, max( max( nodeVar78.confidence, nodeVar83.confidence ), max( nodeVar88.confidence, nodeVar93.confidence ) ), min( min( nodeVar78.confidence, nodeVar83.confidence ), min( nodeVar88.confidence, nodeVar93.confidence ) ) );
	nodeVar97 = bool( object.nodeUniform20 );

	if ( ( ( ( ( ( nodeVar72.x >= 0.0 ) && ( nodeVar72.x <= 1.0 ) ) && ( nodeVar72.y >= 0.0 ) ) && ( nodeVar72.y <= 1.0 ) ) && nodeVar97 ) ) {

		nodeVar96 = nodeVar74.tapConfidence;

	} else {

		nodeVar96 = 0.0;

	}

	let nodeConst78 = ( ( ( nodeVar74.minConfidence * nodeVar39.stdDevRayLength ) * ( 1.0 - clamp( ( length( fwidth( nodeVar40 ) ) * 50.0 ), 0.0, 1.0 ) ) ) * nodeVar96 );
	let nodeConst79 = ( nodeConst78 * ( 1.0 - ( nodeVar39.envProbability * nodeVar39.envProbability ) ) );

	if ( ( ( ( ( nodeVar45.x >= 0.0 ) && ( nodeVar45.x <= 1.0 ) ) && ( nodeVar45.y >= 0.0 ) ) && ( nodeVar45.y <= 1.0 ) ) ) {

		nodeVar98 = nodeVar47.tapConfidence;

	} else {

		nodeVar98 = 0.0;

	}

	let nodeConst80 = ( ( 1.0 - nodeConst79 ) * nodeVar98 );
	let nodeConst81 = max( ( nodeConst79 + nodeConst80 ), 0.000001 );
	nodeVar99 = vec4<f32>( ( ( ( max( nodeVar74.color.xyz, vec3<f32>( 0.0 ) ) * vec3<f32>( nodeConst79 ) ) + ( max( nodeVar47.color.xyz, vec3<f32>( 0.0 ) ) * vec3<f32>( nodeConst80 ) ) ) / vec3<f32>( nodeConst81 ) ), nodeVar47.color.w );

	if ( ( length( nodeVar99.xyz ) < 0.000001 ) ) {

		nodeVar99 = vec4<f32>( nodeVar3.xyz, 1.0 );
		

	}

	nodeVar69 = nodeVar99;
	nodeVar70 = ( ( ( nodeVar96 * nodeConst79 ) + ( nodeVar98 * nodeConst80 ) ) / nodeConst81 );
	nodeVar71 = nodeConst78;
	let nodeConst82 = fn143( nodeVar45, object.nodeUniform2 );
	nodeVar70 = ( nodeVar70 * pow( nodeConst82, 2.0 ) );
	nodeVar100 = nodeVar69.xyz;
	let nodeConst83 = ( ( ( dot( nodeVar100, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform4 ) * 10.0 ) + 1.0 );
	let nodeConst84 = ( nodeVar100 / vec3<f32>( nodeConst83 ) );
	let nodeConst85 = ( 1.0 - nodeConst50 );
	let nodeConst86 = ( nodeVar39.stdColor * vec3<f32>( mix( 0.5, 1.0, ( nodeConst85 * nodeConst85 ) ) ) );
	let nodeConst87 = fn142( vec3<f32>( dot( nodeConst84, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeConst84, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeConst84, vec3<f32>( -0.25, 0.5, -0.25 ) ) ), ( nodeVar39.mean - nodeConst86 ), ( nodeVar39.mean + nodeConst86 ) );
	nodeVar101 = ( vec3<f32>( ( ( nodeConst87.x + nodeConst87.y ) - nodeConst87.z ), ( nodeConst87.x + nodeConst87.z ), ( ( nodeConst87.x - nodeConst87.y ) - nodeConst87.z ) ) * vec3<f32>( nodeConst83 ) );
	let nodeConst88 = ( ( object.nodeUniform21 * max( min( ( nodeConst50 * 10.0 ), 1.0 ), 0.25 ) ) * ( 1.0 + clamp( ( ( 1.0 - nodeConst82 ) + ( 1.0 - nodeVar71 ) ), 0.0, 1.0 ) ) );
	nodeVar102 = mix( nodeVar69.xyz, nodeVar101, nodeConst88 );
	nodeVar69.x = nodeVar102[ 0 ];
	nodeVar69.y = nodeVar102[ 1 ];
	nodeVar69.z = nodeVar102[ 2 ];
	nodeVar70 = ( nodeVar70 * exp( ( - ( ( length( ( nodeVar69.xyz - nodeVar101 ) ) * nodeConst88 ) * 30.0 ) ) ) );
	nodeVar70 = ( nodeVar70 * mix( 1.0, ( ( nodeVar71 * 0.05 ) + 0.95 ), clamp( ( nodeConst50 * 100.0 ), 0.0, 1.0 ) ) );

	if ( ( nodeVar70 < 0.000001 ) ) {

		nodeVar69 = vec4<f32>( nodeVar3.xyz, 1.0 );
		

	}

	nodeVar103 = min( ( ( ( 1.0 / max( nodeVar69.w, 0.000001 ) ) * nodeVar70 ) + 1.0 ), object.nodeUniform22 );

	if ( ( length( nodeVar3.xyz ) < 0.000001 ) ) {

		nodeVar103 = max( ( nodeVar103 - 1.0 ), 1.0 );
		

	}


	// result

	output.color = vec4<f32>( nodeVar69.xyz, ( 1.0 / nodeVar103 ) );

	return output;

}
