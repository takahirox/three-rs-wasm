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

// vars
var<private> positionWorldDirection : vec3<f32>;
var<private> nodeVar0 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionViewDirection : vec3<f32>,
	@location( 1 ) v_positionWorldDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	positionWorldDirection = normalize( v_positionWorldDirection );
	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, vec3<f32>( ( - positionWorldDirection.x ), positionWorldDirection.yz ) );

	// result

	output.color = nodeVar0;

	return output;

}
