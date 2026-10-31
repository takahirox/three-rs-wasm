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

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform1 : vec2<f32>,
	nodeUniform2 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : vec4<f32>;

// codes
fn fn41 ( x : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<f32>;
	var nodeVar2 : vec3<f32>;

	nodeVar0 = x;
	nodeVar1 = ( nodeVar0 * nodeVar0 );
	nodeVar2 = ( nodeVar1 * nodeVar1 );

	return ( ( ( vec3<f32>( 15.5 ) * ( nodeVar2 * nodeVar1 ) ) - ( vec3<f32>( 40.14 ) * ( nodeVar2 * nodeVar0 ) ) ) + ( ( ( vec3<f32>( 31.96 ) * nodeVar2 ) - ( vec3<f32>( 6.868 ) * ( nodeVar1 * nodeVar0 ) ) ) + ( ( vec3<f32>( 0.4298 ) * nodeVar1 ) + ( ( vec3<f32>( 0.1191 ) * nodeVar0 ) - vec3<f32>( 0.00232 ) ) ) ) );

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


fn fn42 ( color : vec3<f32>, exposure : f32 ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;

	nodeVar0 = color;
	nodeVar0 = ( nodeVar0 * vec3<f32>( exposure ) );
	nodeVar0 = ( mat3x3<f32>( vec3<f32>( 0.6274, 0.0691, 0.0164 ), vec3<f32>( 0.3293, 0.9195, 0.088 ), vec3<f32>( 0.0433, 0.0113, 0.8956 ) ) * nodeVar0 );
	nodeVar0 = ( mat3x3<f32>( vec3<f32>( 0.856627153315983, 0.137318972929847, 0.11189821299995 ), vec3<f32>( 0.0951212405381588, 0.761241990602591, 0.0767994186031903 ), vec3<f32>( 0.0482516061458583, 0.101439036467562, 0.811302368396859 ) ) * nodeVar0 );
	nodeVar0 = max( nodeVar0, vec3<f32>( 1e-10 ) );
	nodeVar0 = log2( nodeVar0 );
	nodeVar0 = ( ( nodeVar0 - vec3<f32>( -12.47393 ) ) / vec3<f32>( ( 4.026069 - -12.47393 ) ) );
	nodeVar0 = clamp( nodeVar0, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );
	nodeVar0 = fn41( nodeVar0 );
	nodeVar0 = ( mat3x3<f32>( vec3<f32>( 1.1271005818144368, -0.1413297634984383, -0.14132976349843826 ), vec3<f32>( -0.11060664309660323, 1.157823702216272, -0.11060664309660294 ), vec3<f32>( -0.016493938717834573, -0.016493938717834257, 1.2519364065950405 ) ) * nodeVar0 );
	nodeVar0 = pow( max( vec3<f32>( 0.0, 0.0, 0.0 ), nodeVar0 ), vec3<f32>( 2.2, 2.2, 2.2 ) );
	nodeVar0 = ( mat3x3<f32>( vec3<f32>( 1.6605, -0.1246, -0.0182 ), vec3<f32>( -0.5876, 1.1329, -0.1006 ), vec3<f32>( -0.0728, -0.0083, 1.1187 ) ) * nodeVar0 );
	nodeVar0 = clamp( nodeVar0, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );

	return nodeVar0;

}


fn fn2 ( color : vec3<f32> ) -> vec3<f32> {

	


	return mix( ( ( pow( color, vec3<f32>( 0.41666 ) ) * vec3<f32>( 1.055 ) ) - vec3<f32>( 0.055 ) ), ( color * vec3<f32>( 12.92 ) ), vec3<f32>( ( color <= vec3<f32>( 0.0031308 ) ) ) );

}


fn fn3 ( color : vec4<f32> ) -> vec4<f32> {

	


	return vec4<f32>( ( color.xyz * vec3<f32>( color.w ) ), color.w );

}




@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, ( fragCoord.xy / render.nodeUniform1 ) );
	let nodeConst0 = fn4( vec4<f32>( nodeVar0.xyz, clamp( nodeVar0.w, 0.0, 1.0 ) ) );
	let nodeConst1 = vec4<f32>( fn42( nodeConst0.xyz, render.nodeUniform2 ), nodeConst0.w );

	// result

	output.color = fn3( vec4<f32>( fn2( nodeConst1.xyz ), nodeConst1.w ) );

	return output;

}
