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
@binding( 3 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform21_sampler : sampler_comparison;
@binding( 8 ) @group( 1 ) var nodeUniform21 : texture_depth_2d;
@binding( 9 ) @group( 1 ) var nodeUniform26_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform26 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : f32,
	nodeUniform13 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform24 : f32,
	nodeUniform5 : vec2<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : vec2<f32>
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
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec4<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec4<f32>;

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
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( fragCoord.xy / render.nodeUniform5 ) ).x;
	nodeVar2 = max( nodeVar1, 0.001 );
	AmbientOcclusion = nodeVar2;
	Metalness = object.nodeUniform6;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar3 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform7, 0.0525 ) + max( max( nodeVar3.x, nodeVar3.y ), nodeVar3.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform6 ) ) );
	EmissiveColor = ( object.nodeUniform10 * vec3<f32>( object.nodeUniform11 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = dot( normalView, positionViewDirection );
	nodeVar5 = textureSample( nodeUniform12, nodeUniform12_sampler, vec2<f32>( Roughness, clamp( nodeVar4, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar5;
	nodeVar6 = ( dfg.x + dfg.y );
	nodeVar7 = ( 1.0 / nodeVar6 );
	nodeVar8 = nodeVar7;
	nodeVar9 = ( nodeVar8 - 1.0 );
	nodeVar10 = ( SpecularColorBlended * vec3<f32>( nodeVar9 ) );
	nodeVar11 = ( nodeVar10 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar11;
	nodeVar12 = ( render.nodeUniform15 - render.nodeUniform16 );
	nodeVar13 = vec4<f32>( nodeVar12, 0.0 );
	nodeVar14 = ( render.cameraViewMatrix * nodeVar13 );
	nodeVar15 = normalize( nodeVar14.xyz );
	nodeVar16 = nodeVar15;
	nodeVar17 = dot( normalView, nodeVar16 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar19 = ( render.nodeUniform18 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform19 ) ) ), 1.0 ) );
	nodeVar20 = ( nodeVar19.xyz / vec3<f32>( nodeVar19.w ) );
	nodeVar21 = vec3<f32>( nodeVar20.x, ( 1.0 - nodeVar20.y ), ( nodeVar20.z + render.nodeUniform20 ) );

	if ( ( ( ( ( ( nodeVar21.x >= 0.0 ) && ( nodeVar21.x <= 1.0 ) ) && ( nodeVar21.y >= 0.0 ) ) && ( nodeVar21.y <= 1.0 ) ) && ( nodeVar21.z <= 1.0 ) ) ) {

		nodeVar22 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar23 = ( render.nodeUniform22 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform23 ).x );
		nodeVar24 = ( nodeVar21.xy + ( vogelDiskSample( 0, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar25 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar24, nodeVar21.z );
		nodeVar26 = ( nodeVar21.xy + ( vogelDiskSample( 1, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar27 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar26, nodeVar21.z );
		nodeVar28 = ( nodeVar21.xy + ( vogelDiskSample( 2, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar29 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar28, nodeVar21.z );
		nodeVar30 = ( nodeVar21.xy + ( vogelDiskSample( 3, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar31 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar30, nodeVar21.z );
		nodeVar32 = ( nodeVar21.xy + ( vogelDiskSample( 4, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar33 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar32, nodeVar21.z );
		nodeVar18 = ( ( ( ( ( nodeVar25 + nodeVar27 ) + nodeVar29 ) + nodeVar31 ) + nodeVar33 ) * 0.2 );

	} else {

		nodeVar18 = 1.0;

	}

	nodeVar34 = mix( 1.0, nodeVar18, render.nodeUniform24 );
	nodeVar35 = ( vec3<f32>( clamp( nodeVar17, 0.0, 1.0 ) ) * ( render.nodeUniform17 * vec3<f32>( nodeVar34 ) ) );
	nodeVar36 = nodeVar35;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar37 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar38 = ( nodeVar36 * nodeVar37 );
	nodeVar39 = ( nodeVar16 + positionViewDirection );
	nodeVar40 = normalize( nodeVar39 );
	nodeVar41 = dot( positionViewDirection, nodeVar40 );
	nodeVar42 = clamp( nodeVar41, 0.0, 1.0 );
	nodeVar43 = exp2( ( ( ( nodeVar42 * -5.55473 ) - 6.98316 ) * nodeVar42 ) );
	nodeVar44 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar43 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar43 ) ) );
	nodeVar45 = ( vec3<f32>( 1.0 ) - nodeVar44 );
	nodeVar46 = nodeVar45;
	nodeVar47 = ( nodeVar38 * nodeVar46 );
	nodeVar48 = ( directDiffuse + nodeVar47 );
	directDiffuse = nodeVar48;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar49 = normalize( ( nodeVar16 + positionViewDirection ) );
	nodeVar50 = clamp( dot( positionViewDirection, nodeVar49 ), 0.0, 1.0 );
	nodeVar51 = exp2( ( ( ( nodeVar50 * -5.55473 ) - 6.98316 ) * nodeVar50 ) );
	nodeVar52 = ( Roughness * Roughness );
	nodeVar53 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar51 ) ) ) + vec3<f32>( ( 1.0 * nodeVar51 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar52, clamp( dot( normalView, nodeVar16 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar52, clamp( dot( normalView, nodeVar49 ), 0.0, 1.0 ) ) ) );
	nodeVar54 = ( nodeVar36 * nodeVar53 );
	nodeVar55 = ( nodeVar54 * multiScatteringCompensation );
	nodeVar56 = ( directSpecular + nodeVar55 );
	directSpecular = nodeVar56;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar57 = ( irradiance + render.nodeUniform25 );
	irradiance = nodeVar57;
	ambientOcclusion = 1.0;
	nodeVar58 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar58;
	nodeVar59 = textureSample( nodeUniform26, nodeUniform26_sampler, ( fragCoord.xy / render.nodeUniform5 ) );
	nodeVar60 = ( irradiance + ( nodeVar59.xyz / vec3<f32>( nodeVar2 ) ) );
	irradiance = nodeVar60;
	nodeVar61 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar62 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar63 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar64 = ( SpecularF90 * dfg.y );
	nodeVar65 = ( nodeVar63 + vec3<f32>( nodeVar64 ) );
	nodeVar66 = ( nodeVar61 + nodeVar65 );
	nodeVar61 = nodeVar66;
	nodeVar67 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar68 = nodeVar67;
	nodeVar69 = ( nodeVar68 * vec3<f32>( 0.047619 ) );
	nodeVar70 = ( SpecularColor + nodeVar69 );
	nodeVar71 = ( nodeVar65 * nodeVar70 );
	nodeVar72 = ( dfg.x + dfg.y );
	nodeVar73 = ( 1.0 - nodeVar72 );
	nodeVar74 = nodeVar73;
	nodeVar75 = ( vec3<f32>( nodeVar74 ) * nodeVar70 );
	nodeVar76 = ( vec3<f32>( 1.0 ) - nodeVar75 );
	nodeVar77 = nodeVar76;
	nodeVar78 = ( nodeVar71 / nodeVar77 );
	nodeVar79 = ( nodeVar78 * vec3<f32>( nodeVar74 ) );
	nodeVar80 = ( nodeVar62 + nodeVar79 );
	nodeVar62 = nodeVar80;
	nodeVar81 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar82 = ( irradiance * nodeVar81 );
	nodeVar83 = ( nodeVar61 + nodeVar62 );
	nodeVar84 = ( vec3<f32>( 1.0 ) - nodeVar83 );
	nodeVar85 = nodeVar84;
	nodeVar86 = ( nodeVar82 * nodeVar85 );
	nodeVar87 = nodeVar86;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar88 = ( indirectDiffuse + nodeVar87 );
	indirectDiffuse = nodeVar88;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar89 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar90 = ( SpecularF90 * dfg.y );
	nodeVar91 = ( nodeVar89 + vec3<f32>( nodeVar90 ) );
	nodeVar92 = ( singleScatteringDielectric + nodeVar91 );
	singleScatteringDielectric = nodeVar92;
	nodeVar93 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar94 = nodeVar93;
	nodeVar95 = ( nodeVar94 * vec3<f32>( 0.047619 ) );
	nodeVar96 = ( SpecularColor + nodeVar95 );
	nodeVar97 = ( nodeVar91 * nodeVar96 );
	nodeVar98 = ( dfg.x + dfg.y );
	nodeVar99 = ( 1.0 - nodeVar98 );
	nodeVar100 = nodeVar99;
	nodeVar101 = ( vec3<f32>( nodeVar100 ) * nodeVar96 );
	nodeVar102 = ( vec3<f32>( 1.0 ) - nodeVar101 );
	nodeVar103 = nodeVar102;
	nodeVar104 = ( nodeVar97 / nodeVar103 );
	nodeVar105 = ( nodeVar104 * vec3<f32>( nodeVar100 ) );
	nodeVar106 = ( multiScatteringDielectric + nodeVar105 );
	multiScatteringDielectric = nodeVar106;
	nodeVar107 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar108 = ( SpecularF90 * dfg.y );
	nodeVar109 = ( nodeVar107 + vec3<f32>( nodeVar108 ) );
	nodeVar110 = ( singleScatteringMetallic + nodeVar109 );
	singleScatteringMetallic = nodeVar110;
	nodeVar111 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( nodeVar112 * vec3<f32>( 0.047619 ) );
	nodeVar114 = ( DiffuseColor.xyz + nodeVar113 );
	nodeVar115 = ( nodeVar109 * nodeVar114 );
	nodeVar116 = ( dfg.x + dfg.y );
	nodeVar117 = ( 1.0 - nodeVar116 );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( vec3<f32>( nodeVar118 ) * nodeVar114 );
	nodeVar120 = ( vec3<f32>( 1.0 ) - nodeVar119 );
	nodeVar121 = nodeVar120;
	nodeVar122 = ( nodeVar115 / nodeVar121 );
	nodeVar123 = ( nodeVar122 * vec3<f32>( nodeVar118 ) );
	nodeVar124 = ( multiScatteringMetallic + nodeVar123 );
	multiScatteringMetallic = nodeVar124;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar125 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar126 = ( radiance * nodeVar125 );
	nodeVar127 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar128 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar129 = ( nodeVar127 * nodeVar128 );
	nodeVar130 = ( nodeVar126 + nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar133 = ( vec3<f32>( 1.0 ) - nodeVar132 );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( DiffuseContribution * nodeVar134 );
	nodeVar136 = ( nodeVar135 * nodeVar128 );
	nodeVar137 = nodeVar136;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar138 = ( indirectSpecular + nodeVar131 );
	indirectSpecular = nodeVar138;
	nodeVar139 = ( indirectDiffuse + nodeVar137 );
	indirectDiffuse = nodeVar139;
	nodeVar140 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar140;
	nodeVar141 = dot( normalView, positionViewDirection );
	nodeVar142 = ( clamp( nodeVar141, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar143 = ( Roughness * -16.0 );
	nodeVar144 = ( 1.0 - nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( - nodeVar145 );
	nodeVar147 = exp2( nodeVar146 );
	nodeVar148 = pow( nodeVar142, nodeVar147 );
	nodeVar149 = ( 1.0 - nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( ambientOcclusion - nodeVar150 );
	nodeVar152 = ( indirectSpecular * vec3<f32>( clamp( nodeVar151, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar152;
	nodeVar153 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar153;
	nodeVar154 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar154;
	nodeVar155 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar155;
	nodeVar156 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar156;

	// result

	output.color = nodeVar156;

	return output;

}
