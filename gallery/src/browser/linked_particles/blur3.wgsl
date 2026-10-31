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
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform2 : vec2<f32>,
	nodeUniform3 : vec2<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( nodeVarying0, 1.0 ) ).xy );
	nodeVar1 = ( nodeVar0.xyz * vec3<f32>( 0.19947 ) );

	for ( var i : i32 = 0; i < 3; i ++ ) {

		let nodeConst0 = ( ( object.nodeUniform2 * object.nodeUniform3 ) * vec2<f32>( array< f32, 3 >( 1.40733340004593, 3.294214972162989, 5.0 )[ i ] ) );
		nodeVar2 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVarying0 + nodeConst0 ), 1.0 ) ).xy );
		nodeVar3 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( ( nodeVarying0 - nodeConst0 ), 1.0 ) ).xy );
		nodeVar1 = ( nodeVar1 + ( ( nodeVar2.xyz + nodeVar3.xyz ) * vec3<f32>( array< f32, 3 >( 0.2970163278514283, 0.09175375661117716, 0.008764100149861079 )[ i ] ) ) );

	}


	// result

	output.color = vec4<f32>( nodeVar1, 1.0 );

	return output;

}
