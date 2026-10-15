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
	nodeUniform1 : mat3x3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : mat3x3<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec3<f32>;

// codes

@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( vec2<f32>( 1.0, 1.0 ) / vec2<f32>( textureDimensions( nodeUniform0, 0 ) ) );
	nodeVar1 = ( vec4<f32>( 1.0, 0.0, 0.0, 1.0 ) * vec4<f32>( nodeVar0, nodeVar0 ) );
	nodeVar2 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVarying0 + nodeVar1.xy ), 1.0 ) ).xy );
	nodeVar3 = nodeVar2;
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform2 * vec3<f32>( ( nodeVarying0 - nodeVar1.xy ), 1.0 ) ).xy );
	nodeVar5 = nodeVar4;
	nodeVar6 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform3 * vec3<f32>( ( nodeVarying0 + nodeVar1.yw ), 1.0 ) ).xy );
	nodeVar7 = nodeVar6;
	nodeVar8 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform4 * vec3<f32>( ( nodeVarying0 - nodeVar1.yw ), 1.0 ) ).xy );
	nodeVar9 = nodeVar8;

	if ( ( ( 1.0 - min( min( nodeVar3.y, nodeVar5.y ), min( nodeVar7.y, nodeVar9.y ) ) ) > 0.001 ) ) {

		nodeVar10 = vec3<f32>( 1.0, 0.0, 0.0 );

	} else {

		nodeVar10 = vec3<f32>( 0.0, 1.0, 0.0 );

	}

	// result

	output.color = ( vec4<f32>( nodeVar10, 1.0 ) * vec4<f32>( length( vec2<f32>( ( ( nodeVar3.x - nodeVar5.x ) * 0.5 ), ( ( nodeVar7.x - nodeVar9.x ) * 0.5 ) ) ) ) );

	return output;

}
