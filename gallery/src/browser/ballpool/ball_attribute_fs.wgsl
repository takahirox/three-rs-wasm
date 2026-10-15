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
@binding( 1 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform30_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform30 : texture_depth_cube;

struct objectStruct {
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform14 : mat3x3<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : f32,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform36 : mat4x4<f32>,
	nodeUniform37 : mat4x4<f32>,
	nodeUniform39 : mat4x4<f32>,
	nodeUniform40 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform38 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform22 : vec3<f32>,
	nodeUniform24 : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform33 : f32,
	nodeUniform28 : f32,
	nodeUniform27 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar147 : vec3<f32>;
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
var<private> nodeVar158 : f32;
var<private> nodeVar159 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar164 : vec4<f32>;
var<private> nodeVar165 : vec4<f32>;
var<private> nodeVar166 : vec2<f32>;

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

	DiffuseColor = vec4<f32>( ( vInstanceColor * object.nodeUniform9 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform10 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform11;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform12, 0.0525 ) + max( max( nodeVar1.x, nodeVar1.y ), nodeVar1.z ) ), 1.0 );
	IOR = object.nodeUniform15;
	nodeVar2 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar2 * nodeVar2 ) ) * object.nodeUniform16 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform17 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform17, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform11 ) ) );
	EmissiveColor = ( object.nodeUniform18 * vec3<f32>( object.nodeUniform19 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar3 = dot( normalView, positionViewDirection );
	nodeVar4 = textureSample( nodeUniform20, nodeUniform20_sampler, vec2<f32>( Roughness, clamp( nodeVar3, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	nodeVar5 = ( dfg.x + dfg.y );
	nodeVar6 = ( 1.0 / nodeVar5 );
	nodeVar7 = nodeVar6;
	nodeVar8 = ( nodeVar7 - 1.0 );
	nodeVar9 = ( SpecularColorBlended * vec3<f32>( nodeVar8 ) );
	nodeVar10 = ( nodeVar9 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar10;
	nodeVar11 = ( render.nodeUniform22 - v_positionView );
	nodeVar12 = normalize( nodeVar11 );
	nodeVar13 = dot( normalView, nodeVar12 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst0 = ( render.nodeUniform24 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform26 ) ) ), 1.0 ) ).xyz;
	let nodeConst1 = abs( nodeConst0 );
	nodeVar14 = 1.0;
	nodeVar15 = max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z );

	if ( ( ( ( nodeVar15 - render.nodeUniform27 ) <= 0.0 ) && ( ( nodeVar15 - render.nodeUniform28 ) >= 0.0 ) ) ) {

		nodeVar16 = ( - nodeVar15 );
		nodeVar17 = ( ( ( render.nodeUniform28 + nodeVar16 ) * render.nodeUniform27 ) / ( ( render.nodeUniform27 - render.nodeUniform28 ) * nodeVar16 ) );
		nodeVar17 = ( nodeVar17 + render.nodeUniform29 );
		nodeVar18 = normalize( nodeConst0 );
		nodeVar20 = abs( nodeVar18 );

		if ( ( nodeVar20.x > nodeVar20.z ) ) {

			nodeVar19 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar19 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar21 = normalize( cross( nodeVar18, nodeVar19 ) );
		nodeVar22 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar23 = vogelDiskSample( 0, 5, nodeVar22 );
		nodeVar24 = cross( nodeVar18, nodeVar21 );
		nodeVar25 = ( render.nodeUniform31 / render.nodeUniform32.x );
		nodeVar26 = ( nodeVar18 + ( ( ( nodeVar21 * vec3<f32>( nodeVar23.x ) ) + ( nodeVar24 * vec3<f32>( nodeVar23.y ) ) ) * vec3<f32>( nodeVar25 ) ) );
		nodeVar27 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeVar26.x, ( - nodeVar26.y ), nodeVar26.z ), nodeVar17 );
		nodeVar28 = vogelDiskSample( 1, 5, nodeVar22 );
		nodeVar29 = ( nodeVar18 + ( ( ( nodeVar21 * vec3<f32>( nodeVar28.x ) ) + ( nodeVar24 * vec3<f32>( nodeVar28.y ) ) ) * vec3<f32>( nodeVar25 ) ) );
		nodeVar30 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeVar29.x, ( - nodeVar29.y ), nodeVar29.z ), nodeVar17 );
		nodeVar31 = vogelDiskSample( 2, 5, nodeVar22 );
		nodeVar32 = ( nodeVar18 + ( ( ( nodeVar21 * vec3<f32>( nodeVar31.x ) ) + ( nodeVar24 * vec3<f32>( nodeVar31.y ) ) ) * vec3<f32>( nodeVar25 ) ) );
		nodeVar33 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeVar32.x, ( - nodeVar32.y ), nodeVar32.z ), nodeVar17 );
		nodeVar34 = vogelDiskSample( 3, 5, nodeVar22 );
		nodeVar35 = ( nodeVar18 + ( ( ( nodeVar21 * vec3<f32>( nodeVar34.x ) ) + ( nodeVar24 * vec3<f32>( nodeVar34.y ) ) ) * vec3<f32>( nodeVar25 ) ) );
		nodeVar36 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeVar35.x, ( - nodeVar35.y ), nodeVar35.z ), nodeVar17 );
		nodeVar37 = vogelDiskSample( 4, 5, nodeVar22 );
		nodeVar38 = ( nodeVar18 + ( ( ( nodeVar21 * vec3<f32>( nodeVar37.x ) ) + ( nodeVar24 * vec3<f32>( nodeVar37.y ) ) ) * vec3<f32>( nodeVar25 ) ) );
		nodeVar39 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeVar38.x, ( - nodeVar38.y ), nodeVar38.z ), nodeVar17 );
		nodeVar14 = ( ( ( ( ( nodeVar27 + nodeVar30 ) + nodeVar33 ) + nodeVar36 ) + nodeVar39 ) * 0.2 );
		

	}

	nodeVar40 = mix( 1.0, nodeVar14, render.nodeUniform33 );

	if ( ( render.nodeUniform34 > 0.0 ) ) {

		nodeVar42 = length( nodeVar11 );
		nodeVar43 = ( nodeVar42 / render.nodeUniform34 );
		nodeVar44 = clamp( ( 1.0 - ( ( ( nodeVar43 * nodeVar43 ) * nodeVar43 ) * nodeVar43 ) ), 0.0, 1.0 );
		nodeVar41 = ( ( 1.0 / max( pow( nodeVar42, render.nodeUniform35 ), 0.01 ) ) * ( nodeVar44 * nodeVar44 ) );

	} else {

		nodeVar41 = ( 1.0 / max( pow( length( nodeVar11 ), render.nodeUniform35 ), 0.01 ) );

	}

	nodeVar45 = ( ( render.nodeUniform23 * vec3<f32>( nodeVar40 ) ) * vec3<f32>( nodeVar41 ) );
	nodeVar46 = ( vec3<f32>( clamp( nodeVar13, 0.0, 1.0 ) ) * nodeVar45 );
	nodeVar47 = nodeVar46;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar48 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar49 = ( nodeVar47 * nodeVar48 );
	nodeVar50 = ( nodeVar12 + positionViewDirection );
	nodeVar51 = normalize( nodeVar50 );
	nodeVar52 = dot( positionViewDirection, nodeVar51 );
	nodeVar53 = clamp( nodeVar52, 0.0, 1.0 );
	nodeVar54 = exp2( ( ( ( nodeVar53 * -5.55473 ) - 6.98316 ) * nodeVar53 ) );
	nodeVar55 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar54 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar54 ) ) );
	nodeVar56 = ( vec3<f32>( 1.0 ) - nodeVar55 );
	nodeVar57 = nodeVar56;
	nodeVar58 = ( nodeVar49 * nodeVar57 );
	nodeVar59 = ( directDiffuse + nodeVar58 );
	directDiffuse = nodeVar59;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar60 = normalize( ( nodeVar12 + positionViewDirection ) );
	nodeVar61 = clamp( dot( positionViewDirection, nodeVar60 ), 0.0, 1.0 );
	nodeVar62 = exp2( ( ( ( nodeVar61 * -5.55473 ) - 6.98316 ) * nodeVar61 ) );
	nodeVar63 = ( Roughness * Roughness );
	nodeVar64 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar62 ) ) ) + vec3<f32>( ( 1.0 * nodeVar62 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar63, clamp( dot( normalView, nodeVar12 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar63, clamp( dot( normalView, nodeVar60 ), 0.0, 1.0 ) ) ) );
	nodeVar65 = ( nodeVar47 * nodeVar64 );
	nodeVar66 = ( nodeVar65 * multiScatteringCompensation );
	nodeVar67 = ( directSpecular + nodeVar66 );
	directSpecular = nodeVar67;
	nodeVar68 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar70 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar71 = ( SpecularF90 * dfg.y );
	nodeVar72 = ( nodeVar70 + vec3<f32>( nodeVar71 ) );
	nodeVar73 = ( nodeVar68 + nodeVar72 );
	nodeVar68 = nodeVar73;
	nodeVar74 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar75 = nodeVar74;
	nodeVar76 = ( nodeVar75 * vec3<f32>( 0.047619 ) );
	nodeVar77 = ( SpecularColor + nodeVar76 );
	nodeVar78 = ( nodeVar72 * nodeVar77 );
	nodeVar79 = ( dfg.x + dfg.y );
	nodeVar80 = ( 1.0 - nodeVar79 );
	nodeVar81 = nodeVar80;
	nodeVar82 = ( vec3<f32>( nodeVar81 ) * nodeVar77 );
	nodeVar83 = ( vec3<f32>( 1.0 ) - nodeVar82 );
	nodeVar84 = nodeVar83;
	nodeVar85 = ( nodeVar78 / nodeVar84 );
	nodeVar86 = ( nodeVar85 * vec3<f32>( nodeVar81 ) );
	nodeVar87 = ( nodeVar69 + nodeVar86 );
	nodeVar69 = nodeVar87;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar88 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar89 = ( irradiance * nodeVar88 );
	nodeVar90 = ( nodeVar68 + nodeVar69 );
	nodeVar91 = ( vec3<f32>( 1.0 ) - nodeVar90 );
	nodeVar92 = nodeVar91;
	nodeVar93 = ( nodeVar89 * nodeVar92 );
	nodeVar94 = nodeVar93;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar95 = ( indirectDiffuse + nodeVar94 );
	indirectDiffuse = nodeVar95;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar96 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar97 = ( SpecularF90 * dfg.y );
	nodeVar98 = ( nodeVar96 + vec3<f32>( nodeVar97 ) );
	nodeVar99 = ( singleScatteringDielectric + nodeVar98 );
	singleScatteringDielectric = nodeVar99;
	nodeVar100 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar101 = nodeVar100;
	nodeVar102 = ( nodeVar101 * vec3<f32>( 0.047619 ) );
	nodeVar103 = ( SpecularColor + nodeVar102 );
	nodeVar104 = ( nodeVar98 * nodeVar103 );
	nodeVar105 = ( dfg.x + dfg.y );
	nodeVar106 = ( 1.0 - nodeVar105 );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( vec3<f32>( nodeVar107 ) * nodeVar103 );
	nodeVar109 = ( vec3<f32>( 1.0 ) - nodeVar108 );
	nodeVar110 = nodeVar109;
	nodeVar111 = ( nodeVar104 / nodeVar110 );
	nodeVar112 = ( nodeVar111 * vec3<f32>( nodeVar107 ) );
	nodeVar113 = ( multiScatteringDielectric + nodeVar112 );
	multiScatteringDielectric = nodeVar113;
	nodeVar114 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar115 = ( SpecularF90 * dfg.y );
	nodeVar116 = ( nodeVar114 + vec3<f32>( nodeVar115 ) );
	nodeVar117 = ( singleScatteringMetallic + nodeVar116 );
	singleScatteringMetallic = nodeVar117;
	nodeVar118 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar119 = nodeVar118;
	nodeVar120 = ( nodeVar119 * vec3<f32>( 0.047619 ) );
	nodeVar121 = ( DiffuseColor.xyz + nodeVar120 );
	nodeVar122 = ( nodeVar116 * nodeVar121 );
	nodeVar123 = ( dfg.x + dfg.y );
	nodeVar124 = ( 1.0 - nodeVar123 );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( vec3<f32>( nodeVar125 ) * nodeVar121 );
	nodeVar127 = ( vec3<f32>( 1.0 ) - nodeVar126 );
	nodeVar128 = nodeVar127;
	nodeVar129 = ( nodeVar122 / nodeVar128 );
	nodeVar130 = ( nodeVar129 * vec3<f32>( nodeVar125 ) );
	nodeVar131 = ( multiScatteringMetallic + nodeVar130 );
	multiScatteringMetallic = nodeVar131;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar132 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar133 = ( radiance * nodeVar132 );
	nodeVar134 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar135 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar136 = ( nodeVar134 * nodeVar135 );
	nodeVar137 = ( nodeVar133 + nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar140 = ( vec3<f32>( 1.0 ) - nodeVar139 );
	nodeVar141 = nodeVar140;
	nodeVar142 = ( DiffuseContribution * nodeVar141 );
	nodeVar143 = ( nodeVar142 * nodeVar135 );
	nodeVar144 = nodeVar143;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar145 = ( indirectSpecular + nodeVar138 );
	indirectSpecular = nodeVar145;
	nodeVar146 = ( indirectDiffuse + nodeVar144 );
	indirectDiffuse = nodeVar146;
	ambientOcclusion = 1.0;
	nodeVar147 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar147;
	nodeVar148 = dot( normalView, positionViewDirection );
	nodeVar149 = ( clamp( nodeVar148, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar150 = ( Roughness * -16.0 );
	nodeVar151 = ( 1.0 - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( - nodeVar152 );
	nodeVar154 = exp2( nodeVar153 );
	nodeVar155 = pow( nodeVar149, nodeVar154 );
	nodeVar156 = ( 1.0 - nodeVar155 );
	nodeVar157 = nodeVar156;
	nodeVar158 = ( ambientOcclusion - nodeVar157 );
	nodeVar159 = ( indirectSpecular * vec3<f32>( clamp( nodeVar158, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar159;
	nodeVar160 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar160;
	nodeVar161 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar161;
	nodeVar162 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar162;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	output.m1 = DiffuseColor;
	nodeVar163 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m2 = vec4<f32>( nodeVar163, 1.0 );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform37 );
	nodeVar164 = ( ( object.nodeUniform36 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar165 = ( ( render.nodeUniform38 * ( object.nodeUniform39 * object.nodeUniform40 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar166 = ( ( nodeVar164.xy / vec2<f32>( nodeVar164.w ) ) - ( nodeVar165.xy / vec2<f32>( nodeVar165.w ) ) );
	output.m3 = vec4<f32>( vec3<f32>( nodeVar166, 0.0 ), 1.0 );

	// result

	return output;

}
