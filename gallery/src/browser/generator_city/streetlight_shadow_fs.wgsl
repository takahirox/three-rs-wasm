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
	nodeUniform4 : f32,
	nodeUniform7 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> Output : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32 ) -> OutputStruct {

	// flow
	// code


	if ( ( nodeVarying3 == 1.0 ) ) {

		nodeVar0 = vec3<f32>( 1.0, 0.8713671191959567, 0.6038273388475408 );

	} else {

		nodeVar0 = vec3<f32>( 0.05126945836711539, 0.05612849004241121, 0.04666508633021928 );

	}

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( nodeVar0, 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	let nodeConst1 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst1;

	// result

	output.color = nodeConst1;

	return output;

}
