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
@binding( 2 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform1 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform2_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform2 : texture_2d<f32>;
@binding( 6 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 7 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;

// codes
fn fn4 ( color : vec4<f32> ) -> vec4<f32> {

	var nodeVar0 : vec4<f32>;


	if ( ( color.w == 0.0 ) ) {

		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	} else {

		nodeVar0 = vec4<f32>( ( color.xyz / vec3<f32>( color.w ) ), color.w );

	}


	return nodeVar0;

}


fn fn2 ( color : vec3<f32> ) -> vec3<f32> {

	


	return mix( ( ( pow( color, vec3<f32>( 0.41666 ) ) * vec3<f32>( 1.055 ) ) - vec3<f32>( 0.055 ) ), ( color * vec3<f32>( 12.92 ) ), vec3<f32>( ( color <= vec3<f32>( 0.0031308 ) ) ) );

}


fn fn3 ( color : vec4<f32> ) -> vec4<f32> {

	


	return vec4<f32>( ( color.xyz * vec3<f32>( color.w ) ), color.w );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	let nodeConst0 = nodeVar0;
	nodeVar1 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 ).x;
	let nodeConst1 = nodeVar1;
	nodeVar2 = textureSample( nodeUniform2, nodeUniform2_sampler, nodeVarying0 );
	nodeVar3 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 ).xyz;
	let nodeConst2 = vec4<f32>( ( ( nodeConst0.xyz * vec3<f32>( nodeConst1 ) ) + ( nodeVar2.xyz * nodeVar3 ) ), nodeConst0.w );
	let nodeConst3 = fn4( vec4<f32>( nodeConst2.xyz, clamp( nodeConst2.w, 0.0, 1.0 ) ) );

	// result

	output.color = fn3( vec4<f32>( fn2( nodeConst3.xyz ), nodeConst3.w ) );

	return output;

}
