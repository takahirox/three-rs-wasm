// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform5 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : bool;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> Output : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) nodeVarying5 : f32,
	@location( 2 ) nodeVarying6 : vec2<f32>,
	@location( 3 ) nodeVarying7 : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32> ) -> OutputStruct {

	// flow
	// code


	if ( ( nodeVarying3 == 0.0 ) ) {

		nodeVar0 = array< vec3<f32>, 5 >( vec3<f32>( 0.5647115056965487, 0.24620132669705552, 0.12477181755144427 ), vec3<f32>( 0.3967552307153359, 0.158960835050774, 0.0722718506743852 ), vec3<f32>( 0.2541520943200296, 0.09084171117479915, 0.035601314869097636 ), vec3<f32>( 0.14702726648767014, 0.04666508633021928, 0.01764195448412081 ), vec3<f32>( 0.6938717612856897, 0.3515325994898463, 0.1844749944900301 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 17.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ];

	} else {


		if ( ( nodeVarying3 == 1.0 ) ) {

			nodeVar2 = ( nodeVarying6.y * 6.283185307179586 );
			nodeVar3 = vec3<f32>( ( sin( nodeVar2 ) * 0.09 ), nodeVarying6.x, ( cos( nodeVar2 ) * 0.09 ) );
			nodeVar4 = smoothstep( 0.013, 0.006, abs( ( abs( nodeVar3.x ) - 0.036 ) ) );
			nodeVar5 = smoothstep( 0.045, 0.075, nodeVar3.z );
			nodeVar6 = fract( ( sin( ( ( nodeVarying5 + 13.0 ) * 12.9898 ) ) * 43758.5453 ) );
			nodeVar7 = ( mix( mix( 1.55, 1.61, nodeVar6 ), mix( 1.67, 1.71, nodeVar6 ), smoothstep( -0.025, 0.07, nodeVar3.z ) ) + ( ( nodeVar3.x * ( nodeVar6 - 0.5 ) ) * 0.35 ) );
			nodeVar1 = mix( ( array< vec3<f32>, 5 >( vec3<f32>( 0.5647115056965487, 0.24620132669705552, 0.12477181755144427 ), vec3<f32>( 0.3967552307153359, 0.158960835050774, 0.0722718506743852 ), vec3<f32>( 0.2541520943200296, 0.09084171117479915, 0.035601314869097636 ), vec3<f32>( 0.14702726648767014, 0.04666508633021928, 0.01764195448412081 ), vec3<f32>( 0.6938717612856897, 0.3515325994898463, 0.1844749944900301 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 17.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( ( ( ( nodeVar4 * smoothstep( 0.005, 0.0015, abs( ( nodeVar3.y - 1.651 ) ) ) ) * nodeVar5 ) * 0.65 ) + ( ( ( nodeVar4 * smoothstep( 0.0035, 0.001, abs( ( nodeVar3.y - 1.665 ) ) ) ) * nodeVar5 ) * 0.35 ) ) + ( ( ( smoothstep( 0.03, 0.016, abs( nodeVar3.x ) ) * smoothstep( 0.004, 0.001, abs( ( nodeVar3.y - 1.58 ) ) ) ) * nodeVar5 ) * 0.25 ) ) ) ) ), array< vec3<f32>, 5 >( vec3<f32>( 0.010329823026364548, 0.007499032040460618, 0.006048833020386069 ), vec3<f32>( 0.04231141061442144, 0.023153366173251363, 0.010329823026364548 ), vec3<f32>( 0.09758734713304495, 0.05126945836711539, 0.015996293361446288 ), vec3<f32>( 0.15592646369776456, 0.13843161502267545, 0.12213877222015301 ), vec3<f32>( 0.023153366173251363, 0.019382360952473074, 0.01764195448412081 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 3.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ], smoothstep( ( nodeVar7 - 0.003 ), ( nodeVar7 + 0.003 ), nodeVar3.y ) );

		} else {


			if ( ( nodeVarying3 == 2.0 ) ) {


				if ( ( fract( ( sin( ( ( nodeVarying5 + 89.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.42 ) ) {

					nodeVar10 = ( nodeVarying7 * vec3<f32>( object.nodeUniform1 ) );
					nodeVar11 = ( ( nodeVar10.y - 1.33 ) * 0.42 );
					nodeVar12 = smoothstep( 0.03, 0.07, nodeVar10.z );
					nodeVar13 = ( ( ( smoothstep( 1.33, 1.35, nodeVar10.y ) * smoothstep( 1.5, 1.48, nodeVar10.y ) ) * smoothstep( 0.004, 0.0, ( abs( nodeVar10.x ) - nodeVar11 ) ) ) * nodeVar12 );
					nodeVar9 = mix( ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( ( ( ( smoothstep( 0.025, 0.009, abs( ( abs( nodeVar10.x ) - nodeVar11 ) ) ) * ( 1.0 - nodeVar13 ) ) * smoothstep( 1.32, 1.37, nodeVar10.y ) ) * nodeVar12 ) * 0.25 ) + ( ( smoothstep( 0.008, 0.003, abs( nodeVar10.x ) ) * nodeVar12 ) * 0.35 ) ) ) ) ), array< vec3<f32>, 4 >( vec3<f32>( 0.8069522576650873, 0.7912979403281551, 0.7454042095350284 ), vec3<f32>( 0.4793201830913402, 0.55201140150344, 0.6866853124288864 ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4793201830913402 ), vec3<f32>( 0.3231432091022285, 0.38642943377667954, 0.4793201830913402 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 71.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 4.0 ) ), 3.0 ) ) ], nodeVar13 );

				} else {

					nodeVar14 = ( nodeVarying7 * vec3<f32>( object.nodeUniform1 ) );
					nodeVar9 = ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( max( smoothstep( 1.065, 1.035, nodeVar14.y ), smoothstep( 1.455, 1.48, nodeVar14.y ) ) * 0.18 ) + ( ( smoothstep( 0.008, 0.003, abs( nodeVar14.x ) ) * smoothstep( 0.03, 0.07, nodeVar14.z ) ) * 0.3 ) ) ) ) );

				}

				nodeVar8 = nodeVar9;

			} else {


				if ( ( nodeVarying3 == 6.0 ) ) {


					if ( ( fract( ( sin( ( ( nodeVarying5 + 89.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.42 ) ) {

						nodeVar16 = array< vec3<f32>, 4 >( vec3<f32>( 0.8069522576650873, 0.7912979403281551, 0.7454042095350284 ), vec3<f32>( 0.4793201830913402, 0.55201140150344, 0.6866853124288864 ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4793201830913402 ), vec3<f32>( 0.3231432091022285, 0.38642943377667954, 0.4793201830913402 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 71.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 4.0 ) ), 3.0 ) ) ];

					} else {

						nodeVar16 = ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( 0.75 ) );

					}

					nodeVar15 = mix( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ], nodeVar16, smoothstep( 0.94, 0.985, nodeVarying6.x ) );

				} else {


					if ( ( nodeVarying3 == 3.0 ) ) {

						nodeVar17 = array< vec3<f32>, 5 >( vec3<f32>( 0.015996293361446288, 0.01764195448412081, 0.025186859622305935 ), vec3<f32>( 0.035601314869097636, 0.06480326668529614, 0.10461648408208657 ), vec3<f32>( 0.08228270712149792, 0.06301001764564068, 0.04666508633021928 ), vec3<f32>( 0.20155625378383743, 0.16513219449147767, 0.11697066774917994 ), vec3<f32>( 0.018500220124016652, 0.018500220124016652, 0.019382360952473074 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 47.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ];

					} else {


						if ( ( nodeVarying3 == 4.0 ) ) {

							nodeVar20 = ( fract( ( sin( ( ( nodeVarying5 + 5.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.4 );

							if ( nodeVar20 ) {

								nodeVar19 = mix( vec3<f32>( 0.018500220124016652, 0.024157632443547246, 0.031896033067374104 ), vec3<f32>( 0.5775804404214573, 0.5457244613615395, 0.4793201830913402 ), step( 0.55, fract( ( sin( ( ( nodeVarying5 + 31.0 ) * 12.9898 ) ) * 43758.5453 ) ) ) );

							} else {

								nodeVar19 = mix( vec3<f32>( 0.014443843592229466, 0.012983032338510335, 0.011612245176281512 ), vec3<f32>( 0.06847816983662762, 0.035601314869097636, 0.01764195448412081 ), fract( ( sin( ( ( nodeVarying5 + 23.0 ) * 12.9898 ) ) * 43758.5453 ) ) );

							}


							if ( nodeVar20 ) {

								nodeVar21 = vec3<f32>( 0.6653872982754769, 0.6375968739867731, 0.5775804404214573 );

							} else {

								nodeVar21 = vec3<f32>( 0.00972121731707524, 0.009134058699157796, 0.008023192982520563 );

							}

							nodeVar22 = ( nodeVarying6.y * 110.0 );
							nodeVar18 = mix( mix( nodeVar19, nodeVar21, smoothstep( -0.064, -0.074, nodeVarying6.x ) ), vec3<f32>( 0.407240211891531, 0.3915724777393922, 0.34670405634441115 ), ( ( ( ( smoothstep( 0.68, 0.78, fract( nodeVar22 ) ) * clamp( ( 1.0 - fwidth( nodeVar22 ) ), 0.0, 1.0 ) ) * smoothstep( -0.015, 0.005, nodeVarying6.x ) ) * smoothstep( -0.01, 0.015, nodeVarying6.y ) ) * 0.45 ) );

						} else {


							if ( ( nodeVarying3 == 5.0 ) ) {

								nodeVar24 = min( nodeVarying6, ( vec2<f32>( 1.0 ) - nodeVarying6 ) );

								if ( ( abs( nodeVarying8.x ) > 0.8 ) ) {

									nodeVar25 = 1.0;

								} else {

									nodeVar25 = 0.0;

								}

								nodeVar23 = mix( ( vec3<f32>( 0.06662593863608139, 0.038204371589236, 0.023153366173251363 ) * vec3<f32>( ( 1.0 - ( max( smoothstep( 0.045, 0.02, min( nodeVar24.x, nodeVar24.y ) ), smoothstep( 0.02, 0.008, abs( ( nodeVarying6.y - 0.72 ) ) ) ) * 0.35 ) ) ) ), vec3<f32>( 0.2746773120495699, 0.24620132669705552, 0.1878207722902346 ), ( ( smoothstep( 0.06, 0.045, abs( ( nodeVarying6.x - 0.5 ) ) ) * smoothstep( 0.07, 0.05, abs( ( nodeVarying6.y - 0.72 ) ) ) ) * nodeVar25 ) );

							} else {

								nodeVar23 = vec3<f32>( 0.02955683443236377, 0.019382360952473074, 0.013702083043526807 );

							}

							nodeVar18 = nodeVar23;

						}

						nodeVar17 = nodeVar18;

					}

					nodeVar15 = nodeVar17;

				}

				nodeVar8 = nodeVar15;

			}

			nodeVar1 = nodeVar8;

		}

		nodeVar0 = nodeVar1;

	}

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( nodeVar0, 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	nodeVar26 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar26;

	// result

	output.color = nodeVar26;

	return output;

}
