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
var<private> nodeVar4 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar5 : f32;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec4<f32>;
var<private> nodeVar31 : vec4<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
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
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec4<f32>;
var<private> nodeVar59 : vec4<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec4<f32>;

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
	@location( 3 ) nodeVarying6 : vec2<f32>,
	@builtin( front_facing ) isFront : bool ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform3, 1.0 ) * nodeVar1 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform11 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform10 * nodeVar3.y ), 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	EmissiveColor = ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) );
	nodeVar5 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeVar5 ) );
	nodeVar6 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	nodeVar7 = dpdx( nodeVarying6 );
	nodeVar8 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	nodeVar9 = - dpdy( nodeVarying6 );
	nodeVar10 = ( ( nodeVar6 * vec3<f32>( nodeVar7.x ) ) + ( nodeVar8 * vec3<f32>( nodeVar9.x ) ) );
	nodeVar12 = ( ( nodeVar6 * vec3<f32>( nodeVar7.y ) ) + ( nodeVar8 * vec3<f32>( nodeVar9.y ) ) );
	nodeVar13 = max( dot( nodeVar10, nodeVar10 ), dot( nodeVar12, nodeVar12 ) );

	if ( ( nodeVar13 == 0.0 ) ) {

		nodeVar11 = 0.0;

	} else {

		nodeVar11 = inverseSqrt( nodeVar13 );

	}

	tangentViewFrame = ( nodeVar10 * vec3<f32>( nodeVar11 ) );
	NORMAL_tangentView = ( tangentViewFrame * vec3<f32>( nodeVar5 ) );
	bitangentViewFrame = ( nodeVar12 * nodeVar11 );
	NORMAL_bitangentView = ( bitangentViewFrame * vec3<f32>( nodeVar5 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar14 = textureSample( nodeUniform18, nodeUniform18_sampler, ( object.nodeUniform19 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	nodeVar15 = ( ( nodeVar14 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar15.xy * object.nodeUniform20 ), nodeVar15.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar16 = dot( normalView, positionViewDirection );
	nodeVar17 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( nodeVar16, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar17;
	nodeVar18 = ( dfg.x + dfg.y );
	nodeVar19 = ( 1.0 / nodeVar18 );
	nodeVar20 = nodeVar19;
	nodeVar21 = ( nodeVar20 - 1.0 );
	nodeVar22 = ( SpecularColorBlended * vec3<f32>( nodeVar21 ) );
	nodeVar23 = ( nodeVar22 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar23;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar24 = dot( normalWorld, normalize( render.nodeUniform24 ) );
	nodeVar25 = ( nodeVar24 * 0.5 );
	nodeVar26 = ( nodeVar25 + 0.5 );
	nodeVar27 = mix( render.nodeUniform21, render.nodeUniform22, nodeVar26 );
	nodeVar28 = ( irradiance + nodeVar27 );
	irradiance = nodeVar28;
	nodeVar29 = ( render.nodeUniform25 - render.nodeUniform26 );
	nodeVar30 = vec4<f32>( nodeVar29, 0.0 );
	nodeVar31 = ( render.cameraViewMatrix * nodeVar30 );
	nodeVar32 = normalize( nodeVar31.xyz );
	nodeVar33 = nodeVar32;
	nodeVar34 = dot( normalView, nodeVar33 );
	nodeVar35 = ( vec3<f32>( clamp( nodeVar34, 0.0, 1.0 ) ) * render.nodeUniform27 );
	nodeVar36 = nodeVar35;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar37 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar38 = ( nodeVar36 * nodeVar37 );
	nodeVar39 = ( nodeVar33 + positionViewDirection );
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
	nodeVar49 = normalize( ( nodeVar33 + positionViewDirection ) );
	nodeVar50 = clamp( dot( positionViewDirection, nodeVar49 ), 0.0, 1.0 );
	nodeVar51 = exp2( ( ( ( nodeVar50 * -5.55473 ) - 6.98316 ) * nodeVar50 ) );
	nodeVar52 = ( Roughness * Roughness );
	nodeVar53 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar51 ) ) ) + vec3<f32>( ( 1.0 * nodeVar51 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar52, clamp( dot( normalView, nodeVar33 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar52, clamp( dot( normalView, nodeVar49 ), 0.0, 1.0 ) ) ) );
	nodeVar54 = ( nodeVar36 * nodeVar53 );
	nodeVar55 = ( nodeVar54 * multiScatteringCompensation );
	nodeVar56 = ( directSpecular + nodeVar55 );
	directSpecular = nodeVar56;
	nodeVar57 = ( render.nodeUniform28 - render.nodeUniform29 );
	nodeVar58 = vec4<f32>( nodeVar57, 0.0 );
	nodeVar59 = ( render.cameraViewMatrix * nodeVar58 );
	nodeVar60 = normalize( nodeVar59.xyz );
	nodeVar61 = nodeVar60;
	nodeVar62 = dot( normalView, nodeVar61 );
	nodeVar63 = ( vec3<f32>( clamp( nodeVar62, 0.0, 1.0 ) ) * render.nodeUniform30 );
	nodeVar64 = nodeVar63;
	nodeVar65 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar66 = ( nodeVar64 * nodeVar65 );
	nodeVar67 = ( nodeVar61 + positionViewDirection );
	nodeVar68 = normalize( nodeVar67 );
	nodeVar69 = dot( positionViewDirection, nodeVar68 );
	nodeVar70 = clamp( nodeVar69, 0.0, 1.0 );
	nodeVar71 = exp2( ( ( ( nodeVar70 * -5.55473 ) - 6.98316 ) * nodeVar70 ) );
	nodeVar72 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar71 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar71 ) ) );
	nodeVar73 = ( vec3<f32>( 1.0 ) - nodeVar72 );
	nodeVar74 = nodeVar73;
	nodeVar75 = ( nodeVar66 * nodeVar74 );
	nodeVar76 = ( directDiffuse + nodeVar75 );
	directDiffuse = nodeVar76;
	nodeVar77 = normalize( ( nodeVar61 + positionViewDirection ) );
	nodeVar78 = clamp( dot( positionViewDirection, nodeVar77 ), 0.0, 1.0 );
	nodeVar79 = exp2( ( ( ( nodeVar78 * -5.55473 ) - 6.98316 ) * nodeVar78 ) );
	nodeVar80 = ( Roughness * Roughness );
	nodeVar81 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar79 ) ) ) + vec3<f32>( ( 1.0 * nodeVar79 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar80, clamp( dot( normalView, nodeVar61 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar80, clamp( dot( normalView, nodeVar77 ), 0.0, 1.0 ) ) ) );
	nodeVar82 = ( nodeVar64 * nodeVar81 );
	nodeVar83 = ( nodeVar82 * multiScatteringCompensation );
	nodeVar84 = ( directSpecular + nodeVar83 );
	directSpecular = nodeVar84;
	nodeVar85 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar86 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar87 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar88 = ( SpecularF90 * dfg.y );
	nodeVar89 = ( nodeVar87 + vec3<f32>( nodeVar88 ) );
	nodeVar90 = ( nodeVar85 + nodeVar89 );
	nodeVar85 = nodeVar90;
	nodeVar91 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar92 = nodeVar91;
	nodeVar93 = ( nodeVar92 * vec3<f32>( 0.047619 ) );
	nodeVar94 = ( SpecularColor + nodeVar93 );
	nodeVar95 = ( nodeVar89 * nodeVar94 );
	nodeVar96 = ( dfg.x + dfg.y );
	nodeVar97 = ( 1.0 - nodeVar96 );
	nodeVar98 = nodeVar97;
	nodeVar99 = ( vec3<f32>( nodeVar98 ) * nodeVar94 );
	nodeVar100 = ( vec3<f32>( 1.0 ) - nodeVar99 );
	nodeVar101 = nodeVar100;
	nodeVar102 = ( nodeVar95 / nodeVar101 );
	nodeVar103 = ( nodeVar102 * vec3<f32>( nodeVar98 ) );
	nodeVar104 = ( nodeVar86 + nodeVar103 );
	nodeVar86 = nodeVar104;
	nodeVar105 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar106 = ( irradiance * nodeVar105 );
	nodeVar107 = ( nodeVar85 + nodeVar86 );
	nodeVar108 = ( vec3<f32>( 1.0 ) - nodeVar107 );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( nodeVar106 * nodeVar109 );
	nodeVar111 = nodeVar110;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar112 = ( indirectDiffuse + nodeVar111 );
	indirectDiffuse = nodeVar112;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar113 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar114 = ( SpecularF90 * dfg.y );
	nodeVar115 = ( nodeVar113 + vec3<f32>( nodeVar114 ) );
	nodeVar116 = ( singleScatteringDielectric + nodeVar115 );
	singleScatteringDielectric = nodeVar116;
	nodeVar117 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar118 * vec3<f32>( 0.047619 ) );
	nodeVar120 = ( SpecularColor + nodeVar119 );
	nodeVar121 = ( nodeVar115 * nodeVar120 );
	nodeVar122 = ( dfg.x + dfg.y );
	nodeVar123 = ( 1.0 - nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( vec3<f32>( nodeVar124 ) * nodeVar120 );
	nodeVar126 = ( vec3<f32>( 1.0 ) - nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( nodeVar121 / nodeVar127 );
	nodeVar129 = ( nodeVar128 * vec3<f32>( nodeVar124 ) );
	nodeVar130 = ( multiScatteringDielectric + nodeVar129 );
	multiScatteringDielectric = nodeVar130;
	nodeVar131 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar132 = ( SpecularF90 * dfg.y );
	nodeVar133 = ( nodeVar131 + vec3<f32>( nodeVar132 ) );
	nodeVar134 = ( singleScatteringMetallic + nodeVar133 );
	singleScatteringMetallic = nodeVar134;
	nodeVar135 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( nodeVar136 * vec3<f32>( 0.047619 ) );
	nodeVar138 = ( DiffuseColor.xyz + nodeVar137 );
	nodeVar139 = ( nodeVar133 * nodeVar138 );
	nodeVar140 = ( dfg.x + dfg.y );
	nodeVar141 = ( 1.0 - nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( vec3<f32>( nodeVar142 ) * nodeVar138 );
	nodeVar144 = ( vec3<f32>( 1.0 ) - nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( nodeVar139 / nodeVar145 );
	nodeVar147 = ( nodeVar146 * vec3<f32>( nodeVar142 ) );
	nodeVar148 = ( multiScatteringMetallic + nodeVar147 );
	multiScatteringMetallic = nodeVar148;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar149 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar150 = ( radiance * nodeVar149 );
	nodeVar151 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar152 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar153 = ( nodeVar151 * nodeVar152 );
	nodeVar154 = ( nodeVar150 + nodeVar153 );
	nodeVar155 = nodeVar154;
	nodeVar156 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar157 = ( vec3<f32>( 1.0 ) - nodeVar156 );
	nodeVar158 = nodeVar157;
	nodeVar159 = ( DiffuseContribution * nodeVar158 );
	nodeVar160 = ( nodeVar159 * nodeVar152 );
	nodeVar161 = nodeVar160;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar162 = ( indirectSpecular + nodeVar155 );
	indirectSpecular = nodeVar162;
	nodeVar163 = ( indirectDiffuse + nodeVar161 );
	indirectDiffuse = nodeVar163;
	ambientOcclusion = 1.0;
	nodeVar164 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar164;
	nodeVar165 = dot( normalView, positionViewDirection );
	nodeVar166 = ( clamp( nodeVar165, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar167 = ( Roughness * -16.0 );
	nodeVar168 = ( 1.0 - nodeVar167 );
	nodeVar169 = nodeVar168;
	nodeVar170 = ( - nodeVar169 );
	nodeVar171 = exp2( nodeVar170 );
	nodeVar172 = pow( nodeVar166, nodeVar171 );
	nodeVar173 = ( 1.0 - nodeVar172 );
	nodeVar174 = nodeVar173;
	nodeVar175 = ( ambientOcclusion - nodeVar174 );
	nodeVar176 = ( indirectSpecular * vec3<f32>( clamp( nodeVar175, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar176;
	nodeVar177 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar177;
	nodeVar178 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar178;
	nodeVar179 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar179;
	nodeVar180 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar180;

	// result

	output.color = nodeVar180;

	return output;

}
