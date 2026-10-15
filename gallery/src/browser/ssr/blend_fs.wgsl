// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_2d<f32>;
@binding( 3 ) @group( 0 ) var nodeUniform5_sampler : sampler;
@binding( 4 ) @group( 0 ) var nodeUniform5 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : vec2<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform6 : vec2<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec4<f32>,
	@location( 1 ) nodeVarying1 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
	nodeVar1 = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
	nodeVar2 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( nodeVarying1, 1.0 ) ).xy );
	nodeVar3 = nodeVar2.xz;
	nodeVar1.x = nodeVar3[ 0 ];
	nodeVar1.z = nodeVar3[ 1 ];
	nodeVar5 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying0.zw, 1.0 ) ).xy );
	nodeVar1.y = nodeVar5.y;
	nodeVar6 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform4 * vec3<f32>( nodeVarying0.xy, 1.0 ) ).xy );
	nodeVar1.w = nodeVar6.w;

	if ( ( dot( nodeVar1, vec4<f32>( 1.0, 1.0, 1.0, 1.0 ) ) < 0.00001 ) ) {

		nodeVar7 = textureSample( nodeUniform5, nodeUniform5_sampler, nodeVarying1 );
		nodeVar0 = nodeVar7;
		

	} else {

		nodeVar8 = vec2<f32>( 0.0, 0.0 );

		if ( ( nodeVar1.w > nodeVar1.z ) ) {

			nodeVar9 = nodeVar1.w;

		} else {

			nodeVar9 = ( - nodeVar1.z );

		}

		nodeVar8.x = nodeVar9;

		if ( ( nodeVar1.y > nodeVar1.x ) ) {

			nodeVar10 = nodeVar1.y;

		} else {

			nodeVar10 = ( - nodeVar1.x );

		}

		nodeVar8.y = nodeVar10;

		if ( ( abs( nodeVar8.x ) > abs( nodeVar8.y ) ) ) {

			nodeVar8.y = 0.0;
			

		} else {

			nodeVar8.x = 0.0;
			

		}

		nodeVar11 = textureSample( nodeUniform5, nodeUniform5_sampler, nodeVarying1 );
		nodeVar12 = nodeVar11;
		nodeVar13 = nodeVarying1;
		nodeVar13 = ( nodeVar13 + ( sign( nodeVar8 ) * object.nodeUniform6 ) );
		nodeVar14 = textureSample( nodeUniform5, nodeUniform5_sampler, nodeVar13 );
		nodeVar15 = nodeVar14;

		if ( ( abs( nodeVar8.x ) > abs( nodeVar8.y ) ) ) {

			nodeVar16 = abs( nodeVar8.x );

		} else {

			nodeVar16 = abs( nodeVar8.y );

		}

		nodeVar17 = nodeVar16;
		nodeVar0 = mix( nodeVar12, nodeVar15, nodeVar17 );
		

	}


	// result

	output.color = nodeVar0;

	return output;

}
