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

struct objectStruct {
	nodeUniform3 : f32,
	nodeUniform7 : mat4x4<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) @interpolate(flat, either) vBatchIndirectId : u32,
	@location( 1 ) vBatchColor : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = ( vec4<f32>( 0.0, 0.0, 0.0, 1.0 ) * vBatchColor );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	let nodeConst14 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst14;

	// result

	output.color = nodeConst14;

	return output;

}
