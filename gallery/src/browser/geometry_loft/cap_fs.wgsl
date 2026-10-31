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


fn fn61 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn fn62 ( a : u32, b : u32, c : u32 ) -> vec3<u32> {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = a;
	nodeVar1 = b;
	nodeVar2 = c;
	nodeVar0 = ( nodeVar0 - nodeVar2 );
	nodeVar0 = ( nodeVar0 ^ fn61( nodeVar2, 4 ) );
	nodeVar2 = ( nodeVar2 + nodeVar1 );
	nodeVar1 = ( nodeVar1 - nodeVar0 );
	nodeVar1 = ( nodeVar1 ^ fn61( nodeVar0, 6 ) );
	nodeVar0 = ( nodeVar0 + nodeVar2 );
	nodeVar2 = ( nodeVar2 - nodeVar1 );
	nodeVar2 = ( nodeVar2 ^ fn61( nodeVar1, 8 ) );
	nodeVar1 = ( nodeVar1 + nodeVar0 );
	nodeVar0 = ( nodeVar0 - nodeVar2 );
	nodeVar0 = ( nodeVar0 ^ fn61( nodeVar2, 16 ) );
	nodeVar2 = ( nodeVar2 + nodeVar1 );
	nodeVar1 = ( nodeVar1 - nodeVar0 );
	nodeVar1 = ( nodeVar1 ^ fn61( nodeVar0, 19 ) );
	nodeVar0 = ( nodeVar0 + nodeVar2 );
	nodeVar2 = ( nodeVar2 - nodeVar1 );
	nodeVar2 = ( nodeVar2 ^ fn61( nodeVar1, 4 ) );
	nodeVar1 = ( nodeVar1 + nodeVar0 );

	return vec3<u32>( nodeVar0, nodeVar1, nodeVar2 );

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


fn fn64 ( bits : u32 ) -> f32 {

	var nodeVar0 : u32;

	nodeVar0 = bits;

	return ( f32( nodeVar0 ) / 4294967295.0 );

}


fn fn85 ( position : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;
	var nodeVar6 : u32;
	var nodeVar7 : u32;
	var nodeVar8 : vec3<u32>;

	nodeVar0 = position;
	nodeVar1 = i32( floor( nodeVar0.x ) );
	nodeVar2 = i32( floor( nodeVar0.y ) );
	nodeVar3 = i32( floor( nodeVar0.z ) );
	nodeVar4 = 3735928588u;
	nodeVar5 = 0u;
	nodeVar6 = 0u;
	nodeVar7 = 0u;
	nodeVar7 = nodeVar4;
	nodeVar6 = nodeVar7;
	nodeVar5 = nodeVar6;
	nodeVar5 = ( nodeVar5 + u32( nodeVar1 ) );
	nodeVar6 = ( nodeVar6 + u32( nodeVar2 ) );
	nodeVar7 = ( nodeVar7 + u32( nodeVar3 ) );
	nodeVar8 = fn62( nodeVar5, nodeVar6, nodeVar7 );

	return vec3<f32>( fn64( fn63( nodeVar8.x, nodeVar8.y, nodeVar8.z ) ), fn64( fn63( ( nodeVar8.x + 1u ), nodeVar8.y, nodeVar8.z ) ), fn64( fn63( ( nodeVar8.x + 2u ), nodeVar8.y, nodeVar8.z ) ) );

}


fn fn92 ( p : vec3<f32>, x : i32, y : i32, z : i32, xoff : i32, yoff : i32, zoff : i32, jitter : f32, metric : i32 ) -> f32 {

	var nodeVar0 : i32;
	var nodeVar1 : f32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : i32;
	var nodeVar5 : i32;
	var nodeVar6 : i32;
	var nodeVar7 : i32;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : vec3<f32>;
	var nodeVar11 : vec3<f32>;

	nodeVar0 = metric;
	nodeVar1 = jitter;
	nodeVar2 = zoff;
	nodeVar3 = yoff;
	nodeVar4 = xoff;
	nodeVar5 = z;
	nodeVar6 = y;
	nodeVar7 = x;
	nodeVar8 = p;
	nodeVar9 = fn85( vec3<f32>( f32( ( nodeVar7 + nodeVar4 ) ), f32( ( nodeVar6 + nodeVar3 ) ), f32( ( nodeVar5 + nodeVar2 ) ) ) );
	nodeVar9 = ( nodeVar9 - vec3<f32>( 0.5 ) );
	nodeVar9 = ( nodeVar9 * vec3<f32>( nodeVar1 ) );
	nodeVar9 = ( nodeVar9 + vec3<f32>( 0.5 ) );
	nodeVar10 = ( vec3<f32>( f32( nodeVar7 ), f32( nodeVar6 ), f32( nodeVar5 ) ) + nodeVar9 );
	nodeVar11 = ( nodeVar10 - nodeVar8 );

	if ( ( nodeVar0 == 2 ) ) {

		return ( ( abs( nodeVar11.x ) + abs( nodeVar11.y ) ) + abs( nodeVar11.z ) );

	}


	if ( ( nodeVar0 == 3 ) ) {

		return max( max( abs( nodeVar11.x ), abs( nodeVar11.y ) ), abs( nodeVar11.z ) );

	}


	return dot( nodeVar11, nodeVar11 );

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


fn fn79 ( p : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;

	nodeVar0 = p;
	nodeVar1 = fn48( nodeVar0.x );
	nodeVar2 = fn48( nodeVar0.y );
	nodeVar3 = fn48( nodeVar0.z );

	return fn64( fn68( nodeVar1, nodeVar2, nodeVar3 ) );

}


fn fn93 ( position : vec3<f32>, jitter : f32, style : i32 ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : i32;
	var nodeVar5 : i32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : f32;
	var nodeVar11 : vec3<f32>;
	var nodeVar12 : f32;
	var nodeVar13 : vec3<f32>;
	var nodeVar14 : vec3<f32>;

	nodeVar0 = position;
	nodeVar1 = jitter;
	nodeVar2 = style;
	nodeVar3 = 0;
	nodeVar4 = 0;
	nodeVar5 = 0;
	nodeVar6 = nodeVar0.x;
	nodeVar3 = fn48( nodeVar6 );
	nodeVar7 = nodeVar0.y;
	nodeVar4 = fn48( nodeVar7 );
	nodeVar8 = nodeVar0.z;
	nodeVar5 = fn48( nodeVar8 );
	nodeVar9 = vec3<f32>( ( nodeVar6 - f32( nodeVar3 ) ), ( nodeVar7 - f32( nodeVar4 ) ), ( nodeVar8 - f32( nodeVar5 ) ) );
	nodeVar10 = 1000000.0;
	nodeVar11 = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {


			for ( var z : i32 = -1; z <= 1; z ++ ) {

				nodeVar12 = fn92( nodeVar9, x, y, z, nodeVar3, nodeVar4, nodeVar5, nodeVar1, 0 );
				nodeVar13 = fn85( vec3<f32>( f32( ( nodeVar3 + x ) ), f32( ( nodeVar4 + y ) ), f32( ( nodeVar5 + z ) ) ) );
				nodeVar13 = ( nodeVar13 - vec3<f32>( 0.5 ) );
				nodeVar13 = ( nodeVar13 * vec3<f32>( nodeVar1 ) );
				nodeVar13 = ( nodeVar13 + vec3<f32>( 0.5 ) );
				nodeVar14 = ( ( vec3<f32>( f32( x ), f32( y ), f32( z ) ) + nodeVar13 ) - nodeVar9 );

				if ( ( nodeVar12 < nodeVar10 ) ) {

					nodeVar10 = nodeVar12;
					nodeVar11 = nodeVar14;
					

				}


			}


		}


	}


	if ( ( nodeVar2 == 1 ) ) {

		nodeVar10 = fn79( ( nodeVar11 + nodeVar0 ) );
		

	} else {

		nodeVar10 = sqrt( nodeVar10 );
		

	}


	return nodeVar10;

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

	let nodeConst0 = smoothstep( 0.42, 0.58, nodeVarying7.x );
	let nodeConst1 = ( ( ( sin( ( nodeVarying7.y * 376.99111843077515 ) ) * 0.5 ) + 0.5 ) * ( 1.0 - nodeConst0 ) );
	let nodeConst2 = ( ( 1.0 - smoothstep( 0.18, 0.38, fn93( ( positionLocal * vec3<f32>( 2.4 ) ), 1.0, 0 ) ) ) * nodeConst0 );
	DiffuseColor = vec4<f32>( mix( mix( vec3<f32>( 0.8069522576650873, 0.7156935005005721, 0.55201140150344 ), vec3<f32>( 0.5209955731953768, 0.4232676699760063, 0.2704977910022518 ), nodeConst1 ), mix( vec3<f32>( 0.36625259558833256, 0.026241221889696346, 0.014443843592229466 ), vec3<f32>( 0.8879231178794776, 0.8148465722120952, 0.6866853124288864 ), nodeConst2 ), nodeConst0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst3 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( 0.55 - ( nodeConst0 * 0.2 ) ) + ( nodeConst2 * 0.25 ) ), 0.045 ) + max( max( nodeConst3.x, nodeConst3.y ), nodeConst3.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	let nodeConst4 = normalize( dpdx( v_positionView ) );
	NORMAL_normalView = normalViewGeometry;
	let nodeConst5 = cross( normalize( - dpdy( v_positionView ) ), NORMAL_normalView );
	let nodeConst6 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	let nodeConst7 = ( dot( nodeConst4, nodeConst5 ) * nodeConst6 );
	let nodeConst8 = ( ( nodeConst2 * 0.08 ) - ( nodeConst1 * 0.015 ) );
	let nodeConst9 = ( vec2<f32>( ( ( ( nodeConst2 * 0.08 ) - ( nodeConst1 * 0.015 ) ) - nodeConst8 ), ( ( ( nodeConst2 * 0.08 ) - ( nodeConst1 * 0.015 ) ) - nodeConst8 ) ) * vec2<f32>( 1.0 ) );
	normalView = normalize( ( ( vec3<f32>( abs( nodeConst7 ) ) * NORMAL_normalView ) - ( vec3<f32>( sign( nodeConst7 ) ) * ( ( vec3<f32>( nodeConst9.x ) * nodeConst5 ) + ( vec3<f32>( nodeConst9.y ) * cross( NORMAL_normalView, nodeConst4 ) ) ) ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst10 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst10 );
	let nodeConst11 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst12 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst10 * nodeConst11 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst12 ) * nodeConst11 ) ) ) * vec3<f32>( nodeConst12 ) ) );
	let nodeConst13 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform9 - render.nodeUniform10 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst14 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform13 ) ) ), 1.0 ) );
	let nodeConst15 = ( nodeConst14.xyz / vec3<f32>( nodeConst14.w ) );
	let nodeConst16 = vec3<f32>( nodeConst15.x, ( 1.0 - nodeConst15.y ), ( nodeConst15.z + render.nodeUniform14 ) );

	if ( ( ( ( ( ( nodeConst16.x >= 0.0 ) && ( nodeConst16.x <= 1.0 ) ) && ( nodeConst16.y >= 0.0 ) ) && ( nodeConst16.y <= 1.0 ) ) && ( nodeConst16.z <= 1.0 ) ) ) {

		let nodeConst17 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst18 = ( vec2<f32>( render.nodeUniform16 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ) );
		let nodeConst19 = ( nodeConst16.xy + ( fn25( 0, 5, nodeConst17 ) * nodeConst18 ) );
		nodeVar2 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst19, nodeConst16.z );
		let nodeConst20 = ( nodeConst16.xy + ( fn25( 1, 5, nodeConst17 ) * nodeConst18 ) );
		nodeVar3 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst20, nodeConst16.z );
		let nodeConst21 = ( nodeConst16.xy + ( fn25( 2, 5, nodeConst17 ) * nodeConst18 ) );
		nodeVar4 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst21, nodeConst16.z );
		let nodeConst22 = ( nodeConst16.xy + ( fn25( 3, 5, nodeConst17 ) * nodeConst18 ) );
		nodeVar5 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst22, nodeConst16.z );
		let nodeConst23 = ( nodeConst16.xy + ( fn25( 4, 5, nodeConst17 ) * nodeConst18 ) );
		nodeVar6 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst23, nodeConst16.z );
		nodeVar1 = ( ( ( ( ( nodeVar2 + nodeVar3 ) + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) * 0.2 );

	} else {

		nodeVar1 = 1.0;

	}

	nodeVar7 = mix( 1.0, nodeVar1, render.nodeUniform18 );
	nodeVar8 = ( vec3<f32>( clamp( dot( normalView, nodeConst13 ), 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar7 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst24 = clamp( dot( positionViewDirection, normalize( ( nodeConst13 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst25 = exp2( ( ( ( nodeConst24 * -5.55473 ) - 6.98316 ) * nodeConst24 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar8 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst25 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst25 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst26 = normalize( ( nodeConst13 + positionViewDirection ) );
	let nodeConst27 = clamp( dot( positionViewDirection, nodeConst26 ), 0.0, 1.0 );
	let nodeConst28 = exp2( ( ( ( nodeConst27 * -5.55473 ) - 6.98316 ) * nodeConst27 ) );
	let nodeConst29 = max( Roughness, 0.045 );
	let nodeConst30 = ( nodeConst29 * nodeConst29 );
	directSpecular = ( directSpecular + ( ( nodeVar8 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst28 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst28 ) ) ) * vec3<f32>( fn110( nodeConst30, clamp( dot( normalView, nodeConst13 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst30, clamp( dot( normalView, nodeConst26 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst31 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst32 = ( object.nodeUniform20 * vec4<f32>( nodeConst31, 1.0 ) );
	let nodeConst33 = clamp( Roughness, 0.0, 1.0 );
	nodeVar9 = textureSampleLevel( nodeUniform19, nodeUniform19_sampler, vec3<f32>( ( - nodeConst32.x ), nodeConst32.yz ), ( ( object.nodeUniform22 * nodeConst33 ) * ( 2.0 - nodeConst33 ) ) );
	radiance = ( radiance + ( nodeVar9.xyz * vec3<f32>( object.nodeUniform23 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst34 = ( object.nodeUniform20 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst35 = clamp( 1.0, 0.0, 1.0 );
	nodeVar10 = textureSampleLevel( nodeUniform19, nodeUniform19_sampler, vec3<f32>( ( - nodeConst34.x ), nodeConst34.yz ), ( ( object.nodeUniform22 * nodeConst35 ) * ( 2.0 - nodeConst35 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar10.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform23 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar11 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar11 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst36 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst36 );
	let nodeConst37 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst38 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst36 * nodeConst37 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst38 ) * nodeConst37 ) ) ) * vec3<f32>( nodeConst38 ) ) );
	let nodeConst39 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar12 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst39 ) );
	nodeVar13 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst39 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar12 );
	indirectDiffuse = ( indirectDiffuse + nodeVar13 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst40 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst40;

	// result

	output.color = nodeConst40;

	return output;

}
