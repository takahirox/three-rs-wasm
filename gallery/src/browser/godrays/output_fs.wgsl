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
@binding( 1 ) @group( 0 ) var nodeUniform2 : texture_depth_2d;
@binding( 2 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform6_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform6 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform4 : i32,
	nodeUniform5 : vec3<f32>,
	nodeUniform7 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec2<u32>;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec4<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn4 ( color : vec4<f32> ) -> vec4<f32> {

	var nodeVar0 : vec4<f32>;


	if ( ( color.w == 0.0 ) ) {

		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	} else {

		nodeVar0 = vec4<f32>( ( color.xyz / vec3<f32>( color.w ) ), color.w );

	}


	return nodeVar0;

}


fn fn2 ( color : vec3<f32> ) -> vec3<f32> {

	


	return mix( ( ( pow( color, vec3<f32>( 0.41666 ) ) * vec3<f32>( 1.055 ) ) - vec3<f32>( 0.055 ) ), ( color * vec3<f32>( 12.92 ) ), vec3<f32>( ( color <= vec3<f32>( 0.0031308 ) ) ) );

}


fn fn3 ( color : vec4<f32> ) -> vec4<f32> {

	


	return vec4<f32>( ( color.xyz * vec3<f32>( color.w ) ), color.w );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureDimensions( nodeUniform2, u32( 0 ) );
	nodeVar0 = textureLoad( nodeUniform2, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst0 = nodeVar0;
	var nodeVar2 : f32 = ( ( object.nodeUniform0 * object.nodeUniform1 ) / ( ( ( object.nodeUniform1 - object.nodeUniform0 ) * nodeConst0 ) - object.nodeUniform1 ) );
	nodeVar3 = vec2<f32>( 0.0, 0.0 );
	nodeVar4 = 0.0;
	let nodeConst1 = vec2<i32>( textureDimensions( nodeUniform3, 0 ) );

	for ( var i : i32 = 0; i < 8; i ++ ) {

		let nodeConst2 = ( array< vec2<f32>, 8 >( vec2<f32>( 0.493393, 0.394269 ), vec2<f32>( 0.798547, 0.885922 ), vec2<f32>( 0.259143, 0.650754 ), vec2<f32>( 0.605322, 0.023588 ), vec2<f32>( -0.574681, 0.137452 ), vec2<f32>( -0.430397, -0.638423 ), vec2<f32>( -0.849487, -0.366258 ), vec2<f32>( 0.170621, -0.569941 ) )[ i ] * vec2<f32>( f32( object.nodeUniform4 ) ) );
		nodeVar5 = textureLoad( nodeUniform2, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( nodeVarying0 + ( nodeConst2 * ( vec2<f32>( 1.0, 1.0 ) / vec2<f32>( nodeConst1 ) ) ) ) ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		let nodeConst3 = ( ( nodeVar2 + object.nodeUniform0 ) / ( object.nodeUniform0 - object.nodeUniform1 ) );

		if ( ( abs( ( ( ( ( ( object.nodeUniform0 * object.nodeUniform1 ) / ( ( ( object.nodeUniform1 - object.nodeUniform0 ) * nodeVar5 ) - object.nodeUniform1 ) ) + object.nodeUniform0 ) / ( object.nodeUniform0 - object.nodeUniform1 ) ) - nodeConst3 ) ) < ( 0.05 * nodeConst3 ) ) ) {

			nodeVar3 = ( nodeVar3 + nodeConst2 );
			nodeVar4 = ( nodeVar4 + 1.0 );
			

		}


	}


	if ( ( nodeVar4 == 0.0 ) ) {

		nodeVar6 = 1.0;

	} else {

		nodeVar6 = nodeVar4;

	}

	nodeVar4 = nodeVar6;
	nodeVar3 = ( nodeVar3 / vec2<f32>( nodeVar4 ) );
	nodeVar7 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );

	if ( ( length( nodeVar3 ) > 0.0 ) ) {

		nodeVar8 = ( nodeVarying0 + ( vec2<f32>( object.nodeUniform7 ) * ( nodeVar3 / vec2<f32>( nodeConst1 ) ) ) );

	} else {

		nodeVar8 = nodeVarying0;

	}

	nodeVar9 = textureSample( nodeUniform6, nodeUniform6_sampler, nodeVar8 );
	let nodeConst4 = mix( nodeVar7, vec4<f32>( object.nodeUniform5, 1.0 ), nodeVar9.x );
	let nodeConst5 = fn4( vec4<f32>( nodeConst4.xyz, clamp( nodeConst4.w, 0.0, 1.0 ) ) );

	// result

	output.color = fn3( vec4<f32>( fn2( nodeConst5.xyz ), nodeConst5.w ) );

	return output;

}
