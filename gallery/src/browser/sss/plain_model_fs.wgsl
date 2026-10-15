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
@binding( 9 ) @group( 1 ) var nodeUniform28_sampler : sampler_comparison;
@binding( 10 ) @group( 1 ) var nodeUniform28 : texture_depth_2d;

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
	nodeUniform19 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform25 : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : vec3<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : vec2<f32>
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec3<f32>;
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
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar146 : vec3<f32>;
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
var<private> nodeVar157 : f32;
var<private> nodeVar158 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec4<f32>;

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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_tangentView : vec3<f32>,
	@location( 3 ) v_bitangentView : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 7 ) nodeVarying10 : vec2<f32>,
	@builtin( front_facing ) isFront : bool,
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
	nodeVar5 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_tangentView = ( normalize( v_tangentView ) * vec3<f32>( nodeVar5 ) );
	NORMAL_bitangentView = ( normalize( NORMAL_v_bitangentView ) * vec3<f32>( nodeVar5 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeVar5 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar7 = textureSample( nodeUniform15, nodeUniform15_sampler, ( object.nodeUniform16 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	nodeVar8 = ( ( nodeVar7 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar8.xy * object.nodeUniform17 ), nodeVar8.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar9 = dot( normalView, positionViewDirection );
	nodeVar10 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( nodeVar9, 0.0, 1.0 ) ) ).xy;
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
	shadowPositionWorld = v_positionWorld;
	nodeVar29 = ( render.nodeUniform25 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform26 ) ) ), 1.0 ) );
	nodeVar30 = ( nodeVar29.xyz / vec3<f32>( nodeVar29.w ) );
	nodeVar31 = vec3<f32>( nodeVar30.x, ( 1.0 - nodeVar30.y ), ( nodeVar30.z + render.nodeUniform27 ) );

	if ( ( ( ( ( ( nodeVar31.x >= 0.0 ) && ( nodeVar31.x <= 1.0 ) ) && ( nodeVar31.y >= 0.0 ) ) && ( nodeVar31.y <= 1.0 ) ) && ( nodeVar31.z <= 1.0 ) ) ) {

		nodeVar32 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar33 = ( render.nodeUniform29 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform30 ).x );
		nodeVar34 = ( nodeVar31.xy + ( vogelDiskSample( 0, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
		nodeVar35 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar34, nodeVar31.z );
		nodeVar36 = ( nodeVar31.xy + ( vogelDiskSample( 1, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
		nodeVar37 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar36, nodeVar31.z );
		nodeVar38 = ( nodeVar31.xy + ( vogelDiskSample( 2, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
		nodeVar39 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar38, nodeVar31.z );
		nodeVar40 = ( nodeVar31.xy + ( vogelDiskSample( 3, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
		nodeVar41 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar40, nodeVar31.z );
		nodeVar42 = ( nodeVar31.xy + ( vogelDiskSample( 4, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
		nodeVar43 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeVar42, nodeVar31.z );
		nodeVar28 = ( ( ( ( ( nodeVar35 + nodeVar37 ) + nodeVar39 ) + nodeVar41 ) + nodeVar43 ) * 0.2 );

	} else {

		nodeVar28 = 1.0;

	}

	nodeVar44 = mix( 1.0, nodeVar28, render.nodeUniform31 );
	nodeVar45 = ( vec3<f32>( clamp( nodeVar27, 0.0, 1.0 ) ) * ( render.nodeUniform24 * vec3<f32>( nodeVar44 ) ) );
	nodeVar46 = nodeVar45;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar47 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar48 = ( nodeVar46 * nodeVar47 );
	nodeVar49 = ( nodeVar26 + positionViewDirection );
	nodeVar50 = normalize( nodeVar49 );
	nodeVar51 = dot( positionViewDirection, nodeVar50 );
	nodeVar52 = clamp( nodeVar51, 0.0, 1.0 );
	nodeVar53 = exp2( ( ( ( nodeVar52 * -5.55473 ) - 6.98316 ) * nodeVar52 ) );
	nodeVar54 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar53 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar53 ) ) );
	nodeVar55 = ( vec3<f32>( 1.0 ) - nodeVar54 );
	nodeVar56 = nodeVar55;
	nodeVar57 = ( nodeVar48 * nodeVar56 );
	nodeVar58 = ( directDiffuse + nodeVar57 );
	directDiffuse = nodeVar58;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar59 = normalize( ( nodeVar26 + positionViewDirection ) );
	nodeVar60 = clamp( dot( positionViewDirection, nodeVar59 ), 0.0, 1.0 );
	nodeVar61 = exp2( ( ( ( nodeVar60 * -5.55473 ) - 6.98316 ) * nodeVar60 ) );
	nodeVar62 = ( Roughness * Roughness );
	nodeVar63 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar61 ) ) ) + vec3<f32>( ( 1.0 * nodeVar61 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar62, clamp( dot( normalView, nodeVar26 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar62, clamp( dot( normalView, nodeVar59 ), 0.0, 1.0 ) ) ) );
	nodeVar64 = ( nodeVar46 * nodeVar63 );
	nodeVar65 = ( nodeVar64 * multiScatteringCompensation );
	nodeVar66 = ( directSpecular + nodeVar65 );
	directSpecular = nodeVar66;
	nodeVar67 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar68 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar70 = ( SpecularF90 * dfg.y );
	nodeVar71 = ( nodeVar69 + vec3<f32>( nodeVar70 ) );
	nodeVar72 = ( nodeVar67 + nodeVar71 );
	nodeVar67 = nodeVar72;
	nodeVar73 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar74 = nodeVar73;
	nodeVar75 = ( nodeVar74 * vec3<f32>( 0.047619 ) );
	nodeVar76 = ( SpecularColor + nodeVar75 );
	nodeVar77 = ( nodeVar71 * nodeVar76 );
	nodeVar78 = ( dfg.x + dfg.y );
	nodeVar79 = ( 1.0 - nodeVar78 );
	nodeVar80 = nodeVar79;
	nodeVar81 = ( vec3<f32>( nodeVar80 ) * nodeVar76 );
	nodeVar82 = ( vec3<f32>( 1.0 ) - nodeVar81 );
	nodeVar83 = nodeVar82;
	nodeVar84 = ( nodeVar77 / nodeVar83 );
	nodeVar85 = ( nodeVar84 * vec3<f32>( nodeVar80 ) );
	nodeVar86 = ( nodeVar68 + nodeVar85 );
	nodeVar68 = nodeVar86;
	nodeVar87 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar88 = ( irradiance * nodeVar87 );
	nodeVar89 = ( nodeVar67 + nodeVar68 );
	nodeVar90 = ( vec3<f32>( 1.0 ) - nodeVar89 );
	nodeVar91 = nodeVar90;
	nodeVar92 = ( nodeVar88 * nodeVar91 );
	nodeVar93 = nodeVar92;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( indirectDiffuse + nodeVar93 );
	indirectDiffuse = nodeVar94;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar95 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar96 = ( SpecularF90 * dfg.y );
	nodeVar97 = ( nodeVar95 + vec3<f32>( nodeVar96 ) );
	nodeVar98 = ( singleScatteringDielectric + nodeVar97 );
	singleScatteringDielectric = nodeVar98;
	nodeVar99 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar100 = nodeVar99;
	nodeVar101 = ( nodeVar100 * vec3<f32>( 0.047619 ) );
	nodeVar102 = ( SpecularColor + nodeVar101 );
	nodeVar103 = ( nodeVar97 * nodeVar102 );
	nodeVar104 = ( dfg.x + dfg.y );
	nodeVar105 = ( 1.0 - nodeVar104 );
	nodeVar106 = nodeVar105;
	nodeVar107 = ( vec3<f32>( nodeVar106 ) * nodeVar102 );
	nodeVar108 = ( vec3<f32>( 1.0 ) - nodeVar107 );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( nodeVar103 / nodeVar109 );
	nodeVar111 = ( nodeVar110 * vec3<f32>( nodeVar106 ) );
	nodeVar112 = ( multiScatteringDielectric + nodeVar111 );
	multiScatteringDielectric = nodeVar112;
	nodeVar113 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar114 = ( SpecularF90 * dfg.y );
	nodeVar115 = ( nodeVar113 + vec3<f32>( nodeVar114 ) );
	nodeVar116 = ( singleScatteringMetallic + nodeVar115 );
	singleScatteringMetallic = nodeVar116;
	nodeVar117 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar118 * vec3<f32>( 0.047619 ) );
	nodeVar120 = ( DiffuseColor.xyz + nodeVar119 );
	nodeVar121 = ( nodeVar115 * nodeVar120 );
	nodeVar122 = ( dfg.x + dfg.y );
	nodeVar123 = ( 1.0 - nodeVar122 );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( vec3<f32>( nodeVar124 ) * nodeVar120 );
	nodeVar126 = ( vec3<f32>( 1.0 ) - nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( nodeVar121 / nodeVar127 );
	nodeVar129 = ( nodeVar128 * vec3<f32>( nodeVar124 ) );
	nodeVar130 = ( multiScatteringMetallic + nodeVar129 );
	multiScatteringMetallic = nodeVar130;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar131 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar132 = ( radiance * nodeVar131 );
	nodeVar133 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar134 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar135 = ( nodeVar133 * nodeVar134 );
	nodeVar136 = ( nodeVar132 + nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar139 = ( vec3<f32>( 1.0 ) - nodeVar138 );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( DiffuseContribution * nodeVar140 );
	nodeVar142 = ( nodeVar141 * nodeVar134 );
	nodeVar143 = nodeVar142;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar144 = ( indirectSpecular + nodeVar137 );
	indirectSpecular = nodeVar144;
	nodeVar145 = ( indirectDiffuse + nodeVar143 );
	indirectDiffuse = nodeVar145;
	ambientOcclusion = 1.0;
	nodeVar146 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar146;
	nodeVar147 = dot( normalView, positionViewDirection );
	nodeVar148 = ( clamp( nodeVar147, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar149 = ( Roughness * -16.0 );
	nodeVar150 = ( 1.0 - nodeVar149 );
	nodeVar151 = nodeVar150;
	nodeVar152 = ( - nodeVar151 );
	nodeVar153 = exp2( nodeVar152 );
	nodeVar154 = pow( nodeVar148, nodeVar153 );
	nodeVar155 = ( 1.0 - nodeVar154 );
	nodeVar156 = nodeVar155;
	nodeVar157 = ( ambientOcclusion - nodeVar156 );
	nodeVar158 = ( indirectSpecular * vec3<f32>( clamp( nodeVar157, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar158;
	nodeVar159 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar159;
	nodeVar160 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar160;
	nodeVar161 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar161;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar162 = vec4<f32>( mix( Output.xyz, render.nodeUniform32, smoothstep( render.nodeUniform33, render.nodeUniform34, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar162;

	// result

	output.color = nodeVar162;

	return output;

}
