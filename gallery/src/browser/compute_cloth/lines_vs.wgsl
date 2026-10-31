// Three.js r187 - Node System

// directives


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< vec3<f32> >
};
@binding( 1 ) @group( 1 )
var<storage, read> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform1Struct {
	value : array< vec2<u32> >
};
@binding( 2 ) @group( 1 )
var<storage, read> nodeUniform1 : nodeUniform1Struct;

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

	if ( ( vertexIndex == 0u ) ) {

		nodeVar0 = nodeUniform1.value[ instanceIndex ].x;

	} else {

		nodeVar0 = nodeUniform1.value[ instanceIndex ].y;

	}

	positionLocal = nodeUniform0.value[ nodeVar0 ];
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform6 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let VERTEX_nodeConst1 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst1;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
