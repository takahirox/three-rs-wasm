// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> normalLocal : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
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
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec4<f32>;

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
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) nodeVarying6 : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, ( positionLocal.yz * vec2<f32>( 1.0 ) ) );
	normalLocal = nodeVarying6;
	nodeVar1 = normalize( abs( normalLocal ) );
	nodeVar2 = ( nodeVar1 / vec3<f32>( dot( nodeVar1, vec3<f32>( 1.0, 1.0, 1.0 ) ) ) );
	nodeVar3 = textureSample( nodeUniform0, nodeUniform0_sampler, ( positionLocal.zx * vec2<f32>( 1.0 ) ) );
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, ( positionLocal.xy * vec2<f32>( 1.0 ) ) );
	DiffuseColor = ( ( ( ( ( nodeVar0 * vec4<f32>( nodeVar2.x ) ) + ( nodeVar3 * vec4<f32>( nodeVar2.y ) ) ) + ( nodeVar4 * vec4<f32>( nodeVar2.z ) ) ) + vec4<f32>( vec3<f32>( 0.0, 0.13286832154414627, 1.0 ), 1.0 ) ) * vec4<f32>( 0.8 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar5 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.0525 ) + max( max( nodeVar5.x, nodeVar5.y ), nodeVar5.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = dot( normalView, positionViewDirection );
	nodeVar7 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar6, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar7;
	nodeVar8 = ( dfg.x + dfg.y );
	nodeVar9 = ( 1.0 / nodeVar8 );
	nodeVar10 = nodeVar9;
	nodeVar11 = ( nodeVar10 - 1.0 );
	nodeVar12 = ( SpecularColorBlended * vec3<f32>( nodeVar11 ) );
	nodeVar13 = ( nodeVar12 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar13;
	nodeVar14 = ( render.nodeUniform11 - render.nodeUniform12 );
	nodeVar15 = vec4<f32>( nodeVar14, 0.0 );
	nodeVar16 = ( render.cameraViewMatrix * nodeVar15 );
	nodeVar17 = normalize( nodeVar16.xyz );
	nodeVar18 = nodeVar17;
	nodeVar19 = dot( normalView, nodeVar18 );
	nodeVar20 = ( vec3<f32>( clamp( nodeVar19, 0.0, 1.0 ) ) * render.nodeUniform13 );
	nodeVar21 = nodeVar20;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar22 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar23 = ( nodeVar21 * nodeVar22 );
	nodeVar24 = ( nodeVar18 + positionViewDirection );
	nodeVar25 = normalize( nodeVar24 );
	nodeVar26 = dot( positionViewDirection, nodeVar25 );
	nodeVar27 = clamp( nodeVar26, 0.0, 1.0 );
	nodeVar28 = exp2( ( ( ( nodeVar27 * -5.55473 ) - 6.98316 ) * nodeVar27 ) );
	nodeVar29 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar28 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar28 ) ) );
	nodeVar30 = ( vec3<f32>( 1.0 ) - nodeVar29 );
	nodeVar31 = nodeVar30;
	nodeVar32 = ( nodeVar23 * nodeVar31 );
	nodeVar33 = ( directDiffuse + nodeVar32 );
	directDiffuse = nodeVar33;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar34 = normalize( ( nodeVar18 + positionViewDirection ) );
	nodeVar35 = clamp( dot( positionViewDirection, nodeVar34 ), 0.0, 1.0 );
	nodeVar36 = exp2( ( ( ( nodeVar35 * -5.55473 ) - 6.98316 ) * nodeVar35 ) );
	nodeVar37 = ( Roughness * Roughness );
	nodeVar38 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar36 ) ) ) + vec3<f32>( ( 1.0 * nodeVar36 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar37, clamp( dot( normalView, nodeVar18 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar37, clamp( dot( normalView, nodeVar34 ), 0.0, 1.0 ) ) ) );
	nodeVar39 = ( nodeVar21 * nodeVar38 );
	nodeVar40 = ( nodeVar39 * multiScatteringCompensation );
	nodeVar41 = ( directSpecular + nodeVar40 );
	directSpecular = nodeVar41;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar42 = dot( normalWorld, normalize( render.nodeUniform16 ) );
	nodeVar43 = ( nodeVar42 * 0.5 );
	nodeVar44 = ( nodeVar43 + 0.5 );
	nodeVar45 = mix( render.nodeUniform14, render.nodeUniform15, nodeVar44 );
	nodeVar46 = ( irradiance + nodeVar45 );
	irradiance = nodeVar46;
	nodeVar47 = dot( normalWorld, normalize( render.nodeUniform19 ) );
	nodeVar48 = ( nodeVar47 * 0.5 );
	nodeVar49 = ( nodeVar48 + 0.5 );
	nodeVar50 = mix( render.nodeUniform17, render.nodeUniform18, nodeVar49 );
	nodeVar51 = ( irradiance + nodeVar50 );
	irradiance = nodeVar51;
	nodeVar52 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar53 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar54 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar55 = ( SpecularF90 * dfg.y );
	nodeVar56 = ( nodeVar54 + vec3<f32>( nodeVar55 ) );
	nodeVar57 = ( nodeVar52 + nodeVar56 );
	nodeVar52 = nodeVar57;
	nodeVar58 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar59 = nodeVar58;
	nodeVar60 = ( nodeVar59 * vec3<f32>( 0.047619 ) );
	nodeVar61 = ( SpecularColor + nodeVar60 );
	nodeVar62 = ( nodeVar56 * nodeVar61 );
	nodeVar63 = ( dfg.x + dfg.y );
	nodeVar64 = ( 1.0 - nodeVar63 );
	nodeVar65 = nodeVar64;
	nodeVar66 = ( vec3<f32>( nodeVar65 ) * nodeVar61 );
	nodeVar67 = ( vec3<f32>( 1.0 ) - nodeVar66 );
	nodeVar68 = nodeVar67;
	nodeVar69 = ( nodeVar62 / nodeVar68 );
	nodeVar70 = ( nodeVar69 * vec3<f32>( nodeVar65 ) );
	nodeVar71 = ( nodeVar53 + nodeVar70 );
	nodeVar53 = nodeVar71;
	nodeVar72 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar73 = ( irradiance * nodeVar72 );
	nodeVar74 = ( nodeVar52 + nodeVar53 );
	nodeVar75 = ( vec3<f32>( 1.0 ) - nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = ( nodeVar73 * nodeVar76 );
	nodeVar78 = nodeVar77;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar79 = ( indirectDiffuse + nodeVar78 );
	indirectDiffuse = nodeVar79;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar80 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar81 = ( SpecularF90 * dfg.y );
	nodeVar82 = ( nodeVar80 + vec3<f32>( nodeVar81 ) );
	nodeVar83 = ( singleScatteringDielectric + nodeVar82 );
	singleScatteringDielectric = nodeVar83;
	nodeVar84 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar85 = nodeVar84;
	nodeVar86 = ( nodeVar85 * vec3<f32>( 0.047619 ) );
	nodeVar87 = ( SpecularColor + nodeVar86 );
	nodeVar88 = ( nodeVar82 * nodeVar87 );
	nodeVar89 = ( dfg.x + dfg.y );
	nodeVar90 = ( 1.0 - nodeVar89 );
	nodeVar91 = nodeVar90;
	nodeVar92 = ( vec3<f32>( nodeVar91 ) * nodeVar87 );
	nodeVar93 = ( vec3<f32>( 1.0 ) - nodeVar92 );
	nodeVar94 = nodeVar93;
	nodeVar95 = ( nodeVar88 / nodeVar94 );
	nodeVar96 = ( nodeVar95 * vec3<f32>( nodeVar91 ) );
	nodeVar97 = ( multiScatteringDielectric + nodeVar96 );
	multiScatteringDielectric = nodeVar97;
	nodeVar98 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar99 = ( SpecularF90 * dfg.y );
	nodeVar100 = ( nodeVar98 + vec3<f32>( nodeVar99 ) );
	nodeVar101 = ( singleScatteringMetallic + nodeVar100 );
	singleScatteringMetallic = nodeVar101;
	nodeVar102 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar103 = nodeVar102;
	nodeVar104 = ( nodeVar103 * vec3<f32>( 0.047619 ) );
	nodeVar105 = ( DiffuseColor.xyz + nodeVar104 );
	nodeVar106 = ( nodeVar100 * nodeVar105 );
	nodeVar107 = ( dfg.x + dfg.y );
	nodeVar108 = ( 1.0 - nodeVar107 );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( vec3<f32>( nodeVar109 ) * nodeVar105 );
	nodeVar111 = ( vec3<f32>( 1.0 ) - nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( nodeVar106 / nodeVar112 );
	nodeVar114 = ( nodeVar113 * vec3<f32>( nodeVar109 ) );
	nodeVar115 = ( multiScatteringMetallic + nodeVar114 );
	multiScatteringMetallic = nodeVar115;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar116 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar117 = ( radiance * nodeVar116 );
	nodeVar118 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar119 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar120 = ( nodeVar118 * nodeVar119 );
	nodeVar121 = ( nodeVar117 + nodeVar120 );
	nodeVar122 = nodeVar121;
	nodeVar123 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar124 = ( vec3<f32>( 1.0 ) - nodeVar123 );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( DiffuseContribution * nodeVar125 );
	nodeVar127 = ( nodeVar126 * nodeVar119 );
	nodeVar128 = nodeVar127;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar129 = ( indirectSpecular + nodeVar122 );
	indirectSpecular = nodeVar129;
	nodeVar130 = ( indirectDiffuse + nodeVar128 );
	indirectDiffuse = nodeVar130;
	ambientOcclusion = 1.0;
	nodeVar131 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar131;
	nodeVar132 = dot( normalView, positionViewDirection );
	nodeVar133 = ( clamp( nodeVar132, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar134 = ( Roughness * -16.0 );
	nodeVar135 = ( 1.0 - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( - nodeVar136 );
	nodeVar138 = exp2( nodeVar137 );
	nodeVar139 = pow( nodeVar133, nodeVar138 );
	nodeVar140 = ( 1.0 - nodeVar139 );
	nodeVar141 = nodeVar140;
	nodeVar142 = ( ambientOcclusion - nodeVar141 );
	nodeVar143 = ( indirectSpecular * vec3<f32>( clamp( nodeVar142, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar143;
	nodeVar144 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar144;
	nodeVar145 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar145;
	nodeVar146 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar146;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar147 = vec4<f32>( mix( Output.xyz, render.nodeUniform20, smoothstep( render.nodeUniform21, render.nodeUniform22, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar147;

	// result

	output.color = nodeVar147;

	return output;

}
