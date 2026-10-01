// Three.js r186 - Node System

// directives


// structs


// uniforms

struct NodeBuffer_7764Struct {
	value : array< vec4<f32> >
};
@binding( 11 ) @group( 1 )
var<storage, read> NodeBuffer_7764 : NodeBuffer_7764Struct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : f32,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : mat4x4<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32,
	nodeUniform36 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) nodeVarying4 : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : u32;
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar208 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( instance_index ) instanceIndex : u32,
	@builtin( vertex_index ) vertexIndex : u32,
	@location( 0 ) position : vec3<f32>,
	@location( 1 ) uv : vec2<f32>,
	@location( 2 ) normal : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	nodeVar0 = ( ( ( instanceIndex * 16340u ) + vertexIndex ) * 2u );
	positionLocal = NodeBuffer_7764.value[ nodeVar0 ].xyz;
	varyings.nodeVarying7 = uv;
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform11 * normalLocal ), 0.0 ) ).xyz );
	varyings.nodeVarying4 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform11 * NodeBuffer_7764.value[ ( nodeVar0 + 1u ) ].xyz ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform20 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	VERTEX_nodeVar208 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar208;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
