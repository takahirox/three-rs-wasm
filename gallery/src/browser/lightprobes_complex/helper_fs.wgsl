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
@binding( 0 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform4 : texture_3d<f32>;

struct objectStruct {
	nodeUniform5 : vec3<f32>,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform10 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;

// codes


@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) nodeVarying5 : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst1 = ( ( nodeVarying5.z * object.nodeUniform5.z ) + 1.0 );
	let nodeConst2 = ( object.nodeUniform5.z + 2.0 );
	let nodeConst3 = ( nodeConst2 * 7.0 );
	nodeVar0 = textureSample( nodeUniform4, nodeUniform4_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeConst1 + ( nodeConst2 * 0.0 ) ) / nodeConst3 ) ) );
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeConst1 + ( nodeConst2 * 1.0 ) ) / nodeConst3 ) ) );
	nodeVar2 = textureSample( nodeUniform4, nodeUniform4_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeConst1 + ( nodeConst2 * 2.0 ) ) / nodeConst3 ) ) );
	nodeVar3 = textureSample( nodeUniform4, nodeUniform4_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeConst1 + ( nodeConst2 * 3.0 ) ) / nodeConst3 ) ) );
	nodeVar4 = textureSample( nodeUniform4, nodeUniform4_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeConst1 + ( nodeConst2 * 4.0 ) ) / nodeConst3 ) ) );
	nodeVar5 = textureSample( nodeUniform4, nodeUniform4_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeConst1 + ( nodeConst2 * 5.0 ) ) / nodeConst3 ) ) );
	nodeVar6 = textureSample( nodeUniform4, nodeUniform4_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeConst1 + ( nodeConst2 * 6.0 ) ) / nodeConst3 ) ) );
	let nodeConst4 = array< vec3<f32>, 9 >( nodeVar0.xyz, vec3<f32>( nodeVar0.w, nodeVar1.xy ), vec3<f32>( nodeVar1.zw, nodeVar2.x ), nodeVar2.yzw, nodeVar3.xyz, vec3<f32>( nodeVar3.w, nodeVar4.xy ), vec3<f32>( nodeVar4.zw, nodeVar5.x ), nodeVar5.yzw, nodeVar6.xyz );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );

	// result

	output.color = vec4<f32>( max( ( ( ( ( ( ( ( ( ( nodeConst4[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst4[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst4[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst4[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst4[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst4[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst4[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst4[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst4[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ), 1.0 );

	return output;

}
