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
	nodeUniform0 : f32,
	nodeUniform6 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> r2 : f32;
var<private> Output : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) vSplatUv : vec2<f32>,
	@location( 1 ) vSplatColor : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	r2 = dot( vSplatUv, vSplatUv );

	if ( ( r2 > 4.0 ) ) {

		discard;
		

	}

	DiffuseColor = vec4<f32>( vSplatColor.xyz, ( exp( ( r2 * -0.5 ) ) * vSplatColor.w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	let nodeConst0 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst0;

	// result

	output.color = nodeConst0;

	return output;

}
