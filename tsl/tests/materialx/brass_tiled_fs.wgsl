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
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform3_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform3 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform12 : texture_cube<f32>;

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32
};
@binding( 2 ) @group( 1 )
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
var<private> nodeVar0 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> Clearcoat : f32;
var<private> ClearcoatRoughness : f32;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> clearcoatRadiance : vec3<f32>;
var<private> clearcoatSpecularDirect : vec3<f32>;
var<private> clearcoatSpecularIndirect : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> clearcoatNormalView : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar7 : vec2<f32>;
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
fn fn105 ( color : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<bool>;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : vec3<f32>;

	nodeVar0 = color;
	nodeVar1 = ( nodeVar0 > vec3<f32>( 0.04045, 0.04045, 0.04045 ) );
	nodeVar2 = ( nodeVar0 / vec3<f32>( 12.92 ) );
	nodeVar3 = pow( ( max( ( nodeVar0 + vec3<f32>( 0.055, 0.055, 0.055 ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) / vec3<f32>( 1.055 ) ), vec3<f32>( 2.4, 2.4, 2.4 ) );

	return mix( nodeVar2, nodeVar3, vec3<f32>( nodeVar1 ) );

}




@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = ( ( vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) ) * vec2<f32>( 1.0, 1.0 ) ) + vec2<f32>( 0.0, 0.0 ) );
	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( vec2<f32>( nodeConst0[ 0u ], ( 1.0 - nodeConst0[ 1u ] ) ), 1.0 ) ).xy );
	DiffuseColor = vec4<f32>( ( ( vec3<f32>( 1.0 ) * vec3<f32>( 1.0, 1.0, 1.0 ) ) * fn105( nodeVar0.xyz ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	DiffuseColor.w = 1.0;
	let nodeConst1 = 1.0;
	Metalness = nodeConst1;
	let nodeConst2 = ( ( vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) ) * vec2<f32>( 1.0, 1.0 ) ) + vec2<f32>( 0.0, 0.0 ) );
	nodeVar1 = textureSample( nodeUniform3, nodeUniform3_sampler, ( object.nodeUniform4 * vec3<f32>( vec2<f32>( nodeConst2[ 0u ], ( 1.0 - nodeConst2[ 1u ] ) ), 1.0 ) ).xy );
	let nodeConst3 = nodeVar1.x;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeConst3, 0.045 ) + max( max( nodeConst4.x, nodeConst4.y ), nodeConst4.z ) ), 1.0 );
	IOR = object.nodeUniform7;
	let nodeConst5 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	let nodeConst6 = 0.0;
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst5 * nodeConst5 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( nodeConst6 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( nodeConst6, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeConst1 ) ) );
	Clearcoat = 1.0;
	let nodeConst7 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	ClearcoatRoughness = min( ( max( nodeConst3, 0.045 ) + max( max( nodeConst7.x, nodeConst7.y ), nodeConst7.z ) ), 1.0 );
	EmissiveColor = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	clearcoatRadiance = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst8 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst8 );
	let nodeConst9 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst10 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst8 * nodeConst9 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst10 ) * nodeConst9 ) ) ) * vec3<f32>( nodeConst10 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst11 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst12 = ( object.nodeUniform13 * vec4<f32>( nodeConst11, 1.0 ) );
	let nodeConst13 = clamp( Roughness, 0.0, 1.0 );
	nodeVar3 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, vec3<f32>( ( - nodeConst12.x ), nodeConst12.yz ), ( ( object.nodeUniform15 * nodeConst13 ) * ( 2.0 - nodeConst13 ) ) );
	radiance = ( radiance + ( nodeVar3.xyz * vec3<f32>( object.nodeUniform16 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst14 = ( object.nodeUniform13 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst15 = clamp( 1.0, 0.0, 1.0 );
	nodeVar4 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, vec3<f32>( ( - nodeConst14.x ), nodeConst14.yz ), ( ( object.nodeUniform15 * nodeConst15 ) * ( 2.0 - nodeConst15 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar4.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform16 ) ) );
	clearcoatNormalView = NORMAL_normalView;
	let nodeConst16 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), clearcoatNormalView ), clearcoatNormalView, ( ( ( ClearcoatRoughness * ClearcoatRoughness ) * ClearcoatRoughness ) * ClearcoatRoughness ) ) ), 0.0 ) ).xyz );
	let nodeConst17 = ( object.nodeUniform13 * vec4<f32>( nodeConst16, 1.0 ) );
	let nodeConst18 = clamp( ClearcoatRoughness, 0.0, 1.0 );
	nodeVar5 = textureSampleLevel( nodeUniform12, nodeUniform12_sampler, vec3<f32>( ( - nodeConst17.x ), nodeConst17.yz ), ( ( object.nodeUniform15 * nodeConst18 ) * ( 2.0 - nodeConst18 ) ) );
	clearcoatRadiance = ( clearcoatRadiance + ( nodeVar5.xyz * vec3<f32>( object.nodeUniform16 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar6 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar6 );
	nodeVar7 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( ClearcoatRoughness, clamp( dot( clearcoatNormalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	clearcoatSpecularIndirect = ( clearcoatSpecularIndirect + ( clearcoatRadiance * ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( nodeVar7.x ) ) + vec3<f32>( ( 1.0 * nodeVar7.y ) ) ) ) );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst19 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst19 );
	let nodeConst20 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst21 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst19 * nodeConst20 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst21 ) * nodeConst20 ) ) ) * vec3<f32>( nodeConst21 ) ) );
	let nodeConst22 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar8 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst22 ) );
	nodeVar9 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst22 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar8 );
	indirectDiffuse = ( indirectDiffuse + nodeVar9 );
	ambientOcclusion = 1.0;
	clearcoatSpecularIndirect = ( clearcoatSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst23 = clamp( dot( clearcoatNormalView, positionViewDirection ), 0.0, 1.0 );
	let nodeConst24 = exp2( ( ( ( nodeConst23 * -5.55473 ) - 6.98316 ) * nodeConst23 ) );
	outgoingLight = ( ( outgoingLight * ( vec3<f32>( 1.0 ) - ( vec3<f32>( Clearcoat ) * ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( ( 1.0 - nodeConst24 ) ) ) + vec3<f32>( ( 1.0 * nodeConst24 ) ) ) ) ) ) + ( ( clearcoatSpecularDirect + clearcoatSpecularIndirect ) * vec3<f32>( Clearcoat ) ) );
	let nodeConst25 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst25;

	// result

	output.color = nodeConst25;

	return output;

}
