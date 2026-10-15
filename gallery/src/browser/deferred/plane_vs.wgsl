// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : vec3<f32>,
	nodeUniform32 : f32,
	nodeUniform33 : f32,
	nodeUniform35 : vec3<f32>,
	nodeUniform36 : f32,
	nodeUniform37 : f32,
	nodeUniform39 : vec3<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform34 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform42 : f32,
	nodeUniform43 : mat4x4<f32>,
	nodeUniform45 : f32,
	nodeUniform46 : f32,
	nodeUniform48 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar382 : vec4<f32>;
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
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform5 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform9 );
	positionLocal = position;
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - varyings.v_positionView );
	VERTEX_nodeVar382 = ( render.cameraProjectionMatrix * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar382;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
