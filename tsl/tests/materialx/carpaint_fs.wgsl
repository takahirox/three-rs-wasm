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
@binding( 1 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform10 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : mat4x4<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : f32
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
var<private> Clearcoat : f32;
var<private> ClearcoatRoughness : f32;
var<private> nodeVar0 : vec2<f32>;
var<private> Anisotropy : f32;
var<private> AlphaT : f32;
var<private> AnisotropyT : vec3<f32>;
var<private> tangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> bitangentView : vec3<f32>;
var<private> TBNViewMatrix : mat3x3<f32>;
var<private> AnisotropyB : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> clearcoatRadiance : vec3<f32>;
var<private> clearcoatSpecularDirect : vec3<f32>;
var<private> clearcoatSpecularIndirect : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> clearcoatNormalView : vec3<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes


@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( ( vec3<f32>( 0.5 ) * vec3<f32>( 0.1037792, 0.59212029, 0.85064936 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.4, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	IOR = object.nodeUniform4;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform5 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform5, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	Clearcoat = 1.0;
	let nodeConst2 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	ClearcoatRoughness = min( ( max( 0.0, 0.045 ) + max( max( nodeConst2.x, nodeConst2.y ), nodeConst2.z ) ), 1.0 );
	nodeVar0 = ( vec2<f32>( cos( 0.0 ), sin( 0.0 ) ) * vec2<f32>( 0.5 ) );
	Anisotropy = length( nodeVar0 );

	if ( ( Anisotropy == 0.0 ) ) {

		nodeVar0 = vec2<f32>( 1.0, 0.0 );
		

	} else {

		nodeVar0 = ( nodeVar0 / vec2<f32>( Anisotropy ) );
		Anisotropy = clamp( Anisotropy, 0.0, 1.0 );
		

	}

	AlphaT = mix( ( Roughness * Roughness ), 1.0, ( Anisotropy * Anisotropy ) );
	tangentView = normalize( v_tangentView );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	bitangentView = normalize( ( cross( normalView, tangentView ) * vec3<f32>( nodeVarying7.w ) ) );
	TBNViewMatrix = mat3x3<f32>( tangentView, bitangentView, normalView );
	AnisotropyT = ( ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar0.x ) ) + ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar0.y ) ) );
	AnisotropyB = ( ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar0.x ) ) - ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar0.y ) ) );
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	clearcoatRadiance = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst3 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst3 );
	let nodeConst4 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst5 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst3 * nodeConst4 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst5 ) * nodeConst4 ) ) ) * vec3<f32>( nodeConst5 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst6 = ( 1.0 - ( Anisotropy * ( 1.0 - Roughness ) ) );
	let nodeConst7 = ( nodeConst6 * nodeConst6 );
	let nodeConst8 = normalize( mix( normalize( cross( cross( AnisotropyB, positionViewDirection ), AnisotropyB ) ), normalView, ( nodeConst7 * nodeConst7 ) ) );
	let nodeConst9 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), nodeConst8 ), nodeConst8, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst10 = ( object.nodeUniform11 * vec4<f32>( nodeConst9, 1.0 ) );
	let nodeConst11 = clamp( Roughness, 0.0, 1.0 );
	nodeVar2 = textureSampleLevel( nodeUniform10, nodeUniform10_sampler, vec3<f32>( ( - nodeConst10.x ), nodeConst10.yz ), ( ( object.nodeUniform13 * nodeConst11 ) * ( 2.0 - nodeConst11 ) ) );
	radiance = ( radiance + ( nodeVar2.xyz * vec3<f32>( object.nodeUniform14 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst12 = ( object.nodeUniform11 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst13 = clamp( 1.0, 0.0, 1.0 );
	nodeVar3 = textureSampleLevel( nodeUniform10, nodeUniform10_sampler, vec3<f32>( ( - nodeConst12.x ), nodeConst12.yz ), ( ( object.nodeUniform13 * nodeConst13 ) * ( 2.0 - nodeConst13 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar3.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform14 ) ) );
	clearcoatNormalView = NORMAL_normalView;
	let nodeConst14 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), clearcoatNormalView ), clearcoatNormalView, ( ( ( ClearcoatRoughness * ClearcoatRoughness ) * ClearcoatRoughness ) * ClearcoatRoughness ) ) ), 0.0 ) ).xyz );
	let nodeConst15 = ( object.nodeUniform11 * vec4<f32>( nodeConst14, 1.0 ) );
	let nodeConst16 = clamp( ClearcoatRoughness, 0.0, 1.0 );
	nodeVar4 = textureSampleLevel( nodeUniform10, nodeUniform10_sampler, vec3<f32>( ( - nodeConst15.x ), nodeConst15.yz ), ( ( object.nodeUniform13 * nodeConst16 ) * ( 2.0 - nodeConst16 ) ) );
	clearcoatRadiance = ( clearcoatRadiance + ( nodeVar4.xyz * vec3<f32>( object.nodeUniform14 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar5 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar5 );
	nodeVar6 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( ClearcoatRoughness, clamp( dot( clearcoatNormalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	clearcoatSpecularIndirect = ( clearcoatSpecularIndirect + ( clearcoatRadiance * ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( nodeVar6.x ) ) + vec3<f32>( ( 1.0 * nodeVar6.y ) ) ) ) );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst17 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst17 );
	let nodeConst18 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst19 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst17 * nodeConst18 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst19 ) * nodeConst18 ) ) ) * vec3<f32>( nodeConst19 ) ) );
	let nodeConst20 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar7 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst20 ) );
	nodeVar8 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst20 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar7 );
	indirectDiffuse = ( indirectDiffuse + nodeVar8 );
	ambientOcclusion = 1.0;
	clearcoatSpecularIndirect = ( clearcoatSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst21 = clamp( dot( clearcoatNormalView, positionViewDirection ), 0.0, 1.0 );
	let nodeConst22 = exp2( ( ( ( nodeConst21 * -5.55473 ) - 6.98316 ) * nodeConst21 ) );
	outgoingLight = ( ( outgoingLight * ( vec3<f32>( 1.0 ) - ( vec3<f32>( Clearcoat ) * ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( ( 1.0 - nodeConst22 ) ) ) + vec3<f32>( ( 1.0 * nodeConst22 ) ) ) ) ) ) + ( ( clearcoatSpecularDirect + clearcoatSpecularIndirect ) * vec3<f32>( Clearcoat ) ) );
	let nodeConst23 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst23;

	// result

	output.color = nodeConst23;

	return output;

}
