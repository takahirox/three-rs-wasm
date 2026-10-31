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
@binding( 3 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform5 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform28_sampler : sampler_comparison;
@binding( 12 ) @group( 1 ) var nodeUniform28 : texture_depth_2d;
@binding( 13 ) @group( 1 ) var nodeUniform33_sampler : sampler;
@binding( 14 ) @group( 1 ) var nodeUniform33 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform19 : mat3x3<f32>,
	nodeUniform20 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform25 : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : vec2<f32>,
	nodeUniform31 : f32,
	nodeUniform6 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar1 : f32;
var<private> Metalness : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
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
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_bitangentView : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 6 ) nodeVarying10 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32>,
	@builtin( front_facing ) isFront : bool ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );

	if ( ( DiffuseColor.w <= object.nodeUniform4 ) ) {

		discard;
		

	}

	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform5, nodeUniform5_sampler, ( fragCoord.xy / render.nodeUniform6 ) ).x;
	let nodeConst0 = max( nodeVar1, 0.001 );
	AmbientOcclusion = nodeConst0;
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform11 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst2 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform10 * nodeVar3.y ), 0.045 ) + max( max( nodeConst2.x, nodeConst2.y ), nodeConst2.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	EmissiveColor = ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) );
	let nodeConst3 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_tangentView = ( normalize( v_tangentView ) * vec3<f32>( nodeConst3 ) );
	NORMAL_bitangentView = ( normalize( NORMAL_v_bitangentView ) * vec3<f32>( nodeConst3 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeConst3 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar4 = textureSample( nodeUniform18, nodeUniform18_sampler, ( object.nodeUniform19 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	let nodeConst5 = ( ( nodeVar4 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeConst5.xy * object.nodeUniform20 ), nodeConst5.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar5 = textureSample( nodeUniform16, nodeUniform16_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar5;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst6 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst6 );
	let nodeConst7 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst8 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst6 * nodeConst7 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst8 ) * nodeConst7 ) ) ) * vec3<f32>( nodeConst8 ) ) );
	let nodeConst9 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform22 - render.nodeUniform23 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst10 = ( render.nodeUniform25 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform26 ) ) ), 1.0 ) );
	let nodeConst11 = ( nodeConst10.xyz / vec3<f32>( nodeConst10.w ) );
	let nodeConst12 = vec3<f32>( nodeConst11.x, ( 1.0 - nodeConst11.y ), ( nodeConst11.z + render.nodeUniform27 ) );

	if ( ( ( ( ( ( nodeConst12.x >= 0.0 ) && ( nodeConst12.x <= 1.0 ) ) && ( nodeConst12.y >= 0.0 ) ) && ( nodeConst12.y <= 1.0 ) ) && ( nodeConst12.z <= 1.0 ) ) ) {

		let nodeConst13 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst14 = ( vec2<f32>( render.nodeUniform29 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform30 ) );
		let nodeConst15 = ( nodeConst12.xy + ( fn25( 0, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar7 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeConst15, nodeConst12.z );
		let nodeConst16 = ( nodeConst12.xy + ( fn25( 1, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar8 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeConst16, nodeConst12.z );
		let nodeConst17 = ( nodeConst12.xy + ( fn25( 2, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar9 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeConst17, nodeConst12.z );
		let nodeConst18 = ( nodeConst12.xy + ( fn25( 3, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar10 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeConst18, nodeConst12.z );
		let nodeConst19 = ( nodeConst12.xy + ( fn25( 4, 5, nodeConst13 ) * nodeConst14 ) );
		nodeVar11 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, nodeConst19, nodeConst12.z );
		nodeVar6 = ( ( ( ( ( nodeVar7 + nodeVar8 ) + nodeVar9 ) + nodeVar10 ) + nodeVar11 ) * 0.2 );

	} else {

		nodeVar6 = 1.0;

	}

	nodeVar12 = mix( 1.0, nodeVar6, render.nodeUniform31 );
	nodeVar13 = ( vec3<f32>( clamp( dot( normalView, nodeConst9 ), 0.0, 1.0 ) ) * ( render.nodeUniform24 * vec3<f32>( nodeVar12 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst20 = clamp( dot( positionViewDirection, normalize( ( nodeConst9 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst21 = exp2( ( ( ( nodeConst20 * -5.55473 ) - 6.98316 ) * nodeConst20 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar13 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst21 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst21 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst22 = normalize( ( nodeConst9 + positionViewDirection ) );
	let nodeConst23 = clamp( dot( positionViewDirection, nodeConst22 ), 0.0, 1.0 );
	let nodeConst24 = exp2( ( ( ( nodeConst23 * -5.55473 ) - 6.98316 ) * nodeConst23 ) );
	let nodeConst25 = max( Roughness, 0.045 );
	let nodeConst26 = ( nodeConst25 * nodeConst25 );
	directSpecular = ( directSpecular + ( ( nodeVar13 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst24 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst24 ) ) ) * vec3<f32>( fn110( nodeConst26, clamp( dot( normalView, nodeConst9 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst26, clamp( dot( normalView, nodeConst22 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform32 );
	ambientOcclusion = 1.0;
	ambientOcclusion = ( ambientOcclusion * AmbientOcclusion );
	nodeVar14 = textureSample( nodeUniform33, nodeUniform33_sampler, ( fragCoord.xy / render.nodeUniform6 ) );
	irradiance = ( irradiance + ( nodeVar14.xyz / vec3<f32>( nodeConst0 ) ) );
	nodeVar15 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar15 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst27 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst27 );
	let nodeConst28 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst29 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst27 * nodeConst28 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst29 ) * nodeConst28 ) ) ) * vec3<f32>( nodeConst29 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst30 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar16 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst30 ) );
	nodeVar17 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst30 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar16 );
	indirectDiffuse = ( indirectDiffuse + nodeVar17 );
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst31 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst31;

	// result

	output.color = nodeConst31;

	return output;

}
