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
@binding( 1 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform19 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : vec2<f32>,
	nodeUniform18 : f32
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
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
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) nodeVarying7 : vec2<f32>,
	@builtin( front_facing ) isFront : bool,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( mix( vec3<f32>( 0.02732089163382382, 0.158960835050774, 0.34191442489801843 ), vec3<f32>( 0.22322795730611386, 0.4793201830913402, 0.6866853124288864 ), ( ( ( ( fn74( ( positionLocal * vec3<f32>( 1.2 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	let nodeConst0 = sin( ( nodeVarying7.x * 160.0 ) );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( nodeConst0 * 0.08 ) + 0.3 ), 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	let nodeConst2 = normalize( dpdx( v_positionView ) );
	let nodeConst3 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeConst3 ) );
	let nodeConst4 = cross( normalize( - dpdy( v_positionView ) ), NORMAL_normalView );
	let nodeConst5 = ( dot( nodeConst2, nodeConst4 ) * nodeConst3 );
	let nodeConst6 = ( nodeConst0 * 0.012 );
	let nodeConst7 = ( vec2<f32>( ( ( nodeConst0 * 0.012 ) - nodeConst6 ), ( ( nodeConst0 * 0.012 ) - nodeConst6 ) ) * vec2<f32>( 1.0 ) );
	normalView = normalize( ( ( vec3<f32>( abs( nodeConst5 ) ) * NORMAL_normalView ) - ( vec3<f32>( sign( nodeConst5 ) ) * ( ( vec3<f32>( nodeConst7.x ) * nodeConst4 ) + ( vec3<f32>( nodeConst7.y ) * cross( NORMAL_normalView, nodeConst2 ) ) ) ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst8 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst8 );
	let nodeConst9 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst10 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst8 * nodeConst9 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst10 ) * nodeConst9 ) ) ) * vec3<f32>( nodeConst10 ) ) );
	let nodeConst11 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform9 - render.nodeUniform10 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst12 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform13 ) ) ), 1.0 ) );
	let nodeConst13 = ( nodeConst12.xyz / vec3<f32>( nodeConst12.w ) );
	let nodeConst14 = vec3<f32>( nodeConst13.x, ( 1.0 - nodeConst13.y ), ( nodeConst13.z + render.nodeUniform14 ) );

	if ( ( ( ( ( ( nodeConst14.x >= 0.0 ) && ( nodeConst14.x <= 1.0 ) ) && ( nodeConst14.y >= 0.0 ) ) && ( nodeConst14.y <= 1.0 ) ) && ( nodeConst14.z <= 1.0 ) ) ) {

		let nodeConst15 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst16 = ( vec2<f32>( render.nodeUniform16 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ) );
		let nodeConst17 = ( nodeConst14.xy + ( fn25( 0, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar2 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst17, nodeConst14.z );
		let nodeConst18 = ( nodeConst14.xy + ( fn25( 1, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar3 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst18, nodeConst14.z );
		let nodeConst19 = ( nodeConst14.xy + ( fn25( 2, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar4 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst19, nodeConst14.z );
		let nodeConst20 = ( nodeConst14.xy + ( fn25( 3, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar5 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst20, nodeConst14.z );
		let nodeConst21 = ( nodeConst14.xy + ( fn25( 4, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar6 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst21, nodeConst14.z );
		nodeVar1 = ( ( ( ( ( nodeVar2 + nodeVar3 ) + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) * 0.2 );

	} else {

		nodeVar1 = 1.0;

	}

	nodeVar7 = mix( 1.0, nodeVar1, render.nodeUniform18 );
	nodeVar8 = ( vec3<f32>( clamp( dot( normalView, nodeConst11 ), 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar7 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst22 = clamp( dot( positionViewDirection, normalize( ( nodeConst11 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst23 = exp2( ( ( ( nodeConst22 * -5.55473 ) - 6.98316 ) * nodeConst22 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar8 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst23 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst23 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst24 = normalize( ( nodeConst11 + positionViewDirection ) );
	let nodeConst25 = clamp( dot( positionViewDirection, nodeConst24 ), 0.0, 1.0 );
	let nodeConst26 = exp2( ( ( ( nodeConst25 * -5.55473 ) - 6.98316 ) * nodeConst25 ) );
	let nodeConst27 = max( Roughness, 0.045 );
	let nodeConst28 = ( nodeConst27 * nodeConst27 );
	directSpecular = ( directSpecular + ( ( nodeVar8 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst26 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst26 ) ) ) * vec3<f32>( fn110( nodeConst28, clamp( dot( normalView, nodeConst11 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst28, clamp( dot( normalView, nodeConst24 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst29 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst30 = ( object.nodeUniform20 * vec4<f32>( nodeConst29, 1.0 ) );
	let nodeConst31 = clamp( Roughness, 0.0, 1.0 );
	nodeVar9 = textureSampleLevel( nodeUniform19, nodeUniform19_sampler, vec3<f32>( ( - nodeConst30.x ), nodeConst30.yz ), ( ( object.nodeUniform22 * nodeConst31 ) * ( 2.0 - nodeConst31 ) ) );
	radiance = ( radiance + ( nodeVar9.xyz * vec3<f32>( object.nodeUniform23 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst32 = ( object.nodeUniform20 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst33 = clamp( 1.0, 0.0, 1.0 );
	nodeVar10 = textureSampleLevel( nodeUniform19, nodeUniform19_sampler, vec3<f32>( ( - nodeConst32.x ), nodeConst32.yz ), ( ( object.nodeUniform22 * nodeConst33 ) * ( 2.0 - nodeConst33 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar10.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform23 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar11 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar11 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst34 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst34 );
	let nodeConst35 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst36 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst34 * nodeConst35 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst36 ) * nodeConst35 ) ) ) * vec3<f32>( nodeConst36 ) ) );
	let nodeConst37 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar12 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst37 ) );
	nodeVar13 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst37 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar12 );
	indirectDiffuse = ( indirectDiffuse + nodeVar13 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst38 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst38;

	// result

	output.color = nodeConst38;

	return output;

}
