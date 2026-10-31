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
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : vec2<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	nodeVar1 = ( nodeVar0 * vec4<f32>( 0.240841295721373 ) );
	let nodeConst0 = ( vec2<f32>( object.nodeUniform1 ) * vec2<f32>( 0.0, 1.0 ) );
	nodeVar2 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 1.0 ) ) );
	nodeVar3 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar2 ) );
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar2 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar3 + nodeVar4 ) * vec4<f32>( 0.2011675599937559 ) ) );
	nodeVar5 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 2.0 ) ) );
	nodeVar6 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar5 ) );
	nodeVar7 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar5 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar6 + nodeVar7 ) * vec4<f32>( 0.117230044020701 ) ) );
	nodeVar8 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 3.0 ) ) );
	nodeVar9 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar8 ) );
	nodeVar10 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar8 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar9 + nodeVar10 ) * vec4<f32>( 0.047662179108871855 ) ) );
	nodeVar11 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 4.0 ) ) );
	nodeVar12 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar11 ) );
	nodeVar13 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar11 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar12 + nodeVar13 ) * vec4<f32>( 0.013519569015984745 ) ) );

	// result

	output.color = nodeVar1;

	return output;

}
