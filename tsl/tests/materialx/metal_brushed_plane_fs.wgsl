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
@binding( 3 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform7 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : mat4x4<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32
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

	DiffuseColor = vec4<f32>( ( vec3<f32>( 1.0 ) * vec3<f32>( 0.5, 0.5, 0.5 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	let nodeConst0 = 1.0;
	Metalness = nodeConst0;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.25, 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	IOR = 1.52;
	let nodeConst2 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	let nodeConst3 = 0.0;
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst2 * nodeConst2 ) ) * vec3<f32>( 0.0, 0.0, 0.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( nodeConst3 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( nodeConst3, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeConst0 ) ) );
	let nodeConst4 = 0.0;
	nodeVar0 = ( vec2<f32>( cos( nodeConst4 ), sin( nodeConst4 ) ) * vec2<f32>( 0.65 ) );
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
	let nodeConst5 = cross( - dpdy( v_positionView ), normalView );
	let nodeConst6 = dpdx( nodeVarying6 );
	let nodeConst7 = cross( normalView, dpdx( v_positionView ) );
	let nodeConst8 = - dpdy( nodeVarying6 );
	let nodeConst9 = ( ( nodeConst5 * vec3<f32>( nodeConst6.x ) ) + ( nodeConst7 * vec3<f32>( nodeConst8.x ) ) );
	let nodeConst10 = ( ( nodeConst5 * vec3<f32>( nodeConst6.y ) ) + ( nodeConst7 * vec3<f32>( nodeConst8.y ) ) );
	let nodeConst11 = max( dot( nodeConst9, nodeConst9 ), dot( nodeConst10, nodeConst10 ) );

	if ( ( nodeConst11 == 0.0 ) ) {

		nodeVar1 = 0.0;

	} else {

		nodeVar1 = inverseSqrt( nodeConst11 );

	}

	tangentViewFrame = ( nodeConst9 * vec3<f32>( nodeVar1 ) );
	tangentView = tangentViewFrame;
	bitangentViewFrame = ( nodeConst10 * vec3<f32>( nodeVar1 ) );
	bitangentView = bitangentViewFrame;
	TBNViewMatrix = mat3x3<f32>( tangentView, bitangentView, normalView );
	AnisotropyT = ( ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar0.x ) ) + ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar0.y ) ) );
	AnisotropyB = ( ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar0.x ) ) - ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar0.y ) ) );
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst12 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst12 );
	let nodeConst13 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst14 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst12 * nodeConst13 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst14 ) * nodeConst13 ) ) ) * vec3<f32>( nodeConst14 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst15 = ( 1.0 - ( Anisotropy * ( 1.0 - Roughness ) ) );
	let nodeConst16 = ( nodeConst15 * nodeConst15 );
	let nodeConst17 = normalize( mix( normalize( cross( cross( AnisotropyB, positionViewDirection ), AnisotropyB ) ), normalView, ( nodeConst16 * nodeConst16 ) ) );
	let nodeConst18 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), nodeConst17 ), nodeConst17, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst19 = ( object.nodeUniform8 * vec4<f32>( nodeConst18, 1.0 ) );
	let nodeConst20 = clamp( Roughness, 0.0, 1.0 );
	nodeVar3 = textureSampleLevel( nodeUniform7, nodeUniform7_sampler, vec3<f32>( ( - nodeConst19.x ), nodeConst19.yz ), ( ( object.nodeUniform10 * nodeConst20 ) * ( 2.0 - nodeConst20 ) ) );
	radiance = ( radiance + ( nodeVar3.xyz * vec3<f32>( object.nodeUniform11 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst21 = ( object.nodeUniform8 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst22 = clamp( 1.0, 0.0, 1.0 );
	nodeVar4 = textureSampleLevel( nodeUniform7, nodeUniform7_sampler, vec3<f32>( ( - nodeConst21.x ), nodeConst21.yz ), ( ( object.nodeUniform10 * nodeConst22 ) * ( 2.0 - nodeConst22 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar4.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform11 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar5 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar5 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst23 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst23 );
	let nodeConst24 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst25 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst23 * nodeConst24 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst25 ) * nodeConst24 ) ) ) * vec3<f32>( nodeConst25 ) ) );
	let nodeConst26 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar6 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst26 ) );
	nodeVar7 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst26 );
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
	let nodeConst27 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst27;

	// result

	output.color = nodeConst27;

	return output;

}
