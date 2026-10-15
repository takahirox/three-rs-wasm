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
@binding( 3 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform14 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform26_sampler : sampler_comparison;
@binding( 10 ) @group( 1 ) var nodeUniform26 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform29 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : vec2<f32>
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
var<private> nodeVar5 : f32;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
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
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
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
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec4<f32>;

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
	@builtin( front_facing ) isFront : bool,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );

	if ( ( DiffuseColor.w <= object.nodeUniform4 ) ) {

		discard;
		

	}

	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform6, nodeUniform6_sampler, ( object.nodeUniform7 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform5 * nodeVar1.z );
	nodeVar2 = textureSample( nodeUniform6, nodeUniform6_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform8 * nodeVar2.y ), 0.0525 ) + max( max( nodeVar4.x, nodeVar4.y ), nodeVar4.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform5 * nodeVar1.z ) ) ) );
	EmissiveColor = ( object.nodeUniform12 * vec3<f32>( object.nodeUniform13 ) );
	nodeVar5 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_tangentView = ( normalize( v_tangentView ) * vec3<f32>( nodeVar5 ) );
	NORMAL_bitangentView = ( normalize( NORMAL_v_bitangentView ) * vec3<f32>( nodeVar5 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeVar5 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar7 = textureSample( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	nodeVar8 = ( ( nodeVar7 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar8.xy * object.nodeUniform18 ), nodeVar8.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar9 = dot( normalView, positionViewDirection );
	nodeVar10 = textureSample( nodeUniform14, nodeUniform14_sampler, vec2<f32>( Roughness, clamp( nodeVar9, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar10;
	nodeVar11 = ( dfg.x + dfg.y );
	nodeVar12 = ( 1.0 / nodeVar11 );
	nodeVar13 = nodeVar12;
	nodeVar14 = ( nodeVar13 - 1.0 );
	nodeVar15 = ( SpecularColorBlended * vec3<f32>( nodeVar14 ) );
	nodeVar16 = ( nodeVar15 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar16;
	nodeVar17 = ( render.nodeUniform20 - render.nodeUniform21 );
	nodeVar18 = vec4<f32>( nodeVar17, 0.0 );
	nodeVar19 = ( render.cameraViewMatrix * nodeVar18 );
	nodeVar20 = normalize( nodeVar19.xyz );
	nodeVar21 = nodeVar20;
	nodeVar22 = dot( normalView, nodeVar21 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar24 = ( render.nodeUniform23 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform24 ) ) ), 1.0 ) );
	nodeVar25 = ( nodeVar24.xyz / vec3<f32>( nodeVar24.w ) );
	nodeVar26 = vec3<f32>( nodeVar25.x, ( 1.0 - nodeVar25.y ), ( nodeVar25.z + render.nodeUniform25 ) );

	if ( ( ( ( ( ( nodeVar26.x >= 0.0 ) && ( nodeVar26.x <= 1.0 ) ) && ( nodeVar26.y >= 0.0 ) ) && ( nodeVar26.y <= 1.0 ) ) && ( nodeVar26.z <= 1.0 ) ) ) {

		nodeVar27 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar28 = ( render.nodeUniform27 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform28 ).x );
		nodeVar29 = ( nodeVar26.xy + ( vogelDiskSample( 0, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
		nodeVar30 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar29, nodeVar26.z );
		nodeVar31 = ( nodeVar26.xy + ( vogelDiskSample( 1, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
		nodeVar32 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar31, nodeVar26.z );
		nodeVar33 = ( nodeVar26.xy + ( vogelDiskSample( 2, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
		nodeVar34 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar33, nodeVar26.z );
		nodeVar35 = ( nodeVar26.xy + ( vogelDiskSample( 3, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
		nodeVar36 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar35, nodeVar26.z );
		nodeVar37 = ( nodeVar26.xy + ( vogelDiskSample( 4, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
		nodeVar38 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar37, nodeVar26.z );
		nodeVar23 = ( ( ( ( ( nodeVar30 + nodeVar32 ) + nodeVar34 ) + nodeVar36 ) + nodeVar38 ) * 0.2 );

	} else {

		nodeVar23 = 1.0;

	}

	nodeVar39 = mix( 1.0, nodeVar23, render.nodeUniform29 );
	nodeVar40 = ( vec3<f32>( clamp( nodeVar22, 0.0, 1.0 ) ) * ( render.nodeUniform22 * vec3<f32>( nodeVar39 ) ) );
	nodeVar41 = nodeVar40;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar42 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar43 = ( nodeVar41 * nodeVar42 );
	nodeVar44 = ( nodeVar21 + positionViewDirection );
	nodeVar45 = normalize( nodeVar44 );
	nodeVar46 = dot( positionViewDirection, nodeVar45 );
	nodeVar47 = clamp( nodeVar46, 0.0, 1.0 );
	nodeVar48 = exp2( ( ( ( nodeVar47 * -5.55473 ) - 6.98316 ) * nodeVar47 ) );
	nodeVar49 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar48 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar48 ) ) );
	nodeVar50 = ( vec3<f32>( 1.0 ) - nodeVar49 );
	nodeVar51 = nodeVar50;
	nodeVar52 = ( nodeVar43 * nodeVar51 );
	nodeVar53 = ( directDiffuse + nodeVar52 );
	directDiffuse = nodeVar53;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar54 = normalize( ( nodeVar21 + positionViewDirection ) );
	nodeVar55 = clamp( dot( positionViewDirection, nodeVar54 ), 0.0, 1.0 );
	nodeVar56 = exp2( ( ( ( nodeVar55 * -5.55473 ) - 6.98316 ) * nodeVar55 ) );
	nodeVar57 = ( Roughness * Roughness );
	nodeVar58 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar56 ) ) ) + vec3<f32>( ( 1.0 * nodeVar56 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar57, clamp( dot( normalView, nodeVar21 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar57, clamp( dot( normalView, nodeVar54 ), 0.0, 1.0 ) ) ) );
	nodeVar59 = ( nodeVar41 * nodeVar58 );
	nodeVar60 = ( nodeVar59 * multiScatteringCompensation );
	nodeVar61 = ( directSpecular + nodeVar60 );
	directSpecular = nodeVar61;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar62 = ( irradiance + render.nodeUniform30 );
	irradiance = nodeVar62;
	nodeVar63 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar64 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar65 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar66 = ( SpecularF90 * dfg.y );
	nodeVar67 = ( nodeVar65 + vec3<f32>( nodeVar66 ) );
	nodeVar68 = ( nodeVar63 + nodeVar67 );
	nodeVar63 = nodeVar68;
	nodeVar69 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar70 = nodeVar69;
	nodeVar71 = ( nodeVar70 * vec3<f32>( 0.047619 ) );
	nodeVar72 = ( SpecularColor + nodeVar71 );
	nodeVar73 = ( nodeVar67 * nodeVar72 );
	nodeVar74 = ( dfg.x + dfg.y );
	nodeVar75 = ( 1.0 - nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = ( vec3<f32>( nodeVar76 ) * nodeVar72 );
	nodeVar78 = ( vec3<f32>( 1.0 ) - nodeVar77 );
	nodeVar79 = nodeVar78;
	nodeVar80 = ( nodeVar73 / nodeVar79 );
	nodeVar81 = ( nodeVar80 * vec3<f32>( nodeVar76 ) );
	nodeVar82 = ( nodeVar64 + nodeVar81 );
	nodeVar64 = nodeVar82;
	nodeVar83 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar84 = ( irradiance * nodeVar83 );
	nodeVar85 = ( nodeVar63 + nodeVar64 );
	nodeVar86 = ( vec3<f32>( 1.0 ) - nodeVar85 );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( nodeVar84 * nodeVar87 );
	nodeVar89 = nodeVar88;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar90 = ( indirectDiffuse + nodeVar89 );
	indirectDiffuse = nodeVar90;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar91 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar92 = ( SpecularF90 * dfg.y );
	nodeVar93 = ( nodeVar91 + vec3<f32>( nodeVar92 ) );
	nodeVar94 = ( singleScatteringDielectric + nodeVar93 );
	singleScatteringDielectric = nodeVar94;
	nodeVar95 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar96 = nodeVar95;
	nodeVar97 = ( nodeVar96 * vec3<f32>( 0.047619 ) );
	nodeVar98 = ( SpecularColor + nodeVar97 );
	nodeVar99 = ( nodeVar93 * nodeVar98 );
	nodeVar100 = ( dfg.x + dfg.y );
	nodeVar101 = ( 1.0 - nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( vec3<f32>( nodeVar102 ) * nodeVar98 );
	nodeVar104 = ( vec3<f32>( 1.0 ) - nodeVar103 );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( nodeVar99 / nodeVar105 );
	nodeVar107 = ( nodeVar106 * vec3<f32>( nodeVar102 ) );
	nodeVar108 = ( multiScatteringDielectric + nodeVar107 );
	multiScatteringDielectric = nodeVar108;
	nodeVar109 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar110 = ( SpecularF90 * dfg.y );
	nodeVar111 = ( nodeVar109 + vec3<f32>( nodeVar110 ) );
	nodeVar112 = ( singleScatteringMetallic + nodeVar111 );
	singleScatteringMetallic = nodeVar112;
	nodeVar113 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar114 * vec3<f32>( 0.047619 ) );
	nodeVar116 = ( DiffuseColor.xyz + nodeVar115 );
	nodeVar117 = ( nodeVar111 * nodeVar116 );
	nodeVar118 = ( dfg.x + dfg.y );
	nodeVar119 = ( 1.0 - nodeVar118 );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( vec3<f32>( nodeVar120 ) * nodeVar116 );
	nodeVar122 = ( vec3<f32>( 1.0 ) - nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( nodeVar117 / nodeVar123 );
	nodeVar125 = ( nodeVar124 * vec3<f32>( nodeVar120 ) );
	nodeVar126 = ( multiScatteringMetallic + nodeVar125 );
	multiScatteringMetallic = nodeVar126;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar127 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar128 = ( radiance * nodeVar127 );
	nodeVar129 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar130 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar131 = ( nodeVar129 * nodeVar130 );
	nodeVar132 = ( nodeVar128 + nodeVar131 );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar135 = ( vec3<f32>( 1.0 ) - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( DiffuseContribution * nodeVar136 );
	nodeVar138 = ( nodeVar137 * nodeVar130 );
	nodeVar139 = nodeVar138;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = ( indirectSpecular + nodeVar133 );
	indirectSpecular = nodeVar140;
	nodeVar141 = ( indirectDiffuse + nodeVar139 );
	indirectDiffuse = nodeVar141;
	ambientOcclusion = 1.0;
	nodeVar142 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar142;
	nodeVar143 = dot( normalView, positionViewDirection );
	nodeVar144 = ( clamp( nodeVar143, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar145 = ( Roughness * -16.0 );
	nodeVar146 = ( 1.0 - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( - nodeVar147 );
	nodeVar149 = exp2( nodeVar148 );
	nodeVar150 = pow( nodeVar144, nodeVar149 );
	nodeVar151 = ( 1.0 - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( ambientOcclusion - nodeVar152 );
	nodeVar154 = ( indirectSpecular * vec3<f32>( clamp( nodeVar153, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar154;
	nodeVar155 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar155;
	nodeVar156 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar156;
	nodeVar157 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar157;
	nodeVar158 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar158;

	// result

	output.color = nodeVar158;

	return output;

}
