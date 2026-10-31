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
var<private> positionView : vec3<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> nodeVar2 : f32;

// codes


@fragment
fn main( @location( 0 ) v_clipSpace : vec4<f32>,
	@location( 1 ) nodeVarying2 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	let nodeConst4 = ( render.cameraProjectionMatrixInverse * v_clipSpace );
	positionView = ( nodeConst4.xyz / vec3<f32>( nodeConst4.w ) );
	nodeVar0 = vec4<f32>( mix( Output.xyz, render.nodeUniform1, smoothstep( render.nodeUniform2, render.nodeUniform3, ( - positionView.z ) ) ), Output.w );
	Output = nodeVar0;
	nodeVar1 = nodeVarying2;
	nodeVar2 = textureSampleCompare( nodeUniform6, nodeUniform6_sampler, vec2<f32>( nodeVar1.x, 1.0 - nodeVar1.y ), 3, 0.9 );

	// result

	output.color = vec4<f32>( clamp( ( vec3<f32>( 0.5, 0.75, 0.9249999999999999 ) * vec3<f32>( nodeVar2 ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ), 1.0 );

	return output;

}
