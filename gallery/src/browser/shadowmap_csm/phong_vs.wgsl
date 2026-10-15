// Three.js r186 - Node System

// directives

// structs

// uniforms

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	shadowFar : f32,
	nodeUniform18 : f32,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : vec2<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : vec2<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : mat4x4<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform38 : f32,
	nodeUniform39 : vec2<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : mat4x4<f32>,
	nodeUniform42 : f32,
	nodeUniform43 : f32,
	nodeUniform45 : f32,
	nodeUniform46 : vec2<f32>,
	nodeUniform47 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform13 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar115 : vec4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes

@vertex
fn main( @location( 0 ) normal : vec3<f32>,
	@location( 1 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform8 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform13 );
	positionLocal = position;
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - varyings.v_positionView );
	varyings.v_positionWorld = ( object.nodeUniform13 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar115 = ( render.cameraProjectionMatrix * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar115;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
