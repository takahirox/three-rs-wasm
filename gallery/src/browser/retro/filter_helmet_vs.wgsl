// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform33 : vec3<f32>,
	nodeUniform35 : vec3<f32>,
	nodeUniform36 : f32,
	nodeUniform37 : f32,
	nodeUniform31 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform34 : vec3<f32>,
	cameraProjectionMatrixInverse : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform11 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform14 : mat3x3<f32>,
	nodeUniform15 : vec2<f32>,
	nodeUniform16 : f32,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform19 : f32,
	nodeUniform21 : mat3x3<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : f32,
	nodeUniform28 : mat3x3<f32>
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
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
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
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform8 );
	positionLocal = position;
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	v_positionWorld = ( object.nodeUniform8 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	nodeVar1 = ( ( render.cameraProjectionMatrix * render.cameraViewMatrix ) * vec4<f32>( v_positionWorld, 1.0 ) );
	varyings.nodeVarying2 = ( uv * vec2<f32>( nodeVar1.w ) );
	varyings.nodeVarying3 = nodeVar1.w;
	nodeVar2 = vec4<f32>( ( ( round( ( ( nodeVar1.xy / vec2<f32>( ( nodeVar1.w * 2.0 ) ) ) * render.nodeUniform11 ) ) / render.nodeUniform11 ) * vec2<f32>( ( nodeVar1.w * 2.0 ) ) ), nodeVar1.zw );
	varyings.v_clipSpace = nodeVar2;
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform12 * normalLocal ), 0.0 ) ).xyz );

	// result

	varyings.builtinClipSpace = nodeVar2;

	return varyings;

}
