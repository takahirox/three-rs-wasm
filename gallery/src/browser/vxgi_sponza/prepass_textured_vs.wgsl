// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform32 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform22 : mat4x4<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform26 : f32,
	nodeUniform27 : vec2<f32>,
	nodeUniform28 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform16 : mat3x3<f32>,
	nodeUniform17 : vec2<f32>,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform31 : mat4x4<f32>,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform34 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_tangentView : vec3<f32>,
	@location( 3 ) v_bitangentView : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) positionPrevious : vec3<f32>,
	@location( 7 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 8 ) nodeVarying11 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> tangentLocal : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> VERTEX_normalView : vec3<f32>;
var<private> VERTEX_tangentView : vec3<f32>;
var<private> VERTEX_nodeVar161 : vec4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) uv : vec2<f32>,
	@location( 1 ) normal : vec3<f32>,
	@location( 2 ) tangent : vec4<f32>,
	@location( 3 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.nodeVarying11 = uv;
	normalLocal = normal;
	nodeVar3 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform10 * normalLocal ), 0.0 ) ).xyz );
	varyings.v_normalViewGeometry = nodeVar3;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform14 );
	tangentLocal = tangent.xyz;
	varyings.v_tangentView = ( modelViewMatrix * vec4<f32>( tangentLocal, 0.0 ) ).xyz;
	normalViewGeometry = normalize( varyings.v_normalViewGeometry );
	VERTEX_normalView = normalViewGeometry;
	VERTEX_tangentView = normalize( varyings.v_tangentView );
	varyings.NORMAL_v_bitangentView = ( cross( VERTEX_normalView, VERTEX_tangentView ) * vec3<f32>( tangent.w ) );
	varyings.positionLocal = position;
	v_positionView = ( modelViewMatrix * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	varyings.v_positionWorld = ( object.nodeUniform14 * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	varyings.positionPrevious = position;
	VERTEX_nodeVar161 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar161;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
