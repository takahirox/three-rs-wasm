// Three.js r186 - Node System

// directives


// structs


// uniforms

struct NodeBuffer_5410Struct {
	value : array< mat4x4<f32>, 67 >
};
@binding( 9 ) @group( 1 )
var<uniform> NodeBuffer_5410 : NodeBuffer_5410Struct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform19 : mat3x3<f32>,
	nodeUniform20 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform29 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying6 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar181 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) skinIndex : vec4<u32>,
	@location( 2 ) skinWeight : vec4<f32>,
	@location( 3 ) normal : vec3<f32>,
	@location( 4 ) uv : vec2<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	nodeVar0 = ( object.nodeUniform2 * vec4<f32>( positionLocal, 1.0 ) );
	positionLocal = ( object.nodeUniform0 * ( ( ( ( ( skinWeight.x * NodeBuffer_5410.value[ skinIndex.x ] ) * nodeVar0 ) + ( ( skinWeight.y * NodeBuffer_5410.value[ skinIndex.y ] ) * nodeVar0 ) ) + ( ( skinWeight.z * NodeBuffer_5410.value[ skinIndex.z ] ) * nodeVar0 ) ) + ( ( skinWeight.w * NodeBuffer_5410.value[ skinIndex.w ] ) * nodeVar0 ) ) ).xyz;
	normalLocal = normal;
	normalLocal = ( mat3x3<f32>( ( ( object.nodeUniform0 * ( ( ( skinWeight.x * NodeBuffer_5410.value[ skinIndex.x ] + skinWeight.y * NodeBuffer_5410.value[ skinIndex.y ] ) + skinWeight.z * NodeBuffer_5410.value[ skinIndex.z ] ) + skinWeight.w * NodeBuffer_5410.value[ skinIndex.w ] ) ) * object.nodeUniform2 )[ 0 ].xyz, ( ( object.nodeUniform0 * ( ( ( skinWeight.x * NodeBuffer_5410.value[ skinIndex.x ] + skinWeight.y * NodeBuffer_5410.value[ skinIndex.y ] ) + skinWeight.z * NodeBuffer_5410.value[ skinIndex.z ] ) + skinWeight.w * NodeBuffer_5410.value[ skinIndex.w ] ) ) * object.nodeUniform2 )[ 1 ].xyz, ( ( object.nodeUniform0 * ( ( ( skinWeight.x * NodeBuffer_5410.value[ skinIndex.x ] + skinWeight.y * NodeBuffer_5410.value[ skinIndex.y ] ) + skinWeight.z * NodeBuffer_5410.value[ skinIndex.z ] ) + skinWeight.w * NodeBuffer_5410.value[ skinIndex.w ] ) ) * object.nodeUniform2 )[ 2 ].xyz ) * normalLocal );
	varyings.nodeVarying6 = uv;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform13 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform17 );
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - varyings.v_positionView );
	VERTEX_nodeVar181 = ( render.cameraProjectionMatrix * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar181;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
