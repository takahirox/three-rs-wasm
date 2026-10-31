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
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform38_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform38 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform43_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform43 : texture_depth_2d;

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
	nodeUniform23 : f32,
	nodeUniform26 : f32,
	nodeUniform12 : f32,
	nodeUniform18 : vec3<f32>,
	nodeUniform27 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform17 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform33 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform25 : f32,
	nodeUniform24 : f32,
	nodeUniform20 : f32,
	nodeUniform19 : vec3<f32>,
	nodeUniform30 : f32,
	nodeUniform13 : f32,
	nodeUniform16 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform47 : f32,
	nodeUniform48 : f32,
	nodeUniform51 : f32,
	nodeUniform52 : f32,
	nodeUniform37 : vec3<f32>,
	nodeUniform49 : vec3<f32>,
	nodeUniform50 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform36 : vec3<f32>,
	nodeUniform39 : mat4x4<f32>,
	nodeUniform41 : f32,
	nodeUniform42 : f32,
	nodeUniform44 : f32,
	nodeUniform45 : vec2<f32>,
	nodeUniform46 : f32
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
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar0 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> workingLuminanceCoefficients : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn32 ( color : vec3<f32>, adjustment : f32, luminanceCoefficients : vec3<f32> ) -> vec3<f32> {

	


	return max( mix( vec3<f32>( dot( color, luminanceCoefficients ) ), color, adjustment ), vec3<f32>( 0.0 ) );

}


fn fn34 ( color : vec3<f32>, adjustment : f32 ) -> vec3<f32> {

	

	let nodeConst0 = cos( adjustment );

	return max( ( ( color * vec3<f32>( nodeConst0 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * vec3<f32>( sin( adjustment ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * ( 1.0 - nodeConst0 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

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
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	let nodeConst4 = ( render.nodeUniform10 - v_positionView );
	let nodeConst5 = normalize( nodeConst4 );

	if ( ( 0.0 == 1.0 ) ) {

		nodeVar2 = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst6 = clamp( ( ( ( ( render.nodeUniform12 / 8.34 ) * 0.5 ) + 0.2 ) + render.nodeUniform13 ), 0.0, 1.0 );
		nodeVar2 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform11, smoothstep( 0.05, 0.35, nodeConst6 ) );
		nodeVar2 = mix( nodeVar2, render.nodeUniform14, smoothstep( 0.35, 0.65, nodeConst6 ) );
		nodeVar2 = mix( nodeVar2, render.nodeUniform15, smoothstep( 0.65, 1.0, nodeConst6 ) );
		workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
		nodeVar1 = fn34( fn32( nodeVar2, render.nodeUniform16, workingLuminanceCoefficients ), render.nodeUniform17 );

	} else {

		nodeVar3 = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst7 = vec3<f32>( 0.0, render.nodeUniform20, 0.0 );
		let nodeConst8 = length( ( v_positionWorld - ( ( render.nodeUniform18 + render.nodeUniform19 ) + ( nodeConst7 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform18 ), nodeConst7 ) / dot( nodeConst7, nodeConst7 ) ), 0.0, 1.0 ) ) ) ) ) );
		let nodeConst9 = ( v_positionWorld.xz - render.nodeUniform18.xz );
		let nodeConst10 = atan2( nodeConst9.y, nodeConst9.x );
		let nodeConst11 = clamp( ( clamp( ( 1.0 - ( nodeConst8 / render.nodeUniform21 ) ), 0.0, 1.0 ) * mix( ( ( ( ( ( ( ( fn74( vec3<f32>( ( v_positionWorld.x * ( 0.6 * render.nodeUniform22 ) ), ( render.nodeUniform23 * 1.2 ), ( v_positionWorld.z * ( 0.6 * render.nodeUniform22 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.65 ) + ( ( ( ( ( fn74( vec3<f32>( ( v_positionWorld.x * ( 1.5 * render.nodeUniform22 ) ), ( render.nodeUniform23 * 2.5 ), ( v_positionWorld.z * ( 1.5 * render.nodeUniform22 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.35 ) ) * ( ( mix( 1.0, ( ( ( ( fn74( vec3<f32>( ( cos( nodeConst10 ) * ( 1.5 * render.nodeUniform22 ) ), ( sin( nodeConst10 ) * ( 1.5 * render.nodeUniform22 ) ), ( render.nodeUniform23 * 0.6 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ), smoothstep( 0.0, render.nodeUniform24, length( nodeConst9 ) ) ) * 0.5 ) + 0.5 ) ), 1.0, clamp( ( nodeConst8 / render.nodeUniform25 ), 0.0, 1.0 ) ) ), 0.0, 1.0 );
		nodeVar3 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform11, smoothstep( 0.05, 0.35, nodeConst11 ) );
		nodeVar3 = mix( nodeVar3, render.nodeUniform14, smoothstep( 0.35, 0.65, nodeConst11 ) );
		nodeVar3 = mix( nodeVar3, render.nodeUniform15, smoothstep( 0.65, 1.0, nodeConst11 ) );
		workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
		nodeVar1 = fn34( fn32( nodeVar3, render.nodeUniform16, workingLuminanceCoefficients ), render.nodeUniform17 );

	}


	if ( ( 0.0 == 1.0 ) ) {

		nodeVar4 = render.nodeUniform28;

	} else {

		nodeVar4 = render.nodeUniform29;

	}


	if ( ( 0.0 == 1.0 ) ) {

		let nodeConst7 = vec3<f32>( 0.0, render.nodeUniform20, 0.0 );
		let nodeConst8 = length( ( v_positionWorld - ( ( render.nodeUniform18 + render.nodeUniform19 ) + ( nodeConst7 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform18 ), nodeConst7 ) / dot( nodeConst7, nodeConst7 ) ), 0.0, 1.0 ) ) ) ) ) );
		nodeVar5 = ( ( 1.0 / ( pow( nodeConst8, 2.0 ) + pow( 1.2, 2.0 ) ) ) / ( 1.0 / max( pow( length( ( v_positionWorld - render.nodeUniform18 ) ), 2.0 ), 0.01 ) ) );

	} else {

		nodeVar5 = 1.0;

	}


	if ( ( 0.0 == 1.0 ) ) {

		let nodeConst7 = vec3<f32>( 0.0, render.nodeUniform20, 0.0 );
		let nodeConst8 = length( ( v_positionWorld - ( ( render.nodeUniform18 + render.nodeUniform19 ) + ( nodeConst7 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform18 ), nodeConst7 ) / dot( nodeConst7, nodeConst7 ) ), 0.0, 1.0 ) ) ) ) ) );
		nodeVar6 = mix( render.nodeUniform31, render.nodeUniform32, smoothstep( 0.0, 1.0, clamp( ( nodeConst8 / render.nodeUniform33 ), 0.0, 1.0 ) ) );

	} else {

		nodeVar6 = 1.0;

	}

	nodeVar7 = ( vec3<f32>( clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ) ) * ( ( ( ( ( ( ( ( ( nodeVar1 * vec3<f32>( pow( max( ( render.nodeUniform12 / 8.34 ), 0.0 ), 4.0 ) ) ) * vec3<f32>( max( ( render.nodeUniform26 / 11.02 ), 0.0 ) ) ) * vec3<f32>( render.nodeUniform27 ) ) * vec3<f32>( nodeVar4 ) ) * vec3<f32>( render.nodeUniform30 ) ) * vec3<f32>( smoothstep( 0.0, 3.0, render.nodeUniform23 ) ) ) * vec3<f32>( nodeVar5 ) ) * vec3<f32>( nodeVar6 ) ) * vec3<f32>( fn44( length( nodeConst4 ), render.nodeUniform34, render.nodeUniform35 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst12 = clamp( dot( positionViewDirection, normalize( ( nodeConst5 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst13 = exp2( ( ( ( nodeConst12 * -5.55473 ) - 6.98316 ) * nodeConst12 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar7 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst13 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst13 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst14 = normalize( ( nodeConst5 + positionViewDirection ) );
	let nodeConst15 = clamp( dot( positionViewDirection, nodeConst14 ), 0.0, 1.0 );
	let nodeConst16 = exp2( ( ( ( nodeConst15 * -5.55473 ) - 6.98316 ) * nodeConst15 ) );
	let nodeConst17 = max( Roughness, 0.045 );
	let nodeConst18 = ( nodeConst17 * nodeConst17 );
	directSpecular = ( directSpecular + ( ( nodeVar7 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst16 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst16 ) ) ) * vec3<f32>( fn110( nodeConst18, clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst18, clamp( dot( normalView, nodeConst14 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst19 = ( render.nodeUniform36 - v_positionView );
	let nodeConst20 = normalize( nodeConst19 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst21 = ( render.nodeUniform39 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform41 ) ) ), 1.0 ) );
	let nodeConst22 = ( nodeConst21.xyz / vec3<f32>( nodeConst21.w ) );
	let nodeConst23 = vec3<f32>( nodeConst22.x, ( 1.0 - nodeConst22.y ), ( nodeConst22.z + render.nodeUniform42 ) );
	nodeVar8 = textureSample( nodeUniform38, nodeUniform38_sampler, nodeConst23.xy );

	if ( ( ( ( ( ( nodeConst23.x >= 0.0 ) && ( nodeConst23.x <= 1.0 ) ) && ( nodeConst23.y >= 0.0 ) ) && ( nodeConst23.y <= 1.0 ) ) && ( nodeConst23.z <= 1.0 ) ) ) {

		let nodeConst24 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst25 = ( vec2<f32>( render.nodeUniform44 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform45 ) );
		let nodeConst26 = ( nodeConst23.xy + ( fn25( 0, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar10 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeConst26, nodeConst23.z );
		let nodeConst27 = ( nodeConst23.xy + ( fn25( 1, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar11 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeConst27, nodeConst23.z );
		let nodeConst28 = ( nodeConst23.xy + ( fn25( 2, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar12 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeConst28, nodeConst23.z );
		let nodeConst29 = ( nodeConst23.xy + ( fn25( 3, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar13 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeConst29, nodeConst23.z );
		let nodeConst30 = ( nodeConst23.xy + ( fn25( 4, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar14 = textureSampleCompare( nodeUniform43, nodeUniform43_sampler, nodeConst30, nodeConst23.z );
		nodeVar9 = ( ( ( ( ( nodeVar10 + nodeVar11 ) + nodeVar12 ) + nodeVar13 ) + nodeVar14 ) * 0.2 );

	} else {

		nodeVar9 = 1.0;

	}

	nodeVar15 = mix( vec4<f32>( 1.0 ), mix( nodeVar8, vec4<f32>( 1.0 ), vec4<f32>( nodeVar9 ) ), ( render.nodeUniform46 * nodeVar8.w ) );
	nodeVar16 = ( vec4<f32>( clamp( dot( normalView, nodeConst20 ), 0.0, 1.0 ) ) * ( ( ( vec4<f32>( render.nodeUniform37, 1.0 ) * nodeVar15 ) * vec4<f32>( smoothstep( render.nodeUniform47, render.nodeUniform48, dot( nodeConst20, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform49 - render.nodeUniform50 ), 0.0 ) ).xyz ) ) ) ) ) * vec4<f32>( fn44( length( nodeConst19 ), render.nodeUniform51, render.nodeUniform52 ) ) ) );
	let nodeConst31 = clamp( dot( positionViewDirection, normalize( ( nodeConst20 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst32 = exp2( ( ( ( nodeConst31 * -5.55473 ) - 6.98316 ) * nodeConst31 ) );
	directDiffuse = ( vec4<f32>( directDiffuse, 1.0 ) + ( ( nodeVar16 * vec4<f32>( ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ), 1.0 ) ) * vec4<f32>( ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst32 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst32 ) ) ) ), 1.0 ) ) ).xyz;
	let nodeConst33 = normalize( ( nodeConst20 + positionViewDirection ) );
	let nodeConst34 = clamp( dot( positionViewDirection, nodeConst33 ), 0.0, 1.0 );
	let nodeConst35 = exp2( ( ( ( nodeConst34 * -5.55473 ) - 6.98316 ) * nodeConst34 ) );
	let nodeConst36 = max( Roughness, 0.045 );
	let nodeConst37 = ( nodeConst36 * nodeConst36 );
	directSpecular = ( vec4<f32>( directSpecular, 1.0 ) + ( ( nodeVar16 * vec4<f32>( ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst35 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst35 ) ) ) * vec3<f32>( fn110( nodeConst37, clamp( dot( normalView, nodeConst20 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst37, clamp( dot( normalView, nodeConst33 ), 0.0, 1.0 ) ) ) ), 1.0 ) ) * vec4<f32>( multiScatteringCompensation, 1.0 ) ) ).xyz;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar17 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar17 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst38 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst38 );
	let nodeConst39 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst40 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst38 * nodeConst39 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst40 ) * nodeConst39 ) ) ) * vec3<f32>( nodeConst40 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst41 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar18 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst41 ) );
	nodeVar19 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst41 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar18 );
	indirectDiffuse = ( indirectDiffuse + nodeVar19 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst42 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst42;

	// result

	output.color = nodeConst42;

	return output;

}
