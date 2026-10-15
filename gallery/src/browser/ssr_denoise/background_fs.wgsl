// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	@location( 3 ) m3 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_cube<f32>;

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

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) v_normalWorldGeometry : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) positionPrevious : vec3<f32> ) -> OutputType {

	// flow
	// code

	normalWorldGeometry = normalize( v_normalWorldGeometry );
	nodeVar1 = ( object.nodeUniform1 * ( render.nodeUniform2 * vec4<f32>( normalWorldGeometry, 1.0 ) ) );
	nodeVar2 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, vec3<f32>( ( - nodeVar1.x ), nodeVar1.yz ), render.nodeUniform5 );
	DiffuseColor = ( nodeVar2 * vec4<f32>( render.nodeUniform6 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	nodeVar3 = vec4<f32>( ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ), object.nodeUniform8 );
	output.m1 = nodeVar3;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform11 );
	nodeVar4 = ( ( object.nodeUniform9 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar5 = ( ( render.nodeUniform12 * ( object.nodeUniform13 * object.nodeUniform14 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar6 = ( ( nodeVar4.xy / vec2<f32>( nodeVar4.w ) ) - ( nodeVar5.xy / vec2<f32>( nodeVar5.w ) ) );
	output.m2 = vec4<f32>( vec3<f32>( nodeVar6, 0.0 ), 1.0 );
	nodeVar7 = vec4<f32>( DiffuseColor.xyz, object.nodeUniform15 );
	output.m3 = nodeVar7;

	// result

	return output;

}
