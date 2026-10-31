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
@binding( 1 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform18_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform18 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform22 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform29 : f32,
	nodeUniform27 : f32,
	nodeUniform30 : f32,
	nodeUniform28 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform21 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar0 : f32;
var<private> NORMAL_normalWorld : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> workingLuminanceCoefficients : vec3<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar2 : f32;
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


fn fn67 ( x : i32, y : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;

	nodeVar0 = y;
	nodeVar1 = x;
	nodeVar2 = 2u;
	nodeVar3 = 0u;
	nodeVar4 = 0u;
	nodeVar5 = 0u;
	nodeVar5 = ( ( 3735928559u + ( nodeVar2 << 2u ) ) + 13u );
	nodeVar4 = nodeVar5;
	nodeVar3 = nodeVar4;
	nodeVar3 = ( nodeVar3 + u32( nodeVar1 ) );
	nodeVar4 = ( nodeVar4 + u32( nodeVar0 ) );

	return fn63( nodeVar3, nodeVar4, nodeVar5 );

}


fn fn53 ( hash : u32, x : f32, y : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;

	nodeVar0 = y;
	nodeVar1 = x;
	nodeVar2 = hash;
	nodeVar3 = ( nodeVar2 & 7u );
	nodeVar4 = fn46( ( nodeVar3 < 4u ), nodeVar1, nodeVar0 );
	nodeVar5 = ( 2.0 * fn46( ( nodeVar3 < 4u ), nodeVar0, nodeVar1 ) );

	return ( fn47( nodeVar4, bool( ( nodeVar3 & 1u ) ) ) + fn47( nodeVar5, bool( ( nodeVar3 & 2u ) ) ) );

}


fn fn49 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, s : f32, t : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;

	nodeVar0 = t;
	nodeVar1 = s;
	nodeVar2 = v3;
	nodeVar3 = v2;
	nodeVar4 = v1;
	nodeVar5 = v0;
	nodeVar6 = ( 1.0 - nodeVar1 );

	return ( ( ( 1.0 - nodeVar0 ) * ( ( nodeVar5 * nodeVar6 ) + ( nodeVar4 * nodeVar1 ) ) ) + ( nodeVar0 * ( ( nodeVar3 * nodeVar6 ) + ( nodeVar2 * nodeVar1 ) ) ) );

}


fn fn57 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.6616 * nodeVar0 );

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


fn fn14 ( x : f32 ) -> f32 {

	


	return abs( ( fract( x ) - 0.5 ) );

}


fn fn15 ( p : vec3<f32> ) -> vec3<f32> {

	


	return vec3<f32>( fn14( ( p.z + fn14( ( p.y * 1.0 ) ) ) ), fn14( ( p.z + fn14( ( p.x * 1.0 ) ) ) ), fn14( ( p.y + fn14( ( p.x * 1.0 ) ) ) ) );

}


fn fn73 ( p : vec2<f32> ) -> f32 {

	var nodeVar0 : vec2<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;

	nodeVar0 = p;
	nodeVar1 = 0;
	nodeVar2 = 0;
	nodeVar3 = nodeVar0.x;
	nodeVar1 = fn48( nodeVar3 );
	nodeVar4 = ( nodeVar3 - f32( nodeVar1 ) );
	nodeVar5 = nodeVar0.y;
	nodeVar2 = fn48( nodeVar5 );
	nodeVar6 = ( nodeVar5 - f32( nodeVar2 ) );
	nodeVar7 = fn65( nodeVar4 );
	nodeVar8 = fn65( nodeVar6 );
	nodeVar9 = fn49( fn53( fn67( nodeVar1, nodeVar2 ), nodeVar4, nodeVar6 ), fn53( fn67( ( nodeVar1 + 1 ), nodeVar2 ), ( nodeVar4 - 1.0 ), nodeVar6 ), fn53( fn67( nodeVar1, ( nodeVar2 + 1 ) ), nodeVar4, ( nodeVar6 - 1.0 ) ), fn53( fn67( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ) ), ( nodeVar4 - 1.0 ), ( nodeVar6 - 1.0 ) ), nodeVar7, nodeVar8 );

	return fn57( nodeVar9 );

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


fn fn32 ( color : vec3<f32>, adjustment : f32, luminanceCoefficients : vec3<f32> ) -> vec3<f32> {

	


	return max( mix( vec3<f32>( dot( color, luminanceCoefficients ) ), color, adjustment ), vec3<f32>( 0.0 ) );

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
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = ( ( fn73( ( v_positionWorld.xz * vec2<f32>( 0.012 ) ) ) * 1.0 ) + 0.0 );
	let nodeConst1 = clamp( ( ( v_positionWorld.y - object.nodeUniform1 ) / ( object.nodeUniform2 - object.nodeUniform1 ) ), 0.0, 1.0 );
	let nodeConst2 = mix( vec3<f32>( 0.15592646369776456, 0.16826940017946088, 0.08650046202808521 ), vec3<f32>( 0.2541520943200296, 0.23455058215026167, 0.08021982030622662 ), ( smoothstep( 0.15, 0.75, nodeConst0 ) * smoothstep( 0.22, 0.5, nodeConst1 ) ) );
	let nodeConst3 = smoothstep( 0.16, 0.34, nodeConst1 );
	let nodeConst4 = dpdx( v_positionView );
	let nodeConst5 = - dpdy( v_positionView );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	let NORMAL_nodeConst6 = cross( nodeConst5, NORMAL_normalView );
	let NORMAL_nodeConst7 = dot( nodeConst4, NORMAL_nodeConst6 );
	nodeVar0 = 0.0;
	let nodeConst8 = distance( v_positionWorld, render.cameraPosition );
	let nodeConst9 = smoothstep( 420.0, 60.0, nodeConst8 );

	if ( ( nodeConst9 > 0.01 ) ) {

		nodeVar0 = ( ( fn74( ( v_positionWorld * vec3<f32>( 0.6 ) ) ) * 1.0 ) + 0.0 );
		let nodeConst10 = smoothstep( 180.0, 40.0, nodeConst8 );

		if ( ( nodeConst10 > 0.01 ) ) {

			nodeVar0 = ( nodeVar0 + ( ( ( ( fn74( ( v_positionWorld * vec3<f32>( 1.7 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) * nodeConst10 ) );
			

		}

		

	}

	NORMAL_normalWorld = normalize( ( vec4<f32>( NORMAL_normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let NORMAL_nodeConst11 = clamp( NORMAL_normalWorld.y, 0.0, 1.0 );
	let nodeConst12 = ( 1.0 - NORMAL_nodeConst11 );
	let nodeConst13 = ( ( fn73( ( v_positionWorld.xz * vec2<f32>( 0.05 ) ) ) * 1.0 ) + 0.0 );
	let nodeConst14 = ( ( fn73( ( v_positionWorld.xz * vec2<f32>( 0.18 ) ) ) * 1.0 ) + 0.0 );
	let nodeConst15 = ( smoothstep( 0.56, 0.78, ( ( nodeConst1 + ( nodeConst13 * 0.08 ) ) + ( nodeConst14 * 0.05 ) ) ) * smoothstep( 0.45, 0.75, NORMAL_nodeConst11 ) );
	let nodeConst16 = ( ( vec3<f32>( sign( NORMAL_nodeConst7 ) ) * ( ( vec3<f32>( dpdx( nodeVar0 ) ) * NORMAL_nodeConst6 ) + ( vec3<f32>( - dpdy( nodeVar0 ) ) * cross( NORMAL_normalView, nodeConst4 ) ) ) ) * vec3<f32>( ( ( mix( mix( 0.25, 0.55, nodeConst12 ), 0.04, nodeConst15 ) * nodeConst9 ) * 0.25 ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeConst7 ) ) * NORMAL_normalView ) - nodeConst16 ) );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst17 = ( ( mix( vec3<f32>( 0.16513219449147767, 0.15592646369776456, 0.13843161502267545 ), vec3<f32>( 0.1499597898006365, 0.171441100722554, 0.09084171117479915 ), ( ( ( smoothstep( 0.45, 0.72, nodeConst14 ) * smoothstep( 0.62, 0.32, nodeConst12 ) ) * smoothstep( 0.66, 0.34, nodeConst1 ) ) * 0.45 ) ) * vec3<f32>( ( ( ( ( ( ( sin( ( ( ( ( ( v_positionWorld.y * 0.5 ) + ( v_positionWorld.x * 0.08 ) ) + ( v_positionWorld.z * 0.05 ) ) + ( nodeConst13 * 3.0 ) ) + ( nodeConst0 * 4.0 ) ) ) * 0.6 ) + ( sin( ( ( v_positionWorld.y * 1.4 ) + ( nodeConst14 * 2.0 ) ) ) * 0.4 ) ) * 0.5 ) + 0.5 ) * 0.36 ) + 0.8 ) ) ) * vec3<f32>( ( ( nodeConst14 * 0.18 ) + 1.0 ) ) );
	let nodeConst18 = smoothstep( 0.46, 0.64, ( nodeConst1 + ( nodeConst13 * 0.06 ) ) );
	let nodeConst19 = smoothstep( 0.34, 0.62, nodeConst12 );
	let nodeConst20 = ( ( ( smoothstep( 0.42, 0.7, nodeConst12 ) * smoothstep( 0.35, 0.7, clamp( normalWorld.y, 0.0, 1.0 ) ) ) * ( ( nodeConst13 * 0.5 ) + 0.5 ) ) * 0.5 );
	let nodeConst21 = mix( vec3<f32>( 0.8148465722120952, 0.8387990117372213, 0.8713671191959567 ), vec3<f32>( 0.6038273388475408, 0.6724431569510133, 0.76052450467022 ), ( smoothstep( 0.2, 0.7, nodeConst14 ) * 0.6 ) );
	let nodeConst22 = ( 1.0 - ( ( smoothstep( 0.24, 0.06, nodeConst1 ) * clamp( normalWorld.y, 0.0, 1.0 ) ) * 0.32 ) );
	let nodeConst23 = ( ( ( ( nodeConst0 * 0.5 ) + 0.5 ) * 0.3 ) + 0.84 );
	let nodeConst24 = ( ( ( ( nodeConst14 * 0.5 ) + 0.5 ) * 0.12 ) + 0.94 );
	let nodeConst25 = smoothstep( 180.0, 820.0, nodeConst8 );
	let nodeConst26 = ( ( ( 1.0 - nodeConst25 ) * 0.5 ) + 0.5 );
	workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
	DiffuseColor = vec4<f32>( mix( fn32( ( ( ( mix( mix( mix( mix( mix( nodeConst2, vec3<f32>( 0.040915196900556984, 0.05126945836711539, 0.028426039499072558 ), ( ( nodeConst3 * smoothstep( 0.5, 0.72, clamp( normalWorld.y, 0.0, 1.0 ) ) ) * 0.75 ) ), nodeConst17, nodeConst18 ), nodeConst17, nodeConst19 ), vec3<f32>( 0.22696587349938613, 0.19461783043107173, 0.158960835050774 ), nodeConst20 ), nodeConst21, nodeConst15 ) * vec3<f32>( nodeConst22 ) ) * vec3<f32>( nodeConst23 ) ) * vec3<f32>( nodeConst24 ) ), nodeConst26, workingLuminanceCoefficients ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4910208498384856 ), ( nodeConst25 * 0.62 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform8;
	let nodeConst27 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( mix( 0.95, 0.72, nodeConst15 ), 0.045 ) + max( max( nodeConst27.x, nodeConst27.y ), nodeConst27.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform8 ) ) );
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst28 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst28 );
	let nodeConst29 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst30 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst28 * nodeConst29 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst30 ) * nodeConst29 ) ) ) * vec3<f32>( nodeConst30 ) ) );
	let nodeConst31 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform12 - render.nodeUniform13 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	let nodeConst32 = ( render.nodeUniform15 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform16 ) ) ), 1.0 ) );
	let nodeConst33 = ( nodeConst32.xyz / vec3<f32>( nodeConst32.w ) );
	let nodeConst34 = vec3<f32>( nodeConst33.x, ( 1.0 - nodeConst33.y ), ( nodeConst33.z + render.nodeUniform17 ) );

	if ( ( ( ( ( ( nodeConst34.x >= 0.0 ) && ( nodeConst34.x <= 1.0 ) ) && ( nodeConst34.y >= 0.0 ) ) && ( nodeConst34.y <= 1.0 ) ) && ( nodeConst34.z <= 1.0 ) ) ) {

		let nodeConst35 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst36 = ( vec2<f32>( render.nodeUniform19 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform20 ) );
		let nodeConst37 = ( nodeConst34.xy + ( fn25( 0, 5, nodeConst35 ) * nodeConst36 ) );
		nodeVar3 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst37, nodeConst34.z );
		let nodeConst38 = ( nodeConst34.xy + ( fn25( 1, 5, nodeConst35 ) * nodeConst36 ) );
		nodeVar4 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst38, nodeConst34.z );
		let nodeConst39 = ( nodeConst34.xy + ( fn25( 2, 5, nodeConst35 ) * nodeConst36 ) );
		nodeVar5 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst39, nodeConst34.z );
		let nodeConst40 = ( nodeConst34.xy + ( fn25( 3, 5, nodeConst35 ) * nodeConst36 ) );
		nodeVar6 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst40, nodeConst34.z );
		let nodeConst41 = ( nodeConst34.xy + ( fn25( 4, 5, nodeConst35 ) * nodeConst36 ) );
		nodeVar7 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst41, nodeConst34.z );
		nodeVar2 = ( ( ( ( ( nodeVar3 + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) * 0.2 );

	} else {

		nodeVar2 = 1.0;

	}

	nodeVar8 = mix( 1.0, nodeVar2, render.nodeUniform21 );
	nodeVar9 = ( vec3<f32>( clamp( dot( normalView, nodeConst31 ), 0.0, 1.0 ) ) * ( render.nodeUniform14 * vec3<f32>( nodeVar8 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst42 = clamp( dot( positionViewDirection, normalize( ( nodeConst31 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst43 = exp2( ( ( ( nodeConst42 * -5.55473 ) - 6.98316 ) * nodeConst42 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar9 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst43 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst43 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst44 = normalize( ( nodeConst31 + positionViewDirection ) );
	let nodeConst45 = clamp( dot( positionViewDirection, nodeConst44 ), 0.0, 1.0 );
	let nodeConst46 = exp2( ( ( ( nodeConst45 * -5.55473 ) - 6.98316 ) * nodeConst45 ) );
	let nodeConst47 = max( Roughness, 0.045 );
	let nodeConst48 = ( nodeConst47 * nodeConst47 );
	directSpecular = ( directSpecular + ( ( nodeVar9 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst46 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst46 ) ) ) * vec3<f32>( fn110( nodeConst48, clamp( dot( normalView, nodeConst31 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst48, clamp( dot( normalView, nodeConst44 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst49 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst50 = ( object.nodeUniform23 * vec4<f32>( nodeConst49, 1.0 ) );
	let nodeConst51 = clamp( Roughness, 0.0, 1.0 );
	nodeVar10 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, vec3<f32>( ( - nodeConst50.x ), nodeConst50.yz ), ( ( object.nodeUniform25 * nodeConst51 ) * ( 2.0 - nodeConst51 ) ) );
	radiance = ( radiance + ( nodeVar10.xyz * vec3<f32>( object.nodeUniform26 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst52 = ( object.nodeUniform23 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst53 = clamp( 1.0, 0.0, 1.0 );
	nodeVar11 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, vec3<f32>( ( - nodeConst52.x ), nodeConst52.yz ), ( ( object.nodeUniform25 * nodeConst53 ) * ( 2.0 - nodeConst53 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar11.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform26 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar12 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar12 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst54 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst54 );
	let nodeConst55 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst56 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst54 * nodeConst55 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst56 ) * nodeConst55 ) ) ) * vec3<f32>( nodeConst56 ) ) );
	let nodeConst57 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar13 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst57 ) );
	nodeVar14 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst57 );
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
	let nodeConst58 = ( render.nodeUniform27 + ( ( ( fn16( ( v_positionWorld * vec3<f32>( 0.005 ) ), 0.2, render.nodeUniform28 ) + fn16( ( v_positionWorld * vec3<f32>( 0.01 ) ), 0.2, ( render.nodeUniform28 * 1.2 ) ) ) - 0.7 ) * 22.0 ) );
	let nodeConst59 = ( - v_positionView.z );
	nodeVar15 = vec4<f32>( mix( Output.xyz, vec3<f32>( 0.6307571363387763, 0.7304607400847158, 0.7991027380100881 ), ( 1.0 - ( ( 1.0 - ( clamp( ( ( nodeConst58 - v_positionWorld.y ) / ( nodeConst58 - render.nodeUniform29 ) ), 0.0, 1.0 ) * 0.98 ) ) * ( 1.0 - ( 1.0 - exp( ( - ( ( ( render.nodeUniform30 * render.nodeUniform30 ) * nodeConst59 ) * nodeConst59 ) ) ) ) ) ) ) ), Output.w );
	let nodeConst60 = nodeVar15;
	Output = nodeConst60;

	// result

	output.color = nodeConst60;

	return output;

}
