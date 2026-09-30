// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec2<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform1 : texture_depth_2d;

struct renderStruct {
	nodeUniform0 : f32,
	nodeUniform3 : f32,
	nodeUniform4 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform2 : mat3x3<f32>
};
@binding( 1 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> meanVertical : f32;
var<private> squareMeanVertical : f32;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<u32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	meanVertical = 0.0;
	squareMeanVertical = 0.0;

	for ( var i : i32 = 0; i < i32( render.nodeUniform0 ); i ++ ) {

		if ( ( render.nodeUniform0 <= 1.0 ) ) {

			nodeVar0 = 0.0;

		} else {

			nodeVar0 = -1.0;

		}

		if ( ( render.nodeUniform0 <= 1.0 ) ) {

			nodeVar1 = 0.0;

		} else {

			nodeVar1 = ( 2.0 / ( render.nodeUniform0 - 1.0 ) );

		}

		nodeVar3 = textureDimensions( nodeUniform1, u32( 0 ) );
		nodeVar2 = textureLoad( nodeUniform1, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( object.nodeUniform2 * vec3<f32>( ( ( fragCoord.xy + ( vec2<f32>( 0.0, ( nodeVar0 + ( f32( i ) * nodeVar1 ) ) ) * vec2<f32>( render.nodeUniform3 ) ) ) / render.nodeUniform4 ), 1.0 ) ).xy ) * vec2<f32>( nodeVar3 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar3 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		meanVertical = ( meanVertical + nodeVar2 );
		squareMeanVertical = ( squareMeanVertical + ( nodeVar2 * nodeVar2 ) );

	}

	meanVertical = ( meanVertical / render.nodeUniform0 );
	squareMeanVertical = ( squareMeanVertical / render.nodeUniform0 );

	// result

	output.color = vec2<f32>( meanVertical, sqrt( max( ( squareMeanVertical - ( meanVertical * meanVertical ) ), 0.0 ) ) );

	return output;

}
