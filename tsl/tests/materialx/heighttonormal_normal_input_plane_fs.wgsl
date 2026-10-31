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
@binding( 3 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform14 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat3x3<f32>,
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
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
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
var<private> directDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes


@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 1.0 ) * vec3<f32>( 0.8, 0.8, 0.8 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.2, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	IOR = object.nodeUniform4;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform5 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform5, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	let nodeConst2 = vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) );
	let nodeConst3 = vec2<f32>( dpdx( nodeConst2.x ), - dpdy( nodeConst2.x ) );
	let nodeConst4 = vec2<f32>( dpdx( nodeConst2.y ), - dpdy( nodeConst2.y ) );
	let nodeConst5 = vec2<f32>( nodeVarying6[ 0u ], ( 1.0 - nodeVarying6[ 1u ] ) );
	nodeVar0 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform12 * vec3<f32>( vec2<f32>( nodeConst5[ 0u ], ( 1.0 - nodeConst5[ 1u ] ) ), 1.0 ) ).xy );
	let nodeConst6 = nodeVar0.x;
	let nodeConst7 = ( ( vec2<f32>( dpdx( nodeConst6 ), - dpdy( nodeConst6 ) ) * vec2<f32>( 2.0 ) ) * vec2<f32>( 0.0625 ) );
	let nodeConst8 = cross( vec3<f32>( nodeConst3.x, nodeConst4.x, nodeConst7.x ), vec3<f32>( nodeConst3.y, nodeConst4.y, nodeConst7.y ) );
	let nodeConst9 = mix( nodeConst8, vec3<f32>( 0.0, 0.0, 1.0 ), f32( ( dot( nodeConst8, nodeConst8 ) < 1e-12 ) ) );
	normalView = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform10 * ( ( normalize( mix( nodeConst9, ( nodeConst9 * vec3<f32>( -1.0 ) ), f32( ( nodeConst9.z < 0.0 ) ) ) ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ) ), 0.0 ) ).xyz );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst10 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst10 );
	let nodeConst11 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst12 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst10 * nodeConst11 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst12 ) * nodeConst11 ) ) ) * vec3<f32>( nodeConst12 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst13 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst14 = ( object.nodeUniform15 * vec4<f32>( nodeConst13, 1.0 ) );
	let nodeConst15 = clamp( Roughness, 0.0, 1.0 );
	nodeVar2 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec3<f32>( ( - nodeConst14.x ), nodeConst14.yz ), ( ( object.nodeUniform17 * nodeConst15 ) * ( 2.0 - nodeConst15 ) ) );
	radiance = ( radiance + ( nodeVar2.xyz * vec3<f32>( object.nodeUniform18 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst16 = ( object.nodeUniform15 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst17 = clamp( 1.0, 0.0, 1.0 );
	nodeVar3 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec3<f32>( ( - nodeConst16.x ), nodeConst16.yz ), ( ( object.nodeUniform17 * nodeConst17 ) * ( 2.0 - nodeConst17 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar3.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform18 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar4 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar4 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst18 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst18 );
	let nodeConst19 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst20 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst18 * nodeConst19 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst20 ) * nodeConst19 ) ) ) * vec3<f32>( nodeConst20 ) ) );
	let nodeConst21 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar5 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst21 ) );
	nodeVar6 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst21 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar5 );
	indirectDiffuse = ( indirectDiffuse + nodeVar6 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst22 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst22;

	// result

	output.color = nodeConst22;

	return output;

}
