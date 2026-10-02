// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
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
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec4<f32>;

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
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = dot( normalView, positionViewDirection );
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar1, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	nodeVar3 = ( dfg.x + dfg.y );
	nodeVar4 = ( 1.0 / nodeVar3 );
	nodeVar5 = nodeVar4;
	nodeVar6 = ( nodeVar5 - 1.0 );
	nodeVar7 = ( SpecularColorBlended * vec3<f32>( nodeVar6 ) );
	nodeVar8 = ( nodeVar7 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar8;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( irradiance + render.nodeUniform10 );
	irradiance = nodeVar9;
	nodeVar10 = ( render.nodeUniform12 - render.nodeUniform13 );
	nodeVar11 = vec4<f32>( nodeVar10, 0.0 );
	nodeVar12 = ( render.cameraViewMatrix * nodeVar11 );
	nodeVar13 = normalize( nodeVar12.xyz );
	nodeVar14 = nodeVar13;
	nodeVar15 = dot( normalView, nodeVar14 );
	nodeVar16 = ( vec3<f32>( clamp( nodeVar15, 0.0, 1.0 ) ) * render.nodeUniform14 );
	nodeVar17 = nodeVar16;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar18 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar19 = ( nodeVar17 * nodeVar18 );
	nodeVar20 = ( nodeVar14 + positionViewDirection );
	nodeVar21 = normalize( nodeVar20 );
	nodeVar22 = dot( positionViewDirection, nodeVar21 );
	nodeVar23 = clamp( nodeVar22, 0.0, 1.0 );
	nodeVar24 = exp2( ( ( ( nodeVar23 * -5.55473 ) - 6.98316 ) * nodeVar23 ) );
	nodeVar25 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar24 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar24 ) ) );
	nodeVar26 = ( vec3<f32>( 1.0 ) - nodeVar25 );
	nodeVar27 = nodeVar26;
	nodeVar28 = ( nodeVar19 * nodeVar27 );
	nodeVar29 = ( directDiffuse + nodeVar28 );
	directDiffuse = nodeVar29;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar30 = normalize( ( nodeVar14 + positionViewDirection ) );
	nodeVar31 = clamp( dot( positionViewDirection, nodeVar30 ), 0.0, 1.0 );
	nodeVar32 = exp2( ( ( ( nodeVar31 * -5.55473 ) - 6.98316 ) * nodeVar31 ) );
	nodeVar33 = ( Roughness * Roughness );
	nodeVar34 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar32 ) ) ) + vec3<f32>( ( 1.0 * nodeVar32 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar33, clamp( dot( normalView, nodeVar14 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar33, clamp( dot( normalView, nodeVar30 ), 0.0, 1.0 ) ) ) );
	nodeVar35 = ( nodeVar17 * nodeVar34 );
	nodeVar36 = ( nodeVar35 * multiScatteringCompensation );
	nodeVar37 = ( directSpecular + nodeVar36 );
	directSpecular = nodeVar37;
	nodeVar38 = ( render.nodeUniform15 - render.nodeUniform16 );
	nodeVar39 = vec4<f32>( nodeVar38, 0.0 );
	nodeVar40 = ( render.cameraViewMatrix * nodeVar39 );
	nodeVar41 = normalize( nodeVar40.xyz );
	nodeVar42 = nodeVar41;
	nodeVar43 = dot( normalView, nodeVar42 );
	nodeVar44 = ( vec3<f32>( clamp( nodeVar43, 0.0, 1.0 ) ) * render.nodeUniform17 );
	nodeVar45 = nodeVar44;
	nodeVar46 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar47 = ( nodeVar45 * nodeVar46 );
	nodeVar48 = ( nodeVar42 + positionViewDirection );
	nodeVar49 = normalize( nodeVar48 );
	nodeVar50 = dot( positionViewDirection, nodeVar49 );
	nodeVar51 = clamp( nodeVar50, 0.0, 1.0 );
	nodeVar52 = exp2( ( ( ( nodeVar51 * -5.55473 ) - 6.98316 ) * nodeVar51 ) );
	nodeVar53 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar52 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar52 ) ) );
	nodeVar54 = ( vec3<f32>( 1.0 ) - nodeVar53 );
	nodeVar55 = nodeVar54;
	nodeVar56 = ( nodeVar47 * nodeVar55 );
	nodeVar57 = ( directDiffuse + nodeVar56 );
	directDiffuse = nodeVar57;
	nodeVar58 = normalize( ( nodeVar42 + positionViewDirection ) );
	nodeVar59 = clamp( dot( positionViewDirection, nodeVar58 ), 0.0, 1.0 );
	nodeVar60 = exp2( ( ( ( nodeVar59 * -5.55473 ) - 6.98316 ) * nodeVar59 ) );
	nodeVar61 = ( Roughness * Roughness );
	nodeVar62 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar60 ) ) ) + vec3<f32>( ( 1.0 * nodeVar60 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar61, clamp( dot( normalView, nodeVar42 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar61, clamp( dot( normalView, nodeVar58 ), 0.0, 1.0 ) ) ) );
	nodeVar63 = ( nodeVar45 * nodeVar62 );
	nodeVar64 = ( nodeVar63 * multiScatteringCompensation );
	nodeVar65 = ( directSpecular + nodeVar64 );
	directSpecular = nodeVar65;
	nodeVar66 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar67 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar68 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar69 = ( SpecularF90 * dfg.y );
	nodeVar70 = ( nodeVar68 + vec3<f32>( nodeVar69 ) );
	nodeVar71 = ( nodeVar66 + nodeVar70 );
	nodeVar66 = nodeVar71;
	nodeVar72 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar73 = nodeVar72;
	nodeVar74 = ( nodeVar73 * vec3<f32>( 0.047619 ) );
	nodeVar75 = ( SpecularColor + nodeVar74 );
	nodeVar76 = ( nodeVar70 * nodeVar75 );
	nodeVar77 = ( dfg.x + dfg.y );
	nodeVar78 = ( 1.0 - nodeVar77 );
	nodeVar79 = nodeVar78;
	nodeVar80 = ( vec3<f32>( nodeVar79 ) * nodeVar75 );
	nodeVar81 = ( vec3<f32>( 1.0 ) - nodeVar80 );
	nodeVar82 = nodeVar81;
	nodeVar83 = ( nodeVar76 / nodeVar82 );
	nodeVar84 = ( nodeVar83 * vec3<f32>( nodeVar79 ) );
	nodeVar85 = ( nodeVar67 + nodeVar84 );
	nodeVar67 = nodeVar85;
	nodeVar86 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar87 = ( irradiance * nodeVar86 );
	nodeVar88 = ( nodeVar66 + nodeVar67 );
	nodeVar89 = ( vec3<f32>( 1.0 ) - nodeVar88 );
	nodeVar90 = nodeVar89;
	nodeVar91 = ( nodeVar87 * nodeVar90 );
	nodeVar92 = nodeVar91;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar93 = ( indirectDiffuse + nodeVar92 );
	indirectDiffuse = nodeVar93;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar95 = ( SpecularF90 * dfg.y );
	nodeVar96 = ( nodeVar94 + vec3<f32>( nodeVar95 ) );
	nodeVar97 = ( singleScatteringDielectric + nodeVar96 );
	singleScatteringDielectric = nodeVar97;
	nodeVar98 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar99 = nodeVar98;
	nodeVar100 = ( nodeVar99 * vec3<f32>( 0.047619 ) );
	nodeVar101 = ( SpecularColor + nodeVar100 );
	nodeVar102 = ( nodeVar96 * nodeVar101 );
	nodeVar103 = ( dfg.x + dfg.y );
	nodeVar104 = ( 1.0 - nodeVar103 );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( vec3<f32>( nodeVar105 ) * nodeVar101 );
	nodeVar107 = ( vec3<f32>( 1.0 ) - nodeVar106 );
	nodeVar108 = nodeVar107;
	nodeVar109 = ( nodeVar102 / nodeVar108 );
	nodeVar110 = ( nodeVar109 * vec3<f32>( nodeVar105 ) );
	nodeVar111 = ( multiScatteringDielectric + nodeVar110 );
	multiScatteringDielectric = nodeVar111;
	nodeVar112 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar113 = ( SpecularF90 * dfg.y );
	nodeVar114 = ( nodeVar112 + vec3<f32>( nodeVar113 ) );
	nodeVar115 = ( singleScatteringMetallic + nodeVar114 );
	singleScatteringMetallic = nodeVar115;
	nodeVar116 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar117 = nodeVar116;
	nodeVar118 = ( nodeVar117 * vec3<f32>( 0.047619 ) );
	nodeVar119 = ( DiffuseColor.xyz + nodeVar118 );
	nodeVar120 = ( nodeVar114 * nodeVar119 );
	nodeVar121 = ( dfg.x + dfg.y );
	nodeVar122 = ( 1.0 - nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( vec3<f32>( nodeVar123 ) * nodeVar119 );
	nodeVar125 = ( vec3<f32>( 1.0 ) - nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( nodeVar120 / nodeVar126 );
	nodeVar128 = ( nodeVar127 * vec3<f32>( nodeVar123 ) );
	nodeVar129 = ( multiScatteringMetallic + nodeVar128 );
	multiScatteringMetallic = nodeVar129;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar130 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar131 = ( radiance * nodeVar130 );
	nodeVar132 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar133 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar134 = ( nodeVar132 * nodeVar133 );
	nodeVar135 = ( nodeVar131 + nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar138 = ( vec3<f32>( 1.0 ) - nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( DiffuseContribution * nodeVar139 );
	nodeVar141 = ( nodeVar140 * nodeVar133 );
	nodeVar142 = nodeVar141;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar143 = ( indirectSpecular + nodeVar136 );
	indirectSpecular = nodeVar143;
	nodeVar144 = ( indirectDiffuse + nodeVar142 );
	indirectDiffuse = nodeVar144;
	ambientOcclusion = 1.0;
	nodeVar145 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar145;
	nodeVar146 = dot( normalView, positionViewDirection );
	nodeVar147 = ( clamp( nodeVar146, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar148 = ( Roughness * -16.0 );
	nodeVar149 = ( 1.0 - nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( - nodeVar150 );
	nodeVar152 = exp2( nodeVar151 );
	nodeVar153 = pow( nodeVar147, nodeVar152 );
	nodeVar154 = ( 1.0 - nodeVar153 );
	nodeVar155 = nodeVar154;
	nodeVar156 = ( ambientOcclusion - nodeVar155 );
	nodeVar157 = ( indirectSpecular * vec3<f32>( clamp( nodeVar156, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar157;
	nodeVar158 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar158;
	nodeVar159 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar159;
	nodeVar160 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar160;
	nodeVar161 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar161;

	// result

	output.color = nodeVar161;

	return output;

}
