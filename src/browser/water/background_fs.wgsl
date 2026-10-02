// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_cube<f32>;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform5 : f32,
	nodeUniform6 : f32,
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
var<private> nodeVar1 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> EmissiveColor : vec3<f32>;

// codes


@fragment
fn main( @location( 0 ) v_normalWorldGeometry : vec3<f32> ) -> OutputType {

	// flow
	// code

	normalWorldGeometry = normalize( v_normalWorldGeometry );
	nodeVar0 = ( object.nodeUniform1 * ( render.nodeUniform2 * vec4<f32>( normalWorldGeometry, 1.0 ) ) );
	nodeVar1 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, vec3<f32>( ( - nodeVar0.x ), nodeVar0.yz ), render.nodeUniform5 );
	DiffuseColor = ( nodeVar1 * vec4<f32>( render.nodeUniform6 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	output.m1 = vec4<f32>( EmissiveColor, 1.0 );

	// result

	return output;

}
