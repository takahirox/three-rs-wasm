// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform5 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform28_sampler : sampler_comparison;
@binding( 12 ) @group( 1 ) var nodeUniform28 : texture_depth_2d;
@binding( 13 ) @group( 1 ) var nodeUniform33_sampler : sampler;
@binding( 14 ) @group( 1 ) var nodeUniform33 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform19 : mat3x3<f32>,
	nodeUniform20 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform25 : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform31 : f32,
	nodeUniform6 : vec2<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> Metalness : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar4 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar7 : f32;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec2<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec4<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec4<f32>;

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
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_bitangentView : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 6 ) nodeVarying10 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32>,
	@builtin( front_facing ) isFront : bool ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );

	if ( ( DiffuseColor.w <= object.nodeUniform4 ) ) {

		discard;
		

	}

	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform5, nodeUniform5_sampler, ( fragCoord.xy / render.nodeUniform6 ) ).x;
	nodeVar2 = max( nodeVar1, 0.001 );
	AmbientOcclusion = nodeVar2;
	nodeVar3 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar3.z );
	nodeVar4 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform11 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar6 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform10 * nodeVar4.y ), 0.0525 ) + max( max( nodeVar6.x, nodeVar6.y ), nodeVar6.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar3.z ) ) ) );
	EmissiveColor = ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) );
	nodeVar7 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_tangentView = ( normalize( v_tangentView ) * vec3<f32>( nodeVar7 ) );
	NORMAL_bitangentView = ( normalize( NORMAL_v_bitangentView ) * vec3<f32>( nodeVar7 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeVar7 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar9 = textureSample( nodeUniform18, nodeUniform18_sampler, ( object.nodeUniform19 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	nodeVar10 = ( ( nodeVar9 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar10.xy * object.nodeUniform20 ), nodeVar10.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar11 = dot( normalView, positionViewDirection );
	nodeVar12 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( nodeVar11, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar12;
	nodeVar13 = ( dfg.x + dfg.y );
	nodeVar14 = ( 1.0 / nodeVar13 );
	nodeVar15 = nodeVar14;
	nodeVar16 = ( nodeVar15 - 1.0 );
	nodeVar17 = ( SpecularColorBlended * vec3<f32>( nodeVar16 ) );
	nodeVar18 = ( nodeVar17 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar18;
	nodeVar19 = ( render.nodeUniform22 - render.nodeUniform23 );
	nodeVar20 = vec4<f32>( nodeVar19, 0.0 );
	nodeVar21 = ( render.cameraViewMatrix * nodeVar20 );
	nodeVar22 = normalize( nodeVar21.xyz );
	nodeVar23 = nodeVar22;
	nodeVar24 = dot( normalView, nodeVar23 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar26 = ( render.nodeUniform25 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform26 ) ) ), 1.0 ) );
	nodeVar27 = ( nodeVar26.xyz / vec3<f32>( nodeVar26.w ) );
	nodeVar28 = vec3<f32>( nodeVar27.x, ( 1.0 - nodeVar27.y ), ( nodeVar27.z + render.nodeUniform27 ) );

	if ( ( ( ( ( ( nodeVar28.x >= 0.0 ) && ( nodeVar28.x <= 1.0 ) ) && ( nodeVar28.y >= 0.0 ) ) && ( nodeVar28.y <= 1.0 ) ) && ( nodeVar28.z <= 1.0 ) ) ) {

		nodeVar29 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar30 = ( render.nodeUniform29 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform30 ).x );
		nodeVar31 = ( nodeVar28.xy + ( vogelDiskSample( 0, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
		nodeVar32 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar31, nodeVar28.z );
		nodeVar33 = ( nodeVar28.xy + ( vogelDiskSample( 1, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
		nodeVar34 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar33, nodeVar28.z );
		nodeVar35 = ( nodeVar28.xy + ( vogelDiskSample( 2, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
		nodeVar36 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar35, nodeVar28.z );
		nodeVar37 = ( nodeVar28.xy + ( vogelDiskSample( 3, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
		nodeVar38 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar37, nodeVar28.z );
		nodeVar39 = ( nodeVar28.xy + ( vogelDiskSample( 4, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
		nodeVar40 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar39, nodeVar28.z );
		nodeVar25 = ( ( ( ( ( nodeVar32 + nodeVar34 ) + nodeVar36 ) + nodeVar38 ) + nodeVar40 ) * 0.2 );

	} else {

		nodeVar25 = 1.0;

	}

	nodeVar41 = mix( 1.0, nodeVar25, render.nodeUniform31 );
	nodeVar42 = ( vec3<f32>( clamp( nodeVar24, 0.0, 1.0 ) ) * ( render.nodeUniform24 * vec3<f32>( nodeVar41 ) ) );
	nodeVar43 = nodeVar42;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar44 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar45 = ( nodeVar43 * nodeVar44 );
	nodeVar46 = ( nodeVar23 + positionViewDirection );
	nodeVar47 = normalize( nodeVar46 );
	nodeVar48 = dot( positionViewDirection, nodeVar47 );
	nodeVar49 = clamp( nodeVar48, 0.0, 1.0 );
	nodeVar50 = exp2( ( ( ( nodeVar49 * -5.55473 ) - 6.98316 ) * nodeVar49 ) );
	nodeVar51 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar50 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar50 ) ) );
	nodeVar52 = ( vec3<f32>( 1.0 ) - nodeVar51 );
	nodeVar53 = nodeVar52;
	nodeVar54 = ( nodeVar45 * nodeVar53 );
	nodeVar55 = ( directDiffuse + nodeVar54 );
	directDiffuse = nodeVar55;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar56 = normalize( ( nodeVar23 + positionViewDirection ) );
	nodeVar57 = clamp( dot( positionViewDirection, nodeVar56 ), 0.0, 1.0 );
	nodeVar58 = exp2( ( ( ( nodeVar57 * -5.55473 ) - 6.98316 ) * nodeVar57 ) );
	nodeVar59 = ( Roughness * Roughness );
	nodeVar60 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar58 ) ) ) + vec3<f32>( ( 1.0 * nodeVar58 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar59, clamp( dot( normalView, nodeVar23 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar59, clamp( dot( normalView, nodeVar56 ), 0.0, 1.0 ) ) ) );
	nodeVar61 = ( nodeVar43 * nodeVar60 );
	nodeVar62 = ( nodeVar61 * multiScatteringCompensation );
	nodeVar63 = ( directSpecular + nodeVar62 );
	directSpecular = nodeVar63;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar64 = ( irradiance + render.nodeUniform32 );
	irradiance = nodeVar64;
	ambientOcclusion = 1.0;
	nodeVar65 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar65;
	nodeVar66 = textureSample( nodeUniform33, nodeUniform33_sampler, ( fragCoord.xy / render.nodeUniform6 ) );
	nodeVar67 = ( irradiance + ( nodeVar66.xyz / vec3<f32>( nodeVar2 ) ) );
	irradiance = nodeVar67;
	nodeVar68 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar70 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar71 = ( SpecularF90 * dfg.y );
	nodeVar72 = ( nodeVar70 + vec3<f32>( nodeVar71 ) );
	nodeVar73 = ( nodeVar68 + nodeVar72 );
	nodeVar68 = nodeVar73;
	nodeVar74 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar75 = nodeVar74;
	nodeVar76 = ( nodeVar75 * vec3<f32>( 0.047619 ) );
	nodeVar77 = ( SpecularColor + nodeVar76 );
	nodeVar78 = ( nodeVar72 * nodeVar77 );
	nodeVar79 = ( dfg.x + dfg.y );
	nodeVar80 = ( 1.0 - nodeVar79 );
	nodeVar81 = nodeVar80;
	nodeVar82 = ( vec3<f32>( nodeVar81 ) * nodeVar77 );
	nodeVar83 = ( vec3<f32>( 1.0 ) - nodeVar82 );
	nodeVar84 = nodeVar83;
	nodeVar85 = ( nodeVar78 / nodeVar84 );
	nodeVar86 = ( nodeVar85 * vec3<f32>( nodeVar81 ) );
	nodeVar87 = ( nodeVar69 + nodeVar86 );
	nodeVar69 = nodeVar87;
	nodeVar88 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar89 = ( irradiance * nodeVar88 );
	nodeVar90 = ( nodeVar68 + nodeVar69 );
	nodeVar91 = ( vec3<f32>( 1.0 ) - nodeVar90 );
	nodeVar92 = nodeVar91;
	nodeVar93 = ( nodeVar89 * nodeVar92 );
	nodeVar94 = nodeVar93;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar95 = ( indirectDiffuse + nodeVar94 );
	indirectDiffuse = nodeVar95;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar96 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar97 = ( SpecularF90 * dfg.y );
	nodeVar98 = ( nodeVar96 + vec3<f32>( nodeVar97 ) );
	nodeVar99 = ( singleScatteringDielectric + nodeVar98 );
	singleScatteringDielectric = nodeVar99;
	nodeVar100 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar101 = nodeVar100;
	nodeVar102 = ( nodeVar101 * vec3<f32>( 0.047619 ) );
	nodeVar103 = ( SpecularColor + nodeVar102 );
	nodeVar104 = ( nodeVar98 * nodeVar103 );
	nodeVar105 = ( dfg.x + dfg.y );
	nodeVar106 = ( 1.0 - nodeVar105 );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( vec3<f32>( nodeVar107 ) * nodeVar103 );
	nodeVar109 = ( vec3<f32>( 1.0 ) - nodeVar108 );
	nodeVar110 = nodeVar109;
	nodeVar111 = ( nodeVar104 / nodeVar110 );
	nodeVar112 = ( nodeVar111 * vec3<f32>( nodeVar107 ) );
	nodeVar113 = ( multiScatteringDielectric + nodeVar112 );
	multiScatteringDielectric = nodeVar113;
	nodeVar114 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar115 = ( SpecularF90 * dfg.y );
	nodeVar116 = ( nodeVar114 + vec3<f32>( nodeVar115 ) );
	nodeVar117 = ( singleScatteringMetallic + nodeVar116 );
	singleScatteringMetallic = nodeVar117;
	nodeVar118 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar119 = nodeVar118;
	nodeVar120 = ( nodeVar119 * vec3<f32>( 0.047619 ) );
	nodeVar121 = ( DiffuseColor.xyz + nodeVar120 );
	nodeVar122 = ( nodeVar116 * nodeVar121 );
	nodeVar123 = ( dfg.x + dfg.y );
	nodeVar124 = ( 1.0 - nodeVar123 );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( vec3<f32>( nodeVar125 ) * nodeVar121 );
	nodeVar127 = ( vec3<f32>( 1.0 ) - nodeVar126 );
	nodeVar128 = nodeVar127;
	nodeVar129 = ( nodeVar122 / nodeVar128 );
	nodeVar130 = ( nodeVar129 * vec3<f32>( nodeVar125 ) );
	nodeVar131 = ( multiScatteringMetallic + nodeVar130 );
	multiScatteringMetallic = nodeVar131;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar132 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar133 = ( radiance * nodeVar132 );
	nodeVar134 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar135 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar136 = ( nodeVar134 * nodeVar135 );
	nodeVar137 = ( nodeVar133 + nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar140 = ( vec3<f32>( 1.0 ) - nodeVar139 );
	nodeVar141 = nodeVar140;
	nodeVar142 = ( DiffuseContribution * nodeVar141 );
	nodeVar143 = ( nodeVar142 * nodeVar135 );
	nodeVar144 = nodeVar143;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar145 = ( indirectSpecular + nodeVar138 );
	indirectSpecular = nodeVar145;
	nodeVar146 = ( indirectDiffuse + nodeVar144 );
	indirectDiffuse = nodeVar146;
	nodeVar147 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar147;
	nodeVar148 = dot( normalView, positionViewDirection );
	nodeVar149 = ( clamp( nodeVar148, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar150 = ( Roughness * -16.0 );
	nodeVar151 = ( 1.0 - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( - nodeVar152 );
	nodeVar154 = exp2( nodeVar153 );
	nodeVar155 = pow( nodeVar149, nodeVar154 );
	nodeVar156 = ( 1.0 - nodeVar155 );
	nodeVar157 = nodeVar156;
	nodeVar158 = ( ambientOcclusion - nodeVar157 );
	nodeVar159 = ( indirectSpecular * vec3<f32>( clamp( nodeVar158, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar159;
	nodeVar160 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar160;
	nodeVar161 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar161;
	nodeVar162 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar162;
	nodeVar163 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar163;

	// result

	output.color = nodeVar163;

	return output;

}
