// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform23_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform23 : texture_depth_cube;
@binding( 7 ) @group( 1 ) var nodeUniform30_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform30 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform14 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform19 : f32,
	nodeUniform26 : f32,
	nodeUniform3 : vec2<f32>,
	nodeUniform21 : f32,
	nodeUniform20 : f32,
	nodeUniform22 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
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
var<private> directSpecular : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec4<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec4<f32>;

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
	nodeVar0 = textureSample( nodeUniform2, nodeUniform2_sampler, ( fragCoord.xy / render.nodeUniform3 ) ).x;
	nodeVar1 = max( nodeVar0, 0.001 );
	AmbientOcclusion = nodeVar1;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar2 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform5, 0.0525 ) + max( max( nodeVar2.x, nodeVar2.y ), nodeVar2.z ) ), 1.0 );
	IOR = object.nodeUniform8;
	nodeVar3 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar3 * nodeVar3 ) ) * object.nodeUniform9 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform10 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform10, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = dot( normalView, positionViewDirection );
	nodeVar5 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( nodeVar4, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar5;
	nodeVar6 = ( dfg.x + dfg.y );
	nodeVar7 = ( 1.0 / nodeVar6 );
	nodeVar8 = nodeVar7;
	nodeVar9 = ( nodeVar8 - 1.0 );
	nodeVar10 = ( SpecularColorBlended * vec3<f32>( nodeVar9 ) );
	nodeVar11 = ( nodeVar10 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar11;
	nodeVar12 = ( render.nodeUniform15 - v_positionView );
	nodeVar13 = normalize( nodeVar12 );
	nodeVar14 = dot( normalView, nodeVar13 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst0 = ( render.nodeUniform17 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform19 ) ) ), 1.0 ) ).xyz;
	let nodeConst1 = abs( nodeConst0 );
	nodeVar15 = 1.0;
	nodeVar16 = max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z );

	if ( ( ( ( nodeVar16 - render.nodeUniform20 ) <= 0.0 ) && ( ( nodeVar16 - render.nodeUniform21 ) >= 0.0 ) ) ) {

		nodeVar17 = ( - nodeVar16 );
		nodeVar18 = ( ( ( render.nodeUniform21 + nodeVar17 ) * render.nodeUniform20 ) / ( ( render.nodeUniform20 - render.nodeUniform21 ) * nodeVar17 ) );
		nodeVar18 = ( nodeVar18 + render.nodeUniform22 );
		nodeVar19 = normalize( nodeConst0 );
		nodeVar21 = abs( nodeVar19 );

		if ( ( nodeVar21.x > nodeVar21.z ) ) {

			nodeVar20 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar20 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar22 = normalize( cross( nodeVar19, nodeVar20 ) );
		nodeVar23 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar24 = vogelDiskSample( 0, 5, nodeVar23 );
		nodeVar25 = cross( nodeVar19, nodeVar22 );
		nodeVar26 = ( render.nodeUniform24 / render.nodeUniform25.x );
		nodeVar27 = ( nodeVar19 + ( ( ( nodeVar22 * vec3<f32>( nodeVar24.x ) ) + ( nodeVar25 * vec3<f32>( nodeVar24.y ) ) ) * vec3<f32>( nodeVar26 ) ) );
		nodeVar28 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, vec3<f32>( nodeVar27.x, ( - nodeVar27.y ), nodeVar27.z ), nodeVar18 );
		nodeVar29 = vogelDiskSample( 1, 5, nodeVar23 );
		nodeVar30 = ( nodeVar19 + ( ( ( nodeVar22 * vec3<f32>( nodeVar29.x ) ) + ( nodeVar25 * vec3<f32>( nodeVar29.y ) ) ) * vec3<f32>( nodeVar26 ) ) );
		nodeVar31 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, vec3<f32>( nodeVar30.x, ( - nodeVar30.y ), nodeVar30.z ), nodeVar18 );
		nodeVar32 = vogelDiskSample( 2, 5, nodeVar23 );
		nodeVar33 = ( nodeVar19 + ( ( ( nodeVar22 * vec3<f32>( nodeVar32.x ) ) + ( nodeVar25 * vec3<f32>( nodeVar32.y ) ) ) * vec3<f32>( nodeVar26 ) ) );
		nodeVar34 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, vec3<f32>( nodeVar33.x, ( - nodeVar33.y ), nodeVar33.z ), nodeVar18 );
		nodeVar35 = vogelDiskSample( 3, 5, nodeVar23 );
		nodeVar36 = ( nodeVar19 + ( ( ( nodeVar22 * vec3<f32>( nodeVar35.x ) ) + ( nodeVar25 * vec3<f32>( nodeVar35.y ) ) ) * vec3<f32>( nodeVar26 ) ) );
		nodeVar37 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, vec3<f32>( nodeVar36.x, ( - nodeVar36.y ), nodeVar36.z ), nodeVar18 );
		nodeVar38 = vogelDiskSample( 4, 5, nodeVar23 );
		nodeVar39 = ( nodeVar19 + ( ( ( nodeVar22 * vec3<f32>( nodeVar38.x ) ) + ( nodeVar25 * vec3<f32>( nodeVar38.y ) ) ) * vec3<f32>( nodeVar26 ) ) );
		nodeVar40 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, vec3<f32>( nodeVar39.x, ( - nodeVar39.y ), nodeVar39.z ), nodeVar18 );
		nodeVar15 = ( ( ( ( ( nodeVar28 + nodeVar31 ) + nodeVar34 ) + nodeVar37 ) + nodeVar40 ) * 0.2 );
		

	}

	nodeVar41 = mix( 1.0, nodeVar15, render.nodeUniform26 );

	if ( ( render.nodeUniform27 > 0.0 ) ) {

		nodeVar43 = length( nodeVar12 );
		nodeVar44 = ( nodeVar43 / render.nodeUniform27 );
		nodeVar45 = clamp( ( 1.0 - ( ( ( nodeVar44 * nodeVar44 ) * nodeVar44 ) * nodeVar44 ) ), 0.0, 1.0 );
		nodeVar42 = ( ( 1.0 / max( pow( nodeVar43, render.nodeUniform28 ), 0.01 ) ) * ( nodeVar45 * nodeVar45 ) );

	} else {

		nodeVar42 = ( 1.0 / max( pow( length( nodeVar12 ), render.nodeUniform28 ), 0.01 ) );

	}

	nodeVar46 = ( ( render.nodeUniform16 * vec3<f32>( nodeVar41 ) ) * vec3<f32>( nodeVar42 ) );
	nodeVar47 = ( vec3<f32>( clamp( nodeVar14, 0.0, 1.0 ) ) * nodeVar46 );
	nodeVar48 = nodeVar47;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar49 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar50 = ( nodeVar48 * nodeVar49 );
	nodeVar51 = ( nodeVar13 + positionViewDirection );
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
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar61 = normalize( ( nodeVar13 + positionViewDirection ) );
	nodeVar62 = clamp( dot( positionViewDirection, nodeVar61 ), 0.0, 1.0 );
	nodeVar63 = exp2( ( ( ( nodeVar62 * -5.55473 ) - 6.98316 ) * nodeVar62 ) );
	nodeVar64 = ( Roughness * Roughness );
	nodeVar65 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar63 ) ) ) + vec3<f32>( ( 1.0 * nodeVar63 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar64, clamp( dot( normalView, nodeVar13 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar64, clamp( dot( normalView, nodeVar61 ), 0.0, 1.0 ) ) ) );
	nodeVar66 = ( nodeVar48 * nodeVar65 );
	nodeVar67 = ( nodeVar66 * multiScatteringCompensation );
	nodeVar68 = ( directSpecular + nodeVar67 );
	directSpecular = nodeVar68;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = ( irradiance + render.nodeUniform29 );
	irradiance = nodeVar69;
	ambientOcclusion = 1.0;
	nodeVar70 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar70;
	nodeVar71 = textureSample( nodeUniform30, nodeUniform30_sampler, ( fragCoord.xy / render.nodeUniform3 ) );
	nodeVar72 = ( irradiance + ( nodeVar71.xyz / vec3<f32>( nodeVar1 ) ) );
	irradiance = nodeVar72;
	nodeVar73 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar74 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar75 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar76 = ( SpecularF90 * dfg.y );
	nodeVar77 = ( nodeVar75 + vec3<f32>( nodeVar76 ) );
	nodeVar78 = ( nodeVar73 + nodeVar77 );
	nodeVar73 = nodeVar78;
	nodeVar79 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar80 = nodeVar79;
	nodeVar81 = ( nodeVar80 * vec3<f32>( 0.047619 ) );
	nodeVar82 = ( SpecularColor + nodeVar81 );
	nodeVar83 = ( nodeVar77 * nodeVar82 );
	nodeVar84 = ( dfg.x + dfg.y );
	nodeVar85 = ( 1.0 - nodeVar84 );
	nodeVar86 = nodeVar85;
	nodeVar87 = ( vec3<f32>( nodeVar86 ) * nodeVar82 );
	nodeVar88 = ( vec3<f32>( 1.0 ) - nodeVar87 );
	nodeVar89 = nodeVar88;
	nodeVar90 = ( nodeVar83 / nodeVar89 );
	nodeVar91 = ( nodeVar90 * vec3<f32>( nodeVar86 ) );
	nodeVar92 = ( nodeVar74 + nodeVar91 );
	nodeVar74 = nodeVar92;
	nodeVar93 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar94 = ( irradiance * nodeVar93 );
	nodeVar95 = ( nodeVar73 + nodeVar74 );
	nodeVar96 = ( vec3<f32>( 1.0 ) - nodeVar95 );
	nodeVar97 = nodeVar96;
	nodeVar98 = ( nodeVar94 * nodeVar97 );
	nodeVar99 = nodeVar98;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar100 = ( indirectDiffuse + nodeVar99 );
	indirectDiffuse = nodeVar100;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar101 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar102 = ( SpecularF90 * dfg.y );
	nodeVar103 = ( nodeVar101 + vec3<f32>( nodeVar102 ) );
	nodeVar104 = ( singleScatteringDielectric + nodeVar103 );
	singleScatteringDielectric = nodeVar104;
	nodeVar105 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar106 = nodeVar105;
	nodeVar107 = ( nodeVar106 * vec3<f32>( 0.047619 ) );
	nodeVar108 = ( SpecularColor + nodeVar107 );
	nodeVar109 = ( nodeVar103 * nodeVar108 );
	nodeVar110 = ( dfg.x + dfg.y );
	nodeVar111 = ( 1.0 - nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( vec3<f32>( nodeVar112 ) * nodeVar108 );
	nodeVar114 = ( vec3<f32>( 1.0 ) - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( nodeVar109 / nodeVar115 );
	nodeVar117 = ( nodeVar116 * vec3<f32>( nodeVar112 ) );
	nodeVar118 = ( multiScatteringDielectric + nodeVar117 );
	multiScatteringDielectric = nodeVar118;
	nodeVar119 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar120 = ( SpecularF90 * dfg.y );
	nodeVar121 = ( nodeVar119 + vec3<f32>( nodeVar120 ) );
	nodeVar122 = ( singleScatteringMetallic + nodeVar121 );
	singleScatteringMetallic = nodeVar122;
	nodeVar123 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( nodeVar124 * vec3<f32>( 0.047619 ) );
	nodeVar126 = ( DiffuseColor.xyz + nodeVar125 );
	nodeVar127 = ( nodeVar121 * nodeVar126 );
	nodeVar128 = ( dfg.x + dfg.y );
	nodeVar129 = ( 1.0 - nodeVar128 );
	nodeVar130 = nodeVar129;
	nodeVar131 = ( vec3<f32>( nodeVar130 ) * nodeVar126 );
	nodeVar132 = ( vec3<f32>( 1.0 ) - nodeVar131 );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( nodeVar127 / nodeVar133 );
	nodeVar135 = ( nodeVar134 * vec3<f32>( nodeVar130 ) );
	nodeVar136 = ( multiScatteringMetallic + nodeVar135 );
	multiScatteringMetallic = nodeVar136;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar137 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar138 = ( radiance * nodeVar137 );
	nodeVar139 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar141 = ( nodeVar139 * nodeVar140 );
	nodeVar142 = ( nodeVar138 + nodeVar141 );
	nodeVar143 = nodeVar142;
	nodeVar144 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar145 = ( vec3<f32>( 1.0 ) - nodeVar144 );
	nodeVar146 = nodeVar145;
	nodeVar147 = ( DiffuseContribution * nodeVar146 );
	nodeVar148 = ( nodeVar147 * nodeVar140 );
	nodeVar149 = nodeVar148;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar150 = ( indirectSpecular + nodeVar143 );
	indirectSpecular = nodeVar150;
	nodeVar151 = ( indirectDiffuse + nodeVar149 );
	indirectDiffuse = nodeVar151;
	nodeVar152 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar152;
	nodeVar153 = dot( normalView, positionViewDirection );
	nodeVar154 = ( clamp( nodeVar153, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar155 = ( Roughness * -16.0 );
	nodeVar156 = ( 1.0 - nodeVar155 );
	nodeVar157 = nodeVar156;
	nodeVar158 = ( - nodeVar157 );
	nodeVar159 = exp2( nodeVar158 );
	nodeVar160 = pow( nodeVar154, nodeVar159 );
	nodeVar161 = ( 1.0 - nodeVar160 );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( ambientOcclusion - nodeVar162 );
	nodeVar164 = ( indirectSpecular * vec3<f32>( clamp( nodeVar163, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar164;
	nodeVar165 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar165;
	nodeVar166 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar166;
	nodeVar167 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar167;
	nodeVar168 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar168;

	// result

	output.color = nodeVar168;

	return output;

}
