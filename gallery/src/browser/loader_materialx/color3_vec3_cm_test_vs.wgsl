// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform20 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_tangentWorld : vec3<f32>,
	@location( 2 ) v_bitangentWorld : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) NORMAL_v_tangentWorld : vec3<f32>,
	@location( 5 ) NORMAL_v_bitangentWorld : vec3<f32>,
	@location( 6 ) nodeVarying12 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> tangentLocal : vec3<f32>;
var<private> VERTEX_tangentView : vec3<f32>;
var<private> VERTEX_nodeVar3 : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> VERTEX_normalView : vec3<f32>;
var<private> VERTEX_normalWorld : vec3<f32>;
var<private> VERTEX_tangentWorld : vec3<f32>;
var<private> VERTEX_nodeVar149 : vec4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_tangentView : vec3<f32>;
var<private> VERTEX_v_tangentWorld : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) normal : vec3<f32>,
	@location( 1 ) tangent : vec4<f32>,
	@location( 2 ) uv : vec2<f32>,
	@location( 3 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	normalLocal = normal;
	nodeVar0 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform3 * normalLocal ), 0.0 ) ).xyz );
	varyings.v_normalViewGeometry = nodeVar0;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform12 );
	tangentLocal = tangent.xyz;
	v_tangentView = ( modelViewMatrix * vec4<f32>( tangentLocal, 0.0 ) ).xyz;
	VERTEX_tangentView = normalize( v_tangentView );
	VERTEX_nodeVar3 = normalize( ( render.cameraWorldMatrix * vec4<f32>( VERTEX_tangentView, 0.0 ) ).xyz );
	varyings.NORMAL_v_tangentWorld = VERTEX_nodeVar3;
	varyings.nodeVarying12 = uv;
	normalViewGeometry = normalize( varyings.v_normalViewGeometry );
	VERTEX_normalView = normalViewGeometry;
	VERTEX_normalWorld = normalize( ( vec4<f32>( VERTEX_normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	VERTEX_v_tangentWorld = VERTEX_nodeVar3;
	VERTEX_tangentWorld = normalize( VERTEX_v_tangentWorld );
	varyings.NORMAL_v_bitangentWorld = ( cross( VERTEX_normalWorld, VERTEX_tangentWorld ) * vec3<f32>( tangent.w ) );
	positionLocal = position;
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	VERTEX_nodeVar149 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar149;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
