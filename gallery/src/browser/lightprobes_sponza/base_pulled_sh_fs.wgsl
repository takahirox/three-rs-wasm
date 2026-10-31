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
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform23_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform23 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform6 : u32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform12 : u32,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : vec2<f32>,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform20 : vec4<f32>,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform26 : vec4<f32>,
	nodeUniform19 : f32,
	nodeUniform22 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
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
var<private> nodeVar1 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : f32;
var<private> shadowValue : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn45 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn46 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


fn fn12 ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn fn14 ( alpha : f32, dotNH : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );
	let nodeConst1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeConst0 ) ) );

	return ( ( nodeConst0 / ( nodeConst1 * nodeConst1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) nodeVarying5 : vec2<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying5, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform8;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform9, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform8 ) ) );
	EmissiveColor = ( object.nodeUniform13 * vec3<f32>( object.nodeUniform14 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform15, nodeUniform15_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	let nodeConst4 = normalize( ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform17, 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar2 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform19 ) ) ), 1.0 );
	nodeVar3 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar3 >= render.nodeUniform20.x ) && ( nodeVar3 < render.nodeUniform20.y ) ) ) {

		let nodeConst5 = ( render.nodeUniform21 * nodeVar2 );
		let nodeConst6 = ( nodeConst5.xyz / vec3<f32>( nodeConst5.w ) );
		let nodeConst7 = vec3<f32>( nodeConst6.x, ( 1.0 - nodeConst6.y ), ( nodeConst6.z + render.nodeUniform22 ) );

		if ( ( ( ( ( ( nodeConst7.x >= 0.0 ) && ( nodeConst7.x <= 1.0 ) ) && ( nodeConst7.y >= 0.0 ) ) && ( nodeConst7.y <= 1.0 ) ) && ( nodeConst7.z <= 1.0 ) ) ) {

			let nodeConst8 = ( fn45( fragCoord.xy ) * 6.28318530718 );
			let nodeConst9 = ( vec2<f32>( render.nodeUniform24 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform25 ) );
			let nodeConst10 = ( nodeConst7.xy + ( fn46( 0, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar5 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst10, nodeConst7.z );
			let nodeConst11 = ( nodeConst7.xy + ( fn46( 1, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar6 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst11, nodeConst7.z );
			let nodeConst12 = ( nodeConst7.xy + ( fn46( 2, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar7 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst12, nodeConst7.z );
			let nodeConst13 = ( nodeConst7.xy + ( fn46( 3, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar8 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst13, nodeConst7.z );
			let nodeConst14 = ( nodeConst7.xy + ( fn46( 4, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar9 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst14, nodeConst7.z );
			nodeVar4 = ( ( ( ( ( nodeVar5 + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) + nodeVar9 ) * 0.2 );

		} else {

			nodeVar4 = 1.0;

		}

		shadowValue = mix( nodeVar4, shadowValue, smoothstep( render.nodeUniform20.z, render.nodeUniform20.y, nodeVar3 ) );
		

	}


	if ( ( ( nodeVar3 >= render.nodeUniform26.x ) && ( nodeVar3 < render.nodeUniform26.y ) ) ) {

		let nodeConst15 = ( render.nodeUniform27 * nodeVar2 );
		let nodeConst16 = ( nodeConst15.xyz / vec3<f32>( nodeConst15.w ) );
		let nodeConst17 = vec3<f32>( nodeConst16.x, ( 1.0 - nodeConst16.y ), ( nodeConst16.z + render.nodeUniform28 ) );

		if ( ( ( ( ( ( nodeConst17.x >= 0.0 ) && ( nodeConst17.x <= 1.0 ) ) && ( nodeConst17.y >= 0.0 ) ) && ( nodeConst17.y <= 1.0 ) ) && ( nodeConst17.z <= 1.0 ) ) ) {

			let nodeConst18 = ( fn45( fragCoord.xy ) * 6.28318530718 );
			let nodeConst19 = ( vec2<f32>( render.nodeUniform24 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform25 ) );
			let nodeConst20 = ( nodeConst17.xy + ( fn46( 0, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar11 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst20, nodeConst17.z );
			let nodeConst21 = ( nodeConst17.xy + ( fn46( 1, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar12 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst21, nodeConst17.z );
			let nodeConst22 = ( nodeConst17.xy + ( fn46( 2, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar13 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst22, nodeConst17.z );
			let nodeConst23 = ( nodeConst17.xy + ( fn46( 3, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar14 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst23, nodeConst17.z );
			let nodeConst24 = ( nodeConst17.xy + ( fn46( 4, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar15 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst24, nodeConst17.z );
			nodeVar10 = ( ( ( ( ( nodeVar11 + nodeVar12 ) + nodeVar13 ) + nodeVar14 ) + nodeVar15 ) * 0.2 );

		} else {

			nodeVar10 = 1.0;

		}

		shadowValue = mix( nodeVar10, shadowValue, smoothstep( render.nodeUniform26.z, render.nodeUniform26.y, nodeVar3 ) );
		

	}

	nodeVar16 = mix( 1.0, shadowValue, render.nodeUniform29 );
	nodeVar17 = ( vec3<f32>( clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ) ) * ( render.nodeUniform18 * vec3<f32>( nodeVar16 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst25 = clamp( dot( positionViewDirection, normalize( ( nodeConst4 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst26 = exp2( ( ( ( nodeConst25 * -5.55473 ) - 6.98316 ) * nodeConst25 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar17 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst26 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst26 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst27 = normalize( ( nodeConst4 + positionViewDirection ) );
	let nodeConst28 = clamp( dot( positionViewDirection, nodeConst27 ), 0.0, 1.0 );
	let nodeConst29 = exp2( ( ( ( nodeConst28 * -5.55473 ) - 6.98316 ) * nodeConst28 ) );
	let nodeConst30 = max( Roughness, 0.045 );
	let nodeConst31 = ( nodeConst30 * nodeConst30 );
	directSpecular = ( directSpecular + ( ( nodeVar17 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst29 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst29 ) ) ) * vec3<f32>( fn12( nodeConst31, clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn14( nodeConst31, clamp( dot( normalView, nodeConst27 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar18 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar18 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst32 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst32 );
	let nodeConst33 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst34 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst32 * nodeConst33 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst34 ) * nodeConst33 ) ) ) * vec3<f32>( nodeConst34 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst35 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar19 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst35 ) );
	nodeVar20 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst35 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar19 );
	indirectDiffuse = ( indirectDiffuse + nodeVar20 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst36 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst36;

	// result

	output.color = nodeConst36;

	return output;

}
