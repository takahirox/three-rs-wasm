// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform5 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform25_sampler : sampler_comparison;
@binding( 10 ) @group( 1 ) var nodeUniform25 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : mat3x3<f32>,
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
	nodeUniform21 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform22 : mat4x4<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform28 : f32,
	nodeUniform26 : f32,
	nodeUniform27 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
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
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec4<f32>;

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
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_bitangentView : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 6 ) nodeVarying10 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform5, nodeUniform5_sampler, ( object.nodeUniform6 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform4 * nodeVar1.z );
	nodeVar2 = textureSample( nodeUniform5, nodeUniform5_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform7 * nodeVar2.y ), 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform4 * nodeVar1.z ) ) ) );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	NORMAL_tangentView = normalize( v_tangentView );
	NORMAL_bitangentView = normalize( NORMAL_v_bitangentView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar5 = textureSample( nodeUniform15, nodeUniform15_sampler, ( object.nodeUniform16 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	nodeVar6 = ( ( nodeVar5 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar6.xy * object.nodeUniform17 ), nodeVar6.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar7 = dot( normalView, positionViewDirection );
	nodeVar8 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( nodeVar7, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar8;
	nodeVar9 = ( dfg.x + dfg.y );
	nodeVar10 = ( 1.0 / nodeVar9 );
	nodeVar11 = nodeVar10;
	nodeVar12 = ( nodeVar11 - 1.0 );
	nodeVar13 = ( SpecularColorBlended * vec3<f32>( nodeVar12 ) );
	nodeVar14 = ( nodeVar13 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar14;
	nodeVar15 = ( render.nodeUniform19 - render.nodeUniform20 );
	nodeVar16 = vec4<f32>( nodeVar15, 0.0 );
	nodeVar17 = ( render.cameraViewMatrix * nodeVar16 );
	nodeVar18 = normalize( nodeVar17.xyz );
	nodeVar19 = nodeVar18;
	nodeVar20 = dot( normalView, nodeVar19 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar22 = ( render.nodeUniform22 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform23 ) ) ), 1.0 ) );
	nodeVar23 = ( nodeVar22.xyz / vec3<f32>( nodeVar22.w ) );
	nodeVar24 = vec3<f32>( nodeVar23.x, ( 1.0 - nodeVar23.y ), ( nodeVar23.z + render.nodeUniform24 ) );

	if ( ( ( ( ( ( nodeVar24.x >= 0.0 ) && ( nodeVar24.x <= 1.0 ) ) && ( nodeVar24.y >= 0.0 ) ) && ( nodeVar24.y <= 1.0 ) ) && ( nodeVar24.z <= 1.0 ) ) ) {

		nodeVar25 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar26 = ( render.nodeUniform26 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform27 ).x );
		nodeVar27 = ( nodeVar24.xy + ( vogelDiskSample( 0, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar28 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeVar27, nodeVar24.z );
		nodeVar29 = ( nodeVar24.xy + ( vogelDiskSample( 1, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar30 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeVar29, nodeVar24.z );
		nodeVar31 = ( nodeVar24.xy + ( vogelDiskSample( 2, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar32 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeVar31, nodeVar24.z );
		nodeVar33 = ( nodeVar24.xy + ( vogelDiskSample( 3, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar34 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeVar33, nodeVar24.z );
		nodeVar35 = ( nodeVar24.xy + ( vogelDiskSample( 4, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar36 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeVar35, nodeVar24.z );
		nodeVar21 = ( ( ( ( ( nodeVar28 + nodeVar30 ) + nodeVar32 ) + nodeVar34 ) + nodeVar36 ) * 0.2 );

	} else {

		nodeVar21 = 1.0;

	}

	nodeVar37 = mix( 1.0, nodeVar21, render.nodeUniform28 );
	nodeVar38 = ( vec3<f32>( clamp( nodeVar20, 0.0, 1.0 ) ) * ( render.nodeUniform21 * vec3<f32>( nodeVar37 ) ) );
	nodeVar39 = nodeVar38;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar40 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar41 = ( nodeVar39 * nodeVar40 );
	nodeVar42 = ( nodeVar19 + positionViewDirection );
	nodeVar43 = normalize( nodeVar42 );
	nodeVar44 = dot( positionViewDirection, nodeVar43 );
	nodeVar45 = clamp( nodeVar44, 0.0, 1.0 );
	nodeVar46 = exp2( ( ( ( nodeVar45 * -5.55473 ) - 6.98316 ) * nodeVar45 ) );
	nodeVar47 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar46 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar46 ) ) );
	nodeVar48 = ( vec3<f32>( 1.0 ) - nodeVar47 );
	nodeVar49 = nodeVar48;
	nodeVar50 = ( nodeVar41 * nodeVar49 );
	nodeVar51 = ( directDiffuse + nodeVar50 );
	directDiffuse = nodeVar51;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar52 = normalize( ( nodeVar19 + positionViewDirection ) );
	nodeVar53 = clamp( dot( positionViewDirection, nodeVar52 ), 0.0, 1.0 );
	nodeVar54 = exp2( ( ( ( nodeVar53 * -5.55473 ) - 6.98316 ) * nodeVar53 ) );
	nodeVar55 = ( Roughness * Roughness );
	nodeVar56 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar54 ) ) ) + vec3<f32>( ( 1.0 * nodeVar54 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar55, clamp( dot( normalView, nodeVar19 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar55, clamp( dot( normalView, nodeVar52 ), 0.0, 1.0 ) ) ) );
	nodeVar57 = ( nodeVar39 * nodeVar56 );
	nodeVar58 = ( nodeVar57 * multiScatteringCompensation );
	nodeVar59 = ( directSpecular + nodeVar58 );
	directSpecular = nodeVar59;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar60 = ( irradiance + render.nodeUniform29 );
	irradiance = nodeVar60;
	nodeVar61 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar62 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar63 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar64 = ( SpecularF90 * dfg.y );
	nodeVar65 = ( nodeVar63 + vec3<f32>( nodeVar64 ) );
	nodeVar66 = ( nodeVar61 + nodeVar65 );
	nodeVar61 = nodeVar66;
	nodeVar67 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar68 = nodeVar67;
	nodeVar69 = ( nodeVar68 * vec3<f32>( 0.047619 ) );
	nodeVar70 = ( SpecularColor + nodeVar69 );
	nodeVar71 = ( nodeVar65 * nodeVar70 );
	nodeVar72 = ( dfg.x + dfg.y );
	nodeVar73 = ( 1.0 - nodeVar72 );
	nodeVar74 = nodeVar73;
	nodeVar75 = ( vec3<f32>( nodeVar74 ) * nodeVar70 );
	nodeVar76 = ( vec3<f32>( 1.0 ) - nodeVar75 );
	nodeVar77 = nodeVar76;
	nodeVar78 = ( nodeVar71 / nodeVar77 );
	nodeVar79 = ( nodeVar78 * vec3<f32>( nodeVar74 ) );
	nodeVar80 = ( nodeVar62 + nodeVar79 );
	nodeVar62 = nodeVar80;
	nodeVar81 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar82 = ( irradiance * nodeVar81 );
	nodeVar83 = ( nodeVar61 + nodeVar62 );
	nodeVar84 = ( vec3<f32>( 1.0 ) - nodeVar83 );
	nodeVar85 = nodeVar84;
	nodeVar86 = ( nodeVar82 * nodeVar85 );
	nodeVar87 = nodeVar86;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar88 = ( indirectDiffuse + nodeVar87 );
	indirectDiffuse = nodeVar88;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar89 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar90 = ( SpecularF90 * dfg.y );
	nodeVar91 = ( nodeVar89 + vec3<f32>( nodeVar90 ) );
	nodeVar92 = ( singleScatteringDielectric + nodeVar91 );
	singleScatteringDielectric = nodeVar92;
	nodeVar93 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar94 = nodeVar93;
	nodeVar95 = ( nodeVar94 * vec3<f32>( 0.047619 ) );
	nodeVar96 = ( SpecularColor + nodeVar95 );
	nodeVar97 = ( nodeVar91 * nodeVar96 );
	nodeVar98 = ( dfg.x + dfg.y );
	nodeVar99 = ( 1.0 - nodeVar98 );
	nodeVar100 = nodeVar99;
	nodeVar101 = ( vec3<f32>( nodeVar100 ) * nodeVar96 );
	nodeVar102 = ( vec3<f32>( 1.0 ) - nodeVar101 );
	nodeVar103 = nodeVar102;
	nodeVar104 = ( nodeVar97 / nodeVar103 );
	nodeVar105 = ( nodeVar104 * vec3<f32>( nodeVar100 ) );
	nodeVar106 = ( multiScatteringDielectric + nodeVar105 );
	multiScatteringDielectric = nodeVar106;
	nodeVar107 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar108 = ( SpecularF90 * dfg.y );
	nodeVar109 = ( nodeVar107 + vec3<f32>( nodeVar108 ) );
	nodeVar110 = ( singleScatteringMetallic + nodeVar109 );
	singleScatteringMetallic = nodeVar110;
	nodeVar111 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( nodeVar112 * vec3<f32>( 0.047619 ) );
	nodeVar114 = ( DiffuseColor.xyz + nodeVar113 );
	nodeVar115 = ( nodeVar109 * nodeVar114 );
	nodeVar116 = ( dfg.x + dfg.y );
	nodeVar117 = ( 1.0 - nodeVar116 );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( vec3<f32>( nodeVar118 ) * nodeVar114 );
	nodeVar120 = ( vec3<f32>( 1.0 ) - nodeVar119 );
	nodeVar121 = nodeVar120;
	nodeVar122 = ( nodeVar115 / nodeVar121 );
	nodeVar123 = ( nodeVar122 * vec3<f32>( nodeVar118 ) );
	nodeVar124 = ( multiScatteringMetallic + nodeVar123 );
	multiScatteringMetallic = nodeVar124;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar125 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar126 = ( radiance * nodeVar125 );
	nodeVar127 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar128 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar129 = ( nodeVar127 * nodeVar128 );
	nodeVar130 = ( nodeVar126 + nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar133 = ( vec3<f32>( 1.0 ) - nodeVar132 );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( DiffuseContribution * nodeVar134 );
	nodeVar136 = ( nodeVar135 * nodeVar128 );
	nodeVar137 = nodeVar136;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar138 = ( indirectSpecular + nodeVar131 );
	indirectSpecular = nodeVar138;
	nodeVar139 = ( indirectDiffuse + nodeVar137 );
	indirectDiffuse = nodeVar139;
	ambientOcclusion = 1.0;
	nodeVar140 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar140;
	nodeVar141 = dot( normalView, positionViewDirection );
	nodeVar142 = ( clamp( nodeVar141, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar143 = ( Roughness * -16.0 );
	nodeVar144 = ( 1.0 - nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( - nodeVar145 );
	nodeVar147 = exp2( nodeVar146 );
	nodeVar148 = pow( nodeVar142, nodeVar147 );
	nodeVar149 = ( 1.0 - nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( ambientOcclusion - nodeVar150 );
	nodeVar152 = ( indirectSpecular * vec3<f32>( clamp( nodeVar151, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar152;
	nodeVar153 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar153;
	nodeVar154 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar154;
	nodeVar155 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar155;
	nodeVar156 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar156;

	// result

	output.color = nodeVar156;

	return output;

}
