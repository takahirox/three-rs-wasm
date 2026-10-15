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
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform4 : f32
};
@binding( 4 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	nodeVar1 = nodeVar0;
	nodeVar1 = nodeVar0;
	nodeVar2 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	nodeVar3 = nodeVar2;
	nodeVar3 = nodeVar2;
	nodeVar4 = ( vec4<f32>( max( mix( vec3<f32>( dot( nodeVar3.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar3.xyz, object.nodeUniform2 ), vec3<f32>( 0.0 ) ), nodeVar3.w ) * vec4<f32>( 0.5 ) );
	nodeVar5 = ( max( nodeVar1, nodeVar4 ) + nodeVar4 );

	// result

	output.color = mix( vec4<f32>( 0.0, 0.0, 0.0, 0.0 ), nodeVar5, smoothstep( object.nodeUniform3, ( object.nodeUniform3 + object.nodeUniform4 ), dot( nodeVar5.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ) );

	return output;

}
