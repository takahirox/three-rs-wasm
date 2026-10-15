// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

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
var<private> nodeVar0 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;

// codes

@fragment
fn main(  ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = mix( vec3<f32>( 1.0 ), vec3<f32>( 1.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor = vec4<f32>( nodeVar0, vec4<f32>( nodeVar0, 1.0 ).w );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	nodeVar1 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar1;

	// result

	output.color = nodeVar1;

	return output;

}
