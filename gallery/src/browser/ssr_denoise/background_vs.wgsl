// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform12 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat4x4<f32>
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
var<private> nodeVar0 : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;

// codes


@vertex
fn main( @location( 0 ) normal : vec3<f32>,
	@location( 1 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	normalLocal = normal;
	nodeVar0 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform4 * normalLocal ), 0.0 ) ).xyz );
	varyings.v_normalViewGeometry = nodeVar0;
	normalViewGeometry = normalize( varyings.v_normalViewGeometry );
	varyings.v_normalWorldGeometry = normalize( ( vec4<f32>( normalViewGeometry, 0.0 ) * render.cameraViewMatrix ).xyz );
	varyings.positionLocal = position;
	varyings.positionPrevious = position;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform17 );

	if ( ( render.cameraProjectionMatrix[ 3u ][ 3u ] == 1.0 ) ) {

		nodeVar8 = ( varyings.positionLocal * vec3<f32>( ( ( 1.0 / render.cameraProjectionMatrix[ 1u ][ 1u ] ) * 3.0 ) ) );

	} else {

		nodeVar8 = varyings.positionLocal;

	}

	nodeVar9 = ( render.cameraProjectionMatrix * vec4<f32>( ( modelViewMatrix * vec4<f32>( nodeVar8, 0.0 ) ).xyz, 1.0 ) );
	nodeVar10 = vec4<f32>( nodeVar9.x, nodeVar9.y, nodeVar9.w, nodeVar9.w );

	// result

	varyings.builtinClipSpace = nodeVar10;

	return varyings;

}
