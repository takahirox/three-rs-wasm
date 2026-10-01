// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform3 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform6 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> positionLocal : vec3<f32>;

// codes


@vertex
fn main( @location( 0 ) normal : vec3<f32>,
	@location( 1 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform1 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform6 );

	if ( ( render.cameraProjectionMatrix[ 3u ][ 3u ] == 1.0 ) ) {

		positionLocal = position;
		nodeVar7 = ( positionLocal * vec3<f32>( ( ( 1.0 / render.cameraProjectionMatrix[ 1u ][ 1u ] ) * 3.0 ) ) );

	} else {

		positionLocal = position;
		nodeVar7 = positionLocal;

	}

	nodeVar8 = ( render.cameraProjectionMatrix * vec4<f32>( ( modelViewMatrix * vec4<f32>( nodeVar7, 0.0 ) ).xyz, 1.0 ) );
	nodeVar9 = vec4<f32>( nodeVar8.x, nodeVar8.y, nodeVar8.w, nodeVar8.w );

	// result

	varyings.builtinClipSpace = nodeVar9;

	return varyings;

}
