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
@binding( 1 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform17_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform17 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>
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
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
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
	@location( 1 ) nodeVarying4 : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform3, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	Metalness = object.nodeUniform5;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform6, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform5 ) ) );
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	normalView = nodeVarying4;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar5 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar5;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst7 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst7 );
	let nodeConst8 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst9 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst7 * nodeConst8 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst9 ) * nodeConst8 ) ) ) * vec3<f32>( nodeConst9 ) ) );
	let nodeConst10 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform14 - render.nodeUniform15 ), 0.0 ) ).xyz );
	nodeVar6 = ( vec3<f32>( clamp( dot( normalView, nodeConst10 ), 0.0, 1.0 ) ) * render.nodeUniform16 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst11 = clamp( dot( positionViewDirection, normalize( ( nodeConst10 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst12 = exp2( ( ( ( nodeConst11 * -5.55473 ) - 6.98316 ) * nodeConst11 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar6 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst12 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst12 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst13 = normalize( ( nodeConst10 + positionViewDirection ) );
	let nodeConst14 = clamp( dot( positionViewDirection, nodeConst13 ), 0.0, 1.0 );
	let nodeConst15 = exp2( ( ( ( nodeConst14 * -5.55473 ) - 6.98316 ) * nodeConst14 ) );
	let nodeConst16 = max( Roughness, 0.045 );
	let nodeConst17 = ( nodeConst16 * nodeConst16 );
	directSpecular = ( directSpecular + ( ( nodeVar6 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst15 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst15 ) ) ) * vec3<f32>( fn110( nodeConst17, clamp( dot( normalView, nodeConst10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst17, clamp( dot( normalView, nodeConst13 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst18 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst19 = ( object.nodeUniform18 * vec4<f32>( nodeConst18, 1.0 ) );
	let nodeConst20 = clamp( Roughness, 0.0, 1.0 );
	nodeVar7 = textureSampleLevel( nodeUniform17, nodeUniform17_sampler, vec3<f32>( ( - nodeConst19.x ), nodeConst19.yz ), ( ( object.nodeUniform20 * nodeConst20 ) * ( 2.0 - nodeConst20 ) ) );
	radiance = ( radiance + ( nodeVar7.xyz * vec3<f32>( object.nodeUniform21 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst21 = ( object.nodeUniform18 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst22 = clamp( 1.0, 0.0, 1.0 );
	nodeVar8 = textureSampleLevel( nodeUniform17, nodeUniform17_sampler, vec3<f32>( ( - nodeConst21.x ), nodeConst21.yz ), ( ( object.nodeUniform20 * nodeConst22 ) * ( 2.0 - nodeConst22 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar8.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform21 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar9 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst23 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst23 );
	let nodeConst24 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst25 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst23 * nodeConst24 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst25 ) * nodeConst24 ) ) ) * vec3<f32>( nodeConst25 ) ) );
	let nodeConst26 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar10 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst26 ) );
	nodeVar11 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst26 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar10 );
	indirectDiffuse = ( indirectDiffuse + nodeVar11 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst27 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst27;

	// result

	output.color = nodeConst27;

	return output;

}
