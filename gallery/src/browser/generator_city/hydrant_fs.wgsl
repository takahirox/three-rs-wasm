// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform18_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform18 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform26_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform26 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform28_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform28 : texture_cube<f32>;

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform29 : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform21 : f32,
	nodeUniform27 : f32,
	nodeUniform25 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> Metalness : f32;
var<private> nodeVar1 : f32;
var<private> Roughness : f32;
var<private> nodeVar2 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar3 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn48 ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn fn65 ( t : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = t;

	return ( ( ( nodeVar0 * nodeVar0 ) * nodeVar0 ) * ( ( nodeVar0 * ( ( nodeVar0 * 6.0 ) - 15.0 ) ) + 10.0 ) );

}


fn fn61 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn fn63 ( a : u32, b : u32, c : u32 ) -> u32 {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = c;
	nodeVar1 = b;
	nodeVar2 = a;
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 14 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 11 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 25 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 16 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 4 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 14 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 24 ) );

	return nodeVar0;

}


fn fn68 ( x : i32, y : i32, z : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;
	var nodeVar6 : u32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = 3u;
	nodeVar4 = 0u;
	nodeVar5 = 0u;
	nodeVar6 = 0u;
	nodeVar6 = ( ( 3735928559u + ( nodeVar3 << 2u ) ) + 13u );
	nodeVar5 = nodeVar6;
	nodeVar4 = nodeVar5;
	nodeVar4 = ( nodeVar4 + u32( nodeVar2 ) );
	nodeVar5 = ( nodeVar5 + u32( nodeVar1 ) );
	nodeVar6 = ( nodeVar6 + u32( nodeVar0 ) );

	return fn63( nodeVar4, nodeVar5, nodeVar6 );

}


fn fn46 ( b : bool, t : f32, f : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : bool;
	var nodeVar3 : f32;

	nodeVar0 = f;
	nodeVar1 = t;
	nodeVar2 = b;

	return select( nodeVar0, nodeVar1, nodeVar2 );

}


fn fn47 ( val : f32, b : bool ) -> f32 {

	var nodeVar0 : bool;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = b;
	nodeVar1 = val;

	return select( nodeVar1, ( - nodeVar1 ), nodeVar0 );

}


fn fn54 ( hash : u32, x : f32, y : f32, z : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = hash;
	nodeVar4 = ( nodeVar3 & 15u );
	nodeVar5 = fn46( ( nodeVar4 < 8u ), nodeVar2, nodeVar1 );
	nodeVar6 = fn46( ( nodeVar4 < 4u ), nodeVar1, fn46( ( ( nodeVar4 == 12u ) || ( nodeVar4 == 14u ) ), nodeVar2, nodeVar0 ) );

	return ( fn47( nodeVar5, bool( ( nodeVar4 & 1u ) ) ) + fn47( nodeVar6, bool( ( nodeVar4 & 2u ) ) ) );

}


fn fn51 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, v4 : f32, v5 : f32, v6 : f32, v7 : f32, s : f32, t : f32, r : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : f32;

	nodeVar0 = r;
	nodeVar1 = t;
	nodeVar2 = s;
	nodeVar3 = v7;
	nodeVar4 = v6;
	nodeVar5 = v5;
	nodeVar6 = v4;
	nodeVar7 = v3;
	nodeVar8 = v2;
	nodeVar9 = v1;
	nodeVar10 = v0;
	nodeVar11 = ( 1.0 - nodeVar2 );
	nodeVar12 = ( 1.0 - nodeVar1 );
	nodeVar13 = ( 1.0 - nodeVar0 );

	return ( ( nodeVar13 * ( ( nodeVar12 * ( ( nodeVar10 * nodeVar11 ) + ( nodeVar9 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar8 * nodeVar11 ) + ( nodeVar7 * nodeVar2 ) ) ) ) ) + ( nodeVar0 * ( ( nodeVar12 * ( ( nodeVar6 * nodeVar11 ) + ( nodeVar5 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar4 * nodeVar11 ) + ( nodeVar3 * nodeVar2 ) ) ) ) ) );

}


fn fn58 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.982 * nodeVar0 );

}


fn fn74 ( p : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : f32;

	nodeVar0 = p;
	nodeVar1 = 0;
	nodeVar2 = 0;
	nodeVar3 = 0;
	nodeVar4 = nodeVar0.x;
	nodeVar1 = fn48( nodeVar4 );
	nodeVar5 = ( nodeVar4 - f32( nodeVar1 ) );
	nodeVar6 = nodeVar0.y;
	nodeVar2 = fn48( nodeVar6 );
	nodeVar7 = ( nodeVar6 - f32( nodeVar2 ) );
	nodeVar8 = nodeVar0.z;
	nodeVar3 = fn48( nodeVar8 );
	nodeVar9 = ( nodeVar8 - f32( nodeVar3 ) );
	nodeVar10 = fn65( nodeVar5 );
	nodeVar11 = fn65( nodeVar7 );
	nodeVar12 = fn65( nodeVar9 );
	nodeVar13 = fn51( fn54( fn68( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), fn54( fn68( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), fn54( fn68( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), fn54( fn68( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), fn54( fn68( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), fn54( fn68( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), fn54( fn68( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), fn54( fn68( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return fn58( nodeVar13 );

}


fn fn87 ( p : vec3<f32>, octaves : i32, lacunarity : f32, diminish : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : i32;

	nodeVar0 = diminish;
	nodeVar1 = lacunarity;
	nodeVar2 = p;
	nodeVar3 = 0.0;
	nodeVar4 = 1.0;
	nodeVar5 = octaves;

	for ( var i : i32 = 0; i < nodeVar5; i ++ ) {

		nodeVar3 = ( nodeVar3 + ( nodeVar4 * fn74( nodeVar2 ) ) );
		nodeVar4 = ( nodeVar4 * nodeVar0 );
		nodeVar2 = ( nodeVar2 * vec3<f32>( nodeVar1 ) );

	}


	return nodeVar3;

}


fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


fn fn110 ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn fn112 ( alpha : f32, dotNH : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );
	let nodeConst1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeConst0 ) ) );

	return ( ( nodeConst0 / ( nodeConst1 * nodeConst1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst1 = ( nodeVarying3 == 1.0 );

	if ( nodeConst1 ) {

		nodeVar0 = vec3<f32>( 0.2541520943200296, 0.2541520943200296, 0.21586050010324417 );

	} else {

		let nodeConst2 = ( ( ( fn87( ( nodeVarying8 * vec3<f32>( 9.0 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 );
		let nodeConst3 = max( smoothstep( 0.28, 0.02, nodeVarying8.y ), ( smoothstep( 0.55, 0.85, nodeConst2 ) * 0.5 ) );
		nodeVar0 = mix( mix( vec3<f32>( 0.2746773120495699, 0.028426039499072558, 0.012983032338510335 ), vec3<f32>( 0.4793201830913402, 0.10224173307914941, 0.03954623527052923 ), ( nodeConst2 * 0.6 ) ), vec3<f32>( 0.06847816983662762, 0.025186859622305935, 0.010329823026364548 ), ( nodeConst3 * 0.8 ) );

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;

	if ( nodeConst1 ) {

		nodeVar1 = 0.8;

	} else {

		nodeVar1 = 0.15;

	}

	let nodeConst4 = nodeVar1;
	Metalness = nodeConst4;

	if ( nodeConst1 ) {

		nodeVar2 = 0.45;

	} else {

		let nodeConst2 = ( ( ( fn87( ( nodeVarying8 * vec3<f32>( 9.0 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 );
		let nodeConst3 = max( smoothstep( 0.28, 0.02, nodeVarying8.y ), ( smoothstep( 0.55, 0.85, nodeConst2 ) * 0.5 ) );
		nodeVar2 = ( ( ( nodeConst2 * 0.3 ) + ( nodeConst3 * 0.25 ) ) + 0.5 );

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst5 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar2, 0.045 ) + max( max( nodeConst5.x, nodeConst5.y ), nodeConst5.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeConst4 ) ) );
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar3 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst6 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst6 );
	let nodeConst7 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst8 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst6 * nodeConst7 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst8 ) * nodeConst7 ) ) ) * vec3<f32>( nodeConst8 ) ) );
	let nodeConst9 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform12 - render.nodeUniform13 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst10 = ( render.nodeUniform15 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform16 ) ) ), 1.0 ) );
	let nodeConst11 = ( nodeConst10.xyz / vec3<f32>( nodeConst10.w ) );
	let nodeConst12 = vec3<f32>( nodeConst11.x, ( 1.0 - nodeConst11.y ), ( nodeConst11.z + render.nodeUniform17 ) );

	if ( ( ( ( ( ( nodeConst12.x >= 0.0 ) && ( nodeConst12.x <= 1.0 ) ) && ( nodeConst12.y >= 0.0 ) ) && ( nodeConst12.y <= 1.0 ) ) && ( nodeConst12.z <= 1.0 ) ) ) {

		let nodeConst13 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst14 = ( vec2<f32>( render.nodeUniform19 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform20 ) );
		let nodeConst15 = ( nodeConst12.xy + ( fn25( 0, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar5 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst15, nodeConst12.z );
		let nodeConst16 = ( nodeConst12.xy + ( fn25( 1, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar6 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst16, nodeConst12.z );
		let nodeConst17 = ( nodeConst12.xy + ( fn25( 2, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar7 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst17, nodeConst12.z );
		let nodeConst18 = ( nodeConst12.xy + ( fn25( 3, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar8 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst18, nodeConst12.z );
		let nodeConst19 = ( nodeConst12.xy + ( fn25( 4, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar9 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst19, nodeConst12.z );
		nodeVar4 = ( ( ( ( ( nodeVar5 + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) + nodeVar9 ) * 0.2 );

	} else {

		nodeVar4 = 1.0;

	}

	nodeVar10 = mix( 1.0, nodeVar4, render.nodeUniform21 );
	nodeVar11 = ( vec3<f32>( clamp( dot( normalView, nodeConst9 ), 0.0, 1.0 ) ) * ( render.nodeUniform14 * vec3<f32>( nodeVar10 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst20 = clamp( dot( positionViewDirection, normalize( ( nodeConst9 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst21 = exp2( ( ( ( nodeConst20 * -5.55473 ) - 6.98316 ) * nodeConst20 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar11 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst21 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst21 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst22 = normalize( ( nodeConst9 + positionViewDirection ) );
	let nodeConst23 = clamp( dot( positionViewDirection, nodeConst22 ), 0.0, 1.0 );
	let nodeConst24 = exp2( ( ( ( nodeConst23 * -5.55473 ) - 6.98316 ) * nodeConst23 ) );
	let nodeConst25 = max( Roughness, 0.045 );
	let nodeConst26 = ( nodeConst25 * nodeConst25 );
	directSpecular = ( directSpecular + ( ( nodeVar11 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst24 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst24 ) ) ) * vec3<f32>( fn110( nodeConst26, clamp( dot( normalView, nodeConst9 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst26, clamp( dot( normalView, nodeConst22 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst27 = ( ( object.nodeUniform22 - object.nodeUniform23 ) / object.nodeUniform24 );
	let nodeConst28 = ( v_positionWorld + ( ( normalWorld * nodeConst27 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst29 = ( nodeConst27 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst28 >= ( object.nodeUniform23 - nodeConst29 ) ) ) && all( ( nodeConst28 <= ( object.nodeUniform22 + nodeConst29 ) ) ) ) && ( render.nodeUniform25 >= lightProbePriority ) ) ) {

		let nodeConst30 = ( object.nodeUniform22 - object.nodeUniform23 );
		let nodeConst31 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform24 );
		let nodeConst32 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst30 / object.nodeUniform24 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform23 ) / nodeConst30 ), nodeConst31, ( vec3<f32>( 1.0 ) - nodeConst31 ) );
		let nodeConst33 = ( ( nodeConst32.z * object.nodeUniform24.z ) + 1.0 );
		let nodeConst34 = ( object.nodeUniform24.z + 2.0 );
		let nodeConst35 = ( nodeConst34 * 7.0 );
		nodeVar12 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst32.xy, ( ( nodeConst33 + ( nodeConst34 * 0.0 ) ) / nodeConst35 ) ) );
		nodeVar13 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst32.xy, ( ( nodeConst33 + ( nodeConst34 * 1.0 ) ) / nodeConst35 ) ) );
		nodeVar14 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst32.xy, ( ( nodeConst33 + ( nodeConst34 * 2.0 ) ) / nodeConst35 ) ) );
		nodeVar15 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst32.xy, ( ( nodeConst33 + ( nodeConst34 * 3.0 ) ) / nodeConst35 ) ) );
		nodeVar16 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst32.xy, ( ( nodeConst33 + ( nodeConst34 * 4.0 ) ) / nodeConst35 ) ) );
		nodeVar17 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst32.xy, ( ( nodeConst33 + ( nodeConst34 * 5.0 ) ) / nodeConst35 ) ) );
		nodeVar18 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst32.xy, ( ( nodeConst33 + ( nodeConst34 * 6.0 ) ) / nodeConst35 ) ) );
		let nodeConst36 = array< vec3<f32>, 9 >( nodeVar12.xyz, vec3<f32>( nodeVar12.w, nodeVar13.xy ), vec3<f32>( nodeVar13.zw, nodeVar14.x ), nodeVar14.yzw, nodeVar15.xyz, vec3<f32>( nodeVar15.w, nodeVar16.xy ), vec3<f32>( nodeVar16.zw, nodeVar17.x ), nodeVar17.yzw, nodeVar18.xyz );
		let nodeConst37 = ( max( ( ( ( ( ( ( ( ( ( nodeConst36[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst36[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst36[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst36[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst36[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst36[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst36[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst36[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst36[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform27 ) );
		lightProbeIrradiance = nodeConst37;
		lightProbePriority = render.nodeUniform25;
		

	}

	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst38 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst39 = ( object.nodeUniform29 * vec4<f32>( nodeConst38, 1.0 ) );
	let nodeConst40 = clamp( Roughness, 0.0, 1.0 );
	nodeVar19 = textureSampleLevel( nodeUniform28, nodeUniform28_sampler, vec3<f32>( ( - nodeConst39.x ), nodeConst39.yz ), ( ( object.nodeUniform31 * nodeConst40 ) * ( 2.0 - nodeConst40 ) ) );
	radiance = ( radiance + ( nodeVar19.xyz * vec3<f32>( object.nodeUniform32 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst41 = ( object.nodeUniform29 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst42 = clamp( 1.0, 0.0, 1.0 );
	nodeVar20 = textureSampleLevel( nodeUniform28, nodeUniform28_sampler, vec3<f32>( ( - nodeConst41.x ), nodeConst41.yz ), ( ( object.nodeUniform31 * nodeConst42 ) * ( 2.0 - nodeConst42 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar20.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform32 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar21 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar21 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst43 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst43 );
	let nodeConst44 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst45 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst43 * nodeConst44 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst45 ) * nodeConst44 ) ) ) * vec3<f32>( nodeConst45 ) ) );
	let nodeConst46 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar22 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst46 ) );
	nodeVar23 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst46 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar22 );
	indirectDiffuse = ( indirectDiffuse + nodeVar23 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst47 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst47;

	// result

	output.color = nodeConst47;

	return output;

}
