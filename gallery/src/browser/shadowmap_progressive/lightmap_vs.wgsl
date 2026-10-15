// Three.js r186 - Node System

// directives


// structs


// uniforms

struct renderStruct {
	nodeUniform11 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform42 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	nodeUniform40 : vec3<f32>,
	nodeUniform41 : vec3<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>,
	nodeUniform19 : f32,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : vec2<f32>,
	nodeUniform29 : f32,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform37 : f32,
	nodeUniform38 : vec2<f32>,
	nodeUniform39 : f32,
	nodeUniform43 : mat4x4<f32>,
	nodeUniform44 : f32,
	nodeUniform45 : f32,
	nodeUniform47 : f32,
	nodeUniform48 : vec2<f32>,
	nodeUniform49 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform51 : mat3x3<f32>,
	nodeUniform52 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) nodeVarying5 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar148 : vec2<f32>;
var<private> nodeVar149 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_positionView : vec3<f32>;

// codes


@vertex
fn main( @location( 0 ) normal : vec3<f32>,
	@location( 1 ) position : vec3<f32>,
	@location( 2 ) uv1 : vec2<f32> ) -> VaryingsStruct {

	// flow
	// code

	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform7 * normalLocal ), 0.0 ) ).xyz );
	positionLocal = position;
	varyings.v_positionWorld = ( object.nodeUniform12 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform12 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	varyings.nodeVarying5 = uv1;
	nodeVar148 = uv1;
	nodeVar149 = vec4<f32>( ( ( vec2<f32>( nodeVar148.x, 1.0 - nodeVar148.y ) - vec2<f32>( 0.5, 0.5 ) ) * vec2<f32>( 2.0 ) ), 1.0, 1.0 );

	// result

	varyings.builtinClipSpace = nodeVar149;

	return varyings;

}
