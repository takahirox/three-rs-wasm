// Three.js r187 - Node System

// directives


// structs


// uniforms

struct nodeUniform3Struct {
	value : array< mat4x4<f32> >
};
@binding( 1 ) @group( 1 )
var<storage, read> nodeUniform3 : nodeUniform3Struct;

struct nodeUniform5Struct {
	value : array< vec4<u32> >
};
@binding( 2 ) @group( 1 )
var<storage, read> nodeUniform5 : nodeUniform5Struct;

struct nodeUniform7Struct {
	value : array< vec4<f32> >
};
@binding( 3 ) @group( 1 )
var<storage, read> nodeUniform7 : nodeUniform7Struct;

struct nodeUniform8Struct {
	value : array< u32 >
};
@binding( 4 ) @group( 1 )
var<storage, read> nodeUniform8 : nodeUniform8Struct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform4 : u32,
	nodeUniform6 : u32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> modelViewMatrix : mat4x4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> vMeshletInstance : u32;
var<private> positionLocal : vec3<f32>;
var<private> nodeVarying4 : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( instance_index ) instanceIndex : u32,
	@builtin( vertex_index ) vertexIndex : u32 ) -> VaryingsStruct {

	// flow
	// code

	vMeshletInstance = nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].x;
	modelViewMatrix = ( render.cameraViewMatrix * nodeUniform3.value[ ( object.nodeUniform4 + vMeshletInstance ) ] );
	nodeVarying4 = nodeUniform7.value[ ( nodeUniform8.value[ ( ( ( ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].y + ( nodeUniform5.value[ ( object.nodeUniform6 + instanceIndex ) ].w * 64u ) ) + ( vertexIndex / 3u ) ) * 3u ) + ( vertexIndex % 3u ) ) ] + 0u ) ].xyz;
	positionLocal = nodeVarying4;
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let VERTEX_nodeConst1 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst1;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
