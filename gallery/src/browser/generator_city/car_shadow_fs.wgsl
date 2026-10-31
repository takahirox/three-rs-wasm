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
	nodeUniform14 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

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
var<private> Output : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) nodeVarying5 : vec3<f32>,
	@location( 2 ) nodeVarying6 : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec2<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32> ) -> OutputStruct {

	// flow
	// code


	if ( ( nodeVarying3 == 2.0 ) ) {

		let nodeConst1 = vec2<f32>( ( abs( nodeVarying5.z ) - object.nodeUniform5 ), ( nodeVarying5.y - object.nodeUniform4 ) );
		let nodeConst2 = length( nodeConst1 );
		nodeVar0 = ( vec3<f32>( 0.009134058699157796, 0.00972121731707524, 0.010960094003125918 ) * vec3<f32>( ( ( smoothstep( ( object.nodeUniform4 * 0.85 ), ( object.nodeUniform4 * 0.94 ), nodeConst2 ) * 0.2 ) + 0.8 ) ) );

	} else {


		if ( ( nodeVarying3 == 3.0 ) ) {

			let nodeConst1 = vec2<f32>( ( abs( nodeVarying5.z ) - object.nodeUniform5 ), ( nodeVarying5.y - object.nodeUniform4 ) );
			let nodeConst2 = length( nodeConst1 );
			nodeVar1 = mix( vec3<f32>( 0.008568125615105716, 0.010960094003125918, 0.014443843592229466 ), vec3<f32>( 0.4286904966038916, 0.46778379610254284, 0.4910208498384856 ), max( max( smoothstep( 0.62, 0.42, ( abs( ( fract( ( atan2( nodeConst1.y, nodeConst1.x ) * 0.7957747154594768 ) ) - 0.5 ) ) * 2.0 ) ), smoothstep( ( object.nodeUniform4 * 0.53 ), ( object.nodeUniform4 * 0.59 ), nodeConst2 ) ), smoothstep( 0.06, 0.035, nodeConst2 ) ) );

		} else {


			if ( ( nodeVarying3 == 4.0 ) ) {

				nodeVar2 = vec3<f32>( 0.014443843592229466, 0.01680737574872402, 0.019382360952473074 );

			} else {


				if ( ( nodeVarying3 == 6.0 ) ) {

					nodeVar3 = vec3<f32>( 1.0, 0.6724431569510133, 0.14412847084818123 );

				} else {


					if ( ( nodeVarying3 == 1.0 ) ) {

						let nodeConst3 = ( nodeVarying7 - vec2<f32>( 0.5 ) );
						let nodeConst4 = ( ( abs( nodeConst3 ) - vec2<f32>( 0.47, 0.43 ) ) + vec2<f32>( 0.045 ) );
						let nodeConst5 = ( ( length( max( nodeConst4, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst4.x, nodeConst4.y ), 0.0 ) ) - 0.045 );
						let nodeConst6 = max( fwidth( nodeConst5 ), 0.001 );
						let nodeConst7 = ( abs( nodeVarying8.x ) > 0.5 );

						if ( nodeConst7 ) {

							let nodeConst8 = abs( ( nodeVarying5.z - nodeUniform6.value[ 0u ].x ) );
							nodeVar5 = smoothstep( 0.026, 0.035, nodeConst8 );

						} else {

							nodeVar5 = 1.0;

						}


						if ( nodeConst7 ) {

							let nodeConst9 = abs( ( nodeVarying5.z - nodeUniform6.value[ 1u ].x ) );
							nodeVar6 = smoothstep( 0.026, 0.035, nodeConst9 );

						} else {

							nodeVar6 = 1.0;

						}

						let nodeConst10 = ( ( abs( nodeConst3 ) - vec2<f32>( 0.455, 0.405 ) ) + vec2<f32>( 0.035 ) );
						let nodeConst11 = ( ( length( max( nodeConst10, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst10.x, nodeConst10.y ), 0.0 ) ) - 0.035 );
						let nodeConst12 = max( fwidth( nodeConst11 ), 0.001 );

						if ( nodeConst7 ) {

							let nodeConst8 = abs( ( nodeVarying5.z - nodeUniform6.value[ 0u ].x ) );
							nodeVar7 = smoothstep( 0.043, 0.053, nodeConst8 );

						} else {

							nodeVar7 = 1.0;

						}


						if ( nodeConst7 ) {

							let nodeConst9 = abs( ( nodeVarying5.z - nodeUniform6.value[ 1u ].x ) );
							nodeVar8 = smoothstep( 0.043, 0.053, nodeConst9 );

						} else {

							nodeVar8 = 1.0;

						}

						nodeVar4 = mix( mix( nodeVarying6, vec3<f32>( 0.006512090790025684, 0.00972121731707524, 0.011612245176281512 ), ( ( smoothstep( nodeConst6, ( - nodeConst6 ), nodeConst5 ) * nodeVar5 ) * nodeVar6 ) ), vec3<f32>( 0.012286488353353374, 0.024157632443547246, 0.035601314869097636 ), ( ( smoothstep( nodeConst12, ( - nodeConst12 ), nodeConst11 ) * nodeVar7 ) * nodeVar8 ) );

					} else {


						if ( ( ( nodeVarying3 == 5.0 ) && ( nodeVarying8.z < -0.5 ) ) ) {

							nodeVar9 = vec3<f32>( 0.16202937562896222, 0.21586050010324417, 0.2541520943200296 );

						} else {

							let nodeConst1 = vec2<f32>( ( abs( nodeVarying5.z ) - object.nodeUniform5 ), ( nodeVarying5.y - object.nodeUniform4 ) );
							let nodeConst2 = length( nodeConst1 );
							let nodeConst13 = smoothstep( 0.65, 0.85, abs( nodeVarying5.x ) );
							let nodeConst14 = ( ( abs( vec2<f32>( ( nodeVarying5.z - object.nodeUniform8.x ), ( nodeVarying5.y - ( object.nodeUniform9 - 0.1 ) ) ) ) - vec2<f32>( 0.06, 0.011 ) ) + vec2<f32>( 0.006 ) );
							let nodeConst15 = ( ( length( max( nodeConst14, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst14.x, nodeConst14.y ), 0.0 ) ) - 0.006 );
							let nodeConst16 = max( fwidth( nodeConst15 ), 0.001 );
							let nodeConst17 = ( ( abs( vec2<f32>( ( nodeVarying5.z - object.nodeUniform8.y ), ( nodeVarying5.y - ( object.nodeUniform9 - 0.1 ) ) ) ) - vec2<f32>( 0.06, 0.011 ) ) + vec2<f32>( 0.006 ) );
							let nodeConst18 = ( ( length( max( nodeConst17, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst17.x, nodeConst17.y ), 0.0 ) ) - 0.006 );
							let nodeConst19 = max( fwidth( nodeConst18 ), 0.001 );
							let nodeConst20 = ( ( abs( vec2<f32>( nodeVarying5.x, ( nodeVarying5.y - ( object.nodeUniform10.x - 0.025 ) ) ) ) - vec2<f32>( 0.3, 0.075 ) ) + vec2<f32>( 0.025 ) );
							let nodeConst21 = ( ( length( max( nodeConst20, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst20.x, nodeConst20.y ), 0.0 ) ) - 0.025 );
							let nodeConst22 = max( fwidth( nodeConst21 ), 0.001 );
							let nodeConst23 = ( nodeVarying3 == 7.0 );

							if ( nodeConst23 ) {

								nodeVar10 = 1.0;

							} else {

								nodeVar10 = 0.0;

							}


							if ( nodeConst23 ) {

								nodeVar11 = object.nodeUniform10.x;

							} else {

								nodeVar11 = object.nodeUniform10.y;

							}

							let nodeConst24 = ( ( abs( vec2<f32>( nodeVarying5.x, ( nodeVarying5.y - ( nodeVar11 - 0.29 ) ) ) ) - vec2<f32>( 0.66, 0.045 ) ) + vec2<f32>( 0.03 ) );
							let nodeConst25 = ( ( length( max( nodeConst24, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst24.x, nodeConst24.y ), 0.0 ) ) - 0.03 );
							let nodeConst26 = max( fwidth( nodeConst25 ), 0.001 );
							let nodeConst27 = ( nodeConst23 || ( nodeVarying3 == 8.0 ) );

							if ( nodeConst27 ) {

								nodeVar12 = 1.0;

							} else {

								nodeVar12 = 0.0;

							}


							if ( nodeConst23 ) {

								nodeVar13 = ( object.nodeUniform10.x - 0.2 );

							} else {

								nodeVar13 = ( object.nodeUniform10.y - 0.17 );

							}

							let nodeConst28 = ( ( abs( vec2<f32>( nodeVarying5.x, ( nodeVarying5.y - nodeVar13 ) ) ) - vec2<f32>( 0.155, 0.055 ) ) + vec2<f32>( 0.008 ) );
							let nodeConst29 = ( ( length( max( nodeConst28, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst28.x, nodeConst28.y ), 0.0 ) ) - 0.008 );
							let nodeConst30 = max( fwidth( nodeConst29 ), 0.001 );

							if ( nodeConst27 ) {

								nodeVar14 = 1.0;

							} else {

								nodeVar14 = 0.0;

							}


							if ( nodeConst23 ) {

								nodeVar15 = vec3<f32>( 0.6375968739867731, 0.775822218312646, 0.8307698767709715 );

							} else {

								nodeVar15 = vec3<f32>( 0.19806931954941637, 0.005181516700061659, 0.007499032040460618 );

							}

							let nodeConst31 = ( ( abs( vec2<f32>( ( abs( nodeVarying5.x ) - 0.61 ), ( nodeVarying5.y - nodeVar11 ) ) ) - vec2<f32>( 0.19, 0.055 ) ) + vec2<f32>( 0.018 ) );
							let nodeConst32 = ( ( length( max( nodeConst31, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst31.x, nodeConst31.y ), 0.0 ) ) - 0.018 );
							let nodeConst33 = max( fwidth( nodeConst32 ), 0.001 );

							if ( nodeConst27 ) {

								nodeVar16 = 1.0;

							} else {

								nodeVar16 = 0.0;

							}

							nodeVar9 = mix( mix( mix( ( ( nodeVarying6 * vec3<f32>( ( mix( 1.0, ( ( smoothstep( ( object.nodeUniform4 + 0.025 ), ( object.nodeUniform4 + 0.095 ), nodeConst2 ) * 0.28 ) + 0.72 ), nodeConst13 ) * ( ( smoothstep( 0.25, 0.65, nodeVarying5.y ) * 0.3 ) + 0.7 ) ) ) ) * vec3<f32>( ( 1.0 - ( ( ( ( max( max( max( max( max( 0.0, smoothstep( 0.012, 0.004, abs( ( nodeVarying5.z - object.nodeUniform7.x ) ) ) ), smoothstep( 0.012, 0.004, abs( ( nodeVarying5.z - object.nodeUniform7.y ) ) ) ), smoothstep( 0.012, 0.004, abs( ( nodeVarying5.z - object.nodeUniform7.z ) ) ) ), smoothstep( nodeConst16, ( - nodeConst16 ), nodeConst15 ) ), smoothstep( nodeConst19, ( - nodeConst19 ), nodeConst18 ) ) * nodeConst13 ) * smoothstep( 0.36, 0.43, nodeVarying5.y ) ) * smoothstep( object.nodeUniform9, ( object.nodeUniform9 - 0.025 ), nodeVarying5.y ) ) * 0.5 ) ) ) ), ( vec3<f32>( 0.008023192982520563, 0.010329823026364548, 0.011612245176281512 ) * vec3<f32>( ( ( step( 0.5, fract( ( nodeVarying5.y * 65.0 ) ) ) * 0.35 ) + 0.65 ) ) ), max( ( smoothstep( nodeConst22, ( - nodeConst22 ), nodeConst21 ) * nodeVar10 ), ( smoothstep( nodeConst26, ( - nodeConst26 ), nodeConst25 ) * nodeVar12 ) ) ), vec3<f32>( 0.6866853124288864, 0.6938717612856897, 0.6514056374127929 ), ( smoothstep( nodeConst30, ( - nodeConst30 ), nodeConst29 ) * nodeVar14 ) ), nodeVar15, ( smoothstep( nodeConst33, ( - nodeConst33 ), nodeConst32 ) * nodeVar16 ) );

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

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( nodeVar0, 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform11 );
	let nodeConst34 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst34;

	// result

	output.color = nodeConst34;

	return output;

}
