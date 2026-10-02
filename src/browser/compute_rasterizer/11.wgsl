// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 2 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 3 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;

struct NodeBuffer_996Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 1 )
var<storage, read> NodeBuffer_996 : NodeBuffer_996Struct;

struct objectStruct {
	nodeUniform5 : u32,
	nodeUniform10 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : u32;
var<private> nodeVar3 : u32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) vUv : vec2<f32>,
	@location( 1 ) @interpolate(flat, either) vPayload : u32 ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	if ( ( f32( object.nodeUniform5 ) == 0.0 ) ) {

		nodeVar1 = ( NodeBuffer_996.value[ ( vPayload & 16383u ) ] + ( ( vPayload >> 14u ) * 1000u ) );
		nodeVar2 = ( ( nodeVar1 * 747796405u ) + 289559509u );
		nodeVar3 = ( ( ( nodeVar2 >> 16u ) ^ nodeVar2 ) * 277803737u );
		nodeVar4 = ( ( nodeVar3 >> 16u ) ^ nodeVar3 );
		nodeVar0 = vec4<f32>( ( ( ( f32( ( nodeVar4 & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), ( ( ( f32( ( ( nodeVar4 >> 8u ) & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), ( ( ( f32( ( ( nodeVar4 >> 16u ) & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), 1.0 );
		

	} else {

		nodeVar5 = textureSample( nodeUniform7, nodeUniform7_sampler, vUv );
		nodeVar0 = nodeVar5;
		

	}


	// result

	output.color = nodeVar0;

	return output;

}
