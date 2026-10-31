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

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform8 : f32,
	nodeUniform11 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) vInstanceColor : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = ( vec4<f32>( 0.0, 0.0, 0.0, 1.0 ) * vec4<f32>( vInstanceColor, 1.0 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform8 );
	let nodeConst3 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst3;

	// result

	output.color = nodeConst3;

	return output;

}
