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
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_cube<f32>;

struct renderStruct {
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform1 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform8 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Output : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_normalWorldGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	normalWorldGeometry = normalize( v_normalWorldGeometry );
	let nodeConst0 = ( render.nodeUniform1 * vec4<f32>( normalWorldGeometry, 1.0 ) );
	nodeVar0 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, vec3<f32>( ( - nodeConst0.x ), nodeConst0.yz ), render.nodeUniform4 );
	DiffuseColor = ( nodeVar0 * vec4<f32>( render.nodeUniform5 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	let nodeConst1 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst1;

	// result

	output.color = nodeConst1;

	return output;

}
