// Three.js r186 - Node System

// directives


// structs


// uniforms

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : vec2<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform6 : vec2<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) nodeVarying0 : vec4<f32>,
	@location( 1 ) nodeVarying1 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;

// codes


@vertex
fn main( @builtin( vertex_index ) vertexIndex : u32,
	@location( 0 ) uv : vec2<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.nodeVarying1 = uv;
	nodeVar4 = ( vec4<f32>( uv, uv ) + ( vec4<f32>( object.nodeUniform3, object.nodeUniform3 ) * vec4<f32>( 1.0, 0.0, 0.0, 1.0 ) ) );
	varyings.nodeVarying0 = nodeVar4;
	nodeVar18 = vec4<f32>( array< f32, 3 >( -1.0, -1.0, 3.0 )[ vertexIndex ], array< f32, 3 >( 3.0, -1.0, -1.0 )[ vertexIndex ], 0.0, 1.0 );

	// result

	varyings.builtinClipSpace = nodeVar18;

	return varyings;

}
