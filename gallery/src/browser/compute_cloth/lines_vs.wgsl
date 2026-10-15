// Three.js r186 - Node System

// directives


// structs


// uniforms

struct NodeBuffer_991Struct {
	value : array< vec3<f32> >
};
@binding( 1 ) @group( 1 )
var<storage, read> NodeBuffer_991 : NodeBuffer_991Struct;

struct NodeBuffer_995Struct {
	value : array< vec2<u32> >
};
@binding( 2 ) @group( 1 )
var<storage, read> NodeBuffer_995 : NodeBuffer_995Struct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : f32,
	nodeUniform6 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : u32;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar2 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( instance_index ) instanceIndex : u32,
	@location( 0 ) position : vec3<f32>,
	@location( 1 ) vertexIndex : u32 ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;

	if ( ( f32( vertexIndex ) == 0.0 ) ) {

		nodeVar0 = NodeBuffer_995.value[ instanceIndex ].x;

	} else {

		nodeVar0 = NodeBuffer_995.value[ instanceIndex ].y;

	}

	positionLocal = NodeBuffer_991.value[ nodeVar0 ];
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform6 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar2 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar2;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
