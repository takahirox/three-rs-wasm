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

struct objectStruct {
	nodeUniform2 : f32
};
@binding( 4 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform3 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;

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


fn fn43 ( color : vec3<f32>, exposure : f32 ) -> vec3<f32> {

	var nodeVar1 : f32;

	var nodeVar0 : vec3<f32> = ( color * vec3<f32>( exposure ) );
	let nodeConst0 = min( nodeVar0.x, min( nodeVar0.y, nodeVar0.z ) );

	if ( ( nodeConst0 < 0.08 ) ) {

		nodeVar1 = ( nodeConst0 - ( 6.25 * ( nodeConst0 * nodeConst0 ) ) );

	} else {

		nodeVar1 = 0.04;

	}

	nodeVar0 = ( nodeVar0 - vec3<f32>( nodeVar1 ) );
	let nodeConst1 = max( nodeVar0.x, max( nodeVar0.y, nodeVar0.z ) );

	if ( ( nodeConst1 < 0.76 ) ) {

		return nodeVar0;

	}

	let nodeConst2 = ( 1.0 - 0.76 );
	let nodeConst3 = ( 1.0 - ( ( nodeConst2 * nodeConst2 ) / ( nodeConst1 + ( nodeConst2 - 0.76 ) ) ) );
	nodeVar0 = ( nodeVar0 * vec3<f32>( ( nodeConst3 / nodeConst1 ) ) );

	return mix( nodeVar0, vec3<f32>( nodeConst3 ), ( 1.0 - ( 1.0 / ( ( 0.15 * ( nodeConst1 - nodeConst3 ) ) + 1.0 ) ) ) );

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
	let nodeConst1 = ( nodeVar0 + ( nodeConst0 * vec4<f32>( object.nodeUniform2 ) ) );
	let nodeConst2 = fn4( vec4<f32>( nodeConst1.xyz, clamp( nodeConst1.w, 0.0, 1.0 ) ) );
	let nodeConst3 = vec4<f32>( fn43( nodeConst2.xyz, render.nodeUniform3 ), nodeConst2.w );

	// result

	output.color = fn3( vec4<f32>( fn2( nodeConst3.xyz ), nodeConst3.w ) );

	return output;

}
