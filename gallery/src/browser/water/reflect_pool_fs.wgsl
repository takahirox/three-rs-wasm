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
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform23_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform23 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : f32,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform21 : mat3x3<f32>,
	nodeUniform22 : vec2<f32>,
	nodeUniform24 : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32
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
var<private> nodeVar0 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> irradiance : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
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
	@location( 3 ) nodeVarying6 : vec2<f32>,
	@builtin( front_facing ) isFront : bool ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	AmbientOcclusion = ( ( ( nodeVar1.x - 1.0 ) * object.nodeUniform6 ) + 1.0 );
	nodeVar2 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform10 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform9 * nodeVar3.y ), 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	IOR = object.nodeUniform13;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * object.nodeUniform14 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform15 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform15, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	EmissiveColor = ( object.nodeUniform16 * vec3<f32>( object.nodeUniform17 ) );
	let nodeConst2 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeConst2 ) );
	let nodeConst3 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	let nodeConst4 = dpdx( nodeVarying6 );
	let nodeConst5 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	let nodeConst6 = - dpdy( nodeVarying6 );
	let nodeConst7 = ( ( nodeConst3 * vec3<f32>( nodeConst4.x ) ) + ( nodeConst5 * vec3<f32>( nodeConst6.x ) ) );
	let nodeConst8 = ( ( nodeConst3 * vec3<f32>( nodeConst4.y ) ) + ( nodeConst5 * vec3<f32>( nodeConst6.y ) ) );
	let nodeConst9 = max( dot( nodeConst7, nodeConst7 ), dot( nodeConst8, nodeConst8 ) );

	if ( ( nodeConst9 == 0.0 ) ) {

		nodeVar4 = 0.0;

	} else {

		nodeVar4 = inverseSqrt( nodeConst9 );

	}

	tangentViewFrame = ( nodeConst7 * vec3<f32>( nodeVar4 ) );
	NORMAL_tangentView = ( tangentViewFrame * vec3<f32>( nodeConst2 ) );
	bitangentViewFrame = ( nodeConst8 * vec3<f32>( nodeVar4 ) );
	NORMAL_bitangentView = ( bitangentViewFrame * vec3<f32>( nodeConst2 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar5 = textureSample( nodeUniform20, nodeUniform20_sampler, ( object.nodeUniform21 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	let nodeConst10 = ( ( nodeVar5 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeConst10.xy * object.nodeUniform22 ), nodeConst10.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = textureSample( nodeUniform18, nodeUniform18_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar6;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst11 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst11 );
	let nodeConst12 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst13 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst11 * nodeConst12 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst13 ) * nodeConst12 ) ) ) * vec3<f32>( nodeConst13 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst14 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst15 = ( object.nodeUniform24 * vec4<f32>( nodeConst14, 1.0 ) );
	let nodeConst16 = clamp( Roughness, 0.0, 1.0 );
	nodeVar7 = textureSampleLevel( nodeUniform23, nodeUniform23_sampler, vec3<f32>( ( - nodeConst15.x ), nodeConst15.yz ), ( ( object.nodeUniform26 * nodeConst16 ) * ( 2.0 - nodeConst16 ) ) );
	radiance = ( radiance + ( nodeVar7.xyz * vec3<f32>( object.nodeUniform27 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst17 = ( object.nodeUniform24 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst18 = clamp( 1.0, 0.0, 1.0 );
	nodeVar8 = textureSampleLevel( nodeUniform23, nodeUniform23_sampler, vec3<f32>( ( - nodeConst17.x ), nodeConst17.yz ), ( ( object.nodeUniform26 * nodeConst18 ) * ( 2.0 - nodeConst18 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar8.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform27 ) ) );
	ambientOcclusion = 1.0;
	ambientOcclusion = ( ambientOcclusion * AmbientOcclusion );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar9 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst19 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst19 );
	let nodeConst20 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst21 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst19 * nodeConst20 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst21 ) * nodeConst20 ) ) ) * vec3<f32>( nodeConst21 ) ) );
	let nodeConst22 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar10 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst22 ) );
	nodeVar11 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst22 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar10 );
	indirectDiffuse = ( indirectDiffuse + nodeVar11 );
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst23 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst23;

	// result

	output.color = nodeConst23;

	return output;

}
