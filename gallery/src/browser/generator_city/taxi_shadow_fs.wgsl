// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct NodeBuffer_10726Struct {
	value : array< vec4<f32>, 2 >
};
@binding( 1 ) @group( 1 )
var<uniform> NodeBuffer_10726 : NodeBuffer_10726Struct;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : vec2<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : vec2<f32>,
	nodeUniform8 : f32,
	nodeUniform11 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : bool;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : bool;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : bool;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec2<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> Output : vec4<f32>;
var<private> nodeVar48 : vec4<f32>;

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

		nodeVar0 = ( vec3<f32>( 0.009134058699157796, 0.00972121731707524, 0.010960094003125918 ) * vec3<f32>( ( ( smoothstep( ( object.nodeUniform1 * 0.85 ), ( object.nodeUniform1 * 0.94 ), length( vec2<f32>( ( abs( nodeVarying5.z ) - object.nodeUniform2 ), ( nodeVarying5.y - object.nodeUniform1 ) ) ) ) * 0.2 ) + 0.8 ) ) );

	} else {


		if ( ( nodeVarying3 == 3.0 ) ) {

			nodeVar2 = vec2<f32>( ( abs( nodeVarying5.z ) - object.nodeUniform2 ), ( nodeVarying5.y - object.nodeUniform1 ) );
			nodeVar3 = length( nodeVar2 );
			nodeVar1 = mix( vec3<f32>( 0.008568125615105716, 0.010960094003125918, 0.014443843592229466 ), vec3<f32>( 0.4286904966038916, 0.46778379610254284, 0.4910208498384856 ), max( max( smoothstep( 0.62, 0.42, ( abs( ( fract( ( atan2( nodeVar2.y, nodeVar2.x ) * 0.7957747154594768 ) ) - 0.5 ) ) * 2.0 ) ), smoothstep( ( object.nodeUniform1 * 0.53 ), ( object.nodeUniform1 * 0.59 ), nodeVar3 ) ), smoothstep( 0.06, 0.035, nodeVar3 ) ) );

		} else {


			if ( ( nodeVarying3 == 4.0 ) ) {

				nodeVar4 = vec3<f32>( 0.014443843592229466, 0.01680737574872402, 0.019382360952473074 );

			} else {


				if ( ( nodeVarying3 == 6.0 ) ) {

					nodeVar5 = vec3<f32>( 1.0, 0.6724431569510133, 0.14412847084818123 );

				} else {


					if ( ( nodeVarying3 == 1.0 ) ) {

						nodeVar7 = ( nodeVarying7 - vec2<f32>( 0.5 ) );
						nodeVar8 = ( ( abs( nodeVar7 ) - vec2<f32>( 0.47, 0.43 ) ) + vec2<f32>( 0.045 ) );
						nodeVar9 = ( ( length( max( nodeVar8, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar8.x, nodeVar8.y ), 0.0 ) ) - 0.045 );
						nodeVar10 = max( fwidth( nodeVar9 ), 0.001 );
						nodeVar12 = ( abs( nodeVarying8.x ) > 0.5 );

						if ( nodeVar12 ) {

							nodeVar11 = smoothstep( 0.026, 0.035, abs( ( nodeVarying5.z - NodeBuffer_10726.value[ 0u ].x ) ) );

						} else {

							nodeVar11 = 1.0;

						}


						if ( nodeVar12 ) {

							nodeVar13 = smoothstep( 0.026, 0.035, abs( ( nodeVarying5.z - NodeBuffer_10726.value[ 1u ].x ) ) );

						} else {

							nodeVar13 = 1.0;

						}

						nodeVar14 = ( ( abs( nodeVar7 ) - vec2<f32>( 0.455, 0.405 ) ) + vec2<f32>( 0.035 ) );
						nodeVar15 = ( ( length( max( nodeVar14, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar14.x, nodeVar14.y ), 0.0 ) ) - 0.035 );
						nodeVar16 = max( fwidth( nodeVar15 ), 0.001 );

						if ( nodeVar12 ) {

							nodeVar17 = smoothstep( 0.043, 0.053, abs( ( nodeVarying5.z - NodeBuffer_10726.value[ 0u ].x ) ) );

						} else {

							nodeVar17 = 1.0;

						}


						if ( nodeVar12 ) {

							nodeVar18 = smoothstep( 0.043, 0.053, abs( ( nodeVarying5.z - NodeBuffer_10726.value[ 1u ].x ) ) );

						} else {

							nodeVar18 = 1.0;

						}

						nodeVar6 = mix( mix( nodeVarying6, vec3<f32>( 0.006512090790025684, 0.00972121731707524, 0.011612245176281512 ), ( ( smoothstep( nodeVar10, ( - nodeVar10 ), nodeVar9 ) * nodeVar11 ) * nodeVar13 ) ), vec3<f32>( 0.012286488353353374, 0.024157632443547246, 0.035601314869097636 ), ( ( smoothstep( nodeVar16, ( - nodeVar16 ), nodeVar15 ) * nodeVar17 ) * nodeVar18 ) );

					} else {


						if ( ( ( nodeVarying3 == 5.0 ) && ( nodeVarying8.z < -0.5 ) ) ) {

							nodeVar19 = vec3<f32>( 0.16202937562896222, 0.21586050010324417, 0.2541520943200296 );

						} else {

							nodeVar20 = smoothstep( 0.65, 0.85, abs( nodeVarying5.x ) );
							nodeVar21 = ( ( abs( vec2<f32>( ( nodeVarying5.z - object.nodeUniform5.x ), ( nodeVarying5.y - ( object.nodeUniform6 - 0.1 ) ) ) ) - vec2<f32>( 0.06, 0.011 ) ) + vec2<f32>( 0.006 ) );
							nodeVar22 = ( ( length( max( nodeVar21, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar21.x, nodeVar21.y ), 0.0 ) ) - 0.006 );
							nodeVar23 = max( fwidth( nodeVar22 ), 0.001 );
							nodeVar24 = ( ( abs( vec2<f32>( ( nodeVarying5.z - object.nodeUniform5.y ), ( nodeVarying5.y - ( object.nodeUniform6 - 0.1 ) ) ) ) - vec2<f32>( 0.06, 0.011 ) ) + vec2<f32>( 0.006 ) );
							nodeVar25 = ( ( length( max( nodeVar24, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar24.x, nodeVar24.y ), 0.0 ) ) - 0.006 );
							nodeVar26 = max( fwidth( nodeVar25 ), 0.001 );
							nodeVar27 = ( ( abs( vec2<f32>( nodeVarying5.x, ( nodeVarying5.y - ( object.nodeUniform7.x - 0.025 ) ) ) ) - vec2<f32>( 0.3, 0.075 ) ) + vec2<f32>( 0.025 ) );
							nodeVar28 = ( ( length( max( nodeVar27, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar27.x, nodeVar27.y ), 0.0 ) ) - 0.025 );
							nodeVar29 = max( fwidth( nodeVar28 ), 0.001 );
							nodeVar31 = ( nodeVarying3 == 7.0 );

							if ( nodeVar31 ) {

								nodeVar30 = 1.0;

							} else {

								nodeVar30 = 0.0;

							}


							if ( nodeVar31 ) {

								nodeVar32 = object.nodeUniform7.x;

							} else {

								nodeVar32 = object.nodeUniform7.y;

							}

							nodeVar33 = ( ( abs( vec2<f32>( nodeVarying5.x, ( nodeVarying5.y - ( nodeVar32 - 0.29 ) ) ) ) - vec2<f32>( 0.66, 0.045 ) ) + vec2<f32>( 0.03 ) );
							nodeVar34 = ( ( length( max( nodeVar33, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar33.x, nodeVar33.y ), 0.0 ) ) - 0.03 );
							nodeVar35 = max( fwidth( nodeVar34 ), 0.001 );
							nodeVar37 = ( nodeVar31 || ( nodeVarying3 == 8.0 ) );

							if ( nodeVar37 ) {

								nodeVar36 = 1.0;

							} else {

								nodeVar36 = 0.0;

							}


							if ( nodeVar31 ) {

								nodeVar38 = ( object.nodeUniform7.x - 0.2 );

							} else {

								nodeVar38 = ( object.nodeUniform7.y - 0.17 );

							}

							nodeVar39 = ( ( abs( vec2<f32>( nodeVarying5.x, ( nodeVarying5.y - nodeVar38 ) ) ) - vec2<f32>( 0.155, 0.055 ) ) + vec2<f32>( 0.008 ) );
							nodeVar40 = ( ( length( max( nodeVar39, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar39.x, nodeVar39.y ), 0.0 ) ) - 0.008 );
							nodeVar41 = max( fwidth( nodeVar40 ), 0.001 );

							if ( nodeVar37 ) {

								nodeVar42 = 1.0;

							} else {

								nodeVar42 = 0.0;

							}


							if ( nodeVar31 ) {

								nodeVar43 = vec3<f32>( 0.6375968739867731, 0.775822218312646, 0.8307698767709715 );

							} else {

								nodeVar43 = vec3<f32>( 0.19806931954941637, 0.005181516700061659, 0.007499032040460618 );

							}

							nodeVar44 = ( ( abs( vec2<f32>( ( abs( nodeVarying5.x ) - 0.61 ), ( nodeVarying5.y - nodeVar32 ) ) ) - vec2<f32>( 0.19, 0.055 ) ) + vec2<f32>( 0.018 ) );
							nodeVar45 = ( ( length( max( nodeVar44, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar44.x, nodeVar44.y ), 0.0 ) ) - 0.018 );
							nodeVar46 = max( fwidth( nodeVar45 ), 0.001 );

							if ( nodeVar37 ) {

								nodeVar47 = 1.0;

							} else {

								nodeVar47 = 0.0;

							}

							nodeVar19 = mix( mix( mix( ( ( nodeVarying6 * vec3<f32>( ( mix( 1.0, ( ( smoothstep( ( object.nodeUniform1 + 0.025 ), ( object.nodeUniform1 + 0.095 ), length( vec2<f32>( ( abs( nodeVarying5.z ) - object.nodeUniform2 ), ( nodeVarying5.y - object.nodeUniform1 ) ) ) ) * 0.28 ) + 0.72 ), nodeVar20 ) * ( ( smoothstep( 0.25, 0.65, nodeVarying5.y ) * 0.3 ) + 0.7 ) ) ) ) * vec3<f32>( ( 1.0 - ( ( ( ( max( max( max( max( max( 0.0, smoothstep( 0.012, 0.004, abs( ( nodeVarying5.z - object.nodeUniform4.x ) ) ) ), smoothstep( 0.012, 0.004, abs( ( nodeVarying5.z - object.nodeUniform4.y ) ) ) ), smoothstep( 0.012, 0.004, abs( ( nodeVarying5.z - object.nodeUniform4.z ) ) ) ), smoothstep( nodeVar23, ( - nodeVar23 ), nodeVar22 ) ), smoothstep( nodeVar26, ( - nodeVar26 ), nodeVar25 ) ) * nodeVar20 ) * smoothstep( 0.36, 0.43, nodeVarying5.y ) ) * smoothstep( object.nodeUniform6, ( object.nodeUniform6 - 0.025 ), nodeVarying5.y ) ) * 0.5 ) ) ) ), ( vec3<f32>( 0.008023192982520563, 0.010329823026364548, 0.011612245176281512 ) * vec3<f32>( ( ( step( 0.5, fract( ( nodeVarying5.y * 65.0 ) ) ) * 0.35 ) + 0.65 ) ) ), max( ( smoothstep( nodeVar29, ( - nodeVar29 ), nodeVar28 ) * nodeVar30 ), ( smoothstep( nodeVar35, ( - nodeVar35 ), nodeVar34 ) * nodeVar36 ) ) ), vec3<f32>( 0.6866853124288864, 0.6938717612856897, 0.6514056374127929 ), ( smoothstep( nodeVar41, ( - nodeVar41 ), nodeVar40 ) * nodeVar42 ) ), nodeVar43, ( smoothstep( nodeVar46, ( - nodeVar46 ), nodeVar45 ) * nodeVar47 ) );

						}

						nodeVar6 = nodeVar19;

					}

					nodeVar5 = nodeVar6;

				}

				nodeVar4 = nodeVar5;

			}

			nodeVar1 = nodeVar4;

		}

		nodeVar0 = nodeVar1;

	}

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( nodeVar0, 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform8 );
	nodeVar48 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar48;

	// result

	output.color = nodeVar48;

	return output;

}
