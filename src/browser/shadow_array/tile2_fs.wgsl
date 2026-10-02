// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform6_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform6 : texture_depth_2d_array;

struct objectStruct {
	nodeUniform0 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	cameraProjectionMatrixInverse : mat4x4<f32>,
	nodeUniform5 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> positionView : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : f32;

// codes


@fragment
fn main( @location( 0 ) v_clipSpace : vec4<f32>,
	@location( 1 ) nodeVarying2 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar4 = ( render.cameraProjectionMatrixInverse * v_clipSpace );
	positionView = ( nodeVar4.xyz / vec3<f32>( nodeVar4.w ) );
	nodeVar5 = vec4<f32>( mix( Output.xyz, render.nodeUniform1, smoothstep( render.nodeUniform2, render.nodeUniform3, ( - positionView.z ) ) ), Output.w );
	Output = nodeVar5;
	nodeVar6 = nodeVarying2;
	nodeVar7 = textureSampleCompare( nodeUniform6, nodeUniform6_sampler, vec2<f32>( nodeVar6.x, 1.0 - nodeVar6.y ), 2, 0.9 );

	// result

	output.color = vec4<f32>( clamp( ( vec3<f32>( 0.8200000000000001, 0.5, 0.85 ) * vec3<f32>( nodeVar7 ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ), 1.0 );

	return output;

}
