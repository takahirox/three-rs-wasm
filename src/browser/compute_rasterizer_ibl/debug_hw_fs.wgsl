// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct NodeBuffer_997Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 1 )
var<storage, read> NodeBuffer_997 : NodeBuffer_997Struct;

// vars
var<private> nodeVar11 : u32;
var<private> nodeVar12 : u32;
var<private> nodeVar13 : u32;
var<private> nodeVar14 : u32;

// codes


@fragment
fn main( @location( 0 ) @interpolate(flat, either) vInstId : u32,
	@location( 1 ) @interpolate(flat, either) vMegaTriIdx : u32,
	@location( 2 ) vUv : vec2<f32>,
	@location( 3 ) vNormal : vec3<f32>,
	@location( 4 ) vTangent : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar11 = ( NodeBuffer_997.value[ vMegaTriIdx ] + ( vInstId * 1000u ) );
	nodeVar12 = ( ( nodeVar11 * 747796405u ) + 289559509u );
	nodeVar13 = ( ( ( nodeVar12 >> 16u ) ^ nodeVar12 ) * 277803737u );
	nodeVar14 = ( ( nodeVar13 >> 16u ) ^ nodeVar13 );

	// result

	output.color = vec4<f32>( ( ( ( f32( ( nodeVar14 & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), ( ( ( f32( ( ( nodeVar14 >> 8u ) & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), ( ( ( f32( ( ( nodeVar14 >> 16u ) & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), 1.0 );

	return output;

}
