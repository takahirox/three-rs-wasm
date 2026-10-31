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
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform21 : f32
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
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar3 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;

// codes
fn fn132 ( p : vec3<f32> ) -> vec3<f32> {

	


	return fract( ( sin( vec3<f32>( dot( p, vec3<f32>( 127.1, 311.7, 74.7 ) ), dot( p, vec3<f32>( 269.5, 183.3, 246.1 ) ), dot( p, vec3<f32>( 113.5, 271.9, 124.6 ) ) ) ) * vec3<f32>( 18.5453 ) ) );

}


fn fn133 ( p : vec3<f32>, time : f32 ) -> f32 {

	var nodeVar0 : f32;

	let nodeConst0 = floor( p );
	let nodeConst1 = fract( p );
	nodeVar0 = 8.0;

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {


			for ( var z : i32 = -1; z <= 1; z ++ ) {

				let nodeConst2 = vec3<f32>( f32( x ), f32( y ), f32( z ) );
				let nodeConst3 = fn132( ( nodeConst0 + nodeConst2 ) );
				let nodeConst4 = ( ( nodeConst2 - nodeConst1 ) + ( ( sin( ( vec3<f32>( time ) + ( nodeConst3 * vec3<f32>( 6.283185307179586 ) ) ) ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ) );
				nodeVar0 = min( nodeVar0, dot( nodeConst4, nodeConst4 ) );

			}


		}


	}


	return nodeVar0;

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
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) nodeVarying7 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, ( positionLocal.yz * vec2<f32>( 1.0 ) ) );
	normalLocal = nodeVarying7;
	let nodeConst0 = normalize( abs( normalLocal ) );
	let nodeConst1 = ( nodeConst0 / vec3<f32>( dot( nodeConst0, vec3<f32>( 1.0, 1.0, 1.0 ) ) ) );
	nodeVar1 = textureSample( nodeUniform0, nodeUniform0_sampler, ( positionLocal.zx * vec2<f32>( 1.0 ) ) );
	nodeVar2 = textureSample( nodeUniform0, nodeUniform0_sampler, ( positionLocal.xy * vec2<f32>( 1.0 ) ) );
	let nodeConst2 = ( ( ( ( ( nodeVar0 * vec4<f32>( nodeConst1.x ) ) + ( nodeVar1 * vec4<f32>( nodeConst1.y ) ) ) + ( nodeVar2 * vec4<f32>( nodeConst1.z ) ) ) + vec4<f32>( vec3<f32>( 0.0, 0.13286832154414627, 1.0 ), 1.0 ) ) * vec4<f32>( 0.8 ) );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	DiffuseColor = mix( nodeConst2, ( nodeConst2 + vec4<f32>( fn133( ( v_positionWorld * vec3<f32>( 6.0 ) ), ( render.nodeUniform2 * 0.8 ) ) ) ), mix( clamp( ( 1.0 - distance( v_positionWorld.y, 0.0 ) ), 0.0, 1.0 ), 0.0, normalWorld.y ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform7;
	let nodeConst3 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform8, 0.045 ) + max( max( nodeConst3.x, nodeConst3.y ), nodeConst3.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform7 ) ) );
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar3 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst4 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst4 );
	let nodeConst5 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst6 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst4 * nodeConst5 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst6 ) * nodeConst5 ) ) ) * vec3<f32>( nodeConst6 ) ) );
	let nodeConst7 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform12 - render.nodeUniform13 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	let nodeConst8 = ( render.nodeUniform15 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform16 ) ) ), 1.0 ) );
	let nodeConst9 = ( nodeConst8.xyz / vec3<f32>( nodeConst8.w ) );
	let nodeConst10 = vec3<f32>( nodeConst9.x, ( 1.0 - nodeConst9.y ), ( nodeConst9.z + render.nodeUniform17 ) );

	if ( ( ( ( ( ( nodeConst10.x >= 0.0 ) && ( nodeConst10.x <= 1.0 ) ) && ( nodeConst10.y >= 0.0 ) ) && ( nodeConst10.y <= 1.0 ) ) && ( nodeConst10.z <= 1.0 ) ) ) {

		let nodeConst11 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst12 = ( vec2<f32>( render.nodeUniform19 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform20 ) );
		let nodeConst13 = ( nodeConst10.xy + ( fn25( 0, 5, nodeConst11 ) * nodeConst12 ) );
		nodeVar5 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst13, nodeConst10.z );
		let nodeConst14 = ( nodeConst10.xy + ( fn25( 1, 5, nodeConst11 ) * nodeConst12 ) );
		nodeVar6 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst14, nodeConst10.z );
		let nodeConst15 = ( nodeConst10.xy + ( fn25( 2, 5, nodeConst11 ) * nodeConst12 ) );
		nodeVar7 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst15, nodeConst10.z );
		let nodeConst16 = ( nodeConst10.xy + ( fn25( 3, 5, nodeConst11 ) * nodeConst12 ) );
		nodeVar8 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst16, nodeConst10.z );
		let nodeConst17 = ( nodeConst10.xy + ( fn25( 4, 5, nodeConst11 ) * nodeConst12 ) );
		nodeVar9 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeConst17, nodeConst10.z );
		nodeVar4 = ( ( ( ( ( nodeVar5 + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) + nodeVar9 ) * 0.2 );

	} else {

		nodeVar4 = 1.0;

	}

	nodeVar10 = mix( 1.0, nodeVar4, render.nodeUniform21 );
	nodeVar11 = ( vec3<f32>( clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ) ) * ( render.nodeUniform14 * vec3<f32>( nodeVar10 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst18 = clamp( dot( positionViewDirection, normalize( ( nodeConst7 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst19 = exp2( ( ( ( nodeConst18 * -5.55473 ) - 6.98316 ) * nodeConst18 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar11 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst19 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst19 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst20 = normalize( ( nodeConst7 + positionViewDirection ) );
	let nodeConst21 = clamp( dot( positionViewDirection, nodeConst20 ), 0.0, 1.0 );
	let nodeConst22 = exp2( ( ( ( nodeConst21 * -5.55473 ) - 6.98316 ) * nodeConst21 ) );
	let nodeConst23 = max( Roughness, 0.045 );
	let nodeConst24 = ( nodeConst23 * nodeConst23 );
	directSpecular = ( directSpecular + ( ( nodeVar11 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst22 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst22 ) ) ) * vec3<f32>( fn110( nodeConst24, clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst24, clamp( dot( normalView, nodeConst20 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + mix( render.nodeUniform22, render.nodeUniform23, ( ( dot( normalWorld, normalize( render.nodeUniform24 ) ) * 0.5 ) + 0.5 ) ) );
	irradiance = ( irradiance + mix( render.nodeUniform25, render.nodeUniform26, ( ( dot( normalWorld, normalize( render.nodeUniform27 ) ) * 0.5 ) + 0.5 ) ) );
	nodeVar12 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar12 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst25 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst25 );
	let nodeConst26 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst27 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst25 * nodeConst26 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst27 ) * nodeConst26 ) ) ) * vec3<f32>( nodeConst27 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst28 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar13 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst28 ) );
	nodeVar14 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst28 );
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
	nodeVar15 = vec4<f32>( mix( Output.xyz, render.nodeUniform28, smoothstep( render.nodeUniform29, render.nodeUniform30, ( - v_positionView.z ) ) ), Output.w );
	let nodeConst29 = nodeVar15;
	Output = nodeConst29;

	// result

	output.color = nodeConst29;

	return output;

}
