// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: f32
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform4 : mat4x4<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : f32;
var<private> nodeVar0 : vec4<f32>;

// codes


@fragment
fn main(  ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	nodeVar0 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar0.x;

	// result

	output.color = nodeVar0.x;

	return output;

}
