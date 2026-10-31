// Three.js r187 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform34 : vec3<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform30 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	nodeUniform10 : vec2<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	cameraProjectionMatrixInverse : mat4x4<f32>,
	nodeUniform33 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec2<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : f32,
	nodeUniform20 : mat3x3<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : f32,
	nodeUniform27 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) nodeVarying2 : vec2<f32>,
	@location( 1 ) nodeVarying3 : f32,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_clipSpace : vec4<f32>,
	@location( 4 ) v_normalViewGeometry : vec3<f32>,
	@location( 5 ) nodeVarying8 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> modelViewMatrix : mat4x4<f32>;
var<private> normalLocal : vec3<f32>;
var<private> v_positionWorld : vec3<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_positionView : vec3<f32>;

// codes


@vertex
fn main( @location( 0 ) uv : vec2<f32>,
	@location( 1 ) position : vec3<f32>,
	@location( 2 ) normal : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.nodeVarying8 = uv;
	varyings.nodeVarying2 = vec2<f32>( 0.0, 0.0 );
	varyings.nodeVarying3 = 0.0;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform7 );
	positionLocal = position;
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	v_positionWorld = ( object.nodeUniform7 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let nodeConst0 = ( ( render.cameraProjectionMatrix * render.cameraViewMatrix ) * vec4<f32>( v_positionWorld, 1.0 ) );
	varyings.nodeVarying2 = ( uv * vec2<f32>( nodeConst0.w ) );
	varyings.nodeVarying3 = nodeConst0.w;
	let nodeConst1 = vec4<f32>( ( ( round( ( ( nodeConst0.xy / vec2<f32>( ( nodeConst0.w * 2.0 ) ) ) * render.nodeUniform10 ) ) / render.nodeUniform10 ) * vec2<f32>( ( nodeConst0.w * 2.0 ) ) ), nodeConst0.zw );
	varyings.v_clipSpace = nodeConst1;
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform11 * normalLocal ), 0.0 ) ).xyz );

	// result

	varyings.builtinClipSpace = nodeConst1;

	return varyings;

}
