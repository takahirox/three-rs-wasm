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
@binding( 1 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform14 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : f32,
	nodeUniform16 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> normalFlat : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar3 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;

// codes
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
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) vInstanceColor : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( object.nodeUniform8 * vInstanceColor ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform9 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform10;
	Roughness = min( ( max( object.nodeUniform11, 0.045 ) + 0.0 ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform10 ) ) );
	EmissiveColor = ( object.nodeUniform12 * vec3<f32>( object.nodeUniform13 ) );
	normalFlat = normalize( cross( dpdx( v_positionView ), - dpdy( v_positionView ) ) );
	normalViewGeometry = normalFlat;
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar3 = textureSample( nodeUniform14, nodeUniform14_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst3 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst3 );
	let nodeConst4 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst5 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst3 * nodeConst4 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst5 ) * nodeConst4 ) ) ) * vec3<f32>( nodeConst5 ) ) );
	let nodeConst6 = normalize( ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform18, 0.0 ) ).xyz );
	nodeVar4 = ( vec3<f32>( clamp( dot( normalView, nodeConst6 ), 0.0, 1.0 ) ) * render.nodeUniform19 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst7 = clamp( dot( positionViewDirection, normalize( ( nodeConst6 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst8 = exp2( ( ( ( nodeConst7 * -5.55473 ) - 6.98316 ) * nodeConst7 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar4 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst8 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst8 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst9 = normalize( ( nodeConst6 + positionViewDirection ) );
	let nodeConst10 = clamp( dot( positionViewDirection, nodeConst9 ), 0.0, 1.0 );
	let nodeConst11 = exp2( ( ( ( nodeConst10 * -5.55473 ) - 6.98316 ) * nodeConst10 ) );
	let nodeConst12 = max( Roughness, 0.045 );
	let nodeConst13 = ( nodeConst12 * nodeConst12 );
	directSpecular = ( directSpecular + ( ( nodeVar4 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst11 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst11 ) ) ) * vec3<f32>( fn110( nodeConst13, clamp( dot( normalView, nodeConst6 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst13, clamp( dot( normalView, nodeConst9 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	irradiance = ( irradiance + mix( render.nodeUniform20, render.nodeUniform21, ( ( dot( normalWorld, normalize( render.nodeUniform22 ) ) * 0.5 ) + 0.5 ) ) );
	nodeVar5 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar5 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst14 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst14 );
	let nodeConst15 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst16 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst14 * nodeConst15 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst16 ) * nodeConst15 ) ) ) * vec3<f32>( nodeConst16 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst17 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar6 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst17 ) );
	nodeVar7 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst17 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar6 );
	indirectDiffuse = ( indirectDiffuse + nodeVar7 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar8 = vec4<f32>( mix( Output.xyz, render.nodeUniform23, smoothstep( render.nodeUniform24, render.nodeUniform25, ( - v_positionView.z ) ) ), Output.w );
	let nodeConst18 = nodeVar8;
	Output = nodeConst18;

	// result

	output.color = nodeConst18;

	return output;

}
