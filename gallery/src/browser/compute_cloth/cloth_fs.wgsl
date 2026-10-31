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
@binding( 1 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform17_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform17 : texture_cube<f32>;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32
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
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
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
fn main( @location( 0 ) nodeVarying3 : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform1, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	Metalness = object.nodeUniform3;
	Roughness = min( ( max( object.nodeUniform4, 0.045 ) + 0.0 ), 1.0 );
	IOR = object.nodeUniform5;
	let nodeConst0 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst0 * nodeConst0 ) ) * object.nodeUniform6 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform7 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform7, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform3 ) ) );
	Sheen = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	SheenRoughness = clamp( object.nodeUniform10, 0.0001, 1.0 );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	sheenSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	sheenSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	normalView = nodeVarying3;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst4 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst5 = ( object.nodeUniform18 * vec4<f32>( nodeConst4, 1.0 ) );
	let nodeConst6 = clamp( Roughness, 0.0, 1.0 );
	nodeVar5 = textureSampleLevel( nodeUniform17, nodeUniform17_sampler, vec3<f32>( ( - nodeConst5.x ), nodeConst5.yz ), ( ( object.nodeUniform20 * nodeConst6 ) * ( 2.0 - nodeConst6 ) ) );
	radiance = ( radiance + ( nodeVar5.xyz * vec3<f32>( object.nodeUniform21 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst7 = ( object.nodeUniform18 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst8 = clamp( 1.0, 0.0, 1.0 );
	nodeVar6 = textureSampleLevel( nodeUniform17, nodeUniform17_sampler, vec3<f32>( ( - nodeConst7.x ), nodeConst7.yz ), ( ( object.nodeUniform20 * nodeConst8 ) * ( 2.0 - nodeConst8 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar6.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform21 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar7 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	let nodeConst9 = fn128( normalView, positionViewDirection, SheenRoughness );
	sheenSpecularIndirect = ( sheenSpecularIndirect + ( ( ( irradiance * Sheen ) * vec3<f32>( nodeConst9 ) ) * vec3<f32>( 0.3183098861837907 ) ) );
	nodeVar7 = ( nodeVar7 * vec3<f32>( ( 1.0 - ( max( max( Sheen.x, Sheen.y ), Sheen.z ) * nodeConst9 ) ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar7 );
	sheenSpecularIndirect = ( sheenSpecularIndirect + ( ( ( iblIrradiance * Sheen ) * vec3<f32>( fn128( normalView, positionViewDirection, SheenRoughness ) ) ) * vec3<f32>( 0.3183098861837907 ) ) );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst10 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst10 );
	let nodeConst11 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst12 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst10 * nodeConst11 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst12 ) * nodeConst11 ) ) ) * vec3<f32>( nodeConst12 ) ) );
	let nodeConst13 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar8 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst13 ) );
	nodeVar9 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst13 );
	let nodeConst14 = ( 1.0 - ( max( max( Sheen.x, Sheen.y ), Sheen.z ) * fn128( normalView, positionViewDirection, SheenRoughness ) ) );
	nodeVar8 = ( nodeVar8 * vec3<f32>( nodeConst14 ) );
	nodeVar9 = ( nodeVar9 * vec3<f32>( nodeConst14 ) );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar8 );
	indirectDiffuse = ( indirectDiffuse + nodeVar9 );
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
	let nodeConst15 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst15;

	// result

	output.color = nodeConst15;

	return output;

}
