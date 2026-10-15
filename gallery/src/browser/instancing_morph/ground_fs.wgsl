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
@binding( 3 ) @group( 1 ) var nodeUniform17_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform17 : texture_depth_2d;

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
	nodeUniform12 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform14 : vec4<f32>,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform20 : vec4<f32>,
	nodeUniform13 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : vec2<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : vec2<f32>,
	nodeUniform25 : f32
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
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : f32;
var<private> shadowValue : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
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
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

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
	nodeVar9 = vec4<f32>( render.nodeUniform11, 0.0 );
	nodeVar10 = ( render.cameraViewMatrix * nodeVar9 );
	nodeVar11 = normalize( nodeVar10.xyz );
	nodeVar12 = nodeVar11;
	nodeVar13 = dot( normalView, nodeVar12 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar14 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform13 ) ) ), 1.0 );
	nodeVar15 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar15 >= render.nodeUniform14.x ) && ( nodeVar15 < render.nodeUniform14.y ) ) ) {

		nodeVar17 = ( render.nodeUniform15 * nodeVar14 );
		nodeVar18 = ( nodeVar17.xyz / vec3<f32>( nodeVar17.w ) );
		nodeVar19 = vec3<f32>( nodeVar18.x, ( 1.0 - nodeVar18.y ), ( nodeVar18.z + render.nodeUniform16 ) );

		if ( ( ( ( ( ( nodeVar19.x >= 0.0 ) && ( nodeVar19.x <= 1.0 ) ) && ( nodeVar19.y >= 0.0 ) ) && ( nodeVar19.y <= 1.0 ) ) && ( nodeVar19.z <= 1.0 ) ) ) {

			nodeVar20 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar21 = ( render.nodeUniform18 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ).x );
			nodeVar22 = ( nodeVar19.xy + ( vogelDiskSample( 0, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar23 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar22, nodeVar19.z );
			nodeVar24 = ( nodeVar19.xy + ( vogelDiskSample( 1, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar25 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar24, nodeVar19.z );
			nodeVar26 = ( nodeVar19.xy + ( vogelDiskSample( 2, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar27 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar26, nodeVar19.z );
			nodeVar28 = ( nodeVar19.xy + ( vogelDiskSample( 3, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar29 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar28, nodeVar19.z );
			nodeVar30 = ( nodeVar19.xy + ( vogelDiskSample( 4, 5, nodeVar20 ) * vec2<f32>( nodeVar21 ) ) );
			nodeVar31 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar30, nodeVar19.z );
			nodeVar16 = ( ( ( ( ( nodeVar23 + nodeVar25 ) + nodeVar27 ) + nodeVar29 ) + nodeVar31 ) * 0.2 );

		} else {

			nodeVar16 = 1.0;

		}

		shadowValue = mix( nodeVar16, shadowValue, smoothstep( render.nodeUniform14.z, render.nodeUniform14.y, nodeVar15 ) );
		

	}

	if ( ( ( nodeVar15 >= render.nodeUniform20.x ) && ( nodeVar15 < render.nodeUniform20.y ) ) ) {

		nodeVar33 = ( render.nodeUniform21 * nodeVar14 );
		nodeVar34 = ( nodeVar33.xyz / vec3<f32>( nodeVar33.w ) );
		nodeVar35 = vec3<f32>( nodeVar34.x, ( 1.0 - nodeVar34.y ), ( nodeVar34.z + render.nodeUniform22 ) );

		if ( ( ( ( ( ( nodeVar35.x >= 0.0 ) && ( nodeVar35.x <= 1.0 ) ) && ( nodeVar35.y >= 0.0 ) ) && ( nodeVar35.y <= 1.0 ) ) && ( nodeVar35.z <= 1.0 ) ) ) {

			nodeVar36 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar37 = ( render.nodeUniform23 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform24 ).x );
			nodeVar38 = ( nodeVar35.xy + ( vogelDiskSample( 0, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar39 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar38, nodeVar35.z );
			nodeVar40 = ( nodeVar35.xy + ( vogelDiskSample( 1, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar41 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar40, nodeVar35.z );
			nodeVar42 = ( nodeVar35.xy + ( vogelDiskSample( 2, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar43 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar42, nodeVar35.z );
			nodeVar44 = ( nodeVar35.xy + ( vogelDiskSample( 3, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar45 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar44, nodeVar35.z );
			nodeVar46 = ( nodeVar35.xy + ( vogelDiskSample( 4, 5, nodeVar36 ) * vec2<f32>( nodeVar37 ) ) );
			nodeVar47 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar46, nodeVar35.z );
			nodeVar32 = ( ( ( ( ( nodeVar39 + nodeVar41 ) + nodeVar43 ) + nodeVar45 ) + nodeVar47 ) * 0.2 );

		} else {

			nodeVar32 = 1.0;

		}

		shadowValue = mix( nodeVar32, shadowValue, smoothstep( render.nodeUniform20.z, render.nodeUniform20.y, nodeVar15 ) );
		

	}

	nodeVar48 = mix( 1.0, shadowValue, render.nodeUniform25 );
	nodeVar49 = ( render.nodeUniform12 * vec3<f32>( nodeVar48 ) );
	nodeVar50 = ( vec3<f32>( clamp( nodeVar13, 0.0, 1.0 ) ) * nodeVar49 );
	nodeVar51 = nodeVar50;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar52 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar53 = ( nodeVar51 * nodeVar52 );
	nodeVar54 = ( nodeVar12 + positionViewDirection );
	nodeVar55 = normalize( nodeVar54 );
	nodeVar56 = dot( positionViewDirection, nodeVar55 );
	nodeVar57 = clamp( nodeVar56, 0.0, 1.0 );
	nodeVar58 = exp2( ( ( ( nodeVar57 * -5.55473 ) - 6.98316 ) * nodeVar57 ) );
	nodeVar59 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar58 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar58 ) ) );
	nodeVar60 = ( vec3<f32>( 1.0 ) - nodeVar59 );
	nodeVar61 = nodeVar60;
	nodeVar62 = ( nodeVar53 * nodeVar61 );
	nodeVar63 = ( directDiffuse + nodeVar62 );
	directDiffuse = nodeVar63;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar64 = normalize( ( nodeVar12 + positionViewDirection ) );
	nodeVar65 = clamp( dot( positionViewDirection, nodeVar64 ), 0.0, 1.0 );
	nodeVar66 = exp2( ( ( ( nodeVar65 * -5.55473 ) - 6.98316 ) * nodeVar65 ) );
	nodeVar67 = ( Roughness * Roughness );
	nodeVar68 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar66 ) ) ) + vec3<f32>( ( 1.0 * nodeVar66 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar67, clamp( dot( normalView, nodeVar12 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar67, clamp( dot( normalView, nodeVar64 ), 0.0, 1.0 ) ) ) );
	nodeVar69 = ( nodeVar51 * nodeVar68 );
	nodeVar70 = ( nodeVar69 * multiScatteringCompensation );
	nodeVar71 = ( directSpecular + nodeVar70 );
	directSpecular = nodeVar71;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar72 = dot( normalWorld, normalize( render.nodeUniform28 ) );
	nodeVar73 = ( nodeVar72 * 0.5 );
	nodeVar74 = ( nodeVar73 + 0.5 );
	nodeVar75 = mix( render.nodeUniform26, render.nodeUniform27, nodeVar74 );
	nodeVar76 = ( irradiance + nodeVar75 );
	irradiance = nodeVar76;
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
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar172 = vec4<f32>( mix( Output.xyz, render.nodeUniform29, smoothstep( render.nodeUniform30, render.nodeUniform31, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar172;

	// result

	output.color = nodeVar172;

	return output;

}
