// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	@location( 3 ) m3 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform14 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform24_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform24 : texture_depth_cube;

struct objectStruct {
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform31 : mat4x4<f32>,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform34 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform32 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform27 : f32,
	nodeUniform22 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar1 : f32;
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
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec3<f32>;
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
var<private> irradiance : vec3<f32>;
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
var<private> nodeVar162 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar163 : vec4<f32>;
var<private> nodeVar164 : vec4<f32>;
var<private> nodeVar165 : vec2<f32>;

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
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) positionPrevious : vec3<f32>,
	@location( 2 ) v_positionView : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) vInstanceColor : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vInstanceColor * object.nodeUniform3 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform5;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform6, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	IOR = object.nodeUniform9;
	nodeVar1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar1 * nodeVar1 ) ) * object.nodeUniform10 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform11 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform11, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform5 ) ) );
	EmissiveColor = ( object.nodeUniform12 * vec3<f32>( object.nodeUniform13 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = dot( normalView, positionViewDirection );
	nodeVar3 = textureSample( nodeUniform14, nodeUniform14_sampler, vec2<f32>( Roughness, clamp( nodeVar2, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	nodeVar4 = ( dfg.x + dfg.y );
	nodeVar5 = ( 1.0 / nodeVar4 );
	nodeVar6 = nodeVar5;
	nodeVar7 = ( nodeVar6 - 1.0 );
	nodeVar8 = ( SpecularColorBlended * vec3<f32>( nodeVar7 ) );
	nodeVar9 = ( nodeVar8 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar9;
	nodeVar10 = ( render.nodeUniform16 - v_positionView );
	nodeVar11 = normalize( nodeVar10 );
	nodeVar12 = dot( normalView, nodeVar11 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst0 = ( render.nodeUniform18 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform20 ) ) ), 1.0 ) ).xyz;
	let nodeConst1 = abs( nodeConst0 );
	nodeVar13 = 1.0;
	nodeVar14 = max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z );

	if ( ( ( ( nodeVar14 - render.nodeUniform21 ) <= 0.0 ) && ( ( nodeVar14 - render.nodeUniform22 ) >= 0.0 ) ) ) {

		nodeVar15 = ( - nodeVar14 );
		nodeVar16 = ( ( ( render.nodeUniform22 + nodeVar15 ) * render.nodeUniform21 ) / ( ( render.nodeUniform21 - render.nodeUniform22 ) * nodeVar15 ) );
		nodeVar16 = ( nodeVar16 + render.nodeUniform23 );
		nodeVar17 = normalize( nodeConst0 );
		nodeVar19 = abs( nodeVar17 );

		if ( ( nodeVar19.x > nodeVar19.z ) ) {

			nodeVar18 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar18 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar20 = normalize( cross( nodeVar17, nodeVar18 ) );
		nodeVar21 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar22 = vogelDiskSample( 0, 5, nodeVar21 );
		nodeVar23 = cross( nodeVar17, nodeVar20 );
		nodeVar24 = ( render.nodeUniform25 / render.nodeUniform26.x );
		nodeVar25 = ( nodeVar17 + ( ( ( nodeVar20 * vec3<f32>( nodeVar22.x ) ) + ( nodeVar23 * vec3<f32>( nodeVar22.y ) ) ) * vec3<f32>( nodeVar24 ) ) );
		nodeVar26 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeVar25.x, ( - nodeVar25.y ), nodeVar25.z ), nodeVar16 );
		nodeVar27 = vogelDiskSample( 1, 5, nodeVar21 );
		nodeVar28 = ( nodeVar17 + ( ( ( nodeVar20 * vec3<f32>( nodeVar27.x ) ) + ( nodeVar23 * vec3<f32>( nodeVar27.y ) ) ) * vec3<f32>( nodeVar24 ) ) );
		nodeVar29 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeVar28.x, ( - nodeVar28.y ), nodeVar28.z ), nodeVar16 );
		nodeVar30 = vogelDiskSample( 2, 5, nodeVar21 );
		nodeVar31 = ( nodeVar17 + ( ( ( nodeVar20 * vec3<f32>( nodeVar30.x ) ) + ( nodeVar23 * vec3<f32>( nodeVar30.y ) ) ) * vec3<f32>( nodeVar24 ) ) );
		nodeVar32 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeVar31.x, ( - nodeVar31.y ), nodeVar31.z ), nodeVar16 );
		nodeVar33 = vogelDiskSample( 3, 5, nodeVar21 );
		nodeVar34 = ( nodeVar17 + ( ( ( nodeVar20 * vec3<f32>( nodeVar33.x ) ) + ( nodeVar23 * vec3<f32>( nodeVar33.y ) ) ) * vec3<f32>( nodeVar24 ) ) );
		nodeVar35 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeVar34.x, ( - nodeVar34.y ), nodeVar34.z ), nodeVar16 );
		nodeVar36 = vogelDiskSample( 4, 5, nodeVar21 );
		nodeVar37 = ( nodeVar17 + ( ( ( nodeVar20 * vec3<f32>( nodeVar36.x ) ) + ( nodeVar23 * vec3<f32>( nodeVar36.y ) ) ) * vec3<f32>( nodeVar24 ) ) );
		nodeVar38 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeVar37.x, ( - nodeVar37.y ), nodeVar37.z ), nodeVar16 );
		nodeVar13 = ( ( ( ( ( nodeVar26 + nodeVar29 ) + nodeVar32 ) + nodeVar35 ) + nodeVar38 ) * 0.2 );
		

	}

	nodeVar39 = mix( 1.0, nodeVar13, render.nodeUniform27 );

	if ( ( render.nodeUniform28 > 0.0 ) ) {

		nodeVar41 = length( nodeVar10 );
		nodeVar42 = ( nodeVar41 / render.nodeUniform28 );
		nodeVar43 = clamp( ( 1.0 - ( ( ( nodeVar42 * nodeVar42 ) * nodeVar42 ) * nodeVar42 ) ), 0.0, 1.0 );
		nodeVar40 = ( ( 1.0 / max( pow( nodeVar41, render.nodeUniform29 ), 0.01 ) ) * ( nodeVar43 * nodeVar43 ) );

	} else {

		nodeVar40 = ( 1.0 / max( pow( length( nodeVar10 ), render.nodeUniform29 ), 0.01 ) );

	}

	nodeVar44 = ( ( render.nodeUniform17 * vec3<f32>( nodeVar39 ) ) * vec3<f32>( nodeVar40 ) );
	nodeVar45 = ( vec3<f32>( clamp( nodeVar12, 0.0, 1.0 ) ) * nodeVar44 );
	nodeVar46 = nodeVar45;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar47 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar48 = ( nodeVar46 * nodeVar47 );
	nodeVar49 = ( nodeVar11 + positionViewDirection );
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
	nodeVar59 = normalize( ( nodeVar11 + positionViewDirection ) );
	nodeVar60 = clamp( dot( positionViewDirection, nodeVar59 ), 0.0, 1.0 );
	nodeVar61 = exp2( ( ( ( nodeVar60 * -5.55473 ) - 6.98316 ) * nodeVar60 ) );
	nodeVar62 = ( Roughness * Roughness );
	nodeVar63 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar61 ) ) ) + vec3<f32>( ( 1.0 * nodeVar61 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar62, clamp( dot( normalView, nodeVar11 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar62, clamp( dot( normalView, nodeVar59 ), 0.0, 1.0 ) ) ) );
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
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
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
	output.m0 = Output;
	output.m1 = DiffuseColor;
	nodeVar162 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m2 = vec4<f32>( nodeVar162, 1.0 );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform31 );
	nodeVar163 = ( ( object.nodeUniform30 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar164 = ( ( render.nodeUniform32 * ( object.nodeUniform33 * object.nodeUniform34 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar165 = ( ( nodeVar163.xy / vec2<f32>( nodeVar163.w ) ) - ( nodeVar164.xy / vec2<f32>( nodeVar164.w ) ) );
	output.m3 = vec4<f32>( vec3<f32>( nodeVar165, 0.0 ), 1.0 );

	// result

	return output;

}
