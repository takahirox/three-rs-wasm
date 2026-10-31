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
	nodeUniform1 : mat3x3<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = nodeVarying0;
	nodeVar1 = vec2<f32>( nodeVar0.x, 1.0 - nodeVar0.y );
	nodeVar2 = ( 0.5 / 1024.0 );
	nodeVar3 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVar1 + vec2<f32>( nodeVar2, 0.0 ) ), 1.0 ) ).xy );
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVar1 + vec2<f32>( 0.0, nodeVar2 ) ), 1.0 ) ).xy );
	nodeVar5 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVar1 + vec2<f32>( 0.0, ( - nodeVar2 ) ) ), 1.0 ) ).xy );
	nodeVar6 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVar1 + vec2<f32>( ( - nodeVar2 ), 0.0 ) ), 1.0 ) ).xy );
	nodeVar7 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVar1 + vec2<f32>( nodeVar2, nodeVar2 ) ), 1.0 ) ).xy );
	nodeVar8 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVar1 + vec2<f32>( ( - nodeVar2 ), nodeVar2 ) ), 1.0 ) ).xy );
	nodeVar9 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVar1 + vec2<f32>( nodeVar2, ( - nodeVar2 ) ) ), 1.0 ) ).xy );
	nodeVar10 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVar1 + vec2<f32>( ( - nodeVar2 ), ( - nodeVar2 ) ) ), 1.0 ) ).xy );

	// result

	output.color = ( ( ( ( ( ( ( ( nodeVar3 + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) + nodeVar9 ) + nodeVar10 ) / vec4<f32>( 8.0 ) );

	return output;

}
