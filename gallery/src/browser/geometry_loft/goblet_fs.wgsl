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
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform16_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform16 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform20 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>,
	nodeUniform19 : f32
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
	@builtin( front_facing ) isFront : bool,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	let nodeConst0 = fn93( ( positionLocal * vec3<f32>( 5.0 ) ), 1.0, 0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( nodeConst0 * 0.18 ) + 0.12 ), 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	let nodeConst2 = normalize( dpdx( v_positionView ) );
	NORMAL_normalView = normalViewGeometry;
	let nodeConst3 = cross( normalize( - dpdy( v_positionView ) ), NORMAL_normalView );
	let nodeConst4 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	let nodeConst5 = ( dot( nodeConst2, nodeConst3 ) * nodeConst4 );
	let nodeConst6 = ( nodeConst0 * 0.1 );
	let nodeConst7 = ( vec2<f32>( ( ( nodeConst0 * 0.1 ) - nodeConst6 ), ( ( nodeConst0 * 0.1 ) - nodeConst6 ) ) * vec2<f32>( 1.0 ) );
	normalView = normalize( ( ( vec3<f32>( abs( nodeConst5 ) ) * NORMAL_normalView ) - ( vec3<f32>( sign( nodeConst5 ) ) * ( ( vec3<f32>( nodeConst7.x ) * nodeConst3 ) + ( vec3<f32>( nodeConst7.y ) * cross( NORMAL_normalView, nodeConst2 ) ) ) ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst8 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst8 );
	let nodeConst9 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst10 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst8 * nodeConst9 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst10 ) * nodeConst9 ) ) ) * vec3<f32>( nodeConst10 ) ) );
	let nodeConst11 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform10 - render.nodeUniform11 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst12 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	let nodeConst13 = ( nodeConst12.xyz / vec3<f32>( nodeConst12.w ) );
	let nodeConst14 = vec3<f32>( nodeConst13.x, ( 1.0 - nodeConst13.y ), ( nodeConst13.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeConst14.x >= 0.0 ) && ( nodeConst14.x <= 1.0 ) ) && ( nodeConst14.y >= 0.0 ) ) && ( nodeConst14.y <= 1.0 ) ) && ( nodeConst14.z <= 1.0 ) ) ) {

		let nodeConst15 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst16 = ( vec2<f32>( render.nodeUniform17 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ) );
		let nodeConst17 = ( nodeConst14.xy + ( fn25( 0, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar2 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst17, nodeConst14.z );
		let nodeConst18 = ( nodeConst14.xy + ( fn25( 1, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar3 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst18, nodeConst14.z );
		let nodeConst19 = ( nodeConst14.xy + ( fn25( 2, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar4 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst19, nodeConst14.z );
		let nodeConst20 = ( nodeConst14.xy + ( fn25( 3, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar5 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst20, nodeConst14.z );
		let nodeConst21 = ( nodeConst14.xy + ( fn25( 4, 5, nodeConst15 ) * nodeConst16 ) );
		nodeVar6 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst21, nodeConst14.z );
		nodeVar1 = ( ( ( ( ( nodeVar2 + nodeVar3 ) + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) * 0.2 );

	} else {

		nodeVar1 = 1.0;

	}

	nodeVar7 = mix( 1.0, nodeVar1, render.nodeUniform19 );
	nodeVar8 = ( vec3<f32>( clamp( dot( normalView, nodeConst11 ), 0.0, 1.0 ) ) * ( render.nodeUniform12 * vec3<f32>( nodeVar7 ) ) );
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
	let nodeConst30 = ( object.nodeUniform21 * vec4<f32>( nodeConst29, 1.0 ) );
	let nodeConst31 = clamp( Roughness, 0.0, 1.0 );
	nodeVar9 = textureSampleLevel( nodeUniform20, nodeUniform20_sampler, vec3<f32>( ( - nodeConst30.x ), nodeConst30.yz ), ( ( object.nodeUniform23 * nodeConst31 ) * ( 2.0 - nodeConst31 ) ) );
	radiance = ( radiance + ( nodeVar9.xyz * vec3<f32>( object.nodeUniform24 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst32 = ( object.nodeUniform21 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst33 = clamp( 1.0, 0.0, 1.0 );
	nodeVar10 = textureSampleLevel( nodeUniform20, nodeUniform20_sampler, vec3<f32>( ( - nodeConst32.x ), nodeConst32.yz ), ( ( object.nodeUniform23 * nodeConst33 ) * ( 2.0 - nodeConst33 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar10.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform24 ) ) );
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
