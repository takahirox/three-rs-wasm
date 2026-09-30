// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;

struct objectStruct {
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;

// codes

@fragment
fn main( @location( 0 ) v_positionViewDirection : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	positionViewDirection = normalize( v_positionViewDirection );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	nodeVar0 = ( normalize( refract( ( - positionViewDirection ), normalView, ( 1.0 / 1.5 ) ) ).xy * vec2<f32>( 0.6 ) );
	nodeVar1 = ( pow( normalView.z, -0.9 ) * 0.004 );
	nodeVar2 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVar0 + vec2<f32>( ( - nodeVar1 ), 0.0 ) ) );
	nodeVar3 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVar0 + vec2<f32>( 0.0, ( - nodeVar1 ) ) ) );
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVar0 + vec2<f32>( nodeVar1, nodeVar1 ) ) );
	nodeVar5 = pow( normalView.z, object.nodeUniform4 );
	nodeVar6 = ( ( ( vec3<f32>( nodeVar2.x, nodeVar3.y, nodeVar4.z ) * vec3<f32>( ( nodeVar5 * 25.0 ) ) ) + vec3<f32>( nodeVar5 ) ) * object.nodeUniform5 );
	DiffuseColor = vec4<f32>( nodeVar6, vec4<f32>( nodeVar6, 1.0 ).w );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	nodeVar7 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar7;

	// result

	output.color = nodeVar7;

	return output;

}
