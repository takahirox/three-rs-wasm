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
@binding( 0 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;

// vars
var<private> positionWorldDirection : vec3<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorldDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	positionWorldDirection = normalize( v_positionWorldDirection );
	let nodeConst0 = ( dpdx( positionWorldDirection ) * vec3<f32>( 0.25 ) );
	let nodeConst1 = ( - dpdy( positionWorldDirection ) * vec3<f32>( 0.25 ) );
	let nodeConst2 = normalize( ( ( positionWorldDirection - nodeConst0 ) - nodeConst1 ) );
	nodeVar0 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, vec2<f32>( ( ( atan2( nodeConst2.z, nodeConst2.x ) * 0.15915494309189535 ) + 0.5 ), ( ( asin( clamp( nodeConst2.y, -1.0, 1.0 ) ) * 0.3183098861837907 ) + 0.5 ) ), 0.0 );
	let nodeConst3 = normalize( ( ( positionWorldDirection + nodeConst0 ) - nodeConst1 ) );
	nodeVar1 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, vec2<f32>( ( ( atan2( nodeConst3.z, nodeConst3.x ) * 0.15915494309189535 ) + 0.5 ), ( ( asin( clamp( nodeConst3.y, -1.0, 1.0 ) ) * 0.3183098861837907 ) + 0.5 ) ), 0.0 );
	let nodeConst4 = normalize( ( ( positionWorldDirection - nodeConst0 ) + nodeConst1 ) );
	nodeVar2 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, vec2<f32>( ( ( atan2( nodeConst4.z, nodeConst4.x ) * 0.15915494309189535 ) + 0.5 ), ( ( asin( clamp( nodeConst4.y, -1.0, 1.0 ) ) * 0.3183098861837907 ) + 0.5 ) ), 0.0 );
	let nodeConst5 = normalize( ( ( positionWorldDirection + nodeConst0 ) + nodeConst1 ) );
	nodeVar3 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, vec2<f32>( ( ( atan2( nodeConst5.z, nodeConst5.x ) * 0.15915494309189535 ) + 0.5 ), ( ( asin( clamp( nodeConst5.y, -1.0, 1.0 ) ) * 0.3183098861837907 ) + 0.5 ) ), 0.0 );

	// result

	output.color = vec4<f32>( ( ( ( ( nodeVar0.xyz + nodeVar1.xyz ) + nodeVar2.xyz ) + nodeVar3.xyz ) * vec3<f32>( 0.25 ) ), 1.0 );

	return output;

}
