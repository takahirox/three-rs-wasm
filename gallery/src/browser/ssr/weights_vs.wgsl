// Three.js r187 - Node System

// directives


// structs


// uniforms

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform2 : vec2<f32>,
	nodeUniform3 : vec2<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform7 : mat3x3<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) nodeVarying0 : vec4<f32>,
	@location( 1 ) nodeVarying1 : vec4<f32>,
	@location( 2 ) nodeVarying2 : vec4<f32>,
	@location( 3 ) nodeVarying3 : vec2<f32>,
	@location( 4 ) nodeVarying4 : vec2<f32>,
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

	varyings.nodeVarying4 = uv;
	let nodeConst0 = ( vec4<f32>( uv, uv ) + ( vec4<f32>( object.nodeUniform2, object.nodeUniform2 ) * vec4<f32>( -0.25, -0.125, 1.25, -0.125 ) ) );
	varyings.nodeVarying0 = nodeConst0;
	let nodeConst1 = ( vec4<f32>( uv, uv ) + ( vec4<f32>( object.nodeUniform2, object.nodeUniform2 ) * vec4<f32>( -0.125, -0.25, -0.125, 1.25 ) ) );
	varyings.nodeVarying2 = nodeConst1;
	let nodeConst2 = ( vec4<f32>( varyings.nodeVarying0.xz, varyings.nodeVarying2.yw ) + ( ( vec4<f32>( -2.0, 2.0, -2.0, 2.0 ) * vec4<f32>( object.nodeUniform2.xx, object.nodeUniform2.yy ) ) * vec4<f32>( 8.0 ) ) );
	varyings.nodeVarying1 = nodeConst2;
	let nodeConst3 = ( uv / object.nodeUniform2 );
	varyings.nodeVarying3 = nodeConst3;
	let nodeConst4 = vec4<f32>( array< f32, 3 >( -1.0, -1.0, 3.0 )[ vertexIndex ], array< f32, 3 >( 3.0, -1.0, -1.0 )[ vertexIndex ], 0.0, 1.0 );

	// result

	varyings.builtinClipSpace = nodeConst4;

	return varyings;

}
