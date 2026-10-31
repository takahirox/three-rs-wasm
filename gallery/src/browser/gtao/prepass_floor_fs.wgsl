// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform37_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform37 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform38 : mat4x4<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform43 : mat4x4<f32>,
	nodeUniform46 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform42 : mat4x4<f32>,
	nodeUniform44 : mat4x4<f32>,
	nodeUniform45 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform22 : vec3<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform30 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform33 : vec3<f32>,
	nodeUniform34 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform29 : vec3<f32>
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
var<private> nodeVar2 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;

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
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) positionPrevious : vec3<f32>,
	@location( 5 ) nodeVarying7 : vec2<f32> ) -> OutputType {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform5, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	let nodeConst4 = ( render.nodeUniform12 - v_positionView );
	let nodeConst5 = normalize( nodeConst4 );
	nodeVar2 = ( vec3<f32>( clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform13 * vec3<f32>( smoothstep( render.nodeUniform14, render.nodeUniform15, dot( nodeConst5, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform17 - render.nodeUniform18 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst4 ), render.nodeUniform19, render.nodeUniform20 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst6 = clamp( dot( positionViewDirection, normalize( ( nodeConst5 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst7 = exp2( ( ( ( nodeConst6 * -5.55473 ) - 6.98316 ) * nodeConst6 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar2 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst7 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst7 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst8 = normalize( ( nodeConst5 + positionViewDirection ) );
	let nodeConst9 = clamp( dot( positionViewDirection, nodeConst8 ), 0.0, 1.0 );
	let nodeConst10 = exp2( ( ( ( nodeConst9 * -5.55473 ) - 6.98316 ) * nodeConst9 ) );
	let nodeConst11 = max( Roughness, 0.045 );
	let nodeConst12 = ( nodeConst11 * nodeConst11 );
	directSpecular = ( directSpecular + ( ( nodeVar2 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst10 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst10 ) ) ) * vec3<f32>( fn110( nodeConst12, clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst12, clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst13 = ( render.nodeUniform21 - v_positionView );
	let nodeConst14 = normalize( nodeConst13 );
	nodeVar3 = ( vec3<f32>( clamp( dot( normalView, nodeConst14 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform22 * vec3<f32>( smoothstep( render.nodeUniform23, render.nodeUniform24, dot( nodeConst14, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform25 - render.nodeUniform26 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst13 ), render.nodeUniform27, render.nodeUniform28 ) ) ) );
	let nodeConst15 = clamp( dot( positionViewDirection, normalize( ( nodeConst14 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst16 = exp2( ( ( ( nodeConst15 * -5.55473 ) - 6.98316 ) * nodeConst15 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar3 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst16 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst16 ) ) ) ) ) );
	let nodeConst17 = normalize( ( nodeConst14 + positionViewDirection ) );
	let nodeConst18 = clamp( dot( positionViewDirection, nodeConst17 ), 0.0, 1.0 );
	let nodeConst19 = exp2( ( ( ( nodeConst18 * -5.55473 ) - 6.98316 ) * nodeConst18 ) );
	let nodeConst20 = max( Roughness, 0.045 );
	let nodeConst21 = ( nodeConst20 * nodeConst20 );
	directSpecular = ( directSpecular + ( ( nodeVar3 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst19 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst19 ) ) ) * vec3<f32>( fn110( nodeConst21, clamp( dot( normalView, nodeConst14 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst21, clamp( dot( normalView, nodeConst17 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst22 = ( render.nodeUniform29 - v_positionView );
	let nodeConst23 = normalize( nodeConst22 );
	nodeVar4 = ( vec3<f32>( clamp( dot( normalView, nodeConst23 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform30 * vec3<f32>( smoothstep( render.nodeUniform31, render.nodeUniform32, dot( nodeConst23, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform33 - render.nodeUniform34 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst22 ), render.nodeUniform35, render.nodeUniform36 ) ) ) );
	let nodeConst24 = clamp( dot( positionViewDirection, normalize( ( nodeConst23 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst25 = exp2( ( ( ( nodeConst24 * -5.55473 ) - 6.98316 ) * nodeConst24 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar4 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst25 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst25 ) ) ) ) ) );
	let nodeConst26 = normalize( ( nodeConst23 + positionViewDirection ) );
	let nodeConst27 = clamp( dot( positionViewDirection, nodeConst26 ), 0.0, 1.0 );
	let nodeConst28 = exp2( ( ( ( nodeConst27 * -5.55473 ) - 6.98316 ) * nodeConst27 ) );
	let nodeConst29 = max( Roughness, 0.045 );
	let nodeConst30 = ( nodeConst29 * nodeConst29 );
	directSpecular = ( directSpecular + ( ( nodeVar4 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst28 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst28 ) ) ) * vec3<f32>( fn110( nodeConst30, clamp( dot( normalView, nodeConst23 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst30, clamp( dot( normalView, nodeConst26 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst31 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst32 = ( object.nodeUniform38 * vec4<f32>( nodeConst31, 1.0 ) );
	let nodeConst33 = clamp( Roughness, 0.0, 1.0 );
	nodeVar5 = textureSampleLevel( nodeUniform37, nodeUniform37_sampler, vec3<f32>( ( - nodeConst32.x ), nodeConst32.yz ), ( ( object.nodeUniform40 * nodeConst33 ) * ( 2.0 - nodeConst33 ) ) );
	radiance = ( radiance + ( nodeVar5.xyz * vec3<f32>( object.nodeUniform41 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst34 = ( object.nodeUniform38 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst35 = clamp( 1.0, 0.0, 1.0 );
	nodeVar6 = textureSampleLevel( nodeUniform37, nodeUniform37_sampler, vec3<f32>( ( - nodeConst34.x ), nodeConst34.yz ), ( ( object.nodeUniform40 * nodeConst35 ) * ( 2.0 - nodeConst35 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar6.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform41 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar7 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar7 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst36 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst36 );
	let nodeConst37 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst38 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst36 * nodeConst37 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst38 ) * nodeConst37 ) ) ) * vec3<f32>( nodeConst38 ) ) );
	let nodeConst39 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar8 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst39 ) );
	nodeVar9 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst39 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar8 );
	indirectDiffuse = ( indirectDiffuse + nodeVar9 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	let nodeConst40 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m0 = vec4<f32>( nodeConst40, 1.0 );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform43 );
	let nodeConst41 = ( ( render.nodeUniform42 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	let nodeConst42 = ( ( render.nodeUniform44 * ( render.nodeUniform45 * object.nodeUniform46 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	let nodeConst43 = ( ( nodeConst41.xy / vec2<f32>( nodeConst41.w ) ) - ( nodeConst42.xy / vec2<f32>( nodeConst42.w ) ) );
	output.m1 = vec4<f32>( vec3<f32>( nodeConst43, 0.0 ), 1.0 );

	// result

	return output;

}
