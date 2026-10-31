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
@binding( 1 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 0 ) var nodeUniform1 : texture_depth_2d;
@binding( 3 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 4 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;
@binding( 5 ) @group( 0 ) var nodeUniform8 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : vec2<f32>,
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
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec2<u32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;

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


fn tsl_repeatWrapping_float( coord: f32 ) -> f32 { return fract( coord ); }
fn tsl_coord_repeatS_repeatT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_repeatWrapping_float( coord.x ),
		tsl_repeatWrapping_float( coord.y )
	);

}

fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn fn0 ( uv : vec2<f32> ) -> f32 {

	


	return fract( ( sin( tsl_mod_float( dot( uv, vec2<f32>( 12.9898, 78.233 ) ), 3.141592653589793 ) ) * 43758.5453 ) );

}


fn fn23 ( clipPosition : vec4<f32> ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	nodeVar0 = ( ( ( clipPosition.xy / vec2<f32>( clipPosition.w ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) );

	return vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) );

}


fn fn134 ( uv : vec2<f32>, centerTexel : vec2<f32>, size : vec2<f32> ) -> bool {

	

	let nodeConst0 = floor( ( uv * size ) );

	return ( ( ( ( ( uv.x > 0.0 ) && ( uv.x < 1.0 ) ) && ( uv.y > 0.0 ) ) && ( uv.y < 1.0 ) ) && ( ( nodeConst0.x != centerTexel.x ) || ( nodeConst0.y != centerTexel.y ) ) );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code


	if ( ( object.nodeUniform0 < 1.0 ) ) {

		nodeVar1 = vec4<f32>( textureGather( nodeUniform1, nodeUniform1_sampler, nodeVarying0) );
		let nodeConst0 = nodeVar1;
		nodeVar0 = min( min( nodeConst0.x, nodeConst0.y ), min( nodeConst0.z, nodeConst0.w ) );

	} else {

		nodeVar3 = textureDimensions( nodeUniform1, u32( 0 ) );
		nodeVar2 = textureLoad( nodeUniform1, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar3 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar3 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar0 = nodeVar2;

	}

	let nodeConst1 = nodeVar0;

	if ( ( nodeConst1 >= 1.0 ) ) {

		discard;
		

	}

	let nodeConst2 = vec2<f32>( textureDimensions( nodeUniform1, 0 ) );
	let nodeConst3 = floor( ( nodeVarying0 * nodeConst2 ) );
	let nodeConst4 = ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst1 ), 1.0 ) );
	let nodeConst5 = ( nodeConst4.xyz / vec3<f32>( nodeConst4.w ) );
	nodeVar4 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	let nodeConst6 = normalize( ( ( nodeVar4 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );
	let nodeConst7 = ( 1.0 / object.nodeUniform4 );
	let nodeConst8 = normalize( ( - nodeConst5 ) );
	let nodeConst9 = ( object.nodeUniform5 * vec4<f32>( nodeConst5, 1.0 ) );
	nodeVar5 = 0.0;
	var nodeVar6 : f32 = fn24( ( fragCoord.xy + vec2<f32>( object.nodeUniform6 ) ) );

	for ( var i : i32 = 0; i < 3; i ++ ) {

		let nodeConst10 = ( ( ( f32( i ) / 3.0 ) * 3.141592653589793 ) + object.nodeUniform7 );
		nodeVar8 = textureDimensions( nodeUniform8, u32( 0 ) );
		nodeVar7 = textureLoad( nodeUniform8, vec2<u32>( clamp( floor( tsl_coord_repeatS_repeatT_2d( ( object.nodeUniform9 * vec3<f32>( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * ( object.nodeUniform10 / vec2<f32>( textureDimensions( nodeUniform8, 0 ) ) ) ), 1.0 ) ).xy ) * vec2<f32>( nodeVar8 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar8 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		let nodeConst11 = normalize( vec3<f32>( ( ( nodeVar7.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ).xy, 0.0 ) );
		let nodeConst12 = ( mat3x3<f32>( nodeConst11, vec3<f32>( ( nodeConst11.y * -1.0 ), nodeConst11.x, 0.0 ), vec3<f32>( 0.0, 0.0, 1.0 ) ) * vec3<f32>( cos( nodeConst10 ), sin( nodeConst10 ), 0.0 ) );
		let nodeConst13 = ( ( object.nodeUniform5 * vec4<f32>( nodeConst12, 0.0 ) ) * vec4<f32>( object.nodeUniform4 ) );
		let nodeConst14 = normalize( cross( nodeConst12, nodeConst8 ) );
		let nodeConst15 = cross( nodeConst14, nodeConst8 );
		let nodeConst16 = ( nodeConst6 - ( nodeConst14 * vec3<f32>( dot( nodeConst6, nodeConst14 ) ) ) );
		let nodeConst17 = length( nodeConst16 );
		let nodeConst18 = ( nodeConst16 / vec3<f32>( max( nodeConst17, 0.0001 ) ) );
		let nodeConst19 = dot( nodeConst18, nodeConst15 );
		let nodeConst20 = clamp( dot( nodeConst18, nodeConst8 ), 0.0, 1.0 );

		if ( ( nodeConst19 >= 0.0 ) ) {

			nodeVar9 = 1.0;

		} else {

			nodeVar9 = -1.0;

		}

		let nodeConst21 = ( nodeVar9 * acos( nodeConst20 ) );
		let nodeConst22 = cross( nodeConst18, nodeConst14 );
		let nodeConst23 = dot( nodeConst8, nodeConst22 );
		nodeVar10 = vec2<f32>( nodeConst23, ( - nodeConst23 ) );

		for ( var j : i32 = 0; j < 6; j ++ ) {

			let nodeConst24 = ( ( ( f32( j ) + 1.0 ) + ( nodeVar6 + fn0( ( ( ( nodeVarying0 + vec2<f32>( ( object.nodeUniform7 * 0.02 ) ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ) ) ) ) * 0.16666666666666666 );
			let nodeConst25 = ( nodeConst13 * vec4<f32>( ( nodeConst24 * nodeConst24 ) ) );
			let nodeConst26 = fn23( ( nodeConst9 + nodeConst25 ) );
			nodeVar3 = textureDimensions( nodeUniform1, u32( 0 ) );
			nodeVar11 = textureLoad( nodeUniform1, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst26 ) * vec2<f32>( nodeVar3 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar3 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst27 = nodeVar11;
			let nodeConst28 = ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst26.x, ( 1.0 - nodeConst26.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst27 ), 1.0 ) );
			let nodeConst29 = ( nodeConst28.xyz / vec3<f32>( nodeConst28.w ) );
			let nodeConst30 = ( nodeConst29 - nodeConst5 );
			let nodeConst31 = length( nodeConst30 );

			if ( ( fn134( nodeConst26, nodeConst3, nodeConst2 ) && ( abs( nodeConst30.z ) < object.nodeUniform11 ) ) ) {

				let nodeConst32 = min( ( nodeConst31 * nodeConst7 ), 1.0 );
				nodeVar10.x = mix( max( nodeVar10.x, ( dot( nodeConst8, nodeConst30 ) / max( nodeConst31, 0.0001 ) ) ), nodeVar10.x, ( nodeConst32 * nodeConst32 ) );
				

			}

			let nodeConst33 = fn23( ( nodeConst9 - nodeConst25 ) );
			nodeVar12 = textureLoad( nodeUniform1, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst33 ) * vec2<f32>( nodeVar3 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar3 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst34 = nodeVar12;
			let nodeConst35 = ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst33.x, ( 1.0 - nodeConst33.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst34 ), 1.0 ) );
			let nodeConst36 = ( nodeConst35.xyz / vec3<f32>( nodeConst35.w ) );
			let nodeConst37 = ( nodeConst36 - nodeConst5 );
			let nodeConst38 = length( nodeConst37 );

			if ( ( fn134( nodeConst33, nodeConst3, nodeConst2 ) && ( abs( nodeConst37.z ) < object.nodeUniform11 ) ) ) {

				let nodeConst39 = min( ( nodeConst38 * nodeConst7 ), 1.0 );
				nodeVar10.y = mix( max( nodeVar10.y, ( dot( nodeConst8, nodeConst37 ) / max( nodeConst38, 0.0001 ) ) ), nodeVar10.y, ( nodeConst39 * nodeConst39 ) );
				

			}


		}

		let nodeConst40 = acos( nodeVar10.y );
		let nodeConst41 = ( - acos( nodeVar10.x ) );
		nodeVar5 = ( nodeVar5 + ( nodeConst17 * ( ( ( ( ( - cos( ( ( nodeConst40 * 2.0 ) - nodeConst21 ) ) ) + nodeConst20 ) + ( ( nodeConst40 * 2.0 ) * nodeConst19 ) ) + ( ( ( - cos( ( ( nodeConst41 * 2.0 ) - nodeConst21 ) ) ) + nodeConst20 ) + ( ( nodeConst41 * 2.0 ) * nodeConst19 ) ) ) * 0.25 ) ) );

	}

	nodeVar5 = clamp( ( nodeVar5 / 3.0 ), 0.0, 1.0 );
	nodeVar5 = pow( nodeVar5, object.nodeUniform12 );

	// result

	output.color = nodeVar5;

	return output;

}
