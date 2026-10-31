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
@binding( 1 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform20_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform20 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform28_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform28 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform30_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform30 : texture_cube<f32>;

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform31 : mat4x4<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : vec2<f32>,
	nodeUniform23 : f32,
	nodeUniform29 : f32,
	nodeUniform27 : f32
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
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar15 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : vec4<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar31 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar32 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
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
	@location( 4 ) nodeVarying8 : f32,
	@location( 5 ) nodeVarying9 : vec2<f32>,
	@location( 6 ) nodeVarying10 : vec3<f32>,
	@location( 7 ) nodeVarying11 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst1 = ( nodeVarying3 == 0.0 );

	if ( nodeConst1 ) {

		nodeVar0 = array< vec3<f32>, 5 >( vec3<f32>( 0.5647115056965487, 0.24620132669705552, 0.12477181755144427 ), vec3<f32>( 0.3967552307153359, 0.158960835050774, 0.0722718506743852 ), vec3<f32>( 0.2541520943200296, 0.09084171117479915, 0.035601314869097636 ), vec3<f32>( 0.14702726648767014, 0.04666508633021928, 0.01764195448412081 ), vec3<f32>( 0.6938717612856897, 0.3515325994898463, 0.1844749944900301 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 17.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ];

	} else {

		let nodeConst2 = ( nodeVarying3 == 1.0 );

		if ( nodeConst2 ) {

			let nodeConst3 = ( nodeVarying9.y * 6.283185307179586 );
			let nodeConst4 = vec3<f32>( ( sin( nodeConst3 ) * 0.09 ), nodeVarying9.x, ( cos( nodeConst3 ) * 0.09 ) );
			let nodeConst5 = smoothstep( 0.013, 0.006, abs( ( abs( nodeConst4.x ) - 0.036 ) ) );
			let nodeConst6 = smoothstep( 0.045, 0.075, nodeConst4.z );
			let nodeConst7 = fract( ( sin( ( ( nodeVarying8 + 13.0 ) * 12.9898 ) ) * 43758.5453 ) );
			let nodeConst8 = ( mix( mix( 1.55, 1.61, nodeConst7 ), mix( 1.67, 1.71, nodeConst7 ), smoothstep( -0.025, 0.07, nodeConst4.z ) ) + ( ( nodeConst4.x * ( nodeConst7 - 0.5 ) ) * 0.35 ) );
			let nodeConst9 = smoothstep( ( nodeConst8 - 0.003 ), ( nodeConst8 + 0.003 ), nodeConst4.y );
			nodeVar1 = mix( ( array< vec3<f32>, 5 >( vec3<f32>( 0.5647115056965487, 0.24620132669705552, 0.12477181755144427 ), vec3<f32>( 0.3967552307153359, 0.158960835050774, 0.0722718506743852 ), vec3<f32>( 0.2541520943200296, 0.09084171117479915, 0.035601314869097636 ), vec3<f32>( 0.14702726648767014, 0.04666508633021928, 0.01764195448412081 ), vec3<f32>( 0.6938717612856897, 0.3515325994898463, 0.1844749944900301 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 17.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( ( ( ( nodeConst5 * smoothstep( 0.005, 0.0015, abs( ( nodeConst4.y - 1.651 ) ) ) ) * nodeConst6 ) * 0.65 ) + ( ( ( nodeConst5 * smoothstep( 0.0035, 0.001, abs( ( nodeConst4.y - 1.665 ) ) ) ) * nodeConst6 ) * 0.35 ) ) + ( ( ( smoothstep( 0.03, 0.016, abs( nodeConst4.x ) ) * smoothstep( 0.004, 0.001, abs( ( nodeConst4.y - 1.58 ) ) ) ) * nodeConst6 ) * 0.25 ) ) ) ) ), array< vec3<f32>, 5 >( vec3<f32>( 0.010329823026364548, 0.007499032040460618, 0.006048833020386069 ), vec3<f32>( 0.04231141061442144, 0.023153366173251363, 0.010329823026364548 ), vec3<f32>( 0.09758734713304495, 0.05126945836711539, 0.015996293361446288 ), vec3<f32>( 0.15592646369776456, 0.13843161502267545, 0.12213877222015301 ), vec3<f32>( 0.023153366173251363, 0.019382360952473074, 0.01764195448412081 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 3.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ], nodeConst9 );

		} else {


			if ( ( nodeVarying3 == 2.0 ) ) {

				let nodeConst10 = ( fract( ( sin( ( ( nodeVarying8 + 89.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.42 );

				if ( nodeConst10 ) {

					let nodeConst11 = ( nodeVarying10 * vec3<f32>( object.nodeUniform4 ) );
					let nodeConst12 = ( ( nodeConst11.y - 1.33 ) * 0.42 );
					let nodeConst13 = smoothstep( 0.03, 0.07, nodeConst11.z );
					let nodeConst14 = ( ( ( smoothstep( 1.33, 1.35, nodeConst11.y ) * smoothstep( 1.5, 1.48, nodeConst11.y ) ) * smoothstep( 0.004, 0.0, ( abs( nodeConst11.x ) - nodeConst12 ) ) ) * nodeConst13 );
					let nodeConst15 = ( smoothstep( 0.008, 0.003, abs( nodeConst11.x ) ) * nodeConst13 );
					nodeVar3 = mix( ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( ( ( ( smoothstep( 0.025, 0.009, abs( ( abs( nodeConst11.x ) - nodeConst12 ) ) ) * ( 1.0 - nodeConst14 ) ) * smoothstep( 1.32, 1.37, nodeConst11.y ) ) * nodeConst13 ) * 0.25 ) + ( nodeConst15 * 0.35 ) ) ) ) ), array< vec3<f32>, 4 >( vec3<f32>( 0.8069522576650873, 0.7912979403281551, 0.7454042095350284 ), vec3<f32>( 0.4793201830913402, 0.55201140150344, 0.6866853124288864 ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4793201830913402 ), vec3<f32>( 0.3231432091022285, 0.38642943377667954, 0.4793201830913402 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 71.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 4.0 ) ), 3.0 ) ) ], nodeConst14 );

				} else {

					let nodeConst11 = ( nodeVarying10 * vec3<f32>( object.nodeUniform4 ) );
					let nodeConst13 = smoothstep( 0.03, 0.07, nodeConst11.z );
					let nodeConst15 = ( smoothstep( 0.008, 0.003, abs( nodeConst11.x ) ) * nodeConst13 );
					nodeVar3 = ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( ( 1.0 - ( ( max( smoothstep( 1.065, 1.035, nodeConst11.y ), smoothstep( 1.455, 1.48, nodeConst11.y ) ) * 0.18 ) + ( nodeConst15 * 0.3 ) ) ) ) );

				}

				nodeVar2 = nodeVar3;

			} else {


				if ( ( nodeVarying3 == 6.0 ) ) {

					let nodeConst10 = ( fract( ( sin( ( ( nodeVarying8 + 89.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.42 );

					if ( nodeConst10 ) {

						nodeVar5 = array< vec3<f32>, 4 >( vec3<f32>( 0.8069522576650873, 0.7912979403281551, 0.7454042095350284 ), vec3<f32>( 0.4793201830913402, 0.55201140150344, 0.6866853124288864 ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4793201830913402 ), vec3<f32>( 0.3231432091022285, 0.38642943377667954, 0.4793201830913402 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 71.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 4.0 ) ), 3.0 ) ) ];

					} else {

						nodeVar5 = ( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ] * vec3<f32>( 0.75 ) );

					}

					nodeVar4 = mix( array< vec3<f32>, 8 >( vec3<f32>( 0.024157632443547246, 0.04231141061442144, 0.08437621153575764 ), vec3<f32>( 0.033104766565152086, 0.033104766565152086, 0.028426039499072558 ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.057805430183792694 ), vec3<f32>( 0.10224173307914941, 0.028426039499072558, 0.028426039499072558 ), vec3<f32>( 0.07818742179702069, 0.08437621153575764, 0.03954623527052923 ), vec3<f32>( 0.14412847084818123, 0.14412847084818123, 0.13286832154414627 ), vec3<f32>( 0.015996293361446288, 0.014443843592229466, 0.01764195448412081 ), vec3<f32>( 0.20507873637973145, 0.04970656597728775, 0.015996293361446288 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 29.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 8.0 ) ), 7.0 ) ) ], nodeVar5, smoothstep( 0.94, 0.985, nodeVarying9.x ) );

				} else {


					if ( ( nodeVarying3 == 3.0 ) ) {

						nodeVar6 = array< vec3<f32>, 5 >( vec3<f32>( 0.015996293361446288, 0.01764195448412081, 0.025186859622305935 ), vec3<f32>( 0.035601314869097636, 0.06480326668529614, 0.10461648408208657 ), vec3<f32>( 0.08228270712149792, 0.06301001764564068, 0.04666508633021928 ), vec3<f32>( 0.20155625378383743, 0.16513219449147767, 0.11697066774917994 ), vec3<f32>( 0.018500220124016652, 0.018500220124016652, 0.019382360952473074 ) )[ u32( min( floor( ( fract( ( sin( ( ( nodeVarying8 + 47.0 ) * 12.9898 ) ) * 43758.5453 ) ) * 5.0 ) ), 4.0 ) ) ];

					} else {


						if ( ( nodeVarying3 == 4.0 ) ) {

							let nodeConst16 = ( fract( ( sin( ( ( nodeVarying8 + 5.0 ) * 12.9898 ) ) * 43758.5453 ) ) > 0.4 );

							if ( nodeConst16 ) {

								nodeVar8 = mix( vec3<f32>( 0.018500220124016652, 0.024157632443547246, 0.031896033067374104 ), vec3<f32>( 0.5775804404214573, 0.5457244613615395, 0.4793201830913402 ), step( 0.55, fract( ( sin( ( ( nodeVarying8 + 31.0 ) * 12.9898 ) ) * 43758.5453 ) ) ) );

							} else {

								nodeVar8 = mix( vec3<f32>( 0.014443843592229466, 0.012983032338510335, 0.011612245176281512 ), vec3<f32>( 0.06847816983662762, 0.035601314869097636, 0.01764195448412081 ), fract( ( sin( ( ( nodeVarying8 + 23.0 ) * 12.9898 ) ) * 43758.5453 ) ) );

							}


							if ( nodeConst16 ) {

								nodeVar9 = vec3<f32>( 0.6653872982754769, 0.6375968739867731, 0.5775804404214573 );

							} else {

								nodeVar9 = vec3<f32>( 0.00972121731707524, 0.009134058699157796, 0.008023192982520563 );

							}

							let nodeConst17 = ( nodeVarying9.y * 110.0 );
							nodeVar7 = mix( mix( nodeVar8, nodeVar9, smoothstep( -0.064, -0.074, nodeVarying9.x ) ), vec3<f32>( 0.407240211891531, 0.3915724777393922, 0.34670405634441115 ), ( ( ( ( smoothstep( 0.68, 0.78, fract( nodeConst17 ) ) * clamp( ( 1.0 - fwidth( nodeConst17 ) ), 0.0, 1.0 ) ) * smoothstep( -0.015, 0.005, nodeVarying9.x ) ) * smoothstep( -0.01, 0.015, nodeVarying9.y ) ) * 0.45 ) );

						} else {

							let nodeConst18 = ( nodeVarying3 == 5.0 );

							if ( nodeConst18 ) {

								let nodeConst19 = min( nodeVarying9, ( vec2<f32>( 1.0 ) - nodeVarying9 ) );

								if ( ( abs( nodeVarying11.x ) > 0.8 ) ) {

									nodeVar11 = 1.0;

								} else {

									nodeVar11 = 0.0;

								}

								nodeVar10 = mix( ( vec3<f32>( 0.06662593863608139, 0.038204371589236, 0.023153366173251363 ) * vec3<f32>( ( 1.0 - ( max( smoothstep( 0.045, 0.02, min( nodeConst19.x, nodeConst19.y ) ), smoothstep( 0.02, 0.008, abs( ( nodeVarying9.y - 0.72 ) ) ) ) * 0.35 ) ) ) ), vec3<f32>( 0.2746773120495699, 0.24620132669705552, 0.1878207722902346 ), ( ( smoothstep( 0.06, 0.045, abs( ( nodeVarying9.x - 0.5 ) ) ) * smoothstep( 0.07, 0.05, abs( ( nodeVarying9.y - 0.72 ) ) ) ) * nodeVar11 ) );

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

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform5 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform6;

	if ( nodeConst1 ) {

		nodeVar12 = 0.6;

	} else {

		let nodeConst2 = ( nodeVarying3 == 1.0 );

		if ( nodeConst2 ) {

			let nodeConst7 = fract( ( sin( ( ( nodeVarying8 + 13.0 ) * 12.9898 ) ) * 43758.5453 ) );
			let nodeConst3 = ( nodeVarying9.y * 6.283185307179586 );
			let nodeConst4 = vec3<f32>( ( sin( nodeConst3 ) * 0.09 ), nodeVarying9.x, ( cos( nodeConst3 ) * 0.09 ) );
			let nodeConst8 = ( mix( mix( 1.55, 1.61, nodeConst7 ), mix( 1.67, 1.71, nodeConst7 ), smoothstep( -0.025, 0.07, nodeConst4.z ) ) + ( ( nodeConst4.x * ( nodeConst7 - 0.5 ) ) * 0.35 ) );
			let nodeConst9 = smoothstep( ( nodeConst8 - 0.003 ), ( nodeConst8 + 0.003 ), nodeConst4.y );
			nodeVar13 = mix( 0.6, 0.85, nodeConst9 );

		} else {

			let nodeConst18 = ( nodeVarying3 == 5.0 );

			if ( ( nodeConst18 || ( nodeVarying3 == 7.0 ) ) ) {

				nodeVar14 = 0.55;

			} else {

				nodeVar14 = 0.85;

			}

			nodeVar13 = nodeVar14;

		}

		nodeVar12 = nodeVar13;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst20 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar12, 0.045 ) + max( max( nodeConst20.x, nodeConst20.y ), nodeConst20.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform6 ) ) );
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar15 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar15;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst21 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst21 );
	let nodeConst22 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst23 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst21 * nodeConst22 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst23 ) * nodeConst22 ) ) ) * vec3<f32>( nodeConst23 ) ) );
	let nodeConst24 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform14 - render.nodeUniform15 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst25 = ( render.nodeUniform17 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform18 ) ) ), 1.0 ) );
	let nodeConst26 = ( nodeConst25.xyz / vec3<f32>( nodeConst25.w ) );
	let nodeConst27 = vec3<f32>( nodeConst26.x, ( 1.0 - nodeConst26.y ), ( nodeConst26.z + render.nodeUniform19 ) );

	if ( ( ( ( ( ( nodeConst27.x >= 0.0 ) && ( nodeConst27.x <= 1.0 ) ) && ( nodeConst27.y >= 0.0 ) ) && ( nodeConst27.y <= 1.0 ) ) && ( nodeConst27.z <= 1.0 ) ) ) {

		let nodeConst28 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst29 = ( vec2<f32>( render.nodeUniform21 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform22 ) );
		let nodeConst30 = ( nodeConst27.xy + ( fn25( 0, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar17 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst30, nodeConst27.z );
		let nodeConst31 = ( nodeConst27.xy + ( fn25( 1, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar18 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst31, nodeConst27.z );
		let nodeConst32 = ( nodeConst27.xy + ( fn25( 2, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar19 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst32, nodeConst27.z );
		let nodeConst33 = ( nodeConst27.xy + ( fn25( 3, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar20 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst33, nodeConst27.z );
		let nodeConst34 = ( nodeConst27.xy + ( fn25( 4, 5, nodeConst28 ) * nodeConst29 ) );
		nodeVar21 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst34, nodeConst27.z );
		nodeVar16 = ( ( ( ( ( nodeVar17 + nodeVar18 ) + nodeVar19 ) + nodeVar20 ) + nodeVar21 ) * 0.2 );

	} else {

		nodeVar16 = 1.0;

	}

	nodeVar22 = mix( 1.0, nodeVar16, render.nodeUniform23 );
	nodeVar23 = ( vec3<f32>( clamp( dot( normalView, nodeConst24 ), 0.0, 1.0 ) ) * ( render.nodeUniform16 * vec3<f32>( nodeVar22 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst35 = clamp( dot( positionViewDirection, normalize( ( nodeConst24 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst36 = exp2( ( ( ( nodeConst35 * -5.55473 ) - 6.98316 ) * nodeConst35 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar23 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst36 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst36 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst37 = normalize( ( nodeConst24 + positionViewDirection ) );
	let nodeConst38 = clamp( dot( positionViewDirection, nodeConst37 ), 0.0, 1.0 );
	let nodeConst39 = exp2( ( ( ( nodeConst38 * -5.55473 ) - 6.98316 ) * nodeConst38 ) );
	let nodeConst40 = max( Roughness, 0.045 );
	let nodeConst41 = ( nodeConst40 * nodeConst40 );
	directSpecular = ( directSpecular + ( ( nodeVar23 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst39 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst39 ) ) ) * vec3<f32>( fn110( nodeConst41, clamp( dot( normalView, nodeConst24 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst41, clamp( dot( normalView, nodeConst37 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst42 = ( ( object.nodeUniform24 - object.nodeUniform25 ) / object.nodeUniform26 );
	let nodeConst43 = ( v_positionWorld + ( ( normalWorld * nodeConst42 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst44 = ( nodeConst42 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst43 >= ( object.nodeUniform25 - nodeConst44 ) ) ) && all( ( nodeConst43 <= ( object.nodeUniform24 + nodeConst44 ) ) ) ) && ( render.nodeUniform27 >= lightProbePriority ) ) ) {

		let nodeConst45 = ( object.nodeUniform24 - object.nodeUniform25 );
		let nodeConst46 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform26 );
		let nodeConst47 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst45 / object.nodeUniform26 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform25 ) / nodeConst45 ), nodeConst46, ( vec3<f32>( 1.0 ) - nodeConst46 ) );
		let nodeConst48 = ( ( nodeConst47.z * object.nodeUniform26.z ) + 1.0 );
		let nodeConst49 = ( object.nodeUniform26.z + 2.0 );
		let nodeConst50 = ( nodeConst49 * 7.0 );
		nodeVar24 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst47.xy, ( ( nodeConst48 + ( nodeConst49 * 0.0 ) ) / nodeConst50 ) ) );
		nodeVar25 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst47.xy, ( ( nodeConst48 + ( nodeConst49 * 1.0 ) ) / nodeConst50 ) ) );
		nodeVar26 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst47.xy, ( ( nodeConst48 + ( nodeConst49 * 2.0 ) ) / nodeConst50 ) ) );
		nodeVar27 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst47.xy, ( ( nodeConst48 + ( nodeConst49 * 3.0 ) ) / nodeConst50 ) ) );
		nodeVar28 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst47.xy, ( ( nodeConst48 + ( nodeConst49 * 4.0 ) ) / nodeConst50 ) ) );
		nodeVar29 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst47.xy, ( ( nodeConst48 + ( nodeConst49 * 5.0 ) ) / nodeConst50 ) ) );
		nodeVar30 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst47.xy, ( ( nodeConst48 + ( nodeConst49 * 6.0 ) ) / nodeConst50 ) ) );
		let nodeConst51 = array< vec3<f32>, 9 >( nodeVar24.xyz, vec3<f32>( nodeVar24.w, nodeVar25.xy ), vec3<f32>( nodeVar25.zw, nodeVar26.x ), nodeVar26.yzw, nodeVar27.xyz, vec3<f32>( nodeVar27.w, nodeVar28.xy ), vec3<f32>( nodeVar28.zw, nodeVar29.x ), nodeVar29.yzw, nodeVar30.xyz );
		let nodeConst52 = ( max( ( ( ( ( ( ( ( ( ( nodeConst51[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst51[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst51[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst51[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst51[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst51[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst51[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst51[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst51[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform29 ) );
		lightProbeIrradiance = nodeConst52;
		lightProbePriority = render.nodeUniform27;
		

	}

	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst53 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst54 = ( object.nodeUniform31 * vec4<f32>( nodeConst53, 1.0 ) );
	let nodeConst55 = clamp( Roughness, 0.0, 1.0 );
	nodeVar31 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, vec3<f32>( ( - nodeConst54.x ), nodeConst54.yz ), ( ( object.nodeUniform33 * nodeConst55 ) * ( 2.0 - nodeConst55 ) ) );
	radiance = ( radiance + ( nodeVar31.xyz * vec3<f32>( object.nodeUniform34 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst56 = ( object.nodeUniform31 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst57 = clamp( 1.0, 0.0, 1.0 );
	nodeVar32 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, vec3<f32>( ( - nodeConst56.x ), nodeConst56.yz ), ( ( object.nodeUniform33 * nodeConst57 ) * ( 2.0 - nodeConst57 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar32.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform34 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar33 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar33 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst58 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst58 );
	let nodeConst59 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst60 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst58 * nodeConst59 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst60 ) * nodeConst59 ) ) ) * vec3<f32>( nodeConst60 ) ) );
	let nodeConst61 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar34 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst61 ) );
	nodeVar35 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst61 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar34 );
	indirectDiffuse = ( indirectDiffuse + nodeVar35 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst62 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst62;

	// result

	output.color = nodeConst62;

	return output;

}
