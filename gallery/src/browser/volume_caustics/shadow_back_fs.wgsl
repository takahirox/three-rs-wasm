// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


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
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> Output : vec4<f32>;

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
	let nodeConst0 = ( normalize( refract( ( - positionViewDirection ), normalView, ( 1.0 / 1.5 ) ) ).xy * vec2<f32>( 0.6 ) );
	let nodeConst1 = ( pow( normalView.z, -0.9 ) * 0.004 );
	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeConst0 + vec2<f32>( ( - nodeConst1 ), 0.0 ) ) );
	nodeVar1 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeConst0 + vec2<f32>( 0.0, ( - nodeConst1 ) ) ) );
	nodeVar2 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeConst0 + vec2<f32>( nodeConst1, nodeConst1 ) ) );
	let nodeConst2 = pow( normalView.z, object.nodeUniform4 );
	nodeVar3 = ( ( ( vec3<f32>( nodeVar0.x, nodeVar1.y, nodeVar2.z ) * vec3<f32>( ( nodeConst2 * 60.0 ) ) ) + vec3<f32>( nodeConst2 ) ) * object.nodeUniform5 );
	DiffuseColor = vec4<f32>( nodeVar3, vec4<f32>( nodeVar3, 1.0 ).w );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	let nodeConst3 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst3;

	// result

	output.color = nodeConst3;

	return output;

}
