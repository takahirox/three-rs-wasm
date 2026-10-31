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
@binding( 1 ) @group( 1 ) var nodeUniform17_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform17 : texture_depth_2d_array;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform13 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar0 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> shadowValue : f32;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform6 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	let nodeConst0 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform10 - render.nodeUniform11 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst1 = ( render.nodeUniform14 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
	let nodeConst2 = ( nodeConst1.xyz / vec3<f32>( nodeConst1.w ) );
	let nodeConst3 = vec3<f32>( nodeConst2.x, ( 1.0 - nodeConst2.y ), ( nodeConst2.z + render.nodeUniform16 ) );
	let nodeConst4 = ( nodeConst3.xy * vec2<f32>( 2.0, 2.0 ) );
	let nodeConst5 = clamp( floor( nodeConst4 ), vec2<f32>( 0.0, 0.0 ), vec2<f32>( 1.0, 1.0 ) );
	let nodeConst6 = vec3<f32>( ( nodeConst4 - nodeConst5 ), nodeConst3.z );

	if ( ( ( ( ( ( nodeConst6.x >= 0.0 ) && ( nodeConst6.x <= 1.0 ) ) && ( nodeConst6.y >= 0.0 ) ) && ( nodeConst6.y <= 1.0 ) ) && ( nodeConst6.z <= 1.0 ) ) ) {

		nodeVar1 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst6.xy, i32( ( ( nodeConst5.y * 2.0 ) + nodeConst5.x ) ), nodeConst6.z );
		nodeVar0 = nodeVar1;

	} else {

		nodeVar0 = 1.0;

	}

	shadowValue = mix( 1.0, nodeVar0, render.nodeUniform18 );
	let nodeConst7 = ( vec3<f32>( clamp( dot( normalView, nodeConst0 ), 0.0, 1.0 ) ) * ( render.nodeUniform12 * vec3<f32>( shadowValue ) ) );
	directDiffuse = ( directDiffuse + ( nodeConst7 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	let nodeConst8 = normalize( ( nodeConst0 + positionViewDirection ) );
	let nodeConst9 = clamp( dot( positionViewDirection, nodeConst8 ), 0.0, 1.0 );
	let nodeConst10 = exp2( ( ( ( nodeConst9 * -5.55473 ) - 6.98316 ) * nodeConst9 ) );
	directSpecular = ( directSpecular + ( ( nodeConst7 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst10 ) ) ) + vec3<f32>( ( 1.0 * nodeConst10 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + ( vec4<f32>( irradiance, 1.0 ) * ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) ) ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar2 = vec4<f32>( mix( Output.xyz, render.nodeUniform19, smoothstep( render.nodeUniform20, render.nodeUniform21, ( - v_positionView.z ) ) ), Output.w );
	let nodeConst11 = nodeVar2;
	Output = nodeConst11;

	// result

	output.color = nodeConst11;

	return output;

}
