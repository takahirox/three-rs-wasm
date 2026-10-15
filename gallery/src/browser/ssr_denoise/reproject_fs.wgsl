// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


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
@binding( 6 ) @group( 0 ) var nodeUniform7_sampler : sampler;
@binding( 7 ) @group( 0 ) var nodeUniform7 : texture_2d<f32>;
@binding( 8 ) @group( 0 ) var nodeUniform10 : texture_depth_2d;
@binding( 9 ) @group( 0 ) var nodeUniform12_sampler : sampler;
@binding( 10 ) @group( 0 ) var nodeUniform12 : texture_2d<f32>;
@binding( 11 ) @group( 0 ) var nodeUniform14_sampler : sampler;
@binding( 12 ) @group( 0 ) var nodeUniform14 : texture_2d<f32>;

struct objectStruct {
	nodeUniform2 : vec2<f32>,
	nodeUniform4 : mat4x4<f32>,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : mat4x4<f32>,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform19 : mat3x3<f32>,
	nodeUniform20 : mat3x3<f32>,
	nodeUniform21 : mat3x3<f32>,
	nodeUniform22 : mat3x3<f32>,
	nodeUniform23 : mat3x3<f32>,
	nodeUniform24 : mat3x3<f32>,
	nodeUniform25 : mat3x3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : f32,
	nodeUniform28 : mat4x4<f32>,
	nodeUniform29 : mat3x3<f32>,
	nodeUniform30 : mat3x3<f32>,
	nodeUniform31 : mat3x3<f32>,
	nodeUniform32 : mat3x3<f32>,
	nodeUniform33 : mat3x3<f32>,
	nodeUniform34 : mat3x3<f32>,
	nodeUniform35 : mat3x3<f32>,
	nodeUniform36 : mat3x3<f32>,
	nodeUniform37 : mat3x3<f32>,
	nodeUniform38 : mat3x3<f32>,
	nodeUniform39 : mat3x3<f32>,
	nodeUniform40 : mat3x3<f32>,
	nodeUniform41 : u32,
	nodeUniform42 : f32,
	nodeUniform43 : f32
};
@binding( 3 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<i32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : StructType1;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec2<i32>;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : StructType0;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<i32>;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : StructType0;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<i32>;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : StructType0;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<i32>;
var<private> nodeVar48 : vec2<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : StructType0;
var<private> nodeVar53 : vec4<f32>;
var<private> nodeVar54 : vec4<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec4<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec4<f32>;
var<private> nodeVar72 : vec4<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : vec4<f32>;
var<private> nodeVar77 : vec4<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec4<f32>;
var<private> nodeVar82 : vec4<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec4<f32>;
var<private> nodeVar87 : vec4<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec4<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec4<f32>;
var<private> nodeVar97 : vec4<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec4<f32>;
var<private> nodeVar102 : vec4<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec4<f32>;
var<private> nodeVar107 : vec4<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : StructType2;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec2<f32>;
var<private> nodeVar115 : vec2<f32>;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec2<f32>;
var<private> nodeVar118 : StructType1;
var<private> nodeVar119 : vec4<f32>;
var<private> nodeVar120 : vec2<i32>;
var<private> nodeVar121 : vec2<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : StructType0;
var<private> nodeVar126 : vec4<f32>;
var<private> nodeVar127 : vec2<f32>;
var<private> nodeVar128 : vec4<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : vec2<i32>;
var<private> nodeVar132 : vec2<f32>;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : StructType0;
var<private> nodeVar137 : vec4<f32>;
var<private> nodeVar138 : vec4<f32>;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : vec2<i32>;
var<private> nodeVar142 : vec2<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : StructType0;
var<private> nodeVar147 : vec4<f32>;
var<private> nodeVar148 : vec4<f32>;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : vec2<i32>;
var<private> nodeVar152 : vec2<f32>;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : StructType0;
var<private> nodeVar157 : vec4<f32>;
var<private> nodeVar158 : vec4<f32>;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : bool;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : vec4<f32>;
var<private> nodeVar169 : vec2<f32>;
var<private> nodeVar170 : vec2<f32>;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : f32;

// codes
fn beautyTexelFromScreen ( screenTexel : vec2<i32>, beautySize : vec2<f32>, resolveSize : vec2<f32> ) -> vec2<i32> {

	


	return vec2<i32>( floor( ( ( vec2<f32>( screenTexel ) * beautySize ) / resolveSize ) ) );

}


fn velocityToUVOffset ( velocity : vec2<f32> ) -> vec2<f32> {

	


	return ( velocity * vec2<f32>( 0.5, -0.5 ) );

}


fn projectWorldToUV ( worldPos : vec3<f32>, previousViewMatrix : mat4x4<f32>, previousProjectionMatrix : mat4x4<f32> ) -> vec2<f32> {

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


fn clipToAABB ( history : vec3<f32>, boxMin : vec3<f32>, boxMax : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<f32>;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : f32;

	nodeVar1 = ( ( boxMax + boxMin ) * vec3<f32>( 0.5 ) );
	nodeVar2 = ( history - nodeVar1 );
	nodeVar3 = abs( ( nodeVar2 / ( ( ( boxMax - boxMin ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 1e-7 ) ) ) );
	nodeVar4 = max( max( nodeVar3.x, nodeVar3.y ), nodeVar3.z );

	if ( ( nodeVar4 > 1.0 ) ) {

		nodeVar0 = ( nodeVar1 + ( nodeVar2 / vec3<f32>( nodeVar4 ) ) );

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

	nodeVar0 = textureLoad( nodeUniform0, vec2<i32>( floor( ( fragCoord.xy - vec2<f32>( 0.5 ) ) ) ), u32( 0u ) );
	nodeVar1 = nodeVar0;

	if ( ( nodeVar1 >= 1.0 ) ) {

		discard;
		

	}

	nodeVar2 = beautyTexelFromScreen( vec2<i32>( floor( ( fragCoord.xy - vec2<f32>( 0.5 ) ) ) ), vec2<f32>( textureDimensions( nodeUniform1, 0 ) ), object.nodeUniform2 );
	nodeVar3 = textureLoad( nodeUniform1, nodeVar2, u32( 0u ) );
	nodeVar4 = max( nodeVar3, vec4<f32>( 0.0 ) );
	nodeVar5 = textureLoad( nodeUniform3, vec2<i32>( floor( ( fragCoord.xy - vec2<f32>( 0.5 ) ) ) ), u32( 0u ) );
	nodeVar6 = ( ( nodeVar5.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) );
	nodeVar7 = normalize( ( vec4<f32>( nodeVar6, 0.0 ) * object.nodeUniform4 ).xyz );
	nodeVar8 = ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar1 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar1 ), 1.0 ) ).w ) );
	nodeVar9 = ( object.nodeUniform6 * vec4<f32>( nodeVar8, 1.0 ) ).xyz;
	nodeVar10 = textureLoad( nodeUniform7, vec2<i32>( floor( ( fragCoord.xy - vec2<f32>( 0.5 ) ) ) ), u32( 0u ) );
	nodeVar11 = velocityToUVOffset( nodeVar10.xy );
	nodeVar12 = ( nodeVarying0 - nodeVar11 );
	nodeVar13 = ( ( nodeVar12 * object.nodeUniform2 ) - vec2<f32>( 0.5 ) );
	nodeVar16 = ( vec2<i32>( floor( nodeVar13 ) ) + vec2<i32>( 0, 0 ) );
	nodeVar17 = ( ( vec2<f32>( nodeVar16 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar18 = textureLoad( nodeUniform10, vec2<i32>( ( object.nodeUniform11 * vec3<f32>( vec2<f32>( nodeVar16 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar19 = ( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar17.x, ( 1.0 - nodeVar17.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar18 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar17.x, ( 1.0 - nodeVar17.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar18 ), 1.0 ) ).w ) );
	nodeVar20 = abs( dot( ( ( object.nodeUniform8 * vec4<f32>( nodeVar19, 1.0 ) ).xyz - nodeVar9 ), nodeVar7 ) );
	nodeVar20 = ( nodeVar20 / abs( nodeVar19.z ) );
	nodeVar22 = textureLoad( nodeUniform12, vec2<i32>( ( object.nodeUniform13 * vec3<f32>( vec2<f32>( nodeVar16 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar23 = fract( nodeVar13 );
	nodeVar24 = textureLoad( nodeUniform14, vec2<i32>( ( object.nodeUniform15 * vec3<f32>( vec2<f32>( nodeVar16 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar25 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar20 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar24.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform16 ).xyz ), nodeVar7 ) ) );
	nodeVar26 = ( ( ( 1.0 - nodeVar23.x ) * ( 1.0 - nodeVar23.y ) ) * nodeVar25 );
	nodeVar21 = StructType0( ( max( nodeVar22, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeVar26 ) ), nodeVar26, nodeVar25 );
	nodeVar27 = ( vec2<i32>( floor( nodeVar13 ) ) + vec2<i32>( 1, 0 ) );
	nodeVar28 = ( ( vec2<f32>( nodeVar27 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar29 = textureLoad( nodeUniform10, vec2<i32>( ( object.nodeUniform17 * vec3<f32>( vec2<f32>( nodeVar27 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar30 = ( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar28.x, ( 1.0 - nodeVar28.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar29 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar28.x, ( 1.0 - nodeVar28.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar29 ), 1.0 ) ).w ) );
	nodeVar31 = abs( dot( ( ( object.nodeUniform8 * vec4<f32>( nodeVar30, 1.0 ) ).xyz - nodeVar9 ), nodeVar7 ) );
	nodeVar31 = ( nodeVar31 / abs( nodeVar30.z ) );
	nodeVar33 = textureLoad( nodeUniform12, vec2<i32>( ( object.nodeUniform18 * vec3<f32>( vec2<f32>( nodeVar27 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar34 = textureLoad( nodeUniform14, vec2<i32>( ( object.nodeUniform19 * vec3<f32>( vec2<f32>( nodeVar27 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar35 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar31 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar34.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform16 ).xyz ), nodeVar7 ) ) );
	nodeVar36 = ( ( nodeVar23.x * ( 1.0 - nodeVar23.y ) ) * nodeVar35 );
	nodeVar32 = StructType0( ( max( nodeVar33, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeVar36 ) ), nodeVar36, nodeVar35 );
	nodeVar37 = ( vec2<i32>( floor( nodeVar13 ) ) + vec2<i32>( 0, 1 ) );
	nodeVar38 = ( ( vec2<f32>( nodeVar37 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar39 = textureLoad( nodeUniform10, vec2<i32>( ( object.nodeUniform20 * vec3<f32>( vec2<f32>( nodeVar37 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar40 = ( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar38.x, ( 1.0 - nodeVar38.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar39 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar38.x, ( 1.0 - nodeVar38.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar39 ), 1.0 ) ).w ) );
	nodeVar41 = abs( dot( ( ( object.nodeUniform8 * vec4<f32>( nodeVar40, 1.0 ) ).xyz - nodeVar9 ), nodeVar7 ) );
	nodeVar41 = ( nodeVar41 / abs( nodeVar40.z ) );
	nodeVar43 = textureLoad( nodeUniform12, vec2<i32>( ( object.nodeUniform21 * vec3<f32>( vec2<f32>( nodeVar37 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar44 = textureLoad( nodeUniform14, vec2<i32>( ( object.nodeUniform22 * vec3<f32>( vec2<f32>( nodeVar37 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar45 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar41 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar44.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform16 ).xyz ), nodeVar7 ) ) );
	nodeVar46 = ( ( ( 1.0 - nodeVar23.x ) * nodeVar23.y ) * nodeVar45 );
	nodeVar42 = StructType0( ( max( nodeVar43, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeVar46 ) ), nodeVar46, nodeVar45 );
	nodeVar47 = ( vec2<i32>( floor( nodeVar13 ) ) + vec2<i32>( 1, 1 ) );
	nodeVar48 = ( ( vec2<f32>( nodeVar47 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar49 = textureLoad( nodeUniform10, vec2<i32>( ( object.nodeUniform23 * vec3<f32>( vec2<f32>( nodeVar47 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar50 = ( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar48.x, ( 1.0 - nodeVar48.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar49 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar48.x, ( 1.0 - nodeVar48.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar49 ), 1.0 ) ).w ) );
	nodeVar51 = abs( dot( ( ( object.nodeUniform8 * vec4<f32>( nodeVar50, 1.0 ) ).xyz - nodeVar9 ), nodeVar7 ) );
	nodeVar51 = ( nodeVar51 / abs( nodeVar50.z ) );
	nodeVar53 = textureLoad( nodeUniform12, vec2<i32>( ( object.nodeUniform24 * vec3<f32>( vec2<f32>( nodeVar47 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar54 = textureLoad( nodeUniform14, vec2<i32>( ( object.nodeUniform25 * vec3<f32>( vec2<f32>( nodeVar47 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar55 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar51 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar54.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform16 ).xyz ), nodeVar7 ) ) );
	nodeVar56 = ( ( nodeVar23.x * nodeVar23.y ) * nodeVar55 );
	nodeVar52 = StructType0( ( max( nodeVar53, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeVar56 ) ), nodeVar56, nodeVar55 );
	nodeVar57 = ( ( ( nodeVar21.weight + nodeVar32.weight ) + nodeVar42.weight ) + nodeVar52.weight );

	if ( ( nodeVar57 > 0.01 ) ) {

		nodeVar15 = ( ( ( ( nodeVar21.color + nodeVar32.color ) + nodeVar42.color ) + nodeVar52.color ) / vec4<f32>( nodeVar57 ) );

	} else {

		nodeVar15 = vec4<f32>( nodeVar4.xyz, 1.0 );

	}

	nodeVar14 = StructType1( nodeVar15, max( max( nodeVar21.confidence, nodeVar32.confidence ), max( nodeVar42.confidence, nodeVar52.confidence ) ), min( min( nodeVar21.confidence, nodeVar32.confidence ), min( nodeVar42.confidence, nodeVar52.confidence ) ) );
	nodeVar58 = nodeVar14.color;
	nodeVar59 = 1.0;
	nodeVar60 = 0.0;
	nodeVar61 = normalize( ( nodeVar9 - object.nodeUniform26 ) );
	nodeVar62 = ( nodeVar4.xyz / vec3<f32>( ( ( ( dot( nodeVar4.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 ) ) );
	nodeVar63 = vec3<f32>( dot( nodeVar62, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar62, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar62, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar64 = nodeVar63;
	nodeVar65 = ( nodeVar63 * nodeVar63 );
	nodeVar66 = 0.0;
	nodeVar67 = 0.0;
	nodeVar68 = 0.0;
	nodeVar69 = 0.0;

	if ( ( nodeVar4.w < 1000.0 ) ) {

		nodeVar66 = ( nodeVar66 + nodeVar4.w );
		nodeVar67 = ( nodeVar67 + 1.0 );
		nodeVar70 = ( nodeVar4.w - nodeVar68 );
		nodeVar68 = ( nodeVar68 + ( nodeVar70 / nodeVar67 ) );
		nodeVar69 = ( nodeVar69 + ( nodeVar70 * ( nodeVar4.w - nodeVar68 ) ) );
		

	}

	nodeVar71 = textureLoad( nodeUniform1, ( nodeVar2 + vec2<i32>( -1, -1 ) ), u32( 0u ) );
	nodeVar72 = max( nodeVar71, vec4<f32>( 0.0 ) );
	nodeVar73 = ( nodeVar72.xyz / vec3<f32>( ( ( ( dot( nodeVar72.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 ) ) );
	nodeVar74 = vec3<f32>( dot( nodeVar73, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar73, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar73, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar64 = ( nodeVar64 + nodeVar74 );
	nodeVar65 = ( nodeVar65 + ( nodeVar74 * nodeVar74 ) );

	if ( ( nodeVar72.w < 1000.0 ) ) {

		nodeVar66 = ( nodeVar66 + nodeVar72.w );
		nodeVar67 = ( nodeVar67 + 1.0 );
		nodeVar75 = ( nodeVar72.w - nodeVar68 );
		nodeVar68 = ( nodeVar68 + ( nodeVar75 / nodeVar67 ) );
		nodeVar69 = ( nodeVar69 + ( nodeVar75 * ( nodeVar72.w - nodeVar68 ) ) );
		

	}

	nodeVar76 = textureLoad( nodeUniform1, ( nodeVar2 + vec2<i32>( -1, 1 ) ), u32( 0u ) );
	nodeVar77 = max( nodeVar76, vec4<f32>( 0.0 ) );
	nodeVar78 = ( nodeVar77.xyz / vec3<f32>( ( ( ( dot( nodeVar77.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 ) ) );
	nodeVar79 = vec3<f32>( dot( nodeVar78, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar78, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar78, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar64 = ( nodeVar64 + nodeVar79 );
	nodeVar65 = ( nodeVar65 + ( nodeVar79 * nodeVar79 ) );

	if ( ( nodeVar77.w < 1000.0 ) ) {

		nodeVar66 = ( nodeVar66 + nodeVar77.w );
		nodeVar67 = ( nodeVar67 + 1.0 );
		nodeVar80 = ( nodeVar77.w - nodeVar68 );
		nodeVar68 = ( nodeVar68 + ( nodeVar80 / nodeVar67 ) );
		nodeVar69 = ( nodeVar69 + ( nodeVar80 * ( nodeVar77.w - nodeVar68 ) ) );
		

	}

	nodeVar81 = textureLoad( nodeUniform1, ( nodeVar2 + vec2<i32>( 1, -1 ) ), u32( 0u ) );
	nodeVar82 = max( nodeVar81, vec4<f32>( 0.0 ) );
	nodeVar83 = ( nodeVar82.xyz / vec3<f32>( ( ( ( dot( nodeVar82.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 ) ) );
	nodeVar84 = vec3<f32>( dot( nodeVar83, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar83, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar83, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar64 = ( nodeVar64 + nodeVar84 );
	nodeVar65 = ( nodeVar65 + ( nodeVar84 * nodeVar84 ) );

	if ( ( nodeVar82.w < 1000.0 ) ) {

		nodeVar66 = ( nodeVar66 + nodeVar82.w );
		nodeVar67 = ( nodeVar67 + 1.0 );
		nodeVar85 = ( nodeVar82.w - nodeVar68 );
		nodeVar68 = ( nodeVar68 + ( nodeVar85 / nodeVar67 ) );
		nodeVar69 = ( nodeVar69 + ( nodeVar85 * ( nodeVar82.w - nodeVar68 ) ) );
		

	}

	nodeVar86 = textureLoad( nodeUniform1, ( nodeVar2 + vec2<i32>( 1, 1 ) ), u32( 0u ) );
	nodeVar87 = max( nodeVar86, vec4<f32>( 0.0 ) );
	nodeVar88 = ( nodeVar87.xyz / vec3<f32>( ( ( ( dot( nodeVar87.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 ) ) );
	nodeVar89 = vec3<f32>( dot( nodeVar88, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar88, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar88, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar64 = ( nodeVar64 + nodeVar89 );
	nodeVar65 = ( nodeVar65 + ( nodeVar89 * nodeVar89 ) );

	if ( ( nodeVar87.w < 1000.0 ) ) {

		nodeVar66 = ( nodeVar66 + nodeVar87.w );
		nodeVar67 = ( nodeVar67 + 1.0 );
		nodeVar90 = ( nodeVar87.w - nodeVar68 );
		nodeVar68 = ( nodeVar68 + ( nodeVar90 / nodeVar67 ) );
		nodeVar69 = ( nodeVar69 + ( nodeVar90 * ( nodeVar87.w - nodeVar68 ) ) );
		

	}

	nodeVar91 = textureLoad( nodeUniform1, ( nodeVar2 + vec2<i32>( 1, 0 ) ), u32( 0u ) );
	nodeVar92 = max( nodeVar91, vec4<f32>( 0.0 ) );
	nodeVar93 = ( nodeVar92.xyz / vec3<f32>( ( ( ( dot( nodeVar92.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 ) ) );
	nodeVar94 = vec3<f32>( dot( nodeVar93, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar93, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar93, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar64 = ( nodeVar64 + nodeVar94 );
	nodeVar65 = ( nodeVar65 + ( nodeVar94 * nodeVar94 ) );

	if ( ( nodeVar92.w < 1000.0 ) ) {

		nodeVar66 = ( nodeVar66 + nodeVar92.w );
		nodeVar67 = ( nodeVar67 + 1.0 );
		nodeVar95 = ( nodeVar92.w - nodeVar68 );
		nodeVar68 = ( nodeVar68 + ( nodeVar95 / nodeVar67 ) );
		nodeVar69 = ( nodeVar69 + ( nodeVar95 * ( nodeVar92.w - nodeVar68 ) ) );
		

	}

	nodeVar96 = textureLoad( nodeUniform1, ( nodeVar2 + vec2<i32>( 0, -1 ) ), u32( 0u ) );
	nodeVar97 = max( nodeVar96, vec4<f32>( 0.0 ) );
	nodeVar98 = ( nodeVar97.xyz / vec3<f32>( ( ( ( dot( nodeVar97.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 ) ) );
	nodeVar99 = vec3<f32>( dot( nodeVar98, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar98, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar98, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar64 = ( nodeVar64 + nodeVar99 );
	nodeVar65 = ( nodeVar65 + ( nodeVar99 * nodeVar99 ) );

	if ( ( nodeVar97.w < 1000.0 ) ) {

		nodeVar66 = ( nodeVar66 + nodeVar97.w );
		nodeVar67 = ( nodeVar67 + 1.0 );
		nodeVar100 = ( nodeVar97.w - nodeVar68 );
		nodeVar68 = ( nodeVar68 + ( nodeVar100 / nodeVar67 ) );
		nodeVar69 = ( nodeVar69 + ( nodeVar100 * ( nodeVar97.w - nodeVar68 ) ) );
		

	}

	nodeVar101 = textureLoad( nodeUniform1, ( nodeVar2 + vec2<i32>( 0, 1 ) ), u32( 0u ) );
	nodeVar102 = max( nodeVar101, vec4<f32>( 0.0 ) );
	nodeVar103 = ( nodeVar102.xyz / vec3<f32>( ( ( ( dot( nodeVar102.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 ) ) );
	nodeVar104 = vec3<f32>( dot( nodeVar103, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar103, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar103, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar64 = ( nodeVar64 + nodeVar104 );
	nodeVar65 = ( nodeVar65 + ( nodeVar104 * nodeVar104 ) );

	if ( ( nodeVar102.w < 1000.0 ) ) {

		nodeVar66 = ( nodeVar66 + nodeVar102.w );
		nodeVar67 = ( nodeVar67 + 1.0 );
		nodeVar105 = ( nodeVar102.w - nodeVar68 );
		nodeVar68 = ( nodeVar68 + ( nodeVar105 / nodeVar67 ) );
		nodeVar69 = ( nodeVar69 + ( nodeVar105 * ( nodeVar102.w - nodeVar68 ) ) );
		

	}

	nodeVar106 = textureLoad( nodeUniform1, ( nodeVar2 + vec2<i32>( -1, 0 ) ), u32( 0u ) );
	nodeVar107 = max( nodeVar106, vec4<f32>( 0.0 ) );
	nodeVar108 = ( nodeVar107.xyz / vec3<f32>( ( ( ( dot( nodeVar107.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 ) ) );
	nodeVar109 = vec3<f32>( dot( nodeVar108, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar108, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar108, vec3<f32>( -0.25, 0.5, -0.25 ) ) );
	nodeVar64 = ( nodeVar64 + nodeVar109 );
	nodeVar65 = ( nodeVar65 + ( nodeVar109 * nodeVar109 ) );

	if ( ( nodeVar107.w < 1000.0 ) ) {

		nodeVar66 = ( nodeVar66 + nodeVar107.w );
		nodeVar67 = ( nodeVar67 + 1.0 );
		nodeVar110 = ( nodeVar107.w - nodeVar68 );
		nodeVar68 = ( nodeVar68 + ( nodeVar110 / nodeVar67 ) );
		nodeVar69 = ( nodeVar69 + ( nodeVar110 * ( nodeVar107.w - nodeVar68 ) ) );
		

	}

	nodeVar112 = ( nodeVar64 / vec3<f32>( 9.0 ) );

	if ( ( nodeVar67 < 0.5 ) ) {

		nodeVar113 = 10000.0;

	} else {

		nodeVar113 = ( nodeVar66 / max( nodeVar67, 0.0001 ) );

	}

	nodeVar111 = StructType2( nodeVar112, sqrt( max( ( ( nodeVar65 / vec3<f32>( 9.0 ) ) - ( nodeVar112 * nodeVar112 ) ), vec3<f32>( 0.0 ) ) ), nodeVar113, ( 1.0 - ( nodeVar67 / 9.0 ) ), max( sqrt( ( nodeVar69 / max( nodeVar67, 1.0 ) ) ), 0.001 ) );
	nodeVar114 = projectWorldToUV( ( nodeVar9 + ( nodeVar61 * vec3<f32>( nodeVar111.rayLength ) ) ), object.nodeUniform16, object.nodeUniform28 );
	nodeVar115 = nodeVar114;
	nodeVar116 = clamp( ( length( ( nodeVar11 * object.nodeUniform2 ) ) / 128.0 ), 0.0, 1.0 );
	nodeVar111.stdDevRayLength = ( 1.0 - min( ( ( nodeVar111.stdDevRayLength * min( ( nodeVar116 * 100.0 ), 1.0 ) ) * 3.5 ), 1.0 ) );
	nodeVar117 = ( ( nodeVar115 * object.nodeUniform2 ) - vec2<f32>( 0.5 ) );
	nodeVar120 = ( vec2<i32>( floor( nodeVar117 ) ) + vec2<i32>( 0, 0 ) );
	nodeVar121 = ( ( vec2<f32>( nodeVar120 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar122 = textureLoad( nodeUniform10, vec2<i32>( ( object.nodeUniform29 * vec3<f32>( vec2<f32>( nodeVar120 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar123 = ( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar121.x, ( 1.0 - nodeVar121.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar122 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar121.x, ( 1.0 - nodeVar121.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar122 ), 1.0 ) ).w ) );
	nodeVar124 = abs( dot( ( ( object.nodeUniform8 * vec4<f32>( nodeVar123, 1.0 ) ).xyz - nodeVar9 ), nodeVar7 ) );
	nodeVar124 = ( nodeVar124 / abs( nodeVar123.z ) );
	nodeVar126 = textureLoad( nodeUniform12, vec2<i32>( ( object.nodeUniform30 * vec3<f32>( vec2<f32>( nodeVar120 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar127 = fract( nodeVar117 );
	nodeVar128 = textureLoad( nodeUniform14, vec2<i32>( ( object.nodeUniform31 * vec3<f32>( vec2<f32>( nodeVar120 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar129 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar124 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar128.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform16 ).xyz ), nodeVar7 ) ) );
	nodeVar130 = ( ( ( 1.0 - nodeVar127.x ) * ( 1.0 - nodeVar127.y ) ) * nodeVar129 );
	nodeVar125 = StructType0( ( max( nodeVar126, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeVar130 ) ), nodeVar130, nodeVar129 );
	nodeVar131 = ( vec2<i32>( floor( nodeVar117 ) ) + vec2<i32>( 1, 0 ) );
	nodeVar132 = ( ( vec2<f32>( nodeVar131 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar133 = textureLoad( nodeUniform10, vec2<i32>( ( object.nodeUniform32 * vec3<f32>( vec2<f32>( nodeVar131 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar134 = ( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar132.x, ( 1.0 - nodeVar132.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar133 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar132.x, ( 1.0 - nodeVar132.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar133 ), 1.0 ) ).w ) );
	nodeVar135 = abs( dot( ( ( object.nodeUniform8 * vec4<f32>( nodeVar134, 1.0 ) ).xyz - nodeVar9 ), nodeVar7 ) );
	nodeVar135 = ( nodeVar135 / abs( nodeVar134.z ) );
	nodeVar137 = textureLoad( nodeUniform12, vec2<i32>( ( object.nodeUniform33 * vec3<f32>( vec2<f32>( nodeVar131 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar138 = textureLoad( nodeUniform14, vec2<i32>( ( object.nodeUniform34 * vec3<f32>( vec2<f32>( nodeVar131 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar139 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar135 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar138.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform16 ).xyz ), nodeVar7 ) ) );
	nodeVar140 = ( ( nodeVar127.x * ( 1.0 - nodeVar127.y ) ) * nodeVar139 );
	nodeVar136 = StructType0( ( max( nodeVar137, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeVar140 ) ), nodeVar140, nodeVar139 );
	nodeVar141 = ( vec2<i32>( floor( nodeVar117 ) ) + vec2<i32>( 0, 1 ) );
	nodeVar142 = ( ( vec2<f32>( nodeVar141 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar143 = textureLoad( nodeUniform10, vec2<i32>( ( object.nodeUniform35 * vec3<f32>( vec2<f32>( nodeVar141 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar144 = ( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar142.x, ( 1.0 - nodeVar142.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar143 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar142.x, ( 1.0 - nodeVar142.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar143 ), 1.0 ) ).w ) );
	nodeVar145 = abs( dot( ( ( object.nodeUniform8 * vec4<f32>( nodeVar144, 1.0 ) ).xyz - nodeVar9 ), nodeVar7 ) );
	nodeVar145 = ( nodeVar145 / abs( nodeVar144.z ) );
	nodeVar147 = textureLoad( nodeUniform12, vec2<i32>( ( object.nodeUniform36 * vec3<f32>( vec2<f32>( nodeVar141 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar148 = textureLoad( nodeUniform14, vec2<i32>( ( object.nodeUniform37 * vec3<f32>( vec2<f32>( nodeVar141 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar149 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar145 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar148.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform16 ).xyz ), nodeVar7 ) ) );
	nodeVar150 = ( ( ( 1.0 - nodeVar127.x ) * nodeVar127.y ) * nodeVar149 );
	nodeVar146 = StructType0( ( max( nodeVar147, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeVar150 ) ), nodeVar150, nodeVar149 );
	nodeVar151 = ( vec2<i32>( floor( nodeVar117 ) ) + vec2<i32>( 1, 1 ) );
	nodeVar152 = ( ( vec2<f32>( nodeVar151 ) + vec2<f32>( 0.5 ) ) / object.nodeUniform2 );
	nodeVar153 = textureLoad( nodeUniform10, vec2<i32>( ( object.nodeUniform38 * vec3<f32>( vec2<f32>( nodeVar151 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar154 = ( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar152.x, ( 1.0 - nodeVar152.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar153 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform9 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar152.x, ( 1.0 - nodeVar152.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar153 ), 1.0 ) ).w ) );
	nodeVar155 = abs( dot( ( ( object.nodeUniform8 * vec4<f32>( nodeVar154, 1.0 ) ).xyz - nodeVar9 ), nodeVar7 ) );
	nodeVar155 = ( nodeVar155 / abs( nodeVar154.z ) );
	nodeVar157 = textureLoad( nodeUniform12, vec2<i32>( ( object.nodeUniform39 * vec3<f32>( vec2<f32>( nodeVar151 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar158 = textureLoad( nodeUniform14, vec2<i32>( ( object.nodeUniform40 * vec3<f32>( vec2<f32>( nodeVar151 ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar159 = ( ( 1.0 - smoothstep( 0.0, 0.01, nodeVar155 ) ) * smoothstep( 0.95, 0.999, dot( normalize( ( vec4<f32>( ( ( nodeVar158.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform16 ).xyz ), nodeVar7 ) ) );
	nodeVar160 = ( ( nodeVar127.x * nodeVar127.y ) * nodeVar159 );
	nodeVar156 = StructType0( ( max( nodeVar157, vec4<f32>( 0.0 ) ) * vec4<f32>( nodeVar160 ) ), nodeVar160, nodeVar159 );
	nodeVar161 = ( ( ( nodeVar125.weight + nodeVar136.weight ) + nodeVar146.weight ) + nodeVar156.weight );

	if ( ( nodeVar161 > 0.01 ) ) {

		nodeVar119 = ( ( ( ( nodeVar125.color + nodeVar136.color ) + nodeVar146.color ) + nodeVar156.color ) / vec4<f32>( nodeVar161 ) );

	} else {

		nodeVar119 = vec4<f32>( nodeVar4.xyz, 1.0 );

	}

	nodeVar118 = StructType1( nodeVar119, max( max( nodeVar125.confidence, nodeVar136.confidence ), max( nodeVar146.confidence, nodeVar156.confidence ) ), min( min( nodeVar125.confidence, nodeVar136.confidence ), min( nodeVar146.confidence, nodeVar156.confidence ) ) );
	nodeVar163 = bool( object.nodeUniform41 );

	if ( ( ( ( ( ( nodeVar115.x >= 0.0 ) && ( nodeVar115.x <= 1.0 ) ) && ( nodeVar115.y >= 0.0 ) ) && ( nodeVar115.y <= 1.0 ) ) && nodeVar163 ) ) {

		nodeVar162 = nodeVar118.tapConfidence;

	} else {

		nodeVar162 = 0.0;

	}

	let nodeConst0 = ( ( ( nodeVar118.minConfidence * nodeVar111.stdDevRayLength ) * ( 1.0 - clamp( ( length( fwidth( nodeVar7 ) ) * 50.0 ), 0.0, 1.0 ) ) ) * nodeVar162 );
	nodeVar164 = ( nodeConst0 * ( 1.0 - ( nodeVar111.envProbability * nodeVar111.envProbability ) ) );

	if ( ( ( ( ( nodeVar12.x >= 0.0 ) && ( nodeVar12.x <= 1.0 ) ) && ( nodeVar12.y >= 0.0 ) ) && ( nodeVar12.y <= 1.0 ) ) ) {

		nodeVar165 = nodeVar14.tapConfidence;

	} else {

		nodeVar165 = 0.0;

	}

	nodeVar166 = ( ( 1.0 - nodeVar164 ) * nodeVar165 );
	nodeVar167 = max( ( nodeVar164 + nodeVar166 ), 0.000001 );
	nodeVar168 = vec4<f32>( ( ( ( max( nodeVar118.color.xyz, vec3<f32>( 0.0 ) ) * vec3<f32>( nodeVar164 ) ) + ( max( nodeVar14.color.xyz, vec3<f32>( 0.0 ) ) * vec3<f32>( nodeVar166 ) ) ) / vec3<f32>( nodeVar167 ) ), nodeVar14.color.w );

	if ( ( length( nodeVar168.xyz ) < 0.000001 ) ) {

		nodeVar168 = vec4<f32>( nodeVar4.xyz, 1.0 );
		

	}

	nodeVar58 = nodeVar168;
	nodeVar59 = ( ( ( nodeVar162 * nodeVar164 ) + ( nodeVar165 * nodeVar166 ) ) / nodeVar167 );
	nodeVar60 = nodeConst0;
	nodeVar169 = ( dpdx( nodeVar12 ) * object.nodeUniform2 );
	nodeVar170 = ( - dpdy( nodeVar12 ) * object.nodeUniform2 );
	nodeVar171 = ( dot( nodeVar169, nodeVar169 ) + dot( nodeVar170, nodeVar170 ) );
	nodeVar172 = ( ( nodeVar169.x * nodeVar170.y ) - ( nodeVar169.y * nodeVar170.x ) );
	nodeVar173 = clamp( sqrt( max( ( ( nodeVar171 * 0.5 ) - sqrt( max( ( ( ( nodeVar171 * nodeVar171 ) * 0.25 ) - ( nodeVar172 * nodeVar172 ) ), 0.0 ) ) ), 0.0 ) ), 0.0, 1.0 );
	nodeVar59 = ( nodeVar59 * pow( nodeVar173, 2.0 ) );
	nodeVar174 = nodeVar58.xyz;
	nodeVar175 = ( ( ( dot( nodeVar174, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * object.nodeUniform27 ) * 10.0 ) + 1.0 );
	nodeVar176 = ( nodeVar174 / vec3<f32>( nodeVar175 ) );
	nodeVar177 = ( 1.0 - nodeVar116 );
	nodeVar178 = ( nodeVar111.stdColor * vec3<f32>( mix( 0.5, 1.0, ( nodeVar177 * nodeVar177 ) ) ) );
	nodeVar179 = clipToAABB( vec3<f32>( dot( nodeVar176, vec3<f32>( 0.25, 0.5, 0.25 ) ), dot( nodeVar176, vec3<f32>( 0.5, 0.0, -0.5 ) ), dot( nodeVar176, vec3<f32>( -0.25, 0.5, -0.25 ) ) ), ( nodeVar111.mean - nodeVar178 ), ( nodeVar111.mean + nodeVar178 ) );
	nodeVar180 = ( vec3<f32>( ( ( nodeVar179.x + nodeVar179.y ) - nodeVar179.z ), ( nodeVar179.x + nodeVar179.z ), ( ( nodeVar179.x - nodeVar179.y ) - nodeVar179.z ) ) * vec3<f32>( nodeVar175 ) );
	nodeVar181 = ( ( object.nodeUniform42 * max( min( ( nodeVar116 * 10.0 ), 1.0 ), 0.25 ) ) * ( 1.0 + clamp( ( ( 1.0 - nodeVar173 ) + ( 1.0 - nodeVar60 ) ), 0.0, 1.0 ) ) );
	nodeVar182 = mix( nodeVar58.xyz, nodeVar180, nodeVar181 );
	nodeVar58.x = nodeVar182[ 0 ];
	nodeVar58.y = nodeVar182[ 1 ];
	nodeVar58.z = nodeVar182[ 2 ];
	nodeVar59 = ( nodeVar59 * exp( ( - ( ( length( ( nodeVar58.xyz - nodeVar180 ) ) * nodeVar181 ) * 30.0 ) ) ) );
	nodeVar59 = ( nodeVar59 * mix( 1.0, ( ( nodeVar60 * 0.05 ) + 0.95 ), clamp( ( nodeVar116 * 100.0 ), 0.0, 1.0 ) ) );

	if ( ( nodeVar59 < 0.000001 ) ) {

		nodeVar58 = vec4<f32>( nodeVar4.xyz, 1.0 );
		

	}

	nodeVar183 = min( ( ( ( 1.0 / max( nodeVar58.w, 0.000001 ) ) * nodeVar59 ) + 1.0 ), object.nodeUniform43 );

	if ( ( length( nodeVar4.xyz ) < 0.000001 ) ) {

		nodeVar183 = max( ( nodeVar183 - 1.0 ), 1.0 );
		

	}


	// result

	output.color = vec4<f32>( nodeVar58.xyz, ( 1.0 / nodeVar183 ) );

	return output;

}
