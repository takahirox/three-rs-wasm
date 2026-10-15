// Three.js r186 - Node System

// directives


// structs


// uniforms

struct NodeBuffer_991Struct {
	value : array< vec3<f32> >
};
@binding( 1 ) @group( 1 )
var<storage, read> NodeBuffer_991 : NodeBuffer_991Struct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform2 : f32,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : f32
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
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> VERTEX_nodeVar4 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec4<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( instance_index ) instanceIndex : u32,
	@location( 0 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	positionLocal = NodeBuffer_991.value[ instanceIndex ];
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform5 );
	nodeVar1 = ( modelViewMatrix * vec4<f32>( NodeBuffer_991.value[ instanceIndex ], 1.0 ) );
	nodeVar2 = cos( object.nodeUniform6 );
	nodeVar3 = sin( object.nodeUniform6 );
	v_positionView = vec4<f32>( ( nodeVar1.xy + ( mat2x2<f32>( nodeVar2, nodeVar3, ( - nodeVar3 ), nodeVar2 ) * ( position.xy * vec2<f32>( length( object.nodeUniform5[ 0u ].xyz ), length( object.nodeUniform5[ 1u ].xyz ) ) ) ) ), nodeVar1.zw );
	VERTEX_nodeVar4 = ( render.cameraProjectionMatrix * v_positionView );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar4;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
