// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;

struct objectStruct {
	nodeUniform6 : u32,
	nodeUniform13 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) @interpolate(flat, either) vInstId : u32,
	@location( 1 ) @interpolate(flat, either) vMegaTriIdx : u32,
	@location( 2 ) vUv : vec2<f32>,
	@location( 3 ) vNormal : vec3<f32>,
	@location( 4 ) vTangent : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar11 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	if ( ( f32( object.nodeUniform6 ) == 1.0 ) ) {

		nodeVar11 = vec4<f32>( ( ( normalize( vNormal ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ), 1.0 );
		

	} else {


		if ( ( f32( object.nodeUniform6 ) == 2.0 ) ) {

			nodeVar12 = normalize( vTangent );
			nodeVar13 = textureSample( nodeUniform7, nodeUniform7_sampler, vUv );
			nodeVar14 = ( ( nodeVar13.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) );
			nodeVar15 = normalize( vNormal );
			nodeVar11 = vec4<f32>( ( ( normalize( ( ( ( nodeVar12 * vec3<f32>( nodeVar14.x ) ) + ( cross( nodeVar15, nodeVar12 ) * vec3<f32>( nodeVar14.y ) ) ) + ( nodeVar15 * vec3<f32>( nodeVar14.z ) ) ) ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ), 1.0 );
			

		} else {


			if ( ( f32( object.nodeUniform6 ) == 3.0 ) ) {

				nodeVar11 = vec4<f32>( vUv, 0.0, 1.0 );
				

			} else {


				if ( ( f32( object.nodeUniform6 ) == 4.0 ) ) {

					nodeVar16 = textureSample( nodeUniform8, nodeUniform8_sampler, vUv );
					nodeVar11 = vec4<f32>( nodeVar16.y, nodeVar16.y, nodeVar16.y, 1.0 );
					

				} else {


					if ( ( f32( object.nodeUniform6 ) == 5.0 ) ) {

						nodeVar17 = textureSample( nodeUniform8, nodeUniform8_sampler, vUv );
						nodeVar11 = vec4<f32>( nodeVar17.z, nodeVar17.z, nodeVar17.z, 1.0 );
						

					} else {


						if ( ( f32( object.nodeUniform6 ) == 6.0 ) ) {

							nodeVar18 = textureSample( nodeUniform9, nodeUniform9_sampler, vUv );
							nodeVar11 = vec4<f32>( nodeVar18.x, nodeVar18.x, nodeVar18.x, 1.0 );
							

						} else {


							if ( ( f32( object.nodeUniform6 ) == 7.0 ) ) {

								nodeVar19 = textureSample( nodeUniform10, nodeUniform10_sampler, vUv );
								nodeVar11 = vec4<f32>( nodeVar19.xyz, 1.0 );
								

							}

							

						}

						

					}

					

				}

				

			}

			

		}

		

	}


	// result

	output.color = nodeVar11;

	return output;

}
