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
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform10 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst0 = ( - normalWorld.y );
	let nodeConst1 = ( ( nodeConst0 * 0.5 ) + 0.5 );
	let nodeConst2 = vec2<f32>( ( atan2( normalWorld.x, normalWorld.z ) * 50.0 ), ( asin( nodeConst0 ) * 50.0 ) );
	let nodeConst3 = fract( ( sin( dot( floor( nodeConst2 ), vec2<f32>( 12.9898, 78.233 ) ) ) * 43758.5453 ) );
	let nodeConst4 = ( fract( nodeConst2 ) - vec2<f32>( 0.5 ) );
	let nodeConst5 = length( nodeConst4 );
	DiffuseColor = ( vec4<f32>( mix( mix( vec3<f32>( 0.13286832154414627, 0.033104766565152086, 0.015996293361446288 ), mix( vec3<f32>( 0.033104766565152086, 0.0, 0.13286832154414627 ), vec3<f32>( 0.0, 0.0, 0.033104766565152086 ), smoothstep( 0.4, 0.9, nodeConst1 ) ), smoothstep( 0.0, 0.4, nodeConst1 ) ), mix( vec3<f32>( 1.0, 1.0, 0.95 ), vec3<f32>( 0.8, 0.9, 1.0 ), nodeConst3 ), clamp( ( ( ( step( 0.85, nodeConst3 ) * smoothstep( -0.2, 0.1, nodeConst0 ) ) * ( ( smoothstep( 0.08, 0.0, nodeConst5 ) + ( smoothstep( 0.25, 0.0, nodeConst5 ) * 0.4 ) ) + ( ( ( smoothstep( 0.15, 0.0, abs( nodeConst4.x ) ) * smoothstep( 0.4, 0.0, abs( nodeConst4.y ) ) ) + ( smoothstep( 0.15, 0.0, abs( nodeConst4.y ) ) * smoothstep( 0.4, 0.0, abs( nodeConst4.x ) ) ) ) * 0.3 ) ) ) * ( ( nodeConst3 * 0.6 ) + 0.4 ) ), 0.0, 1.0 ) ), 1.0 ) * vec4<f32>( render.nodeUniform3 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform5, 0.0001 );
	SpecularColor = object.nodeUniform6;
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	let nodeConst6 = max( vec4<f32>( ( DiffuseColor.xyz + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst6;

	// result

	output.color = nodeConst6;

	return output;

}
