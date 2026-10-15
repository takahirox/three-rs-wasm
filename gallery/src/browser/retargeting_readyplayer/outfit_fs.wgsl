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
@binding( 7 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;

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
	nodeUniform22 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform29 : vec3<f32>
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
var<private> nodeVar5 : vec3<f32>;
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
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
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
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : f32;
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
var<private> nodeVar168 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec4<f32>;

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
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform11 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar5 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform10 * nodeVar3.y ), 0.0525 ) + max( max( nodeVar5.x, nodeVar5.y ), nodeVar5.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	EmissiveColor = ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) );
	NORMAL_tangentView = normalize( v_tangentView );
	NORMAL_bitangentView = normalize( NORMAL_v_bitangentView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar6 = textureSample( nodeUniform18, nodeUniform18_sampler, ( object.nodeUniform19 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	nodeVar7 = ( ( nodeVar6 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar7.xy * object.nodeUniform20 ), nodeVar7.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar8 = dot( normalView, positionViewDirection );
	nodeVar9 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( nodeVar8, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar9;
	nodeVar10 = ( dfg.x + dfg.y );
	nodeVar11 = ( 1.0 / nodeVar10 );
	nodeVar12 = nodeVar11;
	nodeVar13 = ( nodeVar12 - 1.0 );
	nodeVar14 = ( SpecularColorBlended * vec3<f32>( nodeVar13 ) );
	nodeVar15 = ( nodeVar14 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar15;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar16 = dot( normalWorld, normalize( render.nodeUniform24 ) );
	nodeVar17 = ( nodeVar16 * 0.5 );
	nodeVar18 = ( nodeVar17 + 0.5 );
	nodeVar19 = mix( render.nodeUniform21, render.nodeUniform22, nodeVar18 );
	nodeVar20 = ( irradiance + nodeVar19 );
	irradiance = nodeVar20;
	nodeVar21 = ( render.nodeUniform25 - render.nodeUniform26 );
	nodeVar22 = vec4<f32>( nodeVar21, 0.0 );
	nodeVar23 = ( render.cameraViewMatrix * nodeVar22 );
	nodeVar24 = normalize( nodeVar23.xyz );
	nodeVar25 = nodeVar24;
	nodeVar26 = dot( normalView, nodeVar25 );
	nodeVar27 = ( vec3<f32>( clamp( nodeVar26, 0.0, 1.0 ) ) * render.nodeUniform27 );
	nodeVar28 = nodeVar27;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar29 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar30 = ( nodeVar28 * nodeVar29 );
	nodeVar31 = ( nodeVar25 + positionViewDirection );
	nodeVar32 = normalize( nodeVar31 );
	nodeVar33 = dot( positionViewDirection, nodeVar32 );
	nodeVar34 = clamp( nodeVar33, 0.0, 1.0 );
	nodeVar35 = exp2( ( ( ( nodeVar34 * -5.55473 ) - 6.98316 ) * nodeVar34 ) );
	nodeVar36 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar35 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar35 ) ) );
	nodeVar37 = ( vec3<f32>( 1.0 ) - nodeVar36 );
	nodeVar38 = nodeVar37;
	nodeVar39 = ( nodeVar30 * nodeVar38 );
	nodeVar40 = ( directDiffuse + nodeVar39 );
	directDiffuse = nodeVar40;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar41 = normalize( ( nodeVar25 + positionViewDirection ) );
	nodeVar42 = clamp( dot( positionViewDirection, nodeVar41 ), 0.0, 1.0 );
	nodeVar43 = exp2( ( ( ( nodeVar42 * -5.55473 ) - 6.98316 ) * nodeVar42 ) );
	nodeVar44 = ( Roughness * Roughness );
	nodeVar45 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar43 ) ) ) + vec3<f32>( ( 1.0 * nodeVar43 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar44, clamp( dot( normalView, nodeVar25 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar44, clamp( dot( normalView, nodeVar41 ), 0.0, 1.0 ) ) ) );
	nodeVar46 = ( nodeVar28 * nodeVar45 );
	nodeVar47 = ( nodeVar46 * multiScatteringCompensation );
	nodeVar48 = ( directSpecular + nodeVar47 );
	directSpecular = nodeVar48;
	nodeVar49 = ( render.nodeUniform28 - render.nodeUniform29 );
	nodeVar50 = vec4<f32>( nodeVar49, 0.0 );
	nodeVar51 = ( render.cameraViewMatrix * nodeVar50 );
	nodeVar52 = normalize( nodeVar51.xyz );
	nodeVar53 = nodeVar52;
	nodeVar54 = dot( normalView, nodeVar53 );
	nodeVar55 = ( vec3<f32>( clamp( nodeVar54, 0.0, 1.0 ) ) * render.nodeUniform30 );
	nodeVar56 = nodeVar55;
	nodeVar57 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar58 = ( nodeVar56 * nodeVar57 );
	nodeVar59 = ( nodeVar53 + positionViewDirection );
	nodeVar60 = normalize( nodeVar59 );
	nodeVar61 = dot( positionViewDirection, nodeVar60 );
	nodeVar62 = clamp( nodeVar61, 0.0, 1.0 );
	nodeVar63 = exp2( ( ( ( nodeVar62 * -5.55473 ) - 6.98316 ) * nodeVar62 ) );
	nodeVar64 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar63 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar63 ) ) );
	nodeVar65 = ( vec3<f32>( 1.0 ) - nodeVar64 );
	nodeVar66 = nodeVar65;
	nodeVar67 = ( nodeVar58 * nodeVar66 );
	nodeVar68 = ( directDiffuse + nodeVar67 );
	directDiffuse = nodeVar68;
	nodeVar69 = normalize( ( nodeVar53 + positionViewDirection ) );
	nodeVar70 = clamp( dot( positionViewDirection, nodeVar69 ), 0.0, 1.0 );
	nodeVar71 = exp2( ( ( ( nodeVar70 * -5.55473 ) - 6.98316 ) * nodeVar70 ) );
	nodeVar72 = ( Roughness * Roughness );
	nodeVar73 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar71 ) ) ) + vec3<f32>( ( 1.0 * nodeVar71 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar72, clamp( dot( normalView, nodeVar53 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar72, clamp( dot( normalView, nodeVar69 ), 0.0, 1.0 ) ) ) );
	nodeVar74 = ( nodeVar56 * nodeVar73 );
	nodeVar75 = ( nodeVar74 * multiScatteringCompensation );
	nodeVar76 = ( directSpecular + nodeVar75 );
	directSpecular = nodeVar76;
	nodeVar77 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar78 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar79 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar80 = ( SpecularF90 * dfg.y );
	nodeVar81 = ( nodeVar79 + vec3<f32>( nodeVar80 ) );
	nodeVar82 = ( nodeVar77 + nodeVar81 );
	nodeVar77 = nodeVar82;
	nodeVar83 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar84 = nodeVar83;
	nodeVar85 = ( nodeVar84 * vec3<f32>( 0.047619 ) );
	nodeVar86 = ( SpecularColor + nodeVar85 );
	nodeVar87 = ( nodeVar81 * nodeVar86 );
	nodeVar88 = ( dfg.x + dfg.y );
	nodeVar89 = ( 1.0 - nodeVar88 );
	nodeVar90 = nodeVar89;
	nodeVar91 = ( vec3<f32>( nodeVar90 ) * nodeVar86 );
	nodeVar92 = ( vec3<f32>( 1.0 ) - nodeVar91 );
	nodeVar93 = nodeVar92;
	nodeVar94 = ( nodeVar87 / nodeVar93 );
	nodeVar95 = ( nodeVar94 * vec3<f32>( nodeVar90 ) );
	nodeVar96 = ( nodeVar78 + nodeVar95 );
	nodeVar78 = nodeVar96;
	nodeVar97 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar98 = ( irradiance * nodeVar97 );
	nodeVar99 = ( nodeVar77 + nodeVar78 );
	nodeVar100 = ( vec3<f32>( 1.0 ) - nodeVar99 );
	nodeVar101 = nodeVar100;
	nodeVar102 = ( nodeVar98 * nodeVar101 );
	nodeVar103 = nodeVar102;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar104 = ( indirectDiffuse + nodeVar103 );
	indirectDiffuse = nodeVar104;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar105 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar106 = ( SpecularF90 * dfg.y );
	nodeVar107 = ( nodeVar105 + vec3<f32>( nodeVar106 ) );
	nodeVar108 = ( singleScatteringDielectric + nodeVar107 );
	singleScatteringDielectric = nodeVar108;
	nodeVar109 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar110 = nodeVar109;
	nodeVar111 = ( nodeVar110 * vec3<f32>( 0.047619 ) );
	nodeVar112 = ( SpecularColor + nodeVar111 );
	nodeVar113 = ( nodeVar107 * nodeVar112 );
	nodeVar114 = ( dfg.x + dfg.y );
	nodeVar115 = ( 1.0 - nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( vec3<f32>( nodeVar116 ) * nodeVar112 );
	nodeVar118 = ( vec3<f32>( 1.0 ) - nodeVar117 );
	nodeVar119 = nodeVar118;
	nodeVar120 = ( nodeVar113 / nodeVar119 );
	nodeVar121 = ( nodeVar120 * vec3<f32>( nodeVar116 ) );
	nodeVar122 = ( multiScatteringDielectric + nodeVar121 );
	multiScatteringDielectric = nodeVar122;
	nodeVar123 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar124 = ( SpecularF90 * dfg.y );
	nodeVar125 = ( nodeVar123 + vec3<f32>( nodeVar124 ) );
	nodeVar126 = ( singleScatteringMetallic + nodeVar125 );
	singleScatteringMetallic = nodeVar126;
	nodeVar127 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar128 = nodeVar127;
	nodeVar129 = ( nodeVar128 * vec3<f32>( 0.047619 ) );
	nodeVar130 = ( DiffuseColor.xyz + nodeVar129 );
	nodeVar131 = ( nodeVar125 * nodeVar130 );
	nodeVar132 = ( dfg.x + dfg.y );
	nodeVar133 = ( 1.0 - nodeVar132 );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( vec3<f32>( nodeVar134 ) * nodeVar130 );
	nodeVar136 = ( vec3<f32>( 1.0 ) - nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( nodeVar131 / nodeVar137 );
	nodeVar139 = ( nodeVar138 * vec3<f32>( nodeVar134 ) );
	nodeVar140 = ( multiScatteringMetallic + nodeVar139 );
	multiScatteringMetallic = nodeVar140;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar141 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar142 = ( radiance * nodeVar141 );
	nodeVar143 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar144 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar145 = ( nodeVar143 * nodeVar144 );
	nodeVar146 = ( nodeVar142 + nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar149 = ( vec3<f32>( 1.0 ) - nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( DiffuseContribution * nodeVar150 );
	nodeVar152 = ( nodeVar151 * nodeVar144 );
	nodeVar153 = nodeVar152;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar154 = ( indirectSpecular + nodeVar147 );
	indirectSpecular = nodeVar154;
	nodeVar155 = ( indirectDiffuse + nodeVar153 );
	indirectDiffuse = nodeVar155;
	ambientOcclusion = 1.0;
	nodeVar156 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar156;
	nodeVar157 = dot( normalView, positionViewDirection );
	nodeVar158 = ( clamp( nodeVar157, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar159 = ( Roughness * -16.0 );
	nodeVar160 = ( 1.0 - nodeVar159 );
	nodeVar161 = nodeVar160;
	nodeVar162 = ( - nodeVar161 );
	nodeVar163 = exp2( nodeVar162 );
	nodeVar164 = pow( nodeVar158, nodeVar163 );
	nodeVar165 = ( 1.0 - nodeVar164 );
	nodeVar166 = nodeVar165;
	nodeVar167 = ( ambientOcclusion - nodeVar166 );
	nodeVar168 = ( indirectSpecular * vec3<f32>( clamp( nodeVar167, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar168;
	nodeVar169 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar169;
	nodeVar170 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar170;
	nodeVar171 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar171;
	nodeVar172 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar172;

	// result

	output.color = nodeVar172;

	return output;

}
