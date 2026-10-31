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
	nodeUniform3 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;

// codes


@fragment
fn main(  ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = mix( vec3<f32>( 1.0 ), vec3<f32>( 1.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor = vec4<f32>( nodeConst0, vec4<f32>( nodeConst0, 1.0 ).w );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	let nodeConst1 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst1;

	// result

	output.color = nodeConst1;

	return output;

}
