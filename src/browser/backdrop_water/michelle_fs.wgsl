// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform21_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform21 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform23_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform23 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform33_sampler : sampler_comparison;
@binding( 12 ) @group( 1 ) var nodeUniform33 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : f32,
	nodeUniform22 : mat4x4<f32>,
	nodeUniform24 : mat3x3<f32>,
	nodeUniform25 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	nodeUniform39 : vec3<f32>,
	nodeUniform37 : vec3<f32>,
	nodeUniform41 : vec3<f32>,
	nodeUniform42 : vec3<f32>,
	nodeUniform40 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform43 : vec3<f32>,
	nodeUniform44 : f32,
	nodeUniform45 : f32,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : vec2<f32>,
	nodeUniform36 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec4<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar7 : f32;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec4<f32>;

// codes
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




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying7 : vec2<f32>,
	@builtin( front_facing ) isFront : bool,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform3, 1.0 ) * nodeVar1 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform11 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform10 * nodeVar3.y ), 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	IOR = object.nodeUniform14;
	nodeVar5 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	nodeVar6 = textureSample( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar5 * nodeVar5 ) ) * ( object.nodeUniform15 * nodeVar6.xyz ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform18 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform18, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	EmissiveColor = ( object.nodeUniform19 * vec3<f32>( object.nodeUniform20 ) );
	nodeVar7 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeVar7 ) );
	nodeVar8 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	nodeVar9 = dpdx( nodeVarying7 );
	nodeVar10 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	nodeVar11 = - dpdy( nodeVarying7 );
	nodeVar12 = ( ( nodeVar8 * vec3<f32>( nodeVar9.x ) ) + ( nodeVar10 * vec3<f32>( nodeVar11.x ) ) );
	nodeVar14 = ( ( nodeVar8 * vec3<f32>( nodeVar9.y ) ) + ( nodeVar10 * vec3<f32>( nodeVar11.y ) ) );
	nodeVar15 = max( dot( nodeVar12, nodeVar12 ), dot( nodeVar14, nodeVar14 ) );

	if ( ( nodeVar15 == 0.0 ) ) {

		nodeVar13 = 0.0;

	} else {

		nodeVar13 = inverseSqrt( nodeVar15 );

	}

	tangentViewFrame = ( nodeVar12 * vec3<f32>( nodeVar13 ) );
	NORMAL_tangentView = ( tangentViewFrame * vec3<f32>( nodeVar7 ) );
	bitangentViewFrame = ( nodeVar14 * nodeVar13 );
	NORMAL_bitangentView = ( bitangentViewFrame * vec3<f32>( nodeVar7 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar16 = textureSample( nodeUniform23, nodeUniform23_sampler, ( object.nodeUniform24 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	nodeVar17 = ( ( nodeVar16 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar17.xy * object.nodeUniform25 ), nodeVar17.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar18 = dot( normalView, positionViewDirection );
	nodeVar19 = textureSample( nodeUniform21, nodeUniform21_sampler, vec2<f32>( Roughness, clamp( nodeVar18, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar19;
	nodeVar20 = ( dfg.x + dfg.y );
	nodeVar21 = ( 1.0 / nodeVar20 );
	nodeVar22 = nodeVar21;
	nodeVar23 = ( nodeVar22 - 1.0 );
	nodeVar24 = ( SpecularColorBlended * vec3<f32>( nodeVar23 ) );
	nodeVar25 = ( nodeVar24 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar25;
	nodeVar26 = ( render.nodeUniform27 - render.nodeUniform28 );
	nodeVar27 = vec4<f32>( nodeVar26, 0.0 );
	nodeVar28 = ( render.cameraViewMatrix * nodeVar27 );
	nodeVar29 = normalize( nodeVar28.xyz );
	nodeVar30 = nodeVar29;
	nodeVar31 = dot( normalView, nodeVar30 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar33 = ( render.nodeUniform30 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform31 ) ) ), 1.0 ) );
	nodeVar34 = ( nodeVar33.xyz / vec3<f32>( nodeVar33.w ) );
	nodeVar35 = vec3<f32>( nodeVar34.x, ( 1.0 - nodeVar34.y ), ( nodeVar34.z + render.nodeUniform32 ) );

	if ( ( ( ( ( ( nodeVar35.x >= 0.0 ) && ( nodeVar35.x <= 1.0 ) ) && ( nodeVar35.y >= 0.0 ) ) && ( nodeVar35.y <= 1.0 ) ) && ( nodeVar35.z <= 1.0 ) ) ) {

		nodeVar36 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar37 = ( render.nodeUniform34 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform35 ).x );
		nodeVar38 = ( nodeVar35.xy + ( vogelDiskSample( 0, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
		nodeVar39 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar38, nodeVar35.z );
		nodeVar40 = ( nodeVar35.xy + ( vogelDiskSample( 1, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
		nodeVar41 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar40, nodeVar35.z );
		nodeVar42 = ( nodeVar35.xy + ( vogelDiskSample( 2, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
		nodeVar43 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar42, nodeVar35.z );
		nodeVar44 = ( nodeVar35.xy + ( vogelDiskSample( 3, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
		nodeVar45 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar44, nodeVar35.z );
		nodeVar46 = ( nodeVar35.xy + ( vogelDiskSample( 4, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
		nodeVar47 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar46, nodeVar35.z );
		nodeVar32 = ( ( ( ( ( nodeVar39 + nodeVar41 ) + nodeVar43 ) + nodeVar45 ) + nodeVar47 ) * 0.2 );

	} else {

		nodeVar32 = 1.0;

	}

	nodeVar48 = mix( 1.0, nodeVar32, render.nodeUniform36 );
	nodeVar49 = ( render.nodeUniform29 * vec3<f32>( nodeVar48 ) );
	nodeVar50 = ( vec3<f32>( clamp( nodeVar31, 0.0, 1.0 ) ) * nodeVar49 );
	nodeVar51 = nodeVar50;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar52 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar53 = ( nodeVar51 * nodeVar52 );
	nodeVar54 = ( nodeVar30 + positionViewDirection );
	nodeVar55 = normalize( nodeVar54 );
	nodeVar56 = dot( positionViewDirection, nodeVar55 );
	nodeVar57 = clamp( nodeVar56, 0.0, 1.0 );
	nodeVar58 = exp2( ( ( ( nodeVar57 * -5.55473 ) - 6.98316 ) * nodeVar57 ) );
	nodeVar59 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar58 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar58 ) ) );
	nodeVar60 = ( vec3<f32>( 1.0 ) - nodeVar59 );
	nodeVar61 = nodeVar60;
	nodeVar62 = ( nodeVar53 * nodeVar61 );
	nodeVar63 = ( directDiffuse + nodeVar62 );
	directDiffuse = nodeVar63;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar64 = normalize( ( nodeVar30 + positionViewDirection ) );
	nodeVar65 = clamp( dot( positionViewDirection, nodeVar64 ), 0.0, 1.0 );
	nodeVar66 = exp2( ( ( ( nodeVar65 * -5.55473 ) - 6.98316 ) * nodeVar65 ) );
	nodeVar67 = ( Roughness * Roughness );
	nodeVar68 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar66 ) ) ) + vec3<f32>( ( 1.0 * nodeVar66 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar67, clamp( dot( normalView, nodeVar30 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar67, clamp( dot( normalView, nodeVar64 ), 0.0, 1.0 ) ) ) );
	nodeVar69 = ( nodeVar51 * nodeVar68 );
	nodeVar70 = ( nodeVar69 * multiScatteringCompensation );
	nodeVar71 = ( directSpecular + nodeVar70 );
	directSpecular = nodeVar71;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar72 = dot( normalWorld, normalize( render.nodeUniform39 ) );
	nodeVar73 = ( nodeVar72 * 0.5 );
	nodeVar74 = ( nodeVar73 + 0.5 );
	nodeVar75 = mix( render.nodeUniform37, render.nodeUniform38, nodeVar74 );
	nodeVar76 = ( irradiance + nodeVar75 );
	irradiance = nodeVar76;
	nodeVar77 = dot( normalWorld, normalize( render.nodeUniform42 ) );
	nodeVar78 = ( nodeVar77 * 0.5 );
	nodeVar79 = ( nodeVar78 + 0.5 );
	nodeVar80 = mix( render.nodeUniform40, render.nodeUniform41, nodeVar79 );
	nodeVar81 = ( irradiance + nodeVar80 );
	irradiance = nodeVar81;
	nodeVar82 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar83 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar84 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar85 = ( SpecularF90 * dfg.y );
	nodeVar86 = ( nodeVar84 + vec3<f32>( nodeVar85 ) );
	nodeVar87 = ( nodeVar82 + nodeVar86 );
	nodeVar82 = nodeVar87;
	nodeVar88 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar89 = nodeVar88;
	nodeVar90 = ( nodeVar89 * vec3<f32>( 0.047619 ) );
	nodeVar91 = ( SpecularColor + nodeVar90 );
	nodeVar92 = ( nodeVar86 * nodeVar91 );
	nodeVar93 = ( dfg.x + dfg.y );
	nodeVar94 = ( 1.0 - nodeVar93 );
	nodeVar95 = nodeVar94;
	nodeVar96 = ( vec3<f32>( nodeVar95 ) * nodeVar91 );
	nodeVar97 = ( vec3<f32>( 1.0 ) - nodeVar96 );
	nodeVar98 = nodeVar97;
	nodeVar99 = ( nodeVar92 / nodeVar98 );
	nodeVar100 = ( nodeVar99 * vec3<f32>( nodeVar95 ) );
	nodeVar101 = ( nodeVar83 + nodeVar100 );
	nodeVar83 = nodeVar101;
	nodeVar102 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar103 = ( irradiance * nodeVar102 );
	nodeVar104 = ( nodeVar82 + nodeVar83 );
	nodeVar105 = ( vec3<f32>( 1.0 ) - nodeVar104 );
	nodeVar106 = nodeVar105;
	nodeVar107 = ( nodeVar103 * nodeVar106 );
	nodeVar108 = nodeVar107;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar109 = ( indirectDiffuse + nodeVar108 );
	indirectDiffuse = nodeVar109;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar110 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar111 = ( SpecularF90 * dfg.y );
	nodeVar112 = ( nodeVar110 + vec3<f32>( nodeVar111 ) );
	nodeVar113 = ( singleScatteringDielectric + nodeVar112 );
	singleScatteringDielectric = nodeVar113;
	nodeVar114 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( nodeVar115 * vec3<f32>( 0.047619 ) );
	nodeVar117 = ( SpecularColor + nodeVar116 );
	nodeVar118 = ( nodeVar112 * nodeVar117 );
	nodeVar119 = ( dfg.x + dfg.y );
	nodeVar120 = ( 1.0 - nodeVar119 );
	nodeVar121 = nodeVar120;
	nodeVar122 = ( vec3<f32>( nodeVar121 ) * nodeVar117 );
	nodeVar123 = ( vec3<f32>( 1.0 ) - nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( nodeVar118 / nodeVar124 );
	nodeVar126 = ( nodeVar125 * vec3<f32>( nodeVar121 ) );
	nodeVar127 = ( multiScatteringDielectric + nodeVar126 );
	multiScatteringDielectric = nodeVar127;
	nodeVar128 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar129 = ( SpecularF90 * dfg.y );
	nodeVar130 = ( nodeVar128 + vec3<f32>( nodeVar129 ) );
	nodeVar131 = ( singleScatteringMetallic + nodeVar130 );
	singleScatteringMetallic = nodeVar131;
	nodeVar132 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( nodeVar133 * vec3<f32>( 0.047619 ) );
	nodeVar135 = ( DiffuseColor.xyz + nodeVar134 );
	nodeVar136 = ( nodeVar130 * nodeVar135 );
	nodeVar137 = ( dfg.x + dfg.y );
	nodeVar138 = ( 1.0 - nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( vec3<f32>( nodeVar139 ) * nodeVar135 );
	nodeVar141 = ( vec3<f32>( 1.0 ) - nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( nodeVar136 / nodeVar142 );
	nodeVar144 = ( nodeVar143 * vec3<f32>( nodeVar139 ) );
	nodeVar145 = ( multiScatteringMetallic + nodeVar144 );
	multiScatteringMetallic = nodeVar145;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar146 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar147 = ( radiance * nodeVar146 );
	nodeVar148 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar149 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar150 = ( nodeVar148 * nodeVar149 );
	nodeVar151 = ( nodeVar147 + nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar154 = ( vec3<f32>( 1.0 ) - nodeVar153 );
	nodeVar155 = nodeVar154;
	nodeVar156 = ( DiffuseContribution * nodeVar155 );
	nodeVar157 = ( nodeVar156 * nodeVar149 );
	nodeVar158 = nodeVar157;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar159 = ( indirectSpecular + nodeVar152 );
	indirectSpecular = nodeVar159;
	nodeVar160 = ( indirectDiffuse + nodeVar158 );
	indirectDiffuse = nodeVar160;
	ambientOcclusion = 1.0;
	nodeVar161 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar161;
	nodeVar162 = dot( normalView, positionViewDirection );
	nodeVar163 = ( clamp( nodeVar162, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar164 = ( Roughness * -16.0 );
	nodeVar165 = ( 1.0 - nodeVar164 );
	nodeVar166 = nodeVar165;
	nodeVar167 = ( - nodeVar166 );
	nodeVar168 = exp2( nodeVar167 );
	nodeVar169 = pow( nodeVar163, nodeVar168 );
	nodeVar170 = ( 1.0 - nodeVar169 );
	nodeVar171 = nodeVar170;
	nodeVar172 = ( ambientOcclusion - nodeVar171 );
	nodeVar173 = ( indirectSpecular * vec3<f32>( clamp( nodeVar172, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar173;
	nodeVar174 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar174;
	nodeVar175 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar175;
	nodeVar176 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar176;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar177 = vec4<f32>( mix( Output.xyz, render.nodeUniform43, smoothstep( render.nodeUniform44, render.nodeUniform45, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar177;

	// result

	output.color = nodeVar177;

	return output;

}
