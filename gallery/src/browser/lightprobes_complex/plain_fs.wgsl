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
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform14 : vec3<f32>
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
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
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
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar148 : vec3<f32>;
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
var<private> nodeVar159 : f32;
var<private> nodeVar160 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec4<f32>;

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
	@location( 2 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

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
	nodeVar9 = ( render.nodeUniform10 - v_positionView );
	nodeVar10 = normalize( nodeVar9 );
	nodeVar11 = dot( normalView, nodeVar10 );

	if ( ( render.nodeUniform12 > 0.0 ) ) {

		nodeVar13 = length( nodeVar9 );
		nodeVar14 = ( nodeVar13 / render.nodeUniform12 );
		nodeVar15 = clamp( ( 1.0 - ( ( ( nodeVar14 * nodeVar14 ) * nodeVar14 ) * nodeVar14 ) ), 0.0, 1.0 );
		nodeVar12 = ( ( 1.0 / max( pow( nodeVar13, render.nodeUniform13 ), 0.01 ) ) * ( nodeVar15 * nodeVar15 ) );

	} else {

		nodeVar12 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform13 ), 0.01 ) );

	}

	nodeVar16 = ( render.nodeUniform11 * vec3<f32>( nodeVar12 ) );
	nodeVar17 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar16 );
	nodeVar18 = nodeVar17;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar19 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar20 = ( nodeVar18 * nodeVar19 );
	nodeVar21 = ( nodeVar10 + positionViewDirection );
	nodeVar22 = normalize( nodeVar21 );
	nodeVar23 = dot( positionViewDirection, nodeVar22 );
	nodeVar24 = clamp( nodeVar23, 0.0, 1.0 );
	nodeVar25 = exp2( ( ( ( nodeVar24 * -5.55473 ) - 6.98316 ) * nodeVar24 ) );
	nodeVar26 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar25 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar25 ) ) );
	nodeVar27 = ( vec3<f32>( 1.0 ) - nodeVar26 );
	nodeVar28 = nodeVar27;
	nodeVar29 = ( nodeVar20 * nodeVar28 );
	nodeVar30 = ( directDiffuse + nodeVar29 );
	directDiffuse = nodeVar30;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar31 = normalize( ( nodeVar10 + positionViewDirection ) );
	nodeVar32 = clamp( dot( positionViewDirection, nodeVar31 ), 0.0, 1.0 );
	nodeVar33 = exp2( ( ( ( nodeVar32 * -5.55473 ) - 6.98316 ) * nodeVar32 ) );
	nodeVar34 = ( Roughness * Roughness );
	nodeVar35 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar33 ) ) ) + vec3<f32>( ( 1.0 * nodeVar33 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar34, clamp( dot( normalView, nodeVar10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar34, clamp( dot( normalView, nodeVar31 ), 0.0, 1.0 ) ) ) );
	nodeVar36 = ( nodeVar18 * nodeVar35 );
	nodeVar37 = ( nodeVar36 * multiScatteringCompensation );
	nodeVar38 = ( directSpecular + nodeVar37 );
	directSpecular = nodeVar38;
	nodeVar39 = ( render.nodeUniform14 - v_positionView );
	nodeVar40 = normalize( nodeVar39 );
	nodeVar41 = dot( normalView, nodeVar40 );

	if ( ( render.nodeUniform16 > 0.0 ) ) {

		nodeVar43 = length( nodeVar39 );
		nodeVar44 = ( nodeVar43 / render.nodeUniform16 );
		nodeVar45 = clamp( ( 1.0 - ( ( ( nodeVar44 * nodeVar44 ) * nodeVar44 ) * nodeVar44 ) ), 0.0, 1.0 );
		nodeVar42 = ( ( 1.0 / max( pow( nodeVar43, render.nodeUniform17 ), 0.01 ) ) * ( nodeVar45 * nodeVar45 ) );

	} else {

		nodeVar42 = ( 1.0 / max( pow( length( nodeVar39 ), render.nodeUniform17 ), 0.01 ) );

	}

	nodeVar46 = ( render.nodeUniform15 * vec3<f32>( nodeVar42 ) );
	nodeVar47 = ( vec3<f32>( clamp( nodeVar41, 0.0, 1.0 ) ) * nodeVar46 );
	nodeVar48 = nodeVar47;
	nodeVar49 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar50 = ( nodeVar48 * nodeVar49 );
	nodeVar51 = ( nodeVar40 + positionViewDirection );
	nodeVar52 = normalize( nodeVar51 );
	nodeVar53 = dot( positionViewDirection, nodeVar52 );
	nodeVar54 = clamp( nodeVar53, 0.0, 1.0 );
	nodeVar55 = exp2( ( ( ( nodeVar54 * -5.55473 ) - 6.98316 ) * nodeVar54 ) );
	nodeVar56 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar55 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar55 ) ) );
	nodeVar57 = ( vec3<f32>( 1.0 ) - nodeVar56 );
	nodeVar58 = nodeVar57;
	nodeVar59 = ( nodeVar50 * nodeVar58 );
	nodeVar60 = ( directDiffuse + nodeVar59 );
	directDiffuse = nodeVar60;
	nodeVar61 = normalize( ( nodeVar40 + positionViewDirection ) );
	nodeVar62 = clamp( dot( positionViewDirection, nodeVar61 ), 0.0, 1.0 );
	nodeVar63 = exp2( ( ( ( nodeVar62 * -5.55473 ) - 6.98316 ) * nodeVar62 ) );
	nodeVar64 = ( Roughness * Roughness );
	nodeVar65 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar63 ) ) ) + vec3<f32>( ( 1.0 * nodeVar63 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar64, clamp( dot( normalView, nodeVar40 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar64, clamp( dot( normalView, nodeVar61 ), 0.0, 1.0 ) ) ) );
	nodeVar66 = ( nodeVar48 * nodeVar65 );
	nodeVar67 = ( nodeVar66 * multiScatteringCompensation );
	nodeVar68 = ( directSpecular + nodeVar67 );
	directSpecular = nodeVar68;
	nodeVar69 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar70 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar71 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar72 = ( SpecularF90 * dfg.y );
	nodeVar73 = ( nodeVar71 + vec3<f32>( nodeVar72 ) );
	nodeVar74 = ( nodeVar69 + nodeVar73 );
	nodeVar69 = nodeVar74;
	nodeVar75 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar76 = nodeVar75;
	nodeVar77 = ( nodeVar76 * vec3<f32>( 0.047619 ) );
	nodeVar78 = ( SpecularColor + nodeVar77 );
	nodeVar79 = ( nodeVar73 * nodeVar78 );
	nodeVar80 = ( dfg.x + dfg.y );
	nodeVar81 = ( 1.0 - nodeVar80 );
	nodeVar82 = nodeVar81;
	nodeVar83 = ( vec3<f32>( nodeVar82 ) * nodeVar78 );
	nodeVar84 = ( vec3<f32>( 1.0 ) - nodeVar83 );
	nodeVar85 = nodeVar84;
	nodeVar86 = ( nodeVar79 / nodeVar85 );
	nodeVar87 = ( nodeVar86 * vec3<f32>( nodeVar82 ) );
	nodeVar88 = ( nodeVar70 + nodeVar87 );
	nodeVar70 = nodeVar88;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar89 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar90 = ( irradiance * nodeVar89 );
	nodeVar91 = ( nodeVar69 + nodeVar70 );
	nodeVar92 = ( vec3<f32>( 1.0 ) - nodeVar91 );
	nodeVar93 = nodeVar92;
	nodeVar94 = ( nodeVar90 * nodeVar93 );
	nodeVar95 = nodeVar94;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar96 = ( indirectDiffuse + nodeVar95 );
	indirectDiffuse = nodeVar96;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar97 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar98 = ( SpecularF90 * dfg.y );
	nodeVar99 = ( nodeVar97 + vec3<f32>( nodeVar98 ) );
	nodeVar100 = ( singleScatteringDielectric + nodeVar99 );
	singleScatteringDielectric = nodeVar100;
	nodeVar101 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( nodeVar102 * vec3<f32>( 0.047619 ) );
	nodeVar104 = ( SpecularColor + nodeVar103 );
	nodeVar105 = ( nodeVar99 * nodeVar104 );
	nodeVar106 = ( dfg.x + dfg.y );
	nodeVar107 = ( 1.0 - nodeVar106 );
	nodeVar108 = nodeVar107;
	nodeVar109 = ( vec3<f32>( nodeVar108 ) * nodeVar104 );
	nodeVar110 = ( vec3<f32>( 1.0 ) - nodeVar109 );
	nodeVar111 = nodeVar110;
	nodeVar112 = ( nodeVar105 / nodeVar111 );
	nodeVar113 = ( nodeVar112 * vec3<f32>( nodeVar108 ) );
	nodeVar114 = ( multiScatteringDielectric + nodeVar113 );
	multiScatteringDielectric = nodeVar114;
	nodeVar115 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar116 = ( SpecularF90 * dfg.y );
	nodeVar117 = ( nodeVar115 + vec3<f32>( nodeVar116 ) );
	nodeVar118 = ( singleScatteringMetallic + nodeVar117 );
	singleScatteringMetallic = nodeVar118;
	nodeVar119 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( nodeVar120 * vec3<f32>( 0.047619 ) );
	nodeVar122 = ( DiffuseColor.xyz + nodeVar121 );
	nodeVar123 = ( nodeVar117 * nodeVar122 );
	nodeVar124 = ( dfg.x + dfg.y );
	nodeVar125 = ( 1.0 - nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( vec3<f32>( nodeVar126 ) * nodeVar122 );
	nodeVar128 = ( vec3<f32>( 1.0 ) - nodeVar127 );
	nodeVar129 = nodeVar128;
	nodeVar130 = ( nodeVar123 / nodeVar129 );
	nodeVar131 = ( nodeVar130 * vec3<f32>( nodeVar126 ) );
	nodeVar132 = ( multiScatteringMetallic + nodeVar131 );
	multiScatteringMetallic = nodeVar132;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar133 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar134 = ( radiance * nodeVar133 );
	nodeVar135 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar136 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar137 = ( nodeVar135 * nodeVar136 );
	nodeVar138 = ( nodeVar134 + nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar141 = ( vec3<f32>( 1.0 ) - nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( DiffuseContribution * nodeVar142 );
	nodeVar144 = ( nodeVar143 * nodeVar136 );
	nodeVar145 = nodeVar144;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar146 = ( indirectSpecular + nodeVar139 );
	indirectSpecular = nodeVar146;
	nodeVar147 = ( indirectDiffuse + nodeVar145 );
	indirectDiffuse = nodeVar147;
	ambientOcclusion = 1.0;
	nodeVar148 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar148;
	nodeVar149 = dot( normalView, positionViewDirection );
	nodeVar150 = ( clamp( nodeVar149, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar151 = ( Roughness * -16.0 );
	nodeVar152 = ( 1.0 - nodeVar151 );
	nodeVar153 = nodeVar152;
	nodeVar154 = ( - nodeVar153 );
	nodeVar155 = exp2( nodeVar154 );
	nodeVar156 = pow( nodeVar150, nodeVar155 );
	nodeVar157 = ( 1.0 - nodeVar156 );
	nodeVar158 = nodeVar157;
	nodeVar159 = ( ambientOcclusion - nodeVar158 );
	nodeVar160 = ( indirectSpecular * vec3<f32>( clamp( nodeVar159, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar160;
	nodeVar161 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar161;
	nodeVar162 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar162;
	nodeVar163 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar163;
	nodeVar164 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar164;

	// result

	output.color = nodeVar164;

	return output;

}
