// Three.js r187 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform1 : mat4x4<f32>,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform12 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform16 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) v_normalWorldGeometry : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) positionPrevious : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar1 : vec3<f32>;

// codes


@vertex
fn main( @location( 0 ) normal : vec3<f32>,
	@location( 1 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	normalLocal = normal;
	let nodeConst0 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform3 * normalLocal ), 0.0 ) ).xyz );
	varyings.v_normalViewGeometry = nodeConst0;
	normalViewGeometry = normalize( varyings.v_normalViewGeometry );
	varyings.v_normalWorldGeometry = normalize( ( vec4<f32>( normalViewGeometry, 0.0 ) * render.cameraViewMatrix ).xyz );
	varyings.positionLocal = position;
	varyings.positionPrevious = position;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform16 );

	if ( ( render.cameraProjectionMatrix[ 3u ][ 3u ] == 1.0 ) ) {

		varyings.positionLocal = position;
		nodeVar1 = ( varyings.positionLocal * vec3<f32>( ( ( 1.0 / render.cameraProjectionMatrix[ 1u ][ 1u ] ) * 3.0 ) ) );

	} else {

		varyings.positionLocal = position;
		nodeVar1 = varyings.positionLocal;

	}

	let nodeConst7 = ( render.cameraProjectionMatrix * vec4<f32>( ( modelViewMatrix * vec4<f32>( nodeVar1, 0.0 ) ).xyz, 1.0 ) );
	let nodeConst8 = vec4<f32>( nodeConst7.x, nodeConst7.y, nodeConst7.w, nodeConst7.w );

	// result

	varyings.builtinClipSpace = nodeConst8;

	return varyings;

}
