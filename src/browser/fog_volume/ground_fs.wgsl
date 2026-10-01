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
@binding( 3 ) @group( 1 ) var nodeUniform19_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform19 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform8 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform16 : vec4<f32>,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform22 : vec4<f32>,
	nodeUniform15 : f32,
	nodeUniform18 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : vec2<f32>,
	nodeUniform27 : f32
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
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : f32;
var<private> shadowValue : f32;
var<private> nodeVar21 : f32;
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
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec2<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
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
var<private> nodeVar79 : f32;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec3<f32>;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : f32;
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
var<private> nodeVar167 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec4<f32>;

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

	DiffuseColor = vec4<f32>( vec3<f32>( 0.31854677811435356, 0.31854677811435356, 0.31854677811435356 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform2, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = dot( normalView, positionViewDirection );
	nodeVar2 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( nodeVar1, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	nodeVar3 = ( dfg.x + dfg.y );
	nodeVar4 = ( 1.0 / nodeVar3 );
	nodeVar5 = nodeVar4;
	nodeVar6 = ( nodeVar5 - 1.0 );
	nodeVar7 = ( SpecularColorBlended * vec3<f32>( nodeVar6 ) );
	nodeVar8 = ( nodeVar7 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar8;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar9 = dot( normalWorld, normalize( render.nodeUniform12 ) );
	nodeVar10 = ( nodeVar9 * 0.5 );
	nodeVar11 = ( nodeVar10 + 0.5 );
	nodeVar12 = mix( render.nodeUniform9, render.nodeUniform10, nodeVar11 );
	nodeVar13 = ( irradiance + nodeVar12 );
	irradiance = nodeVar13;
	nodeVar14 = vec4<f32>( render.nodeUniform13, 0.0 );
	nodeVar15 = ( render.cameraViewMatrix * nodeVar14 );
	nodeVar16 = normalize( nodeVar15.xyz );
	nodeVar17 = nodeVar16;
	nodeVar18 = dot( normalView, nodeVar17 );
	shadowPositionWorld = v_positionWorld;
	nodeVar19 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 );
	nodeVar20 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar20 >= render.nodeUniform16.x ) && ( nodeVar20 < render.nodeUniform16.y ) ) ) {

		nodeVar22 = ( render.nodeUniform17 * nodeVar19 );
		nodeVar23 = ( nodeVar22.xyz / vec3<f32>( nodeVar22.w ) );
		nodeVar24 = vec3<f32>( nodeVar23.x, ( 1.0 - nodeVar23.y ), ( nodeVar23.z + render.nodeUniform18 ) );

		if ( ( ( ( ( ( nodeVar24.x >= 0.0 ) && ( nodeVar24.x <= 1.0 ) ) && ( nodeVar24.y >= 0.0 ) ) && ( nodeVar24.y <= 1.0 ) ) && ( nodeVar24.z <= 1.0 ) ) ) {

			nodeVar25 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar26 = ( render.nodeUniform20 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ).x );
			nodeVar27 = ( nodeVar24.xy + ( vogelDiskSample( 0, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar28 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar27, nodeVar24.z );
			nodeVar29 = ( nodeVar24.xy + ( vogelDiskSample( 1, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar30 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar29, nodeVar24.z );
			nodeVar31 = ( nodeVar24.xy + ( vogelDiskSample( 2, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar32 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar31, nodeVar24.z );
			nodeVar33 = ( nodeVar24.xy + ( vogelDiskSample( 3, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar34 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar33, nodeVar24.z );
			nodeVar35 = ( nodeVar24.xy + ( vogelDiskSample( 4, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
			nodeVar36 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar35, nodeVar24.z );
			nodeVar21 = ( ( ( ( ( nodeVar28 + nodeVar30 ) + nodeVar32 ) + nodeVar34 ) + nodeVar36 ) * 0.2 );

		} else {

			nodeVar21 = 1.0;

		}

		shadowValue = mix( nodeVar21, shadowValue, smoothstep( render.nodeUniform16.z, render.nodeUniform16.y, nodeVar20 ) );
		

	}


	if ( ( ( nodeVar20 >= render.nodeUniform22.x ) && ( nodeVar20 < render.nodeUniform22.y ) ) ) {

		nodeVar38 = ( render.nodeUniform23 * nodeVar19 );
		nodeVar39 = ( nodeVar38.xyz / vec3<f32>( nodeVar38.w ) );
		nodeVar40 = vec3<f32>( nodeVar39.x, ( 1.0 - nodeVar39.y ), ( nodeVar39.z + render.nodeUniform24 ) );

		if ( ( ( ( ( ( nodeVar40.x >= 0.0 ) && ( nodeVar40.x <= 1.0 ) ) && ( nodeVar40.y >= 0.0 ) ) && ( nodeVar40.y <= 1.0 ) ) && ( nodeVar40.z <= 1.0 ) ) ) {

			nodeVar41 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar42 = ( render.nodeUniform25 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform26 ).x );
			nodeVar43 = ( nodeVar40.xy + ( vogelDiskSample( 0, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar44 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar43, nodeVar40.z );
			nodeVar45 = ( nodeVar40.xy + ( vogelDiskSample( 1, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar46 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar45, nodeVar40.z );
			nodeVar47 = ( nodeVar40.xy + ( vogelDiskSample( 2, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar48 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar47, nodeVar40.z );
			nodeVar49 = ( nodeVar40.xy + ( vogelDiskSample( 3, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar50 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar49, nodeVar40.z );
			nodeVar51 = ( nodeVar40.xy + ( vogelDiskSample( 4, 5, nodeVar41 ) * vec2<f32>( nodeVar42 ) ) );
			nodeVar52 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar51, nodeVar40.z );
			nodeVar37 = ( ( ( ( ( nodeVar44 + nodeVar46 ) + nodeVar48 ) + nodeVar50 ) + nodeVar52 ) * 0.2 );

		} else {

			nodeVar37 = 1.0;

		}

		shadowValue = mix( nodeVar37, shadowValue, smoothstep( render.nodeUniform22.z, render.nodeUniform22.y, nodeVar20 ) );
		

	}

	nodeVar53 = mix( 1.0, shadowValue, render.nodeUniform27 );
	nodeVar54 = ( vec3<f32>( clamp( nodeVar18, 0.0, 1.0 ) ) * ( render.nodeUniform14 * vec3<f32>( nodeVar53 ) ) );
	nodeVar55 = nodeVar54;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar56 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar57 = ( nodeVar55 * nodeVar56 );
	nodeVar58 = ( nodeVar17 + positionViewDirection );
	nodeVar59 = normalize( nodeVar58 );
	nodeVar60 = dot( positionViewDirection, nodeVar59 );
	nodeVar61 = clamp( nodeVar60, 0.0, 1.0 );
	nodeVar62 = exp2( ( ( ( nodeVar61 * -5.55473 ) - 6.98316 ) * nodeVar61 ) );
	nodeVar63 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar62 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar62 ) ) );
	nodeVar64 = ( vec3<f32>( 1.0 ) - nodeVar63 );
	nodeVar65 = nodeVar64;
	nodeVar66 = ( nodeVar57 * nodeVar65 );
	nodeVar67 = ( directDiffuse + nodeVar66 );
	directDiffuse = nodeVar67;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar68 = normalize( ( nodeVar17 + positionViewDirection ) );
	nodeVar69 = clamp( dot( positionViewDirection, nodeVar68 ), 0.0, 1.0 );
	nodeVar70 = exp2( ( ( ( nodeVar69 * -5.55473 ) - 6.98316 ) * nodeVar69 ) );
	nodeVar71 = ( Roughness * Roughness );
	nodeVar72 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar70 ) ) ) + vec3<f32>( ( 1.0 * nodeVar70 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar71, clamp( dot( normalView, nodeVar17 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar71, clamp( dot( normalView, nodeVar68 ), 0.0, 1.0 ) ) ) );
	nodeVar73 = ( nodeVar55 * nodeVar72 );
	nodeVar74 = ( nodeVar73 * multiScatteringCompensation );
	nodeVar75 = ( directSpecular + nodeVar74 );
	directSpecular = nodeVar75;
	nodeVar76 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar77 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar78 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar79 = ( SpecularF90 * dfg.y );
	nodeVar80 = ( nodeVar78 + vec3<f32>( nodeVar79 ) );
	nodeVar81 = ( nodeVar76 + nodeVar80 );
	nodeVar76 = nodeVar81;
	nodeVar82 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar83 = nodeVar82;
	nodeVar84 = ( nodeVar83 * vec3<f32>( 0.047619 ) );
	nodeVar85 = ( SpecularColor + nodeVar84 );
	nodeVar86 = ( nodeVar80 * nodeVar85 );
	nodeVar87 = ( dfg.x + dfg.y );
	nodeVar88 = ( 1.0 - nodeVar87 );
	nodeVar89 = nodeVar88;
	nodeVar90 = ( vec3<f32>( nodeVar89 ) * nodeVar85 );
	nodeVar91 = ( vec3<f32>( 1.0 ) - nodeVar90 );
	nodeVar92 = nodeVar91;
	nodeVar93 = ( nodeVar86 / nodeVar92 );
	nodeVar94 = ( nodeVar93 * vec3<f32>( nodeVar89 ) );
	nodeVar95 = ( nodeVar77 + nodeVar94 );
	nodeVar77 = nodeVar95;
	nodeVar96 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar97 = ( irradiance * nodeVar96 );
	nodeVar98 = ( nodeVar76 + nodeVar77 );
	nodeVar99 = ( vec3<f32>( 1.0 ) - nodeVar98 );
	nodeVar100 = nodeVar99;
	nodeVar101 = ( nodeVar97 * nodeVar100 );
	nodeVar102 = nodeVar101;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar103 = ( indirectDiffuse + nodeVar102 );
	indirectDiffuse = nodeVar103;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar104 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar105 = ( SpecularF90 * dfg.y );
	nodeVar106 = ( nodeVar104 + vec3<f32>( nodeVar105 ) );
	nodeVar107 = ( singleScatteringDielectric + nodeVar106 );
	singleScatteringDielectric = nodeVar107;
	nodeVar108 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( nodeVar109 * vec3<f32>( 0.047619 ) );
	nodeVar111 = ( SpecularColor + nodeVar110 );
	nodeVar112 = ( nodeVar106 * nodeVar111 );
	nodeVar113 = ( dfg.x + dfg.y );
	nodeVar114 = ( 1.0 - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( vec3<f32>( nodeVar115 ) * nodeVar111 );
	nodeVar117 = ( vec3<f32>( 1.0 ) - nodeVar116 );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar112 / nodeVar118 );
	nodeVar120 = ( nodeVar119 * vec3<f32>( nodeVar115 ) );
	nodeVar121 = ( multiScatteringDielectric + nodeVar120 );
	multiScatteringDielectric = nodeVar121;
	nodeVar122 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar123 = ( SpecularF90 * dfg.y );
	nodeVar124 = ( nodeVar122 + vec3<f32>( nodeVar123 ) );
	nodeVar125 = ( singleScatteringMetallic + nodeVar124 );
	singleScatteringMetallic = nodeVar125;
	nodeVar126 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( nodeVar127 * vec3<f32>( 0.047619 ) );
	nodeVar129 = ( DiffuseColor.xyz + nodeVar128 );
	nodeVar130 = ( nodeVar124 * nodeVar129 );
	nodeVar131 = ( dfg.x + dfg.y );
	nodeVar132 = ( 1.0 - nodeVar131 );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( vec3<f32>( nodeVar133 ) * nodeVar129 );
	nodeVar135 = ( vec3<f32>( 1.0 ) - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( nodeVar130 / nodeVar136 );
	nodeVar138 = ( nodeVar137 * vec3<f32>( nodeVar133 ) );
	nodeVar139 = ( multiScatteringMetallic + nodeVar138 );
	multiScatteringMetallic = nodeVar139;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar141 = ( radiance * nodeVar140 );
	nodeVar142 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar143 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar144 = ( nodeVar142 * nodeVar143 );
	nodeVar145 = ( nodeVar141 + nodeVar144 );
	nodeVar146 = nodeVar145;
	nodeVar147 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar148 = ( vec3<f32>( 1.0 ) - nodeVar147 );
	nodeVar149 = nodeVar148;
	nodeVar150 = ( DiffuseContribution * nodeVar149 );
	nodeVar151 = ( nodeVar150 * nodeVar143 );
	nodeVar152 = nodeVar151;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar153 = ( indirectSpecular + nodeVar146 );
	indirectSpecular = nodeVar153;
	nodeVar154 = ( indirectDiffuse + nodeVar152 );
	indirectDiffuse = nodeVar154;
	ambientOcclusion = 1.0;
	nodeVar155 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar155;
	nodeVar156 = dot( normalView, positionViewDirection );
	nodeVar157 = ( clamp( nodeVar156, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar158 = ( Roughness * -16.0 );
	nodeVar159 = ( 1.0 - nodeVar158 );
	nodeVar160 = nodeVar159;
	nodeVar161 = ( - nodeVar160 );
	nodeVar162 = exp2( nodeVar161 );
	nodeVar163 = pow( nodeVar157, nodeVar162 );
	nodeVar164 = ( 1.0 - nodeVar163 );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( ambientOcclusion - nodeVar165 );
	nodeVar167 = ( indirectSpecular * vec3<f32>( clamp( nodeVar166, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar167;
	nodeVar168 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar168;
	nodeVar169 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar169;
	nodeVar170 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar170;
	nodeVar171 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar171;

	// result

	output.color = nodeVar171;

	return output;

}
