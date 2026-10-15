// Three.js r186 - Node System

// directives


// structs


// uniforms

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform44 : f32,
	nodeUniform45 : f32,
	nodeUniform46 : mat4x4<f32>,
	nodeUniform47 : mat4x4<f32>,
	nodeUniform49 : mat4x4<f32>,
	nodeUniform50 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraNear : f32,
	cameraFar : f32,
	nodeUniform48 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform26 : f32,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform28 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform37 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	nodeUniform39 : vec3<f32>,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform23 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform34 : f32,
	cameraPosition : vec3<f32>,
	nodeUniform11 : vec2<f32>,
	nodeUniform18 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : vec2<f32>,
	nodeUniform32 : f32,
	nodeUniform33 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) positionPrevious : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar99 : vec4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) position : vec3<f32>,
	@location( 1 ) normal : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.positionLocal = position;
	varyings.v_positionWorld = ( object.nodeUniform2 * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform15 * normalLocal ), 0.0 ) ).xyz );
	varyings.positionPrevious = position;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform2 );
	v_positionView = ( modelViewMatrix * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar99 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar99;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
