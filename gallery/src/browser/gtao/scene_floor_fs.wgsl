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
@binding( 5 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform39_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform39 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : f32,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform40 : mat4x4<f32>,
	nodeUniform42 : f32,
	nodeUniform43 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform24 : vec3<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32,
	nodeUniform37 : f32,
	nodeUniform38 : f32,
	nodeUniform32 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform35 : vec3<f32>,
	nodeUniform36 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	nodeUniform5 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar1 : f32;
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
var<private> nodeVar2 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> irradiance : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying6 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying6, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform4, nodeUniform4_sampler, ( fragCoord.xy / render.nodeUniform5 ) ).x;
	AmbientOcclusion = nodeVar1;
	Metalness = object.nodeUniform6;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform7, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform6 ) ) );
	EmissiveColor = ( object.nodeUniform10 * vec3<f32>( object.nodeUniform11 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = textureSample( nodeUniform12, nodeUniform12_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	let nodeConst4 = ( render.nodeUniform14 - v_positionView );
	let nodeConst5 = normalize( nodeConst4 );
	nodeVar3 = ( vec3<f32>( clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform15 * vec3<f32>( smoothstep( render.nodeUniform16, render.nodeUniform17, dot( nodeConst5, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform19 - render.nodeUniform20 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst4 ), render.nodeUniform21, render.nodeUniform22 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst6 = clamp( dot( positionViewDirection, normalize( ( nodeConst5 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst7 = exp2( ( ( ( nodeConst6 * -5.55473 ) - 6.98316 ) * nodeConst6 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar3 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst7 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst7 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst8 = normalize( ( nodeConst5 + positionViewDirection ) );
	let nodeConst9 = clamp( dot( positionViewDirection, nodeConst8 ), 0.0, 1.0 );
	let nodeConst10 = exp2( ( ( ( nodeConst9 * -5.55473 ) - 6.98316 ) * nodeConst9 ) );
	let nodeConst11 = max( Roughness, 0.045 );
	let nodeConst12 = ( nodeConst11 * nodeConst11 );
	directSpecular = ( directSpecular + ( ( nodeVar3 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst10 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst10 ) ) ) * vec3<f32>( fn110( nodeConst12, clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst12, clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst13 = ( render.nodeUniform23 - v_positionView );
	let nodeConst14 = normalize( nodeConst13 );
	nodeVar4 = ( vec3<f32>( clamp( dot( normalView, nodeConst14 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform24 * vec3<f32>( smoothstep( render.nodeUniform25, render.nodeUniform26, dot( nodeConst14, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform27 - render.nodeUniform28 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst13 ), render.nodeUniform29, render.nodeUniform30 ) ) ) );
	let nodeConst15 = clamp( dot( positionViewDirection, normalize( ( nodeConst14 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst16 = exp2( ( ( ( nodeConst15 * -5.55473 ) - 6.98316 ) * nodeConst15 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar4 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst16 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst16 ) ) ) ) ) );
	let nodeConst17 = normalize( ( nodeConst14 + positionViewDirection ) );
	let nodeConst18 = clamp( dot( positionViewDirection, nodeConst17 ), 0.0, 1.0 );
	let nodeConst19 = exp2( ( ( ( nodeConst18 * -5.55473 ) - 6.98316 ) * nodeConst18 ) );
	let nodeConst20 = max( Roughness, 0.045 );
	let nodeConst21 = ( nodeConst20 * nodeConst20 );
	directSpecular = ( directSpecular + ( ( nodeVar4 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst19 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst19 ) ) ) * vec3<f32>( fn110( nodeConst21, clamp( dot( normalView, nodeConst14 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst21, clamp( dot( normalView, nodeConst17 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst22 = ( render.nodeUniform31 - v_positionView );
	let nodeConst23 = normalize( nodeConst22 );
	nodeVar5 = ( vec3<f32>( clamp( dot( normalView, nodeConst23 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform32 * vec3<f32>( smoothstep( render.nodeUniform33, render.nodeUniform34, dot( nodeConst23, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform35 - render.nodeUniform36 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst22 ), render.nodeUniform37, render.nodeUniform38 ) ) ) );
	let nodeConst24 = clamp( dot( positionViewDirection, normalize( ( nodeConst23 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst25 = exp2( ( ( ( nodeConst24 * -5.55473 ) - 6.98316 ) * nodeConst24 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar5 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst25 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst25 ) ) ) ) ) );
	let nodeConst26 = normalize( ( nodeConst23 + positionViewDirection ) );
	let nodeConst27 = clamp( dot( positionViewDirection, nodeConst26 ), 0.0, 1.0 );
	let nodeConst28 = exp2( ( ( ( nodeConst27 * -5.55473 ) - 6.98316 ) * nodeConst27 ) );
	let nodeConst29 = max( Roughness, 0.045 );
	let nodeConst30 = ( nodeConst29 * nodeConst29 );
	directSpecular = ( directSpecular + ( ( nodeVar5 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst28 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst28 ) ) ) * vec3<f32>( fn110( nodeConst30, clamp( dot( normalView, nodeConst23 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst30, clamp( dot( normalView, nodeConst26 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst31 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst32 = ( object.nodeUniform40 * vec4<f32>( nodeConst31, 1.0 ) );
	let nodeConst33 = clamp( Roughness, 0.0, 1.0 );
	nodeVar6 = textureSampleLevel( nodeUniform39, nodeUniform39_sampler, vec3<f32>( ( - nodeConst32.x ), nodeConst32.yz ), ( ( object.nodeUniform42 * nodeConst33 ) * ( 2.0 - nodeConst33 ) ) );
	radiance = ( radiance + ( nodeVar6.xyz * vec3<f32>( object.nodeUniform43 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst34 = ( object.nodeUniform40 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst35 = clamp( 1.0, 0.0, 1.0 );
	nodeVar7 = textureSampleLevel( nodeUniform39, nodeUniform39_sampler, vec3<f32>( ( - nodeConst34.x ), nodeConst34.yz ), ( ( object.nodeUniform42 * nodeConst35 ) * ( 2.0 - nodeConst35 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar7.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform43 ) ) );
	ambientOcclusion = 1.0;
	ambientOcclusion = ( ambientOcclusion * AmbientOcclusion );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar8 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar8 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst36 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst36 );
	let nodeConst37 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst38 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst36 * nodeConst37 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst38 ) * nodeConst37 ) ) ) * vec3<f32>( nodeConst38 ) ) );
	let nodeConst39 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar9 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst39 ) );
	nodeVar10 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst39 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar9 );
	indirectDiffuse = ( indirectDiffuse + nodeVar10 );
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst40 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst40;

	// result

	output.color = nodeConst40;

	return output;

}
