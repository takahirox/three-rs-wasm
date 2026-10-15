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
@binding( 3 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform17_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform17 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform21_sampler : sampler_comparison;
@binding( 8 ) @group( 1 ) var nodeUniform21 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform11 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : vec2<f32>,
	nodeUniform24 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : vec4<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : vec4<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec4<f32>;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec4<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec3<f32>;
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
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec4<f32>;

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
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform5, 0.0525 ) + max( max( nodeVar1.x, nodeVar1.y ), nodeVar1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = dot( normalView, positionViewDirection );
	nodeVar3 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( Roughness, clamp( nodeVar2, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	nodeVar4 = ( dfg.x + dfg.y );
	nodeVar5 = ( 1.0 / nodeVar4 );
	nodeVar6 = nodeVar5;
	nodeVar7 = ( nodeVar6 - 1.0 );
	nodeVar8 = ( SpecularColorBlended * vec3<f32>( nodeVar7 ) );
	nodeVar9 = ( nodeVar8 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar9;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar10 = ( irradiance + render.nodeUniform12 );
	irradiance = nodeVar10;
	nodeVar11 = ( render.nodeUniform14 - render.nodeUniform15 );
	nodeVar12 = vec4<f32>( nodeVar11, 0.0 );
	nodeVar13 = ( render.cameraViewMatrix * nodeVar12 );
	nodeVar14 = normalize( nodeVar13.xyz );
	nodeVar15 = nodeVar14;
	nodeVar16 = dot( normalView, nodeVar15 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar17 = ( render.nodeUniform18 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform19 ) ) ), 1.0 ) );
	nodeVar18 = ( nodeVar17.xyz / vec3<f32>( nodeVar17.w ) );
	nodeVar19 = vec3<f32>( nodeVar18.x, ( 1.0 - nodeVar18.y ), ( nodeVar18.z + render.nodeUniform20 ) );
	nodeVar20 = textureSample( nodeUniform17, nodeUniform17_sampler, nodeVar19.xy );

	if ( ( ( ( ( ( nodeVar19.x >= 0.0 ) && ( nodeVar19.x <= 1.0 ) ) && ( nodeVar19.y >= 0.0 ) ) && ( nodeVar19.y <= 1.0 ) ) && ( nodeVar19.z <= 1.0 ) ) ) {

		nodeVar22 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar23 = ( render.nodeUniform22 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform23 ).x );
		nodeVar24 = ( nodeVar19.xy + ( vogelDiskSample( 0, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar25 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar24, nodeVar19.z );
		nodeVar26 = ( nodeVar19.xy + ( vogelDiskSample( 1, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar27 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar26, nodeVar19.z );
		nodeVar28 = ( nodeVar19.xy + ( vogelDiskSample( 2, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar29 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar28, nodeVar19.z );
		nodeVar30 = ( nodeVar19.xy + ( vogelDiskSample( 3, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar31 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar30, nodeVar19.z );
		nodeVar32 = ( nodeVar19.xy + ( vogelDiskSample( 4, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
		nodeVar33 = textureSampleCompare( nodeUniform21, nodeUniform21_sampler, nodeVar32, nodeVar19.z );
		nodeVar21 = ( ( ( ( ( nodeVar25 + nodeVar27 ) + nodeVar29 ) + nodeVar31 ) + nodeVar33 ) * 0.2 );

	} else {

		nodeVar21 = 1.0;

	}

	nodeVar34 = mix( vec4<f32>( 1.0 ), mix( nodeVar20, vec4<f32>( 1.0 ), vec4<f32>( nodeVar21 ) ), ( render.nodeUniform24 * nodeVar20.w ) );
	nodeVar35 = ( vec4<f32>( render.nodeUniform16, 1.0 ) * nodeVar34 );
	nodeVar36 = ( vec4<f32>( clamp( nodeVar16, 0.0, 1.0 ) ) * nodeVar35 );
	nodeVar37 = nodeVar36;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar38 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar39 = ( nodeVar37 * vec4<f32>( nodeVar38, 1.0 ) );
	nodeVar40 = ( nodeVar15 + positionViewDirection );
	nodeVar41 = normalize( nodeVar40 );
	nodeVar42 = dot( positionViewDirection, nodeVar41 );
	nodeVar43 = clamp( nodeVar42, 0.0, 1.0 );
	nodeVar44 = exp2( ( ( ( nodeVar43 * -5.55473 ) - 6.98316 ) * nodeVar43 ) );
	nodeVar45 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar44 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar44 ) ) );
	nodeVar46 = ( vec3<f32>( 1.0 ) - nodeVar45 );
	nodeVar47 = nodeVar46;
	nodeVar48 = ( nodeVar39 * vec4<f32>( nodeVar47, 1.0 ) );
	nodeVar49 = ( vec4<f32>( directDiffuse, 1.0 ) + nodeVar48 );
	directDiffuse = nodeVar49.xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar50 = normalize( ( nodeVar15 + positionViewDirection ) );
	nodeVar51 = clamp( dot( positionViewDirection, nodeVar50 ), 0.0, 1.0 );
	nodeVar52 = exp2( ( ( ( nodeVar51 * -5.55473 ) - 6.98316 ) * nodeVar51 ) );
	nodeVar53 = ( Roughness * Roughness );
	nodeVar54 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar52 ) ) ) + vec3<f32>( ( 1.0 * nodeVar52 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar53, clamp( dot( normalView, nodeVar15 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar53, clamp( dot( normalView, nodeVar50 ), 0.0, 1.0 ) ) ) );
	nodeVar55 = ( nodeVar37 * vec4<f32>( nodeVar54, 1.0 ) );
	nodeVar56 = ( nodeVar55 * vec4<f32>( multiScatteringCompensation, 1.0 ) );
	nodeVar57 = ( vec4<f32>( directSpecular, 1.0 ) + nodeVar56 );
	directSpecular = nodeVar57.xyz;
	nodeVar58 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar59 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar60 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar61 = ( SpecularF90 * dfg.y );
	nodeVar62 = ( nodeVar60 + vec3<f32>( nodeVar61 ) );
	nodeVar63 = ( nodeVar58 + nodeVar62 );
	nodeVar58 = nodeVar63;
	nodeVar64 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar65 = nodeVar64;
	nodeVar66 = ( nodeVar65 * vec3<f32>( 0.047619 ) );
	nodeVar67 = ( SpecularColor + nodeVar66 );
	nodeVar68 = ( nodeVar62 * nodeVar67 );
	nodeVar69 = ( dfg.x + dfg.y );
	nodeVar70 = ( 1.0 - nodeVar69 );
	nodeVar71 = nodeVar70;
	nodeVar72 = ( vec3<f32>( nodeVar71 ) * nodeVar67 );
	nodeVar73 = ( vec3<f32>( 1.0 ) - nodeVar72 );
	nodeVar74 = nodeVar73;
	nodeVar75 = ( nodeVar68 / nodeVar74 );
	nodeVar76 = ( nodeVar75 * vec3<f32>( nodeVar71 ) );
	nodeVar77 = ( nodeVar59 + nodeVar76 );
	nodeVar59 = nodeVar77;
	nodeVar78 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar79 = ( irradiance * nodeVar78 );
	nodeVar80 = ( nodeVar58 + nodeVar59 );
	nodeVar81 = ( vec3<f32>( 1.0 ) - nodeVar80 );
	nodeVar82 = nodeVar81;
	nodeVar83 = ( nodeVar79 * nodeVar82 );
	nodeVar84 = nodeVar83;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar85 = ( indirectDiffuse + nodeVar84 );
	indirectDiffuse = nodeVar85;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar86 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar87 = ( SpecularF90 * dfg.y );
	nodeVar88 = ( nodeVar86 + vec3<f32>( nodeVar87 ) );
	nodeVar89 = ( singleScatteringDielectric + nodeVar88 );
	singleScatteringDielectric = nodeVar89;
	nodeVar90 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar91 = nodeVar90;
	nodeVar92 = ( nodeVar91 * vec3<f32>( 0.047619 ) );
	nodeVar93 = ( SpecularColor + nodeVar92 );
	nodeVar94 = ( nodeVar88 * nodeVar93 );
	nodeVar95 = ( dfg.x + dfg.y );
	nodeVar96 = ( 1.0 - nodeVar95 );
	nodeVar97 = nodeVar96;
	nodeVar98 = ( vec3<f32>( nodeVar97 ) * nodeVar93 );
	nodeVar99 = ( vec3<f32>( 1.0 ) - nodeVar98 );
	nodeVar100 = nodeVar99;
	nodeVar101 = ( nodeVar94 / nodeVar100 );
	nodeVar102 = ( nodeVar101 * vec3<f32>( nodeVar97 ) );
	nodeVar103 = ( multiScatteringDielectric + nodeVar102 );
	multiScatteringDielectric = nodeVar103;
	nodeVar104 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar105 = ( SpecularF90 * dfg.y );
	nodeVar106 = ( nodeVar104 + vec3<f32>( nodeVar105 ) );
	nodeVar107 = ( singleScatteringMetallic + nodeVar106 );
	singleScatteringMetallic = nodeVar107;
	nodeVar108 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( nodeVar109 * vec3<f32>( 0.047619 ) );
	nodeVar111 = ( DiffuseColor.xyz + nodeVar110 );
	nodeVar112 = ( nodeVar106 * nodeVar111 );
	nodeVar113 = ( dfg.x + dfg.y );
	nodeVar114 = ( 1.0 - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( vec3<f32>( nodeVar115 ) * nodeVar111 );
	nodeVar117 = ( vec3<f32>( 1.0 ) - nodeVar116 );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar112 / nodeVar118 );
	nodeVar120 = ( nodeVar119 * vec3<f32>( nodeVar115 ) );
	nodeVar121 = ( multiScatteringMetallic + nodeVar120 );
	multiScatteringMetallic = nodeVar121;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar122 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar123 = ( radiance * nodeVar122 );
	nodeVar124 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar125 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar126 = ( nodeVar124 * nodeVar125 );
	nodeVar127 = ( nodeVar123 + nodeVar126 );
	nodeVar128 = nodeVar127;
	nodeVar129 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar130 = ( vec3<f32>( 1.0 ) - nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( DiffuseContribution * nodeVar131 );
	nodeVar133 = ( nodeVar132 * nodeVar125 );
	nodeVar134 = nodeVar133;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar135 = ( indirectSpecular + nodeVar128 );
	indirectSpecular = nodeVar135;
	nodeVar136 = ( indirectDiffuse + nodeVar134 );
	indirectDiffuse = nodeVar136;
	ambientOcclusion = 1.0;
	nodeVar137 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar137;
	nodeVar138 = dot( normalView, positionViewDirection );
	nodeVar139 = ( clamp( nodeVar138, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar140 = ( Roughness * -16.0 );
	nodeVar141 = ( 1.0 - nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( - nodeVar142 );
	nodeVar144 = exp2( nodeVar143 );
	nodeVar145 = pow( nodeVar139, nodeVar144 );
	nodeVar146 = ( 1.0 - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( ambientOcclusion - nodeVar147 );
	nodeVar149 = ( indirectSpecular * vec3<f32>( clamp( nodeVar148, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar149;
	nodeVar150 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar150;
	nodeVar151 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar151;
	nodeVar152 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar152;
	nodeVar153 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar153;

	// result

	output.color = nodeVar153;

	return output;

}
