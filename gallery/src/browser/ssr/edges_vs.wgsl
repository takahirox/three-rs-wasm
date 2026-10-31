// Three.js r187 - Node System

// directives


// structs


// uniforms

struct objectStruct {
	nodeUniform1 : vec2<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) nodeVarying0 : vec4<f32>,
	@location( 1 ) nodeVarying1 : vec4<f32>,
	@location( 2 ) nodeVarying2 : vec4<f32>,
	@location( 3 ) nodeVarying3 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars


// codes


@vertex
fn main( @builtin( vertex_index ) vertexIndex : u32,
	@location( 0 ) uv : vec2<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.nodeVarying3 = uv;
	let nodeConst0 = ( vec4<f32>( uv, uv ) + ( vec4<f32>( object.nodeUniform1, object.nodeUniform1 ) * vec4<f32>( -1.0, 0.0, 0.0, -1.0 ) ) );
	varyings.nodeVarying0 = nodeConst0;
	let nodeConst3 = ( vec4<f32>( uv, uv ) + ( vec4<f32>( object.nodeUniform1, object.nodeUniform1 ) * vec4<f32>( 1.0, 0.0, 0.0, 1.0 ) ) );
	varyings.nodeVarying1 = nodeConst3;
	let nodeConst6 = ( vec4<f32>( uv, uv ) + ( vec4<f32>( object.nodeUniform1, object.nodeUniform1 ) * vec4<f32>( -2.0, 0.0, 0.0, -2.0 ) ) );
	varyings.nodeVarying2 = nodeConst6;
	let nodeConst9 = vec4<f32>( array< f32, 3 >( -1.0, -1.0, 3.0 )[ vertexIndex ], array< f32, 3 >( 3.0, -1.0, -1.0 )[ vertexIndex ], 0.0, 1.0 );

	// result

	varyings.builtinClipSpace = nodeConst9;

	return varyings;

}
