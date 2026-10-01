// Three.js r186 - Node System

// directives

// structs

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_2d_array<f32>;

struct NodeBuffer_3855Struct {
	value : array< mat4x4<f32>, 1024 >
};
@binding( 3 ) @group( 1 )
var<uniform> NodeBuffer_3855 : NodeBuffer_3855Struct;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform5 : f32,
	nodeUniform8 : mat4x4<f32>
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
var<private> nodeVar2 : i32;
var<private> nodeVar3 : i32;
var<private> nodeVar4 : vec2<i32>;
var<private> nodeVar5 : vec4<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar7 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes

@vertex
fn main( @builtin( instance_index ) instanceIndex : u32,
	@builtin( vertex_index ) vertexIndex : u32,
	@location( 0 ) position : vec3<f32>,
	@location( 1 ) nodeAttribute4 : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	positionLocal = ( positionLocal * vec3<f32>( object.nodeUniform0 ) );

	for ( var i : i32 = 0; i < 15; i ++ ) {

		nodeVar0 = 0.0;
		nodeVar1 = textureLoad( nodeUniform1, vec2<i32>( ( i + 1 ), i32( instanceIndex ) ), u32( 0u ) ).x;
		nodeVar0 = nodeVar1;

		if ( ( nodeVar0 != 0.0 ) ) {

			nodeVar2 = ( ( i32( vertexIndex ) * 1 ) + 0 );
			nodeVar3 = ( nodeVar2 / 796 );
			nodeVar4 = vec2<i32>( ( nodeVar2 - ( nodeVar3 * 796 ) ), nodeVar3 );
			nodeVar5 = textureLoad( nodeUniform2, nodeVar4, i, u32( 0u ) );
			positionLocal = ( positionLocal + ( nodeVar5.xyz * vec3<f32>( nodeVar0 ) ) );
			

		}

	}

	positionLocal = ( NodeBuffer_3855.value[ instanceIndex ] * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.vInstanceColor = nodeAttribute4;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform8 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar7 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar7;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
