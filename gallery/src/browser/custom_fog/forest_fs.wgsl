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
@binding( 1 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform24_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform24 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform28_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform28 : texture_cube<f32>;

struct objectStruct {
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform29 : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform35 : f32,
	nodeUniform33 : f32,
	nodeUniform36 : f32,
	nodeUniform34 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : vec2<f32>,
	nodeUniform27 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
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
var<private> nodeVar1 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;

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


fn fn14 ( x : f32 ) -> f32 {

	


	return abs( ( fract( x ) - 0.5 ) );

}


fn fn15 ( p : vec3<f32> ) -> vec3<f32> {

	


	return vec3<f32>( fn14( ( p.z + fn14( ( p.y * 1.0 ) ) ) ), fn14( ( p.z + fn14( ( p.x * 1.0 ) ) ) ), fn14( ( p.y + fn14( ( p.x * 1.0 ) ) ) ) );

}


fn fn16 ( position : vec3<f32>, speed : f32, time : f32 ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : vec3<f32>;
	var nodeVar5 : f32;

	nodeVar0 = position;
	nodeVar1 = 1.4;
	nodeVar2 = 0.0;
	nodeVar3 = nodeVar0;

	for ( var i : f32 = 0.0; i <= 3.0; i += 1. ) {

		nodeVar4 = fn15( ( nodeVar3 * vec3<f32>( 2.0 ) ) );
		nodeVar0 = ( nodeVar0 + ( nodeVar4 + vec3<f32>( ( time * ( 0.1 * speed ) ) ) ) );
		nodeVar3 = ( nodeVar3 * vec3<f32>( 1.8 ) );
		nodeVar1 = ( nodeVar1 * 1.5 );
		nodeVar0 = ( nodeVar0 * vec3<f32>( 1.2 ) );
		nodeVar5 = fn14( ( nodeVar0.z + fn14( ( nodeVar0.x + fn14( nodeVar0.y ) ) ) ) );
		nodeVar2 = ( nodeVar2 + ( nodeVar5 / nodeVar1 ) );
		nodeVar3 = ( nodeVar3 + vec3<f32>( 0.14 ) );

	}


	return nodeVar2;

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) nodeVarying7 : f32,
	@location( 5 ) nodeVarying8 : f32,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = 0.0;
	let nodeConst1 = smoothstep( 280.0, 25.0, distance( v_positionWorld, object.nodeUniform8 ) );

	if ( ( nodeConst1 > 0.01 ) ) {

		nodeVar0 = ( ( ( ( fn74( ( v_positionWorld * vec3<f32>( 0.9 ) ) ) * 1.0 ) + 0.0 ) + ( ( ( fn74( ( v_positionWorld * vec3<f32>( 3.1 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) ) * nodeConst1 );
		

	}

	DiffuseColor = vec4<f32>( mix( mix( vec3<f32>( 0.012286488353353374, 0.033104766565152086, 0.009134058699157796 ), vec3<f32>( 0.02732089163382382, 0.057805430183792694, 0.014443843592229466 ), nodeVarying7 ), mix( vec3<f32>( 0.0722718506743852, 0.14412847084818123, 0.02732089163382382 ), vec3<f32>( 0.15592646369776456, 0.2541520943200296, 0.05126945836711539 ), nodeVarying7 ), clamp( ( ( ( nodeVarying8 * 0.5 ) + 0.32 ) + ( nodeVar0 * 0.18 ) ), 0.0, 1.0 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform9 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform10;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst2 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform11, 0.045 ) + max( max( nodeConst2.x, nodeConst2.y ), nodeConst2.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform10 ) ) );
	EmissiveColor = ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) );
	let nodeConst3 = dpdx( v_positionView );
	let nodeConst4 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	let NORMAL_nodeConst5 = cross( nodeConst4, NORMAL_normalView );
	let NORMAL_nodeConst6 = dot( nodeConst3, NORMAL_nodeConst5 );
	let nodeConst7 = ( nodeVar0 * 0.22 );
	let nodeConst8 = ( vec3<f32>( sign( NORMAL_nodeConst6 ) ) * ( ( vec3<f32>( dpdx( nodeConst7 ) ) * NORMAL_nodeConst5 ) + ( vec3<f32>( - dpdy( nodeConst7 ) ) * cross( NORMAL_normalView, nodeConst3 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeConst6 ) ) * NORMAL_normalView ) - nodeConst8 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst9 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst9 );
	let nodeConst10 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst11 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst9 * nodeConst10 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst11 ) * nodeConst10 ) ) ) * vec3<f32>( nodeConst11 ) ) );
	let nodeConst12 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform18 - render.nodeUniform19 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst13 = ( render.nodeUniform21 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform22 ) ) ), 1.0 ) );
	let nodeConst14 = ( nodeConst13.xyz / vec3<f32>( nodeConst13.w ) );
	let nodeConst15 = vec3<f32>( nodeConst14.x, ( 1.0 - nodeConst14.y ), ( nodeConst14.z + render.nodeUniform23 ) );

	if ( ( ( ( ( ( nodeConst15.x >= 0.0 ) && ( nodeConst15.x <= 1.0 ) ) && ( nodeConst15.y >= 0.0 ) ) && ( nodeConst15.y <= 1.0 ) ) && ( nodeConst15.z <= 1.0 ) ) ) {

		let nodeConst16 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst17 = ( vec2<f32>( render.nodeUniform25 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform26 ) );
		let nodeConst18 = ( nodeConst15.xy + ( fn25( 0, 5, nodeConst16 ) * nodeConst17 ) );
		nodeVar3 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeConst18, nodeConst15.z );
		let nodeConst19 = ( nodeConst15.xy + ( fn25( 1, 5, nodeConst16 ) * nodeConst17 ) );
		nodeVar4 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeConst19, nodeConst15.z );
		let nodeConst20 = ( nodeConst15.xy + ( fn25( 2, 5, nodeConst16 ) * nodeConst17 ) );
		nodeVar5 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeConst20, nodeConst15.z );
		let nodeConst21 = ( nodeConst15.xy + ( fn25( 3, 5, nodeConst16 ) * nodeConst17 ) );
		nodeVar6 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeConst21, nodeConst15.z );
		let nodeConst22 = ( nodeConst15.xy + ( fn25( 4, 5, nodeConst16 ) * nodeConst17 ) );
		nodeVar7 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, nodeConst22, nodeConst15.z );
		nodeVar2 = ( ( ( ( ( nodeVar3 + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) * 0.2 );

	} else {

		nodeVar2 = 1.0;

	}

	nodeVar8 = mix( 1.0, nodeVar2, render.nodeUniform27 );
	nodeVar9 = ( vec3<f32>( clamp( dot( normalView, nodeConst12 ), 0.0, 1.0 ) ) * ( render.nodeUniform20 * vec3<f32>( nodeVar8 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst23 = clamp( dot( positionViewDirection, normalize( ( nodeConst12 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst24 = exp2( ( ( ( nodeConst23 * -5.55473 ) - 6.98316 ) * nodeConst23 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar9 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst24 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst24 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst25 = normalize( ( nodeConst12 + positionViewDirection ) );
	let nodeConst26 = clamp( dot( positionViewDirection, nodeConst25 ), 0.0, 1.0 );
	let nodeConst27 = exp2( ( ( ( nodeConst26 * -5.55473 ) - 6.98316 ) * nodeConst26 ) );
	let nodeConst28 = max( Roughness, 0.045 );
	let nodeConst29 = ( nodeConst28 * nodeConst28 );
	directSpecular = ( directSpecular + ( ( nodeVar9 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst27 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst27 ) ) ) * vec3<f32>( fn110( nodeConst29, clamp( dot( normalView, nodeConst12 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst29, clamp( dot( normalView, nodeConst25 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst30 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst31 = ( object.nodeUniform29 * vec4<f32>( nodeConst30, 1.0 ) );
	let nodeConst32 = clamp( Roughness, 0.0, 1.0 );
	nodeVar10 = textureSampleLevel( nodeUniform28, nodeUniform28_sampler, vec3<f32>( ( - nodeConst31.x ), nodeConst31.yz ), ( ( object.nodeUniform31 * nodeConst32 ) * ( 2.0 - nodeConst32 ) ) );
	radiance = ( radiance + ( nodeVar10.xyz * vec3<f32>( object.nodeUniform32 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst33 = ( object.nodeUniform29 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst34 = clamp( 1.0, 0.0, 1.0 );
	nodeVar11 = textureSampleLevel( nodeUniform28, nodeUniform28_sampler, vec3<f32>( ( - nodeConst33.x ), nodeConst33.yz ), ( ( object.nodeUniform31 * nodeConst34 ) * ( 2.0 - nodeConst34 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar11.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform32 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar12 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar12 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst35 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst35 );
	let nodeConst36 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst37 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst35 * nodeConst36 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst37 ) * nodeConst36 ) ) ) * vec3<f32>( nodeConst37 ) ) );
	let nodeConst38 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar13 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst38 ) );
	nodeVar14 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst38 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar13 );
	indirectDiffuse = ( indirectDiffuse + nodeVar14 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	let nodeConst39 = ( render.nodeUniform33 + ( ( ( fn16( ( v_positionWorld * vec3<f32>( 0.005 ) ), 0.2, render.nodeUniform34 ) + fn16( ( v_positionWorld * vec3<f32>( 0.01 ) ), 0.2, ( render.nodeUniform34 * 1.2 ) ) ) - 0.7 ) * 22.0 ) );
	let nodeConst40 = ( - v_positionView.z );
	nodeVar15 = vec4<f32>( mix( Output.xyz, vec3<f32>( 0.6307571363387763, 0.7304607400847158, 0.7991027380100881 ), ( 1.0 - ( ( 1.0 - ( clamp( ( ( nodeConst39 - v_positionWorld.y ) / ( nodeConst39 - render.nodeUniform35 ) ), 0.0, 1.0 ) * 0.98 ) ) * ( 1.0 - ( 1.0 - exp( ( - ( ( ( render.nodeUniform36 * render.nodeUniform36 ) * nodeConst40 ) * nodeConst40 ) ) ) ) ) ) ) ), Output.w );
	let nodeConst41 = nodeVar15;
	Output = nodeConst41;

	// result

	output.color = nodeConst41;

	return output;

}
