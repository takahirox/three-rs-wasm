// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform18_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform18 : texture_depth_2d;

struct renderStruct {
	nodeUniform2 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform21 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> normalLocal : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
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
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
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
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec4<f32>;

// codes
fn fn6 ( p : vec3<f32> ) -> vec3<f32> {

	


	return fract( ( sin( vec3<f32>( dot( p, vec3<f32>( 127.1, 311.7, 74.7 ) ), dot( p, vec3<f32>( 269.5, 183.3, 246.1 ) ), dot( p, vec3<f32>( 113.5, 271.9, 124.6 ) ) ) ) * vec3<f32>( 18.5453 ) ) );

}


fn fn7 ( p : vec3<f32>, time : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : vec3<f32>;

	let nodeConst0 = floor( p );
	let nodeConst1 = fract( p );
	nodeVar0 = 8.0;

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {


			for ( var z : i32 = -1; z <= 1; z ++ ) {

				let nodeConst2 = vec3<f32>( f32( x ), f32( y ), f32( z ) );
				let nodeConst3 = fn6( ( nodeConst0 + nodeConst2 ) );
				nodeVar1 = ( ( nodeConst2 - nodeConst1 ) + ( ( sin( ( vec3<f32>( time ) + ( nodeConst3 * vec3<f32>( 6.283185307179586 ) ) ) ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ) );
				nodeVar0 = min( nodeVar0, dot( nodeVar1, nodeVar1 ) );

			}


		}


	}


	return nodeVar0;

}


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
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) nodeVarying7 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, ( positionLocal.yz * vec2<f32>( 1.0 ) ) );
	normalLocal = nodeVarying7;
	nodeVar1 = normalize( abs( normalLocal ) );
	nodeVar2 = ( nodeVar1 / vec3<f32>( dot( nodeVar1, vec3<f32>( 1.0, 1.0, 1.0 ) ) ) );
	nodeVar3 = textureSample( nodeUniform0, nodeUniform0_sampler, ( positionLocal.zx * vec2<f32>( 1.0 ) ) );
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, ( positionLocal.xy * vec2<f32>( 1.0 ) ) );
	nodeVar5 = ( ( ( ( ( nodeVar0 * vec4<f32>( nodeVar2.x ) ) + ( nodeVar3 * vec4<f32>( nodeVar2.y ) ) ) + ( nodeVar4 * vec4<f32>( nodeVar2.z ) ) ) + vec4<f32>( vec3<f32>( 0.0, 0.13286832154414627, 1.0 ), 1.0 ) ) * vec4<f32>( 0.8 ) );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	DiffuseColor = mix( nodeVar5, ( nodeVar5 + vec4<f32>( fn7( ( v_positionWorld * vec3<f32>( 6.0 ) ), ( render.nodeUniform2 * 0.8 ) ) ) ), mix( clamp( ( 1.0 - distance( v_positionWorld.y, 0.0 ) ), 0.0, 1.0 ), 0.0, normalWorld.y ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform7;
	nodeVar6 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform8, 0.0525 ) + max( max( nodeVar6.x, nodeVar6.y ), nodeVar6.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform7 ) ) );
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar7 = dot( normalView, positionViewDirection );
	nodeVar8 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( nodeVar7, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar8;
	nodeVar9 = ( dfg.x + dfg.y );
	nodeVar10 = ( 1.0 / nodeVar9 );
	nodeVar11 = nodeVar10;
	nodeVar12 = ( nodeVar11 - 1.0 );
	nodeVar13 = ( SpecularColorBlended * vec3<f32>( nodeVar12 ) );
	nodeVar14 = ( nodeVar13 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar14;
	nodeVar15 = ( render.nodeUniform12 - render.nodeUniform13 );
	nodeVar16 = vec4<f32>( nodeVar15, 0.0 );
	nodeVar17 = ( render.cameraViewMatrix * nodeVar16 );
	nodeVar18 = normalize( nodeVar17.xyz );
	nodeVar19 = nodeVar18;
	nodeVar20 = dot( normalView, nodeVar19 );
	shadowPositionWorld = v_positionWorld;
	nodeVar22 = ( render.nodeUniform15 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform16 ) ) ), 1.0 ) );
	nodeVar23 = ( nodeVar22.xyz / vec3<f32>( nodeVar22.w ) );
	nodeVar24 = vec3<f32>( nodeVar23.x, ( 1.0 - nodeVar23.y ), ( nodeVar23.z + render.nodeUniform17 ) );

	if ( ( ( ( ( ( nodeVar24.x >= 0.0 ) && ( nodeVar24.x <= 1.0 ) ) && ( nodeVar24.y >= 0.0 ) ) && ( nodeVar24.y <= 1.0 ) ) && ( nodeVar24.z <= 1.0 ) ) ) {

		nodeVar25 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar26 = ( render.nodeUniform19 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform20 ).x );
		nodeVar27 = ( nodeVar24.xy + ( vogelDiskSample( 0, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar28 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar27, nodeVar24.z );
		nodeVar29 = ( nodeVar24.xy + ( vogelDiskSample( 1, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar30 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar29, nodeVar24.z );
		nodeVar31 = ( nodeVar24.xy + ( vogelDiskSample( 2, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar32 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar31, nodeVar24.z );
		nodeVar33 = ( nodeVar24.xy + ( vogelDiskSample( 3, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar34 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar33, nodeVar24.z );
		nodeVar35 = ( nodeVar24.xy + ( vogelDiskSample( 4, 5, nodeVar25 ) * vec2<f32>( nodeVar26 ) ) );
		nodeVar36 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar35, nodeVar24.z );
		nodeVar21 = ( ( ( ( ( nodeVar28 + nodeVar30 ) + nodeVar32 ) + nodeVar34 ) + nodeVar36 ) * 0.2 );

	} else {

		nodeVar21 = 1.0;

	}

	nodeVar37 = mix( 1.0, nodeVar21, render.nodeUniform21 );
	nodeVar38 = ( vec3<f32>( clamp( nodeVar20, 0.0, 1.0 ) ) * ( render.nodeUniform14 * vec3<f32>( nodeVar37 ) ) );
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
	nodeVar60 = dot( normalWorld, normalize( render.nodeUniform24 ) );
	nodeVar61 = ( nodeVar60 * 0.5 );
	nodeVar62 = ( nodeVar61 + 0.5 );
	nodeVar63 = mix( render.nodeUniform22, render.nodeUniform23, nodeVar62 );
	nodeVar64 = ( irradiance + nodeVar63 );
	irradiance = nodeVar64;
	nodeVar65 = dot( normalWorld, normalize( render.nodeUniform27 ) );
	nodeVar66 = ( nodeVar65 * 0.5 );
	nodeVar67 = ( nodeVar66 + 0.5 );
	nodeVar68 = mix( render.nodeUniform25, render.nodeUniform26, nodeVar67 );
	nodeVar69 = ( irradiance + nodeVar68 );
	irradiance = nodeVar69;
	nodeVar70 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar71 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar72 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar73 = ( SpecularF90 * dfg.y );
	nodeVar74 = ( nodeVar72 + vec3<f32>( nodeVar73 ) );
	nodeVar75 = ( nodeVar70 + nodeVar74 );
	nodeVar70 = nodeVar75;
	nodeVar76 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar77 = nodeVar76;
	nodeVar78 = ( nodeVar77 * vec3<f32>( 0.047619 ) );
	nodeVar79 = ( SpecularColor + nodeVar78 );
	nodeVar80 = ( nodeVar74 * nodeVar79 );
	nodeVar81 = ( dfg.x + dfg.y );
	nodeVar82 = ( 1.0 - nodeVar81 );
	nodeVar83 = nodeVar82;
	nodeVar84 = ( vec3<f32>( nodeVar83 ) * nodeVar79 );
	nodeVar85 = ( vec3<f32>( 1.0 ) - nodeVar84 );
	nodeVar86 = nodeVar85;
	nodeVar87 = ( nodeVar80 / nodeVar86 );
	nodeVar88 = ( nodeVar87 * vec3<f32>( nodeVar83 ) );
	nodeVar89 = ( nodeVar71 + nodeVar88 );
	nodeVar71 = nodeVar89;
	nodeVar90 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar91 = ( irradiance * nodeVar90 );
	nodeVar92 = ( nodeVar70 + nodeVar71 );
	nodeVar93 = ( vec3<f32>( 1.0 ) - nodeVar92 );
	nodeVar94 = nodeVar93;
	nodeVar95 = ( nodeVar91 * nodeVar94 );
	nodeVar96 = nodeVar95;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar97 = ( indirectDiffuse + nodeVar96 );
	indirectDiffuse = nodeVar97;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar98 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar99 = ( SpecularF90 * dfg.y );
	nodeVar100 = ( nodeVar98 + vec3<f32>( nodeVar99 ) );
	nodeVar101 = ( singleScatteringDielectric + nodeVar100 );
	singleScatteringDielectric = nodeVar101;
	nodeVar102 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar103 = nodeVar102;
	nodeVar104 = ( nodeVar103 * vec3<f32>( 0.047619 ) );
	nodeVar105 = ( SpecularColor + nodeVar104 );
	nodeVar106 = ( nodeVar100 * nodeVar105 );
	nodeVar107 = ( dfg.x + dfg.y );
	nodeVar108 = ( 1.0 - nodeVar107 );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( vec3<f32>( nodeVar109 ) * nodeVar105 );
	nodeVar111 = ( vec3<f32>( 1.0 ) - nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( nodeVar106 / nodeVar112 );
	nodeVar114 = ( nodeVar113 * vec3<f32>( nodeVar109 ) );
	nodeVar115 = ( multiScatteringDielectric + nodeVar114 );
	multiScatteringDielectric = nodeVar115;
	nodeVar116 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar117 = ( SpecularF90 * dfg.y );
	nodeVar118 = ( nodeVar116 + vec3<f32>( nodeVar117 ) );
	nodeVar119 = ( singleScatteringMetallic + nodeVar118 );
	singleScatteringMetallic = nodeVar119;
	nodeVar120 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar121 = nodeVar120;
	nodeVar122 = ( nodeVar121 * vec3<f32>( 0.047619 ) );
	nodeVar123 = ( DiffuseColor.xyz + nodeVar122 );
	nodeVar124 = ( nodeVar118 * nodeVar123 );
	nodeVar125 = ( dfg.x + dfg.y );
	nodeVar126 = ( 1.0 - nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( vec3<f32>( nodeVar127 ) * nodeVar123 );
	nodeVar129 = ( vec3<f32>( 1.0 ) - nodeVar128 );
	nodeVar130 = nodeVar129;
	nodeVar131 = ( nodeVar124 / nodeVar130 );
	nodeVar132 = ( nodeVar131 * vec3<f32>( nodeVar127 ) );
	nodeVar133 = ( multiScatteringMetallic + nodeVar132 );
	multiScatteringMetallic = nodeVar133;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar134 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar135 = ( radiance * nodeVar134 );
	nodeVar136 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar137 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar138 = ( nodeVar136 * nodeVar137 );
	nodeVar139 = ( nodeVar135 + nodeVar138 );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar142 = ( vec3<f32>( 1.0 ) - nodeVar141 );
	nodeVar143 = nodeVar142;
	nodeVar144 = ( DiffuseContribution * nodeVar143 );
	nodeVar145 = ( nodeVar144 * nodeVar137 );
	nodeVar146 = nodeVar145;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar147 = ( indirectSpecular + nodeVar140 );
	indirectSpecular = nodeVar147;
	nodeVar148 = ( indirectDiffuse + nodeVar146 );
	indirectDiffuse = nodeVar148;
	ambientOcclusion = 1.0;
	nodeVar149 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar149;
	nodeVar150 = dot( normalView, positionViewDirection );
	nodeVar151 = ( clamp( nodeVar150, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar152 = ( Roughness * -16.0 );
	nodeVar153 = ( 1.0 - nodeVar152 );
	nodeVar154 = nodeVar153;
	nodeVar155 = ( - nodeVar154 );
	nodeVar156 = exp2( nodeVar155 );
	nodeVar157 = pow( nodeVar151, nodeVar156 );
	nodeVar158 = ( 1.0 - nodeVar157 );
	nodeVar159 = nodeVar158;
	nodeVar160 = ( ambientOcclusion - nodeVar159 );
	nodeVar161 = ( indirectSpecular * vec3<f32>( clamp( nodeVar160, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar161;
	nodeVar162 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar162;
	nodeVar163 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar163;
	nodeVar164 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar164;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar165 = vec4<f32>( mix( Output.xyz, render.nodeUniform28, smoothstep( render.nodeUniform29, render.nodeUniform30, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar165;

	// result

	output.color = nodeVar165;

	return output;

}
