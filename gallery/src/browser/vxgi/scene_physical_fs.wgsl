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
@binding( 1 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform28_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform28 : texture_depth_cube;
@binding( 7 ) @group( 1 ) var nodeUniform33_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform33 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform14 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform19 : f32,
	nodeUniform29 : f32,
	nodeUniform3 : vec2<f32>,
	nodeUniform21 : f32,
	nodeUniform20 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : vec2<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform27 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar0 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> IOR : f32;
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
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
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
	@location( 3 ) v_positionWorld : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	nodeVar0 = textureSample( nodeUniform2, nodeUniform2_sampler, ( fragCoord.xy / render.nodeUniform3 ) ).x;
	let nodeConst0 = max( nodeVar0, 0.001 );
	AmbientOcclusion = nodeConst0;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform5, 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	IOR = object.nodeUniform8;
	let nodeConst2 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst2 * nodeConst2 ) ) * object.nodeUniform9 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform10 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform10, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst3 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst3 );
	let nodeConst4 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst5 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst3 * nodeConst4 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst5 ) * nodeConst4 ) ) ) * vec3<f32>( nodeConst5 ) ) );
	let nodeConst6 = ( render.nodeUniform15 - v_positionView );
	let nodeConst7 = normalize( nodeConst6 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst8 = ( render.nodeUniform17 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform19 ) ) ), 1.0 ) ).xyz;
	let nodeConst9 = abs( nodeConst8 );
	nodeVar2 = 1.0;
	let nodeConst10 = max( max( nodeConst9.x, nodeConst9.y ), nodeConst9.z );

	if ( ( ( ( nodeConst10 - render.nodeUniform20 ) <= 0.0 ) && ( ( nodeConst10 - render.nodeUniform21 ) >= 0.0 ) ) ) {

		let nodeConst11 = ( render.nodeUniform20 - render.nodeUniform21 );
		let nodeConst12 = ( ( render.nodeUniform20 / nodeConst11 ) + render.nodeUniform22 );
		let nodeConst13 = length( nodeConst8 );
		let nodeConst14 = ( - ( ( render.nodeUniform21 * render.nodeUniform20 ) / ( nodeConst11 * nodeConst13 ) ) );
		let nodeConst15 = ( render.nodeUniform23 / render.nodeUniform24.x );
		let nodeConst16 = normalize( nodeConst8 );
		let nodeConst17 = abs( nodeConst16 );

		if ( ( nodeConst17.x > nodeConst17.z ) ) {

			nodeVar3 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar3 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		let nodeConst18 = normalize( cross( nodeConst16, nodeVar3 ) );
		let nodeConst19 = cross( nodeConst16, nodeConst18 );
		let nodeConst20 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst21 = ( render.nodeUniform25 - render.nodeUniform26 );
		let nodeConst22 = ( ( render.nodeUniform25 / nodeConst21 ) + render.nodeUniform27 );
		let nodeConst23 = ( - ( ( render.nodeUniform26 * render.nodeUniform25 ) / ( nodeConst21 * nodeConst13 ) ) );
		nodeVar4 = 0.0;

		for ( var i : i32 = 0; i < 5; i ++ ) {

			let nodeConst24 = fn25( i, 5, nodeConst20 );
			let nodeConst25 = ( nodeConst16 + ( ( ( nodeConst18 * vec3<f32>( nodeConst24.x ) ) + ( nodeConst19 * vec3<f32>( nodeConst24.y ) ) ) * vec3<f32>( nodeConst15 ) ) );
			let nodeConst26 = abs( nodeConst25 );
			let nodeConst27 = max( max( nodeConst26.x, nodeConst26.y ), nodeConst26.z );
			nodeVar5 = textureSampleCompare( nodeUniform28, nodeUniform28_sampler, vec3<f32>( ( - nodeConst25.x ), nodeConst25.yz ), ( nodeConst22 + ( nodeConst23 / nodeConst27 ) ) );
			nodeVar4 = ( nodeVar4 + nodeVar5 );

		}

		nodeVar2 = ( nodeVar4 * 0.2 );
		

	}

	nodeVar6 = mix( 1.0, nodeVar2, render.nodeUniform29 );
	nodeVar7 = ( vec3<f32>( clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform16 * vec3<f32>( nodeVar6 ) ) * vec3<f32>( fn44( length( nodeConst6 ), render.nodeUniform30, render.nodeUniform31 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst28 = clamp( dot( positionViewDirection, normalize( ( nodeConst7 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst29 = exp2( ( ( ( nodeConst28 * -5.55473 ) - 6.98316 ) * nodeConst28 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar7 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst29 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst29 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst30 = normalize( ( nodeConst7 + positionViewDirection ) );
	let nodeConst31 = clamp( dot( positionViewDirection, nodeConst30 ), 0.0, 1.0 );
	let nodeConst32 = exp2( ( ( ( nodeConst31 * -5.55473 ) - 6.98316 ) * nodeConst31 ) );
	let nodeConst33 = max( Roughness, 0.045 );
	let nodeConst34 = ( nodeConst33 * nodeConst33 );
	directSpecular = ( directSpecular + ( ( nodeVar7 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst32 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst32 ) ) ) * vec3<f32>( fn110( nodeConst34, clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst34, clamp( dot( normalView, nodeConst30 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform32 );
	ambientOcclusion = 1.0;
	ambientOcclusion = ( ambientOcclusion * AmbientOcclusion );
	nodeVar8 = textureSample( nodeUniform33, nodeUniform33_sampler, ( fragCoord.xy / render.nodeUniform3 ) );
	irradiance = ( irradiance + ( nodeVar8.xyz / vec3<f32>( nodeConst0 ) ) );
	nodeVar9 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar9 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst35 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst35 );
	let nodeConst36 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst37 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst35 * nodeConst36 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst37 ) * nodeConst36 ) ) ) * vec3<f32>( nodeConst37 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst38 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar10 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst38 ) );
	nodeVar11 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst38 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar10 );
	indirectDiffuse = ( indirectDiffuse + nodeVar11 );
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst39 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst39;

	// result

	output.color = nodeConst39;

	return output;

}
