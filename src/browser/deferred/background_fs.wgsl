// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_cube<f32>;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform10 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform2 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraProjectionMatrixInverse : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> positionView : vec3<f32>;
var<private> Metalness : f32;
var<private> nodeVar7 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> Roughness : f32;
var<private> nodeVar8 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_normalWorldGeometry : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_clipSpace : vec4<f32> ) -> OutputType {

	// flow
	// code

	normalWorldGeometry = normalize( v_normalWorldGeometry );
	nodeVar1 = ( object.nodeUniform1 * ( render.nodeUniform2 * vec4<f32>( normalWorldGeometry, 1.0 ) ) );
	nodeVar2 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, vec3<f32>( ( - nodeVar1.x ), nodeVar1.yz ), render.nodeUniform5 );
	DiffuseColor = ( nodeVar2 * vec4<f32>( render.nodeUniform6 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = DiffuseColor;
	nodeVar6 = ( render.cameraProjectionMatrixInverse * v_clipSpace );
	positionView = ( nodeVar6.xyz / vec3<f32>( nodeVar6.w ) );
	nodeVar7 = vec4<f32>( positionView, Metalness );
	output.m1 = nodeVar7;
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	nodeVar8 = vec4<f32>( normalView, Roughness );
	output.m2 = nodeVar8;

	// result

	return output;

}
