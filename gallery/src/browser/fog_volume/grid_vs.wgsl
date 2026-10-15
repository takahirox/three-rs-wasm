// Three.js r186 - Node System

// directives


// structs


// uniforms

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform16 : vec4<f32>,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform22 : vec4<f32>,
	nodeUniform15 : f32,
	nodeUniform18 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : vec2<f32>,
	nodeUniform27 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar179 : vec4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) normal : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	varyings.v_positionWorld = ( object.nodeUniform0 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform5 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform0 );
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - varyings.v_positionView );
	VERTEX_nodeVar179 = ( render.cameraProjectionMatrix * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar179;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
