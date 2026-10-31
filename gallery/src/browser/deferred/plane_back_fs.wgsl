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
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform42_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform42 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform43 : mat4x4<f32>,
	nodeUniform45 : f32,
	nodeUniform46 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : vec3<f32>,
	nodeUniform32 : f32,
	nodeUniform33 : f32,
	nodeUniform35 : vec3<f32>,
	nodeUniform36 : f32,
	nodeUniform37 : f32,
	nodeUniform39 : vec3<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform34 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
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
var<private> nodeVar0 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
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
	@location( 2 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	let nodeConst4 = ( render.nodeUniform10 - v_positionView );
	let nodeConst5 = normalize( nodeConst4 );
	nodeVar1 = ( vec3<f32>( clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( fn44( length( nodeConst4 ), render.nodeUniform12, render.nodeUniform13 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst6 = clamp( dot( positionViewDirection, normalize( ( nodeConst5 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst7 = exp2( ( ( ( nodeConst6 * -5.55473 ) - 6.98316 ) * nodeConst6 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar1 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst7 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst7 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst8 = normalize( ( nodeConst5 + positionViewDirection ) );
	let nodeConst9 = clamp( dot( positionViewDirection, nodeConst8 ), 0.0, 1.0 );
	let nodeConst10 = exp2( ( ( ( nodeConst9 * -5.55473 ) - 6.98316 ) * nodeConst9 ) );
	let nodeConst11 = max( Roughness, 0.045 );
	let nodeConst12 = ( nodeConst11 * nodeConst11 );
	directSpecular = ( directSpecular + ( ( nodeVar1 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst10 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst10 ) ) ) * vec3<f32>( fn110( nodeConst12, clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst12, clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst13 = ( render.nodeUniform14 - v_positionView );
	let nodeConst14 = normalize( nodeConst13 );
	nodeVar2 = ( vec3<f32>( clamp( dot( normalView, nodeConst14 ), 0.0, 1.0 ) ) * ( render.nodeUniform15 * vec3<f32>( fn44( length( nodeConst13 ), render.nodeUniform16, render.nodeUniform17 ) ) ) );
	let nodeConst15 = clamp( dot( positionViewDirection, normalize( ( nodeConst14 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst16 = exp2( ( ( ( nodeConst15 * -5.55473 ) - 6.98316 ) * nodeConst15 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar2 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst16 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst16 ) ) ) ) ) );
	let nodeConst17 = normalize( ( nodeConst14 + positionViewDirection ) );
	let nodeConst18 = clamp( dot( positionViewDirection, nodeConst17 ), 0.0, 1.0 );
	let nodeConst19 = exp2( ( ( ( nodeConst18 * -5.55473 ) - 6.98316 ) * nodeConst18 ) );
	let nodeConst20 = max( Roughness, 0.045 );
	let nodeConst21 = ( nodeConst20 * nodeConst20 );
	directSpecular = ( directSpecular + ( ( nodeVar2 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst19 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst19 ) ) ) * vec3<f32>( fn110( nodeConst21, clamp( dot( normalView, nodeConst14 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst21, clamp( dot( normalView, nodeConst17 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst22 = ( render.nodeUniform18 - v_positionView );
	let nodeConst23 = normalize( nodeConst22 );
	nodeVar3 = ( vec3<f32>( clamp( dot( normalView, nodeConst23 ), 0.0, 1.0 ) ) * ( render.nodeUniform19 * vec3<f32>( fn44( length( nodeConst22 ), render.nodeUniform20, render.nodeUniform21 ) ) ) );
	let nodeConst24 = clamp( dot( positionViewDirection, normalize( ( nodeConst23 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst25 = exp2( ( ( ( nodeConst24 * -5.55473 ) - 6.98316 ) * nodeConst24 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar3 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst25 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst25 ) ) ) ) ) );
	let nodeConst26 = normalize( ( nodeConst23 + positionViewDirection ) );
	let nodeConst27 = clamp( dot( positionViewDirection, nodeConst26 ), 0.0, 1.0 );
	let nodeConst28 = exp2( ( ( ( nodeConst27 * -5.55473 ) - 6.98316 ) * nodeConst27 ) );
	let nodeConst29 = max( Roughness, 0.045 );
	let nodeConst30 = ( nodeConst29 * nodeConst29 );
	directSpecular = ( directSpecular + ( ( nodeVar3 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst28 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst28 ) ) ) * vec3<f32>( fn110( nodeConst30, clamp( dot( normalView, nodeConst23 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst30, clamp( dot( normalView, nodeConst26 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst31 = ( render.nodeUniform22 - v_positionView );
	let nodeConst32 = normalize( nodeConst31 );
	nodeVar4 = ( vec3<f32>( clamp( dot( normalView, nodeConst32 ), 0.0, 1.0 ) ) * ( render.nodeUniform23 * vec3<f32>( fn44( length( nodeConst31 ), render.nodeUniform24, render.nodeUniform25 ) ) ) );
	let nodeConst33 = clamp( dot( positionViewDirection, normalize( ( nodeConst32 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst34 = exp2( ( ( ( nodeConst33 * -5.55473 ) - 6.98316 ) * nodeConst33 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar4 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst34 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst34 ) ) ) ) ) );
	let nodeConst35 = normalize( ( nodeConst32 + positionViewDirection ) );
	let nodeConst36 = clamp( dot( positionViewDirection, nodeConst35 ), 0.0, 1.0 );
	let nodeConst37 = exp2( ( ( ( nodeConst36 * -5.55473 ) - 6.98316 ) * nodeConst36 ) );
	let nodeConst38 = max( Roughness, 0.045 );
	let nodeConst39 = ( nodeConst38 * nodeConst38 );
	directSpecular = ( directSpecular + ( ( nodeVar4 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst37 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst37 ) ) ) * vec3<f32>( fn110( nodeConst39, clamp( dot( normalView, nodeConst32 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst39, clamp( dot( normalView, nodeConst35 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst40 = ( render.nodeUniform26 - v_positionView );
	let nodeConst41 = normalize( nodeConst40 );
	nodeVar5 = ( vec3<f32>( clamp( dot( normalView, nodeConst41 ), 0.0, 1.0 ) ) * ( render.nodeUniform27 * vec3<f32>( fn44( length( nodeConst40 ), render.nodeUniform28, render.nodeUniform29 ) ) ) );
	let nodeConst42 = clamp( dot( positionViewDirection, normalize( ( nodeConst41 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst43 = exp2( ( ( ( nodeConst42 * -5.55473 ) - 6.98316 ) * nodeConst42 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar5 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst43 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst43 ) ) ) ) ) );
	let nodeConst44 = normalize( ( nodeConst41 + positionViewDirection ) );
	let nodeConst45 = clamp( dot( positionViewDirection, nodeConst44 ), 0.0, 1.0 );
	let nodeConst46 = exp2( ( ( ( nodeConst45 * -5.55473 ) - 6.98316 ) * nodeConst45 ) );
	let nodeConst47 = max( Roughness, 0.045 );
	let nodeConst48 = ( nodeConst47 * nodeConst47 );
	directSpecular = ( directSpecular + ( ( nodeVar5 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst46 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst46 ) ) ) * vec3<f32>( fn110( nodeConst48, clamp( dot( normalView, nodeConst41 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst48, clamp( dot( normalView, nodeConst44 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst49 = ( render.nodeUniform30 - v_positionView );
	let nodeConst50 = normalize( nodeConst49 );
	nodeVar6 = ( vec3<f32>( clamp( dot( normalView, nodeConst50 ), 0.0, 1.0 ) ) * ( render.nodeUniform31 * vec3<f32>( fn44( length( nodeConst49 ), render.nodeUniform32, render.nodeUniform33 ) ) ) );
	let nodeConst51 = clamp( dot( positionViewDirection, normalize( ( nodeConst50 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst52 = exp2( ( ( ( nodeConst51 * -5.55473 ) - 6.98316 ) * nodeConst51 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar6 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst52 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst52 ) ) ) ) ) );
	let nodeConst53 = normalize( ( nodeConst50 + positionViewDirection ) );
	let nodeConst54 = clamp( dot( positionViewDirection, nodeConst53 ), 0.0, 1.0 );
	let nodeConst55 = exp2( ( ( ( nodeConst54 * -5.55473 ) - 6.98316 ) * nodeConst54 ) );
	let nodeConst56 = max( Roughness, 0.045 );
	let nodeConst57 = ( nodeConst56 * nodeConst56 );
	directSpecular = ( directSpecular + ( ( nodeVar6 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst55 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst55 ) ) ) * vec3<f32>( fn110( nodeConst57, clamp( dot( normalView, nodeConst50 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst57, clamp( dot( normalView, nodeConst53 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst58 = ( render.nodeUniform34 - v_positionView );
	let nodeConst59 = normalize( nodeConst58 );
	nodeVar7 = ( vec3<f32>( clamp( dot( normalView, nodeConst59 ), 0.0, 1.0 ) ) * ( render.nodeUniform35 * vec3<f32>( fn44( length( nodeConst58 ), render.nodeUniform36, render.nodeUniform37 ) ) ) );
	let nodeConst60 = clamp( dot( positionViewDirection, normalize( ( nodeConst59 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst61 = exp2( ( ( ( nodeConst60 * -5.55473 ) - 6.98316 ) * nodeConst60 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar7 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst61 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst61 ) ) ) ) ) );
	let nodeConst62 = normalize( ( nodeConst59 + positionViewDirection ) );
	let nodeConst63 = clamp( dot( positionViewDirection, nodeConst62 ), 0.0, 1.0 );
	let nodeConst64 = exp2( ( ( ( nodeConst63 * -5.55473 ) - 6.98316 ) * nodeConst63 ) );
	let nodeConst65 = max( Roughness, 0.045 );
	let nodeConst66 = ( nodeConst65 * nodeConst65 );
	directSpecular = ( directSpecular + ( ( nodeVar7 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst64 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst64 ) ) ) * vec3<f32>( fn110( nodeConst66, clamp( dot( normalView, nodeConst59 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst66, clamp( dot( normalView, nodeConst62 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst67 = ( render.nodeUniform38 - v_positionView );
	let nodeConst68 = normalize( nodeConst67 );
	nodeVar8 = ( vec3<f32>( clamp( dot( normalView, nodeConst68 ), 0.0, 1.0 ) ) * ( render.nodeUniform39 * vec3<f32>( fn44( length( nodeConst67 ), render.nodeUniform40, render.nodeUniform41 ) ) ) );
	let nodeConst69 = clamp( dot( positionViewDirection, normalize( ( nodeConst68 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst70 = exp2( ( ( ( nodeConst69 * -5.55473 ) - 6.98316 ) * nodeConst69 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar8 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst70 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst70 ) ) ) ) ) );
	let nodeConst71 = normalize( ( nodeConst68 + positionViewDirection ) );
	let nodeConst72 = clamp( dot( positionViewDirection, nodeConst71 ), 0.0, 1.0 );
	let nodeConst73 = exp2( ( ( ( nodeConst72 * -5.55473 ) - 6.98316 ) * nodeConst72 ) );
	let nodeConst74 = max( Roughness, 0.045 );
	let nodeConst75 = ( nodeConst74 * nodeConst74 );
	directSpecular = ( directSpecular + ( ( nodeVar8 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst73 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst73 ) ) ) * vec3<f32>( fn110( nodeConst75, clamp( dot( normalView, nodeConst68 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst75, clamp( dot( normalView, nodeConst71 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst76 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst77 = ( object.nodeUniform43 * vec4<f32>( nodeConst76, 1.0 ) );
	let nodeConst78 = clamp( Roughness, 0.0, 1.0 );
	nodeVar9 = textureSampleLevel( nodeUniform42, nodeUniform42_sampler, vec3<f32>( ( - nodeConst77.x ), nodeConst77.yz ), ( ( object.nodeUniform45 * nodeConst78 ) * ( 2.0 - nodeConst78 ) ) );
	radiance = ( radiance + ( nodeVar9.xyz * vec3<f32>( object.nodeUniform46 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst79 = ( object.nodeUniform43 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst80 = clamp( 1.0, 0.0, 1.0 );
	nodeVar10 = textureSampleLevel( nodeUniform42, nodeUniform42_sampler, vec3<f32>( ( - nodeConst79.x ), nodeConst79.yz ), ( ( object.nodeUniform45 * nodeConst80 ) * ( 2.0 - nodeConst80 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar10.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform46 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar11 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar11 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst81 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst81 );
	let nodeConst82 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst83 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst81 * nodeConst82 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst83 ) * nodeConst82 ) ) ) * vec3<f32>( nodeConst83 ) ) );
	let nodeConst84 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar12 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst84 ) );
	nodeVar13 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst84 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar12 );
	indirectDiffuse = ( indirectDiffuse + nodeVar13 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst85 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst85;

	// result

	output.color = nodeConst85;

	return output;

}
