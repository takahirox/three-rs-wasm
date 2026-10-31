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
@binding( 1 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform1 : vec2<f32>
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


fn fn2 ( color : vec3<f32> ) -> vec3<f32> {

	


	return mix( ( ( pow( color, vec3<f32>( 0.41666 ) ) * vec3<f32>( 1.055 ) ) - vec3<f32>( 0.055 ) ), ( color * vec3<f32>( 12.92 ) ), vec3<f32>( ( color <= vec3<f32>( 0.0031308 ) ) ) );

}


fn fn3 ( color : vec4<f32> ) -> vec4<f32> {

	


	return vec4<f32>( ( color.xyz * vec3<f32>( color.w ) ), color.w );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code


	if ( ( object.nodeUniform0.y > ( ( ( fragCoord.xy / render.nodeUniform1 ).y - 0.5 ) * 0.25 ) ) ) {

		nodeVar1 = textureSample( nodeUniform2, nodeUniform2_sampler, nodeVarying0 );
		let nodeConst0 = nodeVar1;
		nodeVar0 = nodeConst0;

	} else {

		nodeVar1 = textureSample( nodeUniform2, nodeUniform2_sampler, nodeVarying0 );
		let nodeConst0 = nodeVar1;
		nodeVar0 = ( ( nodeConst0 * vec4<f32>( vec3<f32>( 0.174647403645279, 0.6038273388475408, 0.9046611743890203 ), 1.0 ) ) * vec4<f32>( ( 1.0 - clamp( ( distance( ( fragCoord.xy / render.nodeUniform1 ), vec2<f32>( 0.5 ) ) * 1.35 ), 0.0, 1.0 ) ) ) );

	}

	let nodeConst1 = fn4( vec4<f32>( nodeVar0.xyz, clamp( nodeVar0.w, 0.0, 1.0 ) ) );

	// result

	output.color = fn3( vec4<f32>( fn2( nodeConst1.xyz ), nodeConst1.w ) );

	return output;

}
