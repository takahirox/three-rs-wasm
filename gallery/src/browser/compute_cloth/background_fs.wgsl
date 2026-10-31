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

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : f32,
	nodeUniform10 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform2 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

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
	let nodeConst0 = ( object.nodeUniform1 * ( render.nodeUniform2 * vec4<f32>( normalWorldGeometry, 1.0 ) ) );
	let nodeConst1 = clamp( render.nodeUniform6, 0.0, 1.0 );
	nodeVar0 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, vec3<f32>( ( - nodeConst0.x ), nodeConst0.yz ), ( ( object.nodeUniform5 * nodeConst1 ) * ( 2.0 - nodeConst1 ) ) );
	DiffuseColor = ( vec4<f32>( nodeVar0.xyz, 1.0 ) * vec4<f32>( render.nodeUniform7 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform8 );
	DiffuseColor.w = 1.0;
	let nodeConst2 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst2;

	// result

	output.color = nodeConst2;

	return output;

}
