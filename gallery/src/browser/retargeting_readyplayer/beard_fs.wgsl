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
@binding( 5 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;

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
var<private> nodeVar11 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec4<f32>;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec4<f32>;
var<private> nodeVar66 : vec4<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
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
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec4<f32>;

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

	nodeVar11 = textureSample( nodeUniform7, nodeUniform7_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform6, 1.0 ) * nodeVar11 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform9 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform10;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar12 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform11, 0.0525 ) + max( max( nodeVar12.x, nodeVar12.y ), nodeVar12.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform10 ) ) );
	EmissiveColor = ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) );
	NORMAL_normalView = normalViewGeometry;
	nodeVar13 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	nodeVar14 = dpdx( nodeVarying6 );
	nodeVar15 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	nodeVar16 = - dpdy( nodeVarying6 );
	nodeVar17 = ( ( nodeVar13 * vec3<f32>( nodeVar14.x ) ) + ( nodeVar15 * vec3<f32>( nodeVar16.x ) ) );
	nodeVar19 = ( ( nodeVar13 * vec3<f32>( nodeVar14.y ) ) + ( nodeVar15 * vec3<f32>( nodeVar16.y ) ) );
	nodeVar20 = max( dot( nodeVar17, nodeVar17 ), dot( nodeVar19, nodeVar19 ) );

	if ( ( nodeVar20 == 0.0 ) ) {

		nodeVar18 = 0.0;

	} else {

		nodeVar18 = inverseSqrt( nodeVar20 );

	}

	tangentViewFrame = ( nodeVar17 * vec3<f32>( nodeVar18 ) );
	NORMAL_tangentView = tangentViewFrame;
	bitangentViewFrame = ( nodeVar19 * nodeVar18 );
	NORMAL_bitangentView = bitangentViewFrame;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar21 = textureSample( nodeUniform18, nodeUniform18_sampler, ( object.nodeUniform19 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	nodeVar22 = ( ( nodeVar21 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar22.xy * object.nodeUniform20 ), nodeVar22.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar23 = dot( normalView, positionViewDirection );
	nodeVar24 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( nodeVar23, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar24;
	nodeVar25 = ( dfg.x + dfg.y );
	nodeVar26 = ( 1.0 / nodeVar25 );
	nodeVar27 = nodeVar26;
	nodeVar28 = ( nodeVar27 - 1.0 );
	nodeVar29 = ( SpecularColorBlended * vec3<f32>( nodeVar28 ) );
	nodeVar30 = ( nodeVar29 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar30;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar31 = dot( normalWorld, normalize( render.nodeUniform24 ) );
	nodeVar32 = ( nodeVar31 * 0.5 );
	nodeVar33 = ( nodeVar32 + 0.5 );
	nodeVar34 = mix( render.nodeUniform21, render.nodeUniform22, nodeVar33 );
	nodeVar35 = ( irradiance + nodeVar34 );
	irradiance = nodeVar35;
	nodeVar36 = ( render.nodeUniform25 - render.nodeUniform26 );
	nodeVar37 = vec4<f32>( nodeVar36, 0.0 );
	nodeVar38 = ( render.cameraViewMatrix * nodeVar37 );
	nodeVar39 = normalize( nodeVar38.xyz );
	nodeVar40 = nodeVar39;
	nodeVar41 = dot( normalView, nodeVar40 );
	nodeVar42 = ( vec3<f32>( clamp( nodeVar41, 0.0, 1.0 ) ) * render.nodeUniform27 );
	nodeVar43 = nodeVar42;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar44 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar45 = ( nodeVar43 * nodeVar44 );
	nodeVar46 = ( nodeVar40 + positionViewDirection );
	nodeVar47 = normalize( nodeVar46 );
	nodeVar48 = dot( positionViewDirection, nodeVar47 );
	nodeVar49 = clamp( nodeVar48, 0.0, 1.0 );
	nodeVar50 = exp2( ( ( ( nodeVar49 * -5.55473 ) - 6.98316 ) * nodeVar49 ) );
	nodeVar51 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar50 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar50 ) ) );
	nodeVar52 = ( vec3<f32>( 1.0 ) - nodeVar51 );
	nodeVar53 = nodeVar52;
	nodeVar54 = ( nodeVar45 * nodeVar53 );
	nodeVar55 = ( directDiffuse + nodeVar54 );
	directDiffuse = nodeVar55;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar56 = normalize( ( nodeVar40 + positionViewDirection ) );
	nodeVar57 = clamp( dot( positionViewDirection, nodeVar56 ), 0.0, 1.0 );
	nodeVar58 = exp2( ( ( ( nodeVar57 * -5.55473 ) - 6.98316 ) * nodeVar57 ) );
	nodeVar59 = ( Roughness * Roughness );
	nodeVar60 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar58 ) ) ) + vec3<f32>( ( 1.0 * nodeVar58 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar59, clamp( dot( normalView, nodeVar40 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar59, clamp( dot( normalView, nodeVar56 ), 0.0, 1.0 ) ) ) );
	nodeVar61 = ( nodeVar43 * nodeVar60 );
	nodeVar62 = ( nodeVar61 * multiScatteringCompensation );
	nodeVar63 = ( directSpecular + nodeVar62 );
	directSpecular = nodeVar63;
	nodeVar64 = ( render.nodeUniform28 - render.nodeUniform29 );
	nodeVar65 = vec4<f32>( nodeVar64, 0.0 );
	nodeVar66 = ( render.cameraViewMatrix * nodeVar65 );
	nodeVar67 = normalize( nodeVar66.xyz );
	nodeVar68 = nodeVar67;
	nodeVar69 = dot( normalView, nodeVar68 );
	nodeVar70 = ( vec3<f32>( clamp( nodeVar69, 0.0, 1.0 ) ) * render.nodeUniform30 );
	nodeVar71 = nodeVar70;
	nodeVar72 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar73 = ( nodeVar71 * nodeVar72 );
	nodeVar74 = ( nodeVar68 + positionViewDirection );
	nodeVar75 = normalize( nodeVar74 );
	nodeVar76 = dot( positionViewDirection, nodeVar75 );
	nodeVar77 = clamp( nodeVar76, 0.0, 1.0 );
	nodeVar78 = exp2( ( ( ( nodeVar77 * -5.55473 ) - 6.98316 ) * nodeVar77 ) );
	nodeVar79 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar78 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar78 ) ) );
	nodeVar80 = ( vec3<f32>( 1.0 ) - nodeVar79 );
	nodeVar81 = nodeVar80;
	nodeVar82 = ( nodeVar73 * nodeVar81 );
	nodeVar83 = ( directDiffuse + nodeVar82 );
	directDiffuse = nodeVar83;
	nodeVar84 = normalize( ( nodeVar68 + positionViewDirection ) );
	nodeVar85 = clamp( dot( positionViewDirection, nodeVar84 ), 0.0, 1.0 );
	nodeVar86 = exp2( ( ( ( nodeVar85 * -5.55473 ) - 6.98316 ) * nodeVar85 ) );
	nodeVar87 = ( Roughness * Roughness );
	nodeVar88 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar86 ) ) ) + vec3<f32>( ( 1.0 * nodeVar86 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar87, clamp( dot( normalView, nodeVar68 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar87, clamp( dot( normalView, nodeVar84 ), 0.0, 1.0 ) ) ) );
	nodeVar89 = ( nodeVar71 * nodeVar88 );
	nodeVar90 = ( nodeVar89 * multiScatteringCompensation );
	nodeVar91 = ( directSpecular + nodeVar90 );
	directSpecular = nodeVar91;
	nodeVar92 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar93 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar95 = ( SpecularF90 * dfg.y );
	nodeVar96 = ( nodeVar94 + vec3<f32>( nodeVar95 ) );
	nodeVar97 = ( nodeVar92 + nodeVar96 );
	nodeVar92 = nodeVar97;
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
	nodeVar111 = ( nodeVar93 + nodeVar110 );
	nodeVar93 = nodeVar111;
	nodeVar112 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar113 = ( irradiance * nodeVar112 );
	nodeVar114 = ( nodeVar92 + nodeVar93 );
	nodeVar115 = ( vec3<f32>( 1.0 ) - nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( nodeVar113 * nodeVar116 );
	nodeVar118 = nodeVar117;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar119 = ( indirectDiffuse + nodeVar118 );
	indirectDiffuse = nodeVar119;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar120 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar121 = ( SpecularF90 * dfg.y );
	nodeVar122 = ( nodeVar120 + vec3<f32>( nodeVar121 ) );
	nodeVar123 = ( singleScatteringDielectric + nodeVar122 );
	singleScatteringDielectric = nodeVar123;
	nodeVar124 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( nodeVar125 * vec3<f32>( 0.047619 ) );
	nodeVar127 = ( SpecularColor + nodeVar126 );
	nodeVar128 = ( nodeVar122 * nodeVar127 );
	nodeVar129 = ( dfg.x + dfg.y );
	nodeVar130 = ( 1.0 - nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( vec3<f32>( nodeVar131 ) * nodeVar127 );
	nodeVar133 = ( vec3<f32>( 1.0 ) - nodeVar132 );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( nodeVar128 / nodeVar134 );
	nodeVar136 = ( nodeVar135 * vec3<f32>( nodeVar131 ) );
	nodeVar137 = ( multiScatteringDielectric + nodeVar136 );
	multiScatteringDielectric = nodeVar137;
	nodeVar138 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar139 = ( SpecularF90 * dfg.y );
	nodeVar140 = ( nodeVar138 + vec3<f32>( nodeVar139 ) );
	nodeVar141 = ( singleScatteringMetallic + nodeVar140 );
	singleScatteringMetallic = nodeVar141;
	nodeVar142 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar143 = nodeVar142;
	nodeVar144 = ( nodeVar143 * vec3<f32>( 0.047619 ) );
	nodeVar145 = ( DiffuseColor.xyz + nodeVar144 );
	nodeVar146 = ( nodeVar140 * nodeVar145 );
	nodeVar147 = ( dfg.x + dfg.y );
	nodeVar148 = ( 1.0 - nodeVar147 );
	nodeVar149 = nodeVar148;
	nodeVar150 = ( vec3<f32>( nodeVar149 ) * nodeVar145 );
	nodeVar151 = ( vec3<f32>( 1.0 ) - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( nodeVar146 / nodeVar152 );
	nodeVar154 = ( nodeVar153 * vec3<f32>( nodeVar149 ) );
	nodeVar155 = ( multiScatteringMetallic + nodeVar154 );
	multiScatteringMetallic = nodeVar155;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar156 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar157 = ( radiance * nodeVar156 );
	nodeVar158 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar159 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar160 = ( nodeVar158 * nodeVar159 );
	nodeVar161 = ( nodeVar157 + nodeVar160 );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar164 = ( vec3<f32>( 1.0 ) - nodeVar163 );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( DiffuseContribution * nodeVar165 );
	nodeVar167 = ( nodeVar166 * nodeVar159 );
	nodeVar168 = nodeVar167;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar169 = ( indirectSpecular + nodeVar162 );
	indirectSpecular = nodeVar169;
	nodeVar170 = ( indirectDiffuse + nodeVar168 );
	indirectDiffuse = nodeVar170;
	ambientOcclusion = 1.0;
	nodeVar171 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar171;
	nodeVar172 = dot( normalView, positionViewDirection );
	nodeVar173 = ( clamp( nodeVar172, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar174 = ( Roughness * -16.0 );
	nodeVar175 = ( 1.0 - nodeVar174 );
	nodeVar176 = nodeVar175;
	nodeVar177 = ( - nodeVar176 );
	nodeVar178 = exp2( nodeVar177 );
	nodeVar179 = pow( nodeVar173, nodeVar178 );
	nodeVar180 = ( 1.0 - nodeVar179 );
	nodeVar181 = nodeVar180;
	nodeVar182 = ( ambientOcclusion - nodeVar181 );
	nodeVar183 = ( indirectSpecular * vec3<f32>( clamp( nodeVar182, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar183;
	nodeVar184 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar184;
	nodeVar185 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar185;
	nodeVar186 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar186;
	nodeVar187 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar187;

	// result

	output.color = nodeVar187;

	return output;

}
