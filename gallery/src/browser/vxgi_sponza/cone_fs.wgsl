// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : f32,
	@location( 1 ) m1 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_depth_2d;
@binding( 2 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform14_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform14 : texture_3d<f32>;
@binding( 6 ) @group( 0 ) var nodeUniform16_sampler : sampler;
@binding( 7 ) @group( 0 ) var nodeUniform16 : texture_3d<f32>;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : u32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : i32,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec2<u32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
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
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec4<f32>;
var<private> Output : f32;

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
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

	// flow
	// code

	nodeVar1 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar0 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst0 = nodeVar0;

	if ( ( nodeConst0 >= 1.0 ) ) {

		discard;
		

	}

	let nodeConst1 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst0 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst0 ), 1.0 ) ).w ) );
	let nodeConst2 = ( object.nodeUniform2 * vec4<f32>( nodeConst1, 1.0 ) ).xyz;
	nodeVar2 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	let nodeConst3 = normalize( ( ( nodeVar2 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );
	let nodeConst4 = normalize( ( object.nodeUniform2 * vec4<f32>( nodeConst3, 0.0 ) ).xyz );
	let nodeConst5 = ( object.nodeUniform4 * 5.588238 );
	let nodeConst6 = interleavedGradientNoise( ( fragCoord.xy + vec2<f32>( nodeConst5 ) ) );
	let nodeConst7 = interleavedGradientNoise( ( ( fragCoord.xy + vec2<f32>( nodeConst5 ) ) + vec2<f32>( 5.588238, 3.14159 ) ) );

	if ( ( abs( nodeConst4.y ) < 0.99 ) ) {

		nodeVar3 = vec3<f32>( 0.0, 1.0, 0.0 );

	} else {

		nodeVar3 = vec3<f32>( 1.0, 0.0, 0.0 );

	}

	let nodeConst8 = normalize( cross( nodeConst4, nodeVar3 ) );
	let nodeConst9 = cross( nodeConst4, nodeConst8 );
	let nodeConst10 = object.nodeUniform5;
	let nodeConst11 = tan( radians( ( object.nodeUniform6 * 0.5 ) ) );
	let nodeConst12 = ( nodeConst4 * vec3<f32>( ( object.nodeUniform7 * object.nodeUniform8 ) ) );
	nodeVar4 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar5 = 0.0;

	for ( var c : u32 = 0u; c < nodeConst10; c ++ ) {

		let nodeConst13 = ( ( f32( c ) + nodeConst7 ) / f32( nodeConst10 ) );
		let nodeConst14 = fract( ( ( f32( c ) * 0.618034 ) + nodeConst6 ) );
		let nodeConst15 = sqrt( nodeConst13 );
		let nodeConst16 = sqrt( ( 1.0 - nodeConst13 ) );
		let nodeConst17 = ( nodeConst14 * ( 3.141592653589793 * 2.0 ) );
		let nodeConst18 = normalize( ( ( ( nodeConst8 * vec3<f32>( ( cos( nodeConst17 ) * nodeConst15 ) ) ) + ( nodeConst9 * vec3<f32>( ( sin( nodeConst17 ) * nodeConst15 ) ) ) ) + ( nodeConst4 * vec3<f32>( nodeConst16 ) ) ) );
		let nodeConst19 = ( nodeConst2 + nodeConst12 );
		nodeVar6 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar7 = 0.0;
		nodeVar8 = 0.0;
		let nodeConst20 = ( object.nodeUniform9 + object.nodeUniform10 );

		if ( ( abs( nodeConst18.x ) < 0.000001 ) ) {

			nodeVar9 = 0.000001;

		} else {

			nodeVar9 = nodeConst18.x;

		}


		if ( ( abs( nodeConst18.y ) < 0.000001 ) ) {

			nodeVar10 = 0.000001;

		} else {

			nodeVar10 = nodeConst18.y;

		}


		if ( ( abs( nodeConst18.z ) < 0.000001 ) ) {

			nodeVar11 = 0.000001;

		} else {

			nodeVar11 = nodeConst18.z;

		}

		let nodeConst21 = vec3<f32>( nodeVar9, nodeVar10, nodeVar11 );
		let nodeConst22 = ( vec3<f32>( 1.0 ) / nodeConst21 );
		let nodeConst23 = ( ( object.nodeUniform9 - nodeConst19 ) * nodeConst22 );
		let nodeConst24 = ( ( nodeConst20 - nodeConst19 ) * nodeConst22 );
		let nodeConst25 = min( nodeConst23, nodeConst24 );
		let nodeConst26 = max( nodeConst23, nodeConst24 );
		let nodeConst27 = max( max( nodeConst25.x, nodeConst25.y ), max( nodeConst25.z, 0.0 ) );
		let nodeConst28 = min( min( nodeConst26.x, nodeConst26.y ), nodeConst26.z );
		nodeVar12 = max( nodeConst27, object.nodeUniform7 );

		if ( ( object.nodeUniform11 > 0.0 ) ) {

			nodeVar13 = object.nodeUniform11;

		} else {

			nodeVar13 = 10000000000.0;

		}

		let nodeConst29 = min( nodeConst28, nodeVar13 );
		let nodeConst30 = ( nodeConst18 * nodeConst18 );

		if ( ( object.nodeUniform12 > 0.0 ) ) {

			nodeVar14 = ( 1.0 / max( object.nodeUniform12, 0.000001 ) );

		} else {

			nodeVar14 = 0.0;

		}

		let nodeConst31 = nodeVar14;

		if ( ( nodeConst28 > nodeVar12 ) ) {


			for ( var s : i32 = 0; s < 128; s ++ ) {


				if ( ( ( nodeVar12 >= nodeConst29 ) || ( nodeVar7 >= 0.98 ) ) ) {

					break;
					

				}

				let nodeConst32 = max( ( ( nodeVar12 * 2.0 ) * nodeConst11 ), object.nodeUniform7 );
				let nodeConst33 = clamp( log2( ( nodeConst32 / object.nodeUniform7 ) ), 0.0, object.nodeUniform13 );
				let nodeConst34 = ( nodeConst19 + ( nodeConst18 * vec3<f32>( nodeVar12 ) ) );
				let nodeConst35 = ( ( nodeConst34 - object.nodeUniform9 ) / object.nodeUniform10 );
				nodeVar15 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, nodeConst35, nodeConst33 );
				let nodeConst36 = nodeVar15;
				let nodeConst37 = ( 1.0 - pow( ( 1.0 - clamp( dot( nodeConst36.xyz, nodeConst30 ), 0.0, 1.0 ) ), object.nodeUniform15 ) );
				let nodeConst38 = ( nodeConst37 * ( 1.0 - nodeVar7 ) );
				nodeVar16 = textureSampleLevel( nodeUniform16, nodeUniform16_sampler, nodeConst35, nodeConst33 );
				let nodeConst39 = nodeVar16;
				nodeVar6 = ( nodeVar6 + ( ( nodeConst39.xyz / vec3<f32>( max( nodeConst39.w, 0.0001 ) ) ) * vec3<f32>( nodeConst38 ) ) );
				nodeVar7 = ( nodeVar7 + nodeConst38 );
				nodeVar8 = ( nodeVar8 + ( ( nodeConst37 * ( 1.0 - nodeVar8 ) ) / ( ( nodeVar12 * nodeConst31 ) + 1.0 ) ) );
				nodeVar12 = ( nodeVar12 + ( ( object.nodeUniform7 * exp2( nodeConst33 ) ) * object.nodeUniform15 ) );

			}

			

		}

		nodeVar4 = ( nodeVar4 + nodeVar6 );
		nodeVar5 = ( nodeVar5 + nodeVar8 );

	}

	nodeVar4 = ( nodeVar4 / vec3<f32>( f32( nodeConst10 ) ) );
	nodeVar4 = ( nodeVar4 * vec3<f32>( ( object.nodeUniform17 * 3.141592653589793 ) ) );
	nodeVar17 = mix( object.nodeUniform18, 1.0, pow( clamp( ( 1.0 - ( nodeVar5 / f32( nodeConst10 ) ) ), 0.0, 1.0 ), object.nodeUniform19 ) );

	if ( ( f32( object.nodeUniform20 ) > 0.0 ) ) {

		let nodeConst40 = normalize( ( nodeConst2 - object.nodeUniform21 ) );
		let nodeConst41 = length( ( nodeConst2 - object.nodeUniform21 ) );
		let nodeConst42 = object.nodeUniform22;
		let nodeConst43 = ( object.nodeUniform7 * exp2( nodeConst42 ) );
		let nodeConst44 = ( object.nodeUniform10 / vec3<f32>( nodeConst43 ) );
		let nodeConst45 = ( object.nodeUniform9 + object.nodeUniform10 );

		if ( ( abs( nodeConst40.x ) < 0.000001 ) ) {

			nodeVar18 = 0.000001;

		} else {

			nodeVar18 = nodeConst40.x;

		}


		if ( ( abs( nodeConst40.y ) < 0.000001 ) ) {

			nodeVar19 = 0.000001;

		} else {

			nodeVar19 = nodeConst40.y;

		}


		if ( ( abs( nodeConst40.z ) < 0.000001 ) ) {

			nodeVar20 = 0.000001;

		} else {

			nodeVar20 = nodeConst40.z;

		}

		let nodeConst46 = vec3<f32>( nodeVar18, nodeVar19, nodeVar20 );
		let nodeConst47 = ( vec3<f32>( 1.0 ) / nodeConst46 );
		let nodeConst48 = ( ( object.nodeUniform9 - object.nodeUniform21 ) * nodeConst47 );
		let nodeConst49 = ( ( nodeConst45 - object.nodeUniform21 ) * nodeConst47 );
		let nodeConst50 = min( nodeConst48, nodeConst49 );
		let nodeConst51 = max( nodeConst48, nodeConst49 );
		let nodeConst52 = max( max( nodeConst50.x, nodeConst50.y ), max( nodeConst50.z, 0.0 ) );
		let nodeConst53 = min( min( nodeConst51.x, nodeConst51.y ), nodeConst51.z );
		let nodeConst54 = min( nodeConst53, nodeConst41 );
		nodeVar21 = nodeConst52;
		nodeVar4 = vec3<f32>( 0.0 );
		nodeVar17 = 1.0;

		for ( var s : i32 = 0; s < 512; s ++ ) {


			if ( ( nodeVar21 >= nodeConst54 ) ) {

				break;
				

			}

			let nodeConst55 = ( ( floor( ( ( ( ( object.nodeUniform21 + ( nodeConst40 * vec3<f32>( nodeVar21 ) ) ) - object.nodeUniform9 ) / object.nodeUniform10 ) * nodeConst44 ) ) + vec3<f32>( 0.5 ) ) / nodeConst44 );

			if ( ( f32( object.nodeUniform20 ) == 1.0 ) ) {

				nodeVar22 = textureSampleLevel( nodeUniform16, nodeUniform16_sampler, nodeConst55, nodeConst42 );
				let nodeConst56 = nodeVar22;

				if ( ( nodeConst56.w > 0.01 ) ) {

					nodeVar4 = ( nodeConst56.xyz / vec3<f32>( nodeConst56.w ) );
					break;
					

				}

				

			} else {

				nodeVar23 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, nodeConst55, nodeConst42 );
				let nodeConst57 = nodeVar23;

				if ( ( nodeConst57.w > 0.01 ) ) {

					nodeVar4 = nodeConst57.xyz;
					break;
					

				}

				

			}

			nodeVar21 = ( nodeVar21 + ( nodeConst43 * 0.25 ) );

		}

		

	}

	nodeVar24 = nodeVar17;
	nodeVar25 = vec4<f32>( nodeVar4, 1.0 );
	DiffuseColor = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform23 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) ).x;
	output.m0 = nodeVar24;
	output.m1 = nodeVar25;

	// result

	return output;

}
