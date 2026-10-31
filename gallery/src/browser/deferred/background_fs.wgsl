// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_cube<f32>;

struct renderStruct {
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform1 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraProjectionMatrixInverse : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform9 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> positionView : vec3<f32>;
var<private> Metalness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> Roughness : f32;

// codes


@fragment
fn main( @location( 0 ) v_normalWorldGeometry : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_clipSpace : vec4<f32> ) -> OutputType {

	// flow
	// code

	normalWorldGeometry = normalize( v_normalWorldGeometry );
	let nodeConst1 = ( render.nodeUniform1 * vec4<f32>( normalWorldGeometry, 1.0 ) );
	nodeVar0 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, vec3<f32>( ( - nodeConst1.x ), nodeConst1.yz ), render.nodeUniform4 );
	DiffuseColor = ( nodeVar0 * vec4<f32>( render.nodeUniform5 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = DiffuseColor;
	let nodeConst4 = ( render.cameraProjectionMatrixInverse * v_clipSpace );
	positionView = ( nodeConst4.xyz / vec3<f32>( nodeConst4.w ) );
	let nodeConst5 = vec4<f32>( positionView, Metalness );
	output.m1 = nodeConst5;
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	let nodeConst6 = vec4<f32>( normalView, Roughness );
	output.m2 = nodeConst6;

	// result

	return output;

}
