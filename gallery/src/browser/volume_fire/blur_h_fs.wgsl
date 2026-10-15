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
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	nodeVar1 = ( nodeVar0 * vec4<f32>( 0.240841295721373 ) );
	nodeVar2 = ( vec2<f32>( object.nodeUniform1 ) * vec2<f32>( 1.0, 0.0 ) );
	nodeVar3 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 1.0 ) ) );
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar3 ) );
	nodeVar5 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar3 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar4 + nodeVar5 ) * vec4<f32>( 0.2011675599937559 ) ) );
	nodeVar6 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 2.0 ) ) );
	nodeVar7 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar6 ) );
	nodeVar8 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar6 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar7 + nodeVar8 ) * vec4<f32>( 0.117230044020701 ) ) );
	nodeVar9 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 3.0 ) ) );
	nodeVar10 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar9 ) );
	nodeVar11 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar9 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar10 + nodeVar11 ) * vec4<f32>( 0.047662179108871855 ) ) );
	nodeVar12 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 4.0 ) ) );
	nodeVar13 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar12 ) );
	nodeVar14 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar12 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar13 + nodeVar14 ) * vec4<f32>( 0.013519569015984745 ) ) );

	// result

	output.color = nodeVar1;

	return output;

}
