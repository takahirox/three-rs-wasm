// Three.js r186 - Node System

// directives


// structs


// uniforms


// varyings

struct VaryingsStruct {
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar7 : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	nodeVar7 = vec4<f32>( position.xy, 0.0, 1.0 );

	// result

	varyings.builtinClipSpace = nodeVar7;

	return varyings;

}
