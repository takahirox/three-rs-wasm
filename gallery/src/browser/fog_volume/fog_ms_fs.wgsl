// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


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
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}




@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureDimensions( nodeUniform4 );
	nodeVar0 = textureLoad( nodeUniform4, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform3 ) ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst0 = ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( ( fragCoord.xy / render.nodeUniform3 ).x, ( 1.0 - ( fragCoord.xy / render.nodeUniform3 ).y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar0 ), 1.0 ) );
	let nodeConst1 = ( ( object.nodeUniform1 * vec4<f32>( ( nodeConst0.xyz / vec3<f32>( nodeConst0.w ) ), 1.0 ) ).xyz - object.nodeUniform0 );
	let nodeConst2 = normalize( nodeConst1 );

	if ( ( nodeConst2.y >= 0.0 ) ) {

		nodeVar2 = max( nodeConst2.y, 0.00001 );

	} else {

		nodeVar2 = min( nodeConst2.y, -0.00001 );

	}

	let nodeConst3 = ( ( -1.0 - object.nodeUniform0.y ) / nodeVar2 );
	let nodeConst4 = ( ( ( -1.0 + object.nodeUniform5 ) - object.nodeUniform0.y ) / nodeVar2 );
	let nodeConst5 = max( min( nodeConst3, nodeConst4 ), 0.0 );
	let nodeConst6 = length( nodeConst1 );
	let nodeConst7 = ( max( ( min( min( max( nodeConst3, nodeConst4 ), nodeConst6 ), ( nodeConst5 + object.nodeUniform6 ) ) - nodeConst5 ), 0.0 ) / object.nodeUniform7 );
	let nodeConst8 = ( nodeConst2 * vec3<f32>( nodeConst7 ) );
	nodeVar3 = ( ( object.nodeUniform0 + ( nodeConst2 * vec3<f32>( nodeConst5 ) ) ) + ( nodeConst8 * vec3<f32>( fn24( fragCoord.xy ) ) ) );
	nodeVar4 = 0.0;

	for ( var i : i32 = 0; i < i32( object.nodeUniform7 ); i ++ ) {

		nodeVar5 = ( nodeVar3 * vec3<f32>( object.nodeUniform8 ) );
		nodeVar6 = textureSample( nodeUniform9, nodeUniform9_sampler, ( nodeVar5 + vec3<f32>( ( object.nodeUniform10 * 0.3 ), ( object.nodeUniform10 * 0.05 ), ( object.nodeUniform10 * 0.2 ) ) ) ).x;
		nodeVar7 = textureSample( nodeUniform9, nodeUniform9_sampler, ( ( ( ( nodeVar5 * vec3<f32>( 2.2 ) ) + vec3<f32>( 1.7, 0.9, 2.5 ) ) + vec3<f32>( ( ( - object.nodeUniform10 ) * 0.15 ), ( object.nodeUniform10 * 0.1 ), ( object.nodeUniform10 * 0.08 ) ) ) + vec3<f32>( ( nodeVar6 * 0.5 ) ) ) ).x;
		nodeVar4 = ( nodeVar4 + ( ( ( max( ( ( nodeVar6 + ( nodeVar7 * 0.5 ) ) - object.nodeUniform11 ), 0.0 ) * pow( clamp( ( 1.0 - ( ( nodeVar3.y - -1.0 ) / max( object.nodeUniform5, 0.01 ) ) ), 0.0, 1.0 ), object.nodeUniform12 ) ) * nodeConst7 ) * object.nodeUniform13 ) );
		nodeVar3 = ( nodeVar3 + nodeConst8 );

	}


	// result

	output.color = max( ( 1.0 - exp( ( - nodeVar4 ) ) ), clamp( ( ( nodeConst6 - object.nodeUniform14 ) / max( ( object.nodeUniform15 - object.nodeUniform14 ), 0.001 ) ), 0.0, 1.0 ) );

	return output;

}
