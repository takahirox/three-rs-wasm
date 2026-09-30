// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec2<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;

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
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> meanHorizontal : f32;
var<private> squareMeanHorizontal : f32;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;

// codes

@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	meanHorizontal = 0.0;
	squareMeanHorizontal = 0.0;

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

		nodeVar2 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( ( ( fragCoord.xy + ( vec2<f32>( ( nodeVar0 + ( f32( i ) * nodeVar1 ) ), 0.0 ) * vec2<f32>( render.nodeUniform3 ) ) ) / render.nodeUniform4 ), 1.0 ) ).xy ).xy;
		meanHorizontal = ( meanHorizontal + nodeVar2.x );
		squareMeanHorizontal = ( squareMeanHorizontal + ( ( nodeVar2.y * nodeVar2.y ) + ( nodeVar2.x * nodeVar2.x ) ) );

	}

	meanHorizontal = ( meanHorizontal / render.nodeUniform0 );
	squareMeanHorizontal = ( squareMeanHorizontal / render.nodeUniform0 );

	// result

	output.color = vec2<f32>( meanHorizontal, sqrt( max( ( squareMeanHorizontal - ( meanHorizontal * meanHorizontal ) ), 0.0 ) ) );

	return output;

}
