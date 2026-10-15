// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct renderStruct {
	nodeUniform3 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform6 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : f32;
var<private> Output : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar0 = ( - normalWorld.y );
	nodeVar1 = ( ( nodeVar0 * 0.5 ) + 0.5 );
	nodeVar2 = vec2<f32>( ( atan2( normalWorld.x, normalWorld.z ) * 50.0 ), ( asin( nodeVar0 ) * 50.0 ) );
	nodeVar3 = fract( ( sin( dot( floor( nodeVar2 ), vec2<f32>( 12.9898, 78.233 ) ) ) * 43758.5453 ) );
	nodeVar4 = ( fract( nodeVar2 ) - vec2<f32>( 0.5 ) );
	nodeVar5 = length( nodeVar4 );
	DiffuseColor = ( vec4<f32>( mix( mix( vec3<f32>( 0.13286832154414627, 0.033104766565152086, 0.015996293361446288 ), mix( vec3<f32>( 0.033104766565152086, 0.0, 0.13286832154414627 ), vec3<f32>( 0.0, 0.0, 0.033104766565152086 ), smoothstep( 0.4, 0.9, nodeVar1 ) ), smoothstep( 0.0, 0.4, nodeVar1 ) ), mix( vec3<f32>( 1.0, 1.0, 0.95 ), vec3<f32>( 0.8, 0.9, 1.0 ), nodeVar3 ), clamp( ( ( ( step( 0.85, nodeVar3 ) * smoothstep( -0.2, 0.1, nodeVar0 ) ) * ( ( smoothstep( 0.08, 0.0, nodeVar5 ) + ( smoothstep( 0.25, 0.0, nodeVar5 ) * 0.4 ) ) + ( ( ( smoothstep( 0.15, 0.0, abs( nodeVar4.x ) ) * smoothstep( 0.4, 0.0, abs( nodeVar4.y ) ) ) + ( smoothstep( 0.15, 0.0, abs( nodeVar4.y ) ) * smoothstep( 0.4, 0.0, abs( nodeVar4.x ) ) ) ) * 0.3 ) ) ) * ( ( nodeVar3 * 0.6 ) + 0.4 ) ), 0.0, 1.0 ) ), 1.0 ) * vec4<f32>( render.nodeUniform3 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	nodeVar6 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar6;

	// result

	output.color = nodeVar6;

	return output;

}
