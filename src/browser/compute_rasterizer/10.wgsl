// Three.js r186 - Node System

// directives


// structs


// uniforms

struct NodeBuffer_994Struct {
	value : array< vec2<f32> >
};
@binding( 4 ) @group( 1 )
var<storage, read> NodeBuffer_994 : NodeBuffer_994Struct;

struct NodeBuffer_995Struct {
	value : array< u32 >
};
@binding( 5 ) @group( 1 )
var<storage, read> NodeBuffer_995 : NodeBuffer_995Struct;

struct NodeBuffer_1012Struct {
	value : array< u32 >
};
@binding( 6 ) @group( 1 )
var<storage, read> NodeBuffer_1012 : NodeBuffer_1012Struct;

struct NodeBuffer_1006Struct {
	value : array< mat4x4<f32> >
};
@binding( 7 ) @group( 1 )
var<storage, read> NodeBuffer_1006 : NodeBuffer_1006Struct;

struct NodeBuffer_993Struct {
	value : array< vec4<f32> >
};
@binding( 8 ) @group( 1 )
var<storage, read> NodeBuffer_993 : NodeBuffer_993Struct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform5 : u32,
	nodeUniform10 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) vUv : vec2<f32>,
	@location( 1 ) @interpolate(flat, either) vPayload : u32,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar6 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( vertex_index ) vertexIndex : u32,
	@location( 0 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	varyings.vUv = NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( ( NodeBuffer_1012.value[ ( ( vertexIndex / 3u ) + 1u ) ] & 16383u ) * 3u ) + ( vertexIndex % 3u ) ) ] ];
	varyings.vPayload = NodeBuffer_1012.value[ ( ( vertexIndex / 3u ) + 1u ) ];
	positionLocal = ( NodeBuffer_1006.value[ ( NodeBuffer_1012.value[ ( ( vertexIndex / 3u ) + 1u ) ] >> 14u ) ] * NodeBuffer_993.value[ NodeBuffer_995.value[ ( ( ( NodeBuffer_1012.value[ ( ( vertexIndex / 3u ) + 1u ) ] & 16383u ) * 3u ) + ( vertexIndex % 3u ) ) ] ] ).xyz;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform10 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar6 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar6;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
