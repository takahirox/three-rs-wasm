// Three.js r187 - Node System

// directives


// structs


// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_2d_array<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform8 : f32,
	nodeUniform11 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) vInstanceColor : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( instance_index ) instanceIndex : u32,
	@builtin( vertex_index ) vertexIndex : u32,
	@location( 0 ) position : vec3<f32>,
	@location( 1 ) nodeAttribute3 : vec4<f32>,
	@location( 2 ) nodeAttribute4 : vec4<f32>,
	@location( 3 ) nodeAttribute5 : vec4<f32>,
	@location( 4 ) nodeAttribute6 : vec4<f32>,
	@location( 5 ) nodeAttribute7 : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	positionLocal = ( positionLocal * vec3<f32>( object.nodeUniform0 ) );

	for ( var i : i32 = 0; i < 15; i ++ ) {

		nodeVar0 = 0.0;
		nodeVar1 = textureLoad( nodeUniform1, vec2<i32>( ( i + 1 ), i32( instanceIndex ) ), u32( 0u ) ).x;
		nodeVar0 = nodeVar1;

		if ( ( nodeVar0 != 0.0 ) ) {

			let nodeConst0 = ( ( i32( vertexIndex ) * 1 ) + 0 );
			let nodeConst1 = ( nodeConst0 / 796 );
			let nodeConst2 = vec2<i32>( ( nodeConst0 - ( nodeConst1 * 796 ) ), nodeConst1 );
			nodeVar2 = textureLoad( nodeUniform2, nodeConst2, i, u32( 0u ) );
			positionLocal = ( positionLocal + ( nodeVar2 * vec4<f32>( nodeVar0 ) ).xyz );
			

		}


	}

	positionLocal = ( mat4x4<f32>( nodeAttribute3, nodeAttribute4, nodeAttribute5, nodeAttribute6 ) * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.vInstanceColor = nodeAttribute7;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform11 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let VERTEX_nodeConst4 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst4;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
