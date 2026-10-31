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
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform21_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform21 : texture_cube<f32>;

struct objectStruct {
	nodeUniform5 : vec3<f32>,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform22 : mat4x4<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
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
var<private> nodeVar2 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

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
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform6, nodeUniform6_sampler, ( object.nodeUniform7 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform5, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform8 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform9;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform10, 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform9 ) ) );
	EmissiveColor = ( object.nodeUniform13 * vec3<f32>( object.nodeUniform14 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform15, nodeUniform15_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst2 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst2 );
	let nodeConst3 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst4 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst2 * nodeConst3 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst4 ) * nodeConst3 ) ) ) * vec3<f32>( nodeConst4 ) ) );
	let nodeConst5 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform18 - render.nodeUniform19 ), 0.0 ) ).xyz );
	nodeVar2 = ( vec3<f32>( clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ) ) * render.nodeUniform20 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst6 = clamp( dot( positionViewDirection, normalize( ( nodeConst5 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst7 = exp2( ( ( ( nodeConst6 * -5.55473 ) - 6.98316 ) * nodeConst6 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar2 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst7 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst7 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst8 = normalize( ( nodeConst5 + positionViewDirection ) );
	let nodeConst9 = clamp( dot( positionViewDirection, nodeConst8 ), 0.0, 1.0 );
	let nodeConst10 = exp2( ( ( ( nodeConst9 * -5.55473 ) - 6.98316 ) * nodeConst9 ) );
	let nodeConst11 = max( Roughness, 0.045 );
	let nodeConst12 = ( nodeConst11 * nodeConst11 );
	directSpecular = ( directSpecular + ( ( nodeVar2 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst10 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst10 ) ) ) * vec3<f32>( fn110( nodeConst12, clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst12, clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst13 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst14 = ( object.nodeUniform22 * vec4<f32>( nodeConst13, 1.0 ) );
	let nodeConst15 = clamp( Roughness, 0.0, 1.0 );
	nodeVar3 = textureSampleLevel( nodeUniform21, nodeUniform21_sampler, vec3<f32>( ( - nodeConst14.x ), nodeConst14.yz ), ( ( object.nodeUniform24 * nodeConst15 ) * ( 2.0 - nodeConst15 ) ) );
	radiance = ( radiance + ( nodeVar3.xyz * vec3<f32>( object.nodeUniform25 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst16 = ( object.nodeUniform22 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst17 = clamp( 1.0, 0.0, 1.0 );
	nodeVar4 = textureSampleLevel( nodeUniform21, nodeUniform21_sampler, vec3<f32>( ( - nodeConst16.x ), nodeConst16.yz ), ( ( object.nodeUniform24 * nodeConst17 ) * ( 2.0 - nodeConst17 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar4.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform25 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar5 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar5 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst18 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst18 );
	let nodeConst19 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst20 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst18 * nodeConst19 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst20 ) * nodeConst19 ) ) ) * vec3<f32>( nodeConst20 ) ) );
	let nodeConst21 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar6 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst21 ) );
	nodeVar7 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst21 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar6 );
	indirectDiffuse = ( indirectDiffuse + nodeVar7 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst22 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst22;

	// result

	output.color = nodeConst22;

	return output;

}
