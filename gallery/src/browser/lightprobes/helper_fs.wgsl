// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform1 : texture_3d<f32>;

struct objectStruct {
	nodeUniform2 : vec3<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform7 : mat4x4<f32>
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
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : array< vec3<f32>, 9 >;
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

	nodeVar0 = ( ( nodeVarying5.z * object.nodeUniform2.z ) + 1.0 );
	nodeVar1 = ( object.nodeUniform2.z + 2.0 );
	nodeVar2 = ( nodeVar1 * 7.0 );
	nodeVar3 = textureSample( nodeUniform1, nodeUniform1_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeVar0 + ( nodeVar1 * 0.0 ) ) / nodeVar2 ) ) );
	nodeVar4 = textureSample( nodeUniform1, nodeUniform1_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeVar0 + ( nodeVar1 * 1.0 ) ) / nodeVar2 ) ) );
	nodeVar5 = textureSample( nodeUniform1, nodeUniform1_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeVar0 + ( nodeVar1 * 2.0 ) ) / nodeVar2 ) ) );
	nodeVar6 = textureSample( nodeUniform1, nodeUniform1_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeVar0 + ( nodeVar1 * 3.0 ) ) / nodeVar2 ) ) );
	nodeVar7 = textureSample( nodeUniform1, nodeUniform1_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeVar0 + ( nodeVar1 * 4.0 ) ) / nodeVar2 ) ) );
	nodeVar8 = textureSample( nodeUniform1, nodeUniform1_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeVar0 + ( nodeVar1 * 5.0 ) ) / nodeVar2 ) ) );
	nodeVar9 = textureSample( nodeUniform1, nodeUniform1_sampler, vec3<f32>( nodeVarying5.xy, ( ( nodeVar0 + ( nodeVar1 * 6.0 ) ) / nodeVar2 ) ) );
	nodeVar10 = array< vec3<f32>, 9 >( nodeVar3.xyz, vec3<f32>( nodeVar3.w, nodeVar4.xy ), vec3<f32>( nodeVar4.zw, nodeVar5.x ), nodeVar5.yzw, nodeVar6.xyz, vec3<f32>( nodeVar6.w, nodeVar7.xy ), vec3<f32>( nodeVar7.zw, nodeVar8.x ), nodeVar8.yzw, nodeVar9.xyz );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );

	// result

	output.color = vec4<f32>( max( ( ( ( ( ( ( ( ( ( nodeVar10[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar10[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar10[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar10[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar10[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar10[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar10[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar10[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar10[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ), 1.0 );

	return output;

}
