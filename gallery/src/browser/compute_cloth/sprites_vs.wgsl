// Three.js r187 - Node System

// directives


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< vec3<f32> >
};
@binding( 1 ) @group( 1 )
var<storage, read> nodeUniform0 : nodeUniform0Struct;

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
	positionLocal = nodeUniform0.value[ instanceIndex ];
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform5 );
	let nodeConst1 = ( modelViewMatrix * vec4<f32>( nodeUniform0.value[ instanceIndex ], 1.0 ) );
	let nodeConst2 = object.nodeUniform6;
	let nodeConst3 = cos( nodeConst2 );
	let nodeConst4 = sin( nodeConst2 );
	v_positionView = vec4<f32>( ( nodeConst1.xy + ( mat2x2<f32>( nodeConst3, nodeConst4, ( - nodeConst4 ), nodeConst3 ) * ( position.xy * vec2<f32>( length( object.nodeUniform5[ 0u ].xyz ), length( object.nodeUniform5[ 1u ].xyz ) ) ) ) ), nodeConst1.zw );
	let VERTEX_nodeConst5 = ( render.cameraProjectionMatrix * v_positionView );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst5;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
