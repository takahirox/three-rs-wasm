// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform3 : mat4x4<f32>,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat4x4<f32>
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
var<private> nodeVar7 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
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
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec4<f32>;

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
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar7 = textureSample( nodeUniform7, nodeUniform7_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform6, 1.0 ) * nodeVar7 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform9 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform10;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar8 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform11, 0.0525 ) + max( max( nodeVar8.x, nodeVar8.y ), nodeVar8.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform10 ) ) );
	EmissiveColor = ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar9 = dot( normalView, positionViewDirection );
	nodeVar10 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( nodeVar9, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar10;
	nodeVar11 = ( dfg.x + dfg.y );
	nodeVar12 = ( 1.0 / nodeVar11 );
	nodeVar13 = nodeVar12;
	nodeVar14 = ( nodeVar13 - 1.0 );
	nodeVar15 = ( SpecularColorBlended * vec3<f32>( nodeVar14 ) );
	nodeVar16 = ( nodeVar15 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar16;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar17 = dot( normalWorld, normalize( render.nodeUniform21 ) );
	nodeVar18 = ( nodeVar17 * 0.5 );
	nodeVar19 = ( nodeVar18 + 0.5 );
	nodeVar20 = mix( render.nodeUniform18, render.nodeUniform19, nodeVar19 );
	nodeVar21 = ( irradiance + nodeVar20 );
	irradiance = nodeVar21;
	nodeVar22 = ( render.nodeUniform22 - render.nodeUniform23 );
	nodeVar23 = vec4<f32>( nodeVar22, 0.0 );
	nodeVar24 = ( render.cameraViewMatrix * nodeVar23 );
	nodeVar25 = normalize( nodeVar24.xyz );
	nodeVar26 = nodeVar25;
	nodeVar27 = dot( normalView, nodeVar26 );
	nodeVar28 = ( vec3<f32>( clamp( nodeVar27, 0.0, 1.0 ) ) * render.nodeUniform24 );
	nodeVar29 = nodeVar28;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar30 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar31 = ( nodeVar29 * nodeVar30 );
	nodeVar32 = ( nodeVar26 + positionViewDirection );
	nodeVar33 = normalize( nodeVar32 );
	nodeVar34 = dot( positionViewDirection, nodeVar33 );
	nodeVar35 = clamp( nodeVar34, 0.0, 1.0 );
	nodeVar36 = exp2( ( ( ( nodeVar35 * -5.55473 ) - 6.98316 ) * nodeVar35 ) );
	nodeVar37 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar36 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar36 ) ) );
	nodeVar38 = ( vec3<f32>( 1.0 ) - nodeVar37 );
	nodeVar39 = nodeVar38;
	nodeVar40 = ( nodeVar31 * nodeVar39 );
	nodeVar41 = ( directDiffuse + nodeVar40 );
	directDiffuse = nodeVar41;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar42 = normalize( ( nodeVar26 + positionViewDirection ) );
	nodeVar43 = clamp( dot( positionViewDirection, nodeVar42 ), 0.0, 1.0 );
	nodeVar44 = exp2( ( ( ( nodeVar43 * -5.55473 ) - 6.98316 ) * nodeVar43 ) );
	nodeVar45 = ( Roughness * Roughness );
	nodeVar46 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar44 ) ) ) + vec3<f32>( ( 1.0 * nodeVar44 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar45, clamp( dot( normalView, nodeVar26 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar45, clamp( dot( normalView, nodeVar42 ), 0.0, 1.0 ) ) ) );
	nodeVar47 = ( nodeVar29 * nodeVar46 );
	nodeVar48 = ( nodeVar47 * multiScatteringCompensation );
	nodeVar49 = ( directSpecular + nodeVar48 );
	directSpecular = nodeVar49;
	nodeVar50 = ( render.nodeUniform25 - render.nodeUniform26 );
	nodeVar51 = vec4<f32>( nodeVar50, 0.0 );
	nodeVar52 = ( render.cameraViewMatrix * nodeVar51 );
	nodeVar53 = normalize( nodeVar52.xyz );
	nodeVar54 = nodeVar53;
	nodeVar55 = dot( normalView, nodeVar54 );
	nodeVar56 = ( vec3<f32>( clamp( nodeVar55, 0.0, 1.0 ) ) * render.nodeUniform27 );
	nodeVar57 = nodeVar56;
	nodeVar58 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar59 = ( nodeVar57 * nodeVar58 );
	nodeVar60 = ( nodeVar54 + positionViewDirection );
	nodeVar61 = normalize( nodeVar60 );
	nodeVar62 = dot( positionViewDirection, nodeVar61 );
	nodeVar63 = clamp( nodeVar62, 0.0, 1.0 );
	nodeVar64 = exp2( ( ( ( nodeVar63 * -5.55473 ) - 6.98316 ) * nodeVar63 ) );
	nodeVar65 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar64 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar64 ) ) );
	nodeVar66 = ( vec3<f32>( 1.0 ) - nodeVar65 );
	nodeVar67 = nodeVar66;
	nodeVar68 = ( nodeVar59 * nodeVar67 );
	nodeVar69 = ( directDiffuse + nodeVar68 );
	directDiffuse = nodeVar69;
	nodeVar70 = normalize( ( nodeVar54 + positionViewDirection ) );
	nodeVar71 = clamp( dot( positionViewDirection, nodeVar70 ), 0.0, 1.0 );
	nodeVar72 = exp2( ( ( ( nodeVar71 * -5.55473 ) - 6.98316 ) * nodeVar71 ) );
	nodeVar73 = ( Roughness * Roughness );
	nodeVar74 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar72 ) ) ) + vec3<f32>( ( 1.0 * nodeVar72 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar73, clamp( dot( normalView, nodeVar54 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar73, clamp( dot( normalView, nodeVar70 ), 0.0, 1.0 ) ) ) );
	nodeVar75 = ( nodeVar57 * nodeVar74 );
	nodeVar76 = ( nodeVar75 * multiScatteringCompensation );
	nodeVar77 = ( directSpecular + nodeVar76 );
	directSpecular = nodeVar77;
	nodeVar78 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar79 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar80 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar81 = ( SpecularF90 * dfg.y );
	nodeVar82 = ( nodeVar80 + vec3<f32>( nodeVar81 ) );
	nodeVar83 = ( nodeVar78 + nodeVar82 );
	nodeVar78 = nodeVar83;
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
	nodeVar97 = ( nodeVar79 + nodeVar96 );
	nodeVar79 = nodeVar97;
	nodeVar98 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar99 = ( irradiance * nodeVar98 );
	nodeVar100 = ( nodeVar78 + nodeVar79 );
	nodeVar101 = ( vec3<f32>( 1.0 ) - nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( nodeVar99 * nodeVar102 );
	nodeVar104 = nodeVar103;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar105 = ( indirectDiffuse + nodeVar104 );
	indirectDiffuse = nodeVar105;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar106 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar107 = ( SpecularF90 * dfg.y );
	nodeVar108 = ( nodeVar106 + vec3<f32>( nodeVar107 ) );
	nodeVar109 = ( singleScatteringDielectric + nodeVar108 );
	singleScatteringDielectric = nodeVar109;
	nodeVar110 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar111 = nodeVar110;
	nodeVar112 = ( nodeVar111 * vec3<f32>( 0.047619 ) );
	nodeVar113 = ( SpecularColor + nodeVar112 );
	nodeVar114 = ( nodeVar108 * nodeVar113 );
	nodeVar115 = ( dfg.x + dfg.y );
	nodeVar116 = ( 1.0 - nodeVar115 );
	nodeVar117 = nodeVar116;
	nodeVar118 = ( vec3<f32>( nodeVar117 ) * nodeVar113 );
	nodeVar119 = ( vec3<f32>( 1.0 ) - nodeVar118 );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( nodeVar114 / nodeVar120 );
	nodeVar122 = ( nodeVar121 * vec3<f32>( nodeVar117 ) );
	nodeVar123 = ( multiScatteringDielectric + nodeVar122 );
	multiScatteringDielectric = nodeVar123;
	nodeVar124 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar125 = ( SpecularF90 * dfg.y );
	nodeVar126 = ( nodeVar124 + vec3<f32>( nodeVar125 ) );
	nodeVar127 = ( singleScatteringMetallic + nodeVar126 );
	singleScatteringMetallic = nodeVar127;
	nodeVar128 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar129 = nodeVar128;
	nodeVar130 = ( nodeVar129 * vec3<f32>( 0.047619 ) );
	nodeVar131 = ( DiffuseColor.xyz + nodeVar130 );
	nodeVar132 = ( nodeVar126 * nodeVar131 );
	nodeVar133 = ( dfg.x + dfg.y );
	nodeVar134 = ( 1.0 - nodeVar133 );
	nodeVar135 = nodeVar134;
	nodeVar136 = ( vec3<f32>( nodeVar135 ) * nodeVar131 );
	nodeVar137 = ( vec3<f32>( 1.0 ) - nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( nodeVar132 / nodeVar138 );
	nodeVar140 = ( nodeVar139 * vec3<f32>( nodeVar135 ) );
	nodeVar141 = ( multiScatteringMetallic + nodeVar140 );
	multiScatteringMetallic = nodeVar141;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar142 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar143 = ( radiance * nodeVar142 );
	nodeVar144 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar145 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar146 = ( nodeVar144 * nodeVar145 );
	nodeVar147 = ( nodeVar143 + nodeVar146 );
	nodeVar148 = nodeVar147;
	nodeVar149 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar150 = ( vec3<f32>( 1.0 ) - nodeVar149 );
	nodeVar151 = nodeVar150;
	nodeVar152 = ( DiffuseContribution * nodeVar151 );
	nodeVar153 = ( nodeVar152 * nodeVar145 );
	nodeVar154 = nodeVar153;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar155 = ( indirectSpecular + nodeVar148 );
	indirectSpecular = nodeVar155;
	nodeVar156 = ( indirectDiffuse + nodeVar154 );
	indirectDiffuse = nodeVar156;
	ambientOcclusion = 1.0;
	nodeVar157 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar157;
	nodeVar158 = dot( normalView, positionViewDirection );
	nodeVar159 = ( clamp( nodeVar158, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar160 = ( Roughness * -16.0 );
	nodeVar161 = ( 1.0 - nodeVar160 );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( - nodeVar162 );
	nodeVar164 = exp2( nodeVar163 );
	nodeVar165 = pow( nodeVar159, nodeVar164 );
	nodeVar166 = ( 1.0 - nodeVar165 );
	nodeVar167 = nodeVar166;
	nodeVar168 = ( ambientOcclusion - nodeVar167 );
	nodeVar169 = ( indirectSpecular * vec3<f32>( clamp( nodeVar168, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar169;
	nodeVar170 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar170;
	nodeVar171 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar171;
	nodeVar172 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar172;
	nodeVar173 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar173;

	// result

	output.color = nodeVar173;

	return output;

}
