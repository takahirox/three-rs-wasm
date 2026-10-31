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
@binding( 1 ) @group( 1 ) var nodeUniform14_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform14 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform10 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform15 : f32,
	nodeUniform16 : vec2<f32>,
	nodeUniform17 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar0 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> indirectDiffuse : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	EmissiveColor = ( object.nodeUniform2 * vec3<f32>( object.nodeUniform3 ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst0 = ( render.nodeUniform11 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform12 ) ) ), 1.0 ) );
	let nodeConst1 = ( nodeConst0.xyz / vec3<f32>( nodeConst0.w ) );
	let nodeConst2 = vec3<f32>( nodeConst1.x, ( 1.0 - nodeConst1.y ), ( nodeConst1.z + render.nodeUniform13 ) );

	if ( ( ( ( ( ( nodeConst2.x >= 0.0 ) && ( nodeConst2.x <= 1.0 ) ) && ( nodeConst2.y >= 0.0 ) ) && ( nodeConst2.y <= 1.0 ) ) && ( nodeConst2.z <= 1.0 ) ) ) {

		let nodeConst3 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst4 = ( vec2<f32>( render.nodeUniform15 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform16 ) );
		let nodeConst5 = ( nodeConst2.xy + ( fn25( 0, 5, nodeConst3 ) * nodeConst4 ) );
		nodeVar1 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeConst5, nodeConst2.z );
		let nodeConst6 = ( nodeConst2.xy + ( fn25( 1, 5, nodeConst3 ) * nodeConst4 ) );
		nodeVar2 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeConst6, nodeConst2.z );
		let nodeConst7 = ( nodeConst2.xy + ( fn25( 2, 5, nodeConst3 ) * nodeConst4 ) );
		nodeVar3 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeConst7, nodeConst2.z );
		let nodeConst8 = ( nodeConst2.xy + ( fn25( 3, 5, nodeConst3 ) * nodeConst4 ) );
		nodeVar4 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeConst8, nodeConst2.z );
		let nodeConst9 = ( nodeConst2.xy + ( fn25( 4, 5, nodeConst3 ) * nodeConst4 ) );
		nodeVar5 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeConst9, nodeConst2.z );
		nodeVar0 = ( ( ( ( ( nodeVar1 + nodeVar2 ) + nodeVar3 ) + nodeVar4 ) + nodeVar5 ) * 0.2 );

	} else {

		nodeVar0 = 1.0;

	}

	nodeVar6 = mix( 1.0, nodeVar0, render.nodeUniform17 );
	directDiffuse = ( directDiffuse + ( ( vec3<f32>( clamp( dot( normalView, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform7 - render.nodeUniform8 ), 0.0 ) ).xyz ) ), 0.0, 1.0 ) ) * ( render.nodeUniform9 * vec3<f32>( nodeVar6 ) ) ) * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + ( vec4<f32>( irradiance, 1.0 ) * ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) ) ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst10 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst10;

	// result

	output.color = nodeConst10;

	return output;

}
