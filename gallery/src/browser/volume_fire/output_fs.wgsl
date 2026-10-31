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
@binding( 4 ) @group( 1 ) var nodeUniform3_sampler : sampler;
@binding( 5 ) @group( 1 ) var nodeUniform3 : texture_2d<f32>;

struct renderStruct {
	nodeUniform2 : f32,
	nodeUniform4 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> workingLuminanceCoefficients : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;

// codes
fn fn39 ( color : vec3<f32> ) -> vec3<f32> {

	


	return ( ( ( color * ( color + vec3<f32>( 0.0245786 ) ) ) - vec3<f32>( 0.000090537 ) ) / ( ( color * ( ( color * vec3<f32>( 0.983729 ) ) + vec3<f32>( 0.432951 ) ) ) + vec3<f32>( 0.238081 ) ) );

}


fn fn32 ( color : vec3<f32>, adjustment : f32, luminanceCoefficients : vec3<f32> ) -> vec3<f32> {

	


	return max( mix( vec3<f32>( dot( color, luminanceCoefficients ) ), color, adjustment ), vec3<f32>( 0.0 ) );

}


fn fn4 ( color : vec4<f32> ) -> vec4<f32> {

	var nodeVar0 : vec4<f32>;


	if ( ( color.w == 0.0 ) ) {

		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	} else {

		nodeVar0 = vec4<f32>( ( color.xyz / vec3<f32>( color.w ) ), color.w );

	}


	return nodeVar0;

}


fn fn40 ( color : vec3<f32>, exposure : f32 ) -> vec3<f32> {

	


	return clamp( ( mat3x3<f32>( 1.60475, -0.10208, -0.00327, -0.53108, 1.10813, -0.07276, -0.07367, -0.00605, 1.07602 ) * fn39( ( mat3x3<f32>( 0.59719, 0.076, 0.0284, 0.35458, 0.90834, 0.13383, 0.04823, 0.01566, 0.83777 ) * ( ( color * vec3<f32>( exposure ) ) / vec3<f32>( 0.6 ) ) ) ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );

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
	nodeVar1 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	let nodeConst0 = nodeVar1;
	workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
	let nodeConst1 = ( vec4<f32>( fn32( nodeConst0.xyz, render.nodeUniform2, workingLuminanceCoefficients ), nodeConst0.w ) * vec4<f32>( 0.5 ) );
	nodeVar2 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	let nodeConst2 = nodeVar2;
	let nodeConst3 = ( ( max( nodeVar0, nodeConst1 ) + nodeConst1 ) + nodeConst2 );
	let nodeConst4 = fn4( vec4<f32>( nodeConst3.xyz, clamp( nodeConst3.w, 0.0, 1.0 ) ) );
	let nodeConst5 = vec4<f32>( fn40( nodeConst4.xyz, render.nodeUniform4 ), nodeConst4.w );

	// result

	output.color = fn3( vec4<f32>( fn2( nodeConst5.xyz ), nodeConst5.w ) );

	return output;

}
