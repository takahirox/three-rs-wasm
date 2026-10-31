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
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 2 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 3 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;

struct renderStruct {
	nodeUniform2 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : f32,
	nodeUniform4 : f32
};
@binding( 4 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> workingLuminanceCoefficients : vec3<f32>;

// codes
fn fn32 ( color : vec3<f32>, adjustment : f32, luminanceCoefficients : vec3<f32> ) -> vec3<f32> {

	


	return max( mix( vec3<f32>( dot( color, luminanceCoefficients ) ), color, adjustment ), vec3<f32>( 0.0 ) );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	nodeVar1 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	let nodeConst0 = nodeVar1;
	workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
	let nodeConst1 = ( vec4<f32>( fn32( nodeConst0.xyz, render.nodeUniform2, workingLuminanceCoefficients ), nodeConst0.w ) * vec4<f32>( 0.5 ) );
	let nodeConst2 = ( max( nodeVar0, nodeConst1 ) + nodeConst1 );

	// result

	output.color = mix( vec4<f32>( 0.0, 0.0, 0.0, 0.0 ), nodeConst2, smoothstep( object.nodeUniform3, ( object.nodeUniform3 + object.nodeUniform4 ), dot( nodeConst2.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ) );

	return output;

}
