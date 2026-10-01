// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : f32,
	@location( 1 ) m1 : vec3<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_depth_2d;
@binding( 2 ) @group( 0 ) var nodeUniform2_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform2 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform18_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform18 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform3 : u32,
	nodeUniform4 : u32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : u32,
	nodeUniform9 : vec2<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform16 : u32,
	nodeUniform17 : f32,
	nodeUniform19 : f32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec2<u32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : bool;
var<private> nodeVar11 : u32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : bool;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : bool;
var<private> nodeVar23 : u32;
var<private> nodeVar24 : u32;
var<private> nodeVar25 : u32;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec4<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : bool;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : bool;
var<private> nodeVar44 : u32;
var<private> nodeVar45 : u32;
var<private> nodeVar46 : u32;
var<private> nodeVar47 : vec4<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
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


fn spatialOffsets ( position : vec2<f32> ) -> f32 {

	


	return ( 0.25 * f32( ( i32( ( position.y - position.x ) ) & 3 ) ) );

}


fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn GTAOFastAcos ( value : vec2<f32> ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = ( ( abs( value ) * vec2<f32>( -0.156583 ) ) + vec2<f32>( 1.5707963267948966 ) );
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

	nodeVar1 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar0 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar2 = nodeVar0;

	if ( ( nodeVar2 >= 1.0 ) ) {

		discard;
		

	}

	nodeVar3 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar2 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar2 ), 1.0 ) ).w ) );
	nodeVar4 = textureSample( nodeUniform2, nodeUniform2_sampler, nodeVarying0 );
	nodeVar5 = normalize( ( ( nodeVar4 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );
	nodeVar6 = normalize( ( - nodeVar3 ) );
	nodeVar7 = 0.0;
	nodeVar8 = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst0 = object.nodeUniform3;
	let nodeConst1 = object.nodeUniform4;
	let nodeConst2 = object.nodeUniform5;
	let nodeConst3 = object.nodeUniform6;
	let nodeConst4 = object.nodeUniform7;
	nodeVar9 = 0.0;
	nodeVar10 = bool( object.nodeUniform8 );

	if ( nodeVar10 ) {

		nodeVar9 = ( ( nodeConst4 * ( object.nodeUniform9.x / 2.0 ) ) / 16.0 );
		

	} else {

		nodeVar9 = max( ( ( nodeConst4 * object.nodeUniform10 ) / ( - nodeVar3.z ) ), f32( nodeConst1 ) );
		

	}

	nodeVar9 = ( nodeVar9 / ( f32( nodeConst1 ) + 1.0 ) );
	let nodeConst5 = ( max( 1.0, f32( ( nodeConst1 - 1u ) ) ) * nodeVar9 );

	for ( var i : u32 = 0u; i < nodeConst0; i ++ ) {

		let nodeConst6 = ( ( ( f32( i ) + interleavedGradientNoise( fragCoord.xy ) ) + object.nodeUniform11 ) * ( 3.141592653589793 / f32( nodeConst0 ) ) );
		let nodeConst7 = vec3<f32>( vec2<f32>( cos( nodeConst6 ), sin( nodeConst6 ) ), 0.0 );
		let nodeConst8 = ( nodeConst7.xy * ( vec2<f32>( 1.0 ) / object.nodeUniform9 ) );
		let nodeConst9 = normalize( cross( nodeConst7, nodeVar6 ) );
		let nodeConst10 = cross( nodeVar6, nodeConst9 );
		let nodeConst11 = ( nodeVar5 - ( nodeConst9 * vec3<f32>( dot( nodeVar5, nodeConst9 ) ) ) );
		let nodeConst12 = normalize( nodeConst11 );
		let nodeConst13 = clamp( dot( nodeConst12, nodeVar6 ), -1.0, 1.0 );
		let nodeConst14 = ( ( - sign( dot( nodeConst11, nodeConst10 ) ) ) * acos( nodeConst13 ) );
		nodeVar11 = 0u;
		nodeVar11 = 0u;
		let nodeConst15 = object.nodeUniform12;
		let nodeConst16 = object.nodeUniform13;
		let nodeConst17 = object.nodeUniform14;
		nodeVar12 = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst18 = object.nodeUniform4;

		for ( var i : u32 = 0u; i < nodeConst18; i ++ ) {

			nodeVar13 = ( fract( ( spatialOffsets( fragCoord.xy ) + object.nodeUniform15 ) ) + fract( ( sin( tsl_mod_float( dot( ( ( ( nodeVarying0 + vec2<f32>( ( object.nodeUniform11 * 0.02 ) ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), vec2<f32>( 12.9898, 78.233 ) ), 3.141592653589793 ) ) * 43758.5453 ) ) );
			let nodeConst19 = ( pow( abs( ( ( nodeVar9 * ( f32( i ) + nodeVar13 ) ) / nodeConst5 ) ), nodeConst15 ) * nodeConst5 );
			let nodeConst20 = ( nodeConst8 * vec2<f32>( max( nodeConst19, ( f32( i ) + 1.0 ) ) ) );

			if ( true ) {

				nodeVar14 = vec2<f32>( 1.0, -1.0 );

			} else {

				nodeVar14 = vec2<f32>( -1.0, 1.0 );

			}

			let nodeConst21 = ( nodeVarying0 + ( nodeConst20 * nodeVar14 ) );

			if ( ( ( ( ( nodeConst21.x <= 0.0 ) || ( nodeConst21.y <= 0.0 ) ) || ( nodeConst21.x >= 1.0 ) ) || ( nodeConst21.y >= 1.0 ) ) ) {

				break;
				

			}

			nodeVar15 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst21 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst22 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst21.x, ( 1.0 - nodeConst21.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar15 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst21.x, ( 1.0 - nodeConst21.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar15 ), 1.0 ) ).w ) );
			let nodeConst23 = normalize( ( nodeConst22 - nodeVar3 ) );

			if ( true ) {


				if ( true ) {

					nodeVar17 = 1.0;

				} else {

					nodeVar17 = -1.0;

				}

				nodeVar19 = bool( object.nodeUniform16 );

				if ( nodeVar19 ) {

					nodeVar18 = ( clamp( ( ( - nodeConst22.z ) / object.nodeUniform17 ), 0.0, 1.0 ) * 100.0 );

				} else {

					nodeVar18 = 1.0;

				}

				nodeVar16 = clamp( ( ( ( vec2<f32>( nodeVar17 ) * ( - GTAOFastAcos( clamp( vec2<f32>( dot( nodeConst23, nodeVar6 ), dot( normalize( ( ( nodeConst22 - ( ( vec3<f32>( nodeVar18 ) * nodeVar6 ) * vec3<f32>( nodeConst16 ) ) ) - nodeVar3 ) ), nodeVar6 ) ), vec2<f32>( -1.0 ), vec2<f32>( 1.0 ) ) ) ) ) - vec2<f32>( ( nodeConst14 - 1.5707963267948966 ) ) ) / vec2<f32>( 3.141592653589793 ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) ).yx;

			} else {


				if ( true ) {

					nodeVar20 = 1.0;

				} else {

					nodeVar20 = -1.0;

				}

				nodeVar22 = bool( object.nodeUniform16 );

				if ( nodeVar22 ) {

					nodeVar21 = ( clamp( ( ( - nodeConst22.z ) / object.nodeUniform17 ), 0.0, 1.0 ) * 100.0 );

				} else {

					nodeVar21 = 1.0;

				}

				nodeVar16 = clamp( ( ( ( vec2<f32>( nodeVar20 ) * ( - GTAOFastAcos( clamp( vec2<f32>( dot( nodeConst23, nodeVar6 ), dot( normalize( ( ( nodeConst22 - ( ( vec3<f32>( nodeVar21 ) * nodeVar6 ) * vec3<f32>( nodeConst16 ) ) ) - nodeVar3 ) ), nodeVar6 ) ), vec2<f32>( -1.0 ), vec2<f32>( 1.0 ) ) ) ) ) - vec2<f32>( ( nodeConst14 - 1.5707963267948966 ) ) ) / vec2<f32>( 3.141592653589793 ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) );

			}

			let nodeConst24 = nodeVar16.x;
			let nodeConst25 = nodeVar16.y;
			let nodeConst26 = u32( ( nodeVar16 * vec2<f32>( 32.0 ) ).x );
			let nodeConst27 = u32( ceil( ( ( nodeConst25 - nodeConst24 ) * 32.0 ) ) );

			if ( ( nodeConst27 > 0u ) ) {

				nodeVar23 = ( 4294967295u >> ( ( 32u - 32u ) + ( 32u - nodeConst27 ) ) );

			} else {

				nodeVar23 = 0u;

			}

			let nodeConst28 = nodeVar23;
			nodeVar24 = ( ( nodeConst28 << nodeConst26 ) & ( ~ nodeVar11 ) );
			nodeVar11 = ( nodeVar11 | nodeVar24 );
			nodeVar25 = countOneBits( nodeVar24 );

			if ( ( f32( nodeVar25 ) > 0.0 ) ) {

				nodeVar26 = textureSample( nodeUniform18, nodeUniform18_sampler, nodeConst21 );

				if ( ( dot( nodeVar26, vec4<f32>( vec3<f32>( 0.2126, 0.7152, 0.0722 ), 1.0 ) ) > 0.001 ) ) {

					nodeVar27 = normalize( nodeConst23 );
					nodeVar28 = clamp( dot( nodeVar5, nodeVar27 ), 0.0, 1.0 );

					if ( ( nodeVar28 > 0.001 ) ) {

						nodeVar30 = textureSample( nodeUniform2, nodeUniform2_sampler, nodeConst21 );
						nodeVar31 = normalize( ( ( nodeVar30 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );

						if ( ( ( nodeConst17 > 0.0 ) && ( dot( nodeVar31, nodeVar6 ) > 0.0 ) ) ) {

							nodeVar33 = dot( nodeVar31, ( - nodeVar27 ) );

							if ( ( sign( nodeVar33 ) < 0.0 ) ) {

								nodeVar32 = ( abs( nodeVar33 ) * nodeConst17 );

							} else {

								nodeVar32 = abs( nodeVar33 );

							}

							nodeVar29 = nodeVar32;

						} else {

							nodeVar29 = clamp( dot( nodeVar31, ( - nodeVar27 ) ), 0.0, 1.0 );

						}

						nodeVar12 = ( vec4<f32>( nodeVar12, 1.0 ) + ( ( ( vec4<f32>( ( f32( nodeVar25 ) / 32.0 ) ) * nodeVar26 ) * vec4<f32>( nodeVar28 ) ) * vec4<f32>( nodeVar29 ) ) ).xyz;
						

					}

					

				}

				

			}


		}

		nodeVar8 = ( nodeVar8 + nodeVar12 );
		let nodeConst29 = object.nodeUniform12;
		let nodeConst30 = object.nodeUniform13;
		let nodeConst31 = object.nodeUniform14;
		nodeVar34 = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst32 = object.nodeUniform4;

		for ( var i : u32 = 0u; i < nodeConst32; i ++ ) {

			let nodeConst33 = ( pow( abs( ( ( nodeVar9 * ( f32( i ) + nodeVar13 ) ) / nodeConst5 ) ), nodeConst29 ) * nodeConst5 );
			let nodeConst34 = ( nodeConst8 * vec2<f32>( max( nodeConst33, ( f32( i ) + 1.0 ) ) ) );

			if ( false ) {

				nodeVar35 = vec2<f32>( 1.0, -1.0 );

			} else {

				nodeVar35 = vec2<f32>( -1.0, 1.0 );

			}

			let nodeConst35 = ( nodeVarying0 + ( nodeConst34 * nodeVar35 ) );

			if ( ( ( ( ( nodeConst35.x <= 0.0 ) || ( nodeConst35.y <= 0.0 ) ) || ( nodeConst35.x >= 1.0 ) ) || ( nodeConst35.y >= 1.0 ) ) ) {

				break;
				

			}

			nodeVar36 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst35 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst36 = ( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst35.x, ( 1.0 - nodeConst35.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform1 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst35.x, ( 1.0 - nodeConst35.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) ).w ) );
			let nodeConst37 = normalize( ( nodeConst36 - nodeVar3 ) );

			if ( false ) {


				if ( false ) {

					nodeVar38 = 1.0;

				} else {

					nodeVar38 = -1.0;

				}

				nodeVar40 = bool( object.nodeUniform16 );

				if ( nodeVar40 ) {

					nodeVar39 = ( clamp( ( ( - nodeConst36.z ) / object.nodeUniform17 ), 0.0, 1.0 ) * 100.0 );

				} else {

					nodeVar39 = 1.0;

				}

				nodeVar37 = clamp( ( ( ( vec2<f32>( nodeVar38 ) * ( - GTAOFastAcos( clamp( vec2<f32>( dot( nodeConst37, nodeVar6 ), dot( normalize( ( ( nodeConst36 - ( ( vec3<f32>( nodeVar39 ) * nodeVar6 ) * vec3<f32>( nodeConst30 ) ) ) - nodeVar3 ) ), nodeVar6 ) ), vec2<f32>( -1.0 ), vec2<f32>( 1.0 ) ) ) ) ) - vec2<f32>( ( nodeConst14 - 1.5707963267948966 ) ) ) / vec2<f32>( 3.141592653589793 ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) ).yx;

			} else {


				if ( false ) {

					nodeVar41 = 1.0;

				} else {

					nodeVar41 = -1.0;

				}

				nodeVar43 = bool( object.nodeUniform16 );

				if ( nodeVar43 ) {

					nodeVar42 = ( clamp( ( ( - nodeConst36.z ) / object.nodeUniform17 ), 0.0, 1.0 ) * 100.0 );

				} else {

					nodeVar42 = 1.0;

				}

				nodeVar37 = clamp( ( ( ( vec2<f32>( nodeVar41 ) * ( - GTAOFastAcos( clamp( vec2<f32>( dot( nodeConst37, nodeVar6 ), dot( normalize( ( ( nodeConst36 - ( ( vec3<f32>( nodeVar42 ) * nodeVar6 ) * vec3<f32>( nodeConst30 ) ) ) - nodeVar3 ) ), nodeVar6 ) ), vec2<f32>( -1.0 ), vec2<f32>( 1.0 ) ) ) ) ) - vec2<f32>( ( nodeConst14 - 1.5707963267948966 ) ) ) / vec2<f32>( 3.141592653589793 ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) );

			}

			let nodeConst38 = nodeVar37.x;
			let nodeConst39 = nodeVar37.y;
			let nodeConst40 = u32( ( nodeVar37 * vec2<f32>( 32.0 ) ).x );
			let nodeConst41 = u32( ceil( ( ( nodeConst39 - nodeConst38 ) * 32.0 ) ) );

			if ( ( nodeConst41 > 0u ) ) {

				nodeVar44 = ( 4294967295u >> ( ( 32u - 32u ) + ( 32u - nodeConst41 ) ) );

			} else {

				nodeVar44 = 0u;

			}

			let nodeConst42 = nodeVar44;
			nodeVar45 = ( ( nodeConst42 << nodeConst40 ) & ( ~ nodeVar11 ) );
			nodeVar11 = ( nodeVar11 | nodeVar45 );
			nodeVar46 = countOneBits( nodeVar45 );

			if ( ( f32( nodeVar46 ) > 0.0 ) ) {

				nodeVar47 = textureSample( nodeUniform18, nodeUniform18_sampler, nodeConst35 );

				if ( ( dot( nodeVar47, vec4<f32>( vec3<f32>( 0.2126, 0.7152, 0.0722 ), 1.0 ) ) > 0.001 ) ) {

					nodeVar48 = normalize( nodeConst37 );
					nodeVar49 = clamp( dot( nodeVar5, nodeVar48 ), 0.0, 1.0 );

					if ( ( nodeVar49 > 0.001 ) ) {

						nodeVar51 = textureSample( nodeUniform2, nodeUniform2_sampler, nodeConst35 );
						nodeVar52 = normalize( ( ( nodeVar51 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) ).xyz );

						if ( ( ( nodeConst31 > 0.0 ) && ( dot( nodeVar52, nodeVar6 ) > 0.0 ) ) ) {

							nodeVar54 = dot( nodeVar52, ( - nodeVar48 ) );

							if ( ( sign( nodeVar54 ) < 0.0 ) ) {

								nodeVar53 = ( abs( nodeVar54 ) * nodeConst31 );

							} else {

								nodeVar53 = abs( nodeVar54 );

							}

							nodeVar50 = nodeVar53;

						} else {

							nodeVar50 = clamp( dot( nodeVar52, ( - nodeVar48 ) ), 0.0, 1.0 );

						}

						nodeVar34 = ( vec4<f32>( nodeVar34, 1.0 ) + ( ( ( vec4<f32>( ( f32( nodeVar46 ) / 32.0 ) ) * nodeVar47 ) * vec4<f32>( nodeVar49 ) ) * vec4<f32>( nodeVar50 ) ) ).xyz;
						

					}

					

				}

				

			}


		}

		nodeVar8 = ( nodeVar8 + nodeVar34 );
		nodeVar7 = ( nodeVar7 + ( f32( countOneBits( nodeVar11 ) ) / 32.0 ) );

	}

	nodeVar7 = ( nodeVar7 / f32( nodeConst0 ) );
	nodeVar7 = clamp( pow( ( 1.0 - clamp( nodeVar7, 0.0, 1.0 ) ), nodeConst2 ), 0.0, 1.0 );
	nodeVar8 = ( nodeVar8 / vec3<f32>( f32( nodeConst0 ) ) );
	nodeVar8 = ( nodeVar8 * vec3<f32>( nodeConst3 ) );
	let nodeConst43 = 7.0;
	nodeVar56 = dot( nodeVar8, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );

	if ( ( nodeVar56 > nodeConst43 ) ) {

		nodeVar55 = ( nodeConst43 / nodeVar56 );

	} else {

		nodeVar55 = 1.0;

	}

	nodeVar8 = ( nodeVar8 * vec3<f32>( nodeVar55 ) );
	nodeVar57 = nodeVar7;
	nodeVar58 = nodeVar8;
	DiffuseColor = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform19 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) ).x;
	output.m0 = nodeVar57;
	output.m1 = nodeVar58;

	// result

	return output;

}
