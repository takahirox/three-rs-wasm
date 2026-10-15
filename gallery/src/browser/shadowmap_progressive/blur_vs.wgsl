// Three.js r186 - Node System

// directives


// structs


// uniforms


// varyings

struct VaryingsStruct {
	@location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar11 : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) uv : vec2<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.nodeVarying0 = uv;
	nodeVar11 = vec4<f32>( ( ( uv - vec2<f32>( 0.5, 0.5 ) ) * vec2<f32>( 2.0 ) ), 1.0, 1.0 );

	// result

	varyings.builtinClipSpace = nodeVar11;

	return varyings;

}
