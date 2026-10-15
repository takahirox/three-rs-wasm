// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform6 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;

// codes

@fragment
fn main( @location( 0 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( nodeVarying4, 1.0 ) ).xy );
	nodeVar1 = vec4<f32>( nodeVar0.xyz, 0.8 );
	nodeVar2 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying4, 1.0 ) ).xy );
	DiffuseColor = vec4<f32>( nodeVar1.xyz, ( nodeVar1.w * nodeVar2.w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	nodeVar3 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar3;

	// result

	output.color = nodeVar3;

	return output;

}
