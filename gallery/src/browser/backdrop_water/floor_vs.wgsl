// Three.js r186 - Node System

// directives


// structs


// uniforms

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform2 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform21 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) nodeVarying7 : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar166 : vec4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) normal : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.positionLocal = position;
	varyings.nodeVarying7 = normal;
	varyings.v_positionWorld = ( object.nodeUniform1 * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform4 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform1 );
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - varyings.v_positionView );
	VERTEX_nodeVar166 = ( render.cameraProjectionMatrix * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar166;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
