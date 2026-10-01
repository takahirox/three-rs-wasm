// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec2<u32>;
var<private> nodeVar3 : vec4<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}



@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( ( nodeVarying0 - vec2<f32>( 0.5 ) ) * vec2<f32>( 2.0 ) );
	nodeVar2 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar1 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( ( ( ( nodeVar0 / vec2<f32>( ( 1.0 - ( dot( nodeVar0, nodeVar0 ) * object.nodeUniform1 ) ) ) ) * vec2<f32>( ( 1.0 - ( object.nodeUniform1 * 2.0 ) ) ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) ) ) * vec2<f32>( nodeVar2 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar2 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar3 = nodeVar1;

	// result

	output.color = nodeVar3;

	return output;

}
