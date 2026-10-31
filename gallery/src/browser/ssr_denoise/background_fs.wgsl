// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	@location( 3 ) m3 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_cube<f32>;

struct renderStruct {
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform1 : mat4x4<f32>,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform12 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform16 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;

// codes


@fragment
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) v_normalWorldGeometry : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) positionPrevious : vec3<f32> ) -> OutputType {

	// flow
	// code

	normalWorldGeometry = normalize( v_normalWorldGeometry );
	let nodeConst1 = ( render.nodeUniform1 * vec4<f32>( normalWorldGeometry, 1.0 ) );
	nodeVar0 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, vec3<f32>( ( - nodeConst1.x ), nodeConst1.yz ), render.nodeUniform4 );
	DiffuseColor = ( nodeVar0 * vec4<f32>( render.nodeUniform5 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	let nodeConst2 = vec4<f32>( ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ), object.nodeUniform7 );
	output.m1 = nodeConst2;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform10 );
	let nodeConst3 = ( ( render.nodeUniform8 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	let nodeConst4 = ( ( render.nodeUniform11 * ( render.nodeUniform12 * object.nodeUniform13 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	let nodeConst5 = ( ( nodeConst3.xy / vec2<f32>( nodeConst3.w ) ) - ( nodeConst4.xy / vec2<f32>( nodeConst4.w ) ) );
	output.m2 = vec4<f32>( vec3<f32>( nodeConst5, 0.0 ), 1.0 );
	let nodeConst6 = vec4<f32>( DiffuseColor.xyz, object.nodeUniform14 );
	output.m3 = nodeConst6;

	// result

	return output;

}
