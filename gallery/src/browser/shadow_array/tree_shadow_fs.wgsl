// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: f32
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	nodeUniform3 : f32,
	nodeUniform7 : mat4x4<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : f32;
var<private> nodeVar7 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) @interpolate(flat, either) vBatchIndirectId : u32,
	@location( 1 ) vBatchColor : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = ( vBatchColor * vec4<f32>( 0.0, 0.0, 0.0, 1.0 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	nodeVar7 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar7.x;

	// result

	output.color = nodeVar7.x;

	return output;

}
