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
@binding( 1 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform3 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> positionWorldDirection : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : u32;
var<private> nodeVar8 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorldDirection : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	positionWorldDirection = normalize( v_positionWorldDirection );
	nodeVar0 = positionWorldDirection;
	nodeVar1 = vec3<f32>( 0.0, 0.0, 0.0 );

	if ( ( object.nodeUniform1 < 0.001 ) ) {

		nodeVar2 = textureSampleLevel( nodeUniform2, nodeUniform2_sampler, vec3<f32>( ( - nodeVar0.x ), nodeVar0.yz ), 0.0 );
		nodeVar1 = nodeVar2.xyz;
		

	} else {

		let nodeConst0 = ( object.nodeUniform1 * object.nodeUniform1 );
		let nodeConst1 = ( nodeConst0 * nodeConst0 );

		if ( ( abs( nodeVar0.z ) < 0.999 ) ) {

			nodeVar3 = vec3<f32>( 0.0, 0.0, 1.0 );

		} else {

			nodeVar3 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar4 = normalize( cross( nodeVar3, nodeVar0 ) );
		nodeVar5 = cross( nodeVar0, nodeVar4 );
		nodeVar6 = 0.0;

		for ( var i : i32 = 0; i < 256; i ++ ) {

			nodeVar7 = u32( i );
			nodeVar7 = ( ( nodeVar7 << 16u ) | ( nodeVar7 >> 16u ) );
			nodeVar7 = ( ( ( nodeVar7 & 1431655765u ) << 1u ) | ( ( nodeVar7 & 2863311530u ) >> 1u ) );
			nodeVar7 = ( ( ( nodeVar7 & 858993459u ) << 2u ) | ( ( nodeVar7 & 3435973836u ) >> 2u ) );
			nodeVar7 = ( ( ( nodeVar7 & 252645135u ) << 4u ) | ( ( nodeVar7 & 4042322160u ) >> 4u ) );
			nodeVar7 = ( ( ( nodeVar7 & 16711935u ) << 8u ) | ( ( nodeVar7 & 4278255360u ) >> 8u ) );
			let nodeConst2 = vec2<f32>( ( f32( i ) / 256.0 ), ( f32( nodeVar7 ) * 2.3283064365386963e-10 ) );
			let nodeConst3 = ( 1.0 / ( ( 1.0 - nodeConst2.x ) + ( nodeConst1 * nodeConst2.x ) ) );
			let nodeConst4 = ( ( ( 1.0 - nodeConst2.x ) - ( nodeConst1 * nodeConst2.x ) ) * nodeConst3 );

			if ( ( nodeConst4 > 0.0 ) ) {

				let nodeConst5 = ( nodeConst2.y * 6.283185307179586 );
				let nodeConst6 = ( ( ( nodeConst0 * 2.0 ) * sqrt( ( nodeConst2.x * ( 1.0 - nodeConst2.x ) ) ) ) * nodeConst3 );
				let nodeConst7 = ( ( nodeVar0 * vec3<f32>( nodeConst4 ) ) + ( ( ( nodeVar4 * vec3<f32>( cos( nodeConst5 ) ) ) + ( nodeVar5 * vec3<f32>( sin( nodeConst5 ) ) ) ) * vec3<f32>( nodeConst6 ) ) );
				nodeVar8 = textureSampleLevel( nodeUniform2, nodeUniform2_sampler, vec3<f32>( ( - nodeConst7.x ), nodeConst7.yz ), max( ( log2( ( nodeConst1 * nodeConst3 ) ) + object.nodeUniform3 ), 0.0 ) );
				nodeVar1 = ( nodeVar1 + ( nodeVar8.xyz * vec3<f32>( nodeConst4 ) ) );
				nodeVar6 = ( nodeVar6 + nodeConst4 );
				

			}


		}

		nodeVar1 = ( nodeVar1 / vec3<f32>( nodeVar6 ) );
		

	}


	// result

	output.color = vec4<f32>( nodeVar1, 1.0 );

	return output;

}
