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
	nodeUniform27 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
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
var<private> NORMAL_tangentView : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
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
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : vec4<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
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
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec4<f32>;

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
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_bitangentView : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 5 ) nodeVarying9 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform3, 1.0 ) * nodeVar1 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform7;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar3 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform8, 0.0525 ) + max( max( nodeVar3.x, nodeVar3.y ), nodeVar3.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform7 ) ) );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	NORMAL_tangentView = normalize( v_tangentView );
	NORMAL_bitangentView = normalize( NORMAL_v_bitangentView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar4 = textureSample( nodeUniform15, nodeUniform15_sampler, ( object.nodeUniform16 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	nodeVar5 = ( ( nodeVar4 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar5.xy * object.nodeUniform17 ), nodeVar5.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = dot( normalView, positionViewDirection );
	nodeVar7 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( nodeVar6, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar7;
	nodeVar8 = ( dfg.x + dfg.y );
	nodeVar9 = ( 1.0 / nodeVar8 );
	nodeVar10 = nodeVar9;
	nodeVar11 = ( nodeVar10 - 1.0 );
	nodeVar12 = ( SpecularColorBlended * vec3<f32>( nodeVar11 ) );
	nodeVar13 = ( nodeVar12 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar13;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar14 = dot( normalWorld, normalize( render.nodeUniform21 ) );
	nodeVar15 = ( nodeVar14 * 0.5 );
	nodeVar16 = ( nodeVar15 + 0.5 );
	nodeVar17 = mix( render.nodeUniform18, render.nodeUniform19, nodeVar16 );
	nodeVar18 = ( irradiance + nodeVar17 );
	irradiance = nodeVar18;
	nodeVar19 = ( render.nodeUniform22 - render.nodeUniform23 );
	nodeVar20 = vec4<f32>( nodeVar19, 0.0 );
	nodeVar21 = ( render.cameraViewMatrix * nodeVar20 );
	nodeVar22 = normalize( nodeVar21.xyz );
	nodeVar23 = nodeVar22;
	nodeVar24 = dot( normalView, nodeVar23 );
	nodeVar25 = ( vec3<f32>( clamp( nodeVar24, 0.0, 1.0 ) ) * render.nodeUniform24 );
	nodeVar26 = nodeVar25;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar27 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar28 = ( nodeVar26 * nodeVar27 );
	nodeVar29 = ( nodeVar23 + positionViewDirection );
	nodeVar30 = normalize( nodeVar29 );
	nodeVar31 = dot( positionViewDirection, nodeVar30 );
	nodeVar32 = clamp( nodeVar31, 0.0, 1.0 );
	nodeVar33 = exp2( ( ( ( nodeVar32 * -5.55473 ) - 6.98316 ) * nodeVar32 ) );
	nodeVar34 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar33 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar33 ) ) );
	nodeVar35 = ( vec3<f32>( 1.0 ) - nodeVar34 );
	nodeVar36 = nodeVar35;
	nodeVar37 = ( nodeVar28 * nodeVar36 );
	nodeVar38 = ( directDiffuse + nodeVar37 );
	directDiffuse = nodeVar38;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar39 = normalize( ( nodeVar23 + positionViewDirection ) );
	nodeVar40 = clamp( dot( positionViewDirection, nodeVar39 ), 0.0, 1.0 );
	nodeVar41 = exp2( ( ( ( nodeVar40 * -5.55473 ) - 6.98316 ) * nodeVar40 ) );
	nodeVar42 = ( Roughness * Roughness );
	nodeVar43 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar41 ) ) ) + vec3<f32>( ( 1.0 * nodeVar41 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar42, clamp( dot( normalView, nodeVar23 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar42, clamp( dot( normalView, nodeVar39 ), 0.0, 1.0 ) ) ) );
	nodeVar44 = ( nodeVar26 * nodeVar43 );
	nodeVar45 = ( nodeVar44 * multiScatteringCompensation );
	nodeVar46 = ( directSpecular + nodeVar45 );
	directSpecular = nodeVar46;
	nodeVar47 = ( render.nodeUniform25 - render.nodeUniform26 );
	nodeVar48 = vec4<f32>( nodeVar47, 0.0 );
	nodeVar49 = ( render.cameraViewMatrix * nodeVar48 );
	nodeVar50 = normalize( nodeVar49.xyz );
	nodeVar51 = nodeVar50;
	nodeVar52 = dot( normalView, nodeVar51 );
	nodeVar53 = ( vec3<f32>( clamp( nodeVar52, 0.0, 1.0 ) ) * render.nodeUniform27 );
	nodeVar54 = nodeVar53;
	nodeVar55 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar56 = ( nodeVar54 * nodeVar55 );
	nodeVar57 = ( nodeVar51 + positionViewDirection );
	nodeVar58 = normalize( nodeVar57 );
	nodeVar59 = dot( positionViewDirection, nodeVar58 );
	nodeVar60 = clamp( nodeVar59, 0.0, 1.0 );
	nodeVar61 = exp2( ( ( ( nodeVar60 * -5.55473 ) - 6.98316 ) * nodeVar60 ) );
	nodeVar62 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar61 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar61 ) ) );
	nodeVar63 = ( vec3<f32>( 1.0 ) - nodeVar62 );
	nodeVar64 = nodeVar63;
	nodeVar65 = ( nodeVar56 * nodeVar64 );
	nodeVar66 = ( directDiffuse + nodeVar65 );
	directDiffuse = nodeVar66;
	nodeVar67 = normalize( ( nodeVar51 + positionViewDirection ) );
	nodeVar68 = clamp( dot( positionViewDirection, nodeVar67 ), 0.0, 1.0 );
	nodeVar69 = exp2( ( ( ( nodeVar68 * -5.55473 ) - 6.98316 ) * nodeVar68 ) );
	nodeVar70 = ( Roughness * Roughness );
	nodeVar71 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar69 ) ) ) + vec3<f32>( ( 1.0 * nodeVar69 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar70, clamp( dot( normalView, nodeVar51 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar70, clamp( dot( normalView, nodeVar67 ), 0.0, 1.0 ) ) ) );
	nodeVar72 = ( nodeVar54 * nodeVar71 );
	nodeVar73 = ( nodeVar72 * multiScatteringCompensation );
	nodeVar74 = ( directSpecular + nodeVar73 );
	directSpecular = nodeVar74;
	nodeVar75 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar76 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar77 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar78 = ( SpecularF90 * dfg.y );
	nodeVar79 = ( nodeVar77 + vec3<f32>( nodeVar78 ) );
	nodeVar80 = ( nodeVar75 + nodeVar79 );
	nodeVar75 = nodeVar80;
	nodeVar81 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar82 = nodeVar81;
	nodeVar83 = ( nodeVar82 * vec3<f32>( 0.047619 ) );
	nodeVar84 = ( SpecularColor + nodeVar83 );
	nodeVar85 = ( nodeVar79 * nodeVar84 );
	nodeVar86 = ( dfg.x + dfg.y );
	nodeVar87 = ( 1.0 - nodeVar86 );
	nodeVar88 = nodeVar87;
	nodeVar89 = ( vec3<f32>( nodeVar88 ) * nodeVar84 );
	nodeVar90 = ( vec3<f32>( 1.0 ) - nodeVar89 );
	nodeVar91 = nodeVar90;
	nodeVar92 = ( nodeVar85 / nodeVar91 );
	nodeVar93 = ( nodeVar92 * vec3<f32>( nodeVar88 ) );
	nodeVar94 = ( nodeVar76 + nodeVar93 );
	nodeVar76 = nodeVar94;
	nodeVar95 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar96 = ( irradiance * nodeVar95 );
	nodeVar97 = ( nodeVar75 + nodeVar76 );
	nodeVar98 = ( vec3<f32>( 1.0 ) - nodeVar97 );
	nodeVar99 = nodeVar98;
	nodeVar100 = ( nodeVar96 * nodeVar99 );
	nodeVar101 = nodeVar100;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar102 = ( indirectDiffuse + nodeVar101 );
	indirectDiffuse = nodeVar102;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar103 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar104 = ( SpecularF90 * dfg.y );
	nodeVar105 = ( nodeVar103 + vec3<f32>( nodeVar104 ) );
	nodeVar106 = ( singleScatteringDielectric + nodeVar105 );
	singleScatteringDielectric = nodeVar106;
	nodeVar107 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar108 = nodeVar107;
	nodeVar109 = ( nodeVar108 * vec3<f32>( 0.047619 ) );
	nodeVar110 = ( SpecularColor + nodeVar109 );
	nodeVar111 = ( nodeVar105 * nodeVar110 );
	nodeVar112 = ( dfg.x + dfg.y );
	nodeVar113 = ( 1.0 - nodeVar112 );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( vec3<f32>( nodeVar114 ) * nodeVar110 );
	nodeVar116 = ( vec3<f32>( 1.0 ) - nodeVar115 );
	nodeVar117 = nodeVar116;
	nodeVar118 = ( nodeVar111 / nodeVar117 );
	nodeVar119 = ( nodeVar118 * vec3<f32>( nodeVar114 ) );
	nodeVar120 = ( multiScatteringDielectric + nodeVar119 );
	multiScatteringDielectric = nodeVar120;
	nodeVar121 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar122 = ( SpecularF90 * dfg.y );
	nodeVar123 = ( nodeVar121 + vec3<f32>( nodeVar122 ) );
	nodeVar124 = ( singleScatteringMetallic + nodeVar123 );
	singleScatteringMetallic = nodeVar124;
	nodeVar125 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( nodeVar126 * vec3<f32>( 0.047619 ) );
	nodeVar128 = ( DiffuseColor.xyz + nodeVar127 );
	nodeVar129 = ( nodeVar123 * nodeVar128 );
	nodeVar130 = ( dfg.x + dfg.y );
	nodeVar131 = ( 1.0 - nodeVar130 );
	nodeVar132 = nodeVar131;
	nodeVar133 = ( vec3<f32>( nodeVar132 ) * nodeVar128 );
	nodeVar134 = ( vec3<f32>( 1.0 ) - nodeVar133 );
	nodeVar135 = nodeVar134;
	nodeVar136 = ( nodeVar129 / nodeVar135 );
	nodeVar137 = ( nodeVar136 * vec3<f32>( nodeVar132 ) );
	nodeVar138 = ( multiScatteringMetallic + nodeVar137 );
	multiScatteringMetallic = nodeVar138;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar139 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar140 = ( radiance * nodeVar139 );
	nodeVar141 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar142 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar143 = ( nodeVar141 * nodeVar142 );
	nodeVar144 = ( nodeVar140 + nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar147 = ( vec3<f32>( 1.0 ) - nodeVar146 );
	nodeVar148 = nodeVar147;
	nodeVar149 = ( DiffuseContribution * nodeVar148 );
	nodeVar150 = ( nodeVar149 * nodeVar142 );
	nodeVar151 = nodeVar150;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar152 = ( indirectSpecular + nodeVar145 );
	indirectSpecular = nodeVar152;
	nodeVar153 = ( indirectDiffuse + nodeVar151 );
	indirectDiffuse = nodeVar153;
	ambientOcclusion = 1.0;
	nodeVar154 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar154;
	nodeVar155 = dot( normalView, positionViewDirection );
	nodeVar156 = ( clamp( nodeVar155, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar157 = ( Roughness * -16.0 );
	nodeVar158 = ( 1.0 - nodeVar157 );
	nodeVar159 = nodeVar158;
	nodeVar160 = ( - nodeVar159 );
	nodeVar161 = exp2( nodeVar160 );
	nodeVar162 = pow( nodeVar156, nodeVar161 );
	nodeVar163 = ( 1.0 - nodeVar162 );
	nodeVar164 = nodeVar163;
	nodeVar165 = ( ambientOcclusion - nodeVar164 );
	nodeVar166 = ( indirectSpecular * vec3<f32>( clamp( nodeVar165, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar166;
	nodeVar167 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar167;
	nodeVar168 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar168;
	nodeVar169 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar169;
	nodeVar170 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar170;

	// result

	output.color = nodeVar170;

	return output;

}
