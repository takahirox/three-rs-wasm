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
@binding( 3 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform8 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> Sheen : vec3<f32>;
var<private> SheenRoughness : f32;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> sheenSpecularDirect : vec3<f32>;
var<private> sheenSpecularIndirect : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar0 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn128 ( normal : vec3<f32>, viewDir : vec3<f32>, roughness : f32 ) -> f32 {

	

	let nodeConst0 = ( roughness * roughness );
	let nodeConst1 = ( 1.0 / ( roughness + 0.1 ) );

	return clamp( exp( ( ( ( ( ( -1.9362 + ( roughness * 1.0678 ) ) + ( nodeConst0 * 0.4573 ) ) - ( nodeConst1 * 0.8469 ) ) * clamp( dot( normal, viewDir ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( roughness * 0.5538 ) ) - ( nodeConst0 * 0.467 ) ) - ( nodeConst1 * 0.1255 ) ) ) ), 0.0, 1.0 );

}




@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 0.8 ) * vec3<f32>( 0.029, 0.0, 0.047 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.693, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	IOR = 0.0;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	let nodeConst2 = 0.0;
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * vec3<f32>( 0.0, 0.0, 0.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( nodeConst2 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( nodeConst2, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	Sheen = ( vec3<f32>( 1.0 ) * vec3<f32>( 0.404, 0.058, 1.0 ) );
	SheenRoughness = 0.3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	sheenSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	sheenSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst3 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst3 );
	let nodeConst4 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst5 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst3 * nodeConst4 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst5 ) * nodeConst4 ) ) ) * vec3<f32>( nodeConst5 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst6 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst7 = ( object.nodeUniform9 * vec4<f32>( nodeConst6, 1.0 ) );
	let nodeConst8 = clamp( Roughness, 0.0, 1.0 );
	nodeVar1 = textureSampleLevel( nodeUniform8, nodeUniform8_sampler, vec3<f32>( ( - nodeConst7.x ), nodeConst7.yz ), ( ( object.nodeUniform11 * nodeConst8 ) * ( 2.0 - nodeConst8 ) ) );
	radiance = ( radiance + ( nodeVar1.xyz * vec3<f32>( object.nodeUniform12 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst9 = ( object.nodeUniform9 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst10 = clamp( 1.0, 0.0, 1.0 );
	nodeVar2 = textureSampleLevel( nodeUniform8, nodeUniform8_sampler, vec3<f32>( ( - nodeConst9.x ), nodeConst9.yz ), ( ( object.nodeUniform11 * nodeConst10 ) * ( 2.0 - nodeConst10 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar2.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform12 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar3 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	let nodeConst11 = fn128( normalView, positionViewDirection, SheenRoughness );
	sheenSpecularIndirect = ( sheenSpecularIndirect + ( ( ( irradiance * Sheen ) * vec3<f32>( nodeConst11 ) ) * vec3<f32>( 0.3183098861837907 ) ) );
	nodeVar3 = ( nodeVar3 * vec3<f32>( ( 1.0 - ( max( max( Sheen.x, Sheen.y ), Sheen.z ) * nodeConst11 ) ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar3 );
	sheenSpecularIndirect = ( sheenSpecularIndirect + ( ( ( iblIrradiance * Sheen ) * vec3<f32>( fn128( normalView, positionViewDirection, SheenRoughness ) ) ) * vec3<f32>( 0.3183098861837907 ) ) );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst12 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst12 );
	let nodeConst13 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst14 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst12 * nodeConst13 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst14 ) * nodeConst13 ) ) ) * vec3<f32>( nodeConst14 ) ) );
	let nodeConst15 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar4 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst15 ) );
	nodeVar5 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst15 );
	let nodeConst16 = ( 1.0 - ( max( max( Sheen.x, Sheen.y ), Sheen.z ) * fn128( normalView, positionViewDirection, SheenRoughness ) ) );
	nodeVar4 = ( nodeVar4 * vec3<f32>( nodeConst16 ) );
	nodeVar5 = ( nodeVar5 * vec3<f32>( nodeConst16 ) );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar4 );
	indirectDiffuse = ( indirectDiffuse + nodeVar5 );
	ambientOcclusion = 1.0;
	sheenSpecularIndirect = ( sheenSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	outgoingLight = ( ( outgoingLight + sheenSpecularDirect ) + sheenSpecularIndirect );
	let nodeConst17 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst17;

	// result

	output.color = nodeConst17;

	return output;

}
