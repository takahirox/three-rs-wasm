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
@binding( 3 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform14 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : vec3<f32>
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
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
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
	@location( 1 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

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
	let nodeConst4 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform11 - render.nodeUniform12 ), 0.0 ) ).xyz );
	nodeVar1 = ( vec3<f32>( clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ) ) * render.nodeUniform13 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst5 = clamp( dot( positionViewDirection, normalize( ( nodeConst4 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst6 = exp2( ( ( ( nodeConst5 * -5.55473 ) - 6.98316 ) * nodeConst5 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar1 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst6 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst6 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst7 = normalize( ( nodeConst4 + positionViewDirection ) );
	let nodeConst8 = clamp( dot( positionViewDirection, nodeConst7 ), 0.0, 1.0 );
	let nodeConst9 = exp2( ( ( ( nodeConst8 * -5.55473 ) - 6.98316 ) * nodeConst8 ) );
	let nodeConst10 = max( Roughness, 0.045 );
	let nodeConst11 = ( nodeConst10 * nodeConst10 );
	directSpecular = ( directSpecular + ( ( nodeVar1 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst9 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst9 ) ) ) * vec3<f32>( fn110( nodeConst11, clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst11, clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst12 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst13 = ( object.nodeUniform15 * vec4<f32>( nodeConst12, 1.0 ) );
	let nodeConst14 = clamp( Roughness, 0.0, 1.0 );
	nodeVar2 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec3<f32>( ( - nodeConst13.x ), nodeConst13.yz ), ( ( object.nodeUniform17 * nodeConst14 ) * ( 2.0 - nodeConst14 ) ) );
	radiance = ( radiance + ( nodeVar2.xyz * vec3<f32>( object.nodeUniform18 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst15 = ( object.nodeUniform15 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst16 = clamp( 1.0, 0.0, 1.0 );
	nodeVar3 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec3<f32>( ( - nodeConst15.x ), nodeConst15.yz ), ( ( object.nodeUniform17 * nodeConst16 ) * ( 2.0 - nodeConst16 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar3.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform18 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar4 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar4 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst17 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst17 );
	let nodeConst18 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst19 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst17 * nodeConst18 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst19 ) * nodeConst18 ) ) ) * vec3<f32>( nodeConst19 ) ) );
	let nodeConst20 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar5 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst20 ) );
	nodeVar6 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst20 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar5 );
	indirectDiffuse = ( indirectDiffuse + nodeVar6 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst21 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst21;

	// result

	output.color = nodeConst21;

	return output;

}
