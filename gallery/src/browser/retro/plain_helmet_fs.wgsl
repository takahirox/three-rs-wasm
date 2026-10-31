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
@binding( 3 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 12 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;
@binding( 13 ) @group( 1 ) var nodeUniform32_sampler : sampler;
@binding( 14 ) @group( 1 ) var nodeUniform32 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform21 : mat3x3<f32>,
	nodeUniform22 : vec2<f32>,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar1 : vec4<f32>;
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
var<private> nodeVar4 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn110 ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn fn112 ( alpha : f32, dotNH : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );
	let nodeConst1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeConst0 ) ) );

	return ( ( nodeConst0 / ( nodeConst1 * nodeConst1 ) ) * 0.3183098861837907 );

}


fn fn44 ( lightDistance : f32, cutoffDistance : f32, decayExponent : f32 ) -> f32 {

	var nodeVar0 : f32;


	if ( ( cutoffDistance > 0.0 ) ) {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		let nodeConst1 = ( lightDistance / cutoffDistance );
		let nodeConst2 = clamp( ( 1.0 - ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * nodeConst1 ) ), 0.0, 1.0 );
		nodeVar0 = ( nodeConst0 * ( nodeConst2 * nodeConst2 ) );

	} else {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		nodeVar0 = nodeConst0;

	}


	return nodeVar0;

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	AmbientOcclusion = ( ( ( nodeVar1.x - 1.0 ) * object.nodeUniform6 ) + 1.0 );
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar2.z );
	nodeVar3 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform11 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform10 * nodeVar3.y ), 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar2.z ) ) ) );
	nodeVar4 = textureSample( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	EmissiveColor = ( vec4<f32>( ( object.nodeUniform14 * vec3<f32>( object.nodeUniform15 ) ), 1.0 ) * nodeVar4 ).xyz;
	NORMAL_normalView = normalViewGeometry;
	let nodeConst1 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	let nodeConst2 = dpdx( nodeVarying6 );
	let nodeConst3 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	let nodeConst4 = - dpdy( nodeVarying6 );
	let nodeConst5 = ( ( nodeConst1 * vec3<f32>( nodeConst2.x ) ) + ( nodeConst3 * vec3<f32>( nodeConst4.x ) ) );
	let nodeConst6 = ( ( nodeConst1 * vec3<f32>( nodeConst2.y ) ) + ( nodeConst3 * vec3<f32>( nodeConst4.y ) ) );
	let nodeConst7 = max( dot( nodeConst5, nodeConst5 ), dot( nodeConst6, nodeConst6 ) );

	if ( ( nodeConst7 == 0.0 ) ) {

		nodeVar5 = 0.0;

	} else {

		nodeVar5 = inverseSqrt( nodeConst7 );

	}

	tangentViewFrame = ( nodeConst5 * vec3<f32>( nodeVar5 ) );
	NORMAL_tangentView = tangentViewFrame;
	bitangentViewFrame = ( nodeConst6 * vec3<f32>( nodeVar5 ) );
	NORMAL_bitangentView = bitangentViewFrame;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar6 = textureSample( nodeUniform20, nodeUniform20_sampler, ( object.nodeUniform21 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	let nodeConst8 = ( ( nodeVar6 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeConst8.xy * object.nodeUniform22 ), nodeConst8.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar7 = textureSample( nodeUniform18, nodeUniform18_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar7;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst9 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst9 );
	let nodeConst10 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst11 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst9 * nodeConst10 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst11 ) * nodeConst10 ) ) ) * vec3<f32>( nodeConst11 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform23 );
	let nodeConst12 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform25 - render.nodeUniform26 ), 0.0 ) ).xyz );
	nodeVar8 = ( vec3<f32>( clamp( dot( normalView, nodeConst12 ), 0.0, 1.0 ) ) * render.nodeUniform27 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst13 = clamp( dot( positionViewDirection, normalize( ( nodeConst12 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst14 = exp2( ( ( ( nodeConst13 * -5.55473 ) - 6.98316 ) * nodeConst13 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar8 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst14 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst14 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst15 = normalize( ( nodeConst12 + positionViewDirection ) );
	let nodeConst16 = clamp( dot( positionViewDirection, nodeConst15 ), 0.0, 1.0 );
	let nodeConst17 = exp2( ( ( ( nodeConst16 * -5.55473 ) - 6.98316 ) * nodeConst16 ) );
	let nodeConst18 = max( Roughness, 0.045 );
	let nodeConst19 = ( nodeConst18 * nodeConst18 );
	directSpecular = ( directSpecular + ( ( nodeVar8 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst17 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst17 ) ) ) * vec3<f32>( fn110( nodeConst19, clamp( dot( normalView, nodeConst12 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst19, clamp( dot( normalView, nodeConst15 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst20 = ( render.nodeUniform28 - v_positionView );
	let nodeConst21 = normalize( nodeConst20 );
	nodeVar9 = ( vec3<f32>( clamp( dot( normalView, nodeConst21 ), 0.0, 1.0 ) ) * ( render.nodeUniform29 * vec3<f32>( fn44( length( nodeConst20 ), render.nodeUniform30, render.nodeUniform31 ) ) ) );
	let nodeConst22 = clamp( dot( positionViewDirection, normalize( ( nodeConst21 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst23 = exp2( ( ( ( nodeConst22 * -5.55473 ) - 6.98316 ) * nodeConst22 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar9 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst23 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst23 ) ) ) ) ) );
	let nodeConst24 = normalize( ( nodeConst21 + positionViewDirection ) );
	let nodeConst25 = clamp( dot( positionViewDirection, nodeConst24 ), 0.0, 1.0 );
	let nodeConst26 = exp2( ( ( ( nodeConst25 * -5.55473 ) - 6.98316 ) * nodeConst25 ) );
	let nodeConst27 = max( Roughness, 0.045 );
	let nodeConst28 = ( nodeConst27 * nodeConst27 );
	directSpecular = ( directSpecular + ( ( nodeVar9 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst26 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst26 ) ) ) * vec3<f32>( fn110( nodeConst28, clamp( dot( normalView, nodeConst21 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst28, clamp( dot( normalView, nodeConst24 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst29 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst30 = ( object.nodeUniform33 * vec4<f32>( nodeConst29, 1.0 ) );
	let nodeConst31 = clamp( Roughness, 0.0, 1.0 );
	nodeVar10 = textureSampleLevel( nodeUniform32, nodeUniform32_sampler, vec3<f32>( ( - nodeConst30.x ), nodeConst30.yz ), ( ( object.nodeUniform35 * nodeConst31 ) * ( 2.0 - nodeConst31 ) ) );
	radiance = ( radiance + ( nodeVar10.xyz * vec3<f32>( object.nodeUniform36 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst32 = ( object.nodeUniform33 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst33 = clamp( 1.0, 0.0, 1.0 );
	nodeVar11 = textureSampleLevel( nodeUniform32, nodeUniform32_sampler, vec3<f32>( ( - nodeConst32.x ), nodeConst32.yz ), ( ( object.nodeUniform35 * nodeConst33 ) * ( 2.0 - nodeConst33 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar11.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform36 ) ) );
	ambientOcclusion = 1.0;
	ambientOcclusion = ( ambientOcclusion * AmbientOcclusion );
	nodeVar12 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar12 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst34 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst34 );
	let nodeConst35 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst36 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst34 * nodeConst35 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst36 ) * nodeConst35 ) ) ) * vec3<f32>( nodeConst36 ) ) );
	let nodeConst37 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar13 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst37 ) );
	nodeVar14 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst37 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar13 );
	indirectDiffuse = ( indirectDiffuse + nodeVar14 );
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst38 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst38;

	// result

	output.color = nodeConst38;

	return output;

}
