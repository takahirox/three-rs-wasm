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
@binding( 1 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform20_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform20 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform28_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform28 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform30_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform30 : texture_cube<f32>;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : vec2<f32>,
	nodeUniform23 : f32,
	nodeUniform29 : f32,
	nodeUniform27 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform4 : mat4x4<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : f32,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform31 : mat4x4<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

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
var<private> nodeVar1 : f32;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst1 = fract( ( sin( ( ( floor( ( v_positionWorld.x / 1.5 ) ) * 127.1 ) + ( floor( ( v_positionWorld.z / 1.5 ) ) * 311.7 ) ) ) * 43758.5453 ) );
	nodeVar0 = 0.0;
	let nodeConst2 = smoothstep( 200.0, 18.0, distance( v_positionWorld, render.cameraPosition ) );

	if ( ( nodeConst2 > 0.0 ) ) {

		nodeVar0 = ( ( ( fn74( ( v_positionWorld * vec3<f32>( 14.0 ) ) ) * 1.0 ) + 0.0 ) * 0.07 );
		

	}

	let nodeConst3 = ( v_positionWorld.x / 1.5 );
	let nodeConst4 = max( fwidth( nodeConst3 ), 0.0001 );
	let nodeConst5 = ( v_positionWorld.z / 1.5 );
	let nodeConst6 = max( fwidth( nodeConst5 ), 0.0001 );
	let nodeConst7 = ( max( smoothstep( ( 0.03 + nodeConst4 ), ( 0.03 - nodeConst4 ), ( 0.5 - abs( ( fract( nodeConst3 ) - 0.5 ) ) ) ), smoothstep( ( 0.03 + nodeConst6 ), ( 0.03 - nodeConst6 ), ( 0.5 - abs( ( fract( nodeConst5 ) - 0.5 ) ) ) ) ) * nodeConst2 );
	DiffuseColor = vec4<f32>( ( ( ( mix( vec3<f32>( 0.158960835050774, 0.158960835050774, 0.13843161502267545 ), vec3<f32>( 0.26225065751888765, 0.26225065751888765, 0.22322795730611386 ), ( ( ( ( fn74( ( v_positionWorld * vec3<f32>( 0.5 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) ) * vec3<f32>( ( ( ( nodeConst1 - 0.5 ) * 0.16 ) + 1.0 ) ) ) + vec3<f32>( ( nodeVar0 * nodeConst2 ) ) ) * vec3<f32>( ( 1.0 - ( nodeConst7 * 0.45 ) ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform7;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst8 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( 0.92 - ( nodeConst1 * 0.05 ) ), 0.045 ) + max( max( nodeConst8.x, nodeConst8.y ), nodeConst8.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform7 ) ) );
	EmissiveColor = ( object.nodeUniform10 * vec3<f32>( object.nodeUniform11 ) );
	let nodeConst9 = dpdx( v_positionView );
	let nodeConst10 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	let NORMAL_nodeConst11 = cross( nodeConst10, NORMAL_normalView );
	let NORMAL_nodeConst12 = dot( nodeConst9, NORMAL_nodeConst11 );
	nodeVar1 = 0.0;

	if ( ( nodeConst2 > 0.0 ) ) {

		nodeVar1 = ( ( ( fn74( ( v_positionWorld * vec3<f32>( 3.0 ) ) ) * 1.0 ) + 0.0 ) * 0.003 );
		

	}

	let nodeConst13 = ( ( nodeVar1 - ( nodeConst7 * 0.012 ) ) * nodeConst2 );
	let nodeConst14 = ( vec3<f32>( sign( NORMAL_nodeConst12 ) ) * ( ( vec3<f32>( dpdx( nodeConst13 ) ) * NORMAL_nodeConst11 ) + ( vec3<f32>( - dpdy( nodeConst13 ) ) * cross( NORMAL_normalView, nodeConst9 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeConst12 ) ) * NORMAL_normalView ) - nodeConst14 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = textureSample( nodeUniform12, nodeUniform12_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst15 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst15 );
	let nodeConst16 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst17 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst15 * nodeConst16 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst17 ) * nodeConst16 ) ) ) * vec3<f32>( nodeConst17 ) ) );
	let nodeConst18 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform14 - render.nodeUniform15 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst19 = ( render.nodeUniform17 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform18 ) ) ), 1.0 ) );
	let nodeConst20 = ( nodeConst19.xyz / vec3<f32>( nodeConst19.w ) );
	let nodeConst21 = vec3<f32>( nodeConst20.x, ( 1.0 - nodeConst20.y ), ( nodeConst20.z + render.nodeUniform19 ) );

	if ( ( ( ( ( ( nodeConst21.x >= 0.0 ) && ( nodeConst21.x <= 1.0 ) ) && ( nodeConst21.y >= 0.0 ) ) && ( nodeConst21.y <= 1.0 ) ) && ( nodeConst21.z <= 1.0 ) ) ) {

		let nodeConst22 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst23 = ( vec2<f32>( render.nodeUniform21 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform22 ) );
		let nodeConst24 = ( nodeConst21.xy + ( fn25( 0, 5, nodeConst22 ) * nodeConst23 ) );
		nodeVar4 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst24, nodeConst21.z );
		let nodeConst25 = ( nodeConst21.xy + ( fn25( 1, 5, nodeConst22 ) * nodeConst23 ) );
		nodeVar5 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst25, nodeConst21.z );
		let nodeConst26 = ( nodeConst21.xy + ( fn25( 2, 5, nodeConst22 ) * nodeConst23 ) );
		nodeVar6 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst26, nodeConst21.z );
		let nodeConst27 = ( nodeConst21.xy + ( fn25( 3, 5, nodeConst22 ) * nodeConst23 ) );
		nodeVar7 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst27, nodeConst21.z );
		let nodeConst28 = ( nodeConst21.xy + ( fn25( 4, 5, nodeConst22 ) * nodeConst23 ) );
		nodeVar8 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst28, nodeConst21.z );
		nodeVar3 = ( ( ( ( ( nodeVar4 + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) * 0.2 );

	} else {

		nodeVar3 = 1.0;

	}

	nodeVar9 = mix( 1.0, nodeVar3, render.nodeUniform23 );
	nodeVar10 = ( vec3<f32>( clamp( dot( normalView, nodeConst18 ), 0.0, 1.0 ) ) * ( render.nodeUniform16 * vec3<f32>( nodeVar9 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst29 = clamp( dot( positionViewDirection, normalize( ( nodeConst18 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst30 = exp2( ( ( ( nodeConst29 * -5.55473 ) - 6.98316 ) * nodeConst29 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar10 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst30 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst30 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst31 = normalize( ( nodeConst18 + positionViewDirection ) );
	let nodeConst32 = clamp( dot( positionViewDirection, nodeConst31 ), 0.0, 1.0 );
	let nodeConst33 = exp2( ( ( ( nodeConst32 * -5.55473 ) - 6.98316 ) * nodeConst32 ) );
	let nodeConst34 = max( Roughness, 0.045 );
	let nodeConst35 = ( nodeConst34 * nodeConst34 );
	directSpecular = ( directSpecular + ( ( nodeVar10 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst33 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst33 ) ) ) * vec3<f32>( fn110( nodeConst35, clamp( dot( normalView, nodeConst18 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst35, clamp( dot( normalView, nodeConst31 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst36 = ( ( object.nodeUniform24 - object.nodeUniform25 ) / object.nodeUniform26 );
	let nodeConst37 = ( v_positionWorld + ( ( normalWorld * nodeConst36 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst38 = ( nodeConst36 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst37 >= ( object.nodeUniform25 - nodeConst38 ) ) ) && all( ( nodeConst37 <= ( object.nodeUniform24 + nodeConst38 ) ) ) ) && ( render.nodeUniform27 >= lightProbePriority ) ) ) {

		let nodeConst39 = ( object.nodeUniform24 - object.nodeUniform25 );
		let nodeConst40 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform26 );
		let nodeConst41 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst39 / object.nodeUniform26 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform25 ) / nodeConst39 ), nodeConst40, ( vec3<f32>( 1.0 ) - nodeConst40 ) );
		let nodeConst42 = ( ( nodeConst41.z * object.nodeUniform26.z ) + 1.0 );
		let nodeConst43 = ( object.nodeUniform26.z + 2.0 );
		let nodeConst44 = ( nodeConst43 * 7.0 );
		nodeVar11 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst41.xy, ( ( nodeConst42 + ( nodeConst43 * 0.0 ) ) / nodeConst44 ) ) );
		nodeVar12 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst41.xy, ( ( nodeConst42 + ( nodeConst43 * 1.0 ) ) / nodeConst44 ) ) );
		nodeVar13 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst41.xy, ( ( nodeConst42 + ( nodeConst43 * 2.0 ) ) / nodeConst44 ) ) );
		nodeVar14 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst41.xy, ( ( nodeConst42 + ( nodeConst43 * 3.0 ) ) / nodeConst44 ) ) );
		nodeVar15 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst41.xy, ( ( nodeConst42 + ( nodeConst43 * 4.0 ) ) / nodeConst44 ) ) );
		nodeVar16 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst41.xy, ( ( nodeConst42 + ( nodeConst43 * 5.0 ) ) / nodeConst44 ) ) );
		nodeVar17 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst41.xy, ( ( nodeConst42 + ( nodeConst43 * 6.0 ) ) / nodeConst44 ) ) );
		let nodeConst45 = array< vec3<f32>, 9 >( nodeVar11.xyz, vec3<f32>( nodeVar11.w, nodeVar12.xy ), vec3<f32>( nodeVar12.zw, nodeVar13.x ), nodeVar13.yzw, nodeVar14.xyz, vec3<f32>( nodeVar14.w, nodeVar15.xy ), vec3<f32>( nodeVar15.zw, nodeVar16.x ), nodeVar16.yzw, nodeVar17.xyz );
		let nodeConst46 = ( max( ( ( ( ( ( ( ( ( ( nodeConst45[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst45[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst45[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst45[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst45[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst45[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst45[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst45[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst45[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform29 ) );
		lightProbeIrradiance = nodeConst46;
		lightProbePriority = render.nodeUniform27;
		

	}

	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst47 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst48 = ( object.nodeUniform31 * vec4<f32>( nodeConst47, 1.0 ) );
	let nodeConst49 = clamp( Roughness, 0.0, 1.0 );
	nodeVar18 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, vec3<f32>( ( - nodeConst48.x ), nodeConst48.yz ), ( ( object.nodeUniform33 * nodeConst49 ) * ( 2.0 - nodeConst49 ) ) );
	radiance = ( radiance + ( nodeVar18.xyz * vec3<f32>( object.nodeUniform34 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst50 = ( object.nodeUniform31 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst51 = clamp( 1.0, 0.0, 1.0 );
	nodeVar19 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, vec3<f32>( ( - nodeConst50.x ), nodeConst50.yz ), ( ( object.nodeUniform33 * nodeConst51 ) * ( 2.0 - nodeConst51 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar19.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform34 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar20 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar20 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst52 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst52 );
	let nodeConst53 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst54 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst52 * nodeConst53 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst54 ) * nodeConst53 ) ) ) * vec3<f32>( nodeConst54 ) ) );
	let nodeConst55 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar21 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst55 ) );
	nodeVar22 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst55 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar21 );
	indirectDiffuse = ( indirectDiffuse + nodeVar22 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst56 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst56;

	// result

	output.color = nodeConst56;

	return output;

}
