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
@binding( 5 ) @group( 0 ) var nodeUniform5 : texture_depth_multisampled_2d;

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
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec2<u32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn1 ( color : vec4<f32> ) -> vec4<f32> {

	var nodeVar0 : vec4<f32>;


	if ( ( color.w == 0.0 ) ) {

		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	} else {

		nodeVar0 = vec4<f32>( ( color.xyz / vec3<f32>( color.w ) ), color.w );

	}


	return nodeVar0;

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
	nodeVar1 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	nodeVar2 = nodeVar1;
	nodeVar4 = textureDimensions( nodeUniform5 );
	nodeVar3 = textureLoad( nodeUniform5, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar4 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar4 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar5 = ( - ( ( object.nodeUniform3 * object.nodeUniform4 ) / ( ( ( object.nodeUniform4 - object.nodeUniform3 ) * nodeVar3 ) - object.nodeUniform4 ) ) );
	nodeVar6 = mix( nodeVar0, nodeVar2, ( 1.0 - exp( ( - ( ( ( object.nodeUniform2 * object.nodeUniform2 ) * nodeVar5 ) * nodeVar5 ) ) ) ) );
	nodeVar7 = fn1( vec4<f32>( nodeVar6.xyz, clamp( nodeVar6.w, 0.0, 1.0 ) ) );

	// result

	output.color = fn0( vec4<f32>( sRGBTransferOETF( nodeVar7.xyz ), nodeVar7.w ) );

	return output;

}
