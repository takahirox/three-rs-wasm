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
@binding( 2 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 3 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform3_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform3 : texture_2d<f32>;

struct objectStruct {
	nodeUniform2 : f32
};
@binding( 4 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform4 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;

// codes
fn fn1 ( color : vec4<f32> ) -> vec4<f32> {

	var nodeVar0 : vec4<f32>;


	if ( ( color.w == 0.0 ) ) {

		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	} else {

		nodeVar0 = vec4<f32>( ( color.xyz / vec3<f32>( color.w ) ), color.w );

	}


	return nodeVar0;

}


fn acesFilmicToneMapping ( color : vec3<f32>, exposure : f32 ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;

	nodeVar0 = ( mat3x3<f32>( 0.59719, 0.076, 0.0284, 0.35458, 0.90834, 0.13383, 0.04823, 0.01566, 0.83777 ) * ( ( color * vec3<f32>( exposure ) ) / vec3<f32>( 0.6 ) ) );

	return clamp( ( mat3x3<f32>( 1.60475, -0.10208, -0.00327, -0.53108, 1.10813, -0.07276, -0.07367, -0.00605, 1.07602 ) * ( ( ( nodeVar0 * ( nodeVar0 + vec3<f32>( 0.0245786 ) ) ) - vec3<f32>( 0.000090537 ) ) / ( ( nodeVar0 * ( ( nodeVar0 + vec3<f32>( 0.432951 ) ) * vec3<f32>( 0.983729 ) ) ) + vec3<f32>( 0.238081 ) ) ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );

}


fn sRGBTransferOETF ( color : vec3<f32> ) -> vec3<f32> {

	


	return mix( ( ( pow( color, vec3<f32>( 0.41666 ) ) * vec3<f32>( 1.055 ) ) - vec3<f32>( 0.055 ) ), ( color * vec3<f32>( 12.92 ) ), vec3<f32>( ( color <= vec3<f32>( 0.0031308 ) ) ) );

}


fn fn0 ( color : vec4<f32> ) -> vec4<f32> {

	


	return vec4<f32>( ( color.xyz * vec3<f32>( color.w ) ), color.w );

}




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
	nodeVar5 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	nodeVar6 = nodeVar5;
	nodeVar7 = ( ( max( nodeVar1, nodeVar4 ) + nodeVar4 ) + nodeVar6 );
	nodeVar8 = fn1( vec4<f32>( nodeVar7.xyz, clamp( nodeVar7.w, 0.0, 1.0 ) ) );
	nodeVar9 = vec4<f32>( acesFilmicToneMapping( nodeVar8.xyz, render.nodeUniform4 ), nodeVar8.w );

	// result

	output.color = fn0( vec4<f32>( sRGBTransferOETF( nodeVar9.xyz ), nodeVar9.w ) );

	return output;

}
