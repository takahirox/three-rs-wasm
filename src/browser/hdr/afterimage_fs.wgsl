// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_2d<f32>;
@binding( 2 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform1 : texture_2d<f32>;

struct objectStruct {
	afterImageDecay : f32
};
@binding( 4 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	nodeVar1 = nodeVar0;
	nodeVar2 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	nodeVar3 = nodeVar2;
	let nodeConst0 = 0.1;
	nodeVar1 = ( nodeVar1 * ( vec4<f32>( object.afterImageDecay ) * max( sign( ( nodeVar1 - vec4<f32>( nodeConst0 ) ) ), vec4<f32>( 0.0 ) ) ) );

	// result

	output.color = max( nodeVar3, nodeVar1 );

	return output;

}
