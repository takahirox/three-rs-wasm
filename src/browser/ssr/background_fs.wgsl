// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms

struct renderStruct {
	nodeUniform1 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform0 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform6 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar1 : vec2<f32>;

// codes


@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

	// flow
	// code

	DiffuseColor = ( vec4<f32>( mix( vec3<f32>( 0.24620132669705552, 0.24620132669705552, 0.1844749944900301 ), vec3<f32>( 0.1844749944900301, 0.13286832154414627, 0.13286832154414627 ), ( ( ( ( distance( ( fragCoord.xy / render.nodeUniform0 ), vec2<f32>( 0.5 ) ) - 0.0 ) / ( 0.5 - 0.0 ) ) * ( 1.0 - 0.0 ) ) + 0.0 ) ), 1.0 ) * vec4<f32>( render.nodeUniform1 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	nodeVar0 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m1 = vec4<f32>( nodeVar0, 1.0 );
	nodeVar1 = vec2<f32>( Metalness, Roughness );
	output.m2 = vec4<f32>( vec3<f32>( nodeVar1, 0.0 ), 1.0 );

	// result

	return output;

}
