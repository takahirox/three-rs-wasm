// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0 : texture_depth_2d;
@binding( 4 ) @group( 1 ) var nodeUniform8_sampler : sampler_comparison;
@binding( 5 ) @group( 1 ) var nodeUniform8 : texture_depth_cube;

struct NodeBuffer_1020Struct {
	value : array< vec4<f32>, 6 >
};
@binding( 2 ) @group( 1 )
var<uniform> NodeBuffer_1020 : NodeBuffer_1020Struct;

struct NodeBuffer_1021Struct {
	value : array< vec4<f32>, 6 >
};
@binding( 3 ) @group( 1 )
var<uniform> NodeBuffer_1021 : NodeBuffer_1021Struct;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32
};
@binding( 1 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform7 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : bool;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<u32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	

	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}

@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
	nodeVar1 = false;
	nodeVar3 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar2 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar3 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar3 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst0 = nodeVar2;
	let nodeConst1 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst0 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst0 ), 1.0 ) ).w ) );
	nodeVar4 = ( object.nodeUniform2 * vec4<f32>( nodeConst1, 1.0 ) );
	nodeVar5 = -10000.0;

	for ( var i : i32 = 0; i < 6; i ++ ) {

		nodeVar5 = max( nodeVar5, ( dot( object.nodeUniform3, NodeBuffer_1020.value[ i ].xyz ) + NodeBuffer_1021.value[ i ].x ) );

	}

	nodeVar6 = object.nodeUniform3;

	if ( ( nodeVar5 < 0.0 ) ) {

		for ( var i : i32 = 0; i < 6; i ++ ) {

			if ( ( ( dot( nodeVar4, vec4<f32>( NodeBuffer_1020.value[ i ].xyz, 1.0 ) ) + NodeBuffer_1021.value[ i ].x ) > 0.0 ) ) {

				let nodeConst2 = ( nodeVar4 - vec4<f32>( object.nodeUniform3, 1.0 ) );
				nodeVar4 = ( vec4<f32>( object.nodeUniform3, 1.0 ) + ( vec4<f32>( ( - ( ( dot( object.nodeUniform3, NodeBuffer_1020.value[ i ].xyz ) + NodeBuffer_1021.value[ i ].x ) / dot( vec4<f32>( NodeBuffer_1020.value[ i ].xyz, 1.0 ), nodeConst2 ) ) ) ) * nodeConst2 ) );
				

			}

		}

		

	} else {

		let nodeConst3 = ( nodeVar4 - vec4<f32>( object.nodeUniform3, 1.0 ) );
		nodeVar7 = 10000.0;

		for ( var i : i32 = 0; i < 6; i ++ ) {

			nodeVar8 = ( - ( ( dot( object.nodeUniform3, NodeBuffer_1020.value[ i ].xyz ) + NodeBuffer_1021.value[ i ].x ) / dot( vec4<f32>( NodeBuffer_1020.value[ i ].xyz, 1.0 ), nodeConst3 ) ) );

			if ( ( ( nodeVar8 < nodeVar7 ) && ( nodeVar8 > 0.0 ) ) ) {

				nodeVar7 = nodeVar8;
				

			}

		}

		if ( ( nodeVar7 == 10000.0 ) ) {

			nodeVar1 = true;
			

		} else {

			nodeVar6 = ( vec4<f32>( object.nodeUniform3, 1.0 ) + ( vec4<f32>( ( nodeVar7 + 0.001 ) ) * nodeConst3 ) ).xyz;
			nodeVar9 = -10000.0;

			for ( var i : i32 = 0; i < 6; i ++ ) {

				nodeVar9 = max( nodeVar9, ( dot( nodeVar4, vec4<f32>( NodeBuffer_1020.value[ i ].xyz, 1.0 ) ) + NodeBuffer_1021.value[ i ].x ) );

			}

			if ( ( nodeVar9 >= 0.0 ) ) {

				nodeVar10 = 10000.0;

				for ( var i : i32 = 0; i < 6; i ++ ) {

					if ( ( ( dot( nodeVar4, vec4<f32>( NodeBuffer_1020.value[ i ].xyz, 1.0 ) ) + NodeBuffer_1021.value[ i ].x ) > 0.0 ) ) {

						nodeVar11 = ( - ( ( dot( nodeVar6, NodeBuffer_1020.value[ i ].xyz ) + NodeBuffer_1021.value[ i ].x ) / dot( vec4<f32>( NodeBuffer_1020.value[ i ].xyz, 1.0 ), nodeConst3 ) ) );

						if ( ( ( nodeVar11 < nodeVar10 ) && ( nodeVar11 > 0.0 ) ) ) {

							nodeVar10 = nodeVar11;
							

						}

						

					}

				}

				if ( ( nodeVar10 < distance( nodeVar4, vec4<f32>( nodeVar6, 1.0 ) ) ) ) {

					nodeVar4 = ( vec4<f32>( nodeVar6, 1.0 ) + ( vec4<f32>( nodeVar10 ) * nodeConst3 ) );
					

				}

				

			}

			

		}

		

	}

	if ( ( nodeVar1 == false ) ) {

		nodeVar12 = 0.0;
		let nodeConst4 = interleavedGradientNoise( fragCoord.xy );
		let nodeConst5 = round( ( object.nodeUniform6 + ( ( ( object.nodeUniform6 / 8.0 ) + 2.0 ) * nodeConst4 ) ) );
		let nodeConst6 = u32( nodeConst5 );

		for ( var i : i32 = 0; i < i32( nodeConst6 ); i ++ ) {

			let nodeConst7 = mix( vec4<f32>( nodeVar6, 1.0 ), nodeVar4, ( f32( i ) / nodeConst5 ) );
			let nodeConst8 = ( nodeConst7 - vec4<f32>( render.nodeUniform7, 1.0 ) );
			let nodeConst9 = abs( nodeConst8 );
			nodeVar13 = ( - max( max( nodeConst9.x, nodeConst9.y ), nodeConst9.z ) );
			nodeVar14 = textureSampleCompare( nodeUniform8, nodeUniform8_sampler, vec3<f32>( nodeConst8.x, ( - nodeConst8.y ), nodeConst8.z ), ( ( ( object.nodeUniform9 + nodeVar13 ) * object.nodeUniform10 ) / ( ( object.nodeUniform10 - object.nodeUniform9 ) * nodeVar13 ) ) );
			nodeVar15 = vec2<f32>( ( ( 1.0 - nodeVar14 ) + 0.005 ), ( - nodeVar13 ) );
			let nodeConst10 = ( 1.0 - nodeVar15.x );
			nodeVar12 = ( nodeVar12 + ( ( nodeConst10 * ( distance( vec4<f32>( nodeVar6, 1.0 ), nodeVar4 ) * ( object.nodeUniform11 / 100.0 ) ) ) * pow( ( 1.0 - ( nodeVar15.y / object.nodeUniform10 ) ), object.nodeUniform12 ) ) );

		}

		nodeVar12 = ( nodeVar12 / nodeConst5 );
		nodeVar0 = vec4<f32>( vec3<f32>( clamp( ( 1.0 - exp( ( - nodeVar12 ) ) ), 0.0, object.nodeUniform13 ) ), nodeConst0 );
		

	}

	// result

	output.color = nodeVar0;

	return output;

}
