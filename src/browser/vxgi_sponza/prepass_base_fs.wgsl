// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform19_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform19 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform24 : mat4x4<f32>,
	nodeUniform25 : mat4x4<f32>,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform28 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform26 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform22 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar152 : vec4<f32>;
var<private> nodeVar153 : vec4<f32>;
var<private> nodeVar154 : vec2<f32>;

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
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) positionPrevious : vec3<f32>,
	@location( 5 ) nodeVarying8 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying8, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform5, 0.0525 ) + max( max( nodeVar1.x, nodeVar1.y ), nodeVar1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = dot( normalView, positionViewDirection );
	nodeVar3 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( Roughness, clamp( nodeVar2, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	nodeVar4 = ( dfg.x + dfg.y );
	nodeVar5 = ( 1.0 / nodeVar4 );
	nodeVar6 = nodeVar5;
	nodeVar7 = ( nodeVar6 - 1.0 );
	nodeVar8 = ( SpecularColorBlended * vec3<f32>( nodeVar7 ) );
	nodeVar9 = ( nodeVar8 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar9;
	nodeVar10 = ( render.nodeUniform13 - render.nodeUniform14 );
	nodeVar11 = vec4<f32>( nodeVar10, 0.0 );
	nodeVar12 = ( render.cameraViewMatrix * nodeVar11 );
	nodeVar13 = normalize( nodeVar12.xyz );
	nodeVar14 = nodeVar13;
	nodeVar15 = dot( normalView, nodeVar14 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar17 = ( render.nodeUniform16 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform17 ) ) ), 1.0 ) );
	nodeVar18 = ( nodeVar17.xyz / vec3<f32>( nodeVar17.w ) );
	nodeVar19 = vec3<f32>( nodeVar18.x, ( 1.0 - nodeVar18.y ), ( nodeVar18.z + render.nodeUniform18 ) );

	if ( ( ( ( ( ( nodeVar19.x >= 0.0 ) && ( nodeVar19.x <= 1.0 ) ) && ( nodeVar19.y >= 0.0 ) ) && ( nodeVar19.y <= 1.0 ) ) && ( nodeVar19.z <= 1.0 ) ) ) {

		nodeVar20 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar21 = ( render.nodeUniform20 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ).x );
		nodeVar22 = ( nodeVar19.xy + ( vogelDiskSample( 0, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
		nodeVar23 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar22, nodeVar19.z );
		nodeVar24 = ( nodeVar19.xy + ( vogelDiskSample( 1, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
		nodeVar25 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar24, nodeVar19.z );
		nodeVar26 = ( nodeVar19.xy + ( vogelDiskSample( 2, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
		nodeVar27 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar26, nodeVar19.z );
		nodeVar28 = ( nodeVar19.xy + ( vogelDiskSample( 3, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
		nodeVar29 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar28, nodeVar19.z );
		nodeVar30 = ( nodeVar19.xy + ( vogelDiskSample( 4, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
		nodeVar31 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar30, nodeVar19.z );
		nodeVar16 = ( ( ( ( ( nodeVar23 + nodeVar25 ) + nodeVar27 ) + nodeVar29 ) + nodeVar31 ) * 0.2 );

	} else {

		nodeVar16 = 1.0;

	}

	nodeVar32 = mix( 1.0, nodeVar16, render.nodeUniform22 );
	nodeVar33 = ( vec3<f32>( clamp( nodeVar15, 0.0, 1.0 ) ) * ( render.nodeUniform15 * vec3<f32>( nodeVar32 ) ) );
	nodeVar34 = nodeVar33;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar35 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar36 = ( nodeVar34 * nodeVar35 );
	nodeVar37 = ( nodeVar14 + positionViewDirection );
	nodeVar38 = normalize( nodeVar37 );
	nodeVar39 = dot( positionViewDirection, nodeVar38 );
	nodeVar40 = clamp( nodeVar39, 0.0, 1.0 );
	nodeVar41 = exp2( ( ( ( nodeVar40 * -5.55473 ) - 6.98316 ) * nodeVar40 ) );
	nodeVar42 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar41 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar41 ) ) );
	nodeVar43 = ( vec3<f32>( 1.0 ) - nodeVar42 );
	nodeVar44 = nodeVar43;
	nodeVar45 = ( nodeVar36 * nodeVar44 );
	nodeVar46 = ( directDiffuse + nodeVar45 );
	directDiffuse = nodeVar46;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar47 = normalize( ( nodeVar14 + positionViewDirection ) );
	nodeVar48 = clamp( dot( positionViewDirection, nodeVar47 ), 0.0, 1.0 );
	nodeVar49 = exp2( ( ( ( nodeVar48 * -5.55473 ) - 6.98316 ) * nodeVar48 ) );
	nodeVar50 = ( Roughness * Roughness );
	nodeVar51 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar49 ) ) ) + vec3<f32>( ( 1.0 * nodeVar49 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar50, clamp( dot( normalView, nodeVar14 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar50, clamp( dot( normalView, nodeVar47 ), 0.0, 1.0 ) ) ) );
	nodeVar52 = ( nodeVar34 * nodeVar51 );
	nodeVar53 = ( nodeVar52 * multiScatteringCompensation );
	nodeVar54 = ( directSpecular + nodeVar53 );
	directSpecular = nodeVar54;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = ( irradiance + render.nodeUniform23 );
	irradiance = nodeVar55;
	nodeVar56 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar57 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar58 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar59 = ( SpecularF90 * dfg.y );
	nodeVar60 = ( nodeVar58 + vec3<f32>( nodeVar59 ) );
	nodeVar61 = ( nodeVar56 + nodeVar60 );
	nodeVar56 = nodeVar61;
	nodeVar62 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar63 = nodeVar62;
	nodeVar64 = ( nodeVar63 * vec3<f32>( 0.047619 ) );
	nodeVar65 = ( SpecularColor + nodeVar64 );
	nodeVar66 = ( nodeVar60 * nodeVar65 );
	nodeVar67 = ( dfg.x + dfg.y );
	nodeVar68 = ( 1.0 - nodeVar67 );
	nodeVar69 = nodeVar68;
	nodeVar70 = ( vec3<f32>( nodeVar69 ) * nodeVar65 );
	nodeVar71 = ( vec3<f32>( 1.0 ) - nodeVar70 );
	nodeVar72 = nodeVar71;
	nodeVar73 = ( nodeVar66 / nodeVar72 );
	nodeVar74 = ( nodeVar73 * vec3<f32>( nodeVar69 ) );
	nodeVar75 = ( nodeVar57 + nodeVar74 );
	nodeVar57 = nodeVar75;
	nodeVar76 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar77 = ( irradiance * nodeVar76 );
	nodeVar78 = ( nodeVar56 + nodeVar57 );
	nodeVar79 = ( vec3<f32>( 1.0 ) - nodeVar78 );
	nodeVar80 = nodeVar79;
	nodeVar81 = ( nodeVar77 * nodeVar80 );
	nodeVar82 = nodeVar81;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar83 = ( indirectDiffuse + nodeVar82 );
	indirectDiffuse = nodeVar83;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar84 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar85 = ( SpecularF90 * dfg.y );
	nodeVar86 = ( nodeVar84 + vec3<f32>( nodeVar85 ) );
	nodeVar87 = ( singleScatteringDielectric + nodeVar86 );
	singleScatteringDielectric = nodeVar87;
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
	nodeVar101 = ( multiScatteringDielectric + nodeVar100 );
	multiScatteringDielectric = nodeVar101;
	nodeVar102 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar103 = ( SpecularF90 * dfg.y );
	nodeVar104 = ( nodeVar102 + vec3<f32>( nodeVar103 ) );
	nodeVar105 = ( singleScatteringMetallic + nodeVar104 );
	singleScatteringMetallic = nodeVar105;
	nodeVar106 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( nodeVar107 * vec3<f32>( 0.047619 ) );
	nodeVar109 = ( DiffuseColor.xyz + nodeVar108 );
	nodeVar110 = ( nodeVar104 * nodeVar109 );
	nodeVar111 = ( dfg.x + dfg.y );
	nodeVar112 = ( 1.0 - nodeVar111 );
	nodeVar113 = nodeVar112;
	nodeVar114 = ( vec3<f32>( nodeVar113 ) * nodeVar109 );
	nodeVar115 = ( vec3<f32>( 1.0 ) - nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( nodeVar110 / nodeVar116 );
	nodeVar118 = ( nodeVar117 * vec3<f32>( nodeVar113 ) );
	nodeVar119 = ( multiScatteringMetallic + nodeVar118 );
	multiScatteringMetallic = nodeVar119;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar120 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar121 = ( radiance * nodeVar120 );
	nodeVar122 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar123 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar124 = ( nodeVar122 * nodeVar123 );
	nodeVar125 = ( nodeVar121 + nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar128 = ( vec3<f32>( 1.0 ) - nodeVar127 );
	nodeVar129 = nodeVar128;
	nodeVar130 = ( DiffuseContribution * nodeVar129 );
	nodeVar131 = ( nodeVar130 * nodeVar123 );
	nodeVar132 = nodeVar131;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar133 = ( indirectSpecular + nodeVar126 );
	indirectSpecular = nodeVar133;
	nodeVar134 = ( indirectDiffuse + nodeVar132 );
	indirectDiffuse = nodeVar134;
	ambientOcclusion = 1.0;
	nodeVar135 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar135;
	nodeVar136 = dot( normalView, positionViewDirection );
	nodeVar137 = ( clamp( nodeVar136, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar138 = ( Roughness * -16.0 );
	nodeVar139 = ( 1.0 - nodeVar138 );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( - nodeVar140 );
	nodeVar142 = exp2( nodeVar141 );
	nodeVar143 = pow( nodeVar137, nodeVar142 );
	nodeVar144 = ( 1.0 - nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( ambientOcclusion - nodeVar145 );
	nodeVar147 = ( indirectSpecular * vec3<f32>( clamp( nodeVar146, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar147;
	nodeVar148 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar148;
	nodeVar149 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar149;
	nodeVar150 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar150;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar151 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m0 = vec4<f32>( nodeVar151, 1.0 );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform25 );
	nodeVar152 = ( ( object.nodeUniform24 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar153 = ( ( render.nodeUniform26 * ( object.nodeUniform27 * object.nodeUniform28 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar154 = ( ( nodeVar152.xy / vec2<f32>( nodeVar152.w ) ) - ( nodeVar153.xy / vec2<f32>( nodeVar153.w ) ) );
	output.m1 = vec4<f32>( vec3<f32>( nodeVar154, 0.0 ), 1.0 );

	// result

	return output;

}
