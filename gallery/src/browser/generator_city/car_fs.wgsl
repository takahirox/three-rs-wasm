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
@binding( 2 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 3 ) @group( 1 ) var nodeUniform14 : texture_2d<f32>;
@binding( 4 ) @group( 1 ) var nodeUniform23_sampler : sampler_comparison;
@binding( 5 ) @group( 1 ) var nodeUniform23 : texture_depth_2d;
@binding( 6 ) @group( 1 ) var nodeUniform31_sampler : sampler;
@binding( 7 ) @group( 1 ) var nodeUniform31 : texture_3d<f32>;
@binding( 8 ) @group( 1 ) var nodeUniform33_sampler : sampler;
@binding( 9 ) @group( 1 ) var nodeUniform33 : texture_cube<f32>;

struct nodeUniform6Struct {
	value : array< vec4<f32>, 2 >
};
@binding( 1 ) @group( 1 )
var<uniform> nodeUniform6 : nodeUniform6Struct;

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : vec2<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : vec2<f32>,
	nodeUniform11 : f32,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform34 : mat4x4<f32>,
	nodeUniform36 : f32,
	nodeUniform37 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : vec2<f32>,
	nodeUniform26 : f32,
	nodeUniform32 : f32,
	nodeUniform30 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> Metalness : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> Roughness : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar32 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec4<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar48 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar49 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


fn fn110 ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn fn112 ( alpha : f32, dotNH : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );
	let nodeConst1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeConst0 ) ) );

	return ( ( nodeConst0 / ( nodeConst1 * nodeConst1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32>,
	@location( 5 ) nodeVarying9 : vec3<f32>,
	@location( 6 ) nodeVarying10 : vec2<f32>,
	@location( 7 ) nodeVarying11 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst1 = ( nodeVarying3 == 2.0 );

	if ( nodeConst1 ) {

		let nodeConst2 = vec2<f32>( ( abs( nodeVarying8.z ) - object.nodeUniform5 ), ( nodeVarying8.y - object.nodeUniform4 ) );
		let nodeConst3 = length( nodeConst2 );
		nodeVar0 = ( vec3<f32>( 0.009134058699157796, 0.00972121731707524, 0.010960094003125918 ) * vec3<f32>( ( ( smoothstep( ( object.nodeUniform4 * 0.85 ), ( object.nodeUniform4 * 0.94 ), nodeConst3 ) * 0.2 ) + 0.8 ) ) );

	} else {

		let nodeConst4 = ( nodeVarying3 == 3.0 );

		if ( nodeConst4 ) {

			let nodeConst2 = vec2<f32>( ( abs( nodeVarying8.z ) - object.nodeUniform5 ), ( nodeVarying8.y - object.nodeUniform4 ) );
			let nodeConst3 = length( nodeConst2 );
			nodeVar1 = mix( vec3<f32>( 0.008568125615105716, 0.010960094003125918, 0.014443843592229466 ), vec3<f32>( 0.4286904966038916, 0.46778379610254284, 0.4910208498384856 ), max( max( smoothstep( 0.62, 0.42, ( abs( ( fract( ( atan2( nodeConst2.y, nodeConst2.x ) * 0.7957747154594768 ) ) - 0.5 ) ) * 2.0 ) ), smoothstep( ( object.nodeUniform4 * 0.53 ), ( object.nodeUniform4 * 0.59 ), nodeConst3 ) ), smoothstep( 0.06, 0.035, nodeConst3 ) ) );

		} else {

			let nodeConst5 = ( nodeVarying3 == 4.0 );

			if ( nodeConst5 ) {

				nodeVar2 = vec3<f32>( 0.014443843592229466, 0.01680737574872402, 0.019382360952473074 );

			} else {

				let nodeConst6 = ( nodeVarying3 == 6.0 );

				if ( nodeConst6 ) {

					nodeVar3 = vec3<f32>( 1.0, 0.6724431569510133, 0.14412847084818123 );

				} else {

					let nodeConst7 = ( nodeVarying3 == 1.0 );

					if ( nodeConst7 ) {

						let nodeConst8 = ( nodeVarying10 - vec2<f32>( 0.5 ) );
						let nodeConst9 = ( ( abs( nodeConst8 ) - vec2<f32>( 0.47, 0.43 ) ) + vec2<f32>( 0.045 ) );
						let nodeConst10 = ( ( length( max( nodeConst9, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst9.x, nodeConst9.y ), 0.0 ) ) - 0.045 );
						let nodeConst11 = max( fwidth( nodeConst10 ), 0.001 );
						let nodeConst12 = ( abs( nodeVarying11.x ) > 0.5 );

						if ( nodeConst12 ) {

							let nodeConst13 = abs( ( nodeVarying8.z - nodeUniform6.value[ 0u ].x ) );
							nodeVar5 = smoothstep( 0.026, 0.035, nodeConst13 );

						} else {

							nodeVar5 = 1.0;

						}


						if ( nodeConst12 ) {

							let nodeConst14 = abs( ( nodeVarying8.z - nodeUniform6.value[ 1u ].x ) );
							nodeVar6 = smoothstep( 0.026, 0.035, nodeConst14 );

						} else {

							nodeVar6 = 1.0;

						}

						let nodeConst15 = ( ( abs( nodeConst8 ) - vec2<f32>( 0.455, 0.405 ) ) + vec2<f32>( 0.035 ) );
						let nodeConst16 = ( ( length( max( nodeConst15, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst15.x, nodeConst15.y ), 0.0 ) ) - 0.035 );
						let nodeConst17 = max( fwidth( nodeConst16 ), 0.001 );

						if ( nodeConst12 ) {

							let nodeConst13 = abs( ( nodeVarying8.z - nodeUniform6.value[ 0u ].x ) );
							nodeVar7 = smoothstep( 0.043, 0.053, nodeConst13 );

						} else {

							nodeVar7 = 1.0;

						}


						if ( nodeConst12 ) {

							let nodeConst14 = abs( ( nodeVarying8.z - nodeUniform6.value[ 1u ].x ) );
							nodeVar8 = smoothstep( 0.043, 0.053, nodeConst14 );

						} else {

							nodeVar8 = 1.0;

						}

						let nodeConst18 = ( ( smoothstep( nodeConst17, ( - nodeConst17 ), nodeConst16 ) * nodeVar7 ) * nodeVar8 );
						nodeVar4 = mix( mix( nodeVarying9, vec3<f32>( 0.006512090790025684, 0.00972121731707524, 0.011612245176281512 ), ( ( smoothstep( nodeConst11, ( - nodeConst11 ), nodeConst10 ) * nodeVar5 ) * nodeVar6 ) ), vec3<f32>( 0.012286488353353374, 0.024157632443547246, 0.035601314869097636 ), nodeConst18 );

					} else {

						let nodeConst19 = ( ( nodeVarying3 == 5.0 ) && ( nodeVarying11.z < -0.5 ) );

						if ( nodeConst19 ) {

							nodeVar9 = vec3<f32>( 0.16202937562896222, 0.21586050010324417, 0.2541520943200296 );

						} else {

							let nodeConst2 = vec2<f32>( ( abs( nodeVarying8.z ) - object.nodeUniform5 ), ( nodeVarying8.y - object.nodeUniform4 ) );
							let nodeConst3 = length( nodeConst2 );
							let nodeConst20 = smoothstep( 0.65, 0.85, abs( nodeVarying8.x ) );
							let nodeConst21 = ( ( abs( vec2<f32>( ( nodeVarying8.z - object.nodeUniform8.x ), ( nodeVarying8.y - ( object.nodeUniform9 - 0.1 ) ) ) ) - vec2<f32>( 0.06, 0.011 ) ) + vec2<f32>( 0.006 ) );
							let nodeConst22 = ( ( length( max( nodeConst21, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst21.x, nodeConst21.y ), 0.0 ) ) - 0.006 );
							let nodeConst23 = max( fwidth( nodeConst22 ), 0.001 );
							let nodeConst24 = ( ( abs( vec2<f32>( ( nodeVarying8.z - object.nodeUniform8.y ), ( nodeVarying8.y - ( object.nodeUniform9 - 0.1 ) ) ) ) - vec2<f32>( 0.06, 0.011 ) ) + vec2<f32>( 0.006 ) );
							let nodeConst25 = ( ( length( max( nodeConst24, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst24.x, nodeConst24.y ), 0.0 ) ) - 0.006 );
							let nodeConst26 = max( fwidth( nodeConst25 ), 0.001 );
							let nodeConst27 = ( ( abs( vec2<f32>( nodeVarying8.x, ( nodeVarying8.y - ( object.nodeUniform10.x - 0.025 ) ) ) ) - vec2<f32>( 0.3, 0.075 ) ) + vec2<f32>( 0.025 ) );
							let nodeConst28 = ( ( length( max( nodeConst27, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst27.x, nodeConst27.y ), 0.0 ) ) - 0.025 );
							let nodeConst29 = max( fwidth( nodeConst28 ), 0.001 );
							let nodeConst30 = ( nodeVarying3 == 7.0 );

							if ( nodeConst30 ) {

								nodeVar10 = 1.0;

							} else {

								nodeVar10 = 0.0;

							}


							if ( nodeConst30 ) {

								nodeVar11 = object.nodeUniform10.x;

							} else {

								nodeVar11 = object.nodeUniform10.y;

							}

							let nodeConst31 = ( ( abs( vec2<f32>( nodeVarying8.x, ( nodeVarying8.y - ( nodeVar11 - 0.29 ) ) ) ) - vec2<f32>( 0.66, 0.045 ) ) + vec2<f32>( 0.03 ) );
							let nodeConst32 = ( ( length( max( nodeConst31, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst31.x, nodeConst31.y ), 0.0 ) ) - 0.03 );
							let nodeConst33 = max( fwidth( nodeConst32 ), 0.001 );
							let nodeConst34 = ( nodeConst30 || ( nodeVarying3 == 8.0 ) );

							if ( nodeConst34 ) {

								nodeVar12 = 1.0;

							} else {

								nodeVar12 = 0.0;

							}


							if ( nodeConst30 ) {

								nodeVar13 = ( object.nodeUniform10.x - 0.2 );

							} else {

								nodeVar13 = ( object.nodeUniform10.y - 0.17 );

							}

							let nodeConst35 = ( ( abs( vec2<f32>( nodeVarying8.x, ( nodeVarying8.y - nodeVar13 ) ) ) - vec2<f32>( 0.155, 0.055 ) ) + vec2<f32>( 0.008 ) );
							let nodeConst36 = ( ( length( max( nodeConst35, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst35.x, nodeConst35.y ), 0.0 ) ) - 0.008 );
							let nodeConst37 = max( fwidth( nodeConst36 ), 0.001 );

							if ( nodeConst34 ) {

								nodeVar14 = 1.0;

							} else {

								nodeVar14 = 0.0;

							}


							if ( nodeConst30 ) {

								nodeVar15 = vec3<f32>( 0.6375968739867731, 0.775822218312646, 0.8307698767709715 );

							} else {

								nodeVar15 = vec3<f32>( 0.19806931954941637, 0.005181516700061659, 0.007499032040460618 );

							}

							let nodeConst38 = ( ( abs( vec2<f32>( ( abs( nodeVarying8.x ) - 0.61 ), ( nodeVarying8.y - nodeVar11 ) ) ) - vec2<f32>( 0.19, 0.055 ) ) + vec2<f32>( 0.018 ) );
							let nodeConst39 = ( ( length( max( nodeConst38, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst38.x, nodeConst38.y ), 0.0 ) ) - 0.018 );
							let nodeConst40 = max( fwidth( nodeConst39 ), 0.001 );

							if ( nodeConst34 ) {

								nodeVar16 = 1.0;

							} else {

								nodeVar16 = 0.0;

							}

							let nodeConst41 = ( smoothstep( nodeConst40, ( - nodeConst40 ), nodeConst39 ) * nodeVar16 );
							nodeVar9 = mix( mix( mix( ( ( nodeVarying9 * vec3<f32>( ( mix( 1.0, ( ( smoothstep( ( object.nodeUniform4 + 0.025 ), ( object.nodeUniform4 + 0.095 ), nodeConst3 ) * 0.28 ) + 0.72 ), nodeConst20 ) * ( ( smoothstep( 0.25, 0.65, nodeVarying8.y ) * 0.3 ) + 0.7 ) ) ) ) * vec3<f32>( ( 1.0 - ( ( ( ( max( max( max( max( max( 0.0, smoothstep( 0.012, 0.004, abs( ( nodeVarying8.z - object.nodeUniform7.x ) ) ) ), smoothstep( 0.012, 0.004, abs( ( nodeVarying8.z - object.nodeUniform7.y ) ) ) ), smoothstep( 0.012, 0.004, abs( ( nodeVarying8.z - object.nodeUniform7.z ) ) ) ), smoothstep( nodeConst23, ( - nodeConst23 ), nodeConst22 ) ), smoothstep( nodeConst26, ( - nodeConst26 ), nodeConst25 ) ) * nodeConst20 ) * smoothstep( 0.36, 0.43, nodeVarying8.y ) ) * smoothstep( object.nodeUniform9, ( object.nodeUniform9 - 0.025 ), nodeVarying8.y ) ) * 0.5 ) ) ) ), ( vec3<f32>( 0.008023192982520563, 0.010329823026364548, 0.011612245176281512 ) * vec3<f32>( ( ( step( 0.5, fract( ( nodeVarying8.y * 65.0 ) ) ) * 0.35 ) + 0.65 ) ) ), max( ( smoothstep( nodeConst29, ( - nodeConst29 ), nodeConst28 ) * nodeVar10 ), ( smoothstep( nodeConst33, ( - nodeConst33 ), nodeConst32 ) * nodeVar12 ) ) ), vec3<f32>( 0.6866853124288864, 0.6938717612856897, 0.6514056374127929 ), ( smoothstep( nodeConst37, ( - nodeConst37 ), nodeConst36 ) * nodeVar14 ) ), nodeVar15, nodeConst41 );

						}

						nodeVar4 = nodeVar9;

					}

					nodeVar3 = nodeVar4;

				}

				nodeVar2 = nodeVar3;

			}

			nodeVar1 = nodeVar2;

		}

		nodeVar0 = nodeVar1;

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform11 );
	DiffuseColor.w = 1.0;
	let nodeConst4 = ( nodeVarying3 == 3.0 );

	if ( nodeConst4 ) {

		nodeVar17 = 0.8;

	} else {

		let nodeConst5 = ( nodeVarying3 == 4.0 );
		let nodeConst6 = ( nodeVarying3 == 6.0 );

		if ( ( ( nodeConst1 || nodeConst5 ) || nodeConst6 ) ) {

			nodeVar18 = 0.0;

		} else {

			let nodeConst7 = ( nodeVarying3 == 1.0 );

			if ( nodeConst7 ) {

				let nodeConst8 = ( nodeVarying10 - vec2<f32>( 0.5 ) );
				let nodeConst15 = ( ( abs( nodeConst8 ) - vec2<f32>( 0.455, 0.405 ) ) + vec2<f32>( 0.035 ) );
				let nodeConst16 = ( ( length( max( nodeConst15, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst15.x, nodeConst15.y ), 0.0 ) ) - 0.035 );
				let nodeConst17 = max( fwidth( nodeConst16 ), 0.001 );
				let nodeConst12 = ( abs( nodeVarying11.x ) > 0.5 );

				if ( nodeConst12 ) {

					let nodeConst13 = abs( ( nodeVarying8.z - nodeUniform6.value[ 0u ].x ) );
					nodeVar20 = smoothstep( 0.043, 0.053, nodeConst13 );

				} else {

					nodeVar20 = 1.0;

				}


				if ( nodeConst12 ) {

					let nodeConst14 = abs( ( nodeVarying8.z - nodeUniform6.value[ 1u ].x ) );
					nodeVar21 = smoothstep( 0.043, 0.053, nodeConst14 );

				} else {

					nodeVar21 = 1.0;

				}

				let nodeConst18 = ( ( smoothstep( nodeConst17, ( - nodeConst17 ), nodeConst16 ) * nodeVar20 ) * nodeVar21 );
				nodeVar19 = nodeConst18;

			} else {

				let nodeConst19 = ( ( nodeVarying3 == 5.0 ) && ( nodeVarying11.z < -0.5 ) );

				if ( nodeConst19 ) {

					nodeVar22 = 1.0;

				} else {

					nodeVar22 = 0.0;

				}

				nodeVar19 = nodeVar22;

			}

			nodeVar18 = mix( 0.25, 0.85, nodeVar19 );

		}

		nodeVar17 = nodeVar18;

	}

	let nodeConst42 = nodeVar17;
	Metalness = nodeConst42;
	let nodeConst5 = ( nodeVarying3 == 4.0 );

	if ( ( nodeConst1 || nodeConst5 ) ) {

		nodeVar23 = 0.85;

	} else {

		let nodeConst7 = ( nodeVarying3 == 1.0 );

		if ( nodeConst7 ) {

			let nodeConst8 = ( nodeVarying10 - vec2<f32>( 0.5 ) );
			let nodeConst15 = ( ( abs( nodeConst8 ) - vec2<f32>( 0.455, 0.405 ) ) + vec2<f32>( 0.035 ) );
			let nodeConst16 = ( ( length( max( nodeConst15, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst15.x, nodeConst15.y ), 0.0 ) ) - 0.035 );
			let nodeConst17 = max( fwidth( nodeConst16 ), 0.001 );
			let nodeConst12 = ( abs( nodeVarying11.x ) > 0.5 );

			if ( nodeConst12 ) {

				let nodeConst13 = abs( ( nodeVarying8.z - nodeUniform6.value[ 0u ].x ) );
				nodeVar25 = smoothstep( 0.043, 0.053, nodeConst13 );

			} else {

				nodeVar25 = 1.0;

			}


			if ( nodeConst12 ) {

				let nodeConst14 = abs( ( nodeVarying8.z - nodeUniform6.value[ 1u ].x ) );
				nodeVar26 = smoothstep( 0.043, 0.053, nodeConst14 );

			} else {

				nodeVar26 = 1.0;

			}

			let nodeConst18 = ( ( smoothstep( nodeConst17, ( - nodeConst17 ), nodeConst16 ) * nodeVar25 ) * nodeVar26 );
			nodeVar24 = nodeConst18;

		} else {

			let nodeConst19 = ( ( nodeVarying3 == 5.0 ) && ( nodeVarying11.z < -0.5 ) );

			if ( nodeConst19 ) {

				nodeVar27 = 1.0;

			} else {

				nodeVar27 = 0.0;

			}

			nodeVar24 = nodeVar27;

		}

		nodeVar23 = mix( 0.32, 0.055, nodeVar24 );

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst43 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar23, 0.045 ) + max( max( nodeConst43.x, nodeConst43.y ), nodeConst43.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeConst42 ) ) );
	let nodeConst30 = ( nodeVarying3 == 7.0 );

	if ( nodeConst30 ) {

		nodeVar28 = ( vec3<f32>( 0.6938717612856897, 0.8631572134510892, 1.0 ) * vec3<f32>( 4.0 ) );

	} else {

		nodeVar28 = ( vec3<f32>( 0.8713671191959567, 0.0, 0.002428215868235294 ) * vec3<f32>( 1.5 ) );

	}


	if ( nodeConst30 ) {

		nodeVar29 = object.nodeUniform10.x;

	} else {

		nodeVar29 = object.nodeUniform10.y;

	}

	let nodeConst38 = ( ( abs( vec2<f32>( ( abs( nodeVarying8.x ) - 0.61 ), ( nodeVarying8.y - nodeVar29 ) ) ) - vec2<f32>( 0.19, 0.055 ) ) + vec2<f32>( 0.018 ) );
	let nodeConst39 = ( ( length( max( nodeConst38, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst38.x, nodeConst38.y ), 0.0 ) ) - 0.018 );
	let nodeConst40 = max( fwidth( nodeConst39 ), 0.001 );
	let nodeConst34 = ( nodeConst30 || ( nodeVarying3 == 8.0 ) );

	if ( nodeConst34 ) {

		nodeVar30 = 1.0;

	} else {

		nodeVar30 = 0.0;

	}

	let nodeConst41 = ( smoothstep( nodeConst40, ( - nodeConst40 ), nodeConst39 ) * nodeVar30 );
	let nodeConst6 = ( nodeVarying3 == 6.0 );

	if ( nodeConst6 ) {

		nodeVar31 = ( vec3<f32>( 1.0, 0.6795424696265424, 0.19806931954941637 ) * vec3<f32>( 2.0 ) );

	} else {

		nodeVar31 = vec3<f32>( 0.0, 0.0, 0.0 );

	}

	EmissiveColor = ( ( nodeVar28 * vec3<f32>( nodeConst41 ) ) + nodeVar31 );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar32 = textureSample( nodeUniform14, nodeUniform14_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar32;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst44 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst44 );
	let nodeConst45 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst46 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst44 * nodeConst45 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst46 ) * nodeConst45 ) ) ) * vec3<f32>( nodeConst46 ) ) );
	let nodeConst47 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform17 - render.nodeUniform18 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst48 = ( render.nodeUniform20 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform21 ) ) ), 1.0 ) );
	let nodeConst49 = ( nodeConst48.xyz / vec3<f32>( nodeConst48.w ) );
	let nodeConst50 = vec3<f32>( nodeConst49.x, ( 1.0 - nodeConst49.y ), ( nodeConst49.z + render.nodeUniform22 ) );

	if ( ( ( ( ( ( nodeConst50.x >= 0.0 ) && ( nodeConst50.x <= 1.0 ) ) && ( nodeConst50.y >= 0.0 ) ) && ( nodeConst50.y <= 1.0 ) ) && ( nodeConst50.z <= 1.0 ) ) ) {

		let nodeConst51 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst52 = ( vec2<f32>( render.nodeUniform24 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform25 ) );
		let nodeConst53 = ( nodeConst50.xy + ( fn25( 0, 5, nodeConst51 ) * nodeConst52 ) );
		nodeVar34 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst53, nodeConst50.z );
		let nodeConst54 = ( nodeConst50.xy + ( fn25( 1, 5, nodeConst51 ) * nodeConst52 ) );
		nodeVar35 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst54, nodeConst50.z );
		let nodeConst55 = ( nodeConst50.xy + ( fn25( 2, 5, nodeConst51 ) * nodeConst52 ) );
		nodeVar36 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst55, nodeConst50.z );
		let nodeConst56 = ( nodeConst50.xy + ( fn25( 3, 5, nodeConst51 ) * nodeConst52 ) );
		nodeVar37 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst56, nodeConst50.z );
		let nodeConst57 = ( nodeConst50.xy + ( fn25( 4, 5, nodeConst51 ) * nodeConst52 ) );
		nodeVar38 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst57, nodeConst50.z );
		nodeVar33 = ( ( ( ( ( nodeVar34 + nodeVar35 ) + nodeVar36 ) + nodeVar37 ) + nodeVar38 ) * 0.2 );

	} else {

		nodeVar33 = 1.0;

	}

	nodeVar39 = mix( 1.0, nodeVar33, render.nodeUniform26 );
	nodeVar40 = ( vec3<f32>( clamp( dot( normalView, nodeConst47 ), 0.0, 1.0 ) ) * ( render.nodeUniform19 * vec3<f32>( nodeVar39 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst58 = clamp( dot( positionViewDirection, normalize( ( nodeConst47 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst59 = exp2( ( ( ( nodeConst58 * -5.55473 ) - 6.98316 ) * nodeConst58 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar40 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst59 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst59 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst60 = normalize( ( nodeConst47 + positionViewDirection ) );
	let nodeConst61 = clamp( dot( positionViewDirection, nodeConst60 ), 0.0, 1.0 );
	let nodeConst62 = exp2( ( ( ( nodeConst61 * -5.55473 ) - 6.98316 ) * nodeConst61 ) );
	let nodeConst63 = max( Roughness, 0.045 );
	let nodeConst64 = ( nodeConst63 * nodeConst63 );
	directSpecular = ( directSpecular + ( ( nodeVar40 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst62 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst62 ) ) ) * vec3<f32>( fn110( nodeConst64, clamp( dot( normalView, nodeConst47 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst64, clamp( dot( normalView, nodeConst60 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst65 = ( ( object.nodeUniform27 - object.nodeUniform28 ) / object.nodeUniform29 );
	let nodeConst66 = ( v_positionWorld + ( ( normalWorld * nodeConst65 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst67 = ( nodeConst65 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst66 >= ( object.nodeUniform28 - nodeConst67 ) ) ) && all( ( nodeConst66 <= ( object.nodeUniform27 + nodeConst67 ) ) ) ) && ( render.nodeUniform30 >= lightProbePriority ) ) ) {

		let nodeConst68 = ( object.nodeUniform27 - object.nodeUniform28 );
		let nodeConst69 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform29 );
		let nodeConst70 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst68 / object.nodeUniform29 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform28 ) / nodeConst68 ), nodeConst69, ( vec3<f32>( 1.0 ) - nodeConst69 ) );
		let nodeConst71 = ( ( nodeConst70.z * object.nodeUniform29.z ) + 1.0 );
		let nodeConst72 = ( object.nodeUniform29.z + 2.0 );
		let nodeConst73 = ( nodeConst72 * 7.0 );
		nodeVar41 = textureSample( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeConst70.xy, ( ( nodeConst71 + ( nodeConst72 * 0.0 ) ) / nodeConst73 ) ) );
		nodeVar42 = textureSample( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeConst70.xy, ( ( nodeConst71 + ( nodeConst72 * 1.0 ) ) / nodeConst73 ) ) );
		nodeVar43 = textureSample( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeConst70.xy, ( ( nodeConst71 + ( nodeConst72 * 2.0 ) ) / nodeConst73 ) ) );
		nodeVar44 = textureSample( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeConst70.xy, ( ( nodeConst71 + ( nodeConst72 * 3.0 ) ) / nodeConst73 ) ) );
		nodeVar45 = textureSample( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeConst70.xy, ( ( nodeConst71 + ( nodeConst72 * 4.0 ) ) / nodeConst73 ) ) );
		nodeVar46 = textureSample( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeConst70.xy, ( ( nodeConst71 + ( nodeConst72 * 5.0 ) ) / nodeConst73 ) ) );
		nodeVar47 = textureSample( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeConst70.xy, ( ( nodeConst71 + ( nodeConst72 * 6.0 ) ) / nodeConst73 ) ) );
		let nodeConst74 = array< vec3<f32>, 9 >( nodeVar41.xyz, vec3<f32>( nodeVar41.w, nodeVar42.xy ), vec3<f32>( nodeVar42.zw, nodeVar43.x ), nodeVar43.yzw, nodeVar44.xyz, vec3<f32>( nodeVar44.w, nodeVar45.xy ), vec3<f32>( nodeVar45.zw, nodeVar46.x ), nodeVar46.yzw, nodeVar47.xyz );
		let nodeConst75 = ( max( ( ( ( ( ( ( ( ( ( nodeConst74[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst74[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst74[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst74[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst74[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst74[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst74[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst74[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst74[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform32 ) );
		lightProbeIrradiance = nodeConst75;
		lightProbePriority = render.nodeUniform30;
		

	}

	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst76 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst77 = ( object.nodeUniform34 * vec4<f32>( nodeConst76, 1.0 ) );
	let nodeConst78 = clamp( Roughness, 0.0, 1.0 );
	nodeVar48 = textureSampleLevel( nodeUniform33, nodeUniform33_sampler, vec3<f32>( ( - nodeConst77.x ), nodeConst77.yz ), ( ( object.nodeUniform36 * nodeConst78 ) * ( 2.0 - nodeConst78 ) ) );
	radiance = ( radiance + ( nodeVar48.xyz * vec3<f32>( object.nodeUniform37 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst79 = ( object.nodeUniform34 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst80 = clamp( 1.0, 0.0, 1.0 );
	nodeVar49 = textureSampleLevel( nodeUniform33, nodeUniform33_sampler, vec3<f32>( ( - nodeConst79.x ), nodeConst79.yz ), ( ( object.nodeUniform36 * nodeConst80 ) * ( 2.0 - nodeConst80 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar49.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform37 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar50 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar50 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst81 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst81 );
	let nodeConst82 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst83 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst81 * nodeConst82 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst83 ) * nodeConst82 ) ) ) * vec3<f32>( nodeConst83 ) ) );
	let nodeConst84 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar51 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst84 ) );
	nodeVar52 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst84 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar51 );
	indirectDiffuse = ( indirectDiffuse + nodeVar52 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst85 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst85;

	// result

	output.color = nodeConst85;

	return output;

}
