// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform0 : f32,
	nodeUniform4 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform1 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : mat3x3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_normalWorldGeometry : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_normalViewGeometry : vec3<f32>;

// codes


@vertex
fn main( @location( 0 ) normal : vec3<f32>,
	@location( 1 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	normalLocal = normal;
	v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform3 * normalLocal ), 0.0 ) ).xyz );
	normalViewGeometry = normalize( v_normalViewGeometry );
	varyings.v_normalWorldGeometry = normalize( ( vec4<f32>( normalViewGeometry, 0.0 ) * render.cameraViewMatrix ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform7 );

	if ( ( render.cameraProjectionMatrix[ 3u ][ 3u ] == 1.0 ) ) {

		positionLocal = position;
		nodeVar5 = ( positionLocal * vec3<f32>( ( ( 1.0 / render.cameraProjectionMatrix[ 1u ][ 1u ] ) * 3.0 ) ) );

	} else {

		positionLocal = position;
		nodeVar5 = positionLocal;

	}

	nodeVar6 = ( render.cameraProjectionMatrix * vec4<f32>( ( modelViewMatrix * vec4<f32>( nodeVar5, 0.0 ) ).xyz, 1.0 ) );
	nodeVar7 = vec4<f32>( nodeVar6.x, nodeVar6.y, nodeVar6.w, nodeVar6.w );

	// result

	varyings.builtinClipSpace = nodeVar7;

	return varyings;

}
