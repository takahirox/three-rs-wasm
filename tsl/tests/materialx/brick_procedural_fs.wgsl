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
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 12 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 13 ) @group( 1 ) var nodeUniform23_sampler : sampler;
@binding( 14 ) @group( 1 ) var nodeUniform23 : texture_2d<f32>;
@binding( 15 ) @group( 1 ) var nodeUniform25_sampler : sampler;
@binding( 16 ) @group( 1 ) var nodeUniform25 : texture_cube<f32>;

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : f32,
	nodeUniform20 : mat3x3<f32>,
	nodeUniform22 : mat4x4<f32>,
	nodeUniform24 : mat3x3<f32>,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar4 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_tangentWorld : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> NORMAL_bitangentWorld : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_normalWorld : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn104 ( c : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;

	nodeVar0 = c;
	nodeVar1 = nodeVar0.x;
	nodeVar2 = nodeVar0.y;
	nodeVar3 = nodeVar0.z;
	nodeVar4 = min( nodeVar1, min( nodeVar2, nodeVar3 ) );
	nodeVar5 = max( nodeVar1, max( nodeVar2, nodeVar3 ) );
	nodeVar6 = ( nodeVar5 - nodeVar4 );
	nodeVar7 = 0.0;
	nodeVar8 = 0.0;
	nodeVar9 = 0.0;
	nodeVar9 = nodeVar5;

	if ( ( nodeVar5 > 0.0 ) ) {

		nodeVar8 = ( nodeVar6 / nodeVar5 );
		

	} else {

		nodeVar8 = 0.0;
		

	}


	if ( ( nodeVar8 <= 0.0 ) ) {

		nodeVar7 = 0.0;
		

	} else {


		if ( ( nodeVar1 >= nodeVar5 ) ) {

			nodeVar7 = ( ( nodeVar2 - nodeVar3 ) / nodeVar6 );
			

		} else {


			if ( ( nodeVar2 >= nodeVar5 ) ) {

				nodeVar7 = ( 2.0 + ( ( nodeVar3 - nodeVar1 ) / nodeVar6 ) );
				

			} else {

				nodeVar7 = ( 4.0 + ( ( nodeVar1 - nodeVar2 ) / nodeVar6 ) );
				

			}

			

		}

		nodeVar7 = ( nodeVar7 * 0.16666666666666666 );

		if ( ( nodeVar7 < 0.0 ) ) {

			nodeVar7 = ( nodeVar7 + 1.0 );
			

		}

		

	}


	return vec3<f32>( nodeVar7, nodeVar8, nodeVar9 );

}


fn fn103 ( hsv : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;

	nodeVar0 = vec3<f32>( 0.0, 0.0, 0.0 );

	if ( ( hsv.y < 0.0001 ) ) {

		nodeVar0 = vec3<f32>( hsv.z, hsv.z, hsv.z );
		

	} else {

		nodeVar1 = ( ( hsv.x - floor( hsv.x ) ) * 6.0 );
		let nodeConst0 = i32( trunc( nodeVar1 ) );

		if ( ( nodeConst0 == 0 ) ) {

			let nodeConst1 = ( nodeVar1 - f32( nodeConst0 ) );
			let nodeConst2 = ( hsv.z * ( 1.0 - ( hsv.y * ( 1.0 - nodeConst1 ) ) ) );
			let nodeConst3 = ( hsv.z * ( 1.0 - hsv.y ) );
			nodeVar0 = vec3<f32>( hsv.z, nodeConst2, nodeConst3 );
			

		} else {


			if ( ( nodeConst0 == 1 ) ) {

				let nodeConst1 = ( nodeVar1 - f32( nodeConst0 ) );
				let nodeConst4 = ( hsv.z * ( 1.0 - ( hsv.y * nodeConst1 ) ) );
				let nodeConst3 = ( hsv.z * ( 1.0 - hsv.y ) );
				nodeVar0 = vec3<f32>( nodeConst4, hsv.z, nodeConst3 );
				

			} else {


				if ( ( nodeConst0 == 2 ) ) {

					let nodeConst3 = ( hsv.z * ( 1.0 - hsv.y ) );
					let nodeConst1 = ( nodeVar1 - f32( nodeConst0 ) );
					let nodeConst2 = ( hsv.z * ( 1.0 - ( hsv.y * ( 1.0 - nodeConst1 ) ) ) );
					nodeVar0 = vec3<f32>( nodeConst3, hsv.z, nodeConst2 );
					

				} else {


					if ( ( nodeConst0 == 3 ) ) {

						let nodeConst3 = ( hsv.z * ( 1.0 - hsv.y ) );
						let nodeConst1 = ( nodeVar1 - f32( nodeConst0 ) );
						let nodeConst4 = ( hsv.z * ( 1.0 - ( hsv.y * nodeConst1 ) ) );
						nodeVar0 = vec3<f32>( nodeConst3, nodeConst4, hsv.z );
						

					} else {


						if ( ( nodeConst0 == 4 ) ) {

							let nodeConst1 = ( nodeVar1 - f32( nodeConst0 ) );
							let nodeConst2 = ( hsv.z * ( 1.0 - ( hsv.y * ( 1.0 - nodeConst1 ) ) ) );
							let nodeConst3 = ( hsv.z * ( 1.0 - hsv.y ) );
							nodeVar0 = vec3<f32>( nodeConst2, nodeConst3, hsv.z );
							

						} else {

							let nodeConst3 = ( hsv.z * ( 1.0 - hsv.y ) );
							let nodeConst1 = ( nodeVar1 - f32( nodeConst0 ) );
							let nodeConst4 = ( hsv.z * ( 1.0 - ( hsv.y * nodeConst1 ) ) );
							nodeVar0 = vec3<f32>( hsv.z, nodeConst3, nodeConst4 );
							

						}

						

					}

					

				}

				

			}

			

		}

		

	}


	return nodeVar0;

}




@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_tangentWorld : vec3<f32>,
	@location( 2 ) v_bitangentWorld : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) NORMAL_v_tangentWorld : vec3<f32>,
	@location( 5 ) NORMAL_v_bitangentWorld : vec3<f32>,
	@location( 6 ) nodeVarying12 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = 3.0;
	let nodeConst1 = vec2<f32>( nodeConst0, nodeConst0 );
	let nodeConst2 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * nodeConst1 ) + vec2<f32>( 0.0, 0.0 ) );
	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( vec2<f32>( nodeConst2[ 0u ], ( 1.0 - nodeConst2[ 1u ] ) ), 1.0 ) ).xy );
	let nodeConst3 = nodeVar0.x;
	let nodeConst4 = 0.083;
	let nodeConst5 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * nodeConst1 ) + vec2<f32>( 0.0, 0.0 ) );
	nodeVar1 = textureSample( nodeUniform2, nodeUniform2_sampler, ( object.nodeUniform3 * vec3<f32>( vec2<f32>( nodeConst5[ 0u ], ( 1.0 - nodeConst5[ 1u ] ) ), 1.0 ) ).xy );
	let nodeConst6 = nodeVar1.x;
	let nodeConst7 = ( ( nodeConst4 * nodeConst6 ) + nodeConst3 );
	let nodeConst8 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * nodeConst1 ) + vec2<f32>( 0.0, 0.0 ) );
	nodeVar2 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( vec2<f32>( nodeConst8[ 0u ], ( 1.0 - nodeConst8[ 1u ] ) ), 1.0 ) ).xy );
	let nodeConst9 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * nodeConst1 ) + vec2<f32>( 0.0, 0.0 ) );
	nodeVar3 = textureSample( nodeUniform6, nodeUniform6_sampler, ( object.nodeUniform7 * vec3<f32>( vec2<f32>( nodeConst9[ 0u ], ( 1.0 - nodeConst9[ 1u ] ) ), 1.0 ) ).xy );
	let nodeConst10 = nodeVar3.x;
	DiffuseColor = vec4<f32>( ( vec3<f32>( 1.0 ) * min( max( mix( ( vec3<f32>( 0.263273, 0.263273, 0.263273 ) * vec3<f32>( nodeConst3 ) ), ( mix( fn103( ( vec3<f32>( ( ( nodeConst7 - 0.35 ) * nodeConst4 ), 0.0, ( nodeConst7 * ( 0.787 * nodeConst6 ) ) ) + fn104( vec3<f32>( 0.661876, 0.19088, 0.0 ) ) ) ), vec3<f32>( 0.56372, 0.56372, 0.56372 ), ( 0.248 * nodeVar2.x ) ) * vec3<f32>( nodeConst3 ) ), nodeConst10 ), vec3<f32>( 0.0, 0.0, 0.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform8 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform9;
	let nodeConst11 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * nodeConst1 ) + vec2<f32>( 0.0, 0.0 ) );
	nodeVar4 = textureSample( nodeUniform10, nodeUniform10_sampler, ( object.nodeUniform11 * vec3<f32>( vec2<f32>( nodeConst11[ 0u ], ( 1.0 - nodeConst11[ 1u ] ) ), 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst13 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( 0.853 / max( nodeConst10, 0.00001 ) ) * nodeVar4.x ), 0.045 ) + max( max( nodeConst13.x, nodeConst13.y ), nodeConst13.z ) ), 1.0 );
	IOR = object.nodeUniform14;
	let nodeConst14 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst14 * nodeConst14 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform15 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform15, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform9 ) ) );
	EmissiveColor = ( object.nodeUniform16 * vec3<f32>( object.nodeUniform17 ) );
	NORMAL_tangentWorld = normalize( NORMAL_v_tangentWorld );
	let nodeConst16 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * nodeConst1 ) + vec2<f32>( 0.0, 0.0 ) );
	nodeVar5 = textureSample( nodeUniform23, nodeUniform23_sampler, ( object.nodeUniform24 * vec3<f32>( vec2<f32>( nodeConst16[ 0u ], ( 1.0 - nodeConst16[ 1u ] ) ), 1.0 ) ).xy );
	let nodeConst17 = nodeVar5.xyz;
	let nodeConst18 = vec3<f32>( nodeConst17[ 0u ], nodeConst17[ 1u ], nodeConst17[ 2u ] );
	let nodeConst19 = mix( ( ( nodeConst18 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 0.0, 0.0, 1.0 ), f32( ( dot( nodeConst18, nodeConst18 ) == 0.0 ) ) );
	let nodeConst20 = vec2<f32>( 1.0 );
	NORMAL_bitangentWorld = normalize( NORMAL_v_bitangentWorld );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_normalWorld = normalize( ( vec4<f32>( NORMAL_normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	normalView = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform20 * normalize( ( ( ( NORMAL_tangentWorld * vec3<f32>( ( nodeConst19[ 0u ] * nodeConst20[ 0u ] ) ) ) + ( NORMAL_bitangentWorld * vec3<f32>( ( nodeConst19[ 1u ] * nodeConst20[ 1u ] ) ) ) ) + ( NORMAL_normalWorld * vec3<f32>( nodeConst19[ 2u ] ) ) ) ) ), 0.0 ) ).xyz );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = textureSample( nodeUniform18, nodeUniform18_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar6;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst21 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst21 );
	let nodeConst22 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst23 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst21 * nodeConst22 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst23 ) * nodeConst22 ) ) ) * vec3<f32>( nodeConst23 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst24 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst25 = ( object.nodeUniform26 * vec4<f32>( nodeConst24, 1.0 ) );
	let nodeConst26 = clamp( Roughness, 0.0, 1.0 );
	nodeVar7 = textureSampleLevel( nodeUniform25, nodeUniform25_sampler, vec3<f32>( ( - nodeConst25.x ), nodeConst25.yz ), ( ( object.nodeUniform28 * nodeConst26 ) * ( 2.0 - nodeConst26 ) ) );
	radiance = ( radiance + ( nodeVar7.xyz * vec3<f32>( object.nodeUniform29 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst27 = ( object.nodeUniform26 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst28 = clamp( 1.0, 0.0, 1.0 );
	nodeVar8 = textureSampleLevel( nodeUniform25, nodeUniform25_sampler, vec3<f32>( ( - nodeConst27.x ), nodeConst27.yz ), ( ( object.nodeUniform28 * nodeConst28 ) * ( 2.0 - nodeConst28 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar8.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform29 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar9 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst29 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst29 );
	let nodeConst30 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst31 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst29 * nodeConst30 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst31 ) * nodeConst30 ) ) ) * vec3<f32>( nodeConst31 ) ) );
	let nodeConst32 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar10 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst32 ) );
	nodeVar11 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst32 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar10 );
	indirectDiffuse = ( indirectDiffuse + nodeVar11 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst33 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst33;

	// result

	output.color = nodeConst33;

	return output;

}
