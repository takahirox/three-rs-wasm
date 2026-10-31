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
@binding( 4 ) @group( 1 ) var nodeUniform4 : texture_cube<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform19 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform26_sampler : sampler;
@binding( 12 ) @group( 1 ) var nodeUniform26 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec2<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : f32,
	nodeUniform20 : mat3x3<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : f32,
	nodeUniform27 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform34 : vec3<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform30 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	cameraProjectionMatrixInverse : mat4x4<f32>,
	nodeUniform33 : vec3<f32>,
	nodeUniform10 : vec2<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> positionView : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> reflectVector : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar5 : vec4<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> indirectDiffuse : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn44 ( lightDistance : f32, cutoffDistance : f32, decayExponent : f32 ) -> f32 {

	var nodeVar0 : f32;


	if ( ( cutoffDistance > 0.0 ) ) {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		let nodeConst1 = ( lightDistance / cutoffDistance );
		let nodeConst2 = clamp( ( 1.0 - ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * nodeConst1 ) ), 0.0, 1.0 );
		nodeVar0 = ( nodeConst0 * ( nodeConst2 * nodeConst2 ) );

	} else {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		nodeVar0 = nodeConst0;

	}


	return nodeVar0;

}




@fragment
fn main( @location( 0 ) nodeVarying2 : vec2<f32>,
	@location( 1 ) nodeVarying3 : f32,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_clipSpace : vec4<f32>,
	@location( 4 ) v_normalViewGeometry : vec3<f32>,
	@location( 5 ) nodeVarying8 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( mix( nodeVarying8, ( nodeVarying2 / vec2<f32>( nodeVarying3 ) ), object.nodeUniform3 ), 1.0 ) ).xy, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	let nodeConst2 = ( render.cameraProjectionMatrixInverse * v_clipSpace );
	positionView = ( nodeConst2.xyz / vec3<f32>( nodeConst2.w ) );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	let nodeConst3 = cross( - dpdy( positionView ), NORMAL_normalView );
	let nodeConst4 = dpdx( nodeVarying8 );
	let nodeConst5 = cross( NORMAL_normalView, dpdx( positionView ) );
	let nodeConst6 = - dpdy( nodeVarying8 );
	let nodeConst7 = ( ( nodeConst3 * vec3<f32>( nodeConst4.x ) ) + ( nodeConst5 * vec3<f32>( nodeConst6.x ) ) );
	let nodeConst8 = ( ( nodeConst3 * vec3<f32>( nodeConst4.y ) ) + ( nodeConst5 * vec3<f32>( nodeConst6.y ) ) );
	let nodeConst9 = max( dot( nodeConst7, nodeConst7 ), dot( nodeConst8, nodeConst8 ) );

	if ( ( nodeConst9 == 0.0 ) ) {

		nodeVar1 = 0.0;

	} else {

		nodeVar1 = inverseSqrt( nodeConst9 );

	}

	tangentViewFrame = ( nodeConst7 * vec3<f32>( nodeVar1 ) );
	NORMAL_tangentView = tangentViewFrame;
	bitangentViewFrame = ( nodeConst8 * vec3<f32>( nodeVar1 ) );
	NORMAL_bitangentView = bitangentViewFrame;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar2 = textureSample( nodeUniform12, nodeUniform12_sampler, ( object.nodeUniform13 * vec3<f32>( nodeVarying8, 1.0 ) ).xy );
	let nodeConst10 = ( ( nodeVar2 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeConst10.xy * object.nodeUniform14 ), nodeConst10.z ) ) );
	reflectVector = normalize( ( render.cameraWorldMatrix * vec4<f32>( reflect( ( - positionViewDirection ), normalView ), 0.0 ) ).xyz );
	nodeVar3 = textureSampleLevel( nodeUniform4, nodeUniform4_sampler, vec3<f32>( ( - reflectVector.x ), reflectVector.yz ), 0.0 );
	nodeVar4 = textureSampleLevel( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( mix( nodeVarying8, ( nodeVarying2 / vec2<f32>( nodeVarying3 ) ), object.nodeUniform3 ), 1.0 ) ).xy, 0.0 );
	DiffuseColor = mix( ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 ), nodeVar3, ( object.nodeUniform15 * nodeVar4.z ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform18 );
	DiffuseColor.w = 1.0;
	nodeVar5 = textureSampleLevel( nodeUniform19, nodeUniform19_sampler, ( object.nodeUniform20 * vec3<f32>( mix( nodeVarying8, ( nodeVarying2 / vec2<f32>( nodeVarying3 ) ), object.nodeUniform3 ), 1.0 ) ).xy, 0.0 );
	AmbientOcclusion = ( ( ( nodeVar5.x - 1.0 ) * object.nodeUniform21 ) + 1.0 );
	Shininess = max( object.nodeUniform22, 0.0001 );
	SpecularColor = object.nodeUniform23;
	nodeVar6 = textureSampleLevel( nodeUniform26, nodeUniform26_sampler, ( object.nodeUniform27 * vec3<f32>( mix( nodeVarying8, ( nodeVarying2 / vec2<f32>( nodeVarying3 ) ), object.nodeUniform3 ), 1.0 ) ).xy, 0.0 );
	EmissiveColor = ( vec4<f32>( ( object.nodeUniform24 * vec3<f32>( object.nodeUniform25 ) ), 1.0 ) * nodeVar6 ).xyz;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform28 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst11 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform30 - render.nodeUniform31 ), 0.0 ) ).xyz );
	let nodeConst12 = ( vec3<f32>( clamp( dot( normalView, nodeConst11 ), 0.0, 1.0 ) ) * render.nodeUniform32 );
	directDiffuse = ( directDiffuse + ( nodeConst12 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst13 = normalize( ( nodeConst11 + positionViewDirection ) );
	let nodeConst14 = clamp( dot( positionViewDirection, nodeConst13 ), 0.0, 1.0 );
	let nodeConst15 = exp2( ( ( ( nodeConst14 * -5.55473 ) - 6.98316 ) * nodeConst14 ) );
	directSpecular = ( directSpecular + ( ( nodeConst12 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst15 ) ) ) + vec3<f32>( ( 1.0 * nodeConst15 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst13 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	let nodeConst16 = ( render.nodeUniform33 - positionView );
	let nodeConst17 = normalize( nodeConst16 );
	let nodeConst18 = ( vec3<f32>( clamp( dot( normalView, nodeConst17 ), 0.0, 1.0 ) ) * ( render.nodeUniform34 * vec3<f32>( fn44( length( nodeConst16 ), render.nodeUniform35, render.nodeUniform36 ) ) ) );
	directDiffuse = ( directDiffuse + ( nodeConst18 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	let nodeConst19 = normalize( ( nodeConst17 + positionViewDirection ) );
	let nodeConst20 = clamp( dot( positionViewDirection, nodeConst19 ), 0.0, 1.0 );
	let nodeConst21 = exp2( ( ( ( nodeConst20 * -5.55473 ) - 6.98316 ) * nodeConst20 ) );
	directSpecular = ( directSpecular + ( ( nodeConst18 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst21 ) ) ) + vec3<f32>( ( 1.0 * nodeConst21 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst19 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	ambientOcclusion = 1.0;
	ambientOcclusion = ( ambientOcclusion * AmbientOcclusion );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + ( vec4<f32>( irradiance, 1.0 ) * ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) ) ) ).xyz;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst22 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst22;

	// result

	output.color = nodeConst22;

	return output;

}
