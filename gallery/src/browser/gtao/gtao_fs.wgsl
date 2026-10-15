// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: f32
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 0 ) var nodeUniform1 : texture_depth_2d;
@binding( 3 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 4 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;
@binding( 5 ) @group( 0 ) var nodeUniform7 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : vec2<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<u32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec2<u32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec2<u32>;
var<private> nodeVar13 : f32;
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

fn tsl_repeatWrapping_float( coord: f32 ) -> f32 { return fract( coord ); }
fn tsl_coord_repeatS_repeatT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_repeatWrapping_float( coord.x ),
		tsl_repeatWrapping_float( coord.y )
	);

}

fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn getScreenPositionFromClip ( clipPosition : vec4<f32> ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	nodeVar0 = ( ( ( clipPosition.xy / vec2<f32>( clipPosition.w ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) );

	return vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code


	if ( ( object.nodeUniform0 < 1.0 ) ) {

		nodeVar1 = vec4<f32>( textureGather( nodeUniform1, nodeUniform1_sampler, nodeVarying0) );
		nodeVar0 = min( min( nodeVar1.x, nodeVar1.y ), min( nodeVar1.z, nodeVar1.w ) );

	} else {

		nodeVar3 = textureDimensions( nodeUniform1, u32( 0 ) );
		nodeVar2 = textureLoad( nodeUniform1, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar3 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar3 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar0 = nodeVar2;

	}

	let nodeConst0 = nodeVar0;

	if ( ( nodeConst0 >= 1.0 ) ) {

		discard;
		

	}

	let nodeConst1 = ( ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst0 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst0 ), 1.0 ) ).w ) );
	nodeVar4 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	let nodeConst2 = normalize( ( ( nodeVar4 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );
	let nodeConst3 = ( 1.0 / object.nodeUniform4 );
	let nodeConst4 = normalize( ( - nodeConst1 ) );
	let nodeConst5 = ( object.nodeUniform5 * vec4<f32>( nodeConst1, 1.0 ) );
	nodeVar5 = 0.0;

	for ( var i : i32 = 0; i < 3; i ++ ) {

		let nodeConst6 = ( ( ( f32( i ) / 3.0 ) * 3.141592653589793 ) + object.nodeUniform6 );
		nodeVar7 = textureDimensions( nodeUniform7, u32( 0 ) );
		nodeVar6 = textureLoad( nodeUniform7, vec2<u32>( clamp( floor( tsl_coord_repeatS_repeatT_2d( ( object.nodeUniform8 * vec3<f32>( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * ( object.nodeUniform9 / vec2<f32>( textureDimensions( nodeUniform7, 0 ) ) ) ), 1.0 ) ).xy ) * vec2<f32>( nodeVar7 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar7 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar8 = normalize( vec3<f32>( ( ( nodeVar6.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ).xy, 0.0 ) );
		let nodeConst7 = ( mat3x3<f32>( nodeVar8, vec3<f32>( ( nodeVar8.y * -1.0 ), nodeVar8.x, 0.0 ), vec3<f32>( 0.0, 0.0, 1.0 ) ) * vec3<f32>( cos( nodeConst6 ), sin( nodeConst6 ), 0.0 ) );
		let nodeConst8 = ( ( object.nodeUniform5 * vec4<f32>( nodeConst7, 0.0 ) ) * vec4<f32>( object.nodeUniform4 ) );
		let nodeConst9 = normalize( cross( nodeConst7, nodeConst4 ) );
		let nodeConst10 = cross( nodeConst9, nodeConst4 );
		let nodeConst11 = ( nodeConst2 - ( nodeConst9 * vec3<f32>( dot( nodeConst2, nodeConst9 ) ) ) );
		let nodeConst12 = length( nodeConst11 );
		let nodeConst13 = ( nodeConst11 / vec3<f32>( max( nodeConst12, 0.0001 ) ) );
		let nodeConst14 = dot( nodeConst13, nodeConst10 );
		let nodeConst15 = clamp( dot( nodeConst13, nodeConst4 ), 0.0, 1.0 );

		if ( ( nodeConst14 >= 0.0 ) ) {

			nodeVar9 = 1.0;

		} else {

			nodeVar9 = -1.0;

		}

		let nodeConst16 = ( nodeVar9 * acos( nodeConst15 ) );
		let nodeConst17 = cross( nodeConst13, nodeConst9 );
		let nodeConst18 = dot( nodeConst4, nodeConst17 );
		nodeVar10 = vec2<f32>( nodeConst18, ( - nodeConst18 ) );

		for ( var j : i32 = 0; j < 6; j ++ ) {

			let nodeConst19 = ( ( ( f32( j ) + 1.0 ) + ( interleavedGradientNoise( ( fragCoord.xy + vec2<f32>( object.nodeUniform10 ) ) ) + fract( ( sin( tsl_mod_float( dot( ( ( ( nodeVarying0 + vec2<f32>( ( object.nodeUniform6 * 0.02 ) ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), vec2<f32>( 12.9898, 78.233 ) ), 3.141592653589793 ) ) * 43758.5453 ) ) ) ) * 0.16666666666666666 );
			let nodeConst20 = ( nodeConst8 * vec4<f32>( ( nodeConst19 * nodeConst19 ) ) );
			let nodeConst21 = getScreenPositionFromClip( ( nodeConst5 + nodeConst20 ) );
			nodeVar12 = textureDimensions( nodeUniform1, u32( 0 ) );
			nodeVar11 = textureLoad( nodeUniform1, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst21 ) * vec2<f32>( nodeVar12 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar12 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst22 = nodeVar11;
			let nodeConst23 = ( ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst21.x, ( 1.0 - nodeConst21.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst22 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst21.x, ( 1.0 - nodeConst21.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst22 ), 1.0 ) ).w ) );
			let nodeConst24 = ( nodeConst23 - nodeConst1 );
			let nodeConst25 = length( nodeConst24 );

			if ( ( abs( nodeConst24.z ) < object.nodeUniform11 ) ) {

				nodeVar13 = min( ( nodeConst25 * nodeConst3 ), 1.0 );
				nodeVar10.x = mix( max( nodeVar10.x, ( dot( nodeConst4, nodeConst24 ) / max( nodeConst25, 0.0001 ) ) ), nodeVar10.x, ( nodeVar13 * nodeVar13 ) );
				

			}

			let nodeConst26 = getScreenPositionFromClip( ( nodeConst5 - nodeConst20 ) );
			nodeVar14 = textureLoad( nodeUniform1, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst26 ) * vec2<f32>( nodeVar12 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar12 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst27 = nodeVar14;
			let nodeConst28 = ( ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst26.x, ( 1.0 - nodeConst26.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst27 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst26.x, ( 1.0 - nodeConst26.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst27 ), 1.0 ) ).w ) );
			let nodeConst29 = ( nodeConst28 - nodeConst1 );
			let nodeConst30 = length( nodeConst29 );

			if ( ( abs( nodeConst29.z ) < object.nodeUniform11 ) ) {

				nodeVar15 = min( ( nodeConst30 * nodeConst3 ), 1.0 );
				nodeVar10.y = mix( max( nodeVar10.y, ( dot( nodeConst4, nodeConst29 ) / max( nodeConst30, 0.0001 ) ) ), nodeVar10.y, ( nodeVar15 * nodeVar15 ) );
				

			}


		}

		let nodeConst31 = acos( nodeVar10.y );
		let nodeConst32 = ( - acos( nodeVar10.x ) );
		nodeVar5 = ( nodeVar5 + ( nodeConst12 * ( ( ( ( ( - cos( ( ( nodeConst31 * 2.0 ) - nodeConst16 ) ) ) + nodeConst15 ) + ( ( nodeConst31 * 2.0 ) * nodeConst14 ) ) + ( ( ( - cos( ( ( nodeConst32 * 2.0 ) - nodeConst16 ) ) ) + nodeConst15 ) + ( ( nodeConst32 * 2.0 ) * nodeConst14 ) ) ) * 0.25 ) ) );

	}

	nodeVar5 = clamp( ( nodeVar5 / 3.0 ), 0.0, 1.0 );
	nodeVar5 = pow( nodeVar5, object.nodeUniform12 );

	// result

	output.color = nodeVar5;

	return output;

}
