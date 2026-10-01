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
@binding( 3 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform16 : mat3x3<f32>,
	nodeUniform17 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
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
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec4<f32>;

// codes
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
	@location( 3 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform3, 1.0 ) * nodeVar1 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform7;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar2 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform8, 0.0525 ) + max( max( nodeVar2.x, nodeVar2.y ), nodeVar2.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform7 ) ) );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	NORMAL_normalView = normalViewGeometry;
	nodeVar3 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	nodeVar4 = dpdx( nodeVarying6 );
	nodeVar5 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	nodeVar6 = - dpdy( nodeVarying6 );
	nodeVar7 = ( ( nodeVar3 * vec3<f32>( nodeVar4.x ) ) + ( nodeVar5 * vec3<f32>( nodeVar6.x ) ) );
	nodeVar9 = ( ( nodeVar3 * vec3<f32>( nodeVar4.y ) ) + ( nodeVar5 * vec3<f32>( nodeVar6.y ) ) );
	nodeVar10 = max( dot( nodeVar7, nodeVar7 ), dot( nodeVar9, nodeVar9 ) );

	if ( ( nodeVar10 == 0.0 ) ) {

		nodeVar8 = 0.0;

	} else {

		nodeVar8 = inverseSqrt( nodeVar10 );

	}

	tangentViewFrame = ( nodeVar7 * vec3<f32>( nodeVar8 ) );
	NORMAL_tangentView = tangentViewFrame;
	bitangentViewFrame = ( nodeVar9 * nodeVar8 );
	NORMAL_bitangentView = bitangentViewFrame;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar11 = textureSample( nodeUniform15, nodeUniform15_sampler, ( object.nodeUniform16 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	nodeVar12 = ( ( nodeVar11 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar12.xy * object.nodeUniform17 ), nodeVar12.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar13 = dot( normalView, positionViewDirection );
	nodeVar14 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( nodeVar13, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar14;
	nodeVar15 = ( dfg.x + dfg.y );
	nodeVar16 = ( 1.0 / nodeVar15 );
	nodeVar17 = nodeVar16;
	nodeVar18 = ( nodeVar17 - 1.0 );
	nodeVar19 = ( SpecularColorBlended * vec3<f32>( nodeVar18 ) );
	nodeVar20 = ( nodeVar19 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar20;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar21 = dot( normalWorld, normalize( render.nodeUniform21 ) );
	nodeVar22 = ( nodeVar21 * 0.5 );
	nodeVar23 = ( nodeVar22 + 0.5 );
	nodeVar24 = mix( render.nodeUniform18, render.nodeUniform19, nodeVar23 );
	nodeVar25 = ( irradiance + nodeVar24 );
	irradiance = nodeVar25;
	nodeVar26 = ( render.nodeUniform22 - render.nodeUniform23 );
	nodeVar27 = vec4<f32>( nodeVar26, 0.0 );
	nodeVar28 = ( render.cameraViewMatrix * nodeVar27 );
	nodeVar29 = normalize( nodeVar28.xyz );
	nodeVar30 = nodeVar29;
	nodeVar31 = dot( normalView, nodeVar30 );
	nodeVar32 = ( vec3<f32>( clamp( nodeVar31, 0.0, 1.0 ) ) * render.nodeUniform24 );
	nodeVar33 = nodeVar32;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar34 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar35 = ( nodeVar33 * nodeVar34 );
	nodeVar36 = ( nodeVar30 + positionViewDirection );
	nodeVar37 = normalize( nodeVar36 );
	nodeVar38 = dot( positionViewDirection, nodeVar37 );
	nodeVar39 = clamp( nodeVar38, 0.0, 1.0 );
	nodeVar40 = exp2( ( ( ( nodeVar39 * -5.55473 ) - 6.98316 ) * nodeVar39 ) );
	nodeVar41 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar40 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar40 ) ) );
	nodeVar42 = ( vec3<f32>( 1.0 ) - nodeVar41 );
	nodeVar43 = nodeVar42;
	nodeVar44 = ( nodeVar35 * nodeVar43 );
	nodeVar45 = ( directDiffuse + nodeVar44 );
	directDiffuse = nodeVar45;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar46 = normalize( ( nodeVar30 + positionViewDirection ) );
	nodeVar47 = clamp( dot( positionViewDirection, nodeVar46 ), 0.0, 1.0 );
	nodeVar48 = exp2( ( ( ( nodeVar47 * -5.55473 ) - 6.98316 ) * nodeVar47 ) );
	nodeVar49 = ( Roughness * Roughness );
	nodeVar50 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar48 ) ) ) + vec3<f32>( ( 1.0 * nodeVar48 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar49, clamp( dot( normalView, nodeVar30 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar49, clamp( dot( normalView, nodeVar46 ), 0.0, 1.0 ) ) ) );
	nodeVar51 = ( nodeVar33 * nodeVar50 );
	nodeVar52 = ( nodeVar51 * multiScatteringCompensation );
	nodeVar53 = ( directSpecular + nodeVar52 );
	directSpecular = nodeVar53;
	nodeVar54 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar56 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar57 = ( SpecularF90 * dfg.y );
	nodeVar58 = ( nodeVar56 + vec3<f32>( nodeVar57 ) );
	nodeVar59 = ( nodeVar54 + nodeVar58 );
	nodeVar54 = nodeVar59;
	nodeVar60 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar61 = nodeVar60;
	nodeVar62 = ( nodeVar61 * vec3<f32>( 0.047619 ) );
	nodeVar63 = ( SpecularColor + nodeVar62 );
	nodeVar64 = ( nodeVar58 * nodeVar63 );
	nodeVar65 = ( dfg.x + dfg.y );
	nodeVar66 = ( 1.0 - nodeVar65 );
	nodeVar67 = nodeVar66;
	nodeVar68 = ( vec3<f32>( nodeVar67 ) * nodeVar63 );
	nodeVar69 = ( vec3<f32>( 1.0 ) - nodeVar68 );
	nodeVar70 = nodeVar69;
	nodeVar71 = ( nodeVar64 / nodeVar70 );
	nodeVar72 = ( nodeVar71 * vec3<f32>( nodeVar67 ) );
	nodeVar73 = ( nodeVar55 + nodeVar72 );
	nodeVar55 = nodeVar73;
	nodeVar74 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar75 = ( irradiance * nodeVar74 );
	nodeVar76 = ( nodeVar54 + nodeVar55 );
	nodeVar77 = ( vec3<f32>( 1.0 ) - nodeVar76 );
	nodeVar78 = nodeVar77;
	nodeVar79 = ( nodeVar75 * nodeVar78 );
	nodeVar80 = nodeVar79;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar81 = ( indirectDiffuse + nodeVar80 );
	indirectDiffuse = nodeVar81;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar82 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar83 = ( SpecularF90 * dfg.y );
	nodeVar84 = ( nodeVar82 + vec3<f32>( nodeVar83 ) );
	nodeVar85 = ( singleScatteringDielectric + nodeVar84 );
	singleScatteringDielectric = nodeVar85;
	nodeVar86 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( nodeVar87 * vec3<f32>( 0.047619 ) );
	nodeVar89 = ( SpecularColor + nodeVar88 );
	nodeVar90 = ( nodeVar84 * nodeVar89 );
	nodeVar91 = ( dfg.x + dfg.y );
	nodeVar92 = ( 1.0 - nodeVar91 );
	nodeVar93 = nodeVar92;
	nodeVar94 = ( vec3<f32>( nodeVar93 ) * nodeVar89 );
	nodeVar95 = ( vec3<f32>( 1.0 ) - nodeVar94 );
	nodeVar96 = nodeVar95;
	nodeVar97 = ( nodeVar90 / nodeVar96 );
	nodeVar98 = ( nodeVar97 * vec3<f32>( nodeVar93 ) );
	nodeVar99 = ( multiScatteringDielectric + nodeVar98 );
	multiScatteringDielectric = nodeVar99;
	nodeVar100 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar101 = ( SpecularF90 * dfg.y );
	nodeVar102 = ( nodeVar100 + vec3<f32>( nodeVar101 ) );
	nodeVar103 = ( singleScatteringMetallic + nodeVar102 );
	singleScatteringMetallic = nodeVar103;
	nodeVar104 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( nodeVar105 * vec3<f32>( 0.047619 ) );
	nodeVar107 = ( DiffuseColor.xyz + nodeVar106 );
	nodeVar108 = ( nodeVar102 * nodeVar107 );
	nodeVar109 = ( dfg.x + dfg.y );
	nodeVar110 = ( 1.0 - nodeVar109 );
	nodeVar111 = nodeVar110;
	nodeVar112 = ( vec3<f32>( nodeVar111 ) * nodeVar107 );
	nodeVar113 = ( vec3<f32>( 1.0 ) - nodeVar112 );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar108 / nodeVar114 );
	nodeVar116 = ( nodeVar115 * vec3<f32>( nodeVar111 ) );
	nodeVar117 = ( multiScatteringMetallic + nodeVar116 );
	multiScatteringMetallic = nodeVar117;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar118 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar119 = ( radiance * nodeVar118 );
	nodeVar120 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar121 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar122 = ( nodeVar120 * nodeVar121 );
	nodeVar123 = ( nodeVar119 + nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar126 = ( vec3<f32>( 1.0 ) - nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( DiffuseContribution * nodeVar127 );
	nodeVar129 = ( nodeVar128 * nodeVar121 );
	nodeVar130 = nodeVar129;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar131 = ( indirectSpecular + nodeVar124 );
	indirectSpecular = nodeVar131;
	nodeVar132 = ( indirectDiffuse + nodeVar130 );
	indirectDiffuse = nodeVar132;
	ambientOcclusion = 1.0;
	nodeVar133 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar133;
	nodeVar134 = dot( normalView, positionViewDirection );
	nodeVar135 = ( clamp( nodeVar134, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar136 = ( Roughness * -16.0 );
	nodeVar137 = ( 1.0 - nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( - nodeVar138 );
	nodeVar140 = exp2( nodeVar139 );
	nodeVar141 = pow( nodeVar135, nodeVar140 );
	nodeVar142 = ( 1.0 - nodeVar141 );
	nodeVar143 = nodeVar142;
	nodeVar144 = ( ambientOcclusion - nodeVar143 );
	nodeVar145 = ( indirectSpecular * vec3<f32>( clamp( nodeVar144, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar145;
	nodeVar146 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar146;
	nodeVar147 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar147;
	nodeVar148 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar148;
	nodeVar149 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar149;

	// result

	output.color = nodeVar149;

	return output;

}
