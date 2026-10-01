// Three.js r186 - Node System

// directives


// structs


// uniforms


// varyings

struct VaryingsStruct {
	@location( 0 ) v_clipSpace : vec4<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : vec4<f32>;

// codes


@vertex
fn main( @builtin( vertex_index ) vertexIndex : u32 ) -> VaryingsStruct {

	// flow
	// code

	nodeVar0 = vec4<f32>( array< f32, 3 >( -1.0, -1.0, 3.0 )[ vertexIndex ], array< f32, 3 >( 3.0, -1.0, -1.0 )[ vertexIndex ], 0.0, 1.0 );
	varyings.v_clipSpace = nodeVar0;

	// result

	varyings.builtinClipSpace = nodeVar0;

	return varyings;

}
