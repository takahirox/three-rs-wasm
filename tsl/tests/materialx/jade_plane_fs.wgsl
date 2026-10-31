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
@binding( 3 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform9 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32
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
var<private> nodeVar0 : vec2<f32>;
var<private> Anisotropy : f32;
var<private> AlphaT : f32;
var<private> AnisotropyT : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> bitangentView : vec3<f32>;
var<private> TBNViewMatrix : mat3x3<f32>;
var<private> AnisotropyB : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
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
var<private> directDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 0.5 ) * vec3<f32>( 0.0603, 0.4398, 0.1916 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.25, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	IOR = 2.418;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform4 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform4, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	nodeVar0 = ( vec2<f32>( cos( 0.0 ), sin( 0.0 ) ) * vec2<f32>( 0.5 ) );
	Anisotropy = length( nodeVar0 );

	if ( ( Anisotropy == 0.0 ) ) {

		nodeVar0 = vec2<f32>( 1.0, 0.0 );
		

	} else {

		nodeVar0 = ( nodeVar0 / vec2<f32>( Anisotropy ) );
		Anisotropy = clamp( Anisotropy, 0.0, 1.0 );
		

	}

	AlphaT = mix( ( Roughness * Roughness ), 1.0, ( Anisotropy * Anisotropy ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	let nodeConst2 = cross( - dpdy( v_positionView ), normalView );
	let nodeConst3 = dpdx( nodeVarying6 );
	let nodeConst4 = cross( normalView, dpdx( v_positionView ) );
	let nodeConst5 = - dpdy( nodeVarying6 );
	let nodeConst6 = ( ( nodeConst2 * vec3<f32>( nodeConst3.x ) ) + ( nodeConst4 * vec3<f32>( nodeConst5.x ) ) );
	let nodeConst7 = ( ( nodeConst2 * vec3<f32>( nodeConst3.y ) ) + ( nodeConst4 * vec3<f32>( nodeConst5.y ) ) );
	let nodeConst8 = max( dot( nodeConst6, nodeConst6 ), dot( nodeConst7, nodeConst7 ) );

	if ( ( nodeConst8 == 0.0 ) ) {

		nodeVar1 = 0.0;

	} else {

		nodeVar1 = inverseSqrt( nodeConst8 );

	}

	tangentViewFrame = ( nodeConst6 * vec3<f32>( nodeVar1 ) );
	tangentView = tangentViewFrame;
	bitangentViewFrame = ( nodeConst7 * vec3<f32>( nodeVar1 ) );
	bitangentView = bitangentViewFrame;
	TBNViewMatrix = mat3x3<f32>( tangentView, bitangentView, normalView );
	AnisotropyT = ( ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar0.x ) ) + ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar0.y ) ) );
	AnisotropyB = ( ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar0.x ) ) - ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar0.y ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst9 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst9 );
	let nodeConst10 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst11 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst9 * nodeConst10 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst11 ) * nodeConst10 ) ) ) * vec3<f32>( nodeConst11 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst12 = ( 1.0 - ( Anisotropy * ( 1.0 - Roughness ) ) );
	let nodeConst13 = ( nodeConst12 * nodeConst12 );
	let nodeConst14 = normalize( mix( normalize( cross( cross( AnisotropyB, positionViewDirection ), AnisotropyB ) ), normalView, ( nodeConst13 * nodeConst13 ) ) );
	let nodeConst15 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), nodeConst14 ), nodeConst14, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst16 = ( object.nodeUniform10 * vec4<f32>( nodeConst15, 1.0 ) );
	let nodeConst17 = clamp( Roughness, 0.0, 1.0 );
	nodeVar3 = textureSampleLevel( nodeUniform9, nodeUniform9_sampler, vec3<f32>( ( - nodeConst16.x ), nodeConst16.yz ), ( ( object.nodeUniform12 * nodeConst17 ) * ( 2.0 - nodeConst17 ) ) );
	radiance = ( radiance + ( nodeVar3.xyz * vec3<f32>( object.nodeUniform13 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst18 = ( object.nodeUniform10 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst19 = clamp( 1.0, 0.0, 1.0 );
	nodeVar4 = textureSampleLevel( nodeUniform9, nodeUniform9_sampler, vec3<f32>( ( - nodeConst18.x ), nodeConst18.yz ), ( ( object.nodeUniform12 * nodeConst19 ) * ( 2.0 - nodeConst19 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar4.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform13 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar5 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar5 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst20 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst20 );
	let nodeConst21 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst22 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst20 * nodeConst21 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst22 ) * nodeConst21 ) ) ) * vec3<f32>( nodeConst22 ) ) );
	let nodeConst23 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar6 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst23 ) );
	nodeVar7 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst23 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar6 );
	indirectDiffuse = ( indirectDiffuse + nodeVar7 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst24 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst24;

	// result

	output.color = nodeConst24;

	return output;

}
