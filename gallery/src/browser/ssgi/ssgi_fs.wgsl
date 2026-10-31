// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputType {
	@location( 0 ) m0 : f32,
	@location( 1 ) m1 : vec3<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 0 ) var nodeUniform1 : texture_depth_2d;
@binding( 3 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 4 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;
@binding( 5 ) @group( 0 ) var nodeUniform19_sampler : sampler;
@binding( 6 ) @group( 0 ) var nodeUniform19 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform4 : u32,
	nodeUniform5 : u32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : u32,
	nodeUniform10 : vec2<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : u32,
	nodeUniform18 : f32,
	nodeUniform20 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<u32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : bool;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : bool;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : u32;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : u32;
var<private> nodeVar37 : vec4<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec3<f32>;
var<private> Output : f32;

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


fn fn134 ( position : vec2<f32> ) -> f32 {

	


	return ( 0.25 * f32( ( i32( ( position.y - position.x ) ) & 3 ) ) );

}


fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn fn0 ( uv : vec2<f32> ) -> f32 {

	


	return fract( ( sin( tsl_mod_float( dot( uv, vec2<f32>( 12.9898, 78.233 ) ), 3.141592653589793 ) ) * 43758.5453 ) );

}


fn fn135 ( value : vec2<f32> ) -> vec2<f32> {

	var nodeVar1 : f32;
	var nodeVar2 : f32;

	var nodeVar0 : vec2<f32> = ( ( abs( value ) * vec2<f32>( -0.156583 ) ) + vec2<f32>( 1.5707963267948966 ) );
	nodeVar0 = ( nodeVar0 * sqrt( ( vec2<f32>( 1.0 ) - abs( value ) ) ) );

	if ( ( value.x >= 0.0 ) ) {

		nodeVar1 = nodeVar0.x;

	} else {

		nodeVar1 = ( 3.141592653589793 - nodeVar0.x );

	}


	if ( ( value.y >= 0.0 ) ) {

		nodeVar2 = nodeVar0.y;

	} else {

		nodeVar2 = ( 3.141592653589793 - nodeVar0.y );

	}


	return vec2<f32>( nodeVar1, nodeVar2 );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

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

	let nodeConst2 = ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst1 ), 1.0 ) );
	nodeVar4 = ( nodeConst2.xyz / vec3<f32>( nodeConst2.w ) );
	nodeVar5 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	nodeVar6 = normalize( ( ( nodeVar5 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );
	nodeVar7 = normalize( ( - nodeVar4 ) );
	var nodeVar8 : f32 = fn24( fragCoord.xy );
	var nodeVar9 : f32 = 0.0;
	var nodeVar10 : vec3<f32> = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst3 = object.nodeUniform4;
	let nodeConst4 = object.nodeUniform5;
	let nodeConst5 = object.nodeUniform6;
	let nodeConst6 = object.nodeUniform7;
	let nodeConst7 = object.nodeUniform8;
	nodeVar11 = 0.0;
	nodeVar12 = bool( object.nodeUniform9 );

	if ( nodeVar12 ) {

		nodeVar11 = ( ( nodeConst7 * ( object.nodeUniform10.x / 2.0 ) ) / 16.0 );
		

	} else {

		nodeVar11 = max( ( ( nodeConst7 * object.nodeUniform11 ) / ( - nodeVar4.z ) ), f32( nodeConst4 ) );
		

	}

	nodeVar11 = ( nodeVar11 / ( f32( nodeConst4 ) + 1.0 ) );
	let nodeConst8 = ( max( 1.0, f32( ( nodeConst4 - 1u ) ) ) * nodeVar11 );
	var nodeVar13 : u32 = 0u;

	for ( var i : u32 = 0u; i < nodeConst3; i ++ ) {

		let nodeConst9 = ( ( ( f32( i ) + nodeVar8 ) + object.nodeUniform12 ) * ( 3.141592653589793 / f32( nodeConst3 ) ) );
		let nodeConst10 = vec3<f32>( vec2<f32>( cos( nodeConst9 ), sin( nodeConst9 ) ), 0.0 );
		let nodeConst11 = ( nodeConst10.xy * ( vec2<f32>( 1.0 ) / object.nodeUniform10 ) );
		let nodeConst12 = normalize( cross( nodeConst10, nodeVar7 ) );
		let nodeConst13 = cross( nodeVar7, nodeConst12 );
		let nodeConst14 = ( nodeVar6 - ( nodeConst12 * vec3<f32>( dot( nodeVar6, nodeConst12 ) ) ) );
		let nodeConst15 = normalize( nodeConst14 );
		let nodeConst16 = clamp( dot( nodeConst15, nodeVar7 ), -1.0, 1.0 );
		let nodeConst17 = ( ( - sign( dot( nodeConst14, nodeConst13 ) ) ) * acos( nodeConst16 ) );
		nodeVar13 = 0u;
		let nodeConst18 = object.nodeUniform13;
		let nodeConst19 = object.nodeUniform14;
		let nodeConst20 = object.nodeUniform15;
		var nodeVar14 : vec3<f32> = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst21 = object.nodeUniform5;

		for ( var i : u32 = 0u; i < nodeConst21; i ++ ) {

			let nodeConst22 = ( fract( ( fn134( fragCoord.xy ) + object.nodeUniform16 ) ) + fn0( ( ( ( nodeVarying0 + vec2<f32>( ( object.nodeUniform12 * 0.02 ) ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ) ) );
			let nodeConst23 = ( pow( abs( ( ( nodeVar11 * ( f32( i ) + nodeConst22 ) ) / nodeConst8 ) ), nodeConst18 ) * nodeConst8 );
			let nodeConst24 = ( nodeConst11 * vec2<f32>( max( nodeConst23, ( f32( i ) + 1.0 ) ) ) );

			if ( true ) {

				nodeVar15 = vec2<f32>( 1.0, -1.0 );

			} else {

				nodeVar15 = vec2<f32>( -1.0, 1.0 );

			}

			let nodeConst25 = ( nodeVarying0 + ( nodeConst24 * nodeVar15 ) );

			if ( ( ( ( ( nodeConst25.x <= 0.0 ) || ( nodeConst25.y <= 0.0 ) ) || ( nodeConst25.x >= 1.0 ) ) || ( nodeConst25.y >= 1.0 ) ) ) {

				break;
				

			}

			nodeVar3 = textureDimensions( nodeUniform1, u32( 0 ) );
			nodeVar16 = textureLoad( nodeUniform1, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst25 ) * vec2<f32>( nodeVar3 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar3 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst26 = ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst25.x, ( 1.0 - nodeConst25.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar16 ), 1.0 ) );
			let nodeConst27 = ( nodeConst26.xyz / vec3<f32>( nodeConst26.w ) );
			let nodeConst28 = normalize( ( nodeConst27 - nodeVar4 ) );

			if ( true ) {


				if ( true ) {

					nodeVar18 = 1.0;

				} else {

					nodeVar18 = -1.0;

				}

				nodeVar20 = bool( object.nodeUniform17 );

				if ( nodeVar20 ) {

					nodeVar19 = ( clamp( ( ( - nodeConst27.z ) / object.nodeUniform18 ), 0.0, 1.0 ) * 100.0 );

				} else {

					nodeVar19 = 1.0;

				}

				let nodeConst29 = clamp( ( ( ( vec2<f32>( nodeVar18 ) * ( - fn135( clamp( vec2<f32>( dot( nodeConst28, nodeVar7 ), dot( normalize( ( ( nodeConst27 - ( ( vec3<f32>( nodeVar19 ) * nodeVar7 ) * vec3<f32>( nodeConst19 ) ) ) - nodeVar4 ) ), nodeVar7 ) ), vec2<f32>( -1.0 ), vec2<f32>( 1.0 ) ) ) ) ) - vec2<f32>( ( nodeConst17 - 1.5707963267948966 ) ) ) / vec2<f32>( 3.141592653589793 ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) );
				nodeVar17 = nodeConst29.yx;

			} else {


				if ( true ) {

					nodeVar21 = 1.0;

				} else {

					nodeVar21 = -1.0;

				}

				nodeVar20 = bool( object.nodeUniform17 );

				if ( nodeVar20 ) {

					nodeVar22 = ( clamp( ( ( - nodeConst27.z ) / object.nodeUniform18 ), 0.0, 1.0 ) * 100.0 );

				} else {

					nodeVar22 = 1.0;

				}

				let nodeConst29 = clamp( ( ( ( vec2<f32>( nodeVar21 ) * ( - fn135( clamp( vec2<f32>( dot( nodeConst28, nodeVar7 ), dot( normalize( ( ( nodeConst27 - ( ( vec3<f32>( nodeVar22 ) * nodeVar7 ) * vec3<f32>( nodeConst19 ) ) ) - nodeVar4 ) ), nodeVar7 ) ), vec2<f32>( -1.0 ), vec2<f32>( 1.0 ) ) ) ) ) - vec2<f32>( ( nodeConst17 - 1.5707963267948966 ) ) ) / vec2<f32>( 3.141592653589793 ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) );
				nodeVar17 = nodeConst29;

			}

			let nodeConst30 = nodeVar17.x;
			let nodeConst31 = nodeVar17.y;
			let nodeConst32 = u32( ( nodeVar17 * vec2<f32>( 32.0 ) ).x );
			let nodeConst33 = u32( ceil( ( ( nodeConst31 - nodeConst30 ) * 32.0 ) ) );

			if ( ( nodeConst33 > 0u ) ) {

				nodeVar23 = ( 4294967295u >> ( ( 32u - 32u ) + ( 32u - nodeConst33 ) ) );

			} else {

				nodeVar23 = 0u;

			}

			let nodeConst34 = nodeVar23;
			let nodeConst35 = ( ( nodeConst34 << nodeConst32 ) & ( ~ nodeVar13 ) );
			nodeVar13 = ( nodeVar13 | nodeConst35 );
			let nodeConst36 = countOneBits( nodeConst35 );

			if ( ( nodeConst36 > 0u ) ) {

				nodeVar24 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeConst25 );
				let nodeConst37 = nodeVar24;

				if ( ( dot( nodeConst37, vec4<f32>( vec3<f32>( 0.2126, 0.7152, 0.0722 ), 1.0 ) ) > 0.001 ) ) {

					let nodeConst38 = normalize( nodeConst28 );
					let nodeConst39 = clamp( dot( nodeVar6, nodeConst38 ), 0.0, 1.0 );

					if ( ( nodeConst39 > 0.001 ) ) {

						nodeVar26 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeConst25 );
						let nodeConst40 = normalize( ( ( nodeVar26 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );

						if ( ( ( nodeConst20 > 0.0 ) && ( dot( nodeConst40, nodeVar7 ) > 0.0 ) ) ) {

							let nodeConst41 = dot( nodeConst40, ( - nodeConst38 ) );

							if ( ( sign( nodeConst41 ) < 0.0 ) ) {

								nodeVar27 = ( abs( nodeConst41 ) * nodeConst20 );

							} else {

								nodeVar27 = abs( nodeConst41 );

							}

							nodeVar25 = nodeVar27;

						} else {

							let nodeConst41 = dot( nodeConst40, ( - nodeConst38 ) );
							nodeVar25 = clamp( nodeConst41, 0.0, 1.0 );

						}

						nodeVar14 = ( vec4<f32>( nodeVar14, 1.0 ) + ( ( ( vec4<f32>( ( f32( nodeConst36 ) / 32.0 ) ) * nodeConst37 ) * vec4<f32>( nodeConst39 ) ) * vec4<f32>( nodeVar25 ) ) ).xyz;
						

					}

					

				}

				

			}


		}

		nodeVar10 = ( nodeVar10 + nodeVar14 );
		let nodeConst42 = object.nodeUniform13;
		let nodeConst43 = object.nodeUniform14;
		let nodeConst44 = object.nodeUniform15;
		var nodeVar28 : vec3<f32> = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst45 = object.nodeUniform5;

		for ( var i : u32 = 0u; i < nodeConst45; i ++ ) {

			let nodeConst22 = ( fract( ( fn134( fragCoord.xy ) + object.nodeUniform16 ) ) + fn0( ( ( ( nodeVarying0 + vec2<f32>( ( object.nodeUniform12 * 0.02 ) ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ) ) );
			let nodeConst46 = ( pow( abs( ( ( nodeVar11 * ( f32( i ) + nodeConst22 ) ) / nodeConst8 ) ), nodeConst42 ) * nodeConst8 );
			let nodeConst47 = ( nodeConst11 * vec2<f32>( max( nodeConst46, ( f32( i ) + 1.0 ) ) ) );

			if ( false ) {

				nodeVar29 = vec2<f32>( 1.0, -1.0 );

			} else {

				nodeVar29 = vec2<f32>( -1.0, 1.0 );

			}

			let nodeConst48 = ( nodeVarying0 + ( nodeConst47 * nodeVar29 ) );

			if ( ( ( ( ( nodeConst48.x <= 0.0 ) || ( nodeConst48.y <= 0.0 ) ) || ( nodeConst48.x >= 1.0 ) ) || ( nodeConst48.y >= 1.0 ) ) ) {

				break;
				

			}

			nodeVar3 = textureDimensions( nodeUniform1, u32( 0 ) );
			nodeVar30 = textureLoad( nodeUniform1, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst48 ) * vec2<f32>( nodeVar3 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar3 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst49 = ( object.nodeUniform2 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst48.x, ( 1.0 - nodeConst48.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar30 ), 1.0 ) );
			let nodeConst50 = ( nodeConst49.xyz / vec3<f32>( nodeConst49.w ) );
			let nodeConst51 = normalize( ( nodeConst50 - nodeVar4 ) );

			if ( false ) {


				if ( false ) {

					nodeVar32 = 1.0;

				} else {

					nodeVar32 = -1.0;

				}

				nodeVar20 = bool( object.nodeUniform17 );

				if ( nodeVar20 ) {

					nodeVar33 = ( clamp( ( ( - nodeConst50.z ) / object.nodeUniform18 ), 0.0, 1.0 ) * 100.0 );

				} else {

					nodeVar33 = 1.0;

				}

				let nodeConst52 = clamp( ( ( ( vec2<f32>( nodeVar32 ) * ( - fn135( clamp( vec2<f32>( dot( nodeConst51, nodeVar7 ), dot( normalize( ( ( nodeConst50 - ( ( vec3<f32>( nodeVar33 ) * nodeVar7 ) * vec3<f32>( nodeConst43 ) ) ) - nodeVar4 ) ), nodeVar7 ) ), vec2<f32>( -1.0 ), vec2<f32>( 1.0 ) ) ) ) ) - vec2<f32>( ( nodeConst17 - 1.5707963267948966 ) ) ) / vec2<f32>( 3.141592653589793 ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) );
				nodeVar31 = nodeConst52.yx;

			} else {


				if ( false ) {

					nodeVar34 = 1.0;

				} else {

					nodeVar34 = -1.0;

				}

				nodeVar20 = bool( object.nodeUniform17 );

				if ( nodeVar20 ) {

					nodeVar35 = ( clamp( ( ( - nodeConst50.z ) / object.nodeUniform18 ), 0.0, 1.0 ) * 100.0 );

				} else {

					nodeVar35 = 1.0;

				}

				let nodeConst52 = clamp( ( ( ( vec2<f32>( nodeVar34 ) * ( - fn135( clamp( vec2<f32>( dot( nodeConst51, nodeVar7 ), dot( normalize( ( ( nodeConst50 - ( ( vec3<f32>( nodeVar35 ) * nodeVar7 ) * vec3<f32>( nodeConst43 ) ) ) - nodeVar4 ) ), nodeVar7 ) ), vec2<f32>( -1.0 ), vec2<f32>( 1.0 ) ) ) ) ) - vec2<f32>( ( nodeConst17 - 1.5707963267948966 ) ) ) / vec2<f32>( 3.141592653589793 ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) );
				nodeVar31 = nodeConst52;

			}

			let nodeConst53 = nodeVar31.x;
			let nodeConst54 = nodeVar31.y;
			let nodeConst55 = u32( ( nodeVar31 * vec2<f32>( 32.0 ) ).x );
			let nodeConst56 = u32( ceil( ( ( nodeConst54 - nodeConst53 ) * 32.0 ) ) );

			if ( ( nodeConst56 > 0u ) ) {

				nodeVar36 = ( 4294967295u >> ( ( 32u - 32u ) + ( 32u - nodeConst56 ) ) );

			} else {

				nodeVar36 = 0u;

			}

			let nodeConst57 = nodeVar36;
			let nodeConst58 = ( ( nodeConst57 << nodeConst55 ) & ( ~ nodeVar13 ) );
			nodeVar13 = ( nodeVar13 | nodeConst58 );
			let nodeConst59 = countOneBits( nodeConst58 );

			if ( ( nodeConst59 > 0u ) ) {

				nodeVar37 = textureSample( nodeUniform19, nodeUniform19_sampler, nodeConst48 );
				let nodeConst60 = nodeVar37;

				if ( ( dot( nodeConst60, vec4<f32>( vec3<f32>( 0.2126, 0.7152, 0.0722 ), 1.0 ) ) > 0.001 ) ) {

					let nodeConst61 = normalize( nodeConst51 );
					let nodeConst62 = clamp( dot( nodeVar6, nodeConst61 ), 0.0, 1.0 );

					if ( ( nodeConst62 > 0.001 ) ) {

						nodeVar39 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeConst48 );
						let nodeConst63 = normalize( ( ( nodeVar39 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );

						if ( ( ( nodeConst44 > 0.0 ) && ( dot( nodeConst63, nodeVar7 ) > 0.0 ) ) ) {

							let nodeConst64 = dot( nodeConst63, ( - nodeConst61 ) );

							if ( ( sign( nodeConst64 ) < 0.0 ) ) {

								nodeVar40 = ( abs( nodeConst64 ) * nodeConst44 );

							} else {

								nodeVar40 = abs( nodeConst64 );

							}

							nodeVar38 = nodeVar40;

						} else {

							let nodeConst64 = dot( nodeConst63, ( - nodeConst61 ) );
							nodeVar38 = clamp( nodeConst64, 0.0, 1.0 );

						}

						nodeVar28 = ( vec4<f32>( nodeVar28, 1.0 ) + ( ( ( vec4<f32>( ( f32( nodeConst59 ) / 32.0 ) ) * nodeConst60 ) * vec4<f32>( nodeConst62 ) ) * vec4<f32>( nodeVar38 ) ) ).xyz;
						

					}

					

				}

				

			}


		}

		nodeVar10 = ( nodeVar10 + nodeVar28 );
		nodeVar9 = ( nodeVar9 + ( f32( countOneBits( nodeVar13 ) ) / 32.0 ) );

	}

	nodeVar9 = ( nodeVar9 / f32( nodeConst3 ) );
	nodeVar9 = clamp( pow( ( 1.0 - clamp( nodeVar9, 0.0, 1.0 ) ), nodeConst5 ), 0.0, 1.0 );
	nodeVar10 = ( nodeVar10 / vec3<f32>( f32( nodeConst3 ) ) );
	nodeVar10 = ( nodeVar10 * vec3<f32>( nodeConst6 ) );
	const nodeConst65 = 7.0;
	let nodeConst66 = dot( nodeVar10, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );

	if ( ( nodeConst66 > nodeConst65 ) ) {

		nodeVar41 = ( nodeConst65 / nodeConst66 );

	} else {

		nodeVar41 = 1.0;

	}

	nodeVar10 = ( nodeVar10 * vec3<f32>( nodeVar41 ) );
	nodeVar42 = nodeVar9;
	nodeVar43 = nodeVar10;
	DiffuseColor = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform20 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) ).x;
	output.m0 = nodeVar42;
	output.m1 = nodeVar43;

	// result

	return output;

}
