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
@binding( 1 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform14 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : f32,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32
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
var<private> DiffuseRoughness : f32;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
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
fn fn108 ( mu : f32, roughness : f32, A : f32 ) -> f32 {

	

	let nodeConst0 = ( 1.0 - mu );

	return ( A * ( ( roughness * ( nodeConst0 * ( ( nodeConst0 * ( ( nodeConst0 * ( ( nodeConst0 * 0.0714429953 ) - 0.332181442 ) ) + 0.491881867 ) ) + 0.0571085289 ) ) ) + 1.0 ) );

}


fn fn109 ( diffuseColor : vec3<f32>, roughness : f32, dotNV : f32 ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;


	if ( ( roughness <= 1e-7 ) ) {

		let nodeConst0 = clamp( diffuseColor, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );
		nodeVar0 = nodeConst0;

	} else {

		let nodeConst0 = clamp( diffuseColor, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );
		let nodeConst1 = ( 1.0 / ( ( roughness * 0.2877934092108062 ) + 1.0 ) );
		let nodeConst2 = fn108( clamp( dotNV, 0.0, 1.0 ), roughness, nodeConst1 );
		let nodeConst3 = ( nodeConst1 * ( ( roughness * 0.07248821245692394 ) + 1.0 ) );
		nodeVar0 = ( ( nodeConst0 * vec3<f32>( nodeConst2 ) ) + ( ( ( ( nodeConst0 * nodeConst0 ) * vec3<f32>( nodeConst3 ) ) / max( ( vec3<f32>( 1.0 ) - ( nodeConst0 * vec3<f32>( ( 1.0 - nodeConst3 ) ) ) ), vec3<f32>( 1e-7 ) ) ) * vec3<f32>( ( 1.0 - nodeConst2 ) ) ) );

	}


	return nodeVar0;

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
	IOR = object.nodeUniform6;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * object.nodeUniform7 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform8 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform8, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	DiffuseRoughness = clamp( object.nodeUniform9, 0.0, 1.0 );
	EmissiveColor = ( object.nodeUniform10 * vec3<f32>( object.nodeUniform11 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	let nodeConst2 = clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 );
	nodeVar0 = textureSample( nodeUniform12, nodeUniform12_sampler, vec2<f32>( Roughness, nodeConst2 ) ).xy;
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
	let nodeConst7 = ( object.nodeUniform15 * vec4<f32>( nodeConst6, 1.0 ) );
	let nodeConst8 = clamp( Roughness, 0.0, 1.0 );
	nodeVar1 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec3<f32>( ( - nodeConst7.x ), nodeConst7.yz ), ( ( object.nodeUniform17 * nodeConst8 ) * ( 2.0 - nodeConst8 ) ) );
	radiance = ( radiance + ( nodeVar1.xyz * vec3<f32>( object.nodeUniform18 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst9 = ( object.nodeUniform15 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst10 = clamp( 1.0, 0.0, 1.0 );
	nodeVar2 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec3<f32>( ( - nodeConst9.x ), nodeConst9.yz ), ( ( object.nodeUniform17 * nodeConst10 ) * ( 2.0 - nodeConst10 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar2.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform18 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst11 = fn109( DiffuseColor.xyz, DiffuseRoughness, nodeConst2 );
	nodeVar3 = ( ( irradiance * ( ( nodeConst11 * vec3<f32>( ( 1.0 - Metalness ) ) ) * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar3 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst12 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst12 );
	let nodeConst13 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst14 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst12 * nodeConst13 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst14 ) * nodeConst13 ) ) ) * vec3<f32>( nodeConst14 ) ) );
	let nodeConst15 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar4 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst15 ) );
	nodeVar5 = ( ( ( nodeConst11 * vec3<f32>( ( 1.0 - Metalness ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst15 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar4 );
	indirectDiffuse = ( indirectDiffuse + nodeVar5 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst16 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst16;

	// result

	output.color = nodeConst16;

	return output;

}
