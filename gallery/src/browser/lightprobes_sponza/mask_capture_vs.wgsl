// Three.js r187 - Node System

// directives


// structs


// uniforms

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 2 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct cameraProjectionMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 3 ) @group( 0 )
var<uniform> cameraProjectionMatrices : cameraProjectionMatricesStruct;

struct cameraIndexStruct {
	u_cameraIndex : u32
};
@binding( 0 ) @group( 1 )
var<uniform> cameraIndex : cameraIndexStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform19 : vec2<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : vec3<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) @interpolate(flat, either) v_cameraIndex : u32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_tangentView : vec3<f32>,
	@location( 3 ) v_bitangentView : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 7 ) nodeVarying11 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> tangentLocal : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> VERTEX_normalView : vec3<f32>;
var<private> VERTEX_tangentView : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> positionLocal : vec3<f32>;
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
	varyings.v_cameraIndex = cameraIndex.u_cameraIndex;
	normalLocal = normal;
	let nodeConst0 = normalize( ( cameraViewMatrices.value[ varyings.v_cameraIndex ] * vec4<f32>( ( object.nodeUniform12 * normalLocal ), 0.0 ) ).xyz );
	varyings.v_normalViewGeometry = nodeConst0;
	modelViewMatrix = ( cameraViewMatrices.value[ varyings.v_cameraIndex ] * object.nodeUniform16 );
	tangentLocal = tangent.xyz;
	varyings.v_tangentView = ( modelViewMatrix * vec4<f32>( tangentLocal, 0.0 ) ).xyz;
	normalViewGeometry = normalize( varyings.v_normalViewGeometry );
	let nodeConst3 = ( ( f32( true ) * 2.0 ) - 1.0 );
	VERTEX_normalView = ( normalViewGeometry * vec3<f32>( nodeConst3 ) );
	VERTEX_tangentView = ( normalize( varyings.v_tangentView ) * vec3<f32>( nodeConst3 ) );
	varyings.NORMAL_v_bitangentView = ( cross( VERTEX_normalView, VERTEX_tangentView ) * vec3<f32>( tangent.w ) );
	positionLocal = position;
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	varyings.v_positionWorld = ( object.nodeUniform16 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let VERTEX_nodeConst32 = ( cameraProjectionMatrices.value[ varyings.v_cameraIndex ] * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst32;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
