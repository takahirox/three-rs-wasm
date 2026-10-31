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

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform8 : mat4x4<f32>
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
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> Output : vec4<f32>;

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

			let nodeConst1 = ( nodeVarying6.y * 6.283185307179586 );
			let nodeConst2 = vec3<f32>( ( sin( nodeConst1 ) * 0.09 ), nodeVarying6.x, ( cos( nodeConst1 ) * 0.09 ) );
			let nodeConst3 = smoothstep( 0.013, 0.006, abs( ( abs( nodeConst2.x ) - 0.036 ) ) );
			let nodeConst4 = smoothstep( 0.045, 0.075, nodeConst2.z );
			let nodeConst5 = fract( ( sin( ( ( nodeVarying5 + 13.0 ) * 12.9898 ) ) * 43758.5453 ) );
			let nodeConst6 = ( mix( mix( 1.55, 1.61, nodeConst5 ), mix( 1.67, 1.71, nodeConst5 ), smoothstep( -0.025, 0.07, nodeConst2.z ) ) + ( ( nodeConst2.x * ( nodeConst5 - 0.5 ) ) * 0.35 ) );
			nodeVar1 = mix( ( array< vec3<f32>, 5 >( vec3<f32>( 0.5647115056965487, 0.24620132669705552, 0.12477181755144427 ), vec3<f32>( 0.3967552307153359, 0.158960835050774, 0.0722718506743852 ), vec3<f32>( 0.2541520943200296, 0.09084171117479915, 0.035601314869097636 ), vec3<f32>( 0.14702726648767014, 0.04666508633021928, 0.01764195448412081 ), vec3<f32>( 0.6938717612856897, 0.3515325994898463, 0.1844749944900301 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 17.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( ( ( ( nodeConst3 * smoothstep( 0.005, 0.0015, abs( ( nodeConst2.y - 1.651 ) ) ) ) * nodeConst4 ) * 0.65 ) + ( ( ( nodeConst3 * smoothstep( 0.0035, 0.001, abs( ( nodeConst2.y - 1.665 ) ) ) ) * nodeConst4 ) * 0.35 ) ) + ( ( ( smoothstep( 0.03, 0.016, abs( nodeConst2.x ) ) * smoothstep( 0.004, 0.001, abs( ( nodeConst2.y - 1.58 ) ) ) ) * nodeConst4 ) * 0.25 ) ) ) ) ), array< vec3<f32>, 5 >( vec3<f32>( 0.010329823026364548, 0.007499032040460618, 0.006048833020386069 ), vec3<f32>( 0.04231141061442144, 0.023153366173251363, 0.010329823026364548 ), vec3<f32>( 0.09758734713304495, 0.05126945836711539, 0.015996293361446288 ), vec3<f32>( 0.15592646369776456, 0.13843161502267545, 0.12213877222015301 ), vec3<f32>( 0.023153366173251363, 0.019382360952473074, 0.01764195448412081 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 3.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ], smoothstep( ( nodeConst6 - 0.003 ), ( nodeConst6 + 0.003 ), nodeConst2.y ) );

		} else {


			if ( ( nodeVarying3 == 2.0 ) ) {

				let nodeConst7 = ( fract( ( sin( ( ( nodeVarying5 + 89.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.42 );

				if ( nodeConst7 ) {

					let nodeConst8 = ( nodeVarying7 * vec3<f32>( object.nodeUniform4 ) );
					let nodeConst9 = ( ( nodeConst8.y - 1.33 ) * 0.42 );
					let nodeConst10 = smoothstep( 0.03, 0.07, nodeConst8.z );
					let nodeConst11 = ( ( ( smoothstep( 1.33, 1.35, nodeConst8.y ) * smoothstep( 1.5, 1.48, nodeConst8.y ) ) * smoothstep( 0.004, 0.0, ( abs( nodeConst8.x ) - nodeConst9 ) ) ) * nodeConst10 );
					let nodeConst12 = ( smoothstep( 0.008, 0.003, abs( nodeConst8.x ) ) * nodeConst10 );
					nodeVar3 = mix( ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( ( ( ( smoothstep( 0.025, 0.009, abs( ( abs( nodeConst8.x ) - nodeConst9 ) ) ) * ( 1.0 - nodeConst11 ) ) * smoothstep( 1.32, 1.37, nodeConst8.y ) ) * nodeConst10 ) * 0.25 ) + ( nodeConst12 * 0.35 ) ) ) ) ), array< vec3<f32>, 4 >( vec3<f32>( 0.8069522576650873, 0.7912979403281551, 0.7454042095350284 ), vec3<f32>( 0.4793201830913402, 0.55201140150344, 0.6866853124288864 ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4793201830913402 ), vec3<f32>( 0.3231432091022285, 0.38642943377667954, 0.4793201830913402 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 71.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 4.0 ) ), 3.0 ) ) ], nodeConst11 );

				} else {

					let nodeConst8 = ( nodeVarying7 * vec3<f32>( object.nodeUniform4 ) );
					let nodeConst10 = smoothstep( 0.03, 0.07, nodeConst8.z );
					let nodeConst12 = ( smoothstep( 0.008, 0.003, abs( nodeConst8.x ) ) * nodeConst10 );
					nodeVar3 = ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( max( smoothstep( 1.065, 1.035, nodeConst8.y ), smoothstep( 1.455, 1.48, nodeConst8.y ) ) * 0.18 ) + ( nodeConst12 * 0.3 ) ) ) ) );

				}

				nodeVar2 = nodeVar3;

			} else {


				if ( ( nodeVarying3 == 6.0 ) ) {

					let nodeConst7 = ( fract( ( sin( ( ( nodeVarying5 + 89.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.42 );

					if ( nodeConst7 ) {

						nodeVar5 = array< vec3<f32>, 4 >( vec3<f32>( 0.8069522576650873, 0.7912979403281551, 0.7454042095350284 ), vec3<f32>( 0.4793201830913402, 0.55201140150344, 0.6866853124288864 ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4793201830913402 ), vec3<f32>( 0.3231432091022285, 0.38642943377667954, 0.4793201830913402 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 71.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 4.0 ) ), 3.0 ) ) ];

					} else {

						nodeVar5 = ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( 0.75 ) );

					}

					nodeVar4 = mix( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ], nodeVar5, smoothstep( 0.94, 0.985, nodeVarying6.x ) );

				} else {


					if ( ( nodeVarying3 == 3.0 ) ) {

						nodeVar6 = array< vec3<f32>, 5 >( vec3<f32>( 0.015996293361446288, 0.01764195448412081, 0.025186859622305935 ), vec3<f32>( 0.035601314869097636, 0.06480326668529614, 0.10461648408208657 ), vec3<f32>( 0.08228270712149792, 0.06301001764564068, 0.04666508633021928 ), vec3<f32>( 0.20155625378383743, 0.16513219449147767, 0.11697066774917994 ), vec3<f32>( 0.018500220124016652, 0.018500220124016652, 0.019382360952473074 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying5 + 47.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ];

					} else {


						if ( ( nodeVarying3 == 4.0 ) ) {

							let nodeConst13 = ( fract( ( sin( ( ( nodeVarying5 + 5.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.4 );

							if ( nodeConst13 ) {

								nodeVar8 = mix( vec3<f32>( 0.018500220124016652, 0.024157632443547246, 0.031896033067374104 ), vec3<f32>( 0.5775804404214573, 0.5457244613615395, 0.4793201830913402 ), step( 0.55, fract( ( sin( ( ( nodeVarying5 + 31.0 ) * 12.9898 ) ) * 43758.5453 ) ) ) );

							} else {

								nodeVar8 = mix( vec3<f32>( 0.014443843592229466, 0.012983032338510335, 0.011612245176281512 ), vec3<f32>( 0.06847816983662762, 0.035601314869097636, 0.01764195448412081 ), fract( ( sin( ( ( nodeVarying5 + 23.0 ) * 12.9898 ) ) * 43758.5453 ) ) );

							}


							if ( nodeConst13 ) {

								nodeVar9 = vec3<f32>( 0.6653872982754769, 0.6375968739867731, 0.5775804404214573 );

							} else {

								nodeVar9 = vec3<f32>( 0.00972121731707524, 0.009134058699157796, 0.008023192982520563 );

							}

							let nodeConst14 = ( nodeVarying6.y * 110.0 );
							nodeVar7 = mix( mix( nodeVar8, nodeVar9, smoothstep( -0.064, -0.074, nodeVarying6.x ) ), vec3<f32>( 0.407240211891531, 0.3915724777393922, 0.34670405634441115 ), ( ( ( ( smoothstep( 0.68, 0.78, fract( nodeConst14 ) ) * clamp( ( 1.0 - fwidth( nodeConst14 ) ), 0.0, 1.0 ) ) * smoothstep( -0.015, 0.005, nodeVarying6.x ) ) * smoothstep( -0.01, 0.015, nodeVarying6.y ) ) * 0.45 ) );

						} else {


							if ( ( nodeVarying3 == 5.0 ) ) {

								let nodeConst15 = min( nodeVarying6, ( vec2<f32>( 1.0 ) - nodeVarying6 ) );

								if ( ( abs( nodeVarying8.x ) > 0.8 ) ) {

									nodeVar11 = 1.0;

								} else {

									nodeVar11 = 0.0;

								}

								nodeVar10 = mix( ( vec3<f32>( 0.06662593863608139, 0.038204371589236, 0.023153366173251363 ) * vec3<f32>( ( 1.0 - ( max( smoothstep( 0.045, 0.02, min( nodeConst15.x, nodeConst15.y ) ), smoothstep( 0.02, 0.008, abs( ( nodeVarying6.y - 0.72 ) ) ) ) * 0.35 ) ) ) ), vec3<f32>( 0.2746773120495699, 0.24620132669705552, 0.1878207722902346 ), ( ( smoothstep( 0.06, 0.045, abs( ( nodeVarying6.x - 0.5 ) ) ) * smoothstep( 0.07, 0.05, abs( ( nodeVarying6.y - 0.72 ) ) ) ) * nodeVar11 ) );

							} else {

								nodeVar10 = vec3<f32>( 0.02955683443236377, 0.019382360952473074, 0.013702083043526807 );

							}

							nodeVar7 = nodeVar10;

						}

						nodeVar6 = nodeVar7;

					}

					nodeVar4 = nodeVar6;

				}

				nodeVar2 = nodeVar4;

			}

			nodeVar1 = nodeVar2;

		}

		nodeVar0 = nodeVar1;

	}

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( nodeVar0, 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform5 );
	let nodeConst16 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst16;

	// result

	output.color = nodeConst16;

	return output;

}
