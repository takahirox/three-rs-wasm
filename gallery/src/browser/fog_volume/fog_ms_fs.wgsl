// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: f32
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform4 : texture_depth_multisampled_2d;
@binding( 2 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 3 ) @group( 1 ) var nodeUniform9 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform15 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform3 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec2<u32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;

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
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureDimensions( nodeUniform4 );
	nodeVar0 = textureLoad( nodeUniform4, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform3 ) ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar2 = ( ( object.nodeUniform1 * vec4<f32>( ( ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( ( fragCoord.xy / render.nodeUniform3 ).x, ( 1.0 - ( fragCoord.xy / render.nodeUniform3 ).y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar0 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( ( fragCoord.xy / render.nodeUniform3 ).x, ( 1.0 - ( fragCoord.xy / render.nodeUniform3 ).y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar0 ), 1.0 ) ).w ) ), 1.0 ) ).xyz - object.nodeUniform0 );
	nodeVar3 = normalize( nodeVar2 );

	if ( ( nodeVar3.y >= 0.0 ) ) {

		nodeVar4 = max( nodeVar3.y, 0.00001 );

	} else {

		nodeVar4 = min( nodeVar3.y, -0.00001 );

	}

	nodeVar5 = ( ( -1.0 - object.nodeUniform0.y ) / nodeVar4 );
	nodeVar6 = ( ( ( -1.0 + object.nodeUniform5 ) - object.nodeUniform0.y ) / nodeVar4 );
	nodeVar7 = max( min( nodeVar5, nodeVar6 ), 0.0 );
	nodeVar8 = length( nodeVar2 );
	nodeVar9 = ( max( ( min( min( max( nodeVar5, nodeVar6 ), nodeVar8 ), ( nodeVar7 + object.nodeUniform6 ) ) - nodeVar7 ), 0.0 ) / object.nodeUniform7 );
	nodeVar10 = ( nodeVar3 * vec3<f32>( nodeVar9 ) );
	nodeVar11 = ( ( object.nodeUniform0 + ( nodeVar3 * vec3<f32>( nodeVar7 ) ) ) + ( nodeVar10 * vec3<f32>( interleavedGradientNoise( fragCoord.xy ) ) ) );
	nodeVar12 = 0.0;

	for ( var i : i32 = 0; i < i32( object.nodeUniform7 ); i ++ ) {

		nodeVar13 = ( nodeVar11 * vec3<f32>( object.nodeUniform8 ) );
		nodeVar14 = textureSample( nodeUniform9, nodeUniform9_sampler, ( nodeVar13 + vec3<f32>( ( object.nodeUniform10 * 0.3 ), ( object.nodeUniform10 * 0.05 ), ( object.nodeUniform10 * 0.2 ) ) ) ).x;
		nodeVar15 = textureSample( nodeUniform9, nodeUniform9_sampler, ( ( ( ( nodeVar13 * vec3<f32>( 2.2 ) ) + vec3<f32>( 1.7, 0.9, 2.5 ) ) + vec3<f32>( ( ( - object.nodeUniform10 ) * 0.15 ), ( object.nodeUniform10 * 0.1 ), ( object.nodeUniform10 * 0.08 ) ) ) + vec3<f32>( ( nodeVar14 * 0.5 ) ) ) ).x;
		nodeVar12 = ( nodeVar12 + ( ( ( max( ( ( nodeVar14 + ( nodeVar15 * 0.5 ) ) - object.nodeUniform11 ), 0.0 ) * pow( clamp( ( 1.0 - ( ( nodeVar11.y - -1.0 ) / max( object.nodeUniform5, 0.01 ) ) ), 0.0, 1.0 ), object.nodeUniform12 ) ) * nodeVar9 ) * object.nodeUniform13 ) );
		nodeVar11 = ( nodeVar11 + nodeVar10 );

	}


	// result

	output.color = max( ( 1.0 - exp( ( - nodeVar12 ) ) ), clamp( ( ( nodeVar8 - object.nodeUniform14 ) / max( ( object.nodeUniform15 - object.nodeUniform14 ), 0.001 ) ), 0.0, 1.0 ) );

	return output;

}
