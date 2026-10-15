// Three.js r186 - Node System

// directives


// structs


// uniforms

struct NodeBuffer_991Struct {
	value : array< vec3<f32> >
};
@binding( 5 ) @group( 1 )
var<storage, read> NodeBuffer_991 : NodeBuffer_991Struct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) nodeVarying3 : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar174 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) vertexIds : vec4<u32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	nodeVar0 = NodeBuffer_991.value[ vertexIds.x ];
	nodeVar1 = NodeBuffer_991.value[ vertexIds.y ];
	nodeVar2 = NodeBuffer_991.value[ vertexIds.z ];
	nodeVar3 = NodeBuffer_991.value[ vertexIds.w ];
	positionLocal = ( ( ( ( nodeVar0 + nodeVar1 ) + nodeVar2 ) + nodeVar3 ) * vec3<f32>( 0.25 ) );
	varyings.nodeVarying3 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform15 * cross( normalize( ( ( nodeVar1 + nodeVar3 ) - ( nodeVar0 + nodeVar2 ) ) ), normalize( ( ( nodeVar2 + nodeVar3 ) - ( nodeVar0 + nodeVar1 ) ) ) ) ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform16 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	VERTEX_nodeVar174 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar174;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
