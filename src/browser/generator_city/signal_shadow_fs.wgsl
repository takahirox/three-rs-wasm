// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform4 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32 ) -> OutputStruct {

	// flow
	// code


	if ( ( nodeVarying3 == 1.0 ) ) {

		nodeVar0 = vec3<f32>( 0.04231141061442144, 0.0030352698352941175, 0.0018211619011764706 );

	} else {


		if ( ( nodeVarying3 == 2.0 ) ) {

			nodeVar1 = vec3<f32>( 0.05126945836711539, 0.019382360952473074, 0.002428215868235294 );

		} else {


			if ( ( nodeVarying3 == 3.0 ) ) {

				nodeVar2 = vec3<f32>( 0.024157632443547246, 0.6444796819634361, 0.07036009568874305 );

			} else {

				nodeVar2 = vec3<f32>( 0.01680737574872402, 0.02955683443236377, 0.02217388478862708 );

			}

			nodeVar1 = nodeVar2;

		}

		nodeVar0 = nodeVar1;

	}

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( nodeVar0, 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	nodeVar3 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar3;

	// result

	output.color = nodeVar3;

	return output;

}
