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
@binding( 1 ) @group( 1 ) var nodeUniform17_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform17 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform37_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform37 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform42_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform42 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform16 : f32,
	nodeUniform18 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform7 : f32,
	nodeUniform13 : f32,
	nodeUniform12 : f32,
	nodeUniform21 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform11 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform27 : f32,
	nodeUniform26 : f32,
	nodeUniform23 : f32,
	nodeUniform22 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform20 : f32,
	nodeUniform10 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32,
	nodeUniform46 : f32,
	nodeUniform47 : f32,
	nodeUniform50 : f32,
	nodeUniform51 : f32,
	nodeUniform36 : vec3<f32>,
	nodeUniform48 : vec3<f32>,
	nodeUniform49 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform35 : vec3<f32>,
	nodeUniform38 : mat4x4<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform43 : f32,
	nodeUniform44 : vec2<f32>,
	nodeUniform45 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> workingLuminanceCoefficients : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
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


fn fn32 ( color : vec3<f32>, adjustment : f32, luminanceCoefficients : vec3<f32> ) -> vec3<f32> {

	


	return max( mix( vec3<f32>( dot( color, luminanceCoefficients ) ), color, adjustment ), vec3<f32>( 0.0 ) );

}


fn fn34 ( color : vec3<f32>, adjustment : f32 ) -> vec3<f32> {

	

	let nodeConst0 = cos( adjustment );

	return max( ( ( color * vec3<f32>( nodeConst0 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * vec3<f32>( sin( adjustment ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * ( 1.0 - nodeConst0 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

}


fn fn44 ( lightDistance : f32, cutoffDistance : f32, decayExponent : f32 ) -> f32 {

	var nodeVar0 : f32;


	if ( ( cutoffDistance > 0.0 ) ) {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		let nodeConst1 = ( lightDistance / cutoffDistance );
		let nodeConst2 = clamp( ( 1.0 - ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * nodeConst1 ) ), 0.0, 1.0 );
		nodeVar0 = ( nodeConst0 * ( nodeConst2 * nodeConst2 ) );

	} else {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		nodeVar0 = nodeConst0;

	}


	return nodeVar0;

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


fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	nodeVar0 = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( positionLocal * vec3<f32>( 0.5 ) );
	let nodeConst2 = vec3<f32>( 0.0, ( - render.nodeUniform7 ), 0.0 );
	let nodeConst3 = ( ( ( ( fn74( ( nodeConst1 + nodeConst2 ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 );
	let nodeConst4 = ( clamp( pow( ( ( ( nodeConst3 * 0.5 ) + ( ( ( ( ( fn74( ( ( ( nodeConst1 * vec3<f32>( 2.0 ) ) - ( nodeConst2 * vec3<f32>( 1.5 ) ) ) + vec3<f32>( ( nodeConst3 * 0.4 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.35 ) ) + ( ( ( ( ( fn74( ( ( nodeConst1 * vec3<f32>( 4.0 ) ) + ( nodeConst2 * vec3<f32>( 2.5 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.15 ) ), 2.5 ), 0.0, 1.0 ) + 0.1 );
	nodeVar0 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform6, smoothstep( 0.05, 0.35, nodeConst4 ) );
	nodeVar0 = mix( nodeVar0, render.nodeUniform8, smoothstep( 0.35, 0.65, nodeConst4 ) );
	nodeVar0 = mix( nodeVar0, render.nodeUniform9, smoothstep( 0.65, 1.0, nodeConst4 ) );
	workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
	EmissiveColor = ( ( ( ( ( ( fn34( fn32( nodeVar0, render.nodeUniform10, workingLuminanceCoefficients ), render.nodeUniform11 ) * vec3<f32>( pow( max( ( render.nodeUniform12 / 8.34 ), 0.0 ), 4.0 ) ) ) * vec3<f32>( max( ( render.nodeUniform13 / 11.02 ), 0.0 ) ) ) * vec3<f32>( render.nodeUniform14 ) ) * vec3<f32>( render.nodeUniform15 ) ) * vec3<f32>( smoothstep( 0.0, 3.0, render.nodeUniform7 ) ) ) * vec3<f32>( object.nodeUniform16 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform17, nodeUniform17_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst5 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst5 );
	let nodeConst6 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst7 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst5 * nodeConst6 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst7 ) * nodeConst6 ) ) ) * vec3<f32>( nodeConst7 ) ) );
	let nodeConst8 = ( render.nodeUniform19 - v_positionView );
	let nodeConst9 = normalize( nodeConst8 );

	if ( ( 0.0 == 1.0 ) ) {

		nodeVar3 = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst10 = clamp( ( ( ( ( render.nodeUniform12 / 8.34 ) * 0.5 ) + 0.2 ) + render.nodeUniform20 ), 0.0, 1.0 );
		nodeVar3 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform6, smoothstep( 0.05, 0.35, nodeConst10 ) );
		nodeVar3 = mix( nodeVar3, render.nodeUniform8, smoothstep( 0.35, 0.65, nodeConst10 ) );
		nodeVar3 = mix( nodeVar3, render.nodeUniform9, smoothstep( 0.65, 1.0, nodeConst10 ) );
		nodeVar2 = fn34( fn32( nodeVar3, render.nodeUniform10, workingLuminanceCoefficients ), render.nodeUniform11 );

	} else {

		nodeVar4 = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst11 = vec3<f32>( 0.0, render.nodeUniform23, 0.0 );
		let nodeConst12 = length( ( v_positionWorld - ( ( render.nodeUniform21 + render.nodeUniform22 ) + ( nodeConst11 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform21 ), nodeConst11 ) / dot( nodeConst11, nodeConst11 ) ), 0.0, 1.0 ) ) ) ) ) );
		let nodeConst13 = ( v_positionWorld.xz - render.nodeUniform21.xz );
		let nodeConst14 = atan2( nodeConst13.y, nodeConst13.x );
		let nodeConst15 = clamp( ( clamp( ( 1.0 - ( nodeConst12 / render.nodeUniform24 ) ), 0.0, 1.0 ) * mix( ( ( ( ( ( ( ( fn74( vec3<f32>( ( v_positionWorld.x * ( 0.6 * render.nodeUniform25 ) ), ( render.nodeUniform7 * 1.2 ), ( v_positionWorld.z * ( 0.6 * render.nodeUniform25 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.65 ) + ( ( ( ( ( fn74( vec3<f32>( ( v_positionWorld.x * ( 1.5 * render.nodeUniform25 ) ), ( render.nodeUniform7 * 2.5 ), ( v_positionWorld.z * ( 1.5 * render.nodeUniform25 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.35 ) ) * ( ( mix( 1.0, ( ( ( ( fn74( vec3<f32>( ( cos( nodeConst14 ) * ( 1.5 * render.nodeUniform25 ) ), ( sin( nodeConst14 ) * ( 1.5 * render.nodeUniform25 ) ), ( render.nodeUniform7 * 0.6 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ), smoothstep( 0.0, render.nodeUniform26, length( nodeConst13 ) ) ) * 0.5 ) + 0.5 ) ), 1.0, clamp( ( nodeConst12 / render.nodeUniform27 ), 0.0, 1.0 ) ) ), 0.0, 1.0 );
		nodeVar4 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform6, smoothstep( 0.05, 0.35, nodeConst15 ) );
		nodeVar4 = mix( nodeVar4, render.nodeUniform8, smoothstep( 0.35, 0.65, nodeConst15 ) );
		nodeVar4 = mix( nodeVar4, render.nodeUniform9, smoothstep( 0.65, 1.0, nodeConst15 ) );
		nodeVar2 = fn34( fn32( nodeVar4, render.nodeUniform10, workingLuminanceCoefficients ), render.nodeUniform11 );

	}


	if ( ( 0.0 == 1.0 ) ) {

		nodeVar5 = render.nodeUniform28;

	} else {

		nodeVar5 = render.nodeUniform29;

	}


	if ( ( 0.0 == 1.0 ) ) {

		let nodeConst11 = vec3<f32>( 0.0, render.nodeUniform23, 0.0 );
		let nodeConst12 = length( ( v_positionWorld - ( ( render.nodeUniform21 + render.nodeUniform22 ) + ( nodeConst11 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform21 ), nodeConst11 ) / dot( nodeConst11, nodeConst11 ) ), 0.0, 1.0 ) ) ) ) ) );
		nodeVar6 = ( ( 1.0 / ( pow( nodeConst12, 2.0 ) + pow( 1.2, 2.0 ) ) ) / ( 1.0 / max( pow( length( ( v_positionWorld - render.nodeUniform21 ) ), 2.0 ), 0.01 ) ) );

	} else {

		nodeVar6 = 1.0;

	}


	if ( ( 0.0 == 1.0 ) ) {

		let nodeConst11 = vec3<f32>( 0.0, render.nodeUniform23, 0.0 );
		let nodeConst12 = length( ( v_positionWorld - ( ( render.nodeUniform21 + render.nodeUniform22 ) + ( nodeConst11 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform21 ), nodeConst11 ) / dot( nodeConst11, nodeConst11 ) ), 0.0, 1.0 ) ) ) ) ) );
		nodeVar7 = mix( render.nodeUniform30, render.nodeUniform31, smoothstep( 0.0, 1.0, clamp( ( nodeConst12 / render.nodeUniform32 ), 0.0, 1.0 ) ) );

	} else {

		nodeVar7 = 1.0;

	}

	nodeVar8 = ( vec3<f32>( clamp( dot( normalView, nodeConst9 ), 0.0, 1.0 ) ) * ( ( ( ( ( ( ( ( ( nodeVar2 * vec3<f32>( pow( max( ( render.nodeUniform12 / 8.34 ), 0.0 ), 4.0 ) ) ) * vec3<f32>( max( ( render.nodeUniform13 / 11.02 ), 0.0 ) ) ) * vec3<f32>( render.nodeUniform14 ) ) * vec3<f32>( nodeVar5 ) ) * vec3<f32>( render.nodeUniform15 ) ) * vec3<f32>( smoothstep( 0.0, 3.0, render.nodeUniform7 ) ) ) * vec3<f32>( nodeVar6 ) ) * vec3<f32>( nodeVar7 ) ) * vec3<f32>( fn44( length( nodeConst8 ), render.nodeUniform33, render.nodeUniform34 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst16 = clamp( dot( positionViewDirection, normalize( ( nodeConst9 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst17 = exp2( ( ( ( nodeConst16 * -5.55473 ) - 6.98316 ) * nodeConst16 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar8 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst17 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst17 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst18 = normalize( ( nodeConst9 + positionViewDirection ) );
	let nodeConst19 = clamp( dot( positionViewDirection, nodeConst18 ), 0.0, 1.0 );
	let nodeConst20 = exp2( ( ( ( nodeConst19 * -5.55473 ) - 6.98316 ) * nodeConst19 ) );
	let nodeConst21 = max( Roughness, 0.045 );
	let nodeConst22 = ( nodeConst21 * nodeConst21 );
	directSpecular = ( directSpecular + ( ( nodeVar8 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst20 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst20 ) ) ) * vec3<f32>( fn110( nodeConst22, clamp( dot( normalView, nodeConst9 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst22, clamp( dot( normalView, nodeConst18 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst23 = ( render.nodeUniform35 - v_positionView );
	let nodeConst24 = normalize( nodeConst23 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst25 = ( render.nodeUniform38 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform40 ) ) ), 1.0 ) );
	let nodeConst26 = ( nodeConst25.xyz / vec3<f32>( nodeConst25.w ) );
	let nodeConst27 = vec3<f32>( nodeConst26.x, ( 1.0 - nodeConst26.y ), ( nodeConst26.z + render.nodeUniform41 ) );
	nodeVar9 = textureSample( nodeUniform37, nodeUniform37_sampler, nodeConst27.xy );

	if ( ( ( ( ( ( nodeConst27.x >= 0.0 ) && ( nodeConst27.x <= 1.0 ) ) && ( nodeConst27.y >= 0.0 ) ) && ( nodeConst27.y <= 1.0 ) ) && ( nodeConst27.z <= 1.0 ) ) ) {

		let nodeConst28 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst29 = ( vec2<f32>( render.nodeUniform43 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform44 ) );
		let nodeConst30 = ( nodeConst27.xy + ( fn25( 0, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar11 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeConst30, nodeConst27.z );
		let nodeConst31 = ( nodeConst27.xy + ( fn25( 1, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar12 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeConst31, nodeConst27.z );
		let nodeConst32 = ( nodeConst27.xy + ( fn25( 2, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar13 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeConst32, nodeConst27.z );
		let nodeConst33 = ( nodeConst27.xy + ( fn25( 3, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar14 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeConst33, nodeConst27.z );
		let nodeConst34 = ( nodeConst27.xy + ( fn25( 4, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar15 = textureSampleCompare( nodeUniform42, nodeUniform42_sampler, nodeConst34, nodeConst27.z );
		nodeVar10 = ( ( ( ( ( nodeVar11 + nodeVar12 ) + nodeVar13 ) + nodeVar14 ) + nodeVar15 ) * 0.2 );

	} else {

		nodeVar10 = 1.0;

	}

	nodeVar16 = mix( vec4<f32>( 1.0 ), mix( nodeVar9, vec4<f32>( 1.0 ), vec4<f32>( nodeVar10 ) ), ( render.nodeUniform45 * nodeVar9.w ) );
	nodeVar17 = ( vec4<f32>( clamp( dot( normalView, nodeConst24 ), 0.0, 1.0 ) ) * ( ( ( vec4<f32>( render.nodeUniform36, 1.0 ) * nodeVar16 ) * vec4<f32>( smoothstep( render.nodeUniform46, render.nodeUniform47, dot( nodeConst24, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform48 - render.nodeUniform49 ), 0.0 ) ).xyz ) ) ) ) ) * vec4<f32>( fn44( length( nodeConst23 ), render.nodeUniform50, render.nodeUniform51 ) ) ) );
	let nodeConst35 = clamp( dot( positionViewDirection, normalize( ( nodeConst24 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst36 = exp2( ( ( ( nodeConst35 * -5.55473 ) - 6.98316 ) * nodeConst35 ) );
	directDiffuse = ( vec4<f32>( directDiffuse, 1.0 ) + ( ( nodeVar17 * vec4<f32>( ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ), 1.0 ) ) * vec4<f32>( ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst36 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst36 ) ) ) ), 1.0 ) ) ).xyz;
	let nodeConst37 = normalize( ( nodeConst24 + positionViewDirection ) );
	let nodeConst38 = clamp( dot( positionViewDirection, nodeConst37 ), 0.0, 1.0 );
	let nodeConst39 = exp2( ( ( ( nodeConst38 * -5.55473 ) - 6.98316 ) * nodeConst38 ) );
	let nodeConst40 = max( Roughness, 0.045 );
	let nodeConst41 = ( nodeConst40 * nodeConst40 );
	directSpecular = ( vec4<f32>( directSpecular, 1.0 ) + ( ( nodeVar17 * vec4<f32>( ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst39 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst39 ) ) ) * vec3<f32>( fn110( nodeConst41, clamp( dot( normalView, nodeConst24 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst41, clamp( dot( normalView, nodeConst37 ), 0.0, 1.0 ) ) ) ), 1.0 ) ) * vec4<f32>( multiScatteringCompensation, 1.0 ) ) ).xyz;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar18 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar18 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst42 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst42 );
	let nodeConst43 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst44 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst42 * nodeConst43 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst44 ) * nodeConst43 ) ) ) * vec3<f32>( nodeConst44 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst45 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar19 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst45 ) );
	nodeVar20 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst45 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar19 );
	indirectDiffuse = ( indirectDiffuse + nodeVar20 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst46 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst46;

	// result

	output.color = nodeConst46;

	return output;

}
